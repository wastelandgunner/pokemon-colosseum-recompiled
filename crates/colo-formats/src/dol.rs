//! `main.dol` (GameCube executable): 7 text + 11 data sections, read by virtual address.

use crate::be::u32_at;
use crate::{Error, Result};

pub struct Dol {
    pub bytes: Vec<u8>,
    /// (file offset, address, size)
    sections: Vec<(usize, u32, usize)>,
    /// Addresses of the text sections.
    text: Vec<u32>,
}

impl Dol {
    pub fn parse(bytes: Vec<u8>) -> Result<Self> {
        let mut sections = vec![];
        let mut text = vec![];
        for i in 0..18 {
            let off = u32_at(&bytes, 4 * i, "dol header")? as usize;
            let addr = u32_at(&bytes, 0x48 + 4 * i, "dol header")?;
            let size = u32_at(&bytes, 0x90 + 4 * i, "dol header")? as usize;
            if size > 0 {
                sections.push((off, addr, size));
                if i < 7 { text.push(addr); }
            }
        }
        Ok(Dol { bytes, sections, text })
    }

    /// `n` bytes at virtual address `addr` (must lie inside one initialized section).
    pub fn read(&self, addr: u32, n: usize) -> Result<&[u8]> {
        for &(off, a, size) in &self.sections {
            if addr >= a && (addr - a) as usize + n <= size {
                let o = off + (addr - a) as usize;
                return self.bytes.get(o..o + n).ok_or(Error::Invalid(format!("dol: {addr:#x} past end of file")));
            }
        }
        Err(Error::Invalid(format!("dol: {addr:#x}+{n:#x} not in any section")))
    }

    /// Every section: (address, bytes, is text). The first 7 header slots are text.
    pub fn sections(&self) -> impl Iterator<Item = (u32, &[u8], bool)> + '_ {
        self.sections.iter().map(|&(off, a, size)| (a, &self.bytes[off..off + size], self.text.contains(&a)))
    }

    /// (address, size) of the zero-initialised bss range from the header (covers .bss/.sbss/.sbss2).
    pub fn bss(&self) -> (u32, u32) {
        let w = |o: usize| u32::from_be_bytes([self.bytes[o], self.bytes[o + 1], self.bytes[o + 2], self.bytes[o + 3]]);
        (w(0xD8), w(0xDC))
    }

    /// Everything from `addr` to the end of its section.
    pub fn read_rest(&self, addr: u32) -> Result<&[u8]> {
        for &(off, a, size) in &self.sections {
            if addr >= a && ((addr - a) as usize) < size {
                let o = off + (addr - a) as usize;
                return Ok(&self.bytes[o..off + size]);
            }
        }
        Err(Error::Invalid(format!("dol: {addr:#x} not in any section")))
    }
}
