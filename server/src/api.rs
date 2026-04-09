use crate::state::ServerState;
use axum::Router;

mod error;
mod layers;
mod routes;

pub async fn build(state: &ServerState) -> Router<ServerState> {
    Router::new().merge(routes::build_routes(state).await)
}
