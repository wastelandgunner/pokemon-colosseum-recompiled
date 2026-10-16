//! GS threads (the game's own cooperative scheduler, `GSthread*`): `threadExecute` / `_threadSwitch`, each GS
//! thread a host coroutine.
//!
//! The game switches GS threads with two asm routines (decompiled 2026-10-13, Working/ghidra):
//! - `threadExecute` 0x800F028C (scheduler side, from `GSthreadExecute*` 0x800F0A74): saves the scheduler's GPRs
//!   into the context at `*(r13-0x5BFC)` (0x804019F0, +0x84 = 0x800F02F4), loads the thread's context
//!   `*(r13-0x5C00)` (GPRs, FPRs when `*(r13-0x5C10)`), LR = ctx+0x84, r1 = ctx+4, `*(r1) = -1`, r5 = ctx+0x14,
//!   r3 = ctx+0xC, jumps to ctx+0x80.
//! - `_threadSwitch` 0x800F0308 (thread side): stores r3/r5 at `r13-0x5C08`/`-0x5C0C`, saves the GPRs into the
//!   thread's context, ctx+0x80 = LR (the resume point), ctx+0xC = r3, ctx+0x14 = r5, ctx+4 = r1, FPRs when the
//!   flag is set; then loads the scheduler's context and returns into `threadExecute`'s tail 0x800F02F4.
//! Context layout: r0..r31 at +4n, +0x80 resume pc, +0x84 lr, f0..f31 (doubles) at +0x88.
//!
//! Here each GS thread runs on its own OS thread (a stackful coroutine) over the one shared `Cpu`; exactly one
//! side runs at a time (a baton), so guest registers and memory change only as on the console. Every register
//! and memory effect of the two routines is reproduced, including their scratch stores below r1. A thread
//! parked before the snapshot (or whose context was re-created) starts fresh at ctx+0x80 through
//! `lifted::resume`, which enters mid-function (lift.py `--resume`); when a function returns there, execution
//! continues at the restored LR, the same as `blr` on hardware.

use super::*;

pub const THREAD_EXECUTE: u32 = 0x800F028C;
pub const THREAD_SWITCH: u32 = 0x800F0308;
pub const GAPP_UPDATE: u32 = 0x800FE7A0;
/// `threadExecute`'s tail: the scheduler context's +0x84.
pub(super) const EXECUTE_TAIL: u32 = 0x800F02F4;

pub(super) const CUR_CTX: u32 = 0x5C04; // r13 - : the context the save/load routines use
pub(super) const THREAD_CTX: u32 = 0x5C00; // r13 - : the running thread's context
pub(super) const SCHED_CTX: u32 = 0x5BFC; // r13 - : the scheduler's context
pub(super) const FP_FLAG: u32 = 0x5C10; // r13 - : save/load FPRs
pub(super) const ARG_R3: u32 = 0x5C08;
pub(super) const ARG_R5: u32 = 0x5C0C;

pub(super) struct Co { pub(super) tx: Sender<Wake>, pub(super) resume_pc: u32 }

pub(super) struct Sched {
    pub(super) cos: HashMap<u32, Co>,
    pub(super) back_tx: Sender<Back>,
}

pub(super) static SCHED: Mutex<Option<Sched>> = Mutex::new(None);
pub(super) static BACK_RX: Mutex<Option<Receiver<Back>>> = Mutex::new(None);

thread_local! {
    /// Set on coroutine threads: (this thread's wake channel).
    static CO_RX: std::cell::RefCell<Option<Receiver<Wake>>> = const { std::cell::RefCell::new(None) };
    /// Where this GS coroutine reports its next switch (the `threadExecute` that resumed it).
    static CO_REPLY: std::cell::RefCell<Option<Sender<Back>>> = const { std::cell::RefCell::new(None) };
}

pub(super) fn sda(c: &Cpu, off: u32) -> u32 { c.r[13].wrapping_sub(off) }

/// `threadSaveGPRRegisters` 0x800F015C.
pub(super) fn save_gpr(c: &mut Cpu) {
    let sp = c.r[1].wrapping_sub(8);
    c.m.set_u32(sp, c.r[1]);
    c.m.set_u32(c.r[1], c.r[3]);
    let base = c.m.u32(sda(c, CUR_CTX));
    c.m.set_u32(base, c.r[0]);
    c.m.set_u32(base + 8, c.r[2]);
    c.m.set_u32(base + 0xC, base);
    for n in 4..32 { c.m.set_u32(base + 4 * n as u32, c.r[n]); }
}

/// `threadLoadGPRRegisters` 0x800F0030 (r3 comes back as it went in).
pub(super) fn load_gpr(c: &mut Cpu) {
    let sp = c.r[1].wrapping_sub(8);
    c.m.set_u32(sp, c.r[1]);
    c.m.set_u32(c.r[1], c.r[3]);
    let base = c.m.u32(sda(c, CUR_CTX));
    c.r[0] = c.m.u32(base);
    c.r[2] = c.m.u32(base + 8);
    for n in 4..32 { c.r[n] = c.m.u32(base + 4 * n as u32); }
    c.r[3] = c.m.u32(c.r[1]);
}

/// `threadSaveFPRRegisters` 0x800F01F0.
pub(super) fn save_fpr(c: &mut Cpu) {
    let sp = c.r[1].wrapping_sub(8);
    c.m.set_u32(sp, c.r[1]);
    c.m.set_u32(c.r[1], c.r[3]);
    let base = c.m.u32(sda(c, CUR_CTX)) + 0x88;
    for n in 0..32 { let v = c.f[n]; c.stfd(base + 8 * n as u32, v); }
}

/// `threadLoadFPRRegisters` 0x800F00C0.
pub(super) fn load_fpr(c: &mut Cpu) {
    let sp = c.r[1].wrapping_sub(8);
    c.m.set_u32(sp, c.r[1]);
    c.m.set_u32(c.r[1], c.r[3]);
    let base = c.m.u32(sda(c, CUR_CTX)) + 0x88;
    for n in 0..32 { c.f[n] = c.lfd(base + 8 * n as u32); }
}

/// `threadExecute` 0x800F028C, run on the scheduler side: switch to the thread and wait until it switches back.
pub(super) fn thread_execute(c: &mut Cpu) {
    // stwu r1,-0xC(r1); stw r3,0xC(r1); mflr r3; stw r3,8(r1)
    let sp = c.r[1].wrapping_sub(0xC);
    c.m.set_u32(sp, c.r[1]);
    c.r[1] = sp;
    c.m.set_u32(sp + 0xC, c.r[3]);
    c.r[3] = c.lr;
    c.m.set_u32(sp + 8, c.r[3]);
    let sched = c.m.u32(sda(c, SCHED_CTX));
    c.r[3] = sched;
    c.m.set_u32(sda(c, CUR_CTX), sched);
    save_gpr(c);
    c.m.set_u32(c.r[3] + 4, c.r[1]);
    let ctx = c.m.u32(sda(c, THREAD_CTX));
    c.r[3] = ctx;
    c.m.set_u32(sda(c, CUR_CTX), ctx);
    load_gpr(c);
    c.r[5] = c.m.u32(sda(c, FP_FLAG));
    if c.r[5] != 0 { load_fpr(c); }
    c.r[5] = c.m.u32(ctx + 0x84);
    c.lr = c.r[5];
    c.r[1] = c.m.u32(ctx + 4);
    c.r[5] = 0xFFFF_FFFF;
    c.m.set_u32(c.r[1], c.r[5]);
    let pc = c.m.u32(ctx + 0x80);
    c.ctr = pc;
    c.r[5] = c.m.u32(ctx + 0x14);
    c.r[3] = c.m.u32(ctx + 0xC);

    // bctr: wake the parked coroutine, or start one; it reports back on this call's own channel (another OS
    // thread may run a GS scheduler while this one waits)
    let (btx, brx) = channel::<Back>();
    {
        let mut g = SCHED.lock().unwrap();
        let s = g.as_mut().expect("run: no scheduler");
        match s.cos.get(&ctx) {
            Some(co) if co.resume_pc == pc => { co.tx.send(Wake::Resume(Some(btx))).ok(); }
            _ => {
                if let Some(old) = s.cos.remove(&ctx) { old.tx.send(Wake::Die).ok(); }
                let (tx, rx) = channel();
                let back = btx;
                let cp = CpuPtr(c as *mut Cpu);
                std::thread::Builder::new().name(format!("gs-thread {ctx:#010x}")).stack_size(CO_STACK)
                    .spawn(move || co_main(cp, pc, rx, back)).expect("run: spawn");
                s.cos.insert(ctx, Co { tx, resume_pc: 0 });
            }
        }
    }
    if trace() { eprintln!("run: {:?} runs GS thread {ctx:#x} at {pc:#x}", std::thread::current().name()); }
    let msg = brx.recv().expect("run: coroutine lost");
    match msg {
        Back::Yield { ctx, pc } => {
            if let Some(co) = SCHED.lock().unwrap().as_mut().and_then(|s| s.cos.get_mut(&ctx)) { co.resume_pc = pc; }
        }
        Back::Panic(m) => panic!("GS thread {ctx:#010x}: {m}"),
    }
    // the scheduler context's +0x84 = 0x800F02F4: lwz r3,8(r1); mtlr r3; lwz r3,0xC(r1); addi r1,r1,0xC
    debug_assert_eq!(c.lr, EXECUTE_TAIL);
    c.r[3] = c.m.u32(c.r[1] + 8);
    c.lr = c.r[3];
    c.r[3] = c.m.u32(c.r[1] + 0xC);
    c.r[1] += 0xC;
}

/// `_threadSwitch` 0x800F0308 on a coroutine: save the thread, hand back to the scheduler, park.
pub(super) fn thread_switch(c: &mut Cpu) {
    let on_co = CO_RX.with(|r| r.borrow().is_some());
    if !on_co { return; } // outside a GS thread (headless replays): a plain return
    c.m.set_u32(sda(c, ARG_R3), c.r[3]);
    c.m.set_u32(sda(c, ARG_R5), c.r[5]);
    c.r[5] = c.lr;
    let ctx = c.m.u32(sda(c, THREAD_CTX));
    c.r[3] = ctx;
    c.m.set_u32(sda(c, CUR_CTX), ctx);
    save_gpr(c);
    c.m.set_u32(ctx + 0x80, c.r[5]);
    let pc = c.r[5];
    c.r[5] = c.m.u32(sda(c, ARG_R3));
    c.m.set_u32(ctx + 0xC, c.r[5]);
    c.r[5] = c.m.u32(sda(c, ARG_R5));
    c.m.set_u32(ctx + 0x14, c.r[5]);
    c.m.set_u32(ctx + 4, c.r[1]);
    c.r[5] = c.m.u32(sda(c, FP_FLAG));
    if c.r[5] != 0 { save_fpr(c); }
    let sched = c.m.u32(sda(c, SCHED_CTX));
    c.r[3] = sched;
    c.m.set_u32(sda(c, CUR_CTX), sched);
    load_gpr(c);
    c.r[5] = c.m.u32(sched + 0x84);
    c.lr = c.r[5];
    c.r[1] = c.m.u32(sched + 4);
    c.r[5] = c.m.u32(sched + 0x14);
    c.r[3] = c.m.u32(sched + 0xC);
    // blr into the scheduler; park until threadExecute picks this thread again (it reloads the registers)
    CO_REPLY.with(|r| { if let Some(b) = r.borrow().as_ref() { b.send(Back::Yield { ctx, pc }).ok(); } });
    let w = CO_RX.with(|r| r.borrow().as_ref().unwrap().recv());
    match w {
        Ok(Wake::Resume(rep)) => { if let Some(b) = rep { CO_REPLY.with(|r| *r.borrow_mut() = Some(b)); } }
        _ => std::panic::panic_any(Die),
    }
}

pub(super) fn co_main(cp: CpuPtr, entry: u32, rx: Receiver<Wake>, back: Sender<Back>) {
    CO_RX.with(|r| *r.borrow_mut() = Some(rx));
    CO_REPLY.with(|r| *r.borrow_mut() = Some(back));
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // SAFETY: the scheduler is blocked in thread_execute until this side switches back (one runs at a time)
        let c = unsafe { &mut *cp.0 };
        let mut pc = entry;
        loop {
            lifted::resume(c, pc);
            pc = c.lr; // blr out of the outermost frame: continue at the restored LR (thread exit 0x800F0F4C last)
        }
    }));
    if let Err(e) = r {
        if e.is::<Die>() { return; }
        let m = e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default();
        CO_REPLY.with(|r| { if let Some(b) = r.borrow().as_ref() { b.send(Back::Panic(m)).ok(); } });
    }
}

/// The GS threads that have run here: (context, resume pc), for diagnostics.
pub fn threads() -> Vec<(u32, u32)> {
    let mut v: Vec<(u32, u32)> = SCHED.lock().unwrap().as_ref().map(|s| s.cos.iter().map(|(k, c)| (*k, c.resume_pc)).collect()).unwrap_or_default();
    v.sort();
    v
}
