use pulse_server::{MIGRATOR, StartupError, config, router, state};
use tracing_subscriber::EnvFilter;

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
    let app_state = state::lazy(&config)?;
    MIGRATOR.run(&app_state.db).await?;
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;
    tracing::info!(addr = %config.bind_addr, "pulse-server listening");
    axum::serve(listener, router(app_state)).await?;
    Ok(())
}
