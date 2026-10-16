//! Draws the game's fade layer (`fade.rs`): the plain fade quad and the battle wipes.
//!
//! The game draws the fade quads over the finished frame and, while a UDLR/trainer wipe runs, copies the
//! frame back into the fade texture every frame (`GSgfxBackFBDoFrame`), so each frame's wipe samples the
//! previous frame including the previous wipe. Here, while the fade layer is up:
//! - a copy of the game camera renders the 3D frame into `scene` (640x480);
//! - once per game frame, a composite pass (`wipe.wgsl`) draws `scene` plus the quads into one of two
//!   640x480 images (gamma-space values, u8 alpha), sampling the other image as the fade texture; the images swap after each composite;
//! - a display pass shows the newest image in the game's 4:3 frame on top of the window.
//! `_fadeSnapshot` (the first frame) takes `scene` as the fade texture.

use crate::fade::{Quad, Src};
use crate::script::ScriptRt;
use crate::GameCam;
use bevy::asset::{embedded_asset, RenderAssetUsages};
use bevy::camera::visibility::RenderLayers;
use bevy::camera::{ClearColorConfig, RenderTarget, ScalingMode};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::image::{ImageSampler, ImageSamplerDescriptor};
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, Extent3d, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError, TextureDimension, TextureFormat};
use bevy::shader::ShaderRef;

/// Render layers of the composite and display passes.
const LAYER_COMPOSITE: usize = 27;
const LAYER_DISPLAY: usize = 28;
const MAX_QUADS: usize = 32;

#[derive(Clone, Copy, Debug, ShaderType, Default)]
pub struct QuadU { a: Vec4, b: Vec4, c: Vec4, col: Vec4, src: UVec4 }

#[derive(Clone, Copy, Debug, ShaderType)]
pub struct WipeU { quads: [QuadU; MAX_QUADS], n: UVec4 }

impl Default for WipeU { fn default() -> Self { WipeU { quads: [QuadU::default(); MAX_QUADS], n: UVec4::ZERO } } }

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct WipeMat {
    #[uniform(0)]
    u: WipeU,
    #[texture(1)]
    scene: Handle<Image>,
    #[texture(2)]
    prev: Handle<Image>,
    #[texture(3)] #[sampler(5)]
    ball0: Handle<Image>,
    #[texture(4)]
    ball1: Handle<Image>,
}

const SHADER: &str = "embedded://colo_viewer/wipe.wgsl";

impl Material for WipeMat {
    fn fragment_shader() -> ShaderRef { SHADER.into() }
    fn enable_prepass() -> bool { false }
    fn enable_shadows() -> bool { false }
    fn specialize(_: &MaterialPipeline, desc: &mut RenderPipelineDescriptor, _: &MeshVertexBufferLayoutRef,
                  _: MaterialPipelineKey<Self>) -> Result<(), SpecializedMeshPipelineError> {
        desc.primitive.cull_mode = None;
        Ok(())
    }
}

/// The cameras, images and materials of the fade layer.
#[derive(Resource)]
pub struct Wipe {
    scene: Handle<Image>,
    efb: [Handle<Image>; 2],
    /// The image holding the fade texture (snapshot or last capture); composites write the other one.
    tex: Option<usize>,
    scene_cam: Entity,
    comp_cam: Entity,
    disp_cam: Entity,
    comp_mat: Handle<WipeMat>,
    disp_mat: Handle<WipeMat>,
    /// The fade serial last composited (one composite per game frame).
    done: u64,
    /// A composite has been shown since the layer came up.
    shown: bool,
    /// Test hook `COLO_WIPE_SNAPSHOT=file.rgba` (640x480 RGBA8, gamma values): use this image as the snapshot,
    /// e.g. a Dolphin frame, to compare the wipe against the game's trace.
    snapshot_override: Option<Handle<Image>>,
}

impl Wipe {
    /// The image holding the fade texture now (the newest composite while a capture runs).
    pub fn current(&self) -> Option<Handle<Image>> { self.tex.map(|t| self.efb[t].clone()) }
}

pub struct WipePlugin;

impl Plugin for WipePlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "wipe.wgsl");
        app.add_plugins(MaterialPlugin::<WipeMat>::default());
    }
}

fn ui_image(img: &colo_formats::gtx::Image) -> Image {
    let mut i = Image::new(Extent3d { width: img.width as u32, height: img.height as u32, depth_or_array_layers: 1 }, TextureDimension::D2,
        img.rgba.clone(), TextureFormat::Rgba8Unorm, RenderAssetUsages::RENDER_WORLD);
    i.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::linear());
    i
}

/// Spawn the (inactive) cameras and quads.
pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, mut mats: ResMut<Assets<WipeMat>>, mut meshes: ResMut<Assets<Mesh>>,
             game: Res<crate::world::Game>) {
    let target = |images: &mut Assets<Image>, f: TextureFormat| images.add(Image::new_target_texture(640, 480, f, None));
    let scene = target(&mut images, TextureFormat::Rgba8UnormSrgb);
    // COPY_SRC: gamebattle.rs reads the newest one back as the game's fade texture
    let efb = [0, 1].map(|_| {
        let mut i = Image::new_target_texture(640, 480, TextureFormat::Rgba8Unorm, None);
        i.texture_descriptor.usage |= bevy::render::render_resource::TextureUsages::COPY_SRC;
        images.add(i)
    });
    let blank = colo_formats::gtx::Image { width: 1, height: 1, rgba: vec![255; 4] };
    let ball0 = images.add(ui_image(game.battle.wipe_tex.first().unwrap_or(&blank)));
    let ball1 = images.add(ui_image(game.battle.wipe_tex.get(1).unwrap_or(&blank)));
    let mat = |prev: Handle<Image>, display: bool| WipeMat {
        u: WipeU { n: UVec4::new(0, display as u32, 0, 0), ..default() }, scene: scene.clone(), prev, ball0: ball0.clone(), ball1: ball1.clone(),
    };
    let snapshot_override = std::env::var("COLO_WIPE_SNAPSHOT").ok().and_then(|p| std::fs::read(p).ok()).filter(|b| b.len() == 640 * 480 * 4).map(|b| {
        images.add(Image::new(Extent3d { width: 640, height: 480, depth_or_array_layers: 1 }, TextureDimension::D2, b,
            TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD))
    });
    let comp_mat = mats.add(mat(efb[1].clone(), false));
    let disp_mat = mats.add(mat(efb[0].clone(), true));
    let rect = meshes.add(Rectangle::new(640.0, 480.0));
    let ortho = || Projection::Orthographic(OrthographicProjection { scaling_mode: ScalingMode::Fixed { width: 640.0, height: 480.0 }, near: -10.0, far: 10.0, ..OrthographicProjection::default_3d() });
    let scene_cam = commands.spawn((Camera3d::default(), Camera { order: -1, is_active: true, clear_color: ClearColorConfig::Custom(Color::BLACK), ..default() },
        RenderTarget::Image(scene.clone().into()), Transform::default(), Projection::custom(crate::tv::FixedAspect(PerspectiveProjection::default())), Name::new("wipe scene"))).id();
    let comp_cam = commands.spawn((Camera3d::default(), Camera { order: 4, is_active: true, clear_color: ClearColorConfig::None, ..default() },
        RenderTarget::Image(efb[0].clone().into()), Tonemapping::None, Msaa::Off, ortho(), Transform::default(),
        RenderLayers::layer(LAYER_COMPOSITE), Name::new("wipe composite"))).id();
    let disp_cam = commands.spawn((Camera3d::default(), Camera { order: 5, is_active: false, clear_color: ClearColorConfig::None, ..default() },
        Tonemapping::None, Msaa::Off, ortho(), Transform::default(), RenderLayers::layer(LAYER_DISPLAY), Name::new("wipe display"))).id();
    commands.spawn((Mesh3d(rect.clone()), MeshMaterial3d(comp_mat.clone()), Transform::from_xyz(0.0, 0.0, -1.0), RenderLayers::layer(LAYER_COMPOSITE)));
    commands.spawn((Mesh3d(rect), MeshMaterial3d(disp_mat.clone()), Transform::from_xyz(0.0, 0.0, -1.0), RenderLayers::layer(LAYER_DISPLAY)));
    commands.insert_resource(Wipe { scene, efb, tex: None, scene_cam, comp_cam, disp_cam, comp_mat, disp_mat, done: 0, shown: false, snapshot_override });
}

fn quad_u(q: &Quad) -> QuadU {
    QuadU {
        a: Vec4::new(q.p0.x, q.p0.y, q.e1.x, q.e1.y),
        b: Vec4::new(q.e2.x, q.e2.y, q.uv0.x, q.uv0.y),
        c: Vec4::new(q.du.x, q.du.y, q.dv.x, q.dv.y),
        col: Vec4::new(q.rgb[0] as f32, q.rgb[1] as f32, q.rgb[2] as f32, q.alpha as f32),
        src: UVec4::new(match q.src { Src::Capture => 0, Src::Ball0 => 1, Src::Mask => 2, Src::Solid => 3 }, 0, 0, 0),
    }
}

/// After the camera systems: run the composite once per game frame while the fade layer is up.
#[allow(clippy::type_complexity)]
pub fn update(mut wipe: ResMut<Wipe>, mut rt: ResMut<ScriptRt>, mut mats: ResMut<Assets<WipeMat>>,
              main: Query<(&Transform, &Projection, &Camera, Option<&Tonemapping>), With<GameCam>>,
              mut cams: Query<(&mut Camera, Option<&mut RenderTarget>, Option<&mut Transform>, Option<&mut Projection>, Option<&mut Tonemapping>), Without<GameCam>>) {
    let fade = &mut rt.w.fade_gs;
    let up = fade.active();
    let Ok((mt, mp, mc, mtm)) = main.single() else { return };
    let viewport = mc.viewport.clone();
    let (scene_cam, comp_cam, disp_cam) = (wipe.scene_cam, wipe.comp_cam, wipe.disp_cam);
    if let Ok((mut c, _, t, p, tm)) = cams.get_mut(scene_cam) {
        // always on (640x480 is cheap), like the composite: the snapshot is this frame's
        c.is_active = true;
        if let (Some(mut t), Some(mut p)) = (t, p) { *t = *mt; *p = mp.clone(); }
        if let (Some(mut tm), Some(m)) = (tm, mtm) { *tm = *m; }
    }
    if !up {
        (wipe.tex, wipe.shown) = (None, false);
        if let Ok((mut c, ..)) = cams.get_mut(disp_cam) { c.is_active = false; }
        return;
    }
    if !fade.texture { wipe.tex = None; }
    let run = fade.serial != wipe.done;
    // the composite camera stays on (re-running a composite with the same inputs gives the same image): a camera
    // just switched on draws nothing in its first frame, which blacked out every snapshot taken without a fade
    // shortly before (Phenac's trainer and ball wipes)
    if let Ok((mut c, ..)) = cams.get_mut(comp_cam) { c.is_active = true; }
    if run { wipe.shown = true; }
    let shown = wipe.shown;
    if let Ok((mut c, ..)) = cams.get_mut(disp_cam) { c.is_active = shown; c.viewport = viewport; }
    if !run { return; }
    wipe.done = fade.serial;
    // `_fadeSnapshot`: this frame (no quads yet) becomes the fade texture
    let snap = std::mem::take(&mut fade.snapshot);
    let write = wipe.tex.map_or(0, |t| 1 - t);
    let prev = match (snap, wipe.tex) { (false, Some(t)) => wipe.efb[t].clone(), _ => wipe.scene.clone() };
    let scene = if snap { wipe.snapshot_override.clone().unwrap_or_else(|| wipe.scene.clone()) } else { wipe.scene.clone() };
    let mut u = WipeU::default();
    let n = fade.quads.len().min(MAX_QUADS);
    for (d, q) in u.quads.iter_mut().zip(&fade.quads) { *d = quad_u(q); }
    u.n = UVec4::new(n as u32, 0, fade.texture as u32, 0);
    if let Ok((_, Some(mut target), ..)) = cams.get_mut(comp_cam) { *target = RenderTarget::Image(wipe.efb[write].clone().into()); }
    let comp = wipe.comp_mat.clone();
    if let Some(mut m) = mats.get_mut(&comp) { m.u = u; m.prev = prev; m.scene = scene; }
    let disp = wipe.disp_mat.clone();
    if let Some(mut m) = mats.get_mut(&disp) { m.prev = wipe.efb[write].clone(); }
    // the screen is copied into the fade texture after the snapshot frame and while a capture runs
    if snap || fade.capturing { wipe.tex = Some(write); }
}
