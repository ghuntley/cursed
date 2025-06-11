/// Common test utilities

pub mod tracing {
    use std::sync::Once;
    use tracing_subscriber::{fmt, EnvFilter};

    static INIT: Once = Once::new();

    /// Set up tracing for tests
    pub fn setup() {
        INIT.call_once(|| {
            let filter = EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("debug"));
            
            fmt()
                .with_env_filter(filter)
                .with_test_writer()
                .init();
        });
    }
}

/// Initialize tracing for tests - convenience function
pub fn init_tracing() {
    tracing::setup();
}

/// Macro for easy tracing initialization
#[macro_export]
macro_rules! init_tracing {
    () => {
        $crate::common::init_tracing();
    };
}

pub mod timing {
    use std::time::Instant;
    
    /// Timer utility for measuring test performance
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
            println!("Timer {} elapsed: {:?}", self.name, elapsed);
        }
    }
}
