/*!
 * JSON Tea Error Types
 * 
 * Specialized error types for JSON operations in CURSED
 */

use crate::error::CursedError;
use std::fmt;

/// JSON-specific error types
#[derive(Debug, Clone, PartialEq)]
pub enum JsonErrorKind {
    /// Invalid JSON syntax
    SyntaxError { message: String, position: usize },
    /// Type mismatch during encoding/decoding
    TypeError { expected: String, found: String },
    /// Invalid UTF-8 sequence
    InvalidUtf8 { message: String },
    /// Number parsing error
    InvalidNumber { value: String },
    /// String parsing error
    InvalidString { message: String },
    /// Unexpected end of input
    UnexpectedEof,
    /// Invalid escape sequence
    InvalidEscape { sequence: String },
    /// Circular reference detected
    CircularReference { path: String },
    /// Unsupported type for JSON encoding
    UnsupportedType { type_name: String },
    /// Invalid JSON tag format
    InvalidTag { tag: String, message: String },
    /// I/O error during streaming operations
    IoError { message: String },
    /// Custom error with message
    Custom { message: String },
}

impl fmt::Display for JsonErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonErrorKind::SyntaxError { message, position } => {
                write!(f, "JSON syntax error at position {}: {}", position, message)
            }
            JsonErrorKind::TypeError { expected, found } => {
                write!(f, "JSON type error: expected {}, found {}", expected, found)
            }
            JsonErrorKind::InvalidUtf8 { message } => {
                write!(f, "Invalid UTF-8 in JSON: {}", message)
            }
            JsonErrorKind::InvalidNumber { value } => {
                write!(f, "Invalid JSON number: {}", value)
            }
            JsonErrorKind::InvalidString { message } => {
                write!(f, "Invalid JSON string: {}", message)
            }
            JsonErrorKind::UnexpectedEof => {
                write!(f, "Unexpected end of JSON input")
            }
            JsonErrorKind::InvalidEscape { sequence } => {
                write!(f, "Invalid JSON escape sequence: {}", sequence)
            }
            JsonErrorKind::CircularReference { path } => {
                write!(f, "Circular reference detected in JSON encoding at path: {}", path)
            }
            JsonErrorKind::UnsupportedType { type_name } => {
                write!(f, "Unsupported type for JSON encoding: {}", type_name)
            }
            JsonErrorKind::InvalidTag { tag, message } => {
                write!(f, "Invalid JSON tag '{}': {}", tag, message)
            }
            JsonErrorKind::IoError { message } => {
                write!(f, "JSON I/O error: {}", message)
            }
            JsonErrorKind::Custom { message } => {
                write!(f, "JSON error: {}", message)
            }
        }
    }
}

/// Helper functions to create JSON errors
impl CursedError {
    pub fn json_syntax_error(message: String, position: usize) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::SyntaxError { message, position }))
    }
    
    pub fn json_type_error(expected: String, found: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::TypeError { expected, found }))
    }
    
    pub fn json_invalid_utf8(message: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::InvalidUtf8 { message }))
    }
    
    pub fn json_invalid_number(value: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::InvalidNumber { value }))
    }
    
    pub fn json_invalid_string(message: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::InvalidString { message }))
    }
    
    pub fn json_unexpected_eof() -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::UnexpectedEof))
    }
    
    pub fn json_invalid_escape(sequence: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::InvalidEscape { sequence }))
    }
    
    pub fn json_circular_reference(path: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::CircularReference { path }))
    }
    
    pub fn json_unsupported_type(type_name: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::UnsupportedType { type_name }))
    }
    
    pub fn json_invalid_tag(tag: String, message: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::InvalidTag { tag, message }))
    }
    
    pub fn json_io_error(message: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::IoError { message }))
    }
    
    pub fn json_custom_error(message: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::Custom { message }))
    }
}

/// Convert from I/O errors to JSON errors
impl From<std::io::Error> for JsonErrorKind {
    fn from(err: std::io::Error) -> Self {
        JsonErrorKind::IoError {
            message: err.to_string(),
        }
    }
}

/// Convert from UTF-8 errors to JSON errors
impl From<std::str::Utf8Error> for JsonErrorKind {
    fn from(err: std::str::Utf8Error) -> Self {
        JsonErrorKind::InvalidUtf8 {
            message: err.to_string(),
        }
    }
}

/// Convert from string UTF-8 errors to JSON errors
impl From<std::string::FromUtf8Error> for JsonErrorKind {
    fn from(err: std::string::FromUtf8Error) -> Self {
        JsonErrorKind::InvalidUtf8 {
            message: err.to_string(),
        }
    }
}

/// Convert from number parsing errors to JSON errors
impl From<std::num::ParseFloatError> for JsonErrorKind {
    fn from(err: std::num::ParseFloatError) -> Self {
        JsonErrorKind::InvalidNumber {
            value: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_json_error_display() {
        let syntax_err = JsonErrorKind::SyntaxError {
            message: "Expected '}'".to_string(),
            position: 42,
        };
        assert!(syntax_err.to_string().contains("position 42"));
        assert!(syntax_err.to_string().contains("Expected '}'"));
        
        let type_err = JsonErrorKind::TypeError {
            expected: "string".to_string(),
            found: "number".to_string(),
        };
        assert!(type_err.to_string().contains("expected string"));
        assert!(type_err.to_string().contains("found number"));
    }
    
    #[test]
    fn test_cursed_error_helpers() {
        let err = CursedError::json_syntax_error("test".to_string(), 10);
        assert!(err.to_string().contains("JSON syntax error"));
        assert!(err.to_string().contains("position 10"));
        
        let type_err = CursedError::json_type_error("string".to_string(), "number".to_string());
        assert!(type_err.to_string().contains("type error"));
    }
    
    #[test]
    fn test_error_conversions() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let json_err: JsonErrorKind = io_err.into();
        assert!(matches!(json_err, JsonErrorKind::IoError { .. }));
        
        let utf8_err = std::str::from_utf8(b"\xFF\xFE").unwrap_err();
        let json_err: JsonErrorKind = utf8_err.into();
        assert!(matches!(json_err, JsonErrorKind::InvalidUtf8 { .. }));
    }
}
