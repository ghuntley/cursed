//! Common test utilities for the CURSED test suite

pub mod tracing {
    use std::sync::Once;
    
    /// Initialize tracing for tests
    pub fn setup() {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            if let Err(_) = tracing_subscriber::fmt()
                .with_env_filter("info,cursed=debug")
                .with_test_writer()
                .try_init() {
                // If it fails, tracing is likely already initialized
            }
        });
    }
    
    /// Macro to initialize tracing in tests
    #[macro_export]
    macro_rules! init_tracing {
        () => {
            // Initialize tracing if not already done
            $crate::common::tracing::setup();
        };
    }
}

/// Benchmark timer utility
pub mod timing {
    use std::time::{Duration, Instant};
    
    pub struct Timer {
        start: Instant,
        operation: String,
    }
    
    impl Timer {
        pub fn new(operation: &str) -> Self {
            let timer = Timer {
                start: Instant::now(),
                operation: operation.to_string(),
            };
            tracing::info!(operation = operation, "Starting operation timing");
            timer
        }
    }
    
    impl Drop for Timer {
        fn drop(&mut self) {
            let elapsed = self.start.elapsed();
            tracing::info!(
                operation = self.operation.as_str(),
                duration_ms = elapsed.as_millis() as u64,
                "Operation completed"
            );
        }
    }
}