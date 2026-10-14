//! Disc access and per-room data for the viewer.

use crate::collide::Collision;
use bevy::prelude::*;
use colo_formats::{
    ccd,
    common::{Common, InteractionPoint, Room},
    fsys::{FileType, Fsys},
    gfl,
    gcm::Disc,
    hsd::{self, Model},
    hsdanim::{self, Animation},
    maprel::{MapCharacter, MapRel},
    msg,
    scd::Script,
};
use std::collections::HashMap;
use std::sync::Arc;

pub type Res2<T> = Result<T, Box<dyn std::error::Error>>;

/// A disc image or a directory of extracted disc files.
pub enum Source { Disc(Disc), Dir(std::path::PathBuf) }

impl Source {
    pub fn open(p: &str) -> Res2<Self> {
        let path = std::path::Path::new(p);
        Ok(if path.is_dir() { Source::Dir(path.to_path_buf()) } else { Source::Disc(Disc::open(p)?) })
    }
    /// main.dol: from the disc image, or `main.dol` / `sys/main.dol` / `../sys/main.dol` next to the files.
    pub fn read_dol(&mut self) -> Res2<Vec<u8>> {
        match self {
            Source::Disc(d) => Ok(d.read_dol()?),
            Source::Dir(p) => ["main.dol", "sys/main.dol", "../sys/main.dol"].iter().find_map(|n| std::fs::read(p.join(n)).ok())
                .ok_or_else(|| "main.dol not found next to the extracted files (main.dol, sys/main.dol or ../sys/main.dol)".into()),
        }
    }
    pub fn read(&mut self, name: &str) -> Res2<Vec<u8>> {
        Ok(match self { Source::Disc(d) => d.read(name)?, Source::Dir(p) => std::fs::read(p.join(name))? })
    }
    /// (archive name without `.fsys`, FSYS group id) for every archive.
    fn fsys_groups(&mut self) -> Vec<(String, u32)> {
        let mut out = vec![];
        match self {
            Source::Disc(d) => {
                let files: Vec<_> = d.files.iter().filter(|f| f.path.ends_with(".fsys") && f.size >= 16).cloned().collect();
                for f in files {
                    if let Ok(h) = d.read_at(f.offset, 16) {
                        let name = f.path.rsplit('/').next().unwrap_or(&f.path).trim_end_matches(".fsys").to_string();
                        out.push((name, u32::from_be_bytes([h[8], h[9], h[10], h[11]])));
                    }
                }
            }
            Source::Dir(p) => {
                for e in std::fs::read_dir(&*p).into_iter().flatten().flatten() {
                    let name = e.file_name().to_string_lossy().to_string();
                    let Some(stem) = name.strip_suffix(".fsys") else { continue };
                    use std::io::Read;
                    let mut h = [0u8; 16];
                    if std::fs::File::open(e.path()).and_then(|mut f| f.read_exact(&mut h)).is_ok() {
                        out.push((stem.to_string(), u32::from_be_bytes([h[8], h[9], h[10], h[11]])));
                    }
                }
            }
        }
        out
    }
}

/// Disc-wide data: room table, interaction points, the common script and the player model.
#[derive(Resource)]
pub struct Game {
    /// Shadow Pokémon table flags (registered, state) per index (`shadowPokemonSetState`).
    pub shadow_flags: Arc<Vec<(u16, u16, u16)>>,
    source: Source,
    rooms: Vec<Room>,
    models_table: Vec<colo_formats::common::CharacterModel>,
    group_names: HashMap<u32, String>,
    /// field_common.fsys bytes (shared character models)
    field_common: Vec<u8>,
    pub interaction_points: Vec<InteractionPoint>,
    pub doors: Vec<colo_formats::common::Door>,
    pub common_script: Script,
    pub player_model: Model,
    pub player_anims: Arc<Vec<Animation>>,
    /// Rui (hizuki_b1), the partner who follows Wes.
    pub partner_model: Model,
    pub partner_anims: Arc<Vec<Animation>>,
    pub player_info: Option<colo_formats::common::CharacterModel>,
    pub partner_info: Option<colo_formats::common::CharacterModel>,
    /// Global strings: common_rel's embedded tables (menu labels, PC menu, memory card messages) plus the PC
    /// menu room's `system_tool.msg` (the save prompt). Room tables are layered on top per room.
    pub messages: HashMap<u32, String>,
    /// Message window art and fonts from main.dol.
    pub ui: Arc<crate::dialogue::UiArt>,
    /// `COLO_PKX=<name>`: a Pokémon battle model (`pkx_<name>.fsys`) shown in front of Wes, for checking
    /// materials (bump maps etc.).
    pub test_model: Option<(Model, Arc<Vec<Animation>>)>,
    /// Battle tables and the wipe textures (battle.rs, wipe.rs).
    pub battle: Arc<crate::battle::BattleTables>,
}

fn model_with_anims(bytes: &[u8]) -> Res2<(Model, Vec<Animation>)> {
    let arc = hsd::Archive::parse(bytes)?;
    let mut model = hsd::load_model(&arc)?;
    let anims = hsdanim::load_animations(&arc, &model.joint_index).unwrap_or_default();
    model.mats = Arc::new(hsdanim::load_mat_animations(&arc).unwrap_or_default());
    Ok((model, anims))
}

impl Game {
    pub fn open(path: &str) -> Res2<Self> {
        let mut source = Source::open(path)?;
        let common_fsys = source.read("common.fsys")?;
        let cfs = Fsys::parse(&common_fsys)?;
        let common_bytes = cfs.read(cfs.find("common_rel", FileType::Rel).ok_or("no common_rel")?)?;
        let common = Common::parse(&common_bytes)?;
        let rooms = common.rooms()?;
        let models_table = common.character_models()?;
        let interaction_points = common.interaction_points()?;
        let doors = common.doors()?;
        let common_script = common.script()?;
        let shadow_flags = Arc::new(common.shadow_flags().unwrap_or_default());
        crate::audio::boot(&mut source, &common, &cfs);
        let dol = colo_formats::dol::Dol::parse(source.read_dol()?)?;
        let ui = Arc::new(crate::dialogue::UiArt::load(&dol)?);
        let mut messages: HashMap<u32, String> = common.messages().iter().map(|m| (m.id, m.render())).collect();
        // pcbox_menu.fsys is the PC menu room (floor 0x391); its system_tool.msg has the save prompt (0x3c01)
        match source.read("pcbox_menu.fsys").map_err(|e| e.to_string()).and_then(|b| {
            let fs = Fsys::parse(&b).map_err(|e| e.to_string())?;
            let mut n = 0;
            for e in fs.find_type(FileType::Msg) {
                for m in msg::parse_table(&fs.read(e).map_err(|e| e.to_string())?).map_err(|e| e.to_string())? { messages.entry(m.id).or_insert_with(|| m.render()); n += 1; }
            }
            Ok(n)
        }) {
            Ok(n) => println!("global messages: {} (+{n} from pcbox_menu)", messages.len()),
            Err(e) => println!("pcbox_menu.fsys: {e} (save prompt text missing)"),
        }
        let field_common = source.read("field_common.fsys")?;
        // Wes and Rui, plus their CharacterModels entries (found by FSYS identifier) for slots and strides
        let lookup = |id: u32| models_table.iter().find(|m| m.fsys_identifier & 0xFFFF_0000 == id & 0xFFFF_0000).cloned();
        let (player_model, player_anims, partner_model, partner_anims, player_info, partner_info) = {
            let fs = Fsys::parse(&field_common)?;
            let e = fs.find("ken_b1", FileType::Dat).ok_or("field_common.fsys has no ken_b1 (Wes)")?;
            let (pm, pa) = model_with_anims(&fs.read(e)?)?;
            let pi = lookup(e.identifier);
            let e = fs.find("hizuki_b1", FileType::Dat).ok_or("field_common.fsys has no hizuki_b1 (Rui)")?;
            let (rm, ra) = model_with_anims(&fs.read(e)?)?;
            let ri = lookup(e.identifier);
            (pm, pa, rm, ra, pi, ri)
        };
        for (who, i) in [("Wes", &player_info), ("Rui", &partner_info)] {
            match i {
                Some(m) => println!("{who}: CharacterModels #{} slots {:?} walk stride {} run stride {}", m.index, &m.anim_slots[..4], m.walk_stride, m.run_stride),
                None => println!("{who}: no CharacterModels entry found"),
            }
        }
        let group_names: HashMap<u32, String> = source.fsys_groups().into_iter().map(|(n, g)| (g, n)).collect();
        let mut battle = crate::battle::BattleTables::load(&common, &dol, &cfs);
        battle.party_cam = source.read("fight_common.fsys").ok().and_then(|b| {
            let fs = Fsys::parse(&b).ok()?;
            let e = fs.entries.iter().find(|e| e.identifier == 0x007B_1800)?;
            let cam = hsd::Archive::parse(&fs.read(e).ok()?).ok().and_then(|a| hsdanim::load_camera(&a).ok().flatten());
            cam
        });
        battle.floor_rooms = rooms.iter().filter_map(|r| group_names.get(&r.group_id).map(|n| (r.floor_id, n.clone()))).collect();
        let battle = Arc::new(battle);
        let test_model = std::env::var("COLO_PKX").ok().and_then(|n| {
            let r = source.read(&format!("pkx_{n}.fsys")).and_then(|b| {
                let fs = Fsys::parse(&b)?;
                let e = fs.find_type(FileType::Pkx).next().ok_or("no .pkx")?;
                let pkx = fs.read(e)?;
                // the .pkx header's first u32 is the DAT size; the DAT starts at 0x40
                let size = u32::from_be_bytes(pkx[0..4].try_into()?) as usize;
                model_with_anims(pkx.get(0x40..0x40 + size).ok_or("pkx: DAT past end")?)
            });
            r.map_err(|e| eprintln!("COLO_PKX {n}: {e}")).ok().map(|(m, a)| (m, Arc::new(a)))
        });
        Ok(Game {
            source, rooms, models_table, group_names, field_common, interaction_points, doors, common_script, shadow_flags,
            player_model, player_anims: Arc::new(player_anims), partner_model, partner_anims: Arc::new(partner_anims),
            player_info, partner_info, messages, ui, test_model, battle,
        })
    }

    /// Archive name of the room with this floor id (the id warps and interaction points use).
    pub fn room_name_for_floor(&self, floor: u32) -> Option<String> {
        let room = self.rooms.iter().find(|r| r.floor_id == floor)?;
        self.group_names.get(&room.group_id).cloned()
    }

    pub fn load_room(&mut self, room_name: &str) -> Res2<RoomData> {
        let data = self.source.read(&format!("{room_name}.fsys"))?;
        let fs = Fsys::parse(&data)?;
        let room = self.rooms.iter().find(|r| r.group_id == fs.group_id).ok_or("no room record for this archive")?.clone();
        let map_rel = match fs.find(room_name, FileType::Rel) { Some(e) => MapRel::parse(&fs.read(e)?, &room)?, None => MapRel::default() };

        let rdat = fs.find(room_name, FileType::Rdat).or_else(|| fs.find_type(FileType::Rdat).next()).ok_or("room has no .rdat")?;
        let rdat_bytes = fs.read(rdat)?;
        let (map, map_anims) = model_with_anims(&rdat_bytes)?;
        // the map's scene camera is the room's default field camera (`cameraSetGScamera`): distance vector
        // eye - interest -> (distance = z, height = y, yaw = atan2(x, z))
        let scene_cam = hsd::Archive::parse(&rdat_bytes).ok().and_then(|a| hsd::scene_camera(&a).ok().flatten());
        // the map's LightSets (`floorReadMapPostFunc` GSlightLoad); every model draws with them
        let (lights, light_anims) = hsd::Archive::parse(&rdat_bytes).ok()
            .map(|a| (hsd::scene_lights(&a).unwrap_or_default(), hsdanim::load_light_anims(&a).unwrap_or_default())).unwrap_or_default();
        // `cameraSetGScamera` also copies its perspective (fov, aspect, near, far) to the field camera
        let clip = scene_cam.as_ref().map_or((1.0, 32768.0, 4.0 / 3.0), |c| (c.near, c.far, if c.aspect > 0.0 { c.aspect } else { 4.0 / 3.0 }));
        let default_camera = scene_cam.map(|c| {
            let d = [c.eye[0] - c.interest[0], c.eye[1] - c.interest[1], c.eye[2] - c.interest[2]];
            (d[2], d[1], d[0].atan2(d[2]), c.fov)
        });

        // NPC models: the room archive first, then field_common.fsys
        let fc = Fsys::parse(&self.field_common)?;
        let mut cache: HashMap<String, Option<LoadedModel>> = HashMap::new();
        let mut character_models = vec![];
        let npc_strides: Vec<(f32, f32)> = map_rel.characters.iter()
            .map(|c| self.models_table.get(c.people_index as usize).map_or((9.0, 13.0), |m| (m.walk_stride, m.run_stride))).collect();
        let npc_anim_slots: Vec<[u8; 8]> = map_rel.characters.iter()
            .map(|c| self.models_table.get(c.people_index as usize).map_or([1, 2, 3, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], |m| m.anim_slots)).collect();
        for c in &map_rel.characters {
            let Some(md) = self.models_table.get(c.people_index as usize) else { character_models.push(None); continue };
            let want = md.fsys_identifier & 0xFFFF_0000;
            let found = [&fs, &fc].into_iter().find_map(|a| {
                a.entries.iter().find(|en| en.file_type == FileType::Dat && en.identifier & 0xFFFF_0000 == want).map(|e| (a, e))
            });
            let Some((arc, e)) = found else { character_models.push(None); continue };
            let entry = cache.entry(e.name.clone()).or_insert_with(|| {
                match arc.read(e).map_err(|x| x.into()).and_then(|b| model_with_anims(&b)) {
                    Ok((m, a)) => Some(Arc::new((m, Arc::new(a)))),
                    Err(err) => { eprintln!("npc model {}: {err}", e.name); None }
                }
            });
            character_models.push(entry.clone());
        }

        // event cameras (`.cam`), by FSYS identifier = the resource id scripts pass to cameraPresetFromFile
        let mut cam_files = HashMap::new();
        for e in fs.find_type(FileType::Cam) {
            let loaded = fs.read(e).ok().and_then(|b| hsd::Archive::parse(&b).ok().and_then(|a| hsdanim::load_camera(&a).ok().flatten()));
            match loaded {
                Some(c) => { cam_files.insert(e.identifier, Arc::new(c)); }
                None => eprintln!("camera {}: could not load", e.name),
            }
        }
        // TV programmes (room record +0x48): model resource = FSYS file id << 16 | 0x1000 | set
        let mut tv_models: HashMap<u32, LoadedModel> = HashMap::new();
        for ch in &map_rel.tv {
            if tv_models.contains_key(&ch.model) { continue; }
            let found = [&fs, &fc].into_iter().find_map(|a| {
                a.entries.iter().find(|en| en.file_type == FileType::Dat && en.identifier >> 16 == ch.model >> 16).map(|e| (a, e))
            });
            match found.map(|(a, e)| a.read(e).map_err(|x| x.into()).and_then(|b| model_with_anims(&b))) {
                Some(Ok((m, a))) => { tv_models.insert(ch.model, Arc::new((m, Arc::new(a)))); }
                Some(Err(err)) => eprintln!("tv model {:#x}: {err}", ch.model),
                None => eprintln!("tv model {:#x}: not in the room archive", ch.model),
            }
        }
        // lens flare (floor.c ~490): on when the room's light block names a sun model; the sun is that model
        // part 0's translation. Only T1_ancient_colo has a .gfl.
        let flare = match (map_rel.sun_res_id, fs.find_type(FileType::Gfl).next()) {
            (0, _) | (_, None) => None,
            (sun, Some(e)) => {
                let sun_pos = fs.entries.iter().find(|en| matches!(en.file_type, FileType::Rdat | FileType::Dat) && en.identifier >> 16 == sun >> 16)
                    .and_then(|en| fs.read(en).ok())
                    .and_then(|b| hsd::Archive::parse(&b).ok().and_then(|a| hsd::load_model(&a).ok()))
                    .and_then(|m| m.set_first_joint.get((sun & 0xFFF) as usize).map(|&j| Vec3::from_array(m.joints[j].rest_translation)));
                match (gfl::Flare::parse(&fs.read(e)?), sun_pos) {
                    (Ok(f), Some(p)) => Some((Arc::new(f), p)),
                    (Err(err), _) => { eprintln!("flare: {err}"); None }
                    (_, None) => { eprintln!("flare: sun model {sun:#x} not found"); None }
                }
            }
        };
        // world_map.fsys: the world map's tables, menu textures and bike models (worldmap.rs)
        let world_map = match fs.find("worldmap_menu_rel", FileType::Rel) {
            None => None,
            Some(e) => {
                let tables = colo_formats::worldmap::WorldMap::parse(&fs.read(e)?)?;
                let mut tex = HashMap::new();
                for g in fs.find_type(FileType::Gtx) {
                    match fs.read(g).map_err(|x| x.to_string()).and_then(|b| colo_formats::gtx::decode(&b).map_err(|x| x.to_string())) {
                        Ok(img) => { tex.insert(g.identifier, img); }
                        Err(err) => eprintln!("world map texture {}: {err}", g.name),
                    }
                }
                let mut bikes = HashMap::new();
                for d in fs.find_type(FileType::Dat) {
                    match fs.read(d).map_err(|x| x.into()).and_then(|b| model_with_anims(&b)) {
                        // resource id = FSYS file id << 16 | 0x1000 | model set 0
                        Ok((m, a)) => { bikes.insert(d.identifier & 0xFFFF_0000 | 0x1000, Arc::new((m, Arc::new(a)))); }
                        Err(err) => eprintln!("world map model {}: {err}", d.name),
                    }
                }
                // move_demo set 0: the joint-less bike path (`wmapRide` plays ride anim N on it)
                let path_anims = hsd::Archive::parse(&rdat_bytes).ok().and_then(|a| hsdanim::load_jointless_animations(&a).ok()).unwrap_or_default()
                    .into_iter().filter(|a| a.model_set == 0).collect();
                let bike_size = bikes.iter().map(|(&id, m)| {
                    let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
                    for p in m.0.meshes.iter().flat_map(|me| me.positions.iter()) { lo = lo.min(Vec3::from_array(*p)); hi = hi.max(Vec3::from_array(*p)); }
                    let d = (hi - lo).max(Vec3::ZERO);
                    (id, ((d.x + d.y + d.z) / 3.0).ceil())
                }).collect();
                // the bike's particle bank (`bike_effect_poke`, resource 0xC831400)
                let bank = fs.entries.iter().find(|e| e.file_type == FileType::Gpt1 && e.identifier == 0x0C83_1400)
                    .and_then(|e| fs.read(e).ok()).and_then(|b| match colo_formats::gpt1::Gpt1::parse(&b) {
                        Ok(g) => Some(Arc::new(g)),
                        Err(err) => { eprintln!("bike_effect_poke: {err}"); None }
                    });
                Some(Arc::new(crate::worldmap::WmapData { tables, tex, bikes, path_anims, bike_size, bank }))
            }
        };
        let mut messages = HashMap::new();
        for e in fs.find_type(FileType::Msg) {
            for m in msg::parse_table(&fs.read(e)?)? { messages.insert(m.id, m.render()); }
        }
        let ccd_data = match fs.find(room_name, FileType::Ccd).or_else(|| fs.find_type(FileType::Ccd).next()) {
            Some(e) => Some(ccd::Collision::parse(&fs.read(e)?)?),
            None => None,
        };
        let collision = ccd_data.as_ref().map_or_else(Collision::default, Collision::from_ccd);
        let colsys = ccd_data.as_ref().map_or_else(crate::colsys::ColSys::default, crate::colsys::ColSys::from_ccd);
        let npc_info: Vec<Option<colo_formats::common::CharacterModel>> = map_rel.characters.iter()
            .map(|c| self.models_table.get(c.people_index as usize).cloned()).collect();
        let room_script = Script::parse(fs.read(fs.find(room_name, FileType::Scd).or_else(|| fs.find_type(FileType::Scd).next()).ok_or("room has no .scd")?)?)?;
        let entries = map_rel.entry_points.iter().map(|e| (Vec3::from_array(e.position), (e.angle as f32).to_radians())).collect();
        if std::env::var("COLO_REGIONS").is_ok() {
            println!("room record: map_res_id {:#x} scripts {:x?}", room.map_res_id, room.scripts);
            for d in self.doors.iter().filter(|d| d.floor_id as u32 == room.floor_id) { println!("  door {d:?}"); }
            let sets: std::collections::BTreeMap<usize, Vec<(usize, f32, usize)>> = map_anims.iter().fold(Default::default(), |mut m, a| {
                m.entry(a.model_set).or_default().push((a.index, a.end_frame, a.tracks.len())); m });
            for (s, v) in sets { println!("  map set {s}: anims {v:?}"); }
            for c in &map_rel.cameras { println!("  camera {c:?}"); }
            for (c, m) in map_rel.characters.iter().zip(&character_models) {
                let raw = self.models_table.get(c.people_index as usize).map(|t| t.raw.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join("")).unwrap_or_default();
                let anims: Vec<String> = m.as_ref().map(|l| l.1.iter().filter(|a| a.model_set == 0).map(|a| format!("{}:{:.0}", a.index, a.end_frame)).collect()).unwrap_or_default();
                println!("  npc {:2} people {:3} move_type {} raw {raw} anims {}", c.index, c.people_index, c.move_type, anims.join(" "));
            }
        }
        println!("{room_name}: floor {:#x}, {} map meshes, {} map anims; {} NPCs; {} messages; collision {}",
            room.floor_id, map.meshes.len(), map_anims.len(), map_rel.characters.len(), messages.len(), collision.summary());
        Ok(RoomData {
            name: room_name.into(), floor_id: room.floor_id, group_id: room.group_id, characters: map_rel.characters,
            character_models, npc_anim_slots, npc_strides, npc_info, colsys, map, map_anims: Arc::new(map_anims), messages, room_script, collision, cameras: map_rel.cameras.clone(), cam_files, entries,
            doors: self.doors.iter().filter(|d| d.floor_id as u32 == room.floor_id).cloned().collect(), scripts: room.scripts, default_camera, clip, lights, light_anims,
            tv: map_rel.tv, tv_models, flare, world_map,
        })
    }
}

/// A character model shared by every NPC that uses it, with its animations.
pub type LoadedModel = Arc<(Model, Arc<Vec<Animation>>)>;

/// Everything loaded for the current room.
#[derive(Resource)]
pub struct RoomData {
    pub name: String,
    pub floor_id: u32,
    pub group_id: u32,
    pub characters: Vec<MapCharacter>,
    pub character_models: Vec<Option<LoadedModel>>,
    /// Per character: animation slots from the CharacterModels table ([0] idle, [1] walk, [2] run).
    pub npc_anim_slots: Vec<[u8; 8]>,
    /// Per character: (walk stride, run stride) from the CharacterModels table.
    pub npc_strides: Vec<(f32, f32)>,
    /// Per character: its CharacterModels entry (radius, human collision kind, head limits).
    pub npc_info: Vec<Option<colo_formats::common::CharacterModel>>,
    /// The game's collision system over the room's `.ccd` (hero and partner movement, regions, checks).
    pub colsys: crate::colsys::ColSys,
    pub map: Model,
    pub map_anims: Arc<Vec<Animation>>,
    pub messages: HashMap<u32, String>,
    pub room_script: Script,
    pub collision: Collision,
    pub cameras: Vec<colo_formats::maprel::CameraPoint>,
    /// Event cameras from the room's `.cam` files, by FSYS identifier.
    pub cam_files: HashMap<u32, Arc<hsdanim::Camera>>,
    /// Doors of this floor (entries of the global Doors table).
    pub doors: Vec<colo_formats::common::Door>,
    /// Room record script thread ids (+0x28 sound, +0x2C modify, ...).
    pub scripts: [u32; 5],
    /// (distance, height, yaw, fov) from the map's scene camera.
    pub default_camera: Option<(f32, f32, f32, f32)>,
    /// Field camera near, far, aspect: the map scene camera's (CObjDesc +0x28/+0x2C/+0x34).
    pub clip: (f32, f32, f32),
    /// The map `.rdat`'s scene lights (`scene_data` +8).
    pub lights: Vec<hsd::Light>,
    pub light_anims: Vec<Option<hsdanim::LightAnim>>,
    /// TV channels (room record +0x48) and their programme models by resource id.
    pub tv: Vec<colo_formats::maprel::TvChannel>,
    pub tv_models: HashMap<u32, LoadedModel>,
    /// Lens flare (`.gfl`) and the sun's world position.
    pub flare: Option<(Arc<gfl::Flare>, Vec3)>,
    /// The world map room's tables, textures and bikes (`world_map.fsys` only).
    pub world_map: Option<Arc<crate::worldmap::WmapData>>,
    entries: Vec<(Vec3, f32)>,
}

impl RoomData {
    /// Entry point `idx` (falls back to the first one, then the origin), dropped onto the floor.
    pub fn entry(&self, idx: usize) -> (Vec3, f32) {
        let (p, yaw) = self.entries.get(idx).or(self.entries.first()).copied().unwrap_or((Vec3::ZERO, 0.0));
        let y = self.collision.floor_height(p, 30.0, 200.0).unwrap_or(p.y);
        (Vec3::new(p.x, y, p.z), yaw)
    }
}
