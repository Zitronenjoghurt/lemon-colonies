use crate::ui::widgets::generic_select::GenericSelect;
use crate::ui::widgets::profile_menu::ProfileMenu;
use crate::ui::windows::WindowId;
use crate::ui::UiViewer;
use egui_macroquad::egui;
use egui_macroquad::egui::{TopBottomPanel, Widget};

pub fn show(ctx: &egui::Context, v: &mut UiViewer) {
    TopBottomPanel::new(v.settings.action_panel_pos.opposite().egui(), "meta_panel").show(
        ctx,
        |ui| {
            ui.horizontal(|ui| {
                ui.label("Lemon Colonies");
                ui.separator();
                v.window_button(ui, WindowId::Settings);
                v.window_button(ui, WindowId::Debug);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ProfileMenu::new(&v.game.data, v.http).ui(ui);
                    v.settings.dirty |=
                        GenericSelect::from_enum(&mut v.settings.locale, "locale_select")
                            .ui(ui)
                            .changed();
                });
            });
        },
    );
}
