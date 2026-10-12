//! HAL sysdolphin (HSD) archives (`.dat`, `.rdat`, `.cam`) and static model extraction.
//!
//! Archive: 0x20-byte header {file size, data size, relocation count, root count, reference count},
//! data block (pointers inside are offsets from the data start), relocation table, root/reference
//! tables {u32 offset, u32 name offset}, string table. Colosseum models have one root, `scene_data`.
//!
//! Node layouts (offsets in bytes) follow the HSD runtime (doldecomp/melee `baselib`) and StarsMmd's
//! Blender addon:
//! - SceneData: models (ModelSet** NULL-terminated), camera, lights, fog
//! - ModelSet: joint, anim joints, mat anim joints, shape anim joints
//! - JObj: name 0, flags 4, child 8, next 0xC, dobj/property 0x10, rot 0x14, scale 0x20, pos 0x2C, inv bind 0x38
//! - DObj: name 0, next 4, mobj 8, pobj 0xC
//! - PObj: name 0, next 4, vtx desc 8, flags u16 0xC, dl blocks u16 0xE (×32 bytes), dl 0x10, property 0x14
//! - Vertex desc (0x18 each, ends at attr 0xFF): attr, attr type, comp count, comp type, frac u8, stride u16 @0x12, data @0x14
//! - MObj: class 0, render mode 4, tobj 8, material 0xC; Material: ambient 0, diffuse 4, specular 8, alpha f32 0xC
//! - TObj: next 4, texmap id 8, coord src 0xC, rot 0x10, scale 0x1C, trans 0x28, wrap s/t 0x34/0x38, flags 0x40, image 0x4C, palette 0x50
//! - Image: data 0, w u16 4, h u16 6, format 8; Palette: data 0, format 4, count u16 0xC

#![allow(clippy::needless_range_loop)] // small fixed-size matrix loops read clearer with indices

use crate::be::{cstr_at, u16_at, u32_at, u8_at};
use crate::{gx, Error, Result};
use std::collections::HashMap;

pub const JOBJ_HIDDEN: u32 = 1 << 4;
pub const JOBJ_PTCL: u32 = 1 << 5;
pub const JOBJ_SPLINE: u32 = 1 << 14;
pub const JOBJ_SKELETON: u32 = 1 << 0;
pub const JOBJ_SKELETON_ROOT: u32 = 1 << 1;
/// The joint's `child` points at another joint whose subtree is drawn again under this transform.
pub const JOBJ_INSTANCE: u32 = 1 << 12;
pub const POBJ_TYPE_MASK: u16 = 3 << 12;
pub const POBJ_SKIN: u16 = 0;
pub const POBJ_ENVELOPE: u16 = 2 << 12;
pub const RENDER_XLU: u32 = 1 << 30;

pub struct Archive<'a> {
    pub file: &'a [u8],
    pub data: &'a [u8],
    pub roots: Vec<(String, usize)>,
    pub references: Vec<(String, usize)>,
}

impl<'a> Archive<'a> {
    pub fn parse(file: &'a [u8]) -> Result<Self> {
        let data_size = u32_at(file, 4, "HSD header")? as usize;
        let nrel = u32_at(file, 8, "HSD header")? as usize;
        let nroot = u32_at(file, 0xC, "HSD header")? as usize;
        let nref = u32_at(file, 0x10, "HSD header")? as usize;
        let data = file.get(0x20..0x20 + data_size).ok_or(Error::Truncated { what: "HSD data", offset: 0x20, need: data_size, have: file.len() })?;
        let root_off = 0x20 + data_size + nrel * 4;
        let str_off = root_off + (nroot + nref) * 8;
        let mut roots = vec![];
        let mut references = vec![];
        for i in 0..nroot + nref {
            let o = u32_at(file, root_off + 8 * i, "HSD roots")? as usize;
            let s = u32_at(file, root_off + 8 * i + 4, "HSD roots")? as usize;
            let name = cstr_at(file, str_off + s, "HSD root name")?;
            if i < nroot { roots.push((name, o)) } else { references.push((name, o)) }
        }
        Ok(Archive { file, data, roots, references })
    }
    pub fn root(&self, name: &str) -> Option<usize> { self.roots.iter().find(|r| r.0 == name).map(|r| r.1) }
    fn u32(&self, o: usize) -> Result<u32> { u32_at(self.data, o, "HSD data") }
    fn ptr(&self, o: usize) -> Result<Option<usize>> { let p = self.u32(o)? as usize; Ok((p != 0).then_some(p)) }
    fn f32(&self, o: usize) -> Result<f32> { Ok(f32::from_bits(self.u32(o)?)) }
    fn vec3(&self, o: usize) -> Result<[f32; 3]> { Ok([self.f32(o)?, self.f32(o + 4)?, self.f32(o + 8)?]) }
}

/// Column-major-free simple 4x4 (row-major, m[r][c]) with affine helpers.
pub type Mat4 = [[f32; 4]; 4];
pub const IDENTITY: Mat4 = [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]];

pub fn mul(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut r = [[0.0; 4]; 4];
    for i in 0..4 { for j in 0..4 { for k in 0..4 { r[i][j] += a[i][k] * b[k][j]; } } }
    r
}
pub fn transform_point(m: &Mat4, p: [f32; 3]) -> [f32; 3] {
    let v = |r: usize| m[r][0] * p[0] + m[r][1] * p[1] + m[r][2] * p[2] + m[r][3];
    [v(0), v(1), v(2)]
}
/// `d` through the inverse transpose of `m`'s linear part (`PSMTXInvXpose`, the GX normal matrix of
/// `PObjSetupMtx` 0x801AABB4; `m` itself when singular), not normalised.
pub fn transform_nrm_raw(m: &Mat4, d: [f32; 3]) -> [f32; 3] {
    let c = |r: usize| [m[0][r], m[1][r], m[2][r]];
    let (c0, c1, c2) = (c(0), c(1), c(2));
    let cross = |a: [f32; 3], b: [f32; 3]| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let (k0, k1, k2) = (cross(c1, c2), cross(c2, c0), cross(c0, c1));
    let det = dot(c0, k0);
    if det == 0.0 { let v = |r: usize| m[r][0] * d[0] + m[r][1] * d[1] + m[r][2] * d[2]; return [v(0), v(1), v(2)]; }
    // the inverse transpose has the cofactor columns k0, k1, k2 / det
    let v = |r: usize| (k0[r] * d[0] + k1[r] * d[1] + k2[r] * d[2]) / det;
    [v(0), v(1), v(2)]
}
pub fn transform_dir(m: &Mat4, d: [f32; 3]) -> [f32; 3] {
    let v = |r: usize| m[r][0] * d[0] + m[r][1] * d[1] + m[r][2] * d[2];
    let o = [v(0), v(1), v(2)];
    let l = (o[0] * o[0] + o[1] * o[1] + o[2] * o[2]).sqrt();
    if l > 0.0 { [o[0] / l, o[1] / l, o[2] / l] } else { o }
}
/// HSD SRT: T * Rz * Ry * Rx * S
pub fn srt(s: [f32; 3], r: [f32; 3], t: [f32; 3]) -> Mat4 {
    let (sx, cx) = r[0].sin_cos();
    let (sy, cy) = r[1].sin_cos();
    let (sz, cz) = r[2].sin_cos();
    let rm = [
        [cy * cz, sx * sy * cz - cx * sz, cx * sy * cz + sx * sz],
        [cy * sz, sx * sy * sz + cx * cz, cx * sy * sz - sx * cz],
        [-sy, sx * cy, cx * cy],
    ];
    let mut m = IDENTITY;
    for i in 0..3 { for j in 0..3 { m[i][j] = rm[i][j] * s[j]; } m[i][3] = t[i]; }
    m
}
pub fn invert_affine(m: &Mat4) -> Mat4 {
    let a = |r: usize, c: usize| m[r][c] as f64;
    let det = a(0, 0) * (a(1, 1) * a(2, 2) - a(1, 2) * a(2, 1)) - a(0, 1) * (a(1, 0) * a(2, 2) - a(1, 2) * a(2, 0)) + a(0, 2) * (a(1, 0) * a(2, 1) - a(1, 1) * a(2, 0));
    if det.abs() < 1e-12 { return IDENTITY; }
    let inv = |r: usize, c: usize| -> f64 {
        let (r1, r2) = ((c + 1) % 3, (c + 2) % 3);
        let (c1, c2) = ((r + 1) % 3, (r + 2) % 3);
        (a(r1, c1) * a(r2, c2) - a(r1, c2) * a(r2, c1)) / det
    };
    let mut o = IDENTITY;
    for r in 0..3 { for c in 0..3 { o[r][c] = inv(r, c) as f32; } }
    for r in 0..3 { o[r][3] = -(o[r][0] * m[0][3] + o[r][1] * m[1][3] + o[r][2] * m[2][3]); }
    o
}

#[derive(Debug, Clone)]
pub struct Joint {
    pub offset: usize,
    pub name: String,
    pub flags: u32,
    pub parent: Option<usize>,
    pub world: Mat4,
    /// Inverse bind matrix (3x4 in file) if present.
    pub inverse_bind: Option<Mat4>,
    /// Rest pose (local): scale, Euler XYZ rotation (radians), translation.
    pub rest_scale: [f32; 3],
    pub rest_rotation: [f32; 3],
    pub rest_translation: [f32; 3],
    /// Reference objects (`HSD_RObjDesc` list at +0x3C): IK hints, limits, joint constraints.
    pub robj: Vec<RObj>,
}

/// One `HSD_RObjDesc`: `flags` holds the type (0x70000000) and the constraint subtype (0x0FFFFFFF).
#[derive(Debug, Clone, Copy)]
pub struct RObj {
    pub flags: u32,
    pub kind: RObjKind,
}

#[derive(Debug, Clone, Copy)]
pub enum RObjKind {
    /// REFTYPE_IKHINT: bone length and twist around the target axis.
    IkHint { bone_length: f32, rotate_x: f32 },
    /// REFTYPE_LIMIT (subtype 5 = min angle, 6 = max angle for IK joint 2).
    Limit(f32),
    /// REFTYPE_JOBJ: referenced joint struct offset (subtype 1 = IK target on the effector, 3 = pole on joint 1).
    Joint(usize),
    Other,
}

pub const REFTYPE_JOBJ: u32 = 0x1000_0000;
pub const REFTYPE_LIMIT: u32 = 0x2000_0000;
pub const REFTYPE_IKHINT: u32 = 0x4000_0000;

impl Joint {
    /// IK role: 0 none, 1 JOINT1, 2 JOINT2, 3 EFFECTOR (flags bits 21..22).
    pub fn ik_type(&self) -> u32 { (self.flags >> 21) & 3 }
    pub fn robj(&self, ty: u32, sub: u32) -> Option<&RObj> {
        self.robj.iter().find(|r| r.flags & 0x7000_0000 == ty && (sub == 0 || r.flags & 0x0FFF_FFFF == sub))
    }
    pub fn ik_hint(&self) -> Option<(f32, f32, u32)> {
        match self.robj(REFTYPE_IKHINT, 0) { Some(RObj { flags, kind: RObjKind::IkHint { bone_length, rotate_x } }) => Some((*bone_length, *rotate_x, *flags)), _ => None }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wrap { Clamp, Repeat, Mirror }

#[derive(Debug, Clone)]
pub struct Material {
    pub render_mode: u32,
    pub diffuse: [f32; 4],
    pub ambient: [f32; 4],
    pub alpha: f32,
    /// First texture (most map/NPC materials use one).
    pub texture: Option<usize>,
    pub texture_count: usize,
    pub wrap: [Wrap; 2],
    /// MObj offset (key for material animations).
    pub mobj: usize,
    /// Offset of the first texture's TObj (key for texture animations).
    pub tobj: Option<usize>,
    /// The first texture's SRT; its matrix is baked into the mesh UVs.
    pub tex_srt: TexSrt,
    pub translucent: bool,
    /// Pixel-engine data present (blend/alpha test set explicitly).
    pub has_pixel_engine: bool,
    /// `HSD_Material` specular colour (+8) and shininess (+0x10).
    pub specular: [f32; 4],
    pub shininess: f32,
    /// Every TObj in list order (what `MObjMakeTExp` walks to build the TEV stages).
    pub layers: Vec<TexLayer>,
    /// `HSD_PEDesc` (+0x14): flags, ref0, ref1, dst_alpha, blend type, src, dst, logic op, z compare,
    /// alpha comp0, alpha op, alpha comp1.
    pub pe: Option<[u8; 12]>,
}

/// `HSD_TObjTevDesc` (TObjDesc +0x58): one custom TEV stage for a TObj (`MakeColorGenTExp` 0x801BCF30).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TObjTev {
    pub color_op: u8, pub alpha_op: u8, pub color_bias: u8, pub alpha_bias: u8,
    pub color_scale: u8, pub alpha_scale: u8, pub color_clamp: u8, pub alpha_clamp: u8,
    /// GX_CC_* inputs a..d, or 0x80 | n: KONST_RGB/RRR/GGG/BBB/AAA, TEX0_RGB/AAA, TEX1_RGB/AAA (`TOBJ_TEV_CC_*`).
    pub color_in: [u8; 4],
    /// GX_CA_* inputs a..d, or 0x40 | n: KONST_R/G/B/A, TEX0_A, TEX1_A (`TOBJ_TEV_CA_*`).
    pub alpha_in: [u8; 4],
    pub konst: [u8; 4],
    pub tev0: [u8; 4],
    pub tev1: [u8; 4],
    /// Bit 30: the colour half is used, bit 31: the alpha half (`TOBJ_TEVREG_ACTIVE_*`).
    pub active: u32,
}

pub const TEX_COORD_UV: u32 = 0;
pub const TEX_COORD_REFLECTION: u32 = 1;
pub const TEX_COORD_HILIGHT: u32 = 2;
pub const TEX_COORD_SHADOW: u32 = 3;
pub const TEX_COORD_TOON: u32 = 4;
pub const TEX_COORD_GRADATION: u32 = 5;
pub const TEX_LIGHTMAP_DIFFUSE: u32 = 0x10;
pub const TEX_LIGHTMAP_SPECULAR: u32 = 0x20;
pub const TEX_LIGHTMAP_AMBIENT: u32 = 0x40;
pub const TEX_LIGHTMAP_EXT: u32 = 0x80;
pub const TEX_LIGHTMAP_SHADOW: u32 = 0x100;
pub const TEX_BUMP: u32 = 1 << 24;

/// One TObj of a material (`HSD_TObjDesc`).
#[derive(Debug, Clone)]
pub struct TexLayer {
    pub tobj: usize,
    /// GXTexMapID (+8); 0xFF (GX_TEXMAP_NULL) TObjs take no part in the TEV.
    pub id: u32,
    /// GXTexGenSrc (+0xC): 4 + n = GX_TG_TEX<n>.
    pub src: u32,
    /// Flags (+0x40): coord 0..3, light map 4..8, colour map 16..19, alpha map 20..23, bump 24.
    pub flags: u32,
    /// +0x44, the BLEND colour/alpha map factor.
    pub blending: f32,
    pub texture: Option<usize>,
    pub wrap: [Wrap; 2],
    pub srt: TexSrt,
    pub tev: Option<TObjTev>,
}

impl TexLayer {
    pub fn coord(&self) -> u32 { self.flags & 0xF }
    pub fn lightmap(&self) -> u32 { self.flags & 0x1F0 }
    pub fn colormap(&self) -> u32 { (self.flags >> 16) & 0xF }
    pub fn alphamap(&self) -> u32 { (self.flags >> 20) & 0xF }
    /// Texture coordinate set (GX_TG_TEX0..7) for UV-mapped layers.
    pub fn uv_set(&self) -> usize { self.src.saturating_sub(4).min(7) as usize }
}

/// A TObj's texture-coordinate transform (`HSD_TObj` scale 0x1C, rotate 0x10, translate 0x28, wrap_t 0x38,
/// repeat_s/t 0x3C/0x3D).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TexSrt {
    pub scale: [f32; 3],
    pub rotate: [f32; 3],
    pub translate: [f32; 3],
    pub repeat: [u8; 2],
    pub wrap_t_mirror: bool,
}

impl Default for TexSrt {
    fn default() -> Self { TexSrt { scale: [1.0; 3], rotate: [0.0; 3], translate: [0.0; 3], repeat: [1, 1], wrap_t_mirror: false } }
}

impl TexSrt {
    /// `MakeTextureMtx` (0x801BDxxx, hsd_tobj.c): `S * R * T` with S = repeat / scale, R = rotation by
    /// (x, y, -z), T = -(translate) (t also shifted by one repeat when wrap_t is mirror). Returned as the
    /// 2x3 rows that map (s, t, 1) -> (s', t'); only the Z rotation acts on 2D texture coordinates.
    pub fn matrix(&self) -> [[f32; 3]; 2] {
        let eps = 1.0e-10;
        let sx = if self.scale[0].abs() < eps { 0.0 } else { self.repeat[0] as f32 / self.scale[0] };
        let sy = if self.scale[1].abs() < eps { 0.0 } else { self.repeat[1] as f32 / self.scale[1] };
        let tx = -self.translate[0];
        let mirror = if self.wrap_t_mirror && sy != 0.0 { 1.0 / sy } else { 0.0 };
        let ty = -(self.translate[1] + mirror);
        let (s, c) = (-self.rotate[2]).sin_cos();
        // rows of S * R * T applied to (u, v, 1)
        [[sx * c, -sx * s, sx * (c * tx - s * ty)], [sy * s, sy * c, sy * (s * tx + c * ty)]]
    }
    pub fn apply(m: &[[f32; 3]; 2], uv: [f32; 2]) -> [f32; 2] {
        [m[0][0] * uv[0] + m[0][1] * uv[1] + m[0][2], m[1][0] * uv[0] + m[1][1] * uv[1] + m[1][2]]
    }
}

impl Material {
    /// RENDER_DIFFUSE_BITS: 0/1 material colour, 2 vertex colour, 3 both.
    pub fn diffuse_source(&self) -> u32 { self.render_mode & 3 }
    /// Material alpha takes part (`MObjMakeTExp`): RENDER_ALPHA_BITS (13..14) MAT or BOTH, where COMPAT (0)
    /// follows the diffuse bits (MAT0 counting as MAT). VTX alpha comes from the vertex colours only.
    pub fn alpha_from_material(&self) -> bool {
        let diffuse = match self.render_mode & 3 { 0 => 1, d => d };
        let alpha = match (self.render_mode >> 13) & 3 { 0 => diffuse, a => a };
        alpha == 1 || alpha == 3
    }
    /// RENDER_DIFFUSE (bit 2): lighting enabled.
    pub fn lit(&self) -> bool { self.render_mode & (1 << 2) != 0 }
}

#[derive(Debug, Clone)]
pub struct Texture {
    /// Image data offset (textures are shared by offset).
    pub key: usize,
    pub format: u32,
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
}

impl Texture {
    pub fn has_alpha(&self) -> bool { self.rgba.chunks(4).any(|p| p[3] < 250) }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub joint: usize,
    pub hidden: bool,
    pub cull_back: bool,
    pub cull_front: bool,
    pub material: Material,
    /// Non-indexed triangle list (3 vertices per triangle), positions in model space.
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    /// GX_VA_NBT binormal and tangent per vertex in model space (through the normal matrix, not normalised:
    /// their length scales the emboss bump offset); empty when the vertex format has no NBT.
    pub binormals: Vec<[f32; 3]>,
    pub tangents: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    /// Raw texture coordinates per GX_VA_TEX0..7 (empty when the vertex format lacks that set).
    pub uv_sets: Vec<Vec<[f32; 2]>>,
    pub colors: Vec<[f32; 4]>,
    /// Per vertex: up to 4 joint indices (into `Model::joints`) and weights (sum 1). Positions are in the
    /// bind pose, so a skinned position is `sum(w * world_anim[j] * inverse(world_bind[j]) * p)`.
    pub skin_joints: Vec<[u16; 4]>,
    pub skin_weights: Vec<[f32; 4]>,
}

#[derive(Debug, Clone, Default)]
pub struct Model {
    pub joints: Vec<Joint>,
    /// Joint struct offset -> index of its first (non-instanced) occurrence in `joints`.
    pub joint_index: HashMap<usize, usize>,
    pub meshes: Vec<Mesh>,
    pub textures: Vec<Texture>,
    pub warnings: Vec<String>,
    /// Per model set (`scene_data` list order): index of its root joint in `joints`. A set's joints run up
    /// to the next set's first joint (a map object's resource id ends in its set index).
    pub set_first_joint: Vec<usize>,
    /// Material/texture animations (filled by [`crate::hsdanim::load_mat_animations`] when wanted).
    pub mats: std::sync::Arc<crate::hsdanim::MatAnims>,
}

impl Model {
    /// The model set a joint belongs to.
    pub fn set_of_joint(&self, j: usize) -> usize {
        // the first set whose range starts at the greatest first joint <= j
        let mut best: Option<(usize, usize)> = None;
        for (i, &f) in self.set_first_joint.iter().enumerate() {
            if f <= j && best.is_none_or(|(_, bf)| f > bf) { best = Some((i, f)); }
        }
        best.map_or(0, |b| b.0)
    }
    pub fn triangle_count(&self) -> usize { self.meshes.iter().map(|m| m.positions.len() / 3).sum() }
    pub fn bounds(&self) -> Option<([f32; 3], [f32; 3])> {
        let mut it = self.meshes.iter().filter(|m| !m.hidden).flat_map(|m| m.positions.iter());
        let first = *it.next()?;
        Some(it.fold((first, first), |(mut lo, mut hi), p| {
            for k in 0..3 { lo[k] = lo[k].min(p[k]); hi[k] = hi[k].max(p[k]); }
            (lo, hi)
        }))
    }
}

#[derive(Clone, Copy)]
struct Attr { attr: u32, kind: u32, count: u32, ctype: u32, frac: u8, stride: usize, data: usize }

const VA_PNMTXIDX: u32 = 0;
const VA_POS: u32 = 9;
const VA_NRM: u32 = 10;
/// GX_VA_NBT: normal, binormal, tangent (the Pokémon bump-mapped meshes).
const VA_NBT: u32 = 25;
const VA_CLR0: u32 = 11;
const VA_TEX0: u32 = 13;
const VA_TEX7: u32 = 20;
const VA_NULL: u32 = 0xFF;

/// Build a static (bind-pose) model from `scene_data`.
pub fn load_model(arc: &Archive) -> Result<Model> {
    let scene = arc.root("scene_data").ok_or_else(|| Error::NotFound("scene_data root".into()))?;
    let mut b = Builder { arc, model: Model::default(), tex_cache: HashMap::new(), joint_index: HashMap::new(), tree_seen: Default::default() };
    if let Some(list) = arc.ptr(scene)? {
        let mut i = 0;
        while let Some(ms) = arc.ptr(list + 4 * i)? {
            // several model sets can share one joint tree (e.g. separate animation sets); load it once
            let mut first = b.model.joints.len();
            if let Some(j) = arc.ptr(ms)? {
                if !b.tree_seen.contains(&j) { b.walk_joints(j, None, &IDENTITY)?; }
                else if let Some(&k) = b.joint_index.get(&j) { first = k; }
            }
            b.model.set_first_joint.push(first);
            i += 1;
            if i > 256 { break; }
        }
    }
    // meshes after all joints exist (envelopes can reference any joint)
    let joints: Vec<(usize, usize)> = b.model.joints.iter().enumerate().map(|(i, j)| (i, j.offset)).collect();
    for (ji, off) in joints {
        let flags = arc.u32(off + 4)?;
        if flags & (JOBJ_PTCL | JOBJ_SPLINE) != 0 { continue; }
        let mut d = arc.ptr(off + 0x10)?;
        let mut guard = 0;
        while let Some(dobj) = d {
            b.dobj(dobj, ji)?;
            d = arc.ptr(dobj + 4)?;
            guard += 1;
            if guard > 4096 { break; }
        }
    }
    b.model.joint_index = b.joint_index.clone();
    Ok(b.model)
}

struct Builder<'a, 'b> {
    arc: &'b Archive<'a>,
    model: Model,
    tex_cache: HashMap<(usize, usize), usize>,
    joint_index: HashMap<usize, usize>,
    tree_seen: std::collections::HashSet<usize>,
}

impl Builder<'_, '_> {
    /// Iterative depth-first walk (child before next sibling, like the HSD runtime), with cycle protection.
    fn walk_joints(&mut self, root: usize, parent: Option<usize>, parent_world: &Mat4) -> Result<()> {
        // (joint, parent index, parent world, reached through an instance edge, follow `next`)
        let mut stack: Vec<(usize, Option<usize>, Mat4, bool, bool)> = vec![(root, parent, *parent_world, false, true)];
        let mut visits: HashMap<usize, u32> = HashMap::new();
        while let Some((j, parent, pw, via_instance, follow_next)) = stack.pop() {
            // the instanced copy may be reached before the original, so track normal visits separately
            if !via_instance && !self.tree_seen.insert(j) {
                self.model.warnings.push(format!("joint {j:#x} reached twice outside an instance; skipped"));
                continue;
            }
            let v = visits.entry(j).or_insert(0);
            *v += 1;
            if *v > 64 {
                self.model.warnings.push(format!("joint {j:#x} instanced more than 64 times; stopping there"));
                continue;
            }
            if self.model.joints.len() > 10_000 {
                return Err(Error::Invalid("more than 10000 joints".into()));
            }
            let a = self.arc;
            let flags = a.u32(j + 4)?;
            let rot = a.vec3(j + 0x14)?;
            let scale = a.vec3(j + 0x20)?;
            let pos = a.vec3(j + 0x2C)?;
            let world = mul(&pw, &srt(scale, rot, pos));
            let inverse_bind = match a.ptr(j + 0x38)? {
                Some(p) => {
                    let mut m = IDENTITY;
                    for r in 0..3 { for c in 0..4 { m[r][c] = a.f32(p + 4 * (r * 4 + c))?; } }
                    Some(m)
                }
                None => None,
            };
            let name = match a.ptr(j)? { Some(p) => cstr_at(a.data, p, "joint name").unwrap_or_default(), None => String::new() };
            let mut robj = vec![];
            let mut rd = a.ptr(j + 0x3C)?;
            while let Some(r) = rd {
                let rflags = a.u32(r + 4)?;
                let kind = match rflags & 0x7000_0000 {
                    REFTYPE_IKHINT => match a.ptr(r + 8)? {
                        Some(h) => RObjKind::IkHint { bone_length: a.f32(h)?, rotate_x: a.f32(h + 4)? },
                        None => RObjKind::Other,
                    },
                    REFTYPE_LIMIT => RObjKind::Limit(a.f32(r + 8)?),
                    REFTYPE_JOBJ => match a.ptr(r + 8)? { Some(t) => RObjKind::Joint(t), None => RObjKind::Other },
                    _ => RObjKind::Other,
                };
                robj.push(RObj { flags: rflags, kind });
                rd = a.ptr(r)?;
                if robj.len() > 32 { break; }
            }
            let idx = self.model.joints.len();
            // envelopes reference joints by offset: keep the first (original) occurrence
            self.joint_index.entry(j).or_insert(idx);
            self.model.joints.push(Joint { offset: j, name, flags, parent, world, inverse_bind, rest_scale: scale, rest_rotation: rot, rest_translation: pos, robj });
            // push sibling first so the child subtree is visited first. An instance joint's `child` is the
            // instanced joint: draw it (and its children) again here, but not its siblings.
            if follow_next { if let Some(n) = a.ptr(j + 0xC)? { stack.push((n, parent, pw, via_instance, true)); } }
            if let Some(c) = a.ptr(j + 8)? {
                let inst = flags & JOBJ_INSTANCE != 0;
                stack.push((c, Some(idx), world, via_instance || inst, !inst));
            }
        }
        Ok(())
    }

    fn inv_bind(&self, mut ji: usize) -> Mat4 {
        loop {
            let j = &self.model.joints[ji];
            if let Some(m) = j.inverse_bind { return m; }
            match j.parent { Some(p) => ji = p, None => return IDENTITY }
        }
    }

    /// Envelope coordinate system for the owning joint (Blender addon `_envelope_coord_system`).
    fn envelope_coord(&self, ji: usize) -> Option<Mat4> {
        let joints = &self.model.joints;
        if joints[ji].flags & JOBJ_SKELETON_ROOT != 0 { return None; }
        let mut k = Some(ji);
        let mut skel = None;
        while let Some(i) = k {
            if joints[i].flags & (JOBJ_SKELETON | JOBJ_SKELETON_ROOT) != 0 { skel = Some(i); break; }
            k = joints[i].parent;
        }
        let skel = skel?;
        let inv_bind = self.inv_bind(ji);
        let world = joints[ji].world;
        if skel == ji { return Some(invert_affine(&inv_bind)); }
        let sw = joints[skel].world;
        if joints[skel].flags & JOBJ_SKELETON_ROOT != 0 {
            Some(mul(&invert_affine(&sw), &world))
        } else {
            Some(mul(&invert_affine(&mul(&sw, &inv_bind)), &world))
        }
    }

    fn dobj(&mut self, dobj: usize, ji: usize) -> Result<()> {
        let a = self.arc;
        let material = match a.ptr(dobj + 8)? { Some(m) => self.mobj(m)?, None => default_material() };
        let mut p = a.ptr(dobj + 0xC)?;
        let mut guard = 0;
        while let Some(pobj) = p {
            if let Err(e) = self.pobj(pobj, ji, &material) {
                self.model.warnings.push(format!("pobj {pobj:#x}: {e}"));
            }
            p = a.ptr(pobj + 4)?;
            guard += 1;
            if guard > 4096 { break; }
        }
        Ok(())
    }

    fn mobj(&mut self, m: usize) -> Result<Material> {
        let a = self.arc;
        let render_mode = a.u32(m + 4)?;
        let mut mat = default_material();
        mat.mobj = m;
        mat.render_mode = render_mode;
        mat.translucent = render_mode & RENDER_XLU != 0;
        mat.has_pixel_engine = a.ptr(m + 0x14)?.is_some();
        if let Some(pe) = a.ptr(m + 0x14)? {
            mat.pe = Some(crate::be::bytes_at(a.data, pe, 12, "pe desc")?.try_into().unwrap());
        }
        if let Some(mt) = a.ptr(m + 0xC)? {
            let c = |o: usize| -> Result<[f32; 4]> { let b = crate::be::bytes_at(a.data, mt + o, 4, "material")?; Ok([b[0] as f32 / 255.0, b[1] as f32 / 255.0, b[2] as f32 / 255.0, b[3] as f32 / 255.0]) };
            mat.ambient = c(0)?;
            mat.diffuse = c(4)?;
            mat.specular = c(8)?;
            mat.alpha = a.f32(mt + 0xC)?;
            mat.shininess = a.f32(mt + 0x10)?;
        }
        let mut t = a.ptr(m + 8)?;
        let mut guard = 0;
        while let Some(tobj) = t {
            // only the first texture with UV coords (src 0..=3 are TEXCOORD0..; we take the first) is used for now
            // the base colour texture: a UV-mapped TObj on the diffuse light map (flags 0x40: coord bits 0..3 = 0
            // UV, bit 4 TEX_LIGHTMAP_DIFFUSE). Reflection/highlight maps (coord 1/2, TEX_LIGHTMAP_EXT) are skipped:
            // they have no UVs of their own and only add a sheen on top of the material colour.
            let tflags = a.u32(tobj + 0x40)?;
            let wrap = |v: u32| match v { 1 => Wrap::Repeat, 2 => Wrap::Mirror, _ => Wrap::Clamp };
            let texture = match self.texture(tobj) {
                Ok(t) => t,
                Err(e) => { self.model.warnings.push(format!("tobj {tobj:#x}: {e}")); None }
            };
            let tev = match a.ptr(tobj + 0x58)? {
                Some(t) => {
                    let b = crate::be::bytes_at(a.data, t, 0x20, "tobj tev")?;
                    let c4 = |o: usize| [b[o], b[o + 1], b[o + 2], b[o + 3]];
                    Some(TObjTev { color_op: b[0], alpha_op: b[1], color_bias: b[2], alpha_bias: b[3], color_scale: b[4], alpha_scale: b[5],
                        color_clamp: b[6], alpha_clamp: b[7], color_in: c4(8), alpha_in: c4(12), konst: c4(16), tev0: c4(20), tev1: c4(24),
                        active: u32::from_be_bytes(c4(28)) })
                }
                None => None,
            };
            mat.layers.push(TexLayer { tobj, id: a.u32(tobj + 8)?, src: a.u32(tobj + 0xC)?, flags: tflags, blending: a.f32(tobj + 0x44)?, texture,
                wrap: [wrap(a.u32(tobj + 0x34)?), wrap(a.u32(tobj + 0x38)?)], srt: tex_srt(a, tobj)?, tev });
            if mat.texture.is_none() && tflags & 0xF == 0 && tflags & 0x10 != 0 {
                if let Some(ti) = texture {
                    let l = mat.layers.last().unwrap();
                    mat.texture = Some(ti);
                    mat.wrap = l.wrap;
                    mat.tobj = Some(tobj);
                    mat.tex_srt = l.srt;
                }
            }
            mat.texture_count += 1;
            t = a.ptr(tobj + 4)?;
            guard += 1;
            if guard > 16 { break; }
        }
        Ok(mat)
    }

    fn texture(&mut self, tobj: usize) -> Result<Option<usize>> {
        let a = self.arc;
        let Some(img) = a.ptr(tobj + 0x4C)? else { return Ok(None) };
        let data = a.u32(img)? as usize;
        let pal_ptr = a.ptr(tobj + 0x50)?;
        let key = (data, pal_ptr.unwrap_or(0));
        if let Some(&i) = self.tex_cache.get(&key) { return Ok(Some(i)); }
        let tex = decode_image(a, img, pal_ptr)?;
        let i = self.model.textures.len();
        self.model.textures.push(tex);
        self.tex_cache.insert(key, i);
        Ok(Some(i))
    }

    fn pobj(&mut self, pobj: usize, ji: usize, material: &Material) -> Result<()> {
        let a = self.arc;
        let tex_m = if material.tobj.is_some() { material.tex_srt.matrix() } else { [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] };
        let flags = u16_at(a.data, pobj + 0xC, "pobj")?;
        let blocks = u16_at(a.data, pobj + 0xE, "pobj")? as usize;
        let dl = a.u32(pobj + 0x10)? as usize;
        let property = a.ptr(pobj + 0x14)?;
        let mut attrs = vec![];
        if let Some(mut v) = a.ptr(pobj + 8)? {
            loop {
                let attr = a.u32(v)?;
                if attr == VA_NULL || attrs.len() > 26 { break; }
                attrs.push(Attr { attr, kind: a.u32(v + 4)?, count: a.u32(v + 8)?, ctype: a.u32(v + 0xC)?,
                    frac: u8_at(a.data, v + 0x10, "vtx")?, stride: u16_at(a.data, v + 0x12, "vtx")? as usize, data: a.u32(v + 0x14)? as usize });
                v += 0x18;
            }
        }
        if !attrs.iter().any(|x| x.attr == VA_POS) { return Ok(()); }

        // per-vertex transform
        let ptype = flags & POBJ_TYPE_MASK;
        let owner_world = self.model.joints[ji].world;
        let mut envelopes: Vec<Mat4> = vec![];
        let mut env_entries: Vec<Vec<(usize, f32)>> = vec![];
        let mut rigid = owner_world;
        let mut rigid_joint = ji;
        if let Some(prop) = property {
            if ptype == POBJ_ENVELOPE {
                let coord = self.envelope_coord(ji);
                let mut e = 0;
                while let Some(env) = a.ptr(prop + 4 * e)? {
                    let mut entries = vec![];
                    let mut k = 0;
                    while let Some(jo) = a.ptr(env + 8 * k)? {
                        entries.push((self.joint_index.get(&jo).copied().unwrap_or(0), a.f32(env + 8 * k + 4)?));
                        k += 1;
                        if k > 16 { break; }
                    }
                    let mut m = if entries.len() == 1 && entries[0].1 == 1.0 {
                        let (j, _) = entries[0];
                        if coord.is_some() { mul(&self.model.joints[j].world, &self.inv_bind(j)) } else { self.model.joints[j].world }
                    } else {
                        let mut acc = [[0.0f32; 4]; 4];
                        for &(j, w) in &entries {
                            let c = mul(&self.model.joints[j].world, &self.inv_bind(j));
                            for r in 0..4 { for cc in 0..4 { acc[r][cc] += w * c[r][cc]; } }
                        }
                        acc
                    };
                    if let Some(c) = coord { m = mul(&m, &c); }
                    envelopes.push(m);
                    env_entries.push(entries);
                    e += 1;
                    if e > 256 { break; }
                }
            } else if ptype == POBJ_SKIN {
                if let Some(&pj) = self.joint_index.get(&prop) { rigid = self.model.joints[pj].world; rigid_joint = pj; }
            }
        }

        let mut mesh = Mesh {
            joint: ji, hidden: self.model.joints[ji].flags & JOBJ_HIDDEN != 0,
            cull_front: flags & (1 << 14) != 0, cull_back: flags & (1 << 15) != 0,
            material: material.clone(), positions: vec![], normals: vec![], binormals: vec![], tangents: vec![], uvs: vec![], uv_sets: vec![vec![]; 8], colors: vec![],
            skin_joints: vec![], skin_weights: vec![],
        };
        let present: Vec<bool> = (0..8).map(|k| attrs.iter().any(|x| x.attr == VA_TEX0 + k)).collect();
        let dl_bytes = crate::be::bytes_at(a.data, dl, blocks * 32, "display list")?;
        let mut p = 0usize;
        while p < dl_bytes.len() {
            let op = dl_bytes[p];
            if op == 0 { break; }
            let n = u16::from_be_bytes([*dl_bytes.get(p + 1).unwrap_or(&0), *dl_bytes.get(p + 2).unwrap_or(&0)]) as usize;
            p += 3;
            let mut verts = Vec::with_capacity(n);
            for _ in 0..n {
                let mut v = Vtx::default();
                for at in &attrs {
                    let val_off = match at.kind {
                        1 => { let o = dl + p; p += direct_size(at); Some((o, true)) }
                        2 => { let i = dl_bytes[p] as usize; p += 1; Some((at.data + i * at.stride, false)) }
                        3 => { let i = u16::from_be_bytes([dl_bytes[p], dl_bytes[p + 1]]) as usize; p += 2; Some((at.data + i * at.stride, false)) }
                        _ => None,
                    };
                    let Some((off, direct)) = val_off else { continue };
                    match at.attr {
                        VA_PNMTXIDX => v.mtx = u8_at(a.data, off, "pnmtx")? as usize / 3,
                        VA_POS => v.pos = read_vec(a.data, off, at, if at.count == 0 { 2 } else { 3 })?,
                        VA_NRM | VA_NBT => {
                            v.nrm = Some(read_vec(a.data, off, at, 3)?);
                            // GX_NRM_NBT (count 1): normal, binormal, tangent back to back; GX_NRM_NBT3 (2): one index each
                            if at.count == 1 || at.count == 2 {
                                let mut nbt = [[0.0; 3]; 2];
                                for (k, x) in nbt.iter_mut().enumerate() {
                                    let o = if at.count == 1 || direct { off + (k + 1) * 3 * comp_size(at.ctype) } else {
                                        let i = if at.kind == 2 { let i = dl_bytes[p] as usize; p += 1; i } else { let i = u16::from_be_bytes([dl_bytes[p], dl_bytes[p + 1]]) as usize; p += 2; i };
                                        at.data + i * at.stride + (k + 1) * 3 * comp_size(at.ctype)
                                    };
                                    *x = read_vec(a.data, o, at, 3)?;
                                }
                                v.nbt = Some(nbt);
                            }
                        }
                        VA_CLR0 => v.clr = Some(read_color(a.data, off, at.ctype, direct)?),
                        VA_TEX0..=VA_TEX7 => {
                            let t = read_vec(a.data, off, at, if at.count == 0 { 1 } else { 2 })?;
                            v.uvs[(at.attr - VA_TEX0) as usize] = Some([t[0], t[1]]);
                            if at.attr == VA_TEX0 { v.uv = Some([t[0], t[1]]); }
                        }
                        _ => {}
                    }
                }
                verts.push(v);
            }
            let tri = |i: usize, j: usize, k: usize, out: &mut Vec<[usize; 3]>| out.push([i, j, k]);
            let mut tris: Vec<[usize; 3]> = vec![];
            match op & 0xF8 {
                0x90 => for t in 0..n / 3 { tri(3 * t, 3 * t + 1, 3 * t + 2, &mut tris) },
                // GX strips/fans/quads wind clockwise; emitted reversed for CCW front faces
                0x98 => for t in 0..n.saturating_sub(2) { if t % 2 == 0 { tri(t, t + 1, t + 2, &mut tris) } else { tri(t + 1, t, t + 2, &mut tris) } },
                0xA0 => for t in 1..n.saturating_sub(1) { tri(0, t, t + 1, &mut tris) },
                0x80 => for q in 0..n / 4 { tri(4 * q, 4 * q + 1, 4 * q + 2, &mut tris); tri(4 * q, 4 * q + 2, 4 * q + 3, &mut tris) },
                0x88 => for q in 0..n / 4 { tri(4 * q, 4 * q + 1, 4 * q + 2, &mut tris); tri(4 * q, 4 * q + 2, 4 * q + 3, &mut tris) },
                _ => {} // lines/points ignored
            }
            for t in tris {
                // GX front faces are clockwise; flip to counter-clockwise
                for &i in &[t[0], t[2], t[1]] {
                    let v = &verts[i];
                    let m = if !envelopes.is_empty() { envelopes.get(v.mtx).unwrap_or(&IDENTITY) } else { &rigid };
                    mesh.positions.push(transform_point(m, v.pos));
                    mesh.normals.push(v.nrm.map(|n| transform_dir(m, n)).unwrap_or([0.0, 1.0, 0.0]));
                    if let Some([b, t]) = v.nbt {
                        mesh.binormals.push(transform_nrm_raw(m, b));
                        mesh.tangents.push(transform_nrm_raw(m, t));
                    }
                    let uv = v.uv.unwrap_or([0.0, 0.0]);
                    mesh.uvs.push(TexSrt::apply(&tex_m, uv));
                    for (k, set) in mesh.uv_sets.iter_mut().enumerate() { if present[k] { set.push(v.uvs[k].unwrap_or([0.0, 0.0])); } }
                    mesh.colors.push(v.clr.unwrap_or([1.0, 1.0, 1.0, 1.0]));
                    let (sj, sw) = if !envelopes.is_empty() {
                        skin4(env_entries.get(v.mtx).map(|e| e.as_slice()).unwrap_or(&[]), rigid_joint)
                    } else {
                        ([rigid_joint as u16, 0, 0, 0], [1.0, 0.0, 0.0, 0.0])
                    };
                    mesh.skin_joints.push(sj);
                    mesh.skin_weights.push(sw);
                }
            }
        }
        if !mesh.positions.is_empty() { self.model.meshes.push(mesh); }
        Ok(())
    }
}

/// Keep the 4 largest weights, renormalized.
fn skin4(entries: &[(usize, f32)], fallback: usize) -> ([u16; 4], [f32; 4]) {
    if entries.is_empty() { return ([fallback as u16, 0, 0, 0], [1.0, 0.0, 0.0, 0.0]); }
    let mut e: Vec<(usize, f32)> = entries.to_vec();
    e.sort_by(|a, b| b.1.total_cmp(&a.1));
    e.truncate(4);
    let sum: f32 = e.iter().map(|x| x.1).sum();
    let mut j = [0u16; 4];
    let mut w = [0f32; 4];
    for (i, (jj, ww)) in e.iter().enumerate() { j[i] = *jj as u16; w[i] = if sum > 0.0 { ww / sum } else { 0.25 }; }
    (j, w)
}

#[derive(Default)]
struct Vtx { mtx: usize, pos: [f32; 3], nrm: Option<[f32; 3]>, nbt: Option<[[f32; 3]; 2]>, clr: Option<[f32; 4]>, uv: Option<[f32; 2]>, uvs: [Option<[f32; 2]>; 8] }

/// Read a TObj's texture SRT.
pub fn tex_srt(a: &Archive, tobj: usize) -> Result<TexSrt> {
    Ok(TexSrt {
        rotate: a.vec3(tobj + 0x10)?,
        scale: a.vec3(tobj + 0x1C)?,
        translate: a.vec3(tobj + 0x28)?,
        wrap_t_mirror: a.u32(tobj + 0x38)? == 2,
        repeat: [u8_at(a.data, tobj + 0x3C, "tobj")?, u8_at(a.data, tobj + 0x3D, "tobj")?],
    })
}

/// Decode an `HSD_ImageDesc` (data 0, u16 width 4, u16 height 6, format 8) with an optional
/// `HSD_TlutDesc` (data 0, format 4, u16 entries 0xC).
pub fn decode_image(a: &Archive, img: usize, tlut: Option<usize>) -> Result<Texture> {
    let data = a.u32(img)? as usize;
    let w = u16_at(a.data, img + 4, "image")? as usize;
    let h = u16_at(a.data, img + 6, "image")? as usize;
    let format = a.u32(img + 8)?;
    let size = gx::data_size(format, w, h).ok_or_else(|| Error::Invalid(format!("texture format {format}")))?;
    let bytes = crate::be::bytes_at(a.data, data, size, "texture data")?;
    let palette = match tlut {
        Some(p) => {
            let pd = a.u32(p)? as usize;
            let pf = a.u32(p + 4)?;
            let n = u16_at(a.data, p + 0xC, "palette")? as usize;
            Some(gx::decode_palette(crate::be::bytes_at(a.data, pd, n * 2, "palette data")?, pf, n)?)
        }
        None => None,
    };
    let rgba = gx::decode(bytes, format, w, h, palette.as_deref())?;
    Ok(Texture { key: data, format, width: w, height: h, rgba })
}

pub(crate) fn default_material() -> Material {
    Material { render_mode: 0, diffuse: [1.0; 4], ambient: [0.5, 0.5, 0.5, 1.0], alpha: 1.0, texture: None, texture_count: 0,
        wrap: [Wrap::Repeat, Wrap::Repeat], mobj: 0, tobj: None, tex_srt: TexSrt::default(), translucent: false, has_pixel_engine: false,
        specular: [1.0; 4], shininess: 50.0, layers: vec![], pe: None }
}

fn comp_size(ctype: u32) -> usize { match ctype { 0 | 1 => 1, 2 | 3 => 2, _ => 4 } }

fn direct_size(at: &Attr) -> usize {
    match at.attr {
        0..=8 => 1,
        VA_POS => comp_size(at.ctype) * if at.count == 0 { 2 } else { 3 },
        VA_NRM | VA_NBT => comp_size(at.ctype) * if at.count == 0 { 3 } else { 9 },
        11 | 12 => match at.ctype { 0 | 3 => 2, 1 | 4 => 3, _ => 4 },
        _ => comp_size(at.ctype) * if at.count == 0 { 1 } else { 2 },
    }
}

fn read_vec(d: &[u8], off: usize, at: &Attr, n: usize) -> Result<[f32; 3]> {
    let mut out = [0.0f32; 3];
    let scale = 1.0 / (1u32 << at.frac) as f32;
    // normals use fixed fractions: s8 -> 6, s16 -> 14 bits
    let scale = if at.attr == VA_NRM || at.attr == VA_NBT { match at.ctype { 1 => 1.0 / 64.0, 3 => 1.0 / 16384.0, _ => scale } } else { scale };
    for (k, o) in out.iter_mut().enumerate().take(n) {
        let p = off + k * comp_size(at.ctype);
        *o = match at.ctype {
            0 => u8_at(d, p, "vtx")? as f32 * scale,
            1 => u8_at(d, p, "vtx")? as i8 as f32 * scale,
            2 => u16_at(d, p, "vtx")? as f32 * scale,
            3 => u16_at(d, p, "vtx")? as i16 as f32 * scale,
            _ => f32::from_bits(u32_at(d, p, "vtx")?),
        };
    }
    Ok(out)
}

fn read_color(d: &[u8], off: usize, ctype: u32, _direct: bool) -> Result<[f32; 4]> {
    let b = |i: usize| -> Result<f32> { Ok(u8_at(d, off + i, "color")? as f32) };
    Ok(match ctype {
        0 => { let v = u16_at(d, off, "color")?; [((v >> 11) & 31) as f32 / 31.0, ((v >> 5) & 63) as f32 / 63.0, (v & 31) as f32 / 31.0, 1.0] }
        1 | 2 => [b(0)? / 255.0, b(1)? / 255.0, b(2)? / 255.0, 1.0],
        3 => { let v = u16_at(d, off, "color")?; [((v >> 12) & 15) as f32 / 15.0, ((v >> 8) & 15) as f32 / 15.0, ((v >> 4) & 15) as f32 / 15.0, (v & 15) as f32 / 15.0] }
        4 => { let v = ((b(0)? as u32) << 16) | ((b(1)? as u32) << 8) | b(2)? as u32;
               [((v >> 18) & 63) as f32 / 63.0, ((v >> 12) & 63) as f32 / 63.0, ((v >> 6) & 63) as f32 / 63.0, (v & 63) as f32 / 63.0] }
        _ => [b(0)? / 255.0, b(1)? / 255.0, b(2)? / 255.0, b(3)? / 255.0],
    })
}

/// World matrices for every joint with per-joint local SRT overrides (`locals[i]` = (scale, rot, trans)).
/// Joints are stored parent-before-child. IK chains (JOINT1 / JOINT2 / EFFECTOR with IK-hint and target RObjs)
/// and position constraints are solved like sysdolphin's `JObjResolveIK` / RObj update: a first plain pass places the IK targets, a second pass
/// rebuilds every joint with JOINT1/JOINT2 orientations replaced by the two-bone solution.
pub fn world_matrices(model: &Model, locals: &[([f32; 3], [f32; 3], [f32; 3])]) -> Vec<Mat4> {
    let n = model.joints.len();
    let plain = |w: &[Mat4], i: usize| -> Mat4 {
        let (s, r, t) = locals[i];
        let local = srt(s, r, t);
        match model.joints[i].parent { Some(p) if p < w.len() => mul(&w[p], &local), _ => local }
    };
    let mut w: Vec<Mat4> = Vec::with_capacity(n);
    for i in 0..n { let m = plain(&w, i); w.push(m); }
    let constrained = |j: &Joint| j.ik_type() == 0 && j.robj(REFTYPE_JOBJ, 1).is_some();
    if !model.joints.iter().any(|j| j.ik_type() == 1 || constrained(j)) { return w; }

    let first_pass = w.clone();
    let mut children: Vec<Vec<usize>> = vec![vec![]; n];
    for (i, j) in model.joints.iter().enumerate() { if let Some(p) = j.parent { if p < i { children[p].push(i); } } }
    let target_of = |ji: usize, sub: u32, w: &[Mat4]| -> Option<[f32; 3]> {
        match model.joints[ji].robj(REFTYPE_JOBJ, sub)?.kind {
            RObjKind::Joint(off) => {
                let t = *model.joint_index.get(&off)?;
                let m = if t < w.len() { &w[t] } else { &first_pass[t] };
                Some([m[0][3], m[1][3], m[2][3]])
            }
            _ => None,
        }
    };
    let find = |of: &[usize], ty: u32| of.iter().copied().find(|&c| model.joints[c].ik_type() == ty);
    // effector among `of` that has a target constraint, and that target's world position
    let effector_target = |of: &[usize], w: &[Mat4]| -> Option<[f32; 3]> {
        of.iter().copied().filter(|&c| model.joints[c].ik_type() == 3).find_map(|c| target_of(c, 1, w))
    };

    let mut w: Vec<Mat4> = Vec::with_capacity(n);
    for i in 0..n {
        let j = &model.joints[i];
        let solved = match j.ik_type() {
            1 => ik_joint1(model, i, locals, &w, &children, &find, &effector_target, &target_of, plain(&w, i)),
            2 => ik_joint2(model, i, locals, &w, &children, &effector_target),
            _ => None,
        };
        let mut m = solved.unwrap_or_else(|| plain(&w, i));
        // position constraint (RObj JOBJ subtype 1 on an ordinary joint, e.g. a held prop): take the
        // referenced joint's world position, keep this joint's orientation
        if constrained(j) {
            if let Some(t) = target_of(i, 1, &w) { for r in 0..3 { m[r][3] = t[r]; } }
        }
        w.push(m);
    }
    w
}

type V3 = [f32; 3];
type FindFn<'a> = &'a dyn Fn(&[usize], u32) -> Option<usize>;
type EffectorFn<'a> = &'a dyn Fn(&[usize], &[Mat4]) -> Option<V3>;
type TargetFn<'a> = &'a dyn Fn(usize, u32, &[Mat4]) -> Option<V3>;
fn v_sub(a: V3, b: V3) -> V3 { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn v_add(a: V3, b: V3) -> V3 { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
fn v_scale(a: V3, s: f32) -> V3 { [a[0] * s, a[1] * s, a[2] * s] }
fn v_dot(a: V3, b: V3) -> f32 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn v_cross(a: V3, b: V3) -> V3 { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }
fn v_norm(a: V3) -> V3 { v_scale(a, 1.0 / (1.0e-10 + v_dot(a, a)).sqrt()) }
fn column(m: &Mat4, c: usize) -> V3 { [m[0][c], m[1][c], m[2][c]] }
fn translation(m: &Mat4) -> V3 { column(m, 3) }
/// Rotate `v` about `axis` by `angle` (Rodrigues).
fn rotate_axis(v: V3, axis: V3, angle: f32) -> V3 {
    let k = v_norm(axis);
    let (s, c) = angle.sin_cos();
    v_add(v_add(v_scale(v, c), v_scale(v_cross(k, v), s)), v_scale(k, v_dot(k, v) * (1.0 - c)))
}
fn from_columns(x: V3, y: V3, z: V3, t: V3) -> Mat4 {
    [[x[0], y[0], z[0], t[0]], [x[1], y[1], z[1], t[1]], [x[2], y[2], z[2], t[2]], [0.0, 0.0, 0.0, 1.0]]
}

/// sysdolphin `JObjResolveIKJoint1` (0x8019E460): place the first bone so a two-bone chain reaches the target.
#[allow(clippy::too_many_arguments)]
fn ik_joint1(
    model: &Model, i: usize, locals: &[([f32; 3], [f32; 3], [f32; 3])], w: &[Mat4], children: &[Vec<usize>],
    find: FindFn, effector_target: EffectorFn, target_of: TargetFn, current: Mat4,
) -> Option<Mat4> {
    let j = &model.joints[i];
    let scale = locals[i].0;
    let (len1, rotate_x, _) = j.ik_hint()?;
    let first_len = len1 * scale[0];
    let (second_len, flip, target) = match find(&children[i], 2) {
        Some(j2) => {
            let (len2, _, hflags) = model.joints[j2].ik_hint()?;
            (len2 * locals[j2].0[0] * scale[0], hflags & 4 != 0, effector_target(&children[j2], w)?)
        }
        None => (0.0, false, effector_target(&children[i], w)?),
    };
    let origin = match j.parent { Some(p) if p < w.len() => translation(&w[p]), _ => [0.0; 3] };
    let target_dir = v_sub(target, origin);
    let dist_sq = v_dot(target_dir, target_dir);
    let (first_sq, second_sq) = (first_len * first_len, second_len * second_len);
    let (mut height_sq, mut axial_len, mut height_len) = (0.0f32, 0.0f32, first_len);
    let (mut normal, mut bend) = (column(&current, 2), [0.0f32; 3]);
    if dist_sq > 1.0e-8 {
        let t = target_dir;
        let mut pole;
        if let Some(ph) = target_of(i, 3, w) {
            pole = v_sub(ph, origin);
            if rotate_x != 0.0 { pole = rotate_axis(pole, t, rotate_x); }
            normal = v_cross(t, pole);
            pole = v_cross(normal, t);
        } else {
            pole = v_cross(normal, t);
            normal = v_cross(t, pole);
        }
        normal = v_norm(normal);
        bend = v_norm(pole);
        let diff_sq = first_sq - second_sq;
        height_sq = (0.25 * ((2.0 * (first_sq + second_sq) - dist_sq) - diff_sq * diff_sq / dist_sq)).max(0.0);
        axial_len = ((first_sq - height_sq) / dist_sq).max(0.0).sqrt();
        height_len = height_sq.sqrt();
    } else {
        normal = v_norm(normal);
    }
    if flip { height_len = -height_len; }
    let along = if second_sq - height_sq < dist_sq { v_scale(target_dir, axial_len) } else { v_scale(target_dir, -axial_len) };
    let x = v_norm(v_add(along, v_scale(bend, height_len)));
    let y = v_cross(normal, x);
    Some(from_columns(v_scale(x, scale[0]), v_scale(y, scale[1]), v_scale(normal, scale[2]), origin))
}

/// sysdolphin `JObjResolveIKJoint2`: second bone starts at the end of the first and points at the target.
fn ik_joint2(
    model: &Model, i: usize, locals: &[([f32; 3], [f32; 3], [f32; 3])], w: &[Mat4], children: &[Vec<usize>],
    effector_target: EffectorFn,
) -> Option<Mat4> {
    let j = &model.joints[i];
    let p = j.parent.filter(|&p| p < w.len())?;
    let target = effector_target(&children[i], w)?;
    let scale = locals[i].0;
    let pw = &w[p];
    let parent_x = v_norm(column(pw, 0));
    let (plen, _, _) = model.joints[p].ik_hint()?;
    let joint_pos = v_add(translation(pw), v_scale(parent_x, plen * locals[p].0[0]));
    let mut dir = v_norm(v_sub(target, joint_pos));
    let min = j.robj(REFTYPE_LIMIT, 5).and_then(|r| if let RObjKind::Limit(v) = r.kind { Some(v) } else { None });
    let max = j.robj(REFTYPE_LIMIT, 6).and_then(|r| if let RObjKind::Limit(v) = r.kind { Some(v) } else { None });
    if min.is_some() || max.is_some() {
        let flip = j.ik_hint().is_some_and(|h| h.2 & 4 != 0);
        let mut angle = v_dot(parent_x, dir).clamp(-1.0, 1.0).acos();
        if !flip { angle = -angle; }
        let clamped = match (min, max) {
            (Some(lo), _) if angle < lo => Some(lo),
            (_, Some(hi)) if angle > hi => Some(hi),
            _ => None,
        };
        if let Some(a) = clamped { dir = rotate_axis(parent_x, column(pw, 2), a); }
    }
    let bend = v_norm(v_cross(column(pw, 2), dir));
    let side = v_cross(dir, bend);
    Some(from_columns(v_scale(dir, scale[0]), v_scale(bend, scale[1]), v_scale(side, scale[2]), joint_pos))
}

/// Re-skin a model's meshes for the given joint world matrices (CPU), returning a posed copy.
pub fn pose(model: &Model, worlds: &[Mat4]) -> Model {
    let skin: Vec<Mat4> = model.joints.iter().zip(worlds).map(|(j, w)| mul(w, &invert_affine(&j.world))).collect();
    let mut out = model.clone();
    for m in &mut out.meshes {
        for (i, p) in m.positions.iter_mut().enumerate() {
            let (js, ws) = (m.skin_joints[i], m.skin_weights[i]);
            let mut acc = [0.0f32; 3];
            let mut nacc = [0.0f32; 3];
            for k in 0..4 {
                if ws[k] == 0.0 { continue; }
                let sm = &skin[js[k] as usize];
                let q = transform_point(sm, *p);
                let n = transform_dir(sm, m.normals[i]);
                for c in 0..3 { acc[c] += ws[k] * q[c]; nacc[c] += ws[k] * n[c]; }
            }
            *p = acc;
            m.normals[i] = nacc;
            for v in [m.binormals.get_mut(i), m.tangents.get_mut(i)].into_iter().flatten() {
                let mut vacc = [0.0f32; 3];
                for k in 0..4 {
                    if ws[k] == 0.0 { continue; }
                    let t = transform_nrm_raw(&skin[js[k] as usize], *v);
                    for c in 0..3 { vacc[c] += ws[k] * t[c]; }
                }
                *v = vacc;
            }
        }
    }
    out
}

/// The scene's camera (`scene_data` +4 -> camera set -> `HSD_CameraDesc`): eye and interest positions,
/// vertical field of view in degrees, near/far. Map `.rdat`s carry the room's default field camera here.
#[derive(Debug, Clone, Copy)]
pub struct SceneCamera {
    pub eye: [f32; 3],
    pub interest: [f32; 3],
    pub fov: f32,
    /// Perspective aspect (CObjDesc +0x34), applied whatever the viewport's shape.
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

pub fn scene_camera(arc: &Archive) -> Result<Option<SceneCamera>> {
    let Some(scene) = arc.root("scene_data") else { return Ok(None) };
    let Some(set) = arc.ptr(scene + 4)? else { return Ok(None) };
    let Some(cam) = arc.ptr(set)? else { return Ok(None) };
    // WObjDesc: class name +0, position +4
    let pos = |o: usize| -> Result<Option<[f32; 3]>> { Ok(match arc.ptr(o)? { Some(w) => Some(arc.vec3(w + 4)?), None => None }) };
    let (Some(eye), Some(interest)) = (pos(cam + 0x18)?, pos(cam + 0x1C)?) else { return Ok(None) };
    Ok(Some(SceneCamera { eye, interest, near: arc.f32(cam + 0x28)?, far: arc.f32(cam + 0x2C)?, fov: arc.f32(cam + 0x30)?, aspect: arc.f32(cam + 0x34)? }))
}

/// A scene light (`scene_data` +8 -> NULL-terminated LightSet list {LObjDesc*, LightAnim**}; `floorReadMapPostFunc`
/// 0x80114AE0 loads each with `GSlightLoad` as resource mapres|0x1600|i). LObjDesc (`LObjLoad` 0x801A4440): next +4,
/// flags u16 +8 (type bits 0-1: 0 ambient, 1 infinite, 2 point, 3 spot; 4 diffuse, 8 specular, 0x10 alpha, 0x20 hidden),
/// attn flags u16 +0xA (1 = raw GX attenuation), colour RGBA +0xC, position WObjDesc* +0x10, interest +0x14, params +0x18:
/// point {ref_br, ref_dist, dist_func}, spot {cutoff, spot_func, ref_br, ref_dist, dist_func}, raw {a0 a1 a2 k0 k1 k2}.
#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub flags: u16,
    /// LObjDesc +0xA bit 0: `params` are raw GX attenuation (LObj flag 0x40).
    pub raw: bool,
    pub color: [u8; 4],
    /// WObj positions (HSD_LObjSetup default 0,0,0).
    pub position: [f32; 3],
    pub interest: [f32; 3],
    /// LObj +0x20..+0x34 as `LObjLoad` fills them: cutoff, -, ref_br, ref_dist, -, - (spot; point uses
    /// ref_br/ref_dist), or raw a0 a1 a2 k0 k1 k2 (raw point: k only).
    pub params: [f32; 6],
    pub spot_func: u32,
    pub dist_func: u32,
}

impl Light {
    pub fn kind(&self) -> u16 { self.flags & 3 }

    /// GX angle (a0, a1, a2) and distance (k0, k1, k2) attenuation as `HSD_LObjSetup` 0x801A4F54 programs it:
    /// infinite = none, point = distance attenuation + spot off, spot = distance attenuation (ref_dist at
    /// least 0.001) + spot; raw parameters as given (point: a = 1, 0, 0).
    pub fn attn(&self) -> [f32; 6] {
        let p = self.params;
        match (self.kind(), self.raw) {
            (2, true) => [1.0, 0.0, 0.0, p[3], p[4], p[5]],
            (2, false) => { let k = gx_light_dist_attn(p[3], p[2], self.dist_func); [1.0, 0.0, 0.0, k[0], k[1], k[2]] }
            (3, true) => p,
            (3, false) => { let a = gx_light_spot(p[0], self.spot_func); let k = gx_light_dist_attn(p[3].max(0.001), p[2], self.dist_func); [a[0], a[1], a[2], k[0], k[1], k[2]] }
            _ => [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        }
    }

    /// One `LObjUpdateFunc` (0x801A6494) track value: 9..11 colour RGB and 22 alpha ((u8)(255 * clamp01)),
    /// 12 visibility (< 0.5 hides), 13/19 cutoff or a0, 14/20 ref_dist or a1, 15/21 ref_br or a2, 16..18 raw k0..k2.
    pub fn apply_track(&mut self, channel: u8, v: f32) {
        let byte = |v: f32| (255.0 * v.clamp(0.0, 1.0)) as u8;
        match channel {
            9..=11 => self.color[(channel - 9) as usize] = byte(v),
            22 => self.color[3] = byte(v),
            12 => if v < 0.5 { self.flags |= 0x20 } else { self.flags &= !0x20 },
            13 | 19 => self.params[0] = v,
            14 | 20 => self.params[if self.raw { 1 } else { 3 }] = v,
            15 | 21 => self.params[2] = v,
            16..=18 if self.raw => self.params[(channel - 13) as usize] = v,
            _ => {}
        }
    }
}

/// `GXInitLightSpot`: angle attenuation for a cutoff in degrees (0 or > 90 = off).
pub fn gx_light_spot(cutoff: f32, func: u32) -> [f32; 3] {
    let func = if cutoff <= 0.0 || cutoff > 90.0 { 0 } else { func };
    let cr = (cutoff * std::f32::consts::PI / 180.0).cos();
    let d = (1.0 - cr) * (1.0 - cr);
    match func {
        1 => [-1000.0 * cr, 1000.0, 0.0],
        2 => [-cr / (1.0 - cr), 1.0 / (1.0 - cr), 0.0],
        3 => [0.0, -cr / (1.0 - cr), 1.0 / (1.0 - cr)],
        4 => [cr * (cr - 2.0) / d, 2.0 / d, -1.0 / d],
        5 => [-4.0 * cr / d, 4.0 * (1.0 + cr) / d, -4.0 / d],
        6 => [1.0 - 2.0 * cr * cr / d, 4.0 * cr / d, -2.0 / d],
        _ => [1.0, 0.0, 0.0],
    }
}

/// `GXInitLightDistAttn`: distance attenuation from a reference distance and brightness.
pub fn gx_light_dist_attn(ref_dist: f32, ref_br: f32, func: u32) -> [f32; 3] {
    let func = if ref_dist < 0.0 || ref_br <= 0.0 || ref_br >= 1.0 { 0 } else { func };
    match func {
        1 => [1.0, (1.0 - ref_br) / (ref_br * ref_dist), 0.0],
        2 => [1.0, 0.5 * (1.0 - ref_br) / (ref_br * ref_dist), 0.5 * (1.0 - ref_br) / (ref_br * ref_dist * ref_dist)],
        3 => [1.0, 0.0, (1.0 - ref_br) / (ref_br * ref_dist * ref_dist)],
        _ => [1.0, 0.0, 0.0],
    }
}

pub fn scene_lights(arc: &Archive) -> Result<Vec<Light>> {
    let mut out = vec![];
    let Some(scene) = arc.root("scene_data") else { return Ok(out) };
    let Some(list) = arc.ptr(scene + 8)? else { return Ok(out) };
    for i in 0.. {
        let Some(set) = arc.ptr(list + 4 * i)? else { break };
        let mut desc = arc.ptr(set)?;
        while let Some(d) = desc {
            let w = arc.u32(d + 8)?;
            let (flags, raw) = ((w >> 16) as u16, w & 1 != 0);
            let pos = |o: usize| -> Result<[f32; 3]> { Ok(match arc.ptr(d + o)? { Some(w) => arc.vec3(w + 4)?, None => [0.0; 3] }) };
            let q = arc.ptr(d + 0x18)?;
            let f = |k: usize| -> Result<f32> { match q { Some(q) => arc.f32(q + 4 * k), None => Ok(0.0) } };
            let u = |k: usize| -> Result<u32> { match q { Some(q) => arc.u32(q + 4 * k), None => Ok(0) } };
            let mut l = Light { flags, raw, color: arc.u32(d + 0xC)?.to_be_bytes(), position: pos(0x10)?, interest: pos(0x14)?, params: [0.0; 6], spot_func: 0, dist_func: 0 };
            if q.is_some() {
                match (flags & 3, raw) {
                    (2, false) => { l.params[2] = f(0)?; l.params[3] = f(1)?; l.dist_func = u(2)?; }
                    (2, true) => { l.params[3] = f(3)?; l.params[4] = f(4)?; l.params[5] = f(5)?; }
                    (3, false) => { l.params[0] = f(0)?; l.spot_func = u(1)?; l.params[2] = f(2)?; l.params[3] = f(3)?; l.dist_func = u(4)?; }
                    (3, true) => { for k in 0..6 { l.params[k] = f(k)?; } }
                    _ => {}
                }
            }
            out.push(l);
            desc = arc.ptr(d + 4)?;
        }
    }
    Ok(out)
}
