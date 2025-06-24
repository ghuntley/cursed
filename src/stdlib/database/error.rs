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
/// - Error context helps developers debug database issues quickly
/// - Proper error categorization enables appropriate retry and recovery strategies

use std::fmt::{self, Display};
use crate::error::CursedError;
use crate::error::Error;

/// fr fr Categories of database errors for proper handling
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseErrorKind {
    /// Connection-related errors (network, authentication, etc.)
    ConnectionError,
    /// Query syntax or execution errors
    QueryError,
    /// Transaction-related errors (deadlock, isolation violations, etc.)
    TransactionError,
    /// Constraint violations (foreign key, unique, check, etc.)
    ConstraintError,
    /// Data type conversion or scanning errors
    ScanError,
    /// Type conversion errors
    ConversionError,
    /// Serialization/deserialization errors
    SerializationError,
    /// Connection pool related errors
    PoolError,
    /// Driver-specific errors
    DriverError,
    /// Timeout errors
    TimeoutError,
    /// No rows found when expecting data
    NoRows,
    /// No last insert ID available
    NoLastInsertId,
    /// Resource exhaustion (too many connections, memory, etc.)
    ResourceExhaustion,
    /// Configuration errors
    ConfigurationError,
    /// Migration errors
    MigrationError,
    /// Unknown or unclassified errors
    Unknown,
    /// Feature not implemented
    NotImplemented,
    /// Request timeout
    Timeout,
    /// Constraint violation (specific to constraints)
    ConstraintViolation,
    /// Authentication or authorization errors
    AuthenticationError,
    /// Resource errors (memory, disk space, etc.)
    ResourceError,
    /// Internal database system errors
    InternalError,
    /// SQL syntax errors
    SyntaxError,
    /// Data integrity errors
    DataIntegrityError,
    /// Resource exhausted errors
    ResourceExhausted,
    /// Type mismatch errors
    TypeMismatch,
    /// Schema-related errors
    SchemaError,
    /// General SQL errors
    SqlError,
}

impl Display for DatabaseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseErrorKind::ConnectionError => write!(f, "ConnectionError"),
            DatabaseErrorKind::QueryError => write!(f, "QueryError"),
            DatabaseErrorKind::TransactionError => write!(f, "TransactionError"),
            DatabaseErrorKind::ConstraintError => write!(f, "ConstraintError"),
            DatabaseErrorKind::ScanError => write!(f, "ScanError"),
            DatabaseErrorKind::SerializationError => write!(f, "SerializationError"),
            DatabaseErrorKind::PoolError => write!(f, "PoolError"),
            DatabaseErrorKind::DriverError => write!(f, "DriverError"),
            DatabaseErrorKind::TimeoutError => write!(f, "TimeoutError"),
            DatabaseErrorKind::NoRows => write!(f, "NoRows"),
            DatabaseErrorKind::NoLastInsertId => write!(f, "NoLastInsertId"),
            DatabaseErrorKind::ResourceExhaustion => write!(f, "ResourceExhaustion"),
            DatabaseErrorKind::ConfigurationError => write!(f, "ConfigurationError"),
            DatabaseErrorKind::MigrationError => write!(f, "MigrationError"),
            DatabaseErrorKind::Unknown => write!(f, "Unknown"),
            DatabaseErrorKind::NotImplemented => write!(f, "NotImplemented"),
            DatabaseErrorKind::Timeout => write!(f, "Timeout"),
            DatabaseErrorKind::ConstraintViolation => write!(f, "ConstraintViolation"),
            DatabaseErrorKind::AuthenticationError => write!(f, "AuthenticationError"),
            DatabaseErrorKind::ResourceError => write!(f, "ResourceError"),
            DatabaseErrorKind::InternalError => write!(f, "InternalError"),
            DatabaseErrorKind::SyntaxError => write!(f, "SyntaxError"),
            DatabaseErrorKind::DataIntegrityError => write!(f, "DataIntegrityError"),
            DatabaseErrorKind::ResourceExhausted => write!(f, "ResourceExhausted"),
            DatabaseErrorKind::TypeMismatch => write!(f, "TypeMismatch"),
            DatabaseErrorKind::SchemaError => write!(f, "SchemaError"),
            DatabaseErrorKind::SqlError => write!(f, "SqlError"),
            DatabaseErrorKind::ConversionError => write!(f, "ConversionError"),
        }
    }
}

/// fr fr SQL state codes for standard error classification
/// These follow the SQL standard SQLSTATE codes for interoperability
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlStateCode {
    /// 00000 - Successful completion
    Success,
    /// 01xxx - Warning
    Warning(String),
    /// 02000 - No data
    NoData,
    /// 07xxx - Dynamic SQL error
    DynamicSqlError(String),
    /// 08xxx - Connection exception
    ConnectionException(String),
    /// 09xxx - Triggered action exception
    TriggeredActionException(String),
    /// 0A xxx - Feature not supported
    FeatureNotSupported(String),
    /// 21xxx - Cardinality violation
    CardinalityViolation(String),
    /// 22xxx - Data exception
    DataException(String),
    /// 23xxx - Integrity constraint violation
    IntegrityConstraintViolation(String),
    /// 24xxx - Invalid cursor state
    InvalidCursorState(String),
    /// 25xxx - Invalid transaction state
    InvalidTransactionState(String),
    /// 26xxx - Invalid SQL statement name
    InvalidSqlStatementName(String),
    /// 27xxx - Triggered data change violation
    TriggeredDataChangeViolation(String),
    /// 28xxx - Invalid authorization specification
    InvalidAuthorizationSpecification(String),
    /// 2B xxx - Dependent privilege descriptors still exist
    DependentPrivilegeDescriptorsStillExist(String),
    /// 2D xxx - Invalid transaction termination
    InvalidTransactionTermination(String),
    /// 2E xxx - Invalid connection name
    InvalidConnectionName(String),
    /// 34xxx - Invalid cursor name
    InvalidCursorName(String),
    /// 35xxx - Invalid condition number
    InvalidConditionNumber(String),
    /// 3C xxx - Ambiguous cursor name
    AmbiguousCursorName(String),
    /// 3D xxx - Invalid catalog name
    InvalidCatalogName(String),
    /// 3F xxx - Invalid schema name
    InvalidSchemaName(String),
    /// 40xxx - Transaction rollback
    TransactionRollback(String),
    /// 42xxx - Syntax error or access rule violation
    SyntaxErrorOrAccessRuleViolation(String),
    /// 44xxx - WITH CHECK OPTION violation
    WithCheckOptionViolation(String),
    /// HY xxx - CLI-specific condition
    CliSpecificCondition(String),
    /// Custom state codes for driver-specific errors
    Custom(String),
}

impl Display for SqlStateCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SqlStateCode::Success => write!(f, "00000"),
            SqlStateCode::Warning(code) => write!(f, "01{}", code),
            SqlStateCode::NoData => write!(f, "02000"),
            SqlStateCode::DynamicSqlError(code) => write!(f, "07{}", code),
            SqlStateCode::ConnectionException(code) => write!(f, "08{}", code),
            SqlStateCode::TriggeredActionException(code) => write!(f, "09{}", code),
            SqlStateCode::FeatureNotSupported(code) => write!(f, "0A{}", code),
            SqlStateCode::CardinalityViolation(code) => write!(f, "21{}", code),
            SqlStateCode::DataException(code) => write!(f, "22{}", code),
            SqlStateCode::IntegrityConstraintViolation(code) => write!(f, "23{}", code),
            SqlStateCode::InvalidCursorState(code) => write!(f, "24{}", code),
            SqlStateCode::InvalidTransactionState(code) => write!(f, "25{}", code),
            SqlStateCode::InvalidSqlStatementName(code) => write!(f, "26{}", code),
            SqlStateCode::TriggeredDataChangeViolation(code) => write!(f, "27{}", code),
            SqlStateCode::InvalidAuthorizationSpecification(code) => write!(f, "28{}", code),
            SqlStateCode::DependentPrivilegeDescriptorsStillExist(code) => write!(f, "2B{}", code),
            SqlStateCode::InvalidTransactionTermination(code) => write!(f, "2D{}", code),
            SqlStateCode::InvalidConnectionName(code) => write!(f, "2E{}", code),
            SqlStateCode::InvalidCursorName(code) => write!(f, "34{}", code),
            SqlStateCode::InvalidConditionNumber(code) => write!(f, "35{}", code),
            SqlStateCode::AmbiguousCursorName(code) => write!(f, "3C{}", code),
            SqlStateCode::InvalidCatalogName(code) => write!(f, "3D{}", code),
            SqlStateCode::InvalidSchemaName(code) => write!(f, "3F{}", code),
            SqlStateCode::TransactionRollback(code) => write!(f, "40{}", code),
            SqlStateCode::SyntaxErrorOrAccessRuleViolation(code) => write!(f, "42{}", code),
            SqlStateCode::WithCheckOptionViolation(code) => write!(f, "44{}", code),
            SqlStateCode::CliSpecificCondition(code) => write!(f, "HY{}", code),
            SqlStateCode::Custom(code) => write!(f, "{}", code),
        }
    }
}

/// fr fr Comprehensive database error with rich context
#[derive(Debug)]
pub struct DatabaseError {
    /// fr fr Category of this error
    pub kind: DatabaseErrorKind,
    /// fr fr Human-readable error message
    pub message: String,
    /// fr fr SQL state code if applicable
    pub sql_state: Option<SqlStateCode>,
    /// fr fr Vendor-specific error code
    pub vendor_code: Option<i32>,
    /// fr fr Source location where error occurred
    pub source_location: Option<String>,
    /// fr fr Query that caused the error
    pub query: Option<String>,
    /// fr fr Additional context information
    pub context: std::collections::HashMap<String, String>,
    /// fr fr Underlying cause error
    pub cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl Clone for DatabaseError {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            message: self.message.clone(),
            sql_state: self.sql_state.clone(),
            vendor_code: self.vendor_code,
            source_location: self.source_location.clone(),
            query: self.query.clone(),
            context: self.context.clone(),
            cause: None, // Cannot clone trait objects, so we'll lose the cause
        }
    }
}

impl DatabaseError {
    /// slay Create a new database error
    pub fn new(kind: DatabaseErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
            sql_state: None,
            vendor_code: None,
            source_location: None,
            query: None,
            context: std::collections::HashMap::new(),
            cause: None,
        }
    }

    /// slay Create a new database error with SQL state
    pub fn with_sql_state(kind: DatabaseErrorKind, message: &str, sql_state: SqlStateCode) -> Self {
        Self {
            kind,
            message: message.to_string(),
            sql_state: Some(sql_state),
            vendor_code: None,
            source_location: None,
            query: None,
            context: std::collections::HashMap::new(),
            cause: None,
        }
    }

    /// slay Create a new database error with vendor code
    pub fn with_vendor_code(kind: DatabaseErrorKind, message: &str, vendor_code: i32) -> Self {
        Self {
            kind,
            message: message.to_string(),
            sql_state: None,
            vendor_code: Some(vendor_code),
            source_location: None,
            query: None,
            context: std::collections::HashMap::new(),
            cause: None,
        }
    }

    /// slay Create a new database error with full context
    pub fn with_context(
        kind: DatabaseErrorKind,
        message: &str,
        sql_state: Option<SqlStateCode>,
        vendor_code: Option<i32>,
        query: Option<String>,
    ) -> Self {
        Self {
            kind,
            message: message.to_string(),
            sql_state,
            vendor_code,
            source_location: None,
            query,
            context: std::collections::HashMap::new(),
            cause: None,
        }
    }

    /// slay Add source location to this error
    pub fn with_location(mut self, location: &str) -> Self {
        self.source_location = Some(location.to_string());
        self
    }

    /// slay Add query context to this error
    pub fn with_query(mut self, query: &str) -> Self {
        self.query = Some(query.to_string());
        self
    }

    /// slay Add additional context information
    pub fn with_context_data(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    /// slay Add underlying cause
    pub fn with_cause(mut self, cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
        self.cause = Some(cause);
        self
    }

    /// slay Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        match self.kind {
            DatabaseErrorKind::ConnectionError => true,
            DatabaseErrorKind::TimeoutError => true,
            DatabaseErrorKind::Timeout => true,
            DatabaseErrorKind::ResourceExhaustion => true,
            DatabaseErrorKind::ResourceExhausted => true,
            DatabaseErrorKind::ResourceError => true,
            DatabaseErrorKind::TransactionError => {
                // Check if it's a deadlock or similar retryable transaction error
                if let Some(ref sql_state) = self.sql_state {
                    match sql_state {
                        SqlStateCode::TransactionRollback(_) => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// slay Check if this error indicates data was not found
    pub fn is_not_found(&self) -> bool {
        matches!(self.kind, DatabaseErrorKind::NoRows) ||
        matches!(self.sql_state, Some(SqlStateCode::NoData))
    }

    /// slay Check if this error is a constraint violation
    pub fn is_constraint_violation(&self) -> bool {
        matches!(self.kind, DatabaseErrorKind::ConstraintError) ||
        matches!(self.kind, DatabaseErrorKind::ConstraintViolation) ||
        matches!(self.sql_state, Some(SqlStateCode::IntegrityConstraintViolation(_)))
    }

    /// slay Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self.kind {
            DatabaseErrorKind::NoRows | DatabaseErrorKind::NoLastInsertId => ErrorSeverity::Info,
            DatabaseErrorKind::TimeoutError | DatabaseErrorKind::ConnectionError => ErrorSeverity::Warning,
            DatabaseErrorKind::ConstraintError | DatabaseErrorKind::QueryError => ErrorSeverity::Error,
            DatabaseErrorKind::ResourceExhaustion | DatabaseErrorKind::DriverError => ErrorSeverity::Critical,
            _ => ErrorSeverity::Error,
        }
    }

    /// slay Create a connection error
    pub fn Connection(message: String) -> Self {
        Self::new(DatabaseErrorKind::ConnectionError, &message)
    }

    /// slay Create a query error
    pub fn Query(message: String) -> Self {
        Self::new(DatabaseErrorKind::QueryError, &message)
    }

    /// slay Create a transaction error
    pub fn Transaction(message: String) -> Self {
        Self::new(DatabaseErrorKind::TransactionError, &message)
    }

    /// slay Create a general error
    pub fn General(message: String) -> Self {
        Self::new(DatabaseErrorKind::Unknown, &message)
    }

    /// slay Create a configuration error
    pub fn Configuration(message: String) -> Self {
        Self::new(DatabaseErrorKind::ConfigurationError, &message)
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DatabaseError({}): {}", self.kind, self.message)?;
        
        if let Some(ref sql_state) = self.sql_state {
            write!(f, " [SQL State: {}]", sql_state)?;
        }
        
        if let Some(vendor_code) = self.vendor_code {
            write!(f, " [Vendor Code: {}]", vendor_code)?;
        }
        
        if let Some(ref location) = self.source_location {
            write!(f, " [Location: {}]", location)?;
        }
        
        if let Some(ref query) = self.query {
            write!(f, " [Query: {}]", query)?;
        }
        
        if !self.context.is_empty() {
            write!(f, " [Context: {:?}]", self.context)?;
        }
        
        Ok(())
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

/// fr fr Convert DatabaseError to CursedError for integration with the language error system
impl From<DatabaseError> for CursedError {
    fn from(err: DatabaseError) -> Self {
        let error_code = match err.kind {
            DatabaseErrorKind::ConnectionError => "DB_CONNECTION_ERROR",
            DatabaseErrorKind::QueryError => "DB_QUERY_ERROR",
            DatabaseErrorKind::TransactionError => "DB_TRANSACTION_ERROR",
            DatabaseErrorKind::ConstraintError => "DB_CONSTRAINT_ERROR",
            DatabaseErrorKind::ScanError => "DB_SCAN_ERROR",
            DatabaseErrorKind::SerializationError => "DB_SERIALIZATION_ERROR",
            DatabaseErrorKind::PoolError => "DB_POOL_ERROR",
            DatabaseErrorKind::DriverError => "DB_DRIVER_ERROR",
            DatabaseErrorKind::TimeoutError => "DB_TIMEOUT_ERROR",
            DatabaseErrorKind::NoRows => "DB_NO_ROWS",
            DatabaseErrorKind::NoLastInsertId => "DB_NO_LAST_INSERT_ID",
            DatabaseErrorKind::ResourceExhaustion => "DB_RESOURCE_EXHAUSTION",
            DatabaseErrorKind::ConfigurationError => "DB_CONFIGURATION_ERROR",
            DatabaseErrorKind::MigrationError => "DB_MIGRATION_ERROR",
            DatabaseErrorKind::Unknown => "DB_UNKNOWN_ERROR",
            DatabaseErrorKind::NotImplemented => "DB_NOT_IMPLEMENTED",
            DatabaseErrorKind::Timeout => "DB_TIMEOUT",
            DatabaseErrorKind::ConstraintViolation => "DB_CONSTRAINT_VIOLATION",
            DatabaseErrorKind::AuthenticationError => "DB_AUTHENTICATION_ERROR",
            DatabaseErrorKind::ResourceError => "DB_RESOURCE_ERROR",
            DatabaseErrorKind::InternalError => "DB_INTERNAL_ERROR",
            DatabaseErrorKind::SyntaxError => "DB_SYNTAX_ERROR",
            DatabaseErrorKind::DataIntegrityError => "DB_DATA_INTEGRITY_ERROR",
            DatabaseErrorKind::ResourceExhausted => "DB_RESOURCE_EXHAUSTED",
            DatabaseErrorKind::TypeMismatch => "DB_TYPE_MISMATCH",
            DatabaseErrorKind::SchemaError => "DB_SCHEMA_ERROR",
            DatabaseErrorKind::SqlError => "DB_SQL_ERROR",
            DatabaseErrorKind::ConversionError => "DB_CONVERSION_ERROR",
        };

        CursedError::Repl(format!("{}: {}", error_code, err.message))
    }
}

/// fr fr Error severity levels for proper logging and handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Informational - operation completed but with notable result
    Info,
    /// Warning - operation completed but with issues
    Warning,
    /// Error - operation failed but system can continue
    Error,
    /// Critical - operation failed and system integrity may be compromised
    Critical,
}

impl Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Info => write!(f, "INFO"),
            ErrorSeverity::Warning => write!(f, "WARNING"),
            ErrorSeverity::Error => write!(f, "ERROR"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// fr fr Helper functions for creating common database errors
impl DatabaseError {
    /// slay Create a connection error
    pub fn connection_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::ConnectionError, message)
    }

    /// slay Create a query error
    pub fn query_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::QueryError, message)
    }

    /// slay Create a transaction error
    pub fn transaction_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::TransactionError, message)
    }

    /// slay Create a constraint violation error
    pub fn constraint_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::ConstraintError, message)
    }

    /// slay Create a timeout error
    pub fn timeout_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::TimeoutError, message)
    }

    /// slay Create a pool error
    pub fn pool_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::PoolError, message)
    }

    /// slay Create a driver error
    pub fn driver_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::DriverError, message)
    }

    /// slay Create a serialization error
    pub fn serialization_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::SerializationError, message)
    }

    /// slay Create a validation error
    pub fn validation_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::ConstraintError, message)
    }

    /// slay Create an internal error
    pub fn internal_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::InternalError, message)
    }

    /// slay Create a not found error
    pub fn not_found(message: &str) -> Self {
        Self::new(DatabaseErrorKind::NoRows, message)
    }

    /// slay Create a not implemented error
    pub fn not_implemented(message: &str) -> Self {
        Self::new(DatabaseErrorKind::NotImplemented, message)
    }

    /// slay Create a constraint violation error
    pub fn constraint_violation(message: &str) -> Self {
        Self::new(DatabaseErrorKind::ConstraintViolation, message)
    }

    /// slay Create an authentication error
    pub fn authentication_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::AuthenticationError, message)
    }

    /// slay Create a syntax error
    pub fn syntax_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::SyntaxError, message)
    }

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
    };
    ($kind:expr, $msg:expr, $($arg:tt)*) => {
        DatabaseError::new($kind, &format!($msg, $($arg)*)).with_location(&format!("{}:{}", file!(), line!()))
    };
}

/// fr fr Helper macro for returning database errors
#[macro_export]
macro_rules! db_bail {
    ($kind:expr, $msg:expr) => {
        return Err(db_error!($kind, $msg))
    };
    ($kind:expr, $msg:expr, $($arg:tt)*) => {
        return Err(db_error!($kind, $msg, $($arg)*))
    };
}
