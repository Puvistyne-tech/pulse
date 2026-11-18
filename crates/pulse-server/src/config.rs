use std::net::SocketAddr;

use crate::error::StartupError;

const DEFAULT_BIND_ADDR: &str = "127.0.0.1:8080";
pub const DEFAULT_DATABASE_URL: &str = "postgres://pulse:pulse@127.0.0.1:55432/pulse";
const DEFAULT_VALKEY_URL: &str = "redis://127.0.0.1:6379";

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    pub valkey_url: String,
}

fn env_or(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_owned())
}

pub fn from_env() -> Result<Config, StartupError> {
    Ok(Config {
        bind_addr: env_or("PULSE_BIND_ADDR", DEFAULT_BIND_ADDR).parse()?,
        database_url: env_or("DATABASE_URL", DEFAULT_DATABASE_URL),
        valkey_url: env_or("VALKEY_URL", DEFAULT_VALKEY_URL),
    })
}
