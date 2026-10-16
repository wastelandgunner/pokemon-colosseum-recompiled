//! Disc interface (DI) under the DVD library: commands run against the host's disc image at once.

use super::*;

pub const DVD_READ: u32 = 0x800A42C4; // the DI driver's `Read` (every read command goes through it)
pub const DVD_LOW_SEEK: u32 = 0x800A46EC;
pub(super) const IRQ_DI: u32 = 0x15;

/// The disc image the DI reads from (offset, buffer).
pub type Disc = Box<dyn Fn(u64, &mut [u8]) + Send + Sync>;
pub(super) static DISC: Mutex<Option<Disc>> = Mutex::new(None);

/// The DI (0xCC006000): writing DICR with TSTART runs the command in DICMDBUF0-2 (DIMAR / DILENGTH for DMA),
/// at once: read 0xA8 (subcommand 0x00 data, 0x40 disk ID), seek 0xAB, inquiry 0x12, request error 0xE0
/// (DIIMMBUF 0 = no error), stop motor 0xE3, audio buffer config 0xE4; then DILENGTH = 0, TSTART clears,
/// DISR TCINT (0x10) is set and the DI interrupt is pending if TCINTMASK (0x08).
pub(super) fn di_execute(m: &mut crate::mem::Mem) {
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
