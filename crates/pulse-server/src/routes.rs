use axum::Router;
use axum::extract::{State, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::{delete, get, patch, post};
use pulse_protocol::api_path;

use crate::state::AppState;
use crate::{auth, campus, media, member, note, ws};

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
        .route(api_path::CAMPUS_MEMBERSHIPS, post(campus::join_campus))
        .route(api_path::CAMPUS_INVITES_ROUTE, post(campus::create_invite))
        .route(
            api_path::CAMPUS_NOTES_ROUTE,
            get(note::list_notes).post(note::start_note),
        )
        .route(
            api_path::CAMPUS_MEMBERS_ROUTE,
            get(member::list_members).post(member::add_member),
        )
        .route(
            api_path::CAMPUS_MEMBER_ROUTE,
            patch(member::update_member).delete(member::remove_member),
        )
        .route(
            api_path::CAMPUS_ROOMS_ROUTE,
            get(campus::list_rooms).post(campus::create_room),
        )
        .route(
            api_path::CAMPUS_ROOM_UPLOADS_ROUTE,
            post(media::create_upload),
        )
        .route(
            api_path::CAMPUS_DOWNLOADS_ROUTE,
            post(media::download_tickets),
        )
        .with_state(state)
}
