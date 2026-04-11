use crate::error::ServerResult;
use crate::state::ServerState;
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use lemon_colonies_core::messages::client::ClientMessage;
use lemon_colonies_core::messages::server::ServerMessage;
use tower_sessions_sqlx_store::sqlx::types::Uuid;
use tracing::error;

pub struct WebsocketConnection {
    user_id: Uuid,
    state: ServerState,
}

impl WebsocketConnection {
    pub fn new(user_id: Uuid, state: ServerState) -> Self {
        Self { user_id, state }
    }

    pub async fn handle_receive(self, mut ws_receive: SplitStream<WebSocket>) {
        while let Some(Ok(message)) = ws_receive.next().await {
            match message {
                Message::Binary(data) => match ClientMessage::from_bytes(data.as_ref()) {
                    Ok(message) => {
                        if let Err(err) = self.handle_client_message(message).await {
                            error!(
                                "[{}] An error occurred on message handling: {err}",
                                self.user_id
                            );
                        }
                    }
                    Err(e) => {
                        error!("[{}] Failed to decode message: {e}", self.user_id);
                    }
                },
                Message::Close(_) => break,
                _ => {}
            };
        }
    }

    fn respond(&self, message: ServerMessage) {
        self.state.ws.send_to_user(self.user_id, message);
    }

    fn send_to_user(&self, user_id: Uuid, message: ServerMessage) {
        self.state.ws.send_to_user(user_id, message);
    }

    async fn handle_client_message(&self, message: ClientMessage) -> ServerResult<()> {
        match message {
            ClientMessage::ColonyPositions => self.handle_colony_positions().await?,
            ClientMessage::Hello => self.respond(ServerMessage::Hello),
            ClientMessage::RequestChunks(chunk_coords) => self.handle_chunks(chunk_coords).await?,
        };
        Ok(())
    }
}

/// Message handling
impl WebsocketConnection {
    async fn handle_colony_positions(&self) -> ServerResult<()> {
        let coords = self
            .state
            .data
            .colony
            .find_coords_by_user_id(self.user_id)
            .await?;
        self.respond(ServerMessage::ColonyPositions(coords));
        Ok(())
    }

    async fn handle_chunks(&self, chunk_coords: Vec<(i32, i32)>) -> ServerResult<()> {
        let visibility = self
            .state
            .service
            .chunk
            .visibility_for_user(self.user_id)
            .await?;

        for coords in chunk_coords.chunks(100) {
            let mut chunks = Vec::new();
            let mut fog_of_war = Vec::new();

            for (x, y) in coords {
                if !visibility.is_visible(*x, *y) {
                    fog_of_war.push((*x, *y));
                } else {
                    let chunk = self
                        .state
                        .data
                        .chunk
                        .load_or_generate(*x, *y, self.state.game_config.world_seed)
                        .await?;
                    chunks.push(chunk);
                }
            }

            if !chunks.is_empty() {
                self.respond(ServerMessage::Chunks(chunks));
            }

            if !fog_of_war.is_empty() {
                self.respond(ServerMessage::FogOfWar(fog_of_war));
            }
        }

        Ok(())
    }
}
