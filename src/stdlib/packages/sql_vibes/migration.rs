/// fr fr Database migration system - schema evolution with CURSED vibes periodt
use crate::stdlib::packages::sql_vibes::{SqlResult, SqlError, SqlValue, DatabaseConnection, QueryBuilder, SelectBuilder, InsertBuilder, UpdateBuilder, DeleteBuilder};
use crate::error::Error;
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
    }
    
    /// bestie Get migration dependencies (other migrations that must run first)
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }
    
    /// flex Validate migration before running
    fn validate(&self, _connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        Ok(())
    }
}

/// fr fr Migration direction - up or down bestie
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationDirection {
    /// Apply migration (forward)
    Up,
    
    /// Rollback migration (backward)
    Down,
}

impl fmt::Display for MigrationDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MigrationDirection::Up => write!(f, "up"),
            MigrationDirection::Down => write!(f, "down"),
        }
    }
}

/// fr fr Migration status - current state of migration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationStatus {
    /// Migration is pending (not yet applied)
    Pending,
    
    /// Migration is currently running
    Running,
    
    /// Migration completed successfully
    Applied,
    
    /// Migration failed
    Failed,
    
    /// Migration was rolled back
    RolledBack,
}

impl fmt::Display for MigrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MigrationStatus::Pending => write!(f, "pending"),
            MigrationStatus::Running => write!(f, "running"),
            MigrationStatus::Applied => write!(f, "applied"),
            MigrationStatus::Failed => write!(f, "failed"),
            MigrationStatus::RolledBack => write!(f, "rolled_back"),
        }
    }
}

/// fr fr Migration manager - handles schema evolution periodt
pub struct MigrationManager {
    /// Registered migrations
    migrations: BTreeMap<String, Box<dyn Migration>>,
    
    /// Migration table name
    migration_table: String,
    
    /// Whether to create migration table automatically
    auto_create_table: bool,
}

impl MigrationManager {
    /// sus Create new migration manager
    pub fn new() -> Self {
        Self {
            migrations: BTreeMap::new(),
            migration_table: "schema_migrations".to_string(),
            auto_create_table: true,
        }
    }
    
    /// facts Create migration manager with custom table name
    pub fn with_table_name(table_name: String) -> Self {
        Self {
            migrations: BTreeMap::new(),
            migration_table: table_name,
            auto_create_table: true,
        }
    }
    
    /// lowkey Register a migration
    pub fn register_migration(&mut self, migration: Box<dyn Migration>) -> SqlResult<()> {
        let version = migration.version();
        
        if self.migrations.contains_key(&version) {
            return Err(MigrationError::duplicate_version(version).into());
        }
        
        self.migrations.insert(version, migration);
        Ok(())
    }
    
    /// highkey Get all registered migrations
    pub fn get_migrations(&self) -> Vec<&dyn Migration> {
        self.migrations.values().map(|m| m.as_ref()).collect()
    }
    
    /// periodt Get migration by version
    pub fn get_migration(&self, version: &str) -> Option<&dyn Migration> {
        self.migrations.get(version).map(|m| m.as_ref())
    }
    
    /// bestie Initialize migration system (create table if needed)
    pub fn initialize(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        if self.auto_create_table {
            self.create_migration_table_if_not_exists(connection)?;
        }
        Ok(())
    }
    
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
    }
    
    /// slay Apply all pending migrations
    pub fn migrate(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<Vec<MigrationResult>> {
        let pending = self.pending_migrations(connection)?;
        let mut results = Vec::new();
        
        for migration in pending {
            let result = self.apply_migration(migration, connection)?;
            results.push(result);
        }
        
        Ok(results)
    }
    
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
        }
        
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
        }
        
        Ok(results)
    }
    
    /// energy Get migration status
    pub fn status(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<Vec<MigrationStatusInfo>> {
        self.initialize(connection)?;
        
        let applied_migrations = self.get_migration_records(connection)?;
        let mut status_list = Vec::new();
        
        for migration in self.migrations.values() {
            let record = applied_migrations.get(&migration.version());
            
            let status_info = if let Some(record) = record {
                MigrationStatusInfo {
                    version: migration.version(),
                    description: migration.description(),
                    status: record.status.clone(),
                    applied_at: record.applied_at,
                    execution_time_ms: record.execution_time_ms,
                    error_message: record.error_message.clone(),
                }
            } else {
                MigrationStatusInfo {
                    version: migration.version(),
                    description: migration.description(),
                    status: MigrationStatus::Pending,
                    applied_at: None,
                    execution_time_ms: None,
                    error_message: None,
                }
            };
            
            status_list.push(status_info);
        }
        
        Ok(status_list)
    }
    
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
    }
    
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
        }
        
        // Apply migration
        match migration.up(connection) {
            Ok(()) => {
                let execution_time = start_time.elapsed().unwrap_or_default();
                self.record_migration_success(&version, execution_time.as_millis() as u64, connection)?;
                
                Ok(MigrationResult {
                    version,
                    direction: MigrationDirection::Up,
                    success: true,
                    execution_time_ms: execution_time.as_millis() as u64,
                    error_message: None,
                })
            }
            Err(e) => {
                self.record_migration_failure(&version, &e.to_string(), connection)?;
                
                Ok(MigrationResult {
                    version,
                    direction: MigrationDirection::Up,
                    success: false,
                    execution_time_ms: start_time.elapsed().unwrap_or_default().as_millis() as u64,
                    error_message: Some(e.to_string()),
                })
            }
        }
    }
    
    /// iconic Rollback single migration
    fn rollback_migration(&self, migration: &dyn Migration, connection: &mut dyn DatabaseConnection) -> SqlResult<MigrationResult> {
        let start_time = SystemTime::now();
        let version = migration.version();
        
        if !migration.is_reversible() {
            return Err(MigrationError::not_reversible(version).into());
        }
        
        // Apply rollback
        match migration.down(connection) {
            Ok(()) => {
                let execution_time = start_time.elapsed().unwrap_or_default();
                self.record_migration_rollback(&version, connection)?;
                
                Ok(MigrationResult {
                    version,
                    direction: MigrationDirection::Down,
                    success: true,
                    execution_time_ms: execution_time.as_millis() as u64,
                    error_message: None,
                })
            }
            Err(e) => {
                Ok(MigrationResult {
                    version,
                    direction: MigrationDirection::Down,
                    success: false,
                    execution_time_ms: start_time.elapsed().unwrap_or_default().as_millis() as u64,
                    error_message: Some(e.to_string()),
                })
            }
        }
    }
    
    /// Internal: Create migration table if it doesn't exist
    fn create_migration_table_if_not_exists(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let create_table_sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                version VARCHAR(255) PRIMARY KEY,
                description TEXT,
                status VARCHAR(50) NOT NULL,
                applied_at TIMESTAMP,
                execution_time_ms BIGINT,
                error_message TEXT
            )",
            self.migration_table
        );
        
        connection.execute_statement(&create_table_sql, &[])?;
        Ok(())
    }
    
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
    }
    
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
                    "applied" => MigrationStatus::Applied,
                    "running" => MigrationStatus::Running,
                    "failed" => MigrationStatus::Failed,
                    "rolled_back" => MigrationStatus::RolledBack,
                    _ => MigrationStatus::Pending,
                };
                
                let record = MigrationRecord {
                    version: version.clone(),
                    description: row.get("description").and_then(|v| v.as_string()),
                    status,
                    applied_at: None, // Would parse timestamp here
                    execution_time_ms: row.get("execution_time_ms").and_then(|v| v.as_i64()).map(|v| v as u64),
                    error_message: row.get("error_message").and_then(|v| v.as_string()),
                };
                
                records.insert(version.clone(), record);
            }
        }
        
        Ok(records)
    }
    
    /// Internal: Record migration start
    fn record_migration_start(&self, version: &str, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let query = InsertBuilder::new(&self.migration_table)
            .columns(&["version", "status", "applied_at"])
            .values(&[
                SqlValue::String(version.to_string()),
                SqlValue::String(MigrationStatus::Running.to_string()),
                SqlValue::String("CURRENT_TIMESTAMP".to_string()),
            ])
            .build()?;
        
        connection.execute_statement(&query, &[])?;
        Ok(())
    }
    
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
    }
    
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
    }
    
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
    pub version: String,
    
    /// Direction (up or down)
    pub direction: MigrationDirection,
    
    /// Whether migration succeeded
    pub success: bool,
    
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    
    /// Error message if failed
    pub error_message: Option<String>,
}

/// fr fr Migration status information - detailed migration state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStatusInfo {
    /// Migration version
    pub version: String,
    
    /// Migration description
    pub description: String,
    
    /// Current status
    pub status: MigrationStatus,
    
    /// When migration was applied
    pub applied_at: Option<SystemTime>,
    
    /// Execution time in milliseconds
    pub execution_time_ms: Option<u64>,
    
    /// Error message if failed
    pub error_message: Option<String>,
}

/// fr fr Internal migration record from database
#[derive(Debug, Clone)]
struct MigrationRecord {
    pub version: String,
    pub description: Option<String>,
    pub status: MigrationStatus,
    pub applied_at: Option<SystemTime>,
    pub execution_time_ms: Option<u64>,
    pub error_message: Option<String>,
}

/// fr fr Migration error types - when migrations go sus
#[derive(Debug, Clone)]
pub struct MigrationError {
    pub kind: MigrationErrorKind,
    pub message: String,
    pub migration_version: Option<String>,
}

impl MigrationError {
    pub fn duplicate_version(version: String) -> Self {
        Self {
            kind: MigrationErrorKind::DuplicateVersion,
            message: format!("Migration version '{}' already registered - use unique versions bestie", version),
            migration_version: Some(version),
        }
    }
    
    pub fn migration_not_found(version: String) -> Self {
        Self {
            kind: MigrationErrorKind::MigrationNotFound,
            message: format!("Migration '{}' not found - check the version periodt", version),
            migration_version: Some(version),
        }
    }
    
    pub fn not_reversible(version: String) -> Self {
        Self {
            kind: MigrationErrorKind::NotReversible,
            message: format!("Migration '{}' is not reversible - cannot rollback bestie", version),
            migration_version: Some(version),
        }
    }
    
    pub fn missing_dependency(version: String, dependency: String) -> Self {
        Self {
            kind: MigrationErrorKind::MissingDependency,
            message: format!("Migration '{}' depends on '{}' which is not registered - fix dependencies periodt", version, dependency),
            migration_version: Some(version),
        }
    }
}

impl fmt::Display for MigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Migration Error [{}]: {}", self.kind, self.message)
    }
}

impl std::error::Error for MigrationError {}

impl From<MigrationError> for SqlError {
    fn from(err: MigrationError) -> Self {
        SqlError::configuration(err.to_string())
    }
}

/// fr fr Migration error kinds
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationErrorKind {
    DuplicateVersion,
    MigrationNotFound,
    NotReversible,
    MissingDependency,
    ValidationFailed,
    ExecutionFailed,
    RollbackFailed,
}

impl fmt::Display for MigrationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MigrationErrorKind::DuplicateVersion => write!(f, "DuplicateVersion"),
            MigrationErrorKind::MigrationNotFound => write!(f, "MigrationNotFound"),
            MigrationErrorKind::NotReversible => write!(f, "NotReversible"),
            MigrationErrorKind::MissingDependency => write!(f, "MissingDependency"),
            MigrationErrorKind::ValidationFailed => write!(f, "ValidationFailed"),
            MigrationErrorKind::ExecutionFailed => write!(f, "ExecutionFailed"),
            MigrationErrorKind::RollbackFailed => write!(f, "RollbackFailed"),
        }
    }
}

/// fr fr Example migration implementation - creates users table
pub struct CreateUsersTableMigration;

impl Migration for CreateUsersTableMigration {
    fn version(&self) -> String {
        "20240101000001_create_users_table".to_string()
    }
    
    fn description(&self) -> String {
        "Create users table with basic fields".to_string()
    }
    
    fn up(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let sql = "
            CREATE TABLE users (
                id BIGINT PRIMARY KEY AUTO_INCREMENT,
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL UNIQUE,
                age INTEGER,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )
        ";
        
        connection.execute_statement(sql, &[])?;
        
        // Create index on email for performance
        let index_sql = "CREATE INDEX idx_users_email ON users (email)";
        connection.execute_statement(index_sql, &[])?;
        
        Ok(())
    }
    
    fn down(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let sql = "DROP TABLE users";
        connection.execute_statement(sql, &[])?;
        Ok(())
    }
    
    fn is_reversible(&self) -> bool {
        true
    }
}

/// fr fr Example migration - adds posts table with foreign key
pub struct CreatePostsTableMigration;

impl Migration for CreatePostsTableMigration {
    fn version(&self) -> String {
        "20240101000002_create_posts_table".to_string()
    }
    
    fn description(&self) -> String {
        "Create posts table with user relationship".to_string()
    }
    
    fn up(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let sql = "
            CREATE TABLE posts (
                id BIGINT PRIMARY KEY AUTO_INCREMENT,
                user_id BIGINT NOT NULL,
                title VARCHAR(255) NOT NULL,
                content TEXT,
                published BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
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
    }
    
    fn down(&self, connection: &mut dyn DatabaseConnection) -> SqlResult<()> {
        let sql = "DROP TABLE posts";
        connection.execute_statement(sql, &[])?;
        Ok(())
    }
    
    fn dependencies(&self) -> Vec<String> {
        Vec::from(["20240101000001_create_users_table".to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_direction_display() {
        assert_eq!(MigrationDirection::Up.to_string(), "up");
        assert_eq!(MigrationDirection::Down.to_string(), "down");
    }

    #[test]
    fn test_migration_status_display() {
        assert_eq!(MigrationStatus::Pending.to_string(), "pending");
        assert_eq!(MigrationStatus::Applied.to_string(), "applied");
        assert_eq!(MigrationStatus::Failed.to_string(), "failed");
        assert_eq!(MigrationStatus::RolledBack.to_string(), "rolled_back");
    }

    #[test]
    fn test_migration_manager_creation() {
        let manager = MigrationManager::new();
        assert_eq!(manager.migration_table, "schema_migrations");
        assert!(manager.auto_create_table);
        assert_eq!(manager.get_migrations().len(), 0);
        
        let custom_manager = MigrationManager::with_table_name("custom_migrations".to_string());
        assert_eq!(custom_manager.migration_table, "custom_migrations");
    }

    #[test]
    fn test_migration_registration() {
        let mut manager = MigrationManager::new();
        let migration = Box::new(CreateUsersTableMigration);
        
        assert!(manager.register_migration(migration).is_ok());
        assert_eq!(manager.get_migrations().len(), 1);
        
        // Try to register duplicate
        let duplicate_migration = Box::new(CreateUsersTableMigration);
        assert!(manager.register_migration(duplicate_migration).is_err());
    }

    #[test]
    fn test_create_users_table_migration() {
        let migration = CreateUsersTableMigration;
        
        assert_eq!(migration.version(), "20240101000001_create_users_table");
        assert_eq!(migration.description(), "Create users table with basic fields");
        assert!(migration.is_reversible());
        assert_eq!(migration.dependencies().len(), 0);
    }

    #[test]
    fn test_create_posts_table_migration() {
        let migration = CreatePostsTableMigration;
        
        assert_eq!(migration.version(), "20240101000002_create_posts_table");
        assert_eq!(migration.description(), "Create posts table with user relationship");
        assert!(migration.is_reversible());
        assert_eq!(migration.dependencies().len(), 1);
        assert_eq!(migration.dependencies()[0], "20240101000001_create_users_table");
    }

    #[test]
    fn test_migration_result() {
        let result = MigrationResult {
            version: "test_migration".to_string(),
            direction: MigrationDirection::Up,
            success: true,
            execution_time_ms: 150,
            error_message: None,
        };
        
        assert_eq!(result.version, "test_migration");
        assert_eq!(result.direction, MigrationDirection::Up);
        assert!(result.success);
        assert_eq!(result.execution_time_ms, 150);
        assert!(result.error_message.is_none());
    }

    #[test]
    fn test_migration_status_info() {
        let status_info = MigrationStatusInfo {
            version: "test_migration".to_string(),
            description: "Test migration".to_string(),
            status: MigrationStatus::Applied,
            applied_at: Some(SystemTime::now()),
            execution_time_ms: Some(200),
            error_message: None,
        };
        
        assert_eq!(status_info.version, "test_migration");
        assert_eq!(status_info.status, MigrationStatus::Applied);
        assert!(status_info.applied_at.is_some());
        assert_eq!(status_info.execution_time_ms, Some(200));
    }

    #[test]
    fn test_migration_errors() {
        let duplicate_error = MigrationError::duplicate_version("test_v1".to_string());
        assert_eq!(duplicate_error.kind, MigrationErrorKind::DuplicateVersion);
        assert!(duplicate_error.message.contains("test_v1"));
        assert_eq!(duplicate_error.migration_version, Some("test_v1".to_string()));
        
        let not_found_error = MigrationError::migration_not_found("missing_v1".to_string());
        assert_eq!(not_found_error.kind, MigrationErrorKind::MigrationNotFound);
        
        let not_reversible_error = MigrationError::not_reversible("irreversible_v1".to_string());
        assert_eq!(not_reversible_error.kind, MigrationErrorKind::NotReversible);
        
        let missing_dep_error = MigrationError::missing_dependency("v2".to_string(), "v1".to_string());
        assert_eq!(missing_dep_error.kind, MigrationErrorKind::MissingDependency);
    }

    #[test]
    fn test_dependency_validation() {
        let mut manager = MigrationManager::new();
        
        // Register migrations in order
        let users_migration = Box::new(CreateUsersTableMigration);
        let posts_migration = Box::new(CreatePostsTableMigration);
        
        assert!(manager.register_migration(users_migration).is_ok());
        assert!(manager.register_migration(posts_migration).is_ok());
        
        // Validate dependencies
        assert!(manager.validate_dependencies().is_ok());
        
        // Test with missing dependency
        let mut manager_with_missing_dep = MigrationManager::new();
        let posts_only = Box::new(CreatePostsTableMigration);
        assert!(manager_with_missing_dep.register_migration(posts_only).is_ok());
        
        // This should fail because posts migration depends on users migration
        assert!(manager_with_missing_dep.validate_dependencies().is_err());
    }
}
