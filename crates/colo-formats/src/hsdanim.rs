//! HSD joint animation (AnimJoint / AObj / FObj) decoding and sampling.
//!
//! - ModelSet +4: AnimJoint** (NULL-terminated), one tree per animation, mirroring the joint tree.
//! - AnimJoint: child 0, next 4, aobj 8, robj 0xC, flags 0x10
//! - AObj: flags 0, end frame f32 4, fobj 8, joint 0xC
//! - FObj: next 0, data length 4, start frame f32 8, track type u8 0xC (1..3 rot XYZ, 5..7 trans, 8..10 scale),
//!   value format u8 0xD, slope format u8 0xE, data 0x10
//!
//! Keyframe stream: opcode byte = interpolation (low 4 bits: 0 none, 1 constant, 2 linear, 3 spline with zero
//! slope, 4 spline, 5 slope-only, 6 key) + node count packed in bits 4..6 with 7-bit extension bytes; then per
//! node: value, optional slope, and a LEB128 wait in frames. Format byte: top 3 bits type (f32, s16, u16, s8,
//! u8), low 5 bits fraction. The keyframe bytes are little-endian (HSDLib, StarsMmd's addon).

use crate::be::{u32_at, u8_at};
use crate::hsd::Archive;
use crate::{Error, Result};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interp { Constant, Linear, Spline }

#[derive(Debug, Clone, Copy)]
pub struct Key { pub frame: f32, pub value: f32, pub interp: Interp, pub slope_in: f32, pub slope_out: f32 }

#[derive(Debug, Clone)]
pub struct Track {
    /// Index into `Model::joints`.
    pub joint: usize,
    /// HSD_A_J_* (1 rotX, 2 rotY, 3 rotZ, 5 transX, 6 transY, 7 transZ, 8 scaleX, 9 scaleY, 10 scaleZ)
    pub channel: u8,
    pub keys: Vec<Key>,
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub model_set: usize,
    pub index: usize,
    pub end_frame: f32,
    pub tracks: Vec<Track>,
}

impl Track {
    pub fn sample(&self, frame: f32) -> Option<f32> {
        let k = &self.keys;
        if k.is_empty() { return None; }
        if frame <= k[0].frame || k.len() == 1 { return Some(k[0].value); }
        let i = match k.iter().rposition(|x| x.frame <= frame) { Some(i) => i, None => return Some(k[0].value) };
        if i + 1 >= k.len() { return Some(k[i].value); }
        let (a, b) = (&k[i], &k[i + 1]);
        let d = b.frame - a.frame;
        if d <= 0.0 { return Some(b.value); }
        let t = (frame - a.frame) / d;
        Some(match a.interp {
            Interp::Constant => a.value,
            Interp::Linear => a.value + (b.value - a.value) * t,
            Interp::Spline => {
                // cubic Hermite, slopes are per frame
                let (t2, t3) = (t * t, t * t * t);
                let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
                let h10 = t3 - 2.0 * t2 + t;
                let h01 = -2.0 * t3 + 3.0 * t2;
                let h11 = t3 - t2;
                h00 * a.value + h10 * a.slope_out * d + h01 * b.value + h11 * b.slope_in * d
            }
        })
    }
}

fn read_val(ad: &[u8], p: &mut usize, fmt: u8) -> Result<f32> {
    let frac = (fmt & 0x1F) as i32;
    let sc = 1.0 / (1u64 << frac) as f32;
    let need = |n: usize| if *p + n <= ad.len() { Ok(()) } else { Err(Error::Invalid("fobj data truncated".into())) };
    let v = match fmt >> 5 {
        0 => { need(4)?; let v = f32::from_le_bytes([ad[*p], ad[*p + 1], ad[*p + 2], ad[*p + 3]]); *p += 4; v }
        1 => { need(2)?; let v = i16::from_le_bytes([ad[*p], ad[*p + 1]]) as f32 * sc; *p += 2; v }
        2 => { need(2)?; let v = u16::from_le_bytes([ad[*p], ad[*p + 1]]) as f32 * sc; *p += 2; v }
        3 => { need(1)?; let v = ad[*p] as i8 as f32 * sc; *p += 1; v }
        4 => { need(1)?; let v = ad[*p] as f32 * sc; *p += 1; v }
        t => return Err(Error::Invalid(format!("fobj value type {t}"))),
    };
    Ok(v)
}

/// Decode one FObj keyframe stream.
pub fn decode_keys(ad: &[u8], start_frame: f32, value_fmt: u8, slope_fmt: u8) -> Result<Vec<Key>> {
    let mut keys: Vec<Key> = vec![];
    let mut p = 0usize;
    let mut frame = -start_frame.floor();
    let mut pending_slope: Option<f32> = None;
    while p < ad.len() {
        let b0 = ad[p];
        let op = b0 & 0xF;
        let mut count = ((b0 & 0x70) >> 4) as usize;
        let mut shift = 0;
        while ad[p] & 0x80 != 0 {
            p += 1;
            if p >= ad.len() { break; }
            count += ((ad[p] & 0x7F) as usize) << (7 * shift + 3);
            shift += 1;
        }
        p += 1;
        count += 1;
        if op == 0 && b0 == 0 { break; }
        for _ in 0..count {
            if p >= ad.len() { break; }
            match op {
                5 => { // slope only: sets incoming slope of the next key
                    pending_slope = Some(read_val(ad, &mut p, slope_fmt)?);
                    continue;
                }
                0 => continue,
                _ => {}
            }
            let value = read_val(ad, &mut p, value_fmt)?;
            let slope = if op == 4 { read_val(ad, &mut p, slope_fmt)? } else { 0.0 };
            let interp = match op { 1 => Interp::Constant, 2 | 6 => Interp::Linear, _ => Interp::Spline };
            let slope_in = pending_slope.take().unwrap_or(slope);
            keys.push(Key { frame, value, interp, slope_in, slope_out: slope });
            // wait (LEB128)
            let mut wait = 0u32;
            let mut sh = 0;
            while p < ad.len() {
                wait |= ((ad[p] & 0x7F) as u32) << sh;
                sh += 7;
                let more = ad[p] & 0x80 != 0;
                p += 1;
                if !more { break; }
            }
            frame += wait as f32;
        }
    }
    Ok(keys)
}

/// The tracks of one AObj: (end frame, tracks with `joint` = 0).
pub fn read_aobj(arc: &Archive, aobj: usize) -> Result<(f32, Vec<Track>)> {
    let d = arc.data;
    let ptr = |o: usize| -> Result<Option<usize>> { let p = u32_at(d, o, "aobj")? as usize; Ok((p != 0).then_some(p)) };
    let end = f32::from_bits(u32_at(d, aobj + 4, "aobj")?);
    let mut tracks = vec![];
    let mut fo = ptr(aobj + 8)?;
    let mut n = 0;
    while let Some(f) = fo {
        let len = u32_at(d, f + 4, "fobj")? as usize;
        let start = f32::from_bits(u32_at(d, f + 8, "fobj")?);
        let (channel, vf, sf) = (u8_at(d, f + 0xC, "fobj")?, u8_at(d, f + 0xD, "fobj")?, u8_at(d, f + 0xE, "fobj")?);
        if let Some(adp) = ptr(f + 0x10)? {
            let ad = crate::be::bytes_at(d, adp, len, "fobj data")?;
            if let Ok(keys) = decode_keys(ad, start, vf, sf) { if !keys.is_empty() { tracks.push(Track { joint: 0, channel, keys }); } }
        }
        fo = ptr(f)?;
        n += 1;
        if n > 64 { break; }
    }
    Ok((end, tracks))
}

/// A camera animation (`HSD_CameraAnim`: CObj AObj + eye / interest WObj anims).
#[derive(Debug, Clone)]
pub struct CameraAnim {
    pub end_frame: f32,
    /// CObj tracks: 9 roll, 10 fov (degrees), 11 near, 12 far.
    pub camera: Vec<Track>,
    /// WObj position tracks: 5/6/7 = x/y/z.
    pub eye: Vec<Track>,
    pub interest: Vec<Track>,
}

/// A scene camera with its animations (`.cam` files and map `.rdat`s: `scene_data` +4 -> camera set
/// {CObjDesc*, CameraAnim**}).
#[derive(Debug, Clone)]
pub struct Camera {
    pub base: crate::hsd::SceneCamera,
    pub roll: f32,
    pub anims: Vec<CameraAnim>,
}

/// Sampled camera state: eye, interest, roll (radians), vertical fov (degrees), near/far clip, aspect.
#[derive(Debug, Clone, Copy)]
pub struct CameraPose { pub eye: [f32; 3], pub interest: [f32; 3], pub roll: f32, pub fov: f32, pub near: f32, pub far: f32, pub aspect: f32 }

impl Camera {
    pub fn pose(&self, anim: usize, frame: f32) -> CameraPose {
        let mut p = CameraPose { eye: self.base.eye, interest: self.base.interest, roll: self.roll, fov: self.base.fov, near: self.base.near, far: self.base.far, aspect: self.base.aspect };
        let Some(a) = self.anims.get(anim) else { return p };
        let f = frame.min(a.end_frame);
        let pos = |tracks: &[Track], v: &mut [f32; 3]| for t in tracks {
            if let (Some(x), 5..=7) = (t.sample(f), t.channel) { v[(t.channel - 5) as usize] = x; }
            if let (Some(x), 1..=3) = (t.sample(f), t.channel) { v[(t.channel - 1) as usize] = x; }
        };
        pos(&a.eye, &mut p.eye);
        pos(&a.interest, &mut p.interest);
        for t in &a.camera {
            match (t.channel, t.sample(f)) { (9, Some(v)) => p.roll = v, (10, Some(v)) => p.fov = v, (11, Some(v)) => p.near = v, (12, Some(v)) => p.far = v, _ => {} }
        }
        p
    }
}

pub fn load_camera(arc: &Archive) -> Result<Option<Camera>> {
    let d = arc.data;
    let ptr = |o: usize| -> Result<Option<usize>> { let p = u32_at(d, o, "camera")? as usize; Ok((p != 0).then_some(p)) };
    let Some(base) = crate::hsd::scene_camera(arc)? else { return Ok(None) };
    let scene = arc.root("scene_data").ok_or_else(|| Error::NotFound("scene_data".into()))?;
    let set = ptr(scene + 4)?.ok_or_else(|| Error::NotFound("camera set".into()))?;
    let cdesc = ptr(set)?.ok_or_else(|| Error::NotFound("camera desc".into()))?;
    let roll = f32::from_bits(u32_at(d, cdesc + 0x20, "camera")?);
    let mut anims = vec![];
    if let Some(list) = ptr(set + 4)? {
        let mut i = 0;
        while let Some(ca) = ptr(list + 4 * i)? {
            let mut a = CameraAnim { end_frame: 0.0, camera: vec![], eye: vec![], interest: vec![] };
            if let Some(ao) = ptr(ca)? { let (e, t) = read_aobj(arc, ao)?; a.end_frame = a.end_frame.max(e); a.camera = t; }
            for (off, slot) in [(4usize, 0), (8, 1)] {
                if let Some(w) = ptr(ca + off)? {
                    if let Some(ao) = ptr(w)? {
                        let (e, t) = read_aobj(arc, ao)?;
                        a.end_frame = a.end_frame.max(e);
                        if slot == 0 { a.eye = t; } else { a.interest = t; }
                    }
                }
            }
            anims.push(a);
            i += 1;
            if i > 64 { break; }
        }
    }
    Ok(Some(Camera { base, roll, anims }))
}

/// All joint animations of an archive's `scene_data`, mapped onto the model's joints via `joint_index`
/// (joint offset -> index into `Model::joints`, as produced by `hsd::load_model`).
pub fn load_animations(arc: &Archive, joint_index: &HashMap<usize, usize>) -> Result<Vec<Animation>> {
    let d = arc.data;
    let ptr = |o: usize| -> Result<Option<usize>> { let p = u32_at(d, o, "anim")? as usize; Ok((p != 0).then_some(p)) };
    let scene = arc.root("scene_data").ok_or_else(|| Error::NotFound("scene_data".into()))?;
    let mut out = vec![];
    let Some(list) = ptr(scene)? else { return Ok(out) };
    let mut ms_i = 0;
    while let Some(ms) = ptr(list + 4 * ms_i)? {
        let root_joint = ptr(ms)?;
        if let (Some(rj), Some(anims)) = (root_joint, ptr(ms + 4)?) {
            let mut ai = 0;
            while let Some(aj) = ptr(anims + 4 * ai)? {
                let mut anim = Animation { model_set: ms_i, index: ai, end_frame: 0.0, tracks: vec![] };
                // parallel walk of (anim joint, joint)
                let mut stack = vec![(aj, rj)];
                let mut guard = 0;
                while let Some((a, j)) = stack.pop() {
                    guard += 1;
                    if guard > 20_000 { break; }
                    if let Some(&ji) = joint_index.get(&j) {
                        if let Some(ao) = ptr(a + 8)? {
                            anim.end_frame = anim.end_frame.max(f32::from_bits(u32_at(d, ao + 4, "aobj")?));
                            let mut fo = ptr(ao + 8)?;
                            let mut fg = 0;
                            while let Some(f) = fo {
                                let len = u32_at(d, f + 4, "fobj")? as usize;
                                let start = f32::from_bits(u32_at(d, f + 8, "fobj")?);
                                let channel = u8_at(d, f + 0xC, "fobj")?;
                                let vf = u8_at(d, f + 0xD, "fobj")?;
                                let sf = u8_at(d, f + 0xE, "fobj")?;
                                if let Some(adp) = ptr(f + 0x10)? {
                                    let ad = crate::be::bytes_at(d, adp, len, "fobj data")?;
                                    if let Ok(keys) = decode_keys(ad, start, vf, sf) {
                                        if !keys.is_empty() { anim.tracks.push(Track { joint: ji, channel, keys }); }
                                    }
                                }
                                fo = ptr(f)?;
                                fg += 1;
                                if fg > 64 { break; }
                            }
                        }
                    }
                    // siblings/children in lockstep
                    if let (Some(an), Some(jn)) = (ptr(a + 4)?, ptr(j + 0xC)?) { stack.push((an, jn)); }
                    if let (Some(ac), Some(jc)) = (ptr(a)?, ptr(j + 8)?) { stack.push((ac, jc)); }
                }
                out.push(anim);
                ai += 1;
                if ai > 512 { break; }
            }
        }
        ms_i += 1;
        if ms_i > 256 { break; }
    }
    Ok(out)
}

/// Animations of `scene_data` model sets that have no JObj (only an AnimJoint list): `GSmodelLoad` gives such
/// a set one bare part driven by the root AnimJoint's AObj, used as an invisible moving anchor (the world
/// map's bike path, `move_demo` set 0). Returns every such animation with its tracks on joint 0.
pub fn load_jointless_animations(arc: &Archive) -> Result<Vec<Animation>> {
    let d = arc.data;
    let ptr = |o: usize| -> Result<Option<usize>> { let p = u32_at(d, o, "anim")? as usize; Ok((p != 0).then_some(p)) };
    let scene = arc.root("scene_data").ok_or_else(|| Error::NotFound("scene_data".into()))?;
    let mut out = vec![];
    let Some(list) = ptr(scene)? else { return Ok(out) };
    let mut ms_i = 0;
    while let Some(ms) = ptr(list + 4 * ms_i)? {
        if let (None, Some(anims)) = (ptr(ms)?, ptr(ms + 4)?) {
            let mut ai = 0;
            while let Some(aj) = ptr(anims + 4 * ai)? {
                let (end_frame, tracks) = match ptr(aj + 8)? { Some(ao) => read_aobj(arc, ao)?, None => (0.0, vec![]) };
                out.push(Animation { model_set: ms_i, index: ai, end_frame, tracks });
                ai += 1;
                if ai > 512 { break; }
            }
        }
        ms_i += 1;
        if ms_i > 256 { break; }
    }
    Ok(out)
}

impl Animation {
    /// SRT of joint 0 at `frame` from (scale, rotation, translation) `rest` (a bare part's pose).
    pub fn root_srt(&self, frame: f32, rest: ([f32; 3], [f32; 3], [f32; 3])) -> ([f32; 3], [f32; 3], [f32; 3]) {
        let mut e = rest;
        for t in self.tracks.iter().filter(|t| t.joint == 0) {
            let Some(v) = t.sample(frame) else { continue };
            match t.channel {
                1..=3 => e.1[(t.channel - 1) as usize] = v,
                5..=7 => e.2[(t.channel - 5) as usize] = v,
                8..=10 => e.0[(t.channel - 8) as usize] = v,
                _ => {}
            }
        }
        e
    }
}

impl Animation {
    /// Local SRT for every joint of `model` at `frame` (rest pose where no track applies).
    pub fn locals(&self, model: &crate::hsd::Model, frame: f32) -> Vec<([f32; 3], [f32; 3], [f32; 3])> {
        let mut l: Vec<_> = model.joints.iter().map(|j| (j.rest_scale, j.rest_rotation, j.rest_translation)).collect();
        for t in &self.tracks {
            let Some(v) = t.sample(frame) else { continue };
            let e = &mut l[t.joint];
            match t.channel {
                1..=3 => e.1[(t.channel - 1) as usize] = v,
                5..=7 => e.2[(t.channel - 5) as usize] = v,
                8..=10 => e.0[(t.channel - 8) as usize] = v,
                _ => {}
            }
        }
        l
    }
}

/// One TObj's texture animation (`HSD_TexAnim`: next 0, texmap id 4, AObjDesc 8, image table 0xC,
/// TLUT table 0x10, u16 image count 0x14, u16 TLUT count 0x16), matched to the TObj with the same texmap id.
#[derive(Debug, Clone)]
pub struct TexAnim {
    pub tobj: usize,
    pub end_frame: f32,
    /// Channels are `HSD_A_T_*`: 1 TIMG, 2 TRAU, 3 TRAV, 4 SCAU, 5 SCAV, 6..8 ROTX..Z, 9 BLEND, 10 TCLT,
    /// 12..15 KONST RGBA, 16..23 TEV0/TEV1 RGBA (`TObjUpdateFunc`, 0x801BE85C).
    pub tracks: Vec<Track>,
    /// Image table, as indices into [`MatAnims::textures`].
    pub images: Vec<Option<usize>>,
}

/// One MObj's material animation (`HSD_MatAnim` +4 AObjDesc): `HSD_A_M_*` 1..3 ambient RGB, 4..6 diffuse RGB,
/// 7..9 specular RGB, 10 alpha (stored as transparency: alpha = 1 - value), 11..13 PE refs (`MObjUpdateFunc`).
#[derive(Debug, Clone)]
pub struct MobjAnim {
    pub mobj: usize,
    pub end_frame: f32,
    pub tracks: Vec<Track>,
}

/// The material/texture animations of one model set's animation index (`ModelSet` +8 MatAnimJoint**).
#[derive(Debug, Clone)]
pub struct MatAnimation {
    pub model_set: usize,
    pub index: usize,
    pub end_frame: f32,
    pub tex: Vec<TexAnim>,
    pub mat: Vec<MobjAnim>,
}

#[derive(Debug, Clone, Default)]
pub struct MatAnims {
    pub anims: Vec<MatAnimation>,
    pub textures: Vec<crate::hsd::Texture>,
}

impl MatAnims {
    pub fn find(&self, model_set: usize, index: usize) -> Option<usize> {
        self.anims.iter().position(|a| a.model_set == model_set && a.index == index)
    }
}

/// All material/texture animations of an archive's `scene_data`. A MatAnimJoint tree mirrors the joint
/// tree (child 0, next 4, MatAnim list 8); each MatAnim (next 0, AObjDesc 4, TexAnim list 8, RenderAnim 0xC)
/// pairs with the joint's DObj list in order.
pub fn load_mat_animations(arc: &Archive) -> Result<MatAnims> {
    let d = arc.data;
    let ptr = |o: usize| -> Result<Option<usize>> { let p = u32_at(d, o, "matanim")? as usize; Ok((p != 0).then_some(p)) };
    let scene = arc.root("scene_data").ok_or_else(|| Error::NotFound("scene_data".into()))?;
    let mut out = MatAnims::default();
    let mut img_cache: HashMap<(usize, usize), usize> = HashMap::new();
    let Some(list) = ptr(scene)? else { return Ok(out) };
    let mut ms_i = 0;
    while let Some(ms) = ptr(list + 4 * ms_i)? {
        if let (Some(rj), Some(mats)) = (ptr(ms)?, ptr(ms + 8)?) {
            let mut ai = 0;
            while let Some(mj) = ptr(mats + 4 * ai)? {
                let mut anim = MatAnimation { model_set: ms_i, index: ai, end_frame: 0.0, tex: vec![], mat: vec![] };
                let mut stack = vec![(mj, rj)];
                let mut guard = 0;
                while let Some((m, j)) = stack.pop() {
                    guard += 1;
                    if guard > 20_000 { break; }
                    // MatAnim list <-> DObj list
                    let (mut ma, mut dobj) = (ptr(m + 8)?, ptr(j + 0x10)?);
                    let flags = u32_at(d, j + 4, "joint")?;
                    if flags & 0x4020 != 0 { dobj = None; } // particle/spline joints have no DObjs
                    let mut g2 = 0;
                    while let (Some(a), Some(dob)) = (ma, dobj) {
                        if let Some(mobj) = ptr(dob + 8)? {
                            if let Some(ao) = ptr(a + 4)? {
                                let (end, tracks) = read_aobj(arc, ao)?;
                                anim.end_frame = anim.end_frame.max(end);
                                if !tracks.is_empty() { anim.mat.push(MobjAnim { mobj, end_frame: end, tracks }); }
                            }
                            let mut ta = ptr(a + 8)?;
                            let mut g3 = 0;
                            while let Some(t) = ta {
                                let id = u32_at(d, t + 4, "texanim")?;
                                // the MObj's TObj with this texmap id
                                let mut tobj = ptr(mobj + 8)?;
                                let mut g4 = 0;
                                while let Some(to) = tobj {
                                    if u32_at(d, to + 8, "tobj")? == id { break; }
                                    tobj = ptr(to + 4)?;
                                    g4 += 1;
                                    if g4 > 16 { tobj = None; break; }
                                }
                                if let (Some(to), Some(ao)) = (tobj, ptr(t + 8)?) {
                                    let (end, tracks) = read_aobj(arc, ao)?;
                                    anim.end_frame = anim.end_frame.max(end);
                                    let n_img = crate::be::u16_at(d, t + 0x14, "texanim")? as usize;
                                    let n_tlut = crate::be::u16_at(d, t + 0x16, "texanim")? as usize;
                                    let tlut = match (ptr(t + 0x10)?, n_tlut) {
                                        (Some(tt), n) if n > 0 => ptr(tt)?,
                                        _ => ptr(to + 0x50)?,
                                    };
                                    let mut images = vec![];
                                    if let Some(it) = ptr(t + 0xC)? {
                                        for k in 0..n_img.min(256) {
                                            let img = ptr(it + 4 * k)?;
                                            images.push(match img {
                                                Some(img) => {
                                                    let key = (u32_at(d, img, "image")? as usize, tlut.unwrap_or(0));
                                                    match img_cache.get(&key) {
                                                        Some(&i) => Some(i),
                                                        None => match crate::hsd::decode_image(arc, img, tlut) {
                                                            Ok(tex) => {
                                                                out.textures.push(tex);
                                                                img_cache.insert(key, out.textures.len() - 1);
                                                                Some(out.textures.len() - 1)
                                                            }
                                                            Err(_) => None,
                                                        },
                                                    }
                                                }
                                                None => None,
                                            });
                                        }
                                    }
                                    anim.tex.push(TexAnim { tobj: to, end_frame: end, tracks, images });
                                }
                                ta = ptr(t)?;
                                g3 += 1;
                                if g3 > 16 { break; }
                            }
                        }
                        ma = ptr(a)?;
                        dobj = ptr(dob + 4)?;
                        g2 += 1;
                        if g2 > 4096 { break; }
                    }
                    if let (Some(mn), Some(jn)) = (ptr(m + 4)?, ptr(j + 0xC)?) { stack.push((mn, jn)); }
                    if let (Some(mc), Some(jc)) = (ptr(m)?, ptr(j + 8)?) { stack.push((mc, jc)); }
                }
                out.anims.push(anim);
                ai += 1;
                if ai > 512 { break; }
            }
        }
        ms_i += 1;
        if ms_i > 256 { break; }
    }
    Ok(out)
}

/// A light's animation (`HSD_LightAnim`: next +0, AObjDesc +4 (`LObjUpdateFunc` tracks), position WObjAnim +8,
/// interest WObjAnim +0xC; WObj tracks 5/6/7 = x/y/z).
#[derive(Debug, Clone, Default)]
pub struct LightAnim {
    /// `lightGetFrameCount`: 1 + the AObj end frame (the last of LObj, position, interest).
    pub frame_count: f32,
    pub lobj: Vec<Track>,
    pub position: Vec<Track>,
    pub interest: Vec<Track>,
}

impl LightAnim {
    pub fn apply(&self, l: &mut crate::hsd::Light, frame: f32) {
        for t in &self.lobj { if let Some(v) = t.sample(frame) { l.apply_track(t.channel, v); } }
        for (tracks, p) in [(&self.position, &mut l.position), (&self.interest, &mut l.interest)] {
            for t in tracks {
                if let (5..=7, Some(v)) = (t.channel, t.sample(frame)) { p[(t.channel - 5) as usize] = v; }
            }
        }
    }
}

/// Animation 0 of each scene light (`scene_lights` order): what `floorInitMap` plays (GSlightSetAnimIndex 0).
pub fn load_light_anims(arc: &Archive) -> Result<Vec<Option<LightAnim>>> {
    let d = arc.data;
    let ptr = |o: usize| -> Result<Option<usize>> { let p = u32_at(d, o, "light")? as usize; Ok((p != 0).then_some(p)) };
    let mut out = vec![];
    let Some(scene) = arc.root("scene_data") else { return Ok(out) };
    let Some(list) = ptr(scene + 8)? else { return Ok(out) };
    for i in 0.. {
        let Some(set) = ptr(list + 4 * i)? else { break };
        let mut la = match ptr(set + 4)? { Some(anims) => ptr(anims)?, None => None };
        let mut desc = ptr(set)?;
        while let Some(dd) = desc {
            let mut a = None;
            if let Some(x) = la {
                let mut an = LightAnim::default();
                let mut end = None;
                if let Some(ao) = ptr(x + 4)? { let (e, t) = read_aobj(arc, ao)?; an.lobj = t; end = Some(e); }
                for (off, slot) in [(8usize, 0), (0xC, 1)] {
                    if let Some(w) = ptr(x + off)? {
                        if let Some(ao) = ptr(w)? {
                            let (e, t) = read_aobj(arc, ao)?;
                            end = Some(e);
                            if slot == 0 { an.position = t; } else { an.interest = t; }
                        }
                    }
                }
                an.frame_count = 1.0 + end.unwrap_or(0.0);
                a = Some(an);
                la = ptr(x)?;
            }
            out.push(a);
            desc = ptr(dd + 4)?;
        }
    }
    Ok(out)
}
