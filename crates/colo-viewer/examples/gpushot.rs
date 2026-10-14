//! Headless check of the recompile's graphics-card path: boot, run `n` frames (A tapped every 20 frames unless
//! NOA), render each with colo-gxgpu, save PNGs of the XFB at frames listed in SHOT to OUT (default /tmp/claude-1000).
//! `cargo run --release -p colo-viewer --example gpushot <gcm> <n>`
use colo_battle::run::{Machine, Pad};
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: u32 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(300);
    let dol = "/media/zach/massstorage/Pokemon Colosseum/Working/dolphin";
    let snap = std::env::var("SNAP").ok();
    let mut m = Machine::from_snapshot(&std::path::Path::new(dol).join(format!("snaps/{}", snap.as_deref().unwrap_or("boot")))).unwrap();
    let f = std::sync::Mutex::new(std::fs::File::open(&a[1]).unwrap());
    m.set_disc(Box::new(move |off, buf| { use std::io::{Read, Seek, SeekFrom}; let mut f = f.lock().unwrap(); f.seek(SeekFrom::Start(off)).unwrap(); f.read_exact(buf).unwrap(); }));
    if snap.is_none() { m.insert_card(&[std::fs::read(format!("{dol}/cards/story-2021.gci")).unwrap()]); m.boot(); }
    let inst = wgpu::Instance::default();
    let ad = futures_lite::future::block_on(inst.request_adapter(&Default::default())).unwrap();
    eprintln!("adapter {:?}", ad.get_info().name);
    let (dev, q) = futures_lite::future::block_on(ad.request_device(&Default::default())).unwrap();
    let mut gpu = colo_gxgpu::Gpu::new(dev, q, std::env::var("COLO_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1));
    let mut b = colo_gx::cmd::Builder::new();
    let shots = std::env::var("SHOT").unwrap_or_default();
    let out = std::env::var("OUT").unwrap_or("/tmp/claude-1000".into());
    let (mut tc, mut tb, mut tr) = (std::time::Duration::ZERO, std::time::Duration::ZERO, std::time::Duration::ZERO);
    for fr in 0..n {
        let bt = if std::env::var("NOA").is_err() && fr % 20 >= 10 && fr % 20 < 13 { 0x100 } else { 0 };
        m.set_pad(Pad { buttons: bt, ..Default::default() });
        let t = std::time::Instant::now(); m.frame(); tc += t.elapsed();
        static AU: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        static PCM: std::sync::Mutex<Vec<i16>> = std::sync::Mutex::new(Vec::new());
        let a = colo_battle::run::take_audio();
        let au = AU.fetch_add(a.len(), std::sync::atomic::Ordering::Relaxed);
        if let Ok(path) = std::env::var("WAV") {
            let mut p = PCM.lock().unwrap(); p.extend(a.chunks(2).flat_map(|x| [x[1], x[0]]));
            if fr + 1 == n { let mut b = Vec::new(); for v in p.iter() { b.extend_from_slice(&v.to_le_bytes()); } std::fs::write(&path, b).unwrap(); }
        } if fr % 100 == 0 { eprintln!("audio: {} stereo frames total, tb {} = {:.3} s", au / 2, colo_battle::run::time_base(&m.c), colo_battle::run::time_base(&m.c) as f64 / 40.5e6); }
        let t = std::time::Instant::now(); let g = if std::env::var("SOFT").is_ok() { m.gpu_cmds_soft(&mut b) } else { m.gpu_cmds(&mut b) }; tb += t.elapsed();
        let t = std::time::Instant::now(); gpu.render(&g); tr += t.elapsed();
        if std::env::var("EFB").is_ok() && shots.split(',').any(|x| x == fr.to_string()) { if let Some((mut px, w, h)) = gpu.read_efb() { for p in px.chunks_mut(4) { p[3] = 255; } image::save_buffer(format!("{out}/efb{fr}.png"), &px, w, h, image::ColorType::Rgba8).unwrap(); } }
        if shots.split(',').any(|x| x == (fr + 1).to_string()) {
            if let Ok(r) = std::env::var("ONLY") { let (a, b) = r.split_once('-').unwrap(); gpu.only = Some(a.parse().unwrap()..b.parse().unwrap()); }
        }
        if shots.split(',').any(|x| x == fr.to_string()) {
            if let Some((px, w, h)) = gpu.read_xfb() { image::save_buffer(format!("{out}/gpu{fr}.png"), &px, w, h, image::ColorType::Rgba8).unwrap(); }
            if std::env::var("SOFT").is_ok() { image::save_buffer(format!("{out}/soft{fr}.png"), &m.display(480), 640, 480, image::ColorType::Rgba8).unwrap();
                let e: Vec<u8> = m.gpu.color.iter().flat_map(|p| [p[0], p[1], p[2], 255]).collect(); image::save_buffer(format!("{out}/softefb{fr}.png"), &e, 640, 528, image::ColorType::Rgba8).unwrap(); }
            if std::env::var("OPS").is_ok() { for (k, o) in g.ops.iter().enumerate() { match o {
                colo_gx::cmd::GOp::Draw { first, count, state } => { let st = &g.states[*state as usize]; let v = &g.verts[*first as usize];
                    eprintln!("{k} draw n{count} p{:?} c{:?} t{:?} tex{:?} st{} cull{} z{:x} cm{:x} af{:x} vp{:?} sc{:?}", v.pos, v.col[0], v.tc[0], st.tex.iter().flatten().collect::<Vec<_>>(), st.nstages, st.cull, st.zmode, st.cmode, st.afunc, st.viewport, st.scissor);
if std::env::var("OPS").unwrap() == k.to_string() { eprintln!("{st:#x?}"); for i in 0..*count as usize { eprintln!("  {:?}", g.verts[*first as usize + i].pos); } } }
                colo_gx::cmd::GOp::Copy(c) => eprintln!("{k} copy {c:?}") } } }
            eprintln!("frame {fr}: {} verts {} ops", g.verts.len(), g.ops.len());
        }
        if fr % 100 == 99 { eprintln!("frame {fr}: cpu {:?} build {:?} render {:?} per frame", tc / 100, tb / 100, tr / 100); tc = Default::default(); tb = tc; tr = tc; }
    }
}
