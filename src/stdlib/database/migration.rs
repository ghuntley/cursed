//! Database migration management

use crate::error::CursedError;
use std::collections::HashMap;
use crate::stdlib::packages::IOError;

/// Result type for migration operations
pub type MigrationResult<T> = Result<T, CursedError>;

/// Database migration
pub struct Migration {
    pub id: String,
    pub description: String,
    pub up_sql: String,
    pub down_sql: String,
    pub checksum: String,
}

/// Migration status
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationStatus {
    Pending,
    Applied,
    Failed,
    Rolled,
}

/// Database migrator
pub struct Migrator {
    migrations: Vec<Migration>,
    applied_migrations: HashMap<String, MigrationStatus>,
}

impl Migration {
    /// Create a new migration
    pub fn new(id: &str, description: &str, up_sql: &str, down_sql: &str) -> Self {
        let checksum = format!("{:x}", md5::compute(up_sql.as_bytes()));
        Self {
            id: id.to_string(),
            description: description.to_string(),
            up_sql: up_sql.to_string(),
            down_sql: down_sql.to_string(),
            checksum,
        }
    }
    
    /// Execute the migration
    pub fn apply(&self) -> MigrationResult<()> {
        println!("⬆️ Applying migration {}: {}", self.id, self.description);
        // In a real implementation, this would execute the SQL
        Ok(())
    }
    
    /// Rollback the migration
    pub fn rollback(&self) -> MigrationResult<()> {
        println!("⬇️ Rolling back migration {}: {}", self.id, self.description);
        // In a real implementation, this would execute the down SQL
        Ok(())
    }
    
    /// Validate the migration
    pub fn validate(&self) -> MigrationResult<()> {
        if self.id.is_empty() {
            return Err(CursedError::runtime_error(&"Migration ID cannot be empty".to_string()));
        }
        if self.up_sql.is_empty() {
            return Err(CursedError::runtime_error(&"Migration up SQL cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl Migrator {
    /// Create a new migrator
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
            applied_migrations: HashMap::new(),
        }
    }
    
    /// Add a migration
    pub fn add_migration(&mut self, migration: Migration) -> MigrationResult<()> {
        migration.validate()?;
        self.migrations.push(migration);
        Ok(())
    }
    
    /// Apply all pending migrations
    pub fn migrate(&mut self) -> MigrationResult<()> {
        let pending = self.get_pending_migrations();
        let pending_ids: Vec<String> = pending.iter().map(|m| m.id.clone()).collect();
        println!("🚀 Applying {} pending migrations", pending.len());
        
        let mut migration_results = Vec::new();
        for migration in pending {
            migration.apply()?;
            migration_results.push((migration.id.clone(), MigrationStatus::Applied));
        }
        
        for (id, status) in migration_results {
            self.applied_migrations.insert(id, status);
        }
        
        Ok(())
    }
    
    /// Rollback the last migration
    pub fn rollback(&mut self) -> MigrationResult<()> {
        if let Some(last_applied) = self.get_last_applied_migration() {
            let migration_id = last_applied.id.clone();
            last_applied.rollback()?;
            self.applied_migrations.insert(migration_id.clone(), MigrationStatus::Rolled);
            println!("🔄 Rolled back migration: {}", migration_id);
        } else {
            println!("ℹ️ No migrations to rollback");
        }
        Ok(())
    }
    
    /// Get pending migrations
    pub fn get_pending_migrations(&self) -> Vec<&Migration> {
        self.migrations.iter()
            .filter(|m| !self.applied_migrations.contains_key(&m.id) || 
                       self.applied_migrations.get(&m.id) == Some(&MigrationStatus::Pending))
            .collect()
    }
    
    /// Get applied migrations
    pub fn get_applied_migrations(&self) -> Vec<&Migration> {
        self.migrations.iter()
            .filter(|m| self.applied_migrations.get(&m.id) == Some(&MigrationStatus::Applied))
            .collect()
    }
    
    /// Get the last applied migration
    fn get_last_applied_migration(&self) -> Option<&Migration> {
        self.migrations.iter()
            .filter(|m| self.applied_migrations.get(&m.id) == Some(&MigrationStatus::Applied))
            .last()
    }
    
    /// Get migration status
    pub fn get_status(&self, migration_id: &str) -> MigrationStatus {
        self.applied_migrations.get(migration_id)
            .cloned()
            .unwrap_or(MigrationStatus::Pending)
    }
    
    /// List all migrations with their status
    pub fn list_migrations(&self) -> Vec<(String, String, MigrationStatus)> {
        self.migrations.iter()
            .map(|m| (m.id.clone(), m.description.clone(), self.get_status(&m.id)))
            .collect()
    }
}

impl Default for Migrator {
    fn default() -> Self {
        Self::new()
    }
}

// Mock md5 module for compilation
mod md5 {
    pub fn compute(data: &[u8]) -> u128 {
        // Simplified hash - in real implementation would use actual MD5
        let mut hash = 0u128;
        for byte in data {
            hash = hash.wrapping_mul(31).wrapping_add(*byte as u128);
        }
        hash
    }
}
