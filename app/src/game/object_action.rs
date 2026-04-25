use crate::game::atlas::AtlasStore;
use crate::game::camera::{mouse_screen_coords, ClientCamera};
use crate::game::sprite::{HasSprite, SpriteDraw};
use crate::ws::Ws;
use egui_macroquad::macroquad::camera::set_default_camera;
use egui_macroquad::macroquad::color::Color;
use egui_macroquad::macroquad::input::{is_mouse_button_pressed, MouseButton};
use egui_macroquad::macroquad::logging::debug;
use lemon_colonies_core::game::object::ObjectData;
use lemon_colonies_core::math::coords::WorldCoords;
use lemon_colonies_core::messages::client::object_placement::ObjectPlacement;

#[derive(Default)]
pub struct ObjectAction {
    target_data: Option<ObjectData>,
    mode: Option<ObjectActionMode>,
    continuous: bool,
}

impl ObjectAction {
    pub fn place(&mut self, data: ObjectData) {
        self.target_data = Some(data);
        self.mode = Some(ObjectActionMode::Place);
        self.continuous = true;
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
        let Some(data) = &self.target_data else {
            return;
        };

        let mouse_world = camera.screen_to_world(mouse_screen_coords());

        let offset = data.pivot_center_offset();
        let world_coords = mouse_world + WorldCoords::new(offset.0, offset.1);
        let pos = world_coords.chunk_local();

        debug!(
            "Tried to place object at {:?} (mouse world: {:?})",
            pos, mouse_world
        );

        ws.place_object(ObjectPlacement {
            data: data.clone(),
            pos,
        });

        if !self.continuous {
            self.target_data = None;
            self.mode = None;
        }
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
        let anchor = mouse_world.floor() + WorldCoords::new(offset.0, offset.1);
        SpriteDraw::new(object.sprite(), anchor)
            .with_tint(Color::new(1.0, 1.0, 1.0, 0.5))
            .draw(atlas);

        set_default_camera();
    }
}

pub enum ObjectActionMode {
    Place,
}
