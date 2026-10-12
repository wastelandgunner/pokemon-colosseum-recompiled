//! `sound`: play a sound id through the game's sound layer and MusyX, rendered offline to a WAV.

use crate::{Src, R};
use colo_audio::{engine, layer, SoundData, SoundLayer, SoundTask};
use colo_formats::common::Common;
use colo_formats::fsys::{FileType, Fsys};
use std::path::Path;

/// The game's mixer rate.
const RATE: u32 = 32000;

/// Engine up, banks pushed, songs loaded, the layer installed. Returns the tables' sound count.
pub fn boot(src: &mut Src) -> R<usize> {
    let cf = src.read("common.fsys")?;
    let common = Fsys::parse(&cf)?;
    let rel = common.read(common.find("common_rel", FileType::Rel).ok_or("common_rel not found")?)?;
    let t = Common::parse(&rel)?.sound_tables()?;
    engine::init(RATE, 2)?;
    let errs = colo_audio::push_banks(&t, &common, |p| src_read(src, p.trim_start_matches('/')));
    for e in &errs { eprintln!("{e}"); }
    let mut data = SoundData::new(&t);
    let bgm = src.read("bgm_archive.fsys")?;
    data.songs = colo_audio::bgm_archive(&Fsys::parse(&bgm)?);
    let n = data.sounds.len();
    layer::install(SoundLayer::new(data));
    Ok(n)
}

fn src_read(src: &Src, path: &str) -> Option<Vec<u8>> {
    // Src::read needs &mut for discs; re-open through a raw pointer-free path: copy the logic
    match src {
        Src::Dir(p) => std::fs::read(p.join(path)).or_else(|_| std::fs::read(p.join("files").join(path))).ok(),
        Src::Disc(_) => None,
    }
}

/// `sound <disc|dir> <id> <out.wav> [seconds] [sub]`
pub fn cmd_sound(path: &str, id: &str, out: &str, opts: &[String]) -> R<()> {
    let mut src = if Path::new(path).is_dir() { Src::Dir(path.into()) } else { Src::Disc(colo_formats::gcm::Disc::open(path)?) };
    if let Src::Disc(d) = &mut src {
        // stage the banks from the disc image
        let dir = std::env::temp_dir().join("colo-sound");
        std::fs::create_dir_all(dir.join("sound"))?;
        for f in ["common.fsys", "bgm_archive.fsys", "sound/snd_music.samp", "sound/snd_music_atmos.samp", "sound/snd_se.samp",
                  "sound/snd_se_battle.samp", "sound/snd_se_motion.samp", "sound/snd_se_nakigoe.samp", "sound/snd_se_nakigoe2.samp"] {
            std::fs::write(dir.join(f), d.read(f)?)?;
        }
        src = Src::Dir(dir);
    }
    let id: u32 = id.parse()?;
    let secs: f32 = opts.first().and_then(|s| s.parse().ok()).unwrap_or(20.0);
    let sub = opts.iter().any(|o| o == "sub");
    let n = boot(&mut src)?;
    println!("{n} sounds; playing {id} ({}) for {secs} s", layer::with(|l| format!("{:?}", l.entry(id))));
    let mut task = if sub { SoundTask::play_sub(id, 0, 127) } else { SoundTask::set_bgm(id, 0, 127) };
    let frames_per_tick = RATE as usize / 60;
    let ticks = (secs * 60.0) as usize;
    let mut pcm: Vec<i16> = Vec::with_capacity(ticks * frames_per_tick * 2);
    let mut buf = vec![0i16; (frames_per_tick + 1) * 2];
    let mut done = false;
    let mut acc = 0usize;
    for t in 0..ticks {
        if !done {
            if let Some(r) = layer::with(|l| { l.service(); task.poll(l) }) { println!("returned {r} after {t} frames"); done = true; }
        } else { layer::with(|l| l.service()); }
        // 32000/60 frames per tick, the remainder spread
        acc += RATE as usize;
        let f = acc / 60;
        acc %= 60;
        engine::render(&mut buf[..f * 2]);
        pcm.extend_from_slice(&buf[..f * 2]);
    }
    let status = layer::with(|l| l.status(id));
    let peak = pcm.iter().map(|s| s.unsigned_abs()).max().unwrap_or(0);
    let rms = (pcm.iter().map(|&s| (s as f64).powi(2)).sum::<f64>() / pcm.len().max(1) as f64).sqrt();
    println!("status {status}, peak {peak}, rms {rms:.0}, cur bgm {} sub {}", layer::with(|l| l.cur_bgm), layer::with(|l| l.cur_sub));
    write_wav(out, &pcm, RATE, 2)?;
    println!("wrote {out}");
    Ok(())
}

fn write_wav(path: &str, pcm: &[i16], rate: u32, ch: u16) -> R<()> {
    let mut b = Vec::with_capacity(44 + pcm.len() * 2);
    let data = (pcm.len() * 2) as u32;
    b.extend_from_slice(b"RIFF");
    b.extend_from_slice(&(36 + data).to_le_bytes());
    b.extend_from_slice(b"WAVEfmt ");
    b.extend_from_slice(&16u32.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&ch.to_le_bytes());
    b.extend_from_slice(&rate.to_le_bytes());
    b.extend_from_slice(&(rate * ch as u32 * 2).to_le_bytes());
    b.extend_from_slice(&(ch * 2).to_le_bytes());
    b.extend_from_slice(&16u16.to_le_bytes());
    b.extend_from_slice(b"data");
    b.extend_from_slice(&data.to_le_bytes());
    for s in pcm { b.extend_from_slice(&s.to_le_bytes()); }
    std::fs::write(path, b)?;
    Ok(())
}
