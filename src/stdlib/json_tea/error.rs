/*!
 * JSON Tea CursedError Types
 * 
 * Specialized error types for JSON operations in CURSED
 */

use crate::error::CursedError;
use std::fmt;

/// JSON-specific error types
#[derive(Debug, Clone, PartialEq)]
pub enum JsonErrorKind {
    /// Invalid JSON syntax
    /// Type mismatch during encoding/decoding
    /// Invalid UTF-8 sequence
    /// Number parsing error
    /// String parsing error
    /// Unexpected end of input
    /// Invalid escape sequence
    /// Circular reference detected
    /// Unsupported type for JSON encoding
    /// Invalid JSON tag format
    /// I/O error during streaming operations
    /// Custom error with message
// impl fmt::Display for JsonErrorKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             JsonErrorKind::SyntaxError { message, position } => {
//                 write!(f, "JSON syntax error at position {}: {}", position, message)
//             }
//             JsonErrorKind::TypeError { expected, found } => {
//                 write!(f, "JSON type error: expected {}, found {}", expected, found)
//             }
//             JsonErrorKind::InvalidUtf8 { message } => {
//                 write!(f, "Invalid UTF-8 in JSON: {}", message)
//             }
//             JsonErrorKind::InvalidNumber { value } => {
//                 write!(f, "Invalid JSON number: {}", value)
//             }
//             JsonErrorKind::InvalidString { message } => {
//                 write!(f, "Invalid JSON string: {}", message)
//             }
//             JsonErrorKind::UnexpectedEof => {
//                 write!(f, "Unexpected end of JSON input")
//             }
//             JsonErrorKind::InvalidEscape { sequence } => {
//                 write!(f, "Invalid JSON escape sequence: {}", sequence)
//             }
//             JsonErrorKind::CircularReference { path } => {
//                 write!(f, "Circular reference detected in JSON encoding at path: {}", path)
//             }
//             JsonErrorKind::UnsupportedType { type_name } => {
//                 write!(f, "Unsupported type for JSON encoding: {}", type_name)
//             }
//             JsonErrorKind::InvalidTag { tag, message } => {
//                 write!(f, "Invalid JSON tag '{}': {}", tag, message)
//             }
//             JsonErrorKind::IoError { message } => {
//                 write!(f, "JSON I/O error: {}", message)
//             }
//             JsonErrorKind::Custom { message } => {
//                 write!(f, "JSON error: {}", message)
//             }
//         }
//     }
// }

/// Helper functions to create JSON errors
impl CursedError {
    pub fn json_syntax_error(message: String, position: usize) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::SyntaxError { message, position }))
    pub fn json_type_error(expected: String, found: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::TypeError { expected, found }))
    pub fn json_invalid_utf8(message: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::InvalidUtf8 { message }))
    pub fn json_invalid_number(value: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::InvalidNumber { value }))
    pub fn json_invalid_string(message: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::InvalidString { message }))
    pub fn json_unexpected_eof() -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::UnexpectedEof))
    pub fn json_invalid_escape(sequence: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::InvalidEscape { sequence }))
    pub fn json_circular_reference(path: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::CircularReference { path }))
    pub fn json_unsupported_type(type_name: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::UnsupportedType { type_name }))
    pub fn json_invalid_tag(tag: String, message: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::InvalidTag { tag, message }))
    pub fn json_io_error(message: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::IoError { message }))
    pub fn json_custom_error(message: String) -> Self {
        CursedError::json_error(format!("{}", JsonErrorKind::Custom { message }))
    }
}

/// Convert from I/O errors to JSON errors
// impl From<std::io::Error> for JsonErrorKind {
//     fn from(err: std::io::Error) -> Self {
//         JsonErrorKind::IoError {
//             message: err.to_string(),
//         }
//     }
// }

/// Convert from UTF-8 errors to JSON errors
impl From<std::str::Utf8Error> for JsonErrorKind {
    fn from(err: std::str::Utf8Error) -> Self {
        JsonErrorKind::InvalidUtf8 {
        }
    }
/// Convert from string UTF-8 errors to JSON errors
impl From<std::string::FromUtf8Error> for JsonErrorKind {
    fn from(err: std::string::FromUtf8Error) -> Self {
        JsonErrorKind::InvalidUtf8 {
        }
    }
/// Convert from number parsing errors to JSON errors
impl From<std::num::ParseFloatError> for JsonErrorKind {
    fn from(err: std::num::ParseFloatError) -> Self {
        JsonErrorKind::InvalidNumber {
        }
    }
