use crate::state::ServerState;
use axum::Router;

mod discord;

pub fn router() -> Router<ServerState> {
    Router::new().nest("/discord", discord::router())
}
