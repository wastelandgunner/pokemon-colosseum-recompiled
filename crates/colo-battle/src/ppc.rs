//! Runtime for code lifted from main.dol (`Working/tools/ppclift`): the Gekko register file and the
//! helpers the generated Rust calls. Lifted functions take `&mut Cpu` and follow the PowerPC EABI
//! exactly (arguments in r3.., f1..; result in r3/f1; r1 stack in `Mem`), so they can call each other
//! and hand-written ports interchangeably.

use crate::mem::Mem;

pub const XER_SO: u32 = 0x8000_0000;
pub const XER_OV: u32 = 0x4000_0000;
pub const XER_CA: u32 = 0x2000_0000;

/// Stack the lifted code runs on (r1 starts at the top).
pub const STACK_TOP: u32 = 0x8180_0000;
pub const STACK_SIZE: u32 = 0x0010_0000;

pub struct Cpu {
    pub r: [u32; 32],
    /// ps0 of each FPR (the scalar FPU value)
    pub f: [f64; 32],
    /// ps1 of each FPR (paired-single slot 1)
    pub ps1: [f64; 32],
    pub cr: u32,
    pub lr: u32,
    pub ctr: u32,
    pub xer: u32,
    pub fpscr: u32,
    pub gqr: [u32; 8],
    pub m: Mem,
    /// Indirect-call dispatcher (bctrl / bctr to another function), set by the lifted module.
    pub dispatch: fn(&mut Cpu, u32),
    /// Basic blocks executed, a limit that panics (runaway loops waiting on hardware), and the last function entered.
    pub steps: u64,
    pub step_limit: u64,
    pub last_fn: u32,
    /// Record function facts (`prof.rs`); lifted functions check this at entry.
    pub prof: bool,
    /// Lifted functions consult `stub_hit`/`prof` only when this is set.
    pub hooks: bool,
    /// Runtime stubs: function address -> value for r3 (or u32::MAX: leave r3), checked at every lifted entry.
    pub stubs: std::collections::HashMap<u32, u32>,
    /// Runtime stubs that also zero `n` bytes at r3 first (output buffers, e.g. pad state).
    pub stub_zero: std::collections::HashMap<u32, u32>,
    /// Runtime register patches at entry that then run the original function: addr -> [(reg, value)].
    pub entry_set: std::collections::HashMap<u32, Vec<(usize, u32)>>,
    /// Runtime stubs that call the function pointer in register N with the entry r3 (completion callbacks,
    /// e.g. an ARAM transfer that "finishes" at once).
    pub stub_cb: std::collections::HashMap<u32, usize>,
    /// Host callbacks that replace a function (checked before `stubs`): colo-viewer parks the battle thread at
    /// `_threadSwitch` and feeds the pad at `menuGetKeyInfo`.
    pub hook_fn: std::collections::HashMap<u32, fn(&mut Cpu)>,
    /// See `mmio`: live hardware register reads (None = plain memory).
    pub mmio_read: Option<fn(&Cpu, u32, u32) -> Option<u32>>,
    /// Debug: log entry (r3-r5, lr) of these functions (needs `hooks`).
    pub trace_fns: std::collections::HashSet<u32>,
    /// Called at a block boundary once `steps` passes `step_limit` (it sets the next limit): run.rs takes
    /// interrupts there, as the hardware does between instructions. Without it, the step limit is an error.
    pub tick: Option<fn(&mut Cpu)>,
}

fn no_dispatch(_: &mut Cpu, a: u32) { panic!("ppc: no dispatcher for indirect call to {a:#010x}") }

impl Cpu {
    pub fn new(mut m: Mem) -> Self {
        m.map(STACK_TOP - STACK_SIZE, vec![0; STACK_SIZE as usize]);
        let mut c = Cpu { r: [0; 32], f: [0.0; 32], ps1: [0.0; 32], cr: 0, lr: 0, ctr: 0, xer: 0, fpscr: 0, gqr: [0; 8], m,
            dispatch: no_dispatch, steps: 0, step_limit: u64::MAX, last_fn: 0, prof: false, hooks: false, stubs: Default::default(), stub_zero: Default::default(), entry_set: Default::default(), stub_cb: Default::default(), hook_fn: Default::default(), mmio_read: None, trace_fns: Default::default(), tick: None };
        c.r[1] = STACK_TOP - 0x100;
        c.r[2] = 0x804836A0; // _SDA2_BASE_
        c.r[13] = 0x80480820; // _SDA_BASE_
        c
    }

    /// A CPU over an existing address space (e.g. a RAM snapshot that already has a stack); no stack is mapped.
    pub fn bare(m: Mem) -> Self {
        Cpu { r: [0; 32], f: [0.0; 32], ps1: [0.0; 32], cr: 0, lr: 0, ctr: 0, xer: 0, fpscr: 0, gqr: [0; 8], m,
            dispatch: no_dispatch, steps: 0, step_limit: u64::MAX, last_fn: 0, prof: false, hooks: false, stubs: Default::default(), stub_zero: Default::default(), entry_set: Default::default(), stub_cb: Default::default(), hook_fn: Default::default(), mmio_read: None, trace_fns: Default::default(), tick: None }
    }

    /// `steps` passed `step_limit` at block `pc` of function `f`.
    #[cold]
    pub fn step_event(&mut self, f: u32, pc: u32) {
        match self.tick {
            Some(t) => t(self),
            None => panic!("step limit in {f:#010x} at pc {pc:#010x}"),
        }
    }

    /// Runtime stub check (see `stubs`): returns true when the call is replaced.
    #[inline]
    pub fn stub_hit(&mut self, f: u32) -> bool {
        if !self.trace_fns.is_empty() && self.trace_fns.contains(&f) {
            eprintln!("call {f:#010x} r3 {:#x} r4 {:#x} r5 {:#x} lr {:#x}", self.r[3], self.r[4], self.r[5], self.lr);
        }
        if let Some(&h) = self.hook_fn.get(&f) { h(self); return true; }
        if let Some(v) = self.entry_set.get(&f) { for &(r, x) in v { self.r[r] = x; } }
        match self.stubs.get(&f) {
            Some(&v) => {
                if let Some(&n) = self.stub_zero.get(&f) { let a = self.r[3]; for k in 0..n { self.m.set_u8(a + k, 0); } }
                if let Some(&n) = self.stub_cb.get(&f) {
                    let (cb, lr) = (self.r[n], self.lr);
                    if cb != 0 { (self.dispatch)(self, cb); }
                    self.lr = lr;
                }
                if v != u32::MAX { self.r[3] = v; }
                true
            }
            None => false,
        }
    }

    /// Load runtime stubs from a text file: `0xADDR [ret=V] [zero=N] [set=rN:V] [cb=rN] # comment` lines.
    pub fn load_stubs(&mut self, text: &str) {
        for l in text.lines() {
            let l = l.split('#').next().unwrap().trim();
            let mut p = l.split_whitespace();
            let Some(a) = p.next() else { continue };
            let Ok(a) = u32::from_str_radix(a.trim_start_matches("0x"), 16) else { continue };
            let mut ret = u32::MAX;
            let mut stub = true;
            for o in p {
                if let Some(v) = o.strip_prefix("set=r") {
                    // set=rN:V -> patch the register, then run the original function
                    let (r, x) = v.split_once(':').unwrap_or(("3", "0"));
                    self.entry_set.entry(a).or_default().push((r.parse().unwrap_or(3), parse_num(x)));
                    stub = false;
                }
                if let Some(v) = o.strip_prefix("ret=") { ret = parse_num(v); }
                if let Some(v) = o.strip_prefix("zero=") { self.stub_zero.insert(a, parse_num(v)); }
                if let Some(v) = o.strip_prefix("cb=r") { self.stub_cb.insert(a, v.parse().unwrap_or(3)); }
            }
            if stub { self.stubs.insert(a, ret); }
        }
        if !self.stubs.is_empty() || !self.entry_set.is_empty() { self.hooks = true; }
    }

    // --- memory (big-endian) ----------------------------------------------------------------------
    /// Hardware register reads with side effects or live values (colo-viewer's run.rs): (cpu, address, bytes)
    /// -> value, or None for plain memory.
    #[inline] fn mmio(&self, a: u32, n: u32) -> Option<u32> {
        if (0xCC00_0000..0xCC00_8000).contains(&a) { self.mmio_read.and_then(|h| h(self, a, n)) } else { None }
    }
    #[inline] pub fn lbz(&self, a: u32) -> u32 { if let Some(v) = self.mmio(a, 1) { return v & 0xFF; } self.m.u8(a) as u32 }
    #[inline] pub fn lhz(&self, a: u32) -> u32 { if let Some(v) = self.mmio(a, 2) { return v & 0xFFFF; } self.m.u16(a) as u32 }
    #[inline] pub fn lha(&self, a: u32) -> u32 { if let Some(v) = self.mmio(a, 2) { return v as u16 as i16 as i32 as u32; } self.m.u16(a) as i16 as i32 as u32 }
    #[inline] pub fn lwz(&self, a: u32) -> u32 { if let Some(v) = self.mmio(a, 4) { return v; } self.m.u32(a) }
    #[inline] pub fn stb(&mut self, a: u32, v: u32) { self.m.set_u8(a, v) }
    #[inline] pub fn sth(&mut self, a: u32, v: u32) { self.m.set_u16(a, v) }
    #[inline] pub fn stw(&mut self, a: u32, v: u32) { self.m.set_u32(a, v) }
    #[inline] pub fn lfs(&self, a: u32) -> f64 { f32::from_bits(self.m.u32(a)) as f64 }
    #[inline] pub fn lfd(&self, a: u32) -> f64 {
        let hi = self.m.u32(a) as u64;
        let lo = self.m.u32(a + 4) as u64;
        f64::from_bits(hi << 32 | lo)
    }
    #[inline] pub fn stfs(&mut self, a: u32, v: f64) { self.m.set_u32(a, (v as f32).to_bits()) }
    #[inline] pub fn stfd(&mut self, a: u32, v: f64) {
        let b = v.to_bits();
        self.m.set_u32(a, (b >> 32) as u32);
        self.m.set_u32(a + 4, b as u32);
    }

    // --- condition register ---------------------------------------------------------------------
    #[inline] pub fn set_crf(&mut self, n: u32, nib: u32) {
        let sh = 28 - 4 * n;
        self.cr = (self.cr & !(0xF << sh)) | ((nib & 0xF) << sh);
    }
    #[inline] pub fn crf(&self, n: u32) -> u32 { (self.cr >> (28 - 4 * n)) & 0xF }
    #[inline] pub fn crbit(&self, b: u32) -> bool { self.cr & (0x8000_0000 >> b) != 0 }
    #[inline] pub fn set_crbit(&mut self, b: u32, v: bool) {
        let m = 0x8000_0000 >> b;
        if v { self.cr |= m } else { self.cr &= !m }
    }
    #[inline] fn so(&self) -> u32 { if self.xer & XER_SO != 0 { 1 } else { 0 } }
    #[inline] pub fn cmps(&mut self, n: u32, a: u32, b: u32) {
        let (a, b) = (a as i32, b as i32);
        let v = if a < b { 8 } else if a > b { 4 } else { 2 };
        self.set_crf(n, v | self.so());
    }
    #[inline] pub fn cmpu(&mut self, n: u32, a: u32, b: u32) {
        let v = if a < b { 8 } else if a > b { 4 } else { 2 };
        self.set_crf(n, v | self.so());
    }
    /// Record form: cr0 from a result.
    #[inline] pub fn rc(&mut self, v: u32) { self.cmps(0, v, 0) }
    #[inline] pub fn fcmp(&mut self, n: u32, a: f64, b: f64) {
        let v = if a.is_nan() || b.is_nan() { 1 } else if a < b { 8 } else if a > b { 4 } else { 2 };
        self.set_crf(n, v);
        self.fpscr = (self.fpscr & !0xF000) | (v << 12);
    }

    // --- carry ----------------------------------------------------------------------------------
    #[inline] pub fn ca(&self) -> u32 { if self.xer & XER_CA != 0 { 1 } else { 0 } }
    #[inline] pub fn set_ca(&mut self, v: bool) { if v { self.xer |= XER_CA } else { self.xer &= !XER_CA } }
    /// a + b + cin, setting CA.
    #[inline] pub fn addc(&mut self, a: u32, b: u32, cin: u32) -> u32 {
        let s = a as u64 + b as u64 + cin as u64;
        self.set_ca(s >> 32 != 0);
        s as u32
    }

    // --- special registers ----------------------------------------------------------------------
    pub fn mfspr(&self, spr: u32) -> u32 {
        match spr {
            1 => self.xer,
            8 => self.lr,
            9 => self.ctr,
            912..=919 => self.gqr[(spr - 912) as usize],
            _ => 0,
        }
    }
    pub fn mtspr(&mut self, spr: u32, v: u32) {
        match spr {
            1 => self.xer = v,
            8 => self.lr = v,
            9 => self.ctr = v,
            912..=919 => self.gqr[(spr - 912) as usize] = v,
            _ => {}
        }
    }

    // --- paired-single quantized load/store (GQR scaling) ---------------------------------------
    fn q_scale(s: u32) -> f64 { let s = (s & 0x3F) as i32; let s = if s >= 32 { s - 64 } else { s }; 2f64.powi(-s) }
    pub fn psq_load(&self, a: u32, w: bool, gqr: u32) -> (f64, f64) {
        let g = self.gqr[gqr as usize];
        let (ty, sc) = ((g >> 16) & 7, (g >> 24) & 0x3F);
        let one = |a: u32| -> (f64, u32) {
            match ty {
                4 => (self.m.u8(a) as f64 * Self::q_scale(sc), 1),
                5 => (self.m.u16(a) as f64 * Self::q_scale(sc), 2),
                6 => (self.m.i8(a) as f64 * Self::q_scale(sc), 1),
                7 => (self.m.i16(a) as f64 * Self::q_scale(sc), 2),
                _ => (f32::from_bits(self.m.u32(a)) as f64, 4),
            }
        };
        let (x, n) = one(a);
        let y = if w { 1.0 } else { one(a + n).0 };
        (x, y)
    }
    pub fn psq_store(&mut self, a: u32, x: f64, y: f64, w: bool, gqr: u32) {
        let g = self.gqr[gqr as usize];
        let (ty, sc) = (g & 7, (g >> 8) & 0x3F);
        let mut one = |a: u32, v: f64| -> u32 {
            let s = v / Self::q_scale(sc);
            match ty {
                4 => { self.m.set_u8(a, s.clamp(0.0, 255.0) as u32); 1 }
                5 => { self.m.set_u16(a, s.clamp(0.0, 65535.0) as u32); 2 }
                6 => { self.m.set_u8(a, s.clamp(-128.0, 127.0) as i32 as u32); 1 }
                7 => { self.m.set_u16(a, s.clamp(-32768.0, 32767.0) as i32 as u32); 2 }
                _ => { self.m.set_u32(a, (v as f32).to_bits()); 4 }
            }
        };
        let n = one(a, x);
        if !w { one(a + n, y); }
    }
}

fn parse_num(v: &str) -> u32 {
    if let Some(h) = v.strip_prefix("0x") { u32::from_str_radix(h, 16).unwrap_or(0) } else { v.parse::<i64>().map(|x| x as u32).unwrap_or(0) }
}

// --- integer helpers ----------------------------------------------------------------------------

/// rlwinm/rlwnm mask from MB..ME (inclusive, MSB = bit 0; wraps when MB > ME).
#[inline] pub fn mask(mb: u32, me: u32) -> u32 {
    let a = 0xFFFF_FFFFu32 >> mb;
    let b = 0xFFFF_FFFFu32 << (31 - me);
    if mb <= me { a & b } else { a | b }
}
#[inline] pub fn divw(a: u32, b: u32) -> u32 {
    let (a, b) = (a as i32, b as i32);
    if b == 0 || (a == i32::MIN && b == -1) { if a < 0 { 0xFFFF_FFFF } else { 0 } } else { (a / b) as u32 }
}
#[inline] pub fn divwu(a: u32, b: u32) -> u32 { if b == 0 { 0 } else { a / b } }
#[inline] pub fn mulhw(a: u32, b: u32) -> u32 { ((a as i32 as i64 * b as i32 as i64) >> 32) as u32 }
#[inline] pub fn mulhwu(a: u32, b: u32) -> u32 { ((a as u64 * b as u64) >> 32) as u32 }
#[inline] pub fn slw(a: u32, n: u32) -> u32 { if n & 0x20 != 0 { 0 } else { a << (n & 31) } }
#[inline] pub fn srw(a: u32, n: u32) -> u32 { if n & 0x20 != 0 { 0 } else { a >> (n & 31) } }

// --- float helpers ------------------------------------------------------------------------------

/// Round to single precision (frsp, and the result of every single-precision op).
#[inline] pub fn s(v: f64) -> f64 { v as f32 as f64 }
/// fctiwz: truncate toward zero, saturating; the integer is the low word of the result bits.
#[inline] pub fn fctiwz(v: f64) -> f64 {
    let i: i32 = if v.is_nan() { i32::MIN } else if v >= 2147483647.0 { i32::MAX } else if v <= -2147483648.0 { i32::MIN } else { v.trunc() as i32 };
    f64::from_bits(0xFFF8_0000_0000_0000 | i as u32 as u64)
}
/// fctiw: round to nearest even (FPSCR RN = 0), saturating.
#[inline] pub fn fctiw(v: f64) -> f64 {
    let r = v.round_ties_even();
    let i: i32 = if v.is_nan() { i32::MIN } else if r >= 2147483647.0 { i32::MAX } else if r <= -2147483648.0 { i32::MIN } else { r as i32 };
    f64::from_bits(0xFFF8_0000_0000_0000 | i as u32 as u64)
}
/// fres: Gekko's reciprocal estimate (single precision 1/x, good to the table's accuracy).
#[inline] pub fn fres(v: f64) -> f64 { s(1.0 / v) }
#[inline] pub fn frsqrte(v: f64) -> f64 { 1.0 / v.sqrt() }
