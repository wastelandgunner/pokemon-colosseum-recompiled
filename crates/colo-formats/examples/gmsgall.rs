fn main() {
    let a: Vec<String> = std::env::args().collect();
    let b = std::fs::read(&a[1]).unwrap();
    let c = colo_formats::common::Common::parse(&b).unwrap();
    for m in c.messages() { println!("{:#x}  {}", m.id, m.render().replace('\n', "\\n")); }
}
