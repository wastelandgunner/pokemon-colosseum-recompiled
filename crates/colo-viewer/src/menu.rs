//! The field menu and the PC menu, the game's only way to save.
//!
//! **Field menu** (`fn_8000D710`, opened from `heroMoveMain` when X, Y or Start is pressed: pad mask 0x1C00):
//! menu 0x41, items POKéMON / P★DA / ITEMS / EXIT (menu item records 0x65, 0xDB0, 0xDB1, 0xDB2; labels are
//! common_rel messages 0x136, 0x138, 0x137, 0x13A), a column of 122x41 buttons at x 484, y 95 + 43n on the
//! 640x480 screen. While it is open the field is blocked and every model animation is paused. POKéMON opens
//! the party screen, P★DA warps to the P★DA menu room (floor 0x392), ITEMS opens the bag. There is no Save:
//! Colosseum only saves at a PC. (With flag 0x8AE set the menu is 0x109: POKéMON / HALT / EXIT.)
//!
//! **PC** (A on a PC's check region → interaction point → common script thread 0x0596000D): the script locks
//! the hero, calls `mapSetReturnMap(floor, previous floor, entry)` with the interaction point's parameters and
//! `mapWarpToMenuRoom(0x391)` (the `pcbox_menu` room). Its menu (`fn_8003842C`, menu 0x1D): "PC" /
//! "Which would you like to access?" (shown in a header panel here; the game's window art isn't decoded) with POKéMON Storage / Item Storage / Save / Exit (items 0x355-0x358,
//! messages 0x1B5B-0x1B5E, on panels at x 42 + 10n, y 179 + 55n, 247x35). Save (`fn_80038A0C`): "Would you
//! like to save your progress?" (0x3C01) Yes/No → if the card already has a save, "Overwrite current save
//! file?" (0x1B60; No asks the first question again) → "Saving to the Memory Card in Slot A..." (0x3C04) →
//! "Your progress has been saved!" (0x3C06) → back to the PC menu. The save's location is the return map, so
//! Continue starts at the entry point in front of the PC. Exit (or B) returns to the return map
//! (`fn_800FF660`), i.e. the room reloads with the hero at that entry.

use bevy::prelude::*;

/// Field menu 0x41 labels in menu order; `menuOpen` returns the index.
pub const FIELD_ITEMS: [u32; 4] = [0x136, 0x138, 0x137, 0x13A];
/// PC menu 0x1D labels.
pub const PC_ITEMS: [u32; 4] = [0x1B5B, 0x1B5C, 0x1B5D, 0x1B5E];
pub const PC_TITLE: u32 = 0x1B59;
pub const PC_PROMPT: u32 = 0x1B5A;
pub const SAVE_PROMPT: u32 = 0x3C01;
pub const OVERWRITE_PROMPT: u32 = 0x1B60;
pub const SAVING: u32 = 0x3C04;
pub const SAVED: u32 = 0x3C06;
pub const SAVE_FAILED: u32 = 0x4444;
/// The PC menu room (`pcbox_menu`).
pub const PC_MENU_ROOM: u32 = 0x391;
/// How long "Saving..." stays up (60 Hz frames); the memory card write takes about this long.
pub const SAVING_FRAMES: u32 = 90;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PcStep {
    /// Picking an option ("Which would you like to access?").
    #[default]
    Choose,
    /// A not-implemented notice is up.
    Notice,
    AskSave,
    AskOverwrite,
    /// "Saving..." on screen for this many frames so far.
    Saving(u32),
    /// "Your progress has been saved!" (or the failure message) waiting for A.
    Saved,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Menu {
    #[default]
    Closed,
    /// `notice`: a not-implemented message for the picked entry is up.
    Field { cursor: usize, notice: bool },
    Pc { cursor: usize, step: PcStep },
}

impl Menu {
    pub fn is_open(&self) -> bool { *self != Menu::Closed }
}

/// Run condition: the field runs (scripts, characters, animations) only while no menu is open.
pub fn world_running(menu: Res<Menu>) -> bool { !menu.is_open() }

/// One frame of menu buttons (keyboard or gamepad).
#[derive(Default, Debug, Clone, Copy)]
pub struct Buttons { pub a: bool, pub b: bool, pub up: bool, pub down: bool, pub menu: bool }

pub fn buttons(keys: &ButtonInput<KeyCode>, pads: &Query<&Gamepad>) -> Buttons {
    let k = |c: &[KeyCode]| c.iter().any(|&c| keys.just_pressed(c));
    let mut b = Buttons {
        a: k(&[KeyCode::KeyF, KeyCode::Enter, KeyCode::Space]),
        b: k(&[KeyCode::Backspace, KeyCode::Escape]),
        up: k(&[KeyCode::KeyW, KeyCode::ArrowUp]),
        down: k(&[KeyCode::KeyS, KeyCode::ArrowDown]),
        menu: k(&[KeyCode::KeyX]),
    };
    for p in pads {
        let j = |c: GamepadButton| p.just_pressed(c);
        b.a |= j(GamepadButton::South);
        b.b |= j(GamepadButton::East);
        b.up |= j(GamepadButton::DPadUp);
        b.down |= j(GamepadButton::DPadDown);
        b.menu |= j(GamepadButton::Start) || j(GamepadButton::North) || j(GamepadButton::West);
    }
    b
}

#[derive(Component)]
pub struct FieldMenuRoot;
#[derive(Component)]
pub struct PcMenuRoot;
/// (which menu: 0 field, 1 PC; item index)
#[derive(Component)]
pub struct MenuItem(pub u8, pub usize);

const SCREEN: Vec2 = Vec2::new(640.0, 480.0);
const PANEL: Color = Color::srgba(0.06, 0.06, 0.08, 0.78);
const SELECTED: Color = Color::srgba(0.16, 0.42, 0.78, 0.92);
const TEXT: Color = Color::srgb(0.75, 0.78, 0.84);

/// A node placed in the game's 640x480 menu coordinates (scaled with the window).
fn at(x: f32, y: f32, w: f32, h: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Percent(x / SCREEN.x * 100.0), top: Val::Percent(y / SCREEN.y * 100.0),
        width: Val::Percent(w / SCREEN.x * 100.0), height: Val::Percent(h / SCREEN.y * 100.0),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center,
        border_radius: BorderRadius::all(Val::Px(10.0)),
        ..default()
    }
}

fn label(font: &Handle<Font>, text: String, size: f32) -> impl Bundle {
    (Text::new(text), TextFont { font: font.clone().into(), font_size: bevy::text::FontSize::Px(size), ..default() }, TextColor(TEXT),
     TextShadow { offset: Vec2::splat(2.0), color: Color::srgba(0.0, 0.0, 0.0, 0.8) })
}

/// A full-window layer holding a centered 4:3 frame (the game's 640x480 screen) that menu nodes sit in.
fn frame_layer(commands: &mut Commands, root: impl Bundle, build: impl FnOnce(&mut bevy::ecs::relationship::RelatedSpawnerCommands<ChildOf>)) {
    commands.spawn((root, Visibility::Hidden, Node {
        position_type: PositionType::Absolute, left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0),
        justify_content: JustifyContent::Center, ..default()
    })).with_children(|l| {
        l.spawn(Node { height: Val::Percent(100.0), aspect_ratio: Some(SCREEN.x / SCREEN.y), ..default() }).with_children(build);
    });
}

/// Build both menus (hidden). `msg` looks up a global message (control codes already stripped).
pub fn spawn(commands: &mut Commands, font: Handle<Font>, msg: impl Fn(u32) -> String) {
    frame_layer(commands, FieldMenuRoot, |r| {
        r.spawn((at(474.0, 85.0, 142.0, 190.0), BackgroundColor(PANEL)));
        for (i, &m) in FIELD_ITEMS.iter().enumerate() {
            r.spawn((MenuItem(0, i), at(484.0, 95.0 + 43.0 * i as f32, 122.0, 41.0), BackgroundColor(Color::NONE)))
                .with_children(|b| { b.spawn(label(&font, msg(m), 22.0)); });
        }
    });
    frame_layer(commands, PcMenuRoot, |r| {
        r.spawn((at(42.0, 112.0, 100.0, 44.0), BackgroundColor(PANEL))).with_children(|b| { b.spawn(label(&font, msg(PC_TITLE), 26.0)); });
        let mut n = at(152.0, 112.0, 446.0, 44.0);
        n.justify_content = JustifyContent::FlexStart;
        n.padding = UiRect::left(Val::Percent(16.0 / SCREEN.x * 100.0));
        r.spawn((n, BackgroundColor(PANEL))).with_children(|b| { b.spawn(label(&font, msg(PC_PROMPT), 22.0)); });
        for (i, &m) in PC_ITEMS.iter().enumerate() {
            let mut n = at(42.0 + 10.0 * i as f32, 179.0 + 55.0 * i as f32, 247.0, 35.0);
            n.justify_content = JustifyContent::FlexStart;
            n.padding = UiRect::left(Val::Percent(16.0 / SCREEN.x * 100.0));
            r.spawn((MenuItem(1, i), n, BackgroundColor(PANEL))).with_children(|b| { b.spawn(label(&font, msg(m), 22.0)); });
        }
    });
}

/// Show the open menu and its cursor.
#[allow(clippy::type_complexity)]
pub fn update(menu: Res<Menu>,
              mut field: Query<&mut Visibility, (With<FieldMenuRoot>, Without<PcMenuRoot>)>,
              mut pc: Query<&mut Visibility, (With<PcMenuRoot>, Without<FieldMenuRoot>)>,
              mut items: Query<(&MenuItem, &mut BackgroundColor, &Children)>,
              mut colors: Query<&mut TextColor>) {
    if !menu.is_changed() { return; }
    let (f, p) = match *menu {
        Menu::Closed => (false, false),
        Menu::Field { .. } => (true, false),
        Menu::Pc { .. } => (false, true),
    };
    let vis = |on| if on { Visibility::Visible } else { Visibility::Hidden };
    if let Ok(mut v) = field.single_mut() { *v = vis(f); }
    if let Ok(mut v) = pc.single_mut() { *v = vis(p); }
    let cur = match *menu { Menu::Field { cursor, .. } => (0, cursor), Menu::Pc { cursor, .. } => (1, cursor), Menu::Closed => (9, 0) };
    for (item, mut bg, children) in &mut items {
        let sel = (item.0, item.1) == cur;
        bg.0 = if sel { SELECTED } else if item.0 == 0 { Color::NONE } else { PANEL };
        for c in children.iter() { if let Ok(mut t) = colors.get_mut(c) { t.0 = if sel { Color::WHITE } else { TEXT }; } }
    }
}
