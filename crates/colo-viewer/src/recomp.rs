//! `colo-viewer <gcm> recomp`: the whole game on its own recompiled code (colo-battle `run.rs`), drawn on the
//! graphics card (colo-gx `cmd` draw lists -> colo-gxgpu) at the settings' resolution scale (default 3) times the
//! GameCube's resolution, from power-on: the disc image alone (`Machine::from_disc`: the DOL's `__start` and OSInit run on the
//! lifted code; no Dolphin data), or with env COLO_BOOT_SNAP=<dir> Dolphin's `boot_snap.py` snapshot at `main`; and
//! memory card A. The card is a raw image (Dolphin's .raw layout) kept on disk: env
//! COLO_CARD names it (default `save::working_dir()`/saves/MemoryCardA.USA.raw); when the file doesn't exist a blank card formatted
//! for this console is made there, and the game's writes go back to it. COLO_CARD as comma-separated .gci files
//! instead builds a card from those saves for this run only (writes are not kept). COLO_CARD_B=<raw image> puts a
//! second card in slot B the same way (made blank if missing, writes kept); without it slot B is empty.
//!
//! The game runs on its own thread, paced by its own time base (real time); frames go to a render thread
//! (`GFrame::finish` + `Gpu::render`) and then to the window (winit + wgpu), which presents each one at its due time,
//! and its AX mix to the audio output (audio.rs, buffered and resampled to the device).
//! Escape pauses the game and opens the settings menu (recomp_menu.rs: tabs for video, audio, controls with key
//! rebinding, resume and quit; keyboard or mouse; saved in saves/recomp.ini, settings.rs, and applied at once).
//! Pacing: a frame is due the frame lead (setting, default 34 ms) after the real time its game time ends at, so a
//! frame may cost up to about one frame more than its budget without a visible hitch. Due times follow the game's
//! clock, so they are evenly spaced; FIFO (the default presentation) shows each at the next refresh,
//! which holds a 30 updates/s game's frames for an even number of refreshes (2 at 60 Hz; 5/6 alternating at 165 Hz,
//! which 30 doesn't divide). One present per frame: the GPU runs one queue, so a present behind a render waits for it
//! (presenting every refresh lost ~1 refresh in 4 at 165 Hz); and the render thread submits a frame's render only
//! after the previous frame's present (a permit per frame shown), so a render never delays a present. The game
//! thread never waits on the window past its own schedule: if presenting falls behind (window hidden, screen locked)
//! it drops frames and keeps its time and sound. Threads ask rtkit for priority (`raise_priority`). Keyboard
//! (controller 1, rebindable): Space/X A, Backspace/C B, V X, F Y, Enter/Tab Start, arrows d-pad, WASD stick (Shift
//! half tilt), IJKL C stick, Z/Q/E Z/L/R.
//! Gamepads (gilrs) in the order they connected: the first is controller 1 too, the next three controllers 2-4
//! (sticks, C stick, analog triggers); a port with no gamepad is empty. The game's rumble (PADControlMotor) drives
//! the gamepad's motor.
//! The performance overlay (setting, or `recomp --perf`; F3 toggles) shows frame rate and frame costs,
//! `--perf-log[=secs]` prints them (perf.rs).
//! Testing: env COLO_SNAP=<dir> runs a story snapshot (Working/dolphin/snaps/storyNN, as the `storybattle` example)
//! instead of booting, without the card; COLO_TAP_A=1 adds storybattle's A taps (3 frames in 20) to the pad.
//! COLO_TEST_KEYS="secs:KeyName,..." presses keys and COLO_TEST_MOUSE="secs:x:y:move|left|right|drag|up|down,..."
//! works the mouse at those times after start (`about_to_wait`).
//!
//! `colo-viewer <gcm> <room>[:entry]` boots the same way and goes straight to that room (warp.rs).

use crate::perf::{self, Perf};
use crate::recomp_menu::{Menu, Outcome};
use crate::settings::{Action, Settings};
use colo_recomp::run::{Machine, Pad};
use std::time::{Duration, Instant};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, sync_channel, Receiver};
use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

pub fn run(gcm: &str, opts: perf::Opts, warp: Option<(u32, u32)>) {
    let snap = std::env::var("COLO_BOOT_SNAP").ok().map(std::path::PathBuf::from);
    let card = std::env::var("COLO_CARD").unwrap_or_else(|_| crate::save::working_dir().join(CARD).to_string_lossy().into_owned());
    let (gcis, card): (Vec<std::path::PathBuf>, _) = if card.split(',').all(|p| p.to_ascii_lowercase().ends_with(".gci")) {
        (card.split(',').map(std::path::PathBuf::from).collect(), None)
    } else { (Vec::new(), Some(std::path::PathBuf::from(card))) };
    let card_b = std::env::var("COLO_CARD_B").ok().map(std::path::PathBuf::from);
    let pad = Arc::new(Mutex::new([Some(Pad::default()), None, None, None]));
    let motors = Arc::new(Mutex::new([0u8; 4]));
    let quit = Arc::new(AtomicBool::new(false));
    let paused = Arc::new(AtomicBool::new(false));
    let settings = Settings::load();
    let shared = Arc::new(Mutex::new(settings.clone()));
    let (tx, rx) = sync_channel::<(colo_gx::cmd::GFrame, Instant)>(2);
    let stats = Some(Arc::new(Perf::default()));
    crate::audio::start_external();
    crate::audio::set_volume(settings.volume);
    let game = {
        let (pad, motors, quit, gcm, stats) = (pad.clone(), motors.clone(), quit.clone(), std::path::PathBuf::from(gcm), stats.clone());
        let (paused, shared) = (paused.clone(), shared.clone());
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
                Some(p) => if let Some(img) = open_card(&m, 0, p) { m.insert_card_image(img) },
                None => {
                    let gcis: Vec<Vec<u8>> = gcis.iter().filter_map(|p| std::fs::read(p).ok()).collect();
                    if !gcis.is_empty() { m.insert_card(&gcis); }
                }
            } }
            let card_b = if story.is_some() { None } else { card_b };
            if let Some(p) = &card_b { if let Some(img) = open_card(&m, 1, p) { m.insert_card_image_in(1, img) } }
            if story.is_none() { m.boot(); }
            if let Some(w) = warp.filter(|_| story.is_none()) {
                crate::warp::install(&mut m, w);
            }
            let mut b = colo_gx::cmd::Builder::new();
            // paced by the game's own clock (time base, 40.5 MHz): 60 or 30 updates a second, whichever the
            // game runs, and the AX mix comes out at its real rate
            const TB_HZ: f64 = 40_500_000.0;
            let mut t0 = (std::time::Instant::now(), colo_recomp::run::time_base(&m.c));
            let mut n = 0u32;
            // textures new in a dropped frame: the builder sends each texture once, so they ride on the next frame
            let mut carry: Vec<(u64, colo_gx::cmd::GTex)> = Vec::new();
            while !quit.load(Ordering::Relaxed) {
                // the settings menu pauses the game; its clock picks up where it stopped
                if paused.load(Ordering::Relaxed) {
                    while paused.load(Ordering::Relaxed) && !quit.load(Ordering::Relaxed) { std::thread::sleep(Duration::from_millis(5)); }
                    t0 = (Instant::now(), colo_recomp::run::time_base(&m.c));
                    continue;
                }
                let lead = Duration::from_millis(shared.lock().unwrap().frame_lead_ms as u64);
                let mut p = *pad.lock().unwrap();
                if tap_a && n % 20 >= 10 && n % 20 < 13 { if let Some(p) = &mut p[0] { p.buttons |= 0x100; } }
                // warp: Start, then A, until the title menu starts (the hook takes WARP)
                if crate::warp::pending() {
                    if let Some(p) = &mut p[0] { p.buttons |= crate::warp::buttons(n); }
                }
                m.set_pads(p);
                let (t, tb0) = (Instant::now(), colo_recomp::run::time_base(&m.c));
                m.frame();
                let t_cpu = Instant::now();
                n += 1;
                *motors.lock().unwrap() = m.motors();
                if n % 30 == 0 {
                    if let Some(p) = &card { save_card(&mut m, 0, p); }
                    if let Some(p) = &card_b { save_card(&mut m, 1, p); }
                }
                let mut g = m.gpu_cmds_deferred(&mut b);
                if !carry.is_empty() { carry.append(&mut g.new_tex); g.new_tex = std::mem::take(&mut carry); }
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
                // makes most of them every frame; its new textures are kept for the next one.)
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
                        Err(std::sync::mpsc::TrySendError::Full(it)) => { dropped = true; carry = it.0.new_tex; }
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
            if let Some(p) = &card { save_card(&mut m, 0, p); }
            if let Some(p) = &card_b { save_card(&mut m, 1, p); }
        }).expect("game thread")
    };
    raise_priority(-10); // the window thread
    let el = EventLoop::new().expect("event loop");
    el.set_control_flow(ControlFlow::Poll);
    let mut opts = opts;
    opts.overlay |= settings.perf_overlay;
    let perf = stats.map(|p| (p, perf::Ui::new(opts), perf::Overlay::new()));
    let mut app = App { rx: Some(rx), shown_rx: None, render: None, permit: None, pending: Default::default(), cur: None,
        pad, motors, rumble: Default::default(), quit, keys: HashSet::new(), gilrs: gilrs::Gilrs::new().ok(), gfx: None, perf,
        settings, shared, paused, menu: None, menu_ov: perf::Overlay::new(), redraw: false, cursor: (0, 0), wheel: 0.0, dragging: false,
        test_mouse: std::env::var("COLO_TEST_MOUSE").ok().map(|v| (Instant::now(), v.split(',').filter_map(|e| {
            let mut f = e.split(':');
            Some((f.next()?.parse().ok()?, f.next()?.parse().ok()?, f.next()?.parse().ok()?, f.next().unwrap_or("move").to_string())) }).collect())),
        test_keys: std::env::var("COLO_TEST_KEYS").ok().map(|v| (Instant::now(), v.split(',').filter_map(|e| {
            let (t, k) = e.split_once(':')?; Some((t.parse().ok()?, crate::settings::key_from_name(k)?)) }).collect())) };
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

/// A gamepad as a GameCube controller: face buttons by position (south A, east B, west X, north Y), d-pad,
/// Start, right shoulder Z, analog triggers L/R (digital at full press), left stick, right stick as the C stick.
fn gamepad_pad(gp: &gilrs::Gamepad<'_>, deadzone: f32) -> Pad {
    use gilrs::{Axis, Button};
    let mut b = 0u16;
    let mut g = |btn: Button, bit: u16| if gp.is_pressed(btn) { b |= bit; };
    g(Button::South, 0x100); g(Button::East, 0x200); g(Button::West, 0x400); g(Button::North, 0x800); g(Button::Start, 0x1000);
    g(Button::DPadLeft, 0x1); g(Button::DPadRight, 0x2); g(Button::DPadDown, 0x4); g(Button::DPadUp, 0x8);
    g(Button::RightTrigger, 0x10); g(Button::RightTrigger2, 0x20); g(Button::LeftTrigger2, 0x40);
    let st = |ax: Axis, ay: Axis| {
        let (x, y) = (gp.value(ax), gp.value(ay));
        if x.abs() > deadzone || y.abs() > deadzone { ((x * 100.0).clamp(-128.0, 127.0) as i8, (y * 100.0).clamp(-128.0, 127.0) as i8) } else { (0, 0) }
    };
    let tr = |btn: Button| (gp.button_data(btn).map_or(0.0, |d| d.value()).clamp(0.0, 1.0) * 255.0) as u8;
    Pad { buttons: b, stick: st(Axis::LeftStickX, Axis::LeftStickY), cstick: st(Axis::RightStickX, Axis::RightStickY),
          trig: (tr(Button::LeftTrigger2), tr(Button::RightTrigger2)) }
}

/// Default card image: Working/saves (beside the Dolphin harness, not in it).
const CARD: &str = "saves/MemoryCardA.USA.raw";

/// The card image at `p`, or a blank card formatted for this console's `slot`, written there, if there's none.
fn open_card(m: &Machine, slot: usize, p: &std::path::Path) -> Option<Vec<u8>> {
    match std::fs::read(p) {
        Ok(img) if img.len() == m.blank_card_in(slot).len() => Some(img),
        Ok(img) => { eprintln!("recomp: {}: {} bytes isn't a 4 Mbit card image; card not inserted", p.display(), img.len()); None }
        Err(_) => {
            let img = m.blank_card_in(slot);
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
fn save_card(m: &mut Machine, slot: usize, p: &std::path::Path) {
    let Some(img) = m.take_card_writes_in(slot) else { return };
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

    pad: Arc<Mutex<[Option<Pad>; 4]>>,
    /// The game's rumble command per port, and each gamepad's motor effect (playing or not).
    motors: Arc<Mutex<[u8; 4]>>,
    rumble: std::collections::HashMap<gilrs::GamepadId, (gilrs::ff::Effect, bool)>,
    quit: Arc<AtomicBool>,
    keys: HashSet<KeyCode>,
    gilrs: Option<gilrs::Gilrs>,
    gfx: Option<Gfx>,
    perf: Option<(Arc<Perf>, perf::Ui, perf::Overlay)>,
    /// Settings (saved on every change), the copy the game and render threads read, the game paused by the menu.
    settings: Settings,
    shared: Arc<Mutex<Settings>>,
    paused: Arc<AtomicBool>,
    /// The Escape menu while it is open, its text, and a repaint wanted (the paused game sends no frames).
    menu: Option<Menu>,
    menu_ov: perf::Overlay,
    /// The pointer in window pixels, wheel turns short of a notch.
    cursor: (i32, i32),
    wheel: f32,
    /// COLO_TEST_MOUSE: its events, a drag held.
    test_mouse: Option<(Instant, Vec<(f64, i32, i32, String)>)>,
    dragging: bool,
    redraw: bool,
    test_keys: Option<(Instant, std::collections::VecDeque<(f64, KeyCode)>)>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.gfx.is_some() { return; }
        let window = Arc::new(el.create_window(Window::default_attributes().with_title("Pokémon Colosseum (recompiled)")
            .with_inner_size(winit::dpi::LogicalSize::new(960.0, 720.0))
            .with_fullscreen(self.settings.fullscreen.then_some(winit::window::Fullscreen::Borderless(None)))).expect("window"));
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).expect("surface");
        let adapter = futures_lite::future::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false })).expect("adapter");
        let (device, queue) = futures_lite::future::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).expect("device");
        let size = window.inner_size();
        let mut config = surface.get_default_config(&adapter, size.width.max(1), size.height.max(1)).expect("surface config");
        let modes = surface.get_capabilities(&adapter).present_modes;
        let want = self.settings.present.wgpu();
        config.present_mode = if modes.contains(&want) { want } else { wgpu::PresentMode::Fifo };
        config.desired_maximum_frame_latency = self.settings.frame_latency;
        // the XFB holds final display values: an sRGB surface would encode them a second time (too bright)
        config.format = config.format.remove_srgb_suffix();
        surface.configure(&device, &config);
        let scale = self.settings.scale;
        let info = adapter.get_info();
        let adapter = format!("{} ({:?})", info.name, info.backend);
        let gpu = colo_gxgpu::Gpu::new(device.clone(), queue.clone(), scale);
        // the render thread: vertex work and GPU encode off the window thread, so presenting keeps to the refresh;
        // vertex work as soon as a frame arrives, the render once permitted (see the top)
        let (stx, srx) = channel::<Shown>();
        let (ptx, prx) = channel::<()>();
        ptx.send(()).ok();
        let rx = self.rx.take().expect("game frames");
        let (rq, rdev, stats, shared) = (queue.clone(), device.clone(), self.perf.as_ref().map(|p| p.0.clone()), self.shared.clone());
        self.render = Some(std::thread::Builder::new().name("render".into()).spawn(move || {
            raise_priority(-10);
            let mut gpu = gpu;
            while let Ok((mut f, due)) = rx.recv() {
                // the settings' resolution (a new EFB: copies kept from earlier frames are lost once) and copy options
                {
                    let s = shared.lock().unwrap();
                    if s.scale != gpu.scale { gpu = colo_gxgpu::Gpu::new(rdev.clone(), rq.clone(), s.scale); }
                    gpu.xfb_yuv = s.xfb_yuv.get(gpu.scale == 1);
                    gpu.copy_filter = s.copy_filter.get(gpu.scale == 1);
                }
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
        let mut blit = colo_gxgpu::Blit::default();
        (blit.stretch, blit.sharp) = (!self.settings.keep_aspect, !self.settings.smooth);
        self.gfx = Some(Gfx { window, surface, config, blit, device, queue, scale, adapter });
        self.perf_info();
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, ev: WindowEvent) {
        match ev {
            WindowEvent::CloseRequested => { self.quit.store(true, Ordering::Relaxed); self.paused.store(false, Ordering::Relaxed); el.exit(); }
            WindowEvent::Resized(sz) => if let Some(g) = self.gfx.as_mut() {
                g.config.width = sz.width.max(1); g.config.height = sz.height.max(1);
                g.surface.configure(&g.device, &g.config);
                self.perf_info();
                self.redraw = true;
            },
            WindowEvent::KeyboardInput { event, .. } => if let PhysicalKey::Code(k) = event.physical_key {
                self.key(el, k, event.state == ElementState::Pressed, event.repeat);
            },
            WindowEvent::CursorMoved { position, .. } => self.mouse(el, Mouse::Move(position.x as i32, position.y as i32)),
            WindowEvent::MouseInput { state, button, .. } => match button {
                MouseButton::Left | MouseButton::Right => self.mouse(el, Mouse::Button(button == MouseButton::Left, state == ElementState::Pressed)),
                _ => {}
            },
            WindowEvent::MouseWheel { delta, .. } => {
                let d = match delta { MouseScrollDelta::LineDelta(_, y) => y, MouseScrollDelta::PixelDelta(p) => p.y as f32 / 40.0 };
                self.wheel += d;
                let n = self.wheel.trunc();
                self.wheel -= n;
                if n != 0.0 { self.mouse(el, Mouse::Wheel(n as i32)); }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, el: &ActiveEventLoop) {
        // testing: env COLO_TEST_KEYS="secs:Key,..." presses (and releases) keys at those times after start
        if let Some((t0, keys)) = self.test_keys.as_mut() {
            let now = t0.elapsed().as_secs_f64();
            let due: Vec<KeyCode> = keys.iter().take_while(|(t, _)| *t <= now).map(|(_, k)| *k).collect();
            keys.drain(..due.len());
            for k in due { self.key(el, k, true, false); self.key(el, k, false, false); }
        }
        // and COLO_TEST_MOUSE="secs:x:y:what,..." moves the pointer to window pixel x, y and then does `what`: move
        // (nothing more), left / right (click), drag (left down, held until the next entry), up / down (a wheel notch)
        if let Some((t0, ev)) = self.test_mouse.as_mut() {
            let now = t0.elapsed().as_secs_f64();
            let due: Vec<(f64, i32, i32, String)> = ev.iter().take_while(|e| e.0 <= now).cloned().collect();
            ev.drain(..due.len());
            for (_, x, y, what) in due {
                if self.dragging { self.mouse(el, Mouse::Move(x, y)); self.mouse(el, Mouse::Button(true, false)); self.dragging = false; continue; }
                self.mouse(el, Mouse::Move(x, y));
                match what.as_str() {
                    "left" | "right" => { let l = what == "left"; self.mouse(el, Mouse::Button(l, true)); self.mouse(el, Mouse::Button(l, false)); }
                    "drag" => { self.mouse(el, Mouse::Button(true, true)); self.dragging = true; }
                    "up" => self.mouse(el, Mouse::Wheel(1)),
                    "down" => self.mouse(el, Mouse::Wheel(-1)),
                    _ => {}
                }
            }
        }
        *self.pad.lock().unwrap() = self.read_pads();
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
        // the menu's text, and a repaint of the paused picture under it when it changed
        let menu_paint = self.menu.is_some() && std::mem::take(&mut self.redraw);
        if menu_paint {
            let (panel, _) = self.menu.as_mut().unwrap().panel(&self.settings, g.config.width, g.config.height, g.window.scale_factor());
            self.menu_ov.prepare(&g.device, g.config.format);
            self.menu_ov.set_panel(&g.device, &g.queue, &panel);
        }
        if n == 0 && !menu_paint {
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
            if self.menu.is_some() {
                let (w, h) = self.menu_ov.size();
                let at = perf::Overlay::centre(g.config.width, g.config.height, w, h);
                self.menu_ov.draw_at(&g.device, &g.queue, &view, g.config.width, g.config.height, at);
            }
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

enum Mouse { Move(i32, i32), Button(bool, bool), Wheel(i32) }

impl App {
    /// The mouse, for the menu (the game doesn't use it).
    fn mouse(&mut self, el: &ActiveEventLoop, ev: Mouse) {
        if let Mouse::Move(x, y) = ev { self.cursor = (x, y); }
        let Some(m) = self.menu.as_mut() else { return };
        let mut s = self.settings.clone();
        let (x, y) = self.cursor;
        let out = match ev {
            Mouse::Move(..) => m.mouse_move(x, y, &mut s),
            Mouse::Button(left, pressed) => m.mouse_button(left, pressed, x, y, &mut s),
            Mouse::Wheel(d) => m.wheel(d, x, y, &mut s),
        };
        self.menu_outcome(el, out, s);
    }

    /// What a key or the mouse did in the menu.
    fn menu_outcome(&mut self, el: &ActiveEventLoop, out: Outcome, s: Settings) {
        match out {
            Outcome::Nothing => return,
            Outcome::Changed => { if s != self.settings { self.apply(s); } }
            Outcome::Close => { self.menu = None; self.paused.store(false, Ordering::Relaxed); }
            Outcome::Quit => { self.quit.store(true, Ordering::Relaxed); self.paused.store(false, Ordering::Relaxed); el.exit(); }
        }
        self.redraw = true;
    }

    /// A key went down or up: F3 (overlay), Escape (the menu), the menu's keys while it's open, else the game's.
    fn key(&mut self, el: &ActiveEventLoop, k: KeyCode, pressed: bool, repeat: bool) {
        let rebinding = self.menu.as_ref().is_some_and(|m| m.rebinding());
        if pressed && !repeat && k == KeyCode::F3 && !rebinding {
            let mut s = self.settings.clone();
            s.perf_overlay = !self.perf.as_ref().is_some_and(|p| p.1.opts.overlay);
            self.apply(s);
        } else if pressed && k == KeyCode::Escape && self.menu.is_none() {
            // open the menu: pause, and let go of every key the game saw held
            self.menu = Some(Menu::new());
            self.paused.store(true, Ordering::Relaxed);
            self.keys.clear();
            self.redraw = true;
        } else if let Some(m) = self.menu.as_mut() {
            if !pressed { return; }
            let mut s = self.settings.clone();
            let out = m.key(k, &mut s);
            self.menu_outcome(el, out, s);
        } else if pressed { self.keys.insert(k); } else { self.keys.remove(&k); }
    }

    /// Take new settings: save them, hand them to the game and render threads, and change the window, surface,
    /// picture and sound to match.
    fn apply(&mut self, s: Settings) {
        let old = std::mem::replace(&mut self.settings, s.clone());
        s.save();
        *self.shared.lock().unwrap() = s.clone();
        crate::audio::set_volume(s.volume);
        if let Some((_, ui, _)) = self.perf.as_mut() { ui.opts.overlay = s.perf_overlay; }
        if let Some(g) = self.gfx.as_mut() {
            if s.fullscreen != old.fullscreen {
                g.window.set_fullscreen(s.fullscreen.then_some(winit::window::Fullscreen::Borderless(None)));
            }
            if s.present != old.present || s.frame_latency != old.frame_latency {
                g.config.present_mode = s.present.wgpu();
                g.config.desired_maximum_frame_latency = s.frame_latency;
                g.surface.configure(&g.device, &g.config);
            }
            (g.blit.stretch, g.blit.sharp) = (!s.keep_aspect, !s.smooth);
            g.scale = s.scale;
        }
        self.perf_info();
        self.redraw = true;
    }

    /// The perf text's fixed line: resolution, window and card.
    fn perf_info(&mut self) {
        let (Some((_, ui, _)), Some(g)) = (self.perf.as_mut(), &self.gfx) else { return };
        ui.info = format!("scale {}x ({}x{} EFB)  window {}x{} {:?}  {}", g.scale, 640 * g.scale, 528 * g.scale,
            g.config.width, g.config.height, g.config.present_mode, g.adapter);
    }

    /// Controllers 1-4: the keyboard and the first gamepad on 1, the next gamepads on 2-4. Also starts or stops
    /// each gamepad's motor as the game asks.
    fn read_pads(&mut self) -> [Option<Pad>; 4] {
        // the keyboard through the bindings: buttons, the main stick (half tilt while Walk is held) and the C stick
        let mut held: Vec<Action> = Vec::new();
        for k in &self.keys { held.extend(self.settings.actions_of(*k)); }
        let on = |a: Action| held.contains(&a);
        let b = held.iter().fold(0u16, |b, a| b | a.button());
        let full = if on(Action::Walk) { 50 } else { 100 };
        let axis = |neg: Action, pos: Action| (on(pos) as i32 - on(neg) as i32) * full;
        let (x, y) = (axis(Action::StickLeft, Action::StickRight), axis(Action::StickDown, Action::StickUp));
        let c = ((on(Action::CRight) as i32 - on(Action::CLeft) as i32) * 100, (on(Action::CUp) as i32 - on(Action::CDown) as i32) * 100);
        let trig = (if on(Action::L) { 255 } else { 0 }, if on(Action::R) { 255 } else { 0 });
        let mut p1 = Pad { buttons: b, stick: (x as i8, y as i8), cstick: (c.0 as i8, c.1 as i8), trig };
        let mut pads = [None; 4];
        let motors = *self.motors.lock().unwrap();
        if let Some(gi) = self.gilrs.as_mut() {
            while gi.next_event().is_some() {}
            let mut ids: Vec<gilrs::GamepadId> = gi.gamepads().map(|(id, _)| id).collect();
            ids.sort_by_key(|id| usize::from(*id));
            self.rumble.retain(|id, _| ids.contains(id));
            for (port, &id) in ids.iter().take(4).enumerate() {
                let gp = gi.gamepad(id);
                let g = gamepad_pad(&gp, self.settings.deadzone as f32 / 100.0);
                if port == 0 {
                    p1.buttons |= g.buttons;
                    if g.stick != (0, 0) { p1.stick = g.stick; }
                    if g.cstick != (0, 0) { p1.cstick = g.cstick; }
                    p1.trig = (p1.trig.0.max(g.trig.0), p1.trig.1.max(g.trig.1));
                } else {
                    pads[port] = Some(g);
                }
                // rumble: 1 runs the motor, 0 / 2 stop it
                let want = motors[port] == 1;
                if !gp.is_ff_supported() { continue; }
                if !self.rumble.contains_key(&id) {
                    use gilrs::ff::{BaseEffect, BaseEffectType, EffectBuilder, Replay, Ticks};
                    let fx = BaseEffect { kind: BaseEffectType::Strong { magnitude: 0xC000 },
                        scheduling: Replay { play_for: Ticks::from_ms(1000), ..Default::default() }, envelope: Default::default() };
                    match EffectBuilder::new().add_effect(fx).gamepads(&[id]).finish(gi) {
                        Ok(e) => { self.rumble.insert(id, (e, false)); }
                        Err(_) => continue,
                    }
                }
                let (e, on) = self.rumble.get_mut(&id).unwrap();
                if want != *on { *on = want; if want { e.play().ok(); } else { e.stop().ok(); } }
            }
        }
        pads[0] = Some(p1);
        pads
    }
}
