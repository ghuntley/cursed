// Common module for CURSED
use thiserror::Error;

// Core modules
pub mod optimization_level;

// Re-export optimization level from dedicated module
pub use optimization_level::OptimizationLevel;

// Basic error type for minimal build
#[derive(Error, Debug, Clone)]
pub enum MinimalError {
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
}

// Use minimal error as Error for now
pub use MinimalError as Error;

pub type Result<T> = std::result::Result<T, Error>;
