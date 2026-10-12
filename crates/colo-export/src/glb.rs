//! Write an HSD [`Model`] (bind pose, model space) as a single-scene .glb.
//!
//! One glTF node per mesh (positions are already in model space, non-indexed triangles). Materials: base colour from
//! the MObj diffuse (if its diffuse source includes the material) times vertex colours (if it includes
//! vertices) times the diffuse texture; unlit (KHR_materials_unlit) when the MObj has lighting off;
//! BLEND for XLU materials, MASK 0.5 for textures with alpha; all double-sided.

use colo_formats::hsd::{Model, Wrap};
use std::fmt::Write;

struct Bin {
    data: Vec<u8>,
    views: Vec<String>,
    accessors: Vec<String>,
}

impl Bin {
    fn view(&mut self, bytes: &[u8], target: Option<u32>) -> usize {
        while !self.data.len().is_multiple_of(4) {
            self.data.push(0);
        }
        let off = self.data.len();
        self.data.extend_from_slice(bytes);
        let t = target.map(|t| format!(",\"target\":{t}")).unwrap_or_default();
        self.views.push(format!("{{\"buffer\":0,\"byteOffset\":{off},\"byteLength\":{}{t}}}", bytes.len()));
        self.views.len() - 1
    }

    fn f32s(&mut self, v: &[f32], ty: &str, n: usize, minmax: bool) -> usize {
        let bytes: Vec<u8> = v.iter().flat_map(|x| x.to_le_bytes()).collect();
        let view = self.view(&bytes, Some(34962));
        let mut mm = String::new();
        if minmax && n > 0 {
            let (mut lo, mut hi) = (vec![f32::MAX; n], vec![f32::MIN; n]);
            for c in v.chunks(n) {
                for i in 0..n {
                    lo[i] = lo[i].min(c[i]);
                    hi[i] = hi[i].max(c[i]);
                }
            }
            let f = |a: &[f32]| a.iter().map(|x| format!("{x}")).collect::<Vec<_>>().join(",");
            mm = format!(",\"min\":[{}],\"max\":[{}]", f(&lo), f(&hi));
        }
        self.accessors.push(format!(
            "{{\"bufferView\":{view},\"componentType\":5126,\"count\":{},\"type\":\"{ty}\"{mm}}}",
            v.len() / n
        ));
        self.accessors.len() - 1
    }

    fn u32s(&mut self, v: &[u32]) -> usize {
        let bytes: Vec<u8> = v.iter().flat_map(|x| x.to_le_bytes()).collect();
        let view = self.view(&bytes, Some(34963));
        self.accessors.push(format!(
            "{{\"bufferView\":{view},\"componentType\":5125,\"count\":{},\"type\":\"SCALAR\"}}",
            v.len()
        ));
        self.accessors.len() - 1
    }
}

fn esc(s: &str) -> String {
    s.chars().filter(|c| !c.is_control() && *c != '"' && *c != '\\').collect()
}

pub fn write_glb(model: &Model, name: &str) -> Vec<u8> {
    let name = esc(name);
    let mut bin = Bin { data: vec![], views: vec![], accessors: vec![] };
    let mut images = vec![];
    for img in &model.textures {
        let png = crate::png::encode_rgba(img.width, img.height, &img.rgba);
        let v = bin.view(&png, None);
        images.push(format!("{{\"bufferView\":{v},\"mimeType\":\"image/png\"}}"));
    }
    let wrap = |w: Wrap| match w {
        Wrap::Clamp => 33071,
        Wrap::Repeat => 10497,
        Wrap::Mirror => 33648,
    };
    let (mut samplers, mut textures, mut materials) = (vec![], vec![], vec![]);
    let mut uses_unlit = false;
    let (mut meshes, mut nodes) = (vec![], vec![]);
    for (i, p) in model.meshes.iter().enumerate() {
        if p.hidden { continue; }
        let m = &p.material;
        // material (one per mesh; meshes sharing an MObj get identical copies)
        let use_mat = m.diffuse_source() != 2;
        let base = if use_mat { [m.diffuse[0], m.diffuse[1], m.diffuse[2], m.alpha] } else { [1., 1., 1., m.alpha] };
        let mut s = format!(
            "{{\"pbrMetallicRoughness\":{{\"baseColorFactor\":[{},{},{},{}],\"metallicFactor\":0,\"roughnessFactor\":1",
            base[0], base[1], base[2], base[3].clamp(0., 1.)
        );
        let mut alpha_mode = if m.translucent { "BLEND" } else { "OPAQUE" };
        if let Some(t) = m.texture {
            samplers.push(format!("{{\"magFilter\":9729,\"minFilter\":9729,\"wrapS\":{},\"wrapT\":{}}}", wrap(m.wrap[0]), wrap(m.wrap[1])));
            textures.push(format!("{{\"sampler\":{},\"source\":{t}}}", samplers.len() - 1));
            let _ = write!(s, ",\"baseColorTexture\":{{\"index\":{}}}", textures.len() - 1);
            if alpha_mode == "OPAQUE" && model.textures[t].has_alpha() { alpha_mode = "MASK"; }
        }
        s.push('}');
        let _ = write!(s, ",\"doubleSided\":{},\"alphaMode\":\"{alpha_mode}\"", !p.cull_back);
        if alpha_mode == "MASK" { s.push_str(",\"alphaCutoff\":0.5"); }
        if !m.lit() { s.push_str(",\"extensions\":{\"KHR_materials_unlit\":{}}"); uses_unlit = true; }
        s.push('}');
        materials.push(s);
        let mat_index = materials.len() - 1;

        let pos: Vec<f32> = p.positions.iter().flatten().copied().collect();
        let mut attrs = format!("\"POSITION\":{}", bin.f32s(&pos, "VEC3", 3, true));
        let n: Vec<f32> = p.normals.iter().flatten().copied().collect();
        let _ = write!(attrs, ",\"NORMAL\":{}", bin.f32s(&n, "VEC3", 3, false));
        if m.texture.is_some() {
            let uv: Vec<f32> = p.uvs.iter().flatten().copied().collect();
            let _ = write!(attrs, ",\"TEXCOORD_0\":{}", bin.f32s(&uv, "VEC2", 2, false));
        }
        if m.diffuse_source() >= 2 {
            let c: Vec<f32> = p.colors.iter().flatten().copied().collect();
            let _ = write!(attrs, ",\"COLOR_0\":{}", bin.f32s(&c, "VEC4", 4, false));
        }
        let indices: Vec<u32> = (0..p.positions.len() as u32).collect();
        let idx = bin.u32s(&indices);
        meshes.push(format!("{{\"primitives\":[{{\"attributes\":{{{attrs}}},\"indices\":{idx},\"material\":{mat_index}}}]}}"));
        nodes.push(format!("{{\"name\":\"{name}_{i}_j{}\",\"mesh\":{}}}", p.joint, meshes.len() - 1));
    }
    let scene_nodes: Vec<String> = (0..nodes.len()).map(|i| i.to_string()).collect();
    while !bin.data.len().is_multiple_of(4) {
        bin.data.push(0);
    }
    let ext = if uses_unlit { ",\"extensionsUsed\":[\"KHR_materials_unlit\"]" } else { "" };
    let json = format!(
        "{{\"asset\":{{\"version\":\"2.0\",\"generator\":\"colo-rs\"}}{ext},\"scene\":0,\"scenes\":[{{\"name\":\"{name}\",\"nodes\":[{}]}}],\
\"nodes\":[{}],\"meshes\":[{}],\"materials\":[{}],\"textures\":[{}],\"samplers\":[{}],\"images\":[{}],\
\"accessors\":[{}],\"bufferViews\":[{}],\"buffers\":[{{\"byteLength\":{}}}]}}",
        scene_nodes.join(","),
        nodes.join(","),
        meshes.join(","),
        materials.join(","),
        textures.join(","),
        samplers.join(","),
        images.join(","),
        bin.accessors.join(","),
        bin.views.join(","),
        bin.data.len()
    );
    let mut j = json.into_bytes();
    while !j.len().is_multiple_of(4) {
        j.push(b' ');
    }
    let mut out = vec![];
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&((12 + 8 + j.len() + 8 + bin.data.len()) as u32).to_le_bytes());
    out.extend_from_slice(&(j.len() as u32).to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(&j);
    out.extend_from_slice(&(bin.data.len() as u32).to_le_bytes());
    out.extend_from_slice(b"BIN\0");
    out.extend_from_slice(&bin.data);
    out
}
