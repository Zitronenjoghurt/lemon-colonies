use crate::error::{ServerError, ServerResult};
use crate::state::ServerState;
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use lemon_colonies_core::data::entity::object;
use lemon_colonies_core::data::store::Store;
use lemon_colonies_core::game::object::Object;
use lemon_colonies_core::math::coords::ChunkCoords;
use lemon_colonies_core::math::rect::Rect;
use lemon_colonies_core::messages::client::object_placement::ObjectPlacement;
use lemon_colonies_core::messages::client::ClientMessage;
use lemon_colonies_core::messages::server::chunk_update::ChunkUpdateMessage;
use lemon_colonies_core::messages::server::ServerMessage;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::timeout;
use tower_sessions_sqlx_store::sqlx::types::Uuid;
use tracing::{error, info};

pub type ConnectionId = Uuid;

const IDLE_TIMEOUT: Duration = Duration::from_secs(60);

pub struct WebsocketConnection {
    id: ConnectionId,
    user_id: Uuid,
    state: ServerState,
}

impl WebsocketConnection {
    pub fn new(id: ConnectionId, user_id: Uuid, state: ServerState) -> Self {
        Self { id, user_id, state }
    }

    pub async fn handle_receive(self, mut ws_receive: SplitStream<WebSocket>) {
        loop {
            let msg = match timeout(IDLE_TIMEOUT, ws_receive.next()).await {
                Ok(Some(Ok(message))) => message,
                Ok(Some(Err(err))) => {
                    error!("[{}] WebSocket error: {err}", self.user_id);
                    break;
                }
                Ok(None) => {
                    info!("[{}] WebSocket stream closed", self.user_id);
                    break;
                }
                Err(_) => {
                    info!(
                        "[{}] Connection timed out (idle for {IDLE_TIMEOUT:?})",
                        self.user_id
                    );
                    break;
                }
            };

            match msg {
                Message::Binary(data) => match ClientMessage::from_bytes(data.as_ref()) {
                    Ok(message) => {
                        if let Err(err) = self.handle_client_message(message).await {
                            if err.is_user_error() {
                                self.respond(ServerMessage::Error(err.message()));
                            } else {
                                error!(
                                    "[{}] An error occurred on message handling: {err}",
                                    self.user_id
                                );
                            }
                        }
                    }
                    Err(e) => {
                        error!("[{}] Failed to decode message: {e}", self.user_id);
                    }
                },
                Message::Close(reason) => {
                    info!("[{}] Client closed connection: {reason:?}", self.user_id);
                    break;
                }
                _ => {}
            }
        }
    }

    fn respond(&self, message: ServerMessage) {
        self.state.ws.send_to_connection(self.id, message);
    }

    fn send_to_user(&self, user_id: Uuid, message: ServerMessage) {
        self.state.ws.send_to_user(user_id, message);
    }

    async fn handle_client_message(&self, message: ClientMessage) -> ServerResult<()> {
        match message {
            ClientMessage::Ping { client_time } => self.handle_ping(client_time).await?,
            ClientMessage::ColonyPositions => self.handle_colony_positions().await?,
            ClientMessage::SubscribeToChunks(rect) => self.handle_chunk_subscription(rect).await?,
            ClientMessage::ObjectPlacement(placement) => {
                self.handle_object_placement(placement).await?
            }
            ClientMessage::OwnedChunks => self.handle_owned_chunks().await?,
            ClientMessage::UserInfo => self.handle_user_info().await?,
        };
        Ok(())
    }
}

/// Message handling
impl WebsocketConnection {
    async fn handle_ping(&self, client_time: f64) -> ServerResult<()> {
        let server_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        self.respond(ServerMessage::Pong {
            client_time,
            server_time,
        });
        Ok(())
    }

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

    async fn handle_chunk_subscription(&self, rect: Rect<i32>) -> ServerResult<()> {
        if rect.area() > self.state.config.max_chunk_subscription_area {
            return Ok(());
        }

        let visibility = self
            .state
            .service
            .chunk
            .visibility_for_user(self.user_id)
            .await?;

        let old_rect = self.state.ws.subscribe_to_chunks(self.id, rect);

        let coords: Vec<_> = rect
            .iter_points()
            .filter(|p| {
                old_rect.is_none_or(|old| !old.contains_point(p)) && visibility.is_visible(p.x, p.y)
            })
            .map(|p| ChunkCoords::new(p.x, p.y))
            .collect();

        for batch in coords.chunks(self.state.config.chunk_batch_size) {
            let chunks = self
                .state
                .data
                .chunk
                .load_or_generate_many(batch, self.state.game_config.world_seed)
                .await?;

            if !chunks.is_empty() {
                self.respond(ServerMessage::Chunks(chunks));
            }
        }

        Ok(())
    }

    async fn handle_object_placement(&self, placement: ObjectPlacement) -> ServerResult<()> {
        let rect = placement.collision_rect();

        self.state
            .service
            .chunk
            .validate_chunks_owned(self.user_id, rect)
            .await?;

        self.state
            .service
            .object
            .validate_placement_collision(rect)
            .await?;

        let chunk_coords = placement.pos.chunk;
        let active = object::ActiveModel::try_from(placement)?;

        let object_model = self.state.data.object.insert(active).await?;
        let object = Object::try_from(object_model)?;
        let chunk_update = ChunkUpdateMessage::update_object(chunk_coords, object);
        self.state.ws.send_chunk_update(chunk_update);

        Ok(())
    }

    async fn handle_owned_chunks(&self) -> ServerResult<()> {
        let owned_chunks = self
            .state
            .service
            .user
            .get_owned_chunk_coords(self.user_id)
            .await?;
        self.respond(ServerMessage::OwnedChunks(owned_chunks));

        Ok(())
    }

    async fn handle_user_info(&self) -> ServerResult<()> {
        let Some(user) = self.state.data.user.find_by_id(self.user_id).await? else {
            return Err(ServerError::Unauthorized);
        };

        self.respond(ServerMessage::UserInfo(
            self.state.service.user.private_info(&user),
        ));

        Ok(())
    }
}
