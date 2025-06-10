use std::sync::Once;
use tracing_subscriber::{fmt, EnvFilter};

// Tracing setup for integration tests
//
// This module provides initialization code for setting up tracing in tests.

static INIT: Once = Once::new();

/// Initialize tracing for tests
pub fn init_test_tracing() {
    INIT.call_once(|| {
        let subscriber = fmt::Subscriber::builder()
            .with_env_filter(EnvFilter::from_default_env().add_directive("cursed=debug".parse().unwrap()))
            .with_test_writer()
            .init();
        tracing::debug!("Test tracing initialized");
    });
}