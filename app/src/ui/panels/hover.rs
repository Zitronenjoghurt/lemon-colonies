use crate::ui::widgets::hover_info::HoverInfo;
use crate::ui::UiViewer;
use egui_macroquad::egui;
use egui_macroquad::egui::{Id, Widget};

pub fn show(ctx: &egui::Context, v: &mut UiViewer) {
    let Some((_, _, object)) = v.game.get_hovered_object() else {
        return;
    };

    let Some(data) = &object.data else {
        return;
    };

    let mouse_pos = ctx.input(|i| i.pointer.hover_pos().unwrap_or_default());

    egui::Area::new(Id::new("hover_panel"))
        .fixed_pos(mouse_pos)
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style())
                .inner_margin(4.0)
                .fill(ui.style().visuals.window_fill().gamma_multiply(0.8))
                .show(ui, |ui| {
                    ui.spacing_mut().interact_size.y = 0.0;
                    HoverInfo::new(data).ui(ui);
                });
        });
}
