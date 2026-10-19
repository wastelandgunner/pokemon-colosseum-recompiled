//! Disc interface (DI) under the DVD library: commands run against the host's disc image, finishing when the drive would.

use super::*;

pub const DVD_READ: u32 = 0x800A42C4; // the DI driver's `Read` (every read command goes through it)
pub const DVD_LOW_SEEK: u32 = 0x800A46EC;
pub(super) const IRQ_DI: u32 = 0x15;

/// The disc image the DI reads from (offset, buffer).
pub type Disc = Box<dyn Fn(u64, &mut [u8]) + Send + Sync>;
pub(super) static DISC: Mutex<Option<Disc>> = Mutex::new(None);

/// The DI (0xCC006000): writing DICR with TSTART starts the command in DICMDBUF0-2 (DIMAR / DILENGTH for DMA).
/// It completes when the drive would finish it (`Drive`, Dolphin's timing): read 0xA8 (subcommand 0x00 data,
/// 0x40 disk ID), seek 0xAB, inquiry 0x12, request error 0xE0 (DIIMMBUF 0 = no error), stop motor 0xE3, audio
/// buffer config 0xE4; then DILENGTH = 0, TSTART clears, DISR TCINT (0x10) is set and the DI interrupt is pending
/// if TCINTMASK (0x08).
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
    /// Its completion (time base), once scheduled.
    due: Option<u64>,
    buf_start_time: u64, buf_end_time: u64, buf_start_off: u64, buf_end_off: u64,
}
pub(super) static DRIVE: Mutex<Drive> = Mutex::new(Drive { started: false, due: None, buf_start_time: 0, buf_end_time: 0, buf_start_off: 0, buf_end_off: 0 });

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
    let cmd = c.m.u32(0xCC00_6008);
    let ticks = if cmd >> 24 == 0xA8 {
        let (off, len) = if cmd & 0xC0 == 0x40 { (0, 0x20) } else { ((c.m.u32(0xCC00_600C) as u64) << 2, c.m.u32(0xCC00_6010) as u64) };
        d.schedule_read(off, len.max(1), now * 12)
    } else { 300 * (TICKS_PER_SEC / 1_000_000) };
    let t = now + ticks / 12;
    d.due = Some(t);
    Some(t)
}

/// The started DI command completes if its time has come (DVD interrupt pending).
pub(super) fn di_due(c: &mut Cpu) {
    match di_next(c) { Some(t) if t <= time_base(c) => {} _ => return }
    { let mut d = DRIVE.lock().unwrap(); d.started = false; d.due = None; }
    di_complete(&mut c.m);
}

fn di_complete(m: &mut crate::mem::Mem) {
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
