use pulse_server::seed::seed_dev_campus;
use pulse_server::shelf::spawn_reminder_loop;
use pulse_server::storage::Storage;
use pulse_server::{MIGRATOR, StartupError, config, router, state};
use tracing_subscriber::EnvFilter;

const SEED_ARGUMENT: &str = "--seed-demo";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    if let Err(error) = run().await {
        tracing::error!(%error, "pulse-server stopped");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), StartupError> {
    let config = config::from_env()?;
    let storage = Storage::new(config::storage_from_env())?;
    let lounge = config::lounge_from_env();
    if lounge.api_secret.len() < 32 {
        tracing::warn!("livekit api secret is shorter than 32 characters");
    }
    let app_state = state::lazy(&config)?
        .with_storage(storage)
        .with_lounge(lounge)
        .with_streams(config::stream_from_env());
    MIGRATOR.run(&app_state.db).await?;
    if std::env::args().any(|argument| argument == SEED_ARGUMENT) {
        match seed_dev_campus(&app_state.db).await {
            Ok(campus_id) => tracing::info!(%campus_id, "seeded demo campus"),
            Err(failure) => tracing::error!(?failure, "seeding demo campus failed"),
        }
    }
    if let Err(error) = app_state.storage.ensure_bucket().await {
        tracing::warn!(%error, "object storage bucket unavailable");
    }
    spawn_reminder_loop(app_state.clone(), config::reminder_scan_interval_from_env());
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;
    tracing::info!(addr = %config.bind_addr, "pulse-server listening");
    axum::serve(listener, router(app_state)).await?;
    Ok(())
}
