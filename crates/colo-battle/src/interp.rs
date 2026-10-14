//! An interpreter for code the lifter does not have: the RELs the game links at run time (common_rel and the
//! map RELs: their `_prolog` / `_epilog` / `_unresolved`, relocated in memory by OSLink). `lifted::dispatch`
//! falls back to it for addresses outside main.dol's functions.
//!
//! Across all 169 RELs on the disc the code uses only addis, addi, or, lwz, stw, stwu and blr; anything else
//! stops with the instruction so it can be added.

use crate::ppc::Cpu;

/// Run the function at `a` until it returns to the caller's LR.
pub fn call(c: &mut Cpu, a: u32) {
    let ret = c.lr;
    let mut pc = a;
    loop {
        if pc == ret { return; }
        let i = c.m.u32(pc);
        let op = i >> 26;
        let rd = ((i >> 21) & 31) as usize;
        let ra = ((i >> 16) & 31) as usize;
        let rb = ((i >> 11) & 31) as usize;
        let simm = i as u16 as i16 as i32 as u32;
        let ra0 = |c: &Cpu| if ra == 0 { 0 } else { c.r[ra] };
        c.steps += 1;
        let mut next = pc.wrapping_add(4);
        match op {
            14 => c.r[rd] = ra0(c).wrapping_add(simm),                       // addi
            15 => c.r[rd] = ra0(c).wrapping_add(simm << 16),                 // addis
            32 => c.r[rd] = c.m.u32(ra0(c).wrapping_add(simm)),              // lwz
            36 => { let ea = ra0(c).wrapping_add(simm); let v = c.r[rd]; c.m.set_u32(ea, v); } // stw
            37 => { let ea = c.r[ra].wrapping_add(simm); let v = c.r[rd]; c.m.set_u32(ea, v); c.r[ra] = ea; } // stwu
            31 if (i >> 1) & 0x3FF == 444 => {                                // or (rs = rd field)
                let v = c.r[rd] | c.r[rb];
                c.r[ra] = v;
                if i & 1 != 0 { c.rc(v); }
            }
            19 if (i >> 1) & 0x3FF == 16 && (i >> 21) & 0x14 == 0x14 => {     // blr (unconditional)
                next = c.lr;
                if i & 1 != 0 { c.lr = pc.wrapping_add(4); }
            }
            18 => {                                                            // b / bl
                let li = ((i & 0x03FF_FFFC) << 6) as i32 >> 6;
                let t = if i & 2 != 0 { li as u32 } else { pc.wrapping_add(li as u32) };
                if i & 1 != 0 {
                    c.lr = pc.wrapping_add(4);
                    if crate::lifted::RANGES.iter().any(|r| r.0 == t) {
                        crate::lifted::dispatch(c, t);
                        next = pc.wrapping_add(4);
                    } else { next = t; }
                } else { next = t; }
            }
            _ => panic!("interp: instruction {i:#010x} at {pc:#010x} not implemented"),
        }
        pc = next;
    }
}
