//! A frame's GX work as a draw list for a graphics-card renderer (colo-gxgpu): vertices already through the XF
//! (clip-space position, lit colour channels, generated texture coordinates), the pixel pipeline state of each
//! draw decoded (TEV stages, konst/registers, alpha test, depth, blend, scissor, viewport), textures decoded to
//! RGBA8 once per content (keyed by a hash of their bytes), and EFB copies in stream order. EFB copies stay on
//! the graphics card: a later draw that samples a copy's destination address uses the copy itself (as Dolphin's
//! "EFB copies to texture only").

use crate::raster::{transform, konst_alpha, konst_color, swap, Fog};
use crate::{Frame, GpuMem, Prim, State};
use std::collections::{HashMap, HashSet};

/// Debug: while set, `Builder::build` prints one line per draw (primitive, texgens, channels, textures, indirect
/// stages, fog) to stderr. Hosts set it for chosen frames (colo-viewer movieperf GXDUMP=frames).
pub static DUMP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct GVertex {
    pub pos: [f32; 4],
    pub col: [[f32; 4]; 2],
    pub tc: [[f32; 4]; 8],
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// A texture: decoded from memory (hash of its bytes), or EFB copy `id` read back as GX texture format `fmt`.
pub enum TexId { Mem(u64), Efb(u32, u32) }

#[derive(Clone, Copy, Default, Debug, PartialEq)]
/// A map's sampler (BP TX_SETMODE0/1): `min` = mip mode (bits 0-1: 0 none, 1 nearest, 2 linear) | min filter
/// linear (bit 2); LOD bias, min and max LOD in 1/16 steps as Dolphin's `SamplerState::Generate` (no mipmaps:
/// all 0; min LOD never above max).
pub struct SamplerDesc { pub wrap_s: u32, pub wrap_t: u32, pub mag_linear: bool, pub min: u32, pub diag_lod: bool, pub lod_bias: f32, pub min_lod: f32, pub max_lod: f32 }

#[derive(Clone, Debug)]
pub struct GState {
    /// XF viewport: wd, ht, zrange, xorig, yorig, farz.
    pub viewport: [f32; 6],
    /// EFB pixels: x0, y0, x1, y1 (inclusive).
    pub scissor: [i32; 4],
    /// GEN_MODE cull: 0 none, 1 back, 2 front, 3 all.
    pub cull: u32,
    pub zmode: u32,
    pub early_z: bool,
    pub cmode: u32,
    pub dstalpha: u32,
    pub pixfmt: u32,
    pub afunc: u32,
    pub nstages: u32,
    pub ntex: u32,
    /// Per stage: colour env, alpha env, order (map | coord << 3 | tex on << 6 | chan << 7), swaps (ras 4 x 2 bits,
    /// tex 4 x 2 bits).
    pub stages: [[u32; 4]; 16],
    /// Per stage konst colour (r, g, b) and alpha.
    pub konst: [[i32; 4]; 16],
    pub regs: [[i32; 4]; 4],
    pub tex: [Option<TexId>; 8],
    pub samp: [SamplerDesc; 8],
    /// Indirect texturing: IND_CMD per stage (BP 0x10+), then (IREF 0x27, coord scales 0x25/0x26, indirect stages).
    pub ind: [u32; 16],
    pub ind_misc: [u32; 4],
    /// Indirect matrices 0-2 as Dolphin's constants: rows (ma, mc, me, 17 - scale) and (mb, md, mf, 0).
    pub ind_mtx: [[i32; 4]; 6],
    /// SU texcoord scales (s, t) for coords 0-7, two per row: the fixed-point texel space of indirect offsets.
    pub su: [[u32; 4]; 4],
    /// Fog (BP 0xE8-0xF2), None when off.
    pub fog: Option<Fog>,
}

#[derive(Clone, Debug)]
pub struct GCopy {
    pub id: u32,
    /// EFB source rectangle: x, y, w, h.
    pub src: [u32; 4],
    pub dst: u32,
    pub to_xfb: bool,
    /// Texture copy format (GX copy formats, real numbering) and intensity conversion.
    pub fmt: u32,
    pub intensity: bool,
    pub half: bool,
    /// A Z copy (PE pixel format Z24): the 24-bit depth as R (bits 16-23), G, B (0-7), A 255, then encoded as colour.
    pub depth: bool,
    /// Clear after copying: colour (r, g, b, a), z, and what the PE masks let through (colour, alpha, z).
    pub clear: Option<([u8; 4], u32, bool, bool, bool)>,
    /// Copy filter, gamma, clamp lines and the XFB's vertical scale.
    pub filter: crate::raster::CopyFilter,
}

#[derive(Clone, Debug)]
pub enum GOp {
    Draw { first: u32, count: u32, state: u32 },
    Copy(GCopy),
}

pub struct GTex { pub w: u32, pub h: u32, pub levels: Vec<Vec<u8>> }

#[derive(Default)]
pub struct GFrame {
    pub verts: Vec<GVertex>,
    pub states: Vec<GState>,
    pub ops: Vec<GOp>,
    pub new_tex: Vec<(u64, GTex)>,
    /// The last XFB copy (what the TV shows).
    pub xfb: Option<u32>,
    /// `Builder::build_deferred`: the frame whose draws still need their vertices (`finish`).
    pending: Option<(Frame, Vec<u32>)>,
}

impl GFrame {
    /// Run the XF over the draws a deferred build left (the vertex work, off the thread that runs the game).
    /// Nothing to do for a frame from `Builder::build`.
    pub fn finish(&mut self) {
        let Some((fr, draws)) = self.pending.take() else { return };
        let t0 = std::time::Instant::now();
        for i in draws {
            let d = &fr.draws[i as usize];
            emit(d, &fr.states[d.state], &mut self.verts);
        }
        tadd(0, t0);
    }
}

impl GFrame {
    /// False until `finish` for a frame from `build_deferred` (its `verts` are still empty).
    pub fn is_finished(&self) -> bool { self.pending.is_none() }
}

/// Vertices a draw produces as triangles (lines and points become quads); 0 = nothing drawn.
fn vert_count(prim: Prim, n: usize) -> usize {
    match prim {
        Prim::Quads | Prim::Quads2 => n / 4 * 6,
        Prim::Triangles => n / 3 * 3,
        Prim::TriStrip | Prim::TriFan => n.saturating_sub(2) * 3,
        Prim::Points => n * 6,
        Prim::Lines => n / 2 * 6,
        Prim::LineStrip => n.saturating_sub(1) * 6,
    }
}

/// Builds GFrames; remembers which textures the renderer already has and where EFB copies landed.
#[derive(Default)]
pub struct Builder {
    tmem: Vec<u8>,
    sent: HashSet<u64>,
    copies: HashMap<u32, u32>,
    next_copy: u32,
    frame_tex: HashMap<(u32, u32, u32, u32, u32, u32), Option<TexId>>,
}

/// Time spent on draw lists (ns): [vertex work (`emit`, in `build` or `finish`), the rest of `build`/`build_deferred`].
/// Diagnostics (colo-viewer storybattle GXTIME=1).
pub static TIMES: [std::sync::atomic::AtomicU64; 2] = [const { std::sync::atomic::AtomicU64::new(0) }; 2];
fn tadd(k: usize, t: std::time::Instant) { TIMES[k].fetch_add(t.elapsed().as_nanos() as u64, std::sync::atomic::Ordering::Relaxed); }

fn fnv(h: &mut u64, b: &[u8]) { for &x in b { *h ^= x as u64; *h = h.wrapping_mul(0x100_0000_01B3); } }
/// FNV-1a over 8-byte words (texture data: hashed every frame for the decode cache; ~8x faster than per byte).
fn fnv_words(h: &mut u64, b: &[u8]) {
    let mut it = b.chunks_exact(8);
    for w in &mut it { *h ^= u64::from_le_bytes([w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7]]); *h = h.wrapping_mul(0x100_0000_01B3); }
    fnv(h, it.remainder());
}

impl Builder {
    pub fn new() -> Builder { Builder { tmem: vec![0; 1 << 20], ..Default::default() } }

    pub fn build(&mut self, fr: &Frame, mem: &dyn GpuMem) -> GFrame {
        let mut g = GFrame::default();
        let draws = self.build_ops(fr, mem, &mut g);
        let t0 = std::time::Instant::now();
        for i in draws { let d = &fr.draws[i as usize]; emit(d, &fr.states[d.state], &mut g.verts); }
        tadd(0, t0);
        g
    }

    /// `build` without the vertex work: everything that reads memory (textures, TLUTs) is done here, and the
    /// returned frame's `finish` transforms the vertices later, on another thread (same result as `build`).
    pub fn build_deferred(&mut self, fr: Frame, mem: &dyn GpuMem) -> GFrame {
        let mut g = GFrame::default();
        let draws = self.build_ops(&fr, mem, &mut g);
        g.pending = Some((fr, draws));
        g
    }

    /// Ops, states and textures; returns the draws (indices into `fr.draws`) whose vertices fill `verts` in order.
    fn build_ops(&mut self, fr: &Frame, mem: &dyn GpuMem, g: &mut GFrame) -> Vec<u32> {
        let t_all = std::time::Instant::now();
        let mut out = Vec::new();
        let mut nv = 0u32;
        self.frame_tex.clear();
        let (mut ci, mut ti) = (0usize, 0usize);
        for i in 0..=fr.draws.len() {
            while ti < fr.tluts.len() && fr.tluts[ti].0 == i {
                let (_, src, dst, n) = fr.tluts[ti];
                let mut b = vec![0u8; n as usize];
                mem.read(src, &mut b);
                let d = dst as usize & 0xF_FFFF;
                let n = b.len().min(self.tmem.len() - d);
                self.tmem[d..d + n].copy_from_slice(&b[..n]);
                self.frame_tex.clear();
                ti += 1;
            }
            while ci < fr.copies.len() && fr.copies[ci].2 == i {
                let (st, v, _) = fr.copies[ci];
                self.copy(&fr.states[st], v, g);
                self.frame_tex.clear();
                ci += 1;
            }
            if let Some(d) = fr.draws.get(i) {
                if DUMP.load(std::sync::atomic::Ordering::Relaxed) { dump(d, &fr.states[d.state], g.ops.len()); }
                let count = vert_count(d.prim, d.verts.len()) as u32;
                if count == 0 { continue; }
                let st = self.state(&fr.states[d.state], mem, g);
                g.states.push(st);
                g.ops.push(GOp::Draw { first: nv, count, state: g.states.len() as u32 - 1 });
                nv += count;
                out.push(i as u32);
            }
        }
        tadd(1, t_all);
        out
    }

    fn state(&mut self, s: &State, mem: &dyn GpuMem, g: &mut GFrame) -> GState {
        let gm = s.bp[0x00];
        let nstages = ((gm >> 10) & 0xF) + 1;
        let mut stages = [[0u32; 4]; 16];
        let mut konst = [[0i32; 4]; 16];
        let mut tex = [None; 8];
        let mut samp = [SamplerDesc::default(); 8];
        for st in 0..nstages as usize {
            let order = (s.bp[0x28 + st / 2] >> (12 * (st & 1))) & 0x3FF;
            let aenv = s.bp[0xC1 + 2 * st];
            let rs = swap(s, aenv & 3);
            let ts = swap(s, aenv >> 2 & 3);
            let sw = (0..4).fold(0u32, |a, k| a | (rs[k] as u32) << (2 * k) | (ts[k] as u32) << (8 + 2 * k));
            stages[st] = [s.bp[0xC0 + 2 * st], aenv, order, sw];
            let ksel = s.bp[0xF6 + st / 2] >> (10 * (st & 1));
            let kc = konst_color(s, ksel >> 4 & 0x1F);
            konst[st] = [kc[0], kc[1], kc[2], konst_alpha(s, ksel >> 9 & 0x1F)];
            let map = (order & 7) as usize;
            if order >> 6 & 1 != 0 { self.bind(s, mem, map, g, &mut tex, &mut samp); }
        }
        // indirect stages: their maps (IREF), and the per-stage commands
        let nind = gm >> 16 & 7;
        let iref = s.bp[0x27];
        for i in 0..nind.min(4) { self.bind(s, mem, (iref >> (6 * i) & 7) as usize, g, &mut tex, &mut samp); }
        let mut ind = [0u32; 16];
        for st in 0..nstages as usize { ind[st] = s.bp[0x10 + st] & 0x1F_FFFF; }
        let mut ind_mtx = [[0i32; 4]; 6];
        for m in 0..3 {
            let [a, b, c] = [s.bp[0x06 + 3 * m], s.bp[0x07 + 3 * m], s.bp[0x08 + 3 * m]];
            let sx = |v: u32| ((v as i32) << 21) >> 21;
            let scale = (a >> 22 & 3) | (b >> 22 & 3) << 2 | (c >> 22 & 3) << 4;
            ind_mtx[2 * m] = [sx(a & 0x7FF), sx(b & 0x7FF), sx(c & 0x7FF), 17 - scale as i32];
            ind_mtx[2 * m + 1] = [sx(a >> 11 & 0x7FF), sx(b >> 11 & 0x7FF), sx(c >> 11 & 0x7FF), 0];
        }
        let mut su = [[0u32; 4]; 4];
        for k in 0..8 { su[k / 2][2 * (k & 1)] = (s.bp[0x30 + 2 * k] & 0xFFFF) + 1; su[k / 2][2 * (k & 1) + 1] = (s.bp[0x31 + 2 * k] & 0xFFFF) + 1; }
        let mut regs = [[0i32; 4]; 4];
        for r in 0..4 {
            let [lo, hi] = s.tevreg[r];
            let sx = |v: u32| ((v as i32) << 21) >> 21;
            regs[r] = [sx(lo & 0x7FF), sx(hi >> 12 & 0x7FF), sx(hi & 0x7FF), sx(lo >> 12 & 0x7FF)];
        }
        let (sc0, sc1, off) = (s.bp[0x20], s.bp[0x21], s.bp[0x59]);
        let ox = ((off & 0x3FF) as i32) * 2 - 342;
        let oy = (((off >> 10) & 0x3FF) as i32) * 2 - 342;
        let f = |u: u32| f32::from_bits(u);
        let xr = &s.xf_reg;
        GState {
            viewport: [f(xr[0x1A]), f(xr[0x1B]), f(xr[0x1C]), f(xr[0x1D]), f(xr[0x1E]), f(xr[0x1F])],
            scissor: [(sc0 >> 12 & 0x7FF) as i32 - 342 - ox, (sc0 & 0x7FF) as i32 - 342 - oy, (sc1 >> 12 & 0x7FF) as i32 - 342 - ox, (sc1 & 0x7FF) as i32 - 342 - oy],
            cull: (gm >> 14) & 3,
            zmode: s.bp[0x40], early_z: s.bp[0x43] >> 6 & 1 != 0, cmode: s.bp[0x41], dstalpha: s.bp[0x42], pixfmt: s.bp[0x43] & 7,
            afunc: s.bp[0xF3], nstages, ntex: gm & 0xF, stages, konst, regs, tex, samp,
            ind, ind_misc: [iref, s.bp[0x25], s.bp[0x26], nind], ind_mtx, su, fog: Fog::new(s),
        }
    }

    /// Texture map `map` and its sampler, once per draw.
    fn bind(&mut self, s: &State, mem: &dyn GpuMem, map: usize, g: &mut GFrame, tex: &mut [Option<TexId>; 8], samp: &mut [SamplerDesc; 8]) {
        if tex[map].is_some() { return; }
        tex[map] = self.texture(s, mem, map, g);
        let r = |base: usize| s.bp[if map < 4 { base + map } else { base + 0x20 + map - 4 }];
        let (m0, m1) = (r(0x80), r(0x84));
        let (min_lod, max_lod, bias) = if m0 >> 5 & 3 == 0 { (0, 0, 0) } else {
            let max = m1 >> 8 & 0xFF;
            ((m1 & 0xFF).min(max), max, ((m0 >> 9 & 0xFF) as i8 as i32 * 8) >> 4)
        };
        samp[map] = SamplerDesc { wrap_s: m0 & 3, wrap_t: m0 >> 2 & 3, mag_linear: m0 >> 4 & 1 != 0, min: m0 >> 5 & 7, diag_lod: m0 >> 8 & 1 != 0,
            lod_bias: bias as f32, min_lod: min_lod as f32, max_lod: max_lod as f32 };
    }

    fn texture(&mut self, s: &State, mem: &dyn GpuMem, map: usize, g: &mut GFrame) -> Option<TexId> {
        let r = |base: usize| s.bp[if map < 4 { base + map } else { base + 0x20 + map - 4 }];
        let (img0, img3, tlut, mode1) = (r(0x88), r(0x94), r(0x98), r(0x84));
        let w = (img0 & 0x3FF) + 1;
        let h = (img0 >> 10 & 0x3FF) + 1;
        let fmt = img0 >> 20 & 0xF;
        let addr = (img3 & 0xFF_FFFF) << 5;
        let key = (addr, fmt, w, h, tlut, mode1);
        if let Some(t) = self.frame_tex.get(&key) { return *t; }
        // an EFB copy made to this address: sample the copy on the graphics card
        if let Some(&id) = self.copies.get(&addr) {
            self.frame_tex.insert(key, Some(TexId::Efb(id, fmt)));
            return Some(TexId::Efb(id, fmt));
        }
        let pal_bytes = if matches!(fmt, 8 | 9 | 10) {
            let n = match fmt { 8 => 16, 9 => 256, _ => 16384 };
            let off = ((tlut & 0x3FF) << 9) as usize & 0xF_FFFF;
            self.tmem[off..(off + 2 * n).min(self.tmem.len())].to_vec()
        } else { Vec::new() };
        let maxlod = ((mode1 >> 8) & 0xFF) as usize / 16;
        let mut raw = Vec::new();
        let mut sizes = Vec::new();
        let (mut lw, mut lh, mut la) = (w as usize, h as usize, addr);
        for _ in 0..=maxlod.min(10) {
            let Some(n) = colo_formats::gx::data_size(fmt, lw, lh) else { self.frame_tex.insert(key, None); return None };
            let mut b = vec![0u8; n];
            mem.read(la, &mut b);
            raw.push(b);
            sizes.push((lw, lh));
            la += n as u32;
            if lw == 1 && lh == 1 { break; }
            lw = (lw / 2).max(1); lh = (lh / 2).max(1);
        }
        let mut hsh = 0xCBF2_9CE4_8422_2325u64;
        fnv(&mut hsh, &[fmt as u8, (tlut >> 10 & 3) as u8]);
        fnv(&mut hsh, &w.to_le_bytes());
        fnv(&mut hsh, &h.to_le_bytes());
        fnv(&mut hsh, &(raw.len() as u32).to_le_bytes());
        for b in &raw { fnv_words(&mut hsh, b); }
        fnv(&mut hsh, &pal_bytes);
        if !self.sent.contains(&hsh) {
            let pal = if pal_bytes.is_empty() { None } else { colo_formats::gx::decode_palette(&pal_bytes, tlut >> 10 & 3, pal_bytes.len() / 2).ok() };
            let mut levels = Vec::new();
            for (b, (lw, lh)) in raw.iter().zip(&sizes) {
                match colo_formats::gx::decode(b, fmt, *lw, *lh, pal.as_deref()) { Ok(d) => levels.push(d), Err(_) => break }
            }
            if levels.is_empty() { self.frame_tex.insert(key, None); return None; }
            g.new_tex.push((hsh, GTex { w, h, levels }));
            self.sent.insert(hsh);
        }
        self.frame_tex.insert(key, Some(TexId::Mem(hsh)));
        Some(TexId::Mem(hsh))
    }

    fn copy(&mut self, s: &State, v: u32, g: &mut GFrame) {
        let (src, sz) = (s.bp[0x49], s.bp[0x4A]);
        let id = self.next_copy;
        self.next_copy += 1;
        let dst = (s.bp[0x4B] & 0xFF_FFFF) << 5;
        let to_xfb = v >> 14 & 1 != 0;
        let tp = v >> 3 & 0xF;
        let clear = if v >> 11 & 1 != 0 {
            let (ar, gb) = (s.bp[0x4F], s.bp[0x50]);
            let cm = s.bp[0x41];
            Some(([(ar & 0xFF) as u8, (gb >> 8 & 0xFF) as u8, (gb & 0xFF) as u8, (ar >> 8 & 0xFF) as u8], s.bp[0x51] & 0xFF_FFFF,
                  cm >> 3 & 1 != 0, cm >> 4 & 1 != 0, s.bp[0x40] >> 4 & 1 != 0))
        } else { None };
        let c = GCopy {
            id, src: [src & 0x3FF, src >> 10 & 0x3FF, (sz & 0x3FF) + 1, (sz >> 10 & 0x3FF) + 1], dst, to_xfb,
            fmt: (tp >> 1) | ((tp & 1) << 3), intensity: v >> 15 & 1 != 0, half: v >> 9 & 1 != 0,
            depth: s.bp[0x43] & 7 == 3, clear, filter: crate::raster::CopyFilter::new(s, v),
        };
        if DUMP.load(std::sync::atomic::Ordering::Relaxed) {
            eprintln!("gx copy{id} src{:?} xfb{} fmt{} half{} z{} filter{:?} gamma{} clamp{:?} ys{}", c.src, to_xfb, c.fmt, c.half, c.depth,
                      c.filter.coef, c.filter.gamma, c.filter.clamp, c.filter.y_scale);
        }
        if to_xfb { g.xfb = Some(id); } else { self.copies.insert(dst, id); }
        g.ops.push(GOp::Copy(c));
    }
}

/// A draw's vertices through the XF, as triangles (`vert_count` of them), appended to `verts`.
fn emit(d: &crate::Draw, s: &State, verts: &mut Vec<GVertex>) {
    let vs = transform(d, s);
    let n = vs.len();
    let start = verts.len();
    let mut push = |k: usize| {
        let v = &vs[k];
        let mut tc = [[0f32; 4]; 8];
        for t in 0..8 { tc[t] = [v.tc[t][0], v.tc[t][1], v.tc[t][2], 0.0]; }
        verts.push(GVertex { pos: v.c, col: v.col, tc });
    };
    match d.prim {
        Prim::Quads | Prim::Quads2 => for q in (0..n / 4 * 4).step_by(4) { for k in [q, q + 1, q + 2, q, q + 2, q + 3] { push(k); } },
        Prim::Triangles => for k in 0..n / 3 * 3 { push(k); },
        Prim::TriStrip => for k in 2..n { let t = if k % 2 == 0 { [k - 2, k - 1, k] } else { [k - 1, k - 2, k] }; for x in t { push(x); } },
        Prim::TriFan => for k in 2..n { for x in [0, k - 1, k] { push(x); } },
        Prim::Lines | Prim::LineStrip | Prim::Points => {
            // lines and points as screen-space quads (SU line/point size 0x22, in 1/6 pixel; lines widen across
            // their minor axis, as Dolphin)
            let su = s.bp[0x22];
            let (vx, vy) = (f32::from_bits(s.xf_reg[0x1A]).abs().max(1e-6), f32::from_bits(s.xf_reg[0x1B]).abs().max(1e-6));
            let quad = |verts: &mut Vec<GVertex>, v: &[&crate::raster::V; 4], off: [[f32; 2]; 4]| {
                for k in [0usize, 1, 2, 0, 2, 3] {
                    let p = v[k].c;
                    let mut tc = [[0f32; 4]; 8];
                    for t in 0..8 { tc[t] = [v[k].tc[t][0], v[k].tc[t][1], v[k].tc[t][2], 0.0]; }
                    verts.push(GVertex { pos: [p[0] + off[k][0] / vx * p[3], p[1] + off[k][1] / vy * p[3], p[2], p[3]], col: v[k].col, tc });
                }
            };
            if d.prim == Prim::Points {
                let h = (su >> 8 & 0xFF) as f32 / 12.0;
                for v in &vs { quad(verts, &[v, v, v, v], [[-h, h], [h, h], [h, -h], [-h, -h]]); }
            } else {
                let h = (su & 0xFF) as f32 / 12.0;
                let pairs: Vec<(usize, usize)> = if d.prim == Prim::Lines { (0..n / 2).map(|k| (2 * k, 2 * k + 1)).collect() } else { (1..n).map(|k| (k - 1, k)).collect() };
                for (a, b) in pairs {
                    let (pa, pb) = (vs[a].c, vs[b].c);
                    let dx = (pb[0] / pb[3] - pa[0] / pa[3]) * vx;
                    let dy = (pb[1] / pb[3] - pa[1] / pa[3]) * vy;
                    let o = if dx.abs() > dy.abs() { [0.0, h] } else { [h, 0.0] };
                    quad(verts, &[&vs[a], &vs[b], &vs[b], &vs[a]], [[o[0], o[1]], [o[0], o[1]], [-o[0], -o[1]], [-o[0], -o[1]]]);
                }
            }
        }
    }
    debug_assert_eq!(verts.len() - start, vert_count(d.prim, n));
}

fn dump(d: &crate::Draw, s: &State, op: usize) {
    let xr = &s.xf_reg;
    let gm = s.bp[0x00];
    let ntex = (xr[0x3F] & 0xF) as usize;
    let tg: Vec<String> = (0..ntex).map(|t| format!("{:x}/{:x}", xr[0x40 + t], xr[0x50 + t])).collect();
    let nst = ((gm >> 10) & 0xF) as usize + 1;
    let tex: Vec<String> = (0..nst).filter_map(|st| {
        let order = (s.bp[0x28 + st / 2] >> (12 * (st & 1))) & 0x3FF;
        if order >> 6 & 1 == 0 { return None; }
        let map = (order & 7) as usize;
        let img0 = s.bp[if map < 4 { 0x88 + map } else { 0xA8 + map - 4 }];
        Some(format!("s{st}:m{map}c{}f{}({}x{})", order >> 3 & 7, img0 >> 20 & 0xF, (img0 & 0x3FF) + 1, (img0 >> 10 & 0x3FF) + 1))
    }).collect();
    let ind: Vec<String> = (0..nst).filter(|&st| s.bp[0x10 + st] != 0).map(|st| format!("{st}:{:x}", s.bp[0x10 + st])).collect();
    let v = d.verts.first();
    eprintln!("gx op{op} {:?} n{} dual{} nchan{} cc[{:x} {:x} {:x} {:x}] tg[{}] st{nst} ntex{} tex[{}] ind[{}] fog{:x} nrm{} nbt{}",
        d.prim, d.verts.len(), xr[0x12] & 1, xr[0x09] & 3, xr[0x0E], xr[0x0F], xr[0x10], xr[0x11], tg.join(" "), gm & 0xF, tex.join(" "), ind.join(" "),
        s.bp[0xF1] >> 21 & 7, v.is_some_and(|v| v.nrm.is_some()), v.is_some_and(|v| v.nbt.is_some()));
}
