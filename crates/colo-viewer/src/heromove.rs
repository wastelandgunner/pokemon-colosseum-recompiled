//! Port of the hero movement module `heroMove` (main.dol 0x8012B184–0x8013024C) and the people movement it
//! drives (`fn_80188214`, `fn_801885C4`, `fn_8018E9B4`).
//!
//! The hero only moves inside `heroMoveMain` (0x8012EBD4), which is script native std 31
//! (`playerProcessEvents`): the room's main loop calls it once per frame after its story-flag checks. So
//! while the main loop runs a cutscene, or an event script it started (talk, check, pass/touch region)
//! is still running, nothing calls it and the hero and partner stand still. It also returns at once while
//! the hero has people flag 0x80000000 (`playerLockMovement` = `heroMoveInitEvent` sets it,
//! `playerFreeMovement` = `heroMoveTermEvent` clears it).
//!
//! Each call: dispatch the check / touch / pass events collected last frame (interaction point methods
//! 3 / 2 / 1), open the field menu on X/Y/Start, move the partner (`fn_8012DE94`), then the leader
//! (`moveLeader`): A talks to the person in front (`fn_8018CD08`) or fires the check ray; otherwise the
//! stick sets the move direction and amount, the people speed setter moves the hero through the collision
//! system, and every 12 units the leader's position goes into a 20-point trail the partner follows.

use crate::colsys::ColSys;
use crate::script::{Actor, HERO, PARTNER};
use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

/// People flags (people entry +0x24).
pub const F_LOCKED: u32 = 0x8000_0000;
pub const F_WALL: u32 = 0x100;
pub const F_FLOOR: u32 = 0x200;
pub const F_HUMAN: u32 = 0x400;
pub const F_EVENTS: u32 = 0x800;
/// `heroMoveInit` (0x8012FCD4): Wes 0x40000F00, Rui 0x701.
pub const HERO_FLAGS: u32 = 0x4000_0F00;
pub const PARTNER_FLAGS: u32 = 0x701;
/// Human collision cylinder height (`fn_8018CB5C`: 17.0 at 0x8047D87C).
pub const HUMAN_HEIGHT: f32 = 17.0;
/// Trail spacing (0x8047D0D4) and the partner's follow distance (`heroMoveSyncWithHero`: 12).
pub const TRAIL_STEP: f32 = 12.0;
pub const TRAIL_LEN: usize = 20;

/// Per-room hero-move state (0x80426BD0 block).
#[derive(Debug, Clone)]
pub struct HeroState {
    /// Leader trail ring (0x80426C1C, 20 × 12 bytes): `head` = next write slot, `count` ≤ 20.
    pub trail: [Vec3; TRAIL_LEN],
    pub head: usize,
    pub count: usize,
    /// Per member: frames left before the partner may be put back near the leader (+0x10, 300).
    pub timer: [f32; 2],
    /// Per member: follow distance (+0x08: leader 0, partner 12).
    pub follow: [f32; 2],
    /// Per member: look mode (+0x0C: 1 = looks at the player when standing).
    pub look: [u8; 2],
    /// Events found during the last move, dispatched at the start of the next call (lists at 0x80426D70,
    /// 0x80426E40, 0x80426F10).
    pub check: Vec<u16>,
    pub pass: Vec<u16>,
    pub touch: Vec<u16>,
    /// `heroMoveSetLockFrame`: calls to skip.
    pub lock_frames: i32,
    /// Set by std 31 this frame.
    pub process: bool,
}

impl Default for HeroState {
    fn default() -> Self {
        HeroState {
            trail: [Vec3::ZERO; TRAIL_LEN], head: 0, count: 0, timer: [300.0; 2], follow: [0.0, TRAIL_STEP], look: [0, 1],
            check: vec![], pass: vec![], touch: vec![], lock_frames: 0, process: false,
        }
    }
}

impl HeroState {
    fn newest(&self, back: usize) -> Option<Vec3> {
        if back >= self.count { return None; }
        Some(self.trail[(self.head + TRAIL_LEN - 1 - back) % TRAIL_LEN])
    }
    fn push(&mut self, p: Vec3) {
        self.trail[self.head] = p;
        self.head = (self.head + 1) % TRAIL_LEN;
        self.count = (self.count + 1).min(TRAIL_LEN);
    }
}

/// Controller state for one frame, in GameCube units.
#[derive(Default, Clone, Copy)]
pub struct Pad {
    /// Main stick (x right, y up), ±100 at full tilt.
    pub stick: Vec2,
    /// D-pad (x right, y up), each -1/0/1.
    pub dpad: IVec2,
    pub a: bool,
    pub menu: bool,
}

/// A person in the human collision table (0x30 cylinders, `fn_8010FAF4`).
#[derive(Clone, Copy)]
pub struct Human { pub index: u32, pub pos: Vec3, pub radius: f32 }

/// Floor height choice of `fn_8018E9B4` (also `initFloor`): no floor → 0, one → it, several → the highest
/// below `y + limit`, else the highest overall.
fn pick_floor(heights: &[f32], y: f32, limit: f32, none: f32) -> f32 {
    match heights.len() {
        0 => none,
        1 => heights[0],
        _ => {
            let below = heights.iter().copied().filter(|h| h - y < limit).fold(None, |m: Option<f32>, h| Some(m.map_or(h, |m| m.max(h))));
            below.unwrap_or_else(|| heights.iter().copied().fold(-1_000_000.0, f32::max))
        }
    }
}

/// Result of one people move.
#[derive(Default)]
pub struct Moved { pub pos: Vec3, pub touch: Vec<u16>, pub pass: Vec<u16> }

/// `fn_8018E9B4`: move a person from `from` to `to` through the collision system according to its flags.
pub fn people_move(cs: &ColSys, humans: &[Human], me: &Actor, from: Vec3, to: Vec3) -> Moved {
    if me.flags & 0x700 == 0 { return Moved { pos: to, ..default() }; }
    let raise = Vec3::Y * 8.5;
    let (start, end) = (from + raise, to + raise);
    let mut out = Moved::default();
    if me.flags & F_EVENTS != 0 { out.touch = cs.touch_regions(me.radius, start, end); }
    let mut cur = end;
    if me.flags & F_HUMAN != 0 {
        if let Some(q) = human_collision(humans, me, from, cur) { cur = q; }
    }
    if me.flags & F_WALL != 0 {
        if let Some(q) = cs.wall_sweep(me.radius, start, cur) { cur = q; }
    }
    if me.flags & F_EVENTS != 0 { out.pass = cs.pass_regions(start, cur); }
    if me.flags & F_FLOOR != 0 {
        let h = cs.floor_heights(cur);
        cur.y = pick_floor(&h, cur.y, 10.0, 0.0);
    }
    out.pos = cur;
    out
}

/// `GScolsys2HumanCollision` 0x8010FDF8 / `fn_8010FAF4`: sweep my cylinder (radius, 17 high) from `a` to `b`
/// in steps of my radius; the first step that overlaps another enabled person is pushed out to the sum of
/// both radii (+0.0001), repeated up to 10 times.
fn human_collision(humans: &[Human], me: &Actor, a: Vec3, b: Vec3) -> Option<Vec3> {
    let dist = a.distance(b);
    let step = if dist > 0.0 { (me.radius / dist).min(1.0) } else { 0.0 };
    let d = b - a;
    let mut t = 0.0f32;
    while t < 1.0 {
        let t1 = (t + step).min(1.0);
        let mut q = d * t1 + a;
        let mut pushes = 0;
        while pushes < 10 {
            let hit = humans.iter().find(|h| {
                h.index != me.index && q.y + HUMAN_HEIGHT > h.pos.y && h.pos.y + HUMAN_HEIGHT > q.y
                    && Vec2::new(q.x - h.pos.x, q.z - h.pos.z).length() < h.radius + me.radius
            });
            let Some(h) = hit else { break };
            let (dx, dz) = (q.x - h.pos.x, q.z - h.pos.z);
            let mut l = (dx * dx + dz * dz).sqrt();
            let (dx, dz) = if l <= 0.0 { l = 1.0; (1.0, 0.0) } else { (dx, dz) };
            let k = (h.radius + me.radius + 0.0001) / l;
            q = Vec3::new(dx * k + h.pos.x, q.y, dz * k + h.pos.z);
            pushes += 1;
        }
        if pushes > 0 { return Some(q); }
        if step <= 0.0 { break; }
        t += step;
    }
    None
}

/// The people speed setter `fn_80188214`: displacement per frame for move `amount` (0..2) in direction `dir`.
pub fn speed(amount: f32, walk: f32, run: f32) -> f32 {
    if amount > 0.4 {
        if amount > 1.0 { (amount - 1.0) * (run - walk) + walk }
        else { let half = walk * 0.5; (amount - 0.4) / 0.6 * (walk - half) + half }
    } else { amount / 0.4 * (walk * 0.5) }
}

/// `fn_801887D8`: move amount (0..2) for a per-frame displacement.
pub fn amount_for(v: f32, walk: f32, run: f32) -> f32 {
    if v >= run { 2.0 } else if v < walk { if walk != 0.0 { v / walk } else { 0.0 } } else { 1.0 + (v - walk) / (run - walk) }
}

/// The hero's stick → direction (`fn_8012E388`): the angle off "up" is (π/2)·sin(min(|x|/|y|, 5) / (10/π)),
/// mirrored per quadrant, plus the camera yaw. Returns (amount, Some(direction) if the stick is past the
/// ±1 dead zone).
fn stick_dir(pad: &Pad, cam_rot_y: f32) -> (f32, Option<f32>) {
    // the game reads stick y with up negative; the d-pad stands in for a centred stick
    let mut x = pad.stick.x.round() as i32;
    let mut y = -(pad.stick.y.round() as i32);
    if x == 0 && y == 0 {
        if pad.dpad.y > 0 { y = -56; }
        if pad.dpad.y < 0 { y = 56; }
        if pad.dpad.x < 0 { x = -56; }
        if pad.dpad.x > 0 { x = 56; }
    }
    if x == 0 && y == 0 { return (0.0, None); }
    let (raw_x, raw_y) = (x, y);
    let (x, y) = (x.clamp(-56, 56), y.clamp(-56, 56));
    let fx = x.abs() as f32 / 28.0;
    let fy = y.abs() as f32 / 28.0;
    let amount = (fx * fx + fy * fy).sqrt().min(2.0);
    if (-1..=1).contains(&raw_x) && (-1..=1).contains(&raw_y) { return (amount, None); }
    let base = if fy >= 0.001 { (PI / 2.0) * ((fx / fy).min(5.0) / 3.183_099).sin() } else { PI / 2.0 };
    let mut a = base;
    if y < 0 { a = PI - base; }
    if x < 0 { a = if y < 0 { PI + base } else { PI + (PI - base) }; }
    (amount, Some(a + cam_rot_y))
}

/// What a `heroMoveMain` call asks the runtime to do.
#[derive(Default)]
pub struct Requests {
    pub open_menu: bool,
    /// Talk to this person (`updateChat`).
    pub talk: Option<u32>,
}

/// Inputs from the room for one call.
pub struct Ctx<'a> {
    pub cs: &'a ColSys,
    pub pad: Pad,
    pub cam_rot_y: f32,
    /// Is a box around this point (radius, 17 high) on screen (`fn_800E3C64`)?
    pub on_screen: &'a dyn Fn(Vec3, f32) -> bool,
    /// A floor character's (talk through walls, talk start type) from the map REL; None for other people.
    pub talk_info: &'a dyn Fn(u32) -> Option<(bool, u8)>,
    pub partner_active: bool,
}

pub fn humans(actors: &[Actor]) -> Vec<Human> {
    actors.iter().filter(|a| a.human && a.human_on && a.visible).map(|a| Human { index: a.index, pos: a.pos, radius: a.radius }).collect()
}

fn wrap_pi(a: f32) -> f32 {
    let a = (TAU + a).rem_euclid(TAU);
    if a > PI { a - TAU } else { a }
}

/// `heroMoveMain` (std 31), first part: whether it runs at all this frame (lock frames, hero locked), and
/// the events the last move found (check, touch, pass → interaction point methods 3, 2, 1), which the caller
/// runs synchronously before calling `hero_move`.
pub fn hero_begin(hs: &mut HeroState, actors: &[Actor]) -> Option<Vec<(u8, u16)>> {
    if hs.lock_frames > 0 { hs.lock_frames -= 1; return None; }
    let hero = actors.iter().find(|a| a.index == HERO)?;
    if hero.flags & F_LOCKED != 0 { return None; }
    let mut ev: Vec<(u8, u16)> = hs.check.drain(..).map(|id| (3, id)).collect();
    ev.extend(hs.touch.drain(..).map(|id| (2, id)));
    ev.extend(hs.pass.drain(..).map(|id| (1, id)));
    Some(ev)
}

/// `heroMoveMain` second part: the field menu (X / Y / Start), the partner, then the leader.
pub fn hero_move(hs: &mut HeroState, actors: &mut [Actor], ctx: &Ctx) -> Requests {
    let mut req = Requests::default();
    let Some(hi) = actors.iter().position(|a| a.index == HERO) else { return req };
    if ctx.pad.menu { req.open_menu = true; return req; }
    if ctx.partner_active {
        if let Some(pi) = actors.iter().position(|a| a.index == PARTNER && a.visible) { partner(hs, actors, hi, pi, ctx); }
    }
    move_leader(hs, actors, hi, ctx, &mut req);
    req
}

/// `moveLeader__F15HEROMOVE_MEMBER` 0x8012E7B8.
fn move_leader(hs: &mut HeroState, actors: &mut [Actor], hi: usize, ctx: &Ctx, req: &mut Requests) {
    if ctx.pad.a {
        if let Some(npc) = talk_target(actors, hi, ctx) { req.talk = Some(npc); return; }
        // the check ray: hero + 5 up, 10 ahead along the body's facing
        let h = &actors[hi];
        let a = h.pos + Vec3::Y * 5.0;
        let b = a + Vec3::new(h.yaw.sin() * 10.0, 0.0, h.yaw.cos() * 10.0);
        let ids = ctx.cs.check_regions(a, b);
        if !ids.is_empty() {
            hs.check = ids;
            actors[hi].amount = Some(0.0);
            return;
        }
    }
    let before = actors[hi].pos;
    let (stick_amount, dir) = stick_dir(&ctx.pad, ctx.cam_rot_y);
    // heroReadStick: a direction turns the body toward it at the stick amount (fn_8018805C); no stick at all
    // stops the turn (fn_8018790C)
    match dir {
        Some(d) => actors[hi].start_turn(d, stick_amount),
        None if stick_amount == 0.0 => actors[hi].stop_turn(),
        None => {}
    }
    if stick_amount > 0.0 {
        let hu = humans(actors);
        let h = &mut actors[hi];
        let step = speed(stick_amount, h.walk_speed, h.run_speed);
        let to = before + Vec3::new(h.dir.sin() * step, 0.0, h.dir.cos() * step);
        let m = people_move(ctx.cs, &hu, h, before, to);
        h.pos = m.pos;
        if !m.touch.is_empty() { hs.touch = m.touch; }
        if !m.pass.is_empty() { hs.pass = m.pass; }
    }
    let h = &mut actors[hi];
    let moved = Vec3::new(h.pos.x - before.x, h.pos.y - before.y, h.pos.z - before.z).length();
    let mut amount = amount_for(moved, h.walk_speed, h.run_speed);
    if amount < 0.2 && stick_amount >= 0.2 { amount = 0.2; }
    h.amount = Some(amount);
    // trail: a new point once the leader is more than 12 from the newest
    let lp = h.pos;
    if hs.newest(0).is_none_or(|t| Vec2::new(t.x - lp.x, t.z - lp.z).length() > TRAIL_STEP) { hs.push(lp); }
}

/// `fn_8012DE94`: the partner's frame.
fn partner(hs: &mut HeroState, actors: &mut [Actor], hi: usize, pi: usize, ctx: &Ctx) {
    let leader = actors[hi].pos;
    let (mv, face) = partner_target(hs, actors[pi].pos, leader, hs.follow[1]);
    partner_apply(hs, actors, hi, pi, mv, face, ctx);
    // put back near the leader when 100+ away for 300 frames and off screen: the newest trail point at
    // which it would be off screen
    let me = actors[pi].pos;
    if Vec2::new(leader.x - me.x, leader.z - me.z).length() >= 100.0 {
        hs.timer[1] -= 1.0;
        if hs.timer[1] < 1.0 && !(ctx.on_screen)(me, actors[pi].radius) {
            let mut k = 0;
            while let Some(p) = hs.newest(k) {
                if !(ctx.on_screen)(p, actors[pi].radius) {
                    actors[pi].pos = p;
                    actors[pi].teleported = true;
                    hs.timer[1] = 300.0;
                    break;
                }
                k += 1;
            }
        }
    } else {
        hs.timer[1] = 300.0;
    }
}

/// `fn_8012D7F0`: the partner's move and facing vectors for this frame.
fn partner_target(hs: &HeroState, me: Vec3, leader: Vec3, follow: f32) -> (Vec3, Vec3) {
    let frames = 1.0f32;
    let d = Vec3::new(leader.x - me.x, 0.0, leader.z - me.z);
    let dist = d.length();
    let trail = hs.newest(0);
    let dtrail = trail.map_or(0.0, |t| Vec2::new(me.x - t.x, me.z - t.z).length());
    if trail.is_none() || dist <= dtrail || dist <= 0.01 + follow {
        let scale = if dist <= 0.01 + follow {
            // too close: back away to 9
            if dist >= 9.0 { 0.0 } else { -(9.0 - dist).min(frames) / 9.0 }
        } else {
            (dist - follow).min(frames) / dist
        };
        return (d * scale, d);
    }
    let t = trail.unwrap();
    let tv = Vec3::new(t.x - me.x, 0.0, t.z - me.z);
    let dt = tv.length();
    let mut v = if dt <= frames { tv } else { tv * (frames / dt) };
    let target = me + v;
    if let Some(mut hit) = circle_line(follow, me, target, leader, me) {
        hit.y = me.y;
        let seg = target - me;
        let l2 = seg.length_squared();
        let k = if l2 == 0.0 { 0.0 } else { seg.dot(hit - me) / l2 };
        if (0.0..=1.0).contains(&k) { v = hit - me; }
    }
    (v, v)
}

/// `fn_8012D39C`: where the line a→b meets the circle (radius r, centre c) in x/z: the tangent point, or of
/// two crossings the one nearer `near`. y = 0.
fn circle_line(r: f32, a: Vec3, b: Vec3, c: Vec3, near: Vec3) -> Option<Vec3> {
    let (dz, dx) = (b.z - a.z, b.x - a.x);
    let l2 = dx * dx + dz * dz;
    if l2 < 1e-7 { return None; }
    let k = 1.0 / l2.sqrt();
    let (la, lb, lc) = (-dz * k, dx * k, k * (a.x * b.z - b.x * a.z));
    let f = 1.0 / (la * la + lb * lb);
    let (x0, z0) = (la * (-lc * f), lb * (-lc * f));
    let s = f.sqrt();
    let (ux, uz) = (lb * s, -la * s);
    let uu = ux * ux + uz * uz;
    if uu < 1e-7 { return None; }
    let perp = ux * (c.z - z0) - uz * (c.x - x0);
    let disc = uu * (r * r) - perp * perp;
    if disc < -1e-7 { return None; }
    let proj = ux * (c.x - x0) + uz * (c.z - z0);
    if disc < 1e-7 {
        return Some(Vec3::new(ux * (proj / uu) + x0, 0.0, uz * (proj / uu) + z0));
    }
    let sq = disc.sqrt();
    let (t1, t2) = ((proj - sq) / uu, (proj + sq) / uu);
    let p1 = Vec3::new(ux * t1 + x0, 0.0, uz * t1 + z0);
    let p2 = Vec3::new(ux * t2 + x0, 0.0, uz * t2 + z0);
    let d = |p: Vec3| (near.x - p.x).powi(2) + (near.z - p.z).powi(2);
    Some(if d(p2) <= d(p1) { p2 } else { p1 })
}

/// `fn_8012CA84`: move the partner by `mv`, face along `face`, and pick its animation amount.
fn partner_apply(hs: &HeroState, actors: &mut [Actor], hi: usize, pi: usize, mv: Vec3, face: Vec3, ctx: &Ctx) {
    let frames = 1.0f32;
    let l = (mv.x * mv.x + mv.z * mv.z).sqrt();
    let leader = actors[hi].pos;
    if l <= 0.0 {
        let p = &mut actors[pi];
        let mut amount = 0.0;
        if hs.look[1] == 1 {
            // standing: turn toward the leader once it is outside the head's turning range
            let rot = wrap_pi(p.yaw);
            let ang = (leader.x - p.pos.x).atan2(leader.z - p.pos.z);
            let diff = wrap_pi(ang - rot);
            let (lim_r, lim_l) = (p.head_yaw.0.abs(), p.head_yaw.1.abs());
            let target = if diff >= 0.0 { (diff > lim_r).then(|| wrap_pi(ang - lim_r)) } else { (-diff > lim_l).then(|| { let t = ang + lim_l; if t >= TAU { t - TAU } else { t } }) };
            match target {
                Some(t) if wrap_pi(t - rot).abs() >= 0.001_745_328_9 => { p.start_turn(t, 1.0); amount = 1.0; }
                _ => p.stop_turn(),
            }
        } else {
            p.stop_turn();
        }
        p.amount = Some(amount);
        return;
    }
    let hu = humans(actors);
    let p = &mut actors[pi];
    let before = Vec3::new(p.pos.x, 0.0, p.pos.z);
    let m = people_move(ctx.cs, &hu, p, p.pos, p.pos + Vec3::new(mv.x, 0.0, mv.z));
    p.pos = m.pos;
    let actual = Vec3::new(p.pos.x, 0.0, p.pos.z) - before;
    let k = (l / frames).min(1.0);
    let scaled = actual * (k / l);
    if (face.x * face.x + face.z * face.z).sqrt() > 0.01 { p.start_turn(face.x.atan2(face.z), k); }
    let mut amount = amount_for(scaled.length(), p.walk_speed, p.run_speed);
    if actual.dot(face) < 0.0 { amount = -amount; }
    p.amount = Some(amount);
}

/// `peopleFindTalkTarget` 0x8018CD08 (1.2, 40), called by `updateChat` for the leader. Candidates: every
/// people but the hero that is visible (+0x21), without people flag 1, not a floor character with talk start
/// type 3, and with a CharacterModels entry. Rui (group 0, not a floor character) is one too. The target point
/// is the head part's x/z at the people's y (`peopleGetHeadGroundPos`). Accepted when:
/// - distance ≤ 10·1.2 + hero radius + target radius;
/// - |angle| ≤ 40° from the hero's turn target (+0x40);
/// - a floor character without talk-through-walls (map REL flag 0x10; nearly every NPC has it): no wall in a
///   hero-radius sweep from the hero's feet to the target point (`colsysWallSweep` 0x8010F320);
/// - with Rui in the party: Rui not "between": (Rui − midpoint) rotated by +atan2(target − hero) about Y must have
///   |x| > the target's radius or |z| > half the distance. (The game rotates by +θ, not −θ, so this frame is
///   not aligned with the hero→target line; ported as is.)
///
/// Score = dist²·|angle|·0.5, starting from 10000; lower wins, ties go to the smaller |angle|, then the positive one.
fn talk_target(actors: &[Actor], hi: usize, ctx: &Ctx) -> Option<u32> {
    let h = &actors[hi];
    let reach = 10.0 * 1.2 + h.radius;
    let max_ang = 0.017_453_292 * 40.0;
    let partner = actors.iter().find(|a| a.index == PARTNER && ctx.partner_active);
    let wrap = |a: f32| -> f32 {
        let v = ((TAU as f64 + a as f64) % TAU as f64) as f32;
        if v as f64 > PI as f64 { v - TAU } else if (v as f64) < -(PI as f64) { v + TAU } else { v }
    };
    let (mut best, mut best_ang, mut out) = (10000.0f32, 0.0f32, None);
    for p in actors {
        if p.index == HERO || !p.visible || p.flags & 1 != 0 { continue; }
        if p.index == PARTNER && !ctx.partner_active { continue; }
        let info = if p.index == PARTNER { None } else { (ctx.talk_info)(p.index) };
        if info.is_some_and(|(_, start)| start == 3) { continue; }
        let head = if p.head_part >= 0 { p.head_pos } else { None };
        let target = head.map_or(p.pos, |q| Vec3::new(q.x, p.pos.y, q.z));
        let dist = h.pos.distance(target);
        if dist > reach + p.radius { continue; }
        let to = target - h.pos;
        let ang = wrap(to.x.atan2(to.z) - h.dir);
        if ang.abs() > max_ang { continue; }
        if info.is_some_and(|(thru, _)| !thru) && ctx.cs.wall_sweep(h.radius, h.pos, target).is_some() { continue; }
        if let Some(r) = partner {
            let mid = (h.pos + target) * 0.5;
            let th = wrap(to.x.atan2(to.z));
            let rel = r.pos - mid;
            let (s, c) = th.sin_cos();
            let lx = rel.x * c + rel.z * s;
            let lz = -rel.x * s + rel.z * c;
            if !(p.radius < lx.abs() || mid.distance(target) < lz.abs()) { continue; }
        }
        let score = dist * dist * ang.abs() * 0.5;
        let better = score < best
            || (score == best && (ang.abs() < best_ang.abs() || (ang.abs() == best_ang.abs() && ang > 0.0)));
        if better { best = score; best_ang = ang; out = Some(p.index); }
    }
    out
}

/// `initFloor__Fv` 0x8012F610: the partner starts 9 behind the leader (along its facing), on the floor
/// below (highest floor less than 10 above the leader, else the highest), facing the same way.
pub fn place_partner(cs: &ColSys, leader: Vec3, yaw: f32) -> Vec3 {
    let mut p = Vec3::new(-yaw.sin() * 9.0 + leader.x, leader.y, -yaw.cos() * 9.0 + leader.z);
    let h = cs.floor_heights(p);
    if !h.is_empty() { p.y = pick_floor(&h, leader.y, 10.0, leader.y); }
    p
}

/// `updateAnimation__Ff15HEROMOVE_MEMBER` 0x8012C660: (anim A, optional blend anim B and weight of B, rate)
/// for a move amount, from the model's slots 1..4 (idle, walk, run, back).
pub fn anim_for(amount: f32, slots: &[u8; 8]) -> (usize, Option<(usize, f32)>, f32) {
    let s = |i: usize| if slots[i] == 0xFF { slots[0] as usize } else { slots[i] as usize };
    let (idle, walk, run, back) = (s(0), s(1), s(2), s(3));
    let a = amount.min(2.0);
    if a < -0.4 { (back, None, 1.0) }
    else if a < 0.0 { (back, Some((idle, (a + 0.4) / 0.4)), 0.5) }
    else if a < 0.4 { (walk, Some((idle, 1.0 - 2.5 * a)), 0.5) }
    else if a < 1.0 { (walk, None, 0.5) }
    else { (walk, Some((run, a - 1.0)), 0.5) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stick_up_is_camera_forward() {
        let pad = Pad { stick: Vec2::new(0.0, 100.0), ..default() };
        let (amount, dir) = stick_dir(&pad, 0.0);
        assert_eq!(amount, 2.0);
        assert!((dir.unwrap() - PI).abs() < 1e-5);
        // right: π/2 (toward +x)
        let (_, dir) = stick_dir(&Pad { stick: Vec2::new(100.0, 0.0), ..default() }, 0.0);
        assert!((dir.unwrap() - PI / 2.0).abs() < 1e-5);
        // left: 3π/2
        let (_, dir) = stick_dir(&Pad { stick: Vec2::new(-100.0, 0.0), ..default() }, 0.0);
        assert!((dir.unwrap() - 1.5 * PI).abs() < 1e-5);
    }
    #[test]
    fn follow_backs_off_when_close() {
        let hs = HeroState::default();
        let (mv, _) = partner_target(&hs, Vec3::ZERO, Vec3::new(0.0, 0.0, 5.0), 12.0);
        assert!(mv.z < 0.0);
        let (mv, _) = partner_target(&hs, Vec3::ZERO, Vec3::new(0.0, 0.0, 30.0), 12.0);
        assert!((mv.z - 1.0).abs() < 1e-6);
    }
}
