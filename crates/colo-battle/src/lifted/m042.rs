#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// fightTrainerAiWazaHit086 (0x802585d4, 288 bytes)
#[inline(never)]
pub fn f_802585d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802585d4) { return; }
        if c.prof { crate::prof::enter(c, 0x802585d4); f_802585d4_b(c, 0x802585d4); crate::prof::exit(c, 0x802585d4); return; }
    }
    f_802585d4_b(c, 0x802585d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802585d4_at(c: &mut Cpu, pc: u32) { f_802585d4_b(c, pc) }
#[inline(always)]
fn f_802585d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802585d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802585d4, pc); }
        match pc {
            0x802585d4 => {
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
                c.lr = 0x80258608; f_802367cc(c);
                pc = 0x80258608;
            }
            0x80258608 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80258618; f_802364bc(c);
                pc = 0x80258618;
            }
            0x80258618 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[31] | c.r[31];
                c.r[7] = 0x0;
                c.lr = 0x80258634; f_8025c264(c);
                pc = 0x80258634;
            }
            0x80258634 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[7] = c.r[1].wrapping_add(0x8);
                c.r[6] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = c.r[1].wrapping_add(0x1c);
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[10] = 0x0;
                pc = 0x80258688; continue;
                pc = 0x80258654;
            }
            0x80258654 => {
                c.r[8] = c.r[10].rotate_left(1) & 0x1fffe;
                c.r[9] = c.lha(c.r[7].wrapping_add(c.r[8]));
                c.r[4] = c.r[9] as u16 as i16 as i32 as u32;
                c.rc(c.r[4]);
                if c.crbit(0) { pc = 0x80258684; continue; }
                pc = 0x80258664;
            }
            0x80258664 => {
                c.r[4] = c.lhz(c.r[5].wrapping_add(c.r[8]));
                c.cmpu(0, c.r[6], c.r[4]);
                if !c.crbit(2) { pc = 0x80258684; continue; }
                pc = 0x80258670;
            }
            0x80258670 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[9].rotate_left(0) & 0xff;
                c.lr = 0x80258680; f_802381c4(c);
                pc = 0x80258680;
            }
            0x80258680 => {
                pc = 0x80258694; continue;
                pc = 0x80258684;
            }
            0x80258684 => {
                c.r[10] = c.r[10].wrapping_add(0x1);
                pc = 0x80258688;
            }
            0x80258688 => {
                c.r[4] = c.r[10].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x80258654; continue; }
                pc = 0x80258694;
            }
            0x80258694 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802586a4; continue; }
                pc = 0x8025869c;
            }
            0x8025869c => {
                c.r[3] = 0x0;
                pc = 0x802586e0; continue;
                pc = 0x802586a4;
            }
            0x802586a4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x29;
                c.lr = 0x802586b4; f_80236bfc(c);
                pc = 0x802586b4;
            }
            0x802586b4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802586c8; continue; }
                pc = 0x802586c0;
            }
            0x802586c0 => {
                c.r[3] = 0x0;
                pc = 0x802586e0; continue;
                pc = 0x802586c8;
            }
            0x802586c8 => {
                c.cmps(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x802586d8; continue; }
                pc = 0x802586d0;
            }
            0x802586d0 => {
                c.r[3] = 0x0;
                pc = 0x802586e0; continue;
                pc = 0x802586d8;
            }
            0x802586d8 => {
                c.cmps(0, c.r[29], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802586e0;
            }
            0x802586e0 => {
                { let mut ea = c.r[1].wrapping_add(0x38); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { return; }
                return;
            }
            _ => panic!("f_802585d4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit085 (0x802586f4, 8 bytes)
#[inline(never)]
pub fn f_802586f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802586f4) { return; }
        if c.prof { crate::prof::enter(c, 0x802586f4); f_802586f4_b(c, 0x802586f4); crate::prof::exit(c, 0x802586f4); return; }
    }
    f_802586f4_b(c, 0x802586f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802586f4_at(c: &mut Cpu, pc: u32) { f_802586f4_b(c, pc) }
#[inline(always)]
fn f_802586f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802586f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802586f4, pc); }
        match pc {
            0x802586f4 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_802586f4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit084 (0x802586fc, 188 bytes)
#[inline(never)]
pub fn f_802586fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802586fc) { return; }
        if c.prof { crate::prof::enter(c, 0x802586fc); f_802586fc_b(c, 0x802586fc); crate::prof::exit(c, 0x802586fc); return; }
    }
    f_802586fc_b(c, 0x802586fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802586fc_at(c: &mut Cpu, pc: u32) { f_802586fc_b(c, pc) }
#[inline(always)]
fn f_802586fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802586fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802586fc, pc); }
        match pc {
            0x802586fc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.lr = 0x8025871c; f_8025c264(c);
                pc = 0x8025871c;
            }
            0x8025871c => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x1c;
                c.lr = 0x80258730; f_80236bfc(c);
                pc = 0x80258730;
            }
            0x80258730 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80258744; continue; }
                pc = 0x8025873c;
            }
            0x8025873c => {
                c.r[3] = 0x0;
                pc = 0x802587a4; continue;
                pc = 0x80258744;
            }
            0x80258744 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0xc;
                c.lr = 0x80258754; f_80237dbc(c);
                pc = 0x80258754;
            }
            0x80258754 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80258768; continue; }
                pc = 0x80258760;
            }
            0x80258760 => {
                c.r[3] = 0x0;
                pc = 0x802587a4; continue;
                pc = 0x80258768;
            }
            0x80258768 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x80258778; f_80236bfc(c);
                pc = 0x80258778;
            }
            0x80258778 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025878c; continue; }
                pc = 0x80258784;
            }
            0x80258784 => {
                c.r[3] = 0x0;
                pc = 0x802587a4; continue;
                pc = 0x8025878c;
            }
            0x8025878c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025879c; continue; }
                pc = 0x80258794;
            }
            0x80258794 => {
                c.r[3] = 0x0;
                pc = 0x802587a4; continue;
                pc = 0x8025879c;
            }
            0x8025879c => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802587a4;
            }
            0x802587a4 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802586fc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit083 (0x802587b8, 8 bytes)
#[inline(never)]
pub fn f_802587b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802587b8) { return; }
        if c.prof { crate::prof::enter(c, 0x802587b8); f_802587b8_b(c, 0x802587b8); crate::prof::exit(c, 0x802587b8); return; }
    }
    f_802587b8_b(c, 0x802587b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802587b8_at(c: &mut Cpu, pc: u32) { f_802587b8_b(c, pc) }
#[inline(always)]
fn f_802587b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802587b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802587b8, pc); }
        match pc {
            0x802587b8 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_802587b8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit082 (0x802587c0, 324 bytes)
#[inline(never)]
pub fn f_802587c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802587c0) { return; }
        if c.prof { crate::prof::enter(c, 0x802587c0); f_802587c0_b(c, 0x802587c0); crate::prof::exit(c, 0x802587c0); return; }
    }
    f_802587c0_b(c, 0x802587c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802587c0_at(c: &mut Cpu, pc: u32) { f_802587c0_b(c, pc) }
#[inline(always)]
fn f_802587c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802587c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802587c0, pc); }
        match pc {
            0x802587c0 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x0;
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x28); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[5] | c.r[5];
                c.r[31] = c.r[6] | c.r[6];
                c.r[26] = c.r[3] | c.r[3];
                c.r[27] = c.r[4] | c.r[4];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[6] = 0x0;
                c.lr = 0x802587f0; f_802367cc(c);
                pc = 0x802587f0;
            }
            0x802587f0 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80258800; f_802364bc(c);
                pc = 0x80258800;
            }
            0x80258800 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x14;
                c.lr = 0x80258818; f_80236bfc(c);
                pc = 0x80258818;
            }
            0x80258818 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025882c; continue; }
                pc = 0x80258824;
            }
            0x80258824 => {
                c.r[3] = 0x0;
                pc = 0x802588f0; continue;
                pc = 0x8025882c;
            }
            0x8025882c => {
                c.r[7] = 0x10000;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[7] = c.r[7].wrapping_add(0xffffffff);
                c.lr = 0x80258848; f_8025c264(c);
                pc = 0x80258848;
            }
            0x80258848 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x10;
                c.lr = 0x8025885c; f_80236bfc(c);
                pc = 0x8025885c;
            }
            0x8025885c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80258898; continue; }
                pc = 0x80258868;
            }
            0x80258868 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80258870; f_8021a2c0(c);
                pc = 0x80258870;
            }
            0x80258870 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80258898; continue; }
                pc = 0x80258878;
            }
            0x80258878 => {
                c.r[5] = c.r[29].rotate_left(0) & 0xffff;
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x80258898; continue; }
                pc = 0x80258880;
            }
            0x80258880 => {
                c.cmpu(0, c.r[5], 0xffff);
                if c.crbit(2) { pc = 0x80258898; continue; }
                pc = 0x80258888;
            }
            0x80258888 => {
                c.cmpu(0, c.r[5], 0x165);
                if c.crbit(2) { pc = 0x80258898; continue; }
                pc = 0x80258890;
            }
            0x80258890 => {
                c.cmpu(0, c.r[5], 0x163);
                if !c.crbit(2) { pc = 0x802588a0; continue; }
                pc = 0x80258898;
            }
            0x80258898 => {
                c.r[3] = 0x0;
                pc = 0x802588f0; continue;
                pc = 0x802588a0;
            }
            0x802588a0 => {
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.r[6] = 0x0;
                pc = 0x802588cc; continue;
                pc = 0x802588b0;
            }
            0x802588b0 => {
                c.r[3] = c.r[6].rotate_left(1) & 0x1fffe;
                c.r[3] = c.lhz(c.r[4].wrapping_add(c.r[3]));
                c.cmpu(0, c.r[5], c.r[3]);
                if !c.crbit(2) { pc = 0x802588c8; continue; }
                pc = 0x802588c0;
            }
            0x802588c0 => {
                c.r[3] = 0x0;
                pc = 0x802588f0; continue;
                pc = 0x802588c8;
            }
            0x802588c8 => {
                c.r[6] = c.r[6].wrapping_add(0x1);
                pc = 0x802588cc;
            }
            0x802588cc => {
                c.r[3] = c.r[6].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x802588b0; continue; }
                pc = 0x802588d8;
            }
            0x802588d8 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x802588e8; continue; }
                pc = 0x802588e0;
            }
            0x802588e0 => {
                c.r[3] = 0x0;
                pc = 0x802588f0; continue;
                pc = 0x802588e8;
            }
            0x802588e8 => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802588f0;
            }
            0x802588f0 => {
                { let mut ea = c.r[1].wrapping_add(0x28); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_802587c0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit081 (0x80258904, 176 bytes)
#[inline(never)]
pub fn f_80258904(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258904) { return; }
        if c.prof { crate::prof::enter(c, 0x80258904); f_80258904_b(c, 0x80258904); crate::prof::exit(c, 0x80258904); return; }
    }
    f_80258904_b(c, 0x80258904)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258904_at(c: &mut Cpu, pc: u32) { f_80258904_b(c, pc) }
#[inline(always)]
fn f_80258904_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80258904;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80258904, pc); }
        match pc {
            0x80258904 => {
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
                c.lr = 0x80258930; f_802395c8(c);
                pc = 0x80258930;
            }
            0x80258930 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80258950; f_8025c264(c);
                pc = 0x80258950;
            }
            0x80258950 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258964; f_80239500(c);
                pc = 0x80258964;
            }
            0x80258964 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80258978; f_8023793c(c);
                pc = 0x80258978;
            }
            0x80258978 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x80258988; continue; }
                pc = 0x80258984;
            }
            0x80258984 => {
                c.r[31] = 0x0;
                pc = 0x80258988;
            }
            0x80258988 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80258998; continue; }
                pc = 0x80258990;
            }
            0x80258990 => {
                c.r[3] = 0x0;
                pc = 0x802589a0; continue;
                pc = 0x80258998;
            }
            0x80258998 => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x802589a0;
            }
            0x802589a0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80258904: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit080 (0x802589b4, 176 bytes)
#[inline(never)]
pub fn f_802589b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802589b4) { return; }
        if c.prof { crate::prof::enter(c, 0x802589b4); f_802589b4_b(c, 0x802589b4); crate::prof::exit(c, 0x802589b4); return; }
    }
    f_802589b4_b(c, 0x802589b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802589b4_at(c: &mut Cpu, pc: u32) { f_802589b4_b(c, pc) }
#[inline(always)]
fn f_802589b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802589b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802589b4, pc); }
        match pc {
            0x802589b4 => {
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
                c.lr = 0x802589e0; f_802395c8(c);
                pc = 0x802589e0;
            }
            0x802589e0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80258a00; f_8025c264(c);
                pc = 0x80258a00;
            }
            0x80258a00 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258a14; f_80239500(c);
                pc = 0x80258a14;
            }
            0x80258a14 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80258a28; f_8023793c(c);
                pc = 0x80258a28;
            }
            0x80258a28 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x80258a38; continue; }
                pc = 0x80258a34;
            }
            0x80258a34 => {
                c.r[31] = 0x0;
                pc = 0x80258a38;
            }
            0x80258a38 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80258a48; continue; }
                pc = 0x80258a40;
            }
            0x80258a40 => {
                c.r[3] = 0x0;
                pc = 0x80258a50; continue;
                pc = 0x80258a48;
            }
            0x80258a48 => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80258a50;
            }
            0x80258a50 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802589b4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit079 (0x80258a64, 108 bytes)
#[inline(never)]
pub fn f_80258a64(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258a64) { return; }
        if c.prof { crate::prof::enter(c, 0x80258a64); f_80258a64_b(c, 0x80258a64); crate::prof::exit(c, 0x80258a64); return; }
    }
    f_80258a64_b(c, 0x80258a64)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258a64_at(c: &mut Cpu, pc: u32) { f_80258a64_b(c, pc) }
#[inline(always)]
fn f_80258a64_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80258a64;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80258a64, pc); }
        match pc {
            0x80258a64 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x14;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x80258a84; f_80236bfc(c);
                pc = 0x80258a84;
            }
            0x80258a84 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80258a98; continue; }
                pc = 0x80258a90;
            }
            0x80258a90 => {
                c.r[3] = 0x0;
                pc = 0x80258abc; continue;
                pc = 0x80258a98;
            }
            0x80258a98 => {
                { let ea = c.r[2].wrapping_add(0xffffafb0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0xffffffff;
                c.lr = 0x80258aac; f_802373b0(c);
                pc = 0x80258aac;
            }
            0x80258aac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                pc = 0x80258abc;
            }
            0x80258abc => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80258a64: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit078 (0x80258ad0, 180 bytes)
#[inline(never)]
pub fn f_80258ad0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258ad0) { return; }
        if c.prof { crate::prof::enter(c, 0x80258ad0); f_80258ad0_b(c, 0x80258ad0); crate::prof::exit(c, 0x80258ad0); return; }
    }
    f_80258ad0_b(c, 0x80258ad0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258ad0_at(c: &mut Cpu, pc: u32) { f_80258ad0_b(c, pc) }
#[inline(always)]
fn f_80258ad0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80258ad0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80258ad0, pc); }
        match pc {
            0x80258ad0 => {
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
                c.lr = 0x80258afc; f_802395c8(c);
                pc = 0x80258afc;
            }
            0x80258afc => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80258b18; f_8025c264(c);
                pc = 0x80258b18;
            }
            0x80258b18 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258b28; f_80239500(c);
                pc = 0x80258b28;
            }
            0x80258b28 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80258b3c; f_8023793c(c);
                pc = 0x80258b3c;
            }
            0x80258b3c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x80258b4c; continue; }
                pc = 0x80258b48;
            }
            0x80258b48 => {
                c.r[30] = 0x0;
                pc = 0x80258b4c;
            }
            0x80258b4c => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80258b5c; continue; }
                pc = 0x80258b54;
            }
            0x80258b54 => {
                c.r[3] = 0x0;
                pc = 0x80258b70; continue;
                pc = 0x80258b5c;
            }
            0x80258b5c => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x80258b6c; continue; }
                pc = 0x80258b64;
            }
            0x80258b64 => {
                c.r[3] = 0x1;
                pc = 0x80258b70; continue;
                pc = 0x80258b6c;
            }
            0x80258b6c => {
                c.r[3] = 0x1;
                pc = 0x80258b70;
            }
            0x80258b70 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80258ad0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit077 (0x80258b84, 180 bytes)
#[inline(never)]
pub fn f_80258b84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258b84) { return; }
        if c.prof { crate::prof::enter(c, 0x80258b84); f_80258b84_b(c, 0x80258b84); crate::prof::exit(c, 0x80258b84); return; }
    }
    f_80258b84_b(c, 0x80258b84)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258b84_at(c: &mut Cpu, pc: u32) { f_80258b84_b(c, pc) }
#[inline(always)]
fn f_80258b84_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80258b84;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80258b84, pc); }
        match pc {
            0x80258b84 => {
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
                c.lr = 0x80258bb0; f_802395c8(c);
                pc = 0x80258bb0;
            }
            0x80258bb0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80258bcc; f_8025c264(c);
                pc = 0x80258bcc;
            }
            0x80258bcc => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258bdc; f_80239500(c);
                pc = 0x80258bdc;
            }
            0x80258bdc => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80258bf0; f_8023793c(c);
                pc = 0x80258bf0;
            }
            0x80258bf0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x80258c00; continue; }
                pc = 0x80258bfc;
            }
            0x80258bfc => {
                c.r[30] = 0x0;
                pc = 0x80258c00;
            }
            0x80258c00 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80258c10; continue; }
                pc = 0x80258c08;
            }
            0x80258c08 => {
                c.r[3] = 0x0;
                pc = 0x80258c24; continue;
                pc = 0x80258c10;
            }
            0x80258c10 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x80258c20; continue; }
                pc = 0x80258c18;
            }
            0x80258c18 => {
                c.r[3] = 0x1;
                pc = 0x80258c24; continue;
                pc = 0x80258c20;
            }
            0x80258c20 => {
                c.r[3] = 0x1;
                pc = 0x80258c24;
            }
            0x80258c24 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80258b84: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit076 (0x80258c38, 180 bytes)
#[inline(never)]
pub fn f_80258c38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258c38) { return; }
        if c.prof { crate::prof::enter(c, 0x80258c38); f_80258c38_b(c, 0x80258c38); crate::prof::exit(c, 0x80258c38); return; }
    }
    f_80258c38_b(c, 0x80258c38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258c38_at(c: &mut Cpu, pc: u32) { f_80258c38_b(c, pc) }
#[inline(always)]
fn f_80258c38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80258c38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80258c38, pc); }
        match pc {
            0x80258c38 => {
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
                c.lr = 0x80258c64; f_802395c8(c);
                pc = 0x80258c64;
            }
            0x80258c64 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80258c80; f_8025c264(c);
                pc = 0x80258c80;
            }
            0x80258c80 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258c90; f_80239500(c);
                pc = 0x80258c90;
            }
            0x80258c90 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80258ca4; f_8023793c(c);
                pc = 0x80258ca4;
            }
            0x80258ca4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x80258cb4; continue; }
                pc = 0x80258cb0;
            }
            0x80258cb0 => {
                c.r[30] = 0x0;
                pc = 0x80258cb4;
            }
            0x80258cb4 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80258cc4; continue; }
                pc = 0x80258cbc;
            }
            0x80258cbc => {
                c.r[3] = 0x0;
                pc = 0x80258cd8; continue;
                pc = 0x80258cc4;
            }
            0x80258cc4 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x80258cd4; continue; }
                pc = 0x80258ccc;
            }
            0x80258ccc => {
                c.r[3] = 0x1;
                pc = 0x80258cd8; continue;
                pc = 0x80258cd4;
            }
            0x80258cd4 => {
                c.r[3] = 0x1;
                pc = 0x80258cd8;
            }
            0x80258cd8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80258c38: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit075 (0x80258cec, 180 bytes)
#[inline(never)]
pub fn f_80258cec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258cec) { return; }
        if c.prof { crate::prof::enter(c, 0x80258cec); f_80258cec_b(c, 0x80258cec); crate::prof::exit(c, 0x80258cec); return; }
    }
    f_80258cec_b(c, 0x80258cec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258cec_at(c: &mut Cpu, pc: u32) { f_80258cec_b(c, pc) }
#[inline(always)]
fn f_80258cec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80258cec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80258cec, pc); }
        match pc {
            0x80258cec => {
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
                c.lr = 0x80258d18; f_802395c8(c);
                pc = 0x80258d18;
            }
            0x80258d18 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80258d34; f_8025c264(c);
                pc = 0x80258d34;
            }
            0x80258d34 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258d44; f_80239500(c);
                pc = 0x80258d44;
            }
            0x80258d44 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80258d58; f_8023793c(c);
                pc = 0x80258d58;
            }
            0x80258d58 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x80258d68; continue; }
                pc = 0x80258d64;
            }
            0x80258d64 => {
                c.r[30] = 0x0;
                pc = 0x80258d68;
            }
            0x80258d68 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80258d78; continue; }
                pc = 0x80258d70;
            }
            0x80258d70 => {
                c.r[3] = 0x0;
                pc = 0x80258d8c; continue;
                pc = 0x80258d78;
            }
            0x80258d78 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x80258d88; continue; }
                pc = 0x80258d80;
            }
            0x80258d80 => {
                c.r[3] = 0x1;
                pc = 0x80258d8c; continue;
                pc = 0x80258d88;
            }
            0x80258d88 => {
                c.r[3] = 0x1;
                pc = 0x80258d8c;
            }
            0x80258d8c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80258cec: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit073 (0x80258da0, 180 bytes)
#[inline(never)]
pub fn f_80258da0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258da0) { return; }
        if c.prof { crate::prof::enter(c, 0x80258da0); f_80258da0_b(c, 0x80258da0); crate::prof::exit(c, 0x80258da0); return; }
    }
    f_80258da0_b(c, 0x80258da0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258da0_at(c: &mut Cpu, pc: u32) { f_80258da0_b(c, pc) }
#[inline(always)]
fn f_80258da0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80258da0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80258da0, pc); }
        match pc {
            0x80258da0 => {
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
                c.lr = 0x80258dcc; f_802395c8(c);
                pc = 0x80258dcc;
            }
            0x80258dcc => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80258de8; f_8025c264(c);
                pc = 0x80258de8;
            }
            0x80258de8 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258df8; f_80239500(c);
                pc = 0x80258df8;
            }
            0x80258df8 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80258e0c; f_8023793c(c);
                pc = 0x80258e0c;
            }
            0x80258e0c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x80258e1c; continue; }
                pc = 0x80258e18;
            }
            0x80258e18 => {
                c.r[30] = 0x0;
                pc = 0x80258e1c;
            }
            0x80258e1c => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80258e2c; continue; }
                pc = 0x80258e24;
            }
            0x80258e24 => {
                c.r[3] = 0x0;
                pc = 0x80258e40; continue;
                pc = 0x80258e2c;
            }
            0x80258e2c => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x80258e3c; continue; }
                pc = 0x80258e34;
            }
            0x80258e34 => {
                c.r[3] = 0x1;
                pc = 0x80258e40; continue;
                pc = 0x80258e3c;
            }
            0x80258e3c => {
                c.r[3] = 0x1;
                pc = 0x80258e40;
            }
            0x80258e40 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80258da0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit072 (0x80258e54, 180 bytes)
#[inline(never)]
pub fn f_80258e54(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258e54) { return; }
        if c.prof { crate::prof::enter(c, 0x80258e54); f_80258e54_b(c, 0x80258e54); crate::prof::exit(c, 0x80258e54); return; }
    }
    f_80258e54_b(c, 0x80258e54)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258e54_at(c: &mut Cpu, pc: u32) { f_80258e54_b(c, pc) }
#[inline(always)]
fn f_80258e54_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80258e54;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80258e54, pc); }
        match pc {
            0x80258e54 => {
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
                c.lr = 0x80258e80; f_802395c8(c);
                pc = 0x80258e80;
            }
            0x80258e80 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80258e9c; f_8025c264(c);
                pc = 0x80258e9c;
            }
            0x80258e9c => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258eac; f_80239500(c);
                pc = 0x80258eac;
            }
            0x80258eac => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80258ec0; f_8023793c(c);
                pc = 0x80258ec0;
            }
            0x80258ec0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x80258ed0; continue; }
                pc = 0x80258ecc;
            }
            0x80258ecc => {
                c.r[30] = 0x0;
                pc = 0x80258ed0;
            }
            0x80258ed0 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80258ee0; continue; }
                pc = 0x80258ed8;
            }
            0x80258ed8 => {
                c.r[3] = 0x0;
                pc = 0x80258ef4; continue;
                pc = 0x80258ee0;
            }
            0x80258ee0 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x80258ef0; continue; }
                pc = 0x80258ee8;
            }
            0x80258ee8 => {
                c.r[3] = 0x1;
                pc = 0x80258ef4; continue;
                pc = 0x80258ef0;
            }
            0x80258ef0 => {
                c.r[3] = 0x1;
                pc = 0x80258ef4;
            }
            0x80258ef4 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80258e54: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit071 (0x80258f08, 180 bytes)
#[inline(never)]
pub fn f_80258f08(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258f08) { return; }
        if c.prof { crate::prof::enter(c, 0x80258f08); f_80258f08_b(c, 0x80258f08); crate::prof::exit(c, 0x80258f08); return; }
    }
    f_80258f08_b(c, 0x80258f08)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258f08_at(c: &mut Cpu, pc: u32) { f_80258f08_b(c, pc) }
#[inline(always)]
fn f_80258f08_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80258f08;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80258f08, pc); }
        match pc {
            0x80258f08 => {
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
                c.lr = 0x80258f34; f_802395c8(c);
                pc = 0x80258f34;
            }
            0x80258f34 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80258f50; f_8025c264(c);
                pc = 0x80258f50;
            }
            0x80258f50 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80258f60; f_80239500(c);
                pc = 0x80258f60;
            }
            0x80258f60 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80258f74; f_8023793c(c);
                pc = 0x80258f74;
            }
            0x80258f74 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x80258f84; continue; }
                pc = 0x80258f80;
            }
            0x80258f80 => {
                c.r[30] = 0x0;
                pc = 0x80258f84;
            }
            0x80258f84 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80258f94; continue; }
                pc = 0x80258f8c;
            }
            0x80258f8c => {
                c.r[3] = 0x0;
                pc = 0x80258fa8; continue;
                pc = 0x80258f94;
            }
            0x80258f94 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x80258fa4; continue; }
                pc = 0x80258f9c;
            }
            0x80258f9c => {
                c.r[3] = 0x1;
                pc = 0x80258fa8; continue;
                pc = 0x80258fa4;
            }
            0x80258fa4 => {
                c.r[3] = 0x1;
                pc = 0x80258fa8;
            }
            0x80258fa8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80258f08: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit070 (0x80258fbc, 180 bytes)
#[inline(never)]
pub fn f_80258fbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80258fbc) { return; }
        if c.prof { crate::prof::enter(c, 0x80258fbc); f_80258fbc_b(c, 0x80258fbc); crate::prof::exit(c, 0x80258fbc); return; }
    }
    f_80258fbc_b(c, 0x80258fbc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80258fbc_at(c: &mut Cpu, pc: u32) { f_80258fbc_b(c, pc) }
#[inline(always)]
fn f_80258fbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80258fbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80258fbc, pc); }
        match pc {
            0x80258fbc => {
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
                c.lr = 0x80258fe8; f_802395c8(c);
                pc = 0x80258fe8;
            }
            0x80258fe8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x80259004; f_8025c264(c);
                pc = 0x80259004;
            }
            0x80259004 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80259014; f_80239500(c);
                pc = 0x80259014;
            }
            0x80259014 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80259028; f_8023793c(c);
                pc = 0x80259028;
            }
            0x80259028 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x80259038; continue; }
                pc = 0x80259034;
            }
            0x80259034 => {
                c.r[30] = 0x0;
                pc = 0x80259038;
            }
            0x80259038 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80259048; continue; }
                pc = 0x80259040;
            }
            0x80259040 => {
                c.r[3] = 0x0;
                pc = 0x8025905c; continue;
                pc = 0x80259048;
            }
            0x80259048 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x80259058; continue; }
                pc = 0x80259050;
            }
            0x80259050 => {
                c.r[3] = 0x1;
                pc = 0x8025905c; continue;
                pc = 0x80259058;
            }
            0x80259058 => {
                c.r[3] = 0x1;
                pc = 0x8025905c;
            }
            0x8025905c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80258fbc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit069 (0x80259070, 180 bytes)
#[inline(never)]
pub fn f_80259070(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80259070) { return; }
        if c.prof { crate::prof::enter(c, 0x80259070); f_80259070_b(c, 0x80259070); crate::prof::exit(c, 0x80259070); return; }
    }
    f_80259070_b(c, 0x80259070)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80259070_at(c: &mut Cpu, pc: u32) { f_80259070_b(c, pc) }
#[inline(always)]
fn f_80259070_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80259070;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80259070, pc); }
        match pc {
            0x80259070 => {
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
                c.lr = 0x8025909c; f_802395c8(c);
                pc = 0x8025909c;
            }
            0x8025909c => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x802590b8; f_8025c264(c);
                pc = 0x802590b8;
            }
            0x802590b8 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802590c8; f_80239500(c);
                pc = 0x802590c8;
            }
            0x802590c8 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x802590dc; f_8023793c(c);
                pc = 0x802590dc;
            }
            0x802590dc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x802590ec; continue; }
                pc = 0x802590e8;
            }
            0x802590e8 => {
                c.r[30] = 0x0;
                pc = 0x802590ec;
            }
            0x802590ec => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x802590fc; continue; }
                pc = 0x802590f4;
            }
            0x802590f4 => {
                c.r[3] = 0x0;
                pc = 0x80259110; continue;
                pc = 0x802590fc;
            }
            0x802590fc => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025910c; continue; }
                pc = 0x80259104;
            }
            0x80259104 => {
                c.r[3] = 0x1;
                pc = 0x80259110; continue;
                pc = 0x8025910c;
            }
            0x8025910c => {
                c.r[3] = 0x1;
                pc = 0x80259110;
            }
            0x80259110 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80259070: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit068 (0x80259124, 180 bytes)
#[inline(never)]
pub fn f_80259124(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80259124) { return; }
        if c.prof { crate::prof::enter(c, 0x80259124); f_80259124_b(c, 0x80259124); crate::prof::exit(c, 0x80259124); return; }
    }
    f_80259124_b(c, 0x80259124)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80259124_at(c: &mut Cpu, pc: u32) { f_80259124_b(c, pc) }
#[inline(always)]
fn f_80259124_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80259124;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80259124, pc); }
        match pc {
            0x80259124 => {
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
                c.lr = 0x80259150; f_802395c8(c);
                pc = 0x80259150;
            }
            0x80259150 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025916c; f_8025c264(c);
                pc = 0x8025916c;
            }
            0x8025916c => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025917c; f_80239500(c);
                pc = 0x8025917c;
            }
            0x8025917c => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80259190; f_8023793c(c);
                pc = 0x80259190;
            }
            0x80259190 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x802591a0; continue; }
                pc = 0x8025919c;
            }
            0x8025919c => {
                c.r[30] = 0x0;
                pc = 0x802591a0;
            }
            0x802591a0 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x802591b0; continue; }
                pc = 0x802591a8;
            }
            0x802591a8 => {
                c.r[3] = 0x0;
                pc = 0x802591c4; continue;
                pc = 0x802591b0;
            }
            0x802591b0 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x802591c0; continue; }
                pc = 0x802591b8;
            }
            0x802591b8 => {
                c.r[3] = 0x1;
                pc = 0x802591c4; continue;
                pc = 0x802591c0;
            }
            0x802591c0 => {
                c.r[3] = 0x1;
                pc = 0x802591c4;
            }
            0x802591c4 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80259124: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit067 (0x802591d8, 608 bytes)
#[inline(never)]
pub fn f_802591d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802591d8) { return; }
        if c.prof { crate::prof::enter(c, 0x802591d8); f_802591d8_b(c, 0x802591d8); crate::prof::exit(c, 0x802591d8); return; }
    }
    f_802591d8_b(c, 0x802591d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802591d8_at(c: &mut Cpu, pc: u32) { f_802591d8_b(c, pc) }
#[inline(always)]
fn f_802591d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802591d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802591d8, pc); }
        match pc {
            0x802591d8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[3] = 0x2;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80259204; f_801f025c(c);
                pc = 0x80259204;
            }
            0x80259204 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[28] | c.r[28];
                c.lr = 0x8025921c; f_802395c8(c);
                pc = 0x8025921c;
            }
            0x8025921c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[26] = c.r[0] | c.r[0];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x7;
                c.lr = 0x80259234; f_80237f74(c);
                pc = 0x80259234;
            }
            0x80259234 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259248; continue; }
                pc = 0x80259240;
            }
            0x80259240 => {
                c.r[3] = 0x0;
                pc = 0x80259424; continue;
                pc = 0x80259248;
            }
            0x80259248 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x11;
                c.lr = 0x80259258; f_80237f74(c);
                pc = 0x80259258;
            }
            0x80259258 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259324; continue; }
                pc = 0x80259264;
            }
            0x80259264 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x80259274; f_80237f74(c);
                pc = 0x80259274;
            }
            0x80259274 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259324; continue; }
                pc = 0x80259280;
            }
            0x80259280 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x7;
                c.lr = 0x80259290; f_80237f74(c);
                pc = 0x80259290;
            }
            0x80259290 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802592a4; continue; }
                pc = 0x8025929c;
            }
            0x8025929c => {
                c.r[0] = 0x0;
                pc = 0x80259328; continue;
                pc = 0x802592a4;
            }
            0x802592a4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0xf;
                c.lr = 0x802592b4; f_80237f74(c);
                pc = 0x802592b4;
            }
            0x802592b4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259324; continue; }
                pc = 0x802592c0;
            }
            0x802592c0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x48;
                c.lr = 0x802592d0; f_80237f74(c);
                pc = 0x802592d0;
            }
            0x802592d0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259324; continue; }
                pc = 0x802592dc;
            }
            0x802592dc => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x29;
                c.lr = 0x802592ec; f_80237f74(c);
                pc = 0x802592ec;
            }
            0x802592ec => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259324; continue; }
                pc = 0x802592f8;
            }
            0x802592f8 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x28;
                c.lr = 0x80259308; f_80237f74(c);
                pc = 0x80259308;
            }
            0x80259308 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259324; continue; }
                pc = 0x80259314;
            }
            0x80259314 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0xc;
                c.lr = 0x80259324; f_80237f74(c);
                pc = 0x80259324;
            }
            0x80259324 => {
                c.r[0] = 0x1;
                pc = 0x80259328;
            }
            0x80259328 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80259338; continue; }
                pc = 0x80259330;
            }
            0x80259330 => {
                c.r[3] = 0x0;
                pc = 0x80259424; continue;
                pc = 0x80259338;
            }
            0x80259338 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x80259348; f_80236bfc(c);
                pc = 0x80259348;
            }
            0x80259348 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025935c; continue; }
                pc = 0x80259354;
            }
            0x80259354 => {
                c.r[3] = 0x0;
                pc = 0x80259424; continue;
                pc = 0x8025935c;
            }
            0x8025935c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80259368; f_80239500(c);
                pc = 0x80259368;
            }
            0x80259368 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[26] | c.r[26];
                c.lr = 0x8025937c; f_8023793c(c);
                pc = 0x8025937c;
            }
            0x8025937c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x80259390; continue; }
                pc = 0x80259388;
            }
            0x80259388 => {
                c.r[3] = 0x0;
                pc = 0x80259424; continue;
                pc = 0x80259390;
            }
            0x80259390 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x5;
                c.lr = 0x802593a0; f_80236bfc(c);
                pc = 0x802593a0;
            }
            0x802593a0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802593b4; continue; }
                pc = 0x802593ac;
            }
            0x802593ac => {
                c.r[3] = 0x0;
                pc = 0x80259424; continue;
                pc = 0x802593b4;
            }
            0x802593b4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x802593c0; f_80237310(c);
                pc = 0x802593c0;
            }
            0x802593c0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802593d0; continue; }
                pc = 0x802593c8;
            }
            0x802593c8 => {
                c.r[3] = 0x0;
                pc = 0x80259424; continue;
                pc = 0x802593d0;
            }
            0x802593d0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x802593e8; f_8025c264(c);
                pc = 0x802593e8;
            }
            0x802593e8 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x4b;
                c.lr = 0x802593f8; f_801f6e98(c);
                pc = 0x802593f8;
            }
            0x802593f8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025940c; continue; }
                pc = 0x80259404;
            }
            0x80259404 => {
                c.r[3] = 0x0;
                pc = 0x80259424; continue;
                pc = 0x8025940c;
            }
            0x8025940c => {
                c.cmps(0, c.r[27], 0x0);
                if !c.crbit(2) { pc = 0x8025941c; continue; }
                pc = 0x80259414;
            }
            0x80259414 => {
                c.r[3] = 0x0;
                pc = 0x80259424; continue;
                pc = 0x8025941c;
            }
            0x8025941c => {
                c.cmps(0, c.r[27], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80259424;
            }
            0x80259424 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802591d8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit066 (0x80259438, 624 bytes)
#[inline(never)]
pub fn f_80259438(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80259438) { return; }
        if c.prof { crate::prof::enter(c, 0x80259438); f_80259438_b(c, 0x80259438); crate::prof::exit(c, 0x80259438); return; }
    }
    f_80259438_b(c, 0x80259438)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80259438_at(c: &mut Cpu, pc: u32) { f_80259438_b(c, pc) }
#[inline(always)]
fn f_80259438_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80259438;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80259438, pc); }
        match pc {
            0x80259438 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[31] = c.r[6] | c.r[6];
                c.r[29] = c.r[4] | c.r[4];
                c.r[28] = c.r[3] | c.r[3];
                c.r[5] = 0x11;
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80259464; f_80237f74(c);
                pc = 0x80259464;
            }
            0x80259464 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259478; continue; }
                pc = 0x80259470;
            }
            0x80259470 => {
                c.r[3] = 0x0;
                pc = 0x80259694; continue;
                pc = 0x80259478;
            }
            0x80259478 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x14;
                c.lr = 0x80259488; f_80236bfc(c);
                pc = 0x80259488;
            }
            0x80259488 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025949c; continue; }
                pc = 0x80259494;
            }
            0x80259494 => {
                c.r[3] = 0x0;
                pc = 0x80259694; continue;
                pc = 0x8025949c;
            }
            0x8025949c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x3;
                c.lr = 0x802594ac; f_80236bfc(c);
                pc = 0x802594ac;
            }
            0x802594ac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802594c0; continue; }
                pc = 0x802594b8;
            }
            0x802594b8 => {
                c.r[3] = 0x0;
                pc = 0x80259694; continue;
                pc = 0x802594c0;
            }
            0x802594c0 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x4;
                c.lr = 0x802594d0; f_80236bfc(c);
                pc = 0x802594d0;
            }
            0x802594d0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802594e4; continue; }
                pc = 0x802594dc;
            }
            0x802594dc => {
                c.r[3] = 0x0;
                pc = 0x80259694; continue;
                pc = 0x802594e4;
            }
            0x802594e4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x3;
                c.lr = 0x802594f4; f_80237dbc(c);
                pc = 0x802594f4;
            }
            0x802594f4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259508; continue; }
                pc = 0x80259500;
            }
            0x80259500 => {
                c.r[3] = 0x0;
                pc = 0x80259694; continue;
                pc = 0x80259508;
            }
            0x80259508 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x8;
                c.lr = 0x80259518; f_80237dbc(c);
                pc = 0x80259518;
            }
            0x80259518 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025952c; continue; }
                pc = 0x80259524;
            }
            0x80259524 => {
                c.r[3] = 0x0;
                pc = 0x80259694; continue;
                pc = 0x8025952c;
            }
            0x8025952c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80259538; f_80237310(c);
                pc = 0x80259538;
            }
            0x80259538 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80259548; continue; }
                pc = 0x80259540;
            }
            0x80259540 => {
                c.r[3] = 0x0;
                pc = 0x80259694; continue;
                pc = 0x80259548;
            }
            0x80259548 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x11;
                c.lr = 0x80259558; f_80237f74(c);
                pc = 0x80259558;
            }
            0x80259558 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025956c; continue; }
                pc = 0x80259564;
            }
            0x80259564 => {
                c.r[0] = 0x0;
                pc = 0x80259628; continue;
                pc = 0x8025956c;
            }
            0x8025956c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x14;
                c.lr = 0x8025957c; f_80237f74(c);
                pc = 0x8025957c;
            }
            0x8025957c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259624; continue; }
                pc = 0x80259588;
            }
            0x80259588 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x7;
                c.lr = 0x80259598; f_80237f74(c);
                pc = 0x80259598;
            }
            0x80259598 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259624; continue; }
                pc = 0x802595a4;
            }
            0x802595a4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0xf;
                c.lr = 0x802595b4; f_80237f74(c);
                pc = 0x802595b4;
            }
            0x802595b4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259624; continue; }
                pc = 0x802595c0;
            }
            0x802595c0 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x48;
                c.lr = 0x802595d0; f_80237f74(c);
                pc = 0x802595d0;
            }
            0x802595d0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259624; continue; }
                pc = 0x802595dc;
            }
            0x802595dc => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x29;
                c.lr = 0x802595ec; f_80237f74(c);
                pc = 0x802595ec;
            }
            0x802595ec => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259624; continue; }
                pc = 0x802595f8;
            }
            0x802595f8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x28;
                c.lr = 0x80259608; f_80237f74(c);
                pc = 0x80259608;
            }
            0x80259608 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259624; continue; }
                pc = 0x80259614;
            }
            0x80259614 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0xc;
                c.lr = 0x80259624; f_80237f74(c);
                pc = 0x80259624;
            }
            0x80259624 => {
                c.r[0] = 0x1;
                pc = 0x80259628;
            }
            0x80259628 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80259638; continue; }
                pc = 0x80259630;
            }
            0x80259630 => {
                c.r[3] = 0x0;
                pc = 0x80259694; continue;
                pc = 0x80259638;
            }
            0x80259638 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[31] | c.r[31];
                c.r[7] = 0x0;
                c.lr = 0x80259650; f_8025c264(c);
                pc = 0x80259650;
            }
            0x80259650 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x80259660; f_801f025c(c);
                pc = 0x80259660;
            }
            0x80259660 => {
                c.r[4] = 0x4b;
                c.lr = 0x80259668; f_801f6e98(c);
                pc = 0x80259668;
            }
            0x80259668 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025967c; continue; }
                pc = 0x80259674;
            }
            0x80259674 => {
                c.r[3] = 0x0;
                pc = 0x80259694; continue;
                pc = 0x8025967c;
            }
            0x8025967c => {
                c.cmps(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x8025968c; continue; }
                pc = 0x80259684;
            }
            0x80259684 => {
                c.r[3] = 0x0;
                pc = 0x80259694; continue;
                pc = 0x8025968c;
            }
            0x8025968c => {
                c.cmps(0, c.r[28], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x80259694;
            }
            0x80259694 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80259438: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit065 (0x802596a8, 60 bytes)
#[inline(never)]
pub fn f_802596a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802596a8) { return; }
        if c.prof { crate::prof::enter(c, 0x802596a8); f_802596a8_b(c, 0x802596a8); crate::prof::exit(c, 0x802596a8); return; }
    }
    f_802596a8_b(c, 0x802596a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802596a8_at(c: &mut Cpu, pc: u32) { f_802596a8_b(c, pc) }
#[inline(always)]
fn f_802596a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802596a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802596a8, pc); }
        match pc {
            0x802596a8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x2;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x802596bc; f_801f025c(c);
                pc = 0x802596bc;
            }
            0x802596bc => {
                c.r[4] = 0x48;
                c.lr = 0x802596c4; f_801f6e44(c);
                pc = 0x802596c4;
            }
            0x802596c4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(!c.r[0], 0x2, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802596a8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit062 (0x802596e4, 232 bytes)
#[inline(never)]
pub fn f_802596e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802596e4) { return; }
        if c.prof { crate::prof::enter(c, 0x802596e4); f_802596e4_b(c, 0x802596e4); crate::prof::exit(c, 0x802596e4); return; }
    }
    f_802596e4_b(c, 0x802596e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802596e4_at(c: &mut Cpu, pc: u32) { f_802596e4_b(c, pc) }
#[inline(always)]
fn f_802596e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802596e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802596e4, pc); }
        match pc {
            0x802596e4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0xa0;
                c.lr = 0x8025970c; f_8021c034(c);
                pc = 0x8025970c;
            }
            0x8025970c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259754; continue; }
                pc = 0x80259714;
            }
            0x80259714 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x80259724; f_80236bfc(c);
                pc = 0x80259724;
            }
            0x80259724 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259738; continue; }
                pc = 0x80259730;
            }
            0x80259730 => {
                c.r[3] = 0x0;
                pc = 0x802597b8; continue;
                pc = 0x80259738;
            }
            0x80259738 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x80259750; f_8025c264(c);
                pc = 0x80259750;
            }
            0x80259750 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80259754;
            }
            0x80259754 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0xa0;
                c.r[8] = 0x5;
                c.r[9] = 0x1;
                c.lr = 0x80259774; f_8025c808(c);
                pc = 0x80259774;
            }
            0x80259774 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80259784; continue; }
                pc = 0x8025977c;
            }
            0x8025977c => {
                c.r[3] = 0x0;
                pc = 0x802597b8; continue;
                pc = 0x80259784;
            }
            0x80259784 => {
                c.r[3] = 0xa0;
                c.lr = 0x8025978c; f_8021c034(c);
                pc = 0x8025978c;
            }
            0x8025978c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x802597b4; continue; }
                pc = 0x80259794;
            }
            0x80259794 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x802597a4; continue; }
                pc = 0x8025979c;
            }
            0x8025979c => {
                c.r[3] = 0x0;
                pc = 0x802597b8; continue;
                pc = 0x802597a4;
            }
            0x802597a4 => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x802597b4; continue; }
                pc = 0x802597ac;
            }
            0x802597ac => {
                c.r[3] = 0x1;
                pc = 0x802597b8; continue;
                pc = 0x802597b4;
            }
            0x802597b4 => {
                c.r[3] = 0x1;
                pc = 0x802597b8;
            }
            0x802597b8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802596e4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit060 (0x802597cc, 232 bytes)
#[inline(never)]
pub fn f_802597cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802597cc) { return; }
        if c.prof { crate::prof::enter(c, 0x802597cc); f_802597cc_b(c, 0x802597cc); crate::prof::exit(c, 0x802597cc); return; }
    }
    f_802597cc_b(c, 0x802597cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802597cc_at(c: &mut Cpu, pc: u32) { f_802597cc_b(c, pc) }
#[inline(always)]
fn f_802597cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802597cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802597cc, pc); }
        match pc {
            0x802597cc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0xa0;
                c.lr = 0x802597f4; f_8021c034(c);
                pc = 0x802597f4;
            }
            0x802597f4 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025983c; continue; }
                pc = 0x802597fc;
            }
            0x802597fc => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x8025980c; f_80236bfc(c);
                pc = 0x8025980c;
            }
            0x8025980c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259820; continue; }
                pc = 0x80259818;
            }
            0x80259818 => {
                c.r[3] = 0x0;
                pc = 0x802598a0; continue;
                pc = 0x80259820;
            }
            0x80259820 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x80259838; f_8025c264(c);
                pc = 0x80259838;
            }
            0x80259838 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025983c;
            }
            0x8025983c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0xa0;
                c.r[8] = 0x3;
                c.r[9] = 0x1;
                c.lr = 0x8025985c; f_8025c808(c);
                pc = 0x8025985c;
            }
            0x8025985c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025986c; continue; }
                pc = 0x80259864;
            }
            0x80259864 => {
                c.r[3] = 0x0;
                pc = 0x802598a0; continue;
                pc = 0x8025986c;
            }
            0x8025986c => {
                c.r[3] = 0xa0;
                c.lr = 0x80259874; f_8021c034(c);
                pc = 0x80259874;
            }
            0x80259874 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025989c; continue; }
                pc = 0x8025987c;
            }
            0x8025987c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025988c; continue; }
                pc = 0x80259884;
            }
            0x80259884 => {
                c.r[3] = 0x0;
                pc = 0x802598a0; continue;
                pc = 0x8025988c;
            }
            0x8025988c => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025989c; continue; }
                pc = 0x80259894;
            }
            0x80259894 => {
                c.r[3] = 0x1;
                pc = 0x802598a0; continue;
                pc = 0x8025989c;
            }
            0x8025989c => {
                c.r[3] = 0x1;
                pc = 0x802598a0;
            }
            0x802598a0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802597cc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit059 (0x802598b4, 232 bytes)
#[inline(never)]
pub fn f_802598b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802598b4) { return; }
        if c.prof { crate::prof::enter(c, 0x802598b4); f_802598b4_b(c, 0x802598b4); crate::prof::exit(c, 0x802598b4); return; }
    }
    f_802598b4_b(c, 0x802598b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802598b4_at(c: &mut Cpu, pc: u32) { f_802598b4_b(c, pc) }
#[inline(always)]
fn f_802598b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802598b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802598b4, pc); }
        match pc {
            0x802598b4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0xa0;
                c.lr = 0x802598dc; f_8021c034(c);
                pc = 0x802598dc;
            }
            0x802598dc => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259924; continue; }
                pc = 0x802598e4;
            }
            0x802598e4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x802598f4; f_80236bfc(c);
                pc = 0x802598f4;
            }
            0x802598f4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259908; continue; }
                pc = 0x80259900;
            }
            0x80259900 => {
                c.r[3] = 0x0;
                pc = 0x80259988; continue;
                pc = 0x80259908;
            }
            0x80259908 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x80259920; f_8025c264(c);
                pc = 0x80259920;
            }
            0x80259920 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80259924;
            }
            0x80259924 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0xa0;
                c.r[8] = 0x2;
                c.r[9] = 0x1;
                c.lr = 0x80259944; f_8025c808(c);
                pc = 0x80259944;
            }
            0x80259944 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80259954; continue; }
                pc = 0x8025994c;
            }
            0x8025994c => {
                c.r[3] = 0x0;
                pc = 0x80259988; continue;
                pc = 0x80259954;
            }
            0x80259954 => {
                c.r[3] = 0xa0;
                c.lr = 0x8025995c; f_8021c034(c);
                pc = 0x8025995c;
            }
            0x8025995c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259984; continue; }
                pc = 0x80259964;
            }
            0x80259964 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80259974; continue; }
                pc = 0x8025996c;
            }
            0x8025996c => {
                c.r[3] = 0x0;
                pc = 0x80259988; continue;
                pc = 0x80259974;
            }
            0x80259974 => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x80259984; continue; }
                pc = 0x8025997c;
            }
            0x8025997c => {
                c.r[3] = 0x1;
                pc = 0x80259988; continue;
                pc = 0x80259984;
            }
            0x80259984 => {
                c.r[3] = 0x1;
                pc = 0x80259988;
            }
            0x80259988 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802598b4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit058 (0x8025999c, 232 bytes)
#[inline(never)]
pub fn f_8025999c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025999c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025999c); f_8025999c_b(c, 0x8025999c); crate::prof::exit(c, 0x8025999c); return; }
    }
    f_8025999c_b(c, 0x8025999c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025999c_at(c: &mut Cpu, pc: u32) { f_8025999c_b(c, pc) }
#[inline(always)]
fn f_8025999c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025999c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025999c, pc); }
        match pc {
            0x8025999c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0xa0;
                c.lr = 0x802599c4; f_8021c034(c);
                pc = 0x802599c4;
            }
            0x802599c4 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259a0c; continue; }
                pc = 0x802599cc;
            }
            0x802599cc => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x802599dc; f_80236bfc(c);
                pc = 0x802599dc;
            }
            0x802599dc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802599f0; continue; }
                pc = 0x802599e8;
            }
            0x802599e8 => {
                c.r[3] = 0x0;
                pc = 0x80259a70; continue;
                pc = 0x802599f0;
            }
            0x802599f0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x80259a08; f_8025c264(c);
                pc = 0x80259a08;
            }
            0x80259a08 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80259a0c;
            }
            0x80259a0c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0xa0;
                c.r[8] = 0x1;
                c.r[9] = 0x1;
                c.lr = 0x80259a2c; f_8025c808(c);
                pc = 0x80259a2c;
            }
            0x80259a2c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80259a3c; continue; }
                pc = 0x80259a34;
            }
            0x80259a34 => {
                c.r[3] = 0x0;
                pc = 0x80259a70; continue;
                pc = 0x80259a3c;
            }
            0x80259a3c => {
                c.r[3] = 0xa0;
                c.lr = 0x80259a44; f_8021c034(c);
                pc = 0x80259a44;
            }
            0x80259a44 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259a6c; continue; }
                pc = 0x80259a4c;
            }
            0x80259a4c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80259a5c; continue; }
                pc = 0x80259a54;
            }
            0x80259a54 => {
                c.r[3] = 0x0;
                pc = 0x80259a70; continue;
                pc = 0x80259a5c;
            }
            0x80259a5c => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x80259a6c; continue; }
                pc = 0x80259a64;
            }
            0x80259a64 => {
                c.r[3] = 0x1;
                pc = 0x80259a70; continue;
                pc = 0x80259a6c;
            }
            0x80259a6c => {
                c.r[3] = 0x1;
                pc = 0x80259a70;
            }
            0x80259a70 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025999c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit057 (0x80259a84, 104 bytes)
#[inline(never)]
pub fn f_80259a84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80259a84) { return; }
        if c.prof { crate::prof::enter(c, 0x80259a84); f_80259a84_b(c, 0x80259a84); crate::prof::exit(c, 0x80259a84); return; }
    }
    f_80259a84_b(c, 0x80259a84)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80259a84_at(c: &mut Cpu, pc: u32) { f_80259a84_b(c, pc) }
#[inline(always)]
fn f_80259a84_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80259a84;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80259a84, pc); }
        match pc {
            0x80259a84 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x10;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[6] | c.r[6];
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80259aa8; f_80236bfc(c);
                pc = 0x80259aa8;
            }
            0x80259aa8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80259acc; continue; }
                pc = 0x80259ab4;
            }
            0x80259ab4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80259ac0; f_80237288(c);
                pc = 0x80259ac0;
            }
            0x80259ac0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259ad4; continue; }
                pc = 0x80259acc;
            }
            0x80259acc => {
                c.r[3] = 0x0;
                pc = 0x80259ad8; continue;
                pc = 0x80259ad4;
            }
            0x80259ad4 => {
                c.r[3] = 0x1;
                pc = 0x80259ad8;
            }
            0x80259ad8 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80259a84: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit054 (0x80259aec, 232 bytes)
#[inline(never)]
pub fn f_80259aec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80259aec) { return; }
        if c.prof { crate::prof::enter(c, 0x80259aec); f_80259aec_b(c, 0x80259aec); crate::prof::exit(c, 0x80259aec); return; }
    }
    f_80259aec_b(c, 0x80259aec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80259aec_at(c: &mut Cpu, pc: u32) { f_80259aec_b(c, pc) }
#[inline(always)]
fn f_80259aec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80259aec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80259aec, pc); }
        match pc {
            0x80259aec => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x20;
                c.lr = 0x80259b14; f_8021c034(c);
                pc = 0x80259b14;
            }
            0x80259b14 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259b5c; continue; }
                pc = 0x80259b1c;
            }
            0x80259b1c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x80259b2c; f_80236bfc(c);
                pc = 0x80259b2c;
            }
            0x80259b2c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259b40; continue; }
                pc = 0x80259b38;
            }
            0x80259b38 => {
                c.r[3] = 0x0;
                pc = 0x80259bc0; continue;
                pc = 0x80259b40;
            }
            0x80259b40 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x80259b58; f_8025c264(c);
                pc = 0x80259b58;
            }
            0x80259b58 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80259b5c;
            }
            0x80259b5c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x20;
                c.r[8] = 0x5;
                c.r[9] = 0x41;
                c.lr = 0x80259b7c; f_8025c808(c);
                pc = 0x80259b7c;
            }
            0x80259b7c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80259b8c; continue; }
                pc = 0x80259b84;
            }
            0x80259b84 => {
                c.r[3] = 0x0;
                pc = 0x80259bc0; continue;
                pc = 0x80259b8c;
            }
            0x80259b8c => {
                c.r[3] = 0x20;
                c.lr = 0x80259b94; f_8021c034(c);
                pc = 0x80259b94;
            }
            0x80259b94 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259bbc; continue; }
                pc = 0x80259b9c;
            }
            0x80259b9c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80259bac; continue; }
                pc = 0x80259ba4;
            }
            0x80259ba4 => {
                c.r[3] = 0x0;
                pc = 0x80259bc0; continue;
                pc = 0x80259bac;
            }
            0x80259bac => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x80259bbc; continue; }
                pc = 0x80259bb4;
            }
            0x80259bb4 => {
                c.r[3] = 0x1;
                pc = 0x80259bc0; continue;
                pc = 0x80259bbc;
            }
            0x80259bbc => {
                c.r[3] = 0x1;
                pc = 0x80259bc0;
            }
            0x80259bc0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80259aec: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit053 (0x80259bd4, 232 bytes)
#[inline(never)]
pub fn f_80259bd4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80259bd4) { return; }
        if c.prof { crate::prof::enter(c, 0x80259bd4); f_80259bd4_b(c, 0x80259bd4); crate::prof::exit(c, 0x80259bd4); return; }
    }
    f_80259bd4_b(c, 0x80259bd4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80259bd4_at(c: &mut Cpu, pc: u32) { f_80259bd4_b(c, pc) }
#[inline(always)]
fn f_80259bd4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80259bd4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80259bd4, pc); }
        match pc {
            0x80259bd4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x20;
                c.lr = 0x80259bfc; f_8021c034(c);
                pc = 0x80259bfc;
            }
            0x80259bfc => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259c44; continue; }
                pc = 0x80259c04;
            }
            0x80259c04 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x80259c14; f_80236bfc(c);
                pc = 0x80259c14;
            }
            0x80259c14 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259c28; continue; }
                pc = 0x80259c20;
            }
            0x80259c20 => {
                c.r[3] = 0x0;
                pc = 0x80259ca8; continue;
                pc = 0x80259c28;
            }
            0x80259c28 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x80259c40; f_8025c264(c);
                pc = 0x80259c40;
            }
            0x80259c40 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80259c44;
            }
            0x80259c44 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x20;
                c.r[8] = 0x4;
                c.r[9] = 0x41;
                c.lr = 0x80259c64; f_8025c808(c);
                pc = 0x80259c64;
            }
            0x80259c64 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80259c74; continue; }
                pc = 0x80259c6c;
            }
            0x80259c6c => {
                c.r[3] = 0x0;
                pc = 0x80259ca8; continue;
                pc = 0x80259c74;
            }
            0x80259c74 => {
                c.r[3] = 0x20;
                c.lr = 0x80259c7c; f_8021c034(c);
                pc = 0x80259c7c;
            }
            0x80259c7c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259ca4; continue; }
                pc = 0x80259c84;
            }
            0x80259c84 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80259c94; continue; }
                pc = 0x80259c8c;
            }
            0x80259c8c => {
                c.r[3] = 0x0;
                pc = 0x80259ca8; continue;
                pc = 0x80259c94;
            }
            0x80259c94 => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x80259ca4; continue; }
                pc = 0x80259c9c;
            }
            0x80259c9c => {
                c.r[3] = 0x1;
                pc = 0x80259ca8; continue;
                pc = 0x80259ca4;
            }
            0x80259ca4 => {
                c.r[3] = 0x1;
                pc = 0x80259ca8;
            }
            0x80259ca8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80259bd4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit052 (0x80259cbc, 232 bytes)
#[inline(never)]
pub fn f_80259cbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80259cbc) { return; }
        if c.prof { crate::prof::enter(c, 0x80259cbc); f_80259cbc_b(c, 0x80259cbc); crate::prof::exit(c, 0x80259cbc); return; }
    }
    f_80259cbc_b(c, 0x80259cbc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80259cbc_at(c: &mut Cpu, pc: u32) { f_80259cbc_b(c, pc) }
#[inline(always)]
fn f_80259cbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80259cbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80259cbc, pc); }
        match pc {
            0x80259cbc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x20;
                c.lr = 0x80259ce4; f_8021c034(c);
                pc = 0x80259ce4;
            }
            0x80259ce4 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259d2c; continue; }
                pc = 0x80259cec;
            }
            0x80259cec => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x80259cfc; f_80236bfc(c);
                pc = 0x80259cfc;
            }
            0x80259cfc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259d10; continue; }
                pc = 0x80259d08;
            }
            0x80259d08 => {
                c.r[3] = 0x0;
                pc = 0x80259d90; continue;
                pc = 0x80259d10;
            }
            0x80259d10 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x80259d28; f_8025c264(c);
                pc = 0x80259d28;
            }
            0x80259d28 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80259d2c;
            }
            0x80259d2c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x20;
                c.r[8] = 0x3;
                c.r[9] = 0x41;
                c.lr = 0x80259d4c; f_8025c808(c);
                pc = 0x80259d4c;
            }
            0x80259d4c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80259d5c; continue; }
                pc = 0x80259d54;
            }
            0x80259d54 => {
                c.r[3] = 0x0;
                pc = 0x80259d90; continue;
                pc = 0x80259d5c;
            }
            0x80259d5c => {
                c.r[3] = 0x20;
                c.lr = 0x80259d64; f_8021c034(c);
                pc = 0x80259d64;
            }
            0x80259d64 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259d8c; continue; }
                pc = 0x80259d6c;
            }
            0x80259d6c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80259d7c; continue; }
                pc = 0x80259d74;
            }
            0x80259d74 => {
                c.r[3] = 0x0;
                pc = 0x80259d90; continue;
                pc = 0x80259d7c;
            }
            0x80259d7c => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x80259d8c; continue; }
                pc = 0x80259d84;
            }
            0x80259d84 => {
                c.r[3] = 0x1;
                pc = 0x80259d90; continue;
                pc = 0x80259d8c;
            }
            0x80259d8c => {
                c.r[3] = 0x1;
                pc = 0x80259d90;
            }
            0x80259d90 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80259cbc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit051 (0x80259da4, 232 bytes)
#[inline(never)]
pub fn f_80259da4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80259da4) { return; }
        if c.prof { crate::prof::enter(c, 0x80259da4); f_80259da4_b(c, 0x80259da4); crate::prof::exit(c, 0x80259da4); return; }
    }
    f_80259da4_b(c, 0x80259da4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80259da4_at(c: &mut Cpu, pc: u32) { f_80259da4_b(c, pc) }
#[inline(always)]
fn f_80259da4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80259da4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80259da4, pc); }
        match pc {
            0x80259da4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x20;
                c.lr = 0x80259dcc; f_8021c034(c);
                pc = 0x80259dcc;
            }
            0x80259dcc => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259e14; continue; }
                pc = 0x80259dd4;
            }
            0x80259dd4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x80259de4; f_80236bfc(c);
                pc = 0x80259de4;
            }
            0x80259de4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259df8; continue; }
                pc = 0x80259df0;
            }
            0x80259df0 => {
                c.r[3] = 0x0;
                pc = 0x80259e78; continue;
                pc = 0x80259df8;
            }
            0x80259df8 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x80259e10; f_8025c264(c);
                pc = 0x80259e10;
            }
            0x80259e10 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80259e14;
            }
            0x80259e14 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x20;
                c.r[8] = 0x2;
                c.r[9] = 0x41;
                c.lr = 0x80259e34; f_8025c808(c);
                pc = 0x80259e34;
            }
            0x80259e34 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80259e44; continue; }
                pc = 0x80259e3c;
            }
            0x80259e3c => {
                c.r[3] = 0x0;
                pc = 0x80259e78; continue;
                pc = 0x80259e44;
            }
            0x80259e44 => {
                c.r[3] = 0x20;
                c.lr = 0x80259e4c; f_8021c034(c);
                pc = 0x80259e4c;
            }
            0x80259e4c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259e74; continue; }
                pc = 0x80259e54;
            }
            0x80259e54 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80259e64; continue; }
                pc = 0x80259e5c;
            }
            0x80259e5c => {
                c.r[3] = 0x0;
                pc = 0x80259e78; continue;
                pc = 0x80259e64;
            }
            0x80259e64 => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x80259e74; continue; }
                pc = 0x80259e6c;
            }
            0x80259e6c => {
                c.r[3] = 0x1;
                pc = 0x80259e78; continue;
                pc = 0x80259e74;
            }
            0x80259e74 => {
                c.r[3] = 0x1;
                pc = 0x80259e78;
            }
            0x80259e78 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80259da4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit050 (0x80259e8c, 232 bytes)
#[inline(never)]
pub fn f_80259e8c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80259e8c) { return; }
        if c.prof { crate::prof::enter(c, 0x80259e8c); f_80259e8c_b(c, 0x80259e8c); crate::prof::exit(c, 0x80259e8c); return; }
    }
    f_80259e8c_b(c, 0x80259e8c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80259e8c_at(c: &mut Cpu, pc: u32) { f_80259e8c_b(c, pc) }
#[inline(always)]
fn f_80259e8c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80259e8c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80259e8c, pc); }
        match pc {
            0x80259e8c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x20;
                c.lr = 0x80259eb4; f_8021c034(c);
                pc = 0x80259eb4;
            }
            0x80259eb4 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259efc; continue; }
                pc = 0x80259ebc;
            }
            0x80259ebc => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x80259ecc; f_80236bfc(c);
                pc = 0x80259ecc;
            }
            0x80259ecc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259ee0; continue; }
                pc = 0x80259ed8;
            }
            0x80259ed8 => {
                c.r[3] = 0x0;
                pc = 0x80259f60; continue;
                pc = 0x80259ee0;
            }
            0x80259ee0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x80259ef8; f_8025c264(c);
                pc = 0x80259ef8;
            }
            0x80259ef8 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80259efc;
            }
            0x80259efc => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x20;
                c.r[8] = 0x1;
                c.r[9] = 0x41;
                c.lr = 0x80259f1c; f_8025c808(c);
                pc = 0x80259f1c;
            }
            0x80259f1c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80259f2c; continue; }
                pc = 0x80259f24;
            }
            0x80259f24 => {
                c.r[3] = 0x0;
                pc = 0x80259f60; continue;
                pc = 0x80259f2c;
            }
            0x80259f2c => {
                c.r[3] = 0x20;
                c.lr = 0x80259f34; f_8021c034(c);
                pc = 0x80259f34;
            }
            0x80259f34 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80259f5c; continue; }
                pc = 0x80259f3c;
            }
            0x80259f3c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80259f4c; continue; }
                pc = 0x80259f44;
            }
            0x80259f44 => {
                c.r[3] = 0x0;
                pc = 0x80259f60; continue;
                pc = 0x80259f4c;
            }
            0x80259f4c => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x80259f5c; continue; }
                pc = 0x80259f54;
            }
            0x80259f54 => {
                c.r[3] = 0x1;
                pc = 0x80259f60; continue;
                pc = 0x80259f5c;
            }
            0x80259f5c => {
                c.r[3] = 0x1;
                pc = 0x80259f60;
            }
            0x80259f60 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80259e8c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit049 (0x80259f74, 504 bytes)
#[inline(never)]
pub fn f_80259f74(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80259f74) { return; }
        if c.prof { crate::prof::enter(c, 0x80259f74); f_80259f74_b(c, 0x80259f74); crate::prof::exit(c, 0x80259f74); return; }
    }
    f_80259f74_b(c, 0x80259f74)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80259f74_at(c: &mut Cpu, pc: u32) { f_80259f74_b(c, pc) }
#[inline(always)]
fn f_80259f74_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80259f74;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80259f74, pc); }
        match pc {
            0x80259f74 => {
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
                c.lr = 0x80259fa0; f_801f025c(c);
                pc = 0x80259fa0;
            }
            0x80259fa0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x14;
                c.lr = 0x80259fb8; f_80237f74(c);
                pc = 0x80259fb8;
            }
            0x80259fb8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259fcc; continue; }
                pc = 0x80259fc4;
            }
            0x80259fc4 => {
                c.r[3] = 0x0;
                pc = 0x8025a158; continue;
                pc = 0x80259fcc;
            }
            0x80259fcc => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x14;
                c.lr = 0x80259fdc; f_80236bfc(c);
                pc = 0x80259fdc;
            }
            0x80259fdc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80259ff0; continue; }
                pc = 0x80259fe8;
            }
            0x80259fe8 => {
                c.r[3] = 0x0;
                pc = 0x8025a158; continue;
                pc = 0x80259ff0;
            }
            0x80259ff0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x9;
                c.lr = 0x8025a000; f_80236bfc(c);
                pc = 0x8025a000;
            }
            0x8025a000 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025a014; continue; }
                pc = 0x8025a00c;
            }
            0x8025a00c => {
                c.r[3] = 0x0;
                pc = 0x8025a158; continue;
                pc = 0x8025a014;
            }
            0x8025a014 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x11;
                c.lr = 0x8025a024; f_80237f74(c);
                pc = 0x8025a024;
            }
            0x8025a024 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025a0f0; continue; }
                pc = 0x8025a030;
            }
            0x8025a030 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x14;
                c.lr = 0x8025a040; f_80237f74(c);
                pc = 0x8025a040;
            }
            0x8025a040 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025a054; continue; }
                pc = 0x8025a04c;
            }
            0x8025a04c => {
                c.r[0] = 0x0;
                pc = 0x8025a0f4; continue;
                pc = 0x8025a054;
            }
            0x8025a054 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x7;
                c.lr = 0x8025a064; f_80237f74(c);
                pc = 0x8025a064;
            }
            0x8025a064 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025a0f0; continue; }
                pc = 0x8025a070;
            }
            0x8025a070 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0xf;
                c.lr = 0x8025a080; f_80237f74(c);
                pc = 0x8025a080;
            }
            0x8025a080 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025a0f0; continue; }
                pc = 0x8025a08c;
            }
            0x8025a08c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x48;
                c.lr = 0x8025a09c; f_80237f74(c);
                pc = 0x8025a09c;
            }
            0x8025a09c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025a0f0; continue; }
                pc = 0x8025a0a8;
            }
            0x8025a0a8 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x29;
                c.lr = 0x8025a0b8; f_80237f74(c);
                pc = 0x8025a0b8;
            }
            0x8025a0b8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025a0f0; continue; }
                pc = 0x8025a0c4;
            }
            0x8025a0c4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x28;
                c.lr = 0x8025a0d4; f_80237f74(c);
                pc = 0x8025a0d4;
            }
            0x8025a0d4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025a0f0; continue; }
                pc = 0x8025a0e0;
            }
            0x8025a0e0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0xc;
                c.lr = 0x8025a0f0; f_80237f74(c);
                pc = 0x8025a0f0;
            }
            0x8025a0f0 => {
                c.r[0] = 0x1;
                pc = 0x8025a0f4;
            }
            0x8025a0f4 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025a104; continue; }
                pc = 0x8025a0fc;
            }
            0x8025a0fc => {
                c.r[3] = 0x0;
                pc = 0x8025a158; continue;
                pc = 0x8025a104;
            }
            0x8025a104 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[31] | c.r[31];
                c.r[7] = 0x0;
                c.lr = 0x8025a11c; f_8025c264(c);
                pc = 0x8025a11c;
            }
            0x8025a11c => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x4b;
                c.lr = 0x8025a12c; f_801f6e98(c);
                pc = 0x8025a12c;
            }
            0x8025a12c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025a140; continue; }
                pc = 0x8025a138;
            }
            0x8025a138 => {
                c.r[3] = 0x0;
                pc = 0x8025a158; continue;
                pc = 0x8025a140;
            }
            0x8025a140 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025a150; continue; }
                pc = 0x8025a148;
            }
            0x8025a148 => {
                c.r[3] = 0x0;
                pc = 0x8025a158; continue;
                pc = 0x8025a150;
            }
            0x8025a150 => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025a158;
            }
            0x8025a158 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80259f74: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit048 (0x8025a16c, 180 bytes)
#[inline(never)]
pub fn f_8025a16c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a16c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a16c); f_8025a16c_b(c, 0x8025a16c); crate::prof::exit(c, 0x8025a16c); return; }
    }
    f_8025a16c_b(c, 0x8025a16c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a16c_at(c: &mut Cpu, pc: u32) { f_8025a16c_b(c, pc) }
#[inline(always)]
fn f_8025a16c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a16c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a16c, pc); }
        match pc {
            0x8025a16c => {
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
                c.lr = 0x8025a198; f_802395c8(c);
                pc = 0x8025a198;
            }
            0x8025a198 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025a1b4; f_8025c264(c);
                pc = 0x8025a1b4;
            }
            0x8025a1b4 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025a1c4; f_80239500(c);
                pc = 0x8025a1c4;
            }
            0x8025a1c4 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025a1d8; f_8023793c(c);
                pc = 0x8025a1d8;
            }
            0x8025a1d8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025a1e8; continue; }
                pc = 0x8025a1e4;
            }
            0x8025a1e4 => {
                c.r[30] = 0x0;
                pc = 0x8025a1e8;
            }
            0x8025a1e8 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025a1f8; continue; }
                pc = 0x8025a1f0;
            }
            0x8025a1f0 => {
                c.r[3] = 0x0;
                pc = 0x8025a20c; continue;
                pc = 0x8025a1f8;
            }
            0x8025a1f8 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025a208; continue; }
                pc = 0x8025a200;
            }
            0x8025a200 => {
                c.r[3] = 0x1;
                pc = 0x8025a20c; continue;
                pc = 0x8025a208;
            }
            0x8025a208 => {
                c.r[3] = 0x1;
                pc = 0x8025a20c;
            }
            0x8025a20c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a16c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit047 (0x8025a220, 52 bytes)
#[inline(never)]
pub fn f_8025a220(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a220) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a220); f_8025a220_b(c, 0x8025a220); crate::prof::exit(c, 0x8025a220); return; }
    }
    f_8025a220_b(c, 0x8025a220)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a220_at(c: &mut Cpu, pc: u32) { f_8025a220_b(c, pc) }
#[inline(always)]
fn f_8025a220_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a220;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a220, pc); }
        match pc {
            0x8025a220 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0xf;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025a234; f_80236bfc(c);
                pc = 0x8025a234;
            }
            0x8025a234 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a220: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit046 (0x8025a254, 60 bytes)
#[inline(never)]
pub fn f_8025a254(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a254) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a254); f_8025a254_b(c, 0x8025a254); crate::prof::exit(c, 0x8025a254); return; }
    }
    f_8025a254_b(c, 0x8025a254)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a254_at(c: &mut Cpu, pc: u32) { f_8025a254_b(c, pc) }
#[inline(always)]
fn f_8025a254_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a254;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a254, pc); }
        match pc {
            0x8025a254 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x2;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025a268; f_801f025c(c);
                pc = 0x8025a268;
            }
            0x8025a268 => {
                c.r[4] = 0x4c;
                c.lr = 0x8025a270; f_801f6e44(c);
                pc = 0x8025a270;
            }
            0x8025a270 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(!c.r[0], 0x2, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a254: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit045 (0x8025a290, 176 bytes)
#[inline(never)]
pub fn f_8025a290(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a290) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a290); f_8025a290_b(c, 0x8025a290); crate::prof::exit(c, 0x8025a290); return; }
    }
    f_8025a290_b(c, 0x8025a290)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a290_at(c: &mut Cpu, pc: u32) { f_8025a290_b(c, pc) }
#[inline(always)]
fn f_8025a290_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a290;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a290, pc); }
        match pc {
            0x8025a290 => {
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
                c.lr = 0x8025a2bc; f_802395c8(c);
                pc = 0x8025a2bc;
            }
            0x8025a2bc => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025a2dc; f_8025c264(c);
                pc = 0x8025a2dc;
            }
            0x8025a2dc => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025a2f0; f_80239500(c);
                pc = 0x8025a2f0;
            }
            0x8025a2f0 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8025a304; f_8023793c(c);
                pc = 0x8025a304;
            }
            0x8025a304 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025a314; continue; }
                pc = 0x8025a310;
            }
            0x8025a310 => {
                c.r[31] = 0x0;
                pc = 0x8025a314;
            }
            0x8025a314 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025a324; continue; }
                pc = 0x8025a31c;
            }
            0x8025a31c => {
                c.r[3] = 0x0;
                pc = 0x8025a32c; continue;
                pc = 0x8025a324;
            }
            0x8025a324 => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025a32c;
            }
            0x8025a32c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a290: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit044 (0x8025a340, 180 bytes)
#[inline(never)]
pub fn f_8025a340(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a340) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a340); f_8025a340_b(c, 0x8025a340); crate::prof::exit(c, 0x8025a340); return; }
    }
    f_8025a340_b(c, 0x8025a340)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a340_at(c: &mut Cpu, pc: u32) { f_8025a340_b(c, pc) }
#[inline(always)]
fn f_8025a340_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a340;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a340, pc); }
        match pc {
            0x8025a340 => {
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
                c.lr = 0x8025a36c; f_802395c8(c);
                pc = 0x8025a36c;
            }
            0x8025a36c => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025a388; f_8025c264(c);
                pc = 0x8025a388;
            }
            0x8025a388 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025a398; f_80239500(c);
                pc = 0x8025a398;
            }
            0x8025a398 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025a3ac; f_8023793c(c);
                pc = 0x8025a3ac;
            }
            0x8025a3ac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025a3bc; continue; }
                pc = 0x8025a3b8;
            }
            0x8025a3b8 => {
                c.r[30] = 0x0;
                pc = 0x8025a3bc;
            }
            0x8025a3bc => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025a3cc; continue; }
                pc = 0x8025a3c4;
            }
            0x8025a3c4 => {
                c.r[3] = 0x0;
                pc = 0x8025a3e0; continue;
                pc = 0x8025a3cc;
            }
            0x8025a3cc => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025a3dc; continue; }
                pc = 0x8025a3d4;
            }
            0x8025a3d4 => {
                c.r[3] = 0x1;
                pc = 0x8025a3e0; continue;
                pc = 0x8025a3dc;
            }
            0x8025a3dc => {
                c.r[3] = 0x1;
                pc = 0x8025a3e0;
            }
            0x8025a3e0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a340: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit043 (0x8025a3f4, 180 bytes)
#[inline(never)]
pub fn f_8025a3f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a3f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a3f4); f_8025a3f4_b(c, 0x8025a3f4); crate::prof::exit(c, 0x8025a3f4); return; }
    }
    f_8025a3f4_b(c, 0x8025a3f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a3f4_at(c: &mut Cpu, pc: u32) { f_8025a3f4_b(c, pc) }
#[inline(always)]
fn f_8025a3f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a3f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a3f4, pc); }
        match pc {
            0x8025a3f4 => {
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
                c.lr = 0x8025a420; f_802395c8(c);
                pc = 0x8025a420;
            }
            0x8025a420 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025a43c; f_8025c264(c);
                pc = 0x8025a43c;
            }
            0x8025a43c => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025a44c; f_80239500(c);
                pc = 0x8025a44c;
            }
            0x8025a44c => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025a460; f_8023793c(c);
                pc = 0x8025a460;
            }
            0x8025a460 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025a470; continue; }
                pc = 0x8025a46c;
            }
            0x8025a46c => {
                c.r[30] = 0x0;
                pc = 0x8025a470;
            }
            0x8025a470 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025a480; continue; }
                pc = 0x8025a478;
            }
            0x8025a478 => {
                c.r[3] = 0x0;
                pc = 0x8025a494; continue;
                pc = 0x8025a480;
            }
            0x8025a480 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025a490; continue; }
                pc = 0x8025a488;
            }
            0x8025a488 => {
                c.r[3] = 0x1;
                pc = 0x8025a494; continue;
                pc = 0x8025a490;
            }
            0x8025a490 => {
                c.r[3] = 0x1;
                pc = 0x8025a494;
            }
            0x8025a494 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a3f4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit042 (0x8025a4a8, 180 bytes)
#[inline(never)]
pub fn f_8025a4a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a4a8) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a4a8); f_8025a4a8_b(c, 0x8025a4a8); crate::prof::exit(c, 0x8025a4a8); return; }
    }
    f_8025a4a8_b(c, 0x8025a4a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a4a8_at(c: &mut Cpu, pc: u32) { f_8025a4a8_b(c, pc) }
#[inline(always)]
fn f_8025a4a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a4a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a4a8, pc); }
        match pc {
            0x8025a4a8 => {
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
                c.lr = 0x8025a4d4; f_802395c8(c);
                pc = 0x8025a4d4;
            }
            0x8025a4d4 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025a4f0; f_8025c264(c);
                pc = 0x8025a4f0;
            }
            0x8025a4f0 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025a500; f_80239500(c);
                pc = 0x8025a500;
            }
            0x8025a500 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025a514; f_8023793c(c);
                pc = 0x8025a514;
            }
            0x8025a514 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025a524; continue; }
                pc = 0x8025a520;
            }
            0x8025a520 => {
                c.r[30] = 0x0;
                pc = 0x8025a524;
            }
            0x8025a524 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025a534; continue; }
                pc = 0x8025a52c;
            }
            0x8025a52c => {
                c.r[3] = 0x0;
                pc = 0x8025a548; continue;
                pc = 0x8025a534;
            }
            0x8025a534 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025a544; continue; }
                pc = 0x8025a53c;
            }
            0x8025a53c => {
                c.r[3] = 0x1;
                pc = 0x8025a548; continue;
                pc = 0x8025a544;
            }
            0x8025a544 => {
                c.r[3] = 0x1;
                pc = 0x8025a548;
            }
            0x8025a548 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a4a8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit041 (0x8025a55c, 176 bytes)
#[inline(never)]
pub fn f_8025a55c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a55c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a55c); f_8025a55c_b(c, 0x8025a55c); crate::prof::exit(c, 0x8025a55c); return; }
    }
    f_8025a55c_b(c, 0x8025a55c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a55c_at(c: &mut Cpu, pc: u32) { f_8025a55c_b(c, pc) }
#[inline(always)]
fn f_8025a55c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a55c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a55c, pc); }
        match pc {
            0x8025a55c => {
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
                c.lr = 0x8025a588; f_802395c8(c);
                pc = 0x8025a588;
            }
            0x8025a588 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025a5a8; f_8025c264(c);
                pc = 0x8025a5a8;
            }
            0x8025a5a8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025a5bc; f_80239500(c);
                pc = 0x8025a5bc;
            }
            0x8025a5bc => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8025a5d0; f_8023793c(c);
                pc = 0x8025a5d0;
            }
            0x8025a5d0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025a5e0; continue; }
                pc = 0x8025a5dc;
            }
            0x8025a5dc => {
                c.r[31] = 0x0;
                pc = 0x8025a5e0;
            }
            0x8025a5e0 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025a5f0; continue; }
                pc = 0x8025a5e8;
            }
            0x8025a5e8 => {
                c.r[3] = 0x0;
                pc = 0x8025a5f8; continue;
                pc = 0x8025a5f0;
            }
            0x8025a5f0 => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025a5f8;
            }
            0x8025a5f8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a55c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit040 (0x8025a60c, 176 bytes)
#[inline(never)]
pub fn f_8025a60c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a60c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a60c); f_8025a60c_b(c, 0x8025a60c); crate::prof::exit(c, 0x8025a60c); return; }
    }
    f_8025a60c_b(c, 0x8025a60c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a60c_at(c: &mut Cpu, pc: u32) { f_8025a60c_b(c, pc) }
#[inline(always)]
fn f_8025a60c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a60c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a60c, pc); }
        match pc {
            0x8025a60c => {
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
                c.lr = 0x8025a638; f_802395c8(c);
                pc = 0x8025a638;
            }
            0x8025a638 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025a658; f_8025c264(c);
                pc = 0x8025a658;
            }
            0x8025a658 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025a66c; f_80239500(c);
                pc = 0x8025a66c;
            }
            0x8025a66c => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8025a680; f_8023793c(c);
                pc = 0x8025a680;
            }
            0x8025a680 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025a690; continue; }
                pc = 0x8025a68c;
            }
            0x8025a68c => {
                c.r[31] = 0x0;
                pc = 0x8025a690;
            }
            0x8025a690 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025a6a0; continue; }
                pc = 0x8025a698;
            }
            0x8025a698 => {
                c.r[3] = 0x0;
                pc = 0x8025a6a8; continue;
                pc = 0x8025a6a0;
            }
            0x8025a6a0 => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025a6a8;
            }
            0x8025a6a8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a60c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit039 (0x8025a6bc, 180 bytes)
#[inline(never)]
pub fn f_8025a6bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a6bc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a6bc); f_8025a6bc_b(c, 0x8025a6bc); crate::prof::exit(c, 0x8025a6bc); return; }
    }
    f_8025a6bc_b(c, 0x8025a6bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a6bc_at(c: &mut Cpu, pc: u32) { f_8025a6bc_b(c, pc) }
#[inline(always)]
fn f_8025a6bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a6bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a6bc, pc); }
        match pc {
            0x8025a6bc => {
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
                c.lr = 0x8025a6e8; f_802395c8(c);
                pc = 0x8025a6e8;
            }
            0x8025a6e8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025a704; f_8025c264(c);
                pc = 0x8025a704;
            }
            0x8025a704 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025a714; f_80239500(c);
                pc = 0x8025a714;
            }
            0x8025a714 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025a728; f_8023793c(c);
                pc = 0x8025a728;
            }
            0x8025a728 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025a738; continue; }
                pc = 0x8025a734;
            }
            0x8025a734 => {
                c.r[30] = 0x0;
                pc = 0x8025a738;
            }
            0x8025a738 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025a748; continue; }
                pc = 0x8025a740;
            }
            0x8025a740 => {
                c.r[3] = 0x0;
                pc = 0x8025a75c; continue;
                pc = 0x8025a748;
            }
            0x8025a748 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025a758; continue; }
                pc = 0x8025a750;
            }
            0x8025a750 => {
                c.r[3] = 0x1;
                pc = 0x8025a75c; continue;
                pc = 0x8025a758;
            }
            0x8025a758 => {
                c.r[3] = 0x1;
                pc = 0x8025a75c;
            }
            0x8025a75c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a6bc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit038 (0x8025a770, 256 bytes)
#[inline(never)]
pub fn f_8025a770(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a770) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a770); f_8025a770_b(c, 0x8025a770); crate::prof::exit(c, 0x8025a770); return; }
    }
    f_8025a770_b(c, 0x8025a770)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a770_at(c: &mut Cpu, pc: u32) { f_8025a770_b(c, pc) }
#[inline(always)]
fn f_8025a770_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a770;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a770, pc); }
        match pc {
            0x8025a770 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[26] = c.r[5] | c.r[5];
                c.r[27] = c.r[6] | c.r[6];
                c.lr = 0x8025a794; f_80237774(c);
                pc = 0x8025a794;
            }
            0x8025a794 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8025a7a4; f_80237774(c);
                pc = 0x8025a7a4;
            }
            0x8025a7a4 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x8025a7b8; f_802395c8(c);
                pc = 0x8025a7b8;
            }
            0x8025a7b8 => {
                c.r[7] = 0x10000;
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[26] | c.r[26];
                c.r[6] = c.r[27] | c.r[27];
                c.r[7] = c.r[7].wrapping_add(0xffffffff);
                c.lr = 0x8025a7d8; f_8025c264(c);
                pc = 0x8025a7d8;
            }
            0x8025a7d8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x8025a7ec; f_80239500(c);
                pc = 0x8025a7ec;
            }
            0x8025a7ec => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[28] | c.r[28];
                c.lr = 0x8025a800; f_8023793c(c);
                pc = 0x8025a800;
            }
            0x8025a800 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025a810; continue; }
                pc = 0x8025a80c;
            }
            0x8025a80c => {
                c.r[29] = 0x0;
                pc = 0x8025a810;
            }
            0x8025a810 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x5;
                c.lr = 0x8025a820; f_80237f74(c);
                pc = 0x8025a820;
            }
            0x8025a820 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025a830; continue; }
                pc = 0x8025a82c;
            }
            0x8025a82c => {
                c.r[29] = 0x0;
                pc = 0x8025a830;
            }
            0x8025a830 => {
                c.r[3] = c.r[30].rotate_left(0) & 0xff;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(0) { pc = 0x8025a844; continue; }
                pc = 0x8025a840;
            }
            0x8025a840 => {
                c.r[29] = 0x0;
                pc = 0x8025a844;
            }
            0x8025a844 => {
                c.cmps(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x8025a854; continue; }
                pc = 0x8025a84c;
            }
            0x8025a84c => {
                c.r[3] = 0x0;
                pc = 0x8025a85c; continue;
                pc = 0x8025a854;
            }
            0x8025a854 => {
                c.cmps(0, c.r[29], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025a85c;
            }
            0x8025a85c => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a770: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit037 (0x8025a870, 140 bytes)
#[inline(never)]
pub fn f_8025a870(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a870) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a870); f_8025a870_b(c, 0x8025a870); crate::prof::exit(c, 0x8025a870); return; }
    }
    f_8025a870_b(c, 0x8025a870)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a870_at(c: &mut Cpu, pc: u32) { f_8025a870_b(c, pc) }
#[inline(always)]
fn f_8025a870_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a870;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a870, pc); }
        match pc {
            0x8025a870 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x8;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x8025a890; f_80236bfc(c);
                pc = 0x8025a890;
            }
            0x8025a890 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025a8a4; continue; }
                pc = 0x8025a89c;
            }
            0x8025a89c => {
                c.r[3] = 0x0;
                pc = 0x8025a8e8; continue;
                pc = 0x8025a8a4;
            }
            0x8025a8a4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8025a8b0; f_8025cc90(c);
                pc = 0x8025a8b0;
            }
            0x8025a8b0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025a8c4; continue; }
                pc = 0x8025a8bc;
            }
            0x8025a8bc => {
                c.r[3] = 0x0;
                pc = 0x8025a8e8; continue;
                pc = 0x8025a8c4;
            }
            0x8025a8c4 => {
                { let ea = c.r[2].wrapping_add(0xffffafa8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x0;
                c.lr = 0x8025a8d8; f_802373b0(c);
                pc = 0x8025a8d8;
            }
            0x8025a8d8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                pc = 0x8025a8e8;
            }
            0x8025a8e8 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a870: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit036 (0x8025a8fc, 180 bytes)
#[inline(never)]
pub fn f_8025a8fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a8fc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a8fc); f_8025a8fc_b(c, 0x8025a8fc); crate::prof::exit(c, 0x8025a8fc); return; }
    }
    f_8025a8fc_b(c, 0x8025a8fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a8fc_at(c: &mut Cpu, pc: u32) { f_8025a8fc_b(c, pc) }
#[inline(always)]
fn f_8025a8fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a8fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a8fc, pc); }
        match pc {
            0x8025a8fc => {
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
                c.lr = 0x8025a928; f_802395c8(c);
                pc = 0x8025a928;
            }
            0x8025a928 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025a944; f_8025c264(c);
                pc = 0x8025a944;
            }
            0x8025a944 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025a954; f_80239500(c);
                pc = 0x8025a954;
            }
            0x8025a954 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025a968; f_8023793c(c);
                pc = 0x8025a968;
            }
            0x8025a968 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025a978; continue; }
                pc = 0x8025a974;
            }
            0x8025a974 => {
                c.r[30] = 0x0;
                pc = 0x8025a978;
            }
            0x8025a978 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025a988; continue; }
                pc = 0x8025a980;
            }
            0x8025a980 => {
                c.r[3] = 0x0;
                pc = 0x8025a99c; continue;
                pc = 0x8025a988;
            }
            0x8025a988 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025a998; continue; }
                pc = 0x8025a990;
            }
            0x8025a990 => {
                c.r[3] = 0x1;
                pc = 0x8025a99c; continue;
                pc = 0x8025a998;
            }
            0x8025a998 => {
                c.r[3] = 0x1;
                pc = 0x8025a99c;
            }
            0x8025a99c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a8fc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit035 (0x8025a9b0, 60 bytes)
#[inline(never)]
pub fn f_8025a9b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a9b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a9b0); f_8025a9b0_b(c, 0x8025a9b0); crate::prof::exit(c, 0x8025a9b0); return; }
    }
    f_8025a9b0_b(c, 0x8025a9b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a9b0_at(c: &mut Cpu, pc: u32) { f_8025a9b0_b(c, pc) }
#[inline(always)]
fn f_8025a9b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a9b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a9b0, pc); }
        match pc {
            0x8025a9b0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x2;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025a9c4; f_801f025c(c);
                pc = 0x8025a9c4;
            }
            0x8025a9c4 => {
                c.r[4] = 0x49;
                c.lr = 0x8025a9cc; f_801f6e44(c);
                pc = 0x8025a9cc;
            }
            0x8025a9cc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(!c.r[0], 0x2, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a9b0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit034 (0x8025a9ec, 180 bytes)
#[inline(never)]
pub fn f_8025a9ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025a9ec) { return; }
        if c.prof { crate::prof::enter(c, 0x8025a9ec); f_8025a9ec_b(c, 0x8025a9ec); crate::prof::exit(c, 0x8025a9ec); return; }
    }
    f_8025a9ec_b(c, 0x8025a9ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025a9ec_at(c: &mut Cpu, pc: u32) { f_8025a9ec_b(c, pc) }
#[inline(always)]
fn f_8025a9ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025a9ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025a9ec, pc); }
        match pc {
            0x8025a9ec => {
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
                c.lr = 0x8025aa18; f_802395c8(c);
                pc = 0x8025aa18;
            }
            0x8025aa18 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025aa34; f_8025c264(c);
                pc = 0x8025aa34;
            }
            0x8025aa34 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025aa44; f_80239500(c);
                pc = 0x8025aa44;
            }
            0x8025aa44 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025aa58; f_8023793c(c);
                pc = 0x8025aa58;
            }
            0x8025aa58 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025aa68; continue; }
                pc = 0x8025aa64;
            }
            0x8025aa64 => {
                c.r[30] = 0x0;
                pc = 0x8025aa68;
            }
            0x8025aa68 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025aa78; continue; }
                pc = 0x8025aa70;
            }
            0x8025aa70 => {
                c.r[3] = 0x0;
                pc = 0x8025aa8c; continue;
                pc = 0x8025aa78;
            }
            0x8025aa78 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025aa88; continue; }
                pc = 0x8025aa80;
            }
            0x8025aa80 => {
                c.r[3] = 0x1;
                pc = 0x8025aa8c; continue;
                pc = 0x8025aa88;
            }
            0x8025aa88 => {
                c.r[3] = 0x1;
                pc = 0x8025aa8c;
            }
            0x8025aa8c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025a9ec: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit033 (0x8025aaa0, 624 bytes)
#[inline(never)]
pub fn f_8025aaa0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025aaa0) { return; }
        if c.prof { crate::prof::enter(c, 0x8025aaa0); f_8025aaa0_b(c, 0x8025aaa0); crate::prof::exit(c, 0x8025aaa0); return; }
    }
    f_8025aaa0_b(c, 0x8025aaa0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025aaa0_at(c: &mut Cpu, pc: u32) { f_8025aaa0_b(c, pc) }
#[inline(always)]
fn f_8025aaa0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025aaa0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025aaa0, pc); }
        match pc {
            0x8025aaa0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[31] = c.r[6] | c.r[6];
                c.r[29] = c.r[4] | c.r[4];
                c.r[28] = c.r[3] | c.r[3];
                c.r[5] = 0x11;
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8025aacc; f_80237f74(c);
                pc = 0x8025aacc;
            }
            0x8025aacc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025aae0; continue; }
                pc = 0x8025aad8;
            }
            0x8025aad8 => {
                c.r[3] = 0x0;
                pc = 0x8025acfc; continue;
                pc = 0x8025aae0;
            }
            0x8025aae0 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x14;
                c.lr = 0x8025aaf0; f_80236bfc(c);
                pc = 0x8025aaf0;
            }
            0x8025aaf0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025ab04; continue; }
                pc = 0x8025aafc;
            }
            0x8025aafc => {
                c.r[3] = 0x0;
                pc = 0x8025acfc; continue;
                pc = 0x8025ab04;
            }
            0x8025ab04 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x3;
                c.lr = 0x8025ab14; f_80236bfc(c);
                pc = 0x8025ab14;
            }
            0x8025ab14 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025ab28; continue; }
                pc = 0x8025ab20;
            }
            0x8025ab20 => {
                c.r[3] = 0x0;
                pc = 0x8025acfc; continue;
                pc = 0x8025ab28;
            }
            0x8025ab28 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x4;
                c.lr = 0x8025ab38; f_80236bfc(c);
                pc = 0x8025ab38;
            }
            0x8025ab38 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025ab4c; continue; }
                pc = 0x8025ab44;
            }
            0x8025ab44 => {
                c.r[3] = 0x0;
                pc = 0x8025acfc; continue;
                pc = 0x8025ab4c;
            }
            0x8025ab4c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8025ab58; f_80237310(c);
                pc = 0x8025ab58;
            }
            0x8025ab58 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025ab68; continue; }
                pc = 0x8025ab60;
            }
            0x8025ab60 => {
                c.r[3] = 0x0;
                pc = 0x8025acfc; continue;
                pc = 0x8025ab68;
            }
            0x8025ab68 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x3;
                c.lr = 0x8025ab78; f_80237dbc(c);
                pc = 0x8025ab78;
            }
            0x8025ab78 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025ab8c; continue; }
                pc = 0x8025ab84;
            }
            0x8025ab84 => {
                c.r[3] = 0x0;
                pc = 0x8025acfc; continue;
                pc = 0x8025ab8c;
            }
            0x8025ab8c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x8;
                c.lr = 0x8025ab9c; f_80237dbc(c);
                pc = 0x8025ab9c;
            }
            0x8025ab9c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025abb0; continue; }
                pc = 0x8025aba8;
            }
            0x8025aba8 => {
                c.r[3] = 0x0;
                pc = 0x8025acfc; continue;
                pc = 0x8025abb0;
            }
            0x8025abb0 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x11;
                c.lr = 0x8025abc0; f_80237f74(c);
                pc = 0x8025abc0;
            }
            0x8025abc0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025abd4; continue; }
                pc = 0x8025abcc;
            }
            0x8025abcc => {
                c.r[0] = 0x0;
                pc = 0x8025ac90; continue;
                pc = 0x8025abd4;
            }
            0x8025abd4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x14;
                c.lr = 0x8025abe4; f_80237f74(c);
                pc = 0x8025abe4;
            }
            0x8025abe4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025ac8c; continue; }
                pc = 0x8025abf0;
            }
            0x8025abf0 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x7;
                c.lr = 0x8025ac00; f_80237f74(c);
                pc = 0x8025ac00;
            }
            0x8025ac00 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025ac8c; continue; }
                pc = 0x8025ac0c;
            }
            0x8025ac0c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0xf;
                c.lr = 0x8025ac1c; f_80237f74(c);
                pc = 0x8025ac1c;
            }
            0x8025ac1c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025ac8c; continue; }
                pc = 0x8025ac28;
            }
            0x8025ac28 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x48;
                c.lr = 0x8025ac38; f_80237f74(c);
                pc = 0x8025ac38;
            }
            0x8025ac38 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025ac8c; continue; }
                pc = 0x8025ac44;
            }
            0x8025ac44 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x29;
                c.lr = 0x8025ac54; f_80237f74(c);
                pc = 0x8025ac54;
            }
            0x8025ac54 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025ac8c; continue; }
                pc = 0x8025ac60;
            }
            0x8025ac60 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x28;
                c.lr = 0x8025ac70; f_80237f74(c);
                pc = 0x8025ac70;
            }
            0x8025ac70 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025ac8c; continue; }
                pc = 0x8025ac7c;
            }
            0x8025ac7c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0xc;
                c.lr = 0x8025ac8c; f_80237f74(c);
                pc = 0x8025ac8c;
            }
            0x8025ac8c => {
                c.r[0] = 0x1;
                pc = 0x8025ac90;
            }
            0x8025ac90 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025aca0; continue; }
                pc = 0x8025ac98;
            }
            0x8025ac98 => {
                c.r[3] = 0x0;
                pc = 0x8025acfc; continue;
                pc = 0x8025aca0;
            }
            0x8025aca0 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[31] | c.r[31];
                c.r[7] = 0x0;
                c.lr = 0x8025acb8; f_8025c264(c);
                pc = 0x8025acb8;
            }
            0x8025acb8 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x8025acc8; f_801f025c(c);
                pc = 0x8025acc8;
            }
            0x8025acc8 => {
                c.r[4] = 0x4b;
                c.lr = 0x8025acd0; f_801f6e98(c);
                pc = 0x8025acd0;
            }
            0x8025acd0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025ace4; continue; }
                pc = 0x8025acdc;
            }
            0x8025acdc => {
                c.r[3] = 0x0;
                pc = 0x8025acfc; continue;
                pc = 0x8025ace4;
            }
            0x8025ace4 => {
                c.cmps(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x8025acf4; continue; }
                pc = 0x8025acec;
            }
            0x8025acec => {
                c.r[3] = 0x0;
                pc = 0x8025acfc; continue;
                pc = 0x8025acf4;
            }
            0x8025acf4 => {
                c.cmps(0, c.r[28], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025acfc;
            }
            0x8025acfc => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025aaa0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit032 (0x8025ad10, 56 bytes)
#[inline(never)]
pub fn f_8025ad10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025ad10) { return; }
        if c.prof { crate::prof::enter(c, 0x8025ad10); f_8025ad10_b(c, 0x8025ad10); crate::prof::exit(c, 0x8025ad10); return; }
    }
    f_8025ad10_b(c, 0x8025ad10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025ad10_at(c: &mut Cpu, pc: u32) { f_8025ad10_b(c, pc) }
#[inline(always)]
fn f_8025ad10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025ad10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025ad10, pc); }
        match pc {
            0x8025ad10 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffffafa8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025ad28; f_802373b0(c);
                pc = 0x8025ad28;
            }
            0x8025ad28 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025ad10: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit031 (0x8025ad48, 180 bytes)
#[inline(never)]
pub fn f_8025ad48(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025ad48) { return; }
        if c.prof { crate::prof::enter(c, 0x8025ad48); f_8025ad48_b(c, 0x8025ad48); crate::prof::exit(c, 0x8025ad48); return; }
    }
    f_8025ad48_b(c, 0x8025ad48)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025ad48_at(c: &mut Cpu, pc: u32) { f_8025ad48_b(c, pc) }
#[inline(always)]
fn f_8025ad48_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025ad48;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025ad48, pc); }
        match pc {
            0x8025ad48 => {
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
                c.lr = 0x8025ad74; f_802395c8(c);
                pc = 0x8025ad74;
            }
            0x8025ad74 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025ad90; f_8025c264(c);
                pc = 0x8025ad90;
            }
            0x8025ad90 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025ada0; f_80239500(c);
                pc = 0x8025ada0;
            }
            0x8025ada0 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025adb4; f_8023793c(c);
                pc = 0x8025adb4;
            }
            0x8025adb4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025adc4; continue; }
                pc = 0x8025adc0;
            }
            0x8025adc0 => {
                c.r[30] = 0x0;
                pc = 0x8025adc4;
            }
            0x8025adc4 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025add4; continue; }
                pc = 0x8025adcc;
            }
            0x8025adcc => {
                c.r[3] = 0x0;
                pc = 0x8025ade8; continue;
                pc = 0x8025add4;
            }
            0x8025add4 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025ade4; continue; }
                pc = 0x8025addc;
            }
            0x8025addc => {
                c.r[3] = 0x1;
                pc = 0x8025ade8; continue;
                pc = 0x8025ade4;
            }
            0x8025ade4 => {
                c.r[3] = 0x1;
                pc = 0x8025ade8;
            }
            0x8025ade8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025ad48: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit030 (0x8025adfc, 44 bytes)
#[inline(never)]
pub fn f_8025adfc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025adfc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025adfc); f_8025adfc_b(c, 0x8025adfc); crate::prof::exit(c, 0x8025adfc); return; }
    }
    f_8025adfc_b(c, 0x8025adfc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025adfc_at(c: &mut Cpu, pc: u32) { f_8025adfc_b(c, pc) }
#[inline(always)]
fn f_8025adfc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025adfc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025adfc, pc); }
        match pc {
            0x8025adfc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025ae0c; f_8025c770(c);
                pc = 0x8025ae0c;
            }
            0x8025ae0c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025adfc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit029 (0x8025ae28, 176 bytes)
#[inline(never)]
pub fn f_8025ae28(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025ae28) { return; }
        if c.prof { crate::prof::enter(c, 0x8025ae28); f_8025ae28_b(c, 0x8025ae28); crate::prof::exit(c, 0x8025ae28); return; }
    }
    f_8025ae28_b(c, 0x8025ae28)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025ae28_at(c: &mut Cpu, pc: u32) { f_8025ae28_b(c, pc) }
#[inline(always)]
fn f_8025ae28_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025ae28;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025ae28, pc); }
        match pc {
            0x8025ae28 => {
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
                c.lr = 0x8025ae54; f_802395c8(c);
                pc = 0x8025ae54;
            }
            0x8025ae54 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025ae74; f_8025c264(c);
                pc = 0x8025ae74;
            }
            0x8025ae74 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025ae88; f_80239500(c);
                pc = 0x8025ae88;
            }
            0x8025ae88 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8025ae9c; f_8023793c(c);
                pc = 0x8025ae9c;
            }
            0x8025ae9c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025aeac; continue; }
                pc = 0x8025aea8;
            }
            0x8025aea8 => {
                c.r[31] = 0x0;
                pc = 0x8025aeac;
            }
            0x8025aeac => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025aebc; continue; }
                pc = 0x8025aeb4;
            }
            0x8025aeb4 => {
                c.r[3] = 0x0;
                pc = 0x8025aec4; continue;
                pc = 0x8025aebc;
            }
            0x8025aebc => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025aec4;
            }
            0x8025aec4 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025ae28: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit028 (0x8025aed8, 224 bytes)
#[inline(never)]
pub fn f_8025aed8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025aed8) { return; }
        if c.prof { crate::prof::enter(c, 0x8025aed8); f_8025aed8_b(c, 0x8025aed8); crate::prof::exit(c, 0x8025aed8); return; }
    }
    f_8025aed8_b(c, 0x8025aed8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025aed8_at(c: &mut Cpu, pc: u32) { f_8025aed8_b(c, pc) }
#[inline(always)]
fn f_8025aed8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025aed8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025aed8, pc); }
        match pc {
            0x8025aed8 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[30] = c.r[6] | c.r[6];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[3] = 0x0;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8025af04; f_801f4354(c);
                pc = 0x8025af04;
            }
            0x8025af04 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x15;
                c.lr = 0x8025af1c; f_80237f74(c);
                pc = 0x8025af1c;
            }
            0x8025af1c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025af30; continue; }
                pc = 0x8025af28;
            }
            0x8025af28 => {
                c.r[3] = 0x0;
                pc = 0x8025afa4; continue;
                pc = 0x8025af30;
            }
            0x8025af30 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x25;
                c.lr = 0x8025af40; f_80236bfc(c);
                pc = 0x8025af40;
            }
            0x8025af40 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025af54; continue; }
                pc = 0x8025af4c;
            }
            0x8025af4c => {
                c.r[3] = 0x0;
                pc = 0x8025afa4; continue;
                pc = 0x8025af54;
            }
            0x8025af54 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x8025af6c; f_8025c264(c);
                pc = 0x8025af6c;
            }
            0x8025af6c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8025af80; f_801f87cc(c);
                pc = 0x8025af80;
            }
            0x8025af80 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025af8c; continue; }
                pc = 0x8025af88;
            }
            0x8025af88 => {
                c.r[31] = 0x0;
                pc = 0x8025af8c;
            }
            0x8025af8c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025af9c; continue; }
                pc = 0x8025af94;
            }
            0x8025af94 => {
                c.r[3] = 0x0;
                pc = 0x8025afa4; continue;
                pc = 0x8025af9c;
            }
            0x8025af9c => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025afa4;
            }
            0x8025afa4 => {
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_8025aed8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit027 (0x8025afb8, 176 bytes)
#[inline(never)]
pub fn f_8025afb8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025afb8) { return; }
        if c.prof { crate::prof::enter(c, 0x8025afb8); f_8025afb8_b(c, 0x8025afb8); crate::prof::exit(c, 0x8025afb8); return; }
    }
    f_8025afb8_b(c, 0x8025afb8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025afb8_at(c: &mut Cpu, pc: u32) { f_8025afb8_b(c, pc) }
#[inline(always)]
fn f_8025afb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025afb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025afb8, pc); }
        match pc {
            0x8025afb8 => {
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
                c.lr = 0x8025afe4; f_802395c8(c);
                pc = 0x8025afe4;
            }
            0x8025afe4 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025b004; f_8025c264(c);
                pc = 0x8025b004;
            }
            0x8025b004 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025b018; f_80239500(c);
                pc = 0x8025b018;
            }
            0x8025b018 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8025b02c; f_8023793c(c);
                pc = 0x8025b02c;
            }
            0x8025b02c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025b03c; continue; }
                pc = 0x8025b038;
            }
            0x8025b038 => {
                c.r[31] = 0x0;
                pc = 0x8025b03c;
            }
            0x8025b03c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025b04c; continue; }
                pc = 0x8025b044;
            }
            0x8025b044 => {
                c.r[3] = 0x0;
                pc = 0x8025b054; continue;
                pc = 0x8025b04c;
            }
            0x8025b04c => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025b054;
            }
            0x8025b054 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025afb8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit026 (0x8025b068, 180 bytes)
#[inline(never)]
pub fn f_8025b068(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025b068) { return; }
        if c.prof { crate::prof::enter(c, 0x8025b068); f_8025b068_b(c, 0x8025b068); crate::prof::exit(c, 0x8025b068); return; }
    }
    f_8025b068_b(c, 0x8025b068)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025b068_at(c: &mut Cpu, pc: u32) { f_8025b068_b(c, pc) }
#[inline(always)]
fn f_8025b068_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025b068;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025b068, pc); }
        match pc {
            0x8025b068 => {
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
                c.lr = 0x8025b094; f_802395c8(c);
                pc = 0x8025b094;
            }
            0x8025b094 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025b0b0; f_8025c264(c);
                pc = 0x8025b0b0;
            }
            0x8025b0b0 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025b0c0; f_80239500(c);
                pc = 0x8025b0c0;
            }
            0x8025b0c0 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025b0d4; f_8023793c(c);
                pc = 0x8025b0d4;
            }
            0x8025b0d4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025b0e4; continue; }
                pc = 0x8025b0e0;
            }
            0x8025b0e0 => {
                c.r[30] = 0x0;
                pc = 0x8025b0e4;
            }
            0x8025b0e4 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025b0f4; continue; }
                pc = 0x8025b0ec;
            }
            0x8025b0ec => {
                c.r[3] = 0x0;
                pc = 0x8025b108; continue;
                pc = 0x8025b0f4;
            }
            0x8025b0f4 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025b104; continue; }
                pc = 0x8025b0fc;
            }
            0x8025b0fc => {
                c.r[3] = 0x1;
                pc = 0x8025b108; continue;
                pc = 0x8025b104;
            }
            0x8025b104 => {
                c.r[3] = 0x1;
                pc = 0x8025b108;
            }
            0x8025b108 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025b068: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit025 (0x8025b11c, 8 bytes)
#[inline(never)]
pub fn f_8025b11c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025b11c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025b11c); f_8025b11c_b(c, 0x8025b11c); crate::prof::exit(c, 0x8025b11c); return; }
    }
    f_8025b11c_b(c, 0x8025b11c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025b11c_at(c: &mut Cpu, pc: u32) { f_8025b11c_b(c, pc) }
#[inline(always)]
fn f_8025b11c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025b11c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025b11c, pc); }
        match pc {
            0x8025b11c => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8025b11c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit024 (0x8025b124, 232 bytes)
#[inline(never)]
pub fn f_8025b124(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025b124) { return; }
        if c.prof { crate::prof::enter(c, 0x8025b124); f_8025b124_b(c, 0x8025b124); crate::prof::exit(c, 0x8025b124); return; }
    }
    f_8025b124_b(c, 0x8025b124)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025b124_at(c: &mut Cpu, pc: u32) { f_8025b124_b(c, pc) }
#[inline(always)]
fn f_8025b124_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025b124;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025b124, pc); }
        match pc {
            0x8025b124 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x90;
                c.lr = 0x8025b14c; f_8021c034(c);
                pc = 0x8025b14c;
            }
            0x8025b14c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b194; continue; }
                pc = 0x8025b154;
            }
            0x8025b154 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x8025b164; f_80236bfc(c);
                pc = 0x8025b164;
            }
            0x8025b164 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025b178; continue; }
                pc = 0x8025b170;
            }
            0x8025b170 => {
                c.r[3] = 0x0;
                pc = 0x8025b1f8; continue;
                pc = 0x8025b178;
            }
            0x8025b178 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x8025b190; f_8025c264(c);
                pc = 0x8025b190;
            }
            0x8025b190 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025b194;
            }
            0x8025b194 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x90;
                c.r[8] = 0x7;
                c.r[9] = 0x1;
                c.lr = 0x8025b1b4; f_8025c808(c);
                pc = 0x8025b1b4;
            }
            0x8025b1b4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025b1c4; continue; }
                pc = 0x8025b1bc;
            }
            0x8025b1bc => {
                c.r[3] = 0x0;
                pc = 0x8025b1f8; continue;
                pc = 0x8025b1c4;
            }
            0x8025b1c4 => {
                c.r[3] = 0x90;
                c.lr = 0x8025b1cc; f_8021c034(c);
                pc = 0x8025b1cc;
            }
            0x8025b1cc => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b1f4; continue; }
                pc = 0x8025b1d4;
            }
            0x8025b1d4 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025b1e4; continue; }
                pc = 0x8025b1dc;
            }
            0x8025b1dc => {
                c.r[3] = 0x0;
                pc = 0x8025b1f8; continue;
                pc = 0x8025b1e4;
            }
            0x8025b1e4 => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025b1f4; continue; }
                pc = 0x8025b1ec;
            }
            0x8025b1ec => {
                c.r[3] = 0x1;
                pc = 0x8025b1f8; continue;
                pc = 0x8025b1f4;
            }
            0x8025b1f4 => {
                c.r[3] = 0x1;
                pc = 0x8025b1f8;
            }
            0x8025b1f8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025b124: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit023 (0x8025b20c, 232 bytes)
#[inline(never)]
pub fn f_8025b20c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025b20c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025b20c); f_8025b20c_b(c, 0x8025b20c); crate::prof::exit(c, 0x8025b20c); return; }
    }
    f_8025b20c_b(c, 0x8025b20c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025b20c_at(c: &mut Cpu, pc: u32) { f_8025b20c_b(c, pc) }
#[inline(always)]
fn f_8025b20c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025b20c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025b20c, pc); }
        match pc {
            0x8025b20c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x90;
                c.lr = 0x8025b234; f_8021c034(c);
                pc = 0x8025b234;
            }
            0x8025b234 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b27c; continue; }
                pc = 0x8025b23c;
            }
            0x8025b23c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x8025b24c; f_80236bfc(c);
                pc = 0x8025b24c;
            }
            0x8025b24c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025b260; continue; }
                pc = 0x8025b258;
            }
            0x8025b258 => {
                c.r[3] = 0x0;
                pc = 0x8025b2e0; continue;
                pc = 0x8025b260;
            }
            0x8025b260 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x8025b278; f_8025c264(c);
                pc = 0x8025b278;
            }
            0x8025b278 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025b27c;
            }
            0x8025b27c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x90;
                c.r[8] = 0x6;
                c.r[9] = 0x1;
                c.lr = 0x8025b29c; f_8025c808(c);
                pc = 0x8025b29c;
            }
            0x8025b29c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025b2ac; continue; }
                pc = 0x8025b2a4;
            }
            0x8025b2a4 => {
                c.r[3] = 0x0;
                pc = 0x8025b2e0; continue;
                pc = 0x8025b2ac;
            }
            0x8025b2ac => {
                c.r[3] = 0x90;
                c.lr = 0x8025b2b4; f_8021c034(c);
                pc = 0x8025b2b4;
            }
            0x8025b2b4 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b2dc; continue; }
                pc = 0x8025b2bc;
            }
            0x8025b2bc => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025b2cc; continue; }
                pc = 0x8025b2c4;
            }
            0x8025b2c4 => {
                c.r[3] = 0x0;
                pc = 0x8025b2e0; continue;
                pc = 0x8025b2cc;
            }
            0x8025b2cc => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025b2dc; continue; }
                pc = 0x8025b2d4;
            }
            0x8025b2d4 => {
                c.r[3] = 0x1;
                pc = 0x8025b2e0; continue;
                pc = 0x8025b2dc;
            }
            0x8025b2dc => {
                c.r[3] = 0x1;
                pc = 0x8025b2e0;
            }
            0x8025b2e0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025b20c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit020 (0x8025b2f4, 232 bytes)
#[inline(never)]
pub fn f_8025b2f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025b2f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8025b2f4); f_8025b2f4_b(c, 0x8025b2f4); crate::prof::exit(c, 0x8025b2f4); return; }
    }
    f_8025b2f4_b(c, 0x8025b2f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025b2f4_at(c: &mut Cpu, pc: u32) { f_8025b2f4_b(c, pc) }
#[inline(always)]
fn f_8025b2f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025b2f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025b2f4, pc); }
        match pc {
            0x8025b2f4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x90;
                c.lr = 0x8025b31c; f_8021c034(c);
                pc = 0x8025b31c;
            }
            0x8025b31c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b364; continue; }
                pc = 0x8025b324;
            }
            0x8025b324 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x8025b334; f_80236bfc(c);
                pc = 0x8025b334;
            }
            0x8025b334 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025b348; continue; }
                pc = 0x8025b340;
            }
            0x8025b340 => {
                c.r[3] = 0x0;
                pc = 0x8025b3c8; continue;
                pc = 0x8025b348;
            }
            0x8025b348 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x8025b360; f_8025c264(c);
                pc = 0x8025b360;
            }
            0x8025b360 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025b364;
            }
            0x8025b364 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x90;
                c.r[8] = 0x3;
                c.r[9] = 0x1;
                c.lr = 0x8025b384; f_8025c808(c);
                pc = 0x8025b384;
            }
            0x8025b384 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025b394; continue; }
                pc = 0x8025b38c;
            }
            0x8025b38c => {
                c.r[3] = 0x0;
                pc = 0x8025b3c8; continue;
                pc = 0x8025b394;
            }
            0x8025b394 => {
                c.r[3] = 0x90;
                c.lr = 0x8025b39c; f_8021c034(c);
                pc = 0x8025b39c;
            }
            0x8025b39c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b3c4; continue; }
                pc = 0x8025b3a4;
            }
            0x8025b3a4 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025b3b4; continue; }
                pc = 0x8025b3ac;
            }
            0x8025b3ac => {
                c.r[3] = 0x0;
                pc = 0x8025b3c8; continue;
                pc = 0x8025b3b4;
            }
            0x8025b3b4 => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025b3c4; continue; }
                pc = 0x8025b3bc;
            }
            0x8025b3bc => {
                c.r[3] = 0x1;
                pc = 0x8025b3c8; continue;
                pc = 0x8025b3c4;
            }
            0x8025b3c4 => {
                c.r[3] = 0x1;
                pc = 0x8025b3c8;
            }
            0x8025b3c8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025b2f4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit019 (0x8025b3dc, 232 bytes)
#[inline(never)]
pub fn f_8025b3dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025b3dc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025b3dc); f_8025b3dc_b(c, 0x8025b3dc); crate::prof::exit(c, 0x8025b3dc); return; }
    }
    f_8025b3dc_b(c, 0x8025b3dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025b3dc_at(c: &mut Cpu, pc: u32) { f_8025b3dc_b(c, pc) }
#[inline(always)]
fn f_8025b3dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025b3dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025b3dc, pc); }
        match pc {
            0x8025b3dc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x90;
                c.lr = 0x8025b404; f_8021c034(c);
                pc = 0x8025b404;
            }
            0x8025b404 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b44c; continue; }
                pc = 0x8025b40c;
            }
            0x8025b40c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x8025b41c; f_80236bfc(c);
                pc = 0x8025b41c;
            }
            0x8025b41c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025b430; continue; }
                pc = 0x8025b428;
            }
            0x8025b428 => {
                c.r[3] = 0x0;
                pc = 0x8025b4b0; continue;
                pc = 0x8025b430;
            }
            0x8025b430 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x8025b448; f_8025c264(c);
                pc = 0x8025b448;
            }
            0x8025b448 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025b44c;
            }
            0x8025b44c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x90;
                c.r[8] = 0x2;
                c.r[9] = 0x1;
                c.lr = 0x8025b46c; f_8025c808(c);
                pc = 0x8025b46c;
            }
            0x8025b46c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025b47c; continue; }
                pc = 0x8025b474;
            }
            0x8025b474 => {
                c.r[3] = 0x0;
                pc = 0x8025b4b0; continue;
                pc = 0x8025b47c;
            }
            0x8025b47c => {
                c.r[3] = 0x90;
                c.lr = 0x8025b484; f_8021c034(c);
                pc = 0x8025b484;
            }
            0x8025b484 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b4ac; continue; }
                pc = 0x8025b48c;
            }
            0x8025b48c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025b49c; continue; }
                pc = 0x8025b494;
            }
            0x8025b494 => {
                c.r[3] = 0x0;
                pc = 0x8025b4b0; continue;
                pc = 0x8025b49c;
            }
            0x8025b49c => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025b4ac; continue; }
                pc = 0x8025b4a4;
            }
            0x8025b4a4 => {
                c.r[3] = 0x1;
                pc = 0x8025b4b0; continue;
                pc = 0x8025b4ac;
            }
            0x8025b4ac => {
                c.r[3] = 0x1;
                pc = 0x8025b4b0;
            }
            0x8025b4b0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025b3dc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit018 (0x8025b4c4, 232 bytes)
#[inline(never)]
pub fn f_8025b4c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025b4c4) { return; }
        if c.prof { crate::prof::enter(c, 0x8025b4c4); f_8025b4c4_b(c, 0x8025b4c4); crate::prof::exit(c, 0x8025b4c4); return; }
    }
    f_8025b4c4_b(c, 0x8025b4c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025b4c4_at(c: &mut Cpu, pc: u32) { f_8025b4c4_b(c, pc) }
#[inline(always)]
fn f_8025b4c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025b4c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025b4c4, pc); }
        match pc {
            0x8025b4c4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x90;
                c.lr = 0x8025b4ec; f_8021c034(c);
                pc = 0x8025b4ec;
            }
            0x8025b4ec => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b534; continue; }
                pc = 0x8025b4f4;
            }
            0x8025b4f4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x8025b504; f_80236bfc(c);
                pc = 0x8025b504;
            }
            0x8025b504 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025b518; continue; }
                pc = 0x8025b510;
            }
            0x8025b510 => {
                c.r[3] = 0x0;
                pc = 0x8025b598; continue;
                pc = 0x8025b518;
            }
            0x8025b518 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x8025b530; f_8025c264(c);
                pc = 0x8025b530;
            }
            0x8025b530 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025b534;
            }
            0x8025b534 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x90;
                c.r[8] = 0x1;
                c.r[9] = 0x1;
                c.lr = 0x8025b554; f_8025c808(c);
                pc = 0x8025b554;
            }
            0x8025b554 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025b564; continue; }
                pc = 0x8025b55c;
            }
            0x8025b55c => {
                c.r[3] = 0x0;
                pc = 0x8025b598; continue;
                pc = 0x8025b564;
            }
            0x8025b564 => {
                c.r[3] = 0x90;
                c.lr = 0x8025b56c; f_8021c034(c);
                pc = 0x8025b56c;
            }
            0x8025b56c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b594; continue; }
                pc = 0x8025b574;
            }
            0x8025b574 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025b584; continue; }
                pc = 0x8025b57c;
            }
            0x8025b57c => {
                c.r[3] = 0x0;
                pc = 0x8025b598; continue;
                pc = 0x8025b584;
            }
            0x8025b584 => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025b594; continue; }
                pc = 0x8025b58c;
            }
            0x8025b58c => {
                c.r[3] = 0x1;
                pc = 0x8025b598; continue;
                pc = 0x8025b594;
            }
            0x8025b594 => {
                c.r[3] = 0x1;
                pc = 0x8025b598;
            }
            0x8025b598 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025b4c4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit017 (0x8025b5ac, 180 bytes)
#[inline(never)]
pub fn f_8025b5ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025b5ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8025b5ac); f_8025b5ac_b(c, 0x8025b5ac); crate::prof::exit(c, 0x8025b5ac); return; }
    }
    f_8025b5ac_b(c, 0x8025b5ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025b5ac_at(c: &mut Cpu, pc: u32) { f_8025b5ac_b(c, pc) }
#[inline(always)]
fn f_8025b5ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025b5ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025b5ac, pc); }
        match pc {
            0x8025b5ac => {
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
                c.lr = 0x8025b5d8; f_802395c8(c);
                pc = 0x8025b5d8;
            }
            0x8025b5d8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025b5f4; f_8025c264(c);
                pc = 0x8025b5f4;
            }
            0x8025b5f4 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025b604; f_80239500(c);
                pc = 0x8025b604;
            }
            0x8025b604 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025b618; f_8023793c(c);
                pc = 0x8025b618;
            }
            0x8025b618 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025b628; continue; }
                pc = 0x8025b624;
            }
            0x8025b624 => {
                c.r[30] = 0x0;
                pc = 0x8025b628;
            }
            0x8025b628 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025b638; continue; }
                pc = 0x8025b630;
            }
            0x8025b630 => {
                c.r[3] = 0x0;
                pc = 0x8025b64c; continue;
                pc = 0x8025b638;
            }
            0x8025b638 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025b648; continue; }
                pc = 0x8025b640;
            }
            0x8025b640 => {
                c.r[3] = 0x1;
                pc = 0x8025b64c; continue;
                pc = 0x8025b648;
            }
            0x8025b648 => {
                c.r[3] = 0x1;
                pc = 0x8025b64c;
            }
            0x8025b64c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025b5ac: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit016 (0x8025b660, 232 bytes)
#[inline(never)]
pub fn f_8025b660(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025b660) { return; }
        if c.prof { crate::prof::enter(c, 0x8025b660); f_8025b660_b(c, 0x8025b660); crate::prof::exit(c, 0x8025b660); return; }
    }
    f_8025b660_b(c, 0x8025b660)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025b660_at(c: &mut Cpu, pc: u32) { f_8025b660_b(c, pc) }
#[inline(always)]
fn f_8025b660_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025b660;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025b660, pc); }
        match pc {
            0x8025b660 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x10;
                c.lr = 0x8025b688; f_8021c034(c);
                pc = 0x8025b688;
            }
            0x8025b688 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b6d0; continue; }
                pc = 0x8025b690;
            }
            0x8025b690 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x8025b6a0; f_80236bfc(c);
                pc = 0x8025b6a0;
            }
            0x8025b6a0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025b6b4; continue; }
                pc = 0x8025b6ac;
            }
            0x8025b6ac => {
                c.r[3] = 0x0;
                pc = 0x8025b734; continue;
                pc = 0x8025b6b4;
            }
            0x8025b6b4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x8025b6cc; f_8025c264(c);
                pc = 0x8025b6cc;
            }
            0x8025b6cc => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025b6d0;
            }
            0x8025b6d0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x7;
                c.r[9] = 0x41;
                c.lr = 0x8025b6f0; f_8025c808(c);
                pc = 0x8025b6f0;
            }
            0x8025b6f0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025b700; continue; }
                pc = 0x8025b6f8;
            }
            0x8025b6f8 => {
                c.r[3] = 0x0;
                pc = 0x8025b734; continue;
                pc = 0x8025b700;
            }
            0x8025b700 => {
                c.r[3] = 0x10;
                c.lr = 0x8025b708; f_8021c034(c);
                pc = 0x8025b708;
            }
            0x8025b708 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b730; continue; }
                pc = 0x8025b710;
            }
            0x8025b710 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025b720; continue; }
                pc = 0x8025b718;
            }
            0x8025b718 => {
                c.r[3] = 0x0;
                pc = 0x8025b734; continue;
                pc = 0x8025b720;
            }
            0x8025b720 => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025b730; continue; }
                pc = 0x8025b728;
            }
            0x8025b728 => {
                c.r[3] = 0x1;
                pc = 0x8025b734; continue;
                pc = 0x8025b730;
            }
            0x8025b730 => {
                c.r[3] = 0x1;
                pc = 0x8025b734;
            }
            0x8025b734 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025b660: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit013 (0x8025b748, 232 bytes)
#[inline(never)]
pub fn f_8025b748(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025b748) { return; }
        if c.prof { crate::prof::enter(c, 0x8025b748); f_8025b748_b(c, 0x8025b748); crate::prof::exit(c, 0x8025b748); return; }
    }
    f_8025b748_b(c, 0x8025b748)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025b748_at(c: &mut Cpu, pc: u32) { f_8025b748_b(c, pc) }
#[inline(always)]
fn f_8025b748_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025b748;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025b748, pc); }
        match pc {
            0x8025b748 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x10;
                c.lr = 0x8025b770; f_8021c034(c);
                pc = 0x8025b770;
            }
            0x8025b770 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b7b8; continue; }
                pc = 0x8025b778;
            }
            0x8025b778 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x8025b788; f_80236bfc(c);
                pc = 0x8025b788;
            }
            0x8025b788 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025b79c; continue; }
                pc = 0x8025b794;
            }
            0x8025b794 => {
                c.r[3] = 0x0;
                pc = 0x8025b81c; continue;
                pc = 0x8025b79c;
            }
            0x8025b79c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x8025b7b4; f_8025c264(c);
                pc = 0x8025b7b4;
            }
            0x8025b7b4 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025b7b8;
            }
            0x8025b7b8 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x4;
                c.r[9] = 0x41;
                c.lr = 0x8025b7d8; f_8025c808(c);
                pc = 0x8025b7d8;
            }
            0x8025b7d8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025b7e8; continue; }
                pc = 0x8025b7e0;
            }
            0x8025b7e0 => {
                c.r[3] = 0x0;
                pc = 0x8025b81c; continue;
                pc = 0x8025b7e8;
            }
            0x8025b7e8 => {
                c.r[3] = 0x10;
                c.lr = 0x8025b7f0; f_8021c034(c);
                pc = 0x8025b7f0;
            }
            0x8025b7f0 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b818; continue; }
                pc = 0x8025b7f8;
            }
            0x8025b7f8 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025b808; continue; }
                pc = 0x8025b800;
            }
            0x8025b800 => {
                c.r[3] = 0x0;
                pc = 0x8025b81c; continue;
                pc = 0x8025b808;
            }
            0x8025b808 => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025b818; continue; }
                pc = 0x8025b810;
            }
            0x8025b810 => {
                c.r[3] = 0x1;
                pc = 0x8025b81c; continue;
                pc = 0x8025b818;
            }
            0x8025b818 => {
                c.r[3] = 0x1;
                pc = 0x8025b81c;
            }
            0x8025b81c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025b748: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit011 (0x8025b830, 232 bytes)
#[inline(never)]
pub fn f_8025b830(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025b830) { return; }
        if c.prof { crate::prof::enter(c, 0x8025b830); f_8025b830_b(c, 0x8025b830); crate::prof::exit(c, 0x8025b830); return; }
    }
    f_8025b830_b(c, 0x8025b830)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025b830_at(c: &mut Cpu, pc: u32) { f_8025b830_b(c, pc) }
#[inline(always)]
fn f_8025b830_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025b830;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025b830, pc); }
        match pc {
            0x8025b830 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x10;
                c.lr = 0x8025b858; f_8021c034(c);
                pc = 0x8025b858;
            }
            0x8025b858 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b8a0; continue; }
                pc = 0x8025b860;
            }
            0x8025b860 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x8025b870; f_80236bfc(c);
                pc = 0x8025b870;
            }
            0x8025b870 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025b884; continue; }
                pc = 0x8025b87c;
            }
            0x8025b87c => {
                c.r[3] = 0x0;
                pc = 0x8025b904; continue;
                pc = 0x8025b884;
            }
            0x8025b884 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x8025b89c; f_8025c264(c);
                pc = 0x8025b89c;
            }
            0x8025b89c => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025b8a0;
            }
            0x8025b8a0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x2;
                c.r[9] = 0x41;
                c.lr = 0x8025b8c0; f_8025c808(c);
                pc = 0x8025b8c0;
            }
            0x8025b8c0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025b8d0; continue; }
                pc = 0x8025b8c8;
            }
            0x8025b8c8 => {
                c.r[3] = 0x0;
                pc = 0x8025b904; continue;
                pc = 0x8025b8d0;
            }
            0x8025b8d0 => {
                c.r[3] = 0x10;
                c.lr = 0x8025b8d8; f_8021c034(c);
                pc = 0x8025b8d8;
            }
            0x8025b8d8 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b900; continue; }
                pc = 0x8025b8e0;
            }
            0x8025b8e0 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025b8f0; continue; }
                pc = 0x8025b8e8;
            }
            0x8025b8e8 => {
                c.r[3] = 0x0;
                pc = 0x8025b904; continue;
                pc = 0x8025b8f0;
            }
            0x8025b8f0 => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025b900; continue; }
                pc = 0x8025b8f8;
            }
            0x8025b8f8 => {
                c.r[3] = 0x1;
                pc = 0x8025b904; continue;
                pc = 0x8025b900;
            }
            0x8025b900 => {
                c.r[3] = 0x1;
                pc = 0x8025b904;
            }
            0x8025b904 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025b830: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit010 (0x8025b918, 232 bytes)
#[inline(never)]
pub fn f_8025b918(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025b918) { return; }
        if c.prof { crate::prof::enter(c, 0x8025b918); f_8025b918_b(c, 0x8025b918); crate::prof::exit(c, 0x8025b918); return; }
    }
    f_8025b918_b(c, 0x8025b918)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025b918_at(c: &mut Cpu, pc: u32) { f_8025b918_b(c, pc) }
#[inline(always)]
fn f_8025b918_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025b918;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025b918, pc); }
        match pc {
            0x8025b918 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x10;
                c.lr = 0x8025b940; f_8021c034(c);
                pc = 0x8025b940;
            }
            0x8025b940 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b988; continue; }
                pc = 0x8025b948;
            }
            0x8025b948 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x8025b958; f_80236bfc(c);
                pc = 0x8025b958;
            }
            0x8025b958 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025b96c; continue; }
                pc = 0x8025b964;
            }
            0x8025b964 => {
                c.r[3] = 0x0;
                pc = 0x8025b9ec; continue;
                pc = 0x8025b96c;
            }
            0x8025b96c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x8025b984; f_8025c264(c);
                pc = 0x8025b984;
            }
            0x8025b984 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025b988;
            }
            0x8025b988 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x10;
                c.r[8] = 0x1;
                c.r[9] = 0x41;
                c.lr = 0x8025b9a8; f_8025c808(c);
                pc = 0x8025b9a8;
            }
            0x8025b9a8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025b9b8; continue; }
                pc = 0x8025b9b0;
            }
            0x8025b9b0 => {
                c.r[3] = 0x0;
                pc = 0x8025b9ec; continue;
                pc = 0x8025b9b8;
            }
            0x8025b9b8 => {
                c.r[3] = 0x10;
                c.lr = 0x8025b9c0; f_8021c034(c);
                pc = 0x8025b9c0;
            }
            0x8025b9c0 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025b9e8; continue; }
                pc = 0x8025b9c8;
            }
            0x8025b9c8 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025b9d8; continue; }
                pc = 0x8025b9d0;
            }
            0x8025b9d0 => {
                c.r[3] = 0x0;
                pc = 0x8025b9ec; continue;
                pc = 0x8025b9d8;
            }
            0x8025b9d8 => {
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025b9e8; continue; }
                pc = 0x8025b9e0;
            }
            0x8025b9e0 => {
                c.r[3] = 0x1;
                pc = 0x8025b9ec; continue;
                pc = 0x8025b9e8;
            }
            0x8025b9e8 => {
                c.r[3] = 0x1;
                pc = 0x8025b9ec;
            }
            0x8025b9ec => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025b918: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit009 (0x8025ba00, 44 bytes)
#[inline(never)]
pub fn f_8025ba00(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025ba00) { return; }
        if c.prof { crate::prof::enter(c, 0x8025ba00); f_8025ba00_b(c, 0x8025ba00); crate::prof::exit(c, 0x8025ba00); return; }
    }
    f_8025ba00_b(c, 0x8025ba00)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025ba00_at(c: &mut Cpu, pc: u32) { f_8025ba00_b(c, pc) }
#[inline(always)]
fn f_8025ba00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025ba00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025ba00, pc); }
        match pc {
            0x8025ba00 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025ba10; f_8025cb3c(c);
                pc = 0x8025ba10;
            }
            0x8025ba10 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025ba00: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit008 (0x8025ba2c, 244 bytes)
#[inline(never)]
pub fn f_8025ba2c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025ba2c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025ba2c); f_8025ba2c_b(c, 0x8025ba2c); crate::prof::exit(c, 0x8025ba2c); return; }
    }
    f_8025ba2c_b(c, 0x8025ba2c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025ba2c_at(c: &mut Cpu, pc: u32) { f_8025ba2c_b(c, pc) }
#[inline(always)]
fn f_8025ba2c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025ba2c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025ba2c, pc); }
        match pc {
            0x8025ba2c => {
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
                c.lr = 0x8025ba58; f_802395c8(c);
                pc = 0x8025ba58;
            }
            0x8025ba58 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x14;
                c.lr = 0x8025ba70; f_80236bfc(c);
                pc = 0x8025ba70;
            }
            0x8025ba70 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025ba84; continue; }
                pc = 0x8025ba7c;
            }
            0x8025ba7c => {
                c.r[3] = 0x0;
                pc = 0x8025bb0c; continue;
                pc = 0x8025ba84;
            }
            0x8025ba84 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x8;
                c.lr = 0x8025ba94; f_80236bfc(c);
                pc = 0x8025ba94;
            }
            0x8025ba94 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025baa4; continue; }
                pc = 0x8025ba9c;
            }
            0x8025ba9c => {
                c.r[3] = 0x0;
                pc = 0x8025bb0c; continue;
                pc = 0x8025baa4;
            }
            0x8025baa4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025babc; f_8025c264(c);
                pc = 0x8025babc;
            }
            0x8025babc => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025bacc; f_80239500(c);
                pc = 0x8025bacc;
            }
            0x8025bacc => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8025bae0; f_8023793c(c);
                pc = 0x8025bae0;
            }
            0x8025bae0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025baf4; continue; }
                pc = 0x8025baec;
            }
            0x8025baec => {
                c.r[3] = 0x0;
                pc = 0x8025bb0c; continue;
                pc = 0x8025baf4;
            }
            0x8025baf4 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025bb04; continue; }
                pc = 0x8025bafc;
            }
            0x8025bafc => {
                c.r[3] = 0x0;
                pc = 0x8025bb0c; continue;
                pc = 0x8025bb04;
            }
            0x8025bb04 => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025bb0c;
            }
            0x8025bb0c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025ba2c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit007 (0x8025bb20, 264 bytes)
#[inline(never)]
pub fn f_8025bb20(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025bb20) { return; }
        if c.prof { crate::prof::enter(c, 0x8025bb20); f_8025bb20_b(c, 0x8025bb20); crate::prof::exit(c, 0x8025bb20); return; }
    }
    f_8025bb20_b(c, 0x8025bb20)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025bb20_at(c: &mut Cpu, pc: u32) { f_8025bb20_b(c, pc) }
#[inline(always)]
fn f_8025bb20_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025bb20;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025bb20, pc); }
        match pc {
            0x8025bb20 => {
                { let ea = c.r[1].wrapping_add(0xffffff80); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x84), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x68); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[26] = c.r[3] | c.r[3];
                c.r[29] = c.r[6] | c.r[6];
                c.r[5] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025bb4c; f_802395c8(c);
                pc = 0x8025bb4c;
            }
            0x8025bb4c => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x1;
                c.r[7] = 0x1;
                c.lr = 0x8025bb68; f_801f1a6c(c);
                pc = 0x8025bb68;
            }
            0x8025bb68 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8025bb74; f_8022967c(c);
                pc = 0x8025bb74;
            }
            0x8025bb74 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025bb94; continue; }
                pc = 0x8025bb80;
            }
            0x8025bb80 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(1) { pc = 0x8025bb94; continue; }
                pc = 0x8025bb8c;
            }
            0x8025bb8c => {
                c.r[3] = 0x0;
                pc = 0x8025bc14; continue;
                pc = 0x8025bb94;
            }
            0x8025bb94 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8025bb9c; f_8025cbe8(c);
                pc = 0x8025bb9c;
            }
            0x8025bb9c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025bbb0; continue; }
                pc = 0x8025bba8;
            }
            0x8025bba8 => {
                c.r[3] = 0x0;
                pc = 0x8025bc14; continue;
                pc = 0x8025bbb0;
            }
            0x8025bbb0 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025bbbc; f_80239500(c);
                pc = 0x8025bbbc;
            }
            0x8025bbbc => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8025bbd0; f_8023793c(c);
                pc = 0x8025bbd0;
            }
            0x8025bbd0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025bbe4; continue; }
                pc = 0x8025bbdc;
            }
            0x8025bbdc => {
                c.r[3] = 0x0;
                pc = 0x8025bc14; continue;
                pc = 0x8025bbe4;
            }
            0x8025bbe4 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025bbfc; f_8025c264(c);
                pc = 0x8025bbfc;
            }
            0x8025bbfc => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8025bc0c; continue; }
                pc = 0x8025bc04;
            }
            0x8025bc04 => {
                c.r[3] = 0x0;
                pc = 0x8025bc14; continue;
                pc = 0x8025bc0c;
            }
            0x8025bc0c => {
                c.cmps(0, c.r[3], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025bc14;
            }
            0x8025bc14 => {
                { let mut ea = c.r[1].wrapping_add(0x68); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x84));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x80);
                if true { return; }
                return;
            }
            _ => panic!("f_8025bb20: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit006 (0x8025bc28, 180 bytes)
#[inline(never)]
pub fn f_8025bc28(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025bc28) { return; }
        if c.prof { crate::prof::enter(c, 0x8025bc28); f_8025bc28_b(c, 0x8025bc28); crate::prof::exit(c, 0x8025bc28); return; }
    }
    f_8025bc28_b(c, 0x8025bc28)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025bc28_at(c: &mut Cpu, pc: u32) { f_8025bc28_b(c, pc) }
#[inline(always)]
fn f_8025bc28_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025bc28;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025bc28, pc); }
        match pc {
            0x8025bc28 => {
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
                c.lr = 0x8025bc54; f_802395c8(c);
                pc = 0x8025bc54;
            }
            0x8025bc54 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025bc70; f_8025c264(c);
                pc = 0x8025bc70;
            }
            0x8025bc70 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025bc80; f_80239500(c);
                pc = 0x8025bc80;
            }
            0x8025bc80 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025bc94; f_8023793c(c);
                pc = 0x8025bc94;
            }
            0x8025bc94 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025bca4; continue; }
                pc = 0x8025bca0;
            }
            0x8025bca0 => {
                c.r[30] = 0x0;
                pc = 0x8025bca4;
            }
            0x8025bca4 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025bcb4; continue; }
                pc = 0x8025bcac;
            }
            0x8025bcac => {
                c.r[3] = 0x0;
                pc = 0x8025bcc8; continue;
                pc = 0x8025bcb4;
            }
            0x8025bcb4 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025bcc4; continue; }
                pc = 0x8025bcbc;
            }
            0x8025bcbc => {
                c.r[3] = 0x1;
                pc = 0x8025bcc8; continue;
                pc = 0x8025bcc4;
            }
            0x8025bcc4 => {
                c.r[3] = 0x1;
                pc = 0x8025bcc8;
            }
            0x8025bcc8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025bc28: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit005 (0x8025bcdc, 180 bytes)
#[inline(never)]
pub fn f_8025bcdc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025bcdc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025bcdc); f_8025bcdc_b(c, 0x8025bcdc); crate::prof::exit(c, 0x8025bcdc); return; }
    }
    f_8025bcdc_b(c, 0x8025bcdc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025bcdc_at(c: &mut Cpu, pc: u32) { f_8025bcdc_b(c, pc) }
#[inline(always)]
fn f_8025bcdc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025bcdc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025bcdc, pc); }
        match pc {
            0x8025bcdc => {
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
                c.lr = 0x8025bd08; f_802395c8(c);
                pc = 0x8025bd08;
            }
            0x8025bd08 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025bd24; f_8025c264(c);
                pc = 0x8025bd24;
            }
            0x8025bd24 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025bd34; f_80239500(c);
                pc = 0x8025bd34;
            }
            0x8025bd34 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025bd48; f_8023793c(c);
                pc = 0x8025bd48;
            }
            0x8025bd48 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025bd58; continue; }
                pc = 0x8025bd54;
            }
            0x8025bd54 => {
                c.r[30] = 0x0;
                pc = 0x8025bd58;
            }
            0x8025bd58 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025bd68; continue; }
                pc = 0x8025bd60;
            }
            0x8025bd60 => {
                c.r[3] = 0x0;
                pc = 0x8025bd7c; continue;
                pc = 0x8025bd68;
            }
            0x8025bd68 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025bd78; continue; }
                pc = 0x8025bd70;
            }
            0x8025bd70 => {
                c.r[3] = 0x1;
                pc = 0x8025bd7c; continue;
                pc = 0x8025bd78;
            }
            0x8025bd78 => {
                c.r[3] = 0x1;
                pc = 0x8025bd7c;
            }
            0x8025bd7c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025bcdc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit004 (0x8025bd90, 180 bytes)
#[inline(never)]
pub fn f_8025bd90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025bd90) { return; }
        if c.prof { crate::prof::enter(c, 0x8025bd90); f_8025bd90_b(c, 0x8025bd90); crate::prof::exit(c, 0x8025bd90); return; }
    }
    f_8025bd90_b(c, 0x8025bd90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025bd90_at(c: &mut Cpu, pc: u32) { f_8025bd90_b(c, pc) }
#[inline(always)]
fn f_8025bd90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025bd90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025bd90, pc); }
        match pc {
            0x8025bd90 => {
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
                c.lr = 0x8025bdbc; f_802395c8(c);
                pc = 0x8025bdbc;
            }
            0x8025bdbc => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025bdd8; f_8025c264(c);
                pc = 0x8025bdd8;
            }
            0x8025bdd8 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025bde8; f_80239500(c);
                pc = 0x8025bde8;
            }
            0x8025bde8 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025bdfc; f_8023793c(c);
                pc = 0x8025bdfc;
            }
            0x8025bdfc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025be0c; continue; }
                pc = 0x8025be08;
            }
            0x8025be08 => {
                c.r[30] = 0x0;
                pc = 0x8025be0c;
            }
            0x8025be0c => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025be1c; continue; }
                pc = 0x8025be14;
            }
            0x8025be14 => {
                c.r[3] = 0x0;
                pc = 0x8025be30; continue;
                pc = 0x8025be1c;
            }
            0x8025be1c => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025be2c; continue; }
                pc = 0x8025be24;
            }
            0x8025be24 => {
                c.r[3] = 0x1;
                pc = 0x8025be30; continue;
                pc = 0x8025be2c;
            }
            0x8025be2c => {
                c.r[3] = 0x1;
                pc = 0x8025be30;
            }
            0x8025be30 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025bd90: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit003 (0x8025be44, 180 bytes)
#[inline(never)]
pub fn f_8025be44(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025be44) { return; }
        if c.prof { crate::prof::enter(c, 0x8025be44); f_8025be44_b(c, 0x8025be44); crate::prof::exit(c, 0x8025be44); return; }
    }
    f_8025be44_b(c, 0x8025be44)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025be44_at(c: &mut Cpu, pc: u32) { f_8025be44_b(c, pc) }
#[inline(always)]
fn f_8025be44_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025be44;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025be44, pc); }
        match pc {
            0x8025be44 => {
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
                c.lr = 0x8025be70; f_802395c8(c);
                pc = 0x8025be70;
            }
            0x8025be70 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025be8c; f_8025c264(c);
                pc = 0x8025be8c;
            }
            0x8025be8c => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025be9c; f_80239500(c);
                pc = 0x8025be9c;
            }
            0x8025be9c => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025beb0; f_8023793c(c);
                pc = 0x8025beb0;
            }
            0x8025beb0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025bec0; continue; }
                pc = 0x8025bebc;
            }
            0x8025bebc => {
                c.r[30] = 0x0;
                pc = 0x8025bec0;
            }
            0x8025bec0 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025bed0; continue; }
                pc = 0x8025bec8;
            }
            0x8025bec8 => {
                c.r[3] = 0x0;
                pc = 0x8025bee4; continue;
                pc = 0x8025bed0;
            }
            0x8025bed0 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025bee0; continue; }
                pc = 0x8025bed8;
            }
            0x8025bed8 => {
                c.r[3] = 0x1;
                pc = 0x8025bee4; continue;
                pc = 0x8025bee0;
            }
            0x8025bee0 => {
                c.r[3] = 0x1;
                pc = 0x8025bee4;
            }
            0x8025bee4 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025be44: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit002 (0x8025bef8, 180 bytes)
#[inline(never)]
pub fn f_8025bef8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025bef8) { return; }
        if c.prof { crate::prof::enter(c, 0x8025bef8); f_8025bef8_b(c, 0x8025bef8); crate::prof::exit(c, 0x8025bef8); return; }
    }
    f_8025bef8_b(c, 0x8025bef8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025bef8_at(c: &mut Cpu, pc: u32) { f_8025bef8_b(c, pc) }
#[inline(always)]
fn f_8025bef8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025bef8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025bef8, pc); }
        match pc {
            0x8025bef8 => {
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
                c.lr = 0x8025bf24; f_802395c8(c);
                pc = 0x8025bf24;
            }
            0x8025bf24 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025bf40; f_8025c264(c);
                pc = 0x8025bf40;
            }
            0x8025bf40 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025bf50; f_80239500(c);
                pc = 0x8025bf50;
            }
            0x8025bf50 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8025bf64; f_8023793c(c);
                pc = 0x8025bf64;
            }
            0x8025bf64 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025bf74; continue; }
                pc = 0x8025bf70;
            }
            0x8025bf70 => {
                c.r[30] = 0x0;
                pc = 0x8025bf74;
            }
            0x8025bf74 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8025bf84; continue; }
                pc = 0x8025bf7c;
            }
            0x8025bf7c => {
                c.r[3] = 0x0;
                pc = 0x8025bf98; continue;
                pc = 0x8025bf84;
            }
            0x8025bf84 => {
                c.cmps(0, c.r[30], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025bf94; continue; }
                pc = 0x8025bf8c;
            }
            0x8025bf8c => {
                c.r[3] = 0x1;
                pc = 0x8025bf98; continue;
                pc = 0x8025bf94;
            }
            0x8025bf94 => {
                c.r[3] = 0x1;
                pc = 0x8025bf98;
            }
            0x8025bf98 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025bef8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit001 (0x8025bfac, 512 bytes)
#[inline(never)]
pub fn f_8025bfac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025bfac) { return; }
        if c.prof { crate::prof::enter(c, 0x8025bfac); f_8025bfac_b(c, 0x8025bfac); crate::prof::exit(c, 0x8025bfac); return; }
    }
    f_8025bfac_b(c, 0x8025bfac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025bfac_at(c: &mut Cpu, pc: u32) { f_8025bfac_b(c, pc) }
#[inline(always)]
fn f_8025bfac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025bfac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025bfac, pc); }
        match pc {
            0x8025bfac => {
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
                c.lr = 0x8025bfd8; f_80236bfc(c);
                pc = 0x8025bfd8;
            }
            0x8025bfd8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025bfec; continue; }
                pc = 0x8025bfe4;
            }
            0x8025bfe4 => {
                c.r[3] = 0x0;
                pc = 0x8025c198; continue;
                pc = 0x8025bfec;
            }
            0x8025bfec => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x8;
                c.lr = 0x8025bffc; f_80236bfc(c);
                pc = 0x8025bffc;
            }
            0x8025bffc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c010; continue; }
                pc = 0x8025c008;
            }
            0x8025c008 => {
                c.r[3] = 0x0;
                pc = 0x8025c198; continue;
                pc = 0x8025c010;
            }
            0x8025c010 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8025c01c; f_8025cc90(c);
                pc = 0x8025c01c;
            }
            0x8025c01c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c030; continue; }
                pc = 0x8025c028;
            }
            0x8025c028 => {
                c.r[3] = 0x0;
                pc = 0x8025c198; continue;
                pc = 0x8025c030;
            }
            0x8025c030 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8025c03c; f_80237310(c);
                pc = 0x8025c03c;
            }
            0x8025c03c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025c04c; continue; }
                pc = 0x8025c044;
            }
            0x8025c044 => {
                c.r[3] = 0x0;
                pc = 0x8025c198; continue;
                pc = 0x8025c04c;
            }
            0x8025c04c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x11;
                c.lr = 0x8025c05c; f_80237f74(c);
                pc = 0x8025c05c;
            }
            0x8025c05c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025c128; continue; }
                pc = 0x8025c068;
            }
            0x8025c068 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x14;
                c.lr = 0x8025c078; f_80237f74(c);
                pc = 0x8025c078;
            }
            0x8025c078 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025c128; continue; }
                pc = 0x8025c084;
            }
            0x8025c084 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x7;
                c.lr = 0x8025c094; f_80237f74(c);
                pc = 0x8025c094;
            }
            0x8025c094 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025c128; continue; }
                pc = 0x8025c0a0;
            }
            0x8025c0a0 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0xf;
                c.lr = 0x8025c0b0; f_80237f74(c);
                pc = 0x8025c0b0;
            }
            0x8025c0b0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025c0d8; continue; }
                pc = 0x8025c0bc;
            }
            0x8025c0bc => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x48;
                c.lr = 0x8025c0cc; f_80237f74(c);
                pc = 0x8025c0cc;
            }
            0x8025c0cc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c0e0; continue; }
                pc = 0x8025c0d8;
            }
            0x8025c0d8 => {
                c.r[0] = 0x0;
                pc = 0x8025c12c; continue;
                pc = 0x8025c0e0;
            }
            0x8025c0e0 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x29;
                c.lr = 0x8025c0f0; f_80237f74(c);
                pc = 0x8025c0f0;
            }
            0x8025c0f0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025c128; continue; }
                pc = 0x8025c0fc;
            }
            0x8025c0fc => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x28;
                c.lr = 0x8025c10c; f_80237f74(c);
                pc = 0x8025c10c;
            }
            0x8025c10c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025c128; continue; }
                pc = 0x8025c118;
            }
            0x8025c118 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0xc;
                c.lr = 0x8025c128; f_80237f74(c);
                pc = 0x8025c128;
            }
            0x8025c128 => {
                c.r[0] = 0x1;
                pc = 0x8025c12c;
            }
            0x8025c12c => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025c13c; continue; }
                pc = 0x8025c134;
            }
            0x8025c134 => {
                c.r[3] = 0x0;
                pc = 0x8025c198; continue;
                pc = 0x8025c13c;
            }
            0x8025c13c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = 0x0;
                c.lr = 0x8025c154; f_8025c264(c);
                pc = 0x8025c154;
            }
            0x8025c154 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x2;
                c.lr = 0x8025c164; f_801f025c(c);
                pc = 0x8025c164;
            }
            0x8025c164 => {
                c.r[4] = 0x4b;
                c.lr = 0x8025c16c; f_801f6e98(c);
                pc = 0x8025c16c;
            }
            0x8025c16c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c180; continue; }
                pc = 0x8025c178;
            }
            0x8025c178 => {
                c.r[3] = 0x0;
                pc = 0x8025c198; continue;
                pc = 0x8025c180;
            }
            0x8025c180 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025c190; continue; }
                pc = 0x8025c188;
            }
            0x8025c188 => {
                c.r[3] = 0x0;
                pc = 0x8025c198; continue;
                pc = 0x8025c190;
            }
            0x8025c190 => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025c198;
            }
            0x8025c198 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025bfac: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHit000 (0x8025c1ac, 176 bytes)
#[inline(never)]
pub fn f_8025c1ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025c1ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8025c1ac); f_8025c1ac_b(c, 0x8025c1ac); crate::prof::exit(c, 0x8025c1ac); return; }
    }
    f_8025c1ac_b(c, 0x8025c1ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025c1ac_at(c: &mut Cpu, pc: u32) { f_8025c1ac_b(c, pc) }
#[inline(always)]
fn f_8025c1ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025c1ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025c1ac, pc); }
        match pc {
            0x8025c1ac => {
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
                c.lr = 0x8025c1d8; f_802395c8(c);
                pc = 0x8025c1d8;
            }
            0x8025c1d8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x0;
                c.lr = 0x8025c1f8; f_8025c264(c);
                pc = 0x8025c1f8;
            }
            0x8025c1f8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025c20c; f_80239500(c);
                pc = 0x8025c20c;
            }
            0x8025c20c => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8025c220; f_8023793c(c);
                pc = 0x8025c220;
            }
            0x8025c220 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x43);
                if !c.crbit(2) { pc = 0x8025c230; continue; }
                pc = 0x8025c22c;
            }
            0x8025c22c => {
                c.r[31] = 0x0;
                pc = 0x8025c230;
            }
            0x8025c230 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025c240; continue; }
                pc = 0x8025c238;
            }
            0x8025c238 => {
                c.r[3] = 0x0;
                pc = 0x8025c248; continue;
                pc = 0x8025c240;
            }
            0x8025c240 => {
                c.cmps(0, c.r[31], 0xffffffff);
                c.r[3] = 0x1;
                pc = 0x8025c248;
            }
            0x8025c248 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025c1ac: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiWazaHitNull (0x8025c25c, 8 bytes)
#[inline(never)]
pub fn f_8025c25c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025c25c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025c25c); f_8025c25c_b(c, 0x8025c25c); crate::prof::exit(c, 0x8025c25c); return; }
    }
    f_8025c25c_b(c, 0x8025c25c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025c25c_at(c: &mut Cpu, pc: u32) { f_8025c25c_b(c, pc) }
#[inline(always)]
fn f_8025c25c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025c25c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025c25c, pc); }
        match pc {
            0x8025c25c => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8025c25c: bad pc {:#x}", pc),
        }
    }
}

/// _fightTrainerAiWazaHitCheck (0x8025c264, 832 bytes)
#[inline(never)]
pub fn f_8025c264(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025c264) { return; }
        if c.prof { crate::prof::enter(c, 0x8025c264); f_8025c264_b(c, 0x8025c264); crate::prof::exit(c, 0x8025c264); return; }
    }
    f_8025c264_b(c, 0x8025c264)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025c264_at(c: &mut Cpu, pc: u32) { f_8025c264_b(c, pc) }
#[inline(always)]
fn f_8025c264_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025c264;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025c264, pc); }
        match pc {
            0x8025c264 => {
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[3] | c.r[3];
                c.r[26] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[28] = c.r[6] | c.r[6];
                c.r[29] = c.r[7] | c.r[7];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x8025c29c; f_801f54a4(c);
                pc = 0x8025c29c;
            }
            0x8025c29c => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8025c2a8; f_801f0134(c);
                pc = 0x8025c2a8;
            }
            0x8025c2a8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.lr = 0x8025c2bc; f_80235b04(c);
                pc = 0x8025c2bc;
            }
            0x8025c2bc => {
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.lr = 0x8025c2d0; f_8011beb4(c);
                pc = 0x8025c2d0;
            }
            0x8025c2d0 => {
                c.cmpu(0, c.r[28], 0x0);
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                if !c.crbit(2) { pc = 0x8025c2e4; continue; }
                pc = 0x8025c2dc;
            }
            0x8025c2dc => {
                c.r[3] = 0x1;
                pc = 0x8025c590; continue;
                pc = 0x8025c2e4;
            }
            0x8025c2e4 => {
                c.r[7] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x8025c2fc; f_801f1d5c(c);
                pc = 0x8025c2fc;
            }
            0x8025c2fc => {
                c.r[24] = c.r[3] | c.r[3];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x0;
                c.lr = 0x8025c314; f_801f3bb4(c);
                pc = 0x8025c314;
            }
            0x8025c314 => {
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[5] = 0x0;
                pc = 0x8025c358; continue;
                pc = 0x8025c32c;
            }
            0x8025c32c => {
                c.r[3] = c.r[5].rotate_left(2) & 0x3fffc;
                c.r[3] = c.lwz(c.r[4].wrapping_add(c.r[3]));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025c354; continue; }
                pc = 0x8025c33c;
            }
            0x8025c33c => {
                c.cmpu(0, c.r[26], c.r[3]);
                if !c.crbit(2) { pc = 0x8025c348; continue; }
                pc = 0x8025c344;
            }
            0x8025c344 => {
                c.r[6] = c.r[5] | c.r[5];
                pc = 0x8025c348;
            }
            0x8025c348 => {
                c.cmpu(0, c.r[28], c.r[3]);
                if !c.crbit(2) { pc = 0x8025c354; continue; }
                pc = 0x8025c350;
            }
            0x8025c350 => {
                c.r[7] = c.r[5] | c.r[5];
                pc = 0x8025c354;
            }
            0x8025c354 => {
                c.r[5] = c.r[5].wrapping_add(0x1);
                pc = 0x8025c358;
            }
            0x8025c358 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8025c32c; continue; }
                pc = 0x8025c364;
            }
            0x8025c364 => {
                c.cmpu(0, c.r[29], 0xffff);
                c.r[3] = c.r[6].rotate_left(0) & 0xffff;
                c.r[0] = c.r[7].rotate_left(0) & 0xffff;
                c.r[0] = c.r[3].wrapping_sub(c.r[0]);
                c.r[24] = c.r[0].rotate_left(1) & 0x1;
                if c.crbit(2) { pc = 0x8025c384; continue; }
                pc = 0x8025c37c;
            }
            0x8025c37c => {
                c.cmpu(0, c.r[29], 0xfffe);
                if !c.crbit(2) { pc = 0x8025c420; continue; }
                pc = 0x8025c384;
            }
            0x8025c384 => {
                c.cmpu(0, c.r[29], 0xffff);
                if !c.crbit(2) { pc = 0x8025c3cc; continue; }
                pc = 0x8025c38c;
            }
            0x8025c38c => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1d;
                c.lr = 0x8025c39c; f_80236bfc(c);
                pc = 0x8025c39c;
            }
            0x8025c39c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c3cc; continue; }
                pc = 0x8025c3a8;
            }
            0x8025c3a8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x1d;
                c.lr = 0x8025c3b4; f_80201d84(c);
                pc = 0x8025c3b4;
            }
            0x8025c3b4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8025c3cc; continue; }
                pc = 0x8025c3c4;
            }
            0x8025c3c4 => {
                c.r[3] = 0xffffffff;
                pc = 0x8025c590; continue;
                pc = 0x8025c3cc;
            }
            0x8025c3cc => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x8025c3d8; f_80237288(c);
                pc = 0x8025c3d8;
            }
            0x8025c3d8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c3f4; continue; }
                pc = 0x8025c3e4;
            }
            0x8025c3e4 => {
                c.cmpu(0, c.r[24], 0x1);
                if !c.crbit(2) { pc = 0x8025c3f4; continue; }
                pc = 0x8025c3ec;
            }
            0x8025c3ec => {
                c.r[3] = 0x0;
                pc = 0x8025c590; continue;
                pc = 0x8025c3f4;
            }
            0x8025c3f4 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[27] | c.r[27];
                c.lr = 0x8025c404; f_8025caa8(c);
                pc = 0x8025c404;
            }
            0x8025c404 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c418; continue; }
                pc = 0x8025c410;
            }
            0x8025c410 => {
                c.r[3] = 0x0;
                pc = 0x8025c590; continue;
                pc = 0x8025c418;
            }
            0x8025c418 => {
                c.r[3] = 0x1;
                pc = 0x8025c590; continue;
                pc = 0x8025c420;
            }
            0x8025c420 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[28] | c.r[28];
                c.lr = 0x8025c430; f_80229934(c);
                pc = 0x8025c430;
            }
            0x8025c430 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c444; continue; }
                pc = 0x8025c43c;
            }
            0x8025c43c => {
                c.r[3] = 0x0;
                pc = 0x8025c590; continue;
                pc = 0x8025c444;
            }
            0x8025c444 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[27] | c.r[27];
                c.lr = 0x8025c454; f_8025caa8(c);
                pc = 0x8025c454;
            }
            0x8025c454 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c468; continue; }
                pc = 0x8025c460;
            }
            0x8025c460 => {
                c.r[3] = 0x0;
                pc = 0x8025c590; continue;
                pc = 0x8025c468;
            }
            0x8025c468 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1d;
                c.lr = 0x8025c478; f_80236bfc(c);
                pc = 0x8025c478;
            }
            0x8025c478 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c4a8; continue; }
                pc = 0x8025c484;
            }
            0x8025c484 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x1d;
                c.lr = 0x8025c490; f_80201d84(c);
                pc = 0x8025c490;
            }
            0x8025c490 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x8025c4a8; continue; }
                pc = 0x8025c4a0;
            }
            0x8025c4a0 => {
                c.r[3] = 0xffffffff;
                pc = 0x8025c590; continue;
                pc = 0x8025c4a8;
            }
            0x8025c4a8 => {
                c.cmpu(0, c.r[24], 0x1);
                if !c.crbit(2) { pc = 0x8025c558; continue; }
                pc = 0x8025c4b0;
            }
            0x8025c4b0 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1f;
                c.lr = 0x8025c4c0; f_80236bfc(c);
                pc = 0x8025c4c0;
            }
            0x8025c4c0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c4f4; continue; }
                pc = 0x8025c4cc;
            }
            0x8025c4cc => {
                c.cmpu(0, c.r[30], 0x92);
                if c.crbit(2) { pc = 0x8025c4f4; continue; }
                pc = 0x8025c4d4;
            }
            0x8025c4d4 => {
                c.cmpu(0, c.r[30], 0x95);
                if c.crbit(2) { pc = 0x8025c4f4; continue; }
                pc = 0x8025c4dc;
            }
            0x8025c4dc => {
                c.cmpu(0, c.r[30], 0x98);
                if c.crbit(2) { pc = 0x8025c4f4; continue; }
                pc = 0x8025c4e4;
            }
            0x8025c4e4 => {
                c.cmpu(0, c.r[30], 0xcf);
                if c.crbit(2) { pc = 0x8025c4f4; continue; }
                pc = 0x8025c4ec;
            }
            0x8025c4ec => {
                c.r[3] = 0x0;
                pc = 0x8025c590; continue;
                pc = 0x8025c4f4;
            }
            0x8025c4f4 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x20;
                c.lr = 0x8025c504; f_80236bfc(c);
                pc = 0x8025c504;
            }
            0x8025c504 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c520; continue; }
                pc = 0x8025c510;
            }
            0x8025c510 => {
                c.cmpu(0, c.r[30], 0x93);
                if c.crbit(2) { pc = 0x8025c520; continue; }
                pc = 0x8025c518;
            }
            0x8025c518 => {
                c.r[3] = 0x0;
                pc = 0x8025c590; continue;
                pc = 0x8025c520;
            }
            0x8025c520 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x21;
                c.lr = 0x8025c530; f_80236bfc(c);
                pc = 0x8025c530;
            }
            0x8025c530 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c558; continue; }
                pc = 0x8025c53c;
            }
            0x8025c53c => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x39);
                if c.crbit(2) { pc = 0x8025c558; continue; }
                pc = 0x8025c548;
            }
            0x8025c548 => {
                c.cmpu(0, c.r[0], 0xfa);
                if c.crbit(2) { pc = 0x8025c558; continue; }
                pc = 0x8025c550;
            }
            0x8025c550 => {
                c.r[3] = 0x0;
                pc = 0x8025c590; continue;
                pc = 0x8025c558;
            }
            0x8025c558 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8025c560; f_80229b70(c);
                pc = 0x8025c560;
            }
            0x8025c560 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c574; continue; }
                pc = 0x8025c56c;
            }
            0x8025c56c => {
                c.r[3] = 0xffffffff;
                pc = 0x8025c590; continue;
                pc = 0x8025c574;
            }
            0x8025c574 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8025c57c; f_80229bd8(c);
                pc = 0x8025c57c;
            }
            0x8025c57c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c590; continue; }
                pc = 0x8025c58c;
            }
            0x8025c58c => {
                c.r[3] = 0xffffffff;
                pc = 0x8025c590;
            }
            0x8025c590 => {
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { return; }
                return;
            }
            _ => panic!("f_8025c264: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiCheckJoutaiKieWazaHitWazaDataId (0x8025c5a4, 208 bytes)
#[inline(never)]
pub fn f_8025c5a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025c5a4) { return; }
        if c.prof { crate::prof::enter(c, 0x8025c5a4); f_8025c5a4_b(c, 0x8025c5a4); crate::prof::exit(c, 0x8025c5a4); return; }
    }
    f_8025c5a4_b(c, 0x8025c5a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025c5a4_at(c: &mut Cpu, pc: u32) { f_8025c5a4_b(c, pc) }
#[inline(always)]
fn f_8025c5a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025c5a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025c5a4, pc); }
        match pc {
            0x8025c5a4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[5] | c.r[5];
                c.r[30] = c.r[4] | c.r[4];
                c.r[5] = 0x9;
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8025c5d0; f_8011beb4(c);
                pc = 0x8025c5d0;
            }
            0x8025c5d0 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1f);
                if !c.crbit(2) { pc = 0x8025c608; continue; }
                pc = 0x8025c5e0;
            }
            0x8025c5e0 => {
                c.cmpu(0, c.r[3], 0x92);
                if c.crbit(2) { pc = 0x8025c600; continue; }
                pc = 0x8025c5e8;
            }
            0x8025c5e8 => {
                c.cmpu(0, c.r[3], 0x95);
                if c.crbit(2) { pc = 0x8025c600; continue; }
                pc = 0x8025c5f0;
            }
            0x8025c5f0 => {
                c.cmpu(0, c.r[3], 0x98);
                if c.crbit(2) { pc = 0x8025c600; continue; }
                pc = 0x8025c5f8;
            }
            0x8025c5f8 => {
                c.cmpu(0, c.r[3], 0xcf);
                if !c.crbit(2) { pc = 0x8025c608; continue; }
                pc = 0x8025c600;
            }
            0x8025c600 => {
                c.r[3] = 0x1;
                pc = 0x8025c660; continue;
                pc = 0x8025c608;
            }
            0x8025c608 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x20);
                if !c.crbit(2) { pc = 0x8025c624; continue; }
                pc = 0x8025c614;
            }
            0x8025c614 => {
                c.cmpu(0, c.r[3], 0x93);
                if !c.crbit(2) { pc = 0x8025c624; continue; }
                pc = 0x8025c61c;
            }
            0x8025c61c => {
                c.r[3] = 0x1;
                pc = 0x8025c660; continue;
                pc = 0x8025c624;
            }
            0x8025c624 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x21);
                if !c.crbit(2) { pc = 0x8025c64c; continue; }
                pc = 0x8025c630;
            }
            0x8025c630 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x39);
                if c.crbit(2) { pc = 0x8025c644; continue; }
                pc = 0x8025c63c;
            }
            0x8025c63c => {
                c.cmpu(0, c.r[0], 0xfa);
                if !c.crbit(2) { pc = 0x8025c64c; continue; }
                pc = 0x8025c644;
            }
            0x8025c644 => {
                c.r[3] = 0x1;
                pc = 0x8025c660; continue;
                pc = 0x8025c64c;
            }
            0x8025c64c => {
                c.cmpu(0, c.r[3], 0x5e);
                if !c.crbit(2) { pc = 0x8025c65c; continue; }
                pc = 0x8025c654;
            }
            0x8025c654 => {
                c.r[3] = 0x1;
                pc = 0x8025c660; continue;
                pc = 0x8025c65c;
            }
            0x8025c65c => {
                c.r[3] = 0x0;
                pc = 0x8025c660;
            }
            0x8025c660 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025c5a4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiCheckHorobinouta (0x8025c674, 72 bytes)
#[inline(never)]
pub fn f_8025c674(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025c674) { return; }
        if c.prof { crate::prof::enter(c, 0x8025c674); f_8025c674_b(c, 0x8025c674); crate::prof::exit(c, 0x8025c674); return; }
    }
    f_8025c674_b(c, 0x8025c674)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025c674_at(c: &mut Cpu, pc: u32) { f_8025c674_b(c, pc) }
#[inline(always)]
fn f_8025c674_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025c674;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025c674, pc); }
        match pc {
            0x8025c674 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80260000;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0xffffc6bc);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x8), c.r[3]);
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.lr = 0x8025c6a4; f_801f37b0(c);
                pc = 0x8025c6a4;
            }
            0x8025c6a4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025c674: bad pc {:#x}", pc),
        }
    }
}

/// _fightTrainerAiCheckHorobinoutaSub (0x8025c6bc, 180 bytes)
#[inline(never)]
pub fn f_8025c6bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025c6bc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025c6bc); f_8025c6bc_b(c, 0x8025c6bc); crate::prof::exit(c, 0x8025c6bc); return; }
    }
    f_8025c6bc_b(c, 0x8025c6bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025c6bc_at(c: &mut Cpu, pc: u32) { f_8025c6bc_b(c, pc) }
#[inline(always)]
fn f_8025c6bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025c6bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025c6bc, pc); }
        match pc {
            0x8025c6bc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[4] | c.r[4];
                c.r[31] = c.r[3] | c.r[3];
                c.r[30] = c.lwz(c.r[5]);
                c.lr = 0x8025c6e0; f_802062fc(c);
                pc = 0x8025c6e0;
            }
            0x8025c6e0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025c6f0; continue; }
                pc = 0x8025c6e8;
            }
            0x8025c6e8 => {
                c.r[3] = 0x1;
                pc = 0x8025c75c; continue;
                pc = 0x8025c6f0;
            }
            0x8025c6f0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.lr = 0x8025c700; f_801f8424(c);
                pc = 0x8025c700;
            }
            0x8025c700 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c714; continue; }
                pc = 0x8025c70c;
            }
            0x8025c70c => {
                c.r[3] = 0x1;
                pc = 0x8025c75c; continue;
                pc = 0x8025c714;
            }
            0x8025c714 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x1e;
                c.lr = 0x8025c724; f_80236bfc(c);
                pc = 0x8025c724;
            }
            0x8025c724 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025c758; continue; }
                pc = 0x8025c730;
            }
            0x8025c730 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x2b;
                c.lr = 0x8025c740; f_80237f74(c);
                pc = 0x8025c740;
            }
            0x8025c740 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025c758; continue; }
                pc = 0x8025c74c;
            }
            0x8025c74c => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x4));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[29].wrapping_add(0x4), c.r[0]);
                pc = 0x8025c758;
            }
            0x8025c758 => {
                c.r[3] = 0x1;
                pc = 0x8025c75c;
            }
            0x8025c75c => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025c6bc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiCheckTextureZokusei (0x8025c770, 152 bytes)
#[inline(never)]
pub fn f_8025c770(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025c770) { return; }
        if c.prof { crate::prof::enter(c, 0x8025c770); f_8025c770_b(c, 0x8025c770); crate::prof::exit(c, 0x8025c770); return; }
    }
    f_8025c770_b(c, 0x8025c770)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025c770_at(c: &mut Cpu, pc: u32) { f_8025c770_b(c, pc) }
#[inline(always)]
fn f_8025c770_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025c770;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025c770, pc); }
        match pc {
            0x8025c770 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.lr = 0x8025c794; f_801fb1c0(c);
                pc = 0x8025c794;
            }
            0x8025c794 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8025c7a8; f_801fb1c0(c);
                pc = 0x8025c7a8;
            }
            0x8025c7a8 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2a;
                c.r[6] = 0x0;
                c.lr = 0x8025c7bc; f_801fb1c0(c);
                pc = 0x8025c7bc;
            }
            0x8025c7bc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c7f0; continue; }
                pc = 0x8025c7c8;
            }
            0x8025c7c8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8025c7d4; f_8021b364(c);
                pc = 0x8025c7d4;
            }
            0x8025c7d4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = 0x1;
                c.r[0] = c.addc(!c.r[0], c.r[3], 1);
                c.r[0] = 0xffffffff;
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[0], 0, ci); }
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x8025c7f4; continue;
                pc = 0x8025c7f0;
            }
            0x8025c7f0 => {
                c.r[3] = 0x1;
                pc = 0x8025c7f4;
            }
            0x8025c7f4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025c770: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiCheckAbiCnt (0x8025c808, 672 bytes)
#[inline(never)]
pub fn f_8025c808(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025c808) { return; }
        if c.prof { crate::prof::enter(c, 0x8025c808); f_8025c808_b(c, 0x8025c808); crate::prof::exit(c, 0x8025c808); return; }
    }
    f_8025c808_b(c, 0x8025c808)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025c808_at(c: &mut Cpu, pc: u32) { f_8025c808_b(c, pc) }
#[inline(always)]
fn f_8025c808_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025c808;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025c808, pc); }
        match pc {
            0x8025c808 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.r[0] = c.r[9].rotate_left(0) & 0x40;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 21..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[21] = c.r[9] | c.r[9];
                c.r[22] = c.r[3] | c.r[3];
                c.r[23] = c.r[5] | c.r[5];
                c.r[24] = c.r[6] | c.r[6];
                c.r[25] = c.r[7] | c.r[7];
                c.r[26] = c.r[8] | c.r[8];
                c.r[28] = 0x0;
                c.r[27] = 0x0;
                if c.crbit(2) { pc = 0x8025c848; continue; }
                pc = 0x8025c840;
            }
            0x8025c840 => {
                c.r[30] = c.r[4] | c.r[4];
                pc = 0x8025c84c; continue;
                pc = 0x8025c848;
            }
            0x8025c848 => {
                c.r[30] = c.r[23] | c.r[23];
                pc = 0x8025c84c;
            }
            0x8025c84c => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x2;
                c.lr = 0x8025c858; f_801f025c(c);
                pc = 0x8025c858;
            }
            0x8025c858 => {
                c.r[31] = c.r[21].rotate_left(0) & 0xff;
                c.r[21] = c.r[3] | c.r[3];
                c.r[0] = c.r[31] & 0xbf;
                c.rc(c.r[0]);
                c.r[0] = c.r[0].rotate_left(0) & 0x80;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025c870; continue; }
                pc = 0x8025c86c;
            }
            0x8025c86c => {
                c.r[28] = 0x1;
                pc = 0x8025c870;
            }
            0x8025c870 => {
                c.r[0] = c.r[31].rotate_left(0) & 0x20;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025c87c; continue; }
                pc = 0x8025c878;
            }
            0x8025c878 => {
                c.r[27] = 0x1;
                pc = 0x8025c87c;
            }
            0x8025c87c => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8025c884; f_8021c090(c);
                pc = 0x8025c884;
            }
            0x8025c884 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = 0x0;
                c.lr = 0x8025c8a0; f_8012640c(c);
                pc = 0x8025c8a0;
            }
            0x8025c8a0 => {
                c.r[26] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8025c8ac; f_8021c034(c);
                pc = 0x8025c8ac;
            }
            0x8025c8ac => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8025ca80; continue; }
                pc = 0x8025c8b4;
            }
            0x8025c8b4 => {
                c.r[3] = c.r[21] | c.r[21];
                c.r[4] = 0x4c;
                c.lr = 0x8025c8c0; f_801f6e98(c);
                pc = 0x8025c8c0;
            }
            0x8025c8c0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c8e8; continue; }
                pc = 0x8025c8cc;
            }
            0x8025c8cc => {
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025c8e8; continue; }
                pc = 0x8025c8d4;
            }
            0x8025c8d4 => {
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xae);
                if c.crbit(2) { pc = 0x8025c8e8; continue; }
                pc = 0x8025c8e0;
            }
            0x8025c8e0 => {
                c.r[3] = 0x0;
                pc = 0x8025ca94; continue;
                pc = 0x8025c8e8;
            }
            0x8025c8e8 => {
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xae);
                if c.crbit(2) { pc = 0x8025c980; continue; }
                pc = 0x8025c8f4;
            }
            0x8025c8f4 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025c980; continue; }
                pc = 0x8025c900;
            }
            0x8025c900 => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x8025c914; f_801fb1c0(c);
                pc = 0x8025c914;
            }
            0x8025c914 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8025c928; f_801fb1c0(c);
                pc = 0x8025c928;
            }
            0x8025c928 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x24;
                c.r[6] = 0x0;
                c.lr = 0x8025c93c; f_801fb1c0(c);
                pc = 0x8025c93c;
            }
            0x8025c93c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c968; continue; }
                pc = 0x8025c948;
            }
            0x8025c948 => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[24] | c.r[24];
                c.lr = 0x8025c954; f_80229c28(c);
                pc = 0x8025c954;
            }
            0x8025c954 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c968; continue; }
                pc = 0x8025c960;
            }
            0x8025c960 => {
                c.r[0] = 0x1;
                pc = 0x8025c96c; continue;
                pc = 0x8025c968;
            }
            0x8025c968 => {
                c.r[0] = 0x0;
                pc = 0x8025c96c;
            }
            0x8025c96c => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c980; continue; }
                pc = 0x8025c978;
            }
            0x8025c978 => {
                c.r[3] = 0x0;
                pc = 0x8025ca94; continue;
                pc = 0x8025c980;
            }
            0x8025c980 => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x1d;
                c.lr = 0x8025c990; f_80237f74(c);
                pc = 0x8025c990;
            }
            0x8025c990 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025c9b8; continue; }
                pc = 0x8025c99c;
            }
            0x8025c99c => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x49;
                c.lr = 0x8025c9ac; f_80237f74(c);
                pc = 0x8025c9ac;
            }
            0x8025c9ac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025c9d4; continue; }
                pc = 0x8025c9b8;
            }
            0x8025c9b8 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025c9d4; continue; }
                pc = 0x8025c9c0;
            }
            0x8025c9c0 => {
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xae);
                if c.crbit(2) { pc = 0x8025c9d4; continue; }
                pc = 0x8025c9cc;
            }
            0x8025c9cc => {
                c.r[3] = 0x0;
                pc = 0x8025ca94; continue;
                pc = 0x8025c9d4;
            }
            0x8025c9d4 => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x33;
                c.lr = 0x8025c9e4; f_80237f74(c);
                pc = 0x8025c9e4;
            }
            0x8025c9e4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025ca0c; continue; }
                pc = 0x8025c9f0;
            }
            0x8025c9f0 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025ca0c; continue; }
                pc = 0x8025c9f8;
            }
            0x8025c9f8 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xeb);
                if !c.crbit(2) { pc = 0x8025ca0c; continue; }
                pc = 0x8025ca04;
            }
            0x8025ca04 => {
                c.r[3] = 0x0;
                pc = 0x8025ca94; continue;
                pc = 0x8025ca0c;
            }
            0x8025ca0c => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x34;
                c.lr = 0x8025ca1c; f_80237f74(c);
                pc = 0x8025ca1c;
            }
            0x8025ca1c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025ca44; continue; }
                pc = 0x8025ca28;
            }
            0x8025ca28 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025ca44; continue; }
                pc = 0x8025ca30;
            }
            0x8025ca30 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xe6);
                if !c.crbit(2) { pc = 0x8025ca44; continue; }
                pc = 0x8025ca3c;
            }
            0x8025ca3c => {
                c.r[3] = 0x0;
                pc = 0x8025ca94; continue;
                pc = 0x8025ca44;
            }
            0x8025ca44 => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x13;
                c.lr = 0x8025ca54; f_80237f74(c);
                pc = 0x8025ca54;
            }
            0x8025ca54 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025ca70; continue; }
                pc = 0x8025ca60;
            }
            0x8025ca60 => {
                c.r[0] = c.r[31].rotate_left(0) & 0x1f;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025ca70; continue; }
                pc = 0x8025ca68;
            }
            0x8025ca68 => {
                c.r[3] = 0x0;
                pc = 0x8025ca94; continue;
                pc = 0x8025ca70;
            }
            0x8025ca70 => {
                c.r[0] = c.r[26] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(1) { pc = 0x8025ca90; continue; }
                pc = 0x8025ca78;
            }
            0x8025ca78 => {
                c.r[3] = 0x0;
                pc = 0x8025ca94; continue;
                pc = 0x8025ca80;
            }
            0x8025ca80 => {
                c.cmps(0, c.r[26], 0xc);
                if c.crbit(0) { pc = 0x8025ca90; continue; }
                pc = 0x8025ca88;
            }
            0x8025ca88 => {
                c.r[3] = 0x0;
                pc = 0x8025ca94; continue;
                pc = 0x8025ca90;
            }
            0x8025ca90 => {
                c.r[3] = 0x1;
                pc = 0x8025ca94;
            }
            0x8025ca94 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 21..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_8025c808: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiCheckGuard (0x8025caa8, 148 bytes)
#[inline(never)]
pub fn f_8025caa8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025caa8) { return; }
        if c.prof { crate::prof::enter(c, 0x8025caa8); f_8025caa8_b(c, 0x8025caa8); crate::prof::exit(c, 0x8025caa8); return; }
    }
    f_8025caa8_b(c, 0x8025caa8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025caa8_at(c: &mut Cpu, pc: u32) { f_8025caa8_b(c, pc) }
#[inline(always)]
fn f_8025caa8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025caa8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025caa8, pc); }
        match pc {
            0x8025caa8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[31] = c.r[5] | c.r[5];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.lr = 0x8025cad0; f_801fb1c0(c);
                pc = 0x8025cad0;
            }
            0x8025cad0 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8025cae4; f_801fb1c0(c);
                pc = 0x8025cae4;
            }
            0x8025cae4 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x24;
                c.r[6] = 0x0;
                c.lr = 0x8025caf8; f_801fb1c0(c);
                pc = 0x8025caf8;
            }
            0x8025caf8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025cb24; continue; }
                pc = 0x8025cb04;
            }
            0x8025cb04 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8025cb10; f_80229c28(c);
                pc = 0x8025cb10;
            }
            0x8025cb10 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025cb24; continue; }
                pc = 0x8025cb1c;
            }
            0x8025cb1c => {
                c.r[3] = 0x1;
                pc = 0x8025cb28; continue;
                pc = 0x8025cb24;
            }
            0x8025cb24 => {
                c.r[3] = 0x0;
                pc = 0x8025cb28;
            }
            0x8025cb28 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025caa8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiCheckOumu (0x8025cb3c, 172 bytes)
#[inline(never)]
pub fn f_8025cb3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025cb3c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025cb3c); f_8025cb3c_b(c, 0x8025cb3c); crate::prof::exit(c, 0x8025cb3c); return; }
    }
    f_8025cb3c_b(c, 0x8025cb3c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025cb3c_at(c: &mut Cpu, pc: u32) { f_8025cb3c_b(c, pc) }
#[inline(always)]
fn f_8025cb3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025cb3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025cb3c, pc); }
        match pc {
            0x8025cb3c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0xf7;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025cb64; f_8012640c(c);
                pc = 0x8025cb64;
            }
            0x8025cb64 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8025cb80; continue; }
                pc = 0x8025cb6c;
            }
            0x8025cb6c => {
                c.cmpu(0, c.r[3], 0x165);
                if c.crbit(2) { pc = 0x8025cb80; continue; }
                pc = 0x8025cb74;
            }
            0x8025cb74 => {
                c.cmpu(0, c.r[3], 0xffff);
                if c.crbit(2) { pc = 0x8025cb80; continue; }
                pc = 0x8025cb7c;
            }
            0x8025cb7c => {
                pc = 0x8025cbd4; continue;
                pc = 0x8025cb80;
            }
            0x8025cb80 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8025cb8c; f_80201248(c);
                pc = 0x8025cb8c;
            }
            0x8025cb8c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.r[31] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x8025cbd0; continue; }
                pc = 0x8025cb98;
            }
            0x8025cb98 => {
                c.lr = 0x8025cb9c; f_800e0c54(c);
                pc = 0x8025cb9c;
            }
            0x8025cb9c => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                c.r[0] = divw(c.r[5], c.r[4]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.r[0].wrapping_mul(c.r[4]);
                c.r[0] = c.r[5].wrapping_sub(c.r[0]);
                c.r[0] = c.r[0].rotate_left(1) & 0x1fe;
                c.r[3] = c.lhz(c.r[3].wrapping_add(c.r[0]));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025cbd0; continue; }
                pc = 0x8025cbc4;
            }
            0x8025cbc4 => {
                c.cmpu(0, c.r[3], 0x165);
                if c.crbit(2) { pc = 0x8025cbd0; continue; }
                pc = 0x8025cbcc;
            }
            0x8025cbcc => {
                pc = 0x8025cbd4; continue;
                pc = 0x8025cbd0;
            }
            0x8025cbd0 => {
                c.r[3] = 0x0;
                pc = 0x8025cbd4;
            }
            0x8025cbd4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025cb3c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiCheckSimerike (0x8025cbe8, 72 bytes)
#[inline(never)]
pub fn f_8025cbe8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025cbe8) { return; }
        if c.prof { crate::prof::enter(c, 0x8025cbe8); f_8025cbe8_b(c, 0x8025cbe8); crate::prof::exit(c, 0x8025cbe8); return; }
    }
    f_8025cbe8_b(c, 0x8025cbe8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025cbe8_at(c: &mut Cpu, pc: u32) { f_8025cbe8_b(c, pc) }
#[inline(always)]
fn f_8025cbe8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025cbe8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025cbe8, pc); }
        match pc {
            0x8025cbe8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80260000;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0xffffcc30);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x8), c.r[3]);
                c.r[3] = 0x0;
                c.lr = 0x8025cc10; f_801f37b0(c);
                pc = 0x8025cc10;
            }
            0x8025cc10 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025cbe8: bad pc {:#x}", pc),
        }
    }
}

/// _fightTrainerAiSimerikeCheckSub (0x8025cc30, 96 bytes)
#[inline(never)]
pub fn f_8025cc30(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025cc30) { return; }
        if c.prof { crate::prof::enter(c, 0x8025cc30); f_8025cc30_b(c, 0x8025cc30); crate::prof::exit(c, 0x8025cc30); return; }
    }
    f_8025cc30_b(c, 0x8025cc30)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025cc30_at(c: &mut Cpu, pc: u32) { f_8025cc30_b(c, pc) }
#[inline(always)]
fn f_8025cc30_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025cc30;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025cc30, pc); }
        match pc {
            0x8025cc30 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.lwz(c.r[5]);
                c.lr = 0x8025cc4c; f_802062fc(c);
                pc = 0x8025cc4c;
            }
            0x8025cc4c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025cc5c; continue; }
                pc = 0x8025cc54;
            }
            0x8025cc54 => {
                c.r[3] = 0x1;
                pc = 0x8025cc7c; continue;
                pc = 0x8025cc5c;
            }
            0x8025cc5c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x6;
                c.lr = 0x8025cc6c; f_80237f74(c);
                pc = 0x8025cc6c;
            }
            0x8025cc6c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                pc = 0x8025cc7c;
            }
            0x8025cc7c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025cc30: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiCheckSawagu (0x8025cc90, 80 bytes)
#[inline(never)]
pub fn f_8025cc90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025cc90) { return; }
        if c.prof { crate::prof::enter(c, 0x8025cc90); f_8025cc90_b(c, 0x8025cc90); crate::prof::exit(c, 0x8025cc90); return; }
    }
    f_8025cc90_b(c, 0x8025cc90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025cc90_at(c: &mut Cpu, pc: u32) { f_8025cc90_b(c, pc) }
#[inline(always)]
fn f_8025cc90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025cc90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025cc90, pc); }
        match pc {
            0x8025cc90 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x80260000;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[5].wrapping_add(0xffffcce0);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x8), c.r[3]);
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[4]);
                c.r[4] = c.r[0] | c.r[0];
                c.lr = 0x8025ccc0; f_801f37b0(c);
                pc = 0x8025ccc0;
            }
            0x8025ccc0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025cc90: bad pc {:#x}", pc),
        }
    }
}

/// _fightTrainerAiSawaguCheckSub (0x8025cce0, 132 bytes)
#[inline(never)]
pub fn f_8025cce0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025cce0) { return; }
        if c.prof { crate::prof::enter(c, 0x8025cce0); f_8025cce0_b(c, 0x8025cce0); crate::prof::exit(c, 0x8025cce0); return; }
    }
    f_8025cce0_b(c, 0x8025cce0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025cce0_at(c: &mut Cpu, pc: u32) { f_8025cce0_b(c, pc) }
#[inline(always)]
fn f_8025cce0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025cce0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025cce0, pc); }
        match pc {
            0x8025cce0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.lwz(c.r[5]);
                c.r[30] = c.lwz(c.r[5].wrapping_add(0x4));
                c.lr = 0x8025cd00; f_802062fc(c);
                pc = 0x8025cd00;
            }
            0x8025cd00 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025cd10; continue; }
                pc = 0x8025cd08;
            }
            0x8025cd08 => {
                c.r[3] = 0x1;
                pc = 0x8025cd50; continue;
                pc = 0x8025cd10;
            }
            0x8025cd10 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0xb;
                c.lr = 0x8025cd20; f_80236bfc(c);
                pc = 0x8025cd20;
            }
            0x8025cd20 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025cd4c; continue; }
                pc = 0x8025cd2c;
            }
            0x8025cd2c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x2b;
                c.lr = 0x8025cd3c; f_80237f74(c);
                pc = 0x8025cd3c;
            }
            0x8025cd3c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025cd4c; continue; }
                pc = 0x8025cd44;
            }
            0x8025cd44 => {
                c.r[3] = 0x0;
                pc = 0x8025cd50; continue;
                pc = 0x8025cd4c;
            }
            0x8025cd4c => {
                c.r[3] = 0x1;
                pc = 0x8025cd50;
            }
            0x8025cd50 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025cce0: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenFreePokemonData (0x8025cd64, 84 bytes)
#[inline(never)]
pub fn f_8025cd64(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025cd64) { return; }
        if c.prof { crate::prof::enter(c, 0x8025cd64); f_8025cd64_b(c, 0x8025cd64); crate::prof::exit(c, 0x8025cd64); return; }
    }
    f_8025cd64_b(c, 0x8025cd64)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025cd64_at(c: &mut Cpu, pc: u32) { f_8025cd64_b(c, pc) }
#[inline(always)]
fn f_8025cd64_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025cd64;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025cd64, pc); }
        match pc {
            0x8025cd64 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae30));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025cda4; continue; }
                pc = 0x8025cd80;
            }
            0x8025cd80 => {
                c.lr = 0x8025cd84; f_800e202c(c);
                pc = 0x8025cd84;
            }
            0x8025cd84 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[31] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x8025cd9c; continue; }
                pc = 0x8025cd90;
            }
            0x8025cd90 => {
                c.lr = 0x8025cd94; f_800e24b0(c);
                pc = 0x8025cd94;
            }
            0x8025cd94 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025cd9c; f_800e209c(c);
                pc = 0x8025cd9c;
            }
            0x8025cd9c => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffae30), c.r[0]);
                pc = 0x8025cda4;
            }
            0x8025cda4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025cd64: bad pc {:#x}", pc),
        }
    }
}

/// toolentryDebugPokemonCreate (0x8025cdb8, 692 bytes)
#[inline(never)]
pub fn f_8025cdb8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025cdb8) { return; }
        if c.prof { crate::prof::enter(c, 0x8025cdb8); f_8025cdb8_b(c, 0x8025cdb8); crate::prof::exit(c, 0x8025cdb8); return; }
    }
    f_8025cdb8_b(c, 0x8025cdb8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025cdb8_at(c: &mut Cpu, pc: u32) { f_8025cdb8_b(c, pc) }
#[inline(always)]
fn f_8025cdb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025cdb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025cdb8, pc); }
        match pc {
            0x8025cdb8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x20;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8578));
                c.stw(c.r[13].wrapping_add(0xffffae34), c.r[3]);
                c.r[3] = c.r[0].wrapping_mul(0x138);
                c.r[0] = c.r[3].wrapping_add(0x1f);
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.lr = 0x8025cde8; f_800e2c04(c);
                pc = 0x8025cde8;
            }
            0x8025cde8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025cdf8; continue; }
                pc = 0x8025cdf0;
            }
            0x8025cdf0 => {
                c.lr = 0x8025cdf4; f_800e27b0(c);
                pc = 0x8025cdf4;
            }
            0x8025cdf4 => {
                pc = 0x8025cdfc; continue;
                pc = 0x8025cdf8;
            }
            0x8025cdf8 => {
                c.r[3] = 0x0;
                pc = 0x8025cdfc;
            }
            0x8025cdfc => {
                c.stw(c.r[13].wrapping_add(0xffffae34), c.r[3]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = 0x0;
                pc = 0x8025ce68; continue;
                pc = 0x8025ce0c;
            }
            0x8025ce0c => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.lr = 0x8025ce18; f_80135938(c);
                pc = 0x8025ce18;
            }
            0x8025ce18 => {
                c.r[0] = c.r[31].wrapping_add(0x1);
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = 0xa;
                c.r[4] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x8025ce30; f_801240c4(c);
                pc = 0x8025ce30;
            }
            0x8025ce30 => {
                c.r[3] = c.r[31].wrapping_add(0x1004);
                c.lr = 0x8025ce38; f_800fa280(c);
                pc = 0x8025ce38;
            }
            0x8025ce38 => {
                c.r[9] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x8025ce58; f_80123ef0(c);
                pc = 0x8025ce58;
            }
            0x8025ce58 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8025ce60; f_8012086c(c);
                pc = 0x8025ce60;
            }
            0x8025ce60 => {
                c.r[30] = c.r[30].wrapping_add(0x138);
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x8025ce68;
            }
            0x8025ce68 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8578));
                c.cmps(0, c.r[31], c.r[0]);
                if c.crbit(0) { pc = 0x8025ce0c; continue; }
                pc = 0x8025ce74;
            }
            0x8025ce74 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae30));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025cea4; continue; }
                pc = 0x8025ce80;
            }
            0x8025ce80 => {
                c.lr = 0x8025ce84; f_800e202c(c);
                pc = 0x8025ce84;
            }
            0x8025ce84 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[30] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x8025ce9c; continue; }
                pc = 0x8025ce90;
            }
            0x8025ce90 => {
                c.lr = 0x8025ce94; f_800e24b0(c);
                pc = 0x8025ce94;
            }
            0x8025ce94 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8025ce9c; f_800e209c(c);
                pc = 0x8025ce9c;
            }
            0x8025ce9c => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffae30), c.r[0]);
                pc = 0x8025cea4;
            }
            0x8025cea4 => {
                c.r[3] = 0x80;
                c.r[4] = 0x20;
                c.lr = 0x8025ceb0; f_800e2c04(c);
                pc = 0x8025ceb0;
            }
            0x8025ceb0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025cec0; continue; }
                pc = 0x8025ceb8;
            }
            0x8025ceb8 => {
                c.lr = 0x8025cebc; f_800e27b0(c);
                pc = 0x8025cebc;
            }
            0x8025cebc => {
                pc = 0x8025cec4; continue;
                pc = 0x8025cec0;
            }
            0x8025cec0 => {
                c.r[3] = 0x0;
                pc = 0x8025cec4;
            }
            0x8025cec4 => {
                c.stw(c.r[13].wrapping_add(0xffffae30), c.r[3]);
                c.r[31] = 0x0;
                pc = 0x8025cecc;
            }
            0x8025cecc => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025ced4; f_8006b09c(c);
                pc = 0x8025ced4;
            }
            0x8025ced4 => {
                c.r[3] = c.r[3].wrapping_add(0xb44);
                c.lr = 0x8025cedc; f_8012a248(c);
                pc = 0x8025cedc;
            }
            0x8025cedc => {
                c.r[30] = 0x0;
                pc = 0x8025cee0;
            }
            0x8025cee0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025cee8; f_8006b09c(c);
                pc = 0x8025cee8;
            }
            0x8025cee8 => {
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0xb44);
                c.lr = 0x8025cef4; f_8012ac08(c);
                pc = 0x8025cef4;
            }
            0x8025cef4 => {
                c.lr = 0x8025cef8; f_80124a60(c);
                pc = 0x8025cef8;
            }
            0x8025cef8 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.cmps(0, c.r[30], 0x6);
                if c.crbit(0) { pc = 0x8025cee0; continue; }
                pc = 0x8025cf04;
            }
            0x8025cf04 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                c.cmps(0, c.r[31], 0x4);
                if c.crbit(0) { pc = 0x8025cecc; continue; }
                pc = 0x8025cf10;
            }
            0x8025cf10 => {
                c.r[30] = 0x0;
                pc = 0x8025cf14;
            }
            0x8025cf14 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8025cf1c; f_8006b09c(c);
                pc = 0x8025cf1c;
            }
            0x8025cf1c => {
                c.r[3] = c.r[3].wrapping_add(0x2c);
                c.lr = 0x8025cf24; f_8012a248(c);
                pc = 0x8025cf24;
            }
            0x8025cf24 => {
                c.r[31] = 0x0;
                pc = 0x8025cf28;
            }
            0x8025cf28 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8025cf30; f_8006b09c(c);
                pc = 0x8025cf30;
            }
            0x8025cf30 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0x2c);
                c.lr = 0x8025cf3c; f_8012ac08(c);
                pc = 0x8025cf3c;
            }
            0x8025cf3c => {
                c.lr = 0x8025cf40; f_80124a60(c);
                pc = 0x8025cf40;
            }
            0x8025cf40 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                c.cmps(0, c.r[31], 0x6);
                if c.crbit(0) { pc = 0x8025cf28; continue; }
                pc = 0x8025cf4c;
            }
            0x8025cf4c => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.cmps(0, c.r[30], 0x4);
                if c.crbit(0) { pc = 0x8025cf14; continue; }
                pc = 0x8025cf58;
            }
            0x8025cf58 => {
                c.r[0] = 0x6;
                c.mtspr(9, c.r[0]);
                pc = 0x8025cf60;
            }
            0x8025cf60 => {
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8025cf60; continue; }
                pc = 0x8025cf64;
            }
            0x8025cf64 => {
                c.r[3] = 0x0;
                c.r[4] = 0x2;
                c.lr = 0x8025cf70; f_80129280(c);
                pc = 0x8025cf70;
            }
            0x8025cf70 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8025cf7c; f_8006b09c(c);
                pc = 0x8025cf7c;
            }
            0x8025cf7c => {
                c.cmpu(0, c.r[30], 0x0);
                c.r[3] = c.r[3].wrapping_add(0xb44);
                if c.crbit(2) { pc = 0x8025cf94; continue; }
                pc = 0x8025cf88;
            }
            0x8025cf88 => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0xb18;
                c.lr = 0x8025cf94; f_800054f4(c);
                pc = 0x8025cf94;
            }
            0x8025cf94 => {
                c.r[29] = 0x0;
                c.r[31] = 0x0;
                pc = 0x8025cf9c;
            }
            0x8025cf9c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8025cfd8; continue; }
                pc = 0x8025cfa4;
            }
            0x8025cfa4 => {
                c.r[3] = 0x0;
                c.r[4] = 0x2;
                c.lr = 0x8025cfb0; f_80129280(c);
                pc = 0x8025cfb0;
            }
            0x8025cfb0 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025cfbc; f_8006b09c(c);
                pc = 0x8025cfbc;
            }
            0x8025cfbc => {
                c.cmpu(0, c.r[30], 0x0);
                c.r[3] = c.r[3].wrapping_add(0xb44);
                if c.crbit(2) { pc = 0x8025d01c; continue; }
                pc = 0x8025cfc8;
            }
            0x8025cfc8 => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0xb18;
                c.lr = 0x8025cfd4; f_800054f4(c);
                pc = 0x8025cfd4;
            }
            0x8025cfd4 => {
                pc = 0x8025d01c; continue;
                pc = 0x8025cfd8;
            }
            0x8025cfd8 => {
                c.r[28] = 0x0;
                pc = 0x8025cfdc;
            }
            0x8025cfdc => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffae34));
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0].wrapping_add(c.r[29]);
                c.lr = 0x8025cfec; f_8006b09c(c);
                pc = 0x8025cfec;
            }
            0x8025cfec => {
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0xb44);
                c.lr = 0x8025cff8; f_8012ac08(c);
                pc = 0x8025cff8;
            }
            0x8025cff8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025d00c; continue; }
                pc = 0x8025d000;
            }
            0x8025d000 => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x138;
                c.lr = 0x8025d00c; f_800054f4(c);
                pc = 0x8025d00c;
            }
            0x8025d00c => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.r[29] = c.r[29].wrapping_add(0x138);
                c.cmps(0, c.r[28], 0x6);
                if c.crbit(0) { pc = 0x8025cfdc; continue; }
                pc = 0x8025d01c;
            }
            0x8025d01c => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                c.cmps(0, c.r[31], 0x4);
                if c.crbit(0) { pc = 0x8025cf9c; continue; }
                pc = 0x8025d028;
            }
            0x8025d028 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae34));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025d058; continue; }
                pc = 0x8025d034;
            }
            0x8025d034 => {
                c.lr = 0x8025d038; f_800e202c(c);
                pc = 0x8025d038;
            }
            0x8025d038 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[30] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x8025d050; continue; }
                pc = 0x8025d044;
            }
            0x8025d044 => {
                c.lr = 0x8025d048; f_800e24b0(c);
                pc = 0x8025d048;
            }
            0x8025d048 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8025d050; f_800e209c(c);
                pc = 0x8025d050;
            }
            0x8025d050 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffae34), c.r[0]);
                pc = 0x8025d058;
            }
            0x8025d058 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025cdb8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025D06C (0x8025d06c, 60 bytes)
#[inline(never)]
pub fn f_8025d06c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d06c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d06c); f_8025d06c_b(c, 0x8025d06c); crate::prof::exit(c, 0x8025d06c); return; }
    }
    f_8025d06c_b(c, 0x8025d06c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d06c_at(c: &mut Cpu, pc: u32) { f_8025d06c_b(c, pc) }
#[inline(always)]
fn f_8025d06c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d06c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d06c, pc); }
        match pc {
            0x8025d06c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025d07c; f_8006b5a8(c);
                pc = 0x8025d07c;
            }
            0x8025d07c => {
                c.lr = 0x8025d080; f_8006afc4(c);
                pc = 0x8025d080;
            }
            0x8025d080 => {
                c.lr = 0x8025d084; f_8006adec(c);
                pc = 0x8025d084;
            }
            0x8025d084 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = c.r[0] | c.r[0];
                c.lr = 0x8025d094; f_801293fc(c);
                pc = 0x8025d094;
            }
            0x8025d094 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d06c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025D0A8 (0x8025d0a8, 188 bytes)
#[inline(never)]
pub fn f_8025d0a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d0a8) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d0a8); f_8025d0a8_b(c, 0x8025d0a8); crate::prof::exit(c, 0x8025d0a8); return; }
    }
    f_8025d0a8_b(c, 0x8025d0a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d0a8_at(c: &mut Cpu, pc: u32) { f_8025d0a8_b(c, pc) }
#[inline(always)]
fn f_8025d0a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d0a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d0a8, pc); }
        match pc {
            0x8025d0a8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.rc(c.r[28]);
                c.r[30] = 0x0;
                if !c.crbit(2) { pc = 0x8025d0d4; continue; }
                pc = 0x8025d0c4;
            }
            0x8025d0c4 => {
                c.r[3] = 0x0;
                c.r[4] = 0x2;
                c.lr = 0x8025d0d0; f_80129280(c);
                pc = 0x8025d0d0;
            }
            0x8025d0d0 => {
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8025d0d4;
            }
            0x8025d0d4 => {
                c.r[31] = 0x0;
                pc = 0x8025d0d8;
            }
            0x8025d0d8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.lr = 0x8025d0e4; f_8012ac08(c);
                pc = 0x8025d0e4;
            }
            0x8025d0e4 => {
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x8025d0ec; f_80123fbc(c);
                pc = 0x8025d0ec;
            }
            0x8025d0ec => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025d128; continue; }
                pc = 0x8025d0f4;
            }
            0x8025d0f4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8025d0fc; f_8011f5c8(c);
                pc = 0x8025d0fc;
            }
            0x8025d0fc => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff868c));
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                pc = 0x8025d108;
            }
            0x8025d108 => {
                c.r[5] = c.lhz(c.r[4].wrapping_add(c.r[3]));
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x8025d128; continue; }
                pc = 0x8025d114;
            }
            0x8025d114 => {
                c.cmpu(0, c.r[0], c.r[5]);
                if !c.crbit(2) { pc = 0x8025d120; continue; }
                pc = 0x8025d11c;
            }
            0x8025d11c => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x8025d120;
            }
            0x8025d120 => {
                c.r[3] = c.r[3].wrapping_add(0x2);
                pc = 0x8025d108; continue;
                pc = 0x8025d128;
            }
            0x8025d128 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                c.cmps(0, c.r[31], 0x6);
                if c.crbit(0) { pc = 0x8025d0d8; continue; }
                pc = 0x8025d134;
            }
            0x8025d134 => {
                { let ea = c.r[2].wrapping_add(0xffffafb8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffffafbc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.mtspr(9, c.r[30]);
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(1) { pc = 0x8025d150; continue; }
                pc = 0x8025d148;
            }
            0x8025d148 => {
                { let v = s(c.f[1] * c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8025d148; continue; }
                pc = 0x8025d150;
            }
            0x8025d150 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d0a8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025D164 (0x8025d164, 296 bytes)
#[inline(never)]
pub fn f_8025d164(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d164) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d164); f_8025d164_b(c, 0x8025d164); crate::prof::exit(c, 0x8025d164); return; }
    }
    f_8025d164_b(c, 0x8025d164)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d164_at(c: &mut Cpu, pc: u32) { f_8025d164_b(c, pc) }
#[inline(always)]
fn f_8025d164_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d164;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d164, pc); }
        match pc {
            0x8025d164 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = 0x0;
                c.lr = 0x8025d17c; f_8006b5a8(c);
                pc = 0x8025d17c;
            }
            0x8025d17c => {
                c.r[27] = c.lwz(c.r[3].wrapping_add(0xc));
                c.lr = 0x8025d184; f_8006b5a8(c);
                pc = 0x8025d184;
            }
            0x8025d184 => {
                c.r[26] = c.lwz(c.r[3]);
                c.lr = 0x8025d18c; f_8006b5a8(c);
                pc = 0x8025d18c;
            }
            0x8025d18c => {
                c.r[30] = c.lwz(c.r[3].wrapping_add(0x14));
                c.r[29] = 0x0;
                pc = 0x8025d194;
            }
            0x8025d194 => {
                c.r[3] = 0x0;
                c.lr = 0x8025d19c; f_8006b09c(c);
                pc = 0x8025d19c;
            }
            0x8025d19c => {
                c.r[4] = c.r[29].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0xb44);
                c.lr = 0x8025d1a8; f_8012ac08(c);
                pc = 0x8025d1a8;
            }
            0x8025d1a8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8025d1b0; f_80123fbc(c);
                pc = 0x8025d1b0;
            }
            0x8025d1b0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025d1ec; continue; }
                pc = 0x8025d1b8;
            }
            0x8025d1b8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025d1c0; f_8011f5c8(c);
                pc = 0x8025d1c0;
            }
            0x8025d1c0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff868c));
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                pc = 0x8025d1cc;
            }
            0x8025d1cc => {
                c.r[5] = c.lhz(c.r[4].wrapping_add(c.r[3]));
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x8025d1ec; continue; }
                pc = 0x8025d1d8;
            }
            0x8025d1d8 => {
                c.cmpu(0, c.r[0], c.r[5]);
                if !c.crbit(2) { pc = 0x8025d1e4; continue; }
                pc = 0x8025d1e0;
            }
            0x8025d1e0 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x8025d1e4;
            }
            0x8025d1e4 => {
                c.r[3] = c.r[3].wrapping_add(0x2);
                pc = 0x8025d1cc; continue;
                pc = 0x8025d1ec;
            }
            0x8025d1ec => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.cmps(0, c.r[29], 0x6);
                if c.crbit(0) { pc = 0x8025d194; continue; }
                pc = 0x8025d1f8;
            }
            0x8025d1f8 => {
                { let ea = c.r[2].wrapping_add(0xffffafb8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffffafbc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.mtspr(9, c.r[28]);
                c.cmps(0, c.r[28], 0x0);
                if !c.crbit(1) { pc = 0x8025d214; continue; }
                pc = 0x8025d20c;
            }
            0x8025d20c => {
                { let v = s(c.f[1] * c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8025d20c; continue; }
                pc = 0x8025d214;
            }
            0x8025d214 => {
                c.cmps(0, c.r[26], 0x1);
                if !c.crbit(2) { pc = 0x8025d24c; continue; }
                pc = 0x8025d21c;
            }
            0x8025d21c => {
                c.r[3] = c.r[30].wrapping_add(0x1);
                c.r[0] = 0xa;
                c.r[0] = divw(c.r[3], c.r[0]);
                c.cmps(0, c.r[0], 0xa);
                if !c.crbit(1) { pc = 0x8025d234; continue; }
                pc = 0x8025d230;
            }
            0x8025d230 => {
                c.r[0] = 0xa;
                pc = 0x8025d234;
            }
            0x8025d234 => {
                c.r[3] = 0x803a0000;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(0xffffa664);
                { let ea = c.r[3].wrapping_add(c.r[0]); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] * c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x8025d26c; continue;
                pc = 0x8025d24c;
            }
            0x8025d24c => {
                c.cmps(0, c.r[27], 0x6);
                if c.crbit(0) { pc = 0x8025d258; continue; }
                pc = 0x8025d254;
            }
            0x8025d254 => {
                c.r[27] = 0x6;
                pc = 0x8025d258;
            }
            0x8025d258 => {
                c.r[3] = 0x803a0000;
                c.r[0] = c.r[27].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(0xffffa648);
                { let ea = c.r[3].wrapping_add(c.r[0]); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] * c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x8025d26c;
            }
            0x8025d26c => {
                c.f[0] = fctiwz(c.f[1]);
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[3] = c.lwz(c.r[1].wrapping_add(0xc));
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d164: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenGetTrainerDataID (0x8025d28c, 36 bytes)
#[inline(never)]
pub fn f_8025d28c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d28c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d28c); f_8025d28c_b(c, 0x8025d28c); crate::prof::exit(c, 0x8025d28c); return; }
    }
    f_8025d28c_b(c, 0x8025d28c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d28c_at(c: &mut Cpu, pc: u32) { f_8025d28c_b(c, pc) }
#[inline(always)]
fn f_8025d28c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d28c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d28c, pc); }
        match pc {
            0x8025d28c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025d29c; f_8006b09c(c);
                pc = 0x8025d29c;
            }
            0x8025d29c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.lhz(c.r[3]);
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d28c: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenGetControlerType (0x8025d2b0, 36 bytes)
#[inline(never)]
pub fn f_8025d2b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d2b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d2b0); f_8025d2b0_b(c, 0x8025d2b0); crate::prof::exit(c, 0x8025d2b0); return; }
    }
    f_8025d2b0_b(c, 0x8025d2b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d2b0_at(c: &mut Cpu, pc: u32) { f_8025d2b0_b(c, pc) }
#[inline(always)]
fn f_8025d2b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d2b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d2b0, pc); }
        match pc {
            0x8025d2b0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025d2c0; f_8006b09c(c);
                pc = 0x8025d2c0;
            }
            0x8025d2c0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d2b0: bad pc {:#x}", pc),
        }
    }
}

/// toolentryGetTrainerSamllFaceResID (0x8025d2d4, 144 bytes)
#[inline(never)]
pub fn f_8025d2d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d2d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d2d4); f_8025d2d4_b(c, 0x8025d2d4); crate::prof::exit(c, 0x8025d2d4); return; }
    }
    f_8025d2d4_b(c, 0x8025d2d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d2d4_at(c: &mut Cpu, pc: u32) { f_8025d2d4_b(c, pc) }
#[inline(always)]
fn f_8025d2d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d2d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d2d4, pc); }
        match pc {
            0x8025d2d4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x8025d2ec; f_8006b09c(c);
                pc = 0x8025d2ec;
            }
            0x8025d2ec => {
                c.r[3] = c.lhz(c.r[3]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8025d300; continue; }
                pc = 0x8025d2f8;
            }
            0x8025d2f8 => {
                c.r[3] = 0x0;
                pc = 0x8025d350; continue;
                pc = 0x8025d300;
            }
            0x8025d300 => {
                c.lr = 0x8025d304; f_801fccc4(c);
                pc = 0x8025d304;
            }
            0x8025d304 => {
                c.lr = 0x8025d308; f_801fcba4(c);
                pc = 0x8025d308;
            }
            0x8025d308 => {
                c.r[0] = c.r[3].wrapping_mul(0x14);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff85e4));
                c.cmps(0, c.r[31], 0x0);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if !c.crbit(2) { pc = 0x8025d338; continue; }
                pc = 0x8025d31c;
            }
            0x8025d31c => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xc));
                c.r[3] = 0xf940000;
                c.r[3] = c.r[3].wrapping_add(0x1200);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025d350; continue; }
                pc = 0x8025d330;
            }
            0x8025d330 => {
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x8025d350; continue;
                pc = 0x8025d338;
            }
            0x8025d338 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x10));
                c.r[3] = 0xf8f0000;
                c.r[3] = c.r[3].wrapping_add(0x1200);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025d350; continue; }
                pc = 0x8025d34c;
            }
            0x8025d34c => {
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x8025d350;
            }
            0x8025d350 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d2d4: bad pc {:#x}", pc),
        }
    }
}

/// toolentryGetTrainerBicFaceResID (0x8025d364, 144 bytes)
#[inline(never)]
pub fn f_8025d364(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d364) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d364); f_8025d364_b(c, 0x8025d364); crate::prof::exit(c, 0x8025d364); return; }
    }
    f_8025d364_b(c, 0x8025d364)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d364_at(c: &mut Cpu, pc: u32) { f_8025d364_b(c, pc) }
#[inline(always)]
fn f_8025d364_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d364;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d364, pc); }
        match pc {
            0x8025d364 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmps(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                if c.crbit(2) { pc = 0x8025d38c; continue; }
                pc = 0x8025d380;
            }
            0x8025d380 => {
                c.lr = 0x8025d384; f_8006b09c(c);
                pc = 0x8025d384;
            }
            0x8025d384 => {
                c.r[0] = c.lhz(c.r[3]);
                pc = 0x8025d394; continue;
                pc = 0x8025d38c;
            }
            0x8025d38c => {
                c.lr = 0x8025d390; f_8006b09c(c);
                pc = 0x8025d390;
            }
            0x8025d390 => {
                c.r[0] = c.lhz(c.r[3]);
                pc = 0x8025d394;
            }
            0x8025d394 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                c.rc(c.r[3]);
                if !c.crbit(2) { pc = 0x8025d3a4; continue; }
                pc = 0x8025d39c;
            }
            0x8025d39c => {
                c.r[3] = 0x0;
                pc = 0x8025d3e0; continue;
                pc = 0x8025d3a4;
            }
            0x8025d3a4 => {
                c.lr = 0x8025d3a8; f_801fccc4(c);
                pc = 0x8025d3a8;
            }
            0x8025d3a8 => {
                c.lr = 0x8025d3ac; f_801fcba4(c);
                pc = 0x8025d3ac;
            }
            0x8025d3ac => {
                c.r[0] = c.r[3].wrapping_mul(0x14);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff85e4));
                c.cmps(0, c.r[31], 0x0);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if !c.crbit(2) { pc = 0x8025d3c8; continue; }
                pc = 0x8025d3c0;
            }
            0x8025d3c0 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                pc = 0x8025d3e0; continue;
                pc = 0x8025d3c8;
            }
            0x8025d3c8 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x8));
                c.r[3] = 0xf990000;
                c.r[3] = c.r[3].wrapping_add(0x1200);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025d3e0; continue; }
                pc = 0x8025d3dc;
            }
            0x8025d3dc => {
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x8025d3e0;
            }
            0x8025d3e0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d364: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenEntryPokemon (0x8025d3f4, 364 bytes)
#[inline(never)]
pub fn f_8025d3f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d3f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d3f4); f_8025d3f4_b(c, 0x8025d3f4); crate::prof::exit(c, 0x8025d3f4); return; }
    }
    f_8025d3f4_b(c, 0x8025d3f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d3f4_at(c: &mut Cpu, pc: u32) { f_8025d3f4_b(c, pc) }
#[inline(always)]
fn f_8025d3f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d3f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d3f4, pc); }
        match pc {
            0x8025d3f4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x8025d40c; f_8006b1d4(c);
                pc = 0x8025d40c;
            }
            0x8025d40c => {
                c.r[30] = 0x0;
                c.r[31] = c.r[27] | c.r[27];
                c.r[29] = c.r[30] | c.r[30];
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8025d41c;
            }
            0x8025d41c => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8025d424; f_8006b09c(c);
                pc = 0x8025d424;
            }
            0x8025d424 => {
                c.r[4] = c.r[29].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0xb44);
                c.lr = 0x8025d430; f_8012ac08(c);
                pc = 0x8025d430;
            }
            0x8025d430 => {
                c.lr = 0x8025d434; f_80123fbc(c);
                pc = 0x8025d434;
            }
            0x8025d434 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025d454; continue; }
                pc = 0x8025d448;
            }
            0x8025d448 => {
                c.r[3] = c.r[30].rotate_left(0) & 0xffff;
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[30] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x8025d454;
            }
            0x8025d454 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.cmps(0, c.r[29], 0x6);
                if c.crbit(0) { pc = 0x8025d41c; continue; }
                pc = 0x8025d460;
            }
            0x8025d460 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[28]);
                if !c.crbit(0) { pc = 0x8025d470; continue; }
                pc = 0x8025d46c;
            }
            0x8025d46c => {
                c.r[28] = c.r[30] | c.r[30];
                pc = 0x8025d470;
            }
            0x8025d470 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[28].rotate_left(0) & 0xffff;
                c.lr = 0x8025d47c; f_8006b09c(c);
                pc = 0x8025d47c;
            }
            0x8025d47c => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025d488; f_8006b09c(c);
                pc = 0x8025d488;
            }
            0x8025d488 => {
                c.r[0] = 0x163;
                c.r[5] = c.r[28].wrapping_add(0x28);
                c.r[4] = c.r[3].wrapping_add(0xb40);
                c.mtspr(9, c.r[0]);
                pc = 0x8025d498;
            }
            0x8025d498 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8025d498; continue; }
                pc = 0x8025d4ac;
            }
            0x8025d4ac => {
                c.r[27] = 0x0;
                c.r[29] = 0x0;
                pc = 0x8025d544; continue;
                pc = 0x8025d4b8;
            }
            0x8025d4b8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025d4c0; f_8006b09c(c);
                pc = 0x8025d4c0;
            }
            0x8025d4c0 => {
                c.r[0] = c.r[29].wrapping_add(0x8);
                c.r[28] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025d4d0; f_8006b09c(c);
                pc = 0x8025d4d0;
            }
            0x8025d4d0 => {
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0xb44);
                c.lr = 0x8025d4dc; f_8012ac08(c);
                pc = 0x8025d4dc;
            }
            0x8025d4dc => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025d4e8; f_8006b09c(c);
                pc = 0x8025d4e8;
            }
            0x8025d4e8 => {
                c.r[4] = c.r[27].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0x2c);
                c.lr = 0x8025d4f4; f_8012ac08(c);
                pc = 0x8025d4f4;
            }
            0x8025d4f4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025d4fc; f_8006b09c(c);
                pc = 0x8025d4fc;
            }
            0x8025d4fc => {
                c.r[4] = c.r[27].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0x2c);
                c.lr = 0x8025d508; f_8012ac08(c);
                pc = 0x8025d508;
            }
            0x8025d508 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025d53c; continue; }
                pc = 0x8025d510;
            }
            0x8025d510 => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x8025d53c; continue; }
                pc = 0x8025d518;
            }
            0x8025d518 => {
                c.r[0] = 0x27;
                c.r[5] = c.r[3].wrapping_add(0xfffffffc);
                c.r[4] = c.r[28].wrapping_add(0xfffffffc);
                c.mtspr(9, c.r[0]);
                pc = 0x8025d528;
            }
            0x8025d528 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8025d528; continue; }
                pc = 0x8025d53c;
            }
            0x8025d53c => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                c.r[29] = c.r[29].wrapping_add(0x4);
                pc = 0x8025d544;
            }
            0x8025d544 => {
                c.cmps(0, c.r[27], c.r[30]);
                if c.crbit(0) { pc = 0x8025d4b8; continue; }
                pc = 0x8025d54c;
            }
            0x8025d54c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d3f4: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisengetEtnryPokemonOrderNum (0x8025d560, 36 bytes)
#[inline(never)]
pub fn f_8025d560(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d560) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d560); f_8025d560_b(c, 0x8025d560); crate::prof::exit(c, 0x8025d560); return; }
    }
    f_8025d560_b(c, 0x8025d560)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d560_at(c: &mut Cpu, pc: u32) { f_8025d560_b(c, pc) }
#[inline(always)]
fn f_8025d560_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d560;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d560, pc); }
        match pc {
            0x8025d560 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025d570; f_8006b09c(c);
                pc = 0x8025d570;
            }
            0x8025d570 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x20));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d560: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenDeleteEtnryPokemonOrder (0x8025d584, 92 bytes)
#[inline(never)]
pub fn f_8025d584(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d584) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d584); f_8025d584_b(c, 0x8025d584); crate::prof::exit(c, 0x8025d584); return; }
    }
    f_8025d584_b(c, 0x8025d584)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d584_at(c: &mut Cpu, pc: u32) { f_8025d584_b(c, pc) }
#[inline(always)]
fn f_8025d584_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d584;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d584, pc); }
        match pc {
            0x8025d584 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025d594; f_8006b09c(c);
                pc = 0x8025d594;
            }
            0x8025d594 => {
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x20));
                c.r[0] = c.addc(c.r[4], 0xffffffff, 0);
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x8025d5a8; continue; }
                pc = 0x8025d5a0;
            }
            0x8025d5a0 => {
                c.cmps(0, c.r[0], 0x6);
                if !c.crbit(1) { pc = 0x8025d5b0; continue; }
                pc = 0x8025d5a8;
            }
            0x8025d5a8 => {
                c.r[3] = 0x0;
                pc = 0x8025d5d0; continue;
                pc = 0x8025d5b0;
            }
            0x8025d5b0 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[5] = 0xffffffff;
                c.r[4] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[5]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x20));
                c.r[0] = c.r[4].wrapping_add(0xffffffff);
                c.stw(c.r[3].wrapping_add(0x20), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x20));
                pc = 0x8025d5d0;
            }
            0x8025d5d0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d584: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenSetEtnryPokemonOrderGBA (0x8025d5e0, 100 bytes)
#[inline(never)]
pub fn f_8025d5e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d5e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d5e0); f_8025d5e0_b(c, 0x8025d5e0); crate::prof::exit(c, 0x8025d5e0); return; }
    }
    f_8025d5e0_b(c, 0x8025d5e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d5e0_at(c: &mut Cpu, pc: u32) { f_8025d5e0_b(c, pc) }
#[inline(always)]
fn f_8025d5e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d5e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d5e0, pc); }
        match pc {
            0x8025d5e0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[31] = c.r[5] | c.r[5];
                c.lr = 0x8025d5fc; f_8006b09c(c);
                pc = 0x8025d5fc;
            }
            0x8025d5fc => {
                c.r[6] = 0x0;
                c.r[4] = 0x0;
                c.mtspr(9, c.r[30]);
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(1) { pc = 0x8025d628; continue; }
                pc = 0x8025d610;
            }
            0x8025d610 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(c.r[4]));
                c.r[0] = c.r[4].wrapping_add(0x8);
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[5]);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8025d610; continue; }
                pc = 0x8025d628;
            }
            0x8025d628 => {
                c.stw(c.r[3].wrapping_add(0x20), c.r[6]);
                c.r[3] = c.r[6] | c.r[6];
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d5e0: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenSetEtnryPokemonOrder (0x8025d644, 256 bytes)
#[inline(never)]
pub fn f_8025d644(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d644) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d644); f_8025d644_b(c, 0x8025d644); crate::prof::exit(c, 0x8025d644); return; }
    }
    f_8025d644_b(c, 0x8025d644)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d644_at(c: &mut Cpu, pc: u32) { f_8025d644_b(c, pc) }
#[inline(always)]
fn f_8025d644_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d644;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d644, pc); }
        match pc {
            0x8025d644 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[25] = c.r[3] | c.r[3];
                c.lr = 0x8025d660; f_8006b09c(c);
                pc = 0x8025d660;
            }
            0x8025d660 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[30] = c.lwz(c.r[3].wrapping_add(0x20));
                c.lr = 0x8025d66c; f_8006b1d4(c);
                pc = 0x8025d66c;
            }
            0x8025d66c => {
                c.r[26] = 0x0;
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[27] = c.r[26] | c.r[26];
                pc = 0x8025d678;
            }
            0x8025d678 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8025d680; f_8006b09c(c);
                pc = 0x8025d680;
            }
            0x8025d680 => {
                c.r[4] = c.r[27].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0xb44);
                c.lr = 0x8025d68c; f_8012ac08(c);
                pc = 0x8025d68c;
            }
            0x8025d68c => {
                c.lr = 0x8025d690; f_80123fbc(c);
                pc = 0x8025d690;
            }
            0x8025d690 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025d6b0; continue; }
                pc = 0x8025d6a4;
            }
            0x8025d6a4 => {
                c.r[3] = c.r[26].rotate_left(0) & 0xffff;
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[26] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x8025d6b0;
            }
            0x8025d6b0 => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                c.cmps(0, c.r[27], 0x6);
                if c.crbit(0) { pc = 0x8025d678; continue; }
                pc = 0x8025d6bc;
            }
            0x8025d6bc => {
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x20));
                c.cmpu(0, c.r[0], c.r[28]);
                if !c.crbit(0) { pc = 0x8025d6d0; continue; }
                pc = 0x8025d6cc;
            }
            0x8025d6cc => {
                c.r[28] = c.r[26] | c.r[26];
                pc = 0x8025d6d0;
            }
            0x8025d6d0 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8025d6e4; continue; }
                pc = 0x8025d6dc;
            }
            0x8025d6dc => {
                c.r[3] = 0xffffffff;
                pc = 0x8025d730; continue;
                pc = 0x8025d6e4;
            }
            0x8025d6e4 => {
                c.r[3] = 0x0;
                c.mtspr(9, c.r[30]);
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(1) { pc = 0x8025d714; continue; }
                pc = 0x8025d6f4;
            }
            0x8025d6f4 => {
                c.r[0] = c.r[3].wrapping_add(0x8);
                c.r[0] = c.lwz(c.r[31].wrapping_add(c.r[0]));
                c.cmps(0, c.r[0], c.r[29]);
                if !c.crbit(2) { pc = 0x8025d70c; continue; }
                pc = 0x8025d704;
            }
            0x8025d704 => {
                c.r[3] = 0xffffffff;
                pc = 0x8025d730; continue;
                pc = 0x8025d70c;
            }
            0x8025d70c => {
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8025d6f4; continue; }
                pc = 0x8025d714;
            }
            0x8025d714 => {
                c.r[0] = c.r[30].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[29]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x20));
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[31].wrapping_add(0x20), c.r[0]);
                pc = 0x8025d730;
            }
            0x8025d730 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d644: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenInitPokemonOrder (0x8025d744, 68 bytes)
#[inline(never)]
pub fn f_8025d744(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d744) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d744); f_8025d744_b(c, 0x8025d744); crate::prof::exit(c, 0x8025d744); return; }
    }
    f_8025d744_b(c, 0x8025d744)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d744_at(c: &mut Cpu, pc: u32) { f_8025d744_b(c, pc) }
#[inline(always)]
fn f_8025d744_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d744;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d744, pc); }
        match pc {
            0x8025d744 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025d754; f_8006b09c(c);
                pc = 0x8025d754;
            }
            0x8025d754 => {
                c.r[5] = 0x0;
                c.r[0] = 0x6;
                c.stw(c.r[3].wrapping_add(0x20), c.r[5]);
                c.r[4] = 0xffffffff;
                c.mtspr(9, c.r[0]);
                pc = 0x8025d768;
            }
            0x8025d768 => {
                c.r[0] = c.r[5].wrapping_add(0x8);
                c.r[5] = c.r[5].wrapping_add(0x4);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8025d768; continue; }
                pc = 0x8025d778;
            }
            0x8025d778 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d744: bad pc {:#x}", pc),
        }
    }
}

/// toolentryCopyHero (0x8025d788, 128 bytes)
#[inline(never)]
pub fn f_8025d788(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d788) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d788); f_8025d788_b(c, 0x8025d788); crate::prof::exit(c, 0x8025d788); return; }
    }
    f_8025d788_b(c, 0x8025d788)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d788_at(c: &mut Cpu, pc: u32) { f_8025d788_b(c, pc) }
#[inline(always)]
fn f_8025d788_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d788;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d788, pc); }
        match pc {
            0x8025d788 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = 0x0;
                c.r[31] = 0x163;
                pc = 0x8025d7a0;
            }
            0x8025d7a0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8025d7a8; f_8006b09c(c);
                pc = 0x8025d7a8;
            }
            0x8025d7a8 => {
                c.r[30] = c.r[3].wrapping_add(0xb44);
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8025d7b4; f_8006b09c(c);
                pc = 0x8025d7b4;
            }
            0x8025d7b4 => {
                c.cmpu(0, c.r[30], 0x0);
                c.r[3] = c.r[3].wrapping_add(0x2c);
                if c.crbit(2) { pc = 0x8025d7e8; continue; }
                pc = 0x8025d7c0;
            }
            0x8025d7c0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025d7e8; continue; }
                pc = 0x8025d7c8;
            }
            0x8025d7c8 => {
                c.r[5] = c.r[3].wrapping_add(0xfffffffc);
                c.r[4] = c.r[30].wrapping_add(0xfffffffc);
                c.mtspr(9, c.r[31]);
                pc = 0x8025d7d4;
            }
            0x8025d7d4 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8025d7d4; continue; }
                pc = 0x8025d7e8;
            }
            0x8025d7e8 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.cmps(0, c.r[29], 0x4);
                if c.crbit(0) { pc = 0x8025d7a0; continue; }
                pc = 0x8025d7f4;
            }
            0x8025d7f4 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d788: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenGetEntryPokemonNum (0x8025d808, 148 bytes)
#[inline(never)]
pub fn f_8025d808(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d808) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d808); f_8025d808_b(c, 0x8025d808); crate::prof::exit(c, 0x8025d808); return; }
    }
    f_8025d808_b(c, 0x8025d808)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d808_at(c: &mut Cpu, pc: u32) { f_8025d808_b(c, pc) }
#[inline(always)]
fn f_8025d808_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d808;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d808, pc); }
        match pc {
            0x8025d808 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x8025d820; f_8006b1d4(c);
                pc = 0x8025d820;
            }
            0x8025d820 => {
                c.r[30] = 0x0;
                c.r[29] = c.r[28] | c.r[28];
                c.r[31] = c.r[30] | c.r[30];
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8025d830;
            }
            0x8025d830 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8025d838; f_8006b09c(c);
                pc = 0x8025d838;
            }
            0x8025d838 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0xb44);
                c.lr = 0x8025d844; f_8012ac08(c);
                pc = 0x8025d844;
            }
            0x8025d844 => {
                c.lr = 0x8025d848; f_80123fbc(c);
                pc = 0x8025d848;
            }
            0x8025d848 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025d868; continue; }
                pc = 0x8025d85c;
            }
            0x8025d85c => {
                c.r[3] = c.r[30].rotate_left(0) & 0xffff;
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[30] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x8025d868;
            }
            0x8025d868 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                c.cmps(0, c.r[31], 0x6);
                if c.crbit(0) { pc = 0x8025d830; continue; }
                pc = 0x8025d874;
            }
            0x8025d874 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.cmpu(0, c.r[0], c.r[28]);
                if !c.crbit(0) { pc = 0x8025d888; continue; }
                pc = 0x8025d884;
            }
            0x8025d884 => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x8025d888;
            }
            0x8025d888 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d808: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenGetPokemonNum (0x8025d89c, 120 bytes)
#[inline(never)]
pub fn f_8025d89c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d89c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d89c); f_8025d89c_b(c, 0x8025d89c); crate::prof::exit(c, 0x8025d89c); return; }
    }
    f_8025d89c_b(c, 0x8025d89c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d89c_at(c: &mut Cpu, pc: u32) { f_8025d89c_b(c, pc) }
#[inline(always)]
fn f_8025d89c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d89c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d89c, pc); }
        match pc {
            0x8025d89c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[29] = 0x0;
                pc = 0x8025d8b8;
            }
            0x8025d8b8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025d8c0; f_8006b09c(c);
                pc = 0x8025d8c0;
            }
            0x8025d8c0 => {
                c.r[4] = c.r[29].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0xb44);
                c.lr = 0x8025d8cc; f_8012ac08(c);
                pc = 0x8025d8cc;
            }
            0x8025d8cc => {
                c.lr = 0x8025d8d0; f_80123fbc(c);
                pc = 0x8025d8d0;
            }
            0x8025d8d0 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025d8f0; continue; }
                pc = 0x8025d8e4;
            }
            0x8025d8e4 => {
                c.r[3] = c.r[30].rotate_left(0) & 0xffff;
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[30] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x8025d8f0;
            }
            0x8025d8f0 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.cmps(0, c.r[29], 0x6);
                if c.crbit(0) { pc = 0x8025d8b8; continue; }
                pc = 0x8025d8fc;
            }
            0x8025d8fc => {
                c.r[3] = c.r[30] | c.r[30];
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d89c: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenGetHeroPtr (0x8025d914, 36 bytes)
#[inline(never)]
pub fn f_8025d914(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d914) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d914); f_8025d914_b(c, 0x8025d914); crate::prof::exit(c, 0x8025d914); return; }
    }
    f_8025d914_b(c, 0x8025d914)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d914_at(c: &mut Cpu, pc: u32) { f_8025d914_b(c, pc) }
#[inline(always)]
fn f_8025d914_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d914;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d914, pc); }
        match pc {
            0x8025d914 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025d924; f_8006b09c(c);
                pc = 0x8025d924;
            }
            0x8025d924 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[3].wrapping_add(0xb44);
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d914: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenGetEntryPokemonPtr (0x8025d938, 56 bytes)
#[inline(never)]
pub fn f_8025d938(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d938) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d938); f_8025d938_b(c, 0x8025d938); crate::prof::exit(c, 0x8025d938); return; }
    }
    f_8025d938_b(c, 0x8025d938)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d938_at(c: &mut Cpu, pc: u32) { f_8025d938_b(c, pc) }
#[inline(always)]
fn f_8025d938_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d938;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d938, pc); }
        match pc {
            0x8025d938 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x8025d950; f_8006b09c(c);
                pc = 0x8025d950;
            }
            0x8025d950 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0x2c);
                c.lr = 0x8025d95c; f_8012ac08(c);
                pc = 0x8025d95c;
            }
            0x8025d95c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d938: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenGetPokemonPtr (0x8025d970, 56 bytes)
#[inline(never)]
pub fn f_8025d970(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d970) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d970); f_8025d970_b(c, 0x8025d970); crate::prof::exit(c, 0x8025d970); return; }
    }
    f_8025d970_b(c, 0x8025d970)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d970_at(c: &mut Cpu, pc: u32) { f_8025d970_b(c, pc) }
#[inline(always)]
fn f_8025d970_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d970;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d970, pc); }
        match pc {
            0x8025d970 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x8025d988; f_8006b09c(c);
                pc = 0x8025d988;
            }
            0x8025d988 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0xb44);
                c.lr = 0x8025d994; f_8012ac08(c);
                pc = 0x8025d994;
            }
            0x8025d994 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d970: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025D9A8 (0x8025d9a8, 36 bytes)
#[inline(never)]
pub fn f_8025d9a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d9a8) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d9a8); f_8025d9a8_b(c, 0x8025d9a8); crate::prof::exit(c, 0x8025d9a8); return; }
    }
    f_8025d9a8_b(c, 0x8025d9a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d9a8_at(c: &mut Cpu, pc: u32) { f_8025d9a8_b(c, pc) }
#[inline(always)]
fn f_8025d9a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d9a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d9a8, pc); }
        match pc {
            0x8025d9a8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025d9b8; f_8006b5a8(c);
                pc = 0x8025d9b8;
            }
            0x8025d9b8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.lwz(c.r[3]);
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d9a8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025D9CC (0x8025d9cc, 36 bytes)
#[inline(never)]
pub fn f_8025d9cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d9cc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d9cc); f_8025d9cc_b(c, 0x8025d9cc); crate::prof::exit(c, 0x8025d9cc); return; }
    }
    f_8025d9cc_b(c, 0x8025d9cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d9cc_at(c: &mut Cpu, pc: u32) { f_8025d9cc_b(c, pc) }
#[inline(always)]
fn f_8025d9cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d9cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d9cc, pc); }
        match pc {
            0x8025d9cc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025d9dc; f_8006b5a8(c);
                pc = 0x8025d9dc;
            }
            0x8025d9dc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d9cc: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenGetHomePlace (0x8025d9f0, 40 bytes)
#[inline(never)]
pub fn f_8025d9f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025d9f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8025d9f0); f_8025d9f0_b(c, 0x8025d9f0); crate::prof::exit(c, 0x8025d9f0); return; }
    }
    f_8025d9f0_b(c, 0x8025d9f0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025d9f0_at(c: &mut Cpu, pc: u32) { f_8025d9f0_b(c, pc) }
#[inline(always)]
fn f_8025d9f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025d9f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025d9f0, pc); }
        match pc {
            0x8025d9f0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025da00; f_8006b09c(c);
                pc = 0x8025da00;
            }
            0x8025da00 => {
                c.lr = 0x8025da04; f_8006a7e8(c);
                pc = 0x8025da04;
            }
            0x8025da04 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025d9f0: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenGetBattlePlayerID (0x8025da18, 36 bytes)
#[inline(never)]
pub fn f_8025da18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025da18) { return; }
        if c.prof { crate::prof::enter(c, 0x8025da18); f_8025da18_b(c, 0x8025da18); crate::prof::exit(c, 0x8025da18); return; }
    }
    f_8025da18_b(c, 0x8025da18)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025da18_at(c: &mut Cpu, pc: u32) { f_8025da18_b(c, pc) }
#[inline(always)]
fn f_8025da18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025da18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025da18, pc); }
        match pc {
            0x8025da18 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025da28; f_8006b09c(c);
                pc = 0x8025da28;
            }
            0x8025da28 => {
                c.lr = 0x8025da2c; f_8006a7d8(c);
                pc = 0x8025da2c;
            }
            0x8025da2c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025da18: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenGetEntryPlayerNum (0x8025da3c, 76 bytes)
#[inline(never)]
pub fn f_8025da3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025da3c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025da3c); f_8025da3c_b(c, 0x8025da3c); crate::prof::exit(c, 0x8025da3c); return; }
    }
    f_8025da3c_b(c, 0x8025da3c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025da3c_at(c: &mut Cpu, pc: u32) { f_8025da3c_b(c, pc) }
#[inline(always)]
fn f_8025da3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025da3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025da3c, pc); }
        match pc {
            0x8025da3c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025da4c; f_8006b5a8(c);
                pc = 0x8025da4c;
            }
            0x8025da4c => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[3] = 0x2;
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x8025da74; continue; }
                pc = 0x8025da5c;
            }
            0x8025da5c => {
                if !c.crbit(0) { pc = 0x8025da78; continue; }
                pc = 0x8025da60;
            }
            0x8025da60 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x8025da6c; continue; }
                pc = 0x8025da68;
            }
            0x8025da68 => {
                pc = 0x8025da78; continue;
                pc = 0x8025da6c;
            }
            0x8025da6c => {
                c.r[3] = 0x2;
                pc = 0x8025da78; continue;
                pc = 0x8025da74;
            }
            0x8025da74 => {
                c.r[3] = 0x4;
                pc = 0x8025da78;
            }
            0x8025da78 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025da3c: bad pc {:#x}", pc),
        }
    }
}

/// toolentryTaisenGetBattleType (0x8025da88, 36 bytes)
#[inline(never)]
pub fn f_8025da88(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025da88) { return; }
        if c.prof { crate::prof::enter(c, 0x8025da88); f_8025da88_b(c, 0x8025da88); crate::prof::exit(c, 0x8025da88); return; }
    }
    f_8025da88_b(c, 0x8025da88)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025da88_at(c: &mut Cpu, pc: u32) { f_8025da88_b(c, pc) }
#[inline(always)]
fn f_8025da88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025da88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025da88, pc); }
        match pc {
            0x8025da88 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025da98; f_8006b5a8(c);
                pc = 0x8025da98;
            }
            0x8025da98 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025da88: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DAAC (0x8025daac, 36 bytes)
#[inline(never)]
pub fn f_8025daac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025daac) { return; }
        if c.prof { crate::prof::enter(c, 0x8025daac); f_8025daac_b(c, 0x8025daac); crate::prof::exit(c, 0x8025daac); return; }
    }
    f_8025daac_b(c, 0x8025daac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025daac_at(c: &mut Cpu, pc: u32) { f_8025daac_b(c, pc) }
#[inline(always)]
fn f_8025daac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025daac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025daac, pc); }
        match pc {
            0x8025daac => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025dabc; f_8006b5a8(c);
                pc = 0x8025dabc;
            }
            0x8025dabc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025daac: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DAD0 (0x8025dad0, 36 bytes)
#[inline(never)]
pub fn f_8025dad0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025dad0) { return; }
        if c.prof { crate::prof::enter(c, 0x8025dad0); f_8025dad0_b(c, 0x8025dad0); crate::prof::exit(c, 0x8025dad0); return; }
    }
    f_8025dad0_b(c, 0x8025dad0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025dad0_at(c: &mut Cpu, pc: u32) { f_8025dad0_b(c, pc) }
#[inline(always)]
fn f_8025dad0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025dad0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025dad0, pc); }
        match pc {
            0x8025dad0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025dae0; f_8006b5a8(c);
                pc = 0x8025dae0;
            }
            0x8025dae0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025dad0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DAF4 (0x8025daf4, 56 bytes)
#[inline(never)]
pub fn f_8025daf4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025daf4) { return; }
        if c.prof { crate::prof::enter(c, 0x8025daf4); f_8025daf4_b(c, 0x8025daf4); crate::prof::exit(c, 0x8025daf4); return; }
    }
    f_8025daf4_b(c, 0x8025daf4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025daf4_at(c: &mut Cpu, pc: u32) { f_8025daf4_b(c, pc) }
#[inline(always)]
fn f_8025daf4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025daf4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025daf4, pc); }
        match pc {
            0x8025daf4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025db04; f_8006b5a8(c);
                pc = 0x8025db04;
            }
            0x8025db04 => {
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x18));
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x8025db18; continue; }
                pc = 0x8025db10;
            }
            0x8025db10 => {
                c.r[0] = c.r[4].wrapping_add(0xffffffff);
                c.stw(c.r[3].wrapping_add(0x18), c.r[0]);
                pc = 0x8025db18;
            }
            0x8025db18 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025daf4: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DB2C (0x8025db2c, 48 bytes)
#[inline(never)]
pub fn f_8025db2c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025db2c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025db2c); f_8025db2c_b(c, 0x8025db2c); crate::prof::exit(c, 0x8025db2c); return; }
    }
    f_8025db2c_b(c, 0x8025db2c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025db2c_at(c: &mut Cpu, pc: u32) { f_8025db2c_b(c, pc) }
#[inline(always)]
fn f_8025db2c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025db2c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025db2c, pc); }
        match pc {
            0x8025db2c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025db3c; f_8006b5a8(c);
                pc = 0x8025db3c;
            }
            0x8025db3c => {
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x18));
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[3].wrapping_add(0x18), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x18));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025db2c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DB5C (0x8025db5c, 36 bytes)
#[inline(never)]
pub fn f_8025db5c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025db5c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025db5c); f_8025db5c_b(c, 0x8025db5c); crate::prof::exit(c, 0x8025db5c); return; }
    }
    f_8025db5c_b(c, 0x8025db5c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025db5c_at(c: &mut Cpu, pc: u32) { f_8025db5c_b(c, pc) }
#[inline(always)]
fn f_8025db5c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025db5c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025db5c, pc); }
        match pc {
            0x8025db5c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025db6c; f_8006b5a8(c);
                pc = 0x8025db6c;
            }
            0x8025db6c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025db5c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DB80 (0x8025db80, 48 bytes)
#[inline(never)]
pub fn f_8025db80(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025db80) { return; }
        if c.prof { crate::prof::enter(c, 0x8025db80); f_8025db80_b(c, 0x8025db80); crate::prof::exit(c, 0x8025db80); return; }
    }
    f_8025db80_b(c, 0x8025db80)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025db80_at(c: &mut Cpu, pc: u32) { f_8025db80_b(c, pc) }
#[inline(always)]
fn f_8025db80_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025db80;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025db80, pc); }
        match pc {
            0x8025db80 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025db90; f_8006b5a8(c);
                pc = 0x8025db90;
            }
            0x8025db90 => {
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x14));
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[3].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x14));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025db80: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DBB0 (0x8025dbb0, 36 bytes)
#[inline(never)]
pub fn f_8025dbb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025dbb0) { return; }
        if c.prof { crate::prof::enter(c, 0x8025dbb0); f_8025dbb0_b(c, 0x8025dbb0); crate::prof::exit(c, 0x8025dbb0); return; }
    }
    f_8025dbb0_b(c, 0x8025dbb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025dbb0_at(c: &mut Cpu, pc: u32) { f_8025dbb0_b(c, pc) }
#[inline(always)]
fn f_8025dbb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025dbb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025dbb0, pc); }
        match pc {
            0x8025dbb0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025dbc0; f_8006b5a8(c);
                pc = 0x8025dbc0;
            }
            0x8025dbc0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025dbb0: bad pc {:#x}", pc),
        }
    }
}

/// tableResBiosGetResPtr (0x8025dbd4, 88 bytes)
#[inline(never)]
pub fn f_8025dbd4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025dbd4) { return; }
        if c.prof { crate::prof::enter(c, 0x8025dbd4); f_8025dbd4_b(c, 0x8025dbd4); crate::prof::exit(c, 0x8025dbd4); return; }
    }
    f_8025dbd4_b(c, 0x8025dbd4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025dbd4_at(c: &mut Cpu, pc: u32) { f_8025dbd4_b(c, pc) }
#[inline(always)]
fn f_8025dbd4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025dbd4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025dbd4, pc); }
        match pc {
            0x8025dbd4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff85e8));
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8025dc10; continue; }
                pc = 0x8025dbf0;
            }
            0x8025dbf0 => {
                c.r[3] = 0x80280000;
                c.r[4] = 0x803a0000;
                c.r[3] = c.r[3].wrapping_add(0xffffa450);
                c.r[4] = c.r[4].wrapping_add(0xffffa690);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8025dc08; stubs_gen::f_800dd970(c);
                pc = 0x8025dc08;
            }
            0x8025dc08 => {
                c.r[3] = 0x0;
                pc = 0x8025dc1c; continue;
                pc = 0x8025dc10;
            }
            0x8025dc10 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff85ec));
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                pc = 0x8025dc1c;
            }
            0x8025dc1c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025dbd4: bad pc {:#x}", pc),
        }
    }
}

/// waitTime__Ff (0x8025dc2c, 144 bytes)
#[inline(never)]
pub fn f_8025dc2c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025dc2c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025dc2c); f_8025dc2c_b(c, 0x8025dc2c); crate::prof::exit(c, 0x8025dc2c); return; }
    }
    f_8025dc2c_b(c, 0x8025dc2c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025dc2c_at(c: &mut Cpu, pc: u32) { f_8025dc2c_b(c, pc) }
#[inline(always)]
fn f_8025dc2c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025dc2c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025dc2c, pc); }
        match pc {
            0x8025dc2c => {
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = (c.f[29], c.ps1[29]); c.psq_store(ea, x, y, false, 0); }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                { let ea = c.r[2].wrapping_add(0xffffafc4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[31] = 0x43300000;
                { let ea = c.r[2].wrapping_add(0xffffafc0); c.f[29] = c.lfs(ea); c.ps1[29] = c.f[29]; }
                { let v = s(c.f[0] * c.f[1]); c.f[30] = v; c.ps1[30] = v; }
                { let ea = c.r[2].wrapping_add(0xffffafc8); c.f[31] = c.lfd(ea); }
                pc = 0x8025dc88; continue;
                pc = 0x8025dc6c;
            }
            0x8025dc6c => {
                c.lr = 0x8025dc70; crate::stubs::f_800f0308(c);
                pc = 0x8025dc70;
            }
            0x8025dc70 => {
                c.lr = 0x8025dc74; f_800d3088(c);
                pc = 0x8025dc74;
            }
            0x8025dc74 => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[31]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[31]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[29] + c.f[0]); c.f[29] = v; c.ps1[29] = v; }
                pc = 0x8025dc88;
            }
            0x8025dc88 => {
                c.fcmp(0, c.f[29], c.f[30]);
                if c.crbit(0) { pc = 0x8025dc6c; continue; }
                pc = 0x8025dc90;
            }
            0x8025dc90 => {
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x40); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x30); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                { let ea = c.r[1].wrapping_add(0x20); c.f[29] = c.lfd(ea); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { return; }
                return;
            }
            _ => panic!("f_8025dc2c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DCBC (0x8025dcbc, 88 bytes)
#[inline(never)]
pub fn f_8025dcbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025dcbc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025dcbc); f_8025dcbc_b(c, 0x8025dcbc); crate::prof::exit(c, 0x8025dcbc); return; }
    }
    f_8025dcbc_b(c, 0x8025dcbc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025dcbc_at(c: &mut Cpu, pc: u32) { f_8025dcbc_b(c, pc) }
#[inline(always)]
fn f_8025dcbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025dcbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025dcbc, pc); }
        match pc {
            0x8025dcbc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025dce8; continue; }
                pc = 0x8025dcdc;
            }
            0x8025dcdc => {
                c.r[4] = 0x32;
                c.r[5] = 0xff;
                c.lr = 0x8025dce8; f_80165a20(c);
                pc = 0x8025dce8;
            }
            0x8025dce8 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x4));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025dd00; continue; }
                pc = 0x8025dcf4;
            }
            0x8025dcf4 => {
                c.r[4] = 0x32;
                c.r[5] = 0xff;
                c.lr = 0x8025dd00; f_801659fc(c);
                pc = 0x8025dd00;
            }
            0x8025dd00 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025dcbc: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DD14 (0x8025dd14, 152 bytes)
#[inline(never)]
pub fn f_8025dd14(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025dd14) { return; }
        if c.prof { crate::prof::enter(c, 0x8025dd14); f_8025dd14_b(c, 0x8025dd14); crate::prof::exit(c, 0x8025dd14); return; }
    }
    f_8025dd14_b(c, 0x8025dd14)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025dd14_at(c: &mut Cpu, pc: u32) { f_8025dd14_b(c, pc) }
#[inline(always)]
fn f_8025dd14_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025dd14;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025dd14, pc); }
        match pc {
            0x8025dd14 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x8025dd38; f_801653c4(c);
                pc = 0x8025dd38;
            }
            0x8025dd38 => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x8025dd60; continue; }
                pc = 0x8025dd40;
            }
            0x8025dd40 => {
                c.lr = 0x8025dd44; f_801656d8(c);
                pc = 0x8025dd44;
            }
            0x8025dd44 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x1;
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = 0x32;
                c.r[5] = 0xff;
                c.lr = 0x8025dd5c; f_80165a20(c);
                pc = 0x8025dd5c;
            }
            0x8025dd5c => {
                pc = 0x8025dd64; continue;
                pc = 0x8025dd60;
            }
            0x8025dd60 => {
                c.r[29] = 0x0;
                pc = 0x8025dd64;
            }
            0x8025dd64 => {
                c.lr = 0x8025dd68; f_801653bc(c);
                pc = 0x8025dd68;
            }
            0x8025dd68 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x8025dd78; continue; }
                pc = 0x8025dd70;
            }
            0x8025dd70 => {
                c.lr = 0x8025dd74; f_801656d8(c);
                pc = 0x8025dd74;
            }
            0x8025dd74 => {
                pc = 0x8025dd7c; continue;
                pc = 0x8025dd78;
            }
            0x8025dd78 => {
                c.r[3] = 0x0;
                pc = 0x8025dd7c;
            }
            0x8025dd7c => {
                c.stw(c.r[28], c.r[30]);
                c.stw(c.r[28].wrapping_add(0x4), c.r[31]);
                c.stw(c.r[28].wrapping_add(0x8), c.r[29]);
                c.stw(c.r[28].wrapping_add(0xc), c.r[3]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025dd14: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DDAC (0x8025ddac, 72 bytes)
#[inline(never)]
pub fn f_8025ddac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025ddac) { return; }
        if c.prof { crate::prof::enter(c, 0x8025ddac); f_8025ddac_b(c, 0x8025ddac); crate::prof::exit(c, 0x8025ddac); return; }
    }
    f_8025ddac_b(c, 0x8025ddac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025ddac_at(c: &mut Cpu, pc: u32) { f_8025ddac_b(c, pc) }
#[inline(always)]
fn f_8025ddac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025ddac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025ddac, pc); }
        match pc {
            0x8025ddac => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025dde0; continue; }
                pc = 0x8025ddcc;
            }
            0x8025ddcc => {
                c.lr = 0x8025ddd0; f_801dac3c(c);
                pc = 0x8025ddd0;
            }
            0x8025ddd0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025dde0; continue; }
                pc = 0x8025ddd8;
            }
            0x8025ddd8 => {
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8025dde0; f_800e4170(c);
                pc = 0x8025dde0;
            }
            0x8025dde0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025ddac: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DDF4 (0x8025ddf4, 24 bytes)
#[inline(never)]
pub fn f_8025ddf4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025ddf4) { return; }
        if c.prof { crate::prof::enter(c, 0x8025ddf4); f_8025ddf4_b(c, 0x8025ddf4); crate::prof::exit(c, 0x8025ddf4); return; }
    }
    f_8025ddf4_b(c, 0x8025ddf4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025ddf4_at(c: &mut Cpu, pc: u32) { f_8025ddf4_b(c, pc) }
#[inline(always)]
fn f_8025ddf4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025ddf4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025ddf4, pc); }
        match pc {
            0x8025ddf4 => {
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8025de00;
            }
            0x8025de00 => {
                c.r[0] = 0x0;
                c.stw(c.r[3], c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025ddf4: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DE0C (0x8025de0c, 72 bytes)
#[inline(never)]
pub fn f_8025de0c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025de0c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025de0c); f_8025de0c_b(c, 0x8025de0c); crate::prof::exit(c, 0x8025de0c); return; }
    }
    f_8025de0c_b(c, 0x8025de0c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025de0c_at(c: &mut Cpu, pc: u32) { f_8025de0c_b(c, pc) }
#[inline(always)]
fn f_8025de0c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025de0c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025de0c, pc); }
        match pc {
            0x8025de0c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025de40; continue; }
                pc = 0x8025de2c;
            }
            0x8025de2c => {
                c.lr = 0x8025de30; f_801dac3c(c);
                pc = 0x8025de30;
            }
            0x8025de30 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025de40; continue; }
                pc = 0x8025de38;
            }
            0x8025de38 => {
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8025de40; f_800e43a4(c);
                pc = 0x8025de40;
            }
            0x8025de40 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025de0c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DE54 (0x8025de54, 228 bytes)
#[inline(never)]
pub fn f_8025de54(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025de54) { return; }
        if c.prof { crate::prof::enter(c, 0x8025de54); f_8025de54_b(c, 0x8025de54); crate::prof::exit(c, 0x8025de54); return; }
    }
    f_8025de54_b(c, 0x8025de54)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025de54_at(c: &mut Cpu, pc: u32) { f_8025de54_b(c, pc) }
#[inline(always)]
fn f_8025de54_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025de54;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025de54, pc); }
        match pc {
            0x8025de54 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmps(0, c.r[7], 0x1);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[28] = c.r[6] | c.r[6];
                c.r[29] = c.r[8] | c.r[8];
                if !c.crbit(2) { pc = 0x8025de8c; continue; }
                pc = 0x8025de80;
            }
            0x8025de80 => {
                { let ea = c.r[2].wrapping_add(0xffffafd0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8025de8c; f_801c41c8(c);
                pc = 0x8025de8c;
            }
            0x8025de8c => {
                c.r[3] = c.lwz(c.r[26]);
                c.r[4] = 0x1;
                c.lr = 0x8025de98; f_801da4e8(c);
                pc = 0x8025de98;
            }
            0x8025de98 => {
                c.r[31] = c.r[30] | c.r[30];
                c.r[30] = 0x0;
                pc = 0x8025df1c; continue;
                pc = 0x8025dea4;
            }
            0x8025dea4 => {
                c.cmps(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x8025debc; continue; }
                pc = 0x8025deac;
            }
            0x8025deac => {
                c.r[3] = c.lwz(c.r[26]);
                c.r[4] = c.lhz(c.r[31]);
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.lr = 0x8025debc; f_801da914(c);
                pc = 0x8025debc;
            }
            0x8025debc => {
                c.r[3] = c.lwz(c.r[26]);
                c.r[4] = c.lhz(c.r[31]);
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.lr = 0x8025decc; f_801da9e8(c);
                pc = 0x8025decc;
            }
            0x8025decc => {
                pc = 0x8025ded8; continue;
                pc = 0x8025ded0;
            }
            0x8025ded0 => {
                c.lr = 0x8025ded4; f_801db088(c);
                pc = 0x8025ded4;
            }
            0x8025ded4 => {
                c.lr = 0x8025ded8; crate::stubs::f_800f0308(c);
                pc = 0x8025ded8;
            }
            0x8025ded8 => {
                c.r[3] = c.lwz(c.r[26]);
                c.r[4] = c.lhz(c.r[31]);
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.lr = 0x8025dee8; f_801da94c(c);
                pc = 0x8025dee8;
            }
            0x8025dee8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025ded0; continue; }
                pc = 0x8025def0;
            }
            0x8025def0 => {
                c.cmps(0, c.r[29], 0x1);
                if !c.crbit(2) { pc = 0x8025df04; continue; }
                pc = 0x8025def8;
            }
            0x8025def8 => {
                c.r[3] = c.lwz(c.r[26]);
                c.r[4] = 0x0;
                c.lr = 0x8025df04; f_801da4e8(c);
                pc = 0x8025df04;
            }
            0x8025df04 => {
                c.r[3] = c.lwz(c.r[26]);
                c.r[4] = c.lhz(c.r[31]);
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.lr = 0x8025df14; f_801da8c4(c);
                pc = 0x8025df14;
            }
            0x8025df14 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x8025df1c;
            }
            0x8025df1c => {
                c.cmps(0, c.r[30], c.r[27]);
                if c.crbit(0) { pc = 0x8025dea4; continue; }
                pc = 0x8025df24;
            }
            0x8025df24 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025de54: bad pc {:#x}", pc),
        }
    }
}

/// fn_8025DF38 (0x8025df38, 376 bytes)
#[inline(never)]
pub fn f_8025df38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025df38) { return; }
        if c.prof { crate::prof::enter(c, 0x8025df38); f_8025df38_b(c, 0x8025df38); crate::prof::exit(c, 0x8025df38); return; }
    }
    f_8025df38_b(c, 0x8025df38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025df38_at(c: &mut Cpu, pc: u32) { f_8025df38_b(c, pc) }
#[inline(always)]
fn f_8025df38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025df38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025df38, pc); }
        match pc {
            0x8025df38 => {
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x4c); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.r[24] = c.r[4] | c.r[4];
                c.r[23] = c.r[5] | c.r[5];
                c.r[25] = c.r[6] | c.r[6];
                c.r[3] = 0xffffffff;
                c.lr = 0x8025df60; f_800e0c04(c);
                pc = 0x8025df60;
            }
            0x8025df60 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[24] | c.r[24];
                c.r[5] = 0x0;
                c.lr = 0x8025df70; f_801de190(c);
                pc = 0x8025df70;
            }
            0x8025df70 => {
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[26], c.r[3]);
                if !c.crbit(2) { pc = 0x8025df84; continue; }
                pc = 0x8025df7c;
            }
            0x8025df7c => {
                c.r[3] = 0x0;
                pc = 0x8025e09c; continue;
                pc = 0x8025df84;
            }
            0x8025df84 => {
                c.r[24] = 0x0;
                pc = 0x8025dfb8; continue;
                pc = 0x8025df8c;
            }
            0x8025df8c => {
                c.r[3] = c.lwz(c.r[26]);
                c.r[6] = 0x0;
                c.r[4] = c.lhz(c.r[23]);
                c.r[5] = c.lhz(c.r[23].wrapping_add(0x2));
                c.lr = 0x8025dfa0; f_801ddd28(c);
                pc = 0x8025dfa0;
            }
            0x8025dfa0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025dfb0; continue; }
                pc = 0x8025dfa8;
            }
            0x8025dfa8 => {
                c.r[3] = 0x0;
                pc = 0x8025e09c; continue;
                pc = 0x8025dfb0;
            }
            0x8025dfb0 => {
                c.r[23] = c.r[23].wrapping_add(0x4);
                c.r[24] = c.r[24].wrapping_add(0x1);
                pc = 0x8025dfb8;
            }
            0x8025dfb8 => {
                c.cmps(0, c.r[24], c.r[25]);
                if c.crbit(0) { pc = 0x8025df8c; continue; }
                pc = 0x8025dfc0;
            }
            0x8025dfc0 => {
                c.cmpu(0, c.r[26], 0x0);
                if c.crbit(2) { pc = 0x8025e098; continue; }
                pc = 0x8025dfc8;
            }
            0x8025dfc8 => {
                c.r[3] = c.lwz(c.r[26]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025e098; continue; }
                pc = 0x8025dfd4;
            }
            0x8025dfd4 => {
                c.lr = 0x8025dfd8; f_801dac3c(c);
                pc = 0x8025dfd8;
            }
            0x8025dfd8 => {
                c.r[28] = c.r[3] | c.r[3];
                c.rc(c.r[28]);
                if c.crbit(2) { pc = 0x8025e098; continue; }
                pc = 0x8025dfe0;
            }
            0x8025dfe0 => {
                c.r[4] = 0x1;
                c.lr = 0x8025dfe8; f_800e90c8(c);
                pc = 0x8025dfe8;
            }
            0x8025dfe8 => {
                c.lr = 0x8025dfec; f_80115bd8(c);
                pc = 0x8025dfec;
            }
            0x8025dfec => {
                c.r[26] = c.r[3] | c.r[3];
                c.rc(c.r[26]);
                if c.crbit(2) { pc = 0x8025e098; continue; }
                pc = 0x8025dff4;
            }
            0x8025dff4 => {
                c.r[3] = 0x1;
                c.lr = 0x8025dffc; f_8018f470(c);
                pc = 0x8025dffc;
            }
            0x8025dffc => {
                c.r[23] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8025e008; f_80115280(c);
                pc = 0x8025e008;
            }
            0x8025e008 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[27] = 0x0;
                c.cmpu(0, c.r[29], 0x1);
                c.r[31] = c.r[27] | c.r[27];
                if !c.crbit(2) { pc = 0x8025e098; continue; }
                pc = 0x8025e01c;
            }
            0x8025e01c => {
                c.r[25] = c.r[27] | c.r[27];
                c.r[30] = c.r[1].wrapping_add(0x8);
                pc = 0x8025e05c; continue;
                pc = 0x8025e028;
            }
            0x8025e028 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[25] | c.r[25];
                c.lr = 0x8025e034; f_8011538c(c);
                pc = 0x8025e034;
            }
            0x8025e034 => {
                c.r[24] = c.r[3] | c.r[3];
                c.lr = 0x8025e03c; f_800ff56c(c);
                pc = 0x8025e03c;
            }
            0x8025e03c => {
                c.r[4] = c.r[24] | c.r[24];
                c.lr = 0x8025e044; f_80113f6c(c);
                pc = 0x8025e044;
            }
            0x8025e044 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025e058; continue; }
                pc = 0x8025e04c;
            }
            0x8025e04c => {
                c.stw(c.r[30].wrapping_add(c.r[31]), c.r[3]);
                c.r[27] = c.r[27].wrapping_add(0x1);
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x8025e058;
            }
            0x8025e058 => {
                c.r[25] = c.r[25].wrapping_add(0x1);
                pc = 0x8025e05c;
            }
            0x8025e05c => {
                c.cmpu(0, c.r[25], c.r[29]);
                if c.crbit(0) { pc = 0x8025e028; continue; }
                pc = 0x8025e064;
            }
            0x8025e064 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x1;
                c.lr = 0x8025e070; f_800e9108(c);
                pc = 0x8025e070;
            }
            0x8025e070 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8025e07c; f_800e8fe8(c);
                pc = 0x8025e07c;
            }
            0x8025e07c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8025e08c; f_800e900c(c);
                pc = 0x8025e08c;
            }
            0x8025e08c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x1;
                c.lr = 0x8025e098; f_800e3c94(c);
                pc = 0x8025e098;
            }
            0x8025e098 => {
                c.r[3] = 0x1;
                pc = 0x8025e09c;
            }
            0x8025e09c => {
                { let mut ea = c.r[1].wrapping_add(0x4c); for n in 23..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x74));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { return; }
                return;
            }
            _ => panic!("f_8025df38: bad pc {:#x}", pc),
        }
    }
}

/// loadSequence (0x8025e0b0, 268 bytes)
#[inline(never)]
pub fn f_8025e0b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025e0b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8025e0b0); f_8025e0b0_b(c, 0x8025e0b0); crate::prof::exit(c, 0x8025e0b0); return; }
    }
    f_8025e0b0_b(c, 0x8025e0b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025e0b0_at(c: &mut Cpu, pc: u32) { f_8025e0b0_b(c, pc) }
#[inline(always)]
fn f_8025e0b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025e0b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025e0b0, pc); }
        match pc {
            0x8025e0b0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[5] | c.r[5];
                c.r[29] = c.r[6] | c.r[6];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x8025e0e4; f_8012640c(c);
                pc = 0x8025e0e4;
            }
            0x8025e0e4 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[4]);
                if !c.crbit(2) { pc = 0x8025e0f4; continue; }
                pc = 0x8025e0ec;
            }
            0x8025e0ec => {
                c.r[3] = 0x0;
                pc = 0x8025e1a8; continue;
                pc = 0x8025e0f4;
            }
            0x8025e0f4 => {
                c.r[3] = 0x0;
                c.r[5] = 0x66;
                c.r[6] = 0x0;
                c.lr = 0x8025e104; f_8012640c(c);
                pc = 0x8025e104;
            }
            0x8025e104 => {
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[30]);
                if !c.crbit(2) { pc = 0x8025e114; continue; }
                pc = 0x8025e10c;
            }
            0x8025e10c => {
                c.r[3] = 0x0;
                pc = 0x8025e1a8; continue;
                pc = 0x8025e114;
            }
            0x8025e114 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025e11c; f_8011f5b0(c);
                pc = 0x8025e11c;
            }
            0x8025e11c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xc1;
                c.r[6] = 0x0;
                c.lr = 0x8025e138; f_8012640c(c);
                pc = 0x8025e138;
            }
            0x8025e138 => {
                c.r[0] = c.r[3].wrapping_neg();
                c.r[4] = c.r[31] | c.r[31];
                c.r[0] = c.r[0] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = c.r[0].rotate_left(1) & 0x1;
                c.lr = 0x8025e150; f_801de190(c);
                pc = 0x8025e150;
            }
            0x8025e150 => {
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[27], c.r[3]);
                if !c.crbit(2) { pc = 0x8025e164; continue; }
                pc = 0x8025e15c;
            }
            0x8025e15c => {
                c.r[3] = 0x0;
                pc = 0x8025e1a8; continue;
                pc = 0x8025e164;
            }
            0x8025e164 => {
                c.r[31] = c.r[28] | c.r[28];
                c.r[30] = 0x0;
                pc = 0x8025e19c; continue;
                pc = 0x8025e170;
            }
            0x8025e170 => {
                c.r[3] = c.lwz(c.r[27]);
                c.r[6] = 0x0;
                c.r[4] = c.lhz(c.r[31]);
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.lr = 0x8025e184; f_801ddd28(c);
                pc = 0x8025e184;
            }
            0x8025e184 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025e194; continue; }
                pc = 0x8025e18c;
            }
            0x8025e18c => {
                c.r[3] = 0x0;
                pc = 0x8025e1a8; continue;
                pc = 0x8025e194;
            }
            0x8025e194 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x8025e19c;
            }
            0x8025e19c => {
                c.cmps(0, c.r[30], c.r[29]);
                if c.crbit(0) { pc = 0x8025e170; continue; }
                pc = 0x8025e1a4;
            }
            0x8025e1a4 => {
                c.r[3] = 0x1;
                pc = 0x8025e1a8;
            }
            0x8025e1a8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025e0b0: bad pc {:#x}", pc),
        }
    }
}

/// std244_spawnEffectAtPosition (0x8025e1bc, 500 bytes)
#[inline(never)]
pub fn f_8025e1bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025e1bc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025e1bc); f_8025e1bc_b(c, 0x8025e1bc); crate::prof::exit(c, 0x8025e1bc); return; }
    }
    f_8025e1bc_b(c, 0x8025e1bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025e1bc_at(c: &mut Cpu, pc: u32) { f_8025e1bc_b(c, pc) }
#[inline(always)]
fn f_8025e1bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025e1bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025e1bc, pc); }
        match pc {
            0x8025e1bc => {
                { let ea = c.r[1].wrapping_add(0xffffff70); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x94), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x80); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x70); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = (c.f[29], c.ps1[29]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[28]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = (c.f[28], c.ps1[28]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[27]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = (c.f[27], c.ps1[27]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[26]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = (c.f[26], c.ps1[26]); c.psq_store(ea, x, y, false, 0); }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.f[26] = c.f[1];
                c.r[3] = 0x1;
                c.f[27] = c.f[2];
                c.f[28] = c.f[3];
                c.f[29] = c.f[4];
                c.f[30] = c.f[5];
                c.f[31] = c.f[6];
                c.lr = 0x8025e220; f_801dadc0(c);
                pc = 0x8025e220;
            }
            0x8025e220 => {
                c.r[3] = c.r[13].wrapping_add(0xffffae38);
                c.lr = 0x8025e228; f_8025ddf4(c);
                pc = 0x8025e228;
            }
            0x8025e228 => {
                c.r[3] = c.r[13].wrapping_add(0xffffae38);
                c.r[4] = 0x3f;
                c.r[5] = c.r[13].wrapping_add(0xffff8580);
                c.r[6] = 0x1;
                c.lr = 0x8025e23c; f_8025df38(c);
                pc = 0x8025e23c;
            }
            0x8025e23c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025e364; continue; }
                pc = 0x8025e254;
            }
            0x8025e254 => {
                c.r[31] = c.r[13].wrapping_add(0xffffae38);
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8025e270; continue; }
                pc = 0x8025e260;
            }
            0x8025e260 => {
                c.r[0] = c.lwz(c.r[31]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025e270; continue; }
                pc = 0x8025e26c;
            }
            0x8025e26c => {
                pc = 0x8025e274; continue;
                pc = 0x8025e270;
            }
            0x8025e270 => {
                c.r[31] = 0x0;
                pc = 0x8025e274;
            }
            0x8025e274 => {
                c.r[30] = c.r[13].wrapping_add(0xffffae38);
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8025e290; continue; }
                pc = 0x8025e280;
            }
            0x8025e280 => {
                c.r[0] = c.lwz(c.r[30]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025e290; continue; }
                pc = 0x8025e28c;
            }
            0x8025e28c => {
                pc = 0x8025e294; continue;
                pc = 0x8025e290;
            }
            0x8025e290 => {
                c.r[30] = 0x0;
                pc = 0x8025e294;
            }
            0x8025e294 => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8025e2c8; continue; }
                pc = 0x8025e29c;
            }
            0x8025e29c => {
                c.r[0] = c.lwz(c.r[30]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025e2c8; continue; }
                pc = 0x8025e2a8;
            }
            0x8025e2a8 => {
                c.f[1] = c.f[26];
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.f[2] = c.f[27];
                c.f[3] = c.f[28];
                c.lr = 0x8025e2bc; f_800e01f4(c);
                pc = 0x8025e2bc;
            }
            0x8025e2bc => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.lr = 0x8025e2c8; f_8025de0c(c);
                pc = 0x8025e2c8;
            }
            0x8025e2c8 => {
                c.r[3] = c.r[13].wrapping_add(0xffffae38);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025e2e4; continue; }
                pc = 0x8025e2d4;
            }
            0x8025e2d4 => {
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025e2e4; continue; }
                pc = 0x8025e2e0;
            }
            0x8025e2e0 => {
                pc = 0x8025e2e8; continue;
                pc = 0x8025e2e4;
            }
            0x8025e2e4 => {
                c.r[3] = 0x0;
                pc = 0x8025e2e8;
            }
            0x8025e2e8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025e320; continue; }
                pc = 0x8025e2f0;
            }
            0x8025e2f0 => {
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025e320; continue; }
                pc = 0x8025e2fc;
            }
            0x8025e2fc => {
                { let ea = c.r[2].wrapping_add(0xffffafd8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.r[1].wrapping_add(0x8);
                { let v = s(c.f[0] * c.f[29]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[0] * c.f[30]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] * c.f[31]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0xc); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x8025e320; f_8025ddac(c);
                pc = 0x8025e320;
            }
            0x8025e320 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8025e364; continue; }
                pc = 0x8025e328;
            }
            0x8025e328 => {
                c.r[0] = c.lwz(c.r[31]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025e364; continue; }
                pc = 0x8025e334;
            }
            0x8025e334 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[13].wrapping_add(0xffff8580);
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x1;
                c.lr = 0x8025e350; f_8025de54(c);
                pc = 0x8025e350;
            }
            0x8025e350 => {
                { let ea = c.r[2].wrapping_add(0xffffafdc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x5;
                c.lr = 0x8025e35c; f_801c41c8(c);
                pc = 0x8025e35c;
            }
            0x8025e35c => {
                c.r[3] = 0x1;
                c.lr = 0x8025e364; f_801c40f0(c);
                pc = 0x8025e364;
            }
            0x8025e364 => {
                c.lr = 0x8025e368; f_801dac90(c);
                pc = 0x8025e368;
            }
            0x8025e368 => {
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x80); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x70); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                { let ea = c.r[1].wrapping_add(0x60); c.f[29] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = c.psq_load(ea, false, 0); c.f[28] = x; c.ps1[28] = y; }
                { let ea = c.r[1].wrapping_add(0x50); c.f[28] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = c.psq_load(ea, false, 0); c.f[27] = x; c.ps1[27] = y; }
                { let ea = c.r[1].wrapping_add(0x40); c.f[27] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = c.psq_load(ea, false, 0); c.f[26] = x; c.ps1[26] = y; }
                { let ea = c.r[1].wrapping_add(0x30); c.f[26] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x94));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x28));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x90);
                if true { return; }
                return;
            }
            _ => panic!("f_8025e1bc: bad pc {:#x}", pc),
        }
    }
}

/// _cbWazaForget__FP7PokemonUsl (0x8025e3b0, 388 bytes)
#[inline(never)]
pub fn f_8025e3b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025e3b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8025e3b0); f_8025e3b0_b(c, 0x8025e3b0); crate::prof::exit(c, 0x8025e3b0); return; }
    }
    f_8025e3b0_b(c, 0x8025e3b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025e3b0_at(c: &mut Cpu, pc: u32) { f_8025e3b0_b(c, pc) }
#[inline(always)]
fn f_8025e3b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025e3b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025e3b0, pc); }
        match pc {
            0x8025e3b0 => {
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffffafe0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[1].wrapping_add(0x64), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x5c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x3;
                c.stw(c.r[1].wrapping_add(0x58), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x54), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x50), c.r[28]);
                c.r[28] = c.r[4] | c.r[4];
                c.lr = 0x8025e3e0; f_801c41c8(c);
                pc = 0x8025e3e0;
            }
            0x8025e3e0 => {
                c.r[3] = 0x1;
                c.lr = 0x8025e3e8; f_801c40f0(c);
                pc = 0x8025e3e8;
            }
            0x8025e3e8 => {
                c.r[3] = c.r[1].wrapping_add(0x1c);
                c.lr = 0x8025e3f0; f_801778b4(c);
                pc = 0x8025e3f0;
            }
            0x8025e3f0 => {
                c.r[3] = c.r[1].wrapping_add(0x28);
                c.lr = 0x8025e3f8; f_80177830(c);
                pc = 0x8025e3f8;
            }
            0x8025e3f8 => {
                c.r[3] = c.r[1].wrapping_add(0x34);
                c.lr = 0x8025e400; f_80177908(c);
                pc = 0x8025e400;
            }
            0x8025e400 => {
                c.r[3] = c.r[1].wrapping_add(0x40);
                c.lr = 0x8025e408; f_8017795c(c);
                pc = 0x8025e408;
            }
            0x8025e408 => {
                c.lr = 0x8025e40c; f_800d2584(c);
                pc = 0x8025e40c;
            }
            0x8025e40c => {
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.r[5] = c.r[1].wrapping_add(0x10);
                c.r[6] = c.r[1].wrapping_add(0x14);
                c.r[7] = c.r[1].wrapping_add(0x18);
                c.lr = 0x8025e420; f_800d1fdc(c);
                pc = 0x8025e420;
            }
            0x8025e420 => {
                c.r[3] = 0x80480000;
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[29] = c.r[3].wrapping_add(0xffff8288);
                c.r[12] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[11] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[10] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[9] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[8] = c.lwz(c.r[1].wrapping_add(0x30));
                c.r[7] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[6] = c.lwz(c.r[1].wrapping_add(0x38));
                c.r[5] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x40));
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x48));
                { let ea = c.r[1].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[29], c.r[30]);
                c.stw(c.r[29].wrapping_add(0x4), c.r[12]);
                c.stw(c.r[29].wrapping_add(0x8), c.r[11]);
                c.stw(c.r[29].wrapping_add(0xc), c.r[10]);
                c.stw(c.r[29].wrapping_add(0x10), c.r[9]);
                c.stw(c.r[29].wrapping_add(0x14), c.r[8]);
                c.stw(c.r[29].wrapping_add(0x18), c.r[7]);
                c.stw(c.r[29].wrapping_add(0x1c), c.r[6]);
                c.stw(c.r[29].wrapping_add(0x20), c.r[5]);
                c.stw(c.r[29].wrapping_add(0x24), c.r[4]);
                c.stw(c.r[29].wrapping_add(0x28), c.r[3]);
                c.stw(c.r[29].wrapping_add(0x2c), c.r[0]);
                { let ea = c.r[29].wrapping_add(0x30); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x8025e494; f_801dac90(c);
                pc = 0x8025e494;
            }
            0x8025e494 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025e4a0; f_80097a38(c);
                pc = 0x8025e4a0;
            }
            0x8025e4a0 => {
                c.r[29] = c.r[3] | c.r[3];
                c.cmps(0, c.r[29], 0x4);
                if c.crbit(0) { pc = 0x8025e4b0; continue; }
                pc = 0x8025e4ac;
            }
            0x8025e4ac => {
                c.r[29] = 0xffffffff;
                pc = 0x8025e4b0;
            }
            0x8025e4b0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x8025e4c4; f_8012640c(c);
                pc = 0x8025e4c4;
            }
            0x8025e4c4 => {
                c.r[3] = 0x1;
                c.lr = 0x8025e4cc; f_801dadc0(c);
                pc = 0x8025e4cc;
            }
            0x8025e4cc => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[5] = c.r[13].wrapping_add(0xffff8590);
                c.r[6] = 0x1;
                c.lr = 0x8025e4e0; f_8025e0b0(c);
                pc = 0x8025e4e0;
            }
            0x8025e4e0 => {
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x8025e510; continue; }
                pc = 0x8025e4e8;
            }
            0x8025e4e8 => {
                { let ea = c.r[2].wrapping_add(0xffffafe0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8025e4f4; f_801c41c8(c);
                pc = 0x8025e4f4;
            }
            0x8025e4f4 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[13].wrapping_add(0xffff8590);
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                c.lr = 0x8025e510; f_8025de54(c);
                pc = 0x8025e510;
            }
            0x8025e510 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x64));
                c.r[3] = c.r[29] as u8 as i8 as i32 as u32;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x5c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x58));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x50));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x60);
                if true { return; }
                return;
            }
            _ => panic!("f_8025e3b0: bad pc {:#x}", pc),
        }
    }
}

/// _preReliveDisplayHokoraParticleThreadFunc__Fv (0x8025e534, 192 bytes)
#[inline(never)]
pub fn f_8025e534(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025e534) { return; }
        if c.prof { crate::prof::enter(c, 0x8025e534); f_8025e534_b(c, 0x8025e534); crate::prof::exit(c, 0x8025e534); return; }
    }
    f_8025e534_b(c, 0x8025e534)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025e534_at(c: &mut Cpu, pc: u32) { f_8025e534_b(c, pc) }
#[inline(always)]
fn f_8025e534_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025e534;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025e534, pc); }
        match pc {
            0x8025e534 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80280000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[5] = c.r[3].wrapping_add(0xffffa478);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffae44));
                c.r[4] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x1);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x8));
                c.stw(c.r[1].wrapping_add(0x8), c.r[4]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                if !c.crbit(2) { pc = 0x8025e588; continue; }
                pc = 0x8025e56c;
            }
            0x8025e56c => {
                if !c.crbit(2) { pc = 0x8025e588; continue; }
                pc = 0x8025e570;
            }
            0x8025e570 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae40));
                c.r[4] = 0x1;
                c.lr = 0x8025e57c; f_80118a68(c);
                pc = 0x8025e57c;
            }
            0x8025e57c => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffae40), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffae44), c.r[0]);
                pc = 0x8025e588;
            }
            0x8025e588 => {
                c.lr = 0x8025e58c; f_80113f48(c);
                pc = 0x8025e58c;
            }
            0x8025e58c => {
                c.r[4] = 0x108a0000;
                c.r[4] = c.r[4].wrapping_add(0x1400);
                c.lr = 0x8025e598; f_800f9318(c);
                pc = 0x8025e598;
            }
            0x8025e598 => {
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x8025e5a4; f_801190dc(c);
                pc = 0x8025e5a4;
            }
            0x8025e5a4 => {
                c.stw(c.r[13].wrapping_add(0xffffae40), c.r[3]);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8025e5b0; f_80118f04(c);
                pc = 0x8025e5b0;
            }
            0x8025e5b0 => {
                c.r[0] = 0x1;
                { let ea = c.r[2].wrapping_add(0xffffafe4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[13].wrapping_add(0xffffae44), c.r[0]);
                c.lr = 0x8025e5c0; f_8025dc2c(c);
                pc = 0x8025e5c0;
            }
            0x8025e5c0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffae44));
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025e5e4; continue; }
                pc = 0x8025e5cc;
            }
            0x8025e5cc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae40));
                c.r[4] = 0x1;
                c.lr = 0x8025e5d8; f_80118a68(c);
                pc = 0x8025e5d8;
            }
            0x8025e5d8 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffae40), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffae44), c.r[0]);
                pc = 0x8025e5e4;
            }
            0x8025e5e4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025e534: bad pc {:#x}", pc),
        }
    }
}

/// preReliveDisplayHokoraParticle (0x8025e5f4, 76 bytes)
#[inline(never)]
pub fn f_8025e5f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025e5f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8025e5f4); f_8025e5f4_b(c, 0x8025e5f4); crate::prof::exit(c, 0x8025e5f4); return; }
    }
    f_8025e5f4_b(c, 0x8025e5f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025e5f4_at(c: &mut Cpu, pc: u32) { f_8025e5f4_b(c, pc) }
#[inline(always)]
fn f_8025e5f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025e5f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025e5f4, pc); }
        match pc {
            0x8025e5f4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8025e604; f_801caf0c(c);
                pc = 0x8025e604;
            }
            0x8025e604 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025e630; continue; }
                pc = 0x8025e60c;
            }
            0x8025e60c => {
                c.lr = 0x8025e610; f_800ff560(c);
                pc = 0x8025e610;
            }
            0x8025e610 => {
                c.r[5] = 0x80260000;
                c.r[4] = c.r[3] | c.r[3];
                c.r[8] = c.r[5].wrapping_add(0xffffe534);
                c.r[3] = 0x1;
                c.r[5] = 0x4000;
                c.r[6] = 0x1;
                c.r[7] = 0x1;
                c.lr = 0x8025e630; f_800f07a8(c);
                pc = 0x8025e630;
            }
            0x8025e630 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025e5f4: bad pc {:#x}", pc),
        }
    }
}

/// _expRecover__FP7PokemonUl (0x8025e640, 892 bytes)
#[inline(never)]
pub fn f_8025e640(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025e640) { return; }
        if c.prof { crate::prof::enter(c, 0x8025e640); f_8025e640_b(c, 0x8025e640); crate::prof::exit(c, 0x8025e640); return; }
    }
    f_8025e640_b(c, 0x8025e640)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025e640_at(c: &mut Cpu, pc: u32) { f_8025e640_b(c, pc) }
#[inline(always)]
fn f_8025e640_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025e640;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025e640, pc); }
        match pc {
            0x8025e640 => {
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x7a;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x64), c.r[0]);
                c.r[0] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x34); for n in 21..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[4] | c.r[4];
                c.r[23] = c.r[3] | c.r[3];
                c.r[24] = 0x0;
                c.r[4] = 0x0;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.lr = 0x8025e674; f_8012640c(c);
                pc = 0x8025e674;
            }
            0x8025e674 => {
                c.cmpu(0, c.r[26], 0x0);
                if !c.crbit(2) { pc = 0x8025e698; continue; }
                pc = 0x8025e67c;
            }
            0x8025e67c => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[7] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0xc6;
                c.r[6] = 0x0;
                c.lr = 0x8025e694; f_801254b4(c);
                pc = 0x8025e694;
            }
            0x8025e694 => {
                pc = 0x8025e9a4; continue;
                pc = 0x8025e698;
            }
            0x8025e698 => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x7a;
                c.r[6] = 0x0;
                c.lr = 0x8025e6ac; f_8012640c(c);
                pc = 0x8025e6ac;
            }
            0x8025e6ac => {
                c.r[21] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[21], 0x64);
                if !c.crbit(0) { pc = 0x8025e9a4; continue; }
                pc = 0x8025e6b8;
            }
            0x8025e6b8 => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x79;
                c.r[6] = 0x0;
                c.lr = 0x8025e6cc; f_8012640c(c);
                pc = 0x8025e6cc;
            }
            0x8025e6cc => {
                c.r[0] = c.r[21].wrapping_add(0x1);
                c.r[25] = c.r[3] | c.r[3];
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x8025e6e0; f_801229f4(c);
                pc = 0x8025e6e0;
            }
            0x8025e6e0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[23] | c.r[23];
                c.r[21] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x87;
                c.r[6] = 0x0;
                c.lr = 0x8025e6fc; f_8012640c(c);
                pc = 0x8025e6fc;
            }
            0x8025e6fc => {
                c.r[31] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x88;
                c.r[6] = 0x0;
                c.lr = 0x8025e714; f_8012640c(c);
                pc = 0x8025e714;
            }
            0x8025e714 => {
                c.r[30] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x89;
                c.r[6] = 0x0;
                c.lr = 0x8025e72c; f_8012640c(c);
                pc = 0x8025e72c;
            }
            0x8025e72c => {
                c.r[29] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x8a;
                c.r[6] = 0x0;
                c.lr = 0x8025e744; f_8012640c(c);
                pc = 0x8025e744;
            }
            0x8025e744 => {
                c.r[28] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x8b;
                c.r[6] = 0x0;
                c.lr = 0x8025e75c; f_8012640c(c);
                pc = 0x8025e75c;
            }
            0x8025e75c => {
                c.r[27] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x8c;
                c.r[6] = 0x0;
                c.lr = 0x8025e774; f_8012640c(c);
                pc = 0x8025e774;
            }
            0x8025e774 => {
                c.r[7] = c.r[25].wrapping_add(c.r[26]);
                c.r[22] = c.r[3] as u16 as i16 as i32 as u32;
                c.cmpu(0, c.r[7], c.r[21]);
                if c.crbit(0) { pc = 0x8025e988; continue; }
                pc = 0x8025e784;
            }
            0x8025e784 => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[21] | c.r[21];
                c.r[26] = c.r[7].wrapping_sub(c.r[21]);
                c.r[24] = 0x1;
                c.lr = 0x8025e798; f_8011de98(c);
                pc = 0x8025e798;
            }
            0x8025e798 => {
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x8025e7a0; f_8012546c(c);
                pc = 0x8025e7a0;
            }
            0x8025e7a0 => {
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x8025e7a8; f_80123090(c);
                pc = 0x8025e7a8;
            }
            0x8025e7a8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[0] | c.r[0];
                c.r[5] = 0x0;
                c.lr = 0x8025e7bc; f_80122370(c);
                pc = 0x8025e7bc;
            }
            0x8025e7bc => {
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x8025e7c4; f_8011f4a8(c);
                pc = 0x8025e7c4;
            }
            0x8025e7c4 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x4ca;
                c.r[25] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x8025e7dc; f_80165668(c);
                pc = 0x8025e7dc;
            }
            0x8025e7dc => {
                c.r[4] = c.r[25].rotate_left(0) & 0xff;
                c.r[3] = 0x2f;
                c.lr = 0x8025e7e8; f_80132a38(c);
                pc = 0x8025e7e8;
            }
            0x8025e7e8 => {
                c.r[3] = 0x44ce;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x8025e7f8; f_801067e8(c);
                pc = 0x8025e7f8;
            }
            0x8025e7f8 => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x87;
                c.r[6] = 0x0;
                c.lr = 0x8025e80c; f_8012640c(c);
                pc = 0x8025e80c;
            }
            0x8025e80c => {
                c.sth(c.r[1].wrapping_add(0xe), c.r[3]);
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x88;
                c.r[6] = 0x0;
                c.lr = 0x8025e824; f_8012640c(c);
                pc = 0x8025e824;
            }
            0x8025e824 => {
                c.sth(c.r[1].wrapping_add(0x10), c.r[3]);
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x89;
                c.r[6] = 0x0;
                c.lr = 0x8025e83c; f_8012640c(c);
                pc = 0x8025e83c;
            }
            0x8025e83c => {
                c.sth(c.r[1].wrapping_add(0x12), c.r[3]);
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x8a;
                c.r[6] = 0x0;
                c.lr = 0x8025e854; f_8012640c(c);
                pc = 0x8025e854;
            }
            0x8025e854 => {
                c.sth(c.r[1].wrapping_add(0x16), c.r[3]);
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x8b;
                c.r[6] = 0x0;
                c.lr = 0x8025e86c; f_8012640c(c);
                pc = 0x8025e86c;
            }
            0x8025e86c => {
                c.sth(c.r[1].wrapping_add(0x18), c.r[3]);
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x8c;
                c.r[6] = 0x0;
                c.lr = 0x8025e884; f_8012640c(c);
                pc = 0x8025e884;
            }
            0x8025e884 => {
                c.r[4] = c.lha(c.r[1].wrapping_add(0xe));
                c.r[5] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[0] = c.lha(c.r[1].wrapping_add(0x10));
                c.r[5] = c.r[5].wrapping_sub(c.r[22]);
                c.r[10] = c.r[4].wrapping_sub(c.r[31]);
                c.r[4] = c.lha(c.r[1].wrapping_add(0x12));
                c.r[9] = c.r[0].wrapping_sub(c.r[30]);
                c.r[0] = c.lha(c.r[1].wrapping_add(0x16));
                c.r[8] = c.r[4].wrapping_sub(c.r[29]);
                c.r[4] = c.lha(c.r[1].wrapping_add(0x18));
                c.r[7] = c.r[0].wrapping_sub(c.r[28]);
                c.r[0] = 0x1;
                c.r[6] = c.r[4].wrapping_sub(c.r[27]);
                c.sth(c.r[1].wrapping_add(0x14), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x1c);
                c.r[4] = 0x1;
                c.sth(c.r[1].wrapping_add(0x1e), c.r[10]);
                c.sth(c.r[1].wrapping_add(0x20), c.r[9]);
                c.sth(c.r[1].wrapping_add(0x22), c.r[8]);
                c.sth(c.r[1].wrapping_add(0x26), c.r[7]);
                c.sth(c.r[1].wrapping_add(0x28), c.r[6]);
                c.sth(c.r[1].wrapping_add(0x24), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x1c), c.r[0]);
                c.lr = 0x8025e8e4; f_80105e7c(c);
                pc = 0x8025e8e4;
            }
            0x8025e8e4 => {
                c.r[0] = 0x0;
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.stb(c.r[1].wrapping_add(0xc), c.r[0]);
                c.r[4] = 0x1;
                c.lr = 0x8025e8f8; f_80105e7c(c);
                pc = 0x8025e8f8;
            }
            0x8025e8f8 => {
                c.r[3] = 0x1;
                c.lr = 0x8025e900; f_80105c68(c);
                pc = 0x8025e900;
            }
            0x8025e900 => {
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8025e968; continue;
                pc = 0x8025e90c;
            }
            0x8025e90c => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x8025e918; f_80123b5c(c);
                pc = 0x8025e918;
            }
            0x8025e918 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0xffffffff);
                if !c.crbit(2) { pc = 0x8025e95c; continue; }
                pc = 0x8025e924;
            }
            0x8025e924 => {
                c.r[4] = 0x80260000;
                c.r[3] = c.r[23] | c.r[23];
                c.r[7] = c.r[4].wrapping_add(0xffffe3b0);
                c.r[5] = c.r[1].wrapping_add(0x9);
                c.r[4] = c.r[21] | c.r[21];
                c.r[6] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x8025e944; f_802600e4(c);
                pc = 0x8025e944;
            }
            0x8025e944 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025e95c; continue; }
                pc = 0x8025e94c;
            }
            0x8025e94c => {
                c.r[4] = c.lbz(c.r[1].wrapping_add(0x9));
                c.r[3] = c.r[23] | c.r[23];
                c.r[5] = c.r[21] | c.r[21];
                c.lr = 0x8025e95c; f_80123d58(c);
                pc = 0x8025e95c;
            }
            0x8025e95c => {
                c.r[3] = c.lbz(c.r[1].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8025e968;
            }
            0x8025e968 => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[25] | c.r[25];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8025e978; f_801236f8(c);
                pc = 0x8025e978;
            }
            0x8025e978 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[21] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x8025e90c; continue; }
                pc = 0x8025e984;
            }
            0x8025e984 => {
                pc = 0x8025e674; continue;
                pc = 0x8025e988;
            }
            0x8025e988 => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[26] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x79;
                c.r[6] = 0x0;
                c.lr = 0x8025e9a0; f_801254b4(c);
                pc = 0x8025e9a0;
            }
            0x8025e9a0 => {
                pc = 0x8025e674; continue;
                pc = 0x8025e9a4;
            }
            0x8025e9a4 => {
                c.r[3] = c.r[24] | c.r[24];
                { let mut ea = c.r[1].wrapping_add(0x34); for n in 21..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x64));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x60);
                if true { return; }
                return;
            }
            _ => panic!("f_8025e640: bad pc {:#x}", pc),
        }
    }
}

/// reliveCeremonyAll (0x8025e9bc, 912 bytes)
#[inline(never)]
pub fn f_8025e9bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025e9bc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025e9bc); f_8025e9bc_b(c, 0x8025e9bc); crate::prof::exit(c, 0x8025e9bc); return; }
    }
    f_8025e9bc_b(c, 0x8025e9bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025e9bc_at(c: &mut Cpu, pc: u32) { f_8025e9bc_b(c, pc) }
#[inline(always)]
fn f_8025e9bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025e9bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025e9bc, pc); }
        match pc {
            0x8025e9bc => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = 0x3;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[28]);
                c.sth(c.r[1].wrapping_add(0x8), c.r[0]);
                c.lr = 0x8025e9f4; f_8012a5b0(c);
                pc = 0x8025e9f4;
            }
            0x8025e9f4 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8025e9fc; f_80123fbc(c);
                pc = 0x8025e9fc;
            }
            0x8025e9fc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025ed2c; continue; }
                pc = 0x8025ea08;
            }
            0x8025ea08 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x8025ea1c; f_8012640c(c);
                pc = 0x8025ea1c;
            }
            0x8025ea1c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025ea24; f_8011ee40(c);
                pc = 0x8025ea24;
            }
            0x8025ea24 => {
                c.r[4] = 0x1;
                c.lr = 0x8025ea2c; f_801eecd8(c);
                pc = 0x8025ea2c;
            }
            0x8025ea2c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3e;
                c.lr = 0x8025ea38; f_80121adc(c);
                pc = 0x8025ea38;
            }
            0x8025ea38 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025ea4c; continue; }
                pc = 0x8025ea40;
            }
            0x8025ea40 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3e;
                c.lr = 0x8025ea4c; f_80121b4c(c);
                pc = 0x8025ea4c;
            }
            0x8025ea4c => {
                { let ea = c.r[2].wrapping_add(0xffffafe8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025ea58; f_8011fbcc(c);
                pc = 0x8025ea58;
            }
            0x8025ea58 => {
                c.r[3] = 0x3f7;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x8025ea68; f_80165668(c);
                pc = 0x8025ea68;
            }
            0x8025ea68 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8025ea74; f_8011f228(c);
                pc = 0x8025ea74;
            }
            0x8025ea74 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[29] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x8025eab4; continue; }
                pc = 0x8025ea80;
            }
            0x8025ea80 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025ea88; f_8011f4f0(c);
                pc = 0x8025ea88;
            }
            0x8025ea88 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x32;
                c.r[4] = c.r[0] | c.r[0];
                c.lr = 0x8025ea98; f_80132a38(c);
                pc = 0x8025ea98;
            }
            0x8025ea98 => {
                c.r[4] = c.r[29].rotate_left(0) & 0xffff;
                c.r[3] = 0x39;
                c.lr = 0x8025eaa4; f_80132a38(c);
                pc = 0x8025eaa4;
            }
            0x8025eaa4 => {
                c.r[3] = 0x3b10;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x8025eab4; f_801067e8(c);
                pc = 0x8025eab4;
            }
            0x8025eab4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025eabc; f_8011ee10(c);
                pc = 0x8025eabc;
            }
            0x8025eabc => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x2f;
                c.r[28] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025ead0; f_80132a38(c);
                pc = 0x8025ead0;
            }
            0x8025ead0 => {
                c.r[3] = 0x3b0b;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x8025eae0; f_801067e8(c);
                pc = 0x8025eae0;
            }
            0x8025eae0 => {
                c.r[3] = 0x1;
                c.lr = 0x8025eae8; f_801065b8(c);
                pc = 0x8025eae8;
            }
            0x8025eae8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025eaf0; f_8011ee58(c);
                pc = 0x8025eaf0;
            }
            0x8025eaf0 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025eafc; f_8011edf8(c);
                pc = 0x8025eafc;
            }
            0x8025eafc => {
                c.r[0] = c.r[3].wrapping_add(0x46);
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.r[29].wrapping_add(c.r[0]);
                c.r[4] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x8025eb10; f_8011d904(c);
                pc = 0x8025eb10;
            }
            0x8025eb10 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8025eb1c; f_8011de68(c);
                pc = 0x8025eb1c;
            }
            0x8025eb1c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8025eb28; f_8025e640(c);
                pc = 0x8025eb28;
            }
            0x8025eb28 => {
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x8025ec64; continue; }
                pc = 0x8025eb30;
            }
            0x8025eb30 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[7] = c.r[1].wrapping_add(0x18);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x8025eb48; f_80128a64(c);
                pc = 0x8025eb48;
            }
            0x8025eb48 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[28] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x8025ec64; continue; }
                pc = 0x8025eb54;
            }
            0x8025eb54 => {
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x8025ec64; continue; }
                pc = 0x8025eb5c;
            }
            0x8025eb5c => {
                { let ea = c.r[2].wrapping_add(0xffffafe0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x3;
                c.lr = 0x8025eb68; f_801c41c8(c);
                pc = 0x8025eb68;
            }
            0x8025eb68 => {
                c.r[3] = 0x1;
                c.lr = 0x8025eb70; f_801c40f0(c);
                pc = 0x8025eb70;
            }
            0x8025eb70 => {
                c.lr = 0x8025eb74; f_801dac90(c);
                pc = 0x8025eb74;
            }
            0x8025eb74 => {
                c.r[5] = c.lhz(c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[1].wrapping_add(0x18);
                c.r[7] = 0x0;
                c.r[8] = 0x1;
                c.r[9] = 0x1;
                c.r[10] = 0x0;
                c.lr = 0x8025eb98; f_8012805c(c);
                pc = 0x8025eb98;
            }
            0x8025eb98 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8025ec04; continue; }
                pc = 0x8025eba0;
            }
            0x8025eba0 => {
                c.r[3] = 0x1;
                c.lr = 0x8025eba8; f_801dadc0(c);
                pc = 0x8025eba8;
            }
            0x8025eba8 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.r[5] = c.r[13].wrapping_add(0xffff8590);
                c.r[6] = 0x1;
                c.lr = 0x8025ebbc; f_8025e0b0(c);
                pc = 0x8025ebbc;
            }
            0x8025ebbc => {
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x8025ebec; continue; }
                pc = 0x8025ebc4;
            }
            0x8025ebc4 => {
                { let ea = c.r[2].wrapping_add(0xffffafe0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8025ebd0; f_801c41c8(c);
                pc = 0x8025ebd0;
            }
            0x8025ebd0 => {
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.r[4] = c.r[13].wrapping_add(0xffff8590);
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                c.lr = 0x8025ebec; f_8025de54(c);
                pc = 0x8025ebec;
            }
            0x8025ebec => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025ebf4; f_8011f4f0(c);
                pc = 0x8025ebf4;
            }
            0x8025ebf4 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x32;
                c.lr = 0x8025ec00; f_80132a38(c);
                pc = 0x8025ec00;
            }
            0x8025ec00 => {
                pc = 0x8025ec64; continue;
                pc = 0x8025ec04;
            }
            0x8025ec04 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025ec0c; f_8011f4f0(c);
                pc = 0x8025ec0c;
            }
            0x8025ec0c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x32;
                c.lr = 0x8025ec18; f_80132a38(c);
                pc = 0x8025ec18;
            }
            0x8025ec18 => {
                c.r[3] = 0x1;
                c.lr = 0x8025ec20; f_801dadc0(c);
                pc = 0x8025ec20;
            }
            0x8025ec20 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.r[5] = c.r[13].wrapping_add(0xffff8590);
                c.r[6] = 0x1;
                c.lr = 0x8025ec34; f_8025e0b0(c);
                pc = 0x8025ec34;
            }
            0x8025ec34 => {
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x8025ec64; continue; }
                pc = 0x8025ec3c;
            }
            0x8025ec3c => {
                { let ea = c.r[2].wrapping_add(0xffffafe0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8025ec48; f_801c41c8(c);
                pc = 0x8025ec48;
            }
            0x8025ec48 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.r[4] = c.r[13].wrapping_add(0xffff8590);
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                c.lr = 0x8025ec64; f_8025de54(c);
                pc = 0x8025ec64;
            }
            0x8025ec64 => {
                c.r[3] = 0x3ca;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x8025ec74; f_80165668(c);
                pc = 0x8025ec74;
            }
            0x8025ec74 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025ec7c; f_80265e7c(c);
                pc = 0x8025ec7c;
            }
            0x8025ec7c => {
                c.r[3] = 0x3b0c;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x8025ec8c; f_801067e8(c);
                pc = 0x8025ec8c;
            }
            0x8025ec8c => {
                c.r[3] = 0x3b0d;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x8025ec9c; f_801067e8(c);
                pc = 0x8025ec9c;
            }
            0x8025ec9c => {
                c.lr = 0x8025eca0; f_8001e184(c);
                pc = 0x8025eca0;
            }
            0x8025eca0 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025ecb4; continue; }
                pc = 0x8025eca8;
            }
            0x8025eca8 => {
                c.r[3] = 0x1;
                c.lr = 0x8025ecb0; f_801065b8(c);
                pc = 0x8025ecb0;
            }
            0x8025ecb0 => {
                pc = 0x8025ed2c; continue;
                pc = 0x8025ecb4;
            }
            0x8025ecb4 => {
                c.r[3] = 0x1;
                c.lr = 0x8025ecbc; f_801065b8(c);
                pc = 0x8025ecbc;
            }
            0x8025ecbc => {
                { let ea = c.r[2].wrapping_add(0xffffafe0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x3;
                c.lr = 0x8025ecc8; f_801c41c8(c);
                pc = 0x8025ecc8;
            }
            0x8025ecc8 => {
                c.r[3] = 0x1;
                c.lr = 0x8025ecd0; f_801c40f0(c);
                pc = 0x8025ecd0;
            }
            0x8025ecd0 => {
                c.lr = 0x8025ecd4; f_801dac90(c);
                pc = 0x8025ecd4;
            }
            0x8025ecd4 => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x2;
                c.lr = 0x8025ece0; f_80029660(c);
                pc = 0x8025ece0;
            }
            0x8025ece0 => {
                c.r[3] = 0x1;
                c.lr = 0x8025ece8; f_801dadc0(c);
                pc = 0x8025ece8;
            }
            0x8025ece8 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[5] = c.r[13].wrapping_add(0xffff8590);
                c.r[6] = 0x1;
                c.lr = 0x8025ecfc; f_8025e0b0(c);
                pc = 0x8025ecfc;
            }
            0x8025ecfc => {
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x8025ed2c; continue; }
                pc = 0x8025ed04;
            }
            0x8025ed04 => {
                { let ea = c.r[2].wrapping_add(0xffffafe0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8025ed10; f_801c41c8(c);
                pc = 0x8025ed10;
            }
            0x8025ed10 => {
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = c.r[13].wrapping_add(0xffff8590);
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                c.lr = 0x8025ed2c; f_8025de54(c);
                pc = 0x8025ed2c;
            }
            0x8025ed2c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x20));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8025e9bc: bad pc {:#x}", pc),
        }
    }
}

/// reliveMain (0x8025ed4c, 524 bytes)
#[inline(never)]
pub fn f_8025ed4c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025ed4c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025ed4c); f_8025ed4c_b(c, 0x8025ed4c); crate::prof::exit(c, 0x8025ed4c); return; }
    }
    f_8025ed4c_b(c, 0x8025ed4c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025ed4c_at(c: &mut Cpu, pc: u32) { f_8025ed4c_b(c, pc) }
#[inline(always)]
fn f_8025ed4c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025ed4c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025ed4c, pc); }
        match pc {
            0x8025ed4c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xffff82bc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025ed78; continue; }
                pc = 0x8025ed70;
            }
            0x8025ed70 => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025ef40; continue; }
                pc = 0x8025ed78;
            }
            0x8025ed78 => {
                c.r[3] = 0x1;
                c.lr = 0x8025ed80; f_801dadc0(c);
                pc = 0x8025ed80;
            }
            0x8025ed80 => {
                c.r[3] = 0x80480000;
                c.r[31] = c.r[3].wrapping_add(0xffff82bc);
                c.r[0] = c.lwz(c.r[31]);
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8025ee30; continue; }
                pc = 0x8025ed94;
            }
            0x8025ed94 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.r[5] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x8025eda8; f_8012a5b0(c);
                pc = 0x8025eda8;
            }
            0x8025eda8 => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x8025edb0; f_80123fbc(c);
                pc = 0x8025edb0;
            }
            0x8025edb0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025edc4; continue; }
                pc = 0x8025edbc;
            }
            0x8025edbc => {
                c.r[0] = 0x0;
                pc = 0x8025ee1c; continue;
                pc = 0x8025edc4;
            }
            0x8025edc4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x8025edd8; f_8012640c(c);
                pc = 0x8025edd8;
            }
            0x8025edd8 => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[5] = c.r[13].wrapping_add(0xffff8588);
                c.r[6] = 0x2;
                c.lr = 0x8025edec; f_8025e0b0(c);
                pc = 0x8025edec;
            }
            0x8025edec => {
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x8025ee18; continue; }
                pc = 0x8025edf4;
            }
            0x8025edf4 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[13].wrapping_add(0xffff8588);
                c.r[5] = 0x2;
                c.r[6] = 0x1;
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                c.lr = 0x8025ee10; f_8025de54(c);
                pc = 0x8025ee10;
            }
            0x8025ee10 => {
                c.r[0] = 0x1;
                pc = 0x8025ee1c; continue;
                pc = 0x8025ee18;
            }
            0x8025ee18 => {
                c.r[0] = 0x0;
                pc = 0x8025ee1c;
            }
            0x8025ee1c => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025eef0; continue; }
                pc = 0x8025ee24;
            }
            0x8025ee24 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.lr = 0x8025ee2c; f_8025e9bc(c);
                pc = 0x8025ee2c;
            }
            0x8025ee2c => {
                pc = 0x8025eef0; continue;
                pc = 0x8025ee30;
            }
            0x8025ee30 => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025eef0; continue; }
                pc = 0x8025ee38;
            }
            0x8025ee38 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.r[5] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x8025ee4c; f_8012a5b0(c);
                pc = 0x8025ee4c;
            }
            0x8025ee4c => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x8025ee54; f_80123fbc(c);
                pc = 0x8025ee54;
            }
            0x8025ee54 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8025ee68; continue; }
                pc = 0x8025ee60;
            }
            0x8025ee60 => {
                c.r[0] = 0x0;
                pc = 0x8025eee0; continue;
                pc = 0x8025ee68;
            }
            0x8025ee68 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xc5;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8025ee80; f_801254b4(c);
                pc = 0x8025ee80;
            }
            0x8025ee80 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x8025ee94; f_8012640c(c);
                pc = 0x8025ee94;
            }
            0x8025ee94 => {
                c.r[3] = 0x803a0000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[3].wrapping_add(0xffffa6a8);
                c.r[6] = 0x4;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8025eeac; f_8025e0b0(c);
                pc = 0x8025eeac;
            }
            0x8025eeac => {
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x8025eedc; continue; }
                pc = 0x8025eeb4;
            }
            0x8025eeb4 => {
                c.r[4] = 0x803a0000;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[4].wrapping_add(0xffffa6a8);
                c.r[5] = 0x4;
                c.r[6] = 0x1;
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                c.lr = 0x8025eed4; f_8025de54(c);
                pc = 0x8025eed4;
            }
            0x8025eed4 => {
                c.r[0] = 0x1;
                pc = 0x8025eee0; continue;
                pc = 0x8025eedc;
            }
            0x8025eedc => {
                c.r[0] = 0x0;
                pc = 0x8025eee0;
            }
            0x8025eee0 => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025eef0; continue; }
                pc = 0x8025eee8;
            }
            0x8025eee8 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.lr = 0x8025eef0; f_8025e9bc(c);
                pc = 0x8025eef0;
            }
            0x8025eef0 => {
                { let ea = c.r[2].wrapping_add(0xffffafe0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x3;
                c.lr = 0x8025eefc; f_801c41c8(c);
                pc = 0x8025eefc;
            }
            0x8025eefc => {
                c.r[3] = 0x1;
                c.lr = 0x8025ef04; f_801c40f0(c);
                pc = 0x8025ef04;
            }
            0x8025ef04 => {
                c.lr = 0x8025ef08; f_801dac90(c);
                pc = 0x8025ef08;
            }
            0x8025ef08 => {
                c.r[5] = 0xffffffff;
                c.r[3] = 0x80480000;
                { let ea = c.r[3].wrapping_add(0xffff82bc); c.stw(ea, c.r[5]); c.r[3] = ea; }
                c.r[0] = 0x0;
                c.r[4] = c.r[13].wrapping_add(0xffffae48);
                c.stw(c.r[13].wrapping_add(0xffffae48), c.r[5]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                c.sth(c.r[3].wrapping_add(0x4), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.lr = 0x8025ef30; f_800ff660(c);
                pc = 0x8025ef30;
            }
            0x8025ef30 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8025ef3c; f_8011288c(c);
                pc = 0x8025ef3c;
            }
            0x8025ef3c => {
                c.lr = 0x8025ef40; crate::stubs::f_800f0308(c);
                pc = 0x8025ef40;
            }
            0x8025ef40 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8025ed4c: bad pc {:#x}", pc),
        }
    }
}

/// preReliveMain (0x8025ef58, 852 bytes)
#[inline(never)]
pub fn f_8025ef58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025ef58) { return; }
        if c.prof { crate::prof::enter(c, 0x8025ef58); f_8025ef58_b(c, 0x8025ef58); crate::prof::exit(c, 0x8025ef58); return; }
    }
    f_8025ef58_b(c, 0x8025ef58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025ef58_at(c: &mut Cpu, pc: u32) { f_8025ef58_b(c, pc) }
#[inline(always)]
fn f_8025ef58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025ef58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025ef58, pc); }
        match pc {
            0x8025ef58 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[28]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffae48));
                c.cmps(0, c.r[0], 0xffffffff);
                if c.crbit(2) { pc = 0x8025f28c; continue; }
                pc = 0x8025ef80;
            }
            0x8025ef80 => {
                c.r[29] = 0x0;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8025ef90; f_80129280(c);
                pc = 0x8025ef90;
            }
            0x8025ef90 => {
                c.r[3] = 0x0;
                c.r[4] = 0x2;
                c.lr = 0x8025ef9c; f_80129280(c);
                pc = 0x8025ef9c;
            }
            0x8025ef9c => {
                c.r[28] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025eff4; continue;
                pc = 0x8025efa8;
            }
            0x8025efa8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.lr = 0x8025efb4; f_8012ac08(c);
                pc = 0x8025efb4;
            }
            0x8025efb4 => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x8025efbc; f_80123fbc(c);
                pc = 0x8025efbc;
            }
            0x8025efbc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025eff0; continue; }
                pc = 0x8025efc4;
            }
            0x8025efc4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8025efcc; f_8011fc74(c);
                pc = 0x8025efcc;
            }
            0x8025efcc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025eff0; continue; }
                pc = 0x8025efd8;
            }
            0x8025efd8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8025efe0; f_8011ee40(c);
                pc = 0x8025efe0;
            }
            0x8025efe0 => {
                c.lr = 0x8025efe4; f_801eec74(c);
                pc = 0x8025efe4;
            }
            0x8025efe4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025eff0; continue; }
                pc = 0x8025efec;
            }
            0x8025efec => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x8025eff0;
            }
            0x8025eff0 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x8025eff4;
            }
            0x8025eff4 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x6);
                if c.crbit(0) { pc = 0x8025efa8; continue; }
                pc = 0x8025f000;
            }
            0x8025f000 => {
                c.lr = 0x8025f004; f_801caf0c(c);
                pc = 0x8025f004;
            }
            0x8025f004 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8025f014; f_80129280(c);
                pc = 0x8025f014;
            }
            0x8025f014 => {
                c.r[3] = 0x0;
                c.r[4] = 0x2;
                c.lr = 0x8025f020; f_80129280(c);
                pc = 0x8025f020;
            }
            0x8025f020 => {
                c.r[3] = 0x0;
                c.r[4] = 0x219;
                c.lr = 0x8025f02c; f_80129b2c(c);
                pc = 0x8025f02c;
            }
            0x8025f02c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025f060; continue; }
                pc = 0x8025f038;
            }
            0x8025f038 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025f058; continue; }
                pc = 0x8025f040;
            }
            0x8025f040 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8025f050; continue; }
                pc = 0x8025f048;
            }
            0x8025f048 => {
                c.r[0] = 0x2;
                pc = 0x8025f074; continue;
                pc = 0x8025f050;
            }
            0x8025f050 => {
                c.r[0] = 0x1;
                pc = 0x8025f074; continue;
                pc = 0x8025f058;
            }
            0x8025f058 => {
                c.r[0] = 0x3;
                pc = 0x8025f074; continue;
                pc = 0x8025f060;
            }
            0x8025f060 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8025f070; continue; }
                pc = 0x8025f068;
            }
            0x8025f068 => {
                c.r[0] = 0x4;
                pc = 0x8025f074; continue;
                pc = 0x8025f070;
            }
            0x8025f070 => {
                c.r[0] = 0x5;
                pc = 0x8025f074;
            }
            0x8025f074 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae48));
                c.r[4] = c.r[0] | c.r[0];
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8025f0b8; continue; }
                pc = 0x8025f084;
            }
            0x8025f084 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025f098; continue; }
                pc = 0x8025f090;
            }
            0x8025f090 => {
                c.r[4] = 0x5;
                pc = 0x8025f0c4; continue;
                pc = 0x8025f098;
            }
            0x8025f098 => {
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x8025f0a8; continue; }
                pc = 0x8025f0a0;
            }
            0x8025f0a0 => {
                c.r[4] = 0x4;
                pc = 0x8025f0c4; continue;
                pc = 0x8025f0a8;
            }
            0x8025f0a8 => {
                c.cmpu(0, c.r[0], 0x3);
                if !c.crbit(2) { pc = 0x8025f0c4; continue; }
                pc = 0x8025f0b0;
            }
            0x8025f0b0 => {
                c.r[4] = 0x5;
                pc = 0x8025f0c4; continue;
                pc = 0x8025f0b8;
            }
            0x8025f0b8 => {
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x8025f0c4; continue; }
                pc = 0x8025f0c0;
            }
            0x8025f0c0 => {
                c.r[4] = 0x2;
                pc = 0x8025f0c4;
            }
            0x8025f0c4 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x8025f174; continue; }
                pc = 0x8025f0d0;
            }
            0x8025f0d0 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8025f0d8; f_8025dd14(c);
                pc = 0x8025f0d8;
            }
            0x8025f0d8 => {
                c.r[3] = 0x3c8;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x8025f0e8; f_80165668(c);
                pc = 0x8025f0e8;
            }
            0x8025f0e8 => {
                c.r[28] = c.r[13].wrapping_add(0xffffae48);
                c.r[3] = 0x0;
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x4));
                c.r[4] = 0x3;
                c.r[5] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x8025f100; f_8012a5b0(c);
                pc = 0x8025f100;
            }
            0x8025f100 => {
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x8025f110; f_8012640c(c);
                pc = 0x8025f110;
            }
            0x8025f110 => {
                c.r[5] = 0x1;
                c.r[4] = 0x80480000;
                { let ea = c.r[4].wrapping_add(0xffff82bc); c.stw(ea, c.r[5]); c.r[4] = ea; }
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x4));
                c.sth(c.r[4].wrapping_add(0x4), c.r[3]);
                c.r[3] = 0x3;
                { let ea = c.r[2].wrapping_add(0xffffafe0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[4].wrapping_add(0x8), c.r[0]);
                c.lr = 0x8025f134; f_801c41c8(c);
                pc = 0x8025f134;
            }
            0x8025f134 => {
                c.r[3] = 0x1;
                c.lr = 0x8025f13c; f_801c40f0(c);
                pc = 0x8025f13c;
            }
            0x8025f13c => {
                c.r[3] = 0x385;
                c.lr = 0x8025f144; f_800ff730(c);
                pc = 0x8025f144;
            }
            0x8025f144 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8025f150; f_8011288c(c);
                pc = 0x8025f150;
            }
            0x8025f150 => {
                c.lr = 0x8025f154; crate::stubs::f_800f0308(c);
                pc = 0x8025f154;
            }
            0x8025f154 => {
                { let ea = c.r[2].wrapping_add(0xffffafec); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8025f160; f_801c41c8(c);
                pc = 0x8025f160;
            }
            0x8025f160 => {
                c.r[3] = 0x1;
                c.lr = 0x8025f168; f_801c40f0(c);
                pc = 0x8025f168;
            }
            0x8025f168 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8025f170; f_8025dcbc(c);
                pc = 0x8025f170;
            }
            0x8025f170 => {
                pc = 0x8025f28c; continue;
                pc = 0x8025f174;
            }
            0x8025f174 => {
                c.cmpu(0, c.r[0], 0x4);
                if !c.crbit(2) { pc = 0x8025f27c; continue; }
                pc = 0x8025f17c;
            }
            0x8025f17c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffae44));
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8025f1a0; continue; }
                pc = 0x8025f188;
            }
            0x8025f188 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae40));
                c.r[4] = 0x1;
                c.lr = 0x8025f194; f_80118a68(c);
                pc = 0x8025f194;
            }
            0x8025f194 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffae40), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffae44), c.r[0]);
                pc = 0x8025f1a0;
            }
            0x8025f1a0 => {
                c.lr = 0x8025f1a4; f_801caf0c(c);
                pc = 0x8025f1a4;
            }
            0x8025f1a4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025f1c0; continue; }
                pc = 0x8025f1ac;
            }
            0x8025f1ac => {
                c.r[0] = 0xffffffff;
                c.r[3] = c.r[13].wrapping_add(0xffffae48);
                c.stw(c.r[13].wrapping_add(0xffffae48), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                pc = 0x8025f28c; continue;
                pc = 0x8025f1c0;
            }
            0x8025f1c0 => {
                c.r[3] = 0x3b0f;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x8025f1d0; f_801067e8(c);
                pc = 0x8025f1d0;
            }
            0x8025f1d0 => {
                c.r[3] = 0x1;
                c.lr = 0x8025f1d8; f_801065b8(c);
                pc = 0x8025f1d8;
            }
            0x8025f1d8 => {
                c.r[3] = 0x7;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x8025f1e8; f_8001bdf4(c);
                pc = 0x8025f1e8;
            }
            0x8025f1e8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmps(0, c.r[31], 0xffffffff);
                if c.crbit(2) { pc = 0x8025f28c; continue; }
                pc = 0x8025f1f4;
            }
            0x8025f1f4 => {
                c.r[5] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x8025f204; f_8012a5b0(c);
                pc = 0x8025f204;
            }
            0x8025f204 => {
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x8025f214; f_8012640c(c);
                pc = 0x8025f214;
            }
            0x8025f214 => {
                c.r[0] = 0x0;
                c.r[4] = 0x80480000;
                { let ea = c.r[4].wrapping_add(0xffff82bc); c.stw(ea, c.r[0]); c.r[4] = ea; }
                { let ea = c.r[2].wrapping_add(0xffffafe0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.sth(c.r[4].wrapping_add(0x4), c.r[3]);
                c.r[3] = 0x3;
                c.stw(c.r[4].wrapping_add(0x8), c.r[31]);
                c.lr = 0x8025f234; f_801c41c8(c);
                pc = 0x8025f234;
            }
            0x8025f234 => {
                c.r[3] = 0x1;
                c.lr = 0x8025f23c; f_801c40f0(c);
                pc = 0x8025f23c;
            }
            0x8025f23c => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8025f244; f_8025dd14(c);
                pc = 0x8025f244;
            }
            0x8025f244 => {
                c.r[3] = 0x385;
                c.lr = 0x8025f24c; f_800ff730(c);
                pc = 0x8025f24c;
            }
            0x8025f24c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8025f258; f_8011288c(c);
                pc = 0x8025f258;
            }
            0x8025f258 => {
                c.lr = 0x8025f25c; crate::stubs::f_800f0308(c);
                pc = 0x8025f25c;
            }
            0x8025f25c => {
                { let ea = c.r[2].wrapping_add(0xffffafec); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8025f268; f_801c41c8(c);
                pc = 0x8025f268;
            }
            0x8025f268 => {
                c.r[3] = 0x1;
                c.lr = 0x8025f270; f_801c40f0(c);
                pc = 0x8025f270;
            }
            0x8025f270 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8025f278; f_8025dcbc(c);
                pc = 0x8025f278;
            }
            0x8025f278 => {
                pc = 0x8025f28c; continue;
                pc = 0x8025f27c;
            }
            0x8025f27c => {
                c.r[0] = 0xffffffff;
                c.r[3] = c.r[13].wrapping_add(0xffffae48);
                c.stw(c.r[13].wrapping_add(0xffffae48), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                pc = 0x8025f28c;
            }
            0x8025f28c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x20));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8025ef58: bad pc {:#x}", pc),
        }
    }
}

/// preReliveInit (0x8025f2ac, 20 bytes)
#[inline(never)]
pub fn f_8025f2ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f2ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f2ac); f_8025f2ac_b(c, 0x8025f2ac); crate::prof::exit(c, 0x8025f2ac); return; }
    }
    f_8025f2ac_b(c, 0x8025f2ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f2ac_at(c: &mut Cpu, pc: u32) { f_8025f2ac_b(c, pc) }
#[inline(always)]
fn f_8025f2ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f2ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f2ac, pc); }
        match pc {
            0x8025f2ac => {
                c.r[0] = 0xffffffff;
                c.r[3] = c.r[13].wrapping_add(0xffffae48);
                c.stw(c.r[13].wrapping_add(0xffffae48), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f2ac: bad pc {:#x}", pc),
        }
    }
}

/// preReliveSetParameter (0x8025f2c0, 60 bytes)
#[inline(never)]
pub fn f_8025f2c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f2c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f2c0); f_8025f2c0_b(c, 0x8025f2c0); crate::prof::exit(c, 0x8025f2c0); return; }
    }
    f_8025f2c0_b(c, 0x8025f2c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f2c0_at(c: &mut Cpu, pc: u32) { f_8025f2c0_b(c, pc) }
#[inline(always)]
fn f_8025f2c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f2c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f2c0, pc); }
        match pc {
            0x8025f2c0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = c.r[13].wrapping_add(0xffffae48);
                c.cmps(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffae48), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x4), c.r[4]);
                if c.crbit(2) { pc = 0x8025f2e8; continue; }
                pc = 0x8025f2e0;
            }
            0x8025f2e0 => {
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x8025f2ec; continue; }
                pc = 0x8025f2e8;
            }
            0x8025f2e8 => {
                c.lr = 0x8025f2ec; f_8025ef58(c);
                pc = 0x8025f2ec;
            }
            0x8025f2ec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f2c0: bad pc {:#x}", pc),
        }
    }
}

/// ShortCommandProc (0x8025f2fc, 84 bytes)
#[inline(never)]
pub fn f_8025f2fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f2fc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f2fc); f_8025f2fc_b(c, 0x8025f2fc); crate::prof::exit(c, 0x8025f2fc); return; }
    }
    f_8025f2fc_b(c, 0x8025f2fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f2fc_at(c: &mut Cpu, pc: u32) { f_8025f2fc_b(c, pc) }
#[inline(always)]
fn f_8025f2fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f2fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f2fc, pc); }
        match pc {
            0x8025f2fc => {
                c.r[4] = 0x80480000;
                c.r[3] = c.r[3].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[4].wrapping_add(0xffff83e0);
                c.r[3] = c.r[0].wrapping_add(c.r[3]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x20));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { return; }
                pc = 0x8025f318;
            }
            0x8025f318 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x5));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8025f330; continue; }
                pc = 0x8025f324;
            }
            0x8025f324 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x6));
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x8025f33c; continue; }
                pc = 0x8025f330;
            }
            0x8025f330 => {
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x20), c.r[0]);
                if true { return; }
                pc = 0x8025f33c;
            }
            0x8025f33c => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x7));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x14));
                c.r[0] = c.r[0] & 0x3a;
                c.rc(c.r[0]);
                c.stb(c.r[3], c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f2fc: bad pc {:#x}", pc),
        }
    }
}

/// GBAInit (0x8025f350, 164 bytes)
#[inline(never)]
pub fn f_8025f350(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f350) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f350); f_8025f350_b(c, 0x8025f350); crate::prof::exit(c, 0x8025f350); return; }
    }
    f_8025f350_b(c, 0x8025f350)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f350_at(c: &mut Cpu, pc: u32) { f_8025f350_b(c, pc) }
#[inline(always)]
fn f_8025f350_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f350;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f350, pc); }
        match pc {
            0x8025f350 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80000000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[4] = 0x80480000;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4].wrapping_add(0xffff83e0);
                c.r[27] = 0x0;
                c.r[31] = 0x0;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[3] = 0x431c0000;
                c.r[3] = c.r[3].wrapping_add(0xffffde83);
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[0] = mulhwu(c.r[3], c.r[0]);
                c.r[0] = c.r[0].rotate_left(17) & 0x1ffff;
                c.r[0] = c.r[0].wrapping_mul(0x3c);
                c.r[3] = 0x80480000;
                c.r[28] = c.r[3].wrapping_add(0xffff82e0);
                c.r[30] = c.r[0].rotate_left(29) & 0x1fffffff;
                pc = 0x8025f39c;
            }
            0x8025f39c => {
                c.stw(c.r[29].wrapping_add(0x34), c.r[30]);
                c.r[3] = c.r[29].wrapping_add(0x24);
                c.stw(c.r[29].wrapping_add(0x30), c.r[31]);
                c.lr = 0x8025f3ac; f_800a13e8(c);
                pc = 0x8025f3ac;
            }
            0x8025f3ac => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                c.stw(c.r[29].wrapping_add(0xf8), c.r[28]);
                c.cmps(0, c.r[27], 0x4);
                c.r[29] = c.r[29].wrapping_add(0x100);
                c.r[28] = c.r[28].wrapping_add(0x40);
                if c.crbit(0) { pc = 0x8025f39c; continue; }
                pc = 0x8025f3c4;
            }
            0x8025f3c4 => {
                c.lr = 0x8025f3c8; f_8009a27c(c);
                pc = 0x8025f3c8;
            }
            0x8025f3c8 => {
                c.lr = 0x8025f3cc; f_800ae7e0(c);
                pc = 0x8025f3cc;
            }
            0x8025f3cc => {
                c.r[0] = 0x0;
                c.r[3] = 0x803a0000;
                c.stw(c.r[13].wrapping_add(0xffffae50), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0xffffa6b8);
                c.lr = 0x8025f3e0; f_8009fe38(c);
                pc = 0x8025f3e0;
            }
            0x8025f3e0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f350: bad pc {:#x}", pc),
        }
    }
}

/// GBAGetStatus (0x8025f3f4, 144 bytes)
#[inline(never)]
pub fn f_8025f3f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f3f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f3f4); f_8025f3f4_b(c, 0x8025f3f4); crate::prof::exit(c, 0x8025f3f4); return; }
    }
    f_8025f3f4_b(c, 0x8025f3f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f3f4_at(c: &mut Cpu, pc: u32) { f_8025f3f4_b(c, pc) }
#[inline(always)]
fn f_8025f3f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f3f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f3f4, pc); }
        match pc {
            0x8025f3f4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x80480000;
                c.r[5] = c.r[31].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[3].wrapping_add(0xffff83e0);
                c.r[7] = c.r[0].wrapping_add(c.r[5]);
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025f42c; continue; }
                pc = 0x8025f424;
            }
            0x8025f424 => {
                c.r[3] = 0x2;
                pc = 0x8025f45c; continue;
                pc = 0x8025f42c;
            }
            0x8025f42c => {
                c.r[0] = 0x0;
                c.stb(c.r[7], c.r[0]);
                c.r[5] = 0x80260000;
                c.r[0] = c.r[5].wrapping_add(0xfffff7e8);
                c.stw(c.r[7].wrapping_add(0x14), c.r[4]);
                c.r[3] = 0x80260000;
                c.r[6] = c.r[3].wrapping_add(0xfffff2fc);
                c.stw(c.r[7].wrapping_add(0x1c), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x1;
                c.r[5] = 0x3;
                c.lr = 0x8025f45c; f_8025f9ac(c);
                pc = 0x8025f45c;
            }
            0x8025f45c => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025f468; continue; }
                pc = 0x8025f464;
            }
            0x8025f464 => {
                pc = 0x8025f470; continue;
                pc = 0x8025f468;
            }
            0x8025f468 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025f470; f_8025f81c(c);
                pc = 0x8025f470;
            }
            0x8025f470 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f3f4: bad pc {:#x}", pc),
        }
    }
}

/// GBAReset (0x8025f484, 144 bytes)
#[inline(never)]
pub fn f_8025f484(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f484) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f484); f_8025f484_b(c, 0x8025f484); crate::prof::exit(c, 0x8025f484); return; }
    }
    f_8025f484_b(c, 0x8025f484)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f484_at(c: &mut Cpu, pc: u32) { f_8025f484_b(c, pc) }
#[inline(always)]
fn f_8025f484_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f484;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f484, pc); }
        match pc {
            0x8025f484 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x80480000;
                c.r[5] = c.r[31].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[3].wrapping_add(0xffff83e0);
                c.r[7] = c.r[0].wrapping_add(c.r[5]);
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025f4bc; continue; }
                pc = 0x8025f4b4;
            }
            0x8025f4b4 => {
                c.r[3] = 0x2;
                pc = 0x8025f4ec; continue;
                pc = 0x8025f4bc;
            }
            0x8025f4bc => {
                c.r[0] = 0xff;
                c.stb(c.r[7], c.r[0]);
                c.r[5] = 0x80260000;
                c.r[0] = c.r[5].wrapping_add(0xfffff7e8);
                c.stw(c.r[7].wrapping_add(0x14), c.r[4]);
                c.r[3] = 0x80260000;
                c.r[6] = c.r[3].wrapping_add(0xfffff2fc);
                c.stw(c.r[7].wrapping_add(0x1c), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x1;
                c.r[5] = 0x3;
                c.lr = 0x8025f4ec; f_8025f9ac(c);
                pc = 0x8025f4ec;
            }
            0x8025f4ec => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025f4f8; continue; }
                pc = 0x8025f4f4;
            }
            0x8025f4f4 => {
                pc = 0x8025f500; continue;
                pc = 0x8025f4f8;
            }
            0x8025f4f8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025f500; f_8025f81c(c);
                pc = 0x8025f500;
            }
            0x8025f500 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f484: bad pc {:#x}", pc),
        }
    }
}

/// OnReset (0x8025f514, 16 bytes)
#[inline(never)]
pub fn f_8025f514(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f514) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f514); f_8025f514_b(c, 0x8025f514); crate::prof::exit(c, 0x8025f514); return; }
    }
    f_8025f514_b(c, 0x8025f514)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f514_at(c: &mut Cpu, pc: u32) { f_8025f514_b(c, pc) }
#[inline(always)]
fn f_8025f514_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f514;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f514, pc); }
        match pc {
            0x8025f514 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffae50), c.r[0]);
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8025f514: bad pc {:#x}", pc),
        }
    }
}

/// ReadProc (0x8025f524, 96 bytes)
#[inline(never)]
pub fn f_8025f524(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f524) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f524); f_8025f524_b(c, 0x8025f524); crate::prof::exit(c, 0x8025f524); return; }
    }
    f_8025f524_b(c, 0x8025f524)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f524_at(c: &mut Cpu, pc: u32) { f_8025f524_b(c, pc) }
#[inline(always)]
fn f_8025f524_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f524;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f524, pc); }
        match pc {
            0x8025f524 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.r[3].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[4].wrapping_add(0xffff83e0);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[0].wrapping_add(c.r[3]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x20));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8025f570; continue; }
                pc = 0x8025f550;
            }
            0x8025f550 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x18));
                c.r[4] = c.r[31].wrapping_add(0x5);
                c.r[5] = 0x4;
                c.lr = 0x8025f560; f_800054f4(c);
                pc = 0x8025f560;
            }
            0x8025f560 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x9));
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x14));
                c.r[0] = c.r[0] & 0x3a;
                c.rc(c.r[0]);
                c.stb(c.r[3], c.r[0]);
                pc = 0x8025f570;
            }
            0x8025f570 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f524: bad pc {:#x}", pc),
        }
    }
}

/// GBARead (0x8025f584, 148 bytes)
#[inline(never)]
pub fn f_8025f584(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f584) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f584); f_8025f584_b(c, 0x8025f584); crate::prof::exit(c, 0x8025f584); return; }
    }
    f_8025f584_b(c, 0x8025f584)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f584_at(c: &mut Cpu, pc: u32) { f_8025f584_b(c, pc) }
#[inline(always)]
fn f_8025f584_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f584;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f584, pc); }
        match pc {
            0x8025f584 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x80480000;
                c.r[6] = c.r[31].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[3].wrapping_add(0xffff83e0);
                c.r[8] = c.r[0].wrapping_add(c.r[6]);
                c.r[0] = c.lwz(c.r[8].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025f5bc; continue; }
                pc = 0x8025f5b4;
            }
            0x8025f5b4 => {
                c.r[3] = 0x2;
                pc = 0x8025f5f0; continue;
                pc = 0x8025f5bc;
            }
            0x8025f5bc => {
                c.r[0] = 0x14;
                c.stb(c.r[8], c.r[0]);
                c.r[7] = 0x80260000;
                c.r[3] = 0x80260000;
                c.stw(c.r[8].wrapping_add(0x18), c.r[4]);
                c.r[6] = c.r[3].wrapping_add(0xfffff524);
                c.r[0] = c.r[7].wrapping_add(0xfffff7e8);
                c.stw(c.r[8].wrapping_add(0x14), c.r[5]);
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.stw(c.r[8].wrapping_add(0x1c), c.r[0]);
                c.r[5] = 0x5;
                c.lr = 0x8025f5f0; f_8025f9ac(c);
                pc = 0x8025f5f0;
            }
            0x8025f5f0 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025f5fc; continue; }
                pc = 0x8025f5f8;
            }
            0x8025f5f8 => {
                pc = 0x8025f604; continue;
                pc = 0x8025f5fc;
            }
            0x8025f5fc => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025f604; f_8025f81c(c);
                pc = 0x8025f604;
            }
            0x8025f604 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f584: bad pc {:#x}", pc),
        }
    }
}

/// WriteProc (0x8025f618, 48 bytes)
#[inline(never)]
pub fn f_8025f618(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f618) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f618); f_8025f618_b(c, 0x8025f618); crate::prof::exit(c, 0x8025f618); return; }
    }
    f_8025f618_b(c, 0x8025f618)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f618_at(c: &mut Cpu, pc: u32) { f_8025f618_b(c, pc) }
#[inline(always)]
fn f_8025f618_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f618;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f618, pc); }
        match pc {
            0x8025f618 => {
                c.r[4] = 0x80480000;
                c.r[3] = c.r[3].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[4].wrapping_add(0xffff83e0);
                c.r[3] = c.r[0].wrapping_add(c.r[3]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x20));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { return; }
                pc = 0x8025f634;
            }
            0x8025f634 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x5));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x14));
                c.r[0] = c.r[0] & 0x3a;
                c.rc(c.r[0]);
                c.stb(c.r[3], c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f618: bad pc {:#x}", pc),
        }
    }
}

/// GBAWrite (0x8025f648, 196 bytes)
#[inline(never)]
pub fn f_8025f648(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f648) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f648); f_8025f648_b(c, 0x8025f648); crate::prof::exit(c, 0x8025f648); return; }
    }
    f_8025f648_b(c, 0x8025f648)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f648_at(c: &mut Cpu, pc: u32) { f_8025f648_b(c, pc) }
#[inline(always)]
fn f_8025f648_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f648;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f648, pc); }
        match pc {
            0x8025f648 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.r[29] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[28]);
                c.r[28] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x80480000;
                c.r[6] = c.r[28].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[3].wrapping_add(0xffff83e0);
                c.r[31] = c.r[0].wrapping_add(c.r[6]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025f694; continue; }
                pc = 0x8025f68c;
            }
            0x8025f68c => {
                c.r[3] = 0x2;
                pc = 0x8025f6d8; continue;
                pc = 0x8025f694;
            }
            0x8025f694 => {
                c.r[0] = 0x15;
                c.stb(c.r[31], c.r[0]);
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.r[3] = c.r[31].wrapping_add(0x1);
                c.r[5] = 0x4;
                c.lr = 0x8025f6ac; f_800054f4(c);
                pc = 0x8025f6ac;
            }
            0x8025f6ac => {
                c.stw(c.r[31].wrapping_add(0x18), c.r[29]);
                c.r[4] = 0x80260000;
                c.r[0] = c.r[4].wrapping_add(0xfffff7e8);
                c.stw(c.r[31].wrapping_add(0x14), c.r[30]);
                c.r[3] = 0x80260000;
                c.r[6] = c.r[3].wrapping_add(0xfffff618);
                c.stw(c.r[31].wrapping_add(0x1c), c.r[0]);
                c.r[3] = c.r[28].wrapping_add(0x0);
                c.r[4] = 0x5;
                c.r[5] = 0x1;
                c.lr = 0x8025f6d8; f_8025f9ac(c);
                pc = 0x8025f6d8;
            }
            0x8025f6d8 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8025f6e4; continue; }
                pc = 0x8025f6e0;
            }
            0x8025f6e0 => {
                pc = 0x8025f6ec; continue;
                pc = 0x8025f6e4;
            }
            0x8025f6e4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8025f6ec; f_8025f81c(c);
                pc = 0x8025f6ec;
            }
            0x8025f6ec => {
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
            _ => panic!("f_8025f648: bad pc {:#x}", pc),
        }
    }
}

/// __GBAHandler (0x8025f70c, 220 bytes)
#[inline(never)]
pub fn f_8025f70c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f70c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f70c); f_8025f70c_b(c, 0x8025f70c); crate::prof::exit(c, 0x8025f70c); return; }
    }
    f_8025f70c_b(c, 0x8025f70c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f70c_at(c: &mut Cpu, pc: u32) { f_8025f70c_b(c, pc) }
#[inline(always)]
fn f_8025f70c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f70c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f70c, pc); }
        match pc {
            0x8025f70c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffd08); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2f4), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x2f0), c.r[30]);
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x2ec), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x80480000;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffae50));
                c.r[6] = c.r[29].rotate_left(8) & 0xffffff00;
                c.cmps(0, c.r[0], 0x0);
                c.r[0] = c.r[3].wrapping_add(0xffff83e0);
                c.r[31] = c.r[0].wrapping_add(c.r[6]);
                if !c.crbit(2) { pc = 0x8025f7cc; continue; }
                pc = 0x8025f748;
            }
            0x8025f748 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xf;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025f75c; continue; }
                pc = 0x8025f750;
            }
            0x8025f750 => {
                c.r[0] = 0x1;
                c.stw(c.r[31].wrapping_add(0x20), c.r[0]);
                pc = 0x8025f764; continue;
                pc = 0x8025f75c;
            }
            0x8025f75c => {
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x20), c.r[0]);
                pc = 0x8025f764;
            }
            0x8025f764 => {
                c.r[12] = c.lwz(c.r[31].wrapping_add(0x38));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x8025f784; continue; }
                pc = 0x8025f770;
            }
            0x8025f770 => {
                c.r[0] = 0x0;
                c.mtspr(8, c.r[12]);
                c.stw(c.r[31].wrapping_add(0x38), c.r[0]);
                c.r[3] = c.r[29] | c.r[29];
                if true { let t = c.lr; c.lr = 0x8025f784; (c.dispatch)(c, t); }
                pc = 0x8025f784;
            }
            0x8025f784 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025f7cc; continue; }
                pc = 0x8025f790;
            }
            0x8025f790 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x8025f798; f_8009bd60(c);
                pc = 0x8025f798;
            }
            0x8025f798 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x8025f7a0; f_8009bb68(c);
                pc = 0x8025f7a0;
            }
            0x8025f7a0 => {
                c.r[12] = c.lwz(c.r[31].wrapping_add(0x1c));
                c.r[0] = 0x0;
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.stw(c.r[31].wrapping_add(0x1c), c.r[0]);
                c.mtspr(8, c.r[12]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x20));
                if true { let t = c.lr; c.lr = 0x8025f7bc; (c.dispatch)(c, t); }
                pc = 0x8025f7bc;
            }
            0x8025f7bc => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x8025f7c4; f_8009bd60(c);
                pc = 0x8025f7c4;
            }
            0x8025f7c4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8025f7cc; f_8009bb68(c);
                pc = 0x8025f7cc;
            }
            0x8025f7cc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2fc));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2f4));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x2f0));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x2ec));
                c.r[1] = c.r[1].wrapping_add(0x2f8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f70c: bad pc {:#x}", pc),
        }
    }
}

/// __GBASyncCallback (0x8025f7e8, 52 bytes)
#[inline(never)]
pub fn f_8025f7e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f7e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f7e8); f_8025f7e8_b(c, 0x8025f7e8); crate::prof::exit(c, 0x8025f7e8); return; }
    }
    f_8025f7e8_b(c, 0x8025f7e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f7e8_at(c: &mut Cpu, pc: u32) { f_8025f7e8_b(c, pc) }
#[inline(always)]
fn f_8025f7e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f7e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f7e8, pc); }
        match pc {
            0x8025f7e8 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.r[3].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[4].wrapping_add(0xffff83e0);
                c.r[3] = c.r[0].wrapping_add(c.r[3]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[3] = c.r[3].wrapping_add(0x24);
                c.lr = 0x8025f80c; f_800a2478(c);
                pc = 0x8025f80c;
            }
            0x8025f80c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f7e8: bad pc {:#x}", pc),
        }
    }
}

/// __GBASync (0x8025f81c, 108 bytes)
#[inline(never)]
pub fn f_8025f81c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f81c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f81c); f_8025f81c_b(c, 0x8025f81c); crate::prof::exit(c, 0x8025f81c); return; }
    }
    f_8025f81c_b(c, 0x8025f81c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f81c_at(c: &mut Cpu, pc: u32) { f_8025f81c_b(c, pc) }
#[inline(always)]
fn f_8025f81c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f81c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f81c, pc); }
        match pc {
            0x8025f81c => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.r[3].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[4].wrapping_add(0xffff83e0);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[0].wrapping_add(c.r[3]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.lr = 0x8025f844; f_8009df3c(c);
                pc = 0x8025f844;
            }
            0x8025f844 => {
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x8025f854; continue;
                pc = 0x8025f84c;
            }
            0x8025f84c => {
                c.r[3] = c.r[31].wrapping_add(0x24);
                c.lr = 0x8025f854; f_800a238c(c);
                pc = 0x8025f854;
            }
            0x8025f854 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8025f84c; continue; }
                pc = 0x8025f860;
            }
            0x8025f860 => {
                c.r[31] = c.lwz(c.r[31].wrapping_add(0x20));
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8025f86c; f_8009df64(c);
                pc = 0x8025f86c;
            }
            0x8025f86c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f81c: bad pc {:#x}", pc),
        }
    }
}

/// TypeAndStatusCallback (0x8025f888, 292 bytes)
#[inline(never)]
pub fn f_8025f888(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f888) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f888); f_8025f888_b(c, 0x8025f888); crate::prof::exit(c, 0x8025f888); return; }
    }
    f_8025f888_b(c, 0x8025f888)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f888_at(c: &mut Cpu, pc: u32) { f_8025f888_b(c, pc) }
#[inline(always)]
fn f_8025f888_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f888;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f888, pc); }
        match pc {
            0x8025f888 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffd10); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2ec), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x2e8), c.r[30]);
                c.r[5] = c.r[31].rotate_left(8) & 0xffffff00;
                c.stw(c.r[1].wrapping_add(0x2e4), c.r[29]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffae50));
                c.cmps(0, c.r[0], 0x0);
                c.r[0] = c.r[3].wrapping_add(0xffff83e0);
                c.r[30] = c.r[0].wrapping_add(c.r[5]);
                if !c.crbit(2) { pc = 0x8025f990; continue; }
                pc = 0x8025f8c0;
            }
            0x8025f8c0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8025f8d8; continue; }
                pc = 0x8025f8c8;
            }
            0x8025f8c8 => {
                c.r[3] = c.r[4].rotate_left(0) & 0xffff0000;
                c.r[0] = c.r[3].wrapping_add(0xfffc0000);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025f8e4; continue; }
                pc = 0x8025f8d8;
            }
            0x8025f8d8 => {
                c.r[0] = 0x1;
                c.stw(c.r[30].wrapping_add(0x20), c.r[0]);
                pc = 0x8025f91c; continue;
                pc = 0x8025f8e4;
            }
            0x8025f8e4 => {
                c.r[3] = 0x80260000;
                c.r[5] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[8] = c.r[3].wrapping_add(0xfffff70c);
                c.r[7] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[9] = c.lwz(c.r[30].wrapping_add(0x30));
                c.r[3] = c.r[31] | c.r[31];
                c.r[10] = c.lwz(c.r[30].wrapping_add(0x34));
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[6] = c.r[30].wrapping_add(0x5);
                c.lr = 0x8025f90c; f_800d06f4(c);
                pc = 0x8025f90c;
            }
            0x8025f90c => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8025f990; continue; }
                pc = 0x8025f914;
            }
            0x8025f914 => {
                c.r[0] = 0x2;
                c.stw(c.r[30].wrapping_add(0x20), c.r[0]);
                pc = 0x8025f91c;
            }
            0x8025f91c => {
                c.r[12] = c.lwz(c.r[30].wrapping_add(0x38));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x8025f93c; continue; }
                pc = 0x8025f928;
            }
            0x8025f928 => {
                c.r[0] = 0x0;
                c.mtspr(8, c.r[12]);
                c.stw(c.r[30].wrapping_add(0x38), c.r[0]);
                c.r[3] = c.r[31] | c.r[31];
                if true { let t = c.lr; c.lr = 0x8025f93c; (c.dispatch)(c, t); }
                pc = 0x8025f93c;
            }
            0x8025f93c => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025f990; continue; }
                pc = 0x8025f948;
            }
            0x8025f948 => {
                c.lr = 0x8025f94c; f_8009bbc4(c);
                pc = 0x8025f94c;
            }
            0x8025f94c => {
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x8025f958; f_8009bd60(c);
                pc = 0x8025f958;
            }
            0x8025f958 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x8025f960; f_8009bb68(c);
                pc = 0x8025f960;
            }
            0x8025f960 => {
                c.r[12] = c.lwz(c.r[30].wrapping_add(0x1c));
                c.r[0] = 0x0;
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.stw(c.r[30].wrapping_add(0x1c), c.r[0]);
                c.mtspr(8, c.r[12]);
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x20));
                if true { let t = c.lr; c.lr = 0x8025f97c; (c.dispatch)(c, t); }
                pc = 0x8025f97c;
            }
            0x8025f97c => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x8025f984; f_8009bd60(c);
                pc = 0x8025f984;
            }
            0x8025f984 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8025f98c; f_8009bb68(c);
                pc = 0x8025f98c;
            }
            0x8025f98c => {
                c.lr = 0x8025f990; f_800a1960(c);
                pc = 0x8025f990;
            }
            0x8025f990 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2f4));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2ec));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x2e8));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x2e4));
                c.r[1] = c.r[1].wrapping_add(0x2f0);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f888: bad pc {:#x}", pc),
        }
    }
}

/// __GBATransfer (0x8025f9ac, 116 bytes)
#[inline(never)]
pub fn f_8025f9ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025f9ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8025f9ac); f_8025f9ac_b(c, 0x8025f9ac); crate::prof::exit(c, 0x8025f9ac); return; }
    }
    f_8025f9ac_b(c, 0x8025f9ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025f9ac_at(c: &mut Cpu, pc: u32) { f_8025f9ac_b(c, pc) }
#[inline(always)]
fn f_8025f9ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025f9ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025f9ac, pc); }
        match pc {
            0x8025f9ac => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x80480000;
                c.r[7] = c.r[27].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[3].wrapping_add(0xffff83e0);
                c.r[28] = c.r[4].wrapping_add(0x0);
                c.r[29] = c.r[5].wrapping_add(0x0);
                c.r[31] = c.r[6].wrapping_add(0x0);
                c.r[30] = c.r[0].wrapping_add(c.r[7]);
                c.lr = 0x8025f9e0; f_8009df3c(c);
                pc = 0x8025f9e0;
            }
            0x8025f9e0 => {
                c.stw(c.r[30].wrapping_add(0x38), c.r[31]);
                c.r[4] = 0x80260000;
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[30].wrapping_add(0xc), c.r[28]);
                c.r[4] = c.r[4].wrapping_add(0xfffff888);
                c.r[3] = c.r[27] | c.r[27];
                c.stw(c.r[30].wrapping_add(0x10), c.r[29]);
                c.lr = 0x8025fa00; f_800d0cbc(c);
                pc = 0x8025fa00;
            }
            0x8025fa00 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8025fa08; f_8009df64(c);
                pc = 0x8025fa08;
            }
            0x8025fa08 => {
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x0;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8025f9ac: bad pc {:#x}", pc),
        }
    }
}

/// memoGetScaleAngle (0x8025fa20, 428 bytes)
#[inline(never)]
pub fn f_8025fa20(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025fa20) { return; }
        if c.prof { crate::prof::enter(c, 0x8025fa20); f_8025fa20_b(c, 0x8025fa20); crate::prof::exit(c, 0x8025fa20); return; }
    }
    f_8025fa20_b(c, 0x8025fa20)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025fa20_at(c: &mut Cpu, pc: u32) { f_8025fa20_b(c, pc) }
#[inline(always)]
fn f_8025fa20_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025fa20;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025fa20, pc); }
        match pc {
            0x8025fa20 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                { let ea = c.r[2].wrapping_add(0xffffaff0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.cmps(0, c.r[0], 0x92);
                if c.crbit(2) { pc = 0x8025fb38; continue; }
                pc = 0x8025fa30;
            }
            0x8025fa30 => {
                if !c.crbit(0) { pc = 0x8025faa4; continue; }
                pc = 0x8025fa34;
            }
            0x8025fa34 => {
                c.cmps(0, c.r[0], 0x4a);
                if c.crbit(2) { pc = 0x8025fb98; continue; }
                pc = 0x8025fa3c;
            }
            0x8025fa3c => {
                if !c.crbit(0) { pc = 0x8025fa70; continue; }
                pc = 0x8025fa40;
            }
            0x8025fa40 => {
                c.cmps(0, c.r[0], 0x26);
                if c.crbit(2) { pc = 0x8025fb10; continue; }
                pc = 0x8025fa48;
            }
            0x8025fa48 => {
                if !c.crbit(0) { pc = 0x8025fa64; continue; }
                pc = 0x8025fa4c;
            }
            0x8025fa4c => {
                c.cmps(0, c.r[0], 0x1a);
                if c.crbit(2) { pc = 0x8025fb60; continue; }
                pc = 0x8025fa54;
            }
            0x8025fa54 => {
                if !c.crbit(0) { pc = 0x8025fbac; continue; }
                pc = 0x8025fa58;
            }
            0x8025fa58 => {
                c.cmps(0, c.r[0], 0x6);
                if c.crbit(2) { pc = 0x8025fb68; continue; }
                pc = 0x8025fa60;
            }
            0x8025fa60 => {
                pc = 0x8025fbac; continue;
                pc = 0x8025fa64;
            }
            0x8025fa64 => {
                c.cmps(0, c.r[0], 0x44);
                if c.crbit(2) { pc = 0x8025fb88; continue; }
                pc = 0x8025fa6c;
            }
            0x8025fa6c => {
                pc = 0x8025fbac; continue;
                pc = 0x8025fa70;
            }
            0x8025fa70 => {
                c.cmps(0, c.r[0], 0x8e);
                if c.crbit(2) { pc = 0x8025fb30; continue; }
                pc = 0x8025fa78;
            }
            0x8025fa78 => {
                if !c.crbit(0) { pc = 0x8025fa94; continue; }
                pc = 0x8025fa7c;
            }
            0x8025fa7c => {
                c.cmps(0, c.r[0], 0x85);
                if c.crbit(2) { pc = 0x8025fba8; continue; }
                pc = 0x8025fa84;
            }
            0x8025fa84 => {
                if !c.crbit(0) { pc = 0x8025fbac; continue; }
                pc = 0x8025fa88;
            }
            0x8025fa88 => {
                c.cmps(0, c.r[0], 0x4c);
                if !c.crbit(0) { pc = 0x8025fbac; continue; }
                pc = 0x8025fa90;
            }
            0x8025fa90 => {
                pc = 0x8025fb18; continue;
                pc = 0x8025fa94;
            }
            0x8025fa94 => {
                c.cmps(0, c.r[0], 0x90);
                if c.crbit(2) { pc = 0x8025fb40; continue; }
                pc = 0x8025fa9c;
            }
            0x8025fa9c => {
                if !c.crbit(0) { pc = 0x8025fb20; continue; }
                pc = 0x8025faa0;
            }
            0x8025faa0 => {
                pc = 0x8025fbac; continue;
                pc = 0x8025faa4;
            }
            0x8025faa4 => {
                c.cmps(0, c.r[0], 0x136);
                if c.crbit(2) { pc = 0x8025fb90; continue; }
                pc = 0x8025faac;
            }
            0x8025faac => {
                if !c.crbit(0) { pc = 0x8025fae0; continue; }
                pc = 0x8025fab0;
            }
            0x8025fab0 => {
                c.cmps(0, c.r[0], 0xfa);
                if c.crbit(2) { pc = 0x8025fb48; continue; }
                pc = 0x8025fab8;
            }
            0x8025fab8 => {
                if !c.crbit(0) { pc = 0x8025fad4; continue; }
                pc = 0x8025fabc;
            }
            0x8025fabc => {
                c.cmps(0, c.r[0], 0xe2);
                if c.crbit(2) { pc = 0x8025fb70; continue; }
                pc = 0x8025fac4;
            }
            0x8025fac4 => {
                if !c.crbit(0) { pc = 0x8025fbac; continue; }
                pc = 0x8025fac8;
            }
            0x8025fac8 => {
                c.cmps(0, c.r[0], 0xd9);
                if c.crbit(2) { pc = 0x8025fb78; continue; }
                pc = 0x8025fad0;
            }
            0x8025fad0 => {
                pc = 0x8025fbac; continue;
                pc = 0x8025fad4;
            }
            0x8025fad4 => {
                c.cmps(0, c.r[0], 0x12c);
                if c.crbit(2) { pc = 0x8025fb28; continue; }
                pc = 0x8025fadc;
            }
            0x8025fadc => {
                pc = 0x8025fbac; continue;
                pc = 0x8025fae0;
            }
            0x8025fae0 => {
                c.cmps(0, c.r[0], 0x16b);
                if c.crbit(2) { pc = 0x8025fba0; continue; }
                pc = 0x8025fae8;
            }
            0x8025fae8 => {
                if !c.crbit(0) { pc = 0x8025faf8; continue; }
                pc = 0x8025faec;
            }
            0x8025faec => {
                c.cmps(0, c.r[0], 0x14b);
                if c.crbit(2) { pc = 0x8025fb80; continue; }
                pc = 0x8025faf4;
            }
            0x8025faf4 => {
                pc = 0x8025fbac; continue;
                pc = 0x8025faf8;
            }
            0x8025faf8 => {
                c.cmps(0, c.r[0], 0x198);
                if c.crbit(2) { pc = 0x8025fb58; continue; }
                pc = 0x8025fb00;
            }
            0x8025fb00 => {
                if !c.crbit(0) { pc = 0x8025fbac; continue; }
                pc = 0x8025fb04;
            }
            0x8025fb04 => {
                c.cmps(0, c.r[0], 0x197);
                if !c.crbit(0) { pc = 0x8025fb50; continue; }
                pc = 0x8025fb0c;
            }
            0x8025fb0c => {
                pc = 0x8025fbac; continue;
                pc = 0x8025fb10;
            }
            0x8025fb10 => {
                { let ea = c.r[2].wrapping_add(0xffffaff4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb18;
            }
            0x8025fb18 => {
                { let ea = c.r[2].wrapping_add(0xffffaff8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb20;
            }
            0x8025fb20 => {
                { let ea = c.r[2].wrapping_add(0xffffaffc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb28;
            }
            0x8025fb28 => {
                { let ea = c.r[2].wrapping_add(0xffffb000); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb30;
            }
            0x8025fb30 => {
                { let ea = c.r[2].wrapping_add(0xffffaffc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb38;
            }
            0x8025fb38 => {
                { let ea = c.r[2].wrapping_add(0xffffb004); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb40;
            }
            0x8025fb40 => {
                { let ea = c.r[2].wrapping_add(0xffffb004); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb48;
            }
            0x8025fb48 => {
                { let ea = c.r[2].wrapping_add(0xffffaff8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb50;
            }
            0x8025fb50 => {
                { let ea = c.r[2].wrapping_add(0xffffb004); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb58;
            }
            0x8025fb58 => {
                { let ea = c.r[2].wrapping_add(0xffffb004); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb60;
            }
            0x8025fb60 => {
                { let ea = c.r[2].wrapping_add(0xffffb000); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb68;
            }
            0x8025fb68 => {
                { let ea = c.r[2].wrapping_add(0xffffb000); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb70;
            }
            0x8025fb70 => {
                { let ea = c.r[2].wrapping_add(0xffffaffc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb78;
            }
            0x8025fb78 => {
                { let ea = c.r[2].wrapping_add(0xffffaffc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb80;
            }
            0x8025fb80 => {
                { let ea = c.r[2].wrapping_add(0xffffb008); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb88;
            }
            0x8025fb88 => {
                { let ea = c.r[2].wrapping_add(0xffffb008); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb90;
            }
            0x8025fb90 => {
                { let ea = c.r[2].wrapping_add(0xffffaffc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fb98;
            }
            0x8025fb98 => {
                { let ea = c.r[2].wrapping_add(0xffffaff8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fba0;
            }
            0x8025fba0 => {
                { let ea = c.r[2].wrapping_add(0xffffaff8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac; continue;
                pc = 0x8025fba8;
            }
            0x8025fba8 => {
                { let ea = c.r[2].wrapping_add(0xffffaff8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8025fbac;
            }
            0x8025fbac => {
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x8025fbbc; continue; }
                pc = 0x8025fbb4;
            }
            0x8025fbb4 => {
                { let ea = c.r[2].wrapping_add(0xffffb00c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5]; let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8025fbbc;
            }
            0x8025fbbc => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8025fbc4;
            }
            0x8025fbc4 => {
                { let ea = c.r[4]; let v = c.f[1]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_8025fa20: bad pc {:#x}", pc),
        }
    }
}

/// memoInitDebug (0x8025fbcc, 360 bytes)
#[inline(never)]
pub fn f_8025fbcc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025fbcc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025fbcc); f_8025fbcc_b(c, 0x8025fbcc); crate::prof::exit(c, 0x8025fbcc); return; }
    }
    f_8025fbcc_b(c, 0x8025fbcc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025fbcc_at(c: &mut Cpu, pc: u32) { f_8025fbcc_b(c, pc) }
#[inline(always)]
fn f_8025fbcc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025fbcc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025fbcc, pc); }
        match pc {
            0x8025fbcc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                if !c.crbit(2) { pc = 0x8025fbf0; continue; }
                pc = 0x8025fbe4;
            }
            0x8025fbe4 => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.lr = 0x8025fbf0; f_80129280(c);
                pc = 0x8025fbf0;
            }
            0x8025fbf0 => {
                c.r[31] = 0x1;
                pc = 0x8025fc7c; continue;
                pc = 0x8025fbf8;
            }
            0x8025fbf8 => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.lr = 0x8025fc04; f_80129280(c);
                pc = 0x8025fc04;
            }
            0x8025fc04 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31].rotate_left(0) & 0xffff;
                c.r[6] = c.lhz(c.r[30]);
                c.r[5] = 0x0;
                pc = 0x8025fc38; continue;
                pc = 0x8025fc18;
            }
            0x8025fc18 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.r[4] = c.r[0].wrapping_mul(0xc);
                c.r[0] = c.r[4].wrapping_add(0x4);
                c.r[0] = c.lhz(c.r[30].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].rotate_left(0) & 0x3fff;
                c.cmpu(0, c.r[0], c.r[3]);
                if c.crbit(2) { pc = 0x8025fc78; continue; }
                pc = 0x8025fc34;
            }
            0x8025fc34 => {
                c.r[5] = c.r[5].wrapping_add(0x1);
                pc = 0x8025fc38;
            }
            0x8025fc38 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[6]);
                if c.crbit(0) { pc = 0x8025fc18; continue; }
                pc = 0x8025fc44;
            }
            0x8025fc44 => {
                c.r[4] = c.r[6].wrapping_mul(0xc);
                c.r[5] = c.r[31] | 0x8000;
                c.r[3] = 0xffffffff;
                c.r[0] = c.r[4].wrapping_add(0x4);
                c.sth(c.r[30].wrapping_add(c.r[0]), c.r[5]);
                c.lr = 0x8025fc5c; f_800e0c04(c);
                pc = 0x8025fc5c;
            }
            0x8025fc5c => {
                c.r[0] = c.lhz(c.r[30]);
                c.r[4] = c.r[0].wrapping_mul(0xc);
                c.r[0] = c.r[4].wrapping_add(0xc);
                c.stw(c.r[30].wrapping_add(c.r[0]), c.r[3]);
                c.r[3] = c.lhz(c.r[30]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[30], c.r[0]);
                pc = 0x8025fc78;
            }
            0x8025fc78 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x8025fc7c;
            }
            0x8025fc7c => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xfb);
                if !c.crbit(1) { pc = 0x8025fbf8; continue; }
                pc = 0x8025fc88;
            }
            0x8025fc88 => {
                c.r[31] = 0x115;
                pc = 0x8025fd14; continue;
                pc = 0x8025fc90;
            }
            0x8025fc90 => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.lr = 0x8025fc9c; f_80129280(c);
                pc = 0x8025fc9c;
            }
            0x8025fc9c => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31].rotate_left(0) & 0xffff;
                c.r[6] = c.lhz(c.r[30]);
                c.r[5] = 0x0;
                pc = 0x8025fcd0; continue;
                pc = 0x8025fcb0;
            }
            0x8025fcb0 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.r[4] = c.r[0].wrapping_mul(0xc);
                c.r[0] = c.r[4].wrapping_add(0x4);
                c.r[0] = c.lhz(c.r[30].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].rotate_left(0) & 0x3fff;
                c.cmpu(0, c.r[0], c.r[3]);
                if c.crbit(2) { pc = 0x8025fd10; continue; }
                pc = 0x8025fccc;
            }
            0x8025fccc => {
                c.r[5] = c.r[5].wrapping_add(0x1);
                pc = 0x8025fcd0;
            }
            0x8025fcd0 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[6]);
                if c.crbit(0) { pc = 0x8025fcb0; continue; }
                pc = 0x8025fcdc;
            }
            0x8025fcdc => {
                c.r[4] = c.r[6].wrapping_mul(0xc);
                c.r[5] = c.r[31] | 0x8000;
                c.r[3] = 0xffffffff;
                c.r[0] = c.r[4].wrapping_add(0x4);
                c.sth(c.r[30].wrapping_add(c.r[0]), c.r[5]);
                c.lr = 0x8025fcf4; f_800e0c04(c);
                pc = 0x8025fcf4;
            }
            0x8025fcf4 => {
                c.r[0] = c.lhz(c.r[30]);
                c.r[4] = c.r[0].wrapping_mul(0xc);
                c.r[0] = c.r[4].wrapping_add(0xc);
                c.stw(c.r[30].wrapping_add(c.r[0]), c.r[3]);
                c.r[3] = c.lhz(c.r[30]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[30], c.r[0]);
                pc = 0x8025fd10;
            }
            0x8025fd10 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x8025fd14;
            }
            0x8025fd14 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x19b);
                if !c.crbit(1) { pc = 0x8025fc90; continue; }
                pc = 0x8025fd20;
            }
            0x8025fd20 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025fbcc: bad pc {:#x}", pc),
        }
    }
}

/// memoDataGetPokemonTrainerRndFromID (0x8025fd34, 168 bytes)
#[inline(never)]
pub fn f_8025fd34(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025fd34) { return; }
        if c.prof { crate::prof::enter(c, 0x8025fd34); f_8025fd34_b(c, 0x8025fd34); crate::prof::exit(c, 0x8025fd34); return; }
    }
    f_8025fd34_b(c, 0x8025fd34)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025fd34_at(c: &mut Cpu, pc: u32) { f_8025fd34_b(c, pc) }
#[inline(always)]
fn f_8025fd34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025fd34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025fd34, pc); }
        match pc {
            0x8025fd34 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[31] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x8025fd64; continue; }
                pc = 0x8025fd54;
            }
            0x8025fd54 => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.lr = 0x8025fd60; f_80129280(c);
                pc = 0x8025fd60;
            }
            0x8025fd60 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025fd64;
            }
            0x8025fd64 => {
                c.cmpu(0, c.r[31], 0x0);
                c.r[3] = c.r[31] | c.r[31];
                if !c.crbit(2) { pc = 0x8025fd7c; continue; }
                pc = 0x8025fd70;
            }
            0x8025fd70 => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.lr = 0x8025fd7c; f_80129280(c);
                pc = 0x8025fd7c;
            }
            0x8025fd7c => {
                c.r[3] = c.lhz(c.r[3]);
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.r[6] = 0x0;
                pc = 0x8025fdb8; continue;
                pc = 0x8025fd8c;
            }
            0x8025fd8c => {
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.r[5] = c.r[0].wrapping_mul(0xc);
                c.r[0] = c.r[5].wrapping_add(0x4);
                c.r[0] = c.lhz(c.r[31].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].rotate_left(0) & 0x3fff;
                c.cmps(0, c.r[0], c.r[4]);
                if !c.crbit(2) { pc = 0x8025fdb4; continue; }
                pc = 0x8025fda8;
            }
            0x8025fda8 => {
                c.r[3] = c.r[31].wrapping_add(c.r[5]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                pc = 0x8025fdc8; continue;
                pc = 0x8025fdb4;
            }
            0x8025fdb4 => {
                c.r[6] = c.r[6].wrapping_add(0x1);
                pc = 0x8025fdb8;
            }
            0x8025fdb8 => {
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if c.crbit(0) { pc = 0x8025fd8c; continue; }
                pc = 0x8025fdc4;
            }
            0x8025fdc4 => {
                c.r[3] = 0x0;
                pc = 0x8025fdc8;
            }
            0x8025fdc8 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025fd34: bad pc {:#x}", pc),
        }
    }
}

/// memoDataGetPokemonRndFromID (0x8025fddc, 168 bytes)
#[inline(never)]
pub fn f_8025fddc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025fddc) { return; }
        if c.prof { crate::prof::enter(c, 0x8025fddc); f_8025fddc_b(c, 0x8025fddc); crate::prof::exit(c, 0x8025fddc); return; }
    }
    f_8025fddc_b(c, 0x8025fddc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025fddc_at(c: &mut Cpu, pc: u32) { f_8025fddc_b(c, pc) }
#[inline(always)]
fn f_8025fddc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025fddc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025fddc, pc); }
        match pc {
            0x8025fddc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[31] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x8025fe0c; continue; }
                pc = 0x8025fdfc;
            }
            0x8025fdfc => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.lr = 0x8025fe08; f_80129280(c);
                pc = 0x8025fe08;
            }
            0x8025fe08 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025fe0c;
            }
            0x8025fe0c => {
                c.cmpu(0, c.r[31], 0x0);
                c.r[3] = c.r[31] | c.r[31];
                if !c.crbit(2) { pc = 0x8025fe24; continue; }
                pc = 0x8025fe18;
            }
            0x8025fe18 => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.lr = 0x8025fe24; f_80129280(c);
                pc = 0x8025fe24;
            }
            0x8025fe24 => {
                c.r[3] = c.lhz(c.r[3]);
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.r[6] = 0x0;
                pc = 0x8025fe60; continue;
                pc = 0x8025fe34;
            }
            0x8025fe34 => {
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.r[5] = c.r[0].wrapping_mul(0xc);
                c.r[0] = c.r[5].wrapping_add(0x4);
                c.r[0] = c.lhz(c.r[31].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].rotate_left(0) & 0x3fff;
                c.cmps(0, c.r[0], c.r[4]);
                if !c.crbit(2) { pc = 0x8025fe5c; continue; }
                pc = 0x8025fe50;
            }
            0x8025fe50 => {
                c.r[3] = c.r[31].wrapping_add(c.r[5]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                pc = 0x8025fe70; continue;
                pc = 0x8025fe5c;
            }
            0x8025fe5c => {
                c.r[6] = c.r[6].wrapping_add(0x1);
                pc = 0x8025fe60;
            }
            0x8025fe60 => {
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if c.crbit(0) { pc = 0x8025fe34; continue; }
                pc = 0x8025fe6c;
            }
            0x8025fe6c => {
                c.r[3] = 0x0;
                pc = 0x8025fe70;
            }
            0x8025fe70 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025fddc: bad pc {:#x}", pc),
        }
    }
}

/// memoDataGetPokemonID (0x8025fe84, 96 bytes)
#[inline(never)]
pub fn f_8025fe84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025fe84) { return; }
        if c.prof { crate::prof::enter(c, 0x8025fe84); f_8025fe84_b(c, 0x8025fe84); crate::prof::exit(c, 0x8025fe84); return; }
    }
    f_8025fe84_b(c, 0x8025fe84)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025fe84_at(c: &mut Cpu, pc: u32) { f_8025fe84_b(c, pc) }
#[inline(always)]
fn f_8025fe84_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025fe84;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025fe84, pc); }
        match pc {
            0x8025fe84 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                if !c.crbit(2) { pc = 0x8025feac; continue; }
                pc = 0x8025fea0;
            }
            0x8025fea0 => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.lr = 0x8025feac; f_80129280(c);
                pc = 0x8025feac;
            }
            0x8025feac => {
                c.r[0] = c.lhz(c.r[3]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8025fecc; continue; }
                pc = 0x8025feb8;
            }
            0x8025feb8 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.r[0] = c.r[0].wrapping_mul(0xc);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x4));
                pc = 0x8025fed0; continue;
                pc = 0x8025fecc;
            }
            0x8025fecc => {
                c.r[3] = 0x0;
                pc = 0x8025fed0;
            }
            0x8025fed0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025fe84: bad pc {:#x}", pc),
        }
    }
}

/// memoDataGetCount (0x8025fee4, 52 bytes)
#[inline(never)]
pub fn f_8025fee4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025fee4) { return; }
        if c.prof { crate::prof::enter(c, 0x8025fee4); f_8025fee4_b(c, 0x8025fee4); crate::prof::exit(c, 0x8025fee4); return; }
    }
    f_8025fee4_b(c, 0x8025fee4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025fee4_at(c: &mut Cpu, pc: u32) { f_8025fee4_b(c, pc) }
#[inline(always)]
fn f_8025fee4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025fee4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025fee4, pc); }
        match pc {
            0x8025fee4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if !c.crbit(2) { pc = 0x8025ff04; continue; }
                pc = 0x8025fef8;
            }
            0x8025fef8 => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.lr = 0x8025ff04; f_80129280(c);
                pc = 0x8025ff04;
            }
            0x8025ff04 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.lhz(c.r[3]);
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025fee4: bad pc {:#x}", pc),
        }
    }
}

/// memoDataSetMemoFlag (0x8025ff18, 132 bytes)
#[inline(never)]
pub fn f_8025ff18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025ff18) { return; }
        if c.prof { crate::prof::enter(c, 0x8025ff18); f_8025ff18_b(c, 0x8025ff18); crate::prof::exit(c, 0x8025ff18); return; }
    }
    f_8025ff18_b(c, 0x8025ff18)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025ff18_at(c: &mut Cpu, pc: u32) { f_8025ff18_b(c, pc) }
#[inline(always)]
fn f_8025ff18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025ff18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025ff18, pc); }
        match pc {
            0x8025ff18 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = 0x0;
                if !c.crbit(2) { pc = 0x8025ff40; continue; }
                pc = 0x8025ff34;
            }
            0x8025ff34 => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.lr = 0x8025ff40; f_80129280(c);
                pc = 0x8025ff40;
            }
            0x8025ff40 => {
                c.r[5] = 0x0;
                pc = 0x8025ff74; continue;
                pc = 0x8025ff48;
            }
            0x8025ff48 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.r[4] = c.r[0].wrapping_mul(0xc);
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.r[0] = c.lhz(c.r[3].wrapping_add(c.r[4]));
                c.r[0] = c.r[0].rotate_left(0) & 0x8000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8025ff64; continue; }
                pc = 0x8025ff60;
            }
            0x8025ff60 => {
                c.r[31] = 0x1;
                pc = 0x8025ff64;
            }
            0x8025ff64 => {
                c.r[0] = c.lhz(c.r[3].wrapping_add(c.r[4]));
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.r[0] = c.r[0].rotate_left(0) & 0x3fff;
                c.sth(c.r[3].wrapping_add(c.r[4]), c.r[0]);
                pc = 0x8025ff74;
            }
            0x8025ff74 => {
                c.r[0] = c.lhz(c.r[3]);
                c.r[4] = c.r[5].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x8025ff48; continue; }
                pc = 0x8025ff84;
            }
            0x8025ff84 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025ff18: bad pc {:#x}", pc),
        }
    }
}

/// memoDataSet (0x8025ff9c, 212 bytes)
#[inline(never)]
pub fn f_8025ff9c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8025ff9c) { return; }
        if c.prof { crate::prof::enter(c, 0x8025ff9c); f_8025ff9c_b(c, 0x8025ff9c); crate::prof::exit(c, 0x8025ff9c); return; }
    }
    f_8025ff9c_b(c, 0x8025ff9c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8025ff9c_at(c: &mut Cpu, pc: u32) { f_8025ff9c_b(c, pc) }
#[inline(always)]
fn f_8025ff9c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8025ff9c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8025ff9c, pc); }
        match pc {
            0x8025ff9c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[31] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x8025ffcc; continue; }
                pc = 0x8025ffbc;
            }
            0x8025ffbc => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.lr = 0x8025ffc8; f_80129280(c);
                pc = 0x8025ffc8;
            }
            0x8025ffc8 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8025ffcc;
            }
            0x8025ffcc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8025ffd4; f_8011f5c8(c);
                pc = 0x8025ffd4;
            }
            0x8025ffd4 => {
                c.r[6] = c.lhz(c.r[31]);
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[7] = 0x0;
                pc = 0x80260004; continue;
                pc = 0x8025ffe4;
            }
            0x8025ffe4 => {
                c.r[0] = c.r[7].rotate_left(0) & 0xffff;
                c.r[5] = c.r[0].wrapping_mul(0xc);
                c.r[0] = c.r[5].wrapping_add(0x4);
                c.r[0] = c.lhz(c.r[31].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].rotate_left(0) & 0x3fff;
                c.cmpu(0, c.r[0], c.r[4]);
                if c.crbit(2) { pc = 0x8026005c; continue; }
                pc = 0x80260000;
            }
            0x80260000 => {
                c.r[7] = c.r[7].wrapping_add(0x1);
                pc = 0x80260004;
            }
            0x80260004 => {
                c.r[0] = c.r[7].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[6]);
                if c.crbit(0) { pc = 0x8025ffe4; continue; }
                pc = 0x80260010;
            }
            0x80260010 => {
                c.r[0] = c.r[6].wrapping_mul(0xc);
                c.r[5] = c.r[3] | 0x8000;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].wrapping_add(c.r[0]);
                c.sth(c.r[4].wrapping_add(0x4), c.r[5]);
                c.lr = 0x80260028; f_8011f5b0(c);
                pc = 0x80260028;
            }
            0x80260028 => {
                c.r[0] = c.lhz(c.r[31]);
                c.r[0] = c.r[0].wrapping_mul(0xc);
                c.r[4] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[4].wrapping_add(0xc), c.r[3]);
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80260040; f_8011f520(c);
                pc = 0x80260040;
            }
            0x80260040 => {
                c.r[0] = c.lhz(c.r[31]);
                c.r[0] = c.r[0].wrapping_mul(0xc);
                c.r[4] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[3]);
                c.r[3] = c.lhz(c.r[31]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[31], c.r[0]);
                pc = 0x8026005c;
            }
            0x8026005c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8025ff9c: bad pc {:#x}", pc),
        }
    }
}

/// memoInit (0x80260070, 116 bytes)
#[inline(never)]
pub fn f_80260070(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80260070) { return; }
        if c.prof { crate::prof::enter(c, 0x80260070); f_80260070_b(c, 0x80260070); crate::prof::exit(c, 0x80260070); return; }
    }
    f_80260070_b(c, 0x80260070)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80260070_at(c: &mut Cpu, pc: u32) { f_80260070_b(c, pc) }
#[inline(always)]
fn f_80260070_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80260070;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80260070, pc); }
        match pc {
            0x80260070 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x8026009c; continue; }
                pc = 0x8026008c;
            }
            0x8026008c => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.lr = 0x80260098; f_80129280(c);
                pc = 0x80260098;
            }
            0x80260098 => {
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x8026009c;
            }
            0x8026009c => {
                c.r[30] = 0x0;
                c.r[27] = 0x0;
                c.sth(c.r[29], c.r[30]);
                c.r[31] = c.r[30] | c.r[30];
                pc = 0x802600ac;
            }
            0x802600ac => {
                c.r[28] = c.r[29].wrapping_add(c.r[31]);
                c.r[3] = 0xffffffff;
                c.sth(c.r[28].wrapping_add(0x4), c.r[30]);
                c.lr = 0x802600bc; f_800e0c04(c);
                pc = 0x802600bc;
            }
            0x802600bc => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                c.stw(c.r[28].wrapping_add(0xc), c.r[3]);
                c.cmps(0, c.r[27], 0x1f4);
                c.r[31] = c.r[31].wrapping_add(0xc);
                if c.crbit(0) { pc = 0x802600ac; continue; }
                pc = 0x802600d0;
            }
            0x802600d0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80260070: bad pc {:#x}", pc),
        }
    }
}

/// evolutionWazaLearn (0x802600e4, 888 bytes)
#[inline(never)]
pub fn f_802600e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802600e4) { return; }
        if c.prof { crate::prof::enter(c, 0x802600e4); f_802600e4_b(c, 0x802600e4); crate::prof::exit(c, 0x802600e4); return; }
    }
    f_802600e4_b(c, 0x802600e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802600e4_at(c: &mut Cpu, pc: u32) { f_802600e4_b(c, pc) }
#[inline(always)]
fn f_802600e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802600e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802600e4, pc); }
        match pc {
            0x802600e4 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[31] = c.r[6] | c.r[6];
                c.r[23] = c.r[7] | c.r[7];
                c.r[24] = c.r[8] | c.r[8];
                c.r[27] = 0x0;
                pc = 0x80260110;
            }
            0x80260110 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27].rotate_left(0) & 0xffff;
                c.lr = 0x8026011c; f_8011f228(c);
                pc = 0x8026011c;
            }
            0x8026011c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80260130; continue; }
                pc = 0x80260124;
            }
            0x80260124 => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                c.cmps(0, c.r[27], 0x4);
                if c.crbit(0) { pc = 0x80260110; continue; }
                pc = 0x80260130;
            }
            0x80260130 => {
                c.cmps(0, c.r[27], 0x4);
                if c.crbit(0) { pc = 0x802603c4; continue; }
                pc = 0x80260138;
            }
            0x80260138 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80260140; f_8011f4f0(c);
                pc = 0x80260140;
            }
            0x80260140 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x32;
                c.r[26] = c.r[0] | c.r[0];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x80260154; f_80132a38(c);
                pc = 0x80260154;
            }
            0x80260154 => {
                c.r[25] = c.r[29].rotate_left(0) & 0xffff;
                c.r[3] = 0x39;
                c.r[4] = c.r[25] | c.r[25];
                c.lr = 0x80260164; f_80132a38(c);
                pc = 0x80260164;
            }
            0x80260164 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80260180; continue; }
                pc = 0x8026016c;
            }
            0x8026016c => {
                c.r[3] = 0x4243;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x8026017c; f_801067e8(c);
                pc = 0x8026017c;
            }
            0x8026017c => {
                pc = 0x80260194; continue;
                pc = 0x80260180;
            }
            0x80260180 => {
                c.r[3] = 0x2;
                c.r[4] = 0x4243;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80260194; f_80106d3c(c);
                pc = 0x80260194;
            }
            0x80260194 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x802601a8; continue; }
                pc = 0x8026019c;
            }
            0x8026019c => {
                c.lr = 0x802601a0; f_8001e184(c);
                pc = 0x802601a0;
            }
            0x802601a0 => {
                c.r[27] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x802601c0; continue;
                pc = 0x802601a8;
            }
            0x802601a8 => {
                c.r[3] = 0x0;
                c.r[4] = 0xffffffff;
                c.r[5] = 0xffffffff;
                c.r[6] = 0x0;
                c.lr = 0x802601bc; f_8001e074(c);
                pc = 0x802601bc;
            }
            0x802601bc => {
                c.r[27] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x802601c0;
            }
            0x802601c0 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x802601d4; continue; }
                pc = 0x802601c8;
            }
            0x802601c8 => {
                c.r[3] = 0x1;
                c.lr = 0x802601d0; f_801065b8(c);
                pc = 0x802601d0;
            }
            0x802601d0 => {
                pc = 0x802601dc; continue;
                pc = 0x802601d4;
            }
            0x802601d4 => {
                c.r[3] = 0x1;
                c.lr = 0x802601dc; f_801069fc(c);
                pc = 0x802601dc;
            }
            0x802601dc => {
                c.cmps(0, c.r[27], 0x1);
                if c.crbit(2) { pc = 0x802601fc; continue; }
                pc = 0x802601e4;
            }
            0x802601e4 => {
                if !c.crbit(0) { pc = 0x80260204; continue; }
                pc = 0x802601e8;
            }
            0x802601e8 => {
                c.cmps(0, c.r[27], 0x0);
                if !c.crbit(0) { pc = 0x802601f4; continue; }
                pc = 0x802601f0;
            }
            0x802601f0 => {
                pc = 0x80260204; continue;
                pc = 0x802601f4;
            }
            0x802601f4 => {
                c.r[0] = 0x0;
                pc = 0x80260208; continue;
                pc = 0x802601fc;
            }
            0x802601fc => {
                c.r[0] = 0x1;
                pc = 0x80260208; continue;
                pc = 0x80260204;
            }
            0x80260204 => {
                c.r[0] = 0x2;
                pc = 0x80260208;
            }
            0x80260208 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80260244; continue; }
                pc = 0x80260210;
            }
            0x80260210 => {
                c.cmpu(0, c.r[23], 0x0);
                if c.crbit(2) { pc = 0x80260238; continue; }
                pc = 0x80260218;
            }
            0x80260218 => {
                c.r[12] = c.r[23] | c.r[23];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[24] | c.r[24];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80260230; (c.dispatch)(c, t); }
                pc = 0x80260230;
            }
            0x80260230 => {
                c.r[27] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x8026023c; continue;
                pc = 0x80260238;
            }
            0x80260238 => {
                c.r[27] = 0x0;
                pc = 0x8026023c;
            }
            0x8026023c => {
                c.cmps(0, c.r[27], 0x0);
                if !c.crbit(0) { pc = 0x8026035c; continue; }
                pc = 0x80260244;
            }
            0x80260244 => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x32;
                c.lr = 0x80260250; f_80132a38(c);
                pc = 0x80260250;
            }
            0x80260250 => {
                c.r[4] = c.r[25] | c.r[25];
                c.r[3] = 0x39;
                c.lr = 0x8026025c; f_80132a38(c);
                pc = 0x8026025c;
            }
            0x8026025c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80260278; continue; }
                pc = 0x80260264;
            }
            0x80260264 => {
                c.r[3] = 0x4242;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x80260274; f_801067e8(c);
                pc = 0x80260274;
            }
            0x80260274 => {
                pc = 0x8026028c; continue;
                pc = 0x80260278;
            }
            0x80260278 => {
                c.r[3] = 0x2;
                c.r[4] = 0x4242;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x8026028c; f_80106d3c(c);
                pc = 0x8026028c;
            }
            0x8026028c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x802602a0; continue; }
                pc = 0x80260294;
            }
            0x80260294 => {
                c.lr = 0x80260298; f_8001e184(c);
                pc = 0x80260298;
            }
            0x80260298 => {
                c.r[27] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x802602b8; continue;
                pc = 0x802602a0;
            }
            0x802602a0 => {
                c.r[3] = 0x0;
                c.r[4] = 0xffffffff;
                c.r[5] = 0xffffffff;
                c.r[6] = 0x0;
                c.lr = 0x802602b4; f_8001e074(c);
                pc = 0x802602b4;
            }
            0x802602b4 => {
                c.r[27] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x802602b8;
            }
            0x802602b8 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x802602cc; continue; }
                pc = 0x802602c0;
            }
            0x802602c0 => {
                c.r[3] = 0x1;
                c.lr = 0x802602c8; f_801065b8(c);
                pc = 0x802602c8;
            }
            0x802602c8 => {
                pc = 0x802602d4; continue;
                pc = 0x802602cc;
            }
            0x802602cc => {
                c.r[3] = 0x1;
                c.lr = 0x802602d4; f_801069fc(c);
                pc = 0x802602d4;
            }
            0x802602d4 => {
                c.cmps(0, c.r[27], 0x1);
                if c.crbit(2) { pc = 0x802602f4; continue; }
                pc = 0x802602dc;
            }
            0x802602dc => {
                if !c.crbit(0) { pc = 0x802602fc; continue; }
                pc = 0x802602e0;
            }
            0x802602e0 => {
                c.cmps(0, c.r[27], 0x0);
                if !c.crbit(0) { pc = 0x802602ec; continue; }
                pc = 0x802602e8;
            }
            0x802602e8 => {
                pc = 0x802602fc; continue;
                pc = 0x802602ec;
            }
            0x802602ec => {
                c.r[0] = 0x0;
                pc = 0x80260300; continue;
                pc = 0x802602f4;
            }
            0x802602f4 => {
                c.r[0] = 0x1;
                pc = 0x80260300; continue;
                pc = 0x802602fc;
            }
            0x802602fc => {
                c.r[0] = 0x2;
                pc = 0x80260300;
            }
            0x80260300 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80260164; continue; }
                pc = 0x80260308;
            }
            0x80260308 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80260324; continue; }
                pc = 0x80260310;
            }
            0x80260310 => {
                c.r[3] = 0x4241;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x80260320; f_801067e8(c);
                pc = 0x80260320;
            }
            0x80260320 => {
                pc = 0x80260338; continue;
                pc = 0x80260324;
            }
            0x80260324 => {
                c.r[3] = 0x2;
                c.r[4] = 0x4241;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80260338; f_80106d3c(c);
                pc = 0x80260338;
            }
            0x80260338 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8026034c; continue; }
                pc = 0x80260340;
            }
            0x80260340 => {
                c.r[3] = 0x1;
                c.lr = 0x80260348; f_801065b8(c);
                pc = 0x80260348;
            }
            0x80260348 => {
                pc = 0x80260354; continue;
                pc = 0x8026034c;
            }
            0x8026034c => {
                c.r[3] = 0x1;
                c.lr = 0x80260354; f_801069fc(c);
                pc = 0x80260354;
            }
            0x80260354 => {
                c.r[3] = 0x0;
                pc = 0x80260448; continue;
                pc = 0x8026035c;
            }
            0x8026035c => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80260364; f_8011f4f0(c);
                pc = 0x80260364;
            }
            0x80260364 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x32;
                c.lr = 0x80260370; f_80132a38(c);
                pc = 0x80260370;
            }
            0x80260370 => {
                c.r[3] = 0x5d;
                c.r[4] = 0x468;
                c.lr = 0x8026037c; f_80132a38(c);
                pc = 0x8026037c;
            }
            0x8026037c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27].rotate_left(0) & 0xffff;
                c.lr = 0x80260388; f_8011f228(c);
                pc = 0x80260388;
            }
            0x80260388 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x39;
                c.lr = 0x80260394; f_80132a38(c);
                pc = 0x80260394;
            }
            0x80260394 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x802603b0; continue; }
                pc = 0x8026039c;
            }
            0x8026039c => {
                c.r[3] = 0x4248;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x802603ac; f_801067e8(c);
                pc = 0x802603ac;
            }
            0x802603ac => {
                pc = 0x802603c4; continue;
                pc = 0x802603b0;
            }
            0x802603b0 => {
                c.r[3] = 0x2;
                c.r[4] = 0x4248;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x802603c4; f_80106d3c(c);
                pc = 0x802603c4;
            }
            0x802603c4 => {
                c.r[3] = 0x4ca;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x802603d4; f_80165668(c);
                pc = 0x802603d4;
            }
            0x802603d4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x802603dc; f_8011f4f0(c);
                pc = 0x802603dc;
            }
            0x802603dc => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x32;
                c.lr = 0x802603e8; f_80132a38(c);
                pc = 0x802603e8;
            }
            0x802603e8 => {
                c.r[4] = c.r[29].rotate_left(0) & 0xffff;
                c.r[3] = 0x39;
                c.lr = 0x802603f4; f_80132a38(c);
                pc = 0x802603f4;
            }
            0x802603f4 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80260410; continue; }
                pc = 0x802603fc;
            }
            0x802603fc => {
                c.r[3] = 0x423d;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x8026040c; f_801067e8(c);
                pc = 0x8026040c;
            }
            0x8026040c => {
                pc = 0x80260424; continue;
                pc = 0x80260410;
            }
            0x80260410 => {
                c.r[3] = 0x2;
                c.r[4] = 0x423d;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80260424; f_80106d3c(c);
                pc = 0x80260424;
            }
            0x80260424 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80260438; continue; }
                pc = 0x8026042c;
            }
            0x8026042c => {
                c.r[3] = 0x1;
                c.lr = 0x80260434; f_801065b8(c);
                pc = 0x80260434;
            }
            0x80260434 => {
                pc = 0x80260440; continue;
                pc = 0x80260438;
            }
            0x80260438 => {
                c.r[3] = 0x1;
                c.lr = 0x80260440; f_801069fc(c);
                pc = 0x80260440;
            }
            0x80260440 => {
                c.stb(c.r[30], c.r[27]);
                c.r[3] = 0x1;
                pc = 0x80260448;
            }
            0x80260448 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_802600e4: bad pc {:#x}", pc),
        }
    }
}

/// cbWazaForget (0x8026045c, 636 bytes)
#[inline(never)]
pub fn f_8026045c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8026045c) { return; }
        if c.prof { crate::prof::enter(c, 0x8026045c); f_8026045c_b(c, 0x8026045c); crate::prof::exit(c, 0x8026045c); return; }
    }
    f_8026045c_b(c, 0x8026045c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8026045c_at(c: &mut Cpu, pc: u32) { f_8026045c_b(c, pc) }
#[inline(always)]
fn f_8026045c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8026045c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8026045c, pc); }
        match pc {
            0x8026045c => {
                { let ea = c.r[1].wrapping_add(0xffffff60); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffffb020); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[1].wrapping_add(0xa4), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x80); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[3] = 0x3;
                c.lr = 0x80260484; f_801c41c8(c);
                pc = 0x80260484;
            }
            0x80260484 => {
                c.r[3] = 0x1;
                c.lr = 0x8026048c; f_801c40f0(c);
                pc = 0x8026048c;
            }
            0x8026048c => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x80260494; f_801778b4(c);
                pc = 0x80260494;
            }
            0x80260494 => {
                c.r[3] = c.r[1].wrapping_add(0x24);
                c.lr = 0x8026049c; f_80177830(c);
                pc = 0x8026049c;
            }
            0x8026049c => {
                c.r[3] = c.r[1].wrapping_add(0x30);
                c.lr = 0x802604a4; f_80177908(c);
                pc = 0x802604a4;
            }
            0x802604a4 => {
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.lr = 0x802604ac; f_8017795c(c);
                pc = 0x802604ac;
            }
            0x802604ac => {
                c.lr = 0x802604b0; f_800d2584(c);
                pc = 0x802604b0;
            }
            0x802604b0 => {
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[5] = c.r[1].wrapping_add(0xc);
                c.r[6] = c.r[1].wrapping_add(0x10);
                c.r[7] = c.r[1].wrapping_add(0x14);
                c.lr = 0x802604c4; f_800d1fdc(c);
                pc = 0x802604c4;
            }
            0x802604c4 => {
                c.r[25] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[31] = c.r[1].wrapping_add(0x54);
                c.r[12] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.r[1].wrapping_add(0x60);
                c.r[11] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.r[1].wrapping_add(0x6c);
                c.r[10] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[9] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[8] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[7] = c.lwz(c.r[1].wrapping_add(0x30));
                c.r[6] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[5] = c.lwz(c.r[1].wrapping_add(0x38));
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x40));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[1].wrapping_add(0x48), c.r[25]);
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
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x78); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x8026053c; f_801dac90(c);
                pc = 0x8026053c;
            }
            0x8026053c => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80260548; f_80097a38(c);
                pc = 0x80260548;
            }
            0x80260548 => {
                c.r[28] = c.r[3] | c.r[3];
                c.cmps(0, c.r[28], 0x4);
                if c.crbit(0) { pc = 0x80260558; continue; }
                pc = 0x80260554;
            }
            0x80260554 => {
                c.r[28] = 0xffffffff;
                pc = 0x80260558;
            }
            0x80260558 => {
                c.r[3] = 0x2;
                c.lr = 0x80260560; f_801dadc0(c);
                pc = 0x80260560;
            }
            0x80260560 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x80260574; f_8012640c(c);
                pc = 0x80260574;
            }
            0x80260574 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[4]);
                if !c.crbit(2) { pc = 0x80260588; continue; }
                pc = 0x8026057c;
            }
            0x8026057c => {
                c.r[3] = 0x10000;
                c.r[24] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x802605a8; continue;
                pc = 0x80260588;
            }
            0x80260588 => {
                c.r[3] = 0x0;
                c.r[5] = 0x66;
                c.r[6] = 0x0;
                c.lr = 0x80260598; f_8012640c(c);
                pc = 0x80260598;
            }
            0x80260598 => {
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[24]);
                if !c.crbit(2) { pc = 0x802605a8; continue; }
                pc = 0x802605a0;
            }
            0x802605a0 => {
                c.r[3] = 0x10000;
                c.r[24] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x802605a8;
            }
            0x802605a8 => {
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xffff);
                if !c.crbit(2) { pc = 0x802605bc; continue; }
                pc = 0x802605b4;
            }
            0x802605b4 => {
                c.r[25] = 0x0;
                pc = 0x80260654; continue;
                pc = 0x802605bc;
            }
            0x802605bc => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x802605c4; f_8011f5b0(c);
                pc = 0x802605c4;
            }
            0x802605c4 => {
                c.r[25] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0xc1;
                c.r[6] = 0x0;
                c.lr = 0x802605dc; f_8012640c(c);
                pc = 0x802605dc;
            }
            0x802605dc => {
                c.r[0] = c.r[3].wrapping_neg();
                c.r[4] = c.r[25] | c.r[25];
                c.r[0] = c.r[0] | c.r[3];
                c.r[3] = c.r[24] | c.r[24];
                c.r[5] = c.r[0].rotate_left(1) & 0x1;
                c.lr = 0x802605f4; f_801de190(c);
                pc = 0x802605f4;
            }
            0x802605f4 => {
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if !c.crbit(2) { pc = 0x80260604; continue; }
                pc = 0x802605fc;
            }
            0x802605fc => {
                c.r[25] = 0x0;
                pc = 0x80260654; continue;
                pc = 0x80260604;
            }
            0x80260604 => {
                c.r[3] = 0x80280000;
                c.r[26] = 0x0;
                c.r[24] = c.r[3].wrapping_add(0xffffa488);
                pc = 0x80260610;
            }
            0x80260610 => {
                c.r[0] = c.lwz(c.r[24].wrapping_add(0x4));
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80260638; continue; }
                pc = 0x8026061c;
            }
            0x8026061c => {
                c.r[4] = c.lhz(c.r[24]);
                c.r[3] = c.r[25] | c.r[25];
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x80260630; f_801ddd28(c);
                pc = 0x80260630;
            }
            0x80260630 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80260648; continue; }
                pc = 0x80260638;
            }
            0x80260638 => {
                c.r[26] = c.r[26].wrapping_add(0x1);
                c.r[24] = c.r[24].wrapping_add(0x8);
                c.cmps(0, c.r[26], 0x5);
                if c.crbit(0) { pc = 0x80260610; continue; }
                pc = 0x80260648;
            }
            0x80260648 => {
                c.cmps(0, c.r[26], 0x5);
                if !c.crbit(0) { pc = 0x80260654; continue; }
                pc = 0x80260650;
            }
            0x80260650 => {
                c.r[25] = 0x0;
                pc = 0x80260654;
            }
            0x80260654 => {
                c.cmpu(0, c.r[25], 0x0);
                if c.crbit(2) { pc = 0x80260660; continue; }
                pc = 0x8026065c;
            }
            0x8026065c => {
                c.stw(c.r[27].wrapping_add(0x4), c.r[25]);
                pc = 0x80260660;
            }
            0x80260660 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x4));
                c.lr = 0x80260668; f_801dac3c(c);
                pc = 0x80260668;
            }
            0x80260668 => {
                c.r[4] = 0x1;
                c.lr = 0x80260670; f_800ecb74(c);
                pc = 0x80260670;
            }
            0x80260670 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x4));
                c.r[4] = 0x1;
                c.lr = 0x8026067c; f_801da4e8(c);
                pc = 0x8026067c;
            }
            0x8026067c => {
                c.r[3] = 0x2;
                c.lr = 0x80260684; f_80177a44(c);
                pc = 0x80260684;
            }
            0x80260684 => {
                c.r[3] = c.r[1].wrapping_add(0x48);
                c.lr = 0x8026068c; f_801778dc(c);
                pc = 0x8026068c;
            }
            0x8026068c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80260694; f_80177858(c);
                pc = 0x80260694;
            }
            0x80260694 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8026069c; f_80177930(c);
                pc = 0x8026069c;
            }
            0x8026069c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x802606a4; f_80177984(c);
                pc = 0x802606a4;
            }
            0x802606a4 => {
                { let ea = c.r[1].wrapping_add(0x78); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x802606ac; f_801766a8(c);
                pc = 0x802606ac;
            }
            0x802606ac => {
                { let ea = c.r[2].wrapping_add(0xffffb020); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x802606b8; f_801c41c8(c);
                pc = 0x802606b8;
            }
            0x802606b8 => {
                c.r[3] = 0x1;
                c.lr = 0x802606c0; f_801c40f0(c);
                pc = 0x802606c0;
            }
            0x802606c0 => {
                c.r[3] = c.r[28] as u8 as i8 as i32 as u32;
                { let mut ea = c.r[1].wrapping_add(0x80); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xa4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xa0);
                if true { return; }
                return;
            }
            _ => panic!("f_8026045c: bad pc {:#x}", pc),
        }
    }
}

/// doWazaSequence (0x802606d8, 568 bytes)
#[inline(never)]
pub fn f_802606d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802606d8) { return; }
        if c.prof { crate::prof::enter(c, 0x802606d8); f_802606d8_b(c, 0x802606d8); crate::prof::exit(c, 0x802606d8); return; }
    }
    f_802606d8_b(c, 0x802606d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802606d8_at(c: &mut Cpu, pc: u32) { f_802606d8_b(c, pc) }
#[inline(always)]
fn f_802606d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802606d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802606d8, pc); }
        match pc {
            0x802606d8 => {
                { let ea = c.r[1].wrapping_add(0xffffff50); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmps(0, c.r[4], 0x0);
                c.stw(c.r[1].wrapping_add(0xb4), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x88); for n in 22..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[23] = c.r[5] | c.r[5];
                c.r[24] = c.r[6] | c.r[6];
                c.r[26] = 0x0;
                c.r[25] = 0x0;
                if c.crbit(0) { pc = 0x80260708; continue; }
                pc = 0x80260700;
            }
            0x80260700 => {
                c.cmps(0, c.r[4], 0x5);
                if c.crbit(0) { pc = 0x80260710; continue; }
                pc = 0x80260708;
            }
            0x80260708 => {
                c.r[3] = 0x0;
                pc = 0x802608fc; continue;
                pc = 0x80260710;
            }
            0x80260710 => {
                c.r[5] = 0x80280000;
                c.r[0] = c.r[4].rotate_left(3) & 0xfffffff8;
                c.r[5] = c.r[5].wrapping_add(0xffffa488);
                c.r[4] = c.r[5].wrapping_add(c.r[0]);
                c.r[28] = c.lhz(c.r[5].wrapping_add(c.r[0]));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80260738; continue; }
                pc = 0x80260730;
            }
            0x80260730 => {
                c.r[27] = c.lwz(c.r[3].wrapping_add(0x4));
                pc = 0x8026073c; continue;
                pc = 0x80260738;
            }
            0x80260738 => {
                c.r[27] = c.lwz(c.r[3]);
                pc = 0x8026073c;
            }
            0x8026073c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x4;
                c.lr = 0x8026074c; f_801da9e8(c);
                pc = 0x8026074c;
            }
            0x8026074c => {
                c.r[31] = c.r[1].wrapping_add(0x54);
                c.r[30] = c.r[1].wrapping_add(0x60);
                c.r[29] = c.r[1].wrapping_add(0x6c);
                pc = 0x80260878; continue;
                pc = 0x8026075c;
            }
            0x8026075c => {
                c.lr = 0x80260760; f_801db088(c);
                pc = 0x80260760;
            }
            0x80260760 => {
                c.r[0] = c.r[24] & 0xa;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80260788; continue; }
                pc = 0x80260768;
            }
            0x80260768 => {
                c.r[0] = c.r[24].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                c.r[3] = 0x4;
                if c.crbit(2) { pc = 0x80260778; continue; }
                pc = 0x80260774;
            }
            0x80260774 => {
                c.r[3] = 0x2;
                pc = 0x80260778;
            }
            0x80260778 => {
                { let ea = c.r[2].wrapping_add(0xffffb020); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x80260780; f_801c41c8(c);
                pc = 0x80260780;
            }
            0x80260780 => {
                c.r[0] = 0xfffffff5;
                c.r[24] = c.r[24] & c.r[0];
                pc = 0x80260788;
            }
            0x80260788 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x4;
                c.lr = 0x80260798; f_801da94c(c);
                pc = 0x80260798;
            }
            0x80260798 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80260880; continue; }
                pc = 0x802607a0;
            }
            0x802607a0 => {
                c.lr = 0x802607a4; crate::stubs::f_800f0308(c);
                pc = 0x802607a4;
            }
            0x802607a4 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x802607ac; f_801778b4(c);
                pc = 0x802607ac;
            }
            0x802607ac => {
                c.r[3] = c.r[1].wrapping_add(0x24);
                c.lr = 0x802607b4; f_80177830(c);
                pc = 0x802607b4;
            }
            0x802607b4 => {
                c.r[3] = c.r[1].wrapping_add(0x30);
                c.lr = 0x802607bc; f_80177908(c);
                pc = 0x802607bc;
            }
            0x802607bc => {
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.lr = 0x802607c4; f_8017795c(c);
                pc = 0x802607c4;
            }
            0x802607c4 => {
                c.lr = 0x802607c8; f_800d2584(c);
                pc = 0x802607c8;
            }
            0x802607c8 => {
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[5] = c.r[1].wrapping_add(0xc);
                c.r[6] = c.r[1].wrapping_add(0x10);
                c.r[7] = c.r[1].wrapping_add(0x14);
                c.lr = 0x802607dc; f_800d1fdc(c);
                pc = 0x802607dc;
            }
            0x802607dc => {
                c.r[22] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[12] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[11] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[10] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[9] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[8] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[7] = c.lwz(c.r[1].wrapping_add(0x30));
                c.r[6] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[5] = c.lwz(c.r[1].wrapping_add(0x38));
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x40));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[1].wrapping_add(0x48), c.r[22]);
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
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x78); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x80260848; f_800d3088(c);
                pc = 0x80260848;
            }
            0x80260848 => {
                c.cmps(0, c.r[23], 0x0);
                c.r[26] = c.r[26].wrapping_add(c.r[3]);
                if c.crbit(2) { pc = 0x80260878; continue; }
                pc = 0x80260854;
            }
            0x80260854 => {
                c.r[3] = 0x1;
                c.lr = 0x8026085c; f_800f7af0(c);
                pc = 0x8026085c;
            }
            0x8026085c => {
                c.r[22] = c.r[3] | c.r[3];
                c.r[3] = 0x1;
                c.lr = 0x80260868; f_800f7bc4(c);
                pc = 0x80260868;
            }
            0x80260868 => {
                c.r[0] = c.r[3] & c.r[22];
                c.r[0] = c.r[0].rotate_left(0) & 0x200;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80260878; continue; }
                pc = 0x80260874;
            }
            0x80260874 => {
                c.r[25] = 0x1;
                pc = 0x80260878;
            }
            0x80260878 => {
                c.cmps(0, c.r[25], 0x0);
                if c.crbit(2) { pc = 0x8026075c; continue; }
                pc = 0x80260880;
            }
            0x80260880 => {
                c.r[3] = 0x2;
                c.lr = 0x80260888; f_80177a44(c);
                pc = 0x80260888;
            }
            0x80260888 => {
                c.r[3] = c.r[1].wrapping_add(0x48);
                c.lr = 0x80260890; f_801778dc(c);
                pc = 0x80260890;
            }
            0x80260890 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80260898; f_80177858(c);
                pc = 0x80260898;
            }
            0x80260898 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802608a0; f_80177930(c);
                pc = 0x802608a0;
            }
            0x802608a0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x802608a8; f_80177984(c);
                pc = 0x802608a8;
            }
            0x802608a8 => {
                { let ea = c.r[1].wrapping_add(0x78); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x802608b0; f_801766a8(c);
                pc = 0x802608b0;
            }
            0x802608b0 => {
                c.r[0] = c.r[24] & 0x14;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x802608d8; continue; }
                pc = 0x802608b8;
            }
            0x802608b8 => {
                c.r[0] = c.r[24].rotate_left(0) & 0x4;
                c.rc(c.r[0]);
                c.r[3] = 0x5;
                if c.crbit(2) { pc = 0x802608c8; continue; }
                pc = 0x802608c4;
            }
            0x802608c4 => {
                c.r[3] = 0x3;
                pc = 0x802608c8;
            }
            0x802608c8 => {
                { let ea = c.r[2].wrapping_add(0xffffb020); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x802608d0; f_801c41c8(c);
                pc = 0x802608d0;
            }
            0x802608d0 => {
                c.r[3] = 0x1;
                c.lr = 0x802608d8; f_801c40f0(c);
                pc = 0x802608d8;
            }
            0x802608d8 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x4;
                c.lr = 0x802608e8; f_801da8c4(c);
                pc = 0x802608e8;
            }
            0x802608e8 => {
                c.cmps(0, c.r[25], 0x0);
                if c.crbit(2) { pc = 0x802608f8; continue; }
                pc = 0x802608f0;
            }
            0x802608f0 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x802608fc; continue;
                pc = 0x802608f8;
            }
            0x802608f8 => {
                c.r[3] = 0xffffffff;
                pc = 0x802608fc;
            }
            0x802608fc => {
                { let mut ea = c.r[1].wrapping_add(0x88); for n in 22..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xb4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xb0);
                if true { return; }
                return;
            }
            _ => panic!("f_802606d8: bad pc {:#x}", pc),
        }
    }
}

/// evolutionDemo (0x80260910, 1452 bytes)
#[inline(never)]
pub fn f_80260910(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80260910) { return; }
        if c.prof { crate::prof::enter(c, 0x80260910); f_80260910_b(c, 0x80260910); crate::prof::exit(c, 0x80260910); return; }
    }
    f_80260910_b(c, 0x80260910)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80260910_at(c: &mut Cpu, pc: u32) { f_80260910_b(c, pc) }
#[inline(always)]
fn f_80260910_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80260910;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80260910, pc); }
        match pc {
            0x80260910 => {
                { let ea = c.r[1].wrapping_add(0xffffff80); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x84), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x70); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = (c.f[29], c.ps1[29]); c.psq_store(ea, x, y, false, 0); }
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 18..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[3] = c.lwz(c.r[3]);
                c.r[31] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[4] = 0x1;
                c.lr = 0x80260954; f_801da4e8(c);
                pc = 0x80260954;
            }
            0x80260954 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8026095c; f_8011f4f0(c);
                pc = 0x8026095c;
            }
            0x8026095c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x32;
                c.r[4] = c.r[0] | c.r[0];
                c.lr = 0x8026096c; f_80132a38(c);
                pc = 0x8026096c;
            }
            0x8026096c => {
                c.r[3] = 0x4401;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x8026097c; f_801067e8(c);
                pc = 0x8026097c;
            }
            0x8026097c => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8026098c; continue; }
                pc = 0x80260984;
            }
            0x80260984 => {
                c.r[3] = 0x0;
                pc = 0x802609b0; continue;
                pc = 0x8026098c;
            }
            0x8026098c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80260994; f_80123fbc(c);
                pc = 0x80260994;
            }
            0x80260994 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802609a4; continue; }
                pc = 0x8026099c;
            }
            0x8026099c => {
                c.r[3] = 0x0;
                pc = 0x802609b0; continue;
                pc = 0x802609a4;
            }
            0x802609a4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802609ac; f_8011f5c8(c);
                pc = 0x802609ac;
            }
            0x802609ac => {
                c.lr = 0x802609b0; f_8011e778(c);
                pc = 0x802609b0;
            }
            0x802609b0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x802609c0; continue; }
                pc = 0x802609b8;
            }
            0x802609b8 => {
                c.r[25] = 0x0;
                pc = 0x802609d0; continue;
                pc = 0x802609c0;
            }
            0x802609c0 => {
                c.lr = 0x802609c4; f_8011e15c(c);
                pc = 0x802609c4;
            }
            0x802609c4 => {
                c.r[25] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x802609d0; f_80166a28(c);
                pc = 0x802609d0;
            }
            0x802609d0 => {
                c.r[3] = 0x80280000;
                c.r[3] = c.r[3].wrapping_add(0xffffa488);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[23] = c.r[3].wrapping_add(0x4);
                c.r[4] = c.lhz(c.r[3]);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x802609f4; continue; }
                pc = 0x802609ec;
            }
            0x802609ec => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x4));
                pc = 0x802609f8; continue;
                pc = 0x802609f4;
            }
            0x802609f4 => {
                c.r[3] = c.lwz(c.r[27]);
                pc = 0x802609f8;
            }
            0x802609f8 => {
                c.r[5] = 0x4;
                c.lr = 0x80260a00; f_801da9e8(c);
                pc = 0x80260a00;
            }
            0x80260a00 => {
                c.lr = 0x80260a04; f_801db088(c);
                pc = 0x80260a04;
            }
            0x80260a04 => {
                c.r[0] = 0x0;
                c.r[3] = 0x80280000;
                c.stw(c.r[27].wrapping_add(0x8), c.r[0]);
                c.r[26] = c.r[3].wrapping_add(0xffffa488);
                c.r[22] = 0x1;
                c.r[20] = 0x0;
                c.r[19] = 0x0;
                c.r[21] = 0x0;
                pc = 0x80260b58; continue;
                pc = 0x80260a28;
            }
            0x80260a28 => {
                c.cmps(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x80260a68; continue; }
                pc = 0x80260a30;
            }
            0x80260a30 => {
                c.cmpu(0, c.r[21], 0x78);
                if c.crbit(0) { pc = 0x80260a68; continue; }
                pc = 0x80260a38;
            }
            0x80260a38 => {
                c.cmps(0, c.r[19], 0x2);
                if c.crbit(0) { pc = 0x80260a68; continue; }
                pc = 0x80260a40;
            }
            0x80260a40 => {
                c.r[3] = 0x1;
                c.lr = 0x80260a48; f_800f7af0(c);
                pc = 0x80260a48;
            }
            0x80260a48 => {
                c.r[24] = c.r[3] | c.r[3];
                c.r[3] = 0x1;
                c.lr = 0x80260a54; f_800f7bc4(c);
                pc = 0x80260a54;
            }
            0x80260a54 => {
                c.r[0] = c.r[3] & c.r[24];
                c.r[0] = c.r[0].rotate_left(0) & 0x200;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80260a68; continue; }
                pc = 0x80260a60;
            }
            0x80260a60 => {
                c.r[20] = 0x1;
                pc = 0x80260b60; continue;
                pc = 0x80260a68;
            }
            0x80260a68 => {
                c.r[4] = c.lwz(c.r[27].wrapping_add(0x8));
                c.r[0] = c.r[4].rotate_left(3) & 0xfffffff8;
                c.r[3] = c.r[26].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80260a88; continue; }
                pc = 0x80260a80;
            }
            0x80260a80 => {
                c.r[24] = c.lwz(c.r[27].wrapping_add(0x4));
                pc = 0x80260a8c; continue;
                pc = 0x80260a88;
            }
            0x80260a88 => {
                c.r[24] = c.lwz(c.r[27]);
                pc = 0x80260a8c;
            }
            0x80260a8c => {
                c.r[0] = c.r[4].rotate_left(3) & 0xfffffff8;
                c.r[18] = c.lhz(c.r[26].wrapping_add(c.r[0]));
                c.lr = 0x80260a98; f_801db088(c);
                pc = 0x80260a98;
            }
            0x80260a98 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[18] | c.r[18];
                c.r[5] = 0x4;
                c.lr = 0x80260aa8; f_801da94c(c);
                pc = 0x80260aa8;
            }
            0x80260aa8 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_neg();
                c.r[0] = c.r[0] | c.r[3];
                c.r[0] = c.r[0].rotate_left(1) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80260b60; continue; }
                pc = 0x80260abc;
            }
            0x80260abc => {
                c.cmps(0, c.r[19], 0x1);
                if c.crbit(2) { pc = 0x80260b00; continue; }
                pc = 0x80260ac4;
            }
            0x80260ac4 => {
                if !c.crbit(0) { pc = 0x80260b34; continue; }
                pc = 0x80260ac8;
            }
            0x80260ac8 => {
                c.cmps(0, c.r[19], 0x0);
                if !c.crbit(0) { pc = 0x80260ad4; continue; }
                pc = 0x80260ad0;
            }
            0x80260ad0 => {
                pc = 0x80260b34; continue;
                pc = 0x80260ad4;
            }
            0x80260ad4 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80260adc; f_801666bc(c);
                pc = 0x80260adc;
            }
            0x80260adc => {
                c.cmps(0, c.r[3], 0x2);
                if c.crbit(2) { pc = 0x80260b34; continue; }
                pc = 0x80260ae4;
            }
            0x80260ae4 => {
                c.r[3] = 0x3d3;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x80260af4; f_80165a20(c);
                pc = 0x80260af4;
            }
            0x80260af4 => {
                c.r[29] = 0x0;
                c.r[19] = 0x1;
                pc = 0x80260b34; continue;
                pc = 0x80260b00;
            }
            0x80260b00 => {
                c.r[3] = 0x3d3;
                c.lr = 0x80260b08; f_801666bc(c);
                pc = 0x80260b08;
            }
            0x80260b08 => {
                c.cmps(0, c.r[3], 0x2);
                if c.crbit(2) { pc = 0x80260b34; continue; }
                pc = 0x80260b10;
            }
            0x80260b10 => {
                c.lr = 0x80260b14; f_800d3088(c);
                pc = 0x80260b14;
            }
            0x80260b14 => {
                c.r[29] = c.r[29].wrapping_add(c.r[3]);
                c.cmps(0, c.r[29], 0x1e);
                if c.crbit(0) { pc = 0x80260b34; continue; }
                pc = 0x80260b20;
            }
            0x80260b20 => {
                c.r[3] = 0x3d4;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x80260b30; f_80165a20(c);
                pc = 0x80260b30;
            }
            0x80260b30 => {
                c.r[19] = 0x2;
                pc = 0x80260b34;
            }
            0x80260b34 => {
                c.cmps(0, c.r[22], 0x0);
                if c.crbit(2) { pc = 0x80260b4c; continue; }
                pc = 0x80260b3c;
            }
            0x80260b3c => {
                { let ea = c.r[2].wrapping_add(0xffffb020); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x80260b48; f_801c41c8(c);
                pc = 0x80260b48;
            }
            0x80260b48 => {
                c.r[22] = 0x0;
                pc = 0x80260b4c;
            }
            0x80260b4c => {
                c.lr = 0x80260b50; crate::stubs::f_800f0308(c);
                pc = 0x80260b50;
            }
            0x80260b50 => {
                c.lr = 0x80260b54; f_800d3088(c);
                pc = 0x80260b54;
            }
            0x80260b54 => {
                c.r[21] = c.r[21].wrapping_add(c.r[3]);
                pc = 0x80260b58;
            }
            0x80260b58 => {
                c.cmpu(0, c.r[21], 0x23a);
                if c.crbit(0) { pc = 0x80260a28; continue; }
                pc = 0x80260b60;
            }
            0x80260b60 => {
                { let ea = c.r[2].wrapping_add(0xffffb020); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x5;
                c.lr = 0x80260b6c; f_801c41c8(c);
                pc = 0x80260b6c;
            }
            0x80260b6c => {
                c.r[3] = 0x80280000;
                c.r[19] = c.r[3].wrapping_add(0xffffa488);
                pc = 0x80260bbc; continue;
                pc = 0x80260b78;
            }
            0x80260b78 => {
                c.r[4] = c.lwz(c.r[27].wrapping_add(0x8));
                c.r[0] = c.r[4].rotate_left(3) & 0xfffffff8;
                c.r[3] = c.r[19].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80260b98; continue; }
                pc = 0x80260b90;
            }
            0x80260b90 => {
                c.r[22] = c.lwz(c.r[27].wrapping_add(0x4));
                pc = 0x80260b9c; continue;
                pc = 0x80260b98;
            }
            0x80260b98 => {
                c.r[22] = c.lwz(c.r[27]);
                pc = 0x80260b9c;
            }
            0x80260b9c => {
                c.r[0] = c.r[4].rotate_left(3) & 0xfffffff8;
                c.r[18] = c.lhz(c.r[19].wrapping_add(c.r[0]));
                c.lr = 0x80260ba8; f_801db088(c);
                pc = 0x80260ba8;
            }
            0x80260ba8 => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[18] | c.r[18];
                c.r[5] = 0x4;
                c.lr = 0x80260bb8; f_801da94c(c);
                pc = 0x80260bb8;
            }
            0x80260bb8 => {
                c.lr = 0x80260bbc; crate::stubs::f_800f0308(c);
                pc = 0x80260bbc;
            }
            0x80260bbc => {
                c.r[3] = 0x0;
                c.lr = 0x80260bc4; f_801c40f0(c);
                pc = 0x80260bc4;
            }
            0x80260bc4 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80260b78; continue; }
                pc = 0x80260bcc;
            }
            0x80260bcc => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x8));
                c.r[0] = c.r[5].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.lwz(c.r[23].wrapping_add(c.r[0]));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80260be8; continue; }
                pc = 0x80260be0;
            }
            0x80260be0 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x4));
                pc = 0x80260bec; continue;
                pc = 0x80260be8;
            }
            0x80260be8 => {
                c.r[3] = c.lwz(c.r[27]);
                pc = 0x80260bec;
            }
            0x80260bec => {
                c.r[4] = 0x80280000;
                c.r[0] = c.r[5].rotate_left(3) & 0xfffffff8;
                c.r[4] = c.r[4].wrapping_add(0xffffa488);
                c.r[5] = 0x4;
                c.r[4] = c.lhz(c.r[4].wrapping_add(c.r[0]));
                c.lr = 0x80260c04; f_801da8c4(c);
                pc = 0x80260c04;
            }
            0x80260c04 => {
                { let ea = c.r[2].wrapping_add(0xffffb010); c.f[29] = c.lfs(ea); c.ps1[29] = c.f[29]; }
                c.r[19] = 0x43300000;
                { let ea = c.r[2].wrapping_add(0xffffb018); c.f[31] = c.lfd(ea); }
                { let ea = c.r[2].wrapping_add(0xffffb014); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                pc = 0x80260c34; continue;
                pc = 0x80260c18;
            }
            0x80260c18 => {
                c.lr = 0x80260c1c; crate::stubs::f_800f0308(c);
                pc = 0x80260c1c;
            }
            0x80260c1c => {
                c.lr = 0x80260c20; f_800d3088(c);
                pc = 0x80260c20;
            }
            0x80260c20 => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[19]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[31]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[29] + c.f[0]); c.f[29] = v; c.ps1[29] = v; }
                pc = 0x80260c34;
            }
            0x80260c34 => {
                c.fcmp(0, c.f[29], c.f[30]);
                if c.crbit(0) { pc = 0x80260c18; continue; }
                pc = 0x80260c3c;
            }
            0x80260c3c => {
                c.cmps(0, c.r[20], 0x0);
                if c.crbit(2) { pc = 0x80260d58; continue; }
                pc = 0x80260c44;
            }
            0x80260c44 => {
                c.r[3] = 0x1;
                c.lr = 0x80260c4c; f_801065b8(c);
                pc = 0x80260c4c;
            }
            0x80260c4c => {
                c.r[3] = 0x3d4;
                c.r[4] = 0x32;
                c.lr = 0x80260c58; f_801657f8(c);
                pc = 0x80260c58;
            }
            0x80260c58 => {
                c.r[3] = 0x80280000;
                { let ea = c.r[3].wrapping_add(0xffffa4b0); c.r[0] = c.lwz(ea); c.r[3] = ea; }
                c.r[4] = 0x0;
                c.cmpu(0, c.r[0], c.r[21]);
                if !c.crbit(0) { pc = 0x80260c80; continue; }
                pc = 0x80260c6c;
            }
            0x80260c6c => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x8));
                c.r[4] = 0x1;
                c.cmpu(0, c.r[0], c.r[21]);
                if !c.crbit(0) { pc = 0x80260c80; continue; }
                pc = 0x80260c7c;
            }
            0x80260c7c => {
                c.r[4] = 0x2;
                pc = 0x80260c80;
            }
            0x80260c80 => {
                c.r[3] = 0x80280000;
                c.r[0] = c.r[4].rotate_left(3) & 0xfffffff8;
                c.r[4] = c.r[3].wrapping_add(0xffffa4b0);
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[5] = 0x0;
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[6] = 0x8;
                c.lr = 0x80260ca4; f_802606d8(c);
                pc = 0x80260ca4;
            }
            0x80260ca4 => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80260cb4; continue; }
                pc = 0x80260cac;
            }
            0x80260cac => {
                c.r[3] = 0x0;
                pc = 0x80260cd8; continue;
                pc = 0x80260cb4;
            }
            0x80260cb4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80260cbc; f_80123fbc(c);
                pc = 0x80260cbc;
            }
            0x80260cbc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80260ccc; continue; }
                pc = 0x80260cc4;
            }
            0x80260cc4 => {
                c.r[3] = 0x0;
                pc = 0x80260cd8; continue;
                pc = 0x80260ccc;
            }
            0x80260ccc => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80260cd4; f_8011f5c8(c);
                pc = 0x80260cd4;
            }
            0x80260cd4 => {
                c.lr = 0x80260cd8; f_8011e778(c);
                pc = 0x80260cd8;
            }
            0x80260cd8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80260ce8; continue; }
                pc = 0x80260ce0;
            }
            0x80260ce0 => {
                c.r[18] = 0x0;
                pc = 0x80260d00; continue;
                pc = 0x80260ce8;
            }
            0x80260ce8 => {
                c.lr = 0x80260cec; f_8011e15c(c);
                pc = 0x80260cec;
            }
            0x80260cec => {
                c.r[18] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[18] | c.r[18];
                c.lr = 0x80260cf8; f_80166a28(c);
                pc = 0x80260cf8;
            }
            0x80260cf8 => {
                pc = 0x80260d00; continue;
                pc = 0x80260cfc;
            }
            0x80260cfc => {
                c.lr = 0x80260d00; crate::stubs::f_800f0308(c);
                pc = 0x80260d00;
            }
            0x80260d00 => {
                c.r[3] = c.r[18] | c.r[18];
                c.lr = 0x80260d08; f_801666bc(c);
                pc = 0x80260d08;
            }
            0x80260d08 => {
                c.cmps(0, c.r[3], 0x2);
                if c.crbit(2) { pc = 0x80260cfc; continue; }
                pc = 0x80260d10;
            }
            0x80260d10 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80260d18; f_8011f4f0(c);
                pc = 0x80260d18;
            }
            0x80260d18 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x32;
                c.lr = 0x80260d24; f_80132a38(c);
                pc = 0x80260d24;
            }
            0x80260d24 => {
                c.r[3] = 0x43ff;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x80260d34; f_801067e8(c);
                pc = 0x80260d34;
            }
            0x80260d34 => {
                c.r[3] = 0x1;
                c.lr = 0x80260d3c; f_801065b8(c);
                pc = 0x80260d3c;
            }
            0x80260d3c => {
                { let ea = c.r[2].wrapping_add(0xffffb020); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x3;
                c.lr = 0x80260d48; f_801c41c8(c);
                pc = 0x80260d48;
            }
            0x80260d48 => {
                c.r[3] = 0x1;
                c.lr = 0x80260d50; f_801c40f0(c);
                pc = 0x80260d50;
            }
            0x80260d50 => {
                c.r[3] = 0x0;
                pc = 0x80260e90; continue;
                pc = 0x80260d58;
            }
            0x80260d58 => {
                c.r[3] = c.lwz(c.r[27]);
                c.r[4] = 0x0;
                c.lr = 0x80260d64; f_801da4e8(c);
                pc = 0x80260d64;
            }
            0x80260d64 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x4));
                c.r[4] = 0x1;
                c.lr = 0x80260d70; f_801da4e8(c);
                pc = 0x80260d70;
            }
            0x80260d70 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.r[6] = 0x8;
                c.lr = 0x80260d84; f_802606d8(c);
                pc = 0x80260d84;
            }
            0x80260d84 => {
                c.r[3] = 0x1;
                c.lr = 0x80260d8c; f_801065b8(c);
                pc = 0x80260d8c;
            }
            0x80260d8c => {
                c.r[3] = 0x3d4;
                c.r[4] = 0x32;
                c.lr = 0x80260d98; f_801657f8(c);
                pc = 0x80260d98;
            }
            0x80260d98 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80260da8; continue; }
                pc = 0x80260da0;
            }
            0x80260da0 => {
                c.r[3] = 0x0;
                pc = 0x80260dcc; continue;
                pc = 0x80260da8;
            }
            0x80260da8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80260db0; f_80123fbc(c);
                pc = 0x80260db0;
            }
            0x80260db0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80260dc0; continue; }
                pc = 0x80260db8;
            }
            0x80260db8 => {
                c.r[3] = 0x0;
                pc = 0x80260dcc; continue;
                pc = 0x80260dc0;
            }
            0x80260dc0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80260dc8; f_8011f5c8(c);
                pc = 0x80260dc8;
            }
            0x80260dc8 => {
                c.lr = 0x80260dcc; f_8011e778(c);
                pc = 0x80260dcc;
            }
            0x80260dcc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80260ddc; continue; }
                pc = 0x80260dd4;
            }
            0x80260dd4 => {
                c.r[18] = 0x0;
                pc = 0x80260df4; continue;
                pc = 0x80260ddc;
            }
            0x80260ddc => {
                c.lr = 0x80260de0; f_8011e15c(c);
                pc = 0x80260de0;
            }
            0x80260de0 => {
                c.r[18] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[18] | c.r[18];
                c.lr = 0x80260dec; f_80166a28(c);
                pc = 0x80260dec;
            }
            0x80260dec => {
                pc = 0x80260df4; continue;
                pc = 0x80260df0;
            }
            0x80260df0 => {
                c.lr = 0x80260df4; crate::stubs::f_800f0308(c);
                pc = 0x80260df4;
            }
            0x80260df4 => {
                c.r[3] = c.r[18] | c.r[18];
                c.lr = 0x80260dfc; f_801666bc(c);
                pc = 0x80260dfc;
            }
            0x80260dfc => {
                c.cmps(0, c.r[3], 0x2);
                if c.crbit(2) { pc = 0x80260df0; continue; }
                pc = 0x80260e04;
            }
            0x80260e04 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80260e0c; f_8011f4f0(c);
                pc = 0x80260e0c;
            }
            0x80260e0c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x32;
                c.lr = 0x80260e18; f_80132a38(c);
                pc = 0x80260e18;
            }
            0x80260e18 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80260e20; f_8011f5c8(c);
                pc = 0x80260e20;
            }
            0x80260e20 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x4e;
                c.r[4] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x80260e30; f_80132a38(c);
                pc = 0x80260e30;
            }
            0x80260e30 => {
                c.r[3] = 0x5d;
                c.r[4] = 0x3d2;
                c.lr = 0x80260e3c; f_80132a38(c);
                pc = 0x80260e3c;
            }
            0x80260e3c => {
                c.r[3] = 0x4400;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x80260e4c; f_801067e8(c);
                pc = 0x80260e4c;
            }
            0x80260e4c => {
                c.r[3] = 0x1;
                c.lr = 0x80260e54; f_801065b8(c);
                pc = 0x80260e54;
            }
            0x80260e54 => {
                { let ea = c.r[2].wrapping_add(0xffffb010); c.f[29] = c.lfs(ea); c.ps1[29] = c.f[29]; }
                c.r[19] = 0x43300000;
                { let ea = c.r[2].wrapping_add(0xffffb018); c.f[30] = c.lfd(ea); }
                { let ea = c.r[2].wrapping_add(0xffffb024); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                pc = 0x80260e84; continue;
                pc = 0x80260e68;
            }
            0x80260e68 => {
                c.lr = 0x80260e6c; crate::stubs::f_800f0308(c);
                pc = 0x80260e6c;
            }
            0x80260e6c => {
                c.lr = 0x80260e70; f_800d3088(c);
                pc = 0x80260e70;
            }
            0x80260e70 => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[19]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[30]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[29] + c.f[0]); c.f[29] = v; c.ps1[29] = v; }
                pc = 0x80260e84;
            }
            0x80260e84 => {
                c.fcmp(0, c.f[29], c.f[31]);
                if c.crbit(0) { pc = 0x80260e68; continue; }
                pc = 0x80260e8c;
            }
            0x80260e8c => {
                c.r[3] = 0x1;
                pc = 0x80260e90;
            }
            0x80260e90 => {
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x70); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x60); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                { let ea = c.r[1].wrapping_add(0x50); c.f[29] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 18..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x84));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x80);
                if true { return; }
                return;
            }
            _ => panic!("f_80260910: bad pc {:#x}", pc),
        }
    }
}

/// evolutionStart (0x80260ebc, 1044 bytes)
#[inline(never)]
pub fn f_80260ebc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80260ebc) { return; }
        if c.prof { crate::prof::enter(c, 0x80260ebc); f_80260ebc_b(c, 0x80260ebc); crate::prof::exit(c, 0x80260ebc); return; }
    }
    f_80260ebc_b(c, 0x80260ebc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80260ebc_at(c: &mut Cpu, pc: u32) { f_80260ebc_b(c, pc) }
#[inline(always)]
fn f_80260ebc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80260ebc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80260ebc, pc); }
        match pc {
            0x80260ebc => {
                { let ea = c.r[1].wrapping_add(0xfffffe70); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x194), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x168); for n in 22..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[26] = c.r[5] | c.r[5];
                c.r[28] = c.r[6] | c.r[6];
                c.r[31] = c.r[7] | c.r[7];
                c.r[27] = c.r[8] | c.r[8];
                c.r[3] = 0x2;
                c.lr = 0x80260eec; f_801dadc0(c);
                pc = 0x80260eec;
            }
            0x80260eec => {
                c.r[0] = 0x0;
                c.r[3] = c.r[29] | c.r[29];
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x80260f0c; f_8012640c(c);
                pc = 0x80260f0c;
            }
            0x80260f0c => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[4]);
                if !c.crbit(2) { pc = 0x80260f20; continue; }
                pc = 0x80260f14;
            }
            0x80260f14 => {
                c.r[3] = 0x10000;
                c.r[23] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x80260f40; continue;
                pc = 0x80260f20;
            }
            0x80260f20 => {
                c.r[3] = 0x0;
                c.r[5] = 0x66;
                c.r[6] = 0x0;
                c.lr = 0x80260f30; f_8012640c(c);
                pc = 0x80260f30;
            }
            0x80260f30 => {
                c.r[23] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[23]);
                if !c.crbit(2) { pc = 0x80260f40; continue; }
                pc = 0x80260f38;
            }
            0x80260f38 => {
                c.r[3] = 0x10000;
                c.r[23] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x80260f40;
            }
            0x80260f40 => {
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xffff);
                if !c.crbit(2) { pc = 0x80260f54; continue; }
                pc = 0x80260f4c;
            }
            0x80260f4c => {
                c.r[22] = 0x0;
                pc = 0x80260fec; continue;
                pc = 0x80260f54;
            }
            0x80260f54 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80260f5c; f_8011f5b0(c);
                pc = 0x80260f5c;
            }
            0x80260f5c => {
                c.r[22] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xc1;
                c.r[6] = 0x0;
                c.lr = 0x80260f74; f_8012640c(c);
                pc = 0x80260f74;
            }
            0x80260f74 => {
                c.r[0] = c.r[3].wrapping_neg();
                c.r[4] = c.r[22] | c.r[22];
                c.r[0] = c.r[0] | c.r[3];
                c.r[3] = c.r[23] | c.r[23];
                c.r[5] = c.r[0].rotate_left(1) & 0x1;
                c.lr = 0x80260f8c; f_801de190(c);
                pc = 0x80260f8c;
            }
            0x80260f8c => {
                c.r[22] = c.r[3] | c.r[3];
                c.rc(c.r[22]);
                if !c.crbit(2) { pc = 0x80260f9c; continue; }
                pc = 0x80260f94;
            }
            0x80260f94 => {
                c.r[22] = 0x0;
                pc = 0x80260fec; continue;
                pc = 0x80260f9c;
            }
            0x80260f9c => {
                c.r[3] = 0x80280000;
                c.r[23] = 0x0;
                c.r[24] = c.r[3].wrapping_add(0xffffa488);
                pc = 0x80260fa8;
            }
            0x80260fa8 => {
                c.r[0] = c.lwz(c.r[24].wrapping_add(0x4));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80260fd0; continue; }
                pc = 0x80260fb4;
            }
            0x80260fb4 => {
                c.r[4] = c.lhz(c.r[24]);
                c.r[3] = c.r[22] | c.r[22];
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x80260fc8; f_801ddd28(c);
                pc = 0x80260fc8;
            }
            0x80260fc8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80260fe0; continue; }
                pc = 0x80260fd0;
            }
            0x80260fd0 => {
                c.r[23] = c.r[23].wrapping_add(0x1);
                c.r[24] = c.r[24].wrapping_add(0x8);
                c.cmps(0, c.r[23], 0x5);
                if c.crbit(0) { pc = 0x80260fa8; continue; }
                pc = 0x80260fe0;
            }
            0x80260fe0 => {
                c.cmps(0, c.r[23], 0x5);
                if !c.crbit(0) { pc = 0x80260fec; continue; }
                pc = 0x80260fe8;
            }
            0x80260fe8 => {
                c.r[22] = 0x0;
                pc = 0x80260fec;
            }
            0x80260fec => {
                c.cmpu(0, c.r[22], 0x0);
                if !c.crbit(2) { pc = 0x80260ffc; continue; }
                pc = 0x80260ff4;
            }
            0x80260ff4 => {
                c.r[0] = 0x0;
                pc = 0x80261004; continue;
                pc = 0x80260ffc;
            }
            0x80260ffc => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[22]);
                c.r[0] = 0x1;
                pc = 0x80261004;
            }
            0x80261004 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80261128; continue; }
                pc = 0x8026100c;
            }
            0x8026100c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x80261020; f_8012640c(c);
                pc = 0x80261020;
            }
            0x80261020 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[4]);
                if !c.crbit(2) { pc = 0x80261034; continue; }
                pc = 0x80261028;
            }
            0x80261028 => {
                c.r[3] = 0x10000;
                c.r[23] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x80261054; continue;
                pc = 0x80261034;
            }
            0x80261034 => {
                c.r[3] = 0x0;
                c.r[5] = 0x66;
                c.r[6] = 0x0;
                c.lr = 0x80261044; f_8012640c(c);
                pc = 0x80261044;
            }
            0x80261044 => {
                c.r[23] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[23]);
                if !c.crbit(2) { pc = 0x80261054; continue; }
                pc = 0x8026104c;
            }
            0x8026104c => {
                c.r[3] = 0x10000;
                c.r[23] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x80261054;
            }
            0x80261054 => {
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xffff);
                if !c.crbit(2) { pc = 0x80261068; continue; }
                pc = 0x80261060;
            }
            0x80261060 => {
                c.r[22] = 0x0;
                pc = 0x80261100; continue;
                pc = 0x80261068;
            }
            0x80261068 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80261070; f_8011f5b0(c);
                pc = 0x80261070;
            }
            0x80261070 => {
                c.r[22] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xc1;
                c.r[6] = 0x0;
                c.lr = 0x80261088; f_8012640c(c);
                pc = 0x80261088;
            }
            0x80261088 => {
                c.r[0] = c.r[3].wrapping_neg();
                c.r[4] = c.r[22] | c.r[22];
                c.r[0] = c.r[0] | c.r[3];
                c.r[3] = c.r[23] | c.r[23];
                c.r[5] = c.r[0].rotate_left(1) & 0x1;
                c.lr = 0x802610a0; f_801de190(c);
                pc = 0x802610a0;
            }
            0x802610a0 => {
                c.r[22] = c.r[3] | c.r[3];
                c.rc(c.r[22]);
                if !c.crbit(2) { pc = 0x802610b0; continue; }
                pc = 0x802610a8;
            }
            0x802610a8 => {
                c.r[22] = 0x0;
                pc = 0x80261100; continue;
                pc = 0x802610b0;
            }
            0x802610b0 => {
                c.r[3] = 0x80280000;
                c.r[23] = 0x0;
                c.r[24] = c.r[3].wrapping_add(0xffffa488);
                pc = 0x802610bc;
            }
            0x802610bc => {
                c.r[0] = c.lwz(c.r[24].wrapping_add(0x4));
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802610e4; continue; }
                pc = 0x802610c8;
            }
            0x802610c8 => {
                c.r[4] = c.lhz(c.r[24]);
                c.r[3] = c.r[22] | c.r[22];
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x802610dc; f_801ddd28(c);
                pc = 0x802610dc;
            }
            0x802610dc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x802610f4; continue; }
                pc = 0x802610e4;
            }
            0x802610e4 => {
                c.r[23] = c.r[23].wrapping_add(0x1);
                c.r[24] = c.r[24].wrapping_add(0x8);
                c.cmps(0, c.r[23], 0x5);
                if c.crbit(0) { pc = 0x802610bc; continue; }
                pc = 0x802610f4;
            }
            0x802610f4 => {
                c.cmps(0, c.r[23], 0x5);
                if !c.crbit(0) { pc = 0x80261100; continue; }
                pc = 0x802610fc;
            }
            0x802610fc => {
                c.r[22] = 0x0;
                pc = 0x80261100;
            }
            0x80261100 => {
                c.cmpu(0, c.r[22], 0x0);
                if !c.crbit(2) { pc = 0x80261110; continue; }
                pc = 0x80261108;
            }
            0x80261108 => {
                c.r[0] = 0x0;
                pc = 0x80261118; continue;
                pc = 0x80261110;
            }
            0x80261110 => {
                c.stw(c.r[1].wrapping_add(0x10), c.r[22]);
                c.r[0] = 0x1;
                pc = 0x80261118;
            }
            0x80261118 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80261128; continue; }
                pc = 0x80261120;
            }
            0x80261120 => {
                c.r[0] = 0x1;
                pc = 0x80261130; continue;
                pc = 0x80261128;
            }
            0x80261128 => {
                c.lr = 0x8026112c; f_801dac90(c);
                pc = 0x8026112c;
            }
            0x8026112c => {
                c.r[0] = 0x0;
                pc = 0x80261130;
            }
            0x80261130 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80261140; continue; }
                pc = 0x80261138;
            }
            0x80261138 => {
                c.r[3] = 0x2;
                pc = 0x802612bc; continue;
                pc = 0x80261140;
            }
            0x80261140 => {
                c.r[25] = c.r[27] | c.r[27];
                c.r[27] = c.r[28] | c.r[28];
                c.lr = 0x8026114c; f_801653c4(c);
                pc = 0x8026114c;
            }
            0x8026114c => {
                c.r[24] = c.r[3] | c.r[3];
                c.rc(c.r[24]);
                if c.crbit(2) { pc = 0x80261170; continue; }
                pc = 0x80261154;
            }
            0x80261154 => {
                c.lr = 0x80261158; f_801656d8(c);
                pc = 0x80261158;
            }
            0x80261158 => {
                c.r[22] = c.r[3] | c.r[3];
                c.r[3] = 0x1;
                c.r[4] = 0x32;
                c.r[5] = 0xff;
                c.lr = 0x8026116c; f_80165a20(c);
                pc = 0x8026116c;
            }
            0x8026116c => {
                pc = 0x80261174; continue;
                pc = 0x80261170;
            }
            0x80261170 => {
                c.r[22] = 0x0;
                pc = 0x80261174;
            }
            0x80261174 => {
                c.lr = 0x80261178; f_801653bc(c);
                pc = 0x80261178;
            }
            0x80261178 => {
                c.r[28] = c.r[3] | c.r[3];
                c.rc(c.r[28]);
                if c.crbit(2) { pc = 0x80261194; continue; }
                pc = 0x80261180;
            }
            0x80261180 => {
                c.lr = 0x80261184; f_801656d8(c);
                pc = 0x80261184;
            }
            0x80261184 => {
                c.r[23] = c.r[3] | c.r[3];
                c.r[3] = 0x32;
                c.lr = 0x80261190; f_801657d0(c);
                pc = 0x80261190;
            }
            0x80261190 => {
                pc = 0x80261198; continue;
                pc = 0x80261194;
            }
            0x80261194 => {
                c.r[23] = 0x0;
                pc = 0x80261198;
            }
            0x80261198 => {
                c.stw(c.r[1].wrapping_add(0x18), c.r[24]);
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.stw(c.r[1].wrapping_add(0x1c), c.r[28]);
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.stw(c.r[1].wrapping_add(0x20), c.r[22]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[23]);
                c.lr = 0x802611bc; f_80260910(c);
                pc = 0x802611bc;
            }
            0x802611bc => {
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[26] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x802611e0; continue; }
                pc = 0x802611cc;
            }
            0x802611cc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x32;
                c.r[5] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x802611e0; f_80165a20(c);
                pc = 0x802611e0;
            }
            0x802611e0 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x802611fc; continue; }
                pc = 0x802611ec;
            }
            0x802611ec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[4] = 0x32;
                c.r[5] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x802611fc; f_801659fc(c);
                pc = 0x802611fc;
            }
            0x802611fc => {
                c.cmps(0, c.r[26], 0x0);
                if !c.crbit(2) { pc = 0x8026120c; continue; }
                pc = 0x80261204;
            }
            0x80261204 => {
                c.r[24] = 0x0;
                pc = 0x802612a4; continue;
                pc = 0x8026120c;
            }
            0x8026120c => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[1].wrapping_add(0x28);
                c.lr = 0x80261218; f_8011f5fc(c);
                pc = 0x80261218;
            }
            0x80261218 => {
                c.r[3] = 0x80260000;
                c.r[22] = c.r[1].wrapping_add(0xc);
                c.r[28] = c.r[3].wrapping_add(0x45c);
                c.r[29] = 0x0;
                c.r[26] = 0xff;
                pc = 0x80261284; continue;
                pc = 0x80261230;
            }
            0x80261230 => {
                c.r[24] = c.lhz(c.r[27]);
                c.r[7] = c.r[28] | c.r[28];
                c.r[8] = c.r[22] | c.r[22];
                c.r[3] = c.r[1].wrapping_add(0x28);
                c.r[4] = c.r[24] | c.r[24];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[6] = 0x0;
                c.lr = 0x80261250; f_802600e4(c);
                pc = 0x80261250;
            }
            0x80261250 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8026126c; continue; }
                pc = 0x80261258;
            }
            0x80261258 => {
                c.r[4] = c.lbz(c.r[1].wrapping_add(0x8));
                c.r[5] = c.r[24] | c.r[24];
                c.r[3] = c.r[1].wrapping_add(0x28);
                c.lr = 0x80261268; f_80123d58(c);
                pc = 0x80261268;
            }
            0x80261268 => {
                pc = 0x80261270; continue;
                pc = 0x8026126c;
            }
            0x8026126c => {
                c.stb(c.r[1].wrapping_add(0x8), c.r[26]);
                pc = 0x80261270;
            }
            0x80261270 => {
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x8));
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[27] = c.r[27].wrapping_add(0x2);
                c.stb(c.r[25], c.r[0]);
                c.r[25] = c.r[25].wrapping_add(0x1);
                pc = 0x80261284;
            }
            0x80261284 => {
                c.cmps(0, c.r[29], c.r[31]);
                if c.crbit(0) { pc = 0x80261230; continue; }
                pc = 0x8026128c;
            }
            0x8026128c => {
                { let ea = c.r[2].wrapping_add(0xffffb020); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x3;
                c.lr = 0x80261298; f_801c41c8(c);
                pc = 0x80261298;
            }
            0x80261298 => {
                c.r[3] = 0x1;
                c.lr = 0x802612a0; f_801c40f0(c);
                pc = 0x802612a0;
            }
            0x802612a0 => {
                c.r[24] = 0x1;
                pc = 0x802612a4;
            }
            0x802612a4 => {
                c.lr = 0x802612a8; f_801dac90(c);
                pc = 0x802612a8;
            }
            0x802612a8 => {
                c.cmps(0, c.r[24], 0x0);
                if c.crbit(2) { pc = 0x802612b8; continue; }
                pc = 0x802612b0;
            }
            0x802612b0 => {
                c.r[3] = 0x0;
                pc = 0x802612bc; continue;
                pc = 0x802612b8;
            }
            0x802612b8 => {
                c.r[3] = 0x1;
                pc = 0x802612bc;
            }
            0x802612bc => {
                { let mut ea = c.r[1].wrapping_add(0x168); for n in 22..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x194));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x190);
                if true { return; }
                return;
            }
            _ => panic!("f_80260ebc: bad pc {:#x}", pc),
        }
    }
}

/// evolution (0x802612d0, 92 bytes)
#[inline(never)]
pub fn f_802612d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802612d0) { return; }
        if c.prof { crate::prof::enter(c, 0x802612d0); f_802612d0_b(c, 0x802612d0); crate::prof::exit(c, 0x802612d0); return; }
    }
    f_802612d0_b(c, 0x802612d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802612d0_at(c: &mut Cpu, pc: u32) { f_802612d0_b(c, pc) }
#[inline(always)]
fn f_802612d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802612d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802612d0, pc); }
        match pc {
            0x802612d0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[8] = c.r[3].wrapping_add(0xffff87e0);
                c.r[3] = c.lwz(c.r[8]);
                c.r[4] = c.lwz(c.r[8].wrapping_add(0x4));
                c.r[5] = c.lwz(c.r[8].wrapping_add(0x8));
                c.r[6] = c.lwz(c.r[8].wrapping_add(0x10));
                c.r[7] = c.lwz(c.r[8].wrapping_add(0xc));
                c.r[8] = c.lwz(c.r[8].wrapping_add(0x14));
                c.lr = 0x80261300; f_80260ebc(c);
                pc = 0x80261300;
            }
            0x80261300 => {
                c.r[4] = 0x80480000;
                c.r[4] = c.r[4].wrapping_add(0xffff87e0);
                c.stw(c.r[4].wrapping_add(0x18), c.r[3]);
                c.lr = 0x80261310; f_800ff660(c);
                pc = 0x80261310;
            }
            0x80261310 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8026131c; f_8011288c(c);
                pc = 0x8026131c;
            }
            0x8026131c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802612d0: bad pc {:#x}", pc),
        }
    }
}

/// evolutionOpen (0x8026132c, 92 bytes)
#[inline(never)]
pub fn f_8026132c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8026132c) { return; }
        if c.prof { crate::prof::enter(c, 0x8026132c); f_8026132c_b(c, 0x8026132c); crate::prof::exit(c, 0x8026132c); return; }
    }
    f_8026132c_b(c, 0x8026132c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8026132c_at(c: &mut Cpu, pc: u32) { f_8026132c_b(c, pc) }
#[inline(always)]
fn f_8026132c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8026132c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8026132c, pc); }
        match pc {
            0x8026132c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[9] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let ea = c.r[9].wrapping_add(0xffff87e0); c.stw(ea, c.r[3]); c.r[9] = ea; }
                c.r[3] = 0x386;
                c.stw(c.r[9].wrapping_add(0x4), c.r[4]);
                c.stw(c.r[9].wrapping_add(0x8), c.r[5]);
                c.stw(c.r[9].wrapping_add(0xc), c.r[7]);
                c.stw(c.r[9].wrapping_add(0x10), c.r[6]);
                c.stw(c.r[9].wrapping_add(0x14), c.r[8]);
                c.lr = 0x8026135c; f_800ff730(c);
                pc = 0x8026135c;
            }
            0x8026135c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80261368; f_8011288c(c);
                pc = 0x80261368;
            }
            0x80261368 => {
                c.lr = 0x8026136c; crate::stubs::f_800f0308(c);
                pc = 0x8026136c;
            }
            0x8026136c => {
                c.r[3] = 0x80480000;
                c.r[3] = c.r[3].wrapping_add(0xffff87e0);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x18));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8026132c: bad pc {:#x}", pc),
        }
    }
}

/// charNameBiosSearchIndex (0x80261388, 76 bytes)
#[inline(never)]
pub fn f_80261388(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80261388) { return; }
        if c.prof { crate::prof::enter(c, 0x80261388); f_80261388_b(c, 0x80261388); crate::prof::exit(c, 0x80261388); return; }
    }
    f_80261388_b(c, 0x80261388)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80261388_at(c: &mut Cpu, pc: u32) { f_80261388_b(c, pc) }
#[inline(always)]
fn f_80261388_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80261388;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80261388, pc); }
        match pc {
            0x80261388 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8760));
                c.r[7] = 0x0;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff8764));
                c.r[6] = c.lwz(c.r[4]);
                pc = 0x802613c0; continue;
                pc = 0x8026139c;
            }
            0x8026139c => {
                c.r[0] = c.r[7].rotate_left(0) & 0xffff;
                c.r[4] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[4].wrapping_add(0x4);
                c.r[0] = c.lwz(c.r[5].wrapping_add(c.r[0]));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x802613bc; continue; }
                pc = 0x802613b4;
            }
            0x802613b4 => {
                c.r[3] = c.r[7] | c.r[7];
                if true { return; }
                pc = 0x802613bc;
            }
            0x802613bc => {
                c.r[7] = c.r[7].wrapping_add(0x1);
                pc = 0x802613c0;
            }
            0x802613c0 => {
                c.r[0] = c.r[7].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[6]);
                if c.crbit(0) { pc = 0x8026139c; continue; }
                pc = 0x802613cc;
            }
            0x802613cc => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80261388: bad pc {:#x}", pc),
        }
    }
}

/// charNameBiosGetNameID (0x802613d4, 112 bytes)
#[inline(never)]
pub fn f_802613d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802613d4) { return; }
        if c.prof { crate::prof::enter(c, 0x802613d4); f_802613d4_b(c, 0x802613d4); crate::prof::exit(c, 0x802613d4); return; }
    }
    f_802613d4_b(c, 0x802613d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802613d4_at(c: &mut Cpu, pc: u32) { f_802613d4_b(c, pc) }
#[inline(always)]
fn f_802613d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802613d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802613d4, pc); }
        match pc {
            0x802613d4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8760));
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[5], c.r[0]);
                if c.crbit(0) { pc = 0x80261414; continue; }
                pc = 0x802613f4;
            }
            0x802613f4 => {
                c.r[3] = 0x80280000;
                c.r[4] = 0x803a0000;
                c.r[3] = c.r[3].wrapping_add(0xffffa4c8);
                c.r[4] = c.r[4].wrapping_add(0xffffa6c8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8026140c; stubs_gen::f_800dd970(c);
                pc = 0x8026140c;
            }
            0x8026140c => {
                c.r[3] = 0x0;
                pc = 0x80261420; continue;
                pc = 0x80261414;
            }
            0x80261414 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8764));
                c.r[0] = c.r[5].rotate_left(3) & 0xfffffff8;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x80261420;
            }
            0x80261420 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80261430; continue; }
                pc = 0x80261428;
            }
            0x80261428 => {
                c.r[3] = 0x0;
                pc = 0x80261434; continue;
                pc = 0x80261430;
            }
            0x80261430 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                pc = 0x80261434;
            }
            0x80261434 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802613d4: bad pc {:#x}", pc),
        }
    }
}

/// charNameBiosGetHearFlag (0x80261444, 112 bytes)
#[inline(never)]
pub fn f_80261444(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80261444) { return; }
        if c.prof { crate::prof::enter(c, 0x80261444); f_80261444_b(c, 0x80261444); crate::prof::exit(c, 0x80261444); return; }
    }
    f_80261444_b(c, 0x80261444)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80261444_at(c: &mut Cpu, pc: u32) { f_80261444_b(c, pc) }
#[inline(always)]
fn f_80261444_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80261444;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80261444, pc); }
        match pc {
            0x80261444 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8760));
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[5], c.r[0]);
                if c.crbit(0) { pc = 0x80261484; continue; }
                pc = 0x80261464;
            }
            0x80261464 => {
                c.r[3] = 0x80280000;
                c.r[4] = 0x803a0000;
                c.r[3] = c.r[3].wrapping_add(0xffffa4c8);
                c.r[4] = c.r[4].wrapping_add(0xffffa6c8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8026147c; stubs_gen::f_800dd970(c);
                pc = 0x8026147c;
            }
            0x8026147c => {
                c.r[3] = 0x0;
                pc = 0x80261490; continue;
                pc = 0x80261484;
            }
            0x80261484 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8764));
                c.r[0] = c.r[5].rotate_left(3) & 0xfffffff8;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x80261490;
            }
            0x80261490 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x802614a0; continue; }
                pc = 0x80261498;
            }
            0x80261498 => {
                c.r[3] = 0x0;
                pc = 0x802614a4; continue;
                pc = 0x802614a0;
            }
            0x802614a0 => {
                c.r[3] = c.lhz(c.r[3]);
                pc = 0x802614a4;
            }
            0x802614a4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80261444: bad pc {:#x}", pc),
        }
    }
}

/// fightGSfloorGetPushDataSize (0x802614b4, 136 bytes)
#[inline(never)]
pub fn f_802614b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802614b4) { return; }
        if c.prof { crate::prof::enter(c, 0x802614b4); f_802614b4_b(c, 0x802614b4); crate::prof::exit(c, 0x802614b4); return; }
    }
    f_802614b4_b(c, 0x802614b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802614b4_at(c: &mut Cpu, pc: u32) { f_802614b4_b(c, pc) }
#[inline(always)]
fn f_802614b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802614b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802614b4, pc); }
        match pc {
            0x802614b4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80260000;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x184c);
                c.r[0] = 0x0;
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[3] = 0x0;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.stb(c.r[1].wrapping_add(0x9), c.r[0]);
                c.sth(c.r[1].wrapping_add(0xa), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x802614f0; f_801f2b5c(c);
                pc = 0x802614f0;
            }
            0x802614f0 => {
                c.r[31] = c.lhz(c.r[1].wrapping_add(0xa));
                c.r[0] = 0x0;
                c.r[3] = 0x80260000;
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.sth(c.r[1].wrapping_add(0xa), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x1708);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80261514; f_801f37b0(c);
                pc = 0x80261514;
            }
            0x80261514 => {
                c.r[0] = c.lhz(c.r[1].wrapping_add(0xa));
                c.r[3] = c.r[31].wrapping_mul(0x78);
                c.r[0] = c.r[0].wrapping_mul(0x7c);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x48);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802614b4: bad pc {:#x}", pc),
        }
    }
}

/// fightGSfloorPushData (0x8026153c, 184 bytes)
#[inline(never)]
pub fn f_8026153c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8026153c) { return; }
        if c.prof { crate::prof::enter(c, 0x8026153c); f_8026153c_b(c, 0x8026153c); crate::prof::exit(c, 0x8026153c); return; }
    }
    f_8026153c_b(c, 0x8026153c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8026153c_at(c: &mut Cpu, pc: u32) { f_8026153c_b(c, pc) }
#[inline(always)]
fn f_8026153c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8026153c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8026153c, pc); }
        match pc {
            0x8026153c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[30].wrapping_add(0x4);
                c.lr = 0x80261558; f_801c3108(c);
                pc = 0x80261558;
            }
            0x80261558 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = 0x44;
                c.lr = 0x80261568; f_800054f4(c);
                pc = 0x80261568;
            }
            0x80261568 => {
                c.r[6] = 0x1;
                c.r[0] = c.r[31].wrapping_add(0x44);
                c.r[7] = 0x0;
                c.r[3] = 0x80260000;
                c.r[4] = c.r[3].wrapping_add(0x184c);
                c.stb(c.r[1].wrapping_add(0x8), c.r[6]);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.stb(c.r[1].wrapping_add(0x9), c.r[6]);
                c.r[6] = 0x0;
                c.sth(c.r[1].wrapping_add(0xa), c.r[7]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x802615a0; f_801f2b5c(c);
                pc = 0x802615a0;
            }
            0x802615a0 => {
                c.r[31] = c.lhz(c.r[1].wrapping_add(0xa));
                c.r[0] = 0x0;
                c.r[3] = 0x80260000;
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.sth(c.r[1].wrapping_add(0xa), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x1708);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802615c4; f_801f37b0(c);
                pc = 0x802615c4;
            }
            0x802615c4 => {
                c.r[0] = c.lhz(c.r[1].wrapping_add(0xa));
                c.r[3] = 0x1;
                c.sth(c.r[30], c.r[31]);
                c.sth(c.r[30].wrapping_add(0x2), c.r[0]);
                c.lr = 0x802615d8; f_801ef8f4(c);
                pc = 0x802615d8;
            }
            0x802615d8 => {
                c.lr = 0x802615dc; f_801c3114(c);
                pc = 0x802615dc;
            }
            0x802615dc => {
                c.lr = 0x802615e0; f_801dac90(c);
                pc = 0x802615e0;
            }
            0x802615e0 => {
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8026153c: bad pc {:#x}", pc),
        }
    }
}

/// fightGSfloorPopData (0x802615f4, 276 bytes)
#[inline(never)]
pub fn f_802615f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802615f4) { return; }
        if c.prof { crate::prof::enter(c, 0x802615f4); f_802615f4_b(c, 0x802615f4); crate::prof::exit(c, 0x802615f4); return; }
    }
    f_802615f4_b(c, 0x802615f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802615f4_at(c: &mut Cpu, pc: u32) { f_802615f4_b(c, pc) }
#[inline(always)]
fn f_802615f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802615f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802615f4, pc); }
        match pc {
            0x802615f4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = 0xa;
                c.r[27] = c.r[28].wrapping_add(0x4);
                c.lr = 0x80261614; f_801daef8(c);
                pc = 0x80261614;
            }
            0x80261614 => {
                c.r[29] = c.lhz(c.r[28]);
                c.r[30] = c.r[27].wrapping_add(0x44);
                c.r[28] = c.lhz(c.r[28].wrapping_add(0x2));
                pc = 0x8026165c; continue;
                pc = 0x80261624;
            }
            0x80261624 => {
                c.r[3] = c.lhz(c.r[30].wrapping_add(0x4));
                c.lr = 0x8026162c; f_801de418(c);
                pc = 0x8026162c;
            }
            0x8026162c => {
                c.r[4] = c.lwz(c.r[30]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[4].wrapping_add(0x27c0), c.r[31]);
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x8));
                c.stw(c.r[4], c.r[31]);
                c.lr = 0x80261644; f_801dac3c(c);
                pc = 0x80261644;
            }
            0x80261644 => {
                c.r[4] = c.r[30].wrapping_add(0xc);
                c.lr = 0x8026164c; f_800e9b2c(c);
                pc = 0x8026164c;
            }
            0x8026164c => {
                c.r[4] = c.lbz(c.r[30].wrapping_add(0x6));
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80261658; f_801da224(c);
                pc = 0x80261658;
            }
            0x80261658 => {
                c.r[30] = c.r[30].wrapping_add(0x78);
                pc = 0x8026165c;
            }
            0x8026165c => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[29] = c.r[29].wrapping_add(0xffffffff);
                if !c.crbit(2) { pc = 0x80261624; continue; }
                pc = 0x80261668;
            }
            0x80261668 => {
                c.r[29] = c.r[30] | c.r[30];
                pc = 0x802616c8; continue;
                pc = 0x80261670;
            }
            0x80261670 => {
                c.r[3] = c.lhz(c.r[29].wrapping_add(0x4));
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x8));
                c.r[5] = c.lbz(c.r[29].wrapping_add(0x7));
                c.lr = 0x80261680; f_801de190(c);
                pc = 0x80261680;
            }
            0x80261680 => {
                c.r[4] = c.lwz(c.r[29]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[4].wrapping_add(0x600), c.r[31]);
                c.r[4] = c.lwz(c.r[29].wrapping_add(0xc));
                c.stw(c.r[4], c.r[31]);
                c.lr = 0x80261698; f_801dac3c(c);
                pc = 0x80261698;
            }
            0x80261698 => {
                c.r[4] = c.r[29].wrapping_add(0x10);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x802616a4; f_800e9b2c(c);
                pc = 0x802616a4;
            }
            0x802616a4 => {
                c.r[4] = c.lbz(c.r[29].wrapping_add(0x6));
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802616b0; f_801da224(c);
                pc = 0x802616b0;
            }
            0x802616b0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802616b8; f_800e3d08(c);
                pc = 0x802616b8;
            }
            0x802616b8 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802616c4; f_801da4e8(c);
                pc = 0x802616c4;
            }
            0x802616c4 => {
                c.r[29] = c.r[29].wrapping_add(0x7c);
                pc = 0x802616c8;
            }
            0x802616c8 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[28] = c.r[28].wrapping_add(0xffffffff);
                if !c.crbit(2) { pc = 0x80261670; continue; }
                pc = 0x802616d4;
            }
            0x802616d4 => {
                c.lr = 0x802616d8; f_801c3108(c);
                pc = 0x802616d8;
            }
            0x802616d8 => {
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x44;
                c.lr = 0x802616e4; f_800054f4(c);
                pc = 0x802616e4;
            }
            0x802616e4 => {
                c.r[3] = 0x1;
                c.lr = 0x802616ec; f_801ef8f4(c);
                pc = 0x802616ec;
            }
            0x802616ec => {
                c.lr = 0x802616f0; f_801c3430(c);
                pc = 0x802616f0;
            }
            0x802616f0 => {
                c.lr = 0x802616f4; f_801f198c(c);
                pc = 0x802616f4;
            }
            0x802616f4 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802615f4: bad pc {:#x}", pc),
        }
    }
}

/// _fightGSfloorPokemonCB__FPvUsPv (0x80261708, 324 bytes)
#[inline(never)]
pub fn f_80261708(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80261708) { return; }
        if c.prof { crate::prof::enter(c, 0x80261708); f_80261708_b(c, 0x80261708); crate::prof::exit(c, 0x80261708); return; }
    }
    f_80261708_b(c, 0x80261708)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80261708_at(c: &mut Cpu, pc: u32) { f_80261708_b(c, pc) }
#[inline(always)]
fn f_80261708_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80261708;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80261708, pc); }
        match pc {
            0x80261708 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[5] = 0xee;
                c.lr = 0x80261730; f_8012640c(c);
                pc = 0x80261730;
            }
            0x80261730 => {
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if !c.crbit(2) { pc = 0x80261740; continue; }
                pc = 0x80261738;
            }
            0x80261738 => {
                c.r[3] = 0x1;
                pc = 0x80261838; continue;
                pc = 0x80261740;
            }
            0x80261740 => {
                c.lr = 0x80261744; f_801dac78(c);
                pc = 0x80261744;
            }
            0x80261744 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80261754; continue; }
                pc = 0x8026174c;
            }
            0x8026174c => {
                c.r[3] = 0x1;
                pc = 0x80261838; continue;
                pc = 0x80261754;
            }
            0x80261754 => {
                c.r[0] = c.lbz(c.r[31]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80261814; continue; }
                pc = 0x80261760;
            }
            0x80261760 => {
                c.r[30] = c.lwz(c.r[31].wrapping_add(0x8));
                c.stw(c.r[30], c.r[28]);
                c.sth(c.r[30].wrapping_add(0x4), c.r[3]);
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80261774; f_801da354(c);
                pc = 0x80261774;
            }
            0x80261774 => {
                c.stb(c.r[30].wrapping_add(0x6), c.r[3]);
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80261780; f_801d9e1c(c);
                pc = 0x80261780;
            }
            0x80261780 => {
                c.stb(c.r[30].wrapping_add(0x7), c.r[3]);
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8026178c; f_801de164(c);
                pc = 0x8026178c;
            }
            0x8026178c => {
                c.stw(c.r[30].wrapping_add(0x8), c.r[3]);
                c.cmpu(0, c.r[29], 0x0);
                c.r[9] = c.lwz(c.r[31].wrapping_add(0x4));
                if !c.crbit(2) { pc = 0x802617a4; continue; }
                pc = 0x8026179c;
            }
            0x8026179c => {
                c.r[0] = 0x0;
                pc = 0x802617f4; continue;
                pc = 0x802617a4;
            }
            0x802617a4 => {
                c.r[7] = 0x0;
                c.r[3] = 0x0;
                c.r[6] = c.r[7] | c.r[7];
                c.r[0] = 0x2;
                pc = 0x802617b4;
            }
            0x802617b4 => {
                c.r[4] = c.r[6] | c.r[6];
                c.r[8] = c.r[9].wrapping_add(c.r[3]);
                c.mtspr(9, c.r[0]);
                pc = 0x802617c0;
            }
            0x802617c0 => {
                c.r[10] = c.r[8].wrapping_add(c.r[4]);
                c.r[5] = c.lwz(c.r[10].wrapping_add(0x4));
                c.cmpu(0, c.r[5], c.r[29]);
                if !c.crbit(2) { pc = 0x802617d8; continue; }
                pc = 0x802617d0;
            }
            0x802617d0 => {
                c.r[0] = c.r[10].wrapping_add(0x4);
                pc = 0x802617f4; continue;
                pc = 0x802617d8;
            }
            0x802617d8 => {
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x802617c0; continue; }
                pc = 0x802617e0;
            }
            0x802617e0 => {
                c.r[7] = c.r[7].wrapping_add(0x1);
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.cmps(0, c.r[7], 0x4);
                if c.crbit(0) { pc = 0x802617b4; continue; }
                pc = 0x802617f0;
            }
            0x802617f0 => {
                c.r[0] = 0x0;
                pc = 0x802617f4;
            }
            0x802617f4 => {
                c.stw(c.r[30].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80261800; f_801dac3c(c);
                pc = 0x80261800;
            }
            0x80261800 => {
                c.r[4] = c.r[30].wrapping_add(0x10);
                c.lr = 0x80261808; f_800e9c6c(c);
                pc = 0x80261808;
            }
            0x80261808 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0x7c);
                c.stw(c.r[31].wrapping_add(0x8), c.r[0]);
                pc = 0x80261814;
            }
            0x80261814 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80261828; continue; }
                pc = 0x80261820;
            }
            0x80261820 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80261828; f_801db100(c);
                pc = 0x80261828;
            }
            0x80261828 => {
                c.r[4] = c.lhz(c.r[31].wrapping_add(0x2));
                c.r[3] = 0x1;
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.sth(c.r[31].wrapping_add(0x2), c.r[0]);
                pc = 0x80261838;
            }
            0x80261838 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80261708: bad pc {:#x}", pc),
        }
    }
}

/// _fightGSfloorTrainerCB__FPvUsPv (0x8026184c, 264 bytes)
#[inline(never)]
pub fn f_8026184c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8026184c) { return; }
        if c.prof { crate::prof::enter(c, 0x8026184c); f_8026184c_b(c, 0x8026184c); crate::prof::exit(c, 0x8026184c); return; }
    }
    f_8026184c_b(c, 0x8026184c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8026184c_at(c: &mut Cpu, pc: u32) { f_8026184c_b(c, pc) }
#[inline(always)]
fn f_8026184c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8026184c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8026184c, pc); }
        match pc {
            0x8026184c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[5] = 0x4c;
                c.lr = 0x80261874; f_801fb1c0(c);
                pc = 0x80261874;
            }
            0x80261874 => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if !c.crbit(2) { pc = 0x80261884; continue; }
                pc = 0x8026187c;
            }
            0x8026187c => {
                c.r[3] = 0x1;
                pc = 0x80261940; continue;
                pc = 0x80261884;
            }
            0x80261884 => {
                c.lr = 0x80261888; f_801dac78(c);
                pc = 0x80261888;
            }
            0x80261888 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80261898; continue; }
                pc = 0x80261890;
            }
            0x80261890 => {
                c.r[3] = 0x1;
                pc = 0x80261940; continue;
                pc = 0x80261898;
            }
            0x80261898 => {
                c.r[0] = c.lbz(c.r[29]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8026191c; continue; }
                pc = 0x802618a4;
            }
            0x802618a4 => {
                c.r[31] = c.lwz(c.r[29].wrapping_add(0x8));
                c.stw(c.r[31], c.r[28]);
                c.sth(c.r[31].wrapping_add(0x4), c.r[3]);
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802618b8; f_801da354(c);
                pc = 0x802618b8;
            }
            0x802618b8 => {
                c.stb(c.r[31].wrapping_add(0x6), c.r[3]);
                c.cmpu(0, c.r[30], 0x0);
                c.r[5] = c.lwz(c.r[29].wrapping_add(0x4));
                if !c.crbit(2) { pc = 0x802618d0; continue; }
                pc = 0x802618c8;
            }
            0x802618c8 => {
                c.r[4] = 0x0;
                pc = 0x802618fc; continue;
                pc = 0x802618d0;
            }
            0x802618d0 => {
                c.r[0] = 0x4;
                c.r[3] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x802618dc;
            }
            0x802618dc => {
                c.r[4] = c.r[5].wrapping_add(c.r[3]);
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], c.r[30]);
                if !c.crbit(2) { pc = 0x802618f0; continue; }
                pc = 0x802618ec;
            }
            0x802618ec => {
                pc = 0x802618fc; continue;
                pc = 0x802618f0;
            }
            0x802618f0 => {
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x802618dc; continue; }
                pc = 0x802618f8;
            }
            0x802618f8 => {
                c.r[4] = 0x0;
                pc = 0x802618fc;
            }
            0x802618fc => {
                c.stw(c.r[31].wrapping_add(0x8), c.r[4]);
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80261908; f_801dac3c(c);
                pc = 0x80261908;
            }
            0x80261908 => {
                c.r[4] = c.r[31].wrapping_add(0xc);
                c.lr = 0x80261910; f_800e9c6c(c);
                pc = 0x80261910;
            }
            0x80261910 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0x78);
                c.stw(c.r[29].wrapping_add(0x8), c.r[0]);
                pc = 0x8026191c;
            }
            0x8026191c => {
                c.r[0] = c.lbz(c.r[29].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80261930; continue; }
                pc = 0x80261928;
            }
            0x80261928 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80261930; f_801db100(c);
                pc = 0x80261930;
            }
            0x80261930 => {
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x2));
                c.r[3] = 0x1;
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.sth(c.r[29].wrapping_add(0x2), c.r[0]);
                pc = 0x80261940;
            }
            0x80261940 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8026184c: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuCloseInfoMenu (0x80261954, 380 bytes)
#[inline(never)]
pub fn f_80261954(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80261954) { return; }
        if c.prof { crate::prof::enter(c, 0x80261954); f_80261954_b(c, 0x80261954); crate::prof::exit(c, 0x80261954); return; }
    }
    f_80261954_b(c, 0x80261954)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80261954_at(c: &mut Cpu, pc: u32) { f_80261954_b(c, pc) }
#[inline(always)]
fn f_80261954_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80261954;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80261954, pc); }
        match pc {
            0x80261954 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80260000;
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x1cbc);
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x80261980; f_801f2b5c(c);
                pc = 0x80261980;
            }
            0x80261980 => {
                c.r[4] = 0x80260000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x1fb4);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80261998; f_801f37b0(c);
                pc = 0x80261998;
            }
            0x80261998 => {
                c.r[3] = 0x0;
                c.lr = 0x802619a0; f_801f1700(c);
                pc = 0x802619a0;
            }
            0x802619a0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802619b0; continue; }
                pc = 0x802619ac;
            }
            0x802619ac => {
                c.lr = 0x802619b0; f_8000ddbc(c);
                pc = 0x802619b0;
            }
            0x802619b0 => {
                c.r[3] = 0x0;
                c.lr = 0x802619b8; f_801f1758(c);
                pc = 0x802619b8;
            }
            0x802619b8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802619c8; continue; }
                pc = 0x802619c4;
            }
            0x802619c4 => {
                c.lr = 0x802619c8; f_8000dd30(c);
                pc = 0x802619c8;
            }
            0x802619c8 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80261abc; continue; }
                pc = 0x802619d4;
            }
            0x802619d4 => {
                c.r[4] = 0x80260000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x1cbc);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802619ec; f_801f2b5c(c);
                pc = 0x802619ec;
            }
            0x802619ec => {
                c.r[3] = 0x80260000;
                c.r[30] = 0x1;
                c.r[31] = c.r[3].wrapping_add(0x1bec);
                pc = 0x802619f8;
            }
            0x802619f8 => {
                c.stb(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80261a10; f_801f2b5c(c);
                pc = 0x80261a10;
            }
            0x80261a10 => {
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80261a24; continue; }
                pc = 0x80261a1c;
            }
            0x80261a1c => {
                c.lr = 0x80261a20; crate::stubs::f_800f0308(c);
                pc = 0x80261a20;
            }
            0x80261a20 => {
                pc = 0x802619f8; continue;
                pc = 0x80261a24;
            }
            0x80261a24 => {
                c.r[4] = 0x80260000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x1fb4);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80261a3c; f_801f37b0(c);
                pc = 0x80261a3c;
            }
            0x80261a3c => {
                c.r[3] = 0x80260000;
                c.r[31] = c.r[3].wrapping_add(0x1ef8);
                pc = 0x80261a44;
            }
            0x80261a44 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80261a58; f_801f37b0(c);
                pc = 0x80261a58;
            }
            0x80261a58 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80261a6c; continue; }
                pc = 0x80261a64;
            }
            0x80261a64 => {
                c.lr = 0x80261a68; crate::stubs::f_800f0308(c);
                pc = 0x80261a68;
            }
            0x80261a68 => {
                pc = 0x80261a44; continue;
                pc = 0x80261a6c;
            }
            0x80261a6c => {
                c.r[3] = 0x0;
                c.lr = 0x80261a74; f_801f1700(c);
                pc = 0x80261a74;
            }
            0x80261a74 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80261a94; continue; }
                pc = 0x80261a80;
            }
            0x80261a80 => {
                c.lr = 0x80261a84; f_8000dd98(c);
                pc = 0x80261a84;
            }
            0x80261a84 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80261a94; continue; }
                pc = 0x80261a8c;
            }
            0x80261a8c => {
                c.lr = 0x80261a90; crate::stubs::f_800f0308(c);
                pc = 0x80261a90;
            }
            0x80261a90 => {
                pc = 0x80261a80; continue;
                pc = 0x80261a94;
            }
            0x80261a94 => {
                c.r[3] = 0x0;
                c.lr = 0x80261a9c; f_801f1758(c);
                pc = 0x80261a9c;
            }
            0x80261a9c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80261abc; continue; }
                pc = 0x80261aa8;
            }
            0x80261aa8 => {
                c.lr = 0x80261aac; f_8000dd0c(c);
                pc = 0x80261aac;
            }
            0x80261aac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80261abc; continue; }
                pc = 0x80261ab4;
            }
            0x80261ab4 => {
                c.lr = 0x80261ab8; crate::stubs::f_800f0308(c);
                pc = 0x80261ab8;
            }
            0x80261ab8 => {
                pc = 0x80261aa8; continue;
                pc = 0x80261abc;
            }
            0x80261abc => {
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80261954: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuOpenInfoMenu (0x80261ad0, 152 bytes)
#[inline(never)]
pub fn f_80261ad0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80261ad0) { return; }
        if c.prof { crate::prof::enter(c, 0x80261ad0); f_80261ad0_b(c, 0x80261ad0); crate::prof::exit(c, 0x80261ad0); return; }
    }
    f_80261ad0_b(c, 0x80261ad0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80261ad0_at(c: &mut Cpu, pc: u32) { f_80261ad0_b(c, pc) }
#[inline(always)]
fn f_80261ad0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80261ad0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80261ad0, pc); }
        match pc {
            0x80261ad0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80260000;
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x1d8c);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x80261afc; f_801f2b5c(c);
                pc = 0x80261afc;
            }
            0x80261afc => {
                c.r[0] = 0x1;
                c.r[3] = 0x80260000;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x2084);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80261b1c; f_801f37b0(c);
                pc = 0x80261b1c;
            }
            0x80261b1c => {
                c.r[0] = c.r[31] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80261b3c; continue; }
                pc = 0x80261b24;
            }
            0x80261b24 => {
                c.r[3] = 0x0;
                c.lr = 0x80261b2c; f_801f1700(c);
                pc = 0x80261b2c;
            }
            0x80261b2c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80261b3c; continue; }
                pc = 0x80261b38;
            }
            0x80261b38 => {
                c.lr = 0x80261b3c; f_8000dde8(c);
                pc = 0x80261b3c;
            }
            0x80261b3c => {
                c.r[3] = 0x0;
                c.lr = 0x80261b44; f_801f1758(c);
                pc = 0x80261b44;
            }
            0x80261b44 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80261b54; continue; }
                pc = 0x80261b50;
            }
            0x80261b50 => {
                c.lr = 0x80261b54; f_8000dd5c(c);
                pc = 0x80261b54;
            }
            0x80261b54 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80261ad0: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuAllFightTrainerCloseStatusMenu (0x80261b68, 132 bytes)
#[inline(never)]
pub fn f_80261b68(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80261b68) { return; }
        if c.prof { crate::prof::enter(c, 0x80261b68); f_80261b68_b(c, 0x80261b68); crate::prof::exit(c, 0x80261b68); return; }
    }
    f_80261b68_b(c, 0x80261b68)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80261b68_at(c: &mut Cpu, pc: u32) { f_80261b68_b(c, pc) }
#[inline(always)]
fn f_80261b68_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80261b68;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80261b68, pc); }
        match pc {
            0x80261b68 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80260000;
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x1cbc);
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x80261b94; f_801f2b5c(c);
                pc = 0x80261b94;
            }
            0x80261b94 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80261bd8; continue; }
                pc = 0x80261ba0;
            }
            0x80261ba0 => {
                c.r[3] = 0x80260000;
                c.r[30] = 0x1;
                c.r[31] = c.r[3].wrapping_add(0x1bec);
                pc = 0x80261bac;
            }
            0x80261bac => {
                c.stb(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80261bc4; f_801f2b5c(c);
                pc = 0x80261bc4;
            }
            0x80261bc4 => {
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80261bd8; continue; }
                pc = 0x80261bd0;
            }
            0x80261bd0 => {
                c.lr = 0x80261bd4; crate::stubs::f_800f0308(c);
                pc = 0x80261bd4;
            }
            0x80261bd4 => {
                pc = 0x80261bac; continue;
                pc = 0x80261bd8;
            }
            0x80261bd8 => {
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80261b68: bad pc {:#x}", pc),
        }
    }
}

/// _fightMenuAllFightTrainerCloseStatusMenuSubCloseCheck__FPvUsPv (0x80261bec, 208 bytes)
#[inline(never)]
pub fn f_80261bec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80261bec) { return; }
        if c.prof { crate::prof::enter(c, 0x80261bec); f_80261bec_b(c, 0x80261bec); crate::prof::exit(c, 0x80261bec); return; }
    }
    f_80261bec_b(c, 0x80261bec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80261bec_at(c: &mut Cpu, pc: u32) { f_80261bec_b(c, pc) }
#[inline(always)]
fn f_80261bec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80261bec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80261bec, pc); }
        match pc {
            0x80261bec => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x2;
                c.r[5] = c.r[28] | c.r[28];
                c.lr = 0x80261c18; f_801f02ac(c);
                pc = 0x80261c18;
            }
            0x80261c18 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80261c28; continue; }
                pc = 0x80261c20;
            }
            0x80261c20 => {
                c.r[3] = 0x0;
                pc = 0x80261c84; continue;
                pc = 0x80261c28;
            }
            0x80261c28 => {
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80261c38; f_801f76b8(c);
                pc = 0x80261c38;
            }
            0x80261c38 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80261c48; f_801f0134(c);
                pc = 0x80261c48;
            }
            0x80261c48 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80261c58; continue; }
                pc = 0x80261c50;
            }
            0x80261c50 => {
                c.r[3] = 0x0;
                pc = 0x80261c84; continue;
                pc = 0x80261c58;
            }
            0x80261c58 => {
                c.lr = 0x80261c5c; f_801f0234(c);
                pc = 0x80261c5c;
            }
            0x80261c5c => {
                c.lr = 0x80261c60; f_801f0204(c);
                pc = 0x80261c60;
            }
            0x80261c60 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x80261c70; continue; }
                pc = 0x80261c68;
            }
            0x80261c68 => {
                c.r[3] = 0x0;
                pc = 0x80261c84; continue;
                pc = 0x80261c70;
            }
            0x80261c70 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80261c84; f_801f76b8(c);
                pc = 0x80261c84;
            }
            0x80261c84 => {
                c.lr = 0x80261c88; f_80102620(c);
                pc = 0x80261c88;
            }
            0x80261c88 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80261ca4; continue; }
                pc = 0x80261c94;
            }
            0x80261c94 => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x80261ca4; continue; }
                pc = 0x80261c9c;
            }
            0x80261c9c => {
                c.r[0] = 0x0;
                c.stb(c.r[29], c.r[0]);
                pc = 0x80261ca4;
            }
            0x80261ca4 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80261bec: bad pc {:#x}", pc),
        }
    }
}

/// _fightMenuAllFightTrainerCloseStatusMenuSub__FPvUsPv (0x80261cbc, 208 bytes)
#[inline(never)]
pub fn f_80261cbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80261cbc) { return; }
        if c.prof { crate::prof::enter(c, 0x80261cbc); f_80261cbc_b(c, 0x80261cbc); crate::prof::exit(c, 0x80261cbc); return; }
    }
    f_80261cbc_b(c, 0x80261cbc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80261cbc_at(c: &mut Cpu, pc: u32) { f_80261cbc_b(c, pc) }
#[inline(always)]
fn f_80261cbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80261cbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80261cbc, pc); }
        match pc {
            0x80261cbc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x80261ce4; f_801f02ac(c);
                pc = 0x80261ce4;
            }
            0x80261ce4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80261cf4; continue; }
                pc = 0x80261cec;
            }
            0x80261cec => {
                c.r[31] = 0x0;
                pc = 0x80261d54; continue;
                pc = 0x80261cf4;
            }
            0x80261cf4 => {
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80261d04; f_801f76b8(c);
                pc = 0x80261d04;
            }
            0x80261d04 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80261d14; f_801f0134(c);
                pc = 0x80261d14;
            }
            0x80261d14 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80261d24; continue; }
                pc = 0x80261d1c;
            }
            0x80261d1c => {
                c.r[31] = 0x0;
                pc = 0x80261d54; continue;
                pc = 0x80261d24;
            }
            0x80261d24 => {
                c.lr = 0x80261d28; f_801f0234(c);
                pc = 0x80261d28;
            }
            0x80261d28 => {
                c.lr = 0x80261d2c; f_801f0204(c);
                pc = 0x80261d2c;
            }
            0x80261d2c => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x80261d3c; continue; }
                pc = 0x80261d34;
            }
            0x80261d34 => {
                c.r[31] = 0x0;
                pc = 0x80261d54; continue;
                pc = 0x80261d3c;
            }
            0x80261d3c => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80261d50; f_801f76b8(c);
                pc = 0x80261d50;
            }
            0x80261d50 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80261d54;
            }
            0x80261d54 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80261d5c; f_80102620(c);
                pc = 0x80261d5c;
            }
            0x80261d5c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80261d74; continue; }
                pc = 0x80261d64;
            }
            0x80261d64 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x80261d74; f_80102568(c);
                pc = 0x80261d74;
            }
            0x80261d74 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80261cbc: bad pc {:#x}", pc),
        }
    }
}

/// _fightMenuAllFightTrainerOpenStatusMenuSub__FPvUsPv (0x80261d8c, 240 bytes)
#[inline(never)]
pub fn f_80261d8c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80261d8c) { return; }
        if c.prof { crate::prof::enter(c, 0x80261d8c); f_80261d8c_b(c, 0x80261d8c); crate::prof::exit(c, 0x80261d8c); return; }
    }
    f_80261d8c_b(c, 0x80261d8c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80261d8c_at(c: &mut Cpu, pc: u32) { f_80261d8c_b(c, pc) }
#[inline(always)]
fn f_80261d8c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80261d8c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80261d8c, pc); }
        match pc {
            0x80261d8c => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x80261db4; f_801f02ac(c);
                pc = 0x80261db4;
            }
            0x80261db4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80261dc4; continue; }
                pc = 0x80261dbc;
            }
            0x80261dbc => {
                c.r[31] = 0x0;
                pc = 0x80261e24; continue;
                pc = 0x80261dc4;
            }
            0x80261dc4 => {
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80261dd4; f_801f76b8(c);
                pc = 0x80261dd4;
            }
            0x80261dd4 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80261de4; f_801f0134(c);
                pc = 0x80261de4;
            }
            0x80261de4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80261df4; continue; }
                pc = 0x80261dec;
            }
            0x80261dec => {
                c.r[31] = 0x0;
                pc = 0x80261e24; continue;
                pc = 0x80261df4;
            }
            0x80261df4 => {
                c.lr = 0x80261df8; f_801f0234(c);
                pc = 0x80261df8;
            }
            0x80261df8 => {
                c.lr = 0x80261dfc; f_801f0204(c);
                pc = 0x80261dfc;
            }
            0x80261dfc => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x80261e0c; continue; }
                pc = 0x80261e04;
            }
            0x80261e04 => {
                c.r[31] = 0x0;
                pc = 0x80261e24; continue;
                pc = 0x80261e0c;
            }
            0x80261e0c => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80261e20; f_801f76b8(c);
                pc = 0x80261e20;
            }
            0x80261e20 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80261e24;
            }
            0x80261e24 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80261e30; f_801f7954(c);
                pc = 0x80261e30;
            }
            0x80261e30 => {
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lhz(c.r[1].wrapping_add(0xc));
                c.r[9] = c.r[1].wrapping_add(0x10);
                c.stw(c.r[1].wrapping_add(0x10), c.r[4]);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.sth(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = 0x0;
                c.r[8] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80261e64; f_801026a4(c);
                pc = 0x80261e64;
            }
            0x80261e64 => {
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80261d8c: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuAllFightOutPokemonCloseStatusMenu (0x80261e7c, 124 bytes)
#[inline(never)]
pub fn f_80261e7c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80261e7c) { return; }
        if c.prof { crate::prof::enter(c, 0x80261e7c); f_80261e7c_b(c, 0x80261e7c); crate::prof::exit(c, 0x80261e7c); return; }
    }
    f_80261e7c_b(c, 0x80261e7c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80261e7c_at(c: &mut Cpu, pc: u32) { f_80261e7c_b(c, pc) }
#[inline(always)]
fn f_80261e7c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80261e7c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80261e7c, pc); }
        match pc {
            0x80261e7c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80260000;
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x1fb4);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x80261ea8; f_801f37b0(c);
                pc = 0x80261ea8;
            }
            0x80261ea8 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80261ee4; continue; }
                pc = 0x80261eb4;
            }
            0x80261eb4 => {
                c.r[3] = 0x80260000;
                c.r[31] = c.r[3].wrapping_add(0x1ef8);
                pc = 0x80261ebc;
            }
            0x80261ebc => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80261ed0; f_801f37b0(c);
                pc = 0x80261ed0;
            }
            0x80261ed0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80261ee4; continue; }
                pc = 0x80261edc;
            }
            0x80261edc => {
                c.lr = 0x80261ee0; crate::stubs::f_800f0308(c);
                pc = 0x80261ee0;
            }
            0x80261ee0 => {
                pc = 0x80261ebc; continue;
                pc = 0x80261ee4;
            }
            0x80261ee4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80261e7c: bad pc {:#x}", pc),
        }
    }
}

/// _fightMenuAllFightOutPokemonCloseStatusMenuSubCloseCheck__FPvUsPv (0x80261ef8, 188 bytes)
#[inline(never)]
pub fn f_80261ef8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80261ef8) { return; }
        if c.prof { crate::prof::enter(c, 0x80261ef8); f_80261ef8_b(c, 0x80261ef8); crate::prof::exit(c, 0x80261ef8); return; }
    }
    f_80261ef8_b(c, 0x80261ef8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80261ef8_at(c: &mut Cpu, pc: u32) { f_80261ef8_b(c, pc) }
#[inline(always)]
fn f_80261ef8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80261ef8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80261ef8, pc); }
        match pc {
            0x80261ef8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x80261f20; f_801f02ac(c);
                pc = 0x80261f20;
            }
            0x80261f20 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80261f30; continue; }
                pc = 0x80261f28;
            }
            0x80261f28 => {
                c.r[3] = 0x0;
                pc = 0x80261f8c; continue;
                pc = 0x80261f30;
            }
            0x80261f30 => {
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80261f40; f_801f76b8(c);
                pc = 0x80261f40;
            }
            0x80261f40 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80261f50; f_801f0134(c);
                pc = 0x80261f50;
            }
            0x80261f50 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80261f60; continue; }
                pc = 0x80261f58;
            }
            0x80261f58 => {
                c.r[3] = 0x0;
                pc = 0x80261f8c; continue;
                pc = 0x80261f60;
            }
            0x80261f60 => {
                c.lr = 0x80261f64; f_801f0234(c);
                pc = 0x80261f64;
            }
            0x80261f64 => {
                c.lr = 0x80261f68; f_801f0204(c);
                pc = 0x80261f68;
            }
            0x80261f68 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x80261f78; continue; }
                pc = 0x80261f70;
            }
            0x80261f70 => {
                c.r[3] = 0x0;
                pc = 0x80261f8c; continue;
                pc = 0x80261f78;
            }
            0x80261f78 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x80261f8c; f_801f76b8(c);
                pc = 0x80261f8c;
            }
            0x80261f8c => {
                c.lr = 0x80261f90; f_80102620(c);
                pc = 0x80261f90;
            }
            0x80261f90 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80261ef8: bad pc {:#x}", pc),
        }
    }
}

/// _fightMenuAllFightOutPokemonCloseStatusMenuSub__FPvUsPv (0x80261fb4, 208 bytes)
#[inline(never)]
pub fn f_80261fb4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80261fb4) { return; }
        if c.prof { crate::prof::enter(c, 0x80261fb4); f_80261fb4_b(c, 0x80261fb4); crate::prof::exit(c, 0x80261fb4); return; }
    }
    f_80261fb4_b(c, 0x80261fb4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80261fb4_at(c: &mut Cpu, pc: u32) { f_80261fb4_b(c, pc) }
#[inline(always)]
fn f_80261fb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80261fb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80261fb4, pc); }
        match pc {
            0x80261fb4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x80261fdc; f_801f02ac(c);
                pc = 0x80261fdc;
            }
            0x80261fdc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80261fec; continue; }
                pc = 0x80261fe4;
            }
            0x80261fe4 => {
                c.r[31] = 0x0;
                pc = 0x8026204c; continue;
                pc = 0x80261fec;
            }
            0x80261fec => {
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80261ffc; f_801f76b8(c);
                pc = 0x80261ffc;
            }
            0x80261ffc => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x8026200c; f_801f0134(c);
                pc = 0x8026200c;
            }
            0x8026200c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8026201c; continue; }
                pc = 0x80262014;
            }
            0x80262014 => {
                c.r[31] = 0x0;
                pc = 0x8026204c; continue;
                pc = 0x8026201c;
            }
            0x8026201c => {
                c.lr = 0x80262020; f_801f0234(c);
                pc = 0x80262020;
            }
            0x80262020 => {
                c.lr = 0x80262024; f_801f0204(c);
                pc = 0x80262024;
            }
            0x80262024 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x80262034; continue; }
                pc = 0x8026202c;
            }
            0x8026202c => {
                c.r[31] = 0x0;
                pc = 0x8026204c; continue;
                pc = 0x80262034;
            }
            0x80262034 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x80262048; f_801f76b8(c);
                pc = 0x80262048;
            }
            0x80262048 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8026204c;
            }
            0x8026204c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80262054; f_80102620(c);
                pc = 0x80262054;
            }
            0x80262054 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8026206c; continue; }
                pc = 0x8026205c;
            }
            0x8026205c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x8026206c; f_80102568(c);
                pc = 0x8026206c;
            }
            0x8026206c => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80261fb4: bad pc {:#x}", pc),
        }
    }
}

/// _fightMenuAllFightOutPokemonOpenStatusMenuSub__FPvUsPv (0x80262084, 320 bytes)
#[inline(never)]
pub fn f_80262084(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80262084) { return; }
        if c.prof { crate::prof::enter(c, 0x80262084); f_80262084_b(c, 0x80262084); crate::prof::exit(c, 0x80262084); return; }
    }
    f_80262084_b(c, 0x80262084)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80262084_at(c: &mut Cpu, pc: u32) { f_80262084_b(c, pc) }
#[inline(always)]
fn f_80262084_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80262084;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80262084, pc); }
        match pc {
            0x80262084 => {
                { let ea = c.r[1].wrapping_add(0xffffff80); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[5], 0x0);
                c.stw(c.r[1].wrapping_add(0x84), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x70); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x802620ac; continue; }
                pc = 0x802620a4;
            }
            0x802620a4 => {
                c.r[29] = 0x1;
                pc = 0x802620b0; continue;
                pc = 0x802620ac;
            }
            0x802620ac => {
                c.r[29] = c.lbz(c.r[5]);
                pc = 0x802620b0;
            }
            0x802620b0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802620b8; f_802062fc(c);
                pc = 0x802620b8;
            }
            0x802620b8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802620c8; continue; }
                pc = 0x802620c0;
            }
            0x802620c0 => {
                c.r[3] = 0x1;
                pc = 0x802621b0; continue;
                pc = 0x802620c8;
            }
            0x802620c8 => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[3] = 0x2;
                c.lr = 0x802620d8; f_801f02ac(c);
                pc = 0x802620d8;
            }
            0x802620d8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x802620e8; continue; }
                pc = 0x802620e0;
            }
            0x802620e0 => {
                c.r[31] = 0x0;
                pc = 0x80262148; continue;
                pc = 0x802620e8;
            }
            0x802620e8 => {
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x802620f8; f_801f76b8(c);
                pc = 0x802620f8;
            }
            0x802620f8 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80262108; f_801f0134(c);
                pc = 0x80262108;
            }
            0x80262108 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80262118; continue; }
                pc = 0x80262110;
            }
            0x80262110 => {
                c.r[31] = 0x0;
                pc = 0x80262148; continue;
                pc = 0x80262118;
            }
            0x80262118 => {
                c.lr = 0x8026211c; f_801f0234(c);
                pc = 0x8026211c;
            }
            0x8026211c => {
                c.lr = 0x80262120; f_801f0204(c);
                pc = 0x80262120;
            }
            0x80262120 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x80262130; continue; }
                pc = 0x80262128;
            }
            0x80262128 => {
                c.r[31] = 0x0;
                pc = 0x80262148; continue;
                pc = 0x80262130;
            }
            0x80262130 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x80262144; f_801f76b8(c);
                pc = 0x80262144;
            }
            0x80262144 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80262148;
            }
            0x80262148 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80262154; f_801fe168(c);
                pc = 0x80262154;
            }
            0x80262154 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80262164; continue; }
                pc = 0x8026215c;
            }
            0x8026215c => {
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0x31), c.r[0]);
                pc = 0x80262164;
            }
            0x80262164 => {
                c.r[0] = 0x6;
                c.r[5] = c.r[1].wrapping_add(0x34);
                c.r[4] = c.r[1].wrapping_add(0x4);
                c.mtspr(9, c.r[0]);
                pc = 0x80262174;
            }
            0x80262174 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80262174; continue; }
                pc = 0x80262188;
            }
            0x80262188 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[9] = c.r[1].wrapping_add(0x38);
                c.r[4] = 0xffffffff;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x802621ac; f_801026a4(c);
                pc = 0x802621ac;
            }
            0x802621ac => {
                c.r[3] = 0x1;
                pc = 0x802621b0;
            }
            0x802621b0 => {
                { let mut ea = c.r[1].wrapping_add(0x70); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x84));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x80);
                if true { return; }
                return;
            }
            _ => panic!("f_80262084: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuOpenLevelUpStatusMenu (0x802621c4, 48 bytes)
#[inline(never)]
pub fn f_802621c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802621c4) { return; }
        if c.prof { crate::prof::enter(c, 0x802621c4); f_802621c4_b(c, 0x802621c4); crate::prof::exit(c, 0x802621c4); return; }
    }
    f_802621c4_b(c, 0x802621c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802621c4_at(c: &mut Cpu, pc: u32) { f_802621c4_b(c, pc) }
#[inline(always)]
fn f_802621c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802621c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802621c4, pc); }
        match pc {
            0x802621c4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if c.crbit(2) { pc = 0x802621e4; continue; }
                pc = 0x802621d8;
            }
            0x802621d8 => {
                c.stb(c.r[3], c.r[4]);
                c.r[4] = 0x1;
                c.lr = 0x802621e4; f_80105e7c(c);
                pc = 0x802621e4;
            }
            0x802621e4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802621c4: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuSubMenuLvupStatus (0x802621f4, 124 bytes)
#[inline(never)]
pub fn f_802621f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802621f4) { return; }
        if c.prof { crate::prof::enter(c, 0x802621f4); f_802621f4_b(c, 0x802621f4); crate::prof::exit(c, 0x802621f4); return; }
    }
    f_802621f4_b(c, 0x802621f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802621f4_at(c: &mut Cpu, pc: u32) { f_802621f4_b(c, pc) }
#[inline(always)]
fn f_802621f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802621f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802621f4, pc); }
        match pc {
            0x802621f4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x802621fc;
            }
            0x802621fc => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80262204;
            }
            0x80262204 => {
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8026220c;
            }
            0x8026220c => {
                c.r[6] = c.lha(c.r[4].wrapping_add(0x2));
                c.r[0] = c.lha(c.r[3].wrapping_add(0x2));
                c.r[0] = c.r[0].wrapping_sub(c.r[6]);
                c.sth(c.r[5].wrapping_add(0x2), c.r[0]);
                c.r[6] = c.lha(c.r[4].wrapping_add(0x4));
                c.r[0] = c.lha(c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[0].wrapping_sub(c.r[6]);
                c.sth(c.r[5].wrapping_add(0x4), c.r[0]);
                c.r[6] = c.lha(c.r[4].wrapping_add(0x6));
                c.r[0] = c.lha(c.r[3].wrapping_add(0x6));
                c.r[0] = c.r[0].wrapping_sub(c.r[6]);
                c.sth(c.r[5].wrapping_add(0x6), c.r[0]);
                c.r[6] = c.lha(c.r[4].wrapping_add(0xa));
                c.r[0] = c.lha(c.r[3].wrapping_add(0xa));
                c.r[0] = c.r[0].wrapping_sub(c.r[6]);
                c.sth(c.r[5].wrapping_add(0xa), c.r[0]);
                c.r[6] = c.lha(c.r[4].wrapping_add(0xc));
                c.r[0] = c.lha(c.r[3].wrapping_add(0xc));
                c.r[0] = c.r[0].wrapping_sub(c.r[6]);
                c.sth(c.r[5].wrapping_add(0xc), c.r[0]);
                c.r[4] = c.lha(c.r[4].wrapping_add(0x8));
                c.r[0] = c.lha(c.r[3].wrapping_add(0x8));
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.sth(c.r[5].wrapping_add(0x8), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_802621f4: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuWazaWasure (0x80262270, 116 bytes)
#[inline(never)]
pub fn f_80262270(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80262270) { return; }
        if c.prof { crate::prof::enter(c, 0x80262270); f_80262270_b(c, 0x80262270); crate::prof::exit(c, 0x80262270); return; }
    }
    f_80262270_b(c, 0x80262270)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80262270_at(c: &mut Cpu, pc: u32) { f_80262270_b(c, pc) }
#[inline(always)]
fn f_80262270_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80262270;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80262270, pc); }
        match pc {
            0x80262270 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffffb028); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = 0x3;
                c.lr = 0x80262294; f_801c41c8(c);
                pc = 0x80262294;
            }
            0x80262294 => {
                c.r[3] = 0x1;
                c.lr = 0x8026229c; f_801c40f0(c);
                pc = 0x8026229c;
            }
            0x8026229c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x802622a8; f_80097a38(c);
                pc = 0x802622a8;
            }
            0x802622a8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmps(0, c.r[31], 0x4);
                if !c.crbit(2) { pc = 0x802622b8; continue; }
                pc = 0x802622b4;
            }
            0x802622b4 => {
                c.r[31] = 0xffffffff;
                pc = 0x802622b8;
            }
            0x802622b8 => {
                { let ea = c.r[2].wrapping_add(0xffffb028); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x802622c4; f_801c41c8(c);
                pc = 0x802622c4;
            }
            0x802622c4 => {
                c.r[3] = 0x1;
                c.lr = 0x802622cc; f_801c40f0(c);
                pc = 0x802622cc;
            }
            0x802622cc => {
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80262270: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuCloseLevelUpStatusMenu (0x802622e4, 36 bytes)
#[inline(never)]
pub fn f_802622e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802622e4) { return; }
        if c.prof { crate::prof::enter(c, 0x802622e4); f_802622e4_b(c, 0x802622e4); crate::prof::exit(c, 0x802622e4); return; }
    }
    f_802622e4_b(c, 0x802622e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802622e4_at(c: &mut Cpu, pc: u32) { f_802622e4_b(c, pc) }
#[inline(always)]
fn f_802622e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802622e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802622e4, pc); }
        match pc {
            0x802622e4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x802622f8; f_80105c68(c);
                pc = 0x802622f8;
            }
            0x802622f8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802622e4: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuYesNo (0x80262308, 44 bytes)
#[inline(never)]
pub fn f_80262308(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80262308) { return; }
        if c.prof { crate::prof::enter(c, 0x80262308); f_80262308_b(c, 0x80262308); crate::prof::exit(c, 0x80262308); return; }
    }
    f_80262308_b(c, 0x80262308)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80262308_at(c: &mut Cpu, pc: u32) { f_80262308_b(c, pc) }
#[inline(always)]
fn f_80262308_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80262308;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80262308, pc); }
        match pc {
            0x80262308 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80262318; f_8001e184(c);
                pc = 0x80262318;
            }
            0x80262318 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0xff;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80262308: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuWazaKoukaMsg (0x80262334, 128 bytes)
#[inline(never)]
pub fn f_80262334(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80262334) { return; }
        if c.prof { crate::prof::enter(c, 0x80262334); f_80262334_b(c, 0x80262334); crate::prof::exit(c, 0x80262334); return; }
    }
    f_80262334_b(c, 0x80262334)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80262334_at(c: &mut Cpu, pc: u32) { f_80262334_b(c, pc) }
#[inline(always)]
fn f_80262334_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80262334;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80262334, pc); }
        match pc {
            0x80262334 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                c.r[3] = 0x10;
                c.lr = 0x80262354; f_80132a38(c);
                pc = 0x80262354;
            }
            0x80262354 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80262368; f_80142cf4(c);
                pc = 0x80262368;
            }
            0x80262368 => {
                c.lr = 0x8026236c; f_800fa280(c);
                pc = 0x8026236c;
            }
            0x8026236c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x29;
                c.r[4] = c.r[0] | c.r[0];
                c.lr = 0x8026237c; f_80132a38(c);
                pc = 0x8026237c;
            }
            0x8026237c => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8026239c; continue; }
                pc = 0x80262384;
            }
            0x80262384 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x80262394; f_80106394(c);
                pc = 0x80262394;
            }
            0x80262394 => {
                c.r[3] = 0x1;
                pc = 0x802623a0; continue;
                pc = 0x8026239c;
            }
            0x8026239c => {
                c.r[3] = 0x0;
                pc = 0x802623a0;
            }
            0x802623a0 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80262334: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuWazaOutMsg (0x802623b4, 184 bytes)
#[inline(never)]
pub fn f_802623b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802623b4) { return; }
        if c.prof { crate::prof::enter(c, 0x802623b4); f_802623b4_b(c, 0x802623b4); crate::prof::exit(c, 0x802623b4); return; }
    }
    f_802623b4_b(c, 0x802623b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802623b4_at(c: &mut Cpu, pc: u32) { f_802623b4_b(c, pc) }
#[inline(always)]
fn f_802623b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802623b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802623b4, pc); }
        match pc {
            0x802623b4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xf;
                c.lr = 0x802623d4; f_80132a38(c);
                pc = 0x802623d4;
            }
            0x802623d4 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0xa;
                c.r[6] = 0x0;
                c.lr = 0x802623e8; f_8011beb4(c);
                pc = 0x802623e8;
            }
            0x802623e8 => {
                c.lr = 0x802623ec; f_800fa280(c);
                pc = 0x802623ec;
            }
            0x802623ec => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.r[4] = c.r[0] | c.r[0];
                c.lr = 0x802623fc; f_80132a38(c);
                pc = 0x802623fc;
            }
            0x802623fc => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80262410; f_8011beb4(c);
                pc = 0x80262410;
            }
            0x80262410 => {
                c.lr = 0x80262414; f_800fa280(c);
                pc = 0x80262414;
            }
            0x80262414 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x28;
                c.lr = 0x80262420; f_80132a38(c);
                pc = 0x80262420;
            }
            0x80262420 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0xb;
                c.r[6] = 0x0;
                c.lr = 0x80262434; f_8011beb4(c);
                pc = 0x80262434;
            }
            0x80262434 => {
                c.lr = 0x80262438; f_800fa280(c);
                pc = 0x80262438;
            }
            0x80262438 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xe;
                c.lr = 0x80262444; f_80132a38(c);
                pc = 0x80262444;
            }
            0x80262444 => {
                c.r[3] = 0x768d;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x80262454; f_80106394(c);
                pc = 0x80262454;
            }
            0x80262454 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802623b4: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuCloseMsg (0x8026246c, 36 bytes)
#[inline(never)]
pub fn f_8026246c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8026246c) { return; }
        if c.prof { crate::prof::enter(c, 0x8026246c); f_8026246c_b(c, 0x8026246c); crate::prof::exit(c, 0x8026246c); return; }
    }
    f_8026246c_b(c, 0x8026246c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8026246c_at(c: &mut Cpu, pc: u32) { f_8026246c_b(c, pc) }
#[inline(always)]
fn f_8026246c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8026246c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8026246c, pc); }
        match pc {
            0x8026246c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80262480; f_80106080(c);
                pc = 0x80262480;
            }
            0x80262480 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8026246c: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuOpenTrainerMsg (0x80262490, 60 bytes)
#[inline(never)]
pub fn f_80262490(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80262490) { return; }
        if c.prof { crate::prof::enter(c, 0x80262490); f_80262490_b(c, 0x80262490); crate::prof::exit(c, 0x80262490); return; }
    }
    f_80262490_b(c, 0x80262490)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80262490_at(c: &mut Cpu, pc: u32) { f_80262490_b(c, pc) }
#[inline(always)]
fn f_80262490_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80262490;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80262490, pc); }
        match pc {
            0x80262490 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if c.crbit(2) { pc = 0x802624b8; continue; }
                pc = 0x802624a4;
            }
            0x802624a4 => {
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x802624b0; f_80106244(c);
                pc = 0x802624b0;
            }
            0x802624b0 => {
                c.r[3] = 0x1;
                pc = 0x802624bc; continue;
                pc = 0x802624b8;
            }
            0x802624b8 => {
                c.r[3] = 0x0;
                pc = 0x802624bc;
            }
            0x802624bc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80262490: bad pc {:#x}", pc),
        }
    }
}

