//! Per-frame cost of the recompile (CPU, draw-list build, GPU render) against the game time each frame covers,
//! booted like `colo-viewer recomp` (no card unless CARD; WARP=floor[:entry] warps there from the title, warp.rs). PADS="from-to:hexbuttons,..." scripts the pad;
//! SHOT=frames saves XFB PNGs to OUT (MONTAGE: one labelled grid); GXDUMP=frames prints each draw's GX state. Logs a line per LOG frames (default 30).
//! `cargo run --release -p colo-viewer --example movieperf <gcm> <n>`
use colo_recomp::run::{Machine, Pad};
#[path = "../src/warp.rs"] mod warp;
use std::time::{Duration, Instant};
unsafe extern "C" { fn prctl(option: i32, arg: u64, ...) -> i32; }
fn main() {
    // PTRACE: let any debugger attach (gdb sampling under yama ptrace_scope 1)
    if std::env::var("PTRACE").is_ok() { unsafe { prctl(0x59616d61, u64::MAX); } eprintln!("pid {}", std::process::id()); }
    let a: Vec<String> = std::env::args().collect();
    let n: u32 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(600);
    let dol = "/media/zach/massstorage/Pokemon Colosseum/Working/dolphin";
    let mut m = Machine::from_snapshot(&std::path::Path::new(dol).join("snaps/boot")).unwrap();
    let f = std::sync::Mutex::new(std::fs::File::open(&a[1]).unwrap());
    m.set_disc(Box::new(move |off, buf| { use std::io::{Read, Seek, SeekFrom}; let mut f = f.lock().unwrap(); if f.seek(SeekFrom::Start(off)).is_err() || f.read_exact(buf).is_err() { buf.fill(0); } }));
    // CARD: a .gci save, or a raw card image (read only; the game's writes aren't kept)
    if let Ok(c) = std::env::var("CARD") {
        let b = std::fs::read(&c).unwrap();
        if c.ends_with(".raw") { m.insert_card_image(b) } else { m.insert_card(&[b]) }
    }
    m.boot();
    // WARP=floor[:entry] (hex floor id): warp.rs's title-menu warp, its Start/A presses added to PADS
    let warping = std::env::var("WARP").ok().map(|v| { let (f, e) = v.split_once(':').unwrap_or((&v, "0"));
        warp::install(&mut m, (u32::from_str_radix(f.trim_start_matches("0x"), 16).unwrap(), e.parse().unwrap())); }).is_some();
    // FNTIME: time the THP decoder's pieces (host hooks that run the lifted body and add up its time)
    macro_rules! timed { ($i:expr, $f:path, $a:expr) => {{ fn h(c: &mut colo_recomp::ppc::Cpu) { let t = Instant::now(); $f(c, $a); FT[$i].fetch_add(t.elapsed().as_nanos() as u64, std::sync::atomic::Ordering::Relaxed); } h as fn(&mut colo_recomp::ppc::Cpu) }}; }
    static FT: [std::sync::atomic::AtomicU64; 6] = [const { std::sync::atomic::AtomicU64::new(0) }; 6];
    const FN: [&str; 6] = ["VideoDecode", "Row640x480", "HuffY", "HuffU", "HuffV", "AudioDecode"];
    if std::env::var("FNTIME").is_ok() {
        use colo_recomp::lifted::*;
        m.c.hook_fn.insert(0x801e5548, timed!(0, f_801e5548_at, 0x801e5548));
        m.c.hook_fn.insert(0x801e810c, timed!(1, f_801e810c_at, 0x801e810c));
        m.c.hook_fn.insert(0x801eb644, timed!(2, f_801eb644_at, 0x801eb644));
        m.c.hook_fn.insert(0x801ebcc0, timed!(3, f_801ebcc0_at, 0x801ebcc0));
        m.c.hook_fn.insert(0x801ec368, timed!(4, f_801ec368_at, 0x801ec368));
        m.c.hook_fn.insert(0x801ecab0, timed!(5, f_801ecab0_at, 0x801ecab0));
    }
    let pads: Vec<(u32, u32, u16)> = std::env::var("PADS").unwrap_or_default().split(',').filter_map(|s| {
        let (r, b) = s.split_once(':')?; let (x, y) = r.split_once('-')?;
        Some((x.parse().ok()?, y.parse().ok()?, u16::from_str_radix(b, 16).ok()?)) }).collect();
    let inst = wgpu::Instance::default();
    let ad = futures_lite::future::block_on(inst.request_adapter(&Default::default())).unwrap();
    let (dev, q) = futures_lite::future::block_on(ad.request_device(&Default::default())).unwrap();
    let mut gpu = colo_gxgpu::Gpu::new(dev, q, std::env::var("COLO_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(3));
    let mut b = colo_gx::cmd::Builder::new();
    let shots = std::env::var("SHOT").unwrap_or_default();
    let out = std::env::var("OUT").unwrap_or("/tmp/claude-1000".into());
    let log: u32 = std::env::var("LOG").ok().and_then(|v| v.parse().ok()).unwrap_or(30);
    let (mut tc, mut tb, mut tr, mut worst) = (Duration::ZERO, Duration::ZERO, Duration::ZERO, Duration::ZERO);
    let mut tb0 = colo_recomp::run::time_base(&m.c);
    // SAMPLE=from-to: sample the last lifted function entered (Cpu::last_fn) every 50 us over those frames
    let samp: Option<(u32, u32)> = std::env::var("SAMPLE").ok().and_then(|v| { let (a, b) = v.split_once('-')?; Some((a.parse().ok()?, b.parse().ok()?)) });
    static ON: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    let hist = std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::<u32, u64>::new()));
    if samp.is_some() {
        let p = &m.c.last_fn as *const u32 as usize;
        let h = hist.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(Duration::from_micros(50));
            if ON.load(std::sync::atomic::Ordering::Relaxed) {
                // SAFETY: racy read of a u32 the game thread writes; only a sample
                let f = unsafe { std::ptr::read_volatile(p as *const u32) };
                *h.lock().unwrap().entry(f).or_default() += 1;
            }
        });
    }
    let mut tiles: Vec<(u32, image::RgbaImage)> = Vec::new();
    let mut texs: std::collections::HashMap<u64, (u32, u32, Vec<u8>)> = Default::default();
    for fr in 0..n {
        let bt = pads.iter().filter(|p| fr >= p.0 && fr < p.1).fold(0, |a, p| a | p.2);
        let bt = if warping && warp::pending() { bt | warp::buttons(fr) } else { bt };
        m.set_pad(Pad { buttons: bt, ..Default::default() });
        if let Some((x, y)) = samp {
            ON.store(fr >= x && fr < y, std::sync::atomic::Ordering::Relaxed);
            if fr == y {
                let h = hist.lock().unwrap(); let tot: u64 = h.values().sum();
                let mut v: Vec<_> = h.iter().collect(); v.sort_by(|a, b| b.1.cmp(a.1));
                for (f, k) in v.iter().take(25) { eprintln!("  {:#010x} {:5.1}%", f, **k as f64 * 100.0 / tot as f64); }
            }
        }
        let t = Instant::now(); m.frame(); let c = t.elapsed(); tc += c;
        colo_recomp::run::take_audio();
        let gxd = std::env::var("GXDUMP").is_ok_and(|v| v.split(',').any(|x| x == fr.to_string()));
        colo_gx::cmd::DUMP.store(gxd, std::sync::atomic::Ordering::Relaxed);
        if gxd { eprintln!("gx frame {fr}"); }
        // SOFT=1: shot frames also run the software renderer (OUT/soft<frame>.png, the reference)
        let soft = std::env::var("SOFT").is_ok() && shots.split(',').any(|x| x == fr.to_string());
        let t = Instant::now(); let g = if soft { m.gpu_cmds_soft(&mut b) } else if std::env::var("DEFER").is_ok() {
            // DEFER=1: the window's path (vertex work after the frame, `GFrame::finish`)
            let mut g = m.gpu_cmds_deferred(&mut b); g.finish(); g } else { m.gpu_cmds(&mut b) }; let bd = t.elapsed(); tb += bd;
        if soft {
            // the software EFB as the frame ends (its XFB copy may go to a buffer the VI shows later)
            let e: Vec<u8> = m.gpu.color.iter().flat_map(|p| [p[0], p[1], p[2], 255]).collect();
            image::save_buffer(format!("{out}/soft{fr}.png"), &e, 640, 528, image::ColorType::Rgba8).unwrap();
        }
        if gxd { eprintln!("gx frame {fr}: {} ops {} verts", g.ops.len(), g.verts.len()); }
        let t = Instant::now(); gpu.render(&g); let r = t.elapsed(); tr += r;
        worst = worst.max(c + bd + r);
        // OPSTATE=i,j: at GXDUMP frames, print those draw ops' full GState and first vertex
        if std::env::var("OPSTATE").is_ok() { for (h, t) in &g.new_tex { texs.insert(*h, (t.w, t.h, t.levels[0].clone())); } }
        if gxd { for k in std::env::var("OPSTATE").unwrap_or_default().split(',').filter_map(|x| x.parse::<usize>().ok()) {
            if let Some(colo_gx::cmd::GOp::Draw { first, state, .. }) = g.ops.get(k) {
                let st = &g.states[*state as usize];
                eprintln!("opstate {k}: {:#x?}\n  v0 {:?}", st, g.verts[*first as usize]);
                for (m, t) in st.tex.iter().enumerate() { if let Some(colo_gx::cmd::TexId::Mem(h)) = t { if let Some((w, hh, px)) = texs.get(h) {
                    image::save_buffer(format!("{out}/op{k}_tex{m}.png"), px, *w, *hh, image::ColorType::Rgba8).ok();
                } } }
            }
        } }
        // ONLY=a-b;c-d: at shot frames, also render the frame again with only those draw ops (OUT/mp<frame>_<a>.png)
        if shots.split(',').any(|x| x == fr.to_string()) {
            for r in std::env::var("ONLY").unwrap_or_default().split(';').filter_map(|r| { let (a, b) = r.split_once('-')?; Some(a.parse::<usize>().ok()?..b.parse::<usize>().ok()?) }) {
                let a = format!("{}_{}", r.start, r.end);
                gpu.only = Some(r);
                gpu.render(&g);
                if let Some((px, w, h)) = gpu.read_xfb() {
                    let lit = px.chunks(4).filter(|p| p[0] > 8 || p[1] > 8 || p[2] > 8).count();
                    let white = px.chunks(4).filter(|p| p[0] > 240 && p[1] > 240 && p[2] > 240).count();
                    eprintln!("only {a}: {lit} drawn px, {white} white");
                    image::save_buffer(format!("{out}/mp{fr}_{a}.png"), &px, w, h, image::ColorType::Rgba8).unwrap();
                }
                gpu.only = None;
            }
        }
        if shots.split(',').any(|x| x == fr.to_string()) {
            if let Some((px, w, h)) = gpu.read_xfb() {
                // MONTAGE: every shot as a labelled 320x240 tile of OUT/montage.png (3 per row) instead of PNGs
                if std::env::var("MONTAGE").is_ok() {
                    let im = image::RgbaImage::from_raw(w, h, px).unwrap();
                    tiles.push((fr, image::imageops::resize(&im, 320, 240, image::imageops::FilterType::Triangle)));
                } else { image::save_buffer(format!("{out}/mp{fr}.png"), &px, w, h, image::ColorType::Rgba8).unwrap(); }
            }
        }
        if fr % log == log - 1 {
            let t1 = colo_recomp::run::time_base(&m.c);
            let game = Duration::from_secs_f64(t1.saturating_sub(tb0) as f64 / 40.5e6);
            let k = log;
            eprintln!("f{fr}: cpu {:.1} build {:.1} render {:.1} ms/frame, worst {:.1}; game {:.1} ms/frame; wall/game {:.2}",
                tc.as_secs_f64() * 1e3 / k as f64, tb.as_secs_f64() * 1e3 / k as f64, tr.as_secs_f64() * 1e3 / k as f64, worst.as_secs_f64() * 1e3,
                game.as_secs_f64() * 1e3 / k as f64, (tc + tb + tr).as_secs_f64() / game.as_secs_f64().max(1e-9));
            if std::env::var("FNTIME").is_ok() { eprintln!("   {}", FN.iter().zip(FT.iter()).map(|(n, t)| format!("{n} {:.2}", t.swap(0, std::sync::atomic::Ordering::Relaxed) as f64 / 1e6 / k as f64)).collect::<Vec<_>>().join(" ")); }
            tc = Duration::ZERO; tb = tc; tr = tc; worst = tc; tb0 = t1;
        }
    }
    if !tiles.is_empty() {
        let rows = (tiles.len() as u32).div_ceil(3);
        let mut m = image::RgbaImage::new(960, 240 * rows);
        for (i, (fr, t)) in tiles.iter().enumerate() {
            let (x, y) = (320 * (i as u32 % 3), 240 * (i as u32 / 3));
            image::imageops::overlay(&mut m, t, x as i64, y as i64);
            // frame number as a row of marks: one red block per hundred, yellow per ten
            for k in 0..fr / 100 { for dy in 0..6 { for dx in 0..4 { m.put_pixel(x + 2 + 6 * (k % 50) + dx, y + 2 + 8 * (k / 50) + dy, image::Rgba([255, 0, 0, 255])); } } }
        }
        m.save(format!("{out}/montage.png")).unwrap();
        eprintln!("montage: {}", tiles.iter().map(|t| t.0.to_string()).collect::<Vec<_>>().join(" "));
    }
}
