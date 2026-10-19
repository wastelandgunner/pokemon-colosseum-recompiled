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
    pub fn any(&self) -> bool { self.overlay || self.log.is_some() }
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

const WGSL: &str = r#"
@group(0) @binding(0) var t: texture_2d<f32>;
@group(0) @binding(1) var s: sampler;
struct V { @builtin(position) p: vec4f, @location(0) uv: vec2f };
@vertex fn vs(@builtin(vertex_index) i: u32) -> V {
    let uv = vec2f(f32(i & 1u), f32(i >> 1u));
    var o: V; o.p = vec4f(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, 0.0, 1.0); o.uv = uv; return o;
}
@fragment fn fs(v: V) -> @location(0) vec4f { return textureSample(t, s, v.uv); }
"#;

/// The overlay: text rasterized on the CPU (ab_glyph, the viewer's UI font) into a texture, drawn top-left over
/// the presented frame.
pub struct Overlay {
    font: ab_glyph::FontRef<'static>,
    pipe: Option<(wgpu::RenderPipeline, wgpu::BindGroupLayout, wgpu::Sampler, wgpu::TextureFormat)>,
    tex: Option<(wgpu::Texture, wgpu::BindGroup, u32, u32)>,
}

impl Overlay {
    pub fn new() -> Overlay { Overlay { font: ab_glyph::FontRef::try_from_slice(FONT).expect("ui font"), pipe: None, tex: None } }

    /// Rasterize `lines` at `px` pixels a line and upload them.
    pub fn set_text(&mut self, dev: &wgpu::Device, q: &wgpu::Queue, lines: &[String], px: f32) {
        use ab_glyph::{Font, ScaleFont};
        let f = self.font.as_scaled(px);
        let pad = (px * 0.4).ceil() as u32;
        let lh = f.height() + f.line_gap();
        let w = lines.iter().map(|l| l.chars().map(|c| f.h_advance(f.glyph_id(c))).sum::<f32>()).fold(0.0f32, f32::max).ceil() as u32 + 2 * pad;
        let h = (lh * lines.len() as f32).ceil() as u32 + 2 * pad;
        let mut img = vec![0u8; (4 * w * h) as usize];
        for px4 in img.chunks_mut(4) { px4.copy_from_slice(&[0, 0, 0, 160]); }
        for (i, l) in lines.iter().enumerate() {
            let mut x = pad as f32;
            let y = pad as f32 + i as f32 * lh + f.ascent();
            for c in l.chars() {
                let g = f.glyph_id(c).with_scale_and_position(px, ab_glyph::point(x, y));
                x += f.h_advance(g.id);
                let Some(o) = self.font.outline_glyph(g) else { continue };
                let b = o.px_bounds();
                o.draw(|gx, gy, v| {
                    let (ix, iy) = (b.min.x as i32 + gx as i32, b.min.y as i32 + gy as i32);
                    if ix < 0 || iy < 0 || ix >= w as i32 || iy >= h as i32 { return; }
                    let p = &mut img[(4 * (iy as u32 * w + ix as u32)) as usize..][..4];
                    let a = v.clamp(0.0, 1.0);
                    for c in &mut p[..3] { *c = (*c as f32 * (1.0 - a) + 255.0 * a) as u8; }
                    p[3] = (p[3] as f32 * (1.0 - a) + 255.0 * a) as u8;
                });
            }
        }
        if self.tex.as_ref().is_none_or(|t| t.2 != w || t.3 != h) {
            let Some((_, bgl, smp, _)) = &self.pipe else { return };
            let tex = dev.create_texture(&wgpu::TextureDescriptor {
                label: Some("perf overlay"), size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[],
            });
            let view = tex.create_view(&Default::default());
            let bg = dev.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: bgl, entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(smp) }] });
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
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None }] });
        let pl = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[Some(&bgl)], immediate_size: 0 });
        let p = dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("perf overlay"), layout: Some(&pl),
            vertex: wgpu::VertexState { module: &m, entry_point: Some("vs"), compilation_options: Default::default(), buffers: &[] },
            fragment: Some(wgpu::FragmentState { module: &m, entry_point: Some("fs"), compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })] }),
            primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleStrip, ..Default::default() },
            depth_stencil: None, multisample: Default::default(), multiview_mask: None, cache: None,
        });
        let smp = dev.create_sampler(&wgpu::SamplerDescriptor::default());
        self.pipe = Some((p, bgl, smp, format));
        self.tex = None;
    }

    /// Draw over `target` (already presented into, so its contents are kept).
    pub fn draw(&self, dev: &wgpu::Device, q: &wgpu::Queue, target: &wgpu::TextureView, tw: u32, th: u32) {
        let (Some((p, ..)), Some((_, bg, w, h))) = (&self.pipe, &self.tex) else { return };
        let (w, h) = ((*w).min(tw.saturating_sub(8)), (*h).min(th.saturating_sub(8)));
        if w == 0 || h == 0 { return; }
        let mut enc = dev.create_command_encoder(&Default::default());
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("perf overlay"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: target, resolve_target: None, depth_slice: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store } })],
                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None, multiview_mask: None,
            });
            pass.set_viewport(8.0, 8.0, w as f32, h as f32, 0.0, 1.0);
            pass.set_pipeline(p);
            pass.set_bind_group(0, bg, &[]);
            pass.draw(0..4, 0..1);
        }
        q.submit([enc.finish()]);
    }
}
