//! Headless memory card check: boot from the disc (BOOT_SNAP=1: Dolphin's snapshot at main) with the raw card image `card` (made blank if missing, as `colo-viewer recomp`
//! does), play scripted input for `n` frames, write the card back if the game wrote to it, PNGs of the XFB at the
//! frames in SHOT to OUT (default /tmp/claude-1000).
//! `cargo run --release -p colo-viewer --example cardplay <gcm> <card.raw> <n>`
//! PRESS="frame:buttons[:frames[:stick x:stick y]],..." (buttons hex PAD_BUTTON bits, held 3 frames by default);
//! TAP_A=from-to taps A every 20 frames over that range; TRACE_FNS, FRAMELOG=1 (frame numbers) for debugging.
use colo_recomp::run::{Machine, Pad};
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let card = std::path::PathBuf::from(&a[2]);
    let n: u32 = a.get(3).and_then(|s| s.parse().ok()).unwrap_or(300);
    let dol = "/media/zach/massstorage/Pokemon Colosseum/Working/dolphin";
    let f = std::sync::Mutex::new(std::fs::File::open(&a[1]).unwrap());
    let disc: colo_recomp::run::Disc = Box::new(move |off, buf| { use std::io::{Read, Seek, SeekFrom}; let mut f = f.lock().unwrap(); f.seek(SeekFrom::Start(off)).unwrap(); f.read_exact(buf).unwrap(); });
    // power-on from the disc; BOOT_SNAP=1 boots from Dolphin's snapshot at main instead
    let mut m = if std::env::var("BOOT_SNAP").is_ok() {
        let mut m = Machine::from_snapshot(&std::path::Path::new(dol).join("snaps/boot")).unwrap();
        m.set_disc(disc);
        m
    } else { Machine::from_disc(disc).unwrap() };
    let img = std::fs::read(&card).unwrap_or_else(|_| { let b = m.blank_card(); std::fs::write(&card, &b).unwrap(); eprintln!("made blank card"); b });
    m.insert_card_image(img);
    for a in std::env::var("TRACE_FNS").unwrap_or_default().split(',').filter_map(|x| u32::from_str_radix(x.trim_start_matches("0x"), 16).ok()) { m.c.trace_fns.insert(a); }
    m.boot();
    let press: Vec<(u32, u16, u32, i8, i8)> = std::env::var("PRESS").unwrap_or_default().split(',').filter_map(|x| {
        let p: Vec<&str> = x.split(':').collect();
        Some((p.first()?.parse().ok()?, u16::from_str_radix(p.get(1)?.trim_start_matches("0x"), 16).ok()?,
            p.get(2).and_then(|v| v.parse().ok()).unwrap_or(3), p.get(3).and_then(|v| v.parse().ok()).unwrap_or(0), p.get(4).and_then(|v| v.parse().ok()).unwrap_or(0)))
    }).collect();
    let tap = std::env::var("TAP_A").ok().and_then(|r| { let (x, y) = r.split_once('-')?; Some((x.parse::<u32>().ok()?, y.parse::<u32>().ok()?)) });
    let inst = wgpu::Instance::default();
    let ad = futures_lite::future::block_on(inst.request_adapter(&Default::default())).unwrap();
    let (dev, q) = futures_lite::future::block_on(ad.request_device(&Default::default())).unwrap();
    let mut gpu = colo_gxgpu::Gpu::new(dev, q, 1);
    let mut b = colo_gx::cmd::Builder::new();
    let shots = std::env::var("SHOT").unwrap_or_default();
    let out = std::env::var("OUT").unwrap_or("/tmp/claude-1000".into());
    let mut writes = 0;
    for fr in 0..n {
        let mut p = Pad::default();
        if let Some((x, y)) = tap { if fr >= x && fr < y && fr % 20 >= 10 && fr % 20 < 13 { p.buttons = 0x100; } }
        for &(at, bits, len, sx, sy) in &press { if fr >= at && fr < at + len { p.buttons = bits; p.stick = (sx, sy); } }
        m.set_pad(p);
        if std::env::var("FRAMELOG").is_ok() { eprintln!("F {fr}"); }
        m.frame();
        let mut g = m.gpu_cmds(&mut b); g.finish(); gpu.render(&g);
        if let Some(img) = m.take_card_writes() { writes += 1; std::fs::write(&card, img).unwrap(); if writes == 1 { eprintln!("frame {fr}: first card write"); } }
        if shots.split(',').any(|x| x == fr.to_string()) {
            if let Some((px, w, h)) = gpu.read_xfb() { image::save_buffer(format!("{out}/card{fr}.png"), &px, w, h, image::ColorType::Rgba8).unwrap(); }
        }
    }
    eprintln!("{writes} frames with card writes");
}
