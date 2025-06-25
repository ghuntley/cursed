use crate::error::CursedError;
/// CursedError handling for LookinGlass reflection package
use std::fmt;

/// Result type for reflection operations
pub type LookinGlassResult<T> = std::result::Result<T, LookinGlassError>;

/// Errors that can occur during reflection operations
#[derive(Debug, Clone)]
pub enum LookinGlassError {
    /// Type not found or invalid
    /// Value cannot be reflected upon
    /// Field access error
    /// Method access error
    /// Conversion error
    /// Index out of bounds
    /// Invalid operation for this type
    /// Cannot set value (not addressable or settable)
    /// Type mismatch
    /// JSON serialization/deserialization error
    /// General reflection error
// impl fmt::Display for LookinGlassError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::TypeError(msg) => write!(f, "Type error: {}", msg),
//             Self::ValueError(msg) => write!(f, "Value error: {}", msg),
//             Self::FieldError(msg) => write!(f, "Field error: {}", msg),
//             Self::MethodError(msg) => write!(f, "Method error: {}", msg),
//             Self::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
//             Self::IndexError(msg) => write!(f, "Index error: {}", msg),
//             Self::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
//             Self::CannotSet(msg) => write!(f, "Cannot set: {}", msg),
//             Self::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
//             Self::JsonError(msg) => write!(f, "JSON error: {}", msg),
//             Self::ReflectionError(msg) => write!(f, "Reflection error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for LookinGlassError {}
// 
// impl From<LookinGlassError> for CursedError {
//     fn from(error: LookinGlassError) -> Self {
//         CursedError::RuntimeError(error.to_string())
//     }
// }

// impl From<serde_json::Error> for LookinGlassError {
//     fn from(error: serde_json::Error) -> Self {
//         LookinGlassError::JsonError(error.to_string())
//     }
// }

/// Helper functions for creating specific errors
pub fn type_error(msg: &str) -> LookinGlassError {
    LookinGlassError::TypeError(msg.to_string())
pub fn value_error(msg: &str) -> LookinGlassError {
    LookinGlassError::ValueError(msg.to_string())
pub fn field_error(msg: &str) -> LookinGlassError {
    LookinGlassError::FieldError(msg.to_string())
pub fn method_error(msg: &str) -> LookinGlassError {
    LookinGlassError::MethodError(msg.to_string())
pub fn conversion_error(msg: &str) -> LookinGlassError {
    LookinGlassError::ConversionError(msg.to_string())
pub fn index_error(msg: &str) -> LookinGlassError {
    LookinGlassError::IndexError(msg.to_string())
pub fn invalid_operation(msg: &str) -> LookinGlassError {
    LookinGlassError::InvalidOperation(msg.to_string())
pub fn cannot_set(msg: &str) -> LookinGlassError {
    LookinGlassError::CannotSet(msg.to_string())
pub fn type_mismatch(msg: &str) -> LookinGlassError {
    LookinGlassError::TypeMismatch(msg.to_string())
pub fn reflection_error(msg: &str) -> LookinGlassError {
    LookinGlassError::ReflectionError(msg.to_string())
