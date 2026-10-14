//! Battle move scripts ("WS", waza sequence): the byte-coded scripts in main.dol `.data` that the
//! battle engine runs for every move effect, much like the Gen 3 battle scripts.
//!
//! Engine side: the runner is `fn_80211B94` (also `fn_8022FE80`/`fn_8022FF90`); it calls
//! `OPS[*pc].handler` through the table at 0x8027A00C until the state byte 0x8047B614 becomes
//! 1 (end) or 2 (return). The script PC is the u32 at 0x8047B610.
//!
//! Operand layouts were read from each handler's decompiled body (how far it advances the PC and
//! which offsets it reads). They are checked by decoding every reachable script with no unknown
//! opcode, out-of-range pointer or overlap, and against Dolphin traces of real battles.

use crate::dol::Dol;
use crate::{Error, Result};
use std::collections::BTreeMap;

pub struct Op {
    pub handler: u32,
    pub name: Option<&'static str>,
    pub len: u8,
    pub fall: bool,
    pub args: &'static str,
}

/// Move-effect → script table (unaligned, right after the scripts). Index = a move's effect id.
pub const EFFECT_TABLE: u32 = 0x80379BFF;
pub const EFFECT_COUNT: u32 = 214;
/// Script bytes live in [SCRIPT_START, SCRIPT_END).
pub const SCRIPT_START: u32 = 0x80375E48;
pub const SCRIPT_END: u32 = 0x80379BFF;
/// Scripts that handlers (and their helpers) jump to directly: (target, opcode or helper).
pub const HANDLER_TARGETS: [(u32, u8); 16] = [
    (0x80375E51, 0xEF), (0x80375E5F, 0xEF), (0x80375F98, 0xB7), (0x80375FBF, 0x00), (0x80375FDF, 0xB3),
    (0x80375FDF, 0xB5), (0x80377AB8, 0xB7), (0x80377AD9, 0xB7), (0x80378CCE, 0x8F), (0x80379021, 0x00),
    (0x803797A0, 0x93), (0x803797BB, 0x78), (0x8037984D, 0x97), (0x8037989E, 0x00), (0x803798D8, 0xD2),
    (0x80379868, 0x06),
];

#[derive(Clone, Debug, PartialEq)]
pub enum Arg {
    B(u8),
    H(u16),
    W(u32),
    /// Script pointer (0 = none).
    P(u32),
}

#[derive(Clone, Debug)]
pub struct Insn {
    pub addr: u32,
    pub op: u8,
    pub args: Vec<Arg>,
}

impl Insn {
    pub fn info(&self) -> &'static Op { OPS[self.op as usize].as_ref().expect("decoded op is valid") }
    pub fn len(&self) -> u32 { self.info().len as u32 }
    pub fn mnemonic(&self) -> String {
        match self.info().name { Some(n) => n.to_string(), None => format!("ws_{:02x}", self.op) }
    }
    pub fn targets(&self) -> impl Iterator<Item = u32> + '_ {
        self.args.iter().filter_map(|a| match a { Arg::P(p) if *p != 0 => Some(*p), _ => None })
    }
}

#[derive(Default)]
pub struct Listing {
    pub insns: BTreeMap<u32, Insn>,
    /// Why an address is an entry point: "effect N", "code 0x...", "data 0x...", "handler op XX", "ptr".
    pub labels: BTreeMap<u32, Vec<String>>,
    /// Byte ranges in the script area that nothing references; decoded linearly when they decode cleanly.
    pub unreferenced: Vec<(u32, u32)>,
    pub problems: Vec<String>,
}

fn be32(b: &[u8]) -> u32 { u32::from_be_bytes([b[0], b[1], b[2], b[3]]) }

/// Decode one command at `addr`.
pub fn decode(dol: &Dol, addr: u32) -> Result<Insn> {
    let op = dol.read(addr, 1)?[0];
    let info = OPS[op as usize].as_ref().ok_or(Error::Invalid(format!("ws: bad opcode {op:#04x} at {addr:#x}")))?;
    let b = dol.read(addr, info.len as usize)?;
    let mut o = 1;
    let mut args = vec![];
    for c in info.args.bytes() {
        args.push(match c {
            b'B' => { o += 1; Arg::B(b[o - 1]) }
            b'H' => { o += 2; Arg::H(u16::from_be_bytes([b[o - 2], b[o - 1]])) }
            b'W' => { o += 4; Arg::W(be32(&b[o - 4..o])) }
            _ => { o += 4; Arg::P(be32(&b[o - 4..o])) }
        });
    }
    Ok(Insn { addr, op, args })
}

fn in_scripts(a: u32) -> bool { (SCRIPT_START..SCRIPT_END).contains(&a) }

/// Entry points: the effect table, handler targets, `lis/addi|ori` pairs in code and aligned data words
/// outside the script area that point into it.
pub fn entry_points(dol: &Dol) -> Result<BTreeMap<u32, Vec<String>>> {
    let mut e: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for i in 0..EFFECT_COUNT {
        e.entry(be32(dol.read(EFFECT_TABLE + 4 * i, 4)?)).or_default().push(format!("effect {i}"));
    }
    for (t, op) in HANDLER_TARGETS { e.entry(t).or_default().push(format!("handler op {op:02x}")); }
    for (addr, bytes, text) in dol.sections() {
        if text {
            let mut hi = [(0u32, usize::MAX); 32];
            for (i, w) in bytes.chunks_exact(4).map(be32).enumerate() {
                let (op, rd, ra, imm) = (w >> 26, ((w >> 21) & 31) as usize, ((w >> 16) & 31) as usize, w & 0xFFFF);
                if op == 15 { hi[rd] = (imm << 16, i); continue; }
                if (op == 14 || op == 24) && hi[ra].1 != usize::MAX && i - hi[ra].1 < 12 {
                    let v = if op == 14 { hi[ra].0.wrapping_add(imm as i16 as i32 as u32) } else { hi[ra].0 | imm };
                    if in_scripts(v) { e.entry(v).or_default().push(format!("code {:#010x}", addr + 4 * i as u32)); }
                }
            }
        } else {
            for (i, w) in bytes.chunks_exact(4).map(be32).enumerate() {
                let at = addr + 4 * i as u32;
                if in_scripts(w) && !(SCRIPT_START..EFFECT_TABLE + 4 * EFFECT_COUNT).contains(&at) {
                    e.entry(w).or_default().push(format!("data {at:#010x}"));
                }
            }
        }
    }
    Ok(e)
}

/// Recursive-descent disassembly of every script reachable from `entry_points`, then a linear
/// sweep of the unreferenced gaps.
pub fn disassemble(dol: &Dol) -> Result<Listing> {
    let mut l = Listing { labels: entry_points(dol)?, ..Default::default() };
    let mut work: Vec<u32> = l.labels.keys().copied().collect();
    while let Some(mut pc) = work.pop() {
        while in_scripts(pc) && !l.insns.contains_key(&pc) {
            let ins = match decode(dol, pc) {
                Ok(i) => i,
                Err(e) => { l.problems.push(e.to_string()); break; }
            };
            for t in ins.targets() {
                if !in_scripts(t) { l.problems.push(format!("{pc:#x}: pointer {t:#x} outside the scripts")); continue; }
                if !l.insns.contains_key(&t) { work.push(t); }
                l.labels.entry(t).or_default();
            }
            let (next, fall) = (pc + ins.len(), ins.info().fall);
            l.insns.insert(pc, ins);
            if !fall { break; }
            pc = next;
        }
    }
    // overlaps
    let mut end = 0;
    for (&a, i) in &l.insns {
        if a < end { l.problems.push(format!("{a:#x}: overlaps the command before it")); }
        end = end.max(a + i.len());
    }
    // gaps
    let mut gaps = vec![];
    let mut pc = SCRIPT_START;
    for (&a, i) in &l.insns {
        if a > pc { gaps.push((pc, a)); }
        pc = pc.max(a + i.len());
    }
    if pc < SCRIPT_END { gaps.push((pc, SCRIPT_END)); }
    for (lo, hi) in gaps {
        l.unreferenced.push((lo, hi));
        let mut v = vec![];
        let mut p = lo;
        let clean = loop {
            if p == hi { break true; }
            match decode(dol, p) {
                Ok(i) if p + i.len() <= hi => { p += i.len(); v.push(i); }
                _ => break false,
            }
        };
        if clean && v.last().is_some_and(|i| !i.info().fall) {
            for i in v { l.insns.insert(i.addr, i); }
        }
    }
    Ok(l)
}

impl Listing {
    /// Text listing: labels, addresses, mnemonics and operands.
    pub fn render(&self) -> String {
        use std::fmt::Write;
        let mut s = String::new();
        let mut un = self.unreferenced.iter().peekable();
        for (&a, i) in &self.insns {
            while un.peek().is_some_and(|&&(lo, _)| lo <= a) {
                let (lo, hi) = un.next().unwrap();
                let _ = writeln!(s, "; --- unreferenced {lo:#010x}..{hi:#010x} ({} bytes)", hi - lo);
            }
            if let Some(why) = self.labels.get(&a) {
                let _ = writeln!(s, "\nL_{a:08x}:{}{}", if why.is_empty() { "" } else { "  ; " }, why.join(", "));
            }
            let args: Vec<String> = i.args.iter().map(|x| match x {
                Arg::B(v) => format!("{v:#04x}"),
                Arg::H(v) => format!("{v:#06x}"),
                Arg::W(v) => format!("{v:#010x}"),
                Arg::P(0) => "0".into(),
                Arg::P(p) => format!("L_{p:08x}"),
            }).collect();
            let _ = writeln!(s, "  {a:08x}  {:<24} {}", i.mnemonic(), args.join(", "));
            if !i.info().fall { s.push('\n'); }
        }
        s
    }
}

/// Per opcode: handler address, Ghidra name (`None` while it is still FUN_), length in bytes,
/// whether execution can continue at the next command, and operand layout after the opcode byte
/// (B u8, H u16, W u32, P u32 script pointer, 0 = no jump). Index 254 (null handler) and 255 are invalid.
pub const OPS: [Option<Op>; 256] = [
    Some(Op { handler: 0x80229E54, name: Some("WS_HITCHECK"), len: 1, fall: true, args: "" }), // 0x00
    Some(Op { handler: 0x802284B0, name: Some("fightCheckAccuracy"), len: 7, fall: true, args: "PH" }), // 0x01
    Some(Op { handler: 0x802282D8, name: None, len: 1, fall: true, args: "" }), // 0x02
    Some(Op { handler: 0x8022808C, name: None, len: 1, fall: true, args: "" }), // 0x03
    Some(Op { handler: 0x80227DB8, name: Some("WS_CRITICAL_CHECK"), len: 1, fall: true, args: "" }), // 0x04
    Some(Op { handler: 0x80227C40, name: None, len: 1, fall: true, args: "" }), // 0x05
    Some(Op { handler: 0x802274C0, name: Some("WS_TYPE_CHECK"), len: 1, fall: true, args: "" }), // 0x06
    Some(Op { handler: 0x802271AC, name: Some("WS_DAMAGE_LOSS"), len: 1, fall: true, args: "" }), // 0x07
    Some(Op { handler: 0x80227178, name: Some("WS_DAMAGE_LOSS_ONLY"), len: 1, fall: true, args: "" }), // 0x08
    Some(Op { handler: 0x80226FD4, name: None, len: 1, fall: true, args: "" }), // 0x09
    Some(Op { handler: 0x80226F0C, name: None, len: 2, fall: true, args: "B" }), // 0x0a
    Some(Op { handler: 0x80226914, name: None, len: 2, fall: true, args: "B" }), // 0x0b
    Some(Op { handler: 0x802267E8, name: None, len: 2, fall: true, args: "B" }), // 0x0c
    Some(Op { handler: 0x80226730, name: None, len: 1, fall: true, args: "" }), // 0x0d
    Some(Op { handler: 0x802266EC, name: None, len: 1, fall: true, args: "" }), // 0x0e
    Some(Op { handler: 0x8022631C, name: None, len: 1, fall: true, args: "" }), // 0x0f
    Some(Op { handler: 0x802262D0, name: None, len: 5, fall: true, args: "W" }), // 0x10
    Some(Op { handler: 0x80226284, name: None, len: 3, fall: true, args: "H" }), // 0x11
    Some(Op { handler: 0x8022622C, name: None, len: 3, fall: true, args: "H" }), // 0x12
    Some(Op { handler: 0x802261B0, name: None, len: 5, fall: true, args: "W" }), // 0x13
    Some(Op { handler: 0x80226134, name: None, len: 5, fall: true, args: "W" }), // 0x14
    Some(Op { handler: 0x80224820, name: None, len: 1, fall: true, args: "" }), // 0x15
    Some(Op { handler: 0x802247F8, name: Some("WS_TSUIKA_DIRECT_ACT"), len: 1, fall: true, args: "" }), // 0x16
    Some(Op { handler: 0x802247D0, name: Some("WS_TSUIKA_INDIRECT_ACT"), len: 1, fall: true, args: "" }), // 0x17
    Some(Op { handler: 0x80224740, name: None, len: 2, fall: true, args: "B" }), // 0x18
    Some(Op { handler: 0x80224158, name: None, len: 7, fall: true, args: "BBP" }), // 0x19
    Some(Op { handler: 0x80224060, name: None, len: 2, fall: true, args: "B" }), // 0x1a
    Some(Op { handler: 0x80223F1C, name: None, len: 2, fall: true, args: "B" }), // 0x1b
    Some(Op { handler: 0x80223E40, name: Some("WS_CONDITION_CHECK"), len: 10, fall: true, args: "BWP" }), // 0x1c
    Some(Op { handler: 0x80223D64, name: None, len: 10, fall: true, args: "BWP" }), // 0x1d
    Some(Op { handler: 0x80223CE8, name: None, len: 7, fall: true, args: "BBP" }), // 0x1e
    Some(Op { handler: 0x80223C74, name: Some("WS_SIDECONDITION_CHECK"), len: 8, fall: true, args: "BHP" }), // 0x1f
    Some(Op { handler: 0x80223AF4, name: None, len: 9, fall: true, args: "BBBBP" }), // 0x20
    Some(Op { handler: 0x80223A88, name: Some("WS_WAZAKOUKA_CHECK"), len: 11, fall: true, args: "BWBP" }), // 0x21
    Some(Op { handler: 0x80223A24, name: Some("WS_CHECK_TYPE"), len: 7, fall: true, args: "BBP" }), // 0x22
    Some(Op { handler: 0x802232F4, name: None, len: 2, fall: true, args: "B" }), // 0x23
    Some(Op { handler: 0x80222C44, name: None, len: 5, fall: true, args: "W" }), // 0x24
    Some(Op { handler: 0x80222BD8, name: None, len: 1, fall: true, args: "" }), // 0x25
    Some(Op { handler: 0x80222B7C, name: None, len: 2, fall: true, args: "B" }), // 0x26
    Some(Op { handler: 0x80222ADC, name: None, len: 5, fall: true, args: "P" }), // 0x27
    Some(Op { handler: 0x80222ACC, name: None, len: 5, fall: false, args: "P" }), // 0x28
    Some(Op { handler: 0x802229EC, name: None, len: 11, fall: true, args: "BWBP" }), // 0x29
    Some(Op { handler: 0x8022290C, name: None, len: 12, fall: true, args: "BWHP" }), // 0x2a
    Some(Op { handler: 0x80222844, name: None, len: 14, fall: true, args: "BWWP" }), // 0x2b
    Some(Op { handler: 0x802227D4, name: None, len: 14, fall: true, args: "WWBP" }), // 0x2c
    Some(Op { handler: 0x8022275C, name: None, len: 14, fall: true, args: "WWBP" }), // 0x2d
    Some(Op { handler: 0x8022273C, name: None, len: 6, fall: true, args: "WB" }), // 0x2e
    Some(Op { handler: 0x80222714, name: None, len: 6, fall: true, args: "WB" }), // 0x2f
    Some(Op { handler: 0x802226EC, name: None, len: 6, fall: true, args: "WB" }), // 0x30
    Some(Op { handler: 0x802226A4, name: None, len: 10, fall: true, args: "WWB" }), // 0x31
    Some(Op { handler: 0x80222654, name: None, len: 14, fall: true, args: "WWWB" }), // 0x32
    Some(Op { handler: 0x8022262C, name: None, len: 6, fall: true, args: "WB" }), // 0x33
    Some(Op { handler: 0x80222604, name: None, len: 7, fall: true, args: "WH" }), // 0x34
    Some(Op { handler: 0x802225DC, name: None, len: 9, fall: true, args: "WW" }), // 0x35
    Some(Op { handler: 0x802225B0, name: None, len: 6, fall: true, args: "WB" }), // 0x36
    Some(Op { handler: 0x80222584, name: None, len: 7, fall: true, args: "WH" }), // 0x37
    Some(Op { handler: 0x80222554, name: None, len: 9, fall: true, args: "WW" }), // 0x38
    Some(Op { handler: 0x80222520, name: None, len: 3, fall: true, args: "H" }), // 0x39
    Some(Op { handler: 0x80222510, name: None, len: 1, fall: true, args: "" }), // 0x3a
    Some(Op { handler: 0x80222500, name: None, len: 2, fall: true, args: "B" }), // 0x3b
    Some(Op { handler: 0x802224F4, name: Some("WS_SEQRET"), len: 1, fall: false, args: "" }), // 0x3c
    Some(Op { handler: 0x802224E8, name: Some("WS_WAZAEND"), len: 1, fall: false, args: "" }), // 0x3d
    Some(Op { handler: 0x802224DC, name: Some("WS_SEQEND"), len: 1, fall: false, args: "" }), // 0x3e
    Some(Op { handler: 0x802224D0, name: Some("WS_SPEABIEND"), len: 1, fall: false, args: "" }), // 0x3f
    Some(Op { handler: 0x80229C90, name: None, len: 5, fall: true, args: "P" }), // 0x40
    Some(Op { handler: 0x80222494, name: None, len: 5, fall: true, args: "P" }), // 0x41
    Some(Op { handler: 0x80222438, name: None, len: 7, fall: true, args: "BBP" }), // 0x42
    Some(Op { handler: 0x802223E0, name: None, len: 6, fall: true, args: "BP" }), // 0x43
    Some(Op { handler: 0x802223D4, name: Some("WS_ALERTEND"), len: 1, fall: false, args: "" }), // 0x44
    Some(Op { handler: 0x80222370, name: None, len: 7, fall: true, args: "BBW" }), // 0x45
    Some(Op { handler: 0x802222F4, name: None, len: 10, fall: true, args: "BWW" }), // 0x46
    Some(Op { handler: 0x8022106C, name: None, len: 1, fall: true, args: "" }), // 0x47
    Some(Op { handler: 0x80220868, name: None, len: 4, fall: true, args: "BBB" }), // 0x48
    Some(Op { handler: 0x8021FAD4, name: None, len: 3, fall: true, args: "BB" }), // 0x49
    Some(Op { handler: 0x80227490, name: None, len: 1, fall: true, args: "" }), // 0x4a
    Some(Op { handler: 0x8021F998, name: None, len: 1, fall: true, args: "" }), // 0x4b
    Some(Op { handler: 0x8021F92C, name: None, len: 2, fall: true, args: "B" }), // 0x4c
    Some(Op { handler: 0x8021F664, name: None, len: 3, fall: true, args: "BB" }), // 0x4d
    Some(Op { handler: 0x8021F458, name: None, len: 3, fall: true, args: "BB" }), // 0x4e
    Some(Op { handler: 0x8021F39C, name: None, len: 7, fall: true, args: "BPB" }), // 0x4f
    Some(Op { handler: 0x8021F24C, name: None, len: 6, fall: true, args: "BP" }), // 0x50
    Some(Op { handler: 0x8021F1CC, name: None, len: 3, fall: true, args: "BB" }), // 0x51
    Some(Op { handler: 0x8021EF24, name: None, len: 2, fall: true, args: "B" }), // 0x52
    Some(Op { handler: 0x8021EF14, name: None, len: 2, fall: true, args: "B" }), // 0x53
    Some(Op { handler: 0x8021EED4, name: None, len: 3, fall: true, args: "H" }), // 0x54
    Some(Op { handler: 0x8021EE98, name: None, len: 3, fall: true, args: "H" }), // 0x55
    Some(Op { handler: 0x8021EE48, name: None, len: 2, fall: true, args: "B" }), // 0x56
    Some(Op { handler: 0x8021EE38, name: None, len: 1, fall: true, args: "" }), // 0x57
    Some(Op { handler: 0x8021ED70, name: None, len: 2, fall: true, args: "B" }), // 0x58
    Some(Op { handler: 0x8021EC00, name: Some("WS_WAZAOBOE_CHECK"), len: 10, fall: true, args: "PPB" }), // 0x59
    Some(Op { handler: 0x8021EAE8, name: None, len: 5, fall: true, args: "P" }), // 0x5a
    Some(Op { handler: 0x8021EA94, name: None, len: 5, fall: true, args: "P" }), // 0x5b
    Some(Op { handler: 0x8021E9F4, name: None, len: 2, fall: true, args: "B" }), // 0x5c
    Some(Op { handler: 0x8021E754, name: None, len: 5, fall: true, args: "P" }), // 0x5d
    Some(Op { handler: 0x8021E744, name: None, len: 2, fall: true, args: "B" }), // 0x5e
    Some(Op { handler: 0x8021E6FC, name: Some("WS_SWITCH_A_D"), len: 1, fall: true, args: "" }), // 0x5f
    Some(Op { handler: 0x8021E6EC, name: None, len: 2, fall: true, args: "B" }), // 0x60
    Some(Op { handler: 0x8021E6DC, name: None, len: 2, fall: true, args: "B" }), // 0x61
    Some(Op { handler: 0x8021E6CC, name: None, len: 2, fall: true, args: "B" }), // 0x62
    Some(Op { handler: 0x8021E600, name: None, len: 2, fall: false, args: "B" }), // 0x63
    Some(Op { handler: 0x8021E288, name: None, len: 2, fall: true, args: "B" }), // 0x64
    Some(Op { handler: 0x8021E04C, name: None, len: 6, fall: true, args: "BW" }), // 0x65
    Some(Op { handler: 0x8021DF80, name: None, len: 7, fall: true, args: "BBW" }), // 0x66
    Some(Op { handler: 0x8021DF70, name: None, len: 1, fall: true, args: "" }), // 0x67
    Some(Op { handler: 0x8021DEFC, name: Some("WS_SCA_END_SET"), len: 1, fall: true, args: "" }), // 0x68
    Some(Op { handler: 0x8021DEC8, name: Some("WS_KORAERU_CHECK"), len: 1, fall: true, args: "" }), // 0x69
    Some(Op { handler: 0x8021DE4C, name: None, len: 2, fall: true, args: "B" }), // 0x6a
    Some(Op { handler: 0x8021DE3C, name: None, len: 1, fall: true, args: "" }), // 0x6b
    Some(Op { handler: 0x8021DDD8, name: None, len: 1, fall: true, args: "" }), // 0x6c
    Some(Op { handler: 0x8021DDC8, name: None, len: 1, fall: true, args: "" }), // 0x6d
    Some(Op { handler: 0x8021DDB8, name: None, len: 1, fall: true, args: "" }), // 0x6e
    Some(Op { handler: 0x8021DD44, name: None, len: 2, fall: true, args: "B" }), // 0x6f
    Some(Op { handler: 0x8021DD34, name: None, len: 1, fall: true, args: "" }), // 0x70
    Some(Op { handler: 0x8021ECF8, name: None, len: 1, fall: true, args: "" }), // 0x71
    Some(Op { handler: 0x8021DD24, name: None, len: 5, fall: true, args: "BBBB" }), // 0x72
    Some(Op { handler: 0x8021DB78, name: None, len: 2, fall: true, args: "B" }), // 0x73
    Some(Op { handler: 0x8021D9C0, name: None, len: 2, fall: true, args: "B" }), // 0x74
    Some(Op { handler: 0x8021D688, name: None, len: 1, fall: true, args: "" }), // 0x75
    Some(Op { handler: 0x8021D40C, name: None, len: 3, fall: true, args: "BB" }), // 0x76
    Some(Op { handler: 0x8021D224, name: None, len: 1, fall: true, args: "" }), // 0x77
    Some(Op { handler: 0x8021D090, name: None, len: 1, fall: true, args: "" }), // 0x78
    Some(Op { handler: 0x8021D010, name: None, len: 1, fall: true, args: "" }), // 0x79
    Some(Op { handler: 0x8021CF3C, name: None, len: 5, fall: true, args: "P" }), // 0x7a
    Some(Op { handler: 0x8021CE60, name: None, len: 6, fall: true, args: "PB" }), // 0x7b
    Some(Op { handler: 0x8021CCE0, name: None, len: 1, fall: true, args: "" }), // 0x7c
    Some(Op { handler: 0x8021CC5C, name: None, len: 1, fall: true, args: "" }), // 0x7d
    Some(Op { handler: 0x8021CB58, name: None, len: 1, fall: true, args: "" }), // 0x7e
    Some(Op { handler: 0x8021CA00, name: None, len: 1, fall: true, args: "" }), // 0x7f
    Some(Op { handler: 0x8021C900, name: None, len: 2, fall: true, args: "B" }), // 0x80
    Some(Op { handler: 0x8021C75C, name: None, len: 5, fall: true, args: "P" }), // 0x81
    Some(Op { handler: 0x8021C704, name: Some("WS_NEKODAMASHI"), len: 5, fall: true, args: "P" }), // 0x82
    Some(Op { handler: 0x8021C6F4, name: None, len: 1, fall: true, args: "" }), // 0x83
    Some(Op { handler: 0x8021C588, name: None, len: 5, fall: true, args: "P" }), // 0x84
    Some(Op { handler: 0x8021C490, name: None, len: 1, fall: true, args: "" }), // 0x85
    Some(Op { handler: 0x8021C308, name: None, len: 5, fall: true, args: "P" }), // 0x86
    Some(Op { handler: 0x8021C190, name: None, len: 5, fall: true, args: "P" }), // 0x87
    Some(Op { handler: 0x8021C0F4, name: None, len: 1, fall: true, args: "" }), // 0x88
    Some(Op { handler: 0x8021B8B8, name: None, len: 6, fall: true, args: "BW" }), // 0x89
    Some(Op { handler: 0x8021B830, name: Some("WS_KUROIKIRI"), len: 1, fall: true, args: "" }), // 0x8a
    Some(Op { handler: 0x8021B760, name: None, len: 1, fall: true, args: "" }), // 0x8b
    Some(Op { handler: 0x8021B70C, name: Some("WS_ABARERU"), len: 1, fall: true, args: "" }), // 0x8c
    Some(Op { handler: 0x8021B628, name: None, len: 2, fall: true, args: "B" }), // 0x8d
    Some(Op { handler: 0x8021B610, name: None, len: 1, fall: true, args: "" }), // 0x8e
    Some(Op { handler: 0x8021B484, name: None, len: 5, fall: false, args: "P" }), // 0x8f
    Some(Op { handler: 0x8021B1A4, name: None, len: 5, fall: true, args: "P" }), // 0x90
    Some(Op { handler: 0x8021B0B0, name: None, len: 1, fall: true, args: "" }), // 0x91
    Some(Op { handler: 0x8021AFAC, name: None, len: 1, fall: true, args: "" }), // 0x92
    Some(Op { handler: 0x8021AC1C, name: Some("fightCheckOneHitKO"), len: 5, fall: true, args: "P" }), // 0x93
    Some(Op { handler: 0x8021AB9C, name: None, len: 1, fall: true, args: "" }), // 0x94
    Some(Op { handler: 0x8021AB18, name: Some("WS_WEATHER_CHANGE"), len: 1, fall: true, args: "" }), // 0x95
    Some(Op { handler: 0x8021A984, name: None, len: 1, fall: true, args: "" }), // 0x96
    Some(Op { handler: 0x8021A878, name: None, len: 5, fall: true, args: "P" }), // 0x97
    Some(Op { handler: 0x8021A80C, name: None, len: 2, fall: true, args: "B" }), // 0x98
    Some(Op { handler: 0x8021A764, name: None, len: 1, fall: true, args: "" }), // 0x99
    Some(Op { handler: 0x8021A6CC, name: None, len: 1, fall: true, args: "" }), // 0x9a
    Some(Op { handler: 0x8021A478, name: None, len: 1, fall: true, args: "" }), // 0x9b
    Some(Op { handler: 0x8021A338, name: None, len: 1, fall: true, args: "" }), // 0x9c
    Some(Op { handler: 0x8021A054, name: None, len: 5, fall: true, args: "P" }), // 0x9d
    Some(Op { handler: 0x80219E10, name: None, len: 1, fall: false, args: "" }), // 0x9e
    Some(Op { handler: 0x80219D98, name: None, len: 1, fall: true, args: "" }), // 0x9f
    Some(Op { handler: 0x80219CF4, name: None, len: 1, fall: true, args: "" }), // 0xa0
    Some(Op { handler: 0x80219B2C, name: None, len: 5, fall: true, args: "P" }), // 0xa1
    Some(Op { handler: 0x80219964, name: None, len: 5, fall: true, args: "P" }), // 0xa2
    Some(Op { handler: 0x80219838, name: None, len: 5, fall: true, args: "P" }), // 0xa3
    Some(Op { handler: 0x802196A8, name: None, len: 5, fall: true, args: "P" }), // 0xa4
    Some(Op { handler: 0x802195A0, name: None, len: 5, fall: true, args: "P" }), // 0xa5
    Some(Op { handler: 0x80219354, name: None, len: 5, fall: true, args: "P" }), // 0xa6
    Some(Op { handler: 0x802192B4, name: None, len: 1, fall: true, args: "" }), // 0xa7
    Some(Op { handler: 0x8021908C, name: None, len: 5, fall: true, args: "P" }), // 0xa8
    Some(Op { handler: 0x80218D24, name: None, len: 5, fall: true, args: "P" }), // 0xa9
    Some(Op { handler: 0x80218C74, name: Some("WS_MICHIDURE"), len: 5, fall: true, args: "P" }), // 0xaa
    Some(Op { handler: 0x80218BD4, name: None, len: 1, fall: true, args: "" }), // 0xab
    Some(Op { handler: 0x80218A6C, name: None, len: 1, fall: true, args: "" }), // 0xac
    Some(Op { handler: 0x80218824, name: None, len: 5, fall: true, args: "P" }), // 0xad
    Some(Op { handler: 0x8021847C, name: None, len: 1, fall: true, args: "" }), // 0xae
    Some(Op { handler: 0x802183BC, name: None, len: 5, fall: true, args: "P" }), // 0xaf
    Some(Op { handler: 0x802182D4, name: None, len: 5, fall: true, args: "P" }), // 0xb0
    Some(Op { handler: 0x80218270, name: Some("WS_MIYABURU"), len: 1, fall: true, args: "" }), // 0xb1
    Some(Op { handler: 0x80218018, name: None, len: 5, fall: true, args: "P" }), // 0xb2
    Some(Op { handler: 0x80217E20, name: None, len: 1, fall: true, args: "" }), // 0xb3
    Some(Op { handler: 0x80217D34, name: None, len: 6, fall: true, args: "BP" }), // 0xb4
    Some(Op { handler: 0x80217C04, name: None, len: 1, fall: true, args: "" }), // 0xb5
    Some(Op { handler: 0x80217AE4, name: None, len: 1, fall: true, args: "" }), // 0xb6
    Some(Op { handler: 0x8021799C, name: None, len: 1, fall: false, args: "" }), // 0xb7
    Some(Op { handler: 0x802178F4, name: None, len: 1, fall: true, args: "" }), // 0xb8
    Some(Op { handler: 0x802177E4, name: None, len: 1, fall: true, args: "" }), // 0xb9
    Some(Op { handler: 0x802175A8, name: None, len: 5, fall: true, args: "P" }), // 0xba
    Some(Op { handler: 0x80217524, name: None, len: 1, fall: true, args: "" }), // 0xbb
    Some(Op { handler: 0x80217434, name: None, len: 5, fall: true, args: "P" }), // 0xbc
    Some(Op { handler: 0x802173D4, name: Some("WS_JIKOANJI"), len: 5, fall: true, args: "BBBB" }), // 0xbd
    Some(Op { handler: 0x80217220, name: None, len: 1, fall: true, args: "" }), // 0xbe
    Some(Op { handler: 0x802171BC, name: Some("WS_MARUKUNARU"), len: 1, fall: true, args: "" }), // 0xbf
    Some(Op { handler: 0x802170B4, name: None, len: 5, fall: true, args: "P" }), // 0xc0
    Some(Op { handler: 0x80217018, name: None, len: 1, fall: true, args: "" }), // 0xc1
    Some(Op { handler: 0x80216F50, name: None, len: 1, fall: true, args: "" }), // 0xc2
    Some(Op { handler: 0x80216D9C, name: None, len: 5, fall: true, args: "P" }), // 0xc3
    Some(Op { handler: 0x80216A58, name: None, len: 9, fall: true, args: "PP" }), // 0xc4
    Some(Op { handler: 0x80216960, name: Some("WS_KIERUTAME"), len: 1, fall: true, args: "" }), // 0xc5
    Some(Op { handler: 0x80216874, name: Some("WS_KIERUTAME_AFTAR"), len: 1, fall: true, args: "" }), // 0xc6
    Some(Op { handler: 0x80216804, name: Some("WS_CHIISAKUNARU"), len: 1, fall: true, args: "" }), // 0xc7
    Some(Op { handler: 0x80216780, name: None, len: 1, fall: true, args: "" }), // 0xc8
    Some(Op { handler: 0x80216650, name: None, len: 5, fall: true, args: "P" }), // 0xc9
    Some(Op { handler: 0x802165B4, name: None, len: 1, fall: true, args: "" }), // 0xca
    Some(Op { handler: 0x80216550, name: Some("WS_JUUDEN"), len: 1, fall: true, args: "" }), // 0xcb
    Some(Op { handler: 0x80216410, name: None, len: 1, fall: false, args: "" }), // 0xcc
    Some(Op { handler: 0x80216364, name: None, len: 5, fall: true, args: "P" }), // 0xcd
    Some(Op { handler: 0x802162F0, name: Some("WS_ICHAMON"), len: 5, fall: true, args: "P" }), // 0xce
    Some(Op { handler: 0x80216264, name: None, len: 5, fall: true, args: "P" }), // 0xcf
    Some(Op { handler: 0x802161F0, name: Some("WS_CHOUHATSU"), len: 5, fall: true, args: "P" }), // 0xd0
    Some(Op { handler: 0x802160EC, name: None, len: 5, fall: true, args: "P" }), // 0xd1
    Some(Op { handler: 0x80215CF0, name: None, len: 5, fall: true, args: "P" }), // 0xd2
    Some(Op { handler: 0x80215C70, name: Some("WS_NARIKIRI"), len: 5, fall: true, args: "P" }), // 0xd3
    Some(Op { handler: 0x80215AEC, name: None, len: 6, fall: true, args: "BP" }), // 0xd4
    Some(Op { handler: 0x80215A78, name: Some("WS_NEWOHARU"), len: 5, fall: true, args: "P" }), // 0xd5
    Some(Op { handler: 0x80215954, name: None, len: 1, fall: true, args: "" }), // 0xd6
    Some(Op { handler: 0x802158D0, name: Some("WS_AKUBI"), len: 5, fall: true, args: "P" }), // 0xd7
    Some(Op { handler: 0x80215808, name: None, len: 5, fall: true, args: "P" }), // 0xd8
    Some(Op { handler: 0x80215720, name: None, len: 1, fall: true, args: "" }), // 0xd9
    Some(Op { handler: 0x80215614, name: None, len: 5, fall: true, args: "P" }), // 0xda
    Some(Op { handler: 0x80215374, name: None, len: 5, fall: true, args: "P" }), // 0xdb
    Some(Op { handler: 0x80215300, name: Some("WS_ONNEN"), len: 5, fall: true, args: "P" }), // 0xdc
    Some(Op { handler: 0x802151C0, name: None, len: 1, fall: true, args: "" }), // 0xdd
    Some(Op { handler: 0x80214F10, name: None, len: 5, fall: true, args: "P" }), // 0xde
    Some(Op { handler: 0x80214E50, name: None, len: 5, fall: true, args: "P" }), // 0xdf
    Some(Op { handler: 0x80214DB0, name: None, len: 5, fall: true, args: "P" }), // 0xe0
    Some(Op { handler: 0x80214CB0, name: None, len: 5, fall: true, args: "BBBB" }), // 0xe1
    Some(Op { handler: 0x80214C04, name: None, len: 2, fall: true, args: "B" }), // 0xe2
    Some(Op { handler: 0x80214BB4, name: None, len: 6, fall: true, args: "BP" }), // 0xe3
    Some(Op { handler: 0x80214B68, name: Some("WS_HIMITUNOTIKARA"), len: 1, fall: true, args: "" }), // 0xe4
    Some(Op { handler: 0x80214B58, name: None, len: 1, fall: true, args: "" }), // 0xe5
    Some(Op { handler: 0x80214AFC, name: None, len: 1, fall: true, args: "" }), // 0xe6
    Some(Op { handler: 0x80214AB4, name: None, len: 1, fall: true, args: "" }), // 0xe7
    Some(Op { handler: 0x802149B8, name: None, len: 5, fall: true, args: "P" }), // 0xe8
    Some(Op { handler: 0x80214864, name: None, len: 1, fall: true, args: "" }), // 0xe9
    Some(Op { handler: 0x80214794, name: None, len: 5, fall: true, args: "P" }), // 0xea
    Some(Op { handler: 0x802146C4, name: None, len: 5, fall: true, args: "P" }), // 0xeb
    Some(Op { handler: 0x802146B4, name: None, len: 5, fall: true, args: "BBBB" }), // 0xec
    Some(Op { handler: 0x802145C8, name: None, len: 1, fall: true, args: "" }), // 0xed
    Some(Op { handler: 0x80214450, name: None, len: 1, fall: true, args: "" }), // 0xee
    Some(Op { handler: 0x80213E94, name: None, len: 1, fall: false, args: "" }), // 0xef
    Some(Op { handler: 0x80213A78, name: None, len: 5, fall: true, args: "P" }), // 0xf0
    Some(Op { handler: 0x80213A68, name: None, len: 5, fall: true, args: "BBBB" }), // 0xf1
    Some(Op { handler: 0x80213A58, name: None, len: 1, fall: true, args: "" }), // 0xf2
    Some(Op { handler: 0x80213A48, name: None, len: 5, fall: true, args: "BBBB" }), // 0xf3
    Some(Op { handler: 0x80213A38, name: None, len: 1, fall: true, args: "" }), // 0xf4
    Some(Op { handler: 0x80213A28, name: None, len: 1, fall: true, args: "" }), // 0xf5
    Some(Op { handler: 0x80213A1C, name: Some("WS_ITEMEND"), len: 1, fall: false, args: "" }), // 0xf6
    Some(Op { handler: 0x80213A10, name: Some("WS_GETEND"), len: 1, fall: false, args: "" }), // 0xf7
    Some(Op { handler: 0x80213980, name: Some("WS_STATUS_GET8"), len: 20, fall: true, args: "BWWHWW" }), // 0xf8
    Some(Op { handler: 0x802138F0, name: Some("WS_STATUS_GET16"), len: 20, fall: true, args: "BWWHWW" }), // 0xf9
    Some(Op { handler: 0x80213860, name: Some("WS_STATUS_GET32"), len: 20, fall: true, args: "BWWHWW" }), // 0xfa
    Some(Op { handler: 0x802137D8, name: Some("WS_STATUS_SET8"), len: 20, fall: true, args: "BWWHWW" }), // 0xfb
    Some(Op { handler: 0x80213750, name: Some("WS_STATUS_SET16"), len: 20, fall: true, args: "BWWHWW" }), // 0xfc
    Some(Op { handler: 0x802136C8, name: Some("WS_STATUS_SET32"), len: 20, fall: true, args: "BWWHWW" }), // 0xfd
    None, // 0xfe
    None, // 0xff
];
