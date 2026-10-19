//! The DSP library's mailboxes and control register; the DSP itself runs AX (crate::dsp_ax).

use super::*;

pub(super) static DSPCR: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static RESET_PENDING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub(super) const DSPCR_W1C: u32 = 0xA8; // AIDINT 0x08, ARINT 0x20, DSPINT 0x80: written 1 = acknowledge

/// DSPCR (0xCC00500A) writes.
pub(super) fn control_write(m: &mut crate::mem::Mem, a: u32, n: usize) {
// DSP control: interrupt status bits are write-1-to-clear
if covers(a, n, 0xCC00_500A) || covers(a, n, 0xCC00_500B) {
    let w = m.u16(0xCC00_500A) as u32;
    let old = DSPCR.load(std::sync::atomic::Ordering::Relaxed);
    // RES (bit 0) clears itself once the DSP has reset
    // DMAState (bit 9) is the ARAM DMA's busy flag, read-only
    let v = (w & !DSPCR_W1C & !1 & !0x200) | (old & DSPCR_W1C & !w) | (old & 0x200);
    m.set_u16(0xCC00_500A, v);
    DSPCR.store(v, std::sync::atomic::Ordering::Relaxed);
    // DSPINIT (bit 11, DSPInit writes it 1 then 0): the DSP restarts in its ROM, which greets with a mail once it
    // runs (HALT, bit 2, clear: OSInit's __OSInitAudioSystem resets it halted and waits for the mailbox to be empty
    // first, then releases it and waits for the greeting)
    if w & 0x800 != 0 && old & 0x800 == 0 { RESET_PENDING.store(true, std::sync::atomic::Ordering::Relaxed); }
    if v & 4 == 0 && RESET_PENDING.swap(false, std::sync::atomic::Ordering::Relaxed) {
        let r = AX.lock().unwrap().get_or_insert_with(crate::dsp_ax::Ax::new).reset();
        m.set_u16(0xCC00_5004, r >> 16);
        m.set_u16(0xCC00_5006, r & 0xFFFF);
    }
}
}

pub const DSP_SEND_MAIL: u32 = 0x800AE7CC;
pub const DSP_READ_MAIL: u32 = 0x800AE7B4;

pub(super) static AX: Mutex<Option<crate::dsp_ax::Ax>> = Mutex::new(None);

/// `DSPSendMailToDSP` 0x800AE7CC: the original writes the CPU->DSP mailbox; the DSP (AX, dsp_ax.rs) takes the
/// mail at once (mailbox empty again) and may answer through the DSP->CPU mailbox with the DSP interrupt.
pub(super) fn dsp_send_mail(c: &mut Cpu) {
    let m = c.r[3];
    call_original(c, DSP_SEND_MAIL);
    let hi = c.m.u16(0xCC00_5000) as u32;
    c.m.set_u16(0xCC00_5000, hi & 0x7FFF);
    let reply = {
        let mut ax = AX.lock().unwrap();
        let ax = ax.get_or_insert_with(crate::dsp_ax::Ax::new);
        let mut g = ARAM.lock().unwrap();
        let a = g.get_or_insert_with(|| Aram { bytes: vec![0; 16 << 20], known: vec![false; 16 << 20], warned: false });
        ax.mail(c, &mut a.bytes, m)
    };
    if let Some(r) = reply {
        c.m.set_u16(0xCC00_5004, r >> 16);
        c.m.set_u16(0xCC00_5006, r & 0xFFFF);
        PENDING.lock().unwrap().push(0x07);
    }
}

/// `DSPReadMailFromDSP` 0x800AE7B4: reading the low half empties the DSP->CPU mailbox.
pub(super) fn dsp_read_mail(c: &mut Cpu) {
    call_original(c, DSP_READ_MAIL);
    let hi = c.m.u16(0xCC00_5004) as u32;
    c.m.set_u16(0xCC00_5004, hi & 0x7FFF);
}

/// The mixed output since the last call (32 kHz, interleaved R, L), for the host's audio.
pub fn take_audio() -> Vec<i16> { AX.lock().unwrap().as_mut().map(|a| std::mem::take(&mut a.out)).unwrap_or_default() }
