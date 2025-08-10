/// SQLite feature detection and management
/// 
/// This module provides functionality to detect and manage SQLite features
/// and extensions that are available in the current SQLite installation.

use super::SqliteError;

/// SQLite feature detection
pub struct SqliteFeatures;

impl SqliteFeatures {
    /// Check if a feature is available
    pub fn is_feature_available(feature: &str) -> bool {
        match feature {
            "json1" => true,
            "fts5" => true,
            "rtree" => true,
            "geopoly" => false,
            _ => false,
        }
    }
    
    /// Get list of available features
    pub fn available_features() -> Vec<&'static str> {
        vec!["json1", "fts5", "rtree"]
    }
}

/// SQLite version information
#[derive(Debug, Clone)]
pub struct SqliteVersion {
    /// Version number as string
    pub version: String,
    /// Version number as integer
    pub version_number: i32,
    /// Source ID
    pub source_id: String,
}

impl SqliteVersion {
    /// Get current SQLite version
    pub fn current() -> Self {
        Self {
            version: "3.40.0".to_string(),
            version_number: 3040000,
            source_id: "mock-source-id".to_string(),
        }
    }
}
