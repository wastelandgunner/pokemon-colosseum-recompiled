//! colo-gx draw lists (`cmd::GFrame`) on the graphics card with wgpu.
//!
//! The EFB is a render target `scale` times the GameCube's 640x528 (so the game renders at any resolution),
//! with a 32-bit float depth buffer holding the 24-bit Z / 2^24. Each draw: one TEV ubershader (tev.wgsl) whose
//! per-draw state comes from a storage buffer, fixed-function blending / depth / cull from a pipeline cache, the
//! scissor scaled. EFB copies stay on the card: copies to textures become textures later draws can sample (by
//! copy id, see cmd.rs), copies to the XFB become the frame `present` shows. Not modelled: logic ops (treated as
//! copy), the XFB's YUV round trip, Z-format copies, fog, indirect texturing.

use colo_gx::cmd::{GCopy, GFrame, GOp, GState, TexId};
use std::collections::HashMap;
use wgpu::util::DeviceExt;

pub const EFB_W: u32 = 640;
pub const EFB_H: u32 = 528;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct DrawU {
    viewport: [[f32; 4]; 2],
    misc: [u32; 4],
    stages: [[u32; 4]; 16],
    konst: [[i32; 4]; 16],
    regs: [[i32; 4]; 4],
    samp: [[u32; 4]; 8],
    lod: [[f32; 4]; 8],
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PipeKey { blend: u32, cmask: bool, amask: bool, alpha_const: bool, zen: bool, zfunc: u32, zwrite: bool, cull: u32 }

struct GTexture { view: wgpu::TextureView, w: u32, h: u32, levels: u32 }

/// An EFB copy kept on the card: the raw EFB pixels (scaled) and how the game asked for them to be encoded.
struct Copy { raw: GTexture, fmt: u32, intensity: bool, half: bool }

pub struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub scale: u32,
    efb: wgpu::Texture,
    efb_view: wgpu::TextureView,
    depth_view: wgpu::TextureView,
    layout: wgpu::PipelineLayout,
    module: wgpu::ShaderModule,
    bgl0: wgpu::BindGroupLayout,
    bgl1: wgpu::BindGroupLayout,
    pipes: HashMap<PipeKey, wgpu::RenderPipeline>,
    clear_pipes: HashMap<(bool, bool, bool), wgpu::RenderPipeline>,
    clear_layout: wgpu::PipelineLayout,
    clear_module: wgpu::ShaderModule,
    clear_bgl: wgpu::BindGroupLayout,
    textures: HashMap<u64, GTexture>,
    copies: HashMap<u32, Copy>,
    /// EFB copies as the texture format they are sampled with (encode with the copy format, decode with the texture's).
    conv: HashMap<(u32, u32), GTexture>,
    conv_pipe: wgpu::RenderPipeline,
    conv_bgl: wgpu::BindGroupLayout,
    dummy: wgpu::TextureView,
    /// The last XFB copy (what the TV shows).
    pub xfb: Option<(wgpu::TextureView, u32, u32)>,
    xfb_tex: Option<wgpu::Texture>,
    /// Debugging: draw only the ops in this range.
    pub only: Option<std::ops::Range<usize>>,
    blit: Option<(wgpu::RenderPipeline, wgpu::BindGroupLayout, wgpu::Sampler, wgpu::TextureFormat)>,
}

const CLEAR_WGSL: &str = r#"
struct C { color: vec4<f32>, z: vec4<f32> };
@group(0) @binding(0) var<uniform> c: C;
@vertex fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u)) * 2.0 - 1.0;
    return vec4<f32>(p, c.z.x, 1.0);
}
@fragment fn fs() -> @location(0) vec4<f32> { return c.color; }
"#;

/// EFB copy -> texture: what the game sees when it samples a copy made with copy format `fmt` (GX copy numbering:
/// 0 R4, 1 R8, 2 RA4, 3 RA8, 4 RGB565, 5 RGB5A3, 6 RGBA8, 7 A8, 8 R8, 9 G8, 10 B8, 11 RG8, 12 GB8; intensity turns
/// R into Y) as texture format `tf` (I4, I8, IA4, IA8, RGB565, RGB5A3, RGBA8), as `raster::encode` and the decoder.
const CONV_WGSL: &str = r#"
struct P { fmt: u32, intensity: u32, tf: u32, half: u32, scale: u32, pad0: u32, pad1: u32, pad2: u32 };
@group(0) @binding(0) var src: texture_2d<f32>;
@group(0) @binding(1) var<uniform> p: P;
@vertex fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let q = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u)) * 2.0 - 1.0;
    return vec4<f32>(q, 0.0, 1.0);
}
fn ld(x: i32, y: i32) -> vec4<u32> {
    let d = vec2<i32>(textureDimensions(src));
    return vec4<u32>(round(textureLoad(src, vec2<i32>(clamp(x, 0, d.x - 1), clamp(y, 0, d.y - 1)), 0) * 255.0));
}
@fragment fn fs(@builtin(position) fp: vec4<f32>) -> @location(0) vec4<f32> {
    let o = vec2<i32>(fp.xy);
    var c: vec4<u32>;
    if p.half != 0u {
        let s = i32(p.scale);
        let g = o / s;
        let f = o % s;
        let acc = ld(2 * g.x * s + f.x, 2 * g.y * s + f.y) + ld((2 * g.x + 1) * s + f.x, 2 * g.y * s + f.y)
                + ld(2 * g.x * s + f.x, (2 * g.y + 1) * s + f.y) + ld((2 * g.x + 1) * s + f.x, (2 * g.y + 1) * s + f.y);
        c = acc / 4u;
    } else {
        c = ld(o.x, o.y);
    }
    var i = c.r;
    if p.intensity != 0u { i = u32(clamp(((66 * i32(c.r) + 129 * i32(c.g) + 25 * i32(c.b) + 128) >> 8u) + 16, 0, 255)); }
    // the texel's bits as encoded
    var v: u32;
    switch p.fmt {
        case 0u: { v = i >> 4u; }
        case 1u, 8u: { v = i; }
        case 2u: { v = (c.a & 0xF0u) | (i >> 4u); }
        case 7u: { v = c.a; }
        case 9u: { v = c.g; }
        case 10u: { v = c.b; }
        case 3u: { v = (c.a << 8u) | i; }
        case 11u: { v = (c.r << 8u) | c.g; }
        case 12u: { v = (c.g << 8u) | c.b; }
        case 4u: { v = ((c.r >> 3u) << 11u) | ((c.g >> 2u) << 5u) | (c.b >> 3u); }
        case 5u: {
            if c.a >= 0xE0u { v = 0x8000u | ((c.r >> 3u) << 10u) | ((c.g >> 3u) << 5u) | (c.b >> 3u); }
            else { v = ((c.a >> 5u) << 12u) | ((c.r >> 4u) << 8u) | ((c.g >> 4u) << 4u) | (c.b >> 4u); }
        }
        default: { return vec4<f32>(c) / 255.0; }
    }
    // read back as the texture format
    var o4: vec4<u32>;
    switch p.tf {
        case 0u: { let n = (v & 15u) * 17u; o4 = vec4<u32>(n); }
        case 1u: { o4 = vec4<u32>(v & 255u); }
        case 2u: { let n = (v & 15u) * 17u; o4 = vec4<u32>(n, n, n, (v >> 4u & 15u) * 17u); }
        case 3u: { let n = v & 255u; o4 = vec4<u32>(n, n, n, v >> 8u & 255u); }
        case 4u: {
            let r = v >> 11u & 31u; let g = v >> 5u & 63u; let b = v & 31u;
            o4 = vec4<u32>((r << 3u) | (r >> 2u), (g << 2u) | (g >> 4u), (b << 3u) | (b >> 2u), 255u);
        }
        case 5u: {
            if (v & 0x8000u) != 0u {
                let r = v >> 10u & 31u; let g = v >> 5u & 31u; let b = v & 31u;
                o4 = vec4<u32>((r << 3u) | (r >> 2u), (g << 3u) | (g >> 2u), (b << 3u) | (b >> 2u), 255u);
            } else {
                let a = v >> 12u & 7u;
                o4 = vec4<u32>((v >> 8u & 15u) * 17u, (v >> 4u & 15u) * 17u, (v & 15u) * 17u, (a << 5u) | (a << 2u) | (a >> 1u));
            }
        }
        default: { o4 = c; }
    }
    return vec4<f32>(o4) / 255.0;
}
"#;

const BLIT_WGSL: &str = r#"
@group(0) @binding(0) var t: texture_2d<f32>;
@group(0) @binding(1) var s: sampler;
struct O { @builtin(position) p: vec4<f32>, @location(0) uv: vec2<f32> };
@vertex fn vs(@builtin(vertex_index) i: u32) -> O {
    let q = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    var o: O; o.p = vec4<f32>(q * 2.0 - 1.0, 0.0, 1.0); o.uv = vec2<f32>(q.x, 1.0 - q.y); return o;
}
@fragment fn fs(o: O) -> @location(0) vec4<f32> { return vec4<f32>(textureSample(t, s, o.uv).rgb, 1.0); }
"#;

fn compare(f: u32) -> wgpu::CompareFunction {
    use wgpu::CompareFunction::*;
    match f { 0 => Never, 1 => Less, 2 => Equal, 3 => LessEqual, 4 => Greater, 5 => NotEqual, 6 => GreaterEqual, _ => Always }
}

fn factor(f: u32, src: bool) -> wgpu::BlendFactor {
    use wgpu::BlendFactor::*;
    match f {
        0 => Zero, 1 => One,
        2 => if src { Dst } else { Src },
        3 => if src { OneMinusDst } else { OneMinusSrc },
        4 => SrcAlpha, 5 => OneMinusSrcAlpha, 6 => DstAlpha, _ => OneMinusDstAlpha,
    }
}

impl Gpu {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, scale: u32) -> Gpu {
        let scale = scale.max(1);
        let efb = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("efb"), size: wgpu::Extent3d { width: EFB_W * scale, height: EFB_H * scale, depth_or_array_layers: 1 },
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
        });
        let depth = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("efb depth"), size: wgpu::Extent3d { width: EFB_W * scale, height: EFB_H * scale, depth_or_array_layers: 1 },
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, view_formats: &[],
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("tev"), source: wgpu::ShaderSource::Wgsl(include_str!("tev.wgsl").into()) });
        let bgl0 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("draws"), entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0, visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None }] });
        let tex_entry = |b: u32| wgpu::BindGroupLayoutEntry { binding: b, visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None };
        let bgl1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("textures"), entries: &(0..8).map(tex_entry).collect::<Vec<_>>() });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("tev"), bind_group_layouts: &[Some(&bgl0), Some(&bgl1)], immediate_size: 0 });
        let clear_module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("clear"), source: wgpu::ShaderSource::Wgsl(CLEAR_WGSL.into()) });
        let clear_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("clear"), entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0, visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }] });
        let clear_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("clear"), bind_group_layouts: &[Some(&clear_bgl)], immediate_size: 0 });
        let dummy = device.create_texture_with_data(&queue, &wgpu::TextureDescriptor {
            label: Some("dummy"), size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
        }, wgpu::util::TextureDataOrder::LayerMajor, &[255, 0, 255, 255]).create_view(&Default::default());
        let conv_module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("copy convert"), source: wgpu::ShaderSource::Wgsl(CONV_WGSL.into()) });
        let conv_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("copy convert"), entries: &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }] });
        let conv_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[Some(&conv_bgl)], immediate_size: 0 });
        let conv_pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("copy convert"), layout: Some(&conv_layout),
            vertex: wgpu::VertexState { module: &conv_module, entry_point: Some("vs"), compilation_options: Default::default(), buffers: &[] },
            fragment: Some(wgpu::FragmentState { module: &conv_module, entry_point: Some("fs"), compilation_options: Default::default(), targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())] }),
            primitive: Default::default(), depth_stencil: None, multisample: Default::default(), multiview_mask: None, cache: None,
        });
        Gpu {
            conv: HashMap::new(), conv_pipe, conv_bgl,
            efb_view: efb.create_view(&Default::default()), depth_view: depth.create_view(&Default::default()), efb,
            device, queue, scale, layout, module, bgl0, bgl1, pipes: HashMap::new(), clear_pipes: HashMap::new(), clear_layout, clear_module, clear_bgl,
            textures: HashMap::new(), copies: HashMap::new(), dummy, xfb: None, xfb_tex: None, only: None, blit: None,
        }
    }

    fn pipe_key(st: &GState) -> PipeKey {
        let cm = st.cmode;
        PipeKey {
            blend: if cm & 1 != 0 { 1 | (cm >> 11 & 1) << 1 | (cm >> 8 & 7) << 2 | (cm >> 5 & 7) << 5 } else { 0 },
            cmask: cm >> 3 & 1 != 0, amask: cm >> 4 & 1 != 0 || st.pixfmt == 0 || st.dstalpha >> 8 & 1 != 0,
            alpha_const: st.pixfmt == 0 || st.dstalpha >> 8 & 1 != 0,
            zen: st.zmode & 1 != 0, zfunc: st.zmode >> 1 & 7, zwrite: st.zmode & 1 != 0 && st.zmode >> 4 & 1 != 0, cull: st.cull,
        }
    }

    fn pipeline(&mut self, k: PipeKey) -> &wgpu::RenderPipeline {
        if !self.pipes.contains_key(&k) {
            let color = if k.blend & 1 != 0 {
                if k.blend & 2 != 0 {
                    wgpu::BlendComponent { src_factor: wgpu::BlendFactor::One, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::ReverseSubtract }
                } else {
                    wgpu::BlendComponent { src_factor: factor(k.blend >> 2 & 7, true), dst_factor: factor(k.blend >> 5 & 7, false), operation: wgpu::BlendOperation::Add }
                }
            } else { wgpu::BlendComponent::REPLACE };
            let alpha = if k.alpha_const {
                wgpu::BlendComponent { src_factor: wgpu::BlendFactor::Constant, dst_factor: wgpu::BlendFactor::Zero, operation: wgpu::BlendOperation::Add }
            } else { color };
            let mut mask = wgpu::ColorWrites::empty();
            if k.cmask { mask |= wgpu::ColorWrites::COLOR; }
            if k.amask { mask |= wgpu::ColorWrites::ALPHA; }
            let attrs: Vec<wgpu::VertexAttribute> = (0..11).map(|l| wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 16 * l as u64, shader_location: l }).collect();
            let p = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("tev"), layout: Some(&self.layout),
                vertex: wgpu::VertexState { module: &self.module, entry_point: Some("vs"), compilation_options: Default::default(),
                    buffers: &[wgpu::VertexBufferLayout { array_stride: 176, step_mode: wgpu::VertexStepMode::Vertex, attributes: &attrs }] },
                fragment: Some(wgpu::FragmentState { module: &self.module, entry_point: Some("fs"), compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState { format: wgpu::TextureFormat::Rgba8Unorm, blend: Some(wgpu::BlendState { color, alpha }), write_mask: mask })] }),
                primitive: wgpu::PrimitiveState {
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: match k.cull { 1 => Some(wgpu::Face::Back), 2 => Some(wgpu::Face::Front), _ => None },
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: Some(k.zwrite),
                    depth_compare: Some(if k.zen { compare(k.zfunc) } else { wgpu::CompareFunction::Always }), stencil: Default::default(), bias: Default::default() }),
                multisample: Default::default(), multiview_mask: None, cache: None,
            });
            self.pipes.insert(k, p);
        }
        &self.pipes[&k]
    }

    fn clear_pipeline(&mut self, k: (bool, bool, bool)) -> &wgpu::RenderPipeline {
        if !self.clear_pipes.contains_key(&k) {
            let mut mask = wgpu::ColorWrites::empty();
            if k.0 { mask |= wgpu::ColorWrites::COLOR; }
            if k.1 { mask |= wgpu::ColorWrites::ALPHA; }
            let p = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("clear"), layout: Some(&self.clear_layout),
                vertex: wgpu::VertexState { module: &self.clear_module, entry_point: Some("vs"), compilation_options: Default::default(), buffers: &[] },
                fragment: Some(wgpu::FragmentState { module: &self.clear_module, entry_point: Some("fs"), compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState { format: wgpu::TextureFormat::Rgba8Unorm, blend: None, write_mask: mask })] }),
                primitive: Default::default(),
                depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: Some(k.2),
                    depth_compare: Some(wgpu::CompareFunction::Always), stencil: Default::default(), bias: Default::default() }),
                multisample: Default::default(), multiview_mask: None, cache: None,
            });
            self.clear_pipes.insert(k, p);
        }
        &self.clear_pipes[&k]
    }

    fn upload(&mut self, f: &GFrame) {
        for (key, t) in &f.new_tex {
            let levels = t.levels.len() as u32;
            let tex = self.device.create_texture(&wgpu::TextureDescriptor {
                label: None, size: wgpu::Extent3d { width: t.w, height: t.h, depth_or_array_layers: 1 }, mip_level_count: levels, sample_count: 1,
                dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[],
            });
            for (l, d) in t.levels.iter().enumerate() {
                let (lw, lh) = ((t.w >> l).max(1), (t.h >> l).max(1));
                self.queue.write_texture(wgpu::TexelCopyTextureInfo { texture: &tex, mip_level: l as u32, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                    d, wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(4 * lw), rows_per_image: Some(lh) },
                    wgpu::Extent3d { width: lw, height: lh, depth_or_array_layers: 1 });
            }
            self.textures.insert(*key, GTexture { view: tex.create_view(&Default::default()), w: t.w, h: t.h, levels });
        }
    }

    fn tex_info(&self, id: Option<TexId>) -> Option<&GTexture> {
        match id? {
            TexId::Mem(k) => self.textures.get(&k),
            TexId::Efb(c, tf) => self.conv.get(&(c, tf)).or_else(|| self.copies.get(&c).map(|cp| &cp.raw)),
        }
    }

    /// Run a frame's draws and copies on the EFB.
    pub fn render(&mut self, f: &GFrame) {
        self.upload(f);
        if f.ops.is_empty() { return; }
        let s = self.scale as f32;
        // EFB copies sampled as some texture format: converted textures, made after their copy (or now, for older copies)
        let made: HashMap<u32, &GCopy> = f.ops.iter().filter_map(|o| if let GOp::Copy(c) = o { (!c.to_xfb).then_some((c.id, c)) } else { None }).collect();
        let mut pending: HashMap<u32, Vec<u32>> = HashMap::new();
        for st in &f.states {
            for t in st.tex.iter().flatten() {
                let TexId::Efb(c, tf) = *t else { continue };
                if self.conv.contains_key(&(c, tf)) || pending.get(&c).is_some_and(|v| v.contains(&tf)) { continue; }
                let (fmt, half, w, h) = if let Some(cp) = made.get(&c) {
                    (cp.fmt, cp.half, cp.src[2].min(EFB_W) * self.scale, cp.src[3].min(EFB_H) * self.scale)
                } else if let Some(cp) = self.copies.get(&c) { (cp.fmt, cp.half, cp.raw.w, cp.raw.h) } else { continue };
                if fmt == 6 && !half && self.copies.contains_key(&c) { continue; }
                let s = self.scale;
                let (w, h) = if half { ((w / s / 2).max(1) * s, (h / s / 2).max(1) * s) } else { (w, h) };
                let tex = self.device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("efb copy as texture"), size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
                    dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT, view_formats: &[],
                });
                self.conv.insert((c, tf), GTexture { view: tex.create_view(&Default::default()), w, h, levels: 1 });
                pending.entry(c).or_default().push(tf);
            }
        }
        // per-draw uniforms
        let us: Vec<DrawU> = f.states.iter().map(|st| {
            let mut samp = [[0u32; 4]; 8];
            let mut lod = [[0f32; 4]; 8];
            for m in 0..8 {
                if let Some(t) = self.tex_info(st.tex[m]) {
                    let sp = st.samp[m];
                    samp[m] = [sp.wrap_s | sp.wrap_t << 2 | (sp.mag_linear as u32) << 4 | sp.min << 5, t.w, t.h, t.levels];
                    lod[m] = [sp.lod_bias, sp.min_lod, sp.max_lod, 1.0];
                }
            }
            DrawU {
                viewport: [[st.viewport[0], st.viewport[1], st.viewport[2], st.viewport[3]], [st.viewport[4], st.viewport[5], s, 0.0]],
                misc: [st.nstages, st.ntex, st.afunc, 0], stages: st.stages, konst: st.konst, regs: st.regs, samp, lod,
            }
        }).collect();
        let ubuf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("draws"), contents: bytemuck::cast_slice(&us), usage: wgpu::BufferUsages::STORAGE });
        let vbuf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("verts"), contents: bytemuck_verts(&f.verts), usage: wgpu::BufferUsages::VERTEX });
        let bg0 = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &self.bgl0, entries: &[wgpu::BindGroupEntry { binding: 0, resource: ubuf.as_entire_binding() }] });
        // texture bind groups, one per distinct set of textures
        let mut tex_bgs: HashMap<[Option<TexId>; 8], wgpu::BindGroup> = HashMap::new();
        for st in &f.states {
            if tex_bgs.contains_key(&st.tex) { continue; }
            let views: Vec<&wgpu::TextureView> = (0..8).map(|m| self.tex_info(st.tex[m]).map_or(&self.dummy, |t| &t.view)).collect();
            let entries: Vec<wgpu::BindGroupEntry> = views.iter().enumerate().map(|(b, v)| wgpu::BindGroupEntry { binding: b as u32, resource: wgpu::BindingResource::TextureView(v) }).collect();
            let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &self.bgl1, entries: &entries });
            tex_bgs.insert(st.tex, bg);
        }
        let keys: Vec<PipeKey> = f.states.iter().map(Gpu::pipe_key).collect();
        for k in &keys { self.pipeline(*k); }
        let mut enc = self.device.create_command_encoder(&Default::default());
        for (c, tfs) in &pending { if !made.contains_key(c) { for tf in tfs { self.convert(&mut enc, *c, *tf); } } }
        let mut i = 0;
        while i < f.ops.len() {
            // a run of draws in one pass
            let j = f.ops[i..].iter().position(|o| matches!(o, GOp::Copy(_))).map_or(f.ops.len(), |p| i + p);
            if j > i {
                let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("efb"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &self.efb_view, resolve_target: None, depth_slice: None,
                        ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store } })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &self.depth_view,
                        depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }), stencil_ops: None }),
                    timestamp_writes: None, occlusion_query_set: None, multiview_mask: None,
                });
                pass.set_vertex_buffer(0, vbuf.slice(..));
                pass.set_bind_group(0, &bg0, &[]);
                for (oi, op) in f.ops[i..j].iter().enumerate() {
                    let GOp::Draw { first, count, state } = op else { continue };
                    if self.only.as_ref().is_some_and(|r| !r.contains(&(i + oi))) { continue; }
                    let st = &f.states[*state as usize];
                    if st.cull == 3 { continue; }
                    let (x0, y0) = (st.scissor[0].max(0) as u32, st.scissor[1].max(0) as u32);
                    let (x1, y1) = ((st.scissor[2] + 1).clamp(0, EFB_W as i32) as u32, (st.scissor[3] + 1).clamp(0, EFB_H as i32) as u32);
                    if x1 <= x0 || y1 <= y0 { continue; }
                    pass.set_scissor_rect(x0 * self.scale, y0 * self.scale, (x1 - x0) * self.scale, (y1 - y0) * self.scale);
                    let a = if st.dstalpha >> 8 & 1 != 0 { (st.dstalpha & 0xFF) as f64 / 255.0 } else { 1.0 };
                    pass.set_blend_constant(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a });
                    pass.set_pipeline(&self.pipes[&keys[*state as usize]]);
                    pass.set_bind_group(1, &tex_bgs[&st.tex], &[]);
                    pass.draw(*first..*first + *count, *state..*state + 1);
                }
            }
            if let Some(GOp::Copy(c)) = f.ops.get(j) {
                self.copy(&mut enc, c);
                if let Some(tfs) = pending.get(&c.id) { for tf in tfs { self.convert(&mut enc, c.id, *tf); } }
            }
            i = j + 1;
        }
        self.queue.submit([enc.finish()]);
    }

    fn copy(&mut self, enc: &mut wgpu::CommandEncoder, c: &GCopy) {
        let s = self.scale;
        let (x, y) = (c.src[0].min(EFB_W - 1), c.src[1].min(EFB_H - 1));
        let (w, h) = (c.src[2].min(EFB_W - x).max(1), c.src[3].min(EFB_H - y).max(1));
        let tex = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("efb copy"), size: wgpu::Extent3d { width: w * s, height: h * s, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC, view_formats: &[],
        });
        enc.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo { texture: &self.efb, mip_level: 0, origin: wgpu::Origin3d { x: x * s, y: y * s, z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyTextureInfo { texture: &tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::Extent3d { width: w * s, height: h * s, depth_or_array_layers: 1 });
        let view = tex.create_view(&Default::default());
        if c.to_xfb { self.xfb = Some((view, w * s, h * s)); self.xfb_tex = Some(tex); } else { self.copies.insert(c.id, Copy { raw: GTexture { view, w: w * s, h: h * s, levels: 1 }, fmt: c.fmt, intensity: c.intensity, half: c.half }); }
        if let Some((col, z, cm, am, zm)) = c.clear {
            let u: [f32; 8] = [col[0] as f32 / 255.0, col[1] as f32 / 255.0, col[2] as f32 / 255.0, col[3] as f32 / 255.0, z as f32 / 16777215.0, 0.0, 0.0, 0.0];
            let ub = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(&u), usage: wgpu::BufferUsages::UNIFORM });
            let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &self.clear_bgl, entries: &[wgpu::BindGroupEntry { binding: 0, resource: ub.as_entire_binding() }] });
            self.clear_pipeline((cm, am, zm));
            let p = &self.clear_pipes[&(cm, am, zm)];
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("efb clear"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &self.efb_view, resolve_target: None, depth_slice: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store } })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }), stencil_ops: None }),
                timestamp_writes: None, occlusion_query_set: None, multiview_mask: None,
            });
            pass.set_scissor_rect(x * s, y * s, w * s, h * s);
            pass.set_pipeline(p);
            pass.set_bind_group(0, &bg, &[]);
            pass.draw(0..3, 0..1);
        }
        if self.copies.len() > 512 {
            // keep the most recent copies (ids grow)
            let mut ids: Vec<u32> = self.copies.keys().copied().collect();
            ids.sort();
            for id in &ids[..ids.len() - 256] { self.copies.remove(id); }
            let keep = ids[ids.len() - 256];
            self.conv.retain(|k, _| k.0 >= keep);
        }
    }

    /// Fill `conv[(id, tf)]` from the raw copy `id`.
    fn convert(&self, enc: &mut wgpu::CommandEncoder, id: u32, tf: u32) {
        let (Some(cp), Some(dst)) = (self.copies.get(&id), self.conv.get(&(id, tf))) else { return };
        let u: [u32; 8] = [cp.fmt, cp.intensity as u32, tf, cp.half as u32, self.scale, 0, 0, 0];
        let ub = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(&u), usage: wgpu::BufferUsages::UNIFORM });
        let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &self.conv_bgl, entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&cp.raw.view) },
            wgpu::BindGroupEntry { binding: 1, resource: ub.as_entire_binding() }] });
        let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("copy convert"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &dst.view, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } })],
            depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None, multiview_mask: None,
        });
        pass.set_pipeline(&self.conv_pipe);
        pass.set_bind_group(0, &bg, &[]);
        pass.draw(0..3, 0..1);
    }

    pub fn device(&self) -> &wgpu::Device { &self.device }

    /// Read the XFB back (RGBA8, width, height): tests and screenshots.
    pub fn read_xfb(&self) -> Option<(Vec<u8>, u32, u32)> {
        let (_, w, h) = self.xfb.as_ref()?;
        let (mut px, w, h) = self.read_tex(self.xfb_tex.as_ref()?, *w, *h)?;
        // the XFB is YUV: no alpha
        for p in px.chunks_mut(4) { p[3] = 255; }
        Some((px, w, h))
    }

    /// Read the EFB colour (scaled 640 x 528) back as RGBA8.
    pub fn read_efb(&self) -> Option<(Vec<u8>, u32, u32)> { self.read_tex(&self.efb, EFB_W * self.scale, EFB_H * self.scale) }

    fn read_tex(&self, tex: &wgpu::Texture, w: u32, h: u32) -> Option<(Vec<u8>, u32, u32)> {
        let bpr = (4 * w).div_ceil(256) * 256;
        let buf = self.device.create_buffer(&wgpu::BufferDescriptor { label: None, size: (bpr * h) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let mut enc = self.device.create_command_encoder(&Default::default());
        enc.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(bpr), rows_per_image: Some(h) } },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 });
        self.queue.submit([enc.finish()]);
        let slice = buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        self.device.poll(wgpu::PollType::wait_indefinitely()).ok();
        let data = slice.get_mapped_range();
        let mut out = Vec::with_capacity((4 * w * h) as usize);
        for y in 0..h { out.extend_from_slice(&data[(y * bpr) as usize..(y * bpr + 4 * w) as usize]); }
        Some((out, w, h))
    }

    /// Draw the XFB into `target` (a surface texture view), letterboxed to 4:3.
    pub fn present(&mut self, target: &wgpu::TextureView, format: wgpu::TextureFormat, tw: u32, th: u32) {
        if self.blit.as_ref().is_none_or(|b| b.3 != format) {
            let m = self.device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("blit"), source: wgpu::ShaderSource::Wgsl(BLIT_WGSL.into()) });
            let bgl = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None }] });
            let pl = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[Some(&bgl)], immediate_size: 0 });
            let p = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("blit"), layout: Some(&pl),
                vertex: wgpu::VertexState { module: &m, entry_point: Some("vs"), compilation_options: Default::default(), buffers: &[] },
                fragment: Some(wgpu::FragmentState { module: &m, entry_point: Some("fs"), compilation_options: Default::default(), targets: &[Some(format.into())] }),
                primitive: Default::default(), depth_stencil: None, multisample: Default::default(), multiview_mask: None, cache: None,
            });
            let smp = self.device.create_sampler(&wgpu::SamplerDescriptor { mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, ..Default::default() });
            self.blit = Some((p, bgl, smp, format));
        }
        let mut enc = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("present"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: target, resolve_target: None, depth_slice: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store } })],
                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None, multiview_mask: None,
            });
            if let (Some((xv, _, _)), Some((p, bgl, smp, _))) = (&self.xfb, &self.blit) {
                let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: bgl, entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(xv) },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(smp) }] });
                // 4:3 box centred
                let (w, h) = if tw * 3 > th * 4 { (th * 4 / 3, th) } else { (tw, tw * 3 / 4) };
                pass.set_viewport(((tw - w) / 2) as f32, ((th - h) / 2) as f32, w as f32, h as f32, 0.0, 1.0);
                pass.set_pipeline(p);
                pass.set_bind_group(0, &bg, &[]);
                pass.draw(0..3, 0..1);
            }
        }
        self.queue.submit([enc.finish()]);
    }
}

fn bytemuck_verts(v: &[colo_gx::cmd::GVertex]) -> &[u8] {
    // GVertex is repr(C), 44 f32
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v)) }
}
