//! Rumble check: boot from the disc, and at frame `at` ask controller 1 to rumble the way the options menu does
//! (`fn_800F78A4(1, 0, 0xFF, 30, 0)`, from inside the game's own PAD sampling callback); print the motor commands
//! the game sends the SI (PADControlAllMotors -> SICxOUTBUF) as they change. PORT=2..4 plugs in that controller
//! too, binds it to a player slot 10 frames before (`fn_800F80B0(port)`, as a versus battle does) and rumbles it
//! instead (the game only drives bound, connected pads).
//! `cargo run --release -p colo-recomp --example rumble <gcm> [frames] [at]`
use colo_recomp::run::{call_original, Machine};
use std::sync::atomic::{AtomicBool, Ordering};

/// Set by the main thread at the chosen frame; the next rumble update (game thread) starts the rumble first.
static GO: AtomicBool = AtomicBool::new(false);
static BIND: AtomicBool = AtomicBool::new(false);
static PORT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

/// fn_800F8428, the rumble driver called from the PAD sampling callback.
fn rumble_update(c: &mut colo_recomp::ppc::Cpu) {
    if BIND.swap(false, Ordering::Relaxed) {
        let save = (c.r, c.lr);
        c.r[3] = PORT.load(Ordering::Relaxed);
        call_original(c, 0x800F_80B0);
        println!("bound port {}: {}", PORT.load(Ordering::Relaxed), c.r[3]);
        (c.r, c.lr) = save;
    }
    if GO.swap(false, Ordering::Relaxed) {
        let save = (c.r, c.lr);
        (c.r[3], c.r[4], c.r[5], c.r[6], c.r[7]) = (PORT.load(Ordering::Relaxed), 0, 0xFF, 30, 0);
        call_original(c, 0x800F_78A4);
        (c.r, c.lr) = save;
    }
    call_original(c, 0x800F_8428);
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: u32 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(400);
    let at: u32 = a.get(3).and_then(|s| s.parse().ok()).unwrap_or(300);
    let f = std::sync::Mutex::new(std::fs::File::open(&a[1]).unwrap());
    let disc: colo_recomp::run::Disc = Box::new(move |off, buf| {
        use std::io::{Read, Seek, SeekFrom};
        let mut f = f.lock().unwrap();
        if f.seek(SeekFrom::Start(off)).is_err() || f.read_exact(buf).is_err() { buf.fill(0); }
    });
    let mut m = Machine::from_disc(disc).unwrap();
    m.c.hook_fn.insert(0x800F_8428, rumble_update);
    let port: u32 = std::env::var("PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(1).clamp(1, 4);
    PORT.store(port, Ordering::Relaxed);
    let mut pads = [Some(colo_recomp::run::Pad::default()), None, None, None];
    pads[port as usize - 1] = Some(Default::default());
    m.set_pads(pads);
    m.boot();
    let st = m.c.m.u32(0x8047_A6D8) as u64 * (1u64 << 32) + m.c.m.u32(0x8047_A6DC) as u64;
    println!("__OSStartTime {st} ({:.6} s)", st as f64 / 40.5e6);
    let mut was = m.motors();
    for fr in 0..n {
        if fr + 10 == at && port > 1 { BIND.store(true, Ordering::Relaxed); }
        if fr == at { GO.store(true, Ordering::Relaxed); }
        m.frame();
        if m.motors() != was { was = m.motors(); println!("frame {fr}: motors {was:?}"); }
    }
}
