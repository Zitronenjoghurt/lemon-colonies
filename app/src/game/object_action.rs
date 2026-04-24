use crate::game::atlas::AtlasStore;
use crate::game::camera::{mouse_screen_coords, world_to_chunk, ClientCamera};
use crate::game::sprite::{HasSprite, SpriteDraw};
use crate::ws::Ws;
use egui_macroquad::macroquad::camera::set_default_camera;
use egui_macroquad::macroquad::color::Color;
use egui_macroquad::macroquad::input::{is_mouse_button_pressed, MouseButton};
use egui_macroquad::macroquad::logging::debug;
use egui_macroquad::macroquad::math::vec2;
use lemon_colonies_core::game::chunk::CHUNK_EDGE_PIXELS;
use lemon_colonies_core::game::object::ObjectData;
use lemon_colonies_core::messages::client::object_placement::ObjectPlacement;

#[derive(Default)]
pub struct ObjectAction {
    target_data: Option<ObjectData>,
    mode: Option<ObjectActionMode>,
}

impl ObjectAction {
    pub fn place(&mut self, data: ObjectData) {
        self.target_data = Some(data);
        self.mode = Some(ObjectActionMode::Place);
    }

    pub fn update(&mut self, ws: &mut Ws, camera: &ClientCamera) {
        if let Some(mode) = &self.mode {
            match mode {
                ObjectActionMode::Place => self.handle_object_placement_input(ws, camera),
            }
        }
    }

    pub fn draw(&self, atlas: &AtlasStore, camera: &ClientCamera) {
        if let Some(mode) = &self.mode {
            match mode {
                ObjectActionMode::Place => self.draw_object_to_place(atlas, camera),
            }
        }
    }
}

// Input
impl ObjectAction {
    pub fn handle_object_placement_input(&mut self, ws: &mut Ws, camera: &ClientCamera) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }
        let Some(data) = self.target_data.take() else {
            return;
        };

        let mouse_world = camera.screen_to_world(mouse_screen_coords());

        let offset = data.pivot_center_offset();
        let world_coords = mouse_world.floor() + vec2(offset.0, offset.1);
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

        self.mode = None;
    }
}

// Rendering
impl ObjectAction {
    pub fn draw_object_to_place(&self, atlas: &AtlasStore, camera: &ClientCamera) {
        let Some(object) = &self.target_data else {
            return;
        };

        camera.apply();

        let mouse_world = camera.screen_to_world(mouse_screen_coords());
        let offset = object.pivot_center_offset();
        let anchor = mouse_world.floor() + vec2(offset.0, offset.1);
        SpriteDraw::new(object.sprite(), anchor)
            .with_tint(Color::new(1.0, 1.0, 1.0, 0.5))
            .draw(atlas);

        set_default_camera();
    }
}

pub enum ObjectActionMode {
    Place,
}
