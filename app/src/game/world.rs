use crate::game::atlas::AtlasStore;
use crate::game::camera::ClientCamera;
use crate::game::chunk::ClientChunk;
use crate::game::sprite::{HasSprite, SpriteDraw};
use crate::settings::Settings;
use egui_macroquad::macroquad::prelude::*;
use lemon_colonies_core::game::chunk::{Chunk, CHUNK_EDGE_PIXELS};
use lemon_colonies_core::game::object::Object;
use std::collections::HashMap;

const CHUNK_RETAIN_PADDING: i32 = 20;
const CHUNK_BORDER_THICKNESS: f32 = 1.0;

#[derive(Default)]
pub struct ClientWorld {
    chunks: HashMap<(i32, i32), ClientChunk>,
}

impl ClientWorld {
    fn rebuild(&mut self, store: &AtlasStore) {
        for chunk in self.chunks.values_mut().filter(|c| c.dirty) {
            chunk.rebuild(store);
        }
    }

    pub fn draw(&mut self, store: &AtlasStore, camera: &ClientCamera, settings: &Settings) {
        self.rebuild(store);

        camera.apply();

        self.draw_chunks();
        self.draw_objects(store);
        self.draw_chunk_grid(settings);

        set_default_camera();
    }

    fn draw_chunks(&mut self) {
        for chunk in self.chunks.values() {
            chunk.draw();
        }
    }

    fn draw_objects(&mut self, store: &AtlasStore) {
        let mut objects: Vec<SpriteDraw> = Vec::new();
        for chunk in self.chunks.values() {
            for obj in chunk.chunk.objects.values() {
                let world_pos = vec2(
                    chunk.chunk.x as f32 * CHUNK_EDGE_PIXELS as f32 + obj.x as f32,
                    chunk.chunk.y as f32 * CHUNK_EDGE_PIXELS as f32 + obj.y as f32,
                );
                objects.push(SpriteDraw::new(obj.data.sprite(), world_pos));
            }
        }
        objects.sort_by(|a, b| a.sort_y.partial_cmp(&b.sort_y).unwrap());
        for obj in objects.drain(..) {
            obj.draw(store);
        }
    }

    fn draw_chunk_grid(&self, settings: &Settings) {
        if !settings.display_chunk_borders {
            return;
        }

        let color = Color::new(1.0, 0.0, 1.0, 0.6);
        for chunk in self.chunks.values() {
            let x = chunk.chunk.x as f32 * CHUNK_EDGE_PIXELS as f32;
            let y = chunk.chunk.y as f32 * CHUNK_EDGE_PIXELS as f32;
            draw_rectangle_lines(
                x,
                y,
                CHUNK_EDGE_PIXELS as f32,
                CHUNK_EDGE_PIXELS as f32,
                CHUNK_BORDER_THICKNESS,
                color,
            );
        }
    }

    pub fn unload_distant_chunks(&mut self, rect: lemon_colonies_core::math::rect::Rect<i32>) {
        let safe_min_x = rect.min.x - CHUNK_RETAIN_PADDING;
        let safe_max_x = rect.max.x + CHUNK_RETAIN_PADDING;
        let safe_min_y = rect.min.y - CHUNK_RETAIN_PADDING;
        let safe_max_y = rect.max.y + CHUNK_RETAIN_PADDING;

        self.chunks.retain(|(x, y), _| {
            *x >= safe_min_x && *x <= safe_max_x && *y >= safe_min_y && *y <= safe_max_y
        });
    }

    pub fn insert_chunks(&mut self, chunks: Vec<Chunk>) {
        for chunk in chunks {
            self.chunks
                .insert((chunk.x, chunk.y), ClientChunk::new(chunk));
        }
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn update_object(&mut self, object: Object) {
        let Some(chunk) = self.chunks.get_mut(&(object.chunk_x, object.chunk_y)) else {
            return;
        };
        chunk.update_object(object);
    }
}
