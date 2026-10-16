//! The running script world: one CMS VM per room that keeps ticking (room sound script, NPC move scripts,
//! talk and interaction threads), the characters it drives, story flags, and the dialogue box state machine.
//!
//! Characters are addressed as `(group, index)` like the game's people system (`peopleFindSelf`): the room's
//! FSYS group plus the map REL character index, falling back to the index alone. Index 100 is the hero
//! (Wes), 101 the partner (Rui, not spawned by the viewer yet).
//!
//! Blocking natives return `Pending` until their condition holds (movement finished, one-shot animation
//! done, message acknowledged, door opened, timer expired), which is how the original natives loop on
//! `_threadSwitch`.

use crate::dialogue::{self, Page};
use bevy::prelude::*;
use colo_script::{Host, Memory, NativeCall, NativeResult, Vm};
use std::collections::{HashMap, HashSet, VecDeque};

pub const HERO: u32 = 100;
pub const PARTNER: u32 = 101;
/// World changes a script asked for that need the map model or collision.
#[derive(Debug, Clone)]
pub enum Cmd {
    /// `floorEventCtrlDoor(floor, index, mode)`: 0 open, 1 opened, 2 close, 3 closed.
    Door { index: usize, mode: u32 },
    /// `mapObjectPlayAnimation(resource, anim, frame, loop)`
    MapAnim { resource: u32, anim: usize, frame: f32, looping: bool },
    /// `scriptSetCol(collision group, on)`
    SetCol { group: u16, on: bool },
    /// `cameraSetFloorDefault(height, distance, rot_y)`
    CameraDefault { height: f32, distance: f32, rot_y: f32 },
    /// `mapObjectSetVisibility(resource, on)` (std 114, `GSmodelSetVisibility` on the map model set)
    MapVisible { resource: u32, on: bool },
    /// `mapObjectStopAnimation(resource)` (std 116, `GSmodelStopAnimation`)
    MapStop { resource: u32 },
}

#[derive(Debug, Clone)]
pub struct Actor {
    pub index: u32,
    pub pos: Vec3,
    /// Facing (radians; 0 = +Z).
    pub yaw: f32,
    pub visible: bool,
    /// CharacterModels animation slots: [0] idle, [1] walk, [2] run.
    pub slots: [u8; 8],
    /// Animation a script asked for: (HSD index, loop). Applied by the ECS sync.
    pub anim_request: Option<(usize, bool)>,
    /// Pose when standing still: the idle slot, or a looping animation a script set (sitting, ...).
    pub base_anim: usize,
    /// A one-shot animation is playing (kept up to date by the ECS sync).
    pub anim_busy: bool,
    pub path: VecDeque<Vec3>,
    pub run: bool,
    pub turn_to: Option<f32>,
    /// Map REL move type: 2 = wanders around its spot.
    pub move_type: u8,
    pub home: Vec3,
    pub wander_wait: f32,
    /// Talking to the hero: no wandering.
    pub talking: bool,
    /// Frames the current path step made no progress (stuck against a wall).
    pub stuck: f32,
    /// Units per 60 Hz frame at a walk (amount 1) and run (amount 2): CharacterModels stride / frame count of
    /// the walk / run animation (`peopleInit`).
    pub walk_speed: f32,
    pub run_speed: f32,
    /// A script placed it (`characterSetPosition`): the ECS takes the position even for the player.
    pub teleported: bool,
    /// Following the hero this frame (Some(running)); drives the walk/run animation.
    pub follow: Option<bool>,
    /// People flags (+0x24; see `heromove`).
    pub flags: u32,
    /// CharacterModels +0x18 collision radius.
    pub radius: f32,
    /// Move direction (+0x40), set by the stick / follow code; the body turns toward it.
    pub dir: f32,
    /// Hero/partner move amount for `updateAnimation` (-2..2), set by `heromove` each processed frame.
    pub amount: Option<f32>,
    /// In the human collision table (CharacterModels byte 0 bits 2-3 = 1) and switched on (std 151).
    pub human: bool,
    pub human_on: bool,
    /// Head turn limits (CharacterModels +0x1C / +0x20, degrees → radians).
    pub head_yaw: (f32, f32),
    /// Text-scroll sound type (CharacterModels byte 0 bits 5-7, `fn_8018F4AC`); characterTalk passes it to
    /// winMsgOpenFieldWithSE. 0 = silent.
    pub talk_se: u8,
    /// Rate (+0x44) of the running turn; `turn_to` is its target (+0x40 while +0x22 is set).
    pub turn_rate: f32,
    /// Default turn rate (+0x58; 1.0 from `peopleCreate` 0x8018FCE0, std 69 sets it). Used by std 68.
    pub turn_speed: f32,
    /// CharacterModels +9: the head part (joint index) the head look rotates; -1 = none.
    pub head_part: i8,
    /// Head look (std 65/66/181 set it, std 67 clears it): what the head looks at (+0x18).
    pub look: Option<LookAt>,
    /// Head (pitch, yaw): target (+0xB4, +0xB8) and current (+0xC, +0x10), radians.
    pub head_target: (f32, f32),
    pub head_rot: (f32, f32),
    /// World position of the head joint (`GSpartGetTransform`), written by the ECS sync.
    pub head_pos: Option<Vec3>,
    /// `characterAttachToCharacter` (std 70): riding another character's model part (people +0xC8/+0xCC/+0xD0 =
    /// group, index, part). `GSmodelAttachToGSpart(model, part, 7, 0, 1)` parents the model to the part's joint
    /// with its own translation/rotation reset, so `pos`/`yaw` are then local to that joint.
    pub attach: Option<(u32, usize)>,
}

/// What a head look follows: a character's position (std 181 keeps the pointer, so it tracks) or a point.
#[derive(Clone, Copy, Debug)]
pub enum LookAt { Actor(u32), Point(Vec3) }

/// Wrap into [0, 2π) by whole turns, as the turn update's loops do.
fn wrap_tau(mut v: f32) -> f32 {
    use std::f32::consts::TAU;
    if v >= 0.0 { while v >= TAU { v -= TAU; } } else { while v < 0.0 { v += TAU; } }
    v
}

impl Actor {
    pub fn new(index: u32, pos: Vec3, yaw: f32, visible: bool, slots: [u8; 8], move_type: u8) -> Self {
        let idle = if slots[0] == 0xFF { 0 } else { slots[0] as usize };
        Actor {
            index, pos, yaw, visible, slots, anim_request: None, base_anim: idle, anim_busy: false, path: VecDeque::new(),
            run: false, turn_to: None, move_type, home: pos, wander_wait: 1.0 + (index % 5) as f32 * 0.7, talking: false, stuck: 0.0, teleported: false, walk_speed: 0.3, run_speed: 0.75, follow: None,
            flags: 0, radius: 4.0, dir: yaw, amount: None, human: false, human_on: true, head_yaw: (0.0, 0.0), talk_se: 0,
            turn_rate: 1.0, turn_speed: 1.0, head_part: -1, look: None, head_target: (0.0, 0.0), head_rot: (0.0, 0.0), head_pos: None, attach: None,
        }
    }
    /// Speeds from the CharacterModels strides and the model's walk/run animation lengths.
    pub fn set_speeds(&mut self, (walk_stride, run_stride): (f32, f32), anims: &[colo_formats::hsdanim::Animation]) {
        let frames = |slot: u8| anims.iter().find(|a| a.model_set == 0 && a.index == slot as usize).map(|a| a.end_frame).filter(|f| *f > 0.0);
        if let Some(f) = frames(self.slots[1]) { self.walk_speed = walk_stride / f; }
        if let Some(f) = frames(self.slots[2]) { self.run_speed = run_stride / f; }
    }
    /// Radius, human-collision kind and head limits from the CharacterModels entry.
    pub fn set_body(&mut self, m: &colo_formats::common::CharacterModel) {
        self.radius = m.collision_radius;
        self.human = m.raw.first().is_some_and(|b| (b >> 2) & 3 == 1);
        let f = |o: usize| m.raw.get(o..o + 4).map_or(0.0, |b| f32::from_be_bytes([b[0], b[1], b[2], b[3]]));
        self.head_yaw = (f(0x1C).to_radians(), f(0x20).to_radians());
        self.talk_se = m.raw.first().map_or(0, |b| b >> 5);
        self.head_part = m.raw.get(9).map_or(-1, |&b| b as i8);
    }
    /// `fn_8018805C` (std 68/164/165 inline the same code): turn toward `angle` at `rate` (+0x44). The target
    /// is put in the revolution of the current rotation: 2π·trunc(rot/2π) + angle.
    pub fn start_turn(&mut self, angle: f32, rate: f32) {
        use std::f32::consts::TAU;
        self.dir = TAU * (self.yaw / TAU).trunc() + angle;
        self.turn_to = Some(self.dir);
        self.turn_rate = rate;
    }
    /// `fn_8018790C`: stop turning; the target becomes the current rotation.
    pub fn stop_turn(&mut self) {
        self.turn_to = None;
        self.dir = self.yaw;
    }
    /// `fn_80184A90`, from `peopleMain` every frame. The free hero (flag 0x40000000 without the lock flag 8)
    /// eases: (target − rot) / (10·(2.6 − rate)) per frame, at least 1°. Everyone else turns at a constant
    /// π/20·rate per frame the shortest way, and snaps once within one step.
    pub fn turn_update(&mut self, frames: u32) {
        use std::f32::consts::{PI, TAU};
        const DEG: f32 = 0.017_453_292;
        let Some(target) = self.turn_to else { return };
        let mut rot = self.yaw;
        let mut active = true;
        if self.flags & 0x4000_0000 == 0 || self.flags & 8 != 0 {
            rot = wrap_tau(rot);
            let t = wrap_tau(target);
            let step = 0.157_079_63 * self.turn_rate;
            for _ in 0..frames {
                let mut d = t - rot;
                if d.abs() >= PI { d = if d >= 0.0 { d - TAU } else { TAU + d }; }
                if d.abs() <= step { active = false; rot = t; break; }
                rot = if d <= 0.0 { rot - step } else { rot + step };
            }
        } else {
            if rot - target > 3.298_672_3 { rot -= TAU; } else if target - rot > 3.298_672_3 { rot += TAU; }
            let mut r = (target - rot) / (10.0 * (2.6 - self.turn_rate));
            if r.abs() <= DEG { r = if r >= 0.0 { DEG } else { -DEG }; }
            rot += r * frames as f32;
            if (r >= 0.0 && target <= rot) || (r < 0.0 && rot <= target) { active = false; rot = target; }
        }
        self.yaw = rot;
        if !active { self.turn_to = None; }
    }
    /// `fn_8018ECEC(75, people)`, from `peopleMain` every frame: aim the head at `target` (the look's
    /// position) if it is within 75 of the head and its yaw within the CharacterModels limits (people flag 2
    /// waives both, clamping instead); otherwise the head goes back to 0. The current angles then move
    /// toward the targets at 0.04 rad per frame. Pitch is clamped to the same +0x1C/+0x20 limits as yaw
    /// (the original reads those two fields for both).
    pub fn look_update(&mut self, target: Option<Vec3>, frames: u32) {
        use std::f32::consts::{FRAC_PI_2, PI, TAU};
        if let Some(t) = target {
            if self.head_part < 0 { return; }
            let Some(head) = self.head_pos else { return };
            let free = self.flags & 2 != 0;
            let (min, max) = self.head_yaw;
            self.head_target = (0.0, 0.0);
            if free || t.distance(head) <= 75.0 {
                let d = t - head;
                let mut a = ((TAU as f64 + (d.x.atan2(d.z) - self.yaw) as f64) % TAU as f64) as f32;
                if a > PI { a -= TAU; } else if a < -PI { a += TAU; }
                let yaw = if a >= min { if a > max { free.then_some(max) } else { Some(a) } } else { free.then_some(min) };
                if let Some(yaw) = yaw {
                    let p = (d.x.abs() + d.z.abs()).atan2(d.y + 13.0) - FRAC_PI_2;
                    self.head_target = (if p < min { min } else if p > max { max } else { p }, yaw);
                }
            }
        }
        let step = 0.04 * frames as f32;
        let ease = |cur: f32, t: f32| if cur <= t { (cur + step).min(t) } else { (cur - step).max(t) };
        self.head_rot = (ease(self.head_rot.0, self.head_target.0), ease(self.head_rot.1, self.head_target.1));
    }
    pub fn walk_anim(&self) -> usize { let w = self.slots[1]; if w == 0xFF { self.base_anim } else { w as usize } }
    pub fn run_anim(&self) -> usize { let r = self.slots[2]; if r == 0xFF { self.walk_anim() } else { r as usize } }
}

/// Message window state: pages of the current message, the typewriter position, an optional Yes/No menu,
/// and whether the player has pressed past the last page (`acked`), which is what the next dialogue native
/// waits for. Advanced once per 60 Hz tick by `tick`.
#[derive(Default)]
pub struct Dialog {
    pub pages: Vec<Page>,
    pub page: usize,
    /// Characters of the current page shown so far (one per tick).
    pub typed: usize,
    pub acked: bool,
    /// Yes/No menu cursor (0 = Yes) while the menu is up.
    pub menu: Option<usize>,
    pub choice: Option<usize>,
    /// Window alpha 0..=255 (open sequence 0x26: 0 → 255 over 2 ticks).
    pub alpha: f32,
    /// Next-page arrow animation counter (global at 0x8047AC68, mod 50).
    pub arrow_t: u32,
    /// Talk-SE type (msg window +3): picks the text-scroll sound (`audio::talk_se`); 0 = silent.
    pub talk_se: u8,
    /// A character was typed last tick (msg window +0x47); the scroll sound starts/stops on changes.
    pub printing: bool,
    /// Closing (window state 3): ticks since the close request (−1 = requested this tick) and the alpha
    /// sequence 0x2A started from. The text is hidden at once; only the background fades.
    pub closing: Option<(i8, f32)>,
}

impl Dialog {
    pub fn visible(&self) -> bool { !self.pages.is_empty() }
    fn page_len(&self) -> usize { self.pages.get(self.page).map_or(0, |p| p.chars()) }
    /// The current page is fully typed out.
    pub fn typed_out(&self) -> bool { self.typed >= self.page_len() }
    /// The last page is on screen and typed out.
    pub fn at_end(&self) -> bool { self.visible() && self.page + 1 >= self.pages.len() && self.typed_out() }
    /// Waiting for nobody: closed, or acknowledged.
    pub fn free(&self) -> bool { !self.visible() || self.acked }
    /// A silent window (winMsgOpenField / std 7 passes talk SE 0).
    pub fn open(&mut self, text: &str) { self.open_se(text, 0); }
    /// winMsgOpenFieldWithSE(id, 1, 0, se): `se` is the talk-SE type.
    pub fn open_se(&mut self, text: &str, se: u8) {
        self.free_se();
        // a reopen during the close fade restarts open sequence 0x26 from alpha 0 (winSeqIsCheck(0x2A), 0x80105B00)
        let keep_alpha = if self.visible() && self.closing.is_none() { self.alpha } else { 0.0 };
        let arrow_t = self.arrow_t;
        *self = Dialog { pages: dialogue::pages(text), alpha: keep_alpha, arrow_t, talk_se: se, ..Default::default() };
        if self.pages.is_empty() { self.pages.push(Page::default()); }
    }
    /// winMsgClose / winMsgCloseField → `windowClose(win, 2)` (0x80104828) sets the close request (win+10).
    /// `menuDaemon` then moves the window to state 3, where `winMsgCtrl` (0x80105A3C) starts sequence 0x2A
    /// (main.dol 0x802E6620: colour op 7 → (255,255,255,0) over 5 frames, linear, then wait 5) and
    /// `winMsgDraw` (0x801058CC) stops drawing the text (it only draws in state 2). The background keeps
    /// drawing at the window alpha: 255 on the request frame, then 204, 153, 102, 51, 0; the next frame the
    /// sequence has ended, state 4 → `windowCloseMain` → event 5 frees the message (GSmsgFree) and the window.
    pub fn close(&mut self) {
        if !self.visible() { self.clear(); return; }
        if self.closing.is_none() { self.menu = None; self.acked = true; self.closing = Some((-1, self.alpha)); }
    }
    /// Close at once (room change). Window item event 5 (0x80105C04) frees the message via GSmsgFree.
    pub fn clear(&mut self) { self.free_se(); let t = self.arrow_t; *self = Dialog { arrow_t: t, ..Default::default() }; }
    /// The close fade is running: the window is still up but shows no text.
    pub fn is_closing(&self) -> bool { self.closing.is_some() }
    /// GSmsgFree 0x800FBD88: keys the window's scroll sound off (FUN_801669BC) whenever win+3 maps to one,
    /// regardless of the edge state, then frees the slot. Called on window close (event 5) and before a
    /// reopen (event 0, 0x80105AA8).
    fn free_se(&mut self) {
        let id = crate::audio::talk_se(self.talk_se);
        if id != 0 { crate::audio::se_stop(id) }
        self.printing = false;
    }
    /// fn_800FC7E0 at 0x800FD14C: on a change of "typed a character this frame", start (FUN_80166A28) or
    /// key off (FUN_801669BC) the window's scroll sound. The MusyX macro retriggers it every ~70 ms.
    fn set_printing(&mut self, on: bool) {
        if on == self.printing { return; }
        self.printing = on;
        let id = crate::audio::talk_se(self.talk_se);
        if id == 0 { return; }
        if on { crate::audio::se(id) } else { crate::audio::se_stop(id) }
    }
    /// The next-page arrow is up.
    pub fn arrow(&self) -> bool {
        self.typed_out() && self.menu.is_none() && !self.acked && self.closing.is_none() && self.pages.get(self.page).is_some_and(|p| p.arrow)
    }
    /// One 60 Hz tick: type one more character, run the open fade and the arrow bob.
    pub fn tick(&mut self) {
        if !self.visible() { return; }
        self.arrow_t = (self.arrow_t + 1) % 50;
        if let Some((k, from)) = self.closing {
            // _winSeqMoveSub colour lerp: (char)(int)(k/5·(0 − from) + from); freed the tick after it reaches 0
            let k = k + 1;
            if k > 5 { self.clear(); return; }
            if k > 0 { self.alpha = (from - from * k as f32 / 5.0).trunc(); }
            self.closing = Some((k, from));
            return;
        }
        self.alpha = (self.alpha + 255.0 / 2.0).min(255.0);
        let typing = !self.typed_out();
        if typing { self.typed += 1; }
        self.set_printing(typing);
    }
    /// A pressed. While a page is still typing it is completed at once (UNVERIFIED against the game).
    /// A key wait passed (`msgctrlKeyWait`) plays sound 36; the Yes/No decide plays the menu's decide sound.
    pub fn press(&mut self) {
        if self.closing.is_some() { return; }
        if let Some(c) = self.menu.take() { self.choice = Some(c); self.acked = true; crate::audio::se(crate::audio::SE_DECIDE); return; }
        if !self.typed_out() { self.typed = self.page_len(); return; }
        if self.acked { return; }
        crate::audio::se(crate::audio::SE_MSG_KEY);
        if self.page + 1 < self.pages.len() { self.page += 1; self.typed = 0; } else { self.acked = true; }
    }
}

/// Script-driven camera (`GSscene_SetMode(3)` + `cameraSetPosition` / `cameraSetRotationAboutAxes`, or the
/// timed `cameraMove*XYZ`); `None` = the field camera.
#[derive(Debug, Clone, Copy)]
pub struct EventCam {
    pub pos: Vec3,
    /// (pitch x, yaw y, roll z) radians, like the field camera's `rotation` vector.
    pub rot: Vec3,
    pub from_pos: Vec3,
    pub from_rot: Vec3,
    /// Seconds into / length of a timed move (0 = jump).
    pub t: f32,
    pub dur: f32,
}

/// An animated event camera from a `.cam` file (`cameraPresetFromFile`, camera mode 4). Plays at rate 0.5.
#[derive(Debug, Clone, Copy)]
pub struct AnimCam {
    /// FSYS identifier of the `.cam` file.
    pub resource: u32,
    pub frame: f32,
    pub end_frame: f32,
    pub playing: bool,
}

impl AnimCam {
    pub fn ended(&self) -> bool { self.frame >= self.end_frame }
}

/// One step of the TV news event (std 182).
#[derive(Debug, Clone, Copy)]
pub enum TvStep {
    Channel(u32),
    /// Frames to wait.
    Wait(u32),
    /// `winMsgOpenFieldWithSE(id, 1, 0, 5)` + `winMsgCloseField(1)`: show the message, wait for it to be read.
    Msg(u32),
    ByStory,
    Term,
}

pub struct ScriptWorld {
    pub floor_id: u32,
    /// Door table (open flag, close flag) by global door index, for `floorEventCtrlDoor`'s early returns.
    pub doors: Vec<(u16, u16)>,
    /// The floor the hero came from (`mapGetPreviousRoomID`).
    pub prev_floor: u32,
    /// `mapSetReturnMap(floor, previous floor, entry index)`: where leaving a menu room returns to; the PC's
    /// Save option stores it as the save's location.
    pub return_map: Option<(u32, u32, usize)>,
    /// `mapWarpToMenuRoom(floor)` in progress (0x391 = the PC menu); the calling thread waits until the
    /// viewer's menu clears it.
    pub menu_room: Option<u32>,
    pub group_id: u32,
    pub messages: HashMap<u32, String>,
    /// Story flags (kept across rooms; no save file yet).
    pub flags: HashSet<u32>,
    /// Hero status 0x18 (`heroSetStatus(0, 0x18, on)`): RUI is in the party; saved with the game. Set by std 35.
    pub partner: bool,
    /// HEROMOVE member 1 (`0x80426BF4` bit 0): RUI is shown and follows (`initFloor`, `heroMoveMain`'s
    /// `partnerMove`). `heroMoveSyncWithHero` 0x80130054 sets it from `partner` (unless flag 0x8AE) on room entry;
    /// std 35 adds/dismisses it.
    pub partner_member: bool,
    pub actors: Vec<Actor>,
    /// The room's GScolsys2 collision (sight checks).
    pub colsys: Option<crate::colsys::ColSys>,
    /// `scriptFloorCharSetDisp` (std 100) writes: (NPC index, visible) for the floor data's NPC record bit 7
    /// (`floorCharacterBiosSetVisibility`). The game reads that bit only when `_floorInitCharacters` creates the
    /// characters, after the modify script ran at script load, so the viewer applies these after the modify script.
    pub char_disp: Vec<(u32, bool)>,
    pub dialog: Dialog,
    pub cmds: Vec<Cmd>,
    pub warp: Option<(u32, usize)>,
    pub hero_locked: bool,
    /// Hero/partner movement state (`heromove`).
    pub hm: crate::heromove::HeroState,
    /// An event started by `playerProcessEvents` (talk, check, region) is running: std 31 waits for it,
    /// like the game's synchronous `fn_800F7434` call inside `heroMoveMain`.
    pub blocking: bool,
    /// A map object (door) is mid-animation.
    pub map_busy: bool,
    pub event_cam: Option<EventCam>,
    pub anim_cam: Option<AnimCam>,
    /// Screen fade: current black level, target, and change per second (`transitionBegin`).
    pub fade: f32,
    pub fade_target: f32,
    pub fade_speed: f32,
    /// End frames of the room's `.cam` animations, by identifier (for cameraWaitSyncAnime).
    pub cam_ends: HashMap<u32, f32>,
    /// Where the camera is now (position, rotation), kept by the camera system for timed moves.
    pub cam_now: (Vec3, Vec3),
    /// The room's TV channels (room record +0x48) and the one showing (`DAT_80478b40`; None = -1, off).
    pub tv_list: Vec<colo_formats::maprel::TvChannel>,
    pub tv: Option<u32>,
    /// Shadow Pokémon table (registered flag, state flag) per index.
    pub shadow_flags: std::sync::Arc<Vec<(u16, u16, u16)>>,
    /// Running `tvNewsEvent` calls by thread serial: the steps left.
    tv_events: HashMap<u16, VecDeque<TvStep>>,
    /// `sleep` timers per thread serial (frames left).
    timers: HashMap<u16, f32>,
    /// Sound natives blocked in the game's sound layer (fades, the fanfare), per thread serial.
    snd_tasks: HashMap<u16, colo_audio::SoundTask>,
    /// Natives that already did their first-call work (door started, message opened), per thread serial.
    started: HashSet<u16>,
    rng: u32,
    pub trace: bool,
    /// The game's fade layer (`fadeSet*`, battle wipes; fade.rs).
    pub fade_gs: crate::fade::Fade,
    /// A battle started by std 102 (battle.rs) and its tables.
    pub battle: Option<crate::battle::Encounter>,
    pub battle_tables: Option<std::sync::Arc<crate::battle::BattleTables>>,
    /// The last battle's result (`battleGetResult`; 2 = won).
    pub battle_result: u32,
    /// The battle floor's camera (eye, interest, fov°, near, far, aspect) while it is pushed.
    pub battle_cam: Option<(Vec3, Vec3, f32, f32, f32, f32)>,
}

impl ScriptWorld {
    pub fn new(flags: HashSet<u32>) -> Self {
        ScriptWorld {
            floor_id: 0, doors: vec![], prev_floor: 0, return_map: None, menu_room: None, group_id: 0, messages: HashMap::new(), flags, partner: false, partner_member: false, actors: vec![], colsys: None, char_disp: vec![], dialog: Dialog::default(), cmds: vec![],
            warp: None, hero_locked: false, hm: Default::default(), blocking: false, map_busy: false, event_cam: None, anim_cam: None, fade: 0.0, fade_target: 0.0, fade_speed: 1.0, cam_ends: HashMap::new(), cam_now: (Vec3::ZERO, Vec3::ZERO), tv_list: vec![], tv: None, shadow_flags: Default::default(), tv_events: HashMap::new(), timers: HashMap::new(), snd_tasks: HashMap::new(), started: HashSet::new(), rng: 0x1234_5678,
            trace: std::env::var("COLO_TRACE").is_ok(), fade_gs: crate::fade::Fade::new(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0x1234_5678, |d| d.subsec_nanos())), battle: None, battle_tables: None, battle_result: 2, battle_cam: None,
        }
    }
    pub fn actor(&mut self, index: u32) -> Option<&mut Actor> { self.actors.iter_mut().find(|a| a.index == index) }
    /// `peopleMain` (0x80181850) per-frame part for every character: the body turn, then the head look.
    pub fn people_frame(&mut self) {
        let pos: Vec<(u32, Vec3)> = self.actors.iter().map(|a| (a.index, a.pos)).collect();
        for a in self.actors.iter_mut() {
            a.turn_update(1);
            let t = a.look.and_then(|l| match l { LookAt::Point(p) => Some(p), LookAt::Actor(i) => pos.iter().find(|p| p.0 == i).map(|p| p.1) });
            a.look_update(t, 1);
        }
    }
    pub fn hero_pos(&self) -> Vec3 { self.actors.iter().find(|a| a.index == HERO).map_or(Vec3::ZERO, |a| a.pos) }
    pub fn random(&mut self) -> u32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 17;
        self.rng ^= self.rng << 5;
        self.rng
    }
    /// heroMoveInitEvent (std 32): both members stop, lose wall/human collision and get the lock flag.
    pub fn hero_init_event(&mut self) {
        self.hero_locked = true;
        for a in self.actors.iter_mut().filter(|a| a.index == HERO || a.index == PARTNER) {
            a.amount = Some(0.0);
            a.flags = (a.flags | 0x8000_0008) & !(0x100 | 0x400);
            a.human_on = false;
        }
    }
    /// heroMoveTermEvent (std 33, 0x8012BBA8): human collision on, flags 0x700 back, unlocked.
    pub fn hero_term_event(&mut self) {
        self.hero_locked = false;
        for a in self.actors.iter_mut().filter(|a| a.index == HERO || a.index == PARTNER) {
            a.human_on = true;
            a.flags = (a.flags | 0x700) & !0x8000_0008;
        }
    }
    /// `tvNewsEvent` (std 182, 0x801C9F5C), what watching a TV does: the steps for the story so far.
    fn tv_news_plan(&mut self) -> VecDeque<TvStep> {
        use TvStep::*;
        let f = |w: &Self, n: u32| w.flags.contains(&n);
        // FUN_801EEAD0(0x43): Shadow Pokemon entry 0x43's +0x12 flag
        let caught = self.shadow_flags.get(0x43).is_some_and(|&(_, _, fl)| fl != 0 && self.flags.contains(&(fl as u32)));
        const WAIT: u32 = 180; // `while (t < 3.0) t += frames / vsync rate`
        if !caught && f(self, 0xADA) {
            self.flags.insert(0xA0F);
            return VecDeque::from([Channel(0xC), Wait(WAIT), Msg(0x439F), Channel(0xF), Msg(0x43A0), ByStory, Term]);
        }
        if !caught && f(self, 0xADB) {
            self.flags.insert(0x483);
            return VecDeque::from([Channel(0xC), Wait(WAIT), Msg(0x439D), Channel(0xE), Msg(0x439E), ByStory, Term]);
        }
        let u = if caught { 4 } else {
            let (flag, u) = if f(self, 0x42C) { (0x42D, 4) } else if f(self, 0xAD3) { (0xA08, 3) } else if f(self, 0xAD0) { (0xA07, 2) }
                else if f(self, 0xACF) { (0xA06, 1) } else { (0xA05, 0) };
            self.flags.insert(flag);
            u
        };
        VecDeque::from([ByStory, Wait(WAIT), Msg(0x426E + u), Term])
    }
    /// `mapSetTextureChange(id)` (std 171, 0x80117AE4): switch the TV to channel `id`. The same channel again
    /// returns `id`; otherwise the old channel is torn down, and 1 (found) or 0 (no such channel: TV off).
    pub fn tv_set_channel(&mut self, id: u32) -> u32 {
        if self.tv == Some(id) { return id; }
        self.tv = None;
        if self.tv_list.iter().any(|c| c.id == id) { self.tv = Some(id); 1 } else { 0 }
    }
    /// `tvSetChannelByStory` (std 211, 0x801CA358): the news channel for the story so far (flags 0xADA, 0xADB,
    /// 0x42C, 0xAD3, 0xAD0, 0xACF, each with a "seen" flag picking the second programme), then
    /// `mapSetTextureChange`. With none of them set it also sets flag 0xA05.
    pub fn tv_set_channel_by_story(&mut self) -> u32 {
        let f = |w: &Self, n: u32| w.flags.contains(&n);
        let ch = if f(self, 0xADA) { if f(self, 0xA0F) { 0xC } else { 0xB } }
            else if f(self, 0xADB) { if f(self, 0x483) { 0xC } else { 0xB } }
            else if f(self, 0x42C) { if f(self, 0x42D) { 4 } else { 3 } }
            else if f(self, 0xAD3) { if f(self, 0xA08) { 4 } else { 3 } }
            else if f(self, 0xAD0) { if f(self, 0xA07) { 4 } else { 3 } }
            else if f(self, 0xACF) { if f(self, 0xA06) { 2 } else { 1 } }
            else { self.flags.insert(0xA05); 0 };
        self.tv_set_channel(ch);
        1
    }
    pub fn message(&self, id: u32) -> String {
        self.messages.get(&id).cloned().unwrap_or_else(|| format!("<message {id:#x} not in this room's tables>"))
    }
}

/// The room's VM plus its world. Rebuilt on every room change (flags carry over).
#[derive(Resource)]
pub struct ScriptRt {
    pub vm: Vm,
    pub w: ScriptWorld,
    /// Threads that hold the hero still while they run (talk, interaction points).
    pub events: Vec<u16>,
    /// Interaction point scripts waiting to run, one at a time like the sync dispatcher.
    pub ip_queue: VecDeque<(u32, [u32; 4])>,
    /// Fractional 60 Hz ticks owed.
    pub acc: f32,
    /// A talk started by heroMoveMain is running (NPC index): its end restores the members.
    pub talk: Option<u32>,
}

impl ScriptRt {
    pub fn new(flags: HashSet<u32>) -> Self {
        ScriptRt { vm: Vm::new(), w: ScriptWorld::new(flags), events: vec![], ip_queue: VecDeque::new(), acc: -1.0, talk: None }
    }
    pub fn host(&mut self) -> (&mut Vm, WorldHost<'_>) { (&mut self.vm, WorldHost(&mut self.w)) }
    /// Start an event thread (talk, interaction point) that locks the hero until it finishes.
    pub fn start_event(&mut self, thread_id: u32, args: &[u32]) -> bool {
        let (vm, mut host) = self.host();
        match vm.spawn(thread_id, args, &mut host) {
            Some(s) => { self.events.push(s); true }
            None => false,
        }
    }
    /// Start an event and run it at once like the game's synchronous `fn_800F7434` call; it stays an event
    /// thread only if it blocks. Returns whether it is still running.
    pub fn run_event_now(&mut self, thread_id: u32, args: &[u32]) -> bool {
        let (vm, mut host) = self.host();
        let Some(serial) = vm.spawn(thread_id, args, &mut host) else { return false };
        let running = vm.run_now(serial, &mut host);
        for (s, _) in self.vm.reap() {
            self.events.retain(|&e| e != s);
            self.w.timers.remove(&s);
            self.w.snd_tasks.remove(&s);
            self.w.started.remove(&s);
        }
        if running { self.events.push(serial); }
        running
    }
    pub fn event_running(&self) -> bool { !self.events.is_empty() || !self.ip_queue.is_empty() }
    /// One 60 Hz game frame.
    pub fn tick(&mut self) {
        // a pushed battle floor: the field's threads are blocked (GSthreadBlockGroup); the battle runs
        if self.w.battle.as_ref().is_some_and(|b| b.pushed) {
            let mut b = self.w.battle.take();
            let t = self.w.battle_tables.clone();
            let frame = match (b.as_mut(), t) { (Some(b), Some(t)) => b.tick(&mut self.w, &t), _ => false };
            let delta = b.as_ref().map_or(1, |b| b.delta());
            self.w.battle = b;
            colo_audio::layer::with(|l| l.service());
            if frame { self.w.fade_gs.daemon(delta); }
            return;
        }
        self.field_tick();
        let mut b = self.w.battle.take();
        if let (Some(b), Some(t)) = (b.as_mut(), self.w.battle_tables.clone()) { b.tick(&mut self.w, &t); }
        self.w.battle = b;
        self.w.fade_gs.daemon(1);
    }

    fn field_tick(&mut self) {
        // interaction points run one after another
        if self.events.is_empty() {
            if let Some((script, params)) = self.ip_queue.pop_front() { self.start_event(script, &params); }
        }
        self.w.blocking = !self.events.is_empty() || !self.ip_queue.is_empty();
        colo_audio::layer::with(|l| l.service());
        let (vm, mut host) = self.host();
        vm.tick(&mut host);
        self.w.dialog.tick();
        for (serial, _) in self.vm.reap() {
            self.events.retain(|&e| e != serial);
            self.w.timers.remove(&serial);
            self.w.snd_tasks.remove(&serial);
            self.w.started.remove(&serial);
        }
        // a box left up by a finished script closes once acknowledged
        if self.events.is_empty() && self.ip_queue.is_empty() && self.w.dialog.visible() && self.w.dialog.acked {
            self.w.dialog.close();
        }
    }
}

/// Flag 0xE05: `battleStart` skips its `fadeSet(0.5, 2)` back on the field (std 152 sets it).
const FLAG_NO_FADE_BACK: u32 = 0xE05;

/// std 102 `battleStart` (0x8020DAD0) for the calling thread: the first call starts the wipe and the battle
/// (battle.rs); `Some(battleGetResult)` once it is over and back on the field.
fn battle_start(w: &mut ScriptWorld, serial: u16, enc: u32) -> Option<u32> {
    if !w.started.contains(&serial) {
        if !w.dialog.free() || w.battle.is_some() { return None; }
        let Some(t) = w.battle_tables.clone() else { return Some(2) };
        if enc == 0 { return Some(0); }
        // characterEndSpeech waited for the window to close
        w.dialog.clear();
        let e = crate::battle::Encounter::start(w, &t, enc);
        w.battle = Some(e);
        w.started.insert(serial);
    }
    if w.battle.as_ref().is_some_and(|b| b.phase == crate::battle::Phase::Done && !b.rerun_modify) {
        let b = w.battle.take()?;
        w.battle_result = b.result;
        w.started.remove(&serial);
        return Some(b.result);
    }
    None
}

pub struct WorldHost<'a>(pub &'a mut ScriptWorld);

impl Host for WorldHost<'_> {
    fn call_native(&mut self, c: &NativeCall, _m: &mut dyn Memory) -> NativeResult {
        let w = &mut *self.0;
        if w.trace && c.first_call {
            println!("  [t{} {:#010x}] {}({:?})", c.thread_serial, c.thread_id, c.name(), c.args);
        }
        let serial = c.thread_serial;
        let idx = |i: usize| c.int(i) as u32;
        let done = NativeResult::Done(0);
        match c.name() {
            // ---- dialogue: a new message or menu waits until the previous one was acknowledged
            "characterTalk" | "dialogDisplaySilentMessage" => {
                let msg = if c.name() == "characterTalk" { idx(2) } else { idx(0) };
                if !w.started.contains(&serial) {
                    if !w.dialog.free() { return NativeResult::Pending; }
                    let text = w.message(msg);
                    // characterTalk → winMsgOpenFieldWithSE(msg, 1, 0, model talk SE) (0x8018A260); std 7 is silent
                    let se = if c.name() == "characterTalk" { w.actor(idx(1)).map_or(0, |a| a.talk_se) } else { 0 };
                    w.dialog.open_se(&text, se);
                    w.started.insert(serial);
                    if c.name() == "characterTalk" { if let Some(a) = w.actor(idx(1)) { a.talking = true; } }
                }
                // returns once the last page is on screen (a menu may follow)
                if w.dialog.at_end() { w.started.remove(&serial); done } else { NativeResult::Pending }
            }
            "dialogDisplayYesNoMenu" => { w.dialog.menu = Some(0); w.dialog.choice = None; w.dialog.acked = false; done }
            "dialogWaitForMessage" => {
                if w.dialog.choice.is_some() { return NativeResult::Done(w.dialog.choice.take().unwrap_or(0) as u32); }
                if w.dialog.menu.is_some() || !w.dialog.free() { NativeResult::Pending } else { done }
            }
            "characterEndSpeech" => {
                if !w.dialog.free() { return NativeResult::Pending; }
                w.dialog.close();
                if let Some(a) = w.actor(idx(1)) { a.talking = false; }
                done
            }
            // ---- std 102: the wipe, the battle floor, back (battle.rs); returns battleGetResult
            "battleStart" => match battle_start(w, serial, idx(0) & 0xFFFF) { Some(r) => NativeResult::Done(r), None => NativeResult::Pending },
            // ---- std 152 `fieldStartEncount` (0x801CA5C4)(enc, fade, keep): flag 0xE05 off; with flag 0x8AE off: 0xE05 on
            // if fade or keep (battleStart then skips its fadeSet(0.5, 2)), battleStart(enc); a lost battle of a
            // Zenmetu encounter (fightFloorIsGcHeroWin(0, result) false) waits 3 frames and skips the fade; else
            // fade && !keep → fadeSet(0.5, 2) (DAT_8047E108). 0xE05 off again; returns battleStart's result.
            // (With flag 0x8AE the game runs the menu-room battle FUN_8006A65C instead: not ported.)
            "battleStartBattleWithOptions" => {
                let (enc, fade, keep) = (idx(0) & 0xFFFF, c.int(1) & 0xFF != 0, c.int(2) != 0);
                if let Some(t) = w.timers.get_mut(&serial) {
                    *t -= 1.0;
                    if *t > 0.0 { return NativeResult::Pending; }
                    w.timers.remove(&serial);
                    return NativeResult::Done(w.battle_result);
                }
                if !w.started.contains(&serial) {
                    w.flags.remove(&FLAG_NO_FADE_BACK);
                    if w.flags.contains(&0x8AE) { println!("battleStartBattleWithOptions({enc}): flag 0x8AE (menu-room battle) not ported; running battleStart"); }
                    else if fade || keep { w.flags.insert(FLAG_NO_FADE_BACK); }
                }
                let Some(r) = battle_start(w, serial, enc) else { return NativeResult::Pending };
                // fightFloorIsGcHeroWin: result 2 = the host side (the field hero's) won
                let zenmetu = w.battle_tables.as_ref().is_some_and(|t| t.zenmetu(enc));
                let lost = zenmetu && r != 2;
                if !keep && !lost && fade { w.fade_gs.set(2, 0.5); }
                w.flags.remove(&FLAG_NO_FADE_BACK);
                if lost { w.timers.insert(serial, 3.0); return NativeResult::Pending; }
                NativeResult::Done(r)
            }
            "battleGetResult" => NativeResult::Done(w.battle_result),
            // ---- flags (std 78 = set, 79 = clear; see NOTES)
            "flagCheckTrue" | "flagGetValue" => NativeResult::Done(w.flags.contains(&idx(0)) as u32),
            "flagSetTrue" => { w.flags.insert(idx(0)); done }
            "flagSetFalse" => { w.flags.remove(&idx(0)); done }
            "flagSetValue" => { if c.int(1) != 0 { w.flags.insert(idx(0)); } else { w.flags.remove(&idx(0)); } done }
            "getCurrentRoomID" => NativeResult::Done(w.floor_id),
            "getCurrentGroupID" => NativeResult::Done(w.group_id),
            "mapGetPreviousRoomID" => NativeResult::Done(w.prev_floor),
            "mapSetReturnMap" => { w.return_map = Some((idx(0), idx(1), c.int(2).max(0) as usize)); done }
            // the game swaps the whole floor for a menu room; the viewer overlays the menu instead and holds
            // the thread here until it closes
            "mapWarpToMenuRoom" => {
                if !w.started.contains(&serial) {
                    w.started.insert(serial);
                    w.menu_room = Some(idx(0));
                    return NativeResult::Pending;
                }
                if w.menu_room.is_some() { NativeResult::Pending } else { w.started.remove(&serial); done }
            }
            "randomNumberMod" => { let n = idx(0).max(1); NativeResult::Done(w.random() % n) }
            // ---- room
            "floorLinkWithSE" | "mapWarpToRoom" => { w.warp = Some((idx(0), c.int(1).max(0) as usize)); done }
            // 0x80116958: the flag checks return at once without touching the door, so a touch region that
            // reruns "open door" every frame while the hero stands in it doesn't hold him up
            "floorEventCtrlDoor" => {
                let mode = idx(2);
                if !w.started.contains(&serial) {
                    let Some(&(open_flag, close_flag)) = w.doors.get(idx(1) as usize) else { return NativeResult::Done(u32::MAX) };
                    let set = |f: u16| w.flags.contains(&(f as u32));
                    match mode {
                        0 if open_flag != 0 && set(open_flag) => return NativeResult::Done(0),
                        0 if close_flag != 0 && !set(close_flag) => return NativeResult::Done(1),
                        2 if open_flag != 0 && !set(open_flag) => return NativeResult::Done(0),
                        _ => {}
                    }
                    w.cmds.push(Cmd::Door { index: idx(1) as usize, mode });
                    if mode == 1 || mode == 3 { return done; }
                    w.started.insert(serial);
                    w.map_busy = true; // until the ECS reports otherwise
                    return NativeResult::Pending;
                }
                if w.map_busy { NativeResult::Pending } else { w.started.remove(&serial); done }
            }
            "mapObjectPlayAnimation" => {
                let id = idx(0);
                let (anim, looping) = (c.int(1).max(0) as usize, c.int(3) != 0);
                if id < 0x1_0000 {
                    // characterPlayAnimation(group, id, anim, frame, loop)
                    if let Some(a) = w.actor(id) {
                        a.anim_request = Some((anim, looping));
                        if looping { a.base_anim = anim; } else { a.anim_busy = true; }
                    }
                } else {
                    w.cmds.push(Cmd::MapAnim { resource: id, anim, frame: c.int(2) as f32, looping });
                }
                done
            }
            // map object natives fall back to the character version when the id is one of the group's
            // characters (`FUN_8018D998`); map objects are model-set resources (file id << 16 | 0x1000 | set)
            "mapObjectSetVisibility" => {
                let id = idx(0);
                if id < 0x1_0000 { if let Some(a) = w.actor(id) { a.visible = c.int(1) != 0; } }
                else { w.cmds.push(Cmd::MapVisible { resource: id, on: c.int(1) != 0 }); }
                done
            }
            "mapObjectStopAnimation" => { if idx(0) >= 0x1_0000 { w.cmds.push(Cmd::MapStop { resource: idx(0) }); } done }
            "mapSetTextureChange" => { let id = idx(0); NativeResult::Done(w.tv_set_channel(id)) }
            "tvSetChannelByStory" => NativeResult::Done(w.tv_set_channel_by_story()),
            "tvNewsEvent" => {
                if !w.tv_events.contains_key(&serial) {
                    w.hero_init_event();
                    let plan = w.tv_news_plan();
                    w.tv_events.insert(serial, plan);
                }
                loop {
                    let Some(step) = w.tv_events.get(&serial).and_then(|p| p.front().copied()) else {
                        w.tv_events.remove(&serial);
                        return NativeResult::Done(1);
                    };
                    match step {
                        TvStep::Channel(id) => { w.tv_set_channel(id); }
                        TvStep::ByStory => { w.tv_set_channel_by_story(); }
                        TvStep::Term => { w.hero_term_event(); }
                        TvStep::Wait(n) => {
                            if n > 1 { if let Some(p) = w.tv_events.get_mut(&serial) { p[0] = TvStep::Wait(n - 1); } return NativeResult::Pending; }
                        }
                        TvStep::Msg(id) => {
                            if !w.started.contains(&serial) {
                                if !w.dialog.free() { return NativeResult::Pending; }
                                let text = w.message(id);
                                w.dialog.open_se(&text, 5);
                                w.started.insert(serial);
                                return NativeResult::Pending;
                            }
                            if !w.dialog.free() { return NativeResult::Pending; }
                            // winMsgCloseField(1) is synchronous: wait out the close fade
                            w.dialog.close();
                            if w.dialog.visible() { return NativeResult::Pending; }
                            w.started.remove(&serial);
                        }
                    }
                    if let Some(p) = w.tv_events.get_mut(&serial) { p.pop_front(); }
                }
            }
            "scriptSetCol" => { w.cmds.push(Cmd::SetCol { group: c.int(0) as u16, on: c.int(1) != 0 }); done }
            "cameraSetFloorDefault" => { w.cmds.push(Cmd::CameraDefault { height: c.float(0), distance: c.float(1), rot_y: c.float(2) }); done }
            // ---- event camera
            "GSscene_SetMode" => { if c.int(0) == 0 { w.event_cam = None; w.anim_cam = None; } done }
            "cameraReturn" => { w.event_cam = None; w.anim_cam = None; done }
            // std 28 = `fadeSet(seconds, type)` (0x801C41C8; type 3 fades to black, 2 back in) on the game's fade layer;
            // std 29 = `fadeCheck(wait)` (0x801C40F0): with wait, loop on _threadSwitch while the fade is pending, then
            // one more frame; returns the pending flag
            "transitionBegin" => { w.fade_gs.set(c.int(0) as u16, c.float(1)); done }
            "transitionWait" => {
                if c.int(0) & 0xFF == 0 { return NativeResult::Done(w.fade_gs.busy() as u32); }
                if w.fade_gs.busy() { return NativeResult::Pending; }
                if w.started.insert(serial) { NativeResult::Pending } else { w.started.remove(&serial); NativeResult::Done(0) }
            }
            // cameraPresetFromFile(group, resource, flag, start frame) (0x80176E0C): camera mode 4, rate 0.5
            "cameraPresetFromFile" => {
                let resource = idx(1);
                match w.cam_ends.get(&resource) {
                    Some(&end) => {
                        w.anim_cam = Some(AnimCam { resource, frame: c.int(3).max(0) as f32, end_frame: end, playing: true });
                        w.event_cam = None;
                    }
                    None => println!("cameraPresetFromFile: no camera {resource:#x} in this room"),
                }
                done
            }
            "cameraWaitSyncAnime" => {
                let running = w.anim_cam.is_some_and(|a| a.playing && !a.ended());
                if c.int(0) != 0 { if running { NativeResult::Pending } else { done } } else { NativeResult::Done(running as u32) }
            }
            "cameraStopAnime" | "cameraStopAnimation" => { if let Some(a) = w.anim_cam.as_mut() { a.playing = false; } done }
            "cameraStartAnimation" => { if let Some(a) = w.anim_cam.as_mut() { a.playing = true; } done }
            "cameraSetPosition" | "cameraSetRotationAboutAxes" | "cameraMovePositionXYZ" | "cameraMoveRotationXYZ" => {
                let (now_p, now_r) = w.cam_now;
                let cam = w.event_cam.get_or_insert(EventCam { pos: now_p, rot: now_r, from_pos: now_p, from_rot: now_r, t: 0.0, dur: 0.0 });
                // settle a move in progress first
                cam.from_pos = cam.pos;
                cam.from_rot = cam.rot;
                let timed = c.name().starts_with("cameraMove");
                let o = if timed { 1 } else { 0 };
                let v = Vec3::new(c.float(o), c.float(o + 1), c.float(o + 2));
                if c.name().contains("Rotation") { cam.rot = v; } else { cam.pos = v; }
                cam.t = 0.0;
                cam.dur = if timed { c.float(4).max(0.0) } else { 0.0 };
                done
            }
            // heroMoveInitEvent 0x8012BCA4: both members idle, locked (0x80000008), no wall / human collision,
            // human collision off
            "playerLockMovement" => { w.hero_init_event(); done }
            // heroMoveTermEvent 0x8012BBA8: human collision on, flags 0x700 back, unlocked
            "playerFreeMovement" => { w.hero_term_event(); done }
            // heroMoveMain: the hero moves only while the main loop calls this (see heromove.rs)
            "playerProcessEvents" => {
                if w.blocking { return NativeResult::Pending; }
                w.hm.process = true;
                done
            }
            "characterSetFlags" => { if let Some(a) = w.actor(idx(1)) { a.flags |= idx(2); } done }
            "characterClearFlags" => { if let Some(a) = w.actor(idx(1)) { a.flags &= !idx(2); } done }
            // std 77 (0x80183018): people flags |= 0x100 (walls) | 0x400 (human collision), &= ~0x80000000;
            // std 76 (0x80183350) the reverse. 0x80000000 on the hero makes heroMoveMain return at once.
            "characterEventFlagsSet" => { if let Some(a) = w.actor(idx(1)) { a.flags = (a.flags | 0x500) & !0x8000_0000; } done }
            "characterEventFlagsClear" => { if let Some(a) = w.actor(idx(1)) { a.flags = (a.flags & !0x500) | 0x8000_0000; } done }
            // std 243 (0x801EE67C): Shadow Pokémon table entry `i` (1..=0x60): its registered flag (+0xC) := 1,
            // its state flag (+0x1C) := state (the save's shadow list slot is not modelled)
            "shadowPokemonSetState" => {
                let (i, state) = (idx(0) as usize, idx(1));
                if (1..=0x60).contains(&i) {
                    if let Some(&(reg, st, _)) = w.shadow_flags.get(i) {
                        if reg != 0 { w.flags.insert(reg as u32); }
                        if st != 0 { if state != 0 { w.flags.insert(st as u32); } else { w.flags.remove(&(st as u32)); } }
                    }
                }
                done
            }
            "characterSetHumanCollision" => { if let Some(a) = w.actor(idx(1)) { a.human_on = c.int(2) != 0; } done }
            // ---- sound (the game's sound layer, colo-audio::layer; blocking ones run as SoundTasks)
            "soundSetBackgroundMusic" | "soundPlaySub" | "soundStop" | "scriptSoundStop" | "soundChangeBackgroundMusic"
            | "soundChangeSub" | "soundPlayFanfareAndWait" => {
                use colo_audio::{layer, SoundTask};
                if c.first_call || !w.snd_tasks.contains_key(&serial) {
                    let t = layer::with(|l| match c.name() {
                        "soundSetBackgroundMusic" => SoundTask::set_bgm(idx(0), idx(1), idx(2)),
                        "soundPlaySub" => SoundTask::play_sub(idx(0), idx(1), idx(2)),
                        "soundStop" => SoundTask::stop(l, idx(0), idx(1)),
                        "scriptSoundStop" => SoundTask::stop_sub(l, idx(0)),
                        "soundChangeBackgroundMusic" => SoundTask::change(l, idx(0), idx(1), idx(2), idx(3), true),
                        "soundChangeSub" => SoundTask::change(l, idx(0), idx(1), idx(2), idx(3), false),
                        _ => SoundTask::fanfare(l, idx(0), idx(1), idx(2)),
                    });
                    w.snd_tasks.insert(serial, t);
                }
                let r = layer::with(|l| w.snd_tasks.get_mut(&serial).and_then(|t| t.poll(l)));
                match r { Some(v) => { w.snd_tasks.remove(&serial); NativeResult::Done(v) } None => NativeResult::Pending }
            }
            // std 137 (FUN_80166670): one sound's volume over ms
            "soundSetVolume" => NativeResult::Done(colo_audio::layer::with(|l| l.set_volume(idx(0), idx(1), idx(2))) as u32),
            // std 109 (FUN_80166D48(vol, ms, music, !fx)): the music master volume; returns 1
            "soundSetMusicMasterVolume" => { colo_audio::layer::with(|l| l.master_volume(idx(0), idx(1), true, false)); NativeResult::Done(1) }
            "soundGetCurrentBackgroundMusic" => NativeResult::Done(colo_audio::layer::with(|l| l.current_bgm())),
            "soundPlayPokemonCry" => { crate::audio::cry(idx(0)); done }
            // std 108 adds a second listener; the only slot is taken at boot, so it always fails
            "soundAddListener" => NativeResult::Done(0),
            "sleep" => {
                let t = w.timers.entry(serial).or_insert(c.float(0) * 60.0);
                *t -= 1.0;
                if *t <= 0.0 { w.timers.remove(&serial); done } else { NativeResult::Pending }
            }
            // ---- characters
            "scriptFloorCharSetDisp" => { w.char_disp.retain(|d| d.0 != idx(0)); w.char_disp.push((idx(0), c.int(1) & 1 != 0)); NativeResult::Done(1) }
            // std 70 characterAttachToCharacter(g, i, g2, i2, part) (0x801845E4): detach from any earlier part, then
            // ride (g2, i2)'s model part `part`; the ECS sync parents the model to that joint
            "characterAttachToCharacter" => {
                let (to, part) = (idx(3), c.int(4));
                // (the arrival's bike attach passes modelGetResourceID handles, which the viewer doesn't model: 0)
                if to != idx(1) && w.actors.iter().any(|a| a.index == to) {
                    if let Some(a) = w.actor(idx(1)) { a.attach = (part >= 0).then_some((to, part as usize)); }
                }
                done
            }
            // std 35 setPartnerFollowing(on) 0x801C9F00: heroSetStatus(0, 0x18, on), then add (FUN_8012F1FC: member bit,
            // RUI looks at the hero, shown) or dismiss (`heroMoveDismissMember`: bit cleared, she stays visible) member 1
            "setPartnerFollowing" => {
                let on = c.int(0) & 0xFF != 0;
                w.partner = on;
                if on && !w.partner_member {
                    if let Some(a) = w.actor(PARTNER) { a.look = Some(LookAt::Actor(HERO)); a.visible = true; }
                }
                w.partner_member = on;
                done
            }
            "characterSetVisibility" => { if let Some(a) = w.actor(idx(1)) { a.visible = c.int(2) != 0; } done }
            "characterSetAnimation" => {
                if let Some(a) = w.actor(idx(1)) {
                    let (anim, looping) = (c.int(2).max(0) as usize, c.int(4) != 0);
                    a.anim_request = Some((anim, looping));
                    if looping { a.base_anim = anim; } else { a.anim_busy = true; }
                }
                done
            }
            "characterWaitForAnimation" => {
                if w.actor(idx(1)).is_some_and(|a| a.anim_busy || a.anim_request.is_some_and(|r| !r.1)) { NativeResult::Pending } else { done }
            }
            "characterSetPosition" => {
                if let Some(a) = w.actor(idx(1)) { a.pos = Vec3::new(c.float(2), c.float(3), c.float(4)); a.path.clear(); a.home = a.pos; a.teleported = true; }
                done
            }
            "characterRotateAroundAxes" => { if let Some(a) = w.actor(idx(1)) { a.yaw = c.float(3).to_radians(); a.turn_to = None; } done }
            // std 164 characterTurnToPosition(g, i, x, y, z, rate) / std 165 characterFaceCharacter(g, i, g2, i2, rate):
            // turn the body toward the point / the other character's position at `rate`
            "characterTurnToPosition" | "characterFaceCharacter" => {
                let (target, rate) = if c.name() == "characterTurnToPosition" { (Some(Vec3::new(c.float(2), c.float(3), c.float(4))), c.float(5)) }
                    else { let t = idx(3); (w.actors.iter().find(|a| a.index == t).map(|a| a.pos), c.float(4)) };
                if let (Some(t), Some(a)) = (target, w.actor(idx(1))) { let d = t - a.pos; a.start_turn(d.x.atan2(d.z), rate); }
                done
            }
            // std 68 characterTurnToAngle(g, i, degrees): at the character's turn speed (+0x58)
            "characterTurnToAngle" => { if let Some(a) = w.actor(idx(1)) { let r = a.turn_speed; a.start_turn(c.float(2).to_radians(), r); } done }
            "characterSetTurnSpeed" => { if let Some(a) = w.actor(idx(1)) { a.turn_speed = c.float(2); } done }
            "characterMoveToPosition" | "characterRunToPosition" => {
                let run = c.name() == "characterRunToPosition" || c.int(2) == 2;
                let p = if c.name() == "characterRunToPosition" { Vec3::new(c.float(2), c.float(3), c.float(4)) } else { Vec3::new(c.float(3), c.float(4), c.float(5)) };
                if let Some(a) = w.actor(idx(1)) { a.path = VecDeque::from([p]); a.run = run; a.stuck = 0.0; }
                done
            }
            "characterMoveToCharacter" | "characterApproachPlayer" => {
                let (target, gap) = if c.name() == "characterApproachPlayer" { (HERO, 12.0) } else { (idx(3), c.float(4).max(8.0)) };
                let tp = w.actors.iter().find(|a| a.index == target).map(|a| a.pos);
                if let (Some(tp), Some(a)) = (tp, w.actor(idx(1))) {
                    let d = tp - a.pos;
                    let goal = if d.length() > gap { tp - d.normalize() * gap } else { a.pos };
                    a.path = VecDeque::from([goal]);
                    a.run = false;
                    a.stuck = 0.0;
                }
                done
            }
            "characterWaitForMovement" => {
                if w.actor(idx(1)).is_some_and(|a| !a.path.is_empty() || a.turn_to.is_some()) { NativeResult::Pending } else { done }
            }
            // head look (the body doesn't turn): std 65 = std 181 (g, i, 0, 100), std 66 a fixed point, std 67 stops
            // (the head part is unregistered at once, so the head snaps back; its angles stay for the next look)
            "characterLookAtPlayer" | "characterTurnHeadTowardsCharacter" | "characterLookAtPosition" => {
                let look = match c.name() {
                    "characterLookAtPlayer" => LookAt::Actor(HERO),
                    "characterLookAtPosition" => LookAt::Point(Vec3::new(c.int(2) as f32, c.int(3) as f32, c.int(4) as f32)),
                    _ => LookAt::Actor(idx(3)),
                };
                if let Some(a) = w.actor(idx(1)) { a.look = Some(look); }
                done
            }
            "characterStopLookAt" => { if let Some(a) = w.actor(idx(1)) { a.look = None; a.head_target = (0.0, 0.0); } done }
            "characterWaitForLookAt" => {
                let busy = w.actor(idx(1)).is_some_and(|a| a.head_rot != a.head_target);
                // std 155: 0 once the head reached its target; 1 if not and not waiting
                if !busy { done } else if c.int(2) != 0 { NativeResult::Pending } else { NativeResult::Done(1) }
            }
            // std 169 peopleGazeHeroCheck(g, i) 0x80186254 = peopleCanSee(3.0, 30.0, g, i, 0, 100) 0x80186284: (g, i) sees
            // the hero within 10·3 = 30 units, within ±0.5·30° of its facing, and no wall between them (FUN_8010F188:
            // the wall sweep from it to the hero with its CharacterModels radius)
            "peopleGazeHeroCheck" => {
                use std::f32::consts::{PI, TAU};
                let hero = w.hero_pos();
                let Some(a) = w.actors.iter().find(|a| a.index == idx(1)) else { return NativeResult::Done(0) };
                let d = hero - a.pos;
                if d.length() > 30.0 { return NativeResult::Done(0); }
                let mut ang = ((TAU as f64 + (d.x.atan2(d.z) - a.yaw) as f64) % TAU as f64) as f32;
                if ang > PI { ang -= TAU; } else if ang < -PI { ang += TAU; }
                if ang.abs() > 0.5 * 30.0f32.to_radians() { return NativeResult::Done(0); }
                let blocked = w.colsys.as_ref().is_some_and(|cs| cs.wall_sweep(a.radius, a.pos, hero).is_some());
                NativeResult::Done(!blocked as u32)
            }
            "characterCheckWithinBounds" => {
                let inside = w.actors.iter().find(|a| a.index == idx(1)).is_some_and(|a| {
                    let (x0, z0, x1, z1) = (c.float(2), c.float(3), c.float(4), c.float(5));
                    a.pos.x >= x0.min(x1) && a.pos.x <= x0.max(x1) && a.pos.z >= z0.min(z1) && a.pos.z <= z0.max(z1)
                });
                NativeResult::Done(inside as u32)
            }
            _ => done,
        }
    }
    fn log(&mut self, line: &str) {
        if self.0.trace { println!("  vm: {line}"); }
    }
}

#[cfg(test)]
mod dialog_tests {
    use super::*;
    #[test]
    fn close_fade() {
        let mut d = Dialog::default();
        d.open("Hi.{end}");
        for _ in 0..10 { d.tick(); }
        assert_eq!(d.alpha, 255.0);
        d.close();
        let mut seen = vec![];
        while d.visible() { d.tick(); assert!(d.is_closing() || !d.visible()); if d.visible() { seen.push(d.alpha as u32); } }
        assert_eq!(seen, [255, 204, 153, 102, 51, 0]);
    }
}
