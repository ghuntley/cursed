//! Functional implementation for error

use crate::error::CursedError;
use std::fmt;
use crate::stdlib::packages::ModuleError;

/// Result type for error operations
pub type ModuleResult<T> = Result<T, CursedError>;

/// SQL-specific error type
#[derive(Debug, Clone)]
pub struct SqlError {
    pub kind: DatabaseErrorKind,
    pub message: String,
}

impl SqlError {
    pub fn new(kind: DatabaseErrorKind, message: String) -> Self {
        Self { kind, message }
    }
}

impl fmt::Display for SqlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for SqlError {}

/// SQL result type
pub type SqlResult<T> = Result<T, SqlError>;

/// Database error categories
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseErrorKind {
    Connection(ConnectionErrorKind),
    Query(QueryErrorKind),
    Transaction,
    Authentication,
    Timeout,
    Unknown,
}

/// Query-specific errors
#[derive(Debug, Clone, PartialEq)]
pub enum QueryErrorKind {
    Syntax,
    InvalidColumn,
    InvalidTable,
    ConstraintViolation,
    TypeMismatch,
    Unknown,
}

/// Connection-specific errors
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionErrorKind {
    Refused,
    Timeout,
    Lost,
    InvalidCredentials,
    DatabaseNotFound,
    Unknown,
}

/// error operations handler
pub struct ModuleHandler {
    enabled: bool,
}

impl ModuleHandler {
    /// Create a new module handler
    pub fn new() -> Self {
        Self {
            enabled: true,
        }
    }
    
    /// Enable or disable the module
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Check if module is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Process data
    pub fn process(&self, data: &str) -> ModuleResult<String> {
        if !self.enabled {
            return Err(CursedError::runtime_error(&"Module is disabled".to_string()));
        }
        Ok(format!("Processed: {}", data))
    }
    
    /// Get module info
    pub fn info(&self) -> String {
        format!("Module: error, Enabled: {}", self.enabled)
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize error processing
pub fn init_error() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("test")?;
    if !result.contains("test") {
        return Err(CursedError::runtime_error(&"Module test failed".to_string()));
    }
    println!("⚙️  Module processing (error) initialized");
    Ok(())
}

/// Test error functionality
pub fn test_error() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("Hello, CURSED!")?;
    if !result.contains("Hello, CURSED!") {
        return Err(CursedError::runtime_error(&"Module test failed".to_string()));
    }
    Ok(())
}
