// GX TEV interpreter for HSD materials (see tevmat.rs and colo_formats::tev).
// Each node of the material's TExp graph is one TEV stage, evaluated in the hardware's 8-bit integer
// arithmetic on gamma-space values. The rasterized channels are GX lighting of the room's lights.

#import bevy_pbr::{
    mesh_functions,
    skinning,
    view_transformations::position_world_to_clip,
    mesh_view_bindings::view,
    pbr_types,
    pbr_functions,
}

const MAX_NODES: u32 = 16u;
const MAX_LAYERS: u32 = 8u;

struct Tev {
    // per node: [layer | chan << 8 | bump << 16, colour op, alpha op, 0], [colour in a|b<<16, c|d<<16, alpha a|b<<16, c|d<<16]
    nodes: array<vec4<u32>, 32>,
    // constant registers, 0..255
    consts: array<vec4<f32>, 8>,
    // per layer: texture matrix rows (s: xyz, coord kind in w; t: xyz, uv set in w)
    layer_s: array<vec4<f32>, 8>,
    layer_t: array<vec4<f32>, 8>,
    // x: node count, y: channel flags (1 lit, 2 colour vtx, 4 alpha vtx, 8 specular), z: layer count, w: alpha test
    info: vec4<u32>,
    // x: material shininess, y: bump TObj layer (-1 none)
    misc: vec4<f32>,
    // material ambient colour bytes
    mat_amb: vec4<f32>,
    // per light 5 vec4s (tevmat.rs RoomLights): [pos, bits], [colour], [spot dir, a0], [a1, a2, k0, k1], [k2, spec vector]
    lights: array<vec4<f32>, 40>,
    // x: ambient light colour (r | g << 8 | b << 16 | a << 24), y: light count
    light_info: vec4<u32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> tev: Tev;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var t0: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var s0: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var t1: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var s1: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var t2: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var s2: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var t3: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(8) var s3: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(9) var t4: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(10) var s4: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(11) var t5: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(12) var s5: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(13) var t6: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(14) var s6: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(15) var t7: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(16) var s7: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) uv01: vec4<f32>,
    @location(4) uv23: vec4<f32>,
    @location(5) uv45: vec4<f32>,
    @location(6) uv67: vec4<f32>,
#ifdef SKINNED
    @location(7) joint_indices: vec4<u32>,
    @location(8) joint_weights: vec4<f32>,
#endif
#ifdef TEV_NBT
    @location(9) binormal: vec3<f32>,
    @location(10) tangent: vec3<f32>,
#endif
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) uv01: vec4<f32>,
    @location(4) uv23: vec4<f32>,
    @location(5) uv45: vec4<f32>,
    @location(6) uv67: vec4<f32>,
    // the model's origin (stands in for the joint's model-view translation of HSD_LObjSetupSpecularInit)
    @location(7) @interpolate(flat) origin: vec3<f32>,
#ifdef TEV_NBT
    @location(8) binormal: vec3<f32>,
    @location(9) tangent: vec3<f32>,
#endif
}

// PSMTXInvXpose: the GX normal matrix (PObjSetupMtx 0x801AABB4), the matrix itself when singular
fn inv_xpose(m: mat3x3<f32>) -> mat3x3<f32> {
    let c0 = cross(m[1], m[2]);
    let det = dot(m[0], c0);
    if det == 0.0 { return m; }
    return mat3x3<f32>(c0, cross(m[2], m[0]), cross(m[0], m[1])) * (1.0 / det);
}

@vertex
fn vertex(v: Vertex) -> VertexOutput {
    var out: VertexOutput;
#ifdef SKINNED
    let world_from_local = skinning::skin_model(v.joint_indices, v.joint_weights, v.instance_index);
    out.world_normal = skinning::skin_normals(world_from_local, v.normal);
#else
    let world_from_local = mesh_functions::get_world_from_local(v.instance_index);
    out.world_normal = mesh_functions::mesh_normal_local_to_world(v.normal, v.instance_index);
#endif
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(v.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
    out.color = v.color;
    out.uv01 = v.uv01;
    out.uv23 = v.uv23;
    out.uv45 = v.uv45;
    out.uv67 = v.uv67;
    out.origin = mesh_functions::get_world_from_local(v.instance_index)[3].xyz;
#ifdef TEV_NBT
    // not normalised: their length is the emboss offset scale
    let nm = inv_xpose(mat3x3<f32>(world_from_local[0].xyz, world_from_local[1].xyz, world_from_local[2].xyz));
    out.binormal = nm * v.binormal;
    out.tangent = nm * v.tangent;
#endif
    return out;
}

fn to_srgb(x: vec3<f32>) -> vec3<f32> {
    let c = clamp(x, vec3(0.0), vec3(1.0));
    return select(1.055 * pow(c, vec3(1.0 / 2.4)) - 0.055, c * 12.92, c <= vec3(0.0031308));
}
fn to_linear(x: vec3<f32>) -> vec3<f32> {
    return select(pow((x + 0.055) / 1.055, vec3(2.4)), x / 12.92, x <= vec3(0.04045));
}
fn bytes(x: vec4<f32>) -> vec4<i32> { return vec4<i32>(round(clamp(x, vec4(0.0), vec4(1.0)) * 255.0)); }

fn sample_layer(i: u32, uv: vec2<f32>) -> vec4<f32> {
    switch i {
        case 0u: { return textureSample(t0, s0, uv); }
        case 1u: { return textureSample(t1, s1, uv); }
        case 2u: { return textureSample(t2, s2, uv); }
        case 3u: { return textureSample(t3, s3, uv); }
        case 4u: { return textureSample(t4, s4, uv); }
        case 5u: { return textureSample(t5, s5, uv); }
        case 6u: { return textureSample(t6, s6, uv); }
        default: { return textureSample(t7, s7, uv); }
    }
}

// TEV input: kind (bits 0..2: 0 zero, 1 tex, 2 ras, 3 const, 4 node), sel (bits 3..6), index (bits 7..11)
fn tev_input(word: u32, alpha: bool, tex: vec4<i32>, ras: vec4<i32>, regs: ptr<function, array<vec4<i32>, 16>>) -> vec3<i32> {
    let kind = word & 7u;
    let sel = (word >> 3u) & 15u;
    let idx = (word >> 7u) & 31u;
    var px = vec4<i32>(0);
    switch kind {
        case 1u: { px = tex; }
        case 2u: { px = ras; }
        case 3u: { px = vec4<i32>(tev.consts[idx]); }
        case 4u: { px = (*regs)[idx]; }
        default: {}
    }
    switch sel {
        case 1u: { if alpha { return vec3(px.a); } return px.rgb; }
        case 2u: { return vec3(px.r); }
        case 3u: { return vec3(px.g); }
        case 4u: { return vec3(px.b); }
        case 5u: { return vec3(px.a); }
        case 6u: { return vec3(px.r); }
        case 7u: { return vec3(0); }
        case 8u: { return vec3(255); }
        default: { return vec3(i32(sel - 8u) * 32); }
    }
}

// op word: op bit 0, bias bits 1..2, scale bits 3..4, clamp bit 5
fn tev_lerp(a_in: vec3<i32>, b_in: vec3<i32>, c_in: vec3<i32>, d_in: vec3<i32>, word: u32, alpha: bool) -> vec3<i32> {
    let a = a_in & vec3(255);
    let b = b_in & vec3(255);
    var c = c_in & vec3(255);
    c = c + (c >> vec3(7u));
    let bias = (word >> 1u) & 3u;
    var d = d_in;
    if bias == 1u { d = d + 128; } else if bias == 2u { d = d - 128; }
    let shift = (word >> 3u) & 3u;
    var lerp = (a << vec3(8u)) + (b - a) * c;
    if shift != 3u { lerp = lerp << vec3(shift); d = d << vec3(shift); }
    let sub = (word & 1u) != 0u;
    if (shift == 3u) == alpha { lerp = lerp + select(128, 127, sub); }
    var r = lerp >> vec3(8u);
    r = select(d + r, d - r, sub);
    if shift == 3u { r = r >> vec3(1u); }
    if ((word >> 5u) & 1u) != 0u { return clamp(r, vec3(0), vec3(255)); }
    return clamp(r, vec3(-1024), vec3(1023));
}

fn alpha_compare(a: i32, func: u32, ref_: i32) -> bool {
    switch func {
        case 0u: { return false; }
        case 1u: { return a < ref_; }
        case 2u: { return a == ref_; }
        case 3u: { return a <= ref_; }
        case 4u: { return a > ref_; }
        case 5u: { return a != ref_; }
        case 6u: { return a >= ref_; }
        default: { return true; }
    }
}

// GX lighting (Dolphin's LightingShaderGen): per light, colour bytes x attenuation, summed onto the
// ambient in integer steps and clamped.
fn light_attn(i: u32, pos: vec3<f32>, n: vec3<f32>) -> vec2<f32> {
    let p = tev.lights[5u * i];
    let dir = tev.lights[5u * i + 2u];
    let k = tev.lights[5u * i + 3u];
    let k2 = tev.lights[5u * i + 4u].x;
    let v = p.xyz - pos;
    let dist2 = dot(v, v);
    let dist = sqrt(dist2);
    let l = v / dist;
    // AF_SPOT: GXInitLightDir stored the negated spot direction
    let c = max(0.0, dot(l, -dir.xyz));
    let attn = max(0.0, dir.w + k.x * c + k.y * c * c) / (k.z + k.w * dist + k2 * dist2);
    return vec2(attn, dot(l, n));
}

// COLOR0 (HSD_ChanLitColor0: GX_DF_CLAMP, GX_AF_SPOT, diffuse lights): amb = mat ambient x ambient light
// (HSD_MulColor), mat = vertex colour or white.
fn color0(in: VertexOutput, n: vec3<f32>, mat: vec3<i32>) -> vec3<i32> {
    let al = tev.light_info.x;
    let amb_light = vec3<u32>(al & 255u, (al >> 8u) & 255u, (al >> 16u) & 255u);
    var lacc = vec3<i32>(vec3<u32>(tev.mat_amb.rgb) * amb_light / 255u);
    for (var i = 0u; i < tev.light_info.y; i++) {
        let bits = u32(tev.lights[5u * i].w);
        if (bits & 1u) == 0u { continue; }
        let a = light_attn(i, in.world_position.xyz, n);
        lacc += vec3<i32>(round(a.x * max(0.0, a.y) * tev.lights[5u * i + 1u].rgb));
    }
    lacc = clamp(lacc, vec3(0), vec3(255));
    return (mat * (lacc + (lacc >> vec3(7u)))) >> vec3(8u);
}

// COLOR1 (HSD_ChanSpecular: white material, black ambient, GX_DF_NONE, GX_AF_SPEC) lit by the specular
// lights. HSD_LObjSetupSpecular (0x801A6098) gives each GXInitLightShininess(mat shininess): cos attn
// (0, 0, 1), dist attn (s/2, 0, 1 - s/2) on x = N.dir (0 when N.L < 0). HSD_LObjSetupSpecularInit
// (0x801A5DCC, per joint) sets dir to the half angle between the light (infinite: its direction, else
// from the light to the joint) and the eye-to-joint vector.
fn color1(in: VertexOutput, n: vec3<f32>) -> vec3<i32> {
    let s = tev.misc.x;
    let e = in.origin - view.world_position;
    let eye = select(normalize(e), -normalize((view.world_from_view * vec4(0.0, 0.0, 1.0, 0.0)).xyz), dot(e, e) < 1e-12);
    var lacc = vec3<i32>(0);
    for (var i = 0u; i < tev.light_info.y; i++) {
        let p = tev.lights[5u * i];
        let bits = u32(p.w);
        if (bits & 2u) == 0u { continue; }
        let sv = tev.lights[5u * i + 4u].yzw;
        var lv = sv;
        if (bits >> 2u) != 1u { lv = normalize(in.origin - sv); }
        let h = normalize(lv + eye);
        if dot(n, normalize(p.xyz - in.world_position.xyz)) < 0.0 { continue; }
        let x = max(0.0, dot(n, -h));
        let attn = max(0.0, x * x) / (0.5 * s + (1.0 - 0.5 * s) * x * x);
        lacc += vec3<i32>(round(attn * tev.lights[5u * i + 1u].rgb));
    }
    return clamp(lacc, vec3(0), vec3(255));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let flags = tev.info.y;
    let lit = (flags & 1u) != 0u;
    let vtx = vec4<f32>(in.color);
    // COLOR0A0 / COLOR1A1 (HSD_SetupChannelMode)
    var ras0: vec4<i32>;
    var ras1 = vec4<i32>(0);
    if lit {
        let n = normalize(in.world_normal);
        let m = bytes(vec4(select(vec3(1.0), vtx.rgb, (flags & 2u) != 0u), 0.0)).rgb;
        ras0 = vec4(color0(in, n, m), bytes(vec4(select(0.0, vtx.a, (flags & 4u) != 0u))).x);
        if (flags & 8u) != 0u {
            ras1 = vec4(color1(in, n), 0);
        }
    } else {
        ras0 = bytes(vec4(select(vec3(1.0), vtx.rgb, (flags & 2u) != 0u), select(1.0, vtx.a, (flags & 4u) != 0u)));
    }

    // texture coordinates and samples per layer
    var uvs = array<vec2<f32>, 8>(in.uv01.xy, in.uv01.zw, in.uv23.xy, in.uv23.zw, in.uv45.xy, in.uv45.zw, in.uv67.xy, in.uv67.zw);
    let n_view = normalize((view.view_from_world * vec4(normalize(in.world_normal), 0.0)).xyz);
    var texs: array<vec4<i32>, 8>;
    var tex_bump = vec4<i32>(255);
    let bump_layer = i32(tev.misc.y);
    for (var i = 0u; i < tev.info.z; i++) {
        let ms = tev.layer_s[i];
        let mt = tev.layer_t[i];
        var st: vec2<f32>;
        let kind = u32(ms.w);
        if kind == 1u {
            // TEX_COORD_REFLECTION: view normal -> (0.5 + 0.5 nx, 0.5 - 0.5 ny), then the TObj matrix
            st = vec2(0.5 + 0.5 * n_view.x, 0.5 - 0.5 * n_view.y);
        } else if kind == 2u {
            // TEX_COORD_HILIGHT: 0.5 + 0.5 (H . n) on both axes
            var h = vec3(0.0, 0.0, 1.0);
            for (var j = 0u; j < tev.light_info.y; j++) {
                if (u32(tev.lights[5u * j].w) >> 2u) != 1u { continue; }
                let l = normalize((view.view_from_world * vec4(-tev.lights[5u * j + 2u].xyz, 0.0)).xyz);
                h = normalize(l + vec3(0.0, 0.0, 1.0));
                break;
            }
            let x = 0.5 + 0.5 * dot(h, n_view);
            st = vec2(x, x);
        } else {
            st = uvs[u32(mt.w)];
        }
        let uv = vec2(dot(ms.xyz, vec3(st, 1.0)), dot(mt.xyz, vec3(st, 1.0)));
        let c = sample_layer(i, uv);
        texs[i] = bytes(vec4(to_srgb(c.rgb), c.a));
        if i32(i) == bump_layer {
            // GX_TG_BUMPn (HSD_TObjSetupTextureCoordGen 0x801BDA58: n = the first diffuse light, else 0) on
            // the layer's coordinate: + (L . binormal, L . tangent), L = normalize(light pos - vertex), as Dolphin
            var off = vec2(0.0);
#ifdef TEV_NBT
            var li = 0u;
            for (var j = 0u; j < tev.light_info.y; j++) {
                if (u32(tev.lights[5u * j].w) & 1u) != 0u { li = j; break; }
            }
            if li < tev.light_info.y && tev.misc.z == 0.0 {
                let l = normalize(tev.lights[5u * li].xyz - in.world_position.xyz);
                off = vec2(dot(l, in.binormal), dot(l, in.tangent));
            }
#endif
            let cb = sample_layer(i, uv + off);
            tex_bump = bytes(vec4(to_srgb(cb.rgb), cb.a));
        }
    }

    // the TEV
    var regs: array<vec4<i32>, 16>;
    let n = min(tev.info.x, MAX_NODES);
    for (var k = 0u; k < n; k++) {
        let h = tev.nodes[2u * k];
        let w = tev.nodes[2u * k + 1u];
        let layer = h.x & 255u;
        let chan = (h.x >> 8u) & 255u;
        var tex = vec4<i32>(255);
        if layer < MAX_LAYERS { tex = texs[layer]; }
        if ((h.x >> 16u) & 1u) != 0u { tex = tex_bump; }
        var ras = vec4<i32>(0);
        if chan == 4u { ras = ras0; } else if chan == 5u { ras = ras1; }
        var o = vec4<i32>(0);
        if (h.y & 64u) != 0u {
            o = vec4(tev_lerp(tev_input(w.x & 0xFFFFu, false, tex, ras, &regs), tev_input(w.x >> 16u, false, tex, ras, &regs),
                              tev_input(w.y & 0xFFFFu, false, tex, ras, &regs), tev_input(w.y >> 16u, false, tex, ras, &regs), h.y, false), 0);
        }
        if (h.z & 64u) != 0u {
            o.a = tev_lerp(tev_input(w.z & 0xFFFFu, true, tex, ras, &regs), tev_input(w.z >> 16u, true, tex, ras, &regs),
                           tev_input(w.w & 0xFFFFu, true, tex, ras, &regs), tev_input(w.w >> 16u, true, tex, ras, &regs), h.z, true).x;
        }
        regs[k] = o;
    }
    let res = clamp(regs[max(n, 1u) - 1u], vec4(0), vec4(255));

    // alpha compare: comp0 bits 0..2, ref0 3..10, op 11..12, comp1 13..15, ref1 16..23
    let at = tev.info.w;
    let p0 = alpha_compare(res.a, at & 7u, i32((at >> 3u) & 255u));
    let p1 = alpha_compare(res.a, (at >> 13u) & 7u, i32((at >> 16u) & 255u));
    var pass_: bool;
    switch (at >> 11u) & 3u {
        case 0u: { pass_ = p0 && p1; }
        case 1u: { pass_ = p0 || p1; }
        case 2u: { pass_ = p0 != p1; }
        default: { pass_ = p0 == p1; }
    }
    if !pass_ { discard; }

    let rgba = vec4(to_linear(vec3<f32>(res.rgb) / 255.0), f32(res.a) / 255.0);
    var p = pbr_types::pbr_input_new();
    p.frag_coord = in.position;
    p.world_position = in.world_position;
    p.material.flags = pbr_types::STANDARD_MATERIAL_FLAGS_FOG_ENABLED_BIT;
    return pbr_functions::main_pass_post_lighting_processing(p, rgba);
}
