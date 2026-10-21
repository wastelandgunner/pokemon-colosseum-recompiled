//! Play a story battle snapshot headless the way the viewer does (`gamebattle.rs`), tapping A, until the game pops
//! its battle floor: `cargo run --release -p colo-viewer --example storybattle <snapdir> [max frames]`.
//! Prints the floor depth and `battleGetResult` at the end, and the game's rumble commands (PADControlMotor) as they change; SHOT=dir saves a PPM every 300 frames (SHOT_EVERY=n).
//! Timing: GPUCMDS=1 builds the viewer's draw list (game / draw-list ms every 60 frames; FTIME=ms also every frame over
//! that; GXTIME=1 splits out GX decode and vertex work), PROFILE=a-b samples the guest call stack over those frames
//! (PROFILE_OUT, one line of hex addresses per sample: last function entered, then return addresses).
//! FPS=30|60 (CLOCK=pct, default 200) sets the viewer's frame rate and taps A by game time (fields) instead of frames, and
//! prints the game time at the pop, to compare the frame rates.
//! HOSTPROF=a-b samples the host program counter (SIGPROF every 100 us of CPU time) over those frames into
//! HOSTPROF_OUT (default /tmp/sb_hostprof.txt): one hex offset into the executable per line (`addr2line -fe`).
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
    // GSsndGetStatus (0x801666BC) result, and count salCallback
    if std::env::var("FANFARE").is_ok() {
        fn fanfare(c: &mut colo_recomp::ppc::Cpu) {
            println!("  soundPlayFanfare({:#x}, {:#x}, {:#x})", c.r[3], c.r[4], c.r[5]);
            c.hook_fn.remove(&0x801653CC); colo_recomp::lifted::dispatch(c, 0x801653CC); c.hook_fn.insert(0x801653CC, fanfare);
        }
        fn status(c: &mut colo_recomp::ppc::Cpu) {
            static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let id = c.r[3];
            c.hook_fn.remove(&0x801666BC); colo_recomp::lifted::dispatch(c, 0x801666BC); c.hook_fn.insert(0x801666BC, status);
            if N.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % 300 == 0 { println!("  GSsndGetStatus({id:#x}) = {}", c.r[3]); }
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
    // PROFILE=from-to: sample the guest call stack (Cpu::last_fn, then the return addresses on the PPC back chain)
    // every 100 us while those frames run the game; one line per sample to PROFILE_OUT (default /tmp/sb_prof.txt)
    let prof: Option<(u32, u32)> = std::env::var("PROFILE").ok().and_then(|v| { let (a, b) = v.split_once('-')?; Some((a.parse().ok()?, b.parse().ok()?)) });
    static ON: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    let samples = std::sync::Arc::new(std::sync::Mutex::new(Vec::<Vec<u32>>::new()));
    if prof.is_some() {
        let r1 = &m.c.r[1] as *const u32 as usize;
        let lf = &m.c.last_fn as *const u32 as usize;
        let ram = m.c.m.regions.iter().find(|r| r.start == 0x8000_0000).map(|r| (r.bytes.as_ptr() as usize, r.bytes.len())).expect("MEM1");
        let out = samples.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_micros(100));
            if !ON.load(std::sync::atomic::Ordering::Relaxed) { continue; }
            let rd = |a: u32| -> Option<u32> {
                let o = a.wrapping_sub(0x8000_0000) as usize;
                if o + 4 > ram.1 { return None; }
                Some(u32::from_be(unsafe { std::ptr::read_volatile((ram.0 + o) as *const u32) }))
            };
            let mut st = vec![unsafe { std::ptr::read_volatile(lf as *const u32) }];
            let mut sp = unsafe { std::ptr::read_volatile(r1 as *const u32) };
            for _ in 0..48 {
                let Some(next) = rd(sp) else { break };
                if next <= sp || next == 0xFFFF_FFFF { break; }
                if let Some(lr) = rd(next + 4) { st.push(lr); }
                sp = next;
            }
            out.lock().unwrap().push(st);
        });
    }
    // DRAWDONE=1: per frame, GSgfxRenderEnd's draw-done callback (0x800D3F50) calls and its wait loop's OSYieldThread calls
    static DD: [std::sync::atomic::AtomicU32; 2] = [const { std::sync::atomic::AtomicU32::new(0) }; 2];
    if std::env::var("DRAWDONE").is_ok() {
        fn cb(c: &mut colo_recomp::ppc::Cpu) {
            DD[0].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            c.hook_fn.remove(&0x800D3F50); colo_recomp::lifted::dispatch(c, 0x800D3F50); c.hook_fn.insert(0x800D3F50, cb);
        }
        fn y(c: &mut colo_recomp::ppc::Cpu) {
            if c.lr == 0x800D31E4 { DD[1].fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
            c.hook_fn.remove(&0x800A1990); colo_recomp::lifted::dispatch(c, 0x800A1990); c.hook_fn.insert(0x800A1990, y);
        }
        m.c.hook_fn.insert(0x800D3F50, cb);
        m.c.hook_fn.insert(0x800A1990, y);
    }
    let hostprof: Option<(u32, u32)> = std::env::var("HOSTPROF").ok().and_then(|v| { let (a, b) = v.split_once('-')?; Some((a.parse().ok()?, b.parse().ok()?)) });
    if hostprof.is_some() { hostprof::start(); }
    let shots = std::env::var("SHOT").ok();
    let every: u32 = std::env::var("SHOT_EVERY").ok().and_then(|v| v.parse().ok()).unwrap_or(300);
    let mut was = m.floor_depth();
    let mut builder = colo_gx::cmd::Builder::new();
    let mut motors = [0u8; 4];
    // SNDLOG=1: every soundStartRaw (0x80166AB8: sound effects and songs) with its game time, to line up two runs
    if std::env::var("SNDLOG").is_ok() {
        fn snd(c: &mut colo_recomp::ppc::Cpu) {
            println!("  snd {:#x} at {:.3} s", c.r[3], colo_recomp::run::time_base(c) as f64 / 40_500_000.0);
            c.hook_fn.remove(&0x80166AB8); colo_recomp::lifted::dispatch(c, 0x80166AB8); c.hook_fn.insert(0x80166AB8, snd);
        }
        m.c.hook_fn.insert(0x80166AB8, snd);
    }
    let fps: Option<u32> = std::env::var("FPS").ok().and_then(|v| v.parse().ok());
    if let Some(f) = fps { m.set_frame_rate(f, std::env::var("CLOCK").ok().and_then(|v| v.parse().ok()).unwrap_or(200)); }
    let tb0 = colo_recomp::run::time_base(&m.c);
    for fr in 0..max {
        // by frames, or with FPS by 30 Hz periods of game time
        let k = if fps.is_some() { ((colo_recomp::run::time_base(&m.c) - tb0) / 1_351_350) as u32 } else { fr };
        let b = if k % 20 >= 10 && k % 20 < 13 { 0x100 } else { 0 };
        m.set_pad(colo_recomp::run::Pad { buttons: b, ..Default::default() });
        let s0 = m.c.steps;
        let t0 = std::time::Instant::now();
        if let Some((a, b)) = prof { ON.store(fr >= a && fr < b, std::sync::atomic::Ordering::Relaxed); }
        if let Some((a, b)) = hostprof { hostprof::ON.store(fr >= a && fr < b, std::sync::atomic::Ordering::Relaxed); }
        m.frame();
        ON.store(false, std::sync::atomic::Ordering::Relaxed);
        if std::env::var("PADDBG").is_ok() && fr % 10 == 0 {
            let r = |a: u32| m.c.m.u32(a);
            println!("frame {fr}: pad0 id {} status {} latched {:#010x} {:#010x} cur {:#010x}", r(0x8040_1C10), r(0x8040_1C1C), r(0x8040_1C28), r(0x8040_1C2C), r(0x8040_1C34));
        }
        if m.motors() != motors { motors = m.motors(); println!("frame {fr}: rumble {motors:?}"); }
        if std::env::var("DRAWDONE").is_ok() && fr % 50 == 0 { println!("frame {fr}: draw-done callbacks {} wait-loop yields {}", DD[0].swap(0, std::sync::atomic::Ordering::Relaxed), DD[1].swap(0, std::sync::atomic::Ordering::Relaxed)); }
        let t1 = t0.elapsed().as_secs_f64();
        if std::env::var("STEPS").is_ok() && fr % 60 == 0 { println!("frame {fr}: {} lifted blocks, {:.1} ms", m.c.steps - s0, t1 * 1e3); }
        if let Some((_, b)) = hostprof { if fr + 1 == b { hostprof::dump(); } }
        if prof.is_some_and(|(_, b)| fr + 1 == b) {
            let s = samples.lock().unwrap();
            let txt: String = s.iter().map(|st| st.iter().map(|a| format!("{a:x}")).collect::<Vec<_>>().join(" ") + "\n").collect();
            std::fs::write(std::env::var("PROFILE_OUT").unwrap_or("/tmp/sb_prof.txt".into()), txt).ok();
            eprintln!("profile: {} samples", s.len());
        }
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
            // as gamebattle.rs: the draw list on the game thread, its vertex work (`finish`) on the render thread
            let tb = std::time::Instant::now();
            // GXDUMP=frames (or all): each draw's GX state (stderr) for those frames
            let gxd = std::env::var("GXDUMP").is_ok_and(|v| v == "all" || v.split(',').any(|x| x == fr.to_string()));
            colo_gx::cmd::DUMP.store(gxd, std::sync::atomic::Ordering::Relaxed);
            if gxd { eprintln!("gx frame {fr}"); }
            let mut g = m.gpu_cmds_deferred(&mut builder);
            let t2 = tb.elapsed().as_secs_f64();
            g.finish();
            // FTIME=ms: every frame whose game + draw list took longer than that
            let slow = std::env::var("FTIME").ok().and_then(|v| v.parse::<f64>().ok()).is_some_and(|ms| (t1 + t2) * 1e3 > ms);
            if std::env::var("GXTIME").is_ok() && fr % 60 == 59 {
                use std::sync::atomic::Ordering::Relaxed;
                let t = |a: &std::sync::atomic::AtomicU64| a.swap(0, Relaxed) as f64 / 60e6;
                println!("  per frame (60): gx decode {:.2} ms, draw list {:.2} ms, vertex work {:.2} ms, {} states", t(&colo_recomp::run::FEED_NS), t(&colo_gx::cmd::TIMES[1]), t(&colo_gx::cmd::TIMES[0]), g.states.len());
            }
            if fr % 60 == 0 || slow { println!("frame {fr}: game {:.1} ms, draw list {:.1} ms ({} ops, {} verts, {} new textures)", t1 * 1e3, t2 * 1e3, g.ops.len(), g.verts.len(), g.new_tex.len()); }
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
            println!("frame {fr}: battle floor popped, battleGetResult {} ({} blocks, {} cycles, {:.2} s game time)", m.battle_result(), m.c.steps, m.c.cycles,
                (colo_recomp::run::time_base(&m.c) - tb0) as f64 / 40_500_000.0);
            return;
        }
        was = d;
        if fr % 600 == 0 { println!("frame {fr}: floor depth {d} threads {:x?} AI callbacks {}", colo_recomp::run::threads(), SYNTH.load(std::sync::atomic::Ordering::Relaxed)); }
    }
    println!("no pop after {max} frames (floor depth {}; {} blocks, {} cycles)", m.floor_depth(), m.c.steps, m.c.cycles);
}

#[path = "support/hostprof.rs"] mod hostprof;
