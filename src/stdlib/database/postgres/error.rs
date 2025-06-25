/// PostgreSQL-specific error handling
/// 
/// Provides comprehensive error handling for PostgreSQL operations including
/// native PostgreSQL error codes, connection errors, and proper integration
/// with the CURSED database error system.

use std::fmt;
// use crate::stdlib::database::{DatabaseError, DatabaseErrorKind};

/// PostgreSQL-specific error kinds
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostgresErrorKind {
    /// Connection failed or was lost
    /// Authentication failed
    /// SSL/TLS connection error
    /// Query execution error
    /// Transaction error
    /// Type conversion error
    /// Configuration error
    /// Connection pool error
    /// Timeout error
    /// Protocol error
    /// Database does not exist
    /// Permission denied
    /// Constraint violation
    /// Syntax error in SQL
    /// Data type error
    /// Serialization failure
    /// Deadlock detected
    /// Invalid cursor state
    /// Feature not supported
    /// Server error (5xx class)
    /// Other PostgreSQL error
// impl fmt::Display for PostgresErrorKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             PostgresErrorKind::ConnectionFailed => write!(f, "Connection failed"),
//             PostgresErrorKind::AuthenticationFailed => write!(f, "Authentication failed"),
//             PostgresErrorKind::SslError => write!(f, "SSL/TLS error"),
//             PostgresErrorKind::QueryError => write!(f, "Query execution error"),
//             PostgresErrorKind::TransactionError => write!(f, "Transaction error"),
//             PostgresErrorKind::TypeConversionError => write!(f, "Type conversion error"),
//             PostgresErrorKind::InvalidConfiguration => write!(f, "Invalid configuration"),
//             PostgresErrorKind::PoolError => write!(f, "Connection pool error"),
//             PostgresErrorKind::TimeoutError => write!(f, "Operation timeout"),
//             PostgresErrorKind::ProtocolError => write!(f, "Protocol error"),
//             PostgresErrorKind::DatabaseNotFound => write!(f, "Database not found"),
//             PostgresErrorKind::PermissionDenied => write!(f, "Permission denied"),
//             PostgresErrorKind::ConstraintViolation => write!(f, "Constraint violation"),
//             PostgresErrorKind::SyntaxError => write!(f, "SQL syntax error"),
//             PostgresErrorKind::DataTypeError => write!(f, "Data type error"),
//             PostgresErrorKind::SerializationFailure => write!(f, "Serialization failure"),
//             PostgresErrorKind::DeadlockDetected => write!(f, "Deadlock detected"),
//             PostgresErrorKind::InvalidCursorState => write!(f, "Invalid cursor state"),
//             PostgresErrorKind::FeatureNotSupported => write!(f, "Feature not supported"),
//             PostgresErrorKind::ServerError => write!(f, "Server error"),
//             PostgresErrorKind::Other(msg) => write!(f, "{}", msg),
//         }
//     }
// }

/// PostgreSQL-specific error with detailed context
#[derive(Debug, Clone)]
pub struct PostgresError {
    /// CursedError kind
    /// CursedError message
    /// PostgreSQL SQLSTATE code (if available)
    /// PostgreSQL error code (if available)
    /// Detail message from PostgreSQL
    /// Hint message from PostgreSQL
    /// Position in query where error occurred
    /// Internal position in query
    /// Internal query that caused the error
    /// Constraint name (for constraint violations)
    /// Table name where error occurred
    /// Column name where error occurred
    /// Data type name related to error
    /// Schema name where error occurred
impl PostgresError {
    /// Create a new PostgreSQL error
    pub fn new(kind: PostgresErrorKind, message: &str) -> Self {
        Self {
        }
    }

    /// Create error from tokio-postgres error
    pub fn from_tokio_postgres(error: tokio_postgres::CursedError) -> Self {
        let kind = classify_tokio_postgres_error(&error);
        let mut pg_error = Self::new(kind, &error.to_string());
        
        // Extract PostgreSQL error fields if available
        if let Some(db_error) = error.as_db_error() {
            pg_error.sqlstate = Some(db_error.code().code().to_string());
            pg_error.detail = db_error.detail().map(|s| s.to_string());
            pg_error.hint = db_error.hint().map(|s| s.to_string());
            pg_error.position = db_error.position().map(|p| *p as u32);
            pg_error.internal_position = db_error.internal_position().map(|p| *p as u32);
            pg_error.internal_query = db_error.internal_query().map(|s| s.to_string());
            pg_error.constraint = db_error.constraint().map(|s| s.to_string());
            pg_error.table = db_error.table().map(|s| s.to_string());
            pg_error.column = db_error.column().map(|s| s.to_string());
            pg_error.datatype = db_error.datatype().map(|s| s.to_string());
            pg_error.schema = db_error.schema().map(|s| s.to_string());
        pg_error
    /// Create error from bb8 pool error
    pub fn from_bb8_error(error: bb8::RunError<tokio_postgres::CursedError>) -> Self {
        match error {
            bb8::RunError::TimedOut => Self::new(
        }
    }

    /// Convert to generic database error
    pub fn to_database_error(&self) -> DatabaseError {
        let kind = match self.kind {
        
        let mut message = self.message.clone();
        if let Some(ref detail) = self.detail {
            message.push_str(&format!(" Detail: {}", detail));
        }
        if let Some(ref hint) = self.hint {
            message.push_str(&format!(" Hint: {}", hint));
        DatabaseError::new(kind, &message)
    /// Get SQLSTATE code if available
    pub fn sqlstate(&self) -> Option<&str> {
        self.sqlstate.as_deref()
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            PostgresErrorKind::ConnectionFailed
                | PostgresErrorKind::TimeoutError
                | PostgresErrorKind::PoolError
                | PostgresErrorKind::SerializationFailure
                | PostgresErrorKind::DeadlockDetected
        )
    /// Check if error is due to connection loss
    pub fn is_connection_error(&self) -> bool {
        matches!(
            PostgresErrorKind::ConnectionFailed
                | PostgresErrorKind::SslError
                | PostgresErrorKind::ProtocolError
                | PostgresErrorKind::PoolError
        )
    }
}

// impl fmt::Display for PostgresError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "PostgreSQL {}: {}", self.kind, self.message)?;
//         
//         if let Some(ref sqlstate) = self.sqlstate {
//             write!(f, " (SQLSTATE: {})", sqlstate)?;
//         }
//         
//         if let Some(ref detail) = self.detail {
//             write!(f, " Detail: {}", detail)?;
//         }
//         
//         if let Some(ref hint) = self.hint {
//             write!(f, " Hint: {}", hint)?;
//         }
//         
//         Ok(())
//     }
// }

// impl std::error::CursedError for PostgresError {}
// 
impl From<PostgresError> for DatabaseError {
    fn from(error: PostgresError) -> Self {
        error.to_database_error()
    }
}

// impl From<tokio_postgres::CursedError> for PostgresError {
//     fn from(error: tokio_postgres::CursedError) -> Self {
//         Self::from_tokio_postgres(error)
//     }
// }

// impl From<bb8::RunError<tokio_postgres::CursedError>> for PostgresError {
//     fn from(error: bb8::RunError<tokio_postgres::CursedError>) -> Self {
//         Self::from_bb8_error(error)
//     }
// }

/// Classify tokio-postgres error into PostgreSQL error kind
fn classify_tokio_postgres_error(error: &tokio_postgres::CursedError) -> PostgresErrorKind {
    if error.is_closed() {
        return PostgresErrorKind::ConnectionFailed;
    if let Some(db_error) = error.as_db_error() {
        let code = db_error.code().code();
        
        // Classify by SQLSTATE class
        match &code[0..2] {
            "02" => PostgresErrorKind::DatabaseNotFound, // No data
            "08" => PostgresErrorKind::ConnectionFailed,  // Connection exception
            "28" => PostgresErrorKind::AuthenticationFailed, // Invalid authorization
            "42" => PostgresErrorKind::SyntaxError,       // Syntax error or access rule violation
            "23" => PostgresErrorKind::ConstraintViolation, // Integrity constraint violation
            "40" => PostgresErrorKind::TransactionError,  // Transaction rollback
            "53" => PostgresErrorKind::ServerError,       // Insufficient resources
            "54" => PostgresErrorKind::ServerError,       // Program limit exceeded
            "55" => PostgresErrorKind::ServerError,       // Object not in prerequisite state
            "57" => PostgresErrorKind::ServerError,       // Operator intervention
            "58" => PostgresErrorKind::ServerError,       // System error
            _ => match code {
            }
        }
    } else {
        // Classify non-database errors
        let error_str = error.to_string();
        if error_str.contains("timeout") || error_str.contains("timed out") {
            PostgresErrorKind::TimeoutError
        } else if error_str.contains("SSL") || error_str.contains("TLS") {
            PostgresErrorKind::SslError
        } else if error_str.contains("connection") {
            PostgresErrorKind::ConnectionFailed
        } else if error_str.contains("authentication") || error_str.contains("password") {
            PostgresErrorKind::AuthenticationFailed
        } else {
            PostgresErrorKind::Other(error_str)
        }
    }
/// Type alias for PostgreSQL result
pub type PostgresResult<T> = std::result::Result<T, PostgresError>;

