//! Functional implementation for error

use crate::error::CursedError;
use crate::stdlib::packages::ModuleError;

/// Result type for error operations
pub type ModuleResult<T> = Result<T, CursedError>;

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

#[derive(Debug, Clone)]
pub enum DatabaseErrorKind {
    ConnectionFailed,
    QueryFailed,
    TransactionFailed,
    InvalidSchema,
    ConstraintViolation,
    Timeout,
    PermissionDenied,
    Other(String),
}

impl std::fmt::Display for DatabaseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseErrorKind::ConnectionFailed => write!(f, "Connection failed"),
            DatabaseErrorKind::QueryFailed => write!(f, "Query failed"),
            DatabaseErrorKind::TransactionFailed => write!(f, "Transaction failed"),
            DatabaseErrorKind::InvalidSchema => write!(f, "Invalid schema"),
            DatabaseErrorKind::ConstraintViolation => write!(f, "Constraint violation"),
            DatabaseErrorKind::Timeout => write!(f, "Timeout"),
            DatabaseErrorKind::PermissionDenied => write!(f, "Permission denied"),
            DatabaseErrorKind::Other(msg) => write!(f, "Other: {}", msg),
        }
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Database-specific error types
#[derive(Debug, Clone)]
pub enum DatabaseError {
    Driver(String),
    Connection(String),
    Query(String),
    Transaction(String),
    Migration(String),
    Parse(String),
    Timeout(String),
    Auth(String),
    Mapping(String),
}

impl DatabaseError {
    pub fn driver(msg: &str) -> Self {
        DatabaseError::Driver(msg.to_string())
    }
    
    pub fn connection(msg: &str) -> Self {
        DatabaseError::Connection(msg.to_string())
    }
    
    pub fn query(msg: &str) -> Self {
        DatabaseError::Query(msg.to_string())
    }
    
    pub fn transaction(msg: &str) -> Self {
        DatabaseError::Transaction(msg.to_string())
    }
    
    pub fn migration(msg: &str) -> Self {
        DatabaseError::Migration(msg.to_string())
    }
    
    pub fn mapping(msg: &str) -> Self {
        DatabaseError::Mapping(msg.to_string())
    }
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::Driver(msg) => write!(f, "Database driver error: {}", msg),
            DatabaseError::Connection(msg) => write!(f, "Database connection error: {}", msg),
            DatabaseError::Query(msg) => write!(f, "Database query error: {}", msg),
            DatabaseError::Transaction(msg) => write!(f, "Database transaction error: {}", msg),
            DatabaseError::Migration(msg) => write!(f, "Database migration error: {}", msg),
            DatabaseError::Parse(msg) => write!(f, "Database parse error: {}", msg),
            DatabaseError::Timeout(msg) => write!(f, "Database timeout error: {}", msg),
            DatabaseError::Auth(msg) => write!(f, "Database auth error: {}", msg),
            DatabaseError::Mapping(msg) => write!(f, "Database mapping error: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}

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
