// The GameCube pixel pipeline as one shader (an "ubershader"): every draw's TEV stages, konst/register state,
// texture sampling (wrap, LOD, 7-bit bilinear like the hardware) and alpha test come from its entry in `draws`
// (indexed by instance_index). Blending, depth and scissor are fixed-function per draw (lib.rs).
// The TEV arithmetic follows Dolphin's software renderer / colo-gx raster.rs (integer, 8-bit inputs, 11-bit
// signed registers, bias/scale/clamp, compare modes).

struct Draw {
    // wd, ht, zrange, xorig, yorig, farz, efb scale, _
    viewport: array<vec4<f32>, 2>,
    // nstages, ntex, afunc, _
    misc: vec4<u32>,
    // per stage: cenv, aenv, order, swaps
    stages: array<vec4<u32>, 16>,
    konst: array<vec4<i32>, 16>,
    regs: array<vec4<i32>, 4>,
    // per map: wrap_s | wrap_t << 2 | mag_linear << 4 | min << 5, width, height, levels
    samp: array<vec4<u32>, 8>,
    // per map: lod bias, min lod, max lod, present
    lod: array<vec4<f32>, 8>,
};

@group(0) @binding(0) var<storage, read> draws: array<Draw>;
@group(1) @binding(0) var t0: texture_2d<f32>;
@group(1) @binding(1) var t1: texture_2d<f32>;
@group(1) @binding(2) var t2: texture_2d<f32>;
@group(1) @binding(3) var t3: texture_2d<f32>;
@group(1) @binding(4) var t4: texture_2d<f32>;
@group(1) @binding(5) var t5: texture_2d<f32>;
@group(1) @binding(6) var t6: texture_2d<f32>;
@group(1) @binding(7) var t7: texture_2d<f32>;

struct VIn {
    @location(0) pos: vec4<f32>,
    @location(1) c0: vec4<f32>,
    @location(2) c1: vec4<f32>,
    @location(3) tc0: vec4<f32>,
    @location(4) tc1: vec4<f32>,
    @location(5) tc2: vec4<f32>,
    @location(6) tc3: vec4<f32>,
    @location(7) tc4: vec4<f32>,
    @location(8) tc5: vec4<f32>,
    @location(9) tc6: vec4<f32>,
    @location(10) tc7: vec4<f32>,
};

struct VOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) c0: vec4<f32>,
    @location(1) c1: vec4<f32>,
    @location(2) tc0: vec3<f32>,
    @location(3) tc1: vec3<f32>,
    @location(4) tc2: vec3<f32>,
    @location(5) tc3: vec3<f32>,
    @location(6) tc4: vec3<f32>,
    @location(7) tc5: vec3<f32>,
    @location(8) tc6: vec3<f32>,
    @location(9) tc7: vec3<f32>,
    @location(10) @interpolate(flat) draw: u32,
};

@vertex
fn vs(v: VIn, @builtin(instance_index) di: u32) -> VOut {
    let d = draws[di];
    let vp0 = d.viewport[0];
    let vp1 = d.viewport[1];
    let w = v.pos.w;
    // screen = ndc * (wd, ht) + orig - 342; depth = farz + z/w * zrange (24-bit units)
    let sx = v.pos.x / w * vp0.x + vp0.w - 342.0;
    let sy = v.pos.y / w * vp0.y + vp1.x - 342.0;
    let sz = (vp1.y + v.pos.z / w * vp0.z) / 16777215.0;
    var o: VOut;
    o.clip = vec4<f32>((sx / 320.0 - 1.0) * w, (1.0 - sy / 264.0) * w, sz * w, w);
    o.c0 = v.c0; o.c1 = v.c1;
    o.tc0 = v.tc0.xyz; o.tc1 = v.tc1.xyz; o.tc2 = v.tc2.xyz; o.tc3 = v.tc3.xyz;
    o.tc4 = v.tc4.xyz; o.tc5 = v.tc5.xyz; o.tc6 = v.tc6.xyz; o.tc7 = v.tc7.xyz;
    o.draw = di;
    return o;
}

fn wrapc(x: i32, n: i32, mode: u32) -> i32 {
    if mode == 0u { return clamp(x, 0, n - 1); }
    if mode == 1u { return ((x % n) + n) % n; }
    let p = ((x % (2 * n)) + 2 * n) % (2 * n);
    return select(2 * n - 1 - p, p, p < n);
}

fn fetch(m: u32, p: vec2<i32>, lv: i32) -> vec4<i32> {
    var c: vec4<f32>;
    switch m {
        case 0u: { c = textureLoad(t0, p, lv); }
        case 1u: { c = textureLoad(t1, p, lv); }
        case 2u: { c = textureLoad(t2, p, lv); }
        case 3u: { c = textureLoad(t3, p, lv); }
        case 4u: { c = textureLoad(t4, p, lv); }
        case 5u: { c = textureLoad(t5, p, lv); }
        case 6u: { c = textureLoad(t6, p, lv); }
        default: { c = textureLoad(t7, p, lv); }
    }
    return vec4<i32>(round(c * 255.0));
}

fn sample(d: Draw, m: u32, uv: vec2<f32>, grad: f32) -> vec4<i32> {
    let sp = d.samp[m];
    let ld = d.lod[m];
    if ld.w == 0.0 { return vec4<i32>(255, 0, 255, 255); }
    let w = i32(sp.y);
    let h = i32(sp.z);
    let nlev = i32(sp.w);
    let mode = sp.x;
    let lod = clamp(log2(max(grad * f32(max(w, h)), 1e-6)) + ld.x, ld.y, ld.z);
    var lin = (mode >> 4u & 1u) != 0u;
    var level = 0;
    if lod > 0.0 {
        let mn = mode >> 5u & 7u;
        lin = (mn & 4u) != 0u || mn == 1u;
        if (mn & 3u) != 0u && nlev > 1 { level = min(i32(round(lod)), nlev - 1); }
    }
    let lw = max(w >> u32(level), 1);
    let lh = max(h >> u32(level), 1);
    let ws = mode & 3u;
    let wt = mode >> 2u & 3u;
    let fu = uv.x * f32(lw);
    let fv = uv.y * f32(lh);
    if !lin {
        return fetch(m, vec2<i32>(wrapc(i32(floor(fu)), lw, ws), wrapc(i32(floor(fv)), lh, wt)), level);
    }
    let u = i32(round(fu * 128.0)) - 64;
    let v = i32(round(fv * 128.0)) - 64;
    let x0 = u >> 7u;
    let y0 = v >> 7u;
    let fx = u & 127;
    let fy = v & 127;
    let xa = wrapc(x0, lw, ws);
    let xb = wrapc(x0 + 1, lw, ws);
    let ya = wrapc(y0, lh, wt);
    let yb = wrapc(y0 + 1, lh, wt);
    let a = fetch(m, vec2<i32>(xa, ya), level);
    let b = fetch(m, vec2<i32>(xb, ya), level);
    let c = fetch(m, vec2<i32>(xa, yb), level);
    let e = fetch(m, vec2<i32>(xb, yb), level);
    let top = a * (128 - fx) + b * fx;
    let bot = c * (128 - fx) + e * fx;
    return (top * (128 - fy) + bot * fy) >> vec4<u32>(14u);
}

fn combine(a0: i32, b0: i32, c0: i32, d0: i32, env: u32, alpha: bool) -> i32 {
    let a = a0 & 255;
    let b = b0 & 255;
    var c = c0 & 255;
    c = c + (c >> 7u);
    let bias = env >> 16u & 3u;
    var d = d0;
    if bias == 1u { d = d + 128; } else if bias == 2u { d = d - 128; }
    let shift = env >> 20u & 3u;
    var lerp = (a << 8u) + (b - a) * c;
    if shift != 3u { lerp = lerp << shift; d = d << shift; }
    let sub = (env >> 18u & 1u) != 0u;
    if (shift == 3u) == alpha { lerp = lerp + select(128, 127, sub); }
    var r = lerp >> 8u;
    r = select(d + r, d - r, sub);
    if shift == 3u { r = r >> 1u; }
    if (env >> 19u & 1u) != 0u { return clamp(r, 0, 255); }
    return clamp(r, -1024, 1023);
}

fn pack(x: vec3<i32>, n: i32) -> i32 {
    var p = x.x & 255;
    if n > 1 { p = p | ((x.y & 255) << 8u); }
    if n > 2 { p = p | ((x.z & 255) << 16u); }
    return p;
}

fn cmp_hit(mode: u32, a: vec3<i32>, b: vec3<i32>, ka: i32, kb: i32) -> bool {
    switch mode {
        case 0u: { return pack(a, 1) > pack(b, 1); }
        case 1u: { return pack(a, 1) == pack(b, 1); }
        case 2u: { return pack(a, 2) > pack(b, 2); }
        case 3u: { return pack(a, 2) == pack(b, 2); }
        case 4u: { return pack(a, 3) > pack(b, 3); }
        case 5u: { return pack(a, 3) == pack(b, 3); }
        case 6u: { return (ka & 255) > (kb & 255); }
        default: { return (ka & 255) == (kb & 255); }
    }
}

fn atest(r: u32, a: i32) -> bool {
    let x = acmp(r >> 16u & 7u, a, i32(r & 255u));
    let y = acmp(r >> 19u & 7u, a, i32(r >> 8u & 255u));
    let l = r >> 22u & 3u;
    if l == 0u { return x && y; }
    if l == 1u { return x || y; }
    if l == 2u { return x != y; }
    return x == y;
}

fn acmp(f: u32, a: i32, rf: i32) -> bool {
    switch f {
        case 0u: { return false; }
        case 1u: { return a < rf; }
        case 2u: { return a == rf; }
        case 3u: { return a <= rf; }
        case 4u: { return a > rf; }
        case 5u: { return a != rf; }
        case 6u: { return a >= rf; }
        default: { return true; }
    }
}

fn swz(c: vec4<i32>, sw: u32) -> vec4<i32> {
    var o: vec4<i32>;
    o.x = c[sw & 3u];
    o.y = c[sw >> 2u & 3u];
    o.z = c[sw >> 4u & 3u];
    o.w = c[sw >> 6u & 3u];
    return o;
}

fn cin(k: u32, reg: array<vec4<i32>, 4>, tx: vec4<i32>, rc: vec4<i32>, kc: vec4<i32>) -> vec3<i32> {
    switch k {
        case 0u: { return reg[0].xyz; }
        case 1u: { return vec3<i32>(reg[0].w); }
        case 2u: { return reg[1].xyz; }
        case 3u: { return vec3<i32>(reg[1].w); }
        case 4u: { return reg[2].xyz; }
        case 5u: { return vec3<i32>(reg[2].w); }
        case 6u: { return reg[3].xyz; }
        case 7u: { return vec3<i32>(reg[3].w); }
        case 8u: { return tx.xyz; }
        case 9u: { return vec3<i32>(tx.w); }
        case 10u: { return rc.xyz; }
        case 11u: { return vec3<i32>(rc.w); }
        case 12u: { return vec3<i32>(255); }
        case 13u: { return vec3<i32>(128); }
        case 14u: { return kc.xyz; }
        default: { return vec3<i32>(0); }
    }
}

fn ain(k: u32, reg: array<vec4<i32>, 4>, tx: vec4<i32>, rc: vec4<i32>, kc: vec4<i32>) -> i32 {
    switch k {
        case 0u: { return reg[0].w; }
        case 1u: { return reg[1].w; }
        case 2u: { return reg[2].w; }
        case 3u: { return reg[3].w; }
        case 4u: { return tx.w; }
        case 5u: { return rc.w; }
        case 6u: { return kc.w; }
        default: { return 0; }
    }
}

@fragment
fn fs(i: VOut) -> @location(0) vec4<f32> {
    let d = draws[i.draw];
    var tcs = array<vec3<f32>, 8>(i.tc0, i.tc1, i.tc2, i.tc3, i.tc4, i.tc5, i.tc6, i.tc7);
    // texture coordinates and their screen-space footprint (for the LOD), in uniform control flow
    var uvs: array<vec2<f32>, 8>;
    var grads: array<f32, 8>;
    for (var k = 0u; k < 8u; k++) {
        let t = tcs[k];
        let q = select(t.z, 1.0, t.z == 0.0);
        let uv = t.xy / q;
        uvs[k] = uv;
        let scale = d.viewport[1].z;
        let g = max(abs(dpdx(uv)), abs(dpdy(uv))) * scale;
        grads[k] = max(g.x, g.y);
    }
    let ras0 = vec4<i32>(clamp(i.c0, vec4<f32>(0.0), vec4<f32>(255.0)));
    let ras1 = vec4<i32>(clamp(i.c1, vec4<f32>(0.0), vec4<f32>(255.0)));
    var reg = d.regs;
    let n = d.misc.x;
    var last = vec2<u32>(0u, 0u);
    for (var s = 0u; s < n; s++) {
        let st = d.stages[s];
        let cenv = st.x;
        let aenv = st.y;
        let order = st.z;
        let map = order & 7u;
        let coord = order >> 3u & 7u;
        let chan = order >> 7u & 7u;
        var rc = vec4<i32>(0);
        if chan == 0u { rc = ras0; } else if chan == 1u { rc = ras1; }
        rc = swz(rc, st.w & 0xFFu);
        var tx = vec4<i32>(255);
        if (order >> 6u & 1u) != 0u { tx = swz(sample(d, map, uvs[coord], grads[coord]), st.w >> 8u & 0xFFu); }
        let kc = d.konst[s];
        let a = cin(cenv >> 12u & 15u, reg, tx, rc, kc);
        let b = cin(cenv >> 8u & 15u, reg, tx, rc, kc);
        let c = cin(cenv >> 4u & 15u, reg, tx, rc, kc);
        let dd = cin(cenv & 15u, reg, tx, rc, kc);
        let aa = ain(aenv >> 13u & 7u, reg, tx, rc, kc);
        let ab = ain(aenv >> 10u & 7u, reg, tx, rc, kc);
        let ac = ain(aenv >> 7u & 7u, reg, tx, rc, kc);
        let ad = ain(aenv >> 4u & 7u, reg, tx, rc, kc);
        var oc: vec3<i32>;
        let cl = (cenv >> 19u & 1u) != 0u;
        if (cenv >> 16u & 3u) == 3u {
            let mode = (cenv >> 18u & 1u) | ((cenv >> 20u & 3u) << 1u);
            for (var k = 0u; k < 3u; k++) {
                let hit = cmp_hit(mode, a, b, a[k], b[k]);
                let r = dd[k] + select(0, c[k] & 255, hit);
                oc[k] = select(clamp(r, -1024, 1023), clamp(r, 0, 255), cl);
            }
        } else {
            oc = vec3<i32>(combine(a.x, b.x, c.x, dd.x, cenv, false), combine(a.y, b.y, c.y, dd.y, cenv, false), combine(a.z, b.z, c.z, dd.z, cenv, false));
        }
        var oa: i32;
        if (aenv >> 16u & 3u) == 3u {
            let mode = (aenv >> 18u & 1u) | ((aenv >> 20u & 3u) << 1u);
            let hit = cmp_hit(mode, a, b, aa, ab);
            let r = ad + select(0, ac & 255, hit);
            oa = select(clamp(r, -1024, 1023), clamp(r, 0, 255), (aenv >> 19u & 1u) != 0u);
        } else {
            oa = combine(aa, ab, ac, ad, aenv, true);
        }
        let cd = cenv >> 22u & 3u;
        let adst = aenv >> 22u & 3u;
        reg[cd] = vec4<i32>(oc, reg[cd].w);
        reg[adst].w = oa;
        last = vec2<u32>(cd, adst);
    }
    let out = vec4<i32>(clamp(reg[last.x].xyz, vec3<i32>(0), vec3<i32>(255)), clamp(reg[last.y].w, 0, 255));
    if !atest(d.misc.z, out.w) { discard; }
    return vec4<f32>(out) / 255.0;
}
