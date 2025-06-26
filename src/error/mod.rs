/// Error handling module for CURSED
/// 
/// This module provides comprehensive error handling for the CURSED language
/// implementation, including error types, debugging contexts, and recovery.

pub mod debug_context;

#[derive(Debug, Clone)]
pub enum CursedError {
    SyntaxError(String),
    TypeError(String),
    RuntimeError(String),
    ImportError(String),
    CompilerError(String),
    General(String),
    Io(String),
    Parse(String),
    InvalidOptimizationLevel(String),
    OptimizationError(String),
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum Error {
    Syntax(String),
    Type(String),
    Runtime(String),
    Import(String),
    Compiler(String),
    Io(String),
    General(String),
}

impl std::fmt::Display for CursedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursedError::SyntaxError(msg) => write!(f, "Syntax error: {}", msg),
            CursedError::TypeError(msg) => write!(f, "Type error: {}", msg),
            CursedError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            CursedError::ImportError(msg) => write!(f, "Import error: {}", msg),
            CursedError::CompilerError(msg) => write!(f, "Compiler error: {}", msg),
            CursedError::General(msg) => write!(f, "Error: {}", msg),
            CursedError::Io(msg) => write!(f, "IO error: {}", msg),
            CursedError::Parse(msg) => write!(f, "Parse error: {}", msg),
            CursedError::InvalidOptimizationLevel(msg) => write!(f, "Invalid optimization level: {}", msg),
            CursedError::OptimizationError(msg) => write!(f, "Optimization error: {}", msg),
        }
    }
}

impl std::error::Error for CursedError {}

pub type Result<T> = std::result::Result<T, CursedError>;

// Convenience constructor functions
impl CursedError {
    pub fn syntax_error(msg: &str) -> Self {
        CursedError::SyntaxError(msg.to_string())
    }
    
    pub fn type_error(msg: &str) -> Self {
        CursedError::TypeError(msg.to_string())
    }
    
    pub fn runtime_error(msg: &str) -> Self {
        CursedError::RuntimeError(msg.to_string())
    }
    
    pub fn import_error(msg: &str) -> Self {
        CursedError::ImportError(msg.to_string())
    }
    
    pub fn compiler_error(msg: &str) -> Self {
        CursedError::CompilerError(msg.to_string())
    }
    
    pub fn general_error(msg: &str) -> Self {
        CursedError::General(msg.to_string())
    }
    
    pub fn parse_error(msg: &str) -> Self {
        CursedError::Parse(msg.to_string())
    }
}

// Add From trait implementations for common conversions
impl From<std::io::Error> for CursedError {
    fn from(error: std::io::Error) -> Self {
        CursedError::Io(error.to_string())
    }
}

impl From<std::string::String> for CursedError {
    fn from(error: String) -> Self {
        CursedError::General(error)
    }
}

// Removed futures_io::Error conversion - use std::io::Error instead

impl From<crate::error_types::Error> for CursedError {
    fn from(error: crate::error_types::Error) -> Self {
        CursedError::General(error.to_string())
    }
}
