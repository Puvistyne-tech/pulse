use std::net::SocketAddr;
use std::time::Duration;

use crate::error::StartupError;
use crate::lounge::LoungeConfig;
use crate::storage::StorageConfig;
use crate::stream::StreamConfig;

const DEFAULT_BIND_ADDR: &str = "127.0.0.1:8080";
pub const DEFAULT_DATABASE_URL: &str = "postgres://pulse:pulse@127.0.0.1:55432/pulse";
const DEFAULT_VALKEY_URL: &str = "redis://127.0.0.1:6379";
const DEFAULT_REMINDER_SCAN_MS: u64 = 1000;

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

pub fn storage_from_env() -> StorageConfig {
    let defaults = StorageConfig::default();
    let endpoint = env_or("PULSE_S3_ENDPOINT", &defaults.endpoint);
    StorageConfig {
        public_endpoint: env_or("PULSE_S3_PUBLIC_ENDPOINT", &endpoint),
        endpoint,
        bucket: env_or("PULSE_S3_BUCKET", &defaults.bucket),
        region: env_or("PULSE_S3_REGION", &defaults.region),
        access_key: env_or("PULSE_S3_ACCESS_KEY", &defaults.access_key),
        secret_key: env_or("PULSE_S3_SECRET_KEY", &defaults.secret_key),
    }
}

pub fn reminder_scan_interval_from_env() -> Duration {
    let millis = std::env::var("PULSE_REMINDER_SCAN_MS")
        .ok()
        .and_then(|text| text.parse::<u64>().ok())
        .filter(|millis| *millis > 0)
        .unwrap_or(DEFAULT_REMINDER_SCAN_MS);
    Duration::from_millis(millis)
}

fn millis_from_env(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|text| text.parse::<u64>().ok())
        .filter(|millis| *millis > 0)
        .unwrap_or(default)
}

pub fn lounge_from_env() -> LoungeConfig {
    let defaults = LoungeConfig::default();
    LoungeConfig {
        livekit_url: env_or("PULSE_LIVEKIT_URL", &defaults.livekit_url),
        api_key: env_or("PULSE_LIVEKIT_API_KEY", &defaults.api_key),
        api_secret: env_or("PULSE_LIVEKIT_API_SECRET", &defaults.api_secret),
        presence_ttl_ms: millis_from_env("PULSE_LOUNGE_PRESENCE_TTL_MS", defaults.presence_ttl_ms),
        ticket_ttl_ms: defaults.ticket_ttl_ms,
    }
}

pub fn stream_from_env() -> StreamConfig {
    StreamConfig {
        idle_timeout_ms: millis_from_env(
            "PULSE_STREAM_IDLE_MS",
            StreamConfig::default().idle_timeout_ms,
        ),
    }
}
