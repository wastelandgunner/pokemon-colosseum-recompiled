//! Serial interface (SI) and the PAD library: command transfers to the controller ports, and `PADRead`.

use super::*;

/// SI command transfers (COMCSR 0xCC006434 TSTART): the command bytes are in the SI buffer 0xCC006480
/// (OUTLNGTH bits 22..16), the reply goes back there (INLNGTH bits 14..8). A port with a host controller
/// (`set_pads`) is a standard controller with rumble (0x00/0xFF type: 09 00 00; 0x41/0x42 origin: buttons
/// 00 80, sticks 80 80 80 80, triggers 00 00, 00 00); an empty port answers nothing (SISR NOREP, COMERR). Then
/// TSTART clears, TCINT sets and the SI interrupt is pending if TCINTMSK.
pub(super) fn si_transfer(m: &mut crate::mem::Mem) {
    let csr = m.u32(0xCC00_6434);
    let ch = (csr >> 1) & 3;
    let out = ((csr >> 16) & 0x7F).max(1) as usize;
    let inl = ((csr >> 8) & 0x7F).max(1) as usize;
    let cmd = m.u8(0xCC00_6480);
    let _ = out;
    let reply: Option<Vec<u8>> = if PADS.lock().unwrap()[ch as usize].is_some() {
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

/// A host controller (PADStatus fields): buttons (PAD_BUTTON_*), main stick x/y, C stick x/y, L/R analog.
#[derive(Clone, Copy, Default, Debug)]
pub struct Pad { pub buttons: u16, pub stick: (i8, i8), pub cstick: (i8, i8), pub trig: (u8, u8) }
/// The four controller ports (None = nothing plugged in). Port 1 starts plugged in.
pub(super) static PADS: Mutex<[Option<Pad>; 4]> = Mutex::new([Some(Pad { buttons: 0, stick: (0, 0), cstick: (0, 0), trig: (0, 0) }), None, None, None]);
/// Rumble motor command per port, as the game last sent it (PADControlMotor: 0 stop, 1 rumble, 2 stop hard).
pub(super) static MOTORS: Mutex<[u8; 4]> = Mutex::new([0; 4]);

/// One SI poll (each video field, once polling is on): every channel enabled in SIPOLL (EN0-3, bits 7..4)
/// answers as a standard controller (Dolphin's `CSIDevice_GCController::GetData`: SICxINBUFH = buttons |
/// PAD_USE_ORIGIN, stick x, y; SICxINBUFL packed by the analog mode the poll command (SICxOUTBUF) asks for) and sets
/// its RDST bit, or, with nothing plugged in, sets NOREP and the error bits. Returns whether any channel answered
/// (RDSTINT). The game's own PADRead turns these into PADStatus (origin, analog mode, errors).
pub(super) fn poll(m: &mut crate::mem::Mem) -> bool {
    let en = m.u32(0xCC00_6430) >> 4 & 0xF;
    let pads = *PADS.lock().unwrap();
    let mut sr = m.u32(0xCC00_6438);
    let mut any = false;
    for ch in 0..4u32 {
        if en >> (3 - ch) & 1 == 0 { continue; }
        let r = 0xCC00_6400 + 12 * ch;
        let shift = 8 * (3 - ch);
        match pads[ch as usize] {
            Some(p) => {
                let u = |v: i8| (v as i32 + 0x80) as u32 & 0xFF;
                let (sx, sy, cx, cy) = (u(p.stick.0), u(p.stick.1), u(p.cstick.0), u(p.cstick.1));
                let (tl, tr) = (p.trig.0 as u32, p.trig.1 as u32);
                let (aa, ab) = (0u32, 0u32);
                let hi = ((p.buttons as u32 | 0x0080) << 16) | sx << 8 | sy;
                let lo = match m.u32(r) >> 8 & 0xFF {
                    1 => ab >> 4 | (aa >> 4) << 4 | tr << 8 | tl << 16 | (cy >> 4) << 24 | (cx >> 4) << 28,
                    2 => ab | aa << 8 | (tr >> 4) << 16 | (tl >> 4) << 20 | (cy >> 4) << 24 | (cx >> 4) << 28,
                    3 => tr | tl << 8 | cy << 16 | cx << 24,
                    4 => ab | aa << 8 | cy << 16 | cx << 24,
                    _ => ab >> 4 | (aa >> 4) << 4 | (tr >> 4) << 8 | (tl >> 4) << 12 | cy << 16 | cx << 24,
                };
                m.set_u32(r + 4, hi);
                m.set_u32(r + 8, lo);
                sr |= 0x20 << shift; // RDSTn
                any = true;
            }
            None => {
                // no response: NOREPn, and ERRSTAT / ERRLATCH in the input buffer
                sr |= 0x08 << shift;
                let hi = m.u32(r + 4);
                m.set_u32(r + 4, hi | 0xC000_0000);
            }
        }
    }
    m.set_u32(0xCC00_6438, sr);
    any
}

/// After the poll's interrupt: the handler read the input buffers, which clears RDSTn (and so RDSTINT).
pub(super) fn poll_done(m: &mut crate::mem::Mem) {
    let sr = m.u32(0xCC00_6438);
    m.set_u32(0xCC00_6438, sr & !0x2020_2020);
    let csr = m.u32(0xCC00_6434);
    m.set_u32(0xCC00_6434, csr & !0x1000_0000);
}

/// SI channel output buffers (SICxOUTBUF 0xCC006400 + 12 * chan): the poll command, whose low two bits are the
/// rumble motor (`PADControlMotor`: SISetCommand(chan, 0x40 << 16 | analog mode << 8 | motor)).
pub(super) fn outbuf_write(m: &mut crate::mem::Mem, a: u32, n: usize) {
    for ch in 0..4 {
        let r = 0xCC00_6400 + 12 * ch;
        if covers(a, n, r) || covers(a, n, r + 3) {
            let v = m.u32(r);
            if v >> 16 & 0xFF == 0x40 { MOTORS.lock().unwrap()[ch as usize] = (v & 3) as u8; }
        }
    }
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
