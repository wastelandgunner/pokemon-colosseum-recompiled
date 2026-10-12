//! Readers for Pokémon Colosseum (GC6E01, NTSC-U rev 0) data formats.
//!
//! Everything is big-endian. All readers are read-only and never modify their input.

pub mod be;
pub mod ccd;
pub mod common;
pub mod dol;
pub mod font;
pub mod gfl;
pub mod gpt1;
pub mod gtx;
pub mod error;
pub mod fsys;
pub mod gcm;
pub mod gx;
pub mod hsd;
pub mod hsdanim;
pub mod lzss;
pub mod maprel;
pub mod msg;
pub mod rel;
pub mod scd;
pub mod tev;
pub mod worldmap;

pub use error::{Error, Result};
