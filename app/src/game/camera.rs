use egui_macroquad::macroquad::prelude::*;

const MIN_ZOOM: f32 = 1.0;
const MAX_ZOOM: f32 = 100.0;

pub struct ClientCamera {
    camera: Camera2D,
    grab: Option<Vec2>,
    zoom_level: f32,
}

impl Default for ClientCamera {
    fn default() -> Self {
        Self {
            camera: Camera2D::default(),
            grab: None,
            zoom_level: 1.0,
        }
    }
}

impl ClientCamera {
    pub fn update(&mut self) {
        self.camera.zoom = vec2(
            self.zoom_level / screen_width(),
            self.zoom_level / screen_height(),
        );

        let scroll = mouse_wheel().1;
        if scroll != 0.0 {
            let mouse_screen = vec2(mouse_position().0, mouse_position().1);
            let before = self.camera.screen_to_world(mouse_screen);

            self.zoom_level *= 1.05_f32.powf(scroll);
            self.zoom_level = self.zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);

            self.camera.zoom = vec2(
                self.zoom_level / screen_width(),
                self.zoom_level / screen_height(),
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
