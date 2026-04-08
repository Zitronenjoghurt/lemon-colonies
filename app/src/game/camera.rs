use egui_macroquad::macroquad::prelude::*;

const MIN_ZOOM: f32 = 1.0;
const MAX_ZOOM: f32 = 100.0;

pub struct ClientCamera {
    camera: Camera2D,
    grab: Option<Vec2>,
}

impl Default for ClientCamera {
    fn default() -> Self {
        let zoom = vec2(1.0 / screen_width(), 1.0 / screen_height());
        Self {
            camera: Camera2D {
                zoom,
                ..Default::default()
            },
            grab: None,
        }
    }
}

impl ClientCamera {
    pub fn update(&mut self) {
        let scroll = mouse_wheel().1;
        if scroll != 0.0 {
            let mouse_screen = vec2(mouse_position().0, mouse_position().1);
            let before = self.camera.screen_to_world(mouse_screen);
            self.camera.zoom *= 1.05_f32.powf(scroll);
            self.camera.zoom = self.camera.zoom.clamp(
                vec2(MIN_ZOOM / screen_width(), MIN_ZOOM / screen_height()),
                vec2(MAX_ZOOM / screen_width(), MAX_ZOOM / screen_height()),
            );
            let after = self.camera.screen_to_world(mouse_screen);
            self.camera.target += before - after;
        }

        if is_mouse_button_down(MouseButton::Middle) {
            let mouse_screen = vec2(mouse_position().0, mouse_position().1);
            if let Some(grab_world) = self.grab {
                let current_world = self.camera.screen_to_world(mouse_screen);
                self.camera.target += grab_world - current_world;
            } else {
                self.grab = Some(self.camera.screen_to_world(mouse_screen));
            }
        } else {
            self.grab = None;
        }
    }

    pub fn apply(&self) {
        set_camera(&self.camera);
    }
}
