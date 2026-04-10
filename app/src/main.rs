use crate::app::App;
use egui_macroquad::macroquad;
use egui_macroquad::macroquad::prelude::*;

mod app;
mod bindings;
mod game;
mod http;
mod ui;

fn window_conf() -> Conf {
    Conf {
        window_title: "Lemon Colonies".to_owned(),
        fullscreen: true,
        window_resizable: true,
        window_width: 1920,
        window_height: 1080,
        high_dpi: true,
        platform: miniquad::conf::Platform {
            webgl_version: miniquad::conf::WebGLVersion::WebGL2,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info| {
        miniquad::error!("{}", info);
    }));

    App::load().unwrap().run().await;
}
