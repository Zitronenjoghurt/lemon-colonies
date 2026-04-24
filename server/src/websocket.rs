use crate::api::error::{ApiError, ApiResult};
use crate::state::ServerState;
use crate::websocket::connection::{ConnectionId, WebsocketConnection};
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use dashmap::DashMap;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use lemon_colonies_core::data::store::Store;
use lemon_colonies_core::math::point::Point;
use lemon_colonies_core::math::rect::Rect;
use lemon_colonies_core::messages::server::chunk_update::ChunkUpdateMessage;
use lemon_colonies_core::messages::server::ServerMessage;
use std::collections::HashSet;
use tokio::sync::mpsc;
use tower_sessions::Session;
use tracing::{error, info};
use uuid::Uuid;

mod chunk_subscriptions;
mod connection;

#[derive(Default)]
pub struct Websocket {
    connections: DashMap<ConnectionId, (Uuid, mpsc::Sender<ServerMessage>)>,
    users: DashMap<Uuid, HashSet<ConnectionId>>,
    chunk_subscriptions: chunk_subscriptions::ChunkSubscriptions,
}

impl Websocket {
    pub fn register(&self, user_id: Uuid) -> (ConnectionId, mpsc::Receiver<ServerMessage>) {
        let connection_id = Uuid::new_v4();
        let (tx, rx) = mpsc::channel(100);
        self.connections.insert(connection_id, (user_id, tx));

        let count = {
            let mut entry = self.users.entry(user_id).or_default();
            entry.insert(connection_id);
            entry.len()
        };

        info!("User '{user_id}' connected ({connection_id}). Active connections for user: {count}");
        (connection_id, rx)
    }

    pub fn unregister(&self, connection_id: ConnectionId) {
        let Some((_, (user_id, _))) = self.connections.remove(&connection_id) else {
            return;
        };

        info!("Connection '{connection_id}' of user '{user_id}' closed.");

        let user_empty = {
            let Some(mut entry) = self.users.get_mut(&user_id) else {
                return;
            };
            entry.remove(&connection_id);
            entry.is_empty()
        };

        if user_empty {
            self.users.remove(&user_id);
            self.chunk_subscriptions.unsubscribe(connection_id);
            info!("All connections closed for '{}'.", user_id);
        }
    }

    pub fn send_to_connection(&self, connection_id: ConnectionId, message: ServerMessage) {
        let Some(conn) = self.connections.get(&connection_id) else {
            return;
        };
        let _ = conn.1.try_send(message);
    }

    pub fn send_to_user(&self, user_id: Uuid, message: ServerMessage) {
        let Some(connection_ids) = self.users.get(&user_id) else {
            return;
        };

        if connection_ids.len() == 1 {
            let Some(conn) = self.connections.get(connection_ids.iter().next().unwrap()) else {
                return;
            };
            let _ = conn.1.try_send(message);
        } else {
            for connection_id in connection_ids.iter() {
                let Some(conn) = self.connections.get(connection_id) else {
                    continue;
                };
                let _ = conn.1.try_send(message.clone());
            }
        }
    }

    pub fn is_user_connected(&self, user_id: Uuid) -> bool {
        self.users.contains_key(&user_id)
    }

    pub fn user_connection_count(&self, user_id: Uuid) -> usize {
        self.users.get(&user_id).map(|s| s.len()).unwrap_or(0)
    }

    /// Returns the previous rect if there is one.
    pub fn subscribe_to_chunks(&self, connection_id: Uuid, rect: Rect<i32>) -> Option<Rect<i32>> {
        self.chunk_subscriptions.subscribe(connection_id, rect)
    }

    pub fn send_chunk_update(&self, update: ChunkUpdateMessage) {
        let coords = Point::new(update.coords.0, update.coords.1);
        let connections = self.chunk_subscriptions.connections_for_chunk(&coords);

        if connections.len() == 1 {
            let connection_id = connections.first().unwrap();
            self.send_to_connection(*connection_id, ServerMessage::ChunkUpdate(update))
        } else {
            for connection_id in self.chunk_subscriptions.connections_for_chunk(&coords) {
                self.send_to_connection(connection_id, ServerMessage::ChunkUpdate(update.clone()))
            }
        }
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

    let connection_count = state.ws.user_connection_count(user.id);
    if connection_count >= state.config.max_user_connection_count {
        return Err(ApiError::TooManyConnections);
    }

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state.clone(), user.id)))
}

async fn handle_socket(socket: WebSocket, state: ServerState, user_id: Uuid) {
    let (ws_send, ws_receive) = socket.split();
    let (connection_id, rx) = state.ws.register(user_id);

    let state_clone = state.clone();
    let send_fut = handle_send(user_id, ws_send, rx);
    let recv_fut =
        WebsocketConnection::new(connection_id, user_id, state_clone).handle_receive(ws_receive);

    tokio::select! {
        _ = send_fut => {}
        _ = recv_fut => {}
    }

    state.ws.unregister(connection_id);
}

async fn handle_send(
    connection_id: ConnectionId,
    mut ws_send: SplitSink<WebSocket, Message>,
    mut rx: mpsc::Receiver<ServerMessage>,
) {
    while let Some(message) = rx.recv().await {
        let encoded = message.as_bytes();
        if let Err(err) = ws_send.send(Message::binary(encoded)).await {
            error!("[{connection_id}] Failed to send message: {err}");
            break;
        }
    }
}
