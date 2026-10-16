//! Serial interface (SI) and the PAD library: command transfers to the controller ports, and `PADRead`.

use super::*;

/// SI command transfers (COMCSR 0xCC006434 TSTART): the command bytes are in the SI buffer 0xCC006480
/// (OUTLNGTH bits 22..16), the reply goes back there (INLNGTH bits 14..8). Port 1 is a standard controller
/// (0x00/0xFF type: 09 00 00; 0x41/0x42 origin: buttons 00 80, sticks 80 80 80 80, triggers 00 00, 00 00);
/// the other ports answer nothing (SISR NOREP, COMERR). Then TSTART clears, TCINT sets and the SI interrupt
/// is pending if TCINTMSK.
pub(super) fn si_transfer(m: &mut crate::mem::Mem) {
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

pub const PAD_READ: u32 = 0x800AB150;

/// The host's controller 1 (PADStatus fields): buttons (PAD_BUTTON_*), main stick x/y, C stick x/y, L/R analog.
#[derive(Clone, Copy, Default, Debug)]
pub struct Pad { pub buttons: u16, pub stick: (i8, i8), pub cstick: (i8, i8), pub trig: (u8, u8) }
pub(super) static PAD: Mutex<Pad> = Mutex::new(Pad { buttons: 0, stick: (0, 0), cstick: (0, 0), trig: (0, 0) });

/// `PADRead(PADStatus status[4])` 0x800AB150: the SI poll results. Port 1 is the host's pad, ports 2-4 have
/// no controller (err PAD_ERR_NO_CONTROLLER = -1). Returns the connected-channel bits (PAD_CHAN0_BIT).
pub(super) fn pad_read(c: &mut Cpu) {
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

/// SI COMCSR (0xCC006434) writes.
pub(super) fn comcsr_write(m: &mut crate::mem::Mem, a: u32, n: usize) {
    if covers(a, n, 0xCC00_6434) || covers(a, n, 0xCC00_6437) {
        // COMCSR: TCINT (bit 31) is write-1-to-clear
        let v = m.u32(0xCC00_6434);
        if v & 0x8000_0000 != 0 { m.set_u32(0xCC00_6434, v & !0x8000_0000); }
        if v & 1 != 0 { si_transfer(m); }
    }
}
