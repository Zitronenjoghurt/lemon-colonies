use crate::game::Game;
use crate::http::Http;
use egui_macroquad::egui;
use egui_macroquad::macroquad::color::BLACK;
use egui_macroquad::macroquad::prelude::{clear_background, next_frame};

pub struct App {
    game: Game,
    http: Http,
}

impl App {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            game: Game::load()?,
            http: Http::default(),
        })
    }

    pub async fn run(mut self) {
        loop {
            self.update();
            next_frame().await;
        }
    }

    pub fn update(&mut self) {
        self.render_game();
        self.render_ui();
        self.http.update();
    }

    pub fn render_game(&mut self) {
        self.game.update();
        clear_background(BLACK);
        self.game.draw();
    }

    pub fn render_ui(&mut self) {
        egui_macroquad::ui(|ctx| {
            egui::Window::new("Debug").show(ctx, |ui| {
                ui.label("It works!");

                if ui.button("Logout").clicked() {
                    self.http.logout();
                }
            });
        });
        egui_macroquad::draw();
    }
}
