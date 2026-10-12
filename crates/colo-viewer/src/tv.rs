//! TV screens (room record +0x48 channel list; `FUN_80117E58` at floor setup, `mapSetTextureChange` (std 171),
//! the per-frame render `FUN_80117D14` called from `_floorUpdate`).
//!
//! A channel is a programme model (loaded with `floorOpenModel`, hidden from the room, its texture animation
//! linked to its joint animation, animation `anim` started) and a `.cam` camera. Every frame the game draws the
//! programme with that camera into a `width`x`height` texture (viewport and scissor 0..w-1 / 0..h-1, the
//! camera's own perspective and aspect, no clear) and copies it out; `GSmodelSetTextureChange` makes that
//! texture the image of the first TObj of every material of the screen's map model set (UV transforms kept).
//!
//! Here the programme sits on render layer 1 and a camera on that layer renders it into an image target.

use crate::model::{spawn_model, Animator, ModelAssets};
use crate::script::ScriptRt;
use crate::world::RoomData;
use bevy::camera::visibility::RenderLayers;
use bevy::camera::{CameraProjection, ClearColorConfig, RenderTarget, SubCameraView};
use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use std::sync::Arc;

/// Render layer of TV programmes.
pub const TV_LAYER: usize = 1;

/// A perspective with a fixed aspect (the CObj's), whatever the target's shape: the game renders the
/// programme into a square viewport with the camera's 4:3 projection.
#[derive(Debug, Clone)]
pub struct FixedAspect(pub PerspectiveProjection);

impl CameraProjection for FixedAspect {
    fn get_clip_from_view(&self) -> Mat4 { self.0.get_clip_from_view() }
    fn get_clip_from_view_for_sub(&self, sub_view: &SubCameraView) -> Mat4 { self.0.get_clip_from_view_for_sub(sub_view) }
    fn update(&mut self, _width: f32, _height: f32) {}
    fn far(&self) -> f32 { self.0.far }
    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8] { self.0.get_frustum_corners(z_near, z_far) }
}

/// The `FixedAspect` perspective inside a `Projection`, if it is one.
pub fn fixed_aspect_mut(proj: &mut Projection) -> Option<&mut FixedAspect> {
    if let Projection::Custom(c) = proj { c.get_mut::<FixedAspect>() } else { None }
}

/// Its near plane (0 for any other projection).
pub fn near(proj: &Projection) -> f32 {
    if let Projection::Custom(c) = proj { c.get::<FixedAspect>().map_or(0.0, |p| p.0.near) } else if let Projection::Perspective(p) = proj { p.near } else { 0.0 }
}

#[derive(Default)]
pub struct TvState {
    room: Option<Entity>,
    channel: Option<u32>,
    spawned: Vec<Entity>,
}

/// Follow the script world's channel: tear down the old programme, build the new one, retarget the screen.
pub fn update(mut commands: Commands, mut assets: ModelAssets, rt: Res<ScriptRt>, room: Res<RoomData>,
              roots: Query<Entity, With<crate::RoomRoot>>, mut map: Query<&mut Animator, With<crate::MapModel>>, mut st: Local<TvState>) {
    let Ok(root) = roots.single() else { return };
    if st.room != Some(root) {
        // a new room: its entities (and the old programme, a child of the old room) are gone
        *st = TvState { room: Some(root), ..Default::default() };
    }
    if st.channel == rt.w.tv && (st.channel.is_none() || !st.spawned.is_empty()) { return; }
    let Ok(mut map) = map.single_mut() else { return };
    // GSmodelResetTextureChange + free the programme
    for e in st.spawned.drain(..) { commands.entity(e).despawn(); }
    map.tex_override.clear();
    st.channel = rt.w.tv;
    let Some(id) = rt.w.tv else { return };
    let Some(ch) = room.tv.iter().find(|c| c.id == id) else { return };
    let (Some(loaded), Some(cam)) = (room.tv_models.get(&ch.model), room.cam_files.get(&ch.camera)) else {
        eprintln!("tv channel {id}: programme {:#x} or camera {:#x} missing", ch.model, ch.camera);
        return;
    };

    // GStextureCreate(w, h, ...): the render target
    let image = assets.add_image(Image::new_target_texture(ch.width as u32, ch.height as u32, TextureFormat::Rgba8UnormSrgb, None));

    // floorOpenModel + GSmodelLinkTexAnimToAnim(1) + GSmodelSetAnimIndex(anim) + GSmodelStartAnimation
    let (model, anims) = (&loaded.0, &loaded.1);
    let (r, mut anim) = spawn_model(&mut commands, &mut assets, model, Arc::clone(anims));
    anim.linked = true;
    anim.play_set((ch.model & 0xFF) as usize, ch.anim as usize, 0.0, crate::MAP_ANIM_RATE, true);
    let layer = RenderLayers::layer(TV_LAYER);
    for t in &anim.targets { commands.entity(t.entity).insert(layer.clone()); }
    commands.entity(r).insert((anim, Name::new(format!("tv programme {:#x}", ch.model))));
    commands.entity(root).add_child(r);

    // the channel's camera: CObj eye / interest / roll, perspective fov (vertical, degrees) and aspect
    let b = cam.base;
    let eye = Vec3::from_array(b.eye);
    let interest = Vec3::from_array(b.interest);
    let fwd = (interest - eye).normalize_or(Vec3::NEG_Z);
    let up = Quat::from_axis_angle(fwd, -cam.roll) * Vec3::Y;
    let proj = PerspectiveProjection {
        fov: b.fov.clamp(1.0, 170.0).to_radians(),
        aspect_ratio: if b.aspect > 0.0 { b.aspect } else { 4.0 / 3.0 },
        near: b.near.max(0.01),
        far: b.far.max(b.near + 1.0),
        ..default()
    };
    let c = commands.spawn((
        Camera3d::default(),
        Camera { order: -1, clear_color: ClearColorConfig::Custom(Color::BLACK), ..default() },
        RenderTarget::Image(image.clone().into()),
        Projection::custom(FixedAspect(proj)),
        Transform::from_translation(eye).looking_at(interest, up),
        layer,
        Name::new(format!("tv camera {:#x}", ch.camera)),
    )).id();
    commands.entity(root).add_child(c);
    st.spawned = vec![r, c];

    // GSmodelSetTextureChange(screen, texture)
    map.tex_override = vec![((ch.screen & 0xFF) as usize, image)];
}
