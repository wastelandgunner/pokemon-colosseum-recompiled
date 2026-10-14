//! Whole game frames through the lifted code (colo-viewer's battle runner): `GSgappUpdate` per frame, with the GS
//! cooperative threads as host coroutines.
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

use crate::lifted;
use crate::ppc::Cpu;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;

pub const THREAD_EXECUTE: u32 = 0x800F028C;
pub const THREAD_SWITCH: u32 = 0x800F0308;
pub const GAPP_UPDATE: u32 = 0x800FE7A0;
/// `threadExecute`'s tail: the scheduler context's +0x84.
const EXECUTE_TAIL: u32 = 0x800F02F4;

const CUR_CTX: u32 = 0x5C04; // r13 - : the context the save/load routines use
const THREAD_CTX: u32 = 0x5C00; // r13 - : the running thread's context
const SCHED_CTX: u32 = 0x5BFC; // r13 - : the scheduler's context
const FP_FLAG: u32 = 0x5C10; // r13 - : save/load FPRs
const ARG_R3: u32 = 0x5C08;
const ARG_R5: u32 = 0x5C0C;

/// Coroutine stack: the lifted code recurses on the host stack.
const CO_STACK: usize = 256 << 20;

struct CpuPtr(*mut Cpu);
unsafe impl Send for CpuPtr {}

/// Resume carries the channel a GS coroutine reports back on (the scheduler call that resumed it).
enum Wake { Resume(Option<Sender<Back>>), Die }
enum Back { Yield { ctx: u32, pc: u32 }, Panic(String) }
struct Die;

struct Co { tx: Sender<Wake>, resume_pc: u32 }

struct Sched {
    cos: HashMap<u32, Co>,
    back_tx: Sender<Back>,
}

static SCHED: Mutex<Option<Sched>> = Mutex::new(None);
static BACK_RX: Mutex<Option<Receiver<Back>>> = Mutex::new(None);

thread_local! {
    /// Set on coroutine threads: (this thread's wake channel).
    static CO_RX: std::cell::RefCell<Option<Receiver<Wake>>> = const { std::cell::RefCell::new(None) };
    /// Where this GS coroutine reports its next switch (the `threadExecute` that resumed it).
    static CO_REPLY: std::cell::RefCell<Option<Sender<Back>>> = const { std::cell::RefCell::new(None) };
}

fn sda(c: &Cpu, off: u32) -> u32 { c.r[13].wrapping_sub(off) }

/// `threadSaveGPRRegisters` 0x800F015C.
fn save_gpr(c: &mut Cpu) {
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
fn load_gpr(c: &mut Cpu) {
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
fn save_fpr(c: &mut Cpu) {
    let sp = c.r[1].wrapping_sub(8);
    c.m.set_u32(sp, c.r[1]);
    c.m.set_u32(c.r[1], c.r[3]);
    let base = c.m.u32(sda(c, CUR_CTX)) + 0x88;
    for n in 0..32 { let v = c.f[n]; c.stfd(base + 8 * n as u32, v); }
}

/// `threadLoadFPRRegisters` 0x800F00C0.
fn load_fpr(c: &mut Cpu) {
    let sp = c.r[1].wrapping_sub(8);
    c.m.set_u32(sp, c.r[1]);
    c.m.set_u32(c.r[1], c.r[3]);
    let base = c.m.u32(sda(c, CUR_CTX)) + 0x88;
    for n in 0..32 { c.f[n] = c.lfd(base + 8 * n as u32); }
}

/// `threadExecute` 0x800F028C, run on the scheduler side: switch to the thread and wait until it switches back.
fn thread_execute(c: &mut Cpu) {
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
fn thread_switch(c: &mut Cpu) {
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

fn co_main(cp: CpuPtr, entry: u32, rx: Receiver<Wake>, back: Sender<Back>) {
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

pub const OS_SLEEP_THREAD: u32 = 0x800A238C;

/// The interrupts the game sleeps on: (thread queue, `__OSInterrupt`, handler). The host is the hardware: one
/// `OSSleepThread` on a queue = that interrupt firing once, handled with the interrupted context
/// (`OSGetCurrentContext`, 0x800000D4), which wakes the queue. Only the main OS thread exists here.
pub const INTERRUPTS: &[(u32, u32, u32)] = &[
    // VIWaitForRetrace's queue: __VIRetraceHandler 0x800A85DC (retraceCount++, pre/post-retrace callbacks)
    (0x8047A854, 0x18, 0x800A85DC),
    // GXDrawDone / GXWaitDrawDone's queue: the PE finish handler 0x800B90E8 (the GX stream is not executed, so the
    // GPU is done at once; sets the draw-done flag and runs the draw-done callback)
    (0x8047A9CC, 0x13, 0x800B90E8),
    // DVD: __DVDInterruptHandler 0x800A3EF0 (no queue of its own; transfers finish through `PENDING`)
    (0, 0x15, 0x800A3EF0),
    // ARAM DMA: __ARHandler 0x800ACB54 (acknowledges, runs the AR callback: the ARQ service routine)
    (0, 0x06, 0x800ACB54),
    // SI: SIInterruptHandler 0x800CFA60 (controller polling: the PAD sampling callback, inputPadCallback 0x800F8268)
    (0, 0x14, 0x800CFA60),
    // DSP mail: __DSPHandler 0x800AE9FC (task protocol; the AX yield runs MusyX's frame-done callback)
    (0, 0x07, 0x800AE9FC),
    // AI DMA: __AIDHandler 0x800AC628 (MusyX salCallback: next output buffer, synth frame, DSP command list)
    (0, 0x05, 0x800AC628),
];

/// The next AI DMA block's end (time base): one block (160 samples at 32 kHz) every 5 ms = 202,500 ticks; 0 = not
/// started. Blocks end at their own times (not bunched at fields) so each `__AIDHandler` finds the DSP's last
/// frame done, as on hardware.
static AI_NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
const TB_PER_AI: u64 = 202_500;
/// The AI DMA block now playing: (physical address, bytes).
static AI_LATCH: Mutex<Option<(u32, u32)>> = Mutex::new(None);

/// AI DMA blocks whose time has come: the AI interrupt for each, while the AI DMA runs (AIDCR 0xCC005036 bit 15).
fn ai_due(c: &mut Cpu) {
    use std::sync::atomic::Ordering::Relaxed;
    loop {
        let now = time_base(c);
        let n = AI_NEXT.load(Relaxed);
        if n == 0 || now > n + 8 * TB_PER_AI { AI_NEXT.store(now + TB_PER_AI, Relaxed); return; }
        if now < n { return; }
        AI_NEXT.store(n + TB_PER_AI, Relaxed);
        let dcr = c.m.u16(0xCC00_5036);
        if dcr & 0x8000 != 0 {
            // the block that just played is what the speakers got (AX writes R, L 16-bit pairs); the DMA then
            // latches the registers for the next block and interrupts so the game can queue another
            let mut latch = AI_LATCH.lock().unwrap();
            if let Some((a, len)) = *latch {
                let b = c.m.read(0x8000_0000 | a, len).to_vec();
                if let Some(ax) = AX.lock().unwrap().as_mut() {
                    ax.out.extend(b.chunks_exact(2).map(|x| i16::from_be_bytes([x[0], x[1]])));
                }
            }
            let a = (c.m.u16(0xCC00_5030) as u32 & 0x3FF) << 16 | (c.m.u16(0xCC00_5032) as u32 & 0xFFE0);
            *latch = Some((a, (dcr as u32 & 0x7FFF) * 32));
            drop(latch);
            raise(c, 0x05);
        } else if trace() { eprintln!("run: AI DMA off (AIDCR {:#x})", c.m.u16(0xCC00_5036)); }
    }
}

/// One video field: the VI retrace interrupt, then the SI poll that completes during it (SICOMCSR RDSTINT +
/// mask, SISR RDST0: controller 1 answered), which runs the polling handlers (PADRead through
/// inputPadCallback).
fn field(c: &mut Cpu) {
    // time jumps to the field: the AI blocks that end before it play out first, each with the DSP's answer
    {
        use std::sync::atomic::Ordering::Relaxed;
        let f = FIELD_TB.load(Relaxed) + TB_PER_FIELD;
        loop {
            let ai = AI_NEXT.load(Relaxed);
            if ai == 0 || ai > f || time_base(c) >= f { break; }
            if ai > time_base(c) { TB_BASE.store(ai, Relaxed); TB_MARK.store(c.steps, Relaxed); }
            ai_due(c);
            raise_pending(c);
        }
    }
    tb_field(c);
    raise(c, 0x18);
    // the SI poll only once polling is on (SIPOLL 0xCC006430 EN0-3, bits 7..4)
    if c.m.u32(0xCC00_6430) & 0xF0 == 0 { return; }
    let csr = c.m.u32(0xCC00_6434);
    c.m.set_u32(0xCC00_6434, csr | 0x1800_0000);
    let sr = c.m.u32(0xCC00_6438);
    c.m.set_u32(0xCC00_6438, sr | 0x2000_0000);
    raise(c, 0x14);
    let csr = c.m.u32(0xCC00_6434);
    c.m.set_u32(0xCC00_6434, csr & !0x1000_0000);
}

pub const OS_YIELD_THREAD: u32 = 0x800A1990;
pub const GX_SET_DRAW_DONE: u32 = 0x800B8D10;
const IRQ_PE_FINISH: u32 = 0x13;

/// Interrupts raised and not yet handled (the GPU finishes a `GXSetDrawDone` at once).
static PENDING: Mutex<Vec<u32>> = Mutex::new(Vec::new());

/// Run a lifted function's own body (bypassing its hook).
pub fn call_original(c: &mut Cpu, a: u32) {
    let i = lifted::RANGES.partition_point(|r| r.0 <= a) - 1;
    (lifted::RANGES[i].2)(c, a);
}

fn trace() -> bool { static T: std::sync::OnceLock<bool> = std::sync::OnceLock::new(); *T.get_or_init(|| std::env::var("RUN_TRACE").is_ok()) }

fn raise(c: &mut Cpu, irq: u32) {
    if trace() && irq != 0x18 { eprintln!("run: interrupt {irq:#x}"); }
    // the OS's handler table (__OSInterruptInit: InterruptHandlerTable = 0x80003040)
    let handler = c.m.u32(0x8000_3040 + 4 * irq);
    if handler == 0 { return; }
    let ee = EE.swap(false, std::sync::atomic::Ordering::Relaxed);
    let lr = c.lr;
    c.r[3] = irq;
    c.r[4] = c.m.u32(0x800000D4);
    lifted::dispatch(c, handler);
    c.lr = lr;
    EE.store(ee, std::sync::atomic::Ordering::Relaxed);
}

/// Deliver pending interrupts (one at a time: a handler may raise more).
fn raise_pending(c: &mut Cpu) -> bool {
    let mut any = false;
    loop {
        let next = { let mut p = PENDING.lock().unwrap(); if p.is_empty() { None } else { Some(p.remove(0)) } };
        let Some(i) = next else { return any };
        // the handler's return value registers must not leak into the interrupted code
        let (r3, r4, r5) = (c.r[3], c.r[4], c.r[5]);
        let save: Vec<u32> = c.r.to_vec();
        let (cr, ctr, xer) = (c.cr, c.ctr, c.xer);
        let f = c.f;
        raise(c, i);
        c.r.copy_from_slice(&save);
        let _ = (r3, r4, r5);
        c.cr = cr; c.ctr = ctr; c.xer = xer; c.f = f;
        any = true;
    }
}

/// `GXSetDrawDone` 0x800B8D10: the original, then the draw-done interrupt is pending.
fn gx_set_draw_done(c: &mut Cpu) {
    call_original(c, GX_SET_DRAW_DONE);
    // its token is in the pipe (the write-gather pipe drains on its own): the command processor takes it now
    gx_feed(&mut c.m);
}

pub const DVD_READ: u32 = 0x800A42C4; // the DI driver's `Read` (every read command goes through it)
pub const DVD_LOW_SEEK: u32 = 0x800A46EC;
const IRQ_DI: u32 = 0x15;

/// The disc image the DI reads from (offset, buffer).
pub type Disc = Box<dyn Fn(u64, &mut [u8]) + Send + Sync>;
static DISC: Mutex<Option<Disc>> = Mutex::new(None);

/// The DI (0xCC006000) executes the command the driver just started (DICR TSTART): read 0xA8 copies
/// DILENGTH bytes from disc offset CMDBUF1 << 2 to DIMAR, seek 0xAB is instant; then DILENGTH = 0, TSTART
/// clears, DISR TCINT (0x10) is set and the DI interrupt is pending.
/// SI command transfers (COMCSR 0xCC006434 TSTART): the command bytes are in the SI buffer 0xCC006480
/// (OUTLNGTH bits 22..16), the reply goes back there (INLNGTH bits 14..8). Port 1 is a standard controller
/// (0x00/0xFF type: 09 00 00; 0x41/0x42 origin: buttons 00 80, sticks 80 80 80 80, triggers 00 00, 00 00);
/// the other ports answer nothing (SISR NOREP, COMERR). Then TSTART clears, TCINT sets and the SI interrupt
/// is pending if TCINTMSK.
fn si_transfer(m: &mut crate::mem::Mem) {
    let csr = m.u32(0xCC00_6434);
    let ch = (csr >> 1) & 3;
    let out = ((csr >> 16) & 0x7F).max(1) as usize;
    let inl = ((csr >> 8) & 0x7F).max(1) as usize;
    let cmd = m.u8(0xCC00_6480);
    let _ = out;
    let reply: Option<Vec<u8>> = if ch == 0 {
        match cmd {
            0x00 | 0xFF => Some(vec![0x09, 0x00, 0x00]),
            0x41 | 0x42 => Some(vec![0x00, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00]),
            _ => Some(vec![0; inl]),
        }
    } else { None };
    // a new transfer: COMERR and this channel's error bits start clear
    let mut c2 = csr & !1 & !0x2000_0000;
    let sr = m.u32(0xCC00_6438);
    m.set_u32(0xCC00_6438, sr & !(0x0F << (8 * (3 - ch))));
    match reply {
        Some(r) => {
            let mut b = vec![0u8; inl];
            for (k, x) in r.iter().enumerate().take(inl) { b[k] = *x; }
            m.write(0xCC00_6480, &b);
        }
        None => {
            c2 |= 0x2000_0000; // COMERR
            let sr = m.u32(0xCC00_6438);
            m.set_u32(0xCC00_6438, sr | (0x08 << (8 * (3 - ch)))); // NOREPn
        }
    }
    c2 |= 0x8000_0000; // TCINT
    m.set_u32(0xCC00_6434, c2);
    if trace() { eprintln!("run: SI ch{ch} cmd {cmd:#04x} -> {:#010x}", m.u32(0xCC00_6480)); }
    if csr & 0x4000_0000 != 0 { PENDING.lock().unwrap().push(0x14); }
}

/// The DI (0xCC006000): writing DICR with TSTART runs the command in DICMDBUF0-2 (DIMAR / DILENGTH for DMA),
/// at once: read 0xA8 (subcommand 0x00 data, 0x40 disk ID), seek 0xAB, inquiry 0x12, request error 0xE0
/// (DIIMMBUF 0 = no error), stop motor 0xE3, audio buffer config 0xE4; then DILENGTH = 0, TSTART clears,
/// DISR TCINT (0x10) is set and the DI interrupt is pending if TCINTMASK (0x08).
fn di_execute(m: &mut crate::mem::Mem) {
    let cmd = m.u32(0xCC00_6008);
    let arg = m.u32(0xCC00_600C);
    let len = m.u32(0xCC00_6018);
    let dst = m.u32(0xCC00_6014) & 0x03FF_FFE0 | 0x8000_0000;
    if trace() { eprintln!("run: DI {cmd:#010x} {arg:#x} len {len:#x} -> {dst:#x}"); }
    match cmd >> 24 {
        0xA8 => {
            let off = if cmd & 0xC0 == 0x40 { 0 } else { (arg as u64) << 2 };
            let mut buf = vec![0u8; len as usize];
            if let Some(d) = DISC.lock().unwrap().as_ref() { d(off, &mut buf); }
            m.write(dst, &buf);
        }
        0x12 => {
            // drive info: revision 0x0002, device code 0x0604 (retail), release date 0x20020402
            let mut b = vec![0u8; len as usize];
            let info = [0x00, 0x00, 0x00, 0x02, 0x20, 0x02, 0x04, 0x02, 0x61, 0x00, 0x00, 0x00];
            let n = info.len().min(b.len());
            b[..n].copy_from_slice(&info[..n]);
            m.write(dst, &b);
        }
        0xE0 => m.set_u32(0xCC00_6020, 0),
        0xAB | 0xE3 | 0xE4 | 0xE1 | 0xE2 => {}
        x => eprintln!("run: DI command {x:#04x} ({cmd:#010x}) not modelled"),
    }
    m.set_u32(0xCC00_6018, 0);
    let cr = m.u32(0xCC00_601C);
    m.set_u32(0xCC00_601C, cr & !1);
    let sr = m.u32(0xCC00_6000);
    m.set_u32(0xCC00_6000, sr | 0x10);
    if sr & 0x08 != 0 { PENDING.lock().unwrap().push(IRQ_DI); }
}

pub const AR_START_DMA: u32 = 0x800AC990;
pub const OS_DISABLE_INTERRUPTS: u32 = 0x8009DF3C;
pub const OS_ENABLE_INTERRUPTS: u32 = 0x8009DF50;
pub const OS_RESTORE_INTERRUPTS: u32 = 0x8009DF64;
const IRQ_DSP_ARAM: u32 = 0x06;

/// ARAM (16 MB) and which bytes were written here (the snapshot has no ARAM; reads of bytes the game stored
/// before it are reported).
struct Aram { bytes: Vec<u8>, known: Vec<bool>, warned: bool }
static ARAM: Mutex<Option<Aram>> = Mutex::new(None);

/// `ARStartDMA(type, mainmem, aram, length)` 0x800AC990: the original programs the AR registers; the transfer
/// happens at once (type 0: main -> ARAM, 1: ARAM -> main) and the ARAM interrupt is pending.
fn ar_start_dma(c: &mut Cpu) {
    let (ty, mm, ar, len) = (c.r[3] & 0xFFFF, c.r[4] | 0x8000_0000, c.r[5] as usize, c.r[6] as usize);
    call_original(c, AR_START_DMA);
    if trace() { eprintln!("run: AR DMA type {ty} main {mm:#x} aram {ar:#x} len {len:#x}"); }
    let mut g = ARAM.lock().unwrap();
    let a = g.get_or_insert_with(|| Aram { bytes: vec![0; 16 << 20], known: vec![false; 16 << 20], warned: false });
    let n = len.min(a.bytes.len().saturating_sub(ar));
    if ty == 0 {
        a.bytes[ar..ar + n].copy_from_slice(c.m.read(mm, n as u32));
        a.known[ar..ar + n].iter_mut().for_each(|k| *k = true);
    } else {
        if !a.warned && a.known[ar..ar + n].iter().any(|k| !k) {
            eprintln!("run: ARAM read {ar:#x}+{n:#x} includes data stored before the snapshot (not captured)");
            a.warned = true;
        }
        c.m.write(mm, &a.bytes[ar..ar + n]);
    }
    PENDING.lock().unwrap().push(IRQ_DSP_ARAM);
}

/// MSR.EE, which the lifter does not model (mfmsr reads 0): kept here so interrupts are delivered exactly when
/// the game enables them again.
static EE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

fn set_ee(c: &mut Cpu, on: bool) {
    let was = EE.swap(on, std::sync::atomic::Ordering::Relaxed);
    if on && !was { interrupt_point(c); }
}

/// OS alarms (OSAlarm.c): the decrementer exception runs `DecrementerExceptionCallback` 0x8009A6AC, which
/// fires the due alarms at the head of AlarmQueue (0x8047A6E0, fire time u64 at +8), reprograms the decrementer
/// and returns through OSLoadContext(interrupted context). Here the decrementer is "due" whenever the queue's
/// head is due (checked at interrupt points); the callback gets a scratch context (ALARM_CTX) whose load returns.
const ALARM_QUEUE: u32 = 0x8047_A6E0;
pub const ALARM_CTX: u32 = 0x8180_0000;
struct AlarmReturn;

/// Nothing to run: time passes to the next event, an alarm or a video field, whichever comes first.
fn idle_advance(c: &mut Cpu) {
    use std::sync::atomic::Ordering::Relaxed;
    let next_field = FIELD_TB.load(Relaxed) + TB_PER_FIELD;
    let ai = AI_NEXT.load(Relaxed);
    if ai != 0 && ai < next_field {
        let head = c.m.u32(ALARM_QUEUE);
        let alarm_first = head != 0 && {
            let fire = (c.m.u32(head + 8) as u64) << 32 | c.m.u32(head + 12) as u64;
            let off = (c.m.u32(0x8000_30D8) as u64) << 32 | c.m.u32(0x8000_30DC) as u64;
            (fire.wrapping_sub(off) as i64) < ai as i64
        };
        if !alarm_first {
            if ai > time_base(c) { TB_BASE.store(ai, Relaxed); TB_MARK.store(c.steps, Relaxed); }
            ai_due(c);
            // the DSP's answer (its frame for the next block) comes back before the next block, as on hardware
            raise_pending(c);
            return;
        }
    }
    let head = c.m.u32(ALARM_QUEUE);
    if head != 0 {
        let fire = (c.m.u32(head + 8) as u64) << 32 | c.m.u32(head + 12) as u64;
        let off = (c.m.u32(0x8000_30D8) as u64) << 32 | c.m.u32(0x8000_30DC) as u64;
        let at = fire.wrapping_sub(off);
        if (at as i64) < next_field as i64 {
            if (at as i64) > time_base(c) as i64 { TB_BASE.store(at, Relaxed); TB_MARK.store(c.steps, Relaxed); }
            alarms(c);
            return;
        }
    }
    field(c);
}

fn alarms(c: &mut Cpu) {
    let head = c.m.u32(ALARM_QUEUE);
    if head == 0 { return; }
    let fire = (c.m.u32(head + 8) as u64) << 32 | c.m.u32(head + 12) as u64;
    let sys = time_base(c).wrapping_add((c.m.u32(0x8000_30D8) as u64) << 32 | c.m.u32(0x8000_30DC) as u64);
    if (fire as i64) > (sys as i64) { return; }
    let save = (c.r, c.cr, c.lr, c.ctr, c.xer, c.f, c.ps1, c.fpscr, c.gqr);
    let ee = EE.swap(false, std::sync::atomic::Ordering::Relaxed);
    c.r[3] = 8; // __OS_EXCEPTION_DECREMENTER
    c.r[4] = ALARM_CTX;
    c.lr = 0;
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| lifted::dispatch(c, 0x8009_A6AC)));
    if let Err(e) = r { if !e.is::<AlarmReturn>() { std::panic::resume_unwind(e); } }
    EE.store(ee, std::sync::atomic::Ordering::Relaxed);
    (c.r, c.cr, c.lr, c.ctr, c.xer, c.f, c.ps1, c.fpscr, c.gqr) = save;
}

/// The background thread's idle passes skip to the next event (on unless RUN_EXACT_BG is set).
static FAST_BG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

static IN_POINT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
const RUN_QUEUE_HINT: u32 = 0x8047A764;

/// Interrupts are enabled again: video fields whose time has come fire, pending interrupts are taken, and, as
/// the OS's external interrupt handler does after a handler (OSEnableScheduler; __OSReschedule), a thread an
/// interrupt readied may preempt the running one.
fn interrupt_point(c: &mut Cpu) {
    use std::sync::atomic::Ordering::Relaxed;
    static SKIP: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    if IN_POINT.swap(true, Relaxed) { if trace() && SKIP.fetch_add(1, Relaxed) % 1000000 == 0 { eprintln!("run: interrupt point busy ({:?})", std::thread::current().name()); } return; }
    static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    if std::env::var("RUN_SAMPLE").is_ok() && N.load(Relaxed) % 20000 == 0 { eprintln!("run: sample last_fn {:#x} lr {:#x}", c.last_fn, c.lr); }
    if trace() && N.fetch_add(1, Relaxed) % 100000 == 0 { eprintln!("run: interrupt point (tb {}, field {})", time_base(c), FIELD_TB.load(Relaxed)); }
    let save = (c.r, c.cr, c.lr, c.ctr, c.xer, c.f, c.ps1, c.fpscr, c.gqr);
    while time_base(c) >= FIELD_TB.load(Relaxed) + TB_PER_FIELD { field(c); }
    ai_due(c);
    raise_pending(c);
    alarms(c);
    let cur = c.m.u32(0x8000_00E4);
    let in_thread = cur != 0 && c.m.u32(0x8000_00D4) == cur && c.m.u32(RUN_QUEUE_HINT) != 0;
    IN_POINT.store(false, Relaxed);
    if in_thread && std::env::var("RUN_CHECKQ").is_ok() {
        let bits = c.m.u32(RUN_QUEUE_BITS);
        let heads: Vec<(u32, u32, u32)> = (0..32u32).filter(|p| bits >> (31 - p) & 1 != 0).map(|p| (p, c.m.u32(0x803F_B898 + 8 * p), c.m.u32(0x803F_B89C + 8 * p))).collect();
        eprintln!("run: resched check cur {cur:#x} prio {} state {} bits {bits:#x} queues {heads:x?}", c.m.u32(cur + 0x2d0), c.m.u16(cur + 0x2c8));
        for p in 0..32u32 {
            if bits >> (31 - p) & 1 != 0 && c.m.u32(0x803F_B898 + 8 * p) == 0 {
                eprintln!("run: run queue {p} marked but empty (bits {bits:#x}) on {:?} cur {cur:#x}", std::thread::current().name());
            }
        }
    }
    if in_thread {
        // as in the OS's interrupt handler: rescheduling runs with interrupts off
        let ee = EE.swap(false, Relaxed);
        c.lr = 0;
        lifted::dispatch(c, 0x800A_1960); // __OSReschedule
        EE.store(ee, Relaxed);
    }
    (c.r, c.cr, c.lr, c.ctr, c.xer, c.f, c.ps1, c.fpscr, c.gqr) = save;
}

/// Blocks between interrupt checks inside running code (the hardware takes an interrupt between any two
/// instructions; here at a block boundary every QUANTUM blocks, when MSR.EE is on).
const QUANTUM: u64 = 20_000;

fn tick(c: &mut Cpu) {
    c.step_limit = c.steps + QUANTUM;
    if std::env::var("RUN_SAMPLE").is_ok() && (c.steps / QUANTUM) % 500 == 0 { eprintln!("run: tick {:?} last_fn {:#x} lr {:#x} ee {}", std::thread::current().name(), c.last_fn, c.lr, EE.load(std::sync::atomic::Ordering::Relaxed)); }
    if EE.load(std::sync::atomic::Ordering::Relaxed) { interrupt_point(c); }
}
/// `OSDisableInterrupts`: returns the previous MSR (EE = 0x8000).
fn os_disable_interrupts(c: &mut Cpu) {
    let was = EE.swap(false, std::sync::atomic::Ordering::Relaxed);
    c.r[3] = if was { 0x8000 } else { 0 };
}
fn os_enable_interrupts(c: &mut Cpu) {
    let was = EE.load(std::sync::atomic::Ordering::Relaxed);
    set_ee(c, true);
    // SelectThread's idle loop (no thread ready: interrupts on, spin on RunQueueBits 0x8047A760): time passes
    // to the next video field until an interrupt readies a thread
    if (0x800A_1738..0x800A_1960).contains(&c.lr) {
        let save = (c.r, c.cr, c.ctr, c.xer, c.lr);
        while c.m.u32(RUN_QUEUE_BITS) == 0 {
            if !raise_pending(c) && c.m.u32(RUN_QUEUE_BITS) == 0 { idle_advance(c); }
        }
        (c.r, c.cr, c.ctr, c.xer, c.lr) = save;
    }
    c.r[3] = if was { 0x8000 } else { 0 };
}

// ---------------------------------------------------------------- Dolphin OS threads
//
// OS threads switch with `OSSaveContext` 0x8009BBD0 (setjmp-like: stores r13-r31, r1, r2, GQR1-7, CR, LR (also
// as SRR0), MSR, CTR, XER and r3 slot = 1; returns 0) and `OSLoadContext` 0x8009BC50 (restores them, r3/r4 from
// the context, and rfi to SRR0). Each OS thread runs on a host thread; the host thread leaving an OS thread parks
// under that thread's context until something loads it again. A thread that resumes continues after the
// `OSLoadContext` call that parked it (in `SelectThread`, whose callers ignore its result), with the
// registers of its saved context. FPRs are kept per host thread (the OS switches them lazily through the FP
// unavailable exception, which the port does not have).

pub const OS_SAVE_CONTEXT: u32 = 0x8009BBD0;
pub const OS_LOAD_CONTEXT: u32 = 0x8009BC50;
const RUN_QUEUE_BITS: u32 = 0x8047A760;

/// Host threads parked per OS context, with the SRR0 they will resume at (a context re-made by OSCreateThread
/// for a new thread has another SRR0: that parked host thread is dead).
static OS_PARKED: Mutex<Option<HashMap<u32, (Sender<Wake>, u32)>>> = Mutex::new(None);
thread_local! {
    /// This host thread's OS context (the last one it saved) and its wake channel.
    static MY_CTX: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    static MY_WAKE: std::cell::RefCell<Option<(Sender<Wake>, Receiver<Wake>)>> = const { std::cell::RefCell::new(None) };
}

fn os_save_context(c: &mut Cpu) {
    let ctx = c.r[3];
    let b = ctx;
    for n in 13..32 { c.m.set_u32(b + 0x34 + 4 * (n as u32 - 13), c.r[n]); }
    for g in 1..8 { c.m.set_u32(b + 0x1A4 + 4 * g as u32, c.gqr[g]); }
    c.m.set_u32(b + 0x80, c.cr);
    c.m.set_u32(b + 0x84, c.lr);
    c.m.set_u32(b + 0x198, c.lr);
    c.m.set_u32(b + 0x19C, if EE.load(std::sync::atomic::Ordering::Relaxed) { 0x9032 } else { 0x1032 });
    c.m.set_u32(b + 0x88, c.ctr);
    c.m.set_u32(b + 0x8C, c.xer);
    c.m.set_u32(b + 0x4, c.r[1]);
    c.m.set_u32(b + 0x8, c.r[2]);
    c.r[0] = 1;
    c.m.set_u32(b + 0xC, 1);
    c.r[3] = 0;
    MY_CTX.with(|m| m.set(ctx));
}

/// The register effects of `OSLoadContext(ctx)` (everything but the jump).
fn load_regs(c: &mut Cpu, ctx: u32) {
    c.r[0] = c.m.u32(ctx);
    c.r[1] = c.m.u32(ctx + 4);
    c.r[2] = c.m.u32(ctx + 8);
    let state = c.m.u16(ctx + 0x1A2) as u32;
    let first = if state & 2 != 0 { c.m.set_u16(ctx + 0x1A2, state & !2); 5 } else { 13 };
    for n in first..32 { c.r[n] = c.m.u32(ctx + 4 * n as u32); }
    for g in 1..8 { c.gqr[g] = c.m.u32(ctx + 0x1A4 + 4 * g as u32); }
    c.cr = c.m.u32(ctx + 0x80);
    c.lr = c.m.u32(ctx + 0x84);
    c.ctr = c.m.u32(ctx + 0x88);
    c.xer = c.m.u32(ctx + 0x8C);
    c.r[4] = c.m.u32(ctx + 0x10);
    c.r[3] = c.m.u32(ctx + 0xC);
}

fn my_wake() -> (Sender<Wake>, ()) {
    MY_WAKE.with(|w| {
        let mut w = w.borrow_mut();
        if w.is_none() { *w = Some(channel()); }
        (w.as_ref().unwrap().0.clone(), ())
    })
}

fn os_load_context(c: &mut Cpu) {
    let ctx = c.r[3];
    // the decrementer callback resuming the "interrupted" context run.rs gave it: back to `alarms`
    if ctx == ALARM_CTX { std::panic::panic_any(AlarmReturn); }
    let srr0 = c.m.u32(ctx + 0x198);
    let mine = MY_CTX.with(|m| m.get());
    let (tx, ()) = my_wake();
    let target = {
        let mut g = OS_PARKED.lock().unwrap();
        let map = g.get_or_insert_with(HashMap::new);
        if mine != 0 && mine != ctx { let pc = c.m.u32(mine + 0x198); map.insert(mine, (tx, pc)); }
        if mine == ctx { None } else {
            Some(match map.remove(&ctx) {
                Some((t, pc)) if pc == srr0 => Some(t),
                Some((t, _)) => { t.send(Wake::Die).ok(); None }
                None => None,
            })
        }
    };
    let Some(target) = target else {
        // loading our own context: a plain longjmp back into this thread
        load_regs(c, ctx);
        return;
    };
    let regs = (c.r, c.cr, c.lr, c.ctr, c.xer, c.gqr, c.f, c.ps1, c.fpscr);
    if trace() { eprintln!("run: OS switch {mine:#x} -> {ctx:#x} (srr0 {srr0:#x}, {})", if target.is_some() { "parked" } else { "new" }); }
    match target {
        Some(t) => { t.send(Wake::Resume(None)).ok(); }
        None => {
            // a thread that never ran: a new host thread starts at SRR0 with the context's registers
            let cp = CpuPtr(c as *mut Cpu);
            std::thread::Builder::new().name(format!("os-thread {ctx:#010x}")).stack_size(CO_STACK).spawn(move || {
                let cp = cp;
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // SAFETY: the host thread that spawned this one parks right after (one runs at a time)
                    let c = unsafe { &mut *cp.0 };
                    MY_CTX.with(|m| m.set(ctx));
                    load_regs(c, ctx);
                    // rfi: MSR from SRR1 (EE)
                    EE.store(c.m.u32(ctx + 0x19C) & 0x8000 != 0, std::sync::atomic::Ordering::Relaxed);
                    let mut pc = srr0;
                    loop { lifted::resume(c, pc); pc = c.lr; }
                }));
                if let Err(e) = r {
                    if e.is::<Die>() { return; }
                    let m = e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default();
                    eprintln!("OS thread {ctx:#010x}: {m}");
                    if let Some(t) = MAIN_TX.lock().unwrap().as_ref() { t.send(MainMsg::Panic(format!("OS thread {ctx:#010x}: {m}"))).ok(); }
                }
            }).expect("run: os thread");
        }
    }
    // park until our context is loaded again
    if trace() { eprintln!("run: {:?} parks as {mine:#x}", std::thread::current().name()); }
    let w = MY_WAKE.with(|w| w.borrow().as_ref().unwrap().1.recv());
    if trace() { eprintln!("run: {:?} resumes as {mine:#x}", std::thread::current().name()); }
    if !matches!(w, Ok(Wake::Resume(_))) { std::panic::panic_any(Die); }
    // continue right after this OSLoadContext call with the registers it had (the context this thread saved
    // in SelectThread describes the same point; its callers ignore SelectThread's result)
    (c.r, c.cr, c.lr, c.ctr, c.xer, c.gqr, c.f, c.ps1, c.fpscr) = regs;
    // rfi: MSR from the saved context's SRR1
    let me = MY_CTX.with(|m| m.get());
    EE.store(c.m.u32(me + 0x19C) & 0x8000 != 0, std::sync::atomic::Ordering::Relaxed);
}
/// `gappBackgroundCallback` 0x800FEA74's return address from OSRestoreInterrupts at the end of each pass of
/// its endless loop (the lowest-priority thread: it runs the background gapps whenever nothing else can).
const BG_PASS: u32 = 0x800F_EB9C;

/// The background thread finished a pass with nothing pending: on the console it would spin until the next
/// interrupt; time jumps there instead (the pass is redone then), as the idle loop does.
fn bg_fast_forward(c: &mut Cpu) {
    use std::sync::atomic::Ordering::Relaxed;
    if !PENDING.lock().unwrap().is_empty() { return; }
    let mut next = FIELD_TB.load(Relaxed) + TB_PER_FIELD;
    let head = c.m.u32(ALARM_QUEUE);
    if head != 0 {
        let fire = (c.m.u32(head + 8) as u64) << 32 | c.m.u32(head + 12) as u64;
        let off = (c.m.u32(0x8000_30D8) as u64) << 32 | c.m.u32(0x8000_30DC) as u64;
        let at = fire.wrapping_sub(off);
        if (at as i64) < next as i64 { next = at; }
    }
    let ai = AI_NEXT.load(Relaxed);
    if ai != 0 && ai < next { next = ai; }
    if (next as i64) > time_base(c) as i64 { TB_BASE.store(next, Relaxed); TB_MARK.store(c.steps, Relaxed); }
}

fn os_restore_interrupts(c: &mut Cpu) {
    let was = EE.load(std::sync::atomic::Ordering::Relaxed);
    let on = c.r[3] & 0x8000 != 0;
    if on && c.lr == BG_PASS && FAST_BG.load(std::sync::atomic::Ordering::Relaxed) { bg_fast_forward(c); }
    set_ee(c, on);
    c.r[3] = if was { 0x8000 } else { 0 };
}

/// Hardware register writes (mem.rs `mmio_write`): the devices whose registers have side effects.
static CARD_A: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static MMIO_BUSY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static DSPCR: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
const DSPCR_W1C: u32 = 0xA8; // AIDINT 0x08, ARINT 0x20, DSPINT 0x80: written 1 = acknowledge

fn covers(a: u32, n: usize, r: u32) -> bool { a <= r && r < a + n as u32 }

fn mmio(m: &mut crate::mem::Mem, a: u32, n: usize) {
    if MMIO_BUSY.swap(true, std::sync::atomic::Ordering::Relaxed) { return; }
    // DSP control: interrupt status bits are write-1-to-clear
    if covers(a, n, 0xCC00_500A) || covers(a, n, 0xCC00_500B) {
        let w = m.u16(0xCC00_500A) as u32;
        let old = DSPCR.load(std::sync::atomic::Ordering::Relaxed);
        let v = (w & !DSPCR_W1C) | (old & DSPCR_W1C & !w);
        m.set_u16(0xCC00_500A, v);
        DSPCR.store(v, std::sync::atomic::Ordering::Relaxed);
        // DSPINIT (bit 11, DSPInit writes it 1 then 0): the DSP restarts in its ROM, which greets with a mail
        if w & 0x800 != 0 && old & 0x800 == 0 {
            let r = AX.lock().unwrap().get_or_insert_with(crate::dsp_ax::Ax::new).reset();
            m.set_u16(0xCC00_5004, r >> 16);
            m.set_u16(0xCC00_5006, r & 0xFFFF);
        }
    }
    // ARAM DMA: writing the count's low half starts it (dir: count bit 31, 1 = ARAM -> main memory)
    if covers(a, n, 0xCC00_502A) || covers(a, n, 0xCC00_502B) {
        let mm = ((m.u16(0xCC00_5020) as u32) << 16 | m.u16(0xCC00_5022) as u32) & 0x03FF_FFE0 | 0x8000_0000;
        let ar = (((m.u16(0xCC00_5024) as u32) << 16 | m.u16(0xCC00_5026) as u32) & 0x03FF_FFE0) as usize;
        let hi = m.u16(0xCC00_5028) as u32;
        let len = ((hi & 0x7FFF) << 16 | m.u16(0xCC00_502A) as u32) as usize & 0x03FF_FFE0;
        let to_main = hi & 0x8000 != 0;
        {
            let mut g = ARAM.lock().unwrap();
            let x = g.get_or_insert_with(|| Aram { bytes: vec![0; 16 << 20], known: vec![false; 16 << 20], warned: false });
            let n = len.min(x.bytes.len().saturating_sub(ar));
            if trace() { eprintln!("run: AR DMA {} main {mm:#x} aram {ar:#x} len {len:#x}", if to_main { "ARAM->main" } else { "main->ARAM" }); }
            if !to_main {
                x.bytes[ar..ar + n].copy_from_slice(m.read(mm, n as u32));
                x.known[ar..ar + n].iter_mut().for_each(|k| *k = true);
            } else {
                if !x.warned && x.known[ar..ar + n].iter().any(|k| !k) {
                    eprintln!("run: ARAM read {ar:#x}+{n:#x} includes data stored before the snapshot (not captured)");
                    x.warned = true;
                }
                let d = x.bytes[ar..ar + n].to_vec();
                m.write(mm, &d);
            }
        }
        // done at once: ARINT set, interrupt if ARINTMASK
        let v = DSPCR.load(std::sync::atomic::Ordering::Relaxed) | 0x20;
        DSPCR.store(v, std::sync::atomic::Ordering::Relaxed);
        m.set_u16(0xCC00_500A, v);
        if v & 0x10 != 0 { PENDING.lock().unwrap().push(IRQ_DSP_ARAM); }
    }
    if covers(a, n, 0xCC00_601F) && m.u32(0xCC00_601C) & 1 != 0 { di_execute(m); }
    // EXI channel 0 CSR: EXT (bit 12) is the hardware's "a device is in slot A", not writable
    if covers(a, n, 0xCC00_6800) || covers(a, n, 0xCC00_6803) {
        if CARD_A.load(std::sync::atomic::Ordering::Relaxed) {
            let v = m.u32(0xCC00_6800);
            if v & 0x1000 == 0 { m.set_u32(0xCC00_6800, v | 0x1000); }
        }
    }
    if covers(a, n, 0xCC00_6434) || covers(a, n, 0xCC00_6437) {
        // COMCSR: TCINT (bit 31) is write-1-to-clear
        let v = m.u32(0xCC00_6434);
        if v & 0x8000_0000 != 0 { m.set_u32(0xCC00_6434, v & !0x8000_0000); }
        if v & 1 != 0 { si_transfer(m); }
    }
    if covers(a, n, 0xCC00_6C03) && m.u32(0xCC00_6C00) & 1 == 0 {
        AI_START.store(u64::MAX, std::sync::atomic::Ordering::Relaxed);
    }
    MMIO_BUSY.store(false, std::sync::atomic::Ordering::Relaxed);
}

/// Emulated time: the time base runs at 40.5 MHz (bus/4). Between video fields it advances with the work
/// done (TB_PER_BLOCK per lifted block, ~ a 486 MHz CPU doing a few instructions per block); a field that
/// waits jumps it to the field's start (675675 ticks per NTSC field).
const TB_HZ: u64 = 40_500_000;
const TB_PER_FIELD: u64 = 675_675;
const TB_PER_BLOCK_X100: u64 = 40;
static TB_BASE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static TB_MARK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static FIELD_TB: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// AI sample counter start (time base when AICR PSTAT went on), u64::MAX = stopped.
static AI_START: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);

pub fn time_base(c: &Cpu) -> u64 {
    use std::sync::atomic::Ordering::Relaxed;
    TB_BASE.load(Relaxed) + c.steps.saturating_sub(TB_MARK.load(Relaxed)) * TB_PER_BLOCK_X100 / 100
}

/// A video field begins: time is at least the field's start.
fn tb_field(c: &Cpu) {
    use std::sync::atomic::Ordering::Relaxed;
    let f = FIELD_TB.load(Relaxed) + TB_PER_FIELD;
    FIELD_TB.store(f, Relaxed);
    if time_base(c) < f { TB_BASE.store(f, Relaxed); TB_MARK.store(c.steps, Relaxed); }
}

/// `OSGetTime` 0x800A2778 (r3:r4) and `OSGetTick` 0x800A2790 (r3): the time base.
fn os_get_time(c: &mut Cpu) { let t = time_base(c); c.r[3] = (t >> 32) as u32; c.r[4] = t as u32; }
fn os_get_tick(c: &mut Cpu) { c.r[3] = time_base(c) as u32; }

/// Live register reads: the AI sample counter (AISCNT 0xCC006C08) counts at the AI rate while playing.
fn mmio_read(c: &Cpu, a: u32, _n: u32) -> Option<u32> {
    use std::sync::atomic::Ordering::Relaxed;
    if (0xCC00_6C08..0xCC00_6C0C).contains(&a) {
        let cr = c.m.u32(0xCC00_6C00);
        if cr & 1 == 0 { return Some(c.m.u32(0xCC00_6C08)); }
        let now = time_base(c);
        let start = match AI_START.load(Relaxed) { u64::MAX => { AI_START.store(now, Relaxed); now } s => s };
        let rate = if cr & 0x40 != 0 { 32000 } else { 48000 };
        return Some(((now - start) * rate / TB_HZ) as u32);
    }
    None
}

pub const PAD_READ: u32 = 0x800AB150;
pub const EXI_IMM: u32 = 0x8009820C;
pub const EXI_DMA: u32 = 0x80098408;
pub const EXI_SELECT: u32 = 0x80098B94;
pub const EXI_DESELECT: u32 = 0x80098CD4;

static EXI: Mutex<Option<crate::exi::Exi>> = Mutex::new(None);

fn exi_with<R>(f: impl FnOnce(&mut crate::exi::Exi) -> R) -> R {
    let mut g = EXI.lock().unwrap();
    f(g.get_or_insert_with(crate::exi::Exi::new))
}

/// `EXIImm` / `EXIDma`: the original starts the transfer, the bus runs it at once (exi.rs).
fn exi_imm(c: &mut Cpu) {
    let ch = c.r[3] as usize;
    call_original(c, EXI_IMM);
    if c.r[3] != 0 && ch < 3 { if let Some(i) = exi_with(|e| e.transfer(c, ch)) { PENDING.lock().unwrap().push(i); } }
}
fn exi_dma(c: &mut Cpu) {
    let ch = c.r[3] as usize;
    call_original(c, EXI_DMA);
    if c.r[3] != 0 && ch < 3 { if let Some(i) = exi_with(|e| e.transfer(c, ch)) { PENDING.lock().unwrap().push(i); } }
}
fn exi_select(c: &mut Cpu) {
    let ch = c.r[3] as usize;
    call_original(c, EXI_SELECT);
    if ch < 3 { exi_with(|e| e.select(c, ch)); }
}
fn exi_deselect(c: &mut Cpu) {
    let ch = c.r[3] as usize;
    call_original(c, EXI_DESELECT);
    if ch < 3 && exi_with(|e| { e.select(c, ch); std::mem::take(&mut e.card_irq) }) {
        // the card's EXI interrupt (EXI_0_EXI, __OSInterrupt 9): CSR EXIINT, delivered if EXIINTMASK
        let csr = c.m.u32(0xCC00_6800);
        c.m.set_u32(0xCC00_6800, csr | 2);
        if csr & 1 != 0 { PENDING.lock().unwrap().push(9); }
    }
}
pub const DSP_SEND_MAIL: u32 = 0x800AE7CC;
pub const DSP_READ_MAIL: u32 = 0x800AE7B4;

static AX: Mutex<Option<crate::dsp_ax::Ax>> = Mutex::new(None);

/// `DSPSendMailToDSP` 0x800AE7CC: the original writes the CPU->DSP mailbox; the DSP (AX, dsp_ax.rs) takes the
/// mail at once (mailbox empty again) and may answer through the DSP->CPU mailbox with the DSP interrupt.
fn dsp_send_mail(c: &mut Cpu) {
    let m = c.r[3];
    call_original(c, DSP_SEND_MAIL);
    let hi = c.m.u16(0xCC00_5000) as u32;
    c.m.set_u16(0xCC00_5000, hi & 0x7FFF);
    let reply = {
        let mut ax = AX.lock().unwrap();
        let ax = ax.get_or_insert_with(crate::dsp_ax::Ax::new);
        let mut g = ARAM.lock().unwrap();
        let a = g.get_or_insert_with(|| Aram { bytes: vec![0; 16 << 20], known: vec![false; 16 << 20], warned: false });
        ax.mail(c, &mut a.bytes, m)
    };
    if let Some(r) = reply {
        c.m.set_u16(0xCC00_5004, r >> 16);
        c.m.set_u16(0xCC00_5006, r & 0xFFFF);
        PENDING.lock().unwrap().push(0x07);
    }
}

/// `DSPReadMailFromDSP` 0x800AE7B4: reading the low half empties the DSP->CPU mailbox.
fn dsp_read_mail(c: &mut Cpu) {
    call_original(c, DSP_READ_MAIL);
    let hi = c.m.u16(0xCC00_5004) as u32;
    c.m.set_u16(0xCC00_5004, hi & 0x7FFF);
}

/// The mixed output since the last call (32 kHz, interleaved R, L), for the host's audio.
pub fn take_audio() -> Vec<i16> { AX.lock().unwrap().as_mut().map(|a| std::mem::take(&mut a.out)).unwrap_or_default() }

/// The host's controller 1 (PADStatus fields): buttons (PAD_BUTTON_*), main stick x/y, C stick x/y, L/R analog.
#[derive(Clone, Copy, Default, Debug)]
pub struct Pad { pub buttons: u16, pub stick: (i8, i8), pub cstick: (i8, i8), pub trig: (u8, u8) }
static PAD: Mutex<Pad> = Mutex::new(Pad { buttons: 0, stick: (0, 0), cstick: (0, 0), trig: (0, 0) });

/// `PADRead(PADStatus status[4])` 0x800AB150: the SI poll results. Port 1 is the host's pad, ports 2-4 have
/// no controller (err PAD_ERR_NO_CONTROLLER = -1). Returns the connected-channel bits (PAD_CHAN0_BIT).
fn pad_read(c: &mut Cpu) {
    let p = *PAD.lock().unwrap();
    let a = c.r[3];
    if trace() && p.buttons != 0 { eprintln!("run: PADRead {:#x} buttons {:#x}", c.lr, p.buttons); }
    let mut s = [0u8; 12];
    s[0..2].copy_from_slice(&p.buttons.to_be_bytes());
    s[2] = p.stick.0 as u8; s[3] = p.stick.1 as u8; s[4] = p.cstick.0 as u8; s[5] = p.cstick.1 as u8;
    s[6] = p.trig.0; s[7] = p.trig.1;
    c.m.write(a, &s);
    for k in 1..4 { let mut e = [0u8; 12]; e[10] = 0xFF; c.m.write(a + 12 * k, &e); }
    c.r[3] = 0x8000_0000;
}

/// `OSYieldThread` 0x800A1990: only the main OS thread exists; pending interrupts get handled.
#[allow(dead_code)]
fn os_yield_thread(c: &mut Cpu) { raise_pending(c); }

/// `OSSleepThread` 0x800A238C: handle pending interrupts; if none, the queue's own interrupt fires.
#[allow(dead_code)]
fn os_sleep_thread(c: &mut Cpu) {
    let q = c.r[3];
    if raise_pending(c) { return; }
    let Some(&(_, irq, _)) = INTERRUPTS.iter().find(|i| i.0 == q) else {
        panic!("OSSleepThread on queue {q:#010x} (lr {:#010x}): no interrupt source modelled", c.lr)
    };
    if irq == 0x18 { field(c) } else { raise(c, irq) }
}

/// Dolphin-OS / hardware waits the host replaces (the viewer paces frames and owns the GPU, DVD and audio):
/// (address, r3 or u32::MAX to leave r3).
pub const HOST_STUBS: &[(u32, u32)] = &[
];

/// The command processor: the pipe's bytes are parsed as the game flushes them (`GXFlush`), so register
/// state and memory references are taken when the GPU would read them; a PE draw-done token (BP 0x45) raises
/// the PE finish interrupt (the GPU is done at once).
struct GpuStream { gx: colo_gx::Gx, frame: colo_gx::Frame, buf: Vec<u8> }
static GPU_IN: Mutex<Option<GpuStream>> = Mutex::new(None);
pub const GX_FLUSH: u32 = 0x800B8920;

fn gx_feed(m: &mut crate::mem::Mem) {
    let data = std::mem::take(m.pipe.as_mut().unwrap());
    let mut g = GPU_IN.lock().unwrap();
    let Some(g) = g.as_mut() else { return };
    g.buf.extend_from_slice(&data);
    let before = g.frame.pe_done;
    let v = GpuView(m);
    let n = g.gx.run(&g.buf, &v, &mut g.frame, 0);
    g.buf.drain(..n);
    if g.frame.pe_done != before { PENDING.lock().unwrap().push(IRQ_PE_FINISH); }
}

fn gx_flush(c: &mut Cpu) {
    call_original(c, GX_FLUSH);
    gx_feed(&mut c.m);
}

pub const VI_SET_NEXT_FB: u32 = 0x800AA198;
/// The XFB the game asked the VI to show next (`VISetNextFrameBuffer`, physical).
static NEXT_XFB: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
fn vi_set_next_fb(c: &mut Cpu) {
    NEXT_XFB.store(c.r[3] & 0x01FF_FFFF, std::sync::atomic::Ordering::Relaxed);
    call_original(c, VI_SET_NEXT_FB);
}

struct GpuView<'a>(&'a mut crate::mem::Mem);
impl colo_gx::GpuMem for GpuView<'_> {
    fn read(&self, a: u32, out: &mut [u8]) {
        let a = (a & 0x01FF_FFFF) | 0x8000_0000;
        let n = out.len() as u32;
        if a.checked_add(n).is_some_and(|e| e <= 0x8180_0000) { out.copy_from_slice(self.0.read(a, n)); } else { out.fill(0); }
    }
}
impl colo_gx::raster::GpuMemMut for GpuView<'_> {
    fn write(&mut self, a: u32, d: &[u8]) {
        let a = (a & 0x01FF_FFFF) | 0x8000_0000;
        if a.checked_add(d.len() as u32).is_some_and(|e| e <= 0x8180_0000) { self.0.write(a, d); }
    }
}

pub const MAIN: u32 = 0x800055E0;

enum MainMsg { Frame, Panic(String) }
static MAIN_TX: Mutex<Option<Sender<MainMsg>>> = Mutex::new(None);
thread_local! {
    /// Set on the thread running the game's `main` (boot mode): its wake channel.
    static MAIN_RX: std::cell::RefCell<Option<Receiver<Wake>>> = const { std::cell::RefCell::new(None) };
}

/// `GSgappUpdate` 0x800FE7A0: in boot mode the game's own main loop calls it once per frame; the frame boundary
/// parks `main` there until the host asks for the next frame.
fn gapp_update(c: &mut Cpu) {
    let on_main = MAIN_RX.with(|r| r.borrow().is_some());
    if on_main {
        if let Some(t) = MAIN_TX.lock().unwrap().as_ref() { t.send(MainMsg::Frame).ok(); }
        let w = MAIN_RX.with(|r| r.borrow().as_ref().unwrap().recv());
        if !matches!(w, Ok(Wake::Resume(_))) { std::panic::panic_any(Die); }
        raise_pending(c);
    }
    call_original(c, GAPP_UPDATE);
}

/// A game image driven a frame at a time.
pub struct Machine {
    pub c: Box<Cpu>,
    pub gpu: colo_gx::raster::Renderer,
    /// The main loop's registers at `GSgappUpdate` entry (r1, r2, r13).
    sp: u32,
    /// Boot mode: the game's `main` runs on its own thread; (wake, frame-done).
    main: Option<(Sender<Wake>, Receiver<MainMsg>)>,
}

impl Machine {
    /// From a snapshot taken at `GSgappUpdate` entry (Working/dolphin/story_entry_snap.py): MEM1 and registers.
    pub fn new(mem1: Vec<u8>, r: &[u32; 32]) -> Machine {
        let mut m = crate::mem::Mem::new();
        m.map(0x8000_0000, mem1);
        // locked L1 cache (GSscratch, 16 KB; the GSgfx state lives there): `set_l1` loads the captured copy
        m.map(0xE000_0000, vec![0; 0x4000]);
        // scratch OS context for the decrementer callback (outside MEM1)
        m.map(ALARM_CTX, vec![0; 0x1000]);
        // hardware registers (CP/PE/VI/PI/MI/DSP/DI/SI/EXI/AI) and the GX write-gather pipe 0xCC008000: plain
        // memory for now (the GX command stream is not drawn; the viewer renders the battle itself)
        let mut hw = vec![0u8; 0x9000];
        // Dolphin's GDB stub reads the registers as 0, so they start at the values the SDK's init leaves:
        // DISR 0x2A (BRK/TC/DE interrupt masks, DVDInit / DVDLowReset)
        hw[0x6000..0x6004].copy_from_slice(&0x2Au32.to_be_bytes());
        // AR_MODE / ARAM ready (0xCC005016 bit 0): the ARAM controller is initialised
        hw[0x5017] = 1;
        m.map(0xCC00_0000, hw);
        m.pipe = Some(Vec::new());
        m.mmio_write = Some(mmio);
        let mut c = Box::new(Cpu::bare(m));
        c.dispatch = lifted::dispatch;
        c.r.copy_from_slice(r);
        c.hooks = true;
        FAST_BG.store(std::env::var("RUN_EXACT_BG").is_err(), std::sync::atomic::Ordering::Relaxed);
        c.tick = Some(tick);
        c.step_limit = QUANTUM;
        c.hook_fn.insert(THREAD_EXECUTE, thread_execute);
        c.hook_fn.insert(THREAD_SWITCH, thread_switch);
        c.hook_fn.insert(OS_SAVE_CONTEXT, os_save_context);
        c.hook_fn.insert(OS_LOAD_CONTEXT, os_load_context);
        c.hook_fn.insert(GX_FLUSH, gx_flush);
        c.hook_fn.insert(GX_SET_DRAW_DONE, gx_set_draw_done);
        c.hook_fn.insert(PAD_READ, pad_read);
        c.hook_fn.insert(DSP_SEND_MAIL, dsp_send_mail);
        c.hook_fn.insert(DSP_READ_MAIL, dsp_read_mail);
        c.hook_fn.insert(VI_SET_NEXT_FB, vi_set_next_fb);
        c.hook_fn.insert(GAPP_UPDATE, gapp_update);
        c.hook_fn.insert(0x800A2778, os_get_time);
        c.hook_fn.insert(0x800A2790, os_get_tick);
        c.mmio_read = Some(mmio_read);
        c.hook_fn.insert(EXI_IMM, exi_imm);
        c.hook_fn.insert(EXI_DMA, exi_dma);
        c.hook_fn.insert(EXI_SELECT, exi_select);
        c.hook_fn.insert(EXI_DESELECT, exi_deselect);
        c.hook_fn.insert(OS_DISABLE_INTERRUPTS, os_disable_interrupts);
        c.hook_fn.insert(OS_ENABLE_INTERRUPTS, os_enable_interrupts);
        c.hook_fn.insert(OS_RESTORE_INTERRUPTS, os_restore_interrupts);
        for &(a, ret) in HOST_STUBS { c.stubs.insert(a, ret); }
        // a killed coroutine unwinds with `Die`: not an error
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |i| if !i.payload().is::<Die>() && !i.payload().is::<AlarmReturn>() { prev(i) }));
        let (tx, rx) = channel();
        *SCHED.lock().unwrap() = Some(Sched { cos: HashMap::new(), back_tx: tx });
        *BACK_RX.lock().unwrap() = Some(rx);
        let mut gx = colo_gx::Gx::new();
        // the GPU's registers are not in the snapshot: start from libGX's shadow (`gx` pointer at r2 - 0x73C0)
        let gxdata = c.m.u32(0x8048_36A0 - 0x73C0);
        if gxdata != 0 { gx.seed_sdk(&GpuView(&mut c.m), gxdata & 0x01FF_FFFF); }
        *GPU_IN.lock().unwrap() = Some(GpuStream { gx, frame: colo_gx::Frame::default(), buf: Vec::new() });
        Machine { sp: r[1], c, gpu: colo_gx::raster::Renderer::new(), main: None }
    }

    /// Boot from a `Working/dolphin/story_entry_snap.py` directory: before.bin (MEM1), regs_before.json, l1.bin,
    /// aram.bin.
    pub fn from_snapshot(dir: &std::path::Path) -> std::io::Result<Machine> {
        let mem = std::fs::read(dir.join("before.bin"))?;
        let s = std::fs::read_to_string(dir.join("regs_before.json"))?;
        let bad = || std::io::Error::new(std::io::ErrorKind::InvalidData, "regs_before.json");
        let i = s.find("\"r\": [").ok_or_else(bad)? + 6;
        let j = s[i..].find(']').ok_or_else(bad)? + i;
        let mut r = [0u32; 32];
        for (k, x) in s[i..j].split(',').enumerate().take(32) { r[k] = x.trim().parse().map_err(|_| bad())?; }
        let mut m = Machine::new(mem, &r);
        if let Ok(a) = std::fs::read(dir.join("aram.bin")) { m.set_aram(a); }
        // a mid-game snapshot: the DSP interrupt masks are on (AIDINTMASK, ARINTMASK, DSPINTMASK); the stub can't read them
        DSPCR.store(0x54, std::sync::atomic::Ordering::Relaxed);
        m.c.m.set_u16(0xCC00_500A, 0x54);
        if let Ok(a) = std::fs::read(dir.join("l1.bin")) { m.set_l1(&a); }
        Ok(m)
    }

    /// The floor stack depth (`0x8047ACC4`): 1 while the battle floor is pushed over the field.
    pub fn floor_depth(&self) -> u32 { self.c.m.u32(0x8047_ACC4) }

    /// `battleGetResult` 0x801EF634 (std 103): 2 = win.
    pub fn battle_result(&mut self) -> u32 {
        let save = (self.c.r, self.c.lr, self.c.ctr, self.c.cr);
        self.c.lr = 0;
        lifted::dispatch(&mut self.c, 0x801E_F634);
        let r = self.c.r[3];
        (self.c.r, self.c.lr, self.c.ctr, self.c.cr) = save;
        r
    }

    /// The locked L1 cache as captured with the snapshot (story_entry_snap.py `l1.bin`).
    pub fn set_l1(&mut self, bytes: &[u8]) { self.c.m.write(0xE000_0000, bytes); }

    /// Controller 1 for the next PADRead.
    pub fn set_pad(&mut self, p: Pad) { *PAD.lock().unwrap() = p; }

    /// ARAM as captured with the snapshot (story_entry_snap.py `aram.bin`).
    pub fn set_aram(&mut self, bytes: Vec<u8>) {
        let n = bytes.len();
        *ARAM.lock().unwrap() = Some(Aram { bytes, known: vec![true; n], warned: false });
    }

    /// The GX command stream written since the last call (the write-gather pipe; display lists are called by
    /// address from it).
    pub fn take_gx(&mut self) -> Vec<u8> { std::mem::take(self.c.m.pipe.as_mut().unwrap()) }

    /// The GX stream written since the last call as a draw list for a graphics-card renderer (colo-gxgpu).
    pub fn gpu_cmds(&mut self, b: &mut colo_gx::cmd::Builder) -> colo_gx::cmd::GFrame {
        gx_feed(&mut self.c.m);
        let f = GPU_IN.lock().unwrap().as_mut().map(|g| std::mem::take(&mut g.frame)).unwrap_or_default();
        b.build(&f, &GpuView(&mut self.c.m))
    }

    /// `gpu_cmds` that also runs the software renderer over the same stream (for comparing the two).
    pub fn gpu_cmds_soft(&mut self, b: &mut colo_gx::cmd::Builder) -> colo_gx::cmd::GFrame {
        gx_feed(&mut self.c.m);
        let f = GPU_IN.lock().unwrap().as_mut().map(|g| std::mem::take(&mut g.frame)).unwrap_or_default();
        let g = b.build(&f, &GpuView(&mut self.c.m));
        let mut f = f;
        // debugging: SOFTONLY=a-b keeps only those draws
        if let Some((a, z)) = std::env::var("SOFTONLY").ok().and_then(|r| { let (a, z) = r.split_once('-')?; Some((a.parse::<usize>().ok()?, z.parse::<usize>().ok()?)) }) {
            for (k, d) in f.draws.iter_mut().enumerate() { if k < a || k >= z { d.verts.clear(); } }
        }
        self.gpu.execute(&f, &mut GpuView(&mut self.c.m));
        g
    }

    /// Run the GPU over the GX stream written since the last call (draws into the EFB, EFB copies into memory).
    pub fn gpu_frame(&mut self) -> colo_gx::Frame {
        gx_feed(&mut self.c.m);
        let f = GPU_IN.lock().unwrap().as_mut().map(|g| std::mem::take(&mut g.frame)).unwrap_or_default();
        let mut v = GpuView(&mut self.c.m);
        self.gpu.execute(&f, &mut v);
        f
    }

    /// The frame the VI shows (the XFB named by the last `VISetNextFrameBuffer`), as RGBA8 640 x `h`.
    pub fn display(&self, h: usize) -> Vec<u8> {
        let a = NEXT_XFB.load(std::sync::atomic::Ordering::Relaxed) | 0x8000_0000;
        let mut out = vec![0u8; 640 * h * 4];
        if a == 0x8000_0000 { return out; }
        for y in 0..h {
            let line = self.c.m.read(a + (y * 1280) as u32, 1280);
            for x in (0..640).step_by(2) {
                let (y0, u, y1, v) = (line[2 * x] as f32, line[2 * x + 1] as f32 - 128.0, line[2 * x + 2] as f32, line[2 * x + 3] as f32 - 128.0);
                for (k, yy) in [(0, y0), (1, y1)] {
                    let yy = 1.164 * (yy - 16.0);
                    let rgb = [yy + 1.596 * v, yy - 0.392 * u - 0.813 * v, yy + 2.017 * u];
                    let o = (y * 640 + x + k) * 4;
                    for c in 0..3 { out[o + c] = rgb[c].clamp(0.0, 255.0) as u8; }
                    out[o + 3] = 255;
                }
            }
        }
        out
    }

    /// Insert a memory card in slot A built from GCI files (the SRAM flash ID and font encoding come from the
    /// game's RAM, so the card is this console's).
    pub fn insert_card(&mut self, gcis: &[Vec<u8>]) {
        let mut flash = [0u8; 12];
        for (k, b) in flash.iter_mut().enumerate() { *b = self.c.m.u8(0x803F_B854 + k as u32); }
        let enc = self.c.m.u16(0x8047_A970) as u16;
        let img = crate::memcard::build(gcis, flash, 0, enc);
        exi_with(|e| e.card = Some(crate::memcard::Card::new(img)));
        CARD_A.store(true, std::sync::atomic::Ordering::Relaxed);
        let csr = self.c.m.u32(0xCC00_6800);
        self.c.m.set_u32(0xCC00_6800, csr | 0x1000); // EXT: a device is in slot A
    }

    /// Give the DI its disc.
    pub fn set_disc(&mut self, d: Disc) { *DISC.lock().unwrap() = Some(d); }

    /// Boot mode: from a snapshot at `main` (Working/dolphin/boot_snap.py), run the game's `main` on its own thread
    /// up to its first `GSgappUpdate` (the game's init: GXInit, DSP, sound, threads...).
    pub fn boot(&mut self) {
        // the DSP sits in its ROM, its greeting mail unread (OSInit's DSP init ran before the snapshot; the
        // mailbox registers aren't in it)
        let r = AX.lock().unwrap().get_or_insert_with(crate::dsp_ax::Ax::new).reset();
        self.c.m.set_u16(0xCC00_5004, r >> 16);
        self.c.m.set_u16(0xCC00_5006, r & 0xFFFF);
        // a DVD read the OS started before the snapshot (OSInit's boot-sequence read) is still in flight in the
        // hardware, which the snapshot doesn't have: finish it now (DVD `executing` 0x8047A7E8, state 1 = busy)
        let e = self.c.m.u32(0x8047_A7E8);
        if e != 0 && self.c.m.u32(e + 0xC) == 1 && matches!(self.c.m.u32(e + 8), 1 | 4 | 5 | 0xE) {
            let (off, len, dst) = (self.c.m.u32(e + 0x10) as u64, self.c.m.u32(e + 0x14), self.c.m.u32(e + 0x18));
            let mut b = vec![0u8; len as usize];
            if let Some(d) = DISC.lock().unwrap().as_ref() { d(off, &mut b); }
            self.c.m.write(dst | 0x8000_0000, &b);
            self.c.m.set_u32(0xCC00_6018, 0);
            let sr = self.c.m.u32(0xCC00_6000);
            self.c.m.set_u32(0xCC00_6000, sr | 0x10 | 0x2A);
            PENDING.lock().unwrap().push(IRQ_DI);
        }
        let (wtx, wrx) = channel::<Wake>();
        let (ftx, frx) = channel::<MainMsg>();
        *MAIN_TX.lock().unwrap() = Some(ftx.clone());
        let cp = CpuPtr(&mut *self.c as *mut Cpu);
        std::thread::Builder::new().name("game-main".into()).stack_size(CO_STACK).spawn(move || {
            let cp = cp;
            MAIN_RX.with(|r| *r.borrow_mut() = Some(wrx));
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // SAFETY: the host waits for MainMsg while this thread runs (one side at a time)
                let c = unsafe { &mut *cp.0 };
                c.lr = 0;
                lifted::dispatch(c, MAIN);
            }));
            let m = match r {
                Err(e) if e.is::<Die>() => return,
                Err(e) => e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default(),
                Ok(()) => "main returned".into(),
            };
            ftx.send(MainMsg::Panic(m)).ok();
        }).expect("run: main thread");
        self.main = Some((wtx, frx));
        self.wait_main();
    }

    fn wait_main(&mut self) {
        let msg = self.main.as_ref().unwrap().1.recv().expect("run: main lost");
        if let MainMsg::Panic(m) = msg { panic!("game main: {m}"); }
    }

    /// One game frame: `GSgappUpdate` (mainInit's loop body).
    pub fn frame(&mut self) {
        if let Some((w, _)) = self.main.as_ref() {
            w.send(Wake::Resume(None)).ok();
            self.wait_main();
            return;
        }
        // interrupts raised since the last frame are delivered at the frame boundary at the latest
        raise_pending(&mut self.c);
        self.c.r[1] = self.sp;
        self.c.lr = 0;
        lifted::dispatch(&mut self.c, GAPP_UPDATE);
    }
}

impl Drop for Machine {
    fn drop(&mut self) {
        if let Some((w, _)) = self.main.take() { w.send(Wake::Die).ok(); }
        if let Some(s) = SCHED.lock().unwrap().take() {
            for (_, co) in s.cos { co.tx.send(Wake::Die).ok(); }
        }
    }
}

/// The GS threads that have run here: (context, resume pc), for diagnostics.
pub fn threads() -> Vec<(u32, u32)> {
    let mut v: Vec<(u32, u32)> = SCHED.lock().unwrap().as_ref().map(|s| s.cos.iter().map(|(k, c)| (*k, c.resume_pc)).collect()).unwrap_or_default();
    v.sort();
    v
}
