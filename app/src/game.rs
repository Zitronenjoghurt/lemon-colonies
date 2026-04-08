use crate::game::atlas::AtlasStore;
use crate::game::camera::ClientCamera;
use crate::game::world::ClientWorld;

pub mod atlas;
pub mod camera;
mod chunk;
pub mod sprite;
mod world;

pub struct Game {
    atlas: AtlasStore,
    camera: ClientCamera,
    world: ClientWorld,
}

impl Game {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            atlas: AtlasStore::load()?,
            camera: Default::default(),
            world: Default::default(),
        })
    }

    pub fn update(&mut self) {
        self.camera.update();
    }

    pub fn draw(&mut self) {
        self.world.draw(&self.atlas, &self.camera);
    }
}
