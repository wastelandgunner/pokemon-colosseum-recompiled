//! Run whole game frames from a story_entry_snap.py snapshot: `cargo run --release -p colo-battle --example frames <dir> [n]`
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let d = std::path::Path::new(&a[1]);
    let n: u32 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);
    let _ = 0;
    let mem = std::fs::read(d.join("before.bin")).unwrap();
    let s = std::fs::read_to_string(d.join("regs_before.json")).unwrap();
    let i = s.find("\"r\": [").unwrap() + 6;
    let j = s[i..].find(']').unwrap() + i;
    let mut r = [0u32; 32];
    for (k, x) in s[i..j].split(',').enumerate() { r[k] = x.trim().parse().unwrap(); }
    let mut m = colo_battle::run::Machine::new(mem, &r);
    let boot = std::env::var("BOOT").is_ok();
    if let Ok(t) = std::fs::read_to_string(std::env::var("SHIM").unwrap_or_default()) { m.c.load_stubs(&t); }
    m.c.hooks = true;
    if let Ok(a) = std::fs::read(d.join("aram.bin")) { m.set_aram(a); }
    if let Ok(a) = std::fs::read(d.join("l1.bin")) { m.set_l1(&a); }
    let gcm = std::env::var("GCM").unwrap_or("/media/zach/massstorage/ROMs/Nintendo/Gamecube/iph-pmc.gcm".into());
    let f = std::sync::Mutex::new(std::fs::File::open(gcm).unwrap());
    m.set_disc(Box::new(move |off, buf| {
        use std::io::{Read, Seek, SeekFrom};
        let mut f = f.lock().unwrap();
        f.seek(SeekFrom::Start(off)).unwrap();
        f.read_exact(buf).unwrap();
    }));

    // report entries into the command menu
    fn menu(c: &mut colo_battle::ppc::Cpu) {
        println!("  command menu (_fightMenuFightTrainerGcHeroOpenMenuSubMain)");
        c.hook_fn.remove(&0x80263BC8);
        let a = 0x80263BC8; colo_battle::lifted::dispatch(c, a);
        c.hook_fn.insert(0x80263BC8, menu);
    }
    m.c.hook_fn.insert(0x80263BC8, menu);
    // last indirect-call targets, printed if the run panics
    static LAST: std::sync::Mutex<Vec<(u32, u32)>> = std::sync::Mutex::new(Vec::new());
    fn wrap(c: &mut colo_battle::ppc::Cpu, a: u32) {
        { let mut l = LAST.lock().unwrap(); l.push((a, c.r[3])); if l.len() > 16 { l.remove(0); } }
        colo_battle::lifted::dispatch(c, a)
    }
    m.c.dispatch = wrap;
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |i| { eprintln!("last indirect calls {:x?}", LAST.lock().unwrap()); prev(i) }));
    for a in std::env::var("TRACE_FNS").unwrap_or_default().split(',').filter_map(|x| u32::from_str_radix(x.trim_start_matches("0x"), 16).ok()) { m.c.trace_fns.insert(a); }
    if let Ok(w) = std::env::var("WATCH") { colo_battle::mem::WATCH.store(u32::from_str_radix(w.trim_start_matches("0x"), 16).unwrap(), std::sync::atomic::Ordering::Relaxed); }
    let t = std::time::Instant::now();
    if let Ok(g) = std::env::var("CARD") { let gcis: Vec<Vec<u8>> = g.split(',').map(|p| std::fs::read(p).unwrap()).collect(); m.insert_card(&gcis); }
    if boot { m.boot(); println!("booted to the first GSgappUpdate: {} blocks, {:?}", m.c.steps, t.elapsed()); }
    for f in 0..n {
        // tap A (PAD_BUTTON_A) every 20 frames
        // PRESS="frame:buttons,..." holds those PAD_BUTTON bits for 3 frames; otherwise A every 20 frames (NOA=1: none)
        let press: Vec<(u32, u16)> = std::env::var("PRESS").unwrap_or_default().split(',').filter_map(|x| { let (a, b) = x.split_once(':')?; Some((a.parse().ok()?, u16::from_str_radix(b.trim_start_matches("0x"), 16).ok()?)) }).collect();
        let mut b = if std::env::var("NOA").is_err() && f % 20 >= 10 && f % 20 < 13 { 0x100 } else { 0 };
        for (at, bits) in &press { if f >= *at && f < at + 3 { b = *bits; } }
        m.set_pad(colo_battle::run::Pad { buttons: b, ..Default::default() });
        m.frame();
        m.gpu.log = std::env::var("DRAWLOG").ok().is_some_and(|v| v == f.to_string());
        let fr = if std::env::var("NOGPU").is_ok() { m.take_gx(); colo_gx::Frame::default() } else { m.gpu_frame() };
        if f % 300 == 0 { println!("  gx: {} draws, {} copies, unsupported {:?}", fr.draws.len(), fr.copies.len(), m.gpu.unsupported); }
        if let Ok(list) = std::env::var("SHOT") {
            if list.split(',').any(|x| x == f.to_string()) {
                let img = m.display(480);
                let mut ppm = format!("P6 640 480 255\n").into_bytes();
                for p in img.chunks(4) { ppm.extend_from_slice(&p[..3]); }
                std::fs::write(format!("/tmp/claude-1000/shot{f}.ppm"), ppm).unwrap();
                let mut ppm = format!("P6 640 528 255\n").into_bytes();
                for p in &m.gpu.color { ppm.extend_from_slice(&p[..3]); }
                std::fs::write(format!("/tmp/claude-1000/efb{f}.ppm"), ppm).unwrap();
                for c in &fr.copies { let st = &fr.states[c.0]; println!("  copy {:#x} src {:#x} size {:#x} dst {:#x} stride {:#x} at draw {}", c.1, st.bp[0x49], st.bp[0x4A], st.bp[0x4B] << 5, st.bp[0x4D], c.2); }
                println!("  xfb {:x?}", m.gpu.xfb);
            }
        }
        if std::env::var("MSG").is_ok() && f < 12 {
            let w = m.c.m.u32(0x80404AD8);
            if w != 0 {
                let msg = m.c.m.u32(w + 0x9C);
                let save = m.c.r; m.c.r[3] = msg; m.c.lr = 0; colo_battle::lifted::dispatch(&mut m.c, 0x800FA3D0);
                let chk = m.c.r[3]; m.c.r = save;
                let g = m.c.m.u32(0x8047AA80); println!("  step +0x4c..0x5c {:x?}", (0..5).map(|i| m.c.m.u32(g + 0x4c + 4 * i)).collect::<Vec<_>>());
                println!("  f{f} win state {} 98 {} flags {:#x} msg {msg:#x} IsCheck {chk}", m.c.m.u8(w + 1), m.c.m.u8(w + 0x98), m.c.m.u8(w));
            } else { println!("  f{f} no window"); }
        }
        if f == 20 {
            let save = m.c.r; m.c.r[3] = 0x50; m.c.lr = 0; colo_battle::lifted::dispatch(&mut m.c, 0x8005da18);
            let p = m.c.r[3]; m.c.r = save;
            println!("  menu 0x50 data {p:#x}: {:x?}", m.c.m.read(p, 0x20));
            let w = 0x8057ee80u32; println!("  window +0x18..0x1c {:x?}", m.c.m.read(w + 0x18, 4));
        }
        if f % std::env::var("EVERY").ok().and_then(|v| v.parse().ok()).unwrap_or(300) == 0 { eprintln!("gpu prep {:?} raster {:?}", m.gpu.prep_time, m.gpu.raster_time); }
        if f % std::env::var("EVERY").ok().and_then(|v| v.parse().ok()).unwrap_or(300) == 0 { println!("frame {f}: {} blocks, {:?} threads {:x?}", m.c.steps, t.elapsed(), colo_battle::run::threads()); }
    }
}
