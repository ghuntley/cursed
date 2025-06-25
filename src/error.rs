/// Minimal error handling for CURSED
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    /// I/O related errors
    Io(String),
    /// Parsing errors
    Parse(String),
    /// Compilation errors
    Compile(String),
    /// Runtime errors
    Runtime(String),
    /// Not implemented errors
    NotImplemented(String),
    /// General errors
    General(String),
    /// Debug errors
    Debug(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(msg) => write!(f, "I/O error: {}", msg),
            Error::Parse(msg) => write!(f, "Parse error: {}", msg),
            Error::Compile(msg) => write!(f, "Compilation error: {}", msg),
            Error::Runtime(msg) => write!(f, "Runtime error: {}", msg),
            Error::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            Error::General(msg) => write!(f, "General error: {}", msg),
            Error::Debug(msg) => write!(f, "Debug error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Runtime(msg)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Runtime(msg.to_string())
    }
}
