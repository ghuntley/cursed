// Minimal ProcessIpcIntegration module - heavy features disabled for minimal build
// This file was auto-generated to reduce compilation scope

// Re-export essential Error type
use crate::{Error, SourceLocation};

// Basic placeholder implementations that return errors indicating features are disabled
pub struct ProcessIpcIntegrationDisabled {}

impl Default for ProcessIpcIntegrationDisabled {
    fn default() -> Self {
        Self {}
    }
}

impl ProcessIpcIntegrationDisabled {
    pub fn new() -> Result<Self, Error> {
        Err(Error::NotImplemented(
            "ProcessIpcIntegration is disabled in minimal build. Use full build for this feature.".to_string()
        ))
    }
}

// Placeholder trait implementations as needed
