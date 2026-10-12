// HSD projected shadows (shadow.rs): casters drawn flat into the light camera's texture, receivers
// multiplied by that texture projected from the light.

#import bevy_pbr::{mesh_functions, skinning, view_transformations::position_world_to_clip}

struct Shadow {
    light_clip: mat4x4<f32>,
    // caster colour (the shadow intensity), rgb
    color: vec4<f32>,
    // x: 0 caster, 1 receiver
    mode: vec4<u32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> sh: Shadow;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var samp: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
#ifdef SKINNED
    @location(7) joint_indices: vec4<u32>,
    @location(8) joint_weights: vec4<f32>,
#endif
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
}

@vertex
fn vertex(v: Vertex) -> VertexOutput {
    var out: VertexOutput;
#ifdef SKINNED
    let world_from_local = skinning::skin_model(v.joint_indices, v.joint_weights, v.instance_index);
#else
    let world_from_local = mesh_functions::get_world_from_local(v.instance_index);
#endif
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(v.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    if sh.mode.x == 0u { return vec4<f32>(sh.color.rgb, 1.0); }
    let c = sh.light_clip * vec4<f32>(in.world_position.xyz, 1.0);
    let ndc = c.xyz / c.w;
    let uv = vec2<f32>(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
    if any(uv < vec2<f32>(0.0)) || any(uv > vec2<f32>(1.0)) { return vec4<f32>(1.0); }
    return vec4<f32>(textureSample(tex, samp, uv).rgb, 1.0);
}
