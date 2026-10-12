//! Player-vs-room collision on the `.ccd` triangles.
//!
//! Floors and walls come from the solid lists (slots 0 and 1); they are told apart by the triangle normal
//! rather than by slot. Slot 2 holds the pass/touch regions (the triangle's second u16 is the region id that
//! interaction points refer to). Slot 4 duplicates slot 1 and is skipped.

use bevy::prelude::*;
use colo_formats::ccd;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Floor,
    Wall,
    /// Slot 2: pass/touch regions (walking into them).
    Region,
    /// Slot 3: check regions (A button: the hero's check ray hits them).
    Check,
}

#[derive(Clone, Debug)]
pub struct Tri {
    #[allow(dead_code)]
    pub slot: u8,
    pub kind: Kind,
    pub v: [Vec3; 3],
    pub normal: Vec3,
    pub region: u16,
    /// `.ccd` group index = the collision id that `scriptSetCol(id, on)` toggles (door blockers, story gates).
    pub group: u16,
}

#[derive(Default)]
pub struct Collision {
    pub tris: Vec<Tri>,
    /// Groups switched off by `scriptSetCol(id, 0)` (an opened door, a removed barrier).
    pub disabled: std::collections::HashSet<u16>,
}

pub const RADIUS: f32 = 4.0;
/// Highest step up the player can take in one move.
pub const STEP_UP: f32 = 8.0;
/// Deepest drop that still counts as walking down (not falling off the world).
pub const STEP_DOWN: f32 = 30.0;

impl Collision {
    pub fn from_ccd(c: &ccd::Collision) -> Self {
        let mut tris = vec![];
        for (g, l, t) in c.groups.iter().enumerate().flat_map(|(g, ls)| ls.iter().flat_map(move |l| l.triangles.iter().map(move |t| (g, l, t)))) {
            let slot = l.slot as u8;
            if slot == 4 { continue; }
            let v = t.vertices.map(Vec3::from_array);
            let n = (v[1] - v[0]).cross(v[2] - v[0]).normalize_or_zero();
            let kind = match slot {
                0 | 1 if n.y.abs() > 0.5 => Kind::Floor,
                0 | 1 => Kind::Wall,
                3 => Kind::Check,
                _ => Kind::Region,
            };
            // floors face up regardless of winding
            let normal = if kind == Kind::Floor && n.y < 0.0 { -n } else { n };
            tris.push(Tri { slot, kind, v, normal, region: t.index, group: g as u16 });
        }
        Collision { tris, disabled: Default::default() }
    }

    /// Center of each region id (for debugging / test walks).
    pub fn region_centers(&self, kind: Kind) -> Vec<(u16, Vec3)> {
        let mut m: std::collections::BTreeMap<u16, (Vec3, f32)> = Default::default();
        for t in self.active(kind) {
            let e = m.entry(t.region).or_insert((Vec3::ZERO, 0.0));
            e.0 += (t.v[0] + t.v[1] + t.v[2]) / 3.0;
            e.1 += 1.0;
        }
        m.into_iter().map(|(r, (s, n))| (r, s / n)).collect()
    }

    pub fn set_group(&mut self, group: u16, on: bool) {
        if on { self.disabled.remove(&group); } else { self.disabled.insert(group); }
    }

    /// Triangles of a kind that are currently switched on.
    pub fn active(&self, kind: Kind) -> impl Iterator<Item = &Tri> {
        self.tris.iter().filter(move |t| t.kind == kind && !self.disabled.contains(&t.group))
    }

    pub fn summary(&self) -> String {
        let n = |k| self.tris.iter().filter(|t| t.kind == k).count();
        format!("{} floor / {} wall / {} region / {} check triangles", n(Kind::Floor), n(Kind::Wall), n(Kind::Region), n(Kind::Check))
    }

    /// Height of the highest floor under `p` within [p.y - down, p.y + up].
    pub fn floor_height(&self, p: Vec3, up: f32, down: f32) -> Option<f32> {
        let mut best: Option<f32> = None;
        for t in self.active(Kind::Floor) {
            let Some(y) = height_on(t, p.x, p.z) else { continue };
            if y <= p.y + up && y >= p.y - down && best.is_none_or(|b| y > b) { best = Some(y); }
        }
        best
    }

    /// Region id of the pass/touch region the player stands in.
    pub fn region_at(&self, p: Vec3) -> Option<u16> {
        self.active(Kind::Region).find(|t| {
            let (lo, hi) = (t.v.iter().map(|v| v.y).fold(f32::MAX, f32::min), t.v.iter().map(|v| v.y).fold(f32::MIN, f32::max));
            // flat regions on the ground: the player is on/near them; upright regions: within their height span
            if t.normal.y.abs() > 0.5 {
                height_on(t, p.x, p.z).is_some_and(|y| (p.y - y).abs() < 25.0)
            } else {
                p.y + 10.0 >= lo && p.y <= hi && dist_xz_to_tri(t, p) < RADIUS + 1.0
            }
        }).map(|t| t.region)
    }

    /// Slide from `from` towards `to`: walls push the player out (radius `RADIUS` at body height), the
    /// result is snapped to the floor; moves onto no floor or up a too-high step are refused.
    pub fn move_player(&self, from: Vec3, to: Vec3) -> Vec3 {
        if self.tris.is_empty() { return to; }
        // sub-steps shorter than the radius so fast moves / long frames cannot tunnel through walls
        let d = to - from;
        let steps = (d.length() / (RADIUS * 0.5)).ceil().max(1.0) as usize;
        let mut p = from;
        for i in 0..steps {
            let target = p + d / steps as f32;
            let next = self.step(p, target);
            if next == p && i > 0 { break; }
            p = next;
        }
        p
    }

    fn step(&self, from: Vec3, to: Vec3) -> Vec3 {
        let mut p = to;
        for _ in 0..3 {
            let mut pushed = false;
            for t in self.active(Kind::Wall) {
                for h in [4.0, 12.0] {
                    let Some((a, b)) = slice(t, from.y + h) else { continue };
                    let q = Vec2::new(p.x, p.z);
                    let c = closest_on_segment(q, a, b);
                    let d = q - c;
                    let len = d.length();
                    if len < RADIUS {
                        let n = if len > 1e-4 { d / len } else { Vec2::new(t.normal.x, t.normal.z).normalize_or_zero() };
                        let fix = c + n * RADIUS;
                        p.x = fix.x;
                        p.z = fix.y;
                        pushed = true;
                    }
                }
            }
            if !pushed { break; }
        }
        match self.floor_height(p, STEP_UP, STEP_DOWN) {
            Some(y) => Vec3::new(p.x, y, p.z),
            None => from,
        }
    }
}

/// Y of the triangle's plane at (x, z) if (x, z) is inside its XZ projection.
fn height_on(t: &Tri, x: f32, z: f32) -> Option<f32> {
    let [a, b, c] = t.v;
    let (v0, v1, v2) = (Vec2::new(b.x - a.x, b.z - a.z), Vec2::new(c.x - a.x, c.z - a.z), Vec2::new(x - a.x, z - a.z));
    let den = v0.x * v1.y - v1.x * v0.y;
    if den.abs() < 1e-6 { return None; }
    let u = (v2.x * v1.y - v1.x * v2.y) / den;
    let v = (v0.x * v2.y - v2.x * v0.y) / den;
    if u < -1e-4 || v < -1e-4 || u + v > 1.0 + 1e-4 { return None; }
    Some(a.y + u * (b.y - a.y) + v * (c.y - a.y))
}

/// The segment where a triangle crosses the horizontal plane at `y`, in XZ.
fn slice(t: &Tri, y: f32) -> Option<(Vec2, Vec2)> {
    let mut pts = [Vec2::ZERO; 2];
    let mut n = 0;
    for i in 0..3 {
        let (a, b) = (t.v[i], t.v[(i + 1) % 3]);
        if (a.y - y) * (b.y - y) <= 0.0 && (a.y - b.y).abs() > 1e-5 && n < 2 {
            let s = (y - a.y) / (b.y - a.y);
            let p = a + (b - a) * s;
            pts[n] = Vec2::new(p.x, p.z);
            n += 1;
        }
    }
    (n == 2).then_some((pts[0], pts[1]))
}

fn closest_on_segment(p: Vec2, a: Vec2, b: Vec2) -> Vec2 {
    let ab = b - a;
    let l = ab.length_squared();
    if l < 1e-8 { return a; }
    a + ab * ((p - a).dot(ab) / l).clamp(0.0, 1.0)
}

fn dist_xz_to_tri(t: &Tri, p: Vec3) -> f32 {
    let q = Vec2::new(p.x, p.z);
    (0..3).map(|i| {
        let (a, b) = (t.v[i], t.v[(i + 1) % 3]);
        closest_on_segment(q, Vec2::new(a.x, a.z), Vec2::new(b.x, b.z)).distance(q)
    }).fold(f32::MAX, f32::min)
}
