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
