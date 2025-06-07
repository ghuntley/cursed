//! Common test utilities for the CURSED language test suite

pub mod tracing;
pub mod timing;

/// Initialize tracing for tests
#[macro_export]
macro_rules! init_tracing {
    () => {
        crate::common::tracing::setup();
    };
}
