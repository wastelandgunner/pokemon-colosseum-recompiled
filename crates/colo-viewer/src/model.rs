//! HSD model -> Bevy entities (joint hierarchy, skinned or static meshes) and joint animation playback.

use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::NoFrustumCulling;
use bevy::ecs::system::SystemParam;
use bevy::image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes};
use bevy::mesh::{Mesh, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use colo_formats::hsd::{self, Wrap};
use colo_formats::hsdanim::{Animation, MatAnims};
use crate::tevmat::{self, TevMaterial, TevUniform};
use colo_formats::tev::{self, Program, Values};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(SystemParam)]
pub struct ModelAssets<'w> {
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<TevMaterial>>,
    images: ResMut<'w, Assets<Image>>,
    bindposes: ResMut<'w, Assets<SkinnedMeshInverseBindposes>>,
}

impl ModelAssets<'_> {
    pub fn add_image(&mut self, image: Image) -> Handle<Image> { self.images.add(image) }
    pub fn images(&mut self) -> &mut Assets<Image> { &mut self.images }
    pub fn image_mut(&mut self, h: &Handle<Image>) -> Option<bevy::asset::AssetMut<'_, Image>> { self.images.get_mut(h) }
}

type Srt = ([f32; 3], [f32; 3], [f32; 3]);

/// Character animations play at half rate: the hero's walk/run blend (`updateAnimation`, 0x8012C660) and the
/// people motion code (0x80185B90) both set `GSmodelSetAnimRate(model, 0.5)`.
pub const CHARACTER_ANIM_RATE: f32 = 0.5;

/// One animation playing on a model.
#[derive(Clone, Copy, Debug)]
pub struct Play {
    /// Index into `Animator::anims`.
    pub anim: usize,
    pub frame: f32,
    /// Frames advanced per game frame (60 fps); doors use 0.5, frozen poses 0.
    pub rate: f32,
    /// Loop, or stop and hold on the last frame.
    pub looping: bool,
    /// `GSmodelSetAnimBlend(a, b)` + `GSmodelSetBlendFactor(w)`: a second animation sampled at the same
    /// phase (frame scaled by the frame-count ratio) and mixed in with weight w.
    pub blend: Option<(usize, f32)>,
}

/// Plays HSD joint animations on a spawned model (component on the model's root entity).
#[derive(Component, Clone)]
pub struct Animator {
    pub anims: Arc<Vec<Animation>>,
    /// Several plays at once = different model sets of a map (doors, props).
    pub active: Vec<Play>,
    /// Entity per `Model::joints` entry.
    pub joints: Vec<Entity>,
    pub rest: Vec<Srt>,
    pub is_player: bool,
    /// `GSpartRegisterRotation(part, v, 3)` (the head look): after sampling, joint `.0`'s Euler rotation gets
    /// x += v.x, y = v.y, z += v.z (the mode-3 case of the joint update, e.g. 0x800EB904).
    pub part_rot: Option<(usize, [f32; 3])>,
    /// Joint data for models with IK chains (legs): their poses need full world-matrix solves.
    pub ik: Option<Arc<hsd::Model>>,
    /// Joints the last frame wrote (restored to rest when no play drives them any more).
    touched: Vec<usize>,
    /// Every mesh entity with its model set, MObj/TObj and base material values.
    pub targets: Vec<MeshTarget>,
    /// Material/texture animations (`ModelSet` +8), played by `tex_plays` (`Play::anim` indexes `mats.anims`).
    pub mats: Arc<MatAnims>,
    pub tex_plays: Vec<Play>,
    /// `GSmodelLinkTexAnimToAnim`: set by default (`_modelLoad` flags 0x2001); starting a joint animation then
    /// starts the texture animation with the same index, frame, rate and loop type.
    pub linked: bool,
    /// Animated-texture images: (image table index, TObj) -> handle (sampler from that TObj's wrap modes).
    mat_images: HashMap<(usize, usize), Handle<Image>>,
    /// Model sets hidden with `GSmodelSetVisibility(set, 0)` (`mapObjectSetVisibility`).
    pub hidden_sets: Vec<usize>,
    /// `GSmodelSetTextureChange`: per model set, an image replacing the first TObj image of every material.
    pub tex_override: Vec<(usize, Handle<Image>)>,
    /// Material state last written per target (only changes are written).
    last: Vec<Option<MatState>>,
    vis_applied: Option<Vec<usize>>,
    /// Per joint: JOBJ_HIDDEN in the file, children, and whether any animation has NODE/BRANCH tracks.
    joint_hidden: Vec<bool>,
    children: Vec<Vec<usize>>,
    vis_tracks: bool,
    vis_last: Vec<bool>,
}

/// One spawned mesh: what material animations and per-set operations need to find it.
#[derive(Clone, Debug)]
pub struct MeshTarget {
    pub entity: Entity,
    pub material: Handle<TevMaterial>,
    pub set: usize,
    /// Owning joint (its JOBJ_HIDDEN flag, or HSD_A_J_NODE / BRANCH tracks, hide the mesh).
    pub joint: usize,
    pub mobj: usize,
    /// The HSD material (layers, base colours) and its TEV program.
    pub mat: Arc<hsd::Material>,
    pub program: Arc<Program>,
    /// Per layer: the image (sampler from the TObj's wrap modes).
    pub base_textures: Vec<Option<Handle<Image>>>,
    /// The uniform with the static program, channels and alpha test.
    base_uniform: TevUniform,
}

/// What a material animation wrote last: constant registers, layer matrices, layer images.
#[derive(Clone, Debug, PartialEq)]
struct MatState { consts: Vec<[u8; 4]>, ambient: [u8; 4], mtx: Vec<[[f32; 3]; 2]>, textures: Vec<Option<AssetId<Image>>> }

impl Animator {
    pub fn player(mut self) -> Self {
        self.is_player = true;
        self
    }
    pub fn find(&self, model_set: usize, index: usize) -> Option<usize> {
        self.anims.iter().position(|a| a.model_set == model_set && a.index == index)
    }
    /// Loop the model-set-0 animation with this HSD index (characters).
    pub fn set(&mut self, hsd_index: usize) {
        self.active = self.find(0, hsd_index).map(|anim| Play { anim, frame: 0.0, rate: CHARACTER_ANIM_RATE, looping: true, blend: None }).into_iter().collect();
        if self.linked { self.play_tex(0, hsd_index, 0.0, CHARACTER_ANIM_RATE, true); }
    }
    /// Play the model-set-0 animation with this HSD index once, holding its last frame.
    pub fn set_once(&mut self, hsd_index: usize) {
        self.active = self.find(0, hsd_index).map(|anim| Play { anim, frame: 0.0, rate: CHARACTER_ANIM_RATE, looping: false, blend: None }).into_iter().collect();
        if self.linked { self.play_tex(0, hsd_index, 0.0, CHARACTER_ANIM_RATE, false); }
    }
    /// `GSmodelSetTexAnimIndex/Frame/Rate` (+ loop type): replace the texture animation on `model_set`.
    pub fn play_tex(&mut self, model_set: usize, index: usize, frame: f32, rate: f32, looping: bool) {
        // an index past the set's texture animations changes nothing (GSmodelSetTexAnimIndex)
        let mats = Arc::clone(&self.mats);
        if let Some(anim) = mats.find(model_set, index) {
            self.tex_plays.retain(|p| mats.anims[p.anim].model_set != model_set);
            let frame = if frame < 0.0 { (mats.anims[anim].end_frame - 1.0).max(0.0) } else { frame };
            self.tex_plays.push(Play { anim, frame, rate, looping, blend: None });
        }
    }
    /// `GSmodelStopAnimation` on one model set: the animation holds its frame (and a linked texture animation).
    pub fn stop_set(&mut self, model_set: usize) {
        let (anims, mats) = (Arc::clone(&self.anims), Arc::clone(&self.mats));
        for p in self.active.iter_mut().filter(|p| anims[p.anim].model_set == model_set) { p.rate = 0.0; }
        if self.linked { for p in self.tex_plays.iter_mut().filter(|p| mats.anims[p.anim].model_set == model_set) { p.rate = 0.0; } }
    }
    /// `GSmodelSetVisibility` on one model set.
    pub fn set_visible(&mut self, model_set: usize, on: bool) {
        self.hidden_sets.retain(|&s| s != model_set);
        if !on { self.hidden_sets.push(model_set); }
    }
    /// The hero/partner walk blend (`updateAnimation`): loop `a` at `rate`, optionally mixed with `b`
    /// (weight w). Switching animations keeps the phase (frame scaled to the new length), as the game does.
    pub fn set_blend(&mut self, a: usize, b: Option<(usize, f32)>, rate: f32) {
        let Some(ai) = self.find(0, a) else { return };
        let bi = b.and_then(|(bh, w)| self.find(0, bh).map(|i| (i, w)));
        let anims = Arc::clone(&self.anims);
        match self.active.first_mut() {
            Some(p) if p.looping && p.anim == ai => { p.blend = bi; p.rate = rate; }
            _ => {
                let frame = self.active.first().map_or(0.0, |p| {
                    let (old, new) = (anims[p.anim].end_frame, anims[ai].end_frame);
                    if old > 0.0 && p.looping { p.frame / old * new } else { 0.0 }
                });
                self.active = vec![Play { anim: ai, frame, rate, looping: true, blend: bi }];
                if self.linked { self.play_tex(0, a, frame, rate, true); }
            }
        }
    }
    /// HSD index of the model-set-0 animation playing (characters).
    pub fn current(&self) -> Option<usize> {
        self.active.first().map(|p| self.anims[p.anim].index)
    }
    /// Replace whatever plays on `model_set` (map objects: doors, props); a linked texture animation follows.
    pub fn play_set(&mut self, model_set: usize, index: usize, frame: f32, rate: f32, looping: bool) {
        let anims = Arc::clone(&self.anims);
        self.active.retain(|p| anims[p.anim].model_set != model_set);
        if let Some(anim) = self.find(model_set, index) {
            let frame = if frame < 0.0 { (anims[anim].end_frame - 1.0).max(0.0) } else { frame };
            self.active.push(Play { anim, frame, rate, looping, blend: None });
        }
        if self.linked { self.play_tex(model_set, index, frame, rate, looping); }
    }
    /// A one-shot animation (door) is still running.
    pub fn busy(&self) -> bool {
        self.active.iter().any(|p| !p.looping && p.rate > 0.0 && p.frame < self.anims[p.anim].end_frame - 1.0)
    }
    /// Model sets that have joint or material/texture animations.
    pub fn model_sets(&self) -> Vec<usize> {
        let mut v: Vec<usize> = self.anims.iter().map(|a| a.model_set).chain(self.mats.anims.iter().map(|a| a.model_set)).collect();
        v.sort_unstable();
        v.dedup();
        v
    }
}

pub(crate) fn srt_transform((s, r, t): Srt) -> Transform {
    Transform {
        translation: Vec3::from_array(t),
        rotation: Quat::from_euler(EulerRot::ZYX, r[2], r[1], r[0]),
        scale: Vec3::from_array(s),
    }
}

fn to_bevy(m: &hsd::Mat4) -> Mat4 { Mat4::from_cols_array_2d(m).transpose() }

/// Spawn `model` as an entity tree. Returns the root entity and an (idle) animator for it; the caller
/// inserts the animator once it has chosen what to play.
pub fn spawn_model(commands: &mut Commands, assets: &mut ModelAssets, model: &hsd::Model, anims: Arc<Vec<Animation>>) -> (Entity, Animator) {
    let root = commands.spawn((Transform::default(), Visibility::default())).id();
    let rest: Vec<Srt> = model.joints.iter().map(|j| (j.rest_scale, j.rest_rotation, j.rest_translation)).collect();
    let bind = hsd::world_matrices(model, &rest);

    // joints, parent before child
    let mut joints: Vec<Entity> = Vec::with_capacity(model.joints.len());
    for (i, j) in model.joints.iter().enumerate() {
        let e = commands.spawn((srt_transform(rest[i]), Visibility::default())).id();
        let parent = j.parent.filter(|&p| p < joints.len()).map(|p| joints[p]).unwrap_or(root);
        commands.entity(parent).add_child(e);
        joints.push(e);
    }

    // joints any animation moves (directly or through an ancestor)
    let mut animated = vec![false; model.joints.len()];
    for a in anims.iter() { for t in &a.tracks { if t.joint < animated.len() { animated[t.joint] = true; } } }
    for i in 0..model.joints.len() {
        if let Some(p) = model.joints[i].parent { if p < i && animated[p] { animated[i] = true; } }
    }
    let inv_bind: Vec<Mat4> = bind.iter().map(|m| to_bevy(m).inverse()).collect();

    let tex_handles: Vec<Handle<Image>> = model.textures.iter().map(|t| {
        let mut img = Image::new(
            Extent3d { width: t.width as u32, height: t.height as u32, depth_or_array_layers: 1 },
            TextureDimension::D2, t.rgba.clone(), TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD,
        );
        img.sampler = sampler(Wrap::Repeat, Wrap::Repeat);
        assets.images.add(img)
    }).collect();
    let mut sampler_variants: HashMap<(usize, [u8; 2]), Handle<Image>> = HashMap::new();
    let mut targets: Vec<MeshTarget> = vec![];
    let mut tobj_wrap: HashMap<usize, [Wrap; 2]> = HashMap::new();
    let mut programs: HashMap<usize, (Arc<hsd::Material>, Arc<Program>)> = HashMap::new();

    for m in model.meshes.iter() {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, m.positions.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, m.normals.clone());
        tevmat::insert_attributes(&mut mesh, m);
        let mat = &m.material;

        // skin only if something moves this mesh's joints; per-mesh joint palette (<= 256 joints)
        let skinned = m.skin_joints.len() == m.positions.len()
            && m.skin_joints.iter().zip(&m.skin_weights).any(|(js, ws)| (0..4).any(|k| ws[k] > 0.0 && animated.get(js[k] as usize).copied().unwrap_or(false)));
        let mut skin = None;
        if skinned {
            let mut palette: Vec<usize> = vec![];
            let mut local: HashMap<usize, u16> = HashMap::new();
            let mut idx: Vec<[u16; 4]> = Vec::with_capacity(m.skin_joints.len());
            for (js, ws) in m.skin_joints.iter().zip(&m.skin_weights) {
                let mut o = [0u16; 4];
                for k in 0..4 {
                    if ws[k] <= 0.0 { continue; }
                    let j = js[k] as usize;
                    o[k] = *local.entry(j).or_insert_with(|| { palette.push(j); (palette.len() - 1) as u16 });
                }
                idx.push(o);
            }
            if palette.len() <= 256 && palette.iter().all(|&j| j < joints.len()) {
                mesh.insert_attribute(Mesh::ATTRIBUTE_JOINT_INDEX, bevy::mesh::VertexAttributeValues::Uint16x4(idx));
                mesh.insert_attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT, m.skin_weights.clone());
                let ib = assets.bindposes.add(SkinnedMeshInverseBindposes::from(palette.iter().map(|&j| inv_bind[j]).collect::<Vec<_>>()));
                skin = Some(SkinnedMesh { inverse_bindposes: ib, joints: palette.iter().map(|&j| joints[j]).collect() });
            }
        }

        let (hmat, program) = programs.entry(mat.mobj).or_insert_with(|| (Arc::new(mat.clone()), Arc::new(tev::build(mat)))).clone();
        let base_textures: Vec<Option<Handle<Image>>> = mat.layers.iter().map(|l| l.texture.map(|ti| {
            let key = (ti, [wrap_id(l.wrap[0]), wrap_id(l.wrap[1])]);
            if key.1 == [1, 1] { return tex_handles[ti].clone(); }
            sampler_variants.entry(key).or_insert_with(|| {
                let mut img = assets.images.get(&tex_handles[ti]).unwrap().clone();
                img.sampler = sampler(l.wrap[0], l.wrap[1]);
                assets.images.add(img)
            }).clone()
        })).collect();
        let mut uniform = tevmat::program_uniform(&program, mat);
        for (i, l) in mat.layers.iter().enumerate() { tevmat::set_layer(&mut uniform, i, l, &l.srt); }
        let base_uniform = uniform;
        tevmat::set_consts(&mut uniform, &program, &Values::of(mat));
        let (key, phase) = tevmat::pipeline(mat, m.cull_back, m.cull_front);
        let mut material = TevMaterial { tev: uniform, t0: None, t1: None, t2: None, t3: None, t4: None, t5: None, t6: None, t7: None, key, phase };
        for (i, t) in base_textures.iter().enumerate() { if let Some(slot) = material.texture_mut(i) { *slot = t.clone(); } }
        let mh = assets.materials.add(material);
        // JOBJ_HIDDEN meshes are spawned hidden: node/branch animation tracks can show them
        let mut ec = commands.spawn((Mesh3d(assets.meshes.add(mesh)), MeshMaterial3d(mh.clone()), Transform::default(),
            if m.hidden { Visibility::Hidden } else { Visibility::Inherited }));
        if let Some(s) = skin { ec.insert((s, NoFrustumCulling)); }
        let child = ec.id();
        for l in &mat.layers { tobj_wrap.insert(l.tobj, l.wrap); }
        targets.push(MeshTarget {
            entity: child, material: mh, set: model.set_of_joint(m.joint), joint: m.joint, mobj: mat.mobj, mat: hmat, program, base_textures, base_uniform,
        });
        commands.entity(root).add_child(child);
    }

    let ik = model.joints.iter().any(|j| j.ik_type() == 1 || (j.ik_type() == 0 && j.robj(hsd::REFTYPE_JOBJ, 1).is_some())).then(|| Arc::new(hsd::Model {
        joints: model.joints.clone(), joint_index: model.joint_index.clone(), ..Default::default()
    }));
    // animated-texture image tables, with the sampler of the TObj they animate
    let mats = Arc::clone(&model.mats);
    let mut mat_images: HashMap<(usize, usize), Handle<Image>> = HashMap::new();
    for a in &mats.anims {
        for t in &a.tex {
            let wrap = tobj_wrap.get(&t.tobj).copied().unwrap_or([Wrap::Repeat, Wrap::Repeat]);
            for &img in t.images.iter().flatten() {
                mat_images.entry((img, t.tobj)).or_insert_with(|| {
                    let tex = &mats.textures[img];
                    let mut im = Image::new(
                        Extent3d { width: tex.width as u32, height: tex.height as u32, depth_or_array_layers: 1 },
                        TextureDimension::D2, tex.rgba.clone(), TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD,
                    );
                    im.sampler = sampler(wrap[0], wrap[1]);
                    assets.images.add(im)
                });
            }
        }
    }
    let n = targets.len();
    let joint_hidden: Vec<bool> = model.joints.iter().map(|j| j.flags & hsd::JOBJ_HIDDEN != 0).collect();
    let mut children = vec![vec![]; model.joints.len()];
    for (i, j) in model.joints.iter().enumerate() { if let Some(p) = j.parent { if p < children.len() { children[p].push(i); } } }
    let vis_tracks = anims.iter().any(|a| a.tracks.iter().any(|t| t.channel == 11 || t.channel == 12));
    let vis_last = targets.iter().map(|t| !joint_hidden.get(t.joint).copied().unwrap_or(false)).collect();
    (root, Animator { anims, active: vec![], joints, rest, is_player: false, part_rot: None, ik, touched: vec![], targets, mats, tex_plays: vec![], linked: true,
        mat_images, hidden_sets: vec![], tex_override: vec![], last: vec![None; n], vis_applied: None, joint_hidden, children, vis_tracks, vis_last })
}

fn sampler(u: Wrap, v: Wrap) -> ImageSampler {
    ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: addr(u), address_mode_v: addr(v),
        mag_filter: ImageFilterMode::Linear, min_filter: ImageFilterMode::Linear, ..default()
    })
}
fn wrap_id(w: Wrap) -> u8 { match w { Wrap::Clamp => 0, Wrap::Repeat => 1, Wrap::Mirror => 2 } }
fn addr(w: Wrap) -> ImageAddressMode { match w { Wrap::Clamp => ImageAddressMode::ClampToEdge, Wrap::Repeat => ImageAddressMode::Repeat, Wrap::Mirror => ImageAddressMode::MirrorRepeat } }

/// Advance every animator at the game's 60 fps and write the sampled joint transforms.
pub fn animate(time: Res<Time>, mut animators: Query<&mut Animator>, mut joints: Query<&mut Transform, Without<Animator>>) {
    let dt = time.delta_secs().min(0.1) * 60.0;
    for mut a in &mut animators {
        if a.active.is_empty() && a.touched.is_empty() && a.part_rot.is_none() { continue; }
        let a = &mut *a;
        // advance
        for p in a.active.iter_mut() {
            let end = a.anims[p.anim].end_frame;
            p.frame += dt * p.rate;
            if p.looping { if end > 0.0 { p.frame %= end; } } else { p.frame = p.frame.min((end - 1.0).max(0.0)); }
        }
        // sample every play into per-joint SRT
        let mut locals: HashMap<usize, Srt> = HashMap::new();
        let sample = |anim: &Animation, frame: f32, rest: &[Srt], out: &mut HashMap<usize, Srt>| {
            for t in &anim.tracks {
                let (Some(v), Some(&r)) = (t.sample(frame), rest.get(t.joint)) else { continue };
                let e = out.entry(t.joint).or_insert(r);
                match t.channel {
                    1..=3 => e.1[(t.channel - 1) as usize] = v,
                    5..=7 => e.2[(t.channel - 5) as usize] = v,
                    8..=10 => e.0[(t.channel - 8) as usize] = v,
                    _ => {}
                }
            }
        };
        for p in &a.active {
            let anim = &a.anims[p.anim];
            match p.blend {
                Some((b, w)) if w > 0.0 => {
                    let mut la = HashMap::new();
                    let mut lb = HashMap::new();
                    sample(anim, p.frame, &a.rest, &mut la);
                    let (ea, eb) = (anim.end_frame, a.anims[b].end_frame);
                    let fb = if ea > 0.0 { p.frame / ea * eb } else { 0.0 };
                    sample(&a.anims[b], fb, &a.rest, &mut lb);
                    let w = w.clamp(0.0, 1.0);
                    let lerp = |x: f32, y: f32| x + (y - x) * w;
                    // rotations take the short way round
                    let lerp_ang = |x: f32, y: f32| { let mut d = (y - x) % std::f32::consts::TAU; if d > std::f32::consts::PI { d -= std::f32::consts::TAU; } else if d < -std::f32::consts::PI { d += std::f32::consts::TAU; } x + d * w };
                    let joints: std::collections::BTreeSet<usize> = la.keys().chain(lb.keys()).copied().collect();
                    for j in joints {
                        let Some(&r) = a.rest.get(j) else { continue };
                        let (x, y) = (la.get(&j).copied().unwrap_or(r), lb.get(&j).copied().unwrap_or(r));
                        let mut o = x;
                        for k in 0..3 { o.0[k] = lerp(x.0[k], y.0[k]); o.1[k] = lerp_ang(x.1[k], y.1[k]); o.2[k] = lerp(x.2[k], y.2[k]); }
                        locals.insert(j, o);
                    }
                }
                _ => sample(anim, p.frame, &a.rest, &mut locals),
            }
        }
        if let Some((j, v)) = a.part_rot {
            if let Some(&r) = a.rest.get(j) {
                let e = locals.entry(j).or_insert(r);
                e.1[0] += v[0];
                e.1[1] = v[1];
                e.1[2] += v[2];
            }
        }
        if let Some(model) = a.ik.clone() {
            // full solve: rest + tracks, then IK/constraints, then back to parent-relative transforms
            let mut all = a.rest.clone();
            for (&j, &v) in &locals { all[j] = v; }
            let worlds: Vec<Mat4> = hsd::world_matrices(&model, &all).iter().map(to_bevy).collect();
            for (i, j) in model.joints.iter().enumerate() {
                let local = match j.parent { Some(p) => worlds[p].inverse() * worlds[i], None => worlds[i] };
                if let Ok(mut tr) = joints.get_mut(a.joints[i]) { *tr = Transform::from_matrix(local); }
            }
            continue;
        }
        for &j in &a.touched {
            if !locals.contains_key(&j) { if let Ok(mut tr) = joints.get_mut(a.joints[j]) { *tr = srt_transform(a.rest[j]); } }
        }
        a.touched = locals.keys().copied().collect();
        for (j, srt) in locals {
            if let Ok(mut tr) = joints.get_mut(a.joints[j]) { *tr = srt_transform(srt); }
        }
    }
}

/// Per-TObj animated values (`TObjUpdateFunc` 0x801BE85C).
#[derive(Clone, Default)]
struct TexAnimState { image: Option<usize>, srt: Option<hsd::TexSrt>, blending: Option<f32>, tev: [Option<u8>; 12] }

/// Material and texture animations (`MObjUpdateFunc` / `TObjUpdateFunc`), per-set visibility and texture
/// changes. Runs after [`animate`]; writes a material only when its state changed.
pub fn animate_materials(time: Res<Time>, mut animators: Query<&mut Animator>, mut materials: ResMut<Assets<TevMaterial>>,
                         mut vis: Query<&mut Visibility, Without<Animator>>) {
    let dt = time.delta_secs().min(0.1) * 60.0;
    for mut a in &mut animators {
        let a = &mut *a;
        // visibility: model set (GSmodelSetVisibility) and joint (JOBJ_HIDDEN, HSD_A_J_NODE 11 / BRANCH 12:
        // value > 0.5 shows, else hides the joint / the joint and its descendants; JObjUpdateFunc)
        if a.vis_tracks || a.vis_applied.as_ref() != Some(&a.hidden_sets) {
            let mut hidden = a.joint_hidden.clone();
            if a.vis_tracks {
                for p in &a.active {
                    for t in &a.anims[p.anim].tracks {
                        if t.channel != 11 && t.channel != 12 { continue; }
                        let Some(v) = t.sample(p.frame) else { continue };
                        let h = v <= 0.5;
                        if t.channel == 11 { if let Some(x) = hidden.get_mut(t.joint) { *x = h; } continue; }
                        let mut stack = vec![t.joint];
                        while let Some(j) = stack.pop() {
                            if let Some(x) = hidden.get_mut(j) { *x = h; }
                            if let Some(c) = a.children.get(j) { stack.extend_from_slice(c); }
                        }
                    }
                }
            }
            for (i, t) in a.targets.iter().enumerate() {
                let show = !a.hidden_sets.contains(&t.set) && !hidden.get(t.joint).copied().unwrap_or(false);
                if a.vis_applied.is_some() && a.vis_last[i] == show { continue; }
                a.vis_last[i] = show;
                if let Ok(mut v) = vis.get_mut(t.entity) {
                    let want = if show { Visibility::Inherited } else { Visibility::Hidden };
                    if *v != want { *v = want; }
                }
            }
            a.vis_applied = Some(a.hidden_sets.clone());
        }
        if a.tex_plays.is_empty() && a.tex_override.is_empty() && a.last.iter().all(Option::is_none) { continue; }
        let mats = Arc::clone(&a.mats);
        // advance
        for p in a.tex_plays.iter_mut() {
            let end = mats.anims[p.anim].end_frame;
            p.frame += dt * p.rate;
            if p.looping { if end > 0.0 { p.frame %= end; } } else { p.frame = p.frame.min((end - 1.0).max(0.0)); }
        }
        // sample: TObj -> image, SRT, blending, KONST/TEV0/TEV1; MObj -> diffuse, specular, alpha
        let byte = |v: f32| (255.0 * v.clamp(0.0, 1.0)) as u8;
        let mut tex_state: HashMap<usize, TexAnimState> = HashMap::new();
        let mut mat_state: HashMap<usize, ([Option<u8>; 9], Option<f32>)> = HashMap::new();
        let base_srt: HashMap<usize, hsd::TexSrt> = a.targets.iter().flat_map(|t| t.mat.layers.iter().map(|l| (l.tobj, l.srt))).collect();
        for p in &a.tex_plays {
            let anim = &mats.anims[p.anim];
            for t in &anim.tex {
                let f = if p.looping && t.end_frame > 0.0 { p.frame % t.end_frame } else { p.frame.min(t.end_frame) };
                let mut st = TexAnimState::default();
                let mut srt = base_srt.get(&t.tobj).copied().unwrap_or_default();
                let mut srt_anim = false;
                for tr in &t.tracks {
                    let Some(v) = tr.sample(f) else { continue };
                    match tr.channel {
                        1 => st.image = t.images.get(v as i32 as usize).copied().flatten(),
                        2 => { srt.translate[0] = v; srt_anim = true; }
                        3 => { srt.translate[1] = v; srt_anim = true; }
                        4 => { srt.scale[0] = v; srt_anim = true; }
                        5 => { srt.scale[1] = v; srt_anim = true; }
                        6..=8 => { srt.rotate[(tr.channel - 6) as usize] = v; srt_anim = true; }
                        9 | 24 => st.blending = Some(v.clamp(0.0, 1.0)),
                        12..=23 => st.tev[(tr.channel - 12) as usize] = Some(byte(v)),
                        _ => {} // TCLT (palette swaps), LOD bias
                    }
                }
                st.srt = srt_anim.then_some(srt);
                tex_state.insert(t.tobj, st);
            }
            for m in &anim.mat {
                let f = if p.looping && m.end_frame > 0.0 { p.frame % m.end_frame } else { p.frame.min(m.end_frame) };
                let e = mat_state.entry(m.mobj).or_insert(([None; 9], None));
                for tr in &m.tracks {
                    let Some(v) = tr.sample(f) else { continue };
                    match tr.channel {
                        // HSD_A_M_AMBIENT_R..B 1..3, DIFFUSE 4..6, SPECULAR 7..9
                        1..=9 => e.0[(tr.channel - 1) as usize] = Some(byte(v)),
                        10 => e.1 = Some((1.0 - v).clamp(0.0, 1.0)),
                        _ => {}
                    }
                }
            }
        }
        for (i, t) in a.targets.iter().enumerate() {
            let mut values = Values::of(&t.mat);
            let mut ambient = t.mat.ambient.map(|x| (x * 255.0).round().clamp(0.0, 255.0) as u8);
            if let Some((c, alpha)) = mat_state.get(&t.mobj) {
                for k in 0..3 {
                    if let Some(x) = c[k] { ambient[k] = x; }
                    if let Some(x) = c[k + 3] { values.diffuse[k] = x; }
                    if let Some(x) = c[k + 6] { values.specular[k] = x; }
                }
                if let Some(al) = alpha { values.alpha = *al; }
            }
            let over = a.tex_override.iter().find(|(s, _)| *s == t.set).map(|(_, h)| h.clone());
            let mut textures = t.base_textures.clone();
            let mut mtx = vec![];
            for (li, l) in t.mat.layers.iter().enumerate() {
                let st = tex_state.get(&l.tobj);
                if let Some(img) = st.and_then(|s| s.image).and_then(|k| a.mat_images.get(&(k, l.tobj)).cloned()) { textures[li] = Some(img); }
                // GSmodelSetTextureChange: the first TObj's image
                if li == 0 { if let Some(h) = &over { textures[0] = Some(h.clone()); } }
                mtx.push(st.and_then(|s| s.srt).unwrap_or(l.srt));
                if let Some(s) = st {
                    if let Some(b) = s.blending { values.blending[li] = b; }
                    if let Some(tv) = values.tev[li].as_mut() {
                        for (k, x) in s.tev.iter().enumerate() {
                            let Some(x) = *x else { continue };
                            match k { 0..=3 => tv.konst[k] = x, 4..=7 => tv.tev0[k - 4] = x, _ => tv.tev1[k - 8] = x }
                        }
                    }
                }
            }
            let state = MatState {
                consts: t.program.consts.iter().map(|c| c.value(&values)).collect(), ambient,
                mtx: mtx.iter().map(|s| s.matrix()).collect(),
                textures: textures.iter().map(|h| h.as_ref().map(|h| h.id())).collect(),
            };
            if a.last[i].as_ref() == Some(&state) { continue; }
            if let Some(mut m) = materials.get_mut(&t.material) {
                let mut u = t.base_uniform;
                // the room lights stay as apply_room_lights set them
                u.lights = m.tev.lights;
                u.light_info = m.tev.light_info;
                tevmat::set_mat_ambient(&mut u, ambient);
                tevmat::set_consts(&mut u, &t.program, &values);
                for (li, l) in t.mat.layers.iter().enumerate() { tevmat::set_layer(&mut u, li, l, &mtx[li]); }
                m.tev = u;
                for (li, h) in textures.into_iter().enumerate() { if let Some(slot) = m.texture_mut(li) { *slot = h; } }
            }
            a.last[i] = Some(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colo_formats::fsys::{FileType, Fsys};

    /// Joint world matrices built the way the viewer builds them (Transform hierarchy) must match
    /// `hsd::world_matrices` for an animated frame.
    #[test]
    fn transform_hierarchy_matches_hsd() {
        let Ok(bytes) = std::fs::read("/tmp/discfiles/field_common.fsys") else { return };
        let fs = Fsys::parse(&bytes).unwrap();
        let b = fs.read(fs.find("ken_b1", FileType::Dat).unwrap()).unwrap();
        let arc = hsd::Archive::parse(&b).unwrap();
        let model = hsd::load_model(&arc).unwrap();
        let anims = colo_formats::hsdanim::load_animations(&arc, &model.joint_index).unwrap();
        let locals = anims[3].locals(&model, 8.0);
        let want = hsd::world_matrices(&model, &locals);
        let mut got: Vec<Mat4> = vec![];
        let mut worst = 0.0f32;
        for (i, j) in model.joints.iter().enumerate() {
            let l = srt_transform(locals[i]).to_matrix();
            let w = match j.parent { Some(p) => got[p] * l, None => l };
            got.push(w);
            let d = (w - to_bevy(&want[i])).to_cols_array().iter().fold(0.0f32, |a, x| a.max(x.abs()));
            worst = worst.max(d);
        }
        println!("joints {} worst diff {worst}", model.joints.len());
        // skinning check on one vertex per mesh
        let rest: Vec<Srt> = model.joints.iter().map(|j| (j.rest_scale, j.rest_rotation, j.rest_translation)).collect();
        let bind = hsd::world_matrices(&model, &rest);
        let posed = hsd::pose(&model, &want);
        let mut wv = 0.0f32;
        for (m, pm) in model.meshes.iter().zip(&posed.meshes) {
            for (k, p) in m.positions.iter().enumerate().step_by(7) {
                let mut acc = Vec3::ZERO;
                for c in 0..4 {
                    let w = m.skin_weights[k][c];
                    if w <= 0.0 { continue; }
                    let j = m.skin_joints[k][c] as usize;
                    acc += w * (got[j] * to_bevy(&bind[j]).inverse()).transform_point3(Vec3::from_array(*p));
                }
                wv = wv.max(acc.distance(Vec3::from_array(pm.positions[k])));
            }
        }
        println!("worst skinned vertex diff {wv}");
        // how different is the pose from bind?
        let mv = model.meshes.iter().zip(&posed.meshes).flat_map(|(a, b)| a.positions.iter().zip(&b.positions)).map(|(a, b)| Vec3::from_array(*a).distance(Vec3::from_array(*b))).fold(0.0f32, f32::max);
        println!("max vertex displacement from bind {mv}");
        // rest vs file world
        let rw = model.joints.iter().zip(&bind).map(|(j, b)| j.world.iter().flatten().zip(b.iter().flatten()).map(|(x, y)| (x - y).abs()).fold(0.0f32, f32::max)).fold(0.0f32, f32::max);
        println!("rest-from-SRT vs joint.world worst {rw}");
    }
}

#[cfg(test)]
mod dump {
    use super::*;
    use colo_formats::fsys::{FileType, Fsys};
    #[test]
    #[ignore = "debug dump"]
    fn dump_tracks() {
        let Ok(bytes) = std::fs::read("/tmp/discfiles/field_common.fsys") else { return };
        let fs = Fsys::parse(&bytes).unwrap();
        let b = fs.read(fs.find("ken_b1", FileType::Dat).unwrap()).unwrap();
        let arc = hsd::Archive::parse(&b).unwrap();
        let model = hsd::load_model(&arc).unwrap();
        let anims = colo_formats::hsdanim::load_animations(&arc, &model.joint_index).unwrap();
        let a = &anims[3];
        for (i, j) in model.joints.iter().enumerate() {
            let tr: Vec<String> = a.tracks.iter().filter(|t| t.joint == i).map(|t| {
                let vals: Vec<f32> = t.keys.iter().map(|k| k.value).collect();
                let (lo, hi) = vals.iter().fold((f32::MAX, f32::MIN), |(l, h), v| (l.min(*v), h.max(*v)));
                format!("c{}:{}k[{lo:.2}..{hi:.2}]", t.channel, t.keys.len())
            }).collect();
            let meshes = model.meshes.iter().filter(|m| m.skin_joints.iter().zip(&m.skin_weights).any(|(js, ws)| (0..4).any(|k| ws[k] > 0.0 && js[k] as usize == i))).count();
            println!("{i:2} off {:#07x} parent {:?} flags {:#x} meshes {meshes} t {:?} robj {:?} | {}", j.offset, j.parent, j.flags, j.rest_translation, j.robj, tr.len());
        }
    }
}

#[cfg(test)]
mod ik_check {
    use super::*;
    use colo_formats::fsys::{FileType, Fsys};
    #[test]
    fn decompose_roundtrip() {
        let Ok(bytes) = std::fs::read("/tmp/discfiles/field_common.fsys") else { return };
        let fs = Fsys::parse(&bytes).unwrap();
        let b = fs.read(fs.find("ken_b1", FileType::Dat).unwrap()).unwrap();
        let arc = hsd::Archive::parse(&b).unwrap();
        let model = hsd::load_model(&arc).unwrap();
        let anims = colo_formats::hsdanim::load_animations(&arc, &model.joint_index).unwrap();
        for (ai, fr) in [(3usize, 5.0f32), (2, 0.0), (2, 10.0)] {
            let locals = anims[ai].locals(&model, fr);
            let worlds: Vec<Mat4> = hsd::world_matrices(&model, &locals).iter().map(to_bevy).collect();
            let mut got: Vec<Mat4> = vec![];
            for (i, j) in model.joints.iter().enumerate() {
                let local = match j.parent { Some(p) => worlds[p].inverse() * worlds[i], None => worlds[i] };
                let t = Transform::from_matrix(local);
                let w = match j.parent { Some(p) => got[p] * t.to_matrix(), None => t.to_matrix() };
                let d = (w - worlds[i]).to_cols_array().iter().fold(0.0f32, |a, x| a.max(x.abs()));
                if d > 1e-3 || !d.is_finite() { println!("anim {ai} f{fr} joint {i}: diff {d} scale {:?} det {}", t.scale, worlds[i].determinant()); }
                got.push(w);
            }
        }
    }
}

#[cfg(test)]
mod robj_scan {
    use super::*;
    use colo_formats::fsys::{FileType, Fsys};
    /// Every non-IK RObj on character models in the test archives.
    #[test]
    #[ignore = "debug dump"]
    fn scan_robjs() {
        let mut counts: std::collections::BTreeMap<String, usize> = Default::default();
        for f in std::fs::read_dir("/tmp/discfiles").into_iter().flatten().flatten() {
            let Ok(bytes) = std::fs::read(f.path()) else { continue };
            let Ok(fs) = Fsys::parse(&bytes) else { continue };
            for e in fs.entries.iter().filter(|e| e.file_type == FileType::Dat) {
                let Ok(b) = fs.read(e) else { continue };
                let Ok(arc) = hsd::Archive::parse(&b) else { continue };
                let Ok(model) = hsd::load_model(&arc) else { continue };
                for (i, j) in model.joints.iter().enumerate() {
                    for r in &j.robj {
                        let key = format!("type {:#x} sub {} ik_type {}", r.flags & 0x7000_0000, r.flags & 0x0FFF_FFFF, j.ik_type());
                        *counts.entry(key.clone()).or_default() += 1;
                        if j.ik_type() == 0 { println!("{} joint {i} {key} {:?}", e.name, r.kind); }
                    }
                }
            }
        }
        for (k, v) in counts { println!("{v:5}  {k}"); }
    }
}

#[cfg(test)]
mod sets_dump {
    use super::*;
    use colo_formats::fsys::{FileType, Fsys};
    #[test]
    #[ignore = "debug dump"]
    fn dump_map_sets() {
        let room = std::env::var("ROOM").unwrap_or("M2_shop_1F".into());
        let Ok(bytes) = std::fs::read(format!("/tmp/discfiles/{room}.fsys")) else { return };
        let fs = Fsys::parse(&bytes).unwrap();
        let b = fs.read(fs.find_type(FileType::Rdat).next().unwrap()).unwrap();
        let arc = hsd::Archive::parse(&b).unwrap();
        let model = hsd::load_model(&arc).unwrap();
        // root joint index of each joint
        let root_of = |mut i: usize| { while let Some(p) = model.joints[i].parent { i = p; } i };
        let mut sets: std::collections::BTreeMap<usize, (usize, [f32; 3], [f32; 3], u32)> = Default::default();
        for m in &model.meshes {
            let r = root_of(m.joint);
            let e = sets.entry(r).or_insert((0, [f32::MAX; 3], [f32::MIN; 3], model.joints[r].flags));
            e.0 += 1;
            for p in &m.positions { for k in 0..3 { e.1[k] = e.1[k].min(p[k]); e.2[k] = e.2[k].max(p[k]); } }
        }
        for (r, (n, lo, hi, fl)) in sets { println!("root joint {r} flags {fl:#x}: {n} meshes  {lo:?}..{hi:?}"); }
        for (i, m) in model.meshes.iter().enumerate() {
            let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
            for p in &m.positions { for k in 0..3 { lo[k] = lo[k].min(p[k]); hi[k] = hi[k].max(p[k]); } }
            println!("mesh {i} joint {} ({:#x}) hidden {} cb {} cf {} tris {} {lo:.0?}..{hi:.0?}", m.joint, model.joints[m.joint].flags, m.hidden, m.cull_back, m.cull_front, m.positions.len() / 3);
        }
    }
}
