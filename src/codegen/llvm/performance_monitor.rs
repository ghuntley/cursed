// Minimal PerformanceMonitor module - heavy features disabled for minimal build
// This file was auto-generated to reduce compilation scope

// Re-export essential Error type
use crate::{Error, SourceLocation};

// Basic placeholder implementations that return errors indicating features are disabled
pub struct PerformanceMonitor {}
pub struct MonitoringConfig {}
pub struct CodeMetrics {}
pub struct BaselineMetrics {}
pub struct PerformanceReport {}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self {}
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {}
    }
}

impl Default for CodeMetrics {
    fn default() -> Self {
        Self {}
    }
}

impl Default for BaselineMetrics {
    fn default() -> Self {
        Self {}
    }
}

impl Default for PerformanceReport {
    fn default() -> Self {
        Self {}
    }
}

impl PerformanceMonitor {
    pub fn new() -> Result<Self, Error> {
        Err(Error::NotImplemented(
            "PerformanceMonitor is disabled in minimal build. Use full build for this feature.".to_string()
        ))
    }
}

// Placeholder trait implementations as needed
