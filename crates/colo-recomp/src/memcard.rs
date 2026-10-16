//! A GameCube memory card (59 blocks, 4 Mbit, ID 0x00000004) on EXI channel 0 device 0, for run.rs / exi.rs,
//! and the card image built from GCI files.
//!
//! Protocol (command byte after chip select, as Dolphin's EXI_DeviceMemoryCard models the card):
//! 0x00 card ID (a dummy byte, then the 4-byte ID), 0x83 status, 0x89 clear status, 0x81 interrupt enable,
//! 0x52 read (AD1 AD2 AD3 BA: address = (AD1 & 0x7F) << 17 | AD2 << 9 | (AD3 & 3) << 7 | (BA & 0x7F); data after),
//! 0xF1 erase sector (AD1 AD2), 0xF2 program page (address, then up to 128 bytes), 0xF4 erase card. Erase and
//! program run when chip select drops; their completion raises the EXI interrupt if enabled. The status starts
//! as Dolphin's card: 0xC1 (busy, unlocked, ready), so the SDK skips the unlock handshake as it does there.

pub const SIZE_MBIT: u16 = 4;
pub const BLOCK: usize = 0x2000;

pub struct Card {
    pub data: Vec<u8>,
    status: u8,
    cmd: Option<u8>,
    pos: usize,
    addr: usize,
    buf: Vec<u8>,
    interrupt_on: bool,
    /// Bytes written to the card since the last save of the image.
    pub dirty: bool,
}

/// Unused card bytes read as 0xFF; GCI = 0x40-byte directory entry + the file's blocks.
pub fn build(gcis: &[Vec<u8>], flash_id: [u8; 12], format_time: u64, encoding: u16) -> Vec<u8> {
    let blocks = (SIZE_MBIT as usize) << 20 >> 3 >> 13; // Mbit -> bytes -> 8 KB blocks
    let mut img = vec![0xFFu8; blocks * BLOCK];
    // header: serial derived from the SRAM flash ID and the format time (the SDK's VerifyID), size, encoding
    {
        let h = &mut img[0..BLOCK];
        let mut rand = format_time;
        for i in 0..12 {
            rand = (rand.wrapping_mul(0x41C6_4E6D).wrapping_add(0x3039)) >> 16;
            h[i] = (rand as u8).wrapping_add(flash_id[i]);
            rand = ((rand.wrapping_mul(0x41C6_4E6D).wrapping_add(0x3039)) >> 16) & 0x7FFF;
        }
        h[0x0C..0x14].copy_from_slice(&format_time.to_be_bytes());
        h[0x14..0x20].fill(0);
        h[0x20..0x22].copy_from_slice(&0u16.to_be_bytes());
        h[0x22..0x24].copy_from_slice(&SIZE_MBIT.to_be_bytes());
        h[0x24..0x26].copy_from_slice(&encoding.to_be_bytes());
        let (a, b) = checksum(&h[..0x1FC]);
        h[0x1FC..0x1FE].copy_from_slice(&a.to_be_bytes());
        h[0x1FE..0x200].copy_from_slice(&b.to_be_bytes());
    }
    // directory and block allocation table (two copies each)
    let mut dir = vec![0xFFu8; BLOCK];
    let mut bat = vec![0u8; BLOCK];
    let mut next = 5u16;
    for (k, g) in gcis.iter().enumerate().take(127) {
        if g.len() < 0x40 { continue; }
        let mut e = g[..0x40].to_vec();
        let n = u16::from_be_bytes([e[0x38], e[0x39]]);
        e[0x36..0x38].copy_from_slice(&next.to_be_bytes());
        dir[k * 0x40..k * 0x40 + 0x40].copy_from_slice(&e);
        for b in 0..n {
            let blk = next + b;
            let link = if b + 1 == n { 0xFFFF } else { blk + 1 };
            let o = 0x0A + 2 * (blk as usize - 5);
            bat[o..o + 2].copy_from_slice(&link.to_be_bytes());
            let src = 0x40 + b as usize * BLOCK;
            let dst = blk as usize * BLOCK;
            if src + BLOCK <= g.len() && dst + BLOCK <= img.len() { img[dst..dst + BLOCK].copy_from_slice(&g[src..src + BLOCK]); }
        }
        next += n;
    }
    dir[0x1FFA..0x1FFC].copy_from_slice(&0u16.to_be_bytes());
    let (a, b) = checksum(&dir[..0x1FFC]);
    dir[0x1FFC..0x1FFE].copy_from_slice(&a.to_be_bytes());
    dir[0x1FFE..0x2000].copy_from_slice(&b.to_be_bytes());
    let user = blocks as u16 - 5;
    bat[0x04..0x06].copy_from_slice(&0u16.to_be_bytes());
    bat[0x06..0x08].copy_from_slice(&(user - (next - 5)).to_be_bytes());
    bat[0x08..0x0A].copy_from_slice(&(next - 1).max(4).to_be_bytes());
    let (a, b) = checksum(&bat[4..]);
    bat[0..2].copy_from_slice(&a.to_be_bytes());
    bat[2..4].copy_from_slice(&b.to_be_bytes());
    for (i, blk) in [(1, &dir), (2, &dir), (3, &bat), (4, &bat)] { img[i * BLOCK..(i + 1) * BLOCK].copy_from_slice(blk); }
    img
}

/// The SDK's `__CARDCheckSum`: u16 sum and sum of complements; 0xFFFF becomes 0.
pub fn checksum(d: &[u8]) -> (u16, u16) {
    let (mut a, mut b) = (0u16, 0u16);
    for w in d.chunks(2) {
        let v = u16::from_be_bytes([w[0], w.get(1).copied().unwrap_or(0)]);
        a = a.wrapping_add(v);
        b = b.wrapping_add(!v);
    }
    (if a == 0xFFFF { 0 } else { a }, if b == 0xFFFF { 0 } else { b })
}

impl Card {
    pub fn new(data: Vec<u8>) -> Card {
        Card { data, status: 0xC1, cmd: None, pos: 0, addr: 0, buf: Vec::new(), interrupt_on: false, dirty: false }
    }

    pub fn select(&mut self) { self.cmd = None; self.pos = 0; self.buf.clear(); }

    /// Chip select dropped: run an erase / program. Returns whether the EXI interrupt fires.
    pub fn deselect(&mut self) -> bool {
        let mut done = false;
        match self.cmd {
            Some(0xF1) if self.pos >= 3 => {
                let a = self.addr & !(BLOCK - 1);
                if a + BLOCK <= self.data.len() { self.data[a..a + BLOCK].fill(0xFF); self.dirty = true; }
                done = true;
            }
            Some(0xF2) if self.pos >= 5 => {
                let a = self.addr;
                for (k, &b) in self.buf.iter().enumerate() {
                    let o = (a & !0x7F) | ((a + k) & 0x7F);
                    if o < self.data.len() { self.data[o] = b; }
                }
                self.dirty = true;
                done = true;
            }
            Some(0xF4) => { self.data.fill(0xFF); self.dirty = true; self.status &= !0x80; }
            _ => {}
        }
        self.cmd = None;
        if done { self.status = (self.status | 0x01) & !0x80; return self.interrupt_on; }
        false
    }

    pub fn write(&mut self, x: u8) {
        let p = self.pos;
        self.pos += 1;
        let Some(cmd) = self.cmd else {
            self.cmd = Some(x);
            if x == 0x89 { self.status = (self.status & !0x18) | 0x01; }
            return;
        };
        match cmd {
            0x52 | 0xF2 => match p {
                1 => self.addr = ((x as usize) & 0x7F) << 17,
                2 => self.addr |= (x as usize) << 9,
                3 => self.addr |= ((x as usize) & 3) << 7,
                4 => self.addr |= (x as usize) & 0x7F,
                _ => if cmd == 0xF2 && self.buf.len() < 128 { self.buf.push(x); },
            },
            0xF1 => match p { 1 => self.addr = ((x as usize) & 0x7F) << 17, 2 => self.addr |= (x as usize) << 9, _ => {} },
            0x81 if p == 1 => self.interrupt_on = x != 0,
            _ => {}
        }
    }

    pub fn read(&mut self) -> u8 {
        let p = self.pos;
        self.pos += 1;
        match self.cmd {
            Some(0x00) => if p < 2 { 0x80 } else { (SIZE_MBIT as u32).to_be_bytes()[(p - 2) % 4] },
            Some(0x83) => self.status,
            Some(0x52) if p >= 5 => {
                let a = self.addr;
                self.addr = (a & !0x1FF) | ((a + 1) & 0x1FF);
                self.data.get(a).copied().unwrap_or(0xFF)
            }
            _ => 0xFF,
        }
    }
}
