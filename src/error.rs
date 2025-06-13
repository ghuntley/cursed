//! Error handling for CURSED
//! 
//! Centralized error types and handling for the CURSED programming language.

use std::fmt;

// Re-export error modules
pub mod error_propagation;
pub mod debug_context;
pub mod types;

/// Main error type for CURSED
#[derive(Debug)]
pub enum Error {
    /// I/O related errors
    Io(std::io::Error),
    /// Parsing errors
    Parse(String),
    /// Compilation errors
    Compile(String),
    /// Runtime errors
    Runtime(String),
    /// Package manager errors
    Package(String),
    /// REPL errors
    Repl(String),
    /// Template system errors
    TemplateError { 
        message: String,
        source_location: Option<SourceLocation>,
    },
    /// Type compilation errors
    TypeCompilation(String),
    /// Type system errors
    Type(String),
    /// Panic-related errors
    Panic {
        message: String,
        panic_id: Option<u64>,
        recoverable: bool,
        source_location: Option<SourceLocation>,
    },
    /// Recovery operation errors
    Recovery {
        message: String,
        recovery_attempts: u32,
        source_location: Option<SourceLocation>,
    },
    /// Error propagation via question mark operator
    ErrorPropagation {
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    },
    /// Parser errors with location information
    ParseError {
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    },
    /// Code generation errors
    CodeGeneration {
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    },
    /// Process management errors
    ProcessError(String),
    /// Optimization errors
    OptimizationError(String),
}

/// Alias for CursedError to match expected naming
pub type CursedError = Error;

impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Error::Io(err) => Error::Io(std::io::Error::new(err.kind(), err.to_string())),
            Error::Parse(msg) => Error::Parse(msg.clone()),
            Error::Compile(msg) => Error::Compile(msg.clone()),
            Error::Runtime(msg) => Error::Runtime(msg.clone()),
            Error::Package(msg) => Error::Package(msg.clone()),
            Error::Repl(msg) => Error::Repl(msg.clone()),
            Error::TemplateError { message, source_location } => Error::TemplateError {
                message: message.clone(),
                source_location: source_location.clone(),
            },
            Error::TypeCompilation(msg) => Error::TypeCompilation(msg.clone()),
            Error::Type(msg) => Error::Type(msg.clone()),
            Error::Panic { message, panic_id, recoverable, source_location } => Error::Panic {
                message: message.clone(),
                panic_id: *panic_id,
                recoverable: *recoverable,
                source_location: source_location.clone(),
            },
            Error::Recovery { message, recovery_attempts, source_location } => Error::Recovery {
                message: message.clone(),
                recovery_attempts: *recovery_attempts,
                source_location: source_location.clone(),
            },
            Error::ErrorPropagation { message, line, column } => Error::ErrorPropagation {
                message: message.clone(),
                line: *line,
                column: *column,
            },
            Error::ParseError { message, line, column } => Error::ParseError {
                message: message.clone(),
                line: *line,
                column: *column,
            },
            Error::CodeGeneration { message, line, column } => Error::CodeGeneration {
                message: message.clone(),
                line: *line,
                column: *column,
            },
            Error::ProcessError(msg) => Error::ProcessError(msg.clone()),
            Error::OptimizationError(msg) => Error::OptimizationError(msg.clone()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Parse(msg) => write!(f, "Parse error: {}", msg),
            Error::Compile(msg) => write!(f, "Compilation error: {}", msg),
            Error::Runtime(msg) => write!(f, "Runtime error: {}", msg),
            Error::Package(msg) => write!(f, "Package error: {}", msg),
            Error::Repl(msg) => write!(f, "REPL error: {}", msg),
            Error::TemplateError { message, source_location } => {
                if let Some(loc) = source_location {
                    write!(f, "Template error at {}: {}", loc, message)
                } else {
                    write!(f, "Template error: {}", message)
                }
            }
            Error::TypeCompilation(msg) => write!(f, "Type compilation error: {}", msg),
            Error::Type(msg) => write!(f, "Type error: {}", msg),
            Error::Panic { message, panic_id, recoverable, source_location } => {
                write!(f, "Panic")?;
                if let Some(id) = panic_id {
                    write!(f, " #{}", id)?;
                }
                if *recoverable {
                    write!(f, " (recoverable)")?;
                }
                if let Some(loc) = source_location {
                    write!(f, " at {}: {}", loc, message)
                } else {
                    write!(f, ": {}", message)
                }
            }
            Error::Recovery { message, recovery_attempts, source_location } => {
                write!(f, "Recovery error")?;
                if *recovery_attempts > 0 {
                    write!(f, " (attempt {})", recovery_attempts)?;
                }
                if let Some(loc) = source_location {
                    write!(f, " at {}: {}", loc, message)
                } else {
                    write!(f, ": {}", message)
                }
            }
            Error::ErrorPropagation { message, line, column } => {
                write!(f, "Error propagation")?;
                if let (Some(line), Some(column)) = (line, column) {
                    write!(f, " at line {}, column {}", line, column)?;
                }
                write!(f, ": {}", message)
            }
            Error::ParseError { message, line, column } => {
                write!(f, "Parse error")?;
                if let (Some(line), Some(column)) = (line, column) {
                    write!(f, " at line {}, column {}", line, column)?;
                }
                write!(f, ": {}", message)
            }
            Error::CodeGeneration { message, line, column } => {
                write!(f, "Code generation error")?;
                if let (Some(line), Some(column)) = (line, column) {
                    write!(f, " at line {}, column {}", line, column)?;
                }
                write!(f, ": {}", message)
            }
            Error::ProcessError(msg) => write!(f, "Process error: {}", msg),
            Error::OptimizationError(msg) => write!(f, "Optimization error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}



impl Error {
    /// Create a REPL error
    pub fn repl_error(msg: String) -> Self {
        Error::Repl(msg)
    }
    
    /// Create an error from a string (for backward compatibility)
    pub fn from_str(msg: &str) -> Self {
        Error::Repl(msg.to_string())
    }

    /// Create a panic error
    pub fn panic_error(message: String) -> Self {
        Error::Panic {
            message,
            panic_id: None,
            recoverable: false,
            source_location: None,
        }
    }

    /// Create a recoverable panic error
    pub fn recoverable_panic(message: String) -> Self {
        Error::Panic {
            message,
            panic_id: None,
            recoverable: true,
            source_location: None,
        }
    }

    /// Create a panic error with ID and location
    pub fn panic_with_details(
        message: String,
        panic_id: u64,
        recoverable: bool,
        source_location: Option<SourceLocation>,
    ) -> Self {
        Error::Panic {
            message,
            panic_id: Some(panic_id),
            recoverable,
            source_location,
        }
    }

    /// Create a recovery error
    pub fn recovery_error(message: String, recovery_attempts: u32) -> Self {
        Error::Recovery {
            message,
            recovery_attempts,
            source_location: None,
        }
    }

    /// Create a recovery error with location
    pub fn recovery_error_with_location(
        message: String,
        recovery_attempts: u32,
        source_location: SourceLocation,
    ) -> Self {
        Error::Recovery {
            message,
            recovery_attempts,
            source_location: Some(source_location),
        }
    }

    /// Create a system error
    pub fn system_error(message: &str) -> Self {
        Error::Runtime(format!("System error: {}", message))
    }

    /// Create a type error
    pub fn type_error(message: String) -> Self {
        Error::Type(message)
    }

    /// Create an optimization error
    pub fn optimization_error(message: String) -> Self {
        Error::Compile(format!("Optimization error: {}", message))
    }

    /// Create a new error with category and message
    pub fn new(category: &str, message: &str) -> Self {
        match category {
            "panic" => Error::panic_error(message.to_string()),
            "recovery" => Error::recovery_error(message.to_string(), 0),
            "runtime" => Error::Runtime(message.to_string()),
            "type" => Error::Type(message.to_string()),
            "parse" => Error::Parse(message.to_string()),
            "compile" => Error::Compile(message.to_string()),
            _ => Error::Runtime(format!("{}: {}", category, message)),
        }
    }

    /// Check if this error represents a recoverable panic
    pub fn is_recoverable_panic(&self) -> bool {
        matches!(self, Error::Panic { recoverable: true, .. })
    }

    /// Check if this error is panic-related
    pub fn is_panic(&self) -> bool {
        matches!(self, Error::Panic { .. })
    }

    /// Check if this error is recovery-related
    pub fn is_recovery(&self) -> bool {
        matches!(self, Error::Recovery { .. })
    }

    /// Get the panic ID if this is a panic error
    pub fn get_panic_id(&self) -> Option<u64> {
        match self {
            Error::Panic { panic_id, .. } => *panic_id,
            _ => None,
        }
    }

    /// Get the source location if available
    pub fn get_source_location(&self) -> Option<&SourceLocation> {
        match self {
            Error::TemplateError { source_location, .. } |
            Error::Panic { source_location, .. } |
            Error::Recovery { source_location, .. } => source_location.as_ref(),
            _ => None,
        }
    }

    /// Create an error propagation error
    pub fn error_propagation(message: String) -> Self {
        Error::ErrorPropagation {
            message,
            line: None,
            column: None,
        }
    }

    /// Create an error propagation error with location
    pub fn error_propagation_with_location(message: String, line: usize, column: usize) -> Self {
        Error::ErrorPropagation {
            message,
            line: Some(line),
            column: Some(column),
        }
    }

    /// Create a parse error with location
    pub fn parse_error_with_location(message: String, line: usize, column: usize) -> Self {
        Error::ParseError {
            message,
            line: Some(line),
            column: Some(column),
        }
    }

    /// Create a code generation error with location
    pub fn code_generation_error(message: String, line: Option<usize>, column: Option<usize>) -> Self {
        Error::CodeGeneration {
            message,
            line,
            column,
        }
    }

    /// Check if this error is error propagation related
    pub fn is_error_propagation(&self) -> bool {
        matches!(self, Error::ErrorPropagation { .. })
    }

    /// Get line number if available
    pub fn get_line(&self) -> Option<usize> {
        match self {
            Error::ErrorPropagation { line, .. } |
            Error::ParseError { line, .. } |
            Error::CodeGeneration { line, .. } => *line,
            _ => self.get_source_location().map(|loc| loc.line as usize),
        }
    }

    /// Get column number if available
    pub fn get_column(&self) -> Option<usize> {
        match self {
            Error::ErrorPropagation { column, .. } |
            Error::ParseError { column, .. } |
            Error::CodeGeneration { column, .. } => *column,
            _ => self.get_source_location().map(|loc| loc.column as usize),
        }
    }

    /// Create a process error
    pub fn process_error(message: String) -> Self {
        Error::ProcessError(message)
    }

    /// Create an optimization error
    pub fn optimization_error(message: String) -> Self {
        Error::OptimizationError(message)
    }
}



/// Source location information for errors
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize) -> Self {
        SourceLocation {
            line,
            column,
            file: None,
        }
    }
    
    pub fn with_file(mut self, file: &str) -> Self {
        self.file = Some(file.to_string());
        self
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            line: 0,
            column: 0,
            file: None,
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(file) = &self.file {
            write!(f, "{}:{}:{}", file, self.line, self.column)
        } else {
            write!(f, "{}:{}", self.line, self.column)
        }
    }
}
