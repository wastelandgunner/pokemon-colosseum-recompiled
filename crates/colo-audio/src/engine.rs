//! The MusyX engine: one global instance (MusyX itself is a singleton), every call serialized by one lock
//! (`sndPCRender` must not run concurrently with any other `snd*` call). Without [`init`] every call is a
//! no-op that reports failure (invalid handle, not valid), so the game layer above still keeps its books.

use crate::sys;
use std::ffi::c_void;
use std::sync::Mutex;

/// `SND_ID_ERROR` / `SND_SEQ_ERROR_ID`.
pub const ID_ERROR: u32 = 0xFFFF_FFFF;

struct State {
    active: bool,
    channels: u32,
    /// Pushed group data (project, sample directory, pool, samples) stays alive like the game's resources.
    keep: Vec<Vec<u8>>,
}

static ENGINE: Mutex<State> = Mutex::new(State { active: false, channels: 2, keep: Vec::new() });

fn lock() -> std::sync::MutexGuard<'static, State> { ENGINE.lock().unwrap_or_else(|e| e.into_inner()) }

/// Run `f` under the engine lock if the engine is up.
fn call<R>(f: impl FnOnce() -> R) -> Option<R> {
    let g = lock();
    if !g.active { return None; }
    let r = f();
    drop(g);
    Some(r)
}

/// `sndInit(64, 48, 16, 1, 0, 0x9FC000)` with ReverbHI on aux A, rendering at `rate` Hz (the game: 32000)
/// with `channels` 2 (stereo; the game's options toggle stereo/mono through [`output_mode`]).
pub fn init(rate: u32, channels: u32) -> Result<(), String> {
    let mut g = lock();
    if g.active { return Ok(()); }
    let r = unsafe { sys::colo_snd_init(rate, channels) };
    if r != 0 { return Err(format!("MusyX init failed ({r})")); }
    g.active = true;
    g.channels = channels;
    Ok(())
}

pub fn active() -> bool { lock().active }
pub fn channels() -> u32 { lock().channels }

/// `sndPushGroup(proj, gid, samples, sdir, pool)`; the data is kept for the life of the engine.
pub fn push_group(proj: Vec<u8>, gid: u16, samples: Vec<u8>, sdir: Vec<u8>, pool: Vec<u8>) -> bool {
    let mut g = lock();
    if !g.active { return false; }
    let (mut proj, mut samples, mut sdir, mut pool) = (proj, samples, sdir, pool);
    let ok = unsafe {
        sys::colo_snd_push_group(proj.as_mut_ptr() as *mut c_void, gid as u32, samples.as_mut_ptr() as *mut c_void,
                                 sdir.as_mut_ptr() as *mut c_void, pool.as_mut_ptr() as *mut c_void)
    } != 0;
    if ok { g.keep.extend([proj, samples, sdir, pool]); }
    ok
}

/// `sndPCValidateGroup` over the raw GC sections: None if valid, else where and why not.
pub fn validate_group(proj: &[u8], pool: &[u8], sdir: &[u8], samples: &[u8]) -> Option<String> {
    let mut msg = [0u8; 256];
    let ok = unsafe {
        sys::colo_snd_validate_group(proj.as_ptr() as _, proj.len(), pool.as_ptr() as _, pool.len(), sdir.as_ptr() as _, sdir.len(),
                                     samples.as_ptr() as _, samples.len(), msg.as_mut_ptr(), msg.len())
    } != 0;
    if ok { None } else { Some(String::from_utf8_lossy(&msg[..msg.iter().position(|&b| b == 0).unwrap_or(0)]).into_owned()) }
}

/// Fill interleaved s16 frames (`out.len() / channels`). Silence when the engine is down.
pub fn render(out: &mut [i16]) {
    let g = lock();
    if !g.active || out.is_empty() { out.fill(0); return; }
    let frames = out.len() / g.channels as usize;
    if unsafe { sys::colo_snd_render(out.as_mut_ptr(), frames) } == 0 { out.fill(0); }
}

pub fn set_reverb(p: [f32; 6]) { call(|| unsafe { sys::colo_snd_set_reverb(p[0], p[1], p[2], p[3], p[4], p[5]) }); }
/// `sndSeqPlayEx(group, song, data, {SND_PLAYPARA_VOLUME, speed 0x100, volume {fade_ms, vol}}, 0)`.
/// MusyX decodes the arrangement into its own copy, so `data` need not outlive the call here.
pub fn seq_play(group: u32, song: u32, data: &[u8], fade_ms: u32, vol: u32) -> u32 {
    call(|| unsafe { sys::colo_snd_seq_play(group, song, data.as_ptr() as *mut c_void, fade_ms, vol) }).unwrap_or(ID_ERROR)
}
/// `sndSeqVolume(vol, ms, seq, mode)`; mode 1 = SND_SEQVOL_STOP (stop when the fade ends).
pub fn seq_volume(vol: u32, ms: u32, seq: u32, mode: u32) { call(|| unsafe { sys::colo_snd_seq_volume(vol, ms, seq, mode) }); }
pub fn seq_pause(seq: u32) { call(|| unsafe { sys::colo_snd_seq_pause(seq) }); }
pub fn seq_continue(seq: u32) { call(|| unsafe { sys::colo_snd_seq_continue(seq) }); }
pub fn seq_valid(seq: u32) -> bool { call(|| unsafe { sys::colo_snd_seq_valid(seq) } != 0).unwrap_or(false) }
pub fn fx_start(fx: u32, vol: u32, pan: u32, studio: u32) -> u32 {
    call(|| unsafe { sys::colo_snd_fx_start(fx, vol, pan, studio) }).unwrap_or(ID_ERROR)
}
pub fn fx_ctrl(vid: u32, ctrl: u32, value: u32) -> bool { call(|| unsafe { sys::colo_snd_fx_ctrl(vid, ctrl, value) } != 0).unwrap_or(false) }
pub fn fx_keyoff(vid: u32) -> bool { call(|| unsafe { sys::colo_snd_fx_keyoff(vid) } != 0).unwrap_or(false) }
pub fn fx_check(vid: u32) -> u32 { call(|| unsafe { sys::colo_snd_fx_check(vid) }).unwrap_or(ID_ERROR) }
/// `sndVolume(vol, ms, volume group)`: 0xFD music, 0xFE effects, 0xFF both.
pub fn volume(vol: u32, ms: u32, group: u32) { call(|| unsafe { sys::colo_snd_volume(vol, ms, group) }); }
/// `sndOutputMode`: 0 mono, 1 stereo.
pub fn output_mode(mode: u32) { call(|| unsafe { sys::colo_snd_output_mode(mode) }); }
/// Voice/sequence ids that sent a MusyX message (the game's receive-message callback).
pub fn take_messages() -> Vec<u32> {
    call(|| {
        let mut v = vec![0u32; 256];
        let n = unsafe { sys::colo_snd_take_messages(v.as_mut_ptr(), 256) } as usize;
        v.truncate(n);
        v
    }).unwrap_or_default()
}
