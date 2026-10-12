fn main() {
    let a: Vec<String> = std::env::args().collect();
    let b = std::fs::read(&a[1]).unwrap();
    let rel = colo_formats::rel::Rel::parse(&b).unwrap();
    let want = u32::from_str_radix(a.get(2).map(|s| s.trim_start_matches("0x")).unwrap_or("0"), 16).unwrap();
    for (g, s, o) in colo_formats::maprel::registrations(&rel) {
        if want == 0 || g == want { println!("{g:#010x} -> s{s}+{o:#x} (file {:#x})", rel.file_offset(s, o).unwrap_or(0)); }
    }
}
