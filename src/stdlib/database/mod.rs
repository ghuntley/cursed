use crate::error::CursedError;
/// SQLSlay - CURSED Database Connectivity System
/// 
/// A high-performance SQL database connectivity system with connection pooling,
/// query builders, and elegant error handling that slays database operations.
/// 
/// Features:
/// - Multi-database support (MySQL, PostgreSQL, SQLite, SQL Server)
/// - Intelligent connection pooling with configurable limits
/// - Enhanced query building and execution with Gen Z syntax
/// - Transaction management with proper isolation levels
/// - Context-aware operations with cancellation support
/// - Built-in migrations and schema management
/// - Real-time connection pool monitoring and statistics
/// - Type-safe struct mapping and JSON serialization

pub mod core;
pub mod driver;
pub mod pool;
pub mod query;
pub mod transaction;
pub mod migration;
pub mod error;
pub mod builder;
pub mod llvm_integration;
pub mod orm;

// Database-specific drivers
pub mod sqlite;
pub mod postgres;
pub mod redis;
// pub mod mysql;  // Temporarily disabled - mysql crate not available

// Re-export main types for easy access
pub use core::{
    SqlValue
};
pub use driver::{Driver, DriverConn, DriverStmt, DriverTx, DriverRegistry};
pub use pool::{ConnectionPool, PoolConfig, PoolStats};
pub use llvm_integration::{
    register_database_functions
};
pub use query::{QueryExecutor, QueryContext, QueryResult};
pub use transaction::{Tx, TransactionManager};
pub use migration::{Migration, Migrator, MigrationStatus};
pub use error::{DatabaseError, DatabaseErrorKind};
pub use builder::{
    QueryBuilder, SelectBuilder, InsertBuilder, UpdateBuilder, DeleteBuilder
};

// Re-export ORM types for easy access
pub use orm::{
    SchemaBuilder, TypeMapper, ResultMapper
};

// Re-export SQLite driver
pub use sqlite::{
    SqliteVersion, SqliteFeatures, SqliteUtils, init_sqlite, register_sqlite_driver
};

// Re-export PostgreSQL driver
// pub use postgres::{};

/// Transaction isolation level (alias for compatibility)
pub type IsolationLevel = SqlIsolationLevel;

/// fr fr Database isolation levels for transaction control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlIsolationLevel {
    /// Default isolation level for the database
    Default,
    /// Read uncommitted data (lowest isolation)
    ReadUncommitted,
    /// Read committed data only
    ReadCommitted,
    /// Write committed (similar to read committed)
    WriteCommitted,
    /// Repeatable reads within transaction
    RepeatableRead,
    /// Snapshot isolation for consistent reads
    Snapshot,
    /// Serializable transactions (highest isolation)
    Serializable,
    /// Linearizable transactions (strictest consistency)
    Linearizable,
}

impl std::fmt::Display for SqlIsolationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlIsolationLevel::Default => write!(f, "DEFAULT"),
            SqlIsolationLevel::ReadUncommitted => write!(f, "READ UNCOMMITTED"),
            SqlIsolationLevel::ReadCommitted => write!(f, "READ COMMITTED"),
            SqlIsolationLevel::WriteCommitted => write!(f, "WRITE COMMITTED"),
            SqlIsolationLevel::RepeatableRead => write!(f, "REPEATABLE READ"),
            SqlIsolationLevel::Snapshot => write!(f, "SNAPSHOT"),
            SqlIsolationLevel::Serializable => write!(f, "SERIALIZABLE"),
            SqlIsolationLevel::Linearizable => write!(f, "LINEARIZABLE"),
        }
    }
}



/// fr fr Configuration for the SQLSlay database system
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Maximum number of open connections in pool
    pub max_connections: u32,
    /// Maximum number of idle connections to maintain
    pub max_idle_connections: u32,
    /// Maximum lifetime of a connection
    pub connection_max_lifetime_seconds: u64,
    /// Maximum idle time for a connection
    pub connection_max_idle_seconds: u64,
    /// Connection timeout when acquiring from pool
    pub connection_timeout_seconds: u64,
    /// Query execution timeout
    pub query_timeout_seconds: u64,
    /// Enable connection pool monitoring
    pub enable_monitoring: bool,
    /// Enable query logging for debugging
    pub enable_query_logging: bool,
    /// Maximum number of retries for failed operations
    pub max_retries: u32,
    /// Retry delay in milliseconds
    pub retry_delay_ms: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            max_connections: 50,
            max_idle_connections: 10,
            connection_max_lifetime_seconds: 3600, // 1 hour
            connection_max_idle_seconds: 600,      // 10 minutes
            connection_timeout_seconds: 30,        // 30 seconds
            query_timeout_seconds: 300,            // 5 minutes
            enable_monitoring: true,
            enable_query_logging: false,
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }
}

/// fr fr Context for database operations with timeout and cancellation support
#[derive(Debug, Clone)]
pub struct VibeContext {
    /// Operation timeout
    pub timeout: Option<std::time::Duration>,
    /// Cancellation token for early termination
    pub cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    /// Additional context data
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for VibeContext {
    fn default() -> Self {
        Self {
            timeout: None,
            cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl VibeContext {
    /// slay Create a new context with timeout
    pub fn with_timeout(timeout: std::time::Duration) -> Self {
        Self {
            timeout: Some(timeout),
            ..Default::default()
        }
    }

    /// slay Check if the context has been cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// slay Cancel the context
    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
