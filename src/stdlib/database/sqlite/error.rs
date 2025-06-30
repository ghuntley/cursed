//! SQLite error handling and types

use crate::error::CursedError;
use std::fmt;

/// SQLite error types
#[derive(Debug, Clone)]
pub struct SqliteError {
    /// Error kind
    pub kind: SqliteErrorKind,
    /// Error message
    pub message: String,
    /// SQLite error code
    pub code: Option<i32>,
}

/// SQLite error kinds
#[derive(Debug, Clone, PartialEq)]
pub enum SqliteErrorKind {
    /// Connection error
    Connection,
    /// SQL syntax error
    Syntax,
    /// Constraint violation
    Constraint,
    /// Transaction error
    Transaction,
    /// I/O error
    Io,
    /// Schema error
    Schema,
    /// Type mismatch
    Type,
    /// Generic error
    Generic,
}

impl SqliteError {
    /// Create a new SQLite error
    pub fn new(kind: SqliteErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
            code: None,
        }
    }
    
    /// Create error with code
    pub fn with_code(kind: SqliteErrorKind, message: String, code: i32) -> Self {
        Self {
            kind,
            message,
            code: Some(code),
        }
    }
    
    /// Create connection error
    pub fn connection_error(message: &str) -> Self {
        Self::new(SqliteErrorKind::Connection, message.to_string())
    }
    
    /// Create connection closed error
    pub fn connection_closed() -> Self {
        Self::new(SqliteErrorKind::Connection, "Connection is closed".to_string())
    }
    
    /// Create syntax error
    pub fn syntax_error(message: &str) -> Self {
        Self::new(SqliteErrorKind::Syntax, message.to_string())
    }
    
    /// Create constraint error
    pub fn constraint_error(message: &str) -> Self {
        Self::new(SqliteErrorKind::Constraint, message.to_string())
    }
    
    /// Create transaction error
    pub fn transaction_error(message: &str) -> Self {
        Self::new(SqliteErrorKind::Transaction, message.to_string())
    }
    
    /// Create transaction not active error
    pub fn transaction_not_active() -> Self {
        Self::new(SqliteErrorKind::Transaction, "Transaction is not active".to_string())
    }
    
    /// Create I/O error
    pub fn io_error(message: &str) -> Self {
        Self::new(SqliteErrorKind::Io, message.to_string())
    }
    
    /// Create schema error
    pub fn schema_error(message: &str) -> Self {
        Self::new(SqliteErrorKind::Schema, message.to_string())
    }
    
    /// Create type error
    pub fn type_error(message: &str) -> Self {
        Self::new(SqliteErrorKind::Type, message.to_string())
    }
    
    /// Create generic error
    pub fn generic_error(message: &str) -> Self {
        Self::new(SqliteErrorKind::Generic, message.to_string())
    }
}

impl fmt::Display for SqliteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.code {
            Some(code) => write!(f, "SQLite error ({}): {}", code, self.message),
            None => write!(f, "SQLite error: {}", self.message),
        }
    }
}

impl std::error::Error for SqliteError {}

impl From<SqliteError> for CursedError {
    fn from(error: SqliteError) -> Self {
        CursedError::runtime_error(&error.to_string())
    }
}

/// Legacy compatibility functions
/// Initialize error processing
pub fn init_error() -> Result<(), CursedError> {
    println!("⚙️  SQLite error handling initialized");
    Ok(())
}

/// Test error functionality
pub fn test_error() -> Result<(), CursedError> {
    let error = SqliteError::generic_error("Test error");
    println!("Error test completed: {}", error);
    Ok(())
}
