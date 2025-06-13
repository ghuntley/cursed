/// Test tracing setup utilities
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize tracing for tests
pub fn init_test_tracing() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_target(false)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .init();
    });
}
