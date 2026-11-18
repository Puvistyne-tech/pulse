use argon2::password_hash::rand_core::{OsRng, RngCore};
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Argon2, password_hash};
use axum::Json;
use axum::extract::{FromRequestParts, State};
use axum::http::StatusCode;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use pulse_protocol::{
    AccountId, AccountProfile, ApiError, ApiErrorCode, AuthSession, AuthToken, InputField,
    SignInRequest, SignUpRequest, Validate, normalize_email,
};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{ApiFailure, ApiJson, is_unique_violation};
use crate::state::AppState;

pub const AUTH_TOKEN_TTL_DAYS: i32 = 30;
const TOKEN_BYTES: usize = 32;
const BEARER_PREFIX: &str = "Bearer ";

fn internal() -> ApiFailure {
    ApiFailure(ApiError::new(ApiErrorCode::Internal))
}

fn unauthenticated() -> ApiFailure {
    ApiFailure(ApiError::new(ApiErrorCode::Unauthenticated))
}

pub async fn hash_password(password: String) -> Result<String, ApiFailure> {
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    })
    .await
    .map_err(|_| internal())?
    .map_err(|_| internal())
}

pub async fn verify_password(password: String, phc: String) -> Result<bool, ApiFailure> {
    let outcome = tokio::task::spawn_blocking(move || {
        let parsed = PasswordHash::new(&phc)?;
        Argon2::default().verify_password(password.as_bytes(), &parsed)
    })
    .await
    .map_err(|_| internal())?;
    match outcome {
        Ok(()) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(_) => Err(internal()),
    }
}

pub fn mint_token() -> AuthToken {
    let mut bytes = [0u8; TOKEN_BYTES];
    OsRng.fill_bytes(&mut bytes);
    AuthToken(URL_SAFE_NO_PAD.encode(bytes))
}

pub fn token_digest(token: &AuthToken) -> Vec<u8> {
    Sha256::digest(token.0.as_bytes()).to_vec()
}

pub async fn issue_token(db: &PgPool, account_id: AccountId) -> Result<AuthToken, ApiFailure> {
    let token = mint_token();
    sqlx::query(
        "INSERT INTO auth_tokens (token_hash, account_id, expires_at) \
         VALUES ($1, $2, now() + make_interval(days => $3))",
    )
    .bind(token_digest(&token))
    .bind(account_id.as_uuid())
    .bind(AUTH_TOKEN_TTL_DAYS)
    .execute(db)
    .await?;
    Ok(token)
}

pub async fn resolve_token(
    db: &PgPool,
    token: &AuthToken,
) -> Result<Option<AccountId>, sqlx::Error> {
    let account = sqlx::query_scalar::<_, Uuid>(
        "SELECT account_id FROM auth_tokens WHERE token_hash = $1 AND expires_at > now()",
    )
    .bind(token_digest(token))
    .fetch_optional(db)
    .await?;
    Ok(account.map(AccountId))
}

pub struct Authenticated {
    pub account_id: AccountId,
    pub token: AuthToken,
}

impl FromRequestParts<AppState> for Authenticated {
    type Rejection = ApiFailure;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix(BEARER_PREFIX))
            .map(|value| AuthToken(value.trim().to_owned()))
            .ok_or_else(unauthenticated)?;
        let account_id = resolve_token(&state.db, &token)
            .await?
            .ok_or_else(unauthenticated)?;
        Ok(Self { account_id, token })
    }
}

pub async fn sign_up(
    State(state): State<AppState>,
    ApiJson(request): ApiJson<SignUpRequest>,
) -> Result<(StatusCode, Json<AuthSession>), ApiFailure> {
    request.validate()?;
    let email = normalize_email(&request.email).map_err(ApiError::invalid)?;
    let display_name = request.display_name.trim().to_owned();
    let password_hash = hash_password(request.password).await?;
    let account_id = AccountId::generate();
    sqlx::query(
        "INSERT INTO accounts (account_id, email, display_name, password_hash) \
         VALUES ($1, $2, $3, $4)",
    )
    .bind(account_id.as_uuid())
    .bind(&email)
    .bind(&display_name)
    .bind(&password_hash)
    .execute(&state.db)
    .await
    .map_err(|error| {
        if is_unique_violation(&error) {
            ApiFailure(ApiError::conflict(InputField::Email))
        } else {
            ApiFailure::from(error)
        }
    })?;
    let token = issue_token(&state.db, account_id).await?;
    let account = AccountProfile {
        account_id,
        email,
        display_name,
    };
    Ok((StatusCode::CREATED, Json(AuthSession { account, token })))
}

pub async fn sign_in(
    State(state): State<AppState>,
    ApiJson(request): ApiJson<SignInRequest>,
) -> Result<Json<AuthSession>, ApiFailure> {
    request.validate()?;
    let email = normalize_email(&request.email).map_err(ApiError::invalid)?;
    let row = sqlx::query_as::<_, (Uuid, String, String)>(
        "SELECT account_id, display_name, password_hash FROM accounts WHERE email = $1",
    )
    .bind(&email)
    .fetch_optional(&state.db)
    .await?;
    let Some((account_id, display_name, password_hash)) = row else {
        hash_password(request.password).await?;
        return Err(unauthenticated());
    };
    if !verify_password(request.password, password_hash).await? {
        return Err(unauthenticated());
    }
    let account_id = AccountId(account_id);
    let token = issue_token(&state.db, account_id).await?;
    let account = AccountProfile {
        account_id,
        email,
        display_name,
    };
    Ok(Json(AuthSession { account, token }))
}

pub async fn sign_out(
    State(state): State<AppState>,
    authenticated: Authenticated,
) -> Result<StatusCode, ApiFailure> {
    sqlx::query("DELETE FROM auth_tokens WHERE token_hash = $1")
        .bind(token_digest(&authenticated.token))
        .execute(&state.db)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
