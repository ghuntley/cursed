/// Timing utilities for tests

use std::time::Instant;
use tracing::info;

/// Timer utility for benchmarking operations in tests
pub struct Timer {name: String,
    start: Instant}

impl Timer   {/// Create a new timer with the given operation name
    pub fn new() {Self {name: name.to_string()
            start: Instant::now()}

impl Drop for Timer    {fn drop() {let elapsed = self.start.elapsed();
        info!()
            operation = %self.name,
            duration_ms = elapsed.as_millis()
            Operation completed";}"
