//! The audio DSP running Nintendo's AX microcode, at the mailbox / command-list level (run.rs's DSP).
//!
//! The game's sound engine is MusyX, which on GameCube drives the AX ucode (its task, uploaded from 0x8036A520,
//! 0x19E0 bytes, hashes to 0x4E8A8B21 with Dolphin's HashEctor: the older AX, whose parameter blocks lack the
//! low-pass filter words). Protocol (`salCtrlDsp` 0x80164238): mail 0xBABE0000 | list length (u16 words), then
//! the list's address; the DSP runs the list and answers 0xDCD10002 (task yield) with an interrupt, which
//! `__DSPHandler` answers with 0xCDD10003 and the task's resume callback (MusyX: frame done).
//!
//! Behaviour follows the hardware ucode as documented by Dolphin's AX HLE (AX.cpp, AXVoice.h, DSPAccelerator.cpp;
//! described, not copied): commands 0x00-0x13, parameter blocks walked through `next`, 5 x 1 ms of 32 samples per
//! list, the accelerator's ADPCM/PCM decoding with its end/loop rules, sample-rate conversion, volume envelope,
//! ramped mixing and dpop. The polyphase coefficients are in the DSP's DROM (not in the game): without them the
//! ucode's polyphase type falls back to linear, which only changes the output samples, never voice state.

use crate::ppc::Cpu;

pub const MAIL_CMDLIST: u32 = 0xBABE_0000;
pub const DSP_YIELD: u32 = 0xDCD1_0002;
const TASK_MAIL_TO_DSP: u32 = 0xCDD1_0000;

/// PB size in memory (words) for this ucode: the 122-word struct minus the 4 LPF words.
const PB_WORDS: usize = 118;
/// Word offset of the LPF block in the full struct (absent in memory here).
const LPF_OFF: usize = 93;

// full-struct word offsets
const NEXT_HI: usize = 0;
const SRC_TYPE: usize = 4;
const COEF_SELECT: usize = 5;
const MIXER_CONTROL: usize = 6;
const RUNNING: usize = 7;
const IS_STREAM: usize = 8;
const MIXER: usize = 9; // 9 x (volume, delta): mainL, mainR, auxAL, auxAR, auxBL, auxBR, auxBS, mainS, auxAS
const UPDATES: usize = 34; // num_updates[5], data_hi, data_lo
const DPOP: usize = 41; // mainL, auxAL, auxBL, mainR, auxAR, auxBR, mainS, auxAS, auxBS
const VOL_ENV: usize = 50; // cur_volume, cur_volume_delta
const AUDIO: usize = 55; // looping, format, loop hi/lo, end hi/lo, cur hi/lo
const ADPCM: usize = 63; // coefs[16], gain, pred_scale, yn1, yn2
const SRC: usize = 83; // ratio hi/lo, cur_addr_frac, last_samples[4]
const LOOP_INFO: usize = 90; // pred_scale, yn1, yn2
const LOOP_COUNTER: usize = 97;
const FULL_WORDS: usize = 122;

#[derive(PartialEq, Eq, Clone, Copy)]
/// Rom: the bootloader taking `__DSP_boot_task`'s 10 mails. Switch: `__DSP_exec_task`'s 10 mails after
/// 0xCDD10001 (save the current task's DRAM: 3; the next task's IRAM: 3; then init vector + 3 zeros for a new task,
/// or resume vector + DRAM restore 3). Card: the memory card unlock ucode.
enum MailState { Size, Address, NextTask, Rom(u32), Switch(u32), Card }

/// The AX task's IRAM image (MusyX's DSP task, salInitDspCtrl): any other task is the CARD unlock ucode.
const AX_IRAM: u32 = 0x8036_A520;

pub struct Ax {
    state: MailState,
    /// Mails of the boot / switch being received.
    task_mails: [u32; 10],
    list_len: u32,
    /// 9 mixing buffers of 5 ms x 32 samples: main L/R/S, auxA L/R/S, auxB L/R/S.
    bufs: [[i32; 160]; 9],
    pb_addr: u32,
    /// DROM polyphase coefficients (0x800 s16), when the host has them.
    pub coefs: Option<Vec<i16>>,
    /// What the AI DMA played since the host last took it (R, L per frame; run.rs `ai_due` fills it).
    pub out: Vec<i16>,
}

const ML: usize = 0; const MR: usize = 1; const MS: usize = 2;
const AL: usize = 3; const AR: usize = 4; const AS: usize = 5;
const BL: usize = 6; const BR: usize = 7; const BS: usize = 8;

fn phys(a: u32) -> u32 { (a & 0x01FF_FFFF) | 0x8000_0000 }
fn sat16(v: i64) -> i16 { v.clamp(-32768, 32767) as i16 }

impl Default for Ax { fn default() -> Self { Ax::new() } }

impl Ax {
    pub fn new() -> Ax {
        Ax { state: MailState::Size, task_mails: [0; 10], list_len: 0, bufs: [[0; 160]; 9], pb_addr: 0, coefs: None, out: Vec::new() }
    }

    /// A mail from the CPU (`DSPSendMailToDSP`). Returns the mail the DSP answers with, if any.
    /// DSP reset (DSPCR RES): the ROM's bootloader takes the task upload (`__DSP_boot_task`: ten mails,
    /// 0x80F3A001 IRAM MMEM address, 0x80F3C002 IRAM address, 0x80F3A002 length, 0x80F3B002 DRAM, 0x80F3D001
    /// start vector) and starts the ucode. Returns its first mail: 0x8071FEED.
    pub fn reset(&mut self) -> u32 {
        self.state = MailState::Rom(0);
        0x8071_FEED
    }

    pub fn mail(&mut self, c: &mut Cpu, aram: &mut [u8], m: u32) -> Option<u32> {
        match self.state {
            MailState::Rom(n) => {
                self.task_mails[n as usize] = m;
                if n + 1 < 10 { self.state = MailState::Rom(n + 1); return None; }
                // boot: 0x80F3A001, IRAM MMEM address, ...: the task starts (DSP_INIT)
                self.state = if self.task_mails[1] == AX_IRAM { MailState::Size } else { MailState::Card };
                Some(0xDCD1_0000)
            }
            MailState::Switch(n) => {
                self.task_mails[n as usize] = m;
                if n + 1 < 10 { self.state = MailState::Switch(n + 1); return None; }
                let iram = self.task_mails[3];
                let resume = self.task_mails[7] != 0 || self.task_mails[8] != 0 || self.task_mails[9] != 0;
                self.state = if iram == AX_IRAM { MailState::Size } else { MailState::Card };
                Some(if resume { 0xDCD1_0001 } else { 0xDCD1_0000 })
            }
            MailState::Card => {
                // the unlock ucode: 0xFF000000 (unlock) -> DSP_DONE, then the DSP is back in its ROM
                self.state = MailState::NextTask;
                Some(0xDCD1_0003)
            }
            MailState::Size => {
                if m & 0xFFFF_0000 == MAIL_CMDLIST { self.list_len = m & 0xFFFF; self.state = MailState::Address; }
                else { return self.task_mail(m); }
                None
            }
            MailState::Address => {
                let list: Vec<u16> = (0..self.list_len).map(|k| c.m.u16(phys(m) + 2 * k)).collect();
                self.run_list(c, aram, list);
                self.state = MailState::NextTask;
                Some(DSP_YIELD)
            }
            MailState::NextTask => self.task_mail(m),
        }
    }

    /// The task manager's mails (TASK_MAIL_TO_DSP 0xCDD1xxxx): 1 switch to another task (10 mails follow),
    /// 2 reset to the ROM (greeting 0x8071FEED), 3 continue.
    fn task_mail(&mut self, m: u32) -> Option<u32> {
        match m | TASK_MAIL_TO_DSP {
            0xCDD1_0001 => { self.state = MailState::Switch(0); None }
            0xCDD1_0002 => Some(self.reset()),
            _ => { self.state = MailState::Size; None }
        }
    }

    fn run_list(&mut self, c: &mut Cpu, aram: &mut [u8], mut list: Vec<u16>) {
        let mut i = 0usize;
        let w = |l: &Vec<u16>, i: &mut usize| -> u16 { let v = l.get(*i).copied().unwrap_or(0); *i += 1; v };
        let a32 = |l: &Vec<u16>, i: &mut usize| -> u32 { let h = w(l, i) as u32; let lo = w(l, i) as u32; h << 16 | lo };
        loop {
            let cmd = w(&list, &mut i);
            match cmd {
                0x00 => { let a = a32(&list, &mut i); self.setup(c, a); }
                0x01 => {
                    let a = a32(&list, &mut i);
                    let vols = [w(&list, &mut i), w(&list, &mut i), w(&list, &mut i)];
                    for g in 0..3 {
                        for ch in 0..3 {
                            for k in 0..160 {
                                let s = c.m.u32(phys(a) + 4 * (((g * 3 + ch) * 160 + k) as u32)) as i32;
                                self.bufs[g * 3 + ch][k] += ((s as i64 * vols[g] as i16 as i64) >> 15) as i32;
                            }
                        }
                    }
                }
                0x02 => { self.pb_addr = a32(&list, &mut i); }
                0x03 => self.process_pbs(c, aram),
                0x04 | 0x05 => {
                    let wa = a32(&list, &mut i);
                    let ra = a32(&list, &mut i);
                    self.mix_aux(c, (cmd - 4) as usize, wa, ra);
                }
                0x06 => { let a = a32(&list, &mut i); self.upload(c, a, &[ML, MR, MS]); }
                0x07 => {
                    let a = a32(&list, &mut i);
                    for k in 0..160 { let s = c.m.u32(phys(a) + 4 * k as u32) as i32; self.bufs[ML][k] = s; self.bufs[MR][k] = s; self.bufs[MS][k] = 0; }
                }
                0x08 => { i += 10; }
                0x09 => { let a = a32(&list, &mut i); self.mix_aux(c, 1, 0, a); }
                0x0A | 0x0B | 0x0C => {}
                0x0D => {
                    let a = a32(&list, &mut i);
                    let n = w(&list, &mut i) as u32;
                    list = (0..n).map(|k| c.m.u16(phys(a) + 2 * k)).collect();
                    i = 0;
                }
                0x0E => {
                    let sa = a32(&list, &mut i);
                    let lr = a32(&list, &mut i);
                    self.upload(c, sa, &[MS]);
                    for k in 0..160 {
                        let r = sat16(self.bufs[MR][k] as i64);
                        let l = sat16(self.bufs[ML][k] as i64);
                        c.m.set_u16(phys(lr) + 4 * k as u32, r as u16 as u32);
                        c.m.set_u16(phys(lr) + 4 * k as u32 + 2, l as u16 as u32);
                    }
                }
                0x0F => break,
                0x10 => {
                    let ua = a32(&list, &mut i);
                    let da = a32(&list, &mut i);
                    self.upload(c, ua, &[BL, BR]);
                    for k in 0..160 {
                        let l = c.m.u32(phys(da) + 4 * k as u32) as i32;
                        let r = c.m.u32(phys(da) + 640 + 4 * k as u32) as i32;
                        self.bufs[BL][k] = l; self.bufs[BR][k] = r;
                        self.bufs[ML][k] += l; self.bufs[MR][k] += r;
                    }
                }
                0x11 => {
                    let a = a32(&list, &mut i);
                    for k in 0..160 { let s = c.m.u32(phys(a) + 4 * k as u32) as i32; self.bufs[ML][k] = -s; self.bufs[MR][k] = s; self.bufs[MS][k] = 0; }
                }
                0x13 => {
                    let auxa = a32(&list, &mut i);
                    let auxbs = a32(&list, &mut i);
                    let dl = [a32(&list, &mut i), a32(&list, &mut i), a32(&list, &mut i), a32(&list, &mut i)];
                    self.upload(c, auxa, &[AL, AR, AS]);
                    self.upload(c, auxbs, &[BS]);
                    for (n, b) in [ML, MR, BL, BR].into_iter().enumerate() {
                        for k in 0..160 { self.bufs[b][k] += c.m.u32(phys(dl[n]) + 4 * k as u32) as i32; }
                    }
                }
                // 0x12 (compressor) does not exist in this ucode
                x => panic!("AX: command {x:#04x} not in ucode 0x4E8A8B21"),
            }
        }
    }

    /// CMD_SETUP: each of the 9 buffers starts from (value hi, value lo, delta) and ramps by delta per sample.
    fn setup(&mut self, c: &mut Cpu, a: u32) {
        for b in 0..9 {
            let base = phys(a) + 6 * b as u32;
            let init = ((c.m.u16(base) as u32) << 16 | c.m.u16(base + 2) as u32) as i32;
            let delta = c.m.u16(base + 4) as i16 as i32;
            for k in 0..160 { self.bufs[b][k] = if init == 0 { 0 } else { init.wrapping_add(k as i32 * delta) }; }
        }
    }

    fn upload(&self, c: &mut Cpu, a: u32, which: &[usize]) {
        for (n, &b) in which.iter().enumerate() {
            for k in 0..160 { c.m.set_u32(phys(a) + (n * 640 + 4 * k) as u32, self.bufs[b][k] as u32); }
        }
    }

    fn mix_aux(&mut self, c: &mut Cpu, aux: usize, wa: u32, ra: u32) {
        let (l, r, s) = if aux == 0 { (AL, AR, AS) } else { (BL, BR, BS) };
        if wa != 0 { self.upload(c, wa, &[l, r, s]); }
        for (n, b) in [ML, MR, MS].into_iter().enumerate() {
            for k in 0..160 { self.bufs[b][k] += c.m.u32(phys(ra) + (n * 640 + 4 * k) as u32) as i32; }
        }
    }

    /// Old ucode (0x4E8A8B21) mixer control: main L/R always; 0x1 aux A, 0x2 aux B, 0x4 main surround, 0x8 all
    /// ramps; 0x10 = Dolby Pro Logic II (surround to every bus).
    fn mix_flags(mc: u16) -> u32 {
        let mut f = 0x1 | 0x4; // MAIN_L | MAIN_R
        if mc & 0x10 != 0 {
            f |= 0x10; // MAIN_S
            if mc & 1 != 0 { f |= 0x40 | 0x100 | 0x400; }
            if mc & 2 != 0 { f |= 0x1000 | 0x4000 | 0x10000; }
        } else {
            if mc & 1 != 0 { f |= 0x40 | 0x100; }
            if mc & 2 != 0 { f |= 0x1000 | 0x4000; }
            if mc & 4 != 0 { f |= 0x10; if mc & 1 != 0 { f |= 0x400; } if mc & 2 != 0 { f |= 0x10000; } }
        }
        if mc & 8 != 0 { f |= 0xAAAAAA; }
        f
    }

    fn process_pbs(&mut self, c: &mut Cpu, aram: &mut [u8]) {
        let mut a = self.pb_addr;
        while a != 0 {
            let mut pb = [0u16; FULL_WORDS];
            // memory: words 0..LPF_OFF, then the rest from loop_counter on (no LPF words)
            for k in 0..LPF_OFF { pb[k] = c.m.u16(phys(a) + 2 * k as u32); }
            for k in LPF_OFF..PB_WORDS { pb[k + 4] = c.m.u16(phys(a) + 2 * k as u32); }
            let updates = self.load_updates(c, &pb);
            for ms in 0..5 {
                apply_updates(&mut pb, &updates, ms);
                self.process_voice(&mut pb, aram, ms);
            }
            for k in 0..LPF_OFF { c.m.set_u16(phys(a) + 2 * k as u32, pb[k] as u32); }
            for k in LPF_OFF..PB_WORDS { c.m.set_u16(phys(a) + 2 * k as u32, pb[k + 4] as u32); }
            a = (pb[NEXT_HI] as u32) << 16 | pb[NEXT_HI + 1] as u32;
        }
    }

    fn load_updates(&self, c: &mut Cpu, pb: &[u16; FULL_WORDS]) -> Vec<(u16, u16)> {
        let n: usize = (0..5).map(|k| pb[UPDATES + k] as usize).sum();
        let d = (pb[UPDATES + 5] as u32) << 16 | pb[UPDATES + 6] as u32;
        (0..n).map(|k| (c.m.u16(phys(d) + 4 * k as u32), c.m.u16(phys(d) + 4 * k as u32 + 2))).collect()
    }

    fn process_voice(&mut self, pb: &mut [u16; FULL_WORDS], aram: &mut [u8], ms: usize) {
        if pb[RUNNING] != 1 { return; }
        let mut samples = [0i16; 32];
        get_input_samples(pb, aram, &mut samples, self.coefs.as_deref());
        // volume envelope (signed on GC)
        let mut vol = pb[VOL_ENV] as i16 as i32;
        let dv = pb[VOL_ENV + 1] as i16 as i32;
        for s in samples.iter_mut() {
            *s = sat16((*s as i64 * vol as i64) >> 15);
            vol = vol.wrapping_add(dv);
        }
        pb[VOL_ENV] = vol as u16;
        let f = Ax::mix_flags(pb[MIXER_CONTROL]);
        let off = ms * 32;
        // (flag bit, buffer, mixer slot, dpop slot)
        const CH: [(u32, usize, usize, usize); 9] = [
            (0x1, ML, 0, 0), (0x4, MR, 1, 3), (0x10, MS, 7, 6),
            (0x40, AL, 2, 1), (0x100, AR, 3, 4), (0x400, AS, 8, 7),
            (0x1000, BL, 4, 2), (0x4000, BR, 5, 5), (0x10000, BS, 6, 8),
        ];
        for (bit, buf, mix, dp) in CH {
            if f & bit == 0 { continue; }
            let ramp = f & (bit << 1) != 0;
            let mut v = pb[MIXER + 2 * mix] as i16 as i32;
            let d = if ramp { pb[MIXER + 2 * mix + 1] as i16 as i32 } else { 0 };
            let mut last = 0i16;
            for k in 0..32 {
                let m = sat16((samples[k] as i64 * v as i64) >> 15);
                self.bufs[buf][off + k] += m as i32;
                last = m;
                v = v.wrapping_add(d);
            }
            pb[MIXER + 2 * mix] = v as u16;
            pb[DPOP + dp] = last as u16;
        }
    }
}

fn apply_updates(pb: &mut [u16; FULL_WORDS], up: &[(u16, u16)], ms: usize) {
    let start: usize = (0..ms).map(|k| pb[UPDATES + k] as usize).sum();
    let n = pb[UPDATES + ms] as usize;
    if start + n > up.len() { return; }
    for &(o, v) in &up[start..start + n] {
        let o = o as usize;
        if o < FULL_WORDS { pb[o] = v; }
    }
}

/// The DSP accelerator for one voice over one millisecond.
struct Acc<'a> {
    aram: &'a [u8],
    start: u32, end: u32, cur: u32,
    fmt: u16, pred_scale: u16, yn1: i16, yn2: i16, gain: i16,
    stopped: bool,
}

impl Acc<'_> {
    fn byte(&self, a: u32) -> u8 { self.aram.get(a as usize & 0x00FF_FFFF).copied().unwrap_or(0) }

    fn current(&self) -> u16 {
        match self.fmt & 3 {
            0 => { let b = self.byte(self.cur >> 1); (if self.cur & 1 != 0 { b & 0xF } else { b >> 4 }) as u16 }
            1 => self.byte(self.cur) as u16,
            2 => (self.byte(self.cur * 2) as u16) << 8 | self.byte(self.cur * 2 + 1) as u16,
            _ => 0,
        }
    }

    /// One sample; `end` is true when the end of the sample was reached (the end exception).
    fn read(&mut self, coefs: &[i16; 16]) -> (i16, bool) {
        if self.stopped { return (0, false); }
        let raw = self.current() as i16 as i32;
        let ci = ((self.pred_scale >> 4) & 7) as usize;
        let (c1, c2) = (coefs[2 * ci] as i32, coefs[2 * ci + 1] as i32);
        let step: u32;
        let val: i16;
        if (self.fmt >> 2) & 3 == 0 {
            let mut n = raw & 0xF;
            if n >= 8 { n -= 16; }
            let scale = 1i32 << (self.pred_scale & 0xF);
            let pred = (0x400 + c1 * self.yn1 as i32 + c2 * self.yn2 as i32) >> 11;
            val = sat16((scale * n + pred) as i64);
            self.yn2 = self.yn1; self.yn1 = val;
            self.cur = self.cur.wrapping_add(1);
            let mut st = 2;
            if self.end & 0xF == 0 && self.cur == self.end {
                self.cur = self.start.wrapping_add(1);
            } else if self.end & 0xF == 1 && self.cur == self.end.wrapping_sub(1) {
                self.cur = self.start;
            } else if self.cur & 0xF == 0 {
                self.pred_scale = self.byte((self.cur & !0xF) >> 1) as u16;
                self.cur = self.cur.wrapping_add(2);
                st += 2;
            }
            step = st;
        } else {
            let sh = match (self.fmt >> 4) & 3 { 0 => 11, 1 => 0, 2 => 16, _ => 0 };
            let v = ((self.gain as i32 * raw) >> sh) + ((c1 * self.yn1 as i32) >> sh) + ((c2 * self.yn2 as i32) >> sh);
            val = v as i16;
            self.yn2 = self.yn1; self.yn1 = val;
            step = 2;
            self.cur = self.cur.wrapping_add(1);
        }
        let mut end = false;
        if self.cur == self.end.wrapping_add(step - 1) {
            self.cur = self.start;
            self.stopped = true;
            end = true;
        }
        self.cur &= 0xBFFF_FFFF;
        (val, end)
    }
}

/// `GetInputSamples`: 32 output samples through the accelerator and the sample-rate converter; the end
/// exception (loop reload or `running = 0`) is applied to the PB as it happens.
fn get_input_samples(pb: &mut [u16; FULL_WORDS], aram: &[u8], out: &mut [i16; 32], coefs: Option<&[i16]>) {
    let h = |i: usize, pb: &[u16; FULL_WORDS]| (pb[i] as u32) << 16 | pb[i + 1] as u32;
    let mut acc = Acc {
        aram,
        start: h(AUDIO + 2, pb) & 0x3FFF_FFFF, end: h(AUDIO + 4, pb) & 0x3FFF_FFFF, cur: h(AUDIO + 6, pb) & 0xBFFF_FFFF,
        fmt: pb[AUDIO + 1], gain: pb[ADPCM + 16] as i16, pred_scale: pb[ADPCM + 17] & 0x7F,
        yn1: pb[ADPCM + 18] as i16, yn2: pb[ADPCM + 19] as i16, stopped: false,
    };
    let mut ac = [0i16; 16];
    for k in 0..16 { ac[k] = pb[ADPCM + k] as i16; }
    let coefs = coefs.map(|c| &c[(pb[COEF_SELECT] as usize * 0x200).min(c.len())..]);
    let mut get = |pb: &mut [u16; FULL_WORDS], acc: &mut Acc| -> i16 {
        let (v, end) = acc.read(&ac);
        if end {
            if pb[AUDIO] != 0 {
                acc.pred_scale = pb[LOOP_INFO] & 0x7F;
                if pb[IS_STREAM] != 1 {
                    acc.yn1 = pb[LOOP_INFO + 1] as i16;
                    acc.yn2 = pb[LOOP_INFO + 2] as i16;
                } else {
                    pb[LOOP_COUNTER] = pb[LOOP_COUNTER].wrapping_add(1);
                }
                // setting yn2 restarts reads
                acc.stopped = false;
            } else {
                pb[RUNNING] = 0;
            }
        }
        v
    };
    let ratio = h(SRC, pb);
    let mut pos = pb[SRC + 2] as u32;
    let mut last = [0i16; 4];
    for k in 0..4 { last[k] = pb[SRC + 3 + k] as i16; }
    let st = pb[SRC_TYPE];
    if st == 0 || st == 1 {
        let mut t = last;
        let mut idx = 4usize;
        for o in out.iter_mut() {
            pos = pos.wrapping_add(ratio);
            while pos >= 0x10000 {
                t[idx & 3] = get(pb, &mut acc);
                idx += 1;
                pos -= 0x10000;
            }
            if st == 0 && coefs.is_some() {
                let c = &coefs.unwrap()[(((pos & 0xFFFF) >> 9) << 2) as usize..];
                let s = t[idx & 3] as i64 * c[0] as i64 + t[(idx + 1) & 3] as i64 * c[1] as i64
                    + t[(idx + 2) & 3] as i64 * c[2] as i64 + t[(idx + 3) & 3] as i64 * c[3] as i64;
                *o = sat16(s >> 15);
            } else {
                let frac = pos & 0xFFFF;
                let inv = (frac.wrapping_neg()) & 0xFFFF;
                *o = if frac != 0 {
                    ((t[idx & 3] as i32 * inv as i32 + t[(idx + 1) & 3] as i32 * frac as i32) >> 16) as i16
                } else { t[idx & 3] };
            }
            idx += 4;
        }
        for k in 0..4 { last[3 - k] = t[(idx - 1 - k) & 3]; }
    } else {
        for o in out.iter_mut() { *o = get(pb, &mut acc); }
        last.copy_from_slice(&out[28..32]);
    }
    for k in 0..4 { pb[SRC + 3 + k] = last[k] as u16; }
    pb[SRC + 2] = (pos & 0xFFFF) as u16;
    pb[AUDIO + 6] = (acc.cur >> 16) as u16;
    pb[AUDIO + 7] = acc.cur as u16;
    pb[ADPCM + 18] = acc.yn1 as u16;
    pb[ADPCM + 19] = acc.yn2 as u16;
    pb[ADPCM + 17] = acc.pred_scale;
}
