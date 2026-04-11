use crate::game::atlas::AtlasStore;
use crate::game::camera::ClientCamera;
use crate::game::chunk::ClientChunk;
use egui_macroquad::macroquad::prelude::*;
use lemon_colonies_core::game::chunk::Chunk;
use std::collections::{HashMap, HashSet};

const CHUNK_RETAIN_PADDING: i32 = 20;

#[derive(Default)]
pub struct ClientWorld {
    chunks: HashMap<(i32, i32), ClientChunk>,
    pending_chunks: HashSet<(i32, i32)>,
    colony_positions: Vec<(i32, i32)>,
    colony_positions_pending: bool,
    fog_of_war: HashSet<(i32, i32)>,
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

    pub fn unload_distant_chunks(&mut self, min_cx: i32, max_cx: i32, min_cy: i32, max_cy: i32) {
        let safe_min_x = min_cx - CHUNK_RETAIN_PADDING;
        let safe_max_x = max_cx + CHUNK_RETAIN_PADDING;
        let safe_min_y = min_cy - CHUNK_RETAIN_PADDING;
        let safe_max_y = max_cy + CHUNK_RETAIN_PADDING;

        self.chunks.retain(|(x, y), _| {
            *x >= safe_min_x && *x <= safe_max_x && *y >= safe_min_y && *y <= safe_max_y
        });

        self.pending_chunks.retain(|(x, y)| {
            *x >= safe_min_x && *x <= safe_max_x && *y >= safe_min_y && *y <= safe_max_y
        });
    }

    pub fn insert_chunks(&mut self, chunks: Vec<Chunk>) {
        for chunk in chunks {
            if !self.pending_chunks.contains(&(chunk.x, chunk.y)) {
                continue;
            }
            self.pending_chunks.remove(&(chunk.x, chunk.y));
            self.chunks
                .insert((chunk.x, chunk.y), ClientChunk::new(chunk));
        }
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn insert_pending_chunks(&mut self, chunks: &Vec<(i32, i32)>) {
        self.pending_chunks.extend(chunks);
    }

    pub fn insert_colony_positions(&mut self, positions: &Vec<(i32, i32)>) {
        self.colony_positions_pending = false;
        self.colony_positions.extend(positions);
    }

    pub fn insert_fog_of_war(&mut self, coords: &Vec<(i32, i32)>) {
        self.fog_of_war.extend(coords);
    }

    pub fn should_request_chunk(&self, chunk: (i32, i32)) -> bool {
        !self.chunks.contains_key(&chunk)
            && !self.pending_chunks.contains(&chunk)
            && !self.fog_of_war.contains(&chunk)
    }

    pub fn should_request_colony_positions(&self) -> bool {
        !self.colony_positions_pending && self.colony_positions.is_empty()
    }

    pub fn set_colony_positions_pending(&mut self) {
        self.colony_positions_pending = true;
    }
}
