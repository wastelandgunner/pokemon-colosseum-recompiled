fn main() {
    let a: Vec<String> = std::env::args().collect();
    let b = std::fs::read(&a[1]).unwrap();
    let fs = colo_formats::fsys::Fsys::parse(&b).unwrap();
    let e = fs.find_type(colo_formats::fsys::FileType::Ccd).next().unwrap();
    let c = colo_formats::ccd::Collision::parse(&fs.read(e).unwrap()).unwrap();
    for g in &c.groups { for l in g.iter().filter(|l| l.slot == 3) {
        let mut by: std::collections::BTreeMap<u16, Vec<[f32;3]>> = Default::default();
        for t in &l.triangles { for v in t.vertices { by.entry(t.raw[0]).or_default().push(v); } }
        for (id, vs) in by { let n = vs.len() as f32; let c: Vec<f32> = (0..3).map(|k| vs.iter().map(|v| v[k]).sum::<f32>() / n).collect();
            let lo: Vec<f32> = (0..3).map(|k| vs.iter().map(|v| v[k]).fold(f32::MAX, f32::min)).collect();
            let hi: Vec<f32> = (0..3).map(|k| vs.iter().map(|v| v[k]).fold(f32::MIN, f32::max)).collect();
            println!("check id {id}: centre {c:.1?} {lo:.1?}..{hi:.1?}"); }
    } }
}
