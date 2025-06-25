// Error type definitions for the CURSED error system
// This module provides detailed error types and management functionality

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt;

/// Source location information
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: std::path::PathBuf,
    pub line: u32,
    pub column: u32,
}

impl SourceLocation {
    pub fn new(file: std::path::PathBuf, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            file: std::path::PathBuf::from("<unknown>"),
            line: 0,
            column: 0,
        }
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
    pub location: Option<SourceLocation>,
    pub message: String,
    pub context_stack: Vec<String>,
}

impl ErrorContext {
    pub fn new(message: String) -> Self {
        Self {
            location: None,
            message,
            context_stack: Vec::new(),
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    pub fn add_context(mut self, context: String) -> Self {
        self.context_stack.push(context);
        self
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(location) = &self.location {
            write!(f, "{}: {}", location, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

/// Trait for CURSED error types
pub trait CursedErrorTrait: fmt::Debug + fmt::Display + Send + Sync {
    fn error_code(&self) -> u32;
    fn category(&self) -> ErrorCategory;
    fn severity(&self) -> ErrorSeverity;
}

/// Error categories for classification
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCategory {
    Syntax,
    Runtime,
    IO,
    Network,
    Database,
    Crypto,
    Memory,
    System,
    User,
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
    Fatal,
}

/// Parse error type
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at {}:{}: {}", self.line, self.column, self.message)
    }
}

impl CursedErrorTrait for ParseError {
    fn error_code(&self) -> u32 { 1001 }
    fn category(&self) -> ErrorCategory { ErrorCategory::Syntax }
    fn severity(&self) -> ErrorSeverity { ErrorSeverity::Error }
}

/// Runtime error type
#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub stack_trace: Option<Vec<String>>,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime error: {}", self.message)
    }
}

impl CursedErrorTrait for RuntimeError {
    fn error_code(&self) -> u32 { 2001 }
    fn category(&self) -> ErrorCategory { ErrorCategory::Runtime }
    fn severity(&self) -> ErrorSeverity { ErrorSeverity::Error }
}

/// IO error type
#[derive(Debug, Clone)]
pub struct IoError {
    pub message: String,
    pub path: Option<String>,
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(path) = &self.path {
            write!(f, "IO error on '{}': {}", path, self.message)
        } else {
            write!(f, "IO error: {}", self.message)
        }
    }
}

impl CursedErrorTrait for IoError {
    fn error_code(&self) -> u32 { 3001 }
    fn category(&self) -> ErrorCategory { ErrorCategory::IO }
    fn severity(&self) -> ErrorSeverity { ErrorSeverity::Error }
}

/// Error manager configuration
#[derive(Debug, Clone)]
pub struct ErrorManagerConfig {
    pub max_errors: usize,
    pub enable_stack_traces: bool,
    pub log_errors: bool,
}

impl Default for ErrorManagerConfig {
    fn default() -> Self {
        Self {
            max_errors: 1000,
            enable_stack_traces: true,
            log_errors: true,
        }
    }
}

/// Error manager for collecting and reporting errors
pub struct ErrorManager {
    config: ErrorManagerConfig,
    errors: Arc<Mutex<Vec<Box<dyn CursedErrorTrait>>>>,
    error_counts: Arc<Mutex<HashMap<ErrorCategory, usize>>>,
}

impl ErrorManager {
    pub fn new(config: ErrorManagerConfig) -> Self {
        Self {
            config,
            errors: Arc::new(Mutex::new(Vec::new())),
            error_counts: Arc::new(Mutex::new(HashMap::new())),
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
    }

    pub fn clear(&self) {
        if let Ok(mut errors) = self.errors.lock() {
            errors.clear();
        }
        if let Ok(mut counts) = self.error_counts.lock() {
            counts.clear();
        }
    }
}

impl Default for ErrorManager {
    fn default() -> Self {
        Self::new(ErrorManagerConfig::default())
    }
}
