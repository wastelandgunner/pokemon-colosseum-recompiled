//! FSYS archives (Genius Sonority). Layout per GoD Tool `XGFsys.swift`, Colosseum variant.
//!
//! Header: `"FSYS"`, u32 group id @0x08, u32 entry count @0x0C, u32 file size @0x20.
//! Per-entry detail pointers are an inline u32 table at 0x60. Each 0x50-byte detail record:
//! +0x00 identifier (byte 2 = file type), +0x04 data offset, +0x08 uncompressed size,
//! +0x14 stored size, +0x1C full-filename pointer (often 0), +0x24 base-filename pointer.

use crate::be::{cstr_at, u32_at};
use crate::{lzss, Error, Result};

pub const MAGIC: &[u8; 4] = b"FSYS";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    None, Rdat, Dat, Ccd, Samp, Msg, Fnt, Scd, Dats, Gtx, Gpt1, Cam, Rel, Pkx, Wzx, Isd, Ish, Thh, Thd, Gsw, Atx, Bin,
    /// 0x22 lens flare (`floorReadGFLPre/Post`; only `T1_ancient_colo`), see [`crate::gfl`]
    Gfl,
    Unknown(u8),
}

impl FileType {
    pub fn from_id(b: u8) -> Self {
        use FileType::*;
        match b {
            0x00 => None, 0x02 => Rdat, 0x04 => Dat, 0x06 => Ccd, 0x08 => Samp, 0x0A => Msg, 0x0C => Fnt,
            0x0E => Scd, 0x10 => Dats, 0x12 => Gtx, 0x14 => Gpt1, 0x18 => Cam, 0x1C => Rel, 0x1E => Pkx,
            0x20 => Wzx, 0x22 => Gfl, 0x28 => Isd, 0x2A => Ish, 0x2C => Thh, 0x2E => Thd, 0x30 => Gsw, 0x32 => Atx,
            0x34 => Bin, other => Unknown(other),
        }
    }
    pub fn extension(self) -> String {
        use FileType::*;
        match self {
            None => "bin".into(), Rdat => "rdat".into(), Dat => "dat".into(), Ccd => "ccd".into(),
            Samp => "samp".into(), Msg => "msg".into(), Fnt => "fnt".into(), Scd => "scd".into(),
            Dats => "dats".into(), Gtx => "gtx".into(), Gpt1 => "gpt1".into(), Cam => "cam".into(),
            Rel => "rel".into(), Pkx => "pkx".into(), Wzx => "wzx".into(), Isd => "isd".into(),
            Ish => "ish".into(), Thh => "thh".into(), Thd => "thd".into(), Gsw => "gsw".into(),
            Atx => "atx".into(), Bin => "bin".into(), Gfl => "gfl".into(), Unknown(b) => format!("t{b:02x}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub index: usize,
    pub identifier: u32,
    pub file_type: FileType,
    /// Base name without extension, e.g. `M2_out`.
    pub name: String,
    /// Full name with extension when the archive stores one (often empty).
    pub full_name: String,
    pub data_offset: usize,
    pub uncompressed_size: usize,
    pub stored_size: usize,
}

impl Entry {
    /// Name to use when writing the file out: full name if present, else `name.ext`.
    pub fn file_name(&self) -> String {
        if !self.full_name.is_empty() {
            self.full_name.clone()
        } else {
            format!("{}.{}", self.name, self.file_type.extension())
        }
    }
}

pub struct Fsys<'a> {
    data: &'a [u8],
    pub group_id: u32,
    pub entries: Vec<Entry>,
}

const DETAIL_TABLE: usize = 0x60;

impl<'a> Fsys<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        if data.get(..4) != Some(&MAGIC[..]) {
            let mut m = [0u8; 4];
            if let Some(s) = data.get(..4) { m.copy_from_slice(s); }
            return Err(Error::BadMagic { what: "FSYS", found: m });
        }
        let group_id = u32_at(data, 0x08, "FSYS header")?;
        let count = u32_at(data, 0x0C, "FSYS header")? as usize;
        let mut entries = Vec::with_capacity(count);
        for i in 0..count {
            let det = u32_at(data, DETAIL_TABLE + 4 * i, "FSYS detail table")? as usize;
            let identifier = u32_at(data, det, "FSYS entry")?;
            let name_ptr = u32_at(data, det + 0x24, "FSYS entry")? as usize;
            let full_ptr = u32_at(data, det + 0x1C, "FSYS entry")? as usize;
            entries.push(Entry {
                index: i,
                identifier,
                file_type: FileType::from_id((identifier >> 8) as u8),
                name: if name_ptr != 0 { cstr_at(data, name_ptr, "FSYS name")? } else { String::new() },
                full_name: if full_ptr != 0 { cstr_at(data, full_ptr, "FSYS name")? } else { String::new() },
                data_offset: u32_at(data, det + 0x04, "FSYS entry")? as usize,
                uncompressed_size: u32_at(data, det + 0x08, "FSYS entry")? as usize,
                stored_size: u32_at(data, det + 0x14, "FSYS entry")? as usize,
            });
        }
        Ok(Fsys { data, group_id, entries })
    }

    /// Raw stored bytes (possibly an LZSS container).
    pub fn raw(&self, e: &Entry) -> Result<&'a [u8]> {
        crate::be::bytes_at(self.data, e.data_offset, e.stored_size, "FSYS entry data")
    }

    pub fn is_compressed(&self, e: &Entry) -> bool {
        self.data.get(e.data_offset..e.data_offset + 4) == Some(&lzss::MAGIC[..])
    }

    /// Entry contents, decompressed if needed.
    pub fn read(&self, e: &Entry) -> Result<Vec<u8>> {
        let raw = self.raw(e)?;
        if lzss::is_lzss(raw) { lzss::decompress_container(raw) } else { Ok(raw.to_vec()) }
    }

    /// First entry with this base name and type.
    pub fn find(&self, name: &str, ty: FileType) -> Option<&Entry> {
        self.entries.iter().find(|e| e.name == name && e.file_type == ty)
    }
    pub fn find_type(&self, ty: FileType) -> impl Iterator<Item = &Entry> {
        self.entries.iter().filter(move |e| e.file_type == ty)
    }
}
