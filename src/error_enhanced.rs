//! Enhanced error handling for the CURSED programming language
//!
//! This module provides an improved error handling system with:
//! - Structured error types with context information
//! - Stack trace capture
//! - Error wrapping (similar to Go's errors package)
//! - Source location tracking
//! - Error testing utilities

use std::backtrace::{Backtrace, BacktraceStatus};
use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;
use std::io;

use crate::error::SourceLocation;

/// Core error type for CURSED that supports wrapping and context
#[derive(Debug)]
pub struct CursedError {
    /// The error kind (type)
    kind: ErrorKind,
    /// The error message
    message: String,
    /// Optional source location information
    location: Option<SourceLocation>,
    /// Optional cause (wrapped error)
    cause: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    /// Optional context information (key-value pairs)
    context: Vec<(String, String)>,
    /// Optional stack trace at the point of error creation
    stack_trace: Option<Backtrace>,
    /// Optional error code for categorization
    code: Option<String>,
}

/// Kind of error that occurred
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Lexer,
    Parser,
    Type,
    Semantic,
    Compilation,
    Runtime,
    Memory,
    Bytecode,
    VM,
    Index,
    DivisionByZero,
    StackOverflow,
    HeapOverflow,
    InvalidOperation,
    System,
    IO,
    Unknown,
    NotImplemented,
    Configuration,
    CodeGen,
    Syntax,
    TypeAssertion,
    InvalidArguments,
    NotFound,
    Parsing,
    Internal,
    Validation,
}

impl ErrorKind {
    /// Get a string representation of the error kind
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::Lexer => "Lexer",
            ErrorKind::Parser => "Parser",
            ErrorKind::Type => "Type",
            ErrorKind::Semantic => "Semantic",
            ErrorKind::Compilation => "Compilation",
            ErrorKind::Runtime => "Runtime",
            ErrorKind::Memory => "Memory",
            ErrorKind::Bytecode => "Bytecode",
            ErrorKind::VM => "VM",
            ErrorKind::Index => "Index",
            ErrorKind::DivisionByZero => "DivisionByZero",
            ErrorKind::StackOverflow => "StackOverflow",
            ErrorKind::HeapOverflow => "HeapOverflow",
            ErrorKind::InvalidOperation => "InvalidOperation",
            ErrorKind::System => "System",
            ErrorKind::IO => "IO",
            ErrorKind::Unknown => "Unknown",
            ErrorKind::NotImplemented => "NotImplemented",
            ErrorKind::Configuration => "Configuration",
            ErrorKind::CodeGen => "CodeGen",
            ErrorKind::Syntax => "Syntax",
            ErrorKind::TypeAssertion => "TypeAssertion",
            ErrorKind::InvalidArguments => "InvalidArguments",
            ErrorKind::NotFound => "NotFound",
            ErrorKind::Parsing => "Parsing",
            ErrorKind::Internal => "Internal",
            ErrorKind::Validation => "Validation",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PartialEq for CursedError {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind 
            && self.message == other.message 
            && self.location == other.location
            && self.context == other.context
            && self.code == other.code
            // Note: We don't compare cause (trait object) or stack_trace (Backtrace)
            // as they don't implement PartialEq
    }
}

impl CursedError {
    /// Create a new error with the given kind and message
    pub fn new<S: Into<String>>(kind: ErrorKind, message: S) -> Self {
        Self {
            kind,
            message: message.into(),
            location: None,
            cause: None,
            context: Vec::new(),
            stack_trace: Some(Backtrace::capture()),
            code: None,
        }
    }

    /// Add a source location to the error
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// Wrap another error as the cause of this error
    pub fn with_cause<E>(mut self, cause: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.cause = Some(Arc::new(cause));
        self
    }

    /// Add context information to the error
    pub fn with_context<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.context.push((key.into(), value.into()));
        self
    }

    /// Add an error code to the error
    pub fn with_code<S: Into<String>>(mut self, code: S) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Get the error kind
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the source location if available
    pub fn location(&self) -> Option<&SourceLocation> {
        self.location.as_ref()
    }

    /// Get the cause if available - use source() instead for compatibility with std::error::Error
    pub fn cause(&self) -> Option<&(dyn StdError + Send + Sync + 'static)> {
        self.cause.as_ref().map(|c| c.as_ref() as &(dyn StdError + Send + Sync + 'static))
    }

    /// Get the context information
    pub fn context(&self) -> &[(String, String)] {
        &self.context
    }

    /// Get the stack trace if available
    pub fn stack_trace(&self) -> Option<&Backtrace> {
        self.stack_trace.as_ref()
    }

    /// Get the error code if available
    pub fn code(&self) -> Option<&str> {
        self.code.as_ref().map(|s| s.as_str())
    }

    /// Check if this error is of the given kind or wraps an error of the given kind
    pub fn is_kind(&self, kind: &ErrorKind) -> bool {
        if &self.kind == kind {
            return true;
        }

        // Check if any wrapped error is of this kind
        if let Some(cause) = &self.cause {
            if let Some(cursed_err) = cause.downcast_ref::<CursedError>() {
                return cursed_err.is_kind(kind);
            }
        }

        false
    }

    /// Get a formatted error message including context information
    pub fn full_message(&self) -> String {
        let mut result = String::new();

        // Start with error kind and code if available
        if let Some(code) = &self.code {
            result.push_str(&format!("[{}/{}] ", self.kind, code));
        } else {
            result.push_str(&format!("[{}] ", self.kind));
        }

        // Add the main message
        result.push_str(&self.message);

        // Add location if available
        if let Some(loc) = &self.location {
            result.push_str(&format!(" (at {})", loc));
        }

        // Add context information
        if !self.context.is_empty() {
            result.push_str("\nContext:");
            for (key, value) in &self.context {
                result.push_str(&format!("\n  {}: {}", key, value));
            }
        }

        // Add cause if available
        if let Some(cause) = &self.cause {
            result.push_str(&format!("\nCaused by: {}", cause));
        }

        result
    }

    /// Format the stack trace as a string if available
    pub fn format_stack_trace(&self) -> Option<String> {
        self.stack_trace.as_ref().map(|bt| {
            if bt.status() == BacktraceStatus::Captured {
                format!("{:?}", bt)
            } else {
                "Stack trace not captured. Set RUST_BACKTRACE=1 to enable.".to_string()
            }
        })
    }

    /// Create a lexer error
    pub fn lexer<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::Lexer, message)
    }

    /// Create a parser error
    pub fn parser<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::Parser, message)
    }

    /// Create a type error
    pub fn type_error<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::Type, message)
    }

    /// Create a semantic error
    pub fn semantic<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::Semantic, message)
    }

    /// Create a compilation error
    pub fn compilation<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::Compilation, message)
    }

    /// Create a runtime error
    pub fn runtime<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::Runtime, message)
    }

    /// Create a memory error
    pub fn memory<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::Memory, message)
    }

    /// Create a bytecode error
    pub fn bytecode<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::Bytecode, message)
    }

    /// Create a VM error
    pub fn vm<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::VM, message)
    }

    /// Create an index error
    pub fn index<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::Index, message)
    }

    /// Create a division by zero error
    pub fn division_by_zero<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::DivisionByZero, message)
    }

    /// Create a stack overflow error
    pub fn stack_overflow<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::StackOverflow, message)
    }

    /// Create a heap overflow error
    pub fn heap_overflow<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::HeapOverflow, message)
    }

    /// Create an invalid operation error
    pub fn invalid_operation<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::InvalidOperation, message)
    }

    /// Create a system error
    pub fn system<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::System, message)
    }

    /// Create a not implemented error
    pub fn not_implemented<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::NotImplemented, message)
    }

    /// Create a code generation error
    pub fn codegen<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::CodeGen, message)
    }

    /// Create a syntax error
    pub fn syntax<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::Syntax, message)
    }

    /// Create an IO error
    pub fn io<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::IO, message)
    }

    /// Create an unknown error
    pub fn unknown<S: Into<String>>(message: S) -> Self {
        Self::new(ErrorKind::Unknown, message)
    }
}

impl fmt::Display for CursedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_message())
    }
}

impl StdError for CursedError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.cause.as_ref().map(|c| c.as_ref() as &(dyn StdError + 'static))
    }
}

// Convert from regular std::io::Error
impl From<io::Error> for CursedError {
    fn from(err: io::Error) -> Self {
        Self::new(ErrorKind::IO, format!("IO error: {}", err))
            .with_cause(err)
    }
}

// Convert from string slice
impl From<&str> for CursedError {
    fn from(message: &str) -> Self {
        Self::new(ErrorKind::Unknown, message)
    }
}

// Convert from String
impl From<String> for CursedError {
    fn from(message: String) -> Self {
        Self::new(ErrorKind::Unknown, message)
    }
}

/// Manual Clone implementation for CursedError
impl Clone for CursedError {
    fn clone(&self) -> Self {
        let cloned = Self {
            kind: self.kind.clone(),
            message: self.message.clone(),
            location: self.location.clone(),
            cause: self.cause.clone(),
            context: self.context.clone(),
            // Skip stack trace in clone - it's not cloneable
            stack_trace: None,
            code: self.code.clone(),
        };
        cloned
    }
}

// Convert from legacy Error type
impl From<crate::error::Error> for CursedError {
    fn from(err: crate::error::Error) -> Self {
        match err {
            crate::error::Error::IoError(io_err) => Self::from(io_err),
            crate::error::Error::Lexer { location, message } => {
                Self::new(ErrorKind::Lexer, message).with_location(location)
            }
            crate::error::Error::Parser { location, message } => {
                Self::new(ErrorKind::Parser, message).with_location(location)
            }
            crate::error::Error::Type { location, message } => {
                Self::new(ErrorKind::Type, message).with_location(location)
            }
            crate::error::Error::Syntax { location, message } => {
                Self::new(ErrorKind::Syntax, message).with_location(location)
            }
            crate::error::Error::SemanticError(msg) => Self::new(ErrorKind::Semantic, msg),
            crate::error::Error::Compilation(msg) => Self::new(ErrorKind::Compilation, msg),
            crate::error::Error::Runtime(msg) => Self::new(ErrorKind::Runtime, msg),
            crate::error::Error::Memory(msg) => Self::new(ErrorKind::Memory, msg),
            crate::error::Error::BytecodeError(msg) => Self::new(ErrorKind::Bytecode, msg),
            crate::error::Error::VMError(msg) => Self::new(ErrorKind::VM, msg),
            crate::error::Error::Unknown(msg) => Self::new(ErrorKind::Unknown, msg),
            crate::error::Error::IndexError(msg) => Self::new(ErrorKind::Index, msg),
            crate::error::Error::DivisionByZero(msg) => Self::new(ErrorKind::DivisionByZero, msg),
            crate::error::Error::StackOverflow(msg) => Self::new(ErrorKind::StackOverflow, msg),
            crate::error::Error::HeapOverflow(msg) => Self::new(ErrorKind::HeapOverflow, msg),
            crate::error::Error::InvalidOperation(msg) => Self::new(ErrorKind::InvalidOperation, msg),
            crate::error::Error::SystemError(msg) => Self::new(ErrorKind::System, msg),
            crate::error::Error::NotImplemented { message } => Self::new(ErrorKind::NotImplemented, message),
            crate::error::Error::CodeGenError(msg) => Self::new(ErrorKind::CodeGen, msg),
            crate::error::Error::TypeAssertion(error) => error,
            crate::error::Error::InvalidArguments(msg) => Self::new(ErrorKind::InvalidArguments, msg),
            crate::error::Error::NotFound(msg) => Self::new(ErrorKind::NotFound, msg),
            crate::error::Error::Parsing(msg) => Self::new(ErrorKind::Parsing, msg),
            crate::error::Error::IO(io_err) => Self::from(io_err),
            crate::error::Error::Internal(msg) => Self::new(ErrorKind::Internal, msg),
            crate::error::Error::Validation(msg) => Self::new(ErrorKind::Validation, msg),
            crate::error::Error::PackageNotFound(pkg) => Self::new(ErrorKind::NotFound, format!("Package not found: {}", pkg)),
            crate::error::Error::SymbolNotFound(symbol, package) => Self::new(ErrorKind::NotFound, format!("Symbol '{}' not found in package '{}'", symbol, package)),
            crate::error::Error::SymbolNotExported(symbol, package) => Self::new(ErrorKind::NotFound, format!("Symbol '{}' not exported from package '{}'", symbol, package)),
            crate::error::Error::CircularDependency(deps) => Self::new(ErrorKind::Semantic, format!("Circular dependency: {}", deps.join(" -> "))),
            crate::error::Error::Configuration(msg) => Self::new(ErrorKind::Configuration, msg),
        }
    }
}

/// Error testing utilities for verifying errors in tests
pub mod test_utils {
    use super::*;

    /// Assert that an error is of a specific kind
    pub fn assert_error_kind<E: Into<CursedError>>(err: E, expected_kind: ErrorKind) {
        let err = err.into();
        assert!(
            err.is_kind(&expected_kind),
            "Expected error of kind {:?}, but got {:?}",
            expected_kind,
            err.kind()
        );
    }

    /// Assert that an error message contains a specific substring
    pub fn assert_error_message_contains<E: Into<CursedError>, S: AsRef<str>>(
        err: E,
        substring: S,
    ) {
        let err = err.into();
        let message = err.message();
        assert!(
            message.contains(substring.as_ref()),
            "Expected error message to contain '{}', but got '{}'",
            substring.as_ref(),
            message
        );
    }

    /// Assert that an error has a specific location
    pub fn assert_error_location<E: Into<CursedError>>(
        err: E,
        expected_line: usize,
        expected_column: usize,
    ) {
        let err = err.into();
        if let Some(location) = err.location() {
            assert_eq!(
                location.line, expected_line,
                "Expected error at line {}, but got {}",
                expected_line, location.line
            );
            assert_eq!(
                location.column, expected_column,
                "Expected error at column {}, but got {}",
                expected_column, location.column
            );
        } else {
            panic!("Expected error with location, but got none");
        }
    }

    /// Create a test error for testing error handling code
    pub fn create_test_error(kind: ErrorKind, message: &str) -> CursedError {
        CursedError::new(kind, message)
            .with_context("test", "true")
            .with_code("TEST-001")
    }
}

/// A Result type that uses CursedError
pub type Result<T> = std::result::Result<T, CursedError>;