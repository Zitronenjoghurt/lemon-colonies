use crate::game::atlas::AtlasStore;
use crate::game::camera::{mouse_screen_coords, ClientCamera};
use crate::game::data::ClientData;
use crate::game::sprite::{HasSprite, SpriteDraw};
use crate::game::world::ClientWorld;
use crate::ws::Ws;
use egui_macroquad::macroquad::camera::set_default_camera;
use egui_macroquad::macroquad::color::Color;
use egui_macroquad::macroquad::input::{is_mouse_button_pressed, MouseButton};
use egui_macroquad::macroquad::logging::debug;
use egui_macroquad::macroquad::prelude::Rect as GlamRect;
use lemon_colonies_core::game::object::data::ObjectData;
use lemon_colonies_core::game::object::purchase::PurchasableObject;
use lemon_colonies_core::math::coords::{ChunkCoords, ChunkLocal, WorldCoords};
use lemon_colonies_core::messages::client::object_placement::ObjectPlacement;

#[derive(Default)]
pub struct ObjectPlace {
    target_data: Option<ObjectData>,
    purchase: Option<PurchasableObject>,
    collision_detected: bool,
    last_collision: Option<ChunkLocal>,
    continuous: bool,
}

impl ObjectPlace {
    pub fn place(&mut self, data: ObjectData) {
        self.target_data = Some(data);
        self.continuous = true;
    }

    pub fn purchase(&mut self, purchase: PurchasableObject) {
        self.target_data = Some(purchase.object_data());
        self.purchase = Some(purchase);
        self.continuous = true;
    }

    pub fn update(
        &mut self,
        ws: &mut Ws,
        camera: &ClientCamera,
        world: &ClientWorld,
        data: &ClientData,
    ) {
        if self.target_data.is_none() {
            return;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            self.purchase = None;
            self.target_data = None;
            self.continuous = false;
            return;
        }

        let mouse_world = camera.screen_to_world(mouse_screen_coords());
        self.update_collision(world, data, mouse_world);
        self.handle_input(ws, mouse_world);
    }

    pub fn draw(&self, atlas: &AtlasStore, camera: &ClientCamera) {
        self.draw_object_to_place(atlas, camera)
    }

    fn update_collision(
        &mut self,
        world: &ClientWorld,
        data: &ClientData,
        mouse_world: WorldCoords,
    ) {
        let Some(object) = &self.target_data else {
            return;
        };

        let mouse_pos = mouse_world.chunk_local();
        if Some(mouse_pos) == self.last_collision {
            return;
        }

        let visuals = object.visuals();

        let offset = visuals.pivot_center_offset();
        let world_coords = mouse_world.floor() + WorldCoords::new(offset.0, offset.1);
        let collision_rect = visuals.collision_rect(world_coords);

        let (chunk_min, chunk_max) = collision_rect.chunk_range();
        for chunk_y in chunk_min.y..=chunk_max.y {
            for chunk_x in chunk_min.x..=chunk_max.x {
                let chunk_coords = ChunkCoords::new(chunk_x, chunk_y);
                if let Some(owned_chunks) = data.player_owned_chunks.value()
                    && !owned_chunks.contains(&chunk_coords)
                {
                    self.collision_detected = true;
                    self.last_collision = Some(mouse_pos);
                    return;
                }
            }
        }

        self.collision_detected = world.rect_collides_with_object(collision_rect);
        self.last_collision = Some(mouse_pos);
    }

    pub fn wants_to_place(&self) -> bool {
        self.target_data.is_some()
    }

    pub fn purchasable_object(&self) -> &Option<PurchasableObject> {
        &self.purchase
    }
}

// Input
impl ObjectPlace {
    pub fn handle_input(&mut self, ws: &mut Ws, mouse_world: WorldCoords) {
        if self.collision_detected {
            return;
        }

        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }

        let Some(data) = &self.target_data else {
            return;
        };

        let offset = data.visuals().pivot_center_offset();
        let world_coords = mouse_world.floor() + WorldCoords::new(offset.0, offset.1);
        let pos = world_coords.chunk_local();

        debug!(
            "Tried to place object at {:?} (mouse world: {:?})",
            pos, mouse_world
        );

        if let Some(purchase) = &self.purchase {
            ws.purchase_object(*purchase, pos);
        } else {
            ws.place_object(ObjectPlacement {
                data: data.clone(),
                pos,
            });
        }

        if !self.continuous {
            self.target_data = None;
            self.purchase = None;
        }
    }
}

// Rendering
impl ObjectPlace {
    pub fn draw_object_to_place(&self, atlas: &AtlasStore, camera: &ClientCamera) {
        let Some(object) = &self.target_data else {
            return;
        };
        let visuals = object.visuals();

        camera.apply();

        let mouse_world = camera.screen_to_world(mouse_screen_coords());
        let offset = visuals.pivot_center_offset();
        let anchor = mouse_world.floor() + WorldCoords::new(offset.0, offset.1);
        let tint = if self.collision_detected {
            Color::new(1.0, 0.2, 0.2, 0.5)
        } else {
            Color::new(1.0, 1.0, 1.0, 0.5)
        };

        let collision = visuals.collision_rect(anchor);
        let collision_rect = GlamRect::new(
            collision.min.x,
            collision.min.y,
            collision.width(),
            collision.height(),
        );

        let sprite_draw = SpriteDraw::new(visuals.sprite(), anchor)
            .with_tint(tint)
            .with_collision(collision_rect);

        sprite_draw.draw(atlas);
        sprite_draw.draw_collision();

        set_default_camera();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectActionMode {
    Place,
}
