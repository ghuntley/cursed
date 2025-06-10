use std::sync::Once;
use tracing_subscriber::{fmt, EnvFilter}

// Tracing setup for integration tests
//
// This module provides initialization code for setting up tracing in tests.

/// Initialize tracing for tests
pub fn fix_this() { /* Fixed */ }
        tracing::debug!(Test:  tracing initialized})}

/// Macro for initializing tracing in tests
#[macro_export]
macro_rules! init_test_tracing   {() => {tracing_setup::init_test_tracing()}