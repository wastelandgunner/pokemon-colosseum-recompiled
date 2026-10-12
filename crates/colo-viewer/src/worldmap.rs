//! The world map (room 0x38E, `world_map.fsys`): the area select every town exit warps to, and the bike
//! ride to the picked place. NOTES.md §4e has the decompiled original.
//!
//! The room has no map REL; its record's pre/main/post funcs are native (`wmapInit` 0x800259B0, `wmapMain`
//! 0x80025730, `wmapExit` 0x800255A4) and its script only plays the BGM and keeps story flags. `wmapMain`
//! opens menu 0xBD: the map picture, one icon per unlocked destination, a crosshair that slides between them
//! (`wmapCursorMove` follows the D-pad links of `worldmap_menu_rel`), a preview snapshot and the
//! destination's description. A picks the place under the crosshair, B goes back where you came from. Unless
//! the pick is that same place (Pyrite and The Under count as one), `wmapRide` plays the bike ride
//! (`move_demo.rdat`, a bike model riding map object 0xC6A1000 along one of 20 path animations filmed by
//! its `cam_move_demo_NN`) while the map picture zooms and fades away; then `floorLink(target, 0)` and
//! `wmapExit` fade out.
//!
//! `wmapRideOverlay`'s first quad (the live frame captured and drawn zoomed 3x → 1x while fading out) is a
//! second camera rendering the same view into a texture, shown on a scaled UI image ([`zoom_sync`]).
//! The bike casts its projected shadow on `move_demo` set 2 (`shadow.rs`) and kicks up `bike_effect_poke`
//! particles from its animation's particle keys (`particles.rs`).

use crate::dialogue::{self, Canvas, UiArt, SCREEN_H, SCREEN_W};
use crate::model::{spawn_model, Animator, ModelAssets};
use crate::script::{AnimCam, ScriptRt, HERO, PARTNER};
use crate::world::{LoadedModel, RoomData};
use crate::{MapModel, MAP_ANIM_RATE};
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use colo_audio::{layer, SoundTask};
use colo_formats::gtx;
use colo_formats::worldmap::{WorldMap, DESCRIPTION_COLUMN};
use std::collections::HashMap;
use std::sync::Arc;


/// What `world_map.fsys` adds to its room: the tables, the menu textures and the two bike models.
pub struct WmapData {
    pub tables: WorldMap,
    /// `.gtx` images by FSYS identifier (menu_039 0x0C681200 map picture, menu_038 0x0C691200 atlas,
    /// mapsnap000..009 snapshots).
    pub tex: HashMap<u32, gtx::Image>,
    /// Bike models by resource id: 0xC6B1000 `bike_hizuki` (Rui along), 0xC6C1000 `bike_pokemon`.
    pub bikes: HashMap<u32, LoadedModel>,
    /// `move_demo` model set 0 (map object 0xC6A1000): no JObj, just the ride path animations by index.
    pub path_anims: Vec<colo_formats::hsdanim::Animation>,
    /// Shadow view half-size per bike: ceil((x + y + z extent) / 3) of the bind pose (the caster size of
    /// `modelShadowRender`).
    pub bike_size: HashMap<u32, f32>,
    /// `bike_effect_poke`, linked to the bike (`GSmodelLinkToGSparticleBank`, attach mode 4).
    pub bank: Option<Arc<colo_formats::gpt1::Gpt1>>,
}

const TEX_MAP: u32 = 0x0C68_1200;
const TEX_ATLAS: u32 = 0x0C69_1200;
/// D4_out's snapshot once it's Realgam Tower (`wmapDrawSnapshot`: floor 0x66 with flag 0x45D).
const SNAP_REALGAM: u32 = 0x0C5F_1200;
const FLAG_REALGAM: u32 = 0x45D;
/// Rui rides along (`wmapRide`: bike_hizuki instead of bike_pokemon).
const FLAG_RUI: u32 = 0x3E5;
const BIKE_RUI: u32 = 0xC6B_1000;
const BIKE_ALONE: u32 = 0xC6C_1000;
const BGM_RIDE: u32 = 0x449;
const SE_CURSOR: u32 = 0x464;
/// Floors: D4_out (construction site) and D4_out_2 (Realgam Tower), Pyrite and The Under.
const FLOOR_D4: u32 = 0x66;
const FLOOR_D4_2: u32 = 0x7B;
const FLOOR_PYRITE: u32 = 0xF;
const FLOOR_UNDER: u32 = 0x7A;

/// Atlas sprites (main.dol sprite table 0x802E7CE8, 0x18 each: +8 u, +0xA v, +0xC w, +0xE h, +0x10 texture).
mod spr {
    pub const ICON: (i32, i32, i32, i32) = (29, 35, 33, 33); // 0x240
    pub const MARK: (i32, i32, i32, i32) = (29, 69, 15, 15); // 0x251
    pub const VLINE: (i32, i32, i32, i32) = (40, 103, 3, 8); // 0x241
    pub const HLINE: (i32, i32, i32, i32) = (29, 105, 8, 3); // 0x242
    pub const CORNER: (i32, i32, i32, i32) = (29, 87, 15, 15); // 0x23F
    pub const BOX_CORNER: (i32, i32, i32, i32) = (28, 1, 23, 23); // 0x24D
    pub const BOX_SIDE: (i32, i32, i32, i32) = (28, 26, 23, 8); // 0x24E
    pub const BOX_FILL: (i32, i32, i32, i32) = (53, 26, 8, 8); // 0x24F
    pub const BOX_TOP: (i32, i32, i32, i32) = (53, 1, 8, 23); // 0x250
    pub const SNAP_SIDE: (i32, i32, i32, i32) = (1, 1, 26, 112); // 0x283
    pub const SNAP_BACK: (i32, i32, i32, i32) = (46, 70, 8, 8); // 0x284
    pub const SNAP_TOP: (i32, i32, i32, i32) = (46, 79, 8, 10); // 0x285
    pub const SNAP_BOTTOM: (i32, i32, i32, i32) = (46, 90, 8, 10); // 0x286
    pub const BUTTON_A: (i32, i32, i32, i32) = (63, 28, 29, 29); // 0x300
    pub const BUTTON_B: (i32, i32, i32, i32) = (62, 1, 27, 27); // 0x301
}

/// Menu items 0x907.. (locations): +2 x, +4 y of the menu item records (main.dol 0x802EF0A8 + i·0x1C).
fn item_xy(item: u32) -> Vec2 {
    match item {
        0x907 => Vec2::new(509.0, 251.0),
        0x908 => Vec2::new(489.0, 140.0),
        0x909 => Vec2::new(283.0, 170.0),
        0x90A => Vec2::new(287.0, 40.0),
        0x90B => Vec2::new(289.0, 239.0),
        0x90C => Vec2::new(355.0, 257.0),
        0x90D => Vec2::new(131.0, 268.0),
        0x918 => Vec2::new(80.0, 94.0),
        0x919 => Vec2::new(110.0, 245.0),
        _ => Vec2::ZERO,
    }
}

/// `cos(deg)` (`FUN_800E0CA0`, GSmath's cosine table).
fn cos_deg(t: f32) -> f32 { t.to_radians().cos() }

#[derive(Clone, Copy, Debug, PartialEq)]
enum Phase {
    /// Menu 0xBD is up.
    Select,
    /// `wmapRide`: frames since it started, and the ride camera entry.
    Ride { frame: u32, target: u32, cam: usize },
    /// `wmapExit`: fading out before `floorLink`; frames since it started.
    Leave { frame: u32, target: u32, ride: bool },
    /// `floorLink` issued; waiting for the room to change.
    Gone,
}

struct St {
    data: Arc<WmapData>,
    phase: Phase,
    /// Unlocked per location (`menuItemBiosSetSelectFlag`).
    open: Vec<bool>,
    /// Location the marker sits on (0x8047A390: the place you came from).
    origin: usize,
    /// Cursor location (0x8047A368) and the one it is moving to (0x8047A36C).
    cur: usize,
    next: usize,
    /// Cursor move: 0x8047A370 flag, 0x80478898 timer (90 → 0 by 4.5 a frame), item positions
    /// 0x803A2058 (from) and 0x803A204C (to; 0 until the first move).
    moving: bool,
    t: f32,
    from: Vec2,
    to: Vec2,
    /// Crosshair bob (0x8047A374 between 12 and 20, step 0x8047A378 = ±0.5) and the marker's alpha pulse
    /// (item alpha, step 0x8047A37C = ±8, between 64 and 255).
    bob: f32,
    bob_v: f32,
    mark_alpha: f32,
    mark_v: f32,
    /// Ride heading (0x8047A380): 0 when the last pick lies east of the origin, else 1.
    heading: usize,
    /// The map screen as captured for the ride overlay, and its fade timer (0x8047A3A4, 90 → 0 by 4.5).
    capture: Option<Vec<[f32; 4]>>,
    overlay_t: f32,
    sounds: Vec<SoundTask>,
    acc: f32,
    /// Buttons pressed since the last 60 Hz tick (D-pad mask, A, B).
    input: (u8, bool, bool),
    /// The bike (riding the path object) and the path animation's (index, frame).
    bike: Option<Entity>,
    path: Option<(usize, f32)>,
    /// The layer was last uploaded empty (nothing to draw until the room changes).
    blank: bool,
    /// `wmapRideOverlay` quad 1 timer (0x8047A3A0, 90 → 0 by 2.25 a tick) and its camera / image entities.
    zoom_t: f32,
    zoom: Option<(Entity, Entity)>,
    canvas: Canvas,
}

/// World map state. The ride counters (0x8047A394 / 0x8047A398) live in `.sbss` and survive between visits.
#[derive(Resource, Default)]
pub struct Wmap {
    st: Option<St>,
    ride_next: [usize; 2],
}

impl Wmap {
    pub fn active(&self) -> bool { self.st.is_some() }
}

/// The world map's 640x480 layer (under the message window).
#[derive(Component)]
pub struct WmapLayer;

#[derive(Resource)]
pub struct WmapImage(Handle<Image>);

pub fn spawn_layer(commands: &mut Commands, images: &mut Assets<Image>) {
    let img = Image::new(Extent3d { width: SCREEN_W as u32, height: SCREEN_H as u32, depth_or_array_layers: 1 }, TextureDimension::D2,
        vec![0u8; SCREEN_W * SCREEN_H * 4], TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    let image = images.add(img);
    commands.spawn((GlobalZIndex(9), Node {
        position_type: PositionType::Absolute, left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0),
        justify_content: JustifyContent::Center, ..default()
    })).with_children(|l| {
        l.spawn((WmapLayer, Visibility::Hidden, ImageNode::new(image.clone()), GlobalZIndex(9),
            Node { height: Val::Percent(100.0), aspect_ratio: Some(4.0 / 3.0), ..default() }));
    });
    commands.insert_resource(WmapImage(image));
}

/// One frame of world map buttons: D-pad mask (1 up, 2 right, 4 down, 8 left) when a direction was just
/// pressed, A, B.
fn read_buttons(keys: &ButtonInput<KeyCode>, pads: &Query<&Gamepad>) -> (u8, bool, bool) {
    let dirs = [
        (1u8, [KeyCode::ArrowUp, KeyCode::KeyW], GamepadButton::DPadUp),
        (2, [KeyCode::ArrowRight, KeyCode::KeyD], GamepadButton::DPadRight),
        (4, [KeyCode::ArrowDown, KeyCode::KeyS], GamepadButton::DPadDown),
        (8, [KeyCode::ArrowLeft, KeyCode::KeyA], GamepadButton::DPadLeft),
    ];
    let (mut held, mut fresh) = (0u8, false);
    for (bit, ks, pb) in dirs {
        if ks.iter().any(|&k| keys.pressed(k)) || pads.iter().any(|p| p.pressed(pb)) { held |= bit; }
        if ks.iter().any(|&k| keys.just_pressed(k)) || pads.iter().any(|p| p.just_pressed(pb)) { fresh = true; }
    }
    let a = [KeyCode::KeyF, KeyCode::Enter, KeyCode::Space].iter().any(|&k| keys.just_pressed(k)) || pads.iter().any(|p| p.just_pressed(GamepadButton::South));
    let b = [KeyCode::Backspace, KeyCode::Escape].iter().any(|&k| keys.just_pressed(k)) || pads.iter().any(|p| p.just_pressed(GamepadButton::East));
    (if fresh { held } else { 0 }, a, b)
}

fn flag(rt: &ScriptRt, f: u32) -> bool { f == 0 || rt.w.flags.contains(&f) }

/// `wmapInit` + the setup half of `wmapMain`.
fn enter(data: Arc<WmapData>, rt: &mut ScriptRt, map: &mut Animator) -> St {
    // wmapInit: hide move_demo's ride sets; the start place is the previous floor (Realgam counts as D4_out)
    for set in 0..3 { map.set_visible(set, false); }
    let prev = if rt.w.prev_floor == FLOOR_D4_2 { FLOOR_D4 } else { rt.w.prev_floor };
    let locs = &data.tables.locations;
    let open: Vec<bool> = locs.iter().map(|l| flag(rt, l.flag)).collect();
    let start = locs.iter().position(|l| l.floor == prev).unwrap_or(0);
    // the field characters aren't part of this room
    for i in [HERO, PARTNER] { if let Some(a) = rt.w.actor(i) { a.visible = false; } }
    println!("world map: from floor {prev:#x} (location {start}); open {:?}", open.iter().enumerate().filter(|x| *x.1).map(|x| x.0).collect::<Vec<_>>());
    St {
        data, phase: Phase::Select, open, origin: start, cur: start, next: start, moving: false, t: 90.0, from: Vec2::ZERO, to: Vec2::ZERO,
        bob: 0.0, bob_v: 0.5, mark_alpha: 255.0, mark_v: 8.0, heading: 0, capture: None, overlay_t: 0.0, sounds: vec![], acc: -1.0, input: (0, false, false), bike: None, path: None, blank: false, zoom_t: 0.0, zoom: None,
        canvas: Canvas::default(),
    }
}

/// Description message for a location (`wmapDrawInfo`): the first block whose flag is 0 or set.
fn description(st: &St, rt: &ScriptRt, loc: usize) -> u32 {
    st.data.tables.descriptions.iter().find(|d| flag(rt, d.flag))
        .map_or(1, |d| d.messages[DESCRIPTION_COLUMN.get(loc).copied().unwrap_or(0)])
}

fn snapshot(st: &St, rt: &ScriptRt, loc: usize) -> u32 {
    let l = st.data.tables.locations[loc];
    if l.floor == FLOOR_D4 && flag(rt, FLAG_REALGAM) { SNAP_REALGAM } else { l.snapshot }
}

fn play(st: &mut St, id: u32) { st.sounds.push(SoundTask::set_bgm(id, 0, 255)); }

/// Draw menu 0xBD (its items in list order).
fn draw_menu(st: &mut St, rt: &ScriptRt, art: &UiArt) {
    let d = Arc::clone(&st.data);
    let (Some(map), Some(atlas)) = (d.tex.get(&TEX_MAP), d.tex.get(&TEX_ATLAS)) else { return };
    let (snap_next, snap_cur) = (snapshot(st, rt, st.next), snapshot(st, rt, st.cur));
    // 0x91C's text: cross-faded at the halfway point of a cursor move
    let (loc, alpha) = if !st.moving { (st.cur, 255.0) } else if st.t > 45.0 { (st.cur, 255.0 * (st.t - 45.0) / 45.0) } else { (st.next, 255.0 * (45.0 - st.t) / 45.0) };
    let desc = description(st, rt, loc);
    let cv = &mut st.canvas;
    cv.clear();
    let a = |v: u8| v as f32 / 255.0;
    // 0x906: the map picture
    cv.sprite(map, (0, 0, 640, 480), (0, 0), (640, 480), 1.0);
    // 0x907..0x919: unlocked places (item alpha 0xBF)
    for (i, l) in d.tables.locations.iter().enumerate() {
        if !st.open[i] { continue; }
        let p = item_xy(l.item);
        cv.sprite(atlas, spr::ICON, (p.x as i32, p.y as i32), (33, 33), a(0xBF));
    }
    // 0x91A: the you-are-here marker, pulsing
    let o = item_xy(d.tables.locations[st.origin].item);
    cv.sprite(atlas, spr::MARK, (o.x as i32 + 8, o.y as i32 + 8), (15, 15), st.mark_alpha / 255.0);
    // crosshair: lines through the cursor (0x935, 0x936) and four corners bobbing around it (0x931-0x934)
    let c = if st.moving { st.from.lerp(st.to, cos_deg(st.t)) } else { item_xy(d.tables.locations[st.cur].item) };
    let (cx, cy) = (c.x.trunc() as i32, c.y.trunc() as i32);
    cv.sprite(atlas, spr::HLINE, (0, cy + 15), (640, 3), 1.0);
    cv.sprite(atlas, spr::VLINE, (cx + 15, 0), (3, 480), 1.0);
    let b = st.bob;
    let corner = |dx: f32, dy: f32| ((9.0 + c.x + dx) as i32, (9.0 + c.y + dy) as i32);
    let corners = [(corner(-b, -b), (false, false)), (corner(-b, b), (false, true)), (corner(b, -b), (true, false)), (corner(b, b), (true, true))];
    for (at, flip) in corners { cv.sprite_flip(atlas, spr::CORNER, at, (15, 15), 1.0, flip); }
    // preview: dark backing (0x9AC), the target's snapshot (0x91B), the current one fading out over it (0xA41)
    cv.sprite(atlas, spr::SNAP_BACK, (23, 334), (134, 100), a(0x4D));
    if let Some(t) = d.tex.get(&snap_next) { cv.sprite(t, (0, 0, 134, 100), (23, 334), (134, 100), 1.0); }
    let fade = if st.moving { st.t / 90.0 } else { 1.0 };
    if let Some(t) = d.tex.get(&snap_cur) { cv.sprite(t, (0, 0, 134, 100), (23, 334), (134, 100), fade); }
    // its frame (0x9AD-0x9B0)
    cv.sprite(atlas, spr::SNAP_SIDE, (14, 329), (26, 112), 1.0);
    cv.sprite_flip(atlas, spr::SNAP_SIDE, (140, 329), (26, 112), 1.0, (true, false));
    cv.sprite(atlas, spr::SNAP_TOP, (40, 329), (100, 10), 1.0);
    cv.sprite(atlas, spr::SNAP_BOTTOM, (40, 431), (100, 10), 1.0);
    // two translucent boxes (alpha 0x66): the description (0x91D-0x925) and the buttons (0xCAB-0xCB3)
    for (x, y, w, h) in [(168, 306, 436, 123), (516, 31, 97, 46)] {
        nine_slice(cv, atlas, x, y, w, h, a(0x66));
    }
    cv.sprite(atlas, spr::BUTTON_A, (523, 38), (29, 29), 1.0);
    cv.sprite(atlas, spr::BUTTON_B, (524, 67), (27, 27), 1.0);
    for (id, y) in [(0x3CE4, 40), (0x3CDF, 68)] {
        let page = dialogue::pages(&rt.w.message(id)).into_iter().next().unwrap_or_default();
        dialogue::draw_text(cv, art, &page.items, usize::MAX, 554, y, 0, false);
    }
    // 0x91C: the description (font 0, message 0xCE)
    let mut items = dialogue::pages(&rt.w.message(desc)).into_iter().next().unwrap_or_default().items;
    let col = [1.0, 1.0, 1.0, alpha.trunc() / 255.0].map(|v| (v * 255.0) as u8);
    items.insert(0, dialogue::Item::Color(col));
    dialogue::draw_text(cv, art, &items, usize::MAX, 191, 316, 0, false);
}

/// A 23-px-border box: corners 0x24D, edges 0x24E / 0x250, fill 0x24F (mirrored for the far sides);
/// `(x, y)` is the outer top-left, `w`/`h` the inner size.
fn nine_slice(cv: &mut Canvas, atlas: &gtx::Image, x: i32, y: i32, w: i32, h: i32, alpha: f32) {
    let (ix, iy, rx, by) = (x + 23, y + 23, x + 23 + w, y + 23 + h);
    cv.sprite(atlas, spr::BOX_FILL, (ix, iy), (w, h), alpha);
    cv.sprite(atlas, spr::BOX_TOP, (ix, y), (w, 23), alpha);
    cv.sprite_flip(atlas, spr::BOX_TOP, (ix, by), (w, 23), alpha, (false, true));
    cv.sprite(atlas, spr::BOX_SIDE, (x, iy), (23, h), alpha);
    cv.sprite_flip(atlas, spr::BOX_SIDE, (rx, iy), (23, h), alpha, (true, false));
    for (cx, cy, f) in [(x, y, (false, false)), (rx, y, (true, false)), (x, by, (false, true)), (rx, by, (true, true))] {
        cv.sprite_flip(atlas, spr::BOX_CORNER, (cx, cy), (23, 23), alpha, f);
    }
}

/// `wmapDrawCaptureQuad` for the map capture: the full-screen quad scaled by `s` about the centre and moved
/// to `centre`, at `alpha`.
fn draw_capture(cv: &mut Canvas, src: &[[f32; 4]], s: f32, centre: Vec2, alpha: f32) {
    if alpha <= 0.0 { return; }
    let (w, h) = (SCREEN_W as i32, SCREEN_H as i32);
    let at = |x: i32, y: i32| src[(y.clamp(0, h - 1) * w + x.clamp(0, w - 1)) as usize];
    for y in 0..h {
        for x in 0..w {
            let u = (x as f32 + 0.5 - centre.x) / s + 320.0 - 0.5;
            let v = (y as f32 + 0.5 - centre.y) / s + 240.0 - 0.5;
            if u < -0.5 || v < -0.5 || u > w as f32 - 0.5 || v > h as f32 - 0.5 { continue; }
            let (u0, v0) = (u.floor(), v.floor());
            let (fu, fv) = (u - u0, v - v0);
            let (iu, iv) = (u0 as i32, v0 as i32);
            let (a, b, c, d) = (at(iu, iv), at(iu + 1, iv), at(iu, iv + 1), at(iu + 1, iv + 1));
            let mut o = [0.0; 4];
            for k in 0..4 { o[k] = (a[k] * (1.0 - fu) + b[k] * fu) * (1.0 - fv) + (c[k] * (1.0 - fu) + d[k] * fu) * fv; }
            o[3] *= alpha;
            cv.blend(x, y, o);
        }
    }
}

/// Runs the world map at 60 Hz while its room is loaded.
#[allow(clippy::too_many_arguments)]
pub fn update(mut commands: Commands, time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, pads: Query<&Gamepad>, mut wmap: ResMut<Wmap>,
              room: Res<RoomData>, game: Res<crate::world::Game>, mut rt: ResMut<ScriptRt>, mut maps: Query<&mut Animator, With<MapModel>>,
              mut assets: ModelAssets, image: Res<WmapImage>, mut layer: Query<&mut Visibility, With<WmapLayer>>,
              (roots, mut tf, meshes, bikes, mut shadow_mats): (Query<Entity, With<crate::RoomRoot>>, Query<&mut Transform>,
                  Query<(&Mesh3d, Option<&bevy::mesh::skinning::SkinnedMesh>, Option<&ChildOf>)>, Query<&Animator, Without<MapModel>>,
                  ResMut<Assets<crate::shadow::ShadowMat>>), mut particles: ResMut<crate::particles::Particles>) {
    let wmap = &mut *wmap;
    let Some(data) = room.world_map.clone() else {
        if wmap.st.take().is_some() { if let Ok(mut v) = layer.single_mut() { *v = Visibility::Hidden; } }
        return;
    };
    let Ok(mut map) = maps.single_mut() else { return };
    let rt = &mut *rt;
    if wmap.st.is_none() {
        wmap.st = Some(enter(data, rt, &mut map));
        if let Ok(mut v) = layer.single_mut() { *v = Visibility::Visible; }
    }
    let ride_next = &mut wmap.ride_next;
    let Some(st) = wmap.st.as_mut() else { return };
    let (d, a, b) = read_buttons(&keys, &pads);
    if d != 0 { st.input.0 = d; }
    st.input.1 |= a;
    st.input.2 |= b;
    st.acc = if st.acc < 0.0 { 1.0 } else { (st.acc + time.delta_secs() * 60.0).min(4.0) };
    let mut ticks = 0;
    while st.acc >= 1.0 {
        st.acc -= 1.0;
        let locs = st.data.tables.locations.clone();
        let (dirs, a, b) = std::mem::take(&mut st.input);
        match st.phase {
            Phase::Select => {
                let mut pick = None;
                // wmapMenuUpdate: D-pad → first link from the cursor's item whose directions cover the
                // pressed ones and whose target is unlocked
                if !st.moving && dirs != 0 {
                    let from = locs[st.cur].item;
                    let target = st.data.tables.links.iter()
                        .filter(|l| l.from == from && dirs == dirs & l.dir)
                        .find_map(|l| locs.iter().position(|x| x.item == l.to).filter(|&i| st.open[i]));
                    if let Some(n) = target.filter(|&n| n != st.cur) {
                        st.next = n;
                        st.moving = true;
                        st.t = 90.0;
                        st.from = item_xy(locs[st.cur].item);
                        st.to = item_xy(locs[n].item);
                        let d = st.to - item_xy(locs[st.origin].item);
                        if d != Vec2::ZERO { st.heading = if d.normalize().x >= 0.0 { 0 } else { 1 }; }
                        play(st, SE_CURSOR);
                    }
                }
                // wmapMenuInput: A picks (not while moving), B backs out
                if a && !st.moving {
                    let l = locs[st.cur];
                    pick = Some(if l.floor == FLOOR_D4 && flag(rt, FLAG_REALGAM) { FLOOR_D4_2 } else { l.floor });
                    crate::audio::se(crate::audio::SE_DECIDE);
                } else if b {
                    pick = Some(rt.w.prev_floor);
                }
                st.bob += st.bob_v;
                if st.bob < 12.0 { st.bob_v = -st.bob_v; st.bob = 12.0; } else if st.bob > 20.0 { st.bob_v = -st.bob_v; st.bob = 20.0; }
                if st.moving {
                    st.t -= 4.5;
                    if st.t < 0.0 { st.t = 0.0; st.moving = false; st.cur = st.next; }
                }
                st.mark_alpha += st.mark_v;
                if st.mark_alpha < 64.0 { st.mark_v = -st.mark_v; st.mark_alpha = 64.0; } else if st.mark_alpha > 255.0 { st.mark_v = -st.mark_v; st.mark_alpha = 255.0; }
                if let Some(target) = pick {
                    let prev = rt.w.prev_floor;
                    let same = target == prev || (target == FLOOR_UNDER && prev == FLOOR_PYRITE) || (target == FLOOR_PYRITE && prev == FLOOR_UNDER);
                    println!("world map: {} floor {target:#x}{}", if b { "back to" } else { "picked" }, if same { "" } else { ", riding" });
                    if same {
                        st.phase = Phase::Leave { frame: 0, target, ride: false };
                    } else {
                        st.capture = Some(st.canvas.px.clone());
                        st.overlay_t = 90.0;
                        // the next camera of the heading (round robin per heading: entries 10-19 east, 0-9 west)
                        let k = ride_next[st.heading];
                        ride_next[st.heading] = (k + 1) % 10;
                        let cam = if st.heading == 0 { 10 + k } else { k };
                        st.phase = Phase::Ride { frame: 0, target, cam };
                    }
                }
            }
            Phase::Ride { frame, target, cam } => {
                let rc = st.data.tables.ride_cams.get(cam).copied();
                if frame == 0 {
                    // wmapRide: the ride sets, the bike on map object 0xC6A1000, BGM, the camera
                    for set in 0..3 { map.set_visible(set, true); }
                    let bike = if flag(rt, FLAG_RUI) { BIKE_RUI } else { BIKE_ALONE };
                    if let (Some(m), Ok(room_root)) = (st.data.bikes.get(&bike), roots.single()) {
                        let (root, mut anim) = spawn_model(&mut commands, &mut assets, &m.0, Arc::clone(&m.1));
                        anim.play_set(0, 0, 0.0, MAP_ANIM_RATE, true);
                        commands.entity(root).insert(anim);
                        commands.entity(room_root).add_child(root);
                        st.bike = Some(root);
                        // its dust: the bike animation's particle keys start generators of bike_effect_poke
                        if let Some(bank) = st.data.bank.clone() {
                            let b = particles.add_bank(bank, assets.images());
                            commands.entity(root).insert(crate::particles::ParticleLink::new(b, 4));
                        }
                    }
                    st.path = Some((0, 0.0));
                    // quad 1: the live frame, captured by a second camera with the field camera's view
                    let img = assets.add_image(Image::new_target_texture(SCREEN_W as u32, SCREEN_H as u32, TextureFormat::Rgba8UnormSrgb, None));
                    if let Ok(room_root) = roots.single() {
                        let cam = commands.spawn((Camera3d::default(),
                            Camera { order: -1, clear_color: bevy::camera::ClearColorConfig::Custom(Color::BLACK), ..default() },
                            bevy::camera::RenderTarget::Image(img.clone().into()), ZoomCam, Transform::default(),
                            Projection::custom(crate::tv::FixedAspect(PerspectiveProjection::default())))).id();
                        commands.entity(room_root).add_child(cam);
                        let frame = commands.spawn((GlobalZIndex(8), Node {
                            position_type: PositionType::Absolute, left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0),
                            justify_content: JustifyContent::Center, ..default()
                        })).with_children(|l| {
                            // the 4:3 screen clips the zoomed image
                            l.spawn((GlobalZIndex(8), Node { height: Val::Percent(100.0), aspect_ratio: Some(4.0 / 3.0), overflow: Overflow::clip(), ..default() }))
                                .with_children(|c| {
                                    c.spawn((ZoomImage, ImageNode::new(img).with_color(Color::srgba(1.0, 1.0, 1.0, 1.0)), GlobalZIndex(8),
                                        UiTransform::from_scale(Vec2::splat(3.0)),
                                        Node { width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() }));
                                });
                        }).id();
                        st.zoom = Some((cam, frame));
                    }
                    st.zoom_t = 90.0;
                    play(st, BGM_RIDE);
                    if let Some(c) = rc {
                        let end = rt.w.cam_ends.get(&c.camera).copied().unwrap_or(0.0);
                        rt.w.anim_cam = Some(AnimCam { resource: c.camera, frame: 0.0, end_frame: end, playing: true });
                        println!("world map ride: camera {cam} {:#x} (end frame {end}), path anim {}, bike {bike:#x}", c.camera, c.anim);
                    }
                }
                // the bike's shadow on move_demo set 2 (0xC6A1002) from light 0xC6A1603 (GSmodelSetShadow*:
                // flags 2, texture 640x480), once its meshes exist
                if frame == 1 {
                    let light = room.lights.get(3);
                    let bike_anim = st.bike.and_then(|b| bikes.get(b).ok());
                    if let (Some(light), Some(ba), Ok(room_root), Some(bike)) = (light, bike_anim, roots.single(), st.bike) {
                        let grab = |e: Entity| meshes.get(e).ok().map(|(m, s, p)| (e, m.0.clone(), s.cloned(), p.map(|p| p.parent())));
                        let casters: Vec<_> = ba.targets.iter().filter_map(|t| grab(t.entity)).collect();
                        let receivers: Vec<_> = map.targets.iter().filter(|t| t.set == 2).filter_map(|t| grab(t.entity)).collect();
                        let size = st.data.bike_size.get(&(if flag(rt, FLAG_RUI) { BIKE_RUI } else { BIKE_ALONE })).copied().unwrap_or(20.0);
                        crate::shadow::spawn(&mut commands, assets.images(), &mut shadow_mats, room_root, bike, size, &casters, &receivers, light, (640, 480), 0x80);
                    }
                }
                // one frame later the path animation starts
                if let (1, Some(c)) = (frame, rc) { st.path = Some((c.anim as usize, 0.0)); }
                st.overlay_t = (st.overlay_t - 4.5).max(0.0);
                // 1 + 174 frames, then floorLink
                st.phase = if frame + 1 >= 175 { Phase::Leave { frame: 0, target, ride: true } } else { Phase::Ride { frame: frame + 1, target, cam } };
            }
            Phase::Gone => {}
            Phase::Leave { frame, target, ride } => {
                st.overlay_t = (st.overlay_t - 4.5).max(0.0);
                // wmapExit: fadeSet(2.0 after a ride, else 0.5, out); a ride stops its BGM at once and waits
                // a second before the music fades over 2 s, otherwise it fades over 0.5 s
                if frame == 0 {
                    rt.w.fade_target = 1.0;
                    rt.w.fade_speed = if ride { 0.5 } else { 2.0 };
                    if ride {
                        layer::with(|l| { let t = SoundTask::stop(l, BGM_RIDE, 0); st.sounds.push(t); });
                    } else {
                        layer::with(|l| { let cur = l.current_bgm(); l.set_volume(cur, 500, 0); });
                    }
                }
                if ride && frame == 60 { layer::with(|l| { let cur = l.current_bgm(); l.set_volume(cur, 2000, 0); }); }
                if (rt.w.fade - 1.0).abs() < 0.001 && (!ride || frame >= 60) {
                    rt.w.warp = Some((target, 0));
                    st.phase = Phase::Gone;
                } else {
                    st.phase = Phase::Leave { frame: frame + 1, target, ride };
                }
            }
        }
        if st.zoom.is_some() { st.zoom_t = (st.zoom_t - 2.25).max(0.0); }
        // the path object plays at rate 0.5 and holds its last frame
        if let Some((i, f)) = st.path {
            let end = st.data.path_anims.iter().find(|a| a.index == i).map_or(0.0, |a| (a.end_frame - 1.0).max(0.0));
            st.path = Some((i, (f + MAP_ANIM_RATE).min(end)));
        }
        ticks += 1;
        let mut done = vec![];
        layer::with(|l| for (i, t) in st.sounds.iter_mut().enumerate() { if t.poll(l).is_some() { done.push(i); } });
        for i in done.into_iter().rev() { st.sounds.remove(i); }
    }
    // the bike sits on the path object (std 166), placed every displayed frame between the 60 Hz ticks
    // (the camera animation advances per displayed frame too)
    if let Some((i, f)) = st.path {
        if let Some(an) = st.data.path_anims.iter().find(|a| a.index == i) {
            let shown = (f - MAP_ANIM_RATE * (1.0 - st.acc.clamp(0.0, 1.0))).max(0.0);
            let srt = an.root_srt(shown, ([1.0; 3], [0.0; 3], [0.0; 3]));
            if let Some(mut t) = st.bike.and_then(|b| tf.get_mut(b).ok()) { *t = crate::model::srt_transform(srt); }
            if std::env::var("COLO_BIKE").is_ok() { println!("BIKE {shown:.2} {:?} {:?}", srt.2, srt.1); }
        }
    }
    // draw only when a tick changed something: the menu while selecting (and while fading out without a
    // ride); during the ride the map capture zooming away, then nothing
    if ticks == 0 || (st.blank && st.capture.is_none() && !matches!(st.phase, Phase::Select | Phase::Leave { ride: false, .. })) { return; }
    match st.phase {
        Phase::Select | Phase::Leave { ride: false, .. } => draw_menu(st, rt, &game.ui),
        _ => {
            st.canvas.clear();
            if let Some(src) = st.capture.take() {
                // wmapRideOverlay quad 2: alpha 1 → 0, scale 1 → 2, centre (320, 240) → (640 − (9 + x), 480 − (9 + y))
                let f = cos_deg(st.overlay_t);
                let to = Vec2::new(640.0 - (9.0 + st.to.x), 480.0 - (9.0 + st.to.y));
                draw_capture(&mut st.canvas, &src, 1.0 + f, Vec2::new(320.0, 240.0).lerp(to, f), 1.0 - f);
                if st.overlay_t > 0.0 { st.capture = Some(src); }
            }
        }
    }
    st.blank = st.capture.is_none() && !matches!(st.phase, Phase::Select | Phase::Leave { ride: false, .. });
    if let Some(mut img) = assets.image_mut(&image.0) { img.data = Some(st.canvas.to_rgba8()); }
}

#[derive(Component)]
pub struct ZoomCam;
#[derive(Component)]
pub struct ZoomImage;

/// `wmapRideOverlay` quad 1 (after `camera_follow`): the zoom camera copies the field camera, and its image is
/// drawn at alpha 1 − cos(t), scale 3 − 2·cos(t) about the screen centre; gone once t reaches 0.
#[allow(clippy::type_complexity)]
pub fn zoom_sync(mut commands: Commands, mut wmap: ResMut<Wmap>,
                 main: Query<(&Transform, &Projection), (With<crate::GameCam>, Without<ZoomCam>)>,
                 mut zc: Query<(&mut Transform, &mut Projection), With<ZoomCam>>, mut zi: Query<(&mut ImageNode, &mut UiTransform), With<ZoomImage>>) {
    let Some(st) = wmap.st.as_mut() else { return };
    let Some((cam, frame)) = st.zoom else { return };
    if st.zoom_t <= 0.0 {
        commands.entity(cam).despawn();
        commands.entity(frame).despawn();
        st.zoom = None;
        return;
    }
    let f = cos_deg(st.zoom_t);
    if let (Some((t, p)), Ok((mut zt, mut zp))) = (main.single().ok(), zc.single_mut()) { *zt = *t; *zp = p.clone(); }
    if let Ok((mut img, mut ut)) = zi.single_mut() {
        img.color = Color::srgba(1.0, 1.0, 1.0, 1.0 - f);
        *ut = UiTransform::from_scale(Vec2::splat(3.0 - 2.0 * f));
    }
}
