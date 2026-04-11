use crate::game::atlas::AtlasStore;
use crate::game::camera::ClientCamera;
use crate::game::world::ClientWorld;
use crate::ws::Ws;
use egui_macroquad::macroquad::logging::debug;
use lemon_colonies_core::game::chunk::{Chunk, CHUNK_EDGE_PIXELS};

pub mod atlas;
pub mod camera;
mod chunk;
pub mod sprite;
mod world;

pub struct Game {
    atlas: AtlasStore,
    pub camera: ClientCamera,
    pub world: ClientWorld,
}

impl Game {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            atlas: AtlasStore::load()?,
            camera: Default::default(),
            world: Default::default(),
        })
    }

    pub fn update(&mut self, ws: &mut Ws) {
        self.camera.update();

        if ws.is_connected() {
            self.request_colony_positions(ws);
            self.request_chunks(ws);
        }
    }

    pub fn draw(&mut self) {
        self.world.draw(&self.atlas, &self.camera);
    }
}

// Updates
impl Game {
    pub fn request_chunks(&mut self, ws: &mut Ws) {
        let (top_left, bottom_right) = self.camera.visible_world_bounds();

        let min_x = top_left.x.min(bottom_right.x);
        let max_x = top_left.x.max(bottom_right.x);
        let min_y = top_left.y.min(bottom_right.y);
        let max_y = top_left.y.max(bottom_right.y);

        let min_cx = (min_x / CHUNK_EDGE_PIXELS as f32).floor() as i32;
        let min_cy = (min_y / CHUNK_EDGE_PIXELS as f32).floor() as i32;
        let max_cx = (max_x / CHUNK_EDGE_PIXELS as f32).ceil() as i32;
        let max_cy = (max_y / CHUNK_EDGE_PIXELS as f32).ceil() as i32;

        let mut chunks_to_request = Vec::new();
        for x in (min_cx - 1)..=(max_cx + 1) {
            for y in (min_cy - 1)..=(max_cy + 1) {
                if self.world.should_request_chunk((x, y)) {
                    chunks_to_request.push((x, y));
                }
            }
        }

        if !chunks_to_request.is_empty() {
            debug!("Requesting {} chunk", chunks_to_request.len());
            self.world.insert_pending_chunks(&chunks_to_request);
            ws.request_chunks(chunks_to_request);
        }

        self.world
            .unload_distant_chunks(min_cx, max_cx, min_cy, max_cy);
    }

    pub fn request_colony_positions(&mut self, ws: &mut Ws) {
        if self.world.should_request_colony_positions() {
            self.world.set_colony_positions_pending();
            ws.request_colony_positions();
        }
    }
}

// Message handling
impl Game {
    pub fn handle_chunks(&mut self, chunks: Vec<Chunk>) {
        self.world.insert_chunks(chunks);
    }

    pub fn handle_colony_positions(&mut self, positions: &Vec<(i32, i32)>) {
        self.world.insert_colony_positions(positions)
    }

    pub fn handle_fog_of_war(&mut self, coords: &Vec<(i32, i32)>) {
        self.world.insert_fog_of_war(coords);
    }
}
