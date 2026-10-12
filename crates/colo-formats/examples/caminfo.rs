fn main() {
    for f in std::env::args().skip(1) {
        let b = std::fs::read(&f).unwrap();
        let arc = colo_formats::hsd::Archive::parse(&b).unwrap();
        let c = colo_formats::hsdanim::load_camera(&arc).unwrap().unwrap();
        println!("{f}: {:?} roll {} anims {}", c.base, c.roll, c.anims.len());
    }
}
