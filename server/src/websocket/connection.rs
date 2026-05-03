use crate::config::Config;
use crate::error::{ServerError, ServerResult};
use crate::server_time;
use crate::state::ServerState;
use crate::websocket::rate_limiter::{RateLimitResult, RateLimiter};
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitStream;
use futures_util::{StreamExt, TryStreamExt};
use lemon_colonies_core::data::entity::object;
use lemon_colonies_core::data::store::Store;
use lemon_colonies_core::error::CoreError;
use lemon_colonies_core::game::object::command::{
    ObjectCommand, ObjectCommandResult, ObjectCommandResultKind,
};
use lemon_colonies_core::game::object::Object;
use lemon_colonies_core::game::resource::ResourceId;
use lemon_colonies_core::math::coords::ChunkCoords;
use lemon_colonies_core::math::rect::Rect;
use lemon_colonies_core::messages::client::object_placement::ObjectPlacement;
use lemon_colonies_core::messages::client::object_purchase::ObjectPurchase;
use lemon_colonies_core::messages::client::ClientMessage;
use lemon_colonies_core::messages::server::chunk_update::ChunkUpdate;
use lemon_colonies_core::messages::server::ServerMessage;
use metrics::{counter, gauge};
use std::collections::HashSet;
use std::ops::ControlFlow;
use std::time::Duration;
use tokio::time::timeout;
use tower_sessions_sqlx_store::sqlx::types::Uuid;
use tracing::{error, info};

pub type ConnectionId = Uuid;

const IDLE_TIMEOUT: Duration = Duration::from_secs(60);

pub struct WebsocketConnection {
    id: ConnectionId,
    user_id: Uuid,
    state: ServerState,
    rate_limiter: RateLimiter,
}

impl WebsocketConnection {
    pub fn new(config: &Config, id: ConnectionId, user_id: Uuid, state: ServerState) -> Self {
        Self {
            id,
            user_id,
            state,
            rate_limiter: RateLimiter::new(config),
        }
    }

    pub async fn handle_receive(mut self, mut ws_receive: SplitStream<WebSocket>) {
        gauge!("ws.active_connections").increment(1.0);

        loop {
            let msg = match timeout(IDLE_TIMEOUT, ws_receive.next()).await {
                Ok(Some(Ok(message))) => message,
                Ok(Some(Err(err))) => {
                    error!("[{}] WebSocket error: {err}", self.user_id);
                    counter!("ws.disconnect_total", "reason" => "error").increment(1);
                    break;
                }
                Ok(None) => {
                    info!("[{}] WebSocket stream closed", self.user_id);
                    counter!("ws.disconnect_total", "reason" => "stream_closed").increment(1);
                    break;
                }
                Err(_) => {
                    info!(
                        "[{}] Connection timed out (idle for {IDLE_TIMEOUT:?})",
                        self.user_id
                    );
                    counter!("ws.disconnect_total", "reason" => "idle_timeout").increment(1);
                    break;
                }
            };

            match msg {
                Message::Binary(data) if self.handle_binary(&data).await.is_break() => break,
                Message::Close(reason) => {
                    info!("[{}] Client closed connection: {reason:?}", self.user_id);
                    counter!("ws.disconnect_total", "reason" => "client_close").increment(1);
                    break;
                }
                _ => {}
            }
        }

        gauge!("ws.active_connections").decrement(1.0);
    }

    async fn handle_binary(&mut self, data: &[u8]) -> ControlFlow<()> {
        metrics::histogram!("ws.inbound_size_bytes").record(data.len() as f64);

        let message = match ClientMessage::from_bytes(data) {
            Ok(msg) => msg,
            Err(e) => {
                error!("[{}] Failed to decode message: {e}", self.user_id);
                return ControlFlow::Continue(());
            }
        };

        match self.rate_limiter.check(&self.state.config, &message) {
            RateLimitResult::Allow => {}
            RateLimitResult::Drop => {
                self.respond(ServerMessage::Error(
                    "Too many requests. Please wait a moment.".into(),
                ));
                counter!("ws.rate_limit_total", "action" => "drop").increment(1);
                return ControlFlow::Continue(());
            }
            RateLimitResult::Warn => {
                self.respond(ServerMessage::Error(
                    "You are being rate limited. Continued excessive requests may result in a disconnect.".into(),
                ));
                counter!("ws.rate_limit_total", "action" => "warn").increment(1);
                return ControlFlow::Continue(());
            }
            RateLimitResult::Disconnect => {
                let infractions = self
                    .state
                    .service
                    .user
                    .log_rate_limit_infraction(self.user_id)
                    .await
                    .unwrap_or_default();
                self.respond(ServerMessage::Error(
                    "Connection closed due to excessive requests. Continued abuse will lead to a permanent ban.".into(),
                ));
                info!(
                    "[{}] Disconnected for rate limit abuse, {infractions} infractions.",
                    self.user_id
                );
                counter!("ws.disconnect_total", "reason" => "rate_limit").increment(1);
                counter!("ws.rate_limit_total", "action" => "disconnect").increment(1);
                return ControlFlow::Break(());
            }
        }

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

        ControlFlow::Continue(())
    }

    fn respond(&self, message: ServerMessage) {
        self.state.ws.send_to_connection(self.id, message);
    }

    fn send_to_user(&self, user_id: Uuid, message: ServerMessage) {
        self.state.ws.send_to_user(user_id, message);
    }

    async fn handle_client_message(&self, message: ClientMessage) -> ServerResult<()> {
        let msg_type = message.name();
        metrics::counter!("ws.inbound_total", "type" => msg_type).increment(1);

        let start = std::time::Instant::now();

        match message {
            ClientMessage::Ping { client_time } => self.handle_ping(client_time).await?,
            ClientMessage::AllResources => self.handle_all_resources().await?,
            ClientMessage::ColonyPositions => self.handle_colony_positions().await?,
            ClientMessage::Resources(resource_ids) => self.handle_resources(resource_ids).await?,
            ClientMessage::SubscribeToChunks(rect) => self.handle_chunk_subscription(rect).await?,
            ClientMessage::ObjectCommand(command) => self.handle_object_command(command).await?,
            ClientMessage::ObjectsInChunks(chunks) => self.handle_objects_in_chunks(chunks).await?,
            ClientMessage::ObjectPlacement(placement) => {
                self.handle_object_placement(placement).await?
            }
            ClientMessage::ObjectPurchase(purchase) => {
                self.handle_object_purchase(purchase).await?
            }
            ClientMessage::PlayerOwnedChunks => self.handle_player_owned_chunks().await?,
            ClientMessage::UserInfo => self.handle_user_info().await?,
        };

        let elapsed = start.elapsed().as_secs_f64();
        metrics::histogram!("ws.inbound_duration_secs", "type" => msg_type).record(elapsed);

        Ok(())
    }
}

/// Message handling
impl WebsocketConnection {
    async fn handle_ping(&self, client_time: f64) -> ServerResult<()> {
        self.respond(ServerMessage::Pong {
            client_time,
            server_time: server_time(),
        });
        Ok(())
    }

    async fn handle_all_resources(&self) -> ServerResult<()> {
        let resources = self.state.data.user_resources.get_all(self.user_id).await?;
        self.respond(ServerMessage::ResourceUpdateAll(resources));
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

        let mut owned_chunks = self
            .state
            .data
            .colony_chunk
            .find_owned_coords_in_rect(rect)
            .await?;
        visibility.evict_invisible_chunk_coords(&mut owned_chunks);
        self.send_to_user(self.user_id, ServerMessage::OwnedChunks(owned_chunks));

        Ok(())
    }

    async fn handle_resources(&self, resource_ids: HashSet<ResourceId>) -> ServerResult<()> {
        let resources = self
            .state
            .data
            .user_resources
            .get_multiple(self.user_id, &resource_ids)
            .await?;
        self.respond(ServerMessage::ResourceUpdate(resources));
        Ok(())
    }

    async fn handle_object_command(&self, command: ObjectCommand) -> ServerResult<()> {
        let server_time = server_time();
        if let Some((result, model)) = self
            .state
            .service
            .object
            .handle_command(server_time, command)
            .await?
        {
            let dirty = result.dirty;
            self.handle_object_command_result(result).await?;
            if dirty {
                let object = Object::try_from(model)?;
                let chunk_update = ChunkUpdate::update_object(object.pos.chunk, object);
                self.state.ws.send_chunk_update(self.user_id, chunk_update);
            }
        }

        Ok(())
    }

    async fn handle_objects_in_chunks(&self, chunks: HashSet<ChunkCoords>) -> ServerResult<()> {
        if chunks.is_empty() || chunks.len() > self.state.config.max_object_fetch_chunk_count {
            return Ok(());
        };

        let owned_chunks = self
            .state
            .service
            .user
            .get_owned_chunk_coords(self.user_id)
            .await?;

        let mut object_stream = self.state.data.object.stream_by_chunks(&chunks).await?;
        let mut batch = Vec::with_capacity(self.state.config.object_fetch_batch_size);

        let now = server_time();
        while let Some(model) = object_stream.try_next().await? {
            let mut object = Object::try_from(model)?;
            object.tick(now);
            if !owned_chunks.contains(&object.pos.chunk) {
                object.anonymize();
            }
            batch.push(object);
            if batch.len() >= self.state.config.object_fetch_batch_size {
                self.respond(ServerMessage::Objects(std::mem::replace(
                    &mut batch,
                    Vec::with_capacity(self.state.config.object_fetch_batch_size),
                )));
            }
        }

        if !batch.is_empty() {
            self.respond(ServerMessage::Objects(batch));
        }

        Ok(())
    }

    async fn handle_object_placement(&self, _placement: ObjectPlacement) -> ServerResult<()> {
        // ToDo: Rework for admin-use only
        Ok(())
    }

    async fn handle_object_purchase(&self, purchase: ObjectPurchase) -> ServerResult<()> {
        let rect = purchase.collision_rect();

        let txn = self.state.data.begin_txn().await?;
        self.state
            .data
            .user_resources
            .adjust(&txn, self.user_id, &purchase.kind.resource_adjustments())
            .await?;

        self.state
            .service
            .chunk
            .validate_all_chunks_owned_in_rect(self.user_id, rect)
            .await?;

        self.state
            .service
            .object
            .validate_placement_collision(rect)
            .await?;

        let chunk_coords = purchase.pos.chunk;
        let active = object::ActiveModel::try_from(purchase)?;

        let object_model = self.state.data.object.insert(active).await?;
        let object = Object::try_from(object_model)?;
        let chunk_update = ChunkUpdate::update_object(chunk_coords, object);
        self.state.ws.send_chunk_update(self.user_id, chunk_update);

        txn.commit().await.map_err(CoreError::from)?;

        Ok(())
    }

    async fn handle_player_owned_chunks(&self) -> ServerResult<()> {
        let owned_chunks = self
            .state
            .service
            .user
            .get_owned_chunk_coords(self.user_id)
            .await?;
        self.respond(ServerMessage::PlayerOwnedChunks(owned_chunks));

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

// Object command result handling
impl WebsocketConnection {
    async fn handle_object_command_result(&self, result: ObjectCommandResult) -> ServerResult<()> {
        match result.kind {
            ObjectCommandResultKind::None => {}
            ObjectCommandResultKind::ReceiveResources { id, amount } => {
                self.handle_object_command_receive_resources(id, amount)
                    .await?
            }
        }
        Ok(())
    }

    async fn handle_object_command_receive_resources(
        &self,
        id: ResourceId,
        amount: u64,
    ) -> ServerResult<()> {
        let txn = self.state.data.begin_txn().await?;
        let updated = self
            .state
            .data
            .user_resources
            .adjust(&txn, self.user_id, &[(id, amount as i64)])
            .await?;
        txn.commit().await.map_err(CoreError::from)?;
        self.respond(ServerMessage::ResourceUpdate(updated));
        Ok(())
    }
}
