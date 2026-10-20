//! Boot from a `main` snapshot (or `disc`: the disc image alone) with no input and write the AX mix (what the AI DMA played) as a 32 kHz stereo
//! WAV, to compare with Dolphin's dspdump.wav: `cargo run --release -p colo-recomp --example audio_dump <snap>
//! <frames> <out.wav>`. `COLO_DSP_COEF=linear` drops the polyphase table. SNDTRACE=1 prints each sound start
//! (the game's play 0x80165A44: time base, id), to compare with Dolphin's.
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: u32 = a[2].parse().unwrap();
    let gcm = std::env::var("GCM").unwrap_or("/media/zach/massstorage/ROMs/Nintendo/Gamecube/iph-pmc.gcm".into());
    let f = std::sync::Mutex::new(std::fs::File::open(gcm).unwrap());
    let disc: colo_recomp::run::Disc = Box::new(move |off, buf| {
        use std::io::{Read, Seek, SeekFrom};
        let mut f = f.lock().unwrap();
        if f.seek(SeekFrom::Start(off)).is_err() || f.read_exact(buf).is_err() { buf.fill(0); }
    });
    let mut m = if a[1] == "disc" { colo_recomp::run::Machine::from_disc(disc).unwrap() } else {
        let mut m = colo_recomp::run::Machine::from_snapshot(std::path::Path::new(&a[1])).unwrap();
        m.set_disc(disc);
        m
    };
    if std::env::var("SNDTRACE").is_ok() {
        fn play(c: &mut colo_recomp::ppc::Cpu) {
            eprintln!("sndtrace {} {:#x}", colo_recomp::run::time_base(c), c.r[3]);
            colo_recomp::run::call_original(c, 0x8016_5A44);
        }
        m.c.hook_fn.insert(0x8016_5A44, play);
        // AIBLK=1: at each AI DMA interrupt (__AIDHandler), the time, MusyX's block index and its four output blocks'
        // hashes and mean |sample| (as aiblk.py, kept beside the notes' audio check, prints them from Dolphin)
        fn aid(c: &mut colo_recomp::ppc::Cpu) {
            let fnv = |b: &[u8]| b.iter().fold(0xcbf29ce484222325u64, |h, &x| (h ^ x as u64).wrapping_mul(0x100000001b3));
            let (buf, idx) = (c.m.u32(0x8047_B09C), c.m.u8(0x8047_B0A0));
            let hs: Vec<String> = if (0x8000_0000..0x8180_0000).contains(&buf) {
                let d = c.m.read(buf, 0xA00).to_vec();
                (0..4).map(|i| format!("{:016x}", fnv(&d[i * 0x280..(i + 1) * 0x280]))).collect()
            } else { vec!["-".into()] };
            let lv = if (0x8000_0000..0x8180_0000).contains(&buf) {
                let d = c.m.read(buf, 0xA00);
                d.chunks_exact(2).map(|x| (i16::from_be_bytes([x[0], x[1]]) as i64).abs()).sum::<i64>() / 0x500
            } else { 0 };
            eprintln!("aiblk {} {} {} {}", colo_recomp::run::time_base(c), idx, hs.join(" "), lv);
            colo_recomp::run::call_original(c, 0x800A_C628);
        }
        if std::env::var("AIBLK").is_ok() { m.c.hook_fn.insert(0x800A_C628, aid); }
    }
    m.boot();
    let mut pcm: Vec<i16> = Vec::new();
    for _ in 0..n {
        m.set_pad(colo_recomp::run::Pad::default());
        m.frame();
        m.take_gx();
        let mut s = colo_recomp::run::take_audio();
        if std::env::var("SNDTRACE").is_ok() && pcm.is_empty() && !s.is_empty() {
            eprintln!("sndtrace audio starts by tb {} ({} samples this frame)", colo_recomp::run::time_base(&m.c), s.len() / 2);
        }
        if std::env::var("SNDTRACE").is_ok() && !s.is_empty() && pcm.iter().all(|&v| v == 0) && s.iter().any(|&v| v != 0) {
            eprintln!("sndtrace first sound in frame ending tb {} at output {:.4} s", colo_recomp::run::time_base(&m.c), (pcm.len() / 2) as f64 / 32000.0);
        }
        for p in s.chunks_mut(2) { p.swap(0, 1); } // R, L -> L, R
        pcm.extend(s);
    }
    let mut w = Vec::with_capacity(44 + pcm.len() * 2);
    let len = (pcm.len() * 2) as u32;
    w.extend(b"RIFF"); w.extend((36 + len).to_le_bytes()); w.extend(b"WAVEfmt ");
    w.extend(16u32.to_le_bytes()); w.extend(1u16.to_le_bytes()); w.extend(2u16.to_le_bytes());
    w.extend(32000u32.to_le_bytes()); w.extend(128000u32.to_le_bytes()); w.extend(4u16.to_le_bytes()); w.extend(16u16.to_le_bytes());
    w.extend(b"data"); w.extend(len.to_le_bytes());
    for v in &pcm { w.extend(v.to_le_bytes()); }
    std::fs::write(&a[3], w).unwrap();
    println!("{} frames, {} stereo samples -> {}", n, pcm.len() / 2, a[3]);
}
