//! Battle frame pacing as the viewer runs it, headless: a game thread (frame + deferred draw list, as gamebattle.rs /
//! recomp.rs) feeding a render thread (`GFrame::finish`, colo-gxgpu render, XFB readback unless NOREAD) through a
//! channel of depth 2, the game paced at 29.97 Hz like the viewer. Every 60 frames: mean and worst game-thread time,
//! render-thread time split (finish / render / readback), and how many frames came out later than their slot.
//! `cargo run --release -p colo-viewer --example pipeperf <snapdir> [frames]` (SCALE=n, default 1; FREE=1: no pacing;
//! IMMEDIATE=1: the vertex work on the game thread, as before 2026-10-17).
use std::time::{Duration, Instant};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let dir = std::path::PathBuf::from(&a[1]);
    let n: u32 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(900);
    let scale: u32 = std::env::var("SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1);
    let mut m = colo_recomp::run::Machine::from_snapshot(&dir).expect("snapshot");
    let gcm = std::env::var("GCM").unwrap_or("/media/zach/massstorage/ROMs/Nintendo/Gamecube/iph-pmc.gcm".into());
    let f = std::sync::Mutex::new(std::fs::File::open(gcm).expect("disc image"));
    m.set_disc(Box::new(move |off, buf| {
        use std::io::{Read, Seek, SeekFrom};
        let mut f = f.lock().unwrap();
        if f.seek(SeekFrom::Start(off)).is_err() || f.read_exact(buf).is_err() { buf.fill(0); }
    }));
    let inst = wgpu::Instance::default();
    let ad = futures_lite::future::block_on(inst.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: None, force_fallback_adapter: false })).unwrap();
    let (dev, q) = futures_lite::future::block_on(ad.request_device(&Default::default())).unwrap();
    let mut gpu = colo_gxgpu::Gpu::new(dev, q, scale);
    let read = std::env::var("NOREAD").is_err();
    let (tx, rx) = std::sync::mpsc::sync_channel::<(u32, Instant, colo_gx::cmd::GFrame)>(2);
    let render = std::thread::spawn(move || {
        let (mut tf, mut tr, mut tx_, mut late, mut worst) = (Duration::ZERO, Duration::ZERO, Duration::ZERO, 0u32, Duration::ZERO);
        while let Ok((fr, due, mut g)) = rx.recv() {
            let t0 = Instant::now();
            g.finish();
            let t1 = Instant::now();
            gpu.render(&g);
            let t2 = Instant::now();
            if read { let _ = gpu.read_xfb(); }
            let t3 = Instant::now();
            tf += t1 - t0; tr += t2 - t1; tx_ += t3 - t2;
            worst = worst.max(t3 - t0);
            // shown at the next slot after it is done: late if past due + one slot
            if t3 > due + Duration::from_micros(33367) { late += 1; }
            if fr % 60 == 59 {
                println!("  render {fr}: finish {:.1} render {:.1} readback {:.1} ms (worst {:.1}), {late} of 60 late",
                    tf.as_secs_f64() * 1e3 / 60.0, tr.as_secs_f64() * 1e3 / 60.0, tx_.as_secs_f64() * 1e3 / 60.0, worst.as_secs_f64() * 1e3);
                (tf, tr, tx_, late, worst) = (Duration::ZERO, Duration::ZERO, Duration::ZERO, 0, Duration::ZERO);
            }
        }
    });
    let mut b = colo_gx::cmd::Builder::new();
    let free = std::env::var("FREE").is_ok();
    let start = Instant::now();
    let (mut sum, mut worst) = (Duration::ZERO, Duration::ZERO);
    for fr in 0..n {
        let due = start + Duration::from_secs_f64(fr as f64 * 1001.0 / 30000.0);
        if !free { let now = Instant::now(); if due > now { std::thread::sleep(due - now); } }
        let bt = if fr % 20 >= 10 && fr % 20 < 13 { 0x100 } else { 0 };
        m.set_pad(colo_recomp::run::Pad { buttons: bt, ..Default::default() });
        let t0 = Instant::now();
        m.frame();
        let g = if std::env::var("IMMEDIATE").is_ok() { m.gpu_cmds(&mut b) } else { m.gpu_cmds_deferred(&mut b) };
        let _ = colo_recomp::run::take_audio();
        let t = t0.elapsed();
        sum += t; worst = worst.max(t);
        tx.send((fr, due.max(t0), g)).unwrap();
        if fr % 60 == 59 {
            println!("game {fr}: {:.1} ms mean, worst {:.1}, behind schedule {:.0} ms", sum.as_secs_f64() * 1e3 / 60.0, worst.as_secs_f64() * 1e3,
                Instant::now().saturating_duration_since(due).as_secs_f64() * 1e3);
            (sum, worst) = (Duration::ZERO, Duration::ZERO);
        }
    }
    drop(tx);
    render.join().unwrap();
}
