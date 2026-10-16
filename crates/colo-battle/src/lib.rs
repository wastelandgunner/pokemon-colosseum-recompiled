//! Pokémon Colosseum battle engine, ported function by function from GC6E01 main.dol.
//!
//! Headless: no rendering, no I/O. Every module names the original functions it ports. The address space
//! (`mem`) is the recompile's (colo-recomp), so ported and lifted functions share one memory.

pub use colo_recomp::mem;
pub mod bios;
pub mod data_bios;
pub mod rng;
/// Readable port of the status-id API; incomplete (hand ports pending), so off by default.
#[cfg(feature = "wip-status")]
pub mod status;
#[cfg(feature = "wip-status")]
pub mod hand;
pub mod world;
