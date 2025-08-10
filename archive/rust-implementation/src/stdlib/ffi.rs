/// FFI Bridge Module for CURSED
/// 
/// This module provides the necessary Foreign Function Interface (FFI) 
/// bridges for CURSED stdlib modules to interact with Rust runtime functions.

use crate::error::CursedError;
use std::ffi::{CStr, CString};

/// FFI interface for file operations
pub struct FFIBridge;

impl FFIBridge {
    pub fn new() -> Self {
        Self
    }
    
    /// Bridge function for file handle operations
    pub fn create_file_handle() -> Result<i32, CursedError> {
        // Simplified implementation - returns a mock file handle
        Ok(1)
    }
    
    /// Bridge function for buffer operations  
    pub fn create_buffer(size: usize) -> Result<Vec<u8>, CursedError> {
        Ok(vec![0; size])
    }
    
    /// Bridge function for path operations
    pub fn join_path(path1: &str, path2: &str) -> Result<String, CursedError> {
        Ok(format!("{}/{}", path1, path2))
    }
}

/// Error types for FFI operations
#[derive(Debug, Clone)]
pub enum FFIError {
    InvalidHandle,
    BufferOverflow,
    PathError,
    GeneralError(String),
}

impl std::fmt::Display for FFIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FFIError::InvalidHandle => write!(f, "Invalid file handle"),
            FFIError::BufferOverflow => write!(f, "Buffer overflow"),
            FFIError::PathError => write!(f, "Path operation error"),
            FFIError::GeneralError(msg) => write!(f, "FFI error: {}", msg),
        }
    }
}

impl std::error::Error for FFIError {}

impl From<FFIError> for CursedError {
    fn from(err: FFIError) -> Self {
        CursedError::General(err.to_string())
    }
}
