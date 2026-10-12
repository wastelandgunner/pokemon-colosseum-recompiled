//! `common.fsys/common_rel.rel`: global game tables.
//!
//! Tables are addressed by GoD Tool's symbol indices (see [`crate::rel::Rel::god_symbols`]); each table
//! index is followed by an index whose target holds the entry count.

use crate::be::{u16_at, u32_at, u8_at};
use crate::rel::Rel;
use crate::{Error, Result};

pub const SYM_ROOMS: usize = 14;
pub const SYM_CHARACTER_MODELS: usize = 72;
pub const SYM_DOORS: usize = 30;
pub const SYM_INTERACTION_POINTS: usize = 86;
/// The global script (script id 0x596): warps (func 4), doors (func 5), shared event code.
pub const SYM_SCRIPT: usize = 101;

/// One room ("floor") record, 0x4C bytes. Accessors in main.dol: `floorDataBios*`.
#[derive(Debug, Clone)]
pub struct Room {
    pub index: usize,
    /// +0 top 3 bits (`floorDataBiosGetFloorKind`)
    pub kind: u8,
    /// +1 (`floorDataBiosGetArea`)
    pub area: u8,
    /// +4 FSYS group id of the room's archive (`floorDataBiosGetGroupID`)
    pub group_id: u32,
    /// +8 map model resource id (`floorDataBiosGetMapResID`)
    pub map_res_id: u32,
    /// +0xC room id used by warps/scripts (`floorDataBiosGetFloorID`)
    pub floor_id: u32,
    /// +0x10 main.dol global holding the light/shadow block pointer
    pub g_shadow: Option<u32>,
    /// +0x14 main.dol global holding the character list header (`floorDataBiosGetCharInfo`)
    pub g_characters: Option<u32>,
    /// +0x18 global holding the entry point list header (`floorDataBiosGetPosListPtr`)
    pub g_entry_points: Option<u32>,
    /// +0x1C global holding the field camera list header (`floorDataBiosGetFieldCameraListPtr`)
    pub g_cameras: Option<u32>,
    pub unk_20: u32,
    pub unk_24: u32,
    /// +0x28 sound script (spawned async, priority 15, on load), +0x2C "modify" script (called sync),
    /// +0x30/+0x34/+0x38 further script thread ids (roles not confirmed).
    pub scripts: [u32; 5],
    /// +0x48 extra global (rare)
    pub g_extra: Option<u32>,
}

/// One entry of the game's sound table (main.dol global 0x80478FAC, 12 bytes, indexed by sound id).
#[derive(Debug, Clone, Copy, Default)]
pub struct SoundEntry {
    /// +0 bit 7: a MusyX sound effect (else a song).
    pub flags: u8,
    /// +1 MusyX group id (songs: passed to sndSeqPlayEx).
    pub group: u8,
    /// +2 ReverbHI preset index (songs; 0 keeps the current one).
    pub reverb: u8,
    /// +3 song file index (songs).
    pub song_file: u8,
    /// +4 MusyX SongID (songs) or FX id (effects).
    pub musyx_id: u16,
}

impl SoundEntry {
    pub fn is_fx(&self) -> bool { self.flags & 0x80 != 0 }
}

/// Wave table entry (global 0x80478FB4, 0x18 bytes): a MusyX group and its FSYS files.
#[derive(Debug, Clone)]
pub struct WaveGroup {
    pub byte0: u8,
    pub musyx_group: u16,
    pub fsys_group: u32,
    pub pool: u32,
    pub proj: u32,
    pub sdir: u32,
    /// DVD path of the `.samp` sample bank.
    pub samp_path: String,
}

/// ReverbHI preset (global 0x80478FA4, 0x18 bytes): coloration, mix, time, damping, preDelay, crosstalk.
pub type ReverbPreset = [f32; 6];

/// All sound tables, from common_rel.
#[derive(Debug, Clone, Default)]
pub struct SoundTables {
    pub sounds: Vec<SoundEntry>,
    /// Song file table (global 0x80478E34): FSYS identifier in bgm_archive (0 = none) and DVD path.
    pub song_files: Vec<(u32, String)>,
    pub waves: Vec<WaveGroup>,
    pub reverbs: Vec<ReverbPreset>,
}

#[derive(Debug, Clone)]
pub struct CharacterModel {
    pub index: usize,
    /// +0xC FSYS entry identifier of the model `.dat`
    pub fsys_identifier: u32,
    /// +0x10 distance covered by one walk-animation cycle; walk speed per frame = this / walk anim frames
    /// (people entry +0x34, set in `peopleInit`, used by the speed setter 0x80188214)
    pub walk_stride: f32,
    /// +0x14 distance covered by one run-animation cycle; run speed per frame = this / run anim frames
    pub run_stride: f32,
    /// +0x18
    pub collision_radius: f32,
    /// +1..+8 animation slots (0xFF = none): [0] idle, [1] walk, [2] run, then model-specific extras.
    /// Most humans use 1/2/3; some models start at 0 (e.g. rider_m_b2: 0/1/2).
    pub anim_slots: [u8; 8],
    pub raw: Vec<u8>,
}

/// 0x1C bytes. Dispatcher at 0x80116D30 matches (method, room, collision id) and calls `script` sync
/// with the four parameters.
#[derive(Debug, Clone)]
pub struct InteractionPoint {
    pub index: usize,
    /// 1 pass, 2 touch, 3 check (A button), 4 menu
    pub method: u8,
    pub room_id: u16,
    /// collision region id (+4)
    pub region: u32,
    /// thread id (+8)
    pub script: u32,
    pub params: [u32; 4],
}

/// 0x18 bytes (`Doors`, GoD symbol 30). Driven by `floorEventCtrlDoor(floor, index, mode)` (0x80116958):
/// mode 0 open (animated, blocks), 1 opened (jump to end), 2 close, 3 closed. Animation indices refer to the
/// map model's animations.
#[derive(Debug, Clone)]
pub struct Door {
    pub index: usize,
    pub open_anim: i8,
    pub close_anim: i8,
    pub open_end_anim: i8,
    pub close_end_anim: i8,
    /// +7: 1/3 = flag-controlled (open if flag set on room entry), 2 = always closed on entry
    pub door_type: u8,
    /// +8
    pub part_index: u8,
    /// +0xA collision id toggled with `scriptSetCol`
    pub collision_id: i16,
    /// +0xC floor id of the room the door is in
    pub floor_id: u16,
    /// +0xE flag that records the door as open
    pub open_flag: u16,
    /// +0x10
    pub close_flag: u16,
    /// +0x14 resource (map object) id passed to `floorGetResource`
    pub resource_id: u32,
}

pub struct Common<'a> {
    pub rel: Rel<'a>,
    symbols: Vec<usize>,
}

impl<'a> Common<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        let rel = Rel::parse(bytes)?;
        let symbols = rel.god_symbols();
        Ok(Common { rel, symbols })
    }

    /// (file offset of table, entry count) for a table symbol index.
    pub fn table(&self, sym: usize) -> Result<(usize, usize)> {
        let base = *self.symbols.get(sym).ok_or_else(|| Error::Invalid(format!("common_rel: no symbol {sym}")))?;
        let cnt = *self.symbols.get(sym + 1).ok_or_else(|| Error::Invalid(format!("common_rel: no symbol {}", sym + 1)))?;
        Ok((base, u32_at(self.rel.data, cnt, "common_rel count")? as usize))
    }

    fn main_ptr(&self, file_offset: usize) -> Option<u32> {
        let (s, o) = self.rel.locate(file_offset)?;
        self.rel.main_pointer_at(s, o)
    }

    /// The global string tables embedded in common_rel (menu labels, items, memory card messages...): every
    /// `.msg`-layout block (count @4, "US" region tag @6, zero padding, string offsets past the id table).
    pub fn messages(&self) -> Vec<crate::msg::Message> {
        let d = self.rel.data;
        let mut out = vec![];
        let mut p = 0;
        while p + 0x18 <= d.len() {
            let count = u16::from_be_bytes([d[p + 4], d[p + 5]]) as usize;
            let table_end = 0x10 + 8 * count;
            let plausible = count > 0 && &d[p + 6..p + 8] == b"US" && d[p + 8..p + 0x10].iter().all(|&b| b == 0)
                && p + table_end <= d.len()
                && u32_at(d, p + 0x14, "msg").is_ok_and(|o| (o as usize) >= table_end && p + (o as usize) < d.len());
            if plausible {
                if let Ok(m) = crate::msg::parse_table(&d[p..]) { out.extend(m); p += table_end; continue; }
            }
            p += 4;
        }
        out
    }

    /// The global CMS script embedded in common_rel (from its symbol to the next symbol or section end).
    pub fn script(&self) -> Result<crate::scd::Script> {
        let start = *self.symbols.get(SYM_SCRIPT).ok_or_else(|| Error::Invalid("common_rel: no script symbol".into()))?;
        let (sec, _) = self.rel.locate(start).ok_or_else(|| Error::Invalid("common_rel: script outside sections".into()))?;
        let s = self.rel.sections[sec as usize];
        let sec_end = s.offset + s.size;
        let end = self.symbols.iter().copied().filter(|&o| o > start && o <= sec_end).min().unwrap_or(sec_end);
        crate::scd::Script::parse(self.rel.data[start..end].to_vec())
    }

    pub fn rooms(&self) -> Result<Vec<Room>> {
        let (base, n) = self.table(SYM_ROOMS)?;
        let d = self.rel.data;
        (0..n).map(|i| {
            let b = base + i * 0x4C;
            let w = |o: usize| u32_at(d, b + o, "room");
            let b0 = u8_at(d, b, "room")?;
            Ok(Room {
                index: i,
                kind: b0 >> 5,
                area: u8_at(d, b + 1, "room")?,
                group_id: w(4)?,
                map_res_id: w(8)?,
                floor_id: w(0xC)?,
                g_shadow: self.main_ptr(b + 0x10),
                g_characters: self.main_ptr(b + 0x14),
                g_entry_points: self.main_ptr(b + 0x18),
                g_cameras: self.main_ptr(b + 0x1C),
                unk_20: w(0x20)?,
                unk_24: w(0x24)?,
                scripts: [w(0x28)?, w(0x2C)?, w(0x30)?, w(0x34)?, w(0x38)?],
                g_extra: self.main_ptr(b + 0x48),
            })
        }).collect()
    }

    pub fn character_models(&self) -> Result<Vec<CharacterModel>> {
        let (base, n) = self.table(SYM_CHARACTER_MODELS)?;
        let d = self.rel.data;
        (0..n).map(|i| {
            let b = base + i * 0x2C;
            Ok(CharacterModel {
                index: i,
                fsys_identifier: u32_at(d, b + 0xC, "character model")?,
                walk_stride: f32::from_bits(u32_at(d, b + 0x10, "character model")?),
                run_stride: f32::from_bits(u32_at(d, b + 0x14, "character model")?),
                collision_radius: f32::from_bits(u32_at(d, b + 0x18, "character model")?),
                anim_slots: {
                    let mut a = [0xFFu8; 8];
                    for (k, v) in a.iter_mut().enumerate() { *v = u8_at(d, b + 1 + k, "character model")?; }
                    a
                },
                raw: crate::be::bytes_at(d, b, 0x2C, "character model")?.to_vec(),
            })
        }).collect()
    }

    /// The Shadow Pokémon table (0x38-byte entries; main.dol global 0x80478F6C, registered by common_rel's
    /// prolog): per entry its "registered" flag (+0xC, set to 1) and state flag (+0x1C), which
    /// `shadowPokemonSetState(index, state)` (std 243, 0x801EE67C) writes. Index 0 is a blank entry; valid
    /// indices are 1..=0x60. The third value is the +0x12 flag (`FUN_801EEAD0` checks it, e.g. in the TV event).
    pub fn shadow_flags(&self) -> Result<Vec<(u16, u16, u16)>> {
        let regs = crate::maprel::registrations(&self.rel);
        let (_, s, o) = regs.iter().find(|r| r.0 == 0x8047_8F6C).copied().ok_or_else(|| Error::NotFound("shadow table".into()))?;
        let base = self.rel.file_offset(s, o).ok_or_else(|| Error::Invalid("shadow table section".into()))?;
        let d = self.rel.data;
        (0..=0x60).map(|i| { let b = base + i * 0x38; Ok((u16_at(d, b + 0xC, "shadow")?, u16_at(d, b + 0x1C, "shadow")?, u16_at(d, b + 0x12, "shadow")?)) }).collect()
    }

    /// The sound tables (see [`SoundTables`]); each is registered by common_rel's prolog as a main.dol
    /// (data, count) global pair.
    pub fn sound_tables(&self) -> Result<SoundTables> {
        let regs = crate::maprel::registrations(&self.rel);
        let d = self.rel.data;
        let off = |g: u32| -> Result<usize> {
            let (_, s, o) = regs.iter().find(|r| r.0 == g).copied().ok_or_else(|| Error::NotFound(format!("global {g:#x}")))?;
            self.rel.file_offset(s, o).ok_or_else(|| Error::Invalid("sound table section".into()))
        };
        let count = |g: u32| -> Result<usize> { Ok(u32_at(d, off(g)?, "count")? as usize) };
        let cstr = |p: usize| -> String {
            let e = d[p..].iter().position(|&b| b == 0).unwrap_or(0);
            String::from_utf8_lossy(&d[p..p + e]).into_owned()
        };
        // a pointer field inside the table: its relocation target
        let ptr_str = |sec_off: usize| -> String {
            self.rel.relocs.iter().find(|((s, o), _)| self.rel.file_offset(*s, *o as usize) == Some(sec_off))
                .and_then(|(_, r)| self.rel.file_offset(r.target_section, r.addend as usize)).map(cstr).unwrap_or_default()
        };
        let mut t = SoundTables::default();
        let (b, n) = (off(0x8047_8FAC)?, count(0x8047_8FA8)?);
        for i in 0..n {
            let e = b + i * 0xC;
            t.sounds.push(SoundEntry { flags: u8_at(d, e, "snd")?, group: u8_at(d, e + 1, "snd")?, reverb: u8_at(d, e + 2, "snd")?,
                song_file: u8_at(d, e + 3, "snd")?, musyx_id: u16_at(d, e + 4, "snd")? });
        }
        let (b, n) = (off(0x8047_8E34)?, count(0x8047_8E30)?);
        for i in 0..n { let e = b + i * 8; t.song_files.push((u32_at(d, e, "song file")?, ptr_str(e + 4))); }
        let (b, n) = (off(0x8047_8FB4)?, count(0x8047_8FB0)?);
        for i in 0..n {
            let e = b + i * 0x18;
            t.waves.push(WaveGroup { byte0: u8_at(d, e, "wave")?, musyx_group: u16_at(d, e + 2, "wave")?, fsys_group: u32_at(d, e + 4, "wave")?,
                pool: u32_at(d, e + 8, "wave")?, proj: u32_at(d, e + 0xC, "wave")?, sdir: u32_at(d, e + 0x10, "wave")?, samp_path: ptr_str(e + 0x14) });
        }
        let (b, n) = (off(0x8047_8FA4)?, count(0x8047_8FA0)?);
        for i in 0..n {
            let e = b + i * 0x18;
            let mut p = [0f32; 6];
            for (k, v) in p.iter_mut().enumerate() { *v = f32::from_bits(u32_at(d, e + 4 * k, "reverb")?); }
            t.reverbs.push(p);
        }
        Ok(t)
    }

    /// Species table (main.dol global 0x80478F94 / count 0x80478F90, 0x11C bytes each) +0xE: the cry's sound
    /// id per species index (`pokemonDataBiosGetVoice`).
    pub fn species_voices(&self) -> Result<Vec<u16>> {
        let regs = crate::maprel::registrations(&self.rel);
        let d = self.rel.data;
        let off = |g: u32| -> Result<usize> {
            let (_, s, o) = regs.iter().find(|r| r.0 == g).copied().ok_or_else(|| Error::NotFound(format!("global {g:#x}")))?;
            self.rel.file_offset(s, o).ok_or_else(|| Error::Invalid("species table section".into()))
        };
        let (b, n) = (off(0x8047_8F94)?, u32_at(d, off(0x8047_8F90)?, "species count")? as usize);
        (0..n).map(|i| u16_at(d, b + i * 0x11C + 0xE, "species")).collect()
    }

    pub fn doors(&self) -> Result<Vec<Door>> {
        let (base, n) = self.table(SYM_DOORS)?;
        let d = self.rel.data;
        (0..n).map(|i| {
            let b = base + i * 0x18;
            let s8 = |o: usize| -> Result<i8> { Ok(u8_at(d, b + o, "door")? as i8) };
            Ok(Door {
                index: i,
                open_anim: s8(0)?, close_anim: s8(1)?, open_end_anim: s8(2)?, close_end_anim: s8(3)?,
                door_type: u8_at(d, b + 7, "door")?,
                part_index: u8_at(d, b + 8, "door")?,
                collision_id: u16_at(d, b + 0xA, "door")? as i16,
                floor_id: u16_at(d, b + 0xC, "door")?,
                open_flag: u16_at(d, b + 0xE, "door")?,
                close_flag: u16_at(d, b + 0x10, "door")?,
                resource_id: u32_at(d, b + 0x14, "door")?,
            })
        }).collect()
    }

    pub fn interaction_points(&self) -> Result<Vec<InteractionPoint>> {
        let (base, n) = self.table(SYM_INTERACTION_POINTS)?;
        let d = self.rel.data;
        (0..n).map(|i| {
            let b = base + i * 0x1C;
            Ok(InteractionPoint {
                index: i,
                method: u8_at(d, b, "interaction point")?,
                room_id: u16_at(d, b + 2, "interaction point")?,
                region: u32_at(d, b + 4, "interaction point")?,
                script: u32_at(d, b + 8, "interaction point")?,
                params: [u32_at(d, b + 0xC, "ip")?, u32_at(d, b + 0x10, "ip")?, u32_at(d, b + 0x14, "ip")?, u32_at(d, b + 0x18, "ip")?],
            })
        }).collect()
    }
}
