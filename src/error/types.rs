// Error type definitions for the CURSED error system
// This module provides detailed error types and management functionality

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt;

/// Source location information
#[derive(Debug, Clone)]
pub struct SourceLocation {
impl SourceLocation {
    pub fn new(file: std::path::PathBuf, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }
impl Default for SourceLocation {
    fn default() -> Self {
        Self {
        }
    }
impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.column)
    }
}

/// Error context information
#[derive(Debug, Clone)]
pub struct ErrorContext {
impl ErrorContext {
    pub fn new(message: String) -> Self {
        Self {
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    pub fn add_context(mut self, context: String) -> Self {
        self.context_stack.push(context);
        self
    }
}

// impl fmt::Display for ErrorContext {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if let Some(location) = &self.location {
//             write!(f, "{}: {}", location, self.message)
//         } else {
//             write!(f, "{}", self.message)
//         }
//     }
// }

/// Trait for CURSED error types
pub trait CursedErrorTrait: fmt::Debug + fmt::Display + Send + Sync {
    fn error_code(&self) -> u32;
    fn category(&self) -> ErrorCategory;
    fn severity(&self) -> ErrorSeverity;
/// Error categories for classification
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCategory {
/// Error severity levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ErrorSeverity {
/// Parse error type
#[derive(Debug, Clone)]
pub struct ParseError {
// impl fmt::Display for ParseError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Parse error at {}:{}: {}", self.line, self.column, self.message)
//     }
// }

// impl CursedErrorTrait for ParseError {
//     fn error_code(&self) -> u32 { 1001 }
//     fn category(&self) -> ErrorCategory { ErrorCategory::Syntax }
//     fn severity(&self) -> ErrorSeverity { ErrorSeverity::Error }
// }

/// Runtime error type
#[derive(Debug, Clone)]
pub struct RuntimeError {
// impl fmt::Display for RuntimeError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Runtime error: {}", self.message)
//     }
// }

// impl CursedErrorTrait for RuntimeError {
//     fn error_code(&self) -> u32 { 2001 }
//     fn category(&self) -> ErrorCategory { ErrorCategory::Runtime }
//     fn severity(&self) -> ErrorSeverity { ErrorSeverity::Error }
// }

/// IO error type
#[derive(Debug, Clone)]
pub struct IoError {
// impl fmt::Display for IoError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if let Some(path) = &self.path {
//             write!(f, "IO error on '{}': {}", path, self.message)
//         } else {
//             write!(f, "IO error: {}", self.message)
//         }
//     }
// }

// impl CursedErrorTrait for IoError {
//     fn error_code(&self) -> u32 { 3001 }
//     fn category(&self) -> ErrorCategory { ErrorCategory::IO }
//     fn severity(&self) -> ErrorSeverity { ErrorSeverity::Error }
// }

/// Error manager configuration
#[derive(Debug, Clone)]
pub struct ErrorManagerConfig {
impl Default for ErrorManagerConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Error manager for collecting and reporting errors
pub struct ErrorManager {
impl ErrorManager {
    pub fn new(config: ErrorManagerConfig) -> Self {
        Self {
        }
    }

    pub fn add_error(&self, error: Box<dyn CursedErrorTrait>) {
        let category = error.category();
        
        if let Ok(mut errors) = self.errors.lock() {
            if errors.len() < self.config.max_errors {
                errors.push(error);
            }
        }

        if let Ok(mut counts) = self.error_counts.lock() {
            *counts.entry(category).or_insert(0) += 1;
        }
    }

    pub fn error_count(&self) -> usize {
        self.errors.lock().map(|e| e.len()).unwrap_or(0)
    pub fn clear(&self) {
        if let Ok(mut errors) = self.errors.lock() {
            errors.clear();
        }
        if let Ok(mut counts) = self.error_counts.lock() {
            counts.clear();
        }
    }
impl Default for ErrorManager {
    fn default() -> Self {
        Self::new(ErrorManagerConfig::default())
    }
}
