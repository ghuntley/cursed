//! Simplified standard library error handling utilities for CURSED
//!
//! This module provides basic error constructors and utilities that integrate
//! with the CURSED error handling system without type conflicts.

use crate::error::{CursedError, SourceLocation};
use crate::error::types::{
    ErrorManager, ErrorManagerConfig, ErrorCategory, ErrorSeverity
};
use std::sync::{Arc, OnceLock};

/// Global error manager instance
static GLOBAL_ERROR_MANAGER: OnceLock<Arc<ErrorManager>> = OnceLock::new();

/// Initialize the global error manager
pub fn init_error_system() -> std::result::Result<(), CursedError> {
    let config = ErrorManagerConfig {
        max_error_chains: 10000,
        auto_cleanup: true,
        severity_threshold: ErrorSeverity::Warning,
        enable_monitoring: true,
        enable_colored_output: true,
    };

    let manager = Arc::new(ErrorManager::with_config(config));
    
    GLOBAL_ERROR_MANAGER.set(manager)
        .map_err(|_| CursedError::system_error("Error manager already initialized"))?;

    Ok(())
}

/// Get the global error manager
pub fn get_error_manager() -> std::result::Result<Arc<ErrorManager>, CursedError> {
    GLOBAL_ERROR_MANAGER.get()
        .cloned()
        .ok_or_else(|| CursedError::system_error("Error manager not initialized"))
}

/// Standard error constructors for common scenarios
pub mod std_errors {
    use super::*;

    /// File system errors
    pub fn file_not_found(path: &str) -> std::result::Result<(), CursedError> {
        Err(CursedError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", path)
        )))
    }

    pub fn permission_denied(path: &str) -> std::result::Result<(), CursedError> {
        Err(CursedError::Io(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("Permission denied: {}", path)
        )))
    }

    /// Parsing errors
    pub fn syntax_error(message: &str, line: usize, column: usize) -> std::result::Result<(), CursedError> {
        Err(CursedError::parse_error_with_location(
            message.to_string(),
            line,
            column
        ))
    }

    /// Runtime errors
    pub fn division_by_zero(line: usize, column: usize) -> std::result::Result<(), CursedError> {
        Err(CursedError::parse_error_with_location(
            "Division by zero".to_string(),
            line,
            column
        ))
    }

    pub fn type_mismatch(expected: &str, actual: &str) -> std::result::Result<(), CursedError> {
        Err(CursedError::Type(format!("Type mismatch: expected {}, got {}", expected, actual)))
    }
}

/// Error recovery utilities
pub mod recovery {
    use super::*;

    /// Retry an operation with exponential backoff
    pub fn retry_with_backoff<T, F>(
        mut operation: F,
        max_attempts: usize,
        base_delay_ms: u64,
    ) -> std::result::Result<T, CursedError>
    where
        F: FnMut() -> std::result::Result<T, CursedError>,
    {
        let mut attempts = 0;
        let mut delay = base_delay_ms;

        loop {
            attempts += 1;
            
            match operation() {
                Ok(result) => return Ok(result),
                Err(error) => {
                    if attempts >= max_attempts {
                        return Err(error);
                    }
                    
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
        F: FnOnce() -> std::result::Result<T, CursedError>,
    {
        match operation() {
            Ok(value) => std::option::Option::Some(value),
            Err(_) => std::option::Option::None,
        }
    }

    /// Try an operation and return default on error
    pub fn try_or_default<T, F>(operation: F, default: T) -> T
    where
        F: FnOnce() -> std::result::Result<T, CursedError>,
    {
        match operation() {
            Ok(value) => value,
            Err(_) => default,
        }
    }
}

/// Basic error formatting
pub struct ErrorFormatter {
    pub use_colors: bool,
}

impl ErrorFormatter {
    pub fn new() -> Self {
        Self { use_colors: true }
    }

    pub fn format_error(&self, error: &CursedError) -> String {
        if self.use_colors {
            format!("\x1b[31mError:\x1b[0m {}", error)
        } else {
            format!("Error: {}", error)
        }
    }
}

impl Default for ErrorFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_std_errors() {
        let result = std_errors::file_not_found("/nonexistent/file.txt");
        assert!(result.is_err());

        let result = std_errors::division_by_zero(10, 5);
        assert!(result.is_err());

        let result = std_errors::type_mismatch("String", "Number");
        assert!(result.is_err());
    }

    #[test]
    fn test_error_formatter() {
        let formatter = ErrorFormatter::new();
        let error = CursedError::Runtime("Test error".to_string());
        let formatted = formatter.format_error(&error);
        
        assert!(formatted.contains("Test error"));
    }

    #[test]
    fn test_recovery_utilities() {
        let mut attempt_count = 0;
        let result = recovery::retry_with_backoff(
            || {
                attempt_count += 1;
                if attempt_count < 3 {
                    Err(CursedError::Runtime("Test error".to_string()))
                } else {
                    Ok(42)
                }
            },
            5,
            1
        );

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempt_count, 3);

        let option_result = recovery::try_or_none(|| Err::<i32, CursedError>(CursedError::Runtime("Test error".to_string())));
        assert!(option_result.is_none());

        let default_result = recovery::try_or_default(|| Err::<&str, CursedError>(CursedError::Runtime("Test error".to_string())), "default");
        assert_eq!(default_result, "default");
    }
}
