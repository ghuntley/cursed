use crate::error::CursedError;
/// fr fr SQLite Database Driver for CURSED - the lightweight database that slays periodt
/// 
/// This module provides a complete SQLite driver implementation with FFI bindings
/// to libsqlite3. Features include connection management, prepared statements,
/// transactions, and SQLite-specific functionality like pragmas and user-defined functions.
/// 
/// Key features that make this driver periodt:
/// - Connection pooling integration with the base database system
/// - Prepared statement support with parameter binding
/// - Transaction management with ACID compliance
/// - SQLite-specific features (pragmas, UDFs, virtual tables)
/// - Thread-safe operations for concurrent access
/// - Memory-mapped I/O support for performance
/// - Backup and restore functionality
/// - Full-text search integration
/// - JSON extension support

pub mod driver;
pub mod connection;
pub mod real_connection;
pub mod production_driver;
pub mod statement;
pub mod transaction;
pub mod ffi;
pub mod config;
pub mod pragmas;
pub mod backup;
pub mod extension;
pub mod error;
pub mod utils;

// Re-export main types for convenience
pub use driver::{SqliteDriver, SqliteDriverCapabilities};
pub use connection::{SqliteConnection, SqliteConnectionInfo, ConnectionState};
pub use production_driver::{ProductionSqliteConnection, ProductionSqliteStatement, ProductionSqliteTransaction, ConnectionStats};
pub use statement::{SqliteStatement, StatementInfo, ParameterInfo};
pub use transaction::{SqliteTransaction, TransactionState, SqliteTransactionOptions};
pub use ffi::{SqliteFFI, SqliteHandle, SqliteStmtHandle, SqliteBackupHandle};
pub use config::{SqliteConfig, SqliteConnectionString, SqliteFlags, SqliteJournalMode, SqliteSynchronous};
pub use pragmas::{SqlitePragma, PragmaValue, SqlitePragmaManager};
pub use backup::{SqliteBackup, BackupProgress, BackupOptions};
pub use extension::{SqliteExtension, SqliteExtensionManager, SqliteFunction, SqliteCollation, SqliteVirtualTable};
pub use error::{SqliteError, SqliteErrorCode, SqliteResult};
pub use utils::{SqliteUtils, SqliteVersion, SqliteFeatures};

use std::sync::{Arc, Mutex, RwLock, Once};
use std::collections::HashMap;
use std::ffi::{CString, CStr};
use std::time::{SystemTime, Duration};
use super::{DatabaseError, DatabaseErrorKind, SqlValue, Driver};
use super::driver::DriverCapabilities;

/// fr fr SQLite result type
pub type Result<T> = std::result::Result<T, DatabaseError>;

/// fr fr SQLite data types supported by the driver
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteType {
impl SqliteType {
    /// slay Convert from SQLite type code
    pub fn from_code(code: i32) -> Self {
        match code {
        }
    }

    /// slay Get SQLite type code
    pub fn to_code(self) -> i32 {
        match self {
        }
    }

    /// slay Get type name for debugging
    pub fn name(self) -> &'static str {
        match self {
        }
    }
impl std::fmt::Display for SqliteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// fr fr SQLite column metadata
#[derive(Debug, Clone)]
pub struct SqliteColumnInfo {
    /// fr fr Column name
    /// fr fr Column type
    /// fr fr Whether column allows NULL
    /// fr fr Default value if any
    /// fr fr Whether column is primary key
    /// fr fr Whether column is auto-increment
    /// fr fr Column index in result set
    /// fr fr Database type name (original)
impl SqliteColumnInfo {
    /// slay Create new column info
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    /// slay Create column info with full metadata
    pub fn with_metadata(
    ) -> Self {
        Self {
        }
    }
/// fr fr SQLite statistics and performance metrics
#[derive(Debug, Clone)]
pub struct SqliteStats {
    /// fr fr Total connections created
    /// fr fr Active connections
    /// fr fr Total statements prepared
    /// fr fr Active statements
    /// fr fr Total transactions started
    /// fr fr Active transactions
    /// fr fr Total queries executed
    /// fr fr Total time spent in queries
    /// fr fr Cache hit ratio
    /// fr fr Memory usage in bytes
    /// fr fr Database file size
    /// fr fr Page count
    /// fr fr Page size
    /// fr fr WAL file size
    /// fr fr Last update time
impl Default for SqliteStats {
    fn default() -> Self {
        Self {
        }
    }
impl SqliteStats {
    /// slay Update statistics
    pub fn update(&mut self) {
        self.last_updated = SystemTime::now();
    /// slay Calculate average query time
    pub fn average_query_time(&self) -> Duration {
        if self.queries_executed > 0 {
            self.total_query_time / self.queries_executed as u32
        } else {
            Duration::ZERO
        }
    }

    /// slay Get database utilization ratio
    pub fn utilization_ratio(&self) -> f64 {
        if self.page_count > 0 {
            (self.database_size as f64) / ((self.page_count * self.page_size) as f64)
        } else {
            0.0
        }
    }
/// fr fr SQLite driver initialization state
static SQLITE_INIT: Once = Once::new();
static mut SQLITE_INITIALIZED: bool = false;

/// slay Initialize SQLite library (thread-safe)
pub fn init_sqlite() -> Result<()> {
    let mut result = Ok(());
    
    SQLITE_INIT.call_once(|| {
        match SqliteFFI::initialize() {
            Ok(_) => {
                unsafe { SQLITE_INITIALIZED = true; }
                println!("📊 SQLite driver initialized - database operations ready bestie!");
            }
            Err(e) => {
                result = Err(e);
            }
        }
    });
    
    result
/// slay Check if SQLite is initialized
pub fn is_sqlite_initialized() -> bool {
    unsafe { SQLITE_INITIALIZED }
}

/// slay Shutdown SQLite library
pub fn shutdown_sqlite() -> Result<()> {
    SqliteFFI::shutdown()
/// fr fr Global SQLite configuration
static GLOBAL_CONFIG: RwLock<Option<SqliteConfig>> = RwLock::new(None);

/// slay Set global SQLite configuration
pub fn set_global_config(config: SqliteConfig) -> Result<()> {
    let mut global_config = GLOBAL_CONFIG.write()
        .map_err(|_| SqliteError::internal("Failed to acquire config lock"))?;
    *global_config = Some(config);
    Ok(())
/// slay Get global SQLite configuration
pub fn get_global_config() -> Result<SqliteConfig> {
    let global_config = GLOBAL_CONFIG.read()
        .map_err(|_| SqliteError::internal("Failed to acquire config lock"))?;
    
    Ok(global_config.clone().unwrap_or_default())
/// fr fr SQLite utility functions
pub mod sqlite_utils {
    use super::*;
    
    /// slay Convert SqlValue to SQLite type
    pub fn sql_value_to_sqlite_type(value: &SqlValue) -> SqliteType {
        match value {
            SqlValue::Timestamp(_) => SqliteType::Text, // Store as ISO string
            SqlValue::Json(_) => SqliteType::Text,      // Store as JSON string
        }
    }

    /// slay Convert SQLite value to SqlValue
    pub fn sqlite_value_to_sql_value(
    ) -> Result<SqlValue> {
        match data_type {
            SqliteType::Integer => {
                let value = SqliteFFI::column_int64(stmt, column)?;
                Ok(SqlValue::Integer(value))
            }
            SqliteType::Real => {
                let value = SqliteFFI::column_double(stmt, column)?;
                Ok(SqlValue::Float(value))
            }
            SqliteType::Text => {
                let value = SqliteFFI::column_text(stmt, column)?;
                Ok(SqlValue::String(value))
            }
            SqliteType::Blob => {
                let value = SqliteFFI::column_blob(stmt, column)?;
                Ok(SqlValue::Bytes(value))
            }
        }
    /// slay Format connection string for logging (hide sensitive data)
    pub fn sanitize_connection_string(conn_str: &str) -> String {
        let mut sanitized = conn_str.to_string();
        
        // Hide password if present in connection string
        if let Some(start) = sanitized.find("password=") {
            if let Some(end) = sanitized[start..].find(';') {
                sanitized.replace_range(start + 9..start + end, "***");
            } else {
                sanitized.replace_range(start + 9.., "***");
            }
        }
        
        sanitized
    /// slay Validate SQLite database path
    pub fn validate_database_path(path: &str) -> Result<()> {
        if path.is_empty() {
            return Err(SqliteError::invalid_parameter("Database path cannot be empty"));
        if path == ":memory:" {
            return Ok(()); // In-memory database is valid
        // Check for invalid characters
        if path.contains('\0') {
            return Err(SqliteError::invalid_parameter("Database path contains null character"));
        Ok(())
    /// slay Get SQLite library version
    pub fn get_sqlite_version() -> Result<SqliteVersion> {
        SqliteFFI::get_version()
    /// slay Check if SQLite feature is available
    pub fn is_feature_available(feature: &str) -> bool {
        SqliteFFI::is_feature_compiled(feature).unwrap_or(false)
    /// slay Get SQLite compile options
    pub fn get_compile_options() -> Result<Vec<String>> {
        SqliteFFI::get_compile_options()
    }
}

/// fr fr Register SQLite driver with global registry
pub fn register_sqlite_driver() -> Result<()> {
    use super::driver::register_driver;
    
    init_sqlite()?;
    
    let driver = Box::new(SqliteDriver::new()?);
    register_driver("sqlite".to_string(), driver)
        .map_err(|e| SqliteError::internal(&format!("Failed to register SQLite driver: {}", e)))
