//! Build the battle engine's address space from the disc: main.dol at its addresses, common_rel at
//! `REL_BASE` with its relocations applied (as OSLink does), and the (count, base) globals that
//! common_rel's prolog fills in for the `*DataBiosGetPtr` functions.

use crate::mem::{Mem, REL_BASE};
use colo_formats::{common::Common, dol::Dol, rel::{Rel, R_PPC_ADDR32}, Result};

/// main.dol globals that point at common_rel tables: (count pointer global, symbol index). The base
/// pointer global follows at +4. Measured in a running game (Dolphin, 2026-10-12): each count pointer
/// is REL_BASE + the symbol's count slot, each base REL_BASE + the table.
pub const REL_GLOBALS: [(u32, usize); 16] = [
    (0x80478F00, 42), (0x80478F08, 58), (0x80478F10, 48), (0x80478F18, 24), (0x80478F20, 44), (0x80478F28, 56),
    (0x80478F30, 46), (0x80478F38, 26), (0x80478F40, 32), (0x80478F48, 28), (0x80478F50, 50), (0x80478DF8, 62),
    (0x80478F90, 68), (0x80478E58, 70), (0x80478E60, 64), (0x80478E68, 66),
];

pub fn load(dol: &Dol, rel_bytes: &[u8]) -> Result<Mem> {
    let mut m = Mem::new();
    // The header's bss range also spans the initialised .sdata/.sdata2 sections, so bss is mapped as
    // the gaps between sections.
    let (bss, bss_size) = dol.bss();
    let mut secs: Vec<(u32, Vec<u8>)> = dol.sections().map(|(a, b, _)| (a, b.to_vec())).collect();
    secs.sort_by_key(|s| s.0);
    let mut p = bss;
    for (a, b) in &secs {
        if *a >= bss + bss_size || a + b.len() as u32 <= bss { continue; }
        if *a > p { m.map(p, vec![0; (a - p) as usize]); }
        p = p.max(a + b.len() as u32);
    }
    if p < bss + bss_size { m.map(p, vec![0; (bss + bss_size - p) as usize]); }
    for (addr, bytes) in secs { m.map(addr, bytes); }
    let rel = Rel::parse(rel_bytes)?;
    let mut image = rel_bytes.to_vec();
    for (&(sec, pos), r) in &rel.relocs {
        if r.kind != R_PPC_ADDR32 { continue; }
        let Some(s) = rel.sections.get(sec as usize) else { continue };
        if s.offset == 0 { continue; } // bss
        let v = if r.module == 0 {
            r.addend
        } else if r.module == rel.id {
            match rel.sections.get(r.target_section as usize) { Some(t) => REL_BASE + t.offset as u32 + r.addend, None => continue }
        } else { continue };
        let o = s.offset + pos;
        if o + 4 <= image.len() { image[o..o + 4].copy_from_slice(&v.to_be_bytes()); }
    }
    m.map(REL_BASE, image);
    let common = Common::parse(rel_bytes)?;
    for (g, sym) in REL_GLOBALS {
        let (base, _) = common.table(sym)?;
        let cnt = common.symbol(sym + 1).expect("count symbol");
        m.set_u32(g, REL_BASE + cnt as u32);
        m.set_u32(g + 4, REL_BASE + base as u32);
    }
    Ok(m)
}
