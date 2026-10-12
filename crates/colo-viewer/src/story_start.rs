//! Story flags for a New Game started part-way through the story.
//!
//! The real New Game (`std021`'s title flow, then the opening movie callback `FUN_80035E04`) sets flag
//! 0x8D0 (2256) and links to floor 1 (S1_out, Outskirt Stand) entry 0 with global script func_8 as the fade
//! script. Pyrite becomes reachable on the world map once flag 1016 is set (worldmap_menu_rel destination
//! table: Phenac/floor 3 needs 991, the D4_out construction site/floor 0x66 needs 2649, Pyrite/floor 0xF
//! needs 1016, set by D4_out's entry script).
//!
//! `PYRITE_ARRIVAL` is the state on first reaching Pyrite, derived with `colo-tool story-explore` by playing
//! Outskirt Stand, Phenac and D4_out with the real scripts and keeping only the steps needed for flag 1016:
//! - Outskirt Stand: arrival (986, 2808), the shop's TV news (987, 2565, 2766), talk to WILLIE (988),
//!   leave on the bike (989, 990, 991, 2650).
//! - Phenac: arrival (992-995), RUI joins (996, 997, 2454, 2767), the MAYOR's house (2468), the stadium
//!   desk (998, 2835), the battle at the fountain (999, 2832), the shop clerk (2462), back to Outskirt
//!   Stand for the shop owner (1000, 1161), MAYOR's house again with FOLLY and TRUDLY (1001, 1002,
//!   2481-2484), the CIPHER agent at the gate (1004, 2514, 2649) with its random Pyrite trainer picks
//!   (2588, 2724).
//! - D4_out: 1016, 2648.
//! Choices where the game branches are fixed: the first of the three gate agents (2514), random numbers 0
//! (2588/2724), and the Shadow MAKUHITA snagged in the MAYOR's house (Shadow table entry 1: registered
//! flag 1166, snagged flag 2126). Optional events (the Phenac gym, trainers) are left out.
pub const PYRITE_ARRIVAL: &[u32] = &[
    986, 987, 988, 989, 990, 991, 992, 993, 994, 995, 996, 997, 998, 999, 1000, 1001, 1002, 1004, 1016, 1161, 1166, 2126,
    2256, 2454, 2462, 2468, 2481, 2482, 2483, 2484, 2514, 2565, 2588, 2648, 2649, 2650, 2724, 2766, 2767, 2808, 2832, 2835,
];

/// The New Game flag (set by the opening movie's end callback).
pub const NEW_GAME: &[u32] = &[2256];

/// Flags for a New Game started in `room` (`COLO_STORY=start|pyrite|none` overrides the choice).
pub fn for_room(room: &str) -> &'static [u32] {
    match std::env::var("COLO_STORY").as_deref() {
        Ok("none") => &[],
        Ok("start") => NEW_GAME,
        Ok("pyrite") => PYRITE_ARRIVAL,
        _ if room.starts_with("M2_") => PYRITE_ARRIVAL,
        _ if room.starts_with("S1_") => NEW_GAME,
        _ => &[],
    }
}
