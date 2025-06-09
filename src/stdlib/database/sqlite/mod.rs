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
pub use statement::{SqliteStatement, StatementInfo, ParameterInfo};
pub use transaction::{SqliteTransaction, TransactionState, SqliteTransactionOptions};
pub use ffi::{SqliteFFI, SqliteHandle, SqliteStmtHandle, SqliteBackupHandle};
pub use config::{SqliteConfig, SqliteConnectionString, SqliteFlags, SqliteJournalMode, SqliteSynchronous};
pub use pragmas::{SqlitePragma, PragmaValue, SqlitePragmaManager};
pub use backup::{SqliteBackup, BackupProgress, BackupOptions};
pub use extension::{SqliteExtension, SqliteFunction, SqliteCollation, SqliteVirtualTable};
pub use error::{SqliteError, SqliteErrorCode, SqliteResult};
pub use utils::{SqliteUtils, SqliteVersion, SqliteFeatures};

use std::sync::{Arc, Mutex, RwLock, Once};
use std::collections::HashMap;
use std::ffi::{CString, CStr};
use std::time::{SystemTime, Duration};
use super::{DatabaseError, DatabaseErrorKind, SqlValue, Driver};
use super::driver::DriverCapabilities;

/// fr fr SQLite result type
pub type Result<T> = std::result::Result<T, SqliteError>;

/// fr fr SQLite data types supported by the driver
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteType {
    Null,
    Integer,
    Real,
    Text,
    Blob,
}

impl SqliteType {
    /// slay Convert from SQLite type code
    pub fn from_code(code: i32) -> Self {
        match code {
            1 => SqliteType::Integer,
            2 => SqliteType::Real, 
            3 => SqliteType::Text,
            4 => SqliteType::Blob,
            _ => SqliteType::Null,
        }
    }

    /// slay Get SQLite type code
    pub fn to_code(self) -> i32 {
        match self {
            SqliteType::Null => 5,
            SqliteType::Integer => 1,
            SqliteType::Real => 2,
            SqliteType::Text => 3,
            SqliteType::Blob => 4,
        }
    }

    /// slay Get type name for debugging
    pub fn name(self) -> &'static str {
        match self {
            SqliteType::Null => "NULL",
            SqliteType::Integer => "INTEGER",
            SqliteType::Real => "REAL",
            SqliteType::Text => "TEXT", 
            SqliteType::Blob => "BLOB",
        }
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
    pub name: String,
    /// fr fr Column type
    pub data_type: SqliteType,
    /// fr fr Whether column allows NULL
    pub nullable: bool,
    /// fr fr Default value if any
    pub default_value: Option<SqlValue>,
    /// fr fr Whether column is primary key
    pub primary_key: bool,
    /// fr fr Whether column is auto-increment
    pub auto_increment: bool,
    /// fr fr Column index in result set
    pub index: usize,
    /// fr fr Database type name (original)
    pub type_name: String,
}

impl SqliteColumnInfo {
    /// slay Create new column info
    pub fn new(
        name: String,
        data_type: SqliteType,
        index: usize,
        type_name: String,
    ) -> Self {
        Self {
            name,
            data_type,
            nullable: true,
            default_value: None,
            primary_key: false,
            auto_increment: false,
            index,
            type_name,
        }
    }

    /// slay Create column info with full metadata
    pub fn with_metadata(
        name: String,
        data_type: SqliteType,
        nullable: bool,
        default_value: Option<SqlValue>,
        primary_key: bool,
        auto_increment: bool,
        index: usize,
        type_name: String,
    ) -> Self {
        Self {
            name,
            data_type,
            nullable,
            default_value,
            primary_key,
            auto_increment,
            index,
            type_name,
        }
    }
}

/// fr fr SQLite statistics and performance metrics
#[derive(Debug, Clone)]
pub struct SqliteStats {
    /// fr fr Total connections created
    pub connections_created: u64,
    /// fr fr Active connections
    pub active_connections: u64,
    /// fr fr Total statements prepared
    pub statements_prepared: u64,
    /// fr fr Active statements
    pub active_statements: u64,
    /// fr fr Total transactions started
    pub transactions_started: u64,
    /// fr fr Active transactions
    pub active_transactions: u64,
    /// fr fr Total queries executed
    pub queries_executed: u64,
    /// fr fr Total time spent in queries
    pub total_query_time: Duration,
    /// fr fr Cache hit ratio
    pub cache_hit_ratio: f64,
    /// fr fr Memory usage in bytes
    pub memory_usage: u64,
    /// fr fr Database file size
    pub database_size: u64,
    /// fr fr Page count
    pub page_count: u64,
    /// fr fr Page size
    pub page_size: u64,
    /// fr fr WAL file size
    pub wal_size: u64,
    /// fr fr Last update time
    pub last_updated: SystemTime,
}

impl Default for SqliteStats {
    fn default() -> Self {
        Self {
            connections_created: 0,
            active_connections: 0,
            statements_prepared: 0,
            active_statements: 0,
            transactions_started: 0,
            active_transactions: 0,
            queries_executed: 0,
            total_query_time: Duration::from_secs(0),
            cache_hit_ratio: 0.0,
            memory_usage: 0,
            database_size: 0,
            page_count: 0,
            page_size: 4096,
            wal_size: 0,
            last_updated: SystemTime::UNIX_EPOCH,
        }
    }
}

impl SqliteStats {
    /// slay Update statistics
    pub fn update(&mut self) {
        self.last_updated = SystemTime::now();
    }

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
}

/// slay Check if SQLite is initialized
pub fn is_sqlite_initialized() -> bool {
    unsafe { SQLITE_INITIALIZED }
}

/// slay Shutdown SQLite library
pub fn shutdown_sqlite() -> Result<()> {
    SqliteFFI::shutdown()
}

/// fr fr Global SQLite configuration
static GLOBAL_CONFIG: RwLock<Option<SqliteConfig>> = RwLock::new(None);

/// slay Set global SQLite configuration
pub fn set_global_config(config: SqliteConfig) -> Result<()> {
    let mut global_config = GLOBAL_CONFIG.write()
        .map_err(|_| SqliteError::internal("Failed to acquire config lock"))?;
    *global_config = Some(config);
    Ok(())
}

/// slay Get global SQLite configuration
pub fn get_global_config() -> Result<SqliteConfig> {
    let global_config = GLOBAL_CONFIG.read()
        .map_err(|_| SqliteError::internal("Failed to acquire config lock"))?;
    
    Ok(global_config.clone().unwrap_or_default())
}

/// fr fr SQLite utility functions
pub mod sqlite_utils {
    use super::*;
    
    /// slay Convert SqlValue to SQLite type
    pub fn sql_value_to_sqlite_type(value: &SqlValue) -> SqliteType {
        match value {
            SqlValue::Null => SqliteType::Null,
            SqlValue::Boolean(_) | SqlValue::Integer(_) => SqliteType::Integer,
            SqlValue::Float(_) => SqliteType::Real,
            SqlValue::String(_) => SqliteType::Text,
            SqlValue::Bytes(_) => SqliteType::Blob,
            SqlValue::Timestamp(_) => SqliteType::Text, // Store as ISO string
            SqlValue::Json(_) => SqliteType::Text,      // Store as JSON string
        }
    }

    /// slay Convert SQLite value to SqlValue
    pub fn sqlite_value_to_sql_value(
        stmt: &SqliteStmtHandle,
        column: i32,
        data_type: SqliteType,
    ) -> Result<SqlValue> {
        match data_type {
            SqliteType::Null => Ok(SqlValue::Null),
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
    }

    /// slay Validate SQLite database path
    pub fn validate_database_path(path: &str) -> Result<()> {
        if path.is_empty() {
            return Err(SqliteError::invalid_parameter("Database path cannot be empty"));
        }
        
        if path == ":memory:" {
            return Ok(()); // In-memory database is valid
        }
        
        // Check for invalid characters
        if path.contains('\0') {
            return Err(SqliteError::invalid_parameter("Database path contains null character"));
        }
        
        Ok(())
    }

    /// slay Get SQLite library version
    pub fn get_sqlite_version() -> Result<SqliteVersion> {
        SqliteFFI::get_version()
    }

    /// slay Check if SQLite feature is available
    pub fn is_feature_available(feature: &str) -> bool {
        SqliteFFI::is_feature_compiled(feature).unwrap_or(false)
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_type_conversion() {
        assert_eq!(SqliteType::from_code(1), SqliteType::Integer);
        assert_eq!(SqliteType::from_code(2), SqliteType::Real);
        assert_eq!(SqliteType::from_code(3), SqliteType::Text);
        assert_eq!(SqliteType::from_code(4), SqliteType::Blob);
        assert_eq!(SqliteType::from_code(99), SqliteType::Null);

        assert_eq!(SqliteType::Integer.to_code(), 1);
        assert_eq!(SqliteType::Real.to_code(), 2);
        assert_eq!(SqliteType::Text.to_code(), 3);
        assert_eq!(SqliteType::Blob.to_code(), 4);
        assert_eq!(SqliteType::Null.to_code(), 5);
    }

    #[test]
    fn test_sqlite_type_names() {
        assert_eq!(SqliteType::Integer.name(), "INTEGER");
        assert_eq!(SqliteType::Real.name(), "REAL");
        assert_eq!(SqliteType::Text.name(), "TEXT");
        assert_eq!(SqliteType::Blob.name(), "BLOB");
        assert_eq!(SqliteType::Null.name(), "NULL");
        
        assert_eq!(format!("{}", SqliteType::Integer), "INTEGER");
    }

    #[test]
    fn test_column_info() {
        let column = SqliteColumnInfo::new(
            "test_col".to_string(),
            SqliteType::Integer,
            0,
            "INTEGER".to_string(),
        );
        
        assert_eq!(column.name, "test_col");
        assert_eq!(column.data_type, SqliteType::Integer);
        assert_eq!(column.index, 0);
        assert!(column.nullable);
        assert!(!column.primary_key);

        let column_with_metadata = SqliteColumnInfo::with_metadata(
            "id".to_string(),
            SqliteType::Integer,
            false,
            None,
            true,
            true,
            0,
            "INTEGER".to_string(),
        );
        
        assert!(!column_with_metadata.nullable);
        assert!(column_with_metadata.primary_key);
        assert!(column_with_metadata.auto_increment);
    }

    #[test]
    fn test_sqlite_stats() {
        let mut stats = SqliteStats::default();
        assert_eq!(stats.queries_executed, 0);
        assert_eq!(stats.average_query_time(), Duration::ZERO);
        assert_eq!(stats.utilization_ratio(), 0.0);

        stats.queries_executed = 10;
        stats.total_query_time = Duration::from_millis(100);
        assert_eq!(stats.average_query_time(), Duration::from_millis(10));

        stats.page_count = 100;
        stats.page_size = 4096;
        stats.database_size = 409600; // 100 pages
        assert_eq!(stats.utilization_ratio(), 1.0);
        
        stats.update();
        assert!(stats.last_updated > SystemTime::UNIX_EPOCH);
    }

    #[test]
    fn test_sql_value_to_sqlite_type() {
        use sqlite_utils::sql_value_to_sqlite_type;
        
        assert_eq!(sql_value_to_sqlite_type(&SqlValue::Null), SqliteType::Null);
        assert_eq!(sql_value_to_sqlite_type(&SqlValue::Boolean(true)), SqliteType::Integer);
        assert_eq!(sql_value_to_sqlite_type(&SqlValue::Integer(42)), SqliteType::Integer);
        assert_eq!(sql_value_to_sqlite_type(&SqlValue::Float(3.14)), SqliteType::Real);
        assert_eq!(sql_value_to_sqlite_type(&SqlValue::String("test".to_string())), SqliteType::Text);
        assert_eq!(sql_value_to_sqlite_type(&SqlValue::Bytes(Vec::from([1, 2, 3]))), SqliteType::Blob);
        assert_eq!(sql_value_to_sqlite_type(&SqlValue::Timestamp(SystemTime::now())), SqliteType::Text);
        assert_eq!(sql_value_to_sqlite_type(&SqlValue::Json(serde_json::Value::Null)), SqliteType::Text);
    }

    #[test]
    fn test_sanitize_connection_string() {
        use sqlite_utils::sanitize_connection_string;
        
        let conn_str = "file:test.db?mode=rw";
        assert_eq!(sanitize_connection_string(conn_str), conn_str);
        
        let conn_str_with_password = "file:test.db?password=secret123";
        let sanitized = sanitize_connection_string(conn_str_with_password);
        assert_eq!(sanitized, "file:test.db?password=***");
        
        let conn_str_with_password_and_params = "file:test.db?password=secret123;mode=rw";
        let sanitized = sanitize_connection_string(conn_str_with_password_and_params);
        assert_eq!(sanitized, "file:test.db?password=***;mode=rw");
    }

    #[test]
    fn test_validate_database_path() {
        use sqlite_utils::validate_database_path;
        
        assert!(validate_database_path(":memory:").is_ok());
        assert!(validate_database_path("test.db").is_ok());
        assert!(validate_database_path("/path/to/database.db").is_ok());
        
        assert!(validate_database_path("").is_err());
        assert!(validate_database_path("test\0.db").is_err());
    }

    #[test]
    fn test_global_config() {
        let config = SqliteConfig::default();
        assert!(set_global_config(config.clone()).is_ok());
        
        let retrieved = get_global_config().unwrap();
        assert_eq!(retrieved.page_size, config.page_size);
        assert_eq!(retrieved.cache_size, config.cache_size);
    }

    #[test]
    fn test_initialization_state() {
        // Note: These tests depend on initialization order
        // In a real test suite, we'd use proper setup/teardown
        println!("SQLite initialized: {}", is_sqlite_initialized());
    }
}
