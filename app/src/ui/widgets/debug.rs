use crate::fps_counter::FpsCounter;
use crate::game::camera::mouse_screen_coords;
use crate::game::Game;
use crate::i18n::Translatable;
use crate::server_time::{DateTime, ServerTime};
use crate::settings::Settings;
use crate::tl;
use egui_macroquad::egui::{Grid, Response, Ui, Widget};
use lemon_colonies_core::game::object::data::bush::BushObject;
use lemon_colonies_core::game::object::data::ObjectData;
use lemon_colonies_core::game::object::kind::ObjectKind;
use lemon_colonies_core::lingo::Lingo::*;

pub struct DebugWidget<'a> {
    pub fps_counter: FpsCounter,
    pub game: &'a mut Game,
    pub settings: &'a mut Settings,
    pub server_time: &'a ServerTime,
}

impl<'a> DebugWidget<'a> {
    pub fn new(
        fps_counter: FpsCounter,
        game: &'a mut Game,
        settings: &'a mut Settings,
        server_time: &'a ServerTime,
    ) -> Self {
        Self {
            fps_counter,
            game,
            settings,
            server_time,
        }
    }
}

impl Widget for DebugWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            if self.server_time.ready() {
                Grid::new("debug_time_grid").num_columns(2).show(ui, |ui| {
                    ui.label(Latency.t());
                    ui.label(format!("{:.2}ms", self.server_time.latency() * 1000.0));
                    ui.end_row();

                    ui.label(ServerTime.t());
                    ui.vertical(|ui| {
                        let time = self.server_time.now();
                        ui.label(DateTime::from_unix(time).to_string());
                        ui.label(format!("{:.2}s", time));
                    });
                    ui.end_row();
                });
            }

            ui.separator();

            Grid::new("debug_performance_grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("FPS");
                    ui.label(format!("{:.2}", self.fps_counter.fps()));
                    ui.end_row();

                    ui.label(LoadedChunks.t());
                    ui.label(self.game.world.chunk_count().to_string());
                    ui.end_row();

                    ui.label(LoadedObjects.t());
                    ui.label(self.game.world.object_count().to_string());
                    ui.end_row();
                });

            ui.separator();

            Grid::new("debug_mouse_grid").num_columns(2).show(ui, |ui| {
                let mouse_screen = mouse_screen_coords();
                ui.label(format!("{} (screen)", MousePosition.t()));
                ui.label(format!("({:.2}, {:.2})", mouse_screen.x, mouse_screen.y));
                ui.end_row();

                let mouse_world = self.game.camera.screen_to_world(mouse_screen);
                ui.label(format!("{} (world)", MousePosition.t()));
                ui.label(format!("({:.2}, {:.2})", mouse_world.x, mouse_world.y));
                ui.end_row();

                let mouse_chunk = mouse_world.chunk();
                ui.label(format!("{} (chunk)", MousePosition.t()));
                ui.label(format!("({}, {})", mouse_chunk.x, mouse_chunk.y));
                ui.end_row();

                let mouse_local = mouse_world.local();
                ui.label(format!("{} (local)", MousePosition.t()));
                ui.label(format!("({}, {})", mouse_local.x, mouse_local.y));
                ui.end_row();
            });

            ui.separator();

            Grid::new("debug_action_grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label(tl!(DisplayX, x = ChunkBorders.t()));
                    self.settings.dirty |= ui
                        .checkbox(&mut self.settings.display_chunk_borders, "")
                        .changed();
                    ui.end_row();

                    ui.label(tl!(DisplayX, x = ObjectBounds.t()));
                    self.settings.dirty |= ui
                        .checkbox(&mut self.settings.display_object_bounds, "")
                        .changed();
                    ui.end_row();

                    ui.label(tl!(DisplayX, x = ObjectCollisions.t()));
                    self.settings.dirty |= ui
                        .checkbox(&mut self.settings.display_object_collisions, "")
                        .changed();
                    ui.end_row();

                    ui.label(ObjectKind::Bush.t());
                    if ui.button(PlaceVerb.t()).clicked() {
                        self.game
                            .object_place
                            .start(ObjectData::Bush(BushObject::default()));
                    }
                    ui.end_row();
                });
        })
        .response
    }
}
