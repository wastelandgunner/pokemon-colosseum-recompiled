//! Pokémon Colosseum's sound: the MusyX runtime (decompiled, AxioDL/musyx, built for its PC renderer) and
//! the game's own sound layer above it (`layer`), loaded from the disc's tables and banks.

pub mod engine;
pub mod layer;
pub mod sys;

use colo_formats::common::SoundTables;
use colo_formats::fsys::Fsys;

pub use layer::{SoundData, SoundLayer, SoundTask};

/// The game's boot order of wave table pushes: 1 (music) and 2 (se) by the sound thread, then 5, 4, 6
/// (boot step 0x800364C8), then 3, 7 (0x800363BC). Entry 0 duplicates 1 and is never pushed.
pub const BOOT_WAVES: [usize; 7] = [1, 2, 5, 4, 6, 3, 7];

/// FUN_80167118 for every bank: project, sample directory and pool from common.fsys (group 0xA) by FSYS
/// id, the `.samp` sample data from `samp(path)` (the game streams it from DVD through the upload callback).
/// Returns one line per problem.
pub fn push_banks(t: &SoundTables, common: &Fsys, mut samp: impl FnMut(&str) -> Option<Vec<u8>>) -> Vec<String> {
    let mut errs = vec![];
    let get = |id: u32| common.entries.iter().find(|e| e.identifier == id).and_then(|e| common.read(e).ok());
    for &i in &BOOT_WAVES {
        let Some(w) = t.waves.get(i) else { errs.push(format!("ERROR: can't open WAVE ID: {i}")); continue };
        let (Some(proj), Some(sdir), Some(pool)) = (get(w.proj), get(w.sdir), get(w.pool)) else {
            errs.push(format!("wave {i}: project/sdir/pool missing from common.fsys"));
            continue;
        };
        let mut sdir = sdir;
        for id in sanitize_sdir(&mut sdir) { errs.push(format!("wave {i}: sample {id}: loop start -1 (made one-shot)")); }
        let Some(samples) = samp(&w.samp_path) else { errs.push(format!("ERROR: can't open WAVE File: {}", w.samp_path)); continue };
        if let Some(why) = engine::validate_group(&proj, &pool, &sdir, &samples) { errs.push(format!("wave {i} ({}): {why}", w.samp_path)); }
        if !engine::push_group(proj, w.musyx_group, samples, sdir, pool) { errs.push(format!("wave {i}: sndPushGroup(group {}) failed", w.musyx_group)); }
    }
    errs
}

/// Every bgm_archive (group 0x99) entry by identifier, decompressed (FUN_80165F40 loads the archive at boot).
pub fn bgm_archive(fs: &Fsys) -> std::collections::HashMap<u32, Vec<u8>> {
    fs.entries.iter().filter_map(|e| Some((e.identifier, fs.read(e).ok()?))).collect()
}

/// Sample directory entries whose loop lies outside the sample (snd_music sample 581: loop start 0xFFFFFFFF,
/// length 1). The GameCube DSP would take the wrapped addresses as they are; the PC renderer rejects the whole
/// group, so these become one-shot samples (loop length 0). Returns the sample ids changed.
pub fn sanitize_sdir(sdir: &mut [u8]) -> Vec<u16> {
    let mut out = vec![];
    for e in sdir.chunks_exact_mut(32) {
        let id = u16::from_be_bytes([e[0], e[1]]);
        if id == 0xFFFF { break; }
        let be = |o: usize| u32::from_be_bytes([e[o], e[o + 1], e[o + 2], e[o + 3]]);
        let (len, lp, ll) = (be(16) & 0xFF_FFFF, be(20), be(24));
        if lp > len || ll > len - lp {
            e[20..28].fill(0);
            out.push(id);
        }
    }
    out
}
