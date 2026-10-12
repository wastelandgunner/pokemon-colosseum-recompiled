#![allow(clippy::needless_range_loop)]
//! Port of the game's collision system `GScolsys2` (main.dol 0x8010C650–0x80111E00) over a room's `.ccd`.
//!
//! Every collision object (ccd group) has up to five triangle lists: floors (+0x24), walls (+0x28),
//! pass/touch regions (+0x2C), check regions (+0x30) and a second wall list (+0x34). Objects can be switched
//! off by `scriptSetCol` (`GScolsys2SetObjEnable`).
//!
//! - Floors (`fn_8010E138`): every floor triangle under (x, z) in the point's grid cell gives a height
//!   (`getCpPolyVec` 0x8010DEF0, a 2D point-in-triangle test with one winding).
//! - Walls (`fn_8010F320` → `fn_8010EFE4` → `fn_8010E53C`): a sphere of radius r is swept along the move in
//!   steps of r. At each step end the sphere is tested against wall faces (one-sided: only when the point is
//!   on the front side), then flagged edges (+0x30 bits 1/2/4), then vertices shared by two flagged edges;
//!   the first hit pushes the point out to r + 0.0001 from the contact. All objects are retested until no
//!   hit (at most 10 passes). The move stops at the first step that hit.
//! - Touch regions (`GScolsys2ThruGetEventID` 0x8011163C): the same sphere tests against region triangles
//!   facing the move; pass regions (`fn_801101B4`): the segment crosses a region triangle facing the move;
//!   check regions (`GScolsys2CheckGetEventID` 0x80111B9C): likewise for the A-button ray. Region ids are
//!   the triangle's u16 at +0x32 (pass/touch) or +0x30 (check); at most 4, no duplicates.
//!
//! Transformed (moving) objects (group flag bit 0) are treated as static: their runtime matrices aren't
//! modelled yet.

use bevy::prelude::*;
use colo_formats::ccd;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct CTri {
    pub v: [Vec3; 3],
    pub n: Vec3,
    /// u16 at +0x30 / +0x32.
    pub a: u16,
    pub b: u16,
    /// ccd group (collision object) index.
    pub group: u16,
}

#[derive(Clone, Debug, Default)]
pub struct CList {
    pub tris: Vec<CTri>,
    pub grid: Option<ccd::Grid>,
}

impl CList {
    /// Triangles in the grid cells covering [p − r, p + r] (`fn_8010E53C` cell range), in cell order.
    fn near(&self, p: Vec3, r: f32) -> Vec<&CTri> {
        let Some(g) = &self.grid else { return self.tris.iter().collect() };
        let cx = |x: f32| ((x - g.origin_x) / g.cell_x) as i32;
        let cz = |z: f32| ((z - g.origin_z) / g.cell_z) as i32;
        let x0 = cx(p.x - r).max(0);
        let z0 = cz(p.z - r).max(0);
        let x1 = cx(p.x + r).min(g.nx as i32 - 1);
        let z1 = cz(p.z + r).min(g.nz as i32 - 1);
        let mut out = vec![];
        for z in z0..=z1 {
            for x in x0..=x1 {
                let (first, count) = g.cells[(x + z * g.nx as i32) as usize];
                for k in first..first + count {
                    if let Some(t) = g.indices.get(k as usize).and_then(|&i| self.tris.get(i as usize)) { out.push(t); }
                }
            }
        }
        out
    }
}

#[derive(Clone, Debug, Default)]
pub struct CObj {
    pub floor: Option<CList>,
    pub wall: Option<CList>,
    pub region: Option<CList>,
    pub check: Option<CList>,
    /// +0x38 sun occluders (`GScolsys2Sun`).
    pub sun: Option<CList>,
}

#[derive(Default)]
pub struct ColSys {
    pub objs: Vec<CObj>,
    /// Objects switched off (`scriptSetCol(id, 0)`).
    pub disabled: HashSet<u16>,
}

/// `GScolsy2UtilGetSidePlanePoint`: n · (p − v0).
fn side(n: Vec3, v0: Vec3, p: Vec3) -> f32 { n.z * (p.z - v0.z) + n.x * (p.x - v0.x) + n.y * (p.y - v0.y) }

/// `GScolsy2UtilGetCpPlanePoint`: closest point to `p` on the plane (n, v0).
fn plane_point(n: Vec3, v0: Vec3, p: Vec3) -> Vec3 {
    let k = (n.z * (v0.z - p.z) + n.x * (v0.x - p.x) + n.y * (v0.y - p.y)) / (n.z * n.z + n.x * n.x + n.y * n.y);
    n * k + p
}

/// `GScolsys2UtilGetCpLinePoint`: closest point on the line a→b to p, and its parameter.
fn line_point(a: Vec3, b: Vec3, p: Vec3) -> (Vec3, f32) {
    let d = b - a;
    let l = d.z * d.z + d.x * d.x + d.y * d.y;
    if l == 0.0 { return (a, 0.0); }
    let t = (d.z * (p.z - a.z) + d.x * (p.x - a.x) + d.y * (p.y - a.y)) / l;
    (d * t + a, t)
}

/// `GScolsys2UtilGetCpPlaneLine`: where the segment a→b meets the plane (n, v0), and its parameter.
fn plane_line(n: Vec3, v0: Vec3, a: Vec3, b: Vec3) -> Option<(Vec3, f32)> {
    let d = b - a;
    let den = n.z * d.z + n.x * d.x + n.y * d.y;
    if den == 0.0 { return None; }
    let t = (n.z * (v0.z - a.z) + n.x * (v0.x - a.x) + n.y * (v0.y - a.y)) / den;
    Some((d * t + a, t))
}

/// `GScolsy2UtilChkInTri` 0x8010F71C: is `p` (on the plane) inside the triangle, projected on the plane of
/// the two axes the normal is smallest along.
fn in_tri(p: Vec3, v: &[Vec3; 3], n: Vec3) -> bool {
    let (ax, ay, az) = (n.x.abs(), n.y.abs(), n.z.abs());
    let ax_ = |q: Vec3, i: usize| [q.x, q.y, q.z][i];
    let (i0, i1, s) = if ax < ay {
        if ay < az { (0, 1, n.z) } else { (2, 0, n.y) }
    } else if ax > az { (2, 1, -n.x) } else { (0, 1, n.z) };
    let (a, b, c) = if s < 0.0 { (v[0], v[1], v[2]) } else { (v[2], v[1], v[0]) };
    let (a, b, c) = ((ax_(a, i0), ax_(a, i1)), (ax_(b, i0), ax_(b, i1)), (ax_(c, i0), ax_(c, i1)));
    let (px, py) = (ax_(p, i0), ax_(p, i1));
    let minx = a.0.min(b.0).min(c.0).min(1e6);
    let miny = a.1.min(b.1).min(c.1).min(1e6);
    let maxx = a.0.max(b.0).max(c.0).max(-1e6);
    let maxy = a.1.max(b.1).max(c.1).max(-1e6);
    if px < minx || py < miny || maxx < px || maxy < py { return false; }
    (b.0 - a.0) * (py - a.1) - (b.1 - a.1) * (px - a.0) <= 0.0
        && (c.0 - b.0) * (py - b.1) - (c.1 - b.1) * (px - b.0) <= 0.0
        && (a.0 - c.0) * (py - c.1) - (a.1 - c.1) * (px - c.0) <= 0.0
}

/// Edge flags at +0x30: edge i (v_i → v_{i+1}) is exposed if bit `1 << i` is set.
const EDGE: [u16; 3] = [1, 2, 4];

/// Sphere (centre p, radius r) against one triangle list: faces, then edges, then vertices
/// (`fn_8010E53C`). `facing`: only triangles whose normal opposes this direction (region tests).
/// Returns the contact point of the first hit.
fn sphere_hit(tris: &[&CTri], p: Vec3, r: f32, facing: Option<Vec3>) -> Option<(Vec3, u16)> {
    let r2 = r * r;
    let ok = |t: &CTri| facing.is_none_or(|d| t.n.dot(d) < 0.0) && side(t.n, t.v[0], p) >= 0.0;
    for t in tris {
        if !ok(t) { continue; }
        let cp = plane_point(t.n, t.v[0], p);
        if cp.distance_squared(p) < r2 && in_tri(cp, &t.v, t.n) { return Some((cp, t.b)); }
    }
    for t in tris {
        if t.a & 7 == 0 || !ok(t) { continue; }
        for i in 0..3 {
            if t.a & EDGE[i] == 0 { continue; }
            let (c, s) = line_point(t.v[i], t.v[(i + 1) % 3], p);
            if (0.0..=1.0).contains(&s) && c.distance_squared(p) < r2 { return Some((c, t.b)); }
        }
    }
    for t in tris {
        if t.a & 7 == 0 || !ok(t) { continue; }
        for i in 0..3 {
            let j = if i == 0 { 2 } else { i - 1 };
            if t.a & EDGE[i] != 0 && t.a & EDGE[j] != 0 && t.v[i].distance_squared(p) < r2 { return Some((t.v[i], t.b)); }
        }
    }
    None
}

/// `GScolsy2UtilGetPointExtentionLine`: from `contact` through `p` to distance `len`.
fn extend(contact: Vec3, p: Vec3, len: f32) -> Vec3 {
    let d = p - contact;
    d * (len / d.length()) + contact
}

impl ColSys {
    pub fn from_ccd(c: &ccd::Collision) -> Self {
        let mut objs = vec![];
        for (g, lists) in c.groups.iter().enumerate() {
            let mut o = CObj::default();
            for l in lists {
                let tris = l.triangles.iter().map(|t| CTri {
                    v: t.vertices.map(Vec3::from_array), n: Vec3::from_array(t.normal), a: t.raw[0], b: t.raw[1], group: g as u16,
                }).collect();
                let list = Some(CList { tris, grid: l.grid.clone() });
                match l.slot { 0 => o.floor = list, 1 => o.wall = list, 2 => o.region = list, 3 => o.check = list, 5 => o.sun = list, _ => {} }
            }
            objs.push(o);
        }
        ColSys { objs, disabled: HashSet::new() }
    }

    fn enabled(&self) -> impl Iterator<Item = &CObj> {
        self.objs.iter().enumerate().filter(|(i, _)| !self.disabled.contains(&(*i as u16))).map(|(_, o)| o)
    }

    /// `fn_8010E138`: floor heights under (p.x, p.z), at most 8, no duplicates within an object.
    pub fn floor_heights(&self, p: Vec3) -> Vec<f32> {
        let mut out = vec![];
        for o in self.enabled() {
            let Some(l) = &o.floor else { continue };
            let Some(g) = &l.grid else { continue };
            let ix = ((p.x - g.origin_x) / g.cell_x) as i32;
            let iz = ((p.z - g.origin_z) / g.cell_z) as i32;
            if ix < 0 || ix >= g.nx as i32 || iz < 0 || iz >= g.nz as i32 { continue; }
            let (first, count) = g.cells[(ix + iz * g.nx as i32) as usize];
            let mut local: Vec<f32> = vec![];
            for k in first..first + count {
                if local.len() >= 8 { break; }
                let Some(t) = g.indices.get(k as usize).and_then(|&i| l.tris.get(i as usize)) else { continue };
                if let Some(y) = poly_height(p, t) {
                    if !local.contains(&y) { local.push(y); }
                }
            }
            for y in local { if out.len() < 8 { out.push(y); } }
        }
        out
    }

    /// `fn_8010EFE4`: push the sphere at `p` out of every enabled object's walls, repeating while anything
    /// hit (at most 10 passes). Returns the corrected point if anything hit.
    fn walls_push(&self, r: f32, p: Vec3) -> Option<Vec3> {
        let mut q = p;
        let mut passes = 0;
        loop {
            let mut hits = 0;
            for o in self.enabled() {
                let Some(l) = &o.wall else { continue };
                if let Some((c, _)) = sphere_hit(&l.near(q, r), q, r, None) {
                    hits += 1;
                    q = extend(c, q, r + 0.0001);
                }
            }
            if hits == 0 { break; }
            passes += 1;
            if passes >= 10 { break; }
        }
        (passes > 0).then_some(q)
    }

    /// `fn_8010F320`: sweep a sphere of radius `r` from `a` to `b` in steps of r; the first step whose end
    /// hits a wall returns the pushed-out point.
    pub fn wall_sweep(&self, r: f32, a: Vec3, b: Vec3) -> Option<Vec3> {
        let dist = a.distance(b);
        let step = if dist > 0.0 { (r / dist).min(1.0) } else { 0.0 };
        let d = b - a;
        let mut t = 0.0f32;
        while t < 1.0 {
            let t1 = (t + step).min(1.0);
            if let Some(q) = self.walls_push(r, d * t1 + a) { return Some(q); }
            if step <= 0.0 { break; }
            t += step;
        }
        None
    }

    /// `GScolsys2ThruGetEventID`: touch-region ids the sphere meets moving from `a` to `b`.
    pub fn touch_regions(&self, r: f32, a: Vec3, b: Vec3) -> Vec<u16> {
        let mut out: Vec<u16> = vec![];
        let d = b - a;
        let len = d.length();
        if len <= 0.0 { return out; }
        let step = (r / len).min(1.0);
        let mut t = 0.0f32;
        while t < 1.0 && out.len() < 4 {
            let q = d * (t + step).min(1.0) + a;
            for o in self.enabled() {
                let Some(l) = &o.region else { continue };
                // faces, edges and vertices each add every hit (not just the first), skipping known ids
                for id in region_sphere_all(&l.near(q, r), q, r, d) {
                    if out.len() < 4 && !out.contains(&id) { out.push(id); }
                }
            }
            if step <= 0.0 { break; }
            t += step;
        }
        out
    }

    /// `fn_801101B4`: pass-region ids whose triangles the segment a→b crosses (facing the move).
    pub fn pass_regions(&self, a: Vec3, b: Vec3) -> Vec<u16> {
        let d = b - a;
        let mut out: Vec<u16> = vec![];
        for o in self.enabled() {
            let Some(l) = &o.region else { continue };
            for t in &l.tris {
                if out.len() >= 4 { break; }
                if out.contains(&t.b) || t.n.dot(d) >= 0.0 { continue; }
                if let Some((c, s)) = plane_line(t.n, t.v[0], a, b) {
                    if (0.0..=1.0).contains(&s) && in_tri(c, &t.v, t.n) { out.push(t.b); }
                }
            }
        }
        out
    }

    /// `GScolsys2CheckGetEventID`: check-region ids the ray a→b crosses (id = u16 at +0x30).
    pub fn check_regions(&self, a: Vec3, b: Vec3) -> Vec<u16> {
        let d = b - a;
        let mut out: Vec<u16> = vec![];
        if d.length() <= 0.0 { return out; }
        for o in self.enabled() {
            let Some(l) = &o.check else { continue };
            for t in &l.tris {
                if out.len() >= 4 { break; }
                if out.contains(&t.a) || t.n.dot(d) >= 0.0 { continue; }
                if let Some((c, s)) = plane_line(t.n, t.v[0], a, b) {
                    if (0.0..=1.0).contains(&s) && in_tri(c, &t.v, t.n) { out.push(t.a); }
                }
            }
        }
        out
    }

    /// `GScolsys2Sun` 0x80111C24: does a sun-occluder triangle (+0x38 list) of an enabled object cross the
    /// segment sun→eye? Both-sided, parameter in [0, 1]. Skipped when the two points coincide.
    pub fn sun_blocked(&self, sun: Vec3, eye: Vec3) -> bool {
        if eye.distance(sun) <= 0.0 { return false; }
        self.enabled().filter_map(|o| o.sun.as_ref()).flat_map(|l| &l.tris).any(|t| {
            plane_line(t.n, t.v[0], sun, eye).is_some_and(|(c, s)| (0.0..=1.0).contains(&s) && in_tri(c, &t.v, t.n))
        })
    }

    /// All region triangles (for debug drawing and region centres).
    pub fn tris(&self) -> impl Iterator<Item = (&'static str, &CTri)> {
        self.objs.iter().flat_map(|o| {
            [("floor", &o.floor), ("wall", &o.wall), ("region", &o.region), ("check", &o.check)].into_iter()
                .filter_map(|(k, l)| l.as_ref().map(|l| (k, l)))
                .flat_map(|(k, l)| l.tris.iter().map(move |t| (k, t)))
        })
    }
}

/// Every touch-region triangle the sphere meets (`GScolsys2ThruGetFixedMdlEventList` 0x8011069C: faces,
/// then flagged edges, then vertices; each list max 4 with unique ids).
fn region_sphere_all(tris: &[&CTri], p: Vec3, r: f32, dir: Vec3) -> Vec<u16> {
    let r2 = r * r;
    let mut out: Vec<u16> = vec![];
    let ok = |t: &CTri| t.n.dot(dir) < 0.0 && side(t.n, t.v[0], p) >= 0.0;
    for t in tris {
        if out.len() >= 4 || out.contains(&t.b) || !ok(t) { continue; }
        let cp = plane_point(t.n, t.v[0], p);
        if cp.distance_squared(p) < r2 && in_tri(cp, &t.v, t.n) { out.push(t.b); }
    }
    for t in tris {
        if out.len() >= 4 || t.a & 7 == 0 || out.contains(&t.b) || !ok(t) { continue; }
        if (0..3).any(|i| t.a & EDGE[i] != 0 && {
            let (c, s) = line_point(t.v[i], t.v[(i + 1) % 3], p);
            (0.0..=1.0).contains(&s) && c.distance_squared(p) < r2
        }) { out.push(t.b); }
    }
    for t in tris {
        if out.len() >= 4 || t.a & 7 == 0 || out.contains(&t.b) || !ok(t) { continue; }
        if (0..3).any(|i| { let j = if i == 0 { 2 } else { i - 1 }; t.a & EDGE[i] != 0 && t.a & EDGE[j] != 0 && t.v[i].distance_squared(p) < r2 }) {
            out.push(t.b);
        }
    }
    out
}

/// `getCpPolyVec` 0x8010DEF0: height of the floor triangle at (p.x, p.z) if p projects inside it.
fn poly_height(p: Vec3, t: &CTri) -> Option<f32> {
    let [a, b, c] = t.v;
    let minx = a.x.min(b.x).min(c.x).min(1e6);
    let minz = a.z.min(b.z).min(c.z).min(1e6);
    let maxx = a.x.max(b.x).max(c.x).max(-1e6);
    let maxz = a.z.max(b.z).max(c.z).max(-1e6);
    if p.x < minx || p.z < minz || maxx < p.x || maxz < p.z { return None; }
    if (b.x - a.x) * (p.z - a.z) - (b.z - a.z) * (p.x - a.x) > 0.0 { return None; }
    if (c.x - b.x) * (p.z - b.z) - (c.z - b.z) * (p.x - b.x) > 0.0 { return None; }
    if (a.x - c.x) * (p.z - c.z) - (a.z - c.z) * (p.x - c.x) > 0.0 { return None; }
    let n = t.n;
    if n.y == 0.0 { return None; }
    Some(p.y + -(n.z * (p.z - a.z) + n.x * (p.x - a.x) + n.y * (p.y - a.y)) / n.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tri(v: [[f32; 3]; 3], a: u16) -> CTri {
        let v = v.map(Vec3::from_array);
        CTri { v, n: (v[1] - v[0]).cross(v[2] - v[0]).normalize(), a, b: 0, group: 0 }
    }
    #[test]
    fn one_sided_wall() {
        // wall in the plane z = 0 facing +z (normal (0,0,1))
        let t = tri([[-10.0, 0.0, 0.0], [10.0, 0.0, 0.0], [0.0, 20.0, 0.0]], 0);
        assert!(t.n.z > 0.9);
        let cs = ColSys { objs: vec![CObj { wall: Some(CList { tris: vec![t], grid: None }), ..default() }], disabled: default() };
        // from the front: pushed back to radius 4
        let hit = cs.wall_sweep(4.0, Vec3::new(0.0, 5.0, 10.0), Vec3::new(0.0, 5.0, 1.0)).unwrap();
        assert!((hit.z - 4.0001).abs() < 1e-3, "{hit:?}");
        // from behind: passes
        assert!(cs.wall_sweep(4.0, Vec3::new(0.0, 5.0, -10.0), Vec3::new(0.0, 5.0, -1.0)).is_none());
    }
}

#[cfg(test)]
mod real_tests {
    use super::*;
    #[test]
    fn guild_walls_block() {
        let Ok(b) = std::fs::read("/tmp/ccdx2/M2_guild_1F_2.ccd") else { return };
        let c = ccd::Collision::parse(&b).unwrap();
        let cs = ColSys::from_ccd(&c);
        let (mut faces, mut inside) = (0, 0);
        for o in &cs.objs {
            let Some(l) = &o.wall else { continue };
            for t in &l.tris {
                faces += 1;
                let centre = (t.v[0] + t.v[1] + t.v[2]) / 3.0;
                if in_tri(centre, &t.v, t.n) { inside += 1; }
            }
        }
        println!("wall tris {faces}, centroid inside {inside}");
        assert_eq!(faces, inside);
    }
}
