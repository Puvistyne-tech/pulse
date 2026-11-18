use pulse_protocol::{
    ApiError, ApiErrorCode, AuthSession, AuthToken, CampusCreated, CampusId, CampusList,
    CreateCampusRequest, RoomList, SignInRequest, SignUpRequest, api_path,
};
use reqwest::{Method, RequestBuilder};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::api::{ApiTransport, decode_api_response};

pub const DEFAULT_SERVER_URL: &str = "http://127.0.0.1:8080";
const REQUEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(15);
const SERVER_URL_VARIABLE: &str = "PULSE_SERVER_URL";

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
            http: reqwest::Client::builder()
                .timeout(REQUEST_TIMEOUT)
                .build()
                .unwrap_or_default(),
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
