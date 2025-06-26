use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    /// Parsing error
    Parse(String),
    /// Type checking error  
    TypeCheck(String),
    /// Compilation error
    Compile(String),
    /// Runtime error
    Runtime(String),
    /// I/O error
    Io(String),
    /// Import error
    Import(String),
    /// Package management error
    Package(String),
    /// Template error  
    Template(String),
    /// Optimization error
    Optimization(String),
    /// Memory error
    Memory(String),
    /// Debug error
    Debug(String),
    /// Invalid optimization level
    InvalidOptimizationLevel(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(msg) => write!(f, "Parse error: {}", msg),
            Error::TypeCheck(msg) => write!(f, "Type check error: {}", msg),
            Error::Compile(msg) => write!(f, "Compilation error: {}", msg),
            Error::Runtime(msg) => write!(f, "Runtime error: {}", msg),
            Error::Io(msg) => write!(f, "I/O error: {}", msg),
            Error::Import(msg) => write!(f, "Import error: {}", msg),
            Error::Package(msg) => write!(f, "Package error: {}", msg),
            Error::Template(msg) => write!(f, "Template error: {}", msg),
            Error::Optimization(msg) => write!(f, "Optimization error: {}", msg),
            Error::Memory(msg) => write!(f, "Memory error: {}", msg),
            Error::Debug(msg) => write!(f, "Debug error: {}", msg),
            Error::InvalidOptimizationLevel(msg) => write!(f, "Invalid optimization level: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

// Re-export for compatibility
pub type CursedError = Error;
