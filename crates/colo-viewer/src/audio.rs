//! Sound output: MusyX (colo-audio) rendered on its own output stream, the game's sound layer driven by the
//! script natives (script.rs), and the UI sounds (message key wait, Yes/No menu).
//!
//! The game mixes at 32 kHz; the stream asks for 32000 Hz stereo and falls back to the device's default
//! rate (MusyX's PC renderer resamples). With no output device the engine still runs, rendered and
//! discarded in real time, so sequence lifetimes (and `soundGetCurrentBackgroundMusic`) behave the same.

use crate::world::Source;
use colo_audio::{engine, layer, SoundData, SoundLayer};
use colo_formats::common::Common;
use colo_formats::fsys::Fsys;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// The game's mixer rate.
pub const GAME_RATE: u32 = 32000;

/// Boot the sound system like the game (sndInit, push the seven banks, load bgm_archive). Never fails the
/// viewer: problems are printed and the layer runs silent.
pub fn boot(source: &mut Source, common: &Common, common_fsys: &Fsys) {
    let tables = match common.sound_tables() {
        Ok(t) => t,
        Err(e) => { eprintln!("sound: tables: {e}"); return; }
    };
    let mut data = SoundData::new(&tables);
    let voices = common.species_voices().unwrap_or_default();
    if std::env::var("COLO_NO_AUDIO").is_ok() {
        println!("sound: off (COLO_NO_AUDIO)");
        layer::install(SoundLayer::new(data));
        CRIES.set(voices).ok();
        return;
    }
    let rate = start_output();
    if let Err(e) = engine::init(rate, 2) { eprintln!("sound: {e}"); }
    let errs = colo_audio::push_banks(&tables, common_fsys, |p| source.read(p.trim_start_matches('/')).ok());
    for e in &errs { eprintln!("sound: {e}"); }
    match source.read("bgm_archive.fsys").map_err(|e| e.to_string()).and_then(|b| Fsys::parse(&b).map(|f| colo_audio::bgm_archive(&f)).map_err(|e| e.to_string())) {
        Ok(s) => data.songs = s,
        Err(e) => eprintln!("sound: bgm_archive.fsys: {e}"),
    }
    println!("sound: {} ids, {} songs, mixing at {rate} Hz", data.sounds.len(), data.songs.len());
    layer::install(SoundLayer::new(data));
    CRIES.set(voices).ok();
}

static CRIES: std::sync::OnceLock<Vec<u16>> = std::sync::OnceLock::new();

/// std 218 `soundPlayPokemonCry(species)` (0x801CAE80 → `etctoolSetPokemonNakigoe` 0x801E0F78): the species'
/// voice id (species table +0xE), started at its stored volume (FUN_80166AB8(id, 0, 0)).
pub fn cry(species: u32) {
    let Some(&id) = CRIES.get().and_then(|v| v.get(species as usize)) else { return };
    layer::with(|l| { l.start_raw(id as u32, 0); });
}

/// FUN_80166A28(id): start a sound at its stored volume (the message key-wait sound 0x24, menu sounds).
pub fn se(id: u32) { layer::with(|l| { l.start_raw(id, 0); }); }

/// FUN_801669BC(id): key the sound off (no fade), keep its work.
pub fn se_stop(id: u32) { layer::with(|l| { l.stop_raw(id); }); }

/// Text-scroll sound by message window talk-SE type (win+3), from fn_800FC7E0 at 0x800FD15C: 1 → 87,
/// 2 → 88, 3 → 89, 4 → 1175, 5 → 1176, else none. The repeat (~70 ms) is in the MusyX macro.
pub fn talk_se(kind: u8) -> u32 {
    match kind { 1 => 0x57, 2 => 0x58, 3 => 0x59, 4 => 0x497, 5 => 0x498, _ => 0 }
}

/// Message key wait (`msgctrlKeyWait`): A pressed on the arrow plays sound 36.
pub const SE_MSG_KEY: u32 = 36;
/// Menu SE set 1 (menu_se_sets: kind 1 = slot 2, kind 2 = slot 3): cursor 35, decide 36.
pub const SE_CURSOR: u32 = 35;
pub const SE_DECIDE: u32 = 36;

/// While set, the output plays this queue (interleaved L, R at 32 kHz) instead of the viewer's sound engine:
/// the game's own mix during a battle run on its code (`gamebattle.rs`).
static EXTERNAL: std::sync::Mutex<Option<std::collections::VecDeque<i16>>> = std::sync::Mutex::new(None);

/// Open the output for external audio only (recomp mode: the game's mix, no viewer engine).
pub fn start_external() { external(true); start_output(); }

pub fn external(on: bool) { *EXTERNAL.lock().unwrap() = if on { Some(Default::default()) } else { None }; }

static DEVICE_RATE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(GAME_RATE);

/// Resampler / buffering state for the external stream: playing (after the prebuffer), fractional position.
struct ExtState { playing: bool, pos: f64 }
static EXT_STATE: std::sync::Mutex<ExtState> = std::sync::Mutex::new(ExtState { playing: false, pos: 0.0 });

/// The recompiled game's audio DMA rate: the GameCube's "32 kHz", 108 MHz / 3372 (colo-recomp ai.rs).
const AI_RATE: f64 = 108_000_000.0 / 3372.0;

/// The recompiled game's AI output (AI_RATE stereo) from the queue to the device rate (linear interpolation). Playback starts once ~100 ms are
/// queued and pauses (silence) to rebuffer on an underrun, so a slow frame costs a gap only once instead of
/// tearing every buffer.
fn external_fill(q: &mut std::collections::VecDeque<i16>, out: &mut [i16]) {
    let rate = DEVICE_RATE.load(std::sync::atomic::Ordering::Relaxed).max(1) as f64;
    let step = AI_RATE / rate;
    let mut st = EXT_STATE.lock().unwrap();
    let need = ((out.len() / 2) as f64 * step) as usize + 2;
    if !st.playing && q.len() / 2 >= (GAME_RATE as usize / 10).max(need) { st.playing = true; }
    if !st.playing || q.len() / 2 < need { st.playing = false; out.fill(0); return; }
    for f in out.chunks_mut(2) {
        let i = st.pos as usize;
        let t = (st.pos - i as f64) as f32;
        for c in 0..2 {
            let a = q.get(2 * i + c).copied().unwrap_or(0) as f32;
            let b = q.get(2 * i + 2 + c).copied().unwrap_or(a as i16) as f32;
            f[c] = (a + (b - a) * t) as i16;
        }
        st.pos += step;
    }
    let used = st.pos as usize;
    st.pos -= used as f64;
    for _ in 0..(2 * used).min(q.len()) { q.pop_front(); }
}

/// Volume of the external stream, 0-100.
static VOLUME: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(100);
pub fn set_volume(v: u32) { VOLUME.store(v.min(100), std::sync::atomic::Ordering::Relaxed); }

/// Queue game audio (L, R pairs); keeps at most ~200 ms queued so latency stays bounded.
pub fn push_external(pcm: &[i16]) {
    if let Some(q) = EXTERNAL.lock().unwrap().as_mut() {
        let v = VOLUME.load(std::sync::atomic::Ordering::Relaxed) as i32;
        if v == 100 { q.extend(pcm.iter().copied()); } else { q.extend(pcm.iter().map(|&x| (x as i32 * v / 100) as i16)); }
        let max = 32000 / 4 * 2;
        while q.len() > max { q.pop_front(); }
    }
}

/// Open the output stream on its own thread (cpal streams are not Send everywhere); returns the mix rate.
fn start_output() -> u32 {
    let (tx, rx) = std::sync::mpsc::channel::<u32>();
    std::thread::Builder::new().name("colo-audio".into()).spawn(move || {
        match open_stream() {
            Ok((stream, rate)) => {
                let _ = tx.send(rate);
                if let Err(e) = stream.play() { eprintln!("sound: play: {e}"); }
                loop { std::thread::park(); }
            }
            Err(e) => {
                eprintln!("sound: no output ({e}); rendering silently");
                let _ = tx.send(GAME_RATE);
                // keep the engine's clock running: 5 ms cycles, discarded
                let mut buf = vec![0i16; (GAME_RATE as usize / 200) * 2];
                let start = std::time::Instant::now();
                let mut done: u64 = 0;
                loop {
                    let due = (start.elapsed().as_secs_f64() * GAME_RATE as f64) as u64;
                    while done + (buf.len() / 2) as u64 <= due { engine::render(&mut buf); done += (buf.len() / 2) as u64; }
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
            }
        }
    }).expect("audio thread");
    rx.recv().unwrap_or(GAME_RATE)
}

fn open_stream() -> Result<(cpal::Stream, u32), String> {
    let host = cpal::default_host();
    let dev = host.default_output_device().ok_or("no output device")?;
    let ranges: Vec<_> = dev.supported_output_configs().map_err(|e| e.to_string())?.collect();
    // 32000 Hz if the device takes it (prefer stereo i16, then f32), else its default
    let pick = |fmt: cpal::SampleFormat| ranges.iter().find(|r| r.sample_format() == fmt && r.channels() >= 2
        && r.min_sample_rate().0 <= GAME_RATE && r.max_sample_rate().0 >= GAME_RATE).cloned();
    let cfg = match pick(cpal::SampleFormat::I16).or_else(|| pick(cpal::SampleFormat::F32)) {
        Some(r) => r.with_sample_rate(cpal::SampleRate(GAME_RATE)),
        None => dev.default_output_config().map_err(|e| e.to_string())?,
    };
    let rate = cfg.sample_rate().0;
    let ch = cfg.channels() as usize;
    let fmt = cfg.sample_format();
    let config: cpal::StreamConfig = cfg.into();
    let err = |e| eprintln!("sound: stream: {e}");
    DEVICE_RATE.store(rate, std::sync::atomic::Ordering::Relaxed);
    fn fill(scratch: &mut Vec<i16>, frames: usize) {
        scratch.resize(frames * 2, 0);
        let mut ext = EXTERNAL.lock().unwrap();
        match ext.as_mut() {
            Some(q) => external_fill(q, scratch),
            None => engine::render(scratch),
        }
    }
    let mut scratch: Vec<i16> = vec![];
    let stream = match fmt {
        cpal::SampleFormat::I16 => dev.build_output_stream(&config, move |out: &mut [i16], _| {
            fill(&mut scratch, out.len() / ch);
            let src = &scratch;
            for (f, s) in out.chunks_mut(ch).zip(src.chunks(2)) { for (c, o) in f.iter_mut().enumerate() { *o = if c < 2 { s[c] } else { 0 }; } }
        }, err, None),
        cpal::SampleFormat::F32 => dev.build_output_stream(&config, move |out: &mut [f32], _| {
            fill(&mut scratch, out.len() / ch);
            let src = &scratch;
            for (f, s) in out.chunks_mut(ch).zip(src.chunks(2)) { for (c, o) in f.iter_mut().enumerate() { *o = if c < 2 { s[c] as f32 / 32768.0 } else { 0.0 }; } }
        }, err, None),
        other => return Err(format!("unsupported sample format {other:?}")),
    }.map_err(|e| e.to_string())?;
    Ok((stream, rate))
}
