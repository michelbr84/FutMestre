//! Tracing initialization.

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize tracing with the given filter.
pub fn init_tracing(filter: EnvFilter) {
    tracing_subscriber::registry()
        .with(fmt::layer().with_target(true).with_thread_ids(false))
        .with(filter)
        .init();
}

/// Initialize tracing with default settings (info level).
pub fn init_default_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    init_tracing(filter);
}
