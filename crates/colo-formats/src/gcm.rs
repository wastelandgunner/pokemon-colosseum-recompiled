//! GameCube disc image (.gcm / .iso) reader. Opens the image read-only.
//!
//! Header: game id @0, DOL offset @0x420, FST offset/size @0x424/0x428. FST entries are 12 bytes:
//! (flags<<24 | name offset), (file offset | parent), (size | next index); names are Shift-JIS
//! (Colosseum uses ASCII only).

use crate::be::{u32_at, cstr_at};
use crate::{Error, Result};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct DiscFile {
    /// Path inside the disc, '/'-separated, no leading slash (e.g. `sound/snd_se.samp`).
    pub path: String,
    pub offset: u64,
    pub size: u64,
}

pub struct Disc {
    file: File,
    pub game_id: String,
    pub disc_number: u8,
    pub version: u8,
    pub title: String,
    pub dol_offset: u64,
    pub files: Vec<DiscFile>,
}

impl Disc {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut file = File::open(path)?; // read-only
        let mut hdr = vec![0u8; 0x440];
        file.read_exact(&mut hdr)?;
        let game_id = String::from_utf8_lossy(&hdr[..6]).into_owned();
        let title = cstr_at(&hdr, 0x20, "disc title")?;
        let dol_offset = u32_at(&hdr, 0x420, "disc header")? as u64;
        let fst_off = u32_at(&hdr, 0x424, "disc header")? as u64;
        let fst_size = u32_at(&hdr, 0x428, "disc header")? as usize;
        let mut fst = vec![0u8; fst_size];
        file.seek(SeekFrom::Start(fst_off))?;
        file.read_exact(&mut fst)?;
        let files = parse_fst(&fst)?;
        Ok(Disc { file, game_id, disc_number: hdr[6], version: hdr[7], title, dol_offset, files })
    }

    pub fn find(&self, path: &str) -> Option<&DiscFile> {
        let p = path.trim_start_matches('/');
        self.files.iter().find(|f| f.path.eq_ignore_ascii_case(p))
    }

    pub fn read_at(&mut self, offset: u64, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read(&mut self, path: &str) -> Result<Vec<u8>> {
        let f = self.find(path).ok_or_else(|| Error::NotFound(path.to_string()))?.clone();
        self.read_at(f.offset, f.size as usize)
    }

    /// The main executable (DOL), sized from its 18 section headers.
    pub fn read_dol(&mut self) -> Result<Vec<u8>> {
        let h = self.read_at(self.dol_offset, 0x100)?;
        let mut end = 0u64;
        for i in 0..18 {
            let off = u32_at(&h, 4 * i, "DOL header")? as u64;
            let sz = u32_at(&h, 0x90 + 4 * i, "DOL header")? as u64;
            if sz != 0 { end = end.max(off + sz); }
        }
        self.read_at(self.dol_offset, end as usize)
    }
}

fn parse_fst(fst: &[u8]) -> Result<Vec<DiscFile>> {
    let n = u32_at(fst, 8, "FST root")? as usize;
    let str_base = n * 12;
    let mut out = Vec::new();
    // stack of (end_index, dir_path)
    let mut dirs: Vec<(usize, String)> = vec![(n, String::new())];
    for i in 1..n {
        while dirs.len() > 1 && i >= dirs.last().unwrap().0 { dirs.pop(); }
        let w0 = u32_at(fst, i * 12, "FST entry")?;
        let a = u32_at(fst, i * 12 + 4, "FST entry")? as u64;
        let b = u32_at(fst, i * 12 + 8, "FST entry")? as u64;
        let name = cstr_at(fst, str_base + (w0 & 0x00FF_FFFF) as usize, "FST name")?;
        let parent = &dirs.last().unwrap().1;
        let path = if parent.is_empty() { name } else { format!("{parent}/{name}") };
        if w0 >> 24 != 0 {
            dirs.push((b as usize, path));
        } else {
            out.push(DiscFile { path, offset: a, size: b });
        }
    }
    Ok(out)
}
