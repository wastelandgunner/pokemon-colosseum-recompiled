//! The HSD particle system (sysdolphin psystem as linked in main.dol: `particle.c`, `generator.c`,
//! `psinterpret.c`, `psdisp.c`) and Genius Sonority's `GSparticle` glue, ported from the decompiled code.
//!
//! - A bank is a [`Gpt1`] (`psInitDataBank`). Generators come from its templates (`psCreateGeneratorID`
//!   0x80173718); each 60 Hz frame `psInterpretParticles` runs every particle's bytecode and physics, then
//!   `psExecGenerator` lets every generator emit (`generateParticle` 0x8017424C) and age.
//! - Models trigger generators from animation tracks of type 0x28 (HSD_A_J_PTCL): when a key is reached the
//!   jobj callback (`GSparticle` 0x80119BD0) creates generator `value >> 6` of the model's linked bank
//!   (`GSmodelLinkToGSparticleBank`) and, with `GSmodelSetGSparticleLinkAttachMode` ≠ 0, ties it to that joint
//!   (0x801183EC: mode 4 copies the joint's world position and rotation into the generator every frame).
//! - Drawing (`psDispSub` 0x8016BA18 / `psDispSubMakePolygon` 0x8016C150): camera-facing quads of half-size
//!   `size` along camera right ± up, rotated about the view axis; trails stretch from the previous position;
//!   TEV per `psSetupTev` 0x8016EC1C: no texture → prim colour; texture → lerp(env, prim, tex) (env = 0
//!   without PrimEnv); trails take the vertex colour (tail alpha × trail factor). Blend by kind bits 22-23
//!   (0 alpha, 1 additive, 2 subtract, 3 multiply), z compare unless NoZComp, z update only with TexEdge,
//!   alpha compare from +0x56..0x58 (default a ≥ 1).
//!
//! Also: AppSRT generators (kind 0x20000; their particles live in the generator's space), point joints
//! (animation tracks 0x2A → `psSetPointJObj` slots; 0xB7 aim at, 0xB8 pull toward, 0xBF carry), lighting
//! (kind 0x80000000: material / ambient gray-alpha (0xEA / 0xEB) × the room's light colours, GX diffuse NONE),
//! DisPoint point sprites (min(6·size, 255)/6 pixels), palettes (0xE3, ComTLUT), texture mirroring (0xAE-0xB1).
//! Not ported: shape-list polygons (unused by the GS banks seen), the 0xE1/0xEC/0xF4 extensions (no handler in
//! this build's interpreter either, except 0xF4).

use crate::model::Animator;
use bevy::asset::{embedded_asset, RenderAssetUsages};
use bevy::mesh::{Indices, MeshVertexAttribute, MeshVertexBufferLayoutRef, PrimitiveTopology, VertexFormat};
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState, CompareFunction, Extent3d, RenderPipelineDescriptor,
    ShaderType, SpecializedMeshPipelineError, TextureDimension, TextureFormat,
};
use bevy::shader::ShaderRef;
use colo_formats::gpt1::{GenTemplate, Gpt1};
use std::sync::Arc;

/// Env colour per vertex (prim rides in `ATTRIBUTE_COLOR`).
pub const ATTR_ENV: MeshVertexAttribute = MeshVertexAttribute::new("PsEnv", 0x7E5_0101, VertexFormat::Float32x4);

const PI: f32 = std::f32::consts::PI;
const TAU: f32 = std::f32::consts::TAU;
const EPS_F: f32 = 1.192_092_9e-7;

/// HSD_Randf (0x801ADC7C): seed = seed·0x343FD + 0x269EC3, (seed >> 16) / 65536.
#[derive(Clone, Copy)]
struct Rng(u32);
impl Rng {
    fn f(&mut self) -> f32 {
        self.0 = self.0.wrapping_mul(0x343FD).wrapping_add(0x269EC3);
        (self.0 >> 16) as f32 / 65536.0
    }
}

fn atan2_safe(y: f32, x: f32) -> f32 {
    if x.abs() >= 1.175_494_4e-38 { y.atan2(x) } else if y < 0.0 { -PI / 2.0 } else { PI / 2.0 }
}

/// Matrix whose z axis points along `v` (`generateParticle`'s velocity alignment).
fn align_z(v: Vec3) -> Mat3 {
    let n = v.normalize_or_zero();
    let a = atan2_safe(n.y, n.z);
    let (sa, ca) = a.sin_cos();
    let b = atan2_safe(n.x, n.y * sa + n.z * ca);
    let (sb, cb) = b.sin_cos();
    // rows: (cb, 0, sb), (-sa sb, ca, sa cb), (-ca sb, -sa, ca cb)
    Mat3::from_cols(Vec3::new(cb, -sa * sb, -ca * sb), Vec3::new(0.0, ca, -sa), Vec3::new(sb, sa * cb, ca * cb))
}

/// One generator (0xB4 bytes in the game; field offsets in comments).
#[derive(Clone)]
pub struct Generator {
    /// +4 kind (particle flags, also 0x100 emit at once, 0x10000 billboard, 0x20000 AppSRT).
    pub kind: u32,
    /// +8 rate, +0xC emission accumulator.
    rate: f32,
    acc: f32,
    /// +0x10 frames left (0 = endless).
    life: u16,
    /// +0x12 type (low 4 bits: shape).
    gtype: u16,
    /// +0x14 bank, +0x15 link list, +0x16 texture group.
    bank: usize,
    link: u8,
    tex_group: u16,
    /// +0x18 family id.
    id: u16,
    /// +0x1A particle lifetime, +0x1C bytecode (template index).
    plife: u16,
    template: usize,
    /// +0x20 position, +0x2C velocity, +0x38 gravity, +0x3C friction, +0x40 size, +0x44 radius, +0x48 angle.
    pub pos: Vec3,
    vel: Vec3,
    grav: f32,
    fric: f32,
    size: f32,
    radius: f32,
    angle: f32,
    /// +0x4C live children.
    children: i32,
    /// +0x54..+0x84 shape parameters (`aux[0]` = +0x54), +0x84 box flags.
    aux: [f32; 12],
    box_sel: u16,
    /// +0x88 flags (bit 1 = no joint), +0x8C rotation (as a matrix), +0x98 scale.
    flags88: u16,
    pub rot: Mat3,
    pub scale: Vec3,
    alive: bool,
}

/// One particle (0x94 bytes; offsets in comments).
#[derive(Clone)]
struct Particle {
    /// +4 kind, +8 bank, +9 texture group, +0xA pose, +0x1D link, +0x1E id.
    kind: u32,
    bank: usize,
    tex_group: u16,
    pose: u8,
    link: u8,
    id: u16,
    /// +0xC size time, +0x60 size target.
    size_t: u16,
    size_target: f32,
    /// +0xE / +0x6C prim colour time and counter, +0x12 prim, +0x70 target; likewise env (+0x10 / +0x6E, +0x16, +0x74).
    prim_t: u16,
    prim_c: u16,
    prim: [u8; 4],
    prim_to: [u8; 4],
    env_t: u16,
    env_c: u16,
    env: [u8; 4],
    env_to: [u8; 4],
    /// +0x54 alpha compare time / +0x7C counter, +0x56 mode, +0x57/+0x58 refs, +0x7E/+0x7F targets.
    ac_t: u16,
    ac_c: u16,
    ac_mode: u8,
    ac_ref: [u8; 2],
    ac_to: [u8; 2],
    /// +0x1A wait, +0x1C loop count, +0x20 code (template), +0x24 pc, +0x26 saved jump, +0x28 loop start.
    wait: u16,
    loop_n: u8,
    template: usize,
    pc: usize,
    jump: usize,
    loop_pc: usize,
    /// +0x2A frames until death.
    life: u16,
    /// +0x2C velocity (tornado: angle, radius factor, height), +0x38 gravity, +0x3C friction (tornado: axis angles).
    vel: Vec3,
    grav: f32,
    fric: f32,
    /// +0x40 position, +0x4C size, +0x50 rotation, +0x5E rotation time, +0x64 target / rate, +0x68 acceleration.
    pos: Vec3,
    size: f32,
    rot: f32,
    rot_t: u16,
    rot_to: f32,
    rot_acc: f32,
    /// +0x88 trail factor.
    trail: f32,
    /// +0x8C generator.
    gen: Option<usize>,
    /// +0xB palette (0xFF = the image's own), +0x59 point-joint base.
    palette: u8,
    joint_base: u8,
    /// +0x80/+0x81 material colour (gray, alpha), +0x5A time, +0x78 counter, +0x84 targets; +0x82.. ambient likewise
    /// (+0x5C, +0x7A, +0x86). Used when lit (kind 0x80000000).
    mat: [u8; 2],
    mat_to: [u8; 2],
    mat_t: u16,
    mat_c: u16,
    amb: [u8; 2],
    amb_to: [u8; 2],
    amb_t: u16,
    amb_c: u16,
    /// +0x90 AppSRT: the position is local to this generator's position / rotation / scale (gen kind 0x20000).
    local: Option<usize>,
    alive: bool,
}

/// A loaded bank and its textures (one image per texture group pose).
pub struct Bank {
    pub gpt: Arc<Gpt1>,
    textures: Vec<Vec<Handle<Image>>>,
}

/// `GSparticle` model links: models whose PTCL keys create generators of `bank`.
#[derive(Component)]
pub struct ParticleLink {
    pub bank: usize,
    /// `GSmodelSetGSparticleLinkAttachMode` (4: follow the joint's position and rotation).
    pub attach: u32,
    last: Option<(usize, f32)>,
}

impl ParticleLink {
    pub fn new(bank: usize, attach: u32) -> Self { ParticleLink { bank, attach, last: None } }
}

#[derive(Resource)]
pub struct Particles {
    pub banks: Vec<Bank>,
    gens: Vec<Generator>,
    parts: Vec<Particle>,
    /// Generators tied to a joint entity (attach mode).
    ties: Vec<(usize, Entity, u32)>,
    rng: Rng,
    next_id: u16,
    acc: f32,
    /// Drawn batches (entity per material key).
    batches: Vec<(BatchKey, Entity, Handle<Mesh>)>,
    /// `psSetPointJObj` slots 1..=8 (index 0 unused): joints set by animation tracks 0x2A, and their positions
    /// this frame (opcodes 0xB7 / 0xB8).
    points: [Option<Entity>; 9],
    point_pos: [Option<Vec3>; 9],
    /// Images re-decoded with another palette (bank, group, pose, palette).
    pal_images: std::collections::HashMap<(usize, u16, u8, u8), Handle<Image>>,
    /// Ambient + light colours of the room (lit particles: GX diffuse function NONE, so no angle term).
    pub light: Vec3,
    pub trace: bool,
}

impl Default for Particles {
    fn default() -> Self {
        Particles { banks: vec![], gens: vec![], parts: vec![], ties: vec![], rng: Rng(0x1234_5678), next_id: 0x100, acc: -1.0, batches: vec![], points: [None; 9], point_pos: [None; 9],
            pal_images: Default::default(), light: Vec3::ONE,
            trace: std::env::var("COLO_PARTICLES").is_ok() }
    }
}

impl Particles {
    /// Register a bank (textures uploaded as images); returns its index.
    pub fn add_bank(&mut self, gpt: Arc<Gpt1>, images: &mut Assets<Image>) -> usize {
        let textures = gpt.tex_groups.iter().map(|g| g.as_ref().map_or(vec![], |g| g.images.iter().map(|rgba| {
            images.add(Image::new(Extent3d { width: g.width as u32, height: g.height as u32, depth_or_array_layers: 1 }, TextureDimension::D2,
                rgba.clone(), TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD))
        }).collect())).collect();
        self.banks.push(Bank { gpt, textures });
        self.banks.len() - 1
    }

    /// Drop everything (room change; the batch entities go with the room root).
    pub fn clear(&mut self) {
        self.batches.clear();
        self.banks.clear();
        self.points = [None; 9];
        self.pal_images.clear();
        self.gens.clear();
        self.parts.clear();
        self.ties.clear();
    }

    fn template(&self, bank: usize, id: usize) -> Option<&GenTemplate> { self.banks.get(bank)?.gpt.generators.get(id)?.as_ref() }

    /// `psCreateGeneratorID(link, bank, id)`.
    pub fn create_generator(&mut self, link: u8, bank: usize, id: usize) -> Option<usize> {
        let t = self.template(bank, id)?.clone();
        self.next_id = self.next_id.wrapping_add(1).max(0x100);
        let mut g = Generator {
            kind: t.flags, rate: t.rate, acc: 0.0, life: t.life, gtype: t.kind, bank, link, tex_group: t.tex_group, id: self.next_id,
            plife: t.particle_life, template: id, pos: Vec3::ZERO, vel: Vec3::from_array(t.velocity), grav: t.gravity, fric: t.friction,
            size: t.size, radius: t.radius, angle: t.angle, children: 0, aux: [0.0; 12], box_sel: 0, flags88: 2, rot: Mat3::IDENTITY,
            scale: Vec3::ONE, alive: true,
        };
        g.acc = if g.kind & 0x100 == 0 { if g.rate >= 0.0 { self.rng.f() } else { 0.0 } }
            else if g.rate >= 0.0 { 1.0 - EPS_F } else if EPS_F < 1.0 + g.rate { 1.0 } else { 0.0 };
        let a = t.aux;
        match g.gtype & 0xF {
            0 | 3 | 4 => { if a[0] == 0.0 && a[1] == 0.0 { g.aux[0] = 0.0; g.aux[1] = TAU; } else { g.aux[0] = a[0]; g.aux[1] = a[1]; } }
            1 => { g.aux[0] = a[0]; g.aux[1] = a[1]; g.aux[2] = a[2]; }
            5 => {
                // box: half extents on the axes (+0x60/+0x70/+0x80 diagonal), +0x84 which faces are hollow
                g.aux = [a[0], a[1], a[2], a[0], 0.0, 0.0, 0.0, a[1], 0.0, 0.0, 0.0, a[2]];
                g.box_sel = (a[0] < 0.0) as u16 | ((a[1] < 0.0) as u16) << 1 | ((a[2] < 0.0) as u16) << 2;
            }
            6 | 7 => {
                if a[0] == 0.0 && a[1] == 0.0 { g.aux[0] = 0.0; g.aux[1] = TAU; } else { g.aux[0] = a[0]; g.aux[1] = a[1]; }
                g.aux[2] = a[2];
            }
            8 => {
                let v = g.vel;
                g.aux[0] = v.length();
                let xz = (v.x * v.x + v.z * v.z).sqrt();
                g.aux[1] = if xz >= 1.175_494_4e-38 { v.y.atan2(xz) } else if v.y < 0.0 { -PI / 2.0 } else { PI / 2.0 };
                g.aux[3] = atan2_safe(v.z, v.x);
                g.aux[2] = a[0];
                if g.aux[2] < 0.0 { g.aux[2] = -g.aux[2]; g.aux[0] = -g.aux[0]; }
                g.aux[4] = a[1];
            }
            _ => {}
        }
        self.gens.push(g);
        if self.trace { println!("particles: generator {id} (bank {bank}) type {:#x}", t.kind); }
        Some(self.gens.len() - 1)
    }

    /// `psGenerateParticle0` with interpretation of the first frame.
    #[allow(clippy::too_many_arguments)]
    fn spawn(&mut self, pos: Vec3, vel: Vec3, size: f32, grav: f32, fric: f32, kind: u32, bank: usize, tex_group: u16, template: usize,
             life: u16, link: u8, gen: Option<usize>, id: Option<u16>) -> usize {
        let id = id.unwrap_or_else(|| { self.next_id = self.next_id.wrapping_add(1).max(0x100); self.next_id });
        let blend = (kind >> 22) & 3;
        let p = Particle {
            kind, bank, tex_group, pose: 0, link, id, size_t: 0, size_target: 0.0, prim_t: 0, prim_c: 0, prim: [255; 4], prim_to: [255; 4],
            env_t: 0, env_c: 0, env: [0; 4], env_to: [0; 4], ac_t: 0, ac_c: 0, ac_mode: 0x33, ac_ref: [(blend < 2) as u8, 255], ac_to: [0; 2],
            wait: 1, loop_n: 0, template, pc: 0, jump: 0, loop_pc: 0, life: life.wrapping_add(1), vel, grav, fric, pos, size, rot: 0.0,
            rot_t: 0, rot_to: 0.0, rot_acc: 0.0, trail: 1.0, gen, palette: 0xFF, joint_base: 0, mat: [255; 2], mat_to: [255; 2], mat_t: 0, mat_c: 0,
            amb: [255; 2], amb_to: [255; 2], amb_t: 0, amb_c: 0,
            local: gen.filter(|&g| self.gens.get(g).is_some_and(|g| g.kind & 0x20000 != 0)), alive: true,
        };
        if let Some(g) = gen.and_then(|g| self.gens.get_mut(g)) { g.children += 1; }
        self.parts.push(p);
        let i = self.parts.len() - 1;
        self.interpret(i);
        i
    }

    /// `psGenerateParticleID0`: a particle straight from template `id` (spawn opcodes).
    fn spawn_id(&mut self, from: usize, id: usize) -> Option<usize> {
        let (bank, link) = (self.parts[from].bank, self.parts[from].link);
        let t = self.template(bank, id)?.clone();
        Some(self.spawn(Vec3::ZERO, Vec3::from_array(t.velocity), t.size, t.gravity, t.friction, t.flags, bank, t.tex_group, id, t.particle_life, link, None, None))
    }

    fn resolve_ref(&self, bank: usize, id: usize) -> usize {
        self.banks.get(bank).and_then(|b| b.gpt.refs.get(id)).map_or(id, |&r| r as usize)
    }

    /// A child generator from a particle (0xA5 / 0xEF / 0xF0): inherits rotation, scale and position.
    fn child_generator(&mut self, from: usize, id: usize, flags: Option<u8>) {
        let (link, bank, fid, pos, pgen) = { let p = &self.parts[from]; (p.link, p.bank, p.id, p.pos, p.gen) };
        let Some(g) = self.create_generator(link, bank, id) else { return };
        let src = pgen.and_then(|i| self.gens.get(i)).cloned();
        let ng = &mut self.gens[g];
        ng.id = fid;
        if let Some(s) = src {
            // psCopyGeneratorData
            ng.kind |= s.kind & 0x2000_0000;
            ng.rot = s.rot;
            ng.scale = s.scale;
            ng.flags88 = s.flags88 & !2;
        }
        if let Some(f) = flags { ng.kind = (ng.kind & 0xF1FF_FFFF) | ((f as u32 & 7) << 25); }
        ng.pos = pos;
    }

    /// `psInterpretParticle0`: colour/size/rotation timers, then the bytecode until a wait, then physics.
    fn interpret(&mut self, i: usize) {
        if self.parts[i].kind & 0x800 != 0 { return; }
        {
            let p = &mut self.parts[i];
            if p.size_t != 0 { p.size += (p.size_target - p.size) / p.size_t as f32; p.size_t -= 1; }
            if p.prim_t != 0 { p.prim_c = p.prim_c.saturating_sub(1); if p.prim_c == 0 { p.prim_t = 0; p.prim = p.prim_to; } }
            if p.env_t != 0 { p.env_c = p.env_c.saturating_sub(1); if p.env_c == 0 { p.env_t = 0; p.env = p.env_to; } }
            if p.ac_t != 0 { p.ac_c = p.ac_c.saturating_sub(1); if p.ac_c == 0 { p.ac_t = 0; p.ac_ref = p.ac_to; } }
            if p.mat_t != 0 { p.mat_c = p.mat_c.saturating_sub(1); if p.mat_c == 0 { p.mat_t = 0; p.mat = p.mat_to; } }
            if p.amb_t != 0 { p.amb_c = p.amb_c.saturating_sub(1); if p.amb_c == 0 { p.amb_t = 0; p.amb = p.amb_to; } }
            if p.rot_t != 0 {
                if p.rot_acc == 0.0 { p.rot += (p.rot_to - p.rot) / p.rot_t as f32; p.rot_t -= 1; }
                else {
                    p.rot += p.rot_to;
                    p.rot_to += if p.rot_to < 0.0 { -p.rot_acc } else { p.rot_acc };
                    p.rot_t -= 1;
                    if p.rot_t == 0 { p.rot_acc = 0.0; p.rot_to = 0.0; }
                }
            }
        }
        if self.parts[i].wait != 0 {
            self.parts[i].wait -= 1;
            if self.parts[i].wait == 0 { self.run_code(i); }
        }
        let p = &mut self.parts[i];
        if !p.alive { return; }
        p.life = p.life.saturating_sub(1);
        if p.life == 0 {
            p.alive = false;
            if let Some(g) = p.gen.and_then(|g| self.gens.get_mut(g)) { g.children -= 1; }
            return;
        }
        if p.kind & 4 == 0 {
            if p.kind & 1 != 0 { p.vel.y -= p.grav; }
            if p.kind & 2 != 0 { p.vel *= p.fric; }
            p.pos += p.vel;
        } else if let Some(g) = p.gen.and_then(|g| self.gens.get(g)).map(|g| { let mut g = g.clone(); if p.local.is_some() { g.pos = Vec3::ZERO; } g }) {
            let g = &g;
            // tornado: +0x2C angle, +0x30 radius factor, +0x34 height around the generator's axis
            p.vel.z += g.aux[0];
            p.vel.x += g.grav;
            p.pos = tornado_pos(p, g, p.vel.x, p.vel.z);
        }
    }

    fn run_code(&mut self, i: usize) {
        let (bank, template) = (self.parts[i].bank, self.parts[i].template);
        let Some(code) = self.template(bank, template).map(|t| t.code.clone()) else { self.parts[i].life = 1; return };
        let mut pc = self.parts[i].pc;
        let rd = |pc: &mut usize| -> u8 { let b = code.get(*pc).copied().unwrap_or(0xFF); *pc += 1; b };
        let rf = |pc: &mut usize| -> f32 {
            let b = code.get(*pc..*pc + 4).map_or([0; 4], |s| [s[0], s[1], s[2], s[3]]);
            *pc += 4;
            f32::from_be_bytes(b)
        };
        let rt = |pc: &mut usize| -> u16 {
            let b = code.get(*pc).copied().unwrap_or(0) as u16;
            *pc += 1;
            if b & 0x80 == 0 { b } else { let c = code.get(*pc).copied().unwrap_or(0) as u16; *pc += 1; (b & 0x7F) * 256 + c }
        };
        let mut guard = 0;
        loop {
            guard += 1;
            if guard > 4096 { self.parts[i].life = 1; break; }
            let op = rd(&mut pc);
            if op < 0x80 {
                let mut w = (op & 0x1F) as u16;
                if op & 0x20 != 0 { w = w * 256 + rd(&mut pc) as u16; }
                if op & 0xC0 == 0x40 {
                    let pose = rd(&mut pc);
                    let p = &mut self.parts[i];
                    p.pose = pose;
                    let ok = self.banks.get(p.bank).and_then(|b| b.textures.get(p.tex_group as usize)).is_some_and(|t| (pose as usize) < t.len());
                    if ok { p.kind |= 0x400; }
                }
                if w != 0 { self.parts[i].wait = w; break; }
                continue;
            }
            let class = if op & 0xF8 <= 0x98 { op & 0xF8 } else if matches!(op & 0xF0, 0xC0 | 0xD0) { op & 0xF0 } else { op };
            let r = &mut self.rng;
            match class {
                0x80 | 0x88 | 0x90 | 0x98 => {
                    let mut v = Vec3::ZERO;
                    if op & 1 != 0 { v.x = rf(&mut pc); }
                    if op & 2 != 0 { v.y = rf(&mut pc); }
                    if op & 4 != 0 { v.z = rf(&mut pc); }
                    let p = &mut self.parts[i];
                    match class { 0x80 => p.pos = v, 0x88 => p.pos += v, 0x90 => p.vel = v, _ => p.vel += v }
                }
                0xA0 => { let p = &mut self.parts[i]; p.size_t = rt(&mut pc); p.size_target = rf(&mut pc); if p.size_t == 0 { p.size = p.size_target; } }
                0xA1 => self.parts[i].kind &= !0x400,
                0xA2 => { let p = &mut self.parts[i]; p.grav = rf(&mut pc); if p.grav == 0.0 { p.kind &= !1 } else { p.kind |= 1 } }
                0xA3 => { let p = &mut self.parts[i]; p.fric = rf(&mut pc); if p.fric == 1.0 { p.kind &= !2 } else { p.kind |= 2 } }
                0xA4 | 0xB9 | 0xF1 | 0xF2 => {
                    let mut id = (rd(&mut pc) as usize) << 8; id |= rd(&mut pc) as usize;
                    if op >= 0xF1 { id = self.resolve_ref(bank, id); }
                    if let Some(c) = self.spawn_id(i, id) {
                        let (pos, vel, pid, gen) = { let p = &self.parts[i]; (p.pos, p.vel, p.id, p.gen) };
                        let cp = &mut self.parts[c];
                        cp.pos = pos;
                        cp.id = pid;
                        cp.gen = gen;
                        if matches!(op, 0xB9 | 0xF2) { cp.vel = vel; }
                        if let Some(g) = gen.and_then(|g| self.gens.get_mut(g)) { g.children += 1; }
                    }
                }
                0xA5 | 0xEF | 0xF0 => {
                    let mut id = (rd(&mut pc) as usize) << 8; id |= rd(&mut pc) as usize;
                    let flags = if op != 0xA5 { Some(rd(&mut pc)) } else { None };
                    if op == 0xF0 { id = self.resolve_ref(bank, id); }
                    self.child_generator(i, id, flags);
                }
                0xA6 => {
                    let base = (rd(&mut pc) as u16) << 8 | rd(&mut pc) as u16;
                    let range = (rd(&mut pc) as u16) << 8 | rd(&mut pc) as u16;
                    let n = (range as f32 * r.f()) as u16;
                    self.parts[i].life = base.wrapping_add(n);
                }
                0xA7 => { let c = rd(&mut pc); if (100.0 * r.f()) as u32 <= c as u32 { self.parts[i].life = 1; break; } }
                0xA8 => {
                    let mut v = Vec3::ZERO;
                    for k in 0..3 { let f = rf(&mut pc); v[k] = 2.0 * f * r.f() - f; }
                    self.parts[i].pos += v;
                }
                0xA9 => { let f = rf(&mut pc); modify_dir(&mut self.parts[i].vel, f, r); }
                0xAA => {
                    let base = (rd(&mut pc) as usize) << 8 | rd(&mut pc) as usize;
                    let range = (rd(&mut pc) as usize) << 8 | rd(&mut pc) as usize;
                    let pick = (range as f32 * r.f()) as usize;
                    let id = self.resolve_ref(bank, base + pick);
                    if let Some(c) = self.spawn_id(i, id) {
                        let (pos, pid, gen) = { let p = &self.parts[i]; (p.pos, p.id, p.gen) };
                        let cp = &mut self.parts[c];
                        cp.pos = pos;
                        cp.id = pid;
                        cp.gen = gen;
                        if let Some(g) = gen.and_then(|g| self.gens.get_mut(g)) { g.children += 1; }
                    }
                }
                0xAB => { let f = rf(&mut pc); self.parts[i].vel *= f; }
                0xAC => {
                    let p = &mut self.parts[i];
                    p.size_t = rt(&mut pc);
                    p.size_target = rf(&mut pc);
                    let f = rf(&mut pc);
                    p.size_target += f * r.f();
                    if p.size_t == 0 { p.size = p.size_target; }
                }
                0xAD => self.parts[i].kind |= 0x80,
                0xAE => self.parts[i].kind &= !0x60,
                0xAF => self.parts[i].kind = (self.parts[i].kind & !0x40) | 0x20,
                0xB0 => self.parts[i].kind = (self.parts[i].kind & !0x20) | 0x40,
                0xB1 => self.parts[i].kind |= 0x60,
                0xB2 => {
                    // AppSRT applied: the position becomes world space and the particle stops following
                    let p = &self.parts[i];
                    if let Some(g) = p.local.and_then(|g| self.gens.get(g)) { let w = g.pos + g.rot * (p.pos * g.scale); let p = &mut self.parts[i]; p.pos = w; p.local = None; }
                }
                0xB3 => {
                    let p = &mut self.parts[i];
                    if p.ac_t != 0 { let k = p.ac_c as f32 / p.ac_t as f32; for c in 0..2 { p.ac_ref[c] = lerp_u8(p.ac_to[c], p.ac_ref[c], k); } }
                    p.ac_t = rt(&mut pc);
                    p.ac_mode = rd(&mut pc);
                    p.ac_to = [rd(&mut pc), rd(&mut pc)];
                    if p.ac_t == 0 { p.ac_ref = p.ac_to; p.ac_c = 0; } else { p.ac_c = p.ac_t; }
                }
                0xB4 => self.parts[i].kind |= 0x200,
                0xB5 => self.parts[i].kind &= !0x200,
                0xB6 => { let p = &mut self.parts[i]; p.rot_t = rt(&mut pc); let f = rf(&mut pc); p.rot_to += f; if p.rot_t == 0 { p.rot = p.rot_to; } }
                0xB7 => {
                    // velocity toward point joint (speed kept)
                    let slot = rd(&mut pc) as usize + self.parts[i].joint_base as usize;
                    if let Some(t) = self.point_pos.get(slot).copied().flatten() {
                        let p = &mut self.parts[i];
                        let d = t - p.pos;
                        let (sp, l) = (p.vel.length(), d.length());
                        if l != 0.0 { p.vel = d * (sp / l); }
                    }
                }
                0xB8 => {
                    // pull toward point joint by strength / dist², killed within the radius
                    let slot = rd(&mut pc) as usize + self.parts[i].joint_base as usize;
                    let (k, rad) = (rf(&mut pc), rf(&mut pc));
                    if let (Some(t), true) = (self.point_pos.get(slot).copied().flatten(), rad >= 0.0) {
                        let p = &mut self.parts[i];
                        let d = t - p.pos;
                        let d2 = d.length_squared();
                        if rad * rad < d2 { p.vel += d * (k / d2); } else { p.life = 1; break; }
                    }
                }
                0xBF => {
                    // the particle carries a joint (kind 0x8000 | slot << 12); its point follows the particle
                    let slot = rd(&mut pc) as u32 + self.parts[i].joint_base as u32;
                    self.parts[i].kind |= ((slot * 0x1000) & 0x7000) | 0x8000;
                }
                0xBA | 0xBB => {
                    let p = &mut self.parts[i];
                    let (cur, to, t, c) = if op == 0xBA { (&mut p.prim, &mut p.prim_to, p.prim_t, &mut p.prim_c) } else { (&mut p.env, &mut p.env_to, p.env_t, &mut p.env_c) };
                    if t != 0 { let k = *c as f32 / t as f32; for ch in 0..4 { cur[ch] = lerp_u8(to[ch], cur[ch], k); } }
                    for ch in 0..4 { let d = (rd(&mut pc) as i8) as f32 * 2.0 * r.f(); to[ch] = clamp_add(to[ch], d); }
                    if t == 0 { *cur = *to; } else { *c = t; }
                }
                0xBC => {
                    let base = rd(&mut pc);
                    let range = rd(&mut pc) as f32;
                    let p = &mut self.parts[i];
                    p.pose = (range * r.f() + base as f32) as u8;
                    let ok = self.banks.get(p.bank).and_then(|b| b.textures.get(p.tex_group as usize)).is_some_and(|t| (p.pose as usize) < t.len());
                    if ok { p.kind |= 0x400; }
                }
                0xBD => {
                    let base = rf(&mut pc);
                    let range = rf(&mut pc);
                    let s = range * r.f() + base;
                    let p = &mut self.parts[i];
                    let l = p.vel.length();
                    if l > 1e-10 { p.vel *= s / l; }
                }
                0xBE => { let p = &mut self.parts[i]; p.vel.x *= rf(&mut pc); p.vel.y *= rf(&mut pc); p.vel.z *= rf(&mut pc); }
                0xC0 | 0xD0 => {
                    let p = &mut self.parts[i];
                    let prim = class == 0xC0;
                    let (cur, to, t, c) = if prim { (&mut p.prim, &mut p.prim_to, &mut p.prim_t, &mut p.prim_c) } else { (&mut p.env, &mut p.env_to, &mut p.env_t, &mut p.env_c) };
                    if *t != 0 { let k = *c as f32 / *t as f32; for ch in 0..4 { cur[ch] = lerp_u8(to[ch], cur[ch], k); } }
                    *t = rt(&mut pc);
                    *to = *cur;
                    for ch in 0..4 { if op & (1 << ch) != 0 { to[ch] = rd(&mut pc); } }
                    if *t == 0 { *cur = *to; *c = 0; } else { *c = *t; }
                }
                0xE0 => {
                    let p = &mut self.parts[i];
                    resolve(&mut p.prim, &p.prim_to, p.prim_t, p.prim_c);
                    resolve(&mut p.env, &p.env_to, p.env_t, p.env_c);
                    for ch in 0..4 {
                        let d = (rd(&mut pc) as i8) as f32 * 2.0 * r.f();
                        p.prim_to[ch] = clamp_add(p.prim_to[ch], d);
                        p.env_to[ch] = clamp_add(p.env_to[ch], d);
                    }
                    if p.prim_t == 0 { p.prim = p.prim_to; }
                    p.prim_c = p.prim_t;
                    if p.env_t == 0 { p.env = p.env_to; }
                    p.env_c = p.env_t;
                }
                0xE2 => self.parts[i].kind |= 8,
                0xE3 => self.parts[i].palette = rd(&mut pc),
                0xE4 | 0xE5 => {
                    let b = rd(&mut pc) & 3;
                    let bit = if op == 0xE4 { 0x40000 } else { 0x80000 };
                    let p = &mut self.parts[i];
                    match b { 0 => p.kind &= !bit, 1 => p.kind |= bit, 2 => p.kind ^= bit, _ => if r.f() >= 0.5 { p.kind |= bit } else { p.kind &= !bit } }
                }
                0xE6 => self.parts[i].kind |= 0x200000,
                0xE7 => self.parts[i].kind &= !0x200000,
                0xE8 => { let f = rf(&mut pc); let p = &mut self.parts[i]; if f >= 0.0 { p.kind |= 0x100000; p.trail = f; } else { p.kind &= !0x100000; } }
                0xE9 => {
                    let flags = rd(&mut pc);
                    let n = rd(&mut pc) as u32;
                    let p = &mut self.parts[i];
                    resolve(&mut p.prim, &p.prim_to, p.prim_t, p.prim_c);
                    resolve(&mut p.env, &p.env_to, p.env_t, p.env_c);
                    let step = |r: &mut Rng| if n == 0 { r.f() } else { ((n + 1) as f32 * r.f()) as i32 as f32 / n as f32 };
                    let k = step(r);
                    for ch in 0..3 {
                        if flags & (1 << ch) != 0 {
                            let d = k * (rd(&mut pc) as i8) as f32 * 2.0;
                            if flags & 0x10 != 0 { p.prim_to[ch] = clamp_add(p.prim_to[ch], d); }
                            if flags & 0x20 != 0 { p.env_to[ch] = clamp_add(p.env_to[ch], d); }
                        }
                    }
                    if flags & 8 != 0 {
                        let d = step(r) * (rd(&mut pc) as i8) as f32 * 2.0;
                        if flags & 0x10 != 0 { p.prim_to[3] = clamp_add(p.prim_to[3], d); }
                        if flags & 0x20 != 0 { p.env_to[3] = clamp_add(p.env_to[3], d); }
                    }
                    if p.prim_t == 0 { p.prim = p.prim_to; }
                    p.prim_c = p.prim_t;
                    if p.env_t == 0 { p.env = p.env_to; }
                    p.env_c = p.env_t;
                }
                0xEA | 0xEB => {
                    let p = &mut self.parts[i];
                    let (cur, to, t, c) = if op == 0xEA { (&mut p.mat, &mut p.mat_to, &mut p.mat_t, &mut p.mat_c) } else { (&mut p.amb, &mut p.amb_to, &mut p.amb_t, &mut p.amb_c) };
                    if *t != 0 { let k = *c as f32 / *t as f32; for ch in 0..2 { cur[ch] = lerp_u8(to[ch], cur[ch], k); } }
                    *t = rt(&mut pc);
                    let f = rd(&mut pc);
                    *to = *cur;
                    if f & 1 != 0 { to[0] = rd(&mut pc); }
                    if f & 8 != 0 { to[1] = rd(&mut pc); }
                    if *t == 0 { *cur = *to; *c = 0; } else { *c = *t; }
                }
                0xED => {
                    let base = rf(&mut pc);
                    let range = rf(&mut pc);
                    let n = rd(&mut pc) as u32;
                    let d = if n == 0 { range * r.f() + base } else { base + range * ((n + 1) as f32 * r.f()) as i32 as f32 / n as f32 };
                    let p = &mut self.parts[i];
                    p.rot_to += d;
                    p.rot += d;
                }
                0xF3 => {
                    let dir = rd(&mut pc);
                    let p = &mut self.parts[i];
                    p.rot_to = rf(&mut pc);
                    p.rot_acc = rf(&mut pc);
                    p.rot_t = rt(&mut pc);
                    if p.rot_t == 0 { p.rot_to = 0.0; p.rot_acc = 0.0; }
                    else if dir == 0 { p.rot_to += p.rot_acc * 0.5; }
                    else { p.rot_to *= -0.007_812_5; p.rot_to -= p.rot_acc * 0.5; }
                }
                0xF4 => { for _ in 0..4 { rf(&mut pc); } }
                0xF5 | 0xF6 => {}
                0xF7 => self.parts[i].kind |= 0x1000_0000,
                0xFA => { let n = rd(&mut pc); let p = &mut self.parts[i]; p.loop_n = n; p.loop_pc = pc; }
                0xFB => { let p = &mut self.parts[i]; p.loop_n = p.loop_n.wrapping_sub(1); if p.loop_n != 0 { pc = p.loop_pc; } }
                0xFC => self.parts[i].jump = pc,
                0xFD => pc = self.parts[i].jump,
                0xFE | 0xFF => { self.parts[i].life = 1; break; }
                _ => {}
            }
        }
        self.parts[i].pc = pc;
    }

    /// `generateParticle`: emit while the accumulator holds whole particles.
    fn emit(&mut self, gi: usize, cam: Option<(Vec3, Vec3)>) {
        if self.gens[gi].acc < 1.0 { return; }
        let mut g = self.gens[gi].clone();
        // an AppSRT generator emits in its own space (particles follow it, see `local`)
        if g.kind & 0x20000 != 0 { g.pos = Vec3::ZERO; }
        let shape = g.gtype & 0xF;
        let mut vel = g.vel;
        if shape == 2 && g.flags88 & 0x80 != 0 { vel *= g.scale; }
        let speed = vel.length();
        let mut m = if g.kind & 0x30000 == 0 { g.rot } else { Mat3::IDENTITY };
        if g.kind & 0x10000 != 0 {
            if let Some((eye, up)) = cam {
                let z = (eye - g.pos).normalize_or_zero();
                let x = up.normalize_or_zero().cross(z);
                let y = z.cross(x);
                m = Mat3::from_cols(x, y, z);
            }
        }
        if shape != 1 && speed > EPS_F { m *= align_z(g.vel); }
        // tornado axis angles
        let (mut ax, mut ay) = (0.0, 0.0);
        if shape == 2 {
            let (m12, m22, m02) = (m.col(2).y, m.col(2).z, m.col(2).x);
            ax = atan2_safe(m12, m22);
            ay = atan2_safe(m02, m12 * ax.sin() + m22 * ax.cos());
        }
        // stepped azimuth when the angle is fixed (negative)
        let (mut az, mut step) = (0.0f32, 0.0f32);
        if g.angle < 0.0 {
            let n = g.acc as i32 as f32;
            if shape == 5 || matches!(shape, 1 | 2) || shape > 7 {
                az = (2.0 * PI as f64 * self.rng.f() as f64) as f32;
                step = (TAU as f64 / n as f64) as f32;
            } else {
                let lo = g.aux[0];
                let r = self.rng.f();
                step = (g.aux[1] - lo) / n;
                az = step * r + lo;
            }
        }
        let scale = g.scale;
        let mut acc = g.acc;
        while acc >= 1.0 {
            let r = &mut self.rng;
            match shape {
                0 | 3 | 4 | 6 | 7 => {
                    let (rr, dist) = if g.radius >= 0.0 {
                        let mut f = r.f();
                        if matches!(shape, 3 | 4) { f = f.sqrt(); }
                        (f, f * g.radius)
                    } else { (1.0, -g.radius) };
                    let polar;
                    if shape == 7 {
                        if g.angle >= 0.0 { az = (g.aux[1] - g.aux[0]) * r.f() + g.aux[0]; polar = PI / 2.0 + g.angle; }
                        else { az += step; polar = PI / 2.0 - g.angle; }
                    } else if shape == 6 {
                        if g.angle >= 0.0 {
                            az = (g.aux[1] - g.aux[0]) * r.f() + g.aux[0];
                            polar = if dist.abs() >= 1.175_494_4e-38 { g.angle + (PI / 2.0 - g.aux[2].atan2(dist)) } else if g.aux[2] < 0.0 { PI + g.angle } else { g.angle };
                        } else {
                            az += step;
                            polar = if dist.abs() >= 1.175_494_4e-38 { (PI / 2.0 - g.aux[2].atan2(dist)) + g.angle } else if g.aux[2] < 0.0 { PI + g.angle } else { g.angle };
                        }
                    } else if g.angle >= 0.0 {
                        polar = rr * g.angle;
                        az = (g.aux[1] - g.aux[0]) * r.f() + g.aux[0];
                    } else {
                        az += step;
                        polar = rr * -g.angle;
                    }
                    let mut off = Vec3::new(dist * az.cos(), dist * az.sin(), 0.0);
                    if matches!(shape, 6 | 7) {
                        let f = r.f();
                        if shape == 6 { off.x *= 1.0 - f; off.y *= 1.0 - f; }
                        off.z = f * g.aux[2];
                    }
                    let sp = speed * polar.sin();
                    let mut v = Vec3::new(sp * az.cos(), sp * az.sin(), speed * polar.cos());
                    if shape == 3 { v *= rr; }
                    let pos = m * off + g.pos;
                    let v = (m * v) * scale;
                    self.spawn(pos, v, g.size, g.grav, g.fric, g.kind, g.bank, g.tex_group, g.template, g.plife, g.link, Some(gi), None);
                }
                1 => {
                    let f = r.f();
                    let pos = m * Vec3::new(f * g.aux[0], f * g.aux[1], f * g.aux[2]) + g.pos;
                    let v = (m * g.vel) * scale;
                    self.spawn(pos, v, g.size, g.grav, g.fric, g.kind, g.bank, g.tex_group, g.template, g.plife, g.link, Some(gi), None);
                }
                2 => {
                    let rad = if g.radius >= 0.0 { r.f() } else { 1.0 };
                    if g.angle >= 0.0 { az = (2.0 * PI as f64 * r.f() as f64) as f32; } else { az += step; }
                    self.gens[gi].aux[0] = speed;
                    let pi = self.spawn(Vec3::ZERO, Vec3::new(az, rad, 0.0), g.size, ax, ay, g.kind | 4, g.bank, g.tex_group, g.template, g.plife, g.link, Some(gi), None);
                    let mut gref = self.gens[gi].clone();
                    if gref.kind & 0x20000 != 0 { gref.pos = Vec3::ZERO; }
                    let p = &mut self.parts[pi];
                    p.pos = tornado_pos(p, &gref, p.vel.x, p.vel.z);
                }
                5 => {
                    let mut f = Vec3::new(r.f(), r.f(), r.f());
                    let snap = |v: f32| if 0.5 < v { 1.0 } else { 0.0 };
                    let a = g.aux;
                    match g.box_sel {
                        1 => f.x = snap(f.x),
                        2 => f.y = snap(f.y),
                        3 => { if r.f() <= a[0] / (a[0] + a[1]) { f.x = snap(f.x) } else { f.y = snap(f.y) } }
                        4 => f.z = snap(f.z),
                        5 => { if r.f() <= a[0] / (a[0] + a[2]) { f.x = snap(f.x) } else { f.z = snap(f.z) } }
                        6 => { if r.f() <= a[1] / (a[1] + a[2]) { f.y = snap(f.y) } else { f.z = snap(f.z) } }
                        7 => {
                            let k = 1.0 / (a[0] * (a[1] + a[2]) + a[1] * a[2]);
                            let t = r.f();
                            if k * a[0] * a[1] <= t { if t <= 1.0 - k * a[0] * a[2] { f.x = snap(f.x) } else { f.y = snap(f.y) } } else { f.z = snap(f.z) }
                        }
                        _ => {}
                    }
                    f -= Vec3::splat(0.5);
                    let (xa, ya, za) = (Vec3::new(a[3], a[4], a[5]), Vec3::new(a[6], a[7], a[8]), Vec3::new(a[9], a[10], a[11]));
                    let local = xa * f.x + ya * f.y + za * f.z;
                    let pos = m * local + g.pos;
                    let l = za.length();
                    let v = (m * (za * (speed / l))) * scale;
                    self.spawn(pos, v, g.size, g.grav, g.fric, g.kind, g.bank, g.tex_group, g.template, g.plife, g.link, Some(gi), None);
                }
                8 => {
                    let pol = if g.aux[4] == 0.0 { PI * r.f() } else { g.aux[4] * r.f() };
                    let azr = (2.0 * PI as f64 * r.f() as f64) as f32;
                    let rad = if g.radius >= 0.0 { g.radius * r.f() } else { -g.radius };
                    let dir = m * Vec3::new(pol.sin() * azr.cos(), pol.sin() * azr.sin(), pol.cos());
                    let mut v = dir * g.aux[0];
                    if g.radius >= 0.0 && g.aux[0] < 0.0 { v *= rad / g.radius; }
                    let pos = dir * rad + g.pos;
                    self.spawn(pos, v * scale, g.size, g.grav, g.fric, g.kind, g.bank, g.tex_group, g.template, g.plife, g.link, Some(gi), None);
                }
                _ => {}
            }
            acc -= 1.0;
        }
        self.gens[gi].acc = acc;
    }

    /// One 60 Hz frame: `psInterpretParticles` then `psExecGenerator`.
    fn frame(&mut self, cam: Option<(Vec3, Vec3)>) {
        let n = self.parts.len();
        for i in 0..n { if self.parts[i].alive { self.interpret(i); } }
        // a generator whose children are gone and whose time is up goes
        let gn = self.gens.len();
        for gi in 0..gn {
            if !self.gens[gi].alive { continue; }
            let g = &mut self.gens[gi];
            if g.rate >= 0.0 { g.acc += g.rate * self.rng.f(); } else { g.acc -= g.rate; }
            if self.gens[gi].acc >= 1.0 { self.emit(gi, cam); }
            let g = &mut self.gens[gi];
            if g.life != 0 {
                g.life -= 1;
                if g.life == 0 {
                    if g.gtype & 0x80 != 0 { for p in self.parts.iter_mut().filter(|p| p.gen == Some(gi)) { p.alive = false; } }
                    if self.gens[gi].children <= 0 { self.gens[gi].alive = false; } else { let g = &mut self.gens[gi]; g.rate = 0.0; g.life = 1; }
                }
            }
        }
        // compact: dead particles go; generators stay indexed until nothing refers to them
        self.parts.retain(|p| p.alive);
        if self.gens.iter().all(|g| !g.alive) && self.parts.is_empty() { self.gens.clear(); self.ties.clear(); }
    }
}

fn lerp_u8(to: u8, from: u8, k: f32) -> u8 { (to as f32 + k * (from as f32 - to as f32)) as u8 }
fn clamp_add(v: u8, d: f32) -> u8 { (v as f32 + d).clamp(0.0, 255.0) as u8 }
fn resolve(cur: &mut [u8; 4], to: &[u8; 4], t: u16, c: u16) {
    if t != 0 { let k = c as f32 / t as f32; for ch in 0..4 { cur[ch] = lerp_u8(to[ch], cur[ch], k); } }
}

/// The colour a particle shows now (start value faded toward the target by the counter).
fn shown(cur: [u8; 4], to: [u8; 4], t: u16, c: u16) -> [u8; 4] {
    if t == 0 { return cur; }
    let k = c as f32 / t as f32;
    [0, 1, 2, 3].map(|ch| lerp_u8(to[ch], cur[ch], k))
}

/// Tornado position for angle `theta` and height `h` (`psInterpretParticle0`'s kind 4 branch).
fn tornado_pos(p: &Particle, g: &Generator, theta: f32, h: f32) -> Vec3 {
    let (s1, c1) = p.grav.sin_cos();
    let (s2, c2) = p.fric.sin_cos();
    let r = (h * g.angle.abs().tan() + g.radius.abs()) * p.vel.y;
    let x0 = r * theta.cos();
    let y0 = r * theta.sin();
    g.pos + Vec3::new(x0 * c2 + h * s2, c2 * (h * s1) + s2 * (-x0 * s1) + y0 * c1, c2 * (h * c1) + s2 * (-x0 * c1) - y0 * s1)
}

/// `modifyDir`: turn the velocity by `angle` toward a random side.
fn modify_dir(v: &mut Vec3, angle: f32, r: &mut Rng) {
    let a = atan2_safe(v.y, v.z);
    let (sa, ca) = a.sin_cos();
    let b = atan2_safe(v.x, v.y * sa + v.z * ca);
    let (sb, cb) = b.sin_cos();
    let len = v.length();
    let az = (2.0f64 * PI as f64 * r.f() as f64) as f32;
    let s = len * angle.sin();
    let x = s * az.cos();
    let y = s * az.sin();
    let z = len * angle.cos();
    *v = Vec3::new(x * cb + z * sb, cb * (z * sa) + sb * (-x * sa) + y * ca, cb * (z * ca) + sb * (-x * ca) - y * sa);
}

// ---------------------------------------------------------------- drawing

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct BatchKey { bank: usize, tex: Option<(u16, u8, u8)>, blend: u32, ztest: bool, zwrite: bool, trail: bool, mirror: u8 }

#[derive(Clone, Copy, Debug, ShaderType, Default)]
pub struct PsUniform {
    /// x: textured, y: blend mode, z: trail (vertex alpha), w: mirror (1 S, 2 T)
    pub mode: UVec4,
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(PsKey)]
pub struct ParticleMat {
    #[uniform(0)]
    pub u: PsUniform,
    #[texture(1)] #[sampler(2)]
    pub tex: Option<Handle<Image>>,
    pub key: PsKey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct PsKey { pub blend: u8, pub ztest: bool, pub zwrite: bool }

impl From<&ParticleMat> for PsKey { fn from(m: &ParticleMat) -> Self { m.key } }

const SHADER: &str = "embedded://colo_viewer/particles.wgsl";

impl Material for ParticleMat {
    fn vertex_shader() -> ShaderRef { SHADER.into() }
    fn fragment_shader() -> ShaderRef { SHADER.into() }
    fn alpha_mode(&self) -> AlphaMode { AlphaMode::Blend }
    fn enable_prepass() -> bool { false }
    fn enable_shadows() -> bool { false }
    fn specialize(_: &MaterialPipeline, desc: &mut RenderPipelineDescriptor, layout: &MeshVertexBufferLayoutRef,
                  key: MaterialPipelineKey<Self>) -> Result<(), SpecializedMeshPipelineError> {
        desc.vertex.buffers = vec![layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0), Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(2), ATTR_ENV.at_shader_location(3),
        ])?];
        desc.primitive.cull_mode = None;
        let k = key.bind_group_data;
        let c = match k.blend {
            1 => BlendComponent { src_factor: BlendFactor::SrcAlpha, dst_factor: BlendFactor::One, operation: BlendOperation::Add },
            2 => BlendComponent { src_factor: BlendFactor::One, dst_factor: BlendFactor::One, operation: BlendOperation::ReverseSubtract },
            3 => BlendComponent { src_factor: BlendFactor::Dst, dst_factor: BlendFactor::Zero, operation: BlendOperation::Add },
            _ => BlendComponent { src_factor: BlendFactor::SrcAlpha, dst_factor: BlendFactor::OneMinusSrcAlpha, operation: BlendOperation::Add },
        };
        if let Some(f) = desc.fragment.as_mut() {
            for t in f.targets.iter_mut().flatten() { t.blend = Some(BlendState { color: c, alpha: BlendComponent::OVER }); }
        }
        if let Some(ds) = desc.depth_stencil.as_mut() {
            ds.depth_compare = Some(if k.ztest { CompareFunction::GreaterEqual } else { CompareFunction::Always });
            ds.depth_write_enabled = Some(k.zwrite);
        }
        Ok(())
    }
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "particles.wgsl");
        app.add_plugins(MaterialPlugin::<ParticleMat>::default()).insert_resource(Particles::default());
    }
}

/// Fire PTCL keys of linked models, register their point joints (tracks 0x2A), keep tied generators on their
/// joints, run the 60 Hz frames.
#[allow(clippy::type_complexity)]
pub fn update(time: Res<Time>, mut ps: ResMut<Particles>, mut links: Query<(&Animator, &mut ParticleLink)>, globals: Query<&GlobalTransform>,
              cams: Query<&GlobalTransform, With<crate::GameCam>>, lights: Option<Res<crate::tevmat::RoomLights>>) {
    let ps = &mut *ps;
    if ps.banks.is_empty() { return; }
    if let Some(l) = lights { ps.light = l.unlit_sum(); }
    ps.acc = if ps.acc < 0.0 { 1.0 } else { (ps.acc + time.delta_secs() * 60.0).min(4.0) };
    let cam = cams.single().ok().map(|g| (g.translation(), g.up().as_vec3()));
    // keys passed since the last look (the animation itself advances in `animate`)
    for (anim, mut link) in &mut links {
        let Some(play) = anim.active.first() else { continue };
        let a = &anim.anims[play.anim];
        let now = (play.anim, play.frame);
        let prev = link.last.filter(|l| l.0 == play.anim).map(|l| l.1);
        link.last = Some(now);
        for t in a.tracks.iter().filter(|t| t.channel == 0x28 || t.channel == 0x2A) {
            for k in &t.keys {
                let hit = match prev {
                    None => k.frame <= now.1,
                    Some(p) if p <= now.1 => k.frame > p && k.frame <= now.1,
                    Some(p) => k.frame > p || k.frame <= now.1,
                };
                if !hit { continue; }
                // the key holds the raw word
                let v = k.value.to_bits();
                if t.channel == 0x2A {
                    // psSetPointJObjNodup(jobj, slot): the joint becomes point `slot` (0 removes it)
                    let slot = if v <= 8 { v as usize } else { k.value as usize };
                    let Some(&e) = anim.joints.get(t.joint) else { continue };
                    for p in ps.points.iter_mut() { if *p == Some(e) { *p = None; } }
                    if (1..=8).contains(&slot) { ps.points[slot] = Some(e); }
                    continue;
                }
                // PTCL: generator id << 6 | bank slot
                if ps.trace { println!("particles: PTCL key frame {} id {} joint {}", k.frame, v >> 6, t.joint); }
                if let Some(g) = ps.create_generator(0, link.bank, (v >> 6) as usize) {
                    if link.attach != 0 { if let Some(&e) = anim.joints.get(t.joint) { ps.ties.push((g, e, link.attach)); } }
                }
            }
        }
    }
    while ps.acc >= 1.0 {
        ps.acc -= 1.0;
        // 0x801183EC: tied generators copy the joint's position (and, mode 4, its rotation)
        let ties = ps.ties.clone();
        for (g, e, mode) in ties {
            let (Some(gen), Ok(gt)) = (ps.gens.get_mut(g), globals.get(e)) else { continue };
            if !gen.alive { continue; }
            let (_, rot, tr) = gt.to_scale_rotation_translation();
            if matches!(mode, 1 | 4 | 6 | 7) { gen.pos = tr; }
            if matches!(mode, 2 | 4 | 5 | 7) { gen.rot = Mat3::from_quat(rot); }
        }
        for s in 1..9 { ps.point_pos[s] = ps.points[s].and_then(|e| globals.get(e).ok()).map(|g| g.translation()); }
        ps.frame(cam);
        // particles carrying a point (0xBF): the point sits where the particle is
        for p in ps.parts.iter().filter(|p| p.alive && p.kind & 0x8000 != 0) {
            let slot = ((p.kind >> 12) & 7) as usize + 1;
            ps.point_pos[slot] = Some(p.pos);
        }
    }
}

impl Particles {
    fn world(&self, p: &Particle, v: Vec3) -> Vec3 {
        match p.local.and_then(|g| self.gens.get(g)) { Some(g) => g.pos + g.rot * (v * g.scale), None => v }
    }

    /// The image a particle shows: its pose, re-decoded with its palette when it picks one (0xE3, or ComTLUT → 0).
    fn image(&mut self, bank: usize, group: u16, pose: u8, palette: u8, images: &mut Assets<Image>) -> Option<Handle<Image>> {
        let b = self.banks.get(bank)?;
        let g = b.gpt.tex_groups.get(group as usize)?.as_ref()?;
        if palette == 0xFF || g.palettes.is_empty() { return b.textures.get(group as usize)?.get(pose as usize).cloned(); }
        let key = (bank, group, pose, palette);
        if let Some(h) = self.pal_images.get(&key) { return Some(h.clone()); }
        let rgba = g.with_palette(pose as usize, palette as usize)?;
        let h = images.add(Image::new(Extent3d { width: g.width as u32, height: g.height as u32, depth_or_array_layers: 1 }, TextureDimension::D2,
            rgba, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD));
        self.pal_images.insert(key, h.clone());
        Some(h)
    }
}

/// Build this frame's quads, one mesh per material key, sorted back to front.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn draw(mut commands: Commands, mut ps: ResMut<Particles>, cams: Query<(&GlobalTransform, &Projection), With<crate::GameCam>>,
            mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<ParticleMat>>, roots: Query<Entity, With<crate::RoomRoot>>,
            mut vis: Query<&mut Visibility>, mut images: ResMut<Assets<Image>>) {
    let ps = &mut *ps;
    let Ok((cam, proj)) = cams.single() else { return };
    let (right, up, fwd, eye) = (cam.right().as_vec3(), cam.up().as_vec3(), cam.forward().as_vec3(), cam.translation());
    // world units per screen pixel at distance 1 (480-line screen), for point sprites
    let fov = match proj {
        Projection::Perspective(p) => p.fov,
        Projection::Custom(c) => c.get::<crate::tv::FixedAspect>().map_or(30f32.to_radians(), |f| f.0.fov),
        _ => 30f32.to_radians(),
    };
    let px = 2.0 * (fov / 2.0).tan() / 480.0;
    let mut quads: Vec<(f32, BatchKey, [Vec3; 4], [[f32; 2]; 4], [[f32; 4]; 4], [f32; 4])> = vec![];
    const TC: [[u8; 2]; 16] = [[0, 1], [0, 0], [1, 0], [1, 1], [1, 1], [1, 0], [0, 0], [0, 1], [0, 0], [0, 1], [1, 1], [1, 0], [1, 0], [1, 1], [0, 1], [0, 0]];
    let light = ps.light.clamp(Vec3::ZERO, Vec3::ONE);
    let parts: Vec<Particle> = ps.parts.iter().filter(|p| p.alive && p.size >= EPS_F && p.kind & 0x2000_0000 == 0).cloned().collect();
    for p in &parts {
        let mut prim = shown(p.prim, p.prim_to, p.prim_t, p.prim_c);
        let env = if p.kind & 0x80 != 0 { shown(p.env, p.env_to, p.env_t, p.env_c) } else { [0; 4] };
        let palette = if p.palette != 0xFF { p.palette } else if p.kind & 0x10 != 0 { 0 } else { 0xFF };
        let tex = (p.kind & 0x400 != 0).then_some((p.tex_group, p.pose, palette));
        let trail = p.kind & 0x100000 != 0;
        let mirror = ((p.kind >> 5) & 3) as u8;
        let key = BatchKey { bank: p.bank, tex, blend: (p.kind >> 22) & 3, ztest: p.kind & 0x1000_0000 == 0, zwrite: p.kind & 8 != 0, trail, mirror };
        // lit (0x80000000): the channel is the room's lights on the material colour (gray, alpha) × prim
        if p.kind & 0x8000_0000 != 0 {
            let m = shown([p.mat[0], p.mat[1], 0, 0], [p.mat_to[0], p.mat_to[1], 0, 0], p.mat_t, p.mat_c);
            let a = shown([p.amb[0], p.amb[1], 0, 0], [p.amb_to[0], p.amb_to[1], 0, 0], p.amb_t, p.amb_c);
            for ch in 0..3 {
                let lit = (light[ch] * m[0] as f32 / 255.0 + a[0] as f32 / 255.0 * (1.0 - light[ch])).min(1.0);
                prim[ch] = (prim[ch] as f32 * lit) as u8;
            }
            prim[3] = (prim[3] as u32 * m[1] as u32 / 255) as u8;
        }
        let pos = ps.world(p, p.pos);
        let size = if p.kind & 0x4000_0000 != 0 {
            // DisPoint: a GX point of min(6·size, 255)/6 pixels
            let pixels = (6.0 * p.size).min(255.0) / 6.0;
            0.5 * pixels * px * (pos - eye).dot(fwd).max(0.01)
        } else { p.size };
        let mut a = (right + up) * size;
        let mut b = (right - up) * size;
        // rotation: the particle's own, or (DirVec / trails) the screen direction of its motion
        let prev = if p.kind & 4 == 0 { p.pos - p.vel } else if let Some(g) = p.gen.and_then(|g| ps.gens.get(g)) {
            let mut g = g.clone();
            if p.local.is_some() { g.pos = Vec3::ZERO; }
            tornado_pos(p, &g, p.vel.x - g.grav, p.vel.z - g.aux[0])
        } else { p.pos };
        let prev = ps.world(p, prev);
        let mut ang = p.rot;
        if p.kind & 0x300000 != 0 {
            let d = pos - prev;
            ang = atan2_safe(d.dot(right), d.dot(up));
            if p.kind & 0x200000 != 0 { ang += p.rot; }
        }
        if ang.abs() > 0.01 && p.kind & 0x4000_0000 == 0 {
            let q = Quat::from_axis_angle(a.cross(b).normalize_or(fwd), ang);
            a = q * a;
            b = q * b;
        }
        let flip = ((p.kind >> 16) & 0xC) as usize;
        let uv = [0, 1, 2, 3].map(|k| { let t = TC[flip + k]; [t[0] as f32, t[1] as f32] });
        let c = |al: f32| [prim[0] as f32 / 255.0, prim[1] as f32 / 255.0, prim[2] as f32 / 255.0, al];
        let full = prim[3] as f32 / 255.0;
        let envf = env.map(|v| v as f32 / 255.0);
        let (corners, cols) = if trail && p.kind & 0x4000_0000 == 0 {
            let tail = (prim[3] as f32 * p.trail).min(255.0) / 255.0;
            ([prev - a, pos - b, pos + a, prev + b], [c(tail), c(full), c(full), c(tail)])
        } else {
            ([pos - a, pos - b, pos + a, pos + b], [c(full); 4])
        };
        quads.push(((pos - eye).dot(fwd), key, corners, uv, cols, envf));
    }
    quads.sort_by(|x, y| y.0.total_cmp(&x.0));
    // rebuild batches
    let mut used = vec![false; ps.batches.len()];
    let mut groups: Vec<(BatchKey, Vec<usize>)> = vec![];
    for (i, q) in quads.iter().enumerate() {
        match groups.iter_mut().find(|g| g.0 == q.1) { Some(g) => g.1.push(i), None => groups.push((q.1, vec![i])) }
    }
    let Ok(room_root) = roots.single() else { return };
    for (key, idx) in groups {
        let (mut pos, mut uvs, mut cols, mut envs, mut ind) = (vec![], vec![], vec![], vec![], vec![]);
        for &i in &idx {
            let q = &quads[i];
            let base = pos.len() as u32;
            for k in 0..4 { pos.push(q.2[k].to_array()); uvs.push(q.3[k]); cols.push(q.4[k]); envs.push(q.5); }
            ind.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, pos);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, cols);
        mesh.insert_attribute(ATTR_ENV, envs);
        mesh.insert_indices(Indices::U32(ind));
        if let Some(bi) = ps.batches.iter().position(|b| b.0 == key) {
            used[bi] = true;
            if let Some(mut m) = meshes.get_mut(&ps.batches[bi].2) { *m = mesh; }
            if let Ok(mut v) = vis.get_mut(ps.batches[bi].1) { *v = Visibility::Visible; }
            continue;
        }
        let texh = key.tex.and_then(|(g, p, pal)| ps.image(key.bank, g, p, pal, &mut images));
        let mat = mats.add(ParticleMat {
            u: PsUniform { mode: UVec4::new(texh.is_some() as u32, key.blend, key.trail as u32, key.mirror as u32) },
            tex: texh, key: PsKey { blend: key.blend as u8, ztest: key.ztest, zwrite: key.zwrite },
        });
        let mh = meshes.add(mesh);
        let e = commands.spawn((Mesh3d(mh.clone()), MeshMaterial3d(mat), Transform::default(), bevy::camera::visibility::NoFrustumCulling)).id();
        commands.entity(room_root).add_child(e);
        ps.batches.push((key, e, mh));
        used.push(true);
    }
    for (bi, u) in used.iter().enumerate() {
        if !u { if let Some(b) = ps.batches.get(bi) { if let Ok(mut v) = vis.get_mut(b.1) { *v = Visibility::Hidden; } } }
    }
}
