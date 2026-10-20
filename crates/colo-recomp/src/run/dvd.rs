//! Disc interface (DI) under the DVD library: commands run against the host's disc image, finishing when the drive would.

use super::*;

pub const DVD_READ: u32 = 0x800A42C4; // the DI driver's `Read` (every read command goes through it)
pub const DVD_LOW_SEEK: u32 = 0x800A46EC;
pub(super) const IRQ_DI: u32 = 0x15;

/// The disc image the DI reads from (offset, buffer).
pub type Disc = Box<dyn Fn(u64, &mut [u8]) + Send + Sync>;
pub(super) static DISC: Mutex<Option<Disc>> = Mutex::new(None);

/// The DI (0xCC006000): writing DICR with TSTART starts the command in DICMDBUF0-2 (DIMAR / DILENGTH for DMA).
/// It completes when the drive would finish it (`Drive`, Dolphin's timing), with the drive's state and error code
/// as Dolphin's DVDInterface `ExecuteCommand` keeps them (described, not copied): inquiry 0x12, read 0xA8
/// (subcommand 0x00 data, 0x40 disc ID; past the disc's end: BlockOOB), seek 0xAB, BCA 0xDA (0x40 bytes, byte 0x33
/// = 1), request error 0xE0 (DIIMMBUF = drive state << 24 | the last command's error), audio stream 0xE1 / status
/// 0xE2 (this disc has no streaming audio: header byte 8 = 0, so NoAudioBuf), stop motor 0xE3 (reads then fail
/// with MotorStopped), audio buffer config 0xE4 (only before the first read: else InvalidPeriod), drive unlock
/// 0xFF; every other command (Wii-only, patched-firmware and debug ones, anything unknown) is InvalidCommand.
/// Then DILENGTH = 0, TSTART clears and DISR gets TCINT (0x10), or DEINT (0x04) on an error, with the DI interrupt
/// pending if its mask (TCINTMASK 0x08 / DEINTMASK 0x02) is set.
pub(super) fn di_execute(_m: &mut crate::mem::Mem) {
    let mut d = DRIVE.lock().unwrap();
    d.started = true;
    d.due = None;
}

/// CPU clock (Dolphin's core timing ticks; the time base runs at 1/12 of it).
const TICKS_PER_SEC: u64 = 486_000_000;

/// The disc drive's timing, as Dolphin models it (DVDInterface.cpp `ScheduleReads`, DVDMath.cpp; described, not
/// copied): every command takes 300 us, a read 600 us plus, per 32 KB ECC block, either a transfer from the
/// drive's 1 MB read-ahead buffer at 32 MiB/s or a disc access: a seek (linear in the head's radial travel:
/// 35 ms + 50 s/m below 1 mm, 75 ms + 4.5 s/m above) plus rotational latency (28.5 rev/s; the track is an
/// Archimedean spiral of 0.74 um pitch) when the block isn't under the head, else the raw read time (CAV:
/// 2.1 MiB/s at the 24 mm inner radius to 3.325 MiB/s at 38 mm). After a read the drive keeps prefetching to
/// 1 MB beyond it (minus the two blocks it keeps behind), which later reads find buffered once its time has come.
pub(super) struct Drive {
    /// A command was started (DICR TSTART) and has not completed.
    started: bool,
    /// Drive state (Dolphin's DriveState): 0 ready, 1 ready with no reads made yet (the IPL has read the disc ID),
    /// 5 motor stopped.
    state: u8,
    /// The last command's error (Dolphin's DriveError), for request error 0xE0.
    error: u32,
    /// DTK streaming audio enabled (audio buffer config 0xE4; the IPL sets it from disc header byte 8).
    dtk: bool,
    /// Its completion (time base), once scheduled.
    due: Option<u64>,
    buf_start_time: u64, buf_end_time: u64, buf_start_off: u64, buf_end_off: u64,
}
pub(super) static DRIVE: Mutex<Drive> = Mutex::new(Drive { started: false, state: 1, error: 0, dtk: false, due: None, buf_start_time: 0, buf_end_time: 0, buf_start_off: 0, buf_end_off: 0 });

/// A GameCube disc's end (single layer).
const DISC_END: u64 = 0x5705_8000;
const ERR_MOTOR_STOPPED: u32 = 0x20400;
const ERR_INVALID_COMMAND: u32 = 0x52000;
const ERR_NO_AUDIO_BUF: u32 = 0x52001;
const ERR_BLOCK_OOB: u32 = 0x52100;
const ERR_INVALID_PERIOD: u32 = 0x52402;

/// What the command in DICMDBUF does: Ok(Some((offset, length))) a disc read the drive times, Ok(None) a
/// command done in 300 us, Err(error) one that fails (DEINT, also 300 us).
fn di_plan(m: &crate::mem::Mem, d: &Drive) -> Result<Option<(u64, u64)>, u32> {
    let cmd = m.u32(0xCC00_6008);
    // Dolphin's CheckReadPreconditions (the disc is always in)
    let ready = || if d.state == 5 { Err(ERR_MOTOR_STOPPED) } else { Ok(()) };
    match cmd >> 24 {
        0x12 | 0xAB | 0xDA | 0xE0 | 0xE3 | 0xFF => Ok(None),
        0xA8 => {
            ready()?;
            let (off, len) = match cmd & 0xFF {
                0x40 => (0, 0x20),
                0x00 => ((m.u32(0xCC00_600C) as u64) << 2, m.u32(0xCC00_6010) as u64),
                _ => return Ok(None),
            };
            if off + len > DISC_END { return Err(ERR_BLOCK_OOB); }
            Ok(Some((off, len.max(1))))
        }
        0xE1 | 0xE2 => { ready()?; if d.dtk { Ok(None) } else { Err(ERR_NO_AUDIO_BUF) } }
        0xE4 => { ready()?; if d.state == 0 { Err(ERR_INVALID_PERIOD) } else { Ok(None) } }
        _ => Err(ERR_INVALID_COMMAND),
    }
}

const ECC: u64 = 0x8000;
const STREAM_BUF: u64 = 1 << 20;

fn radius(off: u64) -> f64 {
    const LAYER: u64 = 0x1_1824_0000;
    let mut off = off % (LAYER * 2);
    if off > LAYER { off = LAYER * 2 - off; }
    (off as f64 / LAYER as f64 * (0.058 * 0.058 - 0.024 * 0.024) + 0.024 * 0.024).sqrt()
}
fn seek_time(from: u64, to: u64) -> f64 {
    let d = (radius(from) - radius(to)).abs();
    if d < 0.001 { d * 50.0 + 0.035 } else { d * 4.5 + 0.075 }
}
fn rotational_latency(off: u64, time: f64) -> f64 {
    let target = (radius(off) / 0.000_000_74).rem_euclid(1.0);
    let start = (time * 28.5).rem_euclid(1.0);
    (target + 1.0 - start).rem_euclid(1.0) / 28.5
}
fn raw_read_time(off: u64, len: u64) -> f64 {
    let r = radius(off + len / 2);
    let speed = (r - 0.024) / (0.038 - 0.024) * (1024.0 * 1024.0 * (3.325 - 2.1)) + 1024.0 * 1024.0 * 2.1;
    len as f64 / speed
}

impl Drive {
    /// Ticks until a read of `len` bytes at `offset` started at `now` (ticks) completes; updates the buffer.
    fn schedule_read(&mut self, offset: u64, len: u64, now: u64) -> u64 {
        let tps = TICKS_PER_SEC;
        let mut dvd_off = offset & !(ECC - 1);
        let first = dvd_off;
        let (mut buf_start, mut buf_end, mut head);
        if self.buf_start_time == self.buf_end_time {
            buf_start = 0; buf_end = 0; head = 0;
        } else {
            buf_start = self.buf_end_off.saturating_sub(STREAM_BUF);
            buf_end = if now >= self.buf_end_time { self.buf_end_off } else {
                self.buf_start_off + ((now - self.buf_start_time) * (self.buf_end_off - self.buf_start_off)
                    / (self.buf_end_time - self.buf_start_time) & !(ECC - 1))
            };
            head = buf_end;
            if dvd_off < buf_start { buf_start = 0; buf_end = 0; }
        }
        let mut ticks = 600 * (tps / 1_000_000);
        let mut seek = false;
        let (mut offset, mut len) = (offset, len);
        loop {
            let chunk = (((offset + 1 + ECC - 1) & !(ECC - 1)) - offset).min(len);
            if dvd_off >= buf_start && dvd_off < buf_end {
                ticks += chunk * tps / (32 * 1024 * 1024);
            } else {
                if dvd_off != head {
                    seek = true;
                    ticks += (tps as f64 * seek_time(head, dvd_off)) as u64;
                    let t = (now + ticks) as f64 / tps as f64;
                    ticks += (tps as f64 * rotational_latency(dvd_off, t)) as u64;
                } else {
                    ticks += (tps as f64 * raw_read_time(dvd_off, ECC)) as u64;
                }
                head = dvd_off + ECC;
            }
            offset += chunk;
            len -= chunk;
            dvd_off += ECC;
            if len == 0 { break; }
        }
        let last = dvd_off;
        if !(last.wrapping_sub(buf_start) <= 2 * ECC && buf_start != buf_end) {
            self.buf_start_off = if last >= buf_end { last } else { buf_end };
            self.buf_end_off = last + STREAM_BUF - 2 * ECC;
            if seek { self.buf_end_off = self.buf_end_off.max(first + STREAM_BUF); }
            self.buf_start_time = now + ticks;
            self.buf_end_time = self.buf_start_time
                + (tps as f64 * raw_read_time(self.buf_start_off, self.buf_end_off - self.buf_start_off)) as u64;
        }
        ticks
    }
}

/// The started DI command's completion time (time base), scheduled from now if it isn't yet: the host sees the
/// DICR write without the CPU's clock, so a command is timed from the next point that has it.
pub(super) fn di_next(c: &Cpu) -> Option<u64> {
    let mut d = DRIVE.lock().unwrap();
    if !d.started { return None; }
    if let Some(t) = d.due { return Some(t); }
    let now = time_base(c);
    let ticks = match di_plan(&c.m, &d) {
        // the disc's rotation counts from power-on (Dolphin: CoreTiming ticks), when the time base started at 0
        Ok(Some((off, len))) => d.schedule_read(off, len, now * 12),
        _ => 300 * (TICKS_PER_SEC / 1_000_000),
    };
    let t = now + ticks / 12;
    d.due = Some(t);
    Some(t)
}

/// RUN_DITRACE: print each DI command's completion (time base, command, disc offset, length).
fn ditrace() -> bool { static T: std::sync::OnceLock<bool> = std::sync::OnceLock::new(); *T.get_or_init(|| std::env::var("RUN_DITRACE").is_ok()) }

/// The started DI command completes if its time has come (DVD interrupt pending).
pub(super) fn di_due(c: &mut Cpu) {
    match di_next(c) { Some(t) if t <= time_base(c) => {} _ => return }
    { let mut d = DRIVE.lock().unwrap(); d.started = false; d.due = None; }
    if ditrace() {
        let cmd = c.m.u32(0xCC00_6008);
        eprintln!("ditrace {} {:#x} {:#x} {:#x}", time_base(c), cmd, (c.m.u32(0xCC00_600C) as u64) << 2, c.m.u32(0xCC00_6010));
    }
    di_complete(&mut c.m);
}

fn di_complete(m: &mut crate::mem::Mem) {
    let cmd = m.u32(0xCC00_6008);
    let arg = m.u32(0xCC00_600C);
    let len = m.u32(0xCC00_6018);
    let dst = m.u32(0xCC00_6014) & 0x03FF_FFE0 | 0x8000_0000;
    if trace() { eprintln!("run: DI {cmd:#010x} {arg:#x} len {len:#x} -> {dst:#x}"); }
    let mut d = DRIVE.lock().unwrap();
    let plan = di_plan(m, &d);
    // request error reports the previous command's error; every other command starts clean
    if cmd >> 24 != 0xE0 { d.error = 0; }
    if let Err(e) = plan {
        d.error = e;
        if trace() || e == ERR_INVALID_COMMAND { eprintln!("run: DI command {cmd:#010x} fails with error {e:#07x}"); }
    } else {
        match cmd >> 24 {
            0xA8 => {
                if let Ok(Some((off, n))) = plan {
                    let _ = n;
                    let mut buf = vec![0u8; len as usize];
                    if let Some(disc) = DISC.lock().unwrap().as_ref() { disc(off, &mut buf); }
                    m.write(dst, &buf);
                    // the first disc ID read after the IPL's keeps the drive configurable; any other read doesn't
                    d.state = if cmd & 0xFF == 0x40 && d.state == 1 { 1 } else { 0 };
                } else {
                    eprintln!("run: DI read subcommand {cmd:#010x} not known (no data)");
                }
            }
            0x12 => {
                // drive info: revision 0x0002, device code 0x0604 (retail), release date 0x20020402
                let mut b = vec![0u8; len as usize];
                let info = [0x00, 0x00, 0x00, 0x02, 0x20, 0x02, 0x04, 0x02, 0x61, 0x00, 0x00, 0x00];
                let n = info.len().min(b.len());
                b[..n].copy_from_slice(&info[..n]);
                m.write(dst, &b);
            }
            0xDA => {
                let mut b = [0u8; 0x40];
                b[0x33] = 1;
                m.write(dst, &b);
            }
            0xE0 => {
                let st = if d.state <= 1 { 0 } else { d.state as u32 - 1 };
                m.set_u32(0xCC00_6020, st << 24 | d.error);
                d.error = 0;
            }
            0xE1 => {}
            0xE2 => m.set_u32(0xCC00_6020, 0),
            0xE3 => if d.state <= 1 { d.state = 5; },
            0xE4 => d.dtk = cmd >> 16 & 1 != 0,
            _ => {}
        }
    }
    drop(d);
    m.set_u32(0xCC00_6018, 0);
    let cr = m.u32(0xCC00_601C);
    m.set_u32(0xCC00_601C, cr & !1);
    let (int, mask) = if plan.is_err() { (0x04, 0x02) } else { (0x10, 0x08) };
    if disr_raise(m, int) & mask != 0 { PENDING.lock().unwrap().push(IRQ_DI); }
}

/// DISR's interrupt bits (BRKINT 0x40, TCINT 0x10, DEINT 0x04) as the drive raised them: the guest clears them by
/// writing 1s (`disr_write`), which a plain memory register can't do by itself.
static DISR_INT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Raise DISR interrupt bits; returns DISR.
pub(super) fn disr_raise(m: &mut crate::mem::Mem, bits: u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let int = DISR_INT.fetch_or(bits, Relaxed) | bits;
    let v = m.u32(0xCC00_6000) & !0x54 | int;
    let was = MMIO_BUSY.swap(true, Relaxed); // (not a guest write)
    m.set_u32(0xCC00_6000, v);
    MMIO_BUSY.store(was, Relaxed);
    v
}

/// DISR writes: the masks and BRK are plain bits; the interrupt bits are write-1-to-clear.
pub(super) fn disr_write(m: &mut crate::mem::Mem, a: u32, n: usize) {
    use std::sync::atomic::Ordering::Relaxed;
    if !(covers(a, n, 0xCC00_6000) || covers(a, n, 0xCC00_6003)) { return; }
    let w = m.u32(0xCC00_6000);
    let int = DISR_INT.fetch_and(!(w & 0x54), Relaxed) & !(w & 0x54);
    m.set_u32(0xCC00_6000, w & !0x54 | int);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mem() -> crate::mem::Mem {
        let mut m = crate::mem::Mem::new();
        m.map(0x8000_0000, vec![0; 0x10000]);
        m.map(0xCC00_0000, vec![0; 0x9000]);
        m
    }

    fn run(m: &mut crate::mem::Mem, cmd: [u32; 3]) -> u32 {
        for (k, v) in cmd.iter().enumerate() { m.set_u32(0xCC00_6008 + 4 * k as u32, *v); }
        m.set_u32(0xCC00_6014, 0x1000);
        m.set_u32(0xCC00_6018, 0x20);
        di_complete(m);
        let sr = m.u32(0xCC00_6000);
        disr_write(m, 0xCC00_6000, 4); // (a read-back; nothing to clear)
        m.set_u32(0xCC00_6000, sr & 0x54);
        disr_write(m, 0xCC00_6000, 4); // the guest acknowledges
        sr & 0x54
    }

    #[test]
    fn drive_errors() {
        let mut m = mem();
        *DRIVE.lock().unwrap() = Drive { started: false, state: 1, error: 0, dtk: false, due: None, buf_start_time: 0, buf_end_time: 0, buf_start_off: 0, buf_end_off: 0 };
        // an unknown command fails; request error reports it once, then no error
        assert_eq!(run(&mut m, [0xD000_0000, 0, 0]), 0x04);
        assert_eq!(run(&mut m, [0xE000_0000, 0, 0]), 0x10);
        assert_eq!(m.u32(0xCC00_6020), ERR_INVALID_COMMAND);
        assert_eq!(run(&mut m, [0xE000_0000, 0, 0]), 0x10);
        assert_eq!(m.u32(0xCC00_6020), 0);
        // no streaming audio on this disc
        assert_eq!(run(&mut m, [0xE100_0000, 0, 0]), 0x04);
        // audio buffer config is allowed before the first read only
        assert_eq!(run(&mut m, [0xE400_0000, 0, 0]), 0x10);
        assert_eq!(run(&mut m, [0xA800_0000, 0, 0x20]), 0x10);
        assert_eq!(run(&mut m, [0xE400_0000, 0, 0]), 0x04);
        // reads past the end of the disc
        assert_eq!(run(&mut m, [0xA800_0000, (DISC_END >> 2) as u32, 0x20]), 0x04);
        assert_eq!(run(&mut m, [0xE000_0000, 0, 0]), 0x10);
        assert_eq!(m.u32(0xCC00_6020), ERR_BLOCK_OOB);
        // stop motor: reads fail until the drive is reset; the state shows in request error
        assert_eq!(run(&mut m, [0xE300_0000, 0, 0]), 0x10);
        assert_eq!(run(&mut m, [0xA800_0000, 0, 0x20]), 0x04);
        assert_eq!(run(&mut m, [0xE000_0000, 0, 0]), 0x10);
        assert_eq!(m.u32(0xCC00_6020), 4 << 24 | ERR_MOTOR_STOPPED);
        // BCA
        DRIVE.lock().unwrap().state = 0;
        assert_eq!(run(&mut m, [0xDA00_0000, 0, 0]), 0x10);
        assert_eq!(m.u8(0x8000_1033), 1);
    }
}
