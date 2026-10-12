// HSD particles (particles.rs): psSetupTev's colour combiners on gamma-space values, alpha compare a >= 1/255.

#import bevy_pbr::{mesh_functions, view_transformations::position_world_to_clip}

struct Ps {
    // x: textured, y: blend mode, z: trail, w: mirror (1 S, 2 T)
    mode: vec4<u32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> ps: Ps;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var samp: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) prim: vec4<f32>,
    @location(3) env: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) prim: vec4<f32>,
    @location(2) env: vec4<f32>,
}

@vertex
fn vertex(v: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world = mesh_functions::mesh_position_local_to_world(mesh_functions::get_world_from_local(v.instance_index), vec4<f32>(v.position, 1.0));
    out.position = position_world_to_clip(world.xyz);
    out.uv = v.uv;
    out.prim = v.prim;
    out.env = v.env;
    return out;
}

fn to_linear(c: vec3<f32>) -> vec3<f32> {
    let lo = c / 12.92;
    let hi = pow((c + 0.055) / 1.055, vec3<f32>(2.4));
    return select(hi, lo, c <= vec3<f32>(0.04045));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var c: vec4<f32>;
    if ps.mode.x == 1u {
        // texture: lerp(env, prim, tex) (env is 0 without PrimEnv); trails: alpha x vertex alpha
        // MirrorS/T: coordinates doubled on a mirrored wrap
        var uv = in.uv;
        if (ps.mode.w & 1u) != 0u { uv.x = 1.0 - abs(1.0 - in.uv.x * 2.0); }
        if (ps.mode.w & 2u) != 0u { uv.y = 1.0 - abs(1.0 - in.uv.y * 2.0); }
        let t = textureSample(tex, samp, uv);
        c = mix(in.env, in.prim, t);
    } else {
        c = in.prim;
    }
    if c.a < 0.5 / 255.0 { discard; }
    return vec4<f32>(to_linear(clamp(c.rgb, vec3<f32>(0.0), vec3<f32>(1.0))), clamp(c.a, 0.0, 1.0));
}
