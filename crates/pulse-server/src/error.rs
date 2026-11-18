use axum::Json;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use pulse_protocol::{ApiError, ApiErrorCode};
use serde::de::DeserializeOwned;

#[derive(Debug, thiserror::Error)]
pub enum StartupError {
    #[error("invalid bind address: {0}")]
    BindAddr(#[from] std::net::AddrParseError),
    #[error("database configuration rejected: {0}")]
    Database(#[from] sqlx::Error),
    #[error("valkey configuration rejected: {0}")]
    Valkey(#[from] redis::RedisError),
    #[error("migration failed: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("listener failed: {0}")]
    Io(#[from] std::io::Error),
}

const UNIQUE_VIOLATION: &str = "23505";

#[derive(Debug)]
pub struct ApiFailure(pub ApiError);

impl From<ApiError> for ApiFailure {
    fn from(error: ApiError) -> Self {
        Self(error)
    }
}

impl From<sqlx::Error> for ApiFailure {
    fn from(error: sqlx::Error) -> Self {
        tracing::error!(%error, "database failure");
        Self(ApiError::new(ApiErrorCode::Internal))
    }
}

impl IntoResponse for ApiFailure {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.0.code.http_status())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self.0)).into_response()
    }
}

pub struct ApiJson<T>(pub T);

impl<S, T> FromRequest<S> for ApiJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = ApiFailure;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        Json::<T>::from_request(request, state)
            .await
            .map(|Json(value)| Self(value))
            .map_err(|_| ApiFailure(ApiError::new(ApiErrorCode::InvalidInput)))
    }
}

pub fn is_unique_violation(error: &sqlx::Error) -> bool {
    error
        .as_database_error()
        .and_then(|database_error| database_error.code())
        .is_some_and(|code| code == UNIQUE_VIOLATION)
}
