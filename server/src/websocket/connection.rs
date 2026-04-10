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
                        self.handle_client_message(message).await;
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

    async fn handle_client_message(&self, message: ClientMessage) {
        match message {
            ClientMessage::Hello => self.respond(ServerMessage::Hello),
        }
    }
}
