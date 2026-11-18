use std::io;
use std::sync::Arc;

use pulse_protocol::{
    ApiError, ApiErrorCode, AuthSession, AuthToken, CampusCreated, CampusId, CampusList,
    ClientSnapshot, CreateCampusRequest, RoomList, SignInRequest, SignUpRequest, api_path,
};
use reqwest::{Method, RequestBuilder};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::Mutex;

use crate::api::{ApiTransport, decode_api_response};
use crate::client::{Client, unavailable_snapshot};
use crate::session::SessionEvent;

pub const DEFAULT_SERVER_URL: &str = "http://127.0.0.1:8080";
const SERVER_URL_VARIABLE: &str = "PULSE_SERVER_URL";
const RUNTIME_WORKER_THREADS: usize = 2;

pub fn server_url_from_env() -> String {
    std::env::var(SERVER_URL_VARIABLE).unwrap_or_else(|_| DEFAULT_SERVER_URL.to_owned())
}

#[derive(Debug, Clone)]
pub struct HttpApiTransport {
    base_url: String,
    http: reqwest::Client,
}

impl HttpApiTransport {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            http: reqwest::Client::new(),
        }
    }

    fn request(&self, method: Method, path: &str, token: Option<&AuthToken>) -> RequestBuilder {
        let builder = self
            .http
            .request(method, format!("{}{path}", self.base_url));
        match token {
            Some(token) => builder.bearer_auth(&token.0),
            None => builder,
        }
    }

    async fn execute(builder: RequestBuilder) -> Result<(u16, Vec<u8>), ApiError> {
        let unavailable = |_| ApiError::new(ApiErrorCode::Unavailable);
        let response = builder.send().await.map_err(unavailable)?;
        let status = response.status().as_u16();
        let body = response.bytes().await.map_err(unavailable)?;
        Ok((status, body.to_vec()))
    }

    async fn call<T: DeserializeOwned>(builder: RequestBuilder) -> Result<T, ApiError> {
        let (status, body) = Self::execute(builder).await?;
        decode_api_response(status, &body)
    }

    async fn call_with_body<B: Serialize, T: DeserializeOwned>(
        builder: RequestBuilder,
        body: &B,
    ) -> Result<T, ApiError> {
        Self::call(builder.json(body)).await
    }
}

impl ApiTransport for HttpApiTransport {
    async fn sign_up(&mut self, request: SignUpRequest) -> Result<AuthSession, ApiError> {
        let builder = self.request(Method::POST, api_path::ACCOUNTS, None);
        Self::call_with_body(builder, &request).await
    }

    async fn sign_in(&mut self, request: SignInRequest) -> Result<AuthSession, ApiError> {
        let builder = self.request(Method::POST, api_path::AUTH_TOKENS, None);
        Self::call_with_body(builder, &request).await
    }

    async fn sign_out(&mut self, token: AuthToken) -> Result<(), ApiError> {
        let builder = self.request(Method::DELETE, api_path::CURRENT_AUTH_TOKEN, Some(&token));
        let (status, body) = Self::execute(builder).await?;
        if (200..300).contains(&status) {
            Ok(())
        } else {
            decode_api_response::<()>(status, &body)
        }
    }

    async fn list_campuses(&mut self, token: AuthToken) -> Result<CampusList, ApiError> {
        Self::call(self.request(Method::GET, api_path::CAMPUSES, Some(&token))).await
    }

    async fn create_campus(
        &mut self,
        token: AuthToken,
        request: CreateCampusRequest,
    ) -> Result<CampusCreated, ApiError> {
        let builder = self.request(Method::POST, api_path::CAMPUSES, Some(&token));
        Self::call_with_body(builder, &request).await
    }

    async fn list_rooms(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
    ) -> Result<RoomList, ApiError> {
        let path = api_path::campus_rooms(campus_id);
        Self::call(self.request(Method::GET, &path, Some(&token))).await
    }
}

pub struct NativeClient {
    runtime: Option<Runtime>,
    inner: Arc<Mutex<(Client, HttpApiTransport)>>,
}

impl NativeClient {
    pub fn new(server_url: impl Into<String>) -> io::Result<Self> {
        let runtime = Builder::new_multi_thread()
            .worker_threads(RUNTIME_WORKER_THREADS)
            .enable_all()
            .build()?;
        Ok(Self {
            runtime: Some(runtime),
            inner: Arc::new(Mutex::new((
                Client::default(),
                HttpApiTransport::new(server_url),
            ))),
        })
    }

    pub fn from_env() -> io::Result<Self> {
        Self::new(server_url_from_env())
    }

    async fn run(&self, event: Option<SessionEvent>) -> ClientSnapshot {
        let Some(runtime) = self.runtime.as_ref() else {
            return unavailable_snapshot();
        };
        let inner = Arc::clone(&self.inner);
        let handle = runtime.spawn(async move {
            let mut guard = inner.lock().await;
            let (client, api) = &mut *guard;
            match event {
                Some(event) => client.apply(api, event).await,
                None => client.snapshot(),
            }
        });
        handle.await.unwrap_or_else(|_| unavailable_snapshot())
    }

    pub async fn snapshot(&self) -> ClientSnapshot {
        self.run(None).await
    }

    pub async fn sign_up(&self, request: SignUpRequest) -> ClientSnapshot {
        self.run(Some(SessionEvent::SignUpRequested(request))).await
    }

    pub async fn sign_in(&self, request: SignInRequest) -> ClientSnapshot {
        self.run(Some(SessionEvent::SignInRequested(request))).await
    }

    pub async fn sign_out(&self) -> ClientSnapshot {
        self.run(Some(SessionEvent::SignOutRequested)).await
    }

    pub async fn refresh_campuses(&self) -> ClientSnapshot {
        self.run(Some(SessionEvent::CampusesRequested)).await
    }

    pub async fn create_campus(&self, request: CreateCampusRequest) -> ClientSnapshot {
        self.run(Some(SessionEvent::CreateCampusRequested(request)))
            .await
    }

    pub async fn select_campus(&self, campus_id: CampusId) -> ClientSnapshot {
        self.run(Some(SessionEvent::CampusSelected(campus_id)))
            .await
    }
}

impl Drop for NativeClient {
    fn drop(&mut self) {
        if let Some(runtime) = self.runtime.take() {
            runtime.shutdown_background();
        }
    }
}
