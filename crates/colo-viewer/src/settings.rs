//! The recompiled game's settings (`colo-viewer <gcm> recomp`): video, audio and the keyboard / gamepad mapping,
//! kept in `saves/recomp.ini` under the working folder (key = value lines) and changed in game from the Escape menu
//! (recomp.rs). Env COLO_SCALE, COLO_PRESENT, COLO_FRAME_LATENCY, COLO_FRAME_LEAD_MS, COLO_XFB_YUV and COLO_COPY_FILTER
//! still override the file for one run.

use winit::keyboard::KeyCode;

/// GameCube controller inputs a key can be bound to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    A, B, X, Y, Start, Z, L, R,
    DUp, DDown, DLeft, DRight,
    StickUp, StickDown, StickLeft, StickRight,
    CUp, CDown, CLeft, CRight,
    /// Held: the main stick at half tilt (walk).
    Walk,
}

pub const ACTIONS: [Action; 21] = [Action::A, Action::B, Action::X, Action::Y, Action::Start, Action::Z, Action::L, Action::R,
    Action::DUp, Action::DDown, Action::DLeft, Action::DRight, Action::StickUp, Action::StickDown, Action::StickLeft, Action::StickRight,
    Action::CUp, Action::CDown, Action::CLeft, Action::CRight, Action::Walk];

impl Action {
    pub fn name(self) -> &'static str {
        match self {
            Action::A => "A", Action::B => "B", Action::X => "X", Action::Y => "Y", Action::Start => "Start",
            Action::Z => "Z", Action::L => "L", Action::R => "R",
            Action::DUp => "D-pad up", Action::DDown => "D-pad down", Action::DLeft => "D-pad left", Action::DRight => "D-pad right",
            Action::StickUp => "Stick up", Action::StickDown => "Stick down", Action::StickLeft => "Stick left", Action::StickRight => "Stick right",
            Action::CUp => "C-stick up", Action::CDown => "C-stick down", Action::CLeft => "C-stick left", Action::CRight => "C-stick right",
            Action::Walk => "Walk (half tilt)",
        }
    }
    fn key(self) -> &'static str {
        match self {
            Action::A => "a", Action::B => "b", Action::X => "x", Action::Y => "y", Action::Start => "start",
            Action::Z => "z", Action::L => "l", Action::R => "r",
            Action::DUp => "dpad_up", Action::DDown => "dpad_down", Action::DLeft => "dpad_left", Action::DRight => "dpad_right",
            Action::StickUp => "stick_up", Action::StickDown => "stick_down", Action::StickLeft => "stick_left", Action::StickRight => "stick_right",
            Action::CUp => "c_up", Action::CDown => "c_down", Action::CLeft => "c_left", Action::CRight => "c_right",
            Action::Walk => "walk",
        }
    }
    /// PAD_BUTTON_* bit, for the buttons.
    pub fn button(self) -> u16 {
        match self {
            Action::DLeft => 0x1, Action::DRight => 0x2, Action::DDown => 0x4, Action::DUp => 0x8,
            Action::Z => 0x10, Action::R => 0x20, Action::L => 0x40,
            Action::A => 0x100, Action::B => 0x200, Action::X => 0x400, Action::Y => 0x800, Action::Start => 0x1000,
            _ => 0,
        }
    }
    fn default_keys(self) -> [Option<KeyCode>; 2] {
        use KeyCode::*;
        match self {
            Action::A => [Some(Space), Some(KeyX)], Action::B => [Some(Backspace), Some(KeyC)], Action::X => [Some(KeyV), None],
            Action::Y => [Some(KeyF), None], Action::Start => [Some(Enter), Some(Tab)], Action::Z => [Some(KeyZ), None],
            Action::L => [Some(KeyQ), None], Action::R => [Some(KeyE), None],
            Action::DUp => [Some(ArrowUp), None], Action::DDown => [Some(ArrowDown), None], Action::DLeft => [Some(ArrowLeft), None], Action::DRight => [Some(ArrowRight), None],
            Action::StickUp => [Some(KeyW), None], Action::StickDown => [Some(KeyS), None], Action::StickLeft => [Some(KeyA), None], Action::StickRight => [Some(KeyD), None],
            Action::CUp => [Some(KeyI), None], Action::CDown => [Some(KeyK), None], Action::CLeft => [Some(KeyJ), None], Action::CRight => [Some(KeyL), None],
            Action::Walk => [Some(ShiftLeft), Some(ShiftRight)],
        }
    }
}

/// Every key winit names, for reading and writing bindings by name.
pub const KEYS: &[KeyCode] = {
    use KeyCode::*;
    &[
    Backquote, Backslash, BracketLeft, BracketRight, Comma, Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6,
    Digit7, Digit8, Digit9, Equal, IntlBackslash, IntlRo, IntlYen, KeyA, KeyB, KeyC, KeyD, KeyE,
    KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM, KeyN, KeyO, KeyP, KeyQ,
    KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ, Minus, Period, Quote,
    Semicolon, Slash, AltLeft, AltRight, Backspace, CapsLock, ContextMenu, ControlLeft, ControlRight, Enter, SuperLeft, SuperRight,
    ShiftLeft, ShiftRight, Space, Tab, Convert, KanaMode, Lang1, Lang2, Lang3, Lang4, Lang5, NonConvert,
    Delete, End, Help, Home, Insert, PageDown, PageUp, ArrowDown, ArrowLeft, ArrowRight, ArrowUp, NumLock,
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9, NumpadAdd, NumpadBackspace,
    NumpadClear, NumpadClearEntry, NumpadComma, NumpadDecimal, NumpadDivide, NumpadEnter, NumpadEqual, NumpadHash, NumpadMemoryAdd, NumpadMemoryClear, NumpadMemoryRecall, NumpadMemoryStore,
    NumpadMemorySubtract, NumpadMultiply, NumpadParenLeft, NumpadParenRight, NumpadStar, NumpadSubtract, Escape, Fn, FnLock, PrintScreen, ScrollLock, Pause,
    BrowserBack, BrowserFavorites, BrowserForward, BrowserHome, BrowserRefresh, BrowserSearch, BrowserStop, Eject, LaunchApp1, LaunchApp2, LaunchMail, MediaPlayPause,
    MediaSelect, MediaStop, MediaTrackNext, MediaTrackPrevious, Power, Sleep, AudioVolumeDown, AudioVolumeMute, AudioVolumeUp, WakeUp, Meta, Hyper,
    Turbo, Abort, Resume, Suspend, Again, Copy, Cut, Find, Open, Paste, Props, Select,
    Undo, Hiragana, Katakana, F1, F2, F3, F4, F5, F6, F7, F8, F9,
    F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21,
    F22, F23, F24, F25, F26, F27, F28, F29, F30, F31, F32, F33,
    F34, F35
    ]
};

pub fn key_name(k: KeyCode) -> String { format!("{k:?}") }

/// A key as the menu shows it: "KeyX" as "X", "Digit1" as "1", "ArrowUp" as "Up".
pub fn key_label(k: KeyCode) -> String {
    let n = key_name(k);
    for p in ["Key", "Digit", "Arrow"] { if let Some(r) = n.strip_prefix(p) { return r.to_string(); } }
    n
}
pub fn key_from_name(s: &str) -> Option<KeyCode> { KEYS.iter().copied().find(|k| key_name(*k) == s) }

/// A three-way switch: the renderer's own default (by resolution), or forced.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tri { Auto, On, Off }

impl Tri {
    pub fn name(self) -> &'static str { match self { Tri::Auto => "Auto", Tri::On => "On", Tri::Off => "Off" } }
    pub fn next(self, d: i32) -> Tri { let v = [Tri::Auto, Tri::On, Tri::Off]; v[((self as i32 + d).rem_euclid(3)) as usize] }
    pub fn get(self, auto: bool) -> bool { match self { Tri::Auto => auto, Tri::On => true, Tri::Off => false } }
    fn parse(s: &str) -> Option<Tri> { match s { "auto" => Some(Tri::Auto), "on" | "1" => Some(Tri::On), "off" | "0" => Some(Tri::Off), _ => None } }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Present { Fifo, Mailbox, Immediate }

impl Present {
    pub fn name(self) -> &'static str { match self { Present::Fifo => "V-sync (FIFO)", Present::Mailbox => "Mailbox", Present::Immediate => "Immediate (tearing)" } }
    pub fn wgpu(self) -> wgpu::PresentMode { match self { Present::Fifo => wgpu::PresentMode::Fifo, Present::Mailbox => wgpu::PresentMode::Mailbox, Present::Immediate => wgpu::PresentMode::Immediate } }
    fn key(self) -> &'static str { match self { Present::Fifo => "fifo", Present::Mailbox => "mailbox", Present::Immediate => "immediate" } }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// EFB resolution: this many times the GameCube's 640x528.
    pub scale: u32,
    pub fullscreen: bool,
    pub present: Present,
    /// Swapchain frames the GPU may queue (1-3).
    pub frame_latency: u32,
    /// How long after its game time a frame is shown (ms): slack for slow frames, at that much input lag.
    pub frame_lead_ms: u32,
    /// 4:3 picture (false: stretched to the window).
    pub keep_aspect: bool,
    /// Bilinear scaling of the picture to the window (false: nearest).
    pub smooth: bool,
    pub xfb_yuv: Tri,
    pub copy_filter: Tri,
    pub perf_overlay: bool,
    /// 0-100.
    pub volume: u32,
    /// Gamepad stick dead zone, percent.
    pub deadzone: u32,
    /// Two keys per action.
    pub keys: Vec<[Option<KeyCode>; 2]>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings { scale: 3, fullscreen: false, present: Present::Fifo, frame_latency: 2, frame_lead_ms: 34, keep_aspect: true, smooth: true,
            xfb_yuv: Tri::Auto, copy_filter: Tri::Auto, perf_overlay: false, volume: 100, deadzone: 15,
            keys: ACTIONS.iter().map(|a| a.default_keys()).collect() }
    }
}

pub fn path() -> std::path::PathBuf { crate::save::working_dir().join("saves").join("recomp.ini") }

impl Settings {
    /// The file's settings (defaults for anything missing or unreadable), then the env overrides.
    pub fn load() -> Settings {
        let mut s = Settings::default();
        if let Ok(text) = std::fs::read_to_string(path()) {
            for line in text.lines() {
                let Some((k, v)) = line.split_once('=') else { continue };
                s.set(k.trim(), v.trim());
            }
        }
        for (env, key) in [("COLO_SCALE", "scale"), ("COLO_PRESENT", "present"), ("COLO_FRAME_LATENCY", "frame_latency"),
                           ("COLO_FRAME_LEAD_MS", "frame_lead_ms"), ("COLO_XFB_YUV", "xfb_yuv"), ("COLO_COPY_FILTER", "copy_filter")] {
            if let Ok(v) = std::env::var(env) { s.set(key, &v); }
        }
        s
    }

    fn set(&mut self, k: &str, v: &str) {
        let num = |lo: u32, hi: u32| v.parse::<u32>().ok().map(|n| n.clamp(lo, hi));
        let flag = || match v { "on" | "1" | "true" => Some(true), "off" | "0" | "false" => Some(false), _ => None };
        match k {
            "scale" => if let Some(n) = num(1, 8) { self.scale = n },
            "fullscreen" => if let Some(b) = flag() { self.fullscreen = b },
            "present" => self.present = match v { "mailbox" => Present::Mailbox, "immediate" => Present::Immediate, _ => Present::Fifo },
            "frame_latency" => if let Some(n) = num(1, 3) { self.frame_latency = n },
            "frame_lead_ms" => if let Some(n) = num(0, 100) { self.frame_lead_ms = n },
            "keep_aspect" => if let Some(b) = flag() { self.keep_aspect = b },
            "smooth" => if let Some(b) = flag() { self.smooth = b },
            "xfb_yuv" => if let Some(t) = Tri::parse(v) { self.xfb_yuv = t },
            "copy_filter" => if let Some(t) = Tri::parse(v) { self.copy_filter = t },
            "perf_overlay" => if let Some(b) = flag() { self.perf_overlay = b },
            "volume" => if let Some(n) = num(0, 100) { self.volume = n },
            "deadzone" => if let Some(n) = num(0, 60) { self.deadzone = n },
            _ => if let Some(i) = ACTIONS.iter().position(|a| format!("key.{}", a.key()) == k) {
                let mut ks = v.split(',').map(|x| key_from_name(x.trim()));
                self.keys[i] = [ks.next().flatten(), ks.next().flatten()];
            },
        }
    }

    pub fn save(&self) {
        let onoff = |b: bool| if b { "on" } else { "off" };
        let tri = |t: Tri| match t { Tri::Auto => "auto", Tri::On => "on", Tri::Off => "off" };
        let mut t = String::from("# colo-viewer recomp settings (the Escape menu writes this file)\n");
        t += &format!("scale = {}\nfullscreen = {}\npresent = {}\nframe_latency = {}\nframe_lead_ms = {}\nkeep_aspect = {}\nsmooth = {}\n",
            self.scale, onoff(self.fullscreen), self.present.key(), self.frame_latency, self.frame_lead_ms, onoff(self.keep_aspect), onoff(self.smooth));
        t += &format!("xfb_yuv = {}\ncopy_filter = {}\nperf_overlay = {}\nvolume = {}\ndeadzone = {}\n",
            tri(self.xfb_yuv), tri(self.copy_filter), onoff(self.perf_overlay), self.volume, self.deadzone);
        for (a, k) in ACTIONS.iter().zip(&self.keys) {
            let names: Vec<String> = k.iter().flatten().map(|k| key_name(*k)).collect();
            t += &format!("key.{} = {}\n", a.key(), names.join(", "));
        }
        let p = path();
        if let Some(d) = p.parent() { std::fs::create_dir_all(d).ok(); }
        if let Err(e) = std::fs::write(&p, t) { eprintln!("recomp: saving {}: {e}", p.display()); }
    }

    /// The actions a key is bound to.
    pub fn actions_of(&self, k: KeyCode) -> impl Iterator<Item = Action> + '_ {
        ACTIONS.iter().zip(&self.keys).filter(move |(_, ks)| ks.contains(&Some(k))).map(|(a, _)| *a)
    }
    pub fn keys_of(&self, a: Action) -> [Option<KeyCode>; 2] { self.keys[ACTIONS.iter().position(|x| *x == a).unwrap()] }
    pub fn set_keys(&mut self, a: Action, ks: [Option<KeyCode>; 2]) { let i = ACTIONS.iter().position(|x| *x == a).unwrap(); self.keys[i] = ks; }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keys_round_trip() {
        assert_eq!(key_from_name("KeyX"), Some(KeyCode::KeyX));
        assert_eq!(key_from_name("ShiftLeft"), Some(KeyCode::ShiftLeft));
        let mut s = Settings::default();
        s.set("key.a", "KeyM, Numpad0");
        assert_eq!(s.keys_of(Action::A), [Some(KeyCode::KeyM), Some(KeyCode::Numpad0)]);
        s.set("scale", "9");
        assert_eq!(s.scale, 8);
    }
}
