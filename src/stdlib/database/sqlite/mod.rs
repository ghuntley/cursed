/// SQLite Driver for CURSED Database System
/// 
/// High-performance SQLite driver with connection pooling, prepared statements,
/// transaction management, and advanced SQLite-specific features.
/// 
/// Features:
/// - Full SQLite 3.x support with modern API
/// - Connection pooling with intelligent lifecycle management
/// - Prepared statement caching for optimal performance
/// - Transaction support with savepoints and rollback handling
/// - SQLite-specific optimizations and pragma management
/// - WAL mode support for concurrent access
/// - FTS (Full-Text Search) integration
/// - JSON1 extension support for JSON operations
/// - Backup and restore functionality
/// - Performance monitoring and statistics

pub mod connection;
pub mod statement;
pub mod transaction;
pub mod error;
pub mod utils;
pub mod features;

// Re-export main types
pub use connection::{SqliteConnection, SqliteConnectionBuilder};
pub use statement::{SqliteStatement, SqliteRow, SqliteResultSet};
pub use transaction::{SqliteTransaction, SqliteTransactionBuilder};
pub use error::{SqliteError, SqliteErrorKind};
pub use utils::{SqliteUtils, SqliteBackup};
pub use features::{SqliteFeatures, SqliteVersion};

use super::{DatabaseError, DatabaseErrorKind, SqlValue, Driver, DriverConn};
use crate::error::CursedError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// fr fr SQLite data types supported by the driver
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteType {
    /// NULL type
    Null,
    /// INTEGER type (64-bit signed)
    Integer,
    /// REAL type (64-bit float)
    Real,
    /// TEXT type (UTF-8 string)
    Text,
    /// BLOB type (binary data)
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
            SqliteType::Null => 0,
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

/// fr fr SQLite column information
#[derive(Debug, Clone)]
pub struct SqliteColumnInfo {
    /// Column index
    pub index: i32,
    /// Column name
    pub name: String,
    /// Column type
    pub column_type: SqliteType,
    /// Is nullable
    pub nullable: bool,
}

impl SqliteColumnInfo {
    /// slay Create new column info
    pub fn new(index: i32, name: String, column_type: SqliteType, nullable: bool) -> Self {
        Self {
            index,
            name,
            column_type,
            nullable,
        }
    }
}

/// fr fr SQLite connection statistics
#[derive(Debug, Clone)]
pub struct SqliteStats {
    /// Total queries executed
    pub total_queries: u64,
    /// Total preparation time (microseconds)
    pub total_prepare_time_us: u64,
    /// Total execution time (microseconds)
    pub total_execute_time_us: u64,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,
    /// Number of active connections
    pub active_connections: u32,
    /// Database size in bytes
    pub database_size_bytes: u64,
    /// Page cache size
    pub page_cache_size: u32,
}

impl Default for SqliteStats {
    fn default() -> Self {
        Self {
            total_queries: 0,
            total_prepare_time_us: 0,
            total_execute_time_us: 0,
            cache_hit_rate: 0.0,
            active_connections: 0,
            database_size_bytes: 0,
            page_cache_size: 0,
        }
    }
}

/// fr fr SQLite configuration options
#[derive(Debug, Clone)]
pub struct SqliteConfig {
    /// Database file path
    pub database_path: String,
    /// Enable WAL mode
    pub enable_wal: bool,
    /// Cache size in pages
    pub cache_size: i32,
    /// Synchronous mode
    pub synchronous: SqliteSynchronous,
    /// Journal mode
    pub journal_mode: SqliteJournalMode,
    /// Connection timeout in seconds
    pub timeout_seconds: u32,
    /// Enable foreign keys
    pub foreign_keys: bool,
    /// Enable triggers
    pub triggers: bool,
}

/// fr fr SQLite synchronous modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteSynchronous {
    /// No fsync calls
    Off,
    /// Fsync only at critical moments
    Normal,
    /// Extra fsync calls for maximum durability
    Full,
    /// Like FULL but with additional synchronization
    Extra,
}

/// fr fr SQLite journal modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteJournalMode {
    /// Traditional rollback journal
    Delete,
    /// Truncate journal instead of deleting
    Truncate,
    /// Persistent journal file
    Persist,
    /// Write-Ahead Logging
    Wal,
    /// In-memory journal
    Memory,
    /// No journal
    Off,
}

impl Default for SqliteConfig {
    fn default() -> Self {
        Self {
            database_path: ":memory:".to_string(),
            enable_wal: true,
            cache_size: -2000, // 2MB cache
            synchronous: SqliteSynchronous::Normal,
            journal_mode: SqliteJournalMode::Wal,
            timeout_seconds: 30,
            foreign_keys: true,
            triggers: true,
        }
    }
}



/// fr fr Initialize SQLite driver
pub fn init_sqlite() -> Result<(), CursedError> {
    println!("🗃️  SQLite driver initialized");
    Ok(())
}

/// fr fr Register SQLite driver with the driver registry
pub fn register_sqlite_driver() -> Result<(), DatabaseError> {
    // TODO: Implement driver registration
    println!("📝 SQLite driver registered");
    Ok(())
}
