//! Common test utilities for the CURSED language test suite

pub mod tracing;
pub mod timing;

// Re-export tracing for test macros
pub use ::tracing;

/// Initialize tracing for tests
#[macro_export]
macro_rules! init_tracing {
    () => {
        crate::common::tracing::setup();
    };
}
