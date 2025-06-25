/// Tracing setup for panic and recovery tests
/// 
/// Provides standardized tracing configuration for test environments

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize tracing for tests
pub fn init_test_tracing() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "debug".into())
            )
            .with_test_writer()
            .init();
    });
}
