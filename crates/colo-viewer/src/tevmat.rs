//! HSD materials as a GX TEV interpreter (`tev.wgsl`): the TExp graph `MObjMakeTExp` builds
//! (`colo_formats::tev`), up to 8 TObjs with their own texture coordinates (UV sets, reflection and
//! highlight maps), KONST/TEV0/TEV1 colours, and the pixel engine (`HSD_SetupRenderModeWithCustomPE`
//! 0x801B294C: alpha compare, blend mode, z mode).
//!
//! TEV maths runs on gamma-space bytes like the hardware. The rasterized channels (COLOR0A0, COLOR1A1) are the
//! GX lighting of the room's LObjs (`RoomLights`, `HSD_LObjSetup`); blending happens in linear space on Bevy's
//! sRGB target.

use bevy::asset::embedded_asset;
use bevy::mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef, VertexFormat};
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState, ColorWrites, CompareFunction, Face,
    RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError,
};
use bevy::shader::ShaderRef;
use colo_formats::hsd::{self, TexSrt};
use colo_formats::hsdanim::LightAnim;
use colo_formats::tev::{self, Program, Src, Values};

pub const MAX_LAYERS: usize = 8;
const MAX_NODES: usize = 16;
const MAX_CONSTS: usize = 8;
/// GX hardware lights (GX_LIGHT0..7).
pub const MAX_LIGHTS: usize = 8;

/// Raw texture coordinate sets (GX_VA_TEX0..7), two per attribute.
pub const ATTR_UV01: MeshVertexAttribute = MeshVertexAttribute::new("TevUv01", 0x7E5_0001, VertexFormat::Float32x4);
pub const ATTR_UV23: MeshVertexAttribute = MeshVertexAttribute::new("TevUv23", 0x7E5_0002, VertexFormat::Float32x4);
pub const ATTR_UV45: MeshVertexAttribute = MeshVertexAttribute::new("TevUv45", 0x7E5_0003, VertexFormat::Float32x4);
pub const ATTR_UV67: MeshVertexAttribute = MeshVertexAttribute::new("TevUv67", 0x7E5_0004, VertexFormat::Float32x4);
/// GX_VA_NBT binormal and tangent (bump-mapped meshes only; the pipeline gets the `TEV_NBT` shader def).
pub const ATTR_BINORMAL: MeshVertexAttribute = MeshVertexAttribute::new("TevBinormal", 0x7E5_0005, VertexFormat::Float32x3);
pub const ATTR_TANGENT: MeshVertexAttribute = MeshVertexAttribute::new("TevTangent", 0x7E5_0006, VertexFormat::Float32x3);

#[derive(Clone, Copy, Debug, ShaderType)]
pub struct TevUniform {
    pub nodes: [UVec4; 2 * MAX_NODES],
    pub consts: [Vec4; MAX_CONSTS],
    pub layer_s: [Vec4; MAX_LAYERS],
    pub layer_t: [Vec4; MAX_LAYERS],
    pub info: UVec4,
    pub misc: Vec4,
    /// Material ambient colour bytes (`HSD_SetMaterialColor`, animated by MatAnim tracks 1..3).
    pub mat_amb: Vec4,
    /// The room lights (`RoomLights::block`).
    pub lights: [Vec4; 5 * MAX_LIGHTS],
    /// x: ambient light colour bytes as r | g << 8 | b << 16 | a << 24 (0xFF000000 without one), y: light count.
    pub light_info: UVec4,
}

impl Default for TevUniform {
    fn default() -> Self {
        TevUniform { nodes: [UVec4::ZERO; 2 * MAX_NODES], consts: [Vec4::ZERO; MAX_CONSTS], layer_s: [Vec4::ZERO; MAX_LAYERS],
            layer_t: [Vec4::ZERO; MAX_LAYERS], info: UVec4::ZERO, misc: Vec4::ZERO,
            mat_amb: Vec4::ZERO, lights: [Vec4::ZERO; 5 * MAX_LIGHTS], light_info: UVec4::new(0xFF00_0000, 0, 0, 0) }
    }
}

/// Pipeline state from the render mode / PE desc.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct TevKey {
    /// 0 none, 1 back, 2 front.
    pub cull: u8,
    /// GX blend mode type (0 none, 1 blend, 2 logic, 3 subtract) and src/dst factors.
    pub blend: u8,
    pub src: u8,
    pub dst: u8,
    pub z_enable: bool,
    /// GXCompare (0 never .. 7 always).
    pub z_func: u8,
    pub z_update: bool,
    pub color_update: bool,
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(TevKey)]
pub struct TevMaterial {
    #[uniform(0)]
    pub tev: TevUniform,
    #[texture(1)] #[sampler(2)] pub t0: Option<Handle<Image>>,
    #[texture(3)] #[sampler(4)] pub t1: Option<Handle<Image>>,
    #[texture(5)] #[sampler(6)] pub t2: Option<Handle<Image>>,
    #[texture(7)] #[sampler(8)] pub t3: Option<Handle<Image>>,
    #[texture(9)] #[sampler(10)] pub t4: Option<Handle<Image>>,
    #[texture(11)] #[sampler(12)] pub t5: Option<Handle<Image>>,
    #[texture(13)] #[sampler(14)] pub t6: Option<Handle<Image>>,
    #[texture(15)] #[sampler(16)] pub t7: Option<Handle<Image>>,
    pub key: TevKey,
    pub phase: AlphaMode,
}

impl From<&TevMaterial> for TevKey {
    fn from(m: &TevMaterial) -> Self { m.key }
}

impl TevMaterial {
    pub fn texture_mut(&mut self, i: usize) -> Option<&mut Option<Handle<Image>>> {
        Some(match i { 0 => &mut self.t0, 1 => &mut self.t1, 2 => &mut self.t2, 3 => &mut self.t3, 4 => &mut self.t4, 5 => &mut self.t5, 6 => &mut self.t6, 7 => &mut self.t7, _ => return None })
    }
}

const SHADER: &str = "embedded://colo_viewer/tev.wgsl";

impl Material for TevMaterial {
    fn vertex_shader() -> ShaderRef { SHADER.into() }
    fn fragment_shader() -> ShaderRef { SHADER.into() }
    fn alpha_mode(&self) -> AlphaMode { self.phase }
    fn enable_prepass() -> bool { false }
    fn enable_shadows() -> bool { false }

    fn specialize(_: &MaterialPipeline, desc: &mut RenderPipelineDescriptor, layout: &MeshVertexBufferLayoutRef,
                  key: MaterialPipelineKey<Self>) -> Result<(), SpecializedMeshPipelineError> {
        let mut attrs = vec![
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(2),
            ATTR_UV01.at_shader_location(3),
            ATTR_UV23.at_shader_location(4),
            ATTR_UV45.at_shader_location(5),
            ATTR_UV67.at_shader_location(6),
        ];
        if layout.0.contains(Mesh::ATTRIBUTE_JOINT_INDEX) && layout.0.contains(Mesh::ATTRIBUTE_JOINT_WEIGHT) {
            attrs.push(Mesh::ATTRIBUTE_JOINT_INDEX.at_shader_location(7));
            attrs.push(Mesh::ATTRIBUTE_JOINT_WEIGHT.at_shader_location(8));
        }
        if layout.0.contains(ATTR_BINORMAL) && layout.0.contains(ATTR_TANGENT) {
            attrs.push(ATTR_BINORMAL.at_shader_location(9));
            attrs.push(ATTR_TANGENT.at_shader_location(10));
            desc.vertex.shader_defs.push("TEV_NBT".into());
            if let Some(f) = desc.fragment.as_mut() { f.shader_defs.push("TEV_NBT".into()); }
        }
        desc.vertex.buffers = vec![layout.0.get_layout(&attrs)?];
        let k = key.bind_group_data;
        desc.primitive.cull_mode = match k.cull { 1 => Some(Face::Back), 2 => Some(Face::Front), _ => None };
        if let Some(f) = desc.fragment.as_mut() {
            for t in f.targets.iter_mut().flatten() {
                t.blend = blend_state(k.blend, k.src, k.dst);
                if !k.color_update { t.write_mask = ColorWrites::empty(); }
            }
        }
        if let Some(ds) = desc.depth_stencil.as_mut() {
            // Bevy's depth is reversed (near = 1)
            ds.depth_compare = Some(if !k.z_enable { CompareFunction::Always } else {
                match k.z_func { 0 => CompareFunction::Never, 1 => CompareFunction::Greater, 2 => CompareFunction::Equal, 3 => CompareFunction::GreaterEqual,
                    4 => CompareFunction::Less, 5 => CompareFunction::NotEqual, 6 => CompareFunction::LessEqual, _ => CompareFunction::Always }
            });
            ds.depth_write_enabled = Some(k.z_enable && k.z_update);
        }
        Ok(())
    }
}

fn factor(f: u8, is_src: bool) -> BlendFactor {
    // GX_BL_SRCCLR / INVSRCCLR mean the other operand's colour (dst colour as a source factor)
    match f {
        0 => BlendFactor::Zero, 1 => BlendFactor::One,
        2 => if is_src { BlendFactor::Dst } else { BlendFactor::Src },
        3 => if is_src { BlendFactor::OneMinusDst } else { BlendFactor::OneMinusSrc },
        4 => BlendFactor::SrcAlpha, 5 => BlendFactor::OneMinusSrcAlpha,
        6 => BlendFactor::DstAlpha, _ => BlendFactor::OneMinusDstAlpha,
    }
}

fn blend_state(ty: u8, src: u8, dst: u8) -> Option<BlendState> {
    let c = match ty {
        1 => BlendComponent { src_factor: factor(src, true), dst_factor: factor(dst, false), operation: BlendOperation::Add },
        3 => BlendComponent { src_factor: BlendFactor::One, dst_factor: BlendFactor::One, operation: BlendOperation::ReverseSubtract },
        _ => return None, // GX_BM_NONE; GX_BM_LOGIC is not modelled
    };
    Some(BlendState { color: c, alpha: BlendComponent::OVER })
}

pub struct TevPlugin;

impl Plugin for TevPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "tev.wgsl");
        app.add_plugins(MaterialPlugin::<TevMaterial>::default());
    }
}

/// Insert the TEV vertex attributes (vertex colours and the raw texture coordinate sets).
pub fn insert_attributes(mesh: &mut Mesh, m: &hsd::Mesh) {
    let n = m.positions.len();
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, m.colors.clone());
    for (k, attr) in [ATTR_UV01, ATTR_UV23, ATTR_UV45, ATTR_UV67].into_iter().enumerate() {
        let set = |i: usize| m.uv_sets.get(i).filter(|s| s.len() == n);
        let (a, b) = (set(2 * k), set(2 * k + 1));
        let v: Vec<[f32; 4]> = (0..n).map(|i| {
            let x = a.map_or([0.0; 2], |s| s[i]);
            let y = b.map_or([0.0; 2], |s| s[i]);
            [x[0], x[1], y[0], y[1]]
        }).collect();
        mesh.insert_attribute(attr, v);
    }
    if m.binormals.len() == n && m.tangents.len() == n {
        mesh.insert_attribute(ATTR_BINORMAL, m.binormals.clone());
        mesh.insert_attribute(ATTR_TANGENT, m.tangents.clone());
    }
}

fn input_word(inp: &tev::Input) -> u32 {
    let (kind, idx) = match inp.src {
        Src::Zero => (0, 0),
        Src::Tex => (1, 0),
        Src::Ras => (2, 0),
        Src::Const(i) => (3, i as u32),
        Src::Node(i) => (4, i as u32),
    };
    kind | (inp.sel as u32) << 3 | (idx & 31) << 7
}

fn op_word(op: &Option<tev::Op>) -> (u32, u32, u32) {
    let Some(op) = op else { return (0, 0, 0) };
    let w = (op.op as u32 & 1) | (op.bias as u32 & 3) << 1 | (op.scale as u32 & 3) << 3 | ((op.clamp != 0) as u32) << 5 | 64;
    let i = op.inputs.map(|x| input_word(&x));
    (w, i[0] | i[1] << 16, i[2] | i[3] << 16)
}

/// The static part of the uniform (program, channels, alpha test).
pub fn program_uniform(p: &Program, mat: &hsd::Material) -> TevUniform {
    let mut u = TevUniform::default();
    let n = p.nodes.len().min(MAX_NODES);
    for (k, node) in p.nodes.iter().take(n).enumerate() {
        let layer = node.layer.filter(|&l| l < MAX_LAYERS).map_or(255, |l| l as u32);
        let chan = node.chan.map_or(0, |c| c as u32);
        let (cw, ca, cb) = op_word(&node.color);
        let (aw, aa, ab) = op_word(&node.alpha);
        u.nodes[2 * k] = UVec4::new(layer | chan << 8 | (node.bump as u32) << 16, cw, aw, 0);
        u.nodes[2 * k + 1] = UVec4::new(ca, cb, aa, ab);
    }
    let c = &p.channels;
    let flags = c.lit as u32 | (c.color_vtx as u32) << 1 | (c.alpha_vtx as u32) << 2 | (c.specular as u32) << 3;
    u.info = UVec4::new(n as u32, flags, mat.layers.len().min(MAX_LAYERS) as u32, alpha_test(mat));
    // material shininess: the specular lights' GXInitLightShininess (HSD_LObjSetupSpecular 0x801A6098)
    // y: the bump TObj's layer (its GX_TG_BUMPn coordinate), -1 without one
    let bump = p.bump.filter(|&l| l < MAX_LAYERS).map_or(-1.0, |l| l as f32);
    // z: COLO_NO_BUMP drops the emboss offset (debug comparison)
    u.misc = Vec4::new(mat.shininess, bump, std::env::var_os("COLO_NO_BUMP").is_some() as u32 as f32, 0.0);
    set_mat_ambient(&mut u, mat.ambient.map(|x| (x * 255.0).round().clamp(0.0, 255.0) as u8));
    u
}

pub fn set_mat_ambient(u: &mut TevUniform, c: [u8; 4]) {
    u.mat_amb = Vec4::new(c[0] as f32, c[1] as f32, c[2] as f32, c[3] as f32);
}

/// The room's active GX lights as `HSD_LObjSetup` (0x801A4F54) assigns them, in world space (GX lights in view
/// space; the view matrix is rigid, so the maths is the same). Per light, 5 vec4s:
/// [position (infinite: WObj position x 1048576), bits (1 diffuse, 2 specular, kind << 2)],
/// [colour bytes, 0], [spot direction (normalised interest - position), a0], [a1, a2, k0, k1],
/// [k2, specular vector: infinite = spot direction, point/spot = position].
/// Animated lights play animation 0 (`floorInitMap`: GSlightSetAnimIndex 0, rate 0.5, looping in
/// `GSlightUpdate` 0x800DC6D8: apply the frame, advance, wrap at frame count - 1).
#[derive(Resource, Clone)]
pub struct RoomLights {
    pub block: [Vec4; 5 * MAX_LIGHTS],
    pub info: UVec4,
    lights: Vec<hsd::Light>,
    anims: Vec<Option<LightAnim>>,
    frames: Vec<f32>,
}

impl RoomLights {
    /// Ambient plus every light's colour (0..1), the GX channel with diffuse function NONE and no attenuation:
    /// what a lit particle gets.
    pub fn unlit_sum(&self) -> Vec3 {
        self.lights.iter().fold(Vec3::ZERO, |acc, l| acc + Vec3::new(l.color[0] as f32, l.color[1] as f32, l.color[2] as f32) / 255.0)
    }

    pub fn new(lights: &[hsd::Light], anims: &[Option<LightAnim>]) -> RoomLights {
        let anims: Vec<Option<LightAnim>> = (0..lights.len()).map(|i| anims.get(i).cloned().flatten()
            .filter(|a| !(a.lobj.is_empty() && a.position.is_empty() && a.interest.is_empty()))).collect();
        let mut r = RoomLights { block: [Vec4::ZERO; 5 * MAX_LIGHTS], info: UVec4::ZERO, lights: lights.to_vec(), frames: vec![0.0; anims.len()], anims };
        r.build();
        r
    }

    /// Advance the light animations by `dt` game frames; true when the lights changed.
    pub fn tick(&mut self, dt: f32) -> bool {
        if self.anims.iter().all(Option::is_none) { return false; }
        let old = (self.block, self.info);
        self.build();
        for (a, f) in self.anims.iter().zip(self.frames.iter_mut()) {
            let Some(a) = a else { continue };
            *f += 0.5 * dt;
            let end = a.frame_count - 1.0;
            if end > 0.0 && *f >= end { *f -= end; }
        }
        old != (self.block, self.info)
    }

    fn build(&mut self) {
        let lights: Vec<hsd::Light> = self.lights.iter().zip(&self.anims).zip(&self.frames).map(|((l, a), f)| {
            let mut l = *l;
            if let Some(a) = a { a.apply(&mut l, *f); }
            l
        }).collect();
        let lights = &lights[..];
        let r = self;
        r.block = [Vec4::ZERO; 5 * MAX_LIGHTS];
        // up to 8 lights with a diffuse/specular/alpha flag, not hidden; the first such ambient light is the
        // ambient (slot 8); spec-only lights use their own slot, diffuse+spec ones take the slots after them
        let usable = |l: &&hsd::Light| l.flags & 0x20 == 0 && l.flags & 0x1C != 0;
        let mut active: Vec<&hsd::Light> = vec![];
        let mut ambient = None;
        for l in lights.iter().filter(usable) {
            if l.kind() == 0 { if ambient.is_none() { ambient = Some(l); } continue; }
            if active.len() == MAX_LIGHTS { break; }
            active.push(l);
        }
        if ambient.is_none() { ambient = lights.iter().filter(usable).find(|l| l.kind() == 0 && l.flags & 0x14 != 0); }
        let mut next_spec = active.len();
        for (i, l) in active.iter().enumerate() {
            let spec = l.flags & 8 != 0 && (l.flags & 0x14 == 0 || { let ok = next_spec < MAX_LIGHTS; next_spec += 1; ok });
            let bits = (l.flags & 4 != 0) as u32 | (spec as u32) << 1 | (l.kind() as u32) << 2;
            let p = Vec3::from(l.position);
            let d = Vec3::from(l.interest) - p;
            let d = if d.abs().max_element() > f32::MIN_POSITIVE { d.normalize() } else { Vec3::Z };
            let pos = if l.kind() == 1 { p * 1_048_576.0 } else { p };
            let a = l.attn();
            let c = l.color;
            let sv = if l.kind() == 1 { d } else { p };
            r.block[5 * i] = pos.extend(bits as f32);
            r.block[5 * i + 1] = Vec4::new(c[0] as f32, c[1] as f32, c[2] as f32, c[3] as f32);
            r.block[5 * i + 2] = d.extend(a[0]);
            r.block[5 * i + 3] = Vec4::new(a[1], a[2], a[3], a[4]);
            r.block[5 * i + 4] = Vec4::new(a[5], sv.x, sv.y, sv.z);
        }
        // HSD_SetupChannelMode: the ambient colour is used when the ambient light has the diffuse flag
        let amb = ambient.filter(|l| l.flags & 4 != 0).map_or(0xFF00_0000, |l| u32::from_le_bytes(l.color));
        r.info = UVec4::new(amb, active.len() as u32, 0, 0);
    }

    pub fn apply(&self, u: &mut TevUniform) {
        u.lights = self.block;
        u.light_info = self.info;
    }
}

/// Give every TEV material the current room's lights (on a room change, and to new materials).
pub fn apply_room_lights(time: Res<Time>, mut lights: ResMut<RoomLights>, mut events: MessageReader<AssetEvent<TevMaterial>>, mut materials: ResMut<Assets<TevMaterial>>) {
    if lights.tick(time.delta_secs().min(0.1) * 60.0) || lights.is_changed() {
        events.clear();
        for (_, m) in materials.iter_mut() { lights.apply(&mut m.tev); }
        return;
    }
    let added: Vec<_> = events.read().filter_map(|e| match e { AssetEvent::Added { id } => Some(*id), _ => None }).collect();
    for id in added {
        if let Some(mut m) = materials.get_mut(id) { lights.apply(&mut m.tev); }
    }
}

/// Constant registers for the current material/texture animation values.
pub fn set_consts(u: &mut TevUniform, p: &Program, v: &Values) {
    for (k, c) in p.consts.iter().take(MAX_CONSTS).enumerate() {
        let x = c.value(v);
        u.consts[k] = Vec4::new(x[0] as f32, x[1] as f32, x[2] as f32, x[3] as f32);
    }
}

/// A layer's texture matrix and coordinate source.
pub fn set_layer(u: &mut TevUniform, i: usize, l: &hsd::TexLayer, srt: &TexSrt) {
    if i >= MAX_LAYERS { return; }
    let m = srt.matrix();
    u.layer_s[i] = Vec4::new(m[0][0], m[0][1], m[0][2], l.coord() as f32);
    u.layer_t[i] = Vec4::new(m[1][0], m[1][1], m[1][2], l.uv_set() as f32);
}

/// Alpha compare word: comp0, ref0, op, comp1, ref1 (PE desc, else GREATER 0 for z-updating XLU, else ALWAYS).
fn alpha_test(m: &hsd::Material) -> u32 {
    let (c0, r0, op, c1, r1) = match m.pe {
        Some(pe) => (pe[9], pe[1], pe[10], pe[11], pe[2]),
        None if m.render_mode & 0x2000_0000 == 0 && m.render_mode & 0x4000_0000 != 0 => (4, 0, 0, 4, 0),
        None => (7, 0, 0, 7, 0),
    };
    (c0 as u32 & 7) | (r0 as u32) << 3 | (op as u32 & 3) << 11 | (c1 as u32 & 7) << 13 | (r1 as u32) << 16
}

/// Pipeline key and render phase (`fn_801B29E4`, the PE half of `HSD_SetupRenderModeWithCustomPE`).
pub fn pipeline(m: &hsd::Material, cull_back: bool, cull_front: bool) -> (TevKey, AlphaMode) {
    let rm = m.render_mode;
    let cull = if cull_back { 1 } else if cull_front { 2 } else { 0 };
    let key = match m.pe {
        Some(pe) => TevKey { cull, blend: pe[4], src: pe[5], dst: pe[6], z_enable: pe[0] & 0x10 != 0, z_func: pe[8], z_update: pe[0] & 0x20 != 0, color_update: pe[0] & 1 != 0 },
        None => TevKey { cull, blend: ((rm >> 30) & 1) as u8, src: 4, dst: 5, z_enable: true, z_func: if rm & 0x0800_0000 != 0 { 7 } else { 3 },
            z_update: rm & 0x2000_0000 == 0, color_update: true },
    };
    let test = alpha_test(m);
    // two ALWAYS compares pass whatever the op; anything else may discard
    let can_discard = !(test & 7 == 7 && (test >> 13) & 7 == 7);
    let phase = if key.blend == 1 || key.blend == 3 || rm & 0x4000_0000 != 0 { AlphaMode::Blend }
        else if can_discard { AlphaMode::Mask(0.5) } else { AlphaMode::Opaque };
    (key, phase)
}
