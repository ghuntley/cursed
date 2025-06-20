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
// pub mod mysql;  // Temporarily disabled - mysql crate not available

// Re-export main types for easy access
pub use core::{
    DB, Conn, Stmt, Row, Rows, SlayRows, SlayResult,
    DBStats, TxOptions
};
pub use driver::{Driver, DriverConn, DriverStmt, DriverTx, DriverRegistry};
pub use pool::{ConnectionPool, PoolConfig, PoolStats};
pub use query::{QueryExecutor, QueryContext, QueryResult};
pub use transaction::{Tx, TransactionManager};
pub use migration::{Migration, Migrator, MigrationStatus};
pub use error::{DatabaseError, DatabaseErrorKind, SqlStateCode};
pub use builder::{
    QueryBuilder, SelectBuilder, InsertBuilder, UpdateBuilder, DeleteBuilder
};
pub use llvm_integration::{DatabaseLLVMIntegration, register_database_functions};

// Re-export ORM types for easy access
pub use orm::{
    OrmContext, Repository, Entity, EntityManager, FluentQueryBuilder,
    Migration as OrmMigration, MigrationManager, Relationship, RelationshipManager,
    QueryCache, EntityCache, ValidationError, Validator, TransactionalRepository,
    SchemaBuilder, TypeMapper, ResultMapper
};

// Re-export SQLite driver
pub use sqlite::{
    SqliteDriver, SqliteConfig, SqliteConnectionString, SqliteError, SqliteResult,
    SqliteVersion, SqliteFeatures, SqliteUtils, init_sqlite, register_sqlite_driver
};

// Re-export PostgreSQL driver
pub use postgres::{
    PostgresDriver, PostgresConfig, PostgresConnectionString, PostgresError, 
    PostgresPool, PostgresPoolConfig, SslMode, init_postgres, new_postgres_driver,
    parse_connection_string
};

// Re-export MySQL driver
pub use mysql::{
    MySqlDriver, MySqlConfig, MySqlConnection, MySqlStatement, MySqlTransaction,
    MySqlError, MySqlResult, MySqlPool, MySqlPoolConfig, init_mysql, 
    new_mysql_driver, new_mysql_driver_with_config, create_mysql_driver, parse_mysql_dsn
};

/// fr fr Database isolation levels for transaction control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlIsolationLevel {
    /// Default isolation level for the database
    LevelDefault = 0,
    /// Read uncommitted data (lowest isolation)
    LevelReadUncommitted = 1,
    /// Read committed data only
    LevelReadCommitted = 2,
    /// Write committed (similar to read committed)
    LevelWriteCommitted = 3,
    /// Repeatable reads within transaction
    LevelRepeatableRead = 4,
    /// Snapshot isolation for consistent reads
    LevelSnapshot = 5,
    /// Serializable transactions (highest isolation)
    LevelSerializable = 6,
    /// Linearizable transactions (strictest consistency)
    LevelLinearizable = 7,
}

impl std::fmt::Display for SqlIsolationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlIsolationLevel::LevelDefault => write!(f, "DEFAULT"),
            SqlIsolationLevel::LevelReadUncommitted => write!(f, "READ_UNCOMMITTED"),
            SqlIsolationLevel::LevelReadCommitted => write!(f, "READ_COMMITTED"),
            SqlIsolationLevel::LevelWriteCommitted => write!(f, "WRITE_COMMITTED"),
            SqlIsolationLevel::LevelRepeatableRead => write!(f, "REPEATABLE_READ"),
            SqlIsolationLevel::LevelSnapshot => write!(f, "SNAPSHOT"),
            SqlIsolationLevel::LevelSerializable => write!(f, "SERIALIZABLE"),
            SqlIsolationLevel::LevelLinearizable => write!(f, "LINEARIZABLE"),
        }
    }
}

/// fr fr SQL data types supported by the system
#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    /// NULL value
    Null,
    /// Boolean value (lit in CURSED)
    Boolean(bool),
    /// Integer value (normie in CURSED)
    Integer(i64),
    /// Floating point value
    Float(f64),
    /// String value (tea in CURSED)
    String(String),
    /// Binary data
    Bytes(Vec<u8>),
    /// Timestamp value
    Timestamp(std::time::SystemTime),
    /// JSON value for enhanced database support
    Json(serde_json::Value),
}

impl Eq for SqlValue {}

impl std::hash::Hash for SqlValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            SqlValue::Null => {},
            SqlValue::Boolean(b) => b.hash(state),
            SqlValue::Integer(i) => i.hash(state),
            SqlValue::Float(f) => {
                // Handle NaN and infinity cases for hashing
                if f.is_nan() {
                    0u64.hash(state);
                } else if f.is_infinite() {
                    if f.is_sign_positive() {
                        1u64.hash(state);
                    } else {
                        2u64.hash(state);
                    }
                } else {
                    // Use integer representation for finite numbers
                    f.to_bits().hash(state);
                }
            },
            SqlValue::String(s) => s.hash(state),
            SqlValue::Bytes(b) => b.hash(state),
            SqlValue::Timestamp(t) => t.hash(state),
            SqlValue::Json(j) => {
                // Hash JSON as string representation
                format!("{}", j).hash(state);
            },
        }
    }
}

impl std::fmt::Display for SqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlValue::Null => write!(f, "NULL"),
            SqlValue::Boolean(b) => write!(f, "{}", b),
            SqlValue::Integer(i) => write!(f, "{}", i),
            SqlValue::Float(fl) => write!(f, "{}", fl),
            SqlValue::String(s) => write!(f, "'{}'", s),
            SqlValue::Bytes(b) => write!(f, "BLOB({} bytes)", b.len()),
            SqlValue::Timestamp(t) => write!(f, "{:?}", t),
            SqlValue::Json(j) => write!(f, "{}", j),
        }
    }
}

/// fr fr Configuration for the SQLSlay database system
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Maximum number of open connections in pool
    pub max_open_connections: usize,
    /// Maximum number of idle connections to maintain
    pub max_idle_connections: usize,
    /// Maximum lifetime of a connection
    pub connection_max_lifetime_seconds: u64,
    /// Maximum idle time for a connection
    pub connection_max_idle_seconds: u64,
    /// Connection timeout when acquiring from pool
    pub connection_timeout_seconds: u64,
    /// Query execution timeout
    pub query_timeout_seconds: u64,
    /// Enable connection pool monitoring
    pub enable_pool_monitoring: bool,
    /// Enable query logging for debugging
    pub enable_query_logging: bool,
    /// Maximum number of retries for failed operations
    pub max_retry_attempts: usize,
    /// Retry delay in milliseconds
    pub retry_delay_ms: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            max_open_connections: 100,
            max_idle_connections: 10,
            connection_max_lifetime_seconds: 3600, // 1 hour
            connection_max_idle_seconds: 600,      // 10 minutes
            connection_timeout_seconds: 30,
            query_timeout_seconds: 300,            // 5 minutes
            enable_pool_monitoring: true,
            enable_query_logging: false,
            max_retry_attempts: 3,
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
    pub data: std::collections::HashMap<String, String>,
}

impl Default for VibeContext {
    fn default() -> Self {
        Self {
            timeout: None,
            cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            data: std::collections::HashMap::new(),
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
