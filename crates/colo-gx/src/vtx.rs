//! Vertex formats: the vertex descriptor (CP 0x50 / 0x60) says which attributes a vertex has and whether each
//! is direct or indexed (8/16-bit index into the array at CP 0xA0+n with stride CP 0xB0+n); the vertex
//! attribute table (CP 0x70/0x80/0x90 + VAT index) gives each attribute's component count, type and fixed-point
//! shift.

use crate::GpuMem;

#[derive(Clone, Debug, Default)]
pub struct Vertex {
    pub pnmtx: u8,
    pub texmtx: [u8; 8],
    pub pos: [f32; 3],
    pub nrm: Option<[f32; 3]>,
    pub clr: [Option<[u8; 4]>; 2],
    pub tex: [Option<[f32; 2]>; 8],
}

#[derive(Clone, Copy, Debug)]
struct Attr { kind: u32, cnt: u32, fmt: u32, shift: u32 }

#[derive(Clone, Debug)]
pub struct Format {
    pnmtx: bool,
    texmtx: [bool; 8],
    pos: Attr,
    nrm: Attr,
    nrm_index3: bool,
    clr: [Attr; 2],
    tex: [Attr; 8],
}

fn comp_size(fmt: u32) -> usize { match fmt { 0 | 1 => 1, 2 | 3 => 2, _ => 4 } }
fn clr_size(fmt: u32) -> usize { match fmt { 0 | 3 => 2, 1 | 4 => 3, _ => 4 } }

impl Format {
    pub fn new(cp: &[u32; 256], vat: usize) -> Format {
        let lo = cp[0x50];
        let hi = cp[0x60];
        let a = cp[0x70 + vat];
        let b = cp[0x80 + vat];
        let c = cp[0x90 + vat];
        let at = |kind, cnt, fmt, shift| Attr { kind, cnt, fmt, shift };
        let tex_vat = [
            (a >> 21 & 1, a >> 22 & 7, a >> 25 & 0x1F),
            (b & 1, b >> 1 & 7, b >> 4 & 0x1F),
            (b >> 9 & 1, b >> 10 & 7, b >> 13 & 0x1F),
            (b >> 18 & 1, b >> 19 & 7, b >> 22 & 0x1F),
            (b >> 27 & 1, b >> 28 & 7, c & 0x1F),
            (c >> 5 & 1, c >> 6 & 7, c >> 9 & 0x1F),
            (c >> 14 & 1, c >> 15 & 7, c >> 18 & 0x1F),
            (c >> 23 & 1, c >> 24 & 7, c >> 27 & 0x1F),
        ];
        let mut tex = [at(0, 0, 0, 0); 8];
        for t in 0..8 { tex[t] = at(hi >> (2 * t) & 3, tex_vat[t].0, tex_vat[t].1, tex_vat[t].2); }
        let mut texmtx = [false; 8];
        for t in 0..8 { texmtx[t] = lo >> (1 + t) & 1 != 0; }
        Format {
            pnmtx: lo & 1 != 0,
            texmtx,
            pos: at(lo >> 9 & 3, a & 1, a >> 1 & 7, a >> 4 & 0x1F),
            nrm: at(lo >> 11 & 3, a >> 9 & 1, a >> 10 & 7, 0),
            nrm_index3: a >> 31 & 1 != 0,
            clr: [at(lo >> 13 & 3, a >> 13 & 1, a >> 14 & 7, 0), at(lo >> 15 & 3, a >> 17 & 1, a >> 18 & 7, 0)],
            tex,
        }
    }

    fn pos_comps(&self) -> usize { if self.pos.cnt == 1 { 3 } else { 2 } }
    fn nrm_comps(&self) -> usize { if self.nrm.cnt == 1 { 9 } else { 3 } }
    fn tex_comps(t: &Attr) -> usize { if t.cnt == 1 { 2 } else { 1 } }

    fn attr_size(kind: u32, direct: usize) -> usize { match kind { 0 => 0, 1 => direct, 2 => 1, _ => 2 } }

    /// Bytes per vertex in the stream.
    pub fn size(&self) -> usize {
        let mut n = self.pnmtx as usize + self.texmtx.iter().filter(|x| **x).count();
        n += Self::attr_size(self.pos.kind, self.pos_comps() * comp_size(self.pos.fmt));
        n += match self.nrm.kind {
            0 => 0,
            1 => self.nrm_comps() * comp_size(self.nrm.fmt),
            k => { let one = if k == 2 { 1 } else { 2 }; if self.nrm.cnt == 1 && self.nrm_index3 { 3 * one } else { one } }
        };
        for c in &self.clr { n += Self::attr_size(c.kind, clr_size(c.fmt)); }
        for t in &self.tex { n += Self::attr_size(t.kind, Self::tex_comps(t) * comp_size(t.fmt)); }
        n
    }

    fn comps(fmt: u32, shift: u32, b: &[u8], n: usize, out: &mut [f32]) {
        let sc = 1.0 / (1u32 << shift) as f32;
        for k in 0..n {
            out[k] = match fmt {
                0 => b[k] as f32 * sc,
                1 => b[k] as i8 as f32 * sc,
                2 => u16::from_be_bytes([b[2 * k], b[2 * k + 1]]) as f32 * sc,
                3 => i16::from_be_bytes([b[2 * k], b[2 * k + 1]]) as f32 * sc,
                _ => f32::from_be_bytes([b[4 * k], b[4 * k + 1], b[4 * k + 2], b[4 * k + 3]]),
            };
        }
    }

    fn color(fmt: u32, b: &[u8]) -> [u8; 4] {
        match fmt {
            0 => { let v = u16::from_be_bytes([b[0], b[1]]); let e = |x: u16, bits: u32| ((x as u32 * 255 + ((1 << bits) - 1) / 2) / ((1 << bits) - 1)) as u8; [e(v >> 11, 5), e(v >> 5 & 0x3F, 6), e(v & 0x1F, 5), 255] }
            1 | 2 => [b[0], b[1], b[2], 255],
            3 => { let v = u16::from_be_bytes([b[0], b[1]]); let e = |x: u16| (x * 0x11) as u8; [e(v >> 12), e(v >> 8 & 0xF), e(v >> 4 & 0xF), e(v & 0xF)] }
            4 => { let v = (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32; let e = |x: u32| ((x << 2) | (x >> 4)) as u8; [e(v >> 18 & 0x3F), e(v >> 12 & 0x3F), e(v >> 6 & 0x3F), e(v & 0x3F)] }
            _ => [b[0], b[1], b[2], b[3]],
        }
    }

    /// Bytes of attribute `arr` (0 pos, 1 nrm, 2/3 clr, 4.. tex) for a direct value or an index.
    fn fetch<'a>(kind: u32, d: &'a [u8], i: &mut usize, size: usize, arr: usize, cp: &[u32; 256], mem: &dyn GpuMem, buf: &'a mut Vec<u8>) -> &'a [u8] {
        match kind {
            1 => { let s = &d[*i..*i + size]; *i += size; s }
            k => {
                let idx = if k == 2 { let v = d[*i] as u32; *i += 1; v } else { let v = u16::from_be_bytes([d[*i], d[*i + 1]]) as u32; *i += 2; v };
                let base = cp[0xA0 + arr] & 0x3FF_FFFF;
                let stride = cp[0xB0 + arr] & 0xFF;
                buf.resize(size, 0);
                mem.read(base + idx * stride, buf);
                &buf[..]
            }
        }
    }

    pub fn decode(&self, d: &[u8], cp: &[u32; 256], mem: &dyn GpuMem) -> Vertex {
        let mut v = Vertex::default();
        let mut i = 0usize;
        let mut buf = Vec::new();
        if self.pnmtx { v.pnmtx = d[i]; i += 1; } else { v.pnmtx = (cp[0x30] & 0x3F) as u8; }
        for t in 0..8 {
            if self.texmtx[t] { v.texmtx[t] = d[i]; i += 1; } else {
                let m = if t < 4 { cp[0x30] >> (6 + 6 * t) } else { cp[0x40] >> (6 * (t - 4)) };
                v.texmtx[t] = (m & 0x3F) as u8;
            }
        }
        if self.pos.kind != 0 {
            let n = self.pos_comps();
            let b = Self::fetch(self.pos.kind, d, &mut i, n * comp_size(self.pos.fmt), 0, cp, mem, &mut buf);
            let mut o = [0f32; 3];
            Self::comps(self.pos.fmt, self.pos.shift, b, n, &mut o);
            v.pos = o;
        }
        if self.nrm.kind != 0 {
            let n = self.nrm_comps();
            let cs = comp_size(self.nrm.fmt);
            // normals: 1.0 = 1 << 6 (s8) or 1 << 14 (s16)
            let shift = match self.nrm.fmt { 1 => 6, 3 => 14, _ => 0 };
            if self.nrm.kind != 1 && self.nrm.cnt == 1 && self.nrm_index3 {
                // three indices (N, B, T); only N is kept
                let b = Self::fetch(self.nrm.kind, d, &mut i, 3 * cs, 1, cp, mem, &mut buf).to_vec();
                let mut o = [0f32; 3];
                Self::comps(self.nrm.fmt, shift, &b, 3, &mut o);
                v.nrm = Some(o);
                let skip = if self.nrm.kind == 2 { 2 } else { 4 };
                i += skip;
            } else {
                let b = Self::fetch(self.nrm.kind, d, &mut i, n * cs, 1, cp, mem, &mut buf);
                let mut o = [0f32; 9];
                Self::comps(self.nrm.fmt, shift, b, n, &mut o);
                v.nrm = Some([o[0], o[1], o[2]]);
            }
        }
        for k in 0..2 {
            let c = self.clr[k];
            if c.kind != 0 {
                let b = Self::fetch(c.kind, d, &mut i, clr_size(c.fmt), 2 + k, cp, mem, &mut buf);
                v.clr[k] = Some(Self::color(c.fmt, b));
            }
        }
        for t in 0..8 {
            let a = self.tex[t];
            if a.kind != 0 {
                let n = Self::tex_comps(&a);
                let b = Self::fetch(a.kind, d, &mut i, n * comp_size(a.fmt), 4 + t, cp, mem, &mut buf);
                let mut o = [0f32; 2];
                Self::comps(a.fmt, a.shift, b, n, &mut o);
                v.tex[t] = Some(o);
            }
        }
        v
    }
}
