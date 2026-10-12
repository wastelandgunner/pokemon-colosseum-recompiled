fn main() {
    let a: Vec<String> = std::env::args().collect();
    let b = std::fs::read(&a[1]).unwrap();
    let arc = colo_formats::hsd::Archive::parse(&b).unwrap();
    let m = colo_formats::hsd::load_model(&arc).unwrap();
    let anims = colo_formats::hsdanim::load_animations(&arc, &m.joint_index).unwrap();
    let j: usize = a[2].parse().unwrap();
    println!("joint {j} flags {:#x} rest s{:?} t{:?}", m.joints[j].flags, m.joints[j].rest_scale, m.joints[j].rest_translation);
    for an in &anims { for t in an.tracks.iter().filter(|t| t.joint == j) { println!("anim {} ch {}: {:?}", an.index, t.channel, t.keys.iter().map(|k| (k.frame, k.value)).collect::<Vec<_>>()); } }
}
