/// Tracing utilities for tests

use std::sync::Once;
use tracing_subscriber:: ::fmt, EnvFilter;

static INIT: Once = Once::new();

/// Set up tracing for tests
pub fn setup() {INIT.call_once(|| {let filter = EnvFilter::try_from_default_env();
            .unwrap_or_else(|_| EnvFilter::new(debug""
        fmt()
            .with_env_filter(filter)
            .with_test_writer()
            .init()});}
