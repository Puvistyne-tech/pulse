use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::config::Config;
use crate::error::StartupError;
use crate::fanout::Fanout;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub valkey: redis::Client,
    pub fanout: Fanout,
}

impl AppState {
    pub fn new(db: PgPool, valkey: redis::Client) -> Self {
        let fanout = Fanout::new(valkey.clone());
        Self { db, valkey, fanout }
    }
}

pub fn lazy(config: &Config) -> Result<AppState, StartupError> {
    Ok(AppState::new(
        PgPoolOptions::new().connect_lazy(&config.database_url)?,
        redis::Client::open(config.valkey_url.as_str())?,
    ))
}
