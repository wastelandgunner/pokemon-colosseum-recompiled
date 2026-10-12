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
    fn fill(scratch: &mut Vec<i16>, frames: usize) {
        scratch.resize(frames * 2, 0);
        engine::render(scratch);
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
