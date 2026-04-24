use egui_macroquad::macroquad::prelude::*;
use lemon_colonies_core::game::chunk::CHUNK_EDGE_PIXELS;
use lemon_colonies_core::math::point::Point;

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
            let before = self.screen_to_world(mouse_screen_coords());

            self.zoom_level *= 1.05_f32.powf(scroll);
            self.zoom_level = self.zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);

            self.camera.zoom = vec2(
                self.zoom_level / screen_width(),
                self.zoom_level / screen_height(),
            );

            let after = self.camera.screen_to_world(mouse_screen_coords());
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

    pub fn visible_world_bounds(&self) -> (Vec2, Vec2) {
        let top_left = self.camera.screen_to_world(vec2(0.0, 0.0));
        let bottom_right = self
            .camera
            .screen_to_world(vec2(screen_width(), screen_height()));
        (top_left, bottom_right)
    }

    pub fn visible_rect(&self) -> lemon_colonies_core::math::rect::Rect<i32> {
        let (top_left, bottom_right) = self.visible_world_bounds();

        let min_x = top_left.x.min(bottom_right.x);
        let max_x = top_left.x.max(bottom_right.x);
        let min_y = top_left.y.min(bottom_right.y);
        let max_y = top_left.y.max(bottom_right.y);

        let min_cx = (min_x / CHUNK_EDGE_PIXELS as f32).floor() as i32 - 1;
        let min_cy = (min_y / CHUNK_EDGE_PIXELS as f32).floor() as i32 - 1;
        let max_cx = (max_x / CHUNK_EDGE_PIXELS as f32).ceil() as i32 + 1;
        let max_cy = (max_y / CHUNK_EDGE_PIXELS as f32).ceil() as i32 + 1;

        lemon_colonies_core::math::rect::Rect::new(
            Point::new(min_cx, min_cy),
            Point::new(max_cx, max_cy),
        )
    }

    pub fn screen_to_world(&self, screen: Vec2) -> Vec2 {
        self.camera.screen_to_world(screen)
    }
}

pub fn mouse_screen_coords() -> Vec2 {
    vec2(mouse_position().0, mouse_position().1)
}

pub fn world_to_chunk(world: Vec2) -> Vec2 {
    (world / CHUNK_EDGE_PIXELS as f32).floor()
}
