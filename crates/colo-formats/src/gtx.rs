//! `GStexture` images (`.gtx` files in FSYS archives, and a few embedded in main.dol `.data`).
//!
//! 0x80-byte big-endian header (`GStextureLoad` 0x800EFD3C, `GStextureGetGXformat` 0x800EF3E0):
//! +0x00 u16 width, +0x02 u16 height, +0x04 u8 bits per pixel, +0x05 u8 mip count, +0x08 u32 GS format
//! (0x00 C4, 0x01 C8, 0x30 C14X2, 0x40 I4, 0x41 IA4, 0x42 I8, 0x43 IA8, 0x44 RGB565, 0x45 RGBA8, 0x90 RGB5A3,
//! 0xB0 CMPR), +0x0C u32 TLUT format (1 IA8, 2 RGB565, 3 RGB5A3), +0x28 u32[8] mip offsets (level 0 = 0x80),
//! +0x48 u32 palette offset. Offsets are relative to the header.

use crate::be::{u16_at, u32_at};
use crate::{gx, Error, Result};

#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    /// RGBA8, row-major, top row first.
    pub rgba: Vec<u8>,
}

/// GS format enum → GX texture format.
pub fn gx_format(gs: u32) -> Option<u32> {
    Some(match gs { 0x00 => 8, 0x01 => 9, 0x30 => 10, 0x40 => 0, 0x41 => 2, 0x42 => 1, 0x43 => 3, 0x44 => 4, 0x45 => 6, 0x90 => 5, 0xB0 => 14, _ => return None })
}

/// Decode mip level 0 of the GTX starting at `d[0]`.
pub fn decode(d: &[u8]) -> Result<Image> {
    let w = u16_at(d, 0, "gtx header")? as usize;
    let h = u16_at(d, 2, "gtx header")? as usize;
    let gs = u32_at(d, 8, "gtx header")?;
    let fmt = gx_format(gs).ok_or_else(|| Error::Invalid(format!("gtx: unknown format {gs:#x}")))?;
    let level0 = u32_at(d, 0x28, "gtx header")? as usize;
    let size = gx::data_size(fmt, w, h).unwrap_or(0);
    let data = d.get(level0..level0 + size).ok_or(Error::Invalid("gtx: image truncated".into()))?;
    let palette = if matches!(fmt, 8..=10) {
        let po = u32_at(d, 0x48, "gtx header")? as usize;
        let count = match fmt { 8 => 16, 9 => 256, _ => 1024 };
        let pf = match u32_at(d, 0x0C, "gtx header")? { 1 => 0, 2 => 1, _ => 2 };
        Some(gx::decode_palette(d.get(po..).unwrap_or(&[]), pf, count)?)
    } else { None };
    Ok(Image { width: w, height: h, rgba: gx::decode(data, fmt, w, h, palette.as_deref())? })
}
