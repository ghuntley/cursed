/// Local Package Database Management
/// 
/// Provides persistent storage and management of installed package metadata:
/// - SQLite-based package registry with full ACID compliance
/// - Package dependency tracking and integrity verification
/// - Installation history and rollback support
/// - Cross-platform file path handling and metadata storage

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rusqlite::{Connection, params, Row, Result as SqliteResult, OptionalExtension};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use thiserror::Error;
use tracing::{info, warn, error, debug, instrument};

use super::{PackageMetadata, PackageManagerError};
use super::installer::FileOperation;

/// Local package database manager
#[derive(Debug)]
pub struct PackageDatabase {
    connection: Connection,
    db_path: PathBuf,
}

/// Installed package record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub install_time: DateTime<Utc>,
    pub install_path: PathBuf,
    pub file_operations: Vec<FileOperation>,
    pub metadata: PackageMetadata,
}

/// Package dependency record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDependency {
    pub package_name: String,
    pub dependency_name: String,
    pub version_constraint: String,
    pub is_dev_dependency: bool,
}

/// Installation history record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationHistory {
    pub id: i64,
    pub package_name: String,
    pub version: String,
    pub action: InstallAction,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Installation actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallAction {
    Install,
    Upgrade,
    Downgrade,
    Uninstall,
    Verify,
    Repair,
}

/// Database errors
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Package not found: {name}")]
    PackageNotFound { name: String },
    
    #[error("Database corruption: {details}")]
    Corruption { details: String },
    
    #[error("Database schema version mismatch: expected {expected}, found {found}")]
    SchemaMismatch { expected: i32, found: i32 },
    
    #[error("Transaction failed: {reason}")]
    TransactionFailed { reason: String },
    
    #[error("Constraint violation: {constraint}")]
    ConstraintViolation { constraint: String },
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl PackageDatabase {
    /// Create a new package database
    #[instrument(skip(db_path))]
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<(), Error> {
        let db_path = db_path.as_ref().to_path_buf();
        
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let connection = Connection::open(&db_path)?;
        
        let mut db = Self {
            connection,
            db_path,
        };
        
        // Initialize database schema
        db.initialize_schema()?;
        
        info!("Package database initialized");
        Ok(db)
    }
    
    /// Initialize database schema
    fn initialize_schema(&mut self) -> Result<(), Error> {
        // Enable foreign keys
        self.connection.execute("PRAGMA foreign_keys = ON", [])?;
        
        // Create schema version table
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY
            )",
            [],
        )?;
        
        // Check current schema version
        let current_version: Option<i32> = self.connection
            .query_row("SELECT version FROM schema_version LIMIT 1", [], |row| {
                Ok(row.get(0)?)
            })
            .optional()?;
        
        const EXPECTED_VERSION: i32 = 1;
        
        match current_version {
            Some(version) if version == EXPECTED_VERSION => {
                debug!("Database schema is up to date");
                return Ok(());
            }
            Some(version) => {
                return Err(DatabaseError::SchemaMismatch {
                    expected: EXPECTED_VERSION,
                    found: version,
                });
            }
            None => {
                // First time setup
                self.create_tables()?;
                self.connection.execute(
                    "INSERT INTO schema_version (version) VALUES (?1)",
                    params![EXPECTED_VERSION],
                )?;
                info!("Database schema created");
            }
        }
        
        Ok(())
    }
    
    /// Create database tables
    fn create_tables(&mut self) -> Result<(), Error> {
        // Packages table
        self.connection.execute(
            "CREATE TABLE packages (
                name TEXT PRIMARY KEY,
                version TEXT NOT NULL,
                install_time TEXT NOT NULL,
                install_path TEXT NOT NULL,
                metadata_json TEXT NOT NULL,
                file_operations_json TEXT NOT NULL,
                checksum TEXT,
                size_bytes INTEGER
            )",
            [],
        )?;
        
        // Dependencies table
        self.connection.execute(
            "CREATE TABLE dependencies (
                package_name TEXT,
                dependency_name TEXT,
                version_constraint TEXT NOT NULL,
                is_dev_dependency INTEGER NOT NULL,
                PRIMARY KEY (package_name, dependency_name),
                FOREIGN KEY (package_name) REFERENCES packages(name) ON DELETE CASCADE
            )",
            [],
        )?;
        
        // Installation history table
        self.connection.execute(
            "CREATE TABLE installation_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                package_name TEXT NOT NULL,
                version TEXT NOT NULL,
                action TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                success INTEGER NOT NULL,
                error_message TEXT
            )",
            [],
        )?;
        
        // Create indexes separately
        self.connection.execute(
            "CREATE INDEX idx_package_name ON installation_history (package_name)",
            [],
        )?;
        
        self.connection.execute(
            "CREATE INDEX idx_timestamp ON installation_history (timestamp)",
            [],
        )?;
        
        // File operations table (normalized for better querying)
        self.connection.execute(
            "CREATE TABLE file_operations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                package_name TEXT NOT NULL,
                operation_type TEXT NOT NULL,
                file_path TEXT NOT NULL,
                backup_path TEXT,
                permissions INTEGER,
                size_bytes INTEGER,
                checksum TEXT,
                FOREIGN KEY (package_name) REFERENCES packages(name) ON DELETE CASCADE
            )",
            [],
        )?;
        
        // Create indexes separately
        self.connection.execute(
            "CREATE INDEX idx_file_operations_package ON file_operations (package_name)",
            [],
        )?;
        
        self.connection.execute(
            "CREATE INDEX idx_file_operations_path ON file_operations (file_path)",
            [],
        )?;
        
        Ok(())
    }
    
    /// Add a package to the database
    #[instrument(skip(self, package), fields(package = %package.name, version = %package.version))]
    pub fn add_package(&mut self, package: &InstalledPackage) -> Result<(), Error> {
        let tx = self.connection.transaction()?;
        
        // Insert package record
        tx.execute(
            "INSERT OR REPLACE INTO packages 
             (name, version, install_time, install_path, metadata_json, file_operations_json) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                package.name,
                package.version,
                package.install_time.to_rfc3339(),
                package.install_path.to_string_lossy(),
                serde_json::to_string(&package.metadata)?,
                serde_json::to_string(&package.file_operations)?,
            ],
        )?;
        
        // Remove old dependencies
        tx.execute(
            "DELETE FROM dependencies WHERE package_name = ?1",
            params![package.name],
        )?;
        
        // Insert dependencies
        for (dep_name, version_spec) in &package.metadata.dependencies {
            tx.execute(
                "INSERT INTO dependencies 
                 (package_name, dependency_name, version_constraint, is_dev_dependency) 
                 VALUES (?1, ?2, ?3, ?4)",
                params![package.name, dep_name, version_spec.to_string(), false],
            )?;
        }
        
        for (dep_name, version_spec) in &package.metadata.dev_dependencies {
            tx.execute(
                "INSERT INTO dependencies 
                 (package_name, dependency_name, version_constraint, is_dev_dependency) 
                 VALUES (?1, ?2, ?3, ?4)",
                params![package.name, dep_name, version_spec.to_string(), true],
            )?;
        }
        
        // Remove old file operations
        tx.execute(
            "DELETE FROM file_operations WHERE package_name = ?1",
            params![package.name],
        )?;
        
        // Insert file operations
        for file_op in &package.file_operations {
            tx.execute(
                "INSERT INTO file_operations 
                 (package_name, operation_type, file_path, backup_path, permissions, size_bytes, checksum) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    package.name,
                    serde_json::to_string(&file_op.operation_type)?,
                    file_op.path.to_string_lossy(),
                    file_op.backup_path.as_ref().map(|p| p.to_string_lossy().to_string()),
                    file_op.permissions,
                    file_op.size as i64,
                    file_op.checksum,
                ],
            )?;
        }
        
        tx.commit()?;
        
        // Record installation history
        self.record_installation_history(
            &package.name,
            &package.version,
            InstallAction::Install,
            true,
            None,
        )?;
        
        info!("Package added to database");
        Ok(())
    }
    
    /// Get a package from the database
    #[instrument(skip(self), fields(package_name))]
    pub fn get_package(&self, name: &str) -> Result<(), Error> {
        let mut stmt = self.connection.prepare(
            "SELECT name, version, install_time, install_path, metadata_json, file_operations_json 
             FROM packages WHERE name = ?1"
        )?;
        
        let package = stmt.query_row(params![name], |row| {
            let install_time_str: String = row.get(2)?;
            let install_time = DateTime::parse_from_rfc3339(&install_time_str)
                .map_err(|e| rusqlite::Error::InvalidColumnType(2, format!("Invalid datetime: {}", e).into(), rusqlite::crate::types::Type::Text))?
                .with_timezone(&Utc);
            
            let metadata_json: String = row.get(4)?;
            let metadata: PackageMetadata = serde_json::from_str(&metadata_json)
                .map_err(|e| rusqlite::Error::InvalidColumnType(4, format!("Invalid JSON: {}", e).into(), rusqlite::crate::types::Type::Text))?;
            
            let file_operations_json: String = row.get(5)?;
            let file_operations: Vec<FileOperation> = serde_json::from_str(&file_operations_json)
                .map_err(|e| rusqlite::Error::InvalidColumnType(5, format!("Invalid JSON: {}", e).into(), rusqlite::crate::types::Type::Text))?;
            
            Ok(InstalledPackage {
                name: row.get(0)?,
                version: row.get(1)?,
                install_time,
                install_path: PathBuf::from(row.get::<_, String>(3)?),
                file_operations,
                metadata,
            })
        })?;
        
        Ok(package)
    }
    
    /// Remove a package from the database
    #[instrument(skip(self), fields(package_name))]
    pub fn remove_package(&mut self, name: &str) -> Result<(), Error> {
        let tx = self.connection.transaction()?;
        
        let rows_affected = tx.execute("DELETE FROM packages WHERE name = ?1", params![name])?;
        
        if rows_affected == 0 {
            return Err(DatabaseError::PackageNotFound { name: name.to_string() });
        }
        
        tx.commit()?;
        
        // Record removal in history
        self.record_installation_history(
            name,
            "unknown", // Version not available after removal
            InstallAction::Uninstall,
            true,
            None,
        )?;
        
        info!("Package removed from database");
        Ok(())
    }
    
    /// List all installed packages
    pub fn list_packages(&self) -> Result<(), Error> {
        let mut stmt = self.connection.prepare(
            "SELECT name, version, install_time, install_path, metadata_json, file_operations_json 
             FROM packages ORDER BY name"
        )?;
        
        let package_iter = stmt.query_map([], |row| {
            let install_time_str: String = row.get(2)?;
            let install_time = DateTime::parse_from_rfc3339(&install_time_str)
                .map_err(|e| rusqlite::Error::InvalidColumnType(2, format!("Invalid datetime: {}", e).into(), rusqlite::crate::types::Type::Text))?
                .with_timezone(&Utc);
            
            let metadata_json: String = row.get(4)?;
            let metadata: PackageMetadata = serde_json::from_str(&metadata_json)
                .map_err(|e| rusqlite::Error::InvalidColumnType(4, format!("Invalid JSON: {}", e).into(), rusqlite::crate::types::Type::Text))?;
            
            let file_operations_json: String = row.get(5)?;
            let file_operations: Vec<FileOperation> = serde_json::from_str(&file_operations_json)
                .map_err(|e| rusqlite::Error::InvalidColumnType(5, format!("Invalid JSON: {}", e).into(), rusqlite::crate::types::Type::Text))?;
            
            Ok(InstalledPackage {
                name: row.get(0)?,
                version: row.get(1)?,
                install_time,
                install_path: PathBuf::from(row.get::<_, String>(3)?),
                file_operations,
                metadata,
            })
        })?;
        
        let mut packages = Vec::new();
        for package in package_iter {
            packages.push(package?);
        }
        
        Ok(packages)
    }
    
    /// Get package dependencies
    pub fn get_dependencies(&self, package_name: &str) -> Result<(), Error> {
        let mut stmt = self.connection.prepare(
            "SELECT package_name, dependency_name, version_constraint, is_dev_dependency 
             FROM dependencies WHERE package_name = ?1"
        )?;
        
        let dep_iter = stmt.query_map(params![package_name], |row| {
            Ok(PackageDependency {
                package_name: row.get(0)?,
                dependency_name: row.get(1)?,
                version_constraint: row.get(2)?,
                is_dev_dependency: row.get::<_, i32>(3)? != 0,
            })
        })?;
        
        let mut dependencies = Vec::new();
        for dep in dep_iter {
            dependencies.push(dep?);
        }
        
        Ok(dependencies)
    }
    
    /// Get packages that depend on a given package
    pub fn get_dependents(&self, dependency_name: &str) -> Result<(), Error> {
        let mut stmt = self.connection.prepare(
            "SELECT package_name, dependency_name, version_constraint, is_dev_dependency 
             FROM dependencies WHERE dependency_name = ?1"
        )?;
        
        let dep_iter = stmt.query_map(params![dependency_name], |row| {
            Ok(PackageDependency {
                package_name: row.get(0)?,
                dependency_name: row.get(1)?,
                version_constraint: row.get(2)?,
                is_dev_dependency: row.get::<_, i32>(3)? != 0,
            })
        })?;
        
        let mut dependents = Vec::new();
        for dep in dep_iter {
            dependents.push(dep?);
        }
        
        Ok(dependents)
    }
    
    /// Check if a package is installed
    pub fn is_installed(&self, name: &str) -> Result<(), Error> {
        let count: i32 = self.connection.query_row(
            "SELECT COUNT(*) FROM packages WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;
        
        Ok(count > 0)
    }
    
    /// Get installation history
    pub fn get_installation_history(
        &self,
        package_name: Option<&str>,
        limit: Option<usize>,
    ) -> Result<(), Error> {
        let (query, params): (String, Vec<String>) = match package_name {
            Some(name) => (
                format!(
                    "SELECT id, package_name, version, action, timestamp, success, error_message 
                     FROM installation_history WHERE package_name = ?1 
                     ORDER BY timestamp DESC {}",
                    limit.map(|l| format!("LIMIT {}", l)).unwrap_or_default()
                ),
                vec![name.to_string()],
            ),
            None => (
                format!(
                    "SELECT id, package_name, version, action, timestamp, success, error_message 
                     FROM installation_history 
                     ORDER BY timestamp DESC {}",
                    limit.map(|l| format!("LIMIT {}", l)).unwrap_or_default()
                ),
                vec![],
            ),
        };
        
        let mut stmt = self.connection.prepare(&query)?;
        
        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
        let history_iter = stmt.query_map(params_refs.as_slice(), |row| {
            let timestamp_str: String = row.get(4)?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| rusqlite::Error::InvalidColumnType(4, format!("Invalid datetime: {}", e).into(), rusqlite::crate::types::Type::Text))?
                .with_timezone(&Utc);
            
            let action_str: String = row.get(3)?;
            let action: InstallAction = serde_json::from_str(&format!("\"{}\"", action_str))
                .map_err(|e| rusqlite::Error::InvalidColumnType(3, format!("Invalid action: {}", e).into(), rusqlite::crate::types::Type::Text))?;
            
            Ok(InstallationHistory {
                id: row.get(0)?,
                package_name: row.get(1)?,
                version: row.get(2)?,
                action,
                timestamp,
                success: row.get::<_, i32>(5)? != 0,
                error_message: row.get(6)?,
            })
        })?;
        
        let mut history = Vec::new();
        for entry in history_iter {
            history.push(entry?);
        }
        
        Ok(history)
    }
    
    /// Record installation history
    fn record_installation_history(
        &self,
        package_name: &str,
        version: &str,
        action: InstallAction,
        success: bool,
        error_message: Option<String>,
    ) -> Result<(), Error> {
        self.connection.execute(
            "INSERT INTO installation_history 
             (package_name, version, action, timestamp, success, error_message) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                package_name,
                version,
                serde_json::to_string(&action)?.trim_matches('"'),
                Utc::now().to_rfc3339(),
                if success { 1 } else { 0 },
                error_message,
            ],
        )?;
        
        Ok(())
    }
    
    /// Vacuum database to reclaim space
    pub fn vacuum(&self) -> Result<(), Error> {
        self.connection.execute("VACUUM", [])?;
        info!("Database vacuumed");
        Ok(())
    }
    
    /// Get database statistics
    pub fn get_statistics(&self) -> Result<(), Error> {
        let package_count: i32 = self.connection.query_row(
            "SELECT COUNT(*) FROM packages",
            [],
            |row| row.get(0),
        )?;
        
        let total_size: Option<i64> = self.connection.query_row(
            "SELECT SUM(size_bytes) FROM file_operations",
            [],
            |row| row.get(0),
        )?;
        
        let history_count: i32 = self.connection.query_row(
            "SELECT COUNT(*) FROM installation_history",
            [],
            |row| row.get(0),
        )?;
        
        let db_size = std::fs::metadata(&self.db_path)?.len();
        
        Ok(DatabaseStatistics {
            package_count: package_count as usize,
            total_installed_size: total_size.unwrap_or(0) as u64,
            history_entries: history_count as usize,
            database_size: db_size,
        })
    }
    
    /// Verify database integrity
    pub fn verify_integrity(&self) -> Result<(), Error> {
        // Run SQLite integrity check
        let integrity_result: String = self.connection.query_row(
            "PRAGMA integrity_check",
            [],
            |row| row.get(0),
        )?;
        
        if integrity_result != "ok" {
            error!("Database integrity check failed: {}", integrity_result);
            return Ok(false);
        }
        
        // Verify foreign key constraints
        let fk_violations: i32 = self.connection.query_row(
            "PRAGMA foreign_key_check",
            [],
            |row| Ok(1),
        ).unwrap_or(0);
        
        if fk_violations > 0 {
            error!("Foreign key constraint violations found");
            return Ok(false);
        }
        
        info!("Database integrity verified");
        Ok(true)
    }
}

/// Database statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStatistics {
    pub package_count: usize,
    pub total_installed_size: u64,
    pub history_entries: usize,
    pub database_size: u64,
}

impl std::fmt::Display for InstallAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallAction::Install => write!(f, "install"),
            InstallAction::Upgrade => write!(f, "upgrade"),
            InstallAction::Downgrade => write!(f, "downgrade"),
            InstallAction::Uninstall => write!(f, "uninstall"),
            InstallAction::Verify => write!(f, "verify"),
            InstallAction::Repair => write!(f, "repair"),
        }
    }
}

/// Thread-safe database wrapper
#[derive(Debug, Clone)]
pub struct SharedPackageDatabase {
    inner: Arc<Mutex<PackageDatabase>>,
}

impl SharedPackageDatabase {
    pub fn new(db_path: impl AsRef<Path>) -> Result<(), Error> {
        let db = PackageDatabase::new(db_path)?;
        Ok(Self {
            inner: Arc::new(Mutex::new(db)),
        })
    }
    
    pub fn with_db<F, R>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut PackageDatabase) -> Result<(), Error>,
    {
        let mut db = self.inner.lock()
            .map_err(|e| DatabaseError::TransactionFailed {
                reason: format!("Failed to acquire lock: {}", e),
            })?;
        f(&mut *db)
    }
}
