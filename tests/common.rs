//! Common test utilities and setup for the CURSED project

pub mod tracing {
    use std::sync::Once;

    static INIT: Once = Once::new();

    /// Initialize tracing for tests
    pub fn setup() {
        INIT.call_once(|| {
            tracing_subscriber::fmt()
                .with_env_filter("debug")
                .with_test_writer()
                .init();
        });
    }
}
