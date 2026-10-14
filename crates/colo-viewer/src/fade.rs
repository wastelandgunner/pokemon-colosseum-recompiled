//! The game's screen fade and its wipe hooks (`fade.cpp` 0x801C4078.., `fade_effect.cpp` 0x801C4814..0x801C6934),
//! ported 1:1 as logic: every game frame `daemon` advances the fade like `fadeDaemon` 0x801C43F4 and returns
//! the quads the hook and the plain fade draw, in the game's 640x480 screen space. `wipe.rs` draws them.
//!
//! State (`FadeState` 0x80466E30): `mode` 0 idle, 1/2 timed fade (drawn as a full-screen colour quad),
//! 3/4 "held" (type bit 3: nothing drawn; mode 3 is timed, 4 waits). Type bit 0: towards the colour
//! (alpha 255·t), else away from it (255 − 255·t); bit 1: black, else white.
//! A hook (`fadeSetEX`) runs every frame after the timer step with (pending, elapsed, duration,
//! hookElapsed, hookDuration, texture) and is dropped when it returns 0. Its texture is a snapshot of the
//! screen (`_fadeSnapshot`, EFB copy, format 0x44 = RGB565); the UDLR/trainer wipes keep re-capturing the
//! screen into it every frame (`GSgfxBeginBackFBCapture` + `GSgfxBackFBDoFrame`) until they end, which
//! gives their smear. Battle wipes: `fightEncountWipe` table 0x80375980 (11 × 0xC: snapshot, SE, time,
//! init function); NOTES.md §16 "Battle start".
//!
//! Checked against Dolphin (`Working/dolphin/wipe_trace.py`, encounter 55): the per-frame mode, pending,
//! elapsed, hookElapsed and the UDLR work (frame, step, value, target) match `traces/wipe55/trace.tsv`.

use bevy::math::Vec2;

/// What a quad samples.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    /// The fade texture (snapshot / continuous capture), at the quad's uv.
    Capture,
    /// `wipe_ball00` (0x0F861200), the ball wipe's sprite.
    Ball0,
    /// The trainer wipe's mask quad: colour from the capture at the pixel's screen position (tex coord 1),
    /// alpha from `wipe_ball01` (0x0F871200) at the quad's uv.
    Mask,
    /// Vertex colour only.
    Solid,
}

/// One textured quad: corner p0 (uv0), edges e1 (to the corner with uv0 + du) and e2 (uv0 + dv). All the
/// fade quads are parallelograms. Colour and alpha are the vertex colour (the same at all four corners).
#[derive(Clone, Copy, Debug)]
pub struct Quad {
    pub p0: Vec2,
    pub e1: Vec2,
    pub e2: Vec2,
    pub uv0: Vec2,
    pub du: Vec2,
    pub dv: Vec2,
    pub rgb: [u8; 3],
    pub alpha: u8,
    pub src: Src,
}

impl Quad {
    /// Corners (TL, TR, BL) as the game emits them, uv (0,0), (1,0), (0,1).
    fn from_corners(tl: Vec2, tr: Vec2, bl: Vec2, alpha: u8, src: Src) -> Quad {
        Quad { p0: tl, e1: tr - tl, e2: bl - tl, uv0: Vec2::ZERO, du: Vec2::X, dv: Vec2::Y, rgb: [255; 3], alpha, src }
    }
    /// `gfxBegin(2)` rectangle (two corners), untextured.
    fn rect(a: Vec2, b: Vec2, rgb: [u8; 3], alpha: u8) -> Quad {
        Quad { p0: a, e1: Vec2::new(b.x - a.x, 0.0), e2: Vec2::new(0.0, b.y - a.y), uv0: Vec2::ZERO, du: Vec2::X, dv: Vec2::Y, rgb, alpha, src: Src::Solid }
    }
}

/// The 2x3 matrix `GSmtx` scale (0x800E042C) → rotate Z (0x800E02E8, PSMTXConcat(m, rot, m)) → translate
/// (0x800E03B4), applied to (x, y, 0).
#[derive(Clone, Copy)]
struct Mtx { a: f32, b: f32, c: f32, d: f32, tx: f32, ty: f32 }

impl Mtx {
    fn srt(scale: f32, rad: f32, pos: Vec2) -> Mtx {
        let (s, c) = rad.sin_cos();
        // [scale]·[rot]: the rows of the scale matrix times the rotation
        Mtx { a: scale * c, b: -scale * s, c: scale * s, d: scale * c, tx: pos.x, ty: pos.y }
    }
    fn apply(&self, x: f32, y: f32) -> Vec2 { Vec2::new(self.a * x + self.b * y + self.tx, self.c * x + self.d * y + self.ty) }
}

/// The wipes' shared work block (`FadeCameraWork` 0x80467030).
#[derive(Clone, Copy, Debug, Default)]
pub struct Work {
    pub frame: u16,
    /// +0xC: UDLR direction (1/2 left/right, 4/8 up/down).
    pub dir: u16,
    pub step: f32,
    pub value: f32,
    pub target: f32,
}

impl Work {
    /// `_fadeEffect_AdjustParms`: step, value and target / 3.
    fn adjust(&mut self) { self.step /= 3.0; self.value /= 3.0; self.target /= 3.0; }
    /// The accelerating move shared by the UDLR, trainer and boss hooks.
    fn advance(&mut self, progress: f32) {
        self.frame = self.frame.wrapping_add(1);
        self.value += self.step;
        if self.step >= 0.0 {
            self.step += 0.5 * progress;
            if self.value >= self.target { self.value = self.target; }
        } else {
            self.step -= 0.5 * progress;
            if self.value <= self.target { self.value = self.target; }
        }
    }
}

/// One point of the ball wipe's two trails (`FadeTrailPoint`, 12 per side at 0x80466E50).
#[derive(Clone, Copy, Debug, Default)]
pub struct TrailPoint { pub pos: Vec2, pub angle: f32, pub alpha: i32 }

/// The installed per-frame hook (the init functions of the wipe table pick one).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hook {
    /// fn_801C5F6C (yoko, mode 2) / fn_801C5ED0 (tate, mode 8): `_fadeEffectFunction_UDLR` + fn_801C6008.
    Udlr(u16),
    /// fn_801C4CB8 (trainer).
    Trainer,
    /// fn_801C5898 (ball).
    Ball,
    /// fn_801C54FC (fadein): only the black fade fn_801C6688.
    FadeIn,
    /// fn_801C5530 (fadeout_in): black fade plus the snapshot fading out.
    FadeOutIn,
    /// fn_801C55D8 (boss): black fade plus the fluid ripple of the snapshot. The fluid
    /// (`fadeFluid*` 0x801C6934..) is not ported; the snapshot is drawn flat (fn_801C6934 approximated).
    Boss,
}

#[derive(Clone, Debug, Default)]
pub struct Fade {
    pub mode: u8,
    pub pending: u8,
    pub ty: u16,
    pub duration: f32,
    pub elapsed: f32,
    pub hook: Option<Hook>,
    /// The fade texture exists (a snapshot was taken).
    pub texture: bool,
    pub hook_duration: f32,
    pub hook_elapsed: f32,
    /// lbl_8047B3B0: the hook's first call is still to come (set by fadeSetFunction via fn_801C6928).
    pub first: bool,
    pub work: Work,
    pub trails: [[TrailPoint; 12]; 2],
    /// The screen is copied into the texture after this frame (a running `GSgfxBeginBackFBCapture`).
    pub capturing: bool,
    /// `_fadeSnapshot` was requested: copy the next displayed frame into the texture.
    pub snapshot: bool,
    /// The last daemon's draw list (hook quads, then the plain fade quad).
    pub quads: Vec<Quad>,
    /// Bumped by every daemon run (one per game frame), so the renderer composites once per game frame.
    pub serial: u64,
    /// Seed of the shared HSD_Rand LCG (the viewer's own; the game's also feeds the particles).
    pub seed: u32,
    /// `_fadeSnapshot` blocks the calling thread until the frame is copied: the hook's init and the
    /// `fadeSet` of a snapshotting `fadeSetEX` happen at the next frame.
    deferred: Option<(u32, u16, f32)>,
    picks: usize,
}

impl Fade {
    pub fn new(seed: u32) -> Self { Fade { seed, ..Default::default() } }

    fn hsd_rand(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(0x343FD).wrapping_add(0x269EC3);
        self.seed >> 16
    }
    /// `_fadeEffectGetRandom` 0x800E0C04: two draws (second high) mod n.
    fn random(&mut self, n: u32) -> u32 {
        // test hook: COLO_WIPE_RAND="0,0" replays the game's picks (first the wipe kind, then the UDLR flip)
        if let Ok(v) = std::env::var("COLO_WIPE_RAND") {
            let picks: Vec<u32> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
            if let Some(&p) = picks.get(self.picks) { self.picks += 1; return p % n; }
        }
        let lo = self.hsd_rand();
        let hi = self.hsd_rand();
        ((hi << 16) | lo) % n
    }

    /// `fadeSet` 0x801C41C8.
    pub fn set(&mut self, ty: u16, duration: f32) {
        self.pending = 1;
        self.ty = ty;
        self.duration = duration;
        self.elapsed = 0.0;
        self.mode = match (ty & 8 != 0, ty & 1 != 0) { (true, true) => 4, (true, false) => 3, (false, true) => 2, (false, false) => 1 };
    }

    /// `fadeSetEX` 0x801C4164 with a wipe init function address: `fadeSetFunction` (reset the hook, snapshot,
    /// run the init), then `fadeSet`.
    pub fn set_ex(&mut self, ty: u16, init: u32, snapshot: bool, duration: f32, hook_duration: f32) {
        if init != 0 {
            self.pending = 1;
            self.ty = ty;
            self.hook_duration = hook_duration;
            self.hook_elapsed = 0.0;
            self.mode = 0;
            self.hook = None;
            self.capturing = false;
            self.texture = snapshot;
            if snapshot {
                self.snapshot = true;
                self.deferred = Some((init, ty, duration));
                return;
            }
            self.install(init);
        }
        self.set(ty, duration);
    }

    /// fn_801C6928 (first-call flag) and the wipe init function.
    fn install(&mut self, init: u32) {
        self.first = true;
        self.hook = Some(match init {
            0x801C483C => Hook::Trainer,
            0x801C4864 => Hook::FadeIn,
            0x801C488C => Hook::FadeOutIn,
            0x801C48B4 => Hook::Udlr(2), // carde: fadeEffectHookFunction_carde → fn_801C5F6C
            0x801C48DC => Hook::Boss,
            0x801C4904 => match self.random(3) { 0 => Hook::Udlr(2), 1 => Hook::Udlr(8), _ => Hook::Ball },
            0x801C4974 => Hook::Ball,
            0x801C499C => match self.random(2) { 0 => Hook::Udlr(2), _ => Hook::Udlr(8) },
            0x801C49F4 => Hook::Udlr(8),
            _ => Hook::Udlr(2), // 0x801C4A1C yoko
        });
    }

    /// A timed fade or a hook is still running (`fadeCheck(0)`).
    pub fn busy(&self) -> bool { self.pending == 1 }
    /// Something is on screen this frame.
    pub fn active(&self) -> bool { !self.quads.is_empty() || self.texture }

    /// `fadeDaemon` 0x801C43F4 for one game frame; `delta` = retraces per game frame (1 at 60 Hz, 2 at 30).
    pub fn daemon(&mut self, delta: u32) {
        let fps = 60.0f32;
        let dt = delta as f32 / fps;
        self.quads.clear();
        self.serial += 1;
        // the frame after the snapshot: the thread resumes in fadeSetFunction (init) and fadeSet
        if self.deferred.is_some() && !self.snapshot {
            let (init, ty, duration) = self.deferred.take().unwrap_or_default();
            self.install(init);
            self.set(ty, duration);
        }
        if self.pending == 1 && matches!(self.mode, 1..=3) {
            self.elapsed += dt;
            if self.elapsed >= self.duration {
                self.elapsed = self.duration;
                self.pending = 0;
                self.mode = if self.ty & 1 != 0 { 2 } else { 0 };
            }
        }
        if let Some(hook) = self.hook {
            self.hook_elapsed += dt;
            if self.hook_elapsed >= self.hook_duration { self.hook_elapsed = self.hook_duration; }
            if self.run_hook(hook) == 0 {
                self.hook = None;
                self.texture = false;
                self.capturing = false;
            }
        }
        if matches!(self.mode, 1 | 2) {
            let progress = if self.duration == 0.0 { 1.0 } else { self.elapsed / self.duration };
            let alpha = if self.ty & 1 != 0 { (255.0 * progress) as u8 } else { (255 - (255.0 * progress) as i32) as u8 };
            let rgb = if self.ty & 2 != 0 { [0, 0, 0] } else { [255, 255, 255] };
            self.quads.push(Quad::rect(Vec2::ZERO, Vec2::new(640.0, 480.0), rgb, alpha));
        }
    }

    fn run_hook(&mut self, hook: Hook) -> u8 {
        let (finish, frame, duration, angle, angle_dur) = (self.pending, self.elapsed, self.duration, self.hook_elapsed, self.hook_duration);
        match hook {
            Hook::Udlr(mode) => {
                if !self.texture { return finish; }
                if self.first { self.first = false; self.udlr_init(mode); }
                self.udlr(finish, frame, duration)
            }
            Hook::Trainer => {
                if !self.texture { return finish; }
                if self.first { self.first = false; self.trainer_init(); }
                self.trainer(finish, frame, duration)
            }
            Hook::Ball => {
                if self.first { self.first = false; self.ball_init(); }
                self.ball(finish, frame, duration, angle, angle_dur)
            }
            Hook::FadeIn => { self.black(frame / duration); finish }
            Hook::FadeOutIn => {
                let t = frame / duration;
                self.black(t);
                if self.texture { self.screen_quad(Vec2::new(320.0, 240.0), 1.0, 0.0, t, angle / angle_dur); }
                finish
            }
            Hook::Boss => {
                if !self.texture { return finish; }
                if self.first { self.first = false; self.boss_init(); }
                let progress = frame / duration;
                self.work.advance(progress);
                self.black(progress);
                if finish == 1 { self.screen_quad(Vec2::new(320.0, 240.0), 1.0, 0.0, progress, angle / angle_dur); }
                finish
            }
        }
    }

    /// fn_801C6688: the black quad, alpha 255 − 255·t.
    fn black(&mut self, t: f32) {
        let alpha = (255 - (255.0 * t) as i32) as u8;
        self.quads.push(Quad::rect(Vec2::ZERO, Vec2::new(640.0, 480.0), [0, 0, 0], alpha));
    }

    /// fn_801C63C0: the fade texture as a 641x481 quad scaled/rotated (degrees) about `pos`; alpha
    /// (s32)(255·lerp(1, 0, blend)) · (1 − fade).
    fn screen_quad(&mut self, pos: Vec2, scale: f32, angle: f32, fade: f32, blend: f32) {
        let x = 1.0 + (0.0 - 1.0) * blend;
        let mut alpha = (255.0 * x) as i32;
        alpha = (alpha as f32 * (1.0 - fade)) as i32;
        let m = Mtx::srt(scale, 0.017453292 * angle, pos);
        self.quads.push(Quad::from_corners(m.apply(-320.0, -240.0), m.apply(321.0, -240.0), m.apply(-320.0, 241.0), alpha as u8, Src::Capture));
    }

    /// `_fadeEffectFunction_UDLR_FirstInit` 0x801C6274.
    fn udlr_init(&mut self, mode: u16) {
        self.work = Work { frame: 0, dir: mode, step: 0.3, target: 128.0, value: 0.0 };
        if self.random(2) == 0 {
            self.work.step *= -1.0;
            self.work.target *= -1.0;
            self.work.dir = match mode { 1 => 2, 2 => 1, 4 => 8, 8 => 4, m => m };
        }
        self.work.adjust();
        self.capturing = true;
    }

    /// fn_801C6008: the snapshot, plus two ghosts sliding along the axis and turning.
    fn udlr(&mut self, finish: u8, frame: f32, duration: f32) -> u8 {
        let progress = frame / duration;
        self.work.advance(progress);
        let c = Vec2::new(320.0, 240.0);
        self.screen_quad(c, 1.0, 0.0, progress, 0.0);
        let w = self.work;
        let mut pos = match w.dir { 1 | 2 => Vec2::new(320.0 + w.value, 240.0), 4 | 8 => Vec2::new(320.0, 240.0 + w.value), _ => c };
        let pulse = w.frame as f32 / (45.0 * (1.0 - progress) + 1.0);
        self.screen_quad(pos, 1.0, pulse, progress, 0.4);
        match w.dir { 1 | 2 => pos.x = 320.0 + 1.5 * w.value, 4 | 8 => pos.y = 240.0 + 1.5 * w.value, _ => {} }
        self.screen_quad(pos, 1.0, pulse * -1.5, progress, 0.4);
        if !self.first && finish == 0 { self.capturing = false; }
        finish
    }

    /// fn_801C53BC.
    fn trainer_init(&mut self) {
        self.work = Work { frame: 0, dir: 0, step: 0.1, target: 16.0, value: 0.0 };
        self.capturing = true;
        self.work.adjust();
    }

    /// fn_801C4CB8: the snapshot with two ghosts moving apart, plus the growing mask quad (wipe_ball01's alpha
    /// over the capture, TEV stages set up in the function).
    fn trainer(&mut self, finish: u8, frame: f32, duration: f32) -> u8 {
        let progress = frame / duration;
        self.work.advance(progress);
        let w = self.work;
        self.screen_quad(Vec2::new(320.0, 240.0), 1.0, 0.0, progress, 0.0);
        self.screen_quad(Vec2::new(320.0 + w.value, 240.0), 1.0, 0.0, progress, 0.5);
        self.screen_quad(Vec2::new(320.0 - w.value, 240.0), 1.0, 0.0, progress, 0.5);
        // the mask quad lives while the vertex descriptor fn_800D7894 allocated exists (freed on the last frame)
        if finish != 0 {
            let s = 1.0 + (w.frame as f32 / 150.0);
            let alpha = (255.0 * (1.0 - progress)) as u8;
            let m = Mtx::srt(s, 0.0, Vec2::new(320.0, 240.0));
            self.quads.push(Quad::from_corners(m.apply(-128.0, -128.0), m.apply(128.0, -128.0), m.apply(-128.0, 128.0), alpha, Src::Mask));
        }
        if finish == 0 { self.capturing = false; }
        finish
    }

    /// fn_801C5D60.
    fn ball_init(&mut self) {
        self.work = Work { frame: 0, dir: 0, step: 4.0, value: 0.0, target: 48.0 };
        let start = [Vec2::new(-128.0, 364.0), Vec2::new(768.0, 116.0)];
        for side in 0..2 {
            for p in 0..12 {
                self.trails[side][p] = TrailPoint { pos: start[side], angle: 0.0, alpha: 0xFF - p as i32 * 0x15 };
            }
        }
        self.work.adjust();
    }

    /// fn_801C5B60: wipe_ball00 as a 256x256 quad.
    fn ball_quad(&mut self, p: TrailPoint, alpha: i32) {
        let m = Mtx::srt(1.0, 0.017453292 * p.angle, p.pos);
        self.quads.push(Quad::from_corners(m.apply(-128.0, -128.0), m.apply(128.0, -128.0), m.apply(-128.0, 128.0), alpha as u8, Src::Ball0));
    }

    /// fn_801C5898: two balls rolling in from the sides, each with an 11-point trail (copied down every
    /// 4th frame), over the black fade and the snapshot.
    fn ball(&mut self, finish: u8, frame: f32, duration: f32, angle: f32, angle_dur: f32) -> u8 {
        let progress = frame / duration;
        let angle_progress = angle / angle_dur;
        let w = &mut self.work;
        w.frame = w.frame.wrapping_add(1);
        w.value += w.step;
        if w.value >= w.target { w.value = w.target; }
        self.black(progress);
        if !self.texture { return finish; }
        self.screen_quad(Vec2::new(320.0, 240.0), 1.0, 0.0, progress, angle_progress);
        let value = self.work.value;
        let pos_step = [value, -value];
        let a0 = 360.0 * (value / 804.24774);
        let ang_step = [a0, -a0];
        let alpha_scale = 1.0 - progress;
        let fr = self.work.frame;
        for i in 0..2 {
            for j in (1..12).rev() {
                if fr & 3 == 0 {
                    let prev = self.trails[i][j - 1];
                    let p = &mut self.trails[i][j];
                    p.pos = prev.pos;
                    p.angle = prev.angle;
                }
                let p = self.trails[i][j];
                self.ball_quad(p, (alpha_scale * p.alpha as f32) as i32);
            }
            let mut v = self.trails[i][0].angle + ang_step[i];
            if v <= 0.0 { v += 360.0 } else if v >= 360.0 { v -= 360.0 }
            self.trails[i][0].angle = v;
            let mut x = self.trails[i][0].pos.x + pos_step[i];
            if x > 896.0 { x = 896.0 } else if x < -256.0 { x = -256.0 }
            self.trails[i][0].pos.x = x;
            let p = self.trails[i][0];
            self.ball_quad(p, (alpha_scale * p.alpha as f32) as i32);
        }
        finish
    }

    /// fn_801C5748 (the fluid grid and its five shocks are not ported).
    fn boss_init(&mut self) {
        self.work = Work { frame: 0, dir: 0, step: 0.25, target: 32.0, value: 0.0 };
        self.work.adjust();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Encounter 55 (wipe 6, yoko picked) as traced in Dolphin (`traces/wipe55/trace.tsv`): fadeSetEX(9, …, 1.0,
    /// 2.5) at frame 2, the field at 60 Hz, `fadeSet(1.0, 8)` at frame 93 with the battle at 30 Hz.
    #[test]
    fn udlr_matches_trace() {
        let mut f = Fade::new(0);
        f.set_ex(9, 0x801C4A1C, true, 1.0, 2.5);
        f.daemon(1); // the snapshot frame: nothing drawn, the thread waits for the copy
        assert_eq!(f.hook, None);
        f.snapshot = false; // the renderer took it
        f.daemon(1);
        assert_eq!(f.work.frame, 1);
        // frame 3 in the trace: work frame 1, value ±0.1, step ±0.1, target ±42.6667
        assert!((f.work.value.abs() - 0.1).abs() < 1e-6 && (f.work.target.abs() - 42.666_668).abs() < 1e-4);
        for _ in 0..89 { f.daemon(1); }
        assert_eq!(f.work.frame, 90);
        assert!((f.work.value.abs() - 9.0).abs() < 1e-3);
        f.set(8, 1.0);
        let mut ended = 0;
        for i in 0..40 {
            f.daemon(2);
            if i == 0 { assert!((f.elapsed - 0.0333).abs() < 1e-3 && (f.work.step.abs() - 0.1167).abs() < 1e-3); }
            if f.hook.is_none() { ended = i; break; }
        }
        // trace: 29 hook frames in mode 3 (93..121), the hook drops on the 30th (122)
        assert_eq!(ended, 29);
        assert_eq!(f.mode, 0);
    }
}
