use crate::game::atlas::AtlasStore;
use crate::game::camera::{mouse_screen_coords, world_to_chunk, world_to_tile, ClientCamera};
use crate::game::sprite::{HasSprite, SpriteDraw};
use crate::game::world::ClientWorld;
use crate::ws::Ws;
use egui_macroquad::macroquad::camera::set_default_camera;
use egui_macroquad::macroquad::input::{is_mouse_button_pressed, is_mouse_button_released};
use egui_macroquad::macroquad::logging::debug;
use egui_macroquad::macroquad::prelude::{get_time, Color, MouseButton};
use lemon_colonies_core::game::chunk::{Chunk, CHUNK_EDGE_LENGTH};
use lemon_colonies_core::game::object::ObjectData;
use lemon_colonies_core::math::rect::Rect;
use lemon_colonies_core::messages::client::object_placement::ObjectPlacement;

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

        let mouse_world = self.camera.screen_to_world(mouse_screen_coords()).floor();
        let mouse_chunk = world_to_chunk(mouse_world);
        let mouse_tile = world_to_tile(mouse_world);

        let chunk = (mouse_chunk.x as i32, mouse_chunk.y as i32);
        let position = (
            (mouse_tile.x as i32).rem_euclid(CHUNK_EDGE_LENGTH as i32) as u8,
            (mouse_tile.y as i32).rem_euclid(CHUNK_EDGE_LENGTH as i32) as u8,
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
        SpriteDraw::new(object.sprite(), mouse_world.floor())
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
}
