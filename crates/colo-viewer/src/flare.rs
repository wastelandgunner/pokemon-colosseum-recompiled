//! Lens flare (`flareDraw` 0x801ED780, a GSgapp draw task made by `flareCreateTask`; data from the room's
//! `.gfl`, see `colo_formats::gfl`). On only when the room's light block names a sun model (T1_ancient_colo).
//!
//! Each frame, with the active camera:
//! 1. `GScolsys2Sun(sun, eye)`: a sun-occluder triangle (ccd +0x38) between them hides everything.
//! 2. `fn_800D2F34`: the sun in view space; in front of the near plane (z < proj[2][3]) it is projected
//!    (`GXProject`) to 640×480 screen px ("front"), otherwise it isn't projected and the previous position
//!    is used ("behind"; the original reads stale stack, we keep the last projected position).
//! 3. dist = |centre − sun|; nothing is drawn unless 0.001 ≤ dist ≤ max_dist.
//! 4. Ortho 640×480, no z test/update, no culling. Front: additive (GX_BM_BLEND, ONE, ONE) textured quads
//!    with vertex colour (a, a, a), a = 255 · element fade; then, below fade_dist, an untextured full-screen
//!    glare quad at 255 · glare(+0x14). Behind: only the glare, at +0x18 and GX_BM_SUBTRACT (darkens).
//!
//! Here the quads sit on render layer 2, drawn by an orthographic overlay camera (1 unit = 1 window px)
//! after the scene. The 640×480 frame is the window's height-matched centre (the game camera's viewport).
//! Blending happens in linear space on Bevy's sRGB target (the GX adds gamma-space values).

use crate::world::RoomData;
use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::RenderLayers;
use bevy::camera::ClearColorConfig;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline};
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState, Extent3d, RenderPipelineDescriptor,
    SpecializedMeshPipelineError, TextureDimension, TextureFormat,
};
use colo_formats::gfl;

pub const FLARE_LAYER: usize = 2;
const CENTRE: Vec2 = Vec2::new(320.0, 240.0);

/// `GX_BM_SUBTRACT`: dst − src.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct Subtract {}

impl MaterialExtension for Subtract {
    fn specialize(_: &MaterialExtensionPipeline, desc: &mut RenderPipelineDescriptor, _: &MeshVertexBufferLayoutRef,
                  _: MaterialExtensionKey<Self>) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(f) = desc.fragment.as_mut() {
            for t in f.targets.iter_mut().flatten() {
                let sub = BlendComponent { src_factor: BlendFactor::One, dst_factor: BlendFactor::One, operation: BlendOperation::ReverseSubtract };
                t.blend = Some(BlendState { color: sub, alpha: BlendComponent::OVER });
            }
        }
        Ok(())
    }
}

type SubMat = ExtendedMaterial<StandardMaterial, Subtract>;

#[derive(Default)]
struct FlareState {
    room: String,
    /// (entity, material, texture half-size) per element
    elements: Vec<(Entity, Handle<StandardMaterial>, Vec2)>,
    glare_add: Option<(Entity, Handle<StandardMaterial>)>,
    glare_sub: Option<(Entity, Handle<SubMat>)>,
    last_sun: Option<Vec2>,
}

pub struct FlarePlugin;

impl Plugin for FlarePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<SubMat>::default())
            .add_systems(Startup, |mut commands: Commands| {
                commands.spawn((
                    Camera3d::default(),
                    Camera { order: 1, clear_color: ClearColorConfig::None, ..default() },
                    Projection::Orthographic(OrthographicProjection { scaling_mode: bevy::camera::ScalingMode::WindowSize, ..OrthographicProjection::default_3d() }),
                    Tonemapping::None,
                    Transform::from_xyz(0.0, 0.0, 100.0),
                    RenderLayers::layer(FLARE_LAYER),
                    Name::new("flare camera"),
                ));
            });
    }
}

fn quad_material(color: f32, tex: Option<Handle<Image>>) -> StandardMaterial {
    StandardMaterial { base_color: Color::srgb(color, color, color), base_color_texture: tex, unlit: true, alpha_mode: AlphaMode::Add, cull_mode: None, ..default() }
}

fn rebuild(commands: &mut Commands, st: &mut FlareState, room: &RoomData, meshes: &mut Assets<Mesh>, images: &mut Assets<Image>,
           mats: &mut Assets<StandardMaterial>, subs: &mut Assets<SubMat>) {
    for e in st.elements.drain(..).map(|e| e.0).chain(st.glare_add.take().map(|g| g.0)).chain(st.glare_sub.take().map(|g| g.0)) {
        commands.entity(e).despawn();
    }
    st.room = room.name.clone();
    st.last_sun = None;
    let Some((f, _)) = &room.flare else { return };
    let quad = meshes.add(Rectangle::new(1.0, 1.0));
    let spawn = |m: MeshMaterial3d<StandardMaterial>, c: &mut Commands| {
        c.spawn((Mesh3d(quad.clone()), m, Transform::default(), Visibility::Hidden, RenderLayers::layer(FLARE_LAYER), Name::new("flare quad"))).id()
    };
    // GStextureLoad into lbl_8046D630[slot]; the blend ignores texture alpha
    let tex: Vec<(u16, Handle<Image>, Vec2)> = f.textures.iter().map(|(slot, img)| {
        let mut rgba = img.rgba.clone();
        for p in rgba.chunks_mut(4) { p[3] = 255; }
        let h = images.add(Image::new(Extent3d { width: img.width as u32, height: img.height as u32, depth_or_array_layers: 1 },
            TextureDimension::D2, rgba, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD));
        (*slot, h, Vec2::new((img.width >> 1) as f32, (img.height >> 1) as f32))
    }).collect();
    for el in &f.elements {
        let Some((_, h, half)) = tex.iter().find(|t| t.0 == el.slot) else { continue };
        let m = mats.add(quad_material(1.0, Some(h.clone())));
        let e = spawn(MeshMaterial3d(m.clone()), commands);
        st.elements.push((e, m, *half));
    }
    let m = mats.add(quad_material(0.0, None));
    st.glare_add = Some((spawn(MeshMaterial3d(m.clone()), commands), m));
    let s = subs.add(SubMat { base: quad_material(0.0, None), extension: Subtract {} });
    let e = commands.spawn((Mesh3d(quad), MeshMaterial3d(s.clone()), Transform::default(), Visibility::Hidden, RenderLayers::layer(FLARE_LAYER), Name::new("flare glare (subtract)"))).id();
    st.glare_sub = Some((e, s));
}

#[allow(clippy::too_many_arguments)]
pub fn update(mut commands: Commands, room: Res<RoomData>, mut st: Local<FlareStateLocal>, mut meshes: ResMut<Assets<Mesh>>,
              mut images: ResMut<Assets<Image>>, mut mats: ResMut<Assets<StandardMaterial>>, mut subs: ResMut<Assets<SubMat>>,
              cam: Query<(&Camera, &GlobalTransform, &Projection), With<crate::GameCam>>,
              mut vis: Query<(&mut Visibility, &mut Transform)>) {
    let st = &mut st.0;
    if st.room != room.name { rebuild(&mut commands, st, &room, &mut meshes, &mut images, &mut mats, &mut subs); }
    let entities: Vec<Entity> = st.elements.iter().map(|e| e.0).chain(st.glare_add.as_ref().map(|g| g.0)).chain(st.glare_sub.as_ref().map(|g| g.0)).collect();
    for &e in &entities { if let Ok((mut v, _)) = vis.get_mut(e) { *v = Visibility::Hidden; } }
    let (Some((f, sun)), Ok((camera, cam_gt, proj))) = (&room.flare, cam.single()) else { return };
    let (sun, eye) = (*sun, cam_gt.translation());
    let blocked = room.colsys.sun_blocked(sun, eye);
    if std::env::var("COLO_FLARE_LOG").is_ok() {
        let vp = camera.world_to_viewport(cam_gt, sun).ok();
        println!("flare: eye {eye:.1} blocked {blocked} viewport {vp:?} last {:?}", st.last_sun);
    }
    if blocked { return; }
    let near = crate::tv::near(proj);
    let front = cam_gt.affine().inverse().transform_point3(sun).z < -near;
    let Some(rect) = camera.logical_viewport_rect() else { return };
    let size = rect.size();
    let s = size.y / 480.0;
    if front {
        // viewport px are window px; the 4:3 game viewport is centred in the window
        let Ok(vp) = camera.world_to_viewport(cam_gt, sun) else { return };
        st.last_sun = Some((vp - rect.center()) / s + CENTRE);
    }
    let Some(g) = st.last_sun else { return };
    let d = CENTRE - g;
    let dist = d.length();
    if dist.abs() < 0.001 || dist > f.max_dist as f32 { return; }
    // 640×480 px (y down) -> overlay units (window px from the centre, y up)
    let place = |c: Vec2, half: Vec2| Transform::from_xyz((c.x - CENTRE.x) * s, -(c.y - CENTRE.y) * s, 0.0).with_scale((half * 2.0 * s).extend(1.0));
    let full = Transform::from_scale(size.extend(1.0));
    let level = |k: f32| gfl::alpha_byte(k) as f32 / 255.0;
    if front {
        let dir = d / dist;
        let a = level(f.element_fade(dist));
        for ((e, m, half), el) in st.elements.iter().zip(&f.elements) {
            let c = if el.mode == 1 { g } else { CENTRE + dir * (0.333_333_34 * el.pos * dist) };
            if let Ok((mut v, mut t)) = vis.get_mut(*e) { *v = Visibility::Visible; *t = place(c, *half * el.scale); }
            if let Some(mut m) = mats.get_mut(m) { m.base_color = Color::srgb(a, a, a); }
        }
        if let (Some(k), Some((e, m))) = (f.glare(dist, f.glare_alpha_front), &st.glare_add) {
            let a = level(k);
            if let Ok((mut v, mut t)) = vis.get_mut(*e) { *v = Visibility::Visible; *t = full; }
            if let Some(mut m) = mats.get_mut(m) { m.base_color = Color::srgb(a, a, a); }
        }
    } else if let (Some(k), Some((e, m))) = (f.glare(dist, f.glare_alpha_behind), &st.glare_sub) {
        let a = level(k);
        if let Ok((mut v, mut t)) = vis.get_mut(*e) { *v = Visibility::Visible; *t = full; }
        if let Some(mut m) = subs.get_mut(m) { m.base.base_color = Color::srgb(a, a, a); }
    }
}

#[derive(Default)]
pub struct FlareStateLocal(FlareState);
