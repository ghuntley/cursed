/// fr fr Database error types - when things go sideways periodt
///
/// This module provides comprehensive error handling for database operations.
/// Because errors happen bestie, and we need to handle them gracefully!

use std::fmt;
use std::error::Error as StdError;

/// fr fr Main database result type - our error-aware return type
pub type DatabaseResult<T> = Result<T, DatabaseError>;

/// fr fr Main database error type
#[derive(Debug, Clone)]
pub struct DatabaseError {
    /// fr fr Error kind
    pub kind: ErrorKind,
    /// fr fr Error message
    pub message: String,
    /// fr fr Source error (if any)
    pub source: Option<String>,
    /// fr fr Error code (driver-specific)
    pub code: Option<String>,
    /// fr fr SQL state (if applicable)
    pub sql_state: Option<String>,
    /// fr fr Context information
    pub context: std::collections::HashMap<String, String>,
}

/// fr fr Error kinds for different types of database errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// Connection-related errors
    Connection(ConnectionError),
    /// Query-related errors
    Query(QueryError),
    /// Transaction-related errors
    Transaction(TransactionError),
    /// Driver-related errors
    Driver(DriverError),
    /// Configuration errors
    Configuration,
    /// Authentication/authorization errors
    Authentication,
    /// Constraint violation errors
    ConstraintViolation,
    /// Data conversion errors
    DataConversion,
    /// Timeout errors
    Timeout,
    /// Network errors
    Network,
    /// Resource exhaustion
    ResourceExhausted,
    /// Unknown/other errors
    Other,
}

/// fr fr Connection-specific error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionError {
    /// Failed to establish connection
    FailedToConnect,
    /// Connection was lost
    ConnectionLost,
    /// Connection timeout
    Timeout,
    /// Invalid connection string
    InvalidConnectionString,
    /// Authentication failed
    AuthenticationFailed,
    /// Connection refused
    Refused,
    /// Host not found
    HostNotFound,
    /// Database not found
    DatabaseNotFound,
    /// Connection pool exhausted
    PoolExhausted,
    /// Connection closed
    Closed,
}

/// fr fr Query-specific error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryError {
    /// SQL syntax error
    SyntaxError,
    /// Invalid parameters
    InvalidParameters,
    /// Column not found
    ColumnNotFound,
    /// Table not found
    TableNotFound,
    /// Execution failed
    ExecutionFailed,
    /// Result set exhausted
    ResultSetExhausted,
    /// Type mismatch
    TypeMismatch,
    /// Query too complex
    TooComplex,
    /// Query cancelled
    Cancelled,
}

/// fr fr Transaction-specific error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionError {
    /// Transaction not active
    NotActive,
    /// Already committed
    AlreadyCommitted,
    /// Already rolled back
    AlreadyRolledBack,
    /// Deadlock detected
    Deadlock,
    /// Serialization failure
    SerializationFailure,
    /// Constraint violation during transaction
    ConstraintViolation,
    /// Transaction timeout
    Timeout,
    /// Savepoint not found
    SavepointNotFound,
}

/// fr fr Driver-specific error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverError {
    /// Driver not found
    NotFound,
    /// Driver initialization failed
    InitializationFailed,
    /// Feature not supported
    FeatureNotSupported,
    /// Version incompatible
    VersionIncompatible,
    /// Configuration invalid
    InvalidConfiguration,
    /// Internal driver error
    InternalError,
}

impl DatabaseError {
    /// slay Create a new database error
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
            source: None,
            code: None,
            sql_state: None,
            context: std::collections::HashMap::new(),
        }
    }

    /// slay Create a connection error
    pub fn connection(error: ConnectionError, message: &str) -> Self {
        Self::new(ErrorKind::Connection(error), message)
    }

    /// slay Create a query error
    pub fn query(error: QueryError, message: &str) -> Self {
        Self::new(ErrorKind::Query(error), message)
    }

    /// slay Create a transaction error
    pub fn transaction(error: TransactionError, message: &str) -> Self {
        Self::new(ErrorKind::Transaction(error), message)
    }

    /// slay Create a driver error
    pub fn driver(message: &str) -> Self {
        Self::new(ErrorKind::Driver(DriverError::InternalError), message)
    }

    /// slay Create a configuration error
    pub fn config(message: &str) -> Self {
        Self::new(ErrorKind::Configuration, message)
    }

    /// slay Create an authentication error
    pub fn auth(message: &str) -> Self {
        Self::new(ErrorKind::Authentication, message)
    }

    /// slay Create a timeout error
    pub fn timeout(message: &str) -> Self {
        Self::new(ErrorKind::Timeout, message)
    }

    /// slay Add source error information
    pub fn with_source<E: StdError>(mut self, source: E) -> Self {
        self.source = Some(source.to_string());
        self
    }

    /// slay Add error code
    pub fn with_code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }

    /// slay Add SQL state
    pub fn with_sql_state(mut self, state: &str) -> Self {
        self.sql_state = Some(state.to_string());
        self
    }

    /// slay Add context information
    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    /// slay Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match &self.kind {
            ErrorKind::Connection(ConnectionError::Timeout) => true,
            ErrorKind::Connection(ConnectionError::ConnectionLost) => true,
            ErrorKind::Network => true,
            ErrorKind::Timeout => true,
            ErrorKind::Transaction(TransactionError::Deadlock) => true,
            ErrorKind::Transaction(TransactionError::SerializationFailure) => true,
            _ => false,
        }
    }

    /// slay Check if error is permanent
    pub fn is_permanent(&self) -> bool {
        match &self.kind {
            ErrorKind::Authentication => true,
            ErrorKind::Configuration => true,
            ErrorKind::Query(QueryError::SyntaxError) => true,
            ErrorKind::Query(QueryError::TableNotFound) => true,
            ErrorKind::Query(QueryError::ColumnNotFound) => true,
            ErrorKind::Driver(DriverError::NotFound) => true,
            ErrorKind::Driver(DriverError::FeatureNotSupported) => true,
            _ => false,
        }
    }

    /// slay Get error category for metrics/logging
    pub fn category(&self) -> &'static str {
        match &self.kind {
            ErrorKind::Connection(_) => "connection",
            ErrorKind::Query(_) => "query",
            ErrorKind::Transaction(_) => "transaction",
            ErrorKind::Driver(_) => "driver",
            ErrorKind::Configuration => "configuration",
            ErrorKind::Authentication => "authentication",
            ErrorKind::ConstraintViolation => "constraint",
            ErrorKind::DataConversion => "conversion",
            ErrorKind::Timeout => "timeout",
            ErrorKind::Network => "network",
            ErrorKind::ResourceExhausted => "resource",
            ErrorKind::Other => "other",
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database error [{}]: {}", self.category(), self.message)?;
        
        if let Some(code) = &self.code {
            write!(f, " (code: {})", code)?;
        }
        
        if let Some(state) = &self.sql_state {
            write!(f, " (SQL state: {})", state)?;
        }
        
        if let Some(source) = &self.source {
            write!(f, " - caused by: {}", source)?;
        }
        
        Ok(())
    }
}

impl StdError for DatabaseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None // We store source as string for simplicity
    }
}

/// fr fr Error conversion helpers for common error types
impl From<std::io::Error> for DatabaseError {
    fn from(error: std::io::Error) -> Self {
        DatabaseError::new(ErrorKind::Network, &error.to_string())
            .with_source(error)
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(error: serde_json::Error) -> Self {
        DatabaseError::new(ErrorKind::DataConversion, &error.to_string())
            .with_source(error)
    }
}

impl From<url::ParseError> for DatabaseError {
    fn from(error: url::ParseError) -> Self {
        DatabaseError::connection(
            ConnectionError::InvalidConnectionString,
            &error.to_string(),
        ).with_source(error)
    }
}

/// fr fr Error chain helpers for debugging
impl DatabaseError {
    /// slay Get full error chain as a string
    pub fn error_chain(&self) -> String {
        let mut chain = vec![self.message.clone()];
        
        if let Some(source) = &self.source {
            chain.push(format!("caused by: {}", source));
        }
        
        chain.join(" -> ")
    }

    /// slay Get error details for debugging
    pub fn debug_info(&self) -> String {
        let mut info = vec![
            format!("Error: {}", self.message),
            format!("Kind: {:?}", self.kind),
            format!("Category: {}", self.category()),
            format!("Retryable: {}", self.is_retryable()),
            format!("Permanent: {}", self.is_permanent()),
        ];

        if let Some(code) = &self.code {
            info.push(format!("Code: {}", code));
        }

        if let Some(state) = &self.sql_state {
            info.push(format!("SQL State: {}", state));
        }

        if !self.context.is_empty() {
            info.push("Context:".to_string());
            for (key, value) in &self.context {
                info.push(format!("  {}: {}", key, value));
            }
        }

        if let Some(source) = &self.source {
            info.push(format!("Source: {}", source));
        }

        info.join("\n")
    }
}

/// fr fr Result extension helpers
pub trait DatabaseResultExt<T> {
    /// slay Add context to the error
    fn with_context(self, key: &str, value: &str) -> DatabaseResult<T>;
    
    /// slay Convert to a different error type
    fn map_err_kind(self, kind: ErrorKind) -> DatabaseResult<T>;
}

impl<T> DatabaseResultExt<T> for DatabaseResult<T> {
    fn with_context(self, key: &str, value: &str) -> DatabaseResult<T> {
        self.map_err(|e| e.with_context(key, value))
    }
    
    fn map_err_kind(self, kind: ErrorKind) -> DatabaseResult<T> {
        self.map_err(|e| DatabaseError::new(kind, &e.message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_error_creation() {
        let error = DatabaseError::connection(
            ConnectionError::FailedToConnect,
            "Could not connect to database"
        ).with_code("08001").with_context("host", "localhost");

        assert_eq!(error.category(), "connection");
        assert!(error.is_retryable());
        assert!(!error.is_permanent());
        assert_eq!(error.code, Some("08001".to_string()));
        assert_eq!(error.context.get("host"), Some(&"localhost".to_string()));
    }

    #[test]
    fn test_error_chain() {
        let error = DatabaseError::query(
            QueryError::SyntaxError,
            "Invalid SQL syntax"
        ).with_source(std::io::Error::new(std::io::ErrorKind::Other, "Source error"));

        let chain = error.error_chain();
        assert!(chain.contains("Invalid SQL syntax"));
        assert!(chain.contains("caused by"));
    }

    #[test]
    fn test_retryable_errors() {
        let timeout_error = DatabaseError::timeout("Query timeout");
        assert!(timeout_error.is_retryable());

        let syntax_error = DatabaseError::query(QueryError::SyntaxError, "Bad SQL");
        assert!(!syntax_error.is_retryable());
        assert!(syntax_error.is_permanent());
    }

    #[test]
    fn test_error_conversions() {
        let io_error = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection refused");
        let db_error: DatabaseError = io_error.into();
        assert_eq!(db_error.category(), "network");
    }

    #[test]
    fn test_result_extensions() {
        let result: DatabaseResult<i32> = Err(DatabaseError::driver("Test error"));
        let result_with_context = result.with_context("operation", "test");
        
        if let Err(error) = result_with_context {
            assert_eq!(error.context.get("operation"), Some(&"test".to_string()));
        }
    }
}
