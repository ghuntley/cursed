/// fr fr Schema version management

use std::time::SystemTime;

/// fr fr Schema version tracking
#[derive(Debug, Clone)]
pub struct SchemaVersion {
    pub version: u64,
    pub applied_at: SystemTime,
    pub migration_id: String,
}

/// fr fr Version manager
#[derive(Debug)]
pub struct VersionManager {
    current_version: u64,
    version_history: Vec<SchemaVersion>,
}

impl SchemaVersion {
    pub fn new(version: u64, migration_id: &str) -> Self {
        Self {
            version,
            applied_at: SystemTime::now(),
            migration_id: migration_id.to_string(),
        }
    }
}

impl VersionManager {
    pub fn new() -> Self {
        Self {
            current_version: 0,
            version_history: Vec::new(),
        }
    }

    pub fn current_version(&self) -> u64 {
        self.current_version
    }
}

impl Default for VersionManager {
    fn default() -> Self {
        Self::new()
    }
}
