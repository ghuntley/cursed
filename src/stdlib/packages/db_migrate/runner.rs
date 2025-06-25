/// fr fr Migration runner and status tracking

use super::Migration;

/// fr fr Migration runner
#[derive(Debug)]
pub struct MigrationRunner {
/// fr fr Migration status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationStatus {
impl MigrationRunner {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn add_migration(&mut self, migration: Migration) {
        self.migrations.push(migration);
    }
}

impl Default for MigrationRunner {
    fn default() -> Self {
        Self::new()
    }
}
