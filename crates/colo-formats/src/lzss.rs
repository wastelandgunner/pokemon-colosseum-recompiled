//! LZSS as used by Genius Sonority's FSYS archives.
//!
//! Container: `"LZSS"`, u32 uncompressed size, u32 compressed size (including this 0x10-byte header),
//! u32 unused, then the stream. Stream is classic Okumura LZSS: 4096-byte ring buffer pre-filled with
//! zeros, write position starts at N-F (4078), flag byte read LSB-first where 1 = literal;
//! a match is two bytes: offset = b0 | (b1 & 0xF0) << 4, length = (b1 & 0x0F) + 3.

use crate::be::u32_at;
use crate::{Error, Result};

pub const MAGIC: &[u8; 4] = b"LZSS";
pub const HEADER_SIZE: usize = 0x10;
const N: usize = 4096;
const F: usize = 18;

pub fn is_lzss(data: &[u8]) -> bool {
    data.len() >= HEADER_SIZE && &data[..4] == MAGIC
}

/// Decompress a complete `"LZSS"` container (header included).
pub fn decompress_container(data: &[u8]) -> Result<Vec<u8>> {
    if !is_lzss(data) {
        let mut m = [0u8; 4];
        m.copy_from_slice(data.get(..4).unwrap_or(&[0; 4]));
        return Err(Error::BadMagic { what: "LZSS", found: m });
    }
    let usize_ = u32_at(data, 4, "LZSS header")? as usize;
    let csize = u32_at(data, 8, "LZSS header")? as usize;
    let end = csize.min(data.len());
    if end < HEADER_SIZE {
        return Err(Error::Invalid(format!("LZSS compressed size {csize:#x} smaller than header")));
    }
    let out = decompress_raw(&data[HEADER_SIZE..end], usize_);
    if out.len() != usize_ {
        return Err(Error::Invalid(format!("LZSS produced {} bytes, header says {}", out.len(), usize_)));
    }
    Ok(out)
}

/// Decompress a raw stream (no header), stopping at `expected` bytes or end of input.
pub fn decompress_raw(src: &[u8], expected: usize) -> Vec<u8> {
    let mut win = [0u8; N];
    let mut r = N - F;
    let mut out = Vec::with_capacity(expected);
    let mut p = 0usize;
    let mut flags: u32 = 0;
    while p < src.len() && out.len() < expected {
        flags >>= 1;
        if flags & 0x100 == 0 {
            flags = src[p] as u32 | 0xFF00;
            p += 1;
        }
        if flags & 1 != 0 {
            let Some(&c) = src.get(p) else { break };
            p += 1;
            out.push(c);
            win[r] = c;
            r = (r + 1) & (N - 1);
        } else {
            if p + 1 >= src.len() {
                break;
            }
            let (b0, b1) = (src[p] as usize, src[p + 1] as usize);
            p += 2;
            let off = b0 | ((b1 & 0xF0) << 4);
            let len = (b1 & 0x0F) + 3;
            for k in 0..len {
                let c = win[(off + k) & (N - 1)];
                out.push(c);
                win[r] = c;
                r = (r + 1) & (N - 1);
            }
        }
    }
    out.truncate(expected);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn literals_and_match() {
        // flags 0b0000_0111 -> 3 literals "abc", then a match copying 3 bytes from ring pos N-F ("abc").
        let off = N - F;
        let stream = [0x07, b'a', b'b', b'c', (off & 0xFF) as u8, (((off >> 4) & 0xF0) | 0x0) as u8];
        assert_eq!(decompress_raw(&stream, 6), b"abcabc");
    }
}
