//! Nintendo REL modules. Colosseum uses them as relocatable *data* blobs (`common_rel`, per-map `.rel`).
//!
//! Header (v3): id @0, section count @0x0C, section table offset @0x10, version @0x1C, bss size @0x20,
//! relocation offset @0x24, import offset @0x28, import size @0x2C. Section table: n × {u32 offset|exec, u32 size}.
//! Imports: {u32 module id, u32 offset of its relocation list}. Relocation entries: {u16 delta, u8 type,
//! u8 section, u32 addend}; type 0xCA = start section, 0xCB = end, 0xC9 = nop; 1 = ADDR32,
//! 4 = ADDR16_LO, 6 = ADDR16_HA, 10 = REL24 (map RELs contain PowerPC code that references main.dol).
//! A pointer stored in the data is resolved by its relocation: (target section, addend).

use crate::be::{u16_at, u32_at, u8_at};
use crate::{Error, Result};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub struct Section { pub offset: usize, pub size: usize, pub exec: bool }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reloc {
    /// Module this relocation targets (0 = main.dol, otherwise a REL id; self = own id).
    pub module: u32,
    pub kind: u8,
    pub target_section: u8,
    pub addend: u32,
}

pub struct Rel<'a> {
    pub data: &'a [u8],
    pub id: u32,
    pub version: u32,
    pub sections: Vec<Section>,
    /// (section index, offset within section) -> relocation applied there.
    pub relocs: BTreeMap<(u8, usize), Reloc>,
}

pub const R_PPC_ADDR32: u8 = 1;

impl<'a> Rel<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        let id = u32_at(data, 0, "REL header")?;
        let nsec = u32_at(data, 0x0C, "REL header")? as usize;
        let sec_off = u32_at(data, 0x10, "REL header")? as usize;
        let version = u32_at(data, 0x1C, "REL header")?;
        let imp_off = u32_at(data, 0x28, "REL header")? as usize;
        let imp_size = u32_at(data, 0x2C, "REL header")? as usize;
        if nsec > 64 { return Err(Error::Invalid(format!("REL: {nsec} sections"))); }
        let mut sections = Vec::with_capacity(nsec);
        for i in 0..nsec {
            let o = u32_at(data, sec_off + 8 * i, "REL sections")?;
            let s = u32_at(data, sec_off + 8 * i + 4, "REL sections")? as usize;
            sections.push(Section { offset: (o & !1) as usize, size: s, exec: o & 1 != 0 });
        }
        let mut relocs = BTreeMap::new();
        for k in 0..imp_size / 8 {
            let module = u32_at(data, imp_off + 8 * k, "REL imports")?;
            let mut p = u32_at(data, imp_off + 8 * k + 4, "REL imports")? as usize;
            let (mut sec, mut pos) = (0u8, 0usize);
            loop {
                let delta = u16_at(data, p, "REL reloc")? as usize;
                let kind = u8_at(data, p + 2, "REL reloc")?;
                let tsec = u8_at(data, p + 3, "REL reloc")?;
                let addend = u32_at(data, p + 4, "REL reloc")?;
                p += 8;
                match kind {
                    0xCB => break,
                    0xCA => { sec = tsec; pos = 0; }
                    0xC9 => pos += delta,
                    _ => {
                        pos += delta;
                        relocs.insert((sec, pos), Reloc { module, kind, target_section: tsec, addend });
                    }
                }
            }
        }
        Ok(Rel { data, id, version, sections, relocs })
    }

    pub fn section_bytes(&self, i: usize) -> Option<&'a [u8]> {
        let s = self.sections.get(i)?;
        if s.offset == 0 { return None; } // bss / empty
        self.data.get(s.offset..s.offset + s.size)
    }

    /// Symbol list in GoD Tool's numbering (`XGRelocationTable.parsePointers`): distinct relocation targets
    /// in order of first appearance, scanning the relocation stream from the header's relocation offset
    /// up to the first end marker. Returns file offsets. Used to address `common_rel` tables by index.
    pub fn god_symbols(&self) -> Vec<usize> {
        let mut out: Vec<usize> = vec![];
        let Ok(mut p) = u32_at(self.data, 0x24, "REL header").map(|x| x as usize) else { return out };
        while p + 8 <= self.data.len() {
            let kind = self.data[p + 2];
            let sec = self.data[p + 3] as usize;
            let addend = u32::from_be_bytes([self.data[p + 4], self.data[p + 5], self.data[p + 6], self.data[p + 7]]) as usize;
            p += 8;
            if kind == 0xCB { break; }
            if (1..=13).contains(&kind) {
                let base = self.sections.get(sec).map(|s| s.offset).unwrap_or(0);
                let fo = base.wrapping_add(addend);
                if !out.contains(&fo) { out.push(fo); }
            }
        }
        out
    }

    /// main.dol address a relocation at (section, offset) points to, if it targets module 0.
    pub fn main_pointer_at(&self, section: u8, offset: usize) -> Option<u32> {
        self.relocs.get(&(section, offset)).filter(|r| r.module == 0 && r.kind == R_PPC_ADDR32).map(|r| r.addend)
    }

    /// Section containing a file offset.
    pub fn locate(&self, file_offset: usize) -> Option<(u8, usize)> {
        self.sections.iter().enumerate().find(|(_, s)| s.offset != 0 && file_offset >= s.offset && file_offset < s.offset + s.size)
            .map(|(i, s)| (i as u8, file_offset - s.offset))
    }

    /// Resolve the pointer stored at (section, offset), if a relocation exists there.
    pub fn pointer_at(&self, section: u8, offset: usize) -> Option<(u8, usize)> {
        self.relocs.get(&(section, offset)).filter(|r| r.kind == R_PPC_ADDR32 && r.module == self.id)
            .map(|r| (r.target_section, r.addend as usize))
    }

    /// File offset of (section, offset).
    pub fn file_offset(&self, section: u8, offset: usize) -> Option<usize> {
        let s = self.sections.get(section as usize)?;
        (s.offset != 0).then_some(s.offset + offset)
    }
    pub fn u32_in(&self, section: u8, offset: usize) -> Result<u32> {
        let o = self.file_offset(section, offset).ok_or_else(|| Error::Invalid(format!("REL section {section} empty")))?;
        u32_at(self.data, o, "REL data")
    }
}
