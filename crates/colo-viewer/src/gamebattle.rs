//! Battles run on the game's own code (colo-battle `run.rs` + colo-gx): when a battle's frame-boundary
//! snapshot exists (`Working/dolphin/snaps/story<encounter>`, from `story_entry_snap.py`), the battle phase
//! boots it on a worker thread, feeds it the pad at 30 Hz, shows the game's TV image full screen and plays its
//! audio; when the battle floor pops in the game, its `battleGetResult` goes back to the field script.
//!
//! The snapshot carries the Dolphin save's party and story state, not the viewer's (decision pending).

use bevy::prelude::*;
use colo_battle::run::{Machine, Pad};
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

/// Where the snapshot for `encounter` lives, if it does.
pub fn snapshot_dir(encounter: u32) -> Option<std::path::PathBuf> {
    let base = std::env::var("COLO_SNAPS").map(std::path::PathBuf::from).unwrap_or_else(|_| {
        std::path::PathBuf::from("/media/zach/massstorage/Pokemon Colosseum/Working/dolphin/snaps")
    });
    let d = base.join(format!("story{encounter}"));
    d.join("before.bin").exists().then_some(d)
}

impl GameBattle {
    /// The whole game from power-on: the `boot_snap.py` snapshot (MEM1 at `main`), memory card A from GCIs.
    pub fn boot(dir: std::path::PathBuf, gcm: std::path::PathBuf, cards: Vec<std::path::PathBuf>) -> GameBattle {
        Self::spawn(dir, gcm, Some(cards))
    }

    pub fn start(dir: std::path::PathBuf, gcm: std::path::PathBuf) -> GameBattle { Self::spawn(dir, gcm, None) }

    fn spawn(dir: std::path::PathBuf, gcm: std::path::PathBuf, boot: Option<Vec<std::path::PathBuf>>) -> GameBattle {
        let (tx, prx) = channel::<Pad>();
        let (otx, rx) = channel::<Out>();
        std::thread::Builder::new().name("game-battle".into()).stack_size(256 << 20).spawn(move || {
            let mut m = match Machine::from_snapshot(&dir) {
                Ok(m) => m,
                Err(e) => { eprintln!("game battle: {}: {e}", dir.display()); return; }
            };
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
            while let Ok(pad) = prx.recv() {
                m.set_pad(pad);
                m.frame();
                m.gpu_frame();
                let rgba = m.display(480);
                // the AX mix is R, L per frame; the output wants L, R
                let mut audio = colo_battle::run::take_audio();
                for p in audio.chunks_mut(2) { p.swap(0, 1); }
                let d = m.floor_depth();
                let done = if !boot_mode && was == 1 && d == 0 { Some(m.battle_result()) } else { None };
                was = d;
                if otx.send(Out { rgba, audio, done }).is_err() { break; }
                if done.is_some() { break; }
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
            self.frame = Some(o.rgba);
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
