//! The global string tables embedded in common_rel (needs the staged /tmp/cm/common_rel.rel).
#[test]
fn common_rel_messages() {
    let Ok(bytes) = std::fs::read("/tmp/cm/common_rel.rel") else { return };
    let c = colo_formats::common::Common::parse(&bytes).unwrap();
    let m = c.messages();
    let get = |id: u32| m.iter().find(|x| x.id == id).map(|x| x.render()).unwrap_or_default();
    for id in [0x136, 0x137, 0x138, 0x139, 0x13a, 0x1b59, 0x1b5a, 0x1b5b, 0x1b5c, 0x1b5d, 0x1b5e, 0x1b60, 0x3c04, 0x3c06] {
        println!("{id:#x} {:?}", get(id));
    }
    assert!(m.len() > 2000);
    assert!(get(0x1b5d).contains("Save"));
}
