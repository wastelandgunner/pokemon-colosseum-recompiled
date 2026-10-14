//! Battle data tables, laid out exactly as the game's `*DataBios*` getters read them.
//!
//! - Moves (`wazaDataBiosGetPtr` 0x8011... : 0x38-byte records): common_rel table symbol 62/63.
//! - Species (`pokemonDataBiosGetPtr`: 0x11C-byte records): common_rel table symbol 68/69.
//! - Items (`itemDataBiosGetPtr`): main.dol, id → row u16 index at 0x803681E8 (0x224 ids),
//!   0x28-byte rows at 0x80363CE8 (0x18D rows).
//! - Type chart (`FUN_8010C508`, used by `zokuseiGetWazaJoutai`): main.dol 0x8035B508, 18 rows of 0x2C.
//!
//! Field names follow the getters (Japanese romanization kept where the game uses it), with the
//! English meaning in the doc comment.

use crate::be::{u16_at, u32_at, u8_at};
use crate::common::Common;
use crate::dol::Dol;
use crate::Result;

pub const SYM_MOVES: usize = 62;
pub const SYM_SPECIES: usize = 68;
pub const MOVE_SIZE: usize = 0x38;
pub const SPECIES_SIZE: usize = 0x11C;
pub const ITEM_INDEX: u32 = 0x803681E8;
pub const ITEM_ROWS: u32 = 0x80363CE8;
pub const ITEM_SIZE: usize = 0x28;
pub const ITEM_IDS: usize = 0x224;
pub const ITEM_ROW_COUNT: usize = 0x18D;
pub const TYPE_CHART: u32 = 0x8035B508;
pub const TYPE_COUNT: usize = 18;

/// One move (`waza`). Offsets in brackets.
#[derive(Clone, Debug)]
pub struct Move {
    /// [0x00] i8 priority (`Pri`)
    pub pri: i8,
    /// [0x01] base PP (`Pp`)
    pub pp: u8,
    /// [0x02] type (`ZokuseiDataId`)
    pub zokusei: u8,
    /// [0x03] target range (`RangeId`)
    pub range: u8,
    /// [0x04] accuracy (`Avg`)
    pub avg: u8,
    /// [0x05] secondary effect chance (`AddFightKoukaAvg`)
    pub add_kouka_avg: u8,
    /// [0x06..0x13] flags: contact (`Dageki`), Protect (`Mamoru`), Magic Coat (`Majikku`), Snatch (`Yokodori`),
    /// Mirror Move (`Oumugaesi`), King's Rock (`Oujanosirusi`), Metronome (`Yubiwohuru`), Mimic (`Monomane`),
    /// Assist (`Nekonote`), Sleep Talk (`Negoto`), sound (`Bouon`), Pressure (`Pressure`), HM (`HidenFlag`), `RiskFlag`.
    pub flags: [u8; 14],
    /// [0x16] base power (`Iryoku`, s16)
    pub iryoku: i16,
    /// [0x18] effect id (`FightKoukaDataId`): index into `ws::EFFECT_TABLE`
    pub kouka: u16,
    /// [0x1A] animation sequence (`SeqId`)
    pub seq: u16,
    /// [0x1C] index into the per-move function tables (wzx type/variation, AI value/hit/damage; 0x8035C430, 5 ptrs each)
    pub func_index: u16,
    /// [0x20] name msg id (`Name`)
    pub name: u32,
    /// [0x24] `FightAttackTunagiMsgId`, [0x28] `FightAttackMsgId`
    pub tunagi_msg: u32,
    pub attack_msg: u32,
    /// [0x2C] description msg id (`Doc`)
    pub doc: u32,
    /// [0x30] `WazawzxdataId`
    pub wzx: u32,
    /// [0x34..0x37] `TypeId` (indices 0..2)
    pub type_ids: [u8; 3],
    pub raw: [u8; MOVE_SIZE],
}

impl Move {
    pub fn parse(b: &[u8]) -> Result<Self> {
        let raw: [u8; MOVE_SIZE] = b[..MOVE_SIZE].try_into().unwrap();
        Ok(Move {
            pri: raw[0] as i8,
            pp: raw[1],
            zokusei: raw[2],
            range: raw[3],
            avg: raw[4],
            add_kouka_avg: raw[5],
            flags: raw[6..0x14].try_into().unwrap(),
            iryoku: u16_at(b, 0x16, "move")? as i16,
            kouka: u16_at(b, 0x18, "move")?,
            seq: u16_at(b, 0x1A, "move")?,
            func_index: u16_at(b, 0x1C, "move")?,
            name: u32_at(b, 0x20, "move")?,
            tunagi_msg: u32_at(b, 0x24, "move")?,
            attack_msg: u32_at(b, 0x28, "move")?,
            doc: u32_at(b, 0x2C, "move")?,
            wzx: u32_at(b, 0x30, "move")?,
            type_ids: raw[0x34..0x37].try_into().unwrap(),
            raw,
        })
    }
    pub fn dageki(&self) -> bool { self.flags[0] != 0 }
    pub fn mamoru(&self) -> bool { self.flags[1] != 0 }
    pub fn majikku(&self) -> bool { self.flags[2] != 0 }
    pub fn yokodori(&self) -> bool { self.flags[3] != 0 }
    pub fn oumugaesi(&self) -> bool { self.flags[4] != 0 }
    pub fn oujanosirusi(&self) -> bool { self.flags[5] != 0 }
    pub fn yubiwohuru(&self) -> bool { self.flags[6] != 0 }
    pub fn monomane(&self) -> bool { self.flags[7] != 0 }
    pub fn nekonote(&self) -> bool { self.flags[8] != 0 }
    pub fn negoto(&self) -> bool { self.flags[9] != 0 }
    pub fn bouon(&self) -> bool { self.flags[10] != 0 }
    pub fn pressure(&self) -> bool { self.flags[11] != 0 }
    pub fn hiden(&self) -> bool { self.flags[12] != 0 }
    pub fn risk(&self) -> bool { self.flags[13] != 0 }
}

/// Stats in the game's order: HP, Attack, Defense, Sp. Atk, Sp. Def, Speed
/// (`MaxHp`, `PhyAtk`, `PhyDef`, `SpeAtk`, `SpeDef`, `Nimbleness`).
pub type Stats = [u16; 6];

#[derive(Clone, Debug)]
pub struct Evolution {
    /// `SinkaKind` (method)
    pub kind: u8,
    /// `SinkaBuff` (level / item / condition parameter)
    pub buff: u16,
    /// `SinkaPokemonDataId`
    pub into: u16,
}

#[derive(Clone, Debug)]
pub struct LevelMove {
    /// `GetWazaLevel`
    pub level: u8,
    /// `GetWazaDataId`
    pub waza: u16,
}

/// One species (`pokemon`). Offsets in brackets.
#[derive(Clone, Debug)]
pub struct Species {
    /// [0x00] experience growth curve (`GrowDataId`)
    pub grow: u8,
    /// [0x01] catch rate (`Get`)
    pub get: u8,
    /// [0x02] gender ratio (`SexRatio`)
    pub sex_ratio: u8,
    /// [0x06] base experience yield (`GiveExp`)
    pub give_exp: u16,
    /// [0x08] base friendship (`InitFriend`)
    pub init_friend: u16,
    /// [0x0A] `Height`, [0x0C] `Weight`, [0x0E] cry (`Voice`)
    pub height: u16,
    pub weight: u16,
    pub voice: u16,
    /// [0x12] national dex number (`NumPokemon`), [0x14] Hoenn/regional dex number (`NumZukan`)
    pub num_pokemon: u16,
    pub num_zukan: u16,
    /// [0x18] name msg, [0x1C] category msg (`TypeName`), [0x20] dex text (`Doc`)
    pub name: u32,
    pub type_name: u32,
    pub doc: u32,
    /// [0x24] seen flag id (`MitaFlag`), [0x28] caught flag id (`TukamaetaFlag`)
    pub mita_flag: u32,
    pub tukamaeta_flag: u32,
    /// [0x2C] `PkxDataId` (battle model)
    pub pkx: u32,
    /// [0x30] types (`ZokuseiDataId` 0..1), [0x32] abilities (`TokuseiDataId` 0..1)
    pub zokusei: [u8; 2],
    pub tokusei: [u8; 2],
    /// [0x34] TM/HM compatibility (`WazaMcn`, 58 entries)
    pub waza_mcn: [u8; 0x3A],
    /// [0x70] wild held items (`ItemDataId` 0..1)
    pub items: [u16; 2],
    /// [0x74] egg moves (`Kowaza`, 8)
    pub kowaza: [u16; 8],
    /// [0x84] base stats (`Basis*`), [0x90] EV yield (`Give*Effort`)
    pub basis: Stats,
    pub effort: Stats,
    /// [0x9C] 5 evolutions of 6 bytes
    pub sinka: Vec<Evolution>,
    /// [0xBA] 20 level-up moves of 4 bytes (level, ?, move)
    pub get_waza: Vec<LevelMove>,
    pub raw: Vec<u8>,
}

impl Species {
    pub fn parse(b: &[u8]) -> Result<Self> {
        let h = |o| u16_at(b, o, "species");
        let w = |o| u32_at(b, o, "species");
        let stats = |o: usize| -> Result<Stats> {
            let mut s = [0; 6];
            for (i, v) in s.iter_mut().enumerate() { *v = h(o + 2 * i)?; }
            Ok(s)
        };
        let mut kowaza = [0; 8];
        for (i, v) in kowaza.iter_mut().enumerate() { *v = h(0x74 + 2 * i)?; }
        Ok(Species {
            grow: u8_at(b, 0, "species")?,
            get: u8_at(b, 1, "species")?,
            sex_ratio: u8_at(b, 2, "species")?,
            give_exp: h(6)?,
            init_friend: h(8)?,
            height: h(0xA)?,
            weight: h(0xC)?,
            voice: h(0xE)?,
            num_pokemon: h(0x12)?,
            num_zukan: h(0x14)?,
            name: w(0x18)?,
            type_name: w(0x1C)?,
            doc: w(0x20)?,
            mita_flag: w(0x24)?,
            tukamaeta_flag: w(0x28)?,
            pkx: w(0x2C)?,
            zokusei: [b[0x30], b[0x31]],
            tokusei: [b[0x32], b[0x33]],
            waza_mcn: b[0x34..0x6E].try_into().unwrap(),
            items: [h(0x70)?, h(0x72)?],
            kowaza,
            basis: stats(0x84)?,
            effort: stats(0x90)?,
            sinka: (0..5).map(|i| { let o = 0x9C + 6 * i; Ok(Evolution { kind: b[o], buff: h(o + 2)?, into: h(o + 4)? }) })
                .collect::<Result<_>>()?,
            get_waza: (0..20).map(|i| { let o = 0xBA + 4 * i; Ok(LevelMove { level: b[o], waza: h(o + 2)? }) })
                .collect::<Result<_>>()?,
            raw: b[..SPECIES_SIZE].to_vec(),
        })
    }
}

/// One item row (`item`). Offsets in brackets.
#[derive(Clone, Debug)]
pub struct Item {
    /// [0x00] pocket / kind (`Kind`; 4 = TM/HM)
    pub kind: u8,
    /// [0x04] `ItemEffectParam`
    pub effect_param: u8,
    /// [0x06] `Price`, [0x08] `Coupon`
    pub price: u16,
    pub coupon: u16,
    /// [0x0A] held-item effect (`ItemSoubiDataId`), [0x0C] battle-use effect (`FightUseKoukaDataId`)
    pub soubi: u16,
    pub fight_use_kouka: u16,
    /// [0x10] name msg, [0x14] description msg
    pub name: u32,
    pub doc: u32,
    /// [0x18] `Buff` (TM number in the low byte for kind 4)
    pub buff: u32,
    /// [0x1C] field-use function, [0x20] battle-use function (main.dol addresses)
    pub field_use: u32,
    pub battle_use: u32,
    /// [0x24..0x26] friendship change per band (`UseFriend` 0..2, i8)
    pub use_friend: [i8; 3],
    pub raw: [u8; ITEM_SIZE],
}

impl Item {
    pub fn parse(b: &[u8]) -> Result<Self> {
        let raw: [u8; ITEM_SIZE] = b[..ITEM_SIZE].try_into().unwrap();
        Ok(Item {
            kind: raw[0],
            effect_param: raw[4],
            price: u16_at(b, 6, "item")?,
            coupon: u16_at(b, 8, "item")?,
            soubi: u16_at(b, 0xA, "item")?,
            fight_use_kouka: u16_at(b, 0xC, "item")?,
            name: u32_at(b, 0x10, "item")?,
            doc: u32_at(b, 0x14, "item")?,
            buff: u32_at(b, 0x18, "item")?,
            field_use: u32_at(b, 0x1C, "item")?,
            battle_use: u32_at(b, 0x20, "item")?,
            use_friend: [raw[0x24] as i8, raw[0x25] as i8, raw[0x26] as i8],
            raw,
        })
    }
}

/// One type row: effectiveness against each defending type, as message/"joutai" ids:
/// 0x3F normal, 0x41 super effective, 0x42 not very effective, 0x43 no effect.
#[derive(Clone, Debug)]
pub struct TypeRow {
    pub vs: [u16; TYPE_COUNT],
    /// [0x24] 1 / 2 / 0 / 0xFF (physical / special split by type, as in Gen 3; unverified)
    pub b24: u8,
    pub b25: u8,
    pub h26: u16,
    /// [0x28] name msg id
    pub name: u32,
}

pub struct BattleData {
    pub moves: Vec<Move>,
    pub species: Vec<Species>,
    /// Item rows, and the id → row map (`None` = no row).
    pub items: Vec<Item>,
    pub item_rows: Vec<Option<u16>>,
    pub types: Vec<TypeRow>,
}

impl BattleData {
    pub fn load(common: &Common, dol: &Dol) -> Result<Self> {
        let d = common.rel.data;
        let (mb, mn) = common.table(SYM_MOVES)?;
        let moves = (0..mn).map(|i| Move::parse(&d[mb + i * MOVE_SIZE..])).collect::<Result<_>>()?;
        let (sb, sn) = common.table(SYM_SPECIES)?;
        let species = (0..sn).map(|i| Species::parse(&d[sb + i * SPECIES_SIZE..])).collect::<Result<_>>()?;
        let rows = dol.read(ITEM_ROWS, ITEM_ROW_COUNT * ITEM_SIZE)?;
        let items = (0..ITEM_ROW_COUNT).map(|i| Item::parse(&rows[i * ITEM_SIZE..])).collect::<Result<_>>()?;
        let idx = dol.read(ITEM_INDEX, ITEM_IDS * 2)?;
        let item_rows = (0..ITEM_IDS).map(|i| {
            let r = u16::from_be_bytes([idx[2 * i], idx[2 * i + 1]]);
            ((r as usize) < ITEM_ROW_COUNT).then_some(r)
        }).collect();
        let t = dol.read(TYPE_CHART, TYPE_COUNT * 0x2C)?;
        let types = (0..TYPE_COUNT).map(|i| {
            let b = &t[i * 0x2C..];
            let mut vs = [0; TYPE_COUNT];
            for (j, v) in vs.iter_mut().enumerate() { *v = u16::from_be_bytes([b[2 * j], b[2 * j + 1]]); }
            Ok(TypeRow { vs, b24: b[0x24], b25: b[0x25], h26: u16_at(b, 0x26, "type")?, name: u32_at(b, 0x28, "type")? })
        }).collect::<Result<_>>()?;
        Ok(BattleData { moves, species, items, item_rows, types })
    }

    /// `itemDataBiosGetPtr`: the row for an item id.
    pub fn item(&self, id: u16) -> Option<&Item> {
        self.item_rows.get(id as usize).copied().flatten().map(|r| &self.items[r as usize])
    }
}

/// Field type of a generic table record.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ty { U8, I8, U16, I16, U32, F32 }

/// One field: getter name (without the `...DataBiosGet` prefix), offset, type, and for arrays
/// (getters taking an index) the element count and stride.
#[derive(Clone, Copy, Debug)]
pub struct Field { pub name: &'static str, pub off: usize, pub ty: Ty, pub n: usize, pub stride: usize }

const fn f(name: &'static str, off: usize, ty: Ty) -> Field { Field { name, off, ty, n: 1, stride: 0 } }
const fn a(name: &'static str, off: usize, ty: Ty, n: usize, stride: usize) -> Field { Field { name, off, ty, n, stride } }

/// Where a table lives: a common_rel symbol (registered at load as a main.dol (count, base) pair), or a
/// fixed main.dol address with its count in a main.dol u32.
#[derive(Clone, Copy, Debug)]
pub enum Src { Rel(usize), Dol { base: u32, count_at: u32 } }

/// A battle table as one `*DataBiosGetPtr` sees it.
pub struct TableDef { pub name: &'static str, pub getptr: u32, pub src: Src, pub size: usize, pub fields: &'static [Field] }

use Ty::*;

/// Every battle table whose `*DataBiosGetPtr`/getters were read (2026-10-12). Rel symbols were matched to
/// the game's (count, base) globals in a running game (Dolphin), e.g. trainers 0x80478F20/24 = symbol 44.
pub const TABLES: &[TableDef] = &[
    TableDef { name: "fightTrainerPokemon", getptr: 0x801FCA2C, src: Src::Rel(48), size: 0x50, fields: &[
        f("TokuseiFlag", 0, I8), f("SexDataId", 1, I8), f("SeikakuDataId", 2, I8), f("DarkPokemonFlag", 3, U8),
        f("Level", 4, U8), f("KeyPlayerFlag", 5, U8), f("PartDataId", 6, U8), f("Friend", 8, I16),
        f("PokemonDataId", 0xA, U16), f("ItemBallId", 0xC, U16), f("ItemDataId", 0x10, U32), f("Nickname", 0x14, U32),
        a("StatusRnd", 0x1C, I8, 6, 1), a("StatusEffort", 0x22, I16, 6, 2),
        a("WazaDataId", 0x34, U32, 4, 8), a("PpCnt", 0x30, U8, 4, 8)] },
    TableDef { name: "fightTrainerPokemonPart", getptr: 0x801FCAD0, src: Src::Rel(58), size: 0x14, fields: &[
        f("Name", 0, U32), a("WazaTypeRevise", 8, U8, 12, 1)] },
    TableDef { name: "fightTrainer", getptr: 0x801FCCC4, src: Src::Rel(44), size: 0x34, fields: &[
        // Getters are unnamed FUN_s; named after their fightTrainerGetStatus id. Guesses (unverified): 05 first
        // trainer-Pokemon index, 02 trainer AI index, 03 name msg, 08 battle text msgs, 06 items.
        f("Status01", 0, U8), f("KindDataId", 2, U16), f("Status05", 4, U16), f("Status02", 6, U16), f("Status03", 8, U32),
        f("Status07", 0xC, U32), f("Status09", 0x10, U32), a("Status06", 0x14, U16, 8, 2), a("Status08", 0x24, U32, 4, 4)] },
    TableDef { name: "fightTrainerKind", getptr: 0x801FBD58, src: Src::Rel(24), size: 0xC, fields: &[
        f("SyoukinBairitu", 0, U16), f("PrefixName", 4, U32), f("BgmSndId", 8, U32)] },
    TableDef { name: "fightTrainerAi", getptr: 0x801FC658, src: Src::Rel(46), size: 0x28, fields: &[
        f("PokemonSelectRandomFlag", 0, U8), f("PokemonSelectWeakPointFlag", 1, U8), f("PokemonDataOrderOutFlag", 2, U8),
        f("PokemonDataOrderAceBossFlag", 3, U8), f("KeyPlayerFlag", 4, U8), f("PokemonJoutaiFlag", 5, U8),
        f("ComboValue", 6, U8), f("IrekaeValue", 7, U8), f("ItemValue", 8, U8), f("ParamExpectFlag", 9, U8),
        f("ParamStoreFlag", 0xA, U8), f("ZokuseiCheckFlag", 0xB, U8), f("TokuseiCheckFlag", 0xC, U8),
        f("NokoriHpValue", 0xD, U8), f("WazaDamageFlag", 0xE, U8), f("AbicntMaxValue", 0xF, U8), f("AbicntMinValue", 0x10, U8),
        f("DefensePokemonRndSelectFlag", 0x11, U8), f("WazaRndSelectFlag", 0x12, U8), f("WazaInitValueFlag", 0x13, U8),
        f("PartFlag", 0x14, U8), f("WazaHitFlag", 0x15, U8), f("WazaAvgValue", 0x16, U8), f("WazaRiskFlag", 0x17, U8),
        f("WazaNokoriPpValue", 0x18, U8), f("LastValueRevise", 0x19, U8),
        a("ZokuseiReviseZokuseiDataId", 0x20, U8, 2, 2), a("ZokuseiReviseValue", 0x21, U8, 2, 2),
        a("WazaTypeReviseTypeDataId", 0x24, U8, 2, 2), a("WazaTypeReviseValue", 0x25, U8, 2, 2)] },
    TableDef { name: "fightTrainerAiValueAddsub", getptr: 0x801FBFBC, src: Src::Rel(56), size: 0x14, fields: &[
        f("Value", 0, U32), f("KoudouName", 4, U32), f("PrefixName", 8, U32), f("Name", 0xC, U32)] },
    TableDef { name: "fightSide", getptr: 0x801F7870, src: Src::Rel(26), size: 0x14, fields: &[
        f("Yrot", 0, U16), a("FightTrainerStatusMenuDataId", 4, U32, 2, 4), a("FightoutPokemonStatusMenuDataId", 0xC, U32, 2, 4)] },
    TableDef { name: "fightFloor", getptr: 0x801F6738, src: Src::Rel(28), size: 0x18, fields: &[
        f("TikeiDataId", 0, U8), f("FloorDataId", 2, U16), f("Name", 4, U32), f("SyoukaiWzxDataId", 8, U32),
        f("BgmSndId", 0xC, U32), f("EnvSndId", 0x10, U32), a("FightSideDataId", 0x14, U16, 2, 2)] },
    TableDef { name: "fightEncount", getptr: 0x8020E0F8, src: Src::Rel(50), size: 0x38, fields: &[
        f("FightKind", 0, U8), f("Trainer", 1, U8), f("ZenmetuFlag", 2, U8), f("FightFloorDataId", 4, U16),
        f("FightName", 8, U32), f("BgmSndId", 0xC, U32), f("WipeId", 0x10, U32), f("SyoukaiWzxDataId", 0x14, U32),
        a("FightTrainerDataId", 0x18, U16, 4, 8), a("GSInputDevice", 0x1C, U32, 4, 8)] },
    TableDef { name: "fightType", getptr: 0x8020E204, src: Src::Rel(42), size: 8, fields: &[
        f("TrainerNum", 0, U8), f("EntryPokemonNum", 1, U8), f("FightoutPokemonNum", 2, U8), f("Name", 4, U32)] },
    TableDef { name: "fightKind", getptr: 0x8020E488, src: Src::Rel(32), size: 0x20, fields: &[
        f("BackSaveDataFlag", 0, U8), f("DoBadgeCheckFlag", 1, U8), f("DoZukanMitaFlag", 2, U8), f("DoZukanTukamaetaFlag", 3, U8),
        f("UseItemFlag", 4, U8), f("CallFlag", 5, U8), f("NigeruFlag", 6, U8), f("DrawFlag", 7, U8), f("GetExpFlag", 8, U8),
        f("GetOkaneFlag", 9, U8), f("OkanePoolFlag", 0xA, U8), f("GetNekoniKobanFlag", 0xB, U8), f("GetFriendFlag", 0xC, U8),
        f("GetInfectPokerusFlag", 0xD, U8), f("DoCriticalAttackFlag", 0xE, U8), f("DoHizukiAiFlag", 0xF, U8),
        f("DoHizukiMiyaburiFlag", 0x10, U8), f("DoItemSoubiTokukoutokubouupFlag", 0x11, U8), f("KeikentihueruFlag", 0x12, U8),
        f("BossFlag", 0x13, U8), f("DorobouFlag", 0x14, U8), f("MonohiroiFlag", 0x15, U8), f("DarkpokemonHypermodeFlag", 0x16, U8),
        f("PokemonStatusMenuSubbarFlag", 0x17, U8), f("HostEnemyMsgFlag", 0x18, U8), f("Name", 0x1C, U32)] },
    TableDef { name: "pokemonSeikaku", getptr: 0x8011CE18, src: Src::Rel(64), size: 0x28, fields: &[
        f("ReliveFightout", 0, I8), f("ReliveWalk", 1, I8), f("ReliveCall", 2, I8), f("ReliveSodateya", 3, I8),
        f("ReliveNadenade", 4, I8), f("PhyAtkRateDataId", 5, U8), f("PhyDefRateDataId", 6, U8), f("SpeAtkRateDataId", 7, U8),
        f("SpeDefRateDataId", 8, U8), f("NimblenessRateDataId", 9, U8), f("Name", 0x14, U32),
        // % chance by Shadow level (Heart Gauge bars, index 0..6): entering Hyper Mode when Shadow Rush is ordered
        // (0x8011CC24, read by `_fightCheckDarkPokemonAppear`) and a Call snapping it out (0x8011CBF4, `fightActionCallMain`)
        a("HyperRate", 0x18, U8, 7, 1), a("CallRate", 0x1F, U8, 7, 1)] },
    TableDef { name: "pokemonSeikakuRate", getptr: 0x8011CBC8, src: Src::Rel(70), size: 2, fields: &[f("Kake", 0, U8), f("Waru", 1, U8)] },
    TableDef { name: "pokemonDpFilter", getptr: 0x8011CAB8, src: Src::Rel(66), size: 1, fields: &[f("Value", 0, I8)] },
    TableDef { name: "pokemonGrow", getptr: 0x8011CE74, src: Src::Dol { base: 0x8035E940, count_at: 0x80478B60 }, size: 0x194,
        fields: &[a("Exp", 0, U32, 101, 4)] },
    TableDef { name: "pokemonTokusei", getptr: 0x8011CB6C, src: Src::Dol { base: 0x8035F5E0, count_at: 0x80478B68 }, size: 0xC,
        fields: &[f("Name", 4, U32), f("Doc", 8, U32)] },
    TableDef { name: "pokemonFriendFilter", getptr: 0x8011CB10, src: Src::Dol { base: 0x8035F988, count_at: 0x80478B70 }, size: 3,
        fields: &[a("Value", 0, I8, 3, 1)] },
    TableDef { name: "itemSoubi", getptr: 0x80143B48, src: Src::Dol { base: 0x80367EF0, count_at: 0x80478BC0 }, size: 2,
        fields: &[f("FightKoukaDataId", 0, U16)] },
    TableDef { name: "itemBall", getptr: 0x80143DA0, src: Src::Dol { base: 0x80367AF0, count_at: 0x80478BB8 }, size: 0x30, fields: &[
        f("FightKoukaDataId", 0, U16), f("InWzxDataId", 4, U32), f("OpenWzxDataId", 8, U32), f("OutWzxDataId", 0xC, U32),
        f("DowninWzxDataId", 0x10, U32), f("ThrowWzxDataId", 0x14, U32), f("SnatchAttackWzxDataId", 0x18, U32),
        f("SnatchBalllandWzxDataId", 0x1C, U32), f("SnatchMissWzxDataId", 0x20, U32), f("SnatchPokeoutWzxDataId", 0x24, U32),
        f("SnatchShakeWzxDataId", 0x28, U32), f("SnatchSnatchWzxDataId", 0x2C, U32)] },
    TableDef { name: "fightTarget", getptr: 0x801F0234, src: Src::Dol { base: 0x80375AC8, count_at: 0x80478D40 }, size: 8,
        fields: &[f("StatusKid", 0, U8), f("Buff", 4, U32)] },
    TableDef { name: "fightWazaHitKakuritu", getptr: 0x8020A040, src: Src::Dol { base: 0x80375DD0, count_at: 0x80478D70 }, size: 2,
        fields: &[f("Kake", 0, U8), f("Waru", 1, U8)] },
    TableDef { name: "fightWazaCritical", getptr: 0x8020A080, src: Src::Dol { base: 0x80478D58, count_at: 0x80478D60 }, size: 1,
        fields: &[f("Bunbo", 0, U8)] },
    TableDef { name: "fightActionKind", getptr: 0x8020D7E8, src: Src::Dol { base: 0x80375BB8, count_at: 0x80478D48 }, size: 0xC,
        fields: &[f("Pri", 0, I8), f("FlowFuncPtr", 4, U32), f("DispFuncPtr", 8, U32)] },
    TableDef { name: "fightKouka", getptr: 0x8020D9E8, src: Src::Dol { base: 0x80375CB8, count_at: 0x80478D50 }, size: 6,
        fields: &[f("FightJoukenDataId", 0, U16), f("FightTargetDataId", 2, U16), f("KoukaDataId", 4, U16)] },
    TableDef { name: "fightEncountWipe", getptr: 0x8020DEB0, src: Src::Dol { base: 0x80375980, count_at: 0x80478D20 }, size: 0xC,
        fields: &[f("WipeSnapshotUse", 0, U8), f("WipeEffectSndID", 2, U16), f("WipeEffectTime", 4, F32), f("WipeFunction", 8, U32)] },
];

/// A loaded generic table: its definition and record bytes.
pub struct Table { pub def: &'static TableDef, pub data: Vec<u8> }

impl Table {
    pub fn load(def: &'static TableDef, common: &Common, dol: &Dol) -> Result<Self> {
        let data = match def.src {
            Src::Rel(sym) => {
                let (o, n) = common.table(sym)?;
                crate::be::bytes_at(common.rel.data, o, n * def.size, "battle table")?.to_vec()
            }
            Src::Dol { base, count_at } => {
                let n = u32_at(dol.read(count_at, 4)?, 0, "count")? as usize;
                dol.read(base, n * def.size)?.to_vec()
            }
        };
        Ok(Table { def, data })
    }
    pub fn len(&self) -> usize { self.data.len() / self.def.size }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn field(&self, name: &str) -> Option<&'static Field> { self.def.fields.iter().find(|f| f.name == name) }
    /// The value a getter returns for record `i` (element `k` for array getters), sign-extended for signed types.
    /// F32 values come back as their bit pattern.
    pub fn get(&self, i: usize, name: &str, k: usize) -> Option<i64> {
        let f = self.field(name)?;
        if i >= self.len() || k >= f.n { return None; }
        let o = i * self.def.size + f.off + k * f.stride;
        let d = &self.data;
        Some(match f.ty {
            U8 => d[o] as i64,
            I8 => d[o] as i8 as i64,
            U16 => u16::from_be_bytes([d[o], d[o + 1]]) as i64,
            I16 => i16::from_be_bytes([d[o], d[o + 1]]) as i64,
            U32 | F32 => u32::from_be_bytes([d[o], d[o + 1], d[o + 2], d[o + 3]]) as i64,
        })
    }
}
