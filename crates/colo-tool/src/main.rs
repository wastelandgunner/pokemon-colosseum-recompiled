//! colo-tool: inspect Pokémon Colosseum data. Inputs are read-only.
//!
//! A FILE argument is either a path on disk, or `DISC::path/in/disc` to read straight from a .gcm/.iso.

use colo_formats::{fsys::{FileType, Fsys}, gcm::Disc, msg, scd::{self, Script}};
use colo_script::{native, TraceHost, Vm, ThreadStatus};
use std::io::Write;
use std::path::Path;

/// println! that exits quietly when stdout is closed (e.g. piped into `head`).
macro_rules! outln {
    ($($t:tt)*) => {{
        if writeln!(std::io::stdout().lock(), $($t)*).is_err() { std::process::exit(0); }
    }};
}
use std::process::ExitCode;
mod story;
mod explore;
mod sound;

const USAGE: &str = "\
colo-tool <command> ...

  disc <image.gcm>                       disc info and file list
  disc-extract <image.gcm> <outdir>      extract every disc file (and sys/main.dol)
  fsys <FILE>                            list an FSYS archive
  fsys-extract <FILE> <outdir>           extract + decompress every entry
  msg <FILE> [name]                      dump string tables in an FSYS (or a raw .msg; common.fsys: + common_rel's)
  gpt1 <FILE> [name]                     particle banks: generator templates, bytecode, texture groups
  worldmap <FILE>                        world_map.fsys's worldmap_menu_rel tables (destinations, texts, rides, links)
  map <image.gcm> <room>                 room data, e.g. `map disc.gcm M2_out`: NPCs (model, pos, scripts),
                                         entry points, interaction points (warps/doors/events)
  maps <image.gcm> [list]                parse every room on the disc (list: one line per room record)
  room <image.gcm> <room> [options]      emulate a room load with the tracing host, then optionally
      --talk N       talk to character N (sync call with (group, N, 0, 0))
      --ip N         trigger interaction point #N (sync call with its 4 params)
      --ticks N      frames to run after load (default 60);  --calls N;  --ret I=V
  model <FILE> <outdir> [name]           convert .dat/.rdat/.pkx models to .glb (all in an FSYS, or one file)
  export-room <image.gcm> <room> <outdir> map.glb + NPC models + scene.json (placements) for colo-viewer
  ccd <FILE>                             summarize a room's collision (.ccd)
  flare <image.gcm> <room> [pngdir]      a room's lens flare (.gfl elements, textures) and sun position
  anims <FILE> <model>                   list a model's joint animations
  cam <FILE> <name> [f,f,...]            a .cam's base camera, animations and its pose at frames
  check-models <image.gcm>               load every .dat/.rdat/.pkx model on the disc, report failures
  materials <image.gcm>                  tally render modes, TObj flags and TObj TEV setups over every model
  mats <file.dat>                        one model's materials: colours, TObjs and TEV graph
  lights <gcm>                           every map .rdat's scene lights (LObjDescs)
  rel <FILE> [name]                      dump a REL (sections, words, relocated pointers)
  dis <FILE> [name]                      disassemble the .scd in an FSYS (or a raw .scd; common_rel.rel or
                                         common.fsys: the global script embedded in common_rel)
  ws <image.gcm|main.dol> [trace.log]    disassemble the battle move scripts (WS) in main.dol; with a
                                         Dolphin battle trace (Working/dolphin), check every traced command
  battle-data <image.gcm> [moves|species|items|types|tables|table:<name>]
                                         battle tables (common_rel moves/species, main.dol items/type chart)
  battle-replay <snapdir>                 run a call captured by Working/dolphin/snapshot.py through the lifted
                                         battle code and compare RNG draws, script commands, registers and RAM
  ips <FILE> [script]                    interaction points in common_rel (only those calling a script
                                         thread id, hex), e.g. ips common.fsys 596000d  (the PCs)
  check <image.gcm>                      run every function of every script on the disc twice
                                         (natives return 0, then 1) and report VM warnings
  story <dir> <plan>                     run story functions in order (plan lines: ROOM func_N, +flag, -flag,
                                         snag N) with a flag-keeping host; prints flag changes
  story-explore <dir> <rooms> [lo-hi] [+flag] [=goal]
                                         play rooms' story with the real scripts (rooms NAME or NAME@flag);
                                         with =goal, also the minimal steps and flags that reach it
  sound <disc|dir> <id> <out.wav> [secs] [sub]
                                         play sound id through the game's sound layer (std 104, or 105 with
                                         `sub`) and MusyX, rendered offline at 32 kHz to a WAV
  run <FILE> [options]                   run a script with a tracing host (natives return 0)
      --func N       function index to start (default 0)
      --ticks N      frames to simulate (default 600)
      --calls N      stop after N native calls (default 200)
      --ret I=V      make native I return V (repeatable), e.g. --ret 81=1
      --args a,b,..  thread arguments (talk handlers expect character group,id)

FILE = path | DISC::path   e.g. iph-pmc.gcm::M2_out.fsys";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("error: {e}"); ExitCode::FAILURE }
    }
}

type R<T> = Result<T, Box<dyn std::error::Error>>;

fn load(spec: &str) -> R<Vec<u8>> {
    if let Some((disc, inner)) = spec.split_once("::") {
        Ok(Disc::open(disc)?.read(inner)?)
    } else {
        Ok(std::fs::read(spec)?)
    }
}

fn stem(spec: &str) -> String {
    let p = spec.rsplit("::").next().unwrap_or(spec);
    Path::new(p).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default()
}

/// Every map `.rdat`'s scene lights (`scene_data` +8).
fn cmd_lights(path: &str) -> R<()> {
    use colo_formats::hsd::{scene_lights, Archive};
    let mut disc = Disc::open(path)?;
    let files: Vec<_> = disc.files.iter().filter(|f| f.path.ends_with(".fsys")).cloned().collect();
    let mut seen = std::collections::HashSet::new();
    for f in &files {
        let data = disc.read_at(f.offset, f.size as usize)?;
        let fs = Fsys::parse(&data)?;
        for e in &fs.entries {
            if !matches!(e.file_type, FileType::Rdat | FileType::Dat) || !seen.insert(e.name.clone()) { continue; }
            let b = fs.read(e)?;
            let Ok(arc) = Archive::parse(&b) else { continue };
            let Ok(ls) = scene_lights(&arc) else { outln!("{} {}: bad lights", f.path, e.name); continue };
            if ls.is_empty() { continue; }
            let anims = colo_formats::hsdanim::load_light_anims(&arc).unwrap_or_default();
            outln!("{} {} ({})", f.path, e.name, ls.len());
            for (i, l) in ls.iter().enumerate() {
                outln!("  flags {:#06x}{} colour {:02x?} pos {:?} interest {:?} attn {:?}", l.flags, if l.raw { " raw" } else { "" }, l.color, l.position, l.interest, l.attn());
                if let Some(Some(a)) = anims.get(i) {
                    let ch = |t: &[colo_formats::hsdanim::Track]| t.iter().map(|t| format!("{}:{}", t.channel, t.keys.len())).collect::<Vec<_>>().join(",");
                    outln!("    anim frames {} lobj [{}] pos [{}] interest [{}]", a.frame_count, ch(&a.lobj), ch(&a.position), ch(&a.interest));
                }
            }
        }
    }
    Ok(())
}

fn run(args: &[String]) -> R<()> {
    let a = |i: usize| args.get(i).map(String::as_str).ok_or_else(|| format!("missing argument\n\n{USAGE}"));
    match args.first().map(String::as_str) {
        Some("disc") => cmd_disc(a(1)?),
        Some("disc-extract") => cmd_disc_extract(a(1)?, a(2)?),
        Some("fsys") => cmd_fsys(a(1)?),
        Some("fsys-extract") => cmd_fsys_extract(a(1)?, a(2)?),
        Some("msg") => cmd_msg(a(1)?, args.get(2).map(String::as_str)),
        Some("worldmap") => cmd_worldmap(a(1)?),
        Some("gpt1") => cmd_gpt1(a(1)?, args.get(2).map(String::as_str)),
        Some("dis") => cmd_dis(a(1)?, args.get(2).map(String::as_str)),
        Some("ws") => cmd_ws(a(1)?, args.get(2).map(String::as_str)),
        Some("battle-replay") => cmd_battle_replay(a(1)?),
        Some("battle-data") => cmd_battle_data(a(1)?, args.get(2).map(String::as_str)),
        Some("run") => cmd_run(a(1)?, &args[2..]),
        Some("story") => story::cmd_story(a(1)?, a(2)?),
        Some("story-explore") => explore::cmd_story_explore(a(1)?, a(2)?, &args[3..]),
        Some("sound") => sound::cmd_sound(a(1)?, a(2)?, a(3)?, &args[4..]),
        Some("check") => cmd_check(a(1)?),
        Some("map") => cmd_map(a(1)?, a(2)?),
        Some("maps") => cmd_maps(a(1)?, args.get(2).map(|s| s.as_str()) == Some("list")),
        Some("room") => cmd_room(a(1)?, a(2)?, &args[3..]),
        Some("ccd") => cmd_ccd(a(1)?),
        Some("flare") => cmd_flare(a(1)?, a(2)?, args.get(3).map(|s| s.as_str())),
        Some("anims") => cmd_anims(a(1)?, a(2)?),
        Some("cam") => cmd_cam(a(1)?, a(2)?, args.get(3).map(|s| s.as_str()).unwrap_or("0")),
        Some("check-models") => cmd_check_models(a(1)?),
        Some("materials") => cmd_materials(a(1)?),
        Some("mats") => cmd_mats(a(1)?),
        Some("lights") => cmd_lights(a(1)?),
        Some("rel") => cmd_rel(a(1)?, args.get(2).map(String::as_str)),
        Some("ips") => cmd_ips(a(1)?, args.get(2).map(String::as_str)),
        Some("model") => cmd_model(a(1)?, a(2)?, args.get(3).map(String::as_str)),
        Some("export-room") => cmd_export_room(a(1)?, a(2)?, a(3)?),
        _ => { outln!("{USAGE}"); Ok(()) }
    }
}

fn cmd_disc(path: &str) -> R<()> {
    let d = Disc::open(path)?;
    outln!("{} disc {} rev {}  \"{}\"  {} files", d.game_id, d.disc_number, d.version, d.title, d.files.len());
    for f in &d.files { outln!("{:>10}  {:#010x}  {}", f.size, f.offset, f.path); }
    Ok(())
}

fn cmd_disc_extract(path: &str, out: &str) -> R<()> {
    let mut d = Disc::open(path)?;
    let files = d.files.clone();
    for f in &files {
        let dest = Path::new(out).join("files").join(&f.path);
        std::fs::create_dir_all(dest.parent().unwrap())?;
        std::fs::write(&dest, d.read_at(f.offset, f.size as usize)?)?;
    }
    std::fs::create_dir_all(Path::new(out).join("sys"))?;
    std::fs::write(Path::new(out).join("sys/main.dol"), d.read_dol()?)?;
    outln!("extracted {} files + main.dol to {out}", files.len());
    Ok(())
}

fn cmd_fsys(spec: &str) -> R<()> {
    let data = load(spec)?;
    let fs = Fsys::parse(&data)?;
    outln!("group {:#x}, {} entries", fs.group_id, fs.entries.len());
    for e in &fs.entries {
        outln!("{:>3}  {:<5} {:>9} {:>9} {}  {}", e.index, e.file_type.extension(), e.uncompressed_size,
            e.stored_size, if fs.is_compressed(e) { "lz" } else { "  " }, e.file_name());
    }
    Ok(())
}

fn cmd_fsys_extract(spec: &str, out: &str) -> R<()> {
    let data = load(spec)?;
    let fs = Fsys::parse(&data)?;
    std::fs::create_dir_all(out)?;
    for e in &fs.entries {
        std::fs::write(Path::new(out).join(e.file_name()), fs.read(e)?)?;
    }
    outln!("extracted {} entries to {out}", fs.entries.len());
    Ok(())
}

/// Get one entry of a type from an FSYS (by name, else the one named like the archive, else the first),
/// or treat the input as the raw file.
fn entry_of(spec: &str, ty: FileType, name: Option<&str>) -> R<(String, Vec<u8>)> {
    let data = load(spec)?;
    if data.starts_with(colo_formats::fsys::MAGIC) {
        let fs = Fsys::parse(&data)?;
        let want = name.map(str::to_string).unwrap_or_else(|| stem(spec));
        let e = fs.find(&want, ty).or_else(|| fs.find_type(ty).next())
            .ok_or_else(|| format!("no {} entry in {spec}", ty.extension()))?;
        Ok((e.name.clone(), fs.read(e)?))
    } else {
        Ok((stem(spec), data))
    }
}

fn cmd_msg(spec: &str, name: Option<&str>) -> R<()> {
    let data = load(spec)?;
    let is_fsys = data.starts_with(colo_formats::fsys::MAGIC);
    let tables: Vec<(String, Vec<u8>)> = if is_fsys {
        let fs = Fsys::parse(&data)?;
        fs.find_type(FileType::Msg).filter(|e| name.is_none_or(|n| e.name == n))
            .map(|e| Ok((e.name.clone(), fs.read(e)?))).collect::<R<_>>()?
    } else {
        vec![(stem(spec), data)]
    };
    for (n, t) in tables {
        outln!("== {n}");
        for m in msg::parse_table(&t)? {
            outln!("{:#06x}  {}", m.id, m.render().replace('\n', "\\n"));
        }
    }
    // common.fsys: the string tables embedded in common_rel
    if is_fsys && name.is_none() {
        let data = load(spec)?;
        let fs = Fsys::parse(&data)?;
        if let Some(e) = fs.find("common_rel", FileType::Rel) {
            outln!("== common_rel (embedded)");
            for m in colo_formats::common::Common::parse(&fs.read(e)?)?.messages() {
                outln!("{:#06x}  {}", m.id, m.render().replace('\n', "\\n"));
            }
        }
    }
    Ok(())
}

/// GPT1 particle banks in an FSYS (or a raw .gpt1).
fn cmd_gpt1(spec: &str, name: Option<&str>) -> R<()> {
    let data = load(spec)?;
    let banks: Vec<(String, Vec<u8>)> = if data.starts_with(colo_formats::fsys::MAGIC) {
        let fs = Fsys::parse(&data)?;
        fs.entries.iter().filter(|e| matches!(e.file_type, FileType::Gpt1) && name.is_none_or(|n| e.name == n))
            .map(|e| Ok((e.name.clone(), fs.read(e)?))).collect::<R<_>>()?
    } else { vec![(stem(spec), data)] };
    for (n, b) in banks {
        let g = colo_formats::gpt1::Gpt1::parse(&b)?;
        outln!("== {n}: {} generators, {} texture groups, {} refs {:?}", g.generators.iter().flatten().count(), g.tex_groups.len(), g.refs.len(), g.refs);
        for (i, t) in g.tex_groups.iter().enumerate() {
            if let Some(t) = t { outln!("  texgroup {i}: format {} {}x{} x{}", t.format, t.width, t.height, t.images.len()); }
        }
        for (i, t) in g.generators.iter().enumerate() {
            let Some(t) = t else { continue };
            outln!("  gen {i}: type {:#x} texgroup {} life {} plife {} flags {:#010x} grav {} fric {} vel {:?} radius {} angle {} rate {} size {} aux {:?}",
                t.kind, t.tex_group, t.life, t.particle_life, t.flags, t.gravity, t.friction, t.velocity, t.radius, t.angle, t.rate, t.size, t.aux);
            for l in colo_formats::gpt1::disassemble(&t.code) { outln!("      {l}"); }
        }
    }
    Ok(())
}

/// `world_map.fsys`'s worldmap_menu_rel tables.
fn cmd_worldmap(spec: &str) -> R<()> {
    let (_, bytes) = entry_of(spec, FileType::Rel, Some("worldmap_menu_rel"))?;
    let w = colo_formats::worldmap::WorldMap::parse(&bytes)?;
    outln!("{} locations (flag, floor, item, snapshot):", w.locations.len());
    for (i, l) in w.locations.iter().enumerate() { outln!("  {i} {:#06x} {:#05x} {:#06x} {:#010x}", l.flag, l.floor, l.item, l.snapshot); }
    outln!("{} description blocks (flag: messages by column):", w.descriptions.len());
    for d in &w.descriptions { outln!("  {:#06x}: {:x?}", d.flag, d.messages); }
    outln!("{} ride cameras (anim, camera):", w.ride_cams.len());
    for (i, c) in w.ride_cams.iter().enumerate() { outln!("  {i:2} {} {:#010x} {}", c.anim, c.camera, c.unk); }
    outln!("{} cursor links (dir 1 U 2 R 4 D 8 L, from, to):", w.links.len());
    for l in &w.links { outln!("  {:x} {:#06x} -> {:#06x}", l.dir, l.from, l.to); }
    Ok(())
}

/// Interaction points (optionally only those calling one script thread id, hex), with their room's floor and FSYS group.
fn cmd_ips(spec: &str, script: Option<&str>) -> R<()> {
    let data = load(spec)?;
    let bytes = if data.starts_with(colo_formats::fsys::MAGIC) { entry_of(spec, FileType::Rel, Some("common_rel"))?.1 } else { data };
    let c = colo_formats::common::Common::parse(&bytes)?;
    let want = script.map(|s| u32::from_str_radix(s.trim_start_matches("0x"), 16)).transpose().map_err(|e| e.to_string())?;
    let rooms = c.rooms()?;
    for ip in c.interaction_points()? {
        if want.is_some_and(|w| w != ip.script) { continue; }
        let room = rooms.iter().find(|r| r.floor_id == ip.room_id as u32);
        outln!("#{:<4} method {} floor {:#x} group {:#x} region {} script {:#010x} params {:x?}", ip.index, ip.method, ip.room_id,
            room.map(|r| r.group_id).unwrap_or(0), ip.region, ip.script, ip.params);
    }
    Ok(())
}

fn cmd_battle_replay(dir: &str) -> R<()> {
    use colo_recomp::{lifted, mem::Mem, ppc::Cpu, stubs};
    let d = Path::new(dir);
    let before = std::fs::read(d.join("before.bin"))?;
    let after = std::fs::read(d.join("after.bin"))?;
    let num = |v: &str| u32::from_str_radix(v.trim_start_matches("0x"), 16);
    // tiny JSON reader for the regs files: {"r": [..], "pc": n, "cr": n, "lr": n, "ctr": n, "xer": n, "f": ["hex",..]}
    fn regs(path: &Path) -> R<(Vec<u32>, u32, u32, u32, u32, u32, Vec<u64>)> {
        let s = std::fs::read_to_string(path)?;
        let arr = |k: &str| -> Vec<String> {
            let i = s.find(&format!("\"{k}\": [")).unwrap() + k.len() + 5;
            let j = s[i..].find(']').unwrap() + i;
            s[i..j].split(',').map(|x| x.trim().trim_matches('"').to_string()).collect()
        };
        let one = |k: &str| -> u32 {
            let i = s.find(&format!("\"{k}\": ")).unwrap() + k.len() + 4;
            s[i..].split(|c: char| c == ',' || c == '}').next().unwrap().trim().parse().unwrap()
        };
        let r = arr("r").iter().map(|x| x.parse().unwrap()).collect();
        let f = arr("f").iter().map(|x| u64::from_str_radix(x, 16).unwrap_or(0)).collect();
        Ok((r, one("pc"), one("cr"), one("lr"), one("ctr"), one("xer"), f))
    }
    let _ = num;
    let (r, pc, cr, lr, ctr, xer, f) = regs(&d.join("regs_before.json"))?;
    if let Ok(w) = std::env::var("WATCH") { colo_recomp::mem::WATCH.store(u32::from_str_radix(w.trim_start_matches("0x"), 16)?, std::sync::atomic::Ordering::Relaxed); }
    let mut m = Mem::new();
    m.map(0x80000000, before);
    let mut c = Cpu::bare(m);
    c.dispatch = lifted::dispatch;
    c.r.copy_from_slice(&r);
    for (k, b) in f.iter().enumerate() { c.f[k] = f64::from_bits(*b); c.ps1[k] = c.f[k]; }
    c.cr = cr; c.lr = lr; c.ctr = ctr; c.xer = xer;
    let func = lifted::LIFTED.iter().find(|(a, _)| *a == pc).ok_or("snapshot pc is not a lifted function")?;
    outln!("replaying {} ({pc:#010x})", func.1);
    // log WS commands through the dispatcher: wrap it
    thread_local! { static WS: std::cell::RefCell<Vec<(u32, u8)>> = const { std::cell::RefCell::new(Vec::new()) }; }
    fn wrap(c: &mut Cpu, a: u32) {
        let tbl = 0x8027A00Cu32;
        let spc = c.m.u32(0x8047B610);
        if (0x80375E48..0x80379BFF).contains(&spc) {
            let op = c.m.u8(spc);
            if c.m.u32(tbl + 4 * op as u32) == a { WS.with(|w| w.borrow_mut().push((spc, op))); }
        }
        lifted::dispatch(c, a)
    }
    c.dispatch = wrap;
    stubs::RAND_LOG.with(|l| l.borrow_mut().clear());
    // feed the game's seed at each battle draw (particles draw from the same seed between them)
    for line in std::fs::read_to_string(d.join("trace.log"))?.lines() {
        let x: Vec<&str> = line.split('\t').collect();
        // draws made by presentation code the headless port does not run (camera shake etc. via
        // _fadeEffectGetRandom) are not forced or compared
        if x[0] == "R" && (x[2].starts_with("800e0c20") || x[2].starts_with("800e0c28")) { continue; }
        if x[0] == "R" && std::env::var("NOFORCE").is_err() {
            let seed = u32::from_str_radix(x[3], 16)?;
            stubs::RAND_FORCE.with(|q| q.borrow_mut().push_back(seed));
        }
    }
    let prof_out = std::env::var("PROF").ok();
    if prof_out.is_some() { c.prof = true; c.hooks = true; c.m.trace = true; }
    // runtime presentation shim (no rebuild needed): tools/ppclift/shim.txt, or $SHIM
    let shim = std::env::var("SHIM").unwrap_or_else(|_| "../tools/ppclift/shim.txt".into());
    if let Ok(t) = std::fs::read_to_string(&shim) { c.load_stubs(&t); outln!("{} runtime stubs from {shim}", c.stubs.len()); }
    c.step_limit = std::env::var("STEPS").ok().and_then(|s| s.parse().ok()).unwrap_or(2_000_000_000);
    lifted::dispatch(&mut c, pc);
    outln!("{} blocks executed ({} instructions)", c.steps, c.instrs);
    c.m.trace = false;
    if let Some(p) = prof_out { std::fs::write(&p, colo_recomp::prof::dump())?; outln!("function facts -> {p}"); }
    // compare
    let trace = std::fs::read_to_string(d.join("trace.log"))?;
    let mut ws_want = vec![]; let mut r_want = vec![];
    for line in trace.lines() {
        let x: Vec<&str> = line.split('\t').collect();
        match x[0] {
            "WS" => ws_want.push((u32::from_str_radix(x[2], 16)?, u8::from_str_radix(x[3], 16)?)),
            "R" if x[2].starts_with("800e0c20") || x[2].starts_with("800e0c28") => {}
            "R" => r_want.push((u32::from_str_radix(x[2].split(',').next().unwrap(), 16)?, u32::from_str_radix(x[3], 16)?)),
            _ => {}
        }
    }
    // scripts the port runs only because presentation is off (e.g. wsCmd02 runs the 0x20-frame pause
    // script 0x80378964 when the battler has no model sequence; the game waits on the animation instead)
    const PRESENTATION_ONLY: [u32; 2] = [0x80378964, 0x80378967];
    let ws_got: Vec<(u32, u8)> = WS.with(|w| w.borrow().clone()).into_iter().filter(|x| !PRESENTATION_ONLY.contains(&x.0)).collect();
    let r_got = stubs::RAND_LOG.with(|l| l.borrow().clone());
    let first_diff = |a: &[(u32, u32)], b: &[(u32, u32)]| a.iter().zip(b).position(|(x, y)| x != y);
    outln!("WS commands: game {} port {}; first difference at {:?}", ws_want.len(), ws_got.len(),
        ws_want.iter().zip(&ws_got).position(|(x, y)| x != y));
    outln!("RNG draws: game {} port {}; first difference at {:?}", r_want.len(), r_got.len(), first_diff(&r_want, &r_got));
    if std::env::var("SHOW").is_ok() {
        for k in 0..ws_want.len().max(ws_got.len()) {
            let a = ws_want.get(k).map(|x| format!("{:08x} {:02x}", x.0, x.1)).unwrap_or_default();
            let b = ws_got.get(k).map(|x| format!("{:08x} {:02x}", x.0, x.1)).unwrap_or_default();
            outln!("  ws {k:3}  game {a:<12} port {b:<12}{}", if a != b { "  <<" } else { "" });
        }
        for k in 0..r_want.len().max(r_got.len()) {
            outln!("  rng {k}: game {:x?} port {:x?}", r_want.get(k), r_got.get(k));
        }
    }
    let (ra, _, cra, _, _, _, _) = regs(&d.join("regs_after.json"))?;
    outln!("r3 game {:#x} port {:#x}; cr game {:#x} port {:#x}", ra[3], c.r[3], cra, c.cr);
    let got = c.m.read(0x80000000, 0x01800000);
    let before = std::fs::read(d.join("before.bin"))?;
    // bytes the port wrote: does the game end with the same value? (the game also runs presentation for
    // many frames during the call, so bytes only the game changed are expected and reported separately)
    let sp = r[1];
    let in_stack = |a: u32| a < sp && a + 0x10000 > sp;
    let (mut agree, mut disagree, mut game_only) = (0u64, vec![], 0u64);
    for k in 0..got.len() {
        let a = 0x80000000 + k as u32;
        if in_stack(a) { continue; }
        if got[k] != before[k] {
            if got[k] == after[k] { agree += 1 } else { disagree.push(a) }
        } else if after[k] != before[k] { game_only += 1 }
    }
    outln!("RAM: port changed {} bytes; {} end equal to the game, {} differ; {} bytes changed only by the game",
        agree + disagree.len() as u64, agree, disagree.len(), game_only);
    let mut k = 0;
    while k < disagree.len() && k < 4000 {
        let s = disagree[k];
        let mut e = s;
        while k + 1 < disagree.len() && disagree[k + 1] <= e + 8 { k += 1; e = disagree[k]; }
        let o = (s - 0x80000000) as usize;
        let n = (e - s + 1) as usize;
        outln!("  {s:#010x} +{n}: before {:02x?} game {:02x?} port {:02x?}", &before[o..o + n.min(12)], &after[o..o + n.min(12)], &got[o..o + n.min(12)]);
        k += 1;
    }
    Ok(())
}

fn cmd_battle_data(path: &str, what: Option<&str>) -> R<()> {
    use colo_formats::{battle_data::BattleData, common::Common, dol::Dol};
    let mut w = World::open(path)?;
    let common = Common::parse(&w.common_bytes)?;
    let dol = Dol::parse(match &mut w.disc { Src::Disc(d) => d.read_dol()?, Src::Dir(p) => std::fs::read(p.join("sys/main.dol"))? })?;
    let bd = BattleData::load(&common, &dol)?;
    let names: std::collections::HashMap<u32, String> = common.messages().into_iter().map(|m| (m.id, m.render())).collect();
    let n = |id: u32| names.get(&id).cloned().unwrap_or_else(|| format!("#{id:x}"));
    outln!("{} moves, {} species, {} item rows ({} ids), {} types", bd.moves.len(), bd.species.len(), bd.items.len(),
        bd.item_rows.len(), bd.types.len());
    match what.unwrap_or("") {
        "moves" => for (i, m) in bd.moves.iter().enumerate() {
            outln!("{i:3} {:<14} type {:2} pow {:3} acc {:3} pp {:2} pri {:2} range {:2} eff {:3} chance {:3} script {:3} flags {}",
                n(m.name), m.zokusei, m.iryoku, m.avg, m.pp, m.pri, m.range, m.kouka, m.add_kouka_avg, m.seq,
                m.flags.iter().map(|f| if *f != 0 { '1' } else { '.' }).collect::<String>());
        },
        "species" => for (i, s) in bd.species.iter().enumerate() {
            outln!("{i:3} {:<12} dex {:3} types {:?} abil {:?} base {:?} grow {} catch {:3}", n(s.name), s.num_pokemon,
                s.zokusei, s.tokusei, s.basis, s.grow, s.get);
        },
        "items" => for id in 0..bd.item_rows.len() as u16 {
            if let Some(it) = bd.item(id) {
                outln!("{id:3} {:<16} kind {} price {:5} hold {:3} use {:3}", n(it.name), it.kind, it.price, it.soubi, it.fight_use_kouka);
            }
        },
        "types" => for (i, t) in bd.types.iter().enumerate() {
            // name ids collide across common_rel's string tables, so print the id
            outln!("{i:2} name {:#06x} {} | {:02x} {:02x} {:04x}", t.name,
                t.vs.iter().map(|v| match v { 0x3F => '.', 0x41 => '2', 0x42 => 'h', 0x43 => '0', _ => '?' }).collect::<String>(),
                t.b24, t.b25, t.h26);
        },
        t if t.starts_with("table:") => {
            // every row of one table: index then name=value fields (arrays joined with /)
            let def = colo_formats::battle_data::TABLES.iter().find(|d| d.name == &t[6..]).ok_or("unknown table")?;
            let tb = colo_formats::battle_data::Table::load(def, &common, &dol)?;
            for i in 0..tb.len() {
                let v: Vec<String> = def.fields.iter().map(|f| format!("{}={}", f.name,
                    (0..f.n).map(|k| tb.get(i, f.name, k).unwrap().to_string()).collect::<Vec<_>>().join("/"))).collect();
                outln!("{i}\t{}", v.join(" "));
            }
        }
        "tables" => for def in colo_formats::battle_data::TABLES {
            let tb = colo_formats::battle_data::Table::load(def, &common, &dol)?;
            outln!("{:<26} getptr {:#010x} {:4} x {:#05x}", def.name, def.getptr, tb.len(), def.size);
            for i in 0..tb.len().min(2) {
                let v: Vec<String> = def.fields.iter().map(|f| format!("{}={}", f.name,
                    (0..f.n.min(4)).map(|k| tb.get(i, f.name, k).unwrap().to_string()).collect::<Vec<_>>().join("/"))).collect();
                outln!("    [{i}] {}", v.join(" "));
            }
        },
        "mem" => {
            // the battle address space: REL globals as the game's prolog sets them, and a few getters
            use colo_battle::{data_bios as db, world};
            let m = world::load(&dol, &w.common_bytes)?;
            for (g, sym) in world::REL_GLOBALS {
                outln!("{g:#010x} sym {sym:3}: count ptr {:#010x} (= {}), base {:#010x}", m.u32(g), m.u32(m.u32(g)), m.u32(g + 4));
            }
            let pound = db::wazaDataBiosGetPtr(&m, 1);
            outln!("move 1 @ {pound:#010x}: pp {} power {} acc {}", db::wazaDataBiosGetPp(&m, pound),
                db::wazaDataBiosGetIryoku(&m, pound), db::wazaDataBiosGetAvg(&m, pound));
            let b = db::pokemonDataBiosGetPtr(&m, 1);
            outln!("species 1 @ {b:#010x}: hp {} spe {}", db::pokemonDataBiosGetBasisMaxHp(&m, b), db::pokemonDataBiosGetBasisNimbleness(&m, b));
        }
        "lift-check" => {
            // lifted pokemonGetStatus/wazaGetStatus (machine code -> Rust) vs the generated readable getters
            use colo_battle::{data_bios as db, world};
            use colo_recomp::{lifted, ppc::Cpu};
            let m = world::load(&dol, &w.common_bytes)?;
            let mut c = Cpu::new(m);
            c.dispatch = lifted::dispatch;
            let call = |c: &mut Cpu, f: fn(&mut Cpu), args: &[u32]| -> u32 {
                for (k, a) in args.iter().enumerate() { c.r[3 + k] = *a; }
                f(c);
                c.r[3]
            };
            let (mut ok, mut bad) = (0, 0);
            // species kinds 3..8 (base stats), 0x10 exp, 0x11 growth, 0x12 catch, 0x16 types
            for sp in [1u32, 4, 25, 150, 251, 277, 386, 412, 413, 0, 9999] {
                for (kind, idx) in [(3u32, 0u32), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (0x10, 0), (0x11, 0), (0x12, 0), (0x16, 0), (0x16, 1), (0x17, 1), (0x1e, 3)] {
                    let got = call(&mut c, lifted::f_8012640c, &[0, sp, kind, idx]);
                    let p = db::pokemonDataBiosGetPtr(&c.m, sp);
                    let want = if p == 0 { 0 } else { match kind {
                        3 => db::pokemonDataBiosGetBasisMaxHp(&c.m, p), 4 => db::pokemonDataBiosGetBasisPhyAtk(&c.m, p),
                        5 => db::pokemonDataBiosGetBasisPhyDef(&c.m, p), 6 => db::pokemonDataBiosGetBasisSpeAtk(&c.m, p),
                        7 => db::pokemonDataBiosGetBasisSpeDef(&c.m, p), 8 => db::pokemonDataBiosGetBasisNimbleness(&c.m, p),
                        0x10 => db::pokemonDataBiosGetGiveExp(&c.m, p), 0x11 => db::pokemonDataBiosGetGrowDataId(&c.m, p),
                        0x12 => db::pokemonDataBiosGetGet(&c.m, p), 0x16 => db::pokemonDataBiosGetZokuseiDataId(&c.m, p, idx),
                        0x17 => db::pokemonDataBiosGetTokuseiDataId(&c.m, p, idx), 0x1e => db::pokemonDataBiosGetGetWazaDataId(&c.m, p, idx),
                        _ => unreachable!() } };
                    if got == want { ok += 1 } else { bad += 1; outln!("species {sp} kind {kind:#x}[{idx}]: lifted {got} readable {want}"); }
                }
            }
            // a party Pokemon on the heap: write fields with the readable setters, read back through lifted pokemonGetStatus
            let pk = c.m.alloc(0x138);
            c.m.set_u16(pk, 25);
            c.m.set_u16(pk + 0x8C, 35); c.m.set_u16(pk + 0x8A, 30); c.m.set_u8(pk + 0x60, 12); c.m.set_u32(pk + 0x5C, 1234);
            c.m.set_u16(pk + 0x78, 84); c.m.set_u8(pk + 0x7A, 30);
            for (kind, want) in [(0x6e, 25u32), (0x7a, 12), (0x79, 1234), (0x83, 30), (0x87, 35), (0x7f, 84), (0x80, 30)] {
                let got = call(&mut c, lifted::f_8012640c, &[pk, 0, kind, 0]);
                if got == want { ok += 1 } else { bad += 1; outln!("party kind {kind:#x}: lifted {got} want {want}"); }
            }
            // wazaGetStatus kinds 2 pp, 3 type, 6 accuracy, 7 power for a few moves
            for mv in [1u32, 33, 57, 200, 354, 357, 400] {
                for kind in [2u32, 3, 4, 6, 7, 8] {
                    let got = call(&mut c, lifted::f_8011beb4, &[0, mv, kind, 0]);
                    let p = db::wazaDataBiosGetPtr(&c.m, mv);
                    let want = match kind { 2 => db::wazaDataBiosGetPp(&c.m, p), 3 => db::wazaDataBiosGetZokuseiDataId(&c.m, p),
                        4 => db::wazaDataBiosGetPri(&c.m, p) as u8 as i8 as i32 as u32, 6 => db::wazaDataBiosGetAvg(&c.m, p),
                        7 => db::wazaDataBiosGetIryoku(&c.m, p) as u16 as i16 as i32 as u32, 8 => db::wazaDataBiosGetFightKoukaDataId(&c.m, p), _ => 0 };
                    if got == want { ok += 1 } else { bad += 1; outln!("move {mv} kind {kind}: lifted {got} readable {want}"); }
                }
            }
            outln!("lift-check: {ok} match, {bad} differ");
        }
        "syms" => for k in 0..0x6C {
            if let Ok((o, c)) = common.table(k) { outln!("{k:3} offset {o:#08x} count {c}"); }
        },
        _ => {}
    }
    Ok(())
}

fn cmd_ws(spec: &str, trace: Option<&str>) -> R<()> {
    use colo_formats::{dol::Dol, ws};
    let bytes = if spec.ends_with(".dol") { std::fs::read(spec)? } else { Disc::open(spec)?.read_dol()? };
    let dol = Dol::parse(bytes)?;
    let l = ws::disassemble(&dol)?;
    if let Some(t) = trace {
        let (mut ok, mut bad) = (0, 0);
        for line in std::fs::read_to_string(t)?.lines() {
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() == 4 && f[0] == "WS" {
                let pc = u32::from_str_radix(f[2], 16)?;
                let op = u8::from_str_radix(f[3], 16)?;
                if l.insns.get(&pc).map(|i| i.op) == Some(op) { ok += 1 } else {
                    bad += 1;
                    if bad <= 20 { outln!("trace: {pc:#010x} op {op:#04x} is not a decoded command"); }
                }
            }
        }
        outln!("trace {t}: {ok} commands match, {bad} don't");
        return Ok(());
    }
    outln!("{}", l.render());
    let un: u32 = l.unreferenced.iter().map(|(a, b)| b - a).sum();
    outln!("; {} commands, {} labels, {} unreferenced bytes in {} ranges, {} problems",
        l.insns.len(), l.labels.len(), un, l.unreferenced.len(), l.problems.len());
    for p in &l.problems { outln!("; problem: {p}"); }
    Ok(())
}

fn cmd_dis(spec: &str, name: Option<&str>) -> R<()> {
    // common.fsys / common_rel.rel: the global script embedded in the rel
    let (n, s) = match entry_of(spec, FileType::Scd, name) {
        Ok((n, bytes)) if !spec.ends_with(".rel") => (n, Script::parse(bytes)?),
        _ => {
            let (n, rel) = entry_of(spec, FileType::Rel, Some(name.unwrap_or("common_rel")))?;
            (n, colo_formats::common::Common::parse(&rel)?.script()?)
        }
    };
    outln!("; {n}: script_id={:#x} functions={} data@{:#x}", s.script_id, s.functions.len(), s.data_offset);
    let mut starts: Vec<usize> = s.functions.clone();
    starts.push(s.bytes.len());
    starts.sort();
    for (i, &f) in s.functions.iter().enumerate() {
        let end = starts.iter().copied().find(|&x| x > f).unwrap_or(s.bytes.len());
        outln!("\nfunc_{i} @{f:#x}:   ; thread id {:#010x}", s.thread_id(i as u16));
        let mut pc = f;
        let mut imms: Vec<Option<u32>> = vec![];
        // keep going past a return while a jump still targets code further on
        let mut reach = f;
        while pc < end {
            let (op, next) = scd::decode(&s.bytes, pc)?;
            let mut note = String::new();
            match op {
                scd::Op::PushImm { value, .. } => imms.push(Some(value)),
                scd::Op::PushMinus1 => imms.push(Some(u32::MAX)),
                scd::Op::LoadMulti { var, .. } | scd::Op::Load { var } | scd::Op::Load15 { var } => {
                    if !var.on_stack() { imms.pop(); }
                    imms.push(None);
                }
                scd::Op::CallStd { argc } => {
                    let n = argc as usize;
                    if let Some(Some(idx)) = imms.len().checked_sub(n).map(|i| imms[i]) {
                        note = format!("  ; std[{idx}] {}", native(idx as u16).map(|x| x.name).unwrap_or("?"));
                    }
                    imms.clear();
                }
                _ => {}
            }
            if let scd::Op::Jump { target } | scd::Op::JumpFalse { target } = op { reach = reach.max(target as usize); }
            outln!("  {pc:05x}  {op}{note}");
            pc = next;
            if matches!(op, scd::Op::Return | scd::Op::Exit) && reach < pc { break; }
        }
    }
    Ok(())
}

fn cmd_run(spec: &str, opts: &[String]) -> R<()> {
    let (mut func, mut ticks, mut calls) = (0u16, 600u32, 200usize);
    let mut rets = vec![];
    let mut targs: Vec<u32> = vec![];
    let mut it = opts.iter();
    while let Some(o) = it.next() {
        let mut v = || it.next().ok_or_else(|| format!("{o} needs a value"));
        match o.as_str() {
            "--func" => func = v()?.parse()?,
            "--ticks" => ticks = v()?.parse()?,
            "--calls" => calls = v()?.parse()?,
            "--args" => targs = v()?.split(',').map(|x| x.trim().parse::<i64>().map(|n| n as u32)).collect::<Result<_, _>>()?,
            "--ret" => {
                let s = v()?;
                let (i, x) = s.split_once('=').ok_or("--ret wants I=V")?;
                rets.push((i.parse::<u16>()?, x.parse::<i64>()? as u32));
            }
            other => return Err(format!("unknown option {other}").into()),
        }
    }
    let (n, bytes) = entry_of(spec, FileType::Scd, None)?;
    let script = Script::parse(bytes)?;
    let tid = script.thread_id(func);
    let mut vm = Vm::new();
    vm.load(script);
    let mut host = TraceHost::new(calls);
    for (i, v) in rets { host.returns.insert(i, v); }
    let serial = vm.spawn(tid, &targs, &mut host).ok_or("spawn failed")?;
    outln!("running {n} func_{func} (thread id {tid:#010x}, serial {serial})");
    let mut printed = 0;
    let mut logged = 0;
    for _ in 0..ticks {
        vm.tick(&mut host);
        for c in &host.calls[printed..] { outln!("  tick {:>4}  {c}", vm.tick_count); }
        printed = host.calls.len();
        for l in &host.logs[logged..] { outln!("  tick {:>4}  log: {l}", vm.tick_count); }
        logged = host.logs.len();
        if host.full() || vm.is_idle() { break; }
    }
    outln!("-- after {} ticks, {} instructions:", vm.tick_count, vm.instructions_executed);
    for (s, id, st) in vm.threads() {
        let st = match st {
            ThreadStatus::WaitingNative(i) => format!("waiting on std[{i}] {}", native(i).map(|n| n.name).unwrap_or("?")),
            other => format!("{other:?}"),
        };
        outln!("   thread {s} {id:#010x}: {st}");
    }
    Ok(())
}

fn cmd_check(path: &str) -> R<()> {
    let mut disc = Disc::open(path)?;
    let fsys_files: Vec<_> = disc.files.iter().filter(|f| f.path.ends_with(".fsys")).cloned().collect();
    let (mut scripts, mut funcs, mut runs, mut instr) = (0, 0, 0u64, 0u64);
    let mut warn_runs = 0;
    let mut kinds: std::collections::BTreeMap<String, usize> = Default::default();
    let mut examples: Vec<String> = vec![];
    for f in &fsys_files {
        let data = disc.read_at(f.offset, f.size as usize)?;
        let fs = Fsys::parse(&data)?;
        for e in fs.find_type(FileType::Scd) {
            let script = Script::parse(fs.read(e)?)?;
            scripts += 1;
            for func in 0..script.functions.len() as u16 {
                funcs += 1;
                for ret in [0u32, 1] {
                    let mut vm = Vm::new();
                    vm.budget = 20_000;
                    vm.load(script.clone());
                    let mut host = TraceHost::new(5_000);
                    for i in 0..256u16 { host.returns.insert(i, ret); }
                    // field/talk handlers receive (character group, character id, ...) as thread args
                    if script.functions[func as usize] == 0 { continue; }
                    let Some(_) = vm.spawn(script.thread_id(func), &[0; 8], &mut host) else { continue };
                    for _ in 0..300 {
                        vm.tick(&mut host);
                        if vm.is_idle() || host.full() { break; }
                    }
                    runs += 1;
                    instr += vm.instructions_executed;
                    if !host.logs.is_empty() {
                        warn_runs += 1;
                        for l in &host.logs {
                            let k = l.split(':').nth(1).unwrap_or(l).trim().split(';').next().unwrap_or("").to_string();
                            let k = k.split(" at pc").next().unwrap_or(&k).to_string();
                            *kinds.entry(k).or_default() += 1;
                        }
                        for l in host.logs.iter().filter(|l| !l.contains("budget") && !l.starts_with("[script")) {
                            if examples.len() < 15 {
                                examples.push(format!("{}:{} func_{func} ret={ret}: {l}", f.path, e.name));
                            }
                        }
                    }
                }
            }
        }
    }
    outln!("{scripts} scripts, {funcs} functions, {runs} runs, {instr} instructions; {warn_runs} runs with warnings");
    for (k, n) in &kinds { outln!("  {n:>6}  {k}"); }
    for x in &examples { outln!("  e.g. {x}"); }
    Ok(())
}

fn cmd_rel(spec: &str, name: Option<&str>) -> R<()> {
    use colo_formats::rel::Rel;
    let (n, bytes) = entry_of(spec, FileType::Rel, name)?;
    let rel = Rel::parse(&bytes)?;
    outln!("{n}: REL id {} v{} {} sections, {} relocations", rel.id, rel.version, rel.sections.len(), rel.relocs.len());
    for (i, s) in rel.sections.iter().enumerate() {
        if s.size > 0 { outln!("  section {i}: file {:#x} size {:#x}{}", s.offset, s.size, if s.exec { " exec" } else { "" }); }
    }
    // dump each data section as words, marking relocated pointers
    for (i, s) in rel.sections.iter().enumerate() {
        let Some(b) = rel.section_bytes(i) else { continue };
        if s.exec { continue; }
        outln!("\n== section {i}");
        for off in (0..b.len()).step_by(4) {
            let w = b.get(off..off + 4).map(|x| u32::from_be_bytes([x[0], x[1], x[2], x[3]])).unwrap_or(0);
            let p = rel.relocs.get(&(i as u8, off)).map(|r| {
                if r.module == rel.id { format!("  -> s{}:{:#x}", r.target_section, r.addend) } else { format!("  -> mod{} s{}:{:#x}", r.module, r.target_section, r.addend) }
            }).unwrap_or_default();
            outln!("  {off:05x}  {w:08x}{p}");
        }
    }
    Ok(())
}

/// A disc image, or a directory of extracted disc files (`disc-extract`'s `files/`, or loose `.fsys` files).
pub(crate) enum Src { Disc(Disc), Dir(std::path::PathBuf) }

impl Src {
    fn read(&mut self, name: &str) -> R<Vec<u8>> {
        match self {
            Src::Disc(d) => Ok(d.read(name)?),
            Src::Dir(p) => Ok(std::fs::read(p.join(name)).or_else(|_| std::fs::read(p.join("files").join(name)))?),
        }
    }
    fn disc(&mut self) -> R<&mut Disc> { match self { Src::Disc(d) => Ok(d), Src::Dir(_) => Err("this command needs a disc image".into()) } }
}

struct World {
    disc: Src,
    common_bytes: Vec<u8>,
}

impl World {
    fn open(path: &str) -> R<Self> {
        let mut disc = if Path::new(path).is_dir() { Src::Dir(path.into()) } else { Src::Disc(Disc::open(path)?) };
        let cf = disc.read("common.fsys")?;
        let fs = Fsys::parse(&cf)?;
        let e = fs.find("common_rel", FileType::Rel).ok_or("common_rel not found")?;
        let common_bytes = fs.read(e)?;
        Ok(World { disc, common_bytes })
    }
}

fn sc(id: u32) -> String { if id == 0 { "-".into() } else { format!("{:#x}:func_{}", id >> 16, id & 0xFFFF) } }

fn cmd_map(path: &str, room_name: &str) -> R<()> {
    use colo_formats::{common::Common, maprel::MapRel};
    let mut w = World::open(path)?;
    let common = Common::parse(&w.common_bytes)?;
    let rooms = common.rooms()?;
    let models = common.character_models()?;
    let data = w.disc.read(&format!("{room_name}.fsys"))?;
    let fs = Fsys::parse(&data)?;
    let room = rooms.iter().find(|r| r.group_id == fs.group_id).ok_or_else(|| format!("no room record for group {:#x}", fs.group_id))?;
    let e = fs.find(room_name, FileType::Rel).ok_or("room has no .rel")?;
    let m = MapRel::parse(&fs.read(e)?, room)?;
    outln!("{room_name}: room #{} floor id {:#x}, group {:#x}, kind {}, area {}, map res {:#x}", room.index, room.floor_id, room.group_id, room.kind, room.area, room.map_res_id);
    outln!("  scripts: sound {} modify {} +30 {} +34 {} +38 {}", sc(room.scripts[0]), sc(room.scripts[1]), sc(room.scripts[2]), sc(room.scripts[3]), sc(room.scripts[4]));
    if m.sun_res_id != 0 { outln!("  sun res id {:#x} (lens flare on)", m.sun_res_id); }
    outln!("{} characters (talk: sync call with (group, index, 0, 0); move: async thread with (group, index, ...)):", m.characters.len());
    for c in &m.characters {
        let model = models.get(c.people_index as usize).map(|md| {
            fs.entries.iter().find(|en| en.identifier & 0xFFFF_0000 == md.fsys_identifier & 0xFFFF_0000)
                .map(|en| en.name.clone()).unwrap_or_else(|| format!("id {:#010x}", md.fsys_identifier))
        }).unwrap_or_else(|| "?".into());
        outln!("  {:>2} {:<16} name {:#06x} {}{} move_type {} start {} rot {:>4}  pos ({:8.2}, {:7.2}, {:8.2})  talk {}  move {}",
            c.index, model, c.name_id, if c.visible { "vis" } else { "hid" }, if c.talk_through_walls { " thru" } else { "" }, c.move_type, c.talk_start_type, c.rotation_y,
            c.position[0], c.position[1], c.position[2], sc(c.talk_script), sc(c.move_script));
    }
    if !m.tv.is_empty() {
        outln!("{} TV channels (room record +0x48):", m.tv.len());
        for t in &m.tv { outln!("  id {:2}: {}x{} screen {:#010x} model {:#010x} anim {} camera {:#010x}", t.id, t.width, t.height, t.screen, t.model, t.anim, t.camera); }
    }
    outln!("{} entry points:", m.entry_points.len());
    for e in &m.entry_points {
        outln!("  {:>2} angle {:>4}  pos ({:8.2}, {:7.2}, {:8.2})", e.index, e.angle, e.position[0], e.position[1], e.position[2]);
    }
    let ips: Vec<_> = common.interaction_points()?.into_iter().filter(|p| p.room_id as u32 == room.floor_id).collect();
    outln!("{} interaction points (method 1 pass, 2 touch, 3 check, 4 menu; script called sync with params):", ips.len());
    for p in ips {
        outln!("  #{:<3} method {} region {:>3} script {:<16} params {:08x} {:08x} {:08x} {:08x}", p.index, p.method, p.region, sc(p.script), p.params[0], p.params[1], p.params[2], p.params[3]);
    }
    Ok(())
}

fn cmd_maps(path: &str, list: bool) -> R<()> {
    use colo_formats::{common::Common, maprel::MapRel};
    let mut w = World::open(path)?;
    let common = Common::parse(&w.common_bytes)?;
    let rooms = common.rooms()?;
    let ips = common.interaction_points()?;
    let fsys_files: Vec<_> = w.disc.disc()?.files.iter().filter(|f| f.path.ends_with(".fsys")).cloned().collect();
    let (mut ok, mut no_fsys, mut bad, mut chars, mut entries, mut odd) = (0, 0, 0, 0, 0, 0);
    let mut by_group = std::collections::HashMap::new();
    for f in &fsys_files {
        let hdr = w.disc.disc()?.read_at(f.offset, 0x10)?;
        by_group.entry(u32::from_be_bytes([hdr[8], hdr[9], hdr[10], hdr[11]])).or_insert_with(|| f.clone());
    }
    for room in &rooms {
        let name = by_group.get(&room.group_id).map_or("-", |f| f.path.trim_end_matches(".fsys"));
        if list { outln!("  #{:3} floor {:#5x} kind {} area {:2} group {:#6x} {name}", room.index, room.floor_id, room.kind, room.area, room.group_id); }
        let Some(f) = by_group.get(&room.group_id) else { no_fsys += 1; continue };
        let data = w.disc.disc()?.read_at(f.offset, f.size as usize)?;
        let fs = Fsys::parse(&data)?;
        let stem = f.path.trim_end_matches(".fsys");
        let Some(e) = fs.find_type(FileType::Rel).find(|e| e.name == stem).or_else(|| fs.find_type(FileType::Rel).next()) else { continue };
        match MapRel::parse(&fs.read(e)?, room) {
            Ok(m) => {
                ok += 1; chars += m.characters.len(); entries += m.entry_points.len();
                let n_odd = m.characters.iter().filter(|c| [c.talk_script, c.move_script].iter().any(|&s| s != 0 && s >> 16 != 0x100)).count();
                if n_odd > 0 { odd += n_odd; outln!("  {stem}: {n_odd} characters with non-room script ids"); }
                if m.sun_res_id != 0 { outln!("  {stem}: sun res id {:#x}", m.sun_res_id); }
            }
            Err(err) => { bad += 1; outln!("  {stem}: {err}"); }
        }
    }
    outln!("{} rooms: {ok} parsed ({chars} characters, {entries} entry points), {no_fsys} without an archive on disc, {bad} failed; {odd} odd script ids; {} interaction points total",
        rooms.len(), ips.len());
    Ok(())
}

/// Emulate a room load (as far as it is understood) and optionally talk to an NPC or trigger an interaction point.
fn cmd_room(path: &str, room_name: &str, opts: &[String]) -> R<()> {
    use colo_formats::{common::Common, maprel::MapRel};
    let (mut talk, mut ip, mut ticks, mut calls) = (None::<usize>, None::<usize>, 60u32, 400usize);
    let mut rets = vec![];
    let mut it = opts.iter();
    while let Some(o) = it.next() {
        let mut v = || it.next().ok_or_else(|| format!("{o} needs a value"));
        match o.as_str() {
            "--talk" => talk = Some(v()?.parse()?),
            "--ip" => ip = Some(v()?.parse()?),
            "--ticks" => ticks = v()?.parse()?,
            "--calls" => calls = v()?.parse()?,
            "--ret" => {
                let s = v()?;
                let (i, x) = s.split_once('=').ok_or("--ret wants I=V")?;
                rets.push((i.parse::<u16>()?, x.parse::<i64>()? as u32));
            }
            other => return Err(format!("unknown option {other}").into()),
        }
    }
    let mut w = World::open(path)?;
    let common = Common::parse(&w.common_bytes)?;
    let rooms = common.rooms()?;
    let data = w.disc.read(&format!("{room_name}.fsys"))?;
    let fs = Fsys::parse(&data)?;
    let room = rooms.iter().find(|r| r.group_id == fs.group_id).ok_or("no room record")?.clone();
    let map = match fs.find(room_name, FileType::Rel) { Some(e) => MapRel::parse(&fs.read(e)?, &room)?, None => Default::default() };
    let scd = fs.find(room_name, FileType::Scd).ok_or("room has no .scd")?;
    let mut vm = Vm::new();
    vm.load(common.script()?);
    vm.load(Script::parse(fs.read(scd)?)?);
    let mut host = TraceHost::new(calls);
    for (i, v) in rets { host.returns.insert(i, v); }
    let group = room.group_id;
    let mut printed = 0usize;
    let mut logged = 0usize;
    let flush = |host: &TraceHost, vm: &Vm, printed: &mut usize, logged: &mut usize, tag: &str| {
        for c in &host.calls[*printed..] { outln!("  {tag} f{:<4} {c}", vm.tick_count); }
        *printed = host.calls.len();
        for l in &host.logs[*logged..] { outln!("  {tag} f{:<4} log: {l}", vm.tick_count); }
        *logged = host.logs.len();
    };
    outln!("== load {room_name} (floor {:#x}, group {group:#x}): sound script async, NPC move scripts async, modify script sync", room.floor_id);
    if room.scripts[0] != 0 { vm.spawn(room.scripts[0], &[], &mut host); }
    for c in &map.characters {
        if c.move_script != 0 { vm.spawn(c.move_script, &[group, c.index as u32, 0, 0], &mut host); }
    }
    if room.scripts[1] != 0 {
        let r = vm.call_sync(room.scripts[1], &[], &mut host, 600);
        flush(&host, &vm, &mut printed, &mut logged, "modify");
        outln!("  modify script returned {r:?}");
    }
    for _ in 0..ticks {
        vm.tick(&mut host);
        flush(&host, &vm, &mut printed, &mut logged, "tick  ");
        if host.full() { break; }
    }
    if let Some(n) = talk {
        let c = map.characters.get(n).ok_or("no such character")?;
        outln!("== talk to character {n} (talk script {})", sc(c.talk_script));
        let r = vm.call_sync(c.talk_script, &[group, n as u32, 0, 0], &mut host, 6000);
        flush(&host, &vm, &mut printed, &mut logged, "talk  ");
        outln!("  talk script returned {r:?}");
    }
    if let Some(n) = ip {
        let p = common.interaction_points()?.into_iter().find(|p| p.index == n).ok_or("no such interaction point")?;
        outln!("== interaction point #{n}: method {} region {} script {}", p.method, p.region, sc(p.script));
        let r = vm.call_sync(p.script, &p.params, &mut host, 6000);
        flush(&host, &vm, &mut printed, &mut logged, "ip    ");
        outln!("  returned {r:?}");
    }
    outln!("-- threads after {} frames:", vm.tick_count);
    for (s, id, st) in vm.threads() {
        let st = match st {
            ThreadStatus::WaitingNative(i) => format!("waiting on std[{i}] {}", native(i).map(|n| n.name).unwrap_or("?")),
            other => format!("{other:?}"),
        };
        outln!("   thread {s} {}: {st}", sc(id));
    }
    Ok(())
}

fn cmd_model(spec: &str, out: &str, name: Option<&str>) -> R<()> {
    use colo_formats::hsd::{load_model, Archive};
    let data = load(spec)?;
    let mut items: Vec<(String, Vec<u8>)> = vec![];
    if data.starts_with(colo_formats::fsys::MAGIC) {
        let fs = Fsys::parse(&data)?;
        for e in &fs.entries {
            if !matches!(e.file_type, FileType::Dat | FileType::Rdat | FileType::Pkx) { continue; }
            if name.is_some_and(|n| n != e.name) { continue; }
            let mut b = fs.read(e)?;
            if e.file_type == FileType::Pkx { b = pkx_dat(&b)?; }
            items.push((e.name.clone(), b));
        }
    } else {
        let mut b = data;
        if spec.ends_with(".pkx") { b = pkx_dat(&b)?; }
        items.push((stem(spec), b));
    }
    std::fs::create_dir_all(out)?;
    for (n, b) in items {
        let ar = match Archive::parse(&b) { Ok(a) => a, Err(e) => { outln!("{n}: {e}"); continue; } };
        let m = match load_model(&ar) { Ok(m) => m, Err(e) => { outln!("{n}: {e}"); continue; } };
        let tris = m.triangle_count();
        let bounds = m.bounds().map(|(lo, hi)| format!("[{:.1} {:.1} {:.1}]..[{:.1} {:.1} {:.1}]", lo[0], lo[1], lo[2], hi[0], hi[1], hi[2])).unwrap_or_default();
        outln!("{n}: {} joints, {} meshes, {tris} triangles, {} textures, {} warnings  {bounds}",
            m.joints.len(), m.meshes.len(), m.textures.len(), m.warnings.len());
        for w in m.warnings.iter().take(3) { outln!("   warn: {w}"); }
        std::fs::write(Path::new(out).join(format!("{n}.glb")), colo_export::glb::write_glb(&m, &n))?;
    }
    Ok(())
}

/// Export a room for the viewer: map model, NPC models, and `scene.json` with placements.
fn cmd_export_room(path: &str, room_name: &str, out: &str) -> R<()> {
    use colo_formats::{common::Common, hsd::{load_model, Archive}, maprel::MapRel};
    let mut w = World::open(path)?;
    let common = Common::parse(&w.common_bytes)?;
    let rooms = common.rooms()?;
    let models = common.character_models()?;
    let data = w.disc.read(&format!("{room_name}.fsys"))?;
    let fs = Fsys::parse(&data)?;
    let room = rooms.iter().find(|r| r.group_id == fs.group_id).ok_or("no room record")?.clone();
    let map = match fs.find(room_name, FileType::Rel) { Some(e) => MapRel::parse(&fs.read(e)?, &room)?, None => Default::default() };
    std::fs::create_dir_all(out)?;
    let convert = |e: &colo_formats::fsys::Entry, file: &str| -> R<String> {
        let b = fs.read(e)?;
        let ar = Archive::parse(&b)?;
        let m = load_model(&ar)?;
        std::fs::write(Path::new(out).join(file), colo_export::glb::write_glb(&m, &e.name))?;
        Ok(format!("{} ({} meshes, {} tris, {} textures)", e.name, m.meshes.len(), m.triangle_count(), m.textures.len()))
    };
    let mut json = String::from("{\n");
    let rdat = fs.find(room_name, FileType::Rdat).or_else(|| fs.find_type(FileType::Rdat).next()).ok_or("room has no .rdat")?;
    outln!("map: {}", convert(rdat, "map.glb")?);
    json += &format!("  \"room\": \"{room_name}\",\n  \"floor_id\": {},\n  \"map\": \"map.glb\",\n  \"npcs\": [\n", room.floor_id);
    let mut done = std::collections::HashSet::new();
    let mut npc_lines = vec![];
    for c in &map.characters {
        let Some(md) = models.get(c.people_index as usize) else { continue };
        let Some(e) = fs.entries.iter().find(|en| en.file_type == FileType::Dat && en.identifier & 0xFFFF_0000 == md.fsys_identifier & 0xFFFF_0000) else {
            outln!("npc {}: model {:#010x} not in this archive", c.index, md.fsys_identifier);
            continue;
        };
        let file = format!("{}.glb", e.name);
        if done.insert(file.clone()) { outln!("npc model: {}", convert(e, &file)?); }
        npc_lines.push(format!(
            "    {{\"index\": {}, \"model\": \"{file}\", \"visible\": {}, \"position\": [{}, {}, {}], \"rotation_y_deg\": {}, \"talk_script\": {}, \"move_script\": {}}}",
            c.index, c.visible, c.position[0], c.position[1], c.position[2], c.rotation_y, c.talk_script, c.move_script));
    }
    json += &npc_lines.join(",\n");
    json += "\n  ],\n  \"entry_points\": [\n";
    json += &map.entry_points.iter().map(|e| format!("    {{\"index\": {}, \"position\": [{}, {}, {}], \"angle_deg\": {}}}", e.index, e.position[0], e.position[1], e.position[2], e.angle)).collect::<Vec<_>>().join(",\n");
    json += "\n  ]\n}\n";
    std::fs::write(Path::new(out).join("scene.json"), json)?;
    outln!("wrote {out}/scene.json ({} NPCs, {} entry points)", map.characters.len(), map.entry_points.len());
    Ok(())
}

/// Colosseum PKX: 0x40-byte header (u32 DAT size @0), then the DAT.
fn pkx_dat(b: &[u8]) -> R<Vec<u8>> {
    let n = u32::from_be_bytes(b.get(0..4).ok_or("short pkx")?.try_into()?) as usize;
    Ok(b.get(0x40..0x40 + n).ok_or("pkx: DAT size past end")?.to_vec())
}

/// Lens flare of a room: the `.gfl` and the sun position (`floorDataBiosGetSunResID` -> model part 0).
fn cmd_flare(path: &str, room_name: &str, png_dir: Option<&str>) -> R<()> {
    use colo_formats::{common::Common, gfl::Flare, hsd, maprel::MapRel};
    let mut w = World::open(path)?;
    let rooms = Common::parse(&w.common_bytes)?.rooms()?;
    let data = w.disc.read(&format!("{room_name}.fsys"))?;
    let fs = Fsys::parse(&data)?;
    let room = rooms.iter().find(|r| r.group_id == fs.group_id).ok_or("no room record")?;
    let m = MapRel::parse(&fs.read(fs.find(room_name, FileType::Rel).ok_or("room has no .rel")?)?, room)?;
    let e = fs.find_type(FileType::Gfl).next().ok_or("room has no .gfl")?;
    let f = Flare::parse(&fs.read(e)?)?;
    outln!("{}: max {} fade {} glare fade {} glare alpha front {} behind {}", e.name, f.max_dist, f.fade_dist, f.glare_fade_dist, f.glare_alpha_front, f.glare_alpha_behind);
    for el in &f.elements { outln!("  element mode {} slot {} pos {} scale {}", el.mode, el.slot, el.pos, el.scale); }
    for (slot, img) in &f.textures {
        outln!("  texture slot {slot}: {}x{}", img.width, img.height);
        if let Some(dir) = png_dir {
            std::fs::create_dir_all(dir)?;
            std::fs::write(Path::new(dir).join(format!("flare_{slot}.png")), colo_export::png::encode_rgba(img.width, img.height, &img.rgba))?;
        }
    }
    if m.sun_res_id == 0 { outln!("no sun res id: flare stays off"); return Ok(()); }
    let set = (m.sun_res_id & 0xFFF) as usize;
    let rd = fs.entries.iter().find(|en| matches!(en.file_type, FileType::Rdat | FileType::Dat) && en.identifier >> 16 == m.sun_res_id >> 16).ok_or("sun model not in the archive")?;
    let bytes = fs.read(rd)?;
    let model = hsd::load_model(&hsd::Archive::parse(&bytes)?)?;
    let j = *model.set_first_joint.get(set).ok_or("sun model set missing")?;
    // GSpartGetTransform(part 0) translation = the set's root joint translation
    let t = model.joints[j].rest_translation;
    outln!("sun res {:#x}: {} (id {:#x}) set {set} joint {j}: position [{:.2} {:.2} {:.2}]", m.sun_res_id, rd.name, rd.identifier, t[0], t[1], t[2]);
    Ok(())
}

fn cmd_ccd(spec: &str) -> R<()> {
    use colo_formats::ccd::Collision;
    let (n, bytes) = entry_of(spec, FileType::Ccd, None)?;
    let c = Collision::parse(&bytes)?;
    let mut by_slot = std::collections::BTreeMap::<usize, (usize, std::collections::BTreeSet<u16>, std::collections::BTreeSet<u16>)>::new();
    let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
    for (l, t) in c.triangles() {
        let e = by_slot.entry(l.slot).or_default();
        e.0 += 1; e.1.insert(t.surface); e.2.insert(t.index);
        for v in &t.vertices { for k in 0..3 { lo[k] = lo[k].min(v[k]); hi[k] = hi[k].max(v[k]); } }
    }
    outln!("{n}: {} groups; bounds [{:.1} {:.1} {:.1}]..[{:.1} {:.1} {:.1}]", c.groups.len(), lo[0], lo[1], lo[2], hi[0], hi[1], hi[2]);
    for (slot, (cnt, surf, idx)) in by_slot {
        let show = |s: &std::collections::BTreeSet<u16>| s.iter().take(24).map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        outln!("  list +{:#x}: {cnt} triangles; first u16s {{{}}}; second {{{}}}", 0x24 + 4 * slot, show(&surf), show(&idx));
    }
    Ok(())
}

fn cmd_check_models(path: &str) -> R<()> {
    use colo_formats::hsd::{load_model, Archive};
    let mut disc = Disc::open(path)?;
    let files: Vec<_> = disc.files.iter().filter(|f| f.path.ends_with(".fsys")).cloned().collect();
    let (mut ok, mut failed, mut warned, mut tris, mut texs) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let mut seen = std::collections::HashSet::new();
    let mut examples = vec![];
    for f in &files {
        let data = disc.read_at(f.offset, f.size as usize)?;
        let fs = Fsys::parse(&data)?;
        for e in &fs.entries {
            if !matches!(e.file_type, FileType::Dat | FileType::Rdat | FileType::Pkx) { continue; }
            if !seen.insert((e.name.clone(), e.file_type)) { continue; }
            let mut b = fs.read(e)?;
            if e.file_type == FileType::Pkx { match pkx_dat(&b) { Ok(x) => b = x, Err(err) => { failed += 1; examples.push(format!("{}: {err}", e.name)); continue; } } }
            match Archive::parse(&b).and_then(|a| load_model(&a)) {
                Ok(m) => {
                    ok += 1; tris += m.triangle_count(); texs += m.textures.len();
                    if !m.warnings.is_empty() { warned += 1; if examples.len() < 12 { examples.push(format!("{} ({}): {}", e.name, f.path, m.warnings[0])); } }
                }
                Err(err) => { failed += 1; if examples.len() < 12 { examples.push(format!("{} ({}): {err}", e.name, f.path)); } }
            }
        }
    }
    outln!("{ok} models loaded ({tris} triangles, {texs} textures), {warned} with warnings, {failed} failed");
    for x in examples { outln!("  {x}"); }
    Ok(())
}

fn cmd_materials(path: &str) -> R<()> {
    use colo_formats::hsd::{load_model, Archive};
    use std::collections::BTreeMap;
    let mut disc = Disc::open(path)?;
    let files: Vec<_> = disc.files.iter().filter(|f| f.path.ends_with(".fsys")).cloned().collect();
    let mut seen = std::collections::HashSet::new();
    let (mut modes, mut layers, mut stacks, mut tevs, mut anim_ch): (BTreeMap<String, usize>, BTreeMap<String, usize>, BTreeMap<String, usize>, BTreeMap<String, usize>, BTreeMap<u8, usize>) = Default::default();
    let mut example: BTreeMap<String, String> = BTreeMap::new();
    for f in &files {
        let data = disc.read_at(f.offset, f.size as usize)?;
        let fs = Fsys::parse(&data)?;
        for e in &fs.entries {
            if !matches!(e.file_type, FileType::Dat | FileType::Rdat | FileType::Pkx) { continue; }
            if !seen.insert((e.name.clone(), e.file_type)) { continue; }
            let mut b = fs.read(e)?;
            if e.file_type == FileType::Pkx { match pkx_dat(&b) { Ok(x) => b = x, Err(_) => continue } }
            let Ok(arc) = Archive::parse(&b) else { continue };
            let Ok(m) = load_model(&arc) else { continue };
            if let Ok(ma) = colo_formats::hsdanim::load_mat_animations(&arc) {
                for a in &ma.anims { for t in &a.tex { for tr in &t.tracks { *anim_ch.entry(tr.channel).or_default() += 1; } } }
            }
            let mut mobjs = std::collections::HashSet::new();
            for mesh in &m.meshes {
                let mt = &mesh.material;
                if mt.layers.iter().any(|l| l.flags & colo_formats::hsd::TEX_BUMP != 0) {
                    let len = |v: &[f32; 3]| (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                    let k = if mesh.binormals.is_empty() { "~bump meshes without NBT".to_string() } else {
                        let n = mesh.binormals.len() as f32;
                        format!("~bump meshes with NBT, |B| {:.3} |T| {:.3}", mesh.binormals.iter().map(len).sum::<f32>() / n, mesh.tangents.iter().map(len).sum::<f32>() / n)
                    };
                    let k = if mesh.binormals.is_empty() { k } else { k[..23].to_string() };
                    *modes.entry(k.clone()).or_default() += 1;
                    if !mesh.binormals.is_empty() {
                        let n = mesh.binormals.len() as f32;
                        example.entry(format!("{k} {}", e.name)).or_insert_with(|| format!("|B| {:.4} |T| {:.4}", mesh.binormals.iter().map(len).sum::<f32>() / n, mesh.tangents.iter().map(len).sum::<f32>() / n));
                    }
                }
                if !mobjs.insert(mt.mobj) { continue; }
                let prog = colo_formats::tev::build(mt);
                *modes.entry(format!("~nodes {:02}", prog.nodes.len())).or_default() += 1;
                *modes.entry(format!("~consts {:02}", prog.consts.len())).or_default() += 1;
                let rm = mt.render_mode;
                let k = format!("lit {} diff {} alpha {} spec {} toon {} shadow {} xlu {} pe {}", (rm >> 2) & 1, rm & 3, (rm >> 13) & 3, (rm >> 3) & 1, (rm >> 12) & 1, (rm >> 26) & 1, (rm >> 30) & 1, mt.has_pixel_engine as u8);
                *modes.entry(k).or_default() += 1;
                let mut st = vec![];
                for l in &mt.layers {
                    let k = format!("coord {} lightmap {:#x} color {} alpha {} bump {} src {} id {} tex {} tev {}", l.coord(), l.lightmap(), l.colormap(), l.alphamap(), (l.flags >> 24) & 1, l.src, l.id, l.texture.is_some() as u8, l.tev.is_some() as u8);
                    *layers.entry(k.clone()).or_default() += 1;
                    example.entry(k).or_insert_with(|| format!("{} {}", f.path, e.name));
                    st.push(format!("[c{} lm{:#x} cm{} am{}{}]", l.coord(), l.lightmap(), l.colormap(), l.alphamap(), if l.tev.is_some() { " tev" } else { "" }));
                    if let Some(t) = l.tev {
                        let k = format!("active {:#x} cop {} cb {} cs {} cc {} in {:x?} | aop {} ab {} as {} ac {} in {:x?}", t.active, t.color_op, t.color_bias, t.color_scale, t.color_clamp, t.color_in, t.alpha_op, t.alpha_bias, t.alpha_scale, t.alpha_clamp, t.alpha_in);
                        *tevs.entry(k.clone()).or_default() += 1;
                        example.entry(k).or_insert_with(|| format!("{} {} konst {:?} tev0 {:?} tev1 {:?}", f.path, e.name, t.konst, t.tev0, t.tev1));
                    }
                }
                let k = format!("lit {} spec {} {}", (rm >> 2) & 1, (rm >> 3) & 1, st.join(" "));
                *stacks.entry(k.clone()).or_default() += 1;
                example.entry(k).or_insert_with(|| format!("{} {}", f.path, e.name));
            }
        }
    }
    outln!("== render modes (materials)");
    for (k, v) in &modes { outln!("{v:6}  {k}"); }
    for (k, v) in example.iter().filter(|(k, _)| k.starts_with("~bump")) { outln!("  {k}: {v}"); }
    outln!("== TObjs");
    for (k, v) in &layers { outln!("{v:6}  {k}   e.g. {}", example[k]); }
    outln!("== TObj TEV descs");
    for (k, v) in &tevs { outln!("{v:6}  {k}   e.g. {}", example[k]); }
    outln!("== layer stacks");
    for (k, v) in &stacks { outln!("{v:6}  {k}   e.g. {}", example[k]); }
    outln!("== texture animation channels: {anim_ch:?}");
    Ok(())
}

/// One model's materials (raw .dat/.rdat): render mode, colours, TObjs and the TEV graph.
fn cmd_mats(path: &str) -> R<()> {
    use colo_formats::hsd::{load_model, Archive};
    let b = std::fs::read(path)?;
    let arc = Archive::parse(&b)?;
    let m = load_model(&arc)?;
    let mut seen = std::collections::HashSet::new();
    for (mi, mesh) in m.meshes.iter().enumerate() {
        let mt = &mesh.material;
        if !seen.insert(mt.mobj) { continue; }
        outln!("mesh {mi} mobj {:#x} rm {:#010x} diff {:?} amb {:?} spec {:?} shin {} alpha {} pe {}", mt.mobj, mt.render_mode, mt.diffuse, mt.ambient, mt.specular, mt.shininess, mt.alpha, mt.has_pixel_engine);
        for (li, l) in mt.layers.iter().enumerate() {
            let t = l.texture.map(|t| &m.textures[t]);
            outln!("  tobj {li} id {:#x} src {} flags {:#010x} coord {} lm {:#x} cm {} am {} blend {} tex {:?} tev {:?}", l.id, l.src, l.flags, l.coord(), l.lightmap(), l.colormap(), l.alphamap(), l.blending,
                t.map(|t| (t.format, t.width, t.height)), l.tev);
        }
        let p = colo_formats::tev::build(mt);
        for (k, n) in p.nodes.iter().enumerate() { outln!("  n{k} layer {:?} chan {:?} c {:?} a {:?}", n.layer, n.chan, n.color.map(|o| o.inputs), n.alpha.map(|o| o.inputs)); }
        outln!("  consts {:?}", p.consts);
    }
    Ok(())
}

/// A `.cam` file's base camera, animations and the pose (eye, interest, fov) at the given frames.
fn cmd_cam(spec: &str, name: &str, frames: &str) -> R<()> {
    use colo_formats::{hsd::Archive, hsdanim::load_camera};
    let data = load(spec)?;
    let fs = Fsys::parse(&data)?;
    let e = fs.entries.iter().find(|e| e.name == name && e.file_type == FileType::Cam).ok_or("no such .cam")?;
    let cam = load_camera(&Archive::parse(&fs.read(e)?)?)?.ok_or("no camera in the archive")?;
    outln!("{name} ({:#x}): base {:?}, roll {}", e.identifier, cam.base, cam.roll);
    for (i, a) in cam.anims.iter().enumerate() { outln!("  anim {i}: end frame {}", a.end_frame); }
    for f in frames.split(',').filter_map(|x| x.trim().parse::<f32>().ok()) {
        let p = cam.pose(0, f);
        outln!("  frame {f:6.1}: eye {:?} interest {:?} fov {} roll {}", p.eye, p.interest, p.fov, p.roll);
    }
    Ok(())
}

fn cmd_anims(spec: &str, name: &str) -> R<()> {
    use colo_formats::{hsd::{load_model, Archive}, hsdanim::load_animations};
    let data = load(spec)?;
    let fs = Fsys::parse(&data)?;
    let e = fs.entries.iter().find(|e| e.name == name && matches!(e.file_type, FileType::Dat | FileType::Rdat)).ok_or("no such model")?;
    let b = fs.read(e)?;
    let arc = Archive::parse(&b)?;
    let m = load_model(&arc)?;
    let anims = load_animations(&arc, &m.joint_index)?;
    outln!("{name}: {} joints, {} animations", m.joints.len(), anims.len());
    for a in &anims {
        let keys: usize = a.tracks.iter().map(|t| t.keys.len()).sum();
        let mut ch = [0usize; 11];
        for t in &a.tracks { if (t.channel as usize) < 11 { ch[t.channel as usize] += 1; } }
        outln!("  set {} anim {:>2}: {:>6.1} frames, {:>3} tracks ({} rot, {} trans, {} scale), {keys} keys",
            a.model_set, a.index, a.end_frame, a.tracks.len(), ch[1] + ch[2] + ch[3], ch[5] + ch[6] + ch[7], ch[8] + ch[9] + ch[10]);
    }
    let mats = colo_formats::hsdanim::load_mat_animations(&arc)?;
    outln!("{} material/texture animations, {} extra images", mats.anims.len(), mats.textures.len());
    for a in &mats.anims {
        let anim_ch = |t: &[colo_formats::hsdanim::Track]| { let mut v: Vec<u8> = t.iter().map(|x| x.channel).collect(); v.sort_unstable(); v.dedup(); v };
        let tex: Vec<String> = a.tex.iter().map(|t| format!("tobj {:#x} ch{:?} imgs {}", t.tobj, anim_ch(&t.tracks), t.images.len())).collect();
        let mat: Vec<String> = a.mat.iter().map(|t| format!("mobj {:#x} ch{:?}", t.mobj, anim_ch(&t.tracks))).collect();
        outln!("  set {} matanim {:>2}: {:>6.1} frames; tex [{}]; mat [{}]", a.model_set, a.index, a.end_frame, tex.join(", "), mat.join(", "));
        if std::env::var("COLO_MAT_KEYS").is_ok() {
            for m in &a.mat { for t in &m.tracks { outln!("    mobj {:#x} ch {:2}: {:?}", m.mobj, t.channel, t.keys.iter().map(|k| (k.frame, k.value)).collect::<Vec<_>>()); } }
            for x in &a.tex { for t in &x.tracks { outln!("    tobj {:#x} ch {:2}: {:?}", x.tobj, t.channel, t.keys.iter().map(|k| (k.frame, k.value)).collect::<Vec<_>>()); } }
        }
    }
    if let Ok(out) = std::env::var("COLO_POSE_OUT") {
        // COLO_POSE_OUT=dir: write <name>_a<anim>_f<frame>.glb for anims 0..3 at 0, 1/3, 2/3 of their length
        std::fs::create_dir_all(&out)?;
        let which: Vec<usize> = std::env::var("COLO_POSE_ANIMS").ok().map(|v| v.split(',').filter_map(|x| x.parse().ok()).collect()).unwrap_or_else(|| vec![0, 1, 2, 3]);
        let steps: usize = std::env::var("COLO_POSE_STEPS").ok().and_then(|v| v.parse().ok()).unwrap_or(3);
        for a in anims.iter().filter(|a| a.model_set == 0 && which.contains(&a.index)) {
            for k in 0..steps {
                let f = a.end_frame * k as f32 / steps as f32;
                let posed = colo_formats::hsd::pose(&m, &colo_formats::hsd::world_matrices(&m, &a.locals(&m, f)));
                std::fs::write(Path::new(&out).join(format!("{name}_a{}_f{}.glb", a.index, f as u32)), colo_export::glb::write_glb(&posed, name))?;
            }
        }
    }
    if let Some(a) = anims.first() {
        if let Some(t) = a.tracks.iter().find(|t| t.keys.len() > 2) {
            outln!("  sample: joint {} ({}) channel {}: {:?}", t.joint, m.joints[t.joint].name, t.channel,
                (0..=4).map(|i| t.sample(a.end_frame * i as f32 / 4.0).unwrap_or(0.0)).collect::<Vec<_>>());
        }
    }
    Ok(())
}
