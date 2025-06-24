// Minimal error module for CURSED minimal build

use thiserror::Error;

// Basic source location for minimal build
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl Default for SourceLocation {
    fn default() -> Self {
        SourceLocation {
            file: "<unknown>".to_string(),
            line: 0,
            column: 0,
        }
    }
}

// Basic error type for minimal build
#[derive(Error, Debug, Clone)]
pub enum CursedError {
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Codegen error: {0}")]  
    Codegen(String),
    
    #[error("Runtime error: {0}")]
    Runtime(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    
    #[error("IO error: {0}")]
    Io(String),
    
    #[error("Type error: {0}")]
    Type(String),
    
    #[error("Memory error: {0}")]
    Memory(String),
    
    #[error("Generic error: {0}")]
    Generic(String),
}

impl From<std::io::Error> for CursedError {
    fn from(err: std::io::Error) -> Self {
        CursedError::Io(err.to_string())
    }
}

impl From<String> for CursedError {
    fn from(err: String) -> Self {
        CursedError::Generic(err)
    }
}

// Type alias for backwards compatibility and easier imports
pub type Error = CursedError;

// Unified Result type for consistent error handling
pub type Result<T> = std::result::Result<T, CursedError>;

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Generic(err.to_string())
    }
}

// Re-export submodules that exist
pub mod debug_context;
pub mod error_propagation;
pub mod types;
