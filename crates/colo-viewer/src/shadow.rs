//! HSD projected shadows (`modelShadowRender` 0x800E8684 → sysdolphin `HSD_Shadow`): a model with shadow
//! flags is drawn from its shadow light (`GSmodelSetShadowLight`) into a texture (`GSmodelSetShadowTextureSize`,
//! default 384x384) cleared to white, in the flat colour 255 − intensity (intensity 0x80 → 0x7F), and every
//! receiver model (`GSmodelSetShadowSurface`) multiplies its materials by that texture projected from the light
//! (`HSD_MObjAddShadowTexture`). The light camera looks at the casters' centre from 30 units along the light
//! (`HSD_ShadowSetEye`), near 0.1 far 3000; an infinite light gives an orthographic camera, point/spot lights a
//! frustum; the view rectangle fits the receivers' bounds, else ± the caster size.
//!
//! Here: the casters' meshes are duplicated on render layer 1 with a flat [`ShadowMat`], a camera on that layer
//! renders them into the texture, and the receivers' meshes are duplicated with a multiplying [`ShadowMat`].

use bevy::asset::embedded_asset;
use bevy::camera::visibility::RenderLayers;
use bevy::camera::{ClearColorConfig, RenderTarget, ScalingMode};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::mesh::skinning::SkinnedMesh;
use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, CompareFunction, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError, TextureFormat};
use bevy::shader::ShaderRef;

/// Render layer of the shadow casters' copies.
pub const LAYER: usize = 1;
/// `HSD_ShadowSetEye` distance and the light camera's clip planes.
const EYE_DISTANCE: f32 = 30.0;
const NEAR: f32 = 0.1;
const FAR: f32 = 3000.0;

#[derive(Clone, Copy, Debug, ShaderType, Default)]
pub struct ShadowUniform {
    pub light_clip: Mat4,
    pub color: Vec4,
    pub mode: UVec4,
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct ShadowMat {
    #[uniform(0)]
    pub u: ShadowUniform,
    #[texture(1)] #[sampler(2)]
    pub tex: Option<Handle<Image>>,
}

const SHADER: &str = "embedded://colo_viewer/shadow.wgsl";

impl Material for ShadowMat {
    fn vertex_shader() -> ShaderRef { SHADER.into() }
    fn fragment_shader() -> ShaderRef { SHADER.into() }
    fn alpha_mode(&self) -> AlphaMode { if self.u.mode.x == 1 { AlphaMode::Multiply } else { AlphaMode::Opaque } }
    fn enable_prepass() -> bool { false }
    fn enable_shadows() -> bool { false }
    fn specialize(_: &MaterialPipeline, desc: &mut RenderPipelineDescriptor, layout: &MeshVertexBufferLayoutRef,
                  _key: MaterialPipelineKey<Self>) -> Result<(), SpecializedMeshPipelineError> {
        let mut attrs = vec![Mesh::ATTRIBUTE_POSITION.at_shader_location(0)];
        if layout.0.contains(Mesh::ATTRIBUTE_JOINT_INDEX) && layout.0.contains(Mesh::ATTRIBUTE_JOINT_WEIGHT) {
            attrs.push(Mesh::ATTRIBUTE_JOINT_INDEX.at_shader_location(7));
            attrs.push(Mesh::ATTRIBUTE_JOINT_WEIGHT.at_shader_location(8));
        }
        desc.vertex.buffers = vec![layout.0.get_layout(&attrs)?];
        desc.primitive.cull_mode = None;
        if let Some(ds) = desc.depth_stencil.as_mut() {
            // receivers redraw surfaces already in the depth buffer (reversed depth: near = 1)
            if desc.fragment.as_ref().is_some_and(|f| f.targets.iter().flatten().any(|t| t.blend.is_some())) {
                ds.depth_compare = CompareFunction::GreaterEqual.into();
                ds.depth_write_enabled = Some(false);
                ds.bias.constant = 4;
                ds.bias.slope_scale = 1.0;
            }
        }
        Ok(())
    }
}

pub struct ShadowPlugin;

impl Plugin for ShadowPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "shadow.wgsl");
        app.add_plugins(MaterialPlugin::<ShadowMat>::default());
    }
}

/// One shadow: its light camera, the receiver material and what it follows.
#[derive(Component)]
pub struct Shadow {
    /// Unit vector from the light's interest to its position (infinite light), or the light position.
    pub light_dir: Vec3,
    pub infinite: bool,
    pub light_pos: Vec3,
    /// The caster root (its world position is the camera's interest).
    pub caster: Entity,
    pub receiver_mat: Handle<ShadowMat>,
}

/// Set up a shadow of `caster_meshes` on `receiver_meshes` (entities with `Mesh3d`), parented to `parent`.
#[allow(clippy::too_many_arguments)]
pub fn spawn(commands: &mut Commands, images: &mut Assets<Image>, mats: &mut Assets<ShadowMat>, parent: Entity,
             caster: Entity, size: f32, caster_meshes: &[(Entity, Handle<Mesh>, Option<SkinnedMesh>, Option<Entity>)],
             receiver_meshes: &[(Entity, Handle<Mesh>, Option<SkinnedMesh>, Option<Entity>)], light: &colo_formats::hsd::Light, tex_size: (u32, u32), intensity: u8) {
    let img = images.add(Image::new_target_texture(tex_size.0, tex_size.1, TextureFormat::Rgba8UnormSrgb, None));
    let lp = Vec3::from_array(light.position);
    let li = Vec3::from_array(light.interest);
    let infinite = light.kind() == 1;
    let c = (255 - intensity) as f32 / 255.0;
    let flat = mats.add(ShadowMat { u: ShadowUniform { color: Color::srgb(c, c, c).to_linear().to_vec4(), ..default() }, tex: None });
    let recv = mats.add(ShadowMat { u: ShadowUniform { mode: UVec4::new(1, 0, 0, 0), ..default() }, tex: Some(img.clone()) });
    for (_, mesh, skin, p) in caster_meshes {
        let mut e = commands.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(flat.clone()), Transform::default(), RenderLayers::layer(LAYER)));
        if let Some(s) = skin { e.insert((s.clone(), bevy::camera::visibility::NoFrustumCulling)); }
        let id = e.id();
        if let Some(p) = p { commands.entity(*p).add_child(id); }
    }
    for (src, mesh, skin, p) in receiver_meshes {
        let mut e = commands.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(recv.clone()), Transform::default(), ReceiverOf(*src)));
        if let Some(s) = skin { e.insert((s.clone(), bevy::camera::visibility::NoFrustumCulling)); }
        let id = e.id();
        if let Some(p) = p { commands.entity(*p).add_child(id); }
    }
    let proj = if infinite {
        Projection::Orthographic(OrthographicProjection { near: NEAR, far: FAR, scaling_mode: ScalingMode::Fixed { width: 2.0 * size, height: 2.0 * size }, ..OrthographicProjection::default_3d() })
    } else {
        Projection::Perspective(PerspectiveProjection { near: NEAR, far: FAR, fov: 2.0 * (size / EYE_DISTANCE).atan(), aspect_ratio: 1.0, ..default() })
    };
    let cam = commands.spawn((Camera3d::default(), Camera { order: -2, clear_color: ClearColorConfig::Custom(Color::WHITE), ..default() },
        RenderTarget::Image(img.into()), Tonemapping::None, proj, Transform::default(), RenderLayers::layer(LAYER),
        Shadow { light_dir: (lp - li).normalize_or(Vec3::Y), infinite, light_pos: lp, caster, receiver_mat: recv })).id();
    commands.entity(parent).add_child(cam);
}

/// A receiver copy follows its source mesh's transform and visibility.
#[derive(Component)]
pub struct ReceiverOf(pub Entity);

/// Place each light camera on its caster and give the receivers its view-projection.
#[allow(clippy::type_complexity)]
pub fn update(mut cams: Query<(&Shadow, &mut Transform, &Projection)>, mut mats: ResMut<Assets<ShadowMat>>,
              mut recv: Query<(&ReceiverOf, &mut Transform, &mut Visibility), Without<Shadow>>, src: Query<(&Transform, &Visibility), (Without<ReceiverOf>, Without<Shadow>)>) {
    for (r, mut t, mut v) in &mut recv {
        if let Ok((st, sv)) = src.get(r.0) { *t = *st; *v = *sv; }
    }
    for (sh, mut t, proj) in &mut cams {
        // the caster root sits directly under the (identity) room root
        let Ok((g, _)) = src.get(sh.caster) else { continue };
        let interest = g.translation;
        let eye = if sh.infinite { interest + sh.light_dir * EYE_DISTANCE } else { sh.light_pos };
        let up = if sh.light_dir.cross(Vec3::Y).length_squared() < 1e-6 { Vec3::Z } else { Vec3::Y };
        *t = Transform::from_translation(eye).looking_at(interest, up);
        let clip = proj.get_clip_from_view() * t.to_matrix().inverse();
        if let Some(mut m) = mats.get_mut(&sh.receiver_mat) { m.u.light_clip = clip; }
    }
}
