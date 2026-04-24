use crate::error::{ServerError, ServerResult};
use crate::state::ServerState;
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use lemon_colonies_core::data::entity::object;
use lemon_colonies_core::data::store::Store;
use lemon_colonies_core::game::object::Object;
use lemon_colonies_core::math::rect::Rect;
use lemon_colonies_core::messages::client::object_placement::ObjectPlacement;
use lemon_colonies_core::messages::client::ClientMessage;
use lemon_colonies_core::messages::server::chunk_update::ChunkUpdateMessage;
use lemon_colonies_core::messages::server::ServerMessage;
use tower_sessions_sqlx_store::sqlx::types::Uuid;
use tracing::error;

pub type ConnectionId = Uuid;

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
        while let Some(Ok(message)) = ws_receive.next().await {
            match message {
                Message::Binary(data) => match ClientMessage::from_bytes(data.as_ref()) {
                    Ok(message) => {
                        if let Err(err) = self.handle_client_message(message).await {
                            if err.is_user_error() {
                                self.respond(ServerMessage::Error(err.to_string()));
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
                Message::Close(_) => break,
                _ => {}
            };
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
            ClientMessage::ColonyPositions => self.handle_colony_positions().await?,
            ClientMessage::Hello => self.respond(ServerMessage::Hello),
            ClientMessage::SubscribeToChunks(rect) => self.handle_chunk_subscription(rect).await?,
            ClientMessage::ObjectPlacement(placement) => {
                self.handle_object_placement(placement).await?
            }
            ClientMessage::UserInfo => self.handle_user_info().await?,
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
                old_rect.is_none_or(|old| !old.contains(p)) && visibility.is_visible(p.x, p.y)
            })
            .map(|p| (p.x, p.y))
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

    // ToDo: Check object obstruction
    async fn handle_object_placement(&self, placement: ObjectPlacement) -> ServerResult<()> {
        let chunk_owned = self
            .state
            .service
            .chunk
            .does_user_own_chunk(self.user_id, placement.chunk.0, placement.chunk.1)
            .await?;
        if !chunk_owned {
            return Err(ServerError::ChunkNotOwned);
        };

        let chunk_coords = placement.chunk;
        let active = object::ActiveModel::try_from(placement)?;

        let object_model = self.state.data.object.insert(active).await?;
        let object = Object::try_from(object_model)?;
        let chunk_update = ChunkUpdateMessage::update_object(chunk_coords, object);
        self.state.ws.send_chunk_update(chunk_update);

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
