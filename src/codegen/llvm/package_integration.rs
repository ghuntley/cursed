// Minimal PackageIntegration module - heavy features disabled for minimal build
// This file was auto-generated to reduce compilation scope

// Re-export essential Error type
use crate::{Error, SourceLocation};

// Basic placeholder implementations that return errors indicating features are disabled
pub struct PackageIntegrationDisabled {}

impl Default for PackageIntegrationDisabled {
    fn default() -> Self {
        Self {}
    }
}

impl PackageIntegrationDisabled {
    pub fn new() -> Result<Self, Error> {
        Err(Error::NotImplemented(
            "PackageIntegration is disabled in minimal build. Use full build for this feature.".to_string()
        ))
    }
}

// Placeholder trait implementations as needed
