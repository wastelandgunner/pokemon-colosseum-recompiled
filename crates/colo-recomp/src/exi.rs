//! The EXI bus (0xCC006800, 3 channels x 0x14: CSR, MAR, LENGTH, CR, DATA) and its devices, for run.rs.
//!
//! The SDK starts a transfer by writing CR with TSTART (`EXIImm` / `EXIDma`); the hardware moves the bytes
//! between the selected device (CSR chip-select bits 7-9, set by `EXISelect`) and DATA or memory, then clears
//! TSTART and raises TCINT (an interrupt when TCINTMASK is set). Here a transfer completes as soon as it starts.
//!
//! Devices: channel 0 device 1 is the IPL chip: ROM, the RTC (0x20000000), SRAM (0x20000100) and the debug UART
//! (0x20010000, where `OSReport` writes). Slots A and B (channels 0 and 1, device 0) can hold a memory card
//! (memcard.rs).

use crate::ppc::Cpu;

const BASE: u32 = 0xCC00_6800;

#[derive(Default)]
struct Ipl { cmd: Option<u32>, pos: u32, sram: Vec<u8>, uart: Vec<u8> }

#[derive(Default)]
pub struct Exi {
    /// Selected device per channel (CSR CS bit index), and bytes transferred since selection.
    sel: [Option<u32>; 3],
    ipl: Ipl,
    /// Memory card in slot A (channel 0 device 0).
    pub card: Option<crate::memcard::Card>,
    /// Memory card in slot B (channel 1 device 0).
    pub card_b: Option<crate::memcard::Card>,
    /// An EXI interrupt a card raised (erase / program done) for run.rs to deliver, per slot.
    pub card_irq: [bool; 2],
    /// `OSReport` text (the debug UART), line by line.
    pub log: Vec<String>,
    /// The RTC (seconds since 2000) as of the current transfer (run::rtc_now: emulated time).
    rtc: u32,
}

impl Exi {
    /// The SRAM's memory card flash ID for slot `k` (the console's ID a formatted card carries).
    pub fn flash_id(&self, k: usize) -> [u8; 12] { self.ipl.sram[0x14 + 12 * k..0x20 + 12 * k].try_into().unwrap() }

    /// The memory card in slot `k` (0 A, 1 B).
    pub fn slot(&mut self, k: usize) -> Option<&mut crate::memcard::Card> {
        match k { 0 => self.card.as_mut(), 1 => self.card_b.as_mut(), _ => None }
    }

    pub fn new() -> Exi {
        let mut e = Exi::default();
        e.ipl.sram = default_sram();
        e
    }

    /// After `EXISelect` / `EXIDeselect`: track the chip select (a new selection starts a new command).
    pub fn select(&mut self, c: &Cpu, ch: usize) {
        let csr = c.m.u32(BASE + 0x14 * ch as u32);
        let cs = (csr >> 7) & 7;
        let dev = (0..3).find(|k| cs >> k & 1 != 0);
        if dev != self.sel[ch] {
            if self.sel[ch] == Some(1) && ch == 0 { self.end_ipl(); }
            if self.sel[ch] == Some(0) && ch < 2 {
                if let Some(cd) = self.slot(ch) { if cd.deselect() { self.card_irq[ch] = true; } }
            }
            if dev == Some(0) && ch < 2 { if let Some(cd) = self.slot(ch) { cd.select(); } }
            self.sel[ch] = dev;
            if ch == 0 && dev == Some(1) { self.ipl.cmd = None; self.ipl.pos = 0; }
        }
    }

    /// After `EXIImm` / `EXIDma` started a transfer: run it. Returns the TC interrupt to raise, if enabled.
    pub fn transfer(&mut self, c: &mut Cpu, ch: usize) -> Option<u32> {
        self.rtc = crate::run::rtc_now(c);
        let b = BASE + 0x14 * ch as u32;
        let cr = c.m.u32(b + 0xC);
        if cr & 1 == 0 { return None; }
        let rw = (cr >> 2) & 3;
        let dev = self.sel[ch];
        if cr & 2 != 0 {
            let mar = (c.m.u32(b + 4) & 0x03FF_FFE0) | 0x8000_0000;
            let len = c.m.u32(b + 8);
            if rw == 1 {
                let d = c.m.read(mar, len).to_vec();
                for x in d { self.write_byte(ch, dev, x); }
            } else {
                let d: Vec<u8> = (0..len).map(|_| self.read_byte(ch, dev)).collect();
                c.m.write(mar, &d);
            }
        } else {
            let n = ((cr >> 4) & 3) + 1;
            let data = c.m.u32(b + 0x10);
            let mut out = 0u32;
            for k in 0..n {
                if rw != 0 { self.write_byte(ch, dev, (data >> (24 - 8 * k)) as u8); }
                if rw != 1 { out |= (self.read_byte(ch, dev) as u32) << (24 - 8 * k); }
            }
            if rw != 1 { c.m.set_u32(b + 0x10, out); }
        }
        c.m.set_u32(b + 0xC, cr & !1);
        let csr = c.m.u32(b);
        c.m.set_u32(b, csr | 8);
        // __OSInterrupt EXI_n_TC: 10, 13, 16
        if csr & 4 != 0 { Some(10 + 3 * ch as u32) } else { None }
    }

    fn write_byte(&mut self, ch: usize, dev: Option<u32>, x: u8) {
        if ch < 2 && dev == Some(0) { if let Some(cd) = self.slot(ch) { cd.write(x); } return; }
        if ch == 0 && dev == Some(1) {
            let p = &mut self.ipl;
            match p.cmd {
                None => {
                    p.pos += 1;
                    let v = p.uart.len(); // reuse as the 4-byte command accumulator
                    let _ = v;
                    p.uart.push(x);
                    if p.uart.len() == 4 {
                        let cmd = u32::from_be_bytes([p.uart[0], p.uart[1], p.uart[2], p.uart[3]]);
                        p.uart.clear();
                        p.cmd = Some(cmd);
                        p.pos = 0;
                    }
                }
                Some(cmd) => {
                    let a = cmd & 0x7FFF_FFFF;
                    if (0x2001_0000..0x2001_0100).contains(&a) {
                        p.uart.push(x);
                    } else if (0x2000_0100..0x2000_0200).contains(&a) {
                        let o = (((a - 0x2000_0100) >> 6) + p.pos) as usize;
                        if o < p.sram.len() { p.sram[o] = x; }
                    }
                    p.pos += 1;
                }
            }
        }
    }

    fn read_byte(&mut self, ch: usize, dev: Option<u32>) -> u8 {
        if ch < 2 && dev == Some(0) { return self.slot(ch).map_or(0xFF, |cd| cd.read()); }
        if ch == 0 && dev == Some(1) {
            let p = &mut self.ipl;
            if let Some(cmd) = p.cmd {
                let a = cmd & 0x7FFF_FFFF;
                let o = p.pos as usize;
                p.pos += 1;
                if a == 0x2000_0000 {
                    // RTC: seconds since 2000-01-01, big-endian
                    let t = self.rtc;
                    return t.to_be_bytes().get(o).copied().unwrap_or(0);
                }
                if (0x2000_0100..0x2000_0200).contains(&a) {
                    let o = ((a - 0x2000_0100) >> 6) as usize + o;
                    return p.sram.get(o).copied().unwrap_or(0);
                }
                // IPL ROM (fonts, boot code): not available
                return 0;
            }
        }
        // nothing selected / no device: the bus reads high
        0xFF
    }

    fn end_ipl(&mut self) {
        let p = &mut self.ipl;
        if let Some(cmd) = p.cmd {
            if (cmd & 0x7FFF_FFFF) == 0x2001_0000 && !p.uart.is_empty() {
                // UART bytes come in 4-byte words prefixed by... the SDK sends raw characters
                let text = String::from_utf8_lossy(&p.uart).to_string();
                for l in text.split('\n') { if !l.is_empty() { self.log.push(l.to_string()); } }
                if std::env::var("RUN_OSREPORT").is_ok() { eprint!("{text}"); }
            }
        }
        p.uart.clear();
        p.cmd = None;
        p.pos = 0;
    }
}

/// The SRAM the IPL keeps (64 bytes), as Dolphin creates it: English, stereo, no offsets; checksums over
/// words 6..0x13 (u16 sum and inverse sum).
fn default_sram() -> Vec<u8> {
    let mut s = vec![0u8; 64];
    s[0x13] = 0x2C; // flags: stereo (bit 2), progressive off; Dolphin default 0x2C
    // the memory card flash IDs (slots A, B) and their checksums (~byte sum), as Dolphin's SRAM has them: cards
    // formatted in Dolphin carry this console's ID
    for (k, id) in [b"DOLPHINSLOTA", b"DOLPHINSLOTB"].iter().enumerate() {
        s[0x14 + 12 * k..0x20 + 12 * k].copy_from_slice(&id[..]);
        s[0x3A + k] = !id.iter().fold(0u8, |a, &b| a.wrapping_add(b));
    }
    let mut sum: u16 = 0;
    let mut inv: u16 = 0;
    for k in (0x0C..0x14).step_by(2) {
        let w = u16::from_be_bytes([s[k], s[k + 1]]);
        sum = sum.wrapping_add(w);
        inv = inv.wrapping_add(!w);
    }
    s[0..2].copy_from_slice(&sum.to_be_bytes());
    s[2..4].copy_from_slice(&inv.to_be_bytes());
    s
}
