use crate::game::atlas::AtlasStore;
use crate::game::camera::ClientCamera;
use crate::game::chunk::ClientChunk;
use crate::game::data::ClientData;
use crate::game::sprite::{HasSprite, SpriteDraw};
use egui_macroquad::macroquad::prelude::{
    draw_line, draw_rectangle_lines, set_default_camera, Color, Rect as GlamRect,
};
use lemon_colonies_core::game::chunk::{Chunk, CHUNK_EDGE_PIXELS};
use lemon_colonies_core::game::object::Object;
use lemon_colonies_core::math::coords::{ChunkCoords, WorldCoords};
use lemon_colonies_core::math::rect::Rect;
use std::collections::HashMap;

const CHUNK_RETAIN_PADDING: i32 = 20;
const CHUNK_BORDER_THICKNESS: f32 = 1.0;
const TERRITORY_OUTLINE_THICKNESS: f32 = 2.0;
const TERRITORY_OUTLINE_COLOR: Color = Color::new(1.0, 6.0, 0.0, 0.8);

pub struct WorldDrawSettings {
    pub chunk_borders: bool,
    pub object_collisions: bool,
}

#[derive(Default)]
pub struct ClientWorld {
    chunks: HashMap<ChunkCoords, ClientChunk>,
}

impl ClientWorld {
    fn rebuild(&mut self, store: &AtlasStore) {
        for chunk in self.chunks.values_mut().filter(|c| c.dirty) {
            chunk.rebuild(store);
        }
    }

    pub fn draw(
        &mut self,
        store: &AtlasStore,
        camera: &ClientCamera,
        settings: &WorldDrawSettings,
        data: &ClientData,
    ) {
        self.rebuild(store);

        camera.apply();

        self.draw_chunks();
        self.draw_objects(store, settings);
        self.draw_chunk_grid(settings);
        self.draw_territory_outline(data);

        set_default_camera();
    }

    fn draw_chunks(&mut self) {
        for chunk in self.chunks.values() {
            chunk.draw();
        }
    }

    fn draw_objects(&mut self, store: &AtlasStore, settings: &WorldDrawSettings) {
        let mut draws: Vec<SpriteDraw> = Vec::new();

        for chunk in self.chunks.values() {
            for obj in chunk.chunk.objects.values() {
                let mut draw = SpriteDraw::new(
                    obj.data.sprite(),
                    obj.pos.with_chunk(chunk.chunk.pos).world(),
                );
                if settings.object_collisions {
                    let rect = obj.data.collision_rect(draw.anchor);
                    draw = draw.with_collision(GlamRect::new(
                        rect.min.x,
                        rect.min.y,
                        rect.width(),
                        rect.height(),
                    ));
                }
                draws.push(draw);
            }
        }

        draws.sort_by(|a, b| {
            a.sort_y
                .partial_cmp(&b.sort_y)
                .unwrap()
                .then(a.anchor.x.partial_cmp(&b.anchor.x).unwrap())
        });

        for sprite_draw in &draws {
            sprite_draw.draw(store);
        }

        if settings.object_collisions {
            for sprite_draw in &draws {
                sprite_draw.draw_collision();
            }
        }
    }

    fn draw_territory_outline(&self, data: &ClientData) {
        let Some(owned) = data.owned_chunks.value() else {
            return;
        };

        for &pos in owned {
            let x = pos.x as f32 * CHUNK_EDGE_PIXELS as f32;
            let y = pos.y as f32 * CHUNK_EDGE_PIXELS as f32;
            let size = CHUNK_EDGE_PIXELS as f32;

            if !owned.contains(&ChunkCoords::new(pos.x, pos.y - 1)) {
                draw_line(
                    x,
                    y,
                    x + size,
                    y,
                    TERRITORY_OUTLINE_THICKNESS,
                    TERRITORY_OUTLINE_COLOR,
                );
            }
            if !owned.contains(&ChunkCoords::new(pos.x, pos.y + 1)) {
                draw_line(
                    x,
                    y + size,
                    x + size,
                    y + size,
                    TERRITORY_OUTLINE_THICKNESS,
                    TERRITORY_OUTLINE_COLOR,
                );
            }
            if !owned.contains(&ChunkCoords::new(pos.x - 1, pos.y)) {
                draw_line(
                    x,
                    y,
                    x,
                    y + size,
                    TERRITORY_OUTLINE_THICKNESS,
                    TERRITORY_OUTLINE_COLOR,
                );
            }
            if !owned.contains(&ChunkCoords::new(pos.x + 1, pos.y)) {
                draw_line(
                    x + size,
                    y,
                    x + size,
                    y + size,
                    TERRITORY_OUTLINE_THICKNESS,
                    TERRITORY_OUTLINE_COLOR,
                );
            }
        }
    }

    fn draw_chunk_grid(&self, settings: &WorldDrawSettings) {
        if !settings.chunk_borders {
            return;
        }

        let color = Color::new(1.0, 0.0, 1.0, 0.6);
        for chunk in self.chunks.values() {
            let x = chunk.chunk.pos.x as f32 * CHUNK_EDGE_PIXELS as f32;
            let y = chunk.chunk.pos.y as f32 * CHUNK_EDGE_PIXELS as f32;
            draw_rectangle_lines(
                x,
                y,
                CHUNK_EDGE_PIXELS as f32,
                CHUNK_EDGE_PIXELS as f32,
                CHUNK_BORDER_THICKNESS,
                color,
            );
        }
    }

    pub fn unload_distant_chunks(&mut self, rect: lemon_colonies_core::math::rect::Rect<i32>) {
        let safe_min_x = rect.min.x - CHUNK_RETAIN_PADDING;
        let safe_max_x = rect.max.x + CHUNK_RETAIN_PADDING;
        let safe_min_y = rect.min.y - CHUNK_RETAIN_PADDING;
        let safe_max_y = rect.max.y + CHUNK_RETAIN_PADDING;

        self.chunks.retain(|pos, _| {
            pos.x >= safe_min_x && pos.x <= safe_max_x && pos.y >= safe_min_y && pos.y <= safe_max_y
        });
    }

    pub fn insert_chunks(&mut self, chunks: Vec<Chunk>) {
        for chunk in chunks {
            self.chunks.insert(chunk.pos, ClientChunk::new(chunk));
        }
    }

    pub fn get_chunk(&self, pos: ChunkCoords) -> Option<&Chunk> {
        self.chunks.get(&pos).map(|c| &c.chunk)
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn update_object(&mut self, object: Object) {
        let Some(chunk) = self.chunks.get_mut(&object.pos.chunk) else {
            return;
        };
        chunk.update_object(object);
    }

    pub fn rect_collides_with_object(&self, rect: Rect<f32>) -> bool {
        let min_chunk = WorldCoords::new(rect.min.x, rect.min.y).chunk();
        let max_chunk = WorldCoords::new(rect.max.x, rect.max.y).chunk();

        for cy in min_chunk.y..=max_chunk.y {
            for cx in min_chunk.x..=max_chunk.x {
                let Some(chunk) = self.get_chunk(ChunkCoords::new(cx, cy)) else {
                    continue;
                };
                if chunk.rect_collides_with_object(rect) {
                    return true;
                }
            }
        }

        false
    }
}
