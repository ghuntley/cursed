use crate::error::CursedError;
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
use tracing::{info, warn, error, debug, instrument};

use super::{PackageMetadata, PackageManagerError};
use super::installer::FileOperation;

/// Local package database manager
#[derive(Debug)]
pub struct PackageDatabase {
/// Installed package record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledPackage {
/// Package dependency record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDependency {
/// Installation history record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationHistory {
/// Installation actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallAction {
/// Database errors
#[derive(CursedError, Debug)]
pub enum DatabaseError {
    #[error("Package not found: {name}")]
    
    #[error("Database corruption: {details}")]
    
    #[error("Database schema version mismatch: expected {expected}, found {found}")]
    
    #[error("Transaction failed: {reason}")]
    
    #[error("Constraint violation: {constraint}")]
    
    #[error("Serialization error: {0}")]
    
    #[error("SQLite error: {0}")]
    
    #[error("IO error: {0}")]
impl PackageDatabase {
    /// Create a new package database
    #[instrument(skip(db_path))]
    pub fn new<P: AsRef<Path>>(db_path: P) -> crate::error::Result<()> {
        let db_path = db_path.as_ref().to_path_buf();
        
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        let connection = Connection::open(&db_path)?;
        
        let mut db = Self {
        
        // Initialize database schema
        db.initialize_schema()?;
        
        info!("Package database initialized");
        Ok(db)
    /// Initialize database schema
    fn initialize_schema(&mut self) -> crate::error::Result<()> {
        // Enable foreign keys
        self.connection.execute("PRAGMA foreign_keys = ON", [])?;
        
        // Create schema version table
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY
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
                });
            }
            None => {
                // First time setup
                self.create_tables()?;
                self.connection.execute(
                )?;
                info!("Database schema created");
            }
        }
        
        Ok(())
    /// Create database tables
    fn create_tables(&mut self) -> crate::error::Result<()> {
        // Packages table
        self.connection.execute(
            "CREATE TABLE packages (
                size_bytes INTEGER
        )?;
        
        // Dependencies table
        self.connection.execute(
            "CREATE TABLE dependencies (
                FOREIGN KEY (package_name) REFERENCES packages(name) ON DELETE CASCADE
        )?;
        
        // Installation history table
        self.connection.execute(
            "CREATE TABLE installation_history (
                error_message TEXT
        )?;
        
        // Create indexes separately
        self.connection.execute(
        )?;
        
        self.connection.execute(
        )?;
        
        // File operations table (normalized for better querying)
        self.connection.execute(
            "CREATE TABLE file_operations (
                FOREIGN KEY (package_name) REFERENCES packages(name) ON DELETE CASCADE
        )?;
        
        // Create indexes separately
        self.connection.execute(
        )?;
        
        self.connection.execute(
        )?;
        
        Ok(())
    /// Add a package to the database
    #[instrument(skip(self, package), fields(package = %package.name, version = %package.version))]
    pub fn add_package(&mut self, package: &InstalledPackage) -> crate::error::Result<()> {
        let tx = self.connection.transaction()?;
        
        // Insert package record
        tx.execute(
            "INSERT OR REPLACE INTO packages 
             (name, version, install_time, install_path, metadata_json, file_operations_json) 
            params![
        )?;
        
        // Remove old dependencies
        tx.execute(
        )?;
        
        // Insert dependencies
        for (dep_name, version_spec) in &package.metadata.dependencies {
            tx.execute(
                "INSERT INTO dependencies 
                 (package_name, dependency_name, version_constraint, is_dev_dependency) 
            )?;
        for (dep_name, version_spec) in &package.metadata.dev_dependencies {
            tx.execute(
                "INSERT INTO dependencies 
                 (package_name, dependency_name, version_constraint, is_dev_dependency) 
            )?;
        // Remove old file operations
        tx.execute(
        )?;
        
        // Insert file operations
        for file_op in &package.file_operations {
            tx.execute(
                "INSERT INTO file_operations 
                 (package_name, operation_type, file_path, backup_path, permissions, size_bytes, checksum) 
                params![
            )?;
        tx.commit()?;
        
        // Record installation history
        self.record_installation_history(
        )?;
        
        info!("Package added to database");
        Ok(())
    /// Get a package from the database
    #[instrument(skip(self), fields(package_name))]
    pub fn get_package(&self, name: &str) -> crate::error::Result<()> {
        let mut stmt = self.connection.prepare(
            "SELECT name, version, install_time, install_path, metadata_json, file_operations_json 
             FROM packages WHERE name = ?1"
        )?;
        
        let package = stmt.query_row(params![name], |row| {
            let install_time_str: String = row.get(2)?;
            let install_time = DateTime::parse_from_rfc3339(&install_time_str)
                .map_err(|e| rusqlite::Error::InvalidColumnType(2, format!("Invalid datetime: {}", e).into(), rusqlite::types::Type::Text))?
                .with_timezone(&Utc);
            
            let metadata_json: String = row.get(4)?;
            let metadata: PackageMetadata = serde_json::from_str(&metadata_json)
                .map_err(|e| rusqlite::Error::InvalidColumnType(4, format!("Invalid JSON: {}", e).into(), rusqlite::types::Type::Text))?;
            
            let file_operations_json: String = row.get(5)?;
            let file_operations: Vec<FileOperation> = serde_json::from_str(&file_operations_json)
                .map_err(|e| rusqlite::Error::InvalidColumnType(5, format!("Invalid JSON: {}", e).into(), rusqlite::types::Type::Text))?;
            
            Ok(InstalledPackage {
            })
        })?;
        
        Ok(package)
    /// Remove a package from the database
    #[instrument(skip(self), fields(package_name))]
    pub fn remove_package(&mut self, name: &str) -> crate::error::Result<()> {
        let tx = self.connection.transaction()?;
        
        let rows_affected = tx.execute("DELETE FROM packages WHERE name = ?1", params![name])?;
        
        if rows_affected == 0 {
            return Err(DatabaseError::PackageNotFound { name: name.to_string() });
        tx.commit()?;
        
        // Record removal in history
        self.record_installation_history(
            "unknown", // Version not available after removal
        )?;
        
        info!("Package removed from database");
        Ok(())
    /// List all installed packages
    pub fn list_packages(&self) -> crate::error::Result<()> {
        let mut stmt = self.connection.prepare(
            "SELECT name, version, install_time, install_path, metadata_json, file_operations_json 
             FROM packages ORDER BY name"
        )?;
        
        let package_iter = stmt.query_map([], |row| {
            let install_time_str: String = row.get(2)?;
            let install_time = DateTime::parse_from_rfc3339(&install_time_str)
                .map_err(|e| rusqlite::Error::InvalidColumnType(2, format!("Invalid datetime: {}", e).into(), rusqlite::types::Type::Text))?
                .with_timezone(&Utc);
            
            let metadata_json: String = row.get(4)?;
            let metadata: PackageMetadata = serde_json::from_str(&metadata_json)
                .map_err(|e| rusqlite::Error::InvalidColumnType(4, format!("Invalid JSON: {}", e).into(), rusqlite::types::Type::Text))?;
            
            let file_operations_json: String = row.get(5)?;
            let file_operations: Vec<FileOperation> = serde_json::from_str(&file_operations_json)
                .map_err(|e| rusqlite::Error::InvalidColumnType(5, format!("Invalid JSON: {}", e).into(), rusqlite::types::Type::Text))?;
            
            Ok(InstalledPackage {
            })
        })?;
        
        let mut packages = Vec::new();
        for package in package_iter {
            packages.push(package?);
        Ok(packages)
    /// Get package dependencies
    pub fn get_dependencies(&self, package_name: &str) -> crate::error::Result<()> {
        let mut stmt = self.connection.prepare(
            "SELECT package_name, dependency_name, version_constraint, is_dev_dependency 
             FROM dependencies WHERE package_name = ?1"
        )?;
        
        let dep_iter = stmt.query_map(params![package_name], |row| {
            Ok(PackageDependency {
            })
        })?;
        
        let mut dependencies = Vec::new();
        for dep in dep_iter {
            dependencies.push(dep?);
        Ok(dependencies)
    /// Get packages that depend on a given package
    pub fn get_dependents(&self, dependency_name: &str) -> crate::error::Result<()> {
        let mut stmt = self.connection.prepare(
            "SELECT package_name, dependency_name, version_constraint, is_dev_dependency 
             FROM dependencies WHERE dependency_name = ?1"
        )?;
        
        let dep_iter = stmt.query_map(params![dependency_name], |row| {
            Ok(PackageDependency {
            })
        })?;
        
        let mut dependents = Vec::new();
        for dep in dep_iter {
            dependents.push(dep?);
        Ok(dependents)
    /// Check if a package is installed
    pub fn is_installed(&self, name: &str) -> crate::error::Result<()> {
        let count: i32 = self.connection.query_row(
        )?;
        
        Ok(count > 0)
    /// Get installation history
    pub fn get_installation_history(
    ) -> crate::error::Result<()> {
        let (query, params): (String, Vec<String>) = match package_name {
            Some(name) => (
                format!(
                    "SELECT id, package_name, version, action, timestamp, success, error_message 
                     FROM installation_history WHERE package_name = ?1 
                    limit.map(|l| format!("LIMIT {}", l)).unwrap_or_default()
            None => (
                format!(
                    "SELECT id, package_name, version, action, timestamp, success, error_message 
                     FROM installation_history 
                    limit.map(|l| format!("LIMIT {}", l)).unwrap_or_default()
        
        let mut stmt = self.connection.prepare(&query)?;
        
        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
        let history_iter = stmt.query_map(params_refs.as_slice(), |row| {
            let timestamp_str: String = row.get(4)?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| rusqlite::Error::InvalidColumnType(4, format!("Invalid datetime: {}", e).into(), rusqlite::types::Type::Text))?
                .with_timezone(&Utc);
            
            let action_str: String = row.get(3)?;
            let action: InstallAction = serde_json::from_str(&format!("\"{}\"", action_str))
                .map_err(|e| rusqlite::Error::InvalidColumnType(3, format!("Invalid action: {}", e).into(), rusqlite::types::Type::Text))?;
            
            Ok(InstallationHistory {
            })
        })?;
        
        let mut history = Vec::new();
        for entry in history_iter {
            history.push(entry?);
        Ok(history)
    /// Record installation history
    fn record_installation_history(
    ) -> crate::error::Result<()> {
        self.connection.execute(
            "INSERT INTO installation_history 
             (package_name, version, action, timestamp, success, error_message) 
            params![
        )?;
        
        Ok(())
    /// Vacuum database to reclaim space
    pub fn vacuum(&self) -> crate::error::Result<()> {
        self.connection.execute("VACUUM", [])?;
        info!("Database vacuumed");
        Ok(())
    /// Get database statistics
    pub fn get_statistics(&self) -> crate::error::Result<()> {
        let package_count: i32 = self.connection.query_row(
        )?;
        
        let total_size: Option<i64> = self.connection.query_row(
        )?;
        
        let history_count: i32 = self.connection.query_row(
        )?;
        
        let db_size = std::fs::metadata(&self.db_path)?.len();
        
        Ok(DatabaseStatistics {
        })
    /// Verify database integrity
    pub fn verify_integrity(&self) -> crate::error::Result<()> {
        // Run SQLite integrity check
        let integrity_result: String = self.connection.query_row(
        )?;
        
        if integrity_result != "ok" {
            error!("Database integrity check failed: {}", integrity_result);
            return Ok(false);
        // Verify foreign key constraints
        let fk_violations: i32 = self.connection.query_row(
        ).unwrap_or(0);
        
        if fk_violations > 0 {
            error!("Foreign key constraint violations found");
            return Ok(false);
        info!("Database integrity verified");
        Ok(true)
    }
}

/// Database statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStatistics {
impl std::fmt::Display for InstallAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
/// Thread-safe database wrapper
#[derive(Debug, Clone)]
pub struct SharedPackageDatabase {
impl SharedPackageDatabase {
    pub fn new(db_path: impl AsRef<Path>) -> crate::error::Result<()> {
        let db = PackageDatabase::new(db_path)?;
        Ok(Self {
        })
    pub fn with_db<F, R>(&self, f: F) -> crate::error::Result<()>
    where
    {
        let mut db = self.inner.lock()
            .map_err(|e| DatabaseError::TransactionFailed {
            })?;
        f(&mut *db)
    }
}
