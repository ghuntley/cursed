//! Error handling for CURSED
//! 
//! Centralized error types and handling for the CURSED programming language.

use std::fmt;

#[derive(Debug, Clone)]
pub struct SourceLocation {
impl SourceLocation {
    pub fn new(file: std::path::PathBuf, line: usize, column: usize) -> Self {
        Self { file, line, column }
    }
impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.column)
    }
}

/// Main error type for CURSED
#[derive(Debug)]
pub enum Error {
    /// I/O related errors
    /// Parsing errors
    /// Compilation errors
    /// Runtime errors
    /// Package manager errors
    /// REPL errors
    /// Template system errors
    TemplateError { 
    /// Optimization errors
    /// Memory management errors
    /// Type system errors
    /// Import errors
    /// Generic error with message
// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Error::Io(e) => write!(f, "I/O error: {}", e),
//             Error::Parse(msg) => write!(f, "Parse error: {}", msg),
//             Error::CompilationError(msg) => write!(f, "Compilation error: {}", msg),
//             Error::Runtime(msg) => write!(f, "Runtime error: {}", msg),
//             Error::Package(msg) => write!(f, "Package error: {}", msg),
//             Error::Repl(msg) => write!(f, "REPL error: {}", msg),
//             Error::TemplateError { message, source_location } => {
//                 if let Some(loc) = source_location {
//                     write!(f, "Template error at {}: {}", loc, message)
//                 } else {
//                     write!(f, "Template error: {}", message)
//                 }
//             },
//             Error::OptimizationError(msg) => write!(f, "Optimization error: {}", msg),
//             Error::MemoryError(msg) => write!(f, "Memory error: {}", msg),
//             Error::TypeError(msg) => write!(f, "Type error: {}", msg),
//             Error::ImportError(msg) => write!(f, "Import error: {}", msg),
//             Error::Generic(msg) => write!(f, "{}", msg),
//         }
//     }
// }

// impl std::error::Error for Error {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             Error::Io(e) => Some(e),
//             _ => None,
//         }
//     }
// }

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Generic(error)
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::Generic(error.to_string())
    }
}

/// Type alias for backward compatibility
pub type CursedError = Error;

/// Result type alias for CURSED operations
pub type Result<T> = std::result::Result<T, Error>;

impl From<crate::runtime::r#async::FutureError> for Error {
    fn from(err: crate::runtime::r#async::FutureError) -> Self {
        Error::Runtime(format!("Future error: {}", err))
    }
}


