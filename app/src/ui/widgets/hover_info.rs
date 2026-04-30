use crate::i18n::Translatable;
use egui_macroquad::egui;
use egui_macroquad::egui::{Response, RichText, Ui, Widget};
use lemon_colonies_core::game::object::data::ObjectData;
use lemon_colonies_core::lingo::Lingo::{Berries, Growth};

pub struct HoverInfo<'a> {
    data: &'a ObjectData,
}

impl<'a> HoverInfo<'a> {
    pub fn new(data: &'a ObjectData) -> Self {
        Self { data }
    }

    fn spacing(&self, ui: &mut Ui) {
        ui.add(egui::Separator::default().spacing(2.0));
    }

    fn name(&self) -> String {
        match self.data {
            ObjectData::Bush(bush) => bush.kind.t().to_string(),
        }
    }

    fn info(&self, ui: &mut Ui) {
        match self.data {
            ObjectData::Bush(bush) => {
                ui.small(Berries.t());
                ui.small(format!("{}/{}", bush.berries, bush.max_berries()));
                self.spacing(ui);
                ui.small(Growth.t());
                ui.small(format!("{:.2}%", bush.growth * 100.0));
            }
        }
    }
}

impl Widget for HoverInfo<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| ui.label(RichText::new(self.name()).small().strong()));

            self.spacing(ui);

            ui.horizontal(|ui| {
                self.info(ui);
            });
        })
        .response
    }
}
