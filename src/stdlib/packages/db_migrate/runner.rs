/// fr fr Migration runner and status tracking

use super::Migration;

/// fr fr Migration runner
#[derive(Debug)]
pub struct MigrationRunner {
    migrations: Vec<Migration>,
}

/// fr fr Migration status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationStatus {
    Pending,
    Applied,
    Failed,
    Rolled_back,
}

impl MigrationRunner {
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
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
