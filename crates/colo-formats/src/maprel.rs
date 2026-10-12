//! Per-room `.rel` (e.g. `M2_out.rel`): the room's data lists.
//!
//! Each map REL has a code section whose prolog stores pointers to its data section into main.dol
//! globals that are specific to that room (and an epilog that zeroes them). The room's record in
//! `common_rel` ([`crate::common::Room`]) names those globals: +0x14 characters, +0x18 entry points,
//! +0x1C field cameras, +0x10 light/shadow block. A list global holds a header `{&count, &array}`.
//! We recover `global -> data offset` from the prolog's relocations: each `lis/addi/stw` triple
//! relocates against (main.dol global G, own data offset X) and means `*G = &data[X]`.

use crate::be::{u16_at, u32_at};
use crate::common::Room;
use crate::rel::Rel;
use crate::{Error, Result};

/// One NPC placement (`floorCharacterBios*` accessors in main.dol).
#[derive(Debug, Clone, PartialEq)]
pub struct MapCharacter {
    pub index: usize,
    /// +0 bit 7
    pub visible: bool,
    /// +0 bit 6 ("LoadInit": initial facing applied on load)
    pub load_init: bool,
    /// +0 bit 4
    pub talk_through_walls: bool,
    /// +0 bits 0..2 (2 and 3 select wander behaviours)
    pub move_type: u8,
    /// +1 bits 6..7 / 4..5
    pub talk_start_type: u8,
    pub talk_end_type: u8,
    /// +4 s16, degrees around Y
    pub rotation_y: i16,
    /// +6 u16 index into the people-info (character model) table in common_rel
    pub people_index: u16,
    /// +8 u16 name id
    pub name_id: u16,
    /// +0xC (always 1 in the files checked)
    pub unk_0c: u32,
    /// +0x10 thread id of the AI/move script, spawned async at load with (group, index, model, 0)
    pub move_script: u32,
    /// +0x14 thread id of the talk script, called sync with (group, index, 0, 0)
    pub talk_script: u32,
    /// +0x18 position
    pub position: [f32; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntryPoint {
    pub index: usize,
    /// degrees
    pub angle: i16,
    pub unk_02: i16,
    pub position: [f32; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct CameraPoint {
    pub index: usize,
    /// Anchor position. The field camera blends every point's settings by inverse squared distance from
    /// the camera target (`floorUpdateFieldCamera`, 0x80117514); one point = fixed settings.
    pub position: [f32; 3],
    /// Horizontal eye distance from the target.
    pub distance: f32,
    /// Eye height above the target.
    pub height: f32,
    /// Yaw of the eye offset around the target (radians).
    pub rot_y: f32,
}

/// A TV channel (room record +0x48 list, 0x18 bytes; read by `FUN_80117e58` at floor setup and
/// `mapSetTextureChange` (std 171)): the news model is rendered with the camera into a `width`x`height`
/// texture that replaces the image of every material of the screen's map object.
#[derive(Debug, Clone, PartialEq)]
pub struct TvChannel {
    pub width: u16,
    pub height: u16,
    /// Channel id (`mapSetTextureChange(id)`); id 0 is selected when the room is set up.
    pub id: u32,
    /// Map object (model set) resource id of the screen, e.g. 0x00AB1002 = set 2.
    pub screen: u32,
    /// Model resource id of the programme (FSYS file id << 16 | 0x1000 | model set).
    pub model: u32,
    /// Animation index played (looping, texture animation linked).
    pub anim: u32,
    /// Camera resource (`.cam` FSYS identifier).
    pub camera: u32,
}

#[derive(Debug, Clone, Default)]
pub struct MapRel {
    pub characters: Vec<MapCharacter>,
    pub entry_points: Vec<EntryPoint>,
    pub cameras: Vec<CameraPoint>,
    pub tv: Vec<TvChannel>,
    /// (section, offset) of the 0x28-byte light/shadow block, if registered.
    pub shadow_block: Option<(u8, usize)>,
    /// Light/shadow block +0x00: the sun model's resource id (`floorDataBiosGetSunResID` 0x801154B4); 0 = none.
    /// The lens flare is enabled only when it is set (floor.c ~490).
    pub sun_res_id: u32,
    /// main.dol global -> (section, offset) as registered by the prolog.
    pub registrations: Vec<(u32, u8, usize)>,
}

fn f32_at(d: &[u8], o: usize) -> Result<f32> { Ok(f32::from_bits(u32_at(d, o, "map rel")?)) }

impl MapRel {
    /// Parse a room REL using the globals named by its `common_rel` room record.
    pub fn parse(bytes: &[u8], room: &Room) -> Result<Self> {
        let rel = Rel::parse(bytes)?;
        let registrations = registrations(&rel);
        let find = |g: Option<u32>| g.and_then(|g| registrations.iter().find(|r| r.0 == g).map(|r| (r.1, r.2)));
        // header {&count, &array} -> (array file offset, count)
        let list = |g: Option<u32>| -> Result<Option<(usize, usize)>> {
            let Some((hs, ho)) = find(g) else { return Ok(None) };
            let (Some((cs, co)), Some((as_, ao))) = (rel.pointer_at(hs, ho), rel.pointer_at(hs, ho + 4)) else {
                return Err(Error::Invalid(format!("map rel: list header at s{hs}+{ho:#x} has no relocations")));
            };
            let n = rel.u32_in(cs, co)? as usize;
            let base = rel.file_offset(as_, ao).ok_or_else(|| Error::Invalid("map rel: empty section".into()))?;
            Ok(Some((base, n)))
        };
        let mut out = MapRel { shadow_block: find(room.g_shadow), ..Default::default() };
        let d = rel.data;
        if let Some((s, o)) = out.shadow_block { out.sun_res_id = rel.u32_in(s, o)?; }
        if let Some((base, n)) = list(room.g_characters)? {
            for i in 0..n {
                let b = base + i * 0x24;
                let f0 = d.get(b).copied().ok_or(Error::Truncated { what: "map character", offset: b, need: 0x24, have: 0 })?;
                let f1 = d[b + 1];
                out.characters.push(MapCharacter {
                    index: i,
                    visible: f0 & 0x80 != 0,
                    load_init: f0 & 0x40 != 0,
                    talk_through_walls: f0 & 0x10 != 0,
                    move_type: f0 & 7,
                    talk_start_type: f1 >> 6,
                    talk_end_type: (f1 >> 4) & 3,
                    rotation_y: u16_at(d, b + 4, "map character")? as i16,
                    people_index: u16_at(d, b + 6, "map character")?,
                    name_id: u16_at(d, b + 8, "map character")?,
                    unk_0c: u32_at(d, b + 0xC, "map character")?,
                    move_script: u32_at(d, b + 0x10, "map character")?,
                    talk_script: u32_at(d, b + 0x14, "map character")?,
                    position: [f32_at(d, b + 0x18)?, f32_at(d, b + 0x1C)?, f32_at(d, b + 0x20)?],
                });
            }
        }
        if let Some((base, n)) = list(room.g_entry_points)? {
            for i in 0..n {
                let b = base + i * 0x10;
                out.entry_points.push(EntryPoint {
                    index: i,
                    angle: u16_at(d, b, "entry point")? as i16,
                    unk_02: u16_at(d, b + 2, "entry point")? as i16,
                    position: [f32_at(d, b + 4)?, f32_at(d, b + 8)?, f32_at(d, b + 0xC)?],
                });
            }
        }
        if let Some((base, n)) = list(room.g_cameras)? {
            for i in 0..n {
                let b = base + i * 0x18;
                out.cameras.push(CameraPoint {
                    index: i,
                    position: [f32_at(d, b)?, f32_at(d, b + 4)?, f32_at(d, b + 8)?],
                    distance: f32_at(d, b + 0xC)?,
                    height: f32_at(d, b + 0x10)?,
                    rot_y: f32_at(d, b + 0x14)?,
                });
            }
        }
        if let Some((base, n)) = list(room.g_extra)? {
            for i in 0..n.min(64) {
                let b = base + i * 0x18;
                out.tv.push(TvChannel {
                    width: u16_at(d, b, "tv")?, height: u16_at(d, b + 2, "tv")?, id: u32_at(d, b + 4, "tv")?,
                    screen: u32_at(d, b + 8, "tv")?, model: u32_at(d, b + 0xC, "tv")?, anim: u32_at(d, b + 0x10, "tv")?,
                    camera: u32_at(d, b + 0x14, "tv")?,
                });
            }
        }
        out.registrations = registrations;
        Ok(out)
    }
}

/// Pair up the prolog's ADDR16_LO relocations: an own-section `addi` followed by a main.dol `stw`.
pub fn registrations(rel: &Rel) -> Vec<(u32, u8, usize)> {
    let mut out = vec![];
    let mut pending: Option<(u8, usize)> = None;
    for ((sec, _off), r) in &rel.relocs {
        if !rel.sections.get(*sec as usize).is_some_and(|s| s.exec) { continue; }
        if r.kind != 4 { continue; } // ADDR16_LO
        if r.module == rel.id {
            pending = Some((r.target_section, r.addend as usize));
        } else if r.module == 0 {
            if let Some((s, o)) = pending.take() {
                if !out.iter().any(|x: &(u32, u8, usize)| x.0 == r.addend) { out.push((r.addend, s, o)); }
            }
        }
    }
    out
}
