use goxlr_profile::default_config_dir;
use std::fs;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

pub fn init() -> tauri::Result<WorkerGuard> {
    let log_dir = default_config_dir()
        .map_err(|error| tauri::Error::Anyhow(anyhow::anyhow!(error.to_string())))?
        .join("logs");

    fs::create_dir_all(&log_dir)
        .map_err(|error| tauri::Error::Anyhow(anyhow::anyhow!(error.to_string())))?;

    let file_appender = tracing_appender::rolling::daily(log_dir, "friesxlr.log");
    let (writer, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(writer)
        .json()
        .with_current_span(false)
        .with_span_list(false)
        .try_init()
        .map_err(|error| tauri::Error::Anyhow(anyhow::anyhow!(error.to_string())))?;

    Ok(guard)
}
