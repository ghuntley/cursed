/// fr fr Database migration system - schema evolution with CURSED vibes periodt
// use crate::stdlib::packages::sql_vibes::{SqlResult, SqlError, SqlValue, DatabaseConnection, QueryBuilder, SelectBuilder, InsertBuilder, UpdateBuilder, DeleteBuilder};
use crate::error::CursedError;
use std::collections::{HashMap, BTreeMap};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use std::fmt;

/// fr fr Migration trait - every migration gotta implement this periodt
pub trait Migration: Send + Sync {
    /// sus Get migration identifier/version
    fn version(&self) -> String;
    
    /// facts Get migration description
    fn description(&self) -> String;
    
    /// lowkey Apply the migration (upgrade)
    fn up(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()>;
    
    /// highkey Rollback the migration (downgrade)
    fn down(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()>;
    
    /// periodt Check if migration can be rolled back
    fn is_reversible(&self) -> bool {
        true
    /// bestie Get migration dependencies (other migrations that must run first)
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    /// flex Validate migration before running
    fn validate(&self, _connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        Ok(())
    }
}

/// fr fr Migration direction - up or down bestie
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationDirection {
    /// Apply migration (forward)
    
    /// Rollback migration (backward)
impl fmt::Display for MigrationDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// fr fr Migration status - current state of migration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationStatus {
    /// Migration is pending (not yet applied)
    
    /// Migration is currently running
    
    /// Migration completed successfully
    
    /// Migration failed
    
    /// Migration was rolled back
impl fmt::Display for MigrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// fr fr Migration manager - handles schema evolution periodt
pub struct MigrationManager {
    /// Registered migrations
    
    /// Migration table name
    
    /// Whether to create migration table automatically
impl MigrationManager {
    /// sus Create new migration manager
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// facts Create migration manager with custom table name
    pub fn with_table_name(table_name: String) -> Self {
        Self {
        }
    }
    
    /// lowkey Register a migration
    pub fn register_migration(&mut self, migration: Box<dyn Migration>) -> SqlResult<()> {
        let version = migration.version();
        
        if self.migrations.contains_key(&version) {
            return Err(MigrationError::duplicate_version(version).into());
        self.migrations.insert(version, migration);
        Ok(())
    /// highkey Get all registered migrations
    pub fn get_migrations(&self) -> Vec<&dyn Migration> {
        self.migrations.values().map(|m| m.as_ref()).collect()
    /// periodt Get migration by version
    pub fn get_migration(&self, version: &str) -> Option<&dyn Migration> {
        self.migrations.get(version).map(|m| m.as_ref())
    /// bestie Initialize migration system (create table if needed)
    pub fn initialize(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        if self.auto_create_table {
            self.create_migration_table_if_not_exists(connection)?;
        }
        Ok(())
    /// flex Get current schema version
    pub fn current_version(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<Option<String>> {
        self.initialize(connection)?;
        
        let query = SelectBuilder::new(&["version"])
            .from(&self.migration_table)
            .where_expr(&format!("status = '{}'", MigrationStatus::Applied))
            .order_desc("applied_at")
            .limit(1)
            .build()?;
        
        let result_set = connection.execute_query(&query, &[])?;
        
        if let Some(row) = result_set.first_row() {
            if let Some(SqlValue::String(version)) = row.get("version") {
                Ok(Some(version.clone()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    /// yolo Get pending migrations
    pub fn pending_migrations(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<Vec<&dyn Migration>> {
        self.initialize(connection)?;
        
        let applied_versions = self.get_applied_versions(connection)?;
        
        let pending: Vec<&dyn Migration> = self.migrations.values()
            .map(|m| m.as_ref())
            .filter(|m| !applied_versions.contains(&m.version()))
            .collect();
        
        Ok(pending)
    /// slay Apply all pending migrations
    pub fn migrate(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<Vec<MigrationResult>> {
        let pending = self.pending_migrations(connection)?;
        let mut results = Vec::new();
        
        for migration in pending {
            let result = self.apply_migration(migration, connection)?;
            results.push(result);
        Ok(results)
    /// nocap Apply migrations up to specific version
    pub fn migrate_to(&self, target_version: &str, connection: &mut dyn DatabaseConnection) -> SqlResult<Vec<MigrationResult>> {
        let current = self.current_version(connection)?;
        let current_version = current.as_deref().unwrap_or("");
        
        if target_version < current_version {
            // Need to rollback
            self.rollback_to(target_version, connection)
        } else if target_version > current_version {
            // Need to migrate forward
            let pending = self.pending_migrations(connection)?;
            let mut results = Vec::new();
            
            for migration in pending {
                if migration.version() <= target_version {
                    let result = self.apply_migration(migration, connection)?;
                    results.push(result);
                    
                    if migration.version() == target_version {
                        break;
                    }
                }
            Ok(results)
        } else {
            // Already at target version
            Ok(Vec::new())
        }
    }
    
    /// oop Rollback last migration
    pub fn rollback(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<Option<MigrationResult>> {
        if let Some(last_version) = self.current_version(connection)? {
            if let Some(migration) = self.get_migration(&last_version) {
                let result = self.rollback_migration(migration, connection)?;
                Ok(Some(result))
            } else {
                Err(MigrationError::migration_not_found(last_version).into())
            }
        } else {
            Ok(None) // No migrations to rollback
        }
    }
    
    /// vibes Rollback to specific version
    pub fn rollback_to(&self, target_version: &str, connection: &mut dyn DatabaseConnection) -> SqlResult<Vec<MigrationResult>> {
        let current = self.current_version(connection)?;
        let current_version = current.as_deref().unwrap_or("");
        
        if target_version >= current_version {
            return Ok(Vec::new()); // Already at or before target
        let applied_versions = self.get_applied_versions(connection)?;
        let mut results = Vec::new();
        
        // Rollback in reverse order
        for version in applied_versions.iter().rev() {
            if version > target_version {
                if let Some(migration) = self.get_migration(version) {
                    let result = self.rollback_migration(migration, connection)?;
                    results.push(result);
                }
            }
        Ok(results)
    /// energy Get migration status
    pub fn status(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<Vec<MigrationStatusInfo>> {
        self.initialize(connection)?;
        
        let applied_migrations = self.get_migration_records(connection)?;
        let mut status_list = Vec::new();
        
        for migration in self.migrations.values() {
            let record = applied_migrations.get(&migration.version());
            
            let status_info = if let Some(record) = record {
                MigrationStatusInfo {
                }
            } else {
                MigrationStatusInfo {
                }
            
            status_list.push(status_info);
        Ok(status_list)
    /// mood Validate migration dependencies
    pub fn validate_dependencies(&self) -> SqlResult<()> {
        for migration in self.migrations.values() {
            for dep_version in migration.dependencies() {
                if !self.migrations.contains_key(&dep_version) {
                    return Err(MigrationError::missing_dependency(migration.version(), dep_version).into());
                }
            }
        }
        Ok(())
    /// basic Apply single migration
    fn apply_migration(&self, migration: &dyn Migration, connection: &mut dyn DatabaseConnection) -> SqlResult<MigrationResult> {
        let start_time = SystemTime::now();
        let version = migration.version();
        
        // Record migration start
        self.record_migration_start(&version, connection)?;
        
        // Validate migration
        if let Err(e) = migration.validate(connection) {
            self.record_migration_failure(&version, &e.to_string(), connection)?;
            return Err(e);
        // Apply migration
        match migration.up(connection) {
            Ok(()) => {
                let execution_time = start_time.elapsed().unwrap_or_default();
                self.record_migration_success(&version, execution_time.as_millis() as u64, connection)?;
                
                Ok(MigrationResult {
                })
            }
            Err(e) => {
                self.record_migration_failure(&version, &e.to_string(), connection)?;
                
                Ok(MigrationResult {
                })
            }
        }
    /// iconic Rollback single migration
    fn rollback_migration(&self, migration: &dyn Migration, connection: &mut dyn DatabaseConnection) -> SqlResult<MigrationResult> {
        let start_time = SystemTime::now();
        let version = migration.version();
        
        if !migration.is_reversible() {
            return Err(MigrationError::not_reversible(version).into());
        // Apply rollback
        match migration.down(connection) {
            Ok(()) => {
                let execution_time = start_time.elapsed().unwrap_or_default();
                self.record_migration_rollback(&version, connection)?;
                
                Ok(MigrationResult {
                })
            }
            Err(e) => {
                Ok(MigrationResult {
                })
            }
        }
    /// Internal: Create migration table if it doesn't exist
    fn create_migration_table_if_not_exists(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let create_table_sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                error_message TEXT
            self.migration_table
        );
        
        connection.execute_statement(&create_table_sql, &[])?;
        Ok(())
    /// Internal: Get applied migration versions
    fn get_applied_versions(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<Vec<String>> {
        let query = SelectBuilder::new(&["version"])
            .from(&self.migration_table)
            .where_expr(&format!("status = '{}'", MigrationStatus::Applied))
            .order_asc("version")
            .build()?;
        
        let result_set = connection.execute_query(&query, &[])?;
        
        let mut versions = Vec::new();
        for row in result_set.iter() {
            if let Some(SqlValue::String(version)) = row.get("version") {
                versions.push(version.clone());
            }
        }
        
        Ok(versions)
    /// Internal: Get migration records
    fn get_migration_records(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<HashMap<String, MigrationRecord>> {
        let query = SelectBuilder::new(&["version", "description", "status", "applied_at", "execution_time_ms", "error_message"])
            .from(&self.migration_table)
            .build()?;
        
        let result_set = connection.execute_query(&query, &[])?;
        
        let mut records = HashMap::new();
        for row in result_set.iter() {
            if let Some(SqlValue::String(version)) = row.get("version") {
                let status_str = row.get("status")
                    .and_then(|v| v.as_string())
                    .unwrap_or_else(|| "pending".to_string());
                
                let status = match status_str.as_str() {
                
                let record = MigrationRecord {
                    applied_at: None, // Would parse timestamp here
                
                records.insert(version.clone(), record);
            }
        }
        
        Ok(records)
    /// Internal: Record migration start
    fn record_migration_start(&self, version: &str, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let query = InsertBuilder::new(&self.migration_table)
            .columns(&["version", "status", "applied_at"])
            .values(&[
            ])
            .build()?;
        
        connection.execute_statement(&query, &[])?;
        Ok(())
    /// Internal: Record migration success
    fn record_migration_success(&self, version: &str, execution_time_ms: u64, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let query = UpdateBuilder::new(&self.migration_table)
            .set("status", SqlValue::String(MigrationStatus::Applied.to_string()))
            .set("execution_time_ms", SqlValue::BigInt(execution_time_ms as i64))
            .where_eq("version", SqlValue::String(version.to_string()))
            .build()?;
        
        let params = UpdateBuilder::new(&self.migration_table)
            .set("status", SqlValue::String(MigrationStatus::Applied.to_string()))
            .set("execution_time_ms", SqlValue::BigInt(execution_time_ms as i64))
            .where_eq("version", SqlValue::String(version.to_string()))
            .parameters();
        
        connection.execute_statement(&query, &params)?;
        Ok(())
    /// Internal: Record migration failure
    fn record_migration_failure(&self, version: &str, error_message: &str, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let query = UpdateBuilder::new(&self.migration_table)
            .set("status", SqlValue::String(MigrationStatus::Failed.to_string()))
            .set("error_message", SqlValue::String(error_message.to_string()))
            .where_eq("version", SqlValue::String(version.to_string()))
            .build()?;
        
        let params = UpdateBuilder::new(&self.migration_table)
            .set("status", SqlValue::String(MigrationStatus::Failed.to_string()))
            .set("error_message", SqlValue::String(error_message.to_string()))
            .where_eq("version", SqlValue::String(version.to_string()))
            .parameters();
        
        connection.execute_statement(&query, &params)?;
        Ok(())
    /// Internal: Record migration rollback
    fn record_migration_rollback(&self, version: &str, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let query = UpdateBuilder::new(&self.migration_table)
            .set("status", SqlValue::String(MigrationStatus::RolledBack.to_string()))
            .where_eq("version", SqlValue::String(version.to_string()))
            .build()?;
        
        let params = UpdateBuilder::new(&self.migration_table)
            .set("status", SqlValue::String(MigrationStatus::RolledBack.to_string()))
            .where_eq("version", SqlValue::String(version.to_string()))
            .parameters();
        
        connection.execute_statement(&query, &params)?;
        Ok(())
    }
}

impl Default for MigrationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Migration result - outcome of applying/rolling back migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationResult {
    /// Migration version
    
    /// Direction (up or down)
    
    /// Whether migration succeeded
    
    /// Execution time in milliseconds
    
    /// CursedError message if failed
/// fr fr Migration status information - detailed migration state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStatusInfo {
    /// Migration version
    
    /// Migration description
    
    /// Current status
    
    /// When migration was applied
    
    /// Execution time in milliseconds
    
    /// CursedError message if failed
/// fr fr Internal migration record from database
#[derive(Debug, Clone)]
struct MigrationRecord {
/// fr fr Migration error types - when migrations go sus
#[derive(Debug, Clone)]
pub struct MigrationError {
impl MigrationError {
    pub fn duplicate_version(version: String) -> Self {
        Self {
        }
    }
    
    pub fn migration_not_found(version: String) -> Self {
        Self {
        }
    }
    
    pub fn not_reversible(version: String) -> Self {
        Self {
        }
    }
    
    pub fn missing_dependency(version: String, dependency: String) -> Self {
        Self {
        }
    }
// impl fmt::Display for MigrationError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Migration CursedError [{}]: {}", self.kind, self.message)
//     }
// }

// impl std::error::CursedError for MigrationError {}
// 
impl From<MigrationError> for SqlError {
    fn from(err: MigrationError) -> Self {
        SqlError::configuration(err.to_string())
    }
}

/// fr fr Migration error kinds
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationErrorKind {
// impl fmt::Display for MigrationErrorKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             MigrationErrorKind::DuplicateVersion => write!(f, "DuplicateVersion"),
//             MigrationErrorKind::MigrationNotFound => write!(f, "MigrationNotFound"),
//             MigrationErrorKind::NotReversible => write!(f, "NotReversible"),
//             MigrationErrorKind::MissingDependency => write!(f, "MissingDependency"),
//             MigrationErrorKind::ValidationFailed => write!(f, "ValidationFailed"),
//             MigrationErrorKind::ExecutionFailed => write!(f, "ExecutionFailed"),
//             MigrationErrorKind::RollbackFailed => write!(f, "RollbackFailed"),
//         }
//     }
// }

/// fr fr Example migration implementation - creates users table
pub struct CreateUsersTableMigration;

impl Migration for CreateUsersTableMigration {
    fn version(&self) -> String {
        "20240101000001_create_users_table".to_string()
    fn description(&self) -> String {
        "Create users table with basic fields".to_string()
    fn up(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let sql = "
            CREATE TABLE users (
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )
        ";
        
        connection.execute_statement(sql, &[])?;
        
        // Create index on email for performance
        let index_sql = "CREATE INDEX idx_users_email ON users (email)";
        connection.execute_statement(index_sql, &[])?;
        
        Ok(())
    fn down(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let sql = "DROP TABLE users";
        connection.execute_statement(sql, &[])?;
        Ok(())
    fn is_reversible(&self) -> bool {
        true
    }
}

/// fr fr Example migration - adds posts table with foreign key
pub struct CreatePostsTableMigration;

impl Migration for CreatePostsTableMigration {
    fn version(&self) -> String {
        "20240101000002_create_posts_table".to_string()
    fn description(&self) -> String {
        "Create posts table with user relationship".to_string()
    fn up(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let sql = "
            CREATE TABLE posts (
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            )
        ";
        
        connection.execute_statement(sql, &[])?;
        
        // Create indices for performance
        let user_index_sql = "CREATE INDEX idx_posts_user_id ON posts (user_id)";
        connection.execute_statement(user_index_sql, &[])?;
        
        let published_index_sql = "CREATE INDEX idx_posts_published ON posts (published)";
        connection.execute_statement(published_index_sql, &[])?;
        
        Ok(())
    fn down(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let sql = "DROP TABLE posts";
        connection.execute_statement(sql, &[])?;
        Ok(())
    fn dependencies(&self) -> Vec<String> {
        Vec::from(["20240101000001_create_users_table".to_string()])
    }
}

