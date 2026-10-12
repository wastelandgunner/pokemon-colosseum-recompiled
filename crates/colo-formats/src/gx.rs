//! GameCube GX texture decoding to RGBA8.
//!
//! Formats: I4 (0), I8 (1), IA4 (2), IA8 (3), RGB565 (4), RGB5A3 (5), RGBA8 (6), C4 (8), C8 (9), C14X2 (10),
//! CMPR (14). Textures are stored in tiles (blocks) whose size depends on the format.
//! Palette formats: IA8 (0), RGB565 (1), RGB5A3 (2).

use crate::{Error, Result};

pub fn format_name(f: u32) -> &'static str {
    match f { 0 => "I4", 1 => "I8", 2 => "IA4", 3 => "IA8", 4 => "RGB565", 5 => "RGB5A3", 6 => "RGBA8",
              8 => "C4", 9 => "C8", 10 => "C14X2", 14 => "CMPR", _ => "?" }
}

/// (block width, block height, bits per pixel)
fn block_info(f: u32) -> Option<(usize, usize, usize)> {
    Some(match f {
        0 | 8 | 14 => (8, 8, 4),
        1 | 2 | 9 => (8, 4, 8),
        3 | 4 | 5 | 10 => (4, 4, 16),
        6 => (4, 4, 32),
        _ => return None,
    })
}

/// Bytes occupied by a `w`×`h` texture in format `f` (padded to whole blocks).
pub fn data_size(f: u32, w: usize, h: usize) -> Option<usize> {
    let (bw, bh, bpp) = block_info(f)?;
    Some(w.div_ceil(bw) * bw * h.div_ceil(bh) * bh * bpp / 8)
}

fn rgb565(v: u16) -> [u8; 4] {
    let r = ((v >> 11) & 0x1F) as u8; let g = ((v >> 5) & 0x3F) as u8; let b = (v & 0x1F) as u8;
    [(r << 3) | (r >> 2), (g << 2) | (g >> 4), (b << 3) | (b >> 2), 255]
}
fn rgb5a3(v: u16) -> [u8; 4] {
    if v & 0x8000 != 0 {
        let r = ((v >> 10) & 0x1F) as u8; let g = ((v >> 5) & 0x1F) as u8; let b = (v & 0x1F) as u8;
        [(r << 3) | (r >> 2), (g << 3) | (g >> 2), (b << 3) | (b >> 2), 255]
    } else {
        let a = ((v >> 12) & 7) as u8; let r = ((v >> 8) & 0xF) as u8; let g = ((v >> 4) & 0xF) as u8; let b = (v & 0xF) as u8;
        [r * 17, g * 17, b * 17, (a << 5) | (a << 2) | (a >> 1)]
    }
}
fn ia8(v: u16) -> [u8; 4] { let a = (v >> 8) as u8; let i = v as u8; [i, i, i, a] }

pub fn decode_palette(data: &[u8], format: u32, count: usize) -> Result<Vec<[u8; 4]>> {
    (0..count).map(|i| {
        let v = u16::from_be_bytes([*data.get(2 * i).ok_or(Error::Invalid("palette truncated".into()))?, data[2 * i + 1]]);
        Ok(match format { 0 => ia8(v), 1 => rgb565(v), 2 => rgb5a3(v), _ => return Err(Error::Invalid(format!("palette format {format}"))) })
    }).collect()
}

/// Decode to tightly packed RGBA8 (row-major, top row first).
pub fn decode(data: &[u8], format: u32, w: usize, h: usize, palette: Option<&[[u8; 4]]>) -> Result<Vec<u8>> {
    let (bw, bh, bpp) = block_info(format).ok_or_else(|| Error::Invalid(format!("unsupported GX format {format}")))?;
    let need = data_size(format, w, h).unwrap();
    if data.len() < need {
        return Err(Error::Truncated { what: "GX texture", offset: 0, need, have: data.len() });
    }
    let mut out = vec![0u8; w * h * 4];
    let mut put = |x: usize, y: usize, c: [u8; 4]| {
        if x < w && y < h { out[(y * w + x) * 4..(y * w + x) * 4 + 4].copy_from_slice(&c); }
    };
    let pal = |i: usize| palette.and_then(|p| p.get(i).copied()).unwrap_or([255, 0, 255, 255]);
    let blocks_x = w.div_ceil(bw);
    let blocks_y = h.div_ceil(bh);
    let block_bytes = bw * bh * bpp / 8;
    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            let bi = by * blocks_x + bx;
            let x0 = bx * bw;
            let y0 = by * bh;
            match format {
                6 => {
                    // RGBA8: 64 bytes block: 32 bytes AR pairs then 32 bytes GB pairs
                    let b = &data[bi * 64..bi * 64 + 64];
                    for p in 0..16 {
                        let (a, r) = (b[2 * p], b[2 * p + 1]);
                        let (g, bl) = (b[32 + 2 * p], b[32 + 2 * p + 1]);
                        put(x0 + p % 4, y0 + p / 4, [r, g, bl, a]);
                    }
                }
                14 => {
                    // CMPR: 2x2 sub-blocks of DXT1
                    let b = &data[bi * 32..bi * 32 + 32];
                    for sb in 0..4 {
                        let s = &b[sb * 8..sb * 8 + 8];
                        let c0 = u16::from_be_bytes([s[0], s[1]]);
                        let c1 = u16::from_be_bytes([s[2], s[3]]);
                        let (a, bb) = (rgb565(c0), rgb565(c1));
                        // GX blends 5/8 + 3/8 (not DXT1's thirds); in 3-colour mode index 3 is the average colour
                        // with alpha 0, not DXT1's transparent black (Dolphin TextureDecoder_Common, hardware-verified)
                        let blend = |x: u8, y: u8| ((x as u16 * 5 + y as u16 * 3) >> 3) as u8;
                        let avg = |x: u8, y: u8| ((x as u16 + y as u16) / 2) as u8;
                        let cols = if c0 > c1 {
                            [a, bb, [blend(a[0], bb[0]), blend(a[1], bb[1]), blend(a[2], bb[2]), 255],
                             [blend(bb[0], a[0]), blend(bb[1], a[1]), blend(bb[2], a[2]), 255]]
                        } else {
                            let m = [avg(a[0], bb[0]), avg(a[1], bb[1]), avg(a[2], bb[2])];
                            [a, bb, [m[0], m[1], m[2], 255], [m[0], m[1], m[2], 0]]
                        };
                        for row in 0..4 {
                            let bits = s[4 + row];
                            for col in 0..4 {
                                let idx = (bits >> (6 - 2 * col)) & 3;
                                put(x0 + (sb % 2) * 4 + col, y0 + (sb / 2) * 4 + row, cols[idx as usize]);
                            }
                        }
                    }
                }
                _ => {
                    let b = &data[bi * block_bytes..bi * block_bytes + block_bytes];
                    for p in 0..bw * bh {
                        let (x, y) = (x0 + p % bw, y0 + p / bw);
                        let c = match format {
                            0 => { let v = (b[p / 2] >> if p % 2 == 0 { 4 } else { 0 }) & 0xF; let i = v * 17; [i, i, i, i] }
                            1 => { let i = b[p]; [i, i, i, i] }
                            2 => { let a = (b[p] >> 4) * 17; let i = (b[p] & 0xF) * 17; [i, i, i, a] }
                            3 => ia8(u16::from_be_bytes([b[2 * p], b[2 * p + 1]])),
                            4 => rgb565(u16::from_be_bytes([b[2 * p], b[2 * p + 1]])),
                            5 => rgb5a3(u16::from_be_bytes([b[2 * p], b[2 * p + 1]])),
                            8 => pal(((b[p / 2] >> if p % 2 == 0 { 4 } else { 0 }) & 0xF) as usize),
                            9 => pal(b[p] as usize),
                            10 => pal((u16::from_be_bytes([b[2 * p], b[2 * p + 1]]) & 0x3FFF) as usize),
                            _ => unreachable!(),
                        };
                        put(x, y, c);
                    }
                }
            }
        }
    }
    Ok(out)
}
