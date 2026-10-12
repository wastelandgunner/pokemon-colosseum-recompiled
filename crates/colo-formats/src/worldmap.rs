//! `world_map.fsys/worldmap_menu_rel`: the world map's tables (room 0x38E, menu 0xBD).
//!
//! A data-only REL (id 3) whose prolog registers four arrays and their counts in main.dol `.sbss`; the
//! world map code (`wmapMain` 0x80025730 and the menu 0xBD callbacks) reads them from there:
//! - 0x80478DDC / 0x80478DD8 locations, 0x10 bytes each
//! - 0x80478DE4 / 0x80478DE0 description blocks, 0x28
//! - 0x80478DEC / 0x80478DE8 ride cameras, 0x10
//! - 0x80478DF4 / 0x80478DF0 cursor links, 0xC

use crate::be::{u32_at, u8_at};
use crate::maprel::registrations;
use crate::rel::Rel;
use crate::{Error, Result};

pub const G_LOCATIONS: u32 = 0x8047_8DDC;
pub const G_LOCATION_COUNT: u32 = 0x8047_8DD8;
pub const G_DESCRIPTIONS: u32 = 0x8047_8DE4;
pub const G_DESCRIPTION_COUNT: u32 = 0x8047_8DE0;
pub const G_RIDE_CAMS: u32 = 0x8047_8DEC;
pub const G_RIDE_CAM_COUNT: u32 = 0x8047_8DE8;
pub const G_LINKS: u32 = 0x8047_8DF4;
pub const G_LINK_COUNT: u32 = 0x8047_8DF0;

/// One destination (menu item) on the map.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    /// +0 story flag that unlocks it (0 = always available).
    pub flag: u32,
    /// +4 floor id `floorLink` goes to (also matched against the previous floor for the start cursor).
    pub floor: u32,
    /// +8 menu item (its x/y place the icon).
    pub item: u32,
    /// +0xC snapshot texture resource (`.gtx` FSYS identifier) shown in the preview box.
    pub snapshot: u32,
}

/// One set of location descriptions; the first block whose flag is 0 or set is used (`wmapDrawInfo`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Description {
    pub flag: u32,
    /// Message ids by column (`DESCRIPTION_COLUMN[location]`); 1 = none.
    pub messages: [u32; 9],
}

/// Location index -> description column (main.dol 0x802E4F58).
pub const DESCRIPTION_COLUMN: [usize; 9] = [0, 8, 6, 5, 2, 1, 3, 4, 7];

/// One bike-ride camera: the path animation for `move_demo` set 0 and the `.cam` that films it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RideCam {
    pub flag: u8,
    /// +4 animation index played on map object 0xC6A1000 (`move_demo` model set 0).
    pub anim: u32,
    /// +8 camera resource (`cam_move_demo_NN` FSYS identifier).
    pub camera: u32,
    /// +0xC (not read by `wmapRide`).
    pub unk: f32,
}

/// A D-pad link between two location items (`wmapCursorMove`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Link {
    /// Directions this link answers to: 1 up, 2 right, 4 down, 8 left (diagonals are OR'd).
    pub dir: u8,
    pub from: u32,
    pub to: u32,
}

#[derive(Debug, Clone, Default)]
pub struct WorldMap {
    pub locations: Vec<Location>,
    pub descriptions: Vec<Description>,
    pub ride_cams: Vec<RideCam>,
    pub links: Vec<Link>,
}

impl WorldMap {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let rel = Rel::parse(bytes)?;
        let regs = registrations(&rel);
        let at = |g: u32| -> Result<usize> {
            let &(_, s, o) = regs.iter().find(|r| r.0 == g).ok_or_else(|| Error::NotFound(format!("worldmap_menu_rel: global {g:#x}")))?;
            rel.file_offset(s, o).ok_or_else(|| Error::Invalid("worldmap_menu_rel: empty section".into()))
        };
        let count = |g: u32| -> Result<usize> { Ok(u32_at(rel.data, at(g)?, "worldmap count")? as usize) };
        let d = rel.data;
        let w = |o: usize| u32_at(d, o, "worldmap table");
        let (b, n) = (at(G_LOCATIONS)?, count(G_LOCATION_COUNT)?);
        let locations = (0..n).map(|i| {
            let o = b + i * 0x10;
            Ok(Location { flag: w(o)?, floor: w(o + 4)?, item: w(o + 8)?, snapshot: w(o + 0xC)? })
        }).collect::<Result<_>>()?;
        let (b, n) = (at(G_DESCRIPTIONS)?, count(G_DESCRIPTION_COUNT)?);
        let descriptions = (0..n).map(|i| {
            let o = b + i * 0x28;
            let mut messages = [0; 9];
            for (k, m) in messages.iter_mut().enumerate() { *m = w(o + 4 + 4 * k)?; }
            Ok(Description { flag: w(o)?, messages })
        }).collect::<Result<_>>()?;
        let (b, n) = (at(G_RIDE_CAMS)?, count(G_RIDE_CAM_COUNT)?);
        let ride_cams = (0..n).map(|i| {
            let o = b + i * 0x10;
            Ok(RideCam { flag: u8_at(d, o, "worldmap ride cam")?, anim: w(o + 4)?, camera: w(o + 8)?, unk: f32::from_bits(w(o + 0xC)?) })
        }).collect::<Result<_>>()?;
        let (b, n) = (at(G_LINKS)?, count(G_LINK_COUNT)?);
        let links = (0..n).map(|i| {
            let o = b + i * 0xC;
            Ok(Link { dir: u8_at(d, o, "worldmap link")?, from: w(o + 4)?, to: w(o + 8)? })
        }).collect::<Result<_>>()?;
        Ok(WorldMap { locations, descriptions, ride_cams, links })
    }
}
