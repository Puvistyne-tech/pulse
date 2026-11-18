use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::config::Config;
use crate::error::StartupError;
use crate::fanout::Fanout;
use crate::storage::{Storage, StorageConfig};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub valkey: redis::Client,
    pub fanout: Fanout,
    pub storage: Storage,
}

impl AppState {
    pub fn new(db: PgPool, valkey: redis::Client) -> Self {
        let fanout = Fanout::new(valkey.clone());
        let storage = Storage::new(StorageConfig::default()).unwrap_or_else(|error| {
            tracing::error!(%error, "default object storage rejected");
            Storage::unconfigured()
        });
        Self {
            db,
            valkey,
            fanout,
            storage,
        }
    }

    pub fn with_storage(mut self, storage: Storage) -> Self {
        self.storage = storage;
        self
    }
}

pub fn lazy(config: &Config) -> Result<AppState, StartupError> {
    Ok(AppState::new(
        PgPoolOptions::new().connect_lazy(&config.database_url)?,
        redis::Client::open(config.valkey_url.as_str())?,
    ))
}
