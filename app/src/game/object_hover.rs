use crate::game::camera::{mouse_screen_coords, ClientCamera};
use crate::game::world::ClientWorld;
use lemon_colonies_core::game::object::ObjectId;
use lemon_colonies_core::math::coords::{ChunkCoords, ChunkLocal};
use lemon_colonies_core::math::point::Point;
use lemon_colonies_core::math::rect::Rect;

const MOUSE_HOVER_CHUNK_RADIUS: f32 = 5.0;

#[derive(Default)]
pub struct ObjectHover {
    hovered_object: Option<(ObjectId, ChunkLocal)>,
    last_check: Option<ChunkLocal>,
}

impl ObjectHover {
    pub fn update(&mut self, camera: &ClientCamera, world: &ClientWorld) {
        let mouse_world = camera.screen_to_world(mouse_screen_coords());
        let mouse_pos = mouse_world.chunk_local();
        if Some(mouse_pos) == self.last_check {
            return;
        }
        self.last_check = Some(mouse_pos);

        let hover_rect = Rect::from_size(
            Point::new(
                mouse_world.x - MOUSE_HOVER_CHUNK_RADIUS,
                mouse_world.y - MOUSE_HOVER_CHUNK_RADIUS,
            ),
            MOUSE_HOVER_CHUNK_RADIUS * 2.0,
            MOUSE_HOVER_CHUNK_RADIUS * 2.0,
        );

        let mouse_point = Point::new(mouse_world.x, mouse_world.y);
        let (chunk_min, chunk_max) = hover_rect.chunk_range();
        for chunk_y in chunk_min.y..=chunk_max.y {
            for chunk_x in chunk_min.x..=chunk_max.x {
                let coords = ChunkCoords::new(chunk_x, chunk_y);
                let Some(chunk) = world.get_chunk(coords) else {
                    continue;
                };
                for (id, obj) in &chunk.objects {
                    let pos = obj.pos.with_chunk(coords).world();
                    let bounding_rect = obj.data.bounding_rect(pos);
                    if bounding_rect.contains_point(&mouse_point) {
                        self.hovered_object = Some((*id, pos.chunk_local()));
                        return;
                    }
                }
            }
        }

        self.hovered_object = None;
    }

    pub fn reset(&mut self) {
        self.hovered_object = None;
        self.last_check = None;
    }

    pub fn get(&self) -> Option<(ObjectId, ChunkLocal)> {
        self.hovered_object
    }
}
