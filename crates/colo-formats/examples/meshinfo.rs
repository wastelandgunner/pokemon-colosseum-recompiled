fn main() {
    let a: Vec<String> = std::env::args().collect();
    let b = std::fs::read(&a[1]).unwrap();
    let arc = colo_formats::hsd::Archive::parse(&b).unwrap();
    let m = colo_formats::hsd::load_model(&arc).unwrap();
    let want = a.get(2).map(|s| usize::from_str_radix(s.trim_start_matches("0x"), 16).unwrap());
    for (i, me) in m.meshes.iter().enumerate() {
        if want.is_some_and(|w| w != me.material.mobj) { continue; }
        let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
        for p in &me.positions { for k in 0..3 { lo[k] = lo[k].min(p[k]); hi[k] = hi[k].max(p[k]); } }
        let ca: Vec<f32> = me.colors.iter().map(|c| c[3]).collect();
        let (amin, amax) = ca.iter().fold((f32::MAX, f32::MIN), |(l, h), v| (l.min(*v), h.max(*v)));
        let c0 = me.colors.first().copied().unwrap_or_default();
        println!("mesh {i} joint {} mobj {:#x} rm {:#x} tris {} {lo:.1?}..{hi:.1?} vtx alpha {amin:.3}..{amax:.3} c0 {c0:.3?} hidden {}", me.joint, me.material.mobj, me.material.render_mode, me.positions.len()/3, me.hidden);
    }
}
