//! Battles started from the field: std 102 `battleStart` (0x8020DAD0) and what it sets going, in the game's
//! order (NOTES.md §16 "Battle start"):
//! 1. flag 0x9B0 off, encounter id stored, BGM 1 (fade 1000), the sub sound stopped (1000 ms);
//!    `fadeSetEX(9, wipe init, snapshot, 1.0, wipe time)` with the encounter's wipe (`fightEncountWipe`),
//!    the wipe SE, then `mapWarpToMenuRoom(battle floor)` with no fade scripts. The calling script thread
//!    is blocked until the battle is over.
//! 2. The floor worker pushes the battle floor (`_bf` room of the encounter's `fightFloor`) over the field
//!    (state 4: the field's threads blocked, its data kept) and loads it; its pre func `_fightInitialize`
//!    starts `fightMainThread` (0x801EFA08), which runs the battle at 30 Hz.
//! 3. `fightMainThread`: env sound, `fightActionFlowKaisiPreSubLoad`, `fadeSet(1.0, 8)` (the wipe fades out
//!    over the battle floor), the start flow (action 2: trainer intro, send-outs), the turn loop, the end
//!    flows, `fadeSet(1.0, 3)` (to black), then the floor pops back to the field.
//! 4. Back in `battleStart`: flag 0x9B0 on, the modify script re-run (`floorReadScriptPostFunc`), and
//!    unless flag 0xE05, `fadeSet(0.5, 2)` from black; returns `battleGetResult`.
//!
//! Timing is from the Dolphin trace of encounter 55 (`Working/dolphin/traces/wipe55`): the push the frame
//! after the call, `fightInitialize` 50 frames later, `fightMainThread` 32 frames after that (both disc
//! loads), and `fadeSet(1.0, 8)` 8 battle frames into the thread.
//!
//! Not ported yet: the start flow's camera, trainer and Pokémon models and messages, the battle itself (the
//! rules run headless in `colo-battle`), and the end flows. The viewer shows the battle floor from its
//! scene camera until A is pressed, then ends the battle as a win.

use crate::script::ScriptWorld;
use colo_formats::battle_data::{Table, TABLES};

/// Battle tables the field needs, from common_rel and main.dol.
pub struct BattleTables {
    pub encount: Option<Table>,
    pub floor: Option<Table>,
    pub wipe: Option<Table>,
    pub trainer: Option<Table>,
    pub trainer_kind: Option<Table>,
    /// `wipe_ball00` / `wipe_ball01` (common.fsys 0x0F861200 / 0x0F871200).
    pub wipe_tex: Vec<colo_formats::gtx::Image>,
    /// Room names by floor id (for the battle floors).
    pub floor_rooms: std::collections::HashMap<u32, String>,
    /// `cam_party_a_1` (fight_common.fsys 0x7B1800): the battle camera `battleCameraStartOffsetAnime` plays.
    pub party_cam: Option<colo_formats::hsdanim::Camera>,
}

impl BattleTables {
    pub fn load(common: &colo_formats::common::Common, dol: &colo_formats::dol::Dol, cfs: &colo_formats::fsys::Fsys) -> Self {
        let t = |name: &str| TABLES.iter().find(|d| d.name == name).and_then(|d| Table::load(d, common, dol).map_err(|e| eprintln!("battle table {name}: {e}")).ok());
        let wipe_tex = [0x0F86_1200u32, 0x0F87_1200].iter().filter_map(|&id| {
            let e = cfs.entries.iter().find(|e| e.identifier == id)?;
            colo_formats::gtx::decode(&cfs.read(e).ok()?).map_err(|err| eprintln!("wipe texture {}: {err}", e.name)).ok()
        }).collect();
        BattleTables { encount: t("fightEncount"), floor: t("fightFloor"), wipe: t("fightEncountWipe"), trainer: t("fightTrainer"),
            trainer_kind: t("fightTrainerKind"), wipe_tex, floor_rooms: Default::default(), party_cam: None }
    }

    fn get(t: &Option<Table>, i: usize, f: &str, k: usize) -> i64 { t.as_ref().and_then(|t| t.get(i, f, k)).unwrap_or(0) }

    /// (fight floor index, battle floor id, wipe id) of an encounter.
    pub fn encounter(&self, enc: u32) -> (usize, u32, usize) {
        let ff = Self::get(&self.encount, enc as usize, "FightFloorDataId", 0) as usize;
        let floor = Self::get(&self.floor, ff, "FloorDataId", 0) as u32;
        (ff, floor, Self::get(&self.encount, enc as usize, "WipeId", 0) as usize)
    }

    /// `fightEncountWipe` record: (snapshot, SE, time, init function).
    pub fn wipe(&self, id: usize) -> (bool, u32, f32, u32) {
        (Self::get(&self.wipe, id, "WipeSnapshotUse", 0) != 0, Self::get(&self.wipe, id, "WipeEffectSndID", 0) as u32,
         f32::from_bits(Self::get(&self.wipe, id, "WipeEffectTime", 0) as u32), Self::get(&self.wipe, id, "WipeFunction", 0) as u32)
    }

    /// `fightEncountGetBgmSndDataId` 0x8020DD80: the encounter's BGM, else its floor's, else the first trainer
    /// kind with one, else 1.
    pub fn bgm(&self, enc: u32) -> u32 {
        let b = Self::get(&self.encount, enc as usize, "BgmSndId", 0);
        if b != 0 { return b as u32; }
        let (ff, ..) = self.encounter(enc);
        let b = Self::get(&self.floor, ff, "BgmSndId", 0);
        if b != 0 { return b as u32; }
        for k in 0..4 {
            let tr = Self::get(&self.encount, enc as usize, "FightTrainerDataId", k) as usize & 0xFFFF;
            if tr == 0 { continue; }
            let kind = Self::get(&self.trainer, tr, "KindDataId", 0) as usize;
            if kind == 0 { continue; }
            let b = Self::get(&self.trainer_kind, kind, "BgmSndId", 0);
            if b != 0 { return b as u32; }
        }
        1
    }
}

/// Frames the trace spent loading (see the module docs).
const LOAD_INIT: u32 = 50;
const LOAD_MAIN: u32 = 32;
const MAIN_TO_REVEAL: u32 = 8;
/// `fightEncountDataBiosGetWipeEffectTime` default and `fadeSet` times (0x8047E528/2C, 0x8047E508).
const FADE_WIPE: f32 = 1.0;
const FADE_BACK: f32 = 0.5;
const FADE_BATTLE: f32 = 1.0;
const FLAG_IN_BATTLE: u32 = 0x9B0;
const FLAG_NO_FADE_BACK: u32 = 0xE05;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// The frame of the call: the thread waits in `_fadeSnapshot` for the frame copy.
    Start,
    /// The frame after: the wipe SE and `mapWarpToMenuRoom`.
    Warp,
    /// Waiting for the viewer to push the battle floor (`mapWarpToMenuRoom`).
    Push,
    /// Disc loads (frames left), then `fightMainThread`.
    LoadInit(u32),
    LoadMain(u32),
    /// `fightMainThread` up to its `fadeSet(1.0, 8)` (battle frames left).
    Main(u32),
    /// The battle floor is up; waiting for the battle (A in the viewer).
    Battle,
    /// `fadeSet(1.0, 3)` running, then the pop.
    EndMusic,
    FadeOut,
    Pop,
    /// Back on the field: `fadeSet(0.5, 2)` running.
    FadeBack,
    Done,
}

#[derive(Clone, Debug)]
pub struct Encounter {
    pub id: u32,
    pub phase: Phase,
    /// The `_bf` room to push.
    pub room: Option<String>,
    pub floor_id: u32,
    pub bgm: u32,
    pub result: u32,
    wipe_se: u32,
    /// The battle floor is pushed (the field is hidden and frozen).
    pub pushed: bool,
    /// Odd battle frames are skipped (30 Hz).
    tick: u32,
    /// The viewer's A press ends the battle (no battle logic yet).
    pub end_request: bool,
    /// `cam_party_a_1` frame (`battleCameraStartOffsetAnime`: rate 0.5, looping, from `fightMainThread` on).
    pub cam_frame: Option<f32>,
    /// The field's BGM when the battle started. The game restores the field's sound state with the floor
    /// (`floorLoadResourceState` on the pop); the viewer restarts this BGM instead (approximation).
    /// Sound operations the battle thread is doing (they can take several frames: a song fades out for
    /// 50 ms before the next loads), run one after another every 60 Hz tick.
    snd: std::collections::VecDeque<colo_audio::SoundTask>,
    /// The battle runs on the game's own code (`gamebattle.rs`): its frames, audio and fades replace the
    /// viewer's battle-floor flow; `end_request` comes with its result.
    pub game: bool,
    /// The field's modify script must be re-run (`floorReadScriptPostFunc`) before the thread resumes.
    pub rerun_modify: bool,
}

impl Encounter {
    /// std 102 first call.
    pub fn start(w: &mut ScriptWorld, tables: &BattleTables, id: u32) -> Encounter {
        w.flags.remove(&FLAG_IN_BATTLE);
        let mut snd = std::collections::VecDeque::new();
        snd.push_back(colo_audio::SoundTask::set_bgm(1, 1000, 0xFF));
        snd.push_back(colo_audio::layer::with(|l| colo_audio::SoundTask::stop_sub(l, 1000)));
        let (_, floor_id, wipe_id) = tables.encounter(id);
        let (snap, se, time, init) = tables.wipe(wipe_id);
        w.fade_gs.set_ex(9, init, snap, FADE_WIPE, time);
        let room = tables.floor_rooms.get(&floor_id).cloned();
        println!("battleStart({id}): wipe {wipe_id} (init {init:#x}, {time} s, SE {se:#x}), battle floor {floor_id:#x} {}",
            room.as_deref().unwrap_or("(no room)"));
        Encounter { id, phase: Phase::Start, wipe_se: se, room, floor_id, bgm: tables.bgm(id), result: 2, pushed: false, tick: 0, end_request: false, rerun_modify: false, cam_frame: None, snd, game: false }
    }

    /// The fade layer's frames per game frame: the battle floor runs at 30 Hz.
    pub fn delta(&self) -> u32 { if matches!(self.phase, Phase::Main(_) | Phase::Battle | Phase::EndMusic | Phase::FadeOut) { 2 } else { 1 } }

    /// One 60 Hz tick. Returns whether a game frame happened (the fade daemon runs then).
    pub fn tick(&mut self, w: &mut ScriptWorld, tables: &BattleTables) -> bool {
        while let Some(t) = self.snd.front_mut() {
            if colo_audio::layer::with(|l| t.poll(l)).is_none() { break; }
            self.snd.pop_front();
        }
        // the battle camera (camera mode 8, `_cameraOffsetAnimeUpdate`): the animation's pose with offset
        // (0, 0, 0), rotation 0 and scale 1 (no Pokémon on the grid yet), up (0, 1, 0)
        if let (true, Some(f), Some(cam)) = (self.pushed, self.cam_frame.as_mut(), tables.party_cam.as_ref()) {
            let end = cam.anims.first().map_or(0.0, |a| a.end_frame);
            let p = cam.pose(0, *f);
            w.battle_cam = Some((p.eye.into(), p.interest.into(), p.fov, p.near, p.far, if p.aspect > 0.0 { p.aspect } else { 4.0 / 3.0 }));
            *f += 0.5;
            if end > 0.0 && *f > end { *f -= end; }
        }
        if self.delta() == 2 {
            self.tick ^= 1;
            if self.tick == 1 { return false; }
        }
        match self.phase {
            Phase::Start => if !w.fade_gs.snapshot { self.phase = Phase::Warp; },
            Phase::Warp => {
                if self.wipe_se != 0 { crate::audio::se(self.wipe_se); }
                self.phase = Phase::Push;
            }
            Phase::Push => {} // the viewer pushes the floor and moves on to LoadInit
            Phase::LoadInit(_) | Phase::LoadMain(_) | Phase::Main(_) if self.game => self.phase = Phase::Battle,
            Phase::Battle if self.game => if std::mem::take(&mut self.end_request) { self.phase = Phase::Pop; },
            Phase::LoadInit(n) => self.phase = if n > 1 { Phase::LoadInit(n - 1) } else { Phase::LoadMain(LOAD_MAIN) },
            Phase::LoadMain(n) => {
                self.phase = if n > 1 { Phase::LoadMain(n - 1) } else { Phase::Main(MAIN_TO_REVEAL) };
                if n <= 1 { self.cam_frame = Some(0.0); }
            }
            Phase::Main(n) => {
                if n > 1 { self.phase = Phase::Main(n - 1); } else {
                    w.fade_gs.set(8, FADE_BATTLE);
                    // the start flow plays the battle BGM (sound 0x1A for encounter 55, at this frame)
                    self.snd.push_back(colo_audio::SoundTask::set_bgm(self.bgm, 0, 0xFF));
                    self.phase = Phase::Battle;
                    println!("battle {}: floor up, BGM {}; press A to end it (battle logic not hooked up yet: counted as a win)", self.id, self.bgm);
                }
            }
            Phase::Battle => {
                if std::mem::take(&mut self.end_request) && !w.fade_gs.busy() {
                    // fightMainThread end (floor without env sound): std138(1, 1000, 1000, 0xFF), then fadeSet(3)
                    self.snd.push_back(colo_audio::layer::with(|l| colo_audio::SoundTask::change(l, 1, 1000, 1000, 0xFF, true)));
                    self.phase = Phase::EndMusic;
                }
            }
            Phase::EndMusic => if self.snd.is_empty() {
                w.fade_gs.set(3, FADE_BATTLE);
                self.phase = Phase::FadeOut;
            },
            // fadeCheck(1): wait for the fade, then one more frame
            // _fightFinalize: std104(1, 0, 0xFF), then the floor pops
            Phase::FadeOut => if !w.fade_gs.busy() {
                self.snd.push_back(colo_audio::SoundTask::set_bgm(1, 0, 0xFF));
                self.phase = Phase::Pop;
            },
            Phase::Pop => {} // the viewer pops the floor
            Phase::FadeBack => if !w.fade_gs.busy() && self.snd.is_empty() {
                self.phase = Phase::Done;
                println!("battle {}: back on the field, BGM {}", self.id, colo_audio::layer::with(|l| l.current_bgm()));
            },
            Phase::Done => {}
        }
        true
    }

    /// The viewer pushed the battle floor.
    pub fn pushed(&mut self) { self.pushed = true; self.phase = Phase::LoadInit(LOAD_INIT); }

    /// The viewer popped back to the field: the rest of `battleStart`.
    pub fn popped(&mut self, w: &mut ScriptWorld) {
        self.pushed = false;
        // the battle camera belongs to the battle floor; the field's camera takes over again
        self.cam_frame = None;
        w.battle_cam = None;
        w.flags.insert(FLAG_IN_BATTLE);
        self.rerun_modify = true;
        // the field BGM comes back from the re-run sound script (std104), not from battleStart
        self.phase = Phase::FadeBack;
        if !w.flags.contains(&FLAG_NO_FADE_BACK) {
            w.fade_gs.set(2, FADE_BACK);
        }
    }
}
