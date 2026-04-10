use crate::game::Game;
use crate::http::Http;
use crate::ui::state::UiState;
use crate::ui::UiViewer;
use egui_macroquad::macroquad::color::BLACK;
use egui_macroquad::macroquad::prelude::{clear_background, next_frame};
use egui_notify::Toasts;

pub struct App {
    game: Game,
    http: Http,
    toasts: Toasts,
    ui: UiState,
}

impl App {
    pub fn load() -> anyhow::Result<Self> {
        let mut http = Http::default();
        http.on_start();

        Ok(Self {
            game: Game::load()?,
            http,
            toasts: Toasts::new(),
            ui: UiState::default(),
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
        self.http.update(&mut self.toasts);
    }

    pub fn render_game(&mut self) {
        self.game.update();
        clear_background(BLACK);
        self.game.draw();
    }

    pub fn render_ui(&mut self) {
        egui_macroquad::ui(|ctx| {
            let mut viewer = UiViewer {
                game: &mut self.game,
                http: &mut self.http,
                state: &mut self.ui,
                toasts: &mut self.toasts,
            };
            viewer.show(ctx);
        });
        egui_macroquad::draw();
    }
}
