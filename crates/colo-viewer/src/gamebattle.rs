//! Battles run on the game's own code (colo-battle `run.rs` + colo-gx): when a battle's frame-boundary
//! snapshot exists (`Working/dolphin/snaps/story<encounter>`, from `story_entry_snap.py`), the battle phase
//! boots it on a worker thread, feeds it the pad at 30 Hz, shows the game's TV image full screen and plays its
//! audio; when the battle floor pops in the game, its `battleGetResult` goes back to the field script.
//!
//! The snapshot carries the Dolphin save's party and story state, not the viewer's (decision pending).
//! The viewer runs its own wipe up to the snapshot point (`fightMainThread`'s first run) with the snapshot's
//! random picks, then hands its wipe image over as the game's EFB (the wipe's next back-buffer capture copies it).

use bevy::prelude::*;
use colo_recomp::run::{Machine, Pad};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Out {
    pub rgba: Vec<u8>,
    pub audio: Vec<i16>,
    /// `battleGetResult` once the battle floor has popped.
    pub done: Option<u32>,
}

pub struct GameBattle {
    tx: Sender<Pad>,
    rx: Receiver<Out>,
    pub result: Option<u32>,
    pub frame: Option<Vec<u8>>,
    /// Game frames requested and not yet returned.
    in_flight: u32,
    acc: f32,
}

/// A graphics-card renderer with no window (the battle's frames are read back into the viewer's screen image).
fn headless_gpu() -> Option<colo_gxgpu::Gpu> {
    let instance = wgpu::Instance::default();
    let adapter = futures_lite::future::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: None, force_fallback_adapter: false })).ok()?;
    let (device, queue) = futures_lite::future::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).ok()?;
    let scale = std::env::var("COLO_BATTLE_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1);
    Some(colo_gxgpu::Gpu::new(device, queue, scale))
}

/// The XFB read back (any size) as the 640x480 RGBA screen image: box-averaged down, or nearest up.
fn to_640x480(px: &[u8], w: u32, h: u32) -> Vec<u8> {
    let (w, h) = (w as usize, h as usize);
    if (w, h) == (640, 480) { return px.to_vec(); }
    let mut out = vec![0u8; 640 * 480 * 4];
    for y in 0..480 {
        let (y0, y1) = (y * h / 480, ((y + 1) * h / 480).max(y * h / 480 + 1).min(h));
        for x in 0..640 {
            let (x0, x1) = (x * w / 640, ((x + 1) * w / 640).max(x * w / 640 + 1).min(w));
            let mut acc = [0u32; 4];
            for sy in y0..y1 { for sx in x0..x1 { let i = (sy * w + sx) * 4; for c in 0..4 { acc[c] += px[i + c] as u32; } } }
            let n = ((y1 - y0) * (x1 - x0)) as u32;
            let o = (y * 640 + x) * 4;
            for c in 0..4 { out[o + c] = (acc[c] / n) as u8; }
        }
    }
    out
}

/// Where the snapshot for `encounter` lives, if it does.
pub fn snapshot_dir(encounter: u32) -> Option<std::path::PathBuf> {
    let base = std::env::var("COLO_SNAPS").map(std::path::PathBuf::from).unwrap_or_else(|_| {
        std::path::PathBuf::from("/media/zach/massstorage/Pokemon Colosseum/Working/dolphin/snaps")
    });
    let d = base.join(format!("story{encounter}"));
    (std::env::var("COLO_NO_GAME_BATTLE").is_err() && d.join("before.bin").exists()).then_some(d)
}

/// `fadeState` 0x80466E30 (+0xC hook) and `FadeCameraWork` 0x80467030 (+0xC UDLR direction).
const FADE_STATE: u32 = 0x8046_6E30;
const FADE_WORK: u32 = 0x8046_7030;

fn snap_u32(f: &mut std::fs::File, a: u32) -> Option<u32> {
    use std::io::{Read, Seek, SeekFrom};
    let mut b = [0u8; 4];
    f.seek(SeekFrom::Start((a - 0x8000_0000) as u64)).ok()?;
    f.read_exact(&mut b).ok()?;
    Some(u32::from_be_bytes(b))
}

/// The snapshot's wipe `hookElapsed` (`fadeState` +0x18, seconds): the viewer hands over when its own wipe gets there.
pub fn wipe_elapsed(dir: &std::path::Path) -> f32 {
    std::fs::File::open(dir.join("before.bin")).ok().and_then(|mut f| snap_u32(&mut f, FADE_STATE + 0x18)).map_or(0.0, f32::from_bits)
}

/// The `_fadeEffectGetRandom` results the snapshot's wipe was set up with (the wipe kind for the random inits
/// 0x801C499C / 0x801C4904, then the UDLR flip), so the viewer's wipe before the snapshot point drags the same way.
pub fn wipe_picks(dir: &std::path::Path, init: u32) -> Vec<u32> {
    let Ok(mut f) = std::fs::File::open(dir.join("before.bin")) else { return vec![] };
    let (Some(hook), Some(w)) = (snap_u32(&mut f, FADE_STATE + 0xC), snap_u32(&mut f, FADE_WORK + 0xC)) else { return vec![] };
    let mut p = vec![];
    if matches!(init, 0x801C_499C | 0x801C_4904) {
        p.push(match hook { 0x801C_5F6C => 0, 0x801C_5ED0 => 1, _ => 2 });
    }
    // UDLR first init: a 0 flips the direction (yoko 2 -> 1, tate 8 -> 4)
    if matches!(hook, 0x801C_5F6C | 0x801C_5ED0) { p.push(if matches!(w >> 16, 1 | 4) { 0 } else { 1 }); }
    p
}

impl GameBattle {
    /// The whole game from power-on: the `boot_snap.py` snapshot (MEM1 at `main`), memory card A from GCIs.
    pub fn boot(dir: std::path::PathBuf, gcm: std::path::PathBuf, cards: Vec<std::path::PathBuf>) -> GameBattle {
        Self::spawn(dir, gcm, Some(cards), None)
    }

    /// A story battle snapshot; `wipe` = the viewer's wipe image at the snapshot point.
    pub fn start(dir: std::path::PathBuf, gcm: std::path::PathBuf, wipe: Option<Vec<u8>>) -> GameBattle { Self::spawn(dir, gcm, None, wipe) }

    fn spawn(dir: std::path::PathBuf, gcm: std::path::PathBuf, boot: Option<Vec<std::path::PathBuf>>, wipe: Option<Vec<u8>>) -> GameBattle {
        let (tx, prx) = channel::<Pad>();
        let (otx, rx) = channel::<Out>();
        std::thread::Builder::new().name("game-battle".into()).stack_size(256 << 20).spawn(move || {
            let mut m = match Machine::from_snapshot(&dir) {
                Ok(m) => m,
                Err(e) => { eprintln!("game battle: {}: {e}", dir.display()); return; }
            };
            // the snapshot was taken over another room: the viewer's wipe image becomes the fade texture (a ball wipe
            // draws it as it is) and the EFB (a UDLR/trainer wipe's first capture copies it over the texture)
            if let Some(w) = wipe.as_deref().filter(|w| w.len() >= 640 * 480 * 4) { m.set_fade_texture(w); m.seed_efb(w); }
            let f = std::sync::Mutex::new(std::fs::File::open(&gcm).expect("disc image"));
            m.set_disc(Box::new(move |off, buf| {
                use std::io::{Read, Seek, SeekFrom};
                let mut f = f.lock().unwrap();
                if f.seek(SeekFrom::Start(off)).is_err() || f.read_exact(buf).is_err() { buf.fill(0); }
            }));
            if let Some(cards) = &boot {
                let gcis: Vec<Vec<u8>> = cards.iter().filter_map(|p| std::fs::read(p).ok()).collect();
                if !gcis.is_empty() { m.insert_card(&gcis); }
                m.boot();
            }
            let boot_mode = boot.is_some();
            let mut was = m.floor_depth();
            let mut first = !boot_mode;
            // frames are drawn on the graphics card (colo-gxgpu, as `recomp` mode does) once the battle wipe's hook is
            // gone: the software renderer (~25 ms a battle frame on top of the game's ~22 ms) can't keep 29.97 Hz, so
            // the battle lagged and its audio ran slow and stuttered. The draw list is built here (it reads textures
            // from RAM before the next frame changes them); a render thread draws it and reads the XFB back, so the
            // game thread only pays for the game frame and the build. The wipe stays on the software renderer, whose
            // EFB the viewer's wipe image was seeded into. COLO_BATTLE_SOFT=1 keeps software throughout.
            let gpu = if std::env::var("COLO_BATTLE_SOFT").is_ok() { None } else { headless_gpu() };
            let (rtx, rrx) = channel::<(Option<colo_gx::cmd::GFrame>, Out)>();
            let rotx = otx.clone();
            // every frame goes through the render thread once it exists (software ones pass through), in order
            let render = gpu.map(|mut g| std::thread::Builder::new().name("game-battle-gpu".into()).spawn(move || {
                while let Ok((f, mut out)) = rrx.recv() {
                    if let Some(f) = f {
                        g.render(&f);
                        if let Some((px, w, h)) = g.read_xfb() { out.rgba = to_640x480(&px, w, h); }
                    }
                    let done = out.done.is_some();
                    if rotx.send(out).is_err() || done { break; }
                }
            }).expect("game battle render thread"));
            let mut builder = colo_gx::cmd::Builder::new();
            let mut on_gpu = false;
            // COLO_GB_TIME=1: mean game-thread time (game frame + draw list) per frame every 60 frames (budget 33.4 ms)
            let timing = std::env::var("COLO_GB_TIME").is_ok();
            let (mut t_sum, mut t_n) = (0.0f64, 0u32);
            while let Ok(pad) = prx.recv() {
                let t0 = std::time::Instant::now();
                m.set_pad(pad);
                m.frame();
                if !on_gpu && render.is_some() && m.c.m.u32(FADE_STATE + 0xC) == 0 { on_gpu = true; }
                let list = if on_gpu { Some(m.gpu_cmds(&mut builder)) } else { m.gpu_frame(); None };
                // the first frame shows the XFB Dolphin rendered before the snapshot (the VI shows the previous
                // copy): keep the viewer's wipe on screen for it (an empty image)
                let soft = !std::mem::take(&mut first) && list.is_none();
                let rgba = if soft { m.display(480) } else { Vec::new() };
                // the AX mix is R, L per frame; the output wants L, R
                let mut audio = colo_recomp::run::take_audio();
                for p in audio.chunks_mut(2) { p.swap(0, 1); }
                let d = m.floor_depth();
                let done = if !boot_mode && was == 1 && d == 0 { Some(m.battle_result()) } else { None };
                was = d;
                if timing {
                    t_sum += t0.elapsed().as_secs_f64(); t_n += 1;
                    if t_n == 60 { println!("game battle: {:.1} ms/frame ({})", t_sum / 60.0 * 1000.0, if on_gpu { "gpu" } else { "software" }); (t_sum, t_n) = (0.0, 0); }
                }
                let out = Out { rgba, audio, done };
                let sent = if render.is_some() { rtx.send((list, out)).is_ok() } else { otx.send(out).is_ok() };
                if !sent || done.is_some() { break; }
            }
        }).expect("game battle thread");
        crate::audio::external(true);
        GameBattle { tx, rx, result: None, frame: None, in_flight: 0, acc: 0.0 }
    }

    /// Advance with the viewer's frame time: one game frame per 1/29.97 s, at most one ahead.
    pub fn update(&mut self, dt: f32, pad: Pad) {
        while let Ok(o) = self.rx.try_recv() {
            self.in_flight -= 1;
            crate::audio::push_external(&o.audio);
            if !o.rgba.is_empty() { self.frame = Some(o.rgba); }
            if let Some(r) = o.done { self.result = Some(r); }
        }
        if self.result.is_some() { return; }
        self.acc += dt;
        let step = 1001.0 / 30000.0;
        if self.acc >= step && self.in_flight < 2 {
            self.acc -= step;
            if self.acc > step { self.acc = 0.0; }
            if self.tx.send(pad).is_ok() { self.in_flight += 1; }
        }
    }
}

impl Drop for GameBattle {
    fn drop(&mut self) { crate::audio::external(false); }
}

/// The controller as the game's PADRead returns it: PAD_BUTTON_* bits and the sticks (-128..127).
pub fn pad(keys: &ButtonInput<KeyCode>, pads: &Query<&Gamepad>) -> Pad {
    let mut b = 0u16;
    let mut set = |on: bool, bit: u16| if on { b |= bit; };
    set(keys.pressed(KeyCode::ArrowLeft), 0x1);
    set(keys.pressed(KeyCode::ArrowRight), 0x2);
    set(keys.pressed(KeyCode::ArrowDown), 0x4);
    set(keys.pressed(KeyCode::ArrowUp), 0x8);
    set(keys.pressed(KeyCode::KeyZ), 0x10);
    set(keys.pressed(KeyCode::KeyE), 0x20);
    set(keys.pressed(KeyCode::KeyQ), 0x40);
    set(keys.pressed(KeyCode::Space) || keys.pressed(KeyCode::KeyX) || keys.pressed(KeyCode::Enter), 0x100);
    set(keys.pressed(KeyCode::Backspace) || keys.pressed(KeyCode::KeyC), 0x200);
    set(keys.pressed(KeyCode::KeyV), 0x400);
    set(keys.pressed(KeyCode::KeyF), 0x800);
    set(keys.pressed(KeyCode::Tab), 0x1000);
    let mut stick = (0i8, 0i8);
    let mut x = 0i32; let mut y = 0i32;
    if keys.pressed(KeyCode::KeyD) { x += 100; }
    if keys.pressed(KeyCode::KeyA) { x -= 100; }
    if keys.pressed(KeyCode::KeyW) { y += 100; }
    if keys.pressed(KeyCode::KeyS) { y -= 100; }
    for p in pads {
        let v = p.left_stick();
        if v.length() > 0.15 { x = (v.x * 100.0) as i32; y = (v.y * 100.0) as i32; }
        let mut g = |btn: GamepadButton, bit: u16| if p.pressed(btn) { b |= bit; };
        g(GamepadButton::South, 0x100); g(GamepadButton::East, 0x200); g(GamepadButton::West, 0x400);
        g(GamepadButton::North, 0x800); g(GamepadButton::Start, 0x1000); g(GamepadButton::DPadLeft, 0x1);
        g(GamepadButton::DPadRight, 0x2); g(GamepadButton::DPadDown, 0x4); g(GamepadButton::DPadUp, 0x8);
        g(GamepadButton::RightTrigger, 0x10); g(GamepadButton::RightTrigger2, 0x20); g(GamepadButton::LeftTrigger2, 0x40);
    }
    stick.0 = x.clamp(-128, 127) as i8; stick.1 = y.clamp(-128, 127) as i8;
    Pad { buttons: b, stick, ..Default::default() }
}
