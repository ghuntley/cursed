//! Error handling for CURSED
//! 
//! Centralized error types and handling for the CURSED programming language.

use std::fmt;

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
}

/// Alias for CursedError to match expected naming
pub type CursedError = Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Parse(msg) => write!(f, "Parse error: {}", msg),
            Error::Compile(msg) => write!(f, "Compilation error: {}", msg),
            Error::Runtime(msg) => write!(f, "Runtime error: {}", msg),
            Error::Package(msg) => write!(f, "Package error: {}", msg),
            Error::Repl(msg) => write!(f, "REPL error: {}", msg),
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
}



/// Source location information for errors
#[derive(Debug, Clone)]
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

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(file) = &self.file {
            write!(f, "{}:{}:{}", file, self.line, self.column)
        } else {
            write!(f, "{}:{}", self.line, self.column)
        }
    }
}
