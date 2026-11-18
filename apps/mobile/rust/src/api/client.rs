use std::future::Future;
use std::io;
use std::sync::LazyLock;

use pulse_core::{NativeClient, unavailable_snapshot};
use pulse_protocol::{CampusId, ClientSnapshot, CreateCampusRequest, SignInRequest, SignUpRequest};

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
