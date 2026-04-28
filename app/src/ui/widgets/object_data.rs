use crate::i18n::Translatable;
use crate::tl;
use egui_macroquad::egui::{Grid, Response, Ui, Widget};
use lemon_colonies_core::game::object::data::ObjectData;
use lemon_colonies_core::lingo::Lingo;

pub struct ObjectDataWidget<'a> {
    pub data: &'a ObjectData,
}

impl<'a> ObjectDataWidget<'a> {
    pub fn new(data: &'a ObjectData) -> Self {
        Self { data }
    }

    fn info(&self, ui: &mut Ui) {
        match self.data {
            ObjectData::Bush(bush) => {
                ui.label(Lingo::Kind.t());
                ui.label(tl!(bush.kind));
                ui.end_row();

                ui.label(Lingo::Berries.t());
                ui.label(format!("{}/{}", bush.berries, bush.max_berries()));
                ui.end_row();

                ui.label(Lingo::BerryGrowth.t());
                ui.label(format!("{:.2}%", bush.growth * 100.0));
                ui.end_row();
            }
        }
    }
}

impl Widget for ObjectDataWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let spacing = ui.spacing().item_spacing.x;
        let col_width = (ui.available_width() - spacing) / 2.0;

        Grid::new("object_data_info_grid")
            .num_columns(2)
            .min_col_width(col_width)
            .striped(true)
            .show(ui, |ui| self.info(ui))
            .response
    }
}
