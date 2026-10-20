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
    /// Guest CPU cycles executed (each lifted block adds its instructions' cycles on entry, Dolphin's per-opcode
    /// estimates: tools/ppclift/cycles.py): the emulated clock (run.rs time base).
    pub cycles: u64,
    pub step_limit: u64,
    pub last_fn: u32,
    /// Record function facts (`prof.rs`); lifted functions check this at entry.
    pub prof: bool,
    /// Lifted functions consult `stub_hit`/`prof` only when this is set.
    pub hooks: bool,
    /// Runtime stubs: function address -> value for r3 (or u32::MAX: leave r3), checked at every lifted entry.
    pub stubs: AddrMap<u32>,
    /// Runtime stubs that also zero `n` bytes at r3 first (output buffers, e.g. pad state).
    pub stub_zero: AddrMap<u32>,
    /// Runtime register patches at entry that then run the original function: addr -> [(reg, value)].
    pub entry_set: AddrMap<Vec<(usize, u32)>>,
    /// Runtime stubs that call the function pointer in register N with the entry r3 (completion callbacks,
    /// e.g. an ARAM transfer that "finishes" at once).
    pub stub_cb: AddrMap<usize>,
    /// Host callbacks that replace a function (checked before `stubs`): colo-viewer parks the battle thread at
    /// `_threadSwitch` and feeds the pad at `menuGetKeyInfo`.
    pub hook_fn: AddrMap<fn(&mut Cpu)>,
    /// See `mmio`: live hardware register reads (None = plain memory).
    pub mmio_read: Option<fn(&Cpu, u32, u32) -> Option<u32>>,
    /// Debug: log entry (r3-r5, lr) of these functions (needs `hooks`).
    pub trace_fns: std::collections::HashSet<u32>,
    /// Called at a block boundary once `steps` passes `step_limit` (it sets the next limit): run.rs takes
    /// interrupts there, as the hardware does between instructions. Without it, the step limit is an error.
    pub tick: Option<fn(&mut Cpu)>,
}

/// Function-address keyed maps (hooks, stubs): `stub_hit` checks them at every lifted call (millions a frame), so
/// each keeps a 4096-bit filter of its keys (bit (addr >> 2) & 0xFFF) and only hashes (one multiply, not SipHash)
/// when the address's bit is set. Writes go through `insert` / `entry` / `remove`, which keep the filter a superset
/// of the keys; reads may use the map directly (Deref).
pub struct AddrMap<V> {
    map: std::collections::HashMap<u32, V, std::hash::BuildHasherDefault<AddrHasher>>,
    bits: Box<[u64; 64]>,
}

impl<V> Default for AddrMap<V> {
    fn default() -> Self { AddrMap { map: Default::default(), bits: Box::new([0; 64]) } }
}

impl<V> std::ops::Deref for AddrMap<V> {
    type Target = std::collections::HashMap<u32, V, std::hash::BuildHasherDefault<AddrHasher>>;
    fn deref(&self) -> &Self::Target { &self.map }
}

impl<V> AddrMap<V> {
    #[inline(always)]
    fn bit(a: u32) -> (usize, u64) { let i = (a >> 2) & 0xFFF; ((i >> 6) as usize, 1 << (i & 63)) }
    /// Whether `a` may be a key (false: it isn't).
    #[inline(always)]
    pub fn may(&self, a: u32) -> bool { let (w, b) = Self::bit(a); self.bits[w] & b != 0 }
    /// The value at `a`; the filter answers most misses without hashing.
    #[inline(always)]
    pub fn get(&self, a: &u32) -> Option<&V> {
        let (w, b) = Self::bit(*a);
        if self.bits[w] & b == 0 { return None; }
        self.map.get(a)
    }
    pub fn insert(&mut self, a: u32, v: V) -> Option<V> {
        let (w, b) = Self::bit(a);
        self.bits[w] |= b;
        self.map.insert(a, v)
    }
    pub fn entry(&mut self, a: u32) -> std::collections::hash_map::Entry<'_, u32, V> {
        let (w, b) = Self::bit(a);
        self.bits[w] |= b;
        self.map.entry(a)
    }
    pub fn remove(&mut self, a: &u32) -> Option<V> {
        let v = self.map.remove(a);
        self.bits.fill(0);
        for &k in self.map.keys() { let (w, b) = Self::bit(k); self.bits[w] |= b; }
        v
    }
}

#[derive(Default, Clone, Copy)]
pub struct AddrHasher(u64);
impl std::hash::Hasher for AddrHasher {
    fn finish(&self) -> u64 { self.0 }
    fn write(&mut self, b: &[u8]) { for &x in b { self.0 = (self.0.rotate_left(8) ^ x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15); } }
    fn write_u32(&mut self, n: u32) { let h = (n as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15); self.0 = h ^ (h >> 32); }
}

fn no_dispatch(_: &mut Cpu, a: u32) { panic!("ppc: no dispatcher for indirect call to {a:#010x}") }

/// HID2 (920), DMA_U (922), DMA_L (923): the locked-cache DMA registers (index spr - 920).
static LC_SPR: [std::sync::atomic::AtomicU32; 4] = [const { std::sync::atomic::AtomicU32::new(0) }; 4];

impl Cpu {
    pub fn new(mut m: Mem) -> Self {
        m.map(STACK_TOP - STACK_SIZE, vec![0; STACK_SIZE as usize]);
        let mut c = Cpu { r: [0; 32], f: [0.0; 32], ps1: [0.0; 32], cr: 0, lr: 0, ctr: 0, xer: 0, fpscr: 0, gqr: [0; 8], m,
            dispatch: no_dispatch, steps: 0, cycles: 0, step_limit: u64::MAX, last_fn: 0, prof: false, hooks: false, stubs: Default::default(), stub_zero: Default::default(), entry_set: Default::default(), stub_cb: Default::default(), hook_fn: Default::default(), mmio_read: None, trace_fns: Default::default(), tick: None };
        c.r[1] = STACK_TOP - 0x100;
        c.r[2] = 0x804836A0; // _SDA2_BASE_
        c.r[13] = 0x80480820; // _SDA_BASE_
        c
    }

    /// A CPU over an existing address space (e.g. a RAM snapshot that already has a stack); no stack is mapped.
    pub fn bare(m: Mem) -> Self {
        Cpu { r: [0; 32], f: [0.0; 32], ps1: [0.0; 32], cr: 0, lr: 0, ctr: 0, xer: 0, fpscr: 0, gqr: [0; 8], m,
            dispatch: no_dispatch, steps: 0, cycles: 0, step_limit: u64::MAX, last_fn: 0, prof: false, hooks: false, stubs: Default::default(), stub_zero: Default::default(), entry_set: Default::default(), stub_cb: Default::default(), hook_fn: Default::default(), mmio_read: None, trace_fns: Default::default(), tick: None }
    }

    /// `steps` passed `step_limit` at block `pc` of function `f`.
    #[cold]
    pub fn step_event(&mut self, f: u32, pc: u32) {
        match self.tick {
            Some(t) => t(self),
            None => panic!("step limit in {f:#010x} at pc {pc:#010x}"),
        }
    }

    /// Runtime stub check (see `stubs`): returns true when the call is replaced. Inline: the maps' key filters
    /// answer most calls (no hook at `f`) without leaving the lifted function.
    #[inline(always)]
    pub fn stub_hit(&mut self, f: u32) -> bool {
        if self.trace_fns.is_empty() && !self.hook_fn.may(f) && !self.entry_set.may(f) && !self.stubs.may(f) { return false; }
        self.stub_hit_slow(f)
    }
    #[inline(never)]
    fn stub_hit_slow(&mut self, f: u32) -> bool {
        if !self.trace_fns.is_empty() && self.trace_fns.contains(&f) {
            eprintln!("call {f:#010x} r3 {:#x} r4 {:#x} r5 {:#x} lr {:#x}", self.r[3], self.r[4], self.r[5], self.lr);
        }
        if let Some(&h) = self.hook_fn.get(&f) { h(self); return true; }
        if !self.entry_set.is_empty() { if let Some(v) = self.entry_set.get(&f) { for &(r, x) in v { self.r[r] = x; } } }
        if self.stubs.is_empty() { return false; }
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
    #[inline(always)] fn mmio(&self, a: u32, n: u32) -> Option<u32> {
        if (0xCC00_0000..0xCC00_8000).contains(&a) { self.mmio_cold(a, n) } else { None }
    }
    #[inline(never)] #[cold] fn mmio_cold(&self, a: u32, n: u32) -> Option<u32> { self.mmio_read.and_then(|h| h(self, a, n)) }
    /// A hardware register read from a lifted body that keeps its instruction count in a local (`ins`): the handler
    /// sees the current count (the time base), then the field goes back to the body's last sync.
    #[inline(never)] #[cold] fn mmio_at(&mut self, ins: u64, a: u32, n: u32) -> Option<u32> {
        let h = self.mmio_read?;
        let old = std::mem::replace(&mut self.cycles, ins);
        let v = h(self, a, n);
        self.cycles = old;
        v
    }
    #[inline(always)] fn io(a: u32) -> bool { a.wrapping_sub(0xCC00_0000) < 0x8000 }
    #[inline(always)] pub fn lbz_t(&mut self, ins: u64, a: u32) -> u32 { if Self::io(a) { if let Some(v) = self.mmio_at(ins, a, 1) { return v & 0xFF; } } self.m.u8(a) as u32 }
    #[inline(always)] pub fn lhz_t(&mut self, ins: u64, a: u32) -> u32 { if Self::io(a) { if let Some(v) = self.mmio_at(ins, a, 2) { return v & 0xFFFF; } } self.m.u16(a) as u32 }
    #[inline(always)] pub fn lha_t(&mut self, ins: u64, a: u32) -> u32 { if Self::io(a) { if let Some(v) = self.mmio_at(ins, a, 2) { return v as u16 as i16 as i32 as u32; } } self.m.u16(a) as i16 as i32 as u32 }
    #[inline(always)] pub fn lwz_t(&mut self, ins: u64, a: u32) -> u32 { if Self::io(a) { if let Some(v) = self.mmio_at(ins, a, 4) { return v; } } self.m.u32(a) }
    #[inline(always)] pub fn lbz(&self, a: u32) -> u32 { if let Some(v) = self.mmio(a, 1) { return v & 0xFF; } self.m.u8(a) as u32 }
    #[inline(always)] pub fn lhz(&self, a: u32) -> u32 { if let Some(v) = self.mmio(a, 2) { return v & 0xFFFF; } self.m.u16(a) as u32 }
    #[inline(always)] pub fn lha(&self, a: u32) -> u32 { if let Some(v) = self.mmio(a, 2) { return v as u16 as i16 as i32 as u32; } self.m.u16(a) as i16 as i32 as u32 }
    #[inline(always)] pub fn lwz(&self, a: u32) -> u32 { if let Some(v) = self.mmio(a, 4) { return v; } self.m.u32(a) }
    #[inline(always)] pub fn stb(&mut self, a: u32, v: u32) { self.m.set_u8(a, v) }
    #[inline(always)] pub fn sth(&mut self, a: u32, v: u32) { self.m.set_u16(a, v) }
    #[inline(always)] pub fn stw(&mut self, a: u32, v: u32) { self.m.set_u32(a, v) }
    #[inline(always)] pub fn lfs(&self, a: u32) -> f64 { f32::from_bits(self.m.u32(a)) as f64 }
    #[inline(always)] pub fn lfd(&self, a: u32) -> f64 {
        let hi = self.m.u32(a) as u64;
        let lo = self.m.u32(a + 4) as u64;
        f64::from_bits(hi << 32 | lo)
    }
    #[inline(always)] pub fn stfs(&mut self, a: u32, v: f64) { self.m.set_u32(a, (v as f32).to_bits()) }
    #[inline(always)] pub fn stfd(&mut self, a: u32, v: f64) {
        let b = v.to_bits();
        self.m.set_u32(a, (b >> 32) as u32);
        self.m.set_u32(a + 4, b as u32);
    }

    // --- condition register ---------------------------------------------------------------------
    #[inline] pub fn set_crf(&mut self, n: u32, nib: u32) { cr_set_crf(&mut self.cr, n, nib) }
    #[inline] pub fn crf(&self, n: u32) -> u32 { cr_crf(self.cr, n) }
    #[inline] pub fn crbit(&self, b: u32) -> bool { cr_bit(self.cr, b) }
    #[inline] pub fn set_crbit(&mut self, b: u32, v: bool) { cr_set_bit(&mut self.cr, b, v) }
    #[inline] pub fn cmps(&mut self, n: u32, a: u32, b: u32) { cr_cmps(&mut self.cr, self.xer, n, a, b) }
    #[inline] pub fn cmpu(&mut self, n: u32, a: u32, b: u32) { cr_cmpu(&mut self.cr, self.xer, n, a, b) }
    /// Record form: cr0 from a result.
    #[inline] pub fn rc(&mut self, v: u32) { cr_rc(&mut self.cr, self.xer, v) }
    #[inline] pub fn fcmp(&mut self, n: u32, a: f64, b: f64) { cr_fcmp(&mut self.cr, &mut self.fpscr, n, a, b) }

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
            920 | 922 | 923 => LC_SPR[(spr - 920) as usize].load(std::sync::atomic::Ordering::Relaxed),
            _ => 0,
        }
    }
    pub fn mtspr(&mut self, spr: u32, v: u32) {
        match spr {
            1 => self.xer = v,
            8 => self.lr = v,
            9 => self.ctr = v,
            912..=919 => self.gqr[(spr - 912) as usize] = v,
            920 | 922 => LC_SPR[(spr - 920) as usize].store(v, std::sync::atomic::Ordering::Relaxed),
            923 => {
                // DMA_L with T set: move (DMA_U len hi 5 bits, DMA_L len lo 2 bits) x 32 bytes (0 = 128) between
                // the locked L1 cache (DMA_L address, 0xE000_0000) and memory (DMA_U address); LD = load to the
                // cache. It completes at once, so T reads back clear (LCQueueWait, THP's JPEG decoder).
                if v & 2 != 0 {
                    let u = LC_SPR[2].load(std::sync::atomic::Ordering::Relaxed);
                    let n = match ((u & 0x1F) << 2) | ((v >> 2) & 3) { 0 => 128, n => n } * 32;
                    let (mem, lc) = ((u & !0x1F) | 0x8000_0000, v & !0x1F);
                    if v & 0x10 != 0 { let d = self.m.read(mem, n).to_vec(); self.m.write(lc, &d); }
                    else { let d = self.m.read(lc, n).to_vec(); self.m.write(mem, &d); }
                }
                LC_SPR[3].store(v & !2, std::sync::atomic::Ordering::Relaxed);
            }
            _ => {}
        }
    }

    // --- paired-single quantized load/store (GQR scaling) ---------------------------------------
    /// 2^-s for the 6-bit signed GQR scale (built from the exponent: exact, and no `powi` call per element).
    #[inline(always)] fn q_scale(s: u32) -> f64 { let s = (s & 0x3F) as i32; let s = if s >= 32 { s - 64 } else { s }; f64::from_bits(((1023 - s) as u64) << 52) }
    #[inline(always)] fn psq_load_one(&self, a: u32, ty: u32, sc: u32) -> (f64, u32) {
        match ty {
            4 => (self.m.u8(a) as f64 * Self::q_scale(sc), 1),
            5 => (self.m.u16(a) as f64 * Self::q_scale(sc), 2),
            6 => (self.m.i8(a) as f64 * Self::q_scale(sc), 1),
            7 => (self.m.i16(a) as f64 * Self::q_scale(sc), 2),
            _ => (f32::from_bits(self.m.u32(a)) as f64, 4),
        }
    }
    #[inline(always)] pub fn psq_load(&self, a: u32, w: bool, gqr: u32) -> (f64, f64) {
        let g = self.gqr[gqr as usize];
        let (ty, sc) = ((g >> 16) & 7, (g >> 24) & 0x3F);
        let (x, n) = self.psq_load_one(a, ty, sc);
        let y = if w { 1.0 } else { self.psq_load_one(a + n, ty, sc).0 };
        (x, y)
    }
    #[inline(always)] fn psq_store_one(&mut self, a: u32, v: f64, ty: u32, sc: u32) -> u32 {
        let s = v / Self::q_scale(sc);
        match ty {
            4 => { self.m.set_u8(a, s.clamp(0.0, 255.0) as u32); 1 }
            5 => { self.m.set_u16(a, s.clamp(0.0, 65535.0) as u32); 2 }
            6 => { self.m.set_u8(a, s.clamp(-128.0, 127.0) as i32 as u32); 1 }
            7 => { self.m.set_u16(a, s.clamp(-32768.0, 32767.0) as i32 as u32); 2 }
            _ => { self.m.set_u32(a, (v as f32).to_bits()); 4 }
        }
    }
    #[inline(always)] pub fn psq_store(&mut self, a: u32, x: f64, y: f64, w: bool, gqr: u32) {
        let g = self.gqr[gqr as usize];
        let (ty, sc) = (g & 7, (g >> 8) & 0x3F);
        let n = self.psq_store_one(a, x, ty, sc);
        if !w { self.psq_store_one(a + n, y, ty, sc); }
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

// --- condition register on a value: lifted bodies keep CR in a local (tools/ppclift/counters.py) ----------------
#[inline(always)] pub fn cr_set_crf(cr: &mut u32, n: u32, nib: u32) {
    let sh = 28 - 4 * n;
    *cr = (*cr & !(0xF << sh)) | ((nib & 0xF) << sh);
}
#[inline(always)] pub fn cr_crf(cr: u32, n: u32) -> u32 { (cr >> (28 - 4 * n)) & 0xF }
#[inline(always)] pub fn cr_bit(cr: u32, b: u32) -> bool { cr & (0x8000_0000 >> b) != 0 }
#[inline(always)] pub fn cr_set_bit(cr: &mut u32, b: u32, v: bool) {
    let m = 0x8000_0000 >> b;
    if v { *cr |= m } else { *cr &= !m }
}
#[inline(always)] fn so(xer: u32) -> u32 { if xer & XER_SO != 0 { 1 } else { 0 } }
#[inline(always)] pub fn cr_cmps(cr: &mut u32, xer: u32, n: u32, a: u32, b: u32) {
    let (a, b) = (a as i32, b as i32);
    let v = if a < b { 8 } else if a > b { 4 } else { 2 };
    cr_set_crf(cr, n, v | so(xer));
}
#[inline(always)] pub fn cr_cmpu(cr: &mut u32, xer: u32, n: u32, a: u32, b: u32) {
    let v = if a < b { 8 } else if a > b { 4 } else { 2 };
    cr_set_crf(cr, n, v | so(xer));
}
/// Record form: cr0 from a result.
#[inline(always)] pub fn cr_rc(cr: &mut u32, xer: u32, v: u32) { cr_cmps(cr, xer, 0, v, 0) }
#[inline(always)] pub fn cr_fcmp(cr: &mut u32, fpscr: &mut u32, n: u32, a: f64, b: f64) {
    let v = if a.is_nan() || b.is_nan() { 1 } else if a < b { 8 } else if a > b { 4 } else { 2 };
    cr_set_crf(cr, n, v);
    *fpscr = (*fpscr & !0xF000) | (v << 12);
}
