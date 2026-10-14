//! Generated from the Ghidra decompiles of the game's trivial `*Bios` getters and setters
//! (scratch `gen_bios.py` + `emit_bios.py`, 2026-10-12). Names, offsets, null checks, index bounds and clamps
//! are the original's. Functions with more logic are ported by hand in `pokemon.rs`.
#![allow(non_snake_case, unused_variables, clippy::all)]

use crate::mem::Mem;

/// 0x8011cea0
pub fn FUN_8011cea0(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x4), v);
}
/// 0x8011ceb0
pub fn FUN_8011ceb0(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x3), v);
}
/// 0x8011cec0
pub fn FUN_8011cec0(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x16), v);
}
/// 0x8011ced0
pub fn FUN_8011ced0(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 2 { return; }
    m.set_u8(p.wrapping_add((i & 0xffff).wrapping_mul(1)).wrapping_add(0x6e), v);
}
/// 0x8011e000
pub fn FUN_8011e000(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x4)) as u32
}
/// 0x8011e018
pub fn FUN_8011e018(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x3)) as u32
}
/// 0x8011e030
pub fn FUN_8011e030(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x16)) as u32
}
/// 0x8011e048
pub fn FUN_8011e048(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 2 { return 0x0; }
    m.u8(p.wrapping_add((i & 0xffff).wrapping_mul(1)).wrapping_add(0x6e)) as u32
}
/// 0x8011edc4
pub fn FUN_8011edc4(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 1 { return 0x0; }
    p.wrapping_add(0xe8)
}
/// 0x8011f474
pub fn FUN_8011f474(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 1 { return 0x0; }
    p.wrapping_add(0x64)
}
/// 0x801fd978
pub fn fightOutPokemonBiosGetAbicntAverage(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x609)) as u32
}
/// 0x801fd960
pub fn fightOutPokemonBiosGetAbicntAvoid(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x60a)) as u32
}
/// 0x801fd990
pub fn fightOutPokemonBiosGetAbicntNimbleness(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x608)) as u32
}
/// 0x801fd9f0
pub fn fightOutPokemonBiosGetAbicntPhyAtk(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x604)) as u32
}
/// 0x801fd9d8
pub fn fightOutPokemonBiosGetAbicntPhyDef(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x605)) as u32
}
/// 0x801fd9c0
pub fn fightOutPokemonBiosGetAbicntSpeAtk(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x606)) as u32
}
/// 0x801fd9a8
pub fn fightOutPokemonBiosGetAbicntSpeDef(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x607)) as u32
}
/// 0x801fd4d8
pub fn fightOutPokemonBiosGetChouhatsuNoAttackFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x692)) as u32
}
/// 0x801fd768
pub fn fightOutPokemonBiosGetDamageAtkTargetId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x684)) as u32
}
/// 0x801fd780
pub fn fightOutPokemonBiosGetDamageAtkValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i16(p.wrapping_add(0x682)) as i32 as u32
}
/// 0x801fd738
pub fn fightOutPokemonBiosGetDamageSpeTargetId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x688)) as u32
}
/// 0x801fd750
pub fn fightOutPokemonBiosGetDamageSpeValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i16(p.wrapping_add(0x686)) as i32 as u32
}
/// 0x801fd460
pub fn fightOutPokemonBiosGetDoClearbodyFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x697)) as u32
}
/// 0x801fd418
pub fn fightOutPokemonBiosGetDoIkakuFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x69a)) as u32
}
/// 0x801fd400
pub fn fightOutPokemonBiosGetDoTraceFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x69b)) as u32
}
/// 0x801fd648
pub fn fightOutPokemonBiosGetFightActionBuffPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    p.wrapping_add(0x62c)
}
/// 0x801fd478
pub fn fightOutPokemonBiosGetFightActionFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x696)) as u32
}
/// 0x801fda08
pub fn fightOutPokemonBiosGetFightItemPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    p.wrapping_add(0x5f4)
}
/// 0x801fd11c
pub fn fightOutPokemonBiosGetFightOutPokemonEnemyPtr(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 4 { return 0x0; }
    p.wrapping_add((i & 0xffff).wrapping_mul(12)).wrapping_add(0x6b0)
}
/// 0x801fda6c
pub fn fightOutPokemonBiosGetFightPokemonHensinBuffPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    p.wrapping_add(0x8)
}
/// 0x801fda84
pub fn fightOutPokemonBiosGetFightPokemonPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x4))
}
/// 0x801fda20
pub fn fightOutPokemonBiosGetFightWazaPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    p.wrapping_add(0x49c)
}
/// 0x801fda38
pub fn fightOutPokemonBiosGetFightoutJoutaiPtr(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 52 { return 0x0; }
    p.wrapping_add((i & 0xffff).wrapping_mul(16)).wrapping_add(0x15c)
}
/// 0x801fd948
pub fn fightOutPokemonBiosGetFightoutTurnCount(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x610)) as u32
}
/// 0x801fcfbc
pub fn fightOutPokemonBiosGetGamanDamageTargetId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x676)) as u32
}
/// 0x801fcfd4
pub fn fightOutPokemonBiosGetGamanDamageValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i16(p.wrapping_add(0x674)) as i32 as u32
}
/// 0x801fd4a8
pub fn fightOutPokemonBiosGetHirumuNoAttackFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x694)) as u32
}
/// 0x801fd004
pub fn fightOutPokemonBiosGetHitWazaDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x670)) as u32
}
/// 0x801fcfec
pub fn fightOutPokemonBiosGetHitWazaZokuseiDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x9; }
    m.u16(p.wrapping_add(0x672)) as u32
}
/// 0x801fd520
pub fn fightOutPokemonBiosGetHuuinNoAttackFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x68f)) as u32
}
/// 0x801fd4c0
pub fn fightOutPokemonBiosGetIchamonNoAttackFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x693)) as u32
}
/// 0x801fd160
pub fn fightOutPokemonBiosGetIrekaeTargetEntryId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0xffffffff; }
    m.i16(p.wrapping_add(0x6ae)) as i32 as u32
}
/// 0x801fd3d0
pub fn fightOutPokemonBiosGetIrekaetaFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x69d)) as u32
}
/// 0x801fd3b8
pub fn fightOutPokemonBiosGetItemKoraetaFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x69e)) as u32
}
/// 0x801fd538
pub fn fightOutPokemonBiosGetItemNigeruFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x68e)) as u32
}
/// 0x801fd3a0
pub fn fightOutPokemonBiosGetKaigaraDamageValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x6a0))
}
/// 0x801fd4f0
pub fn fightOutPokemonBiosGetKanashibariNoAttackFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x691)) as u32
}
/// 0x801fcf8c
pub fn fightOutPokemonBiosGetKeepFightWazaPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    p.wrapping_add(0x548)
}
/// 0x801fd188
pub fn fightOutPokemonBiosGetKizetuFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x6ac)) as u32
}
/// 0x801fd580
pub fn fightOutPokemonBiosGetKonranMyselfAttackFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x68b)) as u32
}
/// 0x801fd01c
pub fn fightOutPokemonBiosGetLastReceiveWazaTargetDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x66e)) as u32
}
/// 0x801fd04c
pub fn fightOutPokemonBiosGetLastSelectWazaDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x66a)) as u32
}
/// 0x801fd034
pub fn fightOutPokemonBiosGetLastUseWazaDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x66c)) as u32
}
/// 0x801fd598
pub fn fightOutPokemonBiosGetMahiNoAttackFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x68a)) as u32
}
/// 0x801fd684
pub fn fightOutPokemonBiosGetMeetEnemyFightPokemonEntryId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0xffffffff; }
    if i & 0xff >= 12 { return 0xffffffff; }
    m.i16(p.wrapping_add((i & 0xff).wrapping_mul(2)).wrapping_add(0x612)) as i32 as u32
}
/// 0x801fd508
pub fn fightOutPokemonBiosGetMeroMeroNoAttackFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x690)) as u32
}
/// 0x801fda9c
pub fn fightOutPokemonBiosGetMotoFightPokemonPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p)
}
/// 0x801fd370
pub fn fightOutPokemonBiosGetMyselfDamageAtkTargetId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x6a6)) as u32
}
/// 0x801fd388
pub fn fightOutPokemonBiosGetMyselfDamageAtkValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i16(p.wrapping_add(0x6a4)) as i32 as u32
}
/// 0x801fd340
pub fn fightOutPokemonBiosGetMyselfDamageSpeTargetId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x6aa)) as u32
}
/// 0x801fd358
pub fn fightOutPokemonBiosGetMyselfDamageSpeValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.i16(p.wrapping_add(0x6a8)) as i32 as u32
}
/// 0x801fd7e0
pub fn fightOutPokemonBiosGetNamakeFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x67a)) as u32
}
/// 0x801fd3e8
pub fn fightOutPokemonBiosGetNoPressureFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x69c)) as u32
}
/// 0x801fcfa4
pub fn fightOutPokemonBiosGetOumuWazaDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x678)) as u32
}
/// 0x801fd568
pub fn fightOutPokemonBiosGetOutWazaKoukanaiFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x68c)) as u32
}
/// 0x801fd490
pub fn fightOutPokemonBiosGetPassPpdecFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x695)) as u32
}
/// 0x801fd448
pub fn fightOutPokemonBiosGetReceivesWazaHiraishinFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x698)) as u32
}
/// 0x801fd808
pub fn fightOutPokemonBiosGetSequencePtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x600))
}
/// 0x801fd064
pub fn fightOutPokemonBiosGetSketchWazaDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x668)) as u32
}
/// 0x801fd7b0
pub fn fightOutPokemonBiosGetStockItemDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x680)) as u32
}
/// 0x801fd798
pub fn fightOutPokemonBiosGetSuccessCnt(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x67e)) as u32
}
/// 0x801fd550
pub fn fightOutPokemonBiosGetTameWazaFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x68d)) as u32
}
/// 0x801fd5d8
pub fn fightOutPokemonBiosGetTokuseiDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x660)) as u32
}
/// 0x801fd7c8
pub fn fightOutPokemonBiosGetUsedItemDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x67c)) as u32
}
/// 0x801fd430
pub fn fightOutPokemonBiosGetVanishoffFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x699)) as u32
}
/// 0x801fd5b0
pub fn fightOutPokemonBiosGetWazaMenuCurPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    p.wrapping_add(0x664)
}
/// 0x801fd614
pub fn fightOutPokemonBiosGetZokuseiDataId(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xff >= 2 { return 0x0; }
    m.u16(p.wrapping_add((i & 0xff).wrapping_mul(2)).wrapping_add(0x65c)) as u32
}
/// 0x801fd850
pub fn fightOutPokemonBiosSetAbicntAverage(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x609), v);
}
/// 0x801fd840
pub fn fightOutPokemonBiosSetAbicntAvoid(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x60a), v);
}
/// 0x801fd860
pub fn fightOutPokemonBiosSetAbicntNimbleness(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x608), v);
}
/// 0x801fd8a0
pub fn fightOutPokemonBiosSetAbicntPhyAtk(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x604), v);
}
/// 0x801fd890
pub fn fightOutPokemonBiosSetAbicntPhyDef(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x605), v);
}
/// 0x801fd880
pub fn fightOutPokemonBiosSetAbicntSpeAtk(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x606), v);
}
/// 0x801fd870
pub fn fightOutPokemonBiosSetAbicntSpeDef(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x607), v);
}
/// 0x801fd2b0
pub fn fightOutPokemonBiosSetChouhatsuNoAttackFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x692), v);
}
/// 0x801fd6d8
pub fn fightOutPokemonBiosSetDamageAtkTargetId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x684), v);
}
/// 0x801fd6e8
pub fn fightOutPokemonBiosSetDamageAtkValue(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x682), v);
}
/// 0x801fd6b8
pub fn fightOutPokemonBiosSetDamageSpeTargetId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x688), v);
}
/// 0x801fd6c8
pub fn fightOutPokemonBiosSetDamageSpeValue(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x686), v);
}
/// 0x801fd260
pub fn fightOutPokemonBiosSetDoClearbodyFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x697), v);
}
/// 0x801fd230
pub fn fightOutPokemonBiosSetDoIkakuFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x69a), v);
}
/// 0x801fd220
pub fn fightOutPokemonBiosSetDoTraceFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x69b), v);
}
/// 0x801fd270
pub fn fightOutPokemonBiosSetFightActionFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x696), v);
}
/// 0x801fd8b0
pub fn fightOutPokemonBiosSetFightPokemonPtr(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x4), v);
}
/// 0x801fd820
pub fn fightOutPokemonBiosSetFightoutTurnCount(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x610), v);
}
/// 0x801fcf0c
pub fn fightOutPokemonBiosSetGamanDamageTargetId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x676), v);
}
/// 0x801fcf1c
pub fn fightOutPokemonBiosSetGamanDamageValue(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x674), v);
}
/// 0x801fd290
pub fn fightOutPokemonBiosSetHirumuNoAttackFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x694), v);
}
/// 0x801fcf3c
pub fn fightOutPokemonBiosSetHitWazaDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x670), v);
}
/// 0x801fcf2c
pub fn fightOutPokemonBiosSetHitWazaZokuseiDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x672), v);
}
/// 0x801fd2e0
pub fn fightOutPokemonBiosSetHuuinNoAttackFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x68f), v);
}
/// 0x801fd2a0
pub fn fightOutPokemonBiosSetIchamonNoAttackFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x693), v);
}
/// 0x801fd150
pub fn fightOutPokemonBiosSetIrekaeTargetEntryId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x6ae), v);
}
/// 0x801fd200
pub fn fightOutPokemonBiosSetIrekaetaFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x69d), v);
}
/// 0x801fd1f0
pub fn fightOutPokemonBiosSetItemKoraetaFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x69e), v);
}
/// 0x801fd2f0
pub fn fightOutPokemonBiosSetItemNigeruFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x68e), v);
}
/// 0x801fd1e0
pub fn fightOutPokemonBiosSetKaigaraDamageValue(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x6a0), v);
}
/// 0x801fd2c0
pub fn fightOutPokemonBiosSetKanashibariNoAttackFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x691), v);
}
/// 0x801fd178
pub fn fightOutPokemonBiosSetKizetuFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x6ac), v);
}
/// 0x801fd320
pub fn fightOutPokemonBiosSetKonranMyselfAttackFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x68b), v);
}
/// 0x801fcf4c
pub fn fightOutPokemonBiosSetLastReceiveWazaTargetDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x66e), v);
}
/// 0x801fcf6c
pub fn fightOutPokemonBiosSetLastSelectWazaDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x66a), v);
}
/// 0x801fcf5c
pub fn fightOutPokemonBiosSetLastUseWazaDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x66c), v);
}
/// 0x801fd330
pub fn fightOutPokemonBiosSetMahiNoAttackFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x68a), v);
}
/// 0x801fd2d0
pub fn fightOutPokemonBiosSetMeroMeroNoAttackFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x690), v);
}
/// 0x801fd8c0
pub fn fightOutPokemonBiosSetMotoFightPokemonPtr(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p, v);
}
/// 0x801fd1c0
pub fn fightOutPokemonBiosSetMyselfDamageAtkTargetId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x6a6), v);
}
/// 0x801fd1d0
pub fn fightOutPokemonBiosSetMyselfDamageAtkValue(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x6a4), v);
}
/// 0x801fd1a0
pub fn fightOutPokemonBiosSetMyselfDamageSpeTargetId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x6aa), v);
}
/// 0x801fd1b0
pub fn fightOutPokemonBiosSetMyselfDamageSpeValue(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x6a8), v);
}
/// 0x801fd728
pub fn fightOutPokemonBiosSetNamakeFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x67a), v);
}
/// 0x801fd210
pub fn fightOutPokemonBiosSetNoPressureFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x69c), v);
}
/// 0x801fcefc
pub fn fightOutPokemonBiosSetOumuWazaDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x678), v);
}
/// 0x801fd310
pub fn fightOutPokemonBiosSetOutWazaKoukanaiFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x68c), v);
}
/// 0x801fd280
pub fn fightOutPokemonBiosSetPassPpdecFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x695), v);
}
/// 0x801fd250
pub fn fightOutPokemonBiosSetReceivesWazaHiraishinFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x698), v);
}
/// 0x801fd7f8
pub fn fightOutPokemonBiosSetSequencePtr(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x600), v);
}
/// 0x801fcf7c
pub fn fightOutPokemonBiosSetSketchWazaDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x668), v);
}
/// 0x801fd708
pub fn fightOutPokemonBiosSetStockItemDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x680), v);
}
/// 0x801fd6f8
pub fn fightOutPokemonBiosSetSuccessCnt(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x67e), v);
}
/// 0x801fd300
pub fn fightOutPokemonBiosSetTameWazaFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x68d), v);
}
/// 0x801fd5c8
pub fn fightOutPokemonBiosSetTokuseiDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x660), v);
}
/// 0x801fd718
pub fn fightOutPokemonBiosSetUsedItemDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x67c), v);
}
/// 0x801fd240
pub fn fightOutPokemonBiosSetVanishoffFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x699), v);
}
/// 0x801fd5f0
pub fn fightOutPokemonBiosSetZokuseiDataId(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xff >= 2 { return; }
    m.set_u16(p.wrapping_add((i & 0xff).wrapping_mul(2)).wrapping_add(0x65c), v);
}
/// 0x801fdae4
pub fn fightPokemonBiosGetCatchEntryFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x14e)) as u32
}
/// 0x801fdab4
pub fn fightPokemonBiosGetDarkOutFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x150)) as u32
}
/// 0x801fdafc
pub fn fightPokemonBiosGetEntryId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0xffffffff; }
    m.i16(p.wrapping_add(0x14c)) as i32 as u32
}
/// 0x801fdb14
pub fn fightPokemonBiosGetFightJoutaiPtr(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 1 { return 0x0; }
    p.wrapping_add(0x13c)
}
/// 0x801fd8e0
pub fn fightPokemonBiosGetHokakuFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x151)) as u32
}
/// 0x801fdacc
pub fn fightPokemonBiosGetLevelUpFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x14f)) as u32
}
/// 0x801fdb60
pub fn fightPokemonBiosGetMotoPokemonPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p)
}
/// 0x801fdb48
pub fn fightPokemonBiosGetPokemonBuffPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    p.wrapping_add(0x4)
}
/// 0x801fd918
pub fn fightPokemonBiosSetCatchEntryFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x14e), v);
}
/// 0x801fd8f8
pub fn fightPokemonBiosSetDarkOutFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x150), v);
}
/// 0x801fd928
pub fn fightPokemonBiosSetEntryId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x14c), v);
}
/// 0x801fd8d0
pub fn fightPokemonBiosSetHokakuFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x151), v);
}
/// 0x801fd908
pub fn fightPokemonBiosSetLevelUpFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x14f), v);
}
/// 0x801fd938
pub fn fightPokemonBiosSetMotoPokemonPtr(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p, v);
}
/// 0x8020a164
pub fn fightWazaBiosGetAutoMakeFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xa9)) as u32
}
/// 0x8020a20c
pub fn fightWazaBiosGetCritical(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x98)) as u32
}
/// 0x8020a1dc
pub fn fightWazaBiosGetDamage(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x9c))
}
/// 0x8020a1f4
pub fn fightWazaBiosGetDamageValue(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x99)) as u32
}
/// 0x8020a1c4
pub fn fightWazaBiosGetHitDamage(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0xa0))
}
/// 0x8020a1ac
pub fn fightWazaBiosGetIryoku(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xa4)) as u32
}
/// 0x8020a224
pub fn fightWazaBiosGetJoutaiPtr(m: &Mem, p: u32, i: u32) -> u32 {
    if p == 0 { return 0x0; }
    if i & 0xffff >= 9 { return 0x0; }
    p.wrapping_add((i & 0xffff).wrapping_mul(16)).wrapping_add(0x8)
}
/// 0x8020a17c
pub fn fightWazaBiosGetKaisuu(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xa8)) as u32
}
/// 0x8020a288
pub fn fightWazaBiosGetMotoWazaDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x2)) as u32
}
/// 0x8020a258
pub fn fightWazaBiosGetTargetDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x6)) as u32
}
/// 0x8020a270
pub fn fightWazaBiosGetUseWazaDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x4)) as u32
}
/// 0x8020a2a0
pub fn fightWazaBiosGetWazaBanme(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p) as u32
}
/// 0x8020a194
pub fn fightWazaBiosGetZokusei(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xa6)) as u32
}
/// 0x8020a114
pub fn fightWazaBiosSetCritical(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x98), v);
}
/// 0x8020a0f4
pub fn fightWazaBiosSetDamage(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x9c), v);
}
/// 0x8020a104
pub fn fightWazaBiosSetDamageValue(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x99), v);
}
/// 0x8020a0e4
pub fn fightWazaBiosSetHitDamage(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0xa0), v);
}
/// 0x8020a0d4
pub fn fightWazaBiosSetIryoku(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0xa4), v);
}
/// 0x8020a0b4
pub fn fightWazaBiosSetKaisuu(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xa8), v);
}
/// 0x8020a144
pub fn fightWazaBiosSetMotoWazaDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x2), v);
}
/// 0x8020a124
pub fn fightWazaBiosSetTargetDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x6), v);
}
/// 0x8020a134
pub fn fightWazaBiosSetUseWazaDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x4), v);
}
/// 0x8020a154
pub fn fightWazaBiosSetWazaBanme(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p, v);
}
/// 0x8020a0c4
pub fn fightWazaBiosSetZokusei(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0xa6), v);
}
/// 0x8011e7f0
pub fn pokemonBiosGetAmari(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xd4)) as u32
}
/// 0x8011e90c
pub fn pokemonBiosGetAmariRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc9)) as u32
}
/// 0x8011f598
pub fn pokemonBiosGetAttest(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    p.wrapping_add(0x8)
}
/// 0x8011ecc0
pub fn pokemonBiosGetBeautiful(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xb3)) as u32
}
/// 0x8011ebe4
pub fn pokemonBiosGetBeautifulMedal(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xb8)) as u32
}
/// 0x8011ea98
pub fn pokemonBiosGetBromideRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc0)) as u32
}
/// 0x8011f550
pub fn pokemonBiosGetCatchBallId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xf)) as u32
}
/// 0x8011f580
pub fn pokemonBiosGetCatchFloorId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xc)) as u32
}
/// 0x8011f568
pub fn pokemonBiosGetCatchLevel(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xe)) as u32
}
/// 0x8011f508
pub fn pokemonBiosGetCatchTrainerNamePtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    p.wrapping_add(0x18)
}
/// 0x8011f520
pub fn pokemonBiosGetCatchTrainerRnd(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x14))
}
/// 0x8011f538
pub fn pokemonBiosGetCatchTrainerSex(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x10)) as u32
}
/// 0x8011eb1c
pub fn pokemonBiosGetChampRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xbd)) as u32
}
/// 0x8011ec68
pub fn pokemonBiosGetClever(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xb5)) as u32
}
/// 0x8011eb8c
pub fn pokemonBiosGetCleverMedal(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xba)) as u32
}
/// 0x8011f45c
pub fn pokemonBiosGetConditionAmari(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x74))
}
/// 0x8011e9bc
pub fn pokemonBiosGetCountryRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc5)) as u32
}
/// 0x8011ec94
pub fn pokemonBiosGetCute(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xb4)) as u32
}
/// 0x8011ebb8
pub fn pokemonBiosGetCuteMedal(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xb9)) as u32
}
/// 0x8011ee40
pub fn pokemonBiosGetDarkpokemonDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xd8)) as u32
}
/// 0x8011ee28
pub fn pokemonBiosGetDp(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0xdc))
}
/// 0x8011e964
pub fn pokemonBiosGetEarthRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc7)) as u32
}
/// 0x8011f4c0
pub fn pokemonBiosGetExp(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x5c))
}
/// 0x8011e7c0
pub fn pokemonBiosGetFightTrainerPokemonDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xd6)) as u32
}
/// 0x8011e838
pub fn pokemonBiosGetFlagAmari(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xce)) as u32
}
/// 0x8011ee58
pub fn pokemonBiosGetFriend(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xb0)) as u32
}
/// 0x8011eb48
pub fn pokemonBiosGetFur(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xbc)) as u32
}
/// 0x8011e850
pub fn pokemonBiosGetFuseiFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xcd)) as u32
}
/// 0x8011ea6c
pub fn pokemonBiosGetGanbaRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc1)) as u32
}
/// 0x8011f188
pub fn pokemonBiosGetHp(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x8a)) as u32
}
/// 0x8011f1a0
pub fn pokemonBiosGetItemDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x88)) as u32
}
/// 0x8011ea14
pub fn pokemonBiosGetLandRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc3)) as u32
}
/// 0x8011f4a8
pub fn pokemonBiosGetLevel(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0x60)) as u32
}
/// 0x8011e808
pub fn pokemonBiosGetMailId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xd0)) as u32
}
/// 0x8011ea40
pub fn pokemonBiosGetMarineRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc2)) as u32
}
/// 0x8011f15c
pub fn pokemonBiosGetMaxHp(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x8c)) as u32
}
/// 0x8011f054
pub fn pokemonBiosGetMaxHpEffort(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x98)) as u32
}
/// 0x8011ef4c
pub fn pokemonBiosGetMaxHpRnd(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xa4)) as u32
}
/// 0x8011e990
pub fn pokemonBiosGetNationalRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc6)) as u32
}
/// 0x8011f4d8
pub fn pokemonBiosGetNicknameOrgPtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    p.wrapping_add(0x44)
}
/// 0x8011f4f0
pub fn pokemonBiosGetNicknamePtr(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    p.wrapping_add(0x2e)
}
/// 0x8011f080
pub fn pokemonBiosGetNimbleness(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x96)) as u32
}
/// 0x8011ef78
pub fn pokemonBiosGetNimblenessEffort(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xa2)) as u32
}
/// 0x8011ee70
pub fn pokemonBiosGetNimblenessRnd(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xae)) as u32
}
/// 0x8011e7d8
pub fn pokemonBiosGetPara1Amari(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xd2)) as u32
}
/// 0x8011e820
pub fn pokemonBiosGetPcboxMark(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xcf)) as u32
}
/// 0x8011f130
pub fn pokemonBiosGetPhyAtk(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x8e)) as u32
}
/// 0x8011f028
pub fn pokemonBiosGetPhyAtkEffort(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x9a)) as u32
}
/// 0x8011ef20
pub fn pokemonBiosGetPhyAtkRnd(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xa6)) as u32
}
/// 0x8011f104
pub fn pokemonBiosGetPhyDef(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x90)) as u32
}
/// 0x8011effc
pub fn pokemonBiosGetPhyDefEffort(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x9c)) as u32
}
/// 0x8011eef4
pub fn pokemonBiosGetPhyDefRnd(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xa8)) as u32
}
/// 0x8011f5c8
pub fn pokemonBiosGetPokemonDataId(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p) as u32
}
/// 0x8011e8f4
pub fn pokemonBiosGetPokerus(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xca)) as u32
}
/// 0x8011ee10
pub fn pokemonBiosGetPoolExp(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0xe0))
}
/// 0x8011edf8
pub fn pokemonBiosGetPoolFriend(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xe4)) as u32
}
/// 0x8011f5b0
pub fn pokemonBiosGetRnd(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u32(p.wrapping_add(0x4))
}
/// 0x8011e9e8
pub fn pokemonBiosGetSkyRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc4)) as u32
}
/// 0x8011f0d8
pub fn pokemonBiosGetSpeAtk(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x92)) as u32
}
/// 0x8011efd0
pub fn pokemonBiosGetSpeAtkEffort(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x9e)) as u32
}
/// 0x8011eec8
pub fn pokemonBiosGetSpeAtkRnd(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xaa)) as u32
}
/// 0x8011f0ac
pub fn pokemonBiosGetSpeDef(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0x94)) as u32
}
/// 0x8011efa4
pub fn pokemonBiosGetSpeDefEffort(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xa0)) as u32
}
/// 0x8011ee9c
pub fn pokemonBiosGetSpeDefRnd(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u16(p.wrapping_add(0xac)) as u32
}
/// 0x8011ec3c
pub fn pokemonBiosGetStrong(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xb6)) as u32
}
/// 0x8011eb60
pub fn pokemonBiosGetStrongMedal(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xbb)) as u32
}
/// 0x8011ecec
pub fn pokemonBiosGetStyle(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xb2)) as u32
}
/// 0x8011ec10
pub fn pokemonBiosGetStyleMedal(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xb7)) as u32
}
/// 0x8011e8dc
pub fn pokemonBiosGetTamagoFlag(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xcb)) as u32
}
/// 0x8011eac4
pub fn pokemonBiosGetVictoryRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xbf)) as u32
}
/// 0x8011eaf0
pub fn pokemonBiosGetWinningRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xbe)) as u32
}
/// 0x8011e938
pub fn pokemonBiosGetWorldRibbon(m: &Mem, p: u32) -> u32 {
    if p == 0 { return 0x0; }
    m.u8(p.wrapping_add(0xc8)) as u32
}
/// 0x8011d4b4
pub fn pokemonBiosSetAmari(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0xd4), v);
}
/// 0x8011d58c
pub fn pokemonBiosSetAmariRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xc9), v);
}
/// 0x8011d890
pub fn pokemonBiosSetBeautiful(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xb3), v);
}
/// 0x8011d7dc
pub fn pokemonBiosSetBeautifulMedal(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xb8), v);
}
/// 0x8011d6d0
pub fn pokemonBiosSetBromideRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xc0), v);
}
/// 0x8011dfb0
pub fn pokemonBiosSetCatchBallId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xf), v);
}
/// 0x8011dfd0
pub fn pokemonBiosSetCatchFloorId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0xc), v);
}
/// 0x8011dfc0
pub fn pokemonBiosSetCatchLevel(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xe), v);
}
/// 0x8011df90
pub fn pokemonBiosSetCatchTrainerRnd(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x14), v);
}
/// 0x8011dfa0
pub fn pokemonBiosSetCatchTrainerSex(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x10), v);
}
/// 0x8011d73c
pub fn pokemonBiosSetChampRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xbd), v);
}
/// 0x8011d848
pub fn pokemonBiosSetClever(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xb5), v);
}
/// 0x8011d794
pub fn pokemonBiosSetCleverMedal(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xba), v);
}
/// 0x8011de38
pub fn pokemonBiosSetConditionAmari(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x74), v);
}
/// 0x8011d61c
pub fn pokemonBiosSetCountryRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xc5), v);
}
/// 0x8011d86c
pub fn pokemonBiosSetCute(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xb4), v);
}
/// 0x8011d7b8
pub fn pokemonBiosSetCuteMedal(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xb9), v);
}
/// 0x8011d8f4
pub fn pokemonBiosSetDarkpokemonDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0xd8), v);
}
/// 0x8011d8d8
pub fn pokemonBiosSetDp(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v > 0x639b { 0x639c } else { v };
    m.set_u32(p.wrapping_add(0xdc), v);
}
/// 0x8011d5d4
pub fn pokemonBiosSetEarthRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xc7), v);
}
/// 0x8011de98
pub fn pokemonBiosSetExp(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x5c), v);
}
/// 0x8011d494
pub fn pokemonBiosSetFightTrainerPokemonDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0xd6), v);
}
/// 0x8011d4e4
pub fn pokemonBiosSetFlagAmari(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xce), v);
}
/// 0x8011d904
pub fn pokemonBiosSetFriend(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0xff { 0xff } else { v };
    m.set_u16(p.wrapping_add(0xb0), v);
}
/// 0x8011d760
pub fn pokemonBiosSetFur(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xbc), v);
}
/// 0x8011d4f4
pub fn pokemonBiosSetFuseiFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xcd), v);
}
/// 0x8011d6ac
pub fn pokemonBiosSetGanbaRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xc1), v);
}
/// 0x8011dcb4
pub fn pokemonBiosSetItemDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x88), v);
}
/// 0x8011d664
pub fn pokemonBiosSetLandRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xc3), v);
}
/// 0x8011de48
pub fn pokemonBiosSetLevel(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xff > 0x64 { 0x64 } else { v };
    m.set_u8(p.wrapping_add(0x60), v);
}
/// 0x8011d4c4
pub fn pokemonBiosSetMailId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xd0), v);
}
/// 0x8011d688
pub fn pokemonBiosSetMarineRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xc2), v);
}
/// 0x8011dc48
pub fn pokemonBiosSetMaxHp(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x8c), v);
}
/// 0x8011db60
pub fn pokemonBiosSetMaxHpEffort(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0xff { 0xff } else { v };
    m.set_u16(p.wrapping_add(0x98), v);
}
/// 0x8011da28
pub fn pokemonBiosSetMaxHpRnd(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0x1f { 0x1f } else { v };
    m.set_u16(p.wrapping_add(0xa4), v);
}
/// 0x8011d5f8
pub fn pokemonBiosSetNationalRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xc6), v);
}
/// 0x8011db94
pub fn pokemonBiosSetNimbleness(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x96), v);
}
/// 0x8011da5c
pub fn pokemonBiosSetNimblenessEffort(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0xff { 0xff } else { v };
    m.set_u16(p.wrapping_add(0xa2), v);
}
/// 0x8011d924
pub fn pokemonBiosSetNimblenessRnd(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0x1f { 0x1f } else { v };
    m.set_u16(p.wrapping_add(0xae), v);
}
/// 0x8011d4a4
pub fn pokemonBiosSetPara1Amari(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0xd2), v);
}
/// 0x8011d4d4
pub fn pokemonBiosSetPcboxMark(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xcf), v);
}
/// 0x8011dc24
pub fn pokemonBiosSetPhyAtk(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x8e), v);
}
/// 0x8011db2c
pub fn pokemonBiosSetPhyAtkEffort(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0xff { 0xff } else { v };
    m.set_u16(p.wrapping_add(0x9a), v);
}
/// 0x8011d9f4
pub fn pokemonBiosSetPhyAtkRnd(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0x1f { 0x1f } else { v };
    m.set_u16(p.wrapping_add(0xa6), v);
}
/// 0x8011dc00
pub fn pokemonBiosSetPhyDef(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x90), v);
}
/// 0x8011daf8
pub fn pokemonBiosSetPhyDefEffort(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0xff { 0xff } else { v };
    m.set_u16(p.wrapping_add(0x9c), v);
}
/// 0x8011d9c0
pub fn pokemonBiosSetPhyDefRnd(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0x1f { 0x1f } else { v };
    m.set_u16(p.wrapping_add(0xa8), v);
}
/// 0x8011dff0
pub fn pokemonBiosSetPokemonDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p, v);
}
/// 0x8011d57c
pub fn pokemonBiosSetPokerus(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xca), v);
}
/// 0x8011de88
pub fn pokemonBiosSetPoolExp(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0xe0), v);
}
/// 0x8011de68
pub fn pokemonBiosSetPoolFriend(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0xff { 0xff } else { v };
    m.set_u16(p.wrapping_add(0xe4), v);
}
/// 0x8011dfe0
pub fn pokemonBiosSetRnd(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x4), v);
}
/// 0x8011d640
pub fn pokemonBiosSetSkyRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xc4), v);
}
/// 0x8011dbdc
pub fn pokemonBiosSetSpeAtk(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x92), v);
}
/// 0x8011dac4
pub fn pokemonBiosSetSpeAtkEffort(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0xff { 0xff } else { v };
    m.set_u16(p.wrapping_add(0x9e), v);
}
/// 0x8011d98c
pub fn pokemonBiosSetSpeAtkRnd(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0x1f { 0x1f } else { v };
    m.set_u16(p.wrapping_add(0xaa), v);
}
/// 0x8011dbb8
pub fn pokemonBiosSetSpeDef(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x94), v);
}
/// 0x8011da90
pub fn pokemonBiosSetSpeDefEffort(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0xff { 0xff } else { v };
    m.set_u16(p.wrapping_add(0xa0), v);
}
/// 0x8011d958
pub fn pokemonBiosSetSpeDefRnd(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    let v = if v & 0xffff > 0x1f { 0x1f } else { v };
    m.set_u16(p.wrapping_add(0xac), v);
}
/// 0x8011d824
pub fn pokemonBiosSetStrong(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xb6), v);
}
/// 0x8011d770
pub fn pokemonBiosSetStrongMedal(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xbb), v);
}
/// 0x8011d8b4
pub fn pokemonBiosSetStyle(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xb2), v);
}
/// 0x8011d800
pub fn pokemonBiosSetStyleMedal(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xb7), v);
}
/// 0x8011d56c
pub fn pokemonBiosSetTamagoFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xcb), v);
}
/// 0x8011d6f4
pub fn pokemonBiosSetVictoryRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xbf), v);
}
/// 0x8011d718
pub fn pokemonBiosSetWinningRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xbe), v);
}
/// 0x8011d5b0
pub fn pokemonBiosSetWorldRibbon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xc8), v);
}
/// 0x8011d44c
pub fn pokemonDataBiosSetBasisMaxHp(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x84), v);
}
/// 0x8011d398
pub fn pokemonDataBiosSetBasisNimbleness(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x8e), v);
}
/// 0x8011d428
pub fn pokemonDataBiosSetBasisPhyAtk(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x86), v);
}
/// 0x8011d404
pub fn pokemonDataBiosSetBasisPhyDef(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x88), v);
}
/// 0x8011d3e0
pub fn pokemonDataBiosSetBasisSpeAtk(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x8a), v);
}
/// 0x8011d3bc
pub fn pokemonDataBiosSetBasisSpeDef(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x8c), v);
}
/// 0x8011cfec
pub fn pokemonDataBiosSetColor(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 2 { return; }
    m.set_u8(p.wrapping_add((i & 0xffff).wrapping_mul(8)).wrapping_add(0x10c), v);
}
/// 0x8011cfac
pub fn pokemonDataBiosSetDoc(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x20), v);
}
/// 0x8011d290
pub fn pokemonDataBiosSetGet(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x1), v);
}
/// 0x8011d0cc
pub fn pokemonDataBiosSetGetWazaDataId(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 20 { return; }
    m.set_u16(p.wrapping_add((i & 0xffff).wrapping_mul(4)).wrapping_add(0xbc), v);
}
/// 0x8011d10c
pub fn pokemonDataBiosSetGetWazaLevel(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 20 { return; }
    m.set_u8(p.wrapping_add((i & 0xffff).wrapping_mul(4)).wrapping_add(0xba), v);
}
/// 0x8011d2b0
pub fn pokemonDataBiosSetGiveExp(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x6), v);
}
/// 0x8011d374
pub fn pokemonDataBiosSetGiveMaxHpEffort(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x90), v);
}
/// 0x8011d2c0
pub fn pokemonDataBiosSetGiveNimblenessEffort(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x9a), v);
}
/// 0x8011d350
pub fn pokemonDataBiosSetGivePhyAtkEffort(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x92), v);
}
/// 0x8011d32c
pub fn pokemonDataBiosSetGivePhyDefEffort(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x94), v);
}
/// 0x8011d308
pub fn pokemonDataBiosSetGiveSpeAtkEffort(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x96), v);
}
/// 0x8011d2e4
pub fn pokemonDataBiosSetGiveSpeDefEffort(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x98), v);
}
/// 0x8011d2a0
pub fn pokemonDataBiosSetGrowDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p, v);
}
/// 0x8011cfcc
pub fn pokemonDataBiosSetHeight(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0xa), v);
}
/// 0x8011d270
pub fn pokemonDataBiosSetInitFriend(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x8), v);
}
/// 0x8011d24c
pub fn pokemonDataBiosSetItemDataId(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 2 { return; }
    m.set_u16(p.wrapping_add((i & 0xffff).wrapping_mul(2)).wrapping_add(0x70), v);
}
/// 0x8011cef0
pub fn pokemonDataBiosSetKowaza(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 8 { return; }
    m.set_u16(p.wrapping_add((i & 0xffff).wrapping_mul(2)).wrapping_add(0x74), v);
}
/// 0x8011d470
pub fn pokemonDataBiosSetName(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x18), v);
}
/// 0x8011cf24
pub fn pokemonDataBiosSetNumPokemon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x12), v);
}
/// 0x8011cf34
pub fn pokemonDataBiosSetNumZukan(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x14), v);
}
/// 0x8011cf14
pub fn pokemonDataBiosSetPkxDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x2c), v);
}
/// 0x8011d06c
pub fn pokemonDataBiosSetPokebodyId(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 2 { return; }
    m.set_u32(p.wrapping_add((i & 0xffff).wrapping_mul(8)).wrapping_add(0x110), v);
}
/// 0x8011d280
pub fn pokemonDataBiosSetSexRatio(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x2), v);
}
/// 0x8011d18c
pub fn pokemonDataBiosSetSinkaBuff(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 5 { return; }
    m.set_u16(p.wrapping_add((i & 0xffff).wrapping_mul(6)).wrapping_add(0x9e), v);
}
/// 0x8011d1cc
pub fn pokemonDataBiosSetSinkaKind(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 5 { return; }
    m.set_u8(p.wrapping_add((i & 0xffff).wrapping_mul(6)).wrapping_add(0x9c), v);
}
/// 0x8011d14c
pub fn pokemonDataBiosSetSinkaPokemonDataId(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 5 { return; }
    m.set_u16(p.wrapping_add((i & 0xffff).wrapping_mul(6)).wrapping_add(0xa0), v);
}
/// 0x8011d02c
pub fn pokemonDataBiosSetStatusFaceMenuSpriteId(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 2 { return; }
    m.set_u16(p.wrapping_add((i & 0xffff).wrapping_mul(8)).wrapping_add(0x10e), v);
}
/// 0x8011d20c
pub fn pokemonDataBiosSetTokuseiDataId(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 2 { return; }
    m.set_u8(p.wrapping_add((i & 0xffff).wrapping_mul(1)).wrapping_add(0x32), v);
}
/// 0x8011cfdc
pub fn pokemonDataBiosSetTypeName(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x1c), v);
}
/// 0x8011cf9c
pub fn pokemonDataBiosSetVoice(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0xe), v);
}
/// 0x8011d0ac
pub fn pokemonDataBiosSetWazaMcn(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 58 { return; }
    m.set_u8(p.wrapping_add((i & 0xffff).wrapping_mul(1)).wrapping_add(0x34), v);
}
/// 0x8011cfbc
pub fn pokemonDataBiosSetWeight(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0xc), v);
}
/// 0x8011d22c
pub fn pokemonDataBiosSetZokuseiDataId(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 2 { return; }
    m.set_u8(p.wrapping_add((i & 0xffff).wrapping_mul(1)).wrapping_add(0x30), v);
}
/// 0x8011c6e0
pub fn wazaDataBiosSetAddFightKoukaAvg(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x5), v);
}
/// 0x8011c730
pub fn wazaDataBiosSetAvg(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x4), v);
}
/// 0x8011c630
pub fn wazaDataBiosSetBouon(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x10), v);
}
/// 0x8011c6d0
pub fn wazaDataBiosSetDageki(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x6), v);
}
/// 0x8011c600
pub fn wazaDataBiosSetDoc(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x2c), v);
}
/// 0x8011c6f0
pub fn wazaDataBiosSetFightAttackMsgId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x28), v);
}
/// 0x8011c710
pub fn wazaDataBiosSetFightKoukaDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x18), v);
}
/// 0x8011c5b8
pub fn wazaDataBiosSetHidenFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x12), v);
}
/// 0x8011c720
pub fn wazaDataBiosSetIryoku(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x16), v);
}
/// 0x8011c6b0
pub fn wazaDataBiosSetMajikku(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x8), v);
}
/// 0x8011c6c0
pub fn wazaDataBiosSetMamoru(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x7), v);
}
/// 0x8011c660
pub fn wazaDataBiosSetMonomane(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xd), v);
}
/// 0x8011c780
pub fn wazaDataBiosSetName(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x20), v);
}
/// 0x8011c640
pub fn wazaDataBiosSetNegoto(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xf), v);
}
/// 0x8011c650
pub fn wazaDataBiosSetNekonote(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xe), v);
}
/// 0x8011c680
pub fn wazaDataBiosSetOujanosirusi(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xb), v);
}
/// 0x8011c690
pub fn wazaDataBiosSetOumugaesi(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xa), v);
}
/// 0x8011c770
pub fn wazaDataBiosSetPp(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x1), v);
}
/// 0x8011c620
pub fn wazaDataBiosSetPressure(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x11), v);
}
/// 0x8011c750
pub fn wazaDataBiosSetPri(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p, v);
}
/// 0x8011c740
pub fn wazaDataBiosSetRangeId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x3), v);
}
/// 0x8011c420
pub fn wazaDataBiosSetRiskFlag(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x13), v);
}
/// 0x8011c700
pub fn wazaDataBiosSetSeqId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u16(p.wrapping_add(0x1a), v);
}
/// 0x8011c430
pub fn wazaDataBiosSetTypeId(m: &mut Mem, p: u32, i: u32, v: u32) {
    if p == 0 { return; }
    if i & 0xffff >= 3 { return; }
    m.set_u8(p.wrapping_add((i & 0xffff).wrapping_mul(1)).wrapping_add(0x34), v);
}
/// 0x8011c610
pub fn wazaDataBiosSetWazawzxdataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u32(p.wrapping_add(0x30), v);
}
/// 0x8011c6a0
pub fn wazaDataBiosSetYokodori(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x9), v);
}
/// 0x8011c670
pub fn wazaDataBiosSetYubiwohuru(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0xc), v);
}
/// 0x8011c760
pub fn wazaDataBiosSetZokuseiDataId(m: &mut Mem, p: u32, v: u32) {
    if p == 0 { return; }
    m.set_u8(p.wrapping_add(0x2), v);
}

// Not generated (ported by hand): FUN_801eeeb8, GScharLenCpy, fightOutPokemonCheckFightOut, fightOutPokemonGetMotoWazaDataId, fightOutPokemonGetUseWazaDataId, fightWazaCheckWriteJoutaiDataId, fightWazaInitJoutai, fightWazaInitJoutaiDataId, fightWazaIsHit, fightWazaIsJoutaiDataId, fightWazaWriteJoutaiDataId, pokemonBiosGetDarkFlag, pokemonBiosGetInitDp, pokemonBiosGetPokemonWazaDataId, pokemonBiosGetPokemonWazaPp, pokemonBiosGetPokemonWazaPpCount, pokemonBiosGetTokuseiFlag, pokemonBiosSetCatchTrainerNamePtr, pokemonBiosSetHp, pokemonBiosSetNicknameOrgPtr, pokemonBiosSetNicknamePtr, pokemonBiosSetPokemonWazaDataId, pokemonBiosSetPokemonWazaPp, pokemonBiosSetPokemonWazaPpCount, pokemonBiosSetTokuseiFlag, pokemonDataBiosSetMitaFlag, pokemonDataBiosSetTukamaetaFlag, pokemonWazaCheckValid, pokemonWazaGetMaxPP, sexGetPokemonSexRaitoKotei, std082_flagGetValue, wazaDataBiosSetFightTrainerAiWazaDamageFuncPtr, wazaDataBiosSetFightTrainerAiWazaHitFuncPtr, wazaDataBiosSetFightTrainerAiWazaValueFuncPtr, wazaDataBiosSetFightWazaWzxTypeFuncPtr, wazaDataBiosSetFightWazaWzxVariationFuncPtr, wazaGetStatus
