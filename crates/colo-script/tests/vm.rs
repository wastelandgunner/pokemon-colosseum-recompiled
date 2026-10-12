//! VM tests on hand-assembled scripts (same encoding as the game's .scd files).
use colo_formats::scd::Script;
use colo_script::{Cell, Host, Memory, NativeCall, NativeResult, ThreadStatus, Vm};

struct Asm { code: Vec<u8> }
impl Asm {
    fn new() -> Self { Asm { code: vec![] } }
    fn at(&self) -> u32 { self.code.len() as u32 }
    fn imm(&mut self, v: i32) -> &mut Self { self.code.extend([8, 2]); self.code.extend((v as u32).to_be_bytes()); self }
    fn callstd(&mut self, argc: u16) -> &mut Self { self.code.extend([37, 0, 1]); self.code.extend(argc.to_be_bytes()); self }
    fn yld(&mut self) -> &mut Self { self.code.extend([36, 0, 1, 0, 1]); self }
    fn pop(&mut self, n: u16) -> &mut Self { self.code.extend([3, 0x82]); self.code.extend(n.to_be_bytes()); self }
    fn jf(&mut self, t: u32) -> &mut Self { self.code.push(7); self.code.extend(t.to_be_bytes()); self }
    fn jmp(&mut self, t: u32) -> &mut Self { self.code.push(6); self.code.extend(t.to_be_bytes()); self }
    fn ret(&mut self) -> &mut Self { self.code.push(5); self }
    /// lhs (pushed first) OP rhs (pushed second), both immediate ints on the stack
    fn bin(&mut self, op: u8) -> &mut Self { self.code.extend([op, 0x82, 0x82]); self }
    fn load_local(&mut self, idx: i32) -> &mut Self { self.imm(idx); self.code.extend([16, 0x42, 0, 1]); self }
    fn set_local(&mut self, idx: i32, v: i32) -> &mut Self { self.imm(idx); self.imm(v); self.code.extend([31, 0x82, 0x42]); self }
}

/// Build an .scd: header + function table + 4 data bytes + code. `funcs` are offsets into `code`.
fn script(funcs: &[u32], code: &[u8]) -> Script {
    let n = funcs.len();
    let table_end = 0x18 + 4 * n;
    let data = table_end + 4;
    let code_base = data + 4;
    let mut b = vec![0u8; code_base];
    b[0..2].copy_from_slice(&0x100u16.to_be_bytes());
    b[2..4].copy_from_slice(&0x101u16.to_be_bytes());
    b[4..6].copy_from_slice(&(n as u16).to_be_bytes());
    b[0xC..0x10].copy_from_slice(&(table_end as u32).to_be_bytes());
    b[0x10..0x14].copy_from_slice(&(data as u32).to_be_bytes());
    for (i, f) in funcs.iter().enumerate() {
        b[0x18 + 4 * i..0x1C + 4 * i].copy_from_slice(&(code_base as u32 + f).to_be_bytes());
    }
    // relocate absolute jump targets: tests write targets relative to `code`, so add code_base
    let mut c = code.to_vec();
    let mut pc = 0;
    while pc < c.len() {
        let (op, next) = colo_formats::scd::decode(&c, pc).unwrap();
        if matches!(op, colo_formats::scd::Op::Jump { .. } | colo_formats::scd::Op::JumpFalse { .. }) {
            let t = u32::from_be_bytes(c[pc + 1..pc + 5].try_into().unwrap()) + code_base as u32;
            c[pc + 1..pc + 5].copy_from_slice(&t.to_be_bytes());
        }
        pc = next;
    }
    b.extend(c);
    Script::parse(b).unwrap()
}

#[derive(Default)]
struct Rec { calls: Vec<(u16, Vec<Cell>)>, logs: Vec<String>, pending_left: u32 }
impl Host for Rec {
    fn call_native(&mut self, c: &NativeCall, _m: &mut dyn Memory) -> NativeResult {
        self.calls.push((c.index, c.args.to_vec()));
        match c.index {
            81 => NativeResult::Done(1), // flagCheckTrue -> true
            62 if self.pending_left > 0 => { self.pending_left -= 1; NativeResult::Pending }
            _ => NativeResult::Done(0),
        }
    }
    fn log(&mut self, l: &str) { self.logs.push(l.into()); }
}

#[test]
fn callstd_args_return_and_branch() {
    let mut a = Asm::new();
    // if (flagCheckTrue(2480)) { soundSetBackgroundMusic(7, 0, 64) } ; return
    a.imm(0).imm(81).imm(2480).callstd(2);
    let jf_at = a.at(); a.jf(0);
    a.imm(0).imm(104).imm(7).imm(0).imm(64).callstd(4).pop(1);
    let end = a.at(); a.imm(0).pop(1).ret();
    let mut code = a.code.clone();
    code[jf_at as usize + 1..jf_at as usize + 5].copy_from_slice(&end.to_be_bytes());
    let mut vm = Vm::new();
    vm.load(script(&[0], &code));
    let mut h = Rec::default();
    let s = vm.spawn(0x0100_0000, &[], &mut h).unwrap();
    vm.tick(&mut h);
    assert_eq!(h.calls, vec![(81, vec![2480]), (104, vec![7, 0, 64])]);
    assert!(matches!(vm.status(s), Some(ThreadStatus::Finished(_))));
    assert!(h.logs.is_empty(), "{:?}", h.logs);
}

#[test]
fn yield_waits_frames_and_pending_native_resumes() {
    let mut a = Asm::new();
    a.imm(3).yld();                                    // wait 3 frames
    a.imm(0).imm(62).imm(0).imm(5).imm(1).callstd(4).pop(1); // characterWaitForMovement: pending twice
    a.imm(0).imm(18).callstd(1).pop(1).imm(0).pop(1).ret();
    let mut vm = Vm::new();
    vm.load(script(&[0], &a.code));
    let mut h = Rec { pending_left: 2, ..Default::default() };
    let s = vm.spawn(0x0100_0000, &[], &mut h).unwrap();
    vm.tick(&mut h);
    assert_eq!(vm.status(s), Some(ThreadStatus::WaitingFrames(3)));
    vm.tick(&mut h); vm.tick(&mut h);
    assert!(h.calls.is_empty());
    vm.tick(&mut h); // yield done -> native 62 called, pending
    assert_eq!(vm.status(s), Some(ThreadStatus::WaitingNative(62)));
    vm.tick(&mut h); // still pending
    vm.tick(&mut h); // done -> std18 -> return
    assert_eq!(h.calls.iter().map(|c| c.0).collect::<Vec<_>>(), vec![62, 62, 62, 18]);
    assert!(matches!(vm.status(s), Some(ThreadStatus::Finished(_))));
    assert!(h.logs.is_empty(), "{:?}", h.logs);
}

#[test]
fn spawn_args_locals_arith_and_background_thread() {
    // func0(arg1, arg2, arg3): arg3 = arg1 - arg2 (locals are the arg slots; slot argc+1 holds the saved pc) ; std[3] spawn func1 with (local3) ; return
    let mut f0 = Asm::new();
    f0.imm(3);                              // dst index (local 3)
    f0.load_local(1).load_local(2);
    f0.code.extend([24, 0x82, 0x82]);       // sub: local1 - local2
    f0.code.extend([31, 0x82, 0x42]);       // setVar local[3] = result
    f0.pop(1);
    f0.imm(0).imm(3).imm(0x0100_0001).imm(0);
    f0.load_local(3);
    f0.imm(0).imm(0).imm(0).callstd(7).pop(1); // threadRunScriptInBackground(id, prio, a1..a4)
    f0.imm(0).pop(1).ret();
    let f1_at = f0.at();
    let mut f1 = Asm::new();
    f1.imm(0).imm(104).load_local(1).imm(0).imm(0).callstd(4).pop(1).imm(0).pop(1).ret();
    let mut code = f0.code.clone();
    code.extend(&f1.code);
    let mut vm = Vm::new();
    vm.load(script(&[0, f1_at], &code));
    let mut h = Rec::default();
    vm.spawn(0x0100_0000, &[10, 3, 0], &mut h).unwrap();
    vm.tick(&mut h);
    // background thread received 7 (=10-3) as its first argument and passed it to std104
    assert_eq!(h.calls.last(), Some(&(104, vec![7, 0, 0])), "{:?}", h.calls);
    assert!(vm.is_idle());
    assert!(h.logs.is_empty(), "{:?}", h.logs);
}

#[test]
fn lt_and_jump_loop_counts() {
    // i = 0; while (i < 3) { std[18](); i = i + 1 }
    let mut a = Asm::new();
    a.set_local(1, 0).pop(1);
    let top = a.at();
    a.load_local(1).imm(3).bin(25);
    let jf = a.at(); a.jf(0);
    a.imm(0).imm(18).callstd(1).pop(1);
    a.imm(1); a.load_local(1).imm(1).bin(23); a.code.extend([31, 0x82, 0x42]); a.pop(1);
    a.jmp(top);
    let end = a.at(); a.imm(0).pop(1).ret();
    let mut code = a.code.clone();
    code[jf as usize + 1..jf as usize + 5].copy_from_slice(&end.to_be_bytes());
    let mut vm = Vm::new();
    vm.load(script(&[0], &code));
    let mut h = Rec::default();
    vm.spawn(0x0100_0000, &[0, 0], &mut h).unwrap();
    vm.tick(&mut h);
    assert_eq!(h.calls.len(), 3);
    assert!(h.logs.is_empty(), "{:?}", h.logs);
}
