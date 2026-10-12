//! Camera animation sampling on a real `.cam` (skipped when the extracted file isn't present).
use colo_formats::{hsd, hsdanim};

#[test]
fn pyrite_arrival_camera() {
    let Ok(b) = std::fs::read("/tmp/camx/M2_out_01.cam") else { return };
    let arc = hsd::Archive::parse(&b).unwrap();
    let cam = hsdanim::load_camera(&arc).unwrap().unwrap();
    assert_eq!(cam.anims.len(), 1);
    assert_eq!(cam.anims[0].end_frame, 330.0);
    for f in [0.0, 100.0, 200.0, 330.0] {
        let p = cam.pose(0, f);
        println!("f{f}: eye {:?} interest {:?} roll {:.3} fov {:.1}", p.eye.map(|v| v.round()), p.interest.map(|v| v.round()), p.roll, p.fov);
        assert!(p.eye.iter().all(|v| v.is_finite() && v.abs() < 5000.0));
    }
}
