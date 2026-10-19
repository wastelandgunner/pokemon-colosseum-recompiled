// The GameCube pixel pipeline as one shader (an "ubershader"): every draw's TEV stages, konst/register state,
// texture sampling (wrap, LOD, 7-bit bilinear like the hardware) and alpha test come from its entry in `draws`
// (indexed by instance_index), then fog and logic ops. Blending, depth and scissor are fixed-function per draw (lib.rs).
// The TEV arithmetic follows Dolphin's software renderer / colo-gx raster.rs (integer, 8-bit inputs, 11-bit
// signed registers, bias/scale/clamp, compare modes).

struct Draw {
    // wd, ht, zrange, xorig, yorig, farz, efb scale, _
    viewport: array<vec4<f32>, 2>,
    // nstages, ntex, afunc, logic op (16 | op when on)
    misc: vec4<u32>,
    // per stage: cenv, aenv, order, swaps
    stages: array<vec4<u32>, 16>,
    konst: array<vec4<i32>, 16>,
    regs: array<vec4<i32>, 4>,
    // per map: wrap_s | wrap_t << 2 | mag_linear << 4 | min << 5, width, height, levels
    samp: array<vec4<u32>, 8>,
    // per map: lod bias, min lod, max lod, present (texels per GX texel: EFB copies are scaled; 0 = no texture)
    lod: array<vec4<f32>, 8>,
    // indirect texturing: IND_CMD per stage; IREF, coord scales (RAS1_SS0/1), indirect stages
    ind: array<vec4<u32>, 4>,
    ind_misc: vec4<u32>,
    // matrices 0-2, two rows each: (ma, mc, me, 17 - scale), (mb, md, mf, 0)
    ind_mtx: array<vec4<i32>, 6>,
    // SU texcoord scales (s, t) for coords 0-7, two per entry
    su: array<vec4<u32>, 4>,
    // fog: fsel | ortho << 3 | range adjust << 4, B magnitude, B shift, colour (r << 16 | g << 8 | b)
    fog: vec4<u32>,
    // A, C, range centre (EFB x), viewport half-width
    fogf: vec4<f32>,
    // range adjust k[0..10] (k[10] = 0)
    fogk: array<vec4<f32>, 3>,
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
// the EFB before this draw (logic ops)
@group(2) @binding(0) var efb_dst: texture_2d<f32>;

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

// A coordinate in the TEV's fixed-point texel space (1/128 texel of its SU scale), truncated like Dolphin.
fn fixuv(d: Draw, c: u32, uv: vec2<f32>) -> vec2<i32> {
    let e = d.su[c >> 1u];
    let sz = select(e.xy, e.zw, (c & 1u) != 0u);
    return vec2<i32>(uv * vec2<f32>(sz) * 128.0);
}

// A map's size in GameCube texels (an EFB copy's texture is `scale` times larger).
fn gxsize(d: Draw, m: u32) -> vec2<f32> {
    return vec2<f32>(f32(d.samp[m].y), f32(d.samp[m].z)) / max(d.lod[m].w, 1.0);
}

// Indirect wrap of a fixed-point coordinate: off, 256, 128, 64, 32, 16 texels, 0.
fn wrapi(x: i32, w: u32) -> i32 {
    if w == 0u { return x; }
    if w >= 6u { return 0; }
    return x & ((256 >> (w - 1u)) * 128 - 1);
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
    // indirect lookups (Dolphin's iindtex: the texel's alpha, blue, green as S, T, U)
    let nind = d.ind_misc.w;
    var iind: array<vec3<i32>, 4>;
    for (var k = 0u; k < 4u; k++) {
        if k >= nind { break; }
        let r = d.ind_misc.x >> (6u * k);
        let m = r & 7u;
        let c = r >> 3u & 7u;
        let ss = d.ind_misc[1u + (k >> 1u)] >> (8u * (k & 1u));
        let sh = vec2<u32>(ss & 15u, ss >> 4u & 15u);
        let fp = fixuv(d, c, uvs[c]) >> sh;
        let t = sample(d, m, vec2<f32>(fp) / (128.0 * gxsize(d, m)), grads[c] / f32(1u << max(sh.x, sh.y)));
        iind[k] = vec3<i32>(t.w, t.z, t.y);
    }
    var tevcoord = vec2<i32>(0);
    for (var s = 0u; s < n; s++) {
        let st = d.stages[s];
        let cenv = st.x;
        let aenv = st.y;
        let order = st.z;
        let map = order & 7u;
        let coord = order >> 3u & 7u;
        let chan = order >> 7u & 7u;
        let ten = (order >> 6u & 1u) != 0u;
        // indirect stage: offset (matrix times the indirect texel), wrap, add to the previous stage's coordinate
        let ic = d.ind[s >> 2u][s & 3u];
        let bt = ic & 3u;
        let fmt = ic >> 2u & 3u;
        let mid = ic >> 9u & 15u;
        let sw = ic >> 13u & 7u;
        let tw = ic >> 16u & 7u;
        var uv = uvs[coord];
        var alphabump = 0;
        if (sw != 0u || tw != 0u || mid != 0u || (ic >> 20u & 1u) != 0u) && bt < nind {
            let it = iind[bt];
            let bs = ic >> 7u & 3u;
            if bs != 0u { alphabump = (it[bs - 1u] << select(select(select(3u, 4u, fmt == 2u), 5u, fmt == 1u), 0u, fmt == 0u)) & 248; }
            let fu = fixuv(d, coord, uvs[coord]);
            var tr = vec2<i32>(0);
            if (mid & 3u) != 0u {
                var crd = it >> vec3<u32>(select(select(select(5u, 4u, fmt == 2u), 3u, fmt == 1u), 0u, fmt == 0u));
                let bias = ic >> 4u & 7u;
                let badd = select(1, -128, fmt == 0u);
                if (bias & 1u) != 0u { crd.x += badd; }
                if (bias & 2u) != 0u { crd.y += badd; }
                if (bias & 4u) != 0u { crd.z += badd; }
                let mi = (mid & 3u) - 1u;
                let r0 = d.ind_mtx[2u * mi];
                let r1 = d.ind_mtx[2u * mi + 1u];
                if mid < 4u { tr = vec2<i32>(dot(r0.xyz, crd), dot(r1.xyz, crd)) >> vec2<u32>(3u); }
                else if mid < 8u { tr = (fu * crd.xx) >> vec2<u32>(8u); }
                else { tr = (fu * crd.yy) >> vec2<u32>(8u); }
                if r0.w >= 0 { tr = tr >> vec2<u32>(u32(r0.w)); } else { tr = tr << vec2<u32>(u32(-r0.w)); }
            }
            let wr = vec2<i32>(wrapi(fu.x, sw), wrapi(fu.y, tw));
            if (ic >> 20u & 1u) != 0u { tevcoord = tevcoord + wr + tr; } else { tevcoord = wr + tr; }
            tevcoord = (tevcoord << vec2<u32>(8u)) >> vec2<u32>(8u);
            uv = vec2<f32>(tevcoord) / (128.0 * gxsize(d, map));
        } else if ten {
            tevcoord = fixuv(d, coord, uvs[coord]);
        } else {
            tevcoord = vec2<i32>(0);
        }
        var rc = vec4<i32>(0);
        if chan == 0u { rc = ras0; } else if chan == 1u { rc = ras1; }
        else if chan == 5u { rc = vec4<i32>(alphabump); } else if chan == 6u { rc = vec4<i32>(alphabump | (alphabump >> 5u)); }
        rc = swz(rc, st.w & 0xFFu);
        var tx = vec4<i32>(255);
        if ten { tx = swz(sample(d, map, uv, grads[coord]), st.w >> 8u & 0xFFu); }
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
    var out = vec4<i32>(clamp(reg[last.x].xyz, vec3<i32>(0), vec3<i32>(255)), clamp(reg[last.y].w, 0, 255));
    if !atest(d.misc.z, out.w) { discard; }
    out = vec4<i32>(fog(d, i.clip, out.xyz), out.w);
    if (d.misc.w & 16u) != 0u {
        let dst = vec4<i32>(round(textureLoad(efb_dst, vec2<i32>(i.clip.xy), 0) * 255.0));
        out = logic(d.misc.w & 15u, out, dst) & vec4<i32>(255);
    }
    return vec4<f32>(out) / 255.0;
}

// Fog as Dolphin's PixelShaderGen WriteFog (raster::Fog): eye z from the 24-bit screen z, range adjust, minus C,
// clamped, the fog function, then mixed with the fog colour in 1/256ths.
fn fog(d: Draw, p: vec4<f32>, c: vec3<i32>) -> vec3<i32> {
    let fsel = d.fog.x & 7u;
    if fsel == 0u { return c; }
    let z = u32(clamp(p.z, 0.0, 1.0) * 16777215.0 + 0.5);
    var ze: f32;
    if (d.fog.x >> 3u & 1u) != 0u { ze = d.fogf.x * f32(z) / 16777216.0; }
    else { ze = d.fogf.x * 16777216.0 / f32(i32(d.fog.y) - i32(z >> d.fog.z)); }
    if (d.fog.x >> 4u & 1u) != 0u {
        let off = (p.x / d.viewport[1].z - d.fogf.z) / d.fogf.w;
        let fi = clamp(9.0 - abs(off) * 9.0, 0.0, 9.0);
        let lo = u32(fi);
        let hi = lo + 1u;
        let k = mix(d.fogk[lo >> 2u][lo & 3u], d.fogk[hi >> 2u][hi & 3u], fract(fi));
        ze *= sqrt(off * off + k * k) / k;
    }
    var f = clamp(ze - d.fogf.y, 0.0, 1.0);
    if f != f { f = 0.0; }
    switch fsel {
        case 4u: { f = 1.0 - exp2(-8.0 * f); }
        case 5u: { f = 1.0 - exp2(-8.0 * f * f); }
        case 6u: { f = exp2(-8.0 * (1.0 - f)); }
        case 7u: { let g = 1.0 - f; f = exp2(-8.0 * g * g); }
        default: {}
    }
    let n = i32(round(f * 256.0));
    let col = vec3<i32>(i32(d.fog.w >> 16u & 255u), i32(d.fog.w >> 8u & 255u), i32(d.fog.w & 255u));
    return (c * (256 - n) + col * n) >> vec3<u32>(8u);
}

// PE logic ops (source = the pixel, destination = the EFB), as Dolphin's WriteLogicOp.
fn logic(op: u32, s: vec4<i32>, d: vec4<i32>) -> vec4<i32> {
    switch op {
        case 0u: { return vec4<i32>(0); }
        case 1u: { return s & d; }
        case 2u: { return s & ~d; }
        case 3u: { return s; }
        case 4u: { return ~s & d; }
        case 5u: { return d; }
        case 6u: { return s ^ d; }
        case 7u: { return s | d; }
        case 8u: { return ~(s | d); }
        case 9u: { return ~(s ^ d); }
        case 10u: { return ~d; }
        case 11u: { return s | ~d; }
        case 12u: { return ~s; }
        case 13u: { return ~s | d; }
        case 14u: { return ~(s & d); }
        default: { return vec4<i32>(255); }
    }
}
