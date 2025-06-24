/// PostgreSQL-specific error handling
/// 
/// Provides comprehensive error handling for PostgreSQL operations including
/// native PostgreSQL error codes, connection errors, and proper integration
/// with the CURSED database error system.

use std::fmt;
use crate::stdlib::database::{DatabaseError, DatabaseErrorKind};

/// PostgreSQL-specific error kinds
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostgresErrorKind {
    /// Connection failed or was lost
    ConnectionFailed,
    /// Authentication failed
    AuthenticationFailed,
    /// SSL/TLS connection error
    SslError,
    /// Query execution error
    QueryError,
    /// Transaction error
    TransactionError,
    /// Type conversion error
    TypeConversionError,
    /// Configuration error
    InvalidConfiguration,
    /// Connection pool error
    PoolError,
    /// Timeout error
    TimeoutError,
    /// Protocol error
    ProtocolError,
    /// Database does not exist
    DatabaseNotFound,
    /// Permission denied
    PermissionDenied,
    /// Constraint violation
    ConstraintViolation,
    /// Syntax error in SQL
    SyntaxError,
    /// Data type error
    DataTypeError,
    /// Serialization failure
    SerializationFailure,
    /// Deadlock detected
    DeadlockDetected,
    /// Invalid cursor state
    InvalidCursorState,
    /// Feature not supported
    FeatureNotSupported,
    /// Server error (5xx class)
    ServerError,
    /// Other PostgreSQL error
    Other(String),
}

impl fmt::Display for PostgresErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PostgresErrorKind::ConnectionFailed => write!(f, "Connection failed"),
            PostgresErrorKind::AuthenticationFailed => write!(f, "Authentication failed"),
            PostgresErrorKind::SslError => write!(f, "SSL/TLS error"),
            PostgresErrorKind::QueryError => write!(f, "Query execution error"),
            PostgresErrorKind::TransactionError => write!(f, "Transaction error"),
            PostgresErrorKind::TypeConversionError => write!(f, "Type conversion error"),
            PostgresErrorKind::InvalidConfiguration => write!(f, "Invalid configuration"),
            PostgresErrorKind::PoolError => write!(f, "Connection pool error"),
            PostgresErrorKind::TimeoutError => write!(f, "Operation timeout"),
            PostgresErrorKind::ProtocolError => write!(f, "Protocol error"),
            PostgresErrorKind::DatabaseNotFound => write!(f, "Database not found"),
            PostgresErrorKind::PermissionDenied => write!(f, "Permission denied"),
            PostgresErrorKind::ConstraintViolation => write!(f, "Constraint violation"),
            PostgresErrorKind::SyntaxError => write!(f, "SQL syntax error"),
            PostgresErrorKind::DataTypeError => write!(f, "Data type error"),
            PostgresErrorKind::SerializationFailure => write!(f, "Serialization failure"),
            PostgresErrorKind::DeadlockDetected => write!(f, "Deadlock detected"),
            PostgresErrorKind::InvalidCursorState => write!(f, "Invalid cursor state"),
            PostgresErrorKind::FeatureNotSupported => write!(f, "Feature not supported"),
            PostgresErrorKind::ServerError => write!(f, "Server error"),
            PostgresErrorKind::Other(msg) => write!(f, "{}", msg),
        }
    }
}

/// PostgreSQL-specific error with detailed context
#[derive(Debug, Clone)]
pub struct PostgresError {
    /// Error kind
    pub kind: PostgresErrorKind,
    /// Error message
    pub message: String,
    /// PostgreSQL SQLSTATE code (if available)
    pub sqlstate: Option<String>,
    /// PostgreSQL error code (if available)
    pub code: Option<String>,
    /// Detail message from PostgreSQL
    pub detail: Option<String>,
    /// Hint message from PostgreSQL
    pub hint: Option<String>,
    /// Position in query where error occurred
    pub position: Option<u32>,
    /// Internal position in query
    pub internal_position: Option<u32>,
    /// Internal query that caused the error
    pub internal_query: Option<String>,
    /// Constraint name (for constraint violations)
    pub constraint: Option<String>,
    /// Table name where error occurred
    pub table: Option<String>,
    /// Column name where error occurred
    pub column: Option<String>,
    /// Data type name related to error
    pub datatype: Option<String>,
    /// Schema name where error occurred
    pub schema: Option<String>,
}

impl PostgresError {
    /// Create a new PostgreSQL error
    pub fn new(kind: PostgresErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
            sqlstate: None,
            code: None,
            detail: None,
            hint: None,
            position: None,
            internal_position: None,
            internal_query: None,
            constraint: None,
            table: None,
            column: None,
            datatype: None,
            schema: None,
        }
    }

    /// Create error from tokio-postgres error
    pub fn from_tokio_postgres(error: tokio_postgres::Error) -> Self {
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
        }
        
        pg_error
    }

    /// Create error from bb8 pool error
    pub fn from_bb8_error(error: bb8::RunError<tokio_postgres::Error>) -> Self {
        match error {
            bb8::RunError::User(pg_error) => Self::from_tokio_postgres(pg_error),
            bb8::RunError::TimedOut => Self::new(
                PostgresErrorKind::TimeoutError,
                "Connection pool timeout",
            ),
        }
    }

    /// Convert to generic database error
    pub fn to_database_error(&self) -> DatabaseError {
        let kind = match self.kind {
            PostgresErrorKind::ConnectionFailed => DatabaseErrorKind::ConnectionFailed,
            PostgresErrorKind::AuthenticationFailed => DatabaseErrorKind::AuthenticationFailed,
            PostgresErrorKind::SslError => DatabaseErrorKind::ConnectionFailed,
            PostgresErrorKind::QueryError => DatabaseErrorKind::QueryError,
            PostgresErrorKind::TransactionError => DatabaseErrorKind::TransactionError,
            PostgresErrorKind::TypeConversionError => DatabaseErrorKind::TypeConversionError,
            PostgresErrorKind::InvalidConfiguration => DatabaseErrorKind::InvalidConfiguration,
            PostgresErrorKind::PoolError => DatabaseErrorKind::ConnectionFailed,
            PostgresErrorKind::TimeoutError => DatabaseErrorKind::TimeoutError,
            PostgresErrorKind::ProtocolError => DatabaseErrorKind::ConnectionFailed,
            PostgresErrorKind::DatabaseNotFound => DatabaseErrorKind::DatabaseNotFound,
            PostgresErrorKind::PermissionDenied => DatabaseErrorKind::PermissionDenied,
            PostgresErrorKind::ConstraintViolation => DatabaseErrorKind::ConstraintViolation,
            PostgresErrorKind::SyntaxError => DatabaseErrorKind::SyntaxError,
            PostgresErrorKind::DataTypeError => DatabaseErrorKind::TypeConversionError,
            PostgresErrorKind::SerializationFailure => DatabaseErrorKind::TransactionError,
            PostgresErrorKind::DeadlockDetected => DatabaseErrorKind::DeadlockDetected,
            PostgresErrorKind::InvalidCursorState => DatabaseErrorKind::QueryError,
            PostgresErrorKind::FeatureNotSupported => DatabaseErrorKind::NotSupported,
            PostgresErrorKind::ServerError => DatabaseErrorKind::ServerError,
            PostgresErrorKind::Other(_) => DatabaseErrorKind::UnknownError,
        };
        
        let mut message = self.message.clone();
        if let Some(ref detail) = self.detail {
            message.push_str(&format!(" Detail: {}", detail));
        }
        if let Some(ref hint) = self.hint {
            message.push_str(&format!(" Hint: {}", hint));
        }
        
        DatabaseError::new(kind, &message)
    }

    /// Get SQLSTATE code if available
    pub fn sqlstate(&self) -> Option<&str> {
        self.sqlstate.as_deref()
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self.kind,
            PostgresErrorKind::ConnectionFailed
                | PostgresErrorKind::TimeoutError
                | PostgresErrorKind::PoolError
                | PostgresErrorKind::SerializationFailure
                | PostgresErrorKind::DeadlockDetected
        )
    }

    /// Check if error is due to connection loss
    pub fn is_connection_error(&self) -> bool {
        matches!(
            self.kind,
            PostgresErrorKind::ConnectionFailed
                | PostgresErrorKind::SslError
                | PostgresErrorKind::ProtocolError
                | PostgresErrorKind::PoolError
        )
    }
}

impl fmt::Display for PostgresError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PostgreSQL {}: {}", self.kind, self.message)?;
        
        if let Some(ref sqlstate) = self.sqlstate {
            write!(f, " (SQLSTATE: {})", sqlstate)?;
        }
        
        if let Some(ref detail) = self.detail {
            write!(f, " Detail: {}", detail)?;
        }
        
        if let Some(ref hint) = self.hint {
            write!(f, " Hint: {}", hint)?;
        }
        
        Ok(())
    }
}

impl std::error::Error for PostgresError {}

impl From<PostgresError> for DatabaseError {
    fn from(error: PostgresError) -> Self {
        error.to_database_error()
    }
}

impl From<tokio_postgres::Error> for PostgresError {
    fn from(error: tokio_postgres::Error) -> Self {
        Self::from_tokio_postgres(error)
    }
}

impl From<bb8::RunError<tokio_postgres::Error>> for PostgresError {
    fn from(error: bb8::RunError<tokio_postgres::Error>) -> Self {
        Self::from_bb8_error(error)
    }
}

/// Classify tokio-postgres error into PostgreSQL error kind
fn classify_tokio_postgres_error(error: &tokio_postgres::Error) -> PostgresErrorKind {
    if error.is_closed() {
        return PostgresErrorKind::ConnectionFailed;
    }
    
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
                "40001" => PostgresErrorKind::SerializationFailure,
                "40P01" => PostgresErrorKind::DeadlockDetected,
                _ => PostgresErrorKind::QueryError,
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
}

/// Type alias for PostgreSQL result
pub type PostgresResult<T> = std::result::Result<T, PostgresError>;

#[cfg(test)]
mod tests {
    use super::*;
use crate::error::Error;

    #[test]
    fn test_error_creation() {
        let error = PostgresError::new(PostgresErrorKind::ConnectionFailed, "Connection lost");
        assert_eq!(error.kind, PostgresErrorKind::ConnectionFailed);
        assert_eq!(error.message, "Connection lost");
    }

    #[test]
    fn test_error_conversion() {
        let pg_error = PostgresError::new(PostgresErrorKind::QueryError, "Query failed");
        let db_error: DatabaseError = pg_error.into();
        assert!(matches!(db_error.kind(), DatabaseErrorKind::QueryError));
    }

    #[test]
    fn test_retryable_errors() {
        let retryable = PostgresError::new(PostgresErrorKind::ConnectionFailed, "Connection lost");
        assert!(retryable.is_retryable());
        
        let non_retryable = PostgresError::new(PostgresErrorKind::SyntaxError, "Bad SQL");
        assert!(!non_retryable.is_retryable());
    }

    #[test]
    fn test_connection_errors() {
        let conn_error = PostgresError::new(PostgresErrorKind::ConnectionFailed, "Connection lost");
        assert!(conn_error.is_connection_error());
        
        let query_error = PostgresError::new(PostgresErrorKind::QueryError, "Bad query");
        assert!(!query_error.is_connection_error());
    }
}
