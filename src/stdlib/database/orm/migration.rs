/// Migration system for CURSED ORM database schema management
/// 
/// Provides versioned schema migrations with rollback support,
/// dependency tracking, and database-agnostic DDL generation.

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tracing::{instrument, debug, info, warn, error};

use super::super::{DatabaseError, DatabaseErrorKind, SqlValue, DB};
use super::entity::{Entity, ColumnDefinition, SqlColumnType, IndexDefinition, IndexType};
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
    fn up(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()>;
    
    /// Rollback the migration (move schema backward)
    fn down(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()>;
    
    /// Check if migration can be safely applied
    fn can_apply(&self, schema: &DatabaseSchema) -> crate::error::Result<()> {
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
    pub fn register_migration(&self, migration: Box<dyn Migration>) -> crate::error::Result<()> {
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
    pub async fn migrate(&self) -> crate::error::Result<()> {
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
    pub async fn rollback_to(&self, target_version: &str) -> crate::error::Result<()> {
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
    pub fn current_version(&self) -> crate::error::Result<()> {
        debug!("Getting current schema version");
        
        let applied_migrations = self.get_applied_migrations()?;
        let latest_version = applied_migrations.last().cloned()
            .unwrap_or_else(|| "0.0.0".to_string());
        
        let dialect = self.detect_database_dialect()?;
        
        Ok(SchemaVersion {
            version: latest_version,
            applied_migrations,
            last_migration_at: Some(std::time::SystemTime::now()),
            dialect,
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
    pub fn generate_migration<T: Entity>(&self, operation: MigrationOperation) -> crate::error::Result<()> {
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
            MigrationOperation::AddIndex { index, .. } => {
                Ok(Box::new(AddIndexMigration::<T>::new(version, migration_name, index)))
            }
            MigrationOperation::DropIndex { index, .. } => {
                Ok(Box::new(DropIndexMigration::<T>::new(version, migration_name, index)))
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
    async fn apply_migration(&self, version: &str) -> crate::error::Result<()> {
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
    
    async fn rollback_migration(&self, version: &str) -> crate::error::Result<()> {
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
    
    async fn execute_migration_up(&self, migration: &dyn Migration) -> crate::error::Result<()> {
        debug!(migration = %migration.name(), "Executing migration up");
        
        let sql_statements = if let Ok(mut schema) = self.schema.lock() {
            migration.up(&mut schema)?
        } else {
            return Err(DatabaseError::internal_error("Failed to access schema"));
        };
        
        // Execute SQL statements within a transaction
        let mut tx = self.db.begin().map_err(|e| {
            error!(error = %e, "Failed to begin transaction for migration");
            e
        })?;
        
        for sql in sql_statements {
            debug!(sql = %sql, "Executing migration SQL");
            tx.exec(sql, Vec::new()).map_err(|e| {
                error!(sql = %sql, error = %e, "Failed to execute migration SQL");
                e
            })?;
        }
        
        tx.commit().map_err(|e| {
            error!(error = %e, "Failed to commit migration transaction");
            e
        })?;
        
        Ok(())
    }
    
    async fn execute_migration_down(&self, migration: &dyn Migration) -> crate::error::Result<()> {
        debug!(migration = %migration.name(), "Executing migration down");
        
        let sql_statements = if let Ok(mut schema) = self.schema.lock() {
            migration.down(&mut schema)?
        } else {
            return Err(DatabaseError::internal_error("Failed to access schema"));
        };
        
        // Execute SQL statements in reverse order within a transaction
        let mut tx = self.db.begin().map_err(|e| {
            error!(error = %e, "Failed to begin transaction for rollback");
            e
        })?;
        
        for sql in sql_statements.iter().rev() {
            debug!(sql = %sql, "Executing rollback SQL");
            tx.exec(sql.clone(), Vec::new()).map_err(|e| {
                error!(sql = %sql, error = %e, "Failed to execute rollback SQL");
                e
            })?;
        }
        
        tx.commit().map_err(|e| {
            error!(error = %e, "Failed to commit rollback transaction");
            e
        })?;
        
        Ok(())
    }
    
    fn get_pending_migrations(&self) -> crate::error::Result<()> {
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
    
    fn get_applied_migrations(&self) -> crate::error::Result<()> {
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
    
    fn sort_by_dependencies(&self, migrations: Vec<String>) -> crate::error::Result<()> {
        debug!("Sorting migrations by dependencies");
        
        let migrations_guard = self.migrations.lock().map_err(|_| 
            DatabaseError::internal_error("Failed to access migrations for dependency sorting"))?;
        
        // Kahn's algorithm for topological sorting
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        
        // Build dependency graph
        for migration_version in &migrations {
            if let Some(migration) = migrations_guard.get(migration_version) {
                let deps = migration.dependencies();
                graph.insert(migration_version.clone(), deps.clone());
                
                // Initialize in-degree count
                in_degree.entry(migration_version.clone()).or_insert(0);
                
                // Update in-degrees based on dependencies
                for dep in deps {
                    if migrations.contains(&dep) {
                        *in_degree.entry(migration_version.clone()).or_insert(0) += 1;
                    }
                }
            }
        }
        
        // Find migrations with no dependencies (in-degree 0)
        let mut queue: Vec<String> = in_degree.iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(version, _)| version.clone())
            .collect();
        
        let mut result = Vec::new();
        
        // Process migrations in topological order
        while let Some(current) = queue.pop() {
            result.push(current.clone());
            
            // Update dependencies
            if let Some(deps) = graph.get(&current) {
                for dep_version in deps {
                    if let Some(degree) = in_degree.get_mut(dep_version) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(dep_version.clone());
                        }
                    }
                }
            }
        }
        
        // Check for circular dependencies
        if result.len() != migrations.len() {
            let remaining: Vec<String> = migrations.into_iter()
                .filter(|m| !result.contains(m))
                .collect();
            
            return Err(DatabaseError::validation_error(
                &format!("Circular dependency detected in migrations: {:?}", remaining)
            ));
        }
        
        debug!(sorted_migrations = ?result, "Sorted migrations by dependencies");
        Ok(result)
    }
    
    fn generate_version(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("{}", timestamp)
    }

    /// Detect database dialect from connection information
    /// 
    /// This method analyzes the database driver name and connection string to determine
    /// the appropriate SQL dialect for generating migration statements. It supports
    /// PostgreSQL, MySQL, SQLite, SQL Server, and Oracle databases.
    /// 
    /// # Returns
    /// 
    /// A string representing the detected database dialect, or "postgresql" as default
    /// 
    /// # Supported Dialects
    /// 
    /// - "postgresql" - PostgreSQL database
    /// - "mysql" - MySQL/MariaDB database  
    /// - "sqlite" - SQLite database
    /// - "mssql" - Microsoft SQL Server
    /// - "oracle" - Oracle Database
    /// 
    /// # Detection Logic
    /// 
    /// 1. First checks the driver name for known database identifiers
    /// 2. Falls back to analyzing the connection string/data source name
    /// 3. Defaults to PostgreSQL if detection fails
    #[instrument(skip(self))]
    fn detect_database_dialect(&self) -> crate::error::Result<()> {
        debug!("Detecting database dialect");
        
        let driver_name = &self.db.driver_name;
        let data_source = &self.db.data_source_name;
        
        // Detect based on driver name first
        let dialect = match driver_name.to_lowercase().as_str() {
            "postgres" | "postgresql" | "pg" => "postgresql",
            "mysql" => "mysql", 
            "sqlite" | "sqlite3" => "sqlite",
            "mssql" | "sqlserver" => "mssql",
            "oracle" => "oracle",
            _ => {
                // Try to detect from data source name/connection string
                if data_source.contains("postgres") || data_source.contains("postgresql") {
                    "postgresql"
                } else if data_source.contains("mysql") {
                    "mysql"
                } else if data_source.contains("sqlite") || data_source.ends_with(".db") || data_source.ends_with(".sqlite") {
                    "sqlite"
                } else if data_source.contains("sqlserver") || data_source.contains("mssql") {
                    "mssql"
                } else if data_source.contains("oracle") {
                    "oracle"
                } else {
                    // Default to PostgreSQL if we can't detect
                    warn!(driver = driver_name, data_source = data_source, "Could not detect database dialect, defaulting to PostgreSQL");
                    "postgresql"
                }
            }
        };
        
        debug!(dialect = dialect, "Detected database dialect");
        Ok(dialect.to_string())
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
    
    fn up(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()> {
        let table_schema = TableSchema::from_entity::<T>()?;
        let sql = table_schema.to_create_sql("postgresql");
        schema.add_table(table_schema);
        Ok(Vec::from([sql]))
    }
    
    fn down(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()> {
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
    
    fn up(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()> {
        let sql = format!("DROP TABLE IF EXISTS {}", T::table_name());
        schema.remove_table(T::table_name());
        Ok(Vec::from([sql]))
    }
    
    fn down(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()> {
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
    
    fn up(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()> {
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
    
    fn down(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()> {
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
    
    fn up(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()> {
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
    
    fn down(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()> {
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

/// Migration for adding database indexes to improve query performance
/// 
/// This migration creates a new index on specified columns of an entity's table.
/// Supports unique indexes, composite indexes, partial indexes with WHERE clauses,
/// and different index types (B-tree, Hash, GIN, GiST for PostgreSQL).
/// 
/// # Examples
/// 
/// ```rust
/// use cursed::stdlib::database::orm::migration::AddIndexMigration;
/// use cursed::stdlib::database::orm::entity::{IndexDefinition, IndexType};
/// 
/// let index = IndexDefinition {
///     name: "idx_users_email".to_string(),
///     columns: vec!["email".to_string()],
///     unique: true,
///     index_type: IndexType::BTree,
///     condition: None,
/// };
/// 
/// let migration = AddIndexMigration::<User>::new(
///     "20231201_001".to_string(),
///     "add_users_email_index".to_string(),
///     index,
/// );
/// ```
#[derive(Debug)]
pub struct AddIndexMigration<T: Entity> {
    version: String,
    name: String,
    index: IndexDefinition,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> AddIndexMigration<T> {
    /// Create a new index addition migration
    /// 
    /// # Arguments
    /// 
    /// * `version` - Unique version identifier for this migration (e.g., timestamp)
    /// * `name` - Descriptive name for this migration operation
    /// * `index` - Index definition containing name, columns, type, and constraints
    /// 
    /// # Returns
    /// 
    /// A new `AddIndexMigration` instance ready for registration with the migration manager
    pub fn new(version: String, name: String, index: IndexDefinition) -> Self {
        Self {
            version,
            name,
            index,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Entity> Migration for AddIndexMigration<T> {
    fn version(&self) -> String {
        self.version.clone()
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn up(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()> {
        let unique_keyword = if self.index.unique { "UNIQUE " } else { "" };
        let condition_clause = if let Some(condition) = &self.index.condition {
            format!(" WHERE {}", condition)
        } else {
            String::new()
        };
        
        let sql = format!(
            "CREATE {}INDEX {} ON {} ({}){};",
            unique_keyword,
            self.index.name,
            T::table_name(),
            self.index.columns.join(", "),
            condition_clause
        );
        
        // Add index to schema
        if let Some(table) = schema.get_table_mut(T::table_name()) {
            table.indexes.push(self.index.clone());
        }
        
        Ok(Vec::from([sql]))
    }
    
    fn down(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()> {
        let sql = format!("DROP INDEX IF EXISTS {};", self.index.name);
        
        // Remove index from schema
        if let Some(table) = schema.get_table_mut(T::table_name()) {
            table.indexes.retain(|idx| idx.name != self.index.name);
        }
        
        Ok(Vec::from([sql]))
    }
}

/// Migration for removing database indexes
/// 
/// This migration drops an existing index from an entity's table. Note that this is a 
/// destructive operation and the rollback implementation creates a basic B-tree index
/// which may not match the original index specification.
/// 
/// # Examples
/// 
/// ```rust
/// use cursed::stdlib::database::orm::migration::DropIndexMigration;
/// 
/// let migration = DropIndexMigration::<User>::new(
///     "20231201_002".to_string(),
///     "drop_users_email_index".to_string(),
///     "idx_users_email".to_string(),
/// );
/// ```
/// 
/// # Warning
/// 
/// Dropping indexes is destructive. Rollback operations will create a basic index
/// on the 'id' column but cannot restore the original index specification.
#[derive(Debug)]
pub struct DropIndexMigration<T: Entity> {
    version: String,
    name: String,
    index_name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> DropIndexMigration<T> {
    /// Create a new index removal migration
    /// 
    /// # Arguments
    /// 
    /// * `version` - Unique version identifier for this migration (e.g., timestamp)
    /// * `name` - Descriptive name for this migration operation
    /// * `index_name` - Name of the index to be dropped
    /// 
    /// # Returns
    /// 
    /// A new `DropIndexMigration` instance ready for registration with the migration manager
    pub fn new(version: String, name: String, index_name: String) -> Self {
        Self {
            version,
            name,
            index_name,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Entity> Migration for DropIndexMigration<T> {
    fn version(&self) -> String {
        self.version.clone()
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn up(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()> {
        let sql = format!("DROP INDEX IF EXISTS {};", self.index_name);
        
        // Remove index from schema
        if let Some(table) = schema.get_table_mut(T::table_name()) {
            table.indexes.retain(|idx| idx.name != self.index_name);
        }
        
        Ok(Vec::from([sql]))
    }
    
    fn down(&self, schema: &mut DatabaseSchema) -> crate::error::Result<()> {
        // Note: This is a destructive operation - we can't perfectly restore the index
        // In a real implementation, we'd need to store the original index definition
        warn!("Dropping index is a destructive operation - rollback may not be perfect");
        
        // Create a basic B-tree index for rollback
        let sql = format!(
            "CREATE INDEX {} ON {} ({});",
            self.index_name,
            T::table_name(),
            "id" // Default to 'id' column if we don't know the original columns
        );
        
        Ok(Vec::from([sql]))
    }
}

/// Implementation of IndexType SQL generation methods for cross-database compatibility
impl IndexType {
    /// Generate database-specific index type clause for CREATE INDEX statements
    /// 
    /// This method converts CURSED's generic index types into database-specific SQL clauses.
    /// Different databases support different index types and syntax.
    /// 
    /// # Arguments
    /// 
    /// * `dialect` - Target database dialect ("postgresql", "mysql", "sqlite", etc.)
    /// 
    /// # Returns
    /// 
    /// Database-specific SQL clause for index type, or empty string if not supported
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use cursed::stdlib::database::orm::entity::IndexType;
    /// 
    /// assert_eq!(IndexType::BTree.to_sql("postgresql"), "USING btree");
    /// assert_eq!(IndexType::Hash.to_sql("mysql"), "USING HASH");
    /// assert_eq!(IndexType::BTree.to_sql("sqlite"), ""); // SQLite doesn't support type specification
    /// ```
    pub fn to_sql(&self, dialect: &str) -> String {
        match (self, dialect) {
            (IndexType::BTree, "postgresql") => "USING btree",
            (IndexType::Hash, "postgresql") => "USING hash",
            (IndexType::Gin, "postgresql") => "USING gin",
            (IndexType::Gist, "postgresql") => "USING gist",
            (IndexType::BTree, "mysql") => "USING BTREE",
            (IndexType::Hash, "mysql") => "USING HASH",
            (IndexType::BTree, "sqlite") => "", // SQLite doesn't support type specification
            (IndexType::Hash, "sqlite") => "",
            _ => "", // Default: no specific type clause
        }.to_string()
    }
}

