//! `.ccd` collision files (per room). Layout per GoD Tool `XGCollisionData.swift`.
//!
//! Header: u32 group list offset, u32 group count. Groups are 0x40 bytes; words at +0x24..+0x3C are
//! offsets of up to 7 triangle lists `{u32 offset, u32 count}`. Triangles are 0x34 bytes: three vertices
//! (f32 xyz), a face normal, then two u16s: (surface, index); list 3 (+0x30) stores them swapped.
//! Verified visually against the map model:
//! - slot 0 (+0x24): walkable floor (surface values look like flags: 0x0F00, 0xF000, 0xFF00, ...)
//! - slot 1 (+0x28): walls (surface 1..7)
//! - slot 2 (+0x2C): pass/touch interaction regions; index = region id used by interaction points
//! - slot 3 (+0x30): check (A-button) interaction regions
//! - slot 4 (+0x34): same triangles as slot 1 (`fn_8010F188` tests it instead of slot 1)
//! - slot 5 (+0x38): sun occluders for the lens flare (`GScolsys2Sun` 0x80111C24); 0x30-byte triangles
//!   (vertices + normal, no u16s), no grid
//!
//! Each list header (`_offsetCCD` 0x8010CE04): u32 triangle offset, u32 count, then (not for slot 3) a
//! spatial grid: u32 cell table offset ({u32 first index, u32 count} per cell), u32 index list offset,
//! u16 cells x, u16 cells z, f32 cell size x, f32 cell size z, f32 origin x, f32 origin z. Group header
//! +0x00 position, +0x0C rotation, +0x18 scale (moving objects), +0x3C u16 flags (bit 0: transformed).

use crate::be::u32_at;
use crate::Result;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertices: [[f32; 3]; 3],
    pub normal: [f32; 3],
    pub surface: u16,
    /// Region index (interaction lists) or the second u16.
    pub index: u16,
    /// The raw u16s at +0x30 / +0x32 (wall edge flags, floor attribute nibbles, region id at +0x32,
    /// check id at +0x30).
    pub raw: [u16; 2],
}

/// Spatial grid of a triangle list (`fn_8010E53C` walks the cells around a point).
#[derive(Debug, Clone)]
pub struct Grid {
    pub nx: usize,
    pub nz: usize,
    pub cell_x: f32,
    pub cell_z: f32,
    pub origin_x: f32,
    pub origin_z: f32,
    /// (first index, count) into `indices`, row-major (x fastest).
    pub cells: Vec<(u32, u32)>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct TriList {
    /// 0..=6 for group offsets 0x24..0x3C
    pub slot: usize,
    pub interactable: bool,
    pub triangles: Vec<Triangle>,
    pub grid: Option<Grid>,
}

#[derive(Debug, Clone, Default)]
pub struct Collision {
    pub groups: Vec<Vec<TriList>>,
    /// Per group: +0x3C flags (bit 0: transformed object).
    pub group_flags: Vec<u16>,
}

impl Collision {
    pub fn parse(d: &[u8]) -> Result<Self> {
        let list_start = u32_at(d, 0, "ccd header")? as usize;
        let count = u32_at(d, 4, "ccd header")? as usize;
        let f = |o: usize| -> Result<f32> { Ok(f32::from_bits(u32_at(d, o, "ccd triangle")?)) };
        let mut groups = Vec::with_capacity(count);
        let mut group_flags = Vec::with_capacity(count);
        for g in 0..count {
            let base = list_start + 0x40 * g;
            group_flags.push(crate::be::u16_at(d, base + 0x3C, "ccd group")?);
            let mut lists = vec![];
            for slot in 0..6 {
                let lo = u32_at(d, base + 0x24 + 4 * slot, "ccd group")? as usize;
                if lo == 0 { continue; }
                let start = u32_at(d, lo, "ccd list")? as usize;
                let n = u32_at(d, lo + 4, "ccd list")? as usize;
                let mut tris = Vec::with_capacity(n);
                // slot 5 (+0x38, sun occlusion, `GScolsys2Sun`): 0x30-byte triangles with no u16s
                let stride = if slot == 5 { 0x30 } else { 0x34 };
                for t in 0..n {
                    let o = start + t * stride;
                    let mut v = [[0.0f32; 3]; 3];
                    for (c, vv) in v.iter_mut().enumerate() { *vv = [f(o + c * 12)?, f(o + c * 12 + 4)?, f(o + c * 12 + 8)?]; }
                    let (a, b) = if slot == 5 { (0, 0) } else {
                        (crate::be::u16_at(d, o + 0x30, "ccd triangle")?, crate::be::u16_at(d, o + 0x32, "ccd triangle")?)
                    };
                    let (surface, index) = if slot == 3 { (b, a) } else { (a, b) };
                    tris.push(Triangle { vertices: v, normal: [f(o + 36)?, f(o + 40)?, f(o + 44)?], surface, index, raw: [a, b] });
                }
                let grid = if slot == 3 || slot == 5 { None } else {
                    let co = u32_at(d, lo + 8, "ccd grid")? as usize;
                    let io = u32_at(d, lo + 12, "ccd grid")? as usize;
                    let nx = crate::be::u16_at(d, lo + 16, "ccd grid")? as usize;
                    let nz = crate::be::u16_at(d, lo + 18, "ccd grid")? as usize;
                    let cells = (0..nx * nz).map(|c| Ok((u32_at(d, co + 8 * c, "ccd cell")?, u32_at(d, co + 8 * c + 4, "ccd cell")?)))
                        .collect::<Result<Vec<_>>>()?;
                    let total = cells.iter().map(|c| (c.0 + c.1) as usize).max().unwrap_or(0);
                    let indices = (0..total).map(|i| u32_at(d, io + 4 * i, "ccd index")).collect::<Result<Vec<_>>>()?;
                    Some(Grid { nx, nz, cell_x: f(lo + 20)?, cell_z: f(lo + 24)?, origin_x: f(lo + 28)?, origin_z: f(lo + 32)?, cells, indices })
                };
                lists.push(TriList { slot, interactable: slot == 2 || slot == 3, triangles: tris, grid });
            }
            groups.push(lists);
        }
        Ok(Collision { groups, group_flags })
    }

    pub fn triangles(&self) -> impl Iterator<Item = (&TriList, &Triangle)> {
        self.groups.iter().flatten().flat_map(|l| l.triangles.iter().map(move |t| (l, t)))
    }
}
