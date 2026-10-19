//! `colo-viewer <gcm> recomp`: the whole game on its own recompiled code (colo-battle `run.rs`), drawn on the
//! graphics card (colo-gx `cmd` draw lists -> colo-gxgpu) at `COLO_SCALE` (default 3) times the GameCube's
//! resolution, from power-on: the disc image alone (`Machine::from_disc`: the DOL's `__start` and OSInit run on the
//! lifted code; no Dolphin data), or with env COLO_BOOT_SNAP=<dir> Dolphin's `boot_snap.py` snapshot at `main`; and
//! memory card A. The card is a raw image (Dolphin's .raw layout) kept on disk: env
//! COLO_CARD names it (default `save::working_dir()`/saves/MemoryCardA.USA.raw); when the file doesn't exist a blank card formatted
//! for this console is made there, and the game's writes go back to it. COLO_CARD as comma-separated .gci files
//! instead builds a card from those saves for this run only (writes are not kept).
//!
//! The game runs on its own thread, paced by its own time base (real time); frames go to a render thread
//! (`GFrame::finish` + `Gpu::render`) and then to the window (winit + wgpu), which presents each one at its due time,
//! and its AX mix to the audio output (audio.rs, buffered and resampled to the device).
//! Pacing: a frame is due `LEAD` (env COLO_FRAME_LEAD_MS, default 34) after the real time its game time ends at, so a
//! frame may cost up to about one frame more than its budget without a visible hitch. Due times follow the game's
//! clock, so they are evenly spaced; FIFO (env COLO_PRESENT=fifo|mailbox|immediate) shows each at the next refresh,
//! which holds a 30 updates/s game's frames for an even number of refreshes (2 at 60 Hz; 5/6 alternating at 165 Hz,
//! which 30 doesn't divide). One present per frame: the GPU runs one queue, so a present behind a render waits for it
//! (presenting every refresh lost ~1 refresh in 4 at 165 Hz); and the render thread submits a frame's render only
//! after the previous frame's present (a permit per frame shown), so a render never delays a present. The game
//! thread never waits on the window past its own schedule: if presenting falls behind (window hidden, screen locked)
//! it drops frames and keeps its time and sound. Threads ask rtkit for priority (`raise_priority`). Keyboard:
//! Space/X/Enter A, Backspace/C B, V X, F Y, Tab Start, arrows d-pad, WASD stick, Z/Q/E Z/L/R; gamepads via gilrs.
//! `recomp --perf` shows frame rate and frame costs on screen (F3 toggles), `--perf-log[=secs]` prints them (perf.rs).
//! Testing: env COLO_SNAP=<dir> runs a story snapshot (Working/dolphin/snaps/storyNN, as the `storybattle` example)
//! instead of booting, without the card; COLO_TAP_A=1 adds storybattle's A taps (3 frames in 20) to the pad.

use crate::perf::{self, Perf};
use colo_recomp::run::{Machine, Pad};
use std::time::{Duration, Instant};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, sync_channel, Receiver};
use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

pub fn run(gcm: &str, opts: perf::Opts) {
    let snap = std::env::var("COLO_BOOT_SNAP").ok().map(std::path::PathBuf::from);
    let card = std::env::var("COLO_CARD").unwrap_or_else(|_| crate::save::working_dir().join(CARD).to_string_lossy().into_owned());
    let (gcis, card): (Vec<std::path::PathBuf>, _) = if card.split(',').all(|p| p.to_ascii_lowercase().ends_with(".gci")) {
        (card.split(',').map(std::path::PathBuf::from).collect(), None)
    } else { (Vec::new(), Some(std::path::PathBuf::from(card))) };
    let pad = Arc::new(Mutex::new(Pad::default()));
    let quit = Arc::new(AtomicBool::new(false));
    let (tx, rx) = sync_channel::<(colo_gx::cmd::GFrame, Instant)>(2);
    let lead = Duration::from_millis(std::env::var("COLO_FRAME_LEAD_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(34));
    let stats = opts.any().then(|| Arc::new(Perf::default()));
    crate::audio::start_external();
    let game = {
        let (pad, quit, gcm, stats) = (pad.clone(), quit.clone(), std::path::PathBuf::from(gcm), stats.clone());
        std::thread::Builder::new().name("game".into()).stack_size(256 << 20).spawn(move || {
            raise_priority(-15);
            let story = std::env::var("COLO_SNAP").ok().map(std::path::PathBuf::from);
            let tap_a = std::env::var("COLO_TAP_A").is_ok();
            let f = match std::fs::File::open(&gcm) { Ok(f) => Mutex::new(f), Err(e) => { eprintln!("recomp: {}: {e}", gcm.display()); return; } };
            let ds = stats.clone();
            let disc: colo_recomp::run::Disc = Box::new(move |off, buf| {
                use std::io::{Read, Seek, SeekFrom};
                let t = Instant::now();
                let mut f = f.lock().unwrap();
                if f.seek(SeekFrom::Start(off)).is_err() || f.read_exact(buf).is_err() { buf.fill(0); }
                if let Some(p) = &ds {
                    p.disc_n.fetch_add(1, Ordering::Relaxed);
                    p.disc_bytes.fetch_add(buf.len() as u64, Ordering::Relaxed);
                    perf::add_max(&p.disc_ns, &p.disc_max, t.elapsed());
                }
            });
            // power-on from the disc (the game's own __start / OSInit), or a snapshot when one is named
            let m = match story.as_ref().or(snap.as_ref()) {
                Some(d) => Machine::from_snapshot(d).map(|mut m| { m.set_disc(disc); m }).map_err(|e| format!("{}: {e}", d.display())),
                None => Machine::from_disc(disc).map_err(|e| format!("{}: {e}", gcm.display())),
            };
            let mut m = match m { Ok(m) => m, Err(e) => { eprintln!("recomp: {e}"); return; } };
            let card = if story.is_some() { None } else { card };
            if story.is_none() { match &card {
                Some(p) => if let Some(img) = open_card(&m, p) { m.insert_card_image(img) },
                None => {
                    let gcis: Vec<Vec<u8>> = gcis.iter().filter_map(|p| std::fs::read(p).ok()).collect();
                    if !gcis.is_empty() { m.insert_card(&gcis); }
                }
            } }
            if story.is_none() { m.boot(); }
            let mut b = colo_gx::cmd::Builder::new();
            // paced by the game's own clock (time base, 40.5 MHz): 60 or 30 updates a second, whichever the
            // game runs, and the AX mix comes out at its real rate
            const TB_HZ: f64 = 40_500_000.0;
            let mut t0 = (std::time::Instant::now(), colo_recomp::run::time_base(&m.c));
            let mut n = 0u32;
            let lead = lead;
            while !quit.load(Ordering::Relaxed) {
                let mut p = *pad.lock().unwrap();
                if tap_a && n % 20 >= 10 && n % 20 < 13 { p.buttons |= 0x100; }
                m.set_pad(p);
                let (t, tb0) = (Instant::now(), colo_recomp::run::time_base(&m.c));
                m.frame();
                let t_cpu = Instant::now();
                n += 1;
                if n % 30 == 0 { if let Some(p) = &card { save_card(&mut m, p); } }
                let g = m.gpu_cmds_deferred(&mut b);
                let t_build = Instant::now();
                let mut audio = colo_recomp::run::take_audio();
                for p in audio.chunks_mut(2) { p.swap(0, 1); }
                crate::audio::push_external(&audio);
                // `next`: the real time this frame's game time ends at (the next frame starts then); shown `lead` later
                let tb = colo_recomp::run::time_base(&m.c);
                let next = t0.0 + Duration::from_secs_f64(tb.saturating_sub(t0.1) as f64 / TB_HZ);
                // the render thread may be a little behind (two frames queued): wait for it while the game is ahead of
                // its time (until `next`, when the next frame starts anyway). Past that, presenting is what's slow (the
                // window hidden or the screen locked: the compositor takes frames slowly or not at all), and the frame
                // is dropped so the game keeps its time and sound. (A dropped frame's EFB copies are lost too; the game
                // makes most of them every frame.)
                let (mut dropped, mut gone) = (false, false);
                let mut item = Some((g, next + lead));
                let give_up = next;
                while let Some(it) = item.take() {
                    match tx.try_send(it) {
                        Ok(()) => {}
                        Err(std::sync::mpsc::TrySendError::Full(it)) if Instant::now() < give_up => {
                            item = Some(it);
                            std::thread::sleep(Duration::from_micros(500));
                        }
                        Err(std::sync::mpsc::TrySendError::Full(_)) => dropped = true,
                        Err(std::sync::mpsc::TrySendError::Disconnected(_)) => gone = true,
                    }
                }
                if gone { break; }
                let t_sent = Instant::now();
                let now = Instant::now();
                let mut resync = false;
                if next > now { std::thread::sleep(next - now); }
                else if now - next > std::time::Duration::from_millis(150) { t0 = (now, tb); resync = true; }
                if let Some(p) = &stats {
                    p.frames.fetch_add(1, Ordering::Relaxed);
                    perf::add_max(&p.cpu_ns, &p.cpu_max, t_cpu - t);
                    perf::add(&p.build_ns, t_build - t_cpu);
                    perf::add(&p.wait_ns, t_sent - t_build);
                    perf::add(&p.idle_ns, now.elapsed());
                    p.tb.fetch_add(tb.saturating_sub(tb0), Ordering::Relaxed);
                    if resync { p.resync.fetch_add(1, Ordering::Relaxed); }
                    if dropped { p.dropped.fetch_add(1, Ordering::Relaxed); }
                }
            }
            if let Some(p) = &card { save_card(&mut m, p); }
        }).expect("game thread")
    };
    raise_priority(-10); // the window thread
    let el = EventLoop::new().expect("event loop");
    el.set_control_flow(ControlFlow::Poll);
    let perf = stats.map(|p| (p, perf::Ui::new(opts), perf::Overlay::new()));
    let mut app = App { rx: Some(rx), shown_rx: None, render: None, permit: None, pending: Default::default(), cur: None,
        pad, quit, keys: HashSet::new(), gilrs: gilrs::Gilrs::new().ok(), gfx: None, perf };
    el.run_app(&mut app).ok();
    let render = app.render.take();
    drop(app); // unblocks the render thread's send (and so the game thread's) so the game can save the card and stop
    if let Some(r) = render { r.join().ok(); }
    game.join().ok();
}

/// Ask rtkit (the desktop's realtime/priority broker, no root needed; it allows nice down to -15) to raise the
/// calling thread's priority, so compiles and other desktop work don't take its CPU time: Claude-started builds run at
/// nice -8 and starved the game thread to 72-90% speed. Silently does nothing where rtkit isn't there.
fn raise_priority(nice: i32) {
    #[cfg(target_os = "linux")]
    {
        let Ok(t) = std::fs::read_link("/proc/thread-self") else { return };
        let tid = t.file_name().and_then(|n| n.to_str()).unwrap_or("0").to_string();
        let ok = std::process::Command::new("busctl").args(["call", "--", "org.freedesktop.RealtimeKit1", "/org/freedesktop/RealtimeKit1",
            "org.freedesktop.RealtimeKit1", "MakeThreadHighPriorityWithPID", "tti", &std::process::id().to_string(), &tid, &nice.to_string()])
            .stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).status().is_ok_and(|s| s.success());
        if !ok { eprintln!("recomp: couldn't raise thread {tid}'s priority (rtkit)"); }
    }
    #[cfg(not(target_os = "linux"))]
    let _ = nice;
}

/// Default card image: Working/saves (beside the Dolphin harness, not in it).
const CARD: &str = "saves/MemoryCardA.USA.raw";

/// The card image at `p`, or a blank card formatted for this console, written there, if there's none.
fn open_card(m: &Machine, p: &std::path::Path) -> Option<Vec<u8>> {
    match std::fs::read(p) {
        Ok(img) if img.len() == m.blank_card().len() => Some(img),
        Ok(img) => { eprintln!("recomp: {}: {} bytes isn't a 4 Mbit card image; card not inserted", p.display(), img.len()); None }
        Err(_) => {
            let img = m.blank_card();
            if let Some(d) = p.parent() { std::fs::create_dir_all(d).ok(); }
            match std::fs::write(p, &img) {
                Ok(()) => eprintln!("recomp: made a blank memory card at {}", p.display()),
                Err(e) => eprintln!("recomp: {}: {e} (the card's writes won't be kept)", p.display()),
            }
            Some(img)
        }
    }
}

/// Write the card image back to `p` if the game wrote to it (whole file, through a temporary, so a crash mid-write
/// leaves the last good image).
fn save_card(m: &mut Machine, p: &std::path::Path) {
    let Some(img) = m.take_card_writes() else { return };
    let tmp = p.with_extension("raw.tmp");
    if let Err(e) = std::fs::write(&tmp, &img).and_then(|_| std::fs::rename(&tmp, p)) { eprintln!("recomp: saving {}: {e}", p.display()); }
}

struct Gfx {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    blit: colo_gxgpu::Blit,
    // clones (the render thread's Gpu owns the originals)
    device: wgpu::Device,
    queue: wgpu::Queue,
    scale: u32,
    adapter: String,
}

/// A rendered frame: its XFB (each copy is its own texture) and when it is due on screen.
struct Shown { xfb: Option<wgpu::TextureView>, due: Instant }

struct App {
    /// Game frames, until the render thread takes them at `resumed`.
    rx: Option<Receiver<(colo_gx::cmd::GFrame, Instant)>>,
    shown_rx: Option<Receiver<Shown>>,
    /// One permit per frame shown lets the render thread submit the next frame's render (see the top).
    permit: Option<std::sync::mpsc::Sender<()>>,
    render: Option<std::thread::JoinHandle<()>>,
    /// Rendered frames not yet due, oldest first, and the one on screen.
    pending: std::collections::VecDeque<Shown>,
    cur: Option<Shown>,

    pad: Arc<Mutex<Pad>>,
    quit: Arc<AtomicBool>,
    keys: HashSet<KeyCode>,
    gilrs: Option<gilrs::Gilrs>,
    gfx: Option<Gfx>,
    perf: Option<(Arc<Perf>, perf::Ui, perf::Overlay)>,
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
        let modes = surface.get_capabilities(&adapter).present_modes;
        let want = match std::env::var("COLO_PRESENT").as_deref() {
            Ok("mailbox") => wgpu::PresentMode::Mailbox, Ok("immediate") => wgpu::PresentMode::Immediate, _ => wgpu::PresentMode::Fifo };
        config.present_mode = if modes.contains(&want) { want } else { wgpu::PresentMode::Fifo };
        if let Some(n) = std::env::var("COLO_FRAME_LATENCY").ok().and_then(|v| v.parse().ok()) { config.desired_maximum_frame_latency = n; }
        // the XFB holds final display values: an sRGB surface would encode them a second time (too bright)
        config.format = config.format.remove_srgb_suffix();
        surface.configure(&device, &config);
        let scale = std::env::var("COLO_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(3);
        let info = adapter.get_info();
        let adapter = format!("{} ({:?})", info.name, info.backend);
        let gpu = colo_gxgpu::Gpu::new(device.clone(), queue.clone(), scale);
        // the render thread: vertex work and GPU encode off the window thread, so presenting keeps to the refresh;
        // vertex work as soon as a frame arrives, the render once permitted (see the top)
        let (stx, srx) = channel::<Shown>();
        let (ptx, prx) = channel::<()>();
        ptx.send(()).ok();
        let rx = self.rx.take().expect("game frames");
        let (rq, stats) = (queue.clone(), self.perf.as_ref().map(|p| p.0.clone()));
        self.render = Some(std::thread::Builder::new().name("render".into()).spawn(move || {
            raise_priority(-10);
            let mut gpu = gpu;
            while let Ok((mut f, due)) = rx.recv() {
                let t = Instant::now();
                f.finish();
                let t_fin = Instant::now();
                if prx.recv().is_err() { break; }
                let t_go = Instant::now();
                gpu.render(&f);
                if let Some(p) = &stats {
                    perf::add(&p.finish_ns, t_fin - t);
                    perf::add(&p.render_ns, t_go.elapsed());
                    p.rendered.fetch_add(1, Ordering::Relaxed);
                    let p = p.clone();
                    rq.on_submitted_work_done(move || { p.gpu_n.fetch_add(1, Ordering::Relaxed); perf::add_max(&p.gpu_ns, &p.gpu_max, t_go.elapsed()); });
                }
                if stx.send(Shown { xfb: gpu.xfb.as_ref().map(|x| x.0.clone()), due }).is_err() { break; }
            }
        }).expect("render thread"));
        self.shown_rx = Some(srx);
        self.permit = Some(ptx);
        self.gfx = Some(Gfx { window, surface, config, blit: colo_gxgpu::Blit::default(), device, queue, scale, adapter });
        self.perf_info();
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, ev: WindowEvent) {
        match ev {
            WindowEvent::CloseRequested => { self.quit.store(true, Ordering::Relaxed); el.exit(); }
            WindowEvent::Resized(sz) => if let Some(g) = self.gfx.as_mut() {
                g.config.width = sz.width.max(1); g.config.height = sz.height.max(1);
                g.surface.configure(&g.device, &g.config);
                self.perf_info();
            },
            WindowEvent::KeyboardInput { event, .. } if event.state == ElementState::Pressed && !event.repeat
                && event.physical_key == PhysicalKey::Code(KeyCode::F3) && self.perf.is_some() => {
                if let Some((_, ui, _)) = self.perf.as_mut() { ui.opts.overlay = !ui.opts.overlay; }
            }
            WindowEvent::KeyboardInput { event, .. } => if let PhysicalKey::Code(k) = event.physical_key {
                if event.state == ElementState::Pressed { self.keys.insert(k); } else { self.keys.remove(&k); }
            },
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _el: &ActiveEventLoop) {
        *self.pad.lock().unwrap() = self.read_pad();
        let Some(g) = self.gfx.as_mut() else { std::thread::sleep(Duration::from_millis(1)); return };
        if let Some(rx) = &self.shown_rx { while let Ok(s) = rx.try_recv() { self.pending.push_back(s); } }
        let now = Instant::now();
        // the newest frame that is due
        let mut n = 0;
        while self.pending.front().is_some_and(|s| s.due <= now) { self.cur = self.pending.pop_front(); n += 1; }
        if let Some((p, ui, ov)) = self.perf.as_mut() {
            if n > 0 { ui.shown(now, n - 1, now > self.cur.as_ref().unwrap().due + Duration::from_millis(2)); }
            g.device.poll(wgpu::PollType::Poll).ok();
            if ui.tick(p) && ui.opts.overlay {
                ov.prepare(&g.device, g.config.format);
                ov.set_text(&g.device, &g.queue, &ui.lines, (15.0 * g.window.scale_factor()) as f32);
            }
        }
        if n == 0 {
            // until the next frame is due (input is read every couple of ms meanwhile)
            let wait = self.pending.front().map_or(Duration::from_millis(1), |s| s.due.saturating_duration_since(now));
            std::thread::sleep(wait.min(Duration::from_millis(2)));
            return;
        }
        let frame = match g.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(f) | wgpu::CurrentSurfaceTexture::Suboptimal(f) => Some(f),
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => { g.surface.configure(&g.device, &g.config); None }
            _ => None,
        };
        if let Some(frame) = frame {
            let view = frame.texture.create_view(&Default::default());
            let t = Instant::now();
            g.blit.draw(&g.device, &g.queue, self.cur.as_ref().and_then(|s| s.xfb.as_ref()), &view, g.config.format, g.config.width, g.config.height);
            if let Some((_, ui, ov)) = self.perf.as_mut() {
                if ui.opts.overlay { ov.draw(&g.device, &g.queue, &view, g.config.width, g.config.height); }
                g.window.pre_present_notify();
                frame.present();
                ui.present += t.elapsed();
                ui.presented();
            } else { g.window.pre_present_notify(); frame.present(); }
        }
        // this frame's present is queued: the next render may go behind it
        if let Some(p) = &self.permit { for _ in 0..n { p.send(()).ok(); } }
    }
}

impl App {
    /// The perf text's fixed line: resolution, window and card.
    fn perf_info(&mut self) {
        let (Some((_, ui, _)), Some(g)) = (self.perf.as_mut(), &self.gfx) else { return };
        ui.info = format!("scale {}x ({}x{} EFB)  window {}x{} {:?}  {}", g.scale, 640 * g.scale, 528 * g.scale,
            g.config.width, g.config.height, g.config.present_mode, g.adapter);
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
