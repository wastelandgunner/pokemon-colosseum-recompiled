//! `story-explore`: play a set of rooms' story greedily with the real scripts, to find the flags a
//! playthrough sets. The hero is always where a script checks, battles are won, Yes is chosen, random
//! numbers are 0, and Shadow Pokémon are snagged.

use colo_formats::{common::Common, fsys::{FileType, Fsys}, scd::Script};
use colo_script::{Host, Memory, NativeCall, NativeResult, Vm};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

type R<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)]
struct World {
    flags: BTreeSet<u32>,
    vis: BTreeMap<(u32, u32), bool>,
    visited: BTreeSet<u32>,
    /// The room the hero is in and its running VM (main loop thread, script data variables).
    cur: Option<usize>,
    vm: Option<Vm>,
}

struct RoomInfo {
    /// Reachable once this flag is set (0: always).
    gate: u32,
    name: String,
    floor: u32,
    group: u32,
    script: Script,
    modify: u32,
    entry: u32,
    main: u32,
    npcs: Vec<(u32, bool, u32)>,
    ips: Vec<(u32, [u32; 4], u8)>,
}

struct ExHost<'a> {
    flags: &'a mut BTreeSet<u32>,
    vis: &'a mut BTreeMap<(u32, u32), bool>,
    shadow: &'a [(u16, u16, u16)],
    floor: u32,
    group: u32,
    changes: Vec<String>,
}

impl ExHost<'_> {
    fn set(&mut self, f: u32, on: bool) {
        let ch = if on { self.flags.insert(f) } else { self.flags.remove(&f) };
        if ch { self.changes.push(format!("{}{f}", if on { "+" } else { "-" })); }
    }
    fn fl(&self, n: u32) -> bool { self.flags.contains(&n) }
}

impl Host for ExHost<'_> {
    fn call_native(&mut self, c: &NativeCall, _m: &mut dyn Memory) -> NativeResult {
        let u = |i: usize| c.args.get(i).copied().unwrap_or(0);
        if std::env::var("STORY_TRACE").is_ok() { eprintln!("      {}({:?}) first={}", c.name(), c.args, c.first_call); }
        let r = match c.name() {
            "flagSetTrue" => { self.set(u(0), true); 0 }
            "flagSetFalse" => { self.set(u(0), false); 0 }
            "flagCheckTrue" => self.fl(u(0)) as u32,
            "battleGetResult" => 2,
            "getCurrentRoomID" => self.floor,
            "getCurrentGroupID" => self.group,
            "characterCheckWithinBounds" => 1,
            // one main-loop iteration per tick
            "playerProcessEvents" => if c.first_call { return NativeResult::Pending } else { 0 },
            "characterSetVisibility" => { self.vis.insert((self.floor, u(1)), u(2) != 0); 0 }
            "shadowPokemonSetState" => {
                let i = u(0) as usize;
                if let Some(&(reg, st, _)) = self.shadow.get(i).filter(|_| (1..=0x60).contains(&i)) {
                    if reg != 0 { self.set(reg as u32, true); }
                    if st != 0 { self.set(st as u32, u(1) != 0); }
                }
                0
            }
            "tvSetChannelByStory" | "tvNewsEvent" => {
                let caught = self.shadow.get(0x43).is_some_and(|&(_, _, f)| f != 0 && self.fl(f as u32));
                if c.name() == "tvNewsEvent" && !caught {
                    let f = if self.fl(0xADA) { 0xA0F } else if self.fl(0xADB) { 0x483 } else if self.fl(0x42C) { 0x42D } else if self.fl(0xAD3) { 0xA08 }
                        else if self.fl(0xAD0) { 0xA07 } else if self.fl(0xACF) { 0xA06 } else { 0xA05 };
                    self.set(f, true);
                }
                if ![0xADA, 0xADB, 0x42C, 0xAD3, 0xAD0, 0xACF].iter().any(|&n| self.fl(n)) { self.set(0xA05, true); }
                1
            }
            _ => 0,
        };
        NativeResult::Done(r)
    }
}

const TICKS: u32 = 600;

/// Run a thread in the current room's VM (alongside its main loop) for a while.
fn run_in(r: &RoomInfo, w: &mut World, shadow: &[(u16, u16, u16)], tid: u32, args: &[u32]) -> Vec<String> {
    let Some(mut vm) = w.vm.take() else { return vec![] };
    let mut h = ExHost { flags: &mut w.flags, vis: &mut w.vis, shadow, floor: r.floor, group: r.group, changes: vec![] };
    if let Some(s) = vm.spawn(tid, args, &mut h) {
        vm.run_now(s, &mut h);
        for _ in 0..TICKS { vm.tick(&mut h); }
        vm.reap();
    }
    let ch = h.changes;
    w.vm = Some(vm);
    ch
}

/// Enter a room: fresh VM, modify + entry scripts, main loop started.
fn visit(ri: usize, r: &RoomInfo, global: &Script, w: &mut World, shadow: &[(u16, u16, u16)]) -> Vec<String> {
    if w.visited.insert(r.floor) {
        for (i, vis, _) in &r.npcs { w.vis.insert((r.floor, *i), *vis); }
    }
    let mut vm = Vm::new();
    vm.load(global.clone());
    vm.load(r.script.clone());
    w.vm = Some(vm);
    w.cur = Some(ri);
    let mut ch = vec![];
    for t in [r.modify, r.entry] { if t != 0 { ch.extend(run_in(r, w, shadow, t, &[])); } }
    if r.main != 0 { ch.extend(run_in(r, w, shadow, r.main, &[])); }
    ch
}

#[derive(Clone)]
enum Act { Visit(usize), Talk(usize, u32, u32), Ip(usize, u32, [u32; 4]) }

/// `story-explore <dir> <rooms> [lo-hi] [+flag ...]`: rooms are `NAME` or `NAME@flag` (reachable once the
/// flag is set). Each step enters a room, or in the current room talks to a visible person or uses an
/// interaction point. The step setting the lowest new story flag in lo..=hi (default 986-1015) is taken,
/// else one that enables such a step. Prints the steps and the final flags.
pub fn cmd_story_explore(dir: &str, rooms_arg: &str, rest: &[String]) -> R<()> {
    let d = Path::new(dir);
    let read = |n: &str| std::fs::read(d.join(n)).or_else(|_| std::fs::read(d.join("files").join(n)));
    let cf = read("common.fsys")?;
    let cfs = Fsys::parse(&cf)?;
    let common_bytes = cfs.read(cfs.find("common_rel", FileType::Rel).ok_or("no common_rel")?)?;
    let common = Common::parse(&common_bytes)?;
    let records = common.rooms()?;
    let global = common.script()?;
    let shadow = common.shadow_flags()?;
    let ips_all = common.interaction_points()?;
    let (mut lo, mut hi) = (986u32, 1015u32);
    let mut w = World { flags: BTreeSet::new(), vis: BTreeMap::new(), visited: BTreeSet::new(), cur: None, vm: None };
    for a in rest {
        if let Some(n) = a.strip_prefix('+') { w.flags.insert(n.parse()?); }
        else if let Some((a, b)) = a.split_once('-') { lo = a.parse()?; hi = b.parse()?; }
    }
    let mut rooms = vec![];
    for spec in rooms_arg.split(',') {
        let (name, gate) = match spec.split_once('@') { Some((n, g)) => (n, g.parse::<u32>()?), None => (spec, 0) };
        let fsb = read(&format!("{name}.fsys"))?;
        let fs = Fsys::parse(&fsb)?;
        let rec = records.iter().find(|r| r.group_id == fs.group_id).ok_or("no room record")?.clone();
        let script = Script::parse(fs.read(fs.find(name, FileType::Scd).ok_or("no scd")?)?)?;
        let mrel = match fs.find(name, FileType::Rel) { Some(e) => colo_formats::maprel::MapRel::parse(&fs.read(e)?, &rec)?, None => Default::default() };
        let npcs = mrel.characters.iter().map(|c| (c.index as u32, c.visible, c.talk_script)).collect();
        let ips = ips_all.iter().filter(|p| p.room_id as u32 == rec.floor_id && p.script != 0x0596_0004 && p.script != 0x0596_0005 && p.method != 4)
            .map(|p| (p.script, p.params, p.method)).collect();
        rooms.push(RoomInfo { gate, name: name.to_string(), floor: rec.floor_id, group: rec.group_id, script, modify: rec.scripts[1], entry: rec.scripts[2], main: rec.scripts[3], npcs, ips });
    }
    let acts = |w: &World| -> Vec<(Act, String)> {
        let mut v = vec![];
        for (ri, r) in rooms.iter().enumerate() {
            if r.gate != 0 && !w.flags.contains(&r.gate) { continue; }
            v.push((Act::Visit(ri), format!("enter {}", r.name)));
        }
        if let Some(ri) = w.cur {
            let r = &rooms[ri];
            for &(i, def, t) in &r.npcs {
                if t != 0 && *w.vis.get(&(r.floor, i)).unwrap_or(&def) { v.push((Act::Talk(ri, i, t), format!("{} talk #{i} ({:#x})", r.name, t))); }
            }
            for &(t, p, m) in &r.ips { v.push((Act::Ip(ri, t, p), format!("{} ip m{m} {:#x} {:x?}", r.name, t, p))); }
        }
        v
    };
    let apply = |w: &mut World, a: &Act| -> Vec<String> {
        match a {
            Act::Visit(ri) => visit(*ri, &rooms[*ri], &global, w, &shadow),
            Act::Talk(ri, i, t) => { let r = &rooms[*ri]; run_in(r, w, &shadow, *t, &[r.group, *i, 0, 0]) }
            Act::Ip(ri, t, p) => { let r = &rooms[*ri]; run_in(r, w, &shadow, *t, p) }
        }
    };
    // story flags: those a room's main loop tests (the loops gate the events), within lo..=hi
    let mut gating: BTreeSet<u32> = BTreeSet::new();
    for r in &rooms {
        if r.main == 0 { continue; }
        let fi = (r.main & 0xFFFF) as usize;
        if let Some(&start) = r.script.functions.get(fi) {
            let end = r.script.functions.iter().copied().filter(|&x| x > start).min().unwrap_or(r.script.bytes.len());
            let mut pc = start;
            let mut imms: Vec<u32> = vec![];
            while pc < end {
                let Ok((op, next)) = colo_formats::scd::decode(&r.script.bytes, pc) else { break };
                match op {
                    colo_formats::scd::Op::PushImm { value, .. } => imms.push(value),
                    colo_formats::scd::Op::CallStd { argc } => {
                        let n = argc as usize;
                        if n >= 2 && imms.len() >= n && imms[imms.len() - n] == 81 { gating.insert(imms[imms.len() - n + 1]); }
                        imms.clear();
                    }
                    _ => {}
                }
                pc = next;
            }
        }
    }
    gating.retain(|n| (lo..=hi).contains(n));
    for a in rest { if let Some(n) = a.strip_prefix('=') { gating.insert(n.parse()?); } }
    println!("story flags (tested by main loops): {gating:?}");
    let low = |ch: &[String]| ch.iter().filter(|c| c.starts_with('+')).filter_map(|c| c[1..].parse::<u32>().ok()).filter(|n| gating.contains(n)).min();
    let dbg = std::env::var("STORY_DEBUG").is_ok();
    type Best = Option<(u32, World, String, Vec<String>)>;
    let start = w.clone();
    let mut taken: Vec<(Act, String)> = vec![];
    for _ in 0..300 {
        let cands = acts(&w);
        let mut best: Best = None;
        for (a, label) in &cands {
            let mut t = w.clone();
            let ch = apply(&mut t, a);
            if dbg { eprintln!("  try {label}: {}", ch.join(" ")); }
            if let Some(m) = low(&ch) { if best.as_ref().is_none_or(|b| m < b.0) { best = Some((m, t, label.clone(), ch)); } }
        }
        if let Some((_, t, label, ch)) = best {
            println!("{label}: {}", ch.join(" "));
            taken.push((cands.iter().find(|c| c.1 == label).unwrap().0.clone(), label));
            w = t;
            continue;
        }
        // a step that enables a story step next (lowest flag wins)
        let mut best: Best = None;
        for (a, label) in &cands {
            let mut t = w.clone();
            let ch = apply(&mut t, a);
            if ch.is_empty() && t.vis == w.vis && t.visited == w.visited && t.cur == w.cur { continue; }
            for (a2, _) in acts(&t) {
                let mut u = t.clone();
                if let Some(m) = low(&apply(&mut u, &a2)) { if best.as_ref().is_none_or(|b| m < b.0) { best = Some((m, t.clone(), label.clone(), ch.clone())); } }
            }
        }
        if let Some((_, t, label, ch)) = best {
            println!("{label}: {} (enables)", ch.join(" "));
            taken.push((cands.iter().find(|c| c.1 == label).unwrap().0.clone(), label));
            w = t;
            continue;
        }
        break;
    }
    // minimise: drop every step whose removal still reaches the goal flags (=N arguments)
    let goals: Vec<u32> = rest.iter().filter_map(|a| a.strip_prefix('=')).filter_map(|n| n.parse().ok()).collect();
    if !goals.is_empty() && goals.iter().all(|g| w.flags.contains(g)) {
        let replay = |steps: &[(Act, String)]| -> World {
            let mut t = start.clone();
            for (a, _) in steps {
                // only what the player could do at that point
                let ok = match a {
                    Act::Visit(ri) => rooms[*ri].gate == 0 || t.flags.contains(&rooms[*ri].gate),
                    Act::Talk(ri, i, _) => t.cur == Some(*ri) && *t.vis.get(&(rooms[*ri].floor, *i)).unwrap_or(&false),
                    Act::Ip(ri, _, _) => t.cur == Some(*ri),
                };
                if ok { apply(&mut t, a); }
            }
            t
        };
        let mut steps = taken.clone();
        let mut i = 0;
        while i < steps.len() {
            let mut trial = steps.clone();
            trial.remove(i);
            let t = replay(&trial);
            if goals.iter().all(|g| t.flags.contains(g)) { println!("  optional: {}", steps[i].1); steps = trial; } else { i += 1; }
        }
        let t = replay(&steps);
        println!("required steps:");
        for (_, l) in &steps { println!("  {l}"); }
        println!("minimal flags: {}", t.flags.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
    }
    println!("final flags: {}", w.flags.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
    Ok(())
}
