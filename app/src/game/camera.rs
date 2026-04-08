use egui_macroquad::macroquad::prelude::*;

pub struct ClientCamera {
    pos: Vec2,
    zoom: f32,
}

impl Default for ClientCamera {
    fn default() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
            zoom: 1.0,
        }
    }
}

impl ClientCamera {
    pub fn apply(&self) {
        set_camera(&Camera2D {
            target: self.pos,
            zoom: vec2(self.zoom / screen_width(), self.zoom / screen_height()),
            ..Default::default()
        });
    }
}
