// Error types for CURSED language
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    Parse(String),
    Type(String),
    Runtime(String),
    Compile(String),
    Import(String),
    Lexer(String),
    Io(String),
    Memory(String),
    TypeCheck(String),
    Package(String),
    Template(String),
    Optimization(String),
    Debug(String),
    InvalidOptimizationLevel(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(msg) => write!(f, "Parse error: {}", msg),
            Error::Type(msg) => write!(f, "Type error: {}", msg),
            Error::Runtime(msg) => write!(f, "Runtime error: {}", msg),
            Error::Compile(msg) => write!(f, "Compile error: {}", msg),
            Error::Import(msg) => write!(f, "Import error: {}", msg),
            Error::Lexer(msg) => write!(f, "Lexer error: {}", msg),
            Error::Io(msg) => write!(f, "I/O error: {}", msg),
            Error::Memory(msg) => write!(f, "Memory error: {}", msg),
            Error::TypeCheck(msg) => write!(f, "Type check error: {}", msg),
            Error::Package(msg) => write!(f, "Package error: {}", msg),
            Error::Template(msg) => write!(f, "Template error: {}", msg),
            Error::Optimization(msg) => write!(f, "Optimization error: {}", msg),
            Error::Debug(msg) => write!(f, "Debug error: {}", msg),
            Error::InvalidOptimizationLevel(msg) => write!(f, "Invalid optimization level: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

// Legacy type alias for compatibility
pub type CursedError = Error;
