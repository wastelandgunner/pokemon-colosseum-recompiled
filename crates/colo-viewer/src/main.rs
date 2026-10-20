//! colo-viewer: play through Pokémon Colosseum rooms loaded straight from the disc image.
//!
//! `colo-viewer <image.gcm | dir of extracted files> <room>`   e.g. `colo-viewer iph-pmc.gcm M2_out`   (no disc: a file dialog asks for it, then runs `recomp`)
//!
//! Controls: WASD run (full tilt), Shift + WASD walk (half tilt), gamepad left stick analog;
//! F talk / advance text / confirm (also Enter, Space; pad A), Backspace back (pad B), W/S move the cursor,
//! X field menu (pad Start/X/Y), Tab toggle free-fly camera (WASD/QE, hold right mouse to look),
//! C collision overlay, Esc quit (or back, in a menu).
//!
//! Saving works like the game: only from a PC's Save option (A on the PC). `colo-viewer <disc>` with no room
//! (or `continue`) starts from the save, at the entry point in front of that PC.
//!
//! Each room runs its scripts on a persistent CMS VM (`script.rs`): the room's sound script and NPC move
//! scripts start at load, talking and interaction regions start event threads, and the natives move NPCs,
//! play their animations, open doors, show dialogue and warp between rooms.
//!
//! Test hooks (env): COLO_CAM="x,y,z,yaw_deg,pitch_deg" (free camera), COLO_TALK=<npc> (talk at frame 20),
//! COLO_PRESS=<n> (press A every n frames while text is up), COLO_COLLISION=1, COLO_WALK="x,z[;x,z...]"
//! (waypoints), COLO_ANIM="anim,frame" (freeze the player's pose), COLO_REGIONS=1 (print regions, doors,
//! cameras, NPCs), COLO_TRACE=1 (print every native call), COLO_SHOT=<file.png> (screenshot at frame
//! COLO_SHOT_FRAME (default 90), then exit), COLO_KEYS="frame:Key,frame:Key,..." (press keys, e.g. 30:KeyX,
//! 40:KeyS,50:Enter; names are Bevy KeyCode names).

mod collide;
mod colsys;
mod dialogue;
mod heromove;
mod story_start;
mod tv;
mod flare;
mod audio;
mod battle;
mod gamebattle;
mod recomp;
mod recomp_menu;
mod settings;
mod warp;
mod perf;
mod fade;
mod wipe;
mod menu;
mod model;
mod save;
mod script;
mod tevmat;
mod particles;
mod shadow;
mod world;
mod worldmap;

use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use model::{animate, animate_materials, spawn_model, Animator, ModelAssets};
use menu::{Menu, PcStep};
use script::{Actor, Cmd, ScriptRt, HERO, PARTNER};
use std::sync::Arc;
use world::{Game, RoomData};

/// Height of the camera's look-at point above the player (`GSscene_SetCameraViewVector`, 0x80272088).
const VIEW_HEIGHT: f32 = 14.0;
/// Floors whose animated map sets get their texture animations unlinked at 0.35 (`floorInitMap`, 0x801134E4;
/// the check `FUN_800057A0() == 1` always holds).
const MAP_TEX_UNLINKED_ROOMS: [u32; 15] = [0x03, 0x0B, 0x0C, 0x0D, 0x0E, 0x15, 0x24, 0x25, 0x26, 0x72, 0x75, 0x7B, 0xCA, 0xD0, 0xDE];
/// Map object animations run at half rate (`mapObjectPlayAnimation` / `floorEventCtrlDoor`: rate 0.5).
pub const MAP_ANIM_RATE: f32 = 0.5;
/// Wes's animation slots (ken_b1): idle 1, walk 2, run 3.
const HERO_SLOTS: [u8; 8] = [1, 2, 3, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];

#[derive(Resource, Default)]
struct ViewState {
    show_collision: bool,
    free_cam: bool,
    pending_warp: Option<(u32, usize)>,
    /// Region the player stood in when the room loaded (ignored until left).
    ignore_region: Option<u16>,
    /// Test walk: waypoints the player walks to in order (COLO_WALK="x,z;x,z;..."); a warp skips to the next.
    walk_path: std::collections::VecDeque<Vec2>,
    /// Continue from an old save that has a position: the first room spawns the hero here.
    spawn_at: Option<(Vec3, f32)>,
    /// Continue from the save: the first room's entry point, and the floor the save came from.
    spawn_entry: Option<usize>,
    spawn_prev: Option<u32>,
    /// The message window took this frame's A press.
    a_used: bool,
    /// Test hook: press A for heroMoveMain on the next processed frame.
    force_a: bool,
}

#[derive(Component)]
pub struct RoomRoot;

#[derive(Component)]
pub struct MapModel;

#[derive(Component)]
struct Player;

/// Links a character entity (NPC root or the player) to its script actor and its animated model.
#[derive(Component)]
struct ActorLink { index: u32, anim: Entity }
/// A character riding another's model part (std 70): the parent it had before.
#[derive(Component)]
struct Rider(Option<Entity>);

/// The field camera (height/distance/yaw around the player, like `cameraUpdate` mode 0) plus free-fly state.
#[derive(Component)]
pub(crate) struct GameCam { distance: f32, height: f32, rot_y: f32, ready: bool, fly: Vec3, fly_yaw: f32, fly_pitch: f32 }

#[derive(Component)]
struct StatusText;

/// Full-screen black overlay for script fades (`transitionBegin`).
#[derive(Component)]
struct FadeOverlay;

/// Read the disc image once in the background so the OS caches it: the recompiled game reads the disc synchronously
/// on its frame thread, and on a cold cache (a hard disk after a reboot) its first battle loads took 0.5-0.8 s per
/// frame (pipeperf story10: 817 ms at the start, 582 ms at the first attack; nothing over 70 ms once cached).
/// COLO_NO_PREFETCH=1 skips it.
fn prefetch_disc(path: &str) {
    let p = std::path::PathBuf::from(path);
    if std::env::var("COLO_NO_PREFETCH").is_ok() || !p.is_file() { return; }
    std::thread::Builder::new().name("disc-prefetch".into()).spawn(move || {
        use std::io::Read;
        let Ok(mut f) = std::fs::File::open(&p) else { return };
        let mut buf = vec![0u8; 8 << 20];
        while matches!(f.read(&mut buf), Ok(n) if n > 0) {}
    }).ok();
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    // no disc given (e.g. the exe double-clicked): ask for it, and play the whole game from power-on (recomp.rs),
    // with its own title screen and memory card
    if args.is_empty() {
        let pick = rfd::FileDialog::new().set_title("Pokémon Colosseum disc image")
            .add_filter("GameCube disc image", &["gcm", "iso"]).add_filter("All files", &["*"]).pick_file();
        if let Some(p) = pick { args.push(p.to_string_lossy().into_owned()); args.push("recomp".into()); }
    }
    if args.is_empty() {
        eprintln!("usage: colo-viewer <image.gcm | dir> [recomp | room[:entry] | continue]   (room, e.g. M2_out: the recompiled game warped there; COLO_RUST=1: the Rust field; no room = continue the Rust field's save)");
        std::process::exit(2);
    }
    prefetch_disc(&args[0]);
    if args.get(1).map(String::as_str) == Some("recomp") { return recomp::run(&args[0], perf::Opts::parse(&args[2..]), None); }
    // a room (`M3_out`, `M3_out:2` for entry 2): the recompiled game, warped there from the title with the card's
    // story save (recomp.rs `warp`), so the room, its menus and scripts are the game's own code. COLO_RUST=1 keeps
    // the Rust field below (debug keys, text save, story presets).
    if let Some(r) = args.get(1).filter(|r| r.as_str() != "continue" && std::env::var("COLO_RUST").is_err()) {
        let (room, entry) = match r.split_once(':') { Some((n, e)) => (n, e.parse().unwrap_or(0)), None => (r.as_str(), 0) };
        match world::floor_for_room(&args[0], room) {
            Ok(floor) => { println!("{room}: floor {floor:#x}, entry {entry}"); return recomp::run(&args[0], perf::Opts::parse(&args[2..]), Some((floor, entry))); }
            Err(e) => { eprintln!("error: {room}: {e}"); std::process::exit(1); }
        }
    }
    // story progress: the save's flags (unless COLO_NEW_GAME=1), plus COLO_FLAGS overrides
    let saved = if std::env::var("COLO_NEW_GAME").is_ok() { None } else { save::load() };
    // nothing to continue (first run): a New Game, which starts at Outskirt Stand (story_start.rs)
    if args.len() < 2 && saved.as_ref().and_then(|s| s.room.as_ref()).is_none() { args.push("S1_out".into()); }
    let cont = args.len() < 2 || args[1] == "continue";
    // a New Game (no save, or COLO_NEW_GAME) starts with the story state of where it starts (story_start.rs)
    let mut flags = match &saved {
        Some(s) => s.flags.clone(),
        None if !cont => story_start::for_room(&args[1]).iter().copied().collect(),
        None => Default::default(),
    };
    save::apply_overrides(&mut flags);
    // hero status 0x18 (RUI in the party): the save's, else (older saves, story presets) whether her joining in
    // Phenac (M1_out func_6: std 35(1), then flag 996) has happened
    let partner = saved.as_ref().and_then(|s| s.partner).unwrap_or(flags.contains(&996));
    let room_name = if cont {
        match saved.as_ref().and_then(|s| s.room.clone()) {
            Some(r) => r,
            None => { eprintln!("no save to continue; pass a room name (e.g. M2_out)"); std::process::exit(2); }
        }
    } else { args[1].clone() };
    let spawn_entry = if cont { saved.as_ref().and_then(|s| s.entry) } else { None };
    let spawn_prev = if cont { saved.as_ref().and_then(|s| s.prev_floor) } else { None };
    let spawn_at = if cont && spawn_entry.is_none() { saved.as_ref().and_then(|s| s.pos) } else { None };
    println!("story flags set: {} (save: {})", flags.len(), save::path().map_or("-".into(), |p| p.display().to_string()));
    let mut game = match Game::open(&args[0]) {
        Ok(g) => g,
        Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
    };
    let room = match game.load_room(&room_name) {
        Ok(r) => r,
        Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
    };
    let walk_path = std::env::var("COLO_WALK").map(|v| v.split(';').filter_map(|p| {
        let n: Vec<f32> = p.split(',').filter_map(|x| x.trim().parse().ok()).collect();
        (n.len() == 2).then(|| Vec2::new(n[0], n[1]))
    }).collect()).unwrap_or_default();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "colo-viewer".into(), ..default() }),
            ..default()
        }))
        .add_plugins((flare::FlarePlugin, tevmat::TevPlugin, shadow::ShadowPlugin, particles::ParticlePlugin, wipe::WipePlugin))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource({ let mut rt = ScriptRt::new(flags); rt.w.partner = partner; rt })
        .insert_resource(tevmat::RoomLights::new(&room.lights, &room.light_anims))
        .insert_resource(Menu::default())
        .insert_resource(worldmap::Wmap::default())
        .insert_resource(ViewState { walk_path, spawn_at, spawn_entry, spawn_prev, free_cam: std::env::var("COLO_CAM").is_ok(), ..default() })
        .insert_resource(game)
        .insert_resource(room)
        .insert_resource(GcmPath(std::path::PathBuf::from(&args[0])))
        .insert_non_send_resource(GameBattleSlot(None, None))
        .add_systems(Startup, (setup_static, setup_dialogue, spawn_room, wipe::setup).chain())
        .add_systems(Update, (
            inject_keys, toggles, menu_input,
            input.run_if(menu::world_running),
            script_tick.run_if(menu::world_running), (worldmap::update, shadow::update, particles::update).chain(), actor_move.run_if(menu::world_running), actor_sync, (game_viewport, battle_floor, game_battle).chain(), (camera_follow, worldmap::zoom_sync, particles::draw, wipe::update).chain(),
            (animate.run_if(menu::world_running), animate_materials.run_if(menu::world_running), tevmat::apply_room_lights).chain(), tv::update, flare::update, do_warp, draw_collision, update_dialogue, menu::update, update_fade, update_status, test_hooks,
        ).chain())
        .run();
}

#[derive(Resource)]
struct GcmPath(std::path::PathBuf);
struct GameBattleSlot(Option<gamebattle::GameBattle>, Option<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>>);
#[derive(Component)]
struct GameScreen;
#[derive(Component)]
struct GameScreenCam;
#[derive(Resource)]
struct GameScreenImage(Handle<Image>);

/// A battle on the game's own code: at the snapshot point (`fightMainThread`'s first run) read the viewer's wipe
/// image back, boot the snapshot with it as the fade texture, run it at 30 Hz with the pad, show its frames, and
/// hand its result to the field script.
fn game_battle(mut commands: Commands, mut slot: NonSendMut<GameBattleSlot>, mut rt: ResMut<ScriptRt>, gcm: Res<GcmPath>, time: Res<Time>, keys: Res<ButtonInput<KeyCode>>,
               pads: Query<&Gamepad>, screen: Res<GameScreenImage>, mut images: ResMut<Assets<Image>>, mut vis: Query<&mut Visibility, With<GameScreen>>,
               mut cams: Query<&mut Camera, With<GameScreenCam>>, wipe: Res<wipe::Wipe>) {
    let phase = rt.w.battle.as_ref().map(|b| (b.phase, b.id, b.game));
    match phase {
        Some((battle::Phase::Battle, id, true)) if slot.0.is_none() => {
            let Some(dir) = gamebattle::snapshot_dir(id) else { return };
            match (&slot.1, wipe.current()) {
                (None, Some(img)) => {
                    let cell = std::sync::Arc::new(std::sync::Mutex::new(None));
                    let c = cell.clone();
                    commands.spawn(bevy::render::gpu_readback::Readback::texture(img)).observe(
                        move |ev: On<bevy::render::gpu_readback::ReadbackComplete>, mut commands: Commands| {
                            *c.lock().unwrap() = Some(ev.data.clone());
                            commands.entity(ev.entity).despawn();
                        });
                    slot.1 = Some(cell);
                    return;
                }
                (Some(cell), _) => {
                    let Some(data) = cell.lock().unwrap().take() else { return };
                    slot.1 = None;
                    println!("battle {id}: running on the game's code from {}", dir.display());
                    slot.0 = Some(gamebattle::GameBattle::start(dir, gcm.0.clone(), Some(data)));
                }
                (None, None) => {
                    println!("battle {id}: running on the game's code from {}", dir.display());
                    slot.0 = Some(gamebattle::GameBattle::start(dir, gcm.0.clone(), None));
                }
            }
        }
        Some((battle::Phase::Pop | battle::Phase::FadeBack | battle::Phase::Done, _, true)) | None => {
            slot.1 = None;
            if slot.0.take().is_some() {
                for mut v in &mut vis { *v = Visibility::Hidden; }
                for mut c in &mut cams { c.is_active = false; }
            }
            return;
        }
        _ => {}
    }
    let Some(gb) = slot.0.as_mut() else { return };
    gb.update(time.delta_secs(), gamebattle::pad(&keys, &pads));
    if let Some(f) = gb.frame.take() {
        if std::env::var("COLO_GB_LOG").is_ok() { let m: u64 = f.iter().map(|&x| x as u64).sum(); println!("game frame mean {}", m / f.len() as u64); }
        if let Some(mut img) = images.get_mut(&screen.0) { img.data = Some(f); }
        for mut v in &mut vis { *v = Visibility::Visible; }
        for mut c in &mut cams { c.is_active = true; }
    }
    if let Some(r) = gb.result {
        if let Some(b) = rt.w.battle.as_mut() {
            if !b.end_request && b.phase == battle::Phase::Battle { println!("battle {}: the game's result {r}", b.id); b.result = r; b.end_request = true; }
        }
    }
}

fn setup_static(mut commands: Commands, game: Res<Game>, mut assets: ModelAssets) {
    // the game draws into a 640x480 viewport (shown 4:3) with the CObj's own aspect: `game_viewport` keeps
    // the view in a height-matched 4:3 frame, `camera_follow` sets the aspect
    // UI (message layer, menus, fade overlay) draws with this camera: Bevy otherwise picks the highest-order
    // window camera, which is the wipe display camera (inactive outside a wipe)
    commands.spawn((Camera3d::default(), IsDefaultUiCamera, Projection::custom(tv::FixedAspect(PerspectiveProjection { fov: 30f32.to_radians(), near: 1.0, far: 32768.0, aspect_ratio: 4.0 / 3.0, ..default() })),
        Transform::default(), GameCam { distance: if game.test_model.is_some() { 45.0 } else { 100.0 }, height: if game.test_model.is_some() { 15.0 } else { 60.0 }, rot_y: 0.0, ready: false, fly: Vec3::ZERO, fly_yaw: 0.0, fly_pitch: -0.3 }));
    // Wes
    let (root, mut anim) = spawn_model(&mut commands, &mut assets, &game.player_model, Arc::clone(&game.player_anims));
    anim.set(1);
    commands.entity(root).insert(anim.player());
    let player = commands.spawn((Transform::default(), Visibility::Visible, Player, ActorLink { index: HERO, anim: root }, Name::new("player"))).id();
    commands.entity(player).add_child(root);
    // Rui
    let (root, mut anim) = spawn_model(&mut commands, &mut assets, &game.partner_model, Arc::clone(&game.partner_anims));
    anim.set(1);
    commands.entity(root).insert(anim);
    let partner = commands.spawn((Transform::default(), Visibility::Visible, ActorLink { index: PARTNER, anim: root }, Name::new("partner"))).id();
    commands.entity(partner).add_child(root);
    if let Some((m, a)) = &game.test_model {
        let (root, mut anim) = spawn_model(&mut commands, &mut assets, m, Arc::clone(a));
        anim.set(0);
        commands.entity(root).insert((anim, Transform::from_xyz(-22.0, 0.0, 0.0).with_rotation(Quat::from_rotation_y(std::f32::consts::PI))));
        commands.entity(player).add_child(root);
    }
    // the TV image of a battle run on the game's own code (gamebattle.rs), shown 4:3 over everything
    let screen = assets.add_image(Image::new_fill(bevy::render::render_resource::Extent3d { width: 640, height: 480, depth_or_array_layers: 1 },
        bevy::render::render_resource::TextureDimension::D2, &[0, 0, 0, 255], bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::asset::RenderAssetUsages::default()));
    // its own camera above the wipe display camera (order 5), active only while a game battle shows
    let gcam = commands.spawn((Camera2d, Camera { order: 10, is_active: false, clear_color: ClearColorConfig::None, ..default() }, GameScreenCam)).id();
    commands.spawn((Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() },
        BackgroundColor(Color::BLACK), GlobalZIndex(1000), Visibility::Hidden, GameScreen, UiTargetCamera(gcam)))
        .with_children(|p| { p.spawn((ImageNode::new(screen.clone()), Node { height: Val::Percent(100.0), aspect_ratio: Some(4.0 / 3.0), ..default() })); });
    commands.insert_resource(GameScreenImage(screen));
    commands.spawn((Text::new(""), StatusText, Node { position_type: PositionType::Absolute, top: Val::Px(8.0), left: Val::Px(10.0), ..default() }));
}

/// UI font: Poppins (OFL) plus ★ ♂ ♀ from DejaVu Sans (assets/make-font.py), until the game's own field font is decoded; Bevy's built-in font is ASCII-only.
const UI_FONT: &[u8] = include_bytes!("../assets/PoppinsColo-Medium.ttf");

fn setup_dialogue(mut commands: Commands, mut images: ResMut<Assets<Image>>, mut fonts: ResMut<Assets<Font>>, game: Res<Game>,
                  mut status: Query<&mut TextFont, With<StatusText>>) {
    let font = fonts.add(Font::from_bytes(UI_FONT.to_vec()));
    for mut f in &mut status { f.font = font.clone().into(); f.font_size = bevy::text::FontSize::Px(16.0); }
    commands.spawn((FadeOverlay, Node { position_type: PositionType::Absolute, left: Val::Px(0.0), right: Val::Px(0.0),
        top: Val::Px(0.0), bottom: Val::Px(0.0), ..default() }, BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0))));
    // menus first so the dialogue box (save messages) draws over them
    menu::spawn(&mut commands, font.clone(), |id| game.messages.get(&id).map_or_else(|| format!("<{id:#x}>"), |m| dialogue::strip_codes(m).trim().to_string()));
    worldmap::spawn_layer(&mut commands, &mut images);
    dialogue::spawn(&mut commands, &mut images);
}

/// Move the screen fade toward its target and draw it.
fn update_fade(time: Res<Time>, mut rt: ResMut<ScriptRt>, mut q: Query<&mut BackgroundColor, With<FadeOverlay>>) {
    let w = &mut rt.w;
    let step = w.fade_speed * time.delta_secs().min(0.1);
    if w.fade < w.fade_target { w.fade = (w.fade + step).min(w.fade_target); } else if w.fade > w.fade_target { w.fade = (w.fade - step).max(w.fade_target); }
    if let Ok(mut c) = q.single_mut() { c.0 = Color::srgba(0.0, 0.0, 0.0, w.fade); }
}

/// Spawn the current room (map + NPCs), set up its doors like `_floorInitialize`, start its scripts (sound
/// script, NPC move scripts and the main loop async; the modify and entry scripts sync) and place the player
/// at the entry point.
#[allow(clippy::too_many_arguments)]
fn spawn_room(mut commands: Commands, mut room: ResMut<RoomData>, game: Res<Game>, mut rt: ResMut<ScriptRt>, mut assets: ModelAssets,
              mut player: Query<&mut Transform, With<Player>>, mut cam: Query<&mut GameCam>, mut state: ResMut<ViewState>) {
    // fresh VM and world for the room; flags carry over
    let flags = std::mem::take(&mut rt.w.flags);
    let partner = rt.w.partner;
    let prev_floor = state.spawn_prev.take().unwrap_or(rt.w.floor_id);
    *rt = ScriptRt::new(flags);
    // heroMoveSyncWithHero: member 1 = hero status 0x18 unless flag 0x8AE; initFloor shows RUI only as a member
    rt.w.partner = partner;
    rt.w.partner_member = partner && !rt.w.flags.contains(&0x8AE);
    rt.w.prev_floor = prev_floor;
    rt.vm.load(game.common_script.clone());
    rt.vm.load(room.room_script.clone());
    rt.w.floor_id = room.floor_id;
    rt.w.doors = game.doors.iter().map(|d| (d.open_flag, d.close_flag)).collect();
    rt.w.group_id = room.group_id;
    rt.w.char_disp.clear();
    // global strings, with the room's own tables on top
    rt.w.messages = game.messages.clone();
    rt.w.messages.extend(room.messages.iter().map(|(&k, v)| (k, v.clone())));
    rt.w.cam_ends = room.cam_files.iter().map(|(&id, c)| (id, c.anims.first().map_or(0.0, |a| a.end_frame))).collect();

    let root = commands.spawn((Transform::default(), Visibility::default(), RoomRoot, Name::new(room.name.clone()))).id();
    let (map, mut map_anim) = spawn_model(&mut commands, &mut assets, &room.map, Arc::clone(&room.map_anims));

    // floorInitMap / floorOpenObject: every map model set starts its animation 0 and texture animation 0 at
    // rate 0.5, looping (door sets are then put in place by _floorInitialize below). In a few rooms the
    // texture animations of animated sets are unlinked and run at 0.35.
    for set in map_anim.model_sets() { map_anim.play_set(set, 0, 0.0, MAP_ANIM_RATE, true); }
    if MAP_TEX_UNLINKED_ROOMS.contains(&room.floor_id) {
        map_anim.linked = false;
        let joint_sets: Vec<usize> = map_anim.anims.iter().map(|a| a.model_set).collect();
        for p in map_anim.tex_plays.iter_mut() {
            if joint_sets.contains(&map_anim.mats.anims[p.anim].model_set) { p.rate = 0.35; }
        }
    }
    // FUN_80117E58 at floor setup: TV channel 0, if the room has TVs
    rt.w.tv_list = room.tv.clone();
    rt.w.shadow_flags = Arc::clone(&game.shadow_flags);
    rt.w.battle_tables = Some(Arc::clone(&game.battle));
    rt.w.tv = None;
    if room.tv.iter().any(|c| c.id == 0) { rt.w.tv = Some(0); }
    // _floorInitialize: doors of types 1/3 come back opened (mode 1) if their open flag is set, else closed (3);
    // type 2 always closed
    let init: Vec<Cmd> = room.doors.iter().map(|d| {
        let open = matches!(d.door_type, 1 | 3) && d.open_flag != 0 && rt.w.flags.contains(&(d.open_flag as u32));
        Cmd::Door { index: d.index, mode: if open { 1 } else { 3 } }
    }).collect();
    apply(&init, &game, &mut room, &mut map_anim, &mut rt.w.flags);
    rt.w.colsys = Some(room.colsys.clone());

    // characters: the hero, then the room's NPCs
    let entry = state.pending_warp.map(|w| w.1).or(state.spawn_entry.take()).unwrap_or(0);
    let (pos, yaw) = state.spawn_at.take().unwrap_or_else(|| room.entry(entry));
    let hero_slots = game.player_info.as_ref().map_or(HERO_SLOTS, |m| m.anim_slots);
    let partner_slots = game.partner_info.as_ref().map_or(HERO_SLOTS, |m| m.anim_slots);
    let mut hero = Actor::new(HERO, pos, yaw, true, hero_slots, 0);
    let strides = |m: &Option<colo_formats::common::CharacterModel>| m.as_ref().map_or((9.0, 13.0), |m| (m.walk_stride, m.run_stride));
    hero.set_speeds(strides(&game.player_info), &game.player_anims);
    if let Some(m) = &game.player_info { hero.set_body(m); }
    hero.flags = heromove::HERO_FLAGS;
    hero.amount = Some(0.0);
    rt.w.actors.push(hero);
    // initFloor: Rui starts 9 behind Wes, facing the same way
    let behind = heromove::place_partner(&room.colsys, pos, yaw);
    let mut rui = Actor::new(PARTNER, behind, yaw, rt.w.partner_member && std::env::var("COLO_NO_RUI").is_err(), partner_slots, 0);
    rui.set_speeds(strides(&game.partner_info), &game.partner_anims);
    if let Some(m) = &game.partner_info { rui.set_body(m); }
    rui.flags = heromove::PARTNER_FLAGS;
    rui.amount = Some(0.0);
    rui.teleported = true;
    rt.w.actors.push(rui);
    for (c, model) in room.characters.iter().zip(&room.character_models) {
        let slots = room.npc_anim_slots.get(c.index).copied().unwrap_or([1, 2, 3, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        let p = Vec3::from_array(c.position);
        let mut actor = Actor::new(c.index as u32, p, (c.rotation_y as f32).to_radians(), c.visible, slots, c.move_type);
        // peopleOpenSub writes flags 0x700 (walls, floor, human collision); _floorInitCharacters adds 4 and 8
        actor.flags = 0x700 | 4 | 8;
        if let Some(loaded) = model { actor.set_speeds(room.npc_strides.get(c.index).copied().unwrap_or((9.0, 13.0)), &loaded.1); }
        if let Some(Some(m)) = room.npc_info.get(c.index) { actor.set_body(m); }
        rt.w.actors.push(actor);
        let Some(loaded) = model else { continue };
        let (m, anims) = (&loaded.0, &loaded.1);
        let (r, mut anim) = spawn_model(&mut commands, &mut assets, m, Arc::clone(anims));
        let idle = if slots[0] == 0xFF { 0 } else { slots[0] as usize };
        anim.set(idle);
        commands.entity(r).insert(anim);
        let npc = commands.spawn((
            Transform::from_translation(p).with_rotation(Quat::from_rotation_y((c.rotation_y as f32).to_radians())),
            if c.visible { Visibility::Visible } else { Visibility::Hidden }, ActorLink { index: c.index as u32, anim: r },
        )).id();
        commands.entity(npc).add_child(r);
        commands.entity(root).add_child(npc);
    }

    // scripts: sound script (+0x28) and NPC move scripts run as threads; the modify script (+0x2C) runs to
    // completion now
    let group = room.group_id;
    {
        let (vm, mut host) = rt.host();
        if room.scripts[0] != 0 { vm.spawn(room.scripts[0], &[], &mut host); }
        for c in &room.characters {
            if c.move_script != 0 { vm.spawn(c.move_script, &[group, c.index as u32, c.people_index as u32, 0], &mut host); }
        }
        if room.scripts[1] != 0 { vm.call_sync(room.scripts[1], &[], &mut host, 600); }
        // the modify script runs at script load, before _floorInitCharacters reads each NPC record's visible bit
        for (i, on) in std::mem::take(&mut host.0.char_disp) { if let Some(a) = host.0.actor(i) { a.visible = on; } }
        // +0x30: entry script (story-state setup: visibility, flags), run to completion
        if room.scripts[2] != 0 { vm.call_sync(room.scripts[2], &[], &mut host, 600); }
        // +0x34: the room's main loop (checks story flags every frame and starts events)
        if room.scripts[3] != 0 && std::env::var("COLO_NO_EVENTS").is_err() { vm.spawn(room.scripts[3], &[], &mut host); }
    }
    let cmds = std::mem::take(&mut rt.w.cmds);
    apply(&cmds, &game, &mut room, &mut map_anim, &mut rt.w.flags);
    rt.w.colsys = Some(room.colsys.clone());
    commands.entity(map).insert((map_anim, MapModel));
    commands.entity(root).add_child(map);

    state.pending_warp = None;
    if let Ok(mut t) = player.single_mut() {
        t.translation = pos;
        t.rotation = Quat::from_rotation_y(yaw);
        state.ignore_region = room.collision.region_at(pos);
    }
    if let Ok(mut gc) = cam.single_mut() {
        gc.ready = false; // snap to the new room's camera
    }
    println!("entered {} (floor {:#x}) at {pos:?}", room.name, room.floor_id);
    if std::env::var("COLO_REGIONS").is_ok() {
        for (kind, label) in [(collide::Kind::Region, "region"), (collide::Kind::Check, "check ")] {
            for (r, c) in room.collision.region_centers(kind) {
                let ips: Vec<String> = game.interaction_points.iter().filter(|p| p.room_id as u32 == room.floor_id && p.region == r as u32)
                    .map(|p| format!("#{} method {} script {:#x} {:?}", p.index, p.method, p.script, p.params)).collect();
                println!("  {label} {r:3} at {:7.1} {:6.1} {:7.1}  {}", c.x, c.y, c.z, ips.join("; "));
            }
        }
        for c in &room.characters {
            println!("  npc {:2} at {:7.1} {:6.1} {:7.1}  people {}", c.index, c.position[0], c.position[1], c.position[2], c.people_index);
        }
    }
}

/// The battle floor pushed over the field (`floorWorker` state 4) and popped back (state 6), for std 102.
#[derive(Component)]
struct BattleRoot;

#[allow(clippy::too_many_arguments)]
fn battle_floor(mut commands: Commands, mut rt: ResMut<ScriptRt>, mut game: ResMut<Game>, room: Res<RoomData>, mut assets: ModelAssets,
                mut roots: Query<&mut Visibility, With<RoomRoot>>, broots: Query<Entity, With<BattleRoot>>) {
    let Some((phase, name, floor_id)) = rt.w.battle.as_ref().map(|b| (b.phase, b.room.clone(), b.floor_id)) else { return };
    match phase {
        battle::Phase::Push => {
            let loaded = name.as_deref().map(|n| game.load_room(n));
            for mut v in &mut roots { *v = Visibility::Hidden; }
            match loaded {
                Some(Ok(r)) => {
                    let root = commands.spawn((Transform::default(), Visibility::default(), BattleRoot, Name::new(r.name.clone()))).id();
                    let (map, mut anim) = spawn_model(&mut commands, &mut assets, &r.map, Arc::clone(&r.map_anims));
                    for set in anim.model_sets() { anim.play_set(set, 0, 0.0, MAP_ANIM_RATE, true); }
                    commands.entity(map).insert(anim);
                    commands.entity(root).add_child(map);
                    commands.insert_resource(tevmat::RoomLights::new(&r.lights, &r.light_anims));
                    println!("battle floor {} pushed", r.name);
                }
                Some(Err(e)) => println!("battle floor {}: {e}", name.unwrap_or_default()),
                None => println!("battle floor {floor_id:#x}: no room archive"),
            }
            if let Some(b) = rt.w.battle.as_mut() { b.pushed(); }
        }
        battle::Phase::Pop => {
            for e in &broots { commands.entity(e).despawn(); }
            for mut v in &mut roots { *v = Visibility::Inherited; }
            commands.insert_resource(tevmat::RoomLights::new(&room.lights, &room.light_anims));
            rt.w.battle_cam = None;
            let mut b = rt.w.battle.take();
            if let Some(b) = b.as_mut() { b.popped(&mut rt.w); }
            rt.w.battle = b;
            println!("battle floor popped");
        }
        _ => {}
    }
}

/// Message window input: A advances (or completes the page), up/down move the YES/NO cursor.
fn input(keys: Res<ButtonInput<KeyCode>>, pads: Query<&Gamepad>, mut rt: ResMut<ScriptRt>, mut state: ResMut<ViewState>) {
    let rt = &mut *rt;
    // an A that went to the message window must not also reach heroMoveMain this frame
    state.a_used = rt.w.dialog.visible();
    if !rt.w.dialog.visible() { return; }
    let b = menu::buttons(&keys, &pads);
    if let Some(c) = rt.w.dialog.menu.as_mut() {
        let was = *c;
        if b.up { *c = 0; } else if b.down { *c = 1; }
        if *c != was { audio::se(audio::SE_CURSOR); }
    }
    if b.a { rt.w.dialog.press(); }
}

/// Run interaction events like `fn_80116D30(method, region)`: every interaction point of this room with that
/// method and region id.
fn dispatch_event(rt: &mut ScriptRt, game: &Game, room: &RoomData, method: u8, id: u16) {
    let kind = match method { 1 => "pass", 2 => "touch", 3 => "check", _ => "menu" };
    for ip in game.interaction_points.iter().filter(|p| p.room_id as u32 == room.floor_id && p.method == method && p.region == id as u32) {
        println!("{kind} event {id}: interaction point #{} (script {:#x})", ip.index, ip.script);
        rt.ip_queue.push_back((ip.script, ip.params));
    }
}

/// `updateChat`: talk to NPC `idx`. Both members stand idle and locked (`heroMoveInitEvent`), the hero
/// turns to the NPC (std 164 at rate 1: locked, so π/20 per frame; the NPC's body doesn't turn, its talk
/// script may turn it or its head), the NPC's wandering pauses (`fn_801812E8`) and the NPC's talk script runs with (group, index, 0, 0); when it ends,
/// `characterEndSpeech` and `heroMoveTermEvent` follow (see `script_tick`).
fn begin_talk(rt: &mut ScriptRt, room: &RoomData, idx: u32) {
    // Rui (group 0) can be the target too: no floor character, so no talk script (`fn_8018397C` → 0)
    let script = if idx == PARTNER { 0 } else { room.characters.get(idx as usize).map_or(0, |c| c.talk_script) };
    // `peopleGetHeadGroundPos`: the head part's x/z at the people's y
    let npc_pos = rt.w.actors.iter().find(|a| a.index == idx)
        .map(|a| match a.head_pos.filter(|_| a.head_part >= 0) { Some(q) => Vec3::new(q.x, a.pos.y, q.z), None => a.pos })
        .unwrap_or_default();
    for a in rt.w.actors.iter_mut().filter(|a| a.index == HERO || a.index == PARTNER) {
        a.amount = Some(0.0);
        a.flags = (a.flags | 0x8000_0008) & !(0x100 | 0x400);
        a.human_on = false;
    }
    if let Some(h) = rt.w.actor(HERO) { let d = npc_pos - h.pos; h.start_turn(d.x.atan2(d.z), 1.0); }
    if script == 0 {
        // nothing to run: heroMoveTermEvent's unlock follows in the same frame
        println!("talk to {idx}: no talk script");
        for a in rt.w.actors.iter_mut().filter(|a| a.index == HERO || a.index == PARTNER) {
            a.human_on = true;
            a.flags = (a.flags | 0x700) & !0x8000_0008;
        }
        return;
    }
    if let Some(a) = rt.w.actor(idx) {
        a.talking = true;
        a.path.clear();
    }
    println!("talk to NPC {idx} (script {:#x})", script);
    rt.talk = Some(idx);
    // updateChat runs the talk script synchronously and ends the talk at once when it doesn't block, before the
    // room's main loop goes on (a main-loop message opened next must not be closed by this talk's end)
    if !rt.run_event_now(script, &[room.group_id, idx, 0, 0]) { end_talk(rt); }
}

/// The end of a talk started by heroMoveMain: `characterEndSpeech` + `heroMoveTermEvent`.
fn end_talk(rt: &mut ScriptRt) {
    let npc = rt.talk.take().unwrap_or(0);
    rt.w.dialog.close();
    if let Some(a) = rt.w.actor(npc) { a.talking = false; }
    for a in rt.w.actors.iter_mut().filter(|a| a.index == HERO || a.index == PARTNER) {
        a.human_on = true;
        a.flags = (a.flags | 0x700) & !0x8000_0008;
    }
}

/// The controller as the game reads it: main stick (WASD = full tilt, Shift = half; gamepad stick),
/// d-pad (arrow keys), A, and X/Y/Start for the menu.
fn read_pad(keys: &ButtonInput<KeyCode>, pads: &Query<&Gamepad>, state: &ViewState) -> heromove::Pad {
    let mut stick = Vec2::ZERO;
    if !state.free_cam {
        if keys.pressed(KeyCode::KeyW) { stick.y += 1.0; }
        if keys.pressed(KeyCode::KeyS) { stick.y -= 1.0; }
        if keys.pressed(KeyCode::KeyD) { stick.x += 1.0; }
        if keys.pressed(KeyCode::KeyA) { stick.x -= 1.0; }
        stick = stick.normalize_or_zero() * if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) { 28.0 } else { 100.0 };
    }
    let mut dpad = IVec2::ZERO;
    if keys.pressed(KeyCode::ArrowUp) { dpad.y += 1; }
    if keys.pressed(KeyCode::ArrowDown) { dpad.y -= 1; }
    if keys.pressed(KeyCode::ArrowRight) { dpad.x += 1; }
    if keys.pressed(KeyCode::ArrowLeft) { dpad.x -= 1; }
    let b = menu::buttons(keys, pads);
    for pad in pads {
        let v = pad.left_stick();
        if v.length() > 0.15 { stick = v * 100.0; }
    }
    heromove::Pad { stick, dpad, a: b.a && !state.a_used, menu: b.menu }
}

/// Run the room's VM at the game's 60 Hz, run `heroMoveMain` on the frames the main loop asked for it
/// (std 31), and apply what the scripts changed.
#[allow(clippy::too_many_arguments)]
fn script_tick(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, pads: Query<&Gamepad>, mut rt: ResMut<ScriptRt>, mut room: ResMut<RoomData>,
               game: Res<Game>, mut state: ResMut<ViewState>, mut menu: ResMut<Menu>, cams: Query<(&Camera, &GlobalTransform, &GameCam)>,
               mut map: Query<&mut Animator, With<MapModel>>) {
    let rt = &mut *rt;
    let mut pad = read_pad(&keys, &pads, &state);
    let cam = cams.single().ok();
    let cam_rot_y = cam.map_or(0.0, |c| c.2.rot_y);
    // COLO_WALK: half tilt toward the next waypoint (full tilt with COLO_RUN)
    if let Some(target) = state.walk_path.front().copied() {
        let hp = rt.w.hero_pos();
        let d = Vec3::new(target.x - hp.x, 0.0, target.y - hp.z);
        if d.length() < 4.0 { state.walk_path.pop_front(); } else {
            let n = d.normalize();
            let (s, c) = cam_rot_y.sin_cos();
            let (fwd, right) = (Vec3::new(-s, 0.0, -c), Vec3::new(c, 0.0, -s));
            pad.stick = Vec2::new(n.dot(right), n.dot(fwd)) * if std::env::var("COLO_RUN").is_ok() { 100.0 } else { 28.0 };
        }
    }
    if std::mem::take(&mut state.force_a) { pad.a = true; }
    // no battle logic yet: A ends a battle once its floor is up
    if let Some(b) = rt.w.battle.as_mut() {
        if pad.a && b.phase == battle::Phase::Battle && !b.game { b.end_request = true; }
    }
    let on_screen = |p: Vec3, r: f32| -> bool {
        let Some((camera, gt, _)) = cam else { return true };
        let aspect = camera.logical_viewport_size().map_or(4.0 / 3.0, |v| v.x / v.y.max(1.0));
        // the game's screen is 4:3: only the middle of a wider view counts
        let xmax = (4.0 / 3.0) / aspect;
        [(-r, 0.0, -r), (r, 0.0, -r), (-r, 0.0, r), (r, 0.0, r), (-r, 17.0, -r), (r, 17.0, -r), (-r, 17.0, r), (r, 17.0, r)].iter().any(|&(x, y, z)| {
            camera.world_to_ndc(gt, p + Vec3::new(x, y, z)).is_some_and(|n| n.x.abs() <= xmax && n.y.abs() <= 1.0 && n.z > 0.0 && n.z <= 1.0)
        })
    };
    let no_main_loop = room.scripts[3] == 0 || std::env::var("COLO_NO_EVENTS").is_ok();
    // the first update after a room load runs one frame instead of catching up the load time
    rt.acc = if rt.acc < 0.0 { 1.0 } else { (rt.acc + time.delta_secs() * 60.0).min(4.0) };
    while rt.acc >= 1.0 {
        rt.acc -= 1.0;
        rt.tick();
        if no_main_loop && !rt.event_running() && !menu.is_open() { rt.w.hm.process = true; }
        // a talk ended: characterEndSpeech + heroMoveTermEvent
        if rt.talk.is_some() && !rt.event_running() { end_talk(rt); }
        if std::mem::take(&mut rt.w.hm.process) && !menu.is_open() && rt.w.battle.is_none() {
            let talk_info = |i: u32| room.characters.get(i as usize).map(|c| (c.talk_through_walls, c.talk_start_type));
            let partner_active = rt.w.partner_member && std::env::var("COLO_NO_RUI").is_err();
            let ctx = heromove::Ctx { cs: &room.colsys, pad, cam_rot_y, on_screen: &on_screen, talk_info: &talk_info, partner_active };
            if let Some(events) = heromove::hero_begin(&mut rt.w.hm, &rt.w.actors) {
                // events run synchronously (fn_800F7434); if one blocks, the main loop waits for it
                for (m, id) in events { dispatch_event(rt, &game, &room, m, id); }
                while !rt.event_running() {
                    let Some((script, params)) = rt.ip_queue.pop_front() else { break };
                    rt.run_event_now(script, &params);
                }
                if !rt.event_running() {
                    let req = heromove::hero_move(&mut rt.w.hm, &mut rt.w.actors, &ctx);
                    pad.a = false;
                    pad.menu = false;
                    if req.open_menu { *menu = Menu::Field { cursor: 0, notice: false }; }
                    if let Some(npc) = req.talk { begin_talk(rt, &room, npc); }
                }
            }
        }
        if !rt.event_running() { for a in rt.w.actors.iter_mut() { if a.index != HERO && a.index != PARTNER { a.talking = false; } } }
        rt.w.people_frame();
    }
    // battleStart (fn_80112700, floorStartSoundScript): the field's sound script (+0x28) starts again as a thread,
    // and its std104 restarts the field BGM
    if rt.w.battle.as_ref().is_some_and(|b| b.rerun_modify) {
        if let Some(b) = rt.w.battle.as_mut() { b.rerun_modify = false; }
        if room.scripts[0] != 0 {
            let (vm, mut host) = rt.host();
            vm.spawn(room.scripts[0], &[], &mut host);
        }
    }
    let Ok(mut map_anim) = map.single_mut() else { return };
    if !rt.w.cmds.is_empty() {
        let cmds = std::mem::take(&mut rt.w.cmds);
        println!("script: {cmds:?}");
        apply(&cmds, &game, &mut room, &mut map_anim, &mut rt.w.flags);
        rt.w.colsys = Some(room.colsys.clone());
    }
    rt.w.map_busy = map_anim.busy();
    if let Some(w) = rt.w.warp.take() {
        println!("warp -> floor {:#x} entry {}", w.0, w.1);
        state.pending_warp = Some(w);
    }
}

/// Move characters along their script paths (and let wanderers wander), on the room collision.
fn actor_move(time: Res<Time>, room: Res<RoomData>, mut rt: ResMut<ScriptRt>) {
    let dt = time.delta_secs().min(0.1);
    let rt = &mut *rt;
    let locked = rt.event_running() || rt.w.dialog.visible();
    let mut rolls: Vec<u32> = (0..rt.w.actors.len() * 2).map(|_| rt.w.random()).collect();
    let humans = heromove::humans(&rt.w.actors);
    for a in rt.w.actors.iter_mut() {
        // wandering (move type 2): stroll to a random spot near home, pause, repeat
        if a.move_type == 2 && a.path.is_empty() && !a.talking && !locked && a.visible {
            a.wander_wait -= dt;
            if a.wander_wait <= 0.0 {
                let (r1, r2) = (rolls.pop().unwrap_or(1), rolls.pop().unwrap_or(2));
                let ang = (r1 % 6283) as f32 / 1000.0;
                let dist = 8.0 + (r2 % 1700) as f32 / 100.0;
                a.path.push_back(a.home + Vec3::new(ang.sin() * dist, 0.0, ang.cos() * dist));
                a.run = false;
                a.stuck = 0.0;
                a.wander_wait = 2.0 + (r2 % 300) as f32 / 100.0;
            }
        }
        if let Some(&goal) = a.path.front() {
            let to = Vec3::new(goal.x - a.pos.x, 0.0, goal.z - a.pos.z);
            // scripted moves use moveSpeed 1.0 (walk) or the run speed
            let speed = if a.run { a.run_speed } else { a.walk_speed } * 60.0;
            let step = speed * dt;
            // each step goes through `peopleMove` (fn_8018E9B4) by the people flags: NPCs start with 0x700
            // (walls, floor, human); std 76 (characterEventFlagsClear) drops walls and human collision but the
            // floor still carries them (S1_out: WILLIE down the stand's stairs past the railing). Without the floor
            // flag the height follows the target's
            if to.length() <= step.max(0.5) {
                a.pos = heromove::people_move(&room.colsys, &humans, a, a.pos, goal).pos;
                a.path.pop_front();
            } else {
                let dir = to.normalize();
                let mut target = a.pos + dir * step;
                target.y += (goal.y - a.pos.y) * (step / to.length()).min(1.0);
                let next = heromove::people_move(&room.colsys, &humans, a, a.pos, target).pos;
                if next.distance(a.pos) < step * 0.2 {
                    a.stuck += dt;
                    if a.stuck > 1.0 { a.path.pop_front(); a.stuck = 0.0; } // blocked: give up this step
                } else {
                    a.stuck = 0.0;
                }
                a.pos = next;
                a.yaw = dir.x.atan2(dir.z);
                a.turn_to = None;
            }
        }
        // turns run per frame in `people_frame`
    }
}

/// Copy actor state to the ECS: transforms, visibility, and the animation each character should play.
#[allow(clippy::type_complexity)]
fn actor_sync(mut commands: Commands, mut rt: ResMut<ScriptRt>,
              mut links: Query<(Entity, &ActorLink, &mut Transform, &mut Visibility, Option<&ChildOf>, Option<&Rider>)>,
              mut anims: Query<&mut Animator, Without<MapModel>>, globals: Query<&GlobalTransform>) {
    // the battle floor hides the field's characters (their Visibility::Visible ignores the hidden room root)
    let in_battle = rt.w.battle.as_ref().is_some_and(|b| b.pushed);
    // std 70 riders: GSmodelAttachToGSpart(model, part, 7, 0, 1) makes the part's joint the model's parent
    // (translation, rotation and scale inherited); the people position/rotation are then local to it
    let models: Vec<(u32, Entity)> = links.iter().map(|l| (l.1.index, l.1.anim)).collect();
    for (e, link, _, _, parent, rider) in links.iter() {
        let Some(a) = rt.w.actors.iter().find(|a| a.index == link.index) else { continue };
        let want = a.attach.and_then(|(to, part)| {
            let m = models.iter().find(|m| m.0 == to)?;
            anims.get(m.1).ok()?.joints.get(part).copied()
        });
        let now = parent.map(|p| p.parent());
        match (want, rider) {
            (Some(j), _) if now != Some(j) => {
                let home = rider.map_or(now, |r| r.0);
                commands.entity(e).insert((ChildOf(j), Rider(home)));
            }
            (None, Some(r)) => {
                match r.0 { Some(h) => { commands.entity(e).insert(ChildOf(h)); } None => { commands.entity(e).remove::<ChildOf>(); } }
                commands.entity(e).remove::<Rider>();
            }
            _ => {}
        }
    }
    for (_, link, mut t, mut vis, ..) in links.iter_mut() {
        let Some(a) = rt.w.actor(link.index) else { continue };
        t.translation = a.pos;
        a.teleported = false;
        // the body's rotation is the people rotation as `fn_80184A90` turned it (no extra smoothing)
        t.rotation = Quat::from_rotation_y(a.yaw);
        *vis = if a.visible && !in_battle { Visibility::Visible } else { Visibility::Hidden };
        let Ok(mut an) = anims.get_mut(link.anim) else { continue };
        // head look: GSpartRegisterRotation(head, (pitch, yaw, 0), 3) while a look is set; the head's world
        // position (last frame's) feeds the next look update
        let head = (a.head_part >= 0).then_some(a.head_part as usize).filter(|&h| h < an.joints.len());
        a.head_pos = head.and_then(|h| globals.get(an.joints[h]).ok()).map(|g| g.translation());
        an.part_rot = head.filter(|_| a.look.is_some()).map(|h| (h, [a.head_rot.0, a.head_rot.1, 0.0]));
        if let Some((anim, looping)) = a.anim_request.take() {
            if looping { an.set(anim); } else { an.set_once(anim); }
        } else if let (Some(amount), true, false) = (a.amount, a.path.is_empty(), an.busy()) {
            // hero / partner: updateAnimation's walk/run blend
            let (anim, blend, rate) = heromove::anim_for(amount, &a.slots);
            an.set_blend(anim, blend, rate);
        } else if !an.busy() {
            let moving = !a.path.is_empty() || a.follow.is_some();
            let run = a.follow.unwrap_or(a.run);
            let want = if moving { if run { a.run_anim() } else { a.walk_anim() } } else { a.base_anim };
            // a finished one-shot (std 51 with loop 0 = GSmodel anim type 0) holds its last frame: std 53 only waits
            // for GSmodelHasAnimationEnded and nothing restarts the idle, so scripts set the next loop themselves
            // (the S1_out truck stays where its drive-off animation left it until the script hides it)
            let held = !moving && an.active.iter().any(|p| !p.looping);
            if !held && an.current() != Some(want) { an.set(want); }
        }
        a.anim_busy = an.busy();
    }
}

/// `floorUpdateFieldCamera` (0x80117514): blend the room's camera points by inverse squared distance from the
/// camera target; a point closer than 1 unit wins outright, a single point is used as is.
fn field_camera(points: &[colo_formats::maprel::CameraPoint], target: Vec3) -> Option<(f32, f32, f32)> {
    if points.is_empty() { return None; }
    if points.len() == 1 { return Some((points[0].distance, points[0].height, points[0].rot_y)); }
    let (mut total, mut d, mut h, mut r) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
    for p in points {
        let dist2 = Vec3::from_array(p.position).distance_squared(target);
        if dist2 <= 1.0 { return Some((p.distance, p.height, p.rot_y)); }
        let w = 0.001 / dist2;
        total += w;
        d += p.distance * w;
        h += p.height * w;
        r += p.rot_y * w;
    }
    (total > 0.0).then(|| (d / total, h / total, r / total))
}

#[allow(clippy::too_many_arguments)]
fn camera_follow(time: Res<Time>, mouse: Res<ButtonInput<MouseButton>>, motion: Res<AccumulatedMouseMotion>,
                 keys: Res<ButtonInput<KeyCode>>, state: Res<ViewState>, room: Res<RoomData>, player: Query<&Transform, With<Player>>,
                 mut cam: Query<(&mut Transform, &mut GameCam, &mut Projection), Without<Player>>, mut rt: ResMut<ScriptRt>) {
    let Ok((mut ct, mut gc, mut proj)) = cam.single_mut() else { return };
    let set_fov = |proj: &mut Projection, deg: f32, (near, far, aspect): (f32, f32, f32)| if let Some(tv::FixedAspect(p)) = tv::fixed_aspect_mut(proj) {
        p.fov = deg.clamp(3.0, 120.0).to_radians();
        (p.near, p.far, p.aspect_ratio) = (near, far, aspect);
    };
    // a pushed battle floor: its camera
    if let Some((eye, at, fov, near, far, aspect)) = rt.w.battle_cam {
        *ct = Transform::from_translation(eye).looking_at(at, Vec3::Y);
        set_fov(&mut proj, fov, (near, far, aspect));
        gc.ready = false;
        return;
    }
    // an animated event camera (.cam, mode 4) plays at rate 0.5 and holds its last frame until the script
    // returns the camera (cameraReturn / GSscene_SetMode(0))
    if !state.free_cam {
        if let Some(ac) = rt.w.anim_cam.as_mut() {
            if ac.playing { ac.frame = (ac.frame + time.delta_secs().min(0.1) * 60.0 * 0.5).min(ac.end_frame); }
            let ac = *ac;
            if let Some(cam) = room.cam_files.get(&ac.resource) {
                let p = cam.pose(0, ac.frame);
                let (eye, at) = (Vec3::from_array(p.eye), Vec3::from_array(p.interest));
                let mut t = Transform::from_translation(eye).looking_at(at, Vec3::Y);
                t.rotate_local_z(-p.roll);
                *ct = t;
                set_fov(&mut proj, p.fov, (p.near, p.far, if p.aspect > 0.0 { p.aspect } else { 4.0 / 3.0 }));
                gc.ready = false;
                return;
            }
        }
    }
    set_fov(&mut proj, 30.0, room.clip);
    // report where the camera is (scripts start timed moves from here)
    let (yaw, pitch, roll) = ct.rotation.to_euler(EulerRot::YXZ);
    rt.w.cam_now = (ct.translation, Vec3::new(pitch, yaw, roll));
    // a script-driven event camera wins over the field camera
    if !state.free_cam {
        if let Some(ec) = rt.w.event_cam.as_mut() {
            ec.t += time.delta_secs();
            let k = if ec.dur > 0.0 { (ec.t / ec.dur).min(1.0) } else { 1.0 };
            let pos = ec.from_pos.lerp(ec.pos, k);
            let rot = ec.from_rot.lerp(ec.rot, k);
            *ct = Transform::from_translation(pos).with_rotation(Quat::from_euler(EulerRot::YXZ, rot.y, rot.x, rot.z));
            gc.ready = false; // snap back to the field camera afterwards
            return;
        }
    }
    if state.free_cam {
        if mouse.pressed(MouseButton::Right) {
            gc.fly_yaw -= motion.delta.x * 0.004;
            gc.fly_pitch = (gc.fly_pitch - motion.delta.y * 0.004).clamp(-1.5, 1.5);
        }
        let rot = Quat::from_euler(EulerRot::YXZ, gc.fly_yaw, gc.fly_pitch, 0.0);
        let mut v = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) { v += rot * -Vec3::Z; }
        if keys.pressed(KeyCode::KeyS) { v -= rot * -Vec3::Z; }
        if keys.pressed(KeyCode::KeyD) { v += rot * Vec3::X; }
        if keys.pressed(KeyCode::KeyA) { v -= rot * Vec3::X; }
        if keys.pressed(KeyCode::KeyE) { v += Vec3::Y; }
        if keys.pressed(KeyCode::KeyQ) { v -= Vec3::Y; }
        let speed = if keys.pressed(KeyCode::ShiftLeft) { 400.0 } else { 100.0 };
        gc.fly += v.normalize_or_zero() * speed * time.delta_secs();
        *ct = Transform::from_translation(gc.fly).with_rotation(rot);
        return;
    }
    let Ok(pt) = player.single() else { return };
    let look = pt.translation + Vec3::Y * VIEW_HEIGHT;
    // rooms without camera points use the map's scene camera
    let (d, h, r) = field_camera(&room.cameras, look)
        .or(room.default_camera.map(|c| (c.0, c.1, c.2)))
        .unwrap_or((66.0, 80.0, 0.0));
    if gc.ready {
        // the game keeps 25% of the previous value each frame
        let k = 0.25f32.powf(time.delta_secs().min(0.1) * 60.0);
        gc.distance = gc.distance * k + d * (1.0 - k);
        gc.height = gc.height * k + h * (1.0 - k);
        gc.rot_y = gc.rot_y * k + r * (1.0 - k);
    } else {
        (gc.distance, gc.height, gc.rot_y, gc.ready) = (d, h, r, true);
    }
    let eye = pt.translation + Quat::from_rotation_y(gc.rot_y) * Vec3::new(0.0, gc.height, gc.distance);
    *ct = Transform::from_translation(eye).looking_at(look, Vec3::Y);
    // free-fly starts from here
    gc.fly = eye;
    let (yaw, pitch, _) = ct.rotation.to_euler(EulerRot::YXZ);
    (gc.fly_yaw, gc.fly_pitch) = (yaw, pitch);
}

/// Keep the 3D view in the window's height-matched 4:3 centre, the game's 640x480 screen (the 2D layer uses the same frame).
fn game_viewport(windows: Query<&Window, With<bevy::window::PrimaryWindow>>, mut cam: Query<&mut Camera, With<GameCam>>) {
    let (Ok(win), Ok(mut cam)) = (windows.single(), cam.single_mut()) else { return };
    let full = win.physical_size();
    if full.x == 0 || full.y == 0 { return; }
    let size = if full.x * 3 >= full.y * 4 { UVec2::new((full.y * 4).div_ceil(3).min(full.x), full.y) } else { UVec2::new(full.x, full.x * 3 / 4) };
    let vp = bevy::camera::Viewport { physical_position: (full - size) / 2, physical_size: size, ..default() };
    if cam.viewport.as_ref().is_none_or(|v| v.physical_position != vp.physical_position || v.physical_size != vp.physical_size) { cam.viewport = Some(vp); }
}

fn toggles(keys: Res<ButtonInput<KeyCode>>, menu: Res<Menu>, wmap: Res<worldmap::Wmap>, mut state: ResMut<ViewState>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyC) { state.show_collision = !state.show_collision; }
    if keys.just_pressed(KeyCode::Tab) && !menu.is_open() { state.free_cam = !state.free_cam; }
    // Esc quits (nothing is saved: saving is the PC's job); inside a menu it backs out instead
    if keys.just_pressed(KeyCode::Escape) && !menu.is_open() && !wmap.active() { exit.write(AppExit::Success); }
}

/// The field menu (X / Start) and the PC menu with its save flow; see `menu.rs`.
#[allow(clippy::too_many_arguments)]
fn menu_input(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, pads: Query<&Gamepad>, mut menu: ResMut<Menu>, mut rt: ResMut<ScriptRt>,
              game: Res<Game>, room: Res<RoomData>) {
    let b = menu::buttons(&keys, &pads);
    let rt = &mut *rt;
    // the field is paused while a menu is open, but its message window keeps typing at 60 Hz
    if menu.is_open() {
        rt.acc = (rt.acc + time.delta_secs() * 60.0).min(4.0);
        while rt.acc >= 1.0 { rt.acc -= 1.0; rt.w.dialog.tick(); }
    }
    let w = &mut rt.w;
    // the PC script asked for the PC menu room
    if let (Some(r), Menu::Closed) = (w.menu_room, *menu) {
        if r == menu::PC_MENU_ROOM {
            println!("PC menu (return map {:?})", w.return_map);
            w.dialog.close();
            *menu = Menu::Pc { cursor: 0, step: PcStep::Choose };
        } else {
            println!("menu room {r:#x} isn't implemented");
            w.menu_room = None;
        }
        return;
    }
    let step = |c: usize, n: usize| if b.up { c.saturating_sub(1) } else if b.down { (c + 1).min(n - 1) } else { c };
    match *menu {
        Menu::Closed => {
            // opened by heroMoveMain (X / Y / Start), see script_tick
        }
        Menu::Field { cursor, notice: true } => {
            if b.a || b.b { w.dialog.close(); *menu = Menu::Field { cursor, notice: false }; }
        }
        Menu::Field { cursor, notice: false } => {
            let cursor = step(cursor, menu::FIELD_ITEMS.len());
            if b.b || b.menu || (b.a && cursor == 3) {
                *menu = Menu::Closed;
            } else if b.a {
                let name = dialogue::strip_codes(&w.message(menu::FIELD_ITEMS[cursor])).trim().to_string();
                w.dialog.open(&format!("[{name} isn't implemented yet]"));
                *menu = Menu::Field { cursor, notice: true };
            } else {
                *menu = Menu::Field { cursor, notice: false };
            }
        }
        Menu::Pc { cursor, step: s } => {
            let pc = |cursor, step| Menu::Pc { cursor, step };
            match s {
                PcStep::Choose => {
                    let cursor = step(cursor, menu::PC_ITEMS.len());
                    if b.b || (b.a && cursor == 3) {
                        // Exit: back to the return map (the room reloads at the PC's entry point)
                        w.dialog.close();
                        w.menu_room = None;
                        if let Some((floor, _, entry)) = w.return_map { w.warp = Some((floor, entry)); }
                        *menu = Menu::Closed;
                    } else if b.a && cursor == 2 {
                        w.dialog.open(&w.message(menu::SAVE_PROMPT));
                        w.dialog.menu = Some(0);
                        *menu = pc(cursor, PcStep::AskSave);
                    } else if b.a {
                        let name = dialogue::strip_codes(&w.message(menu::PC_ITEMS[cursor])).trim().to_string();
                        w.dialog.open(&format!("[{name} isn't implemented yet]"));
                        *menu = pc(cursor, PcStep::Notice);
                    } else {
                        *menu = pc(cursor, PcStep::Choose);
                    }
                }
                PcStep::Notice | PcStep::Saved => {
                    if b.a || b.b { w.dialog.close(); *menu = pc(cursor, PcStep::Choose); }
                }
                PcStep::AskSave | PcStep::AskOverwrite => {
                    if let Some(c) = w.dialog.menu.as_mut() { if b.up { *c = 0; } else if b.down { *c = 1; } }
                    let yes = b.a && w.dialog.menu == Some(0);
                    let no = b.b || (b.a && w.dialog.menu == Some(1));
                    if yes && s == PcStep::AskSave && save::exists() {
                        w.dialog.open(&w.message(menu::OVERWRITE_PROMPT));
                        w.dialog.menu = Some(0);
                        *menu = pc(cursor, PcStep::AskOverwrite);
                    } else if yes {
                        w.dialog.open(&w.message(menu::SAVING));
                        *menu = pc(cursor, PcStep::Saving(0));
                    } else if no && s == PcStep::AskOverwrite {
                        // fn_80038A0C: declining the overwrite asks the first question again
                        w.dialog.open(&w.message(menu::SAVE_PROMPT));
                        w.dialog.menu = Some(0);
                        *menu = pc(cursor, PcStep::AskSave);
                    } else if no {
                        w.dialog.close();
                        *menu = pc(cursor, PcStep::Choose);
                    }
                }
                PcStep::Saving(n) => {
                    if n == 0 {
                        // the save's location is the return map: (floor, previous floor, entry index)
                        let (floor, prev, entry) = w.return_map.unwrap_or((room.floor_id, w.prev_floor, 0));
                        let name = game.room_name_for_floor(floor).unwrap_or_else(|| room.name.clone());
                        if let Err(e) = save::write(&name, entry, prev, w.partner, &w.flags) {
                            println!("save failed: {e}");
                            w.dialog.open(&w.message(menu::SAVE_FAILED));
                            *menu = pc(cursor, PcStep::Saved);
                            return;
                        }
                    }
                    if n + 1 >= menu::SAVING_FRAMES {
                        w.dialog.open(&w.message(menu::SAVED));
                        *menu = pc(cursor, PcStep::Saved);
                    } else {
                        *menu = pc(cursor, PcStep::Saving(n + 1));
                    }
                }
            }
        }
    }
}

/// COLO_KEYS="frame:Key,...": press a key on a frame (released the next frame), for scripted tests.
fn inject_keys(frames: Res<bevy::diagnostic::FrameCount>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    let Ok(v) = std::env::var("COLO_KEYS") else { return };
    let f = frames.0;
    for item in v.split(',') {
        let Some((at, name)) = item.split_once(':') else { continue };
        let Ok(at) = at.trim().parse::<u32>() else { continue };
        let code = match name.trim() {
            "KeyX" => KeyCode::KeyX, "KeyF" => KeyCode::KeyF, "KeyW" => KeyCode::KeyW, "KeyS" => KeyCode::KeyS,
            "KeyA" => KeyCode::KeyA, "KeyD" => KeyCode::KeyD, "Enter" => KeyCode::Enter, "Space" => KeyCode::Space,
            "Backspace" => KeyCode::Backspace, "Escape" => KeyCode::Escape, "ArrowUp" => KeyCode::ArrowUp, "ArrowDown" => KeyCode::ArrowDown,
            other => { if f == 1 { println!("COLO_KEYS: unknown key {other}"); } continue }
        };
        if f == at { keys.press(code); println!("[key] {name} at frame {f}"); }
        if f == at + 1 { keys.release(code); }
    }
}

fn draw_collision(state: Res<ViewState>, room: Res<RoomData>, mut gizmos: Gizmos) {
    if !state.show_collision { return; }
    for (kind, t) in room.colsys.tris() {
        let c = if room.colsys.disabled.contains(&t.group) { Color::srgb(0.4, 0.4, 0.4) } else {
            match kind { "floor" => Color::srgb(1.0, 0.3, 0.2), "wall" => Color::srgb(0.2, 1.0, 0.3), "region" => Color::srgb(1.0, 0.9, 0.1), _ => Color::srgb(0.2, 0.8, 1.0) }
        };
        gizmos.line(t.v[0], t.v[1], c);
        gizmos.line(t.v[1], t.v[2], c);
        gizmos.line(t.v[2], t.v[0], c);
    }
}

/// Apply script commands to the room: door animations and collision (`floorEventCtrlDoor`, 0x80116958),
/// map object animations, collision groups. A map object's resource id ends in its model set index.
fn apply(cmds: &[Cmd], game: &Game, room: &mut RoomData, map: &mut Animator, flags: &mut std::collections::HashSet<u32>) {
    for c in cmds {
        match *c {
            Cmd::Door { index, mode } => {
                let Some(d) = game.doors.get(index) else { continue };
                let (open_flag, close_flag) = (d.open_flag as u32, d.close_flag as u32);
                // the open flag records the door as open: opening an open door / closing a closed one does
                // nothing; a door whose close flag (lock) is not set stays shut
                let skip = match mode {
                    0 if open_flag != 0 && flags.contains(&open_flag) => Some("already open"),
                    0 if close_flag != 0 && !flags.contains(&close_flag) => Some("locked"),
                    2 if open_flag != 0 && !flags.contains(&open_flag) => Some("already closed"),
                    _ => None,
                };
                if let Some(why) = skip { println!("door {index} mode {mode}: {why}"); continue; }
                if open_flag != 0 { if mode < 2 { flags.insert(open_flag); } else { flags.remove(&open_flag); } }
                if d.collision_id >= 0 {
                    room.collision.set_group(d.collision_id as u16, mode >= 2);
                    if mode >= 2 { room.colsys.disabled.remove(&(d.collision_id as u16)); } else { room.colsys.disabled.insert(d.collision_id as u16); }
                }
                let set = (d.resource_id & 0xFF) as usize;
                let (open, close, open_end, close_end) = (d.open_anim, d.close_anim, d.open_end_anim, d.close_end_anim);
                match mode {
                    0 if open >= 0 => map.play_set(set, open as usize, 0.0, MAP_ANIM_RATE, false),
                    2 if close >= 0 => map.play_set(set, close as usize, 0.0, MAP_ANIM_RATE, false),
                    // jump straight to the end pose
                    1 if open_end >= 0 => map.play_set(set, open_end as usize, 0.0, MAP_ANIM_RATE, false),
                    1 if open >= 0 => map.play_set(set, open as usize, -1.0, 0.0, false),
                    3 if close_end >= 0 => map.play_set(set, close_end as usize, 0.0, MAP_ANIM_RATE, false),
                    3 if close >= 0 => map.play_set(set, close as usize, -1.0, 0.0, false),
                    _ => {}
                }
            }
            // std 115: joint and texture animation index/frame/rate 0.5 are both set; the loop type and the start
            // reach the texture animation only when it is linked
            Cmd::MapAnim { resource, anim, frame, looping } => {
                let set = (resource & 0xFF) as usize;
                map.play_set(set, anim, frame, MAP_ANIM_RATE, looping);
                if !map.linked { map.play_tex(set, anim, frame, MAP_ANIM_RATE, true); }
            }
            Cmd::MapVisible { resource, on } => map.set_visible((resource & 0xFF) as usize, on),
            Cmd::MapStop { resource } => map.stop_set((resource & 0xFF) as usize),
            Cmd::SetCol { group, on } => {
                room.collision.set_group(group, on);
                if on { room.colsys.disabled.remove(&group); } else { room.colsys.disabled.insert(group); }
            }
            Cmd::CameraDefault { height, distance, rot_y } => {
                let fov = room.default_camera.map_or(30.0, |c| c.3);
                room.default_camera = Some((distance, height, rot_y, fov));
            }
        }
    }
}

fn do_warp(mut commands: Commands, mut state: ResMut<ViewState>, mut game: ResMut<Game>, roots: Query<Entity, With<RoomRoot>>, mut rt: ResMut<ScriptRt>,
           mut ps: ResMut<particles::Particles>) {
    let Some((floor, _entry)) = state.pending_warp else { return };
    let Some(name) = game.room_name_for_floor(floor) else {
        println!("warp to floor {floor:#x}: no room archive for it");
        state.pending_warp = None;
        return;
    };
    match game.load_room(&name) {
        Ok(r) => {
            for e in &roots { commands.entity(e).despawn(); }
            ps.clear();
            commands.insert_resource(tevmat::RoomLights::new(&r.lights, &r.light_anims));
            commands.insert_resource(r);
            commands.run_system_cached(spawn_room);
            rt.w.dialog.clear();
            state.walk_path.pop_front();
        }
        Err(e) => { println!("warp to {name}: {e}"); state.pending_warp = None; }
    }
}

/// Redraw the 2D message layer (field window, text, next-page arrow, YES/NO) when it changed.
fn update_dialogue(rt: Res<ScriptRt>, game: Res<Game>, mut layer: ResMut<dialogue::Layer>, mut images: ResMut<Assets<Image>>) {
    let d = &rt.w.dialog;
    let page = d.pages.get(d.page);
    let key = (page.map(|p| p.text()).unwrap_or_default(), d.typed, if d.arrow() { d.arrow_t } else { 0 }, d.arrow(), d.menu, d.alpha as u32, d.is_closing());
    if layer.last.as_ref() == Some(&key) { return; }
    let yes = dialogue::pages(&rt.w.message(0x66)).into_iter().next().unwrap_or_default();
    let no = dialogue::pages(&rt.w.message(0x67)).into_iter().next().unwrap_or_default();
    let frame = dialogue::Frame {
        page, typed: d.typed, window_alpha: d.alpha, text: !d.is_closing(), arrow: d.arrow(), arrow_t: d.arrow_t,
        yes_no: d.menu, yes_label: &yes.items, no_label: &no.items,
    };
    let layer = &mut *layer;
    dialogue::render(&mut layer.canvas, &game.ui, &frame);
    let rgba = layer.canvas.to_rgba8();
    if let Ok(path) = std::env::var("COLO_DUMP2D") {
        let _ = image::save_buffer(&path, &rgba, dialogue::SCREEN_W as u32, dialogue::SCREEN_H as u32, image::ExtendedColorType::Rgba8);
    }
    if let Some(mut img) = images.get_mut(&layer.image) { img.data = Some(rgba); }
    layer.last = Some(key);
}

#[allow(clippy::type_complexity)]
fn update_status(room: Res<RoomData>, state: Res<ViewState>, player: Query<&Transform, With<Player>>,
                 mut q: Query<&mut Text, With<StatusText>>) {
    let (Ok(pt), Ok(mut t)) = (player.single(), q.single_mut()) else { return };
    **t = format!("{} (floor {:#x})  pos {:.0} {:.0} {:.0}   WASD run, Shift walk, F talk, X menu, Tab {}, C collision",
        room.name, room.floor_id, pt.translation.x, pt.translation.y, pt.translation.z, if state.free_cam { "game camera" } else { "free camera" });
}

#[allow(clippy::too_many_arguments)]
fn test_hooks(mut commands: Commands, frames: Res<bevy::diagnostic::FrameCount>, room: Res<RoomData>, mut rt: ResMut<ScriptRt>,
              mut state: ResMut<ViewState>, mut cam: Query<&mut GameCam>, mut anims: Query<&mut Animator, Without<MapModel>>, mut exit: MessageWriter<AppExit>) {
    let f = frames.0;
    if f == 2 {
        if let Ok(v) = std::env::var("COLO_CAM") {
            let n: Vec<f32> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
            if n.len() == 5 {
                if let Ok(mut c) = cam.single_mut() {
                    c.fly = Vec3::new(n[0], n[1], n[2]);
                    c.fly_yaw = n[3].to_radians();
                    c.fly_pitch = n[4].to_radians();
                }
            }
        }
        if std::env::var("COLO_COLLISION").is_ok() { state.show_collision = true; }
        // COLO_ANIM="anim,frame": freeze the player on one animation frame
        if let Some(n) = std::env::var("COLO_ANIM").ok().map(|v| v.split(',').filter_map(|x| x.trim().parse::<f32>().ok()).collect::<Vec<_>>()) {
            for mut a in anims.iter_mut().filter(|a| a.is_player) {
                a.set(n[0] as usize);
                for p in a.active.iter_mut() { p.frame = n.get(1).copied().unwrap_or(0.0); p.rate = 0.0; }
            }
        }
    }
    // COLO_FOLLOW=1: log how far Rui is from Wes
    if std::env::var("COLO_FOLLOW").is_ok() && f.is_multiple_of(15) {
        let p = |i| rt.w.actors.iter().find(|a| a.index == i).map(|a| a.pos);
        if let (Some(h), Some(r)) = (p(HERO), p(PARTNER)) { println!("f{f} gap {:.1} hero {:.1} {:.1} {:.1} rui {:.1} {:.1} {:.1}", Vec2::new(h.x - r.x, h.z - r.z).length(), h.x, h.y, h.z, r.x, r.y, r.z); }
    }
    // COLO_CHECK="x,z,yaw_deg[,frame]": stand there facing that way and press A at that frame (default 20)
    let check = std::env::var("COLO_CHECK").ok().map(|v| v.split(',').filter_map(|x| x.trim().parse::<f32>().ok()).collect::<Vec<_>>());
    let check_at = check.as_ref().and_then(|n| n.get(3)).map_or(20, |&v| v as u32);
    if f + 2 == check_at {
        if let Some(n) = check {
            if let (3 | 4, Some(h)) = (n.len(), rt.w.actor(HERO)) {
                h.pos = Vec3::new(n[0], h.pos.y, n[1]);
                h.pos.y = room.collision.floor_height(h.pos, 30.0, 60.0).unwrap_or(h.pos.y);
                h.yaw = n[2].to_radians();
                h.dir = h.yaw;
                h.teleported = true;
            }
        }
    }
    // the A press goes through heroMoveMain like a real one (talk, else the check ray)
    if f == check_at && std::env::var("COLO_CHECK").is_ok() { state.force_a = true; }
    if f == 20 {
        if let Some(i) = std::env::var("COLO_TALK").ok().and_then(|v| v.parse::<u32>().ok()) { begin_talk(&mut rt, &room, i); }
    }
    if let Some(n) = std::env::var("COLO_PRESS").ok().and_then(|v| v.parse::<u32>().ok()) {
        if n > 0 && f.is_multiple_of(n) && rt.w.dialog.visible() {
            let page = rt.w.dialog.pages.get(rt.w.dialog.page).map(|p| p.text()).unwrap_or_default();
            println!("[press] {}{}", page.replace('\n', " / "), if rt.w.dialog.menu.is_some() { "  (menu: YES)" } else { "" });
            rt.w.dialog.press();
        }
    }
    // COLO_SHOT=path (a "{}" in it is replaced by the frame), COLO_SHOT_FRAME=90 or a list "90,120,150"
    if let Ok(path) = std::env::var("COLO_SHOT") {
        let at: Vec<u32> = std::env::var("COLO_SHOT_FRAME").ok().map(|v| v.split(',').filter_map(|x| x.trim().parse().ok()).collect()).unwrap_or_else(|| vec![90]);
        if at.contains(&f) {
            commands.spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                .observe(bevy::render::view::screenshot::save_to_disk(path.replace("{}", &format!("{f:05}"))));
        }
        if at.iter().max().is_some_and(|&m| f == m + 40) { exit.write(AppExit::Success); }
    }
}
