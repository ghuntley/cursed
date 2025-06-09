//! Common test utilities and setup for integration tests

pub mod tracing {
    use std::sync::Once;

    static INIT: Once = Once::new();

    /// Initialize tracing for tests (call once per test)
    pub fn setup() {
        INIT.call_once(|| {
            tracing_subscriber::fmt()
                .with_env_filter("debug")
                .with_test_writer()
                .init();
        });
    }
}

/// Convenience macro for initializing tracing in tests
#[macro_export]
macro_rules! init_tracing {
    () => {
        $crate::common::tracing::setup();
    };
}

pub mod timing {
    use std::time::Instant;
    use tracing::info;

    /// Timer utility for benchmarking operations in tests
    pub struct Timer {
        name: String,
        start: Instant,
    }

    impl Timer {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                start: Instant::now(),
            }
        }
    }

    impl Drop for Timer {
        fn drop(&mut self) {
            let elapsed = self.start.elapsed();
            info!("Operation '{}' completed in {:?}", self.name, elapsed);
        }
    }
}
