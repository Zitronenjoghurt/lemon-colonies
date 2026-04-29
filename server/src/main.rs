use crate::config::Config;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod api;
mod config;
mod error;
mod integrations;
mod layers;
mod state;
mod websocket;

#[tokio::main]
async fn main() {
    init_logging();
    info!("Starting server...");

    let config = Config::from_env().unwrap();
    let state = state::ServerState::new(config).await.unwrap();
    let api = api::build().await;

    let session_layer = layers::session::build_session_layer(&state).await;

    let router = Router::new()
        .route("/ws", get(websocket::ws_handler))
        .nest("/api", api)
        .fallback_service(ServeDir::new("./static"))
        .layer(session_layer)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 50434));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Listening on {}", addr);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutting down...");
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}

pub fn server_time() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}
