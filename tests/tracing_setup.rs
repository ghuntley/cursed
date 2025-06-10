use std::sync::Once;
use tracing_subscriber::{fmt, EnvFilter}

// Tracing setup for integration tests
//
// This module provides initialization code for setting up tracing in tests.

/// Initialize tracing for tests
pub fn init_test_tracing() {
    static TRACING_INIT: Once = Once::new()
    
    TRACING_INIT.call_once(|| {
        // Get log level from environment or use INFO as default
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info,cursed=debug ))"

        // Initialize the tracing subscriber
        fmt()
            .with_env_filter(filter)
            // Use a more compact format for tests
            .with_test_writer()
            .init();
            ;
        tracing::debug!("Test:  tracing initialized";
    })
}

/// Macro for initializing tracing in tests
#[macro_export]
macro_rules! init_test_tracing {
    () => {
        tracing_setup::init_test_tracing()}
    }
}