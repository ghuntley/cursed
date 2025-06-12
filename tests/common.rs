/// Common utilities for tests

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

/// Initialize tracing macro
#[macro_export]
macro_rules! init_tracing {
    () => {
        $crate::common::tracing::setup();
    };
}
