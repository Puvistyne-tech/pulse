use axum::Router;
use axum::extract::{State, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::{delete, get, post};
use pulse_protocol::api_path;

use crate::state::AppState;
use crate::{auth, campus, ws};

async fn healthz() -> &'static str {
    "ok"
}

async fn upgrade(State(state): State<AppState>, socket: WebSocketUpgrade) -> Response {
    socket.on_upgrade(move |socket| ws::serve_socket(socket, state))
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route(api_path::WEBSOCKET, get(upgrade))
        .route(api_path::ACCOUNTS, post(auth::sign_up))
        .route(api_path::AUTH_TOKENS, post(auth::sign_in))
        .route(api_path::CURRENT_AUTH_TOKEN, delete(auth::sign_out))
        .route(
            api_path::CAMPUSES,
            get(campus::list_campuses).post(campus::create_campus),
        )
        .route(
            api_path::CAMPUS_ROOMS_ROUTE,
            get(campus::list_rooms).post(campus::create_room),
        )
        .with_state(state)
}
