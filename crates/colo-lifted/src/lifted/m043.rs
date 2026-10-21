#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// fightMenuOpenMsg (0x802624cc, 60 bytes)
#[inline(never)]
pub fn f_802624cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802624cc) { return; }
        if c.prof { crate::prof::enter(c, 0x802624cc); f_802624cc_at(c, 0x802624cc); crate::prof::exit(c, 0x802624cc); return; }
    }
    f_802624cc_b(c, 0x802624cc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802624cc_at(c: &mut Cpu, pc: u32) { f_802624cc_b(c, pc) }
#[inline(always)]
fn f_802624cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802624cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802624cc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802624cc => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if cr_bit(cr, 2) { pc = 0x802624f4; continue; }
                pc = 0x802624e0;
            }
            0x802624e0 => { ins += 3;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x802624ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80106394(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802624ec;
            }
            0x802624ec => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x802624f8; continue;
                pc = 0x802624f4;
            }
            0x802624f4 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x802624f8;
            }
            0x802624f8 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802624cc: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuFightTrainerAgbHeroOpenMenu (0x80262508, 2092 bytes)
#[inline(never)]
pub fn f_80262508(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80262508) { return; }
        if c.prof { crate::prof::enter(c, 0x80262508); f_80262508_at(c, 0x80262508); crate::prof::exit(c, 0x80262508); return; }
    }
    f_80262508_b(c, 0x80262508);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80262508_at(c: &mut Cpu, pc: u32) { f_80262508_b(c, pc) }
#[inline(never)]
fn f_80262508_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80262508;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80262508, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80262508 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x4b;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x64), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 14..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[16] = c.r[4] | c.r[4];
                c.r[15] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.lr = 0x80262530; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262530;
            }
            0x80262530 => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x80262540; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262540;
            }
            0x80262540 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8026262c; continue; }
                pc = 0x80262548;
            }
            0x80262548 => { ins += 5;
                c.r[4] = c.r[15] | c.r[15];
                c.r[5] = c.r[16] | c.r[16];
                c.r[14] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x8026255c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026255c;
            }
            0x8026255c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8026256c; continue; }
                pc = 0x80262564;
            }
            0x80262564 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802625c8; continue;
                pc = 0x8026256c;
            }
            0x8026256c => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x8026257c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026257c;
            }
            0x8026257c => { ins += 4;
                c.r[17] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[15] | c.r[15];
                c.r[4] = c.r[16] | c.r[16];
                c.lr = 0x8026258c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026258c;
            }
            0x8026258c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8026259c; continue; }
                pc = 0x80262594;
            }
            0x80262594 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802625c8; continue;
                pc = 0x8026259c;
            }
            0x8026259c => { ins += 1;
                c.lr = 0x802625a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802625a0;
            }
            0x802625a0 => { ins += 1;
                c.lr = 0x802625a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802625a4;
            }
            0x802625a4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x802625b4; continue; }
                pc = 0x802625ac;
            }
            0x802625ac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802625c8; continue;
                pc = 0x802625b4;
            }
            0x802625b4 => { ins += 5;
                c.r[4] = c.r[17] | c.r[17];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x802625c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802625c8;
            }
            0x802625c8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x80262600; continue; }
                pc = 0x802625d0;
            }
            0x802625d0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x802625e4; continue; }
                pc = 0x802625d4;
            }
            0x802625d4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x802625f0; continue; }
                pc = 0x802625dc;
            }
            0x802625dc => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x802625f8; continue; }
                pc = 0x802625e0;
            }
            0x802625e0 => { ins += 1;
                pc = 0x8026260c; continue;
                pc = 0x802625e4;
            }
            0x802625e4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x8026260c; continue; }
                pc = 0x802625ec;
            }
            0x802625ec => { ins += 1;
                pc = 0x80262608; continue;
                pc = 0x802625f0;
            }
            0x802625f0 => { ins += 2;
                c.r[14] = 0x100;
                pc = 0x8026260c; continue;
                pc = 0x802625f8;
            }
            0x802625f8 => { ins += 2;
                c.r[14] = 0x101;
                pc = 0x8026260c; continue;
                pc = 0x80262600;
            }
            0x80262600 => { ins += 2;
                c.r[14] = 0x102;
                pc = 0x8026260c; continue;
                pc = 0x80262608;
            }
            0x80262608 => { ins += 1;
                c.r[14] = 0x103;
                pc = 0x8026260c;
            }
            0x8026260c => { ins += 8;
                c.r[3] = c.r[14] | c.r[14];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x8026262c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801026a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026262c;
            }
            0x8026262c => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x16;
                c.r[6] = 0x0;
                c.lr = 0x80262640; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262640;
            }
            0x80262640 => { ins += 4;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[16] | c.r[16];
                c.sth(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80262650; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262650;
            }
            0x80262650 => { ins += 1;
                c.lr = 0x80262654; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e1a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262654;
            }
            0x80262654 => { ins += 4;
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[25] = 0x0;
                c.r[29] = 0x0;
                pc = 0x80262b58; continue;
                pc = 0x80262664;
            }
            0x80262664 => { ins += 3;
                c.r[3] = c.r[15] | c.r[15];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80262670; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f981c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262670;
            }
            0x80262670 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x80262680; continue; }
                pc = 0x80262678;
            }
            0x80262678 => { ins += 2;
                c.r[25] = c.r[29] | c.r[29];
                pc = 0x80262b54; continue;
                pc = 0x80262680;
            }
            0x80262680 => { ins += 2;
                c.r[4] = 0x1;
                c.lr = 0x80262688; { c.steps = st; c.cycles = ins; c.cr = cr; f_80205c24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262688;
            }
            0x80262688 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80262698; continue; }
                pc = 0x80262690;
            }
            0x80262690 => { ins += 2;
                c.r[25] = c.r[29] | c.r[29];
                pc = 0x80262b54; continue;
                pc = 0x80262698;
            }
            0x80262698 => { ins += 1;
                c.r[14] = c.r[29].rotate_left(0) & 0xffff;
                pc = 0x8026269c;
            }
            0x8026269c => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802626a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80207760(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802626a4;
            }
            0x802626a4 => { ins += 1;
                c.lr = 0x802626a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802626a8;
            }
            0x802626a8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80262970; continue; }
                pc = 0x802626b4;
            }
            0x802626b4 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802626bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802626bc;
            }
            0x802626bc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802626d8; continue; }
                pc = 0x802626c8;
            }
            0x802626c8 => { ins += 1;
                c.lr = 0x802626cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80265924(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802626cc;
            }
            0x802626cc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80262954; continue; }
                pc = 0x802626d8;
            }
            0x802626d8 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[4] = c.r[15] | c.r[15];
                c.r[5] = c.r[14] | c.r[14];
                c.r[6] = c.r[16] | c.r[16];
                c.lr = 0x802626ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_800111c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802626ec;
            }
            0x802626ec => { ins += 2;
                c.r[18] = c.r[3] | c.r[3];
                c.lr = 0x802626f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80089f70(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802626f4;
            }
            0x802626f4 => { ins += 2;
                c.r[17] = c.r[3] | c.r[3];
                c.lr = 0x802626fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802626fc;
            }
            0x802626fc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80262970; continue; }
                pc = 0x80262708;
            }
            0x80262708 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[17], 0x3);
                if !cr_bit(cr, 2) { pc = 0x80262734; continue; }
                pc = 0x80262710;
            }
            0x80262710 => { ins += 8;
                c.r[4] = 0x80370000;
                c.r[3] = c.r[31] | c.r[31];
                c.r[7] = c.r[4].wrapping_add(0x5d30);
                c.r[5] = 0x8;
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x80262730; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020505c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262730;
            }
            0x80262730 => { ins += 1;
                pc = 0x80262b50; continue;
                pc = 0x80262734;
            }
            0x80262734 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[17], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802628f0; continue; }
                pc = 0x8026273c;
            }
            0x8026273c => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.lr = 0x80262748; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ff1bc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262748;
            }
            0x80262748 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80262758; continue; }
                pc = 0x80262750;
            }
            0x80262750 => { ins += 2;
                c.r[25] = c.r[29] | c.r[29];
                pc = 0x80262b54; continue;
                pc = 0x80262758;
            }
            0x80262758 => { ins += 2;
                c.r[3] = c.r[18] | c.r[18];
                c.lr = 0x80262760; { c.steps = st; c.cycles = ins; c.cr = cr; f_80089f68(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262760;
            }
            0x80262760 => { ins += 2;
                c.r[20] = c.r[3] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[20]);
                if cr_bit(cr, 0) { pc = 0x8026269c; continue; }
                pc = 0x80262768;
            }
            0x80262768 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80262770; { c.steps = st; c.cycles = ins; c.cr = cr; f_80205b8c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262770;
            }
            0x80262770 => { ins += 4;
                c.r[6] = c.r[20] | c.r[20];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x80262780; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262780;
            }
            0x80262780 => { ins += 2;
                c.r[26] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[26]);
                if cr_bit(cr, 2) { pc = 0x8026269c; continue; }
                pc = 0x80262788;
            }
            0x80262788 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff85d8));
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x8026269c; continue; }
                pc = 0x80262798;
            }
            0x80262798 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], 0x165);
                if cr_bit(cr, 2) { pc = 0x8026269c; continue; }
                pc = 0x802627a0;
            }
            0x802627a0 => { ins += 10;
                c.r[4] = 0x80260000;
                c.r[3] = c.r[31] | c.r[31];
                c.r[6] = c.r[4].wrapping_add(0x2d34);
                c.r[5] = c.r[16] | c.r[16];
                c.r[4] = c.r[26] | c.r[26];
                c.r[30] = 0x0;
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                c.r[9] = 0xffffffff;
                c.lr = 0x802627c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8022b2cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802627c8;
            }
            0x802627c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x802627d8; continue; }
                pc = 0x802627d0;
            }
            0x802627d0 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x802628a8; continue;
                pc = 0x802627d8;
            }
            0x802627d8 => { ins += 2;
                c.r[3] = c.r[18] | c.r[18];
                c.lr = 0x802627e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80089f60(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802627e0;
            }
            0x802627e0 => { ins += 5;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[22] = 0x0;
                c.r[17] = 0x0;
                c.r[24] = 0x0;
                pc = 0x8026289c; continue;
                pc = 0x802627f4;
            }
            0x802627f4 => { ins += 3;
                c.r[4] = c.r[24] | c.r[24];
                c.r[3] = 0x0;
                c.lr = 0x80262800; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f47b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262800;
            }
            0x80262800 => { ins += 2;
                c.r[18] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[18]);
                if cr_bit(cr, 2) { pc = 0x80262898; continue; }
                pc = 0x80262808;
            }
            0x80262808 => { ins += 2;
                c.r[23] = 0x0;
                pc = 0x8026287c; continue;
                pc = 0x80262810;
            }
            0x80262810 => { ins += 3;
                c.r[3] = c.r[18] | c.r[18];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8026281c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7258(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026281c;
            }
            0x8026281c => { ins += 2;
                c.r[19] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[19]);
                if cr_bit(cr, 2) { pc = 0x80262878; continue; }
                pc = 0x80262824;
            }
            0x80262824 => { ins += 2;
                c.r[21] = 0x0;
                pc = 0x80262860; continue;
                pc = 0x8026282c;
            }
            0x8026282c => { ins += 5;
                c.r[3] = c.r[19] | c.r[19];
                c.r[6] = c.r[21] | c.r[21];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x80262840; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262840;
            }
            0x80262840 => { ins += 4;
                c.r[0] = c.r[22].rotate_left(0) & 0xffff;
                c.r[30] = c.r[3] | c.r[3];
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[27]);
                if !cr_bit(cr, 2) { pc = 0x80262858; continue; }
                pc = 0x80262850;
            }
            0x80262850 => { ins += 2;
                c.r[17] = 0x1;
                pc = 0x8026286c; continue;
                pc = 0x80262858;
            }
            0x80262858 => { ins += 2;
                c.r[22] = c.r[22].wrapping_add(0x1);
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x80262860;
            }
            0x80262860 => { ins += 3;
                c.r[0] = c.r[21].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x8026282c; continue; }
                pc = 0x8026286c;
            }
            0x8026286c => { ins += 3;
                c.r[0] = c.r[17].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8026288c; continue; }
                pc = 0x80262878;
            }
            0x80262878 => { ins += 1;
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8026287c;
            }
            0x8026287c => { ins += 4;
                c.r[0] = c.lhz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[23].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80262810; continue; }
                pc = 0x8026288c;
            }
            0x8026288c => { ins += 3;
                c.r[0] = c.r[17].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802628a8; continue; }
                pc = 0x80262898;
            }
            0x80262898 => { ins += 1;
                c.r[24] = c.r[24].wrapping_add(0x1);
                pc = 0x8026289c;
            }
            0x8026289c => { ins += 3;
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x802627f4; continue; }
                pc = 0x802628a8;
            }
            0x802628a8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x8026269c; continue; }
                pc = 0x802628b0;
            }
            0x802628b0 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[16] | c.r[16];
                c.lr = 0x802628bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802628bc;
            }
            0x802628bc => { ins += 12;
                c.r[0] = 0x0;
                c.r[4] = 0x80370000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[9] = c.r[3] | c.r[3];
                c.r[7] = c.r[4].wrapping_add(0x5ca8);
                c.r[3] = c.r[31] | c.r[31];
                c.r[8] = c.r[26] | c.r[26];
                c.r[10] = c.r[20] | c.r[20];
                c.r[4] = 0x0;
                c.r[5] = 0x13;
                c.r[6] = 0x0;
                c.lr = 0x802628ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80204f6c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802628ec;
            }
            0x802628ec => { ins += 1;
                pc = 0x80262b50; continue;
                pc = 0x802628f0;
            }
            0x802628f0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[17], 0x2);
                if !cr_bit(cr, 2) { pc = 0x8026294c; continue; }
                pc = 0x802628f8;
            }
            0x802628f8 => { ins += 2;
                c.r[3] = c.r[18] | c.r[18];
                c.lr = 0x80262900; { c.steps = st; c.cycles = ins; c.cr = cr; f_80089f58(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262900;
            }
            0x80262900 => { ins += 5;
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[15] | c.r[15];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x80262914; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262914;
            }
            0x80262914 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0xce;
                c.r[6] = 0x0;
                c.lr = 0x80262924; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262924;
            }
            0x80262924 => { ins += 2;
                c.r[8] = c.r[3] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[8]);
                if cr_bit(cr, 0) { pc = 0x8026269c; continue; }
                pc = 0x8026292c;
            }
            0x8026292c => { ins += 7;
                c.r[4] = 0x80370000;
                c.r[3] = c.r[31] | c.r[31];
                c.r[7] = c.r[4].wrapping_add(0x5d30);
                c.r[5] = 0x9;
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80262948; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020505c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262948;
            }
            0x80262948 => { ins += 1;
                pc = 0x80262b50; continue;
                pc = 0x8026294c;
            }
            0x8026294c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[17], 0x4);
                if !cr_bit(cr, 2) { pc = 0x80262968; continue; }
                pc = 0x80262954;
            }
            0x80262954 => { ins += 4;
                c.r[3] = c.r[15] | c.r[15];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[16] | c.r[16];
                c.lr = 0x80262964; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f9130(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262964;
            }
            0x80262964 => { ins += 1;
                pc = 0x80262b50; continue;
                pc = 0x80262968;
            }
            0x80262968 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[17], 0x5);
                if !cr_bit(cr, 2) { pc = 0x80262b38; continue; }
                pc = 0x80262970;
            }
            0x80262970 => { ins += 2;
                c.r[3] = c.r[15] | c.r[15];
                c.lr = 0x80262978; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f9790(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262978;
            }
            0x80262978 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80262980; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262980;
            }
            0x80262980 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80262b30; continue; }
                pc = 0x80262988;
            }
            0x80262988 => { ins += 5;
                c.r[4] = c.r[15] | c.r[15];
                c.r[5] = c.r[16] | c.r[16];
                c.r[14] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x8026299c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026299c;
            }
            0x8026299c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802629ac; continue; }
                pc = 0x802629a4;
            }
            0x802629a4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262a08; continue;
                pc = 0x802629ac;
            }
            0x802629ac => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x802629bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802629bc;
            }
            0x802629bc => { ins += 4;
                c.r[17] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[15] | c.r[15];
                c.r[4] = c.r[16] | c.r[16];
                c.lr = 0x802629cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802629cc;
            }
            0x802629cc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802629dc; continue; }
                pc = 0x802629d4;
            }
            0x802629d4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262a08; continue;
                pc = 0x802629dc;
            }
            0x802629dc => { ins += 1;
                c.lr = 0x802629e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802629e0;
            }
            0x802629e0 => { ins += 1;
                c.lr = 0x802629e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802629e4;
            }
            0x802629e4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x802629f4; continue; }
                pc = 0x802629ec;
            }
            0x802629ec => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262a08; continue;
                pc = 0x802629f4;
            }
            0x802629f4 => { ins += 5;
                c.r[4] = c.r[17] | c.r[17];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80262a08; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262a08;
            }
            0x80262a08 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x80262a40; continue; }
                pc = 0x80262a10;
            }
            0x80262a10 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80262a24; continue; }
                pc = 0x80262a14;
            }
            0x80262a14 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x80262a30; continue; }
                pc = 0x80262a1c;
            }
            0x80262a1c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80262a38; continue; }
                pc = 0x80262a20;
            }
            0x80262a20 => { ins += 1;
                pc = 0x80262a4c; continue;
                pc = 0x80262a24;
            }
            0x80262a24 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x80262a4c; continue; }
                pc = 0x80262a2c;
            }
            0x80262a2c => { ins += 1;
                pc = 0x80262a48; continue;
                pc = 0x80262a30;
            }
            0x80262a30 => { ins += 2;
                c.r[14] = 0x100;
                pc = 0x80262a4c; continue;
                pc = 0x80262a38;
            }
            0x80262a38 => { ins += 2;
                c.r[14] = 0x101;
                pc = 0x80262a4c; continue;
                pc = 0x80262a40;
            }
            0x80262a40 => { ins += 2;
                c.r[14] = 0x102;
                pc = 0x80262a4c; continue;
                pc = 0x80262a48;
            }
            0x80262a48 => { ins += 1;
                c.r[14] = 0x103;
                pc = 0x80262a4c;
            }
            0x80262a4c => { ins += 2;
                c.r[3] = c.r[14] | c.r[14];
                c.lr = 0x80262a54; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102620(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262a54;
            }
            0x80262a54 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80262b30; continue; }
                pc = 0x80262a5c;
            }
            0x80262a5c => { ins += 5;
                c.r[4] = c.r[15] | c.r[15];
                c.r[5] = c.r[16] | c.r[16];
                c.r[14] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x80262a70; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262a70;
            }
            0x80262a70 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80262a80; continue; }
                pc = 0x80262a78;
            }
            0x80262a78 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262adc; continue;
                pc = 0x80262a80;
            }
            0x80262a80 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80262a90; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262a90;
            }
            0x80262a90 => { ins += 4;
                c.r[17] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[15] | c.r[15];
                c.r[4] = c.r[16] | c.r[16];
                c.lr = 0x80262aa0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262aa0;
            }
            0x80262aa0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80262ab0; continue; }
                pc = 0x80262aa8;
            }
            0x80262aa8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262adc; continue;
                pc = 0x80262ab0;
            }
            0x80262ab0 => { ins += 1;
                c.lr = 0x80262ab4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262ab4;
            }
            0x80262ab4 => { ins += 1;
                c.lr = 0x80262ab8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262ab8;
            }
            0x80262ab8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80262ac8; continue; }
                pc = 0x80262ac0;
            }
            0x80262ac0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262adc; continue;
                pc = 0x80262ac8;
            }
            0x80262ac8 => { ins += 5;
                c.r[4] = c.r[17] | c.r[17];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80262adc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262adc;
            }
            0x80262adc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x80262b14; continue; }
                pc = 0x80262ae4;
            }
            0x80262ae4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80262af8; continue; }
                pc = 0x80262ae8;
            }
            0x80262ae8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x80262b04; continue; }
                pc = 0x80262af0;
            }
            0x80262af0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80262b0c; continue; }
                pc = 0x80262af4;
            }
            0x80262af4 => { ins += 1;
                pc = 0x80262b20; continue;
                pc = 0x80262af8;
            }
            0x80262af8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x80262b20; continue; }
                pc = 0x80262b00;
            }
            0x80262b00 => { ins += 1;
                pc = 0x80262b1c; continue;
                pc = 0x80262b04;
            }
            0x80262b04 => { ins += 2;
                c.r[14] = 0x100;
                pc = 0x80262b20; continue;
                pc = 0x80262b0c;
            }
            0x80262b0c => { ins += 2;
                c.r[14] = 0x101;
                pc = 0x80262b20; continue;
                pc = 0x80262b14;
            }
            0x80262b14 => { ins += 2;
                c.r[14] = 0x102;
                pc = 0x80262b20; continue;
                pc = 0x80262b1c;
            }
            0x80262b1c => { ins += 1;
                c.r[14] = 0x103;
                pc = 0x80262b20;
            }
            0x80262b20 => { ins += 4;
                c.r[3] = c.r[14] | c.r[14];
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.lr = 0x80262b30; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102568(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262b30;
            }
            0x80262b30 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262d20; continue;
                pc = 0x80262b38;
            }
            0x80262b38 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[17], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80262954; continue; }
                pc = 0x80262b40;
            }
            0x80262b40 => { ins += 2;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80262664; continue; }
                pc = 0x80262b48;
            }
            0x80262b48 => { ins += 2;
                c.r[29] = c.r[25] | c.r[25];
                pc = 0x80262664; continue;
                pc = 0x80262b50;
            }
            0x80262b50 => { ins += 1;
                c.r[25] = c.r[29] | c.r[29];
                pc = 0x80262b54;
            }
            0x80262b54 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x80262b58;
            }
            0x80262b58 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x80262664; continue; }
                pc = 0x80262b64;
            }
            0x80262b64 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80262b6c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262b6c;
            }
            0x80262b6c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80262d1c; continue; }
                pc = 0x80262b74;
            }
            0x80262b74 => { ins += 5;
                c.r[4] = c.r[15] | c.r[15];
                c.r[5] = c.r[16] | c.r[16];
                c.r[14] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x80262b88; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262b88;
            }
            0x80262b88 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80262b98; continue; }
                pc = 0x80262b90;
            }
            0x80262b90 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262bf4; continue;
                pc = 0x80262b98;
            }
            0x80262b98 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80262ba8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262ba8;
            }
            0x80262ba8 => { ins += 4;
                c.r[17] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[15] | c.r[15];
                c.r[4] = c.r[16] | c.r[16];
                c.lr = 0x80262bb8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262bb8;
            }
            0x80262bb8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80262bc8; continue; }
                pc = 0x80262bc0;
            }
            0x80262bc0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262bf4; continue;
                pc = 0x80262bc8;
            }
            0x80262bc8 => { ins += 1;
                c.lr = 0x80262bcc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262bcc;
            }
            0x80262bcc => { ins += 1;
                c.lr = 0x80262bd0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262bd0;
            }
            0x80262bd0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80262be0; continue; }
                pc = 0x80262bd8;
            }
            0x80262bd8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262bf4; continue;
                pc = 0x80262be0;
            }
            0x80262be0 => { ins += 5;
                c.r[4] = c.r[17] | c.r[17];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80262bf4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262bf4;
            }
            0x80262bf4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x80262c2c; continue; }
                pc = 0x80262bfc;
            }
            0x80262bfc => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80262c10; continue; }
                pc = 0x80262c00;
            }
            0x80262c00 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x80262c1c; continue; }
                pc = 0x80262c08;
            }
            0x80262c08 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80262c24; continue; }
                pc = 0x80262c0c;
            }
            0x80262c0c => { ins += 1;
                pc = 0x80262c38; continue;
                pc = 0x80262c10;
            }
            0x80262c10 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x80262c38; continue; }
                pc = 0x80262c18;
            }
            0x80262c18 => { ins += 1;
                pc = 0x80262c34; continue;
                pc = 0x80262c1c;
            }
            0x80262c1c => { ins += 2;
                c.r[14] = 0x100;
                pc = 0x80262c38; continue;
                pc = 0x80262c24;
            }
            0x80262c24 => { ins += 2;
                c.r[14] = 0x101;
                pc = 0x80262c38; continue;
                pc = 0x80262c2c;
            }
            0x80262c2c => { ins += 2;
                c.r[14] = 0x102;
                pc = 0x80262c38; continue;
                pc = 0x80262c34;
            }
            0x80262c34 => { ins += 1;
                c.r[14] = 0x103;
                pc = 0x80262c38;
            }
            0x80262c38 => { ins += 2;
                c.r[3] = c.r[14] | c.r[14];
                c.lr = 0x80262c40; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102620(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262c40;
            }
            0x80262c40 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80262d1c; continue; }
                pc = 0x80262c48;
            }
            0x80262c48 => { ins += 5;
                c.r[4] = c.r[15] | c.r[15];
                c.r[5] = c.r[16] | c.r[16];
                c.r[14] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x80262c5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262c5c;
            }
            0x80262c5c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80262c6c; continue; }
                pc = 0x80262c64;
            }
            0x80262c64 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262cc8; continue;
                pc = 0x80262c6c;
            }
            0x80262c6c => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80262c7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262c7c;
            }
            0x80262c7c => { ins += 4;
                c.r[17] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[15] | c.r[15];
                c.r[4] = c.r[16] | c.r[16];
                c.lr = 0x80262c8c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262c8c;
            }
            0x80262c8c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80262c9c; continue; }
                pc = 0x80262c94;
            }
            0x80262c94 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262cc8; continue;
                pc = 0x80262c9c;
            }
            0x80262c9c => { ins += 1;
                c.lr = 0x80262ca0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262ca0;
            }
            0x80262ca0 => { ins += 1;
                c.lr = 0x80262ca4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262ca4;
            }
            0x80262ca4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80262cb4; continue; }
                pc = 0x80262cac;
            }
            0x80262cac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262cc8; continue;
                pc = 0x80262cb4;
            }
            0x80262cb4 => { ins += 5;
                c.r[4] = c.r[17] | c.r[17];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80262cc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262cc8;
            }
            0x80262cc8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x80262d00; continue; }
                pc = 0x80262cd0;
            }
            0x80262cd0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80262ce4; continue; }
                pc = 0x80262cd4;
            }
            0x80262cd4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x80262cf0; continue; }
                pc = 0x80262cdc;
            }
            0x80262cdc => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80262cf8; continue; }
                pc = 0x80262ce0;
            }
            0x80262ce0 => { ins += 1;
                pc = 0x80262d0c; continue;
                pc = 0x80262ce4;
            }
            0x80262ce4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x80262d0c; continue; }
                pc = 0x80262cec;
            }
            0x80262cec => { ins += 1;
                pc = 0x80262d08; continue;
                pc = 0x80262cf0;
            }
            0x80262cf0 => { ins += 2;
                c.r[14] = 0x100;
                pc = 0x80262d0c; continue;
                pc = 0x80262cf8;
            }
            0x80262cf8 => { ins += 2;
                c.r[14] = 0x101;
                pc = 0x80262d0c; continue;
                pc = 0x80262d00;
            }
            0x80262d00 => { ins += 2;
                c.r[14] = 0x102;
                pc = 0x80262d0c; continue;
                pc = 0x80262d08;
            }
            0x80262d08 => { ins += 1;
                c.r[14] = 0x103;
                pc = 0x80262d0c;
            }
            0x80262d0c => { ins += 4;
                c.r[3] = c.r[14] | c.r[14];
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.lr = 0x80262d1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102568(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262d1c;
            }
            0x80262d1c => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80262d20;
            }
            0x80262d20 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 14..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x64));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x60);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80262508: bad pc {:#x}", pc),
        }
    }
}

/// _fightMenuFightTrainerAgbHeroSelectDefensePokemon__FP15FightOutPokemonUsUs (0x80262d34, 8 bytes)
#[inline(never)]
pub fn f_80262d34(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80262d34) { return; }
        if c.prof { crate::prof::enter(c, 0x80262d34); f_80262d34_at(c, 0x80262d34); crate::prof::exit(c, 0x80262d34); return; }
    }
    f_80262d34_b(c, 0x80262d34);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80262d34_at(c: &mut Cpu, pc: u32) { f_80262d34_b(c, pc) }
#[inline(always)]
fn f_80262d34_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80262d34;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80262d34, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80262d34 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80262d34: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuFightTrainerAgbHeroSelectIrekaeFightPokemon (0x80262d3c, 1072 bytes)
#[inline(never)]
pub fn f_80262d3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80262d3c) { return; }
        if c.prof { crate::prof::enter(c, 0x80262d3c); f_80262d3c_at(c, 0x80262d3c); crate::prof::exit(c, 0x80262d3c); return; }
    }
    f_80262d3c_b(c, 0x80262d3c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80262d3c_at(c: &mut Cpu, pc: u32) { f_80262d3c_b(c, pc) }
#[inline(never)]
fn f_80262d3c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80262d3c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80262d3c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80262d3c => { ins += 22;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[31] = c.r[6] | c.r[6];
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.r[5] = 0x4b;
                c.r[6] = 0x0;
                c.lr = 0x80262d6c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262d6c;
            }
            0x80262d6c => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[26] = c.r[0] | c.r[0];
                c.lr = 0x80262d7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262d7c;
            }
            0x80262d7c => { ins += 1;
                c.lr = 0x80262d80; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e1a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262d80;
            }
            0x80262d80 => { ins += 3;
                c.r[27] = c.r[3].rotate_left(0) & 0xff;
                c.r[25] = 0x0;
                pc = 0x80262dac; continue;
                pc = 0x80262d8c;
            }
            0x80262d8c => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x80262da0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262da0;
            }
            0x80262da0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[31]);
                if cr_bit(cr, 2) { pc = 0x80262db8; continue; }
                pc = 0x80262da8;
            }
            0x80262da8 => { ins += 1;
                c.r[25] = c.r[25].wrapping_add(0x1);
                pc = 0x80262dac;
            }
            0x80262dac => { ins += 3;
                c.r[0] = c.r[25].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[27]);
                if cr_bit(cr, 0) { pc = 0x80262d8c; continue; }
                pc = 0x80262db8;
            }
            0x80262db8 => { ins += 3;
                c.r[0] = c.r[25].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[27]);
                if cr_bit(cr, 0) { pc = 0x80262ddc; continue; }
                pc = 0x80262dc4;
            }
            0x80262dc4 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.lr = 0x80262dd8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8024e690(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262dd8;
            }
            0x80262dd8 => { ins += 1;
                pc = 0x80263158; continue;
                pc = 0x80262ddc;
            }
            0x80262ddc => { ins += 1;
                c.lr = 0x80262de0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262de0;
            }
            0x80262de0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80262f98; continue; }
                pc = 0x80262dec;
            }
            0x80262dec => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80262df4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262df4;
            }
            0x80262df4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80262e10; continue; }
                pc = 0x80262e00;
            }
            0x80262e00 => { ins += 1;
                c.lr = 0x80262e04; { c.steps = st; c.cycles = ins; c.cr = cr; f_80265924(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262e04;
            }
            0x80262e04 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80262f78; continue; }
                pc = 0x80262e10;
            }
            0x80262e10 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80262e18; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262e18;
            }
            0x80262e18 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80262f04; continue; }
                pc = 0x80262e20;
            }
            0x80262e20 => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.r[28] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x80262e34; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262e34;
            }
            0x80262e34 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80262e44; continue; }
                pc = 0x80262e3c;
            }
            0x80262e3c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262ea0; continue;
                pc = 0x80262e44;
            }
            0x80262e44 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80262e54; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262e54;
            }
            0x80262e54 => { ins += 4;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80262e64; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262e64;
            }
            0x80262e64 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80262e74; continue; }
                pc = 0x80262e6c;
            }
            0x80262e6c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262ea0; continue;
                pc = 0x80262e74;
            }
            0x80262e74 => { ins += 1;
                c.lr = 0x80262e78; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262e78;
            }
            0x80262e78 => { ins += 1;
                c.lr = 0x80262e7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262e7c;
            }
            0x80262e7c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80262e8c; continue; }
                pc = 0x80262e84;
            }
            0x80262e84 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80262ea0; continue;
                pc = 0x80262e8c;
            }
            0x80262e8c => { ins += 5;
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80262ea0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262ea0;
            }
            0x80262ea0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x80262ed8; continue; }
                pc = 0x80262ea8;
            }
            0x80262ea8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80262ebc; continue; }
                pc = 0x80262eac;
            }
            0x80262eac => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x80262ec8; continue; }
                pc = 0x80262eb4;
            }
            0x80262eb4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80262ed0; continue; }
                pc = 0x80262eb8;
            }
            0x80262eb8 => { ins += 1;
                pc = 0x80262ee4; continue;
                pc = 0x80262ebc;
            }
            0x80262ebc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x80262ee4; continue; }
                pc = 0x80262ec4;
            }
            0x80262ec4 => { ins += 1;
                pc = 0x80262ee0; continue;
                pc = 0x80262ec8;
            }
            0x80262ec8 => { ins += 2;
                c.r[28] = 0x100;
                pc = 0x80262ee4; continue;
                pc = 0x80262ed0;
            }
            0x80262ed0 => { ins += 2;
                c.r[28] = 0x101;
                pc = 0x80262ee4; continue;
                pc = 0x80262ed8;
            }
            0x80262ed8 => { ins += 2;
                c.r[28] = 0x102;
                pc = 0x80262ee4; continue;
                pc = 0x80262ee0;
            }
            0x80262ee0 => { ins += 1;
                c.r[28] = 0x103;
                pc = 0x80262ee4;
            }
            0x80262ee4 => { ins += 8;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80262f04; { c.steps = st; c.cycles = ins; c.cr = cr; f_801026a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262f04;
            }
            0x80262f04 => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = c.r[25].rotate_left(0) & 0xffff;
                c.lr = 0x80262f18; { c.steps = st; c.cycles = ins; c.cr = cr; f_800111e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262f18;
            }
            0x80262f18 => { ins += 2;
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x80262f20; { c.steps = st; c.cycles = ins; c.cr = cr; f_80089f70(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262f20;
            }
            0x80262f20 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x80262f28; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262f28;
            }
            0x80262f28 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80262f98; continue; }
                pc = 0x80262f34;
            }
            0x80262f34 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x2);
                if !cr_bit(cr, 2) { pc = 0x80262f70; continue; }
                pc = 0x80262f3c;
            }
            0x80262f3c => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x80262f44; { c.steps = st; c.cycles = ins; c.cr = cr; f_80089f58(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262f44;
            }
            0x80262f44 => { ins += 5;
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x80262f58; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262f58;
            }
            0x80262f58 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0xce;
                c.r[6] = 0x0;
                c.lr = 0x80262f68; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262f68;
            }
            0x80262f68 => { ins += 2;
                c.r[31] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x80262f9c; continue;
                pc = 0x80262f70;
            }
            0x80262f70 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x4);
                if !cr_bit(cr, 2) { pc = 0x80262f90; continue; }
                pc = 0x80262f78;
            }
            0x80262f78 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80262f88; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f93f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262f88;
            }
            0x80262f88 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80262f9c; continue;
                pc = 0x80262f90;
            }
            0x80262f90 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x5);
                if !cr_bit(cr, 2) { pc = 0x80262f78; continue; }
                pc = 0x80262f98;
            }
            0x80262f98 => { ins += 1;
                c.r[31] = 0xfffffffe;
                pc = 0x80262f9c;
            }
            0x80262f9c => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80262fa4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262fa4;
            }
            0x80262fa4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80263154; continue; }
                pc = 0x80262fac;
            }
            0x80262fac => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.r[28] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x80262fc0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262fc0;
            }
            0x80262fc0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80262fd0; continue; }
                pc = 0x80262fc8;
            }
            0x80262fc8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8026302c; continue;
                pc = 0x80262fd0;
            }
            0x80262fd0 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80262fe0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262fe0;
            }
            0x80262fe0 => { ins += 4;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80262ff0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80262ff0;
            }
            0x80262ff0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263000; continue; }
                pc = 0x80262ff8;
            }
            0x80262ff8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8026302c; continue;
                pc = 0x80263000;
            }
            0x80263000 => { ins += 1;
                c.lr = 0x80263004; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263004;
            }
            0x80263004 => { ins += 1;
                c.lr = 0x80263008; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263008;
            }
            0x80263008 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80263018; continue; }
                pc = 0x80263010;
            }
            0x80263010 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8026302c; continue;
                pc = 0x80263018;
            }
            0x80263018 => { ins += 5;
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x8026302c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026302c;
            }
            0x8026302c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x80263064; continue; }
                pc = 0x80263034;
            }
            0x80263034 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80263048; continue; }
                pc = 0x80263038;
            }
            0x80263038 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x80263054; continue; }
                pc = 0x80263040;
            }
            0x80263040 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8026305c; continue; }
                pc = 0x80263044;
            }
            0x80263044 => { ins += 1;
                pc = 0x80263070; continue;
                pc = 0x80263048;
            }
            0x80263048 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x80263070; continue; }
                pc = 0x80263050;
            }
            0x80263050 => { ins += 1;
                pc = 0x8026306c; continue;
                pc = 0x80263054;
            }
            0x80263054 => { ins += 2;
                c.r[28] = 0x100;
                pc = 0x80263070; continue;
                pc = 0x8026305c;
            }
            0x8026305c => { ins += 2;
                c.r[28] = 0x101;
                pc = 0x80263070; continue;
                pc = 0x80263064;
            }
            0x80263064 => { ins += 2;
                c.r[28] = 0x102;
                pc = 0x80263070; continue;
                pc = 0x8026306c;
            }
            0x8026306c => { ins += 1;
                c.r[28] = 0x103;
                pc = 0x80263070;
            }
            0x80263070 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80263078; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102620(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263078;
            }
            0x80263078 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80263154; continue; }
                pc = 0x80263080;
            }
            0x80263080 => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.r[28] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x80263094; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263094;
            }
            0x80263094 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802630a4; continue; }
                pc = 0x8026309c;
            }
            0x8026309c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263100; continue;
                pc = 0x802630a4;
            }
            0x802630a4 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x802630b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802630b4;
            }
            0x802630b4 => { ins += 4;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x802630c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802630c4;
            }
            0x802630c4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802630d4; continue; }
                pc = 0x802630cc;
            }
            0x802630cc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263100; continue;
                pc = 0x802630d4;
            }
            0x802630d4 => { ins += 1;
                c.lr = 0x802630d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802630d8;
            }
            0x802630d8 => { ins += 1;
                c.lr = 0x802630dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802630dc;
            }
            0x802630dc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x802630ec; continue; }
                pc = 0x802630e4;
            }
            0x802630e4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263100; continue;
                pc = 0x802630ec;
            }
            0x802630ec => { ins += 5;
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80263100; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263100;
            }
            0x80263100 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x80263138; continue; }
                pc = 0x80263108;
            }
            0x80263108 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8026311c; continue; }
                pc = 0x8026310c;
            }
            0x8026310c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x80263128; continue; }
                pc = 0x80263114;
            }
            0x80263114 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80263130; continue; }
                pc = 0x80263118;
            }
            0x80263118 => { ins += 1;
                pc = 0x80263144; continue;
                pc = 0x8026311c;
            }
            0x8026311c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x80263144; continue; }
                pc = 0x80263124;
            }
            0x80263124 => { ins += 1;
                pc = 0x80263140; continue;
                pc = 0x80263128;
            }
            0x80263128 => { ins += 2;
                c.r[28] = 0x100;
                pc = 0x80263144; continue;
                pc = 0x80263130;
            }
            0x80263130 => { ins += 2;
                c.r[28] = 0x101;
                pc = 0x80263144; continue;
                pc = 0x80263138;
            }
            0x80263138 => { ins += 2;
                c.r[28] = 0x102;
                pc = 0x80263144; continue;
                pc = 0x80263140;
            }
            0x80263140 => { ins += 1;
                c.r[28] = 0x103;
                pc = 0x80263144;
            }
            0x80263144 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.lr = 0x80263154; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102568(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263154;
            }
            0x80263154 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80263158;
            }
            0x80263158 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80262d3c: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuFightTrainerGcHeroOpenMenu (0x8026316c, 2652 bytes)
#[inline(never)]
pub fn f_8026316c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8026316c) { return; }
        if c.prof { crate::prof::enter(c, 0x8026316c); f_8026316c_at(c, 0x8026316c); crate::prof::exit(c, 0x8026316c); return; }
    }
    f_8026316c_b(c, 0x8026316c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8026316c_at(c: &mut Cpu, pc: u32) { f_8026316c_b(c, pc) }
#[inline(never)]
fn f_8026316c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8026316c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8026316c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8026316c => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                c.r[25] = c.r[5] | c.r[5];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80263190; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263190;
            }
            0x80263190 => { ins += 1;
                c.lr = 0x80263194; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e1a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263194;
            }
            0x80263194 => { ins += 3;
                c.r[27] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x0;
                c.lr = 0x802631a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802631a0;
            }
            0x802631a0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8026328c; continue; }
                pc = 0x802631a8;
            }
            0x802631a8 => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.r[24] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x802631bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802631bc;
            }
            0x802631bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802631cc; continue; }
                pc = 0x802631c4;
            }
            0x802631c4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263228; continue;
                pc = 0x802631cc;
            }
            0x802631cc => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x802631dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802631dc;
            }
            0x802631dc => { ins += 4;
                c.r[26] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x802631ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802631ec;
            }
            0x802631ec => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802631fc; continue; }
                pc = 0x802631f4;
            }
            0x802631f4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263228; continue;
                pc = 0x802631fc;
            }
            0x802631fc => { ins += 1;
                c.lr = 0x80263200; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263200;
            }
            0x80263200 => { ins += 1;
                c.lr = 0x80263204; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263204;
            }
            0x80263204 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80263214; continue; }
                pc = 0x8026320c;
            }
            0x8026320c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263228; continue;
                pc = 0x80263214;
            }
            0x80263214 => { ins += 5;
                c.r[4] = c.r[26] | c.r[26];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80263228; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263228;
            }
            0x80263228 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x80263260; continue; }
                pc = 0x80263230;
            }
            0x80263230 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80263244; continue; }
                pc = 0x80263234;
            }
            0x80263234 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x80263250; continue; }
                pc = 0x8026323c;
            }
            0x8026323c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80263258; continue; }
                pc = 0x80263240;
            }
            0x80263240 => { ins += 1;
                pc = 0x8026326c; continue;
                pc = 0x80263244;
            }
            0x80263244 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x8026326c; continue; }
                pc = 0x8026324c;
            }
            0x8026324c => { ins += 1;
                pc = 0x80263268; continue;
                pc = 0x80263250;
            }
            0x80263250 => { ins += 2;
                c.r[24] = 0x100;
                pc = 0x8026326c; continue;
                pc = 0x80263258;
            }
            0x80263258 => { ins += 2;
                c.r[24] = 0x101;
                pc = 0x8026326c; continue;
                pc = 0x80263260;
            }
            0x80263260 => { ins += 2;
                c.r[24] = 0x102;
                pc = 0x8026326c; continue;
                pc = 0x80263268;
            }
            0x80263268 => { ins += 1;
                c.r[24] = 0x103;
                pc = 0x8026326c;
            }
            0x8026326c => { ins += 8;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x8026328c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801026a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026328c;
            }
            0x8026328c => { ins += 4;
                c.r[30] = 0x0;
                c.r[26] = 0x0;
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                pc = 0x802639ac; continue;
                pc = 0x8026329c;
            }
            0x8026329c => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x802632a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f981c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802632a8;
            }
            0x802632a8 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                if !cr_bit(cr, 2) { pc = 0x802632b8; continue; }
                pc = 0x802632b0;
            }
            0x802632b0 => { ins += 2;
                c.r[26] = c.r[30] | c.r[30];
                pc = 0x802639a8; continue;
                pc = 0x802632b8;
            }
            0x802632b8 => { ins += 2;
                c.r[4] = 0x1;
                c.lr = 0x802632c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80205c24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802632c0;
            }
            0x802632c0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802632d0; continue; }
                pc = 0x802632c8;
            }
            0x802632c8 => { ins += 2;
                c.r[26] = c.r[30] | c.r[30];
                pc = 0x802639a8; continue;
                pc = 0x802632d0;
            }
            0x802632d0 => { ins += 1;
                c.lr = 0x802632d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802632d4;
            }
            0x802632d4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802632ec; continue; }
                pc = 0x802632e0;
            }
            0x802632e0 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x802632e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f9790(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802632e8;
            }
            0x802632e8 => { ins += 1;
                pc = 0x802639b8; continue;
                pc = 0x802632ec;
            }
            0x802632ec => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802632f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802632f4;
            }
            0x802632f4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263330; continue; }
                pc = 0x80263300;
            }
            0x80263300 => { ins += 1;
                c.lr = 0x80263304; { c.steps = st; c.cycles = ins; c.cr = cr; f_80265924(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263304;
            }
            0x80263304 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263330; continue; }
                pc = 0x80263310;
            }
            0x80263310 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80263318; { c.steps = st; c.cycles = ins; c.cr = cr; f_80207760(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263318;
            }
            0x80263318 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80263328; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f9130(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263328;
            }
            0x80263328 => { ins += 2;
                c.r[26] = c.r[30] | c.r[30];
                pc = 0x802639a8; continue;
                pc = 0x80263330;
            }
            0x80263330 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80263338; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263338;
            }
            0x80263338 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802633c8; continue; }
                pc = 0x80263344;
            }
            0x80263344 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x80263354; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263354;
            }
            0x80263354 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80263364; continue; }
                pc = 0x8026335c;
            }
            0x8026335c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802633c0; continue;
                pc = 0x80263364;
            }
            0x80263364 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80263374; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263374;
            }
            0x80263374 => { ins += 4;
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80263384; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263384;
            }
            0x80263384 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263394; continue; }
                pc = 0x8026338c;
            }
            0x8026338c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802633c0; continue;
                pc = 0x80263394;
            }
            0x80263394 => { ins += 1;
                c.lr = 0x80263398; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263398;
            }
            0x80263398 => { ins += 1;
                c.lr = 0x8026339c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026339c;
            }
            0x8026339c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x802633ac; continue; }
                pc = 0x802633a4;
            }
            0x802633a4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802633c0; continue;
                pc = 0x802633ac;
            }
            0x802633ac => { ins += 5;
                c.r[4] = c.r[24] | c.r[24];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x802633c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802633c0;
            }
            0x802633c0 => { ins += 2;
                c.r[4] = 0x1;
                c.lr = 0x802633c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011d9c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802633c8;
            }
            0x802633c8 => { ins += 2;
                c.r[0] = 0x0;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x802633d0;
            }
            0x802633d0 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x802633d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80207760(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802633d8;
            }
            0x802633d8 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[1].wrapping_add(0xc);
                c.r[3] = 0x0;
                c.lr = 0x802633e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1918(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802633e8;
            }
            0x802633e8 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802633f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802633f0;
            }
            0x802633f0 => { ins += 5;
                c.stb(c.r[1].wrapping_add(0x23), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[5] = 0x1;
                c.lr = 0x80263404; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011a1c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263404;
            }
            0x80263404 => { ins += 2;
                c.r[24] = c.r[3] | c.r[3];
                c.lr = 0x8026340c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026340c;
            }
            0x8026340c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802634bc; continue; }
                pc = 0x80263418;
            }
            0x80263418 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80263420; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263420;
            }
            0x80263420 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802634b0; continue; }
                pc = 0x8026342c;
            }
            0x8026342c => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x8026343c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026343c;
            }
            0x8026343c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8026344c; continue; }
                pc = 0x80263444;
            }
            0x80263444 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802634a8; continue;
                pc = 0x8026344c;
            }
            0x8026344c => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x8026345c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026345c;
            }
            0x8026345c => { ins += 4;
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8026346c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026346c;
            }
            0x8026346c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8026347c; continue; }
                pc = 0x80263474;
            }
            0x80263474 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802634a8; continue;
                pc = 0x8026347c;
            }
            0x8026347c => { ins += 1;
                c.lr = 0x80263480; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263480;
            }
            0x80263480 => { ins += 1;
                c.lr = 0x80263484; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263484;
            }
            0x80263484 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80263494; continue; }
                pc = 0x8026348c;
            }
            0x8026348c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802634a8; continue;
                pc = 0x80263494;
            }
            0x80263494 => { ins += 5;
                c.r[4] = c.r[24] | c.r[24];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x802634a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802634a8;
            }
            0x802634a8 => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x802634b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011d9c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802634b0;
            }
            0x802634b0 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x802634b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_800119a8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802634b8;
            }
            0x802634b8 => { ins += 1;
                pc = 0x802632e0; continue;
                pc = 0x802634bc;
            }
            0x802634bc => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802634c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802634c4;
            }
            0x802634c4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8026358c; continue; }
                pc = 0x802634d0;
            }
            0x802634d0 => { ins += 1;
                c.lr = 0x802634d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80265924(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802634d4;
            }
            0x802634d4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8026358c; continue; }
                pc = 0x802634e0;
            }
            0x802634e0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[24], 0x0);
                if !cr_bit(cr, 0) { pc = 0x8026358c; continue; }
                pc = 0x802634e8;
            }
            0x802634e8 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802634f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802634f0;
            }
            0x802634f0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80263580; continue; }
                pc = 0x802634fc;
            }
            0x802634fc => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x8026350c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026350c;
            }
            0x8026350c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8026351c; continue; }
                pc = 0x80263514;
            }
            0x80263514 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263578; continue;
                pc = 0x8026351c;
            }
            0x8026351c => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x8026352c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026352c;
            }
            0x8026352c => { ins += 4;
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8026353c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026353c;
            }
            0x8026353c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8026354c; continue; }
                pc = 0x80263544;
            }
            0x80263544 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263578; continue;
                pc = 0x8026354c;
            }
            0x8026354c => { ins += 1;
                c.lr = 0x80263550; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263550;
            }
            0x80263550 => { ins += 1;
                c.lr = 0x80263554; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263554;
            }
            0x80263554 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80263564; continue; }
                pc = 0x8026355c;
            }
            0x8026355c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263578; continue;
                pc = 0x80263564;
            }
            0x80263564 => { ins += 5;
                c.r[4] = c.r[24] | c.r[24];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x80263578; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263578;
            }
            0x80263578 => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x80263580; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011d9c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263580;
            }
            0x80263580 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x80263588; { c.steps = st; c.cycles = ins; c.cr = cr; f_800119a8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263588;
            }
            0x80263588 => { ins += 1;
                pc = 0x80263310; continue;
                pc = 0x8026358c;
            }
            0x8026358c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[24], 0x0);
                if cr_bit(cr, 0) { pc = 0x8026359c; continue; }
                pc = 0x80263594;
            }
            0x80263594 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x8026359c; { c.steps = st; c.cycles = ins; c.cr = cr; f_800119a8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026359c;
            }
            0x8026359c => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[6] = c.r[24] | c.r[24];
                c.lr = 0x802635b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80263bc8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802635b0;
            }
            0x802635b0 => { ins += 2;
                c.r[24] = c.r[3] | c.r[3];
                c.lr = 0x802635b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802635b8;
            }
            0x802635b8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263668; continue; }
                pc = 0x802635c4;
            }
            0x802635c4 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802635cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802635cc;
            }
            0x802635cc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8026365c; continue; }
                pc = 0x802635d8;
            }
            0x802635d8 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x802635e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802635e8;
            }
            0x802635e8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802635f8; continue; }
                pc = 0x802635f0;
            }
            0x802635f0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263654; continue;
                pc = 0x802635f8;
            }
            0x802635f8 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80263608; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263608;
            }
            0x80263608 => { ins += 4;
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80263618; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263618;
            }
            0x80263618 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263628; continue; }
                pc = 0x80263620;
            }
            0x80263620 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263654; continue;
                pc = 0x80263628;
            }
            0x80263628 => { ins += 1;
                c.lr = 0x8026362c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026362c;
            }
            0x8026362c => { ins += 1;
                c.lr = 0x80263630; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263630;
            }
            0x80263630 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80263640; continue; }
                pc = 0x80263638;
            }
            0x80263638 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263654; continue;
                pc = 0x80263640;
            }
            0x80263640 => { ins += 5;
                c.r[4] = c.r[24] | c.r[24];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x80263654; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263654;
            }
            0x80263654 => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x8026365c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011d9c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026365c;
            }
            0x8026365c => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x80263664; { c.steps = st; c.cycles = ins; c.cr = cr; f_800119a8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263664;
            }
            0x80263664 => { ins += 1;
                pc = 0x802632e0; continue;
                pc = 0x80263668;
            }
            0x80263668 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80263670; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263670;
            }
            0x80263670 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263738; continue; }
                pc = 0x8026367c;
            }
            0x8026367c => { ins += 1;
                c.lr = 0x80263680; { c.steps = st; c.cycles = ins; c.cr = cr; f_80265924(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263680;
            }
            0x80263680 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263738; continue; }
                pc = 0x8026368c;
            }
            0x8026368c => { ins += 2;
                c.r[0] = c.r[24].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80263738; continue; }
                pc = 0x80263694;
            }
            0x80263694 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8026369c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026369c;
            }
            0x8026369c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8026372c; continue; }
                pc = 0x802636a8;
            }
            0x802636a8 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x802636b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802636b8;
            }
            0x802636b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802636c8; continue; }
                pc = 0x802636c0;
            }
            0x802636c0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263724; continue;
                pc = 0x802636c8;
            }
            0x802636c8 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x802636d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802636d8;
            }
            0x802636d8 => { ins += 4;
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x802636e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802636e8;
            }
            0x802636e8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802636f8; continue; }
                pc = 0x802636f0;
            }
            0x802636f0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263724; continue;
                pc = 0x802636f8;
            }
            0x802636f8 => { ins += 1;
                c.lr = 0x802636fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802636fc;
            }
            0x802636fc => { ins += 1;
                c.lr = 0x80263700; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263700;
            }
            0x80263700 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80263710; continue; }
                pc = 0x80263708;
            }
            0x80263708 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263724; continue;
                pc = 0x80263710;
            }
            0x80263710 => { ins += 5;
                c.r[4] = c.r[24] | c.r[24];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x80263724; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263724;
            }
            0x80263724 => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x8026372c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011d9c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026372c;
            }
            0x8026372c => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x80263734; { c.steps = st; c.cycles = ins; c.cr = cr; f_800119a8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263734;
            }
            0x80263734 => { ins += 1;
                pc = 0x80263310; continue;
                pc = 0x80263738;
            }
            0x80263738 => { ins += 3;
                c.r[0] = c.r[24].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263904; continue; }
                pc = 0x80263744;
            }
            0x80263744 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8026374c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7e60(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026374c;
            }
            0x8026374c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802633d0; continue; }
                pc = 0x80263758;
            }
            0x80263758 => { ins += 2;
                c.r[3] = c.r[30].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[3]);
                if cr_bit(cr, 2) { pc = 0x80263808; continue; }
                pc = 0x80263760;
            }
            0x80263760 => { ins += 3;
                c.r[30] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.lr = 0x8026376c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026376c;
            }
            0x8026376c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802637fc; continue; }
                pc = 0x80263778;
            }
            0x80263778 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x80263788; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263788;
            }
            0x80263788 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80263798; continue; }
                pc = 0x80263790;
            }
            0x80263790 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802637f4; continue;
                pc = 0x80263798;
            }
            0x80263798 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x802637a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802637a8;
            }
            0x802637a8 => { ins += 4;
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x802637b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802637b8;
            }
            0x802637b8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802637c8; continue; }
                pc = 0x802637c0;
            }
            0x802637c0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802637f4; continue;
                pc = 0x802637c8;
            }
            0x802637c8 => { ins += 1;
                c.lr = 0x802637cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802637cc;
            }
            0x802637cc => { ins += 1;
                c.lr = 0x802637d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802637d0;
            }
            0x802637d0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x802637e0; continue; }
                pc = 0x802637d8;
            }
            0x802637d8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802637f4; continue;
                pc = 0x802637e0;
            }
            0x802637e0 => { ins += 5;
                c.r[4] = c.r[24] | c.r[24];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x802637f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802637f4;
            }
            0x802637f4 => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x802637fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011d9c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802637fc;
            }
            0x802637fc => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x80263804; { c.steps = st; c.cycles = ins; c.cr = cr; f_800119a8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263804;
            }
            0x80263804 => { ins += 1;
                pc = 0x8026329c; continue;
                pc = 0x80263808;
            }
            0x80263808 => { ins += 3;
                c.r[0] = c.r[25].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802633d0; continue; }
                pc = 0x80263814;
            }
            0x80263814 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802633d0; continue; }
                pc = 0x8026381c;
            }
            0x8026381c => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80263824; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263824;
            }
            0x80263824 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802638b4; continue; }
                pc = 0x80263830;
            }
            0x80263830 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x80263840; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263840;
            }
            0x80263840 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80263850; continue; }
                pc = 0x80263848;
            }
            0x80263848 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802638ac; continue;
                pc = 0x80263850;
            }
            0x80263850 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80263860; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263860;
            }
            0x80263860 => { ins += 4;
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80263870; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263870;
            }
            0x80263870 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263880; continue; }
                pc = 0x80263878;
            }
            0x80263878 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802638ac; continue;
                pc = 0x80263880;
            }
            0x80263880 => { ins += 1;
                c.lr = 0x80263884; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263884;
            }
            0x80263884 => { ins += 1;
                c.lr = 0x80263888; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263888;
            }
            0x80263888 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80263898; continue; }
                pc = 0x80263890;
            }
            0x80263890 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802638ac; continue;
                pc = 0x80263898;
            }
            0x80263898 => { ins += 5;
                c.r[4] = c.r[24] | c.r[24];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x802638ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802638ac;
            }
            0x802638ac => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x802638b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011d9c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802638b4;
            }
            0x802638b4 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x802638bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_800119a8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802638bc;
            }
            0x802638bc => { ins += 2;
                c.r[24] = 0x0;
                pc = 0x802638f0; continue;
                pc = 0x802638c4;
            }
            0x802638c4 => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[24] | c.r[24];
                c.lr = 0x802638d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f981c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802638d0;
            }
            0x802638d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x802638ec; continue; }
                pc = 0x802638d8;
            }
            0x802638d8 => { ins += 5;
                c.r[4] = 0x0;
                c.r[5] = 0x120;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x802638ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802638ec;
            }
            0x802638ec => { ins += 1;
                c.r[24] = c.r[24].wrapping_add(0x1);
                pc = 0x802638f0;
            }
            0x802638f0 => { ins += 3;
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[27]);
                if cr_bit(cr, 0) { pc = 0x802638c4; continue; }
                pc = 0x802638fc;
            }
            0x802638fc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263bb4; continue;
                pc = 0x80263904;
            }
            0x80263904 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x802633d0; continue; }
                pc = 0x8026390c;
            }
            0x8026390c => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80263914; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263914;
            }
            0x80263914 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802639a4; continue; }
                pc = 0x80263920;
            }
            0x80263920 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x80263930; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263930;
            }
            0x80263930 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80263940; continue; }
                pc = 0x80263938;
            }
            0x80263938 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8026399c; continue;
                pc = 0x80263940;
            }
            0x80263940 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80263950; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263950;
            }
            0x80263950 => { ins += 4;
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80263960; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263960;
            }
            0x80263960 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263970; continue; }
                pc = 0x80263968;
            }
            0x80263968 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8026399c; continue;
                pc = 0x80263970;
            }
            0x80263970 => { ins += 1;
                c.lr = 0x80263974; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263974;
            }
            0x80263974 => { ins += 1;
                c.lr = 0x80263978; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263978;
            }
            0x80263978 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80263988; continue; }
                pc = 0x80263980;
            }
            0x80263980 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8026399c; continue;
                pc = 0x80263988;
            }
            0x80263988 => { ins += 5;
                c.r[4] = c.r[24] | c.r[24];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x8026399c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026399c;
            }
            0x8026399c => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x802639a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011d9c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802639a4;
            }
            0x802639a4 => { ins += 1;
                c.r[26] = c.r[30] | c.r[30];
                pc = 0x802639a8;
            }
            0x802639a8 => { ins += 1;
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x802639ac;
            }
            0x802639ac => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[27]);
                if cr_bit(cr, 0) { pc = 0x8026329c; continue; }
                pc = 0x802639b8;
            }
            0x802639b8 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802639c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802639c0;
            }
            0x802639c0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80263b70; continue; }
                pc = 0x802639c8;
            }
            0x802639c8 => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.r[24] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x802639dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802639dc;
            }
            0x802639dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802639ec; continue; }
                pc = 0x802639e4;
            }
            0x802639e4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263a48; continue;
                pc = 0x802639ec;
            }
            0x802639ec => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x802639fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802639fc;
            }
            0x802639fc => { ins += 4;
                c.r[25] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80263a0c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263a0c;
            }
            0x80263a0c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263a1c; continue; }
                pc = 0x80263a14;
            }
            0x80263a14 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263a48; continue;
                pc = 0x80263a1c;
            }
            0x80263a1c => { ins += 1;
                c.lr = 0x80263a20; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263a20;
            }
            0x80263a20 => { ins += 1;
                c.lr = 0x80263a24; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263a24;
            }
            0x80263a24 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80263a34; continue; }
                pc = 0x80263a2c;
            }
            0x80263a2c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263a48; continue;
                pc = 0x80263a34;
            }
            0x80263a34 => { ins += 5;
                c.r[4] = c.r[25] | c.r[25];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80263a48; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263a48;
            }
            0x80263a48 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x80263a80; continue; }
                pc = 0x80263a50;
            }
            0x80263a50 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80263a64; continue; }
                pc = 0x80263a54;
            }
            0x80263a54 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x80263a70; continue; }
                pc = 0x80263a5c;
            }
            0x80263a5c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80263a78; continue; }
                pc = 0x80263a60;
            }
            0x80263a60 => { ins += 1;
                pc = 0x80263a8c; continue;
                pc = 0x80263a64;
            }
            0x80263a64 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x80263a8c; continue; }
                pc = 0x80263a6c;
            }
            0x80263a6c => { ins += 1;
                pc = 0x80263a88; continue;
                pc = 0x80263a70;
            }
            0x80263a70 => { ins += 2;
                c.r[24] = 0x100;
                pc = 0x80263a8c; continue;
                pc = 0x80263a78;
            }
            0x80263a78 => { ins += 2;
                c.r[24] = 0x101;
                pc = 0x80263a8c; continue;
                pc = 0x80263a80;
            }
            0x80263a80 => { ins += 2;
                c.r[24] = 0x102;
                pc = 0x80263a8c; continue;
                pc = 0x80263a88;
            }
            0x80263a88 => { ins += 1;
                c.r[24] = 0x103;
                pc = 0x80263a8c;
            }
            0x80263a8c => { ins += 2;
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x80263a94; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102620(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263a94;
            }
            0x80263a94 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80263b70; continue; }
                pc = 0x80263a9c;
            }
            0x80263a9c => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.r[24] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x80263ab0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263ab0;
            }
            0x80263ab0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80263ac0; continue; }
                pc = 0x80263ab8;
            }
            0x80263ab8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263b1c; continue;
                pc = 0x80263ac0;
            }
            0x80263ac0 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80263ad0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263ad0;
            }
            0x80263ad0 => { ins += 4;
                c.r[25] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80263ae0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263ae0;
            }
            0x80263ae0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263af0; continue; }
                pc = 0x80263ae8;
            }
            0x80263ae8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263b1c; continue;
                pc = 0x80263af0;
            }
            0x80263af0 => { ins += 1;
                c.lr = 0x80263af4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263af4;
            }
            0x80263af4 => { ins += 1;
                c.lr = 0x80263af8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263af8;
            }
            0x80263af8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80263b08; continue; }
                pc = 0x80263b00;
            }
            0x80263b00 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263b1c; continue;
                pc = 0x80263b08;
            }
            0x80263b08 => { ins += 5;
                c.r[4] = c.r[25] | c.r[25];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80263b1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263b1c;
            }
            0x80263b1c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x80263b54; continue; }
                pc = 0x80263b24;
            }
            0x80263b24 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80263b38; continue; }
                pc = 0x80263b28;
            }
            0x80263b28 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x80263b44; continue; }
                pc = 0x80263b30;
            }
            0x80263b30 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80263b4c; continue; }
                pc = 0x80263b34;
            }
            0x80263b34 => { ins += 1;
                pc = 0x80263b60; continue;
                pc = 0x80263b38;
            }
            0x80263b38 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x80263b60; continue; }
                pc = 0x80263b40;
            }
            0x80263b40 => { ins += 1;
                pc = 0x80263b5c; continue;
                pc = 0x80263b44;
            }
            0x80263b44 => { ins += 2;
                c.r[24] = 0x100;
                pc = 0x80263b60; continue;
                pc = 0x80263b4c;
            }
            0x80263b4c => { ins += 2;
                c.r[24] = 0x101;
                pc = 0x80263b60; continue;
                pc = 0x80263b54;
            }
            0x80263b54 => { ins += 2;
                c.r[24] = 0x102;
                pc = 0x80263b60; continue;
                pc = 0x80263b5c;
            }
            0x80263b5c => { ins += 1;
                c.r[24] = 0x103;
                pc = 0x80263b60;
            }
            0x80263b60 => { ins += 4;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.lr = 0x80263b70; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102568(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263b70;
            }
            0x80263b70 => { ins += 2;
                c.r[24] = 0x0;
                pc = 0x80263ba4; continue;
                pc = 0x80263b78;
            }
            0x80263b78 => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[24] | c.r[24];
                c.lr = 0x80263b84; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f981c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263b84;
            }
            0x80263b84 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80263ba0; continue; }
                pc = 0x80263b8c;
            }
            0x80263b8c => { ins += 5;
                c.r[4] = 0x0;
                c.r[5] = 0x120;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80263ba0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263ba0;
            }
            0x80263ba0 => { ins += 1;
                c.r[24] = c.r[24].wrapping_add(0x1);
                pc = 0x80263ba4;
            }
            0x80263ba4 => { ins += 3;
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[27]);
                if cr_bit(cr, 0) { pc = 0x80263b78; continue; }
                pc = 0x80263bb0;
            }
            0x80263bb0 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80263bb4;
            }
            0x80263bb4 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 24..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8026316c: bad pc {:#x}", pc),
        }
    }
}

/// _fightMenuFightTrainerGcHeroOpenMenuSubMain__FP13FIGHT_TRAINERP15FightOutPokemonUsl (0x80263bc8, 540 bytes)
#[inline(never)]
pub fn f_80263bc8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80263bc8) { return; }
        if c.prof { crate::prof::enter(c, 0x80263bc8); f_80263bc8_at(c, 0x80263bc8); crate::prof::exit(c, 0x80263bc8); return; }
    }
    f_80263bc8_b(c, 0x80263bc8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80263bc8_at(c: &mut Cpu, pc: u32) { f_80263bc8_b(c, pc) }
#[inline(never)]
fn f_80263bc8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80263bc8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80263bc8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80263bc8 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0x2);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[31] = 0x0;
                if cr_bit(cr, 2) { pc = 0x80263ce4; continue; }
                pc = 0x80263bf0;
            }
            0x80263bf0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80263c04; continue; }
                pc = 0x80263bf4;
            }
            0x80263bf4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0x0);
                if cr_bit(cr, 2) { pc = 0x80263c10; continue; }
                pc = 0x80263bfc;
            }
            0x80263bfc => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80263c80; continue; }
                pc = 0x80263c00;
            }
            0x80263c00 => { ins += 1;
                pc = 0x80263dc8; continue;
                pc = 0x80263c04;
            }
            0x80263c04 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0x4);
                if !cr_bit(cr, 0) { pc = 0x80263dc8; continue; }
                pc = 0x80263c0c;
            }
            0x80263c0c => { ins += 1;
                pc = 0x80263d38; continue;
                pc = 0x80263c10;
            }
            0x80263c10 => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x1;
                c.lr = 0x80263c1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ff1bc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263c1c;
            }
            0x80263c1c => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263c58; continue; }
                pc = 0x80263c2c;
            }
            0x80263c2c => { ins += 3;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x11;
                c.lr = 0x80263c38; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263c38;
            }
            0x80263c38 => { ins += 4;
                c.r[3] = 0x75fc;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x80263c48; { c.steps = st; c.cycles = ins; c.cr = cr; f_80106394(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263c48;
            }
            0x80263c48 => { ins += 2;
                c.r[3] = 0x40;
                c.lr = 0x80263c50; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f000c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263c50;
            }
            0x80263c50 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80263c58; { c.steps = st; c.cycles = ins; c.cr = cr; f_80106080(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263c58;
            }
            0x80263c58 => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263dcc; continue; }
                pc = 0x80263c60;
            }
            0x80263c60 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x80263c70; { c.steps = st; c.cycles = ins; c.cr = cr; f_80264d58(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263c70;
            }
            0x80263c70 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263dcc; continue; }
                pc = 0x80263c78;
            }
            0x80263c78 => { ins += 2;
                c.r[31] = 0x2;
                pc = 0x80263dcc; continue;
                pc = 0x80263c80;
            }
            0x80263c80 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x20;
                c.r[6] = 0x0;
                c.lr = 0x80263c94; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263c94;
            }
            0x80263c94 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263cc4; continue; }
                pc = 0x80263c9c;
            }
            0x80263c9c => { ins += 4;
                c.r[3] = 0x75f5;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x80263cac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80106394(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263cac;
            }
            0x80263cac => { ins += 2;
                c.r[3] = 0x40;
                c.lr = 0x80263cb4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f000c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263cb4;
            }
            0x80263cb4 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80263cbc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80106080(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263cbc;
            }
            0x80263cbc => { ins += 2;
                c.r[31] = 0x2;
                pc = 0x80263dcc; continue;
                pc = 0x80263cc4;
            }
            0x80263cc4 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x80263cd4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80264488(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263cd4;
            }
            0x80263cd4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263dcc; continue; }
                pc = 0x80263cdc;
            }
            0x80263cdc => { ins += 2;
                c.r[31] = 0x2;
                pc = 0x80263dcc; continue;
                pc = 0x80263ce4;
            }
            0x80263ce4 => { ins += 3;
                c.r[6] = 0x1;
                c.r[7] = 0x1;
                c.lr = 0x80263cf0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80263de4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263cf0;
            }
            0x80263cf0 => { ins += 3;
                c.r[5] = c.r[3] | c.r[3];
                c.r[0] = c.r[5] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x80263d04; continue; }
                pc = 0x80263cfc;
            }
            0x80263cfc => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80263d28; continue;
                pc = 0x80263d04;
            }
            0x80263d04 => { ins += 8;
                c.r[4] = 0x80370000;
                c.r[8] = c.r[5] as u16 as i16 as i32 as u32;
                c.r[7] = c.r[4].wrapping_add(0x5d30);
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.lr = 0x80263d24; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020505c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263d24;
            }
            0x80263d24 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x80263d28;
            }
            0x80263d28 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263dcc; continue; }
                pc = 0x80263d30;
            }
            0x80263d30 => { ins += 2;
                c.r[31] = 0x2;
                pc = 0x80263dcc; continue;
                pc = 0x80263d38;
            }
            0x80263d38 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x22;
                c.r[6] = 0x0;
                c.lr = 0x80263d4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263d4c;
            }
            0x80263d4c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263d7c; continue; }
                pc = 0x80263d58;
            }
            0x80263d58 => { ins += 8;
                c.r[4] = 0x80370000;
                c.r[3] = c.r[28] | c.r[28];
                c.r[7] = c.r[4].wrapping_add(0x5d30);
                c.r[5] = 0x8;
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x80263d78; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020505c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263d78;
            }
            0x80263d78 => { ins += 1;
                pc = 0x80263dcc; continue;
                pc = 0x80263d7c;
            }
            0x80263d7c => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x21;
                c.r[6] = 0x0;
                c.lr = 0x80263d90; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263d90;
            }
            0x80263d90 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263dc0; continue; }
                pc = 0x80263d9c;
            }
            0x80263d9c => { ins += 8;
                c.r[4] = 0x80370000;
                c.r[3] = c.r[28] | c.r[28];
                c.r[7] = c.r[4].wrapping_add(0x5d30);
                c.r[5] = 0xa;
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x80263dbc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020505c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263dbc;
            }
            0x80263dbc => { ins += 1;
                pc = 0x80263dcc; continue;
                pc = 0x80263dc0;
            }
            0x80263dc0 => { ins += 2;
                c.r[31] = 0x2;
                pc = 0x80263dcc; continue;
                pc = 0x80263dc8;
            }
            0x80263dc8 => { ins += 1;
                c.r[31] = 0x1;
                pc = 0x80263dcc;
            }
            0x80263dcc => { ins += 17;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80263bc8: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuFightTrainerGcHeroSelectIrekaeFightPokemon (0x80263de4, 1700 bytes)
#[inline(never)]
pub fn f_80263de4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80263de4) { return; }
        if c.prof { crate::prof::enter(c, 0x80263de4); f_80263de4_at(c, 0x80263de4); crate::prof::exit(c, 0x80263de4); return; }
    }
    f_80263de4_b(c, 0x80263de4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80263de4_at(c: &mut Cpu, pc: u32) { f_80263de4_b(c, pc) }
#[inline(never)]
fn f_80263de4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80263de4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80263de4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80263de4 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[31] = c.r[5] | c.r[5];
                c.r[26] = c.r[6] | c.r[6];
                c.r[27] = c.r[7] | c.r[7];
                pc = 0x80263e08;
            }
            0x80263e08 => { ins += 1;
                c.lr = 0x80263e0c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263e0c;
            }
            0x80263e0c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263e20; continue; }
                pc = 0x80263e18;
            }
            0x80263e18 => { ins += 2;
                c.r[3] = 0xfffffffe;
                pc = 0x80264474; continue;
                pc = 0x80263e20;
            }
            0x80263e20 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80263e28; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263e28;
            }
            0x80263e28 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263e44; continue; }
                pc = 0x80263e34;
            }
            0x80263e34 => { ins += 1;
                c.lr = 0x80263e38; { c.steps = st; c.cycles = ins; c.cr = cr; f_80265924(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263e38;
            }
            0x80263e38 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802643e4; continue; }
                pc = 0x80263e44;
            }
            0x80263e44 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80263e4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263e4c;
            }
            0x80263e4c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80263f40; continue; }
                pc = 0x80263e54;
            }
            0x80263e54 => { ins += 2;
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263f40; continue; }
                pc = 0x80263e5c;
            }
            0x80263e5c => { ins += 5;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[25] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x80263e70; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263e70;
            }
            0x80263e70 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80263e80; continue; }
                pc = 0x80263e78;
            }
            0x80263e78 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263edc; continue;
                pc = 0x80263e80;
            }
            0x80263e80 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80263e90; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263e90;
            }
            0x80263e90 => { ins += 4;
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80263ea0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263ea0;
            }
            0x80263ea0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80263eb0; continue; }
                pc = 0x80263ea8;
            }
            0x80263ea8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263edc; continue;
                pc = 0x80263eb0;
            }
            0x80263eb0 => { ins += 1;
                c.lr = 0x80263eb4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263eb4;
            }
            0x80263eb4 => { ins += 1;
                c.lr = 0x80263eb8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263eb8;
            }
            0x80263eb8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80263ec8; continue; }
                pc = 0x80263ec0;
            }
            0x80263ec0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80263edc; continue;
                pc = 0x80263ec8;
            }
            0x80263ec8 => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80263edc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263edc;
            }
            0x80263edc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x80263f14; continue; }
                pc = 0x80263ee4;
            }
            0x80263ee4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80263ef8; continue; }
                pc = 0x80263ee8;
            }
            0x80263ee8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x80263f04; continue; }
                pc = 0x80263ef0;
            }
            0x80263ef0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80263f0c; continue; }
                pc = 0x80263ef4;
            }
            0x80263ef4 => { ins += 1;
                pc = 0x80263f20; continue;
                pc = 0x80263ef8;
            }
            0x80263ef8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x80263f20; continue; }
                pc = 0x80263f00;
            }
            0x80263f00 => { ins += 1;
                pc = 0x80263f1c; continue;
                pc = 0x80263f04;
            }
            0x80263f04 => { ins += 2;
                c.r[25] = 0x100;
                pc = 0x80263f20; continue;
                pc = 0x80263f0c;
            }
            0x80263f0c => { ins += 2;
                c.r[25] = 0x101;
                pc = 0x80263f20; continue;
                pc = 0x80263f14;
            }
            0x80263f14 => { ins += 2;
                c.r[25] = 0x102;
                pc = 0x80263f20; continue;
                pc = 0x80263f1c;
            }
            0x80263f1c => { ins += 1;
                c.r[25] = 0x103;
                pc = 0x80263f20;
            }
            0x80263f20 => { ins += 8;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80263f40; { c.steps = st; c.cycles = ins; c.cr = cr; f_801026a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263f40;
            }
            0x80263f40 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80263f48; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263f48;
            }
            0x80263f48 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80264098; continue; }
                pc = 0x80263f50;
            }
            0x80263f50 => { ins += 6;
                c.r[4] = 0x80260000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x1cbc);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80263f68; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f2b5c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263f68;
            }
            0x80263f68 => { ins += 6;
                c.r[4] = 0x80260000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x1fb4);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80263f80; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263f80;
            }
            0x80263f80 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80263f88; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263f88;
            }
            0x80263f88 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263f98; continue; }
                pc = 0x80263f94;
            }
            0x80263f94 => { ins += 1;
                c.lr = 0x80263f98; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000ddbc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263f98;
            }
            0x80263f98 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80263fa0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1758(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263fa0;
            }
            0x80263fa0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80263fb0; continue; }
                pc = 0x80263fac;
            }
            0x80263fac => { ins += 1;
                c.lr = 0x80263fb0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000dd30(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263fb0;
            }
            0x80263fb0 => { ins += 6;
                c.r[4] = 0x80260000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x1cbc);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80263fc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f2b5c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263fc8;
            }
            0x80263fc8 => { ins += 3;
                c.r[3] = 0x80260000;
                c.r[25] = 0x1;
                c.r[28] = c.r[3].wrapping_add(0x1bec);
                pc = 0x80263fd4;
            }
            0x80263fd4 => { ins += 6;
                c.stb(c.r[1].wrapping_add(0x9), c.r[25]);
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[1].wrapping_add(0x9);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80263fec; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f2b5c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263fec;
            }
            0x80263fec => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[1].wrapping_add(0x9));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80264000; continue; }
                pc = 0x80263ff8;
            }
            0x80263ff8 => { ins += 1;
                c.lr = 0x80263ffc; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80263ffc;
            }
            0x80263ffc => { ins += 1;
                pc = 0x80263fd4; continue;
                pc = 0x80264000;
            }
            0x80264000 => { ins += 6;
                c.r[4] = 0x80260000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x1fb4);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80264018; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264018;
            }
            0x80264018 => { ins += 2;
                c.r[3] = 0x80260000;
                c.r[28] = c.r[3].wrapping_add(0x1ef8);
                pc = 0x80264020;
            }
            0x80264020 => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80264034; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264034;
            }
            0x80264034 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80264048; continue; }
                pc = 0x80264040;
            }
            0x80264040 => { ins += 1;
                c.lr = 0x80264044; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264044;
            }
            0x80264044 => { ins += 1;
                pc = 0x80264020; continue;
                pc = 0x80264048;
            }
            0x80264048 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80264050; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264050;
            }
            0x80264050 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264070; continue; }
                pc = 0x8026405c;
            }
            0x8026405c => { ins += 1;
                c.lr = 0x80264060; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000dd98(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264060;
            }
            0x80264060 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80264070; continue; }
                pc = 0x80264068;
            }
            0x80264068 => { ins += 1;
                c.lr = 0x8026406c; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026406c;
            }
            0x8026406c => { ins += 1;
                pc = 0x8026405c; continue;
                pc = 0x80264070;
            }
            0x80264070 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80264078; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1758(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264078;
            }
            0x80264078 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264098; continue; }
                pc = 0x80264084;
            }
            0x80264084 => { ins += 1;
                c.lr = 0x80264088; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000dd0c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264088;
            }
            0x80264088 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80264098; continue; }
                pc = 0x80264090;
            }
            0x80264090 => { ins += 1;
                c.lr = 0x80264094; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264094;
            }
            0x80264094 => { ins += 1;
                pc = 0x80264084; continue;
                pc = 0x80264098;
            }
            0x80264098 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802640a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802640a0;
            }
            0x802640a0 => { ins += 6;
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.r[6] = c.r[26] | c.r[26];
                c.lr = 0x802640b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_800114a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802640b8;
            }
            0x802640b8 => { ins += 3;
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = 0xa;
                c.lr = 0x802640c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801effc4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802640c4;
            }
            0x802640c4 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802640cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802640cc;
            }
            0x802640cc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802641e8; continue; }
                pc = 0x802640d4;
            }
            0x802640d4 => { ins += 3;
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802641e8; continue; }
                pc = 0x802640e0;
            }
            0x802640e0 => { ins += 6;
                c.r[4] = 0x80260000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x1d8c);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802640f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f2b5c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802640f8;
            }
            0x802640f8 => { ins += 8;
                c.r[0] = 0x1;
                c.r[3] = 0x80260000;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x2084);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80264118; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264118;
            }
            0x80264118 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80264120; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264120;
            }
            0x80264120 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264130; continue; }
                pc = 0x8026412c;
            }
            0x8026412c => { ins += 1;
                c.lr = 0x80264130; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000dde8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264130;
            }
            0x80264130 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80264138; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1758(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264138;
            }
            0x80264138 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264148; continue; }
                pc = 0x80264144;
            }
            0x80264144 => { ins += 1;
                c.lr = 0x80264148; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000dd5c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264148;
            }
            0x80264148 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if cr_bit(cr, 2) { pc = 0x802641e8; continue; }
                pc = 0x80264150;
            }
            0x80264150 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80264158; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264158;
            }
            0x80264158 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802641e8; continue; }
                pc = 0x80264164;
            }
            0x80264164 => { ins += 4;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x80264174; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264174;
            }
            0x80264174 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80264184; continue; }
                pc = 0x8026417c;
            }
            0x8026417c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802641e0; continue;
                pc = 0x80264184;
            }
            0x80264184 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80264194; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264194;
            }
            0x80264194 => { ins += 4;
                c.r[25] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x802641a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802641a4;
            }
            0x802641a4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802641b4; continue; }
                pc = 0x802641ac;
            }
            0x802641ac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802641e0; continue;
                pc = 0x802641b4;
            }
            0x802641b4 => { ins += 1;
                c.lr = 0x802641b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802641b8;
            }
            0x802641b8 => { ins += 1;
                c.lr = 0x802641bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802641bc;
            }
            0x802641bc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x802641cc; continue; }
                pc = 0x802641c4;
            }
            0x802641c4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802641e0; continue;
                pc = 0x802641cc;
            }
            0x802641cc => { ins += 5;
                c.r[4] = c.r[25] | c.r[25];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x802641e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802641e0;
            }
            0x802641e0 => { ins += 2;
                c.r[4] = 0x1;
                c.lr = 0x802641e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011d9c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802641e8;
            }
            0x802641e8 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802641f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802641f0;
            }
            0x802641f0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x802643a8; continue; }
                pc = 0x802641f8;
            }
            0x802641f8 => { ins += 2;
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802643a8; continue; }
                pc = 0x80264200;
            }
            0x80264200 => { ins += 5;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[25] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x80264214; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264214;
            }
            0x80264214 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80264224; continue; }
                pc = 0x8026421c;
            }
            0x8026421c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264280; continue;
                pc = 0x80264224;
            }
            0x80264224 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80264234; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264234;
            }
            0x80264234 => { ins += 4;
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80264244; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264244;
            }
            0x80264244 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80264254; continue; }
                pc = 0x8026424c;
            }
            0x8026424c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264280; continue;
                pc = 0x80264254;
            }
            0x80264254 => { ins += 1;
                c.lr = 0x80264258; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264258;
            }
            0x80264258 => { ins += 1;
                c.lr = 0x8026425c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026425c;
            }
            0x8026425c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x8026426c; continue; }
                pc = 0x80264264;
            }
            0x80264264 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264280; continue;
                pc = 0x8026426c;
            }
            0x8026426c => { ins += 5;
                c.r[4] = c.r[24] | c.r[24];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80264280; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264280;
            }
            0x80264280 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x802642b8; continue; }
                pc = 0x80264288;
            }
            0x80264288 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8026429c; continue; }
                pc = 0x8026428c;
            }
            0x8026428c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x802642a8; continue; }
                pc = 0x80264294;
            }
            0x80264294 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x802642b0; continue; }
                pc = 0x80264298;
            }
            0x80264298 => { ins += 1;
                pc = 0x802642c4; continue;
                pc = 0x8026429c;
            }
            0x8026429c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x802642c4; continue; }
                pc = 0x802642a4;
            }
            0x802642a4 => { ins += 1;
                pc = 0x802642c0; continue;
                pc = 0x802642a8;
            }
            0x802642a8 => { ins += 2;
                c.r[25] = 0x100;
                pc = 0x802642c4; continue;
                pc = 0x802642b0;
            }
            0x802642b0 => { ins += 2;
                c.r[25] = 0x101;
                pc = 0x802642c4; continue;
                pc = 0x802642b8;
            }
            0x802642b8 => { ins += 2;
                c.r[25] = 0x102;
                pc = 0x802642c4; continue;
                pc = 0x802642c0;
            }
            0x802642c0 => { ins += 1;
                c.r[25] = 0x103;
                pc = 0x802642c4;
            }
            0x802642c4 => { ins += 2;
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x802642cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102620(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802642cc;
            }
            0x802642cc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x802643a8; continue; }
                pc = 0x802642d4;
            }
            0x802642d4 => { ins += 5;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[25] = 0x100;
                c.r[3] = 0x2;
                c.lr = 0x802642e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802642e8;
            }
            0x802642e8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802642f8; continue; }
                pc = 0x802642f0;
            }
            0x802642f0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264354; continue;
                pc = 0x802642f8;
            }
            0x802642f8 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80264308; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264308;
            }
            0x80264308 => { ins += 4;
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80264318; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264318;
            }
            0x80264318 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80264328; continue; }
                pc = 0x80264320;
            }
            0x80264320 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264354; continue;
                pc = 0x80264328;
            }
            0x80264328 => { ins += 1;
                c.lr = 0x8026432c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026432c;
            }
            0x8026432c => { ins += 1;
                c.lr = 0x80264330; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264330;
            }
            0x80264330 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80264340; continue; }
                pc = 0x80264338;
            }
            0x80264338 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264354; continue;
                pc = 0x80264340;
            }
            0x80264340 => { ins += 5;
                c.r[4] = c.r[24] | c.r[24];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80264354; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264354;
            }
            0x80264354 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf3);
                if cr_bit(cr, 2) { pc = 0x8026438c; continue; }
                pc = 0x8026435c;
            }
            0x8026435c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80264370; continue; }
                pc = 0x80264360;
            }
            0x80264360 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf1);
                if cr_bit(cr, 2) { pc = 0x8026437c; continue; }
                pc = 0x80264368;
            }
            0x80264368 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x80264384; continue; }
                pc = 0x8026436c;
            }
            0x8026436c => { ins += 1;
                pc = 0x80264398; continue;
                pc = 0x80264370;
            }
            0x80264370 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xf5);
                if !cr_bit(cr, 0) { pc = 0x80264398; continue; }
                pc = 0x80264378;
            }
            0x80264378 => { ins += 1;
                pc = 0x80264394; continue;
                pc = 0x8026437c;
            }
            0x8026437c => { ins += 2;
                c.r[25] = 0x100;
                pc = 0x80264398; continue;
                pc = 0x80264384;
            }
            0x80264384 => { ins += 2;
                c.r[25] = 0x101;
                pc = 0x80264398; continue;
                pc = 0x8026438c;
            }
            0x8026438c => { ins += 2;
                c.r[25] = 0x102;
                pc = 0x80264398; continue;
                pc = 0x80264394;
            }
            0x80264394 => { ins += 1;
                c.r[25] = 0x103;
                pc = 0x80264398;
            }
            0x80264398 => { ins += 4;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.lr = 0x802643a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102568(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802643a8;
            }
            0x802643a8 => { ins += 1;
                c.lr = 0x802643ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802643ac;
            }
            0x802643ac => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80263e18; continue; }
                pc = 0x802643b8;
            }
            0x802643b8 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802643c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802643c0;
            }
            0x802643c0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264400; continue; }
                pc = 0x802643cc;
            }
            0x802643cc => { ins += 1;
                c.lr = 0x802643d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80265924(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802643d0;
            }
            0x802643d0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264400; continue; }
                pc = 0x802643dc;
            }
            0x802643dc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80264400; continue; }
                pc = 0x802643e4;
            }
            0x802643e4 => { ins += 2;
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80264408; continue; }
                pc = 0x802643ec;
            }
            0x802643ec => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x802643fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f93f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802643fc;
            }
            0x802643fc => { ins += 1;
                pc = 0x80264474; continue;
                pc = 0x80264400;
            }
            0x80264400 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80264418; continue; }
                pc = 0x80264408;
            }
            0x80264408 => { ins += 2;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80263e08; continue; }
                pc = 0x80264410;
            }
            0x80264410 => { ins += 2;
                c.r[3] = 0xffffffff;
                pc = 0x80264474; continue;
                pc = 0x80264418;
            }
            0x80264418 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[28].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x8026442c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026442c;
            }
            0x8026442c => { ins += 2;
                c.r[24] = c.r[3] | c.r[3];
                c.lr = 0x80264434; { c.steps = st; c.cycles = ins; c.cr = cr; f_80206a04(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264434;
            }
            0x80264434 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80263e08; continue; }
                pc = 0x8026443c;
            }
            0x8026443c => { ins += 5;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0xd2;
                c.r[6] = 0x0;
                c.lr = 0x80264450; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264450;
            }
            0x80264450 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80263e08; continue; }
                pc = 0x8026445c;
            }
            0x8026445c => { ins += 5;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0xce;
                c.r[6] = 0x0;
                c.lr = 0x80264470; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264470;
            }
            0x80264470 => { ins += 1;
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x80264474;
            }
            0x80264474 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80263de4: bad pc {:#x}", pc),
        }
    }
}

/// _fightMenuFightTrainerGcHeroOpenMenuSubItem__FP13FIGHT_TRAINERP15FightOutPokemonUs (0x80264488, 1620 bytes)
#[inline(never)]
pub fn f_80264488(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80264488) { return; }
        if c.prof { crate::prof::enter(c, 0x80264488); f_80264488_at(c, 0x80264488); crate::prof::exit(c, 0x80264488); return; }
    }
    f_80264488_b(c, 0x80264488);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80264488_at(c: &mut Cpu, pc: u32) { f_80264488_b(c, pc) }
#[inline(never)]
fn f_80264488_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80264488;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80264488, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80264488 => { ins += 28;
                { let ea = c.r[1].wrapping_add(0xffffff70); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x94), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x80); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x70); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 16..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[6] = 0x0;
                c.r[28] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x20), c.r[6]);
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[5] = c.r[1].wrapping_add(0x1c);
                c.r[4] = c.r[1].wrapping_add(0x18);
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.r[7] = 0x0;
                pc = 0x802644e4; continue;
                pc = 0x802644d0;
            }
            0x802644d0 => { ins += 5;
                c.r[0] = c.r[7].rotate_left(0) & 0xffff;
                c.r[7] = c.r[7].wrapping_add(0x1);
                c.stb(c.r[5].wrapping_add(c.r[0]), c.r[6]);
                c.stb(c.r[4].wrapping_add(c.r[0]), c.r[6]);
                c.stb(c.r[3].wrapping_add(c.r[0]), c.r[6]);
                pc = 0x802644e4;
            }
            0x802644e4 => { ins += 3;
                c.r[0] = c.r[7].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x802644d0; continue; }
                pc = 0x802644f0;
            }
            0x802644f0 => { ins += 2;
                c.r[17] = 0x0;
                pc = 0x80264574; continue;
                pc = 0x802644f8;
            }
            0x802644f8 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[17] | c.r[17];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x8026450c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026450c;
            }
            0x8026450c => { ins += 2;
                c.r[18] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[18]);
                if cr_bit(cr, 2) { pc = 0x80264570; continue; }
                pc = 0x80264514;
            }
            0x80264514 => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[18] | c.r[18];
                c.lr = 0x80264520; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f85b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264520;
            }
            0x80264520 => { ins += 6;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[0].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x80264538; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264538;
            }
            0x80264538 => { ins += 1;
                c.lr = 0x8026453c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026453c;
            }
            0x8026453c => { ins += 6;
                c.r[16] = c.r[17].rotate_left(0) & 0xffff;
                c.r[4] = c.r[1].wrapping_add(0x1c);
                c.stb(c.r[4].wrapping_add(c.r[16]), c.r[3]);
                c.r[3] = c.r[18] | c.r[18];
                c.r[4] = 0x8;
                c.lr = 0x80264554; { c.steps = st; c.cycles = ins; c.cr = cr; f_802026e4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264554;
            }
            0x80264554 => { ins += 5;
                c.r[5] = c.r[1].wrapping_add(0x18);
                c.r[4] = 0x7;
                c.stb(c.r[5].wrapping_add(c.r[16]), c.r[3]);
                c.r[3] = c.r[18] | c.r[18];
                c.lr = 0x80264568; { c.steps = st; c.cycles = ins; c.cr = cr; f_802026e4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264568;
            }
            0x80264568 => { ins += 2;
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.stb(c.r[4].wrapping_add(c.r[16]), c.r[3]);
                pc = 0x80264570;
            }
            0x80264570 => { ins += 1;
                c.r[17] = c.r[17].wrapping_add(0x1);
                pc = 0x80264574;
            }
            0x80264574 => { ins += 3;
                c.r[0] = c.r[17].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x802644f8; continue; }
                pc = 0x80264580;
            }
            0x80264580 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x80264594; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264594;
            }
            0x80264594 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802645a4; continue; }
                pc = 0x8026459c;
            }
            0x8026459c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264ab8; continue;
                pc = 0x802645a4;
            }
            0x802645a4 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xd6;
                c.r[6] = 0x0;
                c.lr = 0x802645b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802645b8;
            }
            0x802645b8 => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[0] | c.r[0];
                c.lr = 0x802645c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8638(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802645c8;
            }
            0x802645c8 => { ins += 17;
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[4] = 0x80260000;
                c.r[3] = 0x80260000;
                c.r[6] = 0x80260000;
                c.r[23] = c.r[4].wrapping_add(0x1fb4);
                c.r[5] = 0x80260000;
                c.r[22] = c.r[3].wrapping_add(0x1cbc);
                c.r[4] = 0x80260000;
                c.r[3] = 0x80260000;
                c.stw(c.r[1].wrapping_add(0x20), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffffb030); c.f[30] = c.lfd(ea); }
                c.r[24] = c.r[6].wrapping_add(0x1bec);
                { let ea = c.r[2].wrapping_add(0xffffb02c); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                c.r[25] = c.r[5].wrapping_add(0x1ef8);
                c.r[26] = c.r[4].wrapping_add(0x1d8c);
                c.r[27] = c.r[3].wrapping_add(0x2084);
                c.r[16] = 0x43300000;
                pc = 0x8026460c;
            }
            0x8026460c => { ins += 5;
                c.r[4] = c.r[22] | c.r[22];
                c.r[3] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80264620; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f2b5c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264620;
            }
            0x80264620 => { ins += 5;
                c.r[4] = c.r[23] | c.r[23];
                c.r[3] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80264634; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264634;
            }
            0x80264634 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8026463c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026463c;
            }
            0x8026463c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8026464c; continue; }
                pc = 0x80264648;
            }
            0x80264648 => { ins += 1;
                c.lr = 0x8026464c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000ddbc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026464c;
            }
            0x8026464c => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80264654; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1758(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264654;
            }
            0x80264654 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264664; continue; }
                pc = 0x80264660;
            }
            0x80264660 => { ins += 1;
                c.lr = 0x80264664; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000dd30(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264664;
            }
            0x80264664 => { ins += 5;
                c.r[4] = c.r[22] | c.r[22];
                c.r[3] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80264678; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f2b5c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264678;
            }
            0x80264678 => { ins += 1;
                c.r[17] = 0x1;
                pc = 0x8026467c;
            }
            0x8026467c => { ins += 6;
                c.stb(c.r[1].wrapping_add(0x11), c.r[17]);
                c.r[4] = c.r[24] | c.r[24];
                c.r[5] = c.r[1].wrapping_add(0x11);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80264694; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f2b5c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264694;
            }
            0x80264694 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[1].wrapping_add(0x11));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802646a8; continue; }
                pc = 0x802646a0;
            }
            0x802646a0 => { ins += 1;
                c.lr = 0x802646a4; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802646a4;
            }
            0x802646a4 => { ins += 1;
                pc = 0x8026467c; continue;
                pc = 0x802646a8;
            }
            0x802646a8 => { ins += 5;
                c.r[4] = c.r[23] | c.r[23];
                c.r[3] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802646bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802646bc;
            }
            0x802646bc => { ins += 5;
                c.r[4] = c.r[25] | c.r[25];
                c.r[3] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802646d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802646d0;
            }
            0x802646d0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x802646e4; continue; }
                pc = 0x802646dc;
            }
            0x802646dc => { ins += 1;
                c.lr = 0x802646e0; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802646e0;
            }
            0x802646e0 => { ins += 1;
                pc = 0x802646bc; continue;
                pc = 0x802646e4;
            }
            0x802646e4 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802646ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802646ec;
            }
            0x802646ec => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8026470c; continue; }
                pc = 0x802646f8;
            }
            0x802646f8 => { ins += 1;
                c.lr = 0x802646fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000dd98(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802646fc;
            }
            0x802646fc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8026470c; continue; }
                pc = 0x80264704;
            }
            0x80264704 => { ins += 1;
                c.lr = 0x80264708; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264708;
            }
            0x80264708 => { ins += 1;
                pc = 0x802646f8; continue;
                pc = 0x8026470c;
            }
            0x8026470c => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80264714; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1758(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264714;
            }
            0x80264714 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264734; continue; }
                pc = 0x80264720;
            }
            0x80264720 => { ins += 1;
                c.lr = 0x80264724; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000dd0c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264724;
            }
            0x80264724 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80264734; continue; }
                pc = 0x8026472c;
            }
            0x8026472c => { ins += 1;
                c.lr = 0x80264730; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264730;
            }
            0x80264730 => { ins += 1;
                pc = 0x80264720; continue;
                pc = 0x80264734;
            }
            0x80264734 => { ins += 1;
                c.lr = 0x80264738; { c.steps = st; c.cycles = ins; c.cr = cr; f_800d37cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264738;
            }
            0x80264738 => { ins += 23;
                c.r[0] = c.r[3] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0x28), c.r[16]);
                c.stw(c.r[1].wrapping_add(0x2c), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x28); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[30]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[31] / c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x80264754; { c.steps = st; c.cycles = ins; c.cr = cr; f_8010206c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264754;
            }
            0x80264754 => { ins += 4;
                c.r[5] = c.r[28] | c.r[28];
                c.r[4] = c.r[1].wrapping_add(0x20);
                c.r[3] = 0x1;
                c.lr = 0x80264764; { c.steps = st; c.cycles = ins; c.cr = cr; f_80018f88(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264764;
            }
            0x80264764 => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0xa;
                c.r[31] = c.r[0] | c.r[0];
                c.lr = 0x80264774; { c.steps = st; c.cycles = ins; c.cr = cr; f_801effc4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264774;
            }
            0x80264774 => { ins += 1;
                c.lr = 0x80264778; { c.steps = st; c.cycles = ins; c.cr = cr; f_80019064(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264778;
            }
            0x80264778 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[21] = c.r[3].rotate_left(0) & 0xff;
                if cr_bit(cr, 2) { pc = 0x80264854; continue; }
                pc = 0x80264784;
            }
            0x80264784 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80264798; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264798;
            }
            0x80264798 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x80264854; continue; }
                pc = 0x802647a4;
            }
            0x802647a4 => { ins += 2;
                c.r[19] = 0x0;
                pc = 0x80264848; continue;
                pc = 0x802647ac;
            }
            0x802647ac => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[19] | c.r[19];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x802647c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802647c0;
            }
            0x802647c0 => { ins += 2;
                c.r[18] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[18]);
                if cr_bit(cr, 2) { pc = 0x80264844; continue; }
                pc = 0x802647c8;
            }
            0x802647c8 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0xee;
                c.r[6] = 0x0;
                c.lr = 0x802647d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802647d8;
            }
            0x802647d8 => { ins += 2;
                c.r[17] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[17]);
                if cr_bit(cr, 2) { pc = 0x80264844; continue; }
                pc = 0x802647e0;
            }
            0x802647e0 => { ins += 5;
                c.r[20] = c.r[19].rotate_left(0) & 0xffff;
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[20]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264814; continue; }
                pc = 0x802647f4;
            }
            0x802647f4 => { ins += 3;
                c.r[3] = c.r[18] | c.r[18];
                c.r[4] = 0x8;
                c.lr = 0x80264800; { c.steps = st; c.cycles = ins; c.cr = cr; f_802026e4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264800;
            }
            0x80264800 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80264814; continue; }
                pc = 0x80264808;
            }
            0x80264808 => { ins += 3;
                c.r[3] = c.r[17] | c.r[17];
                c.r[4] = 0x1;
                c.lr = 0x80264814; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da36c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264814;
            }
            0x80264814 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[20]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264844; continue; }
                pc = 0x80264824;
            }
            0x80264824 => { ins += 3;
                c.r[3] = c.r[18] | c.r[18];
                c.r[4] = 0x7;
                c.lr = 0x80264830; { c.steps = st; c.cycles = ins; c.cr = cr; f_802026e4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264830;
            }
            0x80264830 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80264844; continue; }
                pc = 0x80264838;
            }
            0x80264838 => { ins += 3;
                c.r[3] = c.r[17] | c.r[17];
                c.r[4] = 0x2;
                c.lr = 0x80264844; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da36c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264844;
            }
            0x80264844 => { ins += 1;
                c.r[19] = c.r[19].wrapping_add(0x1);
                pc = 0x80264848;
            }
            0x80264848 => { ins += 3;
                c.r[0] = c.r[19].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x802647ac; continue; }
                pc = 0x80264854;
            }
            0x80264854 => { ins += 5;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80264868; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f2b5c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264868;
            }
            0x80264868 => { ins += 7;
                c.r[0] = 0x1;
                c.r[4] = c.r[27] | c.r[27];
                c.stb(c.r[1].wrapping_add(0x10), c.r[0]);
                c.r[5] = c.r[1].wrapping_add(0x10);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80264884; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264884;
            }
            0x80264884 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8026488c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026488c;
            }
            0x8026488c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8026489c; continue; }
                pc = 0x80264898;
            }
            0x80264898 => { ins += 1;
                c.lr = 0x8026489c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000dde8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026489c;
            }
            0x8026489c => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802648a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1758(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802648a4;
            }
            0x802648a4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802648b4; continue; }
                pc = 0x802648b0;
            }
            0x802648b0 => { ins += 1;
                c.lr = 0x802648b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000dd5c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802648b4;
            }
            0x802648b4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if cr_bit(cr, 2) { pc = 0x80264954; continue; }
                pc = 0x802648bc;
            }
            0x802648bc => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x802648c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802648c4;
            }
            0x802648c4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80264954; continue; }
                pc = 0x802648d0;
            }
            0x802648d0 => { ins += 4;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.r[3] = 0x2;
                c.lr = 0x802648e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802648e0;
            }
            0x802648e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802648f0; continue; }
                pc = 0x802648e8;
            }
            0x802648e8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8026494c; continue;
                pc = 0x802648f0;
            }
            0x802648f0 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80264900; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264900;
            }
            0x80264900 => { ins += 4;
                c.r[17] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80264910; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264910;
            }
            0x80264910 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80264920; continue; }
                pc = 0x80264918;
            }
            0x80264918 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8026494c; continue;
                pc = 0x80264920;
            }
            0x80264920 => { ins += 1;
                c.lr = 0x80264924; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264924;
            }
            0x80264924 => { ins += 1;
                c.lr = 0x80264928; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264928;
            }
            0x80264928 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80264938; continue; }
                pc = 0x80264930;
            }
            0x80264930 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8026494c; continue;
                pc = 0x80264938;
            }
            0x80264938 => { ins += 5;
                c.r[4] = c.r[17] | c.r[17];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x8026494c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026494c;
            }
            0x8026494c => { ins += 2;
                c.r[4] = 0x1;
                c.lr = 0x80264954; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011d9c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264954;
            }
            0x80264954 => { ins += 1;
                c.lr = 0x80264958; { c.steps = st; c.cycles = ins; c.cr = cr; f_800d37cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264958;
            }
            0x80264958 => { ins += 23;
                c.r[0] = c.r[3] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0x28), c.r[16]);
                c.stw(c.r[1].wrapping_add(0x2c), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x28); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[30]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[31] / c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x80264974; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102038(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264974;
            }
            0x80264974 => { ins += 2;
                c.r[3] = 0xa;
                c.lr = 0x8026497c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801effc4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026497c;
            }
            0x8026497c => { ins += 2;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8026498c; continue; }
                pc = 0x80264984;
            }
            0x80264984 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264ab8; continue;
                pc = 0x8026498c;
            }
            0x8026498c => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x802649a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802649a0;
            }
            0x802649a0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802649ec; continue; }
                pc = 0x802649ac;
            }
            0x802649ac => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x802649b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7ef0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802649b4;
            }
            0x802649b4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8026460c; continue; }
                pc = 0x802649c0;
            }
            0x802649c0 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x802649d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80264adc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802649d0;
            }
            0x802649d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8026460c; continue; }
                pc = 0x802649d8;
            }
            0x802649d8 => { ins += 2;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x802649e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802649e0;
            }
            0x802649e0 => { ins += 3;
                c.r[4] = 0x0;
                c.r[9] = c.r[3] | c.r[3];
                pc = 0x80264a8c; continue;
                pc = 0x802649ec;
            }
            0x802649ec => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80264a00; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264a00;
            }
            0x80264a00 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x80264a80; continue; }
                pc = 0x80264a0c;
            }
            0x80264a0c => { ins += 1;
                c.lr = 0x80264a10; { c.steps = st; c.cycles = ins; c.cr = cr; f_8022ff90(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264a10;
            }
            0x80264a10 => { ins += 2;
                c.r[16] = 0x0;
                pc = 0x80264a74; continue;
                pc = 0x80264a18;
            }
            0x80264a18 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[16] | c.r[16];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x80264a2c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264a2c;
            }
            0x80264a2c => { ins += 2;
                c.r[17] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[17]);
                if cr_bit(cr, 2) { pc = 0x80264a70; continue; }
                pc = 0x80264a34;
            }
            0x80264a34 => { ins += 5;
                c.r[0] = c.r[16].rotate_left(0) & 0xffff;
                c.r[4] = c.r[1].wrapping_add(0x1c);
                c.r[0] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[0]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80264a70; continue; }
                pc = 0x80264a48;
            }
            0x80264a48 => { ins += 1;
                c.lr = 0x80264a4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264a4c;
            }
            0x80264a4c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264a70; continue; }
                pc = 0x80264a58;
            }
            0x80264a58 => { ins += 6;
                c.r[3] = c.r[17] | c.r[17];
                c.r[4] = 0x0;
                c.r[5] = 0x120;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80264a70; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264a70;
            }
            0x80264a70 => { ins += 1;
                c.r[16] = c.r[16].wrapping_add(0x1);
                pc = 0x80264a74;
            }
            0x80264a74 => { ins += 3;
                c.r[0] = c.r[16].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x80264a18; continue; }
                pc = 0x80264a80;
            }
            0x80264a80 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x20));
                c.r[4] = 0x1;
                c.r[9] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80264a8c;
            }
            0x80264a8c => { ins += 10;
                c.r[3] = 0x80370000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[4]);
                c.r[7] = c.r[3].wrapping_add(0x5d70);
                c.r[10] = c.r[21] | c.r[21];
                c.r[3] = c.r[29] | c.r[29];
                c.r[8] = c.r[31].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x12;
                c.r[6] = 0x0;
                c.lr = 0x80264ab4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80204ce0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264ab4;
            }
            0x80264ab4 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80264ab8;
            }
            0x80264ab8 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x80); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x70); c.f[30] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 16..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x94));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x90);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80264488: bad pc {:#x}", pc),
        }
    }
}

/// _fightMenuFightTrainerGcHeroOpenMenuSubBallSelectTargetPokemon__FP15FightOutPokemonUsUs (0x80264adc, 636 bytes)
#[inline(never)]
pub fn f_80264adc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80264adc) { return; }
        if c.prof { crate::prof::enter(c, 0x80264adc); f_80264adc_at(c, 0x80264adc); crate::prof::exit(c, 0x80264adc); return; }
    }
    f_80264adc_b(c, 0x80264adc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80264adc_at(c: &mut Cpu, pc: u32) { f_80264adc_b(c, pc) }
#[inline(never)]
fn f_80264adc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80264adc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80264adc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80264adc => { ins += 16;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                pc = 0x80264af4;
            }
            0x80264af4 => { ins += 4;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0xf;
                c.lr = 0x80264b04; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264b04;
            }
            0x80264b04 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x80264b0c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264b0c;
            }
            0x80264b0c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264b98; continue; }
                pc = 0x80264b18;
            }
            0x80264b18 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x80264b28; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264b28;
            }
            0x80264b28 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80264b38; continue; }
                pc = 0x80264b30;
            }
            0x80264b30 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264b9c; continue;
                pc = 0x80264b38;
            }
            0x80264b38 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80264b48; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264b48;
            }
            0x80264b48 => { ins += 4;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80264b58; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264b58;
            }
            0x80264b58 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80264b68; continue; }
                pc = 0x80264b60;
            }
            0x80264b60 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264b9c; continue;
                pc = 0x80264b68;
            }
            0x80264b68 => { ins += 1;
                c.lr = 0x80264b6c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264b6c;
            }
            0x80264b6c => { ins += 1;
                c.lr = 0x80264b70; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264b70;
            }
            0x80264b70 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80264b80; continue; }
                pc = 0x80264b78;
            }
            0x80264b78 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264b9c; continue;
                pc = 0x80264b80;
            }
            0x80264b80 => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x80264b94; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264b94;
            }
            0x80264b94 => { ins += 1;
                pc = 0x80264b9c; continue;
                pc = 0x80264b98;
            }
            0x80264b98 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x80264b9c;
            }
            0x80264b9c => { ins += 5;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x10;
                c.lr = 0x80264bb0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264bb0;
            }
            0x80264bb0 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x80264bb8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264bb8;
            }
            0x80264bb8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264c44; continue; }
                pc = 0x80264bc4;
            }
            0x80264bc4 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x80264bd4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264bd4;
            }
            0x80264bd4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80264be4; continue; }
                pc = 0x80264bdc;
            }
            0x80264bdc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264c48; continue;
                pc = 0x80264be4;
            }
            0x80264be4 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80264bf4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264bf4;
            }
            0x80264bf4 => { ins += 4;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80264c04; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264c04;
            }
            0x80264c04 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80264c14; continue; }
                pc = 0x80264c0c;
            }
            0x80264c0c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264c48; continue;
                pc = 0x80264c14;
            }
            0x80264c14 => { ins += 1;
                c.lr = 0x80264c18; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264c18;
            }
            0x80264c18 => { ins += 1;
                c.lr = 0x80264c1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264c1c;
            }
            0x80264c1c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80264c2c; continue; }
                pc = 0x80264c24;
            }
            0x80264c24 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264c48; continue;
                pc = 0x80264c2c;
            }
            0x80264c2c => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x80264c40; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264c40;
            }
            0x80264c40 => { ins += 1;
                pc = 0x80264c48; continue;
                pc = 0x80264c44;
            }
            0x80264c44 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x80264c48;
            }
            0x80264c48 => { ins += 8;
                c.r[4] = 0x0;
                c.r[0] = 0x2;
                c.stw(c.r[1].wrapping_add(0x14), c.r[3]);
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[4]);
                c.stb(c.r[1].wrapping_add(0x28), c.r[0]);
                c.lr = 0x80264c68; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264c68;
            }
            0x80264c68 => { ins += 5;
                c.stb(c.r[1].wrapping_add(0x29), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.lr = 0x80264c7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011288(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264c7c;
            }
            0x80264c7c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80264c94; continue; }
                pc = 0x80264c84;
            }
            0x80264c84 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x80264c8c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8001120c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264c8c;
            }
            0x80264c8c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80264d44; continue;
                pc = 0x80264c94;
            }
            0x80264c94 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x80264cb0; continue; }
                pc = 0x80264c98;
            }
            0x80264c98 => { ins += 4;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0xf;
                c.lr = 0x80264ca8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264ca8;
            }
            0x80264ca8 => { ins += 2;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x80264ccc; continue;
                pc = 0x80264cb0;
            }
            0x80264cb0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264af4; continue; }
                pc = 0x80264cb8;
            }
            0x80264cb8 => { ins += 4;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x10;
                c.lr = 0x80264cc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264cc8;
            }
            0x80264cc8 => { ins += 1;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x80264ccc;
            }
            0x80264ccc => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80264cd4; { c.steps = st; c.cycles = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264cd4;
            }
            0x80264cd4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80264af4; continue; }
                pc = 0x80264cdc;
            }
            0x80264cdc => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80264ce4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80205b8c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264ce4;
            }
            0x80264ce4 => { ins += 1;
                c.lr = 0x80264ce8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011fc74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264ce8;
            }
            0x80264ce8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80264d38; continue; }
                pc = 0x80264cf0;
            }
            0x80264cf0 => { ins += 2;
                c.r[3] = 0x99f;
                c.lr = 0x80264cf8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801906a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264cf8;
            }
            0x80264cf8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80264d14; continue; }
                pc = 0x80264d00;
            }
            0x80264d00 => { ins += 4;
                c.r[3] = 0x7716;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x80264d10; { c.steps = st; c.cycles = ins; c.cr = cr; f_80106394(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264d10;
            }
            0x80264d10 => { ins += 1;
                pc = 0x80264d24; continue;
                pc = 0x80264d14;
            }
            0x80264d14 => { ins += 4;
                c.r[3] = 0x7702;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x80264d24; { c.steps = st; c.cycles = ins; c.cr = cr; f_80106394(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264d24;
            }
            0x80264d24 => { ins += 2;
                c.r[3] = 0x40;
                c.lr = 0x80264d2c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f000c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264d2c;
            }
            0x80264d2c => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80264d34; { c.steps = st; c.cycles = ins; c.cr = cr; f_80106080(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264d34;
            }
            0x80264d34 => { ins += 1;
                pc = 0x80264af4; continue;
                pc = 0x80264d38;
            }
            0x80264d38 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x80264d40; { c.steps = st; c.cycles = ins; c.cr = cr; f_8001120c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264d40;
            }
            0x80264d40 => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x80264d44;
            }
            0x80264d44 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80264adc: bad pc {:#x}", pc),
        }
    }
}

/// _fightMenuFightTrainerGcHeroOpenMenuSubWaza__FP13FIGHT_TRAINERP15FightOutPokemonUs (0x80264d58, 740 bytes)
#[inline(never)]
pub fn f_80264d58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80264d58) { return; }
        if c.prof { crate::prof::enter(c, 0x80264d58); f_80264d58_at(c, 0x80264d58); crate::prof::exit(c, 0x80264d58); return; }
    }
    f_80264d58_b(c, 0x80264d58);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80264d58_at(c: &mut Cpu, pc: u32) { f_80264d58_b(c, pc) }
#[inline(never)]
fn f_80264d58_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80264d58;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80264d58, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80264d58 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xffffff80); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x84), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x60); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[5] | c.r[5];
                c.r[30] = c.r[4] | c.r[4];
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80264d78; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264d78;
            }
            0x80264d78 => { ins += 1;
                c.lr = 0x80264d7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e1a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264d7c;
            }
            0x80264d7c => { ins += 4;
                c.r[26] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.lr = 0x80264d8c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fe3f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264d8c;
            }
            0x80264d8c => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80264d94; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264d94;
            }
            0x80264d94 => { ins += 3;
                c.stb(c.r[1].wrapping_add(0x58), c.r[3]);
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80264da0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80205b8c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264da0;
            }
            0x80264da0 => { ins += 1;
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x80264da4;
            }
            0x80264da4 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x101;
                c.r[6] = 0x0;
                c.lr = 0x80264db8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264db8;
            }
            0x80264db8 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.r[4] = c.r[0] | c.r[0];
                c.r[5] = 0x1;
                c.lr = 0x80264dcc; { c.steps = st; c.cycles = ins; c.cr = cr; f_800117bc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264dcc;
            }
            0x80264dcc => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 0) { pc = 0x80264de4; continue; }
                pc = 0x80264dd4;
            }
            0x80264dd4 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x80264ddc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264ddc;
            }
            0x80264ddc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265028; continue;
                pc = 0x80264de4;
            }
            0x80264de4 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[6] = c.r[1].wrapping_add(0x10);
                c.r[5] = 0x1;
                c.lr = 0x80264df8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ffec8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264df8;
            }
            0x80264df8 => { ins += 6;
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[31].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x80264e10; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264e10;
            }
            0x80264e10 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                if cr_bit(cr, 2) { pc = 0x80264e70; continue; }
                pc = 0x80264e1c;
            }
            0x80264e1c => { ins += 3;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x11;
                c.lr = 0x80264e28; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264e28;
            }
            0x80264e28 => { ins += 5;
                c.r[4] = c.r[24] | c.r[24];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80264e3c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264e3c;
            }
            0x80264e3c => { ins += 1;
                c.lr = 0x80264e40; { c.steps = st; c.cycles = ins; c.cr = cr; f_800fa280(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264e40;
            }
            0x80264e40 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x28;
                c.lr = 0x80264e4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264e4c;
            }
            0x80264e4c => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80264e54; { c.steps = st; c.cycles = ins; c.cr = cr; f_802040e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264e54;
            }
            0x80264e54 => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[7] = c.r[0].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x56;
                c.r[6] = 0x0;
                c.lr = 0x80264e70; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264e70;
            }
            0x80264e70 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 2) { pc = 0x80264e84; continue; }
                pc = 0x80264e7c;
            }
            0x80264e7c => { ins += 2;
                c.r[27] = 0x7661;
                pc = 0x80264ef4; continue;
                pc = 0x80264e84;
            }
            0x80264e84 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x5);
                if !cr_bit(cr, 2) { pc = 0x80264eb8; continue; }
                pc = 0x80264e8c;
            }
            0x80264e8c => { ins += 5;
                c.r[4] = c.lhz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80264ea0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264ea0;
            }
            0x80264ea0 => { ins += 1;
                c.lr = 0x80264ea4; { c.steps = st; c.cycles = ins; c.cr = cr; f_800fa280(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264ea4;
            }
            0x80264ea4 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x28;
                c.lr = 0x80264eb0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264eb0;
            }
            0x80264eb0 => { ins += 2;
                c.r[27] = 0x76bb;
                pc = 0x80264ef4; continue;
                pc = 0x80264eb8;
            }
            0x80264eb8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if !cr_bit(cr, 2) { pc = 0x80264ec8; continue; }
                pc = 0x80264ec0;
            }
            0x80264ec0 => { ins += 2;
                c.r[27] = 0x7600;
                pc = 0x80264ef4; continue;
                pc = 0x80264ec8;
            }
            0x80264ec8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x80264ed8; continue; }
                pc = 0x80264ed0;
            }
            0x80264ed0 => { ins += 2;
                c.r[27] = 0x75ff;
                pc = 0x80264ef4; continue;
                pc = 0x80264ed8;
            }
            0x80264ed8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x80264ee8; continue; }
                pc = 0x80264ee0;
            }
            0x80264ee0 => { ins += 2;
                c.r[27] = 0x75fe;
                pc = 0x80264ef4; continue;
                pc = 0x80264ee8;
            }
            0x80264ee8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264ef4; continue; }
                pc = 0x80264ef0;
            }
            0x80264ef0 => { ins += 1;
                c.r[27] = 0x75fd;
                pc = 0x80264ef4;
            }
            0x80264ef4 => { ins += 2;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80264f28; continue; }
                pc = 0x80264efc;
            }
            0x80264efc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x0);
                if cr_bit(cr, 2) { pc = 0x80264f14; continue; }
                pc = 0x80264f04;
            }
            0x80264f04 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x80264f14; { c.steps = st; c.cycles = ins; c.cr = cr; f_80106394(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264f14;
            }
            0x80264f14 => { ins += 2;
                c.r[3] = 0x40;
                c.lr = 0x80264f1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f000c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264f1c;
            }
            0x80264f1c => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80264f24; { c.steps = st; c.cycles = ins; c.cr = cr; f_80106080(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264f24;
            }
            0x80264f24 => { ins += 1;
                pc = 0x80264da4; continue;
                pc = 0x80264f28;
            }
            0x80264f28 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[31].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x80264f3c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264f3c;
            }
            0x80264f3c => { ins += 2;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[29]);
                if cr_bit(cr, 2) { pc = 0x80264da4; continue; }
                pc = 0x80264f44;
            }
            0x80264f44 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff85d8));
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x80264da4; continue; }
                pc = 0x80264f54;
            }
            0x80264f54 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x165);
                if cr_bit(cr, 2) { pc = 0x80264da4; continue; }
                pc = 0x80264f5c;
            }
            0x80264f5c => { ins += 11;
                c.r[0] = 0x0;
                c.r[3] = 0x80260000;
                c.stb(c.r[13].wrapping_add(0xffffae58), c.r[0]);
                c.r[6] = c.r[3].wrapping_add(0x503c);
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[25] | c.r[25];
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                c.r[9] = 0xffffffff;
                c.lr = 0x80264f88; { c.steps = st; c.cycles = ins; c.cr = cr; f_8022b2cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264f88;
            }
            0x80264f88 => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[24] = c.r[0] | c.r[0];
                c.lr = 0x80264f98; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264f98;
            }
            0x80264f98 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80264fd0; continue; }
                pc = 0x80264fa4;
            }
            0x80264fa4 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[13].wrapping_add(0xffffae58));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80264fd0; continue; }
                pc = 0x80264fb0;
            }
            0x80264fb0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], 0x2);
                if cr_bit(cr, 0) { pc = 0x80264fd0; continue; }
                pc = 0x80264fb8;
            }
            0x80264fb8 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[25] | c.r[25];
                c.lr = 0x80264fc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026503c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264fc8;
            }
            0x80264fc8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80264da4; continue; }
                pc = 0x80264fd0;
            }
            0x80264fd0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[24], 0x0);
                if cr_bit(cr, 2) { pc = 0x80264da4; continue; }
                pc = 0x80264fd8;
            }
            0x80264fd8 => { ins += 3;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[25] | c.r[25];
                c.lr = 0x80264fe4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264fe4;
            }
            0x80264fe4 => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x1;
                c.r[24] = c.r[0] | c.r[0];
                c.lr = 0x80264ff4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80264ff4;
            }
            0x80264ff4 => { ins += 12;
                c.r[0] = 0x0;
                c.r[3] = 0x80370000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[7] = c.r[3].wrapping_add(0x5ca8);
                c.r[3] = c.r[30] | c.r[30];
                c.r[8] = c.r[29] | c.r[29];
                c.r[9] = c.r[24] | c.r[24];
                c.r[10] = c.r[31] as u8 as i8 as i32 as u32;
                c.r[4] = 0x0;
                c.r[5] = 0x13;
                c.r[6] = 0x0;
                c.lr = 0x80265024; { c.steps = st; c.cycles = ins; c.cr = cr; f_80204f6c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265024;
            }
            0x80265024 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80265028;
            }
            0x80265028 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x60); for n in 24..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x84));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x80);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80264d58: bad pc {:#x}", pc),
        }
    }
}

/// _fightMenuFightTrainerGcHeroOpenMenuSubWazaSelectDefensePokemon__FP15FightOutPokemonUsUs (0x8026503c, 752 bytes)
#[inline(never)]
pub fn f_8026503c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8026503c) { return; }
        if c.prof { crate::prof::enter(c, 0x8026503c); f_8026503c_at(c, 0x8026503c); crate::prof::exit(c, 0x8026503c); return; }
    }
    f_8026503c_b(c, 0x8026503c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8026503c_at(c: &mut Cpu, pc: u32) { f_8026503c_b(c, pc) }
#[inline(never)]
fn f_8026503c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8026503c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8026503c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8026503c => { ins += 16;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                pc = 0x80265054;
            }
            0x80265054 => { ins += 4;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0xf;
                c.lr = 0x80265064; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265064;
            }
            0x80265064 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x8026506c; { c.steps = st; c.cycles = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026506c;
            }
            0x8026506c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802650f8; continue; }
                pc = 0x80265078;
            }
            0x80265078 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x80265088; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265088;
            }
            0x80265088 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80265098; continue; }
                pc = 0x80265090;
            }
            0x80265090 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802650fc; continue;
                pc = 0x80265098;
            }
            0x80265098 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x802650a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802650a8;
            }
            0x802650a8 => { ins += 4;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x802650b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802650b8;
            }
            0x802650b8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802650c8; continue; }
                pc = 0x802650c0;
            }
            0x802650c0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802650fc; continue;
                pc = 0x802650c8;
            }
            0x802650c8 => { ins += 1;
                c.lr = 0x802650cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802650cc;
            }
            0x802650cc => { ins += 1;
                c.lr = 0x802650d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802650d0;
            }
            0x802650d0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x802650e0; continue; }
                pc = 0x802650d8;
            }
            0x802650d8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802650fc; continue;
                pc = 0x802650e0;
            }
            0x802650e0 => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x802650f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802650f4;
            }
            0x802650f4 => { ins += 1;
                pc = 0x802650fc; continue;
                pc = 0x802650f8;
            }
            0x802650f8 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x802650fc;
            }
            0x802650fc => { ins += 5;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x10;
                c.lr = 0x80265110; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265110;
            }
            0x80265110 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x80265118; { c.steps = st; c.cycles = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265118;
            }
            0x80265118 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802651a4; continue; }
                pc = 0x80265124;
            }
            0x80265124 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x80265134; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265134;
            }
            0x80265134 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80265144; continue; }
                pc = 0x8026513c;
            }
            0x8026513c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802651a8; continue;
                pc = 0x80265144;
            }
            0x80265144 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80265154; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265154;
            }
            0x80265154 => { ins += 4;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80265164; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265164;
            }
            0x80265164 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80265174; continue; }
                pc = 0x8026516c;
            }
            0x8026516c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802651a8; continue;
                pc = 0x80265174;
            }
            0x80265174 => { ins += 1;
                c.lr = 0x80265178; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265178;
            }
            0x80265178 => { ins += 1;
                c.lr = 0x8026517c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026517c;
            }
            0x8026517c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x8026518c; continue; }
                pc = 0x80265184;
            }
            0x80265184 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802651a8; continue;
                pc = 0x8026518c;
            }
            0x8026518c => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x802651a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802651a0;
            }
            0x802651a0 => { ins += 1;
                pc = 0x802651a8; continue;
                pc = 0x802651a4;
            }
            0x802651a4 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x802651a8;
            }
            0x802651a8 => { ins += 5;
                c.stw(c.r[1].wrapping_add(0x14), c.r[3]);
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0xe;
                c.lr = 0x802651bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802651bc;
            }
            0x802651bc => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x802651c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802651c4;
            }
            0x802651c4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80265250; continue; }
                pc = 0x802651d0;
            }
            0x802651d0 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x802651e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802651e0;
            }
            0x802651e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802651f0; continue; }
                pc = 0x802651e8;
            }
            0x802651e8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265254; continue;
                pc = 0x802651f0;
            }
            0x802651f0 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80265200; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265200;
            }
            0x80265200 => { ins += 4;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80265210; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265210;
            }
            0x80265210 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80265220; continue; }
                pc = 0x80265218;
            }
            0x80265218 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265254; continue;
                pc = 0x80265220;
            }
            0x80265220 => { ins += 1;
                c.lr = 0x80265224; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265224;
            }
            0x80265224 => { ins += 1;
                c.lr = 0x80265228; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265228;
            }
            0x80265228 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80265238; continue; }
                pc = 0x80265230;
            }
            0x80265230 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265254; continue;
                pc = 0x80265238;
            }
            0x80265238 => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x8026524c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026524c;
            }
            0x8026524c => { ins += 1;
                pc = 0x80265254; continue;
                pc = 0x80265250;
            }
            0x80265250 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x80265254;
            }
            0x80265254 => { ins += 7;
                c.r[4] = 0x0;
                c.r[0] = 0x3;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[3]);
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[4]);
                c.stb(c.r[1].wrapping_add(0x28), c.r[0]);
                c.lr = 0x80265270; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265270;
            }
            0x80265270 => { ins += 5;
                c.stb(c.r[1].wrapping_add(0x29), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.lr = 0x80265284; { c.steps = st; c.cycles = ins; c.cr = cr; f_80011288(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265284;
            }
            0x80265284 => { ins += 4;
                c.r[0] = 0x1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stb(c.r[13].wrapping_add(0xffffae58), c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x802652a4; continue; }
                pc = 0x80265294;
            }
            0x80265294 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x8026529c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8001120c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026529c;
            }
            0x8026529c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265318; continue;
                pc = 0x802652a4;
            }
            0x802652a4 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x802652c0; continue; }
                pc = 0x802652a8;
            }
            0x802652a8 => { ins += 4;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0xf;
                c.lr = 0x802652b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802652b8;
            }
            0x802652b8 => { ins += 2;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x802652fc; continue;
                pc = 0x802652c0;
            }
            0x802652c0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802652e0; continue; }
                pc = 0x802652c8;
            }
            0x802652c8 => { ins += 4;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0x10;
                c.lr = 0x802652d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802652d8;
            }
            0x802652d8 => { ins += 2;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x802652fc; continue;
                pc = 0x802652e0;
            }
            0x802652e0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x2);
                if !cr_bit(cr, 2) { pc = 0x80265054; continue; }
                pc = 0x802652e8;
            }
            0x802652e8 => { ins += 4;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0xe;
                c.lr = 0x802652f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802652f8;
            }
            0x802652f8 => { ins += 1;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x802652fc;
            }
            0x802652fc => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80265304; { c.steps = st; c.cycles = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265304;
            }
            0x80265304 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80265054; continue; }
                pc = 0x8026530c;
            }
            0x8026530c => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x80265314; { c.steps = st; c.cycles = ins; c.cr = cr; f_8001120c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265314;
            }
            0x80265314 => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x80265318;
            }
            0x80265318 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8026503c: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuStatusMenuClose (0x8026532c, 208 bytes)
#[inline(never)]
pub fn f_8026532c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8026532c) { return; }
        if c.prof { crate::prof::enter(c, 0x8026532c); f_8026532c_at(c, 0x8026532c); crate::prof::exit(c, 0x8026532c); return; }
    }
    f_8026532c_b(c, 0x8026532c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8026532c_at(c: &mut Cpu, pc: u32) { f_8026532c_b(c, pc) }
#[inline(always)]
fn f_8026532c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8026532c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8026532c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8026532c => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[3] = 0x2;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x80265358; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265358;
            }
            0x80265358 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80265368; continue; }
                pc = 0x80265360;
            }
            0x80265360 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x802653c8; continue;
                pc = 0x80265368;
            }
            0x80265368 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80265378; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265378;
            }
            0x80265378 => { ins += 4;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80265388; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265388;
            }
            0x80265388 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80265398; continue; }
                pc = 0x80265390;
            }
            0x80265390 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x802653c8; continue;
                pc = 0x80265398;
            }
            0x80265398 => { ins += 1;
                c.lr = 0x8026539c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026539c;
            }
            0x8026539c => { ins += 1;
                c.lr = 0x802653a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802653a0;
            }
            0x802653a0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x802653b0; continue; }
                pc = 0x802653a8;
            }
            0x802653a8 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x802653c8; continue;
                pc = 0x802653b0;
            }
            0x802653b0 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x802653c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802653c4;
            }
            0x802653c4 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x802653c8;
            }
            0x802653c8 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802653d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102620(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802653d0;
            }
            0x802653d0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x802653e8; continue; }
                pc = 0x802653d8;
            }
            0x802653d8 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x802653e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102568(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802653e8;
            }
            0x802653e8 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8026532c: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuFightOutPokemonRenewStatusMenu (0x802653fc, 412 bytes)
#[inline(never)]
pub fn f_802653fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802653fc) { return; }
        if c.prof { crate::prof::enter(c, 0x802653fc); f_802653fc_at(c, 0x802653fc); crate::prof::exit(c, 0x802653fc); return; }
    }
    f_802653fc_b(c, 0x802653fc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802653fc_at(c: &mut Cpu, pc: u32) { f_802653fc_b(c, pc) }
#[inline(never)]
fn f_802653fc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802653fc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802653fc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802653fc => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffff80); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x84), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x70); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[3] = 0x2;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x80265428; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265428;
            }
            0x80265428 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80265438; continue; }
                pc = 0x80265430;
            }
            0x80265430 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265494; continue;
                pc = 0x80265438;
            }
            0x80265438 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80265448; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265448;
            }
            0x80265448 => { ins += 4;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80265458; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265458;
            }
            0x80265458 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80265468; continue; }
                pc = 0x80265460;
            }
            0x80265460 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265494; continue;
                pc = 0x80265468;
            }
            0x80265468 => { ins += 1;
                c.lr = 0x8026546c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026546c;
            }
            0x8026546c => { ins += 1;
                c.lr = 0x80265470; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265470;
            }
            0x80265470 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80265480; continue; }
                pc = 0x80265478;
            }
            0x80265478 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265494; continue;
                pc = 0x80265480;
            }
            0x80265480 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x80265494; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265494;
            }
            0x80265494 => { ins += 1;
                c.lr = 0x80265498; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102620(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265498;
            }
            0x80265498 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80265584; continue; }
                pc = 0x802654a0;
            }
            0x802654a0 => { ins += 4;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = 0x2;
                c.lr = 0x802654b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802654b0;
            }
            0x802654b0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802654c0; continue; }
                pc = 0x802654b8;
            }
            0x802654b8 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80265520; continue;
                pc = 0x802654c0;
            }
            0x802654c0 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x802654d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802654d0;
            }
            0x802654d0 => { ins += 4;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x802654e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802654e0;
            }
            0x802654e0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802654f0; continue; }
                pc = 0x802654e8;
            }
            0x802654e8 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80265520; continue;
                pc = 0x802654f0;
            }
            0x802654f0 => { ins += 1;
                c.lr = 0x802654f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802654f4;
            }
            0x802654f4 => { ins += 1;
                c.lr = 0x802654f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802654f8;
            }
            0x802654f8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80265508; continue; }
                pc = 0x80265500;
            }
            0x80265500 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80265520; continue;
                pc = 0x80265508;
            }
            0x80265508 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x8026551c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026551c;
            }
            0x8026551c => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80265520;
            }
            0x80265520 => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8026552c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fe168(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026552c;
            }
            0x8026552c => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8026553c; continue; }
                pc = 0x80265534;
            }
            0x80265534 => { ins += 2;
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0x31), c.r[0]);
                pc = 0x8026553c;
            }
            0x8026553c => { ins += 5;
                c.r[0] = 0x6;
                c.r[5] = c.r[1].wrapping_add(0x34);
                c.r[4] = c.r[1].wrapping_add(0x4);
                c.mtspr(9, c.r[0]);
                pc = 0x8026554c;
            }
            0x8026554c => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8026554c; continue; }
                pc = 0x80265560;
            }
            0x80265560 => { ins += 9;
                c.r[3] = c.r[31] | c.r[31];
                c.r[9] = c.r[1].wrapping_add(0x38);
                c.r[4] = 0xffffffff;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80265584; { c.steps = st; c.cycles = ins; c.cr = cr; f_801026a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265584;
            }
            0x80265584 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x70); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x84));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x80);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802653fc: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuStatusMenuOpen (0x80265598, 276 bytes)
#[inline(never)]
pub fn f_80265598(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265598) { return; }
        if c.prof { crate::prof::enter(c, 0x80265598); f_80265598_at(c, 0x80265598); crate::prof::exit(c, 0x80265598); return; }
    }
    f_80265598_b(c, 0x80265598);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265598_at(c: &mut Cpu, pc: u32) { f_80265598_b(c, pc) }
#[inline(always)]
fn f_80265598_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265598;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265598, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265598 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffff80); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x84), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x70); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[3] = 0x2;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x802655c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802655c4;
            }
            0x802655c4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802655d4; continue; }
                pc = 0x802655cc;
            }
            0x802655cc => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80265634; continue;
                pc = 0x802655d4;
            }
            0x802655d4 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x802655e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802655e4;
            }
            0x802655e4 => { ins += 4;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x802655f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802655f4;
            }
            0x802655f4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80265604; continue; }
                pc = 0x802655fc;
            }
            0x802655fc => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80265634; continue;
                pc = 0x80265604;
            }
            0x80265604 => { ins += 1;
                c.lr = 0x80265608; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265608;
            }
            0x80265608 => { ins += 1;
                c.lr = 0x8026560c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026560c;
            }
            0x8026560c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x8026561c; continue; }
                pc = 0x80265614;
            }
            0x80265614 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80265634; continue;
                pc = 0x8026561c;
            }
            0x8026561c => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x80265630; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265630;
            }
            0x80265630 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80265634;
            }
            0x80265634 => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80265640; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fe168(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265640;
            }
            0x80265640 => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80265650; continue; }
                pc = 0x80265648;
            }
            0x80265648 => { ins += 2;
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0x31), c.r[0]);
                pc = 0x80265650;
            }
            0x80265650 => { ins += 5;
                c.r[0] = 0x6;
                c.r[5] = c.r[1].wrapping_add(0x34);
                c.r[4] = c.r[1].wrapping_add(0x4);
                c.mtspr(9, c.r[0]);
                pc = 0x80265660;
            }
            0x80265660 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80265660; continue; }
                pc = 0x80265674;
            }
            0x80265674 => { ins += 9;
                c.r[3] = c.r[31] | c.r[31];
                c.r[9] = c.r[1].wrapping_add(0x38);
                c.r[4] = 0xffffffff;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80265698; { c.steps = st; c.cycles = ins; c.cr = cr; f_801026a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265698;
            }
            0x80265698 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x70); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x84));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x80);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265598: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuGetFightOutPokemonPtrToStatusMenuId (0x802656ac, 168 bytes)
#[inline(never)]
pub fn f_802656ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802656ac) { return; }
        if c.prof { crate::prof::enter(c, 0x802656ac); f_802656ac_at(c, 0x802656ac); crate::prof::exit(c, 0x802656ac); return; }
    }
    f_802656ac_b(c, 0x802656ac);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802656ac_at(c: &mut Cpu, pc: u32) { f_802656ac_b(c, pc) }
#[inline(always)]
fn f_802656ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802656ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802656ac, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802656ac => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[3] = 0x2;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x802656d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802656d4;
            }
            0x802656d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802656e4; continue; }
                pc = 0x802656dc;
            }
            0x802656dc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265740; continue;
                pc = 0x802656e4;
            }
            0x802656e4 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x802656f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802656f4;
            }
            0x802656f4 => { ins += 4;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80265704; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265704;
            }
            0x80265704 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80265714; continue; }
                pc = 0x8026570c;
            }
            0x8026570c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265740; continue;
                pc = 0x80265714;
            }
            0x80265714 => { ins += 1;
                c.lr = 0x80265718; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265718;
            }
            0x80265718 => { ins += 1;
                c.lr = 0x8026571c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026571c;
            }
            0x8026571c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x8026572c; continue; }
                pc = 0x80265724;
            }
            0x80265724 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265740; continue;
                pc = 0x8026572c;
            }
            0x8026572c => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.lr = 0x80265740; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265740;
            }
            0x80265740 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802656ac: bad pc {:#x}", pc),
        }
    }
}

/// fightMenuFightTrainerRenewStatusMenu (0x80265754, 372 bytes)
#[inline(never)]
pub fn f_80265754(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265754) { return; }
        if c.prof { crate::prof::enter(c, 0x80265754); f_80265754_at(c, 0x80265754); crate::prof::exit(c, 0x80265754); return; }
    }
    f_80265754_b(c, 0x80265754);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265754_at(c: &mut Cpu, pc: u32) { f_80265754_b(c, pc) }
#[inline(never)]
fn f_80265754_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265754;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265754, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265754 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[3] = 0x2;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8026577c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026577c;
            }
            0x8026577c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8026578c; continue; }
                pc = 0x80265784;
            }
            0x80265784 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802657e8; continue;
                pc = 0x8026578c;
            }
            0x8026578c => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x8026579c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026579c;
            }
            0x8026579c => { ins += 4;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x802657ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802657ac;
            }
            0x802657ac => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802657bc; continue; }
                pc = 0x802657b4;
            }
            0x802657b4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802657e8; continue;
                pc = 0x802657bc;
            }
            0x802657bc => { ins += 1;
                c.lr = 0x802657c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802657c0;
            }
            0x802657c0 => { ins += 1;
                c.lr = 0x802657c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802657c4;
            }
            0x802657c4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x802657d4; continue; }
                pc = 0x802657cc;
            }
            0x802657cc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802657e8; continue;
                pc = 0x802657d4;
            }
            0x802657d4 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x802657e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802657e8;
            }
            0x802657e8 => { ins += 1;
                c.lr = 0x802657ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80102620(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802657ec;
            }
            0x802657ec => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x802658b4; continue; }
                pc = 0x802657f4;
            }
            0x802657f4 => { ins += 4;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.r[3] = 0x2;
                c.lr = 0x80265804; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265804;
            }
            0x80265804 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80265814; continue; }
                pc = 0x8026580c;
            }
            0x8026580c => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80265874; continue;
                pc = 0x80265814;
            }
            0x80265814 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80265824; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265824;
            }
            0x80265824 => { ins += 4;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80265834; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265834;
            }
            0x80265834 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80265844; continue; }
                pc = 0x8026583c;
            }
            0x8026583c => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80265874; continue;
                pc = 0x80265844;
            }
            0x80265844 => { ins += 1;
                c.lr = 0x80265848; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265848;
            }
            0x80265848 => { ins += 1;
                c.lr = 0x8026584c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0204(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026584c;
            }
            0x8026584c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x8026585c; continue; }
                pc = 0x80265854;
            }
            0x80265854 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x80265874; continue;
                pc = 0x8026585c;
            }
            0x8026585c => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x80265870; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265870;
            }
            0x80265870 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80265874;
            }
            0x80265874 => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80265880; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7954(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265880;
            }
            0x80265880 => { ins += 13;
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lhz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[9] = c.r[1].wrapping_add(0x10);
                c.stw(c.r[1].wrapping_add(0x10), c.r[4]);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.sth(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = 0x0;
                c.r[8] = 0x1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x802658b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801026a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802658b4;
            }
            0x802658b4 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265754: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerCommandTerminate (0x802658c8, 92 bytes)
#[inline(never)]
pub fn f_802658c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802658c8) { return; }
        if c.prof { crate::prof::enter(c, 0x802658c8); f_802658c8_at(c, 0x802658c8); crate::prof::exit(c, 0x802658c8); return; }
    }
    f_802658c8_b(c, 0x802658c8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802658c8_at(c: &mut Cpu, pc: u32) { f_802658c8_b(c, pc) }
#[inline(always)]
fn f_802658c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802658c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802658c8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802658c8 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xffff8800);
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x802658f0; continue; }
                pc = 0x802658ec;
            }
            0x802658ec => { ins += 1;
                c.lr = 0x802658f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f05a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802658f0;
            }
            0x802658f0 => { ins += 14;
                c.r[0] = 0x0;
                c.r[3] = 0x80480000;
                { let ea = c.r[3].wrapping_add(0xffff8800); c.stb(ea, c.r[0]); c.r[3] = ea; }
                { let ea = c.r[2].wrapping_add(0xffffb038); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802658c8: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerCommandIsOver (0x80265924, 56 bytes)
#[inline(never)]
pub fn f_80265924(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265924) { return; }
        if c.prof { crate::prof::enter(c, 0x80265924); f_80265924_at(c, 0x80265924); crate::prof::exit(c, 0x80265924); return; }
    }
    f_80265924_b(c, 0x80265924);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265924_at(c: &mut Cpu, pc: u32) { f_80265924_b(c, pc) }
#[inline(always)]
fn f_80265924_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265924;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265924, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265924 => { ins += 5;
                c.r[3] = 0x80480000;
                c.r[3] = c.r[3].wrapping_add(0xffff8800);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80265940; continue; }
                pc = 0x80265938;
            }
            0x80265938 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x80265940;
            }
            0x80265940 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80265954; continue; }
                pc = 0x8026594c;
            }
            0x8026594c => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x80265954;
            }
            0x80265954 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265924: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerCommandGetNokoriTime (0x8026595c, 72 bytes)
#[inline(never)]
pub fn f_8026595c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8026595c) { return; }
        if c.prof { crate::prof::enter(c, 0x8026595c); f_8026595c_at(c, 0x8026595c); crate::prof::exit(c, 0x8026595c); return; }
    }
    f_8026595c_b(c, 0x8026595c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8026595c_at(c: &mut Cpu, pc: u32) { f_8026595c_b(c, pc) }
#[inline(always)]
fn f_8026595c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8026595c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8026595c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8026595c => { ins += 5;
                c.r[3] = 0x80480000;
                c.r[3] = c.r[3].wrapping_add(0xffff8800);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80265978; continue; }
                pc = 0x80265970;
            }
            0x80265970 => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffffb048); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x80265978;
            }
            0x80265978 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8026598c; continue; }
                pc = 0x80265984;
            }
            0x80265984 => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffffb038); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8026598c;
            }
            0x8026598c => { ins += 22;
                { let ea = c.r[3].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffffb03c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[2] - c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8026595c: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerCommandBlock (0x802659a4, 84 bytes)
#[inline(never)]
pub fn f_802659a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802659a4) { return; }
        if c.prof { crate::prof::enter(c, 0x802659a4); f_802659a4_at(c, 0x802659a4); crate::prof::exit(c, 0x802659a4); return; }
    }
    f_802659a4_b(c, 0x802659a4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802659a4_at(c: &mut Cpu, pc: u32) { f_802659a4_b(c, pc) }
#[inline(always)]
fn f_802659a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802659a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802659a4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802659a4 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0xffff8800);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802659cc; continue; }
                pc = 0x802659c4;
            }
            0x802659c4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802659e8; continue;
                pc = 0x802659cc;
            }
            0x802659cc => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x802659e0; continue; }
                pc = 0x802659d8;
            }
            0x802659d8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802659e8; continue;
                pc = 0x802659e0;
            }
            0x802659e0 => { ins += 1;
                c.lr = 0x802659e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f0438(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802659e4;
            }
            0x802659e4 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x802659e8;
            }
            0x802659e8 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802659a4: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerCommandStart (0x802659f8, 116 bytes)
#[inline(never)]
pub fn f_802659f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802659f8) { return; }
        if c.prof { crate::prof::enter(c, 0x802659f8); f_802659f8_at(c, 0x802659f8); crate::prof::exit(c, 0x802659f8); return; }
    }
    f_802659f8_b(c, 0x802659f8);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802659f8_at(c: &mut Cpu, pc: u32) { f_802659f8_b(c, pc) }
#[inline(always)]
fn f_802659f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802659f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802659f8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802659f8 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0xffff8800);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80265a5c; continue; }
                pc = 0x80265a18;
            }
            0x80265a18 => { ins += 1;
                c.lr = 0x80265a1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_800ff560(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265a1c;
            }
            0x80265a1c => { ins += 8;
                c.r[5] = 0x80260000;
                c.r[4] = c.r[3] | c.r[3];
                c.r[8] = c.r[5].wrapping_add(0x5db0);
                c.r[3] = 0x1;
                c.r[5] = 0x4000;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x80265a3c; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f07a8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265a3c;
            }
            0x80265a3c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80265a5c; continue; }
                pc = 0x80265a44;
            }
            0x80265a44 => { ins += 6;
                c.r[5] = 0x80480000;
                c.r[4] = 0x1;
                c.r[5] = c.r[5].wrapping_add(0xffff8800);
                c.stw(c.r[5].wrapping_add(0xc), c.r[3]);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80265a5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f0654(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265a5c;
            }
            0x80265a5c => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802659f8: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerCommandInit (0x80265a6c, 208 bytes)
#[inline(never)]
pub fn f_80265a6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265a6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80265a6c); f_80265a6c_at(c, 0x80265a6c); crate::prof::exit(c, 0x80265a6c); return; }
    }
    f_80265a6c_b(c, 0x80265a6c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265a6c_at(c: &mut Cpu, pc: u32) { f_80265a6c_b(c, pc) }
#[inline(always)]
fn f_80265a6c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265a6c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265a6c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265a6c => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.lr = 0x80265a88; { c.steps = st; c.cycles = ins; c.cr = cr; f_80077b84(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265a88;
            }
            0x80265a88 => { ins += 12;
                c.r[4] = c.r[3] ^ 0x80000000;
                c.r[0] = 0x43300000;
                c.r[3] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[4]);
                c.r[31] = c.r[3].wrapping_add(0xffff8800);
                { let ea = c.r[2].wrapping_add(0xffffb050); c.f[1] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                { let v = s(c.f[0] - c.f[1]); c.f[31] = v; c.ps1[31] = v; }
                if cr_bit(cr, 2) { pc = 0x80265ae0; continue; }
                pc = 0x80265ab8;
            }
            0x80265ab8 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x80265ac0; continue; }
                pc = 0x80265abc;
            }
            0x80265abc => { ins += 1;
                c.lr = 0x80265ac0; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f05a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265ac0;
            }
            0x80265ac0 => { ins += 8;
                c.r[0] = 0x0;
                c.r[3] = 0x80480000;
                { let ea = c.r[3].wrapping_add(0xffff8800); c.stb(ea, c.r[0]); c.r[3] = ea; }
                { let ea = c.r[2].wrapping_add(0xffffb038); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                pc = 0x80265ae0;
            }
            0x80265ae0 => { ins += 7;
                { let ea = c.r[2].wrapping_add(0xffffb038); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[0] = 0x0;
                c.r[3] = 0x80480000;
                { let ea = c.r[3].wrapping_add(0xffff8800); c.stb(ea, c.r[0]); c.r[3] = ea; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[31], c.f[0]);
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                if !cr_bit(cr, 0) { pc = 0x80265b10; continue; }
                pc = 0x80265afc;
            }
            0x80265afc => { ins += 5;
                { let ea = c.r[2].wrapping_add(0xffffb048); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[0] = 0x1;
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80265b18; continue;
                pc = 0x80265b10;
            }
            0x80265b10 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[31]; c.stfs(ea, v); }
                pc = 0x80265b18;
            }
            0x80265b18 => { ins += 10;
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                { let ea = c.r[1].wrapping_add(0x20); c.f[31] = c.lfd(ea); }
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265a6c: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerAllIsOver (0x80265b3c, 56 bytes)
#[inline(never)]
pub fn f_80265b3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265b3c) { return; }
        if c.prof { crate::prof::enter(c, 0x80265b3c); f_80265b3c_at(c, 0x80265b3c); crate::prof::exit(c, 0x80265b3c); return; }
    }
    f_80265b3c_b(c, 0x80265b3c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265b3c_at(c: &mut Cpu, pc: u32) { f_80265b3c_b(c, pc) }
#[inline(always)]
fn f_80265b3c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265b3c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265b3c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265b3c => { ins += 5;
                c.r[3] = 0x80480000;
                c.r[3] = c.r[3].wrapping_add(0xffff8810);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80265b58; continue; }
                pc = 0x80265b50;
            }
            0x80265b50 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x80265b58;
            }
            0x80265b58 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80265b6c; continue; }
                pc = 0x80265b64;
            }
            0x80265b64 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x80265b6c;
            }
            0x80265b6c => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265b3c: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerAllGetNokoriTime (0x80265b74, 72 bytes)
#[inline(never)]
pub fn f_80265b74(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265b74) { return; }
        if c.prof { crate::prof::enter(c, 0x80265b74); f_80265b74_at(c, 0x80265b74); crate::prof::exit(c, 0x80265b74); return; }
    }
    f_80265b74_b(c, 0x80265b74);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265b74_at(c: &mut Cpu, pc: u32) { f_80265b74_b(c, pc) }
#[inline(always)]
fn f_80265b74_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265b74;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265b74, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265b74 => { ins += 5;
                c.r[3] = 0x80480000;
                c.r[3] = c.r[3].wrapping_add(0xffff8810);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80265b90; continue; }
                pc = 0x80265b88;
            }
            0x80265b88 => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffffb048); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x80265b90;
            }
            0x80265b90 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80265ba4; continue; }
                pc = 0x80265b9c;
            }
            0x80265b9c => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffffb038); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x80265ba4;
            }
            0x80265ba4 => { ins += 22;
                { let ea = c.r[3].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffffb03c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[2] - c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265b74: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerAllBlock (0x80265bbc, 84 bytes)
#[inline(never)]
pub fn f_80265bbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265bbc) { return; }
        if c.prof { crate::prof::enter(c, 0x80265bbc); f_80265bbc_at(c, 0x80265bbc); crate::prof::exit(c, 0x80265bbc); return; }
    }
    f_80265bbc_b(c, 0x80265bbc);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265bbc_at(c: &mut Cpu, pc: u32) { f_80265bbc_b(c, pc) }
#[inline(always)]
fn f_80265bbc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265bbc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265bbc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265bbc => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0xffff8810);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80265be4; continue; }
                pc = 0x80265bdc;
            }
            0x80265bdc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265c00; continue;
                pc = 0x80265be4;
            }
            0x80265be4 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80265bf8; continue; }
                pc = 0x80265bf0;
            }
            0x80265bf0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80265c00; continue;
                pc = 0x80265bf8;
            }
            0x80265bf8 => { ins += 1;
                c.lr = 0x80265bfc; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f0438(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265bfc;
            }
            0x80265bfc => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80265c00;
            }
            0x80265c00 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265bbc: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerAllStart (0x80265c10, 116 bytes)
#[inline(never)]
pub fn f_80265c10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265c10) { return; }
        if c.prof { crate::prof::enter(c, 0x80265c10); f_80265c10_at(c, 0x80265c10); crate::prof::exit(c, 0x80265c10); return; }
    }
    f_80265c10_b(c, 0x80265c10);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265c10_at(c: &mut Cpu, pc: u32) { f_80265c10_b(c, pc) }
#[inline(always)]
fn f_80265c10_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265c10;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265c10, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265c10 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0xffff8810);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x80265c74; continue; }
                pc = 0x80265c30;
            }
            0x80265c30 => { ins += 1;
                c.lr = 0x80265c34; { c.steps = st; c.cycles = ins; c.cr = cr; f_800ff560(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265c34;
            }
            0x80265c34 => { ins += 8;
                c.r[5] = 0x80260000;
                c.r[4] = c.r[3] | c.r[3];
                c.r[8] = c.r[5].wrapping_add(0x5db0);
                c.r[3] = 0x1;
                c.r[5] = 0x4000;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x80265c54; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f07a8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265c54;
            }
            0x80265c54 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80265c74; continue; }
                pc = 0x80265c5c;
            }
            0x80265c5c => { ins += 6;
                c.r[5] = 0x80480000;
                c.r[4] = 0x1;
                c.r[5] = c.r[5].wrapping_add(0xffff8810);
                c.stw(c.r[5].wrapping_add(0xc), c.r[3]);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x80265c74; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f0654(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265c74;
            }
            0x80265c74 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265c10: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerAllInit (0x80265c84, 208 bytes)
#[inline(never)]
pub fn f_80265c84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265c84) { return; }
        if c.prof { crate::prof::enter(c, 0x80265c84); f_80265c84_at(c, 0x80265c84); crate::prof::exit(c, 0x80265c84); return; }
    }
    f_80265c84_b(c, 0x80265c84);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265c84_at(c: &mut Cpu, pc: u32) { f_80265c84_b(c, pc) }
#[inline(always)]
fn f_80265c84_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265c84;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265c84, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265c84 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.lr = 0x80265ca0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80077ba8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265ca0;
            }
            0x80265ca0 => { ins += 12;
                c.r[4] = c.r[3] ^ 0x80000000;
                c.r[0] = 0x43300000;
                c.r[3] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[4]);
                c.r[31] = c.r[3].wrapping_add(0xffff8810);
                { let ea = c.r[2].wrapping_add(0xffffb050); c.f[1] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                { let v = s(c.f[0] - c.f[1]); c.f[31] = v; c.ps1[31] = v; }
                if cr_bit(cr, 2) { pc = 0x80265cf8; continue; }
                pc = 0x80265cd0;
            }
            0x80265cd0 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x80265cd8; continue; }
                pc = 0x80265cd4;
            }
            0x80265cd4 => { ins += 1;
                c.lr = 0x80265cd8; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f05a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265cd8;
            }
            0x80265cd8 => { ins += 8;
                c.r[0] = 0x0;
                c.r[3] = 0x80480000;
                { let ea = c.r[3].wrapping_add(0xffff8810); c.stb(ea, c.r[0]); c.r[3] = ea; }
                { let ea = c.r[2].wrapping_add(0xffffb038); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                pc = 0x80265cf8;
            }
            0x80265cf8 => { ins += 7;
                { let ea = c.r[2].wrapping_add(0xffffb038); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[0] = 0x0;
                c.r[3] = 0x80480000;
                { let ea = c.r[3].wrapping_add(0xffff8810); c.stb(ea, c.r[0]); c.r[3] = ea; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[31], c.f[0]);
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                if !cr_bit(cr, 0) { pc = 0x80265d28; continue; }
                pc = 0x80265d14;
            }
            0x80265d14 => { ins += 5;
                { let ea = c.r[2].wrapping_add(0xffffb048); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[0] = 0x1;
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80265d30; continue;
                pc = 0x80265d28;
            }
            0x80265d28 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[31]; c.stfs(ea, v); }
                pc = 0x80265d30;
            }
            0x80265d30 => { ins += 10;
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                { let ea = c.r[1].wrapping_add(0x20); c.f[31] = c.lfd(ea); }
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265c84: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerAllTerminate (0x80265d54, 92 bytes)
#[inline(never)]
pub fn f_80265d54(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265d54) { return; }
        if c.prof { crate::prof::enter(c, 0x80265d54); f_80265d54_at(c, 0x80265d54); crate::prof::exit(c, 0x80265d54); return; }
    }
    f_80265d54_b(c, 0x80265d54);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265d54_at(c: &mut Cpu, pc: u32) { f_80265d54_b(c, pc) }
#[inline(always)]
fn f_80265d54_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265d54;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265d54, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265d54 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80480000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xffff8810);
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x80265d7c; continue; }
                pc = 0x80265d78;
            }
            0x80265d78 => { ins += 1;
                c.lr = 0x80265d7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f05a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265d7c;
            }
            0x80265d7c => { ins += 14;
                c.r[0] = 0x0;
                c.r[3] = 0x80480000;
                { let ea = c.r[3].wrapping_add(0xffff8810); c.stb(ea, c.r[0]); c.r[3] = ea; }
                { let ea = c.r[2].wrapping_add(0xffffb038); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265d54: bad pc {:#x}", pc),
        }
    }
}

/// fightTimerThreadFunc (0x80265db0, 132 bytes)
#[inline(never)]
pub fn f_80265db0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265db0) { return; }
        if c.prof { crate::prof::enter(c, 0x80265db0); f_80265db0_at(c, 0x80265db0); crate::prof::exit(c, 0x80265db0); return; }
    }
    f_80265db0_b(c, 0x80265db0);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265db0_at(c: &mut Cpu, pc: u32) { f_80265db0_b(c, pc) }
#[inline(always)]
fn f_80265db0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265db0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265db0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265db0 => { ins += 25;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                { let ea = c.r[2].wrapping_add(0xffffb038); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[30] = c.r[3] | c.r[3];
                { let ea = c.r[2].wrapping_add(0xffffb040); c.f[30] = c.lfd(ea); }
                c.r[31] = 0x43300000;
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffffb03c); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                pc = 0x80265e10; continue;
                pc = 0x80265dec;
            }
            0x80265dec => { ins += 1;
                c.lr = 0x80265df0; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265df0;
            }
            0x80265df0 => { ins += 1;
                c.lr = 0x80265df4; { c.steps = st; c.cycles = ins; c.cr = cr; f_800d3088(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265df4;
            }
            0x80265df4 => { ins += 7;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                { let ea = c.r[30].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[1].wrapping_add(0x8), c.r[31]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[30]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] + c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[30].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80265e10;
            }
            0x80265e10 => { ins += 5;
                { let ea = c.r[30].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[30].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[31] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                if cr_bit(cr, 0) { pc = 0x80265dec; continue; }
                pc = 0x80265e24;
            }
            0x80265e24 => { ins += 2;
                c.r[0] = 0x1;
                c.stb(c.r[30], c.r[0]);
                pc = 0x80265e2c;
            }
            0x80265e2c => { ins += 1;
                c.lr = 0x80265e30; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265e30;
            }
            0x80265e30 => { ins += 1;
                pc = 0x80265e2c; continue;
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265db0: bad pc {:#x}", pc),
        }
    }
}

/// exribbonSetEarthRibbon (0x80265e34, 72 bytes)
#[inline(never)]
pub fn f_80265e34(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265e34) { return; }
        if c.prof { crate::prof::enter(c, 0x80265e34); f_80265e34_at(c, 0x80265e34); crate::prof::exit(c, 0x80265e34); return; }
    }
    f_80265e34_b(c, 0x80265e34);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265e34_at(c: &mut Cpu, pc: u32) { f_80265e34_b(c, pc) }
#[inline(always)]
fn f_80265e34_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265e34;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265e34, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265e34 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80265e48; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011d5d4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265e48;
            }
            0x80265e48 => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x10;
                c.lr = 0x80265e54; { c.steps = st; c.cycles = ins; c.cr = cr; f_80129280(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265e54;
            }
            0x80265e54 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x5));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80265e68; continue; }
                pc = 0x80265e60;
            }
            0x80265e60 => { ins += 2;
                c.r[0] = 0x2d;
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                pc = 0x80265e68;
            }
            0x80265e68 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265e34: bad pc {:#x}", pc),
        }
    }
}

/// exribbonSetNarionalRibbon (0x80265e7c, 72 bytes)
#[inline(never)]
pub fn f_80265e7c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265e7c) { return; }
        if c.prof { crate::prof::enter(c, 0x80265e7c); f_80265e7c_at(c, 0x80265e7c); crate::prof::exit(c, 0x80265e7c); return; }
    }
    f_80265e7c_b(c, 0x80265e7c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265e7c_at(c: &mut Cpu, pc: u32) { f_80265e7c_b(c, pc) }
#[inline(always)]
fn f_80265e7c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265e7c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265e7c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265e7c => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80265e90; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011d5f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265e90;
            }
            0x80265e90 => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x10;
                c.lr = 0x80265e9c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80129280(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265e9c;
            }
            0x80265e9c => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80265eb0; continue; }
                pc = 0x80265ea8;
            }
            0x80265ea8 => { ins += 2;
                c.r[0] = 0x2c;
                c.stb(c.r[3].wrapping_add(0x4), c.r[0]);
                pc = 0x80265eb0;
            }
            0x80265eb0 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265e7c: bad pc {:#x}", pc),
        }
    }
}

/// exribbonSetNo (0x80265ec4, 80 bytes)
#[inline(never)]
pub fn f_80265ec4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265ec4) { return; }
        if c.prof { crate::prof::enter(c, 0x80265ec4); f_80265ec4_at(c, 0x80265ec4); crate::prof::exit(c, 0x80265ec4); return; }
    }
    f_80265ec4_b(c, 0x80265ec4);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265ec4_at(c: &mut Cpu, pc: u32) { f_80265ec4_b(c, pc) }
#[inline(always)]
fn f_80265ec4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265ec4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265ec4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265ec4 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = 0x0;
                c.r[4] = 0x10;
                c.lr = 0x80265ee8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80129280(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265ee8;
            }
            0x80265ee8 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[30]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80265f00; continue; }
                pc = 0x80265ef4;
            }
            0x80265ef4 => { ins += 2;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x80265f00; continue; }
                pc = 0x80265efc;
            }
            0x80265efc => { ins += 1;
                c.stb(c.r[3].wrapping_add(c.r[30]), c.r[31]);
                pc = 0x80265f00;
            }
            0x80265f00 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265ec4: bad pc {:#x}", pc),
        }
    }
}

/// exribbonGetNo (0x80265f14, 56 bytes)
#[inline(never)]
pub fn f_80265f14(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265f14) { return; }
        if c.prof { crate::prof::enter(c, 0x80265f14); f_80265f14_at(c, 0x80265f14); crate::prof::exit(c, 0x80265f14); return; }
    }
    f_80265f14_b(c, 0x80265f14);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265f14_at(c: &mut Cpu, pc: u32) { f_80265f14_b(c, pc) }
#[inline(always)]
fn f_80265f14_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265f14;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265f14, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265f14 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x10;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x80265f34; { c.steps = st; c.cycles = ins; c.cr = cr; f_80129280(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265f34;
            }
            0x80265f34 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[31]));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265f14: bad pc {:#x}", pc),
        }
    }
}

/// exribbonInit (0x80265f4c, 72 bytes)
#[inline(never)]
pub fn f_80265f4c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265f4c) { return; }
        if c.prof { crate::prof::enter(c, 0x80265f4c); f_80265f4c_at(c, 0x80265f4c); crate::prof::exit(c, 0x80265f4c); return; }
    }
    f_80265f4c_b(c, 0x80265f4c);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265f4c_at(c: &mut Cpu, pc: u32) { f_80265f4c_b(c, pc) }
#[inline(always)]
fn f_80265f4c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265f4c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265f4c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265f4c => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80265f6c; continue; }
                pc = 0x80265f60;
            }
            0x80265f60 => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x10;
                c.lr = 0x80265f6c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80129280(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265f6c;
            }
            0x80265f6c => { ins += 4;
                c.r[0] = 0xb;
                c.r[4] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x80265f78;
            }
            0x80265f78 => { ins += 3;
                c.stb(c.r[3], c.r[4]);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80265f78; continue; }
                pc = 0x80265f84;
            }
            0x80265f84 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265f4c: bad pc {:#x}", pc),
        }
    }
}

/// evolutionPlayWaza (0x80265f94, 700 bytes)
#[inline(never)]
pub fn f_80265f94(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80265f94) { return; }
        if c.prof { crate::prof::enter(c, 0x80265f94); f_80265f94_at(c, 0x80265f94); crate::prof::exit(c, 0x80265f94); return; }
    }
    f_80265f94_b(c, 0x80265f94);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80265f94_at(c: &mut Cpu, pc: u32) { f_80265f94_b(c, pc) }
#[inline(never)]
fn f_80265f94_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80265f94;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80265f94, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80265f94 => { ins += 13;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0xfa;
                c.r[5] = 0x66;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.lr = 0x80265fc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265fc8;
            }
            0x80265fc8 => { ins += 4;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[3] = 0x10000;
                c.r[3] = c.r[3].wrapping_add(0xffffffff);
                if cr_bit(cr, 2) { pc = 0x80265fdc; continue; }
                pc = 0x80265fd8;
            }
            0x80265fd8 => { ins += 1;
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x80265fdc;
            }
            0x80265fdc => { ins += 3;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x80265fe8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801de190(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265fe8;
            }
            0x80265fe8 => { ins += 5;
                c.r[4] = c.lhz_t(ins, c.r[2].wrapping_add(0xffffb058));
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x80265ffc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80265ffc;
            }
            0x80265ffc => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.lr = 0x80266008; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da4e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266008;
            }
            0x80266008 => { ins += 5;
                c.r[4] = c.lhz_t(ins, c.r[2].wrapping_add(0xffffb058));
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = 0x4;
                c.r[30] = c.r[4] | c.r[4];
                c.lr = 0x8026601c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026601c;
            }
            0x8026601c => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffffb05c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x80266028; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c41c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266028;
            }
            0x80266028 => { ins += 1;
                pc = 0x8026605c; continue;
                pc = 0x8026602c;
            }
            0x8026602c => { ins += 1;
                c.lr = 0x80266030; { c.steps = st; c.cycles = ins; c.cr = cr; f_801db088(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266030;
            }
            0x80266030 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x4;
                c.lr = 0x80266040; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266040;
            }
            0x80266040 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80266058; continue; }
                pc = 0x80266048;
            }
            0x80266048 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x4;
                c.lr = 0x80266058; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266058;
            }
            0x80266058 => { ins += 1;
                c.lr = 0x8026605c; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026605c;
            }
            0x8026605c => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80266064; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c40f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266064;
            }
            0x80266064 => { ins += 2;
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8026602c; continue; }
                pc = 0x8026606c;
            }
            0x8026606c => { ins += 2;
                c.r[3] = 0xfa;
                c.lr = 0x80266074; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011e778(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266074;
            }
            0x80266074 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80266084; continue; }
                pc = 0x8026607c;
            }
            0x8026607c => { ins += 2;
                c.r[28] = 0x0;
                pc = 0x802660c8; continue;
                pc = 0x80266084;
            }
            0x80266084 => { ins += 1;
                c.lr = 0x80266088; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011e15c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266088;
            }
            0x80266088 => { ins += 3;
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80266094; { c.steps = st; c.cycles = ins; c.cr = cr; f_80166a28(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266094;
            }
            0x80266094 => { ins += 1;
                pc = 0x802660c8; continue;
                pc = 0x80266098;
            }
            0x80266098 => { ins += 1;
                c.lr = 0x8026609c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801db088(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026609c;
            }
            0x8026609c => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x4;
                c.lr = 0x802660ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802660ac;
            }
            0x802660ac => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802660c4; continue; }
                pc = 0x802660b4;
            }
            0x802660b4 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x4;
                c.lr = 0x802660c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802660c4;
            }
            0x802660c4 => { ins += 1;
                c.lr = 0x802660c8; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802660c8;
            }
            0x802660c8 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x802660d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801666bc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802660d0;
            }
            0x802660d0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x2);
                if cr_bit(cr, 2) { pc = 0x80266098; continue; }
                pc = 0x802660d8;
            }
            0x802660d8 => { ins += 4;
                c.r[3] = 0x44ba;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x802660e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801067e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802660e8;
            }
            0x802660e8 => { ins += 1;
                pc = 0x8026611c; continue;
                pc = 0x802660ec;
            }
            0x802660ec => { ins += 1;
                c.lr = 0x802660f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801db088(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802660f0;
            }
            0x802660f0 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x4;
                c.lr = 0x80266100; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266100;
            }
            0x80266100 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80266118; continue; }
                pc = 0x80266108;
            }
            0x80266108 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x4;
                c.lr = 0x80266118; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266118;
            }
            0x80266118 => { ins += 1;
                c.lr = 0x8026611c; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026611c;
            }
            0x8026611c => { ins += 1;
                c.lr = 0x80266120; { c.steps = st; c.cycles = ins; c.cr = cr; f_80106934(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266120;
            }
            0x80266120 => { ins += 5;
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802660ec; continue; }
                pc = 0x80266134;
            }
            0x80266134 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x8026613c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801065b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026613c;
            }
            0x8026613c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x1);
                if cr_bit(cr, 2) { pc = 0x80266168; continue; }
                pc = 0x80266144;
            }
            0x80266144 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8026617c; continue; }
                pc = 0x80266148;
            }
            0x80266148 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 0) { pc = 0x80266154; continue; }
                pc = 0x80266150;
            }
            0x80266150 => { ins += 1;
                pc = 0x8026617c; continue;
                pc = 0x80266154;
            }
            0x80266154 => { ins += 3;
                c.r[3] = 0x5d;
                c.r[4] = 0x3d2;
                c.lr = 0x80266160; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266160;
            }
            0x80266160 => { ins += 2;
                c.r[3] = 0x44bc;
                pc = 0x80266180; continue;
                pc = 0x80266168;
            }
            0x80266168 => { ins += 3;
                c.r[3] = 0x5d;
                c.r[4] = 0x3d2;
                c.lr = 0x80266174; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266174;
            }
            0x80266174 => { ins += 2;
                c.r[3] = 0x44bb;
                pc = 0x80266180; continue;
                pc = 0x8026617c;
            }
            0x8026617c => { ins += 1;
                c.r[3] = 0x44b9;
                pc = 0x80266180;
            }
            0x80266180 => { ins += 3;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x8026618c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801067e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026618c;
            }
            0x8026618c => { ins += 1;
                pc = 0x802661c0; continue;
                pc = 0x80266190;
            }
            0x80266190 => { ins += 1;
                c.lr = 0x80266194; { c.steps = st; c.cycles = ins; c.cr = cr; f_801db088(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266194;
            }
            0x80266194 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x4;
                c.lr = 0x802661a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802661a4;
            }
            0x802661a4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802661bc; continue; }
                pc = 0x802661ac;
            }
            0x802661ac => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x4;
                c.lr = 0x802661bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802661bc;
            }
            0x802661bc => { ins += 1;
                c.lr = 0x802661c0; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802661c0;
            }
            0x802661c0 => { ins += 1;
                c.lr = 0x802661c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80106934(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802661c4;
            }
            0x802661c4 => { ins += 5;
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80266190; continue; }
                pc = 0x802661d8;
            }
            0x802661d8 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x802661e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801065b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802661e0;
            }
            0x802661e0 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffffb05c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x3;
                c.lr = 0x802661ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c41c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802661ec;
            }
            0x802661ec => { ins += 1;
                pc = 0x80266220; continue;
                pc = 0x802661f0;
            }
            0x802661f0 => { ins += 1;
                c.lr = 0x802661f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801db088(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802661f4;
            }
            0x802661f4 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x4;
                c.lr = 0x80266204; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266204;
            }
            0x80266204 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8026621c; continue; }
                pc = 0x8026620c;
            }
            0x8026620c => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x4;
                c.lr = 0x8026621c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026621c;
            }
            0x8026621c => { ins += 1;
                c.lr = 0x80266220; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266220;
            }
            0x80266220 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x80266228; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c40f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266228;
            }
            0x80266228 => { ins += 2;
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x802661f0; continue; }
                pc = 0x80266230;
            }
            0x80266230 => { ins += 9;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80265f94: bad pc {:#x}", pc),
        }
    }
}

/// evolutionSequenceEnd (0x80266250, 208 bytes)
#[inline(never)]
pub fn f_80266250(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80266250) { return; }
        if c.prof { crate::prof::enter(c, 0x80266250); f_80266250_at(c, 0x80266250); crate::prof::exit(c, 0x80266250); return; }
    }
    f_80266250_b(c, 0x80266250);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80266250_at(c: &mut Cpu, pc: u32) { f_80266250_b(c, pc) }
#[inline(always)]
fn f_80266250_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80266250;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80266250, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80266250 => { ins += 16;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffae60));
                c.lr = 0x80266268; { c.steps = st; c.cycles = ins; c.cr = cr; f_801653c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266268;
            }
            0x80266268 => { ins += 2;
                c.r[27] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[27]);
                if cr_bit(cr, 2) { pc = 0x8026628c; continue; }
                pc = 0x80266270;
            }
            0x80266270 => { ins += 1;
                c.lr = 0x80266274; { c.steps = st; c.cycles = ins; c.cr = cr; f_801656d8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266274;
            }
            0x80266274 => { ins += 5;
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = 0x1;
                c.r[4] = 0x32;
                c.r[5] = 0xff;
                c.lr = 0x80266288; { c.steps = st; c.cycles = ins; c.cr = cr; f_80165a20(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266288;
            }
            0x80266288 => { ins += 1;
                pc = 0x80266290; continue;
                pc = 0x8026628c;
            }
            0x8026628c => { ins += 1;
                c.r[29] = 0x0;
                pc = 0x80266290;
            }
            0x80266290 => { ins += 1;
                c.lr = 0x80266294; { c.steps = st; c.cycles = ins; c.cr = cr; f_801653bc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266294;
            }
            0x80266294 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                if cr_bit(cr, 2) { pc = 0x802662b0; continue; }
                pc = 0x8026629c;
            }
            0x8026629c => { ins += 1;
                c.lr = 0x802662a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801656d8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802662a0;
            }
            0x802662a0 => { ins += 3;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x32;
                c.lr = 0x802662ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_801657d0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802662ac;
            }
            0x802662ac => { ins += 1;
                pc = 0x802662b4; continue;
                pc = 0x802662b0;
            }
            0x802662b0 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x802662b4;
            }
            0x802662b4 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x802662bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801dadc0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802662bc;
            }
            0x802662bc => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802662c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80265f94(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802662c4;
            }
            0x802662c4 => { ins += 1;
                c.lr = 0x802662c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801dac90(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802662c8;
            }
            0x802662c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x0);
                if cr_bit(cr, 2) { pc = 0x802662e0; continue; }
                pc = 0x802662d0;
            }
            0x802662d0 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[5] = c.r[29].rotate_left(0) & 0xff;
                c.r[4] = 0x32;
                c.lr = 0x802662e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80165a20(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802662e0;
            }
            0x802662e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x802662f8; continue; }
                pc = 0x802662e8;
            }
            0x802662e8 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[5] = c.r[30].rotate_left(0) & 0xff;
                c.r[4] = 0x32;
                c.lr = 0x802662f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801659fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802662f8;
            }
            0x802662f8 => { ins += 1;
                c.lr = 0x802662fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_800ff660(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802662fc;
            }
            0x802662fc => { ins += 4;
                c.r[4] = 0x5960000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x8);
                c.lr = 0x8026630c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011288c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026630c;
            }
            0x8026630c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80266250: bad pc {:#x}", pc),
        }
    }
}

/// d2presentOpen (0x80266320, 60 bytes)
#[inline(never)]
pub fn f_80266320(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80266320) { return; }
        if c.prof { crate::prof::enter(c, 0x80266320); f_80266320_at(c, 0x80266320); crate::prof::exit(c, 0x80266320); return; }
    }
    f_80266320_b(c, 0x80266320);
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80266320_at(c: &mut Cpu, pc: u32) { f_80266320_b(c, pc) }
#[inline(always)]
fn f_80266320_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80266320;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80266320, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80266320 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffae60), c.r[3]);
                c.r[3] = 0x8f;
                c.lr = 0x80266338; { c.steps = st; c.cycles = ins; c.cr = cr; f_800ff730(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266338;
            }
            0x80266338 => { ins += 4;
                c.r[3] = 0x5960000;
                c.r[4] = 0x0;
                c.r[3] = c.r[3].wrapping_add(0x9);
                c.lr = 0x80266348; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011288c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80266348;
            }
            0x80266348 => { ins += 1;
                c.lr = 0x8026634c; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8026634c;
            }
            0x8026634c => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80266320: bad pc {:#x}", pc),
        }
    }
}

