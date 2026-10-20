//! Power-on check: boot from the disc image alone (`Machine::from_disc`) or, with SNAP=<dir>, from a
//! `boot_snap.py` snapshot, run `n` frames with no card and print an FNV hash of MEM1 every 60 frames (the two
//! boots should agree). `cargo run --release -p colo-recomp --example discboot <gcm> [n]`
//! MAIN_DUMP=<file>: write MEM1 when `__start` reaches `main` and stop (compare with the snapshot's before.bin).
//! SHOT=<frame,...> writes the XFB of those frames as OUT/boot<frame>.ppm (OUT default /tmp/claude-1000).
//! TAP_A=from-to taps A every 20 frames over that range.
use colo_recomp::run::{Machine, Pad, MAIN};

fn fnv(b: &[u8]) -> u64 { b.iter().fold(0xcbf29ce484222325u64, |h, &x| (h ^ x as u64).wrapping_mul(0x100000001b3)) }

fn main_dump(c: &mut colo_recomp::ppc::Cpu) {
    let p = std::env::var("MAIN_DUMP").unwrap();
    std::fs::write(&p, c.m.read(0x8000_0000, 0x0180_0000)).unwrap();
    eprintln!("main reached after {} blocks: MEM1 -> {p}; regs r1 {:#x} r2 {:#x} r13 {:#x}", c.steps, c.r[1], c.r[2], c.r[13]);
    std::process::exit(0);
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: u32 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(600);
    let f = std::sync::Mutex::new(std::fs::File::open(&a[1]).unwrap());
    let disc: colo_recomp::run::Disc = Box::new(move |off, buf| {
        use std::io::{Read, Seek, SeekFrom};
        let mut f = f.lock().unwrap();
        if f.seek(SeekFrom::Start(off)).is_err() || f.read_exact(buf).is_err() { buf.fill(0); }
    });
    let mut m = match std::env::var("SNAP") {
        Ok(d) => { let mut m = Machine::from_snapshot(std::path::Path::new(&d)).unwrap(); m.set_disc(disc); m }
        Err(_) => Machine::from_disc(disc).unwrap(),
    };
    if std::env::var("MAIN_DUMP").is_ok() { m.c.hook_fn.insert(MAIN, main_dump); }
    for a in std::env::var("TRACE_FNS").unwrap_or_default().split(',').filter_map(|x| u32::from_str_radix(x.trim_start_matches("0x"), 16).ok()) { m.c.trace_fns.insert(a); }
    // WATCHDOG=1: print the last lifted function entered once a second (racy, debugging only)
    if std::env::var("WATCHDOG").is_ok() {
        struct P(*const colo_recomp::ppc::Cpu); unsafe impl Send for P {} unsafe impl Sync for P {}
        let p = P(&*m.c);
        let q = &*m.c as *const colo_recomp::ppc::Cpu as usize;
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |i| { let c = unsafe { &*(q as *const colo_recomp::ppc::Cpu) }; eprintln!("panic in {:#x} lr {:#x} r1 {:#x} r2 {:#x} r13 {:#x}", c.last_fn, c.lr, c.r[1], c.r[2], c.r[13]); prev(i) }));
        std::thread::spawn(move || { let p = p; loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let c = unsafe { &*p.0 };
            eprintln!("watchdog: last fn {:#x} steps {} lr {:#x}", c.last_fn, c.steps, c.lr);
        } });
    }
    // WATCHDOG=1: print the last lifted function entered once a second (racy, debugging only)
    if std::env::var("WATCHDOG").is_ok() {
        struct P(*const colo_recomp::ppc::Cpu); unsafe impl Send for P {} unsafe impl Sync for P {}
        let p = P(&*m.c);
        let q = &*m.c as *const colo_recomp::ppc::Cpu as usize;
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |i| { let c = unsafe { &*(q as *const colo_recomp::ppc::Cpu) }; eprintln!("panic in {:#x} lr {:#x} r1 {:#x} r2 {:#x} r13 {:#x}", c.last_fn, c.lr, c.r[1], c.r[2], c.r[13]); prev(i) }));
        std::thread::spawn(move || { let p = p; loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let c = unsafe { &*p.0 };
            eprintln!("watchdog: last fn {:#x} steps {} lr {:#x}", c.last_fn, c.steps, c.lr);
        } });
    }
    if std::env::var("R1").is_ok() {
        for (a, h) in [(0x80099a44u32, h_osinit as fn(&mut colo_recomp::ppc::Cpu)), (0x8009affc, h_audio), (0x800032b0, h_initreg), (0x80003400, h_inithw), (0x80003340, h_initdata), (0x8009dfb8, h_intinit), (0x8009f664, h_memprot), (0x800a1290, h_thinit), (0x800a07c4, h_sram)] { m.c.hook_fn.insert(a, h); }
    }
    let t = std::time::Instant::now();
    m.boot();
    eprintln!("first GSgappUpdate: {} blocks, {:?}", m.c.steps, t.elapsed());
    let tap = std::env::var("TAP_A").ok().and_then(|r| { let (x, y) = r.split_once('-')?; Some((x.parse::<u32>().ok()?, y.parse::<u32>().ok()?)) });
    let shots = std::env::var("SHOT").unwrap_or_default();
    let out = std::env::var("OUT").unwrap_or("/tmp/claude-1000".into());
    for fr in 0..n {
        let mut p = Pad::default();
        if let Some((x, y)) = tap { if fr >= x && fr < y && fr % 20 >= 10 && fr % 20 < 13 { p.buttons = 0x100; } }
        m.set_pad(p);
        m.frame();
        m.gpu_frame();
        if fr % 60 == 59 || fr + 1 == n { println!("frame {} mem1 {:016x} tb {}", fr + 1, fnv(m.c.m.read(0x8000_0000, 0x0180_0000)), colo_recomp::run::time_base(&m.c)); }
        if shots.split(',').any(|x| x == fr.to_string()) {
            let px = m.display(480);
            let mut ppm = format!("P6 640 480 255\n").into_bytes();
            for p in px.chunks(4) { ppm.extend_from_slice(&p[..3]); }
            std::fs::write(format!("{out}/boot{fr}.ppm"), ppm).unwrap();
        }
    }
}
