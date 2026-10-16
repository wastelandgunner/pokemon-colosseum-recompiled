//! Dolphin OS (OSInterrupt, OSAlarm, OSContext / OSThread, OSTime): MSR.EE and interrupt delivery points, the
//! decrementer's alarms, idle time skips, OS thread switches on host threads, and the time base.

use super::*;

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

pub const OS_YIELD_THREAD: u32 = 0x800A1990;

pub const OS_DISABLE_INTERRUPTS: u32 = 0x8009DF3C;
pub const OS_ENABLE_INTERRUPTS: u32 = 0x8009DF50;
pub const OS_RESTORE_INTERRUPTS: u32 = 0x8009DF64;

/// MSR.EE, which the lifter does not model (mfmsr reads 0): kept here so interrupts are delivered exactly when
/// the game enables them again.
pub(super) static EE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

pub(super) fn set_ee(c: &mut Cpu, on: bool) {
    let was = EE.swap(on, std::sync::atomic::Ordering::Relaxed);
    if on && !was { interrupt_point(c); }
}

/// OS alarms (OSAlarm.c): the decrementer exception runs `DecrementerExceptionCallback` 0x8009A6AC, which
/// fires the due alarms at the head of AlarmQueue (0x8047A6E0, fire time u64 at +8), reprograms the decrementer
/// and returns through OSLoadContext(interrupted context). Here the decrementer is "due" whenever the queue's
/// head is due (checked at interrupt points); the callback gets a scratch context (ALARM_CTX) whose load returns.
pub(super) const ALARM_QUEUE: u32 = 0x8047_A6E0;
pub const ALARM_CTX: u32 = 0x8180_0000;
pub(super) struct AlarmReturn;

/// Nothing to run: time passes to the next event, an alarm or a video field, whichever comes first.
pub(super) fn idle_advance(c: &mut Cpu) {
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

pub(super) fn alarms(c: &mut Cpu) {
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
pub(super) static FAST_BG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

pub(super) static IN_POINT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub(super) const RUN_QUEUE_HINT: u32 = 0x8047A764;

/// Interrupts are enabled again: video fields whose time has come fire, pending interrupts are taken, and, as
/// the OS's external interrupt handler does after a handler (OSEnableScheduler; __OSReschedule), a thread an
/// interrupt readied may preempt the running one.
pub(super) fn interrupt_point(c: &mut Cpu) {
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
pub(super) const QUANTUM: u64 = 20_000;

pub(super) fn tick(c: &mut Cpu) {
    c.step_limit = c.steps + QUANTUM;
    if std::env::var("RUN_SAMPLE").is_ok() && (c.steps / QUANTUM) % 500 == 0 { eprintln!("run: tick {:?} last_fn {:#x} lr {:#x} ee {}", std::thread::current().name(), c.last_fn, c.lr, EE.load(std::sync::atomic::Ordering::Relaxed)); }
    if EE.load(std::sync::atomic::Ordering::Relaxed) { interrupt_point(c); }
}
/// `OSDisableInterrupts`: returns the previous MSR (EE = 0x8000).
pub(super) fn os_disable_interrupts(c: &mut Cpu) {
    let was = EE.swap(false, std::sync::atomic::Ordering::Relaxed);
    c.r[3] = if was { 0x8000 } else { 0 };
}
pub(super) fn os_enable_interrupts(c: &mut Cpu) {
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
pub(super) const RUN_QUEUE_BITS: u32 = 0x8047A760;

/// Host threads parked per OS context, with the SRR0 they will resume at (a context re-made by OSCreateThread
/// for a new thread has another SRR0: that parked host thread is dead).
pub(super) static OS_PARKED: Mutex<Option<HashMap<u32, (Sender<Wake>, u32)>>> = Mutex::new(None);
thread_local! {
    /// This host thread's OS context (the last one it saved) and its wake channel.
    static MY_CTX: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    static MY_WAKE: std::cell::RefCell<Option<(Sender<Wake>, Receiver<Wake>)>> = const { std::cell::RefCell::new(None) };
}

pub(super) fn os_save_context(c: &mut Cpu) {
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
pub(super) fn load_regs(c: &mut Cpu, ctx: u32) {
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

pub(super) fn my_wake() -> (Sender<Wake>, ()) {
    MY_WAKE.with(|w| {
        let mut w = w.borrow_mut();
        if w.is_none() { *w = Some(channel()); }
        (w.as_ref().unwrap().0.clone(), ())
    })
}

pub(super) fn os_load_context(c: &mut Cpu) {
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
                    // a thread an interrupt preempted the moment an `mtmsr` enabled it (a snapshot's thread): the
                    // lifted code has no label there, so do the function's tail (old EE bit to r3, blr) here
                    match pc {
                        0x8009_DF5C => { c.r[3] = c.r[3] >> 15 & 1; pc = c.lr; } // OSEnableInterrupts
                        0x8009_DF80 => { c.r[3] = c.r[4] >> 15 & 1; pc = c.lr; } // OSRestoreInterrupts
                        _ => {}
                    }
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
pub(super) const BG_PASS: u32 = 0x800F_EB9C;

/// The background thread finished a pass with nothing pending: on the console it would spin until the next
/// interrupt; time jumps there instead (the pass is redone then), as the idle loop does.
pub(super) fn bg_fast_forward(c: &mut Cpu) {
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

pub(super) fn os_restore_interrupts(c: &mut Cpu) {
    let was = EE.load(std::sync::atomic::Ordering::Relaxed);
    let on = c.r[3] & 0x8000 != 0;
    if on && c.lr == BG_PASS && FAST_BG.load(std::sync::atomic::Ordering::Relaxed) { bg_fast_forward(c); }
    set_ee(c, on);
    c.r[3] = if was { 0x8000 } else { 0 };
}

/// Emulated time: the time base runs at 40.5 MHz (bus/4). Between video fields it advances with the work
/// done (TB_PER_BLOCK per lifted block, ~ a 486 MHz CPU doing a few instructions per block); a field that
/// waits jumps it to the field's start (675675 ticks per NTSC field).
pub(super) const TB_HZ: u64 = 40_500_000;
pub(super) const TB_PER_FIELD: u64 = 675_675;
pub(super) const TB_PER_BLOCK_X100: u64 = 40;
pub(super) static TB_BASE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub(super) static TB_MARK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub(super) static FIELD_TB: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);


pub fn time_base(c: &Cpu) -> u64 {
    use std::sync::atomic::Ordering::Relaxed;
    TB_BASE.load(Relaxed) + c.steps.saturating_sub(TB_MARK.load(Relaxed)) * TB_PER_BLOCK_X100 / 100
}

/// A video field begins: time is at least the field's start.
pub(super) fn tb_field(c: &Cpu) {
    use std::sync::atomic::Ordering::Relaxed;
    let f = FIELD_TB.load(Relaxed) + TB_PER_FIELD;
    FIELD_TB.store(f, Relaxed);
    if time_base(c) < f { TB_BASE.store(f, Relaxed); TB_MARK.store(c.steps, Relaxed); }
}

/// `OSGetTime` 0x800A2778 (r3:r4) and `OSGetTick` 0x800A2790 (r3): the time base.
pub(super) fn os_get_time(c: &mut Cpu) { let t = time_base(c); c.r[3] = (t >> 32) as u32; c.r[4] = t as u32; }
pub(super) fn os_get_tick(c: &mut Cpu) { c.r[3] = time_base(c) as u32; }

/// `OSYieldThread` 0x800A1990: only the main OS thread exists; pending interrupts get handled.
#[allow(dead_code)]
pub(super) fn os_yield_thread(c: &mut Cpu) { raise_pending(c); }

/// `OSSleepThread` 0x800A238C: handle pending interrupts; if none, the queue's own interrupt fires.
#[allow(dead_code)]
pub(super) fn os_sleep_thread(c: &mut Cpu) {
    let q = c.r[3];
    if raise_pending(c) { return; }
    let Some(&(_, irq, _)) = INTERRUPTS.iter().find(|i| i.0 == q) else {
        panic!("OSSleepThread on queue {q:#010x} (lr {:#010x}): no interrupt source modelled", c.lr)
    };
    if irq == 0x18 { field(c) } else { raise(c, irq) }
}
