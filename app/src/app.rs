use crate::fps_counter::FpsCounter;
use crate::game::Game;
use crate::http::Http;
use crate::server_time::ServerTime;
use crate::settings::Settings;
use crate::ui::state::UiState;
use crate::ui::{setup_egui, UiViewer};
use crate::ws::Ws;
use egui_macroquad::macroquad::color::BLACK;
use egui_macroquad::macroquad::logging::{debug, info};
use egui_macroquad::macroquad::prelude::{clear_background, next_frame};
use egui_notify::Toasts;
use lemon_colonies_core::messages::server::ServerMessage;

pub struct App {
    settings: Settings,
    game: Game,
    http: Http,
    toasts: Toasts,
    ui: UiState,
    ws: Ws,
    server_time: ServerTime,
    fps_counter: FpsCounter,
    egui_initialized: bool,
}

impl App {
    pub fn load() -> anyhow::Result<Self> {
        let mut ws = Ws::default();
        ws.connect();

        Ok(Self {
            settings: Settings::load(),
            game: Game::load()?,
            http: Http::default(),
            toasts: Toasts::new(),
            ui: UiState::load(),
            ws,
            server_time: ServerTime::default(),
            fps_counter: FpsCounter::default(),
            egui_initialized: false,
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
        self.ui.update();
        self.server_time.update(&mut self.ws);
        self.fps_counter.update();

        if self.server_time.is_timed_out() {
            self.ws.disconnect("Connection timed out");
        }
    }

    pub fn render_game(&mut self) {
        self.game
            .update(&mut self.ws, &self.server_time, self.ui.wants_pointer);
        clear_background(BLACK);
        self.game.draw(&self.settings);
    }

    pub fn render_ui(&mut self) {
        egui_macroquad::ui(|ctx| {
            if !self.egui_initialized {
                setup_egui(ctx);
                self.egui_initialized = true;
            }
            let mut viewer = UiViewer {
                settings: &mut self.settings,
                fps_counter: self.fps_counter,
                game: &mut self.game,
                http: &mut self.http,
                server_time: &self.server_time,
                state: &mut self.ui,
                toasts: &mut self.toasts,
                ws: &mut self.ws,
            };
            viewer.show(ctx);
            self.settings.apply(ctx);
            self.ui.wants_pointer = ctx.is_pointer_over_area();
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
            ServerMessage::Pong {
                client_time,
                server_time,
            } => {
                self.server_time.handle_pong(client_time, server_time);
            }
            ServerMessage::Error(error) => {
                self.toasts.error(error);
            }
            ServerMessage::Chunks(chunks) => {
                debug!("Received {} chunks", chunks.len());
                self.game.handle_chunks(chunks);
            }
            ServerMessage::ColonyPositions(positions) => {
                debug!("Received {} colony positions", positions.len());
                self.game.handle_colony_positions(positions);
            }
            ServerMessage::ChunkUpdate(update) => {
                debug!("Received chunk update: {:?}", update);
                self.game.handle_chunk_update(update);
            }
            ServerMessage::OwnedChunks(chunks) => {
                self.game.handle_owned_chunks(chunks);
            }
            ServerMessage::UserInfo(info) => {
                self.game.handle_user_info(info);
            }
        }
    }
}
