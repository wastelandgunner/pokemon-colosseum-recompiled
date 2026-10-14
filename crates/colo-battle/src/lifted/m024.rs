#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// fn_8015F270 (0x8015f270, 944 bytes)
#[inline(never)]
pub fn f_8015f270(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8015f270) { return; }
        if c.prof { crate::prof::enter(c, 0x8015f270); f_8015f270_b(c, 0x8015f270); crate::prof::exit(c, 0x8015f270); return; }
    }
    f_8015f270_b(c, 0x8015f270)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015f270_at(c: &mut Cpu, pc: u32) { f_8015f270_b(c, pc) }
#[inline(always)]
fn f_8015f270_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8015f270;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8015f270, pc); }
        match pc {
            0x8015f270 => {
                { let ea = c.r[1].wrapping_add(0xffffff80); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x84), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x70); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = (c.f[29], c.ps1[29]); c.psq_store(ea, x, y, false, 0); }
                c.r[11] = c.r[1].wrapping_add(0x50);
                c.lr = 0x8015f29c; f_800c47c4(c);
                pc = 0x8015f29c;
            }
            0x8015f29c => {
                c.r[3] = 0x80450000;
                c.r[26] = 0x0;
                c.r[27] = c.r[3].wrapping_add(0xffff8590);
                pc = 0x8015f5e4; continue;
                pc = 0x8015f2ac;
            }
            0x8015f2ac => {
                c.r[25] = c.lwz(c.r[27].wrapping_add(0x4));
                c.r[30] = c.r[27].wrapping_add(0x8);
                c.r[29] = c.r[27].wrapping_add(0x4);
                c.r[28] = c.r[27].wrapping_add(0xc);
                pc = 0x8015f5d4; continue;
                pc = 0x8015f2c0;
            }
            0x8015f2c0 => {
                c.r[5] = c.lwz(c.r[30]);
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x8015f354; continue; }
                pc = 0x8015f2cc;
            }
            0x8015f2cc => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa813));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8015f2fc; continue; }
                pc = 0x8015f2d8;
            }
            0x8015f2d8 => {
                c.r[0] = c.lwz(c.r[27]);
                c.r[0] = c.r[0].rotate_left(0) & 0x80000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8015f2fc; continue; }
                pc = 0x8015f2e4;
            }
            0x8015f2e4 => {
                c.r[3] = c.lwz(c.r[29]);
                c.r[4] = c.lhz(c.r[28]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x18));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x47));
                c.cmps(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x8015f354; continue; }
                pc = 0x8015f2fc;
            }
            0x8015f2fc => {
                { let ea = c.r[25].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[5].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9e08); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] - c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.fcmp(0, c.f[1], c.f[0]);
                { let (a, b) = (c.crbit(0), c.crbit(2)); c.set_crbit(2, a | b); }
                if c.crbit(2) { pc = 0x8015f5d0; continue; }
                pc = 0x8015f318;
            }
            0x8015f318 => {
                { let ea = c.r[2].wrapping_add(0xffff9e0c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                { let (a, b) = (c.crbit(0), c.crbit(2)); c.set_crbit(2, a | b); }
                if !c.crbit(2) { pc = 0x8015f348; continue; }
                pc = 0x8015f328;
            }
            0x8015f328 => {
                c.r[4] = c.lwz(c.r[25].wrapping_add(0x18));
                c.r[3] = c.lhz(c.r[4].wrapping_add(0x48));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.sth(c.r[4].wrapping_add(0x48), c.r[3]);
                c.cmpu(0, c.r[0], 0x14);
                if c.crbit(0) { pc = 0x8015f5d0; continue; }
                pc = 0x8015f344;
            }
            0x8015f344 => {
                pc = 0x8015f354; continue;
                pc = 0x8015f348;
            }
            0x8015f348 => {
                c.r[3] = c.lwz(c.r[25].wrapping_add(0x18));
                c.r[0] = 0x0;
                c.sth(c.r[3].wrapping_add(0x48), c.r[0]);
                pc = 0x8015f354;
            }
            0x8015f354 => {
                c.r[24] = c.lwz(c.r[25].wrapping_add(0x18));
                c.r[6] = c.lwz(c.r[24].wrapping_add(0x8));
                c.cmpu(0, c.r[6], 0x0);
                if c.crbit(2) { pc = 0x8015f370; continue; }
                pc = 0x8015f364;
            }
            0x8015f364 => {
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x8015f3ac; continue; }
                pc = 0x8015f370;
            }
            0x8015f370 => {
                c.cmpu(0, c.r[6], 0x0);
                c.r[3] = c.lhz(c.r[24].wrapping_add(0x44));
                c.r[4] = 0x7f;
                c.r[5] = 0x40;
                if c.crbit(2) { pc = 0x8015f38c; continue; }
                pc = 0x8015f384;
            }
            0x8015f384 => {
                c.r[6] = c.lbz(c.r[6].wrapping_add(0x1c));
                pc = 0x8015f390; continue;
                pc = 0x8015f38c;
            }
            0x8015f38c => {
                c.r[6] = c.lbz(c.r[24].wrapping_add(0x46));
                pc = 0x8015f390;
            }
            0x8015f390 => {
                c.r[0] = c.lwz(c.r[24].wrapping_add(0x10));
                c.r[7] = c.r[0].rotate_left(28) & 0x1;
                c.lr = 0x8015f39c; f_8014c5e8(c);
                pc = 0x8015f39c;
            }
            0x8015f39c => {
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[24].wrapping_add(0x3c), c.r[3]);
                c.cmpu(0, c.r[0], 0xffff);
                if !c.crbit(2) { pc = 0x8015f3d0; continue; }
                pc = 0x8015f3ac;
            }
            0x8015f3ac => {
                c.r[3] = c.lwz(c.r[24].wrapping_add(0x10));
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8015f5d0; continue; }
                pc = 0x8015f3b8;
            }
            0x8015f3b8 => {
                c.r[0] = c.r[3] | 0x40000;
                c.stw(c.r[24].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.lwz(c.r[24].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffdffff;
                c.stw(c.r[24].wrapping_add(0x10), c.r[0]);
                pc = 0x8015f5d0; continue;
                pc = 0x8015f3d0;
            }
            0x8015f3d0 => {
                c.r[3] = c.lwz(c.r[24].wrapping_add(0x10));
                c.r[0] = c.r[3].rotate_left(0) & 0x20;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8015f3f0; continue; }
                pc = 0x8015f3dc;
            }
            0x8015f3dc => {
                c.r[0] = c.r[3] | 0x100000;
                { let ea = c.r[2].wrapping_add(0xffff9dc8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[24].wrapping_add(0x10), c.r[0]);
                { let ea = c.r[24].wrapping_add(0x4c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8015f3f8; continue;
                pc = 0x8015f3f0;
            }
            0x8015f3f0 => {
                { let ea = c.r[2].wrapping_add(0xffff9dec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[24].wrapping_add(0x4c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8015f3f8;
            }
            0x8015f3f8 => {
                c.r[0] = c.lwz(c.r[24].wrapping_add(0x10));
                { let ea = c.r[25].wrapping_add(0x14); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                c.r[0] = c.r[0].rotate_left(0) & 0x100000;
                c.rc(c.r[0]);
                { let ea = c.r[25].wrapping_add(0x10); c.f[29] = c.lfs(ea); c.ps1[29] = c.f[29]; }
                { let ea = c.r[25].wrapping_add(0x8); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                { let ea = c.r[25].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[31] = c.lwz(c.r[24].wrapping_add(0x3c));
                if c.crbit(2) { pc = 0x8015f458; continue; }
                pc = 0x8015f418;
            }
            0x8015f418 => {
                { let ea = c.r[24].wrapping_add(0x4c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[31] | c.r[31];
                { let ea = c.r[2].wrapping_add(0xffff9de8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = 0x7;
                { let v = s(c.f[0] * c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                c.r[5] = 0x7f;
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[6] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7f);
                if c.crbit(1) { pc = 0x8015f450; continue; }
                pc = 0x8015f44c;
            }
            0x8015f44c => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x8015f450;
            }
            0x8015f450 => {
                c.lr = 0x8015f454; f_8014c6b0(c);
                pc = 0x8015f454;
            }
            0x8015f454 => {
                pc = 0x8015f490; continue;
                pc = 0x8015f458;
            }
            0x8015f458 => {
                { let ea = c.r[2].wrapping_add(0xffff9de8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = 0x7f;
                { let v = s(c.f[0] * c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7f);
                if c.crbit(1) { pc = 0x8015f484; continue; }
                pc = 0x8015f47c;
            }
            0x8015f47c => {
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x8015f484;
            }
            0x8015f484 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7;
                c.lr = 0x8015f490; f_8014c6b0(c);
                pc = 0x8015f490;
            }
            0x8015f490 => {
                { let ea = c.r[2].wrapping_add(0xffff9dec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = 0x7f;
                { let ea = c.r[2].wrapping_add(0xffff9dfc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[0] + c.f[30]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7f);
                if c.crbit(1) { pc = 0x8015f4c4; continue; }
                pc = 0x8015f4bc;
            }
            0x8015f4bc => {
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x8015f4c4;
            }
            0x8015f4c4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0xa;
                c.lr = 0x8015f4d0; f_8014c6b0(c);
                pc = 0x8015f4d0;
            }
            0x8015f4d0 => {
                { let ea = c.r[2].wrapping_add(0xffff9dec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = 0x7f;
                { let ea = c.r[2].wrapping_add(0xffff9dfc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[0] - c.f[29]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x18); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7f);
                if c.crbit(1) { pc = 0x8015f504; continue; }
                pc = 0x8015f4fc;
            }
            0x8015f4fc => {
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x8015f504;
            }
            0x8015f504 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x83;
                c.lr = 0x8015f510; f_8014c6b0(c);
                pc = 0x8015f510;
            }
            0x8015f510 => {
                { let ea = c.r[2].wrapping_add(0xffff9e00); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] * c.f[31]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x8015f51c; f_800c46b0(c);
                pc = 0x8015f51c;
            }
            0x8015f51c => {
                c.cmpu(0, c.r[3], 0x3fff);
                c.r[5] = 0x3fff;
                if c.crbit(1) { pc = 0x8015f52c; continue; }
                pc = 0x8015f528;
            }
            0x8015f528 => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8015f52c;
            }
            0x8015f52c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x84;
                c.lr = 0x8015f538; f_8014c794(c);
                pc = 0x8015f538;
            }
            0x8015f538 => {
                c.r[3] = c.lwz(c.r[24].wrapping_add(0xc));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8015f5a4; continue; }
                pc = 0x8015f544;
            }
            0x8015f544 => {
                c.r[23] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[22] = 0x0;
                pc = 0x8015f590; continue;
                pc = 0x8015f550;
            }
            0x8015f550 => {
                c.r[4] = c.lbz(c.r[23]);
                c.cmpu(0, c.r[4], 0x40);
                if c.crbit(0) { pc = 0x8015f56c; continue; }
                pc = 0x8015f55c;
            }
            0x8015f55c => {
                c.cmpu(0, c.r[4], 0x80);
                if c.crbit(2) { pc = 0x8015f56c; continue; }
                pc = 0x8015f564;
            }
            0x8015f564 => {
                c.cmpu(0, c.r[4], 0x84);
                if !c.crbit(2) { pc = 0x8015f57c; continue; }
                pc = 0x8015f56c;
            }
            0x8015f56c => {
                c.r[5] = c.lhz(c.r[23].wrapping_add(0x2));
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8015f578; f_8014c794(c);
                pc = 0x8015f578;
            }
            0x8015f578 => {
                pc = 0x8015f588; continue;
                pc = 0x8015f57c;
            }
            0x8015f57c => {
                c.r[5] = c.lbz(c.r[23].wrapping_add(0x2));
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8015f588; f_8014c6b0(c);
                pc = 0x8015f588;
            }
            0x8015f588 => {
                c.r[23] = c.r[23].wrapping_add(0x4);
                c.r[22] = c.r[22].wrapping_add(0x1);
                pc = 0x8015f590;
            }
            0x8015f590 => {
                c.r[3] = c.lwz(c.r[24].wrapping_add(0xc));
                c.r[4] = c.r[22].rotate_left(0) & 0xff;
                c.r[0] = c.lbz(c.r[3]);
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x8015f550; continue; }
                pc = 0x8015f5a4;
            }
            0x8015f5a4 => {
                c.r[0] = c.lwz(c.r[24].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffdffff;
                c.stw(c.r[24].wrapping_add(0x10), c.r[0]);
                c.r[3] = c.lhz(c.r[28]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[28], c.r[0]);
                c.r[3] = c.lwz(c.r[30]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8015f5d0; continue; }
                pc = 0x8015f5c8;
            }
            0x8015f5c8 => {
                c.r[0] = c.lwz(c.r[3]);
                c.stw(c.r[30], c.r[0]);
                pc = 0x8015f5d0;
            }
            0x8015f5d0 => {
                c.r[25] = c.lwz(c.r[25]);
                pc = 0x8015f5d4;
            }
            0x8015f5d4 => {
                c.cmpu(0, c.r[25], 0x0);
                if !c.crbit(2) { pc = 0x8015f2c0; continue; }
                pc = 0x8015f5dc;
            }
            0x8015f5dc => {
                c.r[27] = c.r[27].wrapping_add(0x10);
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x8015f5e4;
            }
            0x8015f5e4 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa812));
                c.cmps(0, c.r[26], c.r[0]);
                if c.crbit(0) { pc = 0x8015f2ac; continue; }
                pc = 0x8015f5f0;
            }
            0x8015f5f0 => {
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x70); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x60); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                c.r[11] = c.r[1].wrapping_add(0x50);
                { let ea = c.r[1].wrapping_add(0x50); c.f[29] = c.lfd(ea); }
                c.lr = 0x8015f610; f_800c4810(c);
                pc = 0x8015f610;
            }
            0x8015f610 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x84));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x80);
                if true { return; }
                return;
            }
            _ => panic!("f_8015f270: bad pc {:#x}", pc),
        }
    }
}

/// fn_8015F620 (0x8015f620, 2092 bytes)
#[inline(never)]
pub fn f_8015f620(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8015f620) { return; }
        if c.prof { crate::prof::enter(c, 0x8015f620); f_8015f620_b(c, 0x8015f620); crate::prof::exit(c, 0x8015f620); return; }
    }
    f_8015f620_b(c, 0x8015f620)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015f620_at(c: &mut Cpu, pc: u32) { f_8015f620_b(c, pc) }
#[inline(always)]
fn f_8015f620_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8015f620;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8015f620, pc); }
        match pc {
            0x8015f620 => {
                { let ea = c.r[1].wrapping_add(0xffffff70); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x94), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x80); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x70); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = (c.f[29], c.ps1[29]); c.psq_store(ea, x, y, false, 0); }
                c.r[11] = c.r[1].wrapping_add(0x60);
                c.lr = 0x8015f64c; f_800c47d8(c);
                pc = 0x8015f64c;
            }
            0x8015f64c => {
                c.r[3] = c.lbz(c.r[13].wrapping_add(0xffffa82c));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8015f664; continue; }
                pc = 0x8015f658;
            }
            0x8015f658 => {
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.stb(c.r[13].wrapping_add(0xffffa82c), c.r[0]);
                pc = 0x8015fe1c; continue;
                pc = 0x8015f664;
            }
            0x8015f664 => {
                c.r[0] = 0x0;
                c.r[3] = 0x3;
                c.stb(c.r[13].wrapping_add(0xffffa82c), c.r[3]);
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffffa828));
                c.stb(c.r[13].wrapping_add(0xffffa812), c.r[0]);
                c.stb(c.r[13].wrapping_add(0xffffa811), c.r[0]);
                c.stb(c.r[13].wrapping_add(0xffffa810), c.r[0]);
                pc = 0x8015fc60; continue;
                pc = 0x8015f684;
            }
            0x8015f684 => {
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[3] = c.lwz(c.r[30]);
                c.r[0] = c.r[4].rotate_left(0) & 0x40000;
                c.rc(c.r[0]);
                c.r[29] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x8015f6ec; continue; }
                pc = 0x8015f698;
            }
            0x8015f698 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8015f6a8; continue; }
                pc = 0x8015f6a0;
            }
            0x8015f6a0 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x4));
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                pc = 0x8015f6a8;
            }
            0x8015f6a8 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x4));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8015f6c0; continue; }
                pc = 0x8015f6b4;
            }
            0x8015f6b4 => {
                c.r[0] = c.lwz(c.r[30]);
                c.stw(c.r[3], c.r[0]);
                pc = 0x8015f6c8; continue;
                pc = 0x8015f6c0;
            }
            0x8015f6c0 => {
                c.r[0] = c.lwz(c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffffa828), c.r[0]);
                pc = 0x8015f6c8;
            }
            0x8015f6c8 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x3c));
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015f6e4;
            }
            0x8015f6e4 => {
                c.lr = 0x8015f6e8; f_8014c8fc(c);
                pc = 0x8015f6e8;
            }
            0x8015f6e8 => {
                pc = 0x8015fc5c; continue;
                pc = 0x8015f6ec;
            }
            0x8015f6ec => {
                c.r[3] = 0x20000;
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[0] = c.r[4] & c.r[0];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8015f718; continue; }
                pc = 0x8015f6fc;
            }
            0x8015f6fc => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[1].wrapping_add(0x18);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[6] = c.r[1].wrapping_add(0x14);
                c.r[7] = c.r[1].wrapping_add(0x10);
                c.r[8] = c.r[1].wrapping_add(0xc);
                c.lr = 0x8015f718; f_8015e374(c);
                pc = 0x8015f718;
            }
            0x8015f718 => {
                c.r[7] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[7].rotate_left(0) & 0x80000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8015fc18; continue; }
                pc = 0x8015f724;
            }
            0x8015f724 => {
                c.r[0] = c.r[7].rotate_left(0) & 0x20000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8015f868; continue; }
                pc = 0x8015f72c;
            }
            0x8015f72c => {
                { let ea = c.r[2].wrapping_add(0xffff9dc8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x18); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[0], c.f[1]);
                if !c.crbit(2) { pc = 0x8015f75c; continue; }
                pc = 0x8015f73c;
            }
            0x8015f73c => {
                c.r[0] = c.r[7].rotate_left(0) & 0x4;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8015f75c; continue; }
                pc = 0x8015f744;
            }
            0x8015f744 => {
                c.r[0] = c.r[7] | 0x80000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffdffff;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015f8a0; continue;
                pc = 0x8015f75c;
            }
            0x8015f75c => {
                { let ea = c.r[2].wrapping_add(0xffff9dc8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[0], c.f[1]);
                if !c.crbit(2) { pc = 0x8015f7c8; continue; }
                pc = 0x8015f768;
            }
            0x8015f768 => {
                c.r[0] = c.r[7].rotate_left(0) & 0x40;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8015f7c8; continue; }
                pc = 0x8015f770;
            }
            0x8015f770 => {
                c.r[3] = c.lwz(c.r[30]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8015f784; continue; }
                pc = 0x8015f77c;
            }
            0x8015f77c => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x4));
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                pc = 0x8015f784;
            }
            0x8015f784 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x4));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8015f79c; continue; }
                pc = 0x8015f790;
            }
            0x8015f790 => {
                c.r[0] = c.lwz(c.r[30]);
                c.stw(c.r[3], c.r[0]);
                pc = 0x8015f7a4; continue;
                pc = 0x8015f79c;
            }
            0x8015f79c => {
                c.r[0] = c.lwz(c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffffa828), c.r[0]);
                pc = 0x8015f7a4;
            }
            0x8015f7a4 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x3c));
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015f7c0;
            }
            0x8015f7c0 => {
                c.lr = 0x8015f7c4; f_8014c8fc(c);
                pc = 0x8015f7c4;
            }
            0x8015f7c4 => {
                pc = 0x8015fc5c; continue;
                pc = 0x8015f7c8;
            }
            0x8015f7c8 => {
                c.r[0] = c.r[7].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8015f7f4; continue; }
                pc = 0x8015f7d0;
            }
            0x8015f7d0 => {
                { let ea = c.r[1].wrapping_add(0x14); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[3] = c.r[30] | c.r[30];
                { let ea = c.r[1].wrapping_add(0x10); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0xc); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                c.lr = 0x8015f7e8; f_8015f124(c);
                pc = 0x8015f7e8;
            }
            0x8015f7e8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015f7f0;
            }
            0x8015f7f0 => {
                pc = 0x8015f8a0; continue;
                pc = 0x8015f7f4;
            }
            0x8015f7f4 => {
                c.r[6] = c.lwz(c.r[30].wrapping_add(0x8));
                c.cmpu(0, c.r[6], 0x0);
                if c.crbit(2) { pc = 0x8015f80c; continue; }
                pc = 0x8015f800;
            }
            0x8015f800 => {
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x8015f844; continue; }
                pc = 0x8015f80c;
            }
            0x8015f80c => {
                c.cmpu(0, c.r[6], 0x0);
                c.r[3] = c.lhz(c.r[30].wrapping_add(0x44));
                c.r[4] = 0x7f;
                c.r[5] = 0x40;
                if c.crbit(2) { pc = 0x8015f828; continue; }
                pc = 0x8015f820;
            }
            0x8015f820 => {
                c.r[6] = c.lbz(c.r[6].wrapping_add(0x1c));
                pc = 0x8015f82c; continue;
                pc = 0x8015f828;
            }
            0x8015f828 => {
                c.r[6] = c.lbz(c.r[30].wrapping_add(0x46));
                pc = 0x8015f82c;
            }
            0x8015f82c => {
                c.r[7] = c.r[7].rotate_left(28) & 0x1;
                c.lr = 0x8015f834; f_8014c5e8(c);
                pc = 0x8015f834;
            }
            0x8015f834 => {
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[30].wrapping_add(0x3c), c.r[3]);
                c.cmpu(0, c.r[0], 0xffff);
                if !c.crbit(2) { pc = 0x8015f8a0; continue; }
                pc = 0x8015f844;
            }
            0x8015f844 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015f850;
            }
            0x8015f850 => {
                c.r[0] = c.r[3] | 0x40000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffdffff;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015f8a0; continue;
                pc = 0x8015f868;
            }
            0x8015f868 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x3c));
                c.lr = 0x8015f870; f_8014d880(c);
                pc = 0x8015f870;
            }
            0x8015f870 => {
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[30].wrapping_add(0x3c), c.r[3]);
                c.cmpu(0, c.r[0], 0xffff);
                if !c.crbit(2) { pc = 0x8015f8a0; continue; }
                pc = 0x8015f880;
            }
            0x8015f880 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8015f898; continue; }
                pc = 0x8015f88c;
            }
            0x8015f88c => {
                c.r[0] = c.r[3] | 0x20000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015f8a0; continue;
                pc = 0x8015f898;
            }
            0x8015f898 => {
                c.r[0] = c.r[3] | 0x40000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015f8a0;
            }
            0x8015f8a0 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x3c));
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x8015fbd8; continue; }
                pc = 0x8015f8b0;
            }
            0x8015f8b0 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8015f9e0; continue; }
                pc = 0x8015f8bc;
            }
            0x8015f8bc => {
                c.r[5] = c.lbz(c.r[13].wrapping_add(0xffffa812));
                c.r[3] = 0x80450000;
                c.r[4] = c.r[3].wrapping_add(0xffff8590);
                { let ea = c.r[1].wrapping_add(0x18); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[7] = 0x0;
                c.mtspr(9, c.r[5]);
                c.cmps(0, c.r[5], 0x0);
                if !c.crbit(1) { pc = 0x8015f8f8; continue; }
                pc = 0x8015f8dc;
            }
            0x8015f8dc => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x40));
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(2) { pc = 0x8015f8f8; continue; }
                pc = 0x8015f8ec;
            }
            0x8015f8ec => {
                c.r[4] = c.r[4].wrapping_add(0x10);
                c.r[7] = c.r[7].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8015f8dc; continue; }
                pc = 0x8015f8f8;
            }
            0x8015f8f8 => {
                c.cmps(0, c.r[7], c.r[5]);
                if !c.crbit(2) { pc = 0x8015f934; continue; }
                pc = 0x8015f900;
            }
            0x8015f900 => {
                c.r[4] = 0x80450000;
                c.r[3] = c.lbz(c.r[13].wrapping_add(0xffffa812));
                c.r[5] = c.r[7].rotate_left(4) & 0xfffffff0;
                c.r[6] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0xffff8590);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[4] = c.r[4].wrapping_add(c.r[5]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[6]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[6]);
                c.sth(c.r[4].wrapping_add(0xc), c.r[6]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x40));
                c.stw(c.r[4], c.r[3]);
                c.stb(c.r[13].wrapping_add(0xffffa812), c.r[0]);
                pc = 0x8015f934;
            }
            0x8015f934 => {
                c.r[3] = 0x80450000;
                c.r[4] = c.r[7].rotate_left(4) & 0xfffffff0;
                c.r[0] = c.r[3].wrapping_add(0xffff8590);
                c.r[5] = 0x0;
                c.r[4] = c.r[0].wrapping_add(c.r[4]);
                c.r[3] = c.lhz(c.r[4].wrapping_add(0xc));
                c.r[7] = c.r[4].wrapping_add(0x8);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[6] = c.lwz(c.r[4].wrapping_add(0x8));
                pc = 0x8015f974; continue;
                pc = 0x8015f960;
            }
            0x8015f960 => {
                { let ea = c.r[6].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[0], c.f[1]);
                if c.crbit(1) { pc = 0x8015f97c; continue; }
                pc = 0x8015f96c;
            }
            0x8015f96c => {
                c.r[5] = c.r[6] | c.r[6];
                c.r[6] = c.lwz(c.r[6]);
                pc = 0x8015f974;
            }
            0x8015f974 => {
                c.cmpu(0, c.r[6], 0x0);
                if !c.crbit(2) { pc = 0x8015f960; continue; }
                pc = 0x8015f97c;
            }
            0x8015f97c => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8015f9a0; continue; }
                pc = 0x8015f984;
            }
            0x8015f984 => {
                c.r[4] = c.lbz(c.r[13].wrapping_add(0xffffa810));
                c.r[3] = 0x80450000;
                c.r[0] = c.r[3].wrapping_add(0xffff8990);
                c.r[3] = c.r[4].wrapping_mul(0xc);
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.stw(c.r[7], c.r[0]);
                pc = 0x8015f9b8; continue;
                pc = 0x8015f9a0;
            }
            0x8015f9a0 => {
                c.r[4] = c.lbz(c.r[13].wrapping_add(0xffffa810));
                c.r[3] = 0x80450000;
                c.r[0] = c.r[3].wrapping_add(0xffff8990);
                c.r[3] = c.r[4].wrapping_mul(0xc);
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.stw(c.r[5], c.r[0]);
                pc = 0x8015f9b8;
            }
            0x8015f9b8 => {
                c.r[5] = c.lbz(c.r[13].wrapping_add(0xffffa810));
                c.r[3] = 0x80450000;
                c.r[3] = c.r[3].wrapping_add(0xffff8990);
                c.r[4] = c.r[5].wrapping_mul(0xc);
                c.r[0] = c.r[5].wrapping_add(0x1);
                c.stb(c.r[13].wrapping_add(0xffffa810), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.stw(c.r[3], c.r[6]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[30]);
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[1]; c.stfs(ea, v); }
                pc = 0x8015f9e0;
            }
            0x8015f9e0 => {
                { let ea = c.r[2].wrapping_add(0xffff9dc8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x18); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.fcmp(0, c.f[0], c.f[2]);
                if !c.crbit(2) { pc = 0x8015fa30; continue; }
                pc = 0x8015f9f0;
            }
            0x8015f9f0 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0x4;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8015fa30; continue; }
                pc = 0x8015f9fc;
            }
            0x8015f9fc => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x3c));
                c.lr = 0x8015fa04; f_8014c8fc(c);
                pc = 0x8015fa04;
            }
            0x8015fa04 => {
                c.r[0] = 0xffffffff;
                c.stw(c.r[30].wrapping_add(0x3c), c.r[0]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8015fa24; continue; }
                pc = 0x8015fa18;
            }
            0x8015fa18 => {
                c.r[0] = c.r[3] | 0x80000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015fbd8; continue;
                pc = 0x8015fa24;
            }
            0x8015fa24 => {
                c.r[0] = c.r[3] | 0x40000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015fbd8; continue;
                pc = 0x8015fa30;
            }
            0x8015fa30 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[1].wrapping_add(0x8); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                c.r[0] = c.r[0].rotate_left(0) & 0x100000;
                c.rc(c.r[0]);
                { let ea = c.r[1].wrapping_add(0xc); c.f[29] = c.lfs(ea); c.ps1[29] = c.f[29]; }
                { let ea = c.r[1].wrapping_add(0x14); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                c.r[31] = c.lwz(c.r[30].wrapping_add(0x3c));
                if c.crbit(2) { pc = 0x8015fa8c; continue; }
                pc = 0x8015fa4c;
            }
            0x8015fa4c => {
                { let ea = c.r[30].wrapping_add(0x4c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[31] | c.r[31];
                { let ea = c.r[2].wrapping_add(0xffff9de8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = 0x7;
                { let v = s(c.f[0] * c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                c.r[5] = 0x7f;
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                c.r[6] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7f);
                if c.crbit(1) { pc = 0x8015fa84; continue; }
                pc = 0x8015fa80;
            }
            0x8015fa80 => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x8015fa84;
            }
            0x8015fa84 => {
                c.lr = 0x8015fa88; f_8014c6b0(c);
                pc = 0x8015fa88;
            }
            0x8015fa88 => {
                pc = 0x8015fac4; continue;
                pc = 0x8015fa8c;
            }
            0x8015fa8c => {
                { let ea = c.r[2].wrapping_add(0xffff9de8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = 0x7f;
                { let v = s(c.f[0] * c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7f);
                if c.crbit(1) { pc = 0x8015fab8; continue; }
                pc = 0x8015fab0;
            }
            0x8015fab0 => {
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz(c.r[1].wrapping_add(0x2c));
                pc = 0x8015fab8;
            }
            0x8015fab8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7;
                c.lr = 0x8015fac4; f_8014c6b0(c);
                pc = 0x8015fac4;
            }
            0x8015fac4 => {
                { let ea = c.r[2].wrapping_add(0xffff9dec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = 0x7f;
                { let ea = c.r[2].wrapping_add(0xffff9dfc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[0] + c.f[30]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7f);
                if c.crbit(1) { pc = 0x8015faf8; continue; }
                pc = 0x8015faf0;
            }
            0x8015faf0 => {
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x8015faf8;
            }
            0x8015faf8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0xa;
                c.lr = 0x8015fb04; f_8014c6b0(c);
                pc = 0x8015fb04;
            }
            0x8015fb04 => {
                { let ea = c.r[2].wrapping_add(0xffff9dec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = 0x7f;
                { let ea = c.r[2].wrapping_add(0xffff9dfc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[0] - c.f[29]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7f);
                if c.crbit(1) { pc = 0x8015fb38; continue; }
                pc = 0x8015fb30;
            }
            0x8015fb30 => {
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz(c.r[1].wrapping_add(0x3c));
                pc = 0x8015fb38;
            }
            0x8015fb38 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x83;
                c.lr = 0x8015fb44; f_8014c6b0(c);
                pc = 0x8015fb44;
            }
            0x8015fb44 => {
                { let ea = c.r[2].wrapping_add(0xffff9e00); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] * c.f[31]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x8015fb50; f_800c46b0(c);
                pc = 0x8015fb50;
            }
            0x8015fb50 => {
                c.cmpu(0, c.r[3], 0x3fff);
                c.r[5] = 0x3fff;
                if c.crbit(1) { pc = 0x8015fb60; continue; }
                pc = 0x8015fb5c;
            }
            0x8015fb5c => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8015fb60;
            }
            0x8015fb60 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x84;
                c.lr = 0x8015fb6c; f_8014c794(c);
                pc = 0x8015fb6c;
            }
            0x8015fb6c => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8015fbd8; continue; }
                pc = 0x8015fb78;
            }
            0x8015fb78 => {
                c.r[28] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[27] = 0x0;
                pc = 0x8015fbc4; continue;
                pc = 0x8015fb84;
            }
            0x8015fb84 => {
                c.r[4] = c.lbz(c.r[28]);
                c.cmpu(0, c.r[4], 0x40);
                if c.crbit(0) { pc = 0x8015fba0; continue; }
                pc = 0x8015fb90;
            }
            0x8015fb90 => {
                c.cmpu(0, c.r[4], 0x80);
                if c.crbit(2) { pc = 0x8015fba0; continue; }
                pc = 0x8015fb98;
            }
            0x8015fb98 => {
                c.cmpu(0, c.r[4], 0x84);
                if !c.crbit(2) { pc = 0x8015fbb0; continue; }
                pc = 0x8015fba0;
            }
            0x8015fba0 => {
                c.r[5] = c.lhz(c.r[28].wrapping_add(0x2));
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8015fbac; f_8014c794(c);
                pc = 0x8015fbac;
            }
            0x8015fbac => {
                pc = 0x8015fbbc; continue;
                pc = 0x8015fbb0;
            }
            0x8015fbb0 => {
                c.r[5] = c.lbz(c.r[28].wrapping_add(0x2));
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8015fbbc; f_8014c6b0(c);
                pc = 0x8015fbbc;
            }
            0x8015fbbc => {
                c.r[28] = c.r[28].wrapping_add(0x4);
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x8015fbc4;
            }
            0x8015fbc4 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[4] = c.r[27].rotate_left(0) & 0xff;
                c.r[0] = c.lbz(c.r[3]);
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x8015fb84; continue; }
                pc = 0x8015fbd8;
            }
            0x8015fbd8 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0x100000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015fbe4;
            }
            0x8015fbe4 => {
                { let ea = c.r[30].wrapping_add(0x4c); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[2].wrapping_add(0xffff9e10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9dec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] + c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[30].wrapping_add(0x4c); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x4c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[1], c.f[0]);
                { let (a, b) = (c.crbit(1), c.crbit(2)); c.set_crbit(2, a | b); }
                if !c.crbit(2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015fc08;
            }
            0x8015fc08 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xffefffff;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015fc5c; continue;
                pc = 0x8015fc18;
            }
            0x8015fc18 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x8));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8015fc34; continue; }
                pc = 0x8015fc24;
            }
            0x8015fc24 => {
                if c.crbit(2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015fc28;
            }
            0x8015fc28 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015fc34;
            }
            0x8015fc34 => {
                { let ea = c.r[2].wrapping_add(0xffff9dc8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if c.crbit(2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015fc44;
            }
            0x8015fc44 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xfff7ffff;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0] | 0x20000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015fc5c;
            }
            0x8015fc5c => {
                c.r[30] = c.r[29] | c.r[29];
                pc = 0x8015fc60;
            }
            0x8015fc60 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8015f684; continue; }
                pc = 0x8015fc68;
            }
            0x8015fc68 => {
                c.lr = 0x8015fc6c; f_8015f270(c);
                pc = 0x8015fc6c;
            }
            0x8015fc6c => {
                c.lr = 0x8015fc70; f_8015dec0(c);
                pc = 0x8015fc70;
            }
            0x8015fc70 => {
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa81c));
                pc = 0x8015fe14; continue;
                pc = 0x8015fc78;
            }
            0x8015fc78 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x28));
                c.r[0] = c.r[0].rotate_left(0) & 0x80000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8015fd50; continue; }
                pc = 0x8015fc84;
            }
            0x8015fc84 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x20));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x8015fe10; continue; }
                pc = 0x8015fc94;
            }
            0x8015fc94 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x8015fe10; continue; }
                pc = 0x8015fca4;
            }
            0x8015fca4 => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x1c));
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0x38), c.r[0]);
                c.r[3] = 0x0;
                { let ea = c.r[2].wrapping_add(0xffff9dd0); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x3c), c.r[4]);
                { let ea = c.r[31].wrapping_add(0x14); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x38); c.f[1] = c.lfd(ea); }
                { let ea = c.r[2].wrapping_add(0xffff9de8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] * c.f[3]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                c.f[0] = fctiwz(c.f[0]);
                c.f[1] = fctiwz(c.f[1]);
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[0]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[1]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x34));
                c.stb(c.r[31].wrapping_add(0x35), c.r[4]);
                c.stb(c.r[31].wrapping_add(0x36), c.r[3]);
                c.stb(c.r[31].wrapping_add(0x34), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x28));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8015fd24; continue; }
                pc = 0x8015fd04;
            }
            0x8015fd04 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[4] = c.r[31].wrapping_add(0x34);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.stb(c.r[31].wrapping_add(0x37), c.r[0]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x20));
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.lr = 0x8015fd20; f_8014dd98(c);
                pc = 0x8015fd20;
            }
            0x8015fd20 => {
                pc = 0x8015fd40; continue;
                pc = 0x8015fd24;
            }
            0x8015fd24 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x20));
                c.r[4] = c.r[31].wrapping_add(0x34);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.stb(c.r[31].wrapping_add(0x37), c.r[0]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.lr = 0x8015fd40; f_8014dd98(c);
                pc = 0x8015fd40;
            }
            0x8015fd40 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x28));
                c.r[0] = c.r[0] | 0x80000000;
                c.stw(c.r[31].wrapping_add(0x28), c.r[0]);
                pc = 0x8015fe10; continue;
                pc = 0x8015fd50;
            }
            0x8015fd50 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x20));
                c.r[4] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[4], 0xff);
                if c.crbit(2) { pc = 0x8015fd70; continue; }
                pc = 0x8015fd60;
            }
            0x8015fd60 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0xff);
                if !c.crbit(2) { pc = 0x8015fdbc; continue; }
                pc = 0x8015fd70;
            }
            0x8015fd70 => {
                c.cmpu(0, c.r[4], 0xff);
                if c.crbit(2) { pc = 0x8015fd84; continue; }
                pc = 0x8015fd78;
            }
            0x8015fd78 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1d));
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(2) { pc = 0x8015fda0; continue; }
                pc = 0x8015fd84;
            }
            0x8015fd84 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[3], 0xff);
                if c.crbit(2) { pc = 0x8015fdac; continue; }
                pc = 0x8015fd94;
            }
            0x8015fd94 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1d));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x8015fdac; continue; }
                pc = 0x8015fda0;
            }
            0x8015fda0 => {
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x1d));
                c.r[4] = c.r[31].wrapping_add(0x34);
                c.lr = 0x8015fdac; f_8014ddb8(c);
                pc = 0x8015fdac;
            }
            0x8015fdac => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x28));
                c.r[0] = c.r[0].rotate_left(0) & 0x7fffffff;
                c.stw(c.r[31].wrapping_add(0x28), c.r[0]);
                pc = 0x8015fe10; continue;
                pc = 0x8015fdbc;
            }
            0x8015fdbc => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x1c));
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0x38), c.r[0]);
                c.r[3] = 0x0;
                { let ea = c.r[2].wrapping_add(0xffff9dd0); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x3c), c.r[4]);
                { let ea = c.r[31].wrapping_add(0x14); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x38); c.f[1] = c.lfd(ea); }
                { let ea = c.r[2].wrapping_add(0xffff9de8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] * c.f[3]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                c.f[0] = fctiwz(c.f[0]);
                c.f[1] = fctiwz(c.f[1]);
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[0]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[1]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x34));
                c.stb(c.r[31].wrapping_add(0x35), c.r[4]);
                c.stb(c.r[31].wrapping_add(0x36), c.r[3]);
                c.stb(c.r[31].wrapping_add(0x34), c.r[0]);
                pc = 0x8015fe10;
            }
            0x8015fe10 => {
                c.r[31] = c.lwz(c.r[31]);
                pc = 0x8015fe14;
            }
            0x8015fe14 => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8015fc78; continue; }
                pc = 0x8015fe1c;
            }
            0x8015fe1c => {
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x80); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x70); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                c.r[11] = c.r[1].wrapping_add(0x60);
                { let ea = c.r[1].wrapping_add(0x60); c.f[29] = c.lfd(ea); }
                c.lr = 0x8015fe3c; f_800c4824(c);
                pc = 0x8015fe3c;
            }
            0x8015fe3c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x94));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x90);
                if true { return; }
                return;
            }
            _ => panic!("f_8015f620: bad pc {:#x}", pc),
        }
    }
}

/// fn_8015FE4C (0x8015fe4c, 56 bytes)
#[inline(never)]
pub fn f_8015fe4c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8015fe4c) { return; }
        if c.prof { crate::prof::enter(c, 0x8015fe4c); f_8015fe4c_b(c, 0x8015fe4c); crate::prof::exit(c, 0x8015fe4c); return; }
    }
    f_8015fe4c_b(c, 0x8015fe4c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015fe4c_at(c: &mut Cpu, pc: u32) { f_8015fe4c_b(c, pc) }
#[inline(always)]
fn f_8015fe4c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8015fe4c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8015fe4c, pc); }
        match pc {
            0x8015fe4c => {
                c.r[5] = 0x0;
                c.r[0] = c.r[3].rotate_left(31) & 0x1;
                c.r[4] = 0x1;
                c.r[3] = 0x3;
                c.stw(c.r[13].wrapping_add(0xffffa828), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa824), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa820), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa81c), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa818), c.r[5]);
                c.stb(c.r[13].wrapping_add(0xffffa815), c.r[4]);
                c.stb(c.r[13].wrapping_add(0xffffa814), c.r[3]);
                c.stb(c.r[13].wrapping_add(0xffffa82c), c.r[5]);
                c.stb(c.r[13].wrapping_add(0xffffa813), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8015fe4c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8015FE84 (0x8015fe84, 4 bytes)
#[inline(never)]
pub fn f_8015fe84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8015fe84) { return; }
        if c.prof { crate::prof::enter(c, 0x8015fe84); f_8015fe84_b(c, 0x8015fe84); crate::prof::exit(c, 0x8015fe84); return; }
    }
    f_8015fe84_b(c, 0x8015fe84)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015fe84_at(c: &mut Cpu, pc: u32) { f_8015fe84_b(c, pc) }
#[inline(always)]
fn f_8015fe84_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8015fe84;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8015fe84, pc); }
        match pc {
            0x8015fe84 => {
                if true { return; }
                return;
            }
            _ => panic!("f_8015fe84: bad pc {:#x}", pc),
        }
    }
}

/// fn_8015FE88 (0x8015fe88, 280 bytes)
#[inline(never)]
pub fn f_8015fe88(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8015fe88) { return; }
        if c.prof { crate::prof::enter(c, 0x8015fe88); f_8015fe88_b(c, 0x8015fe88); crate::prof::exit(c, 0x8015fe88); return; }
    }
    f_8015fe88_b(c, 0x8015fe88)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015fe88_at(c: &mut Cpu, pc: u32) { f_8015fe88_b(c, pc) }
#[inline(always)]
fn f_8015fe88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8015fe88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8015fe88, pc); }
        match pc {
            0x8015fe88 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[9] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x40);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[8] | c.r[8];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[7] | c.r[7];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stb(c.r[13].wrapping_add(0xffffa6f8), c.r[9]);
                if c.crbit(1) { pc = 0x8015fecc; continue; }
                pc = 0x8015febc;
            }
            0x8015febc => {
                c.r[7] = 0x80430000;
                c.r[7] = c.r[7].wrapping_add(0x4c50);
                c.stb(c.r[7].wrapping_add(0x210), c.r[3]);
                pc = 0x8015fedc; continue;
                pc = 0x8015fecc;
            }
            0x8015fecc => {
                c.r[3] = 0x80430000;
                c.r[0] = 0x40;
                c.r[3] = c.r[3].wrapping_add(0x4c50);
                c.stb(c.r[3].wrapping_add(0x210), c.r[0]);
                pc = 0x8015fedc;
            }
            0x8015fedc => {
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x8);
                if c.crbit(1) { pc = 0x8015fef8; continue; }
                pc = 0x8015fee8;
            }
            0x8015fee8 => {
                c.r[3] = 0x80430000;
                c.r[3] = c.r[3].wrapping_add(0x4c50);
                c.stb(c.r[3].wrapping_add(0x213), c.r[6]);
                pc = 0x8015ff08; continue;
                pc = 0x8015fef8;
            }
            0x8015fef8 => {
                c.r[3] = 0x80430000;
                c.r[0] = 0x8;
                c.r[3] = c.r[3].wrapping_add(0x4c50);
                c.stb(c.r[3].wrapping_add(0x213), c.r[0]);
                pc = 0x8015ff08;
            }
            0x8015ff08 => {
                c.r[3] = 0x80430000;
                c.r[0] = 0x7d00;
                c.r[29] = c.r[3].wrapping_add(0x4c50);
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[6] = c.r[30] | c.r[30];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.stb(c.r[29].wrapping_add(0x211), c.r[4]);
                c.r[4] = c.lbz(c.r[29].wrapping_add(0x210));
                c.stb(c.r[29].wrapping_add(0x212), c.r[5]);
                c.r[5] = c.lbz(c.r[29].wrapping_add(0x213));
                c.lr = 0x8015ff34; f_80162370(c);
                pc = 0x8015ff34;
            }
            0x8015ff34 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8015ff84; continue; }
                pc = 0x8015ff3c;
            }
            0x8015ff3c => {
                c.r[29] = c.lbz(c.r[29].wrapping_add(0x210));
                c.lr = 0x8015ff44; f_80159c48(c);
                pc = 0x8015ff44;
            }
            0x8015ff44 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.lr = 0x8015ff50; f_801524e0(c);
                pc = 0x8015ff50;
            }
            0x8015ff50 => {
                c.lr = 0x8015ff54; f_8014a094(c);
                pc = 0x8015ff54;
            }
            0x8015ff54 => {
                c.r[0] = 0x0;
                c.r[4] = c.r[29] | c.r[29];
                c.stb(c.r[13].wrapping_add(0xffffa730), c.r[0]);
                c.r[3] = 0x7d00;
                c.lr = 0x8015ff68; f_8014d000(c);
                pc = 0x8015ff68;
            }
            0x8015ff68 => {
                c.lr = 0x8015ff6c; f_8014ddd8(c);
                pc = 0x8015ff6c;
            }
            0x8015ff6c => {
                c.lr = 0x8015ff70; f_80159494(c);
                pc = 0x8015ff70;
            }
            0x8015ff70 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8015ff78; f_8015fe4c(c);
                pc = 0x8015ff78;
            }
            0x8015ff78 => {
                c.r[0] = 0x1;
                c.r[3] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa6f8), c.r[0]);
                pc = 0x8015ff84;
            }
            0x8015ff84 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8015fe88: bad pc {:#x}", pc),
        }
    }
}

/// sndQuit (0x8015ffa0, 52 bytes)
#[inline(never)]
pub fn f_8015ffa0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8015ffa0) { return; }
        if c.prof { crate::prof::enter(c, 0x8015ffa0); f_8015ffa0_b(c, 0x8015ffa0); crate::prof::exit(c, 0x8015ffa0); return; }
    }
    f_8015ffa0_b(c, 0x8015ffa0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015ffa0_at(c: &mut Cpu, pc: u32) { f_8015ffa0_b(c, pc) }
#[inline(always)]
fn f_8015ffa0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8015ffa0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8015ffa0, pc); }
        match pc {
            0x8015ffa0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8015ffb0; f_80162428(c);
                pc = 0x8015ffb0;
            }
            0x8015ffb0 => {
                c.lr = 0x8015ffb4; f_801525c4(c);
                pc = 0x8015ffb4;
            }
            0x8015ffb4 => {
                c.lr = 0x8015ffb8; f_8015fe84(c);
                pc = 0x8015ffb8;
            }
            0x8015ffb8 => {
                c.lr = 0x8015ffbc; f_8014d574(c);
                pc = 0x8015ffbc;
            }
            0x8015ffbc => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa6f8), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8015ffa0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8015FFD4 (0x8015ffd4, 8 bytes)
#[inline(never)]
pub fn f_8015ffd4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8015ffd4) { return; }
        if c.prof { crate::prof::enter(c, 0x8015ffd4); f_8015ffd4_b(c, 0x8015ffd4); crate::prof::exit(c, 0x8015ffd4); return; }
    }
    f_8015ffd4_b(c, 0x8015ffd4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015ffd4_at(c: &mut Cpu, pc: u32) { f_8015ffd4_b(c, pc) }
#[inline(always)]
fn f_8015ffd4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8015ffd4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8015ffd4, pc); }
        match pc {
            0x8015ffd4 => {
                c.r[3] = c.lbz(c.r[13].wrapping_add(0xffffa6f8));
                if true { return; }
                return;
            }
            _ => panic!("f_8015ffd4: bad pc {:#x}", pc),
        }
    }
}

/// salApplyMatrix (0x8015ffdc, 172 bytes)
#[inline(never)]
pub fn f_8015ffdc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8015ffdc) { return; }
        if c.prof { crate::prof::enter(c, 0x8015ffdc); f_8015ffdc_b(c, 0x8015ffdc); crate::prof::exit(c, 0x8015ffdc); return; }
    }
    f_8015ffdc_b(c, 0x8015ffdc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015ffdc_at(c: &mut Cpu, pc: u32) { f_8015ffdc_b(c, pc) }
#[inline(always)]
fn f_8015ffdc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8015ffdc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8015ffdc, pc); }
        match pc {
            0x8015ffdc => {
                { let ea = c.r[3]; c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[4]; c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[3].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0x24); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[2] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[4] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[5]; let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0xc); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[4]; c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[3].wrapping_add(0x10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3].wrapping_add(0x14); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0x28); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[2] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[4] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[5].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x18); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[4]; c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[3].wrapping_add(0x1c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3].wrapping_add(0x20); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0x2c); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[2] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[4] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[5].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_8015ffdc: bad pc {:#x}", pc),
        }
    }
}

/// salNormalizeVector (0x80160088, 180 bytes)
#[inline(never)]
pub fn f_80160088(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80160088) { return; }
        if c.prof { crate::prof::enter(c, 0x80160088); f_80160088_b(c, 0x80160088); crate::prof::exit(c, 0x80160088); return; }
    }
    f_80160088_b(c, 0x80160088)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160088_at(c: &mut Cpu, pc: u32) { f_80160088_b(c, pc) }
#[inline(always)]
fn f_80160088_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80160088;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80160088, pc); }
        match pc {
            0x80160088 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[3]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] * c.f[1]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[0] * c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9e18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[2] + c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[3] + c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x80160110; continue; }
                pc = 0x801600b8;
            }
            0x801600b8 => {
                c.f[2] = frsqrte(c.f[1]);
                { let ea = c.r[2].wrapping_add(0xffff9e20); c.f[4] = c.lfd(ea); }
                { let ea = c.r[2].wrapping_add(0xffff9e28); c.f[3] = c.lfd(ea); }
                c.f[0] = c.f[2] * c.f[2];
                c.f[2] = c.f[4] * c.f[2];
                c.f[0] = c.f[1] * c.f[0];
                c.f[0] = c.f[3] - c.f[0];
                c.f[2] = c.f[2] * c.f[0];
                c.f[0] = c.f[2] * c.f[2];
                c.f[2] = c.f[4] * c.f[2];
                c.f[0] = c.f[1] * c.f[0];
                c.f[0] = c.f[3] - c.f[0];
                c.f[2] = c.f[2] * c.f[0];
                c.f[0] = c.f[2] * c.f[2];
                c.f[2] = c.f[4] * c.f[2];
                c.f[0] = c.f[1] * c.f[0];
                c.f[0] = c.f[3] - c.f[0];
                c.f[0] = c.f[2] * c.f[0];
                c.f[0] = c.f[1] * c.f[0];
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x80160110;
            }
            0x80160110 => {
                { let ea = c.r[3]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] / c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3]; let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] / c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] / c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80160088: bad pc {:#x}", pc),
        }
    }
}

/// salCrossProduct (0x8016013c, 100 bytes)
#[inline(never)]
pub fn f_8016013c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016013c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016013c); f_8016013c_b(c, 0x8016013c); crate::prof::exit(c, 0x8016013c); return; }
    }
    f_8016013c_b(c, 0x8016013c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016013c_at(c: &mut Cpu, pc: u32) { f_8016013c_b(c, pc) }
#[inline(always)]
fn f_8016013c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016013c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016013c, pc); }
        match pc {
            0x8016013c => {
                { let ea = c.r[4].wrapping_add(0x4); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[5].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[5].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[2] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3]; let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[5]; c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[5].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[2] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4]; c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[5].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[5]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[2] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_8016013c: bad pc {:#x}", pc),
        }
    }
}

/// salInvertMatrix (0x801601a0, 508 bytes)
#[inline(never)]
pub fn f_801601a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801601a0) { return; }
        if c.prof { crate::prof::enter(c, 0x801601a0); f_801601a0_b(c, 0x801601a0); crate::prof::exit(c, 0x801601a0); return; }
    }
    f_801601a0_b(c, 0x801601a0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801601a0_at(c: &mut Cpu, pc: u32) { f_801601a0_b(c, pc) }
#[inline(always)]
fn f_801601a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801601a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801601a0, pc); }
        match pc {
            0x801601a0 => {
                { let ea = c.r[4].wrapping_add(0x20); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[4].wrapping_add(0xc); c.f[10] = c.lfs(ea); c.ps1[10] = c.f[10]; }
                { let ea = c.r[4].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[4].wrapping_add(0x1c); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                { let v = s(c.f[10] * c.f[5]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[4].wrapping_add(0x18); c.f[9] = c.lfs(ea); c.ps1[9] = c.f[9]; }
                { let ea = c.r[4].wrapping_add(0x10); c.f[8] = c.lfs(ea); c.ps1[8] = c.f[8]; }
                { let v = s(c.f[6] * c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[9] * c.f[0]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[4]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[8] * c.f[5]); c.f[7] = v; c.ps1[7] = v; }
                { let ea = c.r[4].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[10] * c.f[6]); c.f[5] = v; c.ps1[5] = v; }
                { let v = s(c.f[3] - c.f[2]); c.f[6] = v; c.ps1[6] = v; }
                { let v = s(c.f[7] - c.f[4]); c.f[7] = v; c.ps1[7] = v; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[9] * c.f[8]); c.f[4] = v; c.ps1[4] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9e30); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.f[6] = -c.f[6];
                { let v = s(c.f[1] * c.f[7]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[5] - c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[0] * c.f[6]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[2] * c.f[4]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[2] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[3] / c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] * c.f[7]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[6]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[1] * c.f[4]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3]; let v = c.f[0]; c.stfs(ea, v); }
                c.f[0] = -c.f[1];
                { let ea = c.r[3].wrapping_add(0xc); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x18); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x4); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[4].wrapping_add(0x20); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[4].wrapping_add(0x1c); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[5] * c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[4] - c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[0] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[4]; c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[4].wrapping_add(0x20); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[4].wrapping_add(0x18); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[5] * c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[4] - c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3].wrapping_add(0x10); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[4]; c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[4].wrapping_add(0x1c); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[4].wrapping_add(0x18); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[4].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[5] * c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[4] - c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[0] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3].wrapping_add(0x1c); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x4); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[4].wrapping_add(0x14); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[4].wrapping_add(0x10); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[5] * c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[4] - c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[4]; c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[4].wrapping_add(0x14); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[4].wrapping_add(0xc); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[5] * c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[4] - c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[0] * c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x14); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4]; c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[4].wrapping_add(0x10); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[4].wrapping_add(0xc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[4] * c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[2] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[3] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x24); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3]; c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.f[5] = -c.f[0];
                { let ea = c.r[4].wrapping_add(0x28); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[3].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4].wrapping_add(0x2c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[5] * c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[4] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x24); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0xc); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.f[5] = -c.f[0];
                { let ea = c.r[4].wrapping_add(0x28); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[3].wrapping_add(0x10); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4].wrapping_add(0x2c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[5] * c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[4] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x24); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[4] = -c.f[1];
                { let ea = c.r[4].wrapping_add(0x28); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[3].wrapping_add(0x1c); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4].wrapping_add(0x2c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[4] * c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                { let ea = c.r[3].wrapping_add(0x20); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[4] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_801601a0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8016039C (0x8016039c, 36 bytes)
#[inline(never)]
pub fn f_8016039c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016039c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016039c); f_8016039c_b(c, 0x8016039c); crate::prof::exit(c, 0x8016039c); return; }
    }
    f_8016039c_b(c, 0x8016039c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016039c_at(c: &mut Cpu, pc: u32) { f_8016039c_b(c, pc) }
#[inline(always)]
fn f_8016039c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016039c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016039c, pc); }
        match pc {
            0x8016039c => {
                c.r[6] = 0x80450000;
                c.r[4] = c.r[4].rotate_left(6) & 0x3fc0;
                c.r[0] = c.r[6].wrapping_add(0xffff9390);
                c.r[3] = c.r[3].rotate_left(2) & 0x3fc;
                c.r[4] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[3]));
                c.r[0] = c.r[0] | c.r[5];
                c.stw(c.r[4].wrapping_add(c.r[3]), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8016039c: bad pc {:#x}", pc),
        }
    }
}

/// fn_801603C0 (0x801603c0, 1544 bytes)
#[inline(never)]
pub fn f_801603c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801603c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801603c0); f_801603c0_b(c, 0x801603c0); crate::prof::exit(c, 0x801603c0); return; }
    }
    f_801603c0_b(c, 0x801603c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801603c0_at(c: &mut Cpu, pc: u32) { f_801603c0_b(c, pc) }
#[inline(always)]
fn f_801603c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801603c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801603c0, pc); }
        match pc {
            0x801603c0 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x801603d4; f_800c47d0(c);
                pc = 0x801603d4;
            }
            0x801603d4 => {
                c.r[7] = c.r[4].rotate_left(0) & 0xff;
                c.r[8] = 0x80450000;
                c.cmpu(0, c.r[7], 0xff);
                c.r[31] = c.r[8].wrapping_add(0xffff9390);
                if c.crbit(2) { pc = 0x801609b0; continue; }
                pc = 0x801603e8;
            }
            0x801603e8 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x801606e4; continue; }
                pc = 0x801603f4;
            }
            0x801603f4 => {
                c.r[8] = c.r[3].rotate_left(0) & 0xff;
                c.cmps(0, c.r[8], 0x26);
                if c.crbit(2) { pc = 0x80160644; continue; }
                pc = 0x80160400;
            }
            0x80160400 => {
                if !c.crbit(0) { pc = 0x80160410; continue; }
                pc = 0x80160404;
            }
            0x80160404 => {
                c.cmps(0, c.r[8], 0x6);
                if c.crbit(2) { pc = 0x80160428; continue; }
                pc = 0x8016040c;
            }
            0x8016040c => {
                pc = 0x80160644; continue;
                pc = 0x80160410;
            }
            0x80160410 => {
                c.cmps(0, c.r[8], 0x61);
                if c.crbit(2) { pc = 0x80160594; continue; }
                pc = 0x80160418;
            }
            0x80160418 => {
                if !c.crbit(0) { pc = 0x80160644; continue; }
                pc = 0x8016041c;
            }
            0x8016041c => {
                c.cmps(0, c.r[8], 0x60);
                if !c.crbit(0) { pc = 0x801604e0; continue; }
                pc = 0x80160424;
            }
            0x80160424 => {
                pc = 0x80160644; continue;
                pc = 0x80160428;
            }
            0x80160428 => {
                c.r[10] = c.r[0].wrapping_mul(0x860);
                c.r[8] = c.r[31].wrapping_add(0x200);
                c.r[9] = c.r[7].wrapping_mul(0x86);
                c.r[8] = c.r[8].wrapping_add(c.r[10]);
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.lbz(c.r[8].wrapping_add(0x64));
                c.r[8] = c.lbz(c.r[8].wrapping_add(0x65));
                c.r[9] = (c.r[8].rotate_left(8) & 0xff00) | (c.r[9] & 0xffff00ff);
                c.r[8] = c.r[9].rotate_left(0) & 0xffff;
                c.rc(c.r[8]);
                if c.crbit(2) { pc = 0x80160454; continue; }
                pc = 0x80160450;
            }
            0x80160450 => {
                pc = 0x80160644; continue;
                pc = 0x80160454;
            }
            0x80160454 => {
                c.r[8] = c.r[6].rotate_left(0) & 0xff;
                c.r[10] = 0x18;
                c.cmpu(0, c.r[8], 0x18);
                if c.crbit(1) { pc = 0x80160468; continue; }
                pc = 0x80160464;
            }
            0x80160464 => {
                c.r[10] = c.r[8] | c.r[8];
                pc = 0x80160468;
            }
            0x80160468 => {
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[12] = 0x0;
                c.r[9] = c.r[31].wrapping_add(0x4500);
                c.r[8] = 0x80430000;
                c.r[0] = c.r[0].wrapping_add(c.r[7]);
                c.r[26] = c.r[12] | c.r[12];
                c.stb(c.r[9].wrapping_add(c.r[0]), c.r[10]);
                c.r[11] = c.r[10].rotate_left(0) & 0xff;
                c.r[10] = c.r[8].wrapping_add(0x4c50);
                c.r[9] = c.r[5].rotate_left(0) & 0xff;
                c.r[8] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x801604d0; continue;
                pc = 0x80160498;
            }
            0x80160498 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.r[7] = c.r[0].wrapping_add(c.r[26]);
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x122));
                c.cmpu(0, c.r[9], c.r[0]);
                if !c.crbit(2) { pc = 0x801604c8; continue; }
                pc = 0x801604ac;
            }
            0x801604ac => {
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x121));
                c.cmpu(0, c.r[8], c.r[0]);
                if !c.crbit(2) { pc = 0x801604c8; continue; }
                pc = 0x801604b8;
            }
            0x801604b8 => {
                c.stb(c.r[7].wrapping_add(0x1d7), c.r[11]);
                c.r[0] = c.r[26].wrapping_add(0x1d6);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.stb(c.r[7].wrapping_add(c.r[0]), c.r[11]);
                pc = 0x801604c8;
            }
            0x801604c8 => {
                c.r[26] = c.r[26].wrapping_add(0x404);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x801604d0;
            }
            0x801604d0 => {
                c.r[0] = c.lbz(c.r[10].wrapping_add(0x210));
                c.cmpu(0, c.r[12], c.r[0]);
                if c.crbit(0) { pc = 0x80160498; continue; }
                pc = 0x801604dc;
            }
            0x801604dc => {
                pc = 0x80160644; continue;
                pc = 0x801604e0;
            }
            0x801604e0 => {
                c.r[10] = c.r[0].wrapping_mul(0x860);
                c.r[8] = c.r[31].wrapping_add(0x200);
                c.r[9] = c.r[7].wrapping_mul(0x86);
                c.r[8] = c.r[8].wrapping_add(c.r[10]);
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.lbz(c.r[8].wrapping_add(0x64));
                c.r[8] = c.lbz(c.r[8].wrapping_add(0x65));
                c.r[9] = (c.r[8].rotate_left(8) & 0xff00) | (c.r[9] & 0xffff00ff);
                c.r[8] = c.r[9].rotate_left(0) & 0xffff;
                c.rc(c.r[8]);
                if c.crbit(2) { pc = 0x8016050c; continue; }
                pc = 0x80160508;
            }
            0x80160508 => {
                pc = 0x80160644; continue;
                pc = 0x8016050c;
            }
            0x8016050c => {
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[8] = c.r[31].wrapping_add(0x4500);
                c.r[9] = c.r[0].wrapping_add(c.r[7]);
                c.r[11] = c.lbz(c.r[8].wrapping_add(c.r[9]));
                c.cmpu(0, c.r[11], 0x0);
                if c.crbit(2) { pc = 0x8016052c; continue; }
                pc = 0x80160524;
            }
            0x80160524 => {
                c.r[0] = c.r[11].wrapping_add(0xffffffff);
                c.r[11] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x8016052c;
            }
            0x8016052c => {
                c.r[12] = 0x0;
                c.r[7] = 0x80430000;
                c.stb(c.r[8].wrapping_add(c.r[9]), c.r[11]);
                c.r[26] = c.r[12] | c.r[12];
                c.r[10] = c.r[7].wrapping_add(0x4c50);
                c.r[9] = c.r[5].rotate_left(0) & 0xff;
                c.r[8] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x80160584; continue;
                pc = 0x8016054c;
            }
            0x8016054c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.r[7] = c.r[0].wrapping_add(c.r[26]);
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x122));
                c.cmpu(0, c.r[9], c.r[0]);
                if !c.crbit(2) { pc = 0x8016057c; continue; }
                pc = 0x80160560;
            }
            0x80160560 => {
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x121));
                c.cmpu(0, c.r[8], c.r[0]);
                if !c.crbit(2) { pc = 0x8016057c; continue; }
                pc = 0x8016056c;
            }
            0x8016056c => {
                c.stb(c.r[7].wrapping_add(0x1d7), c.r[11]);
                c.r[0] = c.r[26].wrapping_add(0x1d6);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.stb(c.r[7].wrapping_add(c.r[0]), c.r[11]);
                pc = 0x8016057c;
            }
            0x8016057c => {
                c.r[26] = c.r[26].wrapping_add(0x404);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x80160584;
            }
            0x80160584 => {
                c.r[0] = c.lbz(c.r[10].wrapping_add(0x210));
                c.cmpu(0, c.r[12], c.r[0]);
                if c.crbit(0) { pc = 0x8016054c; continue; }
                pc = 0x80160590;
            }
            0x80160590 => {
                pc = 0x80160644; continue;
                pc = 0x80160594;
            }
            0x80160594 => {
                c.r[10] = c.r[0].wrapping_mul(0x860);
                c.r[8] = c.r[31].wrapping_add(0x200);
                c.r[9] = c.r[7].wrapping_mul(0x86);
                c.r[8] = c.r[8].wrapping_add(c.r[10]);
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.lbz(c.r[8].wrapping_add(0x64));
                c.r[8] = c.lbz(c.r[8].wrapping_add(0x65));
                c.r[9] = (c.r[8].rotate_left(8) & 0xff00) | (c.r[9] & 0xffff00ff);
                c.r[8] = c.r[9].rotate_left(0) & 0xffff;
                c.rc(c.r[8]);
                if c.crbit(2) { pc = 0x801605c0; continue; }
                pc = 0x801605bc;
            }
            0x801605bc => {
                pc = 0x80160644; continue;
                pc = 0x801605c0;
            }
            0x801605c0 => {
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[8] = c.r[31].wrapping_add(0x4500);
                c.r[9] = c.r[0].wrapping_add(c.r[7]);
                c.r[11] = c.lbz(c.r[8].wrapping_add(c.r[9]));
                c.cmpu(0, c.r[11], 0x18);
                if !c.crbit(0) { pc = 0x801605e0; continue; }
                pc = 0x801605d8;
            }
            0x801605d8 => {
                c.r[0] = c.r[11].wrapping_add(0x1);
                c.r[11] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x801605e0;
            }
            0x801605e0 => {
                c.r[12] = 0x0;
                c.r[7] = 0x80430000;
                c.stb(c.r[8].wrapping_add(c.r[9]), c.r[11]);
                c.r[26] = c.r[12] | c.r[12];
                c.r[10] = c.r[7].wrapping_add(0x4c50);
                c.r[9] = c.r[5].rotate_left(0) & 0xff;
                c.r[8] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x80160638; continue;
                pc = 0x80160600;
            }
            0x80160600 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.r[7] = c.r[0].wrapping_add(c.r[26]);
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x122));
                c.cmpu(0, c.r[9], c.r[0]);
                if !c.crbit(2) { pc = 0x80160630; continue; }
                pc = 0x80160614;
            }
            0x80160614 => {
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x121));
                c.cmpu(0, c.r[8], c.r[0]);
                if !c.crbit(2) { pc = 0x80160630; continue; }
                pc = 0x80160620;
            }
            0x80160620 => {
                c.stb(c.r[7].wrapping_add(0x1d7), c.r[11]);
                c.r[0] = c.r[26].wrapping_add(0x1d6);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.stb(c.r[7].wrapping_add(c.r[0]), c.r[11]);
                pc = 0x80160630;
            }
            0x80160630 => {
                c.r[26] = c.r[26].wrapping_add(0x404);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x80160638;
            }
            0x80160638 => {
                c.r[0] = c.lbz(c.r[10].wrapping_add(0x210));
                c.cmpu(0, c.r[12], c.r[0]);
                if c.crbit(0) { pc = 0x80160600; continue; }
                pc = 0x80160644;
            }
            0x80160644 => {
                c.r[29] = c.r[5].rotate_left(0) & 0xff;
                c.r[30] = c.r[4].rotate_left(0) & 0xff;
                c.r[7] = c.r[29].wrapping_mul(0x860);
                c.r[4] = 0x80430000;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[6] = c.r[6].rotate_left(0) & 0x7f;
                c.r[3] = c.r[30].wrapping_mul(0x86);
                c.r[5] = c.r[31].wrapping_add(0x200);
                c.r[26] = c.r[4].wrapping_add(0x4c50);
                c.r[28] = 0x0;
                c.r[3] = c.r[7].wrapping_add(c.r[3]);
                c.r[25] = 0x0;
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.r[27] = 0x1fff;
                c.stb(c.r[5].wrapping_add(c.r[0]), c.r[6]);
                pc = 0x801606bc; continue;
                pc = 0x80160684;
            }
            0x80160684 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.r[3] = c.r[0].wrapping_add(c.r[25]);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x122));
                c.cmpu(0, c.r[29], c.r[0]);
                if !c.crbit(2) { pc = 0x801606b4; continue; }
                pc = 0x80160698;
            }
            0x80160698 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x121));
                c.cmpu(0, c.r[30], c.r[0]);
                if !c.crbit(2) { pc = 0x801606b4; continue; }
                pc = 0x801606a4;
            }
            0x801606a4 => {
                c.stw(c.r[3].wrapping_add(0x214), c.r[27]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.r[3] = c.r[0].wrapping_add(c.r[25]);
                c.lr = 0x801606b4; f_8014c07c(c);
                pc = 0x801606b4;
            }
            0x801606b4 => {
                c.r[25] = c.r[25].wrapping_add(0x404);
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x801606bc;
            }
            0x801606bc => {
                c.r[0] = c.lbz(c.r[26].wrapping_add(0x210));
                c.cmpu(0, c.r[28], c.r[0]);
                if c.crbit(0) { pc = 0x80160684; continue; }
                pc = 0x801606c8;
            }
            0x801606c8 => {
                c.r[4] = c.r[29].rotate_left(6) & 0xffffffc0;
                c.r[0] = c.r[30].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[5] = 0xff;
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[5]);
                pc = 0x801609b0; continue;
                pc = 0x801606e4;
            }
            0x801606e4 => {
                c.r[8] = c.r[3].rotate_left(0) & 0xff;
                c.cmps(0, c.r[8], 0x26);
                if c.crbit(2) { pc = 0x80160934; continue; }
                pc = 0x801606f0;
            }
            0x801606f0 => {
                if !c.crbit(0) { pc = 0x80160700; continue; }
                pc = 0x801606f4;
            }
            0x801606f4 => {
                c.cmps(0, c.r[8], 0x6);
                if c.crbit(2) { pc = 0x80160718; continue; }
                pc = 0x801606fc;
            }
            0x801606fc => {
                pc = 0x80160934; continue;
                pc = 0x80160700;
            }
            0x80160700 => {
                c.cmps(0, c.r[8], 0x61);
                if c.crbit(2) { pc = 0x80160884; continue; }
                pc = 0x80160708;
            }
            0x80160708 => {
                if !c.crbit(0) { pc = 0x80160934; continue; }
                pc = 0x8016070c;
            }
            0x8016070c => {
                c.cmps(0, c.r[8], 0x60);
                if !c.crbit(0) { pc = 0x801607d0; continue; }
                pc = 0x80160714;
            }
            0x80160714 => {
                pc = 0x80160934; continue;
                pc = 0x80160718;
            }
            0x80160718 => {
                c.r[10] = c.r[0].wrapping_mul(0x860);
                c.r[8] = c.r[31].wrapping_add(0x200);
                c.r[9] = c.r[7].wrapping_mul(0x86);
                c.r[8] = c.r[8].wrapping_add(c.r[10]);
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.lbz(c.r[8].wrapping_add(0x64));
                c.r[8] = c.lbz(c.r[8].wrapping_add(0x65));
                c.r[9] = (c.r[8].rotate_left(8) & 0xff00) | (c.r[9] & 0xffff00ff);
                c.r[8] = c.r[9].rotate_left(0) & 0xffff;
                c.rc(c.r[8]);
                if c.crbit(2) { pc = 0x80160744; continue; }
                pc = 0x80160740;
            }
            0x80160740 => {
                pc = 0x80160934; continue;
                pc = 0x80160744;
            }
            0x80160744 => {
                c.r[8] = c.r[6].rotate_left(0) & 0xff;
                c.r[10] = 0x18;
                c.cmpu(0, c.r[8], 0x18);
                if c.crbit(1) { pc = 0x80160758; continue; }
                pc = 0x80160754;
            }
            0x80160754 => {
                c.r[10] = c.r[8] | c.r[8];
                pc = 0x80160758;
            }
            0x80160758 => {
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[12] = 0x0;
                c.r[9] = c.r[31].wrapping_add(0x4500);
                c.r[8] = 0x80430000;
                c.r[0] = c.r[0].wrapping_add(c.r[7]);
                c.r[25] = c.r[12] | c.r[12];
                c.stb(c.r[9].wrapping_add(c.r[0]), c.r[10]);
                c.r[11] = c.r[10].rotate_left(0) & 0xff;
                c.r[10] = c.r[8].wrapping_add(0x4c50);
                c.r[9] = c.r[5].rotate_left(0) & 0xff;
                c.r[8] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x801607c0; continue;
                pc = 0x80160788;
            }
            0x80160788 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.r[7] = c.r[0].wrapping_add(c.r[25]);
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x122));
                c.cmpu(0, c.r[9], c.r[0]);
                if !c.crbit(2) { pc = 0x801607b8; continue; }
                pc = 0x8016079c;
            }
            0x8016079c => {
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x121));
                c.cmpu(0, c.r[8], c.r[0]);
                if !c.crbit(2) { pc = 0x801607b8; continue; }
                pc = 0x801607a8;
            }
            0x801607a8 => {
                c.stb(c.r[7].wrapping_add(0x1d7), c.r[11]);
                c.r[0] = c.r[25].wrapping_add(0x1d6);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.stb(c.r[7].wrapping_add(c.r[0]), c.r[11]);
                pc = 0x801607b8;
            }
            0x801607b8 => {
                c.r[25] = c.r[25].wrapping_add(0x404);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x801607c0;
            }
            0x801607c0 => {
                c.r[0] = c.lbz(c.r[10].wrapping_add(0x210));
                c.cmpu(0, c.r[12], c.r[0]);
                if c.crbit(0) { pc = 0x80160788; continue; }
                pc = 0x801607cc;
            }
            0x801607cc => {
                pc = 0x80160934; continue;
                pc = 0x801607d0;
            }
            0x801607d0 => {
                c.r[10] = c.r[0].wrapping_mul(0x860);
                c.r[8] = c.r[31].wrapping_add(0x200);
                c.r[9] = c.r[7].wrapping_mul(0x86);
                c.r[8] = c.r[8].wrapping_add(c.r[10]);
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.lbz(c.r[8].wrapping_add(0x64));
                c.r[8] = c.lbz(c.r[8].wrapping_add(0x65));
                c.r[9] = (c.r[8].rotate_left(8) & 0xff00) | (c.r[9] & 0xffff00ff);
                c.r[8] = c.r[9].rotate_left(0) & 0xffff;
                c.rc(c.r[8]);
                if c.crbit(2) { pc = 0x801607fc; continue; }
                pc = 0x801607f8;
            }
            0x801607f8 => {
                pc = 0x80160934; continue;
                pc = 0x801607fc;
            }
            0x801607fc => {
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[8] = c.r[31].wrapping_add(0x4500);
                c.r[9] = c.r[0].wrapping_add(c.r[7]);
                c.r[11] = c.lbz(c.r[8].wrapping_add(c.r[9]));
                c.cmpu(0, c.r[11], 0x0);
                if c.crbit(2) { pc = 0x8016081c; continue; }
                pc = 0x80160814;
            }
            0x80160814 => {
                c.r[0] = c.r[11].wrapping_add(0xffffffff);
                c.r[11] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x8016081c;
            }
            0x8016081c => {
                c.r[12] = 0x0;
                c.r[7] = 0x80430000;
                c.stb(c.r[8].wrapping_add(c.r[9]), c.r[11]);
                c.r[25] = c.r[12] | c.r[12];
                c.r[10] = c.r[7].wrapping_add(0x4c50);
                c.r[9] = c.r[5].rotate_left(0) & 0xff;
                c.r[8] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x80160874; continue;
                pc = 0x8016083c;
            }
            0x8016083c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.r[7] = c.r[0].wrapping_add(c.r[25]);
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x122));
                c.cmpu(0, c.r[9], c.r[0]);
                if !c.crbit(2) { pc = 0x8016086c; continue; }
                pc = 0x80160850;
            }
            0x80160850 => {
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x121));
                c.cmpu(0, c.r[8], c.r[0]);
                if !c.crbit(2) { pc = 0x8016086c; continue; }
                pc = 0x8016085c;
            }
            0x8016085c => {
                c.stb(c.r[7].wrapping_add(0x1d7), c.r[11]);
                c.r[0] = c.r[25].wrapping_add(0x1d6);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.stb(c.r[7].wrapping_add(c.r[0]), c.r[11]);
                pc = 0x8016086c;
            }
            0x8016086c => {
                c.r[25] = c.r[25].wrapping_add(0x404);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x80160874;
            }
            0x80160874 => {
                c.r[0] = c.lbz(c.r[10].wrapping_add(0x210));
                c.cmpu(0, c.r[12], c.r[0]);
                if c.crbit(0) { pc = 0x8016083c; continue; }
                pc = 0x80160880;
            }
            0x80160880 => {
                pc = 0x80160934; continue;
                pc = 0x80160884;
            }
            0x80160884 => {
                c.r[10] = c.r[0].wrapping_mul(0x860);
                c.r[8] = c.r[31].wrapping_add(0x200);
                c.r[9] = c.r[7].wrapping_mul(0x86);
                c.r[8] = c.r[8].wrapping_add(c.r[10]);
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.lbz(c.r[8].wrapping_add(0x64));
                c.r[8] = c.lbz(c.r[8].wrapping_add(0x65));
                c.r[9] = (c.r[8].rotate_left(8) & 0xff00) | (c.r[9] & 0xffff00ff);
                c.r[8] = c.r[9].rotate_left(0) & 0xffff;
                c.rc(c.r[8]);
                if c.crbit(2) { pc = 0x801608b0; continue; }
                pc = 0x801608ac;
            }
            0x801608ac => {
                pc = 0x80160934; continue;
                pc = 0x801608b0;
            }
            0x801608b0 => {
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[8] = c.r[31].wrapping_add(0x4500);
                c.r[9] = c.r[0].wrapping_add(c.r[7]);
                c.r[11] = c.lbz(c.r[8].wrapping_add(c.r[9]));
                c.cmpu(0, c.r[11], 0x18);
                if !c.crbit(0) { pc = 0x801608d0; continue; }
                pc = 0x801608c8;
            }
            0x801608c8 => {
                c.r[0] = c.r[11].wrapping_add(0x1);
                c.r[11] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x801608d0;
            }
            0x801608d0 => {
                c.r[12] = 0x0;
                c.r[7] = 0x80430000;
                c.stb(c.r[8].wrapping_add(c.r[9]), c.r[11]);
                c.r[25] = c.r[12] | c.r[12];
                c.r[10] = c.r[7].wrapping_add(0x4c50);
                c.r[9] = c.r[5].rotate_left(0) & 0xff;
                c.r[8] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x80160928; continue;
                pc = 0x801608f0;
            }
            0x801608f0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.r[7] = c.r[0].wrapping_add(c.r[25]);
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x122));
                c.cmpu(0, c.r[9], c.r[0]);
                if !c.crbit(2) { pc = 0x80160920; continue; }
                pc = 0x80160904;
            }
            0x80160904 => {
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x121));
                c.cmpu(0, c.r[8], c.r[0]);
                if !c.crbit(2) { pc = 0x80160920; continue; }
                pc = 0x80160910;
            }
            0x80160910 => {
                c.stb(c.r[7].wrapping_add(0x1d7), c.r[11]);
                c.r[0] = c.r[25].wrapping_add(0x1d6);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.stb(c.r[7].wrapping_add(c.r[0]), c.r[11]);
                pc = 0x80160920;
            }
            0x80160920 => {
                c.r[25] = c.r[25].wrapping_add(0x404);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x80160928;
            }
            0x80160928 => {
                c.r[0] = c.lbz(c.r[10].wrapping_add(0x210));
                c.cmpu(0, c.r[12], c.r[0]);
                if c.crbit(0) { pc = 0x801608f0; continue; }
                pc = 0x80160934;
            }
            0x80160934 => {
                c.r[27] = c.r[4].rotate_left(0) & 0xff;
                c.r[29] = 0x0;
                c.r[7] = c.r[27].wrapping_mul(0x86);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x80430000;
                c.r[6] = c.r[6].rotate_left(0) & 0x7f;
                c.r[4] = c.r[31].wrapping_add(0x4580);
                c.r[25] = c.r[29] | c.r[29];
                c.r[0] = c.r[7].wrapping_add(c.r[0]);
                c.r[31] = c.r[3].wrapping_add(0x4c50);
                c.stb(c.r[4].wrapping_add(c.r[0]), c.r[6]);
                c.r[30] = c.r[5].rotate_left(0) & 0xff;
                c.r[28] = 0x1fff;
                pc = 0x801609a4; continue;
                pc = 0x8016096c;
            }
            0x8016096c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.r[3] = c.r[0].wrapping_add(c.r[25]);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x122));
                c.cmpu(0, c.r[30], c.r[0]);
                if !c.crbit(2) { pc = 0x8016099c; continue; }
                pc = 0x80160980;
            }
            0x80160980 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x121));
                c.cmpu(0, c.r[27], c.r[0]);
                if !c.crbit(2) { pc = 0x8016099c; continue; }
                pc = 0x8016098c;
            }
            0x8016098c => {
                c.stw(c.r[3].wrapping_add(0x214), c.r[28]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa728));
                c.r[3] = c.r[0].wrapping_add(c.r[25]);
                c.lr = 0x8016099c; f_8014c07c(c);
                pc = 0x8016099c;
            }
            0x8016099c => {
                c.r[25] = c.r[25].wrapping_add(0x404);
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801609a4;
            }
            0x801609a4 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x210));
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x8016096c; continue; }
                pc = 0x801609b0;
            }
            0x801609b0 => {
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x801609b8; f_800c481c(c);
                pc = 0x801609b8;
            }
            0x801609b8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_801603c0: bad pc {:#x}", pc),
        }
    }
}

/// inpSetMidiCtrl14 (0x801609c8, 292 bytes)
#[inline(never)]
pub fn f_801609c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801609c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801609c8); f_801609c8_b(c, 0x801609c8); crate::prof::exit(c, 0x801609c8); return; }
    }
    f_801609c8_b(c, 0x801609c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801609c8_at(c: &mut Cpu, pc: u32) { f_801609c8_b(c, pc) }
#[inline(always)]
fn f_801609c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801609c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801609c8, pc); }
        match pc {
            0x801609c8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0xff);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[4] | c.r[4];
                if c.crbit(2) { pc = 0x80160acc; continue; }
                pc = 0x801609f8;
            }
            0x801609f8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x40);
                if !c.crbit(0) { pc = 0x80160a34; continue; }
                pc = 0x80160a04;
            }
            0x80160a04 => {
                c.r[30] = c.r[3].rotate_left(0) & 0x1f;
                c.r[31] = c.r[6].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[6].rotate_left(25) & 0xff;
                c.lr = 0x80160a18; f_801603c0(c);
                pc = 0x80160a18;
            }
            0x80160a18 => {
                c.r[0] = c.r[30].wrapping_add(0x20);
                c.r[6] = c.r[31].rotate_left(0) & 0x7f;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x80160a30; f_801603c0(c);
                pc = 0x80160a30;
            }
            0x80160a30 => {
                pc = 0x80160acc; continue;
                pc = 0x80160a34;
            }
            0x80160a34 => {
                c.cmpu(0, c.r[0], 0x80);
                if c.crbit(2) { pc = 0x80160a44; continue; }
                pc = 0x80160a3c;
            }
            0x80160a3c => {
                c.cmpu(0, c.r[0], 0x81);
                if !c.crbit(2) { pc = 0x80160a7c; continue; }
                pc = 0x80160a44;
            }
            0x80160a44 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xfe;
                c.r[30] = c.r[6].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[6].rotate_left(25) & 0xff;
                c.lr = 0x80160a60; f_801603c0(c);
                pc = 0x80160a60;
            }
            0x80160a60 => {
                c.r[0] = c.r[31].wrapping_add(0x1);
                c.r[6] = c.r[30].rotate_left(0) & 0x7f;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x80160a78; f_801603c0(c);
                pc = 0x80160a78;
            }
            0x80160a78 => {
                pc = 0x80160acc; continue;
                pc = 0x80160a7c;
            }
            0x80160a7c => {
                c.cmpu(0, c.r[0], 0x84);
                if c.crbit(2) { pc = 0x80160a8c; continue; }
                pc = 0x80160a84;
            }
            0x80160a84 => {
                c.cmpu(0, c.r[0], 0x85);
                if !c.crbit(2) { pc = 0x80160ac4; continue; }
                pc = 0x80160a8c;
            }
            0x80160a8c => {
                c.r[31] = c.r[3].rotate_left(0) & 0xfe;
                c.r[30] = c.r[6].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[6].rotate_left(25) & 0xff;
                c.lr = 0x80160aa8; f_801603c0(c);
                pc = 0x80160aa8;
            }
            0x80160aa8 => {
                c.r[0] = c.r[31].wrapping_add(0x1);
                c.r[6] = c.r[30].rotate_left(0) & 0x7f;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x80160ac0; f_801603c0(c);
                pc = 0x80160ac0;
            }
            0x80160ac0 => {
                pc = 0x80160acc; continue;
                pc = 0x80160ac4;
            }
            0x80160ac4 => {
                c.r[6] = c.r[6].rotate_left(25) & 0xff;
                c.lr = 0x80160acc; f_801603c0(c);
                pc = 0x80160acc;
            }
            0x80160acc => {
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
            _ => panic!("f_801609c8: bad pc {:#x}", pc),
        }
    }
}

/// inpResetMidiCtrl (0x80160aec, 240 bytes)
#[inline(never)]
pub fn f_80160aec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80160aec) { return; }
        if c.prof { crate::prof::enter(c, 0x80160aec); f_80160aec_b(c, 0x80160aec); crate::prof::exit(c, 0x80160aec); return; }
    }
    f_80160aec_b(c, 0x80160aec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160aec_at(c: &mut Cpu, pc: u32) { f_80160aec_b(c, pc) }
#[inline(always)]
fn f_80160aec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80160aec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80160aec, pc); }
        match pc {
            0x80160aec => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x80270000;
                c.cmpu(0, c.r[5], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = c.r[6].wrapping_add(0x33c0);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x80160b20; continue; }
                pc = 0x80160b18;
            }
            0x80160b18 => {
                c.r[3] = 0x80270000;
                c.r[6] = c.r[3].wrapping_add(0x3338);
                pc = 0x80160b20;
            }
            0x80160b20 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[4], 0xff);
                if c.crbit(2) { pc = 0x80160b4c; continue; }
                pc = 0x80160b2c;
            }
            0x80160b2c => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.r[3] = 0x80450000;
                c.r[4] = c.r[4].wrapping_mul(0x860);
                c.r[3] = c.r[3].wrapping_add(0xffff9590);
                c.r[0] = c.r[0].wrapping_mul(0x86);
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x80160b60; continue;
                pc = 0x80160b4c;
            }
            0x80160b4c => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.r[3] = 0x80450000;
                c.r[4] = c.r[0].wrapping_mul(0x86);
                c.r[0] = c.r[3].wrapping_add(0xffffd910);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80160b60;
            }
            0x80160b60 => {
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x80160b78; continue; }
                pc = 0x80160b68;
            }
            0x80160b68 => {
                c.r[4] = c.r[6] | c.r[6];
                c.r[5] = 0x86;
                c.lr = 0x80160b74; f_800054f4(c);
                pc = 0x80160b74;
            }
            0x80160b74 => {
                pc = 0x80160bb4; continue;
                pc = 0x80160b78;
            }
            0x80160b78 => {
                c.r[0] = 0x43;
                c.r[4] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x80160b84;
            }
            0x80160b84 => {
                c.r[0] = c.lbz(c.r[6]);
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x80160b94; continue; }
                pc = 0x80160b90;
            }
            0x80160b90 => {
                c.stb(c.r[3], c.r[0]);
                pc = 0x80160b94;
            }
            0x80160b94 => {
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x80160ba4; continue; }
                pc = 0x80160ba0;
            }
            0x80160ba0 => {
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                pc = 0x80160ba4;
            }
            0x80160ba4 => {
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[3] = c.r[3].wrapping_add(0x2);
                c.r[6] = c.r[6].wrapping_add(0x2);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80160b84; continue; }
                pc = 0x80160bb4;
            }
            0x80160bb4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0xff;
                c.lr = 0x80160bc4; f_801610bc(c);
                pc = 0x80160bc4;
            }
            0x80160bc4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80160aec: bad pc {:#x}", pc),
        }
    }
}

/// inpGetMidiCtrl (0x80160bdc, 708 bytes)
#[inline(never)]
pub fn f_80160bdc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80160bdc) { return; }
        if c.prof { crate::prof::enter(c, 0x80160bdc); f_80160bdc_b(c, 0x80160bdc); crate::prof::exit(c, 0x80160bdc); return; }
    }
    f_80160bdc_b(c, 0x80160bdc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160bdc_at(c: &mut Cpu, pc: u32) { f_80160bdc_b(c, pc) }
#[inline(always)]
fn f_80160bdc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80160bdc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80160bdc, pc); }
        match pc {
            0x80160bdc => {
                c.r[8] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[8], 0xff);
                if c.crbit(2) { pc = 0x80160e98; continue; }
                pc = 0x80160be8;
            }
            0x80160be8 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x80160d60; continue; }
                pc = 0x80160bf4;
            }
            0x80160bf4 => {
                c.r[7] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[7], 0x40);
                if !c.crbit(0) { pc = 0x80160c38; continue; }
                pc = 0x80160c00;
            }
            0x80160c00 => {
                c.r[5] = c.r[0].wrapping_mul(0x860);
                c.r[4] = 0x80450000;
                c.r[6] = c.r[3].rotate_left(0) & 0x1f;
                c.r[0] = c.r[4].wrapping_add(0xffff9590);
                c.r[3] = c.r[8].wrapping_mul(0x86);
                c.r[0] = c.r[0].wrapping_add(c.r[5]);
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                c.r[3] = c.r[4].wrapping_add(c.r[6]);
                c.r[4] = c.lbz(c.r[4].wrapping_add(c.r[6]));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x20));
                c.r[3] = c.r[4].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { return; }
                pc = 0x80160c38;
            }
            0x80160c38 => {
                c.cmpu(0, c.r[7], 0x46);
                if !c.crbit(0) { pc = 0x80160c78; continue; }
                pc = 0x80160c40;
            }
            0x80160c40 => {
                c.r[6] = c.r[0].wrapping_mul(0x860);
                c.r[4] = 0x80450000;
                c.r[3] = 0x40;
                c.r[5] = c.r[4].wrapping_add(0xffff9590);
                c.r[4] = c.r[8].wrapping_mul(0x86);
                c.r[0] = 0x3fff;
                c.r[5] = c.r[5].wrapping_add(c.r[6]);
                c.r[4] = c.r[5].wrapping_add(c.r[4]);
                c.r[4] = c.lbz(c.r[4].wrapping_add(c.r[7]));
                c.r[3] = c.addc(!c.r[3], c.r[4], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[3], ci); }
                c.r[0] = c.r[0] & !c.r[3];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { return; }
                pc = 0x80160c78;
            }
            0x80160c78 => {
                c.cmpu(0, c.r[7], 0x60);
                if c.crbit(0) { pc = 0x80160c90; continue; }
                pc = 0x80160c80;
            }
            0x80160c80 => {
                c.cmpu(0, c.r[7], 0x66);
                if !c.crbit(0) { pc = 0x80160c90; continue; }
                pc = 0x80160c88;
            }
            0x80160c88 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80160c90;
            }
            0x80160c90 => {
                c.r[6] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[6], 0x80);
                if c.crbit(2) { pc = 0x80160ca4; continue; }
                pc = 0x80160c9c;
            }
            0x80160c9c => {
                c.cmpu(0, c.r[6], 0x81);
                if !c.crbit(2) { pc = 0x80160ce4; continue; }
                pc = 0x80160ca4;
            }
            0x80160ca4 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                c.r[5] = c.r[4].rotate_left(0) & 0xff;
                c.r[6] = c.r[0].wrapping_mul(0x860);
                c.r[4] = 0x80450000;
                c.r[7] = c.r[3].rotate_left(0) & 0xfe;
                c.r[0] = c.r[4].wrapping_add(0xffff9590);
                c.r[3] = c.r[5].wrapping_mul(0x86);
                c.r[0] = c.r[0].wrapping_add(c.r[6]);
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                c.r[3] = c.r[4].wrapping_add(c.r[7]);
                c.r[4] = c.lbz(c.r[4].wrapping_add(c.r[7]));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1));
                c.r[3] = c.r[4].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { return; }
                pc = 0x80160ce4;
            }
            0x80160ce4 => {
                c.cmpu(0, c.r[6], 0x84);
                if c.crbit(2) { pc = 0x80160cf4; continue; }
                pc = 0x80160cec;
            }
            0x80160cec => {
                c.cmpu(0, c.r[6], 0x85);
                if !c.crbit(2) { pc = 0x80160d34; continue; }
                pc = 0x80160cf4;
            }
            0x80160cf4 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                c.r[5] = c.r[4].rotate_left(0) & 0xff;
                c.r[6] = c.r[0].wrapping_mul(0x860);
                c.r[4] = 0x80450000;
                c.r[7] = c.r[3].rotate_left(0) & 0xfe;
                c.r[0] = c.r[4].wrapping_add(0xffff9590);
                c.r[3] = c.r[5].wrapping_mul(0x86);
                c.r[0] = c.r[0].wrapping_add(c.r[6]);
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                c.r[3] = c.r[4].wrapping_add(c.r[7]);
                c.r[4] = c.lbz(c.r[4].wrapping_add(c.r[7]));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1));
                c.r[3] = c.r[4].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { return; }
                pc = 0x80160d34;
            }
            0x80160d34 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[4] = c.r[3].wrapping_mul(0x860);
                c.r[3] = 0x80450000;
                c.r[3] = c.r[3].wrapping_add(0xffff9590);
                c.r[0] = c.r[0].wrapping_mul(0x86);
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz(c.r[3].wrapping_add(c.r[6]));
                c.r[3] = c.r[0].rotate_left(7) & 0xff80;
                if true { return; }
                pc = 0x80160d60;
            }
            0x80160d60 => {
                c.r[6] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[6], 0x40);
                if !c.crbit(0) { pc = 0x80160d9c; continue; }
                pc = 0x80160d6c;
            }
            0x80160d6c => {
                c.r[6] = c.r[8].wrapping_mul(0x86);
                c.r[4] = 0x80450000;
                c.r[5] = c.r[3].rotate_left(0) & 0x1f;
                c.r[0] = c.r[4].wrapping_add(0xffffd910);
                c.r[4] = c.r[0].wrapping_add(c.r[6]);
                c.r[3] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.lbz(c.r[4].wrapping_add(c.r[5]));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x20));
                c.r[3] = c.r[4].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { return; }
                pc = 0x80160d9c;
            }
            0x80160d9c => {
                c.cmpu(0, c.r[6], 0x46);
                if !c.crbit(0) { pc = 0x80160dd4; continue; }
                pc = 0x80160da4;
            }
            0x80160da4 => {
                c.r[5] = c.r[8].wrapping_mul(0x86);
                c.r[4] = 0x80450000;
                c.r[3] = 0x40;
                c.r[0] = c.r[4].wrapping_add(0xffffd910);
                c.r[4] = c.r[0].wrapping_add(c.r[5]);
                c.r[0] = 0x3fff;
                c.r[4] = c.lbz(c.r[4].wrapping_add(c.r[6]));
                c.r[3] = c.addc(!c.r[3], c.r[4], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[3], ci); }
                c.r[0] = c.r[0] & !c.r[3];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { return; }
                pc = 0x80160dd4;
            }
            0x80160dd4 => {
                c.cmpu(0, c.r[6], 0x60);
                if c.crbit(0) { pc = 0x80160dec; continue; }
                pc = 0x80160ddc;
            }
            0x80160ddc => {
                c.cmpu(0, c.r[6], 0x66);
                if !c.crbit(0) { pc = 0x80160dec; continue; }
                pc = 0x80160de4;
            }
            0x80160de4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80160dec;
            }
            0x80160dec => {
                c.r[5] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[5], 0x80);
                if c.crbit(2) { pc = 0x80160e00; continue; }
                pc = 0x80160df8;
            }
            0x80160df8 => {
                c.cmpu(0, c.r[5], 0x81);
                if !c.crbit(2) { pc = 0x80160e34; continue; }
                pc = 0x80160e00;
            }
            0x80160e00 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[4] = 0x80450000;
                c.r[6] = c.r[0].wrapping_mul(0x86);
                c.r[5] = c.r[3].rotate_left(0) & 0xfe;
                c.r[0] = c.r[4].wrapping_add(0xffffd910);
                c.r[4] = c.r[0].wrapping_add(c.r[6]);
                c.r[3] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.lbz(c.r[4].wrapping_add(c.r[5]));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1));
                c.r[3] = c.r[4].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { return; }
                pc = 0x80160e34;
            }
            0x80160e34 => {
                c.cmpu(0, c.r[5], 0x84);
                if c.crbit(2) { pc = 0x80160e44; continue; }
                pc = 0x80160e3c;
            }
            0x80160e3c => {
                c.cmpu(0, c.r[5], 0x85);
                if !c.crbit(2) { pc = 0x80160e78; continue; }
                pc = 0x80160e44;
            }
            0x80160e44 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[4] = 0x80450000;
                c.r[6] = c.r[0].wrapping_mul(0x86);
                c.r[5] = c.r[3].rotate_left(0) & 0xfe;
                c.r[0] = c.r[4].wrapping_add(0xffffd910);
                c.r[4] = c.r[0].wrapping_add(c.r[6]);
                c.r[3] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.lbz(c.r[4].wrapping_add(c.r[5]));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1));
                c.r[3] = c.r[4].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { return; }
                pc = 0x80160e78;
            }
            0x80160e78 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[3] = 0x80450000;
                c.r[4] = c.r[0].wrapping_mul(0x86);
                c.r[0] = c.r[3].wrapping_add(0xffffd910);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.lbz(c.r[3].wrapping_add(c.r[5]));
                c.r[3] = c.r[0].rotate_left(7) & 0xff80;
                if true { return; }
                pc = 0x80160e98;
            }
            0x80160e98 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80160bdc: bad pc {:#x}", pc),
        }
    }
}

/// fn_80160EA0 (0x80160ea0, 52 bytes)
#[inline(never)]
pub fn f_80160ea0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80160ea0) { return; }
        if c.prof { crate::prof::enter(c, 0x80160ea0); f_80160ea0_b(c, 0x80160ea0); crate::prof::exit(c, 0x80160ea0); return; }
    }
    f_80160ea0_b(c, 0x80160ea0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160ea0_at(c: &mut Cpu, pc: u32) { f_80160ea0_b(c, pc) }
#[inline(always)]
fn f_80160ea0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80160ea0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80160ea0, pc); }
        match pc {
            0x80160ea0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[5] = 0x80450000;
                c.cmpu(0, c.r[0], 0xff);
                c.r[6] = c.r[4].rotate_left(4) & 0xff0;
                c.r[0] = c.r[5].wrapping_add(0xffffd890);
                c.r[4] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.r[0].wrapping_add(c.r[6]);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if !c.crbit(2) { return; }
                pc = 0x80160ec4;
            }
            0x80160ec4 => {
                c.r[3] = 0x80450000;
                c.r[0] = c.r[3].wrapping_add(0xfffffa90);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80160ea0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80160ED4 (0x80160ed4, 60 bytes)
#[inline(never)]
pub fn f_80160ed4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80160ed4) { return; }
        if c.prof { crate::prof::enter(c, 0x80160ed4); f_80160ed4_b(c, 0x80160ed4); crate::prof::exit(c, 0x80160ed4); return; }
    }
    f_80160ed4_b(c, 0x80160ed4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160ed4_at(c: &mut Cpu, pc: u32) { f_80160ed4_b(c, pc) }
#[inline(always)]
fn f_80160ed4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80160ed4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80160ed4, pc); }
        match pc {
            0x80160ed4 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[5] = 0x80450000;
                c.cmpu(0, c.r[0], 0xff);
                c.r[6] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.r[5].wrapping_add(0xfffffa90);
                c.r[5] = 0x2;
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                if c.crbit(2) { pc = 0x80160f08; continue; }
                pc = 0x80160ef4;
            }
            0x80160ef4 => {
                c.r[3] = 0x80450000;
                c.r[4] = c.r[4].rotate_left(4) & 0xff0;
                c.r[0] = c.r[3].wrapping_add(0xffffd890);
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                pc = 0x80160f08;
            }
            0x80160f08 => {
                c.stb(c.r[3], c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_80160ed4: bad pc {:#x}", pc),
        }
    }
}

/// inpAddCtrl (0x80160f10, 152 bytes)
#[inline(never)]
pub fn f_80160f10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80160f10) { return; }
        if c.prof { crate::prof::enter(c, 0x80160f10); f_80160f10_b(c, 0x80160f10); crate::prof::exit(c, 0x80160f10); return; }
    }
    f_80160f10_b(c, 0x80160f10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160f10_at(c: &mut Cpu, pc: u32) { f_80160f10_b(c, pc) }
#[inline(always)]
fn f_80160f10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80160f10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80160f10, pc); }
        match pc {
            0x80160f10 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x20);
                c.lr = 0x80160f24; f_800c47d8(c);
                pc = 0x80160f24;
            }
            0x80160f24 => {
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.r[29] = c.r[6] | c.r[6];
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[5] | c.r[5];
                c.r[30] = c.r[7] | c.r[7];
                if !c.crbit(2) { pc = 0x80160f44; continue; }
                pc = 0x80160f3c;
            }
            0x80160f3c => {
                c.r[0] = 0x0;
                c.stb(c.r[27].wrapping_add(0x22), c.r[0]);
                pc = 0x80160f44;
            }
            0x80160f44 => {
                c.r[31] = c.lbz(c.r[27].wrapping_add(0x22));
                c.cmpu(0, c.r[31], 0x4);
                if !c.crbit(0) { pc = 0x80160f90; continue; }
                pc = 0x80160f50;
            }
            0x80160f50 => {
                c.r[0] = c.r[31].wrapping_add(0x1);
                c.cmpu(0, c.r[30], 0x0);
                c.stb(c.r[27].wrapping_add(0x22), c.r[0]);
                if !c.crbit(2) { pc = 0x80160f6c; continue; }
                pc = 0x80160f60;
            }
            0x80160f60 => {
                c.r[3] = c.r[4] | c.r[4];
                c.lr = 0x80160f68; f_80161d20(c);
                pc = 0x80160f68;
            }
            0x80160f68 => {
                c.r[4] = c.r[3] | c.r[3];
                pc = 0x80160f6c;
            }
            0x80160f6c => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x80160f7c; continue; }
                pc = 0x80160f74;
            }
            0x80160f74 => {
                c.r[0] = c.r[29] | 0x10;
                c.r[29] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x80160f7c;
            }
            0x80160f7c => {
                c.r[0] = c.r[31].rotate_left(3) & 0x7f8;
                c.stb(c.r[27].wrapping_add(c.r[0]), c.r[4]);
                c.r[3] = c.r[27].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x1), c.r[29]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[28]);
                pc = 0x80160f90;
            }
            0x80160f90 => {
                c.r[11] = c.r[1].wrapping_add(0x20);
                c.lr = 0x80160f98; f_800c4824(c);
                pc = 0x80160f98;
            }
            0x80160f98 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80160f10: bad pc {:#x}", pc),
        }
    }
}

/// inpFXCopyCtrl (0x80160fa8, 276 bytes)
#[inline(never)]
pub fn f_80160fa8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80160fa8) { return; }
        if c.prof { crate::prof::enter(c, 0x80160fa8); f_80160fa8_b(c, 0x80160fa8); crate::prof::exit(c, 0x80160fa8); return; }
    }
    f_80160fa8_b(c, 0x80160fa8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160fa8_at(c: &mut Cpu, pc: u32) { f_80160fa8_b(c, pc) }
#[inline(always)]
fn f_80160fa8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80160fa8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80160fa8, pc); }
        match pc {
            0x80160fa8 => {
                c.r[6] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.lwz(c.r[4].wrapping_add(0xf4));
                c.r[0] = c.lwz(c.r[5].wrapping_add(0xf4));
                c.cmpu(0, c.r[6], 0x40);
                c.r[7] = c.r[4].rotate_left(0) & 0xff;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                if !c.crbit(0) { pc = 0x80161000; continue; }
                pc = 0x80160fc4;
            }
            0x80160fc4 => {
                c.r[6] = c.r[0].wrapping_mul(0x86);
                c.r[4] = 0x80450000;
                c.r[5] = c.r[3].rotate_left(0) & 0x1f;
                c.r[4] = c.r[4].wrapping_add(0xffffd910);
                c.r[7] = c.r[7].wrapping_mul(0x86);
                c.r[3] = c.r[4].wrapping_add(c.r[6]);
                c.r[0] = c.lbz(c.r[3].wrapping_add(c.r[5]));
                c.r[8] = c.r[4].wrapping_add(0x20);
                c.r[4] = c.r[4].wrapping_add(c.r[7]);
                c.r[3] = c.r[6].wrapping_add(c.r[5]);
                c.stb(c.r[4].wrapping_add(c.r[5]), c.r[0]);
                c.r[0] = c.r[7].wrapping_add(c.r[5]);
                c.r[3] = c.lbz(c.r[8].wrapping_add(c.r[3]));
                c.stb(c.r[8].wrapping_add(c.r[0]), c.r[3]);
                if true { return; }
                pc = 0x80161000;
            }
            0x80161000 => {
                c.cmpu(0, c.r[6], 0x80);
                if c.crbit(2) { pc = 0x80161010; continue; }
                pc = 0x80161008;
            }
            0x80161008 => {
                c.cmpu(0, c.r[6], 0x81);
                if !c.crbit(2) { pc = 0x8016104c; continue; }
                pc = 0x80161010;
            }
            0x80161010 => {
                c.r[6] = c.r[0].wrapping_mul(0x86);
                c.r[4] = 0x80450000;
                c.r[5] = c.r[3].rotate_left(0) & 0xfe;
                c.r[4] = c.r[4].wrapping_add(0xffffd910);
                c.r[3] = c.r[4].wrapping_add(c.r[6]);
                c.r[7] = c.r[7].wrapping_mul(0x86);
                c.r[0] = c.lbz(c.r[3].wrapping_add(c.r[5]));
                c.r[8] = c.r[4].wrapping_add(0x1);
                c.r[3] = c.r[4].wrapping_add(c.r[7]);
                c.stb(c.r[3].wrapping_add(c.r[5]), c.r[0]);
                c.r[3] = c.r[6].wrapping_add(c.r[5]);
                c.r[0] = c.r[7].wrapping_add(c.r[5]);
                c.r[3] = c.lbz(c.r[8].wrapping_add(c.r[3]));
                c.stb(c.r[8].wrapping_add(c.r[0]), c.r[3]);
                if true { return; }
                pc = 0x8016104c;
            }
            0x8016104c => {
                c.cmpu(0, c.r[6], 0x84);
                if c.crbit(2) { pc = 0x8016105c; continue; }
                pc = 0x80161054;
            }
            0x80161054 => {
                c.cmpu(0, c.r[6], 0x85);
                if !c.crbit(2) { pc = 0x80161098; continue; }
                pc = 0x8016105c;
            }
            0x8016105c => {
                c.r[6] = c.r[0].wrapping_mul(0x86);
                c.r[4] = 0x80450000;
                c.r[5] = c.r[3].rotate_left(0) & 0xfe;
                c.r[4] = c.r[4].wrapping_add(0xffffd910);
                c.r[3] = c.r[4].wrapping_add(c.r[6]);
                c.r[7] = c.r[7].wrapping_mul(0x86);
                c.r[0] = c.lbz(c.r[3].wrapping_add(c.r[5]));
                c.r[8] = c.r[4].wrapping_add(0x1);
                c.r[3] = c.r[4].wrapping_add(c.r[7]);
                c.stb(c.r[3].wrapping_add(c.r[5]), c.r[0]);
                c.r[3] = c.r[6].wrapping_add(c.r[5]);
                c.r[0] = c.r[7].wrapping_add(c.r[5]);
                c.r[3] = c.lbz(c.r[8].wrapping_add(c.r[3]));
                c.stb(c.r[8].wrapping_add(c.r[0]), c.r[3]);
                if true { return; }
                pc = 0x80161098;
            }
            0x80161098 => {
                c.r[4] = c.r[0].wrapping_mul(0x86);
                c.r[3] = 0x80450000;
                c.r[5] = c.r[3].wrapping_add(0xffffd910);
                c.r[0] = c.r[7].wrapping_mul(0x86);
                c.r[3] = c.r[5].wrapping_add(c.r[4]);
                c.r[4] = c.lbz(c.r[3].wrapping_add(c.r[6]));
                c.r[3] = c.r[5].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(c.r[6]), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80160fa8: bad pc {:#x}", pc),
        }
    }
}

/// inpSetMidiLastNote (0x801610bc, 60 bytes)
#[inline(never)]
pub fn f_801610bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801610bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801610bc); f_801610bc_b(c, 0x801610bc); crate::prof::exit(c, 0x801610bc); return; }
    }
    f_801610bc_b(c, 0x801610bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801610bc_at(c: &mut Cpu, pc: u32) { f_801610bc_b(c, pc) }
#[inline(always)]
fn f_801610bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801610bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801610bc, pc); }
        match pc {
            0x801610bc => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x801610e4; continue; }
                pc = 0x801610c8;
            }
            0x801610c8 => {
                c.r[6] = 0x80450000;
                c.r[7] = c.r[4].rotate_left(4) & 0xff0;
                c.r[4] = c.r[6].wrapping_add(0xfffffad0);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[4].wrapping_add(c.r[7]);
                c.stb(c.r[3].wrapping_add(c.r[0]), c.r[5]);
                if true { return; }
                pc = 0x801610e4;
            }
            0x801610e4 => {
                c.r[4] = 0x80450000;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[4].wrapping_add(0xfffffb50);
                c.stb(c.r[3].wrapping_add(c.r[0]), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_801610bc: bad pc {:#x}", pc),
        }
    }
}

/// inpGetMidiLastNote (0x801610f8, 60 bytes)
#[inline(never)]
pub fn f_801610f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801610f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801610f8); f_801610f8_b(c, 0x801610f8); crate::prof::exit(c, 0x801610f8); return; }
    }
    f_801610f8_b(c, 0x801610f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801610f8_at(c: &mut Cpu, pc: u32) { f_801610f8_b(c, pc) }
#[inline(always)]
fn f_801610f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801610f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801610f8, pc); }
        match pc {
            0x801610f8 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x80161120; continue; }
                pc = 0x80161104;
            }
            0x80161104 => {
                c.r[5] = 0x80450000;
                c.r[6] = c.r[4].rotate_left(4) & 0xff0;
                c.r[4] = c.r[5].wrapping_add(0xfffffad0);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[4].wrapping_add(c.r[6]);
                c.r[3] = c.lbz(c.r[3].wrapping_add(c.r[0]));
                if true { return; }
                pc = 0x80161120;
            }
            0x80161120 => {
                c.r[4] = 0x80450000;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[4].wrapping_add(0xfffffb50);
                c.r[3] = c.lbz(c.r[3].wrapping_add(c.r[0]));
                if true { return; }
                return;
            }
            _ => panic!("f_801610f8: bad pc {:#x}", pc),
        }
    }
}

/// _GetInputValue (0x80161134, 1184 bytes)
#[inline(never)]
pub fn f_80161134(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80161134) { return; }
        if c.prof { crate::prof::enter(c, 0x80161134); f_80161134_b(c, 0x80161134); crate::prof::exit(c, 0x80161134); return; }
    }
    f_80161134_b(c, 0x80161134)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161134_at(c: &mut Cpu, pc: u32) { f_80161134_b(c, pc) }
#[inline(always)]
fn f_80161134_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80161134;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80161134, pc); }
        match pc {
            0x80161134 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x80161148; f_800c47cc(c);
                pc = 0x80161148;
            }
            0x80161148 => {
                c.r[25] = c.r[4] | c.r[4];
                c.r[24] = c.r[3] | c.r[3];
                c.r[26] = c.r[5] | c.r[5];
                c.r[27] = c.r[6] | c.r[6];
                c.r[31] = c.r[25] | c.r[25];
                c.r[30] = 0x0;
                c.r[29] = 0x0;
                pc = 0x801615a8; continue;
                pc = 0x80161168;
            }
            0x80161168 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.r[0] = c.r[0].rotate_left(0) & 0x10;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x801611a0; continue; }
                pc = 0x80161174;
            }
            0x80161174 => {
                c.cmpu(0, c.r[24], 0x0);
                if c.crbit(2) { pc = 0x80161194; continue; }
                pc = 0x8016117c;
            }
            0x8016117c => {
                c.r[5] = c.lbz(c.r[31]);
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.lr = 0x8016118c; f_80155078(c);
                pc = 0x8016118c;
            }
            0x8016118c => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x80161198; continue;
                pc = 0x80161194;
            }
            0x80161194 => {
                c.r[0] = 0x0;
                pc = 0x80161198;
            }
            0x80161198 => {
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x8016122c; continue;
                pc = 0x801611a0;
            }
            0x801611a0 => {
                c.r[3] = c.lbz(c.r[31]);
                c.cmpu(0, c.r[3], 0x80);
                if c.crbit(2) { pc = 0x801611d4; continue; }
                pc = 0x801611ac;
            }
            0x801611ac => {
                c.cmpu(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x801611d4; continue; }
                pc = 0x801611b4;
            }
            0x801611b4 => {
                c.cmpu(0, c.r[3], 0xa);
                if c.crbit(2) { pc = 0x801611d4; continue; }
                pc = 0x801611bc;
            }
            0x801611bc => {
                c.r[0] = c.r[3].wrapping_add(0xffffff60);
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x801611d4; continue; }
                pc = 0x801611cc;
            }
            0x801611cc => {
                c.cmpu(0, c.r[3], 0x83);
                if !c.crbit(2) { pc = 0x80161394; continue; }
                pc = 0x801611d4;
            }
            0x801611d4 => {
                c.cmps(0, c.r[3], 0xa2);
                if !c.crbit(0) { pc = 0x80161218; continue; }
                pc = 0x801611dc;
            }
            0x801611dc => {
                c.cmps(0, c.r[3], 0xa0);
                if !c.crbit(0) { pc = 0x801611e8; continue; }
                pc = 0x801611e4;
            }
            0x801611e4 => {
                pc = 0x80161218; continue;
                pc = 0x801611e8;
            }
            0x801611e8 => {
                c.cmpu(0, c.r[24], 0x0);
                if c.crbit(2) { pc = 0x80161210; continue; }
                pc = 0x801611f0;
            }
            0x801611f0 => {
                c.r[4] = c.r[3].wrapping_mul(0xc);
                c.r[0] = c.r[3].wrapping_add(0x134);
                c.r[3] = 0x1;
                c.r[4] = c.r[4].wrapping_add(0xfffffa44);
                c.r[4] = c.lha(c.r[24].wrapping_add(c.r[4]));
                c.stb(c.r[24].wrapping_add(c.r[0]), c.r[3]);
                c.r[3] = c.r[4].rotate_left(1) & 0xfffffffe;
                pc = 0x8016122c; continue;
                pc = 0x80161210;
            }
            0x80161210 => {
                c.r[3] = 0x0;
                pc = 0x8016122c; continue;
                pc = 0x80161218;
            }
            0x80161218 => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.lr = 0x80161224; f_80160bdc(c);
                pc = 0x80161224;
            }
            0x80161224 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0xffffe000);
                pc = 0x8016122c;
            }
            0x8016122c => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.r[0] = c.r[3].wrapping_mul(c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 17 != 0); c.r[0] = (v >> 15) as u32; }
                c.cmps(0, c.r[0], 0xffffe000);
                if !c.crbit(0) { pc = 0x8016124c; continue; }
                pc = 0x80161244;
            }
            0x80161244 => {
                c.r[3] = 0xffffe000;
                pc = 0x8016125c; continue;
                pc = 0x8016124c;
            }
            0x8016124c => {
                c.cmps(0, c.r[0], 0x1fff);
                c.r[3] = 0x1fff;
                if c.crbit(1) { pc = 0x8016125c; continue; }
                pc = 0x80161258;
            }
            0x80161258 => {
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x8016125c;
            }
            0x8016125c => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.r[0] = c.r[0].rotate_left(0) & 0xf;
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x801612f0; continue; }
                pc = 0x8016126c;
            }
            0x8016126c => {
                if !c.crbit(0) { pc = 0x80161280; continue; }
                pc = 0x80161270;
            }
            0x80161270 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8016128c; continue; }
                pc = 0x80161278;
            }
            0x80161278 => {
                if !c.crbit(0) { pc = 0x80161298; continue; }
                pc = 0x8016127c;
            }
            0x8016127c => {
                pc = 0x801615a0; continue;
                pc = 0x80161280;
            }
            0x80161280 => {
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(0) { pc = 0x801615a0; continue; }
                pc = 0x80161288;
            }
            0x80161288 => {
                pc = 0x8016133c; continue;
                pc = 0x8016128c;
            }
            0x8016128c => {
                c.r[30] = c.r[3].wrapping_add(0x2000);
                c.r[28] = 0x1;
                pc = 0x801615a0; continue;
                pc = 0x80161298;
            }
            0x80161298 => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x801612d0; continue; }
                pc = 0x801612a0;
            }
            0x801612a0 => {
                c.r[4] = c.r[30].wrapping_add(c.r[3]);
                c.r[4] = c.r[4].wrapping_add(0xffffe000);
                c.cmps(0, c.r[4], 0xffffe000);
                if !c.crbit(0) { pc = 0x801612b8; continue; }
                pc = 0x801612b0;
            }
            0x801612b0 => {
                c.r[3] = 0xffffe000;
                pc = 0x801612c8; continue;
                pc = 0x801612b8;
            }
            0x801612b8 => {
                c.cmps(0, c.r[4], 0x1fff);
                c.r[3] = 0x1fff;
                if c.crbit(1) { pc = 0x801612c8; continue; }
                pc = 0x801612c4;
            }
            0x801612c4 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x801612c8;
            }
            0x801612c8 => {
                c.r[30] = c.r[3].wrapping_add(0x2000);
                pc = 0x801615a0; continue;
                pc = 0x801612d0;
            }
            0x801612d0 => {
                c.r[3] = c.r[30].wrapping_add(c.r[3]);
                c.cmps(0, c.r[3], 0x3fff);
                if !c.crbit(1) { pc = 0x801612e4; continue; }
                pc = 0x801612dc;
            }
            0x801612dc => {
                c.r[30] = 0x3fff;
                pc = 0x801615a0; continue;
                pc = 0x801612e4;
            }
            0x801612e4 => {
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[0] = (v >> 31) as u32; }
                c.r[30] = c.r[3] & !c.r[0];
                pc = 0x801615a0; continue;
                pc = 0x801612f0;
            }
            0x801612f0 => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x80161308; continue; }
                pc = 0x801612f8;
            }
            0x801612f8 => {
                c.r[0] = c.r[30].wrapping_add(0xffffe000);
                c.r[0] = c.r[0].wrapping_mul(c.r[3]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 19 != 0); c.r[0] = (v >> 13) as u32; }
                pc = 0x80161314; continue;
                pc = 0x80161308;
            }
            0x80161308 => {
                c.r[0] = c.r[3].wrapping_mul(c.r[30]);
                c.r[28] = 0x1;
                c.r[0] = c.r[0].rotate_left(19) & 0x7ffff;
                pc = 0x80161314;
            }
            0x80161314 => {
                c.cmps(0, c.r[0], 0xffffe000);
                if !c.crbit(0) { pc = 0x80161324; continue; }
                pc = 0x8016131c;
            }
            0x8016131c => {
                c.r[3] = 0xffffe000;
                pc = 0x80161334; continue;
                pc = 0x80161324;
            }
            0x80161324 => {
                c.cmps(0, c.r[0], 0x1fff);
                c.r[3] = 0x1fff;
                if c.crbit(1) { pc = 0x80161334; continue; }
                pc = 0x80161330;
            }
            0x80161330 => {
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x80161334;
            }
            0x80161334 => {
                c.r[30] = c.r[3].wrapping_add(0x2000);
                pc = 0x801615a0; continue;
                pc = 0x8016133c;
            }
            0x8016133c => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x80161374; continue; }
                pc = 0x80161344;
            }
            0x80161344 => {
                c.r[0] = c.r[30].wrapping_add(0xffffe000);
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.cmps(0, c.r[0], 0xffffe000);
                if !c.crbit(0) { pc = 0x8016135c; continue; }
                pc = 0x80161354;
            }
            0x80161354 => {
                c.r[3] = 0xffffe000;
                pc = 0x8016136c; continue;
                pc = 0x8016135c;
            }
            0x8016135c => {
                c.cmps(0, c.r[0], 0x1fff);
                c.r[3] = 0x1fff;
                if c.crbit(1) { pc = 0x8016136c; continue; }
                pc = 0x80161368;
            }
            0x80161368 => {
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x8016136c;
            }
            0x8016136c => {
                c.r[30] = c.r[3].wrapping_add(0x2000);
                pc = 0x801615a0; continue;
                pc = 0x80161374;
            }
            0x80161374 => {
                c.r[3] = c.r[30].wrapping_sub(c.r[3]);
                c.cmps(0, c.r[3], 0x3fff);
                if !c.crbit(1) { pc = 0x80161388; continue; }
                pc = 0x80161380;
            }
            0x80161380 => {
                c.r[30] = 0x3fff;
                pc = 0x801615a0; continue;
                pc = 0x80161388;
            }
            0x80161388 => {
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[0] = (v >> 31) as u32; }
                c.r[30] = c.r[3] & !c.r[0];
                pc = 0x801615a0; continue;
                pc = 0x80161394;
            }
            0x80161394 => {
                c.cmps(0, c.r[3], 0xa3);
                if c.crbit(2) { pc = 0x801613d4; continue; }
                pc = 0x8016139c;
            }
            0x8016139c => {
                if !c.crbit(0) { pc = 0x801613ac; continue; }
                pc = 0x801613a0;
            }
            0x801613a0 => {
                c.cmps(0, c.r[3], 0xa2);
                if !c.crbit(0) { pc = 0x801613b8; continue; }
                pc = 0x801613a8;
            }
            0x801613a8 => {
                pc = 0x80161438; continue;
                pc = 0x801613ac;
            }
            0x801613ac => {
                c.cmps(0, c.r[3], 0xa5);
                if !c.crbit(0) { pc = 0x80161438; continue; }
                pc = 0x801613b4;
            }
            0x801613b4 => {
                pc = 0x801613f0; continue;
                pc = 0x801613b8;
            }
            0x801613b8 => {
                c.cmpu(0, c.r[24], 0x0);
                if c.crbit(2) { pc = 0x801613cc; continue; }
                pc = 0x801613c0;
            }
            0x801613c0 => {
                c.r[0] = c.lbz(c.r[24].wrapping_add(0x12f));
                c.r[5] = c.r[0].rotate_left(7) & 0xffffff80;
                pc = 0x80161448; continue;
                pc = 0x801613cc;
            }
            0x801613cc => {
                c.r[5] = 0x0;
                pc = 0x80161448; continue;
                pc = 0x801613d4;
            }
            0x801613d4 => {
                c.cmpu(0, c.r[24], 0x0);
                if c.crbit(2) { pc = 0x801613e8; continue; }
                pc = 0x801613dc;
            }
            0x801613dc => {
                c.r[0] = c.lwz(c.r[24].wrapping_add(0x158));
                c.r[5] = c.r[0].rotate_left(23) & 0x7fffff;
                pc = 0x80161448; continue;
                pc = 0x801613e8;
            }
            0x801613e8 => {
                c.r[5] = 0x0;
                pc = 0x80161448; continue;
                pc = 0x801613f0;
            }
            0x801613f0 => {
                c.cmpu(0, c.r[24], 0x0);
                if c.crbit(2) { pc = 0x80161430; continue; }
                pc = 0x801613f8;
            }
            0x801613f8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa73c));
                c.r[0] = c.lwz(c.r[24].wrapping_add(0x94));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa738));
                c.r[0] = c.addc(!c.r[0], c.r[3], 1);
                c.r[3] = c.lwz(c.r[24].wrapping_add(0x90));
                c.r[5] = c.r[0].rotate_left(24) & 0xffffffff;
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[3], c.r[4], ci); }
                c.r[5] = (c.r[0].rotate_left(24) & 0xff000000) | (c.r[5] & 0xffffff);
                c.cmps(0, c.r[5], 0x3fff);
                if !c.crbit(1) { pc = 0x80161424; continue; }
                pc = 0x80161420;
            }
            0x80161420 => {
                c.r[5] = 0x3fff;
                pc = 0x80161424;
            }
            0x80161424 => {
                c.r[0] = 0x1;
                c.stb(c.r[24].wrapping_add(0xa8), c.r[0]);
                pc = 0x80161448; continue;
                pc = 0x80161430;
            }
            0x80161430 => {
                c.r[5] = 0x0;
                pc = 0x80161448; continue;
                pc = 0x80161438;
            }
            0x80161438 => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.lr = 0x80161444; f_80160bdc(c);
                pc = 0x80161444;
            }
            0x80161444 => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80161448;
            }
            0x80161448 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.r[0] = c.r[5].wrapping_mul(c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 17 != 0); c.r[3] = (v >> 15) as u32; }
                c.cmps(0, c.r[3], 0x3fff);
                if !c.crbit(1) { pc = 0x80161464; continue; }
                pc = 0x80161460;
            }
            0x80161460 => {
                c.r[3] = 0x3fff;
                pc = 0x80161464;
            }
            0x80161464 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.r[0] = c.r[0].rotate_left(0) & 0xf;
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x801614f4; continue; }
                pc = 0x80161474;
            }
            0x80161474 => {
                if !c.crbit(0) { pc = 0x80161488; continue; }
                pc = 0x80161478;
            }
            0x80161478 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80161494; continue; }
                pc = 0x80161480;
            }
            0x80161480 => {
                if !c.crbit(0) { pc = 0x801614a0; continue; }
                pc = 0x80161484;
            }
            0x80161484 => {
                pc = 0x801615a0; continue;
                pc = 0x80161488;
            }
            0x80161488 => {
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(0) { pc = 0x801615a0; continue; }
                pc = 0x80161490;
            }
            0x80161490 => {
                pc = 0x8016154c; continue;
                pc = 0x80161494;
            }
            0x80161494 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[28] = 0x0;
                pc = 0x801615a0; continue;
                pc = 0x801614a0;
            }
            0x801614a0 => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x801614d8; continue; }
                pc = 0x801614a8;
            }
            0x801614a8 => {
                c.r[4] = c.r[30].wrapping_add(c.r[3]);
                c.r[4] = c.r[4].wrapping_add(0xffffe000);
                c.cmps(0, c.r[4], 0xffffe000);
                if !c.crbit(0) { pc = 0x801614c0; continue; }
                pc = 0x801614b8;
            }
            0x801614b8 => {
                c.r[3] = 0xffffe000;
                pc = 0x801614d0; continue;
                pc = 0x801614c0;
            }
            0x801614c0 => {
                c.cmps(0, c.r[4], 0x1fff);
                c.r[3] = 0x1fff;
                if c.crbit(1) { pc = 0x801614d0; continue; }
                pc = 0x801614cc;
            }
            0x801614cc => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x801614d0;
            }
            0x801614d0 => {
                c.r[30] = c.r[3].wrapping_add(0x2000);
                pc = 0x801615a0; continue;
                pc = 0x801614d8;
            }
            0x801614d8 => {
                c.r[30] = c.r[30].wrapping_add(c.r[3]);
                c.r[0] = 0x3fff;
                c.cmpu(0, c.r[30], 0x3fff);
                if c.crbit(1) { pc = 0x801614ec; continue; }
                pc = 0x801614e8;
            }
            0x801614e8 => {
                c.r[0] = c.r[30] | c.r[30];
                pc = 0x801614ec;
            }
            0x801614ec => {
                c.r[30] = c.r[0] | c.r[0];
                pc = 0x801615a0; continue;
                pc = 0x801614f4;
            }
            0x801614f4 => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x80161530; continue; }
                pc = 0x801614fc;
            }
            0x801614fc => {
                c.r[0] = c.r[30].wrapping_add(0xffffe000);
                c.r[0] = c.r[3].wrapping_mul(c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 18 != 0); c.r[0] = (v >> 14) as u32; }
                c.cmps(0, c.r[0], 0xffffe000);
                if !c.crbit(0) { pc = 0x80161518; continue; }
                pc = 0x80161510;
            }
            0x80161510 => {
                c.r[3] = 0xffffe000;
                pc = 0x80161528; continue;
                pc = 0x80161518;
            }
            0x80161518 => {
                c.cmps(0, c.r[0], 0x1fff);
                c.r[3] = 0x1fff;
                if c.crbit(1) { pc = 0x80161528; continue; }
                pc = 0x80161524;
            }
            0x80161524 => {
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x80161528;
            }
            0x80161528 => {
                c.r[30] = c.r[3].wrapping_add(0x2000);
                pc = 0x801615a0; continue;
                pc = 0x80161530;
            }
            0x80161530 => {
                c.r[0] = c.r[30].wrapping_mul(c.r[3]);
                c.r[30] = 0x3fff;
                c.r[0] = c.r[0].rotate_left(18) & 0x3ffff;
                c.cmpu(0, c.r[0], 0x3fff);
                if c.crbit(1) { pc = 0x801615a0; continue; }
                pc = 0x80161544;
            }
            0x80161544 => {
                c.r[30] = c.r[0] | c.r[0];
                pc = 0x801615a0; continue;
                pc = 0x8016154c;
            }
            0x8016154c => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x80161584; continue; }
                pc = 0x80161554;
            }
            0x80161554 => {
                c.r[0] = c.r[30].wrapping_add(0xffffe000);
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.cmps(0, c.r[0], 0xffffe000);
                if !c.crbit(0) { pc = 0x8016156c; continue; }
                pc = 0x80161564;
            }
            0x80161564 => {
                c.r[3] = 0xffffe000;
                pc = 0x8016157c; continue;
                pc = 0x8016156c;
            }
            0x8016156c => {
                c.cmps(0, c.r[0], 0x1fff);
                c.r[3] = 0x1fff;
                if c.crbit(1) { pc = 0x8016157c; continue; }
                pc = 0x80161578;
            }
            0x80161578 => {
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x8016157c;
            }
            0x8016157c => {
                c.r[30] = c.r[3].wrapping_add(0x2000);
                pc = 0x801615a0; continue;
                pc = 0x80161584;
            }
            0x80161584 => {
                c.r[3] = c.r[30].wrapping_sub(c.r[3]);
                c.cmps(0, c.r[3], 0x3fff);
                if !c.crbit(1) { pc = 0x80161598; continue; }
                pc = 0x80161590;
            }
            0x80161590 => {
                c.r[30] = 0x3fff;
                pc = 0x801615a0; continue;
                pc = 0x80161598;
            }
            0x80161598 => {
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[0] = (v >> 31) as u32; }
                c.r[30] = c.r[3] & !c.r[0];
                pc = 0x801615a0;
            }
            0x801615a0 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801615a8;
            }
            0x801615a8 => {
                c.r[0] = c.lbz(c.r[25].wrapping_add(0x22));
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x80161168; continue; }
                pc = 0x801615b4;
            }
            0x801615b4 => {
                c.sth(c.r[25].wrapping_add(0x20), c.r[30]);
                c.r[3] = c.r[30].rotate_left(0) & 0xffff;
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x801615c4; f_800c4818(c);
                pc = 0x801615c4;
            }
            0x801615c4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80161134: bad pc {:#x}", pc),
        }
    }
}

/// inpGetVolume (0x801615d4, 72 bytes)
#[inline(never)]
pub fn f_801615d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801615d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801615d4); f_801615d4_b(c, 0x801615d4); crate::prof::exit(c, 0x801615d4); return; }
    }
    f_801615d4_b(c, 0x801615d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801615d4_at(c: &mut Cpu, pc: u32) { f_801615d4_b(c, pc) }
#[inline(always)]
fn f_801615d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801615d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801615d4, pc); }
        match pc {
            0x801615d4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x801615f4; continue; }
                pc = 0x801615ec;
            }
            0x801615ec => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x238));
                pc = 0x8016160c; continue;
                pc = 0x801615f4;
            }
            0x801615f4 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffffe;
                c.r[4] = c.r[3].wrapping_add(0x218);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz(c.r[3].wrapping_add(0x122));
                c.lr = 0x8016160c; f_80161134(c);
                pc = 0x8016160c;
            }
            0x8016160c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801615d4: bad pc {:#x}", pc),
        }
    }
}

/// inpGetPanning (0x8016161c, 72 bytes)
#[inline(never)]
pub fn f_8016161c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016161c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016161c); f_8016161c_b(c, 0x8016161c); crate::prof::exit(c, 0x8016161c); return; }
    }
    f_8016161c_b(c, 0x8016161c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016161c_at(c: &mut Cpu, pc: u32) { f_8016161c_b(c, pc) }
#[inline(always)]
fn f_8016161c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016161c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016161c, pc); }
        match pc {
            0x8016161c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8016163c; continue; }
                pc = 0x80161634;
            }
            0x80161634 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x25c));
                pc = 0x80161654; continue;
                pc = 0x8016163c;
            }
            0x8016163c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffffd;
                c.r[4] = c.r[3].wrapping_add(0x23c);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz(c.r[3].wrapping_add(0x122));
                c.lr = 0x80161654; f_80161134(c);
                pc = 0x80161654;
            }
            0x80161654 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8016161c: bad pc {:#x}", pc),
        }
    }
}

/// inpGetSurroundPanning (0x80161664, 72 bytes)
#[inline(never)]
pub fn f_80161664(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80161664) { return; }
        if c.prof { crate::prof::enter(c, 0x80161664); f_80161664_b(c, 0x80161664); crate::prof::exit(c, 0x80161664); return; }
    }
    f_80161664_b(c, 0x80161664)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161664_at(c: &mut Cpu, pc: u32) { f_80161664_b(c, pc) }
#[inline(always)]
fn f_80161664_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80161664;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80161664, pc); }
        match pc {
            0x80161664 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x4;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80161684; continue; }
                pc = 0x8016167c;
            }
            0x8016167c => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x280));
                pc = 0x8016169c; continue;
                pc = 0x80161684;
            }
            0x80161684 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffffb;
                c.r[4] = c.r[3].wrapping_add(0x260);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz(c.r[3].wrapping_add(0x122));
                c.lr = 0x8016169c; f_80161134(c);
                pc = 0x8016169c;
            }
            0x8016169c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80161664: bad pc {:#x}", pc),
        }
    }
}

/// inpGetPitchBend (0x801616ac, 72 bytes)
#[inline(never)]
pub fn f_801616ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801616ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801616ac); f_801616ac_b(c, 0x801616ac); crate::prof::exit(c, 0x801616ac); return; }
    }
    f_801616ac_b(c, 0x801616ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801616ac_at(c: &mut Cpu, pc: u32) { f_801616ac_b(c, pc) }
#[inline(always)]
fn f_801616ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801616ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801616ac, pc); }
        match pc {
            0x801616ac => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x801616cc; continue; }
                pc = 0x801616c4;
            }
            0x801616c4 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2a4));
                pc = 0x801616e4; continue;
                pc = 0x801616cc;
            }
            0x801616cc => {
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffff7;
                c.r[4] = c.r[3].wrapping_add(0x284);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz(c.r[3].wrapping_add(0x122));
                c.lr = 0x801616e4; f_80161134(c);
                pc = 0x801616e4;
            }
            0x801616e4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801616ac: bad pc {:#x}", pc),
        }
    }
}

/// inpGetDoppler (0x801616f4, 72 bytes)
#[inline(never)]
pub fn f_801616f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801616f4) { return; }
        if c.prof { crate::prof::enter(c, 0x801616f4); f_801616f4_b(c, 0x801616f4); crate::prof::exit(c, 0x801616f4); return; }
    }
    f_801616f4_b(c, 0x801616f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801616f4_at(c: &mut Cpu, pc: u32) { f_801616f4_b(c, pc) }
#[inline(always)]
fn f_801616f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801616f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801616f4, pc); }
        match pc {
            0x801616f4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x10;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80161714; continue; }
                pc = 0x8016170c;
            }
            0x8016170c => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2c8));
                pc = 0x8016172c; continue;
                pc = 0x80161714;
            }
            0x80161714 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffffffef;
                c.r[4] = c.r[3].wrapping_add(0x2a8);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz(c.r[3].wrapping_add(0x122));
                c.lr = 0x8016172c; f_80161134(c);
                pc = 0x8016172c;
            }
            0x8016172c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801616f4: bad pc {:#x}", pc),
        }
    }
}

/// inpGetModulation (0x8016173c, 72 bytes)
#[inline(never)]
pub fn f_8016173c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016173c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016173c); f_8016173c_b(c, 0x8016173c); crate::prof::exit(c, 0x8016173c); return; }
    }
    f_8016173c_b(c, 0x8016173c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016173c_at(c: &mut Cpu, pc: u32) { f_8016173c_b(c, pc) }
#[inline(always)]
fn f_8016173c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016173c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016173c, pc); }
        match pc {
            0x8016173c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x20;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8016175c; continue; }
                pc = 0x80161754;
            }
            0x80161754 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2ec));
                pc = 0x80161774; continue;
                pc = 0x8016175c;
            }
            0x8016175c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffffffdf;
                c.r[4] = c.r[3].wrapping_add(0x2cc);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz(c.r[3].wrapping_add(0x122));
                c.lr = 0x80161774; f_80161134(c);
                pc = 0x80161774;
            }
            0x80161774 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8016173c: bad pc {:#x}", pc),
        }
    }
}

/// inpGetPedal (0x80161784, 72 bytes)
#[inline(never)]
pub fn f_80161784(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80161784) { return; }
        if c.prof { crate::prof::enter(c, 0x80161784); f_80161784_b(c, 0x80161784); crate::prof::exit(c, 0x80161784); return; }
    }
    f_80161784_b(c, 0x80161784)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161784_at(c: &mut Cpu, pc: u32) { f_80161784_b(c, pc) }
#[inline(always)]
fn f_80161784_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80161784;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80161784, pc); }
        match pc {
            0x80161784 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x40;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x801617a4; continue; }
                pc = 0x8016179c;
            }
            0x8016179c => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x310));
                pc = 0x801617bc; continue;
                pc = 0x801617a4;
            }
            0x801617a4 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffffffbf;
                c.r[4] = c.r[3].wrapping_add(0x2f0);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz(c.r[3].wrapping_add(0x122));
                c.lr = 0x801617bc; f_80161134(c);
                pc = 0x801617bc;
            }
            0x801617bc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80161784: bad pc {:#x}", pc),
        }
    }
}

/// inpGetPreAuxA (0x801617cc, 72 bytes)
#[inline(never)]
pub fn f_801617cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801617cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801617cc); f_801617cc_b(c, 0x801617cc); crate::prof::exit(c, 0x801617cc); return; }
    }
    f_801617cc_b(c, 0x801617cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801617cc_at(c: &mut Cpu, pc: u32) { f_801617cc_b(c, pc) }
#[inline(always)]
fn f_801617cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801617cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801617cc, pc); }
        match pc {
            0x801617cc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x100;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x801617ec; continue; }
                pc = 0x801617e4;
            }
            0x801617e4 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x358));
                pc = 0x80161804; continue;
                pc = 0x801617ec;
            }
            0x801617ec => {
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffeff;
                c.r[4] = c.r[3].wrapping_add(0x338);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz(c.r[3].wrapping_add(0x122));
                c.lr = 0x80161804; f_80161134(c);
                pc = 0x80161804;
            }
            0x80161804 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801617cc: bad pc {:#x}", pc),
        }
    }
}

/// inpGetReverb (0x80161814, 72 bytes)
#[inline(never)]
pub fn f_80161814(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80161814) { return; }
        if c.prof { crate::prof::enter(c, 0x80161814); f_80161814_b(c, 0x80161814); crate::prof::exit(c, 0x80161814); return; }
    }
    f_80161814_b(c, 0x80161814)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161814_at(c: &mut Cpu, pc: u32) { f_80161814_b(c, pc) }
#[inline(always)]
fn f_80161814_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80161814;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80161814, pc); }
        match pc {
            0x80161814 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x200;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80161834; continue; }
                pc = 0x8016182c;
            }
            0x8016182c => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x37c));
                pc = 0x8016184c; continue;
                pc = 0x80161834;
            }
            0x80161834 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffdff;
                c.r[4] = c.r[3].wrapping_add(0x35c);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz(c.r[3].wrapping_add(0x122));
                c.lr = 0x8016184c; f_80161134(c);
                pc = 0x8016184c;
            }
            0x8016184c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80161814: bad pc {:#x}", pc),
        }
    }
}

/// inpGetPreAuxB (0x8016185c, 72 bytes)
#[inline(never)]
pub fn f_8016185c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016185c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016185c); f_8016185c_b(c, 0x8016185c); crate::prof::exit(c, 0x8016185c); return; }
    }
    f_8016185c_b(c, 0x8016185c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016185c_at(c: &mut Cpu, pc: u32) { f_8016185c_b(c, pc) }
#[inline(always)]
fn f_8016185c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016185c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016185c, pc); }
        match pc {
            0x8016185c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x400;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8016187c; continue; }
                pc = 0x80161874;
            }
            0x80161874 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x3a0));
                pc = 0x80161894; continue;
                pc = 0x8016187c;
            }
            0x8016187c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffbff;
                c.r[4] = c.r[3].wrapping_add(0x380);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz(c.r[3].wrapping_add(0x122));
                c.lr = 0x80161894; f_80161134(c);
                pc = 0x80161894;
            }
            0x80161894 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8016185c: bad pc {:#x}", pc),
        }
    }
}

/// inpGetPostAuxB (0x801618a4, 72 bytes)
#[inline(never)]
pub fn f_801618a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801618a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801618a4); f_801618a4_b(c, 0x801618a4); crate::prof::exit(c, 0x801618a4); return; }
    }
    f_801618a4_b(c, 0x801618a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801618a4_at(c: &mut Cpu, pc: u32) { f_801618a4_b(c, pc) }
#[inline(always)]
fn f_801618a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801618a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801618a4, pc); }
        match pc {
            0x801618a4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x800;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x801618c4; continue; }
                pc = 0x801618bc;
            }
            0x801618bc => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x3c4));
                pc = 0x801618dc; continue;
                pc = 0x801618c4;
            }
            0x801618c4 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xfffff7ff;
                c.r[4] = c.r[3].wrapping_add(0x3a4);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz(c.r[3].wrapping_add(0x122));
                c.lr = 0x801618dc; f_80161134(c);
                pc = 0x801618dc;
            }
            0x801618dc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801618a4: bad pc {:#x}", pc),
        }
    }
}

/// inpGetTremolo (0x801618ec, 72 bytes)
#[inline(never)]
pub fn f_801618ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801618ec) { return; }
        if c.prof { crate::prof::enter(c, 0x801618ec); f_801618ec_b(c, 0x801618ec); crate::prof::exit(c, 0x801618ec); return; }
    }
    f_801618ec_b(c, 0x801618ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801618ec_at(c: &mut Cpu, pc: u32) { f_801618ec_b(c, pc) }
#[inline(always)]
fn f_801618ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801618ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801618ec, pc); }
        match pc {
            0x801618ec => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x1000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8016190c; continue; }
                pc = 0x80161904;
            }
            0x80161904 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x3e8));
                pc = 0x80161924; continue;
                pc = 0x8016190c;
            }
            0x8016190c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffffefff;
                c.r[4] = c.r[3].wrapping_add(0x3c8);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz(c.r[3].wrapping_add(0x122));
                c.lr = 0x80161924; f_80161134(c);
                pc = 0x80161924;
            }
            0x80161924 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801618ec: bad pc {:#x}", pc),
        }
    }
}

/// fn_80161934 (0x80161934, 180 bytes)
#[inline(never)]
pub fn f_80161934(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80161934) { return; }
        if c.prof { crate::prof::enter(c, 0x80161934); f_80161934_b(c, 0x80161934); crate::prof::exit(c, 0x80161934); return; }
    }
    f_80161934_b(c, 0x80161934)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161934_at(c: &mut Cpu, pc: u32) { f_80161934_b(c, pc) }
#[inline(always)]
fn f_80161934_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80161934;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80161934, pc); }
        match pc {
            0x80161934 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x80450000;
                c.r[9] = 0x80370000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[7].wrapping_add(0xffff9390);
                c.r[8] = c.r[6].rotate_left(6) & 0x3fc0;
                c.r[7] = c.r[9].wrapping_add(0xffff9c90);
                c.r[10] = c.r[4].rotate_left(2) & 0x3fc;
                c.r[11] = c.r[4].rotate_left(0) & 0xff;
                c.r[9] = c.r[0].wrapping_add(c.r[8]);
                c.r[8] = c.r[5].rotate_left(2) & 0x3fc;
                c.r[10] = c.lwz(c.r[7].wrapping_add(c.r[10]));
                c.r[7] = c.lwz(c.r[9].wrapping_add(c.r[8]));
                c.r[4] = c.r[10] & c.r[7];
                c.r[0] = c.r[4].wrapping_neg();
                c.r[0] = c.r[0] | c.r[4];
                c.r[4] = c.r[0].rotate_left(1) & 0x1;
                c.rc(c.r[4]);
                if c.crbit(2) { pc = 0x80161988; continue; }
                pc = 0x80161980;
            }
            0x80161980 => {
                c.r[0] = c.r[7] & !c.r[10];
                c.stw(c.r[9].wrapping_add(c.r[8]), c.r[0]);
                pc = 0x80161988;
            }
            0x80161988 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x801619b8; continue; }
                pc = 0x80161990;
            }
            0x80161990 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x80430000;
                c.r[7] = c.r[0].wrapping_mul(0x90);
                c.r[4] = c.r[3].wrapping_add(0x5b74);
                c.r[3] = 0x0;
                c.r[0] = c.r[11].wrapping_mul(0x24);
                c.r[4] = c.r[4].wrapping_add(c.r[7]);
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.lr = 0x801619b4; f_80161134(c);
                pc = 0x801619b4;
            }
            0x801619b4 => {
                pc = 0x801619d8; continue;
                pc = 0x801619b8;
            }
            0x801619b8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = 0x80430000;
                c.r[3] = c.r[0].wrapping_mul(0x90);
                c.r[4] = c.r[4].wrapping_add(0x5b74);
                c.r[0] = c.r[11].wrapping_mul(0x24);
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x20));
                pc = 0x801619d8;
            }
            0x801619d8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80161934: bad pc {:#x}", pc),
        }
    }
}

/// fn_801619E8 (0x801619e8, 180 bytes)
#[inline(never)]
pub fn f_801619e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801619e8) { return; }
        if c.prof { crate::prof::enter(c, 0x801619e8); f_801619e8_b(c, 0x801619e8); crate::prof::exit(c, 0x801619e8); return; }
    }
    f_801619e8_b(c, 0x801619e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801619e8_at(c: &mut Cpu, pc: u32) { f_801619e8_b(c, pc) }
#[inline(always)]
fn f_801619e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801619e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801619e8, pc); }
        match pc {
            0x801619e8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x80450000;
                c.r[9] = 0x80370000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[7].wrapping_add(0xffff9390);
                c.r[8] = c.r[6].rotate_left(6) & 0x3fc0;
                c.r[7] = c.r[9].wrapping_add(0xffff9ca0);
                c.r[10] = c.r[4].rotate_left(2) & 0x3fc;
                c.r[11] = c.r[4].rotate_left(0) & 0xff;
                c.r[9] = c.r[0].wrapping_add(c.r[8]);
                c.r[8] = c.r[5].rotate_left(2) & 0x3fc;
                c.r[10] = c.lwz(c.r[7].wrapping_add(c.r[10]));
                c.r[7] = c.lwz(c.r[9].wrapping_add(c.r[8]));
                c.r[4] = c.r[10] & c.r[7];
                c.r[0] = c.r[4].wrapping_neg();
                c.r[0] = c.r[0] | c.r[4];
                c.r[4] = c.r[0].rotate_left(1) & 0x1;
                c.rc(c.r[4]);
                if c.crbit(2) { pc = 0x80161a3c; continue; }
                pc = 0x80161a34;
            }
            0x80161a34 => {
                c.r[0] = c.r[7] & !c.r[10];
                c.stw(c.r[9].wrapping_add(c.r[8]), c.r[0]);
                pc = 0x80161a3c;
            }
            0x80161a3c => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x80161a6c; continue; }
                pc = 0x80161a44;
            }
            0x80161a44 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x80430000;
                c.r[7] = c.r[0].wrapping_mul(0x90);
                c.r[4] = c.r[3].wrapping_add(0x56f4);
                c.r[3] = 0x0;
                c.r[0] = c.r[11].wrapping_mul(0x24);
                c.r[4] = c.r[4].wrapping_add(c.r[7]);
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.lr = 0x80161a68; f_80161134(c);
                pc = 0x80161a68;
            }
            0x80161a68 => {
                pc = 0x80161a8c; continue;
                pc = 0x80161a6c;
            }
            0x80161a6c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = 0x80430000;
                c.r[3] = c.r[0].wrapping_mul(0x90);
                c.r[4] = c.r[4].wrapping_add(0x56f4);
                c.r[0] = c.r[11].wrapping_mul(0x24);
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x20));
                pc = 0x80161a8c;
            }
            0x80161a8c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801619e8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80161A9C (0x80161a9c, 644 bytes)
#[inline(never)]
pub fn f_80161a9c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80161a9c) { return; }
        if c.prof { crate::prof::enter(c, 0x80161a9c); f_80161a9c_b(c, 0x80161a9c); crate::prof::exit(c, 0x80161a9c); return; }
    }
    f_80161a9c_b(c, 0x80161a9c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161a9c_at(c: &mut Cpu, pc: u32) { f_80161a9c_b(c, pc) }
#[inline(always)]
fn f_80161a9c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80161a9c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80161a9c, pc); }
        match pc {
            0x80161a9c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                if c.crbit(2) { pc = 0x80161bbc; continue; }
                pc = 0x80161ab4;
            }
            0x80161ab4 => {
                c.r[0] = 0x7;
                c.r[29] = 0x0;
                c.stb(c.r[3].wrapping_add(0x218), c.r[0]);
                c.r[30] = 0x10000;
                c.r[0] = 0xb;
                c.r[31] = 0x2;
                c.stb(c.r[3].wrapping_add(0x219), c.r[29]);
                c.r[12] = 0xa;
                c.r[11] = 0x1;
                c.r[10] = 0x83;
                c.stw(c.r[3].wrapping_add(0x21c), c.r[30]);
                c.r[9] = 0x80;
                c.r[8] = 0x40;
                c.r[7] = 0x41;
                c.stb(c.r[3].wrapping_add(0x220), c.r[0]);
                c.r[6] = 0x5b;
                c.r[5] = 0x5d;
                c.r[4] = 0x84;
                c.stb(c.r[3].wrapping_add(0x221), c.r[31]);
                c.r[0] = 0x1fff;
                c.stw(c.r[3].wrapping_add(0x224), c.r[30]);
                c.stb(c.r[3].wrapping_add(0x23a), c.r[31]);
                c.stb(c.r[3].wrapping_add(0x23c), c.r[12]);
                c.stb(c.r[3].wrapping_add(0x23d), c.r[29]);
                c.stw(c.r[3].wrapping_add(0x240), c.r[30]);
                c.stb(c.r[3].wrapping_add(0x25e), c.r[11]);
                c.stb(c.r[3].wrapping_add(0x260), c.r[10]);
                c.stb(c.r[3].wrapping_add(0x261), c.r[29]);
                c.stw(c.r[3].wrapping_add(0x264), c.r[30]);
                c.stb(c.r[3].wrapping_add(0x282), c.r[11]);
                c.stb(c.r[3].wrapping_add(0x284), c.r[9]);
                c.stb(c.r[3].wrapping_add(0x285), c.r[29]);
                c.stw(c.r[3].wrapping_add(0x288), c.r[30]);
                c.stb(c.r[3].wrapping_add(0x2a6), c.r[11]);
                c.stb(c.r[3].wrapping_add(0x2cc), c.r[11]);
                c.stb(c.r[3].wrapping_add(0x2cd), c.r[29]);
                c.stw(c.r[3].wrapping_add(0x2d0), c.r[30]);
                c.stb(c.r[3].wrapping_add(0x2ee), c.r[11]);
                c.stb(c.r[3].wrapping_add(0x2f0), c.r[8]);
                c.stb(c.r[3].wrapping_add(0x2f1), c.r[29]);
                c.stw(c.r[3].wrapping_add(0x2f4), c.r[30]);
                c.stb(c.r[3].wrapping_add(0x312), c.r[11]);
                c.stb(c.r[3].wrapping_add(0x314), c.r[7]);
                c.stb(c.r[3].wrapping_add(0x315), c.r[29]);
                c.stw(c.r[3].wrapping_add(0x318), c.r[30]);
                c.stb(c.r[3].wrapping_add(0x336), c.r[11]);
                c.stb(c.r[3].wrapping_add(0x35a), c.r[29]);
                c.stb(c.r[3].wrapping_add(0x35c), c.r[6]);
                c.stb(c.r[3].wrapping_add(0x35d), c.r[29]);
                c.stw(c.r[3].wrapping_add(0x360), c.r[30]);
                c.stb(c.r[3].wrapping_add(0x37e), c.r[11]);
                c.stb(c.r[3].wrapping_add(0x3a2), c.r[29]);
                c.stb(c.r[3].wrapping_add(0x3a4), c.r[5]);
                c.stb(c.r[3].wrapping_add(0x3a5), c.r[29]);
                c.stw(c.r[3].wrapping_add(0x3a8), c.r[30]);
                c.stb(c.r[3].wrapping_add(0x3c6), c.r[11]);
                c.stb(c.r[3].wrapping_add(0x2a8), c.r[4]);
                c.stb(c.r[3].wrapping_add(0x2a9), c.r[29]);
                c.stw(c.r[3].wrapping_add(0x2ac), c.r[30]);
                c.stb(c.r[3].wrapping_add(0x2ca), c.r[11]);
                c.stb(c.r[3].wrapping_add(0x3ea), c.r[29]);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x1d4), c.r[29]);
                c.stb(c.r[3].wrapping_add(0x1d5), c.r[29]);
                c.stb(c.r[3].wrapping_add(0xa8), c.r[29]);
                pc = 0x80161d0c; continue;
                pc = 0x80161bbc;
            }
            0x80161bbc => {
                c.r[4] = 0x80430000;
                c.r[3] = 0x80430000;
                c.r[5] = c.r[3].wrapping_add(0x56f4);
                c.r[0] = 0x2;
                c.r[4] = c.r[4].wrapping_add(0x5b74);
                c.r[3] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x80161bd8;
            }
            0x80161bd8 => {
                c.stb(c.r[4].wrapping_add(0x22), c.r[3]);
                c.stb(c.r[5].wrapping_add(0x22), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x46), c.r[3]);
                c.stb(c.r[5].wrapping_add(0x46), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x6a), c.r[3]);
                c.stb(c.r[5].wrapping_add(0x6a), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x8e), c.r[3]);
                c.stb(c.r[5].wrapping_add(0x8e), c.r[3]);
                c.stb(c.r[4].wrapping_add(0xb2), c.r[3]);
                c.stb(c.r[5].wrapping_add(0xb2), c.r[3]);
                c.stb(c.r[4].wrapping_add(0xd6), c.r[3]);
                c.stb(c.r[5].wrapping_add(0xd6), c.r[3]);
                c.stb(c.r[4].wrapping_add(0xfa), c.r[3]);
                c.stb(c.r[5].wrapping_add(0xfa), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x11e), c.r[3]);
                c.stb(c.r[5].wrapping_add(0x11e), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x142), c.r[3]);
                c.stb(c.r[5].wrapping_add(0x142), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x166), c.r[3]);
                c.stb(c.r[5].wrapping_add(0x166), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x18a), c.r[3]);
                c.stb(c.r[5].wrapping_add(0x18a), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x1ae), c.r[3]);
                c.stb(c.r[5].wrapping_add(0x1ae), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x1d2), c.r[3]);
                c.stb(c.r[5].wrapping_add(0x1d2), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x1f6), c.r[3]);
                c.stb(c.r[5].wrapping_add(0x1f6), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x21a), c.r[3]);
                c.stb(c.r[5].wrapping_add(0x21a), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x23e), c.r[3]);
                c.r[4] = c.r[4].wrapping_add(0x240);
                c.stb(c.r[5].wrapping_add(0x23e), c.r[3]);
                c.r[5] = c.r[5].wrapping_add(0x240);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80161bd8; continue; }
                pc = 0x80161c64;
            }
            0x80161c64 => {
                c.r[3] = 0x80450000;
                c.r[0] = 0x4;
                c.r[5] = c.r[3].wrapping_add(0xffff9390);
                c.r[3] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x80161c78;
            }
            0x80161c78 => {
                c.r[0] = 0xff;
                c.stw(c.r[5], c.r[0]);
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xc), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x10), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x18), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1c), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x20), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x28), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x2c), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x30), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x34), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x38), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x3c), c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0x40);
                c.stw(c.r[5], c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xc), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x10), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x18), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1c), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x20), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x28), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x2c), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x30), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x34), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x38), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x3c), c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0x40);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80161c78; continue; }
                pc = 0x80161d0c;
            }
            0x80161d0c => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80161a9c: bad pc {:#x}", pc),
        }
    }
}

/// inpTranslateExCtrl (0x80161d20, 112 bytes)
#[inline(never)]
pub fn f_80161d20(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80161d20) { return; }
        if c.prof { crate::prof::enter(c, 0x80161d20); f_80161d20_b(c, 0x80161d20); crate::prof::exit(c, 0x80161d20); return; }
    }
    f_80161d20_b(c, 0x80161d20)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161d20_at(c: &mut Cpu, pc: u32) { f_80161d20_b(c, pc) }
#[inline(always)]
fn f_80161d20_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80161d20;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80161d20, pc); }
        match pc {
            0x80161d20 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80161d24;
            }
            0x80161d24 => {
                c.r[0] = c.r[4].wrapping_add(0xffffff80);
                pc = 0x80161d28;
            }
            0x80161d28 => {
                c.cmpu(0, c.r[0], 0x8);
                pc = 0x80161d2c;
            }
            0x80161d2c => {
                if c.crbit(1) { return; }
                pc = 0x80161d30;
            }
            0x80161d30 => {
                c.r[4] = 0x80370000;
                pc = 0x80161d34;
            }
            0x80161d34 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80161d38;
            }
            0x80161d38 => {
                c.r[4] = c.r[4].wrapping_add(0xffff9cb0);
                pc = 0x80161d3c;
            }
            0x80161d3c => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                pc = 0x80161d40;
            }
            0x80161d40 => {
                c.mtspr(9, c.r[0]);
                pc = 0x80161d44;
            }
            0x80161d44 => {
                if true { let t = c.ctr; if (0x80161d20..0x80161d90).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x80161d48;
            }
            0x80161d48 => {
                c.r[3] = 0x80;
                pc = 0x80161d4c;
            }
            0x80161d4c => {
                if true { return; }
                pc = 0x80161d50;
            }
            0x80161d50 => {
                c.r[3] = 0x82;
                pc = 0x80161d54;
            }
            0x80161d54 => {
                if true { return; }
                pc = 0x80161d58;
            }
            0x80161d58 => {
                c.r[3] = 0xa0;
                pc = 0x80161d5c;
            }
            0x80161d5c => {
                if true { return; }
                pc = 0x80161d60;
            }
            0x80161d60 => {
                c.r[3] = 0xa1;
                pc = 0x80161d64;
            }
            0x80161d64 => {
                if true { return; }
                pc = 0x80161d68;
            }
            0x80161d68 => {
                c.r[3] = 0x83;
                pc = 0x80161d6c;
            }
            0x80161d6c => {
                if true { return; }
                pc = 0x80161d70;
            }
            0x80161d70 => {
                c.r[3] = 0x84;
                pc = 0x80161d74;
            }
            0x80161d74 => {
                if true { return; }
                pc = 0x80161d78;
            }
            0x80161d78 => {
                c.r[3] = 0xa2;
                pc = 0x80161d7c;
            }
            0x80161d7c => {
                if true { return; }
                pc = 0x80161d80;
            }
            0x80161d80 => {
                c.r[3] = 0xa3;
                pc = 0x80161d84;
            }
            0x80161d84 => {
                if true { return; }
                pc = 0x80161d88;
            }
            0x80161d88 => {
                c.r[3] = 0xa4;
                pc = 0x80161d8c;
            }
            0x80161d8c => {
                if true { return; }
                return;
            }
            _ => panic!("f_80161d20: bad pc {:#x}", pc),
        }
    }
}

/// inpGetExCtrl (0x80161d90, 252 bytes)
#[inline(never)]
pub fn f_80161d90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80161d90) { return; }
        if c.prof { crate::prof::enter(c, 0x80161d90); f_80161d90_b(c, 0x80161d90); crate::prof::exit(c, 0x80161d90); return; }
    }
    f_80161d90_b(c, 0x80161d90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161d90_at(c: &mut Cpu, pc: u32) { f_80161d90_b(c, pc) }
#[inline(always)]
fn f_80161d90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80161d90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80161d90, pc); }
        match pc {
            0x80161d90 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x80161d94;
            }
            0x80161d94 => {
                c.r[0] = c.mfspr(8);
                pc = 0x80161d98;
            }
            0x80161d98 => {
                c.r[5] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x80161d9c;
            }
            0x80161d9c => {
                c.r[6] = c.r[4] | c.r[4];
                pc = 0x80161da0;
            }
            0x80161da0 => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                pc = 0x80161da4;
            }
            0x80161da4 => {
                c.r[0] = c.r[5].wrapping_add(0xffffff80);
                pc = 0x80161da8;
            }
            0x80161da8 => {
                c.cmpu(0, c.r[0], 0x8);
                pc = 0x80161dac;
            }
            0x80161dac => {
                if c.crbit(1) { pc = 0x80161e0c; continue; }
                pc = 0x80161db0;
            }
            0x80161db0 => {
                c.r[5] = 0x80370000;
                pc = 0x80161db4;
            }
            0x80161db4 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80161db8;
            }
            0x80161db8 => {
                c.r[5] = c.r[5].wrapping_add(0xffff9cd4);
                pc = 0x80161dbc;
            }
            0x80161dbc => {
                c.r[0] = c.lwz(c.r[5].wrapping_add(c.r[0]));
                pc = 0x80161dc0;
            }
            0x80161dc0 => {
                c.mtspr(9, c.r[0]);
                pc = 0x80161dc4;
            }
            0x80161dc4 => {
                if true { let t = c.ctr; if (0x80161d90..0x80161e8c).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x80161dc8;
            }
            0x80161dc8 => {
                c.r[6] = 0x80;
                pc = 0x80161dcc;
            }
            0x80161dcc => {
                pc = 0x80161e0c; continue;
                pc = 0x80161dd0;
            }
            0x80161dd0 => {
                c.r[6] = 0x82;
                pc = 0x80161dd4;
            }
            0x80161dd4 => {
                pc = 0x80161e0c; continue;
                pc = 0x80161dd8;
            }
            0x80161dd8 => {
                c.r[6] = 0xa0;
                pc = 0x80161ddc;
            }
            0x80161ddc => {
                pc = 0x80161e0c; continue;
                pc = 0x80161de0;
            }
            0x80161de0 => {
                c.r[6] = 0xa1;
                pc = 0x80161de4;
            }
            0x80161de4 => {
                pc = 0x80161e0c; continue;
                pc = 0x80161de8;
            }
            0x80161de8 => {
                c.r[6] = 0x83;
                pc = 0x80161dec;
            }
            0x80161dec => {
                pc = 0x80161e0c; continue;
                pc = 0x80161df0;
            }
            0x80161df0 => {
                c.r[6] = 0x84;
                pc = 0x80161df4;
            }
            0x80161df4 => {
                pc = 0x80161e0c; continue;
                pc = 0x80161df8;
            }
            0x80161df8 => {
                c.r[6] = 0xa2;
                pc = 0x80161dfc;
            }
            0x80161dfc => {
                pc = 0x80161e0c; continue;
                pc = 0x80161e00;
            }
            0x80161e00 => {
                c.r[6] = 0xa3;
                pc = 0x80161e04;
            }
            0x80161e04 => {
                pc = 0x80161e0c; continue;
                pc = 0x80161e08;
            }
            0x80161e08 => {
                c.r[6] = 0xa4;
                pc = 0x80161e0c;
            }
            0x80161e0c => {
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                pc = 0x80161e10;
            }
            0x80161e10 => {
                c.cmps(0, c.r[0], 0xa1);
                pc = 0x80161e14;
            }
            0x80161e14 => {
                if c.crbit(2) { pc = 0x80161e3c; continue; }
                pc = 0x80161e18;
            }
            0x80161e18 => {
                if !c.crbit(0) { pc = 0x80161e50; continue; }
                pc = 0x80161e1c;
            }
            0x80161e1c => {
                c.cmps(0, c.r[0], 0xa0);
                pc = 0x80161e20;
            }
            0x80161e20 => {
                if !c.crbit(0) { pc = 0x80161e28; continue; }
                pc = 0x80161e24;
            }
            0x80161e24 => {
                pc = 0x80161e50; continue;
                pc = 0x80161e28;
            }
            0x80161e28 => {
                c.r[0] = c.lha(c.r[3].wrapping_add(0x1c4));
                pc = 0x80161e2c;
            }
            0x80161e2c => {
                c.r[3] = c.r[0].rotate_left(1) & 0xfffffffe;
                pc = 0x80161e30;
            }
            0x80161e30 => {
                c.r[0] = c.r[3].wrapping_add(0x2000);
                pc = 0x80161e34;
            }
            0x80161e34 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80161e38;
            }
            0x80161e38 => {
                pc = 0x80161e7c; continue;
                pc = 0x80161e3c;
            }
            0x80161e3c => {
                c.r[0] = c.lha(c.r[3].wrapping_add(0x1d0));
                pc = 0x80161e40;
            }
            0x80161e40 => {
                c.r[3] = c.r[0].rotate_left(1) & 0xfffffffe;
                pc = 0x80161e44;
            }
            0x80161e44 => {
                c.r[0] = c.r[3].wrapping_add(0x2000);
                pc = 0x80161e48;
            }
            0x80161e48 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80161e4c;
            }
            0x80161e4c => {
                pc = 0x80161e7c; continue;
                pc = 0x80161e50;
            }
            0x80161e50 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x121));
                pc = 0x80161e54;
            }
            0x80161e54 => {
                c.cmpu(0, c.r[0], 0xff);
                pc = 0x80161e58;
            }
            0x80161e58 => {
                if c.crbit(2) { pc = 0x80161e74; continue; }
                pc = 0x80161e5c;
            }
            0x80161e5c => {
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x122));
                pc = 0x80161e60;
            }
            0x80161e60 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80161e64;
            }
            0x80161e64 => {
                c.r[4] = c.r[0] | c.r[0];
                pc = 0x80161e68;
            }
            0x80161e68 => {
                c.lr = 0x80161e6c; f_80160bdc(c);
                pc = 0x80161e6c;
            }
            0x80161e6c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80161e70;
            }
            0x80161e70 => {
                pc = 0x80161e78; continue;
                pc = 0x80161e74;
            }
            0x80161e74 => {
                c.r[0] = 0x0;
                pc = 0x80161e78;
            }
            0x80161e78 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80161e7c;
            }
            0x80161e7c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x80161e80;
            }
            0x80161e80 => {
                c.mtspr(8, c.r[0]);
                pc = 0x80161e84;
            }
            0x80161e84 => {
                c.r[1] = c.r[1].wrapping_add(0x10);
                pc = 0x80161e88;
            }
            0x80161e88 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80161d90: bad pc {:#x}", pc),
        }
    }
}

/// inpSetExCtrl (0x80161e8c, 484 bytes)
#[inline(never)]
pub fn f_80161e8c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80161e8c) { return; }
        if c.prof { crate::prof::enter(c, 0x80161e8c); f_80161e8c_b(c, 0x80161e8c); crate::prof::exit(c, 0x80161e8c); return; }
    }
    f_80161e8c_b(c, 0x80161e8c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161e8c_at(c: &mut Cpu, pc: u32) { f_80161e8c_b(c, pc) }
#[inline(always)]
fn f_80161e8c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80161e8c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80161e8c, pc); }
        match pc {
            0x80161e8c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x80161e90;
            }
            0x80161e90 => {
                c.r[0] = c.mfspr(8);
                pc = 0x80161e94;
            }
            0x80161e94 => {
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x80161e98;
            }
            0x80161e98 => {
                c.r[0] = c.r[5] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                pc = 0x80161e9c;
            }
            0x80161e9c => {
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                pc = 0x80161ea0;
            }
            0x80161ea0 => {
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                pc = 0x80161ea4;
            }
            0x80161ea4 => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                pc = 0x80161ea8;
            }
            0x80161ea8 => {
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                pc = 0x80161eac;
            }
            0x80161eac => {
                if !c.crbit(0) { pc = 0x80161eb8; continue; }
                pc = 0x80161eb0;
            }
            0x80161eb0 => {
                c.r[5] = 0x0;
                pc = 0x80161eb4;
            }
            0x80161eb4 => {
                pc = 0x80161ecc; continue;
                pc = 0x80161eb8;
            }
            0x80161eb8 => {
                c.r[0] = c.r[5] as u16 as i16 as i32 as u32;
                pc = 0x80161ebc;
            }
            0x80161ebc => {
                c.r[5] = 0x3fff;
                pc = 0x80161ec0;
            }
            0x80161ec0 => {
                c.cmps(0, c.r[0], 0x3fff);
                pc = 0x80161ec4;
            }
            0x80161ec4 => {
                if c.crbit(1) { pc = 0x80161ecc; continue; }
                pc = 0x80161ec8;
            }
            0x80161ec8 => {
                c.r[5] = c.r[0] | c.r[0];
                pc = 0x80161ecc;
            }
            0x80161ecc => {
                c.r[7] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x80161ed0;
            }
            0x80161ed0 => {
                c.r[6] = c.r[4] | c.r[4];
                pc = 0x80161ed4;
            }
            0x80161ed4 => {
                c.r[0] = c.r[7].wrapping_add(0xffffff80);
                pc = 0x80161ed8;
            }
            0x80161ed8 => {
                c.r[8] = c.r[5] as u16 as i16 as i32 as u32;
                pc = 0x80161edc;
            }
            0x80161edc => {
                c.cmpu(0, c.r[0], 0x8);
                pc = 0x80161ee0;
            }
            0x80161ee0 => {
                if c.crbit(1) { pc = 0x80161f40; continue; }
                pc = 0x80161ee4;
            }
            0x80161ee4 => {
                c.r[5] = 0x80370000;
                pc = 0x80161ee8;
            }
            0x80161ee8 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80161eec;
            }
            0x80161eec => {
                c.r[5] = c.r[5].wrapping_add(0xffff9cf8);
                pc = 0x80161ef0;
            }
            0x80161ef0 => {
                c.r[0] = c.lwz(c.r[5].wrapping_add(c.r[0]));
                pc = 0x80161ef4;
            }
            0x80161ef4 => {
                c.mtspr(9, c.r[0]);
                pc = 0x80161ef8;
            }
            0x80161ef8 => {
                if true { let t = c.ctr; if (0x80161e8c..0x80162070).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x80161efc;
            }
            0x80161efc => {
                c.r[6] = 0x80;
                pc = 0x80161f00;
            }
            0x80161f00 => {
                pc = 0x80161f40; continue;
                pc = 0x80161f04;
            }
            0x80161f04 => {
                c.r[6] = 0x82;
                pc = 0x80161f08;
            }
            0x80161f08 => {
                pc = 0x80161f40; continue;
                pc = 0x80161f0c;
            }
            0x80161f0c => {
                c.r[6] = 0xa0;
                pc = 0x80161f10;
            }
            0x80161f10 => {
                pc = 0x80161f40; continue;
                pc = 0x80161f14;
            }
            0x80161f14 => {
                c.r[6] = 0xa1;
                pc = 0x80161f18;
            }
            0x80161f18 => {
                pc = 0x80161f40; continue;
                pc = 0x80161f1c;
            }
            0x80161f1c => {
                c.r[6] = 0x83;
                pc = 0x80161f20;
            }
            0x80161f20 => {
                pc = 0x80161f40; continue;
                pc = 0x80161f24;
            }
            0x80161f24 => {
                c.r[6] = 0x84;
                pc = 0x80161f28;
            }
            0x80161f28 => {
                pc = 0x80161f40; continue;
                pc = 0x80161f2c;
            }
            0x80161f2c => {
                c.r[6] = 0xa2;
                pc = 0x80161f30;
            }
            0x80161f30 => {
                pc = 0x80161f40; continue;
                pc = 0x80161f34;
            }
            0x80161f34 => {
                c.r[6] = 0xa3;
                pc = 0x80161f38;
            }
            0x80161f38 => {
                pc = 0x80161f40; continue;
                pc = 0x80161f3c;
            }
            0x80161f3c => {
                c.r[6] = 0xa4;
                pc = 0x80161f40;
            }
            0x80161f40 => {
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                pc = 0x80161f44;
            }
            0x80161f44 => {
                c.cmps(0, c.r[0], 0xa2);
                pc = 0x80161f48;
            }
            0x80161f48 => {
                if !c.crbit(0) { pc = 0x80161f54; continue; }
                pc = 0x80161f4c;
            }
            0x80161f4c => {
                c.cmps(0, c.r[0], 0xa0);
                pc = 0x80161f50;
            }
            0x80161f50 => {
                if !c.crbit(0) { pc = 0x80162050; continue; }
                pc = 0x80161f54;
            }
            0x80161f54 => {
                c.r[29] = c.lbz(c.r[3].wrapping_add(0x121));
                pc = 0x80161f58;
            }
            0x80161f58 => {
                c.cmpu(0, c.r[29], 0xff);
                pc = 0x80161f5c;
            }
            0x80161f5c => {
                if c.crbit(2) { pc = 0x80162050; continue; }
                pc = 0x80161f60;
            }
            0x80161f60 => {
                c.r[30] = c.lbz(c.r[3].wrapping_add(0x122));
                pc = 0x80161f64;
            }
            0x80161f64 => {
                if c.crbit(2) { pc = 0x80162050; continue; }
                pc = 0x80161f68;
            }
            0x80161f68 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x80161f6c;
            }
            0x80161f6c => {
                c.cmpu(0, c.r[0], 0x40);
                pc = 0x80161f70;
            }
            0x80161f70 => {
                if !c.crbit(0) { pc = 0x80161fac; continue; }
                pc = 0x80161f74;
            }
            0x80161f74 => {
                c.r[28] = c.r[7].rotate_left(0) & 0x1f;
                pc = 0x80161f78;
            }
            0x80161f78 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80161f7c;
            }
            0x80161f7c => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x80161f80;
            }
            0x80161f80 => {
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80161f84;
            }
            0x80161f84 => {
                c.r[31] = c.r[8].rotate_left(0) & 0xffff;
                pc = 0x80161f88;
            }
            0x80161f88 => {
                c.r[6] = c.r[8].rotate_left(25) & 0xff;
                pc = 0x80161f8c;
            }
            0x80161f8c => {
                c.lr = 0x80161f90; f_801603c0(c);
                pc = 0x80161f90;
            }
            0x80161f90 => {
                c.r[0] = c.r[28].wrapping_add(0x20);
                pc = 0x80161f94;
            }
            0x80161f94 => {
                c.r[6] = c.r[31].rotate_left(0) & 0x7f;
                pc = 0x80161f98;
            }
            0x80161f98 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80161f9c;
            }
            0x80161f9c => {
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80161fa0;
            }
            0x80161fa0 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x80161fa4;
            }
            0x80161fa4 => {
                c.lr = 0x80161fa8; f_801603c0(c);
                pc = 0x80161fa8;
            }
            0x80161fa8 => {
                pc = 0x80162050; continue;
                pc = 0x80161fac;
            }
            0x80161fac => {
                c.cmpu(0, c.r[0], 0x80);
                pc = 0x80161fb0;
            }
            0x80161fb0 => {
                if c.crbit(2) { pc = 0x80161fbc; continue; }
                pc = 0x80161fb4;
            }
            0x80161fb4 => {
                c.cmpu(0, c.r[0], 0x81);
                pc = 0x80161fb8;
            }
            0x80161fb8 => {
                if !c.crbit(2) { pc = 0x80161ff4; continue; }
                pc = 0x80161fbc;
            }
            0x80161fbc => {
                c.r[28] = c.r[7].rotate_left(0) & 0xfe;
                pc = 0x80161fc0;
            }
            0x80161fc0 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80161fc4;
            }
            0x80161fc4 => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x80161fc8;
            }
            0x80161fc8 => {
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80161fcc;
            }
            0x80161fcc => {
                c.r[31] = c.r[8].rotate_left(0) & 0xffff;
                pc = 0x80161fd0;
            }
            0x80161fd0 => {
                c.r[6] = c.r[8].rotate_left(25) & 0xff;
                pc = 0x80161fd4;
            }
            0x80161fd4 => {
                c.lr = 0x80161fd8; f_801603c0(c);
                pc = 0x80161fd8;
            }
            0x80161fd8 => {
                c.r[0] = c.r[28].wrapping_add(0x1);
                pc = 0x80161fdc;
            }
            0x80161fdc => {
                c.r[6] = c.r[31].rotate_left(0) & 0x7f;
                pc = 0x80161fe0;
            }
            0x80161fe0 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80161fe4;
            }
            0x80161fe4 => {
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80161fe8;
            }
            0x80161fe8 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x80161fec;
            }
            0x80161fec => {
                c.lr = 0x80161ff0; f_801603c0(c);
                pc = 0x80161ff0;
            }
            0x80161ff0 => {
                pc = 0x80162050; continue;
                pc = 0x80161ff4;
            }
            0x80161ff4 => {
                c.cmpu(0, c.r[0], 0x84);
                pc = 0x80161ff8;
            }
            0x80161ff8 => {
                if c.crbit(2) { pc = 0x80162004; continue; }
                pc = 0x80161ffc;
            }
            0x80161ffc => {
                c.cmpu(0, c.r[0], 0x85);
                pc = 0x80162000;
            }
            0x80162000 => {
                if !c.crbit(2) { pc = 0x8016203c; continue; }
                pc = 0x80162004;
            }
            0x80162004 => {
                c.r[28] = c.r[7].rotate_left(0) & 0xfe;
                pc = 0x80162008;
            }
            0x80162008 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8016200c;
            }
            0x8016200c => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x80162010;
            }
            0x80162010 => {
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80162014;
            }
            0x80162014 => {
                c.r[31] = c.r[8].rotate_left(0) & 0xffff;
                pc = 0x80162018;
            }
            0x80162018 => {
                c.r[6] = c.r[8].rotate_left(25) & 0xff;
                pc = 0x8016201c;
            }
            0x8016201c => {
                c.lr = 0x80162020; f_801603c0(c);
                pc = 0x80162020;
            }
            0x80162020 => {
                c.r[0] = c.r[28].wrapping_add(0x1);
                pc = 0x80162024;
            }
            0x80162024 => {
                c.r[6] = c.r[31].rotate_left(0) & 0x7f;
                pc = 0x80162028;
            }
            0x80162028 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8016202c;
            }
            0x8016202c => {
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80162030;
            }
            0x80162030 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x80162034;
            }
            0x80162034 => {
                c.lr = 0x80162038; f_801603c0(c);
                pc = 0x80162038;
            }
            0x80162038 => {
                pc = 0x80162050; continue;
                pc = 0x8016203c;
            }
            0x8016203c => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80162040;
            }
            0x80162040 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80162044;
            }
            0x80162044 => {
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80162048;
            }
            0x80162048 => {
                c.r[6] = c.r[8].rotate_left(25) & 0xff;
                pc = 0x8016204c;
            }
            0x8016204c => {
                c.lr = 0x80162050; f_801603c0(c);
                pc = 0x80162050;
            }
            0x80162050 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x80162054;
            }
            0x80162054 => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                pc = 0x80162058;
            }
            0x80162058 => {
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                pc = 0x8016205c;
            }
            0x8016205c => {
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x80162060;
            }
            0x80162060 => {
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x10));
                pc = 0x80162064;
            }
            0x80162064 => {
                c.mtspr(8, c.r[0]);
                pc = 0x80162068;
            }
            0x80162068 => {
                c.r[1] = c.r[1].wrapping_add(0x20);
                pc = 0x8016206c;
            }
            0x8016206c => {
                if true { return; }
                return;
            }
            _ => panic!("f_80161e8c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162070 (0x80162070, 28 bytes)
#[inline(never)]
pub fn f_80162070(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162070) { return; }
        if c.prof { crate::prof::enter(c, 0x80162070); f_80162070_b(c, 0x80162070); crate::prof::exit(c, 0x80162070); return; }
    }
    f_80162070_b(c, 0x80162070)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162070_at(c: &mut Cpu, pc: u32) { f_80162070_b(c, pc) }
#[inline(always)]
fn f_80162070_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162070;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162070, pc); }
        match pc {
            0x80162070 => {
                c.r[3] = 0xa8350000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff83d0));
                c.r[0] = c.r[3].wrapping_add(0x1d63);
                c.r[0] = c.r[4].wrapping_mul(c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff83d0), c.r[0]);
                c.r[3] = c.r[0].rotate_left(26) & 0xffff;
                if true { return; }
                return;
            }
            _ => panic!("f_80162070: bad pc {:#x}", pc),
        }
    }
}

/// sndSin (0x8016208c, 140 bytes)
#[inline(never)]
pub fn f_8016208c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016208c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016208c); f_8016208c_b(c, 0x8016208c); crate::prof::exit(c, 0x8016208c); return; }
    }
    f_8016208c_b(c, 0x8016208c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016208c_at(c: &mut Cpu, pc: u32) { f_8016208c_b(c, pc) }
#[inline(always)]
fn f_8016208c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016208c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016208c, pc); }
        match pc {
            0x8016208c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xfff;
                c.cmpu(0, c.r[0], 0x400);
                if !c.crbit(0) { pc = 0x801620ac; continue; }
                pc = 0x80162098;
            }
            0x80162098 => {
                c.r[3] = 0x80370000;
                c.r[0] = c.r[0].rotate_left(1) & 0x1fffe;
                c.r[3] = c.r[3].wrapping_add(0xffff9d20);
                c.r[3] = c.lha(c.r[3].wrapping_add(c.r[0]));
                if true { return; }
                pc = 0x801620ac;
            }
            0x801620ac => {
                c.cmpu(0, c.r[0], 0x800);
                if !c.crbit(0) { pc = 0x801620d0; continue; }
                pc = 0x801620b4;
            }
            0x801620b4 => {
                c.r[0] = c.r[0].rotate_left(0) & 0x3ff;
                c.r[3] = 0x80370000;
                c.r[0] = c.addc(!c.r[0], 0x3ff, 1);
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[3] = c.r[3].wrapping_add(0xffff9d20);
                c.r[3] = c.lha(c.r[3].wrapping_add(c.r[0]));
                if true { return; }
                pc = 0x801620d0;
            }
            0x801620d0 => {
                c.cmpu(0, c.r[0], 0xc00);
                if !c.crbit(0) { pc = 0x801620f4; continue; }
                pc = 0x801620d8;
            }
            0x801620d8 => {
                c.r[3] = 0x80370000;
                c.r[0] = c.r[0].rotate_left(1) & 0x7fe;
                c.r[3] = c.r[3].wrapping_add(0xffff9d20);
                c.r[0] = c.lha(c.r[3].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].wrapping_neg();
                c.r[3] = c.r[0] as u16 as i16 as i32 as u32;
                if true { return; }
                pc = 0x801620f4;
            }
            0x801620f4 => {
                c.r[0] = c.r[0].rotate_left(0) & 0x3ff;
                c.r[3] = 0x80370000;
                c.r[0] = c.addc(!c.r[0], 0x3ff, 1);
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[3] = c.r[3].wrapping_add(0xffff9d20);
                c.r[0] = c.lha(c.r[3].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].wrapping_neg();
                c.r[3] = c.r[0] as u16 as i16 as i32 as u32;
                if true { return; }
                return;
            }
            _ => panic!("f_8016208c: bad pc {:#x}", pc),
        }
    }
}

/// sndBSearch (0x80162118, 164 bytes)
#[inline(never)]
pub fn f_80162118(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162118) { return; }
        if c.prof { crate::prof::enter(c, 0x80162118); f_80162118_b(c, 0x80162118); crate::prof::exit(c, 0x80162118); return; }
    }
    f_80162118_b(c, 0x80162118)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162118_at(c: &mut Cpu, pc: u32) { f_80162118_b(c, pc) }
#[inline(always)]
fn f_80162118_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162118;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162118, pc); }
        match pc {
            0x80162118 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x8016212c; f_800c47cc(c);
                pc = 0x8016212c;
            }
            0x8016212c => {
                c.cmps(0, c.r[5], 0x0);
                c.r[24] = c.r[3] | c.r[3];
                c.r[25] = c.r[4] | c.r[4];
                c.r[26] = c.r[6] | c.r[6];
                c.r[27] = c.r[7] | c.r[7];
                if c.crbit(2) { pc = 0x801621a0; continue; }
                pc = 0x80162144;
            }
            0x80162144 => {
                c.r[31] = 0x1;
                c.r[30] = c.r[5] | c.r[5];
                pc = 0x8016214c;
            }
            0x8016214c => {
                c.r[0] = c.r[31].wrapping_add(c.r[30]);
                c.r[12] = c.r[27] | c.r[27];
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[29] = (v >> 1) as u32; }
                c.r[3] = c.r[24] | c.r[24];
                c.r[0] = c.r[29].wrapping_add(0xffffffff);
                c.r[0] = c.r[26].wrapping_mul(c.r[0]);
                c.r[28] = c.r[25].wrapping_add(c.r[0]);
                c.r[4] = c.r[28] | c.r[28];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80162174; (c.dispatch)(c, t); }
                pc = 0x80162174;
            }
            0x80162174 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80162184; continue; }
                pc = 0x8016217c;
            }
            0x8016217c => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x801621a4; continue;
                pc = 0x80162184;
            }
            0x80162184 => {
                if !c.crbit(0) { pc = 0x8016218c; continue; }
                pc = 0x80162188;
            }
            0x80162188 => {
                c.r[30] = c.r[29].wrapping_add(0xffffffff);
                pc = 0x8016218c;
            }
            0x8016218c => {
                c.r[0] = c.r[3].rotate_left(1) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80162198; continue; }
                pc = 0x80162194;
            }
            0x80162194 => {
                c.r[31] = c.r[29].wrapping_add(0x1);
                pc = 0x80162198;
            }
            0x80162198 => {
                c.cmps(0, c.r[31], c.r[30]);
                if !c.crbit(1) { pc = 0x8016214c; continue; }
                pc = 0x801621a0;
            }
            0x801621a0 => {
                c.r[3] = 0x0;
                pc = 0x801621a4;
            }
            0x801621a4 => {
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x801621ac; f_800c4818(c);
                pc = 0x801621ac;
            }
            0x801621ac => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80162118: bad pc {:#x}", pc),
        }
    }
}

/// fn_801621BC (0x801621bc, 16 bytes)
#[inline(never)]
pub fn f_801621bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801621bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801621bc); f_801621bc_b(c, 0x801621bc); crate::prof::exit(c, 0x801621bc); return; }
    }
    f_801621bc_b(c, 0x801621bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801621bc_at(c: &mut Cpu, pc: u32) { f_801621bc_b(c, pc) }
#[inline(always)]
fn f_801621bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801621bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801621bc, pc); }
        match pc {
            0x801621bc => {
                c.r[0] = c.lwz(c.r[3]);
                c.r[0] = c.r[0].rotate_left(8) & 0xffffff00;
                c.stw(c.r[3], c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_801621bc: bad pc {:#x}", pc),
        }
    }
}

/// sndConvertTicks (0x801621cc, 72 bytes)
#[inline(never)]
pub fn f_801621cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801621cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801621cc); f_801621cc_b(c, 0x801621cc); crate::prof::exit(c, 0x801621cc); return; }
    }
    f_801621cc_b(c, 0x801621cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801621cc_at(c: &mut Cpu, pc: u32) { f_801621cc_b(c, pc) }
#[inline(always)]
fn f_801621cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801621cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801621cc, pc); }
        match pc {
            0x801621cc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[4] | c.r[4];
                c.lr = 0x801621e8; f_8014a280(c);
                pc = 0x801621e8;
            }
            0x801621e8 => {
                c.r[0] = c.lwz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(16) & 0xffff0000;
                c.r[0] = divwu(c.r[0], c.r[3]);
                c.r[0] = c.r[0].wrapping_mul(0x3e8);
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.stw(c.r[31], c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801621cc: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162214 (0x80162214, 8 bytes)
#[inline(never)]
pub fn f_80162214(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162214) { return; }
        if c.prof { crate::prof::enter(c, 0x80162214); f_80162214_b(c, 0x80162214); crate::prof::exit(c, 0x80162214); return; }
    }
    f_80162214_b(c, 0x80162214)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162214_at(c: &mut Cpu, pc: u32) { f_80162214_b(c, pc) }
#[inline(always)]
fn f_80162214_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162214;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162214, pc); }
        match pc {
            0x80162214 => {
                c.r[3] = c.r[3].rotate_left(24) & 0xffffff;
                if true { return; }
                return;
            }
            _ => panic!("f_80162214: bad pc {:#x}", pc),
        }
    }
}

/// snd_handle_irq (0x8016221c, 340 bytes)
#[inline(never)]
pub fn f_8016221c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016221c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016221c); f_8016221c_b(c, 0x8016221c); crate::prof::exit(c, 0x8016221c); return; }
    }
    f_8016221c_b(c, 0x8016221c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016221c_at(c: &mut Cpu, pc: u32) { f_8016221c_b(c, pc) }
#[inline(always)]
fn f_8016221c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016221c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016221c, pc); }
        match pc {
            0x8016221c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa6f8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8016235c; continue; }
                pc = 0x80162238;
            }
            0x80162238 => {
                c.lr = 0x8016223c; f_8014e7cc(c);
                pc = 0x8016223c;
            }
            0x8016223c => {
                c.lr = 0x80162240; f_80164398(c);
                pc = 0x80162240;
            }
            0x80162240 => {
                c.lr = 0x80162244; f_80164118(c);
                pc = 0x80162244;
            }
            0x80162244 => {
                c.lr = 0x80162248; f_80164238(c);
                pc = 0x80162248;
            }
            0x80162248 => {
                c.lr = 0x8016224c; f_801643b8(c);
                pc = 0x8016224c;
            }
            0x8016224c => {
                c.lr = 0x80162250; f_80164398(c);
                pc = 0x80162250;
            }
            0x80162250 => {
                c.lr = 0x80162254; f_8015d678(c);
                pc = 0x80162254;
            }
            0x80162254 => {
                c.lr = 0x80162258; f_801643b8(c);
                pc = 0x80162258;
            }
            0x80162258 => {
                c.lr = 0x8016225c; f_80164398(c);
                pc = 0x8016225c;
            }
            0x8016225c => {
                c.r[4] = c.lbz(c.r[13].wrapping_add(0xffffa83e));
                c.r[3] = 0x55550000;
                c.r[0] = c.r[3].wrapping_add(0x5556);
                c.r[5] = c.lbz(c.r[13].wrapping_add(0xffffa83f));
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[9] = 0x0;
                c.r[3] = mulhw(c.r[0], c.r[4]);
                c.r[0] = c.r[5] ^ 0x1;
                c.stb(c.r[13].wrapping_add(0xffffa83f), c.r[0]);
                c.r[8] = 0x0;
                c.r[0] = c.r[3].rotate_left(1) & 0x1;
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.r[0].wrapping_mul(0x3);
                c.r[0] = c.r[4].wrapping_sub(c.r[0]);
                c.stb(c.r[13].wrapping_add(0xffffa83e), c.r[0]);
                pc = 0x801622e4; continue;
                pc = 0x8016229c;
            }
            0x8016229c => {
                c.r[0] = c.r[9].rotate_left(0) & 0xff;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[0].wrapping_mul(0xf4);
                c.r[9] = c.r[9].wrapping_add(0x1);
                c.r[7] = c.r[4].wrapping_add(0x24);
                c.stw(c.r[3].wrapping_add(c.r[7]), c.r[8]);
                c.r[5] = c.r[7].wrapping_add(0x4);
                c.r[4] = c.r[7].wrapping_add(0x8);
                c.r[3] = c.r[7].wrapping_add(0xc);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[0] = c.r[7].wrapping_add(0x10);
                c.stw(c.r[6].wrapping_add(c.r[5]), c.r[8]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.stw(c.r[5].wrapping_add(c.r[4]), c.r[8]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.stw(c.r[4].wrapping_add(c.r[3]), c.r[8]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[8]);
                pc = 0x801622e4;
            }
            0x801622e4 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa83d));
                c.r[3] = c.r[9].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8016229c; continue; }
                pc = 0x801622f4;
            }
            0x801622f4 => {
                c.lr = 0x801622f8; f_801643b8(c);
                pc = 0x801622f8;
            }
            0x801622f8 => {
                c.r[31] = 0x0;
                pc = 0x80162324; continue;
                pc = 0x80162300;
            }
            0x80162300 => {
                c.lr = 0x80162304; f_80164398(c);
                pc = 0x80162304;
            }
            0x80162304 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8016230c; f_8016245c(c);
                pc = 0x8016230c;
            }
            0x8016230c => {
                c.r[3] = 0x100;
                c.lr = 0x80162314; f_801496a0(c);
                pc = 0x80162314;
            }
            0x80162314 => {
                c.r[3] = 0x100;
                c.lr = 0x8016231c; f_8014c0a4(c);
                pc = 0x8016231c;
            }
            0x8016231c => {
                c.lr = 0x80162320; f_801643b8(c);
                pc = 0x80162320;
            }
            0x80162320 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80162324;
            }
            0x80162324 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x5);
                if c.crbit(0) { pc = 0x80162300; continue; }
                pc = 0x80162330;
            }
            0x80162330 => {
                c.lr = 0x80162334; f_80164398(c);
                pc = 0x80162334;
            }
            0x80162334 => {
                c.r[3] = 0x0;
                c.lr = 0x8016233c; f_8016245c(c);
                pc = 0x8016233c;
            }
            0x8016233c => {
                c.lr = 0x80162340; f_8015f620(c);
                pc = 0x80162340;
            }
            0x80162340 => {
                c.lr = 0x80162344; f_801643b8(c);
                pc = 0x80162344;
            }
            0x80162344 => {
                c.lr = 0x80162348; f_80164398(c);
                pc = 0x80162348;
            }
            0x80162348 => {
                c.lr = 0x8016234c; f_8014df20(c);
                pc = 0x8016234c;
            }
            0x8016234c => {
                c.lr = 0x80162350; f_801643b8(c);
                pc = 0x80162350;
            }
            0x80162350 => {
                c.lr = 0x80162354; f_80164398(c);
                pc = 0x80162354;
            }
            0x80162354 => {
                c.lr = 0x80162358; f_80159a38(c);
                pc = 0x80162358;
            }
            0x80162358 => {
                c.lr = 0x8016235c; f_801643b8(c);
                pc = 0x8016235c;
            }
            0x8016235c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8016221c: bad pc {:#x}", pc),
        }
    }
}

/// hwInit (0x80162370, 184 bytes)
#[inline(never)]
pub fn f_80162370(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162370) { return; }
        if c.prof { crate::prof::enter(c, 0x80162370); f_80162370_b(c, 0x80162370); crate::prof::exit(c, 0x80162370); return; }
    }
    f_80162370_b(c, 0x80162370)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162370_at(c: &mut Cpu, pc: u32) { f_80162370_b(c, pc) }
#[inline(always)]
fn f_80162370_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162370;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162370, pc); }
        match pc {
            0x80162370 => {
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
                c.lr = 0x801623a0; f_801642f8(c);
                pc = 0x801623a0;
            }
            0x801623a0 => {
                c.r[0] = 0x0;
                c.r[3] = 0x80160000;
                c.stb(c.r[13].wrapping_add(0xffffa83f), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x221c);
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.stb(c.r[13].wrapping_add(0xffffa83e), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa808), c.r[0]);
                c.lr = 0x801623c4; f_80163ffc(c);
                pc = 0x801623c4;
            }
            0x801623c4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80162404; continue; }
                pc = 0x801623cc;
            }
            0x801623cc => {
                c.r[5] = c.r[31].rotate_left(0) & 0x1;
                c.r[3] = c.r[29].rotate_left(0) & 0xff;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                c.lr = 0x801623dc; f_8015a484(c);
                pc = 0x801623dc;
            }
            0x801623dc => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80162404; continue; }
                pc = 0x801623e4;
            }
            0x801623e4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801623ec; f_80164148(c);
                pc = 0x801623ec;
            }
            0x801623ec => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80162404; continue; }
                pc = 0x801623f4;
            }
            0x801623f4 => {
                c.lr = 0x801623f8; f_80164328(c);
                pc = 0x801623f8;
            }
            0x801623f8 => {
                c.lr = 0x801623fc; f_801640c4(c);
                pc = 0x801623fc;
            }
            0x801623fc => {
                c.r[3] = 0x0;
                pc = 0x80162408; continue;
                pc = 0x80162404;
            }
            0x80162404 => {
                c.r[3] = 0xffffffff;
                pc = 0x80162408;
            }
            0x80162408 => {
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
            _ => panic!("f_80162370: bad pc {:#x}", pc),
        }
    }
}

/// hwExit (0x80162428, 52 bytes)
#[inline(never)]
pub fn f_80162428(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162428) { return; }
        if c.prof { crate::prof::enter(c, 0x80162428); f_80162428_b(c, 0x80162428); crate::prof::exit(c, 0x80162428); return; }
    }
    f_80162428_b(c, 0x80162428)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162428_at(c: &mut Cpu, pc: u32) { f_80162428_b(c, pc) }
#[inline(always)]
fn f_80162428_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162428;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162428, pc); }
        match pc {
            0x80162428 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80162438; f_80164360(c);
                pc = 0x80162438;
            }
            0x80162438 => {
                c.lr = 0x8016243c; f_80164204(c);
                pc = 0x8016243c;
            }
            0x8016243c => {
                c.lr = 0x80162440; f_8015a870(c);
                pc = 0x80162440;
            }
            0x80162440 => {
                c.lr = 0x80162444; f_801640e4(c);
                pc = 0x80162444;
            }
            0x80162444 => {
                c.lr = 0x80162448; f_80164328(c);
                pc = 0x80162448;
            }
            0x80162448 => {
                c.lr = 0x8016244c; f_80164324(c);
                pc = 0x8016244c;
            }
            0x8016244c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80162428: bad pc {:#x}", pc),
        }
    }
}

/// fn_8016245C (0x8016245c, 8 bytes)
#[inline(never)]
pub fn f_8016245c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016245c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016245c); f_8016245c_b(c, 0x8016245c); crate::prof::exit(c, 0x8016245c); return; }
    }
    f_8016245c_b(c, 0x8016245c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016245c_at(c: &mut Cpu, pc: u32) { f_8016245c_b(c, pc) }
#[inline(always)]
fn f_8016245c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016245c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016245c, pc); }
        match pc {
            0x8016245c => {
                c.stb(c.r[13].wrapping_add(0xffffa830), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8016245c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162464 (0x80162464, 8 bytes)
#[inline(never)]
pub fn f_80162464(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162464) { return; }
        if c.prof { crate::prof::enter(c, 0x80162464); f_80162464_b(c, 0x80162464); crate::prof::exit(c, 0x80162464); return; }
    }
    f_80162464_b(c, 0x80162464)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162464_at(c: &mut Cpu, pc: u32) { f_80162464_b(c, pc) }
#[inline(always)]
fn f_80162464_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162464;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162464, pc); }
        match pc {
            0x80162464 => {
                c.r[3] = c.lbz(c.r[13].wrapping_add(0xffffa830));
                if true { return; }
                return;
            }
            _ => panic!("f_80162464: bad pc {:#x}", pc),
        }
    }
}

/// fn_8016246C (0x8016246c, 32 bytes)
#[inline(never)]
pub fn f_8016246c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016246c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016246c); f_8016246c_b(c, 0x8016246c); crate::prof::exit(c, 0x8016246c); return; }
    }
    f_8016246c_b(c, 0x8016246c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016246c_at(c: &mut Cpu, pc: u32) { f_8016246c_b(c, pc) }
#[inline(always)]
fn f_8016246c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016246c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016246c, pc); }
        match pc {
            0x8016246c => {
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xec));
                c.r[0] = c.r[3].wrapping_neg();
                c.r[0] = c.r[0] | c.r[3];
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8016246c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8016248C (0x8016248c, 8 bytes)
#[inline(never)]
pub fn f_8016248c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016248c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016248c); f_8016248c_b(c, 0x8016248c); crate::prof::exit(c, 0x8016248c); return; }
    }
    f_8016248c_b(c, 0x8016248c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016248c_at(c: &mut Cpu, pc: u32) { f_8016248c_b(c, pc) }
#[inline(always)]
fn f_8016248c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016248c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016248c, pc); }
        match pc {
            0x8016248c => {
                c.stw(c.r[13].wrapping_add(0xffffa808), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8016248c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162494 (0x80162494, 20 bytes)
#[inline(never)]
pub fn f_80162494(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162494) { return; }
        if c.prof { crate::prof::enter(c, 0x80162494); f_80162494_b(c, 0x80162494); crate::prof::exit(c, 0x80162494); return; }
    }
    f_80162494_b(c, 0x80162494)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162494_at(c: &mut Cpu, pc: u32) { f_80162494_b(c, pc) }
#[inline(always)]
fn f_80162494_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162494;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162494, pc); }
        match pc {
            0x80162494 => {
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1c), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80162494: bad pc {:#x}", pc),
        }
    }
}

/// hwInitSamplePlayback (0x801624a8, 436 bytes)
#[inline(never)]
pub fn f_801624a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801624a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801624a8); f_801624a8_b(c, 0x801624a8); crate::prof::exit(c, 0x801624a8); return; }
    }
    f_801624a8_b(c, 0x801624a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801624a8_at(c: &mut Cpu, pc: u32) { f_801624a8_b(c, pc) }
#[inline(always)]
fn f_801624a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801624a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801624a8, pc); }
        match pc {
            0x801624a8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[12] = 0x0;
                c.r[11] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[10] | c.r[10];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[30].wrapping_mul(0xf4);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = 0x0;
                pc = 0x80162500; continue;
                pc = 0x801624dc;
            }
            0x801624dc => {
                c.r[10] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[0] = c.r[12].rotate_left(2) & 0x3fc;
                c.r[12] = c.r[12].wrapping_add(0x1);
                c.r[10] = c.r[10].wrapping_add(c.r[0]);
                c.r[10] = c.r[10].wrapping_add(0x24);
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[10]));
                c.r[0] = c.r[0].rotate_left(0) & 0x20;
                c.stw(c.r[3].wrapping_add(c.r[10]), c.r[11]);
                c.r[29] = c.r[29] | c.r[0];
                pc = 0x80162500;
            }
            0x80162500 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa830));
                c.r[10] = c.r[12].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[10], c.r[0]);
                if !c.crbit(1) { pc = 0x801624dc; continue; }
                pc = 0x80162510;
            }
            0x80162510 => {
                c.r[10] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[0] = 0x0;
                c.cmpu(0, c.r[6], 0x0);
                c.r[6] = c.r[10].wrapping_add(c.r[3]);
                c.stw(c.r[6].wrapping_add(0x24), c.r[29]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[6] = c.r[6].wrapping_add(c.r[3]);
                c.stw(c.r[6].wrapping_add(0x1c), c.r[7]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[6] = c.r[6].wrapping_add(c.r[3]);
                c.stw(c.r[6].wrapping_add(0x18), c.r[8]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[6] = c.r[6].wrapping_add(c.r[3]);
                c.stw(c.r[6].wrapping_add(0xf0), c.r[0]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[6] = c.r[6].wrapping_add(c.r[3]);
                c.sth(c.r[6].wrapping_add(0x70), c.r[4]);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[6] = c.lwz(c.r[5]);
                c.r[4] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[7] = c.r[7].wrapping_add(c.r[3]);
                c.stw(c.r[7].wrapping_add(0x74), c.r[6]);
                c.stw(c.r[7].wrapping_add(0x78), c.r[4]);
                c.r[6] = c.lwz(c.r[5].wrapping_add(0x8));
                c.r[4] = c.lwz(c.r[5].wrapping_add(0xc));
                c.stw(c.r[7].wrapping_add(0x7c), c.r[6]);
                c.stw(c.r[7].wrapping_add(0x80), c.r[4]);
                c.r[6] = c.lwz(c.r[5].wrapping_add(0x10));
                c.r[4] = c.lwz(c.r[5].wrapping_add(0x14));
                c.stw(c.r[7].wrapping_add(0x84), c.r[6]);
                c.stw(c.r[7].wrapping_add(0x88), c.r[4]);
                c.r[6] = c.lwz(c.r[5].wrapping_add(0x18));
                c.r[4] = c.lwz(c.r[5].wrapping_add(0x1c));
                c.stw(c.r[7].wrapping_add(0x8c), c.r[6]);
                c.stw(c.r[7].wrapping_add(0x90), c.r[4]);
                if c.crbit(2) { pc = 0x801625e0; continue; }
                pc = 0x801625a0;
            }
            0x801625a0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[5] = 0x7fff;
                c.r[4] = c.r[4].wrapping_add(c.r[3]);
                c.stb(c.r[4].wrapping_add(0xa4), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[4].wrapping_add(c.r[3]);
                c.stw(c.r[4].wrapping_add(0xb8), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[4].wrapping_add(c.r[3]);
                c.stw(c.r[4].wrapping_add(0xbc), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[4].wrapping_add(c.r[3]);
                c.sth(c.r[4].wrapping_add(0xc0), c.r[5]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[4].wrapping_add(c.r[3]);
                c.stw(c.r[4].wrapping_add(0xc4), c.r[0]);
                pc = 0x801625e0;
            }
            0x801625e0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[5] = 0xff;
                c.cmpu(0, c.r[9], 0x0);
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                c.stb(c.r[4].wrapping_add(0xe4), c.r[5]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                c.stb(c.r[4].wrapping_add(0xe5), c.r[5]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                c.stb(c.r[4].wrapping_add(0xe6), c.r[5]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[3]);
                c.stb(c.r[3].wrapping_add(0xe7), c.r[5]);
                if c.crbit(2) { pc = 0x80162634; continue; }
                pc = 0x8016261c;
            }
            0x8016261c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x80162628; f_801629a4(c);
                pc = 0x80162628;
            }
            0x80162628 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x1;
                c.lr = 0x80162634; f_801629d0(c);
                pc = 0x80162634;
            }
            0x80162634 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80162640; f_801629fc(c);
                pc = 0x80162640;
            }
            0x80162640 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801624a8: bad pc {:#x}", pc),
        }
    }
}

/// hwBreak (0x8016265c, 80 bytes)
#[inline(never)]
pub fn f_8016265c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016265c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016265c); f_8016265c_b(c, 0x8016265c); crate::prof::exit(c, 0x8016265c); return; }
    }
    f_8016265c_b(c, 0x8016265c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016265c_at(c: &mut Cpu, pc: u32) { f_8016265c_b(c, pc) }
#[inline(always)]
fn f_8016265c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016265c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016265c, pc); }
        match pc {
            0x8016265c => {
                c.r[4] = c.r[3].wrapping_mul(0xf4);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0xec));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80162688; continue; }
                pc = 0x80162674;
            }
            0x80162674 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa830));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80162688; continue; }
                pc = 0x80162680;
            }
            0x80162680 => {
                c.r[0] = 0x1;
                c.stb(c.r[3].wrapping_add(0xee), c.r[0]);
                pc = 0x80162688;
            }
            0x80162688 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa830));
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x20;
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8016265c: bad pc {:#x}", pc),
        }
    }
}

/// hwSetADSR (0x801626ac, 428 bytes)
#[inline(never)]
pub fn f_801626ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801626ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801626ac); f_801626ac_b(c, 0x801626ac); crate::prof::exit(c, 0x801626ac); return; }
    }
    f_801626ac_b(c, 0x801626ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801626ac_at(c: &mut Cpu, pc: u32) { f_801626ac_b(c, pc) }
#[inline(always)]
fn f_801626ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801626ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801626ac, pc); }
        match pc {
            0x801626ac => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = c.r[5].rotate_left(0) & 0xff;
                c.rc(c.r[6]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x801626e4; continue; }
                pc = 0x801626d4;
            }
            0x801626d4 => {
                if c.crbit(0) { pc = 0x80162824; continue; }
                pc = 0x801626d8;
            }
            0x801626d8 => {
                c.cmps(0, c.r[6], 0x3);
                if !c.crbit(0) { pc = 0x80162824; continue; }
                pc = 0x801626e0;
            }
            0x801626e0 => {
                pc = 0x8016274c; continue;
                pc = 0x801626e4;
            }
            0x801626e4 => {
                c.r[5] = c.r[29].wrapping_mul(0xf4);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = 0x0;
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stb(c.r[3].wrapping_add(0xa4), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.lhz(c.r[30]);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stw(c.r[3].wrapping_add(0xb8), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.lhz(c.r[30].wrapping_add(0x2));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stw(c.r[3].wrapping_add(0xbc), c.r[4]);
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x4));
                c.r[4] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.cmpu(0, c.r[4], 0x7fff);
                if !c.crbit(1) { pc = 0x8016272c; continue; }
                pc = 0x80162728;
            }
            0x80162728 => {
                c.r[4] = 0x7fff;
                pc = 0x8016272c;
            }
            0x8016272c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.sth(c.r[3].wrapping_add(0xc0), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.lhz(c.r[30].wrapping_add(0x6));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stw(c.r[3].wrapping_add(0xc4), c.r[4]);
                pc = 0x80162824; continue;
                pc = 0x8016274c;
            }
            0x8016274c => {
                c.r[31] = c.r[29].wrapping_mul(0xf4);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[5] = 0x1;
                c.cmpu(0, c.r[6], 0x1);
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.r[4] = 0x0;
                c.stb(c.r[3].wrapping_add(0xa4), c.r[5]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.stb(c.r[3].wrapping_add(0xca), c.r[4]);
                if !c.crbit(2) { pc = 0x801627dc; continue; }
                pc = 0x80162778;
            }
            0x80162778 => {
                c.r[3] = c.lwz(c.r[30]);
                c.lr = 0x80162780; f_80158cd4(c);
                pc = 0x80162780;
            }
            0x80162780 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[3].wrapping_add(0xb8), c.r[4]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x4));
                c.lr = 0x80162798; f_80158cd4(c);
                pc = 0x80162798;
            }
            0x80162798 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[3].wrapping_add(0xbc), c.r[4]);
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x8));
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 30 != 0); c.r[4] = (v >> 2) as u32; }
                c.cmpu(0, c.r[4], 0x3ff);
                if !c.crbit(1) { pc = 0x801627bc; continue; }
                pc = 0x801627b8;
            }
            0x801627b8 => {
                c.r[4] = 0x3ff;
                pc = 0x801627bc;
            }
            0x801627bc => {
                c.r[3] = 0x80370000;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(0xffff944c);
                c.r[4] = c.lbz(c.r[3].wrapping_add(c.r[4]));
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.r[0] = c.addc(!c.r[4], 0xc1, 1);
                c.sth(c.r[3].wrapping_add(0xc0), c.r[0]);
                pc = 0x80162814; continue;
                pc = 0x801627dc;
            }
            0x801627dc => {
                c.r[3] = c.lwz(c.r[30]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[3].wrapping_add(0xb8), c.r[4]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[3].wrapping_add(0xbc), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.lhz(c.r[30].wrapping_add(0x8));
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.sth(c.r[3].wrapping_add(0xc0), c.r[4]);
                pc = 0x80162814;
            }
            0x80162814 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.lhz(c.r[30].wrapping_add(0xa));
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[3].wrapping_add(0xc4), c.r[4]);
                pc = 0x80162824;
            }
            0x80162824 => {
                c.r[0] = c.r[29].wrapping_mul(0xf4);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x10;
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801626ac: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162858 (0x80162858, 32 bytes)
#[inline(never)]
pub fn f_80162858(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162858) { return; }
        if c.prof { crate::prof::enter(c, 0x80162858); f_80162858_b(c, 0x80162858); crate::prof::exit(c, 0x80162858); return; }
    }
    f_80162858_b(c, 0x80162858)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162858_at(c: &mut Cpu, pc: u32) { f_80162858_b(c, pc) }
#[inline(always)]
fn f_80162858_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162858;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162858, pc); }
        match pc {
            0x80162858 => {
                c.r[6] = c.r[3].wrapping_mul(0xf4);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                c.stw(c.r[3].wrapping_add(0x94), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                c.stw(c.r[3].wrapping_add(0x98), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_80162858: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162878 (0x80162878, 20 bytes)
#[inline(never)]
pub fn f_80162878(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162878) { return; }
        if c.prof { crate::prof::enter(c, 0x80162878); f_80162878_b(c, 0x80162878); crate::prof::exit(c, 0x80162878); return; }
    }
    f_80162878_b(c, 0x80162878)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162878_at(c: &mut Cpu, pc: u32) { f_80162878_b(c, pc) }
#[inline(always)]
fn f_80162878_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162878;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162878, pc); }
        match pc {
            0x80162878 => {
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x9c));
                if true { return; }
                return;
            }
            _ => panic!("f_80162878: bad pc {:#x}", pc),
        }
    }
}

/// fn_8016288C (0x8016288c, 20 bytes)
#[inline(never)]
pub fn f_8016288c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016288c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016288c); f_8016288c_b(c, 0x8016288c); crate::prof::exit(c, 0x8016288c); return; }
    }
    f_8016288c_b(c, 0x8016288c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016288c_at(c: &mut Cpu, pc: u32) { f_8016288c_b(c, pc) }
#[inline(always)]
fn f_8016288c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016288c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016288c, pc); }
        match pc {
            0x8016288c => {
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x90));
                if true { return; }
                return;
            }
            _ => panic!("f_8016288c: bad pc {:#x}", pc),
        }
    }
}

/// fn_801628A0 (0x801628a0, 20 bytes)
#[inline(never)]
pub fn f_801628a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801628a0) { return; }
        if c.prof { crate::prof::enter(c, 0x801628a0); f_801628a0_b(c, 0x801628a0); crate::prof::exit(c, 0x801628a0); return; }
    }
    f_801628a0_b(c, 0x801628a0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801628a0_at(c: &mut Cpu, pc: u32) { f_801628a0_b(c, pc) }
#[inline(always)]
fn f_801628a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801628a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801628a0, pc); }
        match pc {
            0x801628a0 => {
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x70));
                if true { return; }
                return;
            }
            _ => panic!("f_801628a0: bad pc {:#x}", pc),
        }
    }
}

/// fn_801628B4 (0x801628b4, 20 bytes)
#[inline(never)]
pub fn f_801628b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801628b4) { return; }
        if c.prof { crate::prof::enter(c, 0x801628b4); f_801628b4_b(c, 0x801628b4); crate::prof::exit(c, 0x801628b4); return; }
    }
    f_801628b4_b(c, 0x801628b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801628b4_at(c: &mut Cpu, pc: u32) { f_801628b4_b(c, pc) }
#[inline(always)]
fn f_801628b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801628b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801628b4, pc); }
        match pc {
            0x801628b4 => {
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0xa0), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_801628b4: bad pc {:#x}", pc),
        }
    }
}

/// hwStart (0x801628c8, 60 bytes)
#[inline(never)]
pub fn f_801628c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801628c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801628c8); f_801628c8_b(c, 0x801628c8); crate::prof::exit(c, 0x801628c8); return; }
    }
    f_801628c8_b(c, 0x801628c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801628c8_at(c: &mut Cpu, pc: u32) { f_801628c8_b(c, pc) }
#[inline(always)]
fn f_801628c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801628c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801628c8, pc); }
        match pc {
            0x801628c8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = c.r[3].wrapping_mul(0xf4);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[5] = c.lbz(c.r[13].wrapping_add(0xffffa830));
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                c.stb(c.r[3].wrapping_add(0xd4), c.r[5]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                c.lr = 0x801628f4; f_8015d44c(c);
                pc = 0x801628f4;
            }
            0x801628f4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801628c8: bad pc {:#x}", pc),
        }
    }
}

/// hwKeyOff (0x80162904, 40 bytes)
#[inline(never)]
pub fn f_80162904(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162904) { return; }
        if c.prof { crate::prof::enter(c, 0x80162904); f_80162904_b(c, 0x80162904); crate::prof::exit(c, 0x80162904); return; }
    }
    f_80162904_b(c, 0x80162904)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162904_at(c: &mut Cpu, pc: u32) { f_80162904_b(c, pc) }
#[inline(always)]
fn f_80162904_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162904;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162904, pc); }
        match pc {
            0x80162904 => {
                c.r[3] = c.r[3].wrapping_mul(0xf4);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa830));
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x40;
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80162904: bad pc {:#x}", pc),
        }
    }
}

/// hwSetPitch (0x8016292c, 120 bytes)
#[inline(never)]
pub fn f_8016292c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016292c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016292c); f_8016292c_b(c, 0x8016292c); crate::prof::exit(c, 0x8016292c); return; }
    }
    f_8016292c_b(c, 0x8016292c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016292c_at(c: &mut Cpu, pc: u32) { f_8016292c_b(c, pc) }
#[inline(always)]
fn f_8016292c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016292c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016292c, pc); }
        match pc {
            0x8016292c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_mul(0xf4);
                c.cmpu(0, c.r[0], 0x4000);
                c.r[5] = c.r[5].wrapping_add(c.r[3]);
                if c.crbit(0) { pc = 0x80162948; continue; }
                pc = 0x80162944;
            }
            0x80162944 => {
                c.r[4] = 0x3fff;
                pc = 0x80162948;
            }
            0x80162948 => {
                c.r[0] = c.lbz(c.r[5].wrapping_add(0xe4));
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x8016296c; continue; }
                pc = 0x80162954;
            }
            0x80162954 => {
                c.r[3] = c.r[0].rotate_left(2) & 0x3fc;
                c.r[0] = c.r[4].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x38));
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(2) { return; }
                pc = 0x8016296c;
            }
            0x8016296c => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa830));
                c.r[4] = c.r[4].rotate_left(4) & 0xffff0;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[5].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x38), c.r[4]);
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa830));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x8;
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa830));
                c.stb(c.r[5].wrapping_add(0xe4), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8016292c: bad pc {:#x}", pc),
        }
    }
}

/// fn_801629A4 (0x801629a4, 44 bytes)
#[inline(never)]
pub fn f_801629a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801629a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801629a4); f_801629a4_b(c, 0x801629a4); crate::prof::exit(c, 0x801629a4); return; }
    }
    f_801629a4_b(c, 0x801629a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801629a4_at(c: &mut Cpu, pc: u32) { f_801629a4_b(c, pc) }
#[inline(always)]
fn f_801629a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801629a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801629a4, pc); }
        match pc {
            0x801629a4 => {
                c.r[5] = c.r[3].wrapping_mul(0xf4);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[13].wrapping_add(0xffff83d8);
                c.r[0] = c.lhz(c.r[3].wrapping_add(c.r[0]));
                c.r[3] = c.r[6].wrapping_add(c.r[5]);
                c.sth(c.r[3].wrapping_add(0xcc), c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x100;
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_801629a4: bad pc {:#x}", pc),
        }
    }
}

/// fn_801629D0 (0x801629d0, 44 bytes)
#[inline(never)]
pub fn f_801629d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801629d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801629d0); f_801629d0_b(c, 0x801629d0); crate::prof::exit(c, 0x801629d0); return; }
    }
    f_801629d0_b(c, 0x801629d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801629d0_at(c: &mut Cpu, pc: u32) { f_801629d0_b(c, pc) }
#[inline(always)]
fn f_801629d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801629d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801629d0, pc); }
        match pc {
            0x801629d0 => {
                c.r[5] = c.r[3].wrapping_mul(0xf4);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[13].wrapping_add(0xffff83e0);
                c.r[0] = c.lhz(c.r[3].wrapping_add(c.r[0]));
                c.r[3] = c.r[6].wrapping_add(c.r[5]);
                c.sth(c.r[3].wrapping_add(0xce), c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x80;
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_801629d0: bad pc {:#x}", pc),
        }
    }
}

/// hwSetITDMode (0x801629fc, 92 bytes)
#[inline(never)]
pub fn f_801629fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801629fc) { return; }
        if c.prof { crate::prof::enter(c, 0x801629fc); f_801629fc_b(c, 0x801629fc); crate::prof::exit(c, 0x801629fc); return; }
    }
    f_801629fc_b(c, 0x801629fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801629fc_at(c: &mut Cpu, pc: u32) { f_801629fc_b(c, pc) }
#[inline(always)]
fn f_801629fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801629fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801629fc, pc); }
        match pc {
            0x801629fc => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80162a3c; continue; }
                pc = 0x80162a04;
            }
            0x80162a04 => {
                c.r[5] = c.r[3].wrapping_mul(0xf4);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = 0x10;
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xf0));
                c.r[0] = c.r[0] | 0x80000000;
                c.stw(c.r[3].wrapping_add(0xf0), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.sth(c.r[3].wrapping_add(0xd0), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.sth(c.r[3].wrapping_add(0xd2), c.r[4]);
                if true { return; }
                pc = 0x80162a3c;
            }
            0x80162a3c => {
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xf0));
                c.r[0] = c.r[0].rotate_left(0) & 0x7fffffff;
                c.stw(c.r[3].wrapping_add(0xf0), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_801629fc: bad pc {:#x}", pc),
        }
    }
}

/// hwSetVolume (0x80162a58, 704 bytes)
#[inline(never)]
pub fn f_80162a58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162a58) { return; }
        if c.prof { crate::prof::enter(c, 0x80162a58); f_80162a58_b(c, 0x80162a58); crate::prof::exit(c, 0x80162a58); return; }
    }
    f_80162a58_b(c, 0x80162a58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162a58_at(c: &mut Cpu, pc: u32) { f_80162a58_b(c, pc) }
#[inline(always)]
fn f_80162a58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162a58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162a58, pc); }
        match pc {
            0x80162a58 => {
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffff9e38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.fcmp(0, c.f[1], c.f[0]);
                c.stw(c.r[1].wrapping_add(0x4c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x48), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[31] = c.r[3].wrapping_add(c.r[0]);
                { let (a, b) = (c.crbit(1), c.crbit(2)); c.set_crbit(2, a | b); }
                if !c.crbit(2) { pc = 0x80162a90; continue; }
                pc = 0x80162a8c;
            }
            0x80162a8c => {
                c.f[1] = c.f[0];
                pc = 0x80162a90;
            }
            0x80162a90 => {
                { let ea = c.r[2].wrapping_add(0xffff9e38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[2], c.f[0]);
                { let (a, b) = (c.crbit(1), c.crbit(2)); c.set_crbit(2, a | b); }
                if !c.crbit(2) { pc = 0x80162aa4; continue; }
                pc = 0x80162aa0;
            }
            0x80162aa0 => {
                c.f[2] = c.f[0];
                pc = 0x80162aa4;
            }
            0x80162aa4 => {
                { let ea = c.r[2].wrapping_add(0xffff9e38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[3], c.f[0]);
                { let (a, b) = (c.crbit(1), c.crbit(2)); c.set_crbit(2, a | b); }
                if !c.crbit(2) { pc = 0x80162ab8; continue; }
                pc = 0x80162ab4;
            }
            0x80162ab4 => {
                c.f[3] = c.f[0];
                pc = 0x80162ab8;
            }
            0x80162ab8 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0xef));
                c.r[3] = 0x80440000;
                c.r[7] = c.r[3].wrapping_add(0x7e60);
                c.r[8] = c.lwz(c.r[31].wrapping_add(0xf0));
                c.r[0] = c.r[0].wrapping_mul(0xbc);
                c.r[3] = c.r[4] | c.r[4];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[7].wrapping_add(c.r[0]);
                c.r[7] = c.r[8].rotate_left(1) & 0x1;
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x54));
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[8] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.lr = 0x80162af4; f_8015d7d0(c);
                pc = 0x80162af4;
            }
            0x80162af4 => {
                { let ea = c.r[2].wrapping_add(0xffff9e3c); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0xc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.r[0] = c.lbz(c.r[31].wrapping_add(0xe5));
                { let v = s(c.f[3] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[2] = fctiwz(c.f[2]);
                c.cmpu(0, c.r[0], 0xff);
                c.f[1] = fctiwz(c.f[1]);
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[2]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[1]; c.stfd(ea, v); }
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x34));
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[6] = c.lwz(c.r[1].wrapping_add(0x44));
                if c.crbit(2) { pc = 0x80162b70; continue; }
                pc = 0x80162b40;
            }
            0x80162b40 => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x4c));
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x80162b70; continue; }
                pc = 0x80162b50;
            }
            0x80162b50 => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x4e));
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x80162b70; continue; }
                pc = 0x80162b60;
            }
            0x80162b60 => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x50));
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(2) { pc = 0x80162b90; continue; }
                pc = 0x80162b70;
            }
            0x80162b70 => {
                c.sth(c.r[31].wrapping_add(0x4c), c.r[4]);
                c.r[0] = 0x0;
                c.sth(c.r[31].wrapping_add(0x4e), c.r[5]);
                c.sth(c.r[31].wrapping_add(0x50), c.r[6]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[3] = c.r[3] | 0x1;
                c.stw(c.r[31].wrapping_add(0x24), c.r[3]);
                c.stb(c.r[31].wrapping_add(0xe5), c.r[0]);
                pc = 0x80162b90;
            }
            0x80162b90 => {
                { let ea = c.r[2].wrapping_add(0xffff9e3c); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x14); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0x18); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x1c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.r[0] = c.lbz(c.r[31].wrapping_add(0xe6));
                { let v = s(c.f[3] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[2] = fctiwz(c.f[2]);
                c.cmpu(0, c.r[0], 0xff);
                c.f[1] = fctiwz(c.f[1]);
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[2]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[1]; c.stfd(ea, v); }
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x44));
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[6] = c.lwz(c.r[1].wrapping_add(0x34));
                if c.crbit(2) { pc = 0x80162c0c; continue; }
                pc = 0x80162bdc;
            }
            0x80162bdc => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x52));
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x80162c0c; continue; }
                pc = 0x80162bec;
            }
            0x80162bec => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x54));
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x80162c0c; continue; }
                pc = 0x80162bfc;
            }
            0x80162bfc => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x56));
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(2) { pc = 0x80162c2c; continue; }
                pc = 0x80162c0c;
            }
            0x80162c0c => {
                c.sth(c.r[31].wrapping_add(0x52), c.r[4]);
                c.r[0] = 0x0;
                c.sth(c.r[31].wrapping_add(0x54), c.r[5]);
                c.sth(c.r[31].wrapping_add(0x56), c.r[6]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[3] = c.r[3] | 0x2;
                c.stw(c.r[31].wrapping_add(0x24), c.r[3]);
                c.stb(c.r[31].wrapping_add(0xe6), c.r[0]);
                pc = 0x80162c2c;
            }
            0x80162c2c => {
                { let ea = c.r[2].wrapping_add(0xffff9e3c); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x20); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0x24); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.r[0] = c.lbz(c.r[31].wrapping_add(0xe7));
                { let v = s(c.f[3] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[2] = fctiwz(c.f[2]);
                c.cmpu(0, c.r[0], 0xff);
                c.f[1] = fctiwz(c.f[1]);
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[2]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[1]; c.stfd(ea, v); }
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x44));
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[6] = c.lwz(c.r[1].wrapping_add(0x34));
                if c.crbit(2) { pc = 0x80162ca8; continue; }
                pc = 0x80162c78;
            }
            0x80162c78 => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x58));
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x80162ca8; continue; }
                pc = 0x80162c88;
            }
            0x80162c88 => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x5a));
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x80162ca8; continue; }
                pc = 0x80162c98;
            }
            0x80162c98 => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x5c));
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(2) { pc = 0x80162cc8; continue; }
                pc = 0x80162ca8;
            }
            0x80162ca8 => {
                c.sth(c.r[31].wrapping_add(0x58), c.r[4]);
                c.r[0] = 0x0;
                c.sth(c.r[31].wrapping_add(0x5a), c.r[5]);
                c.sth(c.r[31].wrapping_add(0x5c), c.r[6]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[3] = c.r[3] | 0x4;
                c.stw(c.r[31].wrapping_add(0x24), c.r[3]);
                c.stb(c.r[31].wrapping_add(0xe7), c.r[0]);
                pc = 0x80162cc8;
            }
            0x80162cc8 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xf0));
                c.r[0] = c.r[0].rotate_left(0) & 0x80000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80162d00; continue; }
                pc = 0x80162cd4;
            }
            0x80162cd4 => {
                c.r[3] = 0x80270000;
                c.r[0] = c.r[30].rotate_left(17) & 0x1fe;
                c.r[4] = c.r[3].wrapping_add(0x3448);
                c.r[3] = c.lhz(c.r[4].wrapping_add(c.r[0]));
                c.r[0] = c.lhz(c.r[4].wrapping_add(c.r[0]));
                c.sth(c.r[31].wrapping_add(0xd0), c.r[3]);
                c.r[0] = c.addc(!c.r[0], 0x20, 1);
                c.sth(c.r[31].wrapping_add(0xd2), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x200;
                c.stw(c.r[31].wrapping_add(0x24), c.r[0]);
                pc = 0x80162d00;
            }
            0x80162d00 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x4c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x48));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { return; }
                return;
            }
            _ => panic!("f_80162a58: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162D18 (0x80162d18, 44 bytes)
#[inline(never)]
pub fn f_80162d18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162d18) { return; }
        if c.prof { crate::prof::enter(c, 0x80162d18); f_80162d18_b(c, 0x80162d18); crate::prof::exit(c, 0x80162d18); return; }
    }
    f_80162d18_b(c, 0x80162d18)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162d18_at(c: &mut Cpu, pc: u32) { f_80162d18_b(c, pc) }
#[inline(always)]
fn f_80162d18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162d18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162d18, pc); }
        match pc {
            0x80162d18 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.lr = 0x80162d34; f_8015d4ec(c);
                pc = 0x80162d34;
            }
            0x80162d34 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80162d18: bad pc {:#x}", pc),
        }
    }
}

/// hwSetAUXProcessingCallbacks (0x80162d44, 40 bytes)
#[inline(never)]
pub fn f_80162d44(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162d44) { return; }
        if c.prof { crate::prof::enter(c, 0x80162d44); f_80162d44_b(c, 0x80162d44); crate::prof::exit(c, 0x80162d44); return; }
    }
    f_80162d44_b(c, 0x80162d44)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162d44_at(c: &mut Cpu, pc: u32) { f_80162d44_b(c, pc) }
#[inline(always)]
fn f_80162d44_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162d44;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162d44, pc); }
        match pc {
            0x80162d44 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x80440000;
                c.r[8] = c.r[0].wrapping_mul(0xbc);
                c.r[0] = c.r[3].wrapping_add(0x7e60);
                c.r[3] = c.r[0].wrapping_add(c.r[8]);
                c.stw(c.r[3].wrapping_add(0xac), c.r[4]);
                c.stw(c.r[3].wrapping_add(0xb4), c.r[5]);
                c.stw(c.r[3].wrapping_add(0xb0), c.r[6]);
                c.stw(c.r[3].wrapping_add(0xb8), c.r[7]);
                if true { return; }
                return;
            }
            _ => panic!("f_80162d44: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162D6C (0x80162d6c, 32 bytes)
#[inline(never)]
pub fn f_80162d6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162d6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80162d6c); f_80162d6c_b(c, 0x80162d6c); crate::prof::exit(c, 0x80162d6c); return; }
    }
    f_80162d6c_b(c, 0x80162d6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162d6c_at(c: &mut Cpu, pc: u32) { f_80162d6c_b(c, pc) }
#[inline(always)]
fn f_80162d6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162d6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162d6c, pc); }
        match pc {
            0x80162d6c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80162d7c; f_8015a950(c);
                pc = 0x80162d7c;
            }
            0x80162d7c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80162d6c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162D8C (0x80162d8c, 32 bytes)
#[inline(never)]
pub fn f_80162d8c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162d8c) { return; }
        if c.prof { crate::prof::enter(c, 0x80162d8c); f_80162d8c_b(c, 0x80162d8c); crate::prof::exit(c, 0x80162d8c); return; }
    }
    f_80162d8c_b(c, 0x80162d8c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162d8c_at(c: &mut Cpu, pc: u32) { f_80162d8c_b(c, pc) }
#[inline(always)]
fn f_80162d8c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162d8c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162d8c, pc); }
        match pc {
            0x80162d8c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80162d9c; f_8015aaa0(c);
                pc = 0x80162d9c;
            }
            0x80162d9c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80162d8c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162DAC (0x80162dac, 52 bytes)
#[inline(never)]
pub fn f_80162dac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162dac) { return; }
        if c.prof { crate::prof::enter(c, 0x80162dac); f_80162dac_b(c, 0x80162dac); crate::prof::exit(c, 0x80162dac); return; }
    }
    f_80162dac_b(c, 0x80162dac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162dac_at(c: &mut Cpu, pc: u32) { f_80162dac_b(c, pc) }
#[inline(always)]
fn f_80162dac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162dac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162dac, pc); }
        match pc {
            0x80162dac => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[5] = c.r[0].wrapping_mul(0xbc);
                c.r[3] = 0x80440000;
                c.r[0] = c.r[3].wrapping_add(0x7e60);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.lr = 0x80162dd0; f_8015d54c(c);
                pc = 0x80162dd0;
            }
            0x80162dd0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80162dac: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162DE0 (0x80162de0, 52 bytes)
#[inline(never)]
pub fn f_80162de0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162de0) { return; }
        if c.prof { crate::prof::enter(c, 0x80162de0); f_80162de0_b(c, 0x80162de0); crate::prof::exit(c, 0x80162de0); return; }
    }
    f_80162de0_b(c, 0x80162de0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162de0_at(c: &mut Cpu, pc: u32) { f_80162de0_b(c, pc) }
#[inline(always)]
fn f_80162de0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162de0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162de0, pc); }
        match pc {
            0x80162de0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[5] = c.r[0].wrapping_mul(0xbc);
                c.r[3] = 0x80440000;
                c.r[0] = c.r[3].wrapping_add(0x7e60);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.lr = 0x80162e04; f_8015d5f4(c);
                pc = 0x80162e04;
            }
            0x80162e04 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80162de0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162E14 (0x80162e14, 164 bytes)
#[inline(never)]
pub fn f_80162e14(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162e14) { return; }
        if c.prof { crate::prof::enter(c, 0x80162e14); f_80162e14_b(c, 0x80162e14); crate::prof::exit(c, 0x80162e14); return; }
    }
    f_80162e14_b(c, 0x80162e14)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162e14_at(c: &mut Cpu, pc: u32) { f_80162e14_b(c, pc) }
#[inline(always)]
fn f_80162e14_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162e14;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162e14, pc); }
        match pc {
            0x80162e14 => {
                c.r[5] = c.r[3].wrapping_mul(0xf4);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[6].wrapping_add(c.r[5]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(0xec));
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x80162e34; continue; }
                pc = 0x80162e2c;
            }
            0x80162e2c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80162e34;
            }
            0x80162e34 => {
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x90));
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x80162e94; continue; }
                pc = 0x80162e40;
            }
            0x80162e40 => {
                if !c.crbit(0) { pc = 0x80162e58; continue; }
                pc = 0x80162e44;
            }
            0x80162e44 => {
                c.cmps(0, c.r[0], 0x2);
                if !c.crbit(0) { pc = 0x80162ea4; continue; }
                pc = 0x80162e4c;
            }
            0x80162e4c => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x80162e60; continue; }
                pc = 0x80162e54;
            }
            0x80162e54 => {
                if true { return; }
                pc = 0x80162e58;
            }
            0x80162e58 => {
                c.cmps(0, c.r[0], 0x6);
                if !c.crbit(0) { return; }
                pc = 0x80162e60;
            }
            0x80162e60 => {
                c.r[3] = c.r[6].wrapping_add(c.r[5]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x78));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x20));
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[0] = c.r[3].wrapping_sub(c.r[0]);
                c.r[4] = c.r[3].rotate_left(0) & 0xf;
                c.r[0] = c.r[0].rotate_left(28) & 0xfffffff;
                c.cmpu(0, c.r[4], 0x2);
                c.r[3] = c.r[0].wrapping_mul(0xe);
                if c.crbit(0) { return; }
                pc = 0x80162e88;
            }
            0x80162e88 => {
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[3] = c.r[3].wrapping_add(0xfffffffe);
                if true { return; }
                pc = 0x80162e94;
            }
            0x80162e94 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x78));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x20));
                c.r[3] = c.r[0].wrapping_sub(c.r[3]);
                if true { return; }
                pc = 0x80162ea4;
            }
            0x80162ea4 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x78));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x20));
                c.r[3] = c.r[3].rotate_left(31) & 0x7fffffff;
                c.r[3] = c.r[0].wrapping_sub(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_80162e14: bad pc {:#x}", pc),
        }
    }
}

/// hwFlushStream (0x80162eb8, 144 bytes)
#[inline(never)]
pub fn f_80162eb8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162eb8) { return; }
        if c.prof { crate::prof::enter(c, 0x80162eb8); f_80162eb8_b(c, 0x80162eb8); crate::prof::exit(c, 0x80162eb8); return; }
    }
    f_80162eb8_b(c, 0x80162eb8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162eb8_at(c: &mut Cpu, pc: u32) { f_80162eb8_b(c, pc) }
#[inline(always)]
fn f_80162eb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162eb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162eb8, pc); }
        match pc {
            0x80162eb8 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x80162ecc; f_800c47d0(c);
                pc = 0x80162ecc;
            }
            0x80162ecc => {
                c.r[25] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[26] = c.r[5] | c.r[5];
                c.r[27] = c.r[7] | c.r[7];
                c.r[28] = c.r[8] | c.r[8];
                c.r[3] = c.r[6] | c.r[6];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80162eec; f_80163db0(c);
                pc = 0x80162eec;
            }
            0x80162eec => {
                c.r[0] = c.r[29].rotate_left(0) & 0x1f;
                c.r[30] = c.r[29].rotate_left(0) & 0xffffffe0;
                c.r[26] = c.r[26].wrapping_add(c.r[0]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.r[26].wrapping_add(0x1f);
                c.r[29] = c.r[25].wrapping_add(c.r[30]);
                c.r[26] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x80162f14; f_8009b300(c);
                pc = 0x80162f14;
            }
            0x80162f14 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[5] = c.r[26] | c.r[26];
                c.r[7] = c.r[27] | c.r[27];
                c.r[8] = c.r[28] | c.r[28];
                c.r[4] = c.r[31].wrapping_add(c.r[30]);
                c.r[6] = 0x1;
                c.lr = 0x80162f30; f_801632b4(c);
                pc = 0x80162f30;
            }
            0x80162f30 => {
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x80162f38; f_800c481c(c);
                pc = 0x80162f38;
            }
            0x80162f38 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80162eb8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162F48 (0x80162f48, 32 bytes)
#[inline(never)]
pub fn f_80162f48(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162f48) { return; }
        if c.prof { crate::prof::enter(c, 0x80162f48); f_80162f48_b(c, 0x80162f48); crate::prof::exit(c, 0x80162f48); return; }
    }
    f_80162f48_b(c, 0x80162f48)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162f48_at(c: &mut Cpu, pc: u32) { f_80162f48_b(c, pc) }
#[inline(always)]
fn f_80162f48_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162f48;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162f48, pc); }
        match pc {
            0x80162f48 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80162f58; f_80163ca8(c);
                pc = 0x80162f58;
            }
            0x80162f58 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80162f48: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162F68 (0x80162f68, 32 bytes)
#[inline(never)]
pub fn f_80162f68(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162f68) { return; }
        if c.prof { crate::prof::enter(c, 0x80162f68); f_80162f68_b(c, 0x80162f68); crate::prof::exit(c, 0x80162f68); return; }
    }
    f_80162f68_b(c, 0x80162f68)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162f68_at(c: &mut Cpu, pc: u32) { f_80162f68_b(c, pc) }
#[inline(always)]
fn f_80162f68_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162f68;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162f68, pc); }
        match pc {
            0x80162f68 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80162f78; f_80163de8(c);
                pc = 0x80162f78;
            }
            0x80162f78 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80162f68: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162F88 (0x80162f88, 36 bytes)
#[inline(never)]
pub fn f_80162f88(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162f88) { return; }
        if c.prof { crate::prof::enter(c, 0x80162f88); f_80162f88_b(c, 0x80162f88); crate::prof::exit(c, 0x80162f88); return; }
    }
    f_80162f88_b(c, 0x80162f88)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162f88_at(c: &mut Cpu, pc: u32) { f_80162f88_b(c, pc) }
#[inline(always)]
fn f_80162f88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162f88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162f88, pc); }
        match pc {
            0x80162f88 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80162f9c; f_80163db0(c);
                pc = 0x80162f9c;
            }
            0x80162f9c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80162f88: bad pc {:#x}", pc),
        }
    }
}

/// fn_80162FAC (0x80162fac, 4 bytes)
#[inline(never)]
pub fn f_80162fac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162fac) { return; }
        if c.prof { crate::prof::enter(c, 0x80162fac); f_80162fac_b(c, 0x80162fac); crate::prof::exit(c, 0x80162fac); return; }
    }
    f_80162fac_b(c, 0x80162fac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162fac_at(c: &mut Cpu, pc: u32) { f_80162fac_b(c, pc) }
#[inline(always)]
fn f_80162fac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162fac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162fac, pc); }
        match pc {
            0x80162fac => {
                if true { return; }
                return;
            }
            _ => panic!("f_80162fac: bad pc {:#x}", pc),
        }
    }
}

/// hwFrq2Pitch (0x80162fb0, 92 bytes)
#[inline(never)]
pub fn f_80162fb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80162fb0) { return; }
        if c.prof { crate::prof::enter(c, 0x80162fb0); f_80162fb0_b(c, 0x80162fb0); crate::prof::exit(c, 0x80162fb0); return; }
    }
    f_80162fb0_b(c, 0x80162fb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162fb0_at(c: &mut Cpu, pc: u32) { f_80162fb0_b(c, pc) }
#[inline(always)]
fn f_80162fb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80162fb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80162fb0, pc); }
        match pc {
            0x80162fb0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x43300000;
                c.r[4] = 0x80430000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9e48); c.f[2] = c.lfd(ea); }
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4c50));
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff9e40); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.stw(c.r[1].wrapping_add(0x8), c.r[5]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let v = s(c.f[0] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                c.stw(c.r[1].wrapping_add(0x10), c.r[5]);
                { let ea = c.r[1].wrapping_add(0x10); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] - c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x80162ffc; f_800c46b0(c);
                pc = 0x80162ffc;
            }
            0x80162ffc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80162fb0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8016300C (0x8016300c, 36 bytes)
#[inline(never)]
pub fn f_8016300c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016300c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016300c); f_8016300c_b(c, 0x8016300c); crate::prof::exit(c, 0x8016300c); return; }
    }
    f_8016300c_b(c, 0x8016300c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016300c_at(c: &mut Cpu, pc: u32) { f_8016300c_b(c, pc) }
#[inline(always)]
fn f_8016300c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016300c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016300c, pc); }
        match pc {
            0x8016300c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80163020; f_801634a8(c);
                pc = 0x80163020;
            }
            0x80163020 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8016300c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80163030 (0x80163030, 32 bytes)
#[inline(never)]
pub fn f_80163030(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163030) { return; }
        if c.prof { crate::prof::enter(c, 0x80163030); f_80163030_b(c, 0x80163030); crate::prof::exit(c, 0x80163030); return; }
    }
    f_80163030_b(c, 0x80163030)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163030_at(c: &mut Cpu, pc: u32) { f_80163030_b(c, pc) }
#[inline(always)]
fn f_80163030_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163030;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163030, pc); }
        match pc {
            0x80163030 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80163040; f_80163794(c);
                pc = 0x80163040;
            }
            0x80163040 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80163030: bad pc {:#x}", pc),
        }
    }
}

/// fn_80163050 (0x80163050, 148 bytes)
#[inline(never)]
pub fn f_80163050(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163050) { return; }
        if c.prof { crate::prof::enter(c, 0x80163050); f_80163050_b(c, 0x80163050); crate::prof::exit(c, 0x80163050); return; }
    }
    f_80163050_b(c, 0x80163050)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163050_at(c: &mut Cpu, pc: u32) { f_80163050_b(c, pc) }
#[inline(always)]
fn f_80163050_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163050;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163050, pc); }
        match pc {
            0x80163050 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = c.lwz(c.r[3]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[3].rotate_left(8) & 0xff;
                c.r[4] = c.r[3].rotate_left(0) & 0xffffff;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x801630c4; continue; }
                pc = 0x8016307c;
            }
            0x8016307c => {
                if !c.crbit(0) { pc = 0x80163094; continue; }
                pc = 0x80163080;
            }
            0x80163080 => {
                c.cmps(0, c.r[0], 0x2);
                if !c.crbit(0) { pc = 0x801630c0; continue; }
                pc = 0x80163088;
            }
            0x80163088 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x8016309c; continue; }
                pc = 0x80163090;
            }
            0x80163090 => {
                pc = 0x801630c4; continue;
                pc = 0x80163094;
            }
            0x80163094 => {
                c.cmps(0, c.r[0], 0x6);
                if !c.crbit(0) { pc = 0x801630c4; continue; }
                pc = 0x8016309c;
            }
            0x8016309c => {
                c.r[3] = 0x24920000;
                c.r[0] = c.r[4].wrapping_add(0xd);
                c.r[3] = c.r[3].wrapping_add(0x4925);
                c.r[3] = mulhwu(c.r[3], c.r[0]);
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.r[0] = c.r[0].rotate_left(31) & 0x7fffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.r[4] = c.r[0].rotate_left(0) & 0xfffffff8;
                pc = 0x801630c4; continue;
                pc = 0x801630c0;
            }
            0x801630c0 => {
                c.r[4] = c.r[4].rotate_left(1) & 0xfffffffe;
                pc = 0x801630c4;
            }
            0x801630c4 => {
                c.r[3] = c.lwz(c.r[31]);
                c.lr = 0x801630cc; f_80163810(c);
                pc = 0x801630cc;
            }
            0x801630cc => {
                c.stw(c.r[31], c.r[3]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80163050: bad pc {:#x}", pc),
        }
    }
}

/// fn_801630E4 (0x801630e4, 32 bytes)
#[inline(never)]
pub fn f_801630e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801630e4) { return; }
        if c.prof { crate::prof::enter(c, 0x801630e4); f_801630e4_b(c, 0x801630e4); crate::prof::exit(c, 0x801630e4); return; }
    }
    f_801630e4_b(c, 0x801630e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801630e4_at(c: &mut Cpu, pc: u32) { f_801630e4_b(c, pc) }
#[inline(always)]
fn f_801630e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801630e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801630e4, pc); }
        match pc {
            0x801630e4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801630f4; f_801637b8(c);
                pc = 0x801630f4;
            }
            0x801630f4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801630e4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80163104 (0x80163104, 132 bytes)
#[inline(never)]
pub fn f_80163104(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163104) { return; }
        if c.prof { crate::prof::enter(c, 0x80163104); f_80163104_b(c, 0x80163104); crate::prof::exit(c, 0x80163104); return; }
    }
    f_80163104_b(c, 0x80163104)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163104_at(c: &mut Cpu, pc: u32) { f_80163104_b(c, pc) }
#[inline(always)]
fn f_80163104_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163104;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163104, pc); }
        match pc {
            0x80163104 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[3].rotate_left(8) & 0xff;
                c.r[5] = c.r[3].rotate_left(0) & 0xffffff;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x8016316c; continue; }
                pc = 0x80163124;
            }
            0x80163124 => {
                if !c.crbit(0) { pc = 0x8016313c; continue; }
                pc = 0x80163128;
            }
            0x80163128 => {
                c.cmps(0, c.r[0], 0x2);
                if !c.crbit(0) { pc = 0x80163168; continue; }
                pc = 0x80163130;
            }
            0x80163130 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x80163144; continue; }
                pc = 0x80163138;
            }
            0x80163138 => {
                pc = 0x8016316c; continue;
                pc = 0x8016313c;
            }
            0x8016313c => {
                c.cmps(0, c.r[0], 0x6);
                if !c.crbit(0) { pc = 0x8016316c; continue; }
                pc = 0x80163144;
            }
            0x80163144 => {
                c.r[3] = 0x24920000;
                c.r[0] = c.r[5].wrapping_add(0xd);
                c.r[3] = c.r[3].wrapping_add(0x4925);
                c.r[3] = mulhwu(c.r[3], c.r[0]);
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.r[0] = c.r[0].rotate_left(31) & 0x7fffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.r[5] = c.r[0].rotate_left(0) & 0xfffffff8;
                pc = 0x8016316c; continue;
                pc = 0x80163168;
            }
            0x80163168 => {
                c.r[5] = c.r[5].rotate_left(1) & 0xfffffffe;
                pc = 0x8016316c;
            }
            0x8016316c => {
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.lr = 0x80163178; f_80163bcc(c);
                pc = 0x80163178;
            }
            0x80163178 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80163104: bad pc {:#x}", pc),
        }
    }
}

/// fn_80163188 (0x80163188, 32 bytes)
#[inline(never)]
pub fn f_80163188(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163188) { return; }
        if c.prof { crate::prof::enter(c, 0x80163188); f_80163188_b(c, 0x80163188); crate::prof::exit(c, 0x80163188); return; }
    }
    f_80163188_b(c, 0x80163188)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163188_at(c: &mut Cpu, pc: u32) { f_80163188_b(c, pc) }
#[inline(always)]
fn f_80163188_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163188;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163188, pc); }
        match pc {
            0x80163188 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80163198; f_80163490(c);
                pc = 0x80163198;
            }
            0x80163198 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80163188: bad pc {:#x}", pc),
        }
    }
}

/// fn_801631A8 (0x801631a8, 4 bytes)
#[inline(never)]
pub fn f_801631a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801631a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801631a8); f_801631a8_b(c, 0x801631a8); crate::prof::exit(c, 0x801631a8); return; }
    }
    f_801631a8_b(c, 0x801631a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801631a8_at(c: &mut Cpu, pc: u32) { f_801631a8_b(c, pc) }
#[inline(always)]
fn f_801631a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801631a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801631a8, pc); }
        match pc {
            0x801631a8 => {
                if true { return; }
                return;
            }
            _ => panic!("f_801631a8: bad pc {:#x}", pc),
        }
    }
}

/// fn_801631AC (0x801631ac, 20 bytes)
#[inline(never)]
pub fn f_801631ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801631ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801631ac); f_801631ac_b(c, 0x801631ac); crate::prof::exit(c, 0x801631ac); return; }
    }
    f_801631ac_b(c, 0x801631ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801631ac_at(c: &mut Cpu, pc: u32) { f_801631ac_b(c, pc) }
#[inline(always)]
fn f_801631ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801631ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801631ac, pc); }
        match pc {
            0x801631ac => {
                c.r[4] = c.lwz(c.r[3]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.stw(c.r[13].wrapping_add(0xffffa834), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa838), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_801631ac: bad pc {:#x}", pc),
        }
    }
}

/// fn_801631C0 (0x801631c0, 12 bytes)
#[inline(never)]
pub fn f_801631c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801631c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801631c0); f_801631c0_b(c, 0x801631c0); crate::prof::exit(c, 0x801631c0); return; }
    }
    f_801631c0_b(c, 0x801631c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801631c0_at(c: &mut Cpu, pc: u32) { f_801631c0_b(c, pc) }
#[inline(always)]
fn f_801631c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801631c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801631c0, pc); }
        match pc {
            0x801631c0 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa7f4), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_801631c0: bad pc {:#x}", pc),
        }
    }
}

/// hwGetVirtualSampleID (0x801631cc, 40 bytes)
#[inline(never)]
pub fn f_801631cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801631cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801631cc); f_801631cc_b(c, 0x801631cc); crate::prof::exit(c, 0x801631cc); return; }
    }
    f_801631cc_b(c, 0x801631cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801631cc_at(c: &mut Cpu, pc: u32) { f_801631cc_b(c, pc) }
#[inline(always)]
fn f_801631cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801631cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801631cc, pc); }
        match pc {
            0x801631cc => {
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0xec));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801631ec; continue; }
                pc = 0x801631e4;
            }
            0x801631e4 => {
                c.r[3] = 0xffffffff;
                if true { return; }
                pc = 0x801631ec;
            }
            0x801631ec => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xe8));
                if true { return; }
                return;
            }
            _ => panic!("f_801631cc: bad pc {:#x}", pc),
        }
    }
}

/// fn_801631F4 (0x801631f4, 32 bytes)
#[inline(never)]
pub fn f_801631f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801631f4) { return; }
        if c.prof { crate::prof::enter(c, 0x801631f4); f_801631f4_b(c, 0x801631f4); crate::prof::exit(c, 0x801631f4); return; }
    }
    f_801631f4_b(c, 0x801631f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801631f4_at(c: &mut Cpu, pc: u32) { f_801631f4_b(c, pc) }
#[inline(always)]
fn f_801631f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801631f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801631f4, pc); }
        match pc {
            0x801631f4 => {
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0xec));
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                if true { return; }
                return;
            }
            _ => panic!("f_801631f4: bad pc {:#x}", pc),
        }
    }
}

/// aramQueueCallback (0x80163214, 160 bytes)
#[inline(never)]
pub fn f_80163214(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163214) { return; }
        if c.prof { crate::prof::enter(c, 0x80163214); f_80163214_b(c, 0x80163214); crate::prof::exit(c, 0x80163214); return; }
    }
    f_80163214_b(c, 0x80163214)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163214_at(c: &mut Cpu, pc: u32) { f_80163214_b(c, pc) }
#[inline(always)]
fn f_80163214_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163214;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163214, pc); }
        match pc {
            0x80163214 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = 0x80450000;
                c.r[0] = c.lwz(c.r[28].wrapping_add(0xc));
                c.r[31] = c.r[3].wrapping_add(0xfffffb90);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80163250; continue; }
                pc = 0x80163248;
            }
            0x80163248 => {
                c.r[3] = 0x80450000;
                c.r[31] = c.r[3].wrapping_add(0xfffffe14);
                pc = 0x80163250;
            }
            0x80163250 => {
                c.r[29] = 0x0;
                c.r[30] = c.r[31] | c.r[31];
                pc = 0x80163258;
            }
            0x80163258 => {
                c.cmpu(0, c.r[28], c.r[30]);
                if !c.crbit(2) { pc = 0x80163278; continue; }
                pc = 0x80163260;
            }
            0x80163260 => {
                c.r[12] = c.lwz(c.r[30].wrapping_add(0x20));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x80163278; continue; }
                pc = 0x8016326c;
            }
            0x8016326c => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x24));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80163278; (c.dispatch)(c, t); }
                pc = 0x80163278;
            }
            0x80163278 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[30] = c.r[30].wrapping_add(0x28);
                c.cmpu(0, c.r[29], 0x10);
                if c.crbit(0) { pc = 0x80163258; continue; }
                pc = 0x80163288;
            }
            0x80163288 => {
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x281));
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.stb(c.r[31].wrapping_add(0x281), c.r[0]);
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
            _ => panic!("f_80163214: bad pc {:#x}", pc),
        }
    }
}

/// aramUploadData (0x801632b4, 476 bytes)
#[inline(never)]
pub fn f_801632b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801632b4) { return; }
        if c.prof { crate::prof::enter(c, 0x801632b4); f_801632b4_b(c, 0x801632b4); crate::prof::exit(c, 0x801632b4); return; }
    }
    f_801632b4_b(c, 0x801632b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801632b4_at(c: &mut Cpu, pc: u32) { f_801632b4_b(c, pc) }
#[inline(always)]
fn f_801632b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801632b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801632b4, pc); }
        match pc {
            0x801632b4 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x801632c8; f_800c47cc(c);
                pc = 0x801632c8;
            }
            0x801632c8 => {
                c.r[24] = c.r[6] | c.r[6];
                c.rc(c.r[24]);
                c.r[25] = c.r[3] | c.r[3];
                c.r[26] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[28] = c.r[7] | c.r[7];
                c.r[29] = c.r[8] | c.r[8];
                if c.crbit(2) { pc = 0x801632f0; continue; }
                pc = 0x801632e4;
            }
            0x801632e4 => {
                c.r[3] = 0x80450000;
                c.r[31] = c.r[3].wrapping_add(0xfffffe14);
                pc = 0x801632f8; continue;
                pc = 0x801632f0;
            }
            0x801632f0 => {
                c.r[3] = 0x80450000;
                c.r[31] = c.r[3].wrapping_add(0xfffffb90);
                pc = 0x801632f8;
            }
            0x801632f8 => {
                c.lr = 0x801632fc; f_8009df3c(c);
                pc = 0x801632fc;
            }
            0x801632fc => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x281));
                c.r[30] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[0], 0x10);
                if !c.crbit(0) { pc = 0x80163470; continue; }
                pc = 0x8016330c;
            }
            0x8016330c => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[24].wrapping_neg();
                c.r[0] = c.r[0] | c.r[24];
                c.r[3] = 0x80160000;
                c.r[4] = c.r[4].wrapping_mul(0x28);
                c.r[7] = 0x2a;
                c.r[5] = 0x0;
                c.r[8] = c.r[0].rotate_left(1) & 0x1;
                c.r[6] = c.r[31].wrapping_add(c.r[4]);
                c.r[4] = c.r[3].wrapping_add(0x3214);
                c.stw(c.r[6].wrapping_add(0x4), c.r[7]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[5]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[8]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[25]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x14), c.r[26]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x18), c.r[27]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1c), c.r[4]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x20), c.r[28]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x24), c.r[29]);
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[5] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[6] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[7] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[10] = c.r[0].wrapping_mul(0x28);
                c.r[8] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[9] = c.r[4].wrapping_mul(0x28);
                c.r[4] = c.r[31].wrapping_add(c.r[10]);
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[10] = c.r[5].wrapping_mul(0x28);
                c.r[5] = c.r[31].wrapping_add(c.r[9]);
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x8));
                c.r[9] = c.r[6].wrapping_mul(0x28);
                c.r[6] = c.r[31].wrapping_add(c.r[10]);
                c.r[6] = c.lwz(c.r[6].wrapping_add(0xc));
                c.r[10] = c.r[7].wrapping_mul(0x28);
                c.r[7] = c.r[31].wrapping_add(c.r[9]);
                c.r[7] = c.lwz(c.r[7].wrapping_add(0x10));
                c.r[9] = c.r[8].wrapping_mul(0x28);
                c.r[8] = c.r[31].wrapping_add(c.r[10]);
                c.r[8] = c.lwz(c.r[8].wrapping_add(0x14));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[9] = c.r[31].wrapping_add(c.r[9]);
                c.r[9] = c.lwz(c.r[9].wrapping_add(0x18));
                c.r[10] = c.r[31].wrapping_add(c.r[0]);
                c.r[10] = c.lwz(c.r[10].wrapping_add(0x1c));
                c.lr = 0x80163438; f_800ae630(c);
                pc = 0x80163438;
            }
            0x80163438 => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x281));
                c.r[3] = c.r[30] | c.r[30];
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stb(c.r[31].wrapping_add(0x281), c.r[0]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[4].rotate_left(28) & 0xf0000000;
                c.r[4] = c.r[4].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.r[0] = c.r[0].rotate_left(4) & 0xffffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.stb(c.r[31].wrapping_add(0x280), c.r[0]);
                c.lr = 0x8016346c; f_8009df64(c);
                pc = 0x8016346c;
            }
            0x8016346c => {
                pc = 0x80163478; continue;
                pc = 0x80163470;
            }
            0x80163470 => {
                c.lr = 0x80163474; f_8009df64(c);
                pc = 0x80163474;
            }
            0x80163474 => {
                pc = 0x801632f8; continue;
                pc = 0x80163478;
            }
            0x80163478 => {
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x80163480; f_800c4818(c);
                pc = 0x80163480;
            }
            0x80163480 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_801632b4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80163490 (0x80163490, 24 bytes)
#[inline(never)]
pub fn f_80163490(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163490) { return; }
        if c.prof { crate::prof::enter(c, 0x80163490); f_80163490_b(c, 0x80163490); crate::prof::exit(c, 0x80163490); return; }
    }
    f_80163490_b(c, 0x80163490)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163490_at(c: &mut Cpu, pc: u32) { f_80163490_b(c, pc) }
#[inline(always)]
fn f_80163490_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163490;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163490, pc); }
        match pc {
            0x80163490 => {
                c.r[3] = 0x80450000;
                c.r[3] = c.r[3].wrapping_add(0xfffffb90);
                pc = 0x80163498;
            }
            0x80163498 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x281));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80163498; continue; }
                pc = 0x801634a4;
            }
            0x801634a4 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80163490: bad pc {:#x}", pc),
        }
    }
}

/// fn_801634A8 (0x801634a8, 748 bytes)
#[inline(never)]
pub fn f_801634a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801634a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801634a8); f_801634a8_b(c, 0x801634a8); crate::prof::exit(c, 0x801634a8); return; }
    }
    f_801634a8_b(c, 0x801634a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801634a8_at(c: &mut Cpu, pc: u32) { f_801634a8_b(c, pc) }
#[inline(always)]
fn f_801634a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801634a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801634a8, pc); }
        match pc {
            0x801634a8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x20);
                c.lr = 0x801634bc; f_800c47d8(c);
                pc = 0x801634bc;
            }
            0x801634bc => {
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x801634c4; f_800acb44(c);
                pc = 0x801634c4;
            }
            0x801634c4 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x500;
                c.lr = 0x801634d0; f_801643d8(c);
                pc = 0x801634d0;
            }
            0x801634d0 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[0] = 0x10;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x801634e4;
            }
            0x801634e4 => {
                c.sth(c.r[4], c.r[3]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x4), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x6), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x8), c.r[3]);
                c.sth(c.r[4].wrapping_add(0xa), c.r[3]);
                c.sth(c.r[4].wrapping_add(0xc), c.r[3]);
                c.sth(c.r[4].wrapping_add(0xe), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x10), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x12), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x14), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x16), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x18), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x1a), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x1c), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x1e), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x20), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x22), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x24), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x26), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x28), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x2a), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x2c), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x2e), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x30), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x32), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x34), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x36), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x38), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x3a), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x3c), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x3e), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x40), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x42), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x44), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x46), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x48), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x4a), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x4c), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x4e), c.r[3]);
                c.r[4] = c.r[4].wrapping_add(0x50);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x801634e4; continue; }
                pc = 0x8016358c;
            }
            0x8016358c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x500;
                c.lr = 0x80163598; f_8009b2d0(c);
                pc = 0x80163598;
            }
            0x80163598 => {
                c.r[4] = 0x80450000;
                c.r[3] = 0x80450000;
                c.r[31] = c.r[4].wrapping_add(0xfffffb90);
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0x281), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0xfffffe14);
                c.stb(c.r[31].wrapping_add(0x280), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x281), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x280), c.r[0]);
                pc = 0x801635bc;
            }
            0x801635bc => {
                c.lr = 0x801635c0; f_8009df3c(c);
                pc = 0x801635c0;
            }
            0x801635c0 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x281));
                c.r[29] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[0], 0x10);
                if !c.crbit(0) { pc = 0x8016372c; continue; }
                pc = 0x801635d0;
            }
            0x801635d0 => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = 0x80160000;
                c.r[7] = 0x2a;
                c.r[0] = 0x0;
                c.r[6] = c.r[4].wrapping_mul(0x28);
                c.r[5] = 0x500;
                c.r[4] = c.r[3].wrapping_add(0x3214);
                c.r[3] = c.r[31].wrapping_add(c.r[6]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[7]);
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[28]);
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x14), c.r[30]);
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x18), c.r[5]);
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x1c), c.r[4]);
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x20), c.r[0]);
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[5] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[6] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[7] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[10] = c.r[0].wrapping_mul(0x28);
                c.r[8] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[9] = c.r[4].wrapping_mul(0x28);
                c.r[4] = c.r[31].wrapping_add(c.r[10]);
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[10] = c.r[5].wrapping_mul(0x28);
                c.r[5] = c.r[31].wrapping_add(c.r[9]);
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x8));
                c.r[9] = c.r[6].wrapping_mul(0x28);
                c.r[6] = c.r[31].wrapping_add(c.r[10]);
                c.r[6] = c.lwz(c.r[6].wrapping_add(0xc));
                c.r[10] = c.r[7].wrapping_mul(0x28);
                c.r[7] = c.r[31].wrapping_add(c.r[9]);
                c.r[7] = c.lwz(c.r[7].wrapping_add(0x10));
                c.r[9] = c.r[8].wrapping_mul(0x28);
                c.r[8] = c.r[31].wrapping_add(c.r[10]);
                c.r[8] = c.lwz(c.r[8].wrapping_add(0x14));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[9] = c.r[31].wrapping_add(c.r[9]);
                c.r[9] = c.lwz(c.r[9].wrapping_add(0x18));
                c.r[10] = c.r[31].wrapping_add(c.r[0]);
                c.r[10] = c.lwz(c.r[10].wrapping_add(0x1c));
                c.lr = 0x801636f4; f_800ae630(c);
                pc = 0x801636f4;
            }
            0x801636f4 => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x281));
                c.r[3] = c.r[29] | c.r[29];
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stb(c.r[31].wrapping_add(0x281), c.r[0]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x280));
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[4].rotate_left(28) & 0xf0000000;
                c.r[4] = c.r[4].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.r[0] = c.r[0].rotate_left(4) & 0xffffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.stb(c.r[31].wrapping_add(0x280), c.r[0]);
                c.lr = 0x80163728; f_8009df64(c);
                pc = 0x80163728;
            }
            0x80163728 => {
                pc = 0x80163734; continue;
                pc = 0x8016372c;
            }
            0x8016372c => {
                c.lr = 0x80163730; f_8009df64(c);
                pc = 0x80163730;
            }
            0x80163730 => {
                pc = 0x801635bc; continue;
                pc = 0x80163734;
            }
            0x80163734 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x281));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80163734; continue; }
                pc = 0x80163740;
            }
            0x80163740 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80163748; f_80164400(c);
                pc = 0x80163748;
            }
            0x80163748 => {
                c.r[0] = c.r[30].wrapping_add(c.r[27]);
                c.stw(c.r[13].wrapping_add(0xffffa85c), c.r[0]);
                c.lr = 0x80163754; f_800acb4c(c);
                pc = 0x80163754;
            }
            0x80163754 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa85c));
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(1) { pc = 0x80163768; continue; }
                pc = 0x80163760;
            }
            0x80163760 => {
                c.lr = 0x80163764; f_800acb4c(c);
                pc = 0x80163764;
            }
            0x80163764 => {
                c.stw(c.r[13].wrapping_add(0xffffa85c), c.r[3]);
                pc = 0x80163768;
            }
            0x80163768 => {
                c.r[3] = c.r[30].wrapping_add(0x500);
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa858), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa850), c.r[0]);
                c.lr = 0x8016377c; f_80163be4(c);
                pc = 0x8016377c;
            }
            0x8016377c => {
                c.r[11] = c.r[1].wrapping_add(0x20);
                c.lr = 0x80163784; f_800c4824(c);
                pc = 0x80163784;
            }
            0x80163784 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801634a8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80163794 (0x80163794, 4 bytes)
#[inline(never)]
pub fn f_80163794(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163794) { return; }
        if c.prof { crate::prof::enter(c, 0x80163794); f_80163794_b(c, 0x80163794); crate::prof::exit(c, 0x80163794); return; }
    }
    f_80163794_b(c, 0x80163794)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163794_at(c: &mut Cpu, pc: u32) { f_80163794_b(c, pc) }
#[inline(always)]
fn f_80163794_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163794;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163794, pc); }
        match pc {
            0x80163794 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80163794: bad pc {:#x}", pc),
        }
    }
}

/// fn_80163798 (0x80163798, 32 bytes)
#[inline(never)]
pub fn f_80163798(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163798) { return; }
        if c.prof { crate::prof::enter(c, 0x80163798); f_80163798_b(c, 0x80163798); crate::prof::exit(c, 0x80163798); return; }
    }
    f_80163798_b(c, 0x80163798)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163798_at(c: &mut Cpu, pc: u32) { f_80163798_b(c, pc) }
#[inline(always)]
fn f_80163798_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163798;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163798, pc); }
        match pc {
            0x80163798 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801637a8; f_800acb44(c);
                pc = 0x801637a8;
            }
            0x801637a8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80163798: bad pc {:#x}", pc),
        }
    }
}

/// aramSetUploadCallback (0x801637b8, 88 bytes)
#[inline(never)]
pub fn f_801637b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801637b8) { return; }
        if c.prof { crate::prof::enter(c, 0x801637b8); f_801637b8_b(c, 0x801637b8); crate::prof::exit(c, 0x801637b8); return; }
    }
    f_801637b8_b(c, 0x801637b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801637b8_at(c: &mut Cpu, pc: u32) { f_801637b8_b(c, pc) }
#[inline(always)]
fn f_801637b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801637b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801637b8, pc); }
        match pc {
            0x801637b8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x801637f4; continue; }
                pc = 0x801637d4;
            }
            0x801637d4 => {
                c.r[0] = c.r[4].wrapping_add(0x1f);
                c.r[31] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.lr = 0x801637e0; f_800ae78c(c);
                pc = 0x801637e0;
            }
            0x801637e0 => {
                c.cmpu(0, c.r[31], c.r[3]);
                c.r[0] = c.r[31] | c.r[31];
                if !c.crbit(0) { pc = 0x801637f0; continue; }
                pc = 0x801637ec;
            }
            0x801637ec => {
                c.r[0] = c.r[3] | c.r[3];
                pc = 0x801637f0;
            }
            0x801637f0 => {
                c.stw(c.r[13].wrapping_add(0xffffa84c), c.r[0]);
                pc = 0x801637f4;
            }
            0x801637f4 => {
                c.stw(c.r[13].wrapping_add(0xffffa850), c.r[30]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801637b8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80163810 (0x80163810, 956 bytes)
#[inline(never)]
pub fn f_80163810(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163810) { return; }
        if c.prof { crate::prof::enter(c, 0x80163810); f_80163810_b(c, 0x80163810); crate::prof::exit(c, 0x80163810); return; }
    }
    f_80163810_b(c, 0x80163810)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163810_at(c: &mut Cpu, pc: u32) { f_80163810_b(c, pc) }
#[inline(always)]
fn f_80163810_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163810;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163810, pc); }
        match pc {
            0x80163810 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x80163824; f_800c47c8(c);
                pc = 0x80163824;
            }
            0x80163824 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa850));
                c.r[4] = c.r[4].wrapping_add(0x1f);
                c.r[24] = c.lwz(c.r[13].wrapping_add(0xffffa858));
                c.r[25] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[0], 0x0);
                c.r[23] = c.r[4].rotate_left(0) & 0xffffffe0;
                if !c.crbit(2) { pc = 0x801639dc; continue; }
                pc = 0x80163840;
            }
            0x80163840 => {
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x80163848; f_8009b2d0(c);
                pc = 0x80163848;
            }
            0x80163848 => {
                c.r[3] = 0x80450000;
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffffa858));
                c.r[28] = c.r[3].wrapping_add(0xfffffb90);
                pc = 0x80163854;
            }
            0x80163854 => {
                c.lr = 0x80163858; f_8009df3c(c);
                pc = 0x80163858;
            }
            0x80163858 => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x281));
                c.r[29] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[0], 0x10);
                if !c.crbit(0) { pc = 0x801639c0; continue; }
                pc = 0x80163868;
            }
            0x80163868 => {
                c.r[4] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[3] = 0x80160000;
                c.r[6] = 0x2a;
                c.r[0] = 0x0;
                c.r[5] = c.r[4].wrapping_mul(0x28);
                c.r[4] = c.r[3].wrapping_add(0x3214);
                c.r[3] = c.r[28].wrapping_add(c.r[5]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[6]);
                c.r[3] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[25]);
                c.r[3] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x14), c.r[30]);
                c.r[3] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x18), c.r[23]);
                c.r[3] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x1c), c.r[4]);
                c.r[3] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x20), c.r[0]);
                c.r[3] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                c.r[3] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[4] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[5] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[6] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[7] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[10] = c.r[0].wrapping_mul(0x28);
                c.r[8] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[9] = c.r[4].wrapping_mul(0x28);
                c.r[4] = c.r[28].wrapping_add(c.r[10]);
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[10] = c.r[5].wrapping_mul(0x28);
                c.r[5] = c.r[28].wrapping_add(c.r[9]);
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x8));
                c.r[9] = c.r[6].wrapping_mul(0x28);
                c.r[6] = c.r[28].wrapping_add(c.r[10]);
                c.r[6] = c.lwz(c.r[6].wrapping_add(0xc));
                c.r[10] = c.r[7].wrapping_mul(0x28);
                c.r[7] = c.r[28].wrapping_add(c.r[9]);
                c.r[7] = c.lwz(c.r[7].wrapping_add(0x10));
                c.r[9] = c.r[8].wrapping_mul(0x28);
                c.r[8] = c.r[28].wrapping_add(c.r[10]);
                c.r[8] = c.lwz(c.r[8].wrapping_add(0x14));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[9] = c.r[28].wrapping_add(c.r[9]);
                c.r[9] = c.lwz(c.r[9].wrapping_add(0x18));
                c.r[10] = c.r[28].wrapping_add(c.r[0]);
                c.r[10] = c.lwz(c.r[10].wrapping_add(0x1c));
                c.lr = 0x80163988; f_800ae630(c);
                pc = 0x80163988;
            }
            0x80163988 => {
                c.r[4] = c.lbz(c.r[28].wrapping_add(0x281));
                c.r[3] = c.r[29] | c.r[29];
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stb(c.r[28].wrapping_add(0x281), c.r[0]);
                c.r[4] = c.lbz(c.r[28].wrapping_add(0x280));
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[4].rotate_left(28) & 0xf0000000;
                c.r[4] = c.r[4].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.r[0] = c.r[0].rotate_left(4) & 0xffffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.stb(c.r[28].wrapping_add(0x280), c.r[0]);
                c.lr = 0x801639bc; f_8009df64(c);
                pc = 0x801639bc;
            }
            0x801639bc => {
                pc = 0x801639c8; continue;
                pc = 0x801639c0;
            }
            0x801639c0 => {
                c.lr = 0x801639c4; f_8009df64(c);
                pc = 0x801639c4;
            }
            0x801639c4 => {
                pc = 0x80163854; continue;
                pc = 0x801639c8;
            }
            0x801639c8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa858));
                c.r[3] = c.r[24] | c.r[24];
                c.r[0] = c.r[0].wrapping_add(c.r[23]);
                c.stw(c.r[13].wrapping_add(0xffffa858), c.r[0]);
                pc = 0x80163bb4; continue;
                pc = 0x801639dc;
            }
            0x801639dc => {
                c.r[4] = 0x80450000;
                c.r[3] = 0x80160000;
                c.r[30] = c.r[4].wrapping_add(0xfffffb90);
                c.r[31] = c.r[3].wrapping_add(0x3214);
                pc = 0x80163ba8; continue;
                pc = 0x801639f0;
            }
            0x801639f0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa84c));
                c.r[28] = c.r[23] | c.r[23];
                c.cmpu(0, c.r[23], c.r[0]);
                if c.crbit(0) { pc = 0x80163a04; continue; }
                pc = 0x80163a00;
            }
            0x80163a00 => {
                c.r[28] = c.r[0] | c.r[0];
                pc = 0x80163a04;
            }
            0x80163a04 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa850));
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80163a18; (c.dispatch)(c, t); }
                pc = 0x80163a18;
            }
            0x80163a18 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80163a24; f_8009b2d0(c);
                pc = 0x80163a24;
            }
            0x80163a24 => {
                c.r[26] = c.lwz(c.r[13].wrapping_add(0xffffa858));
                pc = 0x80163a28;
            }
            0x80163a28 => {
                c.lr = 0x80163a2c; f_8009df3c(c);
                pc = 0x80163a2c;
            }
            0x80163a2c => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x281));
                c.r[27] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[0], 0x10);
                if !c.crbit(0) { pc = 0x80163b8c; continue; }
                pc = 0x80163a3c;
            }
            0x80163a3c => {
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[4] = 0x2a;
                c.r[0] = 0x0;
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[4]);
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x8);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[0]);
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0xc);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[0]);
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[29]);
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x14);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[26]);
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x18);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[28]);
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x1c);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[31]);
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[0]);
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x24);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[0]);
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[4] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[5] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[6] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[7] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[10] = c.r[0].wrapping_mul(0x28);
                c.r[8] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[30].wrapping_add(c.r[3]);
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[9] = c.r[4].wrapping_mul(0x28);
                c.r[4] = c.r[10].wrapping_add(0x4);
                c.r[4] = c.lwz(c.r[30].wrapping_add(c.r[4]));
                c.r[10] = c.r[5].wrapping_mul(0x28);
                c.r[5] = c.r[9].wrapping_add(0x8);
                c.r[5] = c.lwz(c.r[30].wrapping_add(c.r[5]));
                c.r[9] = c.r[6].wrapping_mul(0x28);
                c.r[6] = c.r[10].wrapping_add(0xc);
                c.r[6] = c.lwz(c.r[30].wrapping_add(c.r[6]));
                c.r[10] = c.r[7].wrapping_mul(0x28);
                c.r[7] = c.r[9].wrapping_add(0x10);
                c.r[7] = c.lwz(c.r[30].wrapping_add(c.r[7]));
                c.r[9] = c.r[8].wrapping_mul(0x28);
                c.r[8] = c.r[10].wrapping_add(0x14);
                c.r[8] = c.lwz(c.r[30].wrapping_add(c.r[8]));
                c.r[10] = c.r[0].wrapping_mul(0x28);
                c.r[0] = c.r[9].wrapping_add(0x18);
                c.r[9] = c.lwz(c.r[30].wrapping_add(c.r[0]));
                c.r[0] = c.r[10].wrapping_add(0x1c);
                c.r[10] = c.lwz(c.r[30].wrapping_add(c.r[0]));
                c.lr = 0x80163b54; f_800ae630(c);
                pc = 0x80163b54;
            }
            0x80163b54 => {
                c.r[4] = c.lbz(c.r[30].wrapping_add(0x281));
                c.r[3] = c.r[27] | c.r[27];
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stb(c.r[30].wrapping_add(0x281), c.r[0]);
                c.r[4] = c.lbz(c.r[30].wrapping_add(0x280));
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[4].rotate_left(28) & 0xf0000000;
                c.r[4] = c.r[4].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.r[0] = c.r[0].rotate_left(4) & 0xffffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.stb(c.r[30].wrapping_add(0x280), c.r[0]);
                c.lr = 0x80163b88; f_8009df64(c);
                pc = 0x80163b88;
            }
            0x80163b88 => {
                pc = 0x80163b94; continue;
                pc = 0x80163b8c;
            }
            0x80163b8c => {
                c.lr = 0x80163b90; f_8009df64(c);
                pc = 0x80163b90;
            }
            0x80163b90 => {
                pc = 0x80163a28; continue;
                pc = 0x80163b94;
            }
            0x80163b94 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa858));
                c.r[23] = c.r[23].wrapping_sub(c.r[28]);
                c.r[25] = c.r[25].wrapping_add(c.r[28]);
                c.r[0] = c.r[0].wrapping_add(c.r[28]);
                c.stw(c.r[13].wrapping_add(0xffffa858), c.r[0]);
                pc = 0x80163ba8;
            }
            0x80163ba8 => {
                c.cmpu(0, c.r[23], 0x0);
                if !c.crbit(2) { pc = 0x801639f0; continue; }
                pc = 0x80163bb0;
            }
            0x80163bb0 => {
                c.r[3] = c.r[24] | c.r[24];
                pc = 0x80163bb4;
            }
            0x80163bb4 => {
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x80163bbc; f_800c4814(c);
                pc = 0x80163bbc;
            }
            0x80163bbc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80163810: bad pc {:#x}", pc),
        }
    }
}

/// fn_80163BCC (0x80163bcc, 24 bytes)
#[inline(never)]
pub fn f_80163bcc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163bcc) { return; }
        if c.prof { crate::prof::enter(c, 0x80163bcc); f_80163bcc_b(c, 0x80163bcc); crate::prof::exit(c, 0x80163bcc); return; }
    }
    f_80163bcc_b(c, 0x80163bcc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163bcc_at(c: &mut Cpu, pc: u32) { f_80163bcc_b(c, pc) }
#[inline(always)]
fn f_80163bcc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163bcc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163bcc, pc); }
        match pc {
            0x80163bcc => {
                c.r[3] = c.r[4].wrapping_add(0x1f);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa858));
                c.r[3] = c.r[3].rotate_left(0) & 0xffffffe0;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa858), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80163bcc: bad pc {:#x}", pc),
        }
    }
}

/// InitStreamBuffers (0x80163be4, 196 bytes)
#[inline(never)]
pub fn f_80163be4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163be4) { return; }
        if c.prof { crate::prof::enter(c, 0x80163be4); f_80163be4_b(c, 0x80163be4); crate::prof::exit(c, 0x80163be4); return; }
    }
    f_80163be4_b(c, 0x80163be4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163be4_at(c: &mut Cpu, pc: u32) { f_80163be4_b(c, pc) }
#[inline(always)]
fn f_80163be4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163be4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163be4, pc); }
        match pc {
            0x80163be4 => {
                c.r[3] = 0x80450000;
                c.r[4] = 0x0;
                c.r[3] = c.r[3].wrapping_add(0x98);
                c.r[0] = 0x7;
                c.stw(c.r[13].wrapping_add(0xffffa848), c.r[4]);
                c.r[7] = c.r[3].wrapping_add(0x10);
                c.r[8] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa844), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa840), c.r[3]);
                c.mtspr(9, c.r[0]);
                pc = 0x80163c0c;
            }
            0x80163c0c => {
                c.stw(c.r[7].wrapping_add(0xfffffff0), c.r[7]);
                c.r[3] = c.r[7].wrapping_add(0x10);
                c.r[0] = c.r[7].wrapping_add(0x20);
                c.r[6] = c.r[7].wrapping_add(0x30);
                c.stw(c.r[7], c.r[3]);
                c.r[5] = c.r[7].wrapping_add(0x40);
                c.r[4] = c.r[7].wrapping_add(0x50);
                c.r[3] = c.r[7].wrapping_add(0x60);
                c.stw(c.r[7].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.r[7].wrapping_add(0x70);
                c.r[8] = c.r[8].wrapping_add(0x8);
                c.stw(c.r[7].wrapping_add(0x20), c.r[6]);
                c.stw(c.r[7].wrapping_add(0x30), c.r[5]);
                c.stw(c.r[7].wrapping_add(0x40), c.r[4]);
                c.stw(c.r[7].wrapping_add(0x50), c.r[3]);
                c.stw(c.r[7].wrapping_add(0x60), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0x80);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80163c0c; continue; }
                pc = 0x80163c54;
            }
            0x80163c54 => {
                c.r[3] = 0x80450000;
                c.r[4] = c.r[8].rotate_left(4) & 0xfffffff0;
                c.r[3] = c.r[3].wrapping_add(0x98);
                c.r[0] = c.addc(!c.r[8], 0x40, 1);
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.mtspr(9, c.r[0]);
                c.cmpu(0, c.r[8], 0x40);
                if !c.crbit(0) { pc = 0x80163c84; continue; }
                pc = 0x80163c74;
            }
            0x80163c74 => {
                c.stw(c.r[3].wrapping_add(0xfffffff0), c.r[3]);
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.r[8] = c.r[8].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80163c74; continue; }
                pc = 0x80163c84;
            }
            0x80163c84 => {
                c.r[3] = 0x80450000;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa85c));
                c.r[4] = c.r[3].wrapping_add(0x98);
                c.r[3] = c.r[8].rotate_left(4) & 0xfffffff0;
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[4] = 0x0;
                c.stw(c.r[3].wrapping_add(0xfffffff0), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa854), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80163be4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80163CA8 (0x80163ca8, 264 bytes)
#[inline(never)]
pub fn f_80163ca8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163ca8) { return; }
        if c.prof { crate::prof::enter(c, 0x80163ca8); f_80163ca8_b(c, 0x80163ca8); crate::prof::exit(c, 0x80163ca8); return; }
    }
    f_80163ca8_b(c, 0x80163ca8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163ca8_at(c: &mut Cpu, pc: u32) { f_80163ca8_b(c, pc) }
#[inline(always)]
fn f_80163ca8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163ca8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163ca8, pc); }
        match pc {
            0x80163ca8 => {
                c.r[0] = c.r[3].wrapping_add(0x1f);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa844));
                c.r[7] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[5] = 0x0;
                c.r[4] = 0x0;
                c.r[6] = 0xffffffff;
                pc = 0x80163cf4; continue;
                pc = 0x80163cc4;
            }
            0x80163cc4 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xc));
                c.cmpu(0, c.r[0], c.r[7]);
                if !c.crbit(2) { pc = 0x80163cd8; continue; }
                pc = 0x80163cd0;
            }
            0x80163cd0 => {
                c.r[5] = c.r[3] | c.r[3];
                pc = 0x80163cfc; continue;
                pc = 0x80163cd8;
            }
            0x80163cd8 => {
                if !c.crbit(1) { pc = 0x80163cec; continue; }
                pc = 0x80163cdc;
            }
            0x80163cdc => {
                c.cmpu(0, c.r[6], c.r[0]);
                if !c.crbit(1) { pc = 0x80163cec; continue; }
                pc = 0x80163ce4;
            }
            0x80163ce4 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[0] | c.r[0];
                pc = 0x80163cec;
            }
            0x80163cec => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.lwz(c.r[3]);
                pc = 0x80163cf4;
            }
            0x80163cf4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80163cc4; continue; }
                pc = 0x80163cfc;
            }
            0x80163cfc => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x80163d58; continue; }
                pc = 0x80163d04;
            }
            0x80163d04 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa840));
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x80163d84; continue; }
                pc = 0x80163d10;
            }
            0x80163d10 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa854));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa858));
                c.r[3] = c.r[3].wrapping_sub(c.r[7]);
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x80163d84; continue; }
                pc = 0x80163d24;
            }
            0x80163d24 => {
                c.r[0] = c.lwz(c.r[4]);
                c.r[5] = c.r[4] | c.r[4];
                c.stw(c.r[13].wrapping_add(0xffffa840), c.r[0]);
                c.stw(c.r[4].wrapping_add(0xc), c.r[7]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[7]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa854));
                c.r[0] = c.r[0].wrapping_sub(c.r[7]);
                c.stw(c.r[13].wrapping_add(0xffffa854), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa848));
                c.stw(c.r[4], c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa848), c.r[4]);
                pc = 0x80163d84; continue;
                pc = 0x80163d58;
            }
            0x80163d58 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x80163d6c; continue; }
                pc = 0x80163d60;
            }
            0x80163d60 => {
                c.r[0] = c.lwz(c.r[5]);
                c.stw(c.r[4], c.r[0]);
                pc = 0x80163d74; continue;
                pc = 0x80163d6c;
            }
            0x80163d6c => {
                c.r[0] = c.lwz(c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa844), c.r[0]);
                pc = 0x80163d74;
            }
            0x80163d74 => {
                c.stw(c.r[5].wrapping_add(0x8), c.r[7]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa848));
                c.stw(c.r[5], c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa848), c.r[5]);
                pc = 0x80163d84;
            }
            0x80163d84 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x80163d94; continue; }
                pc = 0x80163d8c;
            }
            0x80163d8c => {
                c.r[3] = 0xff;
                if true { return; }
                pc = 0x80163d94;
            }
            0x80163d94 => {
                c.r[3] = 0x80450000;
                c.r[0] = c.r[3].wrapping_add(0x98);
                c.r[0] = c.r[5].wrapping_sub(c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 28 != 0); c.r[0] = (v >> 4) as u32; }
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                if true { return; }
                return;
            }
            _ => panic!("f_80163ca8: bad pc {:#x}", pc),
        }
    }
}

/// aramGetStreamBufferAddress (0x80163db0, 56 bytes)
#[inline(never)]
pub fn f_80163db0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163db0) { return; }
        if c.prof { crate::prof::enter(c, 0x80163db0); f_80163db0_b(c, 0x80163db0); crate::prof::exit(c, 0x80163db0); return; }
    }
    f_80163db0_b(c, 0x80163db0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163db0_at(c: &mut Cpu, pc: u32) { f_80163db0_b(c, pc) }
#[inline(always)]
fn f_80163db0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163db0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163db0, pc); }
        match pc {
            0x80163db0 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x80163dd0; continue; }
                pc = 0x80163db8;
            }
            0x80163db8 => {
                c.r[5] = 0x80450000;
                c.r[0] = c.r[3].rotate_left(4) & 0xff0;
                c.r[5] = c.r[5].wrapping_add(0x98);
                c.r[5] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x8));
                c.stw(c.r[4], c.r[0]);
                pc = 0x80163dd0;
            }
            0x80163dd0 => {
                c.r[4] = 0x80450000;
                c.r[0] = c.r[3].rotate_left(4) & 0xff0;
                c.r[3] = c.r[4].wrapping_add(0x98);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_80163db0: bad pc {:#x}", pc),
        }
    }
}

/// aramFreeStreamBuffer (0x80163de8, 248 bytes)
#[inline(never)]
pub fn f_80163de8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163de8) { return; }
        if c.prof { crate::prof::enter(c, 0x80163de8); f_80163de8_b(c, 0x80163de8); crate::prof::exit(c, 0x80163de8); return; }
    }
    f_80163de8_b(c, 0x80163de8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163de8_at(c: &mut Cpu, pc: u32) { f_80163de8_b(c, pc) }
#[inline(always)]
fn f_80163de8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163de8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163de8, pc); }
        match pc {
            0x80163de8 => {
                c.r[4] = 0x80450000;
                c.r[3] = c.r[3].rotate_left(4) & 0xff0;
                c.r[0] = c.r[4].wrapping_add(0x98);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa848));
                c.r[6] = 0x0;
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                pc = 0x80163e34; continue;
                pc = 0x80163e04;
            }
            0x80163e04 => {
                c.cmpu(0, c.r[5], c.r[4]);
                if !c.crbit(2) { pc = 0x80163e2c; continue; }
                pc = 0x80163e0c;
            }
            0x80163e0c => {
                c.cmpu(0, c.r[6], 0x0);
                if c.crbit(2) { pc = 0x80163e20; continue; }
                pc = 0x80163e14;
            }
            0x80163e14 => {
                c.r[0] = c.lwz(c.r[4]);
                c.stw(c.r[6], c.r[0]);
                pc = 0x80163e3c; continue;
                pc = 0x80163e20;
            }
            0x80163e20 => {
                c.r[0] = c.lwz(c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa848), c.r[0]);
                pc = 0x80163e3c; continue;
                pc = 0x80163e2c;
            }
            0x80163e2c => {
                c.r[6] = c.r[5] | c.r[5];
                c.r[5] = c.lwz(c.r[5]);
                pc = 0x80163e34;
            }
            0x80163e34 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x80163e04; continue; }
                pc = 0x80163e3c;
            }
            0x80163e3c => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa854));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x80163ed0; continue; }
                pc = 0x80163e4c;
            }
            0x80163e4c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa840));
                c.r[5] = 0xffffffff;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa848));
                c.stw(c.r[4], c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa840), c.r[4]);
                pc = 0x80163e78; continue;
                pc = 0x80163e64;
            }
            0x80163e64 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.cmpu(0, c.r[0], c.r[5]);
                if c.crbit(1) { pc = 0x80163e74; continue; }
                pc = 0x80163e70;
            }
            0x80163e70 => {
                c.r[5] = c.r[0] | c.r[0];
                pc = 0x80163e74;
            }
            0x80163e74 => {
                c.r[3] = c.lwz(c.r[3]);
                pc = 0x80163e78;
            }
            0x80163e78 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80163e64; continue; }
                pc = 0x80163e80;
            }
            0x80163e80 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa844));
                pc = 0x80163eac; continue;
                pc = 0x80163e88;
            }
            0x80163e88 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[3] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], c.r[5]);
                if !c.crbit(0) { pc = 0x80163ea8; continue; }
                pc = 0x80163e98;
            }
            0x80163e98 => {
                c.stw(c.r[13].wrapping_add(0xffffa844), c.r[3]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa840));
                c.stw(c.r[4], c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa840), c.r[4]);
                pc = 0x80163ea8;
            }
            0x80163ea8 => {
                c.r[4] = c.r[3] | c.r[3];
                pc = 0x80163eac;
            }
            0x80163eac => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x80163e88; continue; }
                pc = 0x80163eb4;
            }
            0x80163eb4 => {
                c.r[0] = c.r[5].wrapping_add(0x10000);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa85c));
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x80163ec8; continue; }
                pc = 0x80163ec4;
            }
            0x80163ec4 => {
                c.r[3] = c.r[5] | c.r[5];
                pc = 0x80163ec8;
            }
            0x80163ec8 => {
                c.stw(c.r[13].wrapping_add(0xffffa854), c.r[3]);
                if true { return; }
                pc = 0x80163ed0;
            }
            0x80163ed0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa844));
                c.stw(c.r[4], c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa844), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80163de8: bad pc {:#x}", pc),
        }
    }
}

/// salCallback (0x80163ee0, 168 bytes)
#[inline(never)]
pub fn f_80163ee0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163ee0) { return; }
        if c.prof { crate::prof::enter(c, 0x80163ee0); f_80163ee0_b(c, 0x80163ee0); crate::prof::exit(c, 0x80163ee0); return; }
    }
    f_80163ee0_b(c, 0x80163ee0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163ee0_at(c: &mut Cpu, pc: u32) { f_80163ee0_b(c, pc) }
#[inline(always)]
fn f_80163ee0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163ee0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163ee0, pc); }
        match pc {
            0x80163ee0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x280;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = c.lbz(c.r[13].wrapping_add(0xffffa880));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa87c));
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.r[0] = c.r[5].rotate_left(30) & 0xc0000000;
                c.r[3] = c.r[3].wrapping_add(0x80000000);
                c.r[5] = c.r[5].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[5]);
                c.r[0] = c.r[0].rotate_left(2) & 0xffffffff;
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                c.stb(c.r[13].wrapping_add(0xffffa880), c.r[5]);
                c.r[0] = c.r[0].wrapping_mul(0x280);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.lr = 0x80163f28; f_800ac070(c);
                pc = 0x80163f28;
            }
            0x80163f28 => {
                c.lr = 0x80163f2c; f_800a2790(c);
                pc = 0x80163f2c;
            }
            0x80163f2c => {
                c.stw(c.r[13].wrapping_add(0xffffa86c), c.r[3]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa878));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80163f70; continue; }
                pc = 0x80163f3c;
            }
            0x80163f3c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa870));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80163f78; continue; }
                pc = 0x80163f48;
            }
            0x80163f48 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa870), c.r[0]);
                c.lr = 0x80163f54; f_8009df50(c);
                pc = 0x80163f54;
            }
            0x80163f54 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa884));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80163f60; (c.dispatch)(c, t); }
                pc = 0x80163f60;
            }
            0x80163f60 => {
                c.lr = 0x80163f64; f_8009df3c(c);
                pc = 0x80163f64;
            }
            0x80163f64 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa870), c.r[0]);
                pc = 0x80163f78; continue;
                pc = 0x80163f70;
            }
            0x80163f70 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa874), c.r[0]);
                pc = 0x80163f78;
            }
            0x80163f78 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80163ee0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80163F88 (0x80163f88, 16 bytes)
#[inline(never)]
pub fn f_80163f88(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163f88) { return; }
        if c.prof { crate::prof::enter(c, 0x80163f88); f_80163f88_b(c, 0x80163f88); crate::prof::exit(c, 0x80163f88); return; }
    }
    f_80163f88_b(c, 0x80163f88)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163f88_at(c: &mut Cpu, pc: u32) { f_80163f88_b(c, pc) }
#[inline(always)]
fn f_80163f88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163f88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163f88, pc); }
        match pc {
            0x80163f88 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa878), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa868), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80163f88: bad pc {:#x}", pc),
        }
    }
}

/// dspResumeCallback (0x80163f98, 100 bytes)
#[inline(never)]
pub fn f_80163f98(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163f98) { return; }
        if c.prof { crate::prof::enter(c, 0x80163f98); f_80163f98_b(c, 0x80163f98); crate::prof::exit(c, 0x80163f98); return; }
    }
    f_80163f98_b(c, 0x80163f98)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163f98_at(c: &mut Cpu, pc: u32) { f_80163f98_b(c, pc) }
#[inline(always)]
fn f_80163f98_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163f98;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163f98, pc); }
        match pc {
            0x80163f98 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa878), c.r[3]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa874));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80163fec; continue; }
                pc = 0x80163fb8;
            }
            0x80163fb8 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa874), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa870));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80163fec; continue; }
                pc = 0x80163fcc;
            }
            0x80163fcc => {
                c.stw(c.r[13].wrapping_add(0xffffa870), c.r[3]);
                c.lr = 0x80163fd4; f_8009df50(c);
                pc = 0x80163fd4;
            }
            0x80163fd4 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa884));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80163fe0; (c.dispatch)(c, t); }
                pc = 0x80163fe0;
            }
            0x80163fe0 => {
                c.lr = 0x80163fe4; f_8009df3c(c);
                pc = 0x80163fe4;
            }
            0x80163fe4 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa870), c.r[0]);
                pc = 0x80163fec;
            }
            0x80163fec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80163f98: bad pc {:#x}", pc),
        }
    }
}

/// salInitAi (0x80163ffc, 200 bytes)
#[inline(never)]
pub fn f_80163ffc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80163ffc) { return; }
        if c.prof { crate::prof::enter(c, 0x80163ffc); f_80163ffc_b(c, 0x80163ffc); crate::prof::exit(c, 0x80163ffc); return; }
    }
    f_80163ffc_b(c, 0x80163ffc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163ffc_at(c: &mut Cpu, pc: u32) { f_80163ffc_b(c, pc) }
#[inline(always)]
fn f_80163ffc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80163ffc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80163ffc, pc); }
        match pc {
            0x80163ffc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0xa00;
                c.lr = 0x80164020; f_801643d8(c);
                pc = 0x80164020;
            }
            0x80164020 => {
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa87c), c.r[3]);
                if c.crbit(2) { pc = 0x801640a8; continue; }
                pc = 0x8016402c;
            }
            0x8016402c => {
                c.r[4] = 0x0;
                c.r[5] = 0xa00;
                c.lr = 0x80164038; f_8000540c(c);
                pc = 0x80164038;
            }
            0x80164038 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa87c));
                c.r[4] = 0xa00;
                c.lr = 0x80164044; f_8009b2d0(c);
                pc = 0x80164044;
            }
            0x80164044 => {
                c.r[4] = 0x1;
                c.r[0] = 0x0;
                c.r[3] = 0x80160000;
                c.stw(c.r[13].wrapping_add(0xffffa874), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x3ee0);
                c.stw(c.r[13].wrapping_add(0xffffa878), c.r[4]);
                c.stb(c.r[13].wrapping_add(0xffffa880), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa870), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa884), c.r[30]);
                c.lr = 0x8016406c; f_800ac02c(c);
                pc = 0x8016406c;
            }
            0x8016406c => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa880));
                c.r[4] = 0x280;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa87c));
                c.r[0] = c.r[0].wrapping_mul(0x280);
                c.r[3] = c.r[3].wrapping_add(0x80000000);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.lr = 0x80164088; f_800ac070(c);
                pc = 0x80164088;
            }
            0x80164088 => {
                c.r[3] = 0x80430000;
                c.r[4] = 0x20;
                c.r[3] = c.r[3].wrapping_add(0x4c50);
                c.r[0] = 0x7d00;
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                c.r[3] = 0x1;
                c.stw(c.r[31], c.r[0]);
                pc = 0x801640ac; continue;
                pc = 0x801640a8;
            }
            0x801640a8 => {
                c.r[3] = 0x0;
                pc = 0x801640ac;
            }
            0x801640ac => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80163ffc: bad pc {:#x}", pc),
        }
    }
}

/// fn_801640C4 (0x801640c4, 32 bytes)
#[inline(never)]
pub fn f_801640c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801640c4) { return; }
        if c.prof { crate::prof::enter(c, 0x801640c4); f_801640c4_b(c, 0x801640c4); crate::prof::exit(c, 0x801640c4); return; }
    }
    f_801640c4_b(c, 0x801640c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801640c4_at(c: &mut Cpu, pc: u32) { f_801640c4_b(c, pc) }
#[inline(always)]
fn f_801640c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801640c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801640c4, pc); }
        match pc {
            0x801640c4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801640d4; f_800ac0f8(c);
                pc = 0x801640d4;
            }
            0x801640d4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801640c4: bad pc {:#x}", pc),
        }
    }
}

/// salExitAi (0x801640e4, 52 bytes)
#[inline(never)]
pub fn f_801640e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801640e4) { return; }
        if c.prof { crate::prof::enter(c, 0x801640e4); f_801640e4_b(c, 0x801640e4); crate::prof::exit(c, 0x801640e4); return; }
    }
    f_801640e4_b(c, 0x801640e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801640e4_at(c: &mut Cpu, pc: u32) { f_801640e4_b(c, pc) }
#[inline(always)]
fn f_801640e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801640e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801640e4, pc); }
        match pc {
            0x801640e4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801640f8; f_800ac02c(c);
                pc = 0x801640f8;
            }
            0x801640f8 => {
                c.lr = 0x801640fc; f_800ac110(c);
                pc = 0x801640fc;
            }
            0x801640fc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa87c));
                c.lr = 0x80164104; f_80164400(c);
                pc = 0x80164104;
            }
            0x80164104 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801640e4: bad pc {:#x}", pc),
        }
    }
}

/// salAiGetDest (0x80164118, 48 bytes)
#[inline(never)]
pub fn f_80164118(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164118) { return; }
        if c.prof { crate::prof::enter(c, 0x80164118); f_80164118_b(c, 0x80164118); crate::prof::exit(c, 0x80164118); return; }
    }
    f_80164118_b(c, 0x80164118)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164118_at(c: &mut Cpu, pc: u32) { f_80164118_b(c, pc) }
#[inline(always)]
fn f_80164118_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164118;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164118, pc); }
        match pc {
            0x80164118 => {
                c.r[3] = c.lbz(c.r[13].wrapping_add(0xffffa880));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa87c));
                c.r[3] = c.r[3].wrapping_add(0x2);
                c.r[0] = c.r[3].rotate_left(30) & 0xc0000000;
                c.r[3] = c.r[3].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.r[0] = c.r[0].rotate_left(2) & 0xffffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.r[0].wrapping_mul(0x280);
                c.r[3] = c.r[4].wrapping_add(c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80164118: bad pc {:#x}", pc),
        }
    }
}

/// fn_80164148 (0x80164148, 188 bytes)
#[inline(never)]
pub fn f_80164148(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164148) { return; }
        if c.prof { crate::prof::enter(c, 0x80164148); f_80164148_b(c, 0x80164148); crate::prof::exit(c, 0x80164148); return; }
    }
    f_80164148_b(c, 0x80164148)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164148_at(c: &mut Cpu, pc: u32) { f_80164148_b(c, pc) }
#[inline(always)]
fn f_80164148_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164148;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164148, pc); }
        match pc {
            0x80164148 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x80370000;
                c.r[6] = 0x80450000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = 0x80160000;
                c.r[3] = 0x80160000;
                c.r[10] = c.r[6].wrapping_add(0x4a0);
                c.r[9] = c.lhz(c.r[13].wrapping_add(0xffff83e8));
                c.r[5] = 0x80450000;
                c.r[8] = 0x0;
                c.r[11] = c.r[7].wrapping_add(0xffffa520);
                c.r[7] = c.r[5].wrapping_add(0x500);
                c.r[0] = c.r[3].wrapping_add(0x3f98);
                c.r[4] = c.r[4].wrapping_add(0x3f88);
                c.r[6] = 0x2000;
                c.r[5] = 0x10;
                c.r[3] = 0x30;
                c.stw(c.r[10].wrapping_add(0xc), c.r[11]);
                c.stw(c.r[10].wrapping_add(0x10), c.r[9]);
                c.stw(c.r[10].wrapping_add(0x14), c.r[8]);
                c.stw(c.r[10].wrapping_add(0x18), c.r[7]);
                c.stw(c.r[10].wrapping_add(0x1c), c.r[6]);
                c.stw(c.r[10].wrapping_add(0x20), c.r[8]);
                c.sth(c.r[10].wrapping_add(0x24), c.r[5]);
                c.sth(c.r[10].wrapping_add(0x26), c.r[3]);
                c.stw(c.r[10].wrapping_add(0x28), c.r[4]);
                c.stw(c.r[10].wrapping_add(0x2c), c.r[0]);
                c.stw(c.r[10].wrapping_add(0x30), c.r[8]);
                c.stw(c.r[10].wrapping_add(0x34), c.r[8]);
                c.stw(c.r[10].wrapping_add(0x4), c.r[8]);
                c.lr = 0x801641c8; f_800ae7e0(c);
                pc = 0x801641c8;
            }
            0x801641c8 => {
                c.r[3] = 0x80450000;
                c.r[3] = c.r[3].wrapping_add(0x4a0);
                c.lr = 0x801641d4; f_800ae93c(c);
                pc = 0x801641d4;
            }
            0x801641d4 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa868), c.r[0]);
                c.lr = 0x801641e0; f_80164328(c);
                pc = 0x801641e0;
            }
            0x801641e0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa868));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801641e0; continue; }
                pc = 0x801641ec;
            }
            0x801641ec => {
                c.lr = 0x801641f0; f_80164360(c);
                pc = 0x801641f0;
            }
            0x801641f0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80164148: bad pc {:#x}", pc),
        }
    }
}

/// fn_80164204 (0x80164204, 52 bytes)
#[inline(never)]
pub fn f_80164204(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164204) { return; }
        if c.prof { crate::prof::enter(c, 0x80164204); f_80164204_b(c, 0x80164204); crate::prof::exit(c, 0x80164204); return; }
    }
    f_80164204_b(c, 0x80164204)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164204_at(c: &mut Cpu, pc: u32) { f_80164204_b(c, pc) }
#[inline(always)]
fn f_80164204_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164204;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164204, pc); }
        match pc {
            0x80164204 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80164214; f_800ae8ec(c);
                pc = 0x80164214;
            }
            0x80164214 => {
                c.lr = 0x80164218; f_800ae92c(c);
                pc = 0x80164218;
            }
            0x80164218 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80164214; continue; }
                pc = 0x80164220;
            }
            0x80164220 => {
                c.lr = 0x80164224; f_800ae8a4(c);
                pc = 0x80164224;
            }
            0x80164224 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80164204: bad pc {:#x}", pc),
        }
    }
}

/// salCtrlDsp (0x80164238, 116 bytes)
#[inline(never)]
pub fn f_80164238(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164238) { return; }
        if c.prof { crate::prof::enter(c, 0x80164238); f_80164238_b(c, 0x80164238); crate::prof::exit(c, 0x80164238); return; }
    }
    f_80164238_b(c, 0x80164238)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164238_at(c: &mut Cpu, pc: u32) { f_80164238_b(c, pc) }
#[inline(always)]
fn f_80164238_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164238;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164238, pc); }
        match pc {
            0x80164238 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80164250; f_801642ac(c);
                pc = 0x80164250;
            }
            0x80164250 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8016425c; f_8015b250(c);
                pc = 0x8016425c;
            }
            0x8016425c => {
                c.r[0] = 0x0;
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa7f0));
                c.stw(c.r[13].wrapping_add(0xffffa878), c.r[0]);
                c.lr = 0x8016426c; f_80098034(c);
                pc = 0x8016426c;
            }
            0x8016426c => {
                c.r[0] = c.lhz(c.r[13].wrapping_add(0xffffa7ec));
                c.r[3] = c.r[0] | 0xbabe0000;
                c.lr = 0x80164278; f_800ae7cc(c);
                pc = 0x80164278;
            }
            0x80164278 => {
                c.lr = 0x8016427c; f_800ae794(c);
                pc = 0x8016427c;
            }
            0x8016427c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80164278; continue; }
                pc = 0x80164284;
            }
            0x80164284 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8016428c; f_800ae7cc(c);
                pc = 0x8016428c;
            }
            0x8016428c => {
                c.lr = 0x80164290; f_800ae794(c);
                pc = 0x80164290;
            }
            0x80164290 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8016428c; continue; }
                pc = 0x80164298;
            }
            0x80164298 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80164238: bad pc {:#x}", pc),
        }
    }
}

/// salGetStartDelay (0x801642ac, 76 bytes)
#[inline(never)]
pub fn f_801642ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801642ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801642ac); f_801642ac_b(c, 0x801642ac); crate::prof::exit(c, 0x801642ac); return; }
    }
    f_801642ac_b(c, 0x801642ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801642ac_at(c: &mut Cpu, pc: u32) { f_801642ac_b(c, pc) }
#[inline(always)]
fn f_801642ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801642ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801642ac, pc); }
        match pc {
            0x801642ac => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801642bc; f_800a2790(c);
                pc = 0x801642bc;
            }
            0x801642bc => {
                c.r[5] = 0x80000000;
                c.r[4] = 0x431c0000;
                c.r[0] = c.lwz(c.r[5].wrapping_add(0xf8));
                c.r[4] = c.r[4].wrapping_add(0xffffde83);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa86c));
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[0] = mulhwu(c.r[4], c.r[0]);
                c.r[3] = c.r[3].wrapping_sub(c.r[5]);
                c.r[3] = c.r[3].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[0].rotate_left(17) & 0x1ffff;
                c.r[3] = divwu(c.r[3], c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801642ac: bad pc {:#x}", pc),
        }
    }
}

/// hwInitIrq (0x801642f8, 44 bytes)
#[inline(never)]
pub fn f_801642f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801642f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801642f8); f_801642f8_b(c, 0x801642f8); crate::prof::exit(c, 0x801642f8); return; }
    }
    f_801642f8_b(c, 0x801642f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801642f8_at(c: &mut Cpu, pc: u32) { f_801642f8_b(c, pc) }
#[inline(always)]
fn f_801642f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801642f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801642f8, pc); }
        match pc {
            0x801642f8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80164308; f_8009df3c(c);
                pc = 0x80164308;
            }
            0x80164308 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa860), c.r[3]);
                c.sth(c.r[13].wrapping_add(0xffffa864), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801642f8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80164324 (0x80164324, 4 bytes)
#[inline(never)]
pub fn f_80164324(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164324) { return; }
        if c.prof { crate::prof::enter(c, 0x80164324); f_80164324_b(c, 0x80164324); crate::prof::exit(c, 0x80164324); return; }
    }
    f_80164324_b(c, 0x80164324)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164324_at(c: &mut Cpu, pc: u32) { f_80164324_b(c, pc) }
#[inline(always)]
fn f_80164324_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164324;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164324, pc); }
        match pc {
            0x80164324 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80164324: bad pc {:#x}", pc),
        }
    }
}

/// hwEnableIrq (0x80164328, 56 bytes)
#[inline(never)]
pub fn f_80164328(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164328) { return; }
        if c.prof { crate::prof::enter(c, 0x80164328); f_80164328_b(c, 0x80164328); crate::prof::exit(c, 0x80164328); return; }
    }
    f_80164328_b(c, 0x80164328)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164328_at(c: &mut Cpu, pc: u32) { f_80164328_b(c, pc) }
#[inline(always)]
fn f_80164328_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164328;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164328, pc); }
        match pc {
            0x80164328 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lhz(c.r[13].wrapping_add(0xffffa864));
                c.r[3] = c.r[3].wrapping_add(0xffffffff);
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.sth(c.r[13].wrapping_add(0xffffa864), c.r[3]);
                if !c.crbit(2) { pc = 0x80164350; continue; }
                pc = 0x80164348;
            }
            0x80164348 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa860));
                c.lr = 0x80164350; f_8009df64(c);
                pc = 0x80164350;
            }
            0x80164350 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80164328: bad pc {:#x}", pc),
        }
    }
}

/// hwDisableIrq (0x80164360, 56 bytes)
#[inline(never)]
pub fn f_80164360(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164360) { return; }
        if c.prof { crate::prof::enter(c, 0x80164360); f_80164360_b(c, 0x80164360); crate::prof::exit(c, 0x80164360); return; }
    }
    f_80164360_b(c, 0x80164360)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164360_at(c: &mut Cpu, pc: u32) { f_80164360_b(c, pc) }
#[inline(always)]
fn f_80164360_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164360;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164360, pc); }
        match pc {
            0x80164360 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lhz(c.r[13].wrapping_add(0xffffa864));
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[13].wrapping_add(0xffffa864), c.r[3]);
                if !c.crbit(2) { pc = 0x80164388; continue; }
                pc = 0x80164380;
            }
            0x80164380 => {
                c.lr = 0x80164384; f_8009df3c(c);
                pc = 0x80164384;
            }
            0x80164384 => {
                c.stw(c.r[13].wrapping_add(0xffffa860), c.r[3]);
                pc = 0x80164388;
            }
            0x80164388 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80164360: bad pc {:#x}", pc),
        }
    }
}

/// fn_80164398 (0x80164398, 32 bytes)
#[inline(never)]
pub fn f_80164398(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164398) { return; }
        if c.prof { crate::prof::enter(c, 0x80164398); f_80164398_b(c, 0x80164398); crate::prof::exit(c, 0x80164398); return; }
    }
    f_80164398_b(c, 0x80164398)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164398_at(c: &mut Cpu, pc: u32) { f_80164398_b(c, pc) }
#[inline(always)]
fn f_80164398_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164398;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164398, pc); }
        match pc {
            0x80164398 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801643a8; f_8009df3c(c);
                pc = 0x801643a8;
            }
            0x801643a8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80164398: bad pc {:#x}", pc),
        }
    }
}

/// fn_801643B8 (0x801643b8, 32 bytes)
#[inline(never)]
pub fn f_801643b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801643b8) { return; }
        if c.prof { crate::prof::enter(c, 0x801643b8); f_801643b8_b(c, 0x801643b8); crate::prof::exit(c, 0x801643b8); return; }
    }
    f_801643b8_b(c, 0x801643b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801643b8_at(c: &mut Cpu, pc: u32) { f_801643b8_b(c, pc) }
#[inline(always)]
fn f_801643b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801643b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801643b8, pc); }
        match pc {
            0x801643b8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801643c8; f_8009df50(c);
                pc = 0x801643c8;
            }
            0x801643c8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801643b8: bad pc {:#x}", pc),
        }
    }
}

/// fn_801643D8 (0x801643d8, 40 bytes)
#[inline(never)]
pub fn f_801643d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801643d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801643d8); f_801643d8_b(c, 0x801643d8); crate::prof::exit(c, 0x801643d8); return; }
    }
    f_801643d8_b(c, 0x801643d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801643d8_at(c: &mut Cpu, pc: u32) { f_801643d8_b(c, pc) }
#[inline(always)]
fn f_801643d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801643d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801643d8, pc); }
        match pc {
            0x801643d8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa834));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x801643f0; (c.dispatch)(c, t); }
                pc = 0x801643f0;
            }
            0x801643f0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801643d8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80164400 (0x80164400, 44 bytes)
#[inline(never)]
pub fn f_80164400(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164400) { return; }
        if c.prof { crate::prof::enter(c, 0x80164400); f_80164400_b(c, 0x80164400); crate::prof::exit(c, 0x80164400); return; }
    }
    f_80164400_b(c, 0x80164400)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164400_at(c: &mut Cpu, pc: u32) { f_80164400_b(c, pc) }
#[inline(always)]
fn f_80164400_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164400;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164400, pc); }
        match pc {
            0x80164400 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = c.r[13].wrapping_add(0xffffa834);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x4));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x8016441c; (c.dispatch)(c, t); }
                pc = 0x8016441c;
            }
            0x8016441c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80164400: bad pc {:#x}", pc),
        }
    }
}

/// sndAuxCallbackReverbHI (0x8016442c, 92 bytes)
#[inline(never)]
pub fn f_8016442c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016442c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016442c); f_8016442c_b(c, 0x8016442c); crate::prof::exit(c, 0x8016442c); return; }
    }
    f_8016442c_b(c, 0x8016442c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016442c_at(c: &mut Cpu, pc: u32) { f_8016442c_b(c, pc) }
#[inline(always)]
fn f_8016442c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016442c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016442c, pc); }
        match pc {
            0x8016442c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80164478; continue; }
                pc = 0x80164448;
            }
            0x80164448 => {
                if !c.crbit(0) { pc = 0x80164478; continue; }
                pc = 0x8016444c;
            }
            0x8016444c => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x80164458; continue; }
                pc = 0x80164454;
            }
            0x80164454 => {
                pc = 0x80164478; continue;
                pc = 0x80164458;
            }
            0x80164458 => {
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x1c4));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80164478; continue; }
                pc = 0x80164464;
            }
            0x80164464 => {
                c.r[3] = c.lwz(c.r[7]);
                c.r[6] = c.r[5] | c.r[5];
                c.r[4] = c.lwz(c.r[7].wrapping_add(0x4));
                c.r[5] = c.lwz(c.r[7].wrapping_add(0x8));
                c.lr = 0x80164478; f_801652dc(c);
                pc = 0x80164478;
            }
            0x80164478 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8016442c: bad pc {:#x}", pc),
        }
    }
}

/// sndAuxCallbackUpdateSettingsReverbHI (0x80164488, 88 bytes)
#[inline(never)]
pub fn f_80164488(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164488) { return; }
        if c.prof { crate::prof::enter(c, 0x80164488); f_80164488_b(c, 0x80164488); crate::prof::exit(c, 0x80164488); return; }
    }
    f_80164488_b(c, 0x80164488)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164488_at(c: &mut Cpu, pc: u32) { f_80164488_b(c, pc) }
#[inline(always)]
fn f_80164488_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164488;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164488, pc); }
        match pc {
            0x80164488 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = 0x1;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stb(c.r[3].wrapping_add(0x1c4), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x1c8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0x1d0); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[3].wrapping_add(0x1cc); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[3].wrapping_add(0x1d4); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[3].wrapping_add(0x1d8); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[3].wrapping_add(0x1dc); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                c.lr = 0x801644c0; f_80164a2c(c);
                pc = 0x801644c0;
            }
            0x801644c0 => {
                c.r[0] = 0x0;
                c.r[3] = 0x1;
                c.stb(c.r[31].wrapping_add(0x1c4), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80164488: bad pc {:#x}", pc),
        }
    }
}

/// sndAuxCallbackPrepareReverbHI (0x801644e0, 64 bytes)
#[inline(never)]
pub fn f_801644e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801644e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801644e0); f_801644e0_b(c, 0x801644e0); crate::prof::exit(c, 0x801644e0); return; }
    }
    f_801644e0_b(c, 0x801644e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801644e0_at(c: &mut Cpu, pc: u32) { f_801644e0_b(c, pc) }
#[inline(always)]
fn f_801644e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801644e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801644e0, pc); }
        match pc {
            0x801644e0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = 0x0;
                c.stb(c.r[3].wrapping_add(0x1c4), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x1c8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0x1d0); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[3].wrapping_add(0x1cc); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[3].wrapping_add(0x1d4); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[3].wrapping_add(0x1d8); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[3].wrapping_add(0x1dc); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                c.lr = 0x80164510; f_80164520(c);
                pc = 0x80164510;
            }
            0x80164510 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801644e0: bad pc {:#x}", pc),
        }
    }
}

/// ReverbHICreate (0x80164520, 1292 bytes)
#[inline(never)]
pub fn f_80164520(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164520) { return; }
        if c.prof { crate::prof::enter(c, 0x80164520); f_80164520_b(c, 0x80164520); crate::prof::exit(c, 0x80164520); return; }
    }
    f_80164520_b(c, 0x80164520)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164520_at(c: &mut Cpu, pc: u32) { f_80164520_b(c, pc) }
#[inline(always)]
fn f_80164520_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164520;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164520, pc); }
        match pc {
            0x80164520 => {
                { let ea = c.r[1].wrapping_add(0xffffff40); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xc4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xb0); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0xb8); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0xa0); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0xa8); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x90); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x98); let (x, y) = (c.f[29], c.ps1[29]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x80); let v = c.f[28]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = (c.f[28], c.ps1[28]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x70); let v = c.f[27]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = (c.f[27], c.ps1[27]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[26]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = (c.f[26], c.ps1[26]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[25]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = (c.f[25], c.ps1[25]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[24]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = (c.f[24], c.ps1[24]); c.psq_store(ea, x, y, false, 0); }
                c.r[11] = c.r[1].wrapping_add(0x40);
                c.lr = 0x80164574; f_800c47bc(c);
                pc = 0x80164574;
            }
            0x80164574 => {
                c.f[24] = c.f[1];
                { let ea = c.r[2].wrapping_add(0xffff9e50); c.f[7] = c.lfs(ea); c.ps1[7] = c.f[7]; }
                c.f[28] = c.f[2];
                c.r[30] = c.r[3] | c.r[3];
                c.f[25] = c.f[3];
                c.f[26] = c.f[4];
                c.f[31] = c.f[5];
                c.f[27] = c.f[6];
                c.fcmp(0, c.f[24], c.f[7]);
                if c.crbit(0) { pc = 0x80164604; continue; }
                pc = 0x8016459c;
            }
            0x8016459c => {
                { let ea = c.r[2].wrapping_add(0xffff9e54); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[24], c.f[1]);
                if c.crbit(1) { pc = 0x80164604; continue; }
                pc = 0x801645a8;
            }
            0x801645a8 => {
                { let ea = c.r[2].wrapping_add(0xffff9e58); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[28], c.f[0]);
                if c.crbit(0) { pc = 0x80164604; continue; }
                pc = 0x801645b4;
            }
            0x801645b4 => {
                { let ea = c.r[2].wrapping_add(0xffff9e5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[28], c.f[0]);
                if c.crbit(1) { pc = 0x80164604; continue; }
                pc = 0x801645c0;
            }
            0x801645c0 => {
                c.fcmp(0, c.f[25], c.f[7]);
                if c.crbit(0) { pc = 0x80164604; continue; }
                pc = 0x801645c8;
            }
            0x801645c8 => {
                c.fcmp(0, c.f[25], c.f[1]);
                if c.crbit(1) { pc = 0x80164604; continue; }
                pc = 0x801645d0;
            }
            0x801645d0 => {
                c.fcmp(0, c.f[27], c.f[7]);
                if c.crbit(0) { pc = 0x80164604; continue; }
                pc = 0x801645d8;
            }
            0x801645d8 => {
                c.fcmp(0, c.f[27], c.f[1]);
                if c.crbit(1) { pc = 0x80164604; continue; }
                pc = 0x801645e0;
            }
            0x801645e0 => {
                c.fcmp(0, c.f[26], c.f[7]);
                if c.crbit(0) { pc = 0x80164604; continue; }
                pc = 0x801645e8;
            }
            0x801645e8 => {
                c.fcmp(0, c.f[26], c.f[1]);
                if c.crbit(1) { pc = 0x80164604; continue; }
                pc = 0x801645f0;
            }
            0x801645f0 => {
                c.fcmp(0, c.f[31], c.f[7]);
                if c.crbit(0) { pc = 0x80164604; continue; }
                pc = 0x801645f8;
            }
            0x801645f8 => {
                { let ea = c.r[2].wrapping_add(0xffff9e60); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[31], c.f[0]);
                if !c.crbit(1) { pc = 0x8016460c; continue; }
                pc = 0x80164604;
            }
            0x80164604 => {
                c.r[3] = 0x0;
                pc = 0x801649d4; continue;
                pc = 0x8016460c;
            }
            0x8016460c => {
                c.r[4] = 0x0;
                c.r[5] = 0x1c4;
                c.lr = 0x80164618; f_8000540c(c);
                pc = 0x80164618;
            }
            0x80164618 => {
                { let ea = c.r[2].wrapping_add(0xffff9e64); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = 0x80370000;
                { let ea = c.r[2].wrapping_add(0xffff9e50); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                c.r[29] = c.r[3].wrapping_add(0xffffbf00);
                { let v = s(c.f[0] * c.f[28]); c.f[28] = v; c.ps1[28] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9e78); c.f[29] = c.lfd(ea); }
                c.r[31] = 0x0;
                c.r[28] = 0x43300000;
                pc = 0x801648ec; continue;
                pc = 0x8016463c;
            }
            0x8016463c => {
                c.r[23] = c.r[31].rotate_left(0) & 0xff;
                c.r[22] = 0x0;
                c.r[24] = c.r[23].wrapping_mul(0x3);
                pc = 0x80164740; continue;
                pc = 0x8016464c;
            }
            0x8016464c => {
                c.r[0] = c.r[22].rotate_left(0) & 0xff;
                c.r[27] = c.r[22].rotate_left(2) & 0x3fc;
                c.r[25] = c.r[0].wrapping_add(c.r[24]);
                c.r[3] = c.lwz(c.r[29].wrapping_add(c.r[27]));
                c.r[0] = c.r[25].wrapping_mul(0x14);
                c.r[21] = c.r[3].wrapping_add(0x2);
                c.r[20] = c.r[21].rotate_left(2) & 0xfffffffc;
                c.r[26] = c.r[30].wrapping_add(c.r[0]);
                c.stw(c.r[26].wrapping_add(0xbc), c.r[20]);
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x80164678; f_801643d8(c);
                pc = 0x80164678;
            }
            0x80164678 => {
                c.stw(c.r[26].wrapping_add(0xc0), c.r[3]);
                c.r[5] = c.r[20] | c.r[20];
                c.r[4] = 0x0;
                c.r[3] = c.lwz(c.r[26].wrapping_add(0xc0));
                c.lr = 0x8016468c; f_8000540c(c);
                pc = 0x8016468c;
            }
            0x8016468c => {
                { let ea = c.r[26].wrapping_add(0xc4); let v = c.f[30]; c.stfs(ea, v); }
                { let v = c.r[21] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.r[3] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.lwz(c.r[26].wrapping_add(0xb4));
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[26].wrapping_add(0xb8), c.r[0]);
                pc = 0x801646b8; continue;
                pc = 0x801646a8;
            }
            0x801646a8 => {
                c.r[3] = c.lwz(c.r[26].wrapping_add(0xb8));
                c.r[0] = c.lwz(c.r[26].wrapping_add(0xbc));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[26].wrapping_add(0xb8), c.r[0]);
                pc = 0x801646b8;
            }
            0x801646b8 => {
                c.r[0] = c.lwz(c.r[26].wrapping_add(0xb8));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(0) { pc = 0x801646a8; continue; }
                pc = 0x801646c4;
            }
            0x801646c4 => {
                c.r[0] = 0x0;
                c.stw(c.r[26].wrapping_add(0xb4), c.r[0]);
                c.stw(c.r[26].wrapping_add(0xb8), c.r[0]);
                c.r[3] = c.lwz(c.r[29].wrapping_add(c.r[27]));
                c.r[0] = c.lwz(c.r[26].wrapping_add(0xb4));
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[26].wrapping_add(0xb8), c.r[0]);
                pc = 0x801646f8; continue;
                pc = 0x801646e8;
            }
            0x801646e8 => {
                c.r[3] = c.lwz(c.r[26].wrapping_add(0xb8));
                c.r[0] = c.lwz(c.r[26].wrapping_add(0xbc));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[26].wrapping_add(0xb8), c.r[0]);
                pc = 0x801646f8;
            }
            0x801646f8 => {
                c.r[0] = c.lwz(c.r[26].wrapping_add(0xb8));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(0) { pc = 0x801646e8; continue; }
                pc = 0x80164704;
            }
            0x80164704 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(c.r[27]));
                c.stw(c.r[1].wrapping_add(0x8), c.r[28]);
                c.r[0] = c.r[0].wrapping_mul(0xfffffffd);
                { let ea = c.r[2].wrapping_add(0xffff9e68); c.f[1] = c.lfd(ea); }
                c.r[0] = c.r[0] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[29]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] / c.f[28]); c.f[2] = v; c.ps1[2] = v; }
                c.lr = 0x8016472c; f_800ce358(c);
                pc = 0x8016472c;
            }
            0x8016472c => {
                { let v = s(c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                c.r[3] = c.r[25].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[3].wrapping_add(0x16c);
                c.r[22] = c.r[22].wrapping_add(0x1);
                { let ea = c.r[30].wrapping_add(c.r[0]); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80164740;
            }
            0x80164740 => {
                c.r[0] = c.r[22].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x8016464c; continue; }
                pc = 0x8016474c;
            }
            0x8016474c => {
                c.r[27] = 0x0;
                pc = 0x80164818; continue;
                pc = 0x80164754;
            }
            0x80164754 => {
                c.r[0] = c.r[27].rotate_left(2) & 0x3fc;
                c.r[4] = c.r[27].rotate_left(0) & 0xff;
                c.r[3] = c.r[29].wrapping_add(c.r[0]);
                c.r[0] = c.r[4].wrapping_add(c.r[24]);
                c.r[26] = c.r[3].wrapping_add(0xc);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                c.r[0] = c.r[0].wrapping_mul(0x14);
                c.r[21] = c.r[3].wrapping_add(0x2);
                c.r[20] = c.r[21].rotate_left(2) & 0xfffffffc;
                c.r[25] = c.r[30].wrapping_add(c.r[0]);
                c.stw(c.r[25].wrapping_add(0x8), c.r[20]);
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x80164788; f_801643d8(c);
                pc = 0x80164788;
            }
            0x80164788 => {
                c.stw(c.r[25].wrapping_add(0xc), c.r[3]);
                c.r[5] = c.r[20] | c.r[20];
                c.r[4] = 0x0;
                c.r[3] = c.lwz(c.r[25].wrapping_add(0xc));
                c.lr = 0x8016479c; f_8000540c(c);
                pc = 0x8016479c;
            }
            0x8016479c => {
                { let ea = c.r[25].wrapping_add(0x10); let v = c.f[30]; c.stfs(ea, v); }
                { let v = c.r[21] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.r[3] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.lwz(c.r[25]);
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[25].wrapping_add(0x4), c.r[0]);
                pc = 0x801647c8; continue;
                pc = 0x801647b8;
            }
            0x801647b8 => {
                c.r[3] = c.lwz(c.r[25].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[25].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[25].wrapping_add(0x4), c.r[0]);
                pc = 0x801647c8;
            }
            0x801647c8 => {
                c.r[0] = c.lwz(c.r[25].wrapping_add(0x4));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(0) { pc = 0x801647b8; continue; }
                pc = 0x801647d4;
            }
            0x801647d4 => {
                c.r[0] = 0x0;
                c.stw(c.r[25], c.r[0]);
                c.stw(c.r[25].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.lwz(c.r[26]);
                c.r[0] = c.lwz(c.r[25]);
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[25].wrapping_add(0x4), c.r[0]);
                pc = 0x80164808; continue;
                pc = 0x801647f8;
            }
            0x801647f8 => {
                c.r[3] = c.lwz(c.r[25].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[25].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[25].wrapping_add(0x4), c.r[0]);
                pc = 0x80164808;
            }
            0x80164808 => {
                c.r[0] = c.lwz(c.r[25].wrapping_add(0x4));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(0) { pc = 0x801647f8; continue; }
                pc = 0x80164814;
            }
            0x80164814 => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x80164818;
            }
            0x80164818 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x80164754; continue; }
                pc = 0x80164824;
            }
            0x80164824 => {
                c.r[0] = c.r[23].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[29].wrapping_add(c.r[0]);
                c.r[24] = c.r[3].wrapping_add(0x14);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x14));
                c.r[0] = c.r[23].wrapping_mul(0x3c);
                c.r[21] = c.r[3].wrapping_add(0x2);
                c.r[20] = c.r[21].rotate_left(2) & 0xfffffffc;
                c.r[23] = c.r[30].wrapping_add(c.r[0]);
                c.stw(c.r[23].wrapping_add(0x30), c.r[20]);
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x80164850; f_801643d8(c);
                pc = 0x80164850;
            }
            0x80164850 => {
                c.stw(c.r[23].wrapping_add(0x34), c.r[3]);
                c.r[5] = c.r[20] | c.r[20];
                c.r[4] = 0x0;
                c.r[3] = c.lwz(c.r[23].wrapping_add(0x34));
                c.lr = 0x80164864; f_8000540c(c);
                pc = 0x80164864;
            }
            0x80164864 => {
                { let ea = c.r[23].wrapping_add(0x38); let v = c.f[30]; c.stfs(ea, v); }
                { let v = c.r[21] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.r[3] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.lwz(c.r[23].wrapping_add(0x28));
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[23].wrapping_add(0x2c), c.r[0]);
                pc = 0x80164890; continue;
                pc = 0x80164880;
            }
            0x80164880 => {
                c.r[3] = c.lwz(c.r[23].wrapping_add(0x2c));
                c.r[0] = c.lwz(c.r[23].wrapping_add(0x30));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[23].wrapping_add(0x2c), c.r[0]);
                pc = 0x80164890;
            }
            0x80164890 => {
                c.r[0] = c.lwz(c.r[23].wrapping_add(0x2c));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(0) { pc = 0x80164880; continue; }
                pc = 0x8016489c;
            }
            0x8016489c => {
                c.r[0] = 0x0;
                c.stw(c.r[23].wrapping_add(0x28), c.r[0]);
                c.stw(c.r[23].wrapping_add(0x2c), c.r[0]);
                c.r[3] = c.lwz(c.r[24]);
                c.r[0] = c.lwz(c.r[23].wrapping_add(0x28));
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[23].wrapping_add(0x2c), c.r[0]);
                pc = 0x801648d0; continue;
                pc = 0x801648c0;
            }
            0x801648c0 => {
                c.r[3] = c.lwz(c.r[23].wrapping_add(0x2c));
                c.r[0] = c.lwz(c.r[23].wrapping_add(0x30));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[23].wrapping_add(0x2c), c.r[0]);
                pc = 0x801648d0;
            }
            0x801648d0 => {
                c.r[0] = c.lwz(c.r[23].wrapping_add(0x2c));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(0) { pc = 0x801648c0; continue; }
                pc = 0x801648dc;
            }
            0x801648dc => {
                c.r[3] = c.r[31].rotate_left(2) & 0x3fc;
                c.r[31] = c.r[31].wrapping_add(0x1);
                c.r[0] = c.r[3].wrapping_add(0x190);
                { let ea = c.r[30].wrapping_add(c.r[0]); let v = c.f[30]; c.stfs(ea, v); }
                pc = 0x801648ec;
            }
            0x801648ec => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x8016463c; continue; }
                pc = 0x801648f8;
            }
            0x801648f8 => {
                { let ea = c.r[30].wrapping_add(0x168); let v = c.f[24]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffff9e70); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[30].wrapping_add(0x19c); let v = c.f[25]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x1a8); let v = c.f[27]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x1a0); let v = c.f[26]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x1a0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(0) { pc = 0x8016491c; continue; }
                pc = 0x80164918;
            }
            0x80164918 => {
                { let ea = c.r[30].wrapping_add(0x1a0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8016491c;
            }
            0x8016491c => {
                { let ea = c.r[2].wrapping_add(0xffff9e74); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[30].wrapping_add(0x1a0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9e50); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9e70); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[2].wrapping_add(0xffff9e54); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.fcmp(0, c.f[0], c.f[31]);
                { let v = s(c.f[2] + c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[3] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[30].wrapping_add(0x1a0); let v = c.f[0]; c.stfs(ea, v); }
                if c.crbit(2) { pc = 0x801649b0; continue; }
                pc = 0x80164948;
            }
            0x80164948 => {
                { let ea = c.r[2].wrapping_add(0xffff9e64); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[21] = 0x0;
                { let v = s(c.f[0] * c.f[31]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.stw(c.r[30].wrapping_add(0x1a4), c.r[0]);
                pc = 0x801649a0; continue;
                pc = 0x80164968;
            }
            0x80164968 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x1a4));
                c.r[3] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.lr = 0x80164974; f_801643d8(c);
                pc = 0x80164974;
            }
            0x80164974 => {
                c.r[0] = c.r[21].rotate_left(2) & 0x3fc;
                c.r[4] = 0x0;
                c.r[20] = c.r[30].wrapping_add(c.r[0]);
                c.stw(c.r[20].wrapping_add(0x1ac), c.r[3]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x1a4));
                c.r[3] = c.lwz(c.r[20].wrapping_add(0x1ac));
                c.r[5] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.lr = 0x80164994; f_8000540c(c);
                pc = 0x80164994;
            }
            0x80164994 => {
                c.r[0] = c.lwz(c.r[20].wrapping_add(0x1ac));
                c.r[21] = c.r[21].wrapping_add(0x1);
                c.stw(c.r[20].wrapping_add(0x1b8), c.r[0]);
                pc = 0x801649a0;
            }
            0x801649a0 => {
                c.r[0] = c.r[21].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x80164968; continue; }
                pc = 0x801649ac;
            }
            0x801649ac => {
                pc = 0x801649d0; continue;
                pc = 0x801649b0;
            }
            0x801649b0 => {
                c.r[0] = 0x0;
                c.stw(c.r[30].wrapping_add(0x1a4), c.r[0]);
                c.stw(c.r[30].wrapping_add(0x1b8), c.r[0]);
                c.stw(c.r[30].wrapping_add(0x1ac), c.r[0]);
                c.stw(c.r[30].wrapping_add(0x1bc), c.r[0]);
                c.stw(c.r[30].wrapping_add(0x1b0), c.r[0]);
                c.stw(c.r[30].wrapping_add(0x1c0), c.r[0]);
                c.stw(c.r[30].wrapping_add(0x1b4), c.r[0]);
                pc = 0x801649d0;
            }
            0x801649d0 => {
                c.r[3] = 0x1;
                pc = 0x801649d4;
            }
            0x801649d4 => {
                { let ea = c.r[1].wrapping_add(0xb8); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0xb0); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0xa8); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0xa0); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x98); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                { let ea = c.r[1].wrapping_add(0x90); c.f[29] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = c.psq_load(ea, false, 0); c.f[28] = x; c.ps1[28] = y; }
                { let ea = c.r[1].wrapping_add(0x80); c.f[28] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = c.psq_load(ea, false, 0); c.f[27] = x; c.ps1[27] = y; }
                { let ea = c.r[1].wrapping_add(0x70); c.f[27] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = c.psq_load(ea, false, 0); c.f[26] = x; c.ps1[26] = y; }
                { let ea = c.r[1].wrapping_add(0x60); c.f[26] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = c.psq_load(ea, false, 0); c.f[25] = x; c.ps1[25] = y; }
                { let ea = c.r[1].wrapping_add(0x50); c.f[25] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = c.psq_load(ea, false, 0); c.f[24] = x; c.ps1[24] = y; }
                c.r[11] = c.r[1].wrapping_add(0x40);
                { let ea = c.r[1].wrapping_add(0x40); c.f[24] = c.lfd(ea); }
                c.lr = 0x80164a1c; f_800c4808(c);
                pc = 0x80164a1c;
            }
            0x80164a1c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xc0);
                if true { return; }
                return;
            }
            _ => panic!("f_80164520: bad pc {:#x}", pc),
        }
    }
}

/// ReverbHIModify (0x80164a2c, 532 bytes)
#[inline(never)]
pub fn f_80164a2c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164a2c) { return; }
        if c.prof { crate::prof::enter(c, 0x80164a2c); f_80164a2c_b(c, 0x80164a2c); crate::prof::exit(c, 0x80164a2c); return; }
    }
    f_80164a2c_b(c, 0x80164a2c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164a2c_at(c: &mut Cpu, pc: u32) { f_80164a2c_b(c, pc) }
#[inline(always)]
fn f_80164a2c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164a2c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164a2c, pc); }
        match pc {
            0x80164a2c => {
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = (c.f[29], c.ps1[29]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[28]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = (c.f[28], c.ps1[28]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[27]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = (c.f[27], c.ps1[27]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[26]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x18); let (x, y) = (c.f[26], c.ps1[26]); c.psq_store(ea, x, y, false, 0); }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.f[26] = c.f[1];
                { let ea = c.r[2].wrapping_add(0xffff9e50); c.f[7] = c.lfs(ea); c.ps1[7] = c.f[7]; }
                c.f[27] = c.f[2];
                c.r[30] = c.r[3] | c.r[3];
                c.f[28] = c.f[3];
                c.f[29] = c.f[4];
                c.f[30] = c.f[5];
                c.f[31] = c.f[6];
                c.fcmp(0, c.f[26], c.f[7]);
                if c.crbit(0) { pc = 0x80164b00; continue; }
                pc = 0x80164a98;
            }
            0x80164a98 => {
                { let ea = c.r[2].wrapping_add(0xffff9e54); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[26], c.f[1]);
                if c.crbit(1) { pc = 0x80164b00; continue; }
                pc = 0x80164aa4;
            }
            0x80164aa4 => {
                { let ea = c.r[2].wrapping_add(0xffff9e58); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[27], c.f[0]);
                if c.crbit(0) { pc = 0x80164b00; continue; }
                pc = 0x80164ab0;
            }
            0x80164ab0 => {
                { let ea = c.r[2].wrapping_add(0xffff9e5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[27], c.f[0]);
                if c.crbit(1) { pc = 0x80164b00; continue; }
                pc = 0x80164abc;
            }
            0x80164abc => {
                c.fcmp(0, c.f[28], c.f[7]);
                if c.crbit(0) { pc = 0x80164b00; continue; }
                pc = 0x80164ac4;
            }
            0x80164ac4 => {
                c.fcmp(0, c.f[28], c.f[1]);
                if c.crbit(1) { pc = 0x80164b00; continue; }
                pc = 0x80164acc;
            }
            0x80164acc => {
                c.fcmp(0, c.f[31], c.f[7]);
                if c.crbit(0) { pc = 0x80164b00; continue; }
                pc = 0x80164ad4;
            }
            0x80164ad4 => {
                c.fcmp(0, c.f[31], c.f[1]);
                if c.crbit(1) { pc = 0x80164b00; continue; }
                pc = 0x80164adc;
            }
            0x80164adc => {
                c.fcmp(0, c.f[29], c.f[7]);
                if c.crbit(0) { pc = 0x80164b00; continue; }
                pc = 0x80164ae4;
            }
            0x80164ae4 => {
                c.fcmp(0, c.f[29], c.f[1]);
                if c.crbit(1) { pc = 0x80164b00; continue; }
                pc = 0x80164aec;
            }
            0x80164aec => {
                c.fcmp(0, c.f[30], c.f[7]);
                if c.crbit(0) { pc = 0x80164b00; continue; }
                pc = 0x80164af4;
            }
            0x80164af4 => {
                { let ea = c.r[2].wrapping_add(0xffff9e80); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[30], c.f[0]);
                if !c.crbit(1) { pc = 0x80164b08; continue; }
                pc = 0x80164b00;
            }
            0x80164b00 => {
                c.r[3] = 0x0;
                pc = 0x80164bf8; continue;
                pc = 0x80164b08;
            }
            0x80164b08 => {
                { let ea = c.r[30].wrapping_add(0x168); let v = c.f[26]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffff9e70); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[30].wrapping_add(0x19c); let v = c.f[28]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x1a8); let v = c.f[31]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x1a0); let v = c.f[29]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x1a0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(0) { pc = 0x80164b2c; continue; }
                pc = 0x80164b28;
            }
            0x80164b28 => {
                { let ea = c.r[30].wrapping_add(0x1a0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80164b2c;
            }
            0x80164b2c => {
                { let ea = c.r[2].wrapping_add(0xffff9e74); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[31] = 0x0;
                { let ea = c.r[30].wrapping_add(0x1a0); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[2].wrapping_add(0xffff9e70); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9e54); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[2] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[30].wrapping_add(0x1a0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80164b6c; continue;
                pc = 0x80164b54;
            }
            0x80164b54 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.r[3] = c.r[0].wrapping_mul(0x14);
                c.r[0] = c.r[3].wrapping_add(0xc);
                c.r[3] = c.lwz(c.r[30].wrapping_add(c.r[0]));
                c.lr = 0x80164b68; f_80164400(c);
                pc = 0x80164b68;
            }
            0x80164b68 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80164b6c;
            }
            0x80164b6c => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x9);
                if c.crbit(0) { pc = 0x80164b54; continue; }
                pc = 0x80164b78;
            }
            0x80164b78 => {
                c.r[31] = 0x0;
                pc = 0x80164b98; continue;
                pc = 0x80164b80;
            }
            0x80164b80 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.r[3] = c.r[0].wrapping_mul(0x14);
                c.r[0] = c.r[3].wrapping_add(0xc0);
                c.r[3] = c.lwz(c.r[30].wrapping_add(c.r[0]));
                c.lr = 0x80164b94; f_80164400(c);
                pc = 0x80164b94;
            }
            0x80164b94 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80164b98;
            }
            0x80164b98 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x9);
                if c.crbit(0) { pc = 0x80164b80; continue; }
                pc = 0x80164ba4;
            }
            0x80164ba4 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x1a4));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80164bd8; continue; }
                pc = 0x80164bb0;
            }
            0x80164bb0 => {
                c.r[31] = 0x0;
                pc = 0x80164bcc; continue;
                pc = 0x80164bb8;
            }
            0x80164bb8 => {
                c.r[3] = c.r[31].rotate_left(2) & 0x3fc;
                c.r[0] = c.r[3].wrapping_add(0x1ac);
                c.r[3] = c.lwz(c.r[30].wrapping_add(c.r[0]));
                c.lr = 0x80164bc8; f_80164400(c);
                pc = 0x80164bc8;
            }
            0x80164bc8 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80164bcc;
            }
            0x80164bcc => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x80164bb8; continue; }
                pc = 0x80164bd8;
            }
            0x80164bd8 => {
                c.f[1] = c.f[26];
                c.r[3] = c.r[30] | c.r[30];
                c.f[2] = c.f[27];
                c.f[3] = c.f[28];
                c.f[4] = c.f[29];
                c.f[5] = c.f[30];
                c.f[6] = c.f[31];
                c.lr = 0x80164bf8; f_80164520(c);
                pc = 0x80164bf8;
            }
            0x80164bf8 => {
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x60); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x50); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                { let ea = c.r[1].wrapping_add(0x40); c.f[29] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = c.psq_load(ea, false, 0); c.f[28] = x; c.ps1[28] = y; }
                { let ea = c.r[1].wrapping_add(0x30); c.f[28] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[27] = x; c.ps1[27] = y; }
                { let ea = c.r[1].wrapping_add(0x20); c.f[27] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[26] = x; c.ps1[26] = y; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[26] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x74));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { return; }
                return;
            }
            _ => panic!("f_80164a2c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80164C40 (0x80164c40, 400 bytes)
#[inline(never)]
pub fn f_80164c40(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164c40) { return; }
        if c.prof { crate::prof::enter(c, 0x80164c40); f_80164c40_b(c, 0x80164c40); crate::prof::exit(c, 0x80164c40); return; }
    }
    f_80164c40_b(c, 0x80164c40)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164c40_at(c: &mut Cpu, pc: u32) { f_80164c40_b(c, pc) }
#[inline(always)]
fn f_80164c40_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164c40;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164c40, pc); }
        match pc {
            0x80164c40 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[14]; c.stfd(ea, v); }
                c.r[5] = 0x80480000;
                { let ea = c.r[5].wrapping_add(0xffffd528); c.f[0] = c.lfd(ea); }
                c.r[5] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[5]);
                { let (x, y) = (c.f[2], c.f[1]); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (c.f[1], c.f[2]); c.f[4] = x; c.ps1[4] = y; }
                c.r[5] = 0x80480000;
                { let ea = c.r[5].wrapping_add(0xffffd530); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                c.r[5] = 0x4f;
                c.mtspr(9, c.r[5]);
                c.r[10] = 0xfffffff8;
                c.r[11] = 0xfffffffc;
                { let (x, y) = (s(c.f[4] * c.f[5]), s(c.ps1[4] * c.f[5])); c.f[4] = x; c.ps1[4] = y; }
                c.r[6] = c.lwz(c.r[3]);
                c.r[7] = c.lwz(c.r[4]);
                c.r[6] = c.r[6] ^ 0x80000000;
                c.r[8] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[7] = c.r[7] ^ 0x80000000;
                c.r[9] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[8] = c.r[8] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[6]);
                c.r[9] = c.r[9] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[9]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[5] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x10); c.f[6] = c.lfd(ea); }
                { let v = s(c.f[5] - c.f[0]); c.f[5] = v; c.ps1[5] = v; }
                { let ea = c.r[1].wrapping_add(0x18); c.f[7] = c.lfd(ea); }
                { let v = s(c.f[6] - c.f[0]); c.f[6] = v; c.ps1[6] = v; }
                { let ea = c.r[1].wrapping_add(0x20); c.f[8] = c.lfd(ea); }
                { let v = s(c.f[7] - c.f[0]); c.f[7] = v; c.ps1[7] = v; }
                { let v = s(c.f[8] - c.f[0]); c.f[8] = v; c.ps1[8] = v; }
                pc = 0x80164cd8;
            }
            0x80164cd8 => {
                { let (x, y) = (c.f[5], c.f[6]); c.f[9] = x; c.ps1[9] = y; }
                { let ea = c.r[3].wrapping_add(0x8); c.r[6] = c.lwz(ea); c.r[3] = ea; }
                { let (x, y) = (c.f[7], c.f[8]); c.f[10] = x; c.ps1[10] = y; }
                { let ea = c.r[4].wrapping_add(0x8); c.r[7] = c.lwz(ea); c.r[4] = ea; }
                c.r[6] = c.r[6] ^ 0x80000000;
                c.r[8] = c.lwz(c.r[3].wrapping_add(0x4));
                { let (x, y) = (s(c.f[9] * c.f[3]), s(c.ps1[9] * c.ps1[3])); c.f[11] = x; c.ps1[11] = y; }
                c.r[7] = c.r[7] ^ 0x80000000;
                { let (x, y) = (s(c.f[9] * c.f[4]), s(c.ps1[9] * c.ps1[4])); c.f[12] = x; c.ps1[12] = y; }
                c.r[9] = c.lwz(c.r[4].wrapping_add(0x4));
                { let (x, y) = (s(c.f[10] * c.f[3]), s(c.ps1[10] * c.ps1[3])); c.f[13] = x; c.ps1[13] = y; }
                c.r[8] = c.r[8] ^ 0x80000000;
                { let (x, y) = (s(c.f[10] * c.f[4]), s(c.ps1[10] * c.ps1[4])); c.f[14] = x; c.ps1[14] = y; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[6]);
                { let (x, y) = (s(c.f[11] + c.ps1[11]), s(c.ps1[11])); c.f[11] = x; c.ps1[11] = y; }
                c.r[9] = c.r[9] ^ 0x80000000;
                { let (x, y) = (s(c.f[12] + c.ps1[12]), s(c.ps1[12])); c.f[12] = x; c.ps1[12] = y; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[7]);
                { let (x, y) = (s(c.f[13] + c.ps1[13]), s(c.ps1[13])); c.f[13] = x; c.ps1[13] = y; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[8]);
                { let (x, y) = (s(c.f[14] + c.ps1[14]), s(c.ps1[14])); c.f[14] = x; c.ps1[14] = y; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[9]);
                c.f[11] = fctiw(c.f[11]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[5] = c.lfd(ea); }
                c.f[12] = fctiw(c.f[12]);
                { let ea = c.r[1].wrapping_add(0x10); c.f[6] = c.lfd(ea); }
                c.f[13] = fctiw(c.f[13]);
                { let v = s(c.f[5] - c.f[0]); c.f[5] = v; c.ps1[5] = v; }
                c.f[14] = fctiw(c.f[14]);
                { let ea = c.r[1].wrapping_add(0x18); c.f[7] = c.lfd(ea); }
                c.stw(c.r[10].wrapping_add(c.r[3]), c.f[11].to_bits() as u32);
                { let v = s(c.f[6] - c.f[0]); c.f[6] = v; c.ps1[6] = v; }
                c.stw(c.r[10].wrapping_add(c.r[4]), c.f[12].to_bits() as u32);
                { let ea = c.r[1].wrapping_add(0x20); c.f[8] = c.lfd(ea); }
                c.stw(c.r[11].wrapping_add(c.r[3]), c.f[13].to_bits() as u32);
                { let v = s(c.f[7] - c.f[0]); c.f[7] = v; c.ps1[7] = v; }
                c.stw(c.r[11].wrapping_add(c.r[4]), c.f[14].to_bits() as u32);
                { let v = s(c.f[8] - c.f[0]); c.f[8] = v; c.ps1[8] = v; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80164cd8; continue; }
                pc = 0x80164d74;
            }
            0x80164d74 => {
                { let (x, y) = (c.f[5], c.f[6]); c.f[9] = x; c.ps1[9] = y; }
                c.r[3] = c.r[3].wrapping_add(0x8);
                { let (x, y) = (c.f[7], c.f[8]); c.f[10] = x; c.ps1[10] = y; }
                c.r[4] = c.r[4].wrapping_add(0x8);
                { let (x, y) = (s(c.f[9] * c.f[3]), s(c.ps1[9] * c.ps1[3])); c.f[11] = x; c.ps1[11] = y; }
                { let (x, y) = (s(c.f[9] * c.f[4]), s(c.ps1[9] * c.ps1[4])); c.f[12] = x; c.ps1[12] = y; }
                { let (x, y) = (s(c.f[10] * c.f[3]), s(c.ps1[10] * c.ps1[3])); c.f[13] = x; c.ps1[13] = y; }
                { let (x, y) = (s(c.f[10] * c.f[4]), s(c.ps1[10] * c.ps1[4])); c.f[14] = x; c.ps1[14] = y; }
                { let (x, y) = (s(c.f[11] + c.ps1[11]), s(c.ps1[11])); c.f[11] = x; c.ps1[11] = y; }
                { let (x, y) = (s(c.f[12] + c.ps1[12]), s(c.ps1[12])); c.f[12] = x; c.ps1[12] = y; }
                { let (x, y) = (s(c.f[13] + c.ps1[13]), s(c.ps1[13])); c.f[13] = x; c.ps1[13] = y; }
                { let (x, y) = (s(c.f[14] + c.ps1[14]), s(c.ps1[14])); c.f[14] = x; c.ps1[14] = y; }
                c.f[11] = fctiw(c.f[11]);
                c.f[12] = fctiw(c.f[12]);
                c.f[13] = fctiw(c.f[13]);
                c.f[14] = fctiw(c.f[14]);
                c.stw(c.r[10].wrapping_add(c.r[3]), c.f[11].to_bits() as u32);
                c.stw(c.r[10].wrapping_add(c.r[4]), c.f[12].to_bits() as u32);
                c.stw(c.r[11].wrapping_add(c.r[3]), c.f[13].to_bits() as u32);
                c.stw(c.r[11].wrapping_add(c.r[4]), c.f[14].to_bits() as u32);
                { let ea = c.r[1].wrapping_add(0x28); c.f[14] = c.lfd(ea); }
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80164c40: bad pc {:#x}", pc),
        }
    }
}

/// HandleReverb (0x80164dd0, 1292 bytes)
#[inline(never)]
pub fn f_80164dd0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80164dd0) { return; }
        if c.prof { crate::prof::enter(c, 0x80164dd0); f_80164dd0_b(c, 0x80164dd0); crate::prof::exit(c, 0x80164dd0); return; }
    }
    f_80164dd0_b(c, 0x80164dd0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164dd0_at(c: &mut Cpu, pc: u32) { f_80164dd0_b(c, pc) }
#[inline(always)]
fn f_80164dd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80164dd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80164dd0, pc); }
        match pc {
            0x80164dd0 => {
                { let ea = c.r[1].wrapping_add(0xffffff40); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 14..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[14]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x68); let v = c.f[15]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x70); let v = c.f[16]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x78); let v = c.f[17]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x80); let v = c.f[18]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x88); let v = c.f[19]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x90); let v = c.f[20]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x98); let v = c.f[21]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0xa0); let v = c.f[22]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0xa8); let v = c.f[23]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0xb0); let v = c.f[24]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0xb8); let v = c.f[25]; c.stfd(ea, v); }
                c.stw(c.r[1].wrapping_add(0x50), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x54), c.r[4]);
                c.r[31] = 0x80480000;
                { let ea = c.r[31].wrapping_add(0xffffd534); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                c.r[31] = 0x80480000;
                { let ea = c.r[31].wrapping_add(0xffffd530); c.f[9] = c.lfs(ea); c.ps1[9] = c.f[9]; }
                c.r[31] = 0x80480000;
                { let ea = c.r[31].wrapping_add(0xffffd528); c.f[5] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x168); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4].wrapping_add(0x1a0); c.f[15] = c.lfs(ea); c.ps1[15] = c.f[15]; }
                { let ea = c.r[4].wrapping_add(0x19c); c.f[8] = c.lfs(ea); c.ps1[8] = c.f[8]; }
                { let v = s(c.f[8] * c.f[9]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[9] - c.f[3]); c.f[4] = v; c.ps1[4] = v; }
                c.r[30] = c.r[5].rotate_left(1) & 0xfffffffe;
                c.r[30] = c.r[30].wrapping_add(c.r[5]);
                c.r[31] = c.r[30].wrapping_mul(0x14);
                c.r[29] = c.r[4].wrapping_add(0xb4);
                c.r[29] = c.r[29].wrapping_add(c.r[31]);
                c.r[27] = c.r[4].wrapping_add(0x0);
                c.r[27] = c.r[27].wrapping_add(c.r[31]);
                c.r[31] = c.r[30].rotate_left(2) & 0xfffffffc;
                c.r[31] = c.r[31].wrapping_add(c.r[4]);
                { let ea = c.r[31].wrapping_add(0x16c); c.f[22] = c.lfs(ea); c.ps1[22] = c.f[22]; }
                { let ea = c.r[31].wrapping_add(0x170); c.f[23] = c.lfs(ea); c.ps1[23] = c.f[23]; }
                { let ea = c.r[31].wrapping_add(0x174); c.f[24] = c.lfs(ea); c.ps1[24] = c.f[24]; }
                c.r[31] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[31] = c.r[31].wrapping_add(c.r[4]);
                { let ea = c.r[31].wrapping_add(0x190); c.f[25] = c.lfs(ea); c.ps1[25] = c.f[25]; }
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x1a4));
                c.r[30] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0x58), c.r[30]);
                c.r[22] = c.r[31].wrapping_add(0xffffffff);
                c.r[22] = c.r[22].rotate_left(2) & 0xfffffffc;
                c.r[28] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[28] = c.r[28].wrapping_add(c.r[4]);
                c.cmps(7, c.r[31], 0x0);
                c.r[21] = c.lwz(c.r[29]);
                c.r[20] = c.lwz(c.r[29].wrapping_add(0x4));
                c.r[19] = c.lwz(c.r[29].wrapping_add(0x14));
                c.r[18] = c.lwz(c.r[29].wrapping_add(0x18));
                c.r[17] = c.lwz(c.r[29].wrapping_add(0x28));
                c.r[16] = c.lwz(c.r[29].wrapping_add(0x2c));
                { let ea = c.r[29].wrapping_add(0x10); c.f[16] = c.lfs(ea); c.ps1[16] = c.f[16]; }
                { let ea = c.r[29].wrapping_add(0x24); c.f[17] = c.lfs(ea); c.ps1[17] = c.f[17]; }
                { let ea = c.r[29].wrapping_add(0x38); c.f[18] = c.lfs(ea); c.ps1[18] = c.f[18]; }
                c.r[25] = c.lwz(c.r[29].wrapping_add(0x8));
                c.r[24] = c.lwz(c.r[29].wrapping_add(0x1c));
                c.r[23] = c.lwz(c.r[29].wrapping_add(0x30));
                c.r[4] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[5] = c.lwz(c.r[29].wrapping_add(0x20));
                c.r[6] = c.lwz(c.r[29].wrapping_add(0x34));
                c.r[12] = c.lwz(c.r[27]);
                c.r[11] = c.lwz(c.r[27].wrapping_add(0x4));
                c.r[10] = c.lwz(c.r[27].wrapping_add(0x14));
                c.r[9] = c.lwz(c.r[27].wrapping_add(0x18));
                c.r[8] = c.lwz(c.r[27].wrapping_add(0x28));
                c.r[7] = c.lwz(c.r[27].wrapping_add(0x2c));
                { let ea = c.r[27].wrapping_add(0x10); c.f[19] = c.lfs(ea); c.ps1[19] = c.f[19]; }
                { let ea = c.r[27].wrapping_add(0x24); c.f[20] = c.lfs(ea); c.ps1[20] = c.f[20]; }
                { let ea = c.r[27].wrapping_add(0x38); c.f[21] = c.lfs(ea); c.ps1[21] = c.f[21]; }
                c.r[15] = c.lwz(c.r[27].wrapping_add(0x8));
                c.r[14] = c.lwz(c.r[27].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[3]);
                c.r[30] = c.r[30] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0x5c), c.r[30]);
                { let ea = c.r[1].wrapping_add(0x58); c.f[12] = c.lfd(ea); }
                { let v = s(c.f[12] - c.f[5]); c.f[12] = v; c.ps1[12] = v; }
                c.r[31] = 0x9f;
                c.mtspr(9, c.r[31]);
                pc = 0x80164f1c;
            }
            0x80164f1c => {
                c.f[13] = c.f[12];
                if c.crbit(30) { pc = 0x80164f4c; continue; }
                pc = 0x80164f24;
            }
            0x80164f24 => {
                c.r[30] = c.lwz(c.r[28].wrapping_add(0x1ac));
                c.r[29] = c.lwz(c.r[28].wrapping_add(0x1b8));
                c.r[31] = c.r[22].wrapping_add(c.r[30]);
                c.r[29] = c.r[29].wrapping_add(0x4);
                { let ea = c.r[29].wrapping_add(0xfffffffc); c.f[13] = c.lfs(ea); c.ps1[13] = c.f[13]; }
                c.cmps(0, c.r[29], c.r[31]);
                { let ea = c.r[29].wrapping_add(0xfffffffc); let v = c.f[12]; c.stfs(ea, v); }
                if !c.crbit(2) { pc = 0x80164f48; continue; }
                pc = 0x80164f44;
            }
            0x80164f44 => {
                c.r[29] = c.r[30] | c.r[30];
                pc = 0x80164f48;
            }
            0x80164f48 => {
                c.stw(c.r[30].wrapping_add(0x1b8), c.r[29]);
                pc = 0x80164f4c;
            }
            0x80164f4c => {
                { let v = s(c.f[22].mul_add(c.f[16], c.f[13])); c.f[8] = v; c.ps1[8] = v; }
                { let ea = c.r[3].wrapping_add(0x4); c.r[29] = c.lwz(ea); c.r[3] = ea; }
                { let v = s(c.f[23].mul_add(c.f[17], c.f[13])); c.f[9] = v; c.ps1[9] = v; }
                { let ea = c.r[4].wrapping_add(c.r[21]); let v = c.f[8]; c.stfs(ea, v); }
                c.r[21] = c.r[21].wrapping_add(0x4);
                { let ea = c.r[5].wrapping_add(c.r[19]); let v = c.f[9]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(c.r[20]); c.f[14] = c.lfs(ea); c.ps1[14] = c.f[14]; }
                c.r[20] = c.r[20].wrapping_add(0x4);
                { let ea = c.r[5].wrapping_add(c.r[18]); c.f[17] = c.lfs(ea); c.ps1[17] = c.f[17]; }
                c.cmps(0, c.r[21], c.r[25]);
                c.cmps(1, c.r[20], c.r[25]);
                c.r[19] = c.r[19].wrapping_add(0x4);
                c.r[18] = c.r[18].wrapping_add(0x4);
                c.f[16] = c.f[14];
                c.cmps(5, c.r[19], c.r[24]);
                { let v = s(c.f[14] + c.f[17]); c.f[14] = v; c.ps1[14] = v; }
                c.cmps(6, c.r[18], c.r[24]);
                if !c.crbit(2) { pc = 0x80164f98; continue; }
                pc = 0x80164f94;
            }
            0x80164f94 => {
                c.r[21] = 0x0;
                pc = 0x80164f98;
            }
            0x80164f98 => {
                { let v = s(c.f[24].mul_add(c.f[18], c.f[13])); c.f[8] = v; c.ps1[8] = v; }
                if !c.crbit(6) { pc = 0x80164fa4; continue; }
                pc = 0x80164fa0;
            }
            0x80164fa0 => {
                c.r[20] = 0x0;
                pc = 0x80164fa4;
            }
            0x80164fa4 => {
                { let ea = c.r[6].wrapping_add(c.r[17]); let v = c.f[8]; c.stfs(ea, v); }
                c.r[17] = c.r[17].wrapping_add(0x4);
                if !c.crbit(22) { pc = 0x80164fb4; continue; }
                pc = 0x80164fb0;
            }
            0x80164fb0 => {
                c.r[19] = 0x0;
                pc = 0x80164fb4;
            }
            0x80164fb4 => {
                { let ea = c.r[6].wrapping_add(c.r[16]); c.f[18] = c.lfs(ea); c.ps1[18] = c.f[18]; }
                c.r[16] = c.r[16].wrapping_add(0x4);
                c.cmps(0, c.r[17], c.r[23]);
                if !c.crbit(26) { pc = 0x80164fc8; continue; }
                pc = 0x80164fc4;
            }
            0x80164fc4 => {
                c.r[18] = 0x0;
                pc = 0x80164fc8;
            }
            0x80164fc8 => {
                { let v = s(c.f[14] + c.f[18]); c.f[14] = v; c.ps1[14] = v; }
                c.cmps(1, c.r[16], c.r[23]);
                c.r[26] = c.lwz(c.r[27].wrapping_add(0xc));
                { let v = s(c.f[2].mul_add(c.f[19], c.f[14])); c.f[9] = v; c.ps1[9] = v; }
                if !c.crbit(2) { pc = 0x80164fe0; continue; }
                pc = 0x80164fdc;
            }
            0x80164fdc => {
                c.r[17] = 0x0;
                pc = 0x80164fe0;
            }
            0x80164fe0 => {
                if !c.crbit(6) { pc = 0x80164fe8; continue; }
                pc = 0x80164fe4;
            }
            0x80164fe4 => {
                c.r[16] = 0x0;
                pc = 0x80164fe8;
            }
            0x80164fe8 => {
                c.r[29] = c.r[29] ^ 0x80000000;
                { let ea = c.r[26].wrapping_add(c.r[12]); let v = c.f[9]; c.stfs(ea, v); }
                { let v = s(-(c.f[2].mul_add(c.f[9], -c.f[19]))); c.f[14] = v; c.ps1[14] = v; }
                c.r[12] = c.r[12].wrapping_add(0x4);
                { let ea = c.r[26].wrapping_add(c.r[11]); c.f[19] = c.lfs(ea); c.ps1[19] = c.f[19]; }
                c.cmps(5, c.r[12], c.r[15]);
                c.r[11] = c.r[11].wrapping_add(0x4);
                c.r[26] = c.lwz(c.r[27].wrapping_add(0x20));
                c.cmps(6, c.r[11], c.r[15]);
                { let v = s(c.f[2].mul_add(c.f[20], c.f[14])); c.f[8] = v; c.ps1[8] = v; }
                if !c.crbit(22) { pc = 0x80165018; continue; }
                pc = 0x80165014;
            }
            0x80165014 => {
                c.r[12] = 0x0;
                pc = 0x80165018;
            }
            0x80165018 => {
                c.stw(c.r[1].wrapping_add(0x5c), c.r[29]);
                { let ea = c.r[26].wrapping_add(c.r[10]); let v = c.f[8]; c.stfs(ea, v); }
                { let v = s(-(c.f[2].mul_add(c.f[8], -c.f[20]))); c.f[14] = v; c.ps1[14] = v; }
                c.r[10] = c.r[10].wrapping_add(0x4);
                if !c.crbit(26) { pc = 0x80165030; continue; }
                pc = 0x8016502c;
            }
            0x8016502c => {
                c.r[11] = 0x0;
                pc = 0x80165030;
            }
            0x80165030 => {
                { let ea = c.r[26].wrapping_add(c.r[9]); c.f[20] = c.lfs(ea); c.ps1[20] = c.f[20]; }
                c.cmps(0, c.r[10], c.r[14]);
                { let v = s(c.f[14] * c.f[6]); c.f[14] = v; c.ps1[14] = v; }
                c.r[9] = c.r[9].wrapping_add(0x4);
                c.cmps(1, c.r[9], c.r[14]);
                { let ea = c.r[1].wrapping_add(0x58); c.f[10] = c.lfd(ea); }
                { let v = s(c.f[15].mul_add(c.f[25], c.f[14])); c.f[14] = v; c.ps1[14] = v; }
                if !c.crbit(2) { pc = 0x80165054; continue; }
                pc = 0x80165050;
            }
            0x80165050 => {
                c.r[10] = 0x0;
                pc = 0x80165054;
            }
            0x80165054 => {
                c.r[26] = c.lwz(c.r[27].wrapping_add(0x34));
                { let v = s(c.f[2].mul_add(c.f[21], c.f[14])); c.f[9] = v; c.ps1[9] = v; }
                c.f[25] = c.f[14];
                if !c.crbit(6) { pc = 0x80165068; continue; }
                pc = 0x80165064;
            }
            0x80165064 => {
                c.r[9] = 0x0;
                pc = 0x80165068;
            }
            0x80165068 => {
                { let ea = c.r[26].wrapping_add(c.r[8]); let v = c.f[9]; c.stfs(ea, v); }
                { let v = s(-(c.f[2].mul_add(c.f[9], -c.f[21]))); c.f[14] = v; c.ps1[14] = v; }
                c.r[31] = c.lwz(c.r[27].wrapping_add(0x30));
                { let v = s(c.f[4] * c.f[12]); c.f[8] = v; c.ps1[8] = v; }
                { let ea = c.r[26].wrapping_add(c.r[7]); c.f[21] = c.lfs(ea); c.ps1[21] = c.f[21]; }
                c.r[8] = c.r[8].wrapping_add(0x4);
                c.r[7] = c.r[7].wrapping_add(0x4);
                { let v = s(c.f[3].mul_add(c.f[14], c.f[8])); c.f[14] = v; c.ps1[14] = v; }
                c.cmps(5, c.r[8], c.r[31]);
                c.cmps(6, c.r[7], c.r[31]);
                c.f[14] = fctiwz(c.f[14]);
                if !c.crbit(22) { pc = 0x8016509c; continue; }
                pc = 0x80165098;
            }
            0x80165098 => {
                c.r[8] = 0x0;
                pc = 0x8016509c;
            }
            0x8016509c => {
                if !c.crbit(26) { pc = 0x801650a4; continue; }
                pc = 0x801650a0;
            }
            0x801650a0 => {
                c.r[7] = 0x0;
                pc = 0x801650a4;
            }
            0x801650a4 => {
                c.r[31] = 0xfffffffc;
                { let v = s(c.f[10] - c.f[5]); c.f[12] = v; c.ps1[12] = v; }
                c.stw(c.r[3].wrapping_add(c.r[31]), c.f[14].to_bits() as u32);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80164f1c; continue; }
                pc = 0x801650b4;
            }
            0x801650b4 => {
                c.f[13] = c.f[12];
                if c.crbit(30) { pc = 0x801650e4; continue; }
                pc = 0x801650bc;
            }
            0x801650bc => {
                c.r[30] = c.lwz(c.r[28].wrapping_add(0x1ac));
                c.r[29] = c.lwz(c.r[28].wrapping_add(0x1b8));
                c.r[31] = c.r[22].wrapping_add(c.r[30]);
                c.r[29] = c.r[29].wrapping_add(0x4);
                { let ea = c.r[29].wrapping_add(0xfffffffc); c.f[13] = c.lfs(ea); c.ps1[13] = c.f[13]; }
                c.cmps(0, c.r[29], c.r[31]);
                { let ea = c.r[29].wrapping_add(0xfffffffc); let v = c.f[12]; c.stfs(ea, v); }
                if !c.crbit(2) { pc = 0x801650e0; continue; }
                pc = 0x801650dc;
            }
            0x801650dc => {
                c.r[29] = c.r[30] | c.r[30];
                pc = 0x801650e0;
            }
            0x801650e0 => {
                c.stw(c.r[30].wrapping_add(0x1b8), c.r[29]);
                pc = 0x801650e4;
            }
            0x801650e4 => {
                { let v = s(c.f[22].mul_add(c.f[16], c.f[13])); c.f[8] = v; c.ps1[8] = v; }
                { let v = s(c.f[23].mul_add(c.f[17], c.f[13])); c.f[9] = v; c.ps1[9] = v; }
                { let ea = c.r[4].wrapping_add(c.r[21]); let v = c.f[8]; c.stfs(ea, v); }
                c.r[21] = c.r[21].wrapping_add(0x4);
                { let ea = c.r[5].wrapping_add(c.r[19]); let v = c.f[9]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(c.r[20]); c.f[14] = c.lfs(ea); c.ps1[14] = c.f[14]; }
                c.r[20] = c.r[20].wrapping_add(0x4);
                { let ea = c.r[5].wrapping_add(c.r[18]); c.f[17] = c.lfs(ea); c.ps1[17] = c.f[17]; }
                c.cmps(0, c.r[21], c.r[25]);
                c.cmps(1, c.r[20], c.r[25]);
                c.r[19] = c.r[19].wrapping_add(0x4);
                c.r[18] = c.r[18].wrapping_add(0x4);
                c.f[16] = c.f[14];
                c.cmps(5, c.r[19], c.r[24]);
                { let v = s(c.f[14] + c.f[17]); c.f[14] = v; c.ps1[14] = v; }
                c.cmps(6, c.r[18], c.r[24]);
                if !c.crbit(2) { pc = 0x8016512c; continue; }
                pc = 0x80165128;
            }
            0x80165128 => {
                c.r[21] = 0x0;
                pc = 0x8016512c;
            }
            0x8016512c => {
                { let v = s(c.f[24].mul_add(c.f[18], c.f[13])); c.f[8] = v; c.ps1[8] = v; }
                if !c.crbit(6) { pc = 0x80165138; continue; }
                pc = 0x80165134;
            }
            0x80165134 => {
                c.r[20] = 0x0;
                pc = 0x80165138;
            }
            0x80165138 => {
                { let ea = c.r[6].wrapping_add(c.r[17]); let v = c.f[8]; c.stfs(ea, v); }
                c.r[17] = c.r[17].wrapping_add(0x4);
                if !c.crbit(22) { pc = 0x80165148; continue; }
                pc = 0x80165144;
            }
            0x80165144 => {
                c.r[19] = 0x0;
                pc = 0x80165148;
            }
            0x80165148 => {
                { let ea = c.r[6].wrapping_add(c.r[16]); c.f[18] = c.lfs(ea); c.ps1[18] = c.f[18]; }
                c.r[16] = c.r[16].wrapping_add(0x4);
                c.cmps(0, c.r[17], c.r[23]);
                if !c.crbit(26) { pc = 0x8016515c; continue; }
                pc = 0x80165158;
            }
            0x80165158 => {
                c.r[18] = 0x0;
                pc = 0x8016515c;
            }
            0x8016515c => {
                { let v = s(c.f[14] + c.f[18]); c.f[14] = v; c.ps1[14] = v; }
                c.cmps(1, c.r[16], c.r[23]);
                c.r[26] = c.lwz(c.r[27].wrapping_add(0xc));
                { let v = s(c.f[2].mul_add(c.f[19], c.f[14])); c.f[9] = v; c.ps1[9] = v; }
                if !c.crbit(2) { pc = 0x80165174; continue; }
                pc = 0x80165170;
            }
            0x80165170 => {
                c.r[17] = 0x0;
                pc = 0x80165174;
            }
            0x80165174 => {
                if !c.crbit(6) { pc = 0x8016517c; continue; }
                pc = 0x80165178;
            }
            0x80165178 => {
                c.r[16] = 0x0;
                pc = 0x8016517c;
            }
            0x8016517c => {
                { let ea = c.r[26].wrapping_add(c.r[12]); let v = c.f[9]; c.stfs(ea, v); }
                { let v = s(-(c.f[2].mul_add(c.f[9], -c.f[19]))); c.f[14] = v; c.ps1[14] = v; }
                c.r[12] = c.r[12].wrapping_add(0x4);
                { let ea = c.r[26].wrapping_add(c.r[11]); c.f[19] = c.lfs(ea); c.ps1[19] = c.f[19]; }
                c.cmps(5, c.r[12], c.r[15]);
                c.r[11] = c.r[11].wrapping_add(0x4);
                c.r[26] = c.lwz(c.r[27].wrapping_add(0x20));
                c.cmps(6, c.r[11], c.r[15]);
                { let v = s(c.f[2].mul_add(c.f[20], c.f[14])); c.f[8] = v; c.ps1[8] = v; }
                if !c.crbit(22) { pc = 0x801651a8; continue; }
                pc = 0x801651a4;
            }
            0x801651a4 => {
                c.r[12] = 0x0;
                pc = 0x801651a8;
            }
            0x801651a8 => {
                { let ea = c.r[26].wrapping_add(c.r[10]); let v = c.f[8]; c.stfs(ea, v); }
                { let v = s(-(c.f[2].mul_add(c.f[8], -c.f[20]))); c.f[14] = v; c.ps1[14] = v; }
                c.r[10] = c.r[10].wrapping_add(0x4);
                if !c.crbit(26) { pc = 0x801651bc; continue; }
                pc = 0x801651b8;
            }
            0x801651b8 => {
                c.r[11] = 0x0;
                pc = 0x801651bc;
            }
            0x801651bc => {
                { let ea = c.r[26].wrapping_add(c.r[9]); c.f[20] = c.lfs(ea); c.ps1[20] = c.f[20]; }
                c.cmps(0, c.r[10], c.r[14]);
                { let v = s(c.f[14] * c.f[6]); c.f[14] = v; c.ps1[14] = v; }
                c.r[9] = c.r[9].wrapping_add(0x4);
                c.cmps(1, c.r[9], c.r[14]);
                { let v = s(c.f[15].mul_add(c.f[25], c.f[14])); c.f[14] = v; c.ps1[14] = v; }
                if !c.crbit(2) { pc = 0x801651dc; continue; }
                pc = 0x801651d8;
            }
            0x801651d8 => {
                c.r[10] = 0x0;
                pc = 0x801651dc;
            }
            0x801651dc => {
                c.r[26] = c.lwz(c.r[27].wrapping_add(0x34));
                c.r[5] = c.lwz(c.r[1].wrapping_add(0x50));
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x54));
                { let v = s(c.f[2].mul_add(c.f[21], c.f[14])); c.f[9] = v; c.ps1[9] = v; }
                c.f[25] = c.f[14];
                if !c.crbit(6) { pc = 0x801651f8; continue; }
                pc = 0x801651f4;
            }
            0x801651f4 => {
                c.r[9] = 0x0;
                pc = 0x801651f8;
            }
            0x801651f8 => {
                { let ea = c.r[26].wrapping_add(c.r[8]); let v = c.f[9]; c.stfs(ea, v); }
                { let v = s(-(c.f[2].mul_add(c.f[9], -c.f[21]))); c.f[14] = v; c.ps1[14] = v; }
                c.r[29] = c.lwz(c.r[27].wrapping_add(0x30));
                { let v = s(c.f[4] * c.f[12]); c.f[8] = v; c.ps1[8] = v; }
                { let ea = c.r[26].wrapping_add(c.r[7]); c.f[21] = c.lfs(ea); c.ps1[21] = c.f[21]; }
                c.r[8] = c.r[8].wrapping_add(0x4);
                c.r[7] = c.r[7].wrapping_add(0x4);
                { let v = s(c.f[3].mul_add(c.f[14], c.f[8])); c.f[14] = v; c.ps1[14] = v; }
                c.cmps(5, c.r[8], c.r[29]);
                c.cmps(6, c.r[7], c.r[29]);
                c.f[14] = fctiwz(c.f[14]);
                if !c.crbit(22) { pc = 0x8016522c; continue; }
                pc = 0x80165228;
            }
            0x80165228 => {
                c.r[8] = 0x0;
                pc = 0x8016522c;
            }
            0x8016522c => {
                if !c.crbit(26) { pc = 0x80165234; continue; }
                pc = 0x80165230;
            }
            0x80165230 => {
                c.r[7] = 0x0;
                pc = 0x80165234;
            }
            0x80165234 => {
                c.r[30] = c.r[5].rotate_left(1) & 0xfffffffe;
                c.r[30] = c.r[30].wrapping_add(c.r[5]);
                c.r[31] = c.r[30].wrapping_mul(0x14);
                c.stw(c.r[3], c.f[14].to_bits() as u32);
                c.r[29] = c.r[4].wrapping_add(0xb4);
                c.r[29] = c.r[29].wrapping_add(c.r[31]);
                c.stw(c.r[29], c.r[21]);
                c.stw(c.r[29].wrapping_add(0x4), c.r[20]);
                c.stw(c.r[29].wrapping_add(0x14), c.r[19]);
                c.stw(c.r[29].wrapping_add(0x18), c.r[18]);
                c.stw(c.r[29].wrapping_add(0x28), c.r[17]);
                c.stw(c.r[29].wrapping_add(0x2c), c.r[16]);
                { let ea = c.r[29].wrapping_add(0x10); let v = c.f[16]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0x24); let v = c.f[17]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0x38); let v = c.f[18]; c.stfs(ea, v); }
                c.stw(c.r[27], c.r[12]);
                c.stw(c.r[27].wrapping_add(0x4), c.r[11]);
                c.stw(c.r[27].wrapping_add(0x14), c.r[10]);
                c.stw(c.r[27].wrapping_add(0x18), c.r[9]);
                c.stw(c.r[27].wrapping_add(0x28), c.r[8]);
                c.stw(c.r[27].wrapping_add(0x2c), c.r[7]);
                { let ea = c.r[27].wrapping_add(0x10); let v = c.f[19]; c.stfs(ea, v); }
                { let ea = c.r[27].wrapping_add(0x24); let v = c.f[20]; c.stfs(ea, v); }
                { let ea = c.r[27].wrapping_add(0x38); let v = c.f[21]; c.stfs(ea, v); }
                c.r[31] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[31] = c.r[31].wrapping_add(c.r[4]);
                { let ea = c.r[31].wrapping_add(0x190); let v = c.f[25]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x60); c.f[14] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x68); c.f[15] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x70); c.f[16] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x78); c.f[17] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x80); c.f[18] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x88); c.f[19] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x90); c.f[20] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x98); c.f[21] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0xa0); c.f[22] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0xa8); c.f[23] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0xb0); c.f[24] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0xb8); c.f[25] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 14..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0xc0);
                if true { return; }
                return;
            }
            _ => panic!("f_80164dd0: bad pc {:#x}", pc),
        }
    }
}

/// ReverbHICallback (0x801652dc, 224 bytes)
#[inline(never)]
pub fn f_801652dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801652dc) { return; }
        if c.prof { crate::prof::enter(c, 0x801652dc); f_801652dc_b(c, 0x801652dc); crate::prof::exit(c, 0x801652dc); return; }
    }
    f_801652dc_b(c, 0x801652dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801652dc_at(c: &mut Cpu, pc: u32) { f_801652dc_b(c, pc) }
#[inline(always)]
fn f_801652dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801652dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801652dc, pc); }
        match pc {
            0x801652dc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x20);
                c.lr = 0x801652f0; f_800c47d8(c);
                pc = 0x801652f0;
            }
            0x801652f0 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[31] = 0x0;
                pc = 0x80165398; continue;
                pc = 0x80165308;
            }
            0x80165308 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80165370; continue; }
                pc = 0x80165314;
            }
            0x80165314 => {
                if !c.crbit(0) { pc = 0x80165324; continue; }
                pc = 0x80165318;
            }
            0x80165318 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x80165330; continue; }
                pc = 0x80165320;
            }
            0x80165320 => {
                pc = 0x80165394; continue;
                pc = 0x80165324;
            }
            0x80165324 => {
                c.cmps(0, c.r[0], 0x3);
                if !c.crbit(0) { pc = 0x80165394; continue; }
                pc = 0x8016532c;
            }
            0x8016532c => {
                pc = 0x80165384; continue;
                pc = 0x80165330;
            }
            0x80165330 => {
                { let ea = c.r[2].wrapping_add(0xffff9e50); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[30].wrapping_add(0x1a8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.fcmp(0, c.f[0], c.f[2]);
                if c.crbit(2) { pc = 0x8016535c; continue; }
                pc = 0x80165340;
            }
            0x80165340 => {
                { let ea = c.r[2].wrapping_add(0xffff9e98); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[27] | c.r[27];
                { let ea = c.r[2].wrapping_add(0xffff9e54); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.r[28] | c.r[28];
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] - c.f[1]); c.f[2] = v; c.ps1[2] = v; }
                c.lr = 0x8016535c; f_80164c40(c);
                pc = 0x8016535c;
            }
            0x8016535c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.lr = 0x8016536c; f_80164dd0(c);
                pc = 0x8016536c;
            }
            0x8016536c => {
                pc = 0x80165394; continue;
                pc = 0x80165370;
            }
            0x80165370 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x1;
                c.lr = 0x80165380; f_80164dd0(c);
                pc = 0x80165380;
            }
            0x80165380 => {
                pc = 0x80165394; continue;
                pc = 0x80165384;
            }
            0x80165384 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x2;
                c.lr = 0x80165394; f_80164dd0(c);
                pc = 0x80165394;
            }
            0x80165394 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80165398;
            }
            0x80165398 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x80165308; continue; }
                pc = 0x801653a4;
            }
            0x801653a4 => {
                c.r[11] = c.r[1].wrapping_add(0x20);
                c.lr = 0x801653ac; f_800c4824(c);
                pc = 0x801653ac;
            }
            0x801653ac => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801652dc: bad pc {:#x}", pc),
        }
    }
}

/// fn_801653BC (0x801653bc, 8 bytes)
#[inline(never)]
pub fn f_801653bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801653bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801653bc); f_801653bc_b(c, 0x801653bc); crate::prof::exit(c, 0x801653bc); return; }
    }
    f_801653bc_b(c, 0x801653bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801653bc_at(c: &mut Cpu, pc: u32) { f_801653bc_b(c, pc) }
#[inline(always)]
fn f_801653bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801653bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801653bc, pc); }
        match pc {
            0x801653bc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa888));
                if true { return; }
                return;
            }
            _ => panic!("f_801653bc: bad pc {:#x}", pc),
        }
    }
}

/// std228_soundGetCurrentBackgroundMusic (0x801653c4, 8 bytes)
#[inline(never)]
pub fn f_801653c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801653c4) { return; }
        if c.prof { crate::prof::enter(c, 0x801653c4); f_801653c4_b(c, 0x801653c4); crate::prof::exit(c, 0x801653c4); return; }
    }
    f_801653c4_b(c, 0x801653c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801653c4_at(c: &mut Cpu, pc: u32) { f_801653c4_b(c, pc) }
#[inline(always)]
fn f_801653c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801653c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801653c4, pc); }
        match pc {
            0x801653c4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa88c));
                if true { return; }
                return;
            }
            _ => panic!("f_801653c4: bad pc {:#x}", pc),
        }
    }
}

/// soundPlayFanfare (0x801653cc, 276 bytes)
#[inline(never)]
pub fn f_801653cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801653cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801653cc); f_801653cc_b(c, 0x801653cc); crate::prof::exit(c, 0x801653cc); return; }
    }
    f_801653cc_b(c, 0x801653cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801653cc_at(c: &mut Cpu, pc: u32) { f_801653cc_b(c, pc) }
#[inline(always)]
fn f_801653cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801653cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801653cc, pc); }
        match pc {
            0x801653cc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.rc(c.r[28]);
                if c.crbit(2) { pc = 0x80165408; continue; }
                pc = 0x801653f8;
            }
            0x801653f8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8788));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[28], c.r[0]);
                if c.crbit(0) { pc = 0x80165410; continue; }
                pc = 0x80165408;
            }
            0x80165408 => {
                c.r[3] = 0x0;
                pc = 0x801654c0; continue;
                pc = 0x80165410;
            }
            0x80165410 => {
                c.r[0] = c.r[28].wrapping_mul(0xc);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8610));
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[3]);
                c.r[31] = c.lbz(c.r[4].wrapping_add(0x3));
                c.cmpu(0, c.r[31], c.r[0]);
                if !c.crbit(0) { pc = 0x801654bc; continue; }
                pc = 0x80165430;
            }
            0x80165430 => {
                c.r[3] = 0x10000;
                c.r[4] = 0x20;
                c.r[5] = 0x0;
                c.r[6] = 0x408;
                c.r[7] = 0x0;
                c.lr = 0x80165448; f_800f9418(c);
                pc = 0x80165448;
            }
            0x80165448 => {
                c.r[4] = c.r[3] | c.r[3];
                c.rc(c.r[4]);
                if c.crbit(2) { pc = 0x801654bc; continue; }
                pc = 0x80165450;
            }
            0x80165450 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8614));
                c.r[0] = c.r[31].rotate_left(3) & 0xfffffff8;
                c.r[3] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801654b0; continue; }
                pc = 0x80165464;
            }
            0x80165464 => {
                c.r[5] = 0x10000;
                c.lr = 0x8016546c; f_80165ee0(c);
                pc = 0x8016546c;
            }
            0x8016546c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x408;
                c.lr = 0x8016547c; f_80166b3c(c);
                pc = 0x8016547c;
            }
            0x8016547c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = 0x0;
                c.lr = 0x80165490; f_80166a50(c);
                pc = 0x80165490;
            }
            0x80165490 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80165498; f_801666bc(c);
                pc = 0x80165498;
            }
            0x80165498 => {
                c.cmps(0, c.r[3], 0x2);
                if !c.crbit(2) { pc = 0x801654a8; continue; }
                pc = 0x801654a0;
            }
            0x801654a0 => {
                c.lr = 0x801654a4; crate::stubs::f_800f0308(c);
                pc = 0x801654a4;
            }
            0x801654a4 => {
                pc = 0x80165490; continue;
                pc = 0x801654a8;
            }
            0x801654a8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x801654b0; f_80166b18(c);
                pc = 0x801654b0;
            }
            0x801654b0 => {
                c.r[3] = 0x0;
                c.r[4] = 0x408;
                c.lr = 0x801654bc; f_800f9210(c);
                pc = 0x801654bc;
            }
            0x801654bc => {
                c.r[3] = 0x1;
                pc = 0x801654c0;
            }
            0x801654c0 => {
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
            _ => panic!("f_801653cc: bad pc {:#x}", pc),
        }
    }
}

/// fn_801654E0 (0x801654e0, 104 bytes)
#[inline(never)]
pub fn f_801654e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801654e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801654e0); f_801654e0_b(c, 0x801654e0); crate::prof::exit(c, 0x801654e0); return; }
    }
    f_801654e0_b(c, 0x801654e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801654e0_at(c: &mut Cpu, pc: u32) { f_801654e0_b(c, pc) }
#[inline(always)]
fn f_801654e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801654e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801654e0, pc); }
        match pc {
            0x801654e0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8610));
                c.r[6] = c.r[6].wrapping_add(c.r[0]);
                c.r[6] = c.lbz(c.r[6].wrapping_add(0x3));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[6], c.r[0]);
                if !c.crbit(0) { pc = 0x8016551c; continue; }
                pc = 0x8016550c;
            }
            0x8016550c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8614));
                c.r[0] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[3] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x80165520; continue;
                pc = 0x8016551c;
            }
            0x8016551c => {
                c.r[3] = 0x0;
                pc = 0x80165520;
            }
            0x80165520 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80165530; continue; }
                pc = 0x80165528;
            }
            0x80165528 => {
                c.r[3] = 0x0;
                pc = 0x80165538; continue;
                pc = 0x80165530;
            }
            0x80165530 => {
                c.lr = 0x80165534; f_80165ee0(c);
                pc = 0x80165534;
            }
            0x80165534 => {
                c.r[3] = 0x1;
                pc = 0x80165538;
            }
            0x80165538 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801654e0: bad pc {:#x}", pc),
        }
    }
}

/// soundResumeMusic (0x80165548, 52 bytes)
#[inline(never)]
pub fn f_80165548(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165548) { return; }
        if c.prof { crate::prof::enter(c, 0x80165548); f_80165548_b(c, 0x80165548); crate::prof::exit(c, 0x80165548); return; }
    }
    f_80165548_b(c, 0x80165548)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165548_at(c: &mut Cpu, pc: u32) { f_80165548_b(c, pc) }
#[inline(always)]
fn f_80165548_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165548;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165548, pc); }
        match pc {
            0x80165548 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x8016556c; continue; }
                pc = 0x80165560;
            }
            0x80165560 => {
                c.r[4] = 0x3e8;
                c.r[5] = 0x0;
                c.lr = 0x8016556c; f_801667d8(c);
                pc = 0x8016556c;
            }
            0x8016556c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80165548: bad pc {:#x}", pc),
        }
    }
}

/// soundPauseMusic (0x8016557c, 88 bytes)
#[inline(never)]
pub fn f_8016557c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016557c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016557c); f_8016557c_b(c, 0x8016557c); crate::prof::exit(c, 0x8016557c); return; }
    }
    f_8016557c_b(c, 0x8016557c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016557c_at(c: &mut Cpu, pc: u32) { f_8016557c_b(c, pc) }
#[inline(always)]
fn f_8016557c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016557c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016557c, pc); }
        match pc {
            0x8016557c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x406;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80165598; f_801662e8(c);
                pc = 0x80165598;
            }
            0x80165598 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.r[31].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x801655bc; continue; }
                pc = 0x801655a8;
            }
            0x801655a8 => {
                c.r[4] = 0x32;
                c.r[5] = 0x0;
                c.lr = 0x801655b4; f_801668dc(c);
                pc = 0x801655b4;
            }
            0x801655b4 => {
                c.r[3] = 0x32;
                c.lr = 0x801655bc; f_80165d0c(c);
                pc = 0x801655bc;
            }
            0x801655bc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8016557c: bad pc {:#x}", pc),
        }
    }
}

/// fn_801655D4 (0x801655d4, 148 bytes)
#[inline(never)]
pub fn f_801655d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801655d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801655d4); f_801655d4_b(c, 0x801655d4); crate::prof::exit(c, 0x801655d4); return; }
    }
    f_801655d4_b(c, 0x801655d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801655d4_at(c: &mut Cpu, pc: u32) { f_801655d4_b(c, pc) }
#[inline(always)]
fn f_801655d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801655d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801655d4, pc); }
        match pc {
            0x801655d4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8790));
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(1) { pc = 0x8016560c; continue; }
                pc = 0x801655f8;
            }
            0x801655f8 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[3].wrapping_add(0x3548);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8016560c; stubs_gen::f_800dd970(c);
                pc = 0x8016560c;
            }
            0x8016560c => {
                c.r[0] = c.r[31].wrapping_mul(0x18);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8794));
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = 0x0;
                c.r[31] = c.r[4].wrapping_add(c.r[0]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x14));
                c.r[6] = c.lwz(c.r[31].wrapping_add(0x4));
                c.r[7] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[8] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[9] = c.lwz(c.r[31].wrapping_add(0x10));
                c.lr = 0x80165638; f_80166be0(c);
                pc = 0x80165638;
            }
            0x80165638 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80165654; continue; }
                pc = 0x80165640;
            }
            0x80165640 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x14));
                c.r[3] = c.r[3].wrapping_add(0x3568);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80165654; stubs_gen::f_800dd970(c);
                pc = 0x80165654;
            }
            0x80165654 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801655d4: bad pc {:#x}", pc),
        }
    }
}

/// std130_soundUnk_801653cc (0x80165668, 112 bytes)
#[inline(never)]
pub fn f_80165668(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165668) { return; }
        if c.prof { crate::prof::enter(c, 0x80165668); f_80165668_b(c, 0x80165668); crate::prof::exit(c, 0x80165668); return; }
    }
    f_80165668_b(c, 0x80165668)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165668_at(c: &mut Cpu, pc: u32) { f_80165668_b(c, pc) }
#[inline(always)]
fn f_80165668_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165668;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165668, pc); }
        match pc {
            0x80165668 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.lr = 0x80165690; f_8016557c(c);
                pc = 0x80165690;
            }
            0x80165690 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x801656a8; f_801653cc(c);
                pc = 0x801656a8;
            }
            0x801656a8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.r[0] | c.r[0];
                c.lr = 0x801656b8; f_80165548(c);
                pc = 0x801656b8;
            }
            0x801656b8 => {
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
            _ => panic!("f_80165668: bad pc {:#x}", pc),
        }
    }
}

/// fn_801656D8 (0x801656d8, 32 bytes)
#[inline(never)]
pub fn f_801656d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801656d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801656d8); f_801656d8_b(c, 0x801656d8); crate::prof::exit(c, 0x801656d8); return; }
    }
    f_801656d8_b(c, 0x801656d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801656d8_at(c: &mut Cpu, pc: u32) { f_801656d8_b(c, pc) }
#[inline(always)]
fn f_801656d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801656d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801656d8, pc); }
        match pc {
            0x801656d8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801656e8; f_80166098(c);
                pc = 0x801656e8;
            }
            0x801656e8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801656d8: bad pc {:#x}", pc),
        }
    }
}

/// std137_soundStopAll (0x801656f8, 32 bytes)
#[inline(never)]
pub fn f_801656f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801656f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801656f8); f_801656f8_b(c, 0x801656f8); crate::prof::exit(c, 0x801656f8); return; }
    }
    f_801656f8_b(c, 0x801656f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801656f8_at(c: &mut Cpu, pc: u32) { f_801656f8_b(c, pc) }
#[inline(always)]
fn f_801656f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801656f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801656f8, pc); }
        match pc {
            0x801656f8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80165708; f_80166670(c);
                pc = 0x80165708;
            }
            0x80165708 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801656f8: bad pc {:#x}", pc),
        }
    }
}

/// std109_soundPlayEffectVariant (0x80165718, 44 bytes)
#[inline(never)]
pub fn f_80165718(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165718) { return; }
        if c.prof { crate::prof::enter(c, 0x80165718); f_80165718_b(c, 0x80165718); crate::prof::exit(c, 0x80165718); return; }
    }
    f_80165718_b(c, 0x80165718)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165718_at(c: &mut Cpu, pc: u32) { f_80165718_b(c, pc) }
#[inline(always)]
fn f_80165718_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165718;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165718, pc); }
        match pc {
            0x80165718 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80165730; f_80166d48(c);
                pc = 0x80165730;
            }
            0x80165730 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80165718: bad pc {:#x}", pc),
        }
    }
}

/// std108_soundIsAnyPlaying (0x80165744, 68 bytes)
#[inline(never)]
pub fn f_80165744(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165744) { return; }
        if c.prof { crate::prof::enter(c, 0x80165744); f_80165744_b(c, 0x80165744); crate::prof::exit(c, 0x80165744); return; }
    }
    f_80165744_b(c, 0x80165744)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165744_at(c: &mut Cpu, pc: u32) { f_80165744_b(c, pc) }
#[inline(always)]
fn f_80165744_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165744;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165744, pc); }
        match pc {
            0x80165744 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffff9ea0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x80165764; f_800e01f4(c);
                pc = 0x80165764;
            }
            0x80165764 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8016576c; f_801664f0(c);
                pc = 0x8016576c;
            }
            0x8016576c => {
                c.r[0] = c.r[3].wrapping_neg();
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80165744: bad pc {:#x}", pc),
        }
    }
}

/// fn_80165788 (0x80165788, 72 bytes)
#[inline(never)]
pub fn f_80165788(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165788) { return; }
        if c.prof { crate::prof::enter(c, 0x80165788); f_80165788_b(c, 0x80165788); crate::prof::exit(c, 0x80165788); return; }
    }
    f_80165788_b(c, 0x80165788)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165788_at(c: &mut Cpu, pc: u32) { f_80165788_b(c, pc) }
#[inline(always)]
fn f_80165788_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165788;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165788, pc); }
        match pc {
            0x80165788 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x801657a4; f_800e01f4(c);
                pc = 0x801657a4;
            }
            0x801657a4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x801657b0; f_80166308(c);
                pc = 0x801657b0;
            }
            0x801657b0 => {
                c.r[0] = c.r[3].wrapping_neg();
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80165788: bad pc {:#x}", pc),
        }
    }
}

/// scriptSoundStop (0x801657d0, 40 bytes)
#[inline(never)]
pub fn f_801657d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801657d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801657d0); f_801657d0_b(c, 0x801657d0); crate::prof::exit(c, 0x801657d0); return; }
    }
    f_801657d0_b(c, 0x801657d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801657d0_at(c: &mut Cpu, pc: u32) { f_801657d0_b(c, pc) }
#[inline(always)]
fn f_801657d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801657d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801657d0, pc); }
        match pc {
            0x801657d0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa888));
                c.lr = 0x801657e8; f_801657f8(c);
                pc = 0x801657e8;
            }
            0x801657e8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801657d0: bad pc {:#x}", pc),
        }
    }
}

/// soundStop (0x801657f8, 260 bytes)
#[inline(never)]
pub fn f_801657f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801657f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801657f8); f_801657f8_b(c, 0x801657f8); crate::prof::exit(c, 0x801657f8); return; }
    }
    f_801657f8_b(c, 0x801657f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801657f8_at(c: &mut Cpu, pc: u32) { f_801657f8_b(c, pc) }
#[inline(always)]
fn f_801657f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801657f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801657f8, pc); }
        match pc {
            0x801657f8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if c.crbit(2) { pc = 0x8016582c; continue; }
                pc = 0x8016581c;
            }
            0x8016581c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8788));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x80165834; continue; }
                pc = 0x8016582c;
            }
            0x8016582c => {
                c.r[3] = 0x0;
                pc = 0x801658e0; continue;
                pc = 0x80165834;
            }
            0x80165834 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa88c));
                c.r[31] = 0x0;
                c.cmpu(0, c.r[0], c.r[29]);
                if !c.crbit(2) { pc = 0x80165850; continue; }
                pc = 0x80165844;
            }
            0x80165844 => {
                c.r[0] = 0x0;
                c.r[31] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa88c), c.r[0]);
                pc = 0x80165850;
            }
            0x80165850 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa888));
                c.cmpu(0, c.r[0], c.r[29]);
                if !c.crbit(2) { pc = 0x80165868; continue; }
                pc = 0x8016585c;
            }
            0x8016585c => {
                c.r[0] = 0x0;
                c.r[31] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa888), c.r[0]);
                pc = 0x80165868;
            }
            0x80165868 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80165870; f_80166084(c);
                pc = 0x80165870;
            }
            0x80165870 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x801658c8; continue; }
                pc = 0x80165878;
            }
            0x80165878 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.lr = 0x80165888; f_80165c70(c);
                pc = 0x80165888;
            }
            0x80165888 => {
                c.cmps(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x801658bc; continue; }
                pc = 0x80165890;
            }
            0x80165890 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.r[3].wrapping_add(0x358c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801658a4; stubs_gen::f_800dd970(c);
                pc = 0x801658a4;
            }
            0x801658a4 => {
                c.r[6] = c.r[31] | c.r[31];
                c.r[3] = 0x1;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x801658b8; f_80165a44(c);
                pc = 0x801658b8;
            }
            0x801658b8 => {
                pc = 0x801658e0; continue;
                pc = 0x801658bc;
            }
            0x801658bc => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801658c4; f_80166b18(c);
                pc = 0x801658c4;
            }
            0x801658c4 => {
                pc = 0x801658e0; continue;
                pc = 0x801658c8;
            }
            0x801658c8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.lr = 0x801658d8; f_801669e4(c);
                pc = 0x801658d8;
            }
            0x801658d8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801658e0; f_80166b18(c);
                pc = 0x801658e0;
            }
            0x801658e0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801657f8: bad pc {:#x}", pc),
        }
    }
}

/// fn_801658FC (0x801658fc, 128 bytes)
#[inline(never)]
pub fn f_801658fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801658fc) { return; }
        if c.prof { crate::prof::enter(c, 0x801658fc); f_801658fc_b(c, 0x801658fc); crate::prof::exit(c, 0x801658fc); return; }
    }
    f_801658fc_b(c, 0x801658fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801658fc_at(c: &mut Cpu, pc: u32) { f_801658fc_b(c, pc) }
#[inline(always)]
fn f_801658fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801658fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801658fc, pc); }
        match pc {
            0x801658fc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if c.crbit(2) { pc = 0x80165934; continue; }
                pc = 0x80165924;
            }
            0x80165924 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8788));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x8016593c; continue; }
                pc = 0x80165934;
            }
            0x80165934 => {
                c.r[3] = 0x0;
                pc = 0x80165960; continue;
                pc = 0x8016593c;
            }
            0x8016593c => {
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.lr = 0x8016594c; f_80165c70(c);
                pc = 0x8016594c;
            }
            0x8016594c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[6] = 0x0;
                c.lr = 0x80165960; f_80165a44(c);
                pc = 0x80165960;
            }
            0x80165960 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801658fc: bad pc {:#x}", pc),
        }
    }
}

/// std138_soundPlayEffectEx (0x8016597c, 128 bytes)
#[inline(never)]
pub fn f_8016597c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016597c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016597c); f_8016597c_b(c, 0x8016597c); crate::prof::exit(c, 0x8016597c); return; }
    }
    f_8016597c_b(c, 0x8016597c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016597c_at(c: &mut Cpu, pc: u32) { f_8016597c_b(c, pc) }
#[inline(always)]
fn f_8016597c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016597c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016597c, pc); }
        match pc {
            0x8016597c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if c.crbit(2) { pc = 0x801659b4; continue; }
                pc = 0x801659a4;
            }
            0x801659a4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8788));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x801659bc; continue; }
                pc = 0x801659b4;
            }
            0x801659b4 => {
                c.r[3] = 0x0;
                pc = 0x801659e0; continue;
                pc = 0x801659bc;
            }
            0x801659bc => {
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x801659cc; f_80165c70(c);
                pc = 0x801659cc;
            }
            0x801659cc => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[6] = 0x1;
                c.lr = 0x801659e0; f_80165a44(c);
                pc = 0x801659e0;
            }
            0x801659e0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8016597c: bad pc {:#x}", pc),
        }
    }
}

/// std105_soundPlayEffect (0x801659fc, 36 bytes)
#[inline(never)]
pub fn f_801659fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801659fc) { return; }
        if c.prof { crate::prof::enter(c, 0x801659fc); f_801659fc_b(c, 0x801659fc); crate::prof::exit(c, 0x801659fc); return; }
    }
    f_801659fc_b(c, 0x801659fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801659fc_at(c: &mut Cpu, pc: u32) { f_801659fc_b(c, pc) }
#[inline(always)]
fn f_801659fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801659fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801659fc, pc); }
        match pc {
            0x801659fc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80165a10; f_80165a44(c);
                pc = 0x80165a10;
            }
            0x80165a10 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801659fc: bad pc {:#x}", pc),
        }
    }
}

/// std104_soundSetBackgroundMusic (0x80165a20, 36 bytes)
#[inline(never)]
pub fn f_80165a20(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165a20) { return; }
        if c.prof { crate::prof::enter(c, 0x80165a20); f_80165a20_b(c, 0x80165a20); crate::prof::exit(c, 0x80165a20); return; }
    }
    f_80165a20_b(c, 0x80165a20)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165a20_at(c: &mut Cpu, pc: u32) { f_80165a20_b(c, pc) }
#[inline(always)]
fn f_80165a20_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165a20;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165a20, pc); }
        match pc {
            0x80165a20 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80165a34; f_80165a44(c);
                pc = 0x80165a34;
            }
            0x80165a34 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80165a20: bad pc {:#x}", pc),
        }
    }
}

/// soundPlay (0x80165a44, 556 bytes)
#[inline(never)]
pub fn f_80165a44(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165a44) { return; }
        if c.prof { crate::prof::enter(c, 0x80165a44); f_80165a44_b(c, 0x80165a44); crate::prof::exit(c, 0x80165a44); return; }
    }
    f_80165a44_b(c, 0x80165a44)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165a44_at(c: &mut Cpu, pc: u32) { f_80165a44_b(c, pc) }
#[inline(always)]
fn f_80165a44_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165a44;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165a44, pc); }
        match pc {
            0x80165a44 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.rc(c.r[26]);
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[29] = c.r[6] | c.r[6];
                if c.crbit(2) { pc = 0x80165a78; continue; }
                pc = 0x80165a68;
            }
            0x80165a68 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8788));
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[26], c.r[0]);
                if c.crbit(0) { pc = 0x80165a80; continue; }
                pc = 0x80165a78;
            }
            0x80165a78 => {
                c.r[3] = 0x0;
                pc = 0x80165c5c; continue;
                pc = 0x80165a80;
            }
            0x80165a80 => {
                c.r[4] = c.r[26].wrapping_mul(0xc);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[0] = c.lbz(c.r[5].wrapping_add(c.r[4]));
                c.r[0] = c.r[0].rotate_left(25) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80165af4; continue; }
                pc = 0x80165a98;
            }
            0x80165a98 => {
                c.cmps(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x80165adc; continue; }
                pc = 0x80165aa0;
            }
            0x80165aa0 => {
                c.lr = 0x80165aa4; f_801666bc(c);
                pc = 0x80165aa4;
            }
            0x80165aa4 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80165ad8; continue; }
                pc = 0x80165aac;
            }
            0x80165aac => {
                if c.crbit(0) { pc = 0x80165ad8; continue; }
                pc = 0x80165ab0;
            }
            0x80165ab0 => {
                c.cmps(0, c.r[3], 0x4);
                if !c.crbit(0) { pc = 0x80165ad8; continue; }
                pc = 0x80165ab8;
            }
            0x80165ab8 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x32;
                c.r[5] = 0x0;
                c.lr = 0x80165ac8; f_80166670(c);
                pc = 0x80165ac8;
            }
            0x80165ac8 => {
                c.r[3] = 0x32;
                c.lr = 0x80165ad0; f_80165d0c(c);
                pc = 0x80165ad0;
            }
            0x80165ad0 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x80165ad8; f_80166b18(c);
                pc = 0x80165ad8;
            }
            0x80165ad8 => {
                c.stw(c.r[13].wrapping_add(0xffffa888), c.r[26]);
                pc = 0x80165adc;
            }
            0x80165adc => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = 0x0;
                c.lr = 0x80165af0; f_80166a50(c);
                pc = 0x80165af0;
            }
            0x80165af0 => {
                pc = 0x80165c5c; continue;
                pc = 0x80165af4;
            }
            0x80165af4 => {
                c.cmps(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x80165b08; continue; }
                pc = 0x80165afc;
            }
            0x80165afc => {
                c.r[31] = 0x406;
                c.r[30] = 0x10000;
                pc = 0x80165b10; continue;
                pc = 0x80165b08;
            }
            0x80165b08 => {
                c.r[31] = 0x407;
                c.r[30] = 0x2000;
                pc = 0x80165b10;
            }
            0x80165b10 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8610));
                c.r[4] = c.r[5].wrapping_add(c.r[4]);
                c.r[25] = c.lbz(c.r[4].wrapping_add(0x3));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[25], c.r[0]);
                if c.crbit(0) { pc = 0x80165b44; continue; }
                pc = 0x80165b28;
            }
            0x80165b28 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[3].wrapping_add(0x35c4);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80165b3c; stubs_gen::f_800dd970(c);
                pc = 0x80165b3c;
            }
            0x80165b3c => {
                c.r[3] = 0x0;
                pc = 0x80165c5c; continue;
                pc = 0x80165b44;
            }
            0x80165b44 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.lr = 0x80165b50; f_801662e8(c);
                pc = 0x80165b50;
            }
            0x80165b50 => {
                c.r[24] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[24], c.r[26]);
                if c.crbit(2) { pc = 0x80165c30; continue; }
                pc = 0x80165b5c;
            }
            0x80165b5c => {
                c.r[0] = c.r[24].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x80165ba0; continue; }
                pc = 0x80165b68;
            }
            0x80165b68 => {
                c.lr = 0x80165b6c; f_801666bc(c);
                pc = 0x80165b6c;
            }
            0x80165b6c => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80165ba0; continue; }
                pc = 0x80165b74;
            }
            0x80165b74 => {
                if c.crbit(0) { pc = 0x80165ba0; continue; }
                pc = 0x80165b78;
            }
            0x80165b78 => {
                c.cmps(0, c.r[3], 0x4);
                if !c.crbit(0) { pc = 0x80165ba0; continue; }
                pc = 0x80165b80;
            }
            0x80165b80 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x32;
                c.r[5] = 0x0;
                c.lr = 0x80165b90; f_80166670(c);
                pc = 0x80165b90;
            }
            0x80165b90 => {
                c.r[3] = 0x32;
                c.lr = 0x80165b98; f_80165d0c(c);
                pc = 0x80165b98;
            }
            0x80165b98 => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x80165ba0; f_80166b18(c);
                pc = 0x80165ba0;
            }
            0x80165ba0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8614));
                c.r[0] = c.r[25].rotate_left(3) & 0xfffffff8;
                c.r[24] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.cmpu(0, c.r[24], 0x0);
                if c.crbit(2) { pc = 0x80165bd4; continue; }
                pc = 0x80165bb4;
            }
            0x80165bb4 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.lr = 0x80165bc0; f_800f9318(c);
                pc = 0x80165bc0;
            }
            0x80165bc0 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[24] | c.r[24];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80165bd0; f_80165ee0(c);
                pc = 0x80165bd0;
            }
            0x80165bd0 => {
                pc = 0x80165c20; continue;
                pc = 0x80165bd4;
            }
            0x80165bd4 => {
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[25] = c.lwz(c.r[3].wrapping_add(0x4));
                c.cmpu(0, c.r[25], 0x0);
                if c.crbit(2) { pc = 0x80165c04; continue; }
                pc = 0x80165be4;
            }
            0x80165be4 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.lr = 0x80165bf0; f_800f9318(c);
                pc = 0x80165bf0;
            }
            0x80165bf0 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80165c00; f_80165dec(c);
                pc = 0x80165c00;
            }
            0x80165c00 => {
                pc = 0x80165c20; continue;
                pc = 0x80165c04;
            }
            0x80165c04 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[3].wrapping_add(0x35f8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80165c18; stubs_gen::f_800dd970(c);
                pc = 0x80165c18;
            }
            0x80165c18 => {
                c.r[3] = 0x0;
                pc = 0x80165c5c; continue;
                pc = 0x80165c20;
            }
            0x80165c20 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x80165c30; f_80166b3c(c);
                pc = 0x80165c30;
            }
            0x80165c30 => {
                c.cmps(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x80165c3c; continue; }
                pc = 0x80165c38;
            }
            0x80165c38 => {
                c.stw(c.r[13].wrapping_add(0xffffa88c), c.r[26]);
                pc = 0x80165c3c;
            }
            0x80165c3c => {
                c.cmps(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x80165c48; continue; }
                pc = 0x80165c44;
            }
            0x80165c44 => {
                c.stw(c.r[13].wrapping_add(0xffffa888), c.r[26]);
                pc = 0x80165c48;
            }
            0x80165c48 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = 0x0;
                c.lr = 0x80165c5c; f_80166a50(c);
                pc = 0x80165c5c;
            }
            0x80165c5c => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80165a44: bad pc {:#x}", pc),
        }
    }
}

/// soundFadeSlot (0x80165c70, 156 bytes)
#[inline(never)]
pub fn f_80165c70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165c70) { return; }
        if c.prof { crate::prof::enter(c, 0x80165c70); f_80165c70_b(c, 0x80165c70); crate::prof::exit(c, 0x80165c70); return; }
    }
    f_80165c70_b(c, 0x80165c70)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165c70_at(c: &mut Cpu, pc: u32) { f_80165c70_b(c, pc) }
#[inline(always)]
fn f_80165c70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165c70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165c70, pc); }
        match pc {
            0x80165c70 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].wrapping_neg();
                c.r[0] = c.r[0] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[4] = (v >> 31) as u32; }
                c.r[4] = c.r[4].wrapping_add(0x407);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x80165ca8; f_801662e8(c);
                pc = 0x80165ca8;
            }
            0x80165ca8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.r[31].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x80165cf0; continue; }
                pc = 0x80165cb8;
            }
            0x80165cb8 => {
                c.lr = 0x80165cbc; f_801666bc(c);
                pc = 0x80165cbc;
            }
            0x80165cbc => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80165cf0; continue; }
                pc = 0x80165cc4;
            }
            0x80165cc4 => {
                if c.crbit(0) { pc = 0x80165cf0; continue; }
                pc = 0x80165cc8;
            }
            0x80165cc8 => {
                c.cmps(0, c.r[3], 0x4);
                if !c.crbit(0) { pc = 0x80165cf0; continue; }
                pc = 0x80165cd0;
            }
            0x80165cd0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.lr = 0x80165ce0; f_80166670(c);
                pc = 0x80165ce0;
            }
            0x80165ce0 => {
                c.cmps(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x80165cf0; continue; }
                pc = 0x80165ce8;
            }
            0x80165ce8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80165cf0; f_80165d0c(c);
                pc = 0x80165cf0;
            }
            0x80165cf0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80165c70: bad pc {:#x}", pc),
        }
    }
}

/// soundWaitMs (0x80165d0c, 224 bytes)
#[inline(never)]
pub fn f_80165d0c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165d0c) { return; }
        if c.prof { crate::prof::enter(c, 0x80165d0c); f_80165d0c_b(c, 0x80165d0c); crate::prof::exit(c, 0x80165d0c); return; }
    }
    f_80165d0c_b(c, 0x80165d0c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165d0c_at(c: &mut Cpu, pc: u32) { f_80165d0c_b(c, pc) }
#[inline(always)]
fn f_80165d0c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165d0c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165d0c, pc); }
        match pc {
            0x80165d0c => {
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = (c.f[29], c.ps1[29]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[28]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = (c.f[28], c.ps1[28]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[27]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = (c.f[27], c.ps1[27]); c.psq_store(ea, x, y, false, 0); }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff9ea8); c.f[29] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[31]);
                { let ea = c.r[2].wrapping_add(0xffff9ea4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfd(ea); }
                { let ea = c.r[2].wrapping_add(0xffff9ea0); c.f[27] = c.lfs(ea); c.ps1[27] = c.f[27]; }
                { let v = s(c.f[1] - c.f[29]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9eb0); c.f[30] = c.lfd(ea); }
                { let v = s(c.f[1] / c.f[0]); c.f[28] = v; c.ps1[28] = v; }
                pc = 0x80165da8; continue;
                pc = 0x80165d70;
            }
            0x80165d70 => {
                c.lr = 0x80165d74; crate::stubs::f_800f0308(c);
                pc = 0x80165d74;
            }
            0x80165d74 => {
                c.lr = 0x80165d78; f_800d37cc(c);
                pc = 0x80165d78;
            }
            0x80165d78 => {
                c.r[0] = c.r[3] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[31]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[30]); c.f[31] = v; c.ps1[31] = v; }
                c.lr = 0x80165d90; f_800d3088(c);
                pc = 0x80165d90;
            }
            0x80165d90 => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[31]);
                { let ea = c.r[1].wrapping_add(0x10); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[29]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] / c.f[31]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[27] + c.f[0]); c.f[27] = v; c.ps1[27] = v; }
                pc = 0x80165da8;
            }
            0x80165da8 => {
                c.fcmp(0, c.f[27], c.f[28]);
                if c.crbit(0) { pc = 0x80165d70; continue; }
                pc = 0x80165db0;
            }
            0x80165db0 => {
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x60); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x50); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                { let ea = c.r[1].wrapping_add(0x40); c.f[29] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = c.psq_load(ea, false, 0); c.f[28] = x; c.ps1[28] = y; }
                { let ea = c.r[1].wrapping_add(0x30); c.f[28] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[27] = x; c.ps1[27] = y; }
                { let ea = c.r[1].wrapping_add(0x20); c.f[27] = c.lfd(ea); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x74));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { return; }
                return;
            }
            _ => panic!("f_80165d0c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80165DEC (0x80165dec, 244 bytes)
#[inline(never)]
pub fn f_80165dec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165dec) { return; }
        if c.prof { crate::prof::enter(c, 0x80165dec); f_80165dec_b(c, 0x80165dec); crate::prof::exit(c, 0x80165dec); return; }
    }
    f_80165dec_b(c, 0x80165dec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165dec_at(c: &mut Cpu, pc: u32) { f_80165dec_b(c, pc) }
#[inline(always)]
fn f_80165dec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165dec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165dec, pc); }
        match pc {
            0x80165dec => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[31] = c.r[6].wrapping_add(0x3548);
                c.lr = 0x80165e14; f_80167f28(c);
                pc = 0x80165e14;
            }
            0x80165e14 => {
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if !c.crbit(2) { pc = 0x80165e30; continue; }
                pc = 0x80165e1c;
            }
            0x80165e1c => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[31].wrapping_add(0xe0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80165e2c; stubs_gen::f_800dd970(c);
                pc = 0x80165e2c;
            }
            0x80165e2c => {
                pc = 0x80165ecc; continue;
                pc = 0x80165e30;
            }
            0x80165e30 => {
                c.lr = 0x80165e34; f_80167e5c(c);
                pc = 0x80165e34;
            }
            0x80165e34 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80165e50; continue; }
                pc = 0x80165e3c;
            }
            0x80165e3c => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[31].wrapping_add(0xfc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80165e4c; stubs_gen::f_800dd970(c);
                pc = 0x80165e4c;
            }
            0x80165e4c => {
                pc = 0x80165ec4; continue;
                pc = 0x80165e50;
            }
            0x80165e50 => {
                c.r[0] = c.r[3].wrapping_add(0x1f);
                c.r[30] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.cmpu(0, c.r[30], c.r[28]);
                if !c.crbit(0) { pc = 0x80165eb8; continue; }
                pc = 0x80165e60;
            }
            0x80165e60 => {
                c.cmpu(0, c.r[27], 0x0);
                if !c.crbit(2) { pc = 0x80165e78; continue; }
                pc = 0x80165e68;
            }
            0x80165e68 => {
                c.r[3] = c.r[31].wrapping_add(0x11c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80165e74; stubs_gen::f_800dd970(c);
                pc = 0x80165e74;
            }
            0x80165e74 => {
                pc = 0x80165ec4; continue;
                pc = 0x80165e78;
            }
            0x80165e78 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = 0xe0;
                c.lr = 0x80165e88; f_8000540c(c);
                pc = 0x80165e88;
            }
            0x80165e88 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = 0x0;
                c.lr = 0x80165e9c; f_80167ed0(c);
                pc = 0x80165e9c;
            }
            0x80165e9c => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(1) { pc = 0x80165ec4; continue; }
                pc = 0x80165ea4;
            }
            0x80165ea4 => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[31].wrapping_add(0x140);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80165eb4; stubs_gen::f_800dd970(c);
                pc = 0x80165eb4;
            }
            0x80165eb4 => {
                pc = 0x80165ec4; continue;
                pc = 0x80165eb8;
            }
            0x80165eb8 => {
                c.r[3] = c.r[31].wrapping_add(0x168);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80165ec4; stubs_gen::f_800dd970(c);
                pc = 0x80165ec4;
            }
            0x80165ec4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80165ecc; f_80167e64(c);
                pc = 0x80165ecc;
            }
            0x80165ecc => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80165dec: bad pc {:#x}", pc),
        }
    }
}

/// fn_80165EE0 (0x80165ee0, 96 bytes)
#[inline(never)]
pub fn f_80165ee0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165ee0) { return; }
        if c.prof { crate::prof::enter(c, 0x80165ee0); f_80165ee0_b(c, 0x80165ee0); crate::prof::exit(c, 0x80165ee0); return; }
    }
    f_80165ee0_b(c, 0x80165ee0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165ee0_at(c: &mut Cpu, pc: u32) { f_80165ee0_b(c, pc) }
#[inline(always)]
fn f_80165ee0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165ee0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165ee0, pc); }
        match pc {
            0x80165ee0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.r[4] = 0xe0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80165f10; f_8000540c(c);
                pc = 0x80165f10;
            }
            0x80165f10 => {
                c.stw(c.r[13].wrapping_add(0xffffa890), c.r[31]);
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x99;
                c.stw(c.r[13].wrapping_add(0xffffa894), c.r[30]);
                c.lr = 0x80165f24; f_8017af6c(c);
                pc = 0x80165f24;
            }
            0x80165f24 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80165ee0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80165F40 (0x80165f40, 68 bytes)
#[inline(never)]
pub fn f_80165f40(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165f40) { return; }
        if c.prof { crate::prof::enter(c, 0x80165f40); f_80165f40_b(c, 0x80165f40); crate::prof::exit(c, 0x80165f40); return; }
    }
    f_80165f40_b(c, 0x80165f40)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165f40_at(c: &mut Cpu, pc: u32) { f_80165f40_b(c, pc) }
#[inline(always)]
fn f_80165f40_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165f40;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165f40, pc); }
        match pc {
            0x80165f40 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x406;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80165f58; f_800f9318(c);
                pc = 0x80165f58;
            }
            0x80165f58 => {
                c.r[0] = 0x10000;
                c.stw(c.r[13].wrapping_add(0xffffa894), c.r[3]);
                c.r[3] = 0x99;
                c.stw(c.r[13].wrapping_add(0xffffa890), c.r[0]);
                c.lr = 0x80165f6c; f_8017b370(c);
                pc = 0x80165f6c;
            }
            0x80165f6c => {
                c.r[3] = 0x99;
                c.lr = 0x80165f74; f_80165fdc(c);
                pc = 0x80165f74;
            }
            0x80165f74 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80165f40: bad pc {:#x}", pc),
        }
    }
}

/// fn_80165F84 (0x80165f84, 88 bytes)
#[inline(never)]
pub fn f_80165f84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165f84) { return; }
        if c.prof { crate::prof::enter(c, 0x80165f84); f_80165f84_b(c, 0x80165f84); crate::prof::exit(c, 0x80165f84); return; }
    }
    f_80165f84_b(c, 0x80165f84)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165f84_at(c: &mut Cpu, pc: u32) { f_80165f84_b(c, pc) }
#[inline(always)]
fn f_80165f84_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165f84;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165f84, pc); }
        match pc {
            0x80165f84 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x10000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80165f98; f_8016604c(c);
                pc = 0x80165f98;
            }
            0x80165f98 => {
                c.r[4] = 0x0;
                c.r[5] = 0x406;
                c.r[6] = 0x0;
                c.lr = 0x80165fa8; f_800f9378(c);
                pc = 0x80165fa8;
            }
            0x80165fa8 => {
                c.r[3] = 0x2000;
                c.lr = 0x80165fb0; f_8016604c(c);
                pc = 0x80165fb0;
            }
            0x80165fb0 => {
                c.r[4] = 0x0;
                c.r[5] = 0x407;
                c.r[6] = 0x0;
                c.lr = 0x80165fc0; f_800f9378(c);
                pc = 0x80165fc0;
            }
            0x80165fc0 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa88c), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa888), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80165f84: bad pc {:#x}", pc),
        }
    }
}

/// fn_80165FDC (0x80165fdc, 112 bytes)
#[inline(never)]
pub fn f_80165fdc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80165fdc) { return; }
        if c.prof { crate::prof::enter(c, 0x80165fdc); f_80165fdc_b(c, 0x80165fdc); crate::prof::exit(c, 0x80165fdc); return; }
    }
    f_80165fdc_b(c, 0x80165fdc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165fdc_at(c: &mut Cpu, pc: u32) { f_80165fdc_b(c, pc) }
#[inline(always)]
fn f_80165fdc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80165fdc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80165fdc, pc); }
        match pc {
            0x80165fdc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = 0x80270000;
                c.r[31] = c.r[3].wrapping_add(0x36cc);
                pc = 0x80166000;
            }
            0x80166000 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80166008; f_8017b2cc(c);
                pc = 0x80166008;
            }
            0x80166008 => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if !c.crbit(0) { pc = 0x80166020; continue; }
                pc = 0x80166010;
            }
            0x80166010 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[29] | c.r[29];
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80166020; stubs_gen::f_800dd970(c);
                pc = 0x80166020;
            }
            0x80166020 => {
                c.cmps(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x80166030; continue; }
                pc = 0x80166028;
            }
            0x80166028 => {
                c.lr = 0x8016602c; crate::stubs::f_800f0308(c);
                pc = 0x8016602c;
            }
            0x8016602c => {
                pc = 0x80166000; continue;
                pc = 0x80166030;
            }
            0x80166030 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80165fdc: bad pc {:#x}", pc),
        }
    }
}

/// fn_8016604C (0x8016604c, 56 bytes)
#[inline(never)]
pub fn f_8016604c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016604c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016604c); f_8016604c_b(c, 0x8016604c); crate::prof::exit(c, 0x8016604c); return; }
    }
    f_8016604c_b(c, 0x8016604c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016604c_at(c: &mut Cpu, pc: u32) { f_8016604c_b(c, pc) }
#[inline(always)]
fn f_8016604c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016604c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016604c, pc); }
        match pc {
            0x8016604c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x20;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80166060; f_800e2c04(c);
                pc = 0x80166060;
            }
            0x80166060 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80166070; continue; }
                pc = 0x80166068;
            }
            0x80166068 => {
                c.lr = 0x8016606c; f_800e27b0(c);
                pc = 0x8016606c;
            }
            0x8016606c => {
                pc = 0x80166074; continue;
                pc = 0x80166070;
            }
            0x80166070 => {
                c.r[3] = 0x0;
                pc = 0x80166074;
            }
            0x80166074 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8016604c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166084 (0x80166084, 20 bytes)
#[inline(never)]
pub fn f_80166084(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166084) { return; }
        if c.prof { crate::prof::enter(c, 0x80166084); f_80166084_b(c, 0x80166084); crate::prof::exit(c, 0x80166084); return; }
    }
    f_80166084_b(c, 0x80166084)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166084_at(c: &mut Cpu, pc: u32) { f_80166084_b(c, pc) }
#[inline(always)]
fn f_80166084_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166084;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166084, pc); }
        match pc {
            0x80166084 => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[0] = c.lbz(c.r[3].wrapping_add(c.r[0]));
                c.r[3] = c.r[0].rotate_left(25) & 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_80166084: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166098 (0x80166098, 64 bytes)
#[inline(never)]
pub fn f_80166098(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166098) { return; }
        if c.prof { crate::prof::enter(c, 0x80166098); f_80166098_b(c, 0x80166098); crate::prof::exit(c, 0x80166098); return; }
    }
    f_80166098_b(c, 0x80166098)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166098_at(c: &mut Cpu, pc: u32) { f_80166098_b(c, pc) }
#[inline(always)]
fn f_80166098_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166098;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166098, pc); }
        match pc {
            0x80166098 => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz(c.r[3]);
                c.r[0] = c.r[0].rotate_left(27) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x801660bc; continue; }
                pc = 0x801660b4;
            }
            0x801660b4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x801660bc;
            }
            0x801660bc => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801660d0; continue; }
                pc = 0x801660c8;
            }
            0x801660c8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                if true { return; }
                pc = 0x801660d0;
            }
            0x801660d0 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80166098: bad pc {:#x}", pc),
        }
    }
}

/// soundUnduck (0x801660d8, 144 bytes)
#[inline(never)]
pub fn f_801660d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801660d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801660d8); f_801660d8_b(c, 0x801660d8); crate::prof::exit(c, 0x801660d8); return; }
    }
    f_801660d8_b(c, 0x801660d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801660d8_at(c: &mut Cpu, pc: u32) { f_801660d8_b(c, pc) }
#[inline(always)]
fn f_801660d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801660d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801660d8, pc); }
        match pc {
            0x801660d8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = 0x0;
                c.r[31] = 0x0;
                pc = 0x80166144; continue;
                pc = 0x80166100;
            }
            0x80166100 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[4] = c.lbz(c.r[4].wrapping_add(c.r[31]));
                c.r[0] = c.r[4].rotate_left(27) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8016613c; continue; }
                pc = 0x80166114;
            }
            0x80166114 => {
                c.r[0] = c.r[4].rotate_left(25) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8016612c; continue; }
                pc = 0x80166120;
            }
            0x80166120 => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x8016613c; continue; }
                pc = 0x80166128;
            }
            0x80166128 => {
                pc = 0x80166134; continue;
                pc = 0x8016612c;
            }
            0x8016612c => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x8016613c; continue; }
                pc = 0x80166134;
            }
            0x80166134 => {
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8016613c; f_80166168(c);
                pc = 0x8016613c;
            }
            0x8016613c => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x80166144;
            }
            0x80166144 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa8c8));
                c.r[3] = c.r[30] | c.r[30];
                c.cmpu(0, c.r[30], c.r[0]);
                if c.crbit(0) { pc = 0x80166100; continue; }
                pc = 0x80166154;
            }
            0x80166154 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801660d8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166168 (0x80166168, 104 bytes)
#[inline(never)]
pub fn f_80166168(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166168) { return; }
        if c.prof { crate::prof::enter(c, 0x80166168); f_80166168_b(c, 0x80166168); crate::prof::exit(c, 0x80166168); return; }
    }
    f_80166168_b(c, 0x80166168)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166168_at(c: &mut Cpu, pc: u32) { f_80166168_b(c, pc) }
#[inline(always)]
fn f_80166168_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166168;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166168, pc); }
        match pc {
            0x80166168 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = c.r[3].wrapping_mul(0xc);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[0] = c.r[5].wrapping_add(0x8);
                c.r[6] = c.lwz(c.r[6].wrapping_add(c.r[0]));
                c.cmpu(0, c.r[6], 0x0);
                if !c.crbit(2) { pc = 0x80166194; continue; }
                pc = 0x8016618c;
            }
            0x8016618c => {
                c.r[3] = 0x0;
                pc = 0x801661c0; continue;
                pc = 0x80166194;
            }
            0x80166194 => {
                c.r[5] = c.lbz(c.r[6].wrapping_add(0x4));
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x801661a8; continue; }
                pc = 0x801661a0;
            }
            0x801661a0 => {
                c.r[3] = 0x0;
                pc = 0x801661c0; continue;
                pc = 0x801661a8;
            }
            0x801661a8 => {
                c.r[0] = c.r[5].wrapping_add(0xffffffff);
                c.stb(c.r[6].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x4));
                c.r[5] = c.r[6].wrapping_add(c.r[0]);
                c.r[5] = c.lbz(c.r[5].wrapping_add(0x5));
                c.lr = 0x801661c0; f_80166670(c);
                pc = 0x801661c0;
            }
            0x801661c0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80166168: bad pc {:#x}", pc),
        }
    }
}

/// soundDuck (0x801661d0, 152 bytes)
#[inline(never)]
pub fn f_801661d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801661d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801661d0); f_801661d0_b(c, 0x801661d0); crate::prof::exit(c, 0x801661d0); return; }
    }
    f_801661d0_b(c, 0x801661d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801661d0_at(c: &mut Cpu, pc: u32) { f_801661d0_b(c, pc) }
#[inline(always)]
fn f_801661d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801661d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801661d0, pc); }
        match pc {
            0x801661d0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[29] = c.r[6] | c.r[6];
                c.r[30] = 0x0;
                c.r[31] = 0x0;
                pc = 0x80166244; continue;
                pc = 0x801661fc;
            }
            0x801661fc => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[4] = c.lbz(c.r[4].wrapping_add(c.r[31]));
                c.r[0] = c.r[4].rotate_left(27) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8016623c; continue; }
                pc = 0x80166210;
            }
            0x80166210 => {
                c.r[0] = c.r[4].rotate_left(25) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166228; continue; }
                pc = 0x8016621c;
            }
            0x8016621c => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x8016623c; continue; }
                pc = 0x80166224;
            }
            0x80166224 => {
                pc = 0x80166230; continue;
                pc = 0x80166228;
            }
            0x80166228 => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x8016623c; continue; }
                pc = 0x80166230;
            }
            0x80166230 => {
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[26] | c.r[26];
                c.lr = 0x8016623c; f_80166268(c);
                pc = 0x8016623c;
            }
            0x8016623c => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x80166244;
            }
            0x80166244 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa8c8));
                c.r[3] = c.r[30] | c.r[30];
                c.cmpu(0, c.r[30], c.r[0]);
                if c.crbit(0) { pc = 0x801661fc; continue; }
                pc = 0x80166254;
            }
            0x80166254 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801661d0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166268 (0x80166268, 128 bytes)
#[inline(never)]
pub fn f_80166268(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166268) { return; }
        if c.prof { crate::prof::enter(c, 0x80166268); f_80166268_b(c, 0x80166268); crate::prof::exit(c, 0x80166268); return; }
    }
    f_80166268_b(c, 0x80166268)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166268_at(c: &mut Cpu, pc: u32) { f_80166268_b(c, pc) }
#[inline(always)]
fn f_80166268_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166268;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166268, pc); }
        match pc {
            0x80166268 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = c.r[3].wrapping_mul(0xc);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[0] = c.r[6].wrapping_add(0x8);
                c.r[7] = c.lwz(c.r[7].wrapping_add(c.r[0]));
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(2) { pc = 0x80166294; continue; }
                pc = 0x8016628c;
            }
            0x8016628c => {
                c.r[3] = 0x0;
                pc = 0x801662d8; continue;
                pc = 0x80166294;
            }
            0x80166294 => {
                c.r[6] = c.lbz(c.r[7].wrapping_add(0x4));
                c.cmpu(0, c.r[6], 0x3);
                if c.crbit(0) { pc = 0x801662a8; continue; }
                pc = 0x801662a0;
            }
            0x801662a0 => {
                c.r[3] = 0x0;
                pc = 0x801662d8; continue;
                pc = 0x801662a8;
            }
            0x801662a8 => {
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x1));
                c.cmps(0, c.r[0], c.r[5]);
                if !c.crbit(0) { pc = 0x801662b8; continue; }
                pc = 0x801662b4;
            }
            0x801662b4 => {
                c.r[5] = c.r[0] | c.r[0];
                pc = 0x801662b8;
            }
            0x801662b8 => {
                c.r[6] = c.r[7].wrapping_add(c.r[6]);
                c.stb(c.r[6].wrapping_add(0x5), c.r[0]);
                c.r[6] = c.lbz(c.r[7].wrapping_add(0x4));
                c.cmpu(0, c.r[6], 0x2);
                if !c.crbit(0) { pc = 0x801662d4; continue; }
                pc = 0x801662cc;
            }
            0x801662cc => {
                c.r[0] = c.r[6].wrapping_add(0x1);
                c.stb(c.r[7].wrapping_add(0x4), c.r[0]);
                pc = 0x801662d4;
            }
            0x801662d4 => {
                c.lr = 0x801662d8; f_80166670(c);
                pc = 0x801662d8;
            }
            0x801662d8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80166268: bad pc {:#x}", pc),
        }
    }
}

/// fn_801662E8 (0x801662e8, 32 bytes)
#[inline(never)]
pub fn f_801662e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801662e8) { return; }
        if c.prof { crate::prof::enter(c, 0x801662e8); f_801662e8_b(c, 0x801662e8); crate::prof::exit(c, 0x801662e8); return; }
    }
    f_801662e8_b(c, 0x801662e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801662e8_at(c: &mut Cpu, pc: u32) { f_801662e8_b(c, pc) }
#[inline(always)]
fn f_801662e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801662e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801662e8, pc); }
        match pc {
            0x801662e8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801662f8; f_80167768(c);
                pc = 0x801662f8;
            }
            0x801662f8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801662e8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166308 (0x80166308, 104 bytes)
#[inline(never)]
pub fn f_80166308(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166308) { return; }
        if c.prof { crate::prof::enter(c, 0x80166308); f_80166308_b(c, 0x80166308); crate::prof::exit(c, 0x80166308); return; }
    }
    f_80166308_b(c, 0x80166308)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166308_at(c: &mut Cpu, pc: u32) { f_80166308_b(c, pc) }
#[inline(always)]
fn f_80166308_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166308;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166308, pc); }
        match pc {
            0x80166308 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffff9ec0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.f[2] = c.f[1];
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.f[3] = c.f[1];
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80166338; f_800e01f4(c);
                pc = 0x80166338;
            }
            0x80166338 => {
                { let ea = c.r[2].wrapping_add(0xffff9ec4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[30] | c.r[30];
                { let ea = c.r[2].wrapping_add(0xffff9ec0); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[6] = 0x7f;
                c.r[7] = 0x0;
                c.lr = 0x80166358; f_80166370(c);
                pc = 0x80166358;
            }
            0x80166358 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80166308: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166370 (0x80166370, 232 bytes)
#[inline(never)]
pub fn f_80166370(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166370) { return; }
        if c.prof { crate::prof::enter(c, 0x80166370); f_80166370_b(c, 0x80166370); crate::prof::exit(c, 0x80166370); return; }
    }
    f_80166370_b(c, 0x80166370)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166370_at(c: &mut Cpu, pc: u32) { f_80166370_b(c, pc) }
#[inline(always)]
fn f_80166370_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166370;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166370, pc); }
        match pc {
            0x80166370 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[31]; c.stfd(ea, v); }
                c.f[31] = c.f[2];
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[30]; c.stfd(ea, v); }
                c.f[30] = c.f[1];
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[3] | c.r[3];
                c.r[0] = c.r[25].wrapping_mul(0xc);
                c.r[26] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[28] = c.r[6] | c.r[6];
                c.r[29] = c.r[7] | c.r[7];
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[31] = c.r[3].wrapping_add(c.r[0]);
                c.lr = 0x801663b4; f_80167864(c);
                pc = 0x801663b4;
            }
            0x801663b4 => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x80166438; continue; }
                pc = 0x801663bc;
            }
            0x801663bc => {
                c.f[1] = c.f[30];
                c.r[7] = c.lhz(c.r[31].wrapping_add(0x4));
                c.f[2] = c.f[31];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.r[8] = c.r[28] | c.r[28];
                c.r[9] = c.r[29] | c.r[29];
                c.r[3] = c.r[30].wrapping_add(0x28);
                c.r[6] = 0x1f;
                c.r[10] = 0x0;
                c.lr = 0x801663e8; f_8015eca8(c);
                pc = 0x801663e8;
            }
            0x801663e8 => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[30].wrapping_add(0x10);
                c.lr = 0x801663f4; f_800e01d0(c);
                pc = 0x801663f4;
            }
            0x801663f4 => {
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = c.r[30].wrapping_add(0x1c);
                c.lr = 0x80166400; f_800e01d0(c);
                pc = 0x80166400;
            }
            0x80166400 => {
                c.stb(c.r[30].wrapping_add(0x1), c.r[28]);
                c.r[3] = c.r[30].wrapping_add(0x28);
                c.stb(c.r[30].wrapping_add(0x2), c.r[29]);
                { let ea = c.r[30].wrapping_add(0x8); let v = c.f[30]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0xc); let v = c.f[31]; c.stfs(ea, v); }
                c.stw(c.r[30].wrapping_add(0x4), c.r[25]);
                c.lr = 0x8016641c; f_8015e890(c);
                pc = 0x8016641c;
            }
            0x8016641c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166430; continue; }
                pc = 0x80166428;
            }
            0x80166428 => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x8016643c; continue;
                pc = 0x80166430;
            }
            0x80166430 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80166438; f_801677bc(c);
                pc = 0x80166438;
            }
            0x80166438 => {
                c.r[3] = 0x0;
                pc = 0x8016643c;
            }
            0x8016643c => {
                { let ea = c.r[1].wrapping_add(0x38); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x30); c.f[30] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_80166370: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166458 (0x80166458, 152 bytes)
#[inline(never)]
pub fn f_80166458(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166458) { return; }
        if c.prof { crate::prof::enter(c, 0x80166458); f_80166458_b(c, 0x80166458); crate::prof::exit(c, 0x80166458); return; }
    }
    f_80166458_b(c, 0x80166458)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166458_at(c: &mut Cpu, pc: u32) { f_80166458_b(c, pc) }
#[inline(always)]
fn f_80166458_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166458;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166458, pc); }
        match pc {
            0x80166458 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x801664d4; continue; }
                pc = 0x80166478;
            }
            0x80166478 => {
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[30].wrapping_add(0x10);
                c.lr = 0x80166488; f_800e0168(c);
                pc = 0x80166488;
            }
            0x80166488 => {
                c.r[8] = c.lbz(c.r[30].wrapping_add(0x1));
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[30].wrapping_add(0x40);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[6] = c.r[30].wrapping_add(0x28);
                c.r[7] = c.r[30].wrapping_add(0x34);
                c.r[9] = 0x0;
                c.lr = 0x801664a8; f_8015ed00(c);
                pc = 0x801664a8;
            }
            0x801664a8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x801664d4; continue; }
                pc = 0x801664b4;
            }
            0x801664b4 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[30].wrapping_add(0x10);
                c.lr = 0x801664c0; f_800e01d0(c);
                pc = 0x801664c0;
            }
            0x801664c0 => {
                c.r[3] = c.r[30].wrapping_add(0x1c);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x801664cc; f_800e01d0(c);
                pc = 0x801664cc;
            }
            0x801664cc => {
                c.r[3] = 0x1;
                pc = 0x801664d8; continue;
                pc = 0x801664d4;
            }
            0x801664d4 => {
                c.r[3] = 0x0;
                pc = 0x801664d8;
            }
            0x801664d8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80166458: bad pc {:#x}", pc),
        }
    }
}

/// fn_801664F0 (0x801664f0, 136 bytes)
#[inline(never)]
pub fn f_801664f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801664f0) { return; }
        if c.prof { crate::prof::enter(c, 0x801664f0); f_801664f0_b(c, 0x801664f0); crate::prof::exit(c, 0x801664f0); return; }
    }
    f_801664f0_b(c, 0x801664f0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801664f0_at(c: &mut Cpu, pc: u32) { f_801664f0_b(c, pc) }
#[inline(always)]
fn f_801664f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801664f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801664f0, pc); }
        match pc {
            0x801664f0 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffff9ec0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.f[2] = c.f[1];
                c.stw(c.r[1].wrapping_add(0x3c), c.r[31]);
                c.f[3] = c.f[1];
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0x20);
                c.lr = 0x80166518; f_800e01f4(c);
                pc = 0x80166518;
            }
            0x80166518 => {
                { let ea = c.r[2].wrapping_add(0xffff9ec0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x14);
                { let ea = c.r[2].wrapping_add(0xffff9ec8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.f[2] = c.f[1];
                c.lr = 0x8016652c; f_800e01f4(c);
                pc = 0x8016652c;
            }
            0x8016652c => {
                { let ea = c.r[2].wrapping_add(0xffff9ec0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x8);
                { let ea = c.r[2].wrapping_add(0xffff9ecc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.f[3] = c.f[1];
                c.lr = 0x80166540; f_800e01f4(c);
                pc = 0x80166540;
            }
            0x80166540 => {
                { let ea = c.r[2].wrapping_add(0xffff9ed0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[31] | c.r[31];
                { let ea = c.r[2].wrapping_add(0xffff9ed4); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.r[4] = c.r[1].wrapping_add(0x20);
                c.f[2] = c.f[1];
                c.r[5] = c.r[1].wrapping_add(0x14);
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[7] = 0x7f;
                c.lr = 0x80166564; f_80166578(c);
                pc = 0x80166564;
            }
            0x80166564 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_801664f0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166578 (0x80166578, 248 bytes)
#[inline(never)]
pub fn f_80166578(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166578) { return; }
        if c.prof { crate::prof::enter(c, 0x80166578); f_80166578_b(c, 0x80166578); crate::prof::exit(c, 0x80166578); return; }
    }
    f_80166578_b(c, 0x80166578)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166578_at(c: &mut Cpu, pc: u32) { f_80166578_b(c, pc) }
#[inline(always)]
fn f_80166578_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166578;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166578, pc); }
        match pc {
            0x80166578 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[31]; c.stfd(ea, v); }
                c.f[31] = c.f[3];
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[30]; c.stfd(ea, v); }
                c.f[30] = c.f[2];
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[29]; c.stfd(ea, v); }
                c.f[29] = c.f[1];
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[29] = c.r[6] | c.r[6];
                c.r[30] = c.r[7] | c.r[7];
                c.lr = 0x801665b8; f_801678e4(c);
                pc = 0x801665b8;
            }
            0x801665b8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x8016664c; continue; }
                pc = 0x801665c0;
            }
            0x801665c0 => {
                c.f[1] = c.f[29];
                c.r[4] = c.r[26] | c.r[26];
                c.f[2] = c.f[30];
                c.r[5] = c.r[27] | c.r[27];
                c.f[3] = c.f[31];
                c.r[6] = c.r[28] | c.r[28];
                c.r[7] = c.r[29] | c.r[29];
                c.r[9] = c.r[30] | c.r[30];
                c.r[3] = c.r[31].wrapping_add(0x40);
                c.r[8] = 0x1;
                c.r[10] = 0x0;
                c.lr = 0x801665f0; f_8015ef04(c);
                pc = 0x801665f0;
            }
            0x801665f0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166644; continue; }
                pc = 0x801665fc;
            }
            0x801665fc => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[31].wrapping_add(0x10);
                c.lr = 0x80166608; f_800e01d0(c);
                pc = 0x80166608;
            }
            0x80166608 => {
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = c.r[31].wrapping_add(0x1c);
                c.lr = 0x80166614; f_800e01d0(c);
                pc = 0x80166614;
            }
            0x80166614 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = c.r[31].wrapping_add(0x28);
                c.lr = 0x80166620; f_800e01d0(c);
                pc = 0x80166620;
            }
            0x80166620 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.r[31].wrapping_add(0x34);
                c.lr = 0x8016662c; f_800e01d0(c);
                pc = 0x8016662c;
            }
            0x8016662c => {
                { let ea = c.r[31].wrapping_add(0x4); let v = c.f[29]; c.stfs(ea, v); }
                c.r[3] = c.r[31] | c.r[31];
                { let ea = c.r[31].wrapping_add(0x8); let v = c.f[30]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0xc); let v = c.f[31]; c.stfs(ea, v); }
                c.stb(c.r[31].wrapping_add(0x1), c.r[30]);
                pc = 0x80166650; continue;
                pc = 0x80166644;
            }
            0x80166644 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8016664c; f_801677f4(c);
                pc = 0x8016664c;
            }
            0x8016664c => {
                c.r[3] = 0x0;
                pc = 0x80166650;
            }
            0x80166650 => {
                { let ea = c.r[1].wrapping_add(0x38); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x30); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x28); c.f[29] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_80166578: bad pc {:#x}", pc),
        }
    }
}

/// soundSetVolume (0x80166670, 76 bytes)
#[inline(never)]
pub fn f_80166670(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166670) { return; }
        if c.prof { crate::prof::enter(c, 0x80166670); f_80166670_b(c, 0x80166670); crate::prof::exit(c, 0x80166670); return; }
    }
    f_80166670_b(c, 0x80166670)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166670_at(c: &mut Cpu, pc: u32) { f_80166670_b(c, pc) }
#[inline(always)]
fn f_80166670_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166670;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166670, pc); }
        match pc {
            0x80166670 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[6] = c.lbz(c.r[3]);
                c.r[0] = c.r[6].rotate_left(25) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x801666a8; continue; }
                pc = 0x80166694;
            }
            0x80166694 => {
                c.r[0] = c.r[6].rotate_left(27) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x801666a8; continue; }
                pc = 0x801666a0;
            }
            0x801666a0 => {
                c.r[3] = 0x0;
                pc = 0x801666ac; continue;
                pc = 0x801666a8;
            }
            0x801666a8 => {
                c.lr = 0x801666ac; f_8016737c(c);
                pc = 0x801666ac;
            }
            0x801666ac => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80166670: bad pc {:#x}", pc),
        }
    }
}

/// soundGetStatus (0x801666bc, 284 bytes)
#[inline(never)]
pub fn f_801666bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801666bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801666bc); f_801666bc_b(c, 0x801666bc); crate::prof::exit(c, 0x801666bc); return; }
    }
    f_801666bc_b(c, 0x801666bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801666bc_at(c: &mut Cpu, pc: u32) { f_801666bc_b(c, pc) }
#[inline(always)]
fn f_801666bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801666bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801666bc, pc); }
        match pc {
            0x801666bc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.r[31].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[4] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz(c.r[4]);
                c.r[0] = c.r[3].rotate_left(27) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x801666f4; continue; }
                pc = 0x801666ec;
            }
            0x801666ec => {
                c.r[3] = 0x0;
                pc = 0x801667c4; continue;
                pc = 0x801666f4;
            }
            0x801666f4 => {
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x8));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x80166708; continue; }
                pc = 0x80166700;
            }
            0x80166700 => {
                c.r[3] = 0x0;
                pc = 0x801667c4; continue;
                pc = 0x80166708;
            }
            0x80166708 => {
                c.r[0] = c.r[3].rotate_left(25) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166764; continue; }
                pc = 0x80166714;
            }
            0x80166714 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x8016673c; continue; }
                pc = 0x80166724;
            }
            0x80166724 => {
                c.lr = 0x80166728; f_8014d880(c);
                pc = 0x80166728;
            }
            0x80166728 => {
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x8016673c; continue; }
                pc = 0x80166734;
            }
            0x80166734 => {
                c.r[3] = 0x2;
                pc = 0x801667c4; continue;
                pc = 0x8016673c;
            }
            0x8016673c => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[3].wrapping_add(0x36f0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80166750; stubs_gen::f_800dd970(c);
                pc = 0x80166750;
            }
            0x80166750 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.lr = 0x8016675c; f_80167070(c);
                pc = 0x8016675c;
            }
            0x8016675c => {
                c.r[3] = 0x0;
                pc = 0x801667c4; continue;
                pc = 0x80166764;
            }
            0x80166764 => {
                c.r[0] = c.r[3].rotate_left(28) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166778; continue; }
                pc = 0x80166770;
            }
            0x80166770 => {
                c.r[3] = 0x3;
                pc = 0x801667c4; continue;
                pc = 0x80166778;
            }
            0x80166778 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x801667a0; continue; }
                pc = 0x80166788;
            }
            0x80166788 => {
                c.lr = 0x8016678c; f_8014d598(c);
                pc = 0x8016678c;
            }
            0x8016678c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x801667a0; continue; }
                pc = 0x80166798;
            }
            0x80166798 => {
                c.r[3] = 0x2;
                pc = 0x801667c4; continue;
                pc = 0x801667a0;
            }
            0x801667a0 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[3].wrapping_add(0x371c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801667b4; stubs_gen::f_800dd970(c);
                pc = 0x801667b4;
            }
            0x801667b4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.lr = 0x801667c0; f_80167070(c);
                pc = 0x801667c0;
            }
            0x801667c0 => {
                c.r[3] = 0x0;
                pc = 0x801667c4;
            }
            0x801667c4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801666bc: bad pc {:#x}", pc),
        }
    }
}

/// soundResumeSong (0x801667d8, 260 bytes)
#[inline(never)]
pub fn f_801667d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801667d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801667d8); f_801667d8_b(c, 0x801667d8); crate::prof::exit(c, 0x801667d8); return; }
    }
    f_801667d8_b(c, 0x801667d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801667d8_at(c: &mut Cpu, pc: u32) { f_801667d8_b(c, pc) }
#[inline(always)]
fn f_801667d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801667d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801667d8, pc); }
        match pc {
            0x801667d8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.r[0] = c.r[28].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[31] = c.r[3].wrapping_add(c.r[0]);
                c.r[4] = c.lbz(c.r[31]);
                c.r[0] = c.r[4].rotate_left(25) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166820; continue; }
                pc = 0x80166818;
            }
            0x80166818 => {
                c.r[3] = 0x0;
                pc = 0x801668bc; continue;
                pc = 0x80166820;
            }
            0x80166820 => {
                c.r[0] = c.r[4].rotate_left(27) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80166834; continue; }
                pc = 0x8016682c;
            }
            0x8016682c => {
                c.r[3] = 0x0;
                pc = 0x801668bc; continue;
                pc = 0x80166834;
            }
            0x80166834 => {
                c.r[30] = c.lwz(c.r[31].wrapping_add(0x8));
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80166848; continue; }
                pc = 0x80166840;
            }
            0x80166840 => {
                c.r[3] = 0x0;
                pc = 0x801668bc; continue;
                pc = 0x80166848;
            }
            0x80166848 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if !c.crbit(2) { pc = 0x80166860; continue; }
                pc = 0x80166858;
            }
            0x80166858 => {
                c.r[3] = 0x0;
                pc = 0x801668bc; continue;
                pc = 0x80166860;
            }
            0x80166860 => {
                c.r[0] = c.r[4].rotate_left(28) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80166874; continue; }
                pc = 0x8016686c;
            }
            0x8016686c => {
                c.r[3] = 0x0;
                pc = 0x801668bc; continue;
                pc = 0x80166874;
            }
            0x80166874 => {
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x2));
                c.lr = 0x8016687c; f_80166c34(c);
                pc = 0x8016687c;
            }
            0x8016687c => {
                c.cmpu(0, c.r[28], 0x1);
                if !c.crbit(2) { pc = 0x8016688c; continue; }
                pc = 0x80166884;
            }
            0x80166884 => {
                c.r[3] = 0x1;
                pc = 0x801668bc; continue;
                pc = 0x8016688c;
            }
            0x8016688c => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x8));
                c.lr = 0x80166894; f_8014d648(c);
                pc = 0x80166894;
            }
            0x80166894 => {
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x1));
                c.r[4] = c.r[29].rotate_left(0) & 0xffff;
                c.r[5] = c.lwz(c.r[30].wrapping_add(0x8));
                c.r[6] = 0x0;
                c.lr = 0x801668a8; f_8014d6d8(c);
                pc = 0x801668a8;
            }
            0x801668a8 => {
                c.r[0] = c.lbz(c.r[31]);
                c.r[3] = 0x0;
                c.r[0] = (c.r[3].rotate_left(4) & 0x10) | (c.r[0] & 0xffffffef);
                c.r[3] = 0x1;
                c.stb(c.r[31], c.r[0]);
                pc = 0x801668bc;
            }
            0x801668bc => {
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
            _ => panic!("f_801667d8: bad pc {:#x}", pc),
        }
    }
}

/// soundPauseSong (0x801668dc, 224 bytes)
#[inline(never)]
pub fn f_801668dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801668dc) { return; }
        if c.prof { crate::prof::enter(c, 0x801668dc); f_801668dc_b(c, 0x801668dc); crate::prof::exit(c, 0x801668dc); return; }
    }
    f_801668dc_b(c, 0x801668dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801668dc_at(c: &mut Cpu, pc: u32) { f_801668dc_b(c, pc) }
#[inline(always)]
fn f_801668dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801668dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801668dc, pc); }
        match pc {
            0x801668dc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[31] = c.r[5].wrapping_add(c.r[0]);
                c.r[6] = c.lbz(c.r[31]);
                c.r[0] = c.r[6].rotate_left(25) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166914; continue; }
                pc = 0x8016690c;
            }
            0x8016690c => {
                c.r[3] = 0x0;
                pc = 0x801669a4; continue;
                pc = 0x80166914;
            }
            0x80166914 => {
                c.r[0] = c.r[6].rotate_left(27) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80166928; continue; }
                pc = 0x80166920;
            }
            0x80166920 => {
                c.r[3] = 0x0;
                pc = 0x801669a4; continue;
                pc = 0x80166928;
            }
            0x80166928 => {
                c.r[30] = c.lwz(c.r[31].wrapping_add(0x8));
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8016693c; continue; }
                pc = 0x80166934;
            }
            0x80166934 => {
                c.r[3] = 0x0;
                pc = 0x801669a4; continue;
                pc = 0x8016693c;
            }
            0x8016693c => {
                c.r[5] = c.lwz(c.r[30].wrapping_add(0x8));
                c.r[0] = c.r[5].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if !c.crbit(2) { pc = 0x80166954; continue; }
                pc = 0x8016694c;
            }
            0x8016694c => {
                c.r[3] = 0x0;
                pc = 0x801669a4; continue;
                pc = 0x80166954;
            }
            0x80166954 => {
                c.r[0] = c.r[6].rotate_left(28) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166968; continue; }
                pc = 0x80166960;
            }
            0x80166960 => {
                c.r[3] = 0x0;
                pc = 0x801669a4; continue;
                pc = 0x80166968;
            }
            0x80166968 => {
                c.cmpu(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x80166978; continue; }
                pc = 0x80166970;
            }
            0x80166970 => {
                c.r[3] = 0x1;
                pc = 0x801669a4; continue;
                pc = 0x80166978;
            }
            0x80166978 => {
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80166988; f_8014d6d8(c);
                pc = 0x80166988;
            }
            0x80166988 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x8));
                c.lr = 0x80166990; f_8014d5c8(c);
                pc = 0x80166990;
            }
            0x80166990 => {
                c.r[0] = c.lbz(c.r[31]);
                c.r[3] = 0x1;
                c.r[0] = (c.r[3].rotate_left(4) & 0x10) | (c.r[0] & 0xffffffef);
                c.r[3] = 0x1;
                c.stb(c.r[31], c.r[0]);
                pc = 0x801669a4;
            }
            0x801669a4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801668dc: bad pc {:#x}", pc),
        }
    }
}

/// soundStopSE (0x801669bc, 40 bytes)
#[inline(never)]
pub fn f_801669bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801669bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801669bc); f_801669bc_b(c, 0x801669bc); crate::prof::exit(c, 0x801669bc); return; }
    }
    f_801669bc_b(c, 0x801669bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801669bc_at(c: &mut Cpu, pc: u32) { f_801669bc_b(c, pc) }
#[inline(always)]
fn f_801669bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801669bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801669bc, pc); }
        match pc {
            0x801669bc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801669d4; f_801669e4(c);
                pc = 0x801669d4;
            }
            0x801669d4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801669bc: bad pc {:#x}", pc),
        }
    }
}

/// soundStopRaw (0x801669e4, 68 bytes)
#[inline(never)]
pub fn f_801669e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801669e4) { return; }
        if c.prof { crate::prof::enter(c, 0x801669e4); f_801669e4_b(c, 0x801669e4); crate::prof::exit(c, 0x801669e4); return; }
    }
    f_801669e4_b(c, 0x801669e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801669e4_at(c: &mut Cpu, pc: u32) { f_801669e4_b(c, pc) }
#[inline(always)]
fn f_801669e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801669e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801669e4, pc); }
        match pc {
            0x801669e4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz(c.r[3]);
                c.r[0] = c.r[0].rotate_left(25) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166a14; continue; }
                pc = 0x80166a0c;
            }
            0x80166a0c => {
                c.lr = 0x80166a10; f_80167490(c);
                pc = 0x80166a10;
            }
            0x80166a10 => {
                pc = 0x80166a18; continue;
                pc = 0x80166a14;
            }
            0x80166a14 => {
                c.lr = 0x80166a18; f_80167508(c);
                pc = 0x80166a18;
            }
            0x80166a18 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801669e4: bad pc {:#x}", pc),
        }
    }
}

/// soundStartSE (0x80166a28, 40 bytes)
#[inline(never)]
pub fn f_80166a28(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166a28) { return; }
        if c.prof { crate::prof::enter(c, 0x80166a28); f_80166a28_b(c, 0x80166a28); crate::prof::exit(c, 0x80166a28); return; }
    }
    f_80166a28_b(c, 0x80166a28)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166a28_at(c: &mut Cpu, pc: u32) { f_80166a28_b(c, pc) }
#[inline(always)]
fn f_80166a28_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166a28;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166a28, pc); }
        match pc {
            0x80166a28 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80166a40; f_80166ab8(c);
                pc = 0x80166a40;
            }
            0x80166a40 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80166a28: bad pc {:#x}", pc),
        }
    }
}

/// soundStart (0x80166a50, 104 bytes)
#[inline(never)]
pub fn f_80166a50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166a50) { return; }
        if c.prof { crate::prof::enter(c, 0x80166a50); f_80166a50_b(c, 0x80166a50); crate::prof::exit(c, 0x80166a50); return; }
    }
    f_80166a50_b(c, 0x80166a50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166a50_at(c: &mut Cpu, pc: u32) { f_80166a50_b(c, pc) }
#[inline(always)]
fn f_80166a50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166a50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166a50, pc); }
        match pc {
            0x80166a50 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x80166a7c; f_80167408(c);
                pc = 0x80166a7c;
            }
            0x80166a7c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80166a8c; continue; }
                pc = 0x80166a84;
            }
            0x80166a84 => {
                c.r[3] = 0x0;
                pc = 0x80166a9c; continue;
                pc = 0x80166a8c;
            }
            0x80166a8c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80166a9c; f_80166ab8(c);
                pc = 0x80166a9c;
            }
            0x80166a9c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80166a50: bad pc {:#x}", pc),
        }
    }
}

/// soundStartRaw (0x80166ab8, 96 bytes)
#[inline(never)]
pub fn f_80166ab8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166ab8) { return; }
        if c.prof { crate::prof::enter(c, 0x80166ab8); f_80166ab8_b(c, 0x80166ab8); crate::prof::exit(c, 0x80166ab8); return; }
    }
    f_80166ab8_b(c, 0x80166ab8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166ab8_at(c: &mut Cpu, pc: u32) { f_80166ab8_b(c, pc) }
#[inline(always)]
fn f_80166ab8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166ab8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166ab8, pc); }
        match pc {
            0x80166ab8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[7].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[6] = c.lbz(c.r[3]);
                c.r[0] = c.r[6].rotate_left(25) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166af0; continue; }
                pc = 0x80166ae4;
            }
            0x80166ae4 => {
                c.r[4] = c.r[7] | c.r[7];
                c.lr = 0x80166aec; f_8016758c(c);
                pc = 0x80166aec;
            }
            0x80166aec => {
                pc = 0x80166b08; continue;
                pc = 0x80166af0;
            }
            0x80166af0 => {
                c.r[0] = c.r[6].rotate_left(27) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80166b04; continue; }
                pc = 0x80166afc;
            }
            0x80166afc => {
                c.r[3] = 0x0;
                pc = 0x80166b08; continue;
                pc = 0x80166b04;
            }
            0x80166b04 => {
                c.lr = 0x80166b08; f_8016761c(c);
                pc = 0x80166b08;
            }
            0x80166b08 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80166ab8: bad pc {:#x}", pc),
        }
    }
}

/// soundKill (0x80166b18, 36 bytes)
#[inline(never)]
pub fn f_80166b18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166b18) { return; }
        if c.prof { crate::prof::enter(c, 0x80166b18); f_80166b18_b(c, 0x80166b18); crate::prof::exit(c, 0x80166b18); return; }
    }
    f_80166b18_b(c, 0x80166b18)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166b18_at(c: &mut Cpu, pc: u32) { f_80166b18_b(c, pc) }
#[inline(always)]
fn f_80166b18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166b18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166b18, pc); }
        match pc {
            0x80166b18 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80166b2c; f_80167070(c);
                pc = 0x80166b2c;
            }
            0x80166b2c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80166b18: bad pc {:#x}", pc),
        }
    }
}

/// soundAttachWork (0x80166b3c, 164 bytes)
#[inline(never)]
pub fn f_80166b3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166b3c) { return; }
        if c.prof { crate::prof::enter(c, 0x80166b3c); f_80166b3c_b(c, 0x80166b3c); crate::prof::exit(c, 0x80166b3c); return; }
    }
    f_80166b3c_b(c, 0x80166b3c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166b3c_at(c: &mut Cpu, pc: u32) { f_80166b3c_b(c, pc) }
#[inline(always)]
fn f_80166b3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166b3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166b3c, pc); }
        match pc {
            0x80166b3c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[4] | c.r[4];
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[31] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(27) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166b84; continue; }
                pc = 0x80166b7c;
            }
            0x80166b7c => {
                c.r[3] = 0x0;
                pc = 0x80166bc0; continue;
                pc = 0x80166b84;
            }
            0x80166b84 => {
                c.lr = 0x80166b88; f_80167964(c);
                pc = 0x80166b88;
            }
            0x80166b88 => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if !c.crbit(2) { pc = 0x80166ba8; continue; }
                pc = 0x80166b90;
            }
            0x80166b90 => {
                c.lr = 0x80166b94; f_80167318(c);
                pc = 0x80166b94;
            }
            0x80166b94 => {
                c.lr = 0x80166b98; f_80167964(c);
                pc = 0x80166b98;
            }
            0x80166b98 => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if !c.crbit(2) { pc = 0x80166ba8; continue; }
                pc = 0x80166ba0;
            }
            0x80166ba0 => {
                c.r[3] = 0x0;
                pc = 0x80166bc0; continue;
                pc = 0x80166ba8;
            }
            0x80166ba8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80166bb4; f_80167af0(c);
                pc = 0x80166bb4;
            }
            0x80166bb4 => {
                c.stw(c.r[30].wrapping_add(0xc), c.r[28]);
                c.r[3] = 0x1;
                c.stw(c.r[30].wrapping_add(0x10), c.r[29]);
                pc = 0x80166bc0;
            }
            0x80166bc0 => {
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
            _ => panic!("f_80166b3c: bad pc {:#x}", pc),
        }
    }
}

/// GSsndOpenWaveDVD (0x80166be0, 84 bytes)
#[inline(never)]
pub fn f_80166be0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166be0) { return; }
        if c.prof { crate::prof::enter(c, 0x80166be0); f_80166be0_b(c, 0x80166be0); crate::prof::exit(c, 0x80166be0); return; }
    }
    f_80166be0_b(c, 0x80166be0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166be0_at(c: &mut Cpu, pc: u32) { f_80166be0_b(c, pc) }
#[inline(always)]
fn f_80166be0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166be0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166be0, pc); }
        match pc {
            0x80166be0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[12] = c.r[6] | c.r[6];
                c.r[11] = c.r[7] | c.r[7];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[8] | c.r[8];
                c.r[10] = c.r[9] | c.r[9];
                c.r[7] = c.r[12] | c.r[12];
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.r[5] = c.r[4] | c.r[4];
                c.r[8] = c.r[11] | c.r[11];
                c.r[6] = c.r[31] | c.r[31];
                c.r[9] = c.r[0] | c.r[0];
                c.r[4] = 0x1;
                c.lr = 0x80166c20; f_80167118(c);
                pc = 0x80166c20;
            }
            0x80166c20 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80166be0: bad pc {:#x}", pc),
        }
    }
}

/// soundApplyReverb (0x80166c34, 64 bytes)
#[inline(never)]
pub fn f_80166c34(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166c34) { return; }
        if c.prof { crate::prof::enter(c, 0x80166c34); f_80166c34_b(c, 0x80166c34); crate::prof::exit(c, 0x80166c34); return; }
    }
    f_80166c34_b(c, 0x80166c34)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166c34_at(c: &mut Cpu, pc: u32) { f_80166c34_b(c, pc) }
#[inline(always)]
fn f_80166c34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166c34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166c34, pc); }
        match pc {
            0x80166c34 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if c.crbit(2) { pc = 0x80166c64; continue; }
                pc = 0x80166c48;
            }
            0x80166c48 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa8c4));
                c.cmpu(0, c.r[0], c.r[3]);
                if c.crbit(2) { pc = 0x80166c64; continue; }
                pc = 0x80166c54;
            }
            0x80166c54 => {
                c.lr = 0x80166c58; f_80167a9c(c);
                pc = 0x80166c58;
            }
            0x80166c58 => {
                c.r[3] = 0x80450000;
                c.r[3] = c.r[3].wrapping_add(0x2500);
                c.lr = 0x80166c64; f_80164488(c);
                pc = 0x80166c64;
            }
            0x80166c64 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80166c34: bad pc {:#x}", pc),
        }
    }
}

/// GSsndGetOutputMode (0x80166c74, 76 bytes)
#[inline(never)]
pub fn f_80166c74(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166c74) { return; }
        if c.prof { crate::prof::enter(c, 0x80166c74); f_80166c74_b(c, 0x80166c74); crate::prof::exit(c, 0x80166c74); return; }
    }
    f_80166c74_b(c, 0x80166c74)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166c74_at(c: &mut Cpu, pc: u32) { f_80166c74_b(c, pc) }
#[inline(always)]
fn f_80166c74_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166c74;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166c74, pc); }
        match pc {
            0x80166c74 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80166c84; f_800a0e34(c);
                pc = 0x80166c84;
            }
            0x80166c84 => {
                c.cmps(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x80166ca4; continue; }
                pc = 0x80166c8c;
            }
            0x80166c8c => {
                if !c.crbit(0) { pc = 0x80166cac; continue; }
                pc = 0x80166c90;
            }
            0x80166c90 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x80166c9c; continue; }
                pc = 0x80166c98;
            }
            0x80166c98 => {
                pc = 0x80166cac; continue;
                pc = 0x80166c9c;
            }
            0x80166c9c => {
                c.r[3] = 0x0;
                pc = 0x80166cb0; continue;
                pc = 0x80166ca4;
            }
            0x80166ca4 => {
                c.r[3] = 0x1;
                pc = 0x80166cb0; continue;
                pc = 0x80166cac;
            }
            0x80166cac => {
                c.r[3] = 0x0;
                pc = 0x80166cb0;
            }
            0x80166cb0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80166c74: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166CC0 (0x80166cc0, 88 bytes)
#[inline(never)]
pub fn f_80166cc0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166cc0) { return; }
        if c.prof { crate::prof::enter(c, 0x80166cc0); f_80166cc0_b(c, 0x80166cc0); crate::prof::exit(c, 0x80166cc0); return; }
    }
    f_80166cc0_b(c, 0x80166cc0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166cc0_at(c: &mut Cpu, pc: u32) { f_80166cc0_b(c, pc) }
#[inline(always)]
fn f_80166cc0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166cc0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166cc0, pc); }
        match pc {
            0x80166cc0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80166cd8; f_8014d9bc(c);
                pc = 0x80166cd8;
            }
            0x80166cd8 => {
                c.cmps(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80166cf0; continue; }
                pc = 0x80166ce0;
            }
            0x80166ce0 => {
                if c.crbit(0) { pc = 0x80166d04; continue; }
                pc = 0x80166ce4;
            }
            0x80166ce4 => {
                c.cmps(0, c.r[31], 0x3);
                if !c.crbit(0) { pc = 0x80166d04; continue; }
                pc = 0x80166cec;
            }
            0x80166cec => {
                pc = 0x80166cfc; continue;
                pc = 0x80166cf0;
            }
            0x80166cf0 => {
                c.r[3] = 0x0;
                c.lr = 0x80166cf8; f_800a0eb4(c);
                pc = 0x80166cf8;
            }
            0x80166cf8 => {
                pc = 0x80166d04; continue;
                pc = 0x80166cfc;
            }
            0x80166cfc => {
                c.r[3] = 0x1;
                c.lr = 0x80166d04; f_800a0eb4(c);
                pc = 0x80166d04;
            }
            0x80166d04 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80166cc0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166D18 (0x80166d18, 48 bytes)
#[inline(never)]
pub fn f_80166d18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166d18) { return; }
        if c.prof { crate::prof::enter(c, 0x80166d18); f_80166d18_b(c, 0x80166d18); crate::prof::exit(c, 0x80166d18); return; }
    }
    f_80166d18_b(c, 0x80166d18)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166d18_at(c: &mut Cpu, pc: u32) { f_80166d18_b(c, pc) }
#[inline(always)]
fn f_80166d18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166d18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166d18, pc); }
        match pc {
            0x80166d18 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = c.r[5].rotate_left(0) & 0xff;
                c.r[6] = c.r[6].rotate_left(0) & 0xff;
                c.lr = 0x80166d38; f_8014d928(c);
                pc = 0x80166d38;
            }
            0x80166d38 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80166d18: bad pc {:#x}", pc),
        }
    }
}

/// soundMasterVolume (0x80166d48, 252 bytes)
#[inline(never)]
pub fn f_80166d48(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166d48) { return; }
        if c.prof { crate::prof::enter(c, 0x80166d48); f_80166d48_b(c, 0x80166d48); crate::prof::exit(c, 0x80166d48); return; }
    }
    f_80166d48_b(c, 0x80166d48)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166d48_at(c: &mut Cpu, pc: u32) { f_80166d48_b(c, pc) }
#[inline(always)]
fn f_80166d48_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166d48;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166d48, pc); }
        match pc {
            0x80166d48 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[5], 0x1);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                if !c.crbit(2) { pc = 0x80166d78; continue; }
                pc = 0x80166d68;
            }
            0x80166d68 => {
                c.cmpu(0, c.r[6], 0x1);
                if !c.crbit(2) { pc = 0x80166d78; continue; }
                pc = 0x80166d70;
            }
            0x80166d70 => {
                c.r[30] = 0xff;
                pc = 0x80166dac; continue;
                pc = 0x80166d78;
            }
            0x80166d78 => {
                c.cmpu(0, c.r[5], 0x1);
                if !c.crbit(2) { pc = 0x80166d90; continue; }
                pc = 0x80166d80;
            }
            0x80166d80 => {
                c.cmpu(0, c.r[6], 0x1);
                if c.crbit(2) { pc = 0x80166d90; continue; }
                pc = 0x80166d88;
            }
            0x80166d88 => {
                c.r[30] = 0xfd;
                pc = 0x80166dac; continue;
                pc = 0x80166d90;
            }
            0x80166d90 => {
                c.cmpu(0, c.r[5], 0x1);
                if c.crbit(2) { pc = 0x80166e30; continue; }
                pc = 0x80166d98;
            }
            0x80166d98 => {
                c.cmpu(0, c.r[6], 0x1);
                if !c.crbit(2) { pc = 0x80166e30; continue; }
                pc = 0x80166da0;
            }
            0x80166da0 => {
                c.r[30] = 0xfe;
                pc = 0x80166dac; continue;
                pc = 0x80166da8;
            }
            0x80166da8 => {
                pc = 0x80166e30; continue;
                pc = 0x80166dac;
            }
            0x80166dac => {
                c.r[29] = 0x0;
                c.r[31] = 0x0;
                pc = 0x80166e10; continue;
                pc = 0x80166db8;
            }
            0x80166db8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[4] = c.lbz(c.r[4].wrapping_add(c.r[31]));
                c.r[0] = c.r[4].rotate_left(27) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166e08; continue; }
                pc = 0x80166dcc;
            }
            0x80166dcc => {
                c.cmps(0, c.r[30], 0xfe);
                if c.crbit(2) { pc = 0x80166df8; continue; }
                pc = 0x80166dd4;
            }
            0x80166dd4 => {
                if !c.crbit(0) { pc = 0x80166e00; continue; }
                pc = 0x80166dd8;
            }
            0x80166dd8 => {
                c.cmps(0, c.r[30], 0xfd);
                if !c.crbit(0) { pc = 0x80166de8; continue; }
                pc = 0x80166de0;
            }
            0x80166de0 => {
                pc = 0x80166e00; continue;
                pc = 0x80166de4;
            }
            0x80166de4 => {
                pc = 0x80166e00; continue;
                pc = 0x80166de8;
            }
            0x80166de8 => {
                c.r[0] = c.r[4].rotate_left(25) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80166e08; continue; }
                pc = 0x80166df4;
            }
            0x80166df4 => {
                pc = 0x80166e00; continue;
                pc = 0x80166df8;
            }
            0x80166df8 => {
                c.r[0] = c.r[4].rotate_left(25) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80166e08; continue; }
                pc = 0x80166e00;
            }
            0x80166e00 => {
                c.r[4] = c.r[27].rotate_left(0) & 0xff;
                c.lr = 0x80166e08; f_80167408(c);
                pc = 0x80166e08;
            }
            0x80166e08 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x80166e10;
            }
            0x80166e10 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa8c8));
                c.r[3] = c.r[29] | c.r[29];
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x80166db8; continue; }
                pc = 0x80166e20;
            }
            0x80166e20 => {
                c.r[3] = c.r[27].rotate_left(0) & 0xff;
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = c.r[30].rotate_left(0) & 0xff;
                c.lr = 0x80166e30; f_8014d8c8(c);
                pc = 0x80166e30;
            }
            0x80166e30 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80166d48: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166E44 (0x80166e44, 68 bytes)
#[inline(never)]
pub fn f_80166e44(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166e44) { return; }
        if c.prof { crate::prof::enter(c, 0x80166e44); f_80166e44_b(c, 0x80166e44); crate::prof::exit(c, 0x80166e44); return; }
    }
    f_80166e44_b(c, 0x80166e44)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166e44_at(c: &mut Cpu, pc: u32) { f_80166e44_b(c, pc) }
#[inline(always)]
fn f_80166e44_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166e44;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166e44, pc); }
        match pc {
            0x80166e44 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80166e54; f_8015ffd4(c);
                pc = 0x80166e54;
            }
            0x80166e54 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80166e78; continue; }
                pc = 0x80166e60;
            }
            0x80166e60 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x80166e74; f_80166d48(c);
                pc = 0x80166e74;
            }
            0x80166e74 => {
                c.lr = 0x80166e78; f_8015ffa0(c);
                pc = 0x80166e78;
            }
            0x80166e78 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80166e44: bad pc {:#x}", pc),
        }
    }
}

/// soundInit (0x80166e88, 440 bytes)
#[inline(never)]
pub fn f_80166e88(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80166e88) { return; }
        if c.prof { crate::prof::enter(c, 0x80166e88); f_80166e88_b(c, 0x80166e88); crate::prof::exit(c, 0x80166e88); return; }
    }
    f_80166e88_b(c, 0x80166e88)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166e88_at(c: &mut Cpu, pc: u32) { f_80166e88_b(c, pc) }
#[inline(always)]
fn f_80166e88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80166e88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80166e88, pc); }
        match pc {
            0x80166e88 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(c.r[5]);
                c.r[3] = c.r[0].wrapping_mul(0x14);
                c.r[5] = 0xffffffff;
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.r[31] = c.r[7] | c.r[7];
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.r[30] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.r[8] = c.lwz(c.r[2].wrapping_add(0xffff9eb8));
                c.r[9] = c.lwz(c.r[2].wrapping_add(0xffff9ebc));
                c.stw(c.r[1].wrapping_add(0x10), c.r[8]);
                c.r[8] = c.lwz(c.r[13].wrapping_add(0xffff8788));
                c.stw(c.r[1].wrapping_add(0x14), c.r[9]);
                c.r[4] = c.lwz(c.r[8]);
                c.stw(c.r[13].wrapping_add(0xffffa8c8), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa8c4), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa8c0), c.r[0]);
                c.lr = 0x80166ee0; f_80167bb0(c);
                pc = 0x80166ee0;
            }
            0x80166ee0 => {
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa8bc), c.r[3]);
                if !c.crbit(2) { pc = 0x80166ef4; continue; }
                pc = 0x80166eec;
            }
            0x80166eec => {
                c.r[3] = 0x0;
                pc = 0x80167024; continue;
                pc = 0x80166ef4;
            }
            0x80166ef4 => {
                c.lr = 0x80166ef8; f_80167a6c(c);
                pc = 0x80166ef8;
            }
            0x80166ef8 => {
                c.stw(c.r[13].wrapping_add(0xffffa8b8), c.r[29]);
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80166f04; f_80167bb0(c);
                pc = 0x80166f04;
            }
            0x80166f04 => {
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa8b4), c.r[3]);
                if !c.crbit(2) { pc = 0x80166f18; continue; }
                pc = 0x80166f10;
            }
            0x80166f10 => {
                c.r[3] = 0x0;
                pc = 0x80167024; continue;
                pc = 0x80166f18;
            }
            0x80166f18 => {
                c.lr = 0x80166f1c; f_80167a44(c);
                pc = 0x80166f1c;
            }
            0x80166f1c => {
                c.r[3] = c.r[30].wrapping_mul(0xd0);
                c.stw(c.r[13].wrapping_add(0xffffa8b0), c.r[30]);
                c.lr = 0x80166f28; f_80167bb0(c);
                pc = 0x80166f28;
            }
            0x80166f28 => {
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa8ac), c.r[3]);
                if !c.crbit(2) { pc = 0x80166f3c; continue; }
                pc = 0x80166f34;
            }
            0x80166f34 => {
                c.r[3] = 0x0;
                pc = 0x80167024; continue;
                pc = 0x80166f3c;
            }
            0x80166f3c => {
                c.lr = 0x80166f40; f_80167a14(c);
                pc = 0x80166f40;
            }
            0x80166f40 => {
                c.r[3] = c.r[31].wrapping_mul(0x78);
                c.stw(c.r[13].wrapping_add(0xffffa8a8), c.r[31]);
                c.lr = 0x80166f4c; f_80167bb0(c);
                pc = 0x80166f4c;
            }
            0x80166f4c => {
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa8a4), c.r[3]);
                if !c.crbit(2) { pc = 0x80166f60; continue; }
                pc = 0x80166f58;
            }
            0x80166f58 => {
                c.r[3] = 0x0;
                pc = 0x80167024; continue;
                pc = 0x80166f60;
            }
            0x80166f60 => {
                c.lr = 0x80166f64; f_801679e4(c);
                pc = 0x80166f64;
            }
            0x80166f64 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x80166f6c; f_801631ac(c);
                pc = 0x80166f6c;
            }
            0x80166f6c => {
                c.r[4] = 0xa00000;
                c.r[3] = 0x40;
                c.r[8] = c.r[4].wrapping_add(0xffffc000);
                c.r[4] = 0x30;
                c.r[5] = 0x10;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x80166f8c; f_8015fe88(c);
                pc = 0x80166f8c;
            }
            0x80166f8c => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80166f9c; continue; }
                pc = 0x80166f94;
            }
            0x80166f94 => {
                c.r[3] = 0x0;
                pc = 0x80167024; continue;
                pc = 0x80166f9c;
            }
            0x80166f9c => {
                c.r[3] = 0x0;
                c.lr = 0x80166fa4; f_80167a9c(c);
                pc = 0x80166fa4;
            }
            0x80166fa4 => {
                c.r[3] = 0x80450000;
                c.r[3] = c.r[3].wrapping_add(0x2500);
                c.lr = 0x80166fb0; f_801644e0(c);
                pc = 0x80166fb0;
            }
            0x80166fb0 => {
                c.r[0] = 0x0;
                c.r[3] = 0x80160000;
                c.r[5] = 0x80450000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x442c);
                c.r[3] = 0x0;
                c.r[5] = c.r[5].wrapping_add(0x2500);
                c.r[6] = 0xff;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.r[9] = 0x0;
                c.r[10] = 0xff;
                c.lr = 0x80166fe4; f_8014daa8(c);
                pc = 0x80166fe4;
            }
            0x80166fe4 => {
                c.r[3] = 0x7f;
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80166ff8; f_80166d48(c);
                pc = 0x80166ff8;
            }
            0x80166ff8 => {
                c.r[3] = 0x64;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x8016700c; f_80166d48(c);
                pc = 0x8016700c;
            }
            0x8016700c => {
                c.lr = 0x80167010; f_80166c74(c);
                pc = 0x80167010;
            }
            0x80167010 => {
                c.lr = 0x80167014; f_80166cc0(c);
                pc = 0x80167014;
            }
            0x80167014 => {
                c.r[3] = 0x80160000;
                c.r[3] = c.r[3].wrapping_add(0x7040);
                c.lr = 0x80167020; f_8014d8c0(c);
                pc = 0x80167020;
            }
            0x80167020 => {
                c.r[3] = 0x1;
                pc = 0x80167024;
            }
            0x80167024 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80166e88: bad pc {:#x}", pc),
        }
    }
}

/// fn_80167040 (0x80167040, 48 bytes)
#[inline(never)]
pub fn f_80167040(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80167040) { return; }
        if c.prof { crate::prof::enter(c, 0x80167040); f_80167040_b(c, 0x80167040); crate::prof::exit(c, 0x80167040); return; }
    }
    f_80167040_b(c, 0x80167040)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80167040_at(c: &mut Cpu, pc: u32) { f_80167040_b(c, pc) }
#[inline(always)]
fn f_80167040_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80167040;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80167040, pc); }
        match pc {
            0x80167040 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80167050; f_80167720(c);
                pc = 0x80167050;
            }
            0x80167050 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80167060; continue; }
                pc = 0x80167058;
            }
            0x80167058 => {
                c.r[4] = 0x0;
                c.lr = 0x80167060; f_80167070(c);
                pc = 0x80167060;
            }
            0x80167060 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80167040: bad pc {:#x}", pc),
        }
    }
}

/// fn_80167070 (0x80167070, 168 bytes)
#[inline(never)]
pub fn f_80167070(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80167070) { return; }
        if c.prof { crate::prof::enter(c, 0x80167070); f_80167070_b(c, 0x80167070); crate::prof::exit(c, 0x80167070); return; }
    }
    f_80167070_b(c, 0x80167070)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80167070_at(c: &mut Cpu, pc: u32) { f_80167070_b(c, pc) }
#[inline(always)]
fn f_80167070_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80167070;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80167070, pc); }
        match pc {
            0x80167070 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[31] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = c.lbz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(27) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x801670a8; continue; }
                pc = 0x801670a0;
            }
            0x801670a0 => {
                c.r[3] = 0x0;
                pc = 0x80167100; continue;
                pc = 0x801670a8;
            }
            0x801670a8 => {
                c.r[30] = c.lwz(c.r[31].wrapping_add(0x8));
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x801670bc; continue; }
                pc = 0x801670b4;
            }
            0x801670b4 => {
                c.r[3] = 0x0;
                pc = 0x80167100; continue;
                pc = 0x801670bc;
            }
            0x801670bc => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x801670d4; continue; }
                pc = 0x801670c8;
            }
            0x801670c8 => {
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x801670d4; f_801669e4(c);
                pc = 0x801670d4;
            }
            0x801670d4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x801670e0; f_80167af0(c);
                pc = 0x801670e0;
            }
            0x801670e0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801670e8; f_8016782c(c);
                pc = 0x801670e8;
            }
            0x801670e8 => {
                c.r[4] = 0x0;
                c.r[3] = 0x1;
                c.stw(c.r[31].wrapping_add(0x8), c.r[4]);
                c.r[0] = c.lbz(c.r[31]);
                c.r[0] = (c.r[4].rotate_left(5) & 0x20) | (c.r[0] & 0xffffffdf);
                c.stb(c.r[31], c.r[0]);
                pc = 0x80167100;
            }
            0x80167100 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80167070: bad pc {:#x}", pc),
        }
    }
}

/// soundPushWave (0x80167118, 384 bytes)
#[inline(never)]
pub fn f_80167118(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80167118) { return; }
        if c.prof { crate::prof::enter(c, 0x80167118); f_80167118_b(c, 0x80167118); crate::prof::exit(c, 0x80167118); return; }
    }
    f_80167118_b(c, 0x80167118)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80167118_at(c: &mut Cpu, pc: u32) { f_80167118_b(c, pc) }
#[inline(always)]
fn f_80167118_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80167118;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80167118, pc); }
        match pc {
            0x80167118 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0x18);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[31] = c.r[6] | c.r[6];
                c.r[23] = c.r[7] | c.r[7];
                c.r[26] = c.r[8] | c.r[8];
                c.r[24] = c.r[10] | c.r[10];
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8794));
                c.r[28] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80167160; continue; }
                pc = 0x80167158;
            }
            0x80167158 => {
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x80167160;
            }
            0x80167160 => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[9] | c.r[9];
                c.lr = 0x8016716c; f_800f9318(c);
                pc = 0x8016716c;
            }
            0x8016716c => {
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                if !c.crbit(2) { pc = 0x8016717c; continue; }
                pc = 0x80167174;
            }
            0x80167174 => {
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x8016717c;
            }
            0x8016717c => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[24] | c.r[24];
                c.lr = 0x80167188; f_800f9318(c);
                pc = 0x80167188;
            }
            0x80167188 => {
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if !c.crbit(2) { pc = 0x80167198; continue; }
                pc = 0x80167190;
            }
            0x80167190 => {
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x80167198;
            }
            0x80167198 => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x801671a4; f_800f9318(c);
                pc = 0x801671a4;
            }
            0x801671a4 => {
                c.r[24] = c.r[3] | c.r[3];
                c.rc(c.r[24]);
                if !c.crbit(2) { pc = 0x801671b4; continue; }
                pc = 0x801671ac;
            }
            0x801671ac => {
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x801671b4;
            }
            0x801671b4 => {
                c.cmpu(0, c.r[29], 0x1);
                if !c.crbit(2) { pc = 0x80167218; continue; }
                pc = 0x801671bc;
            }
            0x801671bc => {
                c.r[26] = 0x0;
                c.lr = 0x801671c4; f_800ae78c(c);
                pc = 0x801671c4;
            }
            0x801671c4 => {
                c.r[5] = 0x80160000;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[5].wrapping_add(0x7298);
                c.lr = 0x801671d4; f_80159ed0(c);
                pc = 0x801671d4;
            }
            0x801671d4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801671dc; f_80167f28(c);
                pc = 0x801671dc;
            }
            0x801671dc => {
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa89c), c.r[3]);
                if !c.crbit(2) { pc = 0x801671f0; continue; }
                pc = 0x801671e8;
            }
            0x801671e8 => {
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x801671f0;
            }
            0x801671f0 => {
                c.stw(c.r[13].wrapping_add(0xffffa898), c.r[31]);
                c.lr = 0x801671f8; f_800ae78c(c);
                pc = 0x801671f8;
            }
            0x801671f8 => {
                c.lr = 0x801671fc; f_80167bb0(c);
                pc = 0x801671fc;
            }
            0x801671fc => {
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa8a0), c.r[3]);
                if !c.crbit(2) { pc = 0x80167228; continue; }
                pc = 0x80167208;
            }
            0x80167208 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa89c));
                c.lr = 0x80167210; f_80167e64(c);
                pc = 0x80167210;
            }
            0x80167210 => {
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x80167218;
            }
            0x80167218 => {
                c.r[26] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80167228; f_80159ed0(c);
                pc = 0x80167228;
            }
            0x80167228 => {
                c.r[4] = c.lhz(c.r[28].wrapping_add(0x2));
                c.r[3] = c.r[27] | c.r[27];
                c.r[5] = c.r[26] | c.r[26];
                c.r[6] = c.r[25] | c.r[25];
                c.r[7] = c.r[24] | c.r[24];
                c.lr = 0x80167240; f_80159ef0(c);
                pc = 0x80167240;
            }
            0x80167240 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80167260; continue; }
                pc = 0x80167248;
            }
            0x80167248 => {
                c.cmpu(0, c.r[29], 0x1);
                if !c.crbit(2) { pc = 0x80167258; continue; }
                pc = 0x80167250;
            }
            0x80167250 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa89c));
                c.lr = 0x80167258; f_80167e64(c);
                pc = 0x80167258;
            }
            0x80167258 => {
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x80167260;
            }
            0x80167260 => {
                c.cmpu(0, c.r[29], 0x1);
                if !c.crbit(2) { pc = 0x80167278; continue; }
                pc = 0x80167268;
            }
            0x80167268 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa8a0));
                c.lr = 0x80167270; f_80167b70(c);
                pc = 0x80167270;
            }
            0x80167270 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa89c));
                c.lr = 0x80167278; f_80167e64(c);
                pc = 0x80167278;
            }
            0x80167278 => {
                c.r[0] = 0x1;
                c.r[3] = 0x1;
                c.stb(c.r[28].wrapping_add(0x1), c.r[0]);
                pc = 0x80167284;
            }
            0x80167284 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80167118: bad pc {:#x}", pc),
        }
    }
}

/// _sndSetSampleDataUploadCallbackFunction (0x80167298, 128 bytes)
#[inline(never)]
pub fn f_80167298(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80167298) { return; }
        if c.prof { crate::prof::enter(c, 0x80167298); f_80167298_b(c, 0x80167298); crate::prof::exit(c, 0x80167298); return; }
    }
    f_80167298_b(c, 0x80167298)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80167298_at(c: &mut Cpu, pc: u32) { f_80167298_b(c, pc) }
#[inline(always)]
fn f_80167298_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80167298;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80167298, pc); }
        match pc {
            0x80167298 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x801672b4;
            }
            0x801672b4 => {
                c.lr = 0x801672b8; f_800ac954(c);
                pc = 0x801672b8;
            }
            0x801672b8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801672b4; continue; }
                pc = 0x801672c0;
            }
            0x801672c0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa898));
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa89c));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa8a0));
                c.r[6] = c.r[30].wrapping_add(c.r[0]);
                c.lr = 0x801672d8; f_80167ed0(c);
                pc = 0x801672d8;
            }
            0x801672d8 => {
                c.cmps(0, c.r[3], 0xfffffffe);
                if c.crbit(2) { pc = 0x801672fc; continue; }
                pc = 0x801672e0;
            }
            0x801672e0 => {
                if !c.crbit(0) { pc = 0x801672f0; continue; }
                pc = 0x801672e4;
            }
            0x801672e4 => {
                c.cmps(0, c.r[3], 0xfffffffd);
                if !c.crbit(0) { pc = 0x801672c0; continue; }
                pc = 0x801672ec;
            }
            0x801672ec => {
                pc = 0x801672fc; continue;
                pc = 0x801672f0;
            }
            0x801672f0 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x801672fc; continue; }
                pc = 0x801672f8;
            }
            0x801672f8 => {
                pc = 0x801672c0; continue;
                pc = 0x801672fc;
            }
            0x801672fc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa8a0));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80167298: bad pc {:#x}", pc),
        }
    }
}

/// _sndCheckSndWorkALL (0x80167318, 100 bytes)
#[inline(never)]
pub fn f_80167318(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80167318) { return; }
        if c.prof { crate::prof::enter(c, 0x80167318); f_80167318_b(c, 0x80167318); crate::prof::exit(c, 0x80167318); return; }
    }
    f_80167318_b(c, 0x80167318)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80167318_at(c: &mut Cpu, pc: u32) { f_80167318_b(c, pc) }
#[inline(always)]
fn f_80167318_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80167318;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80167318, pc); }
        match pc {
            0x80167318 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x3748);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80167338; stubs_gen::f_800dd970(c);
                pc = 0x80167338;
            }
            0x80167338 => {
                c.r[31] = 0x0;
                pc = 0x80167348; continue;
                pc = 0x80167340;
            }
            0x80167340 => {
                c.lr = 0x80167344; f_801666bc(c);
                pc = 0x80167344;
            }
            0x80167344 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80167348;
            }
            0x80167348 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa8c8));
                c.r[3] = c.r[31] | c.r[31];
                c.cmpu(0, c.r[31], c.r[0]);
                if c.crbit(0) { pc = 0x80167340; continue; }
                pc = 0x80167358;
            }
            0x80167358 => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x3764);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80167368; stubs_gen::f_800dd970(c);
                pc = 0x80167368;
            }
            0x80167368 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80167318: bad pc {:#x}", pc),
        }
    }
}

/// fn_8016737C (0x8016737c, 140 bytes)
#[inline(never)]
pub fn f_8016737c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8016737c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016737c); f_8016737c_b(c, 0x8016737c); crate::prof::exit(c, 0x8016737c); return; }
    }
    f_8016737c_b(c, 0x8016737c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016737c_at(c: &mut Cpu, pc: u32) { f_8016737c_b(c, pc) }
#[inline(always)]
fn f_8016737c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8016737c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8016737c, pc); }
        match pc {
            0x8016737c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = c.lwz(c.r[3].wrapping_add(0x8));
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(2) { pc = 0x8016739c; continue; }
                pc = 0x80167394;
            }
            0x80167394 => {
                c.r[3] = 0x0;
                pc = 0x801673f8; continue;
                pc = 0x8016739c;
            }
            0x8016739c => {
                c.r[0] = c.r[5].rotate_left(0) & 0x7f;
                c.stb(c.r[7].wrapping_add(0x1), c.r[0]);
                c.r[6] = c.lwz(c.r[7].wrapping_add(0x8));
                c.r[0] = c.r[6].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x801673f4; continue; }
                pc = 0x801673b4;
            }
            0x801673b4 => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[0] = c.r[0].rotate_left(25) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x801673d8; continue; }
                pc = 0x801673c4;
            }
            0x801673c4 => {
                c.r[5] = c.lbz(c.r[7].wrapping_add(0x1));
                c.r[3] = c.r[6] | c.r[6];
                c.r[4] = 0x7;
                c.lr = 0x801673d4; f_8014d75c(c);
                pc = 0x801673d4;
            }
            0x801673d4 => {
                pc = 0x801673ec; continue;
                pc = 0x801673d8;
            }
            0x801673d8 => {
                c.r[3] = c.lbz(c.r[7].wrapping_add(0x1));
                c.r[5] = c.r[6] | c.r[6];
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                c.r[6] = 0x0;
                c.lr = 0x801673ec; f_8014d6d8(c);
                pc = 0x801673ec;
            }
            0x801673ec => {
                c.r[3] = 0x1;
                pc = 0x801673f8; continue;
                pc = 0x801673f4;
            }
            0x801673f4 => {
                c.r[3] = 0x0;
                pc = 0x801673f8;
            }
            0x801673f8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8016737c: bad pc {:#x}", pc),
        }
    }
}

/// _sndSetVolumeWork (0x80167408, 136 bytes)
#[inline(never)]
pub fn f_80167408(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80167408) { return; }
        if c.prof { crate::prof::enter(c, 0x80167408); f_80167408_b(c, 0x80167408); crate::prof::exit(c, 0x80167408); return; }
    }
    f_80167408_b(c, 0x80167408)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80167408_at(c: &mut Cpu, pc: u32) { f_80167408_b(c, pc) }
#[inline(always)]
fn f_80167408_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80167408;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80167408, pc); }
        match pc {
            0x80167408 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff878c));
                c.r[31] = c.r[5].wrapping_add(c.r[0]);
                c.r[4] = c.lbz(c.r[31]);
                c.r[0] = c.r[4].rotate_left(27) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80167460; continue; }
                pc = 0x8016743c;
            }
            0x8016743c => {
                c.r[0] = c.r[4].rotate_left(25) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80167458; continue; }
                pc = 0x80167448;
            }
            0x80167448 => {
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x80167454; f_80166b3c(c);
                pc = 0x80167454;
            }
            0x80167454 => {
                pc = 0x80167460; continue;
                pc = 0x80167458;
            }
            0x80167458 => {
                c.r[3] = 0x0;
                pc = 0x80167478; continue;
                pc = 0x80167460;
            }
            0x80167460 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80167474; continue; }
                pc = 0x8016746c;
            }
            0x8016746c => {
                c.r[0] = c.r[30].rotate_left(0) & 0x7f;
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                pc = 0x80167474;
            }
            0x80167474 => {
                c.r[3] = 0x1;
                pc = 0x80167478;
            }
            0x80167478 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80167408: bad pc {:#x}", pc),
        }
    }
}

