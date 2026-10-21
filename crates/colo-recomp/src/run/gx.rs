//! GX: the command processor fed from the write-gather pipe at `GXFlush` / `GXSetDrawDone`, over colo-gx.

use super::*;

pub const GX_SET_DRAW_DONE: u32 = 0x800B8D10;
pub(super) const IRQ_PE_FINISH: u32 = 0x13;

/// `GXSetDrawDone` 0x800B8D10: the original, then the draw-done interrupt is pending.
pub(super) fn gx_set_draw_done(c: &mut Cpu) {
    call_original(c, GX_SET_DRAW_DONE);
    // its token is in the pipe (the write-gather pipe drains on its own): the command processor takes it now
    gx_feed(&mut c.m);
}

/// The command processor: the pipe's bytes are parsed as the game flushes them (`GXFlush`), so register
/// state and memory references are taken when the GPU would read them; a PE draw-done token (BP 0x45) raises
/// the PE finish interrupt (the GPU is done at once).
pub(super) struct GpuStream { pub(super) gx: colo_gx::Gx, pub(super) frame: colo_gx::Frame, pub(super) buf: Vec<u8> }
pub(super) static GPU_IN: Mutex<Option<GpuStream>> = Mutex::new(None);
pub const GX_FLUSH: u32 = 0x800B8920;

/// Time spent decoding the GX stream (ns; diagnostics).
pub static FEED_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub(super) fn gx_feed(m: &mut crate::mem::Mem) {
    let t0 = std::time::Instant::now();
    let _t = Defer(t0);
    struct Defer(std::time::Instant);
    impl Drop for Defer { fn drop(&mut self) { FEED_NS.fetch_add(self.0.elapsed().as_nanos() as u64, std::sync::atomic::Ordering::Relaxed); } }
    let mut g = GPU_IN.lock().unwrap();
    let pipe = m.pipe.as_mut().unwrap();
    let Some(g) = g.as_mut() else { pipe.clear(); return };
    // the pipe's bytes move to the stream buffer (swapped when nothing is left over: no copy, and both keep
    // their capacity)
    if g.buf.is_empty() { std::mem::swap(&mut g.buf, pipe); } else { g.buf.extend_from_slice(pipe); pipe.clear(); }
    let before = g.frame.pe_done;
    let v = GpuView(m);
    let n = g.gx.run(&g.buf, &v, &mut g.frame, 0);
    g.buf.drain(..n);
    if g.frame.pe_done != before { PENDING.lock().unwrap().push(IRQ_PE_FINISH); }
}

pub(super) fn gx_flush(c: &mut Cpu) {
    call_original(c, GX_FLUSH);
    gx_feed(&mut c.m);
}

pub(super) struct GpuView<'a>(pub(super) &'a mut crate::mem::Mem);
impl colo_gx::GpuMem for GpuView<'_> {
    fn read(&self, a: u32, out: &mut [u8]) {
        let a = (a & 0x01FF_FFFF) | 0x8000_0000;
        let n = out.len() as u32;
        if a.checked_add(n).is_some_and(|e| e <= 0x8180_0000) { out.copy_from_slice(self.0.read(a, n)); } else { out.fill(0); }
    }
    fn slice(&self, a: u32, n: usize) -> Option<&[u8]> {
        let a = (a & 0x01FF_FFFF) | 0x8000_0000;
        a.checked_add(n as u32).is_some_and(|e| e <= 0x8180_0000).then(|| self.0.read(a, n as u32))
    }
}
impl colo_gx::raster::GpuMemMut for GpuView<'_> {
    fn write(&mut self, a: u32, d: &[u8]) {
        let a = (a & 0x01FF_FFFF) | 0x8000_0000;
        if a.checked_add(d.len() as u32).is_some_and(|e| e <= 0x8180_0000) { self.0.write(a, d); }
    }
}
