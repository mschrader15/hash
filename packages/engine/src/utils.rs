use std::{env::VarError, path::Path, time::Duration};

use tracing_appender::non_blocking::WorkerGuard;

/// Creates the logging environment.
///
/// Returns the guards used for asynchronous writing.
#[must_use]
pub fn init_logger(txt_log: impl AsRef<Path>, json_log: impl AsRef<Path>) -> Vec<WorkerGuard> {
    use tracing_subscriber::{filter::*, fmt::time, prelude::*};
    if cfg!(debug_assertions) && std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            "hash_cloud=debug,hash_engine=debug,cli=debug,server=debug,proto=debug,nano=debug,\
             apiclient=debug",
        );
    }

    let txt_log = txt_log.as_ref();
    let json_log = json_log.as_ref();

    let mut guards = Vec::new();
    let (stderr, guard) = tracing_appender::non_blocking(std::io::stderr());
    guards.push(guard);
    let (log_txt, guard) = tracing_appender::non_blocking(
        std::fs::File::create(txt_log).expect(&format!("Could not create {txt_log:?}")),
    );
    guards.push(guard);
    let (log_json, guard) = tracing_appender::non_blocking(
        std::fs::File::create(json_log).expect(&format!("Could not create {json_log:?}")),
    );
    guards.push(guard);
    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_writer(stderr)
        .with_timer(time::Uptime::default())
        .pretty();
    let log_txt_layer = tracing_subscriber::fmt::layer()
        .with_writer(log_txt)
        .with_timer(time::Uptime::default())
        .with_ansi(false)
        .pretty();
    let log_json_layer = tracing_subscriber::fmt::layer()
        .with_writer(log_json)
        .with_timer(time::Uptime::default())
        .json();
    let error_layer = tracing_error::ErrorLayer::default();

    tracing_subscriber::registry()
        .with(log_txt_layer.with_filter(LevelFilter::TRACE))
        .with(log_json_layer.with_filter(LevelFilter::TRACE))
        .with(EnvFilter::from_default_env().and_then(stderr_layer))
        .with(error_layer)
        .init();

    guards
}

pub fn parse_env_duration(name: &str, default: u64) -> Duration {
    Duration::from_secs(
        std::env::var(name)
            .and_then(|timeout| {
                timeout.parse().map_err(|e| {
                    log::error!("Could not parse `{}` as integral: {}", name, e);
                    VarError::NotPresent
                })
            })
            .unwrap_or_else(|_| {
                log::info!("Setting `{}={}`", name, default);
                default
            }),
    )
}
