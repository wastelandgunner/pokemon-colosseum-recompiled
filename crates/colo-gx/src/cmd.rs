//! A frame's GX work as a draw list for a graphics-card renderer (colo-gxgpu): vertices already through the XF
//! (clip-space position, lit colour channels, generated texture coordinates), the pixel pipeline state of each
//! draw decoded (TEV stages, konst/registers, alpha test, depth, blend, scissor, viewport), textures decoded to
//! RGBA8 once per content (keyed by a hash of their bytes), and EFB copies in stream order. EFB copies stay on
//! the graphics card: a later draw that samples a copy's destination address uses the copy itself (as Dolphin's
//! "EFB copies to texture only").

use crate::raster::{transform, konst_alpha, konst_color, swap};
use crate::{Frame, GpuMem, Prim, State};
use std::collections::{HashMap, HashSet};

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
pub struct SamplerDesc { pub wrap_s: u32, pub wrap_t: u32, pub mag_linear: bool, pub min: u32, pub lod_bias: f32, pub min_lod: f32, pub max_lod: f32 }

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
    /// Clear after copying: colour (r, g, b, a), z, and what the PE masks let through (colour, alpha, z).
    pub clear: Option<([u8; 4], u32, bool, bool, bool)>,
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

fn fnv(h: &mut u64, b: &[u8]) { for &x in b { *h ^= x as u64; *h = h.wrapping_mul(0x100_0000_01B3); } }

impl Builder {
    pub fn new() -> Builder { Builder { tmem: vec![0; 1 << 20], ..Default::default() } }

    pub fn build(&mut self, fr: &Frame, mem: &dyn GpuMem) -> GFrame {
        let mut g = GFrame::default();
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
                self.copy(&fr.states[st], v, &mut g);
                self.frame_tex.clear();
                ci += 1;
            }
            if let Some(d) = fr.draws.get(i) { self.draw(d, &fr.states[d.state], mem, &mut g); }
        }
        g
    }

    fn draw(&mut self, d: &crate::Draw, s: &State, mem: &dyn GpuMem, g: &mut GFrame) {
        let vs = transform(d, s);
        let n = vs.len();
        let first = g.verts.len() as u32;
        let mut push = |k: usize| {
            let v = &vs[k];
            let mut tc = [[0f32; 4]; 8];
            for t in 0..8 { tc[t] = [v.tc[t][0], v.tc[t][1], v.tc[t][2], 0.0]; }
            g.verts.push(GVertex { pos: v.c, col: v.col, tc });
        };
        match d.prim {
            Prim::Quads | Prim::Quads2 => for q in (0..n / 4 * 4).step_by(4) { for k in [q, q + 1, q + 2, q, q + 2, q + 3] { push(k); } },
            Prim::Triangles => for k in 0..n / 3 * 3 { push(k); },
            Prim::TriStrip => for k in 2..n { let t = if k % 2 == 0 { [k - 2, k - 1, k] } else { [k - 1, k - 2, k] }; for x in t { push(x); } },
            Prim::TriFan => for k in 2..n { for x in [0, k - 1, k] { push(x); } },
            _ => return,
        }
        let count = g.verts.len() as u32 - first;
        if count == 0 { return; }
        let st = self.state(s, mem, g);
        g.states.push(st);
        g.ops.push(GOp::Draw { first, count, state: g.states.len() as u32 - 1 });
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
            if order >> 6 & 1 != 0 && tex[map].is_none() {
                tex[map] = self.texture(s, mem, map, g);
                let r = |base: usize| s.bp[if map < 4 { base + map } else { base + 0x20 + map - 4 }];
                let (m0, m1) = (r(0x80), r(0x84));
                samp[map] = SamplerDesc { wrap_s: m0 & 3, wrap_t: m0 >> 2 & 3, mag_linear: m0 >> 4 & 1 != 0, min: m0 >> 5 & 7,
                    lod_bias: ((m0 >> 9 & 0xFF) as i8 as f32) / 32.0, min_lod: (m1 & 0xFF) as f32 / 16.0, max_lod: (m1 >> 8 & 0xFF) as f32 / 16.0 };
            }
        }
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
        }
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
        for b in &raw { fnv(&mut hsh, b); }
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
            fmt: (tp >> 1) | ((tp & 1) << 3), intensity: v >> 15 & 1 != 0, half: v >> 9 & 1 != 0, clear,
        };
        if to_xfb { g.xfb = Some(id); } else { self.copies.insert(dst, id); }
        g.ops.push(GOp::Copy(c));
    }
}
