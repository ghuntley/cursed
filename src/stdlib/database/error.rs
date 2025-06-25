/// fr fr Database error types and handling for SQLSlay
/// 
/// This module provides comprehensive error handling for database operations,
/// including SQL state codes, error categorization, and integration with
/// the CURSED error system.
/// 
/// Why comprehensive database error handling is critical:
/// - Database operations involve external systems that can fail in many ways
/// - Network failures, constraint violations, and resource exhaustion must be handled
/// - SQL errors need to be mapped to meaningful application errors
/// - CursedError context helps developers debug database issues quickly
/// - Proper error categorization enables appropriate retry and recovery strategies

use std::fmt::{self, Display};
use crate::error::CursedError;

/// fr fr Categories of database errors for proper handling
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseErrorKind {
    /// Connection-related errors (network, authentication, etc.)
    /// Query syntax or execution errors
    /// Transaction-related errors (deadlock, isolation violations, etc.)
    /// Constraint violations (foreign key, unique, check, etc.)
    /// Data type conversion or scanning errors
    /// Type conversion errors
    /// Serialization/deserialization errors
    /// Connection pool related errors
    /// Driver-specific errors
    /// Timeout errors
    /// No rows found when expecting data
    /// No last insert ID available
    /// Resource exhaustion (too many connections, memory, etc.)
    /// Configuration errors
    /// Migration errors
    /// Unknown or unclassified errors
    /// Feature not implemented
    /// Request timeout
    /// Constraint violation (specific to constraints)
    /// Authentication or authorization errors
    /// Resource errors (memory, disk space, etc.)
    /// Internal database system errors
    /// SQL syntax errors
    /// Data integrity errors
    /// Resource exhausted errors
    /// Type mismatch errors
    /// Schema-related errors
    /// General SQL errors
// impl Display for DatabaseErrorKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             DatabaseErrorKind::ConnectionError => write!(f, "ConnectionError"),
//             DatabaseErrorKind::QueryError => write!(f, "QueryError"),
//             DatabaseErrorKind::TransactionError => write!(f, "TransactionError"),
//             DatabaseErrorKind::ConstraintError => write!(f, "ConstraintError"),
//             DatabaseErrorKind::ScanError => write!(f, "ScanError"),
//             DatabaseErrorKind::SerializationError => write!(f, "SerializationError"),
//             DatabaseErrorKind::PoolError => write!(f, "PoolError"),
//             DatabaseErrorKind::DriverError => write!(f, "DriverError"),
//             DatabaseErrorKind::TimeoutError => write!(f, "TimeoutError"),
//             DatabaseErrorKind::NoRows => write!(f, "NoRows"),
//             DatabaseErrorKind::NoLastInsertId => write!(f, "NoLastInsertId"),
//             DatabaseErrorKind::ResourceExhaustion => write!(f, "ResourceExhaustion"),
//             DatabaseErrorKind::ConfigurationError => write!(f, "ConfigurationError"),
//             DatabaseErrorKind::MigrationError => write!(f, "MigrationError"),
//             DatabaseErrorKind::Unknown => write!(f, "Unknown"),
//             DatabaseErrorKind::NotImplemented => write!(f, "NotImplemented"),
//             DatabaseErrorKind::Timeout => write!(f, "Timeout"),
//             DatabaseErrorKind::ConstraintViolation => write!(f, "ConstraintViolation"),
//             DatabaseErrorKind::AuthenticationError => write!(f, "AuthenticationError"),
//             DatabaseErrorKind::ResourceError => write!(f, "ResourceError"),
//             DatabaseErrorKind::InternalError => write!(f, "InternalError"),
//             DatabaseErrorKind::SyntaxError => write!(f, "SyntaxError"),
//             DatabaseErrorKind::DataIntegrityError => write!(f, "DataIntegrityError"),
//             DatabaseErrorKind::ResourceExhausted => write!(f, "ResourceExhausted"),
//             DatabaseErrorKind::TypeMismatch => write!(f, "TypeMismatch"),
//             DatabaseErrorKind::SchemaError => write!(f, "SchemaError"),
//             DatabaseErrorKind::SqlError => write!(f, "SqlError"),
//             DatabaseErrorKind::ConversionError => write!(f, "ConversionError"),
//         }
//     }
// }

/// fr fr SQL state codes for standard error classification
/// These follow the SQL standard SQLSTATE codes for interoperability
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlStateCode {
    /// 00000 - Successful completion
    /// 01xxx - Warning
    /// 02000 - No data
    /// 07xxx - Dynamic SQL error
    /// 08xxx - Connection exception
    /// 09xxx - Triggered action exception
    /// 0A xxx - Feature not supported
    /// 21xxx - Cardinality violation
    /// 22xxx - Data exception
    /// 23xxx - Integrity constraint violation
    /// 24xxx - Invalid cursor state
    /// 25xxx - Invalid transaction state
    /// 26xxx - Invalid SQL statement name
    /// 27xxx - Triggered data change violation
    /// 28xxx - Invalid authorization specification
    /// 2B xxx - Dependent privilege descriptors still exist
    /// 2D xxx - Invalid transaction termination
    /// 2E xxx - Invalid connection name
    /// 34xxx - Invalid cursor name
    /// 35xxx - Invalid condition number
    /// 3C xxx - Ambiguous cursor name
    /// 3D xxx - Invalid catalog name
    /// 3F xxx - Invalid schema name
    /// 40xxx - Transaction rollback
    /// 42xxx - Syntax error or access rule violation
    /// 44xxx - WITH CHECK OPTION violation
    /// HY xxx - CLI-specific condition
    /// Custom state codes for driver-specific errors
impl Display for SqlStateCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// fr fr Comprehensive database error with rich context
#[derive(Debug)]
pub struct DatabaseError {
    /// fr fr Category of this error
    /// fr fr Human-readable error message
    /// fr fr SQL state code if applicable
    /// fr fr Vendor-specific error code
    /// fr fr Source location where error occurred
    /// fr fr Query that caused the error
    /// fr fr Additional context information
    /// fr fr Underlying cause error  
impl Clone for DatabaseError {
    fn clone(&self) -> Self {
        Self {
            cause: None, // Cannot clone trait objects, so we'll lose the cause
        }
    }
impl DatabaseError {
    /// slay Create a new database error
    pub fn new(kind: DatabaseErrorKind, message: &str) -> Self {
        Self {
        }
    }

    /// slay Create a new database error with SQL state
    pub fn with_sql_state(kind: DatabaseErrorKind, message: &str, sql_state: SqlStateCode) -> Self {
        Self {
        }
    }

    /// slay Create a new database error with vendor code
    pub fn with_vendor_code(kind: DatabaseErrorKind, message: &str, vendor_code: i32) -> Self {
        Self {
        }
    }

    /// slay Create a new database error with full context
    pub fn with_context(
    ) -> Self {
        Self {
        }
    }

    /// slay Add source location to this error
    pub fn with_location(mut self, location: &str) -> Self {
        self.source_location = Some(location.to_string());
        self
    /// slay Add query context to this error
    pub fn with_query(mut self, query: &str) -> Self {
        self.query = Some(query.to_string());
        self
    /// slay Add additional context information
    pub fn with_context_data(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    /// slay Add underlying cause
    pub fn with_cause(mut self, cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
        self.cause = Some(cause);
        self
    /// slay Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        match self.kind {
            DatabaseErrorKind::TransactionError => {
                // Check if it's a deadlock or similar retryable transaction error
                if let Some(ref sql_state) = self.sql_state {
                    match sql_state {
                    }
                } else {
                    false
                }
            }
        }
    }

    /// slay Check if this error indicates data was not found
    pub fn is_not_found(&self) -> bool {
        matches!(self.kind, DatabaseErrorKind::NoRows) ||
        matches!(self.sql_state, Some(SqlStateCode::NoData))
    /// slay Check if this error is a constraint violation
    pub fn is_constraint_violation(&self) -> bool {
        matches!(self.kind, DatabaseErrorKind::ConstraintError) ||
        matches!(self.kind, DatabaseErrorKind::ConstraintViolation) ||
        matches!(self.sql_state, Some(SqlStateCode::IntegrityConstraintViolation(_)))
    /// slay Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self.kind {
        }
    }

    /// slay Create a connection error
    pub fn Connection(message: String) -> Self {
        Self::new(DatabaseErrorKind::ConnectionError, &message)
    /// slay Create a query error
    pub fn Query(message: String) -> Self {
        Self::new(DatabaseErrorKind::QueryError, &message)
    /// slay Create a transaction error
    pub fn Transaction(message: String) -> Self {
        Self::new(DatabaseErrorKind::TransactionError, &message)
    /// slay Create a general error
    pub fn General(message: String) -> Self {
        Self::new(DatabaseErrorKind::Unknown, &message)
    /// slay Create a configuration error
    pub fn Configuration(message: String) -> Self {
        Self::new(DatabaseErrorKind::ConfigurationError, &message)
    }
}

// impl Display for DatabaseError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "DatabaseError({}): {}", self.kind, self.message)?;
//         
//         if let Some(ref sql_state) = self.sql_state {
//             write!(f, " [SQL State: {}]", sql_state)?;
//         }
//         
//         if let Some(vendor_code) = self.vendor_code {
//             write!(f, " [Vendor Code: {}]", vendor_code)?;
//         }
//         
//         if let Some(ref location) = self.source_location {
//             write!(f, " [Location: {}]", location)?;
//         }
//         
//         if let Some(ref query) = self.query {
//             write!(f, " [Query: {}]", query)?;
//         }
//         
//         if !self.context.is_empty() {
//             write!(f, " [Context: {:?}]", self.context)?;
//         }
//         
//         Ok(())
//     }
// }

// impl std::error::CursedError for DatabaseError {
//     fn source(&self) -> Option<&(dyn std::error::CursedError + 'static)> {
//     }
// }

/// fr fr Convert DatabaseError to CursedError for integration with the language error system
// COMMENTED OUT: Conflicts with implementation in error/mod.rs
// impl From<DatabaseError> for CursedError {
//     fn from(err: DatabaseError) -> Self {
//         let error_code = match err.kind {
//             DatabaseErrorKind::ConnectionError => "DB_CONNECTION_ERROR",
//             DatabaseErrorKind::QueryError => "DB_QUERY_ERROR",
//             DatabaseErrorKind::TransactionError => "DB_TRANSACTION_ERROR",
//             DatabaseErrorKind::ConstraintError => "DB_CONSTRAINT_ERROR",
//             DatabaseErrorKind::ScanError => "DB_SCAN_ERROR",
//             DatabaseErrorKind::SerializationError => "DB_SERIALIZATION_ERROR",
//             DatabaseErrorKind::PoolError => "DB_POOL_ERROR",
//             DatabaseErrorKind::DriverError => "DB_DRIVER_ERROR",
//             DatabaseErrorKind::TimeoutError => "DB_TIMEOUT_ERROR",
//             DatabaseErrorKind::NoRows => "DB_NO_ROWS",
//             DatabaseErrorKind::NoLastInsertId => "DB_NO_LAST_INSERT_ID",
//             DatabaseErrorKind::ResourceExhaustion => "DB_RESOURCE_EXHAUSTION",
//             DatabaseErrorKind::ConfigurationError => "DB_CONFIGURATION_ERROR",
//             DatabaseErrorKind::MigrationError => "DB_MIGRATION_ERROR",
//             DatabaseErrorKind::Unknown => "DB_UNKNOWN_ERROR",
//             DatabaseErrorKind::NotImplemented => "DB_NOT_IMPLEMENTED",
//             DatabaseErrorKind::Timeout => "DB_TIMEOUT",
//             DatabaseErrorKind::ConstraintViolation => "DB_CONSTRAINT_VIOLATION",
//             DatabaseErrorKind::AuthenticationError => "DB_AUTHENTICATION_ERROR",
//             DatabaseErrorKind::ResourceError => "DB_RESOURCE_ERROR",
//             DatabaseErrorKind::InternalError => "DB_INTERNAL_ERROR",
//             DatabaseErrorKind::SyntaxError => "DB_SYNTAX_ERROR",
//             DatabaseErrorKind::DataIntegrityError => "DB_DATA_INTEGRITY_ERROR",
//             DatabaseErrorKind::ResourceExhausted => "DB_RESOURCE_EXHAUSTED",
//             DatabaseErrorKind::TypeMismatch => "DB_TYPE_MISMATCH",
//             DatabaseErrorKind::SchemaError => "DB_SCHEMA_ERROR",
//             DatabaseErrorKind::SqlError => "DB_SQL_ERROR",
//             DatabaseErrorKind::ConversionError => "DB_CONVERSION_ERROR",
//         };
//
//         CursedError::Repl(format!("{}: {}", error_code, err.message))
//     }
// }

/// fr fr CursedError severity levels for proper logging and handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Informational - operation completed but with notable result
    /// Warning - operation completed but with issues
    /// CursedError - operation failed but system can continue
    /// Critical - operation failed and system integrity may be compromised
// impl Display for ErrorSeverity {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ErrorSeverity::Info => write!(f, "INFO"),
//             ErrorSeverity::Warning => write!(f, "WARNING"),
//             ErrorSeverity::CursedError => write!(f, "ERROR"),
//             ErrorSeverity::Critical => write!(f, "CRITICAL"),
//         }
//     }
// }

/// fr fr Helper functions for creating common database errors
impl DatabaseError {
    /// slay Create a connection error
    pub fn connection_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::ConnectionError, message)
    /// slay Create a query error
    pub fn query_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::QueryError, message)
    /// slay Create a transaction error
    pub fn transaction_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::TransactionError, message)
    /// slay Create a constraint violation error
    pub fn constraint_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::ConstraintError, message)
    /// slay Create a timeout error
    pub fn timeout_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::TimeoutError, message)
    /// slay Create a pool error
    pub fn pool_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::PoolError, message)
    /// slay Create a driver error
    pub fn driver_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::DriverError, message)
    /// slay Create a serialization error
    pub fn serialization_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::SerializationError, message)
    /// slay Create a validation error
    pub fn validation_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::ConstraintError, message)
    /// slay Create an internal error
    pub fn internal_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::InternalError, message)
    /// slay Create a not found error
    pub fn not_found(message: &str) -> Self {
        Self::new(DatabaseErrorKind::NoRows, message)
    /// slay Create a not implemented error
    pub fn not_implemented(message: &str) -> Self {
        Self::new(DatabaseErrorKind::NotImplemented, message)
    /// slay Create a constraint violation error
    pub fn constraint_violation(message: &str) -> Self {
        Self::new(DatabaseErrorKind::ConstraintViolation, message)
    /// slay Create an authentication error
    pub fn authentication_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::AuthenticationError, message)
    /// slay Create a syntax error
    pub fn syntax_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::SyntaxError, message)
    /// slay Create a resource error
    pub fn resource_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::ResourceError, message)
    }
}

/// fr fr Result type alias for database operations
pub type DatabaseResult<T> = std::result::Result<T, DatabaseError>;

/// fr fr Helper macro for creating database errors with location
#[macro_export]
macro_rules! db_error {
    ($kind:expr, $msg:expr) => {
        DatabaseError::new($kind, $msg).with_location(&format!("{}:{}", file!(), line!()))
    ($kind:expr, $msg:expr, $($arg:tt)*) => {
        DatabaseError::new($kind, &format!($msg, $($arg)*)).with_location(&format!("{}:{}", file!(), line!()))
/// fr fr Helper macro for returning database errors
#[macro_export]
macro_rules! db_bail {
    ($kind:expr, $msg:expr) => {
        return Err(db_error!($kind, $msg))
    ($kind:expr, $msg:expr, $($arg:tt)*) => {
        return Err(db_error!($kind, $msg, $($arg)*))
}
