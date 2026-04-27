use crate::game::atlas::AtlasStore;
use crate::game::sprite::HasSprite;
use egui_macroquad::macroquad::prelude::*;
use lemon_colonies_core::game::chunk::{Chunk, CHUNK_EDGE_LENGTH};
use lemon_colonies_core::game::object::Object;
use lemon_colonies_core::game::terrain::TERRAIN_SIZE;

const TEXTURE_SIZE: u32 = TERRAIN_SIZE as u32 * CHUNK_EDGE_LENGTH as u32;

pub struct ClientChunk {
    pub chunk: Chunk,
    pub dirty: bool,
    pub rt: RenderTarget,
}

impl ClientChunk {
    pub fn new(chunk: Chunk) -> Self {
        let rt = render_target(TEXTURE_SIZE, TEXTURE_SIZE);
        rt.texture.set_filter(FilterMode::Nearest);
        Self {
            chunk,
            dirty: true,
            rt,
        }
    }

    pub fn rebuild(&mut self, store: &AtlasStore) {
        set_camera(&Camera2D {
            render_target: Some(self.rt.clone()),
            zoom: vec2(2.0 / TEXTURE_SIZE as f32, 2.0 / TEXTURE_SIZE as f32),
            target: vec2(TEXTURE_SIZE as f32 / 2.0, TEXTURE_SIZE as f32 / 2.0),
            ..Default::default()
        });
        clear_background(BLANK);

        for y in 0..CHUNK_EDGE_LENGTH {
            for x in 0..CHUNK_EDGE_LENGTH {
                let Some(sprite) = self.chunk.get_terrain(x, y).map(|t| t.sprite()) else {
                    continue;
                };
                draw_texture_ex(
                    store.get(sprite.atlas),
                    x as f32 * TERRAIN_SIZE as f32,
                    y as f32 * TERRAIN_SIZE as f32,
                    WHITE,
                    DrawTextureParams {
                        source: Some(sprite.src),
                        dest_size: Some(vec2(TERRAIN_SIZE as f32, TERRAIN_SIZE as f32)),
                        ..Default::default()
                    },
                )
            }
        }

        set_default_camera();
        self.dirty = false;
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.rt.texture,
            self.chunk.pos.x as f32 * TEXTURE_SIZE as f32,
            self.chunk.pos.y as f32 * TEXTURE_SIZE as f32,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32)),
                ..Default::default()
            },
        );
    }

    pub fn update_object(&mut self, object: Object) {
        self.chunk.update_object(object);
    }

    pub fn tick(&mut self, server_time: f64) {
        self.chunk.tick(server_time);
    }
}
