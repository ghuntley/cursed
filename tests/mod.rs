/// Common test utilities
pub mod tracing;

/// Initialize tracing for tests - convenience function
pub fn init_tracing() {
    tracing::setup();
}

/// Macro for easy tracing initialization - using function call instead
/// Note: Use common::init_tracing() directly to avoid macro conflicts

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
