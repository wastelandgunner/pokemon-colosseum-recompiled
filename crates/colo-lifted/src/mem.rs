//! The game's address space, as far as the battle engine sees it.
//!
//! The battle code is pointer-heavy: Pokémon, battlers and tables are reached through u32 addresses and
//! fixed byte offsets, and one "status id" API (`pokemonGetStatus`) reads all of them. Keeping the original
//! layouts in a big-endian byte arena lets each function be ported line by line, and lets a state be
//! compared byte for byte with a Dolphin RAM dump.
//!
//! Regions: main.dol sections at their real addresses, common_rel where the game loads it
//! (`REL_BASE`, measured in Dolphin), and a heap for battle objects.

/// common_rel's load address in a running game (Dolphin, 2026-10-12): file offset + this = address.
pub const REL_BASE: u32 = 0x807628A0;
/// Heap for objects the battle allocates (above everything the DOL and REL use).
pub const HEAP_BASE: u32 = 0x81000000;

/// Debug write watchpoint (0 = off): every write covering it prints a backtrace.
pub static WATCH: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

thread_local! {
    /// Accesses since the last profiler event (reads take `&self`, so they are buffered here).
    pub static PEND: std::cell::RefCell<Vec<(u32, u8, bool)>> = const { std::cell::RefCell::new(Vec::new()) };
}

pub struct Region {
    pub start: u32,
    pub bytes: Vec<u8>,
}

#[derive(Default)]
pub struct Mem {
    pub regions: Vec<Region>,
    /// When set, every access is buffered in `PEND` as (address, length, is write) for `prof.rs`.
    pub trace: bool,
    /// When set, writes to the GX write-gather pipe (0xCC008000) are appended here instead (colo-viewer's run.rs:
    /// the frame's GX command stream).
    pub pipe: Option<Vec<u8>>,
    /// Called after every write into the hardware registers (0xCC000000..0xCC008000) with (address, length):
    /// run.rs models the devices behind them.
    pub mmio_write: Option<fn(&mut Mem, u32, usize)>,
    heap_top: u32,
    /// Per 1 MB page: the first region that intersects it (`NO_REGION`: none). Any address the region holds is
    /// found there (no earlier region can hold it); the others fall back to the scan.
    page: Vec<u8>,
}

const NO_REGION: u8 = u8::MAX;

impl Mem {
    pub fn new() -> Self {
        let mut m = Mem { regions: vec![Region { start: HEAP_BASE, bytes: vec![] }], heap_top: HEAP_BASE, ..Default::default() };
        m.index();
        m
    }

    /// Map bytes at `start` (e.g. a DOL section, the REL image).
    pub fn map(&mut self, start: u32, bytes: Vec<u8>) { self.regions.push(Region { start, bytes }); self.index(); }

    fn index(&mut self) {
        self.page = vec![NO_REGION; 4096];
        for (i, r) in self.regions.iter().enumerate().take(NO_REGION as usize).rev() {
            if r.bytes.is_empty() { continue; }
            let end = r.start as u64 + r.bytes.len() as u64 - 1;
            for p in (r.start >> 20) as u64..=(end >> 20).min(4095) { self.page[p as usize] = i as u8; }
        }
    }

    /// Allocate `size` zeroed bytes on the heap (32-byte aligned, like the game's allocators).
    pub fn alloc(&mut self, size: u32) -> u32 {
        let a = (self.heap_top + 31) & !31;
        self.heap_top = a + size;
        let heap = &mut self.regions[0];
        heap.bytes.resize((self.heap_top - HEAP_BASE) as usize, 0);
        self.index();
        a
    }

    #[inline(always)]
    fn find(&self, a: u32, n: u32) -> (usize, usize) {
        // 0xC0000000.. is MEM1 uncached: the same bytes as 0x80000000..
        let a = if (0xC000_0000..0xC180_0000).contains(&a) { a - 0x4000_0000 } else { a };
        let i = self.page.get((a >> 20) as usize).copied().unwrap_or(NO_REGION) as usize;
        if let Some(r) = self.regions.get(i) {
            if a >= r.start && (a - r.start) as u64 + n as u64 <= r.bytes.len() as u64 { return (i, (a - r.start) as usize); }
        }
        self.find_scan(a, n)
    }

    #[inline(never)]
    fn find_scan(&self, a: u32, n: u32) -> (usize, usize) {
        for (i, r) in self.regions.iter().enumerate() {
            if a >= r.start && (a - r.start) as u64 + n as u64 <= r.bytes.len() as u64 {
                return (i, (a - r.start) as usize);
            }
        }
        panic!("mem: {a:#010x}+{n} is not mapped")
    }

    #[inline(always)]
    pub fn read(&self, a: u32, n: u32) -> &[u8] {
        if self.trace { PEND.with(|p| p.borrow_mut().push((a, n as u8, false))); }
        let (i, o) = self.find(a, n);
        &self.regions[i].bytes[o..o + n as usize]
    }
    #[inline(always)]
    pub fn write(&mut self, a: u32, d: &[u8]) {
        // hot path inline (lifted code stores through here); watchpoint, tracing, the gather pipe and hardware
        // registers out of line
        if self.trace || (a >> 24) == 0xCC || WATCH.load(std::sync::atomic::Ordering::Relaxed) != 0 { return self.write_slow(a, d); }
        let (i, o) = self.find(a, d.len() as u32);
        self.regions[i].bytes[o..o + d.len()].copy_from_slice(d);
    }
    #[inline(never)]
    fn write_slow(&mut self, a: u32, d: &[u8]) {
        let w = WATCH.load(std::sync::atomic::Ordering::Relaxed);
        if w != 0 && a <= w && w < a + d.len() as u32 {
            eprintln!("watch {w:#010x}: write {:02x?} at {a:#010x}\n{}", d, std::backtrace::Backtrace::force_capture());
        }
        if self.trace { PEND.with(|p| p.borrow_mut().push((a, d.len() as u8, true))); }
        if let Some(p) = self.pipe.as_mut() {
            if (0xCC00_8000..0xCC00_8020).contains(&a) { p.extend_from_slice(d); return; }
        }
        let (i, o) = self.find(a, d.len() as u32);
        self.regions[i].bytes[o..o + d.len()].copy_from_slice(d);
        if let Some(h) = self.mmio_write {
            if (0xCC00_0000..0xCC00_8000).contains(&a) { h(self, a, d.len()); }
        }
    }

    #[inline(always)] pub fn u8(&self, a: u32) -> u8 { self.read(a, 1)[0] }
    #[inline(always)] pub fn i8(&self, a: u32) -> i8 { self.u8(a) as i8 }
    #[inline(always)] pub fn u16(&self, a: u32) -> u16 { let b = self.read(a, 2); u16::from_be_bytes([b[0], b[1]]) }
    #[inline(always)] pub fn i16(&self, a: u32) -> i16 { self.u16(a) as i16 }
    #[inline(always)] pub fn u32(&self, a: u32) -> u32 { let b = self.read(a, 4); u32::from_be_bytes([b[0], b[1], b[2], b[3]]) }
    #[inline(always)] pub fn i32(&self, a: u32) -> i32 { self.u32(a) as i32 }
    #[inline(always)] pub fn f32(&self, a: u32) -> f32 { f32::from_bits(self.u32(a)) }

    #[inline(always)] pub fn set_u8(&mut self, a: u32, v: u32) { self.write(a, &[v as u8]) }
    #[inline(always)] pub fn set_u16(&mut self, a: u32, v: u32) { self.write(a, &(v as u16).to_be_bytes()) }
    #[inline(always)] pub fn set_u32(&mut self, a: u32, v: u32) { self.write(a, &v.to_be_bytes()) }
    #[inline(always)] pub fn set_f32(&mut self, a: u32, v: f32) { self.write(a, &v.to_bits().to_be_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_is_big_endian_and_aligned() {
        let mut m = Mem::new();
        let a = m.alloc(0x138);
        let b = m.alloc(4);
        assert_eq!(a % 32, 0);
        assert_eq!(b % 32, 0);
        m.set_u16(a + 0x8A, 0x1234);
        assert_eq!(m.read(a + 0x8A, 2), &[0x12, 0x34]);
        m.set_u8(b, 0xFF);
        assert_eq!(m.i8(b), -1);
    }
}
