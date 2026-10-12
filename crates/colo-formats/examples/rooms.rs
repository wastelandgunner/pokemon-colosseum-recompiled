fn main() {
    let a: Vec<String> = std::env::args().collect();
    let b = std::fs::read(&a[1]).unwrap();
    let c = colo_formats::common::Common::parse(&b).unwrap();
    for r in c.rooms().unwrap() { println!("#{} floor {:#x} group {:#x} kind {} area {} scripts {:x?}", r.index, r.floor_id, r.group_id, r.kind, r.area, r.scripts); }
}
