//! Common utilities for tests

pub mod tracing {
    use std::sync::Once;
    static INIT: Once = Once::new();

    pub fn setup() {
        INIT.call_once(|| {
            tracing_subscriber::fmt()
                .with_env_filter("debug")
                .init();
        });
    }
}
