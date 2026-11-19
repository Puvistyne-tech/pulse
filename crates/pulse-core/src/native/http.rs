use pulse_protocol::{
    AccountId, AddMemberRequest, ApiError, ApiErrorCode, AuthSession, AuthToken, CampusCreated,
    CampusId, CampusInvite, CampusList, CampusMember, CreateCampusRequest, CreateUploadRequest,
    DownloadTickets, DownloadTicketsRequest, JoinCampusRequest, MemberList, RoomId, RoomList,
    SignInRequest, SignUpRequest, UpdateMemberRequest, UploadTicket, api_path,
};
use reqwest::header::CONTENT_TYPE;
use reqwest::{Method, RequestBuilder};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::api::{ApiTransport, decode_api_response};
use crate::media::MediaTransport;

pub const DEFAULT_SERVER_URL: &str = "http://127.0.0.1:8080";
const REQUEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(15);
const UPLOAD_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(300);
const SERVER_URL_VARIABLE: &str = "PULSE_SERVER_URL";
const QUICK_SIGN_IN_VARIABLE: &str = "PULSE_QUICK_SIGN_IN";

pub fn server_url_from_env() -> String {
    std::env::var(SERVER_URL_VARIABLE).unwrap_or_else(|_| DEFAULT_SERVER_URL.to_owned())
}

pub fn quick_sign_in_from_env() -> bool {
    std::env::var(QUICK_SIGN_IN_VARIABLE).is_ok_and(|value| value == "1")
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

    async fn call_empty(builder: RequestBuilder) -> Result<(), ApiError> {
        let (status, body) = Self::execute(builder).await?;
        if (200..300).contains(&status) {
            Ok(())
        } else {
            decode_api_response::<()>(status, &body)
        }
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
        Self::call_empty(builder).await
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

    async fn join_campus(
        &mut self,
        token: AuthToken,
        request: JoinCampusRequest,
    ) -> Result<CampusCreated, ApiError> {
        let builder = self.request(Method::POST, api_path::CAMPUS_MEMBERSHIPS, Some(&token));
        Self::call_with_body(builder, &request).await
    }

    async fn create_invite(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
    ) -> Result<CampusInvite, ApiError> {
        let path = api_path::campus_invites(campus_id);
        Self::call(self.request(Method::POST, &path, Some(&token))).await
    }

    async fn list_members(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
    ) -> Result<MemberList, ApiError> {
        let path = api_path::campus_members(campus_id);
        Self::call(self.request(Method::GET, &path, Some(&token))).await
    }

    async fn add_member(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
        request: AddMemberRequest,
    ) -> Result<CampusMember, ApiError> {
        let path = api_path::campus_members(campus_id);
        Self::call_with_body(self.request(Method::POST, &path, Some(&token)), &request).await
    }

    async fn update_member(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
        account_id: AccountId,
        request: UpdateMemberRequest,
    ) -> Result<CampusMember, ApiError> {
        let path = api_path::campus_member(campus_id, account_id);
        Self::call_with_body(self.request(Method::PATCH, &path, Some(&token)), &request).await
    }

    async fn remove_member(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
        account_id: AccountId,
    ) -> Result<(), ApiError> {
        let path = api_path::campus_member(campus_id, account_id);
        let builder = self.request(Method::DELETE, &path, Some(&token));
        Self::call_empty(builder).await
    }
}

impl MediaTransport for HttpApiTransport {
    async fn create_upload(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
        room_id: RoomId,
        request: CreateUploadRequest,
    ) -> Result<UploadTicket, ApiError> {
        let path = api_path::campus_room_uploads(campus_id, room_id);
        let builder = self.request(Method::POST, &path, Some(&token));
        Self::call_with_body(builder, &request).await
    }

    async fn put_object(&mut self, ticket: UploadTicket, bytes: Vec<u8>) -> Result<(), ApiError> {
        let builder = self
            .http
            .put(&ticket.put_url)
            .timeout(UPLOAD_TIMEOUT)
            .header(CONTENT_TYPE, ticket.content_type)
            .body(bytes);
        let (status, _) = Self::execute(builder).await?;
        if (200..300).contains(&status) {
            Ok(())
        } else {
            Err(ApiError::new(ApiErrorCode::from_http_status(status)))
        }
    }

    async fn download_tickets(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
        request: DownloadTicketsRequest,
    ) -> Result<DownloadTickets, ApiError> {
        let path = api_path::campus_downloads(campus_id);
        let builder = self.request(Method::POST, &path, Some(&token));
        Self::call_with_body(builder, &request).await
    }
}
