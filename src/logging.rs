use tracing_subscriber::prelude::*;

/// Init the logging interface by [tracing](tracing)
///
/// By default, if the RUST_LOG variable is not set. The informations, warnings and errors will be outputed
pub fn init_logging() {
    let env_log = std::env::var("RUST_LOG")
        .ok()
        .unwrap_or_else(|| "basiliq=info,warn".to_string());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc3339());
    let env_layer = tracing_subscriber::filter::EnvFilter::new(env_log);
    tracing_subscriber::registry()
        .with(env_layer)
        .with(fmt_layer)
        .init();
}
