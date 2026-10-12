//! `story`: run story event functions in order with a host that keeps flags, to derive the flag state a
//! playthrough reaches. Battles are won, Yes is chosen, random numbers are 0, the hero is always in bounds.

use colo_formats::{common::Common, fsys::{FileType, Fsys}, scd::Script};
use colo_script::{Host, Memory, NativeCall, NativeResult, Vm};
use std::collections::BTreeSet;
use std::path::Path;

type R<T> = Result<T, Box<dyn std::error::Error>>;

pub struct StoryHost {
    pub flags: BTreeSet<u32>,
    pub shadow: Vec<(u16, u16, u16)>,
    pub floor: u32,
    pub log: Vec<String>,
    pub trace: bool,
}

impl StoryHost {
    fn set(&mut self, f: u32, on: bool, why: &str) {
        let changed = if on { self.flags.insert(f) } else { self.flags.remove(&f) };
        if changed { self.log.push(format!("    {}{f} ({why})", if on { "+" } else { "-" })); }
    }
    fn by_story(&mut self) {
        let f = |s: &Self, n: u32| s.flags.contains(&n);
        if !(f(self, 0xADA) || f(self, 0xADB) || f(self, 0x42C) || f(self, 0xAD3) || f(self, 0xAD0) || f(self, 0xACF)) { self.set(0xA05, true, "tvSetChannelByStory"); }
    }
}

impl Host for StoryHost {
    fn call_native(&mut self, c: &NativeCall, _m: &mut dyn Memory) -> NativeResult {
        let name = c.name();
        if self.trace { self.log.push(format!("      {name}({:?})", c.args)); }
        let u = |i: usize| c.args.get(i).copied().unwrap_or(0);
        let r = match name {
            "flagSetTrue" => { self.set(u(0), true, "flagSetTrue"); 0 }
            "flagSetFalse" => { self.set(u(0), false, "flagSetFalse"); 0 }
            "flagCheckTrue" => self.flags.contains(&u(0)) as u32,
            "battleGetResult" => 2,
            "getCurrentRoomID" => self.floor,
            "characterCheckWithinBounds" => 1,
            "shadowPokemonSetState" => {
                let i = u(0) as usize;
                if let Some(&(reg, st, _)) = self.shadow.get(i).filter(|_| (1..=0x60).contains(&i)) {
                    if reg != 0 { self.set(reg as u32, true, "shadow registered"); }
                    if st != 0 { self.set(st as u32, u(1) != 0, "shadow state"); }
                }
                0
            }
            "tvSetChannelByStory" => { self.by_story(); 1 }
            "tvNewsEvent" => {
                let f = |s: &Self, n: u32| s.flags.contains(&n);
                let caught = self.shadow.get(0x43).is_some_and(|&(_, _, fl)| fl != 0 && f(self, fl as u32));
                if !caught && f(self, 0xADA) { self.set(0xA0F, true, "tvNewsEvent"); }
                else if !caught && f(self, 0xADB) { self.set(0x483, true, "tvNewsEvent"); }
                else if !caught {
                    let fl = if f(self, 0x42C) { 0x42D } else if f(self, 0xAD3) { 0xA08 } else if f(self, 0xAD0) { 0xA07 } else if f(self, 0xACF) { 0xA06 } else { 0xA05 };
                    self.set(fl, true, "tvNewsEvent");
                }
                self.by_story();
                1
            }
            _ => 0,
        };
        NativeResult::Done(r)
    }
    fn log(&mut self, _line: &str) {}
}

/// `story <dir> <plan>`: plan lines `ROOM func_N`, `+flag`, `-flag`, `snag N` (Shadow entry N snagged: its
/// +0x12 flag), `# ...`. Prints the flags each step changes and the final set.
pub fn cmd_story(dir: &str, plan: &str) -> R<()> {
    let d = Path::new(dir);
    let read = |n: &str| std::fs::read(d.join(n)).or_else(|_| std::fs::read(d.join("files").join(n)));
    let cf = read("common.fsys")?;
    let cfs = Fsys::parse(&cf)?;
    let common_bytes = cfs.read(cfs.find("common_rel", FileType::Rel).ok_or("no common_rel")?)?;
    let common = Common::parse(&common_bytes)?;
    let rooms = common.rooms()?;
    let global = common.script()?;
    let mut host = StoryHost { flags: BTreeSet::new(), shadow: common.shadow_flags()?, floor: 0, log: vec![], trace: std::env::var("STORY_TRACE").is_ok() };
    for line in std::fs::read_to_string(plan)?.lines() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') { continue; }
        if let Some(n) = l.strip_prefix('+') { host.set(n.parse()?, true, "plan"); continue; }
        if let Some(n) = l.strip_prefix('-') { host.set(n.parse()?, false, "plan"); continue; }
        if let Some(n) = l.strip_prefix("snag ") {
            let i: usize = n.trim().parse()?;
            if let Some(&(reg, _, caught)) = host.shadow.get(i) { host.set(reg as u32, true, "snag"); host.set(caught as u32, true, "snag"); }
            continue;
        }
        let mut it = l.split_whitespace();
        let (Some(room), Some(func)) = (it.next(), it.next()) else { continue };
        let args: Vec<u32> = it.map(|x| x.parse::<i64>().map(|v| v as u32)).collect::<Result<_, _>>()?;
        let fs_bytes = read(&format!("{room}.fsys"))?;
        let fs = Fsys::parse(&fs_bytes)?;
        let rec = rooms.iter().find(|r| r.group_id == fs.group_id).ok_or("no room record")?;
        host.floor = rec.floor_id;
        let script = Script::parse(fs.read(fs.find(room, FileType::Scd).or_else(|| fs.find_type(FileType::Scd).next()).ok_or("no scd")?)?)?;
        let fi: u16 = func.trim_start_matches("func_").parse()?;
        let tid = script.thread_id(fi);
        let mut vm = Vm::new();
        vm.load(global.clone());
        vm.load(script);
        let serial = vm.spawn(tid, &args, &mut host).ok_or("spawn failed")?;
        host.log.push(format!("{room} {func}"));
        vm.run_now(serial, &mut host);
        let mut n = 0;
        while !vm.is_idle() && n < 20000 { vm.tick(&mut host); n += 1; }
        for l in host.log.drain(..) { println!("{l}"); }
    }
    println!("flags: {}", host.flags.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
    Ok(())
}

