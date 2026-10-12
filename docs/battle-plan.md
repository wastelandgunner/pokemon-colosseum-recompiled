# Battle mechanics: ordered task list (1:1 with GC6E01)

Status 2026-10-12: colo-rs has **no battle code**. The viewer only knows the battle natives by name.

## What exists
- **Code range** 0x801C2AE8–0x80265EC4: 2,666 functions, 669 KB. In the decomp's symbols, 767 functions (272 KB) are still `fn_`.
  - `battle_camera` / `battle_grid` 0x801C2AE8: battle-scene camera and model placement.
  - `wazaSequence*` / `wazaViewer` 0x801D2B4C–0x801DCDA8: move animations (camera, models, particles).
  - `battle_main` 0x801ED640, `fight_target` 0x801F0058, `fight_action` 0x801F0898, then `fight_floor`,
    `fight_side`, `fight_trainer(_db)` and `fight_pokemon`.
  - `fight` 0x80201764, `fight_out_pokemon` 0x80202810 (182 accessors for active battlers), `fight_kouka`, `fight_encount`, `fight_waza`.
  - `fight_seq` 0x80211170 is turn flow. 0x80211A00–0x802405C0 (191 KB) is the move-script interpreter and its `WS_*` commands.
  - `fight_trainer_ai_*` 0x802405C0–0x8025CD64: AI move scoring (`WazaValue`/`WazaHit`/`WazaDamage`), switching (`irekae`) and `ai2`.
  - Battle menus and timer: `fight_menu` 0x80261B68, `fight_timer` 0x802658C8, `menuFight*` 0x8000DAA8.
- **Move-script VM** (Gen 3 style):
  - The runner is `fn_80211B94(ctx, script, preserve)`. The PC is `lbl_8047B610`, the state `lbl_8047B614` (1 = yield, 2 = end) and the ctx `lbl_8047B62C`.
  - The command table is `lbl_8027A00C`: 255 handlers (`WS_HITCHECK`, `fightCheckAccuracy`, `WS_CRITICAL_CHECK`, `WS_TYPE_CHECK`, `WS_DAMAGE_LOSS`, …).
  - The script bytecode is in main.dol `.data` (e.g. `lbl_80378798`, the end-of-turn script), with absolute pointers.
- **Script entry points:**
  - std 102 `battleStart` 0x8020DAD0.
  - std 152 `battleStartBattleWithOptions` 0x801CA5C4.
  - std 103 `battleGetResult` 0x801EF634.
  - std 215 (battle after a yes/no).
  - std 9/10 `winMsgOpen/CloseFight`.
- **Data:** in common_rel: Pokémon stats, moves, natures, trainers, trainer Pokémon, trainer AI, battles, battle fields/types/styles and Shadow data. In main.dol: items and the type chart. colo-formats parses none of these yet.
- **Refs:**
  - Fork `docs/fable5_audit_pass3_fight_engine.md` has struct ledgers (`FightActionWork`, `FightSideEntry`) and the 94-field Pokémon getter `fn_801F54A4`.
  - The XD vocabulary holds 376 unapplied battle/floor/waza names.
  - Dolphin is not installed.

## Tasks (in order)

### A. Groundwork
1. **Ghidra pass over the battle range.** Import the applicable XD names, name the 767 `fn_` functions, and define the structs `FightFloor`, `FightTrainer`, `FightPokemon`, `FightOutPokemon`, `FightActionWork`, `FightSideEntry` and the party Pokémon (from `fn_801F54A4`). Add a NOTES.md §16 "Battle". Continue in that section as each later step lands.
2. **Port the RNG exactly** (`GSrand`: seed, LCG, and which calls consume it). Every later step depends on matching RNG call order.
3. **Decide the ground truth.** Either install Dolphin and record battle traces (RNG draws, damage, script PCs), or use the matched `_exact_` decomp units as per-function oracles. *Decision for Zach.*

### B. Data (colo-formats, zero deps)
4. Pokémon base stats, the experience curves and the moves table: power, type, accuracy, PP, effect, target, priority and flags.
5. Type chart, natures and items (held-battle effects and their parameters).
6. Trainers, trainer Pokémon (including the Shadow flag), the Shadow Pokémon table (heart gauge, Shadow moves), the battles table, battle fields/types/styles and trainer AI data.
7. The party Pokémon struct (game layout) and how it is stored in the save. This replaces the viewer's story-only save.
8. A WS disassembler (`colo-tool ws-dis`): command lengths from the 255 handlers, the move-effect → script table and every script blob. Check that all scripts decode end to end.

### C. Core engine (new zero-dep crate `colo-battle`, headless)
9. State model mirroring the game structs: floor, sides, trainers and battlers. Stats use the Gen 3 formulas with nature, IVs and EVs. Also stages, status and volatile conditions.
10. `fight_encount` / `fightSeqInit`: battle setup from a battles-table entry (single/double, field, rules).
11. Turn flow (`fight_seq`): the action FIFO (`fightSeqFightActionCreateAndFlowFifo`), turn order (priority, speed, Quick Claw), end-of-turn checks and fainting/replacement.
12. The WS VM runner (`fn_80211B94`, including the preserve/end-of-turn nesting).
13. Hit pipeline commands: hit check, accuracy, critical hit, type check, damage formula + variance (`fightApplyDamageVariance`) and `DAMAGE_LOSS`.
14. Secondary effects (`TSUIKA_DIRECT/INDIRECT_ACT`) and the condition and side-condition checks.
15. The remaining `WS_*` move effects, one family at a time (multi-turn, Protect/Endure, Encore/Taunt/Torment, Metronome/Mimic, Perish Song, weather, …).
16. Abilities (`tokusei` lookups) and held items (`fightSeqGetItemType`).
17. Switching, Baton Pass, run/forfeit, battle results (`battleGetResult`), and EXP/level-up/move learning (`WS_WAZAOBOE_CHECK`) plus the evolution hook.

### D. Colosseum-specific
18. Shadow Pokémon: Shadow moves, Hyper Mode, the Call command, heart gauge changes in battle, and Rui's Shadow sense (`fightCheckRuiShadowSense`).
19. Snagging: Snag Machine rules, the capture formula and ball effects.
20. `fight_timer` and battle rules and styles (Colosseum modes, bans, level caps).

### E. Trainer AI
21. `fight_trainer_ai_*`: move scoring (`WazaValue`/`Hit`/`Damage`), argmax with an RNG tiebreak, switching (`irekae`) and `ai2`, all driven by the trainer AI data.

### F. Verification
22. `colo-tool battle-sim`: a headless scripted battle that prints an RNG-draw and event log. Compare it with the oracle from step 3, using a regression set per move effect.

### G. Presentation (viewer)
23. The natives std 102/103/152/215 and 9/10: the field → battle transition (`fadeEffect*`) and the return to the field with the result.
24. Battle text (`fight_common` .msg) through the existing message window.
25. Battle stages (`fight_floor`/`fight_gsfloor` maps), `battle_grid` placement and `battle_camera`.
26. PKX battle models and their animation slots (open item in §15), plus send-out and recall.
27. Command menus and HP/status boxes (`fight_menu`, `menuFight`, `menuFightStatus`).
28. Move animations (`wazaSequence*`: camera, models and particles over the existing particle system).
29. Battle music and sound effects (the sound layer is already ported).

Steps A–F are rules only and can be tested without the viewer. Step G starts once a whole battle can be simulated headless.
