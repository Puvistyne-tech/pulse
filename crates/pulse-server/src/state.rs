use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::config::Config;
use crate::error::StartupError;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub valkey: redis::Client,
}

pub fn lazy(config: &Config) -> Result<AppState, StartupError> {
    Ok(AppState {
        db: PgPoolOptions::new().connect_lazy(&config.database_url)?,
        valkey: redis::Client::open(config.valkey_url.as_str())?,
    })
}
