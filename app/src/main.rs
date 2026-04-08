use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::{egui, macroquad};

fn window_conf() -> Conf {
    Conf {
        window_title: "Lemon Colonies".to_owned(),
        fullscreen: false,
        window_resizable: true,
        window_width: 1920,
        window_height: 1080,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Debug").show(ctx, |ui| {
                ui.label("It works!");
            });
        });
        egui_macroquad::draw();

        next_frame().await;
    }
}
