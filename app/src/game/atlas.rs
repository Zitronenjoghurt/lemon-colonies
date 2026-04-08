use egui_macroquad::macroquad::prelude::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Atlas {
    BaseOverworld,
}

impl Atlas {
    pub fn data(&self) -> &'static [u8] {
        match self {
            Atlas::BaseOverworld => include_bytes!("../../assets/base/Overworld.png"),
        }
    }
}

pub struct AtlasStore {
    textures: HashMap<Atlas, Texture2D>,
}

impl AtlasStore {
    pub fn load() -> anyhow::Result<Self> {
        let mut textures = HashMap::new();
        for atlas in Atlas::iter() {
            let tex = Texture2D::from_file_with_format(atlas.data(), None);
            tex.set_filter(FilterMode::Nearest);
            textures.insert(atlas, tex);
        }
        Ok(Self { textures })
    }

    pub fn get(&self, atlas: Atlas) -> &Texture2D {
        &self.textures[&atlas]
    }
}
