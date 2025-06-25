// Simplified standard library error handling utilities for CURSED
//
// This module provides basic error constructors and utilities that integrate
// with the CURSED error handling system without type conflicts.

use crate::error::SourceLocation;
use crate::error::CursedError;

// Re-export CursedError for use by other stdlib modules
// (Commented out to avoid duplicate import)
use crate::error::{
    ErrorManager, ErrorManagerConfig, ErrorCategory, ErrorSeverity
// };
use std::sync::{Arc, OnceLock};

/// Global error manager instance
static GLOBAL_ERROR_MANAGER: OnceLock<Arc<ErrorManager>> = OnceLock::new();

/// Initialize the global error manager
pub fn init_error_system() -> std::result::crate::error::Result<()> {
    let config = ErrorManagerConfig {

    let manager = Arc::new(ErrorManager::with_config(config));
    
    GLOBAL_ERROR_MANAGER.set(manager)
        .map_err(|_| CursedError::system_error("CursedError manager already initialized"))?;

    Ok(())
/// Get the global error manager
pub fn get_error_manager() -> std::result::crate::error::Result<()> {
    GLOBAL_ERROR_MANAGER.get()
        .cloned()
        .ok_or_else(|| CursedError::system_error("CursedError manager not initialized"))
/// Standard error constructors for common scenarios
pub mod std_errors {
    use super::*;

    /// File system errors
    pub fn file_not_found(path: &str) -> std::result::crate::error::Result<()> {
        Err(CursedError::Io(std::io::Error::new(
            format!("File not found: {}", path)
        )))
    pub fn permission_denied(path: &str) -> std::result::crate::error::Result<()> {
        Err(CursedError::Io(std::io::Error::new(
            format!("Permission denied: {}", path)
        )))
    /// Parsing errors
    pub fn syntax_error(message: &str, line: usize, column: usize) -> std::result::crate::error::Result<()> {
        Err(CursedError::parse_error_with_location(
            column
        ))
    /// Runtime errors
    pub fn division_by_zero(line: usize, column: usize) -> std::result::crate::error::Result<()> {
        Err(CursedError::parse_error_with_location(
            column
        ))
    pub fn type_mismatch(expected: &str, actual: &str) -> std::result::crate::error::Result<()> {
        Err(CursedError::Type(format!("Type mismatch: expected {}, got {}", expected, actual)))
    }
}

/// CursedError recovery utilities
pub mod recovery {
    use super::*;

    /// Retry an operation with exponential backoff
    pub fn retry_with_backoff<T, F>(
    ) -> std::result::crate::error::Result<()>
    where
    {
        let mut attempts = 0;
        let mut delay = base_delay_ms;

        loop {
            attempts += 1;
            
            match operation() {
                Err(error) => {
                    if attempts >= max_attempts {
                        return Err(error);
                    // Exponential backoff
                    std::thread::sleep(std::time::Duration::from_millis(delay));
                    delay *= 2;
                }
            }
        }
    }

    /// Try an operation and return Option instead of Result
    pub fn try_or_none<T, F>(operation: F) -> std::option::Option<T>
    where
    {
        match operation() {
        }
    }

    /// Try an operation and return default on error
    pub fn try_or_default<T, F>(operation: F, default: T) -> T
    where
    {
        match operation() {
        }
    }
/// Basic error formatting
pub struct ErrorFormatter {
impl ErrorFormatter {
    pub fn new() -> Self {
        Self { use_colors: true }
    }

    pub fn format_error(&self, error: &CursedError) -> String {
        if self.use_colors {
            format!("\x1b[31mError:\x1b[0m {}", error)
        } else {
            format!("CursedError: {}", error)
        }
    }
impl Default for ErrorFormatter {
    fn default() -> Self {
        Self::new()
    }
}

