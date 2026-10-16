//! Play a story battle snapshot headless the way the viewer does (`gamebattle.rs`), tapping A, until the game pops
//! its battle floor: `cargo run --release -p colo-viewer --example storybattle <snapdir> [max frames]`.
//! Prints the floor depth and `battleGetResult` at the end; SHOT=dir saves a PPM every 300 frames (SHOT_EVERY=n).
/// MusyX `salCallback` (0x80163EE0) calls: one per 5 ms AI DMA block, 6.7 per 30 Hz battle frame.
static SYNTH: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let dir = std::path::PathBuf::from(&a[1]);
    let max: u32 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(20000);
    let mut m = colo_recomp::run::Machine::from_snapshot(&dir).expect("snapshot");
    let gcm = std::env::var("GCM").unwrap_or("/media/zach/massstorage/ROMs/Nintendo/Gamecube/iph-pmc.gcm".into());
    let f = std::sync::Mutex::new(std::fs::File::open(gcm).expect("disc image"));
    m.set_disc(Box::new(move |off, buf| {
        use std::io::{Read, Seek, SeekFrom};
        let mut f = f.lock().unwrap();
        if f.seek(SeekFrom::Start(off)).is_err() || f.read_exact(buf).is_err() { buf.fill(0); }
    }));
    // FANFARE=1: log soundPlayFanfare (0x801653CC), the fanfare's soundStart and song load, every 300th
    // soundGetStatus (0x801666BC) result, and count salCallback
    if std::env::var("FANFARE").is_ok() {
        fn fanfare(c: &mut colo_recomp::ppc::Cpu) {
            println!("  soundPlayFanfare({:#x}, {:#x}, {:#x})", c.r[3], c.r[4], c.r[5]);
            c.hook_fn.remove(&0x801653CC); colo_recomp::lifted::dispatch(c, 0x801653CC); c.hook_fn.insert(0x801653CC, fanfare);
        }
        fn status(c: &mut colo_recomp::ppc::Cpu) {
            static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let id = c.r[3];
            c.hook_fn.remove(&0x801666BC); colo_recomp::lifted::dispatch(c, 0x801666BC); c.hook_fn.insert(0x801666BC, status);
            if N.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % 300 == 0 { println!("  soundGetStatus({id:#x}) = {}", c.r[3]); }
        }
        fn start(c: &mut colo_recomp::ppc::Cpu) {
            let id = c.r[3];
            if id == 0x3F5 {
                let tbl = c.m.u32(0x80478FAC);
                let w: Vec<u32> = (0..3).map(|i| c.m.u32(tbl + id * 0xC + 4 * i)).collect();
                println!("  soundStart({id:#x}) work {w:x?}");
                for &p in &w { if (0x8000_0000..0x8180_0000).contains(&p) { println!("    {p:#x}: {:02x?}", c.m.read(p, 48)); } }
            }
            c.hook_fn.remove(&0x80166A50); colo_recomp::lifted::dispatch(c, 0x80166A50); c.hook_fn.insert(0x80166A50, start);
        }
        m.c.hook_fn.insert(0x80166A50, start);
        // the song loader (0x80165EE0: 0xE0-fill dst, then resource 0x99 copies the song in): dump dst after it
        fn load(c: &mut colo_recomp::ppc::Cpu) {
            let (src, dst) = (c.r[3], c.r[4]);
            c.hook_fn.remove(&0x80165EE0); colo_recomp::lifted::dispatch(c, 0x80165EE0); c.hook_fn.insert(0x80165EE0, load);
            println!("  song load src {src:#x} -> {dst:#x}: {:02x?}", c.m.read(dst, 64));
        }
        m.c.hook_fn.insert(0x80165EE0, load);
        fn synth(c: &mut colo_recomp::ppc::Cpu) {
            SYNTH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            c.hook_fn.remove(&0x80163EE0); colo_recomp::lifted::dispatch(c, 0x80163EE0); c.hook_fn.insert(0x80163EE0, synth);
        }
        m.c.hook_fn.insert(0x80163EE0, synth);
        m.c.hook_fn.insert(0x801653CC, fanfare);
        m.c.hook_fn.insert(0x801666BC, status);
    }
    let shots = std::env::var("SHOT").ok();
    let every: u32 = std::env::var("SHOT_EVERY").ok().and_then(|v| v.parse().ok()).unwrap_or(300);
    let mut was = m.floor_depth();
    let mut builder = colo_gx::cmd::Builder::new();
    for fr in 0..max {
        let b = if fr % 20 >= 10 && fr % 20 < 13 { 0x100 } else { 0 };
        m.set_pad(colo_recomp::run::Pad { buttons: b, ..Default::default() });
        let t0 = std::time::Instant::now();
        m.frame();
        let t1 = t0.elapsed().as_secs_f64();
        let shot = shots.is_some() && fr % every == 0;
        // the GX FIFO is only rasterized for the frames that are saved (NOGPU=1: never)
        if std::env::var("GXOPS").is_ok() {
            // per frame: draws, and each EFB copy (dst, to XFB, source rect, clear)
            let g = m.gpu_cmds_soft(&mut builder);
            // EFBDRAWS=frame: the draws sampling an EFB copy that frame: texture slot, copy, and a few vertices
            if std::env::var("EFBDRAWS").ok().and_then(|v| v.parse::<u32>().ok()) == Some(fr) {
                for op in &g.ops {
                    if let colo_gx::cmd::GOp::Draw { first, count, state } = op {
                        let st = &g.states[*state as usize];
                        for (k, t) in st.tex.iter().enumerate() {
                            if let Some(colo_gx::cmd::TexId::Efb(id, fmt)) = t {
                                let vs: Vec<_> = (0..(*count).min(4)).map(|i| { let v = &g.verts[(*first + i) as usize]; (v.pos, v.tc[k]) }).collect();
                                println!("  draw {count} verts, slot {k} copy {id} fmt {fmt}: {vs:?}");
                            }
                        }
                    }
                }
            }
            let mut draws = 0;
            for op in &g.ops {
                match op {
                    colo_gx::cmd::GOp::Copy(c) => { if draws > 0 { print!(" [{draws} draws]"); draws = 0; } print!(" copy {:#x}{} {:?} clr {}", c.dst, if c.to_xfb { " xfb" } else { "" }, c.src, c.clear.is_some()); }
                    _ => draws += 1,
                }
            }
            println!(" [{draws} draws]  <- frame {fr}");
        } else if std::env::var("GPUCMDS").is_ok() {
            // the viewer's battle path: the graphics-card draw list (built, not drawn)
            let tb = std::time::Instant::now();
            let g = m.gpu_cmds(&mut builder);
            let t2 = tb.elapsed().as_secs_f64();
            if fr % 60 == 0 { println!("frame {fr}: game {:.1} ms, draw list {:.1} ms ({} ops, {} verts, {} new textures)", t1 * 1e3, t2 * 1e3, g.ops.len(), g.verts.len(), g.new_tex.len()); }
        } else if shot && std::env::var("NOGPU").is_err() { m.gpu_frame(); } else { m.take_gx(); }
        let _ = colo_recomp::run::take_audio();
        if let (Some(d), true) = (&shots, shot) {
            let img = m.display(480);
            let mut ppm = b"P6 640 480 255\n".to_vec();
            for p in img.chunks(4) { ppm.extend_from_slice(&p[..3]); }
            std::fs::write(format!("{d}/sb{fr:05}.ppm"), ppm).ok();
            // DUMPTEX="addr:fmt:w:h,...": decode those RAM textures (e.g. EFB copies the software renderer wrote) too
            for spec in std::env::var("DUMPTEX").unwrap_or_default().split(',').filter(|x| !x.is_empty()) {
                let v: Vec<u32> = spec.split(':').filter_map(|x| u32::from_str_radix(x.trim_start_matches("0x"), 16).ok()).collect();
                if v.len() != 4 { continue; }
                let n = colo_formats::gx::data_size(v[1], v[2] as usize, v[3] as usize).unwrap_or(0);
                if let Ok(px) = colo_formats::gx::decode(m.c.m.read(v[0], n as u32), v[1], v[2] as usize, v[3] as usize, None) {
                    let mut ppm = format!("P6 {} {} 255\n", v[2], v[3]).into_bytes();
                    for p in px.chunks(4) { ppm.extend_from_slice(&p[..3]); }
                    std::fs::write(format!("{d}/tex{:x}_{fr:05}.ppm", v[0]), ppm).ok();
                }
            }
        }
        if std::env::var("FADETEX").is_ok() {
            // the fade texture (fadeState +0x10 -> GStexture +0x28): mean of its RGB565 texels, and the XFB shown
            let t = m.c.m.u32(0x8046_6E40);
            if t != 0 {
                let a = m.c.m.u32(t + 0x28);
                let b = m.c.m.read(a, 640 * 480 * 2);
                let s: u64 = b.chunks(2).map(|p| { let v = u16::from_be_bytes([p[0], p[1]]); ((v >> 11) as u64 * 8 + (v >> 5 & 63) as u64 * 4 + (v & 31) as u64 * 8) / 3 }).sum();
                println!("frame {fr}: fade texture {a:#x} mean {}", s / (640 * 480));
            }
        }
        let d = m.floor_depth();
        if was == 1 && d == 0 {
            println!("frame {fr}: battle floor popped, battleGetResult {}", m.battle_result());
            return;
        }
        was = d;
        if fr % 600 == 0 { println!("frame {fr}: floor depth {d} threads {:x?} AI callbacks {}", colo_recomp::run::threads(), SYNTH.load(std::sync::atomic::Ordering::Relaxed)); }
    }
    println!("no pop after {max} frames (floor depth {})", m.floor_depth());
}
