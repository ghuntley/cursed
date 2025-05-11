//! # Timing Utilities for Tests
//!
//! This module provides utilities for timing operations in tests.
//! It helps with performance profiling and identifying slow tests.

use std::time::{Duration, Instant};
use tracing::{debug, info};

/// A timer utility that logs the elapsed time when dropped
///
/// # Example
///
/// ```
/// use crate::tests::common::timing::Timer;
///
/// #[test]
/// fn my_test() {
///     let _timer = Timer::new("my_operation");
///     // Code to be timed here
///     // When _timer goes out of scope, it logs the elapsed time
/// }
/// ```
pub struct Timer {
    /// The name of the operation being timed
    name: String,
    /// The start time of the timer
    start: Instant,
    /// The threshold for logging at INFO level
    threshold: Option<Duration>,
}

impl Timer {
    /// Create a new timer with the given operation name
    pub fn new(name: &str) -> Self {
        debug!("Starting timer for {}", name);
        Self {
            name: name.to_string(),
            start: Instant::now(),
            threshold: None,
        }
    }
    
    /// Create a new timer with the given operation name and threshold
    /// The threshold is the minimum duration required for the timer to log at INFO level
    pub fn with_threshold(name: &str, threshold: Duration) -> Self {
        let mut timer = Self::new(name);
        timer.threshold = Some(threshold);
        timer
    }
    
    /// Get the elapsed time since the timer was created
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    
    /// Get the elapsed time in milliseconds
    pub fn elapsed_ms(&self) -> f64 {
        let elapsed = self.elapsed();
        elapsed.as_secs_f64() * 1000.0
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.elapsed();
        let elapsed_ms = elapsed.as_secs_f64() * 1000.0;
        
        match self.threshold {
            Some(threshold) if elapsed < threshold => {
                // Below threshold, log at debug level
                debug!(
                    operation = self.name.as_str(),
                    elapsed_ms = elapsed_ms,
                    "Operation completed"
                );
            },
            _ => {
                // Above threshold or no threshold, log at info level
                info!(
                    operation = self.name.as_str(),
                    elapsed_ms = elapsed_ms,
                    "Operation timing"
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common::tracing;
    use std::thread;
    
    #[test]
    fn test_timer_basic() {
        tracing::setup();
        
        let _timer = Timer::new("test_operation");
        
        // Simulate some work
        thread::sleep(Duration::from_millis(10));
        
        // Timer will log when it goes out of scope
    }
    
    #[test]
    fn test_timer_with_threshold() {
        tracing::setup();
        
        // Should log at debug level (below threshold)
        let _timer1 = Timer::with_threshold("fast_operation", Duration::from_millis(50));
        thread::sleep(Duration::from_millis(5));
        
        // Should log at info level (above threshold)
        let _timer2 = Timer::with_threshold("slow_operation", Duration::from_millis(5));
        thread::sleep(Duration::from_millis(10));
    }
    
    #[test]
    fn test_elapsed_methods() {
        let timer = Timer::new("elapsed_test");
        thread::sleep(Duration::from_millis(10));
        
        assert!(timer.elapsed().as_millis() >= 10);
        assert!(timer.elapsed_ms() >= 10.0);
    }
}