use axum::Router;
use axum::extract::WebSocketUpgrade;
use axum::response::Response;
use axum::routing::get;

use crate::state::AppState;
use crate::ws;

async fn healthz() -> &'static str {
    "ok"
}

async fn upgrade(socket: WebSocketUpgrade) -> Response {
    socket.on_upgrade(ws::serve_socket)
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/ws", get(upgrade))
        .with_state(state)
}
