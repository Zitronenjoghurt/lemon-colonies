use crate::api::error::{ApiError, ApiResult};
use crate::state::ServerState;
use crate::websocket::connection::WebsocketConnection;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use dashmap::DashMap;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use lemon_colonies_core::data::store::Store;
use lemon_colonies_core::messages::server::ServerMessage;
use tokio::sync::broadcast;
use tower_sessions::Session;
use tower_sessions_sqlx_store::sqlx::types::Uuid;
use tracing::{error, info};

mod connection;

#[derive(Default)]
pub struct Websocket {
    connections: DashMap<Uuid, broadcast::Sender<ServerMessage>>,
}

impl Websocket {
    pub fn subscribe(&self, user_id: Uuid) -> broadcast::Receiver<ServerMessage> {
        let tx = self
            .connections
            .entry(user_id)
            .or_insert_with(|| broadcast::channel(100).0);

        info!(
            "User '{user_id}' connected. Active connections: {}",
            tx.receiver_count() + 1
        );
        tx.subscribe()
    }

    pub fn unregister_if_empty(&self, user_id: Uuid) {
        let should_remove = if let Some(tx) = self.connections.get(&user_id) {
            tx.receiver_count() == 0
        } else {
            false
        };

        if should_remove {
            self.connections.remove(&user_id);
            info!("All connections closed for '{user_id}'.");
        }
    }

    pub fn send_to_user(&self, user_id: Uuid, message: ServerMessage) {
        if let Some(tx) = self.connections.get(&user_id) {
            let _ = tx.send(message);
        }
    }

    pub fn is_user_connected(&self, user_id: Uuid) -> bool {
        self.connections.contains_key(&user_id)
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    session: Session,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    let Some(user_id) = session.get::<Uuid>("user_id").await? else {
        return Err(ApiError::Unauthorized);
    };

    let Some(user) = state.data.user.find_by_id(user_id).await? else {
        return Err(ApiError::Unauthorized);
    };

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state.clone(), user.id)))
}

async fn handle_socket(socket: WebSocket, state: ServerState, user_id: Uuid) {
    let (ws_send, ws_receive) = socket.split();
    let rx = state.ws.subscribe(user_id);

    let state_clone = state.clone();
    let send_fut = handle_send(user_id, ws_send, rx);
    let recv_fut = WebsocketConnection::new(user_id, state_clone).handle_receive(ws_receive);

    tokio::select! {
        _ = send_fut => {}
        _ = recv_fut => {}
    }

    state.ws.unregister_if_empty(user_id);
}

async fn handle_send(
    user_id: Uuid,
    mut ws_send: SplitSink<WebSocket, Message>,
    mut rx: broadcast::Receiver<ServerMessage>,
) {
    loop {
        match rx.recv().await {
            Ok(message) => {
                let encoded = message.as_bytes();
                if let Err(err) = ws_send.send(Message::binary(encoded)).await {
                    error!("[{user_id}] Failed to send message: {err}");
                    break;
                }
            }
            Err(broadcast::error::RecvError::Lagged(count)) => {
                error!("[{user_id}] Connection lagging! Missed {count} messages.");
            }
            Err(broadcast::error::RecvError::Closed) => {
                error!("[{user_id}] One connection closed.");
                break;
            }
        }
    }
}
