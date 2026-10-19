# Pokémon Colosseum (GC6E01) sound layer — RE report

Notation: addresses are main.dol. "rel+X" is a file offset into the decompressed `common.fsys/common_rel.rel`. Anything not proven by code or data is marked UNVERIFIED.

## 0. Data model

### 0.1 Tables
The common_rel prolog registers these tables into main.dol `.sbss` pointers, each as a (table, count) pair:

| main.dol pointer (data / count) | meaning | rel+ data | count | entry size |
|---|---|---|---|---|
| `0x80478FAC` / `0x80478FA8` | sound table (indexed by snd_id) | 0x141654 | 1236 | 0x0C |
| `0x80478E34` / `0x80478E30` | song file table | 0x1133d8 | 79 | 0x08 |
| `0x80478FB4` / `0x80478FB0` | wave table (MusyX groups) | 0x145164 | 8 | 0x18 |
| `0x80478FA4` / `0x80478FA0` | ReverbHI presets | 0x145044 | 12 | 0x18 |
| `0x80478E94` / `0x80478E90` | menu SE sets | 0x14be4e | 8 | 0x0A |
| `0x80478F94` / `0x80478F90` | species table (cry id at +0xE) | 0x123250 | 413 | 0x11C |

### 0.2 Sound table entry (12 bytes, at `snd_id * 0xC`)

| offset | type | meaning |
|---|---|---|
| +0 | u8 | flags: bit7 = sound effect (MusyX FX), 0 = song (MusyX sequence); bit6 is set on all group-6 entries but nothing reads it (UNVERIFIED); bit5 = runtime "work attached"; bit4 = runtime "music paused" |
| +1 | u8 | MusyX group id: 0 music, 1 music_atmos, 4 nakigoe, 5 se_battle, 6 se, 8 se_motion, 9 nakigoe2. Passed to sndSeqPlayEx for songs; informational for effects |
| +2 | u8 | ReverbHI preset index (songs only; 0 = keep current) |
| +3 | u8 | song file index (songs only; every effect has 10 = the null entry) |
| +4 | u16 | MusyX id: SongID for songs, FX id for effects |
| +8 | ptr | runtime pointer to the attached work |

- **Census:** 79 songs (78 in group 0, plus id 34 `wind_atmos_loop` in group 1); 650 se_battle, 200 nakigoe, 186 nakigoe2, 119 se, 2 se_motion.
- **Id 0 is invalid.** Every entry point rejects `id == 0 || id >= 1236`.
- **Id 1 is `null_bgm`** (SongID 3): the "no music" track. Pause and resume refuse id 1.
- **There is no special range for ids ≥ 1000.** Songs and effects are interleaved; see `sound_table.tsv`.

### 0.3 Song file table (8 bytes)
- +0: u32 FSYS id inside bgm_archive (group 0x99).
- +4: DVD path `sound/<name>.song`, used only as a fallback when +0 is 0.
- Entry 10 is all zeros.
- All 78 `*_song.samp` files are covered. The largest is battle2 at 36680 bytes.

### 0.4 Wave table (0x18 bytes)

| offset | meaning |
|---|---|
| +0 | u8, purpose unknown (UNVERIFIED) |
| +1 | runtime "pushed" flag |
| +2 | u16 MusyX group id |
| +4 | FSYS group (0xA = common.fsys) |
| +8 / +C / +10 | pool / proj / sdir FSYS ids |
| +14 | path to `/sound/snd_*.samp` |

| index | group id | bank | loaded by |
|---|---|---|---|
| 0 | 0 | music (duplicate; never loaded) | — |
| 1 | 0 | snd_music | boot, 0x8003686C |
| 2 | 6 | snd_se | 0x800366A8 |
| 3 | 9 | nakigoe2 | 0x800363BC |
| 4 | 1 | music_atmos | 0x800364C8 |
| 5 | 5 | se_battle | 0x800364C8 |
| 6 | 8 | se_motion | 0x800364C8 |
| 7 | 4 | nakigoe | 0x800363BC |

0x800363BC and 0x800364C8 are called from a boot-step function table in common_rel.

### 0.5 Runtime state
- **Init:** `GSsndInit(2, 1, 0x20, 1, 0x10)` gives 34 work slots (2 + 0x20, 0x14 bytes each), 16 emitter slots (0x78 bytes) and 1 listener slot (0xD0 bytes).
- **Work slot layout:**

  | offset | meaning |
  |---|---|
  | +0 | in use |
  | +1 | volume (default 0x7F) |
  | +2 | pan (0x40) |
  | +3 | 0x40, unused |
  | +4 | duck-stack depth |
  | +5..+7 | saved volumes |
  | +8 | MusyX handle (seqId or FX voice; -1 = none) |
  | +C | resource group (always 0) |
  | +10 | buffer resource id: 0x406 = music 64 KB, 0x407 = sub 8 KB, 0x408 = fanfare temp 64 KB, 0 for effects |

- **Current ids:** `0x8047B0AC` is the current background music (std228 returns it). `0x8047B0A8` is the current sub id (std153 stops it). `0x8047B0E4` is the current reverb preset.
- **Which song is "the music" is not tracked by id.** `FUN_80167768(0, res)` searches for the work whose +0x10 is 0x406 (music) or 0x407 (sub).
- **Status, `FUN_801666BC`:**
  - It returns 0 (inactive or just ended), 2 (playing) or 3 (paused). Callers treat 1..3 as busy.
  - A finished sound's work is released inside this check.
  - It tests `sndFXCheck` for effects and `sndSeqGetValid` (0x8014D598) for songs.
- **Kill, `FUN_80166B18`:** stops the sound with time 0, re-initialises the work and frees the slot.
  - `_sndStopSE`: `sndFXKeyOff` only, no fade.
  - `_sndStopBGM(entry, ms)`: `sndSeqVolume(0, ms, seq, STOP=1)`.
- **Set volume, `FUN_80166670(id, ms, vol)`:**
  1. Stores `vol & 0x7F` in the work.
  2. Effects: `sndFXCtrl(h, 7, vol)`, time ignored. Songs: `sndSeqVolume(vol, ms, seq, 0)`.
- **Wait, `FUN_80165D0C(ms)`:** yields the calling thread until ms/1000 seconds of real time have passed (frames elapsed / vsync rate).
- **Volume masking:** every volume is `& 0x7F`. Scripts pass 255, which becomes 127. A value of 128 would become 0; reproduce this exactly.

## 1. Natives → MusyX
All times are milliseconds. The common worker is `play(id, fadeInMs, vol, isBgm)` at 0x80165A44.

| std | address | real semantics (proposed name) | implementation |
|---|---|---|---|
| 104 | 0x80165A20 | soundSetBackgroundMusic(id, fadeInMs, vol) | `play(id, a2, a3, isBgm=1)` |
| 105 | 0x801659FC | soundPlaySub(id, fadeInMs, vol): an effect, or a jingle/ambient song in the 8 KB sub slot | `play(id, a2, a3, isBgm=0)` |
| 106 | 0x801657F8 | soundStop(id, fadeMs) | see §1.3 |
| 153 | 0x801657D0 | scriptSoundStop(fadeMs) | `soundStop(curSub, fadeMs)` |
| 138 | 0x8016597C | soundChangeBGM(id, fadeOutMs, fadeInMs, vol) | `fadeSlot(fadeOut, music, wait)`, then `play(id, fadeIn, vol, 1)` |
| 139 | 0x801658FC | soundChangeSub(id, fadeOutMs, fadeInMs, vol) | same, using the sub slot |
| 137 | 0x801656F8 | soundSetVolume(id, ms, vol) | `FUN_80166670` |
| 109 | 0x80165718 | soundSetMusicMasterVolume(vol, ms); returns 1 | `FUN_80166D48(vol, ms, 1, 0)` |
| 130 | 0x80165668 | soundPlayFanfareAndWait(id, fadeIn, vol) | see §1.5 |
| 107 | 0x80165788 | soundPlay3D(id, x, y, z) → bool | emitter, see §1.6 |
| 108 | 0x80165744 | creates a second listener at the origin; always fails (only one slot, taken at boot); scripts never use it | see §1.6 |
| 218 | 0x801CAE80 | soundPlayPokemonCry(species) | see §5 |
| 228 | 0x801653C4 | soundGetCurrentBackgroundMusic() | returns `0x8047B0AC` |

Two getters have no native: 0x801653BC returns the current sub id, and 0x801656D8 returns a sound's work volume (0 if inactive).

Script census:
- std104: 84 calls. Common music ids are 7, 3, 4, 977, 1094, 1117 and 1. Volumes are 127, 255, 64 or 80; fade-ins are 0, 100 or 200.
- std105: effect ids 41, 44, 959, 1098, 1178, 1219, 1220, plus 34 (the atmos song).
- std109: `(80, 1000)` 15 times to duck the music, and `(127, 0 or 1000)` to restore it.
- std138: `(28 / 1133 / 10 / 1118, 1000 or 500, 0, 255)`.
- std137: `(3, 1000, 0)`, which fades stand_song to 0 without stopping it.
- std130: `(969, 0, 127)`.

### 1.1 play (0x80165A44)
```
SE:   if !isBgm { if 0<status(id)<4 {setVol(id,50,0); wait(50); kill(id)}  curSub=id }
      // isBgm: curBgm/curSub untouched, nothing stopped
      start(id,fadeIn,vol)
SONG: res,size = isBgm?(0x406,0x10000):(0x407,0x2000); fileIdx>=79 -> error 0
      prev = owner(res)
      if prev!=id { if prev!=-1 && 0<status(prev)<4 {setVol(prev,50,0); wait(50); kill(prev)}
                    memset(buf,0xE0,size); load bgm_archive(0x99, fsysId) (or the DVD path); attachWork(id,0,res) }
      if isBgm curBgm=id else curSub=id
      start(id,fadeIn,vol)
```

`start` (FUN_80166A50):
1. `_sndSetVolumeWork(id, vol)`. A song with no attached work plays nothing. An effect gets a work attached automatically.
2. **Effects:** `h = sndFXStartEx(fxId, work.vol, pan 0x40, studio 0)`, then `sndFXCtrl(h, 7, vol)`.
   - The fade-in argument is ignored for effects.
   - The handle is overwritten, so a previous instance keeps sounding untracked. std105 avoids this by stopping first; std104 and the C helpers do not.
3. **Songs** (FUN_8016761C), only if the work's handle is -1:
   1. `applyReverb(e[2])`.
   2. `seq = sndSeqPlayEx(e.group, e.songId, buf, &para, 0)`, with `para = {flags = SND_PLAYPARA_VOLUME (4), trackMute = 0, speed = 0x100, vol.time = fadeIn, vol.target = vol}`.
   3. MusyX then calls `synthVolume` on the sequence's default volume group (seqId + 23). The fade starts from that fader's previous value (UNVERIFIED).

Consequences:
- **Requesting the music that is already playing does nothing.** No restart and no reload; the new volume is only stored, not applied.
- **A sub jingle (std105) plays on top of the music.** Only std130 pauses the music.

### 1.2 fadeSlot(ms, isMusic, wait) (0x80165C70)
- It looks up the owner of the music or sub slot. If that sound is busy, it fades it to 0 (it does not stop it), then waits `ms` if asked.
- **Quirk:** std138 or std139 with the id that is already playing leaves that music silent at volume 0. `play` sees it as still playing and does nothing.

### 1.3 soundStop(id, ms)
```
wasBgm = (curBgm==id), then clear curBgm. If curSub==id, clear curSub and set wasBgm=false.
Effect: key-off and kill (no fade).
Song:   fadeSlot(ms, wasBgm, wait=1)
        if wasBgm: log "BGM cannot be stopped"; play(1, 0, 0xFF, 1)   // replaced by null_bgm
        else:      kill(id)
```

### 1.4 Volume groups and ducking
- `FUN_80166D48(vol, ms, music, fx)`:
  - Volume group: 0xFF when both flags are set, 0xFD (music) when only music is set, 0xFE (effects) when only fx is set.
  - It overwrites the stored volume of every active entry of that kind.
  - It then calls `sndVolume(vol, ms, group)`.
- At init: the music volume group is set to 127 and the effects group to 100.
- Ducking (start menu 0x8000D710, PC box std024):
  1. On open, `FUN_801661D0(0x55, 500, 1, 1)` pushes each active work's volume onto its stack (3 deep, depth saturates at 2).
  2. It then sets the volume to `min(0x55, current)` over 500 ms.
  3. On close, `FUN_801660D8(500, 1, 1)` pops and restores over 500 ms.

### 1.5 std130 (fanfare, blocking)
1. Pause the music (FUN_801668DC, refused for id 1):
   - `sndSeqVolume(0, 50 ms, seq, 0)`, then `sndSeqPause`, then set flag 0x10;
   - wait 50 ms.
2. FUN_801653CC:
   - allocate a 64 KB temporary buffer, resource 0x408;
   - load the song, attach the work and start it;
   - yield while its status is 2;
   - kill it and free 0x408.
3. Resume the music (FUN_801667D8):
   - apply its reverb, then `sndSeqContinue`;
   - `sndSeqVolume(work.vol, 1000 ms, seq, 0)`.

Callers:
- the battle win plays id 1013 through std130;
- the treasure pickup (`floorEventGetTresure`) calls FUN_801653CC(970) directly, so the music keeps playing underneath.

### 1.6 3D sound
- **The single listener** is created at boot (0x8000609C, registered as resource 2000) and updated every frame from `heroReadStick` with the hero position. Arguments:
  `sndAddListenerEx(li, pos, dir 0, heading (0,0,-1), up (0,1,0), front 250, back 250, speed 1000, 1, vol 0x7F, room 0)`
- **std107** calls:
  `sndAddEmitter(em, pos, dir 0, maxDis 10000, comp 0, flags 0x1F, fxId, maxVol 0x7F, minVol 0, room 0)`.
  - The slot is freed if the emitter is inactive.
  - Meaning of the flags bits is UNVERIFIED.

## 2. Id → MusyX mapping
- **Songs:** sndSeqPlayEx(group = e[1], SongID = e[4], data = the song file at e[3]). The SongID matches an `SND_MIDISETUP` entry (0x54 bytes) in the group's MIDI-setup list.
- **Effects:** `sndFXStartEx(fxId)`. FX ids are global across all pushed groups.

| id | kind | group | MusyX id | file / FX macro | reverb |
|---|---|---|---|---|---|
| 1 | song | 0 | 3 | null_bgm | 0 |
| 3 | song | 0 | 1 | watercity | 9 |
| 4 | song | 0 | 2 | stand | 5 |
| 7 | song | 0 | 6 | darkside | 4 |
| 28 | song | 0 | 27 | me_news | 8 |
| 34 | song | 1 | 77 | wind_atmos_loop | 5 |
| 36 | effect | 6 | FX 858 | macro 435 | — |
| 41 | effect | 6 | FX 863 | macro 2313 | — |
| 80 | effect | 6 | FX 902 | macro 2286 | — |
| 1131 | song | 0 | 59 | ev_shadow | 3 |
| 1133 | song | 0 | 61 | me_chukei | 8 |
| 1160 | effect | 6 | FX 949 | macro 463 | — |

Effect ids that scripts pass to soundSetBackgroundMusic — 90, 1088, 1098, 1160, 1177, 1181, 1182 — are presumably ambient loops (UNVERIFIED that they loop). They start without stopping the music and without changing `curBgm`.

- **MusyX FX table** (proj group +0x1C): `u16 count, u16 pad`, then `{u16 fxId, u16 macro, u8 maxVoices, u8 priority, u8 vol, u8 pan, u8 key, u8 vGroup}`.
  - Defaults are vol 127, pan 64, key 60, vGroup 0; priority is 129 or 255.
  - The maxVoices/priority order follows AxioDL and is UNVERIFIED.
- **Group FX/song ranges:**

  | group | id | contents |
  |---|---|---|
  | music | 0 | songs 0–76 |
  | atmos | 1 | song 77 |
  | se | 6 | FX 856–976 |
  | se_battle | 5 | FX 201–855 |
  | se_motion | 8 | FX 977–979 |
  | nakigoe | 4 | FX 0–200 |
  | nakigoe2 | 9 | FX 980–1166 |

## 3. Loading
- **Boot:**
  1. Wait for FSYS group 0xA (common.fsys).
  2. Init the sound system.
  3. FUN_80165F84 registers buffer 0x406 (64 KB) and 0x407 (8 KB), and zeroes both current ids.
  4. A thread pushes the music bank (wave 1), then the effects bank (wave 2).
  5. Boot-step 0x800364C8 pushes waves 5, 4 and 6, then plays the GS logo movie.
  6. Boot-step 0x800363BC pushes waves 3 and 7.
  7. 0x800365E0 → FUN_80165F40 loads FSYS group 0x99 (bgm_archive) and waits.
- **All groups stay resident.** There is no sndPopGroup in this unit. ARAM size is 0x9FC000.
- **Group push (FUN_80167118):**
  1. `GSresGetResource(0xA, id)` for proj, sdir and pool.
  2. `sndSetSampleDataUploadCallback(0x80167298, ARQ chunk size)`; the callback streams the `.samp` file from DVD at the requested offset.
  3. `sndPushGroup(proj, gid, NULL, sdir, pool)`, then set the pushed flag.
- **Songs are loaded per play request.** The decompressed bgm_archive entry is copied into the 0x406, 0x407 or 0x408 buffer (pre-filled with 0xE0). The load appears synchronous (UNVERIFIED).
- **Song file format: confirmed MusyX ARR.**

  | offset | field |
  |---|---|
  | +00 | tTab |
  | +04 | pTab |
  | +08 | tmTab |
  | +0C | mTrack |
  | +10 | info (bits 0–27 = BPM; bit 30 = already shifted; bit 31 = has tsTab) |
  | +14 | loopPoint[16] |
  | +54 | tsTab |

  null_bgm, for example, has tTab 0x18, BPM 120 and loopPoint[0] 0x600.

## 4. MusyX init (0x80166E88)
- **Memory hooks:** malloc = 0x80167BB0 (32-byte aligned), free = 0x80167B70.
- **sndInit(64 voices, 48 music, 16 effects, 1 studio, flags 0, ARAM 0x9FC000).** The mixer runs at a fixed 32000 Hz.
- **One global reverb:** `sndAuxCallbackPrepareReverbHI(&0x80452500)`, then
  `sndSetAuxProcessingCallbacks(0, ReverbHI, &0x80452500, 0xFF, 0, NULL, 0, 0xFF, 0)`.
  This is ReverbHI on aux A of studio 0; there is no aux B.
- **Output mode:** `OSGetSoundMode` 1 → stereo, 0 → mono. FUN_80166CC0 calls `sndOutputMode` and then `OSSetSoundMode`. The options menu only toggles between stereo and mono; surround is never used.
- **Message callback 0x80167040:** when a sequence ends, it releases the matching work.
- **Reverb presets** (`reverb_params.tsv`; field order coloration, mix, time, damping, preDelay, crosstalk):
  - All have coloration 0, mix 1, damping 0, crosstalk 0.
  - Time / preDelay per preset:

    | preset | time | preDelay |
    |---|---|---|
    | 0 | 1.8 | 0.02 |
    | 1 | 1.8 | 0.02 |
    | 2 | 2.0 | 0.02 |
    | 3 | 1.6 | 0.06 |
    | 4 | 1.6 | 0.10 |
    | 5 | 3.0 | 0.10 |
    | 6 | 1.2 | 0.04 |
    | 7 | 1.8 | 0.06 |
    | 8 | 2.0 | 0.10 |
    | 9 | 2.5 | 0.04 |
    | 10 | 2.0 | 0.04 |
    | 11 | 1.0 | 0.10 |

- **Reverb is chosen per song, not per room.** When a song starts or resumes with a preset index that is nonzero and differs from the current one:
  1. `_sndSetReverbParm` copies the preset to 0x80452500+0x1C8 and sets tempDisableFX (+0x1C4) to 0.
  2. `sndAuxCallbackUpdateSettingsReverbHI` applies it.

  Effects never change it.

## 5. Pokémon cries
- **std218(species)** → `etctoolSetPokemonNakigoe` → `start(speciesTable[species] + 0xE, 0, 0)`.
  - This plays the effect at its stored volume (default 127), with no stop and no change to `curSub`.
  - Species record +0x10 is the national dex number.
- **Mapping:**
  - Kanto/Johto species map to sound ids 570–769: group 4, FX 1–200.
  - Hoenn species map to sound ids 770–955: group 9, FX 980–1166.
  - Full list in `cries.tsv` (386 species).
- **In battle:** `FUN_8021EE48` and `fightOutPokemonDasuEffect` use `pokemonGetStatus(species, 0x61)` (the same field) at volume 0xFF.
- **There is no pitch or variant logic in game code.** Any variation must come from the MusyX macros.

## 6. Room- and UI-driven sound
- **Rooms:**
  - The engine never starts music on room entry. The room's sound script (record +0x28) calls std104 — for example M1_gym_1F func 0 does `if flag 2480: soundSetBackgroundMusic(1117, 0, 127)`.
  - Because of §1.1, re-entering a room with the same music continues it seamlessly.
  - `floorLinkWithSE` plays effect 1017 (0x3F9) when its last argument is nonzero.
- **Ambience comes from three sources:**
  - song 34 (`wind_atmos_loop`) played via std105 into the sub slot;
  - song 1228 (`siren_atmos`);
  - snd_se effects started via std104 or std105.
- **Battle:**
  - `std102_battleStart` does `soundSetBackgroundMusic(1, 1000, 0xFF)` and `scriptSoundStop(1000)`, then plays the wipe effect (wipe table +2).
  - **Battle music:**
    1. the encounter's +0xC;
    2. else fight floor status 6;
    3. else the trainer kind's +8;
    4. else id 1.
  - **Environment effect:** fight floor +0x10, played via std105.
  - **End of battle:**
    1. `soundStop(env, 1000)`;
    2. `std138(1, 1000, 1000, 0xFF)`;
    3. `_fightFinalize` sets music id 1;
    4. the room script then restores the field music.
  - The full dump is `battle_sound_ids.tsv`. The status-number → field mapping is UNVERIFIED.
- **Message boxes:**
  - **`FUN_80166a28(0x24)` on message advance is sound id 36 = snd_se FX 858.** `msgctrlKeyWait` plays it on A-press, skipped when window flag 2 (auto) is set. The same id is the "decide" sound in menu SE sets 1, 2 and 5.
  - A message control code plays an arbitrary id via std104 at volume 0xFF; `msgctrlSndWait` blocks while it plays.
  - Text printing (`FUN_800FC7E0`) starts an effect when printing begins and kills it when printing stops. The id depends on window type: 1 → 0x57, 2 → 0x58, 3 → 0x59, 4 → 0x497, 5 → 0x498 (UNVERIFIED interpretation).
- **Menus:** `menuPlaySe(menu, kind)` picks set `menuData.byte0 & 7` from `menu_se_sets.tsv` (default set 1 shown):

  | kind | slot | sound id |
  |---|---|---|
  | 1 | [2] | 35 |
  | 2 | [3] | 36 |
  | 3 | [4] | 37 |
  | 4 | [0] | 39 |
  | 5 | [1] | 40 |

  - My guess for the meanings is cursor, decide, cancel, open, buzzer (UNVERIFIED).
  - Sets 3 and 7 use 965–967 instead of 35–37.
- **Footsteps:** the only se_motion ids are 568 and 569 (FX 978/979). main.dol never loads them as immediate values, so they are probably triggered from model or animation data (UNVERIFIED).

## 7. Port checklist
1. Load the sound, song file, wave and reverb tables from common_rel.
2. Push all 7 MusyX groups at boot.
3. Keep 34 work slots.
   - Identify the music and sub slots by searching for the work that owns buffer 0x406 or 0x407.
   - Track `curBgm` and `curSub` as plain ids.
4. Implement `play`, `fadeSlot`, `soundStop`, kill, status and `start` exactly as in §1.
   - Natives must be able to block the script thread for 50 ms or for the fade time.
   - Mask volumes with `& 0x7F`.
   - Effects ignore fade times and stop with key-off.
   - Songs fade through `sndSeqVolume` in ms.
5. Start songs with `SND_PLAYPARA{flags = 4, speed = 0x100, vol.time = fadeIn, vol.target = vol}`, and apply the song's reverb preset on start and on resume.
6. Initial master faders are music 127 and effects 100. Run the mixer at 32 kHz, stereo or mono.

## DSP mix (recomp)

The recomp runs MusyX on the AX ucode model in `colo-recomp/src/dsp_ax.rs`. Polyphase sample-rate conversion uses
the DSP DROM table, which the game doesn't contain; `free_coefs` rebuilds Dolphin's free replacement (three 4-tap x
128-phase windowed-sinc tables) from its formula, so the mix matches Dolphin HLE's. `COLO_DSP_COEF` takes an
official 4 KB dump instead (or `linear`). `cargo run --release -p colo-recomp --example audio_dump <snap or disc> <frames>
<out.wav>` writes the mix for comparison with Dolphin's `dspdump.wav` (set `[DSP] DumpAudio` and `DumpAudioSilence`).
