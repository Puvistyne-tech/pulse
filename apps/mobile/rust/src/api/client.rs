use std::future::Future;
use std::io;
use std::sync::LazyLock;

use crate::frb_generated::StreamSink;
use pulse_core::native::NativeClient;
use pulse_core::unavailable_snapshot;
use pulse_protocol::{
    CampusId, ClientNonce, ClientSnapshot, CreateCampusRequest, InboxStep, MessageId,
    ReactionAction, RoomId, SignInRequest, SignUpRequest, SubscriptionState,
};

static CLIENT: LazyLock<io::Result<NativeClient>> = LazyLock::new(NativeClient::from_env);

async fn with_client<F, Fut>(run: F) -> ClientSnapshot
where
    F: FnOnce(&'static NativeClient) -> Fut,
    Fut: Future<Output = ClientSnapshot>,
{
    match &*CLIENT {
        Ok(client) => run(client).await,
        Err(_) => unavailable_snapshot(),
    }
}

pub async fn client_snapshot() -> ClientSnapshot {
    with_client(|client| client.snapshot()).await
}

pub async fn sign_up(request: SignUpRequest) -> ClientSnapshot {
    with_client(|client| client.sign_up(request)).await
}

pub async fn sign_in(request: SignInRequest) -> ClientSnapshot {
    with_client(|client| client.sign_in(request)).await
}

pub async fn sign_out() -> ClientSnapshot {
    with_client(|client| client.sign_out()).await
}

pub async fn refresh_campuses() -> ClientSnapshot {
    with_client(|client| client.refresh_campuses()).await
}

pub async fn create_campus(request: CreateCampusRequest) -> ClientSnapshot {
    with_client(|client| client.create_campus(request)).await
}

pub async fn select_campus(campus_id: CampusId) -> ClientSnapshot {
    with_client(|client| client.select_campus(campus_id)).await
}

pub async fn select_room(room_id: RoomId) -> ClientSnapshot {
    with_client(|client| client.select_room(room_id)).await
}

pub async fn send_message(room_id: RoomId, body: String) -> ClientSnapshot {
    with_client(|client| client.send_message(room_id, body)).await
}

pub async fn discard_message(client_nonce: ClientNonce) -> ClientSnapshot {
    with_client(|client| client.discard_message(client_nonce)).await
}

pub async fn send_reply(
    room_id: RoomId,
    root_message_id: MessageId,
    body: String,
) -> ClientSnapshot {
    with_client(|client| client.send_reply(room_id, root_message_id, body)).await
}

pub async fn react(
    room_id: RoomId,
    message_id: MessageId,
    emoji: String,
    action: ReactionAction,
) -> ClientSnapshot {
    with_client(|client| client.react(room_id, message_id, emoji, action)).await
}

pub async fn set_topic_title(
    room_id: RoomId,
    root_message_id: MessageId,
    topic_title: String,
) -> ClientSnapshot {
    with_client(|client| client.set_topic_title(room_id, root_message_id, topic_title)).await
}

pub async fn set_thread_subscription(
    room_id: RoomId,
    root_message_id: MessageId,
    state: SubscriptionState,
) -> ClientSnapshot {
    with_client(|client| client.set_thread_subscription(room_id, root_message_id, state)).await
}

pub async fn open_thread(room_id: RoomId, root_message_id: MessageId) -> ClientSnapshot {
    with_client(|client| client.open_thread(room_id, root_message_id)).await
}

pub async fn close_thread() -> ClientSnapshot {
    with_client(|client| client.close_thread()).await
}

pub async fn toggle_inbox() -> ClientSnapshot {
    with_client(|client| client.toggle_inbox()).await
}

pub async fn move_inbox_selection(step: InboxStep) -> ClientSnapshot {
    with_client(|client| client.move_inbox_selection(step)).await
}

pub async fn archive_inbox_selection() -> ClientSnapshot {
    with_client(|client| client.archive_inbox_selection()).await
}

pub async fn start_inbox_reply() -> ClientSnapshot {
    with_client(|client| client.start_inbox_reply()).await
}

pub async fn cancel_inbox_reply() -> ClientSnapshot {
    with_client(|client| client.cancel_inbox_reply()).await
}

pub fn watch_snapshots(sink: StreamSink<ClientSnapshot>) {
    match &*CLIENT {
        Ok(client) => client.forward_snapshots(move |snapshot| sink.add(snapshot).is_ok()),
        Err(_) => {
            let _ = sink.add(unavailable_snapshot());
        }
    }
}
