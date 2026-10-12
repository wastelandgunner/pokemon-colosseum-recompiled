fn main() {
    let a: Vec<String> = std::env::args().collect();
    let b = std::fs::read(&a[1]).unwrap();
    let c = colo_formats::common::Common::parse(&b).unwrap();
    let t = c.sound_tables().unwrap();
    println!("{} sounds, {} song files, {} waves, {} reverbs", t.sounds.len(), t.song_files.len(), t.waves.len(), t.reverbs.len());
    for (i, s) in t.sounds.iter().enumerate().take(6) { println!("snd {i}: {s:?}"); }
    for (i, s) in t.song_files.iter().enumerate().take(4) { println!("song {i}: {:#x} {}", s.0, s.1); }
    for (i, w) in t.waves.iter().enumerate() { println!("wave {i}: {w:x?}"); }
    let v = c.species_voices().unwrap(); println!("species {}: {:?} .. {:?}", v.len(), &v[..4], &v[v.len()-3..]);
    for (i, r) in t.reverbs.iter().enumerate().take(4) { println!("rev {i}: {r:?}"); }
}
