use crate::game::Game;
use egui_macroquad::egui;
use egui_macroquad::macroquad::color::BLACK;
use egui_macroquad::macroquad::prelude::{clear_background, next_frame};

pub struct App {
    game: Game,
}

impl App {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            game: Game::load()?,
        })
    }

    pub async fn run(mut self) {
        loop {
            self.render();
            next_frame().await;
        }
    }

    pub fn render(&mut self) {
        self.render_game();
        self.render_ui();
    }

    pub fn render_game(&mut self) {
        self.game.update();
        clear_background(BLACK);
        self.game.draw();
    }

    pub fn render_ui(&self) {
        egui_macroquad::ui(|ctx| {
            egui::Window::new("Debug").show(ctx, |ui| {
                ui.label("It works!");
            });
        });
        egui_macroquad::draw();
    }
}
