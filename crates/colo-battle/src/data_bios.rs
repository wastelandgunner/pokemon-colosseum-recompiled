//! Generated from the Ghidra decompiles of the game's data-table `*DataBios` getters and `*GetPtr`s
//! (scratch `gen_bios.py` + `emit_bios.py`, 2026-10-12). Names, offsets, null checks, index bounds and clamps
//! are the original's. Functions with more logic are ported by hand in `pokemon.rs`.
#![allow(non_snake_case, unused_variables, clippy::all)]

use crate::mem::Mem;

/// 0x801fcba4
pub fn FUN_801fcba4(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x10))
}
/// 0x801fcbbc
pub fn FUN_801fcbbc(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xff >= 4 { return 0x0; }
    m.u32(p.wrapping_add((i & 0xff).wrapping_mul(4)).wrapping_add(0x24))
}
/// 0x801fcbf0
pub fn FUN_801fcbf0(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0xc))
}
/// 0x801fcc08
pub fn FUN_801fcc08(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xff >= 8 { return 0x0; }
    m.u16(p.wrapping_add((i & 0xff).wrapping_mul(2)).wrapping_add(0x14)) as u32
}
/// 0x801fcc3c
pub fn FUN_801fcc3c(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x4)) as u32
}
/// 0x801fcc7c
pub fn FUN_801fcc7c(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x8))
}
/// 0x801fcc94
pub fn FUN_801fcc94(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x6)) as u32
}
/// 0x801fccac
pub fn FUN_801fccac(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x8020d79c
pub fn fightActionKindDataBiosGetDispFuncPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x8))
}
/// 0x8020d7b4
pub fn fightActionKindDataBiosGetFlowFuncPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x4))
}
/// 0x8020d7cc
pub fn fightActionKindDataBiosGetPri(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0xffffff80; }
    m.i8(p) as i32 as u32
}
/// 0x8020d7e8
pub fn fightActionKindDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(0x80478d48) <= i { return 0; }
    0x80375bb8 + i * 0xc
}
/// 0x8020e008
pub fn fightEncountDataBiosGetBgmSndId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0xc))
}
/// 0x8020e0b0
pub fn fightEncountDataBiosGetFightFloorDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x4)) as u32
}
/// 0x8020e0e0
pub fn fightEncountDataBiosGetFightKind(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x8020ded8
pub fn fightEncountDataBiosGetFightName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x8))
}
/// 0x8020e068
pub fn fightEncountDataBiosGetFightTrainerDataId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xff >= 4 { return 0x0; }
    m.u16(p.wrapping_add((i & 0xff).wrapping_mul(8)).wrapping_add(0x18)) as u32
}
/// 0x8020e020
pub fn fightEncountDataBiosGetGSInputDevice(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xff >= 4 { return 0x0; }
    m.u32(p.wrapping_add((i & 0xff).wrapping_mul(8)).wrapping_add(0x1c))
}
/// 0x8020e0f8
pub fn fightEncountDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478f50)) <= i { return 0; }
    m.u32(0x80478f54).wrapping_add(i * 0x38)
}
/// 0x8020dfc0
pub fn fightEncountDataBiosGetSyoukaiWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x14))
}
/// 0x8020e0c8
pub fn fightEncountDataBiosGetTrainer(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x1)) as u32
}
/// 0x8020de50
pub fn fightEncountDataBiosGetWipeEffectSndID(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x2)) as u32
}
/// 0x8020de98
pub fn fightEncountDataBiosGetWipeFunction(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x8))
}
/// 0x8020dff0
pub fn fightEncountDataBiosGetWipeId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x10))
}
/// 0x8020de80
pub fn fightEncountDataBiosGetWipeSnapshotUse(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x8020dfd8
pub fn fightEncountDataBiosGetZenmetuFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x2)) as u32
}
/// 0x801f66a4
pub fn fightFloorDataBiosGetBgmSndId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0xc))
}
/// 0x801f668c
pub fn fightFloorDataBiosGetEnvSndId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x10))
}
/// 0x801f66ec
pub fn fightFloorDataBiosGetFightSideDataId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xff >= 2 { return 0x0; }
    m.u16(p.wrapping_add((i & 0xff).wrapping_mul(2)).wrapping_add(0x14)) as u32
}
/// 0x801f6720
pub fn fightFloorDataBiosGetFloorDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x2)) as u32
}
/// 0x801f6600
pub fn fightFloorDataBiosGetName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x4))
}
/// 0x801f6738
pub fn fightFloorDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478f48)) <= i { return 0; }
    m.u32(0x80478f4c).wrapping_add(i * 0x18)
}
/// 0x801f66bc
pub fn fightFloorDataBiosGetSyoukaiWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x8))
}
/// 0x801f66d4
pub fn fightFloorDataBiosGetTikeiDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x8020e4b4
pub fn fightKindDataBiosGetBackSaveDataFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x8020e2c0
pub fn fightKindDataBiosGetBossFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x13)) as u32
}
/// 0x8020e410
pub fn fightKindDataBiosGetCallFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x5)) as u32
}
/// 0x8020e278
pub fn fightKindDataBiosGetDarkpokemonHypermodeFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x16)) as u32
}
/// 0x8020e470
pub fn fightKindDataBiosGetDoBadgeCheckFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x1)) as u32
}
/// 0x8020e338
pub fn fightKindDataBiosGetDoCriticalAttackFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xe)) as u32
}
/// 0x8020e320
pub fn fightKindDataBiosGetDoHizukiAiFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xf)) as u32
}
/// 0x8020e308
pub fn fightKindDataBiosGetDoHizukiMiyaburiFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x10)) as u32
}
/// 0x8020e2f0
pub fn fightKindDataBiosGetDoItemSoubiTokukoutokubouupFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x11)) as u32
}
/// 0x8020e458
pub fn fightKindDataBiosGetDoZukanMitaFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x2)) as u32
}
/// 0x8020e440
pub fn fightKindDataBiosGetDoZukanTukamaetaFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x3)) as u32
}
/// 0x8020e2a8
pub fn fightKindDataBiosGetDorobouFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x14)) as u32
}
/// 0x8020e3e0
pub fn fightKindDataBiosGetDrawFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x7)) as u32
}
/// 0x8020e3c8
pub fn fightKindDataBiosGetGetExpFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x8)) as u32
}
/// 0x8020e368
pub fn fightKindDataBiosGetGetFriendFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc)) as u32
}
/// 0x8020e350
pub fn fightKindDataBiosGetGetInfectPokerusFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xd)) as u32
}
/// 0x8020e380
pub fn fightKindDataBiosGetGetNekoniKobanFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xb)) as u32
}
/// 0x8020e398
pub fn fightKindDataBiosGetGetOkaneFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x9)) as u32
}
/// 0x8020e230
pub fn fightKindDataBiosGetHostEnemyMsgFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x18)) as u32
}
/// 0x8020e2d8
pub fn fightKindDataBiosGetKeikentihueruFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x12)) as u32
}
/// 0x8020e290
pub fn fightKindDataBiosGetMonohiroiFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x15)) as u32
}
/// 0x8020e248
pub fn fightKindDataBiosGetName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x1c))
}
/// 0x8020e3f8
pub fn fightKindDataBiosGetNigeruFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x6)) as u32
}
/// 0x8020e3b0
pub fn fightKindDataBiosGetOkanePoolFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xa)) as u32
}
/// 0x8020e260
pub fn fightKindDataBiosGetPokemonStatusMenuSubbarFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x17)) as u32
}
/// 0x8020e488
pub fn fightKindDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478f40)) < i { return 0; }
    m.u32(0x80478f44).wrapping_add(i * 0x20)
}
/// 0x8020e428
pub fn fightKindDataBiosGetUseItemFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x4)) as u32
}
/// 0x8020d9d0
pub fn fightKoukaDataBiosGetFightJoukenDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p) as u32
}
/// 0x8020d9b8
pub fn fightKoukaDataBiosGetFightTargetDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x2)) as u32
}
/// 0x8020d9a0
pub fn fightKoukaDataBiosGetKoukaDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x4)) as u32
}
/// 0x8020d9e8
pub fn fightKoukaDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(0x80478d50) <= i { return 0; }
    0x80375cb8 + i * 0x6
}
/// 0x801f7824
pub fn fightSideDataBiosGetFightTrainerStatusMenuDataId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xff >= 2 { return 0x0; }
    m.u32(p.wrapping_add((i & 0xff).wrapping_mul(4)).wrapping_add(0x4))
}
/// 0x801f77f0
pub fn fightSideDataBiosGetFightoutPokemonStatusMenuDataId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xff >= 2 { return 0x0; }
    m.u32(p.wrapping_add((i & 0xff).wrapping_mul(4)).wrapping_add(0xc))
}
/// 0x801f7870
pub fn fightSideDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478f38)) <= i { return 0; }
    m.u32(0x80478f3c).wrapping_add(i * 0x14)
}
/// 0x801f7858
pub fn fightSideDataBiosGetYrot(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p) as u32
}
/// 0x801f0204
pub fn fightTargetDataBiosGetBuff(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x4))
}
/// 0x801f0234
pub fn fightTargetDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(0x80478d40) <= i { return 0; }
    0x80375ac8 + i * 0x8
}
/// 0x801f021c
pub fn fightTargetDataBiosGetStatusKid(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x801fc430
pub fn fightTrainerAiDataBiosGetAbicntMaxValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xf)) as u32
}
/// 0x801fc418
pub fn fightTrainerAiDataBiosGetAbicntMinValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x10)) as u32
}
/// 0x801fc5b0
pub fn fightTrainerAiDataBiosGetComboValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x6)) as u32
}
/// 0x801fc400
pub fn fightTrainerAiDataBiosGetDefensePokemonRndSelectFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x11)) as u32
}
/// 0x801fc598
pub fn fightTrainerAiDataBiosGetIrekaeValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x7)) as u32
}
/// 0x801fc580
pub fn fightTrainerAiDataBiosGetItemValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x8)) as u32
}
/// 0x801fc5e0
pub fn fightTrainerAiDataBiosGetKeyPlayerFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x4)) as u32
}
/// 0x801fc298
pub fn fightTrainerAiDataBiosGetLastValueRevise(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x19)) as u32
}
/// 0x801fc460
pub fn fightTrainerAiDataBiosGetNokoriHpValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xd)) as u32
}
/// 0x801fc568
pub fn fightTrainerAiDataBiosGetParamExpectFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x9)) as u32
}
/// 0x801fc550
pub fn fightTrainerAiDataBiosGetParamStoreFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xa)) as u32
}
/// 0x801fc3b8
pub fn fightTrainerAiDataBiosGetPartFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x14)) as u32
}
/// 0x801fc5f8
pub fn fightTrainerAiDataBiosGetPokemonDataOrderAceBossFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x3)) as u32
}
/// 0x801fc610
pub fn fightTrainerAiDataBiosGetPokemonDataOrderOutFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x2)) as u32
}
/// 0x801fc5c8
pub fn fightTrainerAiDataBiosGetPokemonJoutaiFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x5)) as u32
}
/// 0x801fc640
pub fn fightTrainerAiDataBiosGetPokemonSelectRandomFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x801fc628
pub fn fightTrainerAiDataBiosGetPokemonSelectWeakPointFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x1)) as u32
}
/// 0x801fc658
pub fn fightTrainerAiDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478f30)) <= i { return 0; }
    m.u32(0x80478f34).wrapping_add(i * 0x28)
}
/// 0x801fc478
pub fn fightTrainerAiDataBiosGetTokuseiCheckFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc)) as u32
}
/// 0x801fc2e0
pub fn fightTrainerAiDataBiosGetWazaAvgValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x16)) as u32
}
/// 0x801fc448
pub fn fightTrainerAiDataBiosGetWazaDamageFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xe)) as u32
}
/// 0x801fc2f8
pub fn fightTrainerAiDataBiosGetWazaHitFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x15)) as u32
}
/// 0x801fc3d0
pub fn fightTrainerAiDataBiosGetWazaInitValueFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x13)) as u32
}
/// 0x801fc2b0
pub fn fightTrainerAiDataBiosGetWazaNokoriPpValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x18)) as u32
}
/// 0x801fc2c8
pub fn fightTrainerAiDataBiosGetWazaRiskFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x17)) as u32
}
/// 0x801fc3e8
pub fn fightTrainerAiDataBiosGetWazaRndSelectFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x12)) as u32
}
/// 0x801fc538
pub fn fightTrainerAiDataBiosGetZokuseiCheckFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xb)) as u32
}
/// 0x801fbf74
pub fn fightTrainerAiValueAddsubDataBiosGetKoudouName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x4))
}
/// 0x801fbf8c
pub fn fightTrainerAiValueAddsubDataBiosGetName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0xc))
}
/// 0x801fbf5c
pub fn fightTrainerAiValueAddsubDataBiosGetPrefixName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x8))
}
/// 0x801fbfbc
pub fn fightTrainerAiValueAddsubDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478f28)) <= i { return 0; }
    m.u32(0x80478f2c).wrapping_add(i * 0x14)
}
/// 0x801fbfa4
pub fn fightTrainerAiValueAddsubDataBiosGetValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p)
}
/// 0x801fcc64
pub fn fightTrainerDataBiosGetKindDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x2)) as u32
}
/// 0x801fccc4
pub fn fightTrainerDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478f20)) <= i { return 0; }
    m.u32(0x80478f24).wrapping_add(i * 0x34)
}
/// 0x801fbd10
pub fn fightTrainerKindDataBiosGetBgmSndId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x8))
}
/// 0x801fbd28
pub fn fightTrainerKindDataBiosGetPrefixName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x4))
}
/// 0x801fbd58
pub fn fightTrainerKindDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478f18)) <= i { return 0; }
    m.u32(0x80478f1c).wrapping_add(i * 0xc)
}
/// 0x801fbd40
pub fn fightTrainerKindDataBiosGetSyoukinBairitu(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p) as u32
}
/// 0x801fc964
pub fn fightTrainerPokemonDataBiosGetDarkPokemonFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x3)) as u32
}
/// 0x801fc870
pub fn fightTrainerPokemonDataBiosGetFriend(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0xffffffff; }
    m.i16(p.wrapping_add(0x8)) as i32 as u32
}
/// 0x801fc97c
pub fn fightTrainerPokemonDataBiosGetItemBallId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xc)) as u32
}
/// 0x801fc918
pub fn fightTrainerPokemonDataBiosGetItemDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0xffffffff; }
    m.u32(p.wrapping_add(0x10))
}
/// 0x801fc6bc
pub fn fightTrainerPokemonDataBiosGetKeyPlayerFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x5)) as u32
}
/// 0x801fc994
pub fn fightTrainerPokemonDataBiosGetLevel(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x4)) as u32
}
/// 0x801fca14
pub fn fightTrainerPokemonDataBiosGetNickname(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x14))
}
/// 0x801fc6a4
pub fn fightTrainerPokemonDataBiosGetPartDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x6)) as u32
}
/// 0x801fc930
pub fn fightTrainerPokemonDataBiosGetPokemonDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xa)) as u32
}
/// 0x801fc888
pub fn fightTrainerPokemonDataBiosGetPpCnt(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xff >= 4 { return 0x0; }
    m.u8(p.wrapping_add((i & 0xff).wrapping_mul(8)).wrapping_add(0x30)) as u32
}
/// 0x801fca2c
pub fn fightTrainerPokemonDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478f10)) <= i { return 0; }
    m.u32(0x80478f14).wrapping_add(i * 0x50)
}
/// 0x801fc838
pub fn fightTrainerPokemonDataBiosGetSeikakuDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0xffffffff; }
    m.i8(p.wrapping_add(0x2)) as i32 as u32
}
/// 0x801fc854
pub fn fightTrainerPokemonDataBiosGetSexDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0xffffffff; }
    m.i8(p.wrapping_add(0x1)) as i32 as u32
}
/// 0x801fc9ac
pub fn fightTrainerPokemonDataBiosGetStatusEffort(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0xffffffff; }
    if i & 0xff >= 6 { return 0xffffffff; }
    m.i16(p.wrapping_add((i & 0xff).wrapping_mul(2)).wrapping_add(0x22)) as i32 as u32
}
/// 0x801fc9e0
pub fn fightTrainerPokemonDataBiosGetStatusRnd(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0xffffffff; }
    if i & 0xff >= 6 { return 0xffffffff; }
    m.i8(p.wrapping_add((i & 0xff).wrapping_mul(1)).wrapping_add(0x1c)) as i32 as u32
}
/// 0x801fc948
pub fn fightTrainerPokemonDataBiosGetTokuseiFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0xffffffff; }
    m.i8(p) as i32 as u32
}
/// 0x801fc8d0
pub fn fightTrainerPokemonDataBiosGetWazaDataId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0xffffffff; }
    if i & 0xff >= 4 { return 0xffffffff; }
    m.u32(p.wrapping_add((i & 0xff).wrapping_mul(8)).wrapping_add(0x34))
}
/// 0x801fcab8
pub fn fightTrainerPokemonPartDataBiosGetName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p)
}
/// 0x801fcad0
pub fn fightTrainerPokemonPartDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478f08)) <= i { return 0; }
    m.u32(0x80478f0c).wrapping_add(i * 0x14)
}
/// 0x801fca88
pub fn fightTrainerPokemonPartDataBiosGetWazaTypeRevise(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xff >= 12 { return 0x0; }
    m.u8(p.wrapping_add((i & 0xff).wrapping_mul(1)).wrapping_add(0x8)) as u32
}
/// 0x8020e1bc
pub fn fightTypeDataBiosGetEntryPokemonNum(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x1)) as u32
}
/// 0x8020e1a4
pub fn fightTypeDataBiosGetFightoutPokemonNum(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x2)) as u32
}
/// 0x8020e1ec
pub fn fightTypeDataBiosGetName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x4))
}
/// 0x8020e204
pub fn fightTypeDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478f00)) < i { return 0; }
    m.u32(0x80478f04).wrapping_add(i * 0x8)
}
/// 0x8020e1d4
pub fn fightTypeDataBiosGetTrainerNum(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x8020a068
pub fn fightWazaCriticalDataBiosGetBunbo(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x8020a080
pub fn fightWazaCriticalDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(0x80478d60) <= i { return 0; }
    0x80478d58 + i * 0x1
}
/// 0x8020a028
pub fn fightWazaHitKakurituDataBiosGetKake(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x1; }
    m.u8(p) as u32
}
/// 0x8020a040
pub fn fightWazaHitKakurituDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(0x80478d70) <= i { return 0; }
    0x80375dd0 + i * 0x2
}
/// 0x8020a010
pub fn fightWazaHitKakurituDataBiosGetWaru(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x1; }
    m.u8(p.wrapping_add(0x1)) as u32
}
/// 0x80143d28
pub fn itemBallDataBiosGetDowninWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x10))
}
/// 0x80143d88
pub fn itemBallDataBiosGetFightKoukaDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p) as u32
}
/// 0x80143d70
pub fn itemBallDataBiosGetInWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x4))
}
/// 0x80143d58
pub fn itemBallDataBiosGetOpenWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x8))
}
/// 0x80143d40
pub fn itemBallDataBiosGetOutWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0xc))
}
/// 0x80143da0
pub fn itemBallDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(0x80478bb8) <= i { return 0; }
    0x80367af0 + i * 0x30
}
/// 0x80143cf8
pub fn itemBallDataBiosGetSnatchAttackWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x18))
}
/// 0x80143ce0
pub fn itemBallDataBiosGetSnatchBalllandWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x1c))
}
/// 0x80143cc8
pub fn itemBallDataBiosGetSnatchMissWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x20))
}
/// 0x80143cb0
pub fn itemBallDataBiosGetSnatchPokeoutWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x24))
}
/// 0x80143c98
pub fn itemBallDataBiosGetSnatchShakeWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x28))
}
/// 0x80143c80
pub fn itemBallDataBiosGetSnatchSnatchWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x2c))
}
/// 0x80143d10
pub fn itemBallDataBiosGetThrowWzxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x14))
}
/// 0x80143dcc
pub fn itemDataBiosGetBattleUseFunc(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x20))
}
/// 0x80143e14
pub fn itemDataBiosGetBuff(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x18))
}
/// 0x80143fe4
pub fn itemDataBiosGetCoupon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x8)) as u32
}
/// 0x80143f84
pub fn itemDataBiosGetDoc(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x14))
}
/// 0x80143de4
pub fn itemDataBiosGetFieldUseFunc(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x1c))
}
/// 0x80143f54
pub fn itemDataBiosGetFightUseKoukaDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xc)) as u32
}
/// 0x80143dfc
pub fn itemDataBiosGetItemEffectParam(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x4)) as u32
}
/// 0x80143f6c
pub fn itemDataBiosGetItemSoubiDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xa)) as u32
}
/// 0x80144014
pub fn itemDataBiosGetKind(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x80144088
pub fn itemDataBiosGetName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x10))
}
/// 0x80143ffc
pub fn itemDataBiosGetPrice(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x6)) as u32
}
/// 0x80143e2c
pub fn itemDataBiosGetUseFriend(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 3 { return 0x0; }
    m.i8(p.wrapping_add((i & 0xffff).wrapping_mul(1)).wrapping_add(0x24)) as i32 as u32
}
/// 0x80143b30
pub fn itemSoubiDataBiosGetFightKoukaDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p) as u32
}
/// 0x80143b48
pub fn itemSoubiDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(0x80478bc0) <= i { return 0; }
    0x80367ef0 + i * 0x2
}
/// 0x8011e734
pub fn pokemonDataBiosGetBasisMaxHp(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x84)) as u32
}
/// 0x8011e658
pub fn pokemonDataBiosGetBasisNimbleness(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x8e)) as u32
}
/// 0x8011e708
pub fn pokemonDataBiosGetBasisPhyAtk(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x86)) as u32
}
/// 0x8011e6dc
pub fn pokemonDataBiosGetBasisPhyDef(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x88)) as u32
}
/// 0x8011e6b0
pub fn pokemonDataBiosGetBasisSpeAtk(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x8a)) as u32
}
/// 0x8011e684
pub fn pokemonDataBiosGetBasisSpeDef(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x8c)) as u32
}
/// 0x8011e1d4
pub fn pokemonDataBiosGetColor(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 2 { return 0x0; }
    m.u8(p.wrapping_add((i & 0xffff).wrapping_mul(8)).wrapping_add(0x10c)) as u32
}
/// 0x8011e174
pub fn pokemonDataBiosGetDoc(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x20))
}
/// 0x8011e508
pub fn pokemonDataBiosGetGet(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x1)) as u32
}
/// 0x8011e2dc
pub fn pokemonDataBiosGetGetWazaDataId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 20 { return 0x0; }
    m.u16(p.wrapping_add((i & 0xffff).wrapping_mul(4)).wrapping_add(0xbc)) as u32
}
/// 0x8011e324
pub fn pokemonDataBiosGetGetWazaLevel(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 20 { return 0x0; }
    m.u8(p.wrapping_add((i & 0xffff).wrapping_mul(4)).wrapping_add(0xba)) as u32
}
/// 0x8011e538
pub fn pokemonDataBiosGetGiveExp(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x6)) as u32
}
/// 0x8011e62c
pub fn pokemonDataBiosGetGiveMaxHpEffort(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x90)) as u32
}
/// 0x8011e550
pub fn pokemonDataBiosGetGiveNimblenessEffort(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x9a)) as u32
}
/// 0x8011e600
pub fn pokemonDataBiosGetGivePhyAtkEffort(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x92)) as u32
}
/// 0x8011e5d4
pub fn pokemonDataBiosGetGivePhyDefEffort(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x94)) as u32
}
/// 0x8011e5a8
pub fn pokemonDataBiosGetGiveSpeAtkEffort(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x96)) as u32
}
/// 0x8011e57c
pub fn pokemonDataBiosGetGiveSpeDefEffort(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x98)) as u32
}
/// 0x8011e520
pub fn pokemonDataBiosGetGrowDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x8011e1a4
pub fn pokemonDataBiosGetHeight(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xa)) as u32
}
/// 0x8011e4d8
pub fn pokemonDataBiosGetInitFriend(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x8)) as u32
}
/// 0x8011e4a4
pub fn pokemonDataBiosGetItemDataId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 2 { return 0x0; }
    m.u16(p.wrapping_add((i & 0xffff).wrapping_mul(2)).wrapping_add(0x70)) as u32
}
/// 0x8011e078
pub fn pokemonDataBiosGetKowaza(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 8 { return 0x0; }
    m.u16(p.wrapping_add((i & 0xffff).wrapping_mul(2)).wrapping_add(0x74)) as u32
}
/// 0x8011e760
pub fn pokemonDataBiosGetName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x18))
}
/// 0x8011e0c4
pub fn pokemonDataBiosGetNumPokemon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x12)) as u32
}
/// 0x8011e0dc
pub fn pokemonDataBiosGetNumZukan(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x14)) as u32
}
/// 0x8011e0ac
pub fn pokemonDataBiosGetPkxDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x2c))
}
/// 0x8011e264
pub fn pokemonDataBiosGetPokebodyId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 2 { return 0x0; }
    m.u32(p.wrapping_add((i & 0xffff).wrapping_mul(8)).wrapping_add(0x110))
}
/// 0x8011e778
pub fn pokemonDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478f90)) <= i { return 0; }
    m.u32(0x80478f94).wrapping_add(i * 0x11c)
}
/// 0x8011e4f0
pub fn pokemonDataBiosGetSexRatio(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x2)) as u32
}
/// 0x8011e3b4
pub fn pokemonDataBiosGetSinkaBuff(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 5 { return 0x0; }
    m.u16(p.wrapping_add((i & 0xffff).wrapping_mul(6)).wrapping_add(0x9e)) as u32
}
/// 0x8011e3fc
pub fn pokemonDataBiosGetSinkaKind(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 5 { return 0x0; }
    m.u8(p.wrapping_add((i & 0xffff).wrapping_mul(6)).wrapping_add(0x9c)) as u32
}
/// 0x8011e36c
pub fn pokemonDataBiosGetSinkaPokemonDataId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 5 { return 0x0; }
    m.u16(p.wrapping_add((i & 0xffff).wrapping_mul(6)).wrapping_add(0xa0)) as u32
}
/// 0x8011e21c
pub fn pokemonDataBiosGetStatusFaceMenuSpriteId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 2 { return 0x0; }
    m.u16(p.wrapping_add((i & 0xffff).wrapping_mul(8)).wrapping_add(0x10e)) as u32
}
/// 0x8011e444
pub fn pokemonDataBiosGetTokuseiDataId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 2 { return 0x0; }
    m.u8(p.wrapping_add((i & 0xffff).wrapping_mul(1)).wrapping_add(0x32)) as u32
}
/// 0x8011e1bc
pub fn pokemonDataBiosGetTypeName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x1c))
}
/// 0x8011e15c
pub fn pokemonDataBiosGetVoice(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xe)) as u32
}
/// 0x8011e2ac
pub fn pokemonDataBiosGetWazaMcn(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 58 { return 0x0; }
    m.u8(p.wrapping_add((i & 0xffff).wrapping_mul(1)).wrapping_add(0x34)) as u32
}
/// 0x8011e18c
pub fn pokemonDataBiosGetWeight(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xc)) as u32
}
/// 0x8011e474
pub fn pokemonDataBiosGetZokuseiDataId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 2 { return 0x0; }
    m.u8(p.wrapping_add((i & 0xffff).wrapping_mul(1)).wrapping_add(0x30)) as u32
}
/// 0x8011cab8
pub fn pokemonDpFilterDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478e68)) <= i { return 0; }
    m.u32(0x80478e6c).wrapping_add(i * 0x1)
}
/// 0x8011ca9c
pub fn pokemonDpFilterDataBiosGetValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i8(p) as i32 as u32
}
/// 0x8011cb10
pub fn pokemonFriendFilterDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(0x80478b70) <= i { return 0; }
    0x8035f988 + i * 0x3
}
/// 0x8011cae0
pub fn pokemonFriendFilterDataBiosGetValue(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xff >= 3 { return 0x0; }
    m.i8(p.wrapping_add((i & 0xff).wrapping_mul(1)).wrapping_add(0x0)) as i32 as u32
}
/// 0x8011ce44
pub fn pokemonGrowDataBiosGetExp(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xff >= 101 { return 0x0; }
    m.u32(p.wrapping_add((i & 0xff).wrapping_mul(4)).wrapping_add(0x0))
}
/// 0x8011ce74
pub fn pokemonGrowDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xff;
    if m.u32(0x80478b60) <= i { return 0; }
    0x8035e940 + i * 0x194
}
/// 0x8011ce00
pub fn pokemonSeikakuDataBiosGetName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x14))
}
/// 0x8011cd88
pub fn pokemonSeikakuDataBiosGetNimblenessRateDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x9)) as u32
}
/// 0x8011cde8
pub fn pokemonSeikakuDataBiosGetPhyAtkRateDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x5)) as u32
}
/// 0x8011cdd0
pub fn pokemonSeikakuDataBiosGetPhyDefRateDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x6)) as u32
}
/// 0x8011ce18
pub fn pokemonSeikakuDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xff;
    if m.u32(m.u32(0x80478e60)) <= i { return 0; }
    m.u32(0x80478e64).wrapping_add(i * 0x28)
}
/// 0x8011cd34
pub fn pokemonSeikakuDataBiosGetReliveCall(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i8(p.wrapping_add(0x2)) as i32 as u32
}
/// 0x8011cd6c
pub fn pokemonSeikakuDataBiosGetReliveFightout(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i8(p) as i32 as u32
}
/// 0x8011ccfc
pub fn pokemonSeikakuDataBiosGetReliveNadenade(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i8(p.wrapping_add(0x4)) as i32 as u32
}
/// 0x8011cd18
pub fn pokemonSeikakuDataBiosGetReliveSodateya(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i8(p.wrapping_add(0x3)) as i32 as u32
}
/// 0x8011cd50
pub fn pokemonSeikakuDataBiosGetReliveWalk(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i8(p.wrapping_add(0x1)) as i32 as u32
}
/// 0x8011cdb8
pub fn pokemonSeikakuDataBiosGetSpeAtkRateDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x7)) as u32
}
/// 0x8011cda0
pub fn pokemonSeikakuDataBiosGetSpeDefRateDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x8)) as u32
}
/// 0x8011cbb0
pub fn pokemonSeikakuRateDataBiosGetKake(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x8011cbc8
pub fn pokemonSeikakuRateDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xff;
    if m.u32(m.u32(0x80478e58)) <= i { return 0; }
    m.u32(0x80478e5c).wrapping_add(i * 0x2)
}
/// 0x8011cb98
pub fn pokemonSeikakuRateDataBiosGetWaru(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x1)) as u32
}
/// 0x8011cb3c
pub fn pokemonTokuseiDataBiosGetDoc(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x8))
}
/// 0x8011cb54
pub fn pokemonTokuseiDataBiosGetName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x4))
}
/// 0x8011cb6c
pub fn pokemonTokuseiDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(0x80478b68) <= i { return 0; }
    0x8035f5e0 + i * 0xc
}
/// 0x8011c910
pub fn wazaDataBiosGetAddFightKoukaAvg(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x5)) as u32
}
/// 0x8011c9a0
pub fn wazaDataBiosGetAvg(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x4)) as u32
}
/// 0x8011c808
pub fn wazaDataBiosGetBouon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x10)) as u32
}
/// 0x8011c8f8
pub fn wazaDataBiosGetDageki(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x6)) as u32
}
/// 0x8011c7c0
pub fn wazaDataBiosGetDoc(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x2c))
}
/// 0x8011c928
pub fn wazaDataBiosGetFightAttackMsgId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x28))
}
/// 0x8011c940
pub fn wazaDataBiosGetFightAttackTunagiMsgId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x24))
}
/// 0x8011c970
pub fn wazaDataBiosGetFightKoukaDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x18)) as u32
}
/// 0x8011c5c8
pub fn wazaDataBiosGetHidenFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x12)) as u32
}
/// 0x8011c988
pub fn wazaDataBiosGetIryoku(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i16(p.wrapping_add(0x16)) as i32 as u32
}
/// 0x8011c8c8
pub fn wazaDataBiosGetMajikku(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x8)) as u32
}
/// 0x8011c8e0
pub fn wazaDataBiosGetMamoru(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x7)) as u32
}
/// 0x8011c850
pub fn wazaDataBiosGetMonomane(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xd)) as u32
}
/// 0x8011ca1c
pub fn wazaDataBiosGetName(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x20))
}
/// 0x8011c820
pub fn wazaDataBiosGetNegoto(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xf)) as u32
}
/// 0x8011c838
pub fn wazaDataBiosGetNekonote(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xe)) as u32
}
/// 0x8011c880
pub fn wazaDataBiosGetOujanosirusi(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xb)) as u32
}
/// 0x8011c898
pub fn wazaDataBiosGetOumugaesi(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xa)) as u32
}
/// 0x8011ca04
pub fn wazaDataBiosGetPp(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x1)) as u32
}
/// 0x8011c7f0
pub fn wazaDataBiosGetPressure(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x11)) as u32
}
/// 0x8011c9d0
pub fn wazaDataBiosGetPri(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i8(p) as i32 as u32
}
/// 0x8011ca34
pub fn wazaDataBiosGetPtr(m: &Mem, i: u32) -> u32 {
    let i = i & 0xffff;
    if m.u32(m.u32(0x80478df8)) <= i { return m.u32(0x80478dfc); }
    m.u32(0x80478dfc).wrapping_add(i * 0x38)
}
/// 0x8011c9b8
pub fn wazaDataBiosGetRangeId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x3)) as u32
}
/// 0x8011c570
pub fn wazaDataBiosGetRiskFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x13)) as u32
}
/// 0x8011c958
pub fn wazaDataBiosGetSeqId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x1a)) as u32
}
/// 0x8011c588
pub fn wazaDataBiosGetTypeId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 3 { return 0x0; }
    m.u8(p.wrapping_add((i & 0xffff).wrapping_mul(1)).wrapping_add(0x34)) as u32
}
/// 0x8011c7d8
pub fn wazaDataBiosGetWazawzxdataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x30))
}
/// 0x8011c8b0
pub fn wazaDataBiosGetYokodori(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x9)) as u32
}
/// 0x8011c868
pub fn wazaDataBiosGetYubiwohuru(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc)) as u32
}
/// 0x8011c9ec
pub fn wazaDataBiosGetZokuseiDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x2)) as u32
}

// Not generated (ported by hand): fightEncountDataBiosGetWipeEffectTime, fightEncountWipeDataBiosGetPtr, fightTrainerAiDataBiosGetWazaTypeReviseTypeDataId, fightTrainerAiDataBiosGetWazaTypeReviseValue, fightTrainerAiDataBiosGetZokuseiReviseValue, fightTrainerAiDataBiosGetZokuseiReviseZokuseiDataId, itemDataBiosGetHidenMachineNo, itemDataBiosGetKinomiNo, itemDataBiosGetPtr, itemDataBiosGetWazaIDByWazaMachineNo, itemDataBiosGetWazaMachineNo, pokemonDataBiosGetMitaFlag, pokemonDataBiosGetTukamaetaFlag, wazaDataBiosGetFightTrainerAiWazaDamageFuncPtr, wazaDataBiosGetFightTrainerAiWazaHitFuncPtr, wazaDataBiosGetFightTrainerAiWazaValueFuncPtr, wazaDataBiosGetFightWazaWzxTypeFuncPtr, wazaDataBiosGetFightWazaWzxVariationFuncPtr
