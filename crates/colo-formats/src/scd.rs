//! CMS script files (`.scd`) and instruction decoding.
//!
//! Header (big-endian): u16 script_id @0 (matched against `thread_id >> 16`), u16 @2 (always 0x101 seen),
//! u16 function count @4, u32 @0x0C (end of function table), u32 data_offset @0x10 (thread "global" base,
//! the first byte after the function table), u32 @0x14 runtime link pointer (0 on disc),
//! u32 function_offset[count] @0x18. All code offsets and jump targets are relative to the file start.
//! Opcode set verified against the interpreter at 0x800F6BC4 (dispatch table 0x803155D0).

use crate::be::{u16_at, u32_at};
use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Script {
    pub script_id: u16,
    pub unk2: u16,
    pub data_offset: usize,
    pub functions: Vec<usize>,
    pub bytes: Vec<u8>,
}

impl Script {
    pub fn parse(bytes: Vec<u8>) -> Result<Self> {
        let script_id = u16_at(&bytes, 0, "scd header")?;
        let unk2 = u16_at(&bytes, 2, "scd header")?;
        let n = u16_at(&bytes, 4, "scd header")? as usize;
        let data_offset = u32_at(&bytes, 0x10, "scd header")? as usize;
        let mut functions = Vec::with_capacity(n);
        for i in 0..n {
            functions.push(u32_at(&bytes, 0x18 + 4 * i, "scd function table")? as usize);
        }
        if data_offset > bytes.len() {
            return Err(Error::Invalid(format!("scd data offset {data_offset:#x} past end")));
        }
        Ok(Script { script_id, unk2, data_offset, functions, bytes })
    }

    /// Thread id for function `index` as scripts write it: `(script_id << 16) | index`.
    pub fn thread_id(&self, index: u16) -> u32 {
        ((self.script_id as u32) << 16) | index as u32
    }
}

/// Variable operand byte.
///
/// low 5 bits = value type (2 = int, 3 = float); 0x80 = value is already on the stack;
/// otherwise an index is popped: 0x40 = local (frame-relative stack slot) else data-area cell;
/// 0x20 = produce the slot's address instead of its value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Var(pub u8);

impl Var {
    pub const INT: u8 = 2;
    pub const FLOAT: u8 = 3;
    pub fn ty(self) -> u8 { self.0 & 0x1F }
    /// The interpreter compares `flag & 0x3F` against 2 to decide "int".
    pub fn is_int(self) -> bool { self.0 & 0x3F == Self::INT }
    pub fn on_stack(self) -> bool { self.0 & 0x80 != 0 }
    pub fn local(self) -> bool { self.0 & 0x40 != 0 }
    pub fn address(self) -> bool { self.0 & 0x20 != 0 }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = match self.ty() { 2 => "int".to_string(), 3 => "float".to_string(), t => format!("t{t}") };
        let mut fl = vec![];
        if self.on_stack() { fl.push("stk"); }
        if self.local() { fl.push("local"); }
        if self.address() { fl.push("addr"); }
        if fl.is_empty() { write!(f, "{ty}[data]") } else { write!(f, "{ty}[{}]", fl.join(",")) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp { And, Or, Mul, Div, Mod, Add, Sub, Lt, Le, Gt, Ge, Eq, Ne, LAnd, LOr }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Nop,
    Exit,
    PushMinus1,
    Pop { count: u16 },
    Call { func: u32, a: u16, n: u16 },
    Return,
    Jump { target: u32 },
    JumpFalse { target: u32 },
    PushImm { ty: u8, value: u32 },
    Bin { op: BinOp, rhs: Var, lhs: Var },
    /// opcode 15 (GoD Tool "pushVar"); behaves like 19 (load).
    Load15 { var: Var },
    LoadMulti { var: Var, count: u16 },
    Not { var: Var },
    Negate { var: Var },
    Load { var: Var },
    SetVar { src: Var, dst: Var },
    CopyStruct { src: Var, dst: Var, count: u16 },
    Printf { argc: u16 },
    Yield { argc: u16 },
    CallStd { argc: u16 },
}

impl Op {
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Op::Nop => "nop", Op::Exit => "exit", Op::PushMinus1 => "pushMinus1", Op::Pop { .. } => "pop",
            Op::Call { .. } => "call", Op::Return => "return", Op::Jump { .. } => "jump",
            Op::JumpFalse { .. } => "jumpFalse", Op::PushImm { .. } => "pushImm",
            Op::Bin { op, .. } => match op {
                BinOp::And => "and", BinOp::Or => "or", BinOp::Mul => "mul", BinOp::Div => "div",
                BinOp::Mod => "mod", BinOp::Add => "add", BinOp::Sub => "sub", BinOp::Lt => "lt",
                BinOp::Le => "le", BinOp::Gt => "gt", BinOp::Ge => "ge", BinOp::Eq => "eq",
                BinOp::Ne => "ne", BinOp::LAnd => "land", BinOp::LOr => "lor",
            },
            Op::Load15 { .. } => "load15", Op::LoadMulti { .. } => "loadMulti", Op::Not { .. } => "not",
            Op::Negate { .. } => "negate", Op::Load { .. } => "load", Op::SetVar { .. } => "setVar",
            Op::CopyStruct { .. } => "copyStruct", Op::Printf { .. } => "printf", Op::Yield { .. } => "yield",
            Op::CallStd { .. } => "callstd",
        }
    }
}

/// Decode one instruction at `pc`. Returns the op and the offset of the next instruction.
pub fn decode(code: &[u8], pc: usize) -> Result<(Op, usize)> {
    let b = |o: usize| code.get(pc + o).copied().ok_or(Error::Truncated { what: "scd instruction", offset: pc + o, need: 1, have: 0 });
    let h = |o: usize| u16_at(code, pc + o, "scd instruction");
    let w = |o: usize| u32_at(code, pc + o, "scd instruction");
    let v = |o: usize| b(o).map(Var);
    let opc = b(0)?;
    let bin = |op| -> Result<(Op, usize)> { Ok((Op::Bin { op, rhs: v(1)?, lhs: v(2)? }, pc + 3)) };
    Ok(match opc {
        0 => (Op::Nop, pc + 1),
        1 => (Op::Exit, pc + 1),
        2 => (Op::PushMinus1, pc + 1),
        3 => (Op::Pop { count: h(2)? }, pc + 4),
        4 => (Op::Call { func: w(1)?, a: h(5)?, n: h(7)? }, pc + 9),
        5 => (Op::Return, pc + 1),
        6 => (Op::Jump { target: w(1)? }, pc + 5),
        7 => (Op::JumpFalse { target: w(1)? }, pc + 5),
        8 => (Op::PushImm { ty: b(1)?, value: w(2)? }, pc + 6),
        13 => return bin(BinOp::And),
        14 => return bin(BinOp::Or),
        15 => (Op::Load15 { var: v(1)? }, pc + 2),
        16 => (Op::LoadMulti { var: v(1)?, count: h(2)? }, pc + 4),
        17 => (Op::Not { var: v(1)? }, pc + 2),
        18 => (Op::Negate { var: v(1)? }, pc + 2),
        19 => (Op::Load { var: v(1)? }, pc + 2),
        20 => return bin(BinOp::Mul),
        21 => return bin(BinOp::Div),
        22 => return bin(BinOp::Mod),
        23 => return bin(BinOp::Add),
        24 => return bin(BinOp::Sub),
        25 => return bin(BinOp::Lt),
        26 => return bin(BinOp::Le),
        27 => return bin(BinOp::Gt),
        28 => return bin(BinOp::Ge),
        29 => return bin(BinOp::Eq),
        30 => return bin(BinOp::Ne),
        31 => (Op::SetVar { src: v(1)?, dst: v(2)? }, pc + 3),
        32 => (Op::CopyStruct { src: v(1)?, dst: v(2)?, count: h(3)? }, pc + 5),
        33 => return bin(BinOp::LAnd),
        34 => return bin(BinOp::LOr),
        35 => (Op::Printf { argc: h(3)? }, pc + 5),
        36 => (Op::Yield { argc: h(3)? }, pc + 5),
        37 => (Op::CallStd { argc: h(3)? }, pc + 5),
        other => return Err(Error::Invalid(format!("invalid CMS opcode {other} at {pc:#x}"))),
    })
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = self.mnemonic();
        match self {
            Op::Pop { count } => write!(f, "{m} {count}"),
            Op::Call { func, a, n } => write!(f, "{m} func_{func} a={a} n={n}"),
            Op::Jump { target } | Op::JumpFalse { target } => write!(f, "{m} @{target:#x}"),
            Op::PushImm { ty: 3, value } => write!(f, "{m} {}f", f32::from_bits(*value)),
            Op::PushImm { value, .. } => write!(f, "{m} {}", *value as i32),
            Op::Bin { rhs, lhs, .. } => write!(f, "{m} {lhs}, {rhs}"),
            Op::Load15 { var } | Op::Not { var } | Op::Negate { var } | Op::Load { var } => write!(f, "{m} {var}"),
            Op::LoadMulti { var, count } => write!(f, "{m} {var} x{count}"),
            Op::SetVar { src, dst } => write!(f, "{m} {dst} = {src}"),
            Op::CopyStruct { src, dst, count } => write!(f, "{m} {dst} <- {src} x{count}"),
            Op::Printf { argc } | Op::Yield { argc } | Op::CallStd { argc } => write!(f, "{m} argc={argc}"),
            _ => write!(f, "{m}"),
        }
    }
}
