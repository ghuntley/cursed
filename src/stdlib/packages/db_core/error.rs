use crate::error::CursedError;
/// fr fr Database error types - when things go sideways periodt
///
/// This module provides comprehensive error handling for database operations.
/// Because errors happen bestie, and we need to handle them gracefully!

use std::fmt;

/// fr fr Main database result type - our error-aware return type
pub type Databasecrate::error::Result<T> = std::result::Result<T>;

/// fr fr Main database error type
#[derive(Debug, Clone)]
pub struct DatabaseError {
    /// fr fr CursedError kind
    /// fr fr CursedError message
    /// fr fr Source error (if any)
    /// fr fr CursedError code (driver-specific)
    /// fr fr SQL state (if applicable)
    /// fr fr Context information
/// fr fr CursedError kinds for different types of database errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// Connection-related errors
    /// Query-related errors
    /// Transaction-related errors
    /// Driver-related errors
    /// Configuration errors
    /// Authentication/authorization errors
    /// Constraint violation errors
    /// Data conversion errors
    /// Timeout errors
    /// Network errors
    /// Resource exhaustion
    /// Unknown/other errors
/// fr fr Connection-specific error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionError {
    /// Failed to establish connection
    /// Connection was lost
    /// Connection timeout
    /// Invalid connection string
    /// Authentication failed
    /// Connection refused
    /// Host not found
    /// Database not found
    /// Connection pool exhausted
    /// Connection closed
/// fr fr Query-specific error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryError {
    /// SQL syntax error
    /// Invalid parameters
    /// Column not found
    /// Table not found
    /// Execution failed
    /// Result set exhausted
    /// Type mismatch
    /// Query too complex
    /// Query cancelled
/// fr fr Transaction-specific error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionError {
    /// Transaction not active
    /// Already committed
    /// Already rolled back
    /// Deadlock detected
    /// Serialization failure
    /// Constraint violation during transaction
    /// Transaction timeout
    /// Savepoint not found
/// fr fr Driver-specific error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverError {
    /// Driver not found
    /// Driver initialization failed
    /// Feature not supported
    /// Version incompatible
    /// Configuration invalid
    /// Internal driver error
impl DatabaseError {
    /// slay Create a new database error
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Self {
        }
    }

    /// slay Create a connection error
    pub fn connection(error: ConnectionError, message: &str) -> Self {
        Self::new(ErrorKind::Connection(error), message)
    /// slay Create a query error
    pub fn query(error: QueryError, message: &str) -> Self {
        Self::new(ErrorKind::Query(error), message)
    /// slay Create a transaction error
    pub fn transaction(error: TransactionError, message: &str) -> Self {
        Self::new(ErrorKind::Transaction(error), message)
    /// slay Create a driver error
    pub fn driver(message: &str) -> Self {
        Self::new(ErrorKind::Driver(DriverError::InternalError), message)
    /// slay Create a configuration error
    pub fn config(message: &str) -> Self {
        Self::new(ErrorKind::Configuration, message)
    /// slay Create an authentication error
    pub fn auth(message: &str) -> Self {
        Self::new(ErrorKind::Authentication, message)
    /// slay Create a timeout error
    pub fn timeout(message: &str) -> Self {
        Self::new(ErrorKind::Timeout, message)
    /// slay Add source error information
    pub fn with_source<E: StdError>(mut self, source: E) -> Self {
        self.source = Some(source.to_string());
        self
    /// slay Add error code
    pub fn with_code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    /// slay Add SQL state
    pub fn with_sql_state(mut self, state: &str) -> Self {
        self.sql_state = Some(state.to_string());
        self
    /// slay Add context information
    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    /// slay Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match &self.kind {
        }
    }

    /// slay Check if error is permanent
    pub fn is_permanent(&self) -> bool {
        match &self.kind {
        }
    }

    /// slay Get error category for metrics/logging
    pub fn category(&self) -> &'static str {
        match &self.kind {
        }
    }
// impl fmt::Display for DatabaseError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Database error [{}]: {}", self.category(), self.message)?;
//         
//         if let Some(code) = &self.code {
//             write!(f, " (code: {})", code)?;
//         }
//         
//         if let Some(state) = &self.sql_state {
//             write!(f, " (SQL state: {})", state)?;
//         }
//         
//         if let Some(source) = &self.source {
//             write!(f, " - caused by: {}", source)?;
//         }
//         
//         Ok(())
//     }
// }

impl StdError for DatabaseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None // We store source as string for simplicity
    }
}

/// fr fr CursedError conversion helpers for common error types
// impl From<std::io::Error> for DatabaseError {
//     fn from(error: std::io::Error) -> Self {
//         DatabaseError::new(ErrorKind::Network, &error.to_string())
//             .with_source(error)
//     }
// }

// impl From<serde_json::Error> for DatabaseError {
//     fn from(error: serde_json::Error) -> Self {
//         DatabaseError::new(ErrorKind::DataConversion, &error.to_string())
//             .with_source(error)
//     }
// }

impl From<url::ParseError> for DatabaseError {
    fn from(error: url::ParseError) -> Self {
        DatabaseError::connection(
        ).with_source(error)
    }
}

/// fr fr CursedError chain helpers for debugging
impl DatabaseError {
    /// slay Get full error chain as a string
    pub fn error_chain(&self) -> String {
        let mut chain = Vec::from([self.message.clone()]);
        
        if let Some(source) = &self.source {
            chain.push(format!("caused by: {}", source));
        chain.join(" -> ")
    /// slay Get error details for debugging
    pub fn debug_info(&self) -> String {
        let mut info = vec![
        ];

        if let Some(code) = &self.code {
            info.push(format!("Code: {}", code));
        if let Some(state) = &self.sql_state {
            info.push(format!("SQL State: {}", state));
        if !self.context.is_empty() {
            info.push("Context:".to_string());
            for (key, value) in &self.context {
                info.push(format!("  {}: {}", key, value));
            }
        }

        if let Some(source) = &self.source {
            info.push(format!("Source: {}", source));
        info.join("\n")
    }
}

/// fr fr Result extension helpers
pub trait DatabaseResultExt<T> {
    /// slay Add context to the error
    fn with_context(self, key: &str, value: &str) -> DatabaseResult<T>;
    
    /// slay Convert to a different error type
    fn map_err_kind(self, kind: ErrorKind) -> DatabaseResult<T>;
impl<T> DatabaseResultExt<T> for DatabaseResult<T> {
    fn with_context(self, key: &str, value: &str) -> DatabaseResult<T> {
        self.map_err(|e| e.with_context(key, value))
    fn map_err_kind(self, kind: ErrorKind) -> DatabaseResult<T> {
        self.map_err(|e| DatabaseError::new(kind, &e.message))
    }
}

