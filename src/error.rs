use std::fmt;
use std::io;
use thiserror::Error;

/// Source code location information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

impl SourceLocation {
    /// Create a new source location
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Error types for the CURSED compiler and runtime
#[derive(Error, Debug)]
pub enum Error {
    /// I/O errors
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    /// Lexer errors
    #[error("Lexer error at {location}: {message}")]
    Lexer {
        location: SourceLocation,
        message: String,
    },

    /// Parser errors
    #[error("Parser error at {location}: {message}")]
    Parser {
        location: SourceLocation,
        message: String,
    },

    /// Type errors
    #[error("Type error at {location}: {message}")]
    Type {
        location: SourceLocation,
        message: String,
    },

    /// Semantic errors
    #[error("Semantic error: {0}")]
    SemanticError(String),

    /// Compiler errors
    #[error("Compilation error: {0}")]
    Compilation(String),

    /// Runtime errors
    #[error("Runtime error: {0}")]
    Runtime(String),

    /// Bytecode errors
    #[error("Bytecode error: {0}")]
    BytecodeError(String),

    /// Virtual machine errors
    #[error("VM error: {0}")]
    VMError(String),

    /// Unknown errors
    #[error("Unknown error: {0}")]
    Unknown(String),

    /// Syntax errors
    #[error("Syntax error at {location}: {message}")]
    Syntax {
        location: SourceLocation,
        message: String,
    },
}

/// Error reporting for CURSED
pub struct ErrorReporter;

impl ErrorReporter {
    /// Report a lexer error
    pub fn lexer_error(location: SourceLocation, message: &str) -> Error {
        Error::Lexer {
            location,
            message: message.to_string(),
        }
    }

    /// Report a parser error
    pub fn parser_error(location: SourceLocation, message: &str) -> Error {
        Error::Parser {
            location,
            message: message.to_string(),
        }
    }

    /// Report a type error
    pub fn type_error(location: SourceLocation, message: &str) -> Error {
        Error::Type {
            location,
            message: message.to_string(),
        }
    }

    /// Report a semantic error
    pub fn semantic_error(message: &str) -> Error {
        Error::SemanticError(message.to_string())
    }

    /// Report a compilation error
    pub fn compilation_error(message: &str) -> Error {
        Error::Compilation(message.to_string())
    }

    /// Report a runtime error
    pub fn runtime_error(message: &str) -> Error {
        Error::Runtime(message.to_string())
    }

    /// Report a bytecode error
    pub fn bytecode_error(message: &str) -> Error {
        Error::BytecodeError(message.to_string())
    }

    /// Report a virtual machine error
    pub fn vm_error(message: &str) -> Error {
        Error::VMError(message.to_string())
    }
}

impl Error {
    /// Create a new runtime error
    pub fn new<T: Into<String>>(message: T) -> Self {
        Error::Runtime(message.into())
    }
    
    /// Create a runtime error from a string
    pub fn from_str(message: &str) -> Self {
        Error::Runtime(message.to_string())
    }
    
    /// Create a syntax error
    pub fn syntax<T: Into<String>>(message: T, location: SourceLocation) -> Self {
        Error::Syntax { 
            message: message.into(),
            location,
        }
    }
} 