//! Interpreter for Pokémon Colosseum CMS script bytecode (`.scd`).
//!
//! Semantics follow the original interpreter in main.dol (GC6E01): loop at 0x800F6BC4,
//! opcode handlers via table 0x803155D0, thread creation at 0x800F6D18, natives via the
//! callstd table at 0x802E1CF0. Scripts are cooperative threads: they run until they yield
//! (`yield` op, or a native that reports [`NativeResult::Pending`]) and resume on the next tick.

pub mod natives;
pub mod vm;

pub use natives::{native, ArgType, NativeInfo, NATIVES};
pub use vm::{Cell, Host, Memory, NativeCall, NativeResult, ThreadStatus, TraceHost, Vm};
