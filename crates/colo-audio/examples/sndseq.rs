//! Exercise the sound layer against the disc data: `sndseq <dir with common.fsys, bgm_archive.fsys, sound/>`.
use colo_audio::{engine, layer, SoundData, SoundLayer, SoundTask};
use colo_formats::{common::Common, fsys::{FileType, Fsys}};

fn main() {
    let dir = std::path::PathBuf::from(std::env::args().nth(1).unwrap());
    let cf = std::fs::read(dir.join("common.fsys")).unwrap();
    let common = Fsys::parse(&cf).unwrap();
    let rel = common.read(common.find("common_rel", FileType::Rel).unwrap()).unwrap();
    let t = Common::parse(&rel).unwrap().sound_tables().unwrap();
    engine::init(32000, 2).unwrap();
    colo_audio::push_banks(&t, &common, |p| std::fs::read(dir.join(p.trim_start_matches('/'))).ok());
    let mut d = SoundData::new(&t);
    d.songs = colo_audio::bgm_archive(&Fsys::parse(&std::fs::read(dir.join("bgm_archive.fsys")).unwrap()).unwrap());
    layer::install(SoundLayer::new(d));
    let mut frame = 0u32;
    fn run(name: &str, mut task: SoundTask, frame: &mut u32) {
        let mut buf = vec![0i16; 1066];
        let start = *frame;
        loop {
            let r = layer::with(|l| { l.service(); task.poll(l) });
            engine::render(&mut buf);
            *frame += 1;
            if let Some(v) = r { println!("{name}: returned {v} after {} frames", *frame - start); break; }
        }
    }
    let mut buf = vec![0i16; 1066];
    let mut idle = |secs: f32, frame: &mut u32| {
        let mut peak = 0u16;
        for _ in 0..(secs * 60.0) as u32 { layer::with(|l| l.service()); engine::render(&mut buf[..1066]); peak = peak.max(buf[..1066].iter().map(|s| s.unsigned_abs()).max().unwrap()); *frame += 1; }
        let (b, s) = layer::with(|l| (l.cur_bgm, l.cur_sub));
        println!("  {secs}s idle: peak {peak}, bgm {b} sub {s}, status(3) {} status(7) {}", layer::with(|l| l.status(3)), layer::with(|l| l.status(7)));
    };
    run("set_bgm 3", SoundTask::set_bgm(3, 0, 255), &mut frame);
    idle(2.0, &mut frame);
    run("set_bgm 3 again", SoundTask::set_bgm(3, 0, 127), &mut frame);
    let t = layer::with(|l| SoundTask::change(l, 7, 1000, 0, 255, true));
    run("change 7 (fade 1000)", t, &mut frame);
    idle(2.0, &mut frame);
    run("sub 34 (atmos)", SoundTask::play_sub(34, 0, 127), &mut frame);
    idle(1.0, &mut frame);
    let t = layer::with(|l| SoundTask::fanfare(l, 969, 0, 127));
    run("fanfare 969", t, &mut frame);
    idle(1.5, &mut frame);
    let t = layer::with(|l| SoundTask::stop(l, 7, 500));
    run("stop 7 (bgm: becomes null_bgm)", t, &mut frame);
    idle(1.0, &mut frame);
    let t = layer::with(|l| SoundTask::stop_sub(l, 0));
    run("stop sub", t, &mut frame);
    idle(0.5, &mut frame);
}
