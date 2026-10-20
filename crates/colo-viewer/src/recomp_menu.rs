//! The recompiled game's Escape menu (recomp.rs draws it over the paused game): tabs across the top for the video,
//! audio and controls settings (settings.rs), with Resume and Quit at the right.
//!
//! Keyboard: Up / Down pick a row (past the top or bottom is the tab bar, where Left / Right pick a tab), Left / Right
//! change the value, Enter picks, Tab / Q / E / Page Up / Page Down switch tabs, Escape closes. On a control, Left /
//! Right pick which key, Enter rebinds it (the next key pressed; Escape cancels) and Delete clears it.
//! Mouse: hovering picks, clicking tabs, arrows, buttons and keys does what Enter would (a key: rebind it, right click
//! clears it), sliders follow a click or drag, the wheel steps the value under the pointer.
//!
//! The menu is laid out in window pixels (`panel`) and drawn 1:1 (perf.rs `Panel` / `Overlay`); the same layout's
//! rectangles answer the mouse.

use crate::perf::{text_w, Panel};
use crate::settings::{Action, Present, Settings};
use winit::keyboard::KeyCode;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page { Video, Audio, Controls }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tab { Page(Page), Resume, Quit }

const TABS: [Tab; 5] = [Tab::Page(Page::Video), Tab::Page(Page::Audio), Tab::Page(Page::Controls), Tab::Resume, Tab::Quit];
const PAGES: [Page; 3] = [Page::Video, Page::Audio, Page::Controls];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Row {
    Scale, Fullscreen, Present, Latency, Lead, Aspect, Smooth, XfbYuv, CopyFilter, Overlay, ResetVideo,
    Volume,
    Bind(Action), Deadzone, ResetKeys,
}

/// How a row shows and changes its value.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind { Choice, Slider(u32, u32), Button, Keys }

/// The controls page's two columns of keys: buttons on the left, directions on the right.
const LEFT: [Action; 9] = [Action::A, Action::B, Action::X, Action::Y, Action::Start, Action::Z, Action::L, Action::R, Action::Walk];
const RIGHT: [Action; 12] = [Action::DUp, Action::DDown, Action::DLeft, Action::DRight, Action::StickUp, Action::StickDown,
    Action::StickLeft, Action::StickRight, Action::CUp, Action::CDown, Action::CLeft, Action::CRight];

/// Something the mouse can point at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Hit { Tab(usize), Row(usize), Arrow(usize, i32), Key(usize, usize), Slider(usize, i32, i32) }

pub enum Outcome { Nothing, Close, Quit, Changed }

pub struct Menu {
    page: Page,
    sel: usize,
    /// The tab bar has the keyboard (that tab picked) instead of a row.
    bar: Option<usize>,
    /// Which of a control's two keys is picked.
    col: usize,
    /// The controls column last moved in (where Up from below the columns goes).
    side: u8,
    /// Waiting for the key to bind.
    rebind: Option<(Action, usize)>,
    /// What the mouse is over, the slider being dragged.
    hover: Option<Hit>,
    drag: Option<usize>,
    /// The last layout: the panel's top-left in the window and what is where in it.
    at: (i32, i32),
    hits: Vec<([i32; 4], Hit)>,
    font: ab_glyph::FontRef<'static>,
}

fn rows(p: Page) -> Vec<Row> {
    match p {
        Page::Video => vec![Row::Scale, Row::Fullscreen, Row::Present, Row::Latency, Row::Lead, Row::Aspect, Row::Smooth,
                            Row::XfbYuv, Row::CopyFilter, Row::Overlay, Row::ResetVideo],
        Page::Audio => vec![Row::Volume],
        Page::Controls => LEFT.iter().chain(&RIGHT).map(|a| Row::Bind(*a)).chain([Row::Deadzone, Row::ResetKeys]).collect(),
    }
}

/// Where a row sits: column (0 the full width, 1 / 2 the controls' columns) and line.
fn place(p: Page, i: usize) -> (u8, usize) {
    match p {
        Page::Controls if i < LEFT.len() => (1, i),
        Page::Controls if i < LEFT.len() + RIGHT.len() => (2, i - LEFT.len()),
        Page::Controls => (0, i - LEFT.len()),
        _ => (0, i),
    }
}

fn kind(r: Row) -> Kind {
    match r {
        Row::Lead | Row::Volume => Kind::Slider(0, 100),
        Row::Deadzone => Kind::Slider(0, 60),
        Row::ResetVideo | Row::ResetKeys => Kind::Button,
        Row::Bind(_) => Kind::Keys,
        _ => Kind::Choice,
    }
}

fn label(r: Row) -> &'static str {
    match r {
        Row::Scale => "Internal resolution",
        Row::Fullscreen => "Display",
        Row::Present => "Presentation",
        Row::Latency => "Queued frames (GPU)",
        Row::Lead => "Frame lead",
        Row::Aspect => "Aspect ratio",
        Row::Smooth => "Scaling filter",
        Row::XfbYuv => "XFB colour round trip",
        Row::CopyFilter => "Copy filter (deflicker)",
        Row::Overlay => "Performance overlay",
        Row::ResetVideo => "Reset video settings",
        Row::Volume => "Volume",
        Row::Deadzone => "Gamepad stick dead zone",
        Row::ResetKeys => "Reset controls",
        Row::Bind(a) => a.name(),
    }
}

fn value(r: Row, s: &Settings) -> String {
    match r {
        Row::Scale => format!("{}x  ({} x {})", s.scale, 640 * s.scale, 528 * s.scale),
        Row::Fullscreen => if s.fullscreen { "Fullscreen" } else { "Window" }.into(),
        Row::Present => s.present.name().into(),
        Row::Latency => s.frame_latency.to_string(),
        Row::Lead => format!("{} ms", s.frame_lead_ms),
        Row::Aspect => if s.keep_aspect { "4:3" } else { "Stretch to window" }.into(),
        Row::Smooth => if s.smooth { "Smooth" } else { "Sharp" }.into(),
        Row::XfbYuv => s.xfb_yuv.name().into(),
        Row::CopyFilter => s.copy_filter.name().into(),
        Row::Overlay => if s.perf_overlay { "On" } else { "Off" }.into(),
        Row::Volume => format!("{}%", s.volume),
        Row::Deadzone => format!("{}%", s.deadzone),
        _ => String::new(),
    }
}

/// The widest each row's value gets (so the layout doesn't move as values change).
fn widest(r: Row) -> &'static str {
    match r {
        Row::Scale => "8x  (5120 x 4224)",
        Row::Present => "Immediate (tearing)",
        Row::Aspect => "Stretch to window",
        Row::Fullscreen => "Fullscreen",
        _ => "Smooth",
    }
}

fn hint(r: Row) -> &'static str {
    match r {
        Row::Scale => "The game's render size; 1x is the console's own.",
        Row::Present => "V-sync waits for the screen; Mailbox and Immediate don't (Immediate can tear).",
        Row::Latency => "Frames the GPU may queue: more is smoother, fewer has less lag.",
        Row::Lead => "Slack for slow frames; adds input lag.",
        Row::Smooth => "How the picture is scaled to the window.",
        Row::XfbYuv | Row::CopyFilter => "Auto: on at 1x, off when rendering bigger.",
        Row::Overlay => "Frame rate and frame costs, top-left (F3).",
        Row::Bind(_) => "Click a key (or Enter) and press the new one; right click or Delete clears it.",
        _ => "",
    }
}

fn value_of(r: Row, s: &Settings) -> u32 {
    match r { Row::Lead => s.frame_lead_ms, Row::Volume => s.volume, Row::Deadzone => s.deadzone, _ => 0 }
}

fn set_value(r: Row, v: u32, s: &mut Settings) {
    match r { Row::Lead => s.frame_lead_ms = v, Row::Volume => s.volume = v, Row::Deadzone => s.deadzone = v, _ => {} }
}

/// Step a setting by `d`; false if the row has no value.
fn change(row: Row, d: i32, s: &mut Settings) -> bool {
    let step = |v: u32, lo: u32, hi: u32, by: u32| (v as i32 + d * by as i32).clamp(lo as i32, hi as i32) as u32;
    match row {
        Row::Scale => s.scale = step(s.scale, 1, 8, 1),
        Row::Fullscreen => s.fullscreen = !s.fullscreen,
        Row::Present => {
            let v = [Present::Fifo, Present::Mailbox, Present::Immediate];
            let i = v.iter().position(|p| *p == s.present).unwrap_or(0) as i32;
            s.present = v[(i + d).rem_euclid(3) as usize];
        }
        Row::Latency => s.frame_latency = step(s.frame_latency, 1, 3, 1),
        Row::Lead => s.frame_lead_ms = step(s.frame_lead_ms, 0, 100, 2),
        Row::Aspect => s.keep_aspect = !s.keep_aspect,
        Row::Smooth => s.smooth = !s.smooth,
        Row::XfbYuv => s.xfb_yuv = s.xfb_yuv.next(d),
        Row::CopyFilter => s.copy_filter = s.copy_filter.next(d),
        Row::Overlay => s.perf_overlay = !s.perf_overlay,
        Row::Volume => s.volume = step(s.volume, 0, 100, 5),
        Row::Deadzone => s.deadzone = step(s.deadzone, 0, 60, 5),
        _ => return false,
    }
    true
}

const BG: [u8; 4] = [14, 16, 22, 238];
const TEXT: [u8; 4] = [230, 232, 238, 255];
const DIM: [u8; 4] = [150, 156, 170, 255];
const BOX: [u8; 4] = [44, 48, 60, 255];
const BOX_HOT: [u8; 4] = [62, 68, 86, 255];
const ACCENT: [u8; 4] = [62, 112, 200, 255];
const ACCENT_HOT: [u8; 4] = [84, 136, 226, 255];
const WAIT: [u8; 4] = [196, 140, 40, 255];
const BAND: [u8; 4] = [255, 255, 255, 22];

impl Menu {
    pub fn new() -> Menu {
        Menu { page: Page::Video, sel: 0, bar: None, col: 0, side: 1, rebind: None, hover: None, drag: None, at: (0, 0),
               hits: vec![], font: crate::perf::ui_font() }
    }

    fn rows(&self) -> Vec<Row> { rows(self.page) }

    fn open(&mut self, p: Page) { if self.page != p { (self.page, self.sel, self.col, self.side) = (p, 0, 0, 1); } }

    fn tab_index(&self) -> usize { TABS.iter().position(|t| *t == Tab::Page(self.page)).unwrap() }

    /// Waiting for a key to bind (the window sends it every key, Escape included).
    pub fn rebinding(&self) -> bool { self.rebind.is_some() }

    /// The row Up / Down (`d`) picks from row `i`: the nearest line that way in the same column (the full width
    /// counts as both), or None past the end.
    fn vertical(&self, i: usize, d: i32) -> Option<usize> {
        let (c, l) = place(self.page, i);
        let side = if c == 0 { self.side } else { c };
        (0..self.rows().len()).map(|j| (j, place(self.page, j)))
            .filter(|(_, (cj, lj))| (*cj == 0 || *cj == side) && if d > 0 { *lj > l } else { *lj < l })
            .min_by_key(|(_, (_, lj))| (*lj as i32 - l as i32).abs()).map(|(j, _)| j)
    }

    /// A key pressed while the menu is open.
    pub fn key(&mut self, k: KeyCode, s: &mut Settings) -> Outcome {
        if let Some((a, j)) = self.rebind.take() {
            if k == KeyCode::Escape { return Outcome::Changed; }
            let mut ks = s.keys_of(a);
            ks[j] = Some(k);
            s.set_keys(a, ks);
            return Outcome::Changed;
        }
        let tab = |m: &mut Menu, d: i32| {
            let i = (m.tab_index() as i32 + d).rem_euclid(PAGES.len() as i32) as usize;
            m.open(PAGES[i]);
            if m.bar.is_some() { m.bar = Some(i); }
            Outcome::Changed
        };
        match k {
            KeyCode::Escape => return Outcome::Close,
            KeyCode::Tab | KeyCode::KeyE | KeyCode::PageDown => return tab(self, 1),
            KeyCode::KeyQ | KeyCode::PageUp => return tab(self, -1),
            _ => {}
        }
        let rows = self.rows();
        let n = rows.len();
        self.sel = self.sel.min(n - 1);
        let row = rows[self.sel];
        if let Some(b) = self.bar {
            return match k {
                KeyCode::ArrowLeft | KeyCode::ArrowRight => {
                    let b = (b as i32 + if k == KeyCode::ArrowLeft { -1 } else { 1 }).rem_euclid(TABS.len() as i32) as usize;
                    if let Tab::Page(p) = TABS[b] { self.open(p); }
                    self.bar = Some(b);
                    Outcome::Changed
                }
                KeyCode::ArrowDown => { (self.bar, self.sel) = (None, 0); Outcome::Changed }
                KeyCode::ArrowUp => { (self.bar, self.sel) = (None, (0..n).max_by_key(|j| place(self.page, *j).1).unwrap()); Outcome::Changed }
                KeyCode::Enter | KeyCode::NumpadEnter | KeyCode::Space => self.pick_tab(b),
                _ => Outcome::Nothing,
            };
        }
        match k {
            KeyCode::ArrowUp | KeyCode::ArrowDown => {
                match self.vertical(self.sel, if k == KeyCode::ArrowUp { -1 } else { 1 }) {
                    Some(j) => { self.sel = j; if place(self.page, j).0 != 0 { self.side = place(self.page, j).0; } }
                    None => self.bar = Some(self.tab_index()),
                }
                Outcome::Changed
            }
            KeyCode::ArrowLeft | KeyCode::ArrowRight => {
                let d = if k == KeyCode::ArrowLeft { -1 } else { 1 };
                if let Row::Bind(_) = row {
                    // four keys across the two columns
                    let (c, l) = place(self.page, self.sel);
                    let cell = (c as i32 - 1) * 2 + self.col as i32 + d;
                    if !(0..4).contains(&cell) { return Outcome::Nothing; }
                    let (c2, col) = (1 + cell as u8 / 2, cell as usize % 2);
                    if c2 != c {
                        let side = if c2 == 1 { 0..LEFT.len() } else { LEFT.len()..LEFT.len() + RIGHT.len() };
                        self.sel = side.min_by_key(|j| (place(self.page, *j).1 as i32 - l as i32).abs()).unwrap();
                        self.side = c2;
                    }
                    self.col = col;
                    return Outcome::Changed;
                }
                if change(row, d, s) { Outcome::Changed } else { Outcome::Nothing }
            }
            KeyCode::Delete | KeyCode::Backspace => {
                if let Row::Bind(a) = row { let mut ks = s.keys_of(a); ks[self.col] = None; s.set_keys(a, ks); return Outcome::Changed; }
                Outcome::Nothing
            }
            KeyCode::Enter | KeyCode::NumpadEnter | KeyCode::Space => self.activate(row, s),
            _ => Outcome::Nothing,
        }
    }

    /// Enter on a row (or a click on it).
    fn activate(&mut self, row: Row, s: &mut Settings) -> Outcome {
        match row {
            Row::Bind(a) => { self.rebind = Some((a, self.col)); Outcome::Changed }
            Row::ResetKeys => { let d = Settings::default(); s.keys = d.keys; s.deadzone = d.deadzone; Outcome::Changed }
            Row::ResetVideo => {
                let d = Settings::default();
                (s.scale, s.fullscreen, s.present, s.frame_latency, s.frame_lead_ms, s.keep_aspect, s.smooth, s.xfb_yuv, s.copy_filter, s.perf_overlay) =
                    (d.scale, d.fullscreen, d.present, d.frame_latency, d.frame_lead_ms, d.keep_aspect, d.smooth, d.xfb_yuv, d.copy_filter, d.perf_overlay);
                Outcome::Changed
            }
            _ => if change(row, 1, s) { Outcome::Changed } else { Outcome::Nothing },
        }
    }

    fn pick_tab(&mut self, i: usize) -> Outcome {
        match TABS[i] {
            Tab::Page(p) => { self.open(p); Outcome::Changed }
            Tab::Resume => Outcome::Close,
            Tab::Quit => Outcome::Quit,
        }
    }

    fn hit(&self, x: i32, y: i32) -> Option<Hit> {
        let (x, y) = (x - self.at.0, y - self.at.1);
        // the last match: arrows, keys and sliders lie over their row
        self.hits.iter().rev().find(|([rx, ry, rw, rh], _)| x >= *rx && y >= *ry && x < rx + rw && y < ry + rh).map(|(_, h)| *h)
    }

    fn row_of(h: Hit) -> Option<usize> {
        match h { Hit::Row(r) | Hit::Arrow(r, _) | Hit::Key(r, _) | Hit::Slider(r, ..) => Some(r), Hit::Tab(_) => None }
    }

    /// Set slider row `r` from window x.
    fn slide(&self, r: usize, x: i32, s: &mut Settings) -> Outcome {
        let row = self.rows()[r];
        let Kind::Slider(lo, hi) = kind(row) else { return Outcome::Nothing };
        let Some((x0, w)) = self.hits.iter().find_map(|(_, h)| match *h { Hit::Slider(j, x0, w) if j == r => Some((x0, w)), _ => None }) else { return Outcome::Nothing };
        let t = ((x - self.at.0 - x0) as f32 / w.max(1) as f32).clamp(0.0, 1.0);
        let step = if row == Row::Lead { 2 } else { 5 } as f32;
        set_value(row, (((lo as f32 + t * (hi - lo) as f32) / step).round() * step) as u32, s);
        Outcome::Changed
    }

    /// The pointer moved to window pixel `x`, `y`: hover (and pick) what is under it, or drag a slider.
    pub fn mouse_move(&mut self, x: i32, y: i32, s: &mut Settings) -> Outcome {
        if let Some(r) = self.drag { return self.slide(r, x, s); }
        let h = self.hit(x, y);
        let old = (self.hover, self.bar, self.sel, self.col);
        self.hover = h;
        if let Some(r) = h.and_then(Menu::row_of) {
            (self.sel, self.bar) = (r, None);
            if let Some(Hit::Key(_, j)) = h { self.col = j; }
            if place(self.page, r).0 != 0 { self.side = place(self.page, r).0; }
        }
        if (self.hover, self.bar, self.sel, self.col) == old { Outcome::Nothing } else { Outcome::Changed }
    }

    /// A mouse button went down (`pressed`) or up at window pixel `x`, `y`; `left`, else the right button.
    pub fn mouse_button(&mut self, left: bool, pressed: bool, x: i32, y: i32, s: &mut Settings) -> Outcome {
        if !pressed { self.drag = None; return Outcome::Nothing; }
        if self.rebind.take().is_some() { return Outcome::Changed; }
        self.mouse_move(x, y, s);
        let Some(h) = self.hit(x, y) else { return Outcome::Nothing };
        let rows = self.rows();
        if !left {
            let (Hit::Key(r, j), Kind::Keys) = (h, kind(rows[Menu::row_of(h).unwrap_or(0)])) else { return Outcome::Nothing };
            let Row::Bind(a) = rows[r] else { return Outcome::Nothing };
            let mut ks = s.keys_of(a); ks[j] = None; s.set_keys(a, ks);
            return Outcome::Changed;
        }
        match h {
            Hit::Tab(i) => self.pick_tab(i),
            Hit::Arrow(r, d) => if change(rows[r], d, s) { Outcome::Changed } else { Outcome::Nothing },
            Hit::Key(r, j) => { self.col = j; self.activate(rows[r], s) }
            Hit::Slider(r, ..) => { self.drag = Some(r); self.slide(r, x, s) }
            Hit::Row(r) => match kind(rows[r]) { Kind::Keys | Kind::Slider(..) => Outcome::Nothing, _ => self.activate(rows[r], s) },
        }
    }

    /// The wheel turned `d` notches (up is positive) with the pointer at `x`, `y`: step the value under it.
    pub fn wheel(&mut self, d: i32, x: i32, y: i32, s: &mut Settings) -> Outcome {
        let Some(r) = self.hit(x, y).and_then(Menu::row_of) else { return Outcome::Nothing };
        let row = self.rows()[r];
        if d == 0 || !matches!(kind(row), Kind::Choice | Kind::Slider(..)) { return Outcome::Nothing; }
        if change(row, d.signum(), s) { Outcome::Changed } else { Outcome::Nothing }
    }

    /// Lay the menu out for a `tw` x `th` window at `ui_scale` (the window's scale factor): the panel and its top-left
    /// (whole pixels). The text shrinks if it would not fit; the panel's size is the same on every tab.
    pub fn panel(&mut self, s: &Settings, tw: u32, th: u32, ui_scale: f64) -> (Panel, (i32, i32)) {
        let mut px = (20.0 * ui_scale) as f32;
        loop {
            let p = self.layout(s, px);
            if px <= 9.0 || (p.w + 16 <= tw && p.h + 16 <= th) {
                self.at = crate::perf::Overlay::centre(tw, th, p.w, p.h);
                return (p, self.at);
            }
            px = (px * 0.92).max(9.0);
        }
    }

    fn layout(&mut self, s: &Settings, px: f32) -> Panel {
        use ab_glyph::{Font, ScaleFont};
        let f = &self.font;
        let tw = |t: &str| text_w(f, t, px).ceil() as i32;
        let u = |m: f32| (px * m).round() as i32;
        let line_h = { let sf = f.as_scaled(px); (sf.ascent() - sf.descent()).ceil() as i32 };
        let small = (px * 0.8).round();
        let mid = |y: i32, h: i32| y + (h - line_h) / 2;
        let (pad, rh, gap, aw, kg, tab_pad, tab_gap) = (u(1.0), u(1.65), u(1.2), u(1.6), u(0.4), u(0.9), u(0.3));
        let mut rects: Vec<([i32; 4], [u8; 4])> = vec![];
        let mut texts: Vec<(i32, i32, f32, [u8; 4], String)> = vec![];
        let mut hits: Vec<([i32; 4], Hit)> = vec![];

        // column sizes, over every page so tabs don't change the panel's size
        let all: Vec<Row> = PAGES.iter().flat_map(|p| rows(*p)).collect();
        let full = |r: &&Row| !matches!(r, Row::Bind(_));
        let lw = all.iter().filter(full).filter(|r| kind(**r) != Kind::Button).map(|r| tw(label(*r))).max().unwrap();
        let vw = all.iter().filter(full).map(|r| tw(widest(*r))).max().unwrap() + 2 * aw + u(1.0);
        let blw = all.iter().filter(|r| !full(r)).map(|r| tw(label(*r))).max().unwrap();
        // room for most keys' names (a longer one is drawn smaller)
        let kw = tw("ShiftRight").max(tw("press a key")) + u(1.0);
        let colw = blw + u(0.8) + 2 * kw + kg;
        let tab_name = |t: &Tab| match t { Tab::Page(p) => format!("{p:?}"), Tab::Resume => "Resume".into(), Tab::Quit => "Quit".into() };
        let tab_ws: Vec<i32> = TABS.iter().map(|t| tw(&tab_name(t)) + 2 * tab_pad).collect();
        let tabs_w = tab_ws.iter().sum::<i32>() + 4 * tab_gap + gap;
        let hints_w = all.iter().map(|r| text_w(f, hint(*r), small).ceil() as i32).max().unwrap();
        let inner = (lw + gap + vw).max(2 * colw + gap).max(tabs_w).max(hints_w);
        let lines = PAGES.iter().map(|p| (0..rows(*p).len()).map(|i| place(*p, i).1 + 1).max().unwrap()).max().unwrap() as i32;

        // tab bar: pages on the left, Resume / Quit on the right
        let bar_h = u(2.0);
        let mut x = pad;
        for (i, t) in TABS.iter().enumerate() {
            if *t == Tab::Resume { x = pad + inner - tab_ws[3] - tab_ws[4] - tab_gap; }
            let r = [x, pad, tab_ws[i], bar_h];
            let on = *t == Tab::Page(self.page);
            let hot = self.hover == Some(Hit::Tab(i)) || self.bar == Some(i);
            rects.push((r, match (on, hot) { (true, false) => ACCENT, (true, true) => ACCENT_HOT, (false, true) => BOX_HOT, _ => BOX }));
            if self.bar == Some(i) { rects.push(([x, pad + bar_h - u(0.15), tab_ws[i], u(0.15)], TEXT)); }
            texts.push((x + tab_pad, mid(pad, bar_h), px, if on || hot { TEXT } else { DIM }, tab_name(t)));
            hits.push((r, Hit::Tab(i)));
            x += tab_ws[i] + tab_gap;
        }
        rects.push(([pad, pad + bar_h, inner, u(0.1).max(1)], ACCENT));

        // rows
        let top = pad + bar_h + u(0.8);
        let rows = self.rows();
        for (i, row) in rows.iter().enumerate() {
            let (c, l) = place(self.page, i);
            let (x0, ww) = match c { 0 => (pad, inner), 1 => (pad, colw), _ => (pad + colw + gap, colw) };
            let y = top + l as i32 * rh;
            let picked = self.bar.is_none() && self.sel == i;
            let band = [x0 - u(0.4), y, ww + u(0.8), rh];
            if picked { rects.push((band, BAND)); }
            hits.push((band, Hit::Row(i)));
            let (ty, vx) = (mid(y, rh), x0 + lw + gap);
            if kind(*row) != Kind::Button { texts.push((x0, ty, px, TEXT, label(*row).into())); }
            match kind(*row) {
                Kind::Button => {
                    let r = [x0, y + u(0.15), tw(label(*row)) + 2 * tab_pad, rh - u(0.3)];
                    rects.push((r, if picked { BOX_HOT } else { BOX }));
                    texts.push((x0 + tab_pad, mid(r[1], r[3]), px, TEXT, label(*row).into()));
                }
                Kind::Keys => {
                    let Row::Bind(a) = *row else { unreachable!() };
                    let ks = s.keys_of(a);
                    for j in 0..2 {
                        let r = [x0 + colw - (2 - j as i32) * kw - (1 - j as i32) * kg, y + u(0.15), kw, rh - u(0.3)];
                        let waiting = self.rebind == Some((a, j));
                        let bg = if waiting { WAIT } else if picked && self.col == j { ACCENT } else if self.hover == Some(Hit::Key(i, j)) { BOX_HOT } else { BOX };
                        rects.push((r, bg));
                        let t = if waiting { "press a key".into() } else { ks[j].map_or("-".into(), crate::settings::key_label) };
                        let mut kpx = px;
                        while text_w(f, &t, kpx) > (kw - u(0.5)) as f32 && kpx > 8.0 { kpx = (kpx * 0.9).round(); }
                        let (kx, khalf) = ((kw - text_w(f, &t, kpx).ceil() as i32) / 2, { let sf = f.as_scaled(kpx); (sf.ascent() - sf.descent()).ceil() as i32 });
                        texts.push((r[0] + kx, r[1] + (r[3] - khalf) / 2, kpx, if ks[j].is_some() || waiting { TEXT } else { DIM }, t));
                        hits.push((r, Hit::Key(i, j)));
                    }
                }
                Kind::Choice => {
                    for (d, ax, t) in [(-1, vx, "<"), (1, vx + vw - aw, ">")] {
                        let hot = self.hover == Some(Hit::Arrow(i, d));
                        texts.push((ax + (aw - tw(t)) / 2, ty, px, if hot { ACCENT_HOT } else if picked { TEXT } else { DIM }, t.into()));
                        hits.push(([ax, y, aw, rh], Hit::Arrow(i, d)));
                    }
                    let v = value(*row, s);
                    texts.push((vx + (vw - tw(&v)) / 2, ty, px, TEXT, v));
                }
                Kind::Slider(lo, hi) => {
                    let nw = tw("100 ms") + u(0.6);
                    let (sx, sw, bh, kn) = (vx + aw / 2, vw - nw - aw / 2, u(0.3).max(2), u(0.7));
                    let fw = ((value_of(*row, s) - lo) as f32 / (hi - lo) as f32 * sw as f32).round() as i32;
                    let hot = self.drag == Some(i) || matches!(self.hover, Some(Hit::Slider(j, ..)) if j == i);
                    rects.push(([sx, y + (rh - bh) / 2, sw, bh], BOX_HOT));
                    rects.push(([sx, y + (rh - bh) / 2, fw, bh], if hot { ACCENT_HOT } else { ACCENT }));
                    rects.push(([sx + fw - kn / 2, y + (rh - kn) / 2, kn, kn], if hot || picked { TEXT } else { DIM }));
                    texts.push((vx + vw - nw + u(0.6), ty, px, TEXT, value(*row, s)));
                    hits.push(([sx - kn, y, sw + 2 * kn, rh], Hit::Slider(i, sx, sw)));
                }
            }
        }

        // the picked row's (or tab's) hint, and the keys
        let fy = top + lines * rh + u(0.6);
        let hy = fy + u(1.4);
        let h_ = match self.bar.map(|b| TABS[b]) {
            Some(Tab::Resume) => "Back to the game (Escape).",
            Some(Tab::Quit) => "Quit the game (the memory card is saved).",
            Some(_) => "",
            None => hint(rows[self.sel.min(rows.len() - 1)]),
        };
        texts.push((pad, fy, small, DIM, h_.into()));
        let help = if self.rebind.is_some() { "Press the new key   Escape: cancel" } else if self.page == Page::Controls {
            "Arrows: pick   Enter: rebind   Delete: clear   Tab / Q / E: tabs   Esc: resume"
        } else { "Up/Down: pick   Left/Right: change   Tab / Q / E: tabs   Esc: resume" };
        texts.push((pad, hy, small, DIM, help.into()));
        self.hits = hits;
        Panel { w: (inner + 2 * pad) as u32, h: (hy + u(1.2) + pad) as u32, bg: BG, rects, texts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::keyboard::KeyCode as K;

    fn press(m: &mut Menu, s: &mut Settings, keys: &[K]) { for k in keys { m.key(*k, s); } }

    /// The centre of the hit area `h`, in window pixels.
    fn at(m: &Menu, h: Hit) -> (i32, i32) {
        let ([x, y, w, hh], _) = m.hits.iter().find(|(_, x)| *x == h).expect("hit area");
        (m.at.0 + x + w / 2, m.at.1 + y + hh / 2)
    }

    #[test]
    fn keyboard() {
        let (mut m, mut s) = (Menu::new(), Settings::default());
        // Video tab: resolution up, display mode, presentation
        press(&mut m, &mut s, &[K::ArrowRight, K::ArrowDown, K::Enter, K::ArrowDown, K::ArrowLeft]);
        assert_eq!((s.scale, s.fullscreen, s.present), (4, true, Present::Immediate));
        // Up past the top is the tab bar; Left from Video wraps to Quit
        press(&mut m, &mut s, &[K::ArrowUp, K::ArrowUp, K::ArrowUp]);
        assert_eq!(m.bar, Some(0));
        press(&mut m, &mut s, &[K::ArrowLeft]);
        assert!(matches!(m.key(K::Enter, &mut s), Outcome::Quit));
        // Tab / Q / E switch pages
        let mut m = Menu::new();
        press(&mut m, &mut s, &[K::Tab]);
        assert_eq!(m.page, Page::Audio);
        press(&mut m, &mut s, &[K::KeyE, K::KeyE]);
        assert_eq!(m.page, Page::Video);
        press(&mut m, &mut s, &[K::KeyQ]);
        assert_eq!(m.page, Page::Controls);
        // Controls: rebind A's first key to M, clear its second; Escape while rebinding cancels without binding it
        press(&mut m, &mut s, &[K::Enter]);
        assert!(m.rebinding());
        press(&mut m, &mut s, &[K::KeyM, K::ArrowRight, K::Delete]);
        assert_eq!(s.keys_of(Action::A), [Some(K::KeyM), None]);
        press(&mut m, &mut s, &[K::Enter, K::Escape]);
        assert_eq!(s.keys_of(Action::A), [Some(K::KeyM), None]);
        // Right across into the directions column, down it, on to the dead zone below both columns, and back up
        press(&mut m, &mut s, &[K::ArrowRight, K::ArrowDown]);
        assert_eq!((m.rows()[m.sel], m.col), (Row::Bind(Action::DDown), 0));
        press(&mut m, &mut s, &[K::ArrowDown; 11]);
        assert_eq!(m.rows()[m.sel], Row::Deadzone);
        press(&mut m, &mut s, &[K::ArrowUp]);
        assert_eq!(m.rows()[m.sel], Row::Bind(Action::CRight));
        assert!(matches!(m.key(K::Escape, &mut s), Outcome::Close));
    }

    #[test]
    fn mouse() {
        let (mut m, mut s) = (Menu::new(), Settings::default());
        m.panel(&s, 1280, 960, 1.0);
        // hovering a row picks it; its arrows step the value
        let p = at(&m, Hit::Arrow(1, 1));
        assert!(matches!(m.mouse_move(p.0, p.1, &mut s), Outcome::Changed));
        assert_eq!(m.sel, 1);
        let p = at(&m, Hit::Arrow(0, 1));
        m.mouse_button(true, true, p.0, p.1, &mut s);
        assert_eq!(s.scale, 4);
        let p = at(&m, Hit::Arrow(0, -1));
        m.mouse_button(true, true, p.0, p.1, &mut s);
        m.mouse_button(true, true, p.0, p.1, &mut s);
        assert_eq!(s.scale, 2);
        // the wheel over a row steps it
        let p = at(&m, Hit::Row(1));
        m.wheel(1, p.0, p.1, &mut s);
        assert!(s.fullscreen);
        // the Audio tab, then drag the volume slider past both ends
        let p = at(&m, Hit::Tab(1));
        m.mouse_button(true, true, p.0, p.1, &mut s);
        assert_eq!(m.page, Page::Audio);
        m.panel(&s, 1280, 960, 1.0);
        let &([x, y, w, h], _) = m.hits.iter().find(|(_, h)| matches!(h, Hit::Slider(..))).unwrap();
        let (x, y) = (m.at.0 + x, m.at.1 + y + h / 2);
        m.mouse_button(true, true, x + w / 2, y, &mut s);
        m.mouse_move(x - 50, y + 200, &mut s);
        assert_eq!(s.volume, 0);
        m.mouse_move(x + w + 50, y, &mut s);
        assert_eq!(s.volume, 100);
        m.mouse_button(true, false, x, y, &mut s);
        m.mouse_move(x, y, &mut s);
        assert_eq!(s.volume, 100);
        // Controls: click B's second key and press a key; right click clears it; a click cancels a rebind
        let p = at(&m, Hit::Tab(2));
        m.mouse_button(true, true, p.0, p.1, &mut s);
        m.panel(&s, 1280, 960, 1.0);
        let p = at(&m, Hit::Key(1, 1));
        m.mouse_button(true, true, p.0, p.1, &mut s);
        assert_eq!(m.rebind, Some((Action::B, 1)));
        m.key(K::KeyJ, &mut s);
        assert_eq!(s.keys_of(Action::B)[1], Some(K::KeyJ));
        m.mouse_button(false, true, p.0, p.1, &mut s);
        assert_eq!(s.keys_of(Action::B)[1], None);
        m.mouse_button(true, true, p.0, p.1, &mut s);
        m.mouse_button(true, true, 0, 0, &mut s);
        assert!(!m.rebinding());
        // Resume and Quit
        let p = at(&m, Hit::Tab(3));
        assert!(matches!(m.mouse_button(true, true, p.0, p.1, &mut s), Outcome::Close));
        let p = at(&m, Hit::Tab(4));
        assert!(matches!(m.mouse_button(true, true, p.0, p.1, &mut s), Outcome::Quit));
    }

    /// Every page fits the default 960 x 720 window at scale 1 without shrinking the text, at the same size.
    #[test]
    fn fits() {
        let (mut m, s) = (Menu::new(), Settings::default());
        let mut sizes = vec![];
        for p in PAGES {
            m.open(p);
            let (pn, at) = m.panel(&s, 960, 720, 1.0);
            let full = m.layout(&s, 20.0); assert_eq!(full.w, pn.w, "{p:?} shrank from {} x {}", full.w, full.h);
            assert!(at.0 >= 0 && at.1 >= 0);
            sizes.push((pn.w, pn.h));
        }
        assert!(sizes.iter().all(|z| *z == sizes[0]), "{sizes:?}");
    }

    /// Draws the menu over a checkerboard through the GPU and checks it lands 1:1: every pixel is the CPU picture
    /// blended over what was under it. With $MENU_PNG also saves it ($MENU_PAGE: video, audio, controls).
    #[test]
    fn menu_render() {
        let inst = wgpu::Instance::default();
        let Ok(ad) = futures_lite::future::block_on(inst.request_adapter(&Default::default())) else { return };
        let (dev, q) = futures_lite::future::block_on(ad.request_device(&Default::default())).unwrap();
        let (w, h) = (1279u32, 959u32); // odd, so a centred panel would sit on half pixels
        let fmt = wgpu::TextureFormat::Rgba8Unorm;
        let tex = dev.create_texture(&wgpu::TextureDescriptor { label: None, size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: fmt,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST, view_formats: &[] });
        let under: Vec<u8> = (0..w * h).flat_map(|i| if (i % w / 3 + i / w / 3) % 2 == 0 { [200, 60, 40, 255] } else { [40, 160, 220, 255] }).collect();
        q.write_texture(wgpu::TexelCopyTextureInfo { texture: &tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            &under, wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(4 * w), rows_per_image: Some(h) },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 });
        let view = tex.create_view(&Default::default());
        let (mut m, mut s) = (Menu::new(), Settings::default());
        match std::env::var("MENU_PAGE").as_deref() {
            Ok("audio") => m.open(Page::Audio),
            Ok("controls") => { m.open(Page::Controls); press(&mut m, &mut s, &[K::ArrowDown, K::ArrowRight]); }
            _ => press(&mut m, &mut s, &[K::ArrowDown]),
        }
        let (pn, (px, py)) = m.panel(&s, w, h, 1.0);
        let mut ov = crate::perf::Overlay::new();
        ov.prepare(&dev, fmt);
        ov.set_panel(&dev, &q, &pn);
        ov.draw_at(&dev, &q, &view, w, h, (px, py));
        let row = (w * 4).div_ceil(256) * 256;
        let buf = dev.create_buffer(&wgpu::BufferDescriptor { label: None, size: (row * h) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let mut enc = dev.create_command_encoder(&Default::default());
        enc.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(h) } },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 });
        q.submit([enc.finish()]);
        buf.slice(..).map_async(wgpu::MapMode::Read, |_| {});
        dev.poll(wgpu::PollType::wait_indefinitely()).unwrap();
        let got: Vec<u8> = buf.slice(..).get_mapped_range().chunks(row as usize).flat_map(|r| r[..(w * 4) as usize].to_vec()).collect();
        // expected: the premultiplied panel over the checkerboard
        let img = pn.raster(&crate::perf::ui_font(), &mut crate::perf::Hinter::new());
        let mut worst = 0;
        for y in 0..pn.h as i32 { for x in 0..pn.w as i32 {
            let (o, t) = ((4 * ((y + py) * w as i32 + x + px)) as usize, (4 * (y * pn.w as i32 + x)) as usize);
            let a = img[t + 3] as f32 / 255.0;
            for c in 0..3 { worst = worst.max((got[o + c] as i32 - (img[t + c] as f32 + under[o + c] as f32 * (1.0 - a)).round() as i32).abs()); }
        } }
        if let Ok(out) = std::env::var("MENU_PNG") { image::save_buffer(out, &got, w, h, image::ColorType::Rgba8).unwrap(); }
        assert!(worst <= 1, "the panel is not drawn 1:1 (worst channel off by {worst})");
    }
}
