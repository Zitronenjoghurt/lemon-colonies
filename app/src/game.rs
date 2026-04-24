use crate::game::atlas::AtlasStore;
use crate::game::camera::{mouse_screen_coords, world_to_chunk, ClientCamera};
use crate::game::sprite::{HasSprite, SpriteDraw};
use crate::game::world::ClientWorld;
use crate::ws::Ws;
use egui_macroquad::macroquad::camera::set_default_camera;
use egui_macroquad::macroquad::input::is_mouse_button_pressed;
use egui_macroquad::macroquad::logging::debug;
use egui_macroquad::macroquad::prelude::{get_time, vec2, Color, MouseButton};
use lemon_colonies_core::game::chunk::{Chunk, CHUNK_EDGE_PIXELS};
use lemon_colonies_core::game::object::ObjectData;
use lemon_colonies_core::math::rect::Rect;
use lemon_colonies_core::messages::client::object_placement::ObjectPlacement;
use lemon_colonies_core::messages::server::chunk_update::{ChunkUpdateKind, ChunkUpdateMessage};

pub mod atlas;
pub mod camera;
mod chunk;
pub mod sprite;
mod world;

const CHUNK_SUBSCRIBE_DEBOUNCE_SECS: f64 = 0.2;

pub struct Game {
    atlas: AtlasStore,
    pub camera: ClientCamera,
    pub world: ClientWorld,
    object_to_place: Option<ObjectData>,
    last_subscribed_rect: Option<Rect<i32>>,
    rect_dirty_since: Option<f64>,
}

impl Game {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            atlas: AtlasStore::load()?,
            camera: Default::default(),
            world: Default::default(),
            object_to_place: Some(ObjectData::Bush),
            last_subscribed_rect: None,
            rect_dirty_since: None,
        })
    }

    pub fn update(&mut self, ws: &mut Ws) {
        self.camera.update();

        if ws.is_connected() {
            self.request_colony_positions(ws);
            self.update_chunk_subscription(ws);
            self.handle_object_placement_input(ws);
        }
    }

    pub fn draw(&mut self) {
        self.world.draw(&self.atlas, &self.camera);
        self.draw_object_to_place();
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

    pub fn request_colony_positions(&mut self, ws: &mut Ws) {
        if self.world.should_request_colony_positions() {
            self.world.set_colony_positions_pending();
            ws.request_colony_positions();
        }
    }

    pub fn handle_object_placement_input(&mut self, ws: &mut Ws) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }
        let Some(data) = self.object_to_place.take() else {
            return;
        };

        let mouse_world = self.camera.screen_to_world(mouse_screen_coords());

        let offset = data.pivot_center_offset();
        let world_coords = (mouse_world.floor() + vec2(offset.0, offset.1));
        let chunk_coords = world_to_chunk(world_coords);

        let chunk = (chunk_coords.x as i32, chunk_coords.y as i32);
        let position = (
            (world_coords.x as i32).rem_euclid(CHUNK_EDGE_PIXELS as i32) as u8,
            (world_coords.y as i32).rem_euclid(CHUNK_EDGE_PIXELS as i32) as u8,
        );

        debug!(
            "Tried to place object at {:?} in chunk {:?} (mouse world: {})",
            position, chunk, mouse_world
        );

        ws.place_object(ObjectPlacement {
            data,
            chunk,
            position,
        });
    }
}

// Rendering
impl Game {
    pub fn draw_object_to_place(&self) {
        let Some(object) = &self.object_to_place else {
            return;
        };

        self.camera.apply();

        let mouse_world = self.camera.screen_to_world(mouse_screen_coords());
        let offset = object.pivot_center_offset();
        let anchor = mouse_world.floor() + vec2(offset.0, offset.1);
        SpriteDraw::new(object.sprite(), anchor)
            .with_tint(Color::new(1.0, 1.0, 1.0, 0.5))
            .draw(&self.atlas);

        set_default_camera();
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

    pub fn handle_chunk_update(&mut self, update: ChunkUpdateMessage) {
        match update.kind {
            ChunkUpdateKind::UpdateObject(object) => self.world.update_object(object),
        }
    }
}
