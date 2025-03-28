use std::fmt;
use std::io;
use thiserror::Error;

/// Represents a location in the source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    /// Line number (0-based)
    pub line: usize,
    /// Column number (0-based)
    pub column: usize,
    pub file: Option<String>,
    pub source_line: String,
}

impl SourceLocation {
    /// Create a new source location
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
            file: None,
            source_line: String::new(),
        }
    }

    /// Create a new source location with file information
    pub fn with_file(line: usize, column: usize, file: String) -> Self {
        Self { 
            line, 
            column,
            file: Some(file),
            source_line: String::new(),
        }
    }

    /// Get the file name if available
    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    /// Create a default source location (0, 0)
    pub fn default() -> Self {
        Self {
            line: 0,
            column: 0,
            file: None,
            source_line: String::new(),
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(file) = &self.file {
            write!(f, "{}:{}:{}", file, self.line, self.column)
        } else {
            write!(f, "line {}, column {}", self.line, self.column)
        }
    }
}

/// Error type for the CURSED language
#[derive(Debug)]
pub enum Error {
    /// I/O errors
    IoError(io::Error),

    /// Lexer errors
    Lexer {
        location: SourceLocation,
        message: String,
    },

    /// Parser errors
    Parser {
        location: SourceLocation,
        message: String,
    },

    /// Type errors
    Type {
        location: SourceLocation,
        message: String,
    },

    /// Semantic errors
    SemanticError(String),

    /// Compiler errors
    Compilation(String),

    /// Runtime errors
    Runtime(String),
    
    /// Memory management errors
    Memory(String),

    /// Bytecode errors
    BytecodeError(String),

    /// Virtual machine errors
    VMError(String),

    /// Unknown errors
    Unknown(String),

    /// Syntax errors
    Syntax {
        location: SourceLocation,
        message: String,
    },

    /// Index out of bounds errors
    IndexError(String),

    /// Division by zero errors
    DivisionByZero(String),

    /// Stack overflow errors
    StackOverflow(String),

    /// Heap overflow errors
    HeapOverflow(String),

    /// Invalid operation errors
    InvalidOperation(String),

    /// System errors
    SystemError(String),
}

/// Error reporting for CURSED
pub struct ErrorReporter;

impl ErrorReporter {
    /// Create a new error reporter
    pub fn new() -> Self {
        Self
    }

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
    
    /// Report a memory error
    pub fn memory_error(message: &str) -> Error {
        Error::Memory(message.to_string())
    }

    /// Report a bytecode error
    pub fn bytecode_error(message: &str) -> Error {
        Error::BytecodeError(message.to_string())
    }

    /// Report a virtual machine error
    pub fn vm_error(message: &str) -> Error {
        Error::VMError(message.to_string())
    }

    /// Report an index error
    pub fn index_error(message: &str) -> Error {
        Error::IndexError(message.to_string())
    }

    /// Report a division by zero error
    pub fn division_by_zero(message: &str) -> Error {
        Error::DivisionByZero(message.to_string())
    }

    /// Report a stack overflow error
    pub fn stack_overflow(message: &str) -> Error {
        Error::StackOverflow(message.to_string())
    }

    /// Report a heap overflow error
    pub fn heap_overflow(message: &str) -> Error {
        Error::HeapOverflow(message.to_string())
    }

    /// Report an invalid operation error
    pub fn invalid_operation(message: &str) -> Error {
        Error::InvalidOperation(message.to_string())
    }

    /// Report a system error
    pub fn system_error(message: &str) -> Error {
        Error::SystemError(message.to_string())
    }
}

impl Error {
    /// Create a new runtime error
    pub fn new<T: Into<String>>(error_type: &str, message: T, location: Option<SourceLocation>) -> Self {
        match error_type {
            "Lexer" => {
                if let Some(loc) = location {
                    Error::Lexer {
                        location: loc,
                        message: message.into(),
                    }
                } else {
                    Error::SemanticError(message.into())
                }
            },
            "Parser" => {
                if let Some(loc) = location {
                    Error::Parser {
                        location: loc,
                        message: message.into(),
                    }
                } else {
                    Error::SemanticError(message.into())
                }
            },
            "Type" => {
                if let Some(loc) = location {
                    Error::Type {
                        location: loc,
                        message: message.into(),
                    }
                } else {
                    Error::SemanticError(message.into())
                }
            },
            "Semantic" => Error::SemanticError(message.into()),
            "Compilation" => Error::Compilation(message.into()),
            "Runtime" => Error::Runtime(message.into()),
            "Memory" => Error::Memory(message.into()),
            "Bytecode" => Error::BytecodeError(message.into()),
            "VM" => Error::VMError(message.into()),
            "IndexError" => Error::IndexError(message.into()),
            "DivisionByZero" => Error::DivisionByZero(message.into()),
            "StackOverflow" => Error::StackOverflow(message.into()),
            "HeapOverflow" => Error::HeapOverflow(message.into()),
            "InvalidOperation" => Error::InvalidOperation(message.into()),
            "System" => Error::SystemError(message.into()),
            _ => Error::Unknown(message.into()),
        }
    }
    
    /// Create a runtime error from a string
    pub fn from_str(message: &str) -> Self {
        Error::Runtime(message.to_string())
    }
    
    /// Create a memory error
    pub fn memory<T: Into<String>>(message: T) -> Self {
        Error::Memory(message.into())
    }
    
    /// Create a syntax error
    pub fn syntax<T: Into<String>>(message: T, location: SourceLocation) -> Self {
        Error::Syntax { 
            message: message.into(),
            location,
        }
    }
    
    /// Create a lexer error
    pub fn lexer<T: Into<String>>(message: T, location: SourceLocation) -> Self {
        Error::Lexer {
            message: message.into(),
            location,
        }
    }
    
    /// Create a parser error
    pub fn parser<T: Into<String>>(message: T, location: SourceLocation) -> Self {
        Error::Parser {
            message: message.into(),
            location,
        }
    }
    
    /// Create a type error
    pub fn type_error<T: Into<String>>(message: T, location: SourceLocation) -> Self {
        Error::Type {
            message: message.into(),
            location,
        }
    }
    
    /// Create a VM error
    pub fn vm<T: Into<String>>(message: T) -> Self {
        Error::VMError(message.into())
    }
    
    /// Create a bytecode error
    pub fn bytecode<T: Into<String>>(message: T) -> Self {
        Error::BytecodeError(message.into())
    }

    /// Create an index error
    pub fn index<T: Into<String>>(message: T) -> Self {
        Error::IndexError(message.into())
    }

    /// Create a division by zero error
    pub fn division_by_zero<T: Into<String>>(message: T) -> Self {
        Error::DivisionByZero(message.into())
    }

    /// Create a stack overflow error
    pub fn stack_overflow<T: Into<String>>(message: T) -> Self {
        Error::StackOverflow(message.into())
    }

    /// Create a heap overflow error
    pub fn heap_overflow<T: Into<String>>(message: T) -> Self {
        Error::HeapOverflow(message.into())
    }

    /// Create an invalid operation error
    pub fn invalid_operation<T: Into<String>>(message: T) -> Self {
        Error::InvalidOperation(message.into())
    }

    /// Create a system error
    pub fn system<T: Into<String>>(message: T) -> Self {
        Error::SystemError(message.into())
    }

    /// Get the error message
    pub fn message(&self) -> String {
        match self {
            Error::IoError(err) => err.to_string(),
            Error::Lexer { message, .. } => message.clone(),
            Error::Parser { message, .. } => message.clone(),
            Error::Type { message, .. } => message.clone(),
            Error::SemanticError(msg) => msg.clone(),
            Error::Compilation(msg) => msg.clone(),
            Error::Runtime(msg) => msg.clone(),
            Error::Memory(msg) => msg.clone(),
            Error::BytecodeError(msg) => msg.clone(),
            Error::VMError(msg) => msg.clone(),
            Error::Unknown(msg) => msg.clone(),
            Error::Syntax { message, .. } => message.clone(),
            Error::IndexError(msg) => msg.clone(),
            Error::DivisionByZero(msg) => msg.clone(),
            Error::StackOverflow(msg) => msg.clone(),
            Error::HeapOverflow(msg) => msg.clone(),
            Error::InvalidOperation(msg) => msg.clone(),
            Error::SystemError(msg) => msg.clone(),
        }
    }

    /// Get the source location if available
    pub fn location(&self) -> Option<SourceLocation> {
        match self {
            Error::Lexer { location, .. } => Some(location.clone()),
            Error::Parser { location, .. } => Some(location.clone()),
            Error::Type { location, .. } => Some(location.clone()),
            Error::Syntax { location, .. } => Some(location.clone()),
            _ => None,
        }
    }

    /// Create a 'not implemented' error
    pub fn NotImplemented<T: Into<String>>(message: T, location: SourceLocation) -> Self {
        Error::Unknown(format!("Not implemented: {}", message.into()))
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

/// Implement From trait for Error to convert string literals to Error
impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::Unknown(message.to_string())
    }
}

/// Implement From trait for Error to convert owned strings to Error
impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::Unknown(message)
    }
} 