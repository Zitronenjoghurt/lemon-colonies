use crate::game::atlas::AtlasStore;
use crate::game::camera::ClientCamera;
use crate::game::chunk::ClientChunk;
use egui_macroquad::macroquad::prelude::*;
use lemon_colonies_core::game::chunk::Chunk;
use std::collections::HashMap;

pub struct ClientWorld {
    pub chunks: HashMap<(i32, i32), ClientChunk>,
}

impl Default for ClientWorld {
    fn default() -> Self {
        Self {
            chunks: HashMap::from([((0, 0), ClientChunk::new(Chunk::generate(0, 0)))]),
        }
    }
}

impl ClientWorld {
    fn rebuild(&mut self, store: &AtlasStore) {
        for chunk in self.chunks.values_mut().filter(|c| c.dirty) {
            chunk.rebuild(store);
        }
    }

    pub fn draw(&mut self, store: &AtlasStore, camera: &ClientCamera) {
        self.rebuild(store);

        camera.apply();

        for chunk in self.chunks.values() {
            chunk.draw();
        }

        set_default_camera();
    }
}
