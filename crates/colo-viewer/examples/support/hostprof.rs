//! Shared by the examples (`#[path = "support/hostprof.rs"] mod hostprof;`).
/// HOSTPROF: the host program counter at each SIGPROF (process CPU time), kept while `ON`.
#[cfg(target_os = "linux")]
pub mod imp {
    use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU64, Ordering::Relaxed};
    pub static ON: AtomicBool = AtomicBool::new(false);
    const N: usize = 1 << 20;
    static PCS: [AtomicU64; N] = [const { AtomicU64::new(0) }; N];
    static LEN: AtomicUsize = AtomicUsize::new(0);

    extern "C" fn on_prof(_: libc::c_int, _: *mut libc::siginfo_t, ctx: *mut libc::c_void) {
        if !ON.load(Relaxed) { return; }
        let pc = unsafe { (*(ctx as *const libc::ucontext_t)).uc_mcontext.gregs[libc::REG_RIP as usize] } as u64;
        let i = LEN.fetch_add(1, Relaxed);
        if i < N { PCS[i].store(pc, Relaxed); }
    }

    pub fn start() {
        unsafe {
            let mut sa: libc::sigaction = std::mem::zeroed();
            sa.sa_sigaction = on_prof as usize;
            sa.sa_flags = libc::SA_SIGINFO | libc::SA_RESTART;
            libc::sigaction(libc::SIGPROF, &sa, std::ptr::null_mut());
            let t = libc::timeval { tv_sec: 0, tv_usec: 100 };
            let it = libc::itimerval { it_interval: t, it_value: t };
            libc::setitimer(libc::ITIMER_PROF, &it, std::ptr::null_mut());
        }
    }

    /// Write the samples as offsets into the executable (its mapping from /proc/self/maps).
    pub fn dump() {
        let exe = std::fs::read_link("/proc/self/exe").unwrap_or_default();
        let maps = std::fs::read_to_string("/proc/self/maps").unwrap_or_default();
        let segs: Vec<(u64, u64, u64)> = maps.lines().filter(|l| l.ends_with(exe.to_str().unwrap_or("?"))).filter_map(|l| {
            let f: Vec<&str> = l.split_whitespace().collect();
            let (a, b) = f[0].split_once('-')?;
            Some((u64::from_str_radix(a, 16).ok()?, u64::from_str_radix(b, 16).ok()?, u64::from_str_radix(f[2], 16).ok()?))
        }).collect();
        let n = LEN.load(Relaxed).min(N);
        let mut out = String::new();
        for p in PCS[..n].iter().map(|p| p.load(Relaxed)) {
            match segs.iter().find(|s| p >= s.0 && p < s.1) {
                Some(s) => out += &format!("{:x}\n", p - s.0 + s.2),
                // outside the executable: "? <mapped file>+<offset>"
                None => match maps.lines().find(|l| l.split_once('-').and_then(|(a, r)| Some((u64::from_str_radix(a, 16).ok()? <= p) && p < u64::from_str_radix(r.split_whitespace().next()?, 16).ok()?)).unwrap_or(false)) {
                    Some(l) => { let f: Vec<&str> = l.split_whitespace().collect(); let a = u64::from_str_radix(f[0].split('-').next().unwrap(), 16).unwrap();
                        out += &format!("? {}+{:x}\n", f.get(5).unwrap_or(&"anon"), p - a + u64::from_str_radix(f[2], 16).unwrap_or(0)) }
                    None => out += "?\n",
                },
            }
        }
        std::fs::write(std::env::var("HOSTPROF_OUT").unwrap_or("/tmp/sb_hostprof.txt".into()), out).ok();
        eprintln!("hostprof: {n} samples");
    }
}
#[cfg(not(target_os = "linux"))]
pub mod imp {
    pub static ON: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    pub fn start() {}
    pub fn dump() {}
}
pub use imp::*;
