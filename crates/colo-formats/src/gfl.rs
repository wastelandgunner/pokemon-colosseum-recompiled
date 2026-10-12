//! `.gfl` lens flare (FSYS type 0x22; `flareInit` 0x801ED680, drawn by `flareDraw` 0x801ED780).
//!
//! Big-endian. +0x00 magic 0x7B1EE3F0, +0x04 u16 texture count, +0x06 u16 element count,
//! +0x08 u32 max distance, +0x0C u32 fade distance, +0x10 u32 glare fade distance (screen px from the
//! 640×480 centre), +0x14 f32 glare alpha (sun in front of the camera), +0x18 f32 glare alpha (sun behind
//! it), +0x1C elements (0xC each: u8 mode, u8 pad, u16 texture slot, f32 position, f32 scale), then
//! textures (8 each: u16 slot, u16 pad, u32 offset of a GTX from the file start).

use crate::be::{u16_at, u32_at};
use crate::{gtx, Error, Result};

pub const MAGIC: u32 = 0x7B1E_E3F0;

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    /// 1 = centred on the sun; otherwise centred on `centre + dir * (pos / 3 * dist)`, dir = unit sun→centre.
    pub mode: u8,
    /// Texture slot (`lbl_8046D630[slot]`).
    pub slot: u16,
    pub pos: f32,
    /// Half-size multiplier: the quad spans ±scale × (texture size / 2).
    pub scale: f32,
}

#[derive(Debug, Clone)]
pub struct Flare {
    pub max_dist: u32,
    pub fade_dist: u32,
    pub glare_fade_dist: u32,
    pub glare_alpha_front: f32,
    pub glare_alpha_behind: f32,
    pub elements: Vec<Element>,
    /// (slot, image)
    pub textures: Vec<(u16, gtx::Image)>,
}

impl Flare {
    pub fn parse(d: &[u8]) -> Result<Self> {
        let w = |o| u32_at(d, o, "gfl");
        let f = |o| Ok::<f32, Error>(f32::from_bits(w(o)?));
        if w(0)? != MAGIC { return Err(Error::Invalid("gfl: bad magic".into())); }
        let ntex = u16_at(d, 4, "gfl")? as usize;
        let nel = u16_at(d, 6, "gfl")? as usize;
        let elements = (0..nel).map(|i| {
            let b = 0x1C + 0xC * i;
            Ok(Element { mode: *d.get(b).ok_or(Error::Invalid("gfl: truncated".into()))?, slot: u16_at(d, b + 2, "gfl")?, pos: f(b + 4)?, scale: f(b + 8)? })
        }).collect::<Result<Vec<_>>>()?;
        let tb = 0x1C + 0xC * nel;
        let textures = (0..ntex).map(|i| {
            let off = w(tb + 8 * i + 4)? as usize;
            Ok((u16_at(d, tb + 8 * i, "gfl")?, gtx::decode(d.get(off..).ok_or(Error::Invalid("gfl: texture offset".into()))?)?))
        }).collect::<Result<Vec<_>>>()?;
        Ok(Flare {
            max_dist: w(8)?, fade_dist: w(0xC)?, glare_fade_dist: w(0x10)?, glare_alpha_front: f(0x14)?, glare_alpha_behind: f(0x18)?,
            elements, textures,
        })
    }

    /// Element fade factor for a sun `dist` px from the centre (`flareDraw`): 1 below `fade_dist`, then
    /// linear to 0 at `max_dist`.
    pub fn element_fade(&self, dist: f32) -> f32 {
        let (m, fd) = (self.max_dist as f32, self.fade_dist as f32);
        if dist < fd { 1.0 } else { (((m - fd) - (dist - fd)) / (m - fd)).clamp(0.0, 1.0) }
    }

    /// Glare alpha (0..1, before × 255) for `dist`, given the base alpha (+0x14 or +0x18); None = not drawn
    /// (`dist >= fade_dist`).
    pub fn glare(&self, dist: f32, base: f32) -> Option<f32> {
        let (fd, gd) = (self.fade_dist as f32, self.glare_fade_dist as f32);
        if dist >= fd { return None; }
        Some(if dist >= gd { (base * (((fd - gd) - (dist - gd)) / (fd - gd))).clamp(0.0, 1.0) } else { base })
    }
}

/// `(u32)(255 * f)` truncated to a byte, as `flareDraw` feeds the vertex colour.
pub fn alpha_byte(f: f32) -> u8 { ((255.0 * f) as u32 & 0xFF) as u8 }
