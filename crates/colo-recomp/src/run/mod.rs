//! Whole game frames through the lifted code (colo-viewer's battle runner): `GSgappUpdate` per frame, with the GS
//! cooperative threads as host coroutines.
//!
//!
//! The hardware is modelled at the SDK boundary, one module per SDK library: gs (the game's GS threads), os,
//! vi, si (PAD), dvd, ar, ai, dsp, exi, gx. This module holds what they share (interrupt delivery, the MMIO
//! write dispatch) and the `Machine` that drives the game.

use crate::lifted;
use crate::ppc::Cpu;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;

mod ai;
mod ar;
mod dsp;
mod dvd;
mod exi;
mod gs;
mod gx;
mod os;
mod si;
mod vi;
use ai::*;
pub use ar::*;
pub use dsp::*;
pub use dvd::*;
pub use exi::*;
pub use gs::*;
pub use gx::*;
pub use os::*;
pub use si::*;
pub use vi::*;

/// Coroutine stack: the lifted code recurses on the host stack.
const CO_STACK: usize = 256 << 20;

struct CpuPtr(*mut Cpu);
unsafe impl Send for CpuPtr {}

/// Resume carries the channel a GS coroutine reports back on (the scheduler call that resumed it).
enum Wake { Resume(Option<Sender<Back>>), Die }
enum Back { Yield { ctx: u32, pc: u32 }, Panic(String) }
struct Die;

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

/// Hardware register writes (mem.rs `mmio_write`): the devices whose registers have side effects.
static MMIO_BUSY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn covers(a: u32, n: usize, r: u32) -> bool { a <= r && r < a + n as u32 }

fn mmio(m: &mut crate::mem::Mem, a: u32, n: usize) {
    if MMIO_BUSY.swap(true, std::sync::atomic::Ordering::Relaxed) { return; }
    dsp::control_write(m, a, n);
    ar::dma_write(m, a, n);
    if covers(a, n, 0xCC00_601F) && m.u32(0xCC00_601C) & 1 != 0 { di_execute(m); }
    exi::csr_write(m, a, n);
    si::comcsr_write(m, a, n);
    ai::control_write(m, a, n);
    MMIO_BUSY.store(false, std::sync::atomic::Ordering::Relaxed);
}

/// Dolphin-OS / hardware waits the host replaces (the viewer paces frames and owns the GPU, DVD and audio):
/// (address, r3 or u32::MAX to leave r3).
pub const HOST_STUBS: &[(u32, u32)] = &[
];

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
        // nor the AI DMA (AIDCR/AIDMA regs): without it no AI interrupt runs MusyX's `salCallback` 0x80163EE0, so the
        // synth never ticks and a fanfare's soundGetStatus stays 2 (the battle's win fanfare waits forever). Restart
        // it where salCallback left it: AIInitDMA(block 0x8047B0A0 (already advanced) of its 0xA00 buffer 0x8047B09C, 0x280)
        let (buf, idx) = (m.c.m.u32(0x8047_B09C), m.c.m.u8(0x8047_B0A0) as u32);
        if buf != 0 {
            let a = (buf & 0x01FF_FFFF) + (idx & 3) * 0x280;
            m.c.m.set_u16(0xCC00_5030, a >> 16);
            m.c.m.set_u16(0xCC00_5032, a & 0xFFE0);
            m.c.m.set_u16(0xCC00_5036, 0x8000 | (0x280 / 32));
        }
        if let Ok(a) = std::fs::read(dir.join("l1.bin")) { m.set_l1(&a); }
        // nor the EFB: a battle wipe's back-buffer capture (`GSgfxBackFBDoFrame`) copies it into the fade texture at
        // the start of the next frame, before anything is drawn, so a black EFB blacks out the rest of the wipe.
        // The fade texture holds the capture of one frame earlier: start the EFB from it.
        if let Some(t) = m.fade_texture() {
            let b = m.c.m.read(t, 640 * 480 * 2).to_vec();
            let mut rgba = vec![255u8; 640 * 480 * 4];
            for (i, p) in b.chunks(2).enumerate() {
                let v = u16::from_be_bytes([p[0], p[1]]) as u32;
                // 4x4 tiles of RGB565
                let (blk, k) = (i / 16, i % 16);
                let (x, y) = ((blk % 160) * 4 + k % 4, (blk / 160) * 4 + k / 4);
                let o = (y * 640 + x) * 4;
                let (r, g, bl) = (v >> 11, v >> 5 & 63, v & 31);
                rgba[o] = (r << 3 | r >> 2) as u8; rgba[o + 1] = (g << 2 | g >> 4) as u8; rgba[o + 2] = (bl << 3 | bl >> 2) as u8;
            }
            m.seed_efb(&rgba);
        }
        Ok(m)
    }

    /// The battle wipe's fade texture (`fadeState` 0x80466E30 +0x10 -> GStexture: 640x480 RGB565, data +0x28), if any.
    pub fn fade_texture(&self) -> Option<u32> {
        let t = self.c.m.u32(0x8046_6E40);
        if !(0x8000_0000..0x8180_0000).contains(&t) || self.c.m.u32(t) != 640 << 16 | 480 { return None; }
        let d = self.c.m.u32(t + 0x28);
        (0x8000_0000..0x8180_0000).contains(&d).then_some(d)
    }

    /// Replace the fade texture's texels with an RGBA8 640x480 image (encoded RGB565 in 4x4 tiles).
    pub fn set_fade_texture(&mut self, rgba: &[u8]) {
        let Some(d) = self.fade_texture() else { return };
        if rgba.len() < 640 * 480 * 4 { return; }
        let mut out = Vec::with_capacity(640 * 480 * 2);
        for by in (0..480).step_by(4) {
            for bx in (0..640).step_by(4) {
                for y in by..by + 4 {
                    for x in bx..bx + 4 {
                        let p = &rgba[(y * 640 + x) * 4..];
                        let v = ((p[0] as u16 >> 3) << 11) | ((p[1] as u16 >> 2) << 5) | (p[2] as u16 >> 3);
                        out.extend_from_slice(&v.to_be_bytes());
                    }
                }
            }
        }
        self.c.m.write(d, &out);
    }

    /// Fill the software renderer's EFB (top 640x480) with an RGBA8 image (gamma values, as the XFB shows).
    pub fn seed_efb(&mut self, rgba: &[u8]) {
        for (i, p) in rgba.chunks(4).take(640 * 480).enumerate() { self.gpu.color[i] = [p[0], p[1], p[2], 255]; }
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
    /// The frame `GXFlush` decoded so far is dropped too: without a renderer it would keep every draw (a headless
    /// story battle grew to 26 GB).
    pub fn take_gx(&mut self) -> Vec<u8> {
        if let Some(g) = GPU_IN.lock().unwrap().as_mut() { g.frame = colo_gx::Frame::default(); }
        std::mem::take(self.c.m.pipe.as_mut().unwrap())
    }

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
