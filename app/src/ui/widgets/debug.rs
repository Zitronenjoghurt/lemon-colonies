use crate::game::camera::world_to_chunk;
use crate::game::Game;
use egui_macroquad::egui::{Grid, Response, Ui, Widget};
use egui_macroquad::macroquad::prelude::{mouse_position, vec2};

pub struct DebugWidget<'a> {
    pub game: &'a mut Game,
}

impl<'a> DebugWidget<'a> {
    pub fn new(game: &'a mut Game) -> Self {
        Self { game }
    }
}

impl Widget for DebugWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        Grid::new("debug_grid")
            .num_columns(2)
            .show(ui, |ui| {
                let mouse_screen = vec2(mouse_position().0, mouse_position().1);
                ui.label("Mouse pos. (screen)");
                ui.label(format!("({:.2}, {:.2})", mouse_screen.x, mouse_screen.y));
                ui.end_row();

                let mouse_world = self.game.camera.screen_to_world(mouse_screen);
                ui.label("Mouse pos. (world)");
                ui.label(format!("({:.2}, {:.2})", mouse_world.x, mouse_world.y));
                ui.end_row();

                let mouse_chunk = world_to_chunk(mouse_world);
                ui.label("Mouse pos. (chunk)");
                ui.label(format!("({:.2}, {:.2})", mouse_chunk.x, mouse_chunk.y));
                ui.end_row();

                ui.label("Loaded chunks");
                ui.label(self.game.world.chunk_count().to_string());
                ui.end_row();
            })
            .response
    }
}
