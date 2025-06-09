/// Migration system for CURSED ORM database schema management
/// 
/// Provides versioned schema migrations with rollback support,
/// dependency tracking, and database-agnostic DDL generation.

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tracing::{instrument, debug, info, warn, error};

use super::super::{DatabaseError, DatabaseErrorKind, SqlValue, DB};
use super::entity::{Entity, ColumnDefinition, SqlColumnType, IndexDefinition};
use super::schema::{TableSchema, DatabaseSchema, ColumnSchema};

/// fr fr Migration trait for defining schema changes
pub trait Migration: Send + Sync + Debug {
    /// Migration version/timestamp
    fn version(&self) -> String;
    
    /// Migration name/description
    fn name(&self) -> String;
    
    /// Dependencies (migrations that must run before this one)
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }
    
    /// Apply the migration (move schema forward)
    fn up(&self, schema: &mut DatabaseSchema) -> Result<Vec<String>, DatabaseError>;
    
    /// Rollback the migration (move schema backward)
    fn down(&self, schema: &mut DatabaseSchema) -> Result<Vec<String>, DatabaseError>;
    
    /// Check if migration can be safely applied
    fn can_apply(&self, schema: &DatabaseSchema) -> Result<bool, DatabaseError> {
        // Default: check dependencies are satisfied
        let applied_migrations = schema.get_applied_migrations();
        for dep in self.dependencies() {
            if !applied_migrations.contains(&dep) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

/// fr fr Migration operation types
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationOperation {
    CreateTable { 
        name: String,
        columns: Vec<ColumnDefinition>,
        indexes: Vec<IndexDefinition>,
    },
    DropTable { 
        name: String 
    },
    AddColumn { 
        table: String,
        column: ColumnDefinition 
    },
    DropColumn { 
        table: String,
        column: String 
    },
    AddIndex { 
        table: String,
        index: IndexDefinition 
    },
    DropIndex { 
        table: String,
        index: String 
    },
}

impl MigrationOperation {
    /// Get operation name
    pub fn name(&self) -> &str {
        match self {
            MigrationOperation::CreateTable { .. } => "create_table",
            MigrationOperation::DropTable { .. } => "drop_table",
            MigrationOperation::AddColumn { .. } => "add_column",
            MigrationOperation::DropColumn { .. } => "drop_column",
            MigrationOperation::AddIndex { .. } => "add_index",
            MigrationOperation::DropIndex { .. } => "drop_index",
        }
    }
}

/// fr fr Type aliases for migration operations
pub type CreateTable = MigrationOperation;
pub type DropTable = MigrationOperation;
pub type AddColumn = MigrationOperation;
pub type DropColumn = MigrationOperation;
pub type AddIndex = MigrationOperation;

/// fr fr Migration status tracking
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationStatus {
    /// Migration is pending (not yet applied)
    Pending,
    /// Migration has been applied successfully
    Applied { 
        applied_at: std::time::SystemTime,
        duration_ms: u64,
    },
    /// Migration failed during application
    Failed { 
        error: String,
        failed_at: std::time::SystemTime,
    },
    /// Migration is currently being applied
    InProgress { 
        started_at: std::time::SystemTime,
    },
    /// Migration was rolled back
    RolledBack { 
        rolled_back_at: std::time::SystemTime,
    },
}

/// fr fr Schema version information
#[derive(Debug, Clone)]
pub struct SchemaVersion {
    /// Current schema version
    pub version: String,
    /// Applied migrations
    pub applied_migrations: Vec<String>,
    /// Last migration timestamp
    pub last_migration_at: Option<std::time::SystemTime>,
    /// Database dialect
    pub dialect: String,
}

/// fr fr Migration manager for coordinating schema changes
#[derive(Debug)]
pub struct MigrationManager {
    /// Database connection
    db: Arc<DB>,
    /// Registered migrations
    migrations: Arc<Mutex<HashMap<String, Box<dyn Migration>>>>,
    /// Migration status tracking
    status_tracker: Arc<Mutex<HashMap<String, MigrationStatus>>>,
    /// Current schema state
    schema: Arc<Mutex<DatabaseSchema>>,
    /// Configuration
    config: MigrationConfig,
}

impl MigrationManager {
    /// slay Create new migration manager
    #[instrument(skip(db))]
    pub fn new(db: Arc<DB>) -> Self {
        info!("Creating new migration manager");
        Self {
            db,
            migrations: Arc::new(Mutex::new(HashMap::new())),
            status_tracker: Arc::new(Mutex::new(HashMap::new())),
            schema: Arc::new(Mutex::new(DatabaseSchema::new())),
            config: MigrationConfig::default(),
        }
    }

    /// facts Register a migration
    #[instrument(skip(self, migration))]
    pub fn register_migration(&self, migration: Box<dyn Migration>) -> Result<(), DatabaseError> {
        let version = migration.version();
        info!(version = %version, name = %migration.name(), "Registering migration");
        
        if let Ok(mut migrations) = self.migrations.lock() {
            if migrations.contains_key(&version) {
                return Err(DatabaseError::validation_error(
                    &format!("Migration with version {} already registered", version)
                ));
            }
            migrations.insert(version.clone(), migration);
        }
        
        // Initialize status as pending
        if let Ok(mut tracker) = self.status_tracker.lock() {
            tracker.insert(version, MigrationStatus::Pending);
        }
        
        debug!("Migration registered successfully");
        Ok(())
    }

    /// periodt Apply all pending migrations
    #[instrument(skip(self))]
    pub async fn migrate(&self) -> Result<Vec<MigrationStatus>, DatabaseError> {
        info!("Starting migration process");
        
        // Get pending migrations in dependency order
        let pending_migrations = self.get_pending_migrations()?;
        let sorted_migrations = self.sort_by_dependencies(pending_migrations)?;
        
        let mut results = Vec::new();
        
        for migration_version in sorted_migrations {
            let status = self.apply_migration(&migration_version).await?;
            results.push(status);
        }
        
        info!(applied = results.len(), "Migration process completed");
        Ok(results)
    }

    /// bestie Rollback to specific version
    #[instrument(skip(self))]
    pub async fn rollback_to(&self, target_version: &str) -> Result<Vec<MigrationStatus>, DatabaseError> {
        info!(target = target_version, "Rolling back to version");
        
        let applied_migrations = self.get_applied_migrations()?;
        let mut rollback_migrations = Vec::new();
        
        // Find migrations to rollback (reverse order)
        let mut found_target = false;
        for version in applied_migrations.iter().rev() {
            if version == target_version {
                found_target = true;
                break;
            }
            rollback_migrations.push(version.clone());
        }
        
        if !found_target {
            return Err(DatabaseError::validation_error(
                &format!("Target version {} not found in applied migrations", target_version)
            ));
        }
        
        let mut results = Vec::new();
        
        for migration_version in rollback_migrations {
            let status = self.rollback_migration(&migration_version).await?;
            results.push(status);
        }
        
        info!(rolled_back = results.len(), "Rollback process completed");
        Ok(results)
    }

    /// yolo Get current schema version
    #[instrument(skip(self))]
    pub fn current_version(&self) -> Result<SchemaVersion, DatabaseError> {
        debug!("Getting current schema version");
        
        let applied_migrations = self.get_applied_migrations()?;
        let latest_version = applied_migrations.last().cloned()
            .unwrap_or_else(|| "0.0.0".to_string());
        
        Ok(SchemaVersion {
            version: latest_version,
            applied_migrations,
            last_migration_at: Some(std::time::SystemTime::now()),
            dialect: "postgresql".to_string(), // TODO: detect from DB
        })
    }

    /// slay Check migration status
    #[instrument(skip(self))]
    pub fn migration_status(&self, version: &str) -> Option<MigrationStatus> {
        if let Ok(tracker) = self.status_tracker.lock() {
            tracker.get(version).cloned()
        } else {
            None
        }
    }

    /// lit Generate migration for entity
    #[instrument(skip(self))]
    pub fn generate_migration<T: Entity>(&self, operation: MigrationOperation) -> Result<Box<dyn Migration>, DatabaseError> {
        info!(entity = T::table_name(), operation = ?operation, "Generating migration");
        
        let version = self.generate_version();
        let migration_name = format!("{}_{}", operation.name(), T::table_name());
        
        match operation {
            MigrationOperation::CreateTable { .. } => {
                Ok(Box::new(CreateTableMigration::<T>::new(version, migration_name)))
            }
            MigrationOperation::DropTable { .. } => {
                Ok(Box::new(DropTableMigration::<T>::new(version, migration_name)))
            }
            MigrationOperation::AddColumn { column, .. } => {
                Ok(Box::new(AddColumnMigration::<T>::new(version, migration_name, column)))
            }
            MigrationOperation::DropColumn { column, .. } => {
                Ok(Box::new(DropColumnMigration::<T>::new(version, migration_name, column)))
            }
            MigrationOperation::AddIndex { .. } => {
                // TODO: Implement AddIndexMigration
                Err(DatabaseError::validation_error("AddIndex migration not yet implemented"))
            }
            MigrationOperation::DropIndex { .. } => {
                // TODO: Implement DropIndexMigration  
                Err(DatabaseError::validation_error("DropIndex migration not yet implemented"))
            }
        }
    }

    /// tea Get migration statistics
    #[instrument(skip(self))]
    pub fn stats(&self) -> super::MigrationStats {
        let status_counts = if let Ok(tracker) = self.status_tracker.lock() {
            let pending = tracker.values().filter(|s| matches!(s, MigrationStatus::Pending)).count();
            let applied = tracker.values().filter(|s| matches!(s, MigrationStatus::Applied { .. })).count();
            (pending, applied)
        } else {
            (0, 0)
        };
        
        super::MigrationStats {
            pending_migrations: status_counts.0,
            applied_migrations: status_counts.1,
        }
    }

    // Helper methods
    async fn apply_migration(&self, version: &str) -> Result<MigrationStatus, DatabaseError> {
        info!(version = version, "Applying migration");
        
        // Update status to in progress
        if let Ok(mut tracker) = self.status_tracker.lock() {
            tracker.insert(version.to_string(), MigrationStatus::InProgress {
                started_at: std::time::SystemTime::now(),
            });
        }
        
        let start_time = std::time::Instant::now();
        
        // Get migration and apply it
        let migration_exists = if let Ok(migrations) = self.migrations.lock() {
            migrations.contains_key(version)
        } else {
            return Err(DatabaseError::internal_error("Failed to access migrations"));
        };
        
        if migration_exists {
            // Execute migration with reference
            let migrations = self.migrations.lock().map_err(|_| 
                DatabaseError::internal_error("Failed to access migrations"))?;
            let migration = migrations.get(version).ok_or_else(|| 
                DatabaseError::not_found(&format!("Migration {} not found", version)))?;
            
            match self.execute_migration_up(migration.as_ref()).await {
                Ok(_) => {
                    let duration = start_time.elapsed().as_millis() as u64;
                    let status = MigrationStatus::Applied {
                        applied_at: std::time::SystemTime::now(),
                        duration_ms: duration,
                    };
                    
                    if let Ok(mut tracker) = self.status_tracker.lock() {
                        tracker.insert(version.to_string(), status.clone());
                    }
                    
                    info!(version = version, duration_ms = duration, "Migration applied successfully");
                    Ok(status)
                }
                Err(error) => {
                    let status = MigrationStatus::Failed {
                        error: error.to_string(),
                        failed_at: std::time::SystemTime::now(),
                    };
                    
                    if let Ok(mut tracker) = self.status_tracker.lock() {
                        tracker.insert(version.to_string(), status.clone());
                    }
                    
                    error!(version = version, error = %error, "Migration failed");
                    Err(error)
                }
            }
        } else {
            Err(DatabaseError::not_found(&format!("Migration {} not found", version)))
        }
    }
    
    async fn rollback_migration(&self, version: &str) -> Result<MigrationStatus, DatabaseError> {
        info!(version = version, "Rolling back migration");
        
        let migration_exists = if let Ok(migrations) = self.migrations.lock() {
            migrations.contains_key(version)
        } else {
            return Err(DatabaseError::internal_error("Failed to access migrations"));
        };
        
        if migration_exists {
            // Execute migration rollback with reference
            let migrations = self.migrations.lock().map_err(|_| 
                DatabaseError::internal_error("Failed to access migrations"))?;
            let migration = migrations.get(version).ok_or_else(|| 
                DatabaseError::not_found(&format!("Migration {} not found", version)))?;
            self.execute_migration_down(migration.as_ref()).await?;
            
            let status = MigrationStatus::RolledBack {
                rolled_back_at: std::time::SystemTime::now(),
            };
            
            if let Ok(mut tracker) = self.status_tracker.lock() {
                tracker.insert(version.to_string(), status.clone());
            }
            
            info!(version = version, "Migration rolled back successfully");
            Ok(status)
        } else {
            Err(DatabaseError::not_found(&format!("Migration {} not found", version)))
        }
    }
    
    async fn execute_migration_up(&self, migration: &dyn Migration) -> Result<(), DatabaseError> {
        debug!(migration = %migration.name(), "Executing migration up");
        
        let sql_statements = if let Ok(mut schema) = self.schema.lock() {
            migration.up(&mut schema)?
        } else {
            return Err(DatabaseError::internal_error("Failed to access schema"));
        };
        
        // Execute SQL statements
        for sql in sql_statements {
            debug!(sql = %sql, "Executing migration SQL");
            // TODO: Execute actual SQL
        }
        
        Ok(())
    }
    
    async fn execute_migration_down(&self, migration: &dyn Migration) -> Result<(), DatabaseError> {
        debug!(migration = %migration.name(), "Executing migration down");
        
        let sql_statements = if let Ok(mut schema) = self.schema.lock() {
            migration.down(&mut schema)?
        } else {
            return Err(DatabaseError::internal_error("Failed to access schema"));
        };
        
        // Execute SQL statements in reverse order
        for sql in sql_statements.iter().rev() {
            debug!(sql = %sql, "Executing rollback SQL");
            // TODO: Execute actual SQL
        }
        
        Ok(())
    }
    
    fn get_pending_migrations(&self) -> Result<Vec<String>, DatabaseError> {
        if let Ok(tracker) = self.status_tracker.lock() {
            let pending: Vec<String> = tracker.iter()
                .filter(|(_, status)| matches!(status, MigrationStatus::Pending))
                .map(|(version, _)| version.clone())
                .collect();
            Ok(pending)
        } else {
            Err(DatabaseError::internal_error("Failed to access migration status"))
        }
    }
    
    fn get_applied_migrations(&self) -> Result<Vec<String>, DatabaseError> {
        if let Ok(tracker) = self.status_tracker.lock() {
            let mut applied: Vec<String> = tracker.iter()
                .filter(|(_, status)| matches!(status, MigrationStatus::Applied { .. }))
                .map(|(version, _)| version.clone())
                .collect();
            applied.sort();
            Ok(applied)
        } else {
            Err(DatabaseError::internal_error("Failed to access migration status"))
        }
    }
    
    fn sort_by_dependencies(&self, migrations: Vec<String>) -> Result<Vec<String>, DatabaseError> {
        // TODO: Implement topological sort based on dependencies
        // For now, just sort by version
        let mut sorted = migrations;
        sorted.sort();
        Ok(sorted)
    }
    
    fn generate_version(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("{}", timestamp)
    }
}



/// fr fr Migration configuration
#[derive(Debug, Clone)]
pub struct MigrationConfig {
    /// Directory containing migration files
    pub migrations_dir: String,
    /// Whether to automatically run migrations on startup
    pub auto_migrate: bool,
    /// Maximum number of migrations to apply in one batch
    pub batch_size: usize,
    /// Migration timeout
    pub timeout: std::time::Duration,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            migrations_dir: "migrations".to_string(),
            auto_migrate: false,
            batch_size: 100,
            timeout: std::time::Duration::from_secs(300), // 5 minutes
        }
    }
}

// Concrete migration implementations

/// fr fr Create table migration
#[derive(Debug)]
pub struct CreateTableMigration<T: Entity> {
    version: String,
    name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> CreateTableMigration<T> {
    pub fn new(version: String, name: String) -> Self {
        Self {
            version,
            name,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Entity> Migration for CreateTableMigration<T> {
    fn version(&self) -> String {
        self.version.clone()
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn up(&self, schema: &mut DatabaseSchema) -> Result<Vec<String>, DatabaseError> {
        let table_schema = TableSchema::from_entity::<T>()?;
        let sql = table_schema.to_create_sql("postgresql");
        schema.add_table(table_schema);
        Ok(Vec::from([sql]))
    }
    
    fn down(&self, schema: &mut DatabaseSchema) -> Result<Vec<String>, DatabaseError> {
        let sql = format!("DROP TABLE IF EXISTS {}", T::table_name());
        schema.remove_table(T::table_name());
        Ok(Vec::from([sql]))
    }
}

/// fr fr Drop table migration
#[derive(Debug)]
pub struct DropTableMigration<T: Entity> {
    version: String,
    name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> DropTableMigration<T> {
    pub fn new(version: String, name: String) -> Self {
        Self {
            version,
            name,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Entity> Migration for DropTableMigration<T> {
    fn version(&self) -> String {
        self.version.clone()
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn up(&self, schema: &mut DatabaseSchema) -> Result<Vec<String>, DatabaseError> {
        let sql = format!("DROP TABLE IF EXISTS {}", T::table_name());
        schema.remove_table(T::table_name());
        Ok(Vec::from([sql]))
    }
    
    fn down(&self, schema: &mut DatabaseSchema) -> Result<Vec<String>, DatabaseError> {
        let table_schema = TableSchema::from_entity::<T>()?;
        let sql = table_schema.to_create_sql("postgresql");
        schema.add_table(table_schema);
        Ok(Vec::from([sql]))
    }
}

/// fr fr Add column migration
#[derive(Debug)]
pub struct AddColumnMigration<T: Entity> {
    version: String,
    name: String,
    column: ColumnDefinition,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> AddColumnMigration<T> {
    pub fn new(version: String, name: String, column: ColumnDefinition) -> Self {
        Self {
            version,
            name,
            column,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Entity> Migration for AddColumnMigration<T> {
    fn version(&self) -> String {
        self.version.clone()
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn up(&self, schema: &mut DatabaseSchema) -> Result<Vec<String>, DatabaseError> {
        let sql = format!(
            "ALTER TABLE {} ADD COLUMN {} {}{}",
            T::table_name(),
            self.column.name,
            self.column.sql_type.to_sql("postgresql"),
            if self.column.nullable { "" } else { " NOT NULL" }
        );
        
        if let Some(table) = schema.get_table_mut(T::table_name()) {
            let column_schema = ColumnSchema {
                name: self.column.name.clone(),
                sql_type: self.column.sql_type.clone(),
                nullable: self.column.nullable,
                default_value: self.column.default.clone(),
                auto_increment: false, // ColumnDefinition doesn't track this, default to false
            };
            table.add_column(column_schema);
        }
        
        Ok(Vec::from([sql]))
    }
    
    fn down(&self, schema: &mut DatabaseSchema) -> Result<Vec<String>, DatabaseError> {
        let sql = format!(
            "ALTER TABLE {} DROP COLUMN {}",
            T::table_name(),
            self.column.name
        );
        
        if let Some(table) = schema.get_table_mut(T::table_name()) {
            table.remove_column(&self.column.name);
        }
        
        Ok(Vec::from([sql]))
    }
}

/// fr fr Drop column migration
#[derive(Debug)]
pub struct DropColumnMigration<T: Entity> {
    version: String,
    name: String,
    column_name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> DropColumnMigration<T> {
    pub fn new(version: String, name: String, column_name: String) -> Self {
        Self {
            version,
            name,
            column_name,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Entity> Migration for DropColumnMigration<T> {
    fn version(&self) -> String {
        self.version.clone()
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn up(&self, schema: &mut DatabaseSchema) -> Result<Vec<String>, DatabaseError> {
        let sql = format!(
            "ALTER TABLE {} DROP COLUMN {}",
            T::table_name(),
            self.column_name
        );
        
        if let Some(table) = schema.get_table_mut(T::table_name()) {
            table.remove_column(&self.column_name);
        }
        
        Ok(Vec::from([sql]))
    }
    
    fn down(&self, schema: &mut DatabaseSchema) -> Result<Vec<String>, DatabaseError> {
        // Note: This is a destructive operation - we can't perfectly restore the column
        // In a real implementation, we'd need to store the original column definition
        warn!("Dropping column is a destructive operation - rollback may not be perfect");
        
        let sql = format!(
            "ALTER TABLE {} ADD COLUMN {} TEXT",
            T::table_name(),
            self.column_name
        );
        
        Ok(Vec::from([sql]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tracing_test::traced_test;

    #[derive(Debug, Clone)]
    struct TestUser {
        id: Option<i64>,
        name: String,
        email: String,
    }

    impl super::super::entity::Entity for TestUser {
        fn table_name() -> &'static str {
            "users"
        }

        fn primary_key_value(&self) -> Option<SqlValue> {
            self.id.map(SqlValue::Integer)
        }

        fn set_primary_key_value(&mut self, value: SqlValue) {
            if let SqlValue::Integer(id) = value {
                self.id = Some(id);
            }
        }

        fn from_row(row: &HashMap<String, SqlValue>) -> Result<Self, DatabaseError> {
            Ok(Self {
                id: None,
                name: "Test".to_string(),
                email: "test@example.com".to_string(),
            })
        }

        fn to_fields(&self) -> HashMap<String, SqlValue> {
            HashMap::new()
        }

        fn field_names() -> Vec<&'static str> {
            Vec::from(["id", "name", "email"])
        }

        fn column_definitions() -> Vec<super::super::entity::ColumnDefinition> {
            Vec::from([])
        }

        fn metadata() -> super::super::entity::EntityMetadata {
            super::super::entity::EntityMetadata {
                table_name: "users".to_string(),
                primary_key: "id".to_string(),
                fields: Vec::from(["id".to_string(), "name".to_string(), "email".to_string()]),
                relationships: Vec::from([]),
                validation_rules: Vec::from([]),
                indexes: Vec::from([]),
                version: 1,
            }
        }
    }

    fn create_mock_db() -> Arc<DB> {
        Arc::new(DB::open("test".to_string(), "".to_string()).expect("Failed to create test DB"))
    }

    #[traced_test]
    #[test]
    fn test_migration_manager_creation() {
        let db = create_mock_db();
        let manager = MigrationManager::new(db);
        
        let stats = manager.stats();
        assert_eq!(stats.pending_migrations, 0);
        assert_eq!(stats.applied_migrations, 0);
    }

    #[traced_test]
    #[test]
    fn test_migration_registration() {
        let db = create_mock_db();
        let manager = MigrationManager::new(db);
        
        let migration = Box::new(CreateTableMigration::<TestUser>::new(
            "001".to_string(),
            "create_users_table".to_string(),
        ));
        
        manager.register_migration(migration).expect("Should register migration");
        
        let status = manager.migration_status("001");
        assert!(matches!(status, Some(MigrationStatus::Pending)));
    }

    #[traced_test]
    #[test]
    fn test_migration_generation() {
        let db = create_mock_db();
        let manager = MigrationManager::new(db);
        
        let operation = MigrationOperation::CreateTable {
            name: "users".to_string(),
            columns: Vec::new(),
            indexes: Vec::new(),
        };
        
        let migration = manager.generate_migration::<TestUser>(operation)
            .expect("Should generate migration");
        
        assert_eq!(migration.name(), "create_table_users");
    }

    #[traced_test]
    #[test]
    fn test_schema_version() {
        let db = create_mock_db();
        let manager = MigrationManager::new(db);
        
        let version = manager.current_version().expect("Should get current version");
        assert_eq!(version.version, "0.0.0");
        assert_eq!(version.applied_migrations.len(), 0);
    }

    #[traced_test]
    #[test]
    fn test_create_table_migration() {
        let migration = CreateTableMigration::<TestUser>::new(
            "001".to_string(),
            "create_users_table".to_string(),
        );
        
        assert_eq!(migration.version(), "001");
        assert_eq!(migration.name(), "create_users_table");
    }
}
