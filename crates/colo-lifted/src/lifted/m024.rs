#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// fn_8015F270 (0x8015f270, 944 bytes)
#[inline(never)]
pub fn f_8015f270(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8015f270) { return; }
        if c.prof { crate::prof::enter(c, 0x8015f270); p = true; }
    }
    f_8015f270_b(c, 0x8015f270);
    if p { crate::prof::exit(c, 0x8015f270); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015f270_at(c: &mut Cpu, pc: u32) { f_8015f270_b(c, pc) }
#[inline(never)]
fn f_8015f270_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8015f270;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8015f270, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8015f270 => { ins += 11;
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
                c.lr = 0x8015f29c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c47c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f29c;
            }
            0x8015f29c => { ins += 4;
                c.r[3] = 0x80450000;
                c.r[26] = 0x0;
                c.r[27] = c.r[3].wrapping_add(0xffff8590);
                pc = 0x8015f5e4; continue;
                pc = 0x8015f2ac;
            }
            0x8015f2ac => { ins += 5;
                c.r[25] = c.lwz_t(ins, c.r[27].wrapping_add(0x4));
                c.r[30] = c.r[27].wrapping_add(0x8);
                c.r[29] = c.r[27].wrapping_add(0x4);
                c.r[28] = c.r[27].wrapping_add(0xc);
                pc = 0x8015f5d4; continue;
                pc = 0x8015f2c0;
            }
            0x8015f2c0 => { ins += 3;
                c.r[5] = c.lwz_t(ins, c.r[30]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015f354; continue; }
                pc = 0x8015f2cc;
            }
            0x8015f2cc => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa813));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015f2fc; continue; }
                pc = 0x8015f2d8;
            }
            0x8015f2d8 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[27]);
                c.r[0] = c.r[0].rotate_left(0) & 0x80000000;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015f2fc; continue; }
                pc = 0x8015f2e4;
            }
            0x8015f2e4 => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[29]);
                c.r[4] = c.lhz_t(ins, c.r[28]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x18));
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x47));
                cr_cmps(&mut cr, c.xer, 0, c.r[4], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8015f354; continue; }
                pc = 0x8015f2fc;
            }
            0x8015f2fc => { ins += 7;
                { let ea = c.r[25].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[5].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9e08); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] - c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                { let (a, b) = (cr_bit(cr, 0), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if cr_bit(cr, 2) { pc = 0x8015f5d0; continue; }
                pc = 0x8015f318;
            }
            0x8015f318 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff9e0c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                { let (a, b) = (cr_bit(cr, 0), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if !cr_bit(cr, 2) { pc = 0x8015f348; continue; }
                pc = 0x8015f328;
            }
            0x8015f328 => { ins += 7;
                c.r[4] = c.lwz_t(ins, c.r[25].wrapping_add(0x18));
                c.r[3] = c.lhz_t(ins, c.r[4].wrapping_add(0x48));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.sth(c.r[4].wrapping_add(0x48), c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x14);
                if cr_bit(cr, 0) { pc = 0x8015f5d0; continue; }
                pc = 0x8015f344;
            }
            0x8015f344 => { ins += 1;
                pc = 0x8015f354; continue;
                pc = 0x8015f348;
            }
            0x8015f348 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[25].wrapping_add(0x18));
                c.r[0] = 0x0;
                c.sth(c.r[3].wrapping_add(0x48), c.r[0]);
                pc = 0x8015f354;
            }
            0x8015f354 => { ins += 4;
                c.r[24] = c.lwz_t(ins, c.r[25].wrapping_add(0x18));
                c.r[6] = c.lwz_t(ins, c.r[24].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015f370; continue; }
                pc = 0x8015f364;
            }
            0x8015f364 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[6].wrapping_add(0x1c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x8015f3ac; continue; }
                pc = 0x8015f370;
            }
            0x8015f370 => { ins += 5;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x0);
                c.r[3] = c.lhz_t(ins, c.r[24].wrapping_add(0x44));
                c.r[4] = 0x7f;
                c.r[5] = 0x40;
                if cr_bit(cr, 2) { pc = 0x8015f38c; continue; }
                pc = 0x8015f384;
            }
            0x8015f384 => { ins += 2;
                c.r[6] = c.lbz_t(ins, c.r[6].wrapping_add(0x1c));
                pc = 0x8015f390; continue;
                pc = 0x8015f38c;
            }
            0x8015f38c => { ins += 1;
                c.r[6] = c.lbz_t(ins, c.r[24].wrapping_add(0x46));
                pc = 0x8015f390;
            }
            0x8015f390 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[24].wrapping_add(0x10));
                c.r[7] = c.r[0].rotate_left(28) & 0x1;
                c.lr = 0x8015f39c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c5e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f39c;
            }
            0x8015f39c => { ins += 4;
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[24].wrapping_add(0x3c), c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if !cr_bit(cr, 2) { pc = 0x8015f3d0; continue; }
                pc = 0x8015f3ac;
            }
            0x8015f3ac => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[24].wrapping_add(0x10));
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8015f5d0; continue; }
                pc = 0x8015f3b8;
            }
            0x8015f3b8 => { ins += 6;
                c.r[0] = c.r[3] | 0x40000;
                c.stw(c.r[24].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[24].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffdffff;
                c.stw(c.r[24].wrapping_add(0x10), c.r[0]);
                pc = 0x8015f5d0; continue;
                pc = 0x8015f3d0;
            }
            0x8015f3d0 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[24].wrapping_add(0x10));
                c.r[0] = c.r[3].rotate_left(0) & 0x20;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8015f3f0; continue; }
                pc = 0x8015f3dc;
            }
            0x8015f3dc => { ins += 5;
                c.r[0] = c.r[3] | 0x100000;
                { let ea = c.r[2].wrapping_add(0xffff9dc8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[24].wrapping_add(0x10), c.r[0]);
                { let ea = c.r[24].wrapping_add(0x4c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8015f3f8; continue;
                pc = 0x8015f3f0;
            }
            0x8015f3f0 => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffff9dec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[24].wrapping_add(0x4c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8015f3f8;
            }
            0x8015f3f8 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[24].wrapping_add(0x10));
                { let ea = c.r[25].wrapping_add(0x14); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                c.r[0] = c.r[0].rotate_left(0) & 0x100000;
                cr_rc(&mut cr, c.xer, c.r[0]);
                { let ea = c.r[25].wrapping_add(0x10); c.f[29] = c.lfs(ea); c.ps1[29] = c.f[29]; }
                { let ea = c.r[25].wrapping_add(0x8); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                { let ea = c.r[25].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[31] = c.lwz_t(ins, c.r[24].wrapping_add(0x3c));
                if cr_bit(cr, 2) { pc = 0x8015f458; continue; }
                pc = 0x8015f418;
            }
            0x8015f418 => { ins += 13;
                { let ea = c.r[24].wrapping_add(0x4c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[31] | c.r[31];
                { let ea = c.r[2].wrapping_add(0xffff9de8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = 0x7;
                { let v = s(c.f[0] * c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                c.r[5] = 0x7f;
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7f);
                if cr_bit(cr, 1) { pc = 0x8015f450; continue; }
                pc = 0x8015f44c;
            }
            0x8015f44c => { ins += 1;
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x8015f450;
            }
            0x8015f450 => { ins += 1;
                c.lr = 0x8015f454; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c6b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f454;
            }
            0x8015f454 => { ins += 1;
                pc = 0x8015f490; continue;
                pc = 0x8015f458;
            }
            0x8015f458 => { ins += 9;
                { let ea = c.r[2].wrapping_add(0xffff9de8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = 0x7f;
                { let v = s(c.f[0] * c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7f);
                if cr_bit(cr, 1) { pc = 0x8015f484; continue; }
                pc = 0x8015f47c;
            }
            0x8015f47c => { ins += 2;
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                pc = 0x8015f484;
            }
            0x8015f484 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7;
                c.lr = 0x8015f490; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c6b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f490;
            }
            0x8015f490 => { ins += 11;
                { let ea = c.r[2].wrapping_add(0xffff9dec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = 0x7f;
                { let ea = c.r[2].wrapping_add(0xffff9dfc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[0] + c.f[30]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7f);
                if cr_bit(cr, 1) { pc = 0x8015f4c4; continue; }
                pc = 0x8015f4bc;
            }
            0x8015f4bc => { ins += 2;
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                pc = 0x8015f4c4;
            }
            0x8015f4c4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0xa;
                c.lr = 0x8015f4d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c6b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f4d0;
            }
            0x8015f4d0 => { ins += 11;
                { let ea = c.r[2].wrapping_add(0xffff9dec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = 0x7f;
                { let ea = c.r[2].wrapping_add(0xffff9dfc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[0] - c.f[29]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x18); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7f);
                if cr_bit(cr, 1) { pc = 0x8015f504; continue; }
                pc = 0x8015f4fc;
            }
            0x8015f4fc => { ins += 2;
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                pc = 0x8015f504;
            }
            0x8015f504 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x83;
                c.lr = 0x8015f510; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c6b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f510;
            }
            0x8015f510 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9e00); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] * c.f[31]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x8015f51c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c46b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f51c;
            }
            0x8015f51c => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x3fff);
                c.r[5] = 0x3fff;
                if cr_bit(cr, 1) { pc = 0x8015f52c; continue; }
                pc = 0x8015f528;
            }
            0x8015f528 => { ins += 1;
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8015f52c;
            }
            0x8015f52c => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x84;
                c.lr = 0x8015f538; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c794(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f538;
            }
            0x8015f538 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[24].wrapping_add(0xc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015f5a4; continue; }
                pc = 0x8015f544;
            }
            0x8015f544 => { ins += 3;
                c.r[23] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[22] = 0x0;
                pc = 0x8015f590; continue;
                pc = 0x8015f550;
            }
            0x8015f550 => { ins += 3;
                c.r[4] = c.lbz_t(ins, c.r[23]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x40);
                if cr_bit(cr, 0) { pc = 0x8015f56c; continue; }
                pc = 0x8015f55c;
            }
            0x8015f55c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x80);
                if cr_bit(cr, 2) { pc = 0x8015f56c; continue; }
                pc = 0x8015f564;
            }
            0x8015f564 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x84);
                if !cr_bit(cr, 2) { pc = 0x8015f57c; continue; }
                pc = 0x8015f56c;
            }
            0x8015f56c => { ins += 3;
                c.r[5] = c.lhz_t(ins, c.r[23].wrapping_add(0x2));
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8015f578; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c794(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f578;
            }
            0x8015f578 => { ins += 1;
                pc = 0x8015f588; continue;
                pc = 0x8015f57c;
            }
            0x8015f57c => { ins += 3;
                c.r[5] = c.lbz_t(ins, c.r[23].wrapping_add(0x2));
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8015f588; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c6b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f588;
            }
            0x8015f588 => { ins += 2;
                c.r[23] = c.r[23].wrapping_add(0x4);
                c.r[22] = c.r[22].wrapping_add(0x1);
                pc = 0x8015f590;
            }
            0x8015f590 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[24].wrapping_add(0xc));
                c.r[4] = c.r[22].rotate_left(0) & 0xff;
                c.r[0] = c.lbz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8015f550; continue; }
                pc = 0x8015f5a4;
            }
            0x8015f5a4 => { ins += 9;
                c.r[0] = c.lwz_t(ins, c.r[24].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffdffff;
                c.stw(c.r[24].wrapping_add(0x10), c.r[0]);
                c.r[3] = c.lhz_t(ins, c.r[28]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[28], c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[30]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015f5d0; continue; }
                pc = 0x8015f5c8;
            }
            0x8015f5c8 => { ins += 2;
                c.r[0] = c.lwz_t(ins, c.r[3]);
                c.stw(c.r[30], c.r[0]);
                pc = 0x8015f5d0;
            }
            0x8015f5d0 => { ins += 1;
                c.r[25] = c.lwz_t(ins, c.r[25]);
                pc = 0x8015f5d4;
            }
            0x8015f5d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[25], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8015f2c0; continue; }
                pc = 0x8015f5dc;
            }
            0x8015f5dc => { ins += 2;
                c.r[27] = c.r[27].wrapping_add(0x10);
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x8015f5e4;
            }
            0x8015f5e4 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa812));
                cr_cmps(&mut cr, c.xer, 0, c.r[26], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8015f2ac; continue; }
                pc = 0x8015f5f0;
            }
            0x8015f5f0 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x70); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x60); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                c.r[11] = c.r[1].wrapping_add(0x50);
                { let ea = c.r[1].wrapping_add(0x50); c.f[29] = c.lfd(ea); }
                c.lr = 0x8015f610; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c4810(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f610;
            }
            0x8015f610 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x84));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x80);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8015f270: bad pc {:#x}", pc),
        }
    }
}

/// s3dHandle (0x8015f620, 2092 bytes)
#[inline(never)]
pub fn f_8015f620(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8015f620) { return; }
        if c.prof { crate::prof::enter(c, 0x8015f620); p = true; }
    }
    f_8015f620_b(c, 0x8015f620);
    if p { crate::prof::exit(c, 0x8015f620); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015f620_at(c: &mut Cpu, pc: u32) { f_8015f620_b(c, pc) }
#[inline(never)]
fn f_8015f620_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8015f620;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8015f620, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8015f620 => { ins += 11;
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
                c.lr = 0x8015f64c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c47d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f64c;
            }
            0x8015f64c => { ins += 3;
                c.r[3] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa82c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015f664; continue; }
                pc = 0x8015f658;
            }
            0x8015f658 => { ins += 3;
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.stb(c.r[13].wrapping_add(0xffffa82c), c.r[0]);
                pc = 0x8015fe1c; continue;
                pc = 0x8015f664;
            }
            0x8015f664 => { ins += 8;
                c.r[0] = 0x0;
                c.r[3] = 0x3;
                c.stb(c.r[13].wrapping_add(0xffffa82c), c.r[3]);
                c.r[30] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa828));
                c.stb(c.r[13].wrapping_add(0xffffa812), c.r[0]);
                c.stb(c.r[13].wrapping_add(0xffffa811), c.r[0]);
                c.stb(c.r[13].wrapping_add(0xffffa810), c.r[0]);
                pc = 0x8015fc60; continue;
                pc = 0x8015f684;
            }
            0x8015f684 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[3] = c.lwz_t(ins, c.r[30]);
                c.r[0] = c.r[4].rotate_left(0) & 0x40000;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[29] = c.r[3] | c.r[3];
                if cr_bit(cr, 2) { pc = 0x8015f6ec; continue; }
                pc = 0x8015f698;
            }
            0x8015f698 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015f6a8; continue; }
                pc = 0x8015f6a0;
            }
            0x8015f6a0 => { ins += 2;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x4));
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                pc = 0x8015f6a8;
            }
            0x8015f6a8 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015f6c0; continue; }
                pc = 0x8015f6b4;
            }
            0x8015f6b4 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[30]);
                c.stw(c.r[3], c.r[0]);
                pc = 0x8015f6c8; continue;
                pc = 0x8015f6c0;
            }
            0x8015f6c0 => { ins += 2;
                c.r[0] = c.lwz_t(ins, c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffffa828), c.r[0]);
                pc = 0x8015f6c8;
            }
            0x8015f6c8 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x3c));
                c.r[0] = c.r[3].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015f6e4;
            }
            0x8015f6e4 => { ins += 1;
                c.lr = 0x8015f6e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c8fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f6e8;
            }
            0x8015f6e8 => { ins += 1;
                pc = 0x8015fc5c; continue;
                pc = 0x8015f6ec;
            }
            0x8015f6ec => { ins += 4;
                c.r[3] = 0x20000;
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[0] = c.r[4] & c.r[0];
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015f718; continue; }
                pc = 0x8015f6fc;
            }
            0x8015f6fc => { ins += 7;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[1].wrapping_add(0x18);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[6] = c.r[1].wrapping_add(0x14);
                c.r[7] = c.r[1].wrapping_add(0x10);
                c.r[8] = c.r[1].wrapping_add(0xc);
                c.lr = 0x8015f718; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015e374(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f718;
            }
            0x8015f718 => { ins += 3;
                c.r[7] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[7].rotate_left(0) & 0x80000;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8015fc18; continue; }
                pc = 0x8015f724;
            }
            0x8015f724 => { ins += 2;
                c.r[0] = c.r[7].rotate_left(0) & 0x20000;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015f868; continue; }
                pc = 0x8015f72c;
            }
            0x8015f72c => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff9dc8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x18); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[0], c.f[1]);
                if !cr_bit(cr, 2) { pc = 0x8015f75c; continue; }
                pc = 0x8015f73c;
            }
            0x8015f73c => { ins += 2;
                c.r[0] = c.r[7].rotate_left(0) & 0x4;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015f75c; continue; }
                pc = 0x8015f744;
            }
            0x8015f744 => { ins += 6;
                c.r[0] = c.r[7] | 0x80000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffdffff;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015f8a0; continue;
                pc = 0x8015f75c;
            }
            0x8015f75c => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9dc8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[0], c.f[1]);
                if !cr_bit(cr, 2) { pc = 0x8015f7c8; continue; }
                pc = 0x8015f768;
            }
            0x8015f768 => { ins += 2;
                c.r[0] = c.r[7].rotate_left(0) & 0x40;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015f7c8; continue; }
                pc = 0x8015f770;
            }
            0x8015f770 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[30]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015f784; continue; }
                pc = 0x8015f77c;
            }
            0x8015f77c => { ins += 2;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x4));
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                pc = 0x8015f784;
            }
            0x8015f784 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015f79c; continue; }
                pc = 0x8015f790;
            }
            0x8015f790 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[30]);
                c.stw(c.r[3], c.r[0]);
                pc = 0x8015f7a4; continue;
                pc = 0x8015f79c;
            }
            0x8015f79c => { ins += 2;
                c.r[0] = c.lwz_t(ins, c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffffa828), c.r[0]);
                pc = 0x8015f7a4;
            }
            0x8015f7a4 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x3c));
                c.r[0] = c.r[3].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015f7c0;
            }
            0x8015f7c0 => { ins += 1;
                c.lr = 0x8015f7c4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c8fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f7c4;
            }
            0x8015f7c4 => { ins += 1;
                pc = 0x8015fc5c; continue;
                pc = 0x8015f7c8;
            }
            0x8015f7c8 => { ins += 2;
                c.r[0] = c.r[7].rotate_left(0) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015f7f4; continue; }
                pc = 0x8015f7d0;
            }
            0x8015f7d0 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0x14); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[3] = c.r[30] | c.r[30];
                { let ea = c.r[1].wrapping_add(0x10); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0xc); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                c.lr = 0x8015f7e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015f124(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f7e8;
            }
            0x8015f7e8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015f7f0;
            }
            0x8015f7f0 => { ins += 1;
                pc = 0x8015f8a0; continue;
                pc = 0x8015f7f4;
            }
            0x8015f7f4 => { ins += 3;
                c.r[6] = c.lwz_t(ins, c.r[30].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015f80c; continue; }
                pc = 0x8015f800;
            }
            0x8015f800 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[6].wrapping_add(0x1c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x8015f844; continue; }
                pc = 0x8015f80c;
            }
            0x8015f80c => { ins += 5;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x0);
                c.r[3] = c.lhz_t(ins, c.r[30].wrapping_add(0x44));
                c.r[4] = 0x7f;
                c.r[5] = 0x40;
                if cr_bit(cr, 2) { pc = 0x8015f828; continue; }
                pc = 0x8015f820;
            }
            0x8015f820 => { ins += 2;
                c.r[6] = c.lbz_t(ins, c.r[6].wrapping_add(0x1c));
                pc = 0x8015f82c; continue;
                pc = 0x8015f828;
            }
            0x8015f828 => { ins += 1;
                c.r[6] = c.lbz_t(ins, c.r[30].wrapping_add(0x46));
                pc = 0x8015f82c;
            }
            0x8015f82c => { ins += 2;
                c.r[7] = c.r[7].rotate_left(28) & 0x1;
                c.lr = 0x8015f834; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c5e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f834;
            }
            0x8015f834 => { ins += 4;
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[30].wrapping_add(0x3c), c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if !cr_bit(cr, 2) { pc = 0x8015f8a0; continue; }
                pc = 0x8015f844;
            }
            0x8015f844 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015f850;
            }
            0x8015f850 => { ins += 6;
                c.r[0] = c.r[3] | 0x40000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffdffff;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015f8a0; continue;
                pc = 0x8015f868;
            }
            0x8015f868 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x3c));
                c.lr = 0x8015f870; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d880(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015f870;
            }
            0x8015f870 => { ins += 4;
                c.r[0] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[30].wrapping_add(0x3c), c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if !cr_bit(cr, 2) { pc = 0x8015f8a0; continue; }
                pc = 0x8015f880;
            }
            0x8015f880 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015f898; continue; }
                pc = 0x8015f88c;
            }
            0x8015f88c => { ins += 3;
                c.r[0] = c.r[3] | 0x20000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015f8a0; continue;
                pc = 0x8015f898;
            }
            0x8015f898 => { ins += 2;
                c.r[0] = c.r[3] | 0x40000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015f8a0;
            }
            0x8015f8a0 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x3c));
                c.r[0] = c.r[3].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x8015fbd8; continue; }
                pc = 0x8015f8b0;
            }
            0x8015f8b0 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015f9e0; continue; }
                pc = 0x8015f8bc;
            }
            0x8015f8bc => { ins += 8;
                c.r[5] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa812));
                c.r[3] = 0x80450000;
                c.r[4] = c.r[3].wrapping_add(0xffff8590);
                { let ea = c.r[1].wrapping_add(0x18); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[7] = 0x0;
                c.mtspr(9, c.r[5]);
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 1) { pc = 0x8015f8f8; continue; }
                pc = 0x8015f8dc;
            }
            0x8015f8dc => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x40));
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015f8f8; continue; }
                pc = 0x8015f8ec;
            }
            0x8015f8ec => { ins += 3;
                c.r[4] = c.r[4].wrapping_add(0x10);
                c.r[7] = c.r[7].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8015f8dc; continue; }
                pc = 0x8015f8f8;
            }
            0x8015f8f8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[7], c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x8015f934; continue; }
                pc = 0x8015f900;
            }
            0x8015f900 => { ins += 13;
                c.r[4] = 0x80450000;
                c.r[3] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa812));
                c.r[5] = c.r[7].rotate_left(4) & 0xfffffff0;
                c.r[6] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0xffff8590);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[4] = c.r[4].wrapping_add(c.r[5]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[6]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[6]);
                c.sth(c.r[4].wrapping_add(0xc), c.r[6]);
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x40));
                c.stw(c.r[4], c.r[3]);
                c.stb(c.r[13].wrapping_add(0xffffa812), c.r[0]);
                pc = 0x8015f934;
            }
            0x8015f934 => { ins += 11;
                c.r[3] = 0x80450000;
                c.r[4] = c.r[7].rotate_left(4) & 0xfffffff0;
                c.r[0] = c.r[3].wrapping_add(0xffff8590);
                c.r[5] = 0x0;
                c.r[4] = c.r[0].wrapping_add(c.r[4]);
                c.r[3] = c.lhz_t(ins, c.r[4].wrapping_add(0xc));
                c.r[7] = c.r[4].wrapping_add(0x8);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[6] = c.lwz_t(ins, c.r[4].wrapping_add(0x8));
                pc = 0x8015f974; continue;
                pc = 0x8015f960;
            }
            0x8015f960 => { ins += 3;
                { let ea = c.r[6].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[0], c.f[1]);
                if cr_bit(cr, 1) { pc = 0x8015f97c; continue; }
                pc = 0x8015f96c;
            }
            0x8015f96c => { ins += 2;
                c.r[5] = c.r[6] | c.r[6];
                c.r[6] = c.lwz_t(ins, c.r[6]);
                pc = 0x8015f974;
            }
            0x8015f974 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8015f960; continue; }
                pc = 0x8015f97c;
            }
            0x8015f97c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8015f9a0; continue; }
                pc = 0x8015f984;
            }
            0x8015f984 => { ins += 7;
                c.r[4] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa810));
                c.r[3] = 0x80450000;
                c.r[0] = c.r[3].wrapping_add(0xffff8990);
                c.r[3] = c.r[4].wrapping_mul(0xc);
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.stw(c.r[7], c.r[0]);
                pc = 0x8015f9b8; continue;
                pc = 0x8015f9a0;
            }
            0x8015f9a0 => { ins += 6;
                c.r[4] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa810));
                c.r[3] = 0x80450000;
                c.r[0] = c.r[3].wrapping_add(0xffff8990);
                c.r[3] = c.r[4].wrapping_mul(0xc);
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.stw(c.r[5], c.r[0]);
                pc = 0x8015f9b8;
            }
            0x8015f9b8 => { ins += 10;
                c.r[5] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa810));
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
            0x8015f9e0 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff9dc8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x18); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[0], c.f[2]);
                if !cr_bit(cr, 2) { pc = 0x8015fa30; continue; }
                pc = 0x8015f9f0;
            }
            0x8015f9f0 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0x4;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015fa30; continue; }
                pc = 0x8015f9fc;
            }
            0x8015f9fc => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x3c));
                c.lr = 0x8015fa04; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c8fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fa04;
            }
            0x8015fa04 => { ins += 5;
                c.r[0] = 0xffffffff;
                c.stw(c.r[30].wrapping_add(0x3c), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015fa24; continue; }
                pc = 0x8015fa18;
            }
            0x8015fa18 => { ins += 3;
                c.r[0] = c.r[3] | 0x80000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015fbd8; continue;
                pc = 0x8015fa24;
            }
            0x8015fa24 => { ins += 3;
                c.r[0] = c.r[3] | 0x40000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015fbd8; continue;
                pc = 0x8015fa30;
            }
            0x8015fa30 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                { let ea = c.r[1].wrapping_add(0x8); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                c.r[0] = c.r[0].rotate_left(0) & 0x100000;
                cr_rc(&mut cr, c.xer, c.r[0]);
                { let ea = c.r[1].wrapping_add(0xc); c.f[29] = c.lfs(ea); c.ps1[29] = c.f[29]; }
                { let ea = c.r[1].wrapping_add(0x14); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                c.r[31] = c.lwz_t(ins, c.r[30].wrapping_add(0x3c));
                if cr_bit(cr, 2) { pc = 0x8015fa8c; continue; }
                pc = 0x8015fa4c;
            }
            0x8015fa4c => { ins += 13;
                { let ea = c.r[30].wrapping_add(0x4c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[31] | c.r[31];
                { let ea = c.r[2].wrapping_add(0xffff9de8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = 0x7;
                { let v = s(c.f[0] * c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                c.r[5] = 0x7f;
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7f);
                if cr_bit(cr, 1) { pc = 0x8015fa84; continue; }
                pc = 0x8015fa80;
            }
            0x8015fa80 => { ins += 1;
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x8015fa84;
            }
            0x8015fa84 => { ins += 1;
                c.lr = 0x8015fa88; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c6b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fa88;
            }
            0x8015fa88 => { ins += 1;
                pc = 0x8015fac4; continue;
                pc = 0x8015fa8c;
            }
            0x8015fa8c => { ins += 9;
                { let ea = c.r[2].wrapping_add(0xffff9de8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = 0x7f;
                { let v = s(c.f[0] * c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7f);
                if cr_bit(cr, 1) { pc = 0x8015fab8; continue; }
                pc = 0x8015fab0;
            }
            0x8015fab0 => { ins += 2;
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                pc = 0x8015fab8;
            }
            0x8015fab8 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7;
                c.lr = 0x8015fac4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c6b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fac4;
            }
            0x8015fac4 => { ins += 11;
                { let ea = c.r[2].wrapping_add(0xffff9dec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = 0x7f;
                { let ea = c.r[2].wrapping_add(0xffff9dfc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[0] + c.f[30]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7f);
                if cr_bit(cr, 1) { pc = 0x8015faf8; continue; }
                pc = 0x8015faf0;
            }
            0x8015faf0 => { ins += 2;
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                pc = 0x8015faf8;
            }
            0x8015faf8 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0xa;
                c.lr = 0x8015fb04; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c6b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fb04;
            }
            0x8015fb04 => { ins += 11;
                { let ea = c.r[2].wrapping_add(0xffff9dec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = 0x7f;
                { let ea = c.r[2].wrapping_add(0xffff9dfc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[0] - c.f[29]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7f);
                if cr_bit(cr, 1) { pc = 0x8015fb38; continue; }
                pc = 0x8015fb30;
            }
            0x8015fb30 => { ins += 2;
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x3c));
                pc = 0x8015fb38;
            }
            0x8015fb38 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x83;
                c.lr = 0x8015fb44; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c6b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fb44;
            }
            0x8015fb44 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9e00); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] * c.f[31]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x8015fb50; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c46b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fb50;
            }
            0x8015fb50 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x3fff);
                c.r[5] = 0x3fff;
                if cr_bit(cr, 1) { pc = 0x8015fb60; continue; }
                pc = 0x8015fb5c;
            }
            0x8015fb5c => { ins += 1;
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8015fb60;
            }
            0x8015fb60 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x84;
                c.lr = 0x8015fb6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c794(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fb6c;
            }
            0x8015fb6c => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0xc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015fbd8; continue; }
                pc = 0x8015fb78;
            }
            0x8015fb78 => { ins += 3;
                c.r[28] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[27] = 0x0;
                pc = 0x8015fbc4; continue;
                pc = 0x8015fb84;
            }
            0x8015fb84 => { ins += 3;
                c.r[4] = c.lbz_t(ins, c.r[28]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x40);
                if cr_bit(cr, 0) { pc = 0x8015fba0; continue; }
                pc = 0x8015fb90;
            }
            0x8015fb90 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x80);
                if cr_bit(cr, 2) { pc = 0x8015fba0; continue; }
                pc = 0x8015fb98;
            }
            0x8015fb98 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x84);
                if !cr_bit(cr, 2) { pc = 0x8015fbb0; continue; }
                pc = 0x8015fba0;
            }
            0x8015fba0 => { ins += 3;
                c.r[5] = c.lhz_t(ins, c.r[28].wrapping_add(0x2));
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8015fbac; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c794(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fbac;
            }
            0x8015fbac => { ins += 1;
                pc = 0x8015fbbc; continue;
                pc = 0x8015fbb0;
            }
            0x8015fbb0 => { ins += 3;
                c.r[5] = c.lbz_t(ins, c.r[28].wrapping_add(0x2));
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8015fbbc; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c6b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fbbc;
            }
            0x8015fbbc => { ins += 2;
                c.r[28] = c.r[28].wrapping_add(0x4);
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x8015fbc4;
            }
            0x8015fbc4 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0xc));
                c.r[4] = c.r[27].rotate_left(0) & 0xff;
                c.r[0] = c.lbz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8015fb84; continue; }
                pc = 0x8015fbd8;
            }
            0x8015fbd8 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0x100000;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015fbe4;
            }
            0x8015fbe4 => { ins += 9;
                { let ea = c.r[30].wrapping_add(0x4c); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[2].wrapping_add(0xffff9e10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9dec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] + c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[30].wrapping_add(0x4c); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x4c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                { let (a, b) = (cr_bit(cr, 1), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if !cr_bit(cr, 2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015fc08;
            }
            0x8015fc08 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xffefffff;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015fc5c; continue;
                pc = 0x8015fc18;
            }
            0x8015fc18 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8015fc34; continue; }
                pc = 0x8015fc24;
            }
            0x8015fc24 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015fc28;
            }
            0x8015fc28 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015fc34;
            }
            0x8015fc34 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff9dc8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                if cr_bit(cr, 2) { pc = 0x8015fc5c; continue; }
                pc = 0x8015fc44;
            }
            0x8015fc44 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0xfff7ffff;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[0] = c.r[0] | 0x20000;
                c.stw(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x8015fc5c;
            }
            0x8015fc5c => { ins += 1;
                c.r[30] = c.r[29] | c.r[29];
                pc = 0x8015fc60;
            }
            0x8015fc60 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8015f684; continue; }
                pc = 0x8015fc68;
            }
            0x8015fc68 => { ins += 1;
                c.lr = 0x8015fc6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015f270(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fc6c;
            }
            0x8015fc6c => { ins += 1;
                c.lr = 0x8015fc70; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015dec0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fc70;
            }
            0x8015fc70 => { ins += 2;
                c.r[31] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa81c));
                pc = 0x8015fe14; continue;
                pc = 0x8015fc78;
            }
            0x8015fc78 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x28));
                c.r[0] = c.r[0].rotate_left(0) & 0x80000000;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8015fd50; continue; }
                pc = 0x8015fc84;
            }
            0x8015fc84 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x20));
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x8015fe10; continue; }
                pc = 0x8015fc94;
            }
            0x8015fc94 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x24));
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x8015fe10; continue; }
                pc = 0x8015fca4;
            }
            0x8015fca4 => { ins += 24;
                c.r[4] = c.lbz_t(ins, c.r[31].wrapping_add(0x1c));
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
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.stb(c.r[31].wrapping_add(0x35), c.r[4]);
                c.stb(c.r[31].wrapping_add(0x36), c.r[3]);
                c.stb(c.r[31].wrapping_add(0x34), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x28));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015fd24; continue; }
                pc = 0x8015fd04;
            }
            0x8015fd04 => { ins += 7;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x24));
                c.r[4] = c.r[31].wrapping_add(0x34);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1c));
                c.stb(c.r[31].wrapping_add(0x37), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x20));
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1c));
                c.lr = 0x8015fd20; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014dd98(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fd20;
            }
            0x8015fd20 => { ins += 1;
                pc = 0x8015fd40; continue;
                pc = 0x8015fd24;
            }
            0x8015fd24 => { ins += 7;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x20));
                c.r[4] = c.r[31].wrapping_add(0x34);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1c));
                c.stb(c.r[31].wrapping_add(0x37), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x24));
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1c));
                c.lr = 0x8015fd40; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014dd98(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fd40;
            }
            0x8015fd40 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x28));
                c.r[0] = c.r[0] | 0x80000000;
                c.stw(c.r[31].wrapping_add(0x28), c.r[0]);
                pc = 0x8015fe10; continue;
                pc = 0x8015fd50;
            }
            0x8015fd50 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x20));
                c.r[4] = c.lbz_t(ins, c.r[3].wrapping_add(0x1c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0xff);
                if cr_bit(cr, 2) { pc = 0x8015fd70; continue; }
                pc = 0x8015fd60;
            }
            0x8015fd60 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x24));
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if !cr_bit(cr, 2) { pc = 0x8015fdbc; continue; }
                pc = 0x8015fd70;
            }
            0x8015fd70 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0xff);
                if cr_bit(cr, 2) { pc = 0x8015fd84; continue; }
                pc = 0x8015fd78;
            }
            0x8015fd78 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x1d));
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8015fda0; continue; }
                pc = 0x8015fd84;
            }
            0x8015fd84 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x24));
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0xff);
                if cr_bit(cr, 2) { pc = 0x8015fdac; continue; }
                pc = 0x8015fd94;
            }
            0x8015fd94 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x1d));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8015fdac; continue; }
                pc = 0x8015fda0;
            }
            0x8015fda0 => { ins += 3;
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x1d));
                c.r[4] = c.r[31].wrapping_add(0x34);
                c.lr = 0x8015fdac; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014ddb8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fdac;
            }
            0x8015fdac => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x28));
                c.r[0] = c.r[0].rotate_left(0) & 0x7fffffff;
                c.stw(c.r[31].wrapping_add(0x28), c.r[0]);
                pc = 0x8015fe10; continue;
                pc = 0x8015fdbc;
            }
            0x8015fdbc => { ins += 21;
                c.r[4] = c.lbz_t(ins, c.r[31].wrapping_add(0x1c));
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
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.stb(c.r[31].wrapping_add(0x35), c.r[4]);
                c.stb(c.r[31].wrapping_add(0x36), c.r[3]);
                c.stb(c.r[31].wrapping_add(0x34), c.r[0]);
                pc = 0x8015fe10;
            }
            0x8015fe10 => { ins += 1;
                c.r[31] = c.lwz_t(ins, c.r[31]);
                pc = 0x8015fe14;
            }
            0x8015fe14 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8015fc78; continue; }
                pc = 0x8015fe1c;
            }
            0x8015fe1c => { ins += 8;
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x80); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x70); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                c.r[11] = c.r[1].wrapping_add(0x60);
                { let ea = c.r[1].wrapping_add(0x60); c.f[29] = c.lfd(ea); }
                c.lr = 0x8015fe3c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c4824(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015fe3c;
            }
            0x8015fe3c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x94));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x90);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8015f620: bad pc {:#x}", pc),
        }
    }
}

/// s3dInit (0x8015fe4c, 56 bytes)
#[inline(never)]
pub fn f_8015fe4c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8015fe4c) { return; }
        if c.prof { crate::prof::enter(c, 0x8015fe4c); p = true; }
    }
    f_8015fe4c_b(c, 0x8015fe4c);
    if p { crate::prof::exit(c, 0x8015fe4c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015fe4c_at(c: &mut Cpu, pc: u32) { f_8015fe4c_b(c, pc) }
#[inline(always)]
fn f_8015fe4c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8015fe4c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8015fe4c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8015fe4c => { ins += 14;
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
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8015fe4c: bad pc {:#x}", pc),
        }
    }
}

/// s3dExit (0x8015fe84, 4 bytes)
#[inline(never)]
pub fn f_8015fe84(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8015fe84) { return; }
        if c.prof { crate::prof::enter(c, 0x8015fe84); p = true; }
    }
    f_8015fe84_b(c, 0x8015fe84);
    if p { crate::prof::exit(c, 0x8015fe84); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015fe84_at(c: &mut Cpu, pc: u32) { f_8015fe84_b(c, pc) }
#[inline(always)]
fn f_8015fe84_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8015fe84;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8015fe84, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8015fe84 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8015fe84: bad pc {:#x}", pc),
        }
    }
}

/// fn_8015FE88 (0x8015fe88, 280 bytes)
#[inline(never)]
pub fn f_8015fe88(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8015fe88) { return; }
        if c.prof { crate::prof::enter(c, 0x8015fe88); p = true; }
    }
    f_8015fe88_b(c, 0x8015fe88);
    if p { crate::prof::exit(c, 0x8015fe88); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015fe88_at(c: &mut Cpu, pc: u32) { f_8015fe88_b(c, pc) }
#[inline(always)]
fn f_8015fe88_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8015fe88;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8015fe88, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8015fe88 => { ins += 13;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[9] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x40);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[8] | c.r[8];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[7] | c.r[7];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stb(c.r[13].wrapping_add(0xffffa6f8), c.r[9]);
                if cr_bit(cr, 1) { pc = 0x8015fecc; continue; }
                pc = 0x8015febc;
            }
            0x8015febc => { ins += 4;
                c.r[7] = 0x80430000;
                c.r[7] = c.r[7].wrapping_add(0x4c50);
                c.stb(c.r[7].wrapping_add(0x210), c.r[3]);
                pc = 0x8015fedc; continue;
                pc = 0x8015fecc;
            }
            0x8015fecc => { ins += 4;
                c.r[3] = 0x80430000;
                c.r[0] = 0x40;
                c.r[3] = c.r[3].wrapping_add(0x4c50);
                c.stb(c.r[3].wrapping_add(0x210), c.r[0]);
                pc = 0x8015fedc;
            }
            0x8015fedc => { ins += 3;
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                if cr_bit(cr, 1) { pc = 0x8015fef8; continue; }
                pc = 0x8015fee8;
            }
            0x8015fee8 => { ins += 4;
                c.r[3] = 0x80430000;
                c.r[3] = c.r[3].wrapping_add(0x4c50);
                c.stb(c.r[3].wrapping_add(0x213), c.r[6]);
                pc = 0x8015ff08; continue;
                pc = 0x8015fef8;
            }
            0x8015fef8 => { ins += 4;
                c.r[3] = 0x80430000;
                c.r[0] = 0x8;
                c.r[3] = c.r[3].wrapping_add(0x4c50);
                c.stb(c.r[3].wrapping_add(0x213), c.r[0]);
                pc = 0x8015ff08;
            }
            0x8015ff08 => { ins += 11;
                c.r[3] = 0x80430000;
                c.r[0] = 0x7d00;
                c.r[29] = c.r[3].wrapping_add(0x4c50);
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[6] = c.r[30] | c.r[30];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.stb(c.r[29].wrapping_add(0x211), c.r[4]);
                c.r[4] = c.lbz_t(ins, c.r[29].wrapping_add(0x210));
                c.stb(c.r[29].wrapping_add(0x212), c.r[5]);
                c.r[5] = c.lbz_t(ins, c.r[29].wrapping_add(0x213));
                c.lr = 0x8015ff34; { c.steps = st; c.instrs = ins; c.cr = cr; f_80162370(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015ff34;
            }
            0x8015ff34 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8015ff84; continue; }
                pc = 0x8015ff3c;
            }
            0x8015ff3c => { ins += 2;
                c.r[29] = c.lbz_t(ins, c.r[29].wrapping_add(0x210));
                c.lr = 0x8015ff44; { c.steps = st; c.instrs = ins; c.cr = cr; f_80159c48(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015ff44;
            }
            0x8015ff44 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.lr = 0x8015ff50; { c.steps = st; c.instrs = ins; c.cr = cr; f_801524e0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015ff50;
            }
            0x8015ff50 => { ins += 1;
                c.lr = 0x8015ff54; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014a094(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015ff54;
            }
            0x8015ff54 => { ins += 5;
                c.r[0] = 0x0;
                c.r[4] = c.r[29] | c.r[29];
                c.stb(c.r[13].wrapping_add(0xffffa730), c.r[0]);
                c.r[3] = 0x7d00;
                c.lr = 0x8015ff68; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d000(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015ff68;
            }
            0x8015ff68 => { ins += 1;
                c.lr = 0x8015ff6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014ddd8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015ff6c;
            }
            0x8015ff6c => { ins += 1;
                c.lr = 0x8015ff70; { c.steps = st; c.instrs = ins; c.cr = cr; f_80159494(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015ff70;
            }
            0x8015ff70 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8015ff78; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015fe4c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015ff78;
            }
            0x8015ff78 => { ins += 3;
                c.r[0] = 0x1;
                c.r[3] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa6f8), c.r[0]);
                pc = 0x8015ff84;
            }
            0x8015ff84 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8015fe88: bad pc {:#x}", pc),
        }
    }
}

/// sndQuit (0x8015ffa0, 52 bytes)
#[inline(never)]
pub fn f_8015ffa0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8015ffa0) { return; }
        if c.prof { crate::prof::enter(c, 0x8015ffa0); p = true; }
    }
    f_8015ffa0_b(c, 0x8015ffa0);
    if p { crate::prof::exit(c, 0x8015ffa0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015ffa0_at(c: &mut Cpu, pc: u32) { f_8015ffa0_b(c, pc) }
#[inline(always)]
fn f_8015ffa0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8015ffa0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8015ffa0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8015ffa0 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8015ffb0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80162428(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015ffb0;
            }
            0x8015ffb0 => { ins += 1;
                c.lr = 0x8015ffb4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801525c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015ffb4;
            }
            0x8015ffb4 => { ins += 1;
                c.lr = 0x8015ffb8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015fe84(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015ffb8;
            }
            0x8015ffb8 => { ins += 1;
                c.lr = 0x8015ffbc; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d574(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8015ffbc;
            }
            0x8015ffbc => { ins += 6;
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa6f8), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8015ffa0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8015FFD4 (0x8015ffd4, 8 bytes)
#[inline(never)]
pub fn f_8015ffd4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8015ffd4) { return; }
        if c.prof { crate::prof::enter(c, 0x8015ffd4); p = true; }
    }
    f_8015ffd4_b(c, 0x8015ffd4);
    if p { crate::prof::exit(c, 0x8015ffd4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015ffd4_at(c: &mut Cpu, pc: u32) { f_8015ffd4_b(c, pc) }
#[inline(always)]
fn f_8015ffd4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8015ffd4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8015ffd4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8015ffd4 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa6f8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8015ffd4: bad pc {:#x}", pc),
        }
    }
}

/// salApplyMatrix (0x8015ffdc, 172 bytes)
#[inline(never)]
pub fn f_8015ffdc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8015ffdc) { return; }
        if c.prof { crate::prof::enter(c, 0x8015ffdc); p = true; }
    }
    f_8015ffdc_b(c, 0x8015ffdc);
    if p { crate::prof::exit(c, 0x8015ffdc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8015ffdc_at(c: &mut Cpu, pc: u32) { f_8015ffdc_b(c, pc) }
#[inline(always)]
fn f_8015ffdc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8015ffdc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8015ffdc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8015ffdc => { ins += 43;
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
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8015ffdc: bad pc {:#x}", pc),
        }
    }
}

/// salNormalizeVector (0x80160088, 180 bytes)
#[inline(never)]
pub fn f_80160088(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80160088) { return; }
        if c.prof { crate::prof::enter(c, 0x80160088); p = true; }
    }
    f_80160088_b(c, 0x80160088);
    if p { crate::prof::exit(c, 0x80160088); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160088_at(c: &mut Cpu, pc: u32) { f_80160088_b(c, pc) }
#[inline(always)]
fn f_80160088_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80160088;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80160088, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80160088 => { ins += 12;
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
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                if !cr_bit(cr, 1) { pc = 0x80160110; continue; }
                pc = 0x801600b8;
            }
            0x801600b8 => { ins += 22;
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
            0x80160110 => { ins += 11;
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
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80160088: bad pc {:#x}", pc),
        }
    }
}

/// salCrossProduct (0x8016013c, 100 bytes)
#[inline(never)]
pub fn f_8016013c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016013c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016013c); p = true; }
    }
    f_8016013c_b(c, 0x8016013c);
    if p { crate::prof::exit(c, 0x8016013c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016013c_at(c: &mut Cpu, pc: u32) { f_8016013c_b(c, pc) }
#[inline(always)]
fn f_8016013c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016013c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016013c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016013c => { ins += 25;
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
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016013c: bad pc {:#x}", pc),
        }
    }
}

/// salInvertMatrix (0x801601a0, 508 bytes)
#[inline(never)]
pub fn f_801601a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801601a0) { return; }
        if c.prof { crate::prof::enter(c, 0x801601a0); p = true; }
    }
    f_801601a0_b(c, 0x801601a0);
    if p { crate::prof::exit(c, 0x801601a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801601a0_at(c: &mut Cpu, pc: u32) { f_801601a0_b(c, pc) }
#[inline(always)]
fn f_801601a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801601a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801601a0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801601a0 => { ins += 127;
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
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801601a0: bad pc {:#x}", pc),
        }
    }
}

/// inpSetGlobalMIDIDirtyFlag (0x8016039c, 36 bytes)
#[inline(never)]
pub fn f_8016039c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016039c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016039c); p = true; }
    }
    f_8016039c_b(c, 0x8016039c);
    if p { crate::prof::exit(c, 0x8016039c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016039c_at(c: &mut Cpu, pc: u32) { f_8016039c_b(c, pc) }
#[inline(always)]
fn f_8016039c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016039c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016039c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016039c => { ins += 9;
                c.r[6] = 0x80450000;
                c.r[4] = c.r[4].rotate_left(6) & 0x3fc0;
                c.r[0] = c.r[6].wrapping_add(0xffff9390);
                c.r[3] = c.r[3].rotate_left(2) & 0x3fc;
                c.r[4] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(c.r[3]));
                c.r[0] = c.r[0] | c.r[5];
                c.stw(c.r[4].wrapping_add(c.r[3]), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016039c: bad pc {:#x}", pc),
        }
    }
}

/// inpSetMidiCtrl (0x801603c0, 1544 bytes)
#[inline(never)]
pub fn f_801603c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801603c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801603c0); p = true; }
    }
    f_801603c0_b(c, 0x801603c0);
    if p { crate::prof::exit(c, 0x801603c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801603c0_at(c: &mut Cpu, pc: u32) { f_801603c0_b(c, pc) }
#[inline(never)]
fn f_801603c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801603c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801603c0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801603c0 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x801603d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c47d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801603d4;
            }
            0x801603d4 => { ins += 5;
                c.r[7] = c.r[4].rotate_left(0) & 0xff;
                c.r[8] = 0x80450000;
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], 0xff);
                c.r[31] = c.r[8].wrapping_add(0xffff9390);
                if cr_bit(cr, 2) { pc = 0x801609b0; continue; }
                pc = 0x801603e8;
            }
            0x801603e8 => { ins += 3;
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x801606e4; continue; }
                pc = 0x801603f4;
            }
            0x801603f4 => { ins += 3;
                c.r[8] = c.r[3].rotate_left(0) & 0xff;
                cr_cmps(&mut cr, c.xer, 0, c.r[8], 0x26);
                if cr_bit(cr, 2) { pc = 0x80160644; continue; }
                pc = 0x80160400;
            }
            0x80160400 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80160410; continue; }
                pc = 0x80160404;
            }
            0x80160404 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[8], 0x6);
                if cr_bit(cr, 2) { pc = 0x80160428; continue; }
                pc = 0x8016040c;
            }
            0x8016040c => { ins += 1;
                pc = 0x80160644; continue;
                pc = 0x80160410;
            }
            0x80160410 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[8], 0x61);
                if cr_bit(cr, 2) { pc = 0x80160594; continue; }
                pc = 0x80160418;
            }
            0x80160418 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80160644; continue; }
                pc = 0x8016041c;
            }
            0x8016041c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[8], 0x60);
                if !cr_bit(cr, 0) { pc = 0x801604e0; continue; }
                pc = 0x80160424;
            }
            0x80160424 => { ins += 1;
                pc = 0x80160644; continue;
                pc = 0x80160428;
            }
            0x80160428 => { ins += 10;
                c.r[10] = c.r[0].wrapping_mul(0x860);
                c.r[8] = c.r[31].wrapping_add(0x200);
                c.r[9] = c.r[7].wrapping_mul(0x86);
                c.r[8] = c.r[8].wrapping_add(c.r[10]);
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.lbz_t(ins, c.r[8].wrapping_add(0x64));
                c.r[8] = c.lbz_t(ins, c.r[8].wrapping_add(0x65));
                c.r[9] = (c.r[8].rotate_left(8) & 0xff00) | (c.r[9] & 0xffff00ff);
                c.r[8] = c.r[9].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[8]);
                if cr_bit(cr, 2) { pc = 0x80160454; continue; }
                pc = 0x80160450;
            }
            0x80160450 => { ins += 1;
                pc = 0x80160644; continue;
                pc = 0x80160454;
            }
            0x80160454 => { ins += 4;
                c.r[8] = c.r[6].rotate_left(0) & 0xff;
                c.r[10] = 0x18;
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], 0x18);
                if cr_bit(cr, 1) { pc = 0x80160468; continue; }
                pc = 0x80160464;
            }
            0x80160464 => { ins += 1;
                c.r[10] = c.r[8] | c.r[8];
                pc = 0x80160468;
            }
            0x80160468 => { ins += 12;
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
            0x80160498 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.r[7] = c.r[0].wrapping_add(c.r[26]);
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x122));
                cr_cmpu(&mut cr, c.xer, 0, c.r[9], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801604c8; continue; }
                pc = 0x801604ac;
            }
            0x801604ac => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x121));
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801604c8; continue; }
                pc = 0x801604b8;
            }
            0x801604b8 => { ins += 4;
                c.stb(c.r[7].wrapping_add(0x1d7), c.r[11]);
                c.r[0] = c.r[26].wrapping_add(0x1d6);
                c.r[7] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.stb(c.r[7].wrapping_add(c.r[0]), c.r[11]);
                pc = 0x801604c8;
            }
            0x801604c8 => { ins += 2;
                c.r[26] = c.r[26].wrapping_add(0x404);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x801604d0;
            }
            0x801604d0 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[10].wrapping_add(0x210));
                cr_cmpu(&mut cr, c.xer, 0, c.r[12], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80160498; continue; }
                pc = 0x801604dc;
            }
            0x801604dc => { ins += 1;
                pc = 0x80160644; continue;
                pc = 0x801604e0;
            }
            0x801604e0 => { ins += 10;
                c.r[10] = c.r[0].wrapping_mul(0x860);
                c.r[8] = c.r[31].wrapping_add(0x200);
                c.r[9] = c.r[7].wrapping_mul(0x86);
                c.r[8] = c.r[8].wrapping_add(c.r[10]);
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.lbz_t(ins, c.r[8].wrapping_add(0x64));
                c.r[8] = c.lbz_t(ins, c.r[8].wrapping_add(0x65));
                c.r[9] = (c.r[8].rotate_left(8) & 0xff00) | (c.r[9] & 0xffff00ff);
                c.r[8] = c.r[9].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[8]);
                if cr_bit(cr, 2) { pc = 0x8016050c; continue; }
                pc = 0x80160508;
            }
            0x80160508 => { ins += 1;
                pc = 0x80160644; continue;
                pc = 0x8016050c;
            }
            0x8016050c => { ins += 6;
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[8] = c.r[31].wrapping_add(0x4500);
                c.r[9] = c.r[0].wrapping_add(c.r[7]);
                c.r[11] = c.lbz_t(ins, c.r[8].wrapping_add(c.r[9]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[11], 0x0);
                if cr_bit(cr, 2) { pc = 0x8016052c; continue; }
                pc = 0x80160524;
            }
            0x80160524 => { ins += 2;
                c.r[0] = c.r[11].wrapping_add(0xffffffff);
                c.r[11] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x8016052c;
            }
            0x8016052c => { ins += 8;
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
            0x8016054c => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.r[7] = c.r[0].wrapping_add(c.r[26]);
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x122));
                cr_cmpu(&mut cr, c.xer, 0, c.r[9], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8016057c; continue; }
                pc = 0x80160560;
            }
            0x80160560 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x121));
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8016057c; continue; }
                pc = 0x8016056c;
            }
            0x8016056c => { ins += 4;
                c.stb(c.r[7].wrapping_add(0x1d7), c.r[11]);
                c.r[0] = c.r[26].wrapping_add(0x1d6);
                c.r[7] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.stb(c.r[7].wrapping_add(c.r[0]), c.r[11]);
                pc = 0x8016057c;
            }
            0x8016057c => { ins += 2;
                c.r[26] = c.r[26].wrapping_add(0x404);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x80160584;
            }
            0x80160584 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[10].wrapping_add(0x210));
                cr_cmpu(&mut cr, c.xer, 0, c.r[12], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8016054c; continue; }
                pc = 0x80160590;
            }
            0x80160590 => { ins += 1;
                pc = 0x80160644; continue;
                pc = 0x80160594;
            }
            0x80160594 => { ins += 10;
                c.r[10] = c.r[0].wrapping_mul(0x860);
                c.r[8] = c.r[31].wrapping_add(0x200);
                c.r[9] = c.r[7].wrapping_mul(0x86);
                c.r[8] = c.r[8].wrapping_add(c.r[10]);
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.lbz_t(ins, c.r[8].wrapping_add(0x64));
                c.r[8] = c.lbz_t(ins, c.r[8].wrapping_add(0x65));
                c.r[9] = (c.r[8].rotate_left(8) & 0xff00) | (c.r[9] & 0xffff00ff);
                c.r[8] = c.r[9].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[8]);
                if cr_bit(cr, 2) { pc = 0x801605c0; continue; }
                pc = 0x801605bc;
            }
            0x801605bc => { ins += 1;
                pc = 0x80160644; continue;
                pc = 0x801605c0;
            }
            0x801605c0 => { ins += 6;
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[8] = c.r[31].wrapping_add(0x4500);
                c.r[9] = c.r[0].wrapping_add(c.r[7]);
                c.r[11] = c.lbz_t(ins, c.r[8].wrapping_add(c.r[9]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[11], 0x18);
                if !cr_bit(cr, 0) { pc = 0x801605e0; continue; }
                pc = 0x801605d8;
            }
            0x801605d8 => { ins += 2;
                c.r[0] = c.r[11].wrapping_add(0x1);
                c.r[11] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x801605e0;
            }
            0x801605e0 => { ins += 8;
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
            0x80160600 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.r[7] = c.r[0].wrapping_add(c.r[26]);
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x122));
                cr_cmpu(&mut cr, c.xer, 0, c.r[9], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80160630; continue; }
                pc = 0x80160614;
            }
            0x80160614 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x121));
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80160630; continue; }
                pc = 0x80160620;
            }
            0x80160620 => { ins += 4;
                c.stb(c.r[7].wrapping_add(0x1d7), c.r[11]);
                c.r[0] = c.r[26].wrapping_add(0x1d6);
                c.r[7] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.stb(c.r[7].wrapping_add(c.r[0]), c.r[11]);
                pc = 0x80160630;
            }
            0x80160630 => { ins += 2;
                c.r[26] = c.r[26].wrapping_add(0x404);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x80160638;
            }
            0x80160638 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[10].wrapping_add(0x210));
                cr_cmpu(&mut cr, c.xer, 0, c.r[12], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80160600; continue; }
                pc = 0x80160644;
            }
            0x80160644 => { ins += 16;
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
            0x80160684 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.r[3] = c.r[0].wrapping_add(c.r[25]);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801606b4; continue; }
                pc = 0x80160698;
            }
            0x80160698 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801606b4; continue; }
                pc = 0x801606a4;
            }
            0x801606a4 => { ins += 4;
                c.stw(c.r[3].wrapping_add(0x214), c.r[27]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.r[3] = c.r[0].wrapping_add(c.r[25]);
                c.lr = 0x801606b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c07c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801606b4;
            }
            0x801606b4 => { ins += 2;
                c.r[25] = c.r[25].wrapping_add(0x404);
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x801606bc;
            }
            0x801606bc => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[26].wrapping_add(0x210));
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80160684; continue; }
                pc = 0x801606c8;
            }
            0x801606c8 => { ins += 7;
                c.r[4] = c.r[29].rotate_left(6) & 0xffffffc0;
                c.r[0] = c.r[30].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[5] = 0xff;
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[5]);
                pc = 0x801609b0; continue;
                pc = 0x801606e4;
            }
            0x801606e4 => { ins += 3;
                c.r[8] = c.r[3].rotate_left(0) & 0xff;
                cr_cmps(&mut cr, c.xer, 0, c.r[8], 0x26);
                if cr_bit(cr, 2) { pc = 0x80160934; continue; }
                pc = 0x801606f0;
            }
            0x801606f0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80160700; continue; }
                pc = 0x801606f4;
            }
            0x801606f4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[8], 0x6);
                if cr_bit(cr, 2) { pc = 0x80160718; continue; }
                pc = 0x801606fc;
            }
            0x801606fc => { ins += 1;
                pc = 0x80160934; continue;
                pc = 0x80160700;
            }
            0x80160700 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[8], 0x61);
                if cr_bit(cr, 2) { pc = 0x80160884; continue; }
                pc = 0x80160708;
            }
            0x80160708 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80160934; continue; }
                pc = 0x8016070c;
            }
            0x8016070c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[8], 0x60);
                if !cr_bit(cr, 0) { pc = 0x801607d0; continue; }
                pc = 0x80160714;
            }
            0x80160714 => { ins += 1;
                pc = 0x80160934; continue;
                pc = 0x80160718;
            }
            0x80160718 => { ins += 10;
                c.r[10] = c.r[0].wrapping_mul(0x860);
                c.r[8] = c.r[31].wrapping_add(0x200);
                c.r[9] = c.r[7].wrapping_mul(0x86);
                c.r[8] = c.r[8].wrapping_add(c.r[10]);
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.lbz_t(ins, c.r[8].wrapping_add(0x64));
                c.r[8] = c.lbz_t(ins, c.r[8].wrapping_add(0x65));
                c.r[9] = (c.r[8].rotate_left(8) & 0xff00) | (c.r[9] & 0xffff00ff);
                c.r[8] = c.r[9].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[8]);
                if cr_bit(cr, 2) { pc = 0x80160744; continue; }
                pc = 0x80160740;
            }
            0x80160740 => { ins += 1;
                pc = 0x80160934; continue;
                pc = 0x80160744;
            }
            0x80160744 => { ins += 4;
                c.r[8] = c.r[6].rotate_left(0) & 0xff;
                c.r[10] = 0x18;
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], 0x18);
                if cr_bit(cr, 1) { pc = 0x80160758; continue; }
                pc = 0x80160754;
            }
            0x80160754 => { ins += 1;
                c.r[10] = c.r[8] | c.r[8];
                pc = 0x80160758;
            }
            0x80160758 => { ins += 12;
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
            0x80160788 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.r[7] = c.r[0].wrapping_add(c.r[25]);
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x122));
                cr_cmpu(&mut cr, c.xer, 0, c.r[9], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801607b8; continue; }
                pc = 0x8016079c;
            }
            0x8016079c => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x121));
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801607b8; continue; }
                pc = 0x801607a8;
            }
            0x801607a8 => { ins += 4;
                c.stb(c.r[7].wrapping_add(0x1d7), c.r[11]);
                c.r[0] = c.r[25].wrapping_add(0x1d6);
                c.r[7] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.stb(c.r[7].wrapping_add(c.r[0]), c.r[11]);
                pc = 0x801607b8;
            }
            0x801607b8 => { ins += 2;
                c.r[25] = c.r[25].wrapping_add(0x404);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x801607c0;
            }
            0x801607c0 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[10].wrapping_add(0x210));
                cr_cmpu(&mut cr, c.xer, 0, c.r[12], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80160788; continue; }
                pc = 0x801607cc;
            }
            0x801607cc => { ins += 1;
                pc = 0x80160934; continue;
                pc = 0x801607d0;
            }
            0x801607d0 => { ins += 10;
                c.r[10] = c.r[0].wrapping_mul(0x860);
                c.r[8] = c.r[31].wrapping_add(0x200);
                c.r[9] = c.r[7].wrapping_mul(0x86);
                c.r[8] = c.r[8].wrapping_add(c.r[10]);
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.lbz_t(ins, c.r[8].wrapping_add(0x64));
                c.r[8] = c.lbz_t(ins, c.r[8].wrapping_add(0x65));
                c.r[9] = (c.r[8].rotate_left(8) & 0xff00) | (c.r[9] & 0xffff00ff);
                c.r[8] = c.r[9].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[8]);
                if cr_bit(cr, 2) { pc = 0x801607fc; continue; }
                pc = 0x801607f8;
            }
            0x801607f8 => { ins += 1;
                pc = 0x80160934; continue;
                pc = 0x801607fc;
            }
            0x801607fc => { ins += 6;
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[8] = c.r[31].wrapping_add(0x4500);
                c.r[9] = c.r[0].wrapping_add(c.r[7]);
                c.r[11] = c.lbz_t(ins, c.r[8].wrapping_add(c.r[9]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[11], 0x0);
                if cr_bit(cr, 2) { pc = 0x8016081c; continue; }
                pc = 0x80160814;
            }
            0x80160814 => { ins += 2;
                c.r[0] = c.r[11].wrapping_add(0xffffffff);
                c.r[11] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x8016081c;
            }
            0x8016081c => { ins += 8;
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
            0x8016083c => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.r[7] = c.r[0].wrapping_add(c.r[25]);
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x122));
                cr_cmpu(&mut cr, c.xer, 0, c.r[9], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8016086c; continue; }
                pc = 0x80160850;
            }
            0x80160850 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x121));
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8016086c; continue; }
                pc = 0x8016085c;
            }
            0x8016085c => { ins += 4;
                c.stb(c.r[7].wrapping_add(0x1d7), c.r[11]);
                c.r[0] = c.r[25].wrapping_add(0x1d6);
                c.r[7] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.stb(c.r[7].wrapping_add(c.r[0]), c.r[11]);
                pc = 0x8016086c;
            }
            0x8016086c => { ins += 2;
                c.r[25] = c.r[25].wrapping_add(0x404);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x80160874;
            }
            0x80160874 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[10].wrapping_add(0x210));
                cr_cmpu(&mut cr, c.xer, 0, c.r[12], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8016083c; continue; }
                pc = 0x80160880;
            }
            0x80160880 => { ins += 1;
                pc = 0x80160934; continue;
                pc = 0x80160884;
            }
            0x80160884 => { ins += 10;
                c.r[10] = c.r[0].wrapping_mul(0x860);
                c.r[8] = c.r[31].wrapping_add(0x200);
                c.r[9] = c.r[7].wrapping_mul(0x86);
                c.r[8] = c.r[8].wrapping_add(c.r[10]);
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.lbz_t(ins, c.r[8].wrapping_add(0x64));
                c.r[8] = c.lbz_t(ins, c.r[8].wrapping_add(0x65));
                c.r[9] = (c.r[8].rotate_left(8) & 0xff00) | (c.r[9] & 0xffff00ff);
                c.r[8] = c.r[9].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[8]);
                if cr_bit(cr, 2) { pc = 0x801608b0; continue; }
                pc = 0x801608ac;
            }
            0x801608ac => { ins += 1;
                pc = 0x80160934; continue;
                pc = 0x801608b0;
            }
            0x801608b0 => { ins += 6;
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[8] = c.r[31].wrapping_add(0x4500);
                c.r[9] = c.r[0].wrapping_add(c.r[7]);
                c.r[11] = c.lbz_t(ins, c.r[8].wrapping_add(c.r[9]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[11], 0x18);
                if !cr_bit(cr, 0) { pc = 0x801608d0; continue; }
                pc = 0x801608c8;
            }
            0x801608c8 => { ins += 2;
                c.r[0] = c.r[11].wrapping_add(0x1);
                c.r[11] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x801608d0;
            }
            0x801608d0 => { ins += 8;
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
            0x801608f0 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.r[7] = c.r[0].wrapping_add(c.r[25]);
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x122));
                cr_cmpu(&mut cr, c.xer, 0, c.r[9], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80160920; continue; }
                pc = 0x80160904;
            }
            0x80160904 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x121));
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80160920; continue; }
                pc = 0x80160910;
            }
            0x80160910 => { ins += 4;
                c.stb(c.r[7].wrapping_add(0x1d7), c.r[11]);
                c.r[0] = c.r[25].wrapping_add(0x1d6);
                c.r[7] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.stb(c.r[7].wrapping_add(c.r[0]), c.r[11]);
                pc = 0x80160920;
            }
            0x80160920 => { ins += 2;
                c.r[25] = c.r[25].wrapping_add(0x404);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x80160928;
            }
            0x80160928 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[10].wrapping_add(0x210));
                cr_cmpu(&mut cr, c.xer, 0, c.r[12], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801608f0; continue; }
                pc = 0x80160934;
            }
            0x80160934 => { ins += 14;
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
            0x8016096c => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.r[3] = c.r[0].wrapping_add(c.r[25]);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8016099c; continue; }
                pc = 0x80160980;
            }
            0x80160980 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8016099c; continue; }
                pc = 0x8016098c;
            }
            0x8016098c => { ins += 4;
                c.stw(c.r[3].wrapping_add(0x214), c.r[28]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa728));
                c.r[3] = c.r[0].wrapping_add(c.r[25]);
                c.lr = 0x8016099c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c07c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016099c;
            }
            0x8016099c => { ins += 2;
                c.r[25] = c.r[25].wrapping_add(0x404);
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801609a4;
            }
            0x801609a4 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x210));
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8016096c; continue; }
                pc = 0x801609b0;
            }
            0x801609b0 => { ins += 2;
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x801609b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c481c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801609b8;
            }
            0x801609b8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801603c0: bad pc {:#x}", pc),
        }
    }
}

/// inpSetMidiCtrl14 (0x801609c8, 292 bytes)
#[inline(never)]
pub fn f_801609c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801609c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801609c8); p = true; }
    }
    f_801609c8_b(c, 0x801609c8);
    if p { crate::prof::exit(c, 0x801609c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801609c8_at(c: &mut Cpu, pc: u32) { f_801609c8_b(c, pc) }
#[inline(always)]
fn f_801609c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801609c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801609c8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801609c8 => { ins += 12;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[4] | c.r[4];
                if cr_bit(cr, 2) { pc = 0x80160acc; continue; }
                pc = 0x801609f8;
            }
            0x801609f8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x40);
                if !cr_bit(cr, 0) { pc = 0x80160a34; continue; }
                pc = 0x80160a04;
            }
            0x80160a04 => { ins += 5;
                c.r[30] = c.r[3].rotate_left(0) & 0x1f;
                c.r[31] = c.r[6].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[6].rotate_left(25) & 0xff;
                c.lr = 0x80160a18; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80160a18;
            }
            0x80160a18 => { ins += 6;
                c.r[0] = c.r[30].wrapping_add(0x20);
                c.r[6] = c.r[31].rotate_left(0) & 0x7f;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x80160a30; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80160a30;
            }
            0x80160a30 => { ins += 1;
                pc = 0x80160acc; continue;
                pc = 0x80160a34;
            }
            0x80160a34 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x80);
                if cr_bit(cr, 2) { pc = 0x80160a44; continue; }
                pc = 0x80160a3c;
            }
            0x80160a3c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x81);
                if !cr_bit(cr, 2) { pc = 0x80160a7c; continue; }
                pc = 0x80160a44;
            }
            0x80160a44 => { ins += 7;
                c.r[31] = c.r[3].rotate_left(0) & 0xfe;
                c.r[30] = c.r[6].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[6].rotate_left(25) & 0xff;
                c.lr = 0x80160a60; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80160a60;
            }
            0x80160a60 => { ins += 6;
                c.r[0] = c.r[31].wrapping_add(0x1);
                c.r[6] = c.r[30].rotate_left(0) & 0x7f;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x80160a78; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80160a78;
            }
            0x80160a78 => { ins += 1;
                pc = 0x80160acc; continue;
                pc = 0x80160a7c;
            }
            0x80160a7c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x84);
                if cr_bit(cr, 2) { pc = 0x80160a8c; continue; }
                pc = 0x80160a84;
            }
            0x80160a84 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x85);
                if !cr_bit(cr, 2) { pc = 0x80160ac4; continue; }
                pc = 0x80160a8c;
            }
            0x80160a8c => { ins += 7;
                c.r[31] = c.r[3].rotate_left(0) & 0xfe;
                c.r[30] = c.r[6].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[6].rotate_left(25) & 0xff;
                c.lr = 0x80160aa8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80160aa8;
            }
            0x80160aa8 => { ins += 6;
                c.r[0] = c.r[31].wrapping_add(0x1);
                c.r[6] = c.r[30].rotate_left(0) & 0x7f;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x80160ac0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80160ac0;
            }
            0x80160ac0 => { ins += 1;
                pc = 0x80160acc; continue;
                pc = 0x80160ac4;
            }
            0x80160ac4 => { ins += 2;
                c.r[6] = c.r[6].rotate_left(25) & 0xff;
                c.lr = 0x80160acc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80160acc;
            }
            0x80160acc => { ins += 8;
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
            _ => panic!("f_801609c8: bad pc {:#x}", pc),
        }
    }
}

/// inpResetMidiCtrl (0x80160aec, 240 bytes)
#[inline(never)]
pub fn f_80160aec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80160aec) { return; }
        if c.prof { crate::prof::enter(c, 0x80160aec); p = true; }
    }
    f_80160aec_b(c, 0x80160aec);
    if p { crate::prof::exit(c, 0x80160aec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160aec_at(c: &mut Cpu, pc: u32) { f_80160aec_b(c, pc) }
#[inline(always)]
fn f_80160aec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80160aec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80160aec, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80160aec => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x80270000;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = c.r[6].wrapping_add(0x33c0);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                if cr_bit(cr, 2) { pc = 0x80160b20; continue; }
                pc = 0x80160b18;
            }
            0x80160b18 => { ins += 2;
                c.r[3] = 0x80270000;
                c.r[6] = c.r[3].wrapping_add(0x3338);
                pc = 0x80160b20;
            }
            0x80160b20 => { ins += 3;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0xff);
                if cr_bit(cr, 2) { pc = 0x80160b4c; continue; }
                pc = 0x80160b2c;
            }
            0x80160b2c => { ins += 8;
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
            0x80160b4c => { ins += 5;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.r[3] = 0x80450000;
                c.r[4] = c.r[0].wrapping_mul(0x86);
                c.r[0] = c.r[3].wrapping_add(0xffffd910);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80160b60;
            }
            0x80160b60 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if cr_bit(cr, 2) { pc = 0x80160b78; continue; }
                pc = 0x80160b68;
            }
            0x80160b68 => { ins += 3;
                c.r[4] = c.r[6] | c.r[6];
                c.r[5] = 0x86;
                c.lr = 0x80160b74; { c.steps = st; c.instrs = ins; c.cr = cr; f_800054f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80160b74;
            }
            0x80160b74 => { ins += 1;
                pc = 0x80160bb4; continue;
                pc = 0x80160b78;
            }
            0x80160b78 => { ins += 3;
                c.r[0] = 0x43;
                c.r[4] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x80160b84;
            }
            0x80160b84 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[6]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x80160b94; continue; }
                pc = 0x80160b90;
            }
            0x80160b90 => { ins += 1;
                c.stb(c.r[3], c.r[0]);
                pc = 0x80160b94;
            }
            0x80160b94 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[6].wrapping_add(0x1));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x80160ba4; continue; }
                pc = 0x80160ba0;
            }
            0x80160ba0 => { ins += 1;
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                pc = 0x80160ba4;
            }
            0x80160ba4 => { ins += 4;
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[3] = c.r[3].wrapping_add(0x2);
                c.r[6] = c.r[6].wrapping_add(0x2);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80160b84; continue; }
                pc = 0x80160bb4;
            }
            0x80160bb4 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0xff;
                c.lr = 0x80160bc4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801610bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80160bc4;
            }
            0x80160bc4 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80160aec: bad pc {:#x}", pc),
        }
    }
}

/// inpGetMidiCtrl (0x80160bdc, 708 bytes)
#[inline(never)]
pub fn f_80160bdc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80160bdc) { return; }
        if c.prof { crate::prof::enter(c, 0x80160bdc); p = true; }
    }
    f_80160bdc_b(c, 0x80160bdc);
    if p { crate::prof::exit(c, 0x80160bdc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160bdc_at(c: &mut Cpu, pc: u32) { f_80160bdc_b(c, pc) }
#[inline(never)]
fn f_80160bdc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80160bdc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80160bdc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80160bdc => { ins += 3;
                c.r[8] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], 0xff);
                if cr_bit(cr, 2) { pc = 0x80160e98; continue; }
                pc = 0x80160be8;
            }
            0x80160be8 => { ins += 3;
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x80160d60; continue; }
                pc = 0x80160bf4;
            }
            0x80160bf4 => { ins += 3;
                c.r[7] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], 0x40);
                if !cr_bit(cr, 0) { pc = 0x80160c38; continue; }
                pc = 0x80160c00;
            }
            0x80160c00 => { ins += 14;
                c.r[5] = c.r[0].wrapping_mul(0x860);
                c.r[4] = 0x80450000;
                c.r[6] = c.r[3].rotate_left(0) & 0x1f;
                c.r[0] = c.r[4].wrapping_add(0xffff9590);
                c.r[3] = c.r[8].wrapping_mul(0x86);
                c.r[0] = c.r[0].wrapping_add(c.r[5]);
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                c.r[3] = c.r[4].wrapping_add(c.r[6]);
                c.r[4] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[6]));
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x20));
                c.r[3] = c.r[4].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160c38;
            }
            0x80160c38 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], 0x46);
                if !cr_bit(cr, 0) { pc = 0x80160c78; continue; }
                pc = 0x80160c40;
            }
            0x80160c40 => { ins += 14;
                c.r[6] = c.r[0].wrapping_mul(0x860);
                c.r[4] = 0x80450000;
                c.r[3] = 0x40;
                c.r[5] = c.r[4].wrapping_add(0xffff9590);
                c.r[4] = c.r[8].wrapping_mul(0x86);
                c.r[0] = 0x3fff;
                c.r[5] = c.r[5].wrapping_add(c.r[6]);
                c.r[4] = c.r[5].wrapping_add(c.r[4]);
                c.r[4] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[7]));
                c.r[3] = c.addc(!c.r[3], c.r[4], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[3], ci); }
                c.r[0] = c.r[0] & !c.r[3];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160c78;
            }
            0x80160c78 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], 0x60);
                if cr_bit(cr, 0) { pc = 0x80160c90; continue; }
                pc = 0x80160c80;
            }
            0x80160c80 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], 0x66);
                if !cr_bit(cr, 0) { pc = 0x80160c90; continue; }
                pc = 0x80160c88;
            }
            0x80160c88 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160c90;
            }
            0x80160c90 => { ins += 3;
                c.r[6] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x80);
                if cr_bit(cr, 2) { pc = 0x80160ca4; continue; }
                pc = 0x80160c9c;
            }
            0x80160c9c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x81);
                if !cr_bit(cr, 2) { pc = 0x80160ce4; continue; }
                pc = 0x80160ca4;
            }
            0x80160ca4 => { ins += 16;
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
                c.r[4] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[7]));
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                c.r[3] = c.r[4].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160ce4;
            }
            0x80160ce4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x84);
                if cr_bit(cr, 2) { pc = 0x80160cf4; continue; }
                pc = 0x80160cec;
            }
            0x80160cec => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x85);
                if !cr_bit(cr, 2) { pc = 0x80160d34; continue; }
                pc = 0x80160cf4;
            }
            0x80160cf4 => { ins += 16;
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
                c.r[4] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[7]));
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                c.r[3] = c.r[4].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160d34;
            }
            0x80160d34 => { ins += 11;
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[4] = c.r[3].wrapping_mul(0x860);
                c.r[3] = 0x80450000;
                c.r[3] = c.r[3].wrapping_add(0xffff9590);
                c.r[0] = c.r[0].wrapping_mul(0x86);
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[6]));
                c.r[3] = c.r[0].rotate_left(7) & 0xff80;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160d60;
            }
            0x80160d60 => { ins += 3;
                c.r[6] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x40);
                if !cr_bit(cr, 0) { pc = 0x80160d9c; continue; }
                pc = 0x80160d6c;
            }
            0x80160d6c => { ins += 12;
                c.r[6] = c.r[8].wrapping_mul(0x86);
                c.r[4] = 0x80450000;
                c.r[5] = c.r[3].rotate_left(0) & 0x1f;
                c.r[0] = c.r[4].wrapping_add(0xffffd910);
                c.r[4] = c.r[0].wrapping_add(c.r[6]);
                c.r[3] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[5]));
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x20));
                c.r[3] = c.r[4].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160d9c;
            }
            0x80160d9c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x46);
                if !cr_bit(cr, 0) { pc = 0x80160dd4; continue; }
                pc = 0x80160da4;
            }
            0x80160da4 => { ins += 12;
                c.r[5] = c.r[8].wrapping_mul(0x86);
                c.r[4] = 0x80450000;
                c.r[3] = 0x40;
                c.r[0] = c.r[4].wrapping_add(0xffffd910);
                c.r[4] = c.r[0].wrapping_add(c.r[5]);
                c.r[0] = 0x3fff;
                c.r[4] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[6]));
                c.r[3] = c.addc(!c.r[3], c.r[4], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[3], ci); }
                c.r[0] = c.r[0] & !c.r[3];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160dd4;
            }
            0x80160dd4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x60);
                if cr_bit(cr, 0) { pc = 0x80160dec; continue; }
                pc = 0x80160ddc;
            }
            0x80160ddc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x66);
                if !cr_bit(cr, 0) { pc = 0x80160dec; continue; }
                pc = 0x80160de4;
            }
            0x80160de4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160dec;
            }
            0x80160dec => { ins += 3;
                c.r[5] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x80);
                if cr_bit(cr, 2) { pc = 0x80160e00; continue; }
                pc = 0x80160df8;
            }
            0x80160df8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x81);
                if !cr_bit(cr, 2) { pc = 0x80160e34; continue; }
                pc = 0x80160e00;
            }
            0x80160e00 => { ins += 13;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[4] = 0x80450000;
                c.r[6] = c.r[0].wrapping_mul(0x86);
                c.r[5] = c.r[3].rotate_left(0) & 0xfe;
                c.r[0] = c.r[4].wrapping_add(0xffffd910);
                c.r[4] = c.r[0].wrapping_add(c.r[6]);
                c.r[3] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[5]));
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                c.r[3] = c.r[4].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160e34;
            }
            0x80160e34 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x84);
                if cr_bit(cr, 2) { pc = 0x80160e44; continue; }
                pc = 0x80160e3c;
            }
            0x80160e3c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x85);
                if !cr_bit(cr, 2) { pc = 0x80160e78; continue; }
                pc = 0x80160e44;
            }
            0x80160e44 => { ins += 13;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[4] = 0x80450000;
                c.r[6] = c.r[0].wrapping_mul(0x86);
                c.r[5] = c.r[3].rotate_left(0) & 0xfe;
                c.r[0] = c.r[4].wrapping_add(0xffffd910);
                c.r[4] = c.r[0].wrapping_add(c.r[6]);
                c.r[3] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[5]));
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                c.r[3] = c.r[4].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160e78;
            }
            0x80160e78 => { ins += 8;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[3] = 0x80450000;
                c.r[4] = c.r[0].wrapping_mul(0x86);
                c.r[0] = c.r[3].wrapping_add(0xffffd910);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[5]));
                c.r[3] = c.r[0].rotate_left(7) & 0xff80;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160e98;
            }
            0x80160e98 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80160bdc: bad pc {:#x}", pc),
        }
    }
}

/// inpGetChannelDefaults (0x80160ea0, 52 bytes)
#[inline(never)]
pub fn f_80160ea0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80160ea0) { return; }
        if c.prof { crate::prof::enter(c, 0x80160ea0); p = true; }
    }
    f_80160ea0_b(c, 0x80160ea0);
    if p { crate::prof::exit(c, 0x80160ea0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160ea0_at(c: &mut Cpu, pc: u32) { f_80160ea0_b(c, pc) }
#[inline(always)]
fn f_80160ea0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80160ea0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80160ea0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80160ea0 => { ins += 9;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[5] = 0x80450000;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                c.r[6] = c.r[4].rotate_left(4) & 0xff0;
                c.r[0] = c.r[5].wrapping_add(0xffffd890);
                c.r[4] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.r[0].wrapping_add(c.r[6]);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if !cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80160ec4;
            }
            0x80160ec4 => { ins += 4;
                c.r[3] = 0x80450000;
                c.r[0] = c.r[3].wrapping_add(0xfffffa90);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80160ea0: bad pc {:#x}", pc),
        }
    }
}

/// inpResetChannelDefaults (0x80160ed4, 60 bytes)
#[inline(never)]
pub fn f_80160ed4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80160ed4) { return; }
        if c.prof { crate::prof::enter(c, 0x80160ed4); p = true; }
    }
    f_80160ed4_b(c, 0x80160ed4);
    if p { crate::prof::exit(c, 0x80160ed4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160ed4_at(c: &mut Cpu, pc: u32) { f_80160ed4_b(c, pc) }
#[inline(always)]
fn f_80160ed4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80160ed4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80160ed4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80160ed4 => { ins += 8;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[5] = 0x80450000;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                c.r[6] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.r[5].wrapping_add(0xfffffa90);
                c.r[5] = 0x2;
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                if cr_bit(cr, 2) { pc = 0x80160f08; continue; }
                pc = 0x80160ef4;
            }
            0x80160ef4 => { ins += 5;
                c.r[3] = 0x80450000;
                c.r[4] = c.r[4].rotate_left(4) & 0xff0;
                c.r[0] = c.r[3].wrapping_add(0xffffd890);
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                pc = 0x80160f08;
            }
            0x80160f08 => { ins += 2;
                c.stb(c.r[3], c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80160ed4: bad pc {:#x}", pc),
        }
    }
}

/// inpAddCtrl (0x80160f10, 152 bytes)
#[inline(never)]
pub fn f_80160f10(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80160f10) { return; }
        if c.prof { crate::prof::enter(c, 0x80160f10); p = true; }
    }
    f_80160f10_b(c, 0x80160f10);
    if p { crate::prof::exit(c, 0x80160f10); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160f10_at(c: &mut Cpu, pc: u32) { f_80160f10_b(c, pc) }
#[inline(always)]
fn f_80160f10_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80160f10;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80160f10, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80160f10 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x20);
                c.lr = 0x80160f24; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c47d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80160f24;
            }
            0x80160f24 => { ins += 6;
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[29] = c.r[6] | c.r[6];
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[5] | c.r[5];
                c.r[30] = c.r[7] | c.r[7];
                if !cr_bit(cr, 2) { pc = 0x80160f44; continue; }
                pc = 0x80160f3c;
            }
            0x80160f3c => { ins += 2;
                c.r[0] = 0x0;
                c.stb(c.r[27].wrapping_add(0x22), c.r[0]);
                pc = 0x80160f44;
            }
            0x80160f44 => { ins += 3;
                c.r[31] = c.lbz_t(ins, c.r[27].wrapping_add(0x22));
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x4);
                if !cr_bit(cr, 0) { pc = 0x80160f90; continue; }
                pc = 0x80160f50;
            }
            0x80160f50 => { ins += 4;
                c.r[0] = c.r[31].wrapping_add(0x1);
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                c.stb(c.r[27].wrapping_add(0x22), c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80160f6c; continue; }
                pc = 0x80160f60;
            }
            0x80160f60 => { ins += 2;
                c.r[3] = c.r[4] | c.r[4];
                c.lr = 0x80160f68; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161d20(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80160f68;
            }
            0x80160f68 => { ins += 1;
                c.r[4] = c.r[3] | c.r[3];
                pc = 0x80160f6c;
            }
            0x80160f6c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x80160f7c; continue; }
                pc = 0x80160f74;
            }
            0x80160f74 => { ins += 2;
                c.r[0] = c.r[29] | 0x10;
                c.r[29] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x80160f7c;
            }
            0x80160f7c => { ins += 5;
                c.r[0] = c.r[31].rotate_left(3) & 0x7f8;
                c.stb(c.r[27].wrapping_add(c.r[0]), c.r[4]);
                c.r[3] = c.r[27].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x1), c.r[29]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[28]);
                pc = 0x80160f90;
            }
            0x80160f90 => { ins += 2;
                c.r[11] = c.r[1].wrapping_add(0x20);
                c.lr = 0x80160f98; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c4824(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80160f98;
            }
            0x80160f98 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80160f10: bad pc {:#x}", pc),
        }
    }
}

/// inpFXCopyCtrl (0x80160fa8, 276 bytes)
#[inline(never)]
pub fn f_80160fa8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80160fa8) { return; }
        if c.prof { crate::prof::enter(c, 0x80160fa8); p = true; }
    }
    f_80160fa8_b(c, 0x80160fa8);
    if p { crate::prof::exit(c, 0x80160fa8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80160fa8_at(c: &mut Cpu, pc: u32) { f_80160fa8_b(c, pc) }
#[inline(always)]
fn f_80160fa8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80160fa8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80160fa8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80160fa8 => { ins += 7;
                c.r[6] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.lwz_t(ins, c.r[4].wrapping_add(0xf4));
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(0xf4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x40);
                c.r[7] = c.r[4].rotate_left(0) & 0xff;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                if !cr_bit(cr, 0) { pc = 0x80161000; continue; }
                pc = 0x80160fc4;
            }
            0x80160fc4 => { ins += 15;
                c.r[6] = c.r[0].wrapping_mul(0x86);
                c.r[4] = 0x80450000;
                c.r[5] = c.r[3].rotate_left(0) & 0x1f;
                c.r[4] = c.r[4].wrapping_add(0xffffd910);
                c.r[7] = c.r[7].wrapping_mul(0x86);
                c.r[3] = c.r[4].wrapping_add(c.r[6]);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[5]));
                c.r[8] = c.r[4].wrapping_add(0x20);
                c.r[4] = c.r[4].wrapping_add(c.r[7]);
                c.r[3] = c.r[6].wrapping_add(c.r[5]);
                c.stb(c.r[4].wrapping_add(c.r[5]), c.r[0]);
                c.r[0] = c.r[7].wrapping_add(c.r[5]);
                c.r[3] = c.lbz_t(ins, c.r[8].wrapping_add(c.r[3]));
                c.stb(c.r[8].wrapping_add(c.r[0]), c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161000;
            }
            0x80161000 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x80);
                if cr_bit(cr, 2) { pc = 0x80161010; continue; }
                pc = 0x80161008;
            }
            0x80161008 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x81);
                if !cr_bit(cr, 2) { pc = 0x8016104c; continue; }
                pc = 0x80161010;
            }
            0x80161010 => { ins += 15;
                c.r[6] = c.r[0].wrapping_mul(0x86);
                c.r[4] = 0x80450000;
                c.r[5] = c.r[3].rotate_left(0) & 0xfe;
                c.r[4] = c.r[4].wrapping_add(0xffffd910);
                c.r[3] = c.r[4].wrapping_add(c.r[6]);
                c.r[7] = c.r[7].wrapping_mul(0x86);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[5]));
                c.r[8] = c.r[4].wrapping_add(0x1);
                c.r[3] = c.r[4].wrapping_add(c.r[7]);
                c.stb(c.r[3].wrapping_add(c.r[5]), c.r[0]);
                c.r[3] = c.r[6].wrapping_add(c.r[5]);
                c.r[0] = c.r[7].wrapping_add(c.r[5]);
                c.r[3] = c.lbz_t(ins, c.r[8].wrapping_add(c.r[3]));
                c.stb(c.r[8].wrapping_add(c.r[0]), c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x8016104c;
            }
            0x8016104c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x84);
                if cr_bit(cr, 2) { pc = 0x8016105c; continue; }
                pc = 0x80161054;
            }
            0x80161054 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x85);
                if !cr_bit(cr, 2) { pc = 0x80161098; continue; }
                pc = 0x8016105c;
            }
            0x8016105c => { ins += 15;
                c.r[6] = c.r[0].wrapping_mul(0x86);
                c.r[4] = 0x80450000;
                c.r[5] = c.r[3].rotate_left(0) & 0xfe;
                c.r[4] = c.r[4].wrapping_add(0xffffd910);
                c.r[3] = c.r[4].wrapping_add(c.r[6]);
                c.r[7] = c.r[7].wrapping_mul(0x86);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[5]));
                c.r[8] = c.r[4].wrapping_add(0x1);
                c.r[3] = c.r[4].wrapping_add(c.r[7]);
                c.stb(c.r[3].wrapping_add(c.r[5]), c.r[0]);
                c.r[3] = c.r[6].wrapping_add(c.r[5]);
                c.r[0] = c.r[7].wrapping_add(c.r[5]);
                c.r[3] = c.lbz_t(ins, c.r[8].wrapping_add(c.r[3]));
                c.stb(c.r[8].wrapping_add(c.r[0]), c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161098;
            }
            0x80161098 => { ins += 9;
                c.r[4] = c.r[0].wrapping_mul(0x86);
                c.r[3] = 0x80450000;
                c.r[5] = c.r[3].wrapping_add(0xffffd910);
                c.r[0] = c.r[7].wrapping_mul(0x86);
                c.r[3] = c.r[5].wrapping_add(c.r[4]);
                c.r[4] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[6]));
                c.r[3] = c.r[5].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(c.r[6]), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80160fa8: bad pc {:#x}", pc),
        }
    }
}

/// inpSetMidiLastNote (0x801610bc, 60 bytes)
#[inline(never)]
pub fn f_801610bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801610bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801610bc); p = true; }
    }
    f_801610bc_b(c, 0x801610bc);
    if p { crate::prof::exit(c, 0x801610bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801610bc_at(c: &mut Cpu, pc: u32) { f_801610bc_b(c, pc) }
#[inline(always)]
fn f_801610bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801610bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801610bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801610bc => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x801610e4; continue; }
                pc = 0x801610c8;
            }
            0x801610c8 => { ins += 7;
                c.r[6] = 0x80450000;
                c.r[7] = c.r[4].rotate_left(4) & 0xff0;
                c.r[4] = c.r[6].wrapping_add(0xfffffad0);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[4].wrapping_add(c.r[7]);
                c.stb(c.r[3].wrapping_add(c.r[0]), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801610e4;
            }
            0x801610e4 => { ins += 5;
                c.r[4] = 0x80450000;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[4].wrapping_add(0xfffffb50);
                c.stb(c.r[3].wrapping_add(c.r[0]), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801610bc: bad pc {:#x}", pc),
        }
    }
}

/// inpGetMidiLastNote (0x801610f8, 60 bytes)
#[inline(never)]
pub fn f_801610f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801610f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801610f8); p = true; }
    }
    f_801610f8_b(c, 0x801610f8);
    if p { crate::prof::exit(c, 0x801610f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801610f8_at(c: &mut Cpu, pc: u32) { f_801610f8_b(c, pc) }
#[inline(always)]
fn f_801610f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801610f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801610f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801610f8 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x80161120; continue; }
                pc = 0x80161104;
            }
            0x80161104 => { ins += 7;
                c.r[5] = 0x80450000;
                c.r[6] = c.r[4].rotate_left(4) & 0xff0;
                c.r[4] = c.r[5].wrapping_add(0xfffffad0);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[4].wrapping_add(c.r[6]);
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[0]));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161120;
            }
            0x80161120 => { ins += 5;
                c.r[4] = 0x80450000;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[4].wrapping_add(0xfffffb50);
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[0]));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801610f8: bad pc {:#x}", pc),
        }
    }
}

/// _GetInputValue (0x80161134, 1184 bytes)
#[inline(never)]
pub fn f_80161134(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80161134) { return; }
        if c.prof { crate::prof::enter(c, 0x80161134); p = true; }
    }
    f_80161134_b(c, 0x80161134);
    if p { crate::prof::exit(c, 0x80161134); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161134_at(c: &mut Cpu, pc: u32) { f_80161134_b(c, pc) }
#[inline(never)]
fn f_80161134_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80161134;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80161134, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80161134 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x80161148; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c47cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161148;
            }
            0x80161148 => { ins += 8;
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
            0x80161168 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x1));
                c.r[0] = c.r[0].rotate_left(0) & 0x10;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801611a0; continue; }
                pc = 0x80161174;
            }
            0x80161174 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[24], 0x0);
                if cr_bit(cr, 2) { pc = 0x80161194; continue; }
                pc = 0x8016117c;
            }
            0x8016117c => { ins += 4;
                c.r[5] = c.lbz_t(ins, c.r[31]);
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.lr = 0x8016118c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80155078(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016118c;
            }
            0x8016118c => { ins += 2;
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x80161198; continue;
                pc = 0x80161194;
            }
            0x80161194 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x80161198;
            }
            0x80161198 => { ins += 2;
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x8016122c; continue;
                pc = 0x801611a0;
            }
            0x801611a0 => { ins += 3;
                c.r[3] = c.lbz_t(ins, c.r[31]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x80);
                if cr_bit(cr, 2) { pc = 0x801611d4; continue; }
                pc = 0x801611ac;
            }
            0x801611ac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x1);
                if cr_bit(cr, 2) { pc = 0x801611d4; continue; }
                pc = 0x801611b4;
            }
            0x801611b4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0xa);
                if cr_bit(cr, 2) { pc = 0x801611d4; continue; }
                pc = 0x801611bc;
            }
            0x801611bc => { ins += 4;
                c.r[0] = c.r[3].wrapping_add(0xffffff60);
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 1) { pc = 0x801611d4; continue; }
                pc = 0x801611cc;
            }
            0x801611cc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x83);
                if !cr_bit(cr, 2) { pc = 0x80161394; continue; }
                pc = 0x801611d4;
            }
            0x801611d4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xa2);
                if !cr_bit(cr, 0) { pc = 0x80161218; continue; }
                pc = 0x801611dc;
            }
            0x801611dc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xa0);
                if !cr_bit(cr, 0) { pc = 0x801611e8; continue; }
                pc = 0x801611e4;
            }
            0x801611e4 => { ins += 1;
                pc = 0x80161218; continue;
                pc = 0x801611e8;
            }
            0x801611e8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[24], 0x0);
                if cr_bit(cr, 2) { pc = 0x80161210; continue; }
                pc = 0x801611f0;
            }
            0x801611f0 => { ins += 8;
                c.r[4] = c.r[3].wrapping_mul(0xc);
                c.r[0] = c.r[3].wrapping_add(0x134);
                c.r[3] = 0x1;
                c.r[4] = c.r[4].wrapping_add(0xfffffa44);
                c.r[4] = c.lha_t(ins, c.r[24].wrapping_add(c.r[4]));
                c.stb(c.r[24].wrapping_add(c.r[0]), c.r[3]);
                c.r[3] = c.r[4].rotate_left(1) & 0xfffffffe;
                pc = 0x8016122c; continue;
                pc = 0x80161210;
            }
            0x80161210 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8016122c; continue;
                pc = 0x80161218;
            }
            0x80161218 => { ins += 3;
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.lr = 0x80161224; { c.steps = st; c.instrs = ins; c.cr = cr; f_80160bdc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161224;
            }
            0x80161224 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0xffffe000);
                pc = 0x8016122c;
            }
            0x8016122c => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.r[0] = c.r[3].wrapping_mul(c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 17 != 0); c.r[0] = (v >> 15) as u32; }
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xffffe000);
                if !cr_bit(cr, 0) { pc = 0x8016124c; continue; }
                pc = 0x80161244;
            }
            0x80161244 => { ins += 2;
                c.r[3] = 0xffffe000;
                pc = 0x8016125c; continue;
                pc = 0x8016124c;
            }
            0x8016124c => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x1fff);
                c.r[3] = 0x1fff;
                if cr_bit(cr, 1) { pc = 0x8016125c; continue; }
                pc = 0x80161258;
            }
            0x80161258 => { ins += 1;
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x8016125c;
            }
            0x8016125c => { ins += 4;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x1));
                c.r[0] = c.r[0].rotate_left(0) & 0xf;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x801612f0; continue; }
                pc = 0x8016126c;
            }
            0x8016126c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80161280; continue; }
                pc = 0x80161270;
            }
            0x80161270 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x8016128c; continue; }
                pc = 0x80161278;
            }
            0x80161278 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80161298; continue; }
                pc = 0x8016127c;
            }
            0x8016127c => { ins += 1;
                pc = 0x801615a0; continue;
                pc = 0x80161280;
            }
            0x80161280 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x4);
                if !cr_bit(cr, 0) { pc = 0x801615a0; continue; }
                pc = 0x80161288;
            }
            0x80161288 => { ins += 1;
                pc = 0x8016133c; continue;
                pc = 0x8016128c;
            }
            0x8016128c => { ins += 3;
                c.r[30] = c.r[3].wrapping_add(0x2000);
                c.r[28] = 0x1;
                pc = 0x801615a0; continue;
                pc = 0x80161298;
            }
            0x80161298 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x801612d0; continue; }
                pc = 0x801612a0;
            }
            0x801612a0 => { ins += 4;
                c.r[4] = c.r[30].wrapping_add(c.r[3]);
                c.r[4] = c.r[4].wrapping_add(0xffffe000);
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0xffffe000);
                if !cr_bit(cr, 0) { pc = 0x801612b8; continue; }
                pc = 0x801612b0;
            }
            0x801612b0 => { ins += 2;
                c.r[3] = 0xffffe000;
                pc = 0x801612c8; continue;
                pc = 0x801612b8;
            }
            0x801612b8 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x1fff);
                c.r[3] = 0x1fff;
                if cr_bit(cr, 1) { pc = 0x801612c8; continue; }
                pc = 0x801612c4;
            }
            0x801612c4 => { ins += 1;
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x801612c8;
            }
            0x801612c8 => { ins += 2;
                c.r[30] = c.r[3].wrapping_add(0x2000);
                pc = 0x801615a0; continue;
                pc = 0x801612d0;
            }
            0x801612d0 => { ins += 3;
                c.r[3] = c.r[30].wrapping_add(c.r[3]);
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x3fff);
                if !cr_bit(cr, 1) { pc = 0x801612e4; continue; }
                pc = 0x801612dc;
            }
            0x801612dc => { ins += 2;
                c.r[30] = 0x3fff;
                pc = 0x801615a0; continue;
                pc = 0x801612e4;
            }
            0x801612e4 => { ins += 3;
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[0] = (v >> 31) as u32; }
                c.r[30] = c.r[3] & !c.r[0];
                pc = 0x801615a0; continue;
                pc = 0x801612f0;
            }
            0x801612f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x80161308; continue; }
                pc = 0x801612f8;
            }
            0x801612f8 => { ins += 4;
                c.r[0] = c.r[30].wrapping_add(0xffffe000);
                c.r[0] = c.r[0].wrapping_mul(c.r[3]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 19 != 0); c.r[0] = (v >> 13) as u32; }
                pc = 0x80161314; continue;
                pc = 0x80161308;
            }
            0x80161308 => { ins += 3;
                c.r[0] = c.r[3].wrapping_mul(c.r[30]);
                c.r[28] = 0x1;
                c.r[0] = c.r[0].rotate_left(19) & 0x7ffff;
                pc = 0x80161314;
            }
            0x80161314 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xffffe000);
                if !cr_bit(cr, 0) { pc = 0x80161324; continue; }
                pc = 0x8016131c;
            }
            0x8016131c => { ins += 2;
                c.r[3] = 0xffffe000;
                pc = 0x80161334; continue;
                pc = 0x80161324;
            }
            0x80161324 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x1fff);
                c.r[3] = 0x1fff;
                if cr_bit(cr, 1) { pc = 0x80161334; continue; }
                pc = 0x80161330;
            }
            0x80161330 => { ins += 1;
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x80161334;
            }
            0x80161334 => { ins += 2;
                c.r[30] = c.r[3].wrapping_add(0x2000);
                pc = 0x801615a0; continue;
                pc = 0x8016133c;
            }
            0x8016133c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x80161374; continue; }
                pc = 0x80161344;
            }
            0x80161344 => { ins += 4;
                c.r[0] = c.r[30].wrapping_add(0xffffe000);
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xffffe000);
                if !cr_bit(cr, 0) { pc = 0x8016135c; continue; }
                pc = 0x80161354;
            }
            0x80161354 => { ins += 2;
                c.r[3] = 0xffffe000;
                pc = 0x8016136c; continue;
                pc = 0x8016135c;
            }
            0x8016135c => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x1fff);
                c.r[3] = 0x1fff;
                if cr_bit(cr, 1) { pc = 0x8016136c; continue; }
                pc = 0x80161368;
            }
            0x80161368 => { ins += 1;
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x8016136c;
            }
            0x8016136c => { ins += 2;
                c.r[30] = c.r[3].wrapping_add(0x2000);
                pc = 0x801615a0; continue;
                pc = 0x80161374;
            }
            0x80161374 => { ins += 3;
                c.r[3] = c.r[30].wrapping_sub(c.r[3]);
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x3fff);
                if !cr_bit(cr, 1) { pc = 0x80161388; continue; }
                pc = 0x80161380;
            }
            0x80161380 => { ins += 2;
                c.r[30] = 0x3fff;
                pc = 0x801615a0; continue;
                pc = 0x80161388;
            }
            0x80161388 => { ins += 3;
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[0] = (v >> 31) as u32; }
                c.r[30] = c.r[3] & !c.r[0];
                pc = 0x801615a0; continue;
                pc = 0x80161394;
            }
            0x80161394 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xa3);
                if cr_bit(cr, 2) { pc = 0x801613d4; continue; }
                pc = 0x8016139c;
            }
            0x8016139c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801613ac; continue; }
                pc = 0x801613a0;
            }
            0x801613a0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xa2);
                if !cr_bit(cr, 0) { pc = 0x801613b8; continue; }
                pc = 0x801613a8;
            }
            0x801613a8 => { ins += 1;
                pc = 0x80161438; continue;
                pc = 0x801613ac;
            }
            0x801613ac => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xa5);
                if !cr_bit(cr, 0) { pc = 0x80161438; continue; }
                pc = 0x801613b4;
            }
            0x801613b4 => { ins += 1;
                pc = 0x801613f0; continue;
                pc = 0x801613b8;
            }
            0x801613b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[24], 0x0);
                if cr_bit(cr, 2) { pc = 0x801613cc; continue; }
                pc = 0x801613c0;
            }
            0x801613c0 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[24].wrapping_add(0x12f));
                c.r[5] = c.r[0].rotate_left(7) & 0xffffff80;
                pc = 0x80161448; continue;
                pc = 0x801613cc;
            }
            0x801613cc => { ins += 2;
                c.r[5] = 0x0;
                pc = 0x80161448; continue;
                pc = 0x801613d4;
            }
            0x801613d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[24], 0x0);
                if cr_bit(cr, 2) { pc = 0x801613e8; continue; }
                pc = 0x801613dc;
            }
            0x801613dc => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[24].wrapping_add(0x158));
                c.r[5] = c.r[0].rotate_left(23) & 0x7fffff;
                pc = 0x80161448; continue;
                pc = 0x801613e8;
            }
            0x801613e8 => { ins += 2;
                c.r[5] = 0x0;
                pc = 0x80161448; continue;
                pc = 0x801613f0;
            }
            0x801613f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[24], 0x0);
                if cr_bit(cr, 2) { pc = 0x80161430; continue; }
                pc = 0x801613f8;
            }
            0x801613f8 => { ins += 10;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa73c));
                c.r[0] = c.lwz_t(ins, c.r[24].wrapping_add(0x94));
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa738));
                c.r[0] = c.addc(!c.r[0], c.r[3], 1);
                c.r[3] = c.lwz_t(ins, c.r[24].wrapping_add(0x90));
                c.r[5] = c.r[0].rotate_left(24) & 0xffffffff;
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[3], c.r[4], ci); }
                c.r[5] = (c.r[0].rotate_left(24) & 0xff000000) | (c.r[5] & 0xffffff);
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0x3fff);
                if !cr_bit(cr, 1) { pc = 0x80161424; continue; }
                pc = 0x80161420;
            }
            0x80161420 => { ins += 1;
                c.r[5] = 0x3fff;
                pc = 0x80161424;
            }
            0x80161424 => { ins += 3;
                c.r[0] = 0x1;
                c.stb(c.r[24].wrapping_add(0xa8), c.r[0]);
                pc = 0x80161448; continue;
                pc = 0x80161430;
            }
            0x80161430 => { ins += 2;
                c.r[5] = 0x0;
                pc = 0x80161448; continue;
                pc = 0x80161438;
            }
            0x80161438 => { ins += 3;
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.lr = 0x80161444; { c.steps = st; c.instrs = ins; c.cr = cr; f_80160bdc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161444;
            }
            0x80161444 => { ins += 1;
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80161448;
            }
            0x80161448 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.r[0] = c.r[5].wrapping_mul(c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 17 != 0); c.r[3] = (v >> 15) as u32; }
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x3fff);
                if !cr_bit(cr, 1) { pc = 0x80161464; continue; }
                pc = 0x80161460;
            }
            0x80161460 => { ins += 1;
                c.r[3] = 0x3fff;
                pc = 0x80161464;
            }
            0x80161464 => { ins += 4;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x1));
                c.r[0] = c.r[0].rotate_left(0) & 0xf;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x801614f4; continue; }
                pc = 0x80161474;
            }
            0x80161474 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80161488; continue; }
                pc = 0x80161478;
            }
            0x80161478 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x80161494; continue; }
                pc = 0x80161480;
            }
            0x80161480 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801614a0; continue; }
                pc = 0x80161484;
            }
            0x80161484 => { ins += 1;
                pc = 0x801615a0; continue;
                pc = 0x80161488;
            }
            0x80161488 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x4);
                if !cr_bit(cr, 0) { pc = 0x801615a0; continue; }
                pc = 0x80161490;
            }
            0x80161490 => { ins += 1;
                pc = 0x8016154c; continue;
                pc = 0x80161494;
            }
            0x80161494 => { ins += 3;
                c.r[30] = c.r[3] | c.r[3];
                c.r[28] = 0x0;
                pc = 0x801615a0; continue;
                pc = 0x801614a0;
            }
            0x801614a0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x801614d8; continue; }
                pc = 0x801614a8;
            }
            0x801614a8 => { ins += 4;
                c.r[4] = c.r[30].wrapping_add(c.r[3]);
                c.r[4] = c.r[4].wrapping_add(0xffffe000);
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0xffffe000);
                if !cr_bit(cr, 0) { pc = 0x801614c0; continue; }
                pc = 0x801614b8;
            }
            0x801614b8 => { ins += 2;
                c.r[3] = 0xffffe000;
                pc = 0x801614d0; continue;
                pc = 0x801614c0;
            }
            0x801614c0 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x1fff);
                c.r[3] = 0x1fff;
                if cr_bit(cr, 1) { pc = 0x801614d0; continue; }
                pc = 0x801614cc;
            }
            0x801614cc => { ins += 1;
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x801614d0;
            }
            0x801614d0 => { ins += 2;
                c.r[30] = c.r[3].wrapping_add(0x2000);
                pc = 0x801615a0; continue;
                pc = 0x801614d8;
            }
            0x801614d8 => { ins += 4;
                c.r[30] = c.r[30].wrapping_add(c.r[3]);
                c.r[0] = 0x3fff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x3fff);
                if cr_bit(cr, 1) { pc = 0x801614ec; continue; }
                pc = 0x801614e8;
            }
            0x801614e8 => { ins += 1;
                c.r[0] = c.r[30] | c.r[30];
                pc = 0x801614ec;
            }
            0x801614ec => { ins += 2;
                c.r[30] = c.r[0] | c.r[0];
                pc = 0x801615a0; continue;
                pc = 0x801614f4;
            }
            0x801614f4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x80161530; continue; }
                pc = 0x801614fc;
            }
            0x801614fc => { ins += 5;
                c.r[0] = c.r[30].wrapping_add(0xffffe000);
                c.r[0] = c.r[3].wrapping_mul(c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 18 != 0); c.r[0] = (v >> 14) as u32; }
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xffffe000);
                if !cr_bit(cr, 0) { pc = 0x80161518; continue; }
                pc = 0x80161510;
            }
            0x80161510 => { ins += 2;
                c.r[3] = 0xffffe000;
                pc = 0x80161528; continue;
                pc = 0x80161518;
            }
            0x80161518 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x1fff);
                c.r[3] = 0x1fff;
                if cr_bit(cr, 1) { pc = 0x80161528; continue; }
                pc = 0x80161524;
            }
            0x80161524 => { ins += 1;
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x80161528;
            }
            0x80161528 => { ins += 2;
                c.r[30] = c.r[3].wrapping_add(0x2000);
                pc = 0x801615a0; continue;
                pc = 0x80161530;
            }
            0x80161530 => { ins += 5;
                c.r[0] = c.r[30].wrapping_mul(c.r[3]);
                c.r[30] = 0x3fff;
                c.r[0] = c.r[0].rotate_left(18) & 0x3ffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3fff);
                if cr_bit(cr, 1) { pc = 0x801615a0; continue; }
                pc = 0x80161544;
            }
            0x80161544 => { ins += 2;
                c.r[30] = c.r[0] | c.r[0];
                pc = 0x801615a0; continue;
                pc = 0x8016154c;
            }
            0x8016154c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x80161584; continue; }
                pc = 0x80161554;
            }
            0x80161554 => { ins += 4;
                c.r[0] = c.r[30].wrapping_add(0xffffe000);
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xffffe000);
                if !cr_bit(cr, 0) { pc = 0x8016156c; continue; }
                pc = 0x80161564;
            }
            0x80161564 => { ins += 2;
                c.r[3] = 0xffffe000;
                pc = 0x8016157c; continue;
                pc = 0x8016156c;
            }
            0x8016156c => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x1fff);
                c.r[3] = 0x1fff;
                if cr_bit(cr, 1) { pc = 0x8016157c; continue; }
                pc = 0x80161578;
            }
            0x80161578 => { ins += 1;
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x8016157c;
            }
            0x8016157c => { ins += 2;
                c.r[30] = c.r[3].wrapping_add(0x2000);
                pc = 0x801615a0; continue;
                pc = 0x80161584;
            }
            0x80161584 => { ins += 3;
                c.r[3] = c.r[30].wrapping_sub(c.r[3]);
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x3fff);
                if !cr_bit(cr, 1) { pc = 0x80161598; continue; }
                pc = 0x80161590;
            }
            0x80161590 => { ins += 2;
                c.r[30] = 0x3fff;
                pc = 0x801615a0; continue;
                pc = 0x80161598;
            }
            0x80161598 => { ins += 2;
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[0] = (v >> 31) as u32; }
                c.r[30] = c.r[3] & !c.r[0];
                pc = 0x801615a0;
            }
            0x801615a0 => { ins += 2;
                c.r[31] = c.r[31].wrapping_add(0x8);
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801615a8;
            }
            0x801615a8 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[25].wrapping_add(0x22));
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80161168; continue; }
                pc = 0x801615b4;
            }
            0x801615b4 => { ins += 4;
                c.sth(c.r[25].wrapping_add(0x20), c.r[30]);
                c.r[3] = c.r[30].rotate_left(0) & 0xffff;
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x801615c4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c4818(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801615c4;
            }
            0x801615c4 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80161134: bad pc {:#x}", pc),
        }
    }
}

/// inpGetVolume (0x801615d4, 72 bytes)
#[inline(never)]
pub fn f_801615d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801615d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801615d4); p = true; }
    }
    f_801615d4_b(c, 0x801615d4);
    if p { crate::prof::exit(c, 0x801615d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801615d4_at(c: &mut Cpu, pc: u32) { f_801615d4_b(c, pc) }
#[inline(always)]
fn f_801615d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801615d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801615d4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801615d4 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801615f4; continue; }
                pc = 0x801615ec;
            }
            0x801615ec => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x238));
                pc = 0x8016160c; continue;
                pc = 0x801615f4;
            }
            0x801615f4 => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffffe;
                c.r[4] = c.r[3].wrapping_add(0x218);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                c.lr = 0x8016160c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016160c;
            }
            0x8016160c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801615d4: bad pc {:#x}", pc),
        }
    }
}

/// inpGetPanning (0x8016161c, 72 bytes)
#[inline(never)]
pub fn f_8016161c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016161c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016161c); p = true; }
    }
    f_8016161c_b(c, 0x8016161c);
    if p { crate::prof::exit(c, 0x8016161c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016161c_at(c: &mut Cpu, pc: u32) { f_8016161c_b(c, pc) }
#[inline(always)]
fn f_8016161c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016161c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016161c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016161c => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x2;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8016163c; continue; }
                pc = 0x80161634;
            }
            0x80161634 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x25c));
                pc = 0x80161654; continue;
                pc = 0x8016163c;
            }
            0x8016163c => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffffd;
                c.r[4] = c.r[3].wrapping_add(0x23c);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                c.lr = 0x80161654; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161654;
            }
            0x80161654 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016161c: bad pc {:#x}", pc),
        }
    }
}

/// inpGetSurroundPanning (0x80161664, 72 bytes)
#[inline(never)]
pub fn f_80161664(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80161664) { return; }
        if c.prof { crate::prof::enter(c, 0x80161664); p = true; }
    }
    f_80161664_b(c, 0x80161664);
    if p { crate::prof::exit(c, 0x80161664); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161664_at(c: &mut Cpu, pc: u32) { f_80161664_b(c, pc) }
#[inline(always)]
fn f_80161664_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80161664;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80161664, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80161664 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x4;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80161684; continue; }
                pc = 0x8016167c;
            }
            0x8016167c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x280));
                pc = 0x8016169c; continue;
                pc = 0x80161684;
            }
            0x80161684 => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffffb;
                c.r[4] = c.r[3].wrapping_add(0x260);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                c.lr = 0x8016169c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016169c;
            }
            0x8016169c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80161664: bad pc {:#x}", pc),
        }
    }
}

/// inpGetPitchBend (0x801616ac, 72 bytes)
#[inline(never)]
pub fn f_801616ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801616ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801616ac); p = true; }
    }
    f_801616ac_b(c, 0x801616ac);
    if p { crate::prof::exit(c, 0x801616ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801616ac_at(c: &mut Cpu, pc: u32) { f_801616ac_b(c, pc) }
#[inline(always)]
fn f_801616ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801616ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801616ac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801616ac => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x8;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801616cc; continue; }
                pc = 0x801616c4;
            }
            0x801616c4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2a4));
                pc = 0x801616e4; continue;
                pc = 0x801616cc;
            }
            0x801616cc => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffff7;
                c.r[4] = c.r[3].wrapping_add(0x284);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                c.lr = 0x801616e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801616e4;
            }
            0x801616e4 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801616ac: bad pc {:#x}", pc),
        }
    }
}

/// inpGetDoppler (0x801616f4, 72 bytes)
#[inline(never)]
pub fn f_801616f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801616f4) { return; }
        if c.prof { crate::prof::enter(c, 0x801616f4); p = true; }
    }
    f_801616f4_b(c, 0x801616f4);
    if p { crate::prof::exit(c, 0x801616f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801616f4_at(c: &mut Cpu, pc: u32) { f_801616f4_b(c, pc) }
#[inline(always)]
fn f_801616f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801616f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801616f4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801616f4 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x10;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80161714; continue; }
                pc = 0x8016170c;
            }
            0x8016170c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2c8));
                pc = 0x8016172c; continue;
                pc = 0x80161714;
            }
            0x80161714 => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xffffffef;
                c.r[4] = c.r[3].wrapping_add(0x2a8);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                c.lr = 0x8016172c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016172c;
            }
            0x8016172c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801616f4: bad pc {:#x}", pc),
        }
    }
}

/// inpGetModulation (0x8016173c, 72 bytes)
#[inline(never)]
pub fn f_8016173c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016173c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016173c); p = true; }
    }
    f_8016173c_b(c, 0x8016173c);
    if p { crate::prof::exit(c, 0x8016173c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016173c_at(c: &mut Cpu, pc: u32) { f_8016173c_b(c, pc) }
#[inline(always)]
fn f_8016173c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016173c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016173c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016173c => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x20;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8016175c; continue; }
                pc = 0x80161754;
            }
            0x80161754 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2ec));
                pc = 0x80161774; continue;
                pc = 0x8016175c;
            }
            0x8016175c => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xffffffdf;
                c.r[4] = c.r[3].wrapping_add(0x2cc);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                c.lr = 0x80161774; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161774;
            }
            0x80161774 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016173c: bad pc {:#x}", pc),
        }
    }
}

/// inpGetPedal (0x80161784, 72 bytes)
#[inline(never)]
pub fn f_80161784(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80161784) { return; }
        if c.prof { crate::prof::enter(c, 0x80161784); p = true; }
    }
    f_80161784_b(c, 0x80161784);
    if p { crate::prof::exit(c, 0x80161784); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161784_at(c: &mut Cpu, pc: u32) { f_80161784_b(c, pc) }
#[inline(always)]
fn f_80161784_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80161784;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80161784, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80161784 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x40;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801617a4; continue; }
                pc = 0x8016179c;
            }
            0x8016179c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x310));
                pc = 0x801617bc; continue;
                pc = 0x801617a4;
            }
            0x801617a4 => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xffffffbf;
                c.r[4] = c.r[3].wrapping_add(0x2f0);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                c.lr = 0x801617bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801617bc;
            }
            0x801617bc => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80161784: bad pc {:#x}", pc),
        }
    }
}

/// inpGetPreAuxA (0x801617cc, 72 bytes)
#[inline(never)]
pub fn f_801617cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801617cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801617cc); p = true; }
    }
    f_801617cc_b(c, 0x801617cc);
    if p { crate::prof::exit(c, 0x801617cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801617cc_at(c: &mut Cpu, pc: u32) { f_801617cc_b(c, pc) }
#[inline(always)]
fn f_801617cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801617cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801617cc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801617cc => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x100;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801617ec; continue; }
                pc = 0x801617e4;
            }
            0x801617e4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x358));
                pc = 0x80161804; continue;
                pc = 0x801617ec;
            }
            0x801617ec => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffeff;
                c.r[4] = c.r[3].wrapping_add(0x338);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                c.lr = 0x80161804; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161804;
            }
            0x80161804 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801617cc: bad pc {:#x}", pc),
        }
    }
}

/// inpGetReverb (0x80161814, 72 bytes)
#[inline(never)]
pub fn f_80161814(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80161814) { return; }
        if c.prof { crate::prof::enter(c, 0x80161814); p = true; }
    }
    f_80161814_b(c, 0x80161814);
    if p { crate::prof::exit(c, 0x80161814); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161814_at(c: &mut Cpu, pc: u32) { f_80161814_b(c, pc) }
#[inline(always)]
fn f_80161814_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80161814;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80161814, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80161814 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x200;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80161834; continue; }
                pc = 0x8016182c;
            }
            0x8016182c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x37c));
                pc = 0x8016184c; continue;
                pc = 0x80161834;
            }
            0x80161834 => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffdff;
                c.r[4] = c.r[3].wrapping_add(0x35c);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                c.lr = 0x8016184c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016184c;
            }
            0x8016184c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80161814: bad pc {:#x}", pc),
        }
    }
}

/// inpGetPreAuxB (0x8016185c, 72 bytes)
#[inline(never)]
pub fn f_8016185c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016185c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016185c); p = true; }
    }
    f_8016185c_b(c, 0x8016185c);
    if p { crate::prof::exit(c, 0x8016185c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016185c_at(c: &mut Cpu, pc: u32) { f_8016185c_b(c, pc) }
#[inline(always)]
fn f_8016185c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016185c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016185c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016185c => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x400;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8016187c; continue; }
                pc = 0x80161874;
            }
            0x80161874 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x3a0));
                pc = 0x80161894; continue;
                pc = 0x8016187c;
            }
            0x8016187c => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffbff;
                c.r[4] = c.r[3].wrapping_add(0x380);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                c.lr = 0x80161894; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161894;
            }
            0x80161894 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016185c: bad pc {:#x}", pc),
        }
    }
}

/// inpGetPostAuxB (0x801618a4, 72 bytes)
#[inline(never)]
pub fn f_801618a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801618a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801618a4); p = true; }
    }
    f_801618a4_b(c, 0x801618a4);
    if p { crate::prof::exit(c, 0x801618a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801618a4_at(c: &mut Cpu, pc: u32) { f_801618a4_b(c, pc) }
#[inline(always)]
fn f_801618a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801618a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801618a4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801618a4 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x800;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801618c4; continue; }
                pc = 0x801618bc;
            }
            0x801618bc => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x3c4));
                pc = 0x801618dc; continue;
                pc = 0x801618c4;
            }
            0x801618c4 => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xfffff7ff;
                c.r[4] = c.r[3].wrapping_add(0x3a4);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                c.lr = 0x801618dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801618dc;
            }
            0x801618dc => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801618a4: bad pc {:#x}", pc),
        }
    }
}

/// inpGetTremolo (0x801618ec, 72 bytes)
#[inline(never)]
pub fn f_801618ec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801618ec) { return; }
        if c.prof { crate::prof::enter(c, 0x801618ec); p = true; }
    }
    f_801618ec_b(c, 0x801618ec);
    if p { crate::prof::exit(c, 0x801618ec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801618ec_at(c: &mut Cpu, pc: u32) { f_801618ec_b(c, pc) }
#[inline(always)]
fn f_801618ec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801618ec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801618ec, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801618ec => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x214));
                c.r[0] = c.r[4].rotate_left(0) & 0x1000;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8016190c; continue; }
                pc = 0x80161904;
            }
            0x80161904 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x3e8));
                pc = 0x80161924; continue;
                pc = 0x8016190c;
            }
            0x8016190c => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xffffefff;
                c.r[4] = c.r[3].wrapping_add(0x3c8);
                c.stw(c.r[3].wrapping_add(0x214), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                c.r[6] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                c.lr = 0x80161924; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161924;
            }
            0x80161924 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801618ec: bad pc {:#x}", pc),
        }
    }
}

/// inpGetAuxA (0x80161934, 180 bytes)
#[inline(never)]
pub fn f_80161934(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80161934) { return; }
        if c.prof { crate::prof::enter(c, 0x80161934); p = true; }
    }
    f_80161934_b(c, 0x80161934);
    if p { crate::prof::exit(c, 0x80161934); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161934_at(c: &mut Cpu, pc: u32) { f_80161934_b(c, pc) }
#[inline(always)]
fn f_80161934_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80161934;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80161934, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80161934 => { ins += 19;
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
                c.r[10] = c.lwz_t(ins, c.r[7].wrapping_add(c.r[10]));
                c.r[7] = c.lwz_t(ins, c.r[9].wrapping_add(c.r[8]));
                c.r[4] = c.r[10] & c.r[7];
                c.r[0] = c.r[4].wrapping_neg();
                c.r[0] = c.r[0] | c.r[4];
                c.r[4] = c.r[0].rotate_left(1) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[4]);
                if cr_bit(cr, 2) { pc = 0x80161988; continue; }
                pc = 0x80161980;
            }
            0x80161980 => { ins += 2;
                c.r[0] = c.r[7] & !c.r[10];
                c.stw(c.r[9].wrapping_add(c.r[8]), c.r[0]);
                pc = 0x80161988;
            }
            0x80161988 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { pc = 0x801619b8; continue; }
                pc = 0x80161990;
            }
            0x80161990 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x80430000;
                c.r[7] = c.r[0].wrapping_mul(0x90);
                c.r[4] = c.r[3].wrapping_add(0x5b74);
                c.r[3] = 0x0;
                c.r[0] = c.r[11].wrapping_mul(0x24);
                c.r[4] = c.r[4].wrapping_add(c.r[7]);
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.lr = 0x801619b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801619b4;
            }
            0x801619b4 => { ins += 1;
                pc = 0x801619d8; continue;
                pc = 0x801619b8;
            }
            0x801619b8 => { ins += 8;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = 0x80430000;
                c.r[3] = c.r[0].wrapping_mul(0x90);
                c.r[4] = c.r[4].wrapping_add(0x5b74);
                c.r[0] = c.r[11].wrapping_mul(0x24);
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x20));
                pc = 0x801619d8;
            }
            0x801619d8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80161934: bad pc {:#x}", pc),
        }
    }
}

/// inpGetAuxB (0x801619e8, 180 bytes)
#[inline(never)]
pub fn f_801619e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801619e8) { return; }
        if c.prof { crate::prof::enter(c, 0x801619e8); p = true; }
    }
    f_801619e8_b(c, 0x801619e8);
    if p { crate::prof::exit(c, 0x801619e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801619e8_at(c: &mut Cpu, pc: u32) { f_801619e8_b(c, pc) }
#[inline(always)]
fn f_801619e8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801619e8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801619e8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801619e8 => { ins += 19;
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
                c.r[10] = c.lwz_t(ins, c.r[7].wrapping_add(c.r[10]));
                c.r[7] = c.lwz_t(ins, c.r[9].wrapping_add(c.r[8]));
                c.r[4] = c.r[10] & c.r[7];
                c.r[0] = c.r[4].wrapping_neg();
                c.r[0] = c.r[0] | c.r[4];
                c.r[4] = c.r[0].rotate_left(1) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[4]);
                if cr_bit(cr, 2) { pc = 0x80161a3c; continue; }
                pc = 0x80161a34;
            }
            0x80161a34 => { ins += 2;
                c.r[0] = c.r[7] & !c.r[10];
                c.stw(c.r[9].wrapping_add(c.r[8]), c.r[0]);
                pc = 0x80161a3c;
            }
            0x80161a3c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { pc = 0x80161a6c; continue; }
                pc = 0x80161a44;
            }
            0x80161a44 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x80430000;
                c.r[7] = c.r[0].wrapping_mul(0x90);
                c.r[4] = c.r[3].wrapping_add(0x56f4);
                c.r[3] = 0x0;
                c.r[0] = c.r[11].wrapping_mul(0x24);
                c.r[4] = c.r[4].wrapping_add(c.r[7]);
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.lr = 0x80161a68; { c.steps = st; c.instrs = ins; c.cr = cr; f_80161134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161a68;
            }
            0x80161a68 => { ins += 1;
                pc = 0x80161a8c; continue;
                pc = 0x80161a6c;
            }
            0x80161a6c => { ins += 8;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = 0x80430000;
                c.r[3] = c.r[0].wrapping_mul(0x90);
                c.r[4] = c.r[4].wrapping_add(0x56f4);
                c.r[0] = c.r[11].wrapping_mul(0x24);
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x20));
                pc = 0x80161a8c;
            }
            0x80161a8c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801619e8: bad pc {:#x}", pc),
        }
    }
}

/// inpInit (0x80161a9c, 644 bytes)
#[inline(never)]
pub fn f_80161a9c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80161a9c) { return; }
        if c.prof { crate::prof::enter(c, 0x80161a9c); p = true; }
    }
    f_80161a9c_b(c, 0x80161a9c);
    if p { crate::prof::exit(c, 0x80161a9c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161a9c_at(c: &mut Cpu, pc: u32) { f_80161a9c_b(c, pc) }
#[inline(never)]
fn f_80161a9c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80161a9c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80161a9c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80161a9c => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                if cr_bit(cr, 2) { pc = 0x80161bbc; continue; }
                pc = 0x80161ab4;
            }
            0x80161ab4 => { ins += 66;
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
            0x80161bbc => { ins += 7;
                c.r[4] = 0x80430000;
                c.r[3] = 0x80430000;
                c.r[5] = c.r[3].wrapping_add(0x56f4);
                c.r[0] = 0x2;
                c.r[4] = c.r[4].wrapping_add(0x5b74);
                c.r[3] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x80161bd8;
            }
            0x80161bd8 => { ins += 35;
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
            0x80161c64 => { ins += 5;
                c.r[3] = 0x80450000;
                c.r[0] = 0x4;
                c.r[5] = c.r[3].wrapping_add(0xffff9390);
                c.r[3] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x80161c78;
            }
            0x80161c78 => { ins += 37;
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
            0x80161d0c => { ins += 5;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80161a9c: bad pc {:#x}", pc),
        }
    }
}

/// inpTranslateExCtrl (0x80161d20, 112 bytes)
#[inline(never)]
pub fn f_80161d20(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80161d20) { return; }
        if c.prof { crate::prof::enter(c, 0x80161d20); p = true; }
    }
    f_80161d20_b(c, 0x80161d20);
    if p { crate::prof::exit(c, 0x80161d20); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161d20_at(c: &mut Cpu, pc: u32) { f_80161d20_b(c, pc) }
#[inline(always)]
fn f_80161d20_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80161d20;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80161d20, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80161d20 => { ins += 1;
                c.r[4] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80161d24;
            }
            0x80161d24 => { ins += 1;
                c.r[0] = c.r[4].wrapping_add(0xffffff80);
                pc = 0x80161d28;
            }
            0x80161d28 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                pc = 0x80161d2c;
            }
            0x80161d2c => { ins += 1;
                if cr_bit(cr, 1) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161d30;
            }
            0x80161d30 => { ins += 1;
                c.r[4] = 0x80370000;
                pc = 0x80161d34;
            }
            0x80161d34 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80161d38;
            }
            0x80161d38 => { ins += 1;
                c.r[4] = c.r[4].wrapping_add(0xffff9cb0);
                pc = 0x80161d3c;
            }
            0x80161d3c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(c.r[0]));
                pc = 0x80161d40;
            }
            0x80161d40 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x80161d44;
            }
            0x80161d44 => { ins += 1;
                if true { let t = c.ctr; if (0x80161d20..0x80161d90).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161d48;
            }
            0x80161d48 => { ins += 1;
                c.r[3] = 0x80;
                pc = 0x80161d4c;
            }
            0x80161d4c => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161d50;
            }
            0x80161d50 => { ins += 1;
                c.r[3] = 0x82;
                pc = 0x80161d54;
            }
            0x80161d54 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161d58;
            }
            0x80161d58 => { ins += 1;
                c.r[3] = 0xa0;
                pc = 0x80161d5c;
            }
            0x80161d5c => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161d60;
            }
            0x80161d60 => { ins += 1;
                c.r[3] = 0xa1;
                pc = 0x80161d64;
            }
            0x80161d64 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161d68;
            }
            0x80161d68 => { ins += 1;
                c.r[3] = 0x83;
                pc = 0x80161d6c;
            }
            0x80161d6c => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161d70;
            }
            0x80161d70 => { ins += 1;
                c.r[3] = 0x84;
                pc = 0x80161d74;
            }
            0x80161d74 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161d78;
            }
            0x80161d78 => { ins += 1;
                c.r[3] = 0xa2;
                pc = 0x80161d7c;
            }
            0x80161d7c => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161d80;
            }
            0x80161d80 => { ins += 1;
                c.r[3] = 0xa3;
                pc = 0x80161d84;
            }
            0x80161d84 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161d88;
            }
            0x80161d88 => { ins += 1;
                c.r[3] = 0xa4;
                pc = 0x80161d8c;
            }
            0x80161d8c => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80161d20: bad pc {:#x}", pc),
        }
    }
}

/// inpGetExCtrl (0x80161d90, 252 bytes)
#[inline(never)]
pub fn f_80161d90(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80161d90) { return; }
        if c.prof { crate::prof::enter(c, 0x80161d90); p = true; }
    }
    f_80161d90_b(c, 0x80161d90);
    if p { crate::prof::exit(c, 0x80161d90); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161d90_at(c: &mut Cpu, pc: u32) { f_80161d90_b(c, pc) }
#[inline(never)]
fn f_80161d90_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80161d90;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80161d90, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80161d90 => { ins += 1;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x80161d94;
            }
            0x80161d94 => { ins += 1;
                c.r[0] = c.mfspr(8);
                pc = 0x80161d98;
            }
            0x80161d98 => { ins += 1;
                c.r[5] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x80161d9c;
            }
            0x80161d9c => { ins += 1;
                c.r[6] = c.r[4] | c.r[4];
                pc = 0x80161da0;
            }
            0x80161da0 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                pc = 0x80161da4;
            }
            0x80161da4 => { ins += 1;
                c.r[0] = c.r[5].wrapping_add(0xffffff80);
                pc = 0x80161da8;
            }
            0x80161da8 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                pc = 0x80161dac;
            }
            0x80161dac => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x80161e0c; continue; }
                pc = 0x80161db0;
            }
            0x80161db0 => { ins += 1;
                c.r[5] = 0x80370000;
                pc = 0x80161db4;
            }
            0x80161db4 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80161db8;
            }
            0x80161db8 => { ins += 1;
                c.r[5] = c.r[5].wrapping_add(0xffff9cd4);
                pc = 0x80161dbc;
            }
            0x80161dbc => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(c.r[0]));
                pc = 0x80161dc0;
            }
            0x80161dc0 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x80161dc4;
            }
            0x80161dc4 => { ins += 1;
                if true { let t = c.ctr; if (0x80161d90..0x80161e8c).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161dc8;
            }
            0x80161dc8 => { ins += 1;
                c.r[6] = 0x80;
                pc = 0x80161dcc;
            }
            0x80161dcc => { ins += 1;
                pc = 0x80161e0c; continue;
                pc = 0x80161dd0;
            }
            0x80161dd0 => { ins += 1;
                c.r[6] = 0x82;
                pc = 0x80161dd4;
            }
            0x80161dd4 => { ins += 1;
                pc = 0x80161e0c; continue;
                pc = 0x80161dd8;
            }
            0x80161dd8 => { ins += 1;
                c.r[6] = 0xa0;
                pc = 0x80161ddc;
            }
            0x80161ddc => { ins += 1;
                pc = 0x80161e0c; continue;
                pc = 0x80161de0;
            }
            0x80161de0 => { ins += 1;
                c.r[6] = 0xa1;
                pc = 0x80161de4;
            }
            0x80161de4 => { ins += 1;
                pc = 0x80161e0c; continue;
                pc = 0x80161de8;
            }
            0x80161de8 => { ins += 1;
                c.r[6] = 0x83;
                pc = 0x80161dec;
            }
            0x80161dec => { ins += 1;
                pc = 0x80161e0c; continue;
                pc = 0x80161df0;
            }
            0x80161df0 => { ins += 1;
                c.r[6] = 0x84;
                pc = 0x80161df4;
            }
            0x80161df4 => { ins += 1;
                pc = 0x80161e0c; continue;
                pc = 0x80161df8;
            }
            0x80161df8 => { ins += 1;
                c.r[6] = 0xa2;
                pc = 0x80161dfc;
            }
            0x80161dfc => { ins += 1;
                pc = 0x80161e0c; continue;
                pc = 0x80161e00;
            }
            0x80161e00 => { ins += 1;
                c.r[6] = 0xa3;
                pc = 0x80161e04;
            }
            0x80161e04 => { ins += 1;
                pc = 0x80161e0c; continue;
                pc = 0x80161e08;
            }
            0x80161e08 => { ins += 1;
                c.r[6] = 0xa4;
                pc = 0x80161e0c;
            }
            0x80161e0c => { ins += 1;
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                pc = 0x80161e10;
            }
            0x80161e10 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xa1);
                pc = 0x80161e14;
            }
            0x80161e14 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x80161e3c; continue; }
                pc = 0x80161e18;
            }
            0x80161e18 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80161e50; continue; }
                pc = 0x80161e1c;
            }
            0x80161e1c => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xa0);
                pc = 0x80161e20;
            }
            0x80161e20 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80161e28; continue; }
                pc = 0x80161e24;
            }
            0x80161e24 => { ins += 1;
                pc = 0x80161e50; continue;
                pc = 0x80161e28;
            }
            0x80161e28 => { ins += 1;
                c.r[0] = c.lha_t(ins, c.r[3].wrapping_add(0x1c4));
                pc = 0x80161e2c;
            }
            0x80161e2c => { ins += 1;
                c.r[3] = c.r[0].rotate_left(1) & 0xfffffffe;
                pc = 0x80161e30;
            }
            0x80161e30 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x2000);
                pc = 0x80161e34;
            }
            0x80161e34 => { ins += 1;
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80161e38;
            }
            0x80161e38 => { ins += 1;
                pc = 0x80161e7c; continue;
                pc = 0x80161e3c;
            }
            0x80161e3c => { ins += 1;
                c.r[0] = c.lha_t(ins, c.r[3].wrapping_add(0x1d0));
                pc = 0x80161e40;
            }
            0x80161e40 => { ins += 1;
                c.r[3] = c.r[0].rotate_left(1) & 0xfffffffe;
                pc = 0x80161e44;
            }
            0x80161e44 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x2000);
                pc = 0x80161e48;
            }
            0x80161e48 => { ins += 1;
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80161e4c;
            }
            0x80161e4c => { ins += 1;
                pc = 0x80161e7c; continue;
                pc = 0x80161e50;
            }
            0x80161e50 => { ins += 1;
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                pc = 0x80161e54;
            }
            0x80161e54 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                pc = 0x80161e58;
            }
            0x80161e58 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x80161e74; continue; }
                pc = 0x80161e5c;
            }
            0x80161e5c => { ins += 1;
                c.r[5] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                pc = 0x80161e60;
            }
            0x80161e60 => { ins += 1;
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80161e64;
            }
            0x80161e64 => { ins += 1;
                c.r[4] = c.r[0] | c.r[0];
                pc = 0x80161e68;
            }
            0x80161e68 => { ins += 1;
                c.lr = 0x80161e6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80160bdc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161e6c;
            }
            0x80161e6c => { ins += 1;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80161e70;
            }
            0x80161e70 => { ins += 1;
                pc = 0x80161e78; continue;
                pc = 0x80161e74;
            }
            0x80161e74 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x80161e78;
            }
            0x80161e78 => { ins += 1;
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80161e7c;
            }
            0x80161e7c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                pc = 0x80161e80;
            }
            0x80161e80 => { ins += 1;
                c.mtspr(8, c.r[0]);
                pc = 0x80161e84;
            }
            0x80161e84 => { ins += 1;
                c.r[1] = c.r[1].wrapping_add(0x10);
                pc = 0x80161e88;
            }
            0x80161e88 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80161d90: bad pc {:#x}", pc),
        }
    }
}

/// inpSetExCtrl (0x80161e8c, 484 bytes)
#[inline(never)]
pub fn f_80161e8c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80161e8c) { return; }
        if c.prof { crate::prof::enter(c, 0x80161e8c); p = true; }
    }
    f_80161e8c_b(c, 0x80161e8c);
    if p { crate::prof::exit(c, 0x80161e8c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80161e8c_at(c: &mut Cpu, pc: u32) { f_80161e8c_b(c, pc) }
#[inline(never)]
fn f_80161e8c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80161e8c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80161e8c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80161e8c => { ins += 1;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x80161e90;
            }
            0x80161e90 => { ins += 1;
                c.r[0] = c.mfspr(8);
                pc = 0x80161e94;
            }
            0x80161e94 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x80161e98;
            }
            0x80161e98 => { ins += 1;
                c.r[0] = c.r[5] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x80161e9c;
            }
            0x80161e9c => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                pc = 0x80161ea0;
            }
            0x80161ea0 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                pc = 0x80161ea4;
            }
            0x80161ea4 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                pc = 0x80161ea8;
            }
            0x80161ea8 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                pc = 0x80161eac;
            }
            0x80161eac => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80161eb8; continue; }
                pc = 0x80161eb0;
            }
            0x80161eb0 => { ins += 1;
                c.r[5] = 0x0;
                pc = 0x80161eb4;
            }
            0x80161eb4 => { ins += 1;
                pc = 0x80161ecc; continue;
                pc = 0x80161eb8;
            }
            0x80161eb8 => { ins += 1;
                c.r[0] = c.r[5] as u16 as i16 as i32 as u32;
                pc = 0x80161ebc;
            }
            0x80161ebc => { ins += 1;
                c.r[5] = 0x3fff;
                pc = 0x80161ec0;
            }
            0x80161ec0 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x3fff);
                pc = 0x80161ec4;
            }
            0x80161ec4 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x80161ecc; continue; }
                pc = 0x80161ec8;
            }
            0x80161ec8 => { ins += 1;
                c.r[5] = c.r[0] | c.r[0];
                pc = 0x80161ecc;
            }
            0x80161ecc => { ins += 1;
                c.r[7] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x80161ed0;
            }
            0x80161ed0 => { ins += 1;
                c.r[6] = c.r[4] | c.r[4];
                pc = 0x80161ed4;
            }
            0x80161ed4 => { ins += 1;
                c.r[0] = c.r[7].wrapping_add(0xffffff80);
                pc = 0x80161ed8;
            }
            0x80161ed8 => { ins += 1;
                c.r[8] = c.r[5] as u16 as i16 as i32 as u32;
                pc = 0x80161edc;
            }
            0x80161edc => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                pc = 0x80161ee0;
            }
            0x80161ee0 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x80161f40; continue; }
                pc = 0x80161ee4;
            }
            0x80161ee4 => { ins += 1;
                c.r[5] = 0x80370000;
                pc = 0x80161ee8;
            }
            0x80161ee8 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80161eec;
            }
            0x80161eec => { ins += 1;
                c.r[5] = c.r[5].wrapping_add(0xffff9cf8);
                pc = 0x80161ef0;
            }
            0x80161ef0 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(c.r[0]));
                pc = 0x80161ef4;
            }
            0x80161ef4 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x80161ef8;
            }
            0x80161ef8 => { ins += 1;
                if true { let t = c.ctr; if (0x80161e8c..0x80162070).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80161efc;
            }
            0x80161efc => { ins += 1;
                c.r[6] = 0x80;
                pc = 0x80161f00;
            }
            0x80161f00 => { ins += 1;
                pc = 0x80161f40; continue;
                pc = 0x80161f04;
            }
            0x80161f04 => { ins += 1;
                c.r[6] = 0x82;
                pc = 0x80161f08;
            }
            0x80161f08 => { ins += 1;
                pc = 0x80161f40; continue;
                pc = 0x80161f0c;
            }
            0x80161f0c => { ins += 1;
                c.r[6] = 0xa0;
                pc = 0x80161f10;
            }
            0x80161f10 => { ins += 1;
                pc = 0x80161f40; continue;
                pc = 0x80161f14;
            }
            0x80161f14 => { ins += 1;
                c.r[6] = 0xa1;
                pc = 0x80161f18;
            }
            0x80161f18 => { ins += 1;
                pc = 0x80161f40; continue;
                pc = 0x80161f1c;
            }
            0x80161f1c => { ins += 1;
                c.r[6] = 0x83;
                pc = 0x80161f20;
            }
            0x80161f20 => { ins += 1;
                pc = 0x80161f40; continue;
                pc = 0x80161f24;
            }
            0x80161f24 => { ins += 1;
                c.r[6] = 0x84;
                pc = 0x80161f28;
            }
            0x80161f28 => { ins += 1;
                pc = 0x80161f40; continue;
                pc = 0x80161f2c;
            }
            0x80161f2c => { ins += 1;
                c.r[6] = 0xa2;
                pc = 0x80161f30;
            }
            0x80161f30 => { ins += 1;
                pc = 0x80161f40; continue;
                pc = 0x80161f34;
            }
            0x80161f34 => { ins += 1;
                c.r[6] = 0xa3;
                pc = 0x80161f38;
            }
            0x80161f38 => { ins += 1;
                pc = 0x80161f40; continue;
                pc = 0x80161f3c;
            }
            0x80161f3c => { ins += 1;
                c.r[6] = 0xa4;
                pc = 0x80161f40;
            }
            0x80161f40 => { ins += 1;
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                pc = 0x80161f44;
            }
            0x80161f44 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xa2);
                pc = 0x80161f48;
            }
            0x80161f48 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80161f54; continue; }
                pc = 0x80161f4c;
            }
            0x80161f4c => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xa0);
                pc = 0x80161f50;
            }
            0x80161f50 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80162050; continue; }
                pc = 0x80161f54;
            }
            0x80161f54 => { ins += 1;
                c.r[29] = c.lbz_t(ins, c.r[3].wrapping_add(0x121));
                pc = 0x80161f58;
            }
            0x80161f58 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0xff);
                pc = 0x80161f5c;
            }
            0x80161f5c => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x80162050; continue; }
                pc = 0x80161f60;
            }
            0x80161f60 => { ins += 1;
                c.r[30] = c.lbz_t(ins, c.r[3].wrapping_add(0x122));
                pc = 0x80161f64;
            }
            0x80161f64 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x80162050; continue; }
                pc = 0x80161f68;
            }
            0x80161f68 => { ins += 1;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x80161f6c;
            }
            0x80161f6c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x40);
                pc = 0x80161f70;
            }
            0x80161f70 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80161fac; continue; }
                pc = 0x80161f74;
            }
            0x80161f74 => { ins += 1;
                c.r[28] = c.r[7].rotate_left(0) & 0x1f;
                pc = 0x80161f78;
            }
            0x80161f78 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80161f7c;
            }
            0x80161f7c => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x80161f80;
            }
            0x80161f80 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80161f84;
            }
            0x80161f84 => { ins += 1;
                c.r[31] = c.r[8].rotate_left(0) & 0xffff;
                pc = 0x80161f88;
            }
            0x80161f88 => { ins += 1;
                c.r[6] = c.r[8].rotate_left(25) & 0xff;
                pc = 0x80161f8c;
            }
            0x80161f8c => { ins += 1;
                c.lr = 0x80161f90; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161f90;
            }
            0x80161f90 => { ins += 1;
                c.r[0] = c.r[28].wrapping_add(0x20);
                pc = 0x80161f94;
            }
            0x80161f94 => { ins += 1;
                c.r[6] = c.r[31].rotate_left(0) & 0x7f;
                pc = 0x80161f98;
            }
            0x80161f98 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80161f9c;
            }
            0x80161f9c => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80161fa0;
            }
            0x80161fa0 => { ins += 1;
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x80161fa4;
            }
            0x80161fa4 => { ins += 1;
                c.lr = 0x80161fa8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161fa8;
            }
            0x80161fa8 => { ins += 1;
                pc = 0x80162050; continue;
                pc = 0x80161fac;
            }
            0x80161fac => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x80);
                pc = 0x80161fb0;
            }
            0x80161fb0 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x80161fbc; continue; }
                pc = 0x80161fb4;
            }
            0x80161fb4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x81);
                pc = 0x80161fb8;
            }
            0x80161fb8 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x80161ff4; continue; }
                pc = 0x80161fbc;
            }
            0x80161fbc => { ins += 1;
                c.r[28] = c.r[7].rotate_left(0) & 0xfe;
                pc = 0x80161fc0;
            }
            0x80161fc0 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80161fc4;
            }
            0x80161fc4 => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x80161fc8;
            }
            0x80161fc8 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80161fcc;
            }
            0x80161fcc => { ins += 1;
                c.r[31] = c.r[8].rotate_left(0) & 0xffff;
                pc = 0x80161fd0;
            }
            0x80161fd0 => { ins += 1;
                c.r[6] = c.r[8].rotate_left(25) & 0xff;
                pc = 0x80161fd4;
            }
            0x80161fd4 => { ins += 1;
                c.lr = 0x80161fd8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161fd8;
            }
            0x80161fd8 => { ins += 1;
                c.r[0] = c.r[28].wrapping_add(0x1);
                pc = 0x80161fdc;
            }
            0x80161fdc => { ins += 1;
                c.r[6] = c.r[31].rotate_left(0) & 0x7f;
                pc = 0x80161fe0;
            }
            0x80161fe0 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80161fe4;
            }
            0x80161fe4 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80161fe8;
            }
            0x80161fe8 => { ins += 1;
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x80161fec;
            }
            0x80161fec => { ins += 1;
                c.lr = 0x80161ff0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80161ff0;
            }
            0x80161ff0 => { ins += 1;
                pc = 0x80162050; continue;
                pc = 0x80161ff4;
            }
            0x80161ff4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x84);
                pc = 0x80161ff8;
            }
            0x80161ff8 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x80162004; continue; }
                pc = 0x80161ffc;
            }
            0x80161ffc => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x85);
                pc = 0x80162000;
            }
            0x80162000 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8016203c; continue; }
                pc = 0x80162004;
            }
            0x80162004 => { ins += 1;
                c.r[28] = c.r[7].rotate_left(0) & 0xfe;
                pc = 0x80162008;
            }
            0x80162008 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8016200c;
            }
            0x8016200c => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x80162010;
            }
            0x80162010 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80162014;
            }
            0x80162014 => { ins += 1;
                c.r[31] = c.r[8].rotate_left(0) & 0xffff;
                pc = 0x80162018;
            }
            0x80162018 => { ins += 1;
                c.r[6] = c.r[8].rotate_left(25) & 0xff;
                pc = 0x8016201c;
            }
            0x8016201c => { ins += 1;
                c.lr = 0x80162020; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162020;
            }
            0x80162020 => { ins += 1;
                c.r[0] = c.r[28].wrapping_add(0x1);
                pc = 0x80162024;
            }
            0x80162024 => { ins += 1;
                c.r[6] = c.r[31].rotate_left(0) & 0x7f;
                pc = 0x80162028;
            }
            0x80162028 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8016202c;
            }
            0x8016202c => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80162030;
            }
            0x80162030 => { ins += 1;
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x80162034;
            }
            0x80162034 => { ins += 1;
                c.lr = 0x80162038; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162038;
            }
            0x80162038 => { ins += 1;
                pc = 0x80162050; continue;
                pc = 0x8016203c;
            }
            0x8016203c => { ins += 1;
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80162040;
            }
            0x80162040 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80162044;
            }
            0x80162044 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80162048;
            }
            0x80162048 => { ins += 1;
                c.r[6] = c.r[8].rotate_left(25) & 0xff;
                pc = 0x8016204c;
            }
            0x8016204c => { ins += 1;
                c.lr = 0x80162050; { c.steps = st; c.instrs = ins; c.cr = cr; f_801603c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162050;
            }
            0x80162050 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                pc = 0x80162054;
            }
            0x80162054 => { ins += 1;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                pc = 0x80162058;
            }
            0x80162058 => { ins += 1;
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                pc = 0x8016205c;
            }
            0x8016205c => { ins += 1;
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                pc = 0x80162060;
            }
            0x80162060 => { ins += 1;
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                pc = 0x80162064;
            }
            0x80162064 => { ins += 1;
                c.mtspr(8, c.r[0]);
                pc = 0x80162068;
            }
            0x80162068 => { ins += 1;
                c.r[1] = c.r[1].wrapping_add(0x20);
                pc = 0x8016206c;
            }
            0x8016206c => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80161e8c: bad pc {:#x}", pc),
        }
    }
}

/// sndRand (0x80162070, 28 bytes)
#[inline(never)]
pub fn f_80162070(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162070) { return; }
        if c.prof { crate::prof::enter(c, 0x80162070); p = true; }
    }
    f_80162070_b(c, 0x80162070);
    if p { crate::prof::exit(c, 0x80162070); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162070_at(c: &mut Cpu, pc: u32) { f_80162070_b(c, pc) }
#[inline(always)]
fn f_80162070_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162070;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162070, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162070 => { ins += 7;
                c.r[3] = 0xa8350000;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff83d0));
                c.r[0] = c.r[3].wrapping_add(0x1d63);
                c.r[0] = c.r[4].wrapping_mul(c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff83d0), c.r[0]);
                c.r[3] = c.r[0].rotate_left(26) & 0xffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162070: bad pc {:#x}", pc),
        }
    }
}

/// sndSin (0x8016208c, 140 bytes)
#[inline(never)]
pub fn f_8016208c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016208c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016208c); p = true; }
    }
    f_8016208c_b(c, 0x8016208c);
    if p { crate::prof::exit(c, 0x8016208c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016208c_at(c: &mut Cpu, pc: u32) { f_8016208c_b(c, pc) }
#[inline(always)]
fn f_8016208c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016208c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016208c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016208c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xfff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x400);
                if !cr_bit(cr, 0) { pc = 0x801620ac; continue; }
                pc = 0x80162098;
            }
            0x80162098 => { ins += 5;
                c.r[3] = 0x80370000;
                c.r[0] = c.r[0].rotate_left(1) & 0x1fffe;
                c.r[3] = c.r[3].wrapping_add(0xffff9d20);
                c.r[3] = c.lha_t(ins, c.r[3].wrapping_add(c.r[0]));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801620ac;
            }
            0x801620ac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x800);
                if !cr_bit(cr, 0) { pc = 0x801620d0; continue; }
                pc = 0x801620b4;
            }
            0x801620b4 => { ins += 7;
                c.r[0] = c.r[0].rotate_left(0) & 0x3ff;
                c.r[3] = 0x80370000;
                c.r[0] = c.addc(!c.r[0], 0x3ff, 1);
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[3] = c.r[3].wrapping_add(0xffff9d20);
                c.r[3] = c.lha_t(ins, c.r[3].wrapping_add(c.r[0]));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801620d0;
            }
            0x801620d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc00);
                if !cr_bit(cr, 0) { pc = 0x801620f4; continue; }
                pc = 0x801620d8;
            }
            0x801620d8 => { ins += 7;
                c.r[3] = 0x80370000;
                c.r[0] = c.r[0].rotate_left(1) & 0x7fe;
                c.r[3] = c.r[3].wrapping_add(0xffff9d20);
                c.r[0] = c.lha_t(ins, c.r[3].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].wrapping_neg();
                c.r[3] = c.r[0] as u16 as i16 as i32 as u32;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801620f4;
            }
            0x801620f4 => { ins += 9;
                c.r[0] = c.r[0].rotate_left(0) & 0x3ff;
                c.r[3] = 0x80370000;
                c.r[0] = c.addc(!c.r[0], 0x3ff, 1);
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[3] = c.r[3].wrapping_add(0xffff9d20);
                c.r[0] = c.lha_t(ins, c.r[3].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].wrapping_neg();
                c.r[3] = c.r[0] as u16 as i16 as i32 as u32;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016208c: bad pc {:#x}", pc),
        }
    }
}

/// sndBSearch (0x80162118, 164 bytes)
#[inline(never)]
pub fn f_80162118(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162118) { return; }
        if c.prof { crate::prof::enter(c, 0x80162118); p = true; }
    }
    f_80162118_b(c, 0x80162118);
    if p { crate::prof::exit(c, 0x80162118); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162118_at(c: &mut Cpu, pc: u32) { f_80162118_b(c, pc) }
#[inline(always)]
fn f_80162118_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162118;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162118, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162118 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x8016212c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c47cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016212c;
            }
            0x8016212c => { ins += 6;
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0x0);
                c.r[24] = c.r[3] | c.r[3];
                c.r[25] = c.r[4] | c.r[4];
                c.r[26] = c.r[6] | c.r[6];
                c.r[27] = c.r[7] | c.r[7];
                if cr_bit(cr, 2) { pc = 0x801621a0; continue; }
                pc = 0x80162144;
            }
            0x80162144 => { ins += 2;
                c.r[31] = 0x1;
                c.r[30] = c.r[5] | c.r[5];
                pc = 0x8016214c;
            }
            0x8016214c => { ins += 10;
                c.r[0] = c.r[31].wrapping_add(c.r[30]);
                c.r[12] = c.r[27] | c.r[27];
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[29] = (v >> 1) as u32; }
                c.r[3] = c.r[24] | c.r[24];
                c.r[0] = c.r[29].wrapping_add(0xffffffff);
                c.r[0] = c.r[26].wrapping_mul(c.r[0]);
                c.r[28] = c.r[25].wrapping_add(c.r[0]);
                c.r[4] = c.r[28] | c.r[28];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80162174; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x80162174;
            }
            0x80162174 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80162184; continue; }
                pc = 0x8016217c;
            }
            0x8016217c => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x801621a4; continue;
                pc = 0x80162184;
            }
            0x80162184 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8016218c; continue; }
                pc = 0x80162188;
            }
            0x80162188 => { ins += 1;
                c.r[30] = c.r[29].wrapping_add(0xffffffff);
                pc = 0x8016218c;
            }
            0x8016218c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(1) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80162198; continue; }
                pc = 0x80162194;
            }
            0x80162194 => { ins += 1;
                c.r[31] = c.r[29].wrapping_add(0x1);
                pc = 0x80162198;
            }
            0x80162198 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], c.r[30]);
                if !cr_bit(cr, 1) { pc = 0x8016214c; continue; }
                pc = 0x801621a0;
            }
            0x801621a0 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801621a4;
            }
            0x801621a4 => { ins += 2;
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x801621ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c4818(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801621ac;
            }
            0x801621ac => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162118: bad pc {:#x}", pc),
        }
    }
}

/// sndConvertMs (0x801621bc, 16 bytes)
#[inline(never)]
pub fn f_801621bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801621bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801621bc); p = true; }
    }
    f_801621bc_b(c, 0x801621bc);
    if p { crate::prof::exit(c, 0x801621bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801621bc_at(c: &mut Cpu, pc: u32) { f_801621bc_b(c, pc) }
#[inline(always)]
fn f_801621bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801621bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801621bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801621bc => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[3]);
                c.r[0] = c.r[0].rotate_left(8) & 0xffffff00;
                c.stw(c.r[3], c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801621bc: bad pc {:#x}", pc),
        }
    }
}

/// sndConvertTicks (0x801621cc, 72 bytes)
#[inline(never)]
pub fn f_801621cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801621cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801621cc); p = true; }
    }
    f_801621cc_b(c, 0x801621cc);
    if p { crate::prof::exit(c, 0x801621cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801621cc_at(c: &mut Cpu, pc: u32) { f_801621cc_b(c, pc) }
#[inline(always)]
fn f_801621cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801621cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801621cc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801621cc => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[4] | c.r[4];
                c.lr = 0x801621e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014a280(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801621e8;
            }
            0x801621e8 => { ins += 11;
                c.r[0] = c.lwz_t(ins, c.r[31]);
                c.r[0] = c.r[0].rotate_left(16) & 0xffff0000;
                c.r[0] = divwu(c.r[0], c.r[3]);
                c.r[0] = c.r[0].wrapping_mul(0x3e8);
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.stw(c.r[31], c.r[0]);
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801621cc: bad pc {:#x}", pc),
        }
    }
}

/// sndConvert2Ms (0x80162214, 8 bytes)
#[inline(never)]
pub fn f_80162214(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162214) { return; }
        if c.prof { crate::prof::enter(c, 0x80162214); p = true; }
    }
    f_80162214_b(c, 0x80162214);
    if p { crate::prof::exit(c, 0x80162214); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162214_at(c: &mut Cpu, pc: u32) { f_80162214_b(c, pc) }
#[inline(always)]
fn f_80162214_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162214;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162214, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162214 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(24) & 0xffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162214: bad pc {:#x}", pc),
        }
    }
}

/// snd_handle_irq (0x8016221c, 340 bytes)
#[inline(never)]
pub fn f_8016221c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016221c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016221c); p = true; }
    }
    f_8016221c_b(c, 0x8016221c);
    if p { crate::prof::exit(c, 0x8016221c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016221c_at(c: &mut Cpu, pc: u32) { f_8016221c_b(c, pc) }
#[inline(never)]
fn f_8016221c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016221c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016221c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016221c => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa6f8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x8016235c; continue; }
                pc = 0x80162238;
            }
            0x80162238 => { ins += 1;
                c.lr = 0x8016223c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014e7cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016223c;
            }
            0x8016223c => { ins += 1;
                c.lr = 0x80162240; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164398(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162240;
            }
            0x80162240 => { ins += 1;
                c.lr = 0x80162244; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164118(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162244;
            }
            0x80162244 => { ins += 1;
                c.lr = 0x80162248; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164238(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162248;
            }
            0x80162248 => { ins += 1;
                c.lr = 0x8016224c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643b8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016224c;
            }
            0x8016224c => { ins += 1;
                c.lr = 0x80162250; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164398(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162250;
            }
            0x80162250 => { ins += 1;
                c.lr = 0x80162254; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015d678(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162254;
            }
            0x80162254 => { ins += 1;
                c.lr = 0x80162258; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643b8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162258;
            }
            0x80162258 => { ins += 1;
                c.lr = 0x8016225c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164398(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016225c;
            }
            0x8016225c => { ins += 16;
                c.r[4] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa83e));
                c.r[3] = 0x55550000;
                c.r[0] = c.r[3].wrapping_add(0x5556);
                c.r[5] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa83f));
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
            0x8016229c => { ins += 18;
                c.r[0] = c.r[9].rotate_left(0) & 0xff;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[0].wrapping_mul(0xf4);
                c.r[9] = c.r[9].wrapping_add(0x1);
                c.r[7] = c.r[4].wrapping_add(0x24);
                c.stw(c.r[3].wrapping_add(c.r[7]), c.r[8]);
                c.r[5] = c.r[7].wrapping_add(0x4);
                c.r[4] = c.r[7].wrapping_add(0x8);
                c.r[3] = c.r[7].wrapping_add(0xc);
                c.r[6] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[0] = c.r[7].wrapping_add(0x10);
                c.stw(c.r[6].wrapping_add(c.r[5]), c.r[8]);
                c.r[5] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.stw(c.r[5].wrapping_add(c.r[4]), c.r[8]);
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.stw(c.r[4].wrapping_add(c.r[3]), c.r[8]);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[8]);
                pc = 0x801622e4;
            }
            0x801622e4 => { ins += 4;
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa83d));
                c.r[3] = c.r[9].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8016229c; continue; }
                pc = 0x801622f4;
            }
            0x801622f4 => { ins += 1;
                c.lr = 0x801622f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643b8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801622f8;
            }
            0x801622f8 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80162324; continue;
                pc = 0x80162300;
            }
            0x80162300 => { ins += 1;
                c.lr = 0x80162304; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164398(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162304;
            }
            0x80162304 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8016230c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8016245c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016230c;
            }
            0x8016230c => { ins += 2;
                c.r[3] = 0x100;
                c.lr = 0x80162314; { c.steps = st; c.instrs = ins; c.cr = cr; f_801496a0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162314;
            }
            0x80162314 => { ins += 2;
                c.r[3] = 0x100;
                c.lr = 0x8016231c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014c0a4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016231c;
            }
            0x8016231c => { ins += 1;
                c.lr = 0x80162320; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643b8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162320;
            }
            0x80162320 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80162324;
            }
            0x80162324 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x5);
                if cr_bit(cr, 0) { pc = 0x80162300; continue; }
                pc = 0x80162330;
            }
            0x80162330 => { ins += 1;
                c.lr = 0x80162334; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164398(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162334;
            }
            0x80162334 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8016233c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8016245c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016233c;
            }
            0x8016233c => { ins += 1;
                c.lr = 0x80162340; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015f620(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162340;
            }
            0x80162340 => { ins += 1;
                c.lr = 0x80162344; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643b8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162344;
            }
            0x80162344 => { ins += 1;
                c.lr = 0x80162348; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164398(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162348;
            }
            0x80162348 => { ins += 1;
                c.lr = 0x8016234c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014df20(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016234c;
            }
            0x8016234c => { ins += 1;
                c.lr = 0x80162350; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643b8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162350;
            }
            0x80162350 => { ins += 1;
                c.lr = 0x80162354; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164398(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162354;
            }
            0x80162354 => { ins += 1;
                c.lr = 0x80162358; { c.steps = st; c.instrs = ins; c.cr = cr; f_80159a38(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162358;
            }
            0x80162358 => { ins += 1;
                c.lr = 0x8016235c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643b8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016235c;
            }
            0x8016235c => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016221c: bad pc {:#x}", pc),
        }
    }
}

/// hwInit (0x80162370, 184 bytes)
#[inline(never)]
pub fn f_80162370(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162370) { return; }
        if c.prof { crate::prof::enter(c, 0x80162370); p = true; }
    }
    f_80162370_b(c, 0x80162370);
    if p { crate::prof::exit(c, 0x80162370); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162370_at(c: &mut Cpu, pc: u32) { f_80162370_b(c, pc) }
#[inline(always)]
fn f_80162370_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162370;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162370, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162370 => { ins += 12;
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
                c.lr = 0x801623a0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801642f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801623a0;
            }
            0x801623a0 => { ins += 9;
                c.r[0] = 0x0;
                c.r[3] = 0x80160000;
                c.stb(c.r[13].wrapping_add(0xffffa83f), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x221c);
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.stb(c.r[13].wrapping_add(0xffffa83e), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa808), c.r[0]);
                c.lr = 0x801623c4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80163ffc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801623c4;
            }
            0x801623c4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80162404; continue; }
                pc = 0x801623cc;
            }
            0x801623cc => { ins += 4;
                c.r[5] = c.r[31].rotate_left(0) & 0x1;
                c.r[3] = c.r[29].rotate_left(0) & 0xff;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                c.lr = 0x801623dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015a484(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801623dc;
            }
            0x801623dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80162404; continue; }
                pc = 0x801623e4;
            }
            0x801623e4 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801623ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164148(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801623ec;
            }
            0x801623ec => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80162404; continue; }
                pc = 0x801623f4;
            }
            0x801623f4 => { ins += 1;
                c.lr = 0x801623f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164328(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801623f8;
            }
            0x801623f8 => { ins += 1;
                c.lr = 0x801623fc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801640c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801623fc;
            }
            0x801623fc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80162408; continue;
                pc = 0x80162404;
            }
            0x80162404 => { ins += 1;
                c.r[3] = 0xffffffff;
                pc = 0x80162408;
            }
            0x80162408 => { ins += 8;
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
            _ => panic!("f_80162370: bad pc {:#x}", pc),
        }
    }
}

/// hwExit (0x80162428, 52 bytes)
#[inline(never)]
pub fn f_80162428(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162428) { return; }
        if c.prof { crate::prof::enter(c, 0x80162428); p = true; }
    }
    f_80162428_b(c, 0x80162428);
    if p { crate::prof::exit(c, 0x80162428); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162428_at(c: &mut Cpu, pc: u32) { f_80162428_b(c, pc) }
#[inline(always)]
fn f_80162428_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162428;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162428, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162428 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80162438; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164360(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162438;
            }
            0x80162438 => { ins += 1;
                c.lr = 0x8016243c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164204(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016243c;
            }
            0x8016243c => { ins += 1;
                c.lr = 0x80162440; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015a870(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162440;
            }
            0x80162440 => { ins += 1;
                c.lr = 0x80162444; { c.steps = st; c.instrs = ins; c.cr = cr; f_801640e4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162444;
            }
            0x80162444 => { ins += 1;
                c.lr = 0x80162448; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164328(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162448;
            }
            0x80162448 => { ins += 1;
                c.lr = 0x8016244c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164324(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016244c;
            }
            0x8016244c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162428: bad pc {:#x}", pc),
        }
    }
}

/// hwSetTimeOffset (0x8016245c, 8 bytes)
#[inline(never)]
pub fn f_8016245c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016245c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016245c); p = true; }
    }
    f_8016245c_b(c, 0x8016245c);
    if p { crate::prof::exit(c, 0x8016245c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016245c_at(c: &mut Cpu, pc: u32) { f_8016245c_b(c, pc) }
#[inline(always)]
fn f_8016245c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016245c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016245c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016245c => { ins += 2;
                c.stb(c.r[13].wrapping_add(0xffffa830), c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016245c: bad pc {:#x}", pc),
        }
    }
}

/// hwGetTimeOffset (0x80162464, 8 bytes)
#[inline(never)]
pub fn f_80162464(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162464) { return; }
        if c.prof { crate::prof::enter(c, 0x80162464); p = true; }
    }
    f_80162464_b(c, 0x80162464);
    if p { crate::prof::exit(c, 0x80162464); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162464_at(c: &mut Cpu, pc: u32) { f_80162464_b(c, pc) }
#[inline(always)]
fn f_80162464_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162464;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162464, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162464 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa830));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162464: bad pc {:#x}", pc),
        }
    }
}

/// hwIsActive (0x8016246c, 32 bytes)
#[inline(never)]
pub fn f_8016246c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016246c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016246c); p = true; }
    }
    f_8016246c_b(c, 0x8016246c);
    if p { crate::prof::exit(c, 0x8016246c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016246c_at(c: &mut Cpu, pc: u32) { f_8016246c_b(c, pc) }
#[inline(always)]
fn f_8016246c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016246c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016246c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016246c => { ins += 8;
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xec));
                c.r[0] = c.r[3].wrapping_neg();
                c.r[0] = c.r[0] | c.r[3];
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016246c: bad pc {:#x}", pc),
        }
    }
}

/// hwSetMesgCallback (0x8016248c, 8 bytes)
#[inline(never)]
pub fn f_8016248c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016248c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016248c); p = true; }
    }
    f_8016248c_b(c, 0x8016248c);
    if p { crate::prof::exit(c, 0x8016248c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016248c_at(c: &mut Cpu, pc: u32) { f_8016248c_b(c, pc) }
#[inline(always)]
fn f_8016248c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016248c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016248c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016248c => { ins += 2;
                c.stw(c.r[13].wrapping_add(0xffffa808), c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016248c: bad pc {:#x}", pc),
        }
    }
}

/// hwSetPriority (0x80162494, 20 bytes)
#[inline(never)]
pub fn f_80162494(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162494) { return; }
        if c.prof { crate::prof::enter(c, 0x80162494); p = true; }
    }
    f_80162494_b(c, 0x80162494);
    if p { crate::prof::exit(c, 0x80162494); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162494_at(c: &mut Cpu, pc: u32) { f_80162494_b(c, pc) }
#[inline(always)]
fn f_80162494_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162494;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162494, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162494 => { ins += 5;
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1c), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162494: bad pc {:#x}", pc),
        }
    }
}

/// hwInitSamplePlayback (0x801624a8, 436 bytes)
#[inline(never)]
pub fn f_801624a8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801624a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801624a8); p = true; }
    }
    f_801624a8_b(c, 0x801624a8);
    if p { crate::prof::exit(c, 0x801624a8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801624a8_at(c: &mut Cpu, pc: u32) { f_801624a8_b(c, pc) }
#[inline(always)]
fn f_801624a8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801624a8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801624a8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801624a8 => { ins += 13;
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
            0x801624dc => { ins += 9;
                c.r[10] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[0] = c.r[12].rotate_left(2) & 0x3fc;
                c.r[12] = c.r[12].wrapping_add(0x1);
                c.r[10] = c.r[10].wrapping_add(c.r[0]);
                c.r[10] = c.r[10].wrapping_add(0x24);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(c.r[10]));
                c.r[0] = c.r[0].rotate_left(0) & 0x20;
                c.stw(c.r[3].wrapping_add(c.r[10]), c.r[11]);
                c.r[29] = c.r[29] | c.r[0];
                pc = 0x80162500;
            }
            0x80162500 => { ins += 4;
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa830));
                c.r[10] = c.r[12].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[10], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x801624dc; continue; }
                pc = 0x80162510;
            }
            0x80162510 => { ins += 36;
                c.r[10] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[0] = 0x0;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x0);
                c.r[6] = c.r[10].wrapping_add(c.r[3]);
                c.stw(c.r[6].wrapping_add(0x24), c.r[29]);
                c.r[6] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[6] = c.r[6].wrapping_add(c.r[3]);
                c.stw(c.r[6].wrapping_add(0x1c), c.r[7]);
                c.r[6] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[6] = c.r[6].wrapping_add(c.r[3]);
                c.stw(c.r[6].wrapping_add(0x18), c.r[8]);
                c.r[6] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[6] = c.r[6].wrapping_add(c.r[3]);
                c.stw(c.r[6].wrapping_add(0xf0), c.r[0]);
                c.r[6] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[6] = c.r[6].wrapping_add(c.r[3]);
                c.sth(c.r[6].wrapping_add(0x70), c.r[4]);
                c.r[7] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[6] = c.lwz_t(ins, c.r[5]);
                c.r[4] = c.lwz_t(ins, c.r[5].wrapping_add(0x4));
                c.r[7] = c.r[7].wrapping_add(c.r[3]);
                c.stw(c.r[7].wrapping_add(0x74), c.r[6]);
                c.stw(c.r[7].wrapping_add(0x78), c.r[4]);
                c.r[6] = c.lwz_t(ins, c.r[5].wrapping_add(0x8));
                c.r[4] = c.lwz_t(ins, c.r[5].wrapping_add(0xc));
                c.stw(c.r[7].wrapping_add(0x7c), c.r[6]);
                c.stw(c.r[7].wrapping_add(0x80), c.r[4]);
                c.r[6] = c.lwz_t(ins, c.r[5].wrapping_add(0x10));
                c.r[4] = c.lwz_t(ins, c.r[5].wrapping_add(0x14));
                c.stw(c.r[7].wrapping_add(0x84), c.r[6]);
                c.stw(c.r[7].wrapping_add(0x88), c.r[4]);
                c.r[6] = c.lwz_t(ins, c.r[5].wrapping_add(0x18));
                c.r[4] = c.lwz_t(ins, c.r[5].wrapping_add(0x1c));
                c.stw(c.r[7].wrapping_add(0x8c), c.r[6]);
                c.stw(c.r[7].wrapping_add(0x90), c.r[4]);
                if cr_bit(cr, 2) { pc = 0x801625e0; continue; }
                pc = 0x801625a0;
            }
            0x801625a0 => { ins += 16;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[5] = 0x7fff;
                c.r[4] = c.r[4].wrapping_add(c.r[3]);
                c.stb(c.r[4].wrapping_add(0xa4), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[4].wrapping_add(c.r[3]);
                c.stw(c.r[4].wrapping_add(0xb8), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[4].wrapping_add(c.r[3]);
                c.stw(c.r[4].wrapping_add(0xbc), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[4].wrapping_add(c.r[3]);
                c.sth(c.r[4].wrapping_add(0xc0), c.r[5]);
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[4].wrapping_add(c.r[3]);
                c.stw(c.r[4].wrapping_add(0xc4), c.r[0]);
                pc = 0x801625e0;
            }
            0x801625e0 => { ins += 15;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[5] = 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[9], 0x0);
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                c.stb(c.r[4].wrapping_add(0xe4), c.r[5]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                c.stb(c.r[4].wrapping_add(0xe5), c.r[5]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                c.stb(c.r[4].wrapping_add(0xe6), c.r[5]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[3]);
                c.stb(c.r[3].wrapping_add(0xe7), c.r[5]);
                if cr_bit(cr, 2) { pc = 0x80162634; continue; }
                pc = 0x8016261c;
            }
            0x8016261c => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x80162628; { c.steps = st; c.instrs = ins; c.cr = cr; f_801629a4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162628;
            }
            0x80162628 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x1;
                c.lr = 0x80162634; { c.steps = st; c.instrs = ins; c.cr = cr; f_801629d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162634;
            }
            0x80162634 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80162640; { c.steps = st; c.instrs = ins; c.cr = cr; f_801629fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162640;
            }
            0x80162640 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801624a8: bad pc {:#x}", pc),
        }
    }
}

/// hwBreak (0x8016265c, 80 bytes)
#[inline(never)]
pub fn f_8016265c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016265c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016265c); p = true; }
    }
    f_8016265c_b(c, 0x8016265c);
    if p { crate::prof::exit(c, 0x8016265c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016265c_at(c: &mut Cpu, pc: u32) { f_8016265c_b(c, pc) }
#[inline(always)]
fn f_8016265c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016265c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016265c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016265c => { ins += 6;
                c.r[4] = c.r[3].wrapping_mul(0xf4);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0xec));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80162688; continue; }
                pc = 0x80162674;
            }
            0x80162674 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa830));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80162688; continue; }
                pc = 0x80162680;
            }
            0x80162680 => { ins += 2;
                c.r[0] = 0x1;
                c.stb(c.r[3].wrapping_add(0xee), c.r[0]);
                pc = 0x80162688;
            }
            0x80162688 => { ins += 9;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa830));
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x20;
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016265c: bad pc {:#x}", pc),
        }
    }
}

/// hwSetADSR (0x801626ac, 428 bytes)
#[inline(never)]
pub fn f_801626ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801626ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801626ac); p = true; }
    }
    f_801626ac_b(c, 0x801626ac);
    if p { crate::prof::exit(c, 0x801626ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801626ac_at(c: &mut Cpu, pc: u32) { f_801626ac_b(c, pc) }
#[inline(always)]
fn f_801626ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801626ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801626ac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801626ac => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = c.r[5].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[6]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                if cr_bit(cr, 2) { pc = 0x801626e4; continue; }
                pc = 0x801626d4;
            }
            0x801626d4 => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x80162824; continue; }
                pc = 0x801626d8;
            }
            0x801626d8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0x3);
                if !cr_bit(cr, 0) { pc = 0x80162824; continue; }
                pc = 0x801626e0;
            }
            0x801626e0 => { ins += 1;
                pc = 0x8016274c; continue;
                pc = 0x801626e4;
            }
            0x801626e4 => { ins += 17;
                c.r[5] = c.r[29].wrapping_mul(0xf4);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = 0x0;
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stb(c.r[3].wrapping_add(0xa4), c.r[4]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.lhz_t(ins, c.r[30]);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stw(c.r[3].wrapping_add(0xb8), c.r[4]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.lhz_t(ins, c.r[30].wrapping_add(0x2));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stw(c.r[3].wrapping_add(0xbc), c.r[4]);
                c.r[0] = c.lhz_t(ins, c.r[30].wrapping_add(0x4));
                c.r[4] = c.r[0].rotate_left(3) & 0xfffffff8;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x7fff);
                if !cr_bit(cr, 1) { pc = 0x8016272c; continue; }
                pc = 0x80162728;
            }
            0x80162728 => { ins += 1;
                c.r[4] = 0x7fff;
                pc = 0x8016272c;
            }
            0x8016272c => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.sth(c.r[3].wrapping_add(0xc0), c.r[4]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.lhz_t(ins, c.r[30].wrapping_add(0x6));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stw(c.r[3].wrapping_add(0xc4), c.r[4]);
                pc = 0x80162824; continue;
                pc = 0x8016274c;
            }
            0x8016274c => { ins += 11;
                c.r[31] = c.r[29].wrapping_mul(0xf4);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[5] = 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x1);
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.r[4] = 0x0;
                c.stb(c.r[3].wrapping_add(0xa4), c.r[5]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.stb(c.r[3].wrapping_add(0xca), c.r[4]);
                if !cr_bit(cr, 2) { pc = 0x801627dc; continue; }
                pc = 0x80162778;
            }
            0x80162778 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[30]);
                c.lr = 0x80162780; { c.steps = st; c.instrs = ins; c.cr = cr; f_80158cd4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162780;
            }
            0x80162780 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[3].wrapping_add(0xb8), c.r[4]);
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x4));
                c.lr = 0x80162798; { c.steps = st; c.instrs = ins; c.cr = cr; f_80158cd4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162798;
            }
            0x80162798 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[3].wrapping_add(0xbc), c.r[4]);
                c.r[0] = c.lhz_t(ins, c.r[30].wrapping_add(0x8));
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 30 != 0); c.r[4] = (v >> 2) as u32; }
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x3ff);
                if !cr_bit(cr, 1) { pc = 0x801627bc; continue; }
                pc = 0x801627b8;
            }
            0x801627b8 => { ins += 1;
                c.r[4] = 0x3ff;
                pc = 0x801627bc;
            }
            0x801627bc => { ins += 8;
                c.r[3] = 0x80370000;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(0xffff944c);
                c.r[4] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[4]));
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.r[0] = c.addc(!c.r[4], 0xc1, 1);
                c.sth(c.r[3].wrapping_add(0xc0), c.r[0]);
                pc = 0x80162814; continue;
                pc = 0x801627dc;
            }
            0x801627dc => { ins += 14;
                c.r[3] = c.lwz_t(ins, c.r[30]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[3].wrapping_add(0xb8), c.r[4]);
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x4));
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[3].wrapping_add(0xbc), c.r[4]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.lhz_t(ins, c.r[30].wrapping_add(0x8));
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.sth(c.r[3].wrapping_add(0xc0), c.r[4]);
                pc = 0x80162814;
            }
            0x80162814 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.lhz_t(ins, c.r[30].wrapping_add(0xa));
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[3].wrapping_add(0xc4), c.r[4]);
                pc = 0x80162824;
            }
            0x80162824 => { ins += 13;
                c.r[0] = c.r[29].wrapping_mul(0xf4);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x10;
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801626ac: bad pc {:#x}", pc),
        }
    }
}

/// hwSetVirtualSampleLoopBuffer (0x80162858, 32 bytes)
#[inline(never)]
pub fn f_80162858(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162858) { return; }
        if c.prof { crate::prof::enter(c, 0x80162858); p = true; }
    }
    f_80162858_b(c, 0x80162858);
    if p { crate::prof::exit(c, 0x80162858); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162858_at(c: &mut Cpu, pc: u32) { f_80162858_b(c, pc) }
#[inline(always)]
fn f_80162858_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162858;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162858, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162858 => { ins += 8;
                c.r[6] = c.r[3].wrapping_mul(0xf4);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                c.stw(c.r[3].wrapping_add(0x94), c.r[4]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                c.stw(c.r[3].wrapping_add(0x98), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162858: bad pc {:#x}", pc),
        }
    }
}

/// hwGetVirtualSampleState (0x80162878, 20 bytes)
#[inline(never)]
pub fn f_80162878(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162878) { return; }
        if c.prof { crate::prof::enter(c, 0x80162878); p = true; }
    }
    f_80162878_b(c, 0x80162878);
    if p { crate::prof::exit(c, 0x80162878); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162878_at(c: &mut Cpu, pc: u32) { f_80162878_b(c, pc) }
#[inline(always)]
fn f_80162878_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162878;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162878, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162878 => { ins += 5;
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x9c));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162878: bad pc {:#x}", pc),
        }
    }
}

/// hwGetSampleType (0x8016288c, 20 bytes)
#[inline(never)]
pub fn f_8016288c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016288c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016288c); p = true; }
    }
    f_8016288c_b(c, 0x8016288c);
    if p { crate::prof::exit(c, 0x8016288c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016288c_at(c: &mut Cpu, pc: u32) { f_8016288c_b(c, pc) }
#[inline(always)]
fn f_8016288c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016288c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016288c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016288c => { ins += 5;
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x90));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016288c: bad pc {:#x}", pc),
        }
    }
}

/// hwGetSampleID (0x801628a0, 20 bytes)
#[inline(never)]
pub fn f_801628a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801628a0) { return; }
        if c.prof { crate::prof::enter(c, 0x801628a0); p = true; }
    }
    f_801628a0_b(c, 0x801628a0);
    if p { crate::prof::exit(c, 0x801628a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801628a0_at(c: &mut Cpu, pc: u32) { f_801628a0_b(c, pc) }
#[inline(always)]
fn f_801628a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801628a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801628a0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801628a0 => { ins += 5;
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x70));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801628a0: bad pc {:#x}", pc),
        }
    }
}

/// hwSetStreamLoopPS (0x801628b4, 20 bytes)
#[inline(never)]
pub fn f_801628b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801628b4) { return; }
        if c.prof { crate::prof::enter(c, 0x801628b4); p = true; }
    }
    f_801628b4_b(c, 0x801628b4);
    if p { crate::prof::exit(c, 0x801628b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801628b4_at(c: &mut Cpu, pc: u32) { f_801628b4_b(c, pc) }
#[inline(always)]
fn f_801628b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801628b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801628b4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801628b4 => { ins += 5;
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0xa0), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801628b4: bad pc {:#x}", pc),
        }
    }
}

/// hwStart (0x801628c8, 60 bytes)
#[inline(never)]
pub fn f_801628c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801628c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801628c8); p = true; }
    }
    f_801628c8_b(c, 0x801628c8);
    if p { crate::prof::exit(c, 0x801628c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801628c8_at(c: &mut Cpu, pc: u32) { f_801628c8_b(c, pc) }
#[inline(always)]
fn f_801628c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801628c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801628c8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801628c8 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = c.r[3].wrapping_mul(0xf4);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[5] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa830));
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                c.stb(c.r[3].wrapping_add(0xd4), c.r[5]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                c.lr = 0x801628f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015d44c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801628f4;
            }
            0x801628f4 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801628c8: bad pc {:#x}", pc),
        }
    }
}

/// hwKeyOff (0x80162904, 40 bytes)
#[inline(never)]
pub fn f_80162904(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162904) { return; }
        if c.prof { crate::prof::enter(c, 0x80162904); p = true; }
    }
    f_80162904_b(c, 0x80162904);
    if p { crate::prof::exit(c, 0x80162904); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162904_at(c: &mut Cpu, pc: u32) { f_80162904_b(c, pc) }
#[inline(always)]
fn f_80162904_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162904;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162904, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162904 => { ins += 10;
                c.r[3] = c.r[3].wrapping_mul(0xf4);
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa830));
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x40;
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162904: bad pc {:#x}", pc),
        }
    }
}

/// hwSetPitch (0x8016292c, 120 bytes)
#[inline(never)]
pub fn f_8016292c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016292c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016292c); p = true; }
    }
    f_8016292c_b(c, 0x8016292c);
    if p { crate::prof::exit(c, 0x8016292c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016292c_at(c: &mut Cpu, pc: u32) { f_8016292c_b(c, pc) }
#[inline(always)]
fn f_8016292c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016292c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016292c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016292c => { ins += 6;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.r[5] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_mul(0xf4);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4000);
                c.r[5] = c.r[5].wrapping_add(c.r[3]);
                if cr_bit(cr, 0) { pc = 0x80162948; continue; }
                pc = 0x80162944;
            }
            0x80162944 => { ins += 1;
                c.r[4] = 0x3fff;
                pc = 0x80162948;
            }
            0x80162948 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[5].wrapping_add(0xe4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x8016296c; continue; }
                pc = 0x80162954;
            }
            0x80162954 => { ins += 6;
                c.r[3] = c.r[0].rotate_left(2) & 0x3fc;
                c.r[0] = c.r[4].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x38));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x8016296c;
            }
            0x8016296c => { ins += 14;
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa830));
                c.r[4] = c.r[4].rotate_left(4) & 0xffff0;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[5].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x38), c.r[4]);
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa830));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x8;
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa830));
                c.stb(c.r[5].wrapping_add(0xe4), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016292c: bad pc {:#x}", pc),
        }
    }
}

/// hwSetSRCType (0x801629a4, 44 bytes)
#[inline(never)]
pub fn f_801629a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801629a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801629a4); p = true; }
    }
    f_801629a4_b(c, 0x801629a4);
    if p { crate::prof::exit(c, 0x801629a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801629a4_at(c: &mut Cpu, pc: u32) { f_801629a4_b(c, pc) }
#[inline(always)]
fn f_801629a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801629a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801629a4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801629a4 => { ins += 11;
                c.r[5] = c.r[3].wrapping_mul(0xf4);
                c.r[6] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[13].wrapping_add(0xffff83d8);
                c.r[0] = c.lhz_t(ins, c.r[3].wrapping_add(c.r[0]));
                c.r[3] = c.r[6].wrapping_add(c.r[5]);
                c.sth(c.r[3].wrapping_add(0xcc), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x100;
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801629a4: bad pc {:#x}", pc),
        }
    }
}

/// hwSetPolyPhaseFilter (0x801629d0, 44 bytes)
#[inline(never)]
pub fn f_801629d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801629d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801629d0); p = true; }
    }
    f_801629d0_b(c, 0x801629d0);
    if p { crate::prof::exit(c, 0x801629d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801629d0_at(c: &mut Cpu, pc: u32) { f_801629d0_b(c, pc) }
#[inline(always)]
fn f_801629d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801629d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801629d0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801629d0 => { ins += 11;
                c.r[5] = c.r[3].wrapping_mul(0xf4);
                c.r[6] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[13].wrapping_add(0xffff83e0);
                c.r[0] = c.lhz_t(ins, c.r[3].wrapping_add(c.r[0]));
                c.r[3] = c.r[6].wrapping_add(c.r[5]);
                c.sth(c.r[3].wrapping_add(0xce), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x80;
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801629d0: bad pc {:#x}", pc),
        }
    }
}

/// hwSetITDMode (0x801629fc, 92 bytes)
#[inline(never)]
pub fn f_801629fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801629fc) { return; }
        if c.prof { crate::prof::enter(c, 0x801629fc); p = true; }
    }
    f_801629fc_b(c, 0x801629fc);
    if p { crate::prof::exit(c, 0x801629fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801629fc_at(c: &mut Cpu, pc: u32) { f_801629fc_b(c, pc) }
#[inline(always)]
fn f_801629fc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801629fc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801629fc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801629fc => { ins += 2;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80162a3c; continue; }
                pc = 0x80162a04;
            }
            0x80162a04 => { ins += 14;
                c.r[5] = c.r[3].wrapping_mul(0xf4);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = 0x10;
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0xf0));
                c.r[0] = c.r[0] | 0x80000000;
                c.stw(c.r[3].wrapping_add(0xf0), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.sth(c.r[3].wrapping_add(0xd0), c.r[4]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.sth(c.r[3].wrapping_add(0xd2), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80162a3c;
            }
            0x80162a3c => { ins += 7;
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0xf0));
                c.r[0] = c.r[0].rotate_left(0) & 0x7fffffff;
                c.stw(c.r[3].wrapping_add(0xf0), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801629fc: bad pc {:#x}", pc),
        }
    }
}

/// hwSetVolume (0x80162a58, 704 bytes)
#[inline(never)]
pub fn f_80162a58(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162a58) { return; }
        if c.prof { crate::prof::enter(c, 0x80162a58); p = true; }
    }
    f_80162a58_b(c, 0x80162a58);
    if p { crate::prof::exit(c, 0x80162a58); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162a58_at(c: &mut Cpu, pc: u32) { f_80162a58_b(c, pc) }
#[inline(never)]
fn f_80162a58_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162a58;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162a58, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162a58 => { ins += 13;
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffff9e38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                c.stw(c.r[1].wrapping_add(0x4c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x48), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[31] = c.r[3].wrapping_add(c.r[0]);
                { let (a, b) = (cr_bit(cr, 1), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if !cr_bit(cr, 2) { pc = 0x80162a90; continue; }
                pc = 0x80162a8c;
            }
            0x80162a8c => { ins += 1;
                c.f[1] = c.f[0];
                pc = 0x80162a90;
            }
            0x80162a90 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff9e38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[2], c.f[0]);
                { let (a, b) = (cr_bit(cr, 1), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if !cr_bit(cr, 2) { pc = 0x80162aa4; continue; }
                pc = 0x80162aa0;
            }
            0x80162aa0 => { ins += 1;
                c.f[2] = c.f[0];
                pc = 0x80162aa4;
            }
            0x80162aa4 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff9e38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[3], c.f[0]);
                { let (a, b) = (cr_bit(cr, 1), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if !cr_bit(cr, 2) { pc = 0x80162ab8; continue; }
                pc = 0x80162ab4;
            }
            0x80162ab4 => { ins += 1;
                c.f[3] = c.f[0];
                pc = 0x80162ab8;
            }
            0x80162ab8 => { ins += 15;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0xef));
                c.r[3] = 0x80440000;
                c.r[7] = c.r[3].wrapping_add(0x7e60);
                c.r[8] = c.lwz_t(ins, c.r[31].wrapping_add(0xf0));
                c.r[0] = c.r[0].wrapping_mul(0xbc);
                c.r[3] = c.r[4] | c.r[4];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[7].wrapping_add(c.r[0]);
                c.r[7] = c.r[8].rotate_left(1) & 0x1;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x54));
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[8] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.lr = 0x80162af4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015d7d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162af4;
            }
            0x80162af4 => { ins += 19;
                { let ea = c.r[2].wrapping_add(0xffff9e3c); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0xc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0xe5));
                { let v = s(c.f[3] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[2] = fctiwz(c.f[2]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                c.f[1] = fctiwz(c.f[1]);
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[2]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[1]; c.stfd(ea, v); }
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x3c));
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                if cr_bit(cr, 2) { pc = 0x80162b70; continue; }
                pc = 0x80162b40;
            }
            0x80162b40 => { ins += 4;
                c.r[3] = c.lhz_t(ins, c.r[31].wrapping_add(0x4c));
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80162b70; continue; }
                pc = 0x80162b50;
            }
            0x80162b50 => { ins += 4;
                c.r[3] = c.lhz_t(ins, c.r[31].wrapping_add(0x4e));
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80162b70; continue; }
                pc = 0x80162b60;
            }
            0x80162b60 => { ins += 4;
                c.r[3] = c.lhz_t(ins, c.r[31].wrapping_add(0x50));
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80162b90; continue; }
                pc = 0x80162b70;
            }
            0x80162b70 => { ins += 8;
                c.sth(c.r[31].wrapping_add(0x4c), c.r[4]);
                c.r[0] = 0x0;
                c.sth(c.r[31].wrapping_add(0x4e), c.r[5]);
                c.sth(c.r[31].wrapping_add(0x50), c.r[6]);
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x24));
                c.r[3] = c.r[3] | 0x1;
                c.stw(c.r[31].wrapping_add(0x24), c.r[3]);
                c.stb(c.r[31].wrapping_add(0xe5), c.r[0]);
                pc = 0x80162b90;
            }
            0x80162b90 => { ins += 19;
                { let ea = c.r[2].wrapping_add(0xffff9e3c); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x14); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0x18); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x1c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0xe6));
                { let v = s(c.f[3] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[2] = fctiwz(c.f[2]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                c.f[1] = fctiwz(c.f[1]);
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[2]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[1]; c.stfd(ea, v); }
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x3c));
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                if cr_bit(cr, 2) { pc = 0x80162c0c; continue; }
                pc = 0x80162bdc;
            }
            0x80162bdc => { ins += 4;
                c.r[3] = c.lhz_t(ins, c.r[31].wrapping_add(0x52));
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80162c0c; continue; }
                pc = 0x80162bec;
            }
            0x80162bec => { ins += 4;
                c.r[3] = c.lhz_t(ins, c.r[31].wrapping_add(0x54));
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80162c0c; continue; }
                pc = 0x80162bfc;
            }
            0x80162bfc => { ins += 4;
                c.r[3] = c.lhz_t(ins, c.r[31].wrapping_add(0x56));
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80162c2c; continue; }
                pc = 0x80162c0c;
            }
            0x80162c0c => { ins += 8;
                c.sth(c.r[31].wrapping_add(0x52), c.r[4]);
                c.r[0] = 0x0;
                c.sth(c.r[31].wrapping_add(0x54), c.r[5]);
                c.sth(c.r[31].wrapping_add(0x56), c.r[6]);
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x24));
                c.r[3] = c.r[3] | 0x2;
                c.stw(c.r[31].wrapping_add(0x24), c.r[3]);
                c.stb(c.r[31].wrapping_add(0xe6), c.r[0]);
                pc = 0x80162c2c;
            }
            0x80162c2c => { ins += 19;
                { let ea = c.r[2].wrapping_add(0xffff9e3c); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x20); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0x24); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0xe7));
                { let v = s(c.f[3] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[2] = fctiwz(c.f[2]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                c.f[1] = fctiwz(c.f[1]);
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[2]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[1]; c.stfd(ea, v); }
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x3c));
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                if cr_bit(cr, 2) { pc = 0x80162ca8; continue; }
                pc = 0x80162c78;
            }
            0x80162c78 => { ins += 4;
                c.r[3] = c.lhz_t(ins, c.r[31].wrapping_add(0x58));
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80162ca8; continue; }
                pc = 0x80162c88;
            }
            0x80162c88 => { ins += 4;
                c.r[3] = c.lhz_t(ins, c.r[31].wrapping_add(0x5a));
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80162ca8; continue; }
                pc = 0x80162c98;
            }
            0x80162c98 => { ins += 4;
                c.r[3] = c.lhz_t(ins, c.r[31].wrapping_add(0x5c));
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80162cc8; continue; }
                pc = 0x80162ca8;
            }
            0x80162ca8 => { ins += 8;
                c.sth(c.r[31].wrapping_add(0x58), c.r[4]);
                c.r[0] = 0x0;
                c.sth(c.r[31].wrapping_add(0x5a), c.r[5]);
                c.sth(c.r[31].wrapping_add(0x5c), c.r[6]);
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x24));
                c.r[3] = c.r[3] | 0x4;
                c.stw(c.r[31].wrapping_add(0x24), c.r[3]);
                c.stb(c.r[31].wrapping_add(0xe7), c.r[0]);
                pc = 0x80162cc8;
            }
            0x80162cc8 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0xf0));
                c.r[0] = c.r[0].rotate_left(0) & 0x80000000;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80162d00; continue; }
                pc = 0x80162cd4;
            }
            0x80162cd4 => { ins += 11;
                c.r[3] = 0x80270000;
                c.r[0] = c.r[30].rotate_left(17) & 0x1fe;
                c.r[4] = c.r[3].wrapping_add(0x3448);
                c.r[3] = c.lhz_t(ins, c.r[4].wrapping_add(c.r[0]));
                c.r[0] = c.lhz_t(ins, c.r[4].wrapping_add(c.r[0]));
                c.sth(c.r[31].wrapping_add(0xd0), c.r[3]);
                c.r[0] = c.addc(!c.r[0], 0x20, 1);
                c.sth(c.r[31].wrapping_add(0xd2), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x24));
                c.r[0] = c.r[0] | 0x200;
                c.stw(c.r[31].wrapping_add(0x24), c.r[0]);
                pc = 0x80162d00;
            }
            0x80162d00 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x4c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x48));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162a58: bad pc {:#x}", pc),
        }
    }
}

/// hwOff (0x80162d18, 44 bytes)
#[inline(never)]
pub fn f_80162d18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162d18) { return; }
        if c.prof { crate::prof::enter(c, 0x80162d18); p = true; }
    }
    f_80162d18_b(c, 0x80162d18);
    if p { crate::prof::exit(c, 0x80162d18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162d18_at(c: &mut Cpu, pc: u32) { f_80162d18_b(c, pc) }
#[inline(always)]
fn f_80162d18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162d18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162d18, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162d18 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.lr = 0x80162d34; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015d4ec(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162d34;
            }
            0x80162d34 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162d18: bad pc {:#x}", pc),
        }
    }
}

/// hwSetAUXProcessingCallbacks (0x80162d44, 40 bytes)
#[inline(never)]
pub fn f_80162d44(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162d44) { return; }
        if c.prof { crate::prof::enter(c, 0x80162d44); p = true; }
    }
    f_80162d44_b(c, 0x80162d44);
    if p { crate::prof::exit(c, 0x80162d44); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162d44_at(c: &mut Cpu, pc: u32) { f_80162d44_b(c, pc) }
#[inline(always)]
fn f_80162d44_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162d44;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162d44, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162d44 => { ins += 10;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x80440000;
                c.r[8] = c.r[0].wrapping_mul(0xbc);
                c.r[0] = c.r[3].wrapping_add(0x7e60);
                c.r[3] = c.r[0].wrapping_add(c.r[8]);
                c.stw(c.r[3].wrapping_add(0xac), c.r[4]);
                c.stw(c.r[3].wrapping_add(0xb4), c.r[5]);
                c.stw(c.r[3].wrapping_add(0xb0), c.r[6]);
                c.stw(c.r[3].wrapping_add(0xb8), c.r[7]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162d44: bad pc {:#x}", pc),
        }
    }
}

/// hwActivateStudio (0x80162d6c, 32 bytes)
#[inline(never)]
pub fn f_80162d6c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162d6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80162d6c); p = true; }
    }
    f_80162d6c_b(c, 0x80162d6c);
    if p { crate::prof::exit(c, 0x80162d6c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162d6c_at(c: &mut Cpu, pc: u32) { f_80162d6c_b(c, pc) }
#[inline(always)]
fn f_80162d6c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162d6c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162d6c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162d6c => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80162d7c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015a950(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162d7c;
            }
            0x80162d7c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162d6c: bad pc {:#x}", pc),
        }
    }
}

/// hwDeactivateStudio (0x80162d8c, 32 bytes)
#[inline(never)]
pub fn f_80162d8c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162d8c) { return; }
        if c.prof { crate::prof::enter(c, 0x80162d8c); p = true; }
    }
    f_80162d8c_b(c, 0x80162d8c);
    if p { crate::prof::exit(c, 0x80162d8c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162d8c_at(c: &mut Cpu, pc: u32) { f_80162d8c_b(c, pc) }
#[inline(always)]
fn f_80162d8c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162d8c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162d8c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162d8c => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80162d9c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015aaa0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162d9c;
            }
            0x80162d9c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162d8c: bad pc {:#x}", pc),
        }
    }
}

/// hwAddInput (0x80162dac, 52 bytes)
#[inline(never)]
pub fn f_80162dac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162dac) { return; }
        if c.prof { crate::prof::enter(c, 0x80162dac); p = true; }
    }
    f_80162dac_b(c, 0x80162dac);
    if p { crate::prof::exit(c, 0x80162dac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162dac_at(c: &mut Cpu, pc: u32) { f_80162dac_b(c, pc) }
#[inline(always)]
fn f_80162dac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162dac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162dac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162dac => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[5] = c.r[0].wrapping_mul(0xbc);
                c.r[3] = 0x80440000;
                c.r[0] = c.r[3].wrapping_add(0x7e60);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.lr = 0x80162dd0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015d54c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162dd0;
            }
            0x80162dd0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162dac: bad pc {:#x}", pc),
        }
    }
}

/// hwRemoveInput (0x80162de0, 52 bytes)
#[inline(never)]
pub fn f_80162de0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162de0) { return; }
        if c.prof { crate::prof::enter(c, 0x80162de0); p = true; }
    }
    f_80162de0_b(c, 0x80162de0);
    if p { crate::prof::exit(c, 0x80162de0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162de0_at(c: &mut Cpu, pc: u32) { f_80162de0_b(c, pc) }
#[inline(always)]
fn f_80162de0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162de0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162de0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162de0 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[5] = c.r[0].wrapping_mul(0xbc);
                c.r[3] = 0x80440000;
                c.r[0] = c.r[3].wrapping_add(0x7e60);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.lr = 0x80162e04; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015d5f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162e04;
            }
            0x80162e04 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162de0: bad pc {:#x}", pc),
        }
    }
}

/// hwGetPos (0x80162e14, 164 bytes)
#[inline(never)]
pub fn f_80162e14(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162e14) { return; }
        if c.prof { crate::prof::enter(c, 0x80162e14); p = true; }
    }
    f_80162e14_b(c, 0x80162e14);
    if p { crate::prof::exit(c, 0x80162e14); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162e14_at(c: &mut Cpu, pc: u32) { f_80162e14_b(c, pc) }
#[inline(always)]
fn f_80162e14_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162e14;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162e14, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162e14 => { ins += 6;
                c.r[5] = c.r[3].wrapping_mul(0xf4);
                c.r[6] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[4] = c.r[6].wrapping_add(c.r[5]);
                c.r[0] = c.lbz_t(ins, c.r[4].wrapping_add(0xec));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x80162e34; continue; }
                pc = 0x80162e2c;
            }
            0x80162e2c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80162e34;
            }
            0x80162e34 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[4].wrapping_add(0x90));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x3);
                if cr_bit(cr, 2) { pc = 0x80162e94; continue; }
                pc = 0x80162e40;
            }
            0x80162e40 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80162e58; continue; }
                pc = 0x80162e44;
            }
            0x80162e44 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 0) { pc = 0x80162ea4; continue; }
                pc = 0x80162e4c;
            }
            0x80162e4c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80162e60; continue; }
                pc = 0x80162e54;
            }
            0x80162e54 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80162e58;
            }
            0x80162e58 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80162e60;
            }
            0x80162e60 => { ins += 10;
                c.r[3] = c.r[6].wrapping_add(c.r[5]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x78));
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x20));
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[0] = c.r[3].wrapping_sub(c.r[0]);
                c.r[4] = c.r[3].rotate_left(0) & 0xf;
                c.r[0] = c.r[0].rotate_left(28) & 0xfffffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x2);
                c.r[3] = c.r[0].wrapping_mul(0xe);
                if cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80162e88;
            }
            0x80162e88 => { ins += 3;
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[3] = c.r[3].wrapping_add(0xfffffffe);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80162e94;
            }
            0x80162e94 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x78));
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x20));
                c.r[3] = c.r[0].wrapping_sub(c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80162ea4;
            }
            0x80162ea4 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x78));
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x20));
                c.r[3] = c.r[3].rotate_left(31) & 0x7fffffff;
                c.r[3] = c.r[0].wrapping_sub(c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162e14: bad pc {:#x}", pc),
        }
    }
}

/// hwFlushStream (0x80162eb8, 144 bytes)
#[inline(never)]
pub fn f_80162eb8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162eb8) { return; }
        if c.prof { crate::prof::enter(c, 0x80162eb8); p = true; }
    }
    f_80162eb8_b(c, 0x80162eb8);
    if p { crate::prof::exit(c, 0x80162eb8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162eb8_at(c: &mut Cpu, pc: u32) { f_80162eb8_b(c, pc) }
#[inline(always)]
fn f_80162eb8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162eb8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162eb8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162eb8 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x80162ecc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c47d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162ecc;
            }
            0x80162ecc => { ins += 8;
                c.r[25] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[26] = c.r[5] | c.r[5];
                c.r[27] = c.r[7] | c.r[7];
                c.r[28] = c.r[8] | c.r[8];
                c.r[3] = c.r[6] | c.r[6];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80162eec; { c.steps = st; c.instrs = ins; c.cr = cr; f_80163db0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162eec;
            }
            0x80162eec => { ins += 10;
                c.r[0] = c.r[29].rotate_left(0) & 0x1f;
                c.r[30] = c.r[29].rotate_left(0) & 0xffffffe0;
                c.r[26] = c.r[26].wrapping_add(c.r[0]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.r[26].wrapping_add(0x1f);
                c.r[29] = c.r[25].wrapping_add(c.r[30]);
                c.r[26] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x80162f14; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009b300(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162f14;
            }
            0x80162f14 => { ins += 7;
                c.r[3] = c.r[29] | c.r[29];
                c.r[5] = c.r[26] | c.r[26];
                c.r[7] = c.r[27] | c.r[27];
                c.r[8] = c.r[28] | c.r[28];
                c.r[4] = c.r[31].wrapping_add(c.r[30]);
                c.r[6] = 0x1;
                c.lr = 0x80162f30; { c.steps = st; c.instrs = ins; c.cr = cr; f_801632b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162f30;
            }
            0x80162f30 => { ins += 2;
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x80162f38; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c481c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162f38;
            }
            0x80162f38 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162eb8: bad pc {:#x}", pc),
        }
    }
}

/// hwInitStream (0x80162f48, 32 bytes)
#[inline(never)]
pub fn f_80162f48(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162f48) { return; }
        if c.prof { crate::prof::enter(c, 0x80162f48); p = true; }
    }
    f_80162f48_b(c, 0x80162f48);
    if p { crate::prof::exit(c, 0x80162f48); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162f48_at(c: &mut Cpu, pc: u32) { f_80162f48_b(c, pc) }
#[inline(always)]
fn f_80162f48_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162f48;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162f48, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162f48 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80162f58; { c.steps = st; c.instrs = ins; c.cr = cr; f_80163ca8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162f58;
            }
            0x80162f58 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162f48: bad pc {:#x}", pc),
        }
    }
}

/// hwExitStream (0x80162f68, 32 bytes)
#[inline(never)]
pub fn f_80162f68(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162f68) { return; }
        if c.prof { crate::prof::enter(c, 0x80162f68); p = true; }
    }
    f_80162f68_b(c, 0x80162f68);
    if p { crate::prof::exit(c, 0x80162f68); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162f68_at(c: &mut Cpu, pc: u32) { f_80162f68_b(c, pc) }
#[inline(always)]
fn f_80162f68_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162f68;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162f68, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162f68 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80162f78; { c.steps = st; c.instrs = ins; c.cr = cr; f_80163de8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162f78;
            }
            0x80162f78 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162f68: bad pc {:#x}", pc),
        }
    }
}

/// hwGetStreamPlayBuffer (0x80162f88, 36 bytes)
#[inline(never)]
pub fn f_80162f88(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162f88) { return; }
        if c.prof { crate::prof::enter(c, 0x80162f88); p = true; }
    }
    f_80162f88_b(c, 0x80162f88);
    if p { crate::prof::exit(c, 0x80162f88); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162f88_at(c: &mut Cpu, pc: u32) { f_80162f88_b(c, pc) }
#[inline(always)]
fn f_80162f88_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162f88;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162f88, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162f88 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80162f9c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80163db0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162f9c;
            }
            0x80162f9c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162f88: bad pc {:#x}", pc),
        }
    }
}

/// hwTransAddr (0x80162fac, 4 bytes)
#[inline(never)]
pub fn f_80162fac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162fac) { return; }
        if c.prof { crate::prof::enter(c, 0x80162fac); p = true; }
    }
    f_80162fac_b(c, 0x80162fac);
    if p { crate::prof::exit(c, 0x80162fac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162fac_at(c: &mut Cpu, pc: u32) { f_80162fac_b(c, pc) }
#[inline(always)]
fn f_80162fac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162fac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162fac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162fac => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162fac: bad pc {:#x}", pc),
        }
    }
}

/// hwFrq2Pitch (0x80162fb0, 92 bytes)
#[inline(never)]
pub fn f_80162fb0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80162fb0) { return; }
        if c.prof { crate::prof::enter(c, 0x80162fb0); p = true; }
    }
    f_80162fb0_b(c, 0x80162fb0);
    if p { crate::prof::exit(c, 0x80162fb0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80162fb0_at(c: &mut Cpu, pc: u32) { f_80162fb0_b(c, pc) }
#[inline(always)]
fn f_80162fb0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80162fb0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80162fb0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80162fb0 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x43300000;
                c.r[4] = 0x80430000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9e48); c.f[2] = c.lfd(ea); }
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x4c50));
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
                c.lr = 0x80162ffc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c46b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80162ffc;
            }
            0x80162ffc => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80162fb0: bad pc {:#x}", pc),
        }
    }
}

/// hwInitSampleMem (0x8016300c, 36 bytes)
#[inline(never)]
pub fn f_8016300c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016300c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016300c); p = true; }
    }
    f_8016300c_b(c, 0x8016300c);
    if p { crate::prof::exit(c, 0x8016300c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016300c_at(c: &mut Cpu, pc: u32) { f_8016300c_b(c, pc) }
#[inline(always)]
fn f_8016300c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016300c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016300c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016300c => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80163020; { c.steps = st; c.instrs = ins; c.cr = cr; f_801634a8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163020;
            }
            0x80163020 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016300c: bad pc {:#x}", pc),
        }
    }
}

/// hwExitSampleMem (0x80163030, 32 bytes)
#[inline(never)]
pub fn f_80163030(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163030) { return; }
        if c.prof { crate::prof::enter(c, 0x80163030); p = true; }
    }
    f_80163030_b(c, 0x80163030);
    if p { crate::prof::exit(c, 0x80163030); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163030_at(c: &mut Cpu, pc: u32) { f_80163030_b(c, pc) }
#[inline(always)]
fn f_80163030_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163030;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163030, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163030 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80163040; { c.steps = st; c.instrs = ins; c.cr = cr; f_80163794(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163040;
            }
            0x80163040 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163030: bad pc {:#x}", pc),
        }
    }
}

/// hwSaveSample (0x80163050, 148 bytes)
#[inline(never)]
pub fn f_80163050(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163050) { return; }
        if c.prof { crate::prof::enter(c, 0x80163050); p = true; }
    }
    f_80163050_b(c, 0x80163050);
    if p { crate::prof::exit(c, 0x80163050); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163050_at(c: &mut Cpu, pc: u32) { f_80163050_b(c, pc) }
#[inline(always)]
fn f_80163050_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163050;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163050, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163050 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = c.lwz_t(ins, c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[3].rotate_left(8) & 0xff;
                c.r[4] = c.r[3].rotate_left(0) & 0xffffff;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x3);
                if cr_bit(cr, 2) { pc = 0x801630c4; continue; }
                pc = 0x8016307c;
            }
            0x8016307c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80163094; continue; }
                pc = 0x80163080;
            }
            0x80163080 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 0) { pc = 0x801630c0; continue; }
                pc = 0x80163088;
            }
            0x80163088 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 0) { pc = 0x8016309c; continue; }
                pc = 0x80163090;
            }
            0x80163090 => { ins += 1;
                pc = 0x801630c4; continue;
                pc = 0x80163094;
            }
            0x80163094 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 0) { pc = 0x801630c4; continue; }
                pc = 0x8016309c;
            }
            0x8016309c => { ins += 9;
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
            0x801630c0 => { ins += 1;
                c.r[4] = c.r[4].rotate_left(1) & 0xfffffffe;
                pc = 0x801630c4;
            }
            0x801630c4 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[31]);
                c.lr = 0x801630cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80163810(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801630cc;
            }
            0x801630cc => { ins += 6;
                c.stw(c.r[31], c.r[3]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163050: bad pc {:#x}", pc),
        }
    }
}

/// hwSetSaveSampleCallback (0x801630e4, 32 bytes)
#[inline(never)]
pub fn f_801630e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801630e4) { return; }
        if c.prof { crate::prof::enter(c, 0x801630e4); p = true; }
    }
    f_801630e4_b(c, 0x801630e4);
    if p { crate::prof::exit(c, 0x801630e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801630e4_at(c: &mut Cpu, pc: u32) { f_801630e4_b(c, pc) }
#[inline(always)]
fn f_801630e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801630e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801630e4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801630e4 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801630f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801637b8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801630f4;
            }
            0x801630f4 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801630e4: bad pc {:#x}", pc),
        }
    }
}

/// hwRemoveSample (0x80163104, 132 bytes)
#[inline(never)]
pub fn f_80163104(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163104) { return; }
        if c.prof { crate::prof::enter(c, 0x80163104); p = true; }
    }
    f_80163104_b(c, 0x80163104);
    if p { crate::prof::exit(c, 0x80163104); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163104_at(c: &mut Cpu, pc: u32) { f_80163104_b(c, pc) }
#[inline(always)]
fn f_80163104_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163104;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163104, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163104 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[3].rotate_left(8) & 0xff;
                c.r[5] = c.r[3].rotate_left(0) & 0xffffff;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x3);
                if cr_bit(cr, 2) { pc = 0x8016316c; continue; }
                pc = 0x80163124;
            }
            0x80163124 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8016313c; continue; }
                pc = 0x80163128;
            }
            0x80163128 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 0) { pc = 0x80163168; continue; }
                pc = 0x80163130;
            }
            0x80163130 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80163144; continue; }
                pc = 0x80163138;
            }
            0x80163138 => { ins += 1;
                pc = 0x8016316c; continue;
                pc = 0x8016313c;
            }
            0x8016313c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 0) { pc = 0x8016316c; continue; }
                pc = 0x80163144;
            }
            0x80163144 => { ins += 9;
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
            0x80163168 => { ins += 1;
                c.r[5] = c.r[5].rotate_left(1) & 0xfffffffe;
                pc = 0x8016316c;
            }
            0x8016316c => { ins += 3;
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] | c.r[5];
                c.lr = 0x80163178; { c.steps = st; c.instrs = ins; c.cr = cr; f_80163bcc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163178;
            }
            0x80163178 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163104: bad pc {:#x}", pc),
        }
    }
}

/// hwSyncSampleMem (0x80163188, 32 bytes)
#[inline(never)]
pub fn f_80163188(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163188) { return; }
        if c.prof { crate::prof::enter(c, 0x80163188); p = true; }
    }
    f_80163188_b(c, 0x80163188);
    if p { crate::prof::exit(c, 0x80163188); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163188_at(c: &mut Cpu, pc: u32) { f_80163188_b(c, pc) }
#[inline(always)]
fn f_80163188_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163188;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163188, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163188 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80163198; { c.steps = st; c.instrs = ins; c.cr = cr; f_80163490(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163198;
            }
            0x80163198 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163188: bad pc {:#x}", pc),
        }
    }
}

/// hwFrameDone (0x801631a8, 4 bytes)
#[inline(never)]
pub fn f_801631a8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801631a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801631a8); p = true; }
    }
    f_801631a8_b(c, 0x801631a8);
    if p { crate::prof::exit(c, 0x801631a8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801631a8_at(c: &mut Cpu, pc: u32) { f_801631a8_b(c, pc) }
#[inline(always)]
fn f_801631a8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801631a8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801631a8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801631a8 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801631a8: bad pc {:#x}", pc),
        }
    }
}

/// sndSetHooks (0x801631ac, 20 bytes)
#[inline(never)]
pub fn f_801631ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801631ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801631ac); p = true; }
    }
    f_801631ac_b(c, 0x801631ac);
    if p { crate::prof::exit(c, 0x801631ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801631ac_at(c: &mut Cpu, pc: u32) { f_801631ac_b(c, pc) }
#[inline(always)]
fn f_801631ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801631ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801631ac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801631ac => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[3]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.stw(c.r[13].wrapping_add(0xffffa834), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa838), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801631ac: bad pc {:#x}", pc),
        }
    }
}

/// hwDisableHRTF (0x801631c0, 12 bytes)
#[inline(never)]
pub fn f_801631c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801631c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801631c0); p = true; }
    }
    f_801631c0_b(c, 0x801631c0);
    if p { crate::prof::exit(c, 0x801631c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801631c0_at(c: &mut Cpu, pc: u32) { f_801631c0_b(c, pc) }
#[inline(always)]
fn f_801631c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801631c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801631c0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801631c0 => { ins += 3;
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa7f4), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801631c0: bad pc {:#x}", pc),
        }
    }
}

/// hwGetVirtualSampleID (0x801631cc, 40 bytes)
#[inline(never)]
pub fn f_801631cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801631cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801631cc); p = true; }
    }
    f_801631cc_b(c, 0x801631cc);
    if p { crate::prof::exit(c, 0x801631cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801631cc_at(c: &mut Cpu, pc: u32) { f_801631cc_b(c, pc) }
#[inline(always)]
fn f_801631cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801631cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801631cc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801631cc => { ins += 6;
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0xec));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801631ec; continue; }
                pc = 0x801631e4;
            }
            0x801631e4 => { ins += 2;
                c.r[3] = 0xffffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801631ec;
            }
            0x801631ec => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xe8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801631cc: bad pc {:#x}", pc),
        }
    }
}

/// hwVoiceInStartup (0x801631f4, 32 bytes)
#[inline(never)]
pub fn f_801631f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801631f4) { return; }
        if c.prof { crate::prof::enter(c, 0x801631f4); p = true; }
    }
    f_801631f4_b(c, 0x801631f4);
    if p { crate::prof::exit(c, 0x801631f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801631f4_at(c: &mut Cpu, pc: u32) { f_801631f4_b(c, pc) }
#[inline(always)]
fn f_801631f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801631f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801631f4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801631f4 => { ins += 8;
                c.r[0] = c.r[3].wrapping_mul(0xf4);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa804));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0xec));
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801631f4: bad pc {:#x}", pc),
        }
    }
}

/// aramQueueCallback (0x80163214, 160 bytes)
#[inline(never)]
pub fn f_80163214(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163214) { return; }
        if c.prof { crate::prof::enter(c, 0x80163214); p = true; }
    }
    f_80163214_b(c, 0x80163214);
    if p { crate::prof::exit(c, 0x80163214); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163214_at(c: &mut Cpu, pc: u32) { f_80163214_b(c, pc) }
#[inline(always)]
fn f_80163214_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163214;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163214, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163214 => { ins += 13;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = 0x80450000;
                c.r[0] = c.lwz_t(ins, c.r[28].wrapping_add(0xc));
                c.r[31] = c.r[3].wrapping_add(0xfffffb90);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80163250; continue; }
                pc = 0x80163248;
            }
            0x80163248 => { ins += 2;
                c.r[3] = 0x80450000;
                c.r[31] = c.r[3].wrapping_add(0xfffffe14);
                pc = 0x80163250;
            }
            0x80163250 => { ins += 2;
                c.r[29] = 0x0;
                c.r[30] = c.r[31] | c.r[31];
                pc = 0x80163258;
            }
            0x80163258 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], c.r[30]);
                if !cr_bit(cr, 2) { pc = 0x80163278; continue; }
                pc = 0x80163260;
            }
            0x80163260 => { ins += 3;
                c.r[12] = c.lwz_t(ins, c.r[30].wrapping_add(0x20));
                cr_cmpu(&mut cr, c.xer, 0, c.r[12], 0x0);
                if cr_bit(cr, 2) { pc = 0x80163278; continue; }
                pc = 0x8016326c;
            }
            0x8016326c => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x24));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80163278; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x80163278;
            }
            0x80163278 => { ins += 4;
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[30] = c.r[30].wrapping_add(0x28);
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x10);
                if cr_bit(cr, 0) { pc = 0x80163258; continue; }
                pc = 0x80163288;
            }
            0x80163288 => { ins += 11;
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x281));
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.stb(c.r[31].wrapping_add(0x281), c.r[0]);
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163214: bad pc {:#x}", pc),
        }
    }
}

/// aramUploadData (0x801632b4, 476 bytes)
#[inline(never)]
pub fn f_801632b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801632b4) { return; }
        if c.prof { crate::prof::enter(c, 0x801632b4); p = true; }
    }
    f_801632b4_b(c, 0x801632b4);
    if p { crate::prof::exit(c, 0x801632b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801632b4_at(c: &mut Cpu, pc: u32) { f_801632b4_b(c, pc) }
#[inline(always)]
fn f_801632b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801632b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801632b4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801632b4 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x801632c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c47cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801632c8;
            }
            0x801632c8 => { ins += 7;
                c.r[24] = c.r[6] | c.r[6];
                cr_rc(&mut cr, c.xer, c.r[24]);
                c.r[25] = c.r[3] | c.r[3];
                c.r[26] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[28] = c.r[7] | c.r[7];
                c.r[29] = c.r[8] | c.r[8];
                if cr_bit(cr, 2) { pc = 0x801632f0; continue; }
                pc = 0x801632e4;
            }
            0x801632e4 => { ins += 3;
                c.r[3] = 0x80450000;
                c.r[31] = c.r[3].wrapping_add(0xfffffe14);
                pc = 0x801632f8; continue;
                pc = 0x801632f0;
            }
            0x801632f0 => { ins += 2;
                c.r[3] = 0x80450000;
                c.r[31] = c.r[3].wrapping_add(0xfffffb90);
                pc = 0x801632f8;
            }
            0x801632f8 => { ins += 1;
                c.lr = 0x801632fc; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801632fc;
            }
            0x801632fc => { ins += 4;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x281));
                c.r[30] = c.r[3] | c.r[3];
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x10);
                if !cr_bit(cr, 0) { pc = 0x80163470; continue; }
                pc = 0x8016330c;
            }
            0x8016330c => { ins += 75;
                c.r[4] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
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
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[5]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[8]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[25]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x14), c.r[26]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x18), c.r[27]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1c), c.r[4]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x20), c.r[28]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x24), c.r[29]);
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[4] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[5] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[6] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[7] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[10] = c.r[0].wrapping_mul(0x28);
                c.r[8] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[9] = c.r[4].wrapping_mul(0x28);
                c.r[4] = c.r[31].wrapping_add(c.r[10]);
                c.r[4] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                c.r[10] = c.r[5].wrapping_mul(0x28);
                c.r[5] = c.r[31].wrapping_add(c.r[9]);
                c.r[5] = c.lwz_t(ins, c.r[5].wrapping_add(0x8));
                c.r[9] = c.r[6].wrapping_mul(0x28);
                c.r[6] = c.r[31].wrapping_add(c.r[10]);
                c.r[6] = c.lwz_t(ins, c.r[6].wrapping_add(0xc));
                c.r[10] = c.r[7].wrapping_mul(0x28);
                c.r[7] = c.r[31].wrapping_add(c.r[9]);
                c.r[7] = c.lwz_t(ins, c.r[7].wrapping_add(0x10));
                c.r[9] = c.r[8].wrapping_mul(0x28);
                c.r[8] = c.r[31].wrapping_add(c.r[10]);
                c.r[8] = c.lwz_t(ins, c.r[8].wrapping_add(0x14));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[9] = c.r[31].wrapping_add(c.r[9]);
                c.r[9] = c.lwz_t(ins, c.r[9].wrapping_add(0x18));
                c.r[10] = c.r[31].wrapping_add(c.r[0]);
                c.r[10] = c.lwz_t(ins, c.r[10].wrapping_add(0x1c));
                c.lr = 0x80163438; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae630(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163438;
            }
            0x80163438 => { ins += 13;
                c.r[4] = c.lbz_t(ins, c.r[31].wrapping_add(0x281));
                c.r[3] = c.r[30] | c.r[30];
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stb(c.r[31].wrapping_add(0x281), c.r[0]);
                c.r[4] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[4].rotate_left(28) & 0xf0000000;
                c.r[4] = c.r[4].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.r[0] = c.r[0].rotate_left(4) & 0xffffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.stb(c.r[31].wrapping_add(0x280), c.r[0]);
                c.lr = 0x8016346c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016346c;
            }
            0x8016346c => { ins += 1;
                pc = 0x80163478; continue;
                pc = 0x80163470;
            }
            0x80163470 => { ins += 1;
                c.lr = 0x80163474; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163474;
            }
            0x80163474 => { ins += 1;
                pc = 0x801632f8; continue;
                pc = 0x80163478;
            }
            0x80163478 => { ins += 2;
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x80163480; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c4818(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163480;
            }
            0x80163480 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801632b4: bad pc {:#x}", pc),
        }
    }
}

/// aramSyncTransferQueue (0x80163490, 24 bytes)
#[inline(never)]
pub fn f_80163490(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163490) { return; }
        if c.prof { crate::prof::enter(c, 0x80163490); p = true; }
    }
    f_80163490_b(c, 0x80163490);
    if p { crate::prof::exit(c, 0x80163490); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163490_at(c: &mut Cpu, pc: u32) { f_80163490_b(c, pc) }
#[inline(always)]
fn f_80163490_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163490;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163490, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163490 => { ins += 2;
                c.r[3] = 0x80450000;
                c.r[3] = c.r[3].wrapping_add(0xfffffb90);
                pc = 0x80163498;
            }
            0x80163498 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x281));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80163498; continue; }
                pc = 0x801634a4;
            }
            0x801634a4 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163490: bad pc {:#x}", pc),
        }
    }
}

/// aramInit (0x801634a8, 748 bytes)
#[inline(never)]
pub fn f_801634a8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801634a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801634a8); p = true; }
    }
    f_801634a8_b(c, 0x801634a8);
    if p { crate::prof::exit(c, 0x801634a8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801634a8_at(c: &mut Cpu, pc: u32) { f_801634a8_b(c, pc) }
#[inline(never)]
fn f_801634a8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801634a8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801634a8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801634a8 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x20);
                c.lr = 0x801634bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c47d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801634bc;
            }
            0x801634bc => { ins += 2;
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x801634c4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acb44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801634c4;
            }
            0x801634c4 => { ins += 3;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x500;
                c.lr = 0x801634d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801634d0;
            }
            0x801634d0 => { ins += 5;
                c.r[28] = c.r[3] | c.r[3];
                c.r[0] = 0x10;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x801634e4;
            }
            0x801634e4 => { ins += 42;
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
            0x8016358c => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x500;
                c.lr = 0x80163598; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009b2d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163598;
            }
            0x80163598 => { ins += 9;
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
            0x801635bc => { ins += 1;
                c.lr = 0x801635c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801635c0;
            }
            0x801635c0 => { ins += 4;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x281));
                c.r[29] = c.r[3] | c.r[3];
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x10);
                if !cr_bit(cr, 0) { pc = 0x8016372c; continue; }
                pc = 0x801635d0;
            }
            0x801635d0 => { ins += 73;
                c.r[4] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = 0x80160000;
                c.r[7] = 0x2a;
                c.r[0] = 0x0;
                c.r[6] = c.r[4].wrapping_mul(0x28);
                c.r[5] = 0x500;
                c.r[4] = c.r[3].wrapping_add(0x3214);
                c.r[3] = c.r[31].wrapping_add(c.r[6]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[7]);
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[28]);
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x14), c.r[30]);
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x18), c.r[5]);
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x1c), c.r[4]);
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x20), c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[4] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[5] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[6] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[7] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[10] = c.r[0].wrapping_mul(0x28);
                c.r[8] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[9] = c.r[4].wrapping_mul(0x28);
                c.r[4] = c.r[31].wrapping_add(c.r[10]);
                c.r[4] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                c.r[10] = c.r[5].wrapping_mul(0x28);
                c.r[5] = c.r[31].wrapping_add(c.r[9]);
                c.r[5] = c.lwz_t(ins, c.r[5].wrapping_add(0x8));
                c.r[9] = c.r[6].wrapping_mul(0x28);
                c.r[6] = c.r[31].wrapping_add(c.r[10]);
                c.r[6] = c.lwz_t(ins, c.r[6].wrapping_add(0xc));
                c.r[10] = c.r[7].wrapping_mul(0x28);
                c.r[7] = c.r[31].wrapping_add(c.r[9]);
                c.r[7] = c.lwz_t(ins, c.r[7].wrapping_add(0x10));
                c.r[9] = c.r[8].wrapping_mul(0x28);
                c.r[8] = c.r[31].wrapping_add(c.r[10]);
                c.r[8] = c.lwz_t(ins, c.r[8].wrapping_add(0x14));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[9] = c.r[31].wrapping_add(c.r[9]);
                c.r[9] = c.lwz_t(ins, c.r[9].wrapping_add(0x18));
                c.r[10] = c.r[31].wrapping_add(c.r[0]);
                c.r[10] = c.lwz_t(ins, c.r[10].wrapping_add(0x1c));
                c.lr = 0x801636f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae630(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801636f4;
            }
            0x801636f4 => { ins += 13;
                c.r[4] = c.lbz_t(ins, c.r[31].wrapping_add(0x281));
                c.r[3] = c.r[29] | c.r[29];
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stb(c.r[31].wrapping_add(0x281), c.r[0]);
                c.r[4] = c.lbz_t(ins, c.r[31].wrapping_add(0x280));
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[4].rotate_left(28) & 0xf0000000;
                c.r[4] = c.r[4].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.r[0] = c.r[0].rotate_left(4) & 0xffffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.stb(c.r[31].wrapping_add(0x280), c.r[0]);
                c.lr = 0x80163728; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163728;
            }
            0x80163728 => { ins += 1;
                pc = 0x80163734; continue;
                pc = 0x8016372c;
            }
            0x8016372c => { ins += 1;
                c.lr = 0x80163730; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163730;
            }
            0x80163730 => { ins += 1;
                pc = 0x801635bc; continue;
                pc = 0x80163734;
            }
            0x80163734 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x281));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80163734; continue; }
                pc = 0x80163740;
            }
            0x80163740 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80163748; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164400(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163748;
            }
            0x80163748 => { ins += 3;
                c.r[0] = c.r[30].wrapping_add(c.r[27]);
                c.stw(c.r[13].wrapping_add(0xffffa85c), c.r[0]);
                c.lr = 0x80163754; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acb4c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163754;
            }
            0x80163754 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa85c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[3]);
                if !cr_bit(cr, 1) { pc = 0x80163768; continue; }
                pc = 0x80163760;
            }
            0x80163760 => { ins += 1;
                c.lr = 0x80163764; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acb4c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163764;
            }
            0x80163764 => { ins += 1;
                c.stw(c.r[13].wrapping_add(0xffffa85c), c.r[3]);
                pc = 0x80163768;
            }
            0x80163768 => { ins += 5;
                c.r[3] = c.r[30].wrapping_add(0x500);
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa858), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa850), c.r[0]);
                c.lr = 0x8016377c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80163be4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016377c;
            }
            0x8016377c => { ins += 2;
                c.r[11] = c.r[1].wrapping_add(0x20);
                c.lr = 0x80163784; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c4824(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163784;
            }
            0x80163784 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801634a8: bad pc {:#x}", pc),
        }
    }
}

/// aramExit (0x80163794, 4 bytes)
#[inline(never)]
pub fn f_80163794(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163794) { return; }
        if c.prof { crate::prof::enter(c, 0x80163794); p = true; }
    }
    f_80163794_b(c, 0x80163794);
    if p { crate::prof::exit(c, 0x80163794); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163794_at(c: &mut Cpu, pc: u32) { f_80163794_b(c, pc) }
#[inline(always)]
fn f_80163794_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163794;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163794, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163794 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163794: bad pc {:#x}", pc),
        }
    }
}

/// aramGetZeroBuffer (0x80163798, 32 bytes)
#[inline(never)]
pub fn f_80163798(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163798) { return; }
        if c.prof { crate::prof::enter(c, 0x80163798); p = true; }
    }
    f_80163798_b(c, 0x80163798);
    if p { crate::prof::exit(c, 0x80163798); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163798_at(c: &mut Cpu, pc: u32) { f_80163798_b(c, pc) }
#[inline(always)]
fn f_80163798_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163798;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163798, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163798 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801637a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acb44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801637a8;
            }
            0x801637a8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163798: bad pc {:#x}", pc),
        }
    }
}

/// aramSetUploadCallback (0x801637b8, 88 bytes)
#[inline(never)]
pub fn f_801637b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801637b8) { return; }
        if c.prof { crate::prof::enter(c, 0x801637b8); p = true; }
    }
    f_801637b8_b(c, 0x801637b8);
    if p { crate::prof::exit(c, 0x801637b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801637b8_at(c: &mut Cpu, pc: u32) { f_801637b8_b(c, pc) }
#[inline(always)]
fn f_801637b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801637b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801637b8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801637b8 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x801637f4; continue; }
                pc = 0x801637d4;
            }
            0x801637d4 => { ins += 3;
                c.r[0] = c.r[4].wrapping_add(0x1f);
                c.r[31] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.lr = 0x801637e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae78c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801637e0;
            }
            0x801637e0 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], c.r[3]);
                c.r[0] = c.r[31] | c.r[31];
                if !cr_bit(cr, 0) { pc = 0x801637f0; continue; }
                pc = 0x801637ec;
            }
            0x801637ec => { ins += 1;
                c.r[0] = c.r[3] | c.r[3];
                pc = 0x801637f0;
            }
            0x801637f0 => { ins += 1;
                c.stw(c.r[13].wrapping_add(0xffffa84c), c.r[0]);
                pc = 0x801637f4;
            }
            0x801637f4 => { ins += 7;
                c.stw(c.r[13].wrapping_add(0xffffa850), c.r[30]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801637b8: bad pc {:#x}", pc),
        }
    }
}

/// aramStoreData (0x80163810, 956 bytes)
#[inline(never)]
pub fn f_80163810(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163810) { return; }
        if c.prof { crate::prof::enter(c, 0x80163810); p = true; }
    }
    f_80163810_b(c, 0x80163810);
    if p { crate::prof::exit(c, 0x80163810); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163810_at(c: &mut Cpu, pc: u32) { f_80163810_b(c, pc) }
#[inline(never)]
fn f_80163810_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163810;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163810, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163810 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x80163824; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c47c8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163824;
            }
            0x80163824 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa850));
                c.r[4] = c.r[4].wrapping_add(0x1f);
                c.r[24] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa858));
                c.r[25] = c.r[3] | c.r[3];
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                c.r[23] = c.r[4].rotate_left(0) & 0xffffffe0;
                if !cr_bit(cr, 2) { pc = 0x801639dc; continue; }
                pc = 0x80163840;
            }
            0x80163840 => { ins += 2;
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x80163848; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009b2d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163848;
            }
            0x80163848 => { ins += 3;
                c.r[3] = 0x80450000;
                c.r[30] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa858));
                c.r[28] = c.r[3].wrapping_add(0xfffffb90);
                pc = 0x80163854;
            }
            0x80163854 => { ins += 1;
                c.lr = 0x80163858; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163858;
            }
            0x80163858 => { ins += 4;
                c.r[0] = c.lbz_t(ins, c.r[28].wrapping_add(0x281));
                c.r[29] = c.r[3] | c.r[3];
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x10);
                if !cr_bit(cr, 0) { pc = 0x801639c0; continue; }
                pc = 0x80163868;
            }
            0x80163868 => { ins += 72;
                c.r[4] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[3] = 0x80160000;
                c.r[6] = 0x2a;
                c.r[0] = 0x0;
                c.r[5] = c.r[4].wrapping_mul(0x28);
                c.r[4] = c.r[3].wrapping_add(0x3214);
                c.r[3] = c.r[28].wrapping_add(c.r[5]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[6]);
                c.r[3] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[25]);
                c.r[3] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x14), c.r[30]);
                c.r[3] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x18), c.r[23]);
                c.r[3] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x1c), c.r[4]);
                c.r[3] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x20), c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[0] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[4] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[5] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[6] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[7] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[10] = c.r[0].wrapping_mul(0x28);
                c.r[8] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[3] = c.r[28].wrapping_add(c.r[3]);
                c.r[0] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[9] = c.r[4].wrapping_mul(0x28);
                c.r[4] = c.r[28].wrapping_add(c.r[10]);
                c.r[4] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                c.r[10] = c.r[5].wrapping_mul(0x28);
                c.r[5] = c.r[28].wrapping_add(c.r[9]);
                c.r[5] = c.lwz_t(ins, c.r[5].wrapping_add(0x8));
                c.r[9] = c.r[6].wrapping_mul(0x28);
                c.r[6] = c.r[28].wrapping_add(c.r[10]);
                c.r[6] = c.lwz_t(ins, c.r[6].wrapping_add(0xc));
                c.r[10] = c.r[7].wrapping_mul(0x28);
                c.r[7] = c.r[28].wrapping_add(c.r[9]);
                c.r[7] = c.lwz_t(ins, c.r[7].wrapping_add(0x10));
                c.r[9] = c.r[8].wrapping_mul(0x28);
                c.r[8] = c.r[28].wrapping_add(c.r[10]);
                c.r[8] = c.lwz_t(ins, c.r[8].wrapping_add(0x14));
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[9] = c.r[28].wrapping_add(c.r[9]);
                c.r[9] = c.lwz_t(ins, c.r[9].wrapping_add(0x18));
                c.r[10] = c.r[28].wrapping_add(c.r[0]);
                c.r[10] = c.lwz_t(ins, c.r[10].wrapping_add(0x1c));
                c.lr = 0x80163988; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae630(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163988;
            }
            0x80163988 => { ins += 13;
                c.r[4] = c.lbz_t(ins, c.r[28].wrapping_add(0x281));
                c.r[3] = c.r[29] | c.r[29];
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stb(c.r[28].wrapping_add(0x281), c.r[0]);
                c.r[4] = c.lbz_t(ins, c.r[28].wrapping_add(0x280));
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[4].rotate_left(28) & 0xf0000000;
                c.r[4] = c.r[4].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.r[0] = c.r[0].rotate_left(4) & 0xffffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.stb(c.r[28].wrapping_add(0x280), c.r[0]);
                c.lr = 0x801639bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801639bc;
            }
            0x801639bc => { ins += 1;
                pc = 0x801639c8; continue;
                pc = 0x801639c0;
            }
            0x801639c0 => { ins += 1;
                c.lr = 0x801639c4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801639c4;
            }
            0x801639c4 => { ins += 1;
                pc = 0x80163854; continue;
                pc = 0x801639c8;
            }
            0x801639c8 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa858));
                c.r[3] = c.r[24] | c.r[24];
                c.r[0] = c.r[0].wrapping_add(c.r[23]);
                c.stw(c.r[13].wrapping_add(0xffffa858), c.r[0]);
                pc = 0x80163bb4; continue;
                pc = 0x801639dc;
            }
            0x801639dc => { ins += 5;
                c.r[4] = 0x80450000;
                c.r[3] = 0x80160000;
                c.r[30] = c.r[4].wrapping_add(0xfffffb90);
                c.r[31] = c.r[3].wrapping_add(0x3214);
                pc = 0x80163ba8; continue;
                pc = 0x801639f0;
            }
            0x801639f0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa84c));
                c.r[28] = c.r[23] | c.r[23];
                cr_cmpu(&mut cr, c.xer, 0, c.r[23], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80163a04; continue; }
                pc = 0x80163a00;
            }
            0x80163a00 => { ins += 1;
                c.r[28] = c.r[0] | c.r[0];
                pc = 0x80163a04;
            }
            0x80163a04 => { ins += 5;
                c.r[12] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa850));
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80163a18; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x80163a18;
            }
            0x80163a18 => { ins += 3;
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80163a24; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009b2d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163a24;
            }
            0x80163a24 => { ins += 1;
                c.r[26] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa858));
                pc = 0x80163a28;
            }
            0x80163a28 => { ins += 1;
                c.lr = 0x80163a2c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163a2c;
            }
            0x80163a2c => { ins += 4;
                c.r[0] = c.lbz_t(ins, c.r[30].wrapping_add(0x281));
                c.r[27] = c.r[3] | c.r[3];
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x10);
                if !cr_bit(cr, 0) { pc = 0x80163b8c; continue; }
                pc = 0x80163a3c;
            }
            0x80163a3c => { ins += 70;
                c.r[3] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[4] = 0x2a;
                c.r[0] = 0x0;
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[4]);
                c.r[3] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x8);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0xc);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[29]);
                c.r[3] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x14);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[26]);
                c.r[3] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x18);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[28]);
                c.r[3] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x1c);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[31]);
                c.r[3] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.r[3].wrapping_add(0x24);
                c.stw(c.r[30].wrapping_add(c.r[3]), c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[0] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[4] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[3].wrapping_mul(0x28);
                c.r[5] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[6] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[7] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[10] = c.r[0].wrapping_mul(0x28);
                c.r[8] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[3] = c.r[30].wrapping_add(c.r[3]);
                c.r[0] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[9] = c.r[4].wrapping_mul(0x28);
                c.r[4] = c.r[10].wrapping_add(0x4);
                c.r[4] = c.lwz_t(ins, c.r[30].wrapping_add(c.r[4]));
                c.r[10] = c.r[5].wrapping_mul(0x28);
                c.r[5] = c.r[9].wrapping_add(0x8);
                c.r[5] = c.lwz_t(ins, c.r[30].wrapping_add(c.r[5]));
                c.r[9] = c.r[6].wrapping_mul(0x28);
                c.r[6] = c.r[10].wrapping_add(0xc);
                c.r[6] = c.lwz_t(ins, c.r[30].wrapping_add(c.r[6]));
                c.r[10] = c.r[7].wrapping_mul(0x28);
                c.r[7] = c.r[9].wrapping_add(0x10);
                c.r[7] = c.lwz_t(ins, c.r[30].wrapping_add(c.r[7]));
                c.r[9] = c.r[8].wrapping_mul(0x28);
                c.r[8] = c.r[10].wrapping_add(0x14);
                c.r[8] = c.lwz_t(ins, c.r[30].wrapping_add(c.r[8]));
                c.r[10] = c.r[0].wrapping_mul(0x28);
                c.r[0] = c.r[9].wrapping_add(0x18);
                c.r[9] = c.lwz_t(ins, c.r[30].wrapping_add(c.r[0]));
                c.r[0] = c.r[10].wrapping_add(0x1c);
                c.r[10] = c.lwz_t(ins, c.r[30].wrapping_add(c.r[0]));
                c.lr = 0x80163b54; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae630(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163b54;
            }
            0x80163b54 => { ins += 13;
                c.r[4] = c.lbz_t(ins, c.r[30].wrapping_add(0x281));
                c.r[3] = c.r[27] | c.r[27];
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stb(c.r[30].wrapping_add(0x281), c.r[0]);
                c.r[4] = c.lbz_t(ins, c.r[30].wrapping_add(0x280));
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[4].rotate_left(28) & 0xf0000000;
                c.r[4] = c.r[4].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.r[0] = c.r[0].rotate_left(4) & 0xffffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.stb(c.r[30].wrapping_add(0x280), c.r[0]);
                c.lr = 0x80163b88; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163b88;
            }
            0x80163b88 => { ins += 1;
                pc = 0x80163b94; continue;
                pc = 0x80163b8c;
            }
            0x80163b8c => { ins += 1;
                c.lr = 0x80163b90; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163b90;
            }
            0x80163b90 => { ins += 1;
                pc = 0x80163a28; continue;
                pc = 0x80163b94;
            }
            0x80163b94 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa858));
                c.r[23] = c.r[23].wrapping_sub(c.r[28]);
                c.r[25] = c.r[25].wrapping_add(c.r[28]);
                c.r[0] = c.r[0].wrapping_add(c.r[28]);
                c.stw(c.r[13].wrapping_add(0xffffa858), c.r[0]);
                pc = 0x80163ba8;
            }
            0x80163ba8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[23], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801639f0; continue; }
                pc = 0x80163bb0;
            }
            0x80163bb0 => { ins += 1;
                c.r[3] = c.r[24] | c.r[24];
                pc = 0x80163bb4;
            }
            0x80163bb4 => { ins += 2;
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x80163bbc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c4814(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163bbc;
            }
            0x80163bbc => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163810: bad pc {:#x}", pc),
        }
    }
}

/// aramRemoveData (0x80163bcc, 24 bytes)
#[inline(never)]
pub fn f_80163bcc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163bcc) { return; }
        if c.prof { crate::prof::enter(c, 0x80163bcc); p = true; }
    }
    f_80163bcc_b(c, 0x80163bcc);
    if p { crate::prof::exit(c, 0x80163bcc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163bcc_at(c: &mut Cpu, pc: u32) { f_80163bcc_b(c, pc) }
#[inline(always)]
fn f_80163bcc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163bcc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163bcc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163bcc => { ins += 6;
                c.r[3] = c.r[4].wrapping_add(0x1f);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa858));
                c.r[3] = c.r[3].rotate_left(0) & 0xffffffe0;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa858), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163bcc: bad pc {:#x}", pc),
        }
    }
}

/// InitStreamBuffers (0x80163be4, 196 bytes)
#[inline(never)]
pub fn f_80163be4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163be4) { return; }
        if c.prof { crate::prof::enter(c, 0x80163be4); p = true; }
    }
    f_80163be4_b(c, 0x80163be4);
    if p { crate::prof::exit(c, 0x80163be4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163be4_at(c: &mut Cpu, pc: u32) { f_80163be4_b(c, pc) }
#[inline(always)]
fn f_80163be4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163be4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163be4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163be4 => { ins += 10;
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
            0x80163c0c => { ins += 18;
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
            0x80163c54 => { ins += 8;
                c.r[3] = 0x80450000;
                c.r[4] = c.r[8].rotate_left(4) & 0xfffffff0;
                c.r[3] = c.r[3].wrapping_add(0x98);
                c.r[0] = c.addc(!c.r[8], 0x40, 1);
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.mtspr(9, c.r[0]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], 0x40);
                if !cr_bit(cr, 0) { pc = 0x80163c84; continue; }
                pc = 0x80163c74;
            }
            0x80163c74 => { ins += 4;
                c.stw(c.r[3].wrapping_add(0xfffffff0), c.r[3]);
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.r[8] = c.r[8].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80163c74; continue; }
                pc = 0x80163c84;
            }
            0x80163c84 => { ins += 9;
                c.r[3] = 0x80450000;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa85c));
                c.r[4] = c.r[3].wrapping_add(0x98);
                c.r[3] = c.r[8].rotate_left(4) & 0xfffffff0;
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[4] = 0x0;
                c.stw(c.r[3].wrapping_add(0xfffffff0), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa854), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163be4: bad pc {:#x}", pc),
        }
    }
}

/// aramAllocateStreamBuffer (0x80163ca8, 264 bytes)
#[inline(never)]
pub fn f_80163ca8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163ca8) { return; }
        if c.prof { crate::prof::enter(c, 0x80163ca8); p = true; }
    }
    f_80163ca8_b(c, 0x80163ca8);
    if p { crate::prof::exit(c, 0x80163ca8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163ca8_at(c: &mut Cpu, pc: u32) { f_80163ca8_b(c, pc) }
#[inline(always)]
fn f_80163ca8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163ca8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163ca8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163ca8 => { ins += 7;
                c.r[0] = c.r[3].wrapping_add(0x1f);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa844));
                c.r[7] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[5] = 0x0;
                c.r[4] = 0x0;
                c.r[6] = 0xffffffff;
                pc = 0x80163cf4; continue;
                pc = 0x80163cc4;
            }
            0x80163cc4 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[7]);
                if !cr_bit(cr, 2) { pc = 0x80163cd8; continue; }
                pc = 0x80163cd0;
            }
            0x80163cd0 => { ins += 2;
                c.r[5] = c.r[3] | c.r[3];
                pc = 0x80163cfc; continue;
                pc = 0x80163cd8;
            }
            0x80163cd8 => { ins += 1;
                if !cr_bit(cr, 1) { pc = 0x80163cec; continue; }
                pc = 0x80163cdc;
            }
            0x80163cdc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x80163cec; continue; }
                pc = 0x80163ce4;
            }
            0x80163ce4 => { ins += 2;
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[0] | c.r[0];
                pc = 0x80163cec;
            }
            0x80163cec => { ins += 2;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.lwz_t(ins, c.r[3]);
                pc = 0x80163cf4;
            }
            0x80163cf4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80163cc4; continue; }
                pc = 0x80163cfc;
            }
            0x80163cfc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80163d58; continue; }
                pc = 0x80163d04;
            }
            0x80163d04 => { ins += 3;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa840));
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { pc = 0x80163d84; continue; }
                pc = 0x80163d10;
            }
            0x80163d10 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa854));
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa858));
                c.r[3] = c.r[3].wrapping_sub(c.r[7]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80163d84; continue; }
                pc = 0x80163d24;
            }
            0x80163d24 => { ins += 13;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                c.r[5] = c.r[4] | c.r[4];
                c.stw(c.r[13].wrapping_add(0xffffa840), c.r[0]);
                c.stw(c.r[4].wrapping_add(0xc), c.r[7]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[7]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa854));
                c.r[0] = c.r[0].wrapping_sub(c.r[7]);
                c.stw(c.r[13].wrapping_add(0xffffa854), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa848));
                c.stw(c.r[4], c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa848), c.r[4]);
                pc = 0x80163d84; continue;
                pc = 0x80163d58;
            }
            0x80163d58 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { pc = 0x80163d6c; continue; }
                pc = 0x80163d60;
            }
            0x80163d60 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[5]);
                c.stw(c.r[4], c.r[0]);
                pc = 0x80163d74; continue;
                pc = 0x80163d6c;
            }
            0x80163d6c => { ins += 2;
                c.r[0] = c.lwz_t(ins, c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa844), c.r[0]);
                pc = 0x80163d74;
            }
            0x80163d74 => { ins += 4;
                c.stw(c.r[5].wrapping_add(0x8), c.r[7]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa848));
                c.stw(c.r[5], c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa848), c.r[5]);
                pc = 0x80163d84;
            }
            0x80163d84 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80163d94; continue; }
                pc = 0x80163d8c;
            }
            0x80163d8c => { ins += 2;
                c.r[3] = 0xff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80163d94;
            }
            0x80163d94 => { ins += 7;
                c.r[3] = 0x80450000;
                c.r[0] = c.r[3].wrapping_add(0x98);
                c.r[0] = c.r[5].wrapping_sub(c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 28 != 0); c.r[0] = (v >> 4) as u32; }
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163ca8: bad pc {:#x}", pc),
        }
    }
}

/// aramGetStreamBufferAddress (0x80163db0, 56 bytes)
#[inline(never)]
pub fn f_80163db0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163db0) { return; }
        if c.prof { crate::prof::enter(c, 0x80163db0); p = true; }
    }
    f_80163db0_b(c, 0x80163db0);
    if p { crate::prof::exit(c, 0x80163db0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163db0_at(c: &mut Cpu, pc: u32) { f_80163db0_b(c, pc) }
#[inline(always)]
fn f_80163db0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163db0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163db0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163db0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { pc = 0x80163dd0; continue; }
                pc = 0x80163db8;
            }
            0x80163db8 => { ins += 6;
                c.r[5] = 0x80450000;
                c.r[0] = c.r[3].rotate_left(4) & 0xff0;
                c.r[5] = c.r[5].wrapping_add(0x98);
                c.r[5] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(0x8));
                c.stw(c.r[4], c.r[0]);
                pc = 0x80163dd0;
            }
            0x80163dd0 => { ins += 6;
                c.r[4] = 0x80450000;
                c.r[0] = c.r[3].rotate_left(4) & 0xff0;
                c.r[3] = c.r[4].wrapping_add(0x98);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163db0: bad pc {:#x}", pc),
        }
    }
}

/// aramFreeStreamBuffer (0x80163de8, 248 bytes)
#[inline(never)]
pub fn f_80163de8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163de8) { return; }
        if c.prof { crate::prof::enter(c, 0x80163de8); p = true; }
    }
    f_80163de8_b(c, 0x80163de8);
    if p { crate::prof::exit(c, 0x80163de8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163de8_at(c: &mut Cpu, pc: u32) { f_80163de8_b(c, pc) }
#[inline(always)]
fn f_80163de8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163de8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163de8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163de8 => { ins += 7;
                c.r[4] = 0x80450000;
                c.r[3] = c.r[3].rotate_left(4) & 0xff0;
                c.r[0] = c.r[4].wrapping_add(0x98);
                c.r[5] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa848));
                c.r[6] = 0x0;
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                pc = 0x80163e34; continue;
                pc = 0x80163e04;
            }
            0x80163e04 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], c.r[4]);
                if !cr_bit(cr, 2) { pc = 0x80163e2c; continue; }
                pc = 0x80163e0c;
            }
            0x80163e0c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x0);
                if cr_bit(cr, 2) { pc = 0x80163e20; continue; }
                pc = 0x80163e14;
            }
            0x80163e14 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                c.stw(c.r[6], c.r[0]);
                pc = 0x80163e3c; continue;
                pc = 0x80163e20;
            }
            0x80163e20 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa848), c.r[0]);
                pc = 0x80163e3c; continue;
                pc = 0x80163e2c;
            }
            0x80163e2c => { ins += 2;
                c.r[6] = c.r[5] | c.r[5];
                c.r[5] = c.lwz_t(ins, c.r[5]);
                pc = 0x80163e34;
            }
            0x80163e34 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80163e04; continue; }
                pc = 0x80163e3c;
            }
            0x80163e3c => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa854));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80163ed0; continue; }
                pc = 0x80163e4c;
            }
            0x80163e4c => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa840));
                c.r[5] = 0xffffffff;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa848));
                c.stw(c.r[4], c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa840), c.r[4]);
                pc = 0x80163e78; continue;
                pc = 0x80163e64;
            }
            0x80163e64 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[5]);
                if cr_bit(cr, 1) { pc = 0x80163e74; continue; }
                pc = 0x80163e70;
            }
            0x80163e70 => { ins += 1;
                c.r[5] = c.r[0] | c.r[0];
                pc = 0x80163e74;
            }
            0x80163e74 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[3]);
                pc = 0x80163e78;
            }
            0x80163e78 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80163e64; continue; }
                pc = 0x80163e80;
            }
            0x80163e80 => { ins += 2;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa844));
                pc = 0x80163eac; continue;
                pc = 0x80163e88;
            }
            0x80163e88 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                c.r[3] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[5]);
                if !cr_bit(cr, 0) { pc = 0x80163ea8; continue; }
                pc = 0x80163e98;
            }
            0x80163e98 => { ins += 4;
                c.stw(c.r[13].wrapping_add(0xffffa844), c.r[3]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa840));
                c.stw(c.r[4], c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa840), c.r[4]);
                pc = 0x80163ea8;
            }
            0x80163ea8 => { ins += 1;
                c.r[4] = c.r[3] | c.r[3];
                pc = 0x80163eac;
            }
            0x80163eac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80163e88; continue; }
                pc = 0x80163eb4;
            }
            0x80163eb4 => { ins += 4;
                c.r[0] = c.r[5].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa85c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x80163ec8; continue; }
                pc = 0x80163ec4;
            }
            0x80163ec4 => { ins += 1;
                c.r[3] = c.r[5] | c.r[5];
                pc = 0x80163ec8;
            }
            0x80163ec8 => { ins += 2;
                c.stw(c.r[13].wrapping_add(0xffffa854), c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x80163ed0;
            }
            0x80163ed0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa844));
                c.stw(c.r[4], c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa844), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163de8: bad pc {:#x}", pc),
        }
    }
}

/// salCallback (0x80163ee0, 168 bytes)
#[inline(never)]
pub fn f_80163ee0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163ee0) { return; }
        if c.prof { crate::prof::enter(c, 0x80163ee0); p = true; }
    }
    f_80163ee0_b(c, 0x80163ee0);
    if p { crate::prof::exit(c, 0x80163ee0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163ee0_at(c: &mut Cpu, pc: u32) { f_80163ee0_b(c, pc) }
#[inline(always)]
fn f_80163ee0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163ee0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163ee0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163ee0 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x280;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa880));
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa87c));
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
                c.lr = 0x80163f28; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac070(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163f28;
            }
            0x80163f28 => { ins += 1;
                c.lr = 0x80163f2c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800a2790(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163f2c;
            }
            0x80163f2c => { ins += 4;
                c.stw(c.r[13].wrapping_add(0xffffa86c), c.r[3]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa878));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x80163f70; continue; }
                pc = 0x80163f3c;
            }
            0x80163f3c => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa870));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80163f78; continue; }
                pc = 0x80163f48;
            }
            0x80163f48 => { ins += 3;
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa870), c.r[0]);
                c.lr = 0x80163f54; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df50(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163f54;
            }
            0x80163f54 => { ins += 3;
                c.r[12] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa884));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80163f60; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x80163f60;
            }
            0x80163f60 => { ins += 1;
                c.lr = 0x80163f64; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163f64;
            }
            0x80163f64 => { ins += 3;
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa870), c.r[0]);
                pc = 0x80163f78; continue;
                pc = 0x80163f70;
            }
            0x80163f70 => { ins += 2;
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa874), c.r[0]);
                pc = 0x80163f78;
            }
            0x80163f78 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163ee0: bad pc {:#x}", pc),
        }
    }
}

/// dspInitCallback (0x80163f88, 16 bytes)
#[inline(never)]
pub fn f_80163f88(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163f88) { return; }
        if c.prof { crate::prof::enter(c, 0x80163f88); p = true; }
    }
    f_80163f88_b(c, 0x80163f88);
    if p { crate::prof::exit(c, 0x80163f88); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163f88_at(c: &mut Cpu, pc: u32) { f_80163f88_b(c, pc) }
#[inline(always)]
fn f_80163f88_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163f88;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163f88, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163f88 => { ins += 4;
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa878), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa868), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163f88: bad pc {:#x}", pc),
        }
    }
}

/// dspResumeCallback (0x80163f98, 100 bytes)
#[inline(never)]
pub fn f_80163f98(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163f98) { return; }
        if c.prof { crate::prof::enter(c, 0x80163f98); p = true; }
    }
    f_80163f98_b(c, 0x80163f98);
    if p { crate::prof::exit(c, 0x80163f98); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163f98_at(c: &mut Cpu, pc: u32) { f_80163f98_b(c, pc) }
#[inline(always)]
fn f_80163f98_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163f98;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163f98, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163f98 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa878), c.r[3]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa874));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x80163fec; continue; }
                pc = 0x80163fb8;
            }
            0x80163fb8 => { ins += 5;
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa874), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa870));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80163fec; continue; }
                pc = 0x80163fcc;
            }
            0x80163fcc => { ins += 2;
                c.stw(c.r[13].wrapping_add(0xffffa870), c.r[3]);
                c.lr = 0x80163fd4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df50(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163fd4;
            }
            0x80163fd4 => { ins += 3;
                c.r[12] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa884));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80163fe0; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x80163fe0;
            }
            0x80163fe0 => { ins += 1;
                c.lr = 0x80163fe4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80163fe4;
            }
            0x80163fe4 => { ins += 2;
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa870), c.r[0]);
                pc = 0x80163fec;
            }
            0x80163fec => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163f98: bad pc {:#x}", pc),
        }
    }
}

/// salInitAi (0x80163ffc, 200 bytes)
#[inline(never)]
pub fn f_80163ffc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80163ffc) { return; }
        if c.prof { crate::prof::enter(c, 0x80163ffc); p = true; }
    }
    f_80163ffc_b(c, 0x80163ffc);
    if p { crate::prof::exit(c, 0x80163ffc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80163ffc_at(c: &mut Cpu, pc: u32) { f_80163ffc_b(c, pc) }
#[inline(always)]
fn f_80163ffc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80163ffc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80163ffc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80163ffc => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0xa00;
                c.lr = 0x80164020; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164020;
            }
            0x80164020 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa87c), c.r[3]);
                if cr_bit(cr, 2) { pc = 0x801640a8; continue; }
                pc = 0x8016402c;
            }
            0x8016402c => { ins += 3;
                c.r[4] = 0x0;
                c.r[5] = 0xa00;
                c.lr = 0x80164038; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164038;
            }
            0x80164038 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa87c));
                c.r[4] = 0xa00;
                c.lr = 0x80164044; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009b2d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164044;
            }
            0x80164044 => { ins += 10;
                c.r[4] = 0x1;
                c.r[0] = 0x0;
                c.r[3] = 0x80160000;
                c.stw(c.r[13].wrapping_add(0xffffa874), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x3ee0);
                c.stw(c.r[13].wrapping_add(0xffffa878), c.r[4]);
                c.stb(c.r[13].wrapping_add(0xffffa880), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa870), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa884), c.r[30]);
                c.lr = 0x8016406c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac02c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016406c;
            }
            0x8016406c => { ins += 7;
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa880));
                c.r[4] = 0x280;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa87c));
                c.r[0] = c.r[0].wrapping_mul(0x280);
                c.r[3] = c.r[3].wrapping_add(0x80000000);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.lr = 0x80164088; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac070(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164088;
            }
            0x80164088 => { ins += 8;
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
            0x801640a8 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801640ac;
            }
            0x801640ac => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80163ffc: bad pc {:#x}", pc),
        }
    }
}

/// salStartAi (0x801640c4, 32 bytes)
#[inline(never)]
pub fn f_801640c4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801640c4) { return; }
        if c.prof { crate::prof::enter(c, 0x801640c4); p = true; }
    }
    f_801640c4_b(c, 0x801640c4);
    if p { crate::prof::exit(c, 0x801640c4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801640c4_at(c: &mut Cpu, pc: u32) { f_801640c4_b(c, pc) }
#[inline(always)]
fn f_801640c4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801640c4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801640c4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801640c4 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801640d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac0f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801640d4;
            }
            0x801640d4 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801640c4: bad pc {:#x}", pc),
        }
    }
}

/// salExitAi (0x801640e4, 52 bytes)
#[inline(never)]
pub fn f_801640e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801640e4) { return; }
        if c.prof { crate::prof::enter(c, 0x801640e4); p = true; }
    }
    f_801640e4_b(c, 0x801640e4);
    if p { crate::prof::exit(c, 0x801640e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801640e4_at(c: &mut Cpu, pc: u32) { f_801640e4_b(c, pc) }
#[inline(always)]
fn f_801640e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801640e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801640e4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801640e4 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801640f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac02c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801640f8;
            }
            0x801640f8 => { ins += 1;
                c.lr = 0x801640fc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac110(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801640fc;
            }
            0x801640fc => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa87c));
                c.lr = 0x80164104; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164400(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164104;
            }
            0x80164104 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801640e4: bad pc {:#x}", pc),
        }
    }
}

/// salAiGetDest (0x80164118, 48 bytes)
#[inline(never)]
pub fn f_80164118(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164118) { return; }
        if c.prof { crate::prof::enter(c, 0x80164118); p = true; }
    }
    f_80164118_b(c, 0x80164118);
    if p { crate::prof::exit(c, 0x80164118); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164118_at(c: &mut Cpu, pc: u32) { f_80164118_b(c, pc) }
#[inline(always)]
fn f_80164118_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164118;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164118, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164118 => { ins += 12;
                c.r[3] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffa880));
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa87c));
                c.r[3] = c.r[3].wrapping_add(0x2);
                c.r[0] = c.r[3].rotate_left(30) & 0xc0000000;
                c.r[3] = c.r[3].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.r[0] = c.r[0].rotate_left(2) & 0xffffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.r[0].wrapping_mul(0x280);
                c.r[3] = c.r[4].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164118: bad pc {:#x}", pc),
        }
    }
}

/// salInitDsp (0x80164148, 188 bytes)
#[inline(never)]
pub fn f_80164148(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164148) { return; }
        if c.prof { crate::prof::enter(c, 0x80164148); p = true; }
    }
    f_80164148_b(c, 0x80164148);
    if p { crate::prof::exit(c, 0x80164148); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164148_at(c: &mut Cpu, pc: u32) { f_80164148_b(c, pc) }
#[inline(always)]
fn f_80164148_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164148;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164148, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164148 => { ins += 32;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x80370000;
                c.r[6] = 0x80450000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = 0x80160000;
                c.r[3] = 0x80160000;
                c.r[10] = c.r[6].wrapping_add(0x4a0);
                c.r[9] = c.lhz_t(ins, c.r[13].wrapping_add(0xffff83e8));
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
                c.lr = 0x801641c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae7e0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801641c8;
            }
            0x801641c8 => { ins += 3;
                c.r[3] = 0x80450000;
                c.r[3] = c.r[3].wrapping_add(0x4a0);
                c.lr = 0x801641d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae93c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801641d4;
            }
            0x801641d4 => { ins += 3;
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa868), c.r[0]);
                c.lr = 0x801641e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164328(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801641e0;
            }
            0x801641e0 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa868));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x801641e0; continue; }
                pc = 0x801641ec;
            }
            0x801641ec => { ins += 1;
                c.lr = 0x801641f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164360(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801641f0;
            }
            0x801641f0 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164148: bad pc {:#x}", pc),
        }
    }
}

/// salExitDsp (0x80164204, 52 bytes)
#[inline(never)]
pub fn f_80164204(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164204) { return; }
        if c.prof { crate::prof::enter(c, 0x80164204); p = true; }
    }
    f_80164204_b(c, 0x80164204);
    if p { crate::prof::exit(c, 0x80164204); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164204_at(c: &mut Cpu, pc: u32) { f_80164204_b(c, pc) }
#[inline(always)]
fn f_80164204_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164204;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164204, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164204 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80164214; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae8ec(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164214;
            }
            0x80164214 => { ins += 1;
                c.lr = 0x80164218; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae92c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164218;
            }
            0x80164218 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80164214; continue; }
                pc = 0x80164220;
            }
            0x80164220 => { ins += 1;
                c.lr = 0x80164224; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae8a4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164224;
            }
            0x80164224 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164204: bad pc {:#x}", pc),
        }
    }
}

/// salCtrlDsp (0x80164238, 116 bytes)
#[inline(never)]
pub fn f_80164238(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164238) { return; }
        if c.prof { crate::prof::enter(c, 0x80164238); p = true; }
    }
    f_80164238_b(c, 0x80164238);
    if p { crate::prof::exit(c, 0x80164238); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164238_at(c: &mut Cpu, pc: u32) { f_80164238_b(c, pc) }
#[inline(always)]
fn f_80164238_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164238;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164238, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164238 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80164250; { c.steps = st; c.instrs = ins; c.cr = cr; f_801642ac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164250;
            }
            0x80164250 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8016425c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015b250(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016425c;
            }
            0x8016425c => { ins += 4;
                c.r[0] = 0x0;
                c.r[31] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa7f0));
                c.stw(c.r[13].wrapping_add(0xffffa878), c.r[0]);
                c.lr = 0x8016426c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80098034(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016426c;
            }
            0x8016426c => { ins += 3;
                c.r[0] = c.lhz_t(ins, c.r[13].wrapping_add(0xffffa7ec));
                c.r[3] = c.r[0] | 0xbabe0000;
                c.lr = 0x80164278; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae7cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164278;
            }
            0x80164278 => { ins += 1;
                c.lr = 0x8016427c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae794(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016427c;
            }
            0x8016427c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80164278; continue; }
                pc = 0x80164284;
            }
            0x80164284 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8016428c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae7cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016428c;
            }
            0x8016428c => { ins += 1;
                c.lr = 0x80164290; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae794(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164290;
            }
            0x80164290 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8016428c; continue; }
                pc = 0x80164298;
            }
            0x80164298 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164238: bad pc {:#x}", pc),
        }
    }
}

/// salGetStartDelay (0x801642ac, 76 bytes)
#[inline(never)]
pub fn f_801642ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801642ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801642ac); p = true; }
    }
    f_801642ac_b(c, 0x801642ac);
    if p { crate::prof::exit(c, 0x801642ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801642ac_at(c: &mut Cpu, pc: u32) { f_801642ac_b(c, pc) }
#[inline(always)]
fn f_801642ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801642ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801642ac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801642ac => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801642bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800a2790(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801642bc;
            }
            0x801642bc => { ins += 15;
                c.r[5] = 0x80000000;
                c.r[4] = 0x431c0000;
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(0xf8));
                c.r[4] = c.r[4].wrapping_add(0xffffde83);
                c.r[5] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa86c));
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[0] = mulhwu(c.r[4], c.r[0]);
                c.r[3] = c.r[3].wrapping_sub(c.r[5]);
                c.r[3] = c.r[3].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[0].rotate_left(17) & 0x1ffff;
                c.r[3] = divwu(c.r[3], c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801642ac: bad pc {:#x}", pc),
        }
    }
}

/// hwInitIrq (0x801642f8, 44 bytes)
#[inline(never)]
pub fn f_801642f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801642f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801642f8); p = true; }
    }
    f_801642f8_b(c, 0x801642f8);
    if p { crate::prof::exit(c, 0x801642f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801642f8_at(c: &mut Cpu, pc: u32) { f_801642f8_b(c, pc) }
#[inline(always)]
fn f_801642f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801642f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801642f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801642f8 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80164308; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164308;
            }
            0x80164308 => { ins += 7;
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa860), c.r[3]);
                c.sth(c.r[13].wrapping_add(0xffffa864), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801642f8: bad pc {:#x}", pc),
        }
    }
}

/// hwExitIrq (0x80164324, 4 bytes)
#[inline(never)]
pub fn f_80164324(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164324) { return; }
        if c.prof { crate::prof::enter(c, 0x80164324); p = true; }
    }
    f_80164324_b(c, 0x80164324);
    if p { crate::prof::exit(c, 0x80164324); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164324_at(c: &mut Cpu, pc: u32) { f_80164324_b(c, pc) }
#[inline(always)]
fn f_80164324_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164324;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164324, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164324 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164324: bad pc {:#x}", pc),
        }
    }
}

/// hwEnableIrq (0x80164328, 56 bytes)
#[inline(never)]
pub fn f_80164328(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164328) { return; }
        if c.prof { crate::prof::enter(c, 0x80164328); p = true; }
    }
    f_80164328_b(c, 0x80164328);
    if p { crate::prof::exit(c, 0x80164328); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164328_at(c: &mut Cpu, pc: u32) { f_80164328_b(c, pc) }
#[inline(always)]
fn f_80164328_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164328;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164328, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164328 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lhz_t(ins, c.r[13].wrapping_add(0xffffa864));
                c.r[3] = c.r[3].wrapping_add(0xffffffff);
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.sth(c.r[13].wrapping_add(0xffffa864), c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x80164350; continue; }
                pc = 0x80164348;
            }
            0x80164348 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa860));
                c.lr = 0x80164350; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164350;
            }
            0x80164350 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164328: bad pc {:#x}", pc),
        }
    }
}

/// hwDisableIrq (0x80164360, 56 bytes)
#[inline(never)]
pub fn f_80164360(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164360) { return; }
        if c.prof { crate::prof::enter(c, 0x80164360); p = true; }
    }
    f_80164360_b(c, 0x80164360);
    if p { crate::prof::exit(c, 0x80164360); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164360_at(c: &mut Cpu, pc: u32) { f_80164360_b(c, pc) }
#[inline(always)]
fn f_80164360_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164360;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164360, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164360 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lhz_t(ins, c.r[13].wrapping_add(0xffffa864));
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[13].wrapping_add(0xffffa864), c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x80164388; continue; }
                pc = 0x80164380;
            }
            0x80164380 => { ins += 1;
                c.lr = 0x80164384; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164384;
            }
            0x80164384 => { ins += 1;
                c.stw(c.r[13].wrapping_add(0xffffa860), c.r[3]);
                pc = 0x80164388;
            }
            0x80164388 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164360: bad pc {:#x}", pc),
        }
    }
}

/// hwIRQEnterCritical (0x80164398, 32 bytes)
#[inline(never)]
pub fn f_80164398(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164398) { return; }
        if c.prof { crate::prof::enter(c, 0x80164398); p = true; }
    }
    f_80164398_b(c, 0x80164398);
    if p { crate::prof::exit(c, 0x80164398); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164398_at(c: &mut Cpu, pc: u32) { f_80164398_b(c, pc) }
#[inline(always)]
fn f_80164398_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164398;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164398, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164398 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801643a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801643a8;
            }
            0x801643a8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164398: bad pc {:#x}", pc),
        }
    }
}

/// hwIRQLeaveCritical (0x801643b8, 32 bytes)
#[inline(never)]
pub fn f_801643b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801643b8) { return; }
        if c.prof { crate::prof::enter(c, 0x801643b8); p = true; }
    }
    f_801643b8_b(c, 0x801643b8);
    if p { crate::prof::exit(c, 0x801643b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801643b8_at(c: &mut Cpu, pc: u32) { f_801643b8_b(c, pc) }
#[inline(always)]
fn f_801643b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801643b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801643b8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801643b8 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801643c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009df50(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801643c8;
            }
            0x801643c8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801643b8: bad pc {:#x}", pc),
        }
    }
}

/// salMalloc (0x801643d8, 40 bytes)
#[inline(never)]
pub fn f_801643d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801643d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801643d8); p = true; }
    }
    f_801643d8_b(c, 0x801643d8);
    if p { crate::prof::exit(c, 0x801643d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801643d8_at(c: &mut Cpu, pc: u32) { f_801643d8_b(c, pc) }
#[inline(always)]
fn f_801643d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801643d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801643d8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801643d8 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[12] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa834));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x801643f0; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x801643f0;
            }
            0x801643f0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801643d8: bad pc {:#x}", pc),
        }
    }
}

/// salFree (0x80164400, 44 bytes)
#[inline(never)]
pub fn f_80164400(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164400) { return; }
        if c.prof { crate::prof::enter(c, 0x80164400); p = true; }
    }
    f_80164400_b(c, 0x80164400);
    if p { crate::prof::exit(c, 0x80164400); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164400_at(c: &mut Cpu, pc: u32) { f_80164400_b(c, pc) }
#[inline(always)]
fn f_80164400_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164400;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164400, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164400 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = c.r[13].wrapping_add(0xffffa834);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[12] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x8016441c; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x8016441c;
            }
            0x8016441c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164400: bad pc {:#x}", pc),
        }
    }
}

/// sndAuxCallbackReverbHI (0x8016442c, 92 bytes)
#[inline(never)]
pub fn f_8016442c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016442c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016442c); p = true; }
    }
    f_8016442c_b(c, 0x8016442c);
    if p { crate::prof::exit(c, 0x8016442c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016442c_at(c: &mut Cpu, pc: u32) { f_8016442c_b(c, pc) }
#[inline(always)]
fn f_8016442c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016442c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016442c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016442c => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80164478; continue; }
                pc = 0x80164448;
            }
            0x80164448 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80164478; continue; }
                pc = 0x8016444c;
            }
            0x8016444c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80164458; continue; }
                pc = 0x80164454;
            }
            0x80164454 => { ins += 1;
                pc = 0x80164478; continue;
                pc = 0x80164458;
            }
            0x80164458 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[5].wrapping_add(0x1c4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80164478; continue; }
                pc = 0x80164464;
            }
            0x80164464 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[7]);
                c.r[6] = c.r[5] | c.r[5];
                c.r[4] = c.lwz_t(ins, c.r[7].wrapping_add(0x4));
                c.r[5] = c.lwz_t(ins, c.r[7].wrapping_add(0x8));
                c.lr = 0x80164478; { c.steps = st; c.instrs = ins; c.cr = cr; f_801652dc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164478;
            }
            0x80164478 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016442c: bad pc {:#x}", pc),
        }
    }
}

/// sndAuxCallbackUpdateSettingsReverbHI (0x80164488, 88 bytes)
#[inline(never)]
pub fn f_80164488(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164488) { return; }
        if c.prof { crate::prof::enter(c, 0x80164488); p = true; }
    }
    f_80164488_b(c, 0x80164488);
    if p { crate::prof::exit(c, 0x80164488); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164488_at(c: &mut Cpu, pc: u32) { f_80164488_b(c, pc) }
#[inline(always)]
fn f_80164488_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164488;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164488, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164488 => { ins += 14;
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
                c.lr = 0x801644c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164a2c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801644c0;
            }
            0x801644c0 => { ins += 8;
                c.r[0] = 0x0;
                c.r[3] = 0x1;
                c.stb(c.r[31].wrapping_add(0x1c4), c.r[0]);
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164488: bad pc {:#x}", pc),
        }
    }
}

/// sndAuxCallbackPrepareReverbHI (0x801644e0, 64 bytes)
#[inline(never)]
pub fn f_801644e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801644e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801644e0); p = true; }
    }
    f_801644e0_b(c, 0x801644e0);
    if p { crate::prof::exit(c, 0x801644e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801644e0_at(c: &mut Cpu, pc: u32) { f_801644e0_b(c, pc) }
#[inline(always)]
fn f_801644e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801644e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801644e0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801644e0 => { ins += 12;
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
                c.lr = 0x80164510; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164520(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164510;
            }
            0x80164510 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801644e0: bad pc {:#x}", pc),
        }
    }
}

/// ReverbHICreate (0x80164520, 1292 bytes)
#[inline(never)]
pub fn f_80164520(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164520) { return; }
        if c.prof { crate::prof::enter(c, 0x80164520); p = true; }
    }
    f_80164520_b(c, 0x80164520);
    if p { crate::prof::exit(c, 0x80164520); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164520_at(c: &mut Cpu, pc: u32) { f_80164520_b(c, pc) }
#[inline(never)]
fn f_80164520_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164520;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164520, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164520 => { ins += 21;
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
                c.lr = 0x80164574; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c47bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164574;
            }
            0x80164574 => { ins += 10;
                c.f[24] = c.f[1];
                { let ea = c.r[2].wrapping_add(0xffff9e50); c.f[7] = c.lfs(ea); c.ps1[7] = c.f[7]; }
                c.f[28] = c.f[2];
                c.r[30] = c.r[3] | c.r[3];
                c.f[25] = c.f[3];
                c.f[26] = c.f[4];
                c.f[31] = c.f[5];
                c.f[27] = c.f[6];
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[24], c.f[7]);
                if cr_bit(cr, 0) { pc = 0x80164604; continue; }
                pc = 0x8016459c;
            }
            0x8016459c => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9e54); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[24], c.f[1]);
                if cr_bit(cr, 1) { pc = 0x80164604; continue; }
                pc = 0x801645a8;
            }
            0x801645a8 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9e58); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[28], c.f[0]);
                if cr_bit(cr, 0) { pc = 0x80164604; continue; }
                pc = 0x801645b4;
            }
            0x801645b4 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9e5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[28], c.f[0]);
                if cr_bit(cr, 1) { pc = 0x80164604; continue; }
                pc = 0x801645c0;
            }
            0x801645c0 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[25], c.f[7]);
                if cr_bit(cr, 0) { pc = 0x80164604; continue; }
                pc = 0x801645c8;
            }
            0x801645c8 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[25], c.f[1]);
                if cr_bit(cr, 1) { pc = 0x80164604; continue; }
                pc = 0x801645d0;
            }
            0x801645d0 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[27], c.f[7]);
                if cr_bit(cr, 0) { pc = 0x80164604; continue; }
                pc = 0x801645d8;
            }
            0x801645d8 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[27], c.f[1]);
                if cr_bit(cr, 1) { pc = 0x80164604; continue; }
                pc = 0x801645e0;
            }
            0x801645e0 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[26], c.f[7]);
                if cr_bit(cr, 0) { pc = 0x80164604; continue; }
                pc = 0x801645e8;
            }
            0x801645e8 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[26], c.f[1]);
                if cr_bit(cr, 1) { pc = 0x80164604; continue; }
                pc = 0x801645f0;
            }
            0x801645f0 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[31], c.f[7]);
                if cr_bit(cr, 0) { pc = 0x80164604; continue; }
                pc = 0x801645f8;
            }
            0x801645f8 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9e60); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[31], c.f[0]);
                if !cr_bit(cr, 1) { pc = 0x8016460c; continue; }
                pc = 0x80164604;
            }
            0x80164604 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801649d4; continue;
                pc = 0x8016460c;
            }
            0x8016460c => { ins += 3;
                c.r[4] = 0x0;
                c.r[5] = 0x1c4;
                c.lr = 0x80164618; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164618;
            }
            0x80164618 => { ins += 9;
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
            0x8016463c => { ins += 4;
                c.r[23] = c.r[31].rotate_left(0) & 0xff;
                c.r[22] = 0x0;
                c.r[24] = c.r[23].wrapping_mul(0x3);
                pc = 0x80164740; continue;
                pc = 0x8016464c;
            }
            0x8016464c => { ins += 11;
                c.r[0] = c.r[22].rotate_left(0) & 0xff;
                c.r[27] = c.r[22].rotate_left(2) & 0x3fc;
                c.r[25] = c.r[0].wrapping_add(c.r[24]);
                c.r[3] = c.lwz_t(ins, c.r[29].wrapping_add(c.r[27]));
                c.r[0] = c.r[25].wrapping_mul(0x14);
                c.r[21] = c.r[3].wrapping_add(0x2);
                c.r[20] = c.r[21].rotate_left(2) & 0xfffffffc;
                c.r[26] = c.r[30].wrapping_add(c.r[0]);
                c.stw(c.r[26].wrapping_add(0xbc), c.r[20]);
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x80164678; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164678;
            }
            0x80164678 => { ins += 5;
                c.stw(c.r[26].wrapping_add(0xc0), c.r[3]);
                c.r[5] = c.r[20] | c.r[20];
                c.r[4] = 0x0;
                c.r[3] = c.lwz_t(ins, c.r[26].wrapping_add(0xc0));
                c.lr = 0x8016468c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016468c;
            }
            0x8016468c => { ins += 7;
                { let ea = c.r[26].wrapping_add(0xc4); let v = c.f[30]; c.stfs(ea, v); }
                { let v = c.r[21] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.r[3] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.lwz_t(ins, c.r[26].wrapping_add(0xb4));
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[26].wrapping_add(0xb8), c.r[0]);
                pc = 0x801646b8; continue;
                pc = 0x801646a8;
            }
            0x801646a8 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[26].wrapping_add(0xb8));
                c.r[0] = c.lwz_t(ins, c.r[26].wrapping_add(0xbc));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[26].wrapping_add(0xb8), c.r[0]);
                pc = 0x801646b8;
            }
            0x801646b8 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[26].wrapping_add(0xb8));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 0) { pc = 0x801646a8; continue; }
                pc = 0x801646c4;
            }
            0x801646c4 => { ins += 9;
                c.r[0] = 0x0;
                c.stw(c.r[26].wrapping_add(0xb4), c.r[0]);
                c.stw(c.r[26].wrapping_add(0xb8), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[29].wrapping_add(c.r[27]));
                c.r[0] = c.lwz_t(ins, c.r[26].wrapping_add(0xb4));
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[26].wrapping_add(0xb8), c.r[0]);
                pc = 0x801646f8; continue;
                pc = 0x801646e8;
            }
            0x801646e8 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[26].wrapping_add(0xb8));
                c.r[0] = c.lwz_t(ins, c.r[26].wrapping_add(0xbc));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[26].wrapping_add(0xb8), c.r[0]);
                pc = 0x801646f8;
            }
            0x801646f8 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[26].wrapping_add(0xb8));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 0) { pc = 0x801646e8; continue; }
                pc = 0x80164704;
            }
            0x80164704 => { ins += 10;
                c.r[0] = c.lwz_t(ins, c.r[29].wrapping_add(c.r[27]));
                c.stw(c.r[1].wrapping_add(0x8), c.r[28]);
                c.r[0] = c.r[0].wrapping_mul(0xfffffffd);
                { let ea = c.r[2].wrapping_add(0xffff9e68); c.f[1] = c.lfd(ea); }
                c.r[0] = c.r[0] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[29]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] / c.f[28]); c.f[2] = v; c.ps1[2] = v; }
                c.lr = 0x8016472c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ce358(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016472c;
            }
            0x8016472c => { ins += 5;
                { let v = s(c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                c.r[3] = c.r[25].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[3].wrapping_add(0x16c);
                c.r[22] = c.r[22].wrapping_add(0x1);
                { let ea = c.r[30].wrapping_add(c.r[0]); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80164740;
            }
            0x80164740 => { ins += 3;
                c.r[0] = c.r[22].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if cr_bit(cr, 0) { pc = 0x8016464c; continue; }
                pc = 0x8016474c;
            }
            0x8016474c => { ins += 2;
                c.r[27] = 0x0;
                pc = 0x80164818; continue;
                pc = 0x80164754;
            }
            0x80164754 => { ins += 13;
                c.r[0] = c.r[27].rotate_left(2) & 0x3fc;
                c.r[4] = c.r[27].rotate_left(0) & 0xff;
                c.r[3] = c.r[29].wrapping_add(c.r[0]);
                c.r[0] = c.r[4].wrapping_add(c.r[24]);
                c.r[26] = c.r[3].wrapping_add(0xc);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                c.r[0] = c.r[0].wrapping_mul(0x14);
                c.r[21] = c.r[3].wrapping_add(0x2);
                c.r[20] = c.r[21].rotate_left(2) & 0xfffffffc;
                c.r[25] = c.r[30].wrapping_add(c.r[0]);
                c.stw(c.r[25].wrapping_add(0x8), c.r[20]);
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x80164788; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164788;
            }
            0x80164788 => { ins += 5;
                c.stw(c.r[25].wrapping_add(0xc), c.r[3]);
                c.r[5] = c.r[20] | c.r[20];
                c.r[4] = 0x0;
                c.r[3] = c.lwz_t(ins, c.r[25].wrapping_add(0xc));
                c.lr = 0x8016479c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016479c;
            }
            0x8016479c => { ins += 7;
                { let ea = c.r[25].wrapping_add(0x10); let v = c.f[30]; c.stfs(ea, v); }
                { let v = c.r[21] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.r[3] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.lwz_t(ins, c.r[25]);
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[25].wrapping_add(0x4), c.r[0]);
                pc = 0x801647c8; continue;
                pc = 0x801647b8;
            }
            0x801647b8 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[25].wrapping_add(0x4));
                c.r[0] = c.lwz_t(ins, c.r[25].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[25].wrapping_add(0x4), c.r[0]);
                pc = 0x801647c8;
            }
            0x801647c8 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[25].wrapping_add(0x4));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 0) { pc = 0x801647b8; continue; }
                pc = 0x801647d4;
            }
            0x801647d4 => { ins += 9;
                c.r[0] = 0x0;
                c.stw(c.r[25], c.r[0]);
                c.stw(c.r[25].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[26]);
                c.r[0] = c.lwz_t(ins, c.r[25]);
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[25].wrapping_add(0x4), c.r[0]);
                pc = 0x80164808; continue;
                pc = 0x801647f8;
            }
            0x801647f8 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[25].wrapping_add(0x4));
                c.r[0] = c.lwz_t(ins, c.r[25].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[25].wrapping_add(0x4), c.r[0]);
                pc = 0x80164808;
            }
            0x80164808 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[25].wrapping_add(0x4));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 0) { pc = 0x801647f8; continue; }
                pc = 0x80164814;
            }
            0x80164814 => { ins += 1;
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x80164818;
            }
            0x80164818 => { ins += 3;
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x80164754; continue; }
                pc = 0x80164824;
            }
            0x80164824 => { ins += 11;
                c.r[0] = c.r[23].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[29].wrapping_add(c.r[0]);
                c.r[24] = c.r[3].wrapping_add(0x14);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x14));
                c.r[0] = c.r[23].wrapping_mul(0x3c);
                c.r[21] = c.r[3].wrapping_add(0x2);
                c.r[20] = c.r[21].rotate_left(2) & 0xfffffffc;
                c.r[23] = c.r[30].wrapping_add(c.r[0]);
                c.stw(c.r[23].wrapping_add(0x30), c.r[20]);
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x80164850; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164850;
            }
            0x80164850 => { ins += 5;
                c.stw(c.r[23].wrapping_add(0x34), c.r[3]);
                c.r[5] = c.r[20] | c.r[20];
                c.r[4] = 0x0;
                c.r[3] = c.lwz_t(ins, c.r[23].wrapping_add(0x34));
                c.lr = 0x80164864; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164864;
            }
            0x80164864 => { ins += 7;
                { let ea = c.r[23].wrapping_add(0x38); let v = c.f[30]; c.stfs(ea, v); }
                { let v = c.r[21] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.r[3] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.lwz_t(ins, c.r[23].wrapping_add(0x28));
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[23].wrapping_add(0x2c), c.r[0]);
                pc = 0x80164890; continue;
                pc = 0x80164880;
            }
            0x80164880 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[23].wrapping_add(0x2c));
                c.r[0] = c.lwz_t(ins, c.r[23].wrapping_add(0x30));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[23].wrapping_add(0x2c), c.r[0]);
                pc = 0x80164890;
            }
            0x80164890 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[23].wrapping_add(0x2c));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 0) { pc = 0x80164880; continue; }
                pc = 0x8016489c;
            }
            0x8016489c => { ins += 9;
                c.r[0] = 0x0;
                c.stw(c.r[23].wrapping_add(0x28), c.r[0]);
                c.stw(c.r[23].wrapping_add(0x2c), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[24]);
                c.r[0] = c.lwz_t(ins, c.r[23].wrapping_add(0x28));
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[23].wrapping_add(0x2c), c.r[0]);
                pc = 0x801648d0; continue;
                pc = 0x801648c0;
            }
            0x801648c0 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[23].wrapping_add(0x2c));
                c.r[0] = c.lwz_t(ins, c.r[23].wrapping_add(0x30));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[23].wrapping_add(0x2c), c.r[0]);
                pc = 0x801648d0;
            }
            0x801648d0 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[23].wrapping_add(0x2c));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 0) { pc = 0x801648c0; continue; }
                pc = 0x801648dc;
            }
            0x801648dc => { ins += 4;
                c.r[3] = c.r[31].rotate_left(2) & 0x3fc;
                c.r[31] = c.r[31].wrapping_add(0x1);
                c.r[0] = c.r[3].wrapping_add(0x190);
                { let ea = c.r[30].wrapping_add(c.r[0]); let v = c.f[30]; c.stfs(ea, v); }
                pc = 0x801648ec;
            }
            0x801648ec => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if cr_bit(cr, 0) { pc = 0x8016463c; continue; }
                pc = 0x801648f8;
            }
            0x801648f8 => { ins += 8;
                { let ea = c.r[30].wrapping_add(0x168); let v = c.f[24]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffff9e70); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[30].wrapping_add(0x19c); let v = c.f[25]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x1a8); let v = c.f[27]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x1a0); let v = c.f[26]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x1a0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                if !cr_bit(cr, 0) { pc = 0x8016491c; continue; }
                pc = 0x80164918;
            }
            0x80164918 => { ins += 1;
                { let ea = c.r[30].wrapping_add(0x1a0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8016491c;
            }
            0x8016491c => { ins += 11;
                { let ea = c.r[2].wrapping_add(0xffff9e74); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[30].wrapping_add(0x1a0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9e50); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9e70); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[2].wrapping_add(0xffff9e54); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[0], c.f[31]);
                { let v = s(c.f[2] + c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[3] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[30].wrapping_add(0x1a0); let v = c.f[0]; c.stfs(ea, v); }
                if cr_bit(cr, 2) { pc = 0x801649b0; continue; }
                pc = 0x80164948;
            }
            0x80164948 => { ins += 8;
                { let ea = c.r[2].wrapping_add(0xffff9e64); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[21] = 0x0;
                { let v = s(c.f[0] * c.f[31]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.stw(c.r[30].wrapping_add(0x1a4), c.r[0]);
                pc = 0x801649a0; continue;
                pc = 0x80164968;
            }
            0x80164968 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x1a4));
                c.r[3] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.lr = 0x80164974; { c.steps = st; c.instrs = ins; c.cr = cr; f_801643d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164974;
            }
            0x80164974 => { ins += 8;
                c.r[0] = c.r[21].rotate_left(2) & 0x3fc;
                c.r[4] = 0x0;
                c.r[20] = c.r[30].wrapping_add(c.r[0]);
                c.stw(c.r[20].wrapping_add(0x1ac), c.r[3]);
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x1a4));
                c.r[3] = c.lwz_t(ins, c.r[20].wrapping_add(0x1ac));
                c.r[5] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.lr = 0x80164994; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164994;
            }
            0x80164994 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[20].wrapping_add(0x1ac));
                c.r[21] = c.r[21].wrapping_add(0x1);
                c.stw(c.r[20].wrapping_add(0x1b8), c.r[0]);
                pc = 0x801649a0;
            }
            0x801649a0 => { ins += 3;
                c.r[0] = c.r[21].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if cr_bit(cr, 0) { pc = 0x80164968; continue; }
                pc = 0x801649ac;
            }
            0x801649ac => { ins += 1;
                pc = 0x801649d0; continue;
                pc = 0x801649b0;
            }
            0x801649b0 => { ins += 8;
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
            0x801649d0 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x801649d4;
            }
            0x801649d4 => { ins += 18;
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
                c.lr = 0x80164a1c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c4808(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164a1c;
            }
            0x80164a1c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xc0);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164520: bad pc {:#x}", pc),
        }
    }
}

/// ReverbHIModify (0x80164a2c, 532 bytes)
#[inline(never)]
pub fn f_80164a2c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164a2c) { return; }
        if c.prof { crate::prof::enter(c, 0x80164a2c); p = true; }
    }
    f_80164a2c_b(c, 0x80164a2c);
    if p { crate::prof::exit(c, 0x80164a2c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164a2c_at(c: &mut Cpu, pc: u32) { f_80164a2c_b(c, pc) }
#[inline(never)]
fn f_80164a2c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164a2c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164a2c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164a2c => { ins += 27;
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
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[26], c.f[7]);
                if cr_bit(cr, 0) { pc = 0x80164b00; continue; }
                pc = 0x80164a98;
            }
            0x80164a98 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9e54); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[26], c.f[1]);
                if cr_bit(cr, 1) { pc = 0x80164b00; continue; }
                pc = 0x80164aa4;
            }
            0x80164aa4 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9e58); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[27], c.f[0]);
                if cr_bit(cr, 0) { pc = 0x80164b00; continue; }
                pc = 0x80164ab0;
            }
            0x80164ab0 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9e5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[27], c.f[0]);
                if cr_bit(cr, 1) { pc = 0x80164b00; continue; }
                pc = 0x80164abc;
            }
            0x80164abc => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[28], c.f[7]);
                if cr_bit(cr, 0) { pc = 0x80164b00; continue; }
                pc = 0x80164ac4;
            }
            0x80164ac4 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[28], c.f[1]);
                if cr_bit(cr, 1) { pc = 0x80164b00; continue; }
                pc = 0x80164acc;
            }
            0x80164acc => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[31], c.f[7]);
                if cr_bit(cr, 0) { pc = 0x80164b00; continue; }
                pc = 0x80164ad4;
            }
            0x80164ad4 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[31], c.f[1]);
                if cr_bit(cr, 1) { pc = 0x80164b00; continue; }
                pc = 0x80164adc;
            }
            0x80164adc => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[29], c.f[7]);
                if cr_bit(cr, 0) { pc = 0x80164b00; continue; }
                pc = 0x80164ae4;
            }
            0x80164ae4 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[29], c.f[1]);
                if cr_bit(cr, 1) { pc = 0x80164b00; continue; }
                pc = 0x80164aec;
            }
            0x80164aec => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[30], c.f[7]);
                if cr_bit(cr, 0) { pc = 0x80164b00; continue; }
                pc = 0x80164af4;
            }
            0x80164af4 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9e80); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[30], c.f[0]);
                if !cr_bit(cr, 1) { pc = 0x80164b08; continue; }
                pc = 0x80164b00;
            }
            0x80164b00 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80164bf8; continue;
                pc = 0x80164b08;
            }
            0x80164b08 => { ins += 8;
                { let ea = c.r[30].wrapping_add(0x168); let v = c.f[26]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffff9e70); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[30].wrapping_add(0x19c); let v = c.f[28]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x1a8); let v = c.f[31]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x1a0); let v = c.f[29]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x1a0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                if !cr_bit(cr, 0) { pc = 0x80164b2c; continue; }
                pc = 0x80164b28;
            }
            0x80164b28 => { ins += 1;
                { let ea = c.r[30].wrapping_add(0x1a0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80164b2c;
            }
            0x80164b2c => { ins += 10;
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
            0x80164b54 => { ins += 5;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.r[3] = c.r[0].wrapping_mul(0x14);
                c.r[0] = c.r[3].wrapping_add(0xc);
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(c.r[0]));
                c.lr = 0x80164b68; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164400(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164b68;
            }
            0x80164b68 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80164b6c;
            }
            0x80164b6c => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x9);
                if cr_bit(cr, 0) { pc = 0x80164b54; continue; }
                pc = 0x80164b78;
            }
            0x80164b78 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80164b98; continue;
                pc = 0x80164b80;
            }
            0x80164b80 => { ins += 5;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.r[3] = c.r[0].wrapping_mul(0x14);
                c.r[0] = c.r[3].wrapping_add(0xc0);
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(c.r[0]));
                c.lr = 0x80164b94; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164400(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164b94;
            }
            0x80164b94 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80164b98;
            }
            0x80164b98 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x9);
                if cr_bit(cr, 0) { pc = 0x80164b80; continue; }
                pc = 0x80164ba4;
            }
            0x80164ba4 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x1a4));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x80164bd8; continue; }
                pc = 0x80164bb0;
            }
            0x80164bb0 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80164bcc; continue;
                pc = 0x80164bb8;
            }
            0x80164bb8 => { ins += 4;
                c.r[3] = c.r[31].rotate_left(2) & 0x3fc;
                c.r[0] = c.r[3].wrapping_add(0x1ac);
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(c.r[0]));
                c.lr = 0x80164bc8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164400(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164bc8;
            }
            0x80164bc8 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80164bcc;
            }
            0x80164bcc => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if cr_bit(cr, 0) { pc = 0x80164bb8; continue; }
                pc = 0x80164bd8;
            }
            0x80164bd8 => { ins += 8;
                c.f[1] = c.f[26];
                c.r[3] = c.r[30] | c.r[30];
                c.f[2] = c.f[27];
                c.f[3] = c.f[28];
                c.f[4] = c.f[29];
                c.f[5] = c.f[30];
                c.f[6] = c.f[31];
                c.lr = 0x80164bf8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164520(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80164bf8;
            }
            0x80164bf8 => { ins += 18;
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
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x74));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164a2c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80164C40 (0x80164c40, 400 bytes)
#[inline(never)]
pub fn f_80164c40(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164c40) { return; }
        if c.prof { crate::prof::enter(c, 0x80164c40); p = true; }
    }
    f_80164c40_b(c, 0x80164c40);
    if p { crate::prof::exit(c, 0x80164c40); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164c40_at(c: &mut Cpu, pc: u32) { f_80164c40_b(c, pc) }
#[inline(always)]
fn f_80164c40_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164c40;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164c40, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164c40 => { ins += 38;
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
                c.r[6] = c.lwz_t(ins, c.r[3]);
                c.r[7] = c.lwz_t(ins, c.r[4]);
                c.r[6] = c.r[6] ^ 0x80000000;
                c.r[8] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[7] = c.r[7] ^ 0x80000000;
                c.r[9] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
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
            0x80164cd8 => { ins += 39;
                { let (x, y) = (c.f[5], c.f[6]); c.f[9] = x; c.ps1[9] = y; }
                { let ea = c.r[3].wrapping_add(0x8); c.r[6] = c.lwz_t(ins, ea); c.r[3] = ea; }
                { let (x, y) = (c.f[7], c.f[8]); c.f[10] = x; c.ps1[10] = y; }
                { let ea = c.r[4].wrapping_add(0x8); c.r[7] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.r[6] = c.r[6] ^ 0x80000000;
                c.r[8] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                { let (x, y) = (s(c.f[9] * c.f[3]), s(c.ps1[9] * c.ps1[3])); c.f[11] = x; c.ps1[11] = y; }
                c.r[7] = c.r[7] ^ 0x80000000;
                { let (x, y) = (s(c.f[9] * c.f[4]), s(c.ps1[9] * c.ps1[4])); c.f[12] = x; c.ps1[12] = y; }
                c.r[9] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
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
            0x80164d74 => { ins += 23;
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
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164c40: bad pc {:#x}", pc),
        }
    }
}

/// HandleReverb (0x80164dd0, 1292 bytes)
#[inline(never)]
pub fn f_80164dd0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80164dd0) { return; }
        if c.prof { crate::prof::enter(c, 0x80164dd0); p = true; }
    }
    f_80164dd0_b(c, 0x80164dd0);
    if p { crate::prof::exit(c, 0x80164dd0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80164dd0_at(c: &mut Cpu, pc: u32) { f_80164dd0_b(c, pc) }
#[inline(never)]
fn f_80164dd0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80164dd0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80164dd0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80164dd0 => { ins += 83;
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
                c.r[31] = c.lwz_t(ins, c.r[4].wrapping_add(0x1a4));
                c.r[30] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0x58), c.r[30]);
                c.r[22] = c.r[31].wrapping_add(0xffffffff);
                c.r[22] = c.r[22].rotate_left(2) & 0xfffffffc;
                c.r[28] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[28] = c.r[28].wrapping_add(c.r[4]);
                cr_cmps(&mut cr, c.xer, 7, c.r[31], 0x0);
                c.r[21] = c.lwz_t(ins, c.r[29]);
                c.r[20] = c.lwz_t(ins, c.r[29].wrapping_add(0x4));
                c.r[19] = c.lwz_t(ins, c.r[29].wrapping_add(0x14));
                c.r[18] = c.lwz_t(ins, c.r[29].wrapping_add(0x18));
                c.r[17] = c.lwz_t(ins, c.r[29].wrapping_add(0x28));
                c.r[16] = c.lwz_t(ins, c.r[29].wrapping_add(0x2c));
                { let ea = c.r[29].wrapping_add(0x10); c.f[16] = c.lfs(ea); c.ps1[16] = c.f[16]; }
                { let ea = c.r[29].wrapping_add(0x24); c.f[17] = c.lfs(ea); c.ps1[17] = c.f[17]; }
                { let ea = c.r[29].wrapping_add(0x38); c.f[18] = c.lfs(ea); c.ps1[18] = c.f[18]; }
                c.r[25] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                c.r[24] = c.lwz_t(ins, c.r[29].wrapping_add(0x1c));
                c.r[23] = c.lwz_t(ins, c.r[29].wrapping_add(0x30));
                c.r[4] = c.lwz_t(ins, c.r[29].wrapping_add(0xc));
                c.r[5] = c.lwz_t(ins, c.r[29].wrapping_add(0x20));
                c.r[6] = c.lwz_t(ins, c.r[29].wrapping_add(0x34));
                c.r[12] = c.lwz_t(ins, c.r[27]);
                c.r[11] = c.lwz_t(ins, c.r[27].wrapping_add(0x4));
                c.r[10] = c.lwz_t(ins, c.r[27].wrapping_add(0x14));
                c.r[9] = c.lwz_t(ins, c.r[27].wrapping_add(0x18));
                c.r[8] = c.lwz_t(ins, c.r[27].wrapping_add(0x28));
                c.r[7] = c.lwz_t(ins, c.r[27].wrapping_add(0x2c));
                { let ea = c.r[27].wrapping_add(0x10); c.f[19] = c.lfs(ea); c.ps1[19] = c.f[19]; }
                { let ea = c.r[27].wrapping_add(0x24); c.f[20] = c.lfs(ea); c.ps1[20] = c.f[20]; }
                { let ea = c.r[27].wrapping_add(0x38); c.f[21] = c.lfs(ea); c.ps1[21] = c.f[21]; }
                c.r[15] = c.lwz_t(ins, c.r[27].wrapping_add(0x8));
                c.r[14] = c.lwz_t(ins, c.r[27].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[3]);
                c.r[30] = c.r[30] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0x5c), c.r[30]);
                { let ea = c.r[1].wrapping_add(0x58); c.f[12] = c.lfd(ea); }
                { let v = s(c.f[12] - c.f[5]); c.f[12] = v; c.ps1[12] = v; }
                c.r[31] = 0x9f;
                c.mtspr(9, c.r[31]);
                pc = 0x80164f1c;
            }
            0x80164f1c => { ins += 2;
                c.f[13] = c.f[12];
                if cr_bit(cr, 30) { pc = 0x80164f4c; continue; }
                pc = 0x80164f24;
            }
            0x80164f24 => { ins += 8;
                c.r[30] = c.lwz_t(ins, c.r[28].wrapping_add(0x1ac));
                c.r[29] = c.lwz_t(ins, c.r[28].wrapping_add(0x1b8));
                c.r[31] = c.r[22].wrapping_add(c.r[30]);
                c.r[29] = c.r[29].wrapping_add(0x4);
                { let ea = c.r[29].wrapping_add(0xfffffffc); c.f[13] = c.lfs(ea); c.ps1[13] = c.f[13]; }
                cr_cmps(&mut cr, c.xer, 0, c.r[29], c.r[31]);
                { let ea = c.r[29].wrapping_add(0xfffffffc); let v = c.f[12]; c.stfs(ea, v); }
                if !cr_bit(cr, 2) { pc = 0x80164f48; continue; }
                pc = 0x80164f44;
            }
            0x80164f44 => { ins += 1;
                c.r[29] = c.r[30] | c.r[30];
                pc = 0x80164f48;
            }
            0x80164f48 => { ins += 1;
                c.stw(c.r[30].wrapping_add(0x1b8), c.r[29]);
                pc = 0x80164f4c;
            }
            0x80164f4c => { ins += 18;
                { let v = s(c.f[22].mul_add(c.f[16], c.f[13])); c.f[8] = v; c.ps1[8] = v; }
                { let ea = c.r[3].wrapping_add(0x4); c.r[29] = c.lwz_t(ins, ea); c.r[3] = ea; }
                { let v = s(c.f[23].mul_add(c.f[17], c.f[13])); c.f[9] = v; c.ps1[9] = v; }
                { let ea = c.r[4].wrapping_add(c.r[21]); let v = c.f[8]; c.stfs(ea, v); }
                c.r[21] = c.r[21].wrapping_add(0x4);
                { let ea = c.r[5].wrapping_add(c.r[19]); let v = c.f[9]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(c.r[20]); c.f[14] = c.lfs(ea); c.ps1[14] = c.f[14]; }
                c.r[20] = c.r[20].wrapping_add(0x4);
                { let ea = c.r[5].wrapping_add(c.r[18]); c.f[17] = c.lfs(ea); c.ps1[17] = c.f[17]; }
                cr_cmps(&mut cr, c.xer, 0, c.r[21], c.r[25]);
                cr_cmps(&mut cr, c.xer, 1, c.r[20], c.r[25]);
                c.r[19] = c.r[19].wrapping_add(0x4);
                c.r[18] = c.r[18].wrapping_add(0x4);
                c.f[16] = c.f[14];
                cr_cmps(&mut cr, c.xer, 5, c.r[19], c.r[24]);
                { let v = s(c.f[14] + c.f[17]); c.f[14] = v; c.ps1[14] = v; }
                cr_cmps(&mut cr, c.xer, 6, c.r[18], c.r[24]);
                if !cr_bit(cr, 2) { pc = 0x80164f98; continue; }
                pc = 0x80164f94;
            }
            0x80164f94 => { ins += 1;
                c.r[21] = 0x0;
                pc = 0x80164f98;
            }
            0x80164f98 => { ins += 2;
                { let v = s(c.f[24].mul_add(c.f[18], c.f[13])); c.f[8] = v; c.ps1[8] = v; }
                if !cr_bit(cr, 6) { pc = 0x80164fa4; continue; }
                pc = 0x80164fa0;
            }
            0x80164fa0 => { ins += 1;
                c.r[20] = 0x0;
                pc = 0x80164fa4;
            }
            0x80164fa4 => { ins += 3;
                { let ea = c.r[6].wrapping_add(c.r[17]); let v = c.f[8]; c.stfs(ea, v); }
                c.r[17] = c.r[17].wrapping_add(0x4);
                if !cr_bit(cr, 22) { pc = 0x80164fb4; continue; }
                pc = 0x80164fb0;
            }
            0x80164fb0 => { ins += 1;
                c.r[19] = 0x0;
                pc = 0x80164fb4;
            }
            0x80164fb4 => { ins += 4;
                { let ea = c.r[6].wrapping_add(c.r[16]); c.f[18] = c.lfs(ea); c.ps1[18] = c.f[18]; }
                c.r[16] = c.r[16].wrapping_add(0x4);
                cr_cmps(&mut cr, c.xer, 0, c.r[17], c.r[23]);
                if !cr_bit(cr, 26) { pc = 0x80164fc8; continue; }
                pc = 0x80164fc4;
            }
            0x80164fc4 => { ins += 1;
                c.r[18] = 0x0;
                pc = 0x80164fc8;
            }
            0x80164fc8 => { ins += 5;
                { let v = s(c.f[14] + c.f[18]); c.f[14] = v; c.ps1[14] = v; }
                cr_cmps(&mut cr, c.xer, 1, c.r[16], c.r[23]);
                c.r[26] = c.lwz_t(ins, c.r[27].wrapping_add(0xc));
                { let v = s(c.f[2].mul_add(c.f[19], c.f[14])); c.f[9] = v; c.ps1[9] = v; }
                if !cr_bit(cr, 2) { pc = 0x80164fe0; continue; }
                pc = 0x80164fdc;
            }
            0x80164fdc => { ins += 1;
                c.r[17] = 0x0;
                pc = 0x80164fe0;
            }
            0x80164fe0 => { ins += 1;
                if !cr_bit(cr, 6) { pc = 0x80164fe8; continue; }
                pc = 0x80164fe4;
            }
            0x80164fe4 => { ins += 1;
                c.r[16] = 0x0;
                pc = 0x80164fe8;
            }
            0x80164fe8 => { ins += 11;
                c.r[29] = c.r[29] ^ 0x80000000;
                { let ea = c.r[26].wrapping_add(c.r[12]); let v = c.f[9]; c.stfs(ea, v); }
                { let v = s(-(c.f[2].mul_add(c.f[9], -c.f[19]))); c.f[14] = v; c.ps1[14] = v; }
                c.r[12] = c.r[12].wrapping_add(0x4);
                { let ea = c.r[26].wrapping_add(c.r[11]); c.f[19] = c.lfs(ea); c.ps1[19] = c.f[19]; }
                cr_cmps(&mut cr, c.xer, 5, c.r[12], c.r[15]);
                c.r[11] = c.r[11].wrapping_add(0x4);
                c.r[26] = c.lwz_t(ins, c.r[27].wrapping_add(0x20));
                cr_cmps(&mut cr, c.xer, 6, c.r[11], c.r[15]);
                { let v = s(c.f[2].mul_add(c.f[20], c.f[14])); c.f[8] = v; c.ps1[8] = v; }
                if !cr_bit(cr, 22) { pc = 0x80165018; continue; }
                pc = 0x80165014;
            }
            0x80165014 => { ins += 1;
                c.r[12] = 0x0;
                pc = 0x80165018;
            }
            0x80165018 => { ins += 5;
                c.stw(c.r[1].wrapping_add(0x5c), c.r[29]);
                { let ea = c.r[26].wrapping_add(c.r[10]); let v = c.f[8]; c.stfs(ea, v); }
                { let v = s(-(c.f[2].mul_add(c.f[8], -c.f[20]))); c.f[14] = v; c.ps1[14] = v; }
                c.r[10] = c.r[10].wrapping_add(0x4);
                if !cr_bit(cr, 26) { pc = 0x80165030; continue; }
                pc = 0x8016502c;
            }
            0x8016502c => { ins += 1;
                c.r[11] = 0x0;
                pc = 0x80165030;
            }
            0x80165030 => { ins += 8;
                { let ea = c.r[26].wrapping_add(c.r[9]); c.f[20] = c.lfs(ea); c.ps1[20] = c.f[20]; }
                cr_cmps(&mut cr, c.xer, 0, c.r[10], c.r[14]);
                { let v = s(c.f[14] * c.f[6]); c.f[14] = v; c.ps1[14] = v; }
                c.r[9] = c.r[9].wrapping_add(0x4);
                cr_cmps(&mut cr, c.xer, 1, c.r[9], c.r[14]);
                { let ea = c.r[1].wrapping_add(0x58); c.f[10] = c.lfd(ea); }
                { let v = s(c.f[15].mul_add(c.f[25], c.f[14])); c.f[14] = v; c.ps1[14] = v; }
                if !cr_bit(cr, 2) { pc = 0x80165054; continue; }
                pc = 0x80165050;
            }
            0x80165050 => { ins += 1;
                c.r[10] = 0x0;
                pc = 0x80165054;
            }
            0x80165054 => { ins += 4;
                c.r[26] = c.lwz_t(ins, c.r[27].wrapping_add(0x34));
                { let v = s(c.f[2].mul_add(c.f[21], c.f[14])); c.f[9] = v; c.ps1[9] = v; }
                c.f[25] = c.f[14];
                if !cr_bit(cr, 6) { pc = 0x80165068; continue; }
                pc = 0x80165064;
            }
            0x80165064 => { ins += 1;
                c.r[9] = 0x0;
                pc = 0x80165068;
            }
            0x80165068 => { ins += 12;
                { let ea = c.r[26].wrapping_add(c.r[8]); let v = c.f[9]; c.stfs(ea, v); }
                { let v = s(-(c.f[2].mul_add(c.f[9], -c.f[21]))); c.f[14] = v; c.ps1[14] = v; }
                c.r[31] = c.lwz_t(ins, c.r[27].wrapping_add(0x30));
                { let v = s(c.f[4] * c.f[12]); c.f[8] = v; c.ps1[8] = v; }
                { let ea = c.r[26].wrapping_add(c.r[7]); c.f[21] = c.lfs(ea); c.ps1[21] = c.f[21]; }
                c.r[8] = c.r[8].wrapping_add(0x4);
                c.r[7] = c.r[7].wrapping_add(0x4);
                { let v = s(c.f[3].mul_add(c.f[14], c.f[8])); c.f[14] = v; c.ps1[14] = v; }
                cr_cmps(&mut cr, c.xer, 5, c.r[8], c.r[31]);
                cr_cmps(&mut cr, c.xer, 6, c.r[7], c.r[31]);
                c.f[14] = fctiwz(c.f[14]);
                if !cr_bit(cr, 22) { pc = 0x8016509c; continue; }
                pc = 0x80165098;
            }
            0x80165098 => { ins += 1;
                c.r[8] = 0x0;
                pc = 0x8016509c;
            }
            0x8016509c => { ins += 1;
                if !cr_bit(cr, 26) { pc = 0x801650a4; continue; }
                pc = 0x801650a0;
            }
            0x801650a0 => { ins += 1;
                c.r[7] = 0x0;
                pc = 0x801650a4;
            }
            0x801650a4 => { ins += 4;
                c.r[31] = 0xfffffffc;
                { let v = s(c.f[10] - c.f[5]); c.f[12] = v; c.ps1[12] = v; }
                c.stw(c.r[3].wrapping_add(c.r[31]), c.f[14].to_bits() as u32);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80164f1c; continue; }
                pc = 0x801650b4;
            }
            0x801650b4 => { ins += 2;
                c.f[13] = c.f[12];
                if cr_bit(cr, 30) { pc = 0x801650e4; continue; }
                pc = 0x801650bc;
            }
            0x801650bc => { ins += 8;
                c.r[30] = c.lwz_t(ins, c.r[28].wrapping_add(0x1ac));
                c.r[29] = c.lwz_t(ins, c.r[28].wrapping_add(0x1b8));
                c.r[31] = c.r[22].wrapping_add(c.r[30]);
                c.r[29] = c.r[29].wrapping_add(0x4);
                { let ea = c.r[29].wrapping_add(0xfffffffc); c.f[13] = c.lfs(ea); c.ps1[13] = c.f[13]; }
                cr_cmps(&mut cr, c.xer, 0, c.r[29], c.r[31]);
                { let ea = c.r[29].wrapping_add(0xfffffffc); let v = c.f[12]; c.stfs(ea, v); }
                if !cr_bit(cr, 2) { pc = 0x801650e0; continue; }
                pc = 0x801650dc;
            }
            0x801650dc => { ins += 1;
                c.r[29] = c.r[30] | c.r[30];
                pc = 0x801650e0;
            }
            0x801650e0 => { ins += 1;
                c.stw(c.r[30].wrapping_add(0x1b8), c.r[29]);
                pc = 0x801650e4;
            }
            0x801650e4 => { ins += 17;
                { let v = s(c.f[22].mul_add(c.f[16], c.f[13])); c.f[8] = v; c.ps1[8] = v; }
                { let v = s(c.f[23].mul_add(c.f[17], c.f[13])); c.f[9] = v; c.ps1[9] = v; }
                { let ea = c.r[4].wrapping_add(c.r[21]); let v = c.f[8]; c.stfs(ea, v); }
                c.r[21] = c.r[21].wrapping_add(0x4);
                { let ea = c.r[5].wrapping_add(c.r[19]); let v = c.f[9]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(c.r[20]); c.f[14] = c.lfs(ea); c.ps1[14] = c.f[14]; }
                c.r[20] = c.r[20].wrapping_add(0x4);
                { let ea = c.r[5].wrapping_add(c.r[18]); c.f[17] = c.lfs(ea); c.ps1[17] = c.f[17]; }
                cr_cmps(&mut cr, c.xer, 0, c.r[21], c.r[25]);
                cr_cmps(&mut cr, c.xer, 1, c.r[20], c.r[25]);
                c.r[19] = c.r[19].wrapping_add(0x4);
                c.r[18] = c.r[18].wrapping_add(0x4);
                c.f[16] = c.f[14];
                cr_cmps(&mut cr, c.xer, 5, c.r[19], c.r[24]);
                { let v = s(c.f[14] + c.f[17]); c.f[14] = v; c.ps1[14] = v; }
                cr_cmps(&mut cr, c.xer, 6, c.r[18], c.r[24]);
                if !cr_bit(cr, 2) { pc = 0x8016512c; continue; }
                pc = 0x80165128;
            }
            0x80165128 => { ins += 1;
                c.r[21] = 0x0;
                pc = 0x8016512c;
            }
            0x8016512c => { ins += 2;
                { let v = s(c.f[24].mul_add(c.f[18], c.f[13])); c.f[8] = v; c.ps1[8] = v; }
                if !cr_bit(cr, 6) { pc = 0x80165138; continue; }
                pc = 0x80165134;
            }
            0x80165134 => { ins += 1;
                c.r[20] = 0x0;
                pc = 0x80165138;
            }
            0x80165138 => { ins += 3;
                { let ea = c.r[6].wrapping_add(c.r[17]); let v = c.f[8]; c.stfs(ea, v); }
                c.r[17] = c.r[17].wrapping_add(0x4);
                if !cr_bit(cr, 22) { pc = 0x80165148; continue; }
                pc = 0x80165144;
            }
            0x80165144 => { ins += 1;
                c.r[19] = 0x0;
                pc = 0x80165148;
            }
            0x80165148 => { ins += 4;
                { let ea = c.r[6].wrapping_add(c.r[16]); c.f[18] = c.lfs(ea); c.ps1[18] = c.f[18]; }
                c.r[16] = c.r[16].wrapping_add(0x4);
                cr_cmps(&mut cr, c.xer, 0, c.r[17], c.r[23]);
                if !cr_bit(cr, 26) { pc = 0x8016515c; continue; }
                pc = 0x80165158;
            }
            0x80165158 => { ins += 1;
                c.r[18] = 0x0;
                pc = 0x8016515c;
            }
            0x8016515c => { ins += 5;
                { let v = s(c.f[14] + c.f[18]); c.f[14] = v; c.ps1[14] = v; }
                cr_cmps(&mut cr, c.xer, 1, c.r[16], c.r[23]);
                c.r[26] = c.lwz_t(ins, c.r[27].wrapping_add(0xc));
                { let v = s(c.f[2].mul_add(c.f[19], c.f[14])); c.f[9] = v; c.ps1[9] = v; }
                if !cr_bit(cr, 2) { pc = 0x80165174; continue; }
                pc = 0x80165170;
            }
            0x80165170 => { ins += 1;
                c.r[17] = 0x0;
                pc = 0x80165174;
            }
            0x80165174 => { ins += 1;
                if !cr_bit(cr, 6) { pc = 0x8016517c; continue; }
                pc = 0x80165178;
            }
            0x80165178 => { ins += 1;
                c.r[16] = 0x0;
                pc = 0x8016517c;
            }
            0x8016517c => { ins += 10;
                { let ea = c.r[26].wrapping_add(c.r[12]); let v = c.f[9]; c.stfs(ea, v); }
                { let v = s(-(c.f[2].mul_add(c.f[9], -c.f[19]))); c.f[14] = v; c.ps1[14] = v; }
                c.r[12] = c.r[12].wrapping_add(0x4);
                { let ea = c.r[26].wrapping_add(c.r[11]); c.f[19] = c.lfs(ea); c.ps1[19] = c.f[19]; }
                cr_cmps(&mut cr, c.xer, 5, c.r[12], c.r[15]);
                c.r[11] = c.r[11].wrapping_add(0x4);
                c.r[26] = c.lwz_t(ins, c.r[27].wrapping_add(0x20));
                cr_cmps(&mut cr, c.xer, 6, c.r[11], c.r[15]);
                { let v = s(c.f[2].mul_add(c.f[20], c.f[14])); c.f[8] = v; c.ps1[8] = v; }
                if !cr_bit(cr, 22) { pc = 0x801651a8; continue; }
                pc = 0x801651a4;
            }
            0x801651a4 => { ins += 1;
                c.r[12] = 0x0;
                pc = 0x801651a8;
            }
            0x801651a8 => { ins += 4;
                { let ea = c.r[26].wrapping_add(c.r[10]); let v = c.f[8]; c.stfs(ea, v); }
                { let v = s(-(c.f[2].mul_add(c.f[8], -c.f[20]))); c.f[14] = v; c.ps1[14] = v; }
                c.r[10] = c.r[10].wrapping_add(0x4);
                if !cr_bit(cr, 26) { pc = 0x801651bc; continue; }
                pc = 0x801651b8;
            }
            0x801651b8 => { ins += 1;
                c.r[11] = 0x0;
                pc = 0x801651bc;
            }
            0x801651bc => { ins += 7;
                { let ea = c.r[26].wrapping_add(c.r[9]); c.f[20] = c.lfs(ea); c.ps1[20] = c.f[20]; }
                cr_cmps(&mut cr, c.xer, 0, c.r[10], c.r[14]);
                { let v = s(c.f[14] * c.f[6]); c.f[14] = v; c.ps1[14] = v; }
                c.r[9] = c.r[9].wrapping_add(0x4);
                cr_cmps(&mut cr, c.xer, 1, c.r[9], c.r[14]);
                { let v = s(c.f[15].mul_add(c.f[25], c.f[14])); c.f[14] = v; c.ps1[14] = v; }
                if !cr_bit(cr, 2) { pc = 0x801651dc; continue; }
                pc = 0x801651d8;
            }
            0x801651d8 => { ins += 1;
                c.r[10] = 0x0;
                pc = 0x801651dc;
            }
            0x801651dc => { ins += 6;
                c.r[26] = c.lwz_t(ins, c.r[27].wrapping_add(0x34));
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x50));
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                { let v = s(c.f[2].mul_add(c.f[21], c.f[14])); c.f[9] = v; c.ps1[9] = v; }
                c.f[25] = c.f[14];
                if !cr_bit(cr, 6) { pc = 0x801651f8; continue; }
                pc = 0x801651f4;
            }
            0x801651f4 => { ins += 1;
                c.r[9] = 0x0;
                pc = 0x801651f8;
            }
            0x801651f8 => { ins += 12;
                { let ea = c.r[26].wrapping_add(c.r[8]); let v = c.f[9]; c.stfs(ea, v); }
                { let v = s(-(c.f[2].mul_add(c.f[9], -c.f[21]))); c.f[14] = v; c.ps1[14] = v; }
                c.r[29] = c.lwz_t(ins, c.r[27].wrapping_add(0x30));
                { let v = s(c.f[4] * c.f[12]); c.f[8] = v; c.ps1[8] = v; }
                { let ea = c.r[26].wrapping_add(c.r[7]); c.f[21] = c.lfs(ea); c.ps1[21] = c.f[21]; }
                c.r[8] = c.r[8].wrapping_add(0x4);
                c.r[7] = c.r[7].wrapping_add(0x4);
                { let v = s(c.f[3].mul_add(c.f[14], c.f[8])); c.f[14] = v; c.ps1[14] = v; }
                cr_cmps(&mut cr, c.xer, 5, c.r[8], c.r[29]);
                cr_cmps(&mut cr, c.xer, 6, c.r[7], c.r[29]);
                c.f[14] = fctiwz(c.f[14]);
                if !cr_bit(cr, 22) { pc = 0x8016522c; continue; }
                pc = 0x80165228;
            }
            0x80165228 => { ins += 1;
                c.r[8] = 0x0;
                pc = 0x8016522c;
            }
            0x8016522c => { ins += 1;
                if !cr_bit(cr, 26) { pc = 0x80165234; continue; }
                pc = 0x80165230;
            }
            0x80165230 => { ins += 1;
                c.r[7] = 0x0;
                pc = 0x80165234;
            }
            0x80165234 => { ins += 42;
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
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 14..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0xc0);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80164dd0: bad pc {:#x}", pc),
        }
    }
}

/// ReverbHICallback (0x801652dc, 224 bytes)
#[inline(never)]
pub fn f_801652dc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801652dc) { return; }
        if c.prof { crate::prof::enter(c, 0x801652dc); p = true; }
    }
    f_801652dc_b(c, 0x801652dc);
    if p { crate::prof::exit(c, 0x801652dc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801652dc_at(c: &mut Cpu, pc: u32) { f_801652dc_b(c, pc) }
#[inline(always)]
fn f_801652dc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801652dc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801652dc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801652dc => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x20);
                c.lr = 0x801652f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c47d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801652f0;
            }
            0x801652f0 => { ins += 6;
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[31] = 0x0;
                pc = 0x80165398; continue;
                pc = 0x80165308;
            }
            0x80165308 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80165370; continue; }
                pc = 0x80165314;
            }
            0x80165314 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80165324; continue; }
                pc = 0x80165318;
            }
            0x80165318 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80165330; continue; }
                pc = 0x80165320;
            }
            0x80165320 => { ins += 1;
                pc = 0x80165394; continue;
                pc = 0x80165324;
            }
            0x80165324 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 0) { pc = 0x80165394; continue; }
                pc = 0x8016532c;
            }
            0x8016532c => { ins += 1;
                pc = 0x80165384; continue;
                pc = 0x80165330;
            }
            0x80165330 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff9e50); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[30].wrapping_add(0x1a8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[0], c.f[2]);
                if cr_bit(cr, 2) { pc = 0x8016535c; continue; }
                pc = 0x80165340;
            }
            0x80165340 => { ins += 7;
                { let ea = c.r[2].wrapping_add(0xffff9e98); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[27] | c.r[27];
                { let ea = c.r[2].wrapping_add(0xffff9e54); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.r[28] | c.r[28];
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] - c.f[1]); c.f[2] = v; c.ps1[2] = v; }
                c.lr = 0x8016535c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164c40(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016535c;
            }
            0x8016535c => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.lr = 0x8016536c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164dd0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016536c;
            }
            0x8016536c => { ins += 1;
                pc = 0x80165394; continue;
                pc = 0x80165370;
            }
            0x80165370 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x1;
                c.lr = 0x80165380; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164dd0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165380;
            }
            0x80165380 => { ins += 1;
                pc = 0x80165394; continue;
                pc = 0x80165384;
            }
            0x80165384 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x2;
                c.lr = 0x80165394; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164dd0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165394;
            }
            0x80165394 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80165398;
            }
            0x80165398 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if cr_bit(cr, 0) { pc = 0x80165308; continue; }
                pc = 0x801653a4;
            }
            0x801653a4 => { ins += 2;
                c.r[11] = c.r[1].wrapping_add(0x20);
                c.lr = 0x801653ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c4824(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801653ac;
            }
            0x801653ac => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801652dc: bad pc {:#x}", pc),
        }
    }
}

/// soundGetCurrentSubSound (0x801653bc, 8 bytes)
#[inline(never)]
pub fn f_801653bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801653bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801653bc); p = true; }
    }
    f_801653bc_b(c, 0x801653bc);
    if p { crate::prof::exit(c, 0x801653bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801653bc_at(c: &mut Cpu, pc: u32) { f_801653bc_b(c, pc) }
#[inline(always)]
fn f_801653bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801653bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801653bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801653bc => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa888));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801653bc: bad pc {:#x}", pc),
        }
    }
}

/// soundGetCurrentBackgroundMusic (0x801653c4, 8 bytes)
#[inline(never)]
pub fn f_801653c4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801653c4) { return; }
        if c.prof { crate::prof::enter(c, 0x801653c4); p = true; }
    }
    f_801653c4_b(c, 0x801653c4);
    if p { crate::prof::exit(c, 0x801653c4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801653c4_at(c: &mut Cpu, pc: u32) { f_801653c4_b(c, pc) }
#[inline(always)]
fn f_801653c4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801653c4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801653c4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801653c4 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa88c));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801653c4: bad pc {:#x}", pc),
        }
    }
}

/// soundPlayFanfare (0x801653cc, 276 bytes)
#[inline(never)]
pub fn f_801653cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801653cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801653cc); p = true; }
    }
    f_801653cc_b(c, 0x801653cc);
    if p { crate::prof::exit(c, 0x801653cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801653cc_at(c: &mut Cpu, pc: u32) { f_801653cc_b(c, pc) }
#[inline(always)]
fn f_801653cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801653cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801653cc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801653cc => { ins += 11;
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
                cr_rc(&mut cr, c.xer, c.r[28]);
                if cr_bit(cr, 2) { pc = 0x80165408; continue; }
                pc = 0x801653f8;
            }
            0x801653f8 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8788));
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80165410; continue; }
                pc = 0x80165408;
            }
            0x80165408 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801654c0; continue;
                pc = 0x80165410;
            }
            0x80165410 => { ins += 8;
                c.r[0] = c.r[28].wrapping_mul(0xc);
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8610));
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3]);
                c.r[31] = c.lbz_t(ins, c.r[4].wrapping_add(0x3));
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x801654bc; continue; }
                pc = 0x80165430;
            }
            0x80165430 => { ins += 6;
                c.r[3] = 0x10000;
                c.r[4] = 0x20;
                c.r[5] = 0x0;
                c.r[6] = 0x408;
                c.r[7] = 0x0;
                c.lr = 0x80165448; { c.steps = st; c.instrs = ins; c.cr = cr; f_800f9418(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165448;
            }
            0x80165448 => { ins += 2;
                c.r[4] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[4]);
                if cr_bit(cr, 2) { pc = 0x801654bc; continue; }
                pc = 0x80165450;
            }
            0x80165450 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8614));
                c.r[0] = c.r[31].rotate_left(3) & 0xfffffff8;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(c.r[0]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x801654b0; continue; }
                pc = 0x80165464;
            }
            0x80165464 => { ins += 2;
                c.r[5] = 0x10000;
                c.lr = 0x8016546c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165ee0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016546c;
            }
            0x8016546c => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x408;
                c.lr = 0x8016547c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166b3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016547c;
            }
            0x8016547c => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = 0x0;
                c.lr = 0x80165490; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166a50(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165490;
            }
            0x80165490 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80165498; { c.steps = st; c.instrs = ins; c.cr = cr; f_801666bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165498;
            }
            0x80165498 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x2);
                if !cr_bit(cr, 2) { pc = 0x801654a8; continue; }
                pc = 0x801654a0;
            }
            0x801654a0 => { ins += 1;
                c.lr = 0x801654a4; { c.steps = st; c.instrs = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801654a4;
            }
            0x801654a4 => { ins += 1;
                pc = 0x80165490; continue;
                pc = 0x801654a8;
            }
            0x801654a8 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x801654b0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166b18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801654b0;
            }
            0x801654b0 => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x408;
                c.lr = 0x801654bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800f9210(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801654bc;
            }
            0x801654bc => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x801654c0;
            }
            0x801654c0 => { ins += 8;
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
            _ => panic!("f_801653cc: bad pc {:#x}", pc),
        }
    }
}

/// fn_801654E0 (0x801654e0, 104 bytes)
#[inline(never)]
pub fn f_801654e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801654e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801654e0); p = true; }
    }
    f_801654e0_b(c, 0x801654e0);
    if p { crate::prof::exit(c, 0x801654e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801654e0_at(c: &mut Cpu, pc: u32) { f_801654e0_b(c, pc) }
#[inline(always)]
fn f_801654e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801654e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801654e0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801654e0 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[6] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8610));
                c.r[6] = c.r[6].wrapping_add(c.r[0]);
                c.r[6] = c.lbz_t(ins, c.r[6].wrapping_add(0x3));
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x8016551c; continue; }
                pc = 0x8016550c;
            }
            0x8016550c => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8614));
                c.r[0] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(c.r[0]));
                pc = 0x80165520; continue;
                pc = 0x8016551c;
            }
            0x8016551c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x80165520;
            }
            0x80165520 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80165530; continue; }
                pc = 0x80165528;
            }
            0x80165528 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80165538; continue;
                pc = 0x80165530;
            }
            0x80165530 => { ins += 1;
                c.lr = 0x80165534; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165ee0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165534;
            }
            0x80165534 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80165538;
            }
            0x80165538 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801654e0: bad pc {:#x}", pc),
        }
    }
}

/// soundResumeMusic (0x80165548, 52 bytes)
#[inline(never)]
pub fn f_80165548(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165548) { return; }
        if c.prof { crate::prof::enter(c, 0x80165548); p = true; }
    }
    f_80165548_b(c, 0x80165548);
    if p { crate::prof::exit(c, 0x80165548); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165548_at(c: &mut Cpu, pc: u32) { f_80165548_b(c, pc) }
#[inline(always)]
fn f_80165548_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165548;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165548, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165548 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x8016556c; continue; }
                pc = 0x80165560;
            }
            0x80165560 => { ins += 3;
                c.r[4] = 0x3e8;
                c.r[5] = 0x0;
                c.lr = 0x8016556c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801667d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016556c;
            }
            0x8016556c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165548: bad pc {:#x}", pc),
        }
    }
}

/// soundPauseMusic (0x8016557c, 88 bytes)
#[inline(never)]
pub fn f_8016557c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016557c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016557c); p = true; }
    }
    f_8016557c_b(c, 0x8016557c);
    if p { crate::prof::exit(c, 0x8016557c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016557c_at(c: &mut Cpu, pc: u32) { f_8016557c_b(c, pc) }
#[inline(always)]
fn f_8016557c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016557c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016557c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016557c => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x406;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80165598; { c.steps = st; c.instrs = ins; c.cr = cr; f_801662e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165598;
            }
            0x80165598 => { ins += 4;
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.r[31].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x801655bc; continue; }
                pc = 0x801655a8;
            }
            0x801655a8 => { ins += 3;
                c.r[4] = 0x32;
                c.r[5] = 0x0;
                c.lr = 0x801655b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801668dc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801655b4;
            }
            0x801655b4 => { ins += 2;
                c.r[3] = 0x32;
                c.lr = 0x801655bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165d0c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801655bc;
            }
            0x801655bc => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016557c: bad pc {:#x}", pc),
        }
    }
}

/// soundOpenWaveArchive (0x801655d4, 148 bytes)
#[inline(never)]
pub fn f_801655d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801655d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801655d4); p = true; }
    }
    f_801655d4_b(c, 0x801655d4);
    if p { crate::prof::exit(c, 0x801655d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801655d4_at(c: &mut Cpu, pc: u32) { f_801655d4_b(c, pc) }
#[inline(always)]
fn f_801655d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801655d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801655d4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801655d4 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8790));
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 1) { pc = 0x8016560c; continue; }
                pc = 0x801655f8;
            }
            0x801655f8 => { ins += 5;
                c.r[3] = 0x80270000;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[3].wrapping_add(0x3548);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x8016560c; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016560c;
            }
            0x8016560c => { ins += 11;
                c.r[0] = c.r[31].wrapping_mul(0x18);
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8794));
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = 0x0;
                c.r[31] = c.r[4].wrapping_add(c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0x14));
                c.r[6] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                c.r[7] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                c.r[8] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                c.r[9] = c.lwz_t(ins, c.r[31].wrapping_add(0x10));
                c.lr = 0x80165638; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166be0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165638;
            }
            0x80165638 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80165654; continue; }
                pc = 0x80165640;
            }
            0x80165640 => { ins += 5;
                c.r[3] = 0x80270000;
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0x14));
                c.r[3] = c.r[3].wrapping_add(0x3568);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80165654; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165654;
            }
            0x80165654 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801655d4: bad pc {:#x}", pc),
        }
    }
}

/// std130_soundUnk_801653cc (0x80165668, 112 bytes)
#[inline(never)]
pub fn f_80165668(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165668) { return; }
        if c.prof { crate::prof::enter(c, 0x80165668); p = true; }
    }
    f_80165668_b(c, 0x80165668);
    if p { crate::prof::exit(c, 0x80165668); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165668_at(c: &mut Cpu, pc: u32) { f_80165668_b(c, pc) }
#[inline(always)]
fn f_80165668_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165668;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165668, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165668 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.lr = 0x80165690; { c.steps = st; c.instrs = ins; c.cr = cr; f_8016557c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165690;
            }
            0x80165690 => { ins += 6;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x801656a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801653cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801656a8;
            }
            0x801656a8 => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.r[0] | c.r[0];
                c.lr = 0x801656b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165548(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801656b8;
            }
            0x801656b8 => { ins += 8;
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
            _ => panic!("f_80165668: bad pc {:#x}", pc),
        }
    }
}

/// fn_801656D8 (0x801656d8, 32 bytes)
#[inline(never)]
pub fn f_801656d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801656d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801656d8); p = true; }
    }
    f_801656d8_b(c, 0x801656d8);
    if p { crate::prof::exit(c, 0x801656d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801656d8_at(c: &mut Cpu, pc: u32) { f_801656d8_b(c, pc) }
#[inline(always)]
fn f_801656d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801656d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801656d8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801656d8 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801656e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166098(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801656e8;
            }
            0x801656e8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801656d8: bad pc {:#x}", pc),
        }
    }
}

/// std137_soundStopAll (0x801656f8, 32 bytes)
#[inline(never)]
pub fn f_801656f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801656f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801656f8); p = true; }
    }
    f_801656f8_b(c, 0x801656f8);
    if p { crate::prof::exit(c, 0x801656f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801656f8_at(c: &mut Cpu, pc: u32) { f_801656f8_b(c, pc) }
#[inline(always)]
fn f_801656f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801656f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801656f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801656f8 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80165708; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166670(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165708;
            }
            0x80165708 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801656f8: bad pc {:#x}", pc),
        }
    }
}

/// std109_soundPlayEffectVariant (0x80165718, 44 bytes)
#[inline(never)]
pub fn f_80165718(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165718) { return; }
        if c.prof { crate::prof::enter(c, 0x80165718); p = true; }
    }
    f_80165718_b(c, 0x80165718);
    if p { crate::prof::exit(c, 0x80165718); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165718_at(c: &mut Cpu, pc: u32) { f_80165718_b(c, pc) }
#[inline(always)]
fn f_80165718_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165718;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165718, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165718 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80165730; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166d48(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165730;
            }
            0x80165730 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165718: bad pc {:#x}", pc),
        }
    }
}

/// soundIsAnyPlaying (0x80165744, 68 bytes)
#[inline(never)]
pub fn f_80165744(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165744) { return; }
        if c.prof { crate::prof::enter(c, 0x80165744); p = true; }
    }
    f_80165744_b(c, 0x80165744);
    if p { crate::prof::exit(c, 0x80165744); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165744_at(c: &mut Cpu, pc: u32) { f_80165744_b(c, pc) }
#[inline(always)]
fn f_80165744_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165744;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165744, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165744 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffff9ea0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x80165764; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165764;
            }
            0x80165764 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8016576c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801664f0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016576c;
            }
            0x8016576c => { ins += 7;
                c.r[0] = c.r[3].wrapping_neg();
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165744: bad pc {:#x}", pc),
        }
    }
}

/// soundPlayAtPosition (0x80165788, 72 bytes)
#[inline(never)]
pub fn f_80165788(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165788) { return; }
        if c.prof { crate::prof::enter(c, 0x80165788); p = true; }
    }
    f_80165788_b(c, 0x80165788);
    if p { crate::prof::exit(c, 0x80165788); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165788_at(c: &mut Cpu, pc: u32) { f_80165788_b(c, pc) }
#[inline(always)]
fn f_80165788_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165788;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165788, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165788 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x801657a4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801657a4;
            }
            0x801657a4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x801657b0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166308(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801657b0;
            }
            0x801657b0 => { ins += 8;
                c.r[0] = c.r[3].wrapping_neg();
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165788: bad pc {:#x}", pc),
        }
    }
}

/// scriptSoundStop (0x801657d0, 40 bytes)
#[inline(never)]
pub fn f_801657d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801657d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801657d0); p = true; }
    }
    f_801657d0_b(c, 0x801657d0);
    if p { crate::prof::exit(c, 0x801657d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801657d0_at(c: &mut Cpu, pc: u32) { f_801657d0_b(c, pc) }
#[inline(always)]
fn f_801657d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801657d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801657d0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801657d0 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa888));
                c.lr = 0x801657e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801657f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801657e8;
            }
            0x801657e8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801657d0: bad pc {:#x}", pc),
        }
    }
}

/// soundStop (0x801657f8, 260 bytes)
#[inline(never)]
pub fn f_801657f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801657f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801657f8); p = true; }
    }
    f_801657f8_b(c, 0x801657f8);
    if p { crate::prof::exit(c, 0x801657f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801657f8_at(c: &mut Cpu, pc: u32) { f_801657f8_b(c, pc) }
#[inline(always)]
fn f_801657f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801657f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801657f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801657f8 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[29]);
                if cr_bit(cr, 2) { pc = 0x8016582c; continue; }
                pc = 0x8016581c;
            }
            0x8016581c => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8788));
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80165834; continue; }
                pc = 0x8016582c;
            }
            0x8016582c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801658e0; continue;
                pc = 0x80165834;
            }
            0x80165834 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa88c));
                c.r[31] = 0x0;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[29]);
                if !cr_bit(cr, 2) { pc = 0x80165850; continue; }
                pc = 0x80165844;
            }
            0x80165844 => { ins += 3;
                c.r[0] = 0x0;
                c.r[31] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa88c), c.r[0]);
                pc = 0x80165850;
            }
            0x80165850 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa888));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[29]);
                if !cr_bit(cr, 2) { pc = 0x80165868; continue; }
                pc = 0x8016585c;
            }
            0x8016585c => { ins += 3;
                c.r[0] = 0x0;
                c.r[31] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa888), c.r[0]);
                pc = 0x80165868;
            }
            0x80165868 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80165870; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166084(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165870;
            }
            0x80165870 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801658c8; continue; }
                pc = 0x80165878;
            }
            0x80165878 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.lr = 0x80165888; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165c70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165888;
            }
            0x80165888 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x801658bc; continue; }
                pc = 0x80165890;
            }
            0x80165890 => { ins += 5;
                c.r[3] = 0x80270000;
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.r[3].wrapping_add(0x358c);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x801658a4; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801658a4;
            }
            0x801658a4 => { ins += 5;
                c.r[6] = c.r[31] | c.r[31];
                c.r[3] = 0x1;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x801658b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165a44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801658b8;
            }
            0x801658b8 => { ins += 1;
                pc = 0x801658e0; continue;
                pc = 0x801658bc;
            }
            0x801658bc => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801658c4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166b18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801658c4;
            }
            0x801658c4 => { ins += 1;
                pc = 0x801658e0; continue;
                pc = 0x801658c8;
            }
            0x801658c8 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.lr = 0x801658d8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801669e4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801658d8;
            }
            0x801658d8 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801658e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166b18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801658e0;
            }
            0x801658e0 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801657f8: bad pc {:#x}", pc),
        }
    }
}

/// fn_801658FC (0x801658fc, 128 bytes)
#[inline(never)]
pub fn f_801658fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801658fc) { return; }
        if c.prof { crate::prof::enter(c, 0x801658fc); p = true; }
    }
    f_801658fc_b(c, 0x801658fc);
    if p { crate::prof::exit(c, 0x801658fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801658fc_at(c: &mut Cpu, pc: u32) { f_801658fc_b(c, pc) }
#[inline(always)]
fn f_801658fc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801658fc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801658fc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801658fc => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[29]);
                if cr_bit(cr, 2) { pc = 0x80165934; continue; }
                pc = 0x80165924;
            }
            0x80165924 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8788));
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8016593c; continue; }
                pc = 0x80165934;
            }
            0x80165934 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80165960; continue;
                pc = 0x8016593c;
            }
            0x8016593c => { ins += 4;
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.lr = 0x8016594c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165c70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016594c;
            }
            0x8016594c => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[6] = 0x0;
                c.lr = 0x80165960; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165a44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165960;
            }
            0x80165960 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801658fc: bad pc {:#x}", pc),
        }
    }
}

/// soundPlayEffectEx (0x8016597c, 128 bytes)
#[inline(never)]
pub fn f_8016597c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016597c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016597c); p = true; }
    }
    f_8016597c_b(c, 0x8016597c);
    if p { crate::prof::exit(c, 0x8016597c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016597c_at(c: &mut Cpu, pc: u32) { f_8016597c_b(c, pc) }
#[inline(always)]
fn f_8016597c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016597c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016597c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016597c => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[29]);
                if cr_bit(cr, 2) { pc = 0x801659b4; continue; }
                pc = 0x801659a4;
            }
            0x801659a4 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8788));
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801659bc; continue; }
                pc = 0x801659b4;
            }
            0x801659b4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801659e0; continue;
                pc = 0x801659bc;
            }
            0x801659bc => { ins += 4;
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x801659cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165c70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801659cc;
            }
            0x801659cc => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[6] = 0x1;
                c.lr = 0x801659e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165a44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801659e0;
            }
            0x801659e0 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016597c: bad pc {:#x}", pc),
        }
    }
}

/// soundPlayEffect (0x801659fc, 36 bytes)
#[inline(never)]
pub fn f_801659fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801659fc) { return; }
        if c.prof { crate::prof::enter(c, 0x801659fc); p = true; }
    }
    f_801659fc_b(c, 0x801659fc);
    if p { crate::prof::exit(c, 0x801659fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801659fc_at(c: &mut Cpu, pc: u32) { f_801659fc_b(c, pc) }
#[inline(always)]
fn f_801659fc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801659fc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801659fc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801659fc => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80165a10; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165a44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165a10;
            }
            0x80165a10 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801659fc: bad pc {:#x}", pc),
        }
    }
}

/// soundSetBackgroundMusic (0x80165a20, 36 bytes)
#[inline(never)]
pub fn f_80165a20(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165a20) { return; }
        if c.prof { crate::prof::enter(c, 0x80165a20); p = true; }
    }
    f_80165a20_b(c, 0x80165a20);
    if p { crate::prof::exit(c, 0x80165a20); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165a20_at(c: &mut Cpu, pc: u32) { f_80165a20_b(c, pc) }
#[inline(always)]
fn f_80165a20_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165a20;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165a20, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165a20 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80165a34; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165a44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165a34;
            }
            0x80165a34 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165a20: bad pc {:#x}", pc),
        }
    }
}

/// soundPlay (0x80165a44, 556 bytes)
#[inline(never)]
pub fn f_80165a44(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165a44) { return; }
        if c.prof { crate::prof::enter(c, 0x80165a44); p = true; }
    }
    f_80165a44_b(c, 0x80165a44);
    if p { crate::prof::exit(c, 0x80165a44); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165a44_at(c: &mut Cpu, pc: u32) { f_80165a44_b(c, pc) }
#[inline(never)]
fn f_80165a44_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165a44;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165a44, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165a44 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[26]);
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[29] = c.r[6] | c.r[6];
                if cr_bit(cr, 2) { pc = 0x80165a78; continue; }
                pc = 0x80165a68;
            }
            0x80165a68 => { ins += 4;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8788));
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80165a80; continue; }
                pc = 0x80165a78;
            }
            0x80165a78 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80165c5c; continue;
                pc = 0x80165a80;
            }
            0x80165a80 => { ins += 6;
                c.r[4] = c.r[26].wrapping_mul(0xc);
                c.r[5] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[0] = c.lbz_t(ins, c.r[5].wrapping_add(c.r[4]));
                c.r[0] = c.r[0].rotate_left(25) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80165af4; continue; }
                pc = 0x80165a98;
            }
            0x80165a98 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80165adc; continue; }
                pc = 0x80165aa0;
            }
            0x80165aa0 => { ins += 1;
                c.lr = 0x80165aa4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801666bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165aa4;
            }
            0x80165aa4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80165ad8; continue; }
                pc = 0x80165aac;
            }
            0x80165aac => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x80165ad8; continue; }
                pc = 0x80165ab0;
            }
            0x80165ab0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x4);
                if !cr_bit(cr, 0) { pc = 0x80165ad8; continue; }
                pc = 0x80165ab8;
            }
            0x80165ab8 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x32;
                c.r[5] = 0x0;
                c.lr = 0x80165ac8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166670(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165ac8;
            }
            0x80165ac8 => { ins += 2;
                c.r[3] = 0x32;
                c.lr = 0x80165ad0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165d0c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165ad0;
            }
            0x80165ad0 => { ins += 2;
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x80165ad8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166b18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165ad8;
            }
            0x80165ad8 => { ins += 1;
                c.stw(c.r[13].wrapping_add(0xffffa888), c.r[26]);
                pc = 0x80165adc;
            }
            0x80165adc => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = 0x0;
                c.lr = 0x80165af0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166a50(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165af0;
            }
            0x80165af0 => { ins += 1;
                pc = 0x80165c5c; continue;
                pc = 0x80165af4;
            }
            0x80165af4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x0);
                if cr_bit(cr, 2) { pc = 0x80165b08; continue; }
                pc = 0x80165afc;
            }
            0x80165afc => { ins += 3;
                c.r[31] = 0x406;
                c.r[30] = 0x10000;
                pc = 0x80165b10; continue;
                pc = 0x80165b08;
            }
            0x80165b08 => { ins += 2;
                c.r[31] = 0x407;
                c.r[30] = 0x2000;
                pc = 0x80165b10;
            }
            0x80165b10 => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8610));
                c.r[4] = c.r[5].wrapping_add(c.r[4]);
                c.r[25] = c.lbz_t(ins, c.r[4].wrapping_add(0x3));
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[25], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80165b44; continue; }
                pc = 0x80165b28;
            }
            0x80165b28 => { ins += 5;
                c.r[3] = 0x80270000;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[3].wrapping_add(0x35c4);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80165b3c; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165b3c;
            }
            0x80165b3c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80165c5c; continue;
                pc = 0x80165b44;
            }
            0x80165b44 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.lr = 0x80165b50; { c.steps = st; c.instrs = ins; c.cr = cr; f_801662e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165b50;
            }
            0x80165b50 => { ins += 3;
                c.r[24] = c.r[3] | c.r[3];
                cr_cmpu(&mut cr, c.xer, 0, c.r[24], c.r[26]);
                if cr_bit(cr, 2) { pc = 0x80165c30; continue; }
                pc = 0x80165b5c;
            }
            0x80165b5c => { ins += 3;
                c.r[0] = c.r[24].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x80165ba0; continue; }
                pc = 0x80165b68;
            }
            0x80165b68 => { ins += 1;
                c.lr = 0x80165b6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801666bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165b6c;
            }
            0x80165b6c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80165ba0; continue; }
                pc = 0x80165b74;
            }
            0x80165b74 => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x80165ba0; continue; }
                pc = 0x80165b78;
            }
            0x80165b78 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x4);
                if !cr_bit(cr, 0) { pc = 0x80165ba0; continue; }
                pc = 0x80165b80;
            }
            0x80165b80 => { ins += 4;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x32;
                c.r[5] = 0x0;
                c.lr = 0x80165b90; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166670(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165b90;
            }
            0x80165b90 => { ins += 2;
                c.r[3] = 0x32;
                c.lr = 0x80165b98; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165d0c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165b98;
            }
            0x80165b98 => { ins += 2;
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x80165ba0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166b18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165ba0;
            }
            0x80165ba0 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8614));
                c.r[0] = c.r[25].rotate_left(3) & 0xfffffff8;
                c.r[24] = c.lwz_t(ins, c.r[3].wrapping_add(c.r[0]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[24], 0x0);
                if cr_bit(cr, 2) { pc = 0x80165bd4; continue; }
                pc = 0x80165bb4;
            }
            0x80165bb4 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.lr = 0x80165bc0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800f9318(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165bc0;
            }
            0x80165bc0 => { ins += 4;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[24] | c.r[24];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80165bd0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165ee0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165bd0;
            }
            0x80165bd0 => { ins += 1;
                pc = 0x80165c20; continue;
                pc = 0x80165bd4;
            }
            0x80165bd4 => { ins += 4;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[25] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[25], 0x0);
                if cr_bit(cr, 2) { pc = 0x80165c04; continue; }
                pc = 0x80165be4;
            }
            0x80165be4 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.lr = 0x80165bf0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800f9318(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165bf0;
            }
            0x80165bf0 => { ins += 4;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80165c00; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165dec(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165c00;
            }
            0x80165c00 => { ins += 1;
                pc = 0x80165c20; continue;
                pc = 0x80165c04;
            }
            0x80165c04 => { ins += 5;
                c.r[3] = 0x80270000;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[3].wrapping_add(0x35f8);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80165c18; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165c18;
            }
            0x80165c18 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80165c5c; continue;
                pc = 0x80165c20;
            }
            0x80165c20 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x80165c30; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166b3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165c30;
            }
            0x80165c30 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x0);
                if cr_bit(cr, 2) { pc = 0x80165c3c; continue; }
                pc = 0x80165c38;
            }
            0x80165c38 => { ins += 1;
                c.stw(c.r[13].wrapping_add(0xffffa88c), c.r[26]);
                pc = 0x80165c3c;
            }
            0x80165c3c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80165c48; continue; }
                pc = 0x80165c44;
            }
            0x80165c44 => { ins += 1;
                c.stw(c.r[13].wrapping_add(0xffffa888), c.r[26]);
                pc = 0x80165c48;
            }
            0x80165c48 => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = 0x0;
                c.lr = 0x80165c5c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166a50(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165c5c;
            }
            0x80165c5c => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165a44: bad pc {:#x}", pc),
        }
    }
}

/// soundFadeSlot (0x80165c70, 156 bytes)
#[inline(never)]
pub fn f_80165c70(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165c70) { return; }
        if c.prof { crate::prof::enter(c, 0x80165c70); p = true; }
    }
    f_80165c70_b(c, 0x80165c70);
    if p { crate::prof::exit(c, 0x80165c70); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165c70_at(c: &mut Cpu, pc: u32) { f_80165c70_b(c, pc) }
#[inline(always)]
fn f_80165c70_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165c70;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165c70, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165c70 => { ins += 14;
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
                c.lr = 0x80165ca8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801662e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165ca8;
            }
            0x80165ca8 => { ins += 4;
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.r[31].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x80165cf0; continue; }
                pc = 0x80165cb8;
            }
            0x80165cb8 => { ins += 1;
                c.lr = 0x80165cbc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801666bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165cbc;
            }
            0x80165cbc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80165cf0; continue; }
                pc = 0x80165cc4;
            }
            0x80165cc4 => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x80165cf0; continue; }
                pc = 0x80165cc8;
            }
            0x80165cc8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x4);
                if !cr_bit(cr, 0) { pc = 0x80165cf0; continue; }
                pc = 0x80165cd0;
            }
            0x80165cd0 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.lr = 0x80165ce0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166670(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165ce0;
            }
            0x80165ce0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x80165cf0; continue; }
                pc = 0x80165ce8;
            }
            0x80165ce8 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80165cf0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165d0c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165cf0;
            }
            0x80165cf0 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165c70: bad pc {:#x}", pc),
        }
    }
}

/// soundWaitMs (0x80165d0c, 224 bytes)
#[inline(never)]
pub fn f_80165d0c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165d0c) { return; }
        if c.prof { crate::prof::enter(c, 0x80165d0c); p = true; }
    }
    f_80165d0c_b(c, 0x80165d0c);
    if p { crate::prof::exit(c, 0x80165d0c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165d0c_at(c: &mut Cpu, pc: u32) { f_80165d0c_b(c, pc) }
#[inline(always)]
fn f_80165d0c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165d0c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165d0c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165d0c => { ins += 25;
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
            0x80165d70 => { ins += 1;
                c.lr = 0x80165d74; { c.steps = st; c.instrs = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165d74;
            }
            0x80165d74 => { ins += 1;
                c.lr = 0x80165d78; { c.steps = st; c.instrs = ins; c.cr = cr; f_800d37cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165d78;
            }
            0x80165d78 => { ins += 6;
                c.r[0] = c.r[3] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[31]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[30]); c.f[31] = v; c.ps1[31] = v; }
                c.lr = 0x80165d90; { c.steps = st; c.instrs = ins; c.cr = cr; f_800d3088(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165d90;
            }
            0x80165d90 => { ins += 6;
                c.stw(c.r[1].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[31]);
                { let ea = c.r[1].wrapping_add(0x10); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[29]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] / c.f[31]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[27] + c.f[0]); c.f[27] = v; c.ps1[27] = v; }
                pc = 0x80165da8;
            }
            0x80165da8 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[27], c.f[28]);
                if cr_bit(cr, 0) { pc = 0x80165d70; continue; }
                pc = 0x80165db0;
            }
            0x80165db0 => { ins += 15;
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
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x74));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165d0c: bad pc {:#x}", pc),
        }
    }
}

/// soundReadStreamFile (0x80165dec, 244 bytes)
#[inline(never)]
pub fn f_80165dec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165dec) { return; }
        if c.prof { crate::prof::enter(c, 0x80165dec); p = true; }
    }
    f_80165dec_b(c, 0x80165dec);
    if p { crate::prof::exit(c, 0x80165dec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165dec_at(c: &mut Cpu, pc: u32) { f_80165dec_b(c, pc) }
#[inline(always)]
fn f_80165dec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165dec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165dec, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165dec => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[31] = c.r[6].wrapping_add(0x3548);
                c.lr = 0x80165e14; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167f28(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165e14;
            }
            0x80165e14 => { ins += 2;
                c.r[29] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[29]);
                if !cr_bit(cr, 2) { pc = 0x80165e30; continue; }
                pc = 0x80165e1c;
            }
            0x80165e1c => { ins += 4;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[31].wrapping_add(0xe0);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80165e2c; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165e2c;
            }
            0x80165e2c => { ins += 1;
                pc = 0x80165ecc; continue;
                pc = 0x80165e30;
            }
            0x80165e30 => { ins += 1;
                c.lr = 0x80165e34; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167e5c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165e34;
            }
            0x80165e34 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80165e50; continue; }
                pc = 0x80165e3c;
            }
            0x80165e3c => { ins += 4;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[31].wrapping_add(0xfc);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80165e4c; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165e4c;
            }
            0x80165e4c => { ins += 1;
                pc = 0x80165ec4; continue;
                pc = 0x80165e50;
            }
            0x80165e50 => { ins += 4;
                c.r[0] = c.r[3].wrapping_add(0x1f);
                c.r[30] = c.r[0].rotate_left(0) & 0xffffffe0;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[28]);
                if !cr_bit(cr, 0) { pc = 0x80165eb8; continue; }
                pc = 0x80165e60;
            }
            0x80165e60 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80165e78; continue; }
                pc = 0x80165e68;
            }
            0x80165e68 => { ins += 3;
                c.r[3] = c.r[31].wrapping_add(0x11c);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80165e74; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165e74;
            }
            0x80165e74 => { ins += 1;
                pc = 0x80165ec4; continue;
                pc = 0x80165e78;
            }
            0x80165e78 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = 0xe0;
                c.lr = 0x80165e88; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165e88;
            }
            0x80165e88 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = 0x0;
                c.lr = 0x80165e9c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165e9c;
            }
            0x80165e9c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 1) { pc = 0x80165ec4; continue; }
                pc = 0x80165ea4;
            }
            0x80165ea4 => { ins += 4;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[31].wrapping_add(0x140);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80165eb4; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165eb4;
            }
            0x80165eb4 => { ins += 1;
                pc = 0x80165ec4; continue;
                pc = 0x80165eb8;
            }
            0x80165eb8 => { ins += 3;
                c.r[3] = c.r[31].wrapping_add(0x168);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80165ec4; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165ec4;
            }
            0x80165ec4 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80165ecc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167e64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165ecc;
            }
            0x80165ecc => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165dec: bad pc {:#x}", pc),
        }
    }
}

/// fn_80165EE0 (0x80165ee0, 96 bytes)
#[inline(never)]
pub fn f_80165ee0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165ee0) { return; }
        if c.prof { crate::prof::enter(c, 0x80165ee0); p = true; }
    }
    f_80165ee0_b(c, 0x80165ee0);
    if p { crate::prof::exit(c, 0x80165ee0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165ee0_at(c: &mut Cpu, pc: u32) { f_80165ee0_b(c, pc) }
#[inline(always)]
fn f_80165ee0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165ee0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165ee0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165ee0 => { ins += 12;
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
                c.lr = 0x80165f10; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165f10;
            }
            0x80165f10 => { ins += 5;
                c.stw(c.r[13].wrapping_add(0xffffa890), c.r[31]);
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x99;
                c.stw(c.r[13].wrapping_add(0xffffa894), c.r[30]);
                c.lr = 0x80165f24; { c.steps = st; c.instrs = ins; c.cr = cr; f_8017af6c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165f24;
            }
            0x80165f24 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165ee0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80165F40 (0x80165f40, 68 bytes)
#[inline(never)]
pub fn f_80165f40(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165f40) { return; }
        if c.prof { crate::prof::enter(c, 0x80165f40); p = true; }
    }
    f_80165f40_b(c, 0x80165f40);
    if p { crate::prof::exit(c, 0x80165f40); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165f40_at(c: &mut Cpu, pc: u32) { f_80165f40_b(c, pc) }
#[inline(always)]
fn f_80165f40_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165f40;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165f40, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165f40 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x406;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80165f58; { c.steps = st; c.instrs = ins; c.cr = cr; f_800f9318(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165f58;
            }
            0x80165f58 => { ins += 5;
                c.r[0] = 0x10000;
                c.stw(c.r[13].wrapping_add(0xffffa894), c.r[3]);
                c.r[3] = 0x99;
                c.stw(c.r[13].wrapping_add(0xffffa890), c.r[0]);
                c.lr = 0x80165f6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8017b370(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165f6c;
            }
            0x80165f6c => { ins += 2;
                c.r[3] = 0x99;
                c.lr = 0x80165f74; { c.steps = st; c.instrs = ins; c.cr = cr; f_80165fdc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165f74;
            }
            0x80165f74 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165f40: bad pc {:#x}", pc),
        }
    }
}

/// fn_80165F84 (0x80165f84, 88 bytes)
#[inline(never)]
pub fn f_80165f84(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165f84) { return; }
        if c.prof { crate::prof::enter(c, 0x80165f84); p = true; }
    }
    f_80165f84_b(c, 0x80165f84);
    if p { crate::prof::exit(c, 0x80165f84); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165f84_at(c: &mut Cpu, pc: u32) { f_80165f84_b(c, pc) }
#[inline(always)]
fn f_80165f84_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165f84;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165f84, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165f84 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x10000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80165f98; { c.steps = st; c.instrs = ins; c.cr = cr; f_8016604c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165f98;
            }
            0x80165f98 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x406;
                c.r[6] = 0x0;
                c.lr = 0x80165fa8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800f9378(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165fa8;
            }
            0x80165fa8 => { ins += 2;
                c.r[3] = 0x2000;
                c.lr = 0x80165fb0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8016604c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165fb0;
            }
            0x80165fb0 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x407;
                c.r[6] = 0x0;
                c.lr = 0x80165fc0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800f9378(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80165fc0;
            }
            0x80165fc0 => { ins += 7;
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa88c), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa888), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165f84: bad pc {:#x}", pc),
        }
    }
}

/// soundWaitGroup (0x80165fdc, 112 bytes)
#[inline(never)]
pub fn f_80165fdc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80165fdc) { return; }
        if c.prof { crate::prof::enter(c, 0x80165fdc); p = true; }
    }
    f_80165fdc_b(c, 0x80165fdc);
    if p { crate::prof::exit(c, 0x80165fdc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80165fdc_at(c: &mut Cpu, pc: u32) { f_80165fdc_b(c, pc) }
#[inline(always)]
fn f_80165fdc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80165fdc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80165fdc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80165fdc => { ins += 9;
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
            0x80166000 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80166008; { c.steps = st; c.instrs = ins; c.cr = cr; f_8017b2cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166008;
            }
            0x80166008 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if !cr_bit(cr, 0) { pc = 0x80166020; continue; }
                pc = 0x80166010;
            }
            0x80166010 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[29] | c.r[29];
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80166020; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166020;
            }
            0x80166020 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x80166030; continue; }
                pc = 0x80166028;
            }
            0x80166028 => { ins += 1;
                c.lr = 0x8016602c; { c.steps = st; c.instrs = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016602c;
            }
            0x8016602c => { ins += 1;
                pc = 0x80166000; continue;
                pc = 0x80166030;
            }
            0x80166030 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80165fdc: bad pc {:#x}", pc),
        }
    }
}

/// fn_8016604C (0x8016604c, 56 bytes)
#[inline(never)]
pub fn f_8016604c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016604c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016604c); p = true; }
    }
    f_8016604c_b(c, 0x8016604c);
    if p { crate::prof::exit(c, 0x8016604c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016604c_at(c: &mut Cpu, pc: u32) { f_8016604c_b(c, pc) }
#[inline(always)]
fn f_8016604c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016604c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016604c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016604c => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x20;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80166060; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e2c04(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166060;
            }
            0x80166060 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80166070; continue; }
                pc = 0x80166068;
            }
            0x80166068 => { ins += 1;
                c.lr = 0x8016606c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e27b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016606c;
            }
            0x8016606c => { ins += 1;
                pc = 0x80166074; continue;
                pc = 0x80166070;
            }
            0x80166070 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x80166074;
            }
            0x80166074 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016604c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166084 (0x80166084, 20 bytes)
#[inline(never)]
pub fn f_80166084(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166084) { return; }
        if c.prof { crate::prof::enter(c, 0x80166084); p = true; }
    }
    f_80166084_b(c, 0x80166084);
    if p { crate::prof::exit(c, 0x80166084); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166084_at(c: &mut Cpu, pc: u32) { f_80166084_b(c, pc) }
#[inline(always)]
fn f_80166084_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166084;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166084, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166084 => { ins += 5;
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[0]));
                c.r[3] = c.r[0].rotate_left(25) & 0x1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166084: bad pc {:#x}", pc),
        }
    }
}

/// fn_80166098 (0x80166098, 64 bytes)
#[inline(never)]
pub fn f_80166098(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166098) { return; }
        if c.prof { crate::prof::enter(c, 0x80166098); p = true; }
    }
    f_80166098_b(c, 0x80166098);
    if p { crate::prof::exit(c, 0x80166098); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166098_at(c: &mut Cpu, pc: u32) { f_80166098_b(c, pc) }
#[inline(always)]
fn f_80166098_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166098;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166098, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166098 => { ins += 7;
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[3]);
                c.r[0] = c.r[0].rotate_left(27) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x801660bc; continue; }
                pc = 0x801660b4;
            }
            0x801660b4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801660bc;
            }
            0x801660bc => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x801660d0; continue; }
                pc = 0x801660c8;
            }
            0x801660c8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801660d0;
            }
            0x801660d0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166098: bad pc {:#x}", pc),
        }
    }
}

/// soundUnduck (0x801660d8, 144 bytes)
#[inline(never)]
pub fn f_801660d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801660d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801660d8); p = true; }
    }
    f_801660d8_b(c, 0x801660d8);
    if p { crate::prof::exit(c, 0x801660d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801660d8_at(c: &mut Cpu, pc: u32) { f_801660d8_b(c, pc) }
#[inline(always)]
fn f_801660d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801660d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801660d8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801660d8 => { ins += 10;
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
            0x80166100 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[4] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[31]));
                c.r[0] = c.r[4].rotate_left(27) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8016613c; continue; }
                pc = 0x80166114;
            }
            0x80166114 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(25) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8016612c; continue; }
                pc = 0x80166120;
            }
            0x80166120 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if cr_bit(cr, 2) { pc = 0x8016613c; continue; }
                pc = 0x80166128;
            }
            0x80166128 => { ins += 1;
                pc = 0x80166134; continue;
                pc = 0x8016612c;
            }
            0x8016612c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x8016613c; continue; }
                pc = 0x80166134;
            }
            0x80166134 => { ins += 2;
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8016613c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166168(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016613c;
            }
            0x8016613c => { ins += 2;
                c.r[31] = c.r[31].wrapping_add(0xc);
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x80166144;
            }
            0x80166144 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa8c8));
                c.r[3] = c.r[30] | c.r[30];
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80166100; continue; }
                pc = 0x80166154;
            }
            0x80166154 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801660d8: bad pc {:#x}", pc),
        }
    }
}

/// _soundUnduckSlot (0x80166168, 104 bytes)
#[inline(never)]
pub fn f_80166168(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166168) { return; }
        if c.prof { crate::prof::enter(c, 0x80166168); p = true; }
    }
    f_80166168_b(c, 0x80166168);
    if p { crate::prof::exit(c, 0x80166168); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166168_at(c: &mut Cpu, pc: u32) { f_80166168_b(c, pc) }
#[inline(always)]
fn f_80166168_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166168;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166168, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166168 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = c.r[3].wrapping_mul(0xc);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[0] = c.r[5].wrapping_add(0x8);
                c.r[6] = c.lwz_t(ins, c.r[6].wrapping_add(c.r[0]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80166194; continue; }
                pc = 0x8016618c;
            }
            0x8016618c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801661c0; continue;
                pc = 0x80166194;
            }
            0x80166194 => { ins += 3;
                c.r[5] = c.lbz_t(ins, c.r[6].wrapping_add(0x4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801661a8; continue; }
                pc = 0x801661a0;
            }
            0x801661a0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801661c0; continue;
                pc = 0x801661a8;
            }
            0x801661a8 => { ins += 6;
                c.r[0] = c.r[5].wrapping_add(0xffffffff);
                c.stb(c.r[6].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[6].wrapping_add(0x4));
                c.r[5] = c.r[6].wrapping_add(c.r[0]);
                c.r[5] = c.lbz_t(ins, c.r[5].wrapping_add(0x5));
                c.lr = 0x801661c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166670(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801661c0;
            }
            0x801661c0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166168: bad pc {:#x}", pc),
        }
    }
}

/// soundDuck (0x801661d0, 152 bytes)
#[inline(never)]
pub fn f_801661d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801661d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801661d0); p = true; }
    }
    f_801661d0_b(c, 0x801661d0);
    if p { crate::prof::exit(c, 0x801661d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801661d0_at(c: &mut Cpu, pc: u32) { f_801661d0_b(c, pc) }
#[inline(always)]
fn f_801661d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801661d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801661d0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801661d0 => { ins += 11;
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
            0x801661fc => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[4] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[31]));
                c.r[0] = c.r[4].rotate_left(27) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8016623c; continue; }
                pc = 0x80166210;
            }
            0x80166210 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(25) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166228; continue; }
                pc = 0x8016621c;
            }
            0x8016621c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if cr_bit(cr, 2) { pc = 0x8016623c; continue; }
                pc = 0x80166224;
            }
            0x80166224 => { ins += 1;
                pc = 0x80166230; continue;
                pc = 0x80166228;
            }
            0x80166228 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x8016623c; continue; }
                pc = 0x80166230;
            }
            0x80166230 => { ins += 3;
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[26] | c.r[26];
                c.lr = 0x8016623c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166268(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016623c;
            }
            0x8016623c => { ins += 2;
                c.r[31] = c.r[31].wrapping_add(0xc);
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x80166244;
            }
            0x80166244 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa8c8));
                c.r[3] = c.r[30] | c.r[30];
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801661fc; continue; }
                pc = 0x80166254;
            }
            0x80166254 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801661d0: bad pc {:#x}", pc),
        }
    }
}

/// _soundDuckSlot (0x80166268, 128 bytes)
#[inline(never)]
pub fn f_80166268(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166268) { return; }
        if c.prof { crate::prof::enter(c, 0x80166268); p = true; }
    }
    f_80166268_b(c, 0x80166268);
    if p { crate::prof::exit(c, 0x80166268); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166268_at(c: &mut Cpu, pc: u32) { f_80166268_b(c, pc) }
#[inline(always)]
fn f_80166268_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166268;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166268, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166268 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = c.r[3].wrapping_mul(0xc);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[0] = c.r[6].wrapping_add(0x8);
                c.r[7] = c.lwz_t(ins, c.r[7].wrapping_add(c.r[0]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80166294; continue; }
                pc = 0x8016628c;
            }
            0x8016628c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801662d8; continue;
                pc = 0x80166294;
            }
            0x80166294 => { ins += 3;
                c.r[6] = c.lbz_t(ins, c.r[7].wrapping_add(0x4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x3);
                if cr_bit(cr, 0) { pc = 0x801662a8; continue; }
                pc = 0x801662a0;
            }
            0x801662a0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801662d8; continue;
                pc = 0x801662a8;
            }
            0x801662a8 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(0x1));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], c.r[5]);
                if !cr_bit(cr, 0) { pc = 0x801662b8; continue; }
                pc = 0x801662b4;
            }
            0x801662b4 => { ins += 1;
                c.r[5] = c.r[0] | c.r[0];
                pc = 0x801662b8;
            }
            0x801662b8 => { ins += 5;
                c.r[6] = c.r[7].wrapping_add(c.r[6]);
                c.stb(c.r[6].wrapping_add(0x5), c.r[0]);
                c.r[6] = c.lbz_t(ins, c.r[7].wrapping_add(0x4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x2);
                if !cr_bit(cr, 0) { pc = 0x801662d4; continue; }
                pc = 0x801662cc;
            }
            0x801662cc => { ins += 2;
                c.r[0] = c.r[6].wrapping_add(0x1);
                c.stb(c.r[7].wrapping_add(0x4), c.r[0]);
                pc = 0x801662d4;
            }
            0x801662d4 => { ins += 1;
                c.lr = 0x801662d8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166670(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801662d8;
            }
            0x801662d8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166268: bad pc {:#x}", pc),
        }
    }
}

/// fn_801662E8 (0x801662e8, 32 bytes)
#[inline(never)]
pub fn f_801662e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801662e8) { return; }
        if c.prof { crate::prof::enter(c, 0x801662e8); p = true; }
    }
    f_801662e8_b(c, 0x801662e8);
    if p { crate::prof::exit(c, 0x801662e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801662e8_at(c: &mut Cpu, pc: u32) { f_801662e8_b(c, pc) }
#[inline(always)]
fn f_801662e8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801662e8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801662e8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801662e8 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801662f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167768(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801662f8;
            }
            0x801662f8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801662e8: bad pc {:#x}", pc),
        }
    }
}

/// soundEmitterAt (0x80166308, 104 bytes)
#[inline(never)]
pub fn f_80166308(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166308) { return; }
        if c.prof { crate::prof::enter(c, 0x80166308); p = true; }
    }
    f_80166308_b(c, 0x80166308);
    if p { crate::prof::exit(c, 0x80166308); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166308_at(c: &mut Cpu, pc: u32) { f_80166308_b(c, pc) }
#[inline(always)]
fn f_80166308_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166308;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166308, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166308 => { ins += 12;
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
                c.lr = 0x80166338; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166338;
            }
            0x80166338 => { ins += 8;
                { let ea = c.r[2].wrapping_add(0xffff9ec4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[30] | c.r[30];
                { let ea = c.r[2].wrapping_add(0xffff9ec0); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[6] = 0x7f;
                c.r[7] = 0x0;
                c.lr = 0x80166358; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166370(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166358;
            }
            0x80166358 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166308: bad pc {:#x}", pc),
        }
    }
}

/// soundEmitterCreate (0x80166370, 232 bytes)
#[inline(never)]
pub fn f_80166370(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166370) { return; }
        if c.prof { crate::prof::enter(c, 0x80166370); p = true; }
    }
    f_80166370_b(c, 0x80166370);
    if p { crate::prof::exit(c, 0x80166370); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166370_at(c: &mut Cpu, pc: u32) { f_80166370_b(c, pc) }
#[inline(always)]
fn f_80166370_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166370;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166370, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166370 => { ins += 17;
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
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[31] = c.r[3].wrapping_add(c.r[0]);
                c.lr = 0x801663b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167864(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801663b4;
            }
            0x801663b4 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x80166438; continue; }
                pc = 0x801663bc;
            }
            0x801663bc => { ins += 11;
                c.f[1] = c.f[30];
                c.r[7] = c.lhz_t(ins, c.r[31].wrapping_add(0x4));
                c.f[2] = c.f[31];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.r[8] = c.r[28] | c.r[28];
                c.r[9] = c.r[29] | c.r[29];
                c.r[3] = c.r[30].wrapping_add(0x28);
                c.r[6] = 0x1f;
                c.r[10] = 0x0;
                c.lr = 0x801663e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015eca8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801663e8;
            }
            0x801663e8 => { ins += 3;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[30].wrapping_add(0x10);
                c.lr = 0x801663f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801663f4;
            }
            0x801663f4 => { ins += 3;
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = c.r[30].wrapping_add(0x1c);
                c.lr = 0x80166400; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166400;
            }
            0x80166400 => { ins += 7;
                c.stb(c.r[30].wrapping_add(0x1), c.r[28]);
                c.r[3] = c.r[30].wrapping_add(0x28);
                c.stb(c.r[30].wrapping_add(0x2), c.r[29]);
                { let ea = c.r[30].wrapping_add(0x8); let v = c.f[30]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0xc); let v = c.f[31]; c.stfs(ea, v); }
                c.stw(c.r[30].wrapping_add(0x4), c.r[25]);
                c.lr = 0x8016641c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015e890(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016641c;
            }
            0x8016641c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166430; continue; }
                pc = 0x80166428;
            }
            0x80166428 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x8016643c; continue;
                pc = 0x80166430;
            }
            0x80166430 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80166438; { c.steps = st; c.instrs = ins; c.cr = cr; f_801677bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166438;
            }
            0x80166438 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x8016643c;
            }
            0x8016643c => { ins += 7;
                { let ea = c.r[1].wrapping_add(0x38); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x30); c.f[30] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166370: bad pc {:#x}", pc),
        }
    }
}

/// soundUpdateListener (0x80166458, 152 bytes)
#[inline(never)]
pub fn f_80166458(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166458) { return; }
        if c.prof { crate::prof::enter(c, 0x80166458); p = true; }
    }
    f_80166458_b(c, 0x80166458);
    if p { crate::prof::exit(c, 0x80166458); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166458_at(c: &mut Cpu, pc: u32) { f_80166458_b(c, pc) }
#[inline(always)]
fn f_80166458_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166458;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166458, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166458 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x801664d4; continue; }
                pc = 0x80166478;
            }
            0x80166478 => { ins += 4;
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[30].wrapping_add(0x10);
                c.lr = 0x80166488; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e0168(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166488;
            }
            0x80166488 => { ins += 8;
                c.r[8] = c.lbz_t(ins, c.r[30].wrapping_add(0x1));
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[30].wrapping_add(0x40);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[6] = c.r[30].wrapping_add(0x28);
                c.r[7] = c.r[30].wrapping_add(0x34);
                c.r[9] = 0x0;
                c.lr = 0x801664a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015ed00(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801664a8;
            }
            0x801664a8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801664d4; continue; }
                pc = 0x801664b4;
            }
            0x801664b4 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[30].wrapping_add(0x10);
                c.lr = 0x801664c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801664c0;
            }
            0x801664c0 => { ins += 3;
                c.r[3] = c.r[30].wrapping_add(0x1c);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x801664cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801664cc;
            }
            0x801664cc => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x801664d8; continue;
                pc = 0x801664d4;
            }
            0x801664d4 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801664d8;
            }
            0x801664d8 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166458: bad pc {:#x}", pc),
        }
    }
}

/// soundSetListener (0x801664f0, 136 bytes)
#[inline(never)]
pub fn f_801664f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801664f0) { return; }
        if c.prof { crate::prof::enter(c, 0x801664f0); p = true; }
    }
    f_801664f0_b(c, 0x801664f0);
    if p { crate::prof::exit(c, 0x801664f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801664f0_at(c: &mut Cpu, pc: u32) { f_801664f0_b(c, pc) }
#[inline(always)]
fn f_801664f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801664f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801664f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801664f0 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffff9ec0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.f[2] = c.f[1];
                c.stw(c.r[1].wrapping_add(0x3c), c.r[31]);
                c.f[3] = c.f[1];
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0x20);
                c.lr = 0x80166518; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166518;
            }
            0x80166518 => { ins += 5;
                { let ea = c.r[2].wrapping_add(0xffff9ec0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x14);
                { let ea = c.r[2].wrapping_add(0xffff9ec8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.f[2] = c.f[1];
                c.lr = 0x8016652c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016652c;
            }
            0x8016652c => { ins += 5;
                { let ea = c.r[2].wrapping_add(0xffff9ec0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x8);
                { let ea = c.r[2].wrapping_add(0xffff9ecc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.f[3] = c.f[1];
                c.lr = 0x80166540; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166540;
            }
            0x80166540 => { ins += 9;
                { let ea = c.r[2].wrapping_add(0xffff9ed0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[31] | c.r[31];
                { let ea = c.r[2].wrapping_add(0xffff9ed4); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.r[4] = c.r[1].wrapping_add(0x20);
                c.f[2] = c.f[1];
                c.r[5] = c.r[1].wrapping_add(0x14);
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[7] = 0x7f;
                c.lr = 0x80166564; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166578(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166564;
            }
            0x80166564 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x3c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801664f0: bad pc {:#x}", pc),
        }
    }
}

/// _soundSetListener3D (0x80166578, 248 bytes)
#[inline(never)]
pub fn f_80166578(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166578) { return; }
        if c.prof { crate::prof::enter(c, 0x80166578); p = true; }
    }
    f_80166578_b(c, 0x80166578);
    if p { crate::prof::exit(c, 0x80166578); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166578_at(c: &mut Cpu, pc: u32) { f_80166578_b(c, pc) }
#[inline(always)]
fn f_80166578_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166578;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166578, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166578 => { ins += 16;
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
                c.lr = 0x801665b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801678e4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801665b8;
            }
            0x801665b8 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if cr_bit(cr, 2) { pc = 0x8016664c; continue; }
                pc = 0x801665c0;
            }
            0x801665c0 => { ins += 12;
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
                c.lr = 0x801665f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015ef04(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801665f0;
            }
            0x801665f0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166644; continue; }
                pc = 0x801665fc;
            }
            0x801665fc => { ins += 3;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[31].wrapping_add(0x10);
                c.lr = 0x80166608; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166608;
            }
            0x80166608 => { ins += 3;
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = c.r[31].wrapping_add(0x1c);
                c.lr = 0x80166614; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166614;
            }
            0x80166614 => { ins += 3;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = c.r[31].wrapping_add(0x28);
                c.lr = 0x80166620; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166620;
            }
            0x80166620 => { ins += 3;
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.r[31].wrapping_add(0x34);
                c.lr = 0x8016662c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800e01d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016662c;
            }
            0x8016662c => { ins += 6;
                { let ea = c.r[31].wrapping_add(0x4); let v = c.f[29]; c.stfs(ea, v); }
                c.r[3] = c.r[31] | c.r[31];
                { let ea = c.r[31].wrapping_add(0x8); let v = c.f[30]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0xc); let v = c.f[31]; c.stfs(ea, v); }
                c.stb(c.r[31].wrapping_add(0x1), c.r[30]);
                pc = 0x80166650; continue;
                pc = 0x80166644;
            }
            0x80166644 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8016664c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801677f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016664c;
            }
            0x8016664c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x80166650;
            }
            0x80166650 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0x38); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x30); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x28); c.f[29] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166578: bad pc {:#x}", pc),
        }
    }
}

/// soundSetVolume (0x80166670, 76 bytes)
#[inline(never)]
pub fn f_80166670(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166670) { return; }
        if c.prof { crate::prof::enter(c, 0x80166670); p = true; }
    }
    f_80166670_b(c, 0x80166670);
    if p { crate::prof::exit(c, 0x80166670); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166670_at(c: &mut Cpu, pc: u32) { f_80166670_b(c, pc) }
#[inline(always)]
fn f_80166670_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166670;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166670, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166670 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[6] = c.lbz_t(ins, c.r[3]);
                c.r[0] = c.r[6].rotate_left(25) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801666a8; continue; }
                pc = 0x80166694;
            }
            0x80166694 => { ins += 3;
                c.r[0] = c.r[6].rotate_left(27) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x801666a8; continue; }
                pc = 0x801666a0;
            }
            0x801666a0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801666ac; continue;
                pc = 0x801666a8;
            }
            0x801666a8 => { ins += 1;
                c.lr = 0x801666ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_8016737c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801666ac;
            }
            0x801666ac => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166670: bad pc {:#x}", pc),
        }
    }
}

/// GSsndGetStatus (0x801666bc, 284 bytes)
#[inline(never)]
pub fn f_801666bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801666bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801666bc); p = true; }
    }
    f_801666bc_b(c, 0x801666bc);
    if p { crate::prof::exit(c, 0x801666bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801666bc_at(c: &mut Cpu, pc: u32) { f_801666bc_b(c, pc) }
#[inline(always)]
fn f_801666bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801666bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801666bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801666bc => { ins += 12;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.r[31].wrapping_mul(0xc);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[4] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[4]);
                c.r[0] = c.r[3].rotate_left(27) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x801666f4; continue; }
                pc = 0x801666ec;
            }
            0x801666ec => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801667c4; continue;
                pc = 0x801666f4;
            }
            0x801666f4 => { ins += 3;
                c.r[4] = c.lwz_t(ins, c.r[4].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80166708; continue; }
                pc = 0x80166700;
            }
            0x80166700 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801667c4; continue;
                pc = 0x80166708;
            }
            0x80166708 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(25) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166764; continue; }
                pc = 0x80166714;
            }
            0x80166714 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x8016673c; continue; }
                pc = 0x80166724;
            }
            0x80166724 => { ins += 1;
                c.lr = 0x80166728; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d880(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166728;
            }
            0x80166728 => { ins += 3;
                c.r[0] = c.r[3].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x8016673c; continue; }
                pc = 0x80166734;
            }
            0x80166734 => { ins += 2;
                c.r[3] = 0x2;
                pc = 0x801667c4; continue;
                pc = 0x8016673c;
            }
            0x8016673c => { ins += 5;
                c.r[3] = 0x80270000;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[3].wrapping_add(0x36f0);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80166750; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166750;
            }
            0x80166750 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.lr = 0x8016675c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167070(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016675c;
            }
            0x8016675c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801667c4; continue;
                pc = 0x80166764;
            }
            0x80166764 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(28) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166778; continue; }
                pc = 0x80166770;
            }
            0x80166770 => { ins += 2;
                c.r[3] = 0x3;
                pc = 0x801667c4; continue;
                pc = 0x80166778;
            }
            0x80166778 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x801667a0; continue; }
                pc = 0x80166788;
            }
            0x80166788 => { ins += 1;
                c.lr = 0x8016678c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d598(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016678c;
            }
            0x8016678c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801667a0; continue; }
                pc = 0x80166798;
            }
            0x80166798 => { ins += 2;
                c.r[3] = 0x2;
                pc = 0x801667c4; continue;
                pc = 0x801667a0;
            }
            0x801667a0 => { ins += 5;
                c.r[3] = 0x80270000;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[3].wrapping_add(0x371c);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x801667b4; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801667b4;
            }
            0x801667b4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.lr = 0x801667c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167070(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801667c0;
            }
            0x801667c0 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801667c4;
            }
            0x801667c4 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801666bc: bad pc {:#x}", pc),
        }
    }
}

/// soundResumeSong (0x801667d8, 260 bytes)
#[inline(never)]
pub fn f_801667d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801667d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801667d8); p = true; }
    }
    f_801667d8_b(c, 0x801667d8);
    if p { crate::prof::exit(c, 0x801667d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801667d8_at(c: &mut Cpu, pc: u32) { f_801667d8_b(c, pc) }
#[inline(always)]
fn f_801667d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801667d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801667d8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801667d8 => { ins += 16;
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
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[31] = c.r[3].wrapping_add(c.r[0]);
                c.r[4] = c.lbz_t(ins, c.r[31]);
                c.r[0] = c.r[4].rotate_left(25) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166820; continue; }
                pc = 0x80166818;
            }
            0x80166818 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801668bc; continue;
                pc = 0x80166820;
            }
            0x80166820 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(27) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80166834; continue; }
                pc = 0x8016682c;
            }
            0x8016682c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801668bc; continue;
                pc = 0x80166834;
            }
            0x80166834 => { ins += 3;
                c.r[30] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80166848; continue; }
                pc = 0x80166840;
            }
            0x80166840 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801668bc; continue;
                pc = 0x80166848;
            }
            0x80166848 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if !cr_bit(cr, 2) { pc = 0x80166860; continue; }
                pc = 0x80166858;
            }
            0x80166858 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801668bc; continue;
                pc = 0x80166860;
            }
            0x80166860 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(28) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80166874; continue; }
                pc = 0x8016686c;
            }
            0x8016686c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801668bc; continue;
                pc = 0x80166874;
            }
            0x80166874 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x2));
                c.lr = 0x8016687c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166c34(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016687c;
            }
            0x8016687c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8016688c; continue; }
                pc = 0x80166884;
            }
            0x80166884 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x801668bc; continue;
                pc = 0x8016688c;
            }
            0x8016688c => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x8));
                c.lr = 0x80166894; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d648(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166894;
            }
            0x80166894 => { ins += 5;
                c.r[3] = c.lbz_t(ins, c.r[30].wrapping_add(0x1));
                c.r[4] = c.r[29].rotate_left(0) & 0xffff;
                c.r[5] = c.lwz_t(ins, c.r[30].wrapping_add(0x8));
                c.r[6] = 0x0;
                c.lr = 0x801668a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d6d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801668a8;
            }
            0x801668a8 => { ins += 5;
                c.r[0] = c.lbz_t(ins, c.r[31]);
                c.r[3] = 0x0;
                c.r[0] = (c.r[3].rotate_left(4) & 0x10) | (c.r[0] & 0xffffffef);
                c.r[3] = 0x1;
                c.stb(c.r[31], c.r[0]);
                pc = 0x801668bc;
            }
            0x801668bc => { ins += 8;
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
            _ => panic!("f_801667d8: bad pc {:#x}", pc),
        }
    }
}

/// soundPauseSong (0x801668dc, 224 bytes)
#[inline(never)]
pub fn f_801668dc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801668dc) { return; }
        if c.prof { crate::prof::enter(c, 0x801668dc); p = true; }
    }
    f_801668dc_b(c, 0x801668dc);
    if p { crate::prof::exit(c, 0x801668dc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801668dc_at(c: &mut Cpu, pc: u32) { f_801668dc_b(c, pc) }
#[inline(always)]
fn f_801668dc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801668dc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801668dc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801668dc => { ins += 12;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[5] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[31] = c.r[5].wrapping_add(c.r[0]);
                c.r[6] = c.lbz_t(ins, c.r[31]);
                c.r[0] = c.r[6].rotate_left(25) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166914; continue; }
                pc = 0x8016690c;
            }
            0x8016690c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801669a4; continue;
                pc = 0x80166914;
            }
            0x80166914 => { ins += 3;
                c.r[0] = c.r[6].rotate_left(27) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80166928; continue; }
                pc = 0x80166920;
            }
            0x80166920 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801669a4; continue;
                pc = 0x80166928;
            }
            0x80166928 => { ins += 3;
                c.r[30] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8016693c; continue; }
                pc = 0x80166934;
            }
            0x80166934 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801669a4; continue;
                pc = 0x8016693c;
            }
            0x8016693c => { ins += 4;
                c.r[5] = c.lwz_t(ins, c.r[30].wrapping_add(0x8));
                c.r[0] = c.r[5].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if !cr_bit(cr, 2) { pc = 0x80166954; continue; }
                pc = 0x8016694c;
            }
            0x8016694c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801669a4; continue;
                pc = 0x80166954;
            }
            0x80166954 => { ins += 3;
                c.r[0] = c.r[6].rotate_left(28) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166968; continue; }
                pc = 0x80166960;
            }
            0x80166960 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801669a4; continue;
                pc = 0x80166968;
            }
            0x80166968 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166978; continue; }
                pc = 0x80166970;
            }
            0x80166970 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x801669a4; continue;
                pc = 0x80166978;
            }
            0x80166978 => { ins += 4;
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80166988; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d6d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166988;
            }
            0x80166988 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x8));
                c.lr = 0x80166990; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d5c8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166990;
            }
            0x80166990 => { ins += 5;
                c.r[0] = c.lbz_t(ins, c.r[31]);
                c.r[3] = 0x1;
                c.r[0] = (c.r[3].rotate_left(4) & 0x10) | (c.r[0] & 0xffffffef);
                c.r[3] = 0x1;
                c.stb(c.r[31], c.r[0]);
                pc = 0x801669a4;
            }
            0x801669a4 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801668dc: bad pc {:#x}", pc),
        }
    }
}

/// soundStopSE (0x801669bc, 40 bytes)
#[inline(never)]
pub fn f_801669bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801669bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801669bc); p = true; }
    }
    f_801669bc_b(c, 0x801669bc);
    if p { crate::prof::exit(c, 0x801669bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801669bc_at(c: &mut Cpu, pc: u32) { f_801669bc_b(c, pc) }
#[inline(always)]
fn f_801669bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801669bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801669bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801669bc => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801669d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801669e4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801669d4;
            }
            0x801669d4 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801669bc: bad pc {:#x}", pc),
        }
    }
}

/// soundStopRaw (0x801669e4, 68 bytes)
#[inline(never)]
pub fn f_801669e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801669e4) { return; }
        if c.prof { crate::prof::enter(c, 0x801669e4); p = true; }
    }
    f_801669e4_b(c, 0x801669e4);
    if p { crate::prof::exit(c, 0x801669e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801669e4_at(c: &mut Cpu, pc: u32) { f_801669e4_b(c, pc) }
#[inline(always)]
fn f_801669e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801669e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801669e4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801669e4 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[3]);
                c.r[0] = c.r[0].rotate_left(25) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166a14; continue; }
                pc = 0x80166a0c;
            }
            0x80166a0c => { ins += 1;
                c.lr = 0x80166a10; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167490(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166a10;
            }
            0x80166a10 => { ins += 1;
                pc = 0x80166a18; continue;
                pc = 0x80166a14;
            }
            0x80166a14 => { ins += 1;
                c.lr = 0x80166a18; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167508(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166a18;
            }
            0x80166a18 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801669e4: bad pc {:#x}", pc),
        }
    }
}

/// soundStartSE (0x80166a28, 40 bytes)
#[inline(never)]
pub fn f_80166a28(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166a28) { return; }
        if c.prof { crate::prof::enter(c, 0x80166a28); p = true; }
    }
    f_80166a28_b(c, 0x80166a28);
    if p { crate::prof::exit(c, 0x80166a28); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166a28_at(c: &mut Cpu, pc: u32) { f_80166a28_b(c, pc) }
#[inline(always)]
fn f_80166a28_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166a28;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166a28, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166a28 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80166a40; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166ab8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166a40;
            }
            0x80166a40 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166a28: bad pc {:#x}", pc),
        }
    }
}

/// soundStart (0x80166a50, 104 bytes)
#[inline(never)]
pub fn f_80166a50(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166a50) { return; }
        if c.prof { crate::prof::enter(c, 0x80166a50); p = true; }
    }
    f_80166a50_b(c, 0x80166a50);
    if p { crate::prof::exit(c, 0x80166a50); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166a50_at(c: &mut Cpu, pc: u32) { f_80166a50_b(c, pc) }
#[inline(always)]
fn f_80166a50_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166a50;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166a50, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166a50 => { ins += 11;
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
                c.lr = 0x80166a7c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167408(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166a7c;
            }
            0x80166a7c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80166a8c; continue; }
                pc = 0x80166a84;
            }
            0x80166a84 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80166a9c; continue;
                pc = 0x80166a8c;
            }
            0x80166a8c => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80166a9c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166ab8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166a9c;
            }
            0x80166a9c => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166a50: bad pc {:#x}", pc),
        }
    }
}

/// soundStartRaw (0x80166ab8, 96 bytes)
#[inline(never)]
pub fn f_80166ab8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166ab8) { return; }
        if c.prof { crate::prof::enter(c, 0x80166ab8); p = true; }
    }
    f_80166ab8_b(c, 0x80166ab8);
    if p { crate::prof::exit(c, 0x80166ab8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166ab8_at(c: &mut Cpu, pc: u32) { f_80166ab8_b(c, pc) }
#[inline(always)]
fn f_80166ab8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166ab8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166ab8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166ab8 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[7].wrapping_mul(0xc);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[6] = c.lbz_t(ins, c.r[3]);
                c.r[0] = c.r[6].rotate_left(25) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166af0; continue; }
                pc = 0x80166ae4;
            }
            0x80166ae4 => { ins += 2;
                c.r[4] = c.r[7] | c.r[7];
                c.lr = 0x80166aec; { c.steps = st; c.instrs = ins; c.cr = cr; f_8016758c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166aec;
            }
            0x80166aec => { ins += 1;
                pc = 0x80166b08; continue;
                pc = 0x80166af0;
            }
            0x80166af0 => { ins += 3;
                c.r[0] = c.r[6].rotate_left(27) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80166b04; continue; }
                pc = 0x80166afc;
            }
            0x80166afc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80166b08; continue;
                pc = 0x80166b04;
            }
            0x80166b04 => { ins += 1;
                c.lr = 0x80166b08; { c.steps = st; c.instrs = ins; c.cr = cr; f_8016761c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166b08;
            }
            0x80166b08 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166ab8: bad pc {:#x}", pc),
        }
    }
}

/// soundKill (0x80166b18, 36 bytes)
#[inline(never)]
pub fn f_80166b18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166b18) { return; }
        if c.prof { crate::prof::enter(c, 0x80166b18); p = true; }
    }
    f_80166b18_b(c, 0x80166b18);
    if p { crate::prof::exit(c, 0x80166b18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166b18_at(c: &mut Cpu, pc: u32) { f_80166b18_b(c, pc) }
#[inline(always)]
fn f_80166b18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166b18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166b18, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166b18 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80166b2c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167070(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166b2c;
            }
            0x80166b2c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166b18: bad pc {:#x}", pc),
        }
    }
}

/// soundAttachWork (0x80166b3c, 164 bytes)
#[inline(never)]
pub fn f_80166b3c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166b3c) { return; }
        if c.prof { crate::prof::enter(c, 0x80166b3c); p = true; }
    }
    f_80166b3c_b(c, 0x80166b3c);
    if p { crate::prof::exit(c, 0x80166b3c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166b3c_at(c: &mut Cpu, pc: u32) { f_80166b3c_b(c, pc) }
#[inline(always)]
fn f_80166b3c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166b3c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166b3c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166b3c => { ins += 16;
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
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[31] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[31]);
                c.r[0] = c.r[0].rotate_left(27) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166b84; continue; }
                pc = 0x80166b7c;
            }
            0x80166b7c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80166bc0; continue;
                pc = 0x80166b84;
            }
            0x80166b84 => { ins += 1;
                c.lr = 0x80166b88; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167964(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166b88;
            }
            0x80166b88 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if !cr_bit(cr, 2) { pc = 0x80166ba8; continue; }
                pc = 0x80166b90;
            }
            0x80166b90 => { ins += 1;
                c.lr = 0x80166b94; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167318(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166b94;
            }
            0x80166b94 => { ins += 1;
                c.lr = 0x80166b98; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167964(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166b98;
            }
            0x80166b98 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if !cr_bit(cr, 2) { pc = 0x80166ba8; continue; }
                pc = 0x80166ba0;
            }
            0x80166ba0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80166bc0; continue;
                pc = 0x80166ba8;
            }
            0x80166ba8 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80166bb4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167af0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166bb4;
            }
            0x80166bb4 => { ins += 3;
                c.stw(c.r[30].wrapping_add(0xc), c.r[28]);
                c.r[3] = 0x1;
                c.stw(c.r[30].wrapping_add(0x10), c.r[29]);
                pc = 0x80166bc0;
            }
            0x80166bc0 => { ins += 8;
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
            _ => panic!("f_80166b3c: bad pc {:#x}", pc),
        }
    }
}

/// GSsndOpenWaveDVD (0x80166be0, 84 bytes)
#[inline(never)]
pub fn f_80166be0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166be0) { return; }
        if c.prof { crate::prof::enter(c, 0x80166be0); p = true; }
    }
    f_80166be0_b(c, 0x80166be0);
    if p { crate::prof::exit(c, 0x80166be0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166be0_at(c: &mut Cpu, pc: u32) { f_80166be0_b(c, pc) }
#[inline(always)]
fn f_80166be0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166be0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166be0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166be0 => { ins += 16;
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
                c.lr = 0x80166c20; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167118(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166c20;
            }
            0x80166c20 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166be0: bad pc {:#x}", pc),
        }
    }
}

/// soundApplyReverb (0x80166c34, 64 bytes)
#[inline(never)]
pub fn f_80166c34(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166c34) { return; }
        if c.prof { crate::prof::enter(c, 0x80166c34); p = true; }
    }
    f_80166c34_b(c, 0x80166c34);
    if p { crate::prof::exit(c, 0x80166c34); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166c34_at(c: &mut Cpu, pc: u32) { f_80166c34_b(c, pc) }
#[inline(always)]
fn f_80166c34_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166c34;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166c34, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166c34 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80166c64; continue; }
                pc = 0x80166c48;
            }
            0x80166c48 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa8c4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[3]);
                if cr_bit(cr, 2) { pc = 0x80166c64; continue; }
                pc = 0x80166c54;
            }
            0x80166c54 => { ins += 1;
                c.lr = 0x80166c58; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167a9c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166c58;
            }
            0x80166c58 => { ins += 3;
                c.r[3] = 0x80450000;
                c.r[3] = c.r[3].wrapping_add(0x2500);
                c.lr = 0x80166c64; { c.steps = st; c.instrs = ins; c.cr = cr; f_80164488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166c64;
            }
            0x80166c64 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166c34: bad pc {:#x}", pc),
        }
    }
}

/// GSsndGetOutputMode (0x80166c74, 76 bytes)
#[inline(never)]
pub fn f_80166c74(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166c74) { return; }
        if c.prof { crate::prof::enter(c, 0x80166c74); p = true; }
    }
    f_80166c74_b(c, 0x80166c74);
    if p { crate::prof::exit(c, 0x80166c74); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166c74_at(c: &mut Cpu, pc: u32) { f_80166c74_b(c, pc) }
#[inline(always)]
fn f_80166c74_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166c74;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166c74, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166c74 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80166c84; { c.steps = st; c.instrs = ins; c.cr = cr; f_800a0e34(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166c84;
            }
            0x80166c84 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1);
                if cr_bit(cr, 2) { pc = 0x80166ca4; continue; }
                pc = 0x80166c8c;
            }
            0x80166c8c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80166cac; continue; }
                pc = 0x80166c90;
            }
            0x80166c90 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80166c9c; continue; }
                pc = 0x80166c98;
            }
            0x80166c98 => { ins += 1;
                pc = 0x80166cac; continue;
                pc = 0x80166c9c;
            }
            0x80166c9c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80166cb0; continue;
                pc = 0x80166ca4;
            }
            0x80166ca4 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80166cb0; continue;
                pc = 0x80166cac;
            }
            0x80166cac => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x80166cb0;
            }
            0x80166cb0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166c74: bad pc {:#x}", pc),
        }
    }
}

/// soundSetOutputMode (0x80166cc0, 88 bytes)
#[inline(never)]
pub fn f_80166cc0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166cc0) { return; }
        if c.prof { crate::prof::enter(c, 0x80166cc0); p = true; }
    }
    f_80166cc0_b(c, 0x80166cc0);
    if p { crate::prof::exit(c, 0x80166cc0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166cc0_at(c: &mut Cpu, pc: u32) { f_80166cc0_b(c, pc) }
#[inline(always)]
fn f_80166cc0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166cc0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166cc0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166cc0 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80166cd8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d9bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166cd8;
            }
            0x80166cd8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x80166cf0; continue; }
                pc = 0x80166ce0;
            }
            0x80166ce0 => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x80166d04; continue; }
                pc = 0x80166ce4;
            }
            0x80166ce4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x3);
                if !cr_bit(cr, 0) { pc = 0x80166d04; continue; }
                pc = 0x80166cec;
            }
            0x80166cec => { ins += 1;
                pc = 0x80166cfc; continue;
                pc = 0x80166cf0;
            }
            0x80166cf0 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80166cf8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800a0eb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166cf8;
            }
            0x80166cf8 => { ins += 1;
                pc = 0x80166d04; continue;
                pc = 0x80166cfc;
            }
            0x80166cfc => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x80166d04; { c.steps = st; c.instrs = ins; c.cr = cr; f_800a0eb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166d04;
            }
            0x80166d04 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166cc0: bad pc {:#x}", pc),
        }
    }
}

/// soundSetMasterVolume (0x80166d18, 48 bytes)
#[inline(never)]
pub fn f_80166d18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166d18) { return; }
        if c.prof { crate::prof::enter(c, 0x80166d18); p = true; }
    }
    f_80166d18_b(c, 0x80166d18);
    if p { crate::prof::exit(c, 0x80166d18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166d18_at(c: &mut Cpu, pc: u32) { f_80166d18_b(c, pc) }
#[inline(always)]
fn f_80166d18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166d18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166d18, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166d18 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = c.r[5].rotate_left(0) & 0xff;
                c.r[6] = c.r[6].rotate_left(0) & 0xff;
                c.lr = 0x80166d38; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d928(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166d38;
            }
            0x80166d38 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166d18: bad pc {:#x}", pc),
        }
    }
}

/// soundMasterVolume (0x80166d48, 252 bytes)
#[inline(never)]
pub fn f_80166d48(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166d48) { return; }
        if c.prof { crate::prof::enter(c, 0x80166d48); p = true; }
    }
    f_80166d48_b(c, 0x80166d48);
    if p { crate::prof::exit(c, 0x80166d48); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166d48_at(c: &mut Cpu, pc: u32) { f_80166d48_b(c, pc) }
#[inline(always)]
fn f_80166d48_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166d48;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166d48, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166d48 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x1);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                if !cr_bit(cr, 2) { pc = 0x80166d78; continue; }
                pc = 0x80166d68;
            }
            0x80166d68 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166d78; continue; }
                pc = 0x80166d70;
            }
            0x80166d70 => { ins += 2;
                c.r[30] = 0xff;
                pc = 0x80166dac; continue;
                pc = 0x80166d78;
            }
            0x80166d78 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166d90; continue; }
                pc = 0x80166d80;
            }
            0x80166d80 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x1);
                if cr_bit(cr, 2) { pc = 0x80166d90; continue; }
                pc = 0x80166d88;
            }
            0x80166d88 => { ins += 2;
                c.r[30] = 0xfd;
                pc = 0x80166dac; continue;
                pc = 0x80166d90;
            }
            0x80166d90 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x1);
                if cr_bit(cr, 2) { pc = 0x80166e30; continue; }
                pc = 0x80166d98;
            }
            0x80166d98 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166e30; continue; }
                pc = 0x80166da0;
            }
            0x80166da0 => { ins += 2;
                c.r[30] = 0xfe;
                pc = 0x80166dac; continue;
                pc = 0x80166da8;
            }
            0x80166da8 => { ins += 1;
                pc = 0x80166e30; continue;
                pc = 0x80166dac;
            }
            0x80166dac => { ins += 3;
                c.r[29] = 0x0;
                c.r[31] = 0x0;
                pc = 0x80166e10; continue;
                pc = 0x80166db8;
            }
            0x80166db8 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[4] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[31]));
                c.r[0] = c.r[4].rotate_left(27) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166e08; continue; }
                pc = 0x80166dcc;
            }
            0x80166dcc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xfe);
                if cr_bit(cr, 2) { pc = 0x80166df8; continue; }
                pc = 0x80166dd4;
            }
            0x80166dd4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80166e00; continue; }
                pc = 0x80166dd8;
            }
            0x80166dd8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xfd);
                if !cr_bit(cr, 0) { pc = 0x80166de8; continue; }
                pc = 0x80166de0;
            }
            0x80166de0 => { ins += 1;
                pc = 0x80166e00; continue;
                pc = 0x80166de4;
            }
            0x80166de4 => { ins += 1;
                pc = 0x80166e00; continue;
                pc = 0x80166de8;
            }
            0x80166de8 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(25) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80166e08; continue; }
                pc = 0x80166df4;
            }
            0x80166df4 => { ins += 1;
                pc = 0x80166e00; continue;
                pc = 0x80166df8;
            }
            0x80166df8 => { ins += 2;
                c.r[0] = c.r[4].rotate_left(25) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80166e08; continue; }
                pc = 0x80166e00;
            }
            0x80166e00 => { ins += 2;
                c.r[4] = c.r[27].rotate_left(0) & 0xff;
                c.lr = 0x80166e08; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167408(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166e08;
            }
            0x80166e08 => { ins += 2;
                c.r[31] = c.r[31].wrapping_add(0xc);
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x80166e10;
            }
            0x80166e10 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa8c8));
                c.r[3] = c.r[29] | c.r[29];
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80166db8; continue; }
                pc = 0x80166e20;
            }
            0x80166e20 => { ins += 4;
                c.r[3] = c.r[27].rotate_left(0) & 0xff;
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = c.r[30].rotate_left(0) & 0xff;
                c.lr = 0x80166e30; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d8c8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166e30;
            }
            0x80166e30 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166d48: bad pc {:#x}", pc),
        }
    }
}

/// soundShutdown (0x80166e44, 68 bytes)
#[inline(never)]
pub fn f_80166e44(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166e44) { return; }
        if c.prof { crate::prof::enter(c, 0x80166e44); p = true; }
    }
    f_80166e44_b(c, 0x80166e44);
    if p { crate::prof::exit(c, 0x80166e44); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166e44_at(c: &mut Cpu, pc: u32) { f_80166e44_b(c, pc) }
#[inline(always)]
fn f_80166e44_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166e44;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166e44, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166e44 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80166e54; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015ffd4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166e54;
            }
            0x80166e54 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80166e78; continue; }
                pc = 0x80166e60;
            }
            0x80166e60 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x80166e74; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166d48(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166e74;
            }
            0x80166e74 => { ins += 1;
                c.lr = 0x80166e78; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015ffa0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166e78;
            }
            0x80166e78 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166e44: bad pc {:#x}", pc),
        }
    }
}

/// soundInit (0x80166e88, 440 bytes)
#[inline(never)]
pub fn f_80166e88(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80166e88) { return; }
        if c.prof { crate::prof::enter(c, 0x80166e88); p = true; }
    }
    f_80166e88_b(c, 0x80166e88);
    if p { crate::prof::exit(c, 0x80166e88); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80166e88_at(c: &mut Cpu, pc: u32) { f_80166e88_b(c, pc) }
#[inline(never)]
fn f_80166e88_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80166e88;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80166e88, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80166e88 => { ins += 22;
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
                c.r[8] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff9eb8));
                c.r[9] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff9ebc));
                c.stw(c.r[1].wrapping_add(0x10), c.r[8]);
                c.r[8] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8788));
                c.stw(c.r[1].wrapping_add(0x14), c.r[9]);
                c.r[4] = c.lwz_t(ins, c.r[8]);
                c.stw(c.r[13].wrapping_add(0xffffa8c8), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa8c4), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa8c0), c.r[0]);
                c.lr = 0x80166ee0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167bb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166ee0;
            }
            0x80166ee0 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa8bc), c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x80166ef4; continue; }
                pc = 0x80166eec;
            }
            0x80166eec => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167024; continue;
                pc = 0x80166ef4;
            }
            0x80166ef4 => { ins += 1;
                c.lr = 0x80166ef8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167a6c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166ef8;
            }
            0x80166ef8 => { ins += 3;
                c.stw(c.r[13].wrapping_add(0xffffa8b8), c.r[29]);
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80166f04; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167bb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166f04;
            }
            0x80166f04 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa8b4), c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x80166f18; continue; }
                pc = 0x80166f10;
            }
            0x80166f10 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167024; continue;
                pc = 0x80166f18;
            }
            0x80166f18 => { ins += 1;
                c.lr = 0x80166f1c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167a44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166f1c;
            }
            0x80166f1c => { ins += 3;
                c.r[3] = c.r[30].wrapping_mul(0xd0);
                c.stw(c.r[13].wrapping_add(0xffffa8b0), c.r[30]);
                c.lr = 0x80166f28; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167bb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166f28;
            }
            0x80166f28 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa8ac), c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x80166f3c; continue; }
                pc = 0x80166f34;
            }
            0x80166f34 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167024; continue;
                pc = 0x80166f3c;
            }
            0x80166f3c => { ins += 1;
                c.lr = 0x80166f40; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167a14(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166f40;
            }
            0x80166f40 => { ins += 3;
                c.r[3] = c.r[31].wrapping_mul(0x78);
                c.stw(c.r[13].wrapping_add(0xffffa8a8), c.r[31]);
                c.lr = 0x80166f4c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167bb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166f4c;
            }
            0x80166f4c => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa8a4), c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x80166f60; continue; }
                pc = 0x80166f58;
            }
            0x80166f58 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167024; continue;
                pc = 0x80166f60;
            }
            0x80166f60 => { ins += 1;
                c.lr = 0x80166f64; { c.steps = st; c.instrs = ins; c.cr = cr; f_801679e4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166f64;
            }
            0x80166f64 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x80166f6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801631ac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166f6c;
            }
            0x80166f6c => { ins += 8;
                c.r[4] = 0xa00000;
                c.r[3] = 0x40;
                c.r[8] = c.r[4].wrapping_add(0xffffc000);
                c.r[4] = 0x30;
                c.r[5] = 0x10;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x80166f8c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8015fe88(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166f8c;
            }
            0x80166f8c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80166f9c; continue; }
                pc = 0x80166f94;
            }
            0x80166f94 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167024; continue;
                pc = 0x80166f9c;
            }
            0x80166f9c => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80166fa4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167a9c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166fa4;
            }
            0x80166fa4 => { ins += 3;
                c.r[3] = 0x80450000;
                c.r[3] = c.r[3].wrapping_add(0x2500);
                c.lr = 0x80166fb0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801644e0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166fb0;
            }
            0x80166fb0 => { ins += 13;
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
                c.lr = 0x80166fe4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014daa8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166fe4;
            }
            0x80166fe4 => { ins += 5;
                c.r[3] = 0x7f;
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80166ff8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166d48(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80166ff8;
            }
            0x80166ff8 => { ins += 5;
                c.r[3] = 0x64;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x8016700c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166d48(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016700c;
            }
            0x8016700c => { ins += 1;
                c.lr = 0x80167010; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166c74(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167010;
            }
            0x80167010 => { ins += 1;
                c.lr = 0x80167014; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166cc0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167014;
            }
            0x80167014 => { ins += 3;
                c.r[3] = 0x80160000;
                c.r[3] = c.r[3].wrapping_add(0x7040);
                c.lr = 0x80167020; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d8c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167020;
            }
            0x80167020 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80167024;
            }
            0x80167024 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x28));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80166e88: bad pc {:#x}", pc),
        }
    }
}

/// _soundReleaseFinishedVoice (0x80167040, 48 bytes)
#[inline(never)]
pub fn f_80167040(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80167040) { return; }
        if c.prof { crate::prof::enter(c, 0x80167040); p = true; }
    }
    f_80167040_b(c, 0x80167040);
    if p { crate::prof::exit(c, 0x80167040); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80167040_at(c: &mut Cpu, pc: u32) { f_80167040_b(c, pc) }
#[inline(always)]
fn f_80167040_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80167040;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80167040, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80167040 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80167050; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167720(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167050;
            }
            0x80167050 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80167060; continue; }
                pc = 0x80167058;
            }
            0x80167058 => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x80167060; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167070(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167060;
            }
            0x80167060 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80167040: bad pc {:#x}", pc),
        }
    }
}

/// _soundSlotRelease (0x80167070, 168 bytes)
#[inline(never)]
pub fn f_80167070(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80167070) { return; }
        if c.prof { crate::prof::enter(c, 0x80167070); p = true; }
    }
    f_80167070_b(c, 0x80167070);
    if p { crate::prof::exit(c, 0x80167070); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80167070_at(c: &mut Cpu, pc: u32) { f_80167070_b(c, pc) }
#[inline(always)]
fn f_80167070_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80167070;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80167070, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80167070 => { ins += 12;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[5] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[31] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[31]);
                c.r[0] = c.r[0].rotate_left(27) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x801670a8; continue; }
                pc = 0x801670a0;
            }
            0x801670a0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167100; continue;
                pc = 0x801670a8;
            }
            0x801670a8 => { ins += 3;
                c.r[30] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801670bc; continue; }
                pc = 0x801670b4;
            }
            0x801670b4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167100; continue;
                pc = 0x801670bc;
            }
            0x801670bc => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801670d4; continue; }
                pc = 0x801670c8;
            }
            0x801670c8 => { ins += 3;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x801670d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801669e4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801670d4;
            }
            0x801670d4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x801670e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167af0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801670e0;
            }
            0x801670e0 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801670e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8016782c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801670e8;
            }
            0x801670e8 => { ins += 6;
                c.r[4] = 0x0;
                c.r[3] = 0x1;
                c.stw(c.r[31].wrapping_add(0x8), c.r[4]);
                c.r[0] = c.lbz_t(ins, c.r[31]);
                c.r[0] = (c.r[4].rotate_left(5) & 0x20) | (c.r[0] & 0xffffffdf);
                c.stb(c.r[31], c.r[0]);
                pc = 0x80167100;
            }
            0x80167100 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80167070: bad pc {:#x}", pc),
        }
    }
}

/// soundPushWave (0x80167118, 384 bytes)
#[inline(never)]
pub fn f_80167118(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80167118) { return; }
        if c.prof { crate::prof::enter(c, 0x80167118); p = true; }
    }
    f_80167118_b(c, 0x80167118);
    if p { crate::prof::exit(c, 0x80167118); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80167118_at(c: &mut Cpu, pc: u32) { f_80167118_b(c, pc) }
#[inline(never)]
fn f_80167118_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80167118;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80167118, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80167118 => { ins += 16;
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
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8794));
                c.r[28] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[28].wrapping_add(0x1));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80167160; continue; }
                pc = 0x80167158;
            }
            0x80167158 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x80167160;
            }
            0x80167160 => { ins += 3;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[9] | c.r[9];
                c.lr = 0x8016716c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800f9318(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x8016716c;
            }
            0x8016716c => { ins += 2;
                c.r[27] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[27]);
                if !cr_bit(cr, 2) { pc = 0x8016717c; continue; }
                pc = 0x80167174;
            }
            0x80167174 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x8016717c;
            }
            0x8016717c => { ins += 3;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[24] | c.r[24];
                c.lr = 0x80167188; { c.steps = st; c.instrs = ins; c.cr = cr; f_800f9318(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167188;
            }
            0x80167188 => { ins += 2;
                c.r[25] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[25]);
                if !cr_bit(cr, 2) { pc = 0x80167198; continue; }
                pc = 0x80167190;
            }
            0x80167190 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x80167198;
            }
            0x80167198 => { ins += 3;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x801671a4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800f9318(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801671a4;
            }
            0x801671a4 => { ins += 2;
                c.r[24] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[24]);
                if !cr_bit(cr, 2) { pc = 0x801671b4; continue; }
                pc = 0x801671ac;
            }
            0x801671ac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x801671b4;
            }
            0x801671b4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80167218; continue; }
                pc = 0x801671bc;
            }
            0x801671bc => { ins += 2;
                c.r[26] = 0x0;
                c.lr = 0x801671c4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae78c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801671c4;
            }
            0x801671c4 => { ins += 4;
                c.r[5] = 0x80160000;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[5].wrapping_add(0x7298);
                c.lr = 0x801671d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80159ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801671d4;
            }
            0x801671d4 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801671dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167f28(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801671dc;
            }
            0x801671dc => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa89c), c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x801671f0; continue; }
                pc = 0x801671e8;
            }
            0x801671e8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x801671f0;
            }
            0x801671f0 => { ins += 2;
                c.stw(c.r[13].wrapping_add(0xffffa898), c.r[31]);
                c.lr = 0x801671f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ae78c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801671f8;
            }
            0x801671f8 => { ins += 1;
                c.lr = 0x801671fc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167bb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801671fc;
            }
            0x801671fc => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa8a0), c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x80167228; continue; }
                pc = 0x80167208;
            }
            0x80167208 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa89c));
                c.lr = 0x80167210; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167e64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167210;
            }
            0x80167210 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x80167218;
            }
            0x80167218 => { ins += 4;
                c.r[26] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80167228; { c.steps = st; c.instrs = ins; c.cr = cr; f_80159ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167228;
            }
            0x80167228 => { ins += 6;
                c.r[4] = c.lhz_t(ins, c.r[28].wrapping_add(0x2));
                c.r[3] = c.r[27] | c.r[27];
                c.r[5] = c.r[26] | c.r[26];
                c.r[6] = c.r[25] | c.r[25];
                c.r[7] = c.r[24] | c.r[24];
                c.lr = 0x80167240; { c.steps = st; c.instrs = ins; c.cr = cr; f_80159ef0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167240;
            }
            0x80167240 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80167260; continue; }
                pc = 0x80167248;
            }
            0x80167248 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80167258; continue; }
                pc = 0x80167250;
            }
            0x80167250 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa89c));
                c.lr = 0x80167258; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167e64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167258;
            }
            0x80167258 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167284; continue;
                pc = 0x80167260;
            }
            0x80167260 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80167278; continue; }
                pc = 0x80167268;
            }
            0x80167268 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa8a0));
                c.lr = 0x80167270; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167b70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167270;
            }
            0x80167270 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa89c));
                c.lr = 0x80167278; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167e64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167278;
            }
            0x80167278 => { ins += 3;
                c.r[0] = 0x1;
                c.r[3] = 0x1;
                c.stb(c.r[28].wrapping_add(0x1), c.r[0]);
                pc = 0x80167284;
            }
            0x80167284 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80167118: bad pc {:#x}", pc),
        }
    }
}

/// _sndSetSampleDataUploadCallbackFunction (0x80167298, 128 bytes)
#[inline(never)]
pub fn f_80167298(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80167298) { return; }
        if c.prof { crate::prof::enter(c, 0x80167298); p = true; }
    }
    f_80167298_b(c, 0x80167298);
    if p { crate::prof::exit(c, 0x80167298); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80167298_at(c: &mut Cpu, pc: u32) { f_80167298_b(c, pc) }
#[inline(always)]
fn f_80167298_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80167298;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80167298, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80167298 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x801672b4;
            }
            0x801672b4 => { ins += 1;
                c.lr = 0x801672b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac954(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801672b8;
            }
            0x801672b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801672b4; continue; }
                pc = 0x801672c0;
            }
            0x801672c0 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa898));
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa89c));
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa8a0));
                c.r[6] = c.r[30].wrapping_add(c.r[0]);
                c.lr = 0x801672d8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80167ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801672d8;
            }
            0x801672d8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xfffffffe);
                if cr_bit(cr, 2) { pc = 0x801672fc; continue; }
                pc = 0x801672e0;
            }
            0x801672e0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801672f0; continue; }
                pc = 0x801672e4;
            }
            0x801672e4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xfffffffd);
                if !cr_bit(cr, 0) { pc = 0x801672c0; continue; }
                pc = 0x801672ec;
            }
            0x801672ec => { ins += 1;
                pc = 0x801672fc; continue;
                pc = 0x801672f0;
            }
            0x801672f0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x801672fc; continue; }
                pc = 0x801672f8;
            }
            0x801672f8 => { ins += 1;
                pc = 0x801672c0; continue;
                pc = 0x801672fc;
            }
            0x801672fc => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa8a0));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80167298: bad pc {:#x}", pc),
        }
    }
}

/// _sndCheckSndWorkALL (0x80167318, 100 bytes)
#[inline(never)]
pub fn f_80167318(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80167318) { return; }
        if c.prof { crate::prof::enter(c, 0x80167318); p = true; }
    }
    f_80167318_b(c, 0x80167318);
    if p { crate::prof::exit(c, 0x80167318); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80167318_at(c: &mut Cpu, pc: u32) { f_80167318_b(c, pc) }
#[inline(always)]
fn f_80167318_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80167318;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80167318, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80167318 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x3748);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80167338; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167338;
            }
            0x80167338 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80167348; continue;
                pc = 0x80167340;
            }
            0x80167340 => { ins += 1;
                c.lr = 0x80167344; { c.steps = st; c.instrs = ins; c.cr = cr; f_801666bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167344;
            }
            0x80167344 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80167348;
            }
            0x80167348 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa8c8));
                c.r[3] = c.r[31] | c.r[31];
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80167340; continue; }
                pc = 0x80167358;
            }
            0x80167358 => { ins += 4;
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x3764);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80167368; { c.steps = st; c.instrs = ins; c.cr = cr; stubs_gen::f_800dd970(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167368;
            }
            0x80167368 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80167318: bad pc {:#x}", pc),
        }
    }
}

/// _soundSlotSetVolume (0x8016737c, 140 bytes)
#[inline(never)]
pub fn f_8016737c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8016737c) { return; }
        if c.prof { crate::prof::enter(c, 0x8016737c); p = true; }
    }
    f_8016737c_b(c, 0x8016737c);
    if p { crate::prof::exit(c, 0x8016737c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8016737c_at(c: &mut Cpu, pc: u32) { f_8016737c_b(c, pc) }
#[inline(always)]
fn f_8016737c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x8016737c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x8016737c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x8016737c => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8016739c; continue; }
                pc = 0x80167394;
            }
            0x80167394 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801673f8; continue;
                pc = 0x8016739c;
            }
            0x8016739c => { ins += 6;
                c.r[0] = c.r[5].rotate_left(0) & 0x7f;
                c.stb(c.r[7].wrapping_add(0x1), c.r[0]);
                c.r[6] = c.lwz_t(ins, c.r[7].wrapping_add(0x8));
                c.r[0] = c.r[6].wrapping_add(0x10000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xffff);
                if cr_bit(cr, 2) { pc = 0x801673f4; continue; }
                pc = 0x801673b4;
            }
            0x801673b4 => { ins += 4;
                c.r[0] = c.lbz_t(ins, c.r[3]);
                c.r[0] = c.r[0].rotate_left(25) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801673d8; continue; }
                pc = 0x801673c4;
            }
            0x801673c4 => { ins += 4;
                c.r[5] = c.lbz_t(ins, c.r[7].wrapping_add(0x1));
                c.r[3] = c.r[6] | c.r[6];
                c.r[4] = 0x7;
                c.lr = 0x801673d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d75c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801673d4;
            }
            0x801673d4 => { ins += 1;
                pc = 0x801673ec; continue;
                pc = 0x801673d8;
            }
            0x801673d8 => { ins += 5;
                c.r[3] = c.lbz_t(ins, c.r[7].wrapping_add(0x1));
                c.r[5] = c.r[6] | c.r[6];
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                c.r[6] = 0x0;
                c.lr = 0x801673ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_8014d6d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801673ec;
            }
            0x801673ec => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x801673f8; continue;
                pc = 0x801673f4;
            }
            0x801673f4 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801673f8;
            }
            0x801673f8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8016737c: bad pc {:#x}", pc),
        }
    }
}

/// _sndSetVolumeWork (0x80167408, 136 bytes)
#[inline(never)]
pub fn f_80167408(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80167408) { return; }
        if c.prof { crate::prof::enter(c, 0x80167408); p = true; }
    }
    f_80167408_b(c, 0x80167408);
    if p { crate::prof::exit(c, 0x80167408); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80167408_at(c: &mut Cpu, pc: u32) { f_80167408_b(c, pc) }
#[inline(always)]
fn f_80167408_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x80167408;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x80167408, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x80167408 => { ins += 13;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.r[5] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff878c));
                c.r[31] = c.r[5].wrapping_add(c.r[0]);
                c.r[4] = c.lbz_t(ins, c.r[31]);
                c.r[0] = c.r[4].rotate_left(27) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80167460; continue; }
                pc = 0x8016743c;
            }
            0x8016743c => { ins += 3;
                c.r[0] = c.r[4].rotate_left(25) & 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80167458; continue; }
                pc = 0x80167448;
            }
            0x80167448 => { ins += 3;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x80167454; { c.steps = st; c.instrs = ins; c.cr = cr; f_80166b3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x80167454;
            }
            0x80167454 => { ins += 1;
                pc = 0x80167460; continue;
                pc = 0x80167458;
            }
            0x80167458 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80167478; continue;
                pc = 0x80167460;
            }
            0x80167460 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80167474; continue; }
                pc = 0x8016746c;
            }
            0x8016746c => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0x7f;
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                pc = 0x80167474;
            }
            0x80167474 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80167478;
            }
            0x80167478 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80167408: bad pc {:#x}", pc),
        }
    }
}

