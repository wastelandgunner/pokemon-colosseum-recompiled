//! The "status id" API: one getter/setter per object kind, switching on an id to the field accessors.
//! The arms are generated from the decompiled switches (scratch `gen_switch.py`, 2026-10-12); arms with
//! more logic call `hand::` functions. Signatures mirror the game: (object ptr, data id, kind, index[, value]).
#![allow(non_snake_case, unused_variables, unreachable_patterns, clippy::all)]

use crate::bios::*;
use crate::data_bios::*;
use crate::hand;
use crate::mem::Mem;

/// `pokemonGetStatus` 0x8012640C. Kinds 1..0x6C read species data (`id`), 0x6D..0x123 the object `p`
/// (party Pokémon, fight Pokémon or battler, depending on the kind).
pub fn pokemonGetStatus(m: &Mem, p: u32, id: u32, kind: u32, i: u32) -> u32 {
    let kind = kind & 0xffff;
    if kind == 0 || kind > 0x123 { return 0; }
    let p = if kind < 0x6d { pokemonDataBiosGetPtr(m, id) } else { p };
    if p == 0 { return 0; }
    match kind {
        0x1 => { pokemonDataBiosGetName(m, p) }
        0x3 => { (pokemonDataBiosGetBasisMaxHp(m, p) & 0xffff) }
        0x4 => { (pokemonDataBiosGetBasisPhyAtk(m, p) & 0xffff) }
        0x5 => { (pokemonDataBiosGetBasisPhyDef(m, p) & 0xffff) }
        0x6 => { (pokemonDataBiosGetBasisSpeAtk(m, p) & 0xffff) }
        0x7 => { (pokemonDataBiosGetBasisSpeDef(m, p) & 0xffff) }
        0x8 => { (pokemonDataBiosGetBasisNimbleness(m, p) & 0xffff) }
        0xa => { (pokemonDataBiosGetGiveMaxHpEffort(m, p) & 0xffff) }
        0xb => { (pokemonDataBiosGetGivePhyAtkEffort(m, p) & 0xffff) }
        0xc => { (pokemonDataBiosGetGivePhyDefEffort(m, p) & 0xffff) }
        0xd => { (pokemonDataBiosGetGiveSpeAtkEffort(m, p) & 0xffff) }
        0xe => { (pokemonDataBiosGetGiveSpeDefEffort(m, p) & 0xffff) }
        0xf => { (pokemonDataBiosGetGiveNimblenessEffort(m, p) & 0xffff) }
        0x10 => { (pokemonDataBiosGetGiveExp(m, p) & 0xffff) }
        0x11 => { (pokemonDataBiosGetGrowDataId(m, p) & 0xff) }
        0x12 => { (pokemonDataBiosGetGet(m, p) & 0xff) }
        0x13 => { (pokemonDataBiosGetSexRatio(m, p) & 0xff) }
        0x14 => { (pokemonDataBiosGetInitFriend(m, p) & 0xffff) }
        0x15 => { (pokemonDataBiosGetItemDataId(m, p, i) & 0xffff) }
        0x16 => { (pokemonDataBiosGetZokuseiDataId(m, p, i) & 0xff) }
        0x17 => { (pokemonDataBiosGetTokuseiDataId(m, p, i) & 0xff) }
        0x19 => { (pokemonDataBiosGetSinkaKind(m, p, i) & 0xff) }
        0x1a => { (pokemonDataBiosGetSinkaBuff(m, p, i) & 0xffff) }
        0x1b => { (pokemonDataBiosGetSinkaPokemonDataId(m, p, i) & 0xffff) }
        0x1d => { (pokemonDataBiosGetGetWazaLevel(m, p, i) & 0xff) }
        0x1e => { (pokemonDataBiosGetGetWazaDataId(m, p, i) & 0xffff) }
        0x20 => { (pokemonDataBiosGetWazaMcn(m, p, 0x0) & 0xff) }
        0x21 => { (pokemonDataBiosGetWazaMcn(m, p, 0x1) & 0xff) }
        0x22 => { (pokemonDataBiosGetWazaMcn(m, p, 0x2) & 0xff) }
        0x23 => { (pokemonDataBiosGetWazaMcn(m, p, 0x3) & 0xff) }
        0x24 => { (pokemonDataBiosGetWazaMcn(m, p, 0x4) & 0xff) }
        0x25 => { (pokemonDataBiosGetWazaMcn(m, p, 0x5) & 0xff) }
        0x26 => { (pokemonDataBiosGetWazaMcn(m, p, 0x6) & 0xff) }
        0x27 => { (pokemonDataBiosGetWazaMcn(m, p, 0x7) & 0xff) }
        0x28 => { (pokemonDataBiosGetWazaMcn(m, p, 0x8) & 0xff) }
        0x29 => { (pokemonDataBiosGetWazaMcn(m, p, 0x9) & 0xff) }
        0x2a => { (pokemonDataBiosGetWazaMcn(m, p, 0xa) & 0xff) }
        0x2b => { (pokemonDataBiosGetWazaMcn(m, p, 0xb) & 0xff) }
        0x2c => { (pokemonDataBiosGetWazaMcn(m, p, 0xc) & 0xff) }
        0x2d => { (pokemonDataBiosGetWazaMcn(m, p, 0xd) & 0xff) }
        0x2e => { (pokemonDataBiosGetWazaMcn(m, p, 0xe) & 0xff) }
        0x2f => { (pokemonDataBiosGetWazaMcn(m, p, 0xf) & 0xff) }
        0x30 => { (pokemonDataBiosGetWazaMcn(m, p, 0x10) & 0xff) }
        0x31 => { (pokemonDataBiosGetWazaMcn(m, p, 0x11) & 0xff) }
        0x32 => { (pokemonDataBiosGetWazaMcn(m, p, 0x12) & 0xff) }
        0x33 => { (pokemonDataBiosGetWazaMcn(m, p, 0x13) & 0xff) }
        0x34 => { (pokemonDataBiosGetWazaMcn(m, p, 0x14) & 0xff) }
        0x35 => { (pokemonDataBiosGetWazaMcn(m, p, 0x15) & 0xff) }
        0x36 => { (pokemonDataBiosGetWazaMcn(m, p, 0x16) & 0xff) }
        0x37 => { (pokemonDataBiosGetWazaMcn(m, p, 0x17) & 0xff) }
        0x38 => { (pokemonDataBiosGetWazaMcn(m, p, 0x18) & 0xff) }
        0x39 => { (pokemonDataBiosGetWazaMcn(m, p, 0x1a) & 0xff) }
        0x3a => { (pokemonDataBiosGetWazaMcn(m, p, 0x1b) & 0xff) }
        0x3b => { (pokemonDataBiosGetWazaMcn(m, p, 0x1c) & 0xff) }
        0x3c => { (pokemonDataBiosGetWazaMcn(m, p, 0x1d) & 0xff) }
        0x3d => { (pokemonDataBiosGetWazaMcn(m, p, 0x1e) & 0xff) }
        0x3e => { (pokemonDataBiosGetWazaMcn(m, p, 0x1f) & 0xff) }
        0x3f => { (pokemonDataBiosGetWazaMcn(m, p, 0x20) & 0xff) }
        0x40 => { (pokemonDataBiosGetWazaMcn(m, p, 0x21) & 0xff) }
        0x41 => { (pokemonDataBiosGetWazaMcn(m, p, 0x22) & 0xff) }
        0x42 => { (pokemonDataBiosGetWazaMcn(m, p, 0x23) & 0xff) }
        0x43 => { (pokemonDataBiosGetWazaMcn(m, p, 0x24) & 0xff) }
        0x44 => { (pokemonDataBiosGetWazaMcn(m, p, 0x25) & 0xff) }
        0x45 => { (pokemonDataBiosGetWazaMcn(m, p, 0x26) & 0xff) }
        0x46 => { (pokemonDataBiosGetWazaMcn(m, p, 0x27) & 0xff) }
        0x47 => { (pokemonDataBiosGetWazaMcn(m, p, 0x28) & 0xff) }
        0x48 => { (pokemonDataBiosGetWazaMcn(m, p, 0x29) & 0xff) }
        0x49 => { (pokemonDataBiosGetWazaMcn(m, p, 0x2a) & 0xff) }
        0x4a => { (pokemonDataBiosGetWazaMcn(m, p, 0x2b) & 0xff) }
        0x4b => { (pokemonDataBiosGetWazaMcn(m, p, 0x2c) & 0xff) }
        0x4c => { (pokemonDataBiosGetWazaMcn(m, p, 0x2d) & 0xff) }
        0x4d => { (pokemonDataBiosGetWazaMcn(m, p, 0x2e) & 0xff) }
        0x4e => { (pokemonDataBiosGetWazaMcn(m, p, 0x2f) & 0xff) }
        0x4f => { (pokemonDataBiosGetWazaMcn(m, p, 0x30) & 0xff) }
        0x50 => { (pokemonDataBiosGetWazaMcn(m, p, 0x31) & 0xff) }
        0x51 => { (pokemonDataBiosGetWazaMcn(m, p, 0x32) & 0xff) }
        0x52 => { (pokemonDataBiosGetWazaMcn(m, p, 0x33) & 0xff) }
        0x53 => { (pokemonDataBiosGetWazaMcn(m, p, 0x34) & 0xff) }
        0x54 => { (pokemonDataBiosGetWazaMcn(m, p, 0x35) & 0xff) }
        0x55 => { (pokemonDataBiosGetWazaMcn(m, p, 0x36) & 0xff) }
        0x56 => { (pokemonDataBiosGetWazaMcn(m, p, 0x37) & 0xff) }
        0x57 => { (pokemonDataBiosGetWazaMcn(m, p, 0x38) & 0xff) }
        0x58 => { (pokemonDataBiosGetWazaMcn(m, p, 0x39) & 0xff) }
        0x59 => { (pokemonDataBiosGetWazaMcn(m, p, 0x3a) & 0xff) }
        0x5a => { pokemonDataBiosGetPokebodyId(m, p, i) }
        0x5b => { pokemonDataBiosGetStatusFaceMenuSpriteId(m, p, i) }
        0x5c => { (pokemonDataBiosGetColor(m, p, i) & 0xff) }
        0x5d => { pokemonDataBiosGetTypeName(m, p) }
        0x5e => { (pokemonDataBiosGetHeight(m, p) & 0xffff) }
        0x5f => { (pokemonDataBiosGetWeight(m, p) & 0xffff) }
        0x60 => { pokemonDataBiosGetDoc(m, p) }
        0x61 => { (pokemonDataBiosGetVoice(m, p) & 0xffff) }
        0x62 => { pokemonDataBiosGetMitaFlag(m, p) }
        0x63 => { pokemonDataBiosGetTukamaetaFlag(m, p) }
        0x64 => { (pokemonDataBiosGetNumZukan(m, p) & 0xffff) }
        0x65 => { (pokemonDataBiosGetNumPokemon(m, p) & 0xffff) }
        0x66 => { pokemonDataBiosGetPkxDataId(m, p) }
        0x68 => { (pokemonDataBiosGetKowaza(m, p, i) & 0xffff) }
        0x69 => { (FUN_8011e048(m, p, i) & 0xff) }
        0x6a => { (FUN_8011e030(m, p) & 0xffff) }
        0x6b => { (FUN_8011e018(m, p) & 0xff) }
        0x6c => { (FUN_8011e000(m, p) & 0xff) }
        0x6e => { (pokemonBiosGetPokemonDataId(m, p) & 0xffff) }
        0x6f => { pokemonBiosGetRnd(m, p) }
        0x70 => { pokemonBiosGetAttest(m, p) }
        0x71 => { (pokemonBiosGetCatchFloorId(m, p) & 0xffff) }
        0x72 => { (pokemonBiosGetCatchLevel(m, p) & 0xff) }
        0x73 => { (pokemonBiosGetCatchBallId(m, p) & 0xff) }
        0x74 => { (pokemonBiosGetCatchTrainerSex(m, p) & 0xff) }
        0x75 => { pokemonBiosGetCatchTrainerRnd(m, p) }
        0x76 => { pokemonBiosGetCatchTrainerNamePtr(m, p) }
        0x77 => { pokemonBiosGetNicknamePtr(m, p) }
        0x78 => { pokemonBiosGetNicknameOrgPtr(m, p) }
        0x79 => { pokemonBiosGetExp(m, p) }
        0x7a => { (pokemonBiosGetLevel(m, p) & 0xff) }
        0x7b => hand::pokemonGetStatus_case_7b(m, p, id, i), // uVar2 = pokemonGetStatus(param_1,0,0x83,0); uVar2 = countLeadingZeros(uVar2 & 0xffff); uVar2 = uVar2 >> 5 & 0xff;
        0x7c => { FUN_8011f474(m, p, i) }
        0x7d => { pokemonBiosGetConditionAmari(m, p) }
        0x7f => { (pokemonBiosGetPokemonWazaDataId(m, p, i) & 0xffff) }
        0x80 => { (pokemonBiosGetPokemonWazaPp(m, p, i) & 0xff) }
        0x81 => { (pokemonBiosGetPokemonWazaPpCount(m, p, i) & 0xff) }
        0x82 => { (pokemonBiosGetItemDataId(m, p) & 0xffff) }
        0x83 => { (pokemonBiosGetHp(m, p) & 0xffff) }
        0x87 => { (pokemonBiosGetMaxHp(m, p) & 0xffff) }
        0x88 => { (pokemonBiosGetPhyAtk(m, p) & 0xffff) }
        0x89 => { (pokemonBiosGetPhyDef(m, p) & 0xffff) }
        0x8a => { (pokemonBiosGetSpeAtk(m, p) & 0xffff) }
        0x8b => { (pokemonBiosGetSpeDef(m, p) & 0xffff) }
        0x8c => { (pokemonBiosGetNimbleness(m, p) & 0xffff) }
        0x8d => { (pokemonBiosGetMaxHpEffort(m, p) & 0xffff) }
        0x8e => { (pokemonBiosGetPhyAtkEffort(m, p) & 0xffff) }
        0x8f => { (pokemonBiosGetPhyDefEffort(m, p) & 0xffff) }
        0x90 => { (pokemonBiosGetSpeAtkEffort(m, p) & 0xffff) }
        0x91 => { (pokemonBiosGetSpeDefEffort(m, p) & 0xffff) }
        0x92 => { (pokemonBiosGetNimblenessEffort(m, p) & 0xffff) }
        0x93 => { (pokemonBiosGetMaxHpRnd(m, p) & 0xffff) }
        0x94 => { (pokemonBiosGetPhyAtkRnd(m, p) & 0xffff) }
        0x95 => { (pokemonBiosGetPhyDefRnd(m, p) & 0xffff) }
        0x96 => { (pokemonBiosGetSpeAtkRnd(m, p) & 0xffff) }
        0x97 => { (pokemonBiosGetSpeDefRnd(m, p) & 0xffff) }
        0x98 => { (pokemonBiosGetNimblenessRnd(m, p) & 0xffff) }
        0x99 => { (pokemonBiosGetFriend(m, p) & 0xffff) }
        0x9c => { (pokemonBiosGetStyle(m, p) & 0xff) }
        0x9d => { (pokemonBiosGetBeautiful(m, p) & 0xff) }
        0x9e => { (pokemonBiosGetCute(m, p) & 0xff) }
        0x9f => { (pokemonBiosGetClever(m, p) & 0xff) }
        0xa0 => { (pokemonBiosGetStrong(m, p) & 0xff) }
        0xa1 => { (pokemonBiosGetFur(m, p) & 0xff) }
        0xa3 => { (pokemonBiosGetChampRibbon(m, p) & 0xff) }
        0xa4 => { (pokemonBiosGetWinningRibbon(m, p) & 0xff) }
        0xa5 => { (pokemonBiosGetVictoryRibbon(m, p) & 0xff) }
        0xa6 => { (pokemonBiosGetBromideRibbon(m, p) & 0xff) }
        0xa7 => { (pokemonBiosGetGanbaRibbon(m, p) & 0xff) }
        0xa8 => { (pokemonBiosGetMarineRibbon(m, p) & 0xff) }
        0xa9 => { (pokemonBiosGetLandRibbon(m, p) & 0xff) }
        0xaa => { (pokemonBiosGetSkyRibbon(m, p) & 0xff) }
        0xab => { (pokemonBiosGetCountryRibbon(m, p) & 0xff) }
        0xac => { (pokemonBiosGetNationalRibbon(m, p) & 0xff) }
        0xad => { (pokemonBiosGetEarthRibbon(m, p) & 0xff) }
        0xae => { (pokemonBiosGetWorldRibbon(m, p) & 0xff) }
        0xaf => { (pokemonBiosGetAmariRibbon(m, p) & 0xff) }
        0xb0 => { (pokemonBiosGetStyleMedal(m, p) & 0xff) }
        0xb1 => { (pokemonBiosGetBeautifulMedal(m, p) & 0xff) }
        0xb2 => { (pokemonBiosGetCuteMedal(m, p) & 0xff) }
        0xb3 => { (pokemonBiosGetCleverMedal(m, p) & 0xff) }
        0xb4 => { (pokemonBiosGetStrongMedal(m, p) & 0xff) }
        0xb5 => { (pokemonBiosGetPokerus(m, p) & 0xff) }
        0xb6 => { (pokemonBiosGetTamagoFlag(m, p) & 0xff) }
        0xb7 => { (pokemonBiosGetTokuseiFlag(m, p) & 0xff) }
        0xb8 => { (pokemonBiosGetFuseiFlag(m, p) & 0xff) }
        0xb9 => { (pokemonBiosGetFlagAmari(m, p) & 0xff) }
        0xba => hand::pokemonGetStatus_case_ba(m, p, id, i), // if (param_1 == 0) { uVar2 = 2; else { uVar6 = pokemonGetStatus(param_1,0,0x6f,0); uVar10 = pokemonGetStatus(param_1,0,0x6e,0); uVar7 = pokemonGetStatus(0,uVar10,0x13,0); if (param_1 == 0) { bVar12 = 2; else { uVar10 = pokemonGetStatus(param_1,0,0x6e,0); uVar8 = pokemonGetStatus(0,uVar10,0x13,0); uVar9 = sexGetPokemonSexRaitoKotei(0); if (uVar8 == (uVar9 & 0xff)) { bVar12 = 0; else { uVar9 = sexGet
        0xbb => { (pokemonBiosGetPcboxMark(m, p) & 0xff) }
        0xbc => { (pokemonBiosGetMailId(m, p) & 0xff) }
        0xbd => { (pokemonBiosGetPara1Amari(m, p) & 0xffff) }
        0xbe => { (pokemonBiosGetAmari(m, p) & 0xffff) }
        0xbf => hand::pokemonGetStatus_case_bf(m, p, id, i), // uVar2 = pokemonGetStatus(param_1,0,0x6f,0); uVar2 = uVar2 % 0x19;
        0xc0 => hand::pokemonGetStatus_case_c0(m, p, id, i), // uVar10 = pokemonGetStatus(param_1,0,0x6e,0); uVar13 = pokemonGetStatus(0,uVar10,0x11,0); uVar2 = pokemonGetStatus(param_1,0,0x79,0); iVar1 = pokemonGrowDataBiosGetPtr(uVar13); if (iVar1 == 0) { uVar2 = 0; else { uVar4 = 1; do { uVar3 = pokemonGrowDataBiosGetExp(iVar1,uVar4 & 0xff); if (uVar2 < uVar3) break; uVar4 = uVar4 + 1; } while ((int)uVar4 < 0x65); uVar2 = uVar4 - 1 & 0xff;
        0xc1 => hand::pokemonGetStatus_case_c1(m, p, id, i), // if (param_1 == 0) { uVar2 = 0; else { uVar2 = pokemonGetStatus(param_1,0,0x75,0); uVar4 = pokemonGetStatus(param_1,0,0x6f,0); iVar1 = countLeadingZeros(uVar4 & 0xffff ^ uVar4 >> 0x10 ^ uVar2 >> 0x10 ^ uVar2 & 0xffff ^ 8); uVar2 = (uint)(8 << iVar1) >> 0x1f;
        0xc2 => { (pokemonBiosGetDarkFlag(m, p) & 0xff) }
        0xc3 => { (pokemonBiosGetDarkpokemonDataId(m, p) & 0xffff) }
        0xc4 => { pokemonBiosGetInitDp(m, p) }
        0xc5 => { pokemonBiosGetDp(m, p) }
        0xc6 => { pokemonBiosGetPoolExp(m, p) }
        0xc7 => { (pokemonBiosGetPoolFriend(m, p) & 0xffff) }
        0xc8 => { FUN_8011edc4(m, p, i) }
        0xc9 => { (pokemonBiosGetFightTrainerPokemonDataId(m, p) & 0xffff) }
        0xcb => { fightPokemonBiosGetMotoPokemonPtr(m, p) }
        0xcc => { fightPokemonBiosGetPokemonBuffPtr(m, p) }
        0xcd => { fightPokemonBiosGetFightJoutaiPtr(m, p, i) }
        0xce => { (fightPokemonBiosGetEntryId(m, p) as u16 as i16 as i32 as u32) }
        0xcf => { (fightPokemonBiosGetCatchEntryFlag(m, p) & 0xff) }
        0xd0 => { (fightPokemonBiosGetLevelUpFlag(m, p) & 0xff) }
        0xd1 => { (fightPokemonBiosGetDarkOutFlag(m, p) & 0xff) }
        0xd2 => { (fightPokemonBiosGetHokakuFlag(m, p) & 0xff) }
        0xd5 => { fightOutPokemonBiosGetMotoFightPokemonPtr(m, p) }
        0xd6 => { fightOutPokemonBiosGetFightPokemonPtr(m, p) }
        0xd7 => { fightOutPokemonBiosGetFightPokemonHensinBuffPtr(m, p) }
        0xd8 => { fightOutPokemonBiosGetFightoutJoutaiPtr(m, p, i) }
        0xd9 => { fightOutPokemonBiosGetFightWazaPtr(m, p) }
        0xda => { (fightOutPokemonGetUseWazaDataId(m, p) & 0xffff) }
        0xdb => { (fightOutPokemonGetMotoWazaDataId(m, p) & 0xffff) }
        0xdc => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaGetStatus(m, x, 0x0, 0x2c, 0x0) }
        0xdd => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaGetStatus(m, x, 0x0, 0x2b, 0x0) }
        0xde => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); (fightWazaIsJoutaiDataId(m, x, i) & 0xff) }
        0xe0 => hand::pokemonGetStatus_case_e0(m, p, id, i), // pokemonGetStatus(param_1,0,0xd9,0); uVar2 = fightWazaIsHit(); uVar2 = uVar2 & 0xff;
        0xe1 => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaGetStatus(m, x, 0x0, 0x2d, 0x0) }
        0xe2 => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaGetStatus(m, x, 0x0, 0x2f, 0x0) }
        0xe3 => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaGetStatus(m, x, 0x0, 0x29, 0x0) }
        0xe4 => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaGetStatus(m, x, 0x0, 0x2e, 0x0) }
        0xe5 => { fightOutPokemonBiosGetFightItemPtr(m, p) }
        0xe6 => { (fightOutPokemonBiosGetAbicntPhyAtk(m, p) & 0xff) }
        0xe7 => { (fightOutPokemonBiosGetAbicntPhyDef(m, p) & 0xff) }
        0xe8 => { (fightOutPokemonBiosGetAbicntSpeAtk(m, p) & 0xff) }
        0xe9 => { (fightOutPokemonBiosGetAbicntSpeDef(m, p) & 0xff) }
        0xea => { (fightOutPokemonBiosGetAbicntNimbleness(m, p) & 0xff) }
        0xeb => { (fightOutPokemonBiosGetAbicntAverage(m, p) & 0xff) }
        0xec => { (fightOutPokemonBiosGetAbicntAvoid(m, p) & 0xff) }
        0xed => { (fightOutPokemonBiosGetFightoutTurnCount(m, p) & 0xffff) }
        0xee => { fightOutPokemonBiosGetSequencePtr(m, p) }
        0xef => { (fightOutPokemonBiosGetSketchWazaDataId(m, p) & 0xffff) }
        0xf0 => { (fightOutPokemonBiosGetLastSelectWazaDataId(m, p) & 0xffff) }
        0xf1 => { (fightOutPokemonBiosGetLastUseWazaDataId(m, p) & 0xffff) }
        0xf2 => { (fightOutPokemonBiosGetLastReceiveWazaTargetDataId(m, p) & 0xffff) }
        0xf3 => { (fightOutPokemonBiosGetHitWazaDataId(m, p) & 0xffff) }
        0xf4 => { (fightOutPokemonBiosGetHitWazaZokuseiDataId(m, p) & 0xffff) }
        0xf5 => { (fightOutPokemonBiosGetGamanDamageValue(m, p) as u16 as i16 as i32 as u32) }
        0xf6 => { (fightOutPokemonBiosGetGamanDamageTargetId(m, p) & 0xffff) }
        0xf7 => { (fightOutPokemonBiosGetOumuWazaDataId(m, p) & 0xffff) }
        0xf8 => { fightOutPokemonBiosGetKeepFightWazaPtr(m, p) }
        0xf9 => { (fightOutPokemonBiosGetNamakeFlag(m, p) & 0xff) }
        0xfa => { (fightOutPokemonBiosGetUsedItemDataId(m, p) & 0xffff) }
        0xfb => { (fightOutPokemonBiosGetStockItemDataId(m, p) & 0xffff) }
        0xfc => { (fightOutPokemonBiosGetSuccessCnt(m, p) & 0xff) }
        0xfd => { (fightOutPokemonBiosGetMeetEnemyFightPokemonEntryId(m, p, i & 0xff) as u16 as i16 as i32 as u32) }
        0xfe => { fightOutPokemonBiosGetFightActionBuffPtr(m, p) }
        0xff => { (fightOutPokemonBiosGetZokuseiDataId(m, p, i & 0xff) & 0xffff) }
        0x100 => { (fightOutPokemonBiosGetTokuseiDataId(m, p) & 0xffff) }
        0x101 => { fightOutPokemonBiosGetWazaMenuCurPtr(m, p) }
        0x102 => { (fightOutPokemonBiosGetDamageAtkValue(m, p) as u16 as i16 as i32 as u32) }
        0x103 => { (fightOutPokemonBiosGetDamageAtkTargetId(m, p) & 0xffff) }
        0x104 => { (fightOutPokemonBiosGetDamageSpeValue(m, p) as u16 as i16 as i32 as u32) }
        0x105 => { (fightOutPokemonBiosGetDamageSpeTargetId(m, p) & 0xffff) }
        0x106 => { (fightOutPokemonBiosGetMahiNoAttackFlag(m, p) & 0xff) }
        0x107 => { (fightOutPokemonBiosGetKonranMyselfAttackFlag(m, p) & 0xff) }
        0x108 => { (fightOutPokemonBiosGetOutWazaKoukanaiFlag(m, p) & 0xff) }
        0x109 => { (fightOutPokemonBiosGetTameWazaFlag(m, p) & 0xff) }
        0x10a => { (fightOutPokemonBiosGetItemNigeruFlag(m, p) & 0xff) }
        0x10b => { (fightOutPokemonBiosGetHuuinNoAttackFlag(m, p) & 0xff) }
        0x10c => { (fightOutPokemonBiosGetMeroMeroNoAttackFlag(m, p) & 0xff) }
        0x10d => { (fightOutPokemonBiosGetKanashibariNoAttackFlag(m, p) & 0xff) }
        0x10e => { (fightOutPokemonBiosGetChouhatsuNoAttackFlag(m, p) & 0xff) }
        0x10f => { (fightOutPokemonBiosGetIchamonNoAttackFlag(m, p) & 0xff) }
        0x110 => { (fightOutPokemonBiosGetHirumuNoAttackFlag(m, p) & 0xff) }
        0x111 => { (fightOutPokemonBiosGetPassPpdecFlag(m, p) & 0xff) }
        0x112 => { (fightOutPokemonBiosGetFightActionFlag(m, p) & 0xff) }
        0x113 => { (fightOutPokemonBiosGetDoClearbodyFlag(m, p) & 0xff) }
        0x114 => { (fightOutPokemonBiosGetReceivesWazaHiraishinFlag(m, p) & 0xff) }
        0x115 => { (fightOutPokemonBiosGetVanishoffFlag(m, p) & 0xff) }
        0x116 => { (fightOutPokemonBiosGetDoIkakuFlag(m, p) & 0xff) }
        0x117 => { (fightOutPokemonBiosGetDoTraceFlag(m, p) & 0xff) }
        0x118 => { (fightOutPokemonBiosGetNoPressureFlag(m, p) & 0xff) }
        0x119 => { (fightOutPokemonBiosGetIrekaetaFlag(m, p) & 0xff) }
        0x11a => { (fightOutPokemonBiosGetItemKoraetaFlag(m, p) & 0xff) }
        0x11b => { fightOutPokemonBiosGetKaigaraDamageValue(m, p) }
        0x11c => { (fightOutPokemonBiosGetMyselfDamageAtkValue(m, p) as u16 as i16 as i32 as u32) }
        0x11d => { (fightOutPokemonBiosGetMyselfDamageAtkTargetId(m, p) & 0xffff) }
        0x11e => { (fightOutPokemonBiosGetMyselfDamageSpeValue(m, p) as u16 as i16 as i32 as u32) }
        0x11f => { (fightOutPokemonBiosGetMyselfDamageSpeTargetId(m, p) & 0xffff) }
        0x120 => { (fightOutPokemonBiosGetKizetuFlag(m, p) & 0xff) }
        0x121 => { (fightOutPokemonBiosGetIrekaeTargetEntryId(m, p) as u16 as i16 as i32 as u32) }
        0x122 => { fightOutPokemonBiosGetFightOutPokemonEnemyPtr(m, p, i) }
        0x123 => hand::pokemonGetStatus_case_123(m, p, id, i), // uVar2 = fightOutPokemonCheckFightOut(param_1); uVar2 = uVar2 & 0xff; return uVar2;
        _ => 0,
    }
}

/// `pokemonSetStatus` 0x801254B4.
pub fn pokemonSetStatus(m: &mut Mem, p: u32, id: u32, kind: u32, i: u32, v: u32) {
    let kind = kind & 0xffff;
    if kind == 0 || kind >= 0x124 { return; }
    let p = if kind < 0x6d { pokemonDataBiosGetPtr(m, id) } else { p };
    if p == 0 { return; }
    match kind {
        0x1 => { pokemonDataBiosSetName(m, p, v); }
        0x3 => { pokemonDataBiosSetBasisMaxHp(m, p, v & 0xffff); }
        0x4 => { pokemonDataBiosSetBasisPhyAtk(m, p, v & 0xffff); }
        0x5 => { pokemonDataBiosSetBasisPhyDef(m, p, v & 0xffff); }
        0x6 => { pokemonDataBiosSetBasisSpeAtk(m, p, v & 0xffff); }
        0x7 => { pokemonDataBiosSetBasisSpeDef(m, p, v & 0xffff); }
        0x8 => { pokemonDataBiosSetBasisNimbleness(m, p, v & 0xffff); }
        0xa => { pokemonDataBiosSetGiveMaxHpEffort(m, p, v & 0xffff); }
        0xb => { pokemonDataBiosSetGivePhyAtkEffort(m, p, v & 0xffff); }
        0xc => { pokemonDataBiosSetGivePhyDefEffort(m, p, v & 0xffff); }
        0xd => { pokemonDataBiosSetGiveSpeAtkEffort(m, p, v & 0xffff); }
        0xe => { pokemonDataBiosSetGiveSpeDefEffort(m, p, v & 0xffff); }
        0xf => { pokemonDataBiosSetGiveNimblenessEffort(m, p, v & 0xffff); }
        0x10 => { pokemonDataBiosSetGiveExp(m, p, v & 0xffff); }
        0x11 => { pokemonDataBiosSetGrowDataId(m, p, v & 0xff); }
        0x12 => { pokemonDataBiosSetGet(m, p, v & 0xff); }
        0x13 => { pokemonDataBiosSetSexRatio(m, p, v & 0xff); }
        0x14 => { pokemonDataBiosSetInitFriend(m, p, v & 0xffff); }
        0x15 => { pokemonDataBiosSetItemDataId(m, p, i, v & 0xffff); }
        0x16 => { pokemonDataBiosSetZokuseiDataId(m, p, i, v & 0xff); }
        0x17 => { pokemonDataBiosSetTokuseiDataId(m, p, i, v & 0xff); }
        0x19 => { pokemonDataBiosSetSinkaKind(m, p, i, v & 0xff); }
        0x1a => { pokemonDataBiosSetSinkaBuff(m, p, i, v & 0xffff); }
        0x1b => { pokemonDataBiosSetSinkaPokemonDataId(m, p, i, v & 0xffff); }
        0x1d => { pokemonDataBiosSetGetWazaLevel(m, p, i, v & 0xff); }
        0x1e => { pokemonDataBiosSetGetWazaDataId(m, p, i, v & 0xffff); }
        0x20 => { pokemonDataBiosSetWazaMcn(m, p, 0x0, v & 0xff); }
        0x21 => { pokemonDataBiosSetWazaMcn(m, p, 0x1, v & 0xff); }
        0x22 => { pokemonDataBiosSetWazaMcn(m, p, 0x2, v & 0xff); }
        0x23 => { pokemonDataBiosSetWazaMcn(m, p, 0x3, v & 0xff); }
        0x24 => { pokemonDataBiosSetWazaMcn(m, p, 0x4, v & 0xff); }
        0x25 => { pokemonDataBiosSetWazaMcn(m, p, 0x5, v & 0xff); }
        0x26 => { pokemonDataBiosSetWazaMcn(m, p, 0x6, v & 0xff); }
        0x27 => { pokemonDataBiosSetWazaMcn(m, p, 0x7, v & 0xff); }
        0x28 => { pokemonDataBiosSetWazaMcn(m, p, 0x8, v & 0xff); }
        0x29 => { pokemonDataBiosSetWazaMcn(m, p, 0x9, v & 0xff); }
        0x2a => { pokemonDataBiosSetWazaMcn(m, p, 0xa, v & 0xff); }
        0x2b => { pokemonDataBiosSetWazaMcn(m, p, 0xb, v & 0xff); }
        0x2c => { pokemonDataBiosSetWazaMcn(m, p, 0xc, v & 0xff); }
        0x2d => { pokemonDataBiosSetWazaMcn(m, p, 0xd, v & 0xff); }
        0x2e => { pokemonDataBiosSetWazaMcn(m, p, 0xe, v & 0xff); }
        0x2f => { pokemonDataBiosSetWazaMcn(m, p, 0xf, v & 0xff); }
        0x30 => { pokemonDataBiosSetWazaMcn(m, p, 0x10, v & 0xff); }
        0x31 => { pokemonDataBiosSetWazaMcn(m, p, 0x11, v & 0xff); }
        0x32 => { pokemonDataBiosSetWazaMcn(m, p, 0x12, v & 0xff); }
        0x33 => { pokemonDataBiosSetWazaMcn(m, p, 0x13, v & 0xff); }
        0x34 => { pokemonDataBiosSetWazaMcn(m, p, 0x14, v & 0xff); }
        0x35 => { pokemonDataBiosSetWazaMcn(m, p, 0x15, v & 0xff); }
        0x36 => { pokemonDataBiosSetWazaMcn(m, p, 0x16, v & 0xff); }
        0x37 => { pokemonDataBiosSetWazaMcn(m, p, 0x17, v & 0xff); }
        0x38 => { pokemonDataBiosSetWazaMcn(m, p, 0x18, v & 0xff); }
        0x39 => { pokemonDataBiosSetWazaMcn(m, p, 0x1a, v & 0xff); }
        0x3a => { pokemonDataBiosSetWazaMcn(m, p, 0x1b, v & 0xff); }
        0x3b => { pokemonDataBiosSetWazaMcn(m, p, 0x1c, v & 0xff); }
        0x3c => { pokemonDataBiosSetWazaMcn(m, p, 0x1d, v & 0xff); }
        0x3d => { pokemonDataBiosSetWazaMcn(m, p, 0x1e, v & 0xff); }
        0x3e => { pokemonDataBiosSetWazaMcn(m, p, 0x1f, v & 0xff); }
        0x3f => { pokemonDataBiosSetWazaMcn(m, p, 0x20, v & 0xff); }
        0x40 => { pokemonDataBiosSetWazaMcn(m, p, 0x21, v & 0xff); }
        0x41 => { pokemonDataBiosSetWazaMcn(m, p, 0x22, v & 0xff); }
        0x42 => { pokemonDataBiosSetWazaMcn(m, p, 0x23, v & 0xff); }
        0x43 => { pokemonDataBiosSetWazaMcn(m, p, 0x24, v & 0xff); }
        0x44 => { pokemonDataBiosSetWazaMcn(m, p, 0x25, v & 0xff); }
        0x45 => { pokemonDataBiosSetWazaMcn(m, p, 0x26, v & 0xff); }
        0x46 => { pokemonDataBiosSetWazaMcn(m, p, 0x27, v & 0xff); }
        0x47 => { pokemonDataBiosSetWazaMcn(m, p, 0x28, v & 0xff); }
        0x48 => { pokemonDataBiosSetWazaMcn(m, p, 0x29, v & 0xff); }
        0x49 => { pokemonDataBiosSetWazaMcn(m, p, 0x2a, v & 0xff); }
        0x4a => { pokemonDataBiosSetWazaMcn(m, p, 0x2b, v & 0xff); }
        0x4b => { pokemonDataBiosSetWazaMcn(m, p, 0x2c, v & 0xff); }
        0x4c => { pokemonDataBiosSetWazaMcn(m, p, 0x2d, v & 0xff); }
        0x4d => { pokemonDataBiosSetWazaMcn(m, p, 0x2e, v & 0xff); }
        0x4e => { pokemonDataBiosSetWazaMcn(m, p, 0x2f, v & 0xff); }
        0x4f => { pokemonDataBiosSetWazaMcn(m, p, 0x30, v & 0xff); }
        0x50 => { pokemonDataBiosSetWazaMcn(m, p, 0x31, v & 0xff); }
        0x51 => { pokemonDataBiosSetWazaMcn(m, p, 0x32, v & 0xff); }
        0x52 => { pokemonDataBiosSetWazaMcn(m, p, 0x33, v & 0xff); }
        0x53 => { pokemonDataBiosSetWazaMcn(m, p, 0x34, v & 0xff); }
        0x54 => { pokemonDataBiosSetWazaMcn(m, p, 0x35, v & 0xff); }
        0x55 => { pokemonDataBiosSetWazaMcn(m, p, 0x36, v & 0xff); }
        0x56 => { pokemonDataBiosSetWazaMcn(m, p, 0x37, v & 0xff); }
        0x57 => { pokemonDataBiosSetWazaMcn(m, p, 0x38, v & 0xff); }
        0x58 => { pokemonDataBiosSetWazaMcn(m, p, 0x39, v & 0xff); }
        0x59 => { pokemonDataBiosSetWazaMcn(m, p, 0x3a, v & 0xff); }
        0x5a => { pokemonDataBiosSetPokebodyId(m, p, i, v); }
        0x5b => { pokemonDataBiosSetStatusFaceMenuSpriteId(m, p, i, v & 0xffff); }
        0x5c => { pokemonDataBiosSetColor(m, p, i, v & 0xff); }
        0x5d => { pokemonDataBiosSetTypeName(m, p, v); }
        0x5e => { pokemonDataBiosSetHeight(m, p, v & 0xffff); }
        0x5f => { pokemonDataBiosSetWeight(m, p, v & 0xffff); }
        0x60 => { pokemonDataBiosSetDoc(m, p, v); }
        0x61 => { pokemonDataBiosSetVoice(m, p, v & 0xffff); }
        0x62 => { pokemonDataBiosSetMitaFlag(m, p, v); }
        0x63 => { pokemonDataBiosSetTukamaetaFlag(m, p, v); }
        0x64 => { pokemonDataBiosSetNumZukan(m, p, v & 0xffff); }
        0x65 => { pokemonDataBiosSetNumPokemon(m, p, v & 0xffff); }
        0x66 => { pokemonDataBiosSetPkxDataId(m, p, v); }
        0x68 => { pokemonDataBiosSetKowaza(m, p, i, v & 0xffff); }
        0x69 => { FUN_8011ced0(m, p, i, v & 0xff); }
        0x6a => { FUN_8011cec0(m, p, v & 0xffff); }
        0x6b => { FUN_8011ceb0(m, p, v & 0xff); }
        0x6c => { FUN_8011cea0(m, p, v & 0xff); }
        0x6e => { pokemonBiosSetPokemonDataId(m, p, v & 0xffff); }
        0x6f => { pokemonBiosSetRnd(m, p, v); }
        0x71 => { pokemonBiosSetCatchFloorId(m, p, v & 0xffff); }
        0x72 => { pokemonBiosSetCatchLevel(m, p, v & 0xff); }
        0x73 => { pokemonBiosSetCatchBallId(m, p, v & 0xff); }
        0x74 => { pokemonBiosSetCatchTrainerSex(m, p, v & 0xff); }
        0x75 => { pokemonBiosSetCatchTrainerRnd(m, p, v); }
        0x76 => { pokemonBiosSetCatchTrainerNamePtr(m, p, v); }
        0x77 => { pokemonBiosSetNicknamePtr(m, p, v); }
        0x78 => { pokemonBiosSetNicknameOrgPtr(m, p, v); }
        0x79 => { pokemonBiosSetExp(m, p, v); }
        0x7a => { pokemonBiosSetLevel(m, p, v & 0xff); }
        0x7c => hand::pokemonSetStatus_case_7c(m, p, id, i, v), // puVar1 = (undefined4 *)pokemonGetStatus(param_1,0,0x7c,0); if (puVar1 != (undefined4 *)0x0) { uVar2 = param_5[1]; *puVar1 = *param_5; puVar1[1] = uVar2; uVar2 = param_5[3]; puVar1[2] = param_5[2]; puVar1[3] = uVar2;
        0x7d => { pokemonBiosSetConditionAmari(m, p, v); }
        0x7f => { pokemonBiosSetPokemonWazaDataId(m, p, i, v & 0xffff); }
        0x80 => { pokemonBiosSetPokemonWazaPp(m, p, i, v & 0xff); }
        0x81 => { pokemonBiosSetPokemonWazaPpCount(m, p, i, v & 0xff); }
        0x82 => { pokemonBiosSetItemDataId(m, p, v & 0xffff); }
        0x83 => { pokemonBiosSetHp(m, p, v & 0xffff); }
        0x87 => { pokemonBiosSetMaxHp(m, p, v & 0xffff); }
        0x88 => { pokemonBiosSetPhyAtk(m, p, v & 0xffff); }
        0x89 => { pokemonBiosSetPhyDef(m, p, v & 0xffff); }
        0x8a => { pokemonBiosSetSpeAtk(m, p, v & 0xffff); }
        0x8b => { pokemonBiosSetSpeDef(m, p, v & 0xffff); }
        0x8c => { pokemonBiosSetNimbleness(m, p, v & 0xffff); }
        0x8d => { pokemonBiosSetMaxHpEffort(m, p, v & 0xffff); }
        0x8e => { pokemonBiosSetPhyAtkEffort(m, p, v & 0xffff); }
        0x8f => { pokemonBiosSetPhyDefEffort(m, p, v & 0xffff); }
        0x90 => { pokemonBiosSetSpeAtkEffort(m, p, v & 0xffff); }
        0x91 => { pokemonBiosSetSpeDefEffort(m, p, v & 0xffff); }
        0x92 => { pokemonBiosSetNimblenessEffort(m, p, v & 0xffff); }
        0x93 => { pokemonBiosSetMaxHpRnd(m, p, v & 0xffff); }
        0x94 => { pokemonBiosSetPhyAtkRnd(m, p, v & 0xffff); }
        0x95 => { pokemonBiosSetPhyDefRnd(m, p, v & 0xffff); }
        0x96 => { pokemonBiosSetSpeAtkRnd(m, p, v & 0xffff); }
        0x97 => { pokemonBiosSetSpeDefRnd(m, p, v & 0xffff); }
        0x98 => { pokemonBiosSetNimblenessRnd(m, p, v & 0xffff); }
        0x99 => { pokemonBiosSetFriend(m, p, v & 0xffff); }
        0x9c => { pokemonBiosSetStyle(m, p, v & 0xff); }
        0x9d => { pokemonBiosSetBeautiful(m, p, v & 0xff); }
        0x9e => { pokemonBiosSetCute(m, p, v & 0xff); }
        0x9f => { pokemonBiosSetClever(m, p, v & 0xff); }
        0xa0 => { pokemonBiosSetStrong(m, p, v & 0xff); }
        0xa1 => { pokemonBiosSetFur(m, p, v & 0xff); }
        0xa3 => { pokemonBiosSetChampRibbon(m, p, v & 0xff); }
        0xa4 => { pokemonBiosSetWinningRibbon(m, p, v & 0xff); }
        0xa5 => { pokemonBiosSetVictoryRibbon(m, p, v & 0xff); }
        0xa6 => { pokemonBiosSetBromideRibbon(m, p, v & 0xff); }
        0xa7 => { pokemonBiosSetGanbaRibbon(m, p, v & 0xff); }
        0xa8 => { pokemonBiosSetMarineRibbon(m, p, v & 0xff); }
        0xa9 => { pokemonBiosSetLandRibbon(m, p, v & 0xff); }
        0xaa => { pokemonBiosSetSkyRibbon(m, p, v & 0xff); }
        0xab => { pokemonBiosSetCountryRibbon(m, p, v & 0xff); }
        0xac => { pokemonBiosSetNationalRibbon(m, p, v & 0xff); }
        0xad => { pokemonBiosSetEarthRibbon(m, p, v & 0xff); }
        0xae => { pokemonBiosSetWorldRibbon(m, p, v & 0xff); }
        0xaf => { pokemonBiosSetAmariRibbon(m, p, v & 0xff); }
        0xb0 => { pokemonBiosSetStyleMedal(m, p, v & 0xff); }
        0xb1 => { pokemonBiosSetBeautifulMedal(m, p, v & 0xff); }
        0xb2 => { pokemonBiosSetCuteMedal(m, p, v & 0xff); }
        0xb3 => { pokemonBiosSetCleverMedal(m, p, v & 0xff); }
        0xb4 => { pokemonBiosSetStrongMedal(m, p, v & 0xff); }
        0xb5 => { pokemonBiosSetPokerus(m, p, v & 0xff); }
        0xb6 => { pokemonBiosSetTamagoFlag(m, p, v & 0xff); }
        0xb7 => { pokemonBiosSetTokuseiFlag(m, p, v & 0xff); }
        0xb8 => { pokemonBiosSetFuseiFlag(m, p, v & 0xff); }
        0xb9 => { pokemonBiosSetFlagAmari(m, p, v & 0xff); }
        0xbb => { pokemonBiosSetPcboxMark(m, p, v & 0xff); }
        0xbc => { pokemonBiosSetMailId(m, p, v & 0xff); }
        0xbd => { pokemonBiosSetPara1Amari(m, p, v & 0xffff); }
        0xbe => { pokemonBiosSetAmari(m, p, v & 0xffff); }
        0xc3 => { pokemonBiosSetDarkpokemonDataId(m, p, v & 0xffff); }
        0xc5 => { pokemonBiosSetDp(m, p, v); }
        0xc6 => { pokemonBiosSetPoolExp(m, p, v); }
        0xc7 => { pokemonBiosSetPoolFriend(m, p, v & 0xffff); }
        0xc9 => { pokemonBiosSetFightTrainerPokemonDataId(m, p, v & 0xffff); }
        0xcb => { fightPokemonBiosSetMotoPokemonPtr(m, p, v); }
        0xcd => hand::pokemonSetStatus_case_cd(m, p, id, i, v), // puVar1 = (undefined4 *)pokemonGetStatus(param_1,0,0xcd,0); if (puVar1 != (undefined4 *)0x0) { uVar2 = param_5[1]; *puVar1 = *param_5; puVar1[1] = uVar2; uVar2 = param_5[3]; puVar1[2] = param_5[2]; puVar1[3] = uVar2;
        0xce => { fightPokemonBiosSetEntryId(m, p, (v as u16 as i16 as i32 as u32)); }
        0xcf => { fightPokemonBiosSetCatchEntryFlag(m, p, v & 0xff); }
        0xd0 => { fightPokemonBiosSetLevelUpFlag(m, p, v & 0xff); }
        0xd1 => { fightPokemonBiosSetDarkOutFlag(m, p, v & 0xff); }
        0xd2 => { fightPokemonBiosSetHokakuFlag(m, p, v & 0xff); }
        0xd5 => { fightOutPokemonBiosSetMotoFightPokemonPtr(m, p, v); }
        0xd6 => { fightOutPokemonBiosSetFightPokemonPtr(m, p, v); }
        0xda => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaSetStatus(m, x, 0x0, 0x28, 0x0, v); }
        0xdb => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaSetStatus(m, x, 0x0, 0x27, 0x0, v); }
        0xdc => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaSetStatus(m, x, 0x0, 0x2c, 0x0, v); }
        0xdd => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaSetStatus(m, x, 0x0, 0x2b, 0x0, v); }
        0xde => hand::pokemonSetStatus_case_de(m, p, id, i, v), // uVar2 = pokemonGetStatus(param_1,0,0xd9,0); cVar3 = fightWazaCheckWriteJoutaiDataId(uVar2,(uint)param_5 & 0xffff); if (cVar3 == '\x02') { fightWazaWriteJoutaiDataId(uVar2,(uint)param_5 & 0xffff,0);
        0xdf => hand::pokemonSetStatus_case_df(m, p, id, i, v), // uVar2 = pokemonGetStatus(param_1,0,0xd9,0); if (((uint)param_5 & 0xffff) == 0) { fightWazaInitJoutai(); else { cVar3 = fightWazaIsJoutaiDataId(); if (cVar3 == '\x01') { fightWazaInitJoutaiDataId(uVar2,(uint)param_5 & 0xffff);
        0xe1 => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaSetStatus(m, x, 0x0, 0x2d, 0x0, v); }
        0xe2 => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaSetStatus(m, x, 0x0, 0x2f, 0x0, v & 0xffff); }
        0xe3 => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaSetStatus(m, x, 0x0, 0x29, 0x0, v); }
        0xe4 => { let x = pokemonGetStatus(m, p, 0x0, 0xd9, 0x0); wazaSetStatus(m, x, 0x0, 0x2e, 0x0, v); }
        0xe6 => { fightOutPokemonBiosSetAbicntPhyAtk(m, p, v & 0xff); }
        0xe7 => { fightOutPokemonBiosSetAbicntPhyDef(m, p, v & 0xff); }
        0xe8 => { fightOutPokemonBiosSetAbicntSpeAtk(m, p, v & 0xff); }
        0xe9 => { fightOutPokemonBiosSetAbicntSpeDef(m, p, v & 0xff); }
        0xea => { fightOutPokemonBiosSetAbicntNimbleness(m, p, v & 0xff); }
        0xeb => { fightOutPokemonBiosSetAbicntAverage(m, p, v & 0xff); }
        0xec => { fightOutPokemonBiosSetAbicntAvoid(m, p, v & 0xff); }
        0xed => { fightOutPokemonBiosSetFightoutTurnCount(m, p, v & 0xffff); }
        0xee => { fightOutPokemonBiosSetSequencePtr(m, p, v); }
        0xef => { fightOutPokemonBiosSetSketchWazaDataId(m, p, v & 0xffff); }
        0xf0 => { fightOutPokemonBiosSetLastSelectWazaDataId(m, p, v & 0xffff); }
        0xf1 => { fightOutPokemonBiosSetLastUseWazaDataId(m, p, v & 0xffff); }
        0xf2 => { fightOutPokemonBiosSetLastReceiveWazaTargetDataId(m, p, v & 0xffff); }
        0xf3 => { fightOutPokemonBiosSetHitWazaDataId(m, p, v & 0xffff); }
        0xf4 => { fightOutPokemonBiosSetHitWazaZokuseiDataId(m, p, v & 0xffff); }
        0xf5 => { fightOutPokemonBiosSetGamanDamageValue(m, p, (v as u16 as i16 as i32 as u32)); }
        0xf6 => { fightOutPokemonBiosSetGamanDamageTargetId(m, p, v & 0xffff); }
        0xf7 => { fightOutPokemonBiosSetOumuWazaDataId(m, p, v & 0xffff); }
        0xf9 => { fightOutPokemonBiosSetNamakeFlag(m, p, v & 0xff); }
        0xfa => { fightOutPokemonBiosSetUsedItemDataId(m, p, v & 0xffff); }
        0xfb => { fightOutPokemonBiosSetStockItemDataId(m, p, v & 0xffff); }
        0xfc => { fightOutPokemonBiosSetSuccessCnt(m, p, v & 0xff); }
        0xfd => hand::pokemonSetStatus_case_fd(m, p, id, i, v), // fightOutPokemonBiosSetMeetEnemyFightPokemonEntryId (param_1,param_4 & 0xff,(int)(char)param_5);
        0xff => { fightOutPokemonBiosSetZokuseiDataId(m, p, i & 0xff, v & 0xffff); }
        0x100 => { fightOutPokemonBiosSetTokuseiDataId(m, p, v & 0xffff); }
        0x102 => { fightOutPokemonBiosSetDamageAtkValue(m, p, (v as u16 as i16 as i32 as u32)); }
        0x103 => { fightOutPokemonBiosSetDamageAtkTargetId(m, p, v & 0xffff); }
        0x104 => { fightOutPokemonBiosSetDamageSpeValue(m, p, (v as u16 as i16 as i32 as u32)); }
        0x105 => { fightOutPokemonBiosSetDamageSpeTargetId(m, p, v & 0xffff); }
        0x106 => { fightOutPokemonBiosSetMahiNoAttackFlag(m, p, v & 0xff); }
        0x107 => { fightOutPokemonBiosSetKonranMyselfAttackFlag(m, p, v & 0xff); }
        0x108 => { fightOutPokemonBiosSetOutWazaKoukanaiFlag(m, p, v & 0xff); }
        0x109 => { fightOutPokemonBiosSetTameWazaFlag(m, p, v & 0xff); }
        0x10a => { fightOutPokemonBiosSetItemNigeruFlag(m, p, v & 0xff); }
        0x10b => { fightOutPokemonBiosSetHuuinNoAttackFlag(m, p, v & 0xff); }
        0x10c => { fightOutPokemonBiosSetMeroMeroNoAttackFlag(m, p, v & 0xff); }
        0x10d => { fightOutPokemonBiosSetKanashibariNoAttackFlag(m, p, v & 0xff); }
        0x10e => { fightOutPokemonBiosSetChouhatsuNoAttackFlag(m, p, v & 0xff); }
        0x10f => { fightOutPokemonBiosSetIchamonNoAttackFlag(m, p, v & 0xff); }
        0x110 => { fightOutPokemonBiosSetHirumuNoAttackFlag(m, p, v & 0xff); }
        0x111 => { fightOutPokemonBiosSetPassPpdecFlag(m, p, v & 0xff); }
        0x112 => { fightOutPokemonBiosSetFightActionFlag(m, p, v & 0xff); }
        0x113 => { fightOutPokemonBiosSetDoClearbodyFlag(m, p, v & 0xff); }
        0x114 => { fightOutPokemonBiosSetReceivesWazaHiraishinFlag(m, p, v & 0xff); }
        0x115 => { fightOutPokemonBiosSetVanishoffFlag(m, p, v & 0xff); }
        0x116 => { fightOutPokemonBiosSetDoIkakuFlag(m, p, v & 0xff); }
        0x117 => { fightOutPokemonBiosSetDoTraceFlag(m, p, v & 0xff); }
        0x118 => { fightOutPokemonBiosSetNoPressureFlag(m, p, v & 0xff); }
        0x119 => { fightOutPokemonBiosSetIrekaetaFlag(m, p, v & 0xff); }
        0x11a => { fightOutPokemonBiosSetItemKoraetaFlag(m, p, v & 0xff); }
        0x11b => { fightOutPokemonBiosSetKaigaraDamageValue(m, p, v); }
        0x11c => { fightOutPokemonBiosSetMyselfDamageAtkValue(m, p, (v as u16 as i16 as i32 as u32)); }
        0x11d => { fightOutPokemonBiosSetMyselfDamageAtkTargetId(m, p, v & 0xffff); }
        0x11e => { fightOutPokemonBiosSetMyselfDamageSpeValue(m, p, (v as u16 as i16 as i32 as u32)); }
        0x11f => { fightOutPokemonBiosSetMyselfDamageSpeTargetId(m, p, v & 0xffff); }
        0x120 => { fightOutPokemonBiosSetKizetuFlag(m, p, v & 0xff); }
        0x121 => hand::pokemonSetStatus_case_121(m, p, id, i, v), // fightOutPokemonBiosSetIrekaeTargetEntryId(param_1,(int)sVar4); return;
        _ => {}
    }
}

/// `wazaGetStatus` 0x8011BEB4. Kinds 1..0x24 read move data (`id`), 0x25..0x34 a fight move `p`.
pub fn wazaGetStatus(m: &Mem, p: u32, id: u32, kind: u32, i: u32) -> u32 {
    let kind = kind & 0xffff;
    if kind == 0 || kind > 0x34 { return 0; }
    let p = if kind < 0x25 { wazaDataBiosGetPtr(m, id) } else { p };
    if p == 0 { return 0; }
    match kind {
        0x1 => { wazaDataBiosGetName(m, p) }
        0x2 => { (wazaDataBiosGetPp(m, p) & 0xff) }
        0x3 => { (wazaDataBiosGetZokuseiDataId(m, p) & 0xff) }
        0x4 => { (wazaDataBiosGetPri(m, p) as u8 as i8 as i32 as u32) }
        0x5 => { (wazaDataBiosGetRangeId(m, p) & 0xff) }
        0x6 => { (wazaDataBiosGetAvg(m, p) & 0xff) }
        0x7 => { (wazaDataBiosGetIryoku(m, p) as u16 as i16 as i32 as u32) }
        0x8 => { (wazaDataBiosGetFightKoukaDataId(m, p) & 0xffff) }
        0x9 => { (wazaDataBiosGetSeqId(m, p) & 0xffff) }
        0xa => { wazaDataBiosGetFightAttackTunagiMsgId(m, p) }
        0xb => { wazaDataBiosGetFightAttackMsgId(m, p) }
        0xc => { (wazaDataBiosGetAddFightKoukaAvg(m, p) & 0xff) }
        0xd => { (wazaDataBiosGetDageki(m, p) & 0xff) }
        0xe => { (wazaDataBiosGetMamoru(m, p) & 0xff) }
        0xf => { (wazaDataBiosGetMajikku(m, p) & 0xff) }
        0x10 => { (wazaDataBiosGetYokodori(m, p) & 0xff) }
        0x11 => { (wazaDataBiosGetOumugaesi(m, p) & 0xff) }
        0x12 => { (wazaDataBiosGetOujanosirusi(m, p) & 0xff) }
        0x13 => { (wazaDataBiosGetYubiwohuru(m, p) & 0xff) }
        0x14 => { (wazaDataBiosGetMonomane(m, p) & 0xff) }
        0x15 => { (wazaDataBiosGetNekonote(m, p) & 0xff) }
        0x16 => { (wazaDataBiosGetNegoto(m, p) & 0xff) }
        0x17 => { (wazaDataBiosGetBouon(m, p) & 0xff) }
        0x18 => { (wazaDataBiosGetPressure(m, p) & 0xff) }
        0x19 => { (wazaDataBiosGetHidenFlag(m, p) & 0xff) }
        0x1a => { (wazaDataBiosGetTypeId(m, p, i) & 0xff) }
        0x1b => { (wazaDataBiosGetRiskFlag(m, p) & 0xff) }
        0x1c => { wazaDataBiosGetFightTrainerAiWazaValueFuncPtr(m, p) }
        0x1d => { wazaDataBiosGetFightTrainerAiWazaHitFuncPtr(m, p) }
        0x1e => { wazaDataBiosGetFightTrainerAiWazaDamageFuncPtr(m, p) }
        0x1f => { wazaDataBiosGetWazawzxdataId(m, p) }
        0x20 => { wazaDataBiosGetFightWazaWzxTypeFuncPtr(m, p) }
        0x21 => { wazaDataBiosGetFightWazaWzxVariationFuncPtr(m, p) }
        0x22 => { wazaDataBiosGetDoc(m, p) }
        0x23 => { (FUN_8011c7a8(m, p) & 0xff) }
        0x24 => { (FUN_8011c790(m, p) & 0xff) }
        0x26 => { (fightWazaBiosGetWazaBanme(m, p) as u8 as i8 as i32 as u32) }
        0x27 => { (fightWazaBiosGetMotoWazaDataId(m, p) & 0xffff) }
        0x28 => { (fightWazaBiosGetUseWazaDataId(m, p) & 0xffff) }
        0x29 => { (fightWazaBiosGetTargetDataId(m, p) & 0xffff) }
        0x2a => { fightWazaBiosGetJoutaiPtr(m, p, i) }
        0x2b => { (fightWazaBiosGetCritical(m, p) & 0xff) }
        0x2c => { (fightWazaBiosGetDamageValue(m, p) & 0xff) }
        0x2d => { fightWazaBiosGetDamage(m, p) }
        0x2e => { fightWazaBiosGetHitDamage(m, p) }
        0x2f => { (fightWazaBiosGetIryoku(m, p) & 0xffff) }
        0x30 => { (fightWazaBiosGetZokusei(m, p) & 0xffff) }
        0x31 => { (fightWazaBiosGetKaisuu(m, p) & 0xff) }
        0x32 => { (fightWazaBiosGetAutoMakeFlag(m, p) & 0xff) }
        0x33 => { let x = wazaGetStatus(m, p, 0x0, 0x27, 0x0); wazaGetStatus(m, 0x0, x, 0x3, 0x0) }
        0x34 => hand::wazaGetStatus_case_34(m, p, id, i), // uVar3 = wazaGetStatus(param_1,0,0x28,0); uVar1 = wazaGetStatus(0,uVar3,3,0); return uVar1;
        _ => 0,
    }
}

/// `wazaSetStatus` 0x8011BBD8.
pub fn wazaSetStatus(m: &mut Mem, p: u32, id: u32, kind: u32, i: u32, v: u32) {
    let kind = kind & 0xffff;
    if kind == 0 || kind >= 0x35 { return; }
    let p = if kind < 0x25 { wazaDataBiosGetPtr(m, id) } else { p };
    if p == 0 { return; }
    match kind {
        0x1 => { wazaDataBiosSetName(m, p, v); }
        0x2 => { wazaDataBiosSetPp(m, p, v & 0xff); }
        0x3 => { wazaDataBiosSetZokuseiDataId(m, p, v & 0xff); }
        0x4 => { wazaDataBiosSetPri(m, p, (v as u8 as i8 as i32 as u32)); }
        0x5 => { wazaDataBiosSetRangeId(m, p, v & 0xff); }
        0x6 => { wazaDataBiosSetAvg(m, p, v & 0xff); }
        0x7 => { wazaDataBiosSetIryoku(m, p, (v as u16 as i16 as i32 as u32)); }
        0x8 => { wazaDataBiosSetFightKoukaDataId(m, p, v & 0xffff); }
        0x9 => { wazaDataBiosSetSeqId(m, p, v & 0xffff); }
        0xa => { wazaDataBiosSetFightAttackMsgId(m, p, v & 0xffff); }
        0xb => { wazaDataBiosSetFightAttackMsgId(m, p, v & 0xffff); }
        0xc => { wazaDataBiosSetAddFightKoukaAvg(m, p, v & 0xff); }
        0xd => { wazaDataBiosSetDageki(m, p, v & 0xff); }
        0xe => { wazaDataBiosSetMamoru(m, p, v & 0xff); }
        0xf => { wazaDataBiosSetMajikku(m, p, v & 0xff); }
        0x10 => { wazaDataBiosSetYokodori(m, p, v & 0xff); }
        0x11 => { wazaDataBiosSetOumugaesi(m, p, v & 0xff); }
        0x12 => { wazaDataBiosSetOujanosirusi(m, p, v & 0xff); }
        0x13 => { wazaDataBiosSetYubiwohuru(m, p, v & 0xff); }
        0x14 => { wazaDataBiosSetMonomane(m, p, v & 0xff); }
        0x15 => { wazaDataBiosSetNekonote(m, p, v & 0xff); }
        0x16 => { wazaDataBiosSetNegoto(m, p, v & 0xff); }
        0x17 => { wazaDataBiosSetBouon(m, p, v & 0xff); }
        0x18 => { wazaDataBiosSetPressure(m, p, v & 0xff); }
        0x19 => { wazaDataBiosSetHidenFlag(m, p, v & 0xff); }
        0x1a => { wazaDataBiosSetTypeId(m, p, i, v & 0xff); }
        0x1b => { wazaDataBiosSetRiskFlag(m, p, v & 0xff); }
        0x1c => { wazaDataBiosSetFightTrainerAiWazaValueFuncPtr(m, p, v); }
        0x1d => { wazaDataBiosSetFightTrainerAiWazaHitFuncPtr(m, p, v); }
        0x1e => { wazaDataBiosSetFightTrainerAiWazaDamageFuncPtr(m, p, v); }
        0x1f => { wazaDataBiosSetWazawzxdataId(m, p, v); }
        0x20 => { wazaDataBiosSetFightWazaWzxTypeFuncPtr(m, p, v); }
        0x21 => { wazaDataBiosSetFightWazaWzxVariationFuncPtr(m, p, v); }
        0x22 => { wazaDataBiosSetDoc(m, p, v); }
        0x23 => { FUN_8011c5f0(m, p, v & 0xff); }
        0x24 => { FUN_8011c5e0(m, p, v & 0xff); }
        0x26 => { fightWazaBiosSetWazaBanme(m, p, (v as u8 as i8 as i32 as u32)); }
        0x27 => { fightWazaBiosSetMotoWazaDataId(m, p, v & 0xffff); }
        0x28 => { fightWazaBiosSetUseWazaDataId(m, p, v & 0xffff); }
        0x29 => { fightWazaBiosSetTargetDataId(m, p, v & 0xffff); }
        0x2b => { fightWazaBiosSetCritical(m, p, v & 0xff); }
        0x2c => { fightWazaBiosSetDamageValue(m, p, v & 0xff); }
        0x2d => { fightWazaBiosSetDamage(m, p, v); }
        0x2e => { fightWazaBiosSetHitDamage(m, p, v); }
        0x2f => { fightWazaBiosSetIryoku(m, p, v & 0xffff); }
        0x30 => { fightWazaBiosSetZokusei(m, p, v & 0xffff); }
        0x31 => { fightWazaBiosSetKaisuu(m, p, v & 0xff); }
        0x32 => hand::wazaSetStatus_case_32(m, p, id, i, v), // fightWazaBiosSetAutoMakeFlag(param_1,param_5 & 0xff); return;
        _ => {}
    }
}
