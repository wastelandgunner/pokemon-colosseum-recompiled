#![allow(clippy::needless_range_loop)]
//! The field message window and the YES/NO window, drawn like the game into a 640x480 2D layer
//! (`spriteSetEnv` 0x800FE4D4: 640x480 screen plane, y down) which is shown over the 3D view in a 4:3 frame.
//!
//! Field window (menu 0x40, item 0x66 at (53,303) 535x121, `winMsgDraw` 0x801058CC): origin
//! (53, 303 + 121 − 4 − 121) = (53, 299). Background `fn_8001EC08(0, 2, 535, 121, alpha, dim=1)`: black at
//! window alpha / 2, a rounded rectangle with 15 px corners (8 arc samples each) filled by a 34-vertex fan and
//! a 15 px soft edge (66-vertex strip from alpha A to 0 at radius 30). Opens with sequence 0x26 (alpha 0 →
//! 255 over 2 ticks), closes with 0x2A (→ 0 over 5 ticks, text hidden at once).
//!
//! Text (`GSmsgExec` 0x800FBF74, renderer 0x800FC7E0): the game's bitmap font 0 (22 px) from main.dol, white,
//! one character per tick, no drop shadow (work[2] = 0; only control 0x52 sets it). Space advances
//! cell_w / 2; a line break moves down cell_h + 11. A page runs to control 0x03 (`msgctrlKeyWait`), which
//! shows the next-page arrow (sprite 0xB9, 18x19 at (pen_x + 2, pen_y + 8 + trunc(4 cos(π t / 25))), t a
//! global tick counter mod 50) and continues on a cleared window; 0x02 (`msgctrlKeyEnd`) ends without one.
//! `‱` (U+2031) is the speech-bubble glyph of the font.
//!
//! YES/NO (std 11 `dialogDisplayYesNoMenu` → menu 0x12 at (50,200)): item 0x8E black panel (sprite 0x155
//! stretched to 121x66, alpha 179) at (18,10), labels "YES" (msg 0x66) at (63,17) and "NO" (msg 0x67) at
//! (63,46) in font 1 with a (+1,+1) black shadow at alpha 192, the Poké Ball cursor (sprite 0x157, dx −35)
//! beside the selected item, and the frame (sprite 0x156, 151x83) at (2,2) on top.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use colo_formats::{dol::Dol, font::{self, Font}, gtx};
use std::collections::HashMap;

pub const SCREEN_W: usize = 640;
pub const SCREEN_H: usize = 480;

/// Field window origin and size (menu 0x40 / item 0x66).
const WIN_X: i32 = 53;
const WIN_Y: i32 = 299;
const WIN_W: i32 = 535;
const WIN_H: i32 = 121;
/// YES/NO menu 0x12 position.
const YN_X: i32 = 50;
const YN_Y: i32 = 200;

/// Art and fonts from main.dol.
pub struct UiArt {
    /// Font chains by id, in registration order (first match wins).
    pub fonts: HashMap<u16, Vec<Font>>,
    /// Message window atlas (resource 0x031A1200, main.dol 0x803156E0).
    pub msg_atlas: gtx::Image,
    /// YES/NO atlas (resource 0x06221200, main.dol 0x803254E0).
    pub yesno_atlas: gtx::Image,
}

impl UiArt {
    pub fn load(dol: &Dol) -> colo_formats::Result<Self> {
        let mut fonts: HashMap<u16, Vec<Font>> = HashMap::new();
        // GSmsgFontOpen registration order
        for base in [0x8027A500u32, 0x802BD260, 0x802C0CB0] {
            for f in font::parse_chain(dol.read_rest(base)?)? { fonts.entry(f.id).or_default().push(f); }
        }
        Ok(UiArt {
            fonts,
            msg_atlas: gtx::decode(dol.read_rest(0x803156E0)?)?,
            yesno_atlas: gtx::decode(dol.read_rest(0x803254E0)?)?,
        })
    }
    fn glyph(&self, font: u16, c: char) -> Option<&font::Glyph> {
        let code = u16::try_from(c as u32).ok()?;
        self.fonts.get(&font)?.iter().find_map(|f| f.glyph(code))
    }
    fn cell(&self, font: u16) -> (u8, u8) {
        self.fonts.get(&font).and_then(|v| v.first()).map_or((22, 22), |f| (f.cell_w, f.cell_h))
    }
}

/// One element of a page.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Char(char),
    Newline,
    Font(u16),
    Color([u8; 4]),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Page {
    pub items: Vec<Item>,
    /// Ends in a key wait (0x03): the next-page arrow shows once it is typed out.
    pub arrow: bool,
}

impl Page {
    /// Characters the typewriter reveals.
    pub fn chars(&self) -> usize { self.items.iter().filter(|i| matches!(i, Item::Char(_))).count() }
    pub fn text(&self) -> String {
        self.items.iter().filter_map(|i| match i { Item::Char(c) => Some(*c), Item::Newline => Some('\n'), _ => None }).collect()
    }
}

/// Default names for the name controls (the save's names aren't read yet).
const HERO_NAME: &str = "WES";
const PARTNER_NAME: &str = "RUI";

/// Split a rendered message (`msg::Message::render`: `{name:args}` controls, `\n` line breaks) into pages.
pub fn pages(text: &str) -> Vec<Page> {
    let mut out = vec![];
    let mut cur = Page::default();
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        match c {
            '\n' => cur.items.push(Item::Newline),
            '{' => {
                let mut tag = String::new();
                for d in chars.by_ref() { if d == '}' { break; } tag.push(d); }
                let (name, args) = tag.split_once(':').unwrap_or((&tag, ""));
                let args: Vec<u8> = args.split(' ').filter_map(|a| u8::from_str_radix(a, 16).ok()).collect();
                match name {
                    "clear" => { cur.arrow = true; out.push(std::mem::take(&mut cur)); }
                    "end" => { out.push(std::mem::take(&mut cur)); }
                    "font" => cur.items.push(Item::Font(*args.first().unwrap_or(&0) as u16)),
                    "rgba" if args.len() == 4 => cur.items.push(Item::Color([args[0], args[1], args[2], args[3]])),
                    // msgctrlNpc: the speaker's name message; unnamed people use the speech-bubble glyph
                    "speaker" => cur.items.push(Item::Char('\u{2031}')),
                    "player" => cur.items.extend(HERO_NAME.chars().map(Item::Char)),
                    "rui" => cur.items.extend(PARTNER_NAME.chars().map(Item::Char)),
                    _ => {}
                }
            }
            c => cur.items.push(Item::Char(c)),
        }
    }
    if !cur.items.is_empty() { out.push(cur); }
    // drop empty trailing pages (e.g. "{clear}{end}")
    while out.len() > 1 && out.last().is_some_and(|p| p.chars() == 0) { out.pop(); }
    out
}

/// A 640x480 RGBA float canvas with straight-alpha "over" blending.
pub struct Canvas {
    pub px: Vec<[f32; 4]>,
}

impl Default for Canvas {
    fn default() -> Self { Canvas { px: vec![[0.0; 4]; SCREEN_W * SCREEN_H] } }
}

impl Canvas {
    pub fn clear(&mut self) { self.px.fill([0.0; 4]); }

    pub fn blend(&mut self, x: i32, y: i32, c: [f32; 4]) {
        if x < 0 || y < 0 || x >= SCREEN_W as i32 || y >= SCREEN_H as i32 || c[3] <= 0.0 { return; }
        let d = &mut self.px[y as usize * SCREEN_W + x as usize];
        let a = c[3] + d[3] * (1.0 - c[3]);
        if a <= 0.0 { return; }
        for k in 0..3 { d[k] = (c[k] * c[3] + d[k] * d[3] * (1.0 - c[3])) / a; }
        d[3] = a;
    }

    /// Gouraud-shaded triangle (pixel centres, top-left fill rule), vertices (x, y, rgba 0..1).
    pub fn triangle(&mut self, v: [(f32, f32, [f32; 4]); 3]) {
        let [a, b, c] = v;
        let area = (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
        if area.abs() < 1e-6 { return; }
        let (b, c) = if area < 0.0 { (c, b) } else { (b, c) };
        let area = area.abs();
        let x0 = a.0.min(b.0).min(c.0).floor().max(0.0) as i32;
        let x1 = a.0.max(b.0).max(c.0).ceil().min(SCREEN_W as f32) as i32;
        let y0 = a.1.min(b.1).min(c.1).floor().max(0.0) as i32;
        let y1 = a.1.max(b.1).max(c.1).ceil().min(SCREEN_H as f32) as i32;
        let edge = |p: (f32, f32), q: (f32, f32), x: f32, y: f32| (q.0 - p.0) * (y - p.1) - (q.1 - p.1) * (x - p.0);
        // top-left rule: include pixels exactly on a top or left edge
        let tl = |p: (f32, f32), q: (f32, f32)| (q.1 == p.1 && q.0 < p.0) || q.1 < p.1;
        let (ea, eb, ec) = (tl((b.0, b.1), (c.0, c.1)), tl((c.0, c.1), (a.0, a.1)), tl((a.0, a.1), (b.0, b.1)));
        for y in y0..y1 {
            for x in x0..x1 {
                let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                let w0 = edge((b.0, b.1), (c.0, c.1), px, py);
                let w1 = edge((c.0, c.1), (a.0, a.1), px, py);
                let w2 = edge((a.0, a.1), (b.0, b.1), px, py);
                let inside = |w: f32, t: bool| w > 0.0 || (w == 0.0 && t);
                if !(inside(w0, ea) && inside(w1, eb) && inside(w2, ec)) { continue; }
                let (l0, l1, l2) = (w0 / area, w1 / area, w2 / area);
                let mut col = [0.0; 4];
                for k in 0..4 { col[k] = a.2[k] * l0 + b.2[k] * l1 + c.2[k] * l2; }
                self.blend(x, y, col);
            }
        }
    }

    /// Draw a sprite cut from `tex` (crop x, y, w, h) at `at` with `size` (`winSpriteDrawTexture`
    /// 0x80108580: 1:1 samples the crop exactly; stretched draws inset the UVs one texel per side and filter
    /// bilinearly). `alpha` multiplies the texture alpha.
    pub fn sprite(&mut self, tex: &gtx::Image, crop: (i32, i32, i32, i32), at: (i32, i32), size: (i32, i32), alpha: f32) {
        self.sprite_flip(tex, crop, at, size, alpha, (false, false));
    }

    /// [`Canvas::sprite`] mirrored in place (a menu item with a negative width / height covers the same
    /// rectangle with the texture flipped).
    pub fn sprite_flip(&mut self, tex: &gtx::Image, crop: (i32, i32, i32, i32), at: (i32, i32), size: (i32, i32), alpha: f32, flip: (bool, bool)) {
        let (cx, cy, cw, ch) = crop;
        let texel = |x: i32, y: i32| -> [f32; 4] {
            let x = x.clamp(0, tex.width as i32 - 1) as usize;
            let y = y.clamp(0, tex.height as i32 - 1) as usize;
            let p = &tex.rgba[(y * tex.width + x) * 4..][..4];
            [p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0, p[3] as f32 / 255.0]
        };
        let exact = size == (cw, ch);
        for y in 0..size.1 {
            for x in 0..size.0 {
                let mut c = if exact { texel(cx + x, cy + y) } else {
                    // inset crop [cx+1, cx+cw-1] mapped across the quad
                    let u = (cx + 1) as f32 + (x as f32 + 0.5) / size.0 as f32 * (cw - 2) as f32 - 0.5;
                    let v = (cy + 1) as f32 + (y as f32 + 0.5) / size.1 as f32 * (ch - 2) as f32 - 0.5;
                    let (u0, v0) = (u.floor(), v.floor());
                    let (fu, fv) = (u - u0, v - v0);
                    let (iu, iv) = (u0 as i32, v0 as i32);
                    let (t00, t10, t01, t11) = (texel(iu, iv), texel(iu + 1, iv), texel(iu, iv + 1), texel(iu + 1, iv + 1));
                    let mut o = [0.0; 4];
                    for k in 0..4 { o[k] = (t00[k] * (1.0 - fu) + t10[k] * fu) * (1.0 - fv) + (t01[k] * (1.0 - fu) + t11[k] * fu) * fv; }
                    o
                };
                c[3] *= alpha;
                let dx = if flip.0 { size.0 - 1 - x } else { x };
                let dy = if flip.1 { size.1 - 1 - y } else { y };
                self.blend(at.0 + dx, at.1 + dy, c);
            }
        }
    }

    fn glyph(&mut self, g: &font::Glyph, x: i32, y: i32, color: [f32; 4]) {
        for gy in 0..g.rows as i32 {
            for gx in 0..g.width as i32 {
                let cov = g.pixels[(gy * g.width as i32 + gx) as usize] as f32 / 15.0;
                if cov > 0.0 { self.blend(x + gx, y + g.y_offset as i32 + gy, [color[0], color[1], color[2], color[3] * cov]); }
            }
        }
    }

    /// RGBA8 for display. The GameCube blends in gamma (sRGB) space while Bevy's UI blends in linear
    /// space, so the alpha of dark pixels is raised to give the same result over any background (exact for
    /// black: 1 − (1 − a)^2.2); bright pixels (text) keep theirs.
    pub fn to_rgba8(&self) -> Vec<u8> {
        self.px.iter().flat_map(|p| {
            let lum = (0.299 * p[0] + 0.587 * p[1] + 0.114 * p[2]).clamp(0.0, 1.0);
            let dark = 1.0 - (1.0 - p[3].clamp(0.0, 1.0)).powf(2.2);
            let a = dark + (p[3] - dark) * lum;
            [p[0], p[1], p[2], a].map(|v| (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8)
        }).collect()
    }
}

/// Lay out and draw `items` (up to `typed` characters) from (x0, y0) in `font`; returns the pen after the
/// last drawn character (for the next-page arrow). `shadow`: the menu-label drop shadow (+1,+1, black,
/// alpha 192/255).
#[allow(clippy::too_many_arguments)]
pub fn draw_text(cv: &mut Canvas, art: &UiArt, items: &[Item], typed: usize, x0: i32, y0: i32, font: u16, shadow: bool) -> (i32, i32) {
    let mut font = font;
    let (mut x, mut y) = (x0, y0);
    let mut color = [1.0f32; 4];
    // line spacing: 11 for font 0, 6 for font 1, else 0.5·cell_h + 1 (GSmsgExec)
    let linespace = |f: u16, ch: u8| match f { 0 => 11, 1 => 6, _ => (0.5 * ch as f32 + 1.0) as i32 };
    let mut shown = 0;
    for it in items {
        let (cw, ch) = art.cell(font);
        match it {
            Item::Font(f) => font = *f,
            Item::Color(c) => color = c.map(|v| v as f32 / 255.0),
            Item::Newline => {
                if shown >= typed { break; }
                x = x0;
                y += ch as i32 + linespace(font, ch);
            }
            Item::Char(c) => {
                if shown >= typed { break; }
                shown += 1;
                if *c == ' ' { x += (cw >> 1) as i32; continue; }
                match art.glyph(font, *c) {
                    Some(g) => {
                        if shadow { cv.glyph(g, x + 1, y + 1, [0.0, 0.0, 0.0, color[3] * 192.0 / 255.0]); }
                        cv.glyph(g, x, y, color);
                        x += g.width as i32;
                    }
                    None => x += cw as i32,
                }
            }
        }
    }
    (x, y)
}

/// The field window background (`fn_8001EC08`, verified against the asm): `alpha` 0..=255 is the window
/// alpha (halved by the dim flag).
pub fn draw_field_window(cv: &mut Canvas, alpha: f32) {
    let a = (alpha / 2.0).floor() / 255.0;
    let (ox, oy, w, h) = (WIN_X as f32, WIN_Y as f32, WIN_W as f32, WIN_H as f32);
    let corners = [(w, h + 4.0), (w, 2.0), (0.0, 2.0), (0.0, h + 4.0)];
    let mut inner = vec![];
    let mut outer = vec![];
    for i in 0..32 {
        let ang = std::f64::consts::TAU * i as f64 / 32.0;
        let (s, c) = ang.sin_cos();
        let (cx, cy) = corners[i / 8];
        inner.push((ox + cx + (15.0 * s).trunc() as f32, oy + cy + (2.0 + 15.0 * c).trunc() as f32));
        outer.push((ox + cx + (30.0 * s).trunc() as f32, oy + cy + (2.0 + 30.0 * c).trunc() as f32));
    }
    let col = |al: f32| [0.0, 0.0, 0.0, al];
    // 66-vertex strip interleaving outer / inner, closed back to point 0
    let mut strip = vec![];
    for i in 0..=32 { let k = i % 32; strip.push((outer[k].0, outer[k].1, col(0.0))); strip.push((inner[k].0, inner[k].1, col(a))); }
    for t in 0..strip.len() - 2 { cv.triangle([strip[t], strip[t + 1], strip[t + 2]]); }
    // 34-vertex fan from the centre through the inner points
    let centre = (ox + w / 2.0, oy + 2.0 + h / 2.0, col(a));
    for i in 0..32 {
        let (p, q) = (inner[i], inner[(i + 1) % 32]);
        cv.triangle([centre, (p.0, p.1, col(a)), (q.0, q.1, col(a))]);
    }
}

/// Everything the 2D layer shows this frame.
pub struct Frame<'a> {
    pub page: Option<&'a Page>,
    pub typed: usize,
    pub window_alpha: f32,
    /// Draw the page text (window state 2 only; hidden while opening/closing in the game, see `winMsgDraw`).
    pub text: bool,
    /// Show the next-page arrow (page typed out and it ends in a key wait).
    pub arrow: bool,
    /// Global message tick (mod 50) for the arrow bob.
    pub arrow_t: u32,
    /// YES/NO cursor (0 = YES) when the menu is open.
    pub yes_no: Option<usize>,
    pub yes_label: &'a [Item],
    pub no_label: &'a [Item],
}

pub fn render(cv: &mut Canvas, art: &UiArt, f: &Frame) {
    cv.clear();
    if let Some(page) = f.page {
        draw_field_window(cv, f.window_alpha);
        if !f.text { return; }
        let (px, py) = draw_text(cv, art, &page.items, f.typed, WIN_X, WIN_Y, 0, false);
        if f.arrow {
            let bob = (4.0 * (std::f64::consts::PI * f.arrow_t as f64 / 25.0).cos()).trunc() as i32;
            let (ax, ay) = ((px as f32 + 2.0) as i32, (py as f32 + 8.0) as i32 + bob);
            cv.sprite(&art.msg_atlas, (32, 63, 18, 19), (ax, ay), (18, 19), 1.0);
        }
    }
    if let Some(sel) = f.yes_no {
        // item 0x8E: black panel, sprite 0x155 crop (155,28,29,29) stretched, item alpha 179
        cv.sprite(&art.yesno_atlas, (155, 28, 29, 29), (YN_X + 18, YN_Y + 10), (121, 66), 179.0 / 255.0);
        for (i, (label, iy)) in [(f.yes_label, 17), (f.no_label, 46)].into_iter().enumerate() {
            draw_text(cv, art, label, usize::MAX, YN_X + 63, YN_Y + iy, 1, true);
            if i == sel {
                // cursor sprite 0x157 crop (155,1,30,23), dx −35
                cv.sprite(&art.yesno_atlas, (155, 1, 30, 23), (YN_X + 63 - 35, YN_Y + iy), (30, 23), 1.0);
            }
        }
        // item 0x91: frame, sprite 0x156 crop (2,2,151,83) 1:1
        cv.sprite(&art.yesno_atlas, (2, 2, 151, 83), (YN_X + 2, YN_Y + 2), (151, 83), 1.0);
    }
}

/// Marks the 2D layer's image node.
#[derive(Component)]
pub struct Layer2d;

/// The 2D layer: a 640x480 image shown in a centered 4:3 frame over the 3D view.
#[derive(Resource)]
pub struct Layer {
    pub image: Handle<Image>,
    pub canvas: Canvas,
    /// What was last drawn (page text, typed, arrow t, arrow, yes/no, alpha, closing) to skip unchanged frames.
    pub last: Option<(String, usize, u32, bool, Option<usize>, u32, bool)>,
}

pub fn spawn(commands: &mut Commands, images: &mut Assets<Image>) {
    let img = Image::new(Extent3d { width: SCREEN_W as u32, height: SCREEN_H as u32, depth_or_array_layers: 1 }, TextureDimension::D2,
        vec![0u8; SCREEN_W * SCREEN_H * 4], TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    let image = images.add(img);
    commands.spawn((GlobalZIndex(10), Node {
        position_type: PositionType::Absolute, left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0),
        justify_content: JustifyContent::Center, ..default()
    })).with_children(|l| {
        l.spawn((Layer2d, ImageNode::new(image.clone()), GlobalZIndex(10), Node { height: Val::Percent(100.0), aspect_ratio: Some(4.0 / 3.0), ..default() }));
    });
    commands.insert_resource(Layer { image, canvas: Canvas::default(), last: None });
}

/// Plain text of a rendered message (control codes dropped), for logs and menu labels.
pub fn strip_codes(s: &str) -> String {
    pages(s).iter().map(|p| p.text()).collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paging() {
        let p = pages("{speaker}: Yo, we meet again!\nHuh?\nOh, yeah.\nSure.{clear}CAIL: Bye.{end}");
        assert_eq!(p.len(), 2);
        assert!(p[0].arrow && p[0].text().starts_with("\u{2031}: Yo"));
        assert_eq!(p[0].text().lines().count(), 4);
        assert!(!p[1].arrow && p[1].text() == "CAIL: Bye.");
    }
}

#[cfg(test)]
mod raster_tests {
    use super::*;
    #[test]
    fn window_fills() {
        let mut cv = Canvas::default();
        draw_field_window(&mut cv, 255.0);
        let p = cv.px[(WIN_Y as usize + 60) * SCREEN_W + 300];
        assert!((p[3] - 127.0 / 255.0).abs() < 0.01, "{p:?}");
    }
}
