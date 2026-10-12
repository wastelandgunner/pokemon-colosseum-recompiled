//! `GPT1` particle banks (FSYS type 0x14/0x24, and inside `.pkx`): the HSD particle system's generator
//! templates, their bytecode and texture groups (`GSparticle` load 0x801195AC → `psInitDataBank` 0x8016A17C /
//! `psInitDataBankLocate` 0x8016A2D0).
//!
//! Header: "GPT1", +4 PTL offset, +8 TXG offset, +0xC texture data size, +0x10 REF offset (all from the file
//! start). PTL (version 0x43): u16 version, u16, u32 first id, u32 count, then `count` u32 offsets (from the PTL
//! start, 0 = none) for generator ids `first..first + count`. A generator template is 0x3C bytes plus its
//! bytecode (see [`GenTemplate`]). TXG: u32 count, then u32 offsets (from the TXG start) of texture groups:
//! {u32 image count, u32 GX format, u32 TLUT format, u32 width, u32 height, u32 ?, u32 image offsets[n] (from
//! the TXG start), and for C4/C8/C14X2 `n` palette offsets (or +0x14 u16 of them)}. REF: u32 count, u32 ids.

use crate::be::{u16_at, u32_at};
use crate::{gx, Error, Result};

pub const MAGIC: u32 = 0x4750_5431;

/// One generator template (`psCreateGeneratorID` copies it into a generator).
#[derive(Debug, Clone)]
pub struct GenTemplate {
    /// +0 generator type (low 4 bits: emission shape 0-8; 0x80 kill children with it; 0x800 / 0x1000 /
    /// 0x2000 AppSRT tracking).
    pub kind: u16,
    /// +2 texture group of its particles.
    pub tex_group: u16,
    /// +4 generator lifetime in frames (0 = endless).
    pub life: u16,
    /// +6 particle lifetime (kill timer) in frames.
    pub particle_life: u16,
    /// +8 particle flags (`kind`, see the bit list in NOTES §8 Particles).
    pub flags: u32,
    /// +0xC particle gravity, +0x10 friction.
    pub gravity: f32,
    pub friction: f32,
    /// +0x14 emission velocity.
    pub velocity: [f32; 3],
    /// +0x20 radius (negative: fixed, else random up to it), +0x24 angle (likewise).
    pub radius: f32,
    pub angle: f32,
    /// +0x28 particles per frame (negative: exactly that many, else random 0..rate).
    pub rate: f32,
    /// +0x2C particle size.
    pub size: f32,
    /// +0x30.. shape parameters.
    pub aux: [f32; 3],
    /// +0x3C bytecode (to the end of the section; the interpreter stops at EXIT).
    pub code: Vec<u8>,
}

/// A texture group: decoded RGBA8 images (palettes applied) of one size.
#[derive(Debug, Clone)]
pub struct TexGroup {
    pub format: u32,
    pub width: usize,
    pub height: usize,
    pub images: Vec<Vec<u8>>,
    /// C4/C8/C14X2 groups: the raw index data per image and every palette of the group (a particle's palette
    /// number, opcode 0xE3, picks one; `images` use each image's own).
    pub raw: Vec<Vec<u8>>,
    pub palettes: Vec<Vec<[u8; 4]>>,
}

impl TexGroup {
    /// Image `pose` decoded with palette `pal` (CI formats only; others ignore it).
    pub fn with_palette(&self, pose: usize, pal: usize) -> Option<Vec<u8>> {
        let (raw, p) = (self.raw.get(pose)?, self.palettes.get(pal)?);
        gx::decode(raw, self.format, self.width, self.height, Some(p)).ok()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Gpt1 {
    /// Generator templates by id.
    pub generators: Vec<Option<GenTemplate>>,
    pub tex_groups: Vec<Option<TexGroup>>,
    /// REF table: logical id → generator id (the spawn-from-REF opcodes).
    pub refs: Vec<u32>,
}

impl Gpt1 {
    pub fn parse(d: &[u8]) -> Result<Self> {
        if u32_at(d, 0, "gpt1")? != MAGIC { return Err(Error::Invalid("not a GPT1 bank".into())); }
        let ptl = u32_at(d, 4, "gpt1")? as usize;
        let txg = u32_at(d, 8, "gpt1")? as usize;
        let refo = u32_at(d, 0x10, "gpt1")? as usize;
        let version = u16_at(d, ptl, "ptl")?;
        let (first, count, table) = match version {
            0 => (0usize, u32_at(d, ptl + 4, "ptl")? as usize, ptl + 8),
            0x40..=0x43 => (u32_at(d, ptl + 4, "ptl")? as usize, u32_at(d, ptl + 8, "ptl")? as usize, ptl + 0xC),
            v => return Err(Error::Invalid(format!("GPT1: PTL version {v:#x}"))),
        };
        let offs: Vec<usize> = (0..count).map(|i| u32_at(d, table + 4 * i, "ptl").map(|o| o as usize)).collect::<Result<_>>()?;
        let section_end = txg.max(ptl);
        let mut starts: Vec<usize> = offs.iter().filter(|&&o| o != 0).map(|&o| ptl + o).collect();
        starts.sort_unstable();
        let mut generators = vec![None; first + count];
        for (i, &o) in offs.iter().enumerate() {
            if o == 0 { continue; }
            let b = ptl + o;
            let f = |k: usize| u32_at(d, b + k, "generator").map(f32::from_bits);
            let end = starts.iter().copied().find(|&s| s > b).unwrap_or(section_end).max(b + 0x3C);
            generators[first + i] = Some(GenTemplate {
                kind: u16_at(d, b, "generator")?,
                tex_group: u16_at(d, b + 2, "generator")?,
                life: u16_at(d, b + 4, "generator")?,
                particle_life: u16_at(d, b + 6, "generator")?,
                flags: u32_at(d, b + 8, "generator")?,
                gravity: f(0xC)?,
                friction: f(0x10)?,
                velocity: [f(0x14)?, f(0x18)?, f(0x1C)?],
                radius: f(0x20)?,
                angle: f(0x24)?,
                rate: f(0x28)?,
                size: f(0x2C)?,
                aux: [f(0x30)?, f(0x34)?, f(0x38)?],
                code: d.get(b + 0x3C..end.min(d.len())).unwrap_or(&[]).to_vec(),
            });
        }
        let n = u32_at(d, txg, "txg")? as usize;
        let mut tex_groups = vec![];
        for i in 0..n {
            let o = u32_at(d, txg + 4 + 4 * i, "txg")? as usize;
            if o == 0 { tex_groups.push(None); continue; }
            let b = txg + o;
            let nimg = u32_at(d, b, "texgroup")? as usize;
            let format = u32_at(d, b + 4, "texgroup")?;
            let tlut = u32_at(d, b + 8, "texgroup")?;
            let (w, h) = (u32_at(d, b + 0xC, "texgroup")? as usize, u32_at(d, b + 0x10, "texgroup")? as usize);
            let size = gx::data_size(format, w, h).ok_or_else(|| Error::Invalid(format!("GPT1: texture format {format}")))?;
            let (mut images, mut raw, mut palettes) = (vec![], vec![], vec![]);
            if matches!(format, 8..=10) {
                let count = match format { 8 => 16, 9 => 256, _ => 1024 };
                let npal = { let n16 = u16_at(d, b + 0x16, "texgroup").unwrap_or(0) as usize; if n16 == 0 { nimg } else { n16 } };
                for k in 0..npal {
                    let po = u32_at(d, b + 0x18 + 4 * (nimg + k), "texgroup").unwrap_or(0) as usize;
                    if po == 0 { continue; }
                    palettes.push(gx::decode_palette(d.get(txg + po..).unwrap_or(&[]), tlut, count)?);
                }
            }
            for k in 0..nimg {
                let io = u32_at(d, b + 0x18 + 4 * k, "texgroup")? as usize;
                let data = d.get(txg + io..txg + io + size).ok_or(Error::Invalid("GPT1: texture past end".into()))?;
                let palette = if matches!(format, 8..=10) {
                    let po = u32_at(d, b + 0x18 + 4 * (nimg + k), "texgroup").ok().filter(|&p| p != 0)
                        .or_else(|| u32_at(d, b + 0x18 + 4 * nimg, "texgroup").ok()).unwrap_or(0) as usize;
                    let count = match format { 8 => 16, 9 => 256, _ => 1024 };
                    Some(gx::decode_palette(d.get(txg + po..).unwrap_or(&[]), tlut, count)?)
                } else { None };
                images.push(gx::decode(data, format, w, h, palette.as_deref())?);
                if matches!(format, 8..=10) { raw.push(data.to_vec()); }
            }
            tex_groups.push(Some(TexGroup { format, width: w, height: h, images, raw, palettes }));
        }
        let refs = if refo != 0 && refo + 4 <= d.len() {
            let n = u32_at(d, refo, "ref")? as usize;
            (0..n).map(|i| u32_at(d, refo + 4 + 4 * i, "ref")).collect::<Result<_>>()?
        } else { vec![] };
        Ok(Gpt1 { generators, tex_groups, refs })
    }
}

/// Bytes an instruction takes (opcode included), as `psInterpretParticle0` reads them.
pub fn op_len(code: &[u8], pc: usize) -> usize {
    let op = code[pc];
    let at = |k: usize| code.get(pc + k).copied().unwrap_or(0);
    let time = |k: usize| if at(k) & 0x80 != 0 { 2 } else { 1 };
    let pop = |m: u8| (op & m).count_ones() as usize;
    match op {
        0x00..=0x7F => 1 + (op & 0x20 != 0) as usize + ((op & 0xC0) == 0x40) as usize,
        0x80..=0x9F => 1 + 4 * pop(7),
        0xA0 => 1 + time(1) + 4,
        0xA2 | 0xA3 | 0xA9 | 0xAB | 0xE8 => 5,
        0xA4 | 0xA5 | 0xB9 | 0xBC | 0xF1 | 0xF2 => 3,
        0xA6 | 0xAA | 0xBA | 0xBB | 0xE0 => 5,
        0xA7 | 0xB7 | 0xBF | 0xE3 | 0xE4 | 0xE5 | 0xFA => 2,
        0xA8 | 0xBE => 13,
        0xAC => 1 + time(1) + 8,
        0xB3 => 1 + time(1) + 3,
        0xB6 => 1 + time(1) + 4,
        0xB8 => 10,
        0xBD => 9,
        0xC0..=0xDF => 1 + time(1) + pop(0xF),
        0xE9 => 3 + (at(1) & 0xF).count_ones() as usize,
        0xEA | 0xEB => { let t = time(1); 2 + t + (at(1 + t) & 9).count_ones() as usize }
        0xED => 10,
        0xEF | 0xF0 => 4,
        0xF3 => { let t = if at(10) & 0x80 != 0 { 2 } else { 1 }; 10 + t }
        0xF4 => 17,
        _ => 1,
    }
}

/// One-line-per-instruction listing (for `colo-tool gpt1`).
pub fn disassemble(code: &[u8]) -> Vec<String> {
    let mut out = vec![];
    let mut pc = 0;
    while pc < code.len() {
        let n = op_len(code, pc).max(1);
        let bytes: Vec<String> = code[pc..(pc + n).min(code.len())].iter().map(|b| format!("{b:02x}")).collect();
        out.push(format!("{pc:04x}  {}", bytes.join(" ")));
        if matches!(code[pc], 0xFE | 0xFF) { break; }
        pc += n;
    }
    out
}
