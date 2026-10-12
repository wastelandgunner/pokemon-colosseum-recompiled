//! The game's sound layer over MusyX (main.dol 0x801653C4..0x80167BE8, `sound.c`): the sound id table, the
//! work slots, the music / sub / fanfare song buffers and the script-facing operations.
//!
//! Function names in comments are main.dol addresses. Operations that block the calling script thread in the
//! game (`FUN_80165D0C` waits, the fanfare loop) are [`SoundTask`]s, polled once per 60 Hz script frame.

use crate::engine::{self, ID_ERROR};
use colo_formats::common::{SoundEntry, SoundTables};
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

/// Work slots: `GSsndInit(2, 1, 0x20, 1, 0x10)` = 2 + 0x20.
pub const WORKS: usize = 34;
/// GSres ids of the song buffers (group 0): the music (64 KB), the sub slot (8 KB), the fanfare (64 KB).
pub const RES_MUSIC: u32 = 0x406;
pub const RES_SUB: u32 = 0x407;
pub const RES_FANFARE: u32 = 0x408;
/// Sound id of `null_bgm` (the "no music" song; pause/resume refuse it).
pub const NULL_BGM: u32 = 1;

const F_FX: u8 = 0x80;
const F_ATTACHED: u8 = 0x20;
const F_PAUSED: u8 = 0x10;

/// A work slot (0x14 bytes in the game).
#[derive(Debug, Clone, Copy)]
struct Work {
    /// +1 volume (0..127)
    vol: u8,
    /// +2 pan
    pan: u8,
    /// +4 duck stack depth, +5..+7 saved volumes
    depth: u8,
    saved: [u8; 3],
    /// +8 MusyX handle (sequence or FX voice), ID_ERROR = none
    handle: u32,
    /// +C / +10 the GSres (group, id) of the song buffer (0 for effects)
    group: u32,
    res: u32,
}

/// The tables and song data the layer plays from.
#[derive(Default)]
pub struct SoundData {
    pub sounds: Vec<SoundEntry>,
    /// (FSYS id in bgm_archive, DVD path) per song file.
    pub song_files: Vec<(u32, String)>,
    pub reverbs: Vec<[f32; 6]>,
    /// bgm_archive (FSYS group 0x99) entries by identifier, decompressed.
    pub songs: HashMap<u32, Vec<u8>>,
}

impl SoundData {
    pub fn new(t: &SoundTables) -> Self {
        SoundData { sounds: t.sounds.clone(), song_files: t.song_files.clone(), reverbs: t.reverbs.clone(), songs: HashMap::new() }
    }
}

pub struct SoundLayer {
    data: SoundData,
    /// Sound table +0 per id (bit 7 effect, plus the runtime bits 5 attached / 4 paused).
    flags: Vec<u8>,
    /// Sound table +8: the attached work.
    work_of: Vec<Option<usize>>,
    works: [Option<Work>; WORKS],
    /// `DAT_8047B0AC` current background music, `DAT_8047B0A8` current sub sound.
    pub cur_bgm: u32,
    pub cur_sub: u32,
    /// `DAT_8047B0E4` reverb preset in effect.
    cur_reverb: i32,
    /// Song buffers by GSres id (contents of the loaded `.song`, 0xE0-padded to the buffer size).
    bufs: HashMap<u32, Vec<u8>>,
    pub trace: bool,
}

impl Default for SoundLayer {
    fn default() -> Self { SoundLayer::new(SoundData::default()) }
}

static LAYER: Mutex<Option<SoundLayer>> = Mutex::new(None);

/// The global layer (the game's sound state outlives rooms and script worlds).
pub fn with<R>(f: impl FnOnce(&mut SoundLayer) -> R) -> R {
    let mut g = LAYER.lock().unwrap_or_else(|e| e.into_inner());
    f(g.get_or_insert_with(SoundLayer::default))
}

/// Install the layer (`FUN_80166E88` init: reverb preset 0, music volume 127, effect volume 100).
pub fn install(layer: SoundLayer) {
    *LAYER.lock().unwrap_or_else(|e| e.into_inner()) = Some(layer);
}

impl SoundLayer {
    pub fn new(data: SoundData) -> Self {
        let n = data.sounds.len();
        let mut l = SoundLayer {
            flags: data.sounds.iter().map(|s| s.flags).collect(),
            work_of: vec![None; n],
            works: [None; WORKS],
            cur_bgm: 0,
            cur_sub: 0,
            cur_reverb: -1,
            bufs: HashMap::new(),
            data,
            trace: std::env::var("COLO_SND_TRACE").is_ok(),
        };
        // FUN_80166E88 after sndInit: _sndSetReverbParm(0) + prepare, then the master faders
        l.set_reverb_parm(0);
        l.master_volume(0x7F, 0, true, false);
        l.master_volume(100, 0, false, true);
        l
    }

    pub fn count(&self) -> u32 { self.flags.len() as u32 }
    pub fn valid(&self, id: u32) -> bool { id != 0 && id < self.count() }
    pub fn entry(&self, id: u32) -> Option<&SoundEntry> { self.data.sounds.get(id as usize) }
    /// FUN_80166084
    pub fn is_fx(&self, id: u32) -> bool { self.flags.get(id as usize).is_some_and(|f| f & F_FX != 0) }
    fn attached(&self, id: u32) -> bool { self.flags.get(id as usize).is_some_and(|f| f & F_ATTACHED != 0) }
    fn work(&mut self, id: u32) -> Option<&mut Work> {
        let w = (*self.work_of.get(id as usize)?)?;
        self.works[w].as_mut()
    }

    // ---- work slots

    /// FUN_80166B3C: attach a free work slot to `id` with the song buffer (group, res).
    fn attach(&mut self, id: u32, group: u32, res: u32) -> bool {
        if self.attached(id) || !self.valid_index(id) { return false; }
        let free = |l: &Self| l.works.iter().position(|w| w.is_none());
        let slot = match free(self) {
            Some(s) => s,
            None => { self.check_all(); match free(self) { Some(s) => s, None => return false } }
        };
        // _sndInitParms
        self.works[slot] = Some(Work { vol: 0x7F, pan: 0x40, depth: 0, saved: [0; 3], handle: ID_ERROR, group, res });
        self.flags[id as usize] = (self.flags[id as usize] | F_ATTACHED) & !F_PAUSED;
        self.work_of[id as usize] = Some(slot);
        true
    }
    fn valid_index(&self, id: u32) -> bool { (id as usize) < self.flags.len() }

    /// FUN_80167070: release `id`'s work, stopping it first (time 0) if `stop`.
    fn release(&mut self, id: u32, stop: bool) -> bool {
        if !self.attached(id) { return false; }
        let Some(slot) = self.work_of[id as usize] else { return false };
        if stop { self.stop_sound(id, 0); }
        self.works[slot] = None;
        self.work_of[id as usize] = None;
        self.flags[id as usize] &= !F_ATTACHED;
        true
    }
    /// FUN_80166B18
    pub fn kill(&mut self, id: u32) -> bool { self.release(id, true) }
    /// FUN_801669BC(id) = FUN_801669E4(id, 0, 0): stop at time 0 (effects: sndFXKeyOff), keep the work.
    pub fn stop_raw(&mut self, id: u32) -> bool { self.valid_index(id) && self.stop_sound(id, 0) }

    /// _sndCheckSndWorkALL: a status check of every id (frees finished works).
    fn check_all(&mut self) { for id in 0..self.count() { self.status(id); } }

    /// FUN_80167768(group 0, res): the id whose work holds this song buffer.
    pub fn owner(&self, res: u32) -> Option<u32> {
        (0..self.flags.len()).find(|&i| self.work_of[i].and_then(|w| self.works[w]).is_some_and(|w| w.group == 0 && w.res == res)).map(|i| i as u32)
    }

    // ---- MusyX level

    /// FUN_801669E4: effects key off (_sndStopSE), songs fade to 0 over `ms` and stop (_sndStopBGM).
    fn stop_sound(&mut self, id: u32, ms: u32) -> bool {
        let fx = self.is_fx(id);
        let attached = self.attached(id);
        let Some(w) = self.work(id) else { return false };
        if w.handle == ID_ERROR { return false; }
        let h = w.handle;
        w.handle = ID_ERROR;
        if fx {
            engine::fx_check(h) != ID_ERROR && engine::fx_keyoff(h)
        } else {
            if !attached { return false; }
            engine::seq_volume(0, ms, h, 1);
            true
        }
    }

    /// FUN_801666BC: 0 inactive (a finished sound's work is released here), 2 playing, 3 paused.
    pub fn status(&mut self, id: u32) -> u32 {
        if !self.attached(id) { return 0; }
        let fx = self.is_fx(id);
        let paused = self.flags[id as usize] & F_PAUSED != 0;
        let Some(w) = self.work(id).copied() else { return 0 };
        if fx {
            if w.handle == ID_ERROR || engine::fx_check(w.handle) == ID_ERROR {
                if self.trace { println!("snd: GSsndGetStatus forced termination {id}"); }
                self.release(id, true);
                0
            } else { 2 }
        } else if paused {
            3
        } else if w.handle == ID_ERROR || !engine::seq_valid(w.handle) {
            if self.trace { println!("snd: GSsndGetStatus forced termination {id}"); }
            self.release(id, true);
            0
        } else { 2 }
    }
    fn busy(&mut self, id: u32) -> bool { matches!(self.status(id), 1..=3) }

    /// FUN_80166670 / FUN_8016737C: set a sound's volume (effects at once, songs over `ms`).
    pub fn set_volume(&mut self, id: u32, ms: u32, vol: u32) -> bool {
        if !self.valid_index(id) { return false; }
        if !(self.is_fx(id) || self.attached(id)) { return false; }
        let fx = self.is_fx(id);
        let Some(w) = self.work(id) else { return false };
        w.vol = (vol & 0x7F) as u8;
        if w.handle == ID_ERROR { return false; }
        if fx { engine::fx_ctrl(w.handle, 7, w.vol as u32); } else { engine::seq_volume(w.vol as u32, ms, w.handle, 0); }
        true
    }
    /// FUN_801656D8 / FUN_80166098: a sound's work volume (0 if inactive).
    pub fn volume_of(&mut self, id: u32) -> u32 {
        if !self.attached(id) { return 0; }
        self.work(id).map_or(0, |w| w.vol as u32)
    }

    /// _sndSetVolumeWork: store the volume, attaching a work to an effect that has none.
    fn set_volume_work(&mut self, id: u32, vol: u32) -> bool {
        if !self.attached(id) {
            if !self.is_fx(id) { return false; }
            self.attach(id, 0, 0);
        }
        if let Some(w) = self.work(id) { w.vol = (vol & 0x7F) as u8; }
        true
    }

    /// FUN_80166A50: set the volume, then start.
    fn start(&mut self, id: u32, fade: u32, vol: u32) -> u32 {
        if !self.set_volume_work(id, vol) { return 0; }
        self.start_raw(id, fade)
    }

    /// FUN_80166AB8 (also FUN_80166A28 = start at the stored volume: the message-advance sound).
    pub fn start_raw(&mut self, id: u32, fade: u32) -> u32 {
        if !self.valid_index(id) { return 0; }
        if self.is_fx(id) {
            // FUN_8016758C
            if !self.attached(id) { self.attach(id, 0, 0); }
            let fx = self.data.sounds[id as usize].musyx_id as u32;
            let Some(w) = self.work(id) else { return 0 };
            let (vol, pan) = (w.vol as u32, w.pan as u32);
            let h = engine::fx_start(fx, vol, pan, 0);
            if let Some(w) = self.work(id) { w.handle = h; }
            engine::fx_ctrl(h, 7, vol);
            if self.trace { println!("snd: fx {id} (FX {fx}) vol {vol} -> {h:#x}"); }
            1
        } else if self.attached(id) {
            // FUN_8016761C
            let Some(w) = self.work(id).copied() else { return 0 };
            if w.handle != ID_ERROR { return 0; }
            let Some(buf) = self.bufs.get(&w.res) else { return 0 };
            let buf = buf.clone();
            let e = self.data.sounds[id as usize];
            self.apply_reverb(e.reverb as i32);
            let h = engine::seq_play(e.group as u32, e.musyx_id as u32, &buf, fade, w.vol as u32);
            if let Some(w) = self.work(id) { w.handle = h; }
            if self.trace { println!("snd: song {id} (group {} song {}) fade {fade} vol {} -> {h:#x}", e.group, e.musyx_id, w.vol); }
            1
        } else { 0 }
    }

    /// _sndSetReverbParm
    fn set_reverb_parm(&mut self, idx: i32) {
        if let Some(p) = self.data.reverbs.get(idx as usize) { engine::set_reverb(*p); }
        self.cur_reverb = idx;
    }
    /// FUN_80166C34: a song's reverb preset, unless 0 or already in effect.
    fn apply_reverb(&mut self, idx: i32) {
        if idx != 0 && self.cur_reverb != idx { self.set_reverb_parm(idx); }
    }

    /// FUN_80166D48: master volume of the music (0xFD), effects (0xFE) or both (0xFF) groups; every active
    /// sound of that kind gets the volume stored in its work too.
    pub fn master_volume(&mut self, vol: u32, ms: u32, music: bool, fx: bool) {
        let group = match (music, fx) { (true, true) => 0xFF, (true, false) => 0xFD, (false, true) => 0xFE, _ => return };
        for id in 0..self.count() {
            if !self.attached(id) { continue; }
            let is_fx = self.is_fx(id);
            let hit = match group { 0xFE => is_fx, 0xFD => !is_fx, _ => true };
            if hit { self.set_volume_work(id, vol); }
        }
        engine::volume(vol & 0xFF, ms, group);
    }

    /// FUN_801668DC: pause a song (fade to 0 over `ms`, then sndSeqPause). Refused for null_bgm.
    pub fn pause(&mut self, id: u32, ms: u32) -> bool {
        if self.is_fx(id) || !self.attached(id) { return false; }
        let Some(w) = self.work(id).copied() else { return false };
        if w.handle == ID_ERROR || self.flags[id as usize] & F_PAUSED != 0 { return false; }
        if id == NULL_BGM { return true; }
        engine::seq_volume(0, ms, w.handle, 0);
        engine::seq_pause(w.handle);
        self.flags[id as usize] |= F_PAUSED;
        true
    }
    /// FUN_801667D8: resume a paused song: its reverb, sndSeqContinue, fade back to its volume over `ms`.
    pub fn resume(&mut self, id: u32, ms: u32) -> bool {
        if self.is_fx(id) || !self.attached(id) { return false; }
        let Some(w) = self.work(id).copied() else { return false };
        if w.handle == ID_ERROR || self.flags[id as usize] & F_PAUSED == 0 { return false; }
        let rev = self.data.sounds[id as usize].reverb as i32;
        self.apply_reverb(rev);
        if id == NULL_BGM { return true; }
        engine::seq_continue(w.handle);
        engine::seq_volume(w.vol as u32, ms, w.handle, 0);
        self.flags[id as usize] &= !F_PAUSED;
        true
    }

    /// FUN_801661D0: duck every active sound (music and/or effects) to at most `vol` over `ms`, saving its
    /// volume on its 3-deep stack (the start menu and PC box use 0x55 over 500 ms).
    pub fn duck(&mut self, vol: u32, ms: u32, music: bool, fx: bool) {
        for id in 0..self.count() {
            if !self.attached(id) { continue; }
            if !(if self.is_fx(id) { fx } else { music }) { continue; }
            // FUN_80166268
            let Some(w) = self.work(id) else { continue };
            if w.depth >= 3 { continue; }
            let cur = w.vol as u32;
            w.saved[w.depth as usize] = w.vol;
            if w.depth < 2 { w.depth += 1; }
            self.set_volume(id, ms, vol.min(cur));
        }
    }
    /// FUN_801660D8: undo one [`duck`] over `ms`.
    pub fn unduck(&mut self, ms: u32, music: bool, fx: bool) {
        for id in 0..self.count() {
            if !self.attached(id) { continue; }
            if !(if self.is_fx(id) { fx } else { music }) { continue; }
            // FUN_80166168
            let Some(w) = self.work(id) else { continue };
            if w.depth == 0 { continue; }
            w.depth -= 1;
            let v = w.saved[w.depth as usize] as u32;
            self.set_volume(id, ms, v);
        }
    }

    /// The receive-message callback (0x80167040): release the work of the handle that sent it.
    pub fn service(&mut self) {
        for vid in engine::take_messages() {
            // FUN_80167720 (0 when none matches; id 0 is never attached)
            let id = (0..self.flags.len()).find(|&i| self.work_of[i].and_then(|w| self.works[w]).is_some_and(|w| w.handle == vid)).unwrap_or(0);
            if id != 0 { self.release(id as u32, false); }
        }
    }

    /// FUN_80165EE0 (load bgm_archive entry `fsys_id` into the buffer `res` of `size` bytes, 0xE0-filled).
    fn load_song(&mut self, file: usize, res: u32, size: usize) -> bool {
        let (fsys_id, path) = self.data.song_files.get(file).cloned().unwrap_or_default();
        let mut buf = vec![0xE0u8; size];
        if fsys_id != 0 {
            match self.data.songs.get(&fsys_id) {
                Some(d) => { let n = d.len().min(size); buf[..n].copy_from_slice(&d[..n]); }
                None => if self.trace { println!("snd: bgm_archive entry {fsys_id:#x} not loaded"); },
            }
        } else if path.is_empty() {
            return false;
        } else if self.trace {
            println!("snd: song file {file} only on DVD ({path}): not loaded");
        }
        self.bufs.insert(res, buf);
        true
    }

    /// The current background music (std 228).
    pub fn current_bgm(&self) -> u32 { self.cur_bgm }
    pub fn current_sub(&self) -> u32 { self.cur_sub }
}

// ---- blocking operations

/// FUN_80165D0C(ms): yields until `(frames elapsed / 60)` summed reaches `ms / 1000` (single precision).
#[derive(Debug, Clone, Copy)]
struct Wait { acc: f32, target: f32 }

#[derive(Debug, Clone)]
enum Op {
    /// FUN_80165A44 play(id, fadeIn, vol, isBgm)
    Play { id: u32, fade: u32, vol: u32, bgm: bool },
    /// play, effect path, after its 50 ms fade: kill and start
    PlayFxRestart { id: u32, fade: u32, vol: u32 },
    /// play, song path: kill the previous owner (if any), load, attach, then start
    PlayLoad { id: u32, fade: u32, vol: u32, bgm: bool, prev: Option<u32>, file: usize },
    /// play, song path: set current and start
    PlayStart { id: u32, fade: u32, vol: u32, bgm: bool },
    /// FUN_80165C70 fadeSlot(ms, music, wait)
    FadeSlot { ms: u32, music: bool, wait: bool },
    /// soundStop's tail for songs
    StopTail { id: u32, bgm: bool },
    Wait(u32),
    /// FUN_801653CC: the fanfare in the 0x408 buffer
    FanfareStart { id: u32, fade: u32, vol: u32 },
    FanfareLoop { id: u32 },
    /// FUN_80165548: resume the music paused by FUN_8016557C
    Resume { owner: Option<u32> },
    Ret(u32),
}

/// A script-thread sound operation; poll it once per script frame until it returns `Some(result)`.
#[derive(Debug, Clone)]
pub struct SoundTask {
    ops: VecDeque<Op>,
    wait: Option<Wait>,
    ret: u32,
}

impl SoundTask {
    fn new(ops: Vec<Op>) -> Self { SoundTask { ops: ops.into(), wait: None, ret: 0 } }

    /// std 104 `soundSetBackgroundMusic(id, fadeIn, vol)`: play(id, fadeIn, vol, 1).
    pub fn set_bgm(id: u32, fade: u32, vol: u32) -> Self { Self::new(vec![Op::Play { id, fade, vol, bgm: true }]) }
    /// std 105 `soundPlaySub(id, fadeIn, vol)`: play(id, fadeIn, vol, 0) (an effect, or a song in the sub slot).
    pub fn play_sub(id: u32, fade: u32, vol: u32) -> Self { Self::new(vec![Op::Play { id, fade, vol, bgm: false }]) }
    /// std 138 `soundChangeBackgroundMusic(id, fadeOut, fadeIn, vol)` / std 139 `soundChangeSub`: fade the
    /// slot's song to 0 (waiting `fadeOut`), then play.
    pub fn change(l: &SoundLayer, id: u32, fade_out: u32, fade_in: u32, vol: u32, bgm: bool) -> Self {
        if !l.valid(id) { return Self::new(vec![Op::Ret(0)]); }
        Self::new(vec![Op::FadeSlot { ms: fade_out, music: bgm, wait: true }, Op::Play { id, fade: fade_in, vol, bgm }])
    }
    /// std 106 `soundStop(id, fadeMs)` (0x801657F8). Effects key off at once; the music can't be stopped:
    /// it fades out and null_bgm replaces it.
    pub fn stop(l: &mut SoundLayer, id: u32, ms: u32) -> Self {
        if !l.valid(id) { return Self::new(vec![Op::Ret(0)]); }
        let was_bgm = l.cur_bgm == id;
        if was_bgm { l.cur_bgm = 0; }
        let was_sub = l.cur_sub == id;
        if was_sub { l.cur_sub = 0; }
        let bgm = !was_sub && was_bgm;
        if !l.is_fx(id) {
            Self::new(vec![Op::FadeSlot { ms, music: bgm, wait: true }, Op::StopTail { id, bgm }])
        } else {
            l.stop_sound(id, ms);
            let r = l.kill(id) as u32;
            Self::new(vec![Op::Ret(r)])
        }
    }
    /// std 153 `scriptSoundStop(fadeMs)`: stop the current sub sound.
    pub fn stop_sub(l: &mut SoundLayer, ms: u32) -> Self { let id = l.cur_sub; Self::stop(l, id, ms) }
    /// std 130 `soundPlayFanfareAndWait(id, fadeIn, vol)`: pause the music (50 ms), play the fanfare to its
    /// end, resume the music over 1 s.
    pub fn fanfare(l: &mut SoundLayer, id: u32, fade: u32, vol: u32) -> Self {
        // FUN_8016557C
        let owner = l.owner(RES_MUSIC);
        let mut ops = vec![];
        if let Some(o) = owner { l.pause(o, 50); ops.push(Op::Wait(50)); }
        ops.push(Op::FanfareStart { id, fade, vol });
        ops.push(Op::Resume { owner });
        Self::new(ops)
    }
    /// FUN_801653CC on its own (the treasure pickup jingle): the music keeps playing underneath.
    pub fn jingle(id: u32, fade: u32, vol: u32) -> Self { Self::new(vec![Op::FanfareStart { id, fade, vol }]) }

    /// Run until the operation blocks (None) or finishes (Some(result)).
    pub fn poll(&mut self, l: &mut SoundLayer) -> Option<u32> {
        const FRAME: f32 = 1.0 / 60.0;
        if let Some(w) = self.wait.as_mut() {
            // back from _threadSwitch: one frame passed
            w.acc = (w.acc as f64 + FRAME as f64) as f32;
            if (w.acc as f64) < w.target as f64 { return None; }
            self.wait = None;
        }
        while let Some(op) = self.ops.pop_front() {
            match op {
                Op::Ret(v) => self.ret = v,
                Op::Wait(ms) => {
                    let w = Wait { acc: 0.0, target: ms as f32 / 1000.0 };
                    if (w.acc as f64) < w.target as f64 { self.wait = Some(w); return None; }
                }
                Op::Play { id, fade, vol, bgm } => {
                    if !l.valid(id) { self.ret = 0; continue; }
                    if l.is_fx(id) {
                        if !bgm {
                            if l.busy(id) {
                                l.set_volume(id, 50, 0);
                                self.push_front(vec![Op::Wait(50), Op::PlayFxRestart { id, fade, vol }]);
                                continue;
                            }
                            l.cur_sub = id;
                        }
                        self.ret = l.start(id, fade, vol);
                    } else {
                        let res = if bgm { RES_MUSIC } else { RES_SUB };
                        let file = l.data.sounds[id as usize].song_file as usize;
                        if file >= l.data.song_files.len() {
                            if l.trace { println!("snd: ERROR sound.c: invalid file info {id}"); }
                            self.ret = 0;
                            continue;
                        }
                        let prev = l.owner(res);
                        if prev != Some(id) {
                            if let Some(p) = prev {
                                if l.busy(p) {
                                    l.set_volume(p, 50, 0);
                                    self.push_front(vec![Op::Wait(50), Op::PlayLoad { id, fade, vol, bgm, prev: Some(p), file }]);
                                    continue;
                                }
                            }
                            self.push_front(vec![Op::PlayLoad { id, fade, vol, bgm, prev: None, file }]);
                        } else {
                            self.push_front(vec![Op::PlayStart { id, fade, vol, bgm }]);
                        }
                    }
                }
                Op::PlayFxRestart { id, fade, vol } => {
                    l.kill(id);
                    l.cur_sub = id;
                    self.ret = l.start(id, fade, vol);
                }
                Op::PlayLoad { id, fade, vol, bgm, prev, file } => {
                    if let Some(p) = prev { l.kill(p); }
                    let (res, size) = if bgm { (RES_MUSIC, 0x10000) } else { (RES_SUB, 0x2000) };
                    if !l.load_song(file, res, size) {
                        if l.trace { println!("snd: ERROR sound.c: Can't Play Sound {id}"); }
                        self.ret = 0;
                        continue;
                    }
                    l.attach(id, 0, res);
                    self.push_front(vec![Op::PlayStart { id, fade, vol, bgm }]);
                }
                Op::PlayStart { id, fade, vol, bgm } => {
                    if bgm { l.cur_bgm = id; } else { l.cur_sub = id; }
                    self.ret = l.start(id, fade, vol);
                }
                Op::FadeSlot { ms, music, wait } => {
                    if let Some(o) = l.owner(if music { RES_MUSIC } else { RES_SUB }) {
                        if l.busy(o) {
                            l.set_volume(o, ms, 0);
                            if wait { self.push_front(vec![Op::Wait(ms)]); }
                        }
                    }
                }
                Op::StopTail { id, bgm } => {
                    if bgm {
                        if l.trace { println!("snd: soundStop: Warning: BGM cannot be stopped ({id})"); }
                        self.push_front(vec![Op::Play { id: NULL_BGM, fade: 0, vol: 0xFF, bgm: true }]);
                    } else {
                        self.ret = l.kill(id) as u32;
                    }
                }
                Op::FanfareStart { id, fade, vol } => {
                    if !l.valid(id) { self.ret = 0; continue; }
                    let file = l.data.sounds[id as usize].song_file as usize;
                    self.ret = 1;
                    if file < l.data.song_files.len() && l.data.song_files[file].0 != 0 {
                        l.load_song(file, RES_FANFARE, 0x10000);
                        l.attach(id, 0, RES_FANFARE);
                        l.start(id, fade, vol);
                        self.push_front(vec![Op::FanfareLoop { id }]);
                    }
                }
                Op::FanfareLoop { id } => {
                    if l.status(id) == 2 { self.push_front(vec![Op::FanfareLoop { id }]); return None; }
                    l.kill(id);
                    l.bufs.remove(&RES_FANFARE);
                }
                Op::Resume { owner } => { if let Some(o) = owner { l.resume(o, 1000); } }
            }
        }
        Some(self.ret)
    }

    fn push_front(&mut self, ops: Vec<Op>) {
        for op in ops.into_iter().rev() { self.ops.push_front(op); }
    }
}
