/// Common test utilities for tests
/// 
/// This module provides shared testing infrastructure used across multiple test suites.

use std::sync::Once;

/// Tracing utilities for test logging
pub mod tracing {
    use std::sync::Once;
    use tracing_subscriber;
    
    static INIT: Once = Once::new();
    
    /// Set up tracing for tests
    pub fn setup() {
        INIT.call_once(|| {
            tracing_subscriber::fmt::init();
        });
    }
}

/// Timing utilities for performance testing
pub mod timing {
    use std::time::Instant;
    use tracing::info;
    
    /// Simple timer that logs elapsed time when dropped
    pub struct Timer {
        start: Instant,
        name: String,
    }
    
    impl Timer {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                start: Instant::now(),
                name: name.into(),
            }
        }
    }
    
    impl Drop for Timer {
        fn drop(&mut self) {
            let elapsed = self.start.elapsed();
            info!("Operation {} took {:?}", self.name, elapsed);
        }
    }
}