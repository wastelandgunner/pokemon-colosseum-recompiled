//! Builds the decompiled MusyX runtime (AxioDL/musyx, CC0) for its PC target: the same sequencer, synth,
//! sound macro interpreter, voice and reverb code the game runs, rendering through the PC mixer.
//! `musyx/` is a git submodule pinned to AxioDL/musyx @ 86851a3 (clone with `--recursive`).
use std::{env, fs, path::Path};

/// synthdata.c with the macro sub-table raised 2048 -> 4096, the game's build limit
/// (dataInsertMacro 0x80151B84: `if (0xfff < total) return 0`). Patched into OUT_DIR; the submodule stays untouched.
fn patched_synthdata(rt: &str) -> String {
    let src = fs::read_to_string(format!("{rt}/synthdata.c"))
        .expect("musyx submodule missing: run `git submodule update --init`");
    let mut s = src.clone();
    for (from, to, n) in [
        ("dataMacSubTabmem[2048]", "dataMacSubTabmem[4096]", 1),
        ("dataMacTotal < 2048", "dataMacTotal < 4096", 3),
        ("macros <= 2048u - dataMacTotal", "macros <= 4096u - dataMacTotal", 1),
    ] {
        assert_eq!(s.matches(from).count(), n, "synthdata.c patch: `{from}` not found as expected");
        s = s.replace(from, to);
    }
    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("synthdata.c");
    fs::write(&out, s).unwrap();
    out.to_string_lossy().into_owned()
}

fn main() {
    let rt = "musyx/src/musyx/runtime";
    let files = [
        "seq.c", "synth.c", "seq_api.c", "snd_synthapi.c", "stream.c", "synthmacros.c", "synthvoice.c",
        "synth_ac.c", "synth_adsr.c", "synth_vsamples.c", "synth_dbtab.c", "s_data.c", "hw_dspctrl.c", "hw_volconv.c",
        "snd3d.c", "snd_init.c", "snd_math.c", "snd_midictrl.c", "snd_service.c", "hardware.c", "hw_aramdma.c",
        "hw_dolphin.c", "hw_pc_output.c", "hw_pc_rate.c", "hw_pc_vsample.c", "hw_pc_song.c", "hw_pc_samples.c",
        "hw_pc_assets.c", "hw_pc_decode.c", "hw_pc.c", "hw_memory.c", "hw_lib_dolphin.c", "hw_pc_null.c",
        "CheapReverb/creverb_fx.c", "CheapReverb/creverb.c", "StdReverb/reverb_fx.c", "StdReverb/reverb.c",
        "Delay/delay_fx.c", "Chorus/chorus_fx.c",
    ];
    let mut b = cc::Build::new();
    b.include("musyx/include").include(rt)
        .define("MUSY_TARGET", "MUSY_TARGET_PC")
        .define("MUSY_VERSION_MAJOR", "2")
        .define("MUSY_VERSION_MINOR", "0")
        .define("MUSY_VERSION_PATCH", "0")
        .define("NDEBUG", None)
        .flag_if_supported("-std=gnu11")
        .define("_GNU_SOURCE", None)
        .warnings(false);
    for f in files { b.file(format!("{rt}/{f}")); println!("cargo:rerun-if-changed={rt}/{f}"); }
    b.file(patched_synthdata(rt));
    println!("cargo:rerun-if-changed={rt}/synthdata.c");
    b.file("shim.c");
    println!("cargo:rerun-if-changed=shim.c");
    b.compile("musyx");
    println!("cargo:rustc-link-lib=m");
}
