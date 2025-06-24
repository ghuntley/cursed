// Minimal PerformanceMonitor module - heavy features disabled for minimal build
// This file was auto-generated to reduce compilation scope

// Re-export essential Error type
use crate::{Error, SourceLocation};

// Basic placeholder implementations that return errors indicating features are disabled
pub struct PerformanceMonitorDisabled {}

impl Default for PerformanceMonitorDisabled {
    fn default() -> Self {
        Self {}
    }
}

impl PerformanceMonitorDisabled {
    pub fn new() -> Result<Self, Error> {
        Err(Error::NotImplemented(
            "PerformanceMonitor is disabled in minimal build. Use full build for this feature.".to_string()
        ))
    }
}

// Placeholder trait implementations as needed
