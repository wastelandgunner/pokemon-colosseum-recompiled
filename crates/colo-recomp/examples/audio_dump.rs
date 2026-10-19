//! Boot from a `main` snapshot (or `disc`: the disc image alone) with no input and write the AX mix (what the AI DMA played) as a 32 kHz stereo
//! WAV, to compare with Dolphin's dspdump.wav: `cargo run --release -p colo-recomp --example audio_dump <snap>
//! <frames> <out.wav>`. `COLO_DSP_COEF=linear` drops the polyphase table.
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
    m.boot();
    let mut pcm: Vec<i16> = Vec::new();
    for _ in 0..n {
        m.set_pad(colo_recomp::run::Pad::default());
        m.frame();
        m.take_gx();
        let mut s = colo_recomp::run::take_audio();
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
