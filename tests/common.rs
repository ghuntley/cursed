/// Common utilities for tests
pub mod tracing {
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    
    pub fn setup() {
        INIT.call_once(|| {
            let _ = tracing_subscriber::fmt()
                .with_env_filter("debug")
                .try_init();
        });
    }
}

pub mod timing {
    use std::time::Instant;
    
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
            tracing::info!("{} took {:?}", self.name, elapsed);
        }
    }
}

// Tracing initialization macro moved to tests/common/mod.rs
