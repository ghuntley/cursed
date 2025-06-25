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
    DBStats, TxOptions
// };
pub use driver::{Driver, DriverConn, DriverStmt, DriverTx, DriverRegistry};
pub use pool::{ConnectionPool, PoolConfig, PoolStats};
pub use llvm_integration::{
    register_database_functions
// };
pub use query::{QueryExecutor, QueryContext, QueryResult};
pub use transaction::{Tx, TransactionManager};
pub use migration::{Migration, Migrator, MigrationStatus};
pub use error::{DatabaseError, DatabaseErrorKind, SqlStateCode};
pub use builder::{
    QueryBuilder, SelectBuilder, InsertBuilder, UpdateBuilder, DeleteBuilder
// };

// Re-export ORM types for easy access
pub use orm::{
    SchemaBuilder, TypeMapper, ResultMapper
// };

// Re-export SQLite driver
pub use sqlite::{
    SqliteVersion, SqliteFeatures, SqliteUtils, init_sqlite, register_sqlite_driver
// };

// Re-export PostgreSQL driver
pub use postgres::{
    parse_connection_string
// };

// Re-export MySQL driver from db_sql package
// pub use crate::stdlib::packages::db_sql::mysql::{
//     MySqlDriver, MySqlConnection, MySqlPreparedStatement as MySqlStatement, 
//     MySqlTransactionImpl as MySqlTransaction, MySqlError, MySqlResultSet
// };

/// Transaction isolation level (alias for compatibility)
pub type IsolationLevel = SqlIsolationLevel;

/// fr fr Database isolation levels for transaction control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlIsolationLevel {
    /// Default isolation level for the database
    /// Read uncommitted data (lowest isolation)
    /// Read committed data only
    /// Write committed (similar to read committed)
    /// Repeatable reads within transaction
    /// Snapshot isolation for consistent reads
    /// Serializable transactions (highest isolation)
    /// Linearizable transactions (strictest consistency)
impl std::fmt::Display for SqlIsolationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
/// fr fr SQL data types supported by the system
#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    /// NULL value
    /// Boolean value (lit in CURSED)
    /// Integer value (normie in CURSED)
    /// Floating point value
    /// String value (tea in CURSED)
    /// Binary data
    /// Timestamp value
    /// JSON value for enhanced database support
impl std::hash::Hash for SqlValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
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
            SqlValue::Json(j) => {
                // Hash JSON as string representation
                format!("{}", j).hash(state);
        }
    }
impl std::fmt::Display for SqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
/// fr fr Configuration for the SQLSlay database system
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Maximum number of open connections in pool
    /// Maximum number of idle connections to maintain
    /// Maximum lifetime of a connection
    /// Maximum idle time for a connection
    /// Connection timeout when acquiring from pool
    /// Query execution timeout
    /// Enable connection pool monitoring
    /// Enable query logging for debugging
    /// Maximum number of retries for failed operations
    /// Retry delay in milliseconds
impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            connection_max_lifetime_seconds: 3600, // 1 hour
            connection_max_idle_seconds: 600,      // 10 minutes
            query_timeout_seconds: 300,            // 5 minutes
        }
    }
/// fr fr Context for database operations with timeout and cancellation support
#[derive(Debug, Clone)]
pub struct VibeContext {
    /// Operation timeout
    /// Cancellation token for early termination
    /// Additional context data
impl Default for VibeContext {
    fn default() -> Self {
        Self {
        }
    }
impl VibeContext {
    /// slay Create a new context with timeout
    pub fn with_timeout(timeout: std::time::Duration) -> Self {
        Self {
            ..Default::default()
        }
    }

    /// slay Check if the context has been cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Relaxed)
    /// slay Cancel the context
    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
