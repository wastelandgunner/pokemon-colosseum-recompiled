//! A software GX: transform/lighting/texgen (XF), triangle setup and rasterization (SU/RAS), texturing and the
//! TEV, alpha test, depth, blending (PE) into a 640x528 EFB, and EFB copies to textures and the XFB in memory.
//!
//! Register meanings follow the hardware as documented by YAGCD, libogc and Dolphin's VideoCommon /
//! VideoBackends/Software (described, not copied). Precision: TEV and blending in the hardware's integer math;
//! transforms and lighting in f32 (the XF is a float unit); texture filtering with 7-bit weights.

use crate::{Draw, Frame, GpuMem, Prim, State};
use std::collections::HashMap;
use std::sync::Arc;

pub const EFB_W: usize = 640;
pub const EFB_H: usize = 528;

/// Memory the GPU writes (EFB copies).
pub trait GpuMemMut: GpuMem {
    fn write(&mut self, phys: u32, data: &[u8]);
}

struct Tex { w: usize, h: usize, levels: Vec<Vec<u8>> }

pub struct Renderer {
    pub color: Vec<[u8; 4]>,
    pub depth: Vec<u32>,
    tmem: Vec<u8>,
    tex: HashMap<(u32, u32, u32, u32, u32, u32), Arc<Tex>>,
    /// XFB copies made this frame: (physical address, width, height).
    pub xfb: Vec<(u32, u32, u32)>,
    /// Things the renderer does not do yet, counted (for diagnostics).
    pub unsupported: HashMap<&'static str, u32>,
    /// Print every draw (debug).
    pub log: bool,
    /// Time spent preparing draws / rasterizing (profiling).
    pub prep_time: std::time::Duration,
    pub raster_time: std::time::Duration,
}

impl Default for Renderer { fn default() -> Self { Renderer::new() } }

#[derive(Clone, Copy, Default)]
pub(crate) struct V {
    /// clip space
    pub(crate) c: [f32; 4],
    pub(crate) col: [[f32; 4]; 2],
    pub(crate) tc: [[f32; 3]; 8],
}

#[derive(Clone, Copy)]
struct SV {
    x: f32, y: f32, z: f32, iw: f32,
    col: [[f32; 4]; 2],
    tc: [[f32; 3]; 8],
}

fn f(u: u32) -> f32 { f32::from_bits(u) }
fn rgba(u: u32) -> [f32; 4] { [(u >> 24) as f32, (u >> 16 & 0xFF) as f32, (u >> 8 & 0xFF) as f32, (u & 0xFF) as f32] }

impl Renderer {
    pub fn new() -> Renderer {
        Renderer { color: vec![[0; 4]; EFB_W * EFB_H], depth: vec![0xFF_FFFF; EFB_W * EFB_H], tmem: vec![0; 1 << 20], tex: HashMap::new(), xfb: Vec::new(), unsupported: HashMap::new(), log: false, prep_time: Default::default(), raster_time: Default::default() }
    }

    fn note(&mut self, what: &'static str) { *self.unsupported.entry(what).or_default() += 1; }

    /// Execute a frame's draws, TLUT loads and EFB copies in stream order.
    pub fn execute(&mut self, fr: &Frame, mem: &mut dyn GpuMemMut) {
        self.tex.clear();
        self.xfb.clear();
        let (mut ci, mut ti) = (0usize, 0usize);
        let mut batch: Vec<Prep> = Vec::new();
        for i in 0..=fr.draws.len() {
            let event = (ti < fr.tluts.len() && fr.tluts[ti].0 == i) || (ci < fr.copies.len() && fr.copies[ci].2 == i);
            if event || i == fr.draws.len() { let b = std::mem::take(&mut batch); let t0 = std::time::Instant::now(); self.run_batch(&b); self.raster_time += t0.elapsed(); }
            while ti < fr.tluts.len() && fr.tluts[ti].0 == i {
                let (_, src, dst, n) = fr.tluts[ti];
                let mut b = vec![0u8; n as usize];
                mem.read(src, &mut b);
                let d = dst as usize & 0xF_FFFF;
                let n = b.len().min(self.tmem.len() - d);
                self.tmem[d..d + n].copy_from_slice(&b[..n]);
                self.tex.clear();
                ti += 1;
            }
            while ci < fr.copies.len() && fr.copies[ci].2 == i {
                let (st, v, _) = fr.copies[ci];
                self.copy(&fr.states[st], v, mem);
                self.tex.clear();
                ci += 1;
            }
            if let Some(d) = fr.draws.get(i) {
                let t0 = std::time::Instant::now();
                if let Some(p) = self.prepare(d, &fr.states[d.state], mem) { batch.push(p); }
                self.prep_time += t0.elapsed();
            }
        }
    }

    // ---------------------------------------------------------------- XF
    fn transform(&mut self, d: &Draw, s: &State) -> Vec<V> { transform(d, s) }

    // ---------------------------------------------------------------- primitives
    /// Transform, assemble, clip and set up a draw's triangles, with its TEV and texture setup decoded once.
    fn prepare<'s>(&mut self, d: &Draw, s: &'s State, mem: &dyn GpuMem) -> Option<Prep<'s>> {
        let vs = self.transform(d, s);
        let n = vs.len();
        let mut idx: Vec<[usize; 3]> = Vec::new();
        match d.prim {
            Prim::Quads | Prim::Quads2 => for q in (0..n / 4 * 4).step_by(4) { idx.push([q, q + 1, q + 2]); idx.push([q, q + 2, q + 3]); },
            Prim::Triangles => for t in (0..n / 3 * 3).step_by(3) { idx.push([t, t + 1, t + 2]); },
            Prim::TriStrip => for k in 2..n { idx.push(if k % 2 == 0 { [k - 2, k - 1, k] } else { [k - 1, k - 2, k] }); },
            Prim::TriFan => for k in 2..n { idx.push([0, k - 1, k]); },
            _ => { self.note("lines/points"); return None; }
        }
        let gm = s.bp[0x00];
        let nstages = ((gm >> 10) & 0xF) as usize + 1;
        let ntex = (gm & 0xF) as usize;
        let mut stages = Vec::with_capacity(nstages);
        let mut tex: [Option<Arc<Tex>>; 8] = Default::default();
        let mut samp = [Sampler::default(); 8];
        for st in 0..nstages {
            let order = s.bp[0x28 + st / 2] >> (12 * (st & 1));
            let map = (order & 7) as usize;
            let tex_on = order >> 6 & 1 != 0;
            let aenv = s.bp[0xC1 + 2 * st];
            if s.bp[0x10 + st] != 0 { self.note("indirect texturing"); }
            let ksel = s.bp[0xF6 + st / 2] >> (10 * (st & 1));
            if tex_on && tex[map].is_none() {
                tex[map] = self.texture(s, mem, map);
                samp[map] = Sampler::new(s, map);
            }
            stages.push(Stage {
                map, coord: (order >> 3 & 7) as usize, tex_on, chan: order >> 7 & 7,
                cenv: s.bp[0xC0 + 2 * st], aenv, rs: swap(s, aenv & 3), ts: swap(s, aenv >> 2 & 3),
                kc: konst_color(s, ksel >> 4 & 0x1F), ka: konst_alpha(s, ksel >> 9 & 0x1F),
            });
        }
        let mut regs = [[0i32; 4]; 4];
        for r in 0..4 {
            let [lo, hi] = s.tevreg[r];
            let sx = |v: u32| ((v as i32) << 21) >> 21;
            regs[r] = [sx(lo & 0x7FF), sx(hi >> 12 & 0x7FF), sx(hi & 0x7FF), sx(lo >> 12 & 0x7FF)];
        }
        // scissor (BP 0x20/0x21, offset BP 0x59), in EFB pixels
        let (sc0, sc1, off) = (s.bp[0x20], s.bp[0x21], s.bp[0x59]);
        let ox = ((off & 0x3FF) as i32) * 2 - 342;
        let oy = (((off >> 10) & 0x3FF) as i32) * 2 - 342;
        let clipr = [((sc0 >> 12 & 0x7FF) as i32 - 342 - ox).max(0), ((sc0 & 0x7FF) as i32 - 342 - oy).max(0),
                     ((sc1 >> 12 & 0x7FF) as i32 - 342 - ox).min(EFB_W as i32 - 1), ((sc1 & 0x7FF) as i32 - 342 - oy).min(EFB_H as i32 - 1)];
        let mut tris = Vec::new();
        for t in idx {
            let poly = clip(vec![vs[t[0]], vs[t[1]], vs[t[2]]]);
            if poly.len() < 3 { continue; }
            let sv: Vec<SV> = poly.iter().map(|v| to_screen(v, s)).collect();
            for k in 1..sv.len() - 1 {
                let (a, b, c) = (sv[0], sv[k], sv[k + 1]);
                let area = (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y);
                if area == 0.0 || !area.is_finite() { continue; }
                // GEN_MODE cull: 1 back, 2 front, 3 all; front faces are clockwise on screen (area > 0, y down)
                match (gm >> 14) & 3 { 1 if area < 0.0 => continue, 2 if area > 0.0 => continue, 3 => continue, _ => {} }
                let (a, b, c) = if area < 0.0 { (a, c, b) } else { (a, b, c) };
                let area = area.abs();
                let minx = (a.x.min(b.x).min(c.x).floor() as i32).max(clipr[0]);
                let maxx = (a.x.max(b.x).max(c.x).ceil() as i32).min(clipr[2]);
                let miny = (a.y.min(b.y).min(c.y).floor() as i32).max(clipr[1]);
                let maxy = (a.y.max(b.y).max(c.y).ceil() as i32).min(clipr[3]);
                if minx > maxx || miny > maxy { continue; }
                let mut grads = [0f32; 8];
                for tt in 0..ntex { grads[tt] = tex_grad(&a, &b, &c, tt, area); }
                tris.push(Tri { v: [a, b, c], area, grads, bbox: [minx, miny, maxx, maxy] });
            }
        }
        if tris.is_empty() { return None; }
        let zmode = s.bp[0x40];
        Some(Prep { s, stages, ntex, regs, tex, samp, tris, zmode, early: s.bp[0x43] >> 6 & 1 != 0,
                    afunc: s.bp[0xF3], cmode: s.bp[0x41], dstalpha: s.bp[0x42], pixfmt: s.bp[0x43] & 7 })
    }

    /// Rasterize prepared draws into the EFB, in parallel horizontal bands (each band draws everything in order
    /// for its rows, so the per-pixel order is the stream's).
    fn run_batch(&mut self, batch: &[Prep]) {
        if batch.is_empty() { return; }
        let bands = std::thread::available_parallelism().map_or(4, |n| n.get()).clamp(1, 16);
        let rows = EFB_H.div_ceil(bands);
        let color = &mut self.color;
        let depth = &mut self.depth;
        std::thread::scope(|sc| {
            for (k, (cb, db)) in color.chunks_mut(rows * EFB_W).zip(depth.chunks_mut(rows * EFB_W)).enumerate() {
                let y0 = k * rows;
                sc.spawn(move || {
                    for p in batch { for t in &p.tris { raster(p, t, cb, db, y0, cb.len() / EFB_W); } }
                });
            }
        });
    }

    // ---------------------------------------------------------------- textures
    fn texture(&mut self, s: &State, mem: &dyn GpuMem, map: usize) -> Option<Arc<Tex>> {
        let r = |base: usize| s.bp[if map < 4 { base + map } else { base + 0x20 + map - 4 }];
        let img0 = r(0x88);
        let img3 = r(0x94);
        let tlut = r(0x98);
        let mode1 = r(0x84);
        let w = (img0 & 0x3FF) + 1;
        let h = (img0 >> 10 & 0x3FF) + 1;
        let fmt = img0 >> 20 & 0xF;
        let addr = (img3 & 0xFF_FFFF) << 5;
        let key = (addr, fmt, w, h, tlut, mode1);
        if let Some(t) = self.tex.get(&key) { return Some(t.clone()); }
        let pal = if matches!(fmt, 8 | 9 | 10) {
            let n = match fmt { 8 => 16, 9 => 256, _ => 16384 };
            let off = ((tlut & 0x3FF) << 9) as usize & 0xF_FFFF;
            let bytes = &self.tmem[off..(off + 2 * n).min(self.tmem.len())];
            colo_formats::gx::decode_palette(bytes, tlut >> 10 & 3, bytes.len() / 2).ok()
        } else { None };
        let maxlod = ((mode1 >> 8) & 0xFF) as usize / 16;
        let mut levels = Vec::new();
        let (mut lw, mut lh, mut la) = (w as usize, h as usize, addr);
        for _ in 0..=maxlod.min(10) {
            let n = colo_formats::gx::data_size(fmt, lw, lh)?;
            let mut b = vec![0u8; n];
            mem.read(la, &mut b);
            levels.push(colo_formats::gx::decode(&b, fmt, lw, lh, pal.as_deref()).ok()?);
            la += n as u32;
            if lw == 1 && lh == 1 { break; }
            lw = (lw / 2).max(1); lh = (lh / 2).max(1);
        }
        let t = Arc::new(Tex { w: w as usize, h: h as usize, levels });
        self.tex.insert(key, t.clone());
        Some(t)
    }

    // ---------------------------------------------------------------- EFB copies
    fn copy(&mut self, s: &State, v: u32, mem: &mut dyn GpuMemMut) {
        let src = s.bp[0x49];
        let sz = s.bp[0x4A];
        let (sx, sy) = ((src & 0x3FF) as usize, (src >> 10 & 0x3FF) as usize);
        let (w, h) = ((sz & 0x3FF) as usize + 1, (sz >> 10 & 0x3FF) as usize + 1);
        let dst = (s.bp[0x4B] & 0xFF_FFFF) << 5;
        let stride = (s.bp[0x4D] & 0x3FF) << 5;
        let px = |x: usize, y: usize| -> [u8; 4] {
            let (x, y) = ((sx + x).min(EFB_W - 1), (sy + y).min(EFB_H - 1));
            self.color[y * EFB_W + x]
        };
        if v >> 14 & 1 != 0 {
            // XFB: YUYV 4:2:2, vertical scale BP 0x4E (256 = 1:1)
            let ys = s.bp[0x4E] & 0x1FF;
            let oh = if ys != 0 && ys != 256 { (h as u32 * 256 / ys) as usize } else { h };
            let mut line = vec![0u8; w * 2];
            for oy in 0..oh {
                let y = (oy * h / oh.max(1)).min(h - 1);
                for x in (0..w).step_by(2) {
                    let (p0, p1) = (px(x, y), px(x + 1, y));
                    let yv = |p: [u8; 4]| (((66 * p[0] as i32 + 129 * p[1] as i32 + 25 * p[2] as i32 + 128) >> 8) + 16).clamp(0, 255) as u8;
                    let (r, g, b) = ((p0[0] as i32 + p1[0] as i32) / 2, (p0[1] as i32 + p1[1] as i32) / 2, (p0[2] as i32 + p1[2] as i32) / 2);
                    let u = (((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128).clamp(0, 255) as u8;
                    let vv = (((112 * r - 94 * g - 18 * b + 128) >> 8) + 128).clamp(0, 255) as u8;
                    line[x * 2] = yv(p0); line[x * 2 + 1] = u; line[x * 2 + 2] = yv(p1); line[x * 2 + 3] = vv;
                }
                mem.write(dst + oy as u32 * stride, &line);
            }
            self.xfb.push((dst, w as u32, oh as u32));
        } else {
            let tp = v >> 3 & 0xF;
            let fmt = (tp >> 1) | ((tp & 1) << 3);
            let intensity = v >> 15 & 1 != 0;
            let half = v >> 9 & 1 != 0;
            let (w2, h2) = if half { (w / 2, h / 2) } else { (w, h) };
            let mut img = vec![[0u8; 4]; w2 * h2];
            for y in 0..h2 {
                for x in 0..w2 {
                    img[y * w2 + x] = if half {
                        let mut acc = [0u32; 4];
                        for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] { let p = px(2 * x + dx, 2 * y + dy); for k in 0..4 { acc[k] += p[k] as u32; } }
                        acc.map(|a| (a / 4) as u8)
                    } else { px(x, y) };
                }
            }
            match encode(&img, w2, h2, fmt, intensity, stride as usize) {
                Some(b) => mem.write(dst, &b),
                None => self.note("EFB copy format"),
            }
        }
        if v >> 11 & 1 != 0 {
            let ar = s.bp[0x4F];
            let gb = s.bp[0x50];
            let cc = [(ar & 0xFF) as u8, (gb >> 8 & 0xFF) as u8, (gb & 0xFF) as u8, (ar >> 8 & 0xFF) as u8];
            let cz = s.bp[0x51] & 0xFF_FFFF;
            let cm = s.bp[0x41];
            let zm = s.bp[0x40];
            for y in sy..(sy + h).min(EFB_H) {
                for x in sx..(sx + w).min(EFB_W) {
                    let i = y * EFB_W + x;
                    if cm >> 3 & 1 != 0 { self.color[i][0] = cc[0]; self.color[i][1] = cc[1]; self.color[i][2] = cc[2]; }
                    if cm >> 4 & 1 != 0 { self.color[i][3] = cc[3]; }
                    if zm >> 4 & 1 != 0 { self.depth[i] = cz; }
                }
            }
        }
    }
}


/// A draw ready to rasterize: its triangles in screen space and its pixel state decoded.
struct Prep<'s> {
    #[allow(dead_code)]
    s: &'s State,
    stages: Vec<Stage>,
    ntex: usize,
    regs: [[i32; 4]; 4],
    tex: [Option<Arc<Tex>>; 8],
    samp: [Sampler; 8],
    tris: Vec<Tri>,
    zmode: u32,
    early: bool,
    afunc: u32,
    cmode: u32,
    dstalpha: u32,
    pixfmt: u32,
}

struct Tri { v: [SV; 3], area: f32, grads: [f32; 8], bbox: [i32; 4] }

struct Stage {
    map: usize, coord: usize, tex_on: bool, chan: u32, cenv: u32, aenv: u32,
    rs: [usize; 4], ts: [usize; 4], kc: [i32; 3], ka: i32,
}

#[derive(Clone, Copy, Default)]
struct Sampler { ws: u32, wt: u32, mag_lin: bool, min: u32, bias: f32, minlod: f32, maxlod: f32 }

impl Sampler {
    fn new(s: &State, map: usize) -> Sampler {
        let r = |base: usize| s.bp[if map < 4 { base + map } else { base + 0x20 + map - 4 }];
        let (m0, m1) = (r(0x80), r(0x84));
        Sampler { ws: m0 & 3, wt: m0 >> 2 & 3, mag_lin: m0 >> 4 & 1 != 0, min: m0 >> 5 & 7,
                  bias: ((m0 >> 9 & 0xFF) as i8 as f32) / 32.0, minlod: (m1 & 0xFF) as f32 / 16.0, maxlod: (m1 >> 8 & 0xFF) as f32 / 16.0 }
    }
}

fn sample(t: &Tex, sp: &Sampler, uv: [f32; 2], grad: f32) -> [i32; 4] {
    let lod = ((grad * t.w.max(t.h) as f32).max(1e-6).log2() + sp.bias).clamp(sp.minlod, sp.maxlod);
    let (lin, level) = if lod <= 0.0 { (sp.mag_lin, 0usize) } else {
        let lin = sp.min & 4 != 0 || sp.min == 1;
        let lv = if sp.min & 3 != 0 && t.levels.len() > 1 { (lod.round() as usize).min(t.levels.len() - 1) } else { 0 };
        (lin, lv)
    };
    let lw = (t.w >> level).max(1);
    let lh = (t.h >> level).max(1);
    let data = &t.levels[level];
    let fetch = |x: i32, y: i32| -> [i32; 4] {
        let x = wrap(x, lw as i32, sp.ws) as usize;
        let y = wrap(y, lh as i32, sp.wt) as usize;
        let p = &data[(y * lw + x) * 4..(y * lw + x) * 4 + 4];
        [p[0] as i32, p[1] as i32, p[2] as i32, p[3] as i32]
    };
    let fu = uv[0] * lw as f32;
    let fv = uv[1] * lh as f32;
    if !lin { return fetch(fu.floor() as i32, fv.floor() as i32); }
    let u = (fu * 128.0).round() as i32 - 64;
    let v = (fv * 128.0).round() as i32 - 64;
    let (x0, y0, fx, fy) = (u >> 7, v >> 7, u & 127, v & 127);
    let (a, b, c, d) = (fetch(x0, y0), fetch(x0 + 1, y0), fetch(x0, y0 + 1), fetch(x0 + 1, y0 + 1));
    let mut o = [0; 4];
    for k in 0..4 {
        let top = a[k] * (128 - fx) + b[k] * fx;
        let bot = c[k] * (128 - fx) + d[k] * fx;
        o[k] = (top * (128 - fy) + bot * fy) >> 14;
    }
    o
}

/// One triangle into a band of rows [y0, y0 + n).
fn raster(p: &Prep, t: &Tri, color: &mut [[u8; 4]], depth: &mut [u32], y0: usize, n: usize) {
    let [a, b, c] = &t.v;
    let area = t.area;
    let ys = (t.bbox[1].max(y0 as i32)) as usize;
    let ye = (t.bbox[3].min((y0 + n) as i32 - 1)) as i32;
    if (ys as i32) > ye { return; }
    let edge = |p0: &SV, q: &SV, x: f32, y: f32| (q.x - p0.x) * (y - p0.y) - (q.y - p0.y) * (x - p0.x);
    let owns = |p0: &SV, q: &SV| { let dy = q.y - p0.y; let dx = q.x - p0.x; (dy == 0.0 && dx < 0.0) || dy > 0.0 };
    let (o0, o1, o2) = (owns(b, c), owns(c, a), owns(a, b));
    for y in ys as i32..=ye {
        for x in t.bbox[0]..=t.bbox[2] {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let w0 = edge(b, c, px, py);
            let w1 = edge(c, a, px, py);
            let w2 = edge(a, b, px, py);
            if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 { continue; }
            if (w0 == 0.0 && !o0) || (w1 == 0.0 && !o1) || (w2 == 0.0 && !o2) { continue; }
            let (l0, l1, l2) = (w0 / area, w1 / area, w2 / area);
            let z = (l0 * a.z + l1 * b.z + l2 * c.z).clamp(0.0, 16777215.0) as u32;
            let iw = l0 * a.iw + l1 * b.iw + l2 * c.iw;
            let pc = |va: f32, vb: f32, vc: f32| (l0 * va * a.iw + l1 * vb * b.iw + l2 * vc * c.iw) / iw;
            let mut col = [[0f32; 4]; 2];
            for ch in 0..2 { for k in 0..4 { col[ch][k] = pc(a.col[ch][k], b.col[ch][k], c.col[ch][k]); } }
            let mut tc = [[0f32; 2]; 8];
            for tt in 0..p.ntex {
                let s_ = pc(a.tc[tt][0], b.tc[tt][0], c.tc[tt][0]);
                let t_ = pc(a.tc[tt][1], b.tc[tt][1], c.tc[tt][1]);
                let q = pc(a.tc[tt][2], b.tc[tt][2], c.tc[tt][2]);
                tc[tt] = if q != 0.0 { [s_ / q, t_ / q] } else { [s_, t_] };
            }
            let i = (y as usize - y0) * EFB_W + x as usize;
            pixel(p, t, &mut color[i], &mut depth[i], z, col, tc);
        }
    }
}

fn pixel(p: &Prep, t: &Tri, color: &mut [u8; 4], depth: &mut u32, z: u32, col: [[f32; 4]; 2], tc: [[f32; 2]; 8]) {
    let zt = |dz: u32| p.zmode & 1 == 0 || zfunc((p.zmode >> 1) & 7, z, dz);
    if p.early && !zt(*depth) { return; }
    let ras = [col[0].map(|v| v.clamp(0.0, 255.0) as i32), col[1].map(|v| v.clamp(0.0, 255.0) as i32)];
    let out = tev(p, t, &ras, &tc);
    if !alpha_test(p.afunc, out[3]) { return; }
    if !p.early && !zt(*depth) { return; }
    if p.zmode & 1 != 0 && p.zmode >> 4 & 1 != 0 { *depth = z; }
    blend(p, color, [out[0] as u8, out[1] as u8, out[2] as u8, out[3] as u8]);
}

fn blend(p: &Prep, dstp: &mut [u8; 4], src: [u8; 4]) {
    let cm = p.cmode;
    let dst = *dstp;
    let mut o = src;
    if cm & 1 != 0 {
        if cm >> 11 & 1 != 0 {
            for k in 0..3 { o[k] = (dst[k] as i32 - src[k] as i32).max(0) as u8; }
        } else {
            let sf = factor((cm >> 8) & 7, src, dst, true);
            let df = factor((cm >> 5) & 7, src, dst, false);
            for k in 0..4 {
                let v = (src[k] as u32 * (sf[k] + (sf[k] >> 7)) + dst[k] as u32 * (df[k] + (df[k] >> 7))) >> 8;
                o[k] = v.min(255) as u8;
            }
        }
    } else if cm >> 1 & 1 != 0 {
        let op = (cm >> 12) & 0xF;
        for k in 0..4 { o[k] = logic(op, src[k], dst[k]); }
    }
    // RGB8_Z24 has no alpha; RGBA6_Z24 keeps 6 bits per channel
    if p.pixfmt == 1 { for k in 0..4 { o[k] = (o[k] & 0xFC) | (o[k] >> 6); } }
    let mut w = dst;
    if cm >> 3 & 1 != 0 { w[0] = o[0]; w[1] = o[1]; w[2] = o[2]; }
    if cm >> 4 & 1 != 0 { w[3] = o[3]; }
    if p.dstalpha >> 8 & 1 != 0 { w[3] = p.dstalpha as u8; }
    if p.pixfmt == 0 { w[3] = 255; }
    *dstp = w;
}

fn tev(p: &Prep, t: &Tri, ras: &[[i32; 4]; 2], tc: &[[f32; 2]; 8]) -> [i32; 4] {
    let mut reg = p.regs; // PREV, C0, C1, C2 (signed 11-bit)
    for st in &p.stages {
        let rc = match st.chan { 0 => ras[0], 1 => ras[1], _ => [0; 4] };
        let rc = [rc[st.rs[0]], rc[st.rs[1]], rc[st.rs[2]], rc[st.rs[3]]];
        let tx = if st.tex_on {
            match &p.tex[st.map] {
                Some(tx) => { let v = sample(tx, &p.samp[st.map], tc[st.coord], t.grads[st.coord]); [v[st.ts[0]], v[st.ts[1]], v[st.ts[2]], v[st.ts[3]]] }
                None => [255, 0, 255, 255],
            }
        } else { [255; 4] };
        let (kc, ka) = (st.kc, st.ka);
        let ci = |k: u32, reg: &[[i32; 4]; 4]| -> [i32; 3] {
            match k {
                0 => [reg[0][0], reg[0][1], reg[0][2]], 1 => [reg[0][3]; 3],
                2 => [reg[1][0], reg[1][1], reg[1][2]], 3 => [reg[1][3]; 3],
                4 => [reg[2][0], reg[2][1], reg[2][2]], 5 => [reg[2][3]; 3],
                6 => [reg[3][0], reg[3][1], reg[3][2]], 7 => [reg[3][3]; 3],
                8 => [tx[0], tx[1], tx[2]], 9 => [tx[3]; 3],
                10 => [rc[0], rc[1], rc[2]], 11 => [rc[3]; 3],
                12 => [255; 3], 13 => [128; 3], 14 => kc, _ => [0; 3],
            }
        };
        let ai = |k: u32, reg: &[[i32; 4]; 4]| -> i32 {
            match k { 0 => reg[0][3], 1 => reg[1][3], 2 => reg[2][3], 3 => reg[3][3], 4 => tx[3], 5 => rc[3], 6 => ka, _ => 0 }
        };
        let (cenv, aenv) = (st.cenv, st.aenv);
        let (a, b, c, d) = (ci(cenv >> 12 & 0xF, &reg), ci(cenv >> 8 & 0xF, &reg), ci(cenv >> 4 & 0xF, &reg), ci(cenv & 0xF, &reg));
        let (aa, ab, ac, ad) = (ai(aenv >> 13 & 7, &reg), ai(aenv >> 10 & 7, &reg), ai(aenv >> 7 & 7, &reg), ai(aenv >> 4 & 7, &reg));
        let mut oc = [0i32; 3];
        if (cenv >> 16 & 3) == 3 { oc = compare_color(a, b, c, d, cenv); } else {
            for k in 0..3 { oc[k] = combine(a[k], b[k], c[k], d[k], cenv, false, None, k); }
        }
        let oa = if (aenv >> 16 & 3) == 3 { compare_alpha(aa, ab, ac, ad, aenv, [a, b]) } else { combine(aa, ab, ac, ad, aenv, true, None, 0) };
        let cd = (cenv >> 22 & 3) as usize;
        let adst = (aenv >> 22 & 3) as usize;
        reg[cd][0] = oc[0]; reg[cd][1] = oc[1]; reg[cd][2] = oc[2];
        reg[adst][3] = oa;
    }
    let last = p.stages.last().unwrap();
    let cd = (last.cenv >> 22 & 3) as usize;
    let ad = (last.aenv >> 22 & 3) as usize;
    [reg[cd][0].clamp(0, 255), reg[cd][1].clamp(0, 255), reg[cd][2].clamp(0, 255), reg[ad][3].clamp(0, 255)]
}

fn konst(s: &State, k: usize) -> [i32; 4] {
    let [lo, hi] = s.kreg[k];
    [(lo & 0xFF) as i32, (hi >> 12 & 0xFF) as i32, (hi & 0xFF) as i32, (lo >> 12 & 0xFF) as i32]
}

pub(crate) fn konst_color(s: &State, sel: u32) -> [i32; 3] {
    match sel {
        0..=7 => [[255, 223, 191, 159, 128, 96, 64, 32][sel as usize]; 3],
        0x0C..=0x0F => { let k = konst(s, (sel - 0x0C) as usize); [k[0], k[1], k[2]] }
        0x10..=0x1F => { let k = konst(s, ((sel - 0x10) & 3) as usize); [k[((sel - 0x10) >> 2) as usize]; 3] }
        _ => [0; 3],
    }
}

pub(crate) fn konst_alpha(s: &State, sel: u32) -> i32 {
    match sel {
        0..=7 => [255, 223, 191, 159, 128, 96, 64, 32][sel as usize],
        0x10..=0x1F => konst(s, ((sel - 0x10) & 3) as usize)[((sel - 0x10) >> 2) as usize],
        _ => 0,
    }
}

/// Swap table `t` (BP 0xF6 + 2t / + 2t + 1, low 4 bits each): output channel -> input channel.
pub(crate) fn swap(s: &State, t: u32) -> [usize; 4] {
    let a = s.bp[0xF6 + 2 * t as usize];
    let b = s.bp[0xF7 + 2 * t as usize];
    [(a & 3) as usize, (a >> 2 & 3) as usize, (b & 3) as usize, (b >> 2 & 3) as usize]
}

/// One TEV channel (Dolphin's `TevLerp` arithmetic: c extended to 0..256, bias on d, scale inside, rounding).
fn combine(a: i32, b: i32, c: i32, d: i32, env: u32, alpha: bool, _ab: Option<([i32; 3], [i32; 3])>, _k: usize) -> i32 {
    let (a, b, c) = (a & 255, b & 255, c & 255);
    let c = c + (c >> 7);
    let bias = env >> 16 & 3;
    let mut d = d + match bias { 1 => 128, 2 => -128, _ => 0 };
    let shift = (env >> 20 & 3) as i32;
    let mut lerp = (a << 8) + (b - a) * c;
    if shift != 3 { lerp <<= shift; d <<= shift; }
    let sub = env >> 18 & 1 != 0;
    if (shift == 3) == alpha { lerp += if sub { 127 } else { 128 }; }
    let mut r = lerp >> 8;
    r = if sub { d - r } else { d + r };
    if shift == 3 { r >>= 1; }
    if env >> 19 & 1 != 0 { r.clamp(0, 255) } else { r.clamp(-1024, 1023) }
}

fn compare_color(a: [i32; 3], b: [i32; 3], c: [i32; 3], d: [i32; 3], env: u32) -> [i32; 3] {
    let mode = (env >> 18 & 1) | ((env >> 20 & 3) << 1);
    let pack = |x: [i32; 3], n: usize| -> i32 { (0..n).fold(0, |acc, k| acc | ((x[k] & 255) << (8 * k))) };
    let mut o = [0; 3];
    for k in 0..3 {
        let hit = match mode {
            0 => pack(a, 1) > pack(b, 1), 1 => pack(a, 1) == pack(b, 1),
            2 => pack(a, 2) > pack(b, 2), 3 => pack(a, 2) == pack(b, 2),
            4 => pack(a, 3) > pack(b, 3), 5 => pack(a, 3) == pack(b, 3),
            6 => (a[k] & 255) > (b[k] & 255), _ => (a[k] & 255) == (b[k] & 255),
        };
        o[k] = (d[k] + if hit { c[k] & 255 } else { 0 }).clamp(if env >> 19 & 1 != 0 { 0 } else { -1024 }, if env >> 19 & 1 != 0 { 255 } else { 1023 });
    }
    o
}

fn compare_alpha(a: i32, b: i32, c: i32, d: i32, env: u32, rgb: [[i32; 3]; 2]) -> i32 {
    let mode = (env >> 18 & 1) | ((env >> 20 & 3) << 1);
    let pack = |x: [i32; 3], n: usize| -> i32 { (0..n).fold(0, |acc, k| acc | ((x[k] & 255) << (8 * k))) };
    let hit = match mode {
        0 => pack(rgb[0], 1) > pack(rgb[1], 1), 1 => pack(rgb[0], 1) == pack(rgb[1], 1),
        2 => pack(rgb[0], 2) > pack(rgb[1], 2), 3 => pack(rgb[0], 2) == pack(rgb[1], 2),
        4 => pack(rgb[0], 3) > pack(rgb[1], 3), 5 => pack(rgb[0], 3) == pack(rgb[1], 3),
        6 => (a & 255) > (b & 255), _ => (a & 255) == (b & 255),
    };
    let r = d + if hit { c & 255 } else { 0 };
    if env >> 19 & 1 != 0 { r.clamp(0, 255) } else { r.clamp(-1024, 1023) }
}

fn alpha_test(r: u32, a: i32) -> bool {
    let cmp = |f: u32, rf: i32| match f { 0 => false, 1 => a < rf, 2 => a == rf, 3 => a <= rf, 4 => a > rf, 5 => a != rf, 6 => a >= rf, _ => true };
    let x = cmp(r >> 16 & 7, (r & 0xFF) as i32);
    let y = cmp(r >> 19 & 7, (r >> 8 & 0xFF) as i32);
    match r >> 22 & 3 { 0 => x && y, 1 => x || y, 2 => x != y, _ => x == y }
}

fn zfunc(f: u32, z: u32, d: u32) -> bool {
    match f { 0 => false, 1 => z < d, 2 => z == d, 3 => z <= d, 4 => z > d, 5 => z != d, 6 => z >= d, _ => true }
}

fn factor(f: u32, s: [u8; 4], d: [u8; 4], src: bool) -> [u32; 4] {
    let inv = |x: u8| 255 - x as u32;
    match f {
        0 => [0; 4],
        1 => [255; 4],
        2 => if src { d.map(|x| x as u32) } else { s.map(|x| x as u32) },
        3 => if src { d.map(inv) } else { s.map(inv) },
        4 => [s[3] as u32; 4],
        5 => [inv(s[3]); 4],
        6 => [d[3] as u32; 4],
        _ => [inv(d[3]); 4],
    }
}

fn logic(op: u32, s: u8, d: u8) -> u8 {
    match op {
        0 => 0, 1 => s & d, 2 => s & !d, 3 => s, 4 => !s & d, 5 => d, 6 => s ^ d, 7 => s | d,
        8 => !(s | d), 9 => !(s ^ d), 10 => !d, 11 => s | !d, 12 => !s, 13 => !s | d, 14 => !(s & d), _ => 255,
    }
}

pub(crate) fn wrap(x: i32, n: i32, mode: u32) -> i32 {
    match mode {
        0 => x.clamp(0, n - 1),
        1 => x.rem_euclid(n),
        _ => { let p = x.rem_euclid(2 * n); if p < n { p } else { 2 * n - 1 - p } }
    }
}

/// Lighting of one colour channel (XF 0x100E/0x100F colour ctrl, 0x1010/0x1011 alpha ctrl).
/// XF: positions to clip space, lighting into the two colour channels, texgens (with dual-tex post matrices).
pub(crate) fn transform(d: &Draw, s: &State) -> Vec<V> {
    let xm = &s.xf_mem;
    let xr = &s.xf_reg;
    let proj = [f(xr[0x20]), f(xr[0x21]), f(xr[0x22]), f(xr[0x23]), f(xr[0x24]), f(xr[0x25])];
    let ortho = xr[0x26] != 0;
    let nchan = (xr[0x09] & 3) as usize;
    let ntex = (xr[0x3F] & 0xF) as usize;
    let dual = xr[0x12] & 1 != 0;
    d.verts.iter().map(|v| {
        let m = v.pnmtx as usize * 4;
        let row = |r: usize, p: [f32; 4]| f(xm[m + 4 * r]) * p[0] + f(xm[m + 4 * r + 1]) * p[1] + f(xm[m + 4 * r + 2]) * p[2] + f(xm[m + 4 * r + 3]) * p[3];
        let p = [v.pos[0], v.pos[1], v.pos[2], 1.0];
        let e = [row(0, p), row(1, p), row(2, p)];
        let c = if ortho {
            [proj[0] * e[0] + proj[1], proj[2] * e[1] + proj[3], proj[4] * e[2] + proj[5], 1.0]
        } else {
            [proj[0] * e[0] + proj[1] * e[2], proj[2] * e[1] + proj[3] * e[2], proj[4] * e[2] + proj[5], -e[2]]
        };
        // normal: the 3x3 at 0x400 + (pnmtx % 32) * 3
        let n = v.nrm.map(|n| {
            let b = 0x400 + (v.pnmtx as usize % 32) * 3;
            let r = |k: usize| f(xm[b + 3 * k]) * n[0] + f(xm[b + 3 * k + 1]) * n[1] + f(xm[b + 3 * k + 2]) * n[2];
            let o = [r(0), r(1), r(2)];
            let l = (o[0] * o[0] + o[1] * o[1] + o[2] * o[2]).sqrt();
            if l > 0.0 { [o[0] / l, o[1] / l, o[2] / l] } else { o }
        }).unwrap_or([0.0, 0.0, 1.0]);
        // binormal and tangent by the same 3x3, not normalized (as Dolphin)
        let nbt = v.nbt.map(|bt| bt.map(|n| {
            let b = 0x400 + (v.pnmtx as usize % 32) * 3;
            let r = |k: usize| f(xm[b + 3 * k]) * n[0] + f(xm[b + 3 * k + 1]) * n[1] + f(xm[b + 3 * k + 2]) * n[2];
            [r(0), r(1), r(2)]
        }));
        let mut out = V { c, ..V::default() };
        for ch in 0..2 {
            let vc = v.clr[ch].map(|c| c.map(|x| x as f32)).unwrap_or([255.0; 4]);
            out.col[ch] = if ch < nchan { light(s, ch, vc, e, n) } else { [0.0; 4] };
        }
        for t in 0..ntex {
            let info = xr[0x40 + t];
            let src_row = (info >> 7) & 0x1F;
            let src: [f32; 4] = match src_row {
                0 => [v.pos[0], v.pos[1], v.pos[2], 1.0],
                1 => { let n = v.nrm.unwrap_or([0.0; 3]); [n[0], n[1], n[2], 1.0] }
                3 | 4 => { let n = v.nbt.map(|b| b[(src_row - 3) as usize]).unwrap_or([0.0; 3]); [n[0], n[1], n[2], 1.0] }
                5..=12 => { let tc = v.tex[(src_row - 5) as usize].unwrap_or([0.0; 2]); [tc[0], tc[1], 1.0, 1.0] }
                _ => [0.0, 0.0, 1.0, 1.0],
            };
            let src = if info >> 2 & 1 == 0 { [src[0], src[1], 1.0, 1.0] } else { src };
            let ty = (info >> 4) & 7;
            let mut tc = match ty {
                0 => {
                    let tm = v.texmtx[t] as usize * 4;
                    let r = |k: usize| f(xm[tm + 4 * k]) * src[0] + f(xm[tm + 4 * k + 1]) * src[1] + f(xm[tm + 4 * k + 2]) * src[2] + f(xm[tm + 4 * k + 3]) * src[3];
                    if info >> 1 & 1 != 0 { [r(0), r(1), r(2)] } else { [r(0), r(1), 1.0] }
                }
                // emboss: an earlier texgen's coordinates shifted by the light direction in the binormal/tangent
                // frame (info bits 12-14 source texgen, 15-17 light)
                1 => {
                    let src = out.tc[(info >> 12 & 7) as usize];
                    match nbt {
                        Some([b, t]) => {
                            let lb = 0x600 + 0x10 * (info >> 15 & 7) as usize;
                            let l = [f(xm[lb + 10]) - e[0], f(xm[lb + 11]) - e[1], f(xm[lb + 12]) - e[2]];
                            let len = (l[0] * l[0] + l[1] * l[1] + l[2] * l[2]).sqrt();
                            let l = if len > 0.0 { [l[0] / len, l[1] / len, l[2] / len] } else { l };
                            let dot = |a: [f32; 3]| a[0] * l[0] + a[1] * l[1] + a[2] * l[2];
                            [src[0] + dot(b), src[1] + dot(t), src[2]]
                        }
                        None => src,
                    }
                }
                2 | 3 => { let c = out.col[(ty - 2) as usize]; [c[0] / 255.0, c[1] / 255.0, 1.0] }
                _ => [0.0, 0.0, 1.0],
            };
            if dual && ty == 0 {
                let post = xr[0x50 + t];
                if post >> 8 & 1 != 0 {
                    let l = (tc[0] * tc[0] + tc[1] * tc[1] + tc[2] * tc[2]).sqrt();
                    if l > 0.0 { tc = [tc[0] / l, tc[1] / l, tc[2] / l]; }
                }
                let pm = 0x500 + (post & 0x3F) as usize * 4;
                let r = |k: usize| f(xm[pm + 4 * k]) * tc[0] + f(xm[pm + 4 * k + 1]) * tc[1] + f(xm[pm + 4 * k + 2]) * tc[2] + f(xm[pm + 4 * k + 3]);
                tc = [r(0), r(1), r(2)];
            }
            out.tc[t] = tc;
        }
        out
    }).collect()
}


fn light(s: &State, ch: usize, vc: [f32; 4], pos: [f32; 3], n: [f32; 3]) -> [f32; 4] {
    let xr = &s.xf_reg;
    let mut out = [0f32; 4];
    for (k, ctrl) in [(0usize, xr[0x0E + ch]), (1, xr[0x10 + ch])] {
        let mat = if ctrl & 1 != 0 { vc } else { rgba(xr[0x0C + ch]) };
        let amb = if ctrl >> 6 & 1 != 0 { vc } else { rgba(xr[0x0A + ch]) };
        let comps: &[usize] = if k == 0 { &[0, 1, 2] } else { &[3] };
        if ctrl >> 1 & 1 == 0 {
            for &c in comps { out[c] = mat[c]; }
            continue;
        }
        let mut acc = [0f32; 4];
        for &c in comps { acc[c] = amb[c]; }
        let mask = (ctrl >> 2 & 0xF) | ((ctrl >> 11 & 0xF) << 4);
        for l in 0..8 {
            if mask >> l & 1 == 0 { continue; }
            let b = 0x600 + 0x10 * l;
            let xm = &s.xf_mem;
            let lc = rgba(xm[b + 3]);
            let ca = [f(xm[b + 4]), f(xm[b + 5]), f(xm[b + 6])];
            let ka = [f(xm[b + 7]), f(xm[b + 8]), f(xm[b + 9])];
            let lp = [f(xm[b + 10]), f(xm[b + 11]), f(xm[b + 12])];
            let ld = [f(xm[b + 13]), f(xm[b + 14]), f(xm[b + 15])];
            let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
            let diff_fn = ctrl >> 7 & 3;
            // attenuation function (bit 9 | bit 10 << 1, as GXSetChanCtrl writes it: bit 9 = not NONE, bit 10 = not
            // SPEC): 1 specular, 3 spot, 0/2 none
            let attn_fn = (ctrl >> 9 & 1) | (ctrl >> 10 & 1) << 1;
            let (dir, attn) = if attn_fn == 3 {
                let v = [lp[0] - pos[0], lp[1] - pos[1], lp[2] - pos[2]];
                let d2 = dot(v, v);
                let d = d2.sqrt().max(1e-20);
                let dir = [v[0] / d, v[1] / d, v[2] / d];
                let cs = dot(dir, ld).max(0.0);
                let cosatt = (ca[0] + ca[1] * cs + ca[2] * cs * cs).max(0.0);
                let distatt = ka[0] + ka[1] * d + ka[2] * d2;
                (dir, if distatt != 0.0 { cosatt / distatt } else { 0.0 })
            } else if attn_fn == 1 {
                // specular (as Dolphin): the light direction register holds the half-angle vector; the attenuation
                // polynomials run on n.h, the distance one normalized unless the diffuse function is none
                let v = [lp[0] - pos[0], lp[1] - pos[1], lp[2] - pos[2]];
                let l = dot(v, v).sqrt().max(1e-20);
                let dir = [v[0] / l, v[1] / l, v[2] / l];
                let a = if dot(n, dir) >= 0.0 { dot(n, ld).max(0.0) } else { 0.0 };
                let k = if diff_fn == 0 { ka } else { let m = dot(ka, ka).sqrt(); if m > 0.0 { [ka[0] / m, ka[1] / m, ka[2] / m] } else { ka } };
                let cosatt = (ca[0] + ca[1] * a + ca[2] * a * a).max(0.0);
                let distatt = k[0] + k[1] * a + k[2] * a * a;
                (dir, if distatt != 0.0 { cosatt / distatt } else { 0.0 })
            } else {
                let v = [lp[0] - pos[0], lp[1] - pos[1], lp[2] - pos[2]];
                let l = dot(v, v).sqrt();
                (if l > 0.0 { [v[0] / l, v[1] / l, v[2] / l] } else { n }, 1.0)
            };
            let nd = dot(dir, n);
            let diff = match diff_fn { 0 => 1.0, 1 => nd, _ => nd.max(0.0) };
            for &c in comps { acc[c] += attn * diff * lc[c]; }
        }
        for &c in comps { out[c] = (mat[c] * acc[c].clamp(0.0, 255.0) / 255.0).clamp(0.0, 255.0); }
    }
    out
}

fn clip(mut p: Vec<V>) -> Vec<V> {
    // -w <= z <= 0, and w > 0
    let planes: [fn(&V) -> f32; 3] = [|v| v.c[2] + v.c[3], |v| -v.c[2], |v| v.c[3] - 1e-5];
    for pl in planes {
        if p.is_empty() { break; }
        let mut o = Vec::with_capacity(p.len() + 2);
        for i in 0..p.len() {
            let (a, b) = (p[i], p[(i + 1) % p.len()]);
            let (da, db) = (pl(&a), pl(&b));
            if da >= 0.0 { o.push(a); }
            if (da >= 0.0) != (db >= 0.0) {
                let t = da / (da - db);
                o.push(lerp_v(&a, &b, t));
            }
        }
        p = o;
    }
    p
}

fn lerp_v(a: &V, b: &V, t: f32) -> V {
    let mut o = *a;
    for k in 0..4 { o.c[k] = a.c[k] + (b.c[k] - a.c[k]) * t; }
    for ch in 0..2 { for k in 0..4 { o.col[ch][k] = a.col[ch][k] + (b.col[ch][k] - a.col[ch][k]) * t; } }
    for tt in 0..8 { for k in 0..3 { o.tc[tt][k] = a.tc[tt][k] + (b.tc[tt][k] - a.tc[tt][k]) * t; } }
    o
}

fn to_screen(v: &V, s: &State) -> SV {
    let xr = &s.xf_reg;
    let (wd, ht, zr, xo, yo, fz) = (f(xr[0x1A]), f(xr[0x1B]), f(xr[0x1C]), f(xr[0x1D]), f(xr[0x1E]), f(xr[0x1F]));
    let iw = 1.0 / v.c[3];
    SV {
        x: v.c[0] * iw * wd + xo - 342.0,
        y: v.c[1] * iw * ht + yo - 342.0,
        z: fz + v.c[2] * iw * zr,
        iw,
        col: v.col,
        tc: v.tc,
    }
}

/// Screen-space size of one pixel step in texture coordinates (for the LOD), per triangle.
fn tex_grad(a: &SV, b: &SV, c: &SV, t: usize, area: f32) -> f32 {
    let uv = |p: &SV| { let q = if p.tc[t][2] != 0.0 { p.tc[t][2] } else { 1.0 }; [p.tc[t][0] / q, p.tc[t][1] / q] };
    let (ua, ub, uc) = (uv(a), uv(b), uv(c));
    let mut g = 0f32;
    for k in 0..2 {
        let dx = ((ub[k] - ua[k]) * (c.y - a.y) - (uc[k] - ua[k]) * (b.y - a.y)) / area;
        let dy = ((uc[k] - ua[k]) * (b.x - a.x) - (ub[k] - ua[k]) * (c.x - a.x)) / area;
        g = g.max(dx.abs()).max(dy.abs());
    }
    g
}

/// EFB copy encoding into a GX texture format (rows of blocks, `stride` bytes per block row).
fn encode(img: &[[u8; 4]], w: usize, h: usize, fmt: u32, intensity: bool, stride: usize) -> Option<Vec<u8>> {
    let lum = |p: [u8; 4]| (((66 * p[0] as i32 + 129 * p[1] as i32 + 25 * p[2] as i32 + 128) >> 8) + 16).clamp(0, 255) as u8;
    let (bw, bh, bpp) = match fmt { 0 => (8, 8, 4), 1 | 2 | 7 | 8 | 9 | 10 => (8, 4, 8), 3 | 4 | 5 | 11 | 12 => (4, 4, 16), 6 => (4, 4, 32), _ => return None };
    let bx = w.div_ceil(bw);
    let by = h.div_ceil(bh);
    let block = bw * bh * bpp / 8;
    let stride = if stride == 0 { bx * block } else { stride };
    let mut out = vec![0u8; stride * by];
    let get = |x: usize, y: usize| img[y.min(h - 1) * w + x.min(w - 1)];
    for yb in 0..by {
        for xb in 0..bx {
            let o = yb * stride + xb * block;
            for p in 0..bw * bh {
                let (x, y) = (xb * bw + p % bw, yb * bh + p / bw);
                let c = get(x, y);
                let i = if intensity { lum(c) } else { c[0] };
                match fmt {
                    0 => { let n = i >> 4; out[o + p / 2] |= if p % 2 == 0 { n << 4 } else { n }; }
                    1 | 8 => out[o + p] = i,
                    2 => out[o + p] = (c[3] & 0xF0) | (i >> 4),
                    7 => out[o + p] = c[3],
                    9 => out[o + p] = c[1],
                    10 => out[o + p] = c[2],
                    3 => { out[o + 2 * p] = c[3]; out[o + 2 * p + 1] = i; }
                    11 => { out[o + 2 * p] = c[0]; out[o + 2 * p + 1] = c[1]; }
                    12 => { out[o + 2 * p] = c[1]; out[o + 2 * p + 1] = c[2]; }
                    4 => {
                        let v = ((c[0] as u16 >> 3) << 11) | ((c[1] as u16 >> 2) << 5) | (c[2] as u16 >> 3);
                        out[o + 2 * p..o + 2 * p + 2].copy_from_slice(&v.to_be_bytes());
                    }
                    5 => {
                        let v = if c[3] >= 0xE0 {
                            0x8000 | ((c[0] as u16 >> 3) << 10) | ((c[1] as u16 >> 3) << 5) | (c[2] as u16 >> 3)
                        } else {
                            ((c[3] as u16 >> 5) << 12) | ((c[0] as u16 >> 4) << 8) | ((c[1] as u16 >> 4) << 4) | (c[2] as u16 >> 4)
                        };
                        out[o + 2 * p..o + 2 * p + 2].copy_from_slice(&v.to_be_bytes());
                    }
                    6 => { out[o + 2 * p] = c[3]; out[o + 2 * p + 1] = c[0]; out[o + 32 + 2 * p] = c[1]; out[o + 32 + 2 * p + 1] = c[2]; }
                    _ => {}
                }
            }
        }
    }
    Some(out)
}
