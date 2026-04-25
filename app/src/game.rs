use crate::game::atlas::AtlasStore;
use crate::game::camera::ClientCamera;
use crate::game::data::ClientData;
use crate::game::object_action::ObjectAction;
use crate::game::world::ClientWorld;
use crate::settings::Settings;
use crate::ws::Ws;
use egui_macroquad::macroquad::logging::debug;
use egui_macroquad::macroquad::prelude::{get_time, MouseButton};
use lemon_colonies_core::game::chunk::Chunk;
use lemon_colonies_core::math::coords::ChunkCoords;
use lemon_colonies_core::math::rect::Rect;
use lemon_colonies_core::messages::server::chunk_update::{ChunkUpdateKind, ChunkUpdateMessage};
use lemon_colonies_core::types::user_info::PrivateUserInfo;

pub mod atlas;
pub mod camera;
mod chunk;
pub mod data;
mod object_action;
pub mod sprite;
mod world;

const CHUNK_SUBSCRIBE_DEBOUNCE_SECS: f64 = 0.2;

pub struct Game {
    atlas: AtlasStore,
    pub camera: ClientCamera,
    pub data: ClientData,
    pub world: ClientWorld,
    pub object_action: ObjectAction,
    last_subscribed_rect: Option<Rect<i32>>,
    rect_dirty_since: Option<f64>,
}

impl Game {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            atlas: AtlasStore::load()?,
            camera: Default::default(),
            data: Default::default(),
            world: Default::default(),
            object_action: Default::default(),
            last_subscribed_rect: None,
            rect_dirty_since: None,
        })
    }

    pub fn update(&mut self, ws: &mut Ws, pointer_consumed: bool) {
        self.camera.update();

        if ws.is_connected() {
            self.data.update(ws);
            self.update_chunk_subscription(ws);

            if !pointer_consumed {
                self.object_action.update(ws, &self.camera);
            }
        }
    }

    pub fn draw(&mut self, settings: &Settings) {
        self.world.draw(&self.atlas, &self.camera, settings);
        self.object_action.draw(&self.atlas, &self.camera);
    }
}

// Updates
impl Game {
    fn update_chunk_subscription(&mut self, ws: &mut Ws) {
        let rect = self.camera.visible_rect();
        if self.last_subscribed_rect == Some(rect) {
            self.rect_dirty_since = None;
            return;
        }

        let now = get_time();
        let dirty_since = *self.rect_dirty_since.get_or_insert(now);

        if now - dirty_since < CHUNK_SUBSCRIBE_DEBOUNCE_SECS {
            return;
        }

        debug!("Subscribing to chunk rect: {:?}", rect);
        self.last_subscribed_rect = Some(rect);
        self.rect_dirty_since = None;
        ws.subscribe_chunks(rect);
        self.world.unload_distant_chunks(rect);
    }
}

// Message handling
impl Game {
    pub fn handle_chunks(&mut self, chunks: Vec<Chunk>) {
        self.world.insert_chunks(chunks);
    }

    pub fn handle_colony_positions(&mut self, positions: Vec<ChunkCoords>) {
        self.data.colony_positions.set_value(positions);
    }

    pub fn handle_chunk_update(&mut self, update: ChunkUpdateMessage) {
        match update.kind {
            ChunkUpdateKind::UpdateObject(object) => self.world.update_object(object),
        }
    }

    pub fn handle_user_info(&mut self, info: PrivateUserInfo) {
        self.data.user_info.set_value(info);
    }
}
