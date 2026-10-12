//! `main.dol` (GameCube executable): 7 text + 11 data sections, read by virtual address.

use crate::be::u32_at;
use crate::{Error, Result};

pub struct Dol {
    pub bytes: Vec<u8>,
    /// (file offset, address, size)
    sections: Vec<(usize, u32, usize)>,
}

impl Dol {
    pub fn parse(bytes: Vec<u8>) -> Result<Self> {
        let mut sections = vec![];
        for i in 0..18 {
            let off = u32_at(&bytes, 4 * i, "dol header")? as usize;
            let addr = u32_at(&bytes, 0x48 + 4 * i, "dol header")?;
            let size = u32_at(&bytes, 0x90 + 4 * i, "dol header")? as usize;
            if size > 0 { sections.push((off, addr, size)); }
        }
        Ok(Dol { bytes, sections })
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
