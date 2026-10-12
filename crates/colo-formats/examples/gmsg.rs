fn main() {
    let a: Vec<String> = std::env::args().collect();
    let b = std::fs::read(&a[1]).unwrap();
    let c = colo_formats::common::Common::parse(&b).unwrap();
    let ids: Vec<u32> = a[2..].iter().map(|s| u32::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()).collect();
    for m in c.messages() { if ids.contains(&m.id) { println!("{:#x} {}", m.id, m.render()); } }
}
