//! Pokémon Colosseum battle engine, ported function by function from GC6E01 main.dol.
//!
//! Headless: no rendering, no I/O. Every module names the original functions it ports.

pub mod bios;
pub mod data_bios;
pub mod mem;
pub mod rng;
pub mod ppc;
pub mod prof;
pub mod lifted;
pub mod stubs;
/// Readable port of the status-id API; incomplete (hand ports pending), so off by default.
#[cfg(feature = "wip-status")]
pub mod status;
#[cfg(feature = "wip-status")]
pub mod hand;
pub mod world;
pub mod run;
pub mod dsp_ax;
pub mod exi;
pub mod interp;
pub mod memcard;
