//! Common test utilities for all tests

pub mod tracing;
pub mod timing;
pub mod test_utils;

// Re-export commonly used items
pub use tracing::setup as init_tracing;
pub use timing::Timer;

/// Macro to initialize tracing in tests
#[macro_export]
macro_rules! init_tracing {
    () => {
        common::init_tracing();
    };
}
