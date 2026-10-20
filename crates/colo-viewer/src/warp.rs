//! Warp to a room from power-on (`colo-viewer <gcm> <room>[:entry]`, recomp.rs): the pad presses Start/A through
//! the title (`buttons`) until the title menu (`titleMenuMain` 0x800210F0) starts, which is replaced once by its
//! Continue + YES path with the room in place of the save's return map: `heroMoveSyncWithHero`, then
//! `floorLink(floor, entry)` instead of `fn_80113FE8` (the save's floor), `fn_800056E4(1)`, the menu result 1, and
//! the function's epilogue (`fn_801CB9D8(lbl_8047A35C)`). So the room runs with the card's story save (party, bag,
//! flags) on the game's own code, menus included. Without a story save on the card the title menu runs as usual.
//! Standalone (no crate deps but colo_recomp) so examples can `#[path]` it.

use colo_recomp::run::Machine;
use std::sync::Mutex;

/// `titleMenuMain` (the title's Story/Battle/Options menu loop) and what its Continue path calls.
const TITLE_MENU_MAIN: u32 = 0x8002_10F0;
const GAMEDATA_SAVE_GET_STATUS: u32 = 0x8013_5168;
const HERO_MOVE_SYNC_WITH_HERO: u32 = 0x8013_0054;
const FLOOR_LINK: u32 = 0x8011_3828;
const FN_800056E4: u32 = 0x8000_56E4;
const FN_801CB9D8: u32 = 0x801C_B9D8;
/// (floor id, entry) still to warp to; taken by the title menu hook.
static WARP: Mutex<Option<(u32, u32)>> = Mutex::new(None);

fn call(c: &mut colo_recomp::ppc::Cpu, a: u32, args: &[u32]) -> u32 {
    for (i, &v) in args.iter().enumerate() { c.r[3 + i] = v; }
    let lr = c.lr;
    colo_recomp::lifted::dispatch(c, a);
    c.lr = lr;
    c.r[3]
}

/// `titleMenuMain` for a warp: its case 3 (Continue, "Is it okay to continue from here?" YES) and case 0x64
/// without the menus, linking `WARP`'s floor instead of the save's (`fn_80113FE8`).
fn title_menu_warp(c: &mut colo_recomp::ppc::Cpu) {
    let w = WARP.lock().unwrap().take();
    let Some((floor, entry)) = w else { colo_recomp::run::call_original(c, TITLE_MENU_MAIN); return };
    // gamedatasaveGetStatus(0, 4): is there a story save (case 2)
    if call(c, GAMEDATA_SAVE_GET_STATUS, &[0, 4]) == 0 {
        eprintln!("warp: no story save on the memory card; the title menu runs as usual");
        colo_recomp::run::call_original(c, TITLE_MENU_MAIN);
        return;
    }
    let (sel, res) = (0x8047_A360, 0x8047_A358);
    c.m.set_u32(sel + 4, 1);
    call(c, HERO_MOVE_SYNC_WITH_HERO, &[]);
    call(c, FLOOR_LINK, &[floor, entry]);
    call(c, FN_800056E4, &[1]);
    c.m.set_u32(sel, 1);
    c.m.set_u32(res, 1);
    let ctx = c.m.u32(0x8047_A35C);
    call(c, FN_801CB9D8, &[ctx]);
    c.r[3] = 1;
}


/// Set the warp and hook the title menu.
pub fn install(m: &mut Machine, w: (u32, u32)) {
    *WARP.lock().unwrap() = Some(w);
    m.c.hook_fn.insert(TITLE_MENU_MAIN, title_menu_warp);
}

/// Whether the title menu hasn't taken the warp yet (the pad should keep pressing `buttons`).
pub fn pending() -> bool { WARP.lock().unwrap().is_some() }

/// Pad buttons for game frame `n` while `pending`: Start (0x1000), then A (0x100), 3 frames in 40 each.
pub fn buttons(n: u32) -> u16 { match n % 40 { 10..13 => 0x1000, 30..33 => 0x100, _ => 0 } }
