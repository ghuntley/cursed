// Memory management error types

use std::fmt;
use crate::error::Error;

/// Errors that can occur during memory operations
#[derive(Debug, Clone)]
pub enum MemoryError {
    /// Allocation failed
    AllocationFailed(String),
    
    /// Invalid memory access
    InvalidAccess(String),
    
    /// Out of memory
    OutOfMemory(String),
    
    /// Garbage collection error
    GCError(String),
    
    /// General memory error
    Other(String),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::AllocationFailed(msg) => write!(f, "Allocation failed: {}", msg),
            MemoryError::InvalidAccess(msg) => write!(f, "Invalid memory access: {}", msg),
            MemoryError::OutOfMemory(msg) => write!(f, "Out of memory: {}", msg),
            MemoryError::GCError(msg) => write!(f, "Garbage collection error: {}", msg),
            MemoryError::Other(msg) => write!(f, "Memory error: {}", msg),
        }
    }
}

impl From<MemoryError> for Error {
    fn from(err: MemoryError) -> Self {
        match err {
            MemoryError::AllocationFailed(msg) => Error::Memory(msg),
            MemoryError::InvalidAccess(msg) => Error::Memory(msg),
            MemoryError::OutOfMemory(msg) => Error::Memory(msg),
            MemoryError::GCError(msg) => Error::Memory(msg),
            MemoryError::Other(msg) => Error::Memory(msg),
        }
    }
}

impl From<Error> for MemoryError {
    fn from(err: Error) -> Self {
        match err {
            Error::Memory(msg) => MemoryError::Other(msg),
            _ => MemoryError::Other(format!("{:?}", err)),
        }
    }
}
