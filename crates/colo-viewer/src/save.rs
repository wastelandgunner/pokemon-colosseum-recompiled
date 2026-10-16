//! Viewer save file, written only from the PC's Save option (like the game: `fn_80038A0C`).
//!
//! The game stores the PC's return map (`mapSetReturnMap`: floor, previous floor, entry index) as the save's
//! location (`gamedatasaveBiosSetFloorid` / `...Prevfloorid` / `...Floorposindex`), so a continued game
//! starts at that room's entry point in front of the PC, not at an exact position.
//!
//! Plain text, one item per line: `room <name>`, `entry <index>`, `prev <floor>`, `partner <0|1>`, `flag <id>` (older saves
//! may have `pos <x> <y> <z> <yaw>` instead of `entry`). Location: `$COLO_SAVE`, else
//! `$XDG_DATA_HOME/colo-viewer/save.txt`, else `~/.local/share/colo-viewer/save.txt`.
//! (The game's own memory card format isn't decoded yet; this only stands in for its flag block.)

use bevy::prelude::*;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Save {
    pub room: Option<String>,
    pub entry: Option<usize>,
    pub prev_floor: Option<u32>,
    pub pos: Option<(Vec3, f32)>,
    pub flags: HashSet<u32>,
    /// `partner 0|1`: hero status 0x18 (RUI in the party).
    pub partner: Option<bool>,
}

pub fn path() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("COLO_SAVE") { return Some(p.into()); }
    let base = std::env::var("XDG_DATA_HOME").map(PathBuf::from)
        .or_else(|_| std::env::var("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .ok()?;
    Some(base.join("colo-viewer").join("save.txt"))
}

pub fn load() -> Option<Save> {
    let text = std::fs::read_to_string(path()?).ok()?;
    let mut s = Save::default();
    for line in text.lines() {
        let mut it = line.split_whitespace();
        match it.next() {
            Some("room") => s.room = it.next().map(str::to_string),
            Some("pos") => {
                let v: Vec<f32> = it.filter_map(|x| x.parse().ok()).collect();
                if v.len() == 4 { s.pos = Some((Vec3::new(v[0], v[1], v[2]), v[3])); }
            }
            Some("entry") => s.entry = it.next().and_then(|x| x.parse().ok()),
            Some("prev") => s.prev_floor = it.next().and_then(|x| x.parse().ok()),
            Some("partner") => s.partner = it.next().map(|x| x == "1"),
            Some("flag") => { if let Some(f) = it.next().and_then(|x| x.parse().ok()) { s.flags.insert(f); } }
            _ => {}
        }
    }
    Some(s)
}

/// Whether a save file exists (the PC asks before overwriting it).
pub fn exists() -> bool { path().is_some_and(|p| p.exists()) }

pub fn write(room: &str, entry: usize, prev_floor: u32, partner: bool, flags: &HashSet<u32>) -> Result<(), String> {
    let p = path().ok_or("no save location (set COLO_SAVE or HOME)")?;
    if let Some(dir) = p.parent() { let _ = std::fs::create_dir_all(dir); }
    let mut sorted: Vec<&u32> = flags.iter().collect();
    sorted.sort();
    let mut out = format!("# colo-viewer save\nroom {room}\nentry {entry}\nprev {prev_floor}\npartner {}\n", partner as u8);
    for f in sorted { out.push_str(&format!("flag {f}\n")); }
    std::fs::write(&p, out).map_err(|e| format!("{}: {e}", p.display()))?;
    println!("saved to {}", p.display());
    Ok(())
}

/// `COLO_FLAGS="1144,2475,-12"`: set (or with `-`, clear) story flags, e.g. to start at a chosen story point.
pub fn apply_overrides(flags: &mut HashSet<u32>) {
    let Ok(v) = std::env::var("COLO_FLAGS") else { return };
    for tok in v.split(',').map(str::trim).filter(|t| !t.is_empty()) {
        if let Some(n) = tok.strip_prefix('-').and_then(|x| x.parse::<u32>().ok()) { flags.remove(&n); }
        else if let Ok(n) = tok.parse::<u32>() { flags.insert(n); }
    }
}
