use crate::api::error::{ApiError, ApiResult};
use crate::state::ServerState;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use lemon_colonies_core::data::store::Store;
use tower_sessions::Session;
use tower_sessions_sqlx_store::sqlx::types::Uuid;

#[derive(serde::Serialize)]
pub struct MeResponse {
    pub username: String,
}

impl IntoResponse for MeResponse {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

async fn get_me(session: Session, State(state): State<ServerState>) -> ApiResult<MeResponse> {
    let Some(user_id) = session.get::<Uuid>("user_id").await? else {
        return Err(ApiError::Unauthorized);
    };

    let Some(user) = state.data.user.find_by_id(user_id).await? else {
        return Err(ApiError::Unauthorized);
    };

    Ok(MeResponse {
        username: user.username,
    })
}

pub fn router() -> Router<ServerState> {
    Router::<ServerState>::new().route("/", get(get_me))
}
