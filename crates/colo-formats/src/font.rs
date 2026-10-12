//! The game's bitmap fonts (`GSmsgFontOpen` 0x800FC39C, `_msgGetCodeInfo` 0x800FDF1C, renderer
//! 0x800FC7E0, glyph blit 0x800FD69C). Embedded in main.dol as chains of blocks (0x8027A500: ids 0 22 px,
//! 1 18 px, 3 18 px, 2 13 px, 4 13 px; 0x802BD260 / 0x802C0CB0: extra kanji); room `.fnt` files use the
//! same layout.
//!
//! Block: u16 font id, u8 cell width, u8 cell height (line height), u32 offset of the next block (relative,
//! 0 = last). Node at +8: u16 glyph count, u16, s32 bitmap base (relative to the node), 8 runtime bytes,
//! then count × {u16 code (UTF-16), u8 width/advance, u8 rows, u32 (s8 y offset << 24 | bitmap offset)}
//! sorted by code. Bitmaps are rows × ceil(width/2) bytes, 4 bpp coverage, high nibble = left pixel,
//! stored top-cropped (placed `y offset` rows below the pen).

use crate::be::{u16_at, u32_at};
use crate::Result;

#[derive(Debug, Clone)]
pub struct Glyph {
    pub code: u16,
    pub width: u8,
    pub rows: u8,
    pub y_offset: i8,
    /// rows × width coverage values 0..=15
    pub pixels: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Font {
    pub id: u16,
    pub cell_w: u8,
    pub cell_h: u8,
    pub glyphs: Vec<Glyph>,
}

/// Parse a chain of font blocks starting at `d[0]`.
pub fn parse_chain(d: &[u8]) -> Result<Vec<Font>> {
    let mut out = vec![];
    let mut p = 0usize;
    loop {
        let id = u16_at(d, p, "font block")?;
        let cell_w = d[p + 2];
        let cell_h = d[p + 3];
        let next = u32_at(d, p + 4, "font block")? as usize;
        let node = p + 8;
        let count = u16_at(d, node, "font node")? as usize;
        let base = u32_at(d, node + 4, "font node")? as i32 as isize;
        let mut glyphs = Vec::with_capacity(count);
        for i in 0..count {
            let e = node + 0x10 + 8 * i;
            let code = u16_at(d, e, "font glyph")?;
            let width = d[e + 2];
            let rows = d[e + 3];
            let info = u32_at(d, e + 4, "font glyph")?;
            let y_offset = (info >> 24) as i8;
            let off = (node as isize + base) as usize + (info & 0xFF_FFFF) as usize;
            let rb = (width as usize).div_ceil(2);
            let mut pixels = vec![0u8; width as usize * rows as usize];
            for y in 0..rows as usize {
                for x in 0..width as usize {
                    let b = d.get(off + y * rb + x / 2).copied().unwrap_or(0);
                    pixels[y * width as usize + x] = if x % 2 == 0 { b >> 4 } else { b & 15 };
                }
            }
            glyphs.push(Glyph { code, width, rows, y_offset, pixels });
        }
        out.push(Font { id, cell_w, cell_h, glyphs });
        if next == 0 { break; }
        p += next;
    }
    Ok(out)
}

impl Font {
    pub fn glyph(&self, code: u16) -> Option<&Glyph> {
        self.glyphs.binary_search_by_key(&code, |g| g.code).ok().map(|i| &self.glyphs[i])
    }
}
