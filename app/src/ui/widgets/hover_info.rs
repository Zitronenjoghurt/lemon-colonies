use crate::i18n::Translatable;
use crate::server_time::ServerTime;
use crate::tl;
use crate::ui::widgets::object_data::ObjectDataWidget;
use egui_macroquad::egui;
use egui_macroquad::egui::{Grid, Widget};
use lemon_colonies_core::game::chunk::ChunkObject;
use lemon_colonies_core::game::object::ObjectId;
use lemon_colonies_core::lingo::Lingo::{Age, Position};
use lemon_colonies_core::math::coords::ChunkCoords;

pub struct HoverInfo<'a> {
    pub object_id: ObjectId,
    pub chunk_coords: ChunkCoords,
    pub object: &'a ChunkObject,
    pub server_time: &'a ServerTime,
}

impl<'a> HoverInfo<'a> {
    pub fn new(
        object_id: ObjectId,
        chunk_coords: ChunkCoords,
        object: &'a ChunkObject,
        server_time: &'a ServerTime,
    ) -> Self {
        Self {
            object_id,
            chunk_coords,
            object,
            server_time,
        }
    }
}

impl Widget for HoverInfo<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical_centered_justified(|ui| {
            ui.heading(tl!(self.object.data.kind()));
            ui.separator();

            ObjectDataWidget::new(&self.object.data).ui(ui);

            ui.separator();

            let spacing = ui.spacing().item_spacing.x;
            let col_width = (ui.available_width() - spacing) / 2.0;
            Grid::new("hover_info_grid")
                .num_columns(2)
                .min_col_width(col_width)
                .show(ui, |ui| {
                    let pos = self.chunk_coords.with_local(self.object.pos).world();
                    ui.label(Position.t());
                    ui.label(pos.to_string());
                    ui.end_row();

                    let age = self.server_time.elapsed_since(self.object.created_at);
                    ui.label(Age.t());
                    ui.label(age.to_string());
                    ui.end_row();
                });
        })
        .response
    }
}
