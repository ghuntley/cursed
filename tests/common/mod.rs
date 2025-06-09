//! Common test utilities for goroutine scheduler tests

use tracing_subscriber::{fmt, EnvFilter};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize tracing for tests
pub fn init_tracing() {
    INIT.call_once(|| {
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("debug"));

        fmt()
            .with_env_filter(filter)
            .with_test_writer()
            .init();
    });
}

/// Macro to initialize tracing in tests
#[macro_export]
macro_rules! init_tracing {
    () => {
        common::init_tracing();
    };
}
