//! `recomp --perf` / `--perf-log[=secs]`: frame rate and where the time goes, as an on-screen overlay (F3 toggles it)
//! and/or one stdout line per window. The game thread and the disc reader add to atomics in `Perf`; the UI thread
//! (recomp.rs `App`) adds its own costs, and once a window it folds both into text and resets them.
//!
//! What each figure measures:
//! - fps: new frames presented a second; game: `Machine::frame` calls a second; speed: the game's time base advance
//!   against real time (100% = full speed).
//! - cpu: `Machine::frame` (the lifted game code plus the hardware it drives); build: `gpu_cmds_deferred` (draw list
//!   from RAM); idle: the game thread's pacing sleep; wait: handing a frame to the render thread (it never blocks:
//!   with two frames already waiting the frame is dropped, counted in `dropped`).
//! - finish: `GFrame::finish` (vertex work), render: `Gpu::render` encode + submit (from the window's permit, see
//!   recomp.rs), both per frame on the render thread; present: blit, overlay and swap (window thread); gpu: submit to
//!   `on_submitted_work_done` (includes waiting behind earlier work and the swap, so it is latency, not GPU busy time).
//! - frame max: longest gap between presents (FIFO shows each at the next refresh); late: frames presented more than
//!   2 ms after they were due (each is a visible hitch); skipped: frames due but replaced before a present; resync:
//!   the game thread fell >150 ms behind and dropped its pacing debt.
//! - disc: reads through `Machine::set_disc`, their total and longest time (a stall shows as a high max).

use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Perf {
    pub frames: AtomicU64,
    pub cpu_ns: AtomicU64,
    pub cpu_max: AtomicU64,
    pub build_ns: AtomicU64,
    pub idle_ns: AtomicU64,
    pub wait_ns: AtomicU64,
    pub dropped: AtomicU64,
    pub tb: AtomicU64,
    pub resync: AtomicU64,
    pub disc_n: AtomicU64,
    pub disc_bytes: AtomicU64,
    pub disc_ns: AtomicU64,
    pub disc_max: AtomicU64,
    pub gpu_ns: AtomicU64,
    pub gpu_n: AtomicU64,
    pub gpu_max: AtomicU64,
    pub finish_ns: AtomicU64,
    pub render_ns: AtomicU64,
    pub rendered: AtomicU64,
}

pub fn add(a: &AtomicU64, d: Duration) -> u64 { let n = d.as_nanos() as u64; a.fetch_add(n, Relaxed); n }
pub fn add_max(sum: &AtomicU64, max: &AtomicU64, d: Duration) { max.fetch_max(add(sum, d), Relaxed); }

/// Options from the command line after `recomp`.
#[derive(Clone, Copy)]
pub struct Opts { pub overlay: bool, pub log: Option<f64> }

impl Opts {
    pub fn parse(args: &[String]) -> Opts {
        let mut o = Opts { overlay: false, log: None };
        for a in args {
            match a.as_str() {
                "--perf" => o.overlay = true,
                "--perf-log" => o.log = Some(1.0),
                _ => if let Some(s) = a.strip_prefix("--perf-log=") { o.log = Some(s.parse().unwrap_or(1.0f64).max(0.1)) }
                     else { eprintln!("recomp: unknown option {a} (--perf, --perf-log[=secs])") },
            }
        }
        o
    }
}

/// UI-thread side: its own per-window sums, and the text made from both.
pub struct Ui {
    pub opts: Opts,
    t0: Instant,
    last_log: Instant,
    last_shown: Option<Instant>,
    presents: u64,
    shown: u64,
    late: u64,
    frame_max: Duration,
    pub present: Duration,
    pub skipped: u64,
    pub info: String,
    pub lines: Vec<String>,
}

impl Ui {
    pub fn new(opts: Opts) -> Ui {
        let now = Instant::now();
        Ui { opts, t0: now, last_log: now, last_shown: None, presents: 0, shown: 0, late: 0, frame_max: Duration::ZERO,
             present: Duration::ZERO, skipped: 0, info: String::new(), lines: vec!["perf: measuring...".into()] }
    }

    pub fn presented(&mut self) { self.presents += 1; }

    /// A new frame went on screen at `now` (`skipped` due frames were passed over for it; `late`: after its refresh).
    pub fn shown(&mut self, now: Instant, skipped: u64, late: bool) {
        if let Some(l) = self.last_shown { self.frame_max = self.frame_max.max(now - l); }
        self.last_shown = Some(now);
        self.shown += 1;
        self.skipped += skipped;
        self.late += late as u64;
    }

    /// Every half second: new text from the window just ended (true when it changed). Prints the stdout line when
    /// its own interval is due.
    pub fn tick(&mut self, p: &Perf) -> bool {
        let now = Instant::now();
        let dt = (now - self.t0).as_secs_f64();
        if dt < 0.5 { return false; }
        let take = |a: &AtomicU64| a.swap(0, Relaxed);
        let ms = |ns: u64| ns as f64 / 1e6;
        let n = take(&p.frames);
        let per = |ns: u64| if n > 0 { ms(ns) / n as f64 } else { 0.0 };
        let (cpu, cpu_max, build, idle, wait) = (per(take(&p.cpu_ns)), ms(take(&p.cpu_max)), per(take(&p.build_ns)), per(take(&p.idle_ns)), per(take(&p.wait_ns)));
        let speed = take(&p.tb) as f64 / 40_500_000.0 / dt * 100.0;
        let resync = take(&p.resync);
        let dropped = take(&p.dropped);
        let (dn, db, dns, dmax) = (take(&p.disc_n), take(&p.disc_bytes), take(&p.disc_ns), take(&p.disc_max));
        let (gn, gns, gmax) = (take(&p.gpu_n), take(&p.gpu_ns), take(&p.gpu_max));
        let pr = self.presents.max(1) as f64;
        let ui = |d: Duration| d.as_secs_f64() * 1e3 / pr;
        let rn = take(&p.rendered).max(1) as f64;
        let (finish, render) = (ms(take(&p.finish_ns)) / rn, ms(take(&p.render_ns)) / rn);
        let gpu = if gn > 0 { ms(gns) / gn as f64 } else { 0.0 };
        self.lines = vec![
            format!("fps {:5.1}   game {:5.1}/s   speed {:3.0}%", self.shown as f64 / dt, n as f64 / dt, speed),
            format!("game  cpu {cpu:5.2} (max {cpu_max:5.1})  build {build:4.2}  idle {idle:5.2}  wait {wait:4.2} ms"),
            format!("ui    finish {finish:4.2}  render {render:4.2}  present {:4.2} ms", ui(self.present)),
            format!("gpu   {gpu:5.2} ms (max {:5.1})   frame max {:5.1} ms", ms(gmax), self.frame_max.as_secs_f64() * 1e3),
            format!("late {}  skipped {}  dropped {dropped}  resync {resync}   disc {dn} reads {} KB {:.2} ms (max {:.2})", self.late, self.skipped, db >> 10, ms(dns), ms(dmax)),
            self.info.clone(),
        ];
        if let Some(every) = self.opts.log {
            if (now - self.last_log).as_secs_f64() >= every - 0.25 {
                self.last_log = now;
                println!("perf: {}", self.lines[..5].join(" | ").split_whitespace().collect::<Vec<_>>().join(" "));
            }
        }
        self.t0 = now;
        (self.presents, self.shown, self.late, self.frame_max, self.present, self.skipped) = (0, 0, 0, Duration::ZERO, Duration::ZERO, 0);
        true
    }
}

const FONT: &[u8] = include_bytes!("../assets/PoppinsColo-Medium.ttf");

/// The viewer's UI font.
pub fn ui_font() -> ab_glyph::FontRef<'static> { ab_glyph::FontRef::try_from_slice(FONT).expect("ui font") }

/// `s`'s width in pixels at `px`.
pub fn text_w(font: &ab_glyph::FontRef, s: &str, px: f32) -> f32 {
    use ab_glyph::{Font, ScaleFont};
    let f = font.as_scaled(px);
    s.chars().map(|c| f.h_advance(f.glyph_id(c))).sum()
}

// Each texel lands on exactly one pixel: the quad is placed at an integer origin and the fragment shader loads the
// texel under it (no sampler, so no filtering).
const WGSL: &str = r#"
@group(0) @binding(0) var t: texture_2d<f32>;
struct U { origin: vec2f, size: vec2f, target_size: vec2f, _pad: vec2f };
@group(0) @binding(1) var<uniform> u: U;
@vertex fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4f {
    let p = (u.origin + vec2f(f32(i & 1u), f32(i >> 1u)) * u.size) / u.target_size;
    return vec4f(p.x * 2.0 - 1.0, 1.0 - p.y * 2.0, 0.0, 1.0);
}
@fragment fn fs(@builtin(position) p: vec4f) -> @location(0) vec4f { return textureLoad(t, vec2i(floor(p.xy - u.origin)), 0); }
"#;

/// Glyph outlines through skrifa's autohinter at the "light" target (vertical only, as desktop "slight" hinting):
/// the baseline, x-height and cap height land on whole pixels, so horizontal strokes are crisp instead of smeared
/// over two rows (ab_glyph alone draws the outlines unhinted). Advances stay linear, matching `text_w`.
pub struct Hinter {
    outlines: skrifa::outline::OutlineGlyphCollection<'static>,
    sizes: Vec<(f32, skrifa::outline::HintingInstance)>,
    raster: ab_glyph_rasterizer::Rasterizer,
}

/// An outline in pixels (y down), from skrifa (y up).
struct Path { at: (f32, f32), pen: (f32, f32), start: (f32, f32), segs: Vec<[(f32, f32); 4]>, kinds: Vec<u8> }

impl Path {
    fn p(&self, x: f32, y: f32) -> (f32, f32) { (self.at.0 + x, self.at.1 - y) }
    fn push(&mut self, k: u8, s: [(f32, f32); 4]) { self.pen = s[k as usize]; self.segs.push(s); self.kinds.push(k); }
}

impl skrifa::outline::OutlinePen for Path {
    fn move_to(&mut self, x: f32, y: f32) { self.pen = self.p(x, y); self.start = self.pen; }
    fn line_to(&mut self, x: f32, y: f32) { let s = [self.pen, self.p(x, y), (0.0, 0.0), (0.0, 0.0)]; self.push(1, s); }
    fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) { let s = [self.pen, self.p(cx, cy), self.p(x, y), (0.0, 0.0)]; self.push(2, s); }
    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        let s = [self.pen, self.p(cx0, cy0), self.p(cx1, cy1), self.p(x, y)];
        self.push(3, s);
    }
    fn close(&mut self) { if self.pen != self.start { let s = [self.pen, self.start, (0.0, 0.0), (0.0, 0.0)]; self.push(1, s); } }
}

impl Hinter {
    pub fn new() -> Hinter {
        use skrifa::MetadataProvider;
        let font = skrifa::FontRef::new(FONT).expect("ui font");
        Hinter { outlines: font.outline_glyphs(), sizes: vec![], raster: ab_glyph_rasterizer::Rasterizer::new(0, 0) }
    }

    /// Draw glyph `g` at `px` pixels per em with its origin (on the baseline) at `x`, `y`, calling `put` with each pixel's coverage.
    fn draw(&mut self, g: u32, px: f32, x: f32, y: f32, mut put: impl FnMut(i32, i32, f32)) {
        use skrifa::outline::{DrawSettings, Engine, HintingInstance, HintingOptions, SmoothMode, Target};
        let Some(glyph) = self.outlines.get(skrifa::GlyphId::new(g)) else { return };
        if !self.sizes.iter().any(|s| s.0 == px) {
            let opts = HintingOptions { engine: Engine::AutoFallback, target: Target::Smooth { mode: SmoothMode::Light, symmetric_rendering: true, preserve_linear_metrics: true } };
            let Ok(h) = HintingInstance::new(&self.outlines, skrifa::instance::Size::new(px), skrifa::instance::LocationRef::default(), opts) else { return };
            self.sizes.push((px, h));
        }
        let h = &self.sizes.iter().find(|s| s.0 == px).unwrap().1;
        let mut path = Path { at: (x, y), pen: (0.0, 0.0), start: (0.0, 0.0), segs: vec![], kinds: vec![] };
        if glyph.draw(DrawSettings::hinted(h, false), &mut path).is_err() || path.segs.is_empty() { return; }
        let pts = path.segs.iter().zip(&path.kinds).flat_map(|(s, k)| s[..=*k as usize].to_vec());
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for (px_, py) in pts { (x0, y0, x1, y1) = (x0.min(px_), y0.min(py), x1.max(px_), y1.max(py)); }
        let (ox, oy) = (x0.floor(), y0.floor());
        let (w, hh) = ((x1 - ox).ceil() as usize + 1, (y1 - oy).ceil() as usize + 1);
        self.raster.reset(w, hh);
        let q = |p: (f32, f32)| ab_glyph_rasterizer::point(p.0 - ox, p.1 - oy);
        for (s, k) in path.segs.iter().zip(&path.kinds) {
            match k {
                1 => self.raster.draw_line(q(s[0]), q(s[1])),
                2 => self.raster.draw_quad(q(s[0]), q(s[1]), q(s[2])),
                _ => self.raster.draw_cubic(q(s[0]), q(s[1]), q(s[2]), q(s[3])),
            }
        }
        self.raster.for_each_pixel_2d(|gx, gy, v| if v > 0.0 { put(ox as i32 + gx as i32, oy as i32 + gy as i32, v) });
    }
}

/// A picture for the overlay, drawn on the CPU: a background, filled rectangles and text, in pixels from its top-left.
pub struct Panel {
    pub w: u32,
    pub h: u32,
    pub bg: [u8; 4],
    /// x, y, w, h and colour (alpha blends it over what is under it).
    pub rects: Vec<([i32; 4], [u8; 4])>,
    /// x, top y, size in pixels, colour, text.
    pub texts: Vec<(i32, i32, f32, [u8; 4], String)>,
}

impl Panel {
    /// Rasterize: premultiplied RGBA, row by row.
    pub fn raster(&self, font: &ab_glyph::FontRef, hinter: &mut Hinter) -> Vec<u8> {
        use ab_glyph::{Font, ScaleFont};
        let (w, h) = (self.w as i32, self.h as i32);
        let mut img = vec![0u8; (4 * w * h) as usize];
        // source-over with coverage `a` of colour `c`, premultiplied
        let over = |img: &mut [u8], x: i32, y: i32, c: [u8; 4], a: f32| {
            if x < 0 || y < 0 || x >= w || y >= h { return; }
            let p = &mut img[(4 * (y * w + x)) as usize..][..4];
            let a = a.clamp(0.0, 1.0) * c[3] as f32 / 255.0;
            for i in 0..3 { p[i] = (c[i] as f32 * a + p[i] as f32 * (1.0 - a)).round() as u8; }
            p[3] = (255.0 * a + p[3] as f32 * (1.0 - a)).round() as u8;
        };
        for y in 0..h { for x in 0..w { over(&mut img, x, y, self.bg, 1.0); } }
        for ([rx, ry, rw, rh], c) in &self.rects {
            for y in *ry..ry + rh { for x in *rx..rx + rw { over(&mut img, x, y, *c, 1.0); } }
        }
        for (tx, ty, px, c, s) in &self.texts {
            let f = font.as_scaled(*px);
            let (mut x, y) = (*tx as f32, (*ty as f32 + f.ascent()).round());
            // ab_glyph's scale is the ascent-to-descent height; skrifa's is pixels per em
            let ppem = f.h_scale_factor() * font.units_per_em().unwrap_or(1000.0);
            for ch in s.chars() {
                let g = f.glyph_id(ch);
                hinter.draw(g.0 as u32, ppem, x, y, |gx, gy, v| over(&mut img, gx, gy, *c, v));
                x += f.h_advance(g);
            }
        }
        img
    }
}

/// The overlay: a `Panel` (text rasterized on the CPU with the viewer's UI font) in a texture, drawn 1:1 over the
/// presented frame.
pub struct Overlay {
    font: ab_glyph::FontRef<'static>,
    hinter: Hinter,
    pipe: Option<(wgpu::RenderPipeline, wgpu::BindGroupLayout, wgpu::Buffer, wgpu::TextureFormat)>,
    tex: Option<(wgpu::Texture, wgpu::BindGroup, u32, u32)>,
}

impl Overlay {
    pub fn new() -> Overlay { Overlay { font: ui_font(), hinter: Hinter::new(), pipe: None, tex: None } }

    /// `lines` at `px` pixels a line on a dark see-through background.
    pub fn set_text(&mut self, dev: &wgpu::Device, q: &wgpu::Queue, lines: &[String], px: f32) {
        use ab_glyph::{Font, ScaleFont};
        let f = self.font.as_scaled(px);
        let pad = (px * 0.4).ceil() as i32;
        let lh = (f.height() + f.line_gap()).ceil() as i32;
        let w = lines.iter().map(|l| text_w(&self.font, l, px)).fold(0.0f32, f32::max).ceil() as i32 + 2 * pad;
        let texts = lines.iter().enumerate().map(|(i, l)| (pad, pad + i as i32 * lh, px, [255; 4], l.clone())).collect();
        self.set_panel(dev, q, &Panel { w: w as u32, h: (lh * lines.len() as i32 + 2 * pad) as u32, bg: [0, 0, 0, 160], rects: vec![], texts });
    }

    /// Rasterize `p` and upload it.
    pub fn set_panel(&mut self, dev: &wgpu::Device, q: &wgpu::Queue, p: &Panel) {
        let (w, h) = (p.w.max(1), p.h.max(1));
        let img = if p.w == 0 || p.h == 0 { vec![0; 4] } else { p.raster(&self.font, &mut self.hinter) };
        if self.tex.as_ref().is_none_or(|t| t.2 != w || t.3 != h) {
            let Some((_, bgl, ub, _)) = &self.pipe else { return };
            let tex = dev.create_texture(&wgpu::TextureDescriptor {
                label: Some("perf overlay"), size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[],
            });
            let view = tex.create_view(&Default::default());
            let bg = dev.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: bgl, entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 1, resource: ub.as_entire_binding() }] });
            self.tex = Some((tex, bg, w, h));
        }
        let (tex, ..) = self.tex.as_ref().unwrap();
        q.write_texture(wgpu::TexelCopyTextureInfo { texture: tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            &img, wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(4 * w), rows_per_image: Some(h) },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 });
    }

    /// Make the pipeline for `format` (before the first `set_text`, and when the surface format changes).
    pub fn prepare(&mut self, dev: &wgpu::Device, format: wgpu::TextureFormat) {
        if self.pipe.as_ref().is_some_and(|p| p.3 == format) { return; }
        let m = dev.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("perf overlay"), source: wgpu::ShaderSource::Wgsl(WGSL.into()) });
        let bgl = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }] });
        let pl = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[Some(&bgl)], immediate_size: 0 });
        let p = dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("perf overlay"), layout: Some(&pl),
            vertex: wgpu::VertexState { module: &m, entry_point: Some("vs"), compilation_options: Default::default(), buffers: &[] },
            fragment: Some(wgpu::FragmentState { module: &m, entry_point: Some("fs"), compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })] }),
            primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleStrip, ..Default::default() },
            depth_stencil: None, multisample: Default::default(), multiview_mask: None, cache: None,
        });
        let ub = dev.create_buffer(&wgpu::BufferDescriptor { label: Some("perf overlay"), size: 32, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        self.pipe = Some((p, bgl, ub, format));
        self.tex = None;
    }

    /// Draw over `target` (already presented into, so its contents are kept), top-left.
    pub fn draw(&self, dev: &wgpu::Device, q: &wgpu::Queue, target: &wgpu::TextureView, tw: u32, th: u32) {
        self.draw_at(dev, q, target, tw, th, (8, 8));
    }

    /// The panel's size in pixels.
    pub fn size(&self) -> (u32, u32) { self.tex.as_ref().map_or((0, 0), |t| (t.2, t.3)) }

    /// Where a `w` x `h` panel goes centred in a `tw` x `th` target: whole pixels, so it stays 1:1.
    pub fn centre(tw: u32, th: u32, w: u32, h: u32) -> (i32, i32) { ((tw as i32 - w as i32) / 2, (th as i32 - h as i32) / 2) }

    /// Draw over `target` with the panel's top-left at pixel `at` (cut off where it leaves the target).
    pub fn draw_at(&self, dev: &wgpu::Device, q: &wgpu::Queue, target: &wgpu::TextureView, tw: u32, th: u32, at: (i32, i32)) {
        let (Some((p, _, ub, _)), Some((_, bg, w, h))) = (&self.pipe, &self.tex) else { return };
        if tw == 0 || th == 0 { return; }
        let u: [f32; 8] = [at.0 as f32, at.1 as f32, *w as f32, *h as f32, tw as f32, th as f32, 0.0, 0.0];
        q.write_buffer(ub, 0, &u.iter().flat_map(|f| f.to_le_bytes()).collect::<Vec<u8>>());
        let mut enc = dev.create_command_encoder(&Default::default());
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("perf overlay"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: target, resolve_target: None, depth_slice: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store } })],
                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None, multiview_mask: None,
            });
            pass.set_pipeline(p);
            pass.set_bind_group(0, bg, &[]);
            pass.draw(0..4, 0..1);
        }
        q.submit([enc.finish()]);
    }
}
