//! FFI to `shim.c` (plain-argument wrappers over the MusyX runtime).
use std::ffi::c_void;

extern "C" {
    pub fn colo_snd_init(rate: u32, channels: u32) -> i32;
    pub fn colo_snd_set_reverb(coloration: f32, mix: f32, time: f32, damping: f32, pre_delay: f32, crosstalk: f32);
    pub fn colo_snd_push_group(proj: *mut c_void, gid: u32, samples: *mut c_void, sdir: *mut c_void, pool: *mut c_void) -> i32;
    pub fn colo_snd_seq_play(gid: u32, sid: u32, data: *mut c_void, fade_ms: u32, vol: u32) -> u32;
    pub fn colo_snd_seq_volume(vol: u32, ms: u32, seq: u32, mode: u32);
    pub fn colo_snd_seq_stop(seq: u32);
    pub fn colo_snd_seq_pause(seq: u32);
    pub fn colo_snd_seq_continue(seq: u32);
    pub fn colo_snd_seq_valid(seq: u32) -> i32;
    pub fn colo_snd_fx_start(fx: u32, vol: u32, pan: u32, studio: u32) -> u32;
    pub fn colo_snd_fx_ctrl(vid: u32, ctrl: u32, value: u32) -> i32;
    pub fn colo_snd_fx_keyoff(vid: u32) -> i32;
    pub fn colo_snd_fx_check(vid: u32) -> u32;
    pub fn colo_snd_volume(vol: u32, ms: u32, group: u32);
    pub fn colo_snd_output_mode(mode: u32);
    pub fn colo_snd_take_messages(out: *mut u32, max: u32) -> u32;
    pub fn colo_snd_validate_group(proj: *const c_void, proj_n: usize, pool: *const c_void, pool_n: usize, sdir: *const c_void, sdir_n: usize,
                                   samp: *const c_void, samp_n: usize, msg: *mut u8, msg_n: usize) -> i32;
    pub fn colo_snd_render(out: *mut i16, frames: usize) -> i32;
}
