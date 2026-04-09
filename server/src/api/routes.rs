use crate::api::layers::session::build_session_layer;
use crate::state::ServerState;
use axum::Router;

mod auth;
mod me;

pub async fn build_routes(state: &ServerState) -> Router<ServerState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/me", me::router())
        .layer(build_session_layer(state).await)
}
