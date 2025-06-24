// Minimal common module
use thiserror::Error;
use crate::error::Error;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationLevel {
    O0,
    O1,
    O2,
    O3,
}

impl OptimizationLevel {
    pub fn to_llvm_level(&self) -> u32 {
        match self {
            OptimizationLevel::O0 => 0,
            OptimizationLevel::O1 => 1,
            OptimizationLevel::O2 => 2,
            OptimizationLevel::O3 => 3,
        }
    }
    
    pub fn from_string(s: &str) -> Self {
        match s {
            "O0" => OptimizationLevel::O0,
            "O1" => OptimizationLevel::O1, 
            "O2" => OptimizationLevel::O2,
            "O3" => OptimizationLevel::O3,
            _ => OptimizationLevel::O0,
        }
    }
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::O0
    }
}

pub type Result<T> = std::result::Result<T, Error>;
