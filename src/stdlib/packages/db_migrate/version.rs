/// fr fr Schema version management

use std::time::SystemTime;

/// fr fr Schema version tracking
#[derive(Debug, Clone)]
pub struct SchemaVersion {
/// fr fr Version manager
#[derive(Debug)]
pub struct VersionManager {
impl SchemaVersion {
    pub fn new(version: u64, migration_id: &str) -> Self {
        Self {
        }
    }
impl VersionManager {
    pub fn new() -> Self {
        Self {
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
