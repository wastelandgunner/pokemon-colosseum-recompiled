#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// fightTrainerAiWazaDamage087 (0x802529a0, 36 bytes)
#[inline(never)]
pub fn f_802529a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802529a0) { return; }
        if c.prof { crate::prof::enter(c, 0x802529a0); f_802529a0_at(c, 0x802529a0); crate::prof::exit(c, 0x802529a0); return; }
    }
    f_802529a0_b(c, 0x802529a0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802529a0_at(c: &mut Cpu, pc: u32) { f_802529a0_b(c, pc) }
#[inline(always)]
fn f_802529a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802529a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802529a0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802529a0 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x802529b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237774(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802529b0;
            }
            0x802529b0 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802529a0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage086 (0x802529c4, 8 bytes)
#[inline(never)]
pub fn f_802529c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802529c4) { return; }
        if c.prof { crate::prof::enter(c, 0x802529c4); f_802529c4_at(c, 0x802529c4); crate::prof::exit(c, 0x802529c4); return; }
    }
    f_802529c4_b(c, 0x802529c4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802529c4_at(c: &mut Cpu, pc: u32) { f_802529c4_b(c, pc) }
#[inline(always)]
fn f_802529c4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802529c4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802529c4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802529c4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802529c4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage085 (0x802529cc, 8 bytes)
#[inline(never)]
pub fn f_802529cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802529cc) { return; }
        if c.prof { crate::prof::enter(c, 0x802529cc); f_802529cc_at(c, 0x802529cc); crate::prof::exit(c, 0x802529cc); return; }
    }
    f_802529cc_b(c, 0x802529cc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802529cc_at(c: &mut Cpu, pc: u32) { f_802529cc_b(c, pc) }
#[inline(always)]
fn f_802529cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802529cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802529cc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802529cc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802529cc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage084 (0x802529d4, 8 bytes)
#[inline(never)]
pub fn f_802529d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802529d4) { return; }
        if c.prof { crate::prof::enter(c, 0x802529d4); f_802529d4_at(c, 0x802529d4); crate::prof::exit(c, 0x802529d4); return; }
    }
    f_802529d4_b(c, 0x802529d4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802529d4_at(c: &mut Cpu, pc: u32) { f_802529d4_b(c, pc) }
#[inline(always)]
fn f_802529d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802529d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802529d4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802529d4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802529d4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage083 (0x802529dc, 8 bytes)
#[inline(never)]
pub fn f_802529dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802529dc) { return; }
        if c.prof { crate::prof::enter(c, 0x802529dc); f_802529dc_at(c, 0x802529dc); crate::prof::exit(c, 0x802529dc); return; }
    }
    f_802529dc_b(c, 0x802529dc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802529dc_at(c: &mut Cpu, pc: u32) { f_802529dc_b(c, pc) }
#[inline(always)]
fn f_802529dc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802529dc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802529dc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802529dc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802529dc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage082 (0x802529e4, 8 bytes)
#[inline(never)]
pub fn f_802529e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802529e4) { return; }
        if c.prof { crate::prof::enter(c, 0x802529e4); f_802529e4_at(c, 0x802529e4); crate::prof::exit(c, 0x802529e4); return; }
    }
    f_802529e4_b(c, 0x802529e4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802529e4_at(c: &mut Cpu, pc: u32) { f_802529e4_b(c, pc) }
#[inline(always)]
fn f_802529e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802529e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802529e4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802529e4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802529e4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage081 (0x802529ec, 8 bytes)
#[inline(never)]
pub fn f_802529ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802529ec) { return; }
        if c.prof { crate::prof::enter(c, 0x802529ec); f_802529ec_at(c, 0x802529ec); crate::prof::exit(c, 0x802529ec); return; }
    }
    f_802529ec_b(c, 0x802529ec);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802529ec_at(c: &mut Cpu, pc: u32) { f_802529ec_b(c, pc) }
#[inline(always)]
fn f_802529ec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802529ec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802529ec, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802529ec => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802529ec: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage080 (0x802529f4, 132 bytes)
#[inline(never)]
pub fn f_802529f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802529f4) { return; }
        if c.prof { crate::prof::enter(c, 0x802529f4); f_802529f4_at(c, 0x802529f4); crate::prof::exit(c, 0x802529f4); return; }
    }
    f_802529f4_b(c, 0x802529f4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802529f4_at(c: &mut Cpu, pc: u32) { f_802529f4_b(c, pc) }
#[inline(always)]
fn f_802529f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802529f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802529f4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802529f4 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80252a30; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252a30;
            }
            0x80252a30 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80252a44; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252a44;
            }
            0x80252a44 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80252a60; continue; }
                pc = 0x80252a50;
            }
            0x80252a50 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x80252a60; continue; }
                pc = 0x80252a5c;
            }
            0x80252a5c => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x80252a60;
            }
            0x80252a60 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802529f4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage079 (0x80252a78, 8 bytes)
#[inline(never)]
pub fn f_80252a78(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252a78) { return; }
        if c.prof { crate::prof::enter(c, 0x80252a78); f_80252a78_at(c, 0x80252a78); crate::prof::exit(c, 0x80252a78); return; }
    }
    f_80252a78_b(c, 0x80252a78);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252a78_at(c: &mut Cpu, pc: u32) { f_80252a78_b(c, pc) }
#[inline(always)]
fn f_80252a78_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252a78;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252a78, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252a78 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252a78: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage078 (0x80252a80, 132 bytes)
#[inline(never)]
pub fn f_80252a80(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252a80) { return; }
        if c.prof { crate::prof::enter(c, 0x80252a80); f_80252a80_at(c, 0x80252a80); crate::prof::exit(c, 0x80252a80); return; }
    }
    f_80252a80_b(c, 0x80252a80);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252a80_at(c: &mut Cpu, pc: u32) { f_80252a80_b(c, pc) }
#[inline(always)]
fn f_80252a80_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252a80;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252a80, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252a80 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80252abc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252abc;
            }
            0x80252abc => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80252ad0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252ad0;
            }
            0x80252ad0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80252aec; continue; }
                pc = 0x80252adc;
            }
            0x80252adc => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x80252aec; continue; }
                pc = 0x80252ae8;
            }
            0x80252ae8 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x80252aec;
            }
            0x80252aec => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252a80: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage077 (0x80252b04, 64 bytes)
#[inline(never)]
pub fn f_80252b04(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252b04) { return; }
        if c.prof { crate::prof::enter(c, 0x80252b04); f_80252b04_at(c, 0x80252b04); crate::prof::exit(c, 0x80252b04); return; }
    }
    f_80252b04_b(c, 0x80252b04);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252b04_at(c: &mut Cpu, pc: u32) { f_80252b04_b(c, pc) }
#[inline(always)]
fn f_80252b04_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252b04;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252b04, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252b04 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.r[9] = 0x0;
                c.r[5] = c.r[0] | c.r[0];
                c.r[10] = 0x0;
                c.lr = 0x80252b30; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252b30;
            }
            0x80252b30 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[3].rotate_left(1) & 0xfffffffe;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252b04: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage076 (0x80252b44, 132 bytes)
#[inline(never)]
pub fn f_80252b44(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252b44) { return; }
        if c.prof { crate::prof::enter(c, 0x80252b44); f_80252b44_at(c, 0x80252b44); crate::prof::exit(c, 0x80252b44); return; }
    }
    f_80252b44_b(c, 0x80252b44);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252b44_at(c: &mut Cpu, pc: u32) { f_80252b44_b(c, pc) }
#[inline(always)]
fn f_80252b44_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252b44;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252b44, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252b44 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80252b80; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252b80;
            }
            0x80252b80 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80252b94; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252b94;
            }
            0x80252b94 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80252bb0; continue; }
                pc = 0x80252ba0;
            }
            0x80252ba0 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x80252bb0; continue; }
                pc = 0x80252bac;
            }
            0x80252bac => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x80252bb0;
            }
            0x80252bb0 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252b44: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage075 (0x80252bc8, 60 bytes)
#[inline(never)]
pub fn f_80252bc8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252bc8) { return; }
        if c.prof { crate::prof::enter(c, 0x80252bc8); f_80252bc8_at(c, 0x80252bc8); crate::prof::exit(c, 0x80252bc8); return; }
    }
    f_80252bc8_b(c, 0x80252bc8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252bc8_at(c: &mut Cpu, pc: u32) { f_80252bc8_b(c, pc) }
#[inline(always)]
fn f_80252bc8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252bc8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252bc8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252bc8 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.r[9] = 0x0;
                c.r[5] = c.r[0] | c.r[0];
                c.r[10] = 0x0;
                c.lr = 0x80252bf4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252bf4;
            }
            0x80252bf4 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252bc8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage073 (0x80252c04, 132 bytes)
#[inline(never)]
pub fn f_80252c04(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252c04) { return; }
        if c.prof { crate::prof::enter(c, 0x80252c04); f_80252c04_at(c, 0x80252c04); crate::prof::exit(c, 0x80252c04); return; }
    }
    f_80252c04_b(c, 0x80252c04);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252c04_at(c: &mut Cpu, pc: u32) { f_80252c04_b(c, pc) }
#[inline(always)]
fn f_80252c04_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252c04;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252c04, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252c04 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80252c40; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252c40;
            }
            0x80252c40 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80252c54; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252c54;
            }
            0x80252c54 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80252c70; continue; }
                pc = 0x80252c60;
            }
            0x80252c60 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x80252c70; continue; }
                pc = 0x80252c6c;
            }
            0x80252c6c => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x80252c70;
            }
            0x80252c70 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252c04: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage072 (0x80252c88, 132 bytes)
#[inline(never)]
pub fn f_80252c88(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252c88) { return; }
        if c.prof { crate::prof::enter(c, 0x80252c88); f_80252c88_at(c, 0x80252c88); crate::prof::exit(c, 0x80252c88); return; }
    }
    f_80252c88_b(c, 0x80252c88);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252c88_at(c: &mut Cpu, pc: u32) { f_80252c88_b(c, pc) }
#[inline(always)]
fn f_80252c88_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252c88;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252c88, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252c88 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80252cc4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252cc4;
            }
            0x80252cc4 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80252cd8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252cd8;
            }
            0x80252cd8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80252cf4; continue; }
                pc = 0x80252ce4;
            }
            0x80252ce4 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x80252cf4; continue; }
                pc = 0x80252cf0;
            }
            0x80252cf0 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x80252cf4;
            }
            0x80252cf4 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252c88: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage071 (0x80252d0c, 132 bytes)
#[inline(never)]
pub fn f_80252d0c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252d0c) { return; }
        if c.prof { crate::prof::enter(c, 0x80252d0c); f_80252d0c_at(c, 0x80252d0c); crate::prof::exit(c, 0x80252d0c); return; }
    }
    f_80252d0c_b(c, 0x80252d0c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252d0c_at(c: &mut Cpu, pc: u32) { f_80252d0c_b(c, pc) }
#[inline(always)]
fn f_80252d0c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252d0c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252d0c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252d0c => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80252d48; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252d48;
            }
            0x80252d48 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80252d5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252d5c;
            }
            0x80252d5c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80252d78; continue; }
                pc = 0x80252d68;
            }
            0x80252d68 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x80252d78; continue; }
                pc = 0x80252d74;
            }
            0x80252d74 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x80252d78;
            }
            0x80252d78 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252d0c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage070 (0x80252d90, 132 bytes)
#[inline(never)]
pub fn f_80252d90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252d90) { return; }
        if c.prof { crate::prof::enter(c, 0x80252d90); f_80252d90_at(c, 0x80252d90); crate::prof::exit(c, 0x80252d90); return; }
    }
    f_80252d90_b(c, 0x80252d90);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252d90_at(c: &mut Cpu, pc: u32) { f_80252d90_b(c, pc) }
#[inline(always)]
fn f_80252d90_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252d90;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252d90, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252d90 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80252dcc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252dcc;
            }
            0x80252dcc => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80252de0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252de0;
            }
            0x80252de0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80252dfc; continue; }
                pc = 0x80252dec;
            }
            0x80252dec => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x80252dfc; continue; }
                pc = 0x80252df8;
            }
            0x80252df8 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x80252dfc;
            }
            0x80252dfc => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252d90: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage069 (0x80252e14, 132 bytes)
#[inline(never)]
pub fn f_80252e14(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252e14) { return; }
        if c.prof { crate::prof::enter(c, 0x80252e14); f_80252e14_at(c, 0x80252e14); crate::prof::exit(c, 0x80252e14); return; }
    }
    f_80252e14_b(c, 0x80252e14);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252e14_at(c: &mut Cpu, pc: u32) { f_80252e14_b(c, pc) }
#[inline(always)]
fn f_80252e14_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252e14;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252e14, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252e14 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80252e50; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252e50;
            }
            0x80252e50 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80252e64; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252e64;
            }
            0x80252e64 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80252e80; continue; }
                pc = 0x80252e70;
            }
            0x80252e70 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x80252e80; continue; }
                pc = 0x80252e7c;
            }
            0x80252e7c => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x80252e80;
            }
            0x80252e80 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252e14: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage068 (0x80252e98, 132 bytes)
#[inline(never)]
pub fn f_80252e98(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252e98) { return; }
        if c.prof { crate::prof::enter(c, 0x80252e98); f_80252e98_at(c, 0x80252e98); crate::prof::exit(c, 0x80252e98); return; }
    }
    f_80252e98_b(c, 0x80252e98);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252e98_at(c: &mut Cpu, pc: u32) { f_80252e98_b(c, pc) }
#[inline(always)]
fn f_80252e98_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252e98;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252e98, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252e98 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80252ed4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252ed4;
            }
            0x80252ed4 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80252ee8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252ee8;
            }
            0x80252ee8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80252f04; continue; }
                pc = 0x80252ef4;
            }
            0x80252ef4 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x80252f04; continue; }
                pc = 0x80252f00;
            }
            0x80252f00 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x80252f04;
            }
            0x80252f04 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252e98: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage067 (0x80252f1c, 8 bytes)
#[inline(never)]
pub fn f_80252f1c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f1c) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f1c); f_80252f1c_at(c, 0x80252f1c); crate::prof::exit(c, 0x80252f1c); return; }
    }
    f_80252f1c_b(c, 0x80252f1c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f1c_at(c: &mut Cpu, pc: u32) { f_80252f1c_b(c, pc) }
#[inline(always)]
fn f_80252f1c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f1c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f1c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f1c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f1c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage066 (0x80252f24, 8 bytes)
#[inline(never)]
pub fn f_80252f24(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f24) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f24); f_80252f24_at(c, 0x80252f24); crate::prof::exit(c, 0x80252f24); return; }
    }
    f_80252f24_b(c, 0x80252f24);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f24_at(c: &mut Cpu, pc: u32) { f_80252f24_b(c, pc) }
#[inline(always)]
fn f_80252f24_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f24;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f24, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f24 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f24: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage065 (0x80252f2c, 8 bytes)
#[inline(never)]
pub fn f_80252f2c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f2c) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f2c); f_80252f2c_at(c, 0x80252f2c); crate::prof::exit(c, 0x80252f2c); return; }
    }
    f_80252f2c_b(c, 0x80252f2c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f2c_at(c: &mut Cpu, pc: u32) { f_80252f2c_b(c, pc) }
#[inline(always)]
fn f_80252f2c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f2c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f2c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f2c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f2c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage062 (0x80252f34, 8 bytes)
#[inline(never)]
pub fn f_80252f34(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f34) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f34); f_80252f34_at(c, 0x80252f34); crate::prof::exit(c, 0x80252f34); return; }
    }
    f_80252f34_b(c, 0x80252f34);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f34_at(c: &mut Cpu, pc: u32) { f_80252f34_b(c, pc) }
#[inline(always)]
fn f_80252f34_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f34;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f34, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f34 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f34: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage060 (0x80252f3c, 8 bytes)
#[inline(never)]
pub fn f_80252f3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f3c) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f3c); f_80252f3c_at(c, 0x80252f3c); crate::prof::exit(c, 0x80252f3c); return; }
    }
    f_80252f3c_b(c, 0x80252f3c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f3c_at(c: &mut Cpu, pc: u32) { f_80252f3c_b(c, pc) }
#[inline(always)]
fn f_80252f3c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f3c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f3c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f3c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f3c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage059 (0x80252f44, 8 bytes)
#[inline(never)]
pub fn f_80252f44(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f44) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f44); f_80252f44_at(c, 0x80252f44); crate::prof::exit(c, 0x80252f44); return; }
    }
    f_80252f44_b(c, 0x80252f44);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f44_at(c: &mut Cpu, pc: u32) { f_80252f44_b(c, pc) }
#[inline(always)]
fn f_80252f44_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f44;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f44, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f44 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f44: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage058 (0x80252f4c, 8 bytes)
#[inline(never)]
pub fn f_80252f4c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f4c) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f4c); f_80252f4c_at(c, 0x80252f4c); crate::prof::exit(c, 0x80252f4c); return; }
    }
    f_80252f4c_b(c, 0x80252f4c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f4c_at(c: &mut Cpu, pc: u32) { f_80252f4c_b(c, pc) }
#[inline(always)]
fn f_80252f4c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f4c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f4c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f4c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f4c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage057 (0x80252f54, 8 bytes)
#[inline(never)]
pub fn f_80252f54(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f54) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f54); f_80252f54_at(c, 0x80252f54); crate::prof::exit(c, 0x80252f54); return; }
    }
    f_80252f54_b(c, 0x80252f54);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f54_at(c: &mut Cpu, pc: u32) { f_80252f54_b(c, pc) }
#[inline(always)]
fn f_80252f54_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f54;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f54, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f54 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f54: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage054 (0x80252f5c, 8 bytes)
#[inline(never)]
pub fn f_80252f5c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f5c) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f5c); f_80252f5c_at(c, 0x80252f5c); crate::prof::exit(c, 0x80252f5c); return; }
    }
    f_80252f5c_b(c, 0x80252f5c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f5c_at(c: &mut Cpu, pc: u32) { f_80252f5c_b(c, pc) }
#[inline(always)]
fn f_80252f5c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f5c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f5c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f5c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f5c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage053 (0x80252f64, 8 bytes)
#[inline(never)]
pub fn f_80252f64(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f64) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f64); f_80252f64_at(c, 0x80252f64); crate::prof::exit(c, 0x80252f64); return; }
    }
    f_80252f64_b(c, 0x80252f64);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f64_at(c: &mut Cpu, pc: u32) { f_80252f64_b(c, pc) }
#[inline(always)]
fn f_80252f64_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f64;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f64, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f64 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f64: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage052 (0x80252f6c, 8 bytes)
#[inline(never)]
pub fn f_80252f6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f6c); f_80252f6c_at(c, 0x80252f6c); crate::prof::exit(c, 0x80252f6c); return; }
    }
    f_80252f6c_b(c, 0x80252f6c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f6c_at(c: &mut Cpu, pc: u32) { f_80252f6c_b(c, pc) }
#[inline(always)]
fn f_80252f6c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f6c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f6c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f6c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f6c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage051 (0x80252f74, 8 bytes)
#[inline(never)]
pub fn f_80252f74(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f74) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f74); f_80252f74_at(c, 0x80252f74); crate::prof::exit(c, 0x80252f74); return; }
    }
    f_80252f74_b(c, 0x80252f74);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f74_at(c: &mut Cpu, pc: u32) { f_80252f74_b(c, pc) }
#[inline(always)]
fn f_80252f74_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f74;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f74, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f74 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f74: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage050 (0x80252f7c, 8 bytes)
#[inline(never)]
pub fn f_80252f7c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f7c) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f7c); f_80252f7c_at(c, 0x80252f7c); crate::prof::exit(c, 0x80252f7c); return; }
    }
    f_80252f7c_b(c, 0x80252f7c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f7c_at(c: &mut Cpu, pc: u32) { f_80252f7c_b(c, pc) }
#[inline(always)]
fn f_80252f7c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f7c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f7c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f7c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f7c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage049 (0x80252f84, 8 bytes)
#[inline(never)]
pub fn f_80252f84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f84) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f84); f_80252f84_at(c, 0x80252f84); crate::prof::exit(c, 0x80252f84); return; }
    }
    f_80252f84_b(c, 0x80252f84);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f84_at(c: &mut Cpu, pc: u32) { f_80252f84_b(c, pc) }
#[inline(always)]
fn f_80252f84_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f84;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f84, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f84 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f84: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage048 (0x80252f8c, 132 bytes)
#[inline(never)]
pub fn f_80252f8c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80252f8c) { return; }
        if c.prof { crate::prof::enter(c, 0x80252f8c); f_80252f8c_at(c, 0x80252f8c); crate::prof::exit(c, 0x80252f8c); return; }
    }
    f_80252f8c_b(c, 0x80252f8c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80252f8c_at(c: &mut Cpu, pc: u32) { f_80252f8c_b(c, pc) }
#[inline(always)]
fn f_80252f8c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80252f8c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80252f8c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80252f8c => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80252fc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252fc8;
            }
            0x80252fc8 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80252fdc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80252fdc;
            }
            0x80252fdc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80252ff8; continue; }
                pc = 0x80252fe8;
            }
            0x80252fe8 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x80252ff8; continue; }
                pc = 0x80252ff4;
            }
            0x80252ff4 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x80252ff8;
            }
            0x80252ff8 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80252f8c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage047 (0x80253010, 8 bytes)
#[inline(never)]
pub fn f_80253010(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253010) { return; }
        if c.prof { crate::prof::enter(c, 0x80253010); f_80253010_at(c, 0x80253010); crate::prof::exit(c, 0x80253010); return; }
    }
    f_80253010_b(c, 0x80253010);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253010_at(c: &mut Cpu, pc: u32) { f_80253010_b(c, pc) }
#[inline(always)]
fn f_80253010_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253010;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253010, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253010 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253010: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage046 (0x80253018, 8 bytes)
#[inline(never)]
pub fn f_80253018(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253018) { return; }
        if c.prof { crate::prof::enter(c, 0x80253018); f_80253018_at(c, 0x80253018); crate::prof::exit(c, 0x80253018); return; }
    }
    f_80253018_b(c, 0x80253018);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253018_at(c: &mut Cpu, pc: u32) { f_80253018_b(c, pc) }
#[inline(always)]
fn f_80253018_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253018;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253018, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253018 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253018: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage045 (0x80253020, 132 bytes)
#[inline(never)]
pub fn f_80253020(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253020) { return; }
        if c.prof { crate::prof::enter(c, 0x80253020); f_80253020_at(c, 0x80253020); crate::prof::exit(c, 0x80253020); return; }
    }
    f_80253020_b(c, 0x80253020);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253020_at(c: &mut Cpu, pc: u32) { f_80253020_b(c, pc) }
#[inline(always)]
fn f_80253020_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253020;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253020, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253020 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x8025305c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025305c;
            }
            0x8025305c => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80253070; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253070;
            }
            0x80253070 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8025308c; continue; }
                pc = 0x8025307c;
            }
            0x8025307c => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x8025308c; continue; }
                pc = 0x80253088;
            }
            0x80253088 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x8025308c;
            }
            0x8025308c => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253020: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage044 (0x802530a4, 64 bytes)
#[inline(never)]
pub fn f_802530a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802530a4) { return; }
        if c.prof { crate::prof::enter(c, 0x802530a4); f_802530a4_at(c, 0x802530a4); crate::prof::exit(c, 0x802530a4); return; }
    }
    f_802530a4_b(c, 0x802530a4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802530a4_at(c: &mut Cpu, pc: u32) { f_802530a4_b(c, pc) }
#[inline(always)]
fn f_802530a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802530a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802530a4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802530a4 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.r[9] = 0x0;
                c.r[5] = c.r[0] | c.r[0];
                c.r[10] = 0x0;
                c.lr = 0x802530d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802530d0;
            }
            0x802530d0 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[3].rotate_left(1) & 0xfffffffe;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802530a4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage043 (0x802530e4, 132 bytes)
#[inline(never)]
pub fn f_802530e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802530e4) { return; }
        if c.prof { crate::prof::enter(c, 0x802530e4); f_802530e4_at(c, 0x802530e4); crate::prof::exit(c, 0x802530e4); return; }
    }
    f_802530e4_b(c, 0x802530e4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802530e4_at(c: &mut Cpu, pc: u32) { f_802530e4_b(c, pc) }
#[inline(always)]
fn f_802530e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802530e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802530e4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802530e4 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80253120; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253120;
            }
            0x80253120 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80253134; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253134;
            }
            0x80253134 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80253150; continue; }
                pc = 0x80253140;
            }
            0x80253140 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x80253150; continue; }
                pc = 0x8025314c;
            }
            0x8025314c => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x80253150;
            }
            0x80253150 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802530e4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage042 (0x80253168, 136 bytes)
#[inline(never)]
pub fn f_80253168(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253168) { return; }
        if c.prof { crate::prof::enter(c, 0x80253168); f_80253168_at(c, 0x80253168); crate::prof::exit(c, 0x80253168); return; }
    }
    f_80253168_b(c, 0x80253168);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253168_at(c: &mut Cpu, pc: u32) { f_80253168_b(c, pc) }
#[inline(always)]
fn f_80253168_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253168;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253168, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253168 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[6] | c.r[6];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x802531a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802531a4;
            }
            0x802531a4 => { ins += 6;
                c.r[0] = c.r[3] | c.r[3];
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[29] | c.r[29];
                c.r[5] = 0x21;
                c.r[31] = c.r[0] | c.r[0];
                c.lr = 0x802531bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802531bc;
            }
            0x802531bc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802531d8; continue; }
                pc = 0x802531c8;
            }
            0x802531c8 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xfa);
                if !cr_bit(cr, 2) { pc = 0x802531d8; continue; }
                pc = 0x802531d4;
            }
            0x802531d4 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x802531d8;
            }
            0x802531d8 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253168: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage041 (0x802531f0, 8 bytes)
#[inline(never)]
pub fn f_802531f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802531f0) { return; }
        if c.prof { crate::prof::enter(c, 0x802531f0); f_802531f0_at(c, 0x802531f0); crate::prof::exit(c, 0x802531f0); return; }
    }
    f_802531f0_b(c, 0x802531f0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802531f0_at(c: &mut Cpu, pc: u32) { f_802531f0_b(c, pc) }
#[inline(always)]
fn f_802531f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802531f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802531f0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802531f0 => { ins += 2;
                c.r[3] = 0x28;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802531f0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage040 (0x802531f8, 60 bytes)
#[inline(never)]
pub fn f_802531f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802531f8) { return; }
        if c.prof { crate::prof::enter(c, 0x802531f8); f_802531f8_at(c, 0x802531f8); crate::prof::exit(c, 0x802531f8); return; }
    }
    f_802531f8_b(c, 0x802531f8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802531f8_at(c: &mut Cpu, pc: u32) { f_802531f8_b(c, pc) }
#[inline(always)]
fn f_802531f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802531f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802531f8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802531f8 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.r[9] = 0x0;
                c.r[5] = c.r[0] | c.r[0];
                c.r[10] = 0x0;
                c.lr = 0x80253224; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253224;
            }
            0x80253224 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802531f8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage039 (0x80253234, 60 bytes)
#[inline(never)]
pub fn f_80253234(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253234) { return; }
        if c.prof { crate::prof::enter(c, 0x80253234); f_80253234_at(c, 0x80253234); crate::prof::exit(c, 0x80253234); return; }
    }
    f_80253234_b(c, 0x80253234);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253234_at(c: &mut Cpu, pc: u32) { f_80253234_b(c, pc) }
#[inline(always)]
fn f_80253234_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253234;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253234, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253234 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.r[9] = 0x0;
                c.r[5] = c.r[0] | c.r[0];
                c.r[10] = 0x0;
                c.lr = 0x80253260; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253260;
            }
            0x80253260 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253234: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage038 (0x80253270, 40 bytes)
#[inline(never)]
pub fn f_80253270(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253270) { return; }
        if c.prof { crate::prof::enter(c, 0x80253270); f_80253270_at(c, 0x80253270); crate::prof::exit(c, 0x80253270); return; }
    }
    f_80253270_b(c, 0x80253270);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253270_at(c: &mut Cpu, pc: u32) { f_80253270_b(c, pc) }
#[inline(always)]
fn f_80253270_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253270;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253270, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253270 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80253284; { c.steps = st; c.cycles = ins; c.cr = cr; f_802376ec(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253284;
            }
            0x80253284 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253270: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage037 (0x80253298, 40 bytes)
#[inline(never)]
pub fn f_80253298(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253298) { return; }
        if c.prof { crate::prof::enter(c, 0x80253298); f_80253298_at(c, 0x80253298); crate::prof::exit(c, 0x80253298); return; }
    }
    f_80253298_b(c, 0x80253298);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253298_at(c: &mut Cpu, pc: u32) { f_80253298_b(c, pc) }
#[inline(always)]
fn f_80253298_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253298;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253298, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253298 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x802532a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237664(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802532a8;
            }
            0x802532a8 => { ins += 7;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[0].wrapping_neg();
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253298: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage036 (0x802532c0, 132 bytes)
#[inline(never)]
pub fn f_802532c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802532c0) { return; }
        if c.prof { crate::prof::enter(c, 0x802532c0); f_802532c0_at(c, 0x802532c0); crate::prof::exit(c, 0x802532c0); return; }
    }
    f_802532c0_b(c, 0x802532c0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802532c0_at(c: &mut Cpu, pc: u32) { f_802532c0_b(c, pc) }
#[inline(always)]
fn f_802532c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802532c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802532c0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802532c0 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x802532fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802532fc;
            }
            0x802532fc => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80253310; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253310;
            }
            0x80253310 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8025332c; continue; }
                pc = 0x8025331c;
            }
            0x8025331c => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x8025332c; continue; }
                pc = 0x80253328;
            }
            0x80253328 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x8025332c;
            }
            0x8025332c => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802532c0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage035 (0x80253344, 8 bytes)
#[inline(never)]
pub fn f_80253344(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253344) { return; }
        if c.prof { crate::prof::enter(c, 0x80253344); f_80253344_at(c, 0x80253344); crate::prof::exit(c, 0x80253344); return; }
    }
    f_80253344_b(c, 0x80253344);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253344_at(c: &mut Cpu, pc: u32) { f_80253344_b(c, pc) }
#[inline(always)]
fn f_80253344_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253344;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253344, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253344 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253344: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage034 (0x8025334c, 132 bytes)
#[inline(never)]
pub fn f_8025334c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025334c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025334c); f_8025334c_at(c, 0x8025334c); crate::prof::exit(c, 0x8025334c); return; }
    }
    f_8025334c_b(c, 0x8025334c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025334c_at(c: &mut Cpu, pc: u32) { f_8025334c_b(c, pc) }
#[inline(always)]
fn f_8025334c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8025334c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8025334c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8025334c => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80253388; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253388;
            }
            0x80253388 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x8025339c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025339c;
            }
            0x8025339c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802533b8; continue; }
                pc = 0x802533a8;
            }
            0x802533a8 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x802533b8; continue; }
                pc = 0x802533b4;
            }
            0x802533b4 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x802533b8;
            }
            0x802533b8 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8025334c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage033 (0x802533d0, 8 bytes)
#[inline(never)]
pub fn f_802533d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802533d0) { return; }
        if c.prof { crate::prof::enter(c, 0x802533d0); f_802533d0_at(c, 0x802533d0); crate::prof::exit(c, 0x802533d0); return; }
    }
    f_802533d0_b(c, 0x802533d0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802533d0_at(c: &mut Cpu, pc: u32) { f_802533d0_b(c, pc) }
#[inline(always)]
fn f_802533d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802533d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802533d0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802533d0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802533d0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage032 (0x802533d8, 40 bytes)
#[inline(never)]
pub fn f_802533d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802533d8) { return; }
        if c.prof { crate::prof::enter(c, 0x802533d8); f_802533d8_at(c, 0x802533d8); crate::prof::exit(c, 0x802533d8); return; }
    }
    f_802533d8_b(c, 0x802533d8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802533d8_at(c: &mut Cpu, pc: u32) { f_802533d8_b(c, pc) }
#[inline(always)]
fn f_802533d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802533d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802533d8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802533d8 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x802533e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237664(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802533e8;
            }
            0x802533e8 => { ins += 7;
                c.r[0] = c.r[3].rotate_left(31) & 0x7fff;
                c.r[3] = c.r[0].wrapping_neg();
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802533d8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage031 (0x80253400, 132 bytes)
#[inline(never)]
pub fn f_80253400(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253400) { return; }
        if c.prof { crate::prof::enter(c, 0x80253400); f_80253400_at(c, 0x80253400); crate::prof::exit(c, 0x80253400); return; }
    }
    f_80253400_b(c, 0x80253400);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253400_at(c: &mut Cpu, pc: u32) { f_80253400_b(c, pc) }
#[inline(always)]
fn f_80253400_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253400;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253400, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253400 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x8025343c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025343c;
            }
            0x8025343c => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80253450; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253450;
            }
            0x80253450 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8025346c; continue; }
                pc = 0x8025345c;
            }
            0x8025345c => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x8025346c; continue; }
                pc = 0x80253468;
            }
            0x80253468 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x8025346c;
            }
            0x8025346c => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253400: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage030 (0x80253484, 8 bytes)
#[inline(never)]
pub fn f_80253484(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253484) { return; }
        if c.prof { crate::prof::enter(c, 0x80253484); f_80253484_at(c, 0x80253484); crate::prof::exit(c, 0x80253484); return; }
    }
    f_80253484_b(c, 0x80253484);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253484_at(c: &mut Cpu, pc: u32) { f_80253484_b(c, pc) }
#[inline(always)]
fn f_80253484_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253484;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253484, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253484 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253484: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage029 (0x8025348c, 64 bytes)
#[inline(never)]
pub fn f_8025348c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025348c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025348c); f_8025348c_at(c, 0x8025348c); crate::prof::exit(c, 0x8025348c); return; }
    }
    f_8025348c_b(c, 0x8025348c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025348c_at(c: &mut Cpu, pc: u32) { f_8025348c_b(c, pc) }
#[inline(always)]
fn f_8025348c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8025348c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8025348c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8025348c => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.r[9] = 0x0;
                c.r[5] = c.r[0] | c.r[0];
                c.r[10] = 0x0;
                c.lr = 0x802534b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802534b8;
            }
            0x802534b8 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[3].wrapping_mul(0x3);
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8025348c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage028 (0x802534cc, 8 bytes)
#[inline(never)]
pub fn f_802534cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802534cc) { return; }
        if c.prof { crate::prof::enter(c, 0x802534cc); f_802534cc_at(c, 0x802534cc); crate::prof::exit(c, 0x802534cc); return; }
    }
    f_802534cc_b(c, 0x802534cc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802534cc_at(c: &mut Cpu, pc: u32) { f_802534cc_b(c, pc) }
#[inline(always)]
fn f_802534cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802534cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802534cc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802534cc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802534cc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage027 (0x802534d4, 60 bytes)
#[inline(never)]
pub fn f_802534d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802534d4) { return; }
        if c.prof { crate::prof::enter(c, 0x802534d4); f_802534d4_at(c, 0x802534d4); crate::prof::exit(c, 0x802534d4); return; }
    }
    f_802534d4_b(c, 0x802534d4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802534d4_at(c: &mut Cpu, pc: u32) { f_802534d4_b(c, pc) }
#[inline(always)]
fn f_802534d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802534d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802534d4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802534d4 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.r[9] = 0x0;
                c.r[5] = c.r[0] | c.r[0];
                c.r[10] = 0x0;
                c.lr = 0x80253500; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253500;
            }
            0x80253500 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802534d4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage026 (0x80253510, 8 bytes)
#[inline(never)]
pub fn f_80253510(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253510) { return; }
        if c.prof { crate::prof::enter(c, 0x80253510); f_80253510_at(c, 0x80253510); crate::prof::exit(c, 0x80253510); return; }
    }
    f_80253510_b(c, 0x80253510);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253510_at(c: &mut Cpu, pc: u32) { f_80253510_b(c, pc) }
#[inline(always)]
fn f_80253510_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253510;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253510, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253510 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253510: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage025 (0x80253518, 8 bytes)
#[inline(never)]
pub fn f_80253518(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253518) { return; }
        if c.prof { crate::prof::enter(c, 0x80253518); f_80253518_at(c, 0x80253518); crate::prof::exit(c, 0x80253518); return; }
    }
    f_80253518_b(c, 0x80253518);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253518_at(c: &mut Cpu, pc: u32) { f_80253518_b(c, pc) }
#[inline(always)]
fn f_80253518_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253518;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253518, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253518 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253518: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage024 (0x80253520, 8 bytes)
#[inline(never)]
pub fn f_80253520(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253520) { return; }
        if c.prof { crate::prof::enter(c, 0x80253520); f_80253520_at(c, 0x80253520); crate::prof::exit(c, 0x80253520); return; }
    }
    f_80253520_b(c, 0x80253520);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253520_at(c: &mut Cpu, pc: u32) { f_80253520_b(c, pc) }
#[inline(always)]
fn f_80253520_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253520;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253520, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253520 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253520: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage023 (0x80253528, 8 bytes)
#[inline(never)]
pub fn f_80253528(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253528) { return; }
        if c.prof { crate::prof::enter(c, 0x80253528); f_80253528_at(c, 0x80253528); crate::prof::exit(c, 0x80253528); return; }
    }
    f_80253528_b(c, 0x80253528);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253528_at(c: &mut Cpu, pc: u32) { f_80253528_b(c, pc) }
#[inline(always)]
fn f_80253528_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253528;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253528, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253528 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253528: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage020 (0x80253530, 8 bytes)
#[inline(never)]
pub fn f_80253530(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253530) { return; }
        if c.prof { crate::prof::enter(c, 0x80253530); f_80253530_at(c, 0x80253530); crate::prof::exit(c, 0x80253530); return; }
    }
    f_80253530_b(c, 0x80253530);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253530_at(c: &mut Cpu, pc: u32) { f_80253530_b(c, pc) }
#[inline(always)]
fn f_80253530_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253530;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253530, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253530 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253530: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage019 (0x80253538, 8 bytes)
#[inline(never)]
pub fn f_80253538(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253538) { return; }
        if c.prof { crate::prof::enter(c, 0x80253538); f_80253538_at(c, 0x80253538); crate::prof::exit(c, 0x80253538); return; }
    }
    f_80253538_b(c, 0x80253538);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253538_at(c: &mut Cpu, pc: u32) { f_80253538_b(c, pc) }
#[inline(always)]
fn f_80253538_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253538;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253538, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253538 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253538: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage018 (0x80253540, 8 bytes)
#[inline(never)]
pub fn f_80253540(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253540) { return; }
        if c.prof { crate::prof::enter(c, 0x80253540); f_80253540_at(c, 0x80253540); crate::prof::exit(c, 0x80253540); return; }
    }
    f_80253540_b(c, 0x80253540);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253540_at(c: &mut Cpu, pc: u32) { f_80253540_b(c, pc) }
#[inline(always)]
fn f_80253540_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253540;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253540, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253540 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253540: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage017 (0x80253548, 132 bytes)
#[inline(never)]
pub fn f_80253548(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253548) { return; }
        if c.prof { crate::prof::enter(c, 0x80253548); f_80253548_at(c, 0x80253548); crate::prof::exit(c, 0x80253548); return; }
    }
    f_80253548_b(c, 0x80253548);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253548_at(c: &mut Cpu, pc: u32) { f_80253548_b(c, pc) }
#[inline(always)]
fn f_80253548_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253548;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253548, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253548 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80253584; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253584;
            }
            0x80253584 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80253598; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253598;
            }
            0x80253598 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802535b4; continue; }
                pc = 0x802535a4;
            }
            0x802535a4 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x802535b4; continue; }
                pc = 0x802535b0;
            }
            0x802535b0 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x802535b4;
            }
            0x802535b4 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253548: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage016 (0x802535cc, 8 bytes)
#[inline(never)]
pub fn f_802535cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802535cc) { return; }
        if c.prof { crate::prof::enter(c, 0x802535cc); f_802535cc_at(c, 0x802535cc); crate::prof::exit(c, 0x802535cc); return; }
    }
    f_802535cc_b(c, 0x802535cc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802535cc_at(c: &mut Cpu, pc: u32) { f_802535cc_b(c, pc) }
#[inline(always)]
fn f_802535cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802535cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802535cc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802535cc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802535cc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage013 (0x802535d4, 8 bytes)
#[inline(never)]
pub fn f_802535d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802535d4) { return; }
        if c.prof { crate::prof::enter(c, 0x802535d4); f_802535d4_at(c, 0x802535d4); crate::prof::exit(c, 0x802535d4); return; }
    }
    f_802535d4_b(c, 0x802535d4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802535d4_at(c: &mut Cpu, pc: u32) { f_802535d4_b(c, pc) }
#[inline(always)]
fn f_802535d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802535d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802535d4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802535d4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802535d4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage011 (0x802535dc, 8 bytes)
#[inline(never)]
pub fn f_802535dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802535dc) { return; }
        if c.prof { crate::prof::enter(c, 0x802535dc); f_802535dc_at(c, 0x802535dc); crate::prof::exit(c, 0x802535dc); return; }
    }
    f_802535dc_b(c, 0x802535dc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802535dc_at(c: &mut Cpu, pc: u32) { f_802535dc_b(c, pc) }
#[inline(always)]
fn f_802535dc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802535dc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802535dc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802535dc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802535dc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage010 (0x802535e4, 8 bytes)
#[inline(never)]
pub fn f_802535e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802535e4) { return; }
        if c.prof { crate::prof::enter(c, 0x802535e4); f_802535e4_at(c, 0x802535e4); crate::prof::exit(c, 0x802535e4); return; }
    }
    f_802535e4_b(c, 0x802535e4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802535e4_at(c: &mut Cpu, pc: u32) { f_802535e4_b(c, pc) }
#[inline(always)]
fn f_802535e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802535e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802535e4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802535e4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802535e4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage009 (0x802535ec, 8 bytes)
#[inline(never)]
pub fn f_802535ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802535ec) { return; }
        if c.prof { crate::prof::enter(c, 0x802535ec); f_802535ec_at(c, 0x802535ec); crate::prof::exit(c, 0x802535ec); return; }
    }
    f_802535ec_b(c, 0x802535ec);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802535ec_at(c: &mut Cpu, pc: u32) { f_802535ec_b(c, pc) }
#[inline(always)]
fn f_802535ec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802535ec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802535ec, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802535ec => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802535ec: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage008 (0x802535f4, 60 bytes)
#[inline(never)]
pub fn f_802535f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802535f4) { return; }
        if c.prof { crate::prof::enter(c, 0x802535f4); f_802535f4_at(c, 0x802535f4); crate::prof::exit(c, 0x802535f4); return; }
    }
    f_802535f4_b(c, 0x802535f4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802535f4_at(c: &mut Cpu, pc: u32) { f_802535f4_b(c, pc) }
#[inline(always)]
fn f_802535f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802535f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802535f4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802535f4 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.r[9] = 0x0;
                c.r[5] = c.r[0] | c.r[0];
                c.r[10] = 0x0;
                c.lr = 0x80253620; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253620;
            }
            0x80253620 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802535f4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage007 (0x80253630, 60 bytes)
#[inline(never)]
pub fn f_80253630(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253630) { return; }
        if c.prof { crate::prof::enter(c, 0x80253630); f_80253630_at(c, 0x80253630); crate::prof::exit(c, 0x80253630); return; }
    }
    f_80253630_b(c, 0x80253630);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253630_at(c: &mut Cpu, pc: u32) { f_80253630_b(c, pc) }
#[inline(always)]
fn f_80253630_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253630;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253630, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253630 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.r[9] = 0x0;
                c.r[5] = c.r[0] | c.r[0];
                c.r[10] = 0x0;
                c.lr = 0x8025365c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025365c;
            }
            0x8025365c => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253630: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage006 (0x8025366c, 132 bytes)
#[inline(never)]
pub fn f_8025366c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025366c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025366c); f_8025366c_at(c, 0x8025366c); crate::prof::exit(c, 0x8025366c); return; }
    }
    f_8025366c_b(c, 0x8025366c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025366c_at(c: &mut Cpu, pc: u32) { f_8025366c_b(c, pc) }
#[inline(always)]
fn f_8025366c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8025366c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8025366c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8025366c => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x802536a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802536a8;
            }
            0x802536a8 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x802536bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802536bc;
            }
            0x802536bc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802536d8; continue; }
                pc = 0x802536c8;
            }
            0x802536c8 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x802536d8; continue; }
                pc = 0x802536d4;
            }
            0x802536d4 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x802536d8;
            }
            0x802536d8 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8025366c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage005 (0x802536f0, 132 bytes)
#[inline(never)]
pub fn f_802536f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802536f0) { return; }
        if c.prof { crate::prof::enter(c, 0x802536f0); f_802536f0_at(c, 0x802536f0); crate::prof::exit(c, 0x802536f0); return; }
    }
    f_802536f0_b(c, 0x802536f0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802536f0_at(c: &mut Cpu, pc: u32) { f_802536f0_b(c, pc) }
#[inline(always)]
fn f_802536f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802536f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802536f0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802536f0 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x8025372c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025372c;
            }
            0x8025372c => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80253740; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253740;
            }
            0x80253740 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8025375c; continue; }
                pc = 0x8025374c;
            }
            0x8025374c => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x8025375c; continue; }
                pc = 0x80253758;
            }
            0x80253758 => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x8025375c;
            }
            0x8025375c => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802536f0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage004 (0x80253774, 132 bytes)
#[inline(never)]
pub fn f_80253774(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253774) { return; }
        if c.prof { crate::prof::enter(c, 0x80253774); f_80253774_at(c, 0x80253774); crate::prof::exit(c, 0x80253774); return; }
    }
    f_80253774_b(c, 0x80253774);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253774_at(c: &mut Cpu, pc: u32) { f_80253774_b(c, pc) }
#[inline(always)]
fn f_80253774_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253774;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253774, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253774 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x802537b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802537b0;
            }
            0x802537b0 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x802537c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802537c4;
            }
            0x802537c4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802537e0; continue; }
                pc = 0x802537d0;
            }
            0x802537d0 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x802537e0; continue; }
                pc = 0x802537dc;
            }
            0x802537dc => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x802537e0;
            }
            0x802537e0 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253774: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage003 (0x802537f8, 60 bytes)
#[inline(never)]
pub fn f_802537f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802537f8) { return; }
        if c.prof { crate::prof::enter(c, 0x802537f8); f_802537f8_at(c, 0x802537f8); crate::prof::exit(c, 0x802537f8); return; }
    }
    f_802537f8_b(c, 0x802537f8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802537f8_at(c: &mut Cpu, pc: u32) { f_802537f8_b(c, pc) }
#[inline(always)]
fn f_802537f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802537f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802537f8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802537f8 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.r[9] = 0x0;
                c.r[5] = c.r[0] | c.r[0];
                c.r[10] = 0x0;
                c.lr = 0x80253824; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253824;
            }
            0x80253824 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802537f8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage002 (0x80253834, 132 bytes)
#[inline(never)]
pub fn f_80253834(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253834) { return; }
        if c.prof { crate::prof::enter(c, 0x80253834); f_80253834_at(c, 0x80253834); crate::prof::exit(c, 0x80253834); return; }
    }
    f_80253834_b(c, 0x80253834);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253834_at(c: &mut Cpu, pc: u32) { f_80253834_b(c, pc) }
#[inline(always)]
fn f_80253834_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253834;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253834, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253834 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80253870; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253870;
            }
            0x80253870 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x21;
                c.lr = 0x80253884; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253884;
            }
            0x80253884 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802538a0; continue; }
                pc = 0x80253890;
            }
            0x80253890 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x802538a0; continue; }
                pc = 0x8025389c;
            }
            0x8025389c => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x802538a0;
            }
            0x802538a0 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253834: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage001 (0x802538b8, 8 bytes)
#[inline(never)]
pub fn f_802538b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802538b8) { return; }
        if c.prof { crate::prof::enter(c, 0x802538b8); f_802538b8_at(c, 0x802538b8); crate::prof::exit(c, 0x802538b8); return; }
    }
    f_802538b8_b(c, 0x802538b8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802538b8_at(c: &mut Cpu, pc: u32) { f_802538b8_b(c, pc) }
#[inline(always)]
fn f_802538b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802538b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802538b8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802538b8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802538b8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamage000 (0x802538c0, 136 bytes)
#[inline(never)]
pub fn f_802538c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802538c0) { return; }
        if c.prof { crate::prof::enter(c, 0x802538c0); f_802538c0_at(c, 0x802538c0); crate::prof::exit(c, 0x802538c0); return; }
    }
    f_802538c0_b(c, 0x802538c0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802538c0_at(c: &mut Cpu, pc: u32) { f_802538c0_b(c, pc) }
#[inline(always)]
fn f_802538c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802538c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802538c0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802538c0 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[6] | c.r[6];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x802538fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802538fc;
            }
            0x802538fc => { ins += 6;
                c.r[0] = c.r[3] | c.r[3];
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[29] | c.r[29];
                c.r[5] = 0x21;
                c.r[31] = c.r[0] | c.r[0];
                c.lr = 0x80253914; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253914;
            }
            0x80253914 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80253930; continue; }
                pc = 0x80253920;
            }
            0x80253920 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x39);
                if !cr_bit(cr, 2) { pc = 0x80253930; continue; }
                pc = 0x8025392c;
            }
            0x8025392c => { ins += 1;
                c.r[31] = c.r[31].rotate_left(1) & 0xfffffffe;
                pc = 0x80253930;
            }
            0x80253930 => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802538c0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaDamageNull (0x80253948, 8 bytes)
#[inline(never)]
pub fn f_80253948(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253948) { return; }
        if c.prof { crate::prof::enter(c, 0x80253948); f_80253948_at(c, 0x80253948); crate::prof::exit(c, 0x80253948); return; }
    }
    f_80253948_b(c, 0x80253948);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253948_at(c: &mut Cpu, pc: u32) { f_80253948_b(c, pc) }
#[inline(always)]
fn f_80253948_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253948;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253948, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253948 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253948: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit213 (0x80253950, 108 bytes)
#[inline(never)]
pub fn f_80253950(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253950) { return; }
        if c.prof { crate::prof::enter(c, 0x80253950); f_80253950_at(c, 0x80253950); crate::prof::exit(c, 0x80253950); return; }
    }
    f_80253950_b(c, 0x80253950);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253950_at(c: &mut Cpu, pc: u32) { f_80253950_b(c, pc) }
#[inline(always)]
fn f_80253950_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253950;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253950, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253950 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0xf;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8025397c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025397c;
            }
            0x8025397c => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x80253984; { c.steps = st; c.cycles = ins; c.cr = cr; f_80136428(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253984;
            }
            0x80253984 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x80253998; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237dbc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253998;
            }
            0x80253998 => { ins += 20;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253950: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit212 (0x802539bc, 196 bytes)
#[inline(never)]
pub fn f_802539bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802539bc) { return; }
        if c.prof { crate::prof::enter(c, 0x802539bc); f_802539bc_at(c, 0x802539bc); crate::prof::exit(c, 0x802539bc); return; }
    }
    f_802539bc_b(c, 0x802539bc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802539bc_at(c: &mut Cpu, pc: u32) { f_802539bc_b(c, pc) }
#[inline(always)]
fn f_802539bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802539bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802539bc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802539bc => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.lr = 0x802539e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235aa0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802539e0;
            }
            0x802539e0 => { ins += 4;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802539f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235910(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802539f0;
            }
            0x802539f0 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x80253a10; continue; }
                pc = 0x802539fc;
            }
            0x802539fc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x80253a10; continue; }
                pc = 0x80253a08;
            }
            0x80253a08 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253a6c; continue;
                pc = 0x80253a10;
            }
            0x80253a10 => { ins += 8;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x1;
                c.r[9] = 0x41;
                c.lr = 0x80253a30; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253a30;
            }
            0x80253a30 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80253a68; continue; }
                pc = 0x80253a38;
            }
            0x80253a38 => { ins += 8;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x3;
                c.r[9] = 0x41;
                c.lr = 0x80253a58; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253a58;
            }
            0x80253a58 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80253a68; continue; }
                pc = 0x80253a60;
            }
            0x80253a60 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253a6c; continue;
                pc = 0x80253a68;
            }
            0x80253a68 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80253a6c;
            }
            0x80253a6c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802539bc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit211 (0x80253a80, 196 bytes)
#[inline(never)]
pub fn f_80253a80(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253a80) { return; }
        if c.prof { crate::prof::enter(c, 0x80253a80); f_80253a80_at(c, 0x80253a80); crate::prof::exit(c, 0x80253a80); return; }
    }
    f_80253a80_b(c, 0x80253a80);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253a80_at(c: &mut Cpu, pc: u32) { f_80253a80_b(c, pc) }
#[inline(always)]
fn f_80253a80_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253a80;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253a80, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253a80 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.lr = 0x80253aa4; { c.steps = st; c.cycles = ins; c.cr = cr; f_802359d8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253aa4;
            }
            0x80253aa4 => { ins += 4;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80253ab4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235974(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253ab4;
            }
            0x80253ab4 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x80253ad4; continue; }
                pc = 0x80253ac0;
            }
            0x80253ac0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x80253ad4; continue; }
                pc = 0x80253acc;
            }
            0x80253acc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253b30; continue;
                pc = 0x80253ad4;
            }
            0x80253ad4 => { ins += 8;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x4;
                c.r[9] = 0x41;
                c.lr = 0x80253af4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253af4;
            }
            0x80253af4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80253b2c; continue; }
                pc = 0x80253afc;
            }
            0x80253afc => { ins += 8;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x5;
                c.r[9] = 0x41;
                c.lr = 0x80253b1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253b1c;
            }
            0x80253b1c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80253b2c; continue; }
                pc = 0x80253b24;
            }
            0x80253b24 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253b30; continue;
                pc = 0x80253b2c;
            }
            0x80253b2c => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80253b30;
            }
            0x80253b30 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253a80: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit210 (0x80253b44, 52 bytes)
#[inline(never)]
pub fn f_80253b44(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253b44) { return; }
        if c.prof { crate::prof::enter(c, 0x80253b44); f_80253b44_at(c, 0x80253b44); crate::prof::exit(c, 0x80253b44); return; }
    }
    f_80253b44_b(c, 0x80253b44);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253b44_at(c: &mut Cpu, pc: u32) { f_80253b44_b(c, pc) }
#[inline(always)]
fn f_80253b44_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253b44;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253b44, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253b44 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x39;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80253b58; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253b58;
            }
            0x80253b58 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253b44: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit209 (0x80253b78, 180 bytes)
#[inline(never)]
pub fn f_80253b78(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253b78) { return; }
        if c.prof { crate::prof::enter(c, 0x80253b78); f_80253b78_at(c, 0x80253b78); crate::prof::exit(c, 0x80253b78); return; }
    }
    f_80253b78_b(c, 0x80253b78);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253b78_at(c: &mut Cpu, pc: u32) { f_80253b78_b(c, pc) }
#[inline(always)]
fn f_80253b78_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253b78;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253b78, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253b78 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80253ba4; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253ba4;
            }
            0x80253ba4 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80253bc0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253bc0;
            }
            0x80253bc0 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80253bd0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253bd0;
            }
            0x80253bd0 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80253be4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253be4;
            }
            0x80253be4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80253bf4; continue; }
                pc = 0x80253bf0;
            }
            0x80253bf0 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80253bf4;
            }
            0x80253bf4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80253c04; continue; }
                pc = 0x80253bfc;
            }
            0x80253bfc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253c18; continue;
                pc = 0x80253c04;
            }
            0x80253c04 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80253c14; continue; }
                pc = 0x80253c0c;
            }
            0x80253c0c => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80253c18; continue;
                pc = 0x80253c14;
            }
            0x80253c14 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80253c18;
            }
            0x80253c18 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253b78: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit208 (0x80253c2c, 196 bytes)
#[inline(never)]
pub fn f_80253c2c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253c2c) { return; }
        if c.prof { crate::prof::enter(c, 0x80253c2c); f_80253c2c_at(c, 0x80253c2c); crate::prof::exit(c, 0x80253c2c); return; }
    }
    f_80253c2c_b(c, 0x80253c2c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253c2c_at(c: &mut Cpu, pc: u32) { f_80253c2c_b(c, pc) }
#[inline(always)]
fn f_80253c2c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253c2c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253c2c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253c2c => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.lr = 0x80253c50; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235aa0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253c50;
            }
            0x80253c50 => { ins += 4;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80253c60; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235a3c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253c60;
            }
            0x80253c60 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x80253c80; continue; }
                pc = 0x80253c6c;
            }
            0x80253c6c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x80253c80; continue; }
                pc = 0x80253c78;
            }
            0x80253c78 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253cdc; continue;
                pc = 0x80253c80;
            }
            0x80253c80 => { ins += 8;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x1;
                c.r[9] = 0x41;
                c.lr = 0x80253ca0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253ca0;
            }
            0x80253ca0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80253cd8; continue; }
                pc = 0x80253ca8;
            }
            0x80253ca8 => { ins += 8;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x2;
                c.r[9] = 0x41;
                c.lr = 0x80253cc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253cc8;
            }
            0x80253cc8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80253cd8; continue; }
                pc = 0x80253cd0;
            }
            0x80253cd0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253cdc; continue;
                pc = 0x80253cd8;
            }
            0x80253cd8 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80253cdc;
            }
            0x80253cdc => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253c2c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit207 (0x80253cf0, 180 bytes)
#[inline(never)]
pub fn f_80253cf0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253cf0) { return; }
        if c.prof { crate::prof::enter(c, 0x80253cf0); f_80253cf0_at(c, 0x80253cf0); crate::prof::exit(c, 0x80253cf0); return; }
    }
    f_80253cf0_b(c, 0x80253cf0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253cf0_at(c: &mut Cpu, pc: u32) { f_80253cf0_b(c, pc) }
#[inline(always)]
fn f_80253cf0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253cf0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253cf0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253cf0 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80253d1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253d1c;
            }
            0x80253d1c => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80253d38; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253d38;
            }
            0x80253d38 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80253d48; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253d48;
            }
            0x80253d48 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80253d5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253d5c;
            }
            0x80253d5c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80253d6c; continue; }
                pc = 0x80253d68;
            }
            0x80253d68 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80253d6c;
            }
            0x80253d6c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80253d7c; continue; }
                pc = 0x80253d74;
            }
            0x80253d74 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253d90; continue;
                pc = 0x80253d7c;
            }
            0x80253d7c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80253d8c; continue; }
                pc = 0x80253d84;
            }
            0x80253d84 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80253d90; continue;
                pc = 0x80253d8c;
            }
            0x80253d8c => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80253d90;
            }
            0x80253d90 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253cf0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit206 (0x80253da4, 196 bytes)
#[inline(never)]
pub fn f_80253da4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253da4) { return; }
        if c.prof { crate::prof::enter(c, 0x80253da4); f_80253da4_at(c, 0x80253da4); crate::prof::exit(c, 0x80253da4); return; }
    }
    f_80253da4_b(c, 0x80253da4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253da4_at(c: &mut Cpu, pc: u32) { f_80253da4_b(c, pc) }
#[inline(always)]
fn f_80253da4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253da4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253da4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253da4 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.lr = 0x80253dc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235a3c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253dc8;
            }
            0x80253dc8 => { ins += 4;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80253dd8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235974(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253dd8;
            }
            0x80253dd8 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x80253df8; continue; }
                pc = 0x80253de4;
            }
            0x80253de4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x80253df8; continue; }
                pc = 0x80253df0;
            }
            0x80253df0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253e54; continue;
                pc = 0x80253df8;
            }
            0x80253df8 => { ins += 8;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x2;
                c.r[9] = 0x41;
                c.lr = 0x80253e18; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253e18;
            }
            0x80253e18 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80253e50; continue; }
                pc = 0x80253e20;
            }
            0x80253e20 => { ins += 8;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x5;
                c.r[9] = 0x41;
                c.lr = 0x80253e40; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253e40;
            }
            0x80253e40 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80253e50; continue; }
                pc = 0x80253e48;
            }
            0x80253e48 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253e54; continue;
                pc = 0x80253e50;
            }
            0x80253e50 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80253e54;
            }
            0x80253e54 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253da4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit205 (0x80253e68, 192 bytes)
#[inline(never)]
pub fn f_80253e68(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253e68) { return; }
        if c.prof { crate::prof::enter(c, 0x80253e68); f_80253e68_at(c, 0x80253e68); crate::prof::exit(c, 0x80253e68); return; }
    }
    f_80253e68_b(c, 0x80253e68);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253e68_at(c: &mut Cpu, pc: u32) { f_80253e68_b(c, pc) }
#[inline(always)]
fn f_80253e68_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253e68;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253e68, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253e68 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[6] | c.r[6];
                c.r[28] = c.r[4] | c.r[4];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[5] | c.r[5];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80253e90; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235aa0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253e90;
            }
            0x80253e90 => { ins += 4;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80253ea0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235a3c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253ea0;
            }
            0x80253ea0 => { ins += 2;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80253eb8; continue; }
                pc = 0x80253ea8;
            }
            0x80253ea8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80253eb8; continue; }
                pc = 0x80253eb0;
            }
            0x80253eb0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253f14; continue;
                pc = 0x80253eb8;
            }
            0x80253eb8 => { ins += 8;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x90;
                c.r[8] = 0x1;
                c.r[9] = 0x1;
                c.lr = 0x80253ed8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253ed8;
            }
            0x80253ed8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80253f10; continue; }
                pc = 0x80253ee0;
            }
            0x80253ee0 => { ins += 8;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x90;
                c.r[8] = 0x2;
                c.r[9] = 0x1;
                c.lr = 0x80253f00; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253f00;
            }
            0x80253f00 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80253f10; continue; }
                pc = 0x80253f08;
            }
            0x80253f08 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253f14; continue;
                pc = 0x80253f10;
            }
            0x80253f10 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80253f14;
            }
            0x80253f14 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253e68: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit204 (0x80253f28, 180 bytes)
#[inline(never)]
pub fn f_80253f28(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253f28) { return; }
        if c.prof { crate::prof::enter(c, 0x80253f28); f_80253f28_at(c, 0x80253f28); crate::prof::exit(c, 0x80253f28); return; }
    }
    f_80253f28_b(c, 0x80253f28);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253f28_at(c: &mut Cpu, pc: u32) { f_80253f28_b(c, pc) }
#[inline(always)]
fn f_80253f28_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253f28;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253f28, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253f28 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80253f54; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253f54;
            }
            0x80253f54 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80253f70; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253f70;
            }
            0x80253f70 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80253f80; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253f80;
            }
            0x80253f80 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80253f94; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80253f94;
            }
            0x80253f94 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80253fa4; continue; }
                pc = 0x80253fa0;
            }
            0x80253fa0 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80253fa4;
            }
            0x80253fa4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80253fb4; continue; }
                pc = 0x80253fac;
            }
            0x80253fac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80253fc8; continue;
                pc = 0x80253fb4;
            }
            0x80253fb4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80253fc4; continue; }
                pc = 0x80253fbc;
            }
            0x80253fbc => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80253fc8; continue;
                pc = 0x80253fc4;
            }
            0x80253fc4 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80253fc8;
            }
            0x80253fc8 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253f28: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit203 (0x80253fdc, 240 bytes)
#[inline(never)]
pub fn f_80253fdc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80253fdc) { return; }
        if c.prof { crate::prof::enter(c, 0x80253fdc); f_80253fdc_at(c, 0x80253fdc); crate::prof::exit(c, 0x80253fdc); return; }
    }
    f_80253fdc_b(c, 0x80253fdc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80253fdc_at(c: &mut Cpu, pc: u32) { f_80253fdc_b(c, pc) }
#[inline(always)]
fn f_80253fdc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80253fdc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80253fdc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80253fdc => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.lr = 0x80254008; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235b04(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254008;
            }
            0x80254008 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x8025401c; continue; }
                pc = 0x80254014;
            }
            0x80254014 => { ins += 2;
                c.r[30] = 0xb;
                pc = 0x80254050; continue;
                pc = 0x8025401c;
            }
            0x8025401c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x8025402c; continue; }
                pc = 0x80254024;
            }
            0x80254024 => { ins += 2;
                c.r[30] = 0x5;
                pc = 0x80254050; continue;
                pc = 0x8025402c;
            }
            0x8025402c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8025403c; continue; }
                pc = 0x80254034;
            }
            0x80254034 => { ins += 2;
                c.r[30] = 0xa;
                pc = 0x80254050; continue;
                pc = 0x8025403c;
            }
            0x8025403c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if !cr_bit(cr, 2) { pc = 0x8025404c; continue; }
                pc = 0x80254044;
            }
            0x80254044 => { ins += 2;
                c.r[30] = 0xf;
                pc = 0x80254050; continue;
                pc = 0x8025404c;
            }
            0x8025404c => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80254050;
            }
            0x80254050 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80254068; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254068;
            }
            0x80254068 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025407c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025407c;
            }
            0x8025407c => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80254090; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254090;
            }
            0x80254090 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x802540a0; continue; }
                pc = 0x8025409c;
            }
            0x8025409c => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x802540a0;
            }
            0x802540a0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802540b0; continue; }
                pc = 0x802540a8;
            }
            0x802540a8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802540b8; continue;
                pc = 0x802540b0;
            }
            0x802540b0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802540b8;
            }
            0x802540b8 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80253fdc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit202 (0x802540cc, 180 bytes)
#[inline(never)]
pub fn f_802540cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802540cc) { return; }
        if c.prof { crate::prof::enter(c, 0x802540cc); f_802540cc_at(c, 0x802540cc); crate::prof::exit(c, 0x802540cc); return; }
    }
    f_802540cc_b(c, 0x802540cc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802540cc_at(c: &mut Cpu, pc: u32) { f_802540cc_b(c, pc) }
#[inline(always)]
fn f_802540cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802540cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802540cc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802540cc => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802540f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802540f8;
            }
            0x802540f8 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80254114; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254114;
            }
            0x80254114 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254124; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254124;
            }
            0x80254124 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80254138; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254138;
            }
            0x80254138 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80254148; continue; }
                pc = 0x80254144;
            }
            0x80254144 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80254148;
            }
            0x80254148 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80254158; continue; }
                pc = 0x80254150;
            }
            0x80254150 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025416c; continue;
                pc = 0x80254158;
            }
            0x80254158 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80254168; continue; }
                pc = 0x80254160;
            }
            0x80254160 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x8025416c; continue;
                pc = 0x80254168;
            }
            0x80254168 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x8025416c;
            }
            0x8025416c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802540cc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit201 (0x80254180, 52 bytes)
#[inline(never)]
pub fn f_80254180(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254180) { return; }
        if c.prof { crate::prof::enter(c, 0x80254180); f_80254180_at(c, 0x80254180); crate::prof::exit(c, 0x80254180); return; }
    }
    f_80254180_b(c, 0x80254180);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254180_at(c: &mut Cpu, pc: u32) { f_80254180_b(c, pc) }
#[inline(always)]
fn f_80254180_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254180;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254180, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254180 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x38;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80254194; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254194;
            }
            0x80254194 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254180: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit200 (0x802541b4, 180 bytes)
#[inline(never)]
pub fn f_802541b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802541b4) { return; }
        if c.prof { crate::prof::enter(c, 0x802541b4); f_802541b4_at(c, 0x802541b4); crate::prof::exit(c, 0x802541b4); return; }
    }
    f_802541b4_b(c, 0x802541b4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802541b4_at(c: &mut Cpu, pc: u32) { f_802541b4_b(c, pc) }
#[inline(always)]
fn f_802541b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802541b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802541b4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802541b4 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802541e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802541e0;
            }
            0x802541e0 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x802541fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802541fc;
            }
            0x802541fc => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025420c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025420c;
            }
            0x8025420c => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80254220; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254220;
            }
            0x80254220 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80254230; continue; }
                pc = 0x8025422c;
            }
            0x8025422c => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80254230;
            }
            0x80254230 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80254240; continue; }
                pc = 0x80254238;
            }
            0x80254238 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254254; continue;
                pc = 0x80254240;
            }
            0x80254240 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80254250; continue; }
                pc = 0x80254248;
            }
            0x80254248 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80254254; continue;
                pc = 0x80254250;
            }
            0x80254250 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80254254;
            }
            0x80254254 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802541b4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit199 (0x80254268, 504 bytes)
#[inline(never)]
pub fn f_80254268(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254268) { return; }
        if c.prof { crate::prof::enter(c, 0x80254268); f_80254268_at(c, 0x80254268); crate::prof::exit(c, 0x80254268); return; }
    }
    f_80254268_b(c, 0x80254268);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254268_at(c: &mut Cpu, pc: u32) { f_80254268_b(c, pc) }
#[inline(never)]
fn f_80254268_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254268;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254268, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254268 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[31] = c.r[6] | c.r[6];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[3] = 0x2;
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80254294; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254294;
            }
            0x80254294 => { ins += 6;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x14;
                c.lr = 0x802542ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802542ac;
            }
            0x802542ac => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802542c0; continue; }
                pc = 0x802542b8;
            }
            0x802542b8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025444c; continue;
                pc = 0x802542c0;
            }
            0x802542c0 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x11;
                c.lr = 0x802542d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802542d0;
            }
            0x802542d0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8025439c; continue; }
                pc = 0x802542dc;
            }
            0x802542dc => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x14;
                c.lr = 0x802542ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802542ec;
            }
            0x802542ec => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80254300; continue; }
                pc = 0x802542f8;
            }
            0x802542f8 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x802543a0; continue;
                pc = 0x80254300;
            }
            0x80254300 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x7;
                c.lr = 0x80254310; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254310;
            }
            0x80254310 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8025439c; continue; }
                pc = 0x8025431c;
            }
            0x8025431c => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0xf;
                c.lr = 0x8025432c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025432c;
            }
            0x8025432c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8025439c; continue; }
                pc = 0x80254338;
            }
            0x80254338 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x48;
                c.lr = 0x80254348; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254348;
            }
            0x80254348 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8025439c; continue; }
                pc = 0x80254354;
            }
            0x80254354 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x29;
                c.lr = 0x80254364; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254364;
            }
            0x80254364 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8025439c; continue; }
                pc = 0x80254370;
            }
            0x80254370 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x28;
                c.lr = 0x80254380; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254380;
            }
            0x80254380 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8025439c; continue; }
                pc = 0x8025438c;
            }
            0x8025438c => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0xc;
                c.lr = 0x8025439c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025439c;
            }
            0x8025439c => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x802543a0;
            }
            0x802543a0 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802543b0; continue; }
                pc = 0x802543a8;
            }
            0x802543a8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025444c; continue;
                pc = 0x802543b0;
            }
            0x802543b0 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x14;
                c.lr = 0x802543c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802543c0;
            }
            0x802543c0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802543d4; continue; }
                pc = 0x802543cc;
            }
            0x802543cc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025444c; continue;
                pc = 0x802543d4;
            }
            0x802543d4 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x9;
                c.lr = 0x802543e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802543e4;
            }
            0x802543e4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802543f8; continue; }
                pc = 0x802543f0;
            }
            0x802543f0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025444c; continue;
                pc = 0x802543f8;
            }
            0x802543f8 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[31] | c.r[31];
                c.r[7] = 0x0;
                c.lr = 0x80254410; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254410;
            }
            0x80254410 => { ins += 4;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x4b;
                c.lr = 0x80254420; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f6e98(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254420;
            }
            0x80254420 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80254434; continue; }
                pc = 0x8025442c;
            }
            0x8025442c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025444c; continue;
                pc = 0x80254434;
            }
            0x80254434 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80254444; continue; }
                pc = 0x8025443c;
            }
            0x8025443c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025444c; continue;
                pc = 0x80254444;
            }
            0x80254444 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025444c;
            }
            0x8025444c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254268: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit198 (0x80254460, 180 bytes)
#[inline(never)]
pub fn f_80254460(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254460) { return; }
        if c.prof { crate::prof::enter(c, 0x80254460); f_80254460_at(c, 0x80254460); crate::prof::exit(c, 0x80254460); return; }
    }
    f_80254460_b(c, 0x80254460);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254460_at(c: &mut Cpu, pc: u32) { f_80254460_b(c, pc) }
#[inline(always)]
fn f_80254460_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254460;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254460, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254460 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025448c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025448c;
            }
            0x8025448c => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x802544a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802544a8;
            }
            0x802544a8 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802544b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802544b8;
            }
            0x802544b8 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x802544cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802544cc;
            }
            0x802544cc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x802544dc; continue; }
                pc = 0x802544d8;
            }
            0x802544d8 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x802544dc;
            }
            0x802544dc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802544ec; continue; }
                pc = 0x802544e4;
            }
            0x802544e4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254500; continue;
                pc = 0x802544ec;
            }
            0x802544ec => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x802544fc; continue; }
                pc = 0x802544f4;
            }
            0x802544f4 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80254500; continue;
                pc = 0x802544fc;
            }
            0x802544fc => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80254500;
            }
            0x80254500 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254460: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit197 (0x80254514, 180 bytes)
#[inline(never)]
pub fn f_80254514(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254514) { return; }
        if c.prof { crate::prof::enter(c, 0x80254514); f_80254514_at(c, 0x80254514); crate::prof::exit(c, 0x80254514); return; }
    }
    f_80254514_b(c, 0x80254514);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254514_at(c: &mut Cpu, pc: u32) { f_80254514_b(c, pc) }
#[inline(always)]
fn f_80254514_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254514;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254514, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254514 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254540; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254540;
            }
            0x80254540 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025455c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025455c;
            }
            0x8025455c => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025456c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025456c;
            }
            0x8025456c => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80254580; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254580;
            }
            0x80254580 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80254590; continue; }
                pc = 0x8025458c;
            }
            0x8025458c => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80254590;
            }
            0x80254590 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802545a0; continue; }
                pc = 0x80254598;
            }
            0x80254598 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802545b4; continue;
                pc = 0x802545a0;
            }
            0x802545a0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x802545b0; continue; }
                pc = 0x802545a8;
            }
            0x802545a8 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x802545b4; continue;
                pc = 0x802545b0;
            }
            0x802545b0 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802545b4;
            }
            0x802545b4 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254514: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit196 (0x802545c8, 176 bytes)
#[inline(never)]
pub fn f_802545c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802545c8) { return; }
        if c.prof { crate::prof::enter(c, 0x802545c8); f_802545c8_at(c, 0x802545c8); crate::prof::exit(c, 0x802545c8); return; }
    }
    f_802545c8_b(c, 0x802545c8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802545c8_at(c: &mut Cpu, pc: u32) { f_802545c8_b(c, pc) }
#[inline(always)]
fn f_802545c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802545c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802545c8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802545c8 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802545f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802545f4;
            }
            0x802545f4 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80254614; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254614;
            }
            0x80254614 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254628; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254628;
            }
            0x80254628 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8025463c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025463c;
            }
            0x8025463c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x8025464c; continue; }
                pc = 0x80254648;
            }
            0x80254648 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x8025464c;
            }
            0x8025464c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8025465c; continue; }
                pc = 0x80254654;
            }
            0x80254654 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254664; continue;
                pc = 0x8025465c;
            }
            0x8025465c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80254664;
            }
            0x80254664 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802545c8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit195 (0x80254678, 8 bytes)
#[inline(never)]
pub fn f_80254678(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254678) { return; }
        if c.prof { crate::prof::enter(c, 0x80254678); f_80254678_at(c, 0x80254678); crate::prof::exit(c, 0x80254678); return; }
    }
    f_80254678_b(c, 0x80254678);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254678_at(c: &mut Cpu, pc: u32) { f_80254678_b(c, pc) }
#[inline(always)]
fn f_80254678_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254678;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254678, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254678 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254678: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit194 (0x80254680, 56 bytes)
#[inline(never)]
pub fn f_80254680(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254680) { return; }
        if c.prof { crate::prof::enter(c, 0x80254680); f_80254680_at(c, 0x80254680); crate::prof::exit(c, 0x80254680); return; }
    }
    f_80254680_b(c, 0x80254680);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254680_at(c: &mut Cpu, pc: u32) { f_80254680_b(c, pc) }
#[inline(always)]
fn f_80254680_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254680;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254680, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254680 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x28;
                c.r[4] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80254698; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254698;
            }
            0x80254698 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254680: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit193 (0x802546b8, 48 bytes)
#[inline(never)]
pub fn f_802546b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802546b8) { return; }
        if c.prof { crate::prof::enter(c, 0x802546b8); f_802546b8_at(c, 0x802546b8); crate::prof::exit(c, 0x802546b8); return; }
    }
    f_802546b8_b(c, 0x802546b8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802546b8_at(c: &mut Cpu, pc: u32) { f_802546b8_b(c, pc) }
#[inline(always)]
fn f_802546b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802546b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802546b8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802546b8 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x802546c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023720c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802546c8;
            }
            0x802546c8 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0xff;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802546b8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit192 (0x802546e8, 296 bytes)
#[inline(never)]
pub fn f_802546e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802546e8) { return; }
        if c.prof { crate::prof::enter(c, 0x802546e8); f_802546e8_at(c, 0x802546e8); crate::prof::exit(c, 0x802546e8); return; }
    }
    f_802546e8_b(c, 0x802546e8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802546e8_at(c: &mut Cpu, pc: u32) { f_802546e8_b(c, pc) }
#[inline(always)]
fn f_802546e8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802546e8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802546e8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802546e8 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                c.r[5] = c.r[1].wrapping_add(0x1c);
                { let mut ea = c.r[1].wrapping_add(0x5c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[27] = c.r[4] | c.r[4];
                c.lr = 0x80254710; { c.steps = st; c.cycles = ins; c.cr = cr; f_802367cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254710;
            }
            0x80254710 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[1].wrapping_add(0x30);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x8025472c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1c18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025472c;
            }
            0x8025472c => { ins += 5;
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x27;
                c.lr = 0x80254740; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254740;
            }
            0x80254740 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80254754; continue; }
                pc = 0x8025474c;
            }
            0x8025474c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802547fc; continue;
                pc = 0x80254754;
            }
            0x80254754 => { ins += 4;
                c.r[27] = c.r[1].wrapping_add(0x30);
                c.r[28] = c.r[28].rotate_left(0) & 0xffff;
                c.r[30] = 0x0;
                pc = 0x802547ec; continue;
                pc = 0x80254764;
            }
            0x80254764 => { ins += 4;
                c.r[0] = c.r[30].rotate_left(2) & 0x3fffc;
                c.r[4] = c.lwz_t(ins, c.r[27].wrapping_add(c.r[0]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { pc = 0x802547e8; continue; }
                pc = 0x80254774;
            }
            0x80254774 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80254788; { c.steps = st; c.cycles = ins; c.cr = cr; f_802367cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254788;
            }
            0x80254788 => { ins += 6;
                c.r[5] = c.r[1].wrapping_add(0x1c);
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[8] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[10] = 0x0;
                pc = 0x802547dc; continue;
                pc = 0x802547a0;
            }
            0x802547a0 => { ins += 3;
                c.r[7] = c.r[10].rotate_left(1) & 0x1fffe;
                c.r[9] = 0x0;
                pc = 0x802547cc; continue;
                pc = 0x802547ac;
            }
            0x802547ac => { ins += 5;
                c.r[3] = c.r[9].rotate_left(1) & 0x1fffe;
                c.r[6] = c.lhz_t(ins, c.r[8].wrapping_add(c.r[7]));
                c.r[3] = c.lhz_t(ins, c.r[5].wrapping_add(c.r[3]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x802547c8; continue; }
                pc = 0x802547c0;
            }
            0x802547c0 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x802547fc; continue;
                pc = 0x802547c8;
            }
            0x802547c8 => { ins += 1;
                c.r[9] = c.r[9].wrapping_add(0x1);
                pc = 0x802547cc;
            }
            0x802547cc => { ins += 3;
                c.r[3] = c.r[9].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[4]);
                if cr_bit(cr, 0) { pc = 0x802547ac; continue; }
                pc = 0x802547d8;
            }
            0x802547d8 => { ins += 1;
                c.r[10] = c.r[10].wrapping_add(0x1);
                pc = 0x802547dc;
            }
            0x802547dc => { ins += 3;
                c.r[3] = c.r[10].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x802547a0; continue; }
                pc = 0x802547e8;
            }
            0x802547e8 => { ins += 1;
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x802547ec;
            }
            0x802547ec => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x80254764; continue; }
                pc = 0x802547f8;
            }
            0x802547f8 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x802547fc;
            }
            0x802547fc => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x5c); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x74));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802546e8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit191 (0x80254810, 200 bytes)
#[inline(never)]
pub fn f_80254810(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254810) { return; }
        if c.prof { crate::prof::enter(c, 0x80254810); f_80254810_at(c, 0x80254810); crate::prof::exit(c, 0x80254810); return; }
    }
    f_80254810_b(c, 0x80254810);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254810_at(c: &mut Cpu, pc: u32) { f_80254810_b(c, pc) }
#[inline(always)]
fn f_80254810_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254810;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254810, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254810 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x10000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0xffffffff);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.lr = 0x8025483c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025483c;
            }
            0x8025483c => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80254850; { c.steps = st; c.cycles = ins; c.cr = cr; f_80229934(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254850;
            }
            0x80254850 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80254864; continue; }
                pc = 0x8025485c;
            }
            0x8025485c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802548c4; continue;
                pc = 0x80254864;
            }
            0x80254864 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x19;
                c.lr = 0x80254874; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254874;
            }
            0x80254874 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80254888; continue; }
                pc = 0x80254880;
            }
            0x80254880 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802548c4; continue;
                pc = 0x80254888;
            }
            0x80254888 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x19;
                c.lr = 0x80254898; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254898;
            }
            0x80254898 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802548ac; continue; }
                pc = 0x802548a4;
            }
            0x802548a4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802548c4; continue;
                pc = 0x802548ac;
            }
            0x802548ac => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802548bc; continue; }
                pc = 0x802548b4;
            }
            0x802548b4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802548c4; continue;
                pc = 0x802548bc;
            }
            0x802548bc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802548c4;
            }
            0x802548c4 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254810: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit190 (0x802548d8, 180 bytes)
#[inline(never)]
pub fn f_802548d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802548d8) { return; }
        if c.prof { crate::prof::enter(c, 0x802548d8); f_802548d8_at(c, 0x802548d8); crate::prof::exit(c, 0x802548d8); return; }
    }
    f_802548d8_b(c, 0x802548d8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802548d8_at(c: &mut Cpu, pc: u32) { f_802548d8_b(c, pc) }
#[inline(always)]
fn f_802548d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802548d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802548d8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802548d8 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254904; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254904;
            }
            0x80254904 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80254920; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254920;
            }
            0x80254920 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254930; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254930;
            }
            0x80254930 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80254944; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254944;
            }
            0x80254944 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80254954; continue; }
                pc = 0x80254950;
            }
            0x80254950 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80254954;
            }
            0x80254954 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80254964; continue; }
                pc = 0x8025495c;
            }
            0x8025495c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254978; continue;
                pc = 0x80254964;
            }
            0x80254964 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80254974; continue; }
                pc = 0x8025496c;
            }
            0x8025496c => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80254978; continue;
                pc = 0x80254974;
            }
            0x80254974 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80254978;
            }
            0x80254978 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802548d8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit189 (0x8025498c, 228 bytes)
#[inline(never)]
pub fn f_8025498c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025498c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025498c); f_8025498c_at(c, 0x8025498c); crate::prof::exit(c, 0x8025498c); return; }
    }
    f_8025498c_b(c, 0x8025498c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025498c_at(c: &mut Cpu, pc: u32) { f_8025498c_b(c, pc) }
#[inline(always)]
fn f_8025498c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8025498c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8025498c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8025498c => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[26] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802549b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802549b8;
            }
            0x802549b8 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x802549cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_802376ec(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802549cc;
            }
            0x802549cc => { ins += 4;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x802549dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_802376ec(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802549dc;
            }
            0x802549dc => { ins += 4;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 1) { pc = 0x802549f4; continue; }
                pc = 0x802549ec;
            }
            0x802549ec => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254a5c; continue;
                pc = 0x802549f4;
            }
            0x802549f4 => { ins += 6;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80254a0c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254a0c;
            }
            0x80254a0c => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254a20; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254a20;
            }
            0x80254a20 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80254a34; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254a34;
            }
            0x80254a34 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80254a44; continue; }
                pc = 0x80254a40;
            }
            0x80254a40 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80254a44;
            }
            0x80254a44 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80254a54; continue; }
                pc = 0x80254a4c;
            }
            0x80254a4c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254a5c; continue;
                pc = 0x80254a54;
            }
            0x80254a54 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80254a5c;
            }
            0x80254a5c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8025498c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit188 (0x80254a70, 180 bytes)
#[inline(never)]
pub fn f_80254a70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254a70) { return; }
        if c.prof { crate::prof::enter(c, 0x80254a70); f_80254a70_at(c, 0x80254a70); crate::prof::exit(c, 0x80254a70); return; }
    }
    f_80254a70_b(c, 0x80254a70);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254a70_at(c: &mut Cpu, pc: u32) { f_80254a70_b(c, pc) }
#[inline(always)]
fn f_80254a70_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254a70;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254a70, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254a70 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254a9c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254a9c;
            }
            0x80254a9c => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80254ab8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254ab8;
            }
            0x80254ab8 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254ac8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254ac8;
            }
            0x80254ac8 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80254adc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254adc;
            }
            0x80254adc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80254aec; continue; }
                pc = 0x80254ae8;
            }
            0x80254ae8 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80254aec;
            }
            0x80254aec => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80254afc; continue; }
                pc = 0x80254af4;
            }
            0x80254af4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254b10; continue;
                pc = 0x80254afc;
            }
            0x80254afc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80254b0c; continue; }
                pc = 0x80254b04;
            }
            0x80254b04 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80254b10; continue;
                pc = 0x80254b0c;
            }
            0x80254b0c => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80254b10;
            }
            0x80254b10 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254a70: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit187 (0x80254b24, 376 bytes)
#[inline(never)]
pub fn f_80254b24(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254b24) { return; }
        if c.prof { crate::prof::enter(c, 0x80254b24); f_80254b24_at(c, 0x80254b24); crate::prof::exit(c, 0x80254b24); return; }
    }
    f_80254b24_b(c, 0x80254b24);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254b24_at(c: &mut Cpu, pc: u32) { f_80254b24_b(c, pc) }
#[inline(never)]
fn f_80254b24_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254b24;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254b24, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254b24 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[29] = c.r[4] | c.r[4];
                c.r[28] = c.r[3] | c.r[3];
                c.r[5] = 0x48;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80254b50; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254b50;
            }
            0x80254b50 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80254b64; continue; }
                pc = 0x80254b5c;
            }
            0x80254b5c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254c88; continue;
                pc = 0x80254b64;
            }
            0x80254b64 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0xf;
                c.lr = 0x80254b74; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254b74;
            }
            0x80254b74 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80254b88; continue; }
                pc = 0x80254b80;
            }
            0x80254b80 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254c88; continue;
                pc = 0x80254b88;
            }
            0x80254b88 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x80254b98; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254b98;
            }
            0x80254b98 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80254bac; continue; }
                pc = 0x80254ba4;
            }
            0x80254ba4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254c88; continue;
                pc = 0x80254bac;
            }
            0x80254bac => { ins += 3;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x2;
                c.lr = 0x80254bb8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254bb8;
            }
            0x80254bb8 => { ins += 2;
                c.r[4] = 0x4b;
                c.lr = 0x80254bc0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f6e98(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254bc0;
            }
            0x80254bc0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80254bd4; continue; }
                pc = 0x80254bcc;
            }
            0x80254bcc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254c88; continue;
                pc = 0x80254bd4;
            }
            0x80254bd4 => { ins += 6;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x80254bec; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254bec;
            }
            0x80254bec => { ins += 4;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80254bfc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025cc90(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254bfc;
            }
            0x80254bfc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80254c10; continue; }
                pc = 0x80254c08;
            }
            0x80254c08 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254c88; continue;
                pc = 0x80254c10;
            }
            0x80254c10 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x26;
                c.lr = 0x80254c20; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254c20;
            }
            0x80254c20 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80254c34; continue; }
                pc = 0x80254c2c;
            }
            0x80254c2c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254c88; continue;
                pc = 0x80254c34;
            }
            0x80254c34 => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80254c40; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237310(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254c40;
            }
            0x80254c40 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80254c50; continue; }
                pc = 0x80254c48;
            }
            0x80254c48 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254c88; continue;
                pc = 0x80254c50;
            }
            0x80254c50 => { ins += 3;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x8;
                c.lr = 0x80254c5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80229704(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254c5c;
            }
            0x80254c5c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80254c70; continue; }
                pc = 0x80254c68;
            }
            0x80254c68 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254c88; continue;
                pc = 0x80254c70;
            }
            0x80254c70 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80254c80; continue; }
                pc = 0x80254c78;
            }
            0x80254c78 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254c88; continue;
                pc = 0x80254c80;
            }
            0x80254c80 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80254c88;
            }
            0x80254c88 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254b24: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit186 (0x80254c9c, 176 bytes)
#[inline(never)]
pub fn f_80254c9c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254c9c) { return; }
        if c.prof { crate::prof::enter(c, 0x80254c9c); f_80254c9c_at(c, 0x80254c9c); crate::prof::exit(c, 0x80254c9c); return; }
    }
    f_80254c9c_b(c, 0x80254c9c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254c9c_at(c: &mut Cpu, pc: u32) { f_80254c9c_b(c, pc) }
#[inline(always)]
fn f_80254c9c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254c9c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254c9c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254c9c => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254cc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254cc8;
            }
            0x80254cc8 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80254ce8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254ce8;
            }
            0x80254ce8 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254cfc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254cfc;
            }
            0x80254cfc => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80254d10; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254d10;
            }
            0x80254d10 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80254d20; continue; }
                pc = 0x80254d1c;
            }
            0x80254d1c => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80254d20;
            }
            0x80254d20 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80254d30; continue; }
                pc = 0x80254d28;
            }
            0x80254d28 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254d38; continue;
                pc = 0x80254d30;
            }
            0x80254d30 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80254d38;
            }
            0x80254d38 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254c9c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit185 (0x80254d4c, 180 bytes)
#[inline(never)]
pub fn f_80254d4c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254d4c) { return; }
        if c.prof { crate::prof::enter(c, 0x80254d4c); f_80254d4c_at(c, 0x80254d4c); crate::prof::exit(c, 0x80254d4c); return; }
    }
    f_80254d4c_b(c, 0x80254d4c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254d4c_at(c: &mut Cpu, pc: u32) { f_80254d4c_b(c, pc) }
#[inline(always)]
fn f_80254d4c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254d4c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254d4c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254d4c => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254d78; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254d78;
            }
            0x80254d78 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80254d94; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254d94;
            }
            0x80254d94 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254da4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254da4;
            }
            0x80254da4 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80254db8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254db8;
            }
            0x80254db8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80254dc8; continue; }
                pc = 0x80254dc4;
            }
            0x80254dc4 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80254dc8;
            }
            0x80254dc8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80254dd8; continue; }
                pc = 0x80254dd0;
            }
            0x80254dd0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254dec; continue;
                pc = 0x80254dd8;
            }
            0x80254dd8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80254de8; continue; }
                pc = 0x80254de0;
            }
            0x80254de0 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80254dec; continue;
                pc = 0x80254de8;
            }
            0x80254de8 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80254dec;
            }
            0x80254dec => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254d4c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit184 (0x80254e00, 44 bytes)
#[inline(never)]
pub fn f_80254e00(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254e00) { return; }
        if c.prof { crate::prof::enter(c, 0x80254e00); f_80254e00_at(c, 0x80254e00); crate::prof::exit(c, 0x80254e00); return; }
    }
    f_80254e00_b(c, 0x80254e00);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254e00_at(c: &mut Cpu, pc: u32) { f_80254e00_b(c, pc) }
#[inline(always)]
fn f_80254e00_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254e00;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254e00, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254e00 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80254e10; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236b98(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254e10;
            }
            0x80254e10 => { ins += 8;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254e00: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit183 (0x80254e2c, 8 bytes)
#[inline(never)]
pub fn f_80254e2c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254e2c) { return; }
        if c.prof { crate::prof::enter(c, 0x80254e2c); f_80254e2c_at(c, 0x80254e2c); crate::prof::exit(c, 0x80254e2c); return; }
    }
    f_80254e2c_b(c, 0x80254e2c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254e2c_at(c: &mut Cpu, pc: u32) { f_80254e2c_b(c, pc) }
#[inline(always)]
fn f_80254e2c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254e2c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254e2c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254e2c => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254e2c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit182 (0x80254e34, 180 bytes)
#[inline(never)]
pub fn f_80254e34(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254e34) { return; }
        if c.prof { crate::prof::enter(c, 0x80254e34); f_80254e34_at(c, 0x80254e34); crate::prof::exit(c, 0x80254e34); return; }
    }
    f_80254e34_b(c, 0x80254e34);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254e34_at(c: &mut Cpu, pc: u32) { f_80254e34_b(c, pc) }
#[inline(always)]
fn f_80254e34_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254e34;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254e34, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254e34 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254e60; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254e60;
            }
            0x80254e60 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80254e7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254e7c;
            }
            0x80254e7c => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80254e8c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254e8c;
            }
            0x80254e8c => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80254ea0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254ea0;
            }
            0x80254ea0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80254eb0; continue; }
                pc = 0x80254eac;
            }
            0x80254eac => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80254eb0;
            }
            0x80254eb0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80254ec0; continue; }
                pc = 0x80254eb8;
            }
            0x80254eb8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254ed4; continue;
                pc = 0x80254ec0;
            }
            0x80254ec0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80254ed0; continue; }
                pc = 0x80254ec8;
            }
            0x80254ec8 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80254ed4; continue;
                pc = 0x80254ed0;
            }
            0x80254ed0 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80254ed4;
            }
            0x80254ed4 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254e34: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit181 (0x80254ee8, 52 bytes)
#[inline(never)]
pub fn f_80254ee8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254ee8) { return; }
        if c.prof { crate::prof::enter(c, 0x80254ee8); f_80254ee8_at(c, 0x80254ee8); crate::prof::exit(c, 0x80254ee8); return; }
    }
    f_80254ee8_b(c, 0x80254ee8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254ee8_at(c: &mut Cpu, pc: u32) { f_80254ee8_b(c, pc) }
#[inline(always)]
fn f_80254ee8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254ee8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254ee8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254ee8 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x25;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80254efc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254efc;
            }
            0x80254efc => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254ee8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit180 (0x80254f1c, 56 bytes)
#[inline(never)]
pub fn f_80254f1c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254f1c) { return; }
        if c.prof { crate::prof::enter(c, 0x80254f1c); f_80254f1c_at(c, 0x80254f1c); crate::prof::exit(c, 0x80254f1c); return; }
    }
    f_80254f1c_b(c, 0x80254f1c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254f1c_at(c: &mut Cpu, pc: u32) { f_80254f1c_b(c, pc) }
#[inline(always)]
fn f_80254f1c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254f1c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254f1c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254f1c => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = c.r[4] | c.r[4];
                c.r[5] = 0x18;
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80254f38; { c.steps = st; c.cycles = ins; c.cr = cr; f_80215008(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254f38;
            }
            0x80254f38 => { ins += 8;
                c.r[0] = c.r[3].wrapping_neg();
                c.r[0] = c.r[0] & !c.r[3];
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254f1c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit179 (0x80254f54, 52 bytes)
#[inline(never)]
pub fn f_80254f54(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254f54) { return; }
        if c.prof { crate::prof::enter(c, 0x80254f54); f_80254f54_at(c, 0x80254f54); crate::prof::exit(c, 0x80254f54); return; }
    }
    f_80254f54_b(c, 0x80254f54);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254f54_at(c: &mut Cpu, pc: u32) { f_80254f54_b(c, pc) }
#[inline(always)]
fn f_80254f54_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254f54;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254f54, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254f54 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x35;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80254f68; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254f68;
            }
            0x80254f68 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254f54: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit178 (0x80254f88, 120 bytes)
#[inline(never)]
pub fn f_80254f88(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80254f88) { return; }
        if c.prof { crate::prof::enter(c, 0x80254f88); f_80254f88_at(c, 0x80254f88); crate::prof::exit(c, 0x80254f88); return; }
    }
    f_80254f88_b(c, 0x80254f88);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80254f88_at(c: &mut Cpu, pc: u32) { f_80254f88_b(c, pc) }
#[inline(always)]
fn f_80254f88_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80254f88;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80254f88, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80254f88 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x10000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0xffffffff);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.lr = 0x80254fac; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254fac;
            }
            0x80254fac => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x19;
                c.lr = 0x80254fc0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80254fc0;
            }
            0x80254fc0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80254fd4; continue; }
                pc = 0x80254fcc;
            }
            0x80254fcc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254fec; continue;
                pc = 0x80254fd4;
            }
            0x80254fd4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80254fe4; continue; }
                pc = 0x80254fdc;
            }
            0x80254fdc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80254fec; continue;
                pc = 0x80254fe4;
            }
            0x80254fe4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80254fec;
            }
            0x80254fec => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80254f88: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit177 (0x80255000, 240 bytes)
#[inline(never)]
pub fn f_80255000(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255000) { return; }
        if c.prof { crate::prof::enter(c, 0x80255000); f_80255000_at(c, 0x80255000); crate::prof::exit(c, 0x80255000); return; }
    }
    f_80255000_b(c, 0x80255000);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255000_at(c: &mut Cpu, pc: u32) { f_80255000_b(c, pc) }
#[inline(always)]
fn f_80255000_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255000;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255000, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255000 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[29] = c.r[6] | c.r[6];
                c.lr = 0x80255020; { c.steps = st; c.cycles = ins; c.cr = cr; f_802383a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255020;
            }
            0x80255020 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80255034; { c.steps = st; c.cycles = ins; c.cr = cr; f_802383a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255034;
            }
            0x80255034 => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x80255044; { c.steps = st; c.cycles = ins; c.cr = cr; f_80216048(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255044;
            }
            0x80255044 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80255054; continue; }
                pc = 0x8025504c;
            }
            0x8025504c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802550dc; continue;
                pc = 0x80255054;
            }
            0x80255054 => { ins += 2;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80255064; continue; }
                pc = 0x8025505c;
            }
            0x8025505c => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x802550ac; continue; }
                pc = 0x80255064;
            }
            0x80255064 => { ins += 3;
                c.r[3] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0xaf);
                if cr_bit(cr, 2) { pc = 0x802550ac; continue; }
                pc = 0x80255070;
            }
            0x80255070 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xaf);
                if cr_bit(cr, 2) { pc = 0x802550ac; continue; }
                pc = 0x8025507c;
            }
            0x8025507c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80255094; continue; }
                pc = 0x80255084;
            }
            0x80255084 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025508c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142984(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025508c;
            }
            0x8025508c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x802550ac; continue; }
                pc = 0x80255094;
            }
            0x80255094 => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x802550b4; continue; }
                pc = 0x8025509c;
            }
            0x8025509c => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802550a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142984(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802550a4;
            }
            0x802550a4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802550b4; continue; }
                pc = 0x802550ac;
            }
            0x802550ac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802550dc; continue;
                pc = 0x802550b4;
            }
            0x802550b4 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x3c;
                c.lr = 0x802550c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802550c4;
            }
            0x802550c4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802550d8; continue; }
                pc = 0x802550d0;
            }
            0x802550d0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802550dc; continue;
                pc = 0x802550d8;
            }
            0x802550d8 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802550dc;
            }
            0x802550dc => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255000: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit176 (0x802550f0, 180 bytes)
#[inline(never)]
pub fn f_802550f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802550f0) { return; }
        if c.prof { crate::prof::enter(c, 0x802550f0); f_802550f0_at(c, 0x802550f0); crate::prof::exit(c, 0x802550f0); return; }
    }
    f_802550f0_b(c, 0x802550f0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802550f0_at(c: &mut Cpu, pc: u32) { f_802550f0_b(c, pc) }
#[inline(always)]
fn f_802550f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802550f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802550f0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802550f0 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[3] = 0xe;
                c.lr = 0x80255110; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255110;
            }
            0x80255110 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x80255120; continue; }
                pc = 0x80255118;
            }
            0x80255118 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255190; continue;
                pc = 0x80255120;
            }
            0x80255120 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x19;
                c.r[6] = 0x0;
                c.lr = 0x80255134; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255134;
            }
            0x80255134 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x80255184; continue; }
                pc = 0x80255140;
            }
            0x80255140 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80255148; { c.steps = st; c.cycles = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255148;
            }
            0x80255148 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80255184; continue; }
                pc = 0x80255154;
            }
            0x80255154 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x32;
                c.lr = 0x80255164; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255164;
            }
            0x80255164 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80255184; continue; }
                pc = 0x8025516c;
            }
            0x8025516c => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x32;
                c.lr = 0x8025517c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025517c;
            }
            0x8025517c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8025518c; continue; }
                pc = 0x80255184;
            }
            0x80255184 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255190; continue;
                pc = 0x8025518c;
            }
            0x8025518c => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80255190;
            }
            0x80255190 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802550f0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit175 (0x802551a4, 116 bytes)
#[inline(never)]
pub fn f_802551a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802551a4) { return; }
        if c.prof { crate::prof::enter(c, 0x802551a4); f_802551a4_at(c, 0x802551a4); crate::prof::exit(c, 0x802551a4); return; }
    }
    f_802551a4_b(c, 0x802551a4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802551a4_at(c: &mut Cpu, pc: u32) { f_802551a4_b(c, pc) }
#[inline(always)]
fn f_802551a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802551a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802551a4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802551a4 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.lr = 0x802551c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802551c4;
            }
            0x802551c4 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x30;
                c.lr = 0x802551d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802551d8;
            }
            0x802551d8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802551ec; continue; }
                pc = 0x802551e4;
            }
            0x802551e4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255204; continue;
                pc = 0x802551ec;
            }
            0x802551ec => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802551fc; continue; }
                pc = 0x802551f4;
            }
            0x802551f4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255204; continue;
                pc = 0x802551fc;
            }
            0x802551fc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80255204;
            }
            0x80255204 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802551a4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit174 (0x80255218, 8 bytes)
#[inline(never)]
pub fn f_80255218(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255218) { return; }
        if c.prof { crate::prof::enter(c, 0x80255218); f_80255218_at(c, 0x80255218); crate::prof::exit(c, 0x80255218); return; }
    }
    f_80255218_b(c, 0x80255218);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255218_at(c: &mut Cpu, pc: u32) { f_80255218_b(c, pc) }
#[inline(always)]
fn f_80255218_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255218;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255218, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255218 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255218: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit173 (0x80255220, 168 bytes)
#[inline(never)]
pub fn f_80255220(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255220) { return; }
        if c.prof { crate::prof::enter(c, 0x80255220); f_80255220_at(c, 0x80255220); crate::prof::exit(c, 0x80255220); return; }
    }
    f_80255220_b(c, 0x80255220);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255220_at(c: &mut Cpu, pc: u32) { f_80255220_b(c, pc) }
#[inline(always)]
fn f_80255220_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255220;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255220, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255220 => { ins += 23;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[29] = c.r[6] | c.r[6];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0xf;
                c.r[6] = 0x0;
                c.lr = 0x80255254; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255254;
            }
            0x80255254 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8025525c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801363e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025525c;
            }
            0x8025525c => { ins += 6;
                c.r[4] = c.r[30] | c.r[30];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.lr = 0x80255274; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255274;
            }
            0x80255274 => { ins += 6;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.lr = 0x8025528c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025528c;
            }
            0x8025528c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 2) { pc = 0x802552b0; continue; }
                pc = 0x80255298;
            }
            0x80255298 => { ins += 5;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.lr = 0x802552ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023c530(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802552ac;
            }
            0x802552ac => { ins += 1;
                pc = 0x802552b4; continue;
                pc = 0x802552b0;
            }
            0x802552b0 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802552b4;
            }
            0x802552b4 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255220: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit172 (0x802552c8, 8 bytes)
#[inline(never)]
pub fn f_802552c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802552c8) { return; }
        if c.prof { crate::prof::enter(c, 0x802552c8); f_802552c8_at(c, 0x802552c8); crate::prof::exit(c, 0x802552c8); return; }
    }
    f_802552c8_b(c, 0x802552c8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802552c8_at(c: &mut Cpu, pc: u32) { f_802552c8_b(c, pc) }
#[inline(always)]
fn f_802552c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802552c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802552c8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802552c8 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802552c8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit171 (0x802552d0, 180 bytes)
#[inline(never)]
pub fn f_802552d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802552d0) { return; }
        if c.prof { crate::prof::enter(c, 0x802552d0); f_802552d0_at(c, 0x802552d0); crate::prof::exit(c, 0x802552d0); return; }
    }
    f_802552d0_b(c, 0x802552d0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802552d0_at(c: &mut Cpu, pc: u32) { f_802552d0_b(c, pc) }
#[inline(always)]
fn f_802552d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802552d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802552d0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802552d0 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802552fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802552fc;
            }
            0x802552fc => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80255318; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255318;
            }
            0x80255318 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255328; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255328;
            }
            0x80255328 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025533c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025533c;
            }
            0x8025533c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x8025534c; continue; }
                pc = 0x80255348;
            }
            0x80255348 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x8025534c;
            }
            0x8025534c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8025535c; continue; }
                pc = 0x80255354;
            }
            0x80255354 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255370; continue;
                pc = 0x8025535c;
            }
            0x8025535c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x8025536c; continue; }
                pc = 0x80255364;
            }
            0x80255364 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80255370; continue;
                pc = 0x8025536c;
            }
            0x8025536c => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80255370;
            }
            0x80255370 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802552d0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit170 (0x80255384, 180 bytes)
#[inline(never)]
pub fn f_80255384(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255384) { return; }
        if c.prof { crate::prof::enter(c, 0x80255384); f_80255384_at(c, 0x80255384); crate::prof::exit(c, 0x80255384); return; }
    }
    f_80255384_b(c, 0x80255384);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255384_at(c: &mut Cpu, pc: u32) { f_80255384_b(c, pc) }
#[inline(always)]
fn f_80255384_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255384;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255384, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255384 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802553b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802553b0;
            }
            0x802553b0 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x802553cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802553cc;
            }
            0x802553cc => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802553dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802553dc;
            }
            0x802553dc => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x802553f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802553f0;
            }
            0x802553f0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80255400; continue; }
                pc = 0x802553fc;
            }
            0x802553fc => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80255400;
            }
            0x80255400 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80255410; continue; }
                pc = 0x80255408;
            }
            0x80255408 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255424; continue;
                pc = 0x80255410;
            }
            0x80255410 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80255420; continue; }
                pc = 0x80255418;
            }
            0x80255418 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80255424; continue;
                pc = 0x80255420;
            }
            0x80255420 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80255424;
            }
            0x80255424 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255384: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit169 (0x80255438, 180 bytes)
#[inline(never)]
pub fn f_80255438(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255438) { return; }
        if c.prof { crate::prof::enter(c, 0x80255438); f_80255438_at(c, 0x80255438); crate::prof::exit(c, 0x80255438); return; }
    }
    f_80255438_b(c, 0x80255438);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255438_at(c: &mut Cpu, pc: u32) { f_80255438_b(c, pc) }
#[inline(always)]
fn f_80255438_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255438;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255438, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255438 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255464; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255464;
            }
            0x80255464 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80255480; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255480;
            }
            0x80255480 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255490; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255490;
            }
            0x80255490 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x802554a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802554a4;
            }
            0x802554a4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x802554b4; continue; }
                pc = 0x802554b0;
            }
            0x802554b0 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x802554b4;
            }
            0x802554b4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802554c4; continue; }
                pc = 0x802554bc;
            }
            0x802554bc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802554d8; continue;
                pc = 0x802554c4;
            }
            0x802554c4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x802554d4; continue; }
                pc = 0x802554cc;
            }
            0x802554cc => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x802554d8; continue;
                pc = 0x802554d4;
            }
            0x802554d4 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802554d8;
            }
            0x802554d8 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255438: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit168 (0x802554ec, 268 bytes)
#[inline(never)]
pub fn f_802554ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802554ec) { return; }
        if c.prof { crate::prof::enter(c, 0x802554ec); f_802554ec_at(c, 0x802554ec); crate::prof::exit(c, 0x802554ec); return; }
    }
    f_802554ec_b(c, 0x802554ec);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802554ec_at(c: &mut Cpu, pc: u32) { f_802554ec_b(c, pc) }
#[inline(always)]
fn f_802554ec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802554ec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802554ec, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802554ec => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[6] | c.r[6];
                c.r[27] = c.r[4] | c.r[4];
                c.r[26] = c.r[3] | c.r[3];
                c.r[28] = c.r[5] | c.r[5];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80255514; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235aa0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255514;
            }
            0x80255514 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80255524; { c.steps = st; c.cycles = ins; c.cr = cr; f_802359d8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255524;
            }
            0x80255524 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[28] | c.r[28];
                c.lr = 0x80255538; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025caa8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255538;
            }
            0x80255538 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8025554c; continue; }
                pc = 0x80255544;
            }
            0x80255544 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802555e4; continue;
                pc = 0x8025554c;
            }
            0x8025554c => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80255564; continue; }
                pc = 0x80255554;
            }
            0x80255554 => { ins += 2;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80255564; continue; }
                pc = 0x8025555c;
            }
            0x8025555c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802555e4; continue;
                pc = 0x80255564;
            }
            0x80255564 => { ins += 8;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[28] | c.r[28];
                c.r[7] = 0xa0;
                c.r[8] = 0x1;
                c.r[9] = 0x1;
                c.lr = 0x80255584; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255584;
            }
            0x80255584 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802555bc; continue; }
                pc = 0x8025558c;
            }
            0x8025558c => { ins += 8;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[28] | c.r[28];
                c.r[7] = 0xa0;
                c.r[8] = 0x4;
                c.r[9] = 0x1;
                c.lr = 0x802555ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802555ac;
            }
            0x802555ac => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802555bc; continue; }
                pc = 0x802555b4;
            }
            0x802555b4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802555e4; continue;
                pc = 0x802555bc;
            }
            0x802555bc => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x14;
                c.lr = 0x802555cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802555cc;
            }
            0x802555cc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802555e0; continue; }
                pc = 0x802555d8;
            }
            0x802555d8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802555e4; continue;
                pc = 0x802555e0;
            }
            0x802555e0 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802555e4;
            }
            0x802555e4 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802554ec: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit167 (0x802555f8, 552 bytes)
#[inline(never)]
pub fn f_802555f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802555f8) { return; }
        if c.prof { crate::prof::enter(c, 0x802555f8); f_802555f8_at(c, 0x802555f8); crate::prof::exit(c, 0x802555f8); return; }
    }
    f_802555f8_b(c, 0x802555f8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802555f8_at(c: &mut Cpu, pc: u32) { f_802555f8_b(c, pc) }
#[inline(never)]
fn f_802555f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802555f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802555f8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802555f8 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[29] = c.r[4] | c.r[4];
                c.r[28] = c.r[3] | c.r[3];
                c.r[5] = 0x14;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80255624; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255624;
            }
            0x80255624 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80255638; continue; }
                pc = 0x80255630;
            }
            0x80255630 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025580c; continue;
                pc = 0x80255638;
            }
            0x80255638 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x6;
                c.lr = 0x80255648; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255648;
            }
            0x80255648 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8025565c; continue; }
                pc = 0x80255654;
            }
            0x80255654 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025580c; continue;
                pc = 0x8025565c;
            }
            0x8025565c => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0xa;
                c.lr = 0x8025566c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237dbc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025566c;
            }
            0x8025566c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80255680; continue; }
                pc = 0x80255678;
            }
            0x80255678 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025580c; continue;
                pc = 0x80255680;
            }
            0x80255680 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x29;
                c.lr = 0x80255690; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255690;
            }
            0x80255690 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802556a4; continue; }
                pc = 0x8025569c;
            }
            0x8025569c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025580c; continue;
                pc = 0x802556a4;
            }
            0x802556a4 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x11;
                c.lr = 0x802556b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802556b4;
            }
            0x802556b4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80255780; continue; }
                pc = 0x802556c0;
            }
            0x802556c0 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x802556d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802556d0;
            }
            0x802556d0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80255780; continue; }
                pc = 0x802556dc;
            }
            0x802556dc => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x7;
                c.lr = 0x802556ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802556ec;
            }
            0x802556ec => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80255780; continue; }
                pc = 0x802556f8;
            }
            0x802556f8 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0xf;
                c.lr = 0x80255708; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255708;
            }
            0x80255708 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80255780; continue; }
                pc = 0x80255714;
            }
            0x80255714 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x48;
                c.lr = 0x80255724; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255724;
            }
            0x80255724 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80255780; continue; }
                pc = 0x80255730;
            }
            0x80255730 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x29;
                c.lr = 0x80255740; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255740;
            }
            0x80255740 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80255754; continue; }
                pc = 0x8025574c;
            }
            0x8025574c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80255784; continue;
                pc = 0x80255754;
            }
            0x80255754 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x28;
                c.lr = 0x80255764; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255764;
            }
            0x80255764 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80255780; continue; }
                pc = 0x80255770;
            }
            0x80255770 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0xc;
                c.lr = 0x80255780; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255780;
            }
            0x80255780 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x80255784;
            }
            0x80255784 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80255794; continue; }
                pc = 0x8025578c;
            }
            0x8025578c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025580c; continue;
                pc = 0x80255794;
            }
            0x80255794 => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x802557a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237310(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802557a0;
            }
            0x802557a0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802557b0; continue; }
                pc = 0x802557a8;
            }
            0x802557a8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025580c; continue;
                pc = 0x802557b0;
            }
            0x802557b0 => { ins += 6;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x802557c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802557c8;
            }
            0x802557c8 => { ins += 4;
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x2;
                c.lr = 0x802557d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802557d8;
            }
            0x802557d8 => { ins += 2;
                c.r[4] = 0x4b;
                c.lr = 0x802557e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f6e98(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802557e0;
            }
            0x802557e0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802557f4; continue; }
                pc = 0x802557ec;
            }
            0x802557ec => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025580c; continue;
                pc = 0x802557f4;
            }
            0x802557f4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80255804; continue; }
                pc = 0x802557fc;
            }
            0x802557fc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025580c; continue;
                pc = 0x80255804;
            }
            0x80255804 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025580c;
            }
            0x8025580c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802555f8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit166 (0x80255820, 536 bytes)
#[inline(never)]
pub fn f_80255820(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255820) { return; }
        if c.prof { crate::prof::enter(c, 0x80255820); f_80255820_at(c, 0x80255820); crate::prof::exit(c, 0x80255820); return; }
    }
    f_80255820_b(c, 0x80255820);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255820_at(c: &mut Cpu, pc: u32) { f_80255820_b(c, pc) }
#[inline(never)]
fn f_80255820_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255820;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255820, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255820 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3] | c.r[3];
                c.r[27] = c.r[6] | c.r[6];
                c.r[25] = c.r[4] | c.r[4];
                c.r[26] = c.r[5] | c.r[5];
                c.r[3] = 0x2;
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8025584c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025584c;
            }
            0x8025584c => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[28] = c.r[0] | c.r[0];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x80255860; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235910(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255860;
            }
            0x80255860 => { ins += 7;
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[25] | c.r[25];
                c.r[5] = c.r[26] | c.r[26];
                c.r[6] = c.r[27] | c.r[27];
                c.r[7] = 0x0;
                c.lr = 0x8025587c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025587c;
            }
            0x8025587c => { ins += 5;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x9;
                c.lr = 0x80255890; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255890;
            }
            0x80255890 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802558d8; continue; }
                pc = 0x8025589c;
            }
            0x8025589c => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x802558d8; continue; }
                pc = 0x802558a8;
            }
            0x802558a8 => { ins += 8;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[25] | c.r[25];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = c.r[26] | c.r[26];
                c.r[7] = 0x10;
                c.r[8] = 0x4;
                c.r[9] = 0x1;
                c.lr = 0x802558c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802558c8;
            }
            0x802558c8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802558d8; continue; }
                pc = 0x802558d0;
            }
            0x802558d0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255a24; continue;
                pc = 0x802558d8;
            }
            0x802558d8 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x14;
                c.lr = 0x802558e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802558e8;
            }
            0x802558e8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802558fc; continue; }
                pc = 0x802558f4;
            }
            0x802558f4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255a24; continue;
                pc = 0x802558fc;
            }
            0x802558fc => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x11;
                c.lr = 0x8025590c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025590c;
            }
            0x8025590c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802559d8; continue; }
                pc = 0x80255918;
            }
            0x80255918 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x14;
                c.lr = 0x80255928; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255928;
            }
            0x80255928 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8025593c; continue; }
                pc = 0x80255934;
            }
            0x80255934 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x802559dc; continue;
                pc = 0x8025593c;
            }
            0x8025593c => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x7;
                c.lr = 0x8025594c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025594c;
            }
            0x8025594c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802559d8; continue; }
                pc = 0x80255958;
            }
            0x80255958 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0xf;
                c.lr = 0x80255968; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255968;
            }
            0x80255968 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802559d8; continue; }
                pc = 0x80255974;
            }
            0x80255974 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x48;
                c.lr = 0x80255984; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255984;
            }
            0x80255984 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802559d8; continue; }
                pc = 0x80255990;
            }
            0x80255990 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x29;
                c.lr = 0x802559a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802559a0;
            }
            0x802559a0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802559d8; continue; }
                pc = 0x802559ac;
            }
            0x802559ac => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x28;
                c.lr = 0x802559bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802559bc;
            }
            0x802559bc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802559d8; continue; }
                pc = 0x802559c8;
            }
            0x802559c8 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0xc;
                c.lr = 0x802559d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802559d8;
            }
            0x802559d8 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x802559dc;
            }
            0x802559dc => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802559ec; continue; }
                pc = 0x802559e4;
            }
            0x802559e4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255a24; continue;
                pc = 0x802559ec;
            }
            0x802559ec => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x4b;
                c.lr = 0x802559f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f6e98(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802559f8;
            }
            0x802559f8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80255a0c; continue; }
                pc = 0x80255a04;
            }
            0x80255a04 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255a24; continue;
                pc = 0x80255a0c;
            }
            0x80255a0c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80255a1c; continue; }
                pc = 0x80255a14;
            }
            0x80255a14 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255a24; continue;
                pc = 0x80255a1c;
            }
            0x80255a1c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80255a24;
            }
            0x80255a24 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255820: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit165 (0x80255a38, 116 bytes)
#[inline(never)]
pub fn f_80255a38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255a38) { return; }
        if c.prof { crate::prof::enter(c, 0x80255a38); f_80255a38_at(c, 0x80255a38); crate::prof::exit(c, 0x80255a38); return; }
    }
    f_80255a38_b(c, 0x80255a38);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255a38_at(c: &mut Cpu, pc: u32) { f_80255a38_b(c, pc) }
#[inline(always)]
fn f_80255a38_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255a38;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255a38, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255a38 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.lr = 0x80255a58; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255a58;
            }
            0x80255a58 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x1b;
                c.lr = 0x80255a6c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255a6c;
            }
            0x80255a6c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80255a80; continue; }
                pc = 0x80255a78;
            }
            0x80255a78 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255a98; continue;
                pc = 0x80255a80;
            }
            0x80255a80 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80255a90; continue; }
                pc = 0x80255a88;
            }
            0x80255a88 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255a98; continue;
                pc = 0x80255a90;
            }
            0x80255a90 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80255a98;
            }
            0x80255a98 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255a38: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit164 (0x80255aac, 56 bytes)
#[inline(never)]
pub fn f_80255aac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255aac) { return; }
        if c.prof { crate::prof::enter(c, 0x80255aac); f_80255aac_at(c, 0x80255aac); crate::prof::exit(c, 0x80255aac); return; }
    }
    f_80255aac_b(c, 0x80255aac);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255aac_at(c: &mut Cpu, pc: u32) { f_80255aac_b(c, pc) }
#[inline(always)]
fn f_80255aac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255aac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255aac, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255aac => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80255ac4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235b04(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255ac4;
            }
            0x80255ac4 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x4, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255aac: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit162 (0x80255ae4, 104 bytes)
#[inline(never)]
pub fn f_80255ae4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255ae4) { return; }
        if c.prof { crate::prof::enter(c, 0x80255ae4); f_80255ae4_at(c, 0x80255ae4); crate::prof::exit(c, 0x80255ae4); return; }
    }
    f_80255ae4_b(c, 0x80255ae4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255ae4_at(c: &mut Cpu, pc: u32) { f_80255ae4_b(c, pc) }
#[inline(always)]
fn f_80255ae4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255ae4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255ae4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255ae4 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x2d;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x80255b04; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255b04;
            }
            0x80255b04 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80255b14; continue; }
                pc = 0x80255b0c;
            }
            0x80255b0c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255b38; continue;
                pc = 0x80255b14;
            }
            0x80255b14 => { ins += 5;
                { let ea = c.r[2].wrapping_add(0xffffafa8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x0;
                c.lr = 0x80255b28; { c.steps = st; c.cycles = ins; c.cr = cr; f_802373b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255b28;
            }
            0x80255b28 => { ins += 4;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                pc = 0x80255b38;
            }
            0x80255b38 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255ae4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit161 (0x80255b4c, 204 bytes)
#[inline(never)]
pub fn f_80255b4c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255b4c) { return; }
        if c.prof { crate::prof::enter(c, 0x80255b4c); f_80255b4c_at(c, 0x80255b4c); crate::prof::exit(c, 0x80255b4c); return; }
    }
    f_80255b4c_b(c, 0x80255b4c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255b4c_at(c: &mut Cpu, pc: u32) { f_80255b4c_b(c, pc) }
#[inline(always)]
fn f_80255b4c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255b4c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255b4c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255b4c => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[26] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255b78; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255b78;
            }
            0x80255b78 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80255b98; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255b98;
            }
            0x80255b98 => { ins += 6;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2d;
                c.lr = 0x80255bb0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255bb0;
            }
            0x80255bb0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80255bbc; continue; }
                pc = 0x80255bb8;
            }
            0x80255bb8 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80255bbc;
            }
            0x80255bbc => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255bc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255bc8;
            }
            0x80255bc8 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80255bdc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255bdc;
            }
            0x80255bdc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80255bec; continue; }
                pc = 0x80255be8;
            }
            0x80255be8 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80255bec;
            }
            0x80255bec => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80255bfc; continue; }
                pc = 0x80255bf4;
            }
            0x80255bf4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255c04; continue;
                pc = 0x80255bfc;
            }
            0x80255bfc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80255c04;
            }
            0x80255c04 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255b4c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit160 (0x80255c18, 116 bytes)
#[inline(never)]
pub fn f_80255c18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255c18) { return; }
        if c.prof { crate::prof::enter(c, 0x80255c18); f_80255c18_at(c, 0x80255c18); crate::prof::exit(c, 0x80255c18); return; }
    }
    f_80255c18_b(c, 0x80255c18);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255c18_at(c: &mut Cpu, pc: u32) { f_80255c18_b(c, pc) }
#[inline(always)]
fn f_80255c18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255c18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255c18, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255c18 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x2d;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x80255c34; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255c34;
            }
            0x80255c34 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80255c44; continue; }
                pc = 0x80255c3c;
            }
            0x80255c3c => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80255c54; continue;
                pc = 0x80255c44;
            }
            0x80255c44 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2d;
                c.lr = 0x80255c50; { c.steps = st; c.cycles = ins; c.cr = cr; f_80202360(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255c50;
            }
            0x80255c50 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80255c54;
            }
            0x80255c54 => { ins += 2;
                c.r[3] = 0x2d;
                c.lr = 0x80255c5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119dd0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255c5c;
            }
            0x80255c5c => { ins += 4;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], c.r[3]);
                if cr_bit(cr, 0) { pc = 0x80255c74; continue; }
                pc = 0x80255c6c;
            }
            0x80255c6c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255c78; continue;
                pc = 0x80255c74;
            }
            0x80255c74 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80255c78;
            }
            0x80255c78 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255c18: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit159 (0x80255c8c, 176 bytes)
#[inline(never)]
pub fn f_80255c8c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255c8c) { return; }
        if c.prof { crate::prof::enter(c, 0x80255c8c); f_80255c8c_at(c, 0x80255c8c); crate::prof::exit(c, 0x80255c8c); return; }
    }
    f_80255c8c_b(c, 0x80255c8c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255c8c_at(c: &mut Cpu, pc: u32) { f_80255c8c_b(c, pc) }
#[inline(always)]
fn f_80255c8c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255c8c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255c8c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255c8c => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255cb8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255cb8;
            }
            0x80255cb8 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80255cd8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255cd8;
            }
            0x80255cd8 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255cec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255cec;
            }
            0x80255cec => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80255d00; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255d00;
            }
            0x80255d00 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80255d10; continue; }
                pc = 0x80255d0c;
            }
            0x80255d0c => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80255d10;
            }
            0x80255d10 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80255d20; continue; }
                pc = 0x80255d18;
            }
            0x80255d18 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255d28; continue;
                pc = 0x80255d20;
            }
            0x80255d20 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80255d28;
            }
            0x80255d28 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255c8c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit158 (0x80255d3c, 64 bytes)
#[inline(never)]
pub fn f_80255d3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255d3c) { return; }
        if c.prof { crate::prof::enter(c, 0x80255d3c); f_80255d3c_at(c, 0x80255d3c); crate::prof::exit(c, 0x80255d3c); return; }
    }
    f_80255d3c_b(c, 0x80255d3c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255d3c_at(c: &mut Cpu, pc: u32) { f_80255d3c_b(c, pc) }
#[inline(always)]
fn f_80255d3c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255d3c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255d3c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255d3c => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0xed;
                c.r[6] = 0x0;
                c.lr = 0x80255d5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255d5c;
            }
            0x80255d5c => { ins += 9;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255d3c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit157 (0x80255d7c, 56 bytes)
#[inline(never)]
pub fn f_80255d7c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255d7c) { return; }
        if c.prof { crate::prof::enter(c, 0x80255d7c); f_80255d7c_at(c, 0x80255d7c); crate::prof::exit(c, 0x80255d7c); return; }
    }
    f_80255d7c_b(c, 0x80255d7c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255d7c_at(c: &mut Cpu, pc: u32) { f_80255d7c_b(c, pc) }
#[inline(always)]
fn f_80255d7c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255d7c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255d7c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255d7c => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffffafa8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80255d94; { c.steps = st; c.cycles = ins; c.cr = cr; f_802373b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255d94;
            }
            0x80255d94 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255d7c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit156 (0x80255db4, 68 bytes)
#[inline(never)]
pub fn f_80255db4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255db4) { return; }
        if c.prof { crate::prof::enter(c, 0x80255db4); f_80255db4_at(c, 0x80255db4); crate::prof::exit(c, 0x80255db4); return; }
    }
    f_80255db4_b(c, 0x80255db4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255db4_at(c: &mut Cpu, pc: u32) { f_80255db4_b(c, pc) }
#[inline(always)]
fn f_80255db4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255db4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255db4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255db4 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x10;
                c.r[8] = 0x2;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[5] | c.r[5];
                c.r[5] = c.r[6] | c.r[6];
                c.r[9] = 0x41;
                c.r[6] = c.r[0] | c.r[0];
                c.lr = 0x80255ddc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255ddc;
            }
            0x80255ddc => { ins += 8;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255db4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit155 (0x80255df8, 176 bytes)
#[inline(never)]
pub fn f_80255df8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255df8) { return; }
        if c.prof { crate::prof::enter(c, 0x80255df8); f_80255df8_at(c, 0x80255df8); crate::prof::exit(c, 0x80255df8); return; }
    }
    f_80255df8_b(c, 0x80255df8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255df8_at(c: &mut Cpu, pc: u32) { f_80255df8_b(c, pc) }
#[inline(always)]
fn f_80255df8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255df8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255df8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255df8 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255e24; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255e24;
            }
            0x80255e24 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80255e44; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255e44;
            }
            0x80255e44 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255e58; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255e58;
            }
            0x80255e58 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80255e6c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255e6c;
            }
            0x80255e6c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80255e7c; continue; }
                pc = 0x80255e78;
            }
            0x80255e78 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80255e7c;
            }
            0x80255e7c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80255e8c; continue; }
                pc = 0x80255e84;
            }
            0x80255e84 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255e94; continue;
                pc = 0x80255e8c;
            }
            0x80255e8c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80255e94;
            }
            0x80255e94 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255df8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit154 (0x80255ea8, 60 bytes)
#[inline(never)]
pub fn f_80255ea8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255ea8) { return; }
        if c.prof { crate::prof::enter(c, 0x80255ea8); f_80255ea8_at(c, 0x80255ea8); crate::prof::exit(c, 0x80255ea8); return; }
    }
    f_80255ea8_b(c, 0x80255ea8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255ea8_at(c: &mut Cpu, pc: u32) { f_80255ea8_b(c, pc) }
#[inline(always)]
fn f_80255ea8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255ea8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255ea8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255ea8 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80255ebc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255ebc;
            }
            0x80255ebc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80255ecc; continue; }
                pc = 0x80255ec4;
            }
            0x80255ec4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255ed4; continue;
                pc = 0x80255ecc;
            }
            0x80255ecc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80255ed4;
            }
            0x80255ed4 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255ea8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit153 (0x80255ee4, 8 bytes)
#[inline(never)]
pub fn f_80255ee4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255ee4) { return; }
        if c.prof { crate::prof::enter(c, 0x80255ee4); f_80255ee4_at(c, 0x80255ee4); crate::prof::exit(c, 0x80255ee4); return; }
    }
    f_80255ee4_b(c, 0x80255ee4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255ee4_at(c: &mut Cpu, pc: u32) { f_80255ee4_b(c, pc) }
#[inline(always)]
fn f_80255ee4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255ee4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255ee4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255ee4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255ee4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit152 (0x80255eec, 180 bytes)
#[inline(never)]
pub fn f_80255eec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255eec) { return; }
        if c.prof { crate::prof::enter(c, 0x80255eec); f_80255eec_at(c, 0x80255eec); crate::prof::exit(c, 0x80255eec); return; }
    }
    f_80255eec_b(c, 0x80255eec);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255eec_at(c: &mut Cpu, pc: u32) { f_80255eec_b(c, pc) }
#[inline(always)]
fn f_80255eec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255eec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255eec, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255eec => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255f18; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255f18;
            }
            0x80255f18 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80255f34; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255f34;
            }
            0x80255f34 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255f44; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255f44;
            }
            0x80255f44 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80255f58; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255f58;
            }
            0x80255f58 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80255f68; continue; }
                pc = 0x80255f64;
            }
            0x80255f64 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80255f68;
            }
            0x80255f68 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80255f78; continue; }
                pc = 0x80255f70;
            }
            0x80255f70 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80255f8c; continue;
                pc = 0x80255f78;
            }
            0x80255f78 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80255f88; continue; }
                pc = 0x80255f80;
            }
            0x80255f80 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80255f8c; continue;
                pc = 0x80255f88;
            }
            0x80255f88 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80255f8c;
            }
            0x80255f8c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255eec: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit151 (0x80255fa0, 180 bytes)
#[inline(never)]
pub fn f_80255fa0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80255fa0) { return; }
        if c.prof { crate::prof::enter(c, 0x80255fa0); f_80255fa0_at(c, 0x80255fa0); crate::prof::exit(c, 0x80255fa0); return; }
    }
    f_80255fa0_b(c, 0x80255fa0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80255fa0_at(c: &mut Cpu, pc: u32) { f_80255fa0_b(c, pc) }
#[inline(always)]
fn f_80255fa0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80255fa0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80255fa0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80255fa0 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255fcc; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255fcc;
            }
            0x80255fcc => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80255fe8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255fe8;
            }
            0x80255fe8 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80255ff8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80255ff8;
            }
            0x80255ff8 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025600c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025600c;
            }
            0x8025600c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x8025601c; continue; }
                pc = 0x80256018;
            }
            0x80256018 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x8025601c;
            }
            0x8025601c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8025602c; continue; }
                pc = 0x80256024;
            }
            0x80256024 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256040; continue;
                pc = 0x8025602c;
            }
            0x8025602c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x8025603c; continue; }
                pc = 0x80256034;
            }
            0x80256034 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80256040; continue;
                pc = 0x8025603c;
            }
            0x8025603c => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80256040;
            }
            0x80256040 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80255fa0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit150 (0x80256054, 180 bytes)
#[inline(never)]
pub fn f_80256054(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256054) { return; }
        if c.prof { crate::prof::enter(c, 0x80256054); f_80256054_at(c, 0x80256054); crate::prof::exit(c, 0x80256054); return; }
    }
    f_80256054_b(c, 0x80256054);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256054_at(c: &mut Cpu, pc: u32) { f_80256054_b(c, pc) }
#[inline(always)]
fn f_80256054_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256054;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256054, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256054 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256080; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256080;
            }
            0x80256080 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025609c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025609c;
            }
            0x8025609c => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802560ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802560ac;
            }
            0x802560ac => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x802560c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802560c0;
            }
            0x802560c0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x802560d0; continue; }
                pc = 0x802560cc;
            }
            0x802560cc => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x802560d0;
            }
            0x802560d0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802560e0; continue; }
                pc = 0x802560d8;
            }
            0x802560d8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802560f4; continue;
                pc = 0x802560e0;
            }
            0x802560e0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x802560f0; continue; }
                pc = 0x802560e8;
            }
            0x802560e8 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x802560f4; continue;
                pc = 0x802560f0;
            }
            0x802560f0 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802560f4;
            }
            0x802560f4 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256054: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit149 (0x80256108, 180 bytes)
#[inline(never)]
pub fn f_80256108(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256108) { return; }
        if c.prof { crate::prof::enter(c, 0x80256108); f_80256108_at(c, 0x80256108); crate::prof::exit(c, 0x80256108); return; }
    }
    f_80256108_b(c, 0x80256108);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256108_at(c: &mut Cpu, pc: u32) { f_80256108_b(c, pc) }
#[inline(always)]
fn f_80256108_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256108;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256108, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256108 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256134; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256134;
            }
            0x80256134 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80256150; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256150;
            }
            0x80256150 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256160; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256160;
            }
            0x80256160 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80256174; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256174;
            }
            0x80256174 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256184; continue; }
                pc = 0x80256180;
            }
            0x80256180 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80256184;
            }
            0x80256184 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80256194; continue; }
                pc = 0x8025618c;
            }
            0x8025618c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802561a8; continue;
                pc = 0x80256194;
            }
            0x80256194 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x802561a4; continue; }
                pc = 0x8025619c;
            }
            0x8025619c => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x802561a8; continue;
                pc = 0x802561a4;
            }
            0x802561a4 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802561a8;
            }
            0x802561a8 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256108: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit148 (0x802561bc, 56 bytes)
#[inline(never)]
pub fn f_802561bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802561bc) { return; }
        if c.prof { crate::prof::enter(c, 0x802561bc); f_802561bc_at(c, 0x802561bc); crate::prof::exit(c, 0x802561bc); return; }
    }
    f_802561bc_b(c, 0x802561bc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802561bc_at(c: &mut Cpu, pc: u32) { f_802561bc_b(c, pc) }
#[inline(always)]
fn f_802561bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802561bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802561bc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802561bc => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x34;
                c.r[4] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x802561d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802561d4;
            }
            0x802561d4 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802561bc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit147 (0x802561f4, 180 bytes)
#[inline(never)]
pub fn f_802561f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802561f4) { return; }
        if c.prof { crate::prof::enter(c, 0x802561f4); f_802561f4_at(c, 0x802561f4); crate::prof::exit(c, 0x802561f4); return; }
    }
    f_802561f4_b(c, 0x802561f4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802561f4_at(c: &mut Cpu, pc: u32) { f_802561f4_b(c, pc) }
#[inline(always)]
fn f_802561f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802561f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802561f4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802561f4 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256220; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256220;
            }
            0x80256220 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025623c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025623c;
            }
            0x8025623c => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025624c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025624c;
            }
            0x8025624c => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80256260; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256260;
            }
            0x80256260 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256270; continue; }
                pc = 0x8025626c;
            }
            0x8025626c => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80256270;
            }
            0x80256270 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80256280; continue; }
                pc = 0x80256278;
            }
            0x80256278 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256294; continue;
                pc = 0x80256280;
            }
            0x80256280 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80256290; continue; }
                pc = 0x80256288;
            }
            0x80256288 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80256294; continue;
                pc = 0x80256290;
            }
            0x80256290 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80256294;
            }
            0x80256294 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802561f4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit146 (0x802562a8, 180 bytes)
#[inline(never)]
pub fn f_802562a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802562a8) { return; }
        if c.prof { crate::prof::enter(c, 0x802562a8); f_802562a8_at(c, 0x802562a8); crate::prof::exit(c, 0x802562a8); return; }
    }
    f_802562a8_b(c, 0x802562a8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802562a8_at(c: &mut Cpu, pc: u32) { f_802562a8_b(c, pc) }
#[inline(always)]
fn f_802562a8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802562a8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802562a8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802562a8 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802562d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802562d4;
            }
            0x802562d4 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x802562f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802562f0;
            }
            0x802562f0 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256300; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256300;
            }
            0x80256300 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80256314; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256314;
            }
            0x80256314 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256324; continue; }
                pc = 0x80256320;
            }
            0x80256320 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80256324;
            }
            0x80256324 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80256334; continue; }
                pc = 0x8025632c;
            }
            0x8025632c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256348; continue;
                pc = 0x80256334;
            }
            0x80256334 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80256344; continue; }
                pc = 0x8025633c;
            }
            0x8025633c => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80256348; continue;
                pc = 0x80256344;
            }
            0x80256344 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80256348;
            }
            0x80256348 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802562a8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit145 (0x8025635c, 180 bytes)
#[inline(never)]
pub fn f_8025635c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025635c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025635c); f_8025635c_at(c, 0x8025635c); crate::prof::exit(c, 0x8025635c); return; }
    }
    f_8025635c_b(c, 0x8025635c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025635c_at(c: &mut Cpu, pc: u32) { f_8025635c_b(c, pc) }
#[inline(always)]
fn f_8025635c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8025635c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8025635c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8025635c => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256388; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256388;
            }
            0x80256388 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x802563a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802563a4;
            }
            0x802563a4 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802563b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802563b4;
            }
            0x802563b4 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x802563c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802563c8;
            }
            0x802563c8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x802563d8; continue; }
                pc = 0x802563d4;
            }
            0x802563d4 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x802563d8;
            }
            0x802563d8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802563e8; continue; }
                pc = 0x802563e0;
            }
            0x802563e0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802563fc; continue;
                pc = 0x802563e8;
            }
            0x802563e8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x802563f8; continue; }
                pc = 0x802563f0;
            }
            0x802563f0 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x802563fc; continue;
                pc = 0x802563f8;
            }
            0x802563f8 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802563fc;
            }
            0x802563fc => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8025635c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit144 (0x80256410, 176 bytes)
#[inline(never)]
pub fn f_80256410(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256410) { return; }
        if c.prof { crate::prof::enter(c, 0x80256410); f_80256410_at(c, 0x80256410); crate::prof::exit(c, 0x80256410); return; }
    }
    f_80256410_b(c, 0x80256410);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256410_at(c: &mut Cpu, pc: u32) { f_80256410_b(c, pc) }
#[inline(always)]
fn f_80256410_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256410;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256410, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256410 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025643c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025643c;
            }
            0x8025643c => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025645c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025645c;
            }
            0x8025645c => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256470; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256470;
            }
            0x80256470 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80256484; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256484;
            }
            0x80256484 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256494; continue; }
                pc = 0x80256490;
            }
            0x80256490 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80256494;
            }
            0x80256494 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802564a4; continue; }
                pc = 0x8025649c;
            }
            0x8025649c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802564ac; continue;
                pc = 0x802564a4;
            }
            0x802564a4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802564ac;
            }
            0x802564ac => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256410: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit143 (0x802564c0, 8 bytes)
#[inline(never)]
pub fn f_802564c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802564c0) { return; }
        if c.prof { crate::prof::enter(c, 0x802564c0); f_802564c0_at(c, 0x802564c0); crate::prof::exit(c, 0x802564c0); return; }
    }
    f_802564c0_b(c, 0x802564c0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802564c0_at(c: &mut Cpu, pc: u32) { f_802564c0_b(c, pc) }
#[inline(always)]
fn f_802564c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802564c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802564c0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802564c0 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802564c0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit142 (0x802564c8, 100 bytes)
#[inline(never)]
pub fn f_802564c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802564c8) { return; }
        if c.prof { crate::prof::enter(c, 0x802564c8); f_802564c8_at(c, 0x802564c8); crate::prof::exit(c, 0x802564c8); return; }
    }
    f_802564c8_b(c, 0x802564c8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802564c8_at(c: &mut Cpu, pc: u32) { f_802564c8_b(c, pc) }
#[inline(always)]
fn f_802564c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802564c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802564c8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802564c8 => { ins += 17;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x802564e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235aa0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802564e4;
            }
            0x802564e4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if !cr_bit(cr, 0) { pc = 0x8025650c; continue; }
                pc = 0x802564f0;
            }
            0x802564f0 => { ins += 5;
                { let ea = c.r[2].wrapping_add(0xffffafac); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0xffffffff;
                c.lr = 0x80256504; { c.steps = st; c.cycles = ins; c.cr = cr; f_802373b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256504;
            }
            0x80256504 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80256514; continue; }
                pc = 0x8025650c;
            }
            0x8025650c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256518; continue;
                pc = 0x80256514;
            }
            0x80256514 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80256518;
            }
            0x80256518 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802564c8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit140 (0x8025652c, 180 bytes)
#[inline(never)]
pub fn f_8025652c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025652c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025652c); f_8025652c_at(c, 0x8025652c); crate::prof::exit(c, 0x8025652c); return; }
    }
    f_8025652c_b(c, 0x8025652c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025652c_at(c: &mut Cpu, pc: u32) { f_8025652c_b(c, pc) }
#[inline(always)]
fn f_8025652c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8025652c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8025652c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8025652c => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256558; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256558;
            }
            0x80256558 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80256574; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256574;
            }
            0x80256574 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256584; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256584;
            }
            0x80256584 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80256598; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256598;
            }
            0x80256598 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x802565a8; continue; }
                pc = 0x802565a4;
            }
            0x802565a4 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x802565a8;
            }
            0x802565a8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802565b8; continue; }
                pc = 0x802565b0;
            }
            0x802565b0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802565cc; continue;
                pc = 0x802565b8;
            }
            0x802565b8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x802565c8; continue; }
                pc = 0x802565c0;
            }
            0x802565c0 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x802565cc; continue;
                pc = 0x802565c8;
            }
            0x802565c8 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802565cc;
            }
            0x802565cc => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8025652c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit139 (0x802565e0, 180 bytes)
#[inline(never)]
pub fn f_802565e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802565e0) { return; }
        if c.prof { crate::prof::enter(c, 0x802565e0); f_802565e0_at(c, 0x802565e0); crate::prof::exit(c, 0x802565e0); return; }
    }
    f_802565e0_b(c, 0x802565e0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802565e0_at(c: &mut Cpu, pc: u32) { f_802565e0_b(c, pc) }
#[inline(always)]
fn f_802565e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802565e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802565e0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802565e0 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025660c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025660c;
            }
            0x8025660c => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80256628; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256628;
            }
            0x80256628 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256638; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256638;
            }
            0x80256638 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025664c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025664c;
            }
            0x8025664c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x8025665c; continue; }
                pc = 0x80256658;
            }
            0x80256658 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x8025665c;
            }
            0x8025665c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8025666c; continue; }
                pc = 0x80256664;
            }
            0x80256664 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256680; continue;
                pc = 0x8025666c;
            }
            0x8025666c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x8025667c; continue; }
                pc = 0x80256674;
            }
            0x80256674 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80256680; continue;
                pc = 0x8025667c;
            }
            0x8025667c => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80256680;
            }
            0x80256680 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802565e0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit138 (0x80256694, 180 bytes)
#[inline(never)]
pub fn f_80256694(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256694) { return; }
        if c.prof { crate::prof::enter(c, 0x80256694); f_80256694_at(c, 0x80256694); crate::prof::exit(c, 0x80256694); return; }
    }
    f_80256694_b(c, 0x80256694);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256694_at(c: &mut Cpu, pc: u32) { f_80256694_b(c, pc) }
#[inline(always)]
fn f_80256694_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256694;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256694, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256694 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802566c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802566c0;
            }
            0x802566c0 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x802566dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802566dc;
            }
            0x802566dc => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802566ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802566ec;
            }
            0x802566ec => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80256700; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256700;
            }
            0x80256700 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256710; continue; }
                pc = 0x8025670c;
            }
            0x8025670c => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80256710;
            }
            0x80256710 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80256720; continue; }
                pc = 0x80256718;
            }
            0x80256718 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256734; continue;
                pc = 0x80256720;
            }
            0x80256720 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80256730; continue; }
                pc = 0x80256728;
            }
            0x80256728 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80256734; continue;
                pc = 0x80256730;
            }
            0x80256730 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80256734;
            }
            0x80256734 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256694: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit137 (0x80256748, 56 bytes)
#[inline(never)]
pub fn f_80256748(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256748) { return; }
        if c.prof { crate::prof::enter(c, 0x80256748); f_80256748_at(c, 0x80256748); crate::prof::exit(c, 0x80256748); return; }
    }
    f_80256748_b(c, 0x80256748);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256748_at(c: &mut Cpu, pc: u32) { f_80256748_b(c, pc) }
#[inline(always)]
fn f_80256748_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256748;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256748, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256748 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80256760; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235b04(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256760;
            }
            0x80256760 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256748: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit136 (0x80256780, 56 bytes)
#[inline(never)]
pub fn f_80256780(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256780) { return; }
        if c.prof { crate::prof::enter(c, 0x80256780); f_80256780_at(c, 0x80256780); crate::prof::exit(c, 0x80256780); return; }
    }
    f_80256780_b(c, 0x80256780);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256780_at(c: &mut Cpu, pc: u32) { f_80256780_b(c, pc) }
#[inline(always)]
fn f_80256780_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256780;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256780, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256780 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80256798; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235b04(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256798;
            }
            0x80256798 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x2, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256780: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit135 (0x802567b8, 164 bytes)
#[inline(never)]
pub fn f_802567b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802567b8) { return; }
        if c.prof { crate::prof::enter(c, 0x802567b8); f_802567b8_at(c, 0x802567b8); crate::prof::exit(c, 0x802567b8); return; }
    }
    f_802567b8_b(c, 0x802567b8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802567b8_at(c: &mut Cpu, pc: u32) { f_802567b8_b(c, pc) }
#[inline(always)]
fn f_802567b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802567b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802567b8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802567b8 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[28] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x802567e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80205b8c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802567e0;
            }
            0x802567e0 => { ins += 3;
                c.r[4] = c.r[1].wrapping_add(0xa);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.lr = 0x802567ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80120b00(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802567ec;
            }
            0x802567ec => { ins += 6;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x80256804; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256804;
            }
            0x80256804 => { ins += 7;
                c.r[0] = c.lhz_t(ins, c.r[1].wrapping_add(0xa));
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[6] = c.r[0] as u16 as i16 as i32 as u32;
                c.lr = 0x80256820; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256820;
            }
            0x80256820 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256830; continue; }
                pc = 0x8025682c;
            }
            0x8025682c => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80256830;
            }
            0x80256830 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80256840; continue; }
                pc = 0x80256838;
            }
            0x80256838 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256848; continue;
                pc = 0x80256840;
            }
            0x80256840 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80256848;
            }
            0x80256848 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802567b8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit134 (0x8025685c, 56 bytes)
#[inline(never)]
pub fn f_8025685c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025685c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025685c); f_8025685c_at(c, 0x8025685c); crate::prof::exit(c, 0x8025685c); return; }
    }
    f_8025685c_b(c, 0x8025685c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025685c_at(c: &mut Cpu, pc: u32) { f_8025685c_b(c, pc) }
#[inline(always)]
fn f_8025685c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8025685c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8025685c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8025685c => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffffafa8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80256874; { c.steps = st; c.cycles = ins; c.cr = cr; f_802373b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256874;
            }
            0x80256874 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8025685c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit133 (0x80256894, 56 bytes)
#[inline(never)]
pub fn f_80256894(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256894) { return; }
        if c.prof { crate::prof::enter(c, 0x80256894); f_80256894_at(c, 0x80256894); crate::prof::exit(c, 0x80256894); return; }
    }
    f_80256894_b(c, 0x80256894);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256894_at(c: &mut Cpu, pc: u32) { f_80256894_b(c, pc) }
#[inline(always)]
fn f_80256894_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256894;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256894, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256894 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffffafa8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x802568ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_802373b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802568ac;
            }
            0x802568ac => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256894: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit132 (0x802568cc, 56 bytes)
#[inline(never)]
pub fn f_802568cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802568cc) { return; }
        if c.prof { crate::prof::enter(c, 0x802568cc); f_802568cc_at(c, 0x802568cc); crate::prof::exit(c, 0x802568cc); return; }
    }
    f_802568cc_b(c, 0x802568cc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802568cc_at(c: &mut Cpu, pc: u32) { f_802568cc_b(c, pc) }
#[inline(always)]
fn f_802568cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802568cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802568cc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802568cc => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffffafa8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x802568e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_802373b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802568e4;
            }
            0x802568e4 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802568cc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit130 (0x80256904, 176 bytes)
#[inline(never)]
pub fn f_80256904(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256904) { return; }
        if c.prof { crate::prof::enter(c, 0x80256904); f_80256904_at(c, 0x80256904); crate::prof::exit(c, 0x80256904); return; }
    }
    f_80256904_b(c, 0x80256904);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256904_at(c: &mut Cpu, pc: u32) { f_80256904_b(c, pc) }
#[inline(always)]
fn f_80256904_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256904;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256904, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256904 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256930; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256930;
            }
            0x80256930 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80256950; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256950;
            }
            0x80256950 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256964; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256964;
            }
            0x80256964 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80256978; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256978;
            }
            0x80256978 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256988; continue; }
                pc = 0x80256984;
            }
            0x80256984 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80256988;
            }
            0x80256988 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80256998; continue; }
                pc = 0x80256990;
            }
            0x80256990 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802569a0; continue;
                pc = 0x80256998;
            }
            0x80256998 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802569a0;
            }
            0x802569a0 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256904: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit129 (0x802569b4, 180 bytes)
#[inline(never)]
pub fn f_802569b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802569b4) { return; }
        if c.prof { crate::prof::enter(c, 0x802569b4); f_802569b4_at(c, 0x802569b4); crate::prof::exit(c, 0x802569b4); return; }
    }
    f_802569b4_b(c, 0x802569b4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802569b4_at(c: &mut Cpu, pc: u32) { f_802569b4_b(c, pc) }
#[inline(always)]
fn f_802569b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802569b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802569b4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802569b4 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802569e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802569e0;
            }
            0x802569e0 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x802569fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802569fc;
            }
            0x802569fc => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256a0c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256a0c;
            }
            0x80256a0c => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80256a20; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256a20;
            }
            0x80256a20 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256a30; continue; }
                pc = 0x80256a2c;
            }
            0x80256a2c => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80256a30;
            }
            0x80256a30 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80256a40; continue; }
                pc = 0x80256a38;
            }
            0x80256a38 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256a54; continue;
                pc = 0x80256a40;
            }
            0x80256a40 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80256a50; continue; }
                pc = 0x80256a48;
            }
            0x80256a48 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80256a54; continue;
                pc = 0x80256a50;
            }
            0x80256a50 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80256a54;
            }
            0x80256a54 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802569b4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit128 (0x80256a68, 120 bytes)
#[inline(never)]
pub fn f_80256a68(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256a68) { return; }
        if c.prof { crate::prof::enter(c, 0x80256a68); f_80256a68_at(c, 0x80256a68); crate::prof::exit(c, 0x80256a68); return; }
    }
    f_80256a68_b(c, 0x80256a68);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256a68_at(c: &mut Cpu, pc: u32) { f_80256a68_b(c, pc) }
#[inline(always)]
fn f_80256a68_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256a68;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256a68, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256a68 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4] | c.r[4];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x80256a94; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256a94;
            }
            0x80256a94 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80256aa8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256aa8;
            }
            0x80256aa8 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80256abc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256abc;
            }
            0x80256abc => { ins += 20;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[3] = c.addc(!c.r[0], 0x43, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256a68: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit127 (0x80256ae0, 56 bytes)
#[inline(never)]
pub fn f_80256ae0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256ae0) { return; }
        if c.prof { crate::prof::enter(c, 0x80256ae0); f_80256ae0_at(c, 0x80256ae0); crate::prof::exit(c, 0x80256ae0); return; }
    }
    f_80256ae0_b(c, 0x80256ae0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256ae0_at(c: &mut Cpu, pc: u32) { f_80256ae0_b(c, pc) }
#[inline(always)]
fn f_80256ae0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256ae0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256ae0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256ae0 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = 0x0;
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.sth(c.r[1].wrapping_add(0x8), c.r[0]);
                c.lr = 0x80256afc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8a18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256afc;
            }
            0x80256afc => { ins += 8;
                c.r[3] = c.r[3].wrapping_neg();
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256ae0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit126 (0x80256b18, 180 bytes)
#[inline(never)]
pub fn f_80256b18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256b18) { return; }
        if c.prof { crate::prof::enter(c, 0x80256b18); f_80256b18_at(c, 0x80256b18); crate::prof::exit(c, 0x80256b18); return; }
    }
    f_80256b18_b(c, 0x80256b18);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256b18_at(c: &mut Cpu, pc: u32) { f_80256b18_b(c, pc) }
#[inline(always)]
fn f_80256b18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256b18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256b18, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256b18 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256b44; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256b44;
            }
            0x80256b44 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80256b60; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256b60;
            }
            0x80256b60 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256b70; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256b70;
            }
            0x80256b70 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80256b84; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256b84;
            }
            0x80256b84 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256b94; continue; }
                pc = 0x80256b90;
            }
            0x80256b90 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80256b94;
            }
            0x80256b94 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80256ba4; continue; }
                pc = 0x80256b9c;
            }
            0x80256b9c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256bb8; continue;
                pc = 0x80256ba4;
            }
            0x80256ba4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80256bb4; continue; }
                pc = 0x80256bac;
            }
            0x80256bac => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80256bb8; continue;
                pc = 0x80256bb4;
            }
            0x80256bb4 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80256bb8;
            }
            0x80256bb8 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256b18: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit125 (0x80256bcc, 180 bytes)
#[inline(never)]
pub fn f_80256bcc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256bcc) { return; }
        if c.prof { crate::prof::enter(c, 0x80256bcc); f_80256bcc_at(c, 0x80256bcc); crate::prof::exit(c, 0x80256bcc); return; }
    }
    f_80256bcc_b(c, 0x80256bcc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256bcc_at(c: &mut Cpu, pc: u32) { f_80256bcc_b(c, pc) }
#[inline(always)]
fn f_80256bcc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256bcc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256bcc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256bcc => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256bf8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256bf8;
            }
            0x80256bf8 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80256c14; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256c14;
            }
            0x80256c14 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256c24; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256c24;
            }
            0x80256c24 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80256c38; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256c38;
            }
            0x80256c38 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256c48; continue; }
                pc = 0x80256c44;
            }
            0x80256c44 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80256c48;
            }
            0x80256c48 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80256c58; continue; }
                pc = 0x80256c50;
            }
            0x80256c50 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256c6c; continue;
                pc = 0x80256c58;
            }
            0x80256c58 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80256c68; continue; }
                pc = 0x80256c60;
            }
            0x80256c60 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80256c6c; continue;
                pc = 0x80256c68;
            }
            0x80256c68 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80256c6c;
            }
            0x80256c6c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256bcc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit124 (0x80256c80, 60 bytes)
#[inline(never)]
pub fn f_80256c80(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256c80) { return; }
        if c.prof { crate::prof::enter(c, 0x80256c80); f_80256c80_at(c, 0x80256c80); crate::prof::exit(c, 0x80256c80); return; }
    }
    f_80256c80_b(c, 0x80256c80);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256c80_at(c: &mut Cpu, pc: u32) { f_80256c80_b(c, pc) }
#[inline(always)]
fn f_80256c80_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256c80;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256c80, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256c80 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x2;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80256c94; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256c94;
            }
            0x80256c94 => { ins += 2;
                c.r[4] = 0x4b;
                c.lr = 0x80256c9c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f6e44(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256c9c;
            }
            0x80256c9c => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(!c.r[0], 0x2, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256c80: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit123 (0x80256cbc, 180 bytes)
#[inline(never)]
pub fn f_80256cbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256cbc) { return; }
        if c.prof { crate::prof::enter(c, 0x80256cbc); f_80256cbc_at(c, 0x80256cbc); crate::prof::exit(c, 0x80256cbc); return; }
    }
    f_80256cbc_b(c, 0x80256cbc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256cbc_at(c: &mut Cpu, pc: u32) { f_80256cbc_b(c, pc) }
#[inline(always)]
fn f_80256cbc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256cbc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256cbc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256cbc => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256ce8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256ce8;
            }
            0x80256ce8 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80256d04; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256d04;
            }
            0x80256d04 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256d14; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256d14;
            }
            0x80256d14 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80256d28; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256d28;
            }
            0x80256d28 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256d38; continue; }
                pc = 0x80256d34;
            }
            0x80256d34 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80256d38;
            }
            0x80256d38 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80256d48; continue; }
                pc = 0x80256d40;
            }
            0x80256d40 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256d5c; continue;
                pc = 0x80256d48;
            }
            0x80256d48 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80256d58; continue; }
                pc = 0x80256d50;
            }
            0x80256d50 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80256d5c; continue;
                pc = 0x80256d58;
            }
            0x80256d58 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80256d5c;
            }
            0x80256d5c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256cbc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit122 (0x80256d70, 176 bytes)
#[inline(never)]
pub fn f_80256d70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256d70) { return; }
        if c.prof { crate::prof::enter(c, 0x80256d70); f_80256d70_at(c, 0x80256d70); crate::prof::exit(c, 0x80256d70); return; }
    }
    f_80256d70_b(c, 0x80256d70);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256d70_at(c: &mut Cpu, pc: u32) { f_80256d70_b(c, pc) }
#[inline(always)]
fn f_80256d70_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256d70;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256d70, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256d70 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256d9c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256d9c;
            }
            0x80256d9c => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80256dbc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256dbc;
            }
            0x80256dbc => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256dd0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256dd0;
            }
            0x80256dd0 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80256de4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256de4;
            }
            0x80256de4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256df4; continue; }
                pc = 0x80256df0;
            }
            0x80256df0 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80256df4;
            }
            0x80256df4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80256e04; continue; }
                pc = 0x80256dfc;
            }
            0x80256dfc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256e0c; continue;
                pc = 0x80256e04;
            }
            0x80256e04 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80256e0c;
            }
            0x80256e0c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256d70: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit121 (0x80256e20, 176 bytes)
#[inline(never)]
pub fn f_80256e20(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256e20) { return; }
        if c.prof { crate::prof::enter(c, 0x80256e20); f_80256e20_at(c, 0x80256e20); crate::prof::exit(c, 0x80256e20); return; }
    }
    f_80256e20_b(c, 0x80256e20);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256e20_at(c: &mut Cpu, pc: u32) { f_80256e20_b(c, pc) }
#[inline(always)]
fn f_80256e20_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256e20;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256e20, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256e20 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256e4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256e4c;
            }
            0x80256e4c => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80256e6c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256e6c;
            }
            0x80256e6c => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80256e80; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256e80;
            }
            0x80256e80 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80256e94; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256e94;
            }
            0x80256e94 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80256ea4; continue; }
                pc = 0x80256ea0;
            }
            0x80256ea0 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80256ea4;
            }
            0x80256ea4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80256eb4; continue; }
                pc = 0x80256eac;
            }
            0x80256eac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80256ebc; continue;
                pc = 0x80256eb4;
            }
            0x80256eb4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80256ebc;
            }
            0x80256ebc => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256e20: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit120 (0x80256ed0, 512 bytes)
#[inline(never)]
pub fn f_80256ed0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80256ed0) { return; }
        if c.prof { crate::prof::enter(c, 0x80256ed0); f_80256ed0_at(c, 0x80256ed0); crate::prof::exit(c, 0x80256ed0); return; }
    }
    f_80256ed0_b(c, 0x80256ed0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80256ed0_at(c: &mut Cpu, pc: u32) { f_80256ed0_b(c, pc) }
#[inline(never)]
fn f_80256ed0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80256ed0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80256ed0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80256ed0 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[4] | c.r[4];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[6] | c.r[6];
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x80256ef8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802016a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256ef8;
            }
            0x80256ef8 => { ins += 3;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80256f04; { c.steps = st; c.cycles = ins; c.cr = cr; f_802016a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256f04;
            }
            0x80256f04 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[28] | c.r[28];
                c.r[7] = 0x0;
                c.lr = 0x80256f20; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256f20;
            }
            0x80256f20 => { ins += 6;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0xc;
                c.lr = 0x80256f38; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256f38;
            }
            0x80256f38 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80256f48; continue; }
                pc = 0x80256f44;
            }
            0x80256f44 => { ins += 1;
                c.r[29] = 0x0;
                pc = 0x80256f48;
            }
            0x80256f48 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x11;
                c.lr = 0x80256f58; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256f58;
            }
            0x80256f58 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80257030; continue; }
                pc = 0x80256f64;
            }
            0x80256f64 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x14;
                c.lr = 0x80256f74; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256f74;
            }
            0x80256f74 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80257030; continue; }
                pc = 0x80256f80;
            }
            0x80256f80 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x7;
                c.lr = 0x80256f90; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256f90;
            }
            0x80256f90 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80257030; continue; }
                pc = 0x80256f9c;
            }
            0x80256f9c => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0xf;
                c.lr = 0x80256fac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256fac;
            }
            0x80256fac => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80257030; continue; }
                pc = 0x80256fb8;
            }
            0x80256fb8 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x48;
                c.lr = 0x80256fc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256fc8;
            }
            0x80256fc8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80257030; continue; }
                pc = 0x80256fd4;
            }
            0x80256fd4 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x29;
                c.lr = 0x80256fe4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80256fe4;
            }
            0x80256fe4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80257030; continue; }
                pc = 0x80256ff0;
            }
            0x80256ff0 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x28;
                c.lr = 0x80257000; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257000;
            }
            0x80257000 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80257030; continue; }
                pc = 0x8025700c;
            }
            0x8025700c => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0xc;
                c.lr = 0x8025701c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025701c;
            }
            0x8025701c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80257030; continue; }
                pc = 0x80257028;
            }
            0x80257028 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80257034; continue;
                pc = 0x80257030;
            }
            0x80257030 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x80257034;
            }
            0x80257034 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80257044; continue; }
                pc = 0x8025703c;
            }
            0x8025703c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802570bc; continue;
                pc = 0x80257044;
            }
            0x80257044 => { ins += 4;
                c.r[3] = c.r[30].rotate_left(0) & 0xff;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 2) { pc = 0x802570a0; continue; }
                pc = 0x80257054;
            }
            0x80257054 => { ins += 3;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257060; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237288(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257060;
            }
            0x80257060 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802570a0; continue; }
                pc = 0x8025706c;
            }
            0x8025706c => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0xa;
                c.lr = 0x8025707c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025707c;
            }
            0x8025707c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802570a0; continue; }
                pc = 0x80257088;
            }
            0x80257088 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x802570a0; continue; }
                pc = 0x80257094;
            }
            0x80257094 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x802570a4; continue; }
                pc = 0x802570a0;
            }
            0x802570a0 => { ins += 1;
                c.r[29] = 0x0;
                pc = 0x802570a4;
            }
            0x802570a4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802570b4; continue; }
                pc = 0x802570ac;
            }
            0x802570ac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802570bc; continue;
                pc = 0x802570b4;
            }
            0x802570b4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802570bc;
            }
            0x802570bc => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80256ed0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit119 (0x802570d0, 176 bytes)
#[inline(never)]
pub fn f_802570d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802570d0) { return; }
        if c.prof { crate::prof::enter(c, 0x802570d0); f_802570d0_at(c, 0x802570d0); crate::prof::exit(c, 0x802570d0); return; }
    }
    f_802570d0_b(c, 0x802570d0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802570d0_at(c: &mut Cpu, pc: u32) { f_802570d0_b(c, pc) }
#[inline(always)]
fn f_802570d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802570d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802570d0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802570d0 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802570fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802570fc;
            }
            0x802570fc => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025711c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025711c;
            }
            0x8025711c => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257130; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257130;
            }
            0x80257130 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80257144; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257144;
            }
            0x80257144 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80257154; continue; }
                pc = 0x80257150;
            }
            0x80257150 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80257154;
            }
            0x80257154 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80257164; continue; }
                pc = 0x8025715c;
            }
            0x8025715c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025716c; continue;
                pc = 0x80257164;
            }
            0x80257164 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025716c;
            }
            0x8025716c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802570d0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit118 (0x80257180, 572 bytes)
#[inline(never)]
pub fn f_80257180(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257180) { return; }
        if c.prof { crate::prof::enter(c, 0x80257180); f_80257180_at(c, 0x80257180); crate::prof::exit(c, 0x80257180); return; }
    }
    f_80257180_b(c, 0x80257180);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257180_at(c: &mut Cpu, pc: u32) { f_80257180_b(c, pc) }
#[inline(never)]
fn f_80257180_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257180;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257180, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257180 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3] | c.r[3];
                c.r[27] = c.r[6] | c.r[6];
                c.r[25] = c.r[4] | c.r[4];
                c.r[26] = c.r[5] | c.r[5];
                c.r[3] = 0x2;
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x802571ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802571ac;
            }
            0x802571ac => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[28] = c.r[0] | c.r[0];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x802571c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235aa0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802571c0;
            }
            0x802571c0 => { ins += 5;
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x14;
                c.lr = 0x802571d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802571d4;
            }
            0x802571d4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802571e8; continue; }
                pc = 0x802571e0;
            }
            0x802571e0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802573a8; continue;
                pc = 0x802571e8;
            }
            0x802571e8 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[25] | c.r[25];
                c.r[5] = c.r[26] | c.r[26];
                c.r[6] = c.r[27] | c.r[27];
                c.r[7] = 0x0;
                c.lr = 0x80257200; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257200;
            }
            0x80257200 => { ins += 5;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x9;
                c.lr = 0x80257214; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257214;
            }
            0x80257214 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8025725c; continue; }
                pc = 0x80257220;
            }
            0x80257220 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x8025725c; continue; }
                pc = 0x8025722c;
            }
            0x8025722c => { ins += 8;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[25] | c.r[25];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = c.r[26] | c.r[26];
                c.r[7] = 0x20;
                c.r[8] = 0x1;
                c.r[9] = 0x1;
                c.lr = 0x8025724c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025724c;
            }
            0x8025724c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8025725c; continue; }
                pc = 0x80257254;
            }
            0x80257254 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802573a8; continue;
                pc = 0x8025725c;
            }
            0x8025725c => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x14;
                c.lr = 0x8025726c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025726c;
            }
            0x8025726c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80257280; continue; }
                pc = 0x80257278;
            }
            0x80257278 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802573a8; continue;
                pc = 0x80257280;
            }
            0x80257280 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x11;
                c.lr = 0x80257290; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257290;
            }
            0x80257290 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8025735c; continue; }
                pc = 0x8025729c;
            }
            0x8025729c => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x14;
                c.lr = 0x802572ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802572ac;
            }
            0x802572ac => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802572c0; continue; }
                pc = 0x802572b8;
            }
            0x802572b8 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80257360; continue;
                pc = 0x802572c0;
            }
            0x802572c0 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x7;
                c.lr = 0x802572d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802572d0;
            }
            0x802572d0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8025735c; continue; }
                pc = 0x802572dc;
            }
            0x802572dc => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0xf;
                c.lr = 0x802572ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802572ec;
            }
            0x802572ec => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8025735c; continue; }
                pc = 0x802572f8;
            }
            0x802572f8 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x48;
                c.lr = 0x80257308; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257308;
            }
            0x80257308 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8025735c; continue; }
                pc = 0x80257314;
            }
            0x80257314 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x29;
                c.lr = 0x80257324; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257324;
            }
            0x80257324 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8025735c; continue; }
                pc = 0x80257330;
            }
            0x80257330 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x28;
                c.lr = 0x80257340; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257340;
            }
            0x80257340 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8025735c; continue; }
                pc = 0x8025734c;
            }
            0x8025734c => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0xc;
                c.lr = 0x8025735c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237f74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025735c;
            }
            0x8025735c => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x80257360;
            }
            0x80257360 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80257370; continue; }
                pc = 0x80257368;
            }
            0x80257368 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802573a8; continue;
                pc = 0x80257370;
            }
            0x80257370 => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x4b;
                c.lr = 0x8025737c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f6e98(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025737c;
            }
            0x8025737c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80257390; continue; }
                pc = 0x80257388;
            }
            0x80257388 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802573a8; continue;
                pc = 0x80257390;
            }
            0x80257390 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802573a0; continue; }
                pc = 0x80257398;
            }
            0x80257398 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802573a8; continue;
                pc = 0x802573a0;
            }
            0x802573a0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802573a8;
            }
            0x802573a8 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257180: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit117 (0x802573bc, 176 bytes)
#[inline(never)]
pub fn f_802573bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802573bc) { return; }
        if c.prof { crate::prof::enter(c, 0x802573bc); f_802573bc_at(c, 0x802573bc); crate::prof::exit(c, 0x802573bc); return; }
    }
    f_802573bc_b(c, 0x802573bc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802573bc_at(c: &mut Cpu, pc: u32) { f_802573bc_b(c, pc) }
#[inline(always)]
fn f_802573bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802573bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802573bc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802573bc => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802573e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802573e8;
            }
            0x802573e8 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80257408; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257408;
            }
            0x80257408 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025741c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025741c;
            }
            0x8025741c => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80257430; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257430;
            }
            0x80257430 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80257440; continue; }
                pc = 0x8025743c;
            }
            0x8025743c => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80257440;
            }
            0x80257440 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80257450; continue; }
                pc = 0x80257448;
            }
            0x80257448 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257458; continue;
                pc = 0x80257450;
            }
            0x80257450 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80257458;
            }
            0x80257458 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802573bc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit116 (0x8025746c, 8 bytes)
#[inline(never)]
pub fn f_8025746c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025746c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025746c); f_8025746c_at(c, 0x8025746c); crate::prof::exit(c, 0x8025746c); return; }
    }
    f_8025746c_b(c, 0x8025746c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025746c_at(c: &mut Cpu, pc: u32) { f_8025746c_b(c, pc) }
#[inline(always)]
fn f_8025746c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8025746c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8025746c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8025746c => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8025746c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit115 (0x80257474, 56 bytes)
#[inline(never)]
pub fn f_80257474(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257474) { return; }
        if c.prof { crate::prof::enter(c, 0x80257474); f_80257474_at(c, 0x80257474); crate::prof::exit(c, 0x80257474); return; }
    }
    f_80257474_b(c, 0x80257474);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257474_at(c: &mut Cpu, pc: u32) { f_80257474_b(c, pc) }
#[inline(always)]
fn f_80257474_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257474;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257474, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257474 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025748c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235b04(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025748c;
            }
            0x8025748c => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x3, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257474: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit114 (0x802574ac, 92 bytes)
#[inline(never)]
pub fn f_802574ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802574ac) { return; }
        if c.prof { crate::prof::enter(c, 0x802574ac); f_802574ac_at(c, 0x802574ac); crate::prof::exit(c, 0x802574ac); return; }
    }
    f_802574ac_b(c, 0x802574ac);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802574ac_at(c: &mut Cpu, pc: u32) { f_802574ac_b(c, pc) }
#[inline(always)]
fn f_802574ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802574ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802574ac, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802574ac => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[5] | c.r[5];
                c.r[5] = c.r[6] | c.r[6];
                c.lr = 0x802574cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80229934(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802574cc;
            }
            0x802574cc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802574e0; continue; }
                pc = 0x802574d8;
            }
            0x802574d8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802574f4; continue;
                pc = 0x802574e0;
            }
            0x802574e0 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802574e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c674(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802574e8;
            }
            0x802574e8 => { ins += 3;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                pc = 0x802574f4;
            }
            0x802574f4 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802574ac: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit113 (0x80257508, 60 bytes)
#[inline(never)]
pub fn f_80257508(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257508) { return; }
        if c.prof { crate::prof::enter(c, 0x80257508); f_80257508_at(c, 0x80257508); crate::prof::exit(c, 0x80257508); return; }
    }
    f_80257508_b(c, 0x80257508);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257508_at(c: &mut Cpu, pc: u32) { f_80257508_b(c, pc) }
#[inline(always)]
fn f_80257508_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257508;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257508, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257508 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025751c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025751c;
            }
            0x8025751c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8025752c; continue; }
                pc = 0x80257524;
            }
            0x80257524 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257534; continue;
                pc = 0x8025752c;
            }
            0x8025752c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80257534;
            }
            0x80257534 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257508: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit112 (0x80257544, 124 bytes)
#[inline(never)]
pub fn f_80257544(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257544) { return; }
        if c.prof { crate::prof::enter(c, 0x80257544); f_80257544_at(c, 0x80257544); crate::prof::exit(c, 0x80257544); return; }
    }
    f_80257544_b(c, 0x80257544);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257544_at(c: &mut Cpu, pc: u32) { f_80257544_b(c, pc) }
#[inline(always)]
fn f_80257544_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257544;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257544, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257544 => { ins += 16;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x3;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8025755c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025755c;
            }
            0x8025755c => { ins += 4;
                c.r[30] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = 0x4a;
                c.lr = 0x8025756c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f6e98(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025756c;
            }
            0x8025756c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80257588; continue; }
                pc = 0x80257578;
            }
            0x80257578 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x4a;
                c.lr = 0x80257584; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f6d9c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257584;
            }
            0x80257584 => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x80257588;
            }
            0x80257588 => { ins += 2;
                c.r[3] = 0x4a;
                c.lr = 0x80257590; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119dd0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257590;
            }
            0x80257590 => { ins += 4;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.r[30] as u16 as i16 as i32 as u32;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], c.r[3]);
                if cr_bit(cr, 0) { pc = 0x802575a8; continue; }
                pc = 0x802575a0;
            }
            0x802575a0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802575ac; continue;
                pc = 0x802575a8;
            }
            0x802575a8 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802575ac;
            }
            0x802575ac => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257544: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit111 (0x802575c0, 8 bytes)
#[inline(never)]
pub fn f_802575c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802575c0) { return; }
        if c.prof { crate::prof::enter(c, 0x802575c0); f_802575c0_at(c, 0x802575c0); crate::prof::exit(c, 0x802575c0); return; }
    }
    f_802575c0_b(c, 0x802575c0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802575c0_at(c: &mut Cpu, pc: u32) { f_802575c0_b(c, pc) }
#[inline(always)]
fn f_802575c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802575c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802575c0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802575c0 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802575c0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit109 (0x802575c8, 392 bytes)
#[inline(never)]
pub fn f_802575c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802575c8) { return; }
        if c.prof { crate::prof::enter(c, 0x802575c8); f_802575c8_at(c, 0x802575c8); crate::prof::exit(c, 0x802575c8); return; }
    }
    f_802575c8_b(c, 0x802575c8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802575c8_at(c: &mut Cpu, pc: u32) { f_802575c8_b(c, pc) }
#[inline(never)]
fn f_802575c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802575c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802575c8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802575c8 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[6] | c.r[6];
                c.r[26] = c.r[4] | c.r[4];
                c.r[25] = c.r[3] | c.r[3];
                c.r[27] = c.r[5] | c.r[5];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802575f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235aa0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802575f0;
            }
            0x802575f0 => { ins += 4;
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257600; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235a3c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257600;
            }
            0x80257600 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257610; { c.steps = st; c.cycles = ins; c.cr = cr; f_80235910(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257610;
            }
            0x80257610 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x7;
                c.lr = 0x80257624; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237dbc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257624;
            }
            0x80257624 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80257690; continue; }
                pc = 0x80257630;
            }
            0x80257630 => { ins += 4;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x14;
                c.lr = 0x80257640; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257640;
            }
            0x80257640 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80257654; continue; }
                pc = 0x8025764c;
            }
            0x8025764c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025773c; continue;
                pc = 0x80257654;
            }
            0x80257654 => { ins += 4;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x18;
                c.lr = 0x80257664; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257664;
            }
            0x80257664 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80257688; continue; }
                pc = 0x80257670;
            }
            0x80257670 => { ins += 3;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025767c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237288(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025767c;
            }
            0x8025767c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80257738; continue; }
                pc = 0x80257688;
            }
            0x80257688 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025773c; continue;
                pc = 0x80257690;
            }
            0x80257690 => { ins += 2;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802576b8; continue; }
                pc = 0x80257698;
            }
            0x80257698 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x802576b8; continue; }
                pc = 0x802576a4;
            }
            0x802576a4 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x802576b8; continue; }
                pc = 0x802576b0;
            }
            0x802576b0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025773c; continue;
                pc = 0x802576b8;
            }
            0x802576b8 => { ins += 8;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[27] | c.r[27];
                c.r[7] = 0x90;
                c.r[8] = 0x3;
                c.r[9] = 0x41;
                c.lr = 0x802576d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802576d8;
            }
            0x802576d8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80257738; continue; }
                pc = 0x802576e0;
            }
            0x802576e0 => { ins += 8;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[27] | c.r[27];
                c.r[7] = 0x10;
                c.r[8] = 0x1;
                c.r[9] = 0x41;
                c.lr = 0x80257700; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257700;
            }
            0x80257700 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80257738; continue; }
                pc = 0x80257708;
            }
            0x80257708 => { ins += 8;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[27] | c.r[27];
                c.r[7] = 0x10;
                c.r[8] = 0x2;
                c.r[9] = 0x41;
                c.lr = 0x80257728; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257728;
            }
            0x80257728 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80257738; continue; }
                pc = 0x80257730;
            }
            0x80257730 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8025773c; continue;
                pc = 0x80257738;
            }
            0x80257738 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x8025773c;
            }
            0x8025773c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802575c8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit108 (0x80257750, 232 bytes)
#[inline(never)]
pub fn f_80257750(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257750) { return; }
        if c.prof { crate::prof::enter(c, 0x80257750); f_80257750_at(c, 0x80257750); crate::prof::exit(c, 0x80257750); return; }
    }
    f_80257750_b(c, 0x80257750);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257750_at(c: &mut Cpu, pc: u32) { f_80257750_b(c, pc) }
#[inline(always)]
fn f_80257750_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257750;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257750, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257750 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x10;
                c.lr = 0x80257778; { c.steps = st; c.cycles = ins; c.cr = cr; f_8021c034(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257778;
            }
            0x80257778 => { ins += 2;
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x802577c0; continue; }
                pc = 0x80257780;
            }
            0x80257780 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x80257790; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257790;
            }
            0x80257790 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802577a4; continue; }
                pc = 0x8025779c;
            }
            0x8025779c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257824; continue;
                pc = 0x802577a4;
            }
            0x802577a4 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x802577bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802577bc;
            }
            0x802577bc => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x802577c0;
            }
            0x802577c0 => { ins += 8;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x7;
                c.r[9] = 0x41;
                c.lr = 0x802577e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c808(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802577e0;
            }
            0x802577e0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802577f0; continue; }
                pc = 0x802577e8;
            }
            0x802577e8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257824; continue;
                pc = 0x802577f0;
            }
            0x802577f0 => { ins += 2;
                c.r[3] = 0x10;
                c.lr = 0x802577f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8021c034(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802577f8;
            }
            0x802577f8 => { ins += 2;
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x80257820; continue; }
                pc = 0x80257800;
            }
            0x80257800 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80257810; continue; }
                pc = 0x80257808;
            }
            0x80257808 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257824; continue;
                pc = 0x80257810;
            }
            0x80257810 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80257820; continue; }
                pc = 0x80257818;
            }
            0x80257818 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80257824; continue;
                pc = 0x80257820;
            }
            0x80257820 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80257824;
            }
            0x80257824 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257750: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit107 (0x80257838, 140 bytes)
#[inline(never)]
pub fn f_80257838(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257838) { return; }
        if c.prof { crate::prof::enter(c, 0x80257838); f_80257838_at(c, 0x80257838); crate::prof::exit(c, 0x80257838); return; }
    }
    f_80257838_b(c, 0x80257838);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257838_at(c: &mut Cpu, pc: u32) { f_80257838_b(c, pc) }
#[inline(always)]
fn f_80257838_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257838;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257838, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257838 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x14;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[6] | c.r[6];
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8025785c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025785c;
            }
            0x8025785c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80257870; continue; }
                pc = 0x80257868;
            }
            0x80257868 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802578b0; continue;
                pc = 0x80257870;
            }
            0x80257870 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x17;
                c.lr = 0x80257880; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257880;
            }
            0x80257880 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80257894; continue; }
                pc = 0x8025788c;
            }
            0x8025788c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802578b0; continue;
                pc = 0x80257894;
            }
            0x80257894 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x8;
                c.lr = 0x802578a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802578a4;
            }
            0x802578a4 => { ins += 3;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                pc = 0x802578b0;
            }
            0x802578b0 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257838: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit106 (0x802578c4, 120 bytes)
#[inline(never)]
pub fn f_802578c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802578c4) { return; }
        if c.prof { crate::prof::enter(c, 0x802578c4); f_802578c4_at(c, 0x802578c4); crate::prof::exit(c, 0x802578c4); return; }
    }
    f_802578c4_b(c, 0x802578c4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802578c4_at(c: &mut Cpu, pc: u32) { f_802578c4_b(c, pc) }
#[inline(always)]
fn f_802578c4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802578c4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802578c4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802578c4 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x10000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0xfffffffe);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.lr = 0x802578e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802578e8;
            }
            0x802578e8 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x16;
                c.lr = 0x802578fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802578fc;
            }
            0x802578fc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80257910; continue; }
                pc = 0x80257908;
            }
            0x80257908 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257928; continue;
                pc = 0x80257910;
            }
            0x80257910 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80257920; continue; }
                pc = 0x80257918;
            }
            0x80257918 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257928; continue;
                pc = 0x80257920;
            }
            0x80257920 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80257928;
            }
            0x80257928 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802578c4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit105 (0x8025793c, 180 bytes)
#[inline(never)]
pub fn f_8025793c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025793c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025793c); f_8025793c_at(c, 0x8025793c); crate::prof::exit(c, 0x8025793c); return; }
    }
    f_8025793c_b(c, 0x8025793c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025793c_at(c: &mut Cpu, pc: u32) { f_8025793c_b(c, pc) }
#[inline(always)]
fn f_8025793c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8025793c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8025793c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8025793c => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257968; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257968;
            }
            0x80257968 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80257984; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257984;
            }
            0x80257984 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257994; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257994;
            }
            0x80257994 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x802579a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802579a8;
            }
            0x802579a8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x802579b8; continue; }
                pc = 0x802579b4;
            }
            0x802579b4 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x802579b8;
            }
            0x802579b8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802579c8; continue; }
                pc = 0x802579c0;
            }
            0x802579c0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802579dc; continue;
                pc = 0x802579c8;
            }
            0x802579c8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x802579d8; continue; }
                pc = 0x802579d0;
            }
            0x802579d0 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x802579dc; continue;
                pc = 0x802579d8;
            }
            0x802579d8 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802579dc;
            }
            0x802579dc => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8025793c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit104 (0x802579f0, 176 bytes)
#[inline(never)]
pub fn f_802579f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802579f0) { return; }
        if c.prof { crate::prof::enter(c, 0x802579f0); f_802579f0_at(c, 0x802579f0); crate::prof::exit(c, 0x802579f0); return; }
    }
    f_802579f0_b(c, 0x802579f0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802579f0_at(c: &mut Cpu, pc: u32) { f_802579f0_b(c, pc) }
#[inline(always)]
fn f_802579f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802579f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802579f0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802579f0 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257a1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257a1c;
            }
            0x80257a1c => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80257a3c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257a3c;
            }
            0x80257a3c => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257a50; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257a50;
            }
            0x80257a50 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80257a64; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257a64;
            }
            0x80257a64 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80257a74; continue; }
                pc = 0x80257a70;
            }
            0x80257a70 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80257a74;
            }
            0x80257a74 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80257a84; continue; }
                pc = 0x80257a7c;
            }
            0x80257a7c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257a8c; continue;
                pc = 0x80257a84;
            }
            0x80257a84 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80257a8c;
            }
            0x80257a8c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802579f0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit103 (0x80257aa0, 180 bytes)
#[inline(never)]
pub fn f_80257aa0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257aa0) { return; }
        if c.prof { crate::prof::enter(c, 0x80257aa0); f_80257aa0_at(c, 0x80257aa0); crate::prof::exit(c, 0x80257aa0); return; }
    }
    f_80257aa0_b(c, 0x80257aa0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257aa0_at(c: &mut Cpu, pc: u32) { f_80257aa0_b(c, pc) }
#[inline(always)]
fn f_80257aa0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257aa0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257aa0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257aa0 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257acc; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257acc;
            }
            0x80257acc => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80257ae8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257ae8;
            }
            0x80257ae8 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257af8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257af8;
            }
            0x80257af8 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80257b0c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257b0c;
            }
            0x80257b0c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80257b1c; continue; }
                pc = 0x80257b18;
            }
            0x80257b18 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80257b1c;
            }
            0x80257b1c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80257b2c; continue; }
                pc = 0x80257b24;
            }
            0x80257b24 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257b40; continue;
                pc = 0x80257b2c;
            }
            0x80257b2c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80257b3c; continue; }
                pc = 0x80257b34;
            }
            0x80257b34 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80257b40; continue;
                pc = 0x80257b3c;
            }
            0x80257b3c => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80257b40;
            }
            0x80257b40 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257aa0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit102 (0x80257b54, 204 bytes)
#[inline(never)]
pub fn f_80257b54(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257b54) { return; }
        if c.prof { crate::prof::enter(c, 0x80257b54); f_80257b54_at(c, 0x80257b54); crate::prof::exit(c, 0x80257b54); return; }
    }
    f_80257b54_b(c, 0x80257b54);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257b54_at(c: &mut Cpu, pc: u32) { f_80257b54_b(c, pc) }
#[inline(always)]
fn f_80257b54_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257b54;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257b54, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257b54 => { ins += 22;
                { let ea = c.r[1].wrapping_add(0xffffff70); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x1;
                c.r[7] = 0x1;
                c.stw(c.r[1].wrapping_add(0x94), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x74); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[3] | c.r[3];
                c.r[26] = c.r[5] | c.r[5];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[25] | c.r[25];
                c.r[3] = 0x0;
                c.lr = 0x80257b84; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1a6c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257b84;
            }
            0x80257b84 => { ins += 6;
                c.r[28] = c.r[1].wrapping_add(0x8);
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[30] = c.r[26].rotate_left(0) & 0xffff;
                c.r[26] = 0x0;
                c.r[27] = 0x0;
                pc = 0x80257bec; continue;
                pc = 0x80257b9c;
            }
            0x80257b9c => { ins += 4;
                c.r[29] = c.r[27].rotate_left(2) & 0x3fffc;
                c.r[4] = c.lwz_t(ins, c.r[28].wrapping_add(c.r[29]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { pc = 0x80257be8; continue; }
                pc = 0x80257bac;
            }
            0x80257bac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0xd7);
                if !cr_bit(cr, 2) { pc = 0x80257bcc; continue; }
                pc = 0x80257bb4;
            }
            0x80257bb4 => { ins += 3;
                c.r[3] = c.r[25] | c.r[25];
                c.r[5] = 0x2b;
                c.lr = 0x80257bc0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239058(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257bc0;
            }
            0x80257bc0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80257be8; continue; }
                pc = 0x80257bcc;
            }
            0x80257bcc => { ins += 3;
                c.r[4] = c.lwz_t(ins, c.r[28].wrapping_add(c.r[29]));
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80257bd8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80238748(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257bd8;
            }
            0x80257bd8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80257be8; continue; }
                pc = 0x80257be0;
            }
            0x80257be0 => { ins += 2;
                c.r[26] = 0x1;
                pc = 0x80257bf8; continue;
                pc = 0x80257be8;
            }
            0x80257be8 => { ins += 1;
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x80257bec;
            }
            0x80257bec => { ins += 3;
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x80257b9c; continue; }
                pc = 0x80257bf8;
            }
            0x80257bf8 => { ins += 2;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80257c08; continue; }
                pc = 0x80257c00;
            }
            0x80257c00 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257c0c; continue;
                pc = 0x80257c08;
            }
            0x80257c08 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80257c0c;
            }
            0x80257c0c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x74); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x94));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x90);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257b54: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit101 (0x80257c20, 180 bytes)
#[inline(never)]
pub fn f_80257c20(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257c20) { return; }
        if c.prof { crate::prof::enter(c, 0x80257c20); f_80257c20_at(c, 0x80257c20); crate::prof::exit(c, 0x80257c20); return; }
    }
    f_80257c20_b(c, 0x80257c20);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257c20_at(c: &mut Cpu, pc: u32) { f_80257c20_b(c, pc) }
#[inline(always)]
fn f_80257c20_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257c20;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257c20, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257c20 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257c4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257c4c;
            }
            0x80257c4c => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80257c68; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257c68;
            }
            0x80257c68 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257c78; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257c78;
            }
            0x80257c78 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80257c8c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257c8c;
            }
            0x80257c8c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80257c9c; continue; }
                pc = 0x80257c98;
            }
            0x80257c98 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80257c9c;
            }
            0x80257c9c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80257cac; continue; }
                pc = 0x80257ca4;
            }
            0x80257ca4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257cc0; continue;
                pc = 0x80257cac;
            }
            0x80257cac => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80257cbc; continue; }
                pc = 0x80257cb4;
            }
            0x80257cb4 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80257cc0; continue;
                pc = 0x80257cbc;
            }
            0x80257cbc => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80257cc0;
            }
            0x80257cc0 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257c20: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit100 (0x80257cd4, 292 bytes)
#[inline(never)]
pub fn f_80257cd4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257cd4) { return; }
        if c.prof { crate::prof::enter(c, 0x80257cd4); f_80257cd4_at(c, 0x80257cd4); crate::prof::exit(c, 0x80257cd4); return; }
    }
    f_80257cd4_b(c, 0x80257cd4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257cd4_at(c: &mut Cpu, pc: u32) { f_80257cd4_b(c, pc) }
#[inline(always)]
fn f_80257cd4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257cd4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257cd4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257cd4 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x10000;
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0xffffffff);
                { let mut ea = c.r[1].wrapping_add(0x3c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[6] | c.r[6];
                c.lr = 0x80257cf8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257cf8;
            }
            0x80257cf8 => { ins += 7;
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[1].wrapping_add(0x1c);
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[7] = 0x0;
                c.lr = 0x80257d14; { c.steps = st; c.cycles = ins; c.cr = cr; f_802367cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257d14;
            }
            0x80257d14 => { ins += 4;
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80257d24; { c.steps = st; c.cycles = ins; c.cr = cr; f_802364bc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257d24;
            }
            0x80257d24 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[29] = c.r[3] | c.r[3];
                if cr_bit(cr, 2) { pc = 0x80257d58; continue; }
                pc = 0x80257d30;
            }
            0x80257d30 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x165);
                if cr_bit(cr, 2) { pc = 0x80257d58; continue; }
                pc = 0x80257d38;
            }
            0x80257d38 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x80257d58; continue; }
                pc = 0x80257d40;
            }
            0x80257d40 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80257d4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80237288(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257d4c;
            }
            0x80257d4c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80257d60; continue; }
                pc = 0x80257d58;
            }
            0x80257d58 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257de4; continue;
                pc = 0x80257d60;
            }
            0x80257d60 => { ins += 7;
                c.r[7] = c.r[28].rotate_left(0) & 0xffff;
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[5] = c.r[29].rotate_left(0) & 0xffff;
                c.r[4] = c.r[1].wrapping_add(0x1c);
                c.r[3] = 0x0;
                c.r[10] = 0x0;
                pc = 0x80257db0; continue;
                pc = 0x80257d7c;
            }
            0x80257d7c => { ins += 4;
                c.r[8] = c.r[10].rotate_left(1) & 0x1fe;
                c.r[9] = c.lha_t(ins, c.r[6].wrapping_add(c.r[8]));
                c.r[0] = c.r[9] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80257dac; continue; }
                pc = 0x80257d8c;
            }
            0x80257d8c => { ins += 3;
                c.r[0] = c.lhz_t(ins, c.r[4].wrapping_add(c.r[8]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80257dac; continue; }
                pc = 0x80257d98;
            }
            0x80257d98 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[9].rotate_left(0) & 0xff;
                c.lr = 0x80257da8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802381c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257da8;
            }
            0x80257da8 => { ins += 1;
                pc = 0x80257dbc; continue;
                pc = 0x80257dac;
            }
            0x80257dac => { ins += 1;
                c.r[10] = c.r[10].wrapping_add(0x1);
                pc = 0x80257db0;
            }
            0x80257db0 => { ins += 3;
                c.r[0] = c.r[10].rotate_left(0) & 0xff;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], c.r[7]);
                if cr_bit(cr, 0) { pc = 0x80257d7c; continue; }
                pc = 0x80257dbc;
            }
            0x80257dbc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80257dcc; continue; }
                pc = 0x80257dc4;
            }
            0x80257dc4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257de4; continue;
                pc = 0x80257dcc;
            }
            0x80257dcc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80257ddc; continue; }
                pc = 0x80257dd4;
            }
            0x80257dd4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257de4; continue;
                pc = 0x80257ddc;
            }
            0x80257ddc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80257de4;
            }
            0x80257de4 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x3c); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257cd4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit099 (0x80257df8, 180 bytes)
#[inline(never)]
pub fn f_80257df8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257df8) { return; }
        if c.prof { crate::prof::enter(c, 0x80257df8); f_80257df8_at(c, 0x80257df8); crate::prof::exit(c, 0x80257df8); return; }
    }
    f_80257df8_b(c, 0x80257df8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257df8_at(c: &mut Cpu, pc: u32) { f_80257df8_b(c, pc) }
#[inline(always)]
fn f_80257df8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257df8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257df8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257df8 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257e24; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257e24;
            }
            0x80257e24 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80257e40; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257e40;
            }
            0x80257e40 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80257e50; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257e50;
            }
            0x80257e50 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80257e64; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257e64;
            }
            0x80257e64 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80257e74; continue; }
                pc = 0x80257e70;
            }
            0x80257e70 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x80257e74;
            }
            0x80257e74 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80257e84; continue; }
                pc = 0x80257e7c;
            }
            0x80257e7c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257e98; continue;
                pc = 0x80257e84;
            }
            0x80257e84 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x80257e94; continue; }
                pc = 0x80257e8c;
            }
            0x80257e8c => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80257e98; continue;
                pc = 0x80257e94;
            }
            0x80257e94 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80257e98;
            }
            0x80257e98 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257df8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit098 (0x80257eac, 56 bytes)
#[inline(never)]
pub fn f_80257eac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257eac) { return; }
        if c.prof { crate::prof::enter(c, 0x80257eac); f_80257eac_at(c, 0x80257eac); crate::prof::exit(c, 0x80257eac); return; }
    }
    f_80257eac_b(c, 0x80257eac);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257eac_at(c: &mut Cpu, pc: u32) { f_80257eac_b(c, pc) }
#[inline(always)]
fn f_80257eac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257eac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257eac, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257eac => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[5] | c.r[5];
                c.r[5] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80257ec4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80229934(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257ec4;
            }
            0x80257ec4 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257eac: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit097 (0x80257ee4, 228 bytes)
#[inline(never)]
pub fn f_80257ee4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257ee4) { return; }
        if c.prof { crate::prof::enter(c, 0x80257ee4); f_80257ee4_at(c, 0x80257ee4); crate::prof::exit(c, 0x80257ee4); return; }
    }
    f_80257ee4_b(c, 0x80257ee4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257ee4_at(c: &mut Cpu, pc: u32) { f_80257ee4_b(c, pc) }
#[inline(always)]
fn f_80257ee4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257ee4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257ee4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257ee4 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.r[5] = c.r[1].wrapping_add(0x8);
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[27] = c.r[4] | c.r[4];
                c.lr = 0x80257f0c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802367cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257f0c;
            }
            0x80257f0c => { ins += 5;
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x8;
                c.lr = 0x80257f20; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257f20;
            }
            0x80257f20 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80257f30; continue; }
                pc = 0x80257f28;
            }
            0x80257f28 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257fb4; continue;
                pc = 0x80257f30;
            }
            0x80257f30 => { ins += 4;
                c.r[30] = c.r[1].wrapping_add(0x8);
                c.r[31] = c.r[29].rotate_left(0) & 0xffff;
                c.r[28] = 0x0;
                pc = 0x80257f8c; continue;
                pc = 0x80257f40;
            }
            0x80257f40 => { ins += 4;
                c.r[0] = c.r[28].rotate_left(1) & 0x1fffe;
                c.r[27] = c.lhz_t(ins, c.r[30].wrapping_add(c.r[0]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x165);
                if cr_bit(cr, 2) { pc = 0x80257f88; continue; }
                pc = 0x80257f50;
            }
            0x80257f50 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x163);
                if cr_bit(cr, 2) { pc = 0x80257f88; continue; }
                pc = 0x80257f58;
            }
            0x80257f58 => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x80257f60; { c.steps = st; c.cycles = ins; c.cr = cr; f_80218fdc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257f60;
            }
            0x80257f60 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80257f88; continue; }
                pc = 0x80257f68;
            }
            0x80257f68 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x108);
                if cr_bit(cr, 2) { pc = 0x80257f88; continue; }
                pc = 0x80257f70;
            }
            0x80257f70 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0xfd);
                if cr_bit(cr, 2) { pc = 0x80257f88; continue; }
                pc = 0x80257f78;
            }
            0x80257f78 => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x80257f80; { c.steps = st; c.cycles = ins; c.cr = cr; f_8021901c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257f80;
            }
            0x80257f80 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80257f98; continue; }
                pc = 0x80257f88;
            }
            0x80257f88 => { ins += 1;
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x80257f8c;
            }
            0x80257f8c => { ins += 3;
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x80257f40; continue; }
                pc = 0x80257f98;
            }
            0x80257f98 => { ins += 4;
                c.r[3] = c.r[28].rotate_left(0) & 0xffff;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80257fb0; continue; }
                pc = 0x80257fa8;
            }
            0x80257fa8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80257fb4; continue;
                pc = 0x80257fb0;
            }
            0x80257fb0 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80257fb4;
            }
            0x80257fb4 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257ee4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit095 (0x80257fc8, 244 bytes)
#[inline(never)]
pub fn f_80257fc8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80257fc8) { return; }
        if c.prof { crate::prof::enter(c, 0x80257fc8); f_80257fc8_at(c, 0x80257fc8); crate::prof::exit(c, 0x80257fc8); return; }
    }
    f_80257fc8_b(c, 0x80257fc8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80257fc8_at(c: &mut Cpu, pc: u32) { f_80257fc8_b(c, pc) }
#[inline(always)]
fn f_80257fc8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80257fc8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80257fc8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80257fc8 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.r[5] = c.r[1].wrapping_add(0x8);
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[6] | c.r[6];
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[6] = 0x0;
                c.lr = 0x80257ff4; { c.steps = st; c.cycles = ins; c.cr = cr; f_802367cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80257ff4;
            }
            0x80257ff4 => { ins += 4;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80258004; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236458(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258004;
            }
            0x80258004 => { ins += 6;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x14;
                c.lr = 0x8025801c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8025801c;
            }
            0x8025801c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80258030; continue; }
                pc = 0x80258028;
            }
            0x80258028 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802580a8; continue;
                pc = 0x80258030;
            }
            0x80258030 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x10;
                c.lr = 0x80258040; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258040;
            }
            0x80258040 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80258060; continue; }
                pc = 0x8025804c;
            }
            0x8025804c => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80258054; { c.steps = st; c.cycles = ins; c.cr = cr; f_80219270(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258054;
            }
            0x80258054 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80258068; continue; }
                pc = 0x80258060;
            }
            0x80258060 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802580a8; continue;
                pc = 0x80258068;
            }
            0x80258068 => { ins += 5;
                c.r[5] = c.r[30].rotate_left(0) & 0xffff;
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.r[6] = 0x0;
                pc = 0x80258098; continue;
                pc = 0x8025807c;
            }
            0x8025807c => { ins += 4;
                c.r[3] = c.r[6].rotate_left(1) & 0x1fffe;
                c.r[3] = c.lhz_t(ins, c.r[4].wrapping_add(c.r[3]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x80258094; continue; }
                pc = 0x8025808c;
            }
            0x8025808c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802580a8; continue;
                pc = 0x80258094;
            }
            0x80258094 => { ins += 1;
                c.r[6] = c.r[6].wrapping_add(0x1);
                pc = 0x80258098;
            }
            0x80258098 => { ins += 3;
                c.r[3] = c.r[6].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8025807c; continue; }
                pc = 0x802580a4;
            }
            0x802580a4 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802580a8;
            }
            0x802580a8 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80257fc8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit094 (0x802580bc, 120 bytes)
#[inline(never)]
pub fn f_802580bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802580bc) { return; }
        if c.prof { crate::prof::enter(c, 0x802580bc); f_802580bc_at(c, 0x802580bc); crate::prof::exit(c, 0x802580bc); return; }
    }
    f_802580bc_b(c, 0x802580bc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802580bc_at(c: &mut Cpu, pc: u32) { f_802580bc_b(c, pc) }
#[inline(always)]
fn f_802580bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802580bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802580bc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802580bc => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x10000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0xffffffff);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.lr = 0x802580e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802580e0;
            }
            0x802580e0 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x802580f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802580f4;
            }
            0x802580f4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80258108; continue; }
                pc = 0x80258100;
            }
            0x80258100 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80258120; continue;
                pc = 0x80258108;
            }
            0x80258108 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80258118; continue; }
                pc = 0x80258110;
            }
            0x80258110 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80258120; continue;
                pc = 0x80258118;
            }
            0x80258118 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80258120;
            }
            0x80258120 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802580bc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit093 (0x80258134, 8 bytes)
#[inline(never)]
pub fn f_80258134(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258134) { return; }
        if c.prof { crate::prof::enter(c, 0x80258134); f_80258134_at(c, 0x80258134); crate::prof::exit(c, 0x80258134); return; }
    }
    f_80258134_b(c, 0x80258134);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258134_at(c: &mut Cpu, pc: u32) { f_80258134_b(c, pc) }
#[inline(always)]
fn f_80258134_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80258134;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80258134, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80258134 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80258134: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit092 (0x8025813c, 208 bytes)
#[inline(never)]
pub fn f_8025813c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025813c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025813c); f_8025813c_at(c, 0x8025813c); crate::prof::exit(c, 0x8025813c); return; }
    }
    f_8025813c_b(c, 0x8025813c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025813c_at(c: &mut Cpu, pc: u32) { f_8025813c_b(c, pc) }
#[inline(always)]
fn f_8025813c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8025813c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8025813c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8025813c => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258168; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258168;
            }
            0x80258168 => { ins += 6;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x8;
                c.lr = 0x80258180; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258180;
            }
            0x80258180 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80258190; continue; }
                pc = 0x80258188;
            }
            0x80258188 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802581f8; continue;
                pc = 0x80258190;
            }
            0x80258190 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x802581a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802581a8;
            }
            0x802581a8 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802581bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802581bc;
            }
            0x802581bc => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x802581d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802581d0;
            }
            0x802581d0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x802581e0; continue; }
                pc = 0x802581dc;
            }
            0x802581dc => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x802581e0;
            }
            0x802581e0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802581f0; continue; }
                pc = 0x802581e8;
            }
            0x802581e8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802581f8; continue;
                pc = 0x802581f0;
            }
            0x802581f0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802581f8;
            }
            0x802581f8 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8025813c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit091 (0x8025820c, 120 bytes)
#[inline(never)]
pub fn f_8025820c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025820c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025820c); f_8025820c_at(c, 0x8025820c); crate::prof::exit(c, 0x8025820c); return; }
    }
    f_8025820c_b(c, 0x8025820c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025820c_at(c: &mut Cpu, pc: u32) { f_8025820c_b(c, pc) }
#[inline(always)]
fn f_8025820c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8025820c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8025820c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8025820c => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x10000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0xffffffff);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.lr = 0x80258230; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258230;
            }
            0x80258230 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x80258244; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258244;
            }
            0x80258244 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80258258; continue; }
                pc = 0x80258250;
            }
            0x80258250 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80258270; continue;
                pc = 0x80258258;
            }
            0x80258258 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80258268; continue; }
                pc = 0x80258260;
            }
            0x80258260 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80258270; continue;
                pc = 0x80258268;
            }
            0x80258268 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80258270;
            }
            0x80258270 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8025820c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit090 (0x80258284, 320 bytes)
#[inline(never)]
pub fn f_80258284(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258284) { return; }
        if c.prof { crate::prof::enter(c, 0x80258284); f_80258284_at(c, 0x80258284); crate::prof::exit(c, 0x80258284); return; }
    }
    f_80258284_b(c, 0x80258284);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258284_at(c: &mut Cpu, pc: u32) { f_80258284_b(c, pc) }
#[inline(always)]
fn f_80258284_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80258284;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80258284, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80258284 => { ins += 23;
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x38); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[6] | c.r[6];
                c.r[26] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[1].wrapping_add(0x1c);
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.lr = 0x802582b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802367cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802582b8;
            }
            0x802582b8 => { ins += 4;
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x802582c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802364bc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802582c8;
            }
            0x802582c8 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[27] = c.r[0] | c.r[0];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[31] | c.r[31];
                c.r[7] = 0x0;
                c.lr = 0x802582e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802582e8;
            }
            0x802582e8 => { ins += 3;
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x802582f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80219804(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802582f4;
            }
            0x802582f4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80258308; continue; }
                pc = 0x80258300;
            }
            0x80258300 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802583b0; continue;
                pc = 0x80258308;
            }
            0x80258308 => { ins += 7;
                c.r[7] = c.r[1].wrapping_add(0x8);
                c.r[6] = c.r[27].rotate_left(0) & 0xffff;
                c.r[5] = c.r[1].wrapping_add(0x1c);
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[10] = 0x0;
                pc = 0x80258358; continue;
                pc = 0x80258324;
            }
            0x80258324 => { ins += 4;
                c.r[8] = c.r[10].rotate_left(1) & 0x1fffe;
                c.r[9] = c.lha_t(ins, c.r[7].wrapping_add(c.r[8]));
                c.r[4] = c.r[9] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[4]);
                if cr_bit(cr, 0) { pc = 0x80258354; continue; }
                pc = 0x80258334;
            }
            0x80258334 => { ins += 3;
                c.r[4] = c.lhz_t(ins, c.r[5].wrapping_add(c.r[8]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], c.r[4]);
                if !cr_bit(cr, 2) { pc = 0x80258354; continue; }
                pc = 0x80258340;
            }
            0x80258340 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[9].rotate_left(0) & 0xff;
                c.lr = 0x80258350; { c.steps = st; c.cycles = ins; c.cr = cr; f_802381c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258350;
            }
            0x80258350 => { ins += 1;
                pc = 0x80258364; continue;
                pc = 0x80258354;
            }
            0x80258354 => { ins += 1;
                c.r[10] = c.r[10].wrapping_add(0x1);
                pc = 0x80258358;
            }
            0x80258358 => { ins += 3;
                c.r[4] = c.r[10].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80258324; continue; }
                pc = 0x80258364;
            }
            0x80258364 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80258374; continue; }
                pc = 0x8025836c;
            }
            0x8025836c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802583b0; continue;
                pc = 0x80258374;
            }
            0x80258374 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x2a;
                c.lr = 0x80258384; { c.steps = st; c.cycles = ins; c.cr = cr; f_80236bfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258384;
            }
            0x80258384 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80258398; continue; }
                pc = 0x80258390;
            }
            0x80258390 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802583b0; continue;
                pc = 0x80258398;
            }
            0x80258398 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802583a8; continue; }
                pc = 0x802583a0;
            }
            0x802583a0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802583b0; continue;
                pc = 0x802583a8;
            }
            0x802583a8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802583b0;
            }
            0x802583b0 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x38); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80258284: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit089 (0x802583c4, 176 bytes)
#[inline(never)]
pub fn f_802583c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802583c4) { return; }
        if c.prof { crate::prof::enter(c, 0x802583c4); f_802583c4_at(c, 0x802583c4); crate::prof::exit(c, 0x802583c4); return; }
    }
    f_802583c4_b(c, 0x802583c4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802583c4_at(c: &mut Cpu, pc: u32) { f_802583c4_b(c, pc) }
#[inline(always)]
fn f_802583c4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802583c4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802583c4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802583c4 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802583f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802583f0;
            }
            0x802583f0 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80258410; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258410;
            }
            0x80258410 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258424; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258424;
            }
            0x80258424 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80258438; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258438;
            }
            0x80258438 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x80258448; continue; }
                pc = 0x80258444;
            }
            0x80258444 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x80258448;
            }
            0x80258448 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80258458; continue; }
                pc = 0x80258450;
            }
            0x80258450 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80258460; continue;
                pc = 0x80258458;
            }
            0x80258458 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80258460;
            }
            0x80258460 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802583c4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit088 (0x80258474, 176 bytes)
#[inline(never)]
pub fn f_80258474(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258474) { return; }
        if c.prof { crate::prof::enter(c, 0x80258474); f_80258474_at(c, 0x80258474); crate::prof::exit(c, 0x80258474); return; }
    }
    f_80258474_b(c, 0x80258474);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258474_at(c: &mut Cpu, pc: u32) { f_80258474_b(c, pc) }
#[inline(always)]
fn f_80258474_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80258474;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80258474, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80258474 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802584a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802584a0;
            }
            0x802584a0 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x802584c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802584c0;
            }
            0x802584c0 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802584d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802584d4;
            }
            0x802584d4 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x802584e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802584e8;
            }
            0x802584e8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x802584f8; continue; }
                pc = 0x802584f4;
            }
            0x802584f4 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x802584f8;
            }
            0x802584f8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80258508; continue; }
                pc = 0x80258500;
            }
            0x80258500 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80258510; continue;
                pc = 0x80258508;
            }
            0x80258508 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80258510;
            }
            0x80258510 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80258474: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit087 (0x80258524, 176 bytes)
#[inline(never)]
pub fn f_80258524(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258524) { return; }
        if c.prof { crate::prof::enter(c, 0x80258524); f_80258524_at(c, 0x80258524); crate::prof::exit(c, 0x80258524); return; }
    }
    f_80258524_b(c, 0x80258524);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258524_at(c: &mut Cpu, pc: u32) { f_80258524_b(c, pc) }
#[inline(always)]
fn f_80258524_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80258524;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80258524, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80258524 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258550; { c.steps = st; c.cycles = ins; c.cr = cr; f_802395c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258550;
            }
            0x80258550 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80258570; { c.steps = st; c.cycles = ins; c.cr = cr; f_8025c264(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258570;
            }
            0x80258570 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258584; { c.steps = st; c.cycles = ins; c.cr = cr; f_80239500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258584;
            }
            0x80258584 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80258598; { c.steps = st; c.cycles = ins; c.cr = cr; f_8023793c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80258598;
            }
            0x80258598 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x43);
                if !cr_bit(cr, 2) { pc = 0x802585a8; continue; }
                pc = 0x802585a4;
            }
            0x802585a4 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x802585a8;
            }
            0x802585a8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802585b8; continue; }
                pc = 0x802585b0;
            }
            0x802585b0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802585c0; continue;
                pc = 0x802585b8;
            }
            0x802585b8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802585c0;
            }
            0x802585c0 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80258524: bad pc {:#x}", pc),
        }
    }
}

