use crate::game::Game;
use crate::http::Http;
use crate::ui::state::UiState;
use crate::ui::UiViewer;
use crate::ws::Ws;
use egui_macroquad::macroquad::color::BLACK;
use egui_macroquad::macroquad::logging::info;
use egui_macroquad::macroquad::prelude::{clear_background, next_frame};
use egui_notify::Toasts;
use lemon_colonies_core::messages::server::ServerMessage;

pub struct App {
    game: Game,
    http: Http,
    toasts: Toasts,
    ui: UiState,
    ws: Ws,
}

impl App {
    pub fn load() -> anyhow::Result<Self> {
        let mut http = Http::default();
        http.on_start();

        let mut ws = Ws::default();
        ws.connect();

        Ok(Self {
            game: Game::load()?,
            http,
            toasts: Toasts::new(),
            ui: UiState::default(),
            ws,
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
        self.update_ws();
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
                ws: &mut self.ws,
            };
            viewer.show(ctx);
        });
        egui_macroquad::draw();
    }
}

// Message handler
impl App {
    pub fn update_ws(&mut self) {
        self.ws.update(&mut self.toasts);
        for message in self.ws.drain_incoming() {
            self.handle_message(message);
        }
    }

    pub fn handle_message(&mut self, message: ServerMessage) {
        match message {
            ServerMessage::Hello => info!("Hello from server!"),
            ServerMessage::Shutdown => {
                self.ws.disconnect("Server shut down.");
                self.toasts.error("Disconnected: server shut down.");
            }
        }
    }
}
