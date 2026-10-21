//! The GameCube graphics processor's command stream (the "FIFO"), as the game writes it to the write-gather
//! pipe: register loads (CP, XF, BP), indexed XF loads, display-list calls and primitives with their vertex data.
//!
//! `Gx` keeps the register state the commands build up and decodes every primitive into `Draw`s (vertices in
//! the formats the vertex descriptor and attribute tables say); a renderer then turns state + draws into
//! pixels. Register meanings follow the hardware (YAGCD / libogc / Dolphin's VideoCommon documentation).

pub mod vtx;
pub mod raster;
pub mod cmd;

/// Memory the GPU reads by physical address (display lists, vertex arrays, indexed XF data).
pub trait GpuMem {
    fn read(&self, phys: u32, out: &mut [u8]);
    /// `phys..phys+n` in place, when the memory has it as one slice (what `read` would copy); None = use `read`.
    fn slice(&self, _phys: u32, _n: usize) -> Option<&[u8]> { None }
    fn u8(&self, a: u32) -> u8 { let mut b = [0u8; 1]; self.read(a, &mut b); b[0] }
    fn u16(&self, a: u32) -> u16 { let mut b = [0u8; 2]; self.read(a, &mut b); u16::from_be_bytes(b) }
    fn u32(&self, a: u32) -> u32 { let mut b = [0u8; 4]; self.read(a, &mut b); u32::from_be_bytes(b) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prim { Quads, Quads2, Triangles, TriStrip, TriFan, Lines, LineStrip, Points }

impl Prim {
    fn from_cmd(c: u8) -> Prim {
        match (c >> 3) & 7 {
            0 => Prim::Quads, 1 => Prim::Quads2, 2 => Prim::Triangles, 3 => Prim::TriStrip,
            4 => Prim::TriFan, 5 => Prim::Lines, 6 => Prim::LineStrip, _ => Prim::Points,
        }
    }
}

/// One primitive with its vertices decoded, and the BP/XF state it was drawn with (snapshot ids).
#[derive(Clone, Debug)]
pub struct Draw {
    pub prim: Prim,
    pub vat: u8,
    /// The vertices' bytes with indexed attributes already fetched (`vtx::Format::capture`), in `fmt` (all
    /// direct); `verts` decodes them. Decoding waits for the consumer (off the game thread).
    pub raw: Vec<u8>,
    pub fmt: vtx::Format,
    /// Vertex count.
    pub n: usize,
    /// Index of the state snapshot in `Frame::states` this draw uses.
    pub state: usize,
}

/// A `GpuMem` with nothing in it (decoding a draw's captured, all-direct vertices reads no memory).
struct NoMem;
impl GpuMem for NoMem { fn read(&self, _: u32, out: &mut [u8]) { out.fill(0) } }

impl Draw {
    /// Vertex `k`, decoded with the draw's state's CP registers (`State::cp`: the matrix indices not in the stream).
    pub fn vert(&self, k: usize, cp: &[u32; 256]) -> vtx::Vertex {
        let sz = self.raw.len() / self.n.max(1);
        self.fmt.decode(&self.raw[k * sz..(k + 1) * sz], cp, &NoMem)
    }
    /// All the vertices, decoded.
    pub fn verts<'a>(&'a self, cp: &'a [u32; 256]) -> impl Iterator<Item = vtx::Vertex> + 'a { (0..self.n).map(move |k| self.vert(k, cp)) }
}

/// Everything the GPU state holds that affects a draw.
#[derive(Clone)]
pub struct State {
    pub cp: [u32; 256],
    /// XF memory (0x0000..0x0800: position/texture matrices, normal matrices 0x400, post matrices 0x500, lights 0x600) and registers (0x1000..0x1058), as u32 words.
    pub xf_mem: XfMem,
    pub xf_reg: [u32; 0x58],
    pub bp: [u32; 256],
    /// BP 0xE0..0xE7 hold two register files at one address each (bit 23: 0 TEV colour registers PREV, REG0-2;
    /// 1 konst colours K0-K3): [register][lo (R/A), hi (B/G)].
    pub tevreg: [[u32; 2]; 4],
    pub kreg: [[u32; 2]; 4],
}

/// XF memory (0x800 words) as 16 shared blocks of 128 words: a state snapshot (one per draw whose state changed)
/// shares every block until a later write copies that one, instead of copying all 8 KB.
#[derive(Clone)]
pub struct XfMem([std::sync::Arc<[u32; XF_BLOCK]>; 0x800 / XF_BLOCK]);
const XF_BLOCK: usize = 128;

impl XfMem {
    pub fn new() -> XfMem { let z = std::sync::Arc::new([0; XF_BLOCK]); XfMem(std::array::from_fn(|_| z.clone())) }
    #[inline]
    pub fn set(&mut self, a: usize, v: u32) {
        let b = &mut self.0[a / XF_BLOCK];
        if b[a % XF_BLOCK] != v { std::sync::Arc::make_mut(b)[a % XF_BLOCK] = v; }
    }
    pub fn len(&self) -> usize { 0x800 }
}

impl std::ops::Index<usize> for XfMem {
    type Output = u32;
    #[inline]
    fn index(&self, a: usize) -> &u32 { &self.0[a / XF_BLOCK][a % XF_BLOCK] }
}

impl Default for State {
    fn default() -> Self { State { cp: [0; 256], xf_mem: XfMem::new(), xf_reg: [0; 0x58], bp: [0; 256], tevreg: [[0; 2]; 4], kreg: [[0; 2]; 4] } }
}

/// What one frame's stream produced.
pub struct Frame {
    pub draws: Vec<Draw>,
    pub states: Vec<State>,
    /// EFB copies (BP 0x52 executes): (state index, BP 0x52 value), in order with the draws (by draw count).
    pub copies: Vec<(usize, u32, usize)>,
    /// TLUT loads (BP 0x65): (draw count, source physical address, TMEM byte address, bytes), in order.
    pub tluts: Vec<(usize, u32, u32, u32)>,
    /// PE "draw done" (BP 0x45) writes seen: the GPU finishing everything before it (PE finish interrupt).
    pub pe_done: u32,
    /// Counts by command for diagnostics.
    pub stats: [u32; 256],
}

impl Default for Frame {
    fn default() -> Self { Frame { draws: Vec::new(), states: Vec::new(), copies: Vec::new(), tluts: Vec::new(), pe_done: 0, stats: [0; 256] } }
}

#[derive(Default)]
pub struct Gx {
    pub s: State,
    dirty: bool,
    /// Per VAT: the vertex format worked out from the CP words it depends on (`fmt_key`), until they change.
    fmts: [Option<([u32; 5], VtxFmt)>; 8],
}

/// A vertex format and what a primitive in it needs: stream bytes per vertex, the all-direct form, whether any
/// attribute is indexed.
#[derive(Clone)]
struct VtxFmt { fmt: vtx::Format, size: usize, direct: vtx::Format, dsize: usize, indexed: bool }

impl Gx {
    pub fn new() -> Gx { Gx { s: State::default(), dirty: true, fmts: Default::default() } }

    fn vtx_fmt(&mut self, vat: usize) -> &VtxFmt {
        let cp = &self.s.cp;
        let key = [cp[0x50], cp[0x60], cp[0x70 + vat], cp[0x80 + vat], cp[0x90 + vat]];
        if !self.fmts[vat].as_ref().is_some_and(|(k, _)| *k == key) {
            let fmt = vtx::Format::new(cp, vat);
            let direct = fmt.direct();
            let v = VtxFmt { size: fmt.size(), dsize: direct.size(), indexed: fmt.indexed(), fmt, direct };
            self.fmts[vat] = Some((key, v));
        }
        &self.fmts[vat].as_ref().unwrap().1
    }

    fn snap(&mut self, f: &mut Frame) -> usize {
        if self.dirty || f.states.is_empty() {
            f.states.push(self.s.clone());
            self.dirty = false;
        }
        f.states.len() - 1
    }

    /// Run a command stream (the pipe's bytes, or a display list).
    /// Returns how many bytes were consumed: a command cut off at the end is left for the next call.
    pub fn run(&mut self, data: &[u8], mem: &dyn GpuMem, f: &mut Frame, depth: u32) -> usize {
        let mut i = 0usize;
        let rd8 = |i: &mut usize| -> Option<u8> { let v = *data.get(*i)?; *i += 1; Some(v) };
        let rd16 = |i: &mut usize| -> Option<u16> { if *i + 2 > data.len() { return None; } let v = u16::from_be_bytes([data[*i], data[*i + 1]]); *i += 2; Some(v) };
        let rd32 = |i: &mut usize| -> Option<u32> { if *i + 4 > data.len() { return None; } let v = u32::from_be_bytes([data[*i], data[*i + 1], data[*i + 2], data[*i + 3]]); *i += 4; Some(v) };
        while i < data.len() {
            let start = i;
            let cmd = data[i];
            i += 1;
            f.stats[cmd as usize] += 1;
            match cmd {
                0x00 => {}
                0x08 => {
                    let (Some(r), Some(v)) = (rd8(&mut i), rd32(&mut i)) else { return start };
                    self.s.cp[r as usize] = v;
                    self.dirty = true;
                }
                0x10 => {
                    let Some(h) = rd32(&mut i) else { return start };
                    let n = ((h >> 16) & 0xF) + 1;
                    let a = h & 0xFFFF;
                    for k in 0..n {
                        let Some(v) = rd32(&mut i) else { return start };
                        self.xf_write(a + k, v);
                    }
                    self.dirty = true;
                }
                0x20 | 0x28 | 0x30 | 0x38 => {
                    // indexed XF load from array 12 + (A..D): index << 16 | (n - 1) << 12 | xf address
                    let Some(v) = rd32(&mut i) else { return start };
                    let arr = 12 + ((cmd - 0x20) >> 3) as usize;
                    let idx = v >> 16;
                    let n = ((v >> 12) & 0xF) + 1;
                    let a = v & 0xFFF;
                    let base = self.s.cp[0xA0 + arr] & 0x3FF_FFFF;
                    let stride = self.s.cp[0xB0 + arr] & 0xFF;
                    for k in 0..n { let w = mem.u32(base + idx * stride + 4 * k); self.xf_write(a + k, w); }
                    self.dirty = true;
                }
                0x40 => {
                    let (Some(a), Some(n)) = (rd32(&mut i), rd32(&mut i)) else { return start };
                    if depth < 4 {
                        // the display list in place when memory has it as one slice (no copy per call)
                        if let Some(dl) = mem.slice(a & 0x3FF_FFFF, n as usize) { self.run(dl, mem, f, depth + 1); } else {
                            let mut dl = vec![0u8; n as usize];
                            mem.read(a & 0x3FF_FFFF, &mut dl);
                            self.run(&dl, mem, f, depth + 1);
                        }
                    }
                }
                0x44 | 0x48 => {}
                0x61 => {
                    let Some(v) = rd32(&mut i) else { return start };
                    self.bp_write(v, f);
                }
                c if c & 0x80 != 0 => {
                    let Some(n) = rd16(&mut i) else { return start };
                    let vat = c & 7;
                    let size = self.vtx_fmt(vat as usize).size;
                    if i + size * n as usize > data.len() { return start; }
                    let st = self.snap(f);
                    let n = n as usize;
                    let v = &self.fmts[vat as usize].as_ref().unwrap().1;
                    let raw = if v.indexed {
                        let mut raw = Vec::with_capacity(n * v.dsize);
                        v.fmt.capture_n(&data[i..i + size * n], n, &self.s.cp, mem, &mut raw);
                        raw
                    } else { data[i..i + size * n].to_vec() };
                    let direct = v.direct.clone();
                    i += size * n;
                    f.draws.push(Draw { prim: Prim::from_cmd(c), vat, raw, fmt: direct, n, state: st });
                }
                _ => {
                    // unknown opcode: the stream is out of sync
                    return data.len();
                }
            }
        }
        i
    }

    /// Seed the register state from the SDK's GX shadow (`__GXData`, the `gx` pointer; layout of the 2001-2002
    /// libGX: Melee's `__gx.h`, offsets checked against Colosseum's GXInit 0x800B5E8C). For a snapshot taken
    /// mid-game: the GPU's registers are not in RAM, but libGX keeps copies of most; the rest is GXInit's
    /// defaults until the game sends it again.
    pub fn seed_sdk(&mut self, mem: &dyn GpuMem, gx: u32) {
        let u = |o: u32| mem.u32(gx + o);
        let mut f = Frame::default();
        // GXInit defaults the shadow does not keep: identity matrices (GXLoadPosMtxImm(id, 0),
        // GXLoadTexMtxImm(id, GX_IDENTITY 60), GXLoadTexMtxImm(id, GX_PTIDENTITY 125) -> post matrix 0x500 + 61 * 4)
        for base in [0u32, 60 * 4, 0x500 + 61 * 4] {
            for r in 0..3 { for c in 0..4 { self.xf_write(base + 4 * r + c, if r == c { 1.0f32.to_bits() } else { 0 }); } }
        }
        // GXInit also enables the dual texture transform once (XF 0x1012 = 1, __GXInitGX); it is never shadowed.
        // Without it every texgen skipped its post matrix: the battle floor's projected shadows (texgen = eye-space
        // position through post matrices 3/6) landed in front of the camera instead of under the characters
        self.xf_write(0x1012, 1);
                // scissor box offset (0, 0)
        self.bp_write(0x59 << 24 | (171 | 171 << 10), &mut f);
        self.s.cp[0x50] = u(0x14);
        self.s.cp[0x60] = u(0x18);
        for i in 0..8 {
            self.s.cp[0x70 + i] = u(0x1C + 4 * i as u32);
            self.s.cp[0x80 + i] = u(0x3C + 4 * i as u32);
            self.s.cp[0x90 + i] = u(0x5C + 4 * i as u32);
        }
        self.s.cp[0x30] = u(0x80);
        self.s.cp[0x40] = u(0x84);
        for i in 0..4 {
            self.s.cp[0xAC + i] = u(0x88 + 4 * i as u32);
            self.s.cp[0xBC + i] = u(0x98 + 4 * i as u32);
        }
        self.xf_write(0x100A, u(0xA8));
        self.xf_write(0x100B, u(0xAC));
        self.xf_write(0x100C, u(0xB0));
        self.xf_write(0x100D, u(0xB4));
        // BP command words kept whole
        let mut bp: Vec<u32> = vec![u(0x7C)];
        for i in 0..8 { bp.push(u(0xB8 + 4 * i)); bp.push(u(0xD8 + 4 * i)); }
        bp.push(u(0xF8)); bp.push(u(0xFC));
        for i in 0..8 { bp.push(u(0x100 + 4 * i)); }
        bp.push(u(0x120)); bp.push(u(0x128)); bp.push(u(0x12C));
        for i in 0..16 { bp.push(u(0x130 + 4 * i)); bp.push(u(0x170 + 4 * i)); }
        for i in 0..8 { bp.push(u(0x1B0 + 4 * i)); }
        for o in [0x1D0, 0x1D4, 0x1D8, 0x1DC, 0x204] { bp.push(u(o)); }
        for i in 0..8 { bp.push(u(0x45C + 4 * i)); bp.push(u(0x47C + 4 * i)); }
        for w in bp { if w >> 24 != 0 { self.bp_write(w, &mut f); } }
        // projection and viewport (GXSetProjection / GXSetViewport arithmetic)
        let fl = |o: u32| f32::from_bits(u(o));
        for k in 0..6 { self.xf_write(0x1020 + k, u(0x424 + 4 * k)); }
        self.xf_write(0x1026, u(0x420));
        let (l, t, w, h, n, fz) = (fl(0x43C), fl(0x440), fl(0x444), fl(0x448), fl(0x44C), fl(0x450));
        let zmax = 16777215.0f32;
        let vp = [w * 0.5, -h * 0.5, zmax * (fz - n), l + w * 0.5 + 342.0, t + h * 0.5 + 342.0, zmax * fz];
        for k in 0..6 { self.xf_write(0x101A + k as u32, vp[k].to_bits()); }
        self.dirty = true;
    }

    fn xf_write(&mut self, a: u32, v: u32) {
        if a < 0x800 { self.s.xf_mem.set(a as usize, v); } else if (0x1000..0x1058).contains(&a) { self.s.xf_reg[(a - 0x1000) as usize] = v; }
    }

    fn bp_write(&mut self, v: u32, f: &mut Frame) {
        let r = (v >> 24) as usize;
        let d = v & 0xFF_FFFF;
        if r == 0xFE {
            // mask for the next BP write
            self.s.bp[0xFE] = d;
            return;
        }
        let mask = if self.s.bp[0xFE] != 0 { self.s.bp[0xFE] } else { 0xFF_FFFF };
        self.s.bp[0xFE] = 0;
        self.s.bp[r] = (self.s.bp[r] & !mask) | (d & mask);
        self.dirty = true;
        if (0xE0..0xE8).contains(&r) {
            let v = self.s.bp[r];
            let (i, h) = ((r - 0xE0) / 2, (r - 0xE0) % 2);
            if v >> 23 & 1 != 0 { self.s.kreg[i][h] = v; } else { self.s.tevreg[i][h] = v; }
        }
        if r == 0x45 { f.pe_done += 1; }
        if r == 0x65 {
            let v = self.s.bp[0x65];
            f.tluts.push((f.draws.len(), (self.s.bp[0x64] & 0x1F_FFFF) << 5, (v & 0x3FF) << 9, ((v >> 10) & 0x7FF) << 5));
        }
        if r == 0x52 {
            let st = self.snap(f);
            f.copies.push((st, self.s.bp[r], f.draws.len()));
        }
    }
}
