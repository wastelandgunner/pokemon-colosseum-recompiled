//! Function-fact recorder for naming: with `Cpu::prof` and `Mem::trace` on, every lifted function call
//! records its arguments, return value, the move-script command running at entry, its callers, and the
//! memory it touched (as offsets from its pointer arguments, or as absolute addresses). `dump` writes one
//! JSON line per function for `tools/ppclift/name_facts.py`.

use crate::mem::PEND;
use crate::ppc::Cpu;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Default)]
pub struct Rec {
    pub calls: u64,
    pub args: Vec<[u32; 5]>,
    pub rets: Vec<u32>,
    pub callers: BTreeSet<u32>,
    pub ws: BTreeSet<u8>,
    /// (base: 0..3 = r3..r6 argument, 9 = absolute) , offset/address, length, write) -> count
    pub acc: BTreeMap<(u8, u32, u8, bool), u32>,
}

struct Frame { f: u32, args: [u32; 5], sp: u32 }

thread_local! {
    static STACK: RefCell<Vec<Frame>> = const { RefCell::new(Vec::new()) };
    pub static RECS: RefCell<HashMap<u32, Rec>> = RefCell::new(HashMap::new());
}

const ARG_SPAN: u32 = 0x1000;

fn flush() {
    let pend: Vec<(u32, u8, bool)> = PEND.with(|p| std::mem::take(&mut *p.borrow_mut()));
    if pend.is_empty() { return; }
    STACK.with(|s| {
        let s = s.borrow();
        let Some(top) = s.last() else { return };
        RECS.with(|r| {
            let mut r = r.borrow_mut();
            let rec = r.entry(top.f).or_default();
            for (a, n, w) in pend {
                // the function's own stack frame and below are locals: skip
                if a < top.sp && a + 0x4000 > top.sp { continue; }
                let mut key = (9u8, a, n, w);
                for (k, &p) in top.args[..4].iter().enumerate() {
                    if p >= 0x80000000 && a >= p && a < p + ARG_SPAN { key = (k as u8, a - p, n, w); break; }
                }
                *rec.acc.entry(key).or_insert(0) += 1;
            }
        });
    });
}

pub fn enter(c: &mut Cpu, f: u32) {
    flush();
    let args = [c.r[3], c.r[4], c.r[5], c.r[6], c.r[7]];
    let caller = STACK.with(|s| s.borrow().last().map(|x| x.f).unwrap_or(0));
    let trace = c.m.trace;
    c.m.trace = false;
    let ws = {
        let pc = c.m.u32(0x8047B610);
        if (0x80375E48..0x80379BFF).contains(&pc) { Some(c.m.u8(pc)) } else { None }
    };
    c.m.trace = trace;
    RECS.with(|r| {
        let mut r = r.borrow_mut();
        let rec = r.entry(f).or_default();
        rec.calls += 1;
        if rec.args.len() < 24 { rec.args.push(args); }
        rec.callers.insert(caller);
        if let Some(op) = ws { rec.ws.insert(op); }
    });
    STACK.with(|s| s.borrow_mut().push(Frame { f, args, sp: c.r[1] }));
}

pub fn exit(c: &mut Cpu, f: u32) {
    flush();
    let r3 = c.r[3];
    STACK.with(|s| { s.borrow_mut().pop(); });
    RECS.with(|r| {
        let mut r = r.borrow_mut();
        let rec = r.entry(f).or_default();
        if rec.rets.len() < 24 { rec.rets.push(r3); }
    });
}

/// One JSON object per function, sorted by address.
pub fn dump() -> String {
    use std::fmt::Write;
    let mut out = String::new();
    RECS.with(|r| {
        let r = r.borrow();
        let mut keys: Vec<&u32> = r.keys().collect();
        keys.sort();
        for k in keys {
            let x = &r[k];
            let args: Vec<String> = x.args.iter().map(|a| format!("[{}]", a.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","))).collect();
            let acc: Vec<String> = x.acc.iter().map(|((b, o, n, w), c)| format!("[{b},{o},{n},{},{c}]", *w as u8)).collect();
            let _ = writeln!(out, "{{\"addr\":{k},\"calls\":{},\"args\":[{}],\"rets\":[{}],\"callers\":[{}],\"ws\":[{}],\"acc\":[{}]}}",
                x.calls, args.join(","), x.rets.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","),
                x.callers.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","),
                x.ws.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","), acc.join(","));
        }
    });
    out
}
