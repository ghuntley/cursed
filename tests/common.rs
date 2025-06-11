/// Common utilities for CURSED tests
/// 
/// This module provides shared functionality used across multiple test files.

pub mod tracing {
    use std::sync::Once;
    static INIT: Once = Once::new();

    pub fn setup() {
        INIT.call_once(|| {
            tracing_subscriber::fmt()
                .with_env_filter("debug")
                .try_init()
                .ok();
        });
    }
}

pub mod timing {
    use std::time::Instant;

    pub struct Timer {
        start: Instant,
        name: String,
    }

    impl Timer {
        pub fn new(name: &str) -> Self {
            Timer {
                start: Instant::now(),
                name: name.to_string(),
            }
        }
    }

    impl Drop for Timer {
        fn drop(&mut self) {
            let elapsed = self.start.elapsed();
            tracing::info!("Timer '{}' elapsed: {:?}", self.name, elapsed);
        }
    }
}

/// Initialize tracing - convenience function for direct use
pub fn init_tracing() {
    tracing::setup();
}

/// Initialize tracing for tests
#[macro_export]
macro_rules! init_tracing {
    () => {
        $crate::common::tracing::setup();
    };
}
