//! HSD material TEV programs: the expression graph that `MObjMakeTExp` (0x801A7128), `TObjMakeTExp`
//! (0x801BC8BC) and `MakeColorGenTExp` (0x801BCF30) build for an MObj and its TObjs, before
//! `HSD_TExpCompile` (0x801B4300) packs it into GX TEV stages. Every node here is one TEV stage of the
//! uncompiled graph; the compiler only merges and reorders stages, so evaluating the graph node by node
//! gives the same colours (each node clamps like the stage it becomes).
//!
//! Ported from the Ghidra decompilation of main.dol (GC6E01), not the decomp's C (its lit/unlit alpha
//! inputs disagree with the binary in places).

use crate::hsd::{Material, TexLayer, TObjTev, TEX_BUMP, TEX_LIGHTMAP_AMBIENT, TEX_LIGHTMAP_DIFFUSE, TEX_LIGHTMAP_EXT, TEX_LIGHTMAP_SPECULAR};

/// `HSD_TEInput` selectors.
pub const SEL_RGB: u8 = 1;
pub const SEL_R: u8 = 2;
pub const SEL_G: u8 = 3;
pub const SEL_B: u8 = 4;
pub const SEL_A: u8 = 5;
/// A scalar constant (all three colour channels, or the alpha).
pub const SEL_X: u8 = 6;
pub const SEL_0: u8 = 7;
pub const SEL_1: u8 = 8;
/// 1/8 .. 7/8 (9..=15): GX KONST fractions 32 * n.
pub const SEL_4_8: u8 = 12;

/// GXChannelID of a stage's rasterized colour.
pub const CHAN_COLOR0A0: u8 = 4;
pub const CHAN_COLOR1A1: u8 = 5;
pub const CHAN_NULL: u8 = 0xFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Src {
    Zero,
    /// The stage's texture (its TObj).
    Tex,
    /// The stage's rasterized channel colour.
    Ras,
    Const(usize),
    Node(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Input { pub sel: u8, pub src: Src }

const ZERO: Input = Input { sel: SEL_0, src: Src::Zero };

/// One half (colour or alpha) of a stage: `(d + ((1 - c) a + c b) + bias) * scale`, `op` 1 negating the lerp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Op { pub op: u8, pub bias: u8, pub scale: u8, pub clamp: u8, pub inputs: [Input; 4] }

impl Op {
    fn add(inputs: [Input; 4]) -> Op { Op { op: 0, bias: 0, scale: 0, clamp: 1, inputs } }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Node {
    /// `Material::layers` index of the stage's texture (`HSD_TExpOrder` tobj).
    pub layer: Option<usize>,
    /// The texture is sampled at the layer's emboss coordinate (GX_TG_BUMPn) instead of its own.
    pub bump: bool,
    pub chan: Option<u8>,
    pub color: Option<Op>,
    pub alpha: Option<Op>,
}

/// Where a constant register value comes from (re-read every frame for animated materials).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstSrc {
    /// `HSD_Material` diffuse / specular colour (u8 RGB).
    Diffuse,
    Specular,
    /// `HSD_Material` alpha (f32).
    Alpha,
    /// `lbl_80478C88` = 0xFF: the lit vertex-alpha material alpha.
    Opaque,
    /// TObj blending factor (f32).
    Blending(usize),
    /// TObj TEV KONST / TEV0 / TEV1 colour.
    Konst(usize),
    Tev0(usize),
    Tev1(usize),
}

/// `HSD_TExpCnst(val, comp, type)`: `byte` picks the channel of a colour for scalar (X) constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Const { pub src: ConstSrc, pub scalar: bool, pub byte: usize }

/// Current values the constants read (material/texture animation state).
#[derive(Debug, Clone, PartialEq)]
pub struct Values {
    pub diffuse: [u8; 4],
    pub specular: [u8; 4],
    pub alpha: f32,
    pub blending: Vec<f32>,
    pub tev: Vec<Option<TObjTev>>,
}

impl Values {
    pub fn of(m: &Material) -> Values {
        let b = |c: [f32; 4]| c.map(|x| (x * 255.0).round().clamp(0.0, 255.0) as u8);
        Values { diffuse: b(m.diffuse), specular: b(m.specular), alpha: m.alpha, blending: m.layers.iter().map(|l| l.blending).collect(),
            tev: m.layers.iter().map(|l| l.tev).collect() }
    }
}

/// `HSD_TExpSetReg` f32 conversion: (s32)(255 * x), clamped.
pub fn f32_to_u8(x: f32) -> u8 { ((255.0 * x) as i32).clamp(0, 255) as u8 }

impl Const {
    /// The register value: RGB constants fill r, g, b; scalar ones repeat their byte in all four.
    pub fn value(&self, v: &Values) -> [u8; 4] {
        let col = |c: [u8; 4]| if self.scalar { [c[self.byte]; 4] } else { [c[0], c[1], c[2], 0] };
        let tev = |l: usize, f: fn(&TObjTev) -> [u8; 4]| v.tev.get(l).copied().flatten().map(|t| f(&t)).unwrap_or([0; 4]);
        match self.src {
            ConstSrc::Diffuse => col(v.diffuse),
            ConstSrc::Specular => col(v.specular),
            ConstSrc::Alpha => [f32_to_u8(v.alpha); 4],
            ConstSrc::Opaque => [0xFF; 4],
            ConstSrc::Blending(l) => [f32_to_u8(v.blending.get(l).copied().unwrap_or(0.0)); 4],
            ConstSrc::Konst(l) => col(tev(l, |t| t.konst)),
            ConstSrc::Tev0(l) => col(tev(l, |t| t.tev0)),
            ConstSrc::Tev1(l) => col(tev(l, |t| t.tev1)),
        }
    }
}

/// How the rasterized channels are set up (`HSD_SetupChannelMode` 0x801B2F1C with the channel templates
/// at 0x8036CE88..0x8036CF78).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Channels {
    /// RENDER_DIFFUSE: COLOR0 is lit (GX_DF_CLAMP, GX_AF_SPOT, ambient = material ambient x ambient light).
    pub lit: bool,
    /// COLOR0's material colour is the vertex colour (else white).
    pub color_vtx: bool,
    /// ALPHA0 is the vertex alpha (else lit: the ambient light's alpha or 0; unlit: 255).
    pub alpha_vtx: bool,
    /// RENDER_SPECULAR: COLOR1 = specular lights (GX_AF_SPEC), alpha 0.
    pub specular: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// Nodes in evaluation order (every input refers to an earlier node); the last one is the output.
    pub nodes: Vec<Node>,
    pub consts: Vec<Const>,
    pub channels: Channels,
    /// `Material::layers` index of the bump TObj whose emboss coordinate the `bump` nodes sample.
    pub bump: Option<usize>,
}

struct Builder { nodes: Vec<Node>, consts: Vec<Const> }

impl Builder {
    fn tev(&mut self) -> usize { self.nodes.push(Node::default()); self.nodes.len() - 1 }
    fn cnst(&mut self, src: ConstSrc, scalar: bool, byte: usize) -> Src {
        let c = Const { src, scalar, byte };
        let i = self.consts.iter().position(|x| *x == c).unwrap_or_else(|| { self.consts.push(c); self.consts.len() - 1 });
        Src::Const(i)
    }
    fn order(&mut self, e: usize, layer: Option<usize>, chan: u8) {
        self.nodes[e].layer = layer;
        self.nodes[e].chan = (chan != CHAN_NULL).then_some(chan);
    }
    fn color(&mut self, e: usize, op: Op) { self.nodes[e].color = Some(op); }
    fn alpha(&mut self, e: usize, op: Op) { self.nodes[e].alpha = Some(op); }
}

fn i(sel: u8, src: Src) -> Input { Input { sel, src } }

/// Build the TEV graph of a material (`MObjMakeTExp`). The toon / shadow TObjs the engine can splice in
/// are not modelled (no material on the disc sets RENDER_TOON or RENDER_SHADOW).
pub fn build(m: &Material) -> Program {
    let rm = m.render_mode;
    let mut b = Builder { nodes: vec![], consts: vec![] };
    let mut diffuse = rm & 3;
    if diffuse == 0 { diffuse = 1; }
    let mut alpha_bits = rm & 0x6000;
    if alpha_bits == 0 { alpha_bits = diffuse << 13; }
    let lit = rm & 4 != 0;
    let toon: Option<usize> = None;
    let ras = if toon.is_some() { Src::Tex } else { Src::Ras };

    let e = b.tev();
    if !lit {
        match diffuse {
            2 => { b.order(e, toon, CHAN_COLOR0A0); b.color(e, Op::add([ZERO, ZERO, ZERO, i(SEL_RGB, ras)])); }
            1 => { let c = b.cnst(ConstSrc::Diffuse, false, 0); b.color(e, Op::add([ZERO, ZERO, ZERO, i(SEL_RGB, c)])); }
            _ => {
                let c = b.cnst(ConstSrc::Diffuse, false, 0);
                b.order(e, toon, CHAN_COLOR0A0);
                b.color(e, Op::add([i(SEL_RGB, ras), ZERO, ZERO, i(SEL_RGB, c)]));
            }
        }
        match alpha_bits {
            0x4000 => { b.order(e, toon, CHAN_COLOR0A0); b.alpha(e, Op::add([ZERO, ZERO, ZERO, i(SEL_A, Src::Ras)])); }
            0x2000 => { let c = b.cnst(ConstSrc::Alpha, true, 3); b.alpha(e, Op::add([ZERO, ZERO, ZERO, i(SEL_X, c)])); }
            _ => {
                let c = b.cnst(ConstSrc::Alpha, true, 3);
                b.order(e, toon, CHAN_COLOR0A0);
                b.alpha(e, Op::add([ZERO, i(SEL_A, Src::Ras), i(SEL_X, c), ZERO]));
            }
        }
    } else {
        if diffuse == 2 {
            b.color(e, Op::add([ZERO, ZERO, ZERO, i(SEL_1, Src::Zero)]));
        } else {
            let c = b.cnst(ConstSrc::Diffuse, false, 0);
            b.color(e, Op::add([ZERO, ZERO, ZERO, i(SEL_RGB, c)]));
        }
        let c = if alpha_bits == 0x4000 { b.cnst(ConstSrc::Opaque, true, 3) } else { b.cnst(ConstSrc::Alpha, true, 3) };
        b.alpha(e, Op::add([ZERO, ZERO, ZERO, i(SEL_X, c)]));
    }
    let (mut diff, mut alpha) = (Src::Node(e), Src::Node(e));

    let mut done = 0u32;
    // HSD_TObjAssignResources (0x801BC33C): only the last bump TObj keeps a texmap (two coordinates, the
    // UV one and its GX_TG_BUMPn offset); the others become GX_TEXMAP_NULL
    let bump = m.layers.iter().rposition(|l| l.flags & TEX_BUMP != 0 && l.id != 0xFF);
    let usable = |l: &TexLayer| l.id != 0xFF && (l.flags & TEX_BUMP == 0 || bump.is_some_and(|k| std::ptr::eq(l, &m.layers[k])));
    for (li, l) in m.layers.iter().enumerate() {
        if l.flags & (TEX_LIGHTMAP_DIFFUSE | TEX_LIGHTMAP_AMBIENT) != 0 && usable(l) {
            tobj_texp(&mut b, l, li, TEX_LIGHTMAP_DIFFUSE | TEX_LIGHTMAP_AMBIENT, done, &mut diff, &mut alpha);
        }
    }
    done |= TEX_LIGHTMAP_DIFFUSE | TEX_LIGHTMAP_AMBIENT;

    if lit {
        if alpha_bits & 0x4000 != 0 {
            let e = b.tev();
            b.order(e, None, CHAN_COLOR1A1);
            b.color(e, Op::add([ZERO, ZERO, ZERO, i(SEL_RGB, diff)]));
            b.alpha(e, Op::add([i(SEL_A, alpha), ZERO, i(SEL_A, Src::Ras), ZERO]));
            diff = Src::Node(e);
            alpha = Src::Node(e);
        }
        let e = b.tev();
        b.order(e, toon, CHAN_COLOR0A0);
        b.color(e, Op::add([ZERO, i(SEL_RGB, diff), i(SEL_RGB, ras), ZERO]));
        diff = Src::Node(e);
        if alpha_bits & 0x4000 == 0 {
            b.alpha(e, Op::add([i(SEL_A, alpha), ZERO, i(SEL_A, Src::Ras), ZERO]));
        } else {
            b.alpha(e, Op::add([ZERO, i(SEL_A, alpha), i(SEL_A, Src::Ras), ZERO]));
        }
        alpha = Src::Node(e);
    }

    let mut lightmap_done = TEX_LIGHTMAP_DIFFUSE | TEX_LIGHTMAP_AMBIENT;
    if rm & 8 != 0 {
        let c = b.cnst(ConstSrc::Specular, false, 0);
        let e = b.tev();
        b.color(e, Op::add([ZERO, ZERO, ZERO, i(SEL_RGB, c)]));
        let mut spec = Src::Node(e);
        for (li, l) in m.layers.iter().enumerate() {
            if l.flags & TEX_LIGHTMAP_SPECULAR != 0 && usable(l) {
                tobj_texp(&mut b, l, li, TEX_LIGHTMAP_SPECULAR, done, &mut spec, &mut alpha);
            }
        }
        lightmap_done = TEX_LIGHTMAP_DIFFUSE | TEX_LIGHTMAP_AMBIENT | TEX_LIGHTMAP_SPECULAR;
        let e = b.tev();
        b.order(e, None, CHAN_COLOR1A1);
        b.color(e, Op::add([ZERO, i(SEL_RGB, spec), i(SEL_RGB, Src::Ras), ZERO]));
        let e2 = b.tev();
        b.color(e2, Op::add([i(SEL_RGB, Src::Node(e)), ZERO, ZERO, i(SEL_RGB, diff)]));
        diff = Src::Node(e2);
    }

    let mut ext = diff;
    for (li, l) in m.layers.iter().enumerate() {
        if l.flags & TEX_LIGHTMAP_EXT != 0 && usable(l) {
            tobj_texp(&mut b, l, li, TEX_LIGHTMAP_EXT, lightmap_done, &mut ext, &mut alpha);
        }
    }
    let root = match (ext, alpha) {
        (Src::Node(x), Src::Node(y)) if x == y => x,
        _ => {
            let e = b.tev();
            b.color(e, Op::add([ZERO, ZERO, ZERO, i(SEL_RGB, ext)]));
            b.alpha(e, Op::add([ZERO, ZERO, ZERO, i(SEL_A, alpha)]));
            e
        }
    };
    // HSD_TObjSetupVolatileTev (0x801BD8D0), after the compiled TExp stages: emboss bump with the bump
    // TObj's texture, CPREV + tex(uv) (unclamped), then CPREV - tex(uv + light offset) (clamped); alpha passes
    let root = match bump {
        Some(li) => {
            let pass = Op::add([ZERO, ZERO, ZERO, i(SEL_A, Src::Node(root))]);
            let a = b.tev();
            b.order(a, Some(li), CHAN_COLOR0A0);
            b.color(a, Op { clamp: 0, ..Op::add([ZERO, i(SEL_RGB, Src::Tex), i(SEL_1, Src::Zero), i(SEL_RGB, Src::Node(root))]) });
            b.alpha(a, pass);
            let e = b.tev();
            b.order(e, Some(li), CHAN_COLOR0A0);
            b.nodes[e].bump = true;
            b.color(e, Op { op: 1, ..Op::add([ZERO, i(SEL_RGB, Src::Tex), i(SEL_1, Src::Zero), i(SEL_RGB, Src::Node(a))]) });
            b.alpha(e, Op::add([ZERO, ZERO, ZERO, i(SEL_A, Src::Node(a))]));
            e
        }
        None => root,
    };
    let channels = Channels { lit, color_vtx: diffuse & 2 != 0, alpha_vtx: alpha_bits & 0x4000 != 0, specular: rm & 8 != 0 };
    let mut p = sort(b, root, channels);
    p.bump = bump;
    p
}

/// `TObjMakeTExp`: one TObj's colour/alpha map stage on top of `c` / `a`.
fn tobj_texp(b: &mut Builder, l: &TexLayer, li: usize, lightmap: u32, done: u32, c: &mut Src, a: &mut Src) {
    let _ = lightmap;
    let repeat = done & l.flags & 0x1F0 != 0;
    let (mut c_src, mut a_src) = (Src::Tex, Src::Tex);
    let e0 = b.tev();
    if let Some(t) = l.tev.filter(|t| t.active & 0xC000_0000 != 0) {
        color_gen(b, &t, li, &mut c_src, &mut a_src);
    }
    b.order(e0, Some(li), CHAN_NULL);
    let op = match l.colormap() {
        1 => Op::add([i(SEL_RGB, *c), i(SEL_RGB, c_src), i(SEL_A, a_src), ZERO]),
        2 => Op::add([i(SEL_RGB, *c), i(SEL_RGB, c_src), i(SEL_RGB, c_src), ZERO]),
        3 => { let k = b.cnst(ConstSrc::Blending(li), true, 0); Op::add([i(SEL_RGB, *c), i(SEL_RGB, c_src), i(SEL_X, k), ZERO]) }
        4 => Op::add([ZERO, i(SEL_RGB, *c), i(SEL_RGB, c_src), ZERO]),
        5 => Op::add([ZERO, ZERO, ZERO, i(SEL_RGB, c_src)]),
        7 => Op::add([i(SEL_RGB, c_src), ZERO, ZERO, i(SEL_RGB, *c)]),
        8 => Op { op: 1, ..Op::add([i(SEL_RGB, c_src), ZERO, ZERO, i(SEL_RGB, *c)]) },
        _ => Op::add([ZERO, ZERO, ZERO, i(SEL_RGB, *c)]), // NONE, PASS (others assert)
    };
    b.color(e0, op);
    *c = Src::Node(e0);
    if repeat { return; }
    let op = match l.alphamap() {
        1 => Op::add([i(SEL_A, *a), i(SEL_A, a_src), i(SEL_A, a_src), ZERO]),
        2 => { let k = b.cnst(ConstSrc::Blending(li), true, 0); Op::add([i(SEL_A, *a), i(SEL_A, a_src), i(SEL_X, k), ZERO]) }
        3 => Op::add([ZERO, i(SEL_A, *a), i(SEL_A, a_src), ZERO]),
        4 => Op::add([ZERO, ZERO, ZERO, i(SEL_A, a_src)]),
        6 => Op::add([i(SEL_A, a_src), ZERO, ZERO, i(SEL_A, *a)]),
        7 => Op { op: 1, ..Op::add([i(SEL_A, a_src), ZERO, ZERO, i(SEL_A, *a)]) },
        _ => Op::add([ZERO, ZERO, ZERO, i(SEL_A, *a)]),
    };
    b.alpha(e0, op);
    *a = Src::Node(e0);
}

/// `MakeColorGenTExp`: the TObj's own TEV stage (KONST / TEV0 / TEV1 colours), which then replaces the
/// texture as the colour and/or alpha source of the colour/alpha map stage.
fn color_gen(b: &mut Builder, t: &TObjTev, li: usize, c: &mut Src, a: &mut Src) {
    let e0 = b.tev();
    b.order(e0, Some(li), CHAN_NULL);
    // a stage that only forwards a TEV0/TEV1 register (`HSD_TExpOrder(tmp, NULL, GX_COLOR_NULL)`)
    let fwd_c = |b: &mut Builder, sel: u8, src: ConstSrc, byte: usize| {
        let k = b.cnst(src, sel == SEL_X, byte);
        let n = b.tev();
        b.color(n, Op::add([ZERO, ZERO, ZERO, i(sel, k)]));
        Src::Node(n)
    };
    if t.active & 0x4000_0000 != 0 {
        let mut ins = [ZERO; 4];
        for (k, &x) in t.color_in.iter().enumerate() {
            ins[k] = match x {
                0x08 => i(SEL_RGB, Src::Tex),
                0x09 => i(SEL_A, Src::Tex),
                0x0C => i(SEL_1, Src::Zero),
                0x0D => i(SEL_4_8, Src::Zero),
                0x0F => ZERO,
                0x80 => i(SEL_RGB, b.cnst(ConstSrc::Konst(li), false, 0)),
                0x81..=0x84 => i(SEL_X, b.cnst(ConstSrc::Konst(li), true, (x - 0x81) as usize)),
                0x85 => i(SEL_RGB, fwd_c(b, SEL_RGB, ConstSrc::Tev0(li), 0)),
                0x86 => i(SEL_RGB, fwd_c(b, SEL_X, ConstSrc::Tev0(li), 3)),
                0x87 => i(SEL_RGB, fwd_c(b, SEL_RGB, ConstSrc::Tev1(li), 0)),
                0x88 => i(SEL_RGB, fwd_c(b, SEL_X, ConstSrc::Tev1(li), 3)),
                _ => ZERO, // asserts in the game (e.g. GX_CC_QUARTER)
            };
        }
        b.color(e0, Op { op: t.color_op, bias: t.color_bias, scale: t.color_scale, clamp: t.color_clamp, inputs: ins });
        *c = Src::Node(e0);
    }
    if t.active & 0x8000_0000 != 0 {
        let mut ins = [ZERO; 4];
        for (k, &x) in t.alpha_in.iter().enumerate() {
            ins[k] = match x {
                0x04 => i(SEL_A, Src::Tex),
                0x07 => ZERO,
                0x40..=0x43 => i(SEL_X, b.cnst(ConstSrc::Konst(li), true, (x - 0x40) as usize)),
                0x44 | 0x45 => {
                    let k = b.cnst(if x == 0x44 { ConstSrc::Tev0(li) } else { ConstSrc::Tev1(li) }, true, 3);
                    let n = b.tev();
                    b.alpha(n, Op::add([ZERO, ZERO, ZERO, i(SEL_X, k)]));
                    i(SEL_A, Src::Node(n))
                }
                _ => ZERO,
            };
        }
        b.alpha(e0, Op { op: t.alpha_op, bias: t.alpha_bias, scale: t.alpha_scale, clamp: t.alpha_clamp, inputs: ins });
        *a = Src::Node(e0);
    }
}

/// Keep the nodes the output depends on, in dependency order.
fn sort(b: Builder, root: usize, channels: Channels) -> Program {
    fn visit(n: usize, nodes: &[Node], state: &mut [u8], order: &mut Vec<usize>) {
        if state[n] != 0 { return; }
        state[n] = 1;
        for op in [nodes[n].color, nodes[n].alpha].into_iter().flatten() {
            for inp in op.inputs { if let Src::Node(m) = inp.src { visit(m, nodes, state, order); } }
        }
        state[n] = 2;
        order.push(n);
    }
    let mut state = vec![0u8; b.nodes.len()];
    let mut order = vec![];
    visit(root, &b.nodes, &mut state, &mut order);
    let mut remap = vec![usize::MAX; b.nodes.len()];
    for (k, &n) in order.iter().enumerate() { remap[n] = k; }
    let nodes = order.iter().map(|&n| {
        let mut x = b.nodes[n].clone();
        for op in [x.color.as_mut(), x.alpha.as_mut()].into_iter().flatten() {
            for inp in op.inputs.iter_mut() { if let Src::Node(m) = inp.src { inp.src = Src::Node(remap[m]); } }
        }
        x
    }).collect();
    Program { nodes, consts: b.consts, channels, bump: None }
}

/// Reference evaluation of a program on the CPU (u8 GX TEV arithmetic), for tests and tools.
/// `tex(layer, bump)` and `ras(chan)` give the stage inputs as RGBA bytes.
pub fn eval(p: &Program, v: &Values, tex: &dyn Fn(usize, bool) -> [u8; 4], ras: &dyn Fn(u8) -> [u8; 4]) -> [u8; 4] {
    let consts: Vec<[u8; 4]> = p.consts.iter().map(|c| c.value(v)).collect();
    let mut out: Vec<[i32; 4]> = Vec::with_capacity(p.nodes.len());
    for n in &p.nodes {
        let t = n.layer.map(|l| tex(l, n.bump)).unwrap_or([255; 4]);
        let r = n.chan.map(ras).unwrap_or([0; 4]);
        let get = |inp: &Input, alpha: bool| -> [i32; 3] {
            let px: [i32; 4] = match inp.src {
                Src::Zero => [0; 4],
                Src::Tex => t.map(i32::from),
                Src::Ras => r.map(i32::from),
                Src::Const(k) => consts[k].map(i32::from),
                Src::Node(k) => out[k],
            };
            let s = |x: i32| [x, x, x];
            match inp.sel {
                SEL_RGB => if alpha { s(px[3]) } else { [px[0], px[1], px[2]] },
                SEL_R => s(px[0]), SEL_G => s(px[1]), SEL_B => s(px[2]),
                SEL_A => s(px[3]),
                SEL_X => s(px[0]),
                SEL_0 => s(0),
                SEL_1 => s(255),
                k @ 9..=15 => s((k as i32 - 8) * 32),
                _ => s(0),
            }
        };
        let mut o = [0i32; 4];
        if let Some(op) = n.color {
            let x: Vec<[i32; 3]> = op.inputs.iter().map(|inp| get(inp, false)).collect();
            for ch in 0..3 { o[ch] = tev_lerp(x[0][ch], x[1][ch], x[2][ch], x[3][ch], &op, false); }
        }
        if let Some(op) = n.alpha {
            let x: Vec<i32> = op.inputs.iter().map(|inp| get(inp, true)[0]).collect();
            o[3] = tev_lerp(x[0], x[1], x[2], x[3], &op, true);
        }
        out.push(o);
    }
    out.last().map(|o| o.map(|x| x.clamp(0, 255) as u8)).unwrap_or([0; 4])
}

/// One TEV channel in integer arithmetic, as Dolphin's `tevLerp`: 8-bit a/b/c (c extended to 0..256),
/// signed d, the bias added to d, scale applied inside the lerp, round-to-nearest except for undivided
/// alpha (a hardware quirk), then clamp to 0..255 or the 11-bit signed range.
pub fn tev_lerp(a: i32, b: i32, c: i32, d: i32, op: &Op, alpha: bool) -> i32 {
    let (a, b, c) = (a & 255, b & 255, c & 255);
    let c = c + (c >> 7);
    let mut d = d + match op.bias { 1 => 128, 2 => -128, _ => 0 };
    let shift = op.scale.min(3) as i32;
    let mut lerp = (a << 8) + (b - a) * c;
    if shift != 3 { lerp <<= shift; d <<= shift; }
    let sub = op.op & 1 != 0;
    if (shift == 3) == alpha { lerp += if sub { 127 } else { 128 }; }
    let mut r = lerp >> 8;
    r = if sub { d - r } else { d + r };
    if shift == 3 { r >>= 1; }
    if op.clamp != 0 { r.clamp(0, 255) } else { r.clamp(-1024, 1023) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_matches_gx_endpoints() {
        let op = Op::add([ZERO; 4]);
        assert_eq!(tev_lerp(0, 255, 255, 0, &op, false), 255);
        assert_eq!(tev_lerp(255, 0, 0, 0, &op, false), 255);
        assert_eq!(tev_lerp(0, 0, 0, 77, &op, true), 77);
        // modulate: x * 255 = x, colour and alpha
        for x in [0, 1, 17, 128, 254, 255] { assert_eq!(tev_lerp(0, x, 255, 0, &op, false), x); assert_eq!(tev_lerp(0, x, 255, 0, &op, true), x); }
        // subtract clamps at 0
        assert_eq!(tev_lerp(200, 0, 0, 100, &Op { op: 1, ..op }, false), 0);
    }

    #[test]
    fn emboss_bump_stages() {
        use crate::hsd::{TexLayer, TexSrt, Wrap};
        // unlit MAT colour (diffuse 100), one bump TObj with no light map: out = diffuse + tex(uv) - tex(bump uv)
        let mut m = crate::hsd::default_material();
        m.render_mode = 1;
        m.diffuse = [100.0 / 255.0, 100.0 / 255.0, 100.0 / 255.0, 1.0];
        m.layers.push(TexLayer { tobj: 0, id: 0, src: 4, flags: TEX_BUMP, blending: 0.0, texture: None, wrap: [Wrap::Repeat; 2], srt: TexSrt::default(), tev: None });
        let p = build(&m);
        assert_eq!(p.bump, Some(0));
        let v = Values::of(&m);
        let out = |a: u8, b: u8| eval(&p, &v, &|_, bump| if bump { [b; 4] } else { [a; 4] }, &|_| [0; 4])[0];
        assert_eq!(out(200, 50), 250); // stage A unclamped (300), stage B clamps
        assert_eq!(out(200, 0), 255);
        assert_eq!(out(10, 200), 0);
        assert_eq!(out(80, 80), 100);
    }
}
