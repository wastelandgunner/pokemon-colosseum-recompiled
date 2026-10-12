fn main() {
    let a: Vec<String> = std::env::args().collect();
    let b = std::fs::read(&a[1]).unwrap();
    let fs = colo_formats::fsys::Fsys::parse(&b).unwrap();
    for e in &fs.entries { if a.len() < 3 || e.name.contains(&a[2]) { println!("{:#010x} {}", e.identifier, e.file_name()); } }
}
