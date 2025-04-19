use std::time::{Duration, Instant};
use tracing::info;

pub struct Timer {
    name: String,
    start: Instant,
}

impl Timer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
        }
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    
    pub fn elapsed_ms(&self) -> f64 {
        let elapsed = self.elapsed();
        elapsed.as_secs_f64() * 1000.0
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.elapsed_ms();
        info!("Timer '{}' completed in {:.2}ms", self.name, elapsed);
    }
}