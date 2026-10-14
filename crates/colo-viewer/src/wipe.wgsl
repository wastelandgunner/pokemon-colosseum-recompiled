// The game's fade layer (wipe.rs / fade.rs): one full-screen pass per game frame composites the 3D frame and
// the fade quads into a 640x480 "EFB" image, in gamma space with u8 alpha like GX; the image is also the
// fade texture the UDLR/trainer wipes re-capture every frame.
// A second pass shows that image in the window.

#import bevy_pbr::forward_io::VertexOutput

struct Quad {
    // p0.xy, e1.xy
    a: vec4<f32>,
    // e2.xy, uv0.xy
    b: vec4<f32>,
    // du.xy, dv.xy
    c: vec4<f32>,
    // rgb 0..255, alpha 0..255
    col: vec4<f32>,
    // x: source (0 capture, 1 ball0, 2 mask, 3 solid)
    src: vec4<u32>,
}

struct Wipe {
    quads: array<Quad, 32>,
    // x: quad count, y: 0 composite / 1 display, z: 1 = the fade texture exists
    n: vec4<u32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> w: Wipe;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var scene: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var prev: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var ball0: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var ball1: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var samp: sampler;

fn to_gamma(c: vec3<f32>) -> vec3<f32> {
    let lo = c * 12.92;
    let hi = 1.055 * pow(c, vec3<f32>(1.0 / 2.4)) - 0.055;
    return select(hi, lo, c <= vec3<f32>(0.0031308));
}

fn to_linear(c: vec3<f32>) -> vec3<f32> {
    let lo = c / 12.92;
    let hi = pow((c + 0.055) / 1.055, vec3<f32>(2.4));
    return select(hi, lo, c <= vec3<f32>(0.04045));
}

// one texel of the fade texture, clamped. The copy format is RGB565 (0x44), but Dolphin's EFB copies (the
// ground truth) keep full precision; quantizing here tints the smear (rainbow fringes) where Dolphin shows none.
fn texel565(p: vec2<i32>) -> vec3<f32> {
    return textureLoad(prev, clamp(p, vec2<i32>(0), vec2<i32>(639, 479)), 0).rgb;
}

// bilinear filter (GStextureSetFilter(2, 2)), clamp
fn capture(uv: vec2<f32>) -> vec3<f32> {
    let t = uv * vec2<f32>(640.0, 480.0) - 0.5;
    let i = vec2<i32>(floor(t));
    let f = t - floor(t);
    let a = mix(texel565(i), texel565(i + vec2<i32>(1, 0)), f.x);
    let b = mix(texel565(i + vec2<i32>(0, 1)), texel565(i + vec2<i32>(1, 1)), f.x);
    return mix(a, b, f.y);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    if w.n.y == 1u {
        // display: the composite holds gamma values; the window target is sRGB
        return vec4<f32>(to_linear(textureSample(prev, samp, in.uv).rgb), 1.0);
    }
    let px = in.position.xy;
    var c = to_gamma(textureLoad(scene, vec2<i32>(px), 0).rgb);
    for (var i = 0u; i < w.n.x; i++) {
        let q = w.quads[i];
        let d = px - q.a.xy;
        let e1 = q.a.zw;
        let e2 = q.b.xy;
        let det = e1.x * e2.y - e1.y * e2.x;
        if abs(det) < 1e-6 { continue; }
        let s = (d.x * e2.y - d.y * e2.x) / det;
        let t = (e1.x * d.y - e1.y * d.x) / det;
        if s < 0.0 || s >= 1.0 || t < 0.0 || t >= 1.0 { continue; }
        let uv = q.b.zw + s * q.c.xy + t * q.c.zw;
        var rgb = q.col.rgb / 255.0;
        var a = q.col.a / 255.0;
        switch q.src.x {
            case 0u: {
                if w.n.z == 0u { continue; }
                rgb = rgb * capture(uv);
            }
            case 1u: {
                let tx = textureSample(ball0, samp, uv);
                rgb = rgb * tx.rgb;
                a = a * tx.a;
            }
            case 2u: {
                rgb = capture(px / vec2<f32>(640.0, 480.0));
                a = a * textureSample(ball1, samp, uv).a;
            }
            default: {}
        }
        c = c * (1.0 - a) + rgb * a;
    }
    return vec4<f32>(c, 1.0);
}
