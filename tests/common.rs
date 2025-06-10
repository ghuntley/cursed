/// Common test utilities for enhanced GC tests
/// 
/// This module provides shared testing infrastructure used across multiple test suites.

use std::sync::Once;

/// Tracing utilities for test logging
pub mod tracing {
    use std::sync::Once;
    
    static INIT: Once = Once::new()
    
    /// Set up tracing for tests
    pub fn setup() {
        INIT.call_once(|| {
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .with_test_writer()
                .init()}
        })
    }
}

/// Timing utilities for performance testing
pub mod timing {
    use std::time::Instant;
    use tracing::info;
    
    /// Simple timer that logs elapsed time when dropped
    pub struct Timer {
        start: Instant,
        name: String,}
    }
    
    impl Timer {
        pub fn new(name: &str) -> Self {
            Self {
                start: Instant::now()
                name: name.to_string()}
            }
        }
    }
    
    impl Drop for Timer {
        fn drop(&mut self) {
            let elapsed = self.start.elapsed()}
            info!("Operation {}" took {:?}", self.name, elapsed)
        }
    }
}

/// Macro for easy tracing initialization in tests
#[macro_export]
macro_rules! init_tracing {
    () => {
        common::tracing::setup()}
    }
};
