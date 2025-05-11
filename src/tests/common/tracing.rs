//! # Tracing Setup for Tests
//!
//! This module provides utilities for setting up tracing in tests.
//! It helps with structured logging during test execution for better
//! debugging and diagnosis of test failures.

use tracing::subscriber::set_global_default;
use tracing_subscriber::{fmt, EnvFilter, Registry};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use std::sync::Once;

static TRACING_INIT: Once = Once::new();

/// Initialize tracing for tests. This function sets up a global
/// tracing subscriber that logs to stderr. It is safe to call this
/// function multiple times; it will only initialize tracing once.
///
/// # Example
///
/// ```
/// use crate::tests::common::tracing;
///
/// #[test]
/// fn my_test() {
///     tracing::setup();
///     // Test code here
/// }
/// ```
pub fn setup() {
    TRACING_INIT.call_once(|| {
        let fmt_layer = fmt::layer()
            .with_span_events(FmtSpan::CLOSE)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true);

        let filter_layer = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info,cursed=debug,tracing=info"));

        let subscriber = Registry::default()
            .with(filter_layer)
            .with(fmt_layer);

        set_global_default(subscriber).expect("Failed to set global default tracer");
        tracing::info!("Tracing initialized for tests");
    });
}

/// A macro to initialize tracing in tests
#[macro_export]
macro_rules! init_tracing {
    () => {
        crate::tests::common::tracing::setup();
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{debug, info, warn, error};
    
    #[test]
    fn test_tracing_setup() {
        setup();
        
        // Log at different levels
        debug!("This is a debug message");
        info!("This is an info message");
        warn!("This is a warning message");
        error!("This is an error message");
        
        // Log with structured fields
        info!(test_name = "test_tracing_setup", result = "success", "Test completed");
    }
    
    #[test]
    fn test_macro() {
        init_tracing!();
        
        info!("Tracing initialized via macro");
    }
}