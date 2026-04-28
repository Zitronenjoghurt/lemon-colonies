use crate::tl;
use crate::ui::widgets::object_data::ObjectDataWidget;
use egui_macroquad::egui;
use egui_macroquad::egui::Widget;
use lemon_colonies_core::game::chunk::ChunkObject;
use lemon_colonies_core::game::object::ObjectId;
use lemon_colonies_core::math::coords::ChunkCoords;

pub struct HoverInfo<'a> {
    pub object_id: ObjectId,
    pub chunk_coords: ChunkCoords,
    pub object: &'a ChunkObject,
}

impl<'a> HoverInfo<'a> {
    pub fn new(object_id: ObjectId, chunk_coords: ChunkCoords, object: &'a ChunkObject) -> Self {
        Self {
            object_id,
            chunk_coords,
            object,
        }
    }
}

impl Widget for HoverInfo<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical_centered_justified(|ui| {
            ui.heading(tl!(self.object.data.kind()));
            ui.separator();
            ObjectDataWidget::new(&self.object.data).ui(ui);
        })
        .response
    }
}
