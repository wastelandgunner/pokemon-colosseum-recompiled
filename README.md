# Pokémon Colosseum: Recompiled

> **Read this first.** This is a hobby experiment, not a serious project. It was built with very heavy
> AI assistance: most of the reverse engineering notes, the Rust code and this README were written by
> an AI model (Claude) working from my own copy of the game and a Ghidra database, with me steering.
> Expect rough edges, wrong guesses and code nobody has carefully reviewed. Don't rely on it for anything.
>
> No game data is included or will be accepted: no disc images, no extracted assets, no screenshots.
> Every tool reads your own legally dumped disc (GC6E01, NTSC-U rev 0) at runtime, read-only.
> Pokémon and Pokémon Colosseum are trademarks of Nintendo, Creatures Inc. and GAME FREAK inc.;
> this project is not affiliated with or endorsed by any of them.

Code license: MIT (see `Cargo.toml`). The MusyX runtime is not included: it is a git submodule pointing at
[AxioDL/musyx](https://github.com/AxioDL/musyx), fetched from there when you clone.
Fonts: SIL OFL 1.1 / Bitstream Vera (`crates/colo-viewer/assets/FONT-LICENSE.md`).

## Getting the code

Clone recursively so the MusyX submodule (`crates/colo-audio/musyx`) is fetched too:

```sh
git clone --recursive https://github.com/wastelandgunner/pokemon-colosseum-recompiled.git
```

Already cloned without it? Run `git submodule update --init` inside the repo.

## colo-rs

Rust groundwork for a native reimplementation of Pokémon Colosseum (GC6E01, NTSC-U rev 0).
No game data is included; everything reads your own disc image, read-only.

| crate | what |
|---|---|
| `colo-formats` | GCM disc/FST, FSYS archives, LZSS, `.msg` string tables (UTF-16BE + control codes), `.scd` header + CMS instruction decoder, REL modules, `common_rel` tables (rooms, character models, interaction points, global script), per-room `.rel` lists (NPCs, entry points, field cameras) |
| `colo-script` | CMS script VM: cooperative threads, frames/locals, yield, callstd natives through a `Host` trait, VM-side thread natives (std 3/5/6/71), generated table of all 246 natives (`natives.rs`) |
| `colo-export` | glTF 2.0 `.glb` + PNG writers (no dependencies) |
| `colo-tool` | CLI over all of the above |
| `colo-audio` | Sound: the decompiled MusyX runtime (AxioDL/musyx, a git submodule in `musyx/`, built for its PC renderer) and the game's own sound layer (`sound.c` at main.dol 0x801653C4..) over it |
| `colo-viewer` | Bevy 0.19 walkable room viewer: Wes on the collision floor with walls, animated NPCs/props (HSD joint animation, GPU skinning), talk to NPCs (their real talk scripts), warps through doors into the next room |

Zero external dependencies except `colo-viewer` (Bevy, cpal) and `colo-audio` (a C compiler for MusyX, via `cc`).
`cargo build` skips the viewer. Linux needs `libasound2-dev` for the viewer's sound output.

```sh
cargo build --release
cargo test
D=/path/to/your/own/GC6E01.iso   # your own dump; nothing is included here
./target/release/colo-tool fsys "$D::M2_out.fsys"              # list an archive straight from the disc
./target/release/colo-tool msg  "$D::M2_out.fsys" | head        # dialogue with control codes
./target/release/colo-tool dis  "$D::M2_out.fsys" | less        # script disassembly with native names
./target/release/colo-tool run  "$D::M2_out.fsys" --func 2 --ret 81=1   # run a script, natives traced
./target/release/colo-tool check "$D"                           # run every function of every script
./target/release/colo-tool map  "$D" M2_out                     # NPCs (model, position, talk script), entry points, warps
./target/release/colo-tool maps "$D"                            # parse every room
./target/release/colo-tool room "$D" M2_out --talk 2            # emulate the room load, then talk to NPC 2
./target/release/colo-tool model "$D::M2_out.fsys" out/          # every model in an archive -> .glb (open in Blender)
./target/release/colo-tool export-room "$D" M2_out out/          # map.glb + NPC .glbs + scene.json placements
./target/release/colo-tool ccd "$D::M2_out.fsys"                 # collision summary
./target/release/colo-tool flare "$D" T1_ancient_colo out/       # lens flare (.gfl) elements + textures, sun position
./target/release/colo-tool check-models "$D"                     # load all 955 models on the disc
./target/release/colo-tool materials "$D"                        # material / TObj / TEV setups over the disc
./target/release/colo-tool sound "$D" 3 watercity.wav 30         # sound id 3 through the game's layer + MusyX -> WAV
cargo run -p colo-viewer --release -- "$D" M2_out                # play Pyrite: WASD walk, Shift run, F talk, walk through doors
```

## Models (`colo_formats::hsd`)
HSD archives -> static bind-pose meshes: joint tree (SRT, inverse bind matrices, `JOBJ_INSTANCE` re-drawing),
DObj/MObj/TObj/PObj, vertex descriptors (direct/index8/index16, all component types and fractions), display
lists (triangles, strips, fans, quads), envelope skinning (single/multi-weight, coordinate-system rules from
StarsMmd's Blender addon), GX textures (I4/I8/IA4/IA8/RGB565/RGB5A3/RGBA8/C4/C8/C14X2/CMPR + palettes).
`check-models` loads all 955 `.dat`/`.rdat`/`.pkx` models on the disc (2.8M triangles, 6,701 textures)
with no failures or warnings. Texture coordinates use the TObj's full matrix (`MakeTextureMtx`:
S·R·T with S = repeat/scale, R = -rotZ, T = -translate, mirror-wrap shift), baked into the UVs. The base texture is the
first UV-mapped TObj on the diffuse light map; reflection/highlight maps are skipped. Materials keep every TObj
(`Material::layers`: flags, blending, TObjTevDesc, raw UV sets per GX_VA_TEX0..7) and the PE desc.
`colo_formats::tev::build` ports the game's TEV graph (`MObjMakeTExp` / `TObjMakeTExp` / `MakeColorGenTExp`, see
NOTES §8 Materials / TEV); `tev::eval` evaluates it on the CPU with the GX integer TEV maths. The viewer runs the
same graph in `tev.wgsl` (`tevmat.rs`): all TObjs, colour/alpha maps, KONST/TEV0/TEV1 colours and their animation,
reflection/highlight maps, alpha compare, blend and z modes. `colo-tool materials "$D"` tallies render modes, TObj
setups and TEV descs over the disc. Not handled yet: bump/indirect stages, GX lighting (the viewer lights the TEV
channels with Bevy lights), PKX animation metadata.

## Animation (`colo_formats::hsdanim`)
ModelSet +4 -> AnimJoint trees (mirroring the joint tree) -> AObj -> FObj keyframe tracks (rot/trans/scale per
axis; constant/linear/Hermite spline; little-endian key streams). `Animation::locals(model, frame)` gives
per-joint SRT; `hsd::world_matrices` + `hsd::pose` skin on the CPU (used by `anims` with `COLO_POSE_OUT`).
Character set-0 indices: 0 bind/static, 1 idle, 2 walk, 3 run (Wes: 10 is a second run cycle; others unknown).
Legs are **IK-driven**: the leg tracks animate foot targets, not leg bones. `hsd::world_matrices` solves the
chains like sysdolphin's `JObjResolveIKJoint1/2`. An IK chain is JOINT1/JOINT2/EFFECTOR joints (flags bits 21..22)
with IK-hint RObjs (bone length, twist, flip bit 4), an effector target RObj (JOBJ subtype 1), an optional pole
(subtype 3) and an optional angle limit (LIMIT subtypes 5/6). Ordinary joints with a JOBJ subtype 1 RObj are
position constraints (held props such as canes) and take the referenced joint's world position.
Map model sets animate props (signs, fans, water) with their anim 0.
Joint NODE (11) / BRANCH (12) tracks show (> 0.5) or hide a joint's meshes (and its descendants for BRANCH).
**Material/texture animations** (`load_mat_animations`): ModelSet +8 -> MatAnimJoint trees (mirroring the joints)
-> MatAnim per DObj (+4 material AObj, +8 TexAnim list) -> TexAnim matched to the MObj's TObj by texmap id, with
an image table (+0xC) and TLUT table. Tracks: TObj 1 image index, 2/3 translate U/V, 4/5 scale, 6..8 rotate, 9
blend...; MObj 4..6 diffuse, 10 alpha (stored as 1 - alpha). `GSmodel` links the texture animation to the joint
animation by default (`_modelLoad` flags 0x2001): same index, frame, rate and loop.

## Viewer (`colo-viewer`)
- `model.rs`: one entity per joint (rest SRT); models with IK/constraints get a full CPU world-matrix solve per
  frame, written back as parent-relative transforms; meshes skinned on the GPU with a per-mesh joint palette when an
  animation moves their joints, static otherwise; `Animator` plays tracks at 60 fps. Back-face culling follows
  the PObj cull flags.
- `colsys.rs`: port of `GScolsys2`. Each `.ccd` group is a collision object with floor (+0x24), wall (+0x28),
  region (+0x2C) and check (+0x30) lists plus a spatial grid. Floors give every height under (x, z)
  (`fn_8010E138`). Walls are one-sided spheres swept in steps of the radius: faces, then flagged edges, then
  vertices; a hit pushes the point to radius + 0.0001 (`fn_8010F320`/`EFE4`/`E53C`). Touch regions: the
  sphere against region triangles facing the move; pass regions: the segment crossing them; check regions:
  the A ray (+5 up, 10 ahead). `collide.rs` (older, two-sided) still serves NPC script moves.
  The +0x38 list (ccd slot 5, 0x30-byte triangles) holds sun occluders for `GScolsys2Sun`.
- `flare.rs`: port of `flareDraw` (lens flare; only `T1_ancient_colo` has a `.gfl` and a sun res id). The sun is
  the root joint of the sun model set; `GScolsys2Sun` hides it; in front of the camera, additive element quads
  along the sun→centre axis plus a full-screen glare; behind it, a subtractive glare. Drawn by an orthographic
  overlay camera (render layer 2) in a 640×480 frame matched to the window height. Screenshot:
  `Working/screenshots/T1_ancient_colo_flare.png` (`COLO_CAM=0,50,0,132,70`).
- `heromove.rs`: port of `heroMove`. The hero moves only inside `heroMoveMain` = std 31 `playerProcessEvents`,
  which the room's main loop calls once per frame; while it runs a cutscene, or an event it started, nothing
  calls it, so Wes and Rui stand still (also while the hero has people flag 0x80000000, set by
  `playerLockMovement`). Each call runs last frame's check/touch/pass events synchronously, opens the field
  menu on X/Y/Start, moves Rui (`fn_8012DE94`), then Wes (`moveLeader`: A → talk (`fn_8018CD08`: within
  12 + radii, 40° of his direction, no wall between, not behind Rui) or the check ray; else the stick).
  People moves (`fn_8018E9B4`) go through the collision system by flags: Wes 0x40000F00 (events, human,
  floor, wall), Rui 0x701; walls/sphere at +8.5, floors: highest less than 10 above. Human cylinders: radius
  CharacterModels +0x18, 17 high. Rui: placed 9 behind Wes on entry (`initFloor`); follows the leader trail
  (a point every 12 units, 20 kept) up to 1 unit per frame, stopping 12 from Wes and backing off inside 9;
  turns to face Wes beyond her head limits (+0x1C/+0x20); bodies turn per frame like `peopleTurnUpdate` (free Wes
  eases at the stick amount, everyone else π/20·rate per frame); heads aim with `peopleHeadLookUpdate`; put back on the newest off-screen trail point after
  300 frames 100+ away and off screen. Walk/run use `updateAnimation`'s blends (idle/walk below 0.4, walk,
  walk/run above 1, backward walk below 0) at rate 0.5.
- `world.rs`: disc or directory source, room table, floor id -> archive via FSYS group ids, NPC models from the
  room archive then `field_common.fsys`.
- Doors and warps: touch (method 2) and pass (method 1) regions crossed by Wes run their interaction points.
  Common script func 5 = `floorEventCtrlDoor(room, door, 0)` opens a door: it plays the door's open animation
  on its map model set at rate 0.5 and switches off the door's collision group. Func 4 =
  `floorLinkWithSE(floor, entry, se)` warps once the door has finished opening; the hero is frozen meanwhile.
- Room setup like `_floorInitialize`: every door of the floor starts closed (mode 3). The room's modify script
  (room record +0x2C) runs with the viewer host and applies `mapObjectPlayAnimation`, `scriptSetCol` and
  `cameraSetFloorDefault`. Every map model set starts its anim 0 and texture anim 0 at rate 0.5, looping
  (`floorInitMap`/`floorOpenObject`); on floors 0x03, 0x0B-0x0E, 0x15, 0x24-0x26, 0x72, 0x75, 0x7B, 0xCA, 0xD0,
  0xDE the texture animations of animated sets are unlinked and run at 0.35.
- Map objects: `mapObjectSetVisibility` (std 114) hides/shows a map model set (resource id = map file id << 16 |
  0x1000 | set), `mapObjectPlayAnimation` (115) sets joint and texture animation, `mapObjectStopAnimation` (116)
  freezes it. Ids below 0x10000 are characters (the natives fall back to the character versions).
- TVs (`tv.rs`): room record +0x48 lists TV channels (w, h, id, screen map object, programme model, anim, `.cam`).
  At floor setup channel 0 is selected (`FUN_80117E58`); `mapSetTextureChange(id)` (std 171) switches channel;
  `tvSetChannelByStory` (std 211) picks the news for the story flags. Each frame the programme model is rendered
  with the channel camera (its own fov and aspect into a square target, `FUN_80117D14`) and that image replaces the
  first TObj image of every material of the screen set (`GSmodelSetTextureChange`). Pyrite: M2_houseA_1F (16
  channels), M2_building_4F (3). Checking a TV runs global script func_19 (camera on the screen per floor), which
  calls `tvNewsEvent` (std 182, 0x801C9F5C): lock the hero, pick the news for the story (setting its "seen" flag),
  wait 3 s, show the bulletin text (0x426E.. / 0x439D..), unlock.
- Field camera (`floorUpdateFieldCamera` / `cameraUpdate` mode 0):
  - The map REL's camera points (pos, distance, height, rot_y) are blended by inverse squared distance from
    the player + (0, 14, 0).
  - Each frame keeps 25% of the previous value. Eye = player + RotY(rot_y)·(0, height, distance), looking at
    player + 14 up. FOV is 30°, from the map's scene camera.
  - Rooms without points use `cameraSetFloorDefault`, or else the map's scene camera (`scene_data` +4).
  - Movement is relative to the camera yaw. Tab switches to a free-fly camera.
- Scripts (`script.rs`):
  - Each room runs a persistent CMS VM at 60 Hz. At load it starts the sound script (+0x28) and each
    NPC's move script `(group, index, people, 0)`, runs the modify script (+0x2C) and the entry script
    (+0x30) to completion, and starts the room's main loop (+0x34). The main loop checks story flags every
    frame and launches cutscenes (e.g. Johnson bursting into the police station on the first visit).
  - `characterEventFlagsSet/Clear` (std 77/76) set/clear people flags 0x100 | 0x400 and clear/set 0x80000000
    (the hero with 0x80000000 doesn't move). `shadowPokemonSetState(i, state)` (std 243) sets the Shadow Pokémon
    table entry's flags (+0xC := 1, +0x1C := state; table = main.dol global 0x80478F6C, registered by
    common_rel's prolog), e.g. DUKING's hostage Pokémon in Pyrite.
  - Events started by `heroMoveMain` (talk, check, regions) run at once like the game's synchronous call;
    one that waits keeps std 31 blocked until it ends. Blocking natives return
    `Pending` until their condition holds: movement, one-shot animations, messages, Yes/No, doors, `sleep`.
  - Implemented natives:
    - Dialogue and Yes/No: `characterTalk`, `dialogDisplayYesNoMenu`, `dialogWaitForMessage` (returns the
      choice; 0 = Yes), `characterEndSpeech`.
    - Characters: visibility, position, rotation, turning, move/run to a point or toward a character or
      the player, set/wait animation, bounds checks.
    - Player lock/free, flags, `sleep`, doors, map object animations, collision groups.
    - Event cameras: `GSscene_SetMode`, `cameraSetPosition`, `cameraSetRotationAboutAxes`,
      `cameraMove*XYZ`, `cameraReturn`.
    - `battleStart` shows a placeholder and `battleGetResult` returns 2 (won).
  - Characters are actors addressed by index: 100 = Wes, 101 = Rui (see `heromove.rs`). Move type 2 NPCs
    wander around their spot. std 63/64 are `characterSetFlags`/`characterClearFlags` (GoD Tool had them
    swapped); std 151 switches a person's human collision (GoD: "characterFaceDirection").
- Event cameras: `cameraPresetFromFile(group, resource, flag, start)` plays a room `.cam` animation (the resource
  is the FSYS identifier) at rate 0.5: eye and interest WObj tracks 5/6/7, plus CObj roll (9) and fov (10). The
  camera holds the last frame until `cameraReturn` / `GSscene_SetMode(0)`. `cameraWaitSyncAnime(1)` blocks
  until the animation ends. `transitionBegin(3|2, s)` fades to / from black; `transitionWait` blocks.
- A button (F / Enter / Space): talk to the person chosen by `fn_8018CD08`, else the check ray at check
  regions → their method-3 interaction points (signs, items: "A worn gear is on the floor.").
- Menus (`menu.rs`):
  - Field menu (`fn_8000D710`, opened by X / Y / Start: pad mask 0x1C00; X key here): menu 0x41 with POKéMON,
    P★DA, ITEMS, EXIT (common_rel messages 0x136/0x138/0x137/0x13A, item layout from the menu item table at
    0x802EF0A8). The field is frozen while it is open (scripts, characters and animations pause, like
    `GSgappBlock` + `GSmodelAllPauseAnimation`). The three screens behind it aren't implemented yet. It has no
    Save entry: Colosseum only saves at PCs.
  - PC: A on a PC runs common script thread 0x0596000D, which calls `mapSetReturnMap(floor, prev, entry)` and
    `mapWarpToMenuRoom(0x391)`. The viewer overlays the PC menu (`fn_8003842C`, menu 0x1D: POKéMON Storage,
    Item Storage, Save, Exit; the storage screens aren't implemented). Exit/B returns to the return map, so
    the room reloads with Wes at the PC's entry point.
  - Save (`fn_80038A0C`): "Would you like to save your progress?" → Yes → (if a save exists) "Overwrite current
    save file?" (No asks again) → "Saving to the Memory Card in Slot A..." → "Your progress has been saved!".
- New Game story state (`story_start.rs`): with no save (or `COLO_NEW_GAME=1`), a game started in a Pyrite room
  begins with the flags of first reaching Pyrite (`PYRITE_ARRIVAL`, derived with `colo-tool story-explore`); one
  started in Outskirt Stand gets the real New Game flag 0x8D0. `COLO_STORY=none|start|pyrite` overrides.
- Save file: `~/.local/share/colo-viewer/save.txt` (or `$COLO_SAVE`), written only by the PC's Save. Like the
  game it stores the return map (room, entry index, previous floor) plus the story flags, so Continue starts
  in front of the PC. `colo-viewer <gcm>` (or `continue`) resumes the save; with a room name, it starts there
  keeping the saved flags. Esc quits without saving. `COLO_NEW_GAME=1` ignores the save;
  `COLO_FLAGS="1144,-12"` sets/clears flags (pick a story point).
- Global strings: common_rel embeds the game's string tables (menus, items, memory card messages; found by
  their `.msg` header), and the PC menu room's `system_tool.msg` (pcbox_menu.fsys) has the save prompt. Rooms
  layer their own tables on top.
- Story flags live in a viewer-side set that persists across rooms and goes into the save; the flag natives
  and doors use it. Door logic follows `floorEventCtrlDoor`: an open door's open flag
  stays set, so it is not re-opened and comes back opened on re-entry; a door whose lock (close) flag is unset
  stays shut.
- NPC idle = the model's first animation slot (CharacterModels +1: idle, +2 walk, +3 run).
- Message window (`dialogue.rs`), drawn like the game into a 640x480 layer shown in a 4:3 frame:
  - Field window (menu 0x40): origin (53,299), 535x121; background `fn_8001EC08`: black at alpha/2 with 15 px
    rounded corners and a 15 px feathered edge (vertex-coloured fan + strip, rasterised in software). Opens
    with alpha 0 → 255 over 2 ticks.
  - Text: the game's own bitmap font 0 (22 px) from main.dol (0x8027A500), white, no shadow, one character
    per tick (A completes the page), 33 px line pitch, space = 11 px. Pages end at control 0x03 (key wait,
    with the cyan next-page arrow sprite 0xB9 bobbing 4 px over 50 ticks) or 0x02. `‱` is the bubble glyph.
  - YES/NO (menu 0x12 at (50,200)): black panel (alpha 179), "YES"/"NO" in font 1 with a (+1,+1) shadow,
    the Poké Ball cursor, and the pill frame, all from the GTX atlas at main.dol 0x803254E0.
  - GameCube blending happens in gamma space; dark pixels' alpha is raised for Bevy's linear blending.
- Movement matches the game:
  - Stick amount = each axis clamped to ±56, divided by 28, capped at 2 (0x8012E388). WASD is a full tilt
    (run), Shift + WASD a half tilt (walk), arrow keys the d-pad; a gamepad's left stick is analog. Direction
    = camera yaw + the game's stick angle: (π/2)·sin(min(|x|/|y|, 5)/(10/π)) off "up", mirrored per quadrant.
  - Speed per 60 Hz frame (0x80188214): amount 0..0.4 → 0..½ walk, 0.4..1 → ½ walk..walk, 1..2 → walk..run.
  - walk = CharacterModels +0x10 stride ÷ walk animation frames; run = +0x14 stride ÷ run animation frames.
    Wes and Rui: 9/30 and 13/17 → 18 and 46 units/s.
  - Character animations play at rate 0.5, as the game sets.
- Dialogue uses the game font; the remaining Bevy UI (field/PC menus, status line) uses Poppins (OFL) with
  ★ ☆ ♂ ♀ merged in from DejaVu Sans (`assets/make-font.py`).
- Interiors render on black (the clear colour); outdoor maps draw their own sky dome.
- Test hooks: `COLO_WALK=x,z[;x,z...]` (waypoints; a warp moves on to the next one), `COLO_SHOT=file.png`, `COLO_SHOT_FRAME`, `COLO_TALK=n`, `COLO_COLLISION=1`,
  `COLO_PRESS=n` (press A every n frames while text is up), `COLO_TRACE=1` (print every native call),
  `COLO_NO_EVENTS=1` (don't start the room main loop), `COLO_NO_RUI=1`, `COLO_CAM=x,y,z,yaw,pitch`, `COLO_REGIONS=1` (print regions, their interaction points and NPC positions),
  `COLO_ANIM=anim,frame` (freeze the player on one animation frame), `COLO_CHECK=x,z,yaw` (stand there and
  press A at frame 20), `COLO_KEYS=frame:Key,...` (press keys on given frames, e.g. `30:KeyX,40:KeyS,50:Enter`),
  `COLO_DUMP2D=file.png` (write the 640x480 message layer), `COLO_FOLLOW=1` (log Wes/Rui positions).

## Sound (`colo-audio`)
- Tables from `common_rel` (`Common::sound_tables`): 1236 sound ids x 12 bytes (main.dol global 0x80478FAC: bit 7 =
  MusyX effect, MusyX group, ReverbHI preset, song file, SongID/FX id), 79 song files (FSYS ids in `bgm_archive.fsys`,
  group 0x99), 8 wave banks (project / sample directory / pool FSYS ids in `common.fsys` + `/sound/*.samp`), 12
  ReverbHI presets; `Common::species_voices` = species table +0xE (cry sound ids 570–955).
- Boot like `FUN_80166E88`: `sndInit(64, 48, 16, 1, 0, 0x9FC000)`, one ReverbHI on aux A (preset 0), music master
  127 / effects 100, then the banks pushed in the game's order 1, 2, 5, 4, 6, 3, 7.
- `layer::SoundLayer` is `sound.c` function for function: 34 work slots, the music (0x406, 64 KB) / sub (0x407,
  8 KB) / fanfare (0x408) song buffers found by owner search, `curBgm` / `curSub`, status checks that free finished
  sounds, `& 0x7F` volumes, per-song reverb presets, pause/resume, duck/unduck, the receive-message callback.
  Blocking operations (`FUN_80165D0C` waits, the fanfare loop) are `SoundTask`s polled once per script frame.
- Natives: std 104 `soundSetBackgroundMusic`, 105 `soundPlaySub` (effect, or a jingle/ambient song in the sub
  slot), 106 `soundStop` (the music can't be stopped: it fades and null_bgm replaces it), 137 `soundSetVolume`,
  138/139 `soundChangeBackgroundMusic`/`soundChangeSub` (fade out, wait, play), 109 `soundSetMusicMasterVolume`,
  130 `soundPlayFanfareAndWait`, 153 `scriptSoundStop`, 218 `soundPlayPokemonCry`, 228 `soundGetCurrentBackgroundMusic`.
- Viewer: output on its own cpal stream at 32 kHz (else the device rate); with no device the engine still runs
  in real time, discarded. Message key wait plays sound 36; the Yes/No menu cursor 35 / decide 36 (menu SE set 1).
  `COLO_NO_AUDIO=1` turns output off, `COLO_SND_TRACE=1` prints every start.
- Deviations: the game's MusyX holds 4096 macros (`dataInsertMacro` 0x80151B84), the build patches its copy
  to match; snd_music sample 581 has loop start 0xFFFFFFFF (length 1), which the PC renderer rejects, so it is
  made one-shot. Not yet: 3D emitters (std 107), the text-printing sound (window type → 0x57/0x58/0x59/0x497/0x498),
  footsteps (se_motion 568/569, triggered from model/animation data), battle music.

## Room loading (from the decompiled engine)
- Room record: `common_rel` rooms table (GoD symbol 14, 0x4C bytes, 196 rooms). Names the room's FSYS group,
  floor id, five script thread ids and the main.dol globals its map REL fills in.
- Map REL: a few PowerPC instructions that store `&list` into those globals; parsed from their relocations.
- NPC talk = `call_sync(talk_script, [group, index, 0, 0])` (0x8012C0B4 updateChat);
  NPC AI = async thread `(move_script, [group, index, people, 0])` at priority 15 (0x801837D8);
  interaction points = `call_sync(script, params[4])` (0x80116D30); warps/doors are global-script funcs 4/5.
- `maps` parses all 167 rooms that have a map REL: 793 NPCs, 325 entry points, every NPC model resolves to its `.dat`.

## VM status
Semantics come from the decompiled interpreter (see `../../NOTES.md` §7). `colo-tool check` runs all
2,283 functions of all 175 scripts on the disc twice (every native returning 0, then 1): 6.8M instructions,
**zero stack faults**. The remaining warnings are arg-dependent (two handlers index data with a thread argument)
or busy loops that spin because stub natives never block.

Known gaps:
- `copyStruct` (op 32) tail is partially reconstructed; no shipped script uses it.
- Thread priorities are ignored; threads run in creation order each tick.
- The field engine's spawn arguments for talk/event handlers aren't known yet (handlers read up to local 5).
- Natives are all host-side; `TraceHost` just logs them. Implementing them is the port.
