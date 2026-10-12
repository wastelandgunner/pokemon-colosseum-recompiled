use crate::natives::{native, ArgType, NativeInfo};
use colo_formats::scd::{decode, BinOp, Op, Script, Var};

/// One 32-bit stack cell. Ints and floats share cells; operand flags say how to read them.
pub type Cell = u32;

/// The original stack is checked with `depth < 0x41` before each push.
pub const STACK_CELLS: usize = 0x41;

/// Address tags for pointers handed to natives or produced by `addr` operands.
/// Data-area addresses are file offsets inside the owning script; local addresses are stack slots.
pub const ADDR_DATA: u32 = 0x4000_0000;
pub const ADDR_LOCAL: u32 = 0x8000_0000;
const ADDR_MASK: u32 = 0x00FF_FFFF;

/// VM-implemented natives (they need the thread table).
const STD_THREAD_RUN: u16 = 3;
const STD_THREAD_IS_RUNNING: u16 = 5;
const STD_THREAD_WAIT: u16 = 6;
const STD_THREAD_YIELD: u16 = 71;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeResult {
    /// Finished; the value is written to the caller's return slot.
    Done(Cell),
    /// Not finished (e.g. waiting for an animation); the native is called again next tick.
    Pending,
}

pub struct NativeCall<'a> {
    pub thread_serial: u16,
    pub thread_id: u32,
    pub index: u16,
    pub info: Option<&'static NativeInfo>,
    /// Arguments as marshalled by the original trampoline (at most 8, typed by the table).
    pub args: &'a [Cell],
    /// False when re-invoked after returning `Pending`.
    pub first_call: bool,
}

impl NativeCall<'_> {
    pub fn int(&self, i: usize) -> i32 { self.args.get(i).copied().unwrap_or(0) as i32 }
    pub fn float(&self, i: usize) -> f32 { f32::from_bits(self.args.get(i).copied().unwrap_or(0)) }
    pub fn name(&self) -> &'static str { self.info.map(|n| n.name).unwrap_or("?") }
}

/// Access to script memory for natives that take pointers (e.g. "get position" out-params).
pub trait Memory {
    fn read(&self, addr: Cell) -> Option<Cell>;
    fn write(&mut self, addr: Cell, value: Cell) -> bool;
}

pub trait Host {
    fn call_native(&mut self, call: &NativeCall, mem: &mut dyn Memory) -> NativeResult;
    /// Interpreter warnings (stack under/overflow, bad operands) and script `printf` output.
    fn log(&mut self, _line: &str) {}
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThreadStatus {
    Running,
    WaitingFrames(i32),
    WaitingNative(u16),
    Finished(Cell),
}

#[derive(Debug, Clone)]
enum Wait {
    Frames { remaining: i32 },
    Native { index: u16, args: Vec<Cell> },
}

#[derive(Debug, Clone)]
struct Thread {
    serial: u16,
    id: u32,
    script: usize,
    pc: usize,
    fp: usize,
    stack: [Cell; STACK_CELLS],
    sp: usize,
    wait: Option<Wait>,
    finished: Option<Cell>,
    kill: bool,
}

#[derive(Clone)]
struct LoadedScript {
    script: Script,
}

#[derive(Clone)]
pub struct Vm {
    scripts: Vec<LoadedScript>,
    threads: Vec<Thread>,
    next_serial: u16,
    /// Instruction budget per thread per tick, to catch scripts that spin without yielding.
    pub budget: usize,
    pub tick_count: u64,
    pub instructions_executed: u64,
}

impl Default for Vm {
    fn default() -> Self { Self::new() }
}

impl Vm {
    pub fn new() -> Self {
        Vm { scripts: Vec::new(), threads: Vec::new(), next_serial: 0, budget: 100_000, tick_count: 0, instructions_executed: 0 }
    }

    /// Load a script; threads find it by `script_id` (`thread_id >> 16`). Later loads win.
    pub fn load(&mut self, script: Script) -> usize {
        self.scripts.push(LoadedScript { script });
        self.scripts.len() - 1
    }

    fn find_script(&self, script_id: u16) -> Option<usize> {
        self.scripts.iter().rposition(|s| s.script.script_id == script_id)
    }

    /// Spawn a script thread like 0x800F6D18: `thread_id = (script_id << 16) | function`.
    /// Returns the thread serial (what scripts use as a thread handle).
    pub fn spawn(&mut self, thread_id: u32, args: &[Cell], host: &mut dyn Host) -> Option<u16> {
        let Some(si) = self.find_script((thread_id >> 16) as u16) else {
            host.log(&format!("spawn: no script loaded for thread id {thread_id:#010x}"));
            return None;
        };
        let func = (thread_id & 0xFFFF) as usize;
        let pc = match self.scripts[si].script.functions.get(func) {
            Some(&pc) if pc != 0 => pc,
            Some(_) => {
                host.log(&format!("spawn: function {func} of script {:#x} is empty (offset 0)", thread_id >> 16));
                return None;
            }
            None => {
                host.log(&format!("spawn: function {func} out of range for script {:#x}", thread_id >> 16));
                return None;
            }
        };
        // serials skip 0, like the original
        self.next_serial = self.next_serial.wrapping_add(1);
        if self.next_serial == 0 { self.next_serial = 1; }
        let mut t = Thread {
            serial: self.next_serial, id: thread_id, script: si, pc, fp: 0,
            stack: [0; STACK_CELLS], sp: 0, wait: None, finished: None, kill: false,
        };
        let mut log = |s: &str| host.log(s);
        t.push(0, &mut log);
        for &a in args { t.push(a, &mut log); }
        t.push(0, &mut log); // return pc 0 => thread ends on outermost return
        t.push(0, &mut log); // saved fp
        t.push(args.len() as Cell, &mut log);
        self.threads.push(t);
        Some(self.next_serial)
    }

    /// Run a script function to completion like 0x800F7434 (the engine's synchronous script call, used for
    /// talk handlers, interaction points and the room "modify" script). Other threads are not advanced;
    /// frame waits inside the call are counted against `max_frames`. Returns the script's result.
    pub fn call_sync(&mut self, thread_id: u32, args: &[Cell], host: &mut dyn Host, max_frames: u32) -> Option<Cell> {
        let serial = self.spawn(thread_id, args, host)?;
        let idx = self.threads.iter().position(|t| t.serial == serial)?;
        let mut frames = 0;
        loop {
            self.run_thread(idx, host);
            if let Some(r) = self.threads[idx].finished {
                self.threads.remove(idx);
                return Some(r);
            }
            frames += 1;
            if frames >= max_frames {
                host.log(&format!("call_sync {thread_id:#010x}: still running after {max_frames} frames; left as a thread"));
                return None;
            }
        }
    }

    /// Run one thread now, as far as it goes this frame (a synchronous call that may block on a native:
    /// the rest of it then runs with the normal ticks). Returns whether it is still running.
    pub fn run_now(&mut self, serial: u16, host: &mut dyn Host) -> bool {
        let Some(i) = self.threads.iter().position(|t| t.serial == serial) else { return false };
        self.run_thread(i, host);
        self.threads.iter().any(|t| t.serial == serial && t.finished.is_none())
    }

    pub fn status(&self, serial: u16) -> Option<ThreadStatus> {
        self.threads.iter().find(|t| t.serial == serial).map(|t| t.status())
    }

    pub fn threads(&self) -> impl Iterator<Item = (u16, u32, ThreadStatus)> + '_ {
        self.threads.iter().map(|t| (t.serial, t.id, t.status()))
    }

    pub fn kill(&mut self, serial: u16) {
        if let Some(t) = self.threads.iter_mut().find(|t| t.serial == serial) { t.kill = true; }
    }

    /// Remove finished threads, returning (serial, result).
    pub fn reap(&mut self) -> Vec<(u16, Cell)> {
        let mut out = vec![];
        self.threads.retain(|t| match t.finished {
            Some(r) => { out.push((t.serial, r)); false }
            None => true,
        });
        out
    }

    pub fn is_idle(&self) -> bool { self.threads.iter().all(|t| t.finished.is_some()) }

    /// Advance one frame: every live thread runs until it suspends or finishes.
    pub fn tick(&mut self, host: &mut dyn Host) {
        self.tick_count += 1;
        let mut i = 0;
        while i < self.threads.len() {
            if self.threads[i].finished.is_none() {
                self.run_thread(i, host);
            }
            i += 1; // threads spawned this tick are appended and run in the same tick
        }
    }

    fn run_thread(&mut self, ti: usize, host: &mut dyn Host) {
        let mut t = self.threads[ti].clone();
        let mut spawned: Vec<Thread> = vec![];
        let mut steps = 0usize;
        // resume a pending wait first
        if let Some(w) = t.wait.take() {
            match w {
                Wait::Frames { remaining } => {
                    if t.kill { t.finish_yield(host); self.finish(&mut t); self.threads[ti] = t; return; }
                    if remaining - 1 > 0 {
                        t.wait = Some(Wait::Frames { remaining: remaining - 1 });
                        self.threads[ti] = t;
                        return;
                    }
                    t.finish_yield(host);
                }
                Wait::Native { index, args } => {
                    match self.invoke_native(&mut t, index, &args, false, &mut spawned, host) {
                        NativeResult::Pending => {
                            t.wait = Some(Wait::Native { index, args });
                            self.threads[ti] = t;
                            self.threads.extend(spawned);
                            return;
                        }
                        NativeResult::Done(v) => t.finish_callstd(v, host),
                    }
                }
            }
        }
        loop {
            if t.kill || t.finished.is_some() {
                self.finish(&mut t);
                break;
            }
            if steps >= self.budget {
                host.log(&format!("thread {} ({:#010x}): instruction budget exceeded at pc {:#x}; suspending", t.serial, t.id, t.pc));
                break;
            }
            steps += 1;
            self.instructions_executed += 1;
            match self.step(&mut t, &mut spawned, host) {
                Flow::Continue => {}
                Flow::Suspend => break,
                Flow::Stop => { self.finish(&mut t); break; }
            }
        }
        self.threads[ti] = t;
        self.threads.extend(spawned);
    }

    fn finish(&self, t: &mut Thread) {
        if t.finished.is_none() {
            let mut sink = |_: &str| {};
            let r = t.pop(&mut sink);
            t.finished = Some(r);
            t.wait = None;
        }
    }

    fn step(&mut self, t: &mut Thread, spawned: &mut Vec<Thread>, host: &mut dyn Host) -> Flow {
        let script = &self.scripts[t.script].script;
        let (op, next) = match decode(&script.bytes, t.pc) {
            Ok(x) => x,
            Err(e) => {
                host.log(&format!("thread {}: {e}; terminating", t.serial));
                return Flow::Stop;
            }
        };
        t.pc = next;
        let mut lg = |s: &str| host.log(s);
        match op {
            Op::Nop => {}
            Op::Exit => return Flow::Stop,
            Op::PushMinus1 => t.push(0xFFFF_FFFF, &mut lg),
            Op::Pop { count } => { for _ in 0..count { t.pop(&mut lg); } }
            Op::Call { func, a, n } => {
                t.push(next as Cell, &mut lg);
                t.push(t.fp as Cell, &mut lg);
                t.push(n as Cell, &mut lg);
                t.fp = t.sp.saturating_sub(a as usize + n as usize + 3);
                match script.functions.get(func as usize) {
                    Some(&pc) => t.pc = pc,
                    None => lg(&format!("call: function {func} out of range")),
                }
            }
            Op::Return => {
                let n = t.pop(&mut lg);
                t.fp = t.pop(&mut lg) as usize;
                let pc = t.pop(&mut lg);
                for _ in 0..n { t.pop(&mut lg); }
                if pc == 0 { return Flow::Stop; }
                t.pc = pc as usize;
            }
            Op::Jump { target } => t.pc = target as usize,
            Op::JumpFalse { target } => { if t.pop(&mut lg) == 0 { t.pc = target as usize; } }
            Op::PushImm { value, .. } => t.push(value, &mut lg),
            Op::Bin { op, rhs, lhs } => {
                let a = self.read_var(t, rhs, &mut lg);
                let b = self.read_var(t, lhs, &mut lg);
                let r = binop(op, b, lhs, a, rhs, &mut lg);
                t.push(r, &mut lg);
            }
            Op::Load15 { var } | Op::Load { var } => {
                let v = self.read_var(t, var, &mut lg);
                t.push(v, &mut lg);
            }
            Op::LoadMulti { var, count } => {
                let base = t.pop(&mut lg);
                for i in 0..count as u32 {
                    t.push(base.wrapping_add(i), &mut lg);
                    let v = self.read_var(t, var, &mut lg);
                    t.push(v, &mut lg);
                }
            }
            Op::Not { var } => {
                let v = self.read_var(t, var, &mut lg);
                let r = if var.is_int() { (v == 0) as Cell } else { (((f32::from_bits(v) == 0.0) as i32) as f32).to_bits() };
                t.push(r, &mut lg);
            }
            Op::Negate { var } => {
                let v = self.read_var(t, var, &mut lg);
                let r = if var.is_int() { (v as i32).wrapping_neg() as Cell } else { (-f32::from_bits(v)).to_bits() };
                t.push(r, &mut lg);
            }
            Op::SetVar { src, dst } => {
                let mut v = self.read_var(t, src, &mut lg);
                if dst.0 & 0x3F != src.0 & 0x3F {
                    v = if src.is_int() { (v as i32 as f32).to_bits() } else { f32::from_bits(v) as i32 as Cell };
                }
                self.write_var(t, dst, v, &mut lg);
                t.push(v, &mut lg);
            }
            Op::CopyStruct { src, dst, count } => {
                // Unused by the shipped scripts; implemented from the decompiled handler (tail partly unverified).
                let n = count as usize;
                let cells: Vec<Cell> = if src.on_stack() {
                    t.sp = t.sp.saturating_sub(n);
                    t.stack[t.sp..(t.sp + n).min(STACK_CELLS)].to_vec()
                } else {
                    let idx = t.pop(&mut lg) as i32;
                    (0..n as i32).map(|k| self.load_cell(t, src.local(), idx + k, &mut lg)).collect()
                };
                if dst.on_stack() {
                    lg("copyStruct: destination on stack is not supported");
                } else {
                    let idx = t.pop(&mut lg) as i32;
                    for (k, &c) in cells.iter().enumerate() { self.store_cell(t, dst.local(), idx + k as i32, c, &mut lg); }
                }
                for c in cells { t.push(c, &mut lg); }
            }
            Op::Printf { argc } => {
                t.push(t.fp as Cell, &mut lg);
                t.push(argc as Cell, &mut lg);
                t.fp = t.sp.saturating_sub(argc as usize + 2);
                let s = self.format_printf(t);
                lg(&format!("[script {:#010x}] {s}", t.id));
                t.finish_frame(&mut lg);
            }
            Op::Yield { argc } => {
                t.push(t.fp as Cell, &mut lg);
                t.push(argc as Cell, &mut lg);
                t.fp = t.sp.saturating_sub(argc as usize + 2);
                let frames = t.stack.get(t.fp).copied().unwrap_or(0) as i32;
                if frames > 0 {
                    t.wait = Some(Wait::Frames { remaining: frames });
                    return Flow::Suspend;
                }
                t.finish_frame(&mut lg);
            }
            Op::CallStd { argc } => {
                t.push(t.fp as Cell, &mut lg);
                t.push(argc as Cell, &mut lg);
                t.fp = t.sp.saturating_sub(argc as usize + 2);
                let index = t.stack.get(t.fp).copied().unwrap_or(0) as u16;
                let count = t.sp.saturating_sub(t.fp + 3).min(8);
                let ntypes = native(index).map(|n| n.args.len()).unwrap_or(0);
                let args: Vec<Cell> = (0..count.min(ntypes)).map(|k| t.stack[t.fp + 1 + k]).collect();
                match self.invoke_native(t, index, &args, true, spawned, host) {
                    NativeResult::Pending => {
                        t.wait = Some(Wait::Native { index, args });
                        return Flow::Suspend;
                    }
                    NativeResult::Done(v) => t.finish_callstd(v, host),
                }
            }
        }
        Flow::Continue
    }

    fn invoke_native(&mut self, t: &mut Thread, index: u16, args: &[Cell], first: bool, spawned: &mut Vec<Thread>, host: &mut dyn Host) -> NativeResult {
        match index {
            STD_THREAD_RUN => {
                // (thread_id, priority, a1..a4) -> serial; priority is ignored for now.
                let id = args.first().copied().unwrap_or(0);
                let before = self.threads.len();
                let r = self.spawn(id, args.get(2..6).unwrap_or(&[]), host);
                // move the new thread into `spawned` so it is appended after this one finishes its turn
                if self.threads.len() > before { spawned.push(self.threads.pop().unwrap()); }
                return NativeResult::Done(r.unwrap_or(0) as Cell);
            }
            STD_THREAD_IS_RUNNING | STD_THREAD_WAIT => {
                let serial = args.first().copied().unwrap_or(0) as u16;
                let alive = self.threads.iter().chain(spawned.iter())
                    .any(|o| o.serial == serial && o.finished.is_none() && o.serial != t.serial);
                return if index == STD_THREAD_WAIT && alive { NativeResult::Pending } else { NativeResult::Done(alive as Cell) };
            }
            STD_THREAD_YIELD => return if first { NativeResult::Pending } else { NativeResult::Done(0) },
            _ => {}
        }
        let info = native(index);
        if info.is_none() || index == 0 {
            host.log(&format!("callstd {index}: no native at this index; returning 0"));
            return NativeResult::Done(0);
        }
        let call = NativeCall { thread_serial: t.serial, thread_id: t.id, index, info, args, first_call: first };
        let script = &mut self.scripts[t.script].script;
        let mut mem = ThreadMem { thread: t, data: &mut script.bytes };
        host.call_native(&call, &mut mem)
    }

    fn read_var(&self, t: &mut Thread, var: Var, lg: &mut dyn FnMut(&str)) -> Cell {
        if var.0 == 0 { lg("read: null variable operand"); return 0; }
        if var.on_stack() { return t.pop(lg); }
        let idx = t.pop(lg) as i32;
        if var.address() {
            if var.local() { ADDR_LOCAL | ((t.fp as i32 + idx) as u32 & ADDR_MASK) }
            else { ADDR_DATA | ((self.scripts[t.script].script.data_offset as i32 + idx * 4) as u32 & ADDR_MASK) }
        } else {
            self.load_cell(t, var.local(), idx, lg)
        }
    }

    fn write_var(&mut self, t: &mut Thread, var: Var, v: Cell, lg: &mut dyn FnMut(&str)) {
        if var.0 == 0 { lg("write: null variable operand"); return; }
        if var.on_stack() { lg("write: cannot assign to a stack value"); return; }
        let idx = t.pop(lg) as i32;
        self.store_cell(t, var.local(), idx, v, lg);
    }

    fn load_cell(&self, t: &Thread, local: bool, idx: i32, lg: &mut dyn FnMut(&str)) -> Cell {
        if local {
            let slot = t.fp as i32 + idx;
            match usize::try_from(slot).ok().and_then(|s| t.stack.get(s)) {
                Some(&v) => v,
                None => { lg(&format!("load: local slot {slot} out of range")); 0 }
            }
        } else {
            let s = &self.scripts[t.script].script;
            let off = s.data_offset as i64 + idx as i64 * 4;
            match usize::try_from(off).ok().and_then(|o| s.bytes.get(o..o + 4)) {
                Some(b) => u32::from_be_bytes([b[0], b[1], b[2], b[3]]),
                None => { lg(&format!("load: data cell {idx} out of range")); 0 }
            }
        }
    }

    fn store_cell(&mut self, t: &mut Thread, local: bool, idx: i32, v: Cell, lg: &mut dyn FnMut(&str)) {
        if local {
            let slot = t.fp as i32 + idx;
            match usize::try_from(slot).ok().and_then(|s| t.stack.get_mut(s)) {
                Some(c) => *c = v,
                None => lg(&format!("store: local slot {slot} out of range")),
            }
        } else {
            let s = &mut self.scripts[t.script].script;
            let off = s.data_offset as i64 + idx as i64 * 4;
            match usize::try_from(off).ok().and_then(|o| s.bytes.get_mut(o..o + 4)) {
                Some(b) => b.copy_from_slice(&v.to_be_bytes()),
                None => lg(&format!("store: data cell {idx} out of range")),
            }
        }
    }

    fn format_printf(&self, t: &Thread) -> String {
        let bytes = &self.scripts[t.script].script.bytes;
        let fmt_ptr = t.stack.get(t.fp).copied().unwrap_or(0);
        let fmt = cstr(bytes, (fmt_ptr & ADDR_MASK) as usize);
        let mut out = String::new();
        let mut argi = 1;
        let mut it = fmt.chars().peekable();
        while let Some(c) = it.next() {
            if c == '%' {
                let mut spec = String::new();
                while let Some(&n) = it.peek() {
                    it.next();
                    spec.push(n);
                    if "dxcfs".contains(n) { break; }
                }
                let v = t.stack.get(t.fp + argi).copied().unwrap_or(0);
                argi += 1;
                match spec.chars().last() {
                    Some('d') => out += &(v as i32).to_string(),
                    Some('x') => out += &format!("{v:x}"),
                    Some('c') => out.push(char::from_u32(v).unwrap_or('?')),
                    Some('f') => out += &f32::from_bits(v).to_string(),
                    Some('s') => out += &cstr(bytes, (v & ADDR_MASK) as usize),
                    _ => out += &spec,
                }
            } else if c == '\\' && it.peek() == Some(&'n') {
                it.next();
                out.push('\n');
            } else {
                out.push(c);
            }
        }
        out
    }
}

fn cstr(bytes: &[u8], off: usize) -> String {
    let tail = bytes.get(off..).unwrap_or(&[]);
    let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len().min(256));
    tail[..end].iter().map(|&b| b as char).collect()
}

enum Flow { Continue, Suspend, Stop }

impl Thread {
    fn status(&self) -> ThreadStatus {
        if let Some(r) = self.finished { return ThreadStatus::Finished(r); }
        match &self.wait {
            Some(Wait::Frames { remaining }) => ThreadStatus::WaitingFrames(*remaining),
            Some(Wait::Native { index, .. }) => ThreadStatus::WaitingNative(*index),
            None => ThreadStatus::Running,
        }
    }

    fn push(&mut self, v: Cell, lg: &mut dyn FnMut(&str)) {
        if self.sp < STACK_CELLS {
            self.stack[self.sp] = v;
            self.sp += 1;
        } else {
            lg(&format!("thread {}: Stack overflow!", self.serial));
        }
    }

    /// Like the original: on underflow, log and return cell 0 without moving the stack pointer.
    fn pop(&mut self, lg: &mut dyn FnMut(&str)) -> Cell {
        if self.sp < 1 {
            lg(&format!("thread {}: Stack underflow!", self.serial));
            return self.stack[0];
        }
        self.sp -= 1;
        self.stack[self.sp]
    }

    /// Epilogue shared by printf/yield/callstd: pop argc and saved fp, then drop argc cells.
    fn finish_frame(&mut self, lg: &mut dyn FnMut(&str)) {
        let n = self.pop(lg);
        self.fp = self.pop(lg) as usize;
        for _ in 0..n { self.pop(lg); }
    }

    fn finish_yield(&mut self, host: &mut dyn Host) {
        let mut lg = |s: &str| host.log(s);
        self.finish_frame(&mut lg);
    }

    fn finish_callstd(&mut self, result: Cell, host: &mut dyn Host) {
        if self.fp >= 1 { self.stack[self.fp - 1] = result; }
        let mut lg = |s: &str| host.log(s);
        self.finish_frame(&mut lg);
    }
}

fn binop(op: BinOp, b: Cell, bt: Var, a: Cell, at: Var, lg: &mut dyn FnMut(&str)) -> Cell {
    let both_int = bt.is_int() && at.is_int();
    let fl = |v: Cell, t: Var| if t.is_int() { v as i32 as f32 } else { f32::from_bits(v) };
    let int = |v: Cell, t: Var| if t.is_int() { v as i32 } else { f32::from_bits(v) as i32 };
    let truthy = |v: Cell, t: Var| if t.is_int() { v != 0 } else { f32::from_bits(v) != 0.0 };
    let (bi, ai) = (b as i32, a as i32);
    let (bf, af) = (fl(b, bt), fl(a, at));
    match op {
        BinOp::And => (int(b, bt) & int(a, at)) as Cell,
        BinOp::Or => (int(b, bt) | int(a, at)) as Cell,
        BinOp::LAnd => (truthy(b, bt) && truthy(a, at)) as Cell,
        BinOp::LOr => (truthy(b, bt) || truthy(a, at)) as Cell,
        BinOp::Add => if both_int { bi.wrapping_add(ai) as Cell } else { (bf + af).to_bits() },
        BinOp::Sub => if both_int { bi.wrapping_sub(ai) as Cell } else { (bf - af).to_bits() },
        BinOp::Mul => if both_int { bi.wrapping_mul(ai) as Cell } else { (bf * af).to_bits() },
        BinOp::Div => if both_int {
            if ai == 0 { lg("div by zero"); 0 } else { bi.wrapping_div(ai) as Cell }
        } else { (bf / af).to_bits() },
        BinOp::Mod => if both_int {
            if ai == 0 { lg("mod by zero"); 0 } else { bi.wrapping_rem(ai) as Cell }
        } else { (bf % af).to_bits() },
        BinOp::Lt => (if both_int { bi < ai } else { bf < af }) as Cell,
        BinOp::Le => (if both_int { bi <= ai } else { bf <= af }) as Cell,
        BinOp::Gt => (if both_int { bi > ai } else { bf > af }) as Cell,
        BinOp::Ge => (if both_int { bi >= ai } else { bf >= af }) as Cell,
        BinOp::Eq => (if both_int { bi == ai } else { bf == af }) as Cell,
        BinOp::Ne => (if both_int { bi != ai } else { bf != af }) as Cell,
    }
}

struct ThreadMem<'a> {
    thread: &'a mut Thread,
    data: &'a mut Vec<u8>,
}

impl Memory for ThreadMem<'_> {
    fn read(&self, addr: Cell) -> Option<Cell> {
        let off = (addr & ADDR_MASK) as usize;
        if addr & ADDR_LOCAL != 0 { self.thread.stack.get(off).copied() }
        else if addr & ADDR_DATA != 0 { self.data.get(off..off + 4).map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]])) }
        else { None }
    }
    fn write(&mut self, addr: Cell, value: Cell) -> bool {
        let off = (addr & ADDR_MASK) as usize;
        if addr & ADDR_LOCAL != 0 {
            match self.thread.stack.get_mut(off) { Some(c) => { *c = value; true } None => false }
        } else if addr & ADDR_DATA != 0 {
            match self.data.get_mut(off..off + 4) { Some(b) => { b.copy_from_slice(&value.to_be_bytes()); true } None => false }
        } else { false }
    }
}

/// A host that implements nothing: logs every native call with its name and typed arguments,
/// returns 0 (or a scripted value), and stops recording after `limit` calls.
pub struct TraceHost {
    pub calls: Vec<String>,
    pub logs: Vec<String>,
    pub limit: usize,
    /// Optional fixed return values by native index.
    pub returns: std::collections::HashMap<u16, Cell>,
}

impl TraceHost {
    pub fn new(limit: usize) -> Self {
        TraceHost { calls: vec![], logs: vec![], limit, returns: Default::default() }
    }
    pub fn full(&self) -> bool { self.calls.len() >= self.limit }
}

impl Host for TraceHost {
    fn call_native(&mut self, call: &NativeCall, _mem: &mut dyn Memory) -> NativeResult {
        if self.calls.len() < self.limit {
            let types = call.info.map(|n| n.args).unwrap_or(&[]);
            let args: Vec<String> = call.args.iter().enumerate().map(|(i, &v)| match types.get(i) {
                Some(ArgType::Float) => format!("{}", f32::from_bits(v)),
                _ if v >= 0x1000 && v != 0xFFFF_FFFF => format!("{v:#x}"),
                _ => format!("{}", v as i32),
            }).collect();
            self.calls.push(format!("[t{} {:#010x}] std[{}] {}({})", call.thread_serial, call.thread_id, call.index, call.name(), args.join(", ")));
        }
        NativeResult::Done(self.returns.get(&call.index).copied().unwrap_or(0))
    }
    fn log(&mut self, line: &str) { self.logs.push(line.to_string()); }
}
