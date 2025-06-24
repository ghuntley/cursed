// Debug modules for CURSED
pub mod enhanced_debug;
pub mod source_location;

// Re-export key types
pub use crate::error::SourceLocation;
pub use enhanced_debug::EnhancedDebug;
pub use source_location::SourceLocationInfo;

#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub enabled: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}
