//! Power-on from the disc image alone: what the IPL and the disc's apploader leave in memory before the DOL's entry
//! point (`__start`), so the game's own `__start` (registers, `__init_data`, DBInit, OSInit) runs on lifted code up to
//! `main`. Everything comes from the user's disc: the header (boot.bin), bi2.bin, main.dol and fst.bin.
//!
//! Layout as an NTSC console leaves it (and as Dolphin's apploader boot does; checked against
//! `dolphin/boot_snap.py`'s MEM1 at `main`): the disc header's first 0x20 bytes at 0x80000000 (game ID ... DVD
//! magic), the boot magic 0x0D15EA5E, version 1, 24 MB of RAM, console type 0x10000006 (as Dolphin), the FST at the top of
//! MEM1 (32-byte aligned; its address is also the arena top), bi2 0x2000 below it, ARAM 16 MB, bus 162 MHz / CPU
//! 486 MHz. The rest of MEM1 is zero.

use super::*;

/// MEM1 as at the DOL's entry point, and that entry point (`__start` 0x80003154).
pub struct DiscBoot { pub mem1: Vec<u8>, pub entry: u32 }

fn be(b: &[u8], o: usize) -> u32 { u32::from_be_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) }

/// Read the boot files off the disc and lay out MEM1 as at the DOL's entry point.
pub fn load_disc(disc: &Disc) -> std::io::Result<DiscBoot> {
    let bad = |w: &str| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("disc: {w}"));
    let read = |off: u64, n: usize| { let mut b = vec![0u8; n]; disc(off, &mut b); b };
    let hdr = read(0, 0x440);
    if be(&hdr, 0x1C) != 0xC233_9F3D { return Err(bad("no GameCube disc magic at 0x1C")); }
    let (dol_off, fst_off, fst_size, fst_max) = (be(&hdr, 0x420) as u64, be(&hdr, 0x424) as u64, be(&hdr, 0x428), be(&hdr, 0x42C));
    let bi2 = read(0x440, 0x2000);

    let mut mem = vec![0u8; 0x0180_0000];
    let put = |mem: &mut Vec<u8>, a: u32, b: &[u8]| -> std::io::Result<()> {
        let o = (a & 0x01FF_FFFF) as usize;
        if a < 0x8000_0000 || o + b.len() > mem.len() { return Err(bad(&format!("section {a:#x}+{:#x} outside MEM1", b.len()))); }
        mem[o..o + b.len()].copy_from_slice(b);
        Ok(())
    };
    let w32 = |mem: &mut Vec<u8>, a: u32, v: u32| { let o = (a & 0x01FF_FFFF) as usize; mem[o..o + 4].copy_from_slice(&v.to_be_bytes()); };

    // main.dol: 7 text + 11 data sections (offset, address, size tables), bss, entry
    let dh = read(dol_off, 0x100);
    for k in 0..18 {
        let (off, addr, size) = (be(&dh, 4 * k), be(&dh, 0x48 + 4 * k), be(&dh, 0x90 + 4 * k));
        if size == 0 { continue; }
        put(&mut mem, addr, &read(dol_off + off as u64, size as usize))?;
    }
    let entry = be(&dh, 0xE0);

    // FST at the top of MEM1, bi2 below it
    let fst_at = (0x8180_0000 - fst_size) & !0x1F;
    put(&mut mem, fst_at, &read(fst_off, fst_size as usize))?;
    let bi2_at = fst_at - 0x2000;
    put(&mut mem, bi2_at, &bi2)?;

    // low memory (OSBootInfo 0x80000000, then the IPL's globals)
    put(&mut mem, 0x8000_0000, &hdr[..0x20])?;
    w32(&mut mem, 0x8000_0020, 0x0D15_EA5E); // boot magic: booted by the IPL
    w32(&mut mem, 0x8000_0024, 1); // version
    w32(&mut mem, 0x8000_0028, 0x0180_0000); // physical memory size
    w32(&mut mem, 0x8000_002C, 0x1000_0006); // console type: latest devkit (Dolphin's choice; the snapshot has it)
    w32(&mut mem, 0x8000_0034, fst_at); // arena high
    w32(&mut mem, 0x8000_0038, fst_at); // FST
    w32(&mut mem, 0x8000_003C, fst_max); // FST max size
    w32(&mut mem, 0x8000_00CC, 0); // TV mode: NTSC
    w32(&mut mem, 0x8000_00D0, 0x0100_0000); // ARAM size
    w32(&mut mem, 0x8000_00EC, 0x8180_0000); // debugger memory top
    w32(&mut mem, 0x8000_00F0, 0x0180_0000); // simulated memory size
    w32(&mut mem, 0x8000_00F4, bi2_at); // bi2
    w32(&mut mem, 0x8000_00F8, 162_000_000); // bus clock
    w32(&mut mem, 0x8000_00FC, 486_000_000); // CPU clock
    Ok(DiscBoot { mem1: mem, entry })
}

/// `RealMode(fn)` 0x8009F64C: `rfi` into `fn` (a physical address) with translation off; `fn` returns to RealMode's
/// caller with its own `rfi`. The lifter reads both `rfi`s as falling into the next function, so here it is a call.
pub const REAL_MODE: u32 = 0x8009_F64C;
pub(super) fn real_mode(c: &mut Cpu) {
    let (f, lr) = (c.r[3] | 0x8000_0000, c.lr);
    (c.dispatch)(c, f);
    c.lr = lr;
}

/// `Config24MB` 0x8009F54C / `Config48MB` 0x8009F5CC (OSInit's __OSInitMemoryProtection, through RealMode): the
/// BATs for the memory size. Memory here isn't translated through BATs: nothing to do.
pub const CONFIG_24MB: u32 = 0x8009_F54C;
pub const CONFIG_48MB: u32 = 0x8009_F5CC;
pub(super) fn config_bats(_c: &mut Cpu) {}
