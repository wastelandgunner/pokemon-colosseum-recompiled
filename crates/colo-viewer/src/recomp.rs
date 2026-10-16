//! `colo-viewer <gcm> recomp`: the whole game on its own recompiled code (colo-battle `run.rs`), drawn on the
//! graphics card (colo-gx `cmd` draw lists -> colo-gxgpu) at `COLO_SCALE` (default 3) times the GameCube's
//! resolution, from power-on: Dolphin's `boot_snap.py` snapshot at `main` (env COLO_BOOT_SNAP, default
//! Working/dolphin/snaps/boot) and memory card A built from GCI files (env COLO_CARD, comma-separated; default the
//! story-2021 save).
//!
//! The game runs on its own thread, paced by its own time base (real time); frames go to the window
//! (winit + wgpu) and its AX mix to the audio output (audio.rs, buffered and resampled to the device). Keyboard:
//! Space/X/Enter A, Backspace/C B, V X, F Y, Tab Start, arrows d-pad, WASD stick, Z/Q/E Z/L/R; gamepads via gilrs.

use colo_recomp::run::{Machine, Pad};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{sync_channel, Receiver};
use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

const DOLPHIN: &str = "/media/zach/massstorage/Pokemon Colosseum/Working/dolphin";

pub fn run(gcm: &str) {
    let snap = std::env::var("COLO_BOOT_SNAP").map(std::path::PathBuf::from).unwrap_or_else(|_| std::path::Path::new(DOLPHIN).join("snaps/boot"));
    let cards: Vec<std::path::PathBuf> = std::env::var("COLO_CARD").map(|v| v.split(',').map(std::path::PathBuf::from).collect())
        .unwrap_or_else(|_| vec![std::path::Path::new(DOLPHIN).join("cards/story-2021.gci")]);
    let pad = Arc::new(Mutex::new(Pad::default()));
    let quit = Arc::new(AtomicBool::new(false));
    let (tx, rx) = sync_channel::<colo_gx::cmd::GFrame>(2);
    crate::audio::start_external();
    {
        let (pad, quit, gcm) = (pad.clone(), quit.clone(), std::path::PathBuf::from(gcm));
        std::thread::Builder::new().name("game".into()).stack_size(256 << 20).spawn(move || {
            let mut m = match Machine::from_snapshot(&snap) { Ok(m) => m, Err(e) => { eprintln!("recomp: {}: {e}", snap.display()); return; } };
            let f = Mutex::new(std::fs::File::open(&gcm).expect("disc image"));
            m.set_disc(Box::new(move |off, buf| {
                use std::io::{Read, Seek, SeekFrom};
                let mut f = f.lock().unwrap();
                if f.seek(SeekFrom::Start(off)).is_err() || f.read_exact(buf).is_err() { buf.fill(0); }
            }));
            let gcis: Vec<Vec<u8>> = cards.iter().filter_map(|p| std::fs::read(p).ok()).collect();
            if !gcis.is_empty() { m.insert_card(&gcis); }
            m.boot();
            let mut b = colo_gx::cmd::Builder::new();
            // paced by the game's own clock (time base, 40.5 MHz): 60 or 30 updates a second, whichever the
            // game runs, and the AX mix comes out at its real rate
            const TB_HZ: f64 = 40_500_000.0;
            let mut t0 = (std::time::Instant::now(), colo_recomp::run::time_base(&m.c));
            while !quit.load(Ordering::Relaxed) {
                m.set_pad(*pad.lock().unwrap());
                m.frame();
                let g = m.gpu_cmds(&mut b);
                let mut audio = colo_recomp::run::take_audio();
                for p in audio.chunks_mut(2) { p.swap(0, 1); }
                crate::audio::push_external(&audio);
                if tx.send(g).is_err() { break; }
                let tb = colo_recomp::run::time_base(&m.c);
                let next = t0.0 + std::time::Duration::from_secs_f64(tb.saturating_sub(t0.1) as f64 / TB_HZ);
                let now = std::time::Instant::now();
                if next > now { std::thread::sleep(next - now); }
                else if now - next > std::time::Duration::from_millis(150) { t0 = (now, tb); }
            }
        }).expect("game thread");
    }
    let el = EventLoop::new().expect("event loop");
    el.set_control_flow(ControlFlow::Poll);
    let mut app = App { rx, pad, quit, keys: HashSet::new(), gilrs: gilrs::Gilrs::new().ok(), gfx: None };
    el.run_app(&mut app).ok();
}

struct Gfx {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    gpu: colo_gxgpu::Gpu,
}

struct App {
    rx: Receiver<colo_gx::cmd::GFrame>,
    pad: Arc<Mutex<Pad>>,
    quit: Arc<AtomicBool>,
    keys: HashSet<KeyCode>,
    gilrs: Option<gilrs::Gilrs>,
    gfx: Option<Gfx>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.gfx.is_some() { return; }
        let window = Arc::new(el.create_window(Window::default_attributes().with_title("Pokémon Colosseum (recompiled)")
            .with_inner_size(winit::dpi::LogicalSize::new(960.0, 720.0))).expect("window"));
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).expect("surface");
        let adapter = futures_lite::future::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false })).expect("adapter");
        let (device, queue) = futures_lite::future::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).expect("device");
        let size = window.inner_size();
        let mut config = surface.get_default_config(&adapter, size.width.max(1), size.height.max(1)).expect("surface config");
        // the XFB holds final display values: an sRGB surface would encode them a second time (too bright)
        config.format = config.format.remove_srgb_suffix();
        surface.configure(&device, &config);
        let scale = std::env::var("COLO_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(3);
        self.gfx = Some(Gfx { window, surface, config, gpu: colo_gxgpu::Gpu::new(device, queue, scale) });
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, ev: WindowEvent) {
        match ev {
            WindowEvent::CloseRequested => { self.quit.store(true, Ordering::Relaxed); el.exit(); }
            WindowEvent::Resized(sz) => if let Some(g) = self.gfx.as_mut() {
                g.config.width = sz.width.max(1); g.config.height = sz.height.max(1);
                g.surface.configure(g.gpu.device(), &g.config);
            },
            WindowEvent::KeyboardInput { event, .. } => if let PhysicalKey::Code(k) = event.physical_key {
                if event.state == ElementState::Pressed { self.keys.insert(k); } else { self.keys.remove(&k); }
            },
            WindowEvent::RedrawRequested => self.redraw(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _el: &ActiveEventLoop) {
        *self.pad.lock().unwrap() = self.read_pad();
        let mut got = false;
        while let Ok(f) = self.rx.try_recv() {
            if let Some(g) = self.gfx.as_mut() { g.gpu.render(&f); }
            got = true;
        }
        if got { if let Some(g) = &self.gfx { g.window.request_redraw(); } } else { std::thread::sleep(std::time::Duration::from_millis(1)); }
    }
}

impl App {
    fn redraw(&mut self) {
        let Some(g) = self.gfx.as_mut() else { return };
        let frame = match g.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(f) | wgpu::CurrentSurfaceTexture::Suboptimal(f) => f,
            _ => { g.surface.configure(g.gpu.device(), &g.config); return; }
        };
        let view = frame.texture.create_view(&Default::default());
        g.gpu.present(&view, g.config.format, g.config.width, g.config.height);
        frame.present();
    }

    fn read_pad(&mut self) -> Pad {
        let k = |c: KeyCode| self.keys.contains(&c);
        let mut b = 0u16;
        let mut set = |on: bool, bit: u16| if on { b |= bit; };
        set(k(KeyCode::ArrowLeft), 0x1); set(k(KeyCode::ArrowRight), 0x2); set(k(KeyCode::ArrowDown), 0x4); set(k(KeyCode::ArrowUp), 0x8);
        set(k(KeyCode::KeyZ), 0x10); set(k(KeyCode::KeyE), 0x20); set(k(KeyCode::KeyQ), 0x40);
        set(k(KeyCode::Space) || k(KeyCode::KeyX) || k(KeyCode::Enter), 0x100);
        set(k(KeyCode::Backspace) || k(KeyCode::KeyC), 0x200);
        set(k(KeyCode::KeyV), 0x400); set(k(KeyCode::KeyF), 0x800); set(k(KeyCode::Tab), 0x1000);
        let (mut x, mut y) = (0i32, 0i32);
        if k(KeyCode::KeyD) { x += 100; }
        if k(KeyCode::KeyA) { x -= 100; }
        if k(KeyCode::KeyW) { y += 100; }
        if k(KeyCode::KeyS) { y -= 100; }
        if let Some(gi) = self.gilrs.as_mut() {
            while gi.next_event().is_some() {}
            for (_, gp) in gi.gamepads() {
                use gilrs::{Axis, Button};
                let ax = gp.value(Axis::LeftStickX);
                let ay = gp.value(Axis::LeftStickY);
                if ax.abs() > 0.15 || ay.abs() > 0.15 { x = (ax * 100.0) as i32; y = (ay * 100.0) as i32; }
                let mut g = |btn: Button, bit: u16| if gp.is_pressed(btn) { b |= bit; };
                g(Button::South, 0x100); g(Button::East, 0x200); g(Button::West, 0x400); g(Button::North, 0x800); g(Button::Start, 0x1000);
                g(Button::DPadLeft, 0x1); g(Button::DPadRight, 0x2); g(Button::DPadDown, 0x4); g(Button::DPadUp, 0x8);
                g(Button::RightTrigger, 0x10); g(Button::RightTrigger2, 0x20); g(Button::LeftTrigger2, 0x40);
            }
        }
        Pad { buttons: b, stick: (x.clamp(-128, 127) as i8, y.clamp(-128, 127) as i8), ..Default::default() }
    }
}
