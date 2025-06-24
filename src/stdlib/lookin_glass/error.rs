/// Error handling for LookinGlass reflection package
use std::fmt;
use crate::error::CursedError;

/// Result type for reflection operations
pub type LookinGlassResult<T> = std::result::Result<T, LookinGlassError>;

/// Errors that can occur during reflection operations
#[derive(Debug, Clone)]
pub enum LookinGlassError {
    /// Type not found or invalid
    TypeError(String),
    /// Value cannot be reflected upon
    ValueError(String),
    /// Field access error
    FieldError(String),
    /// Method access error
    MethodError(String),
    /// Conversion error
    ConversionError(String),
    /// Index out of bounds
    IndexError(String),
    /// Invalid operation for this type
    InvalidOperation(String),
    /// Cannot set value (not addressable or settable)
    CannotSet(String),
    /// Type mismatch
    TypeMismatch(String),
    /// JSON serialization/deserialization error
    JsonError(String),
    /// General reflection error
    ReflectionError(String),
}

impl fmt::Display for LookinGlassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeError(msg) => write!(f, "Type error: {}", msg),
            Self::ValueError(msg) => write!(f, "Value error: {}", msg),
            Self::FieldError(msg) => write!(f, "Field error: {}", msg),
            Self::MethodError(msg) => write!(f, "Method error: {}", msg),
            Self::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
            Self::IndexError(msg) => write!(f, "Index error: {}", msg),
            Self::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            Self::CannotSet(msg) => write!(f, "Cannot set: {}", msg),
            Self::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            Self::JsonError(msg) => write!(f, "JSON error: {}", msg),
            Self::ReflectionError(msg) => write!(f, "Reflection error: {}", msg),
        }
    }
}

impl std::error::Error for LookinGlassError {}

impl From<LookinGlassError> for CursedError {
    fn from(error: LookinGlassError) -> Self {
        CursedError::RuntimeError(error.to_string())
    }
}

impl From<serde_json::Error> for LookinGlassError {
    fn from(error: serde_json::Error) -> Self {
        LookinGlassError::JsonError(error.to_string())
    }
}

/// Helper functions for creating specific errors
pub fn type_error(msg: &str) -> LookinGlassError {
    LookinGlassError::TypeError(msg.to_string())
}

pub fn value_error(msg: &str) -> LookinGlassError {
    LookinGlassError::ValueError(msg.to_string())
}

pub fn field_error(msg: &str) -> LookinGlassError {
    LookinGlassError::FieldError(msg.to_string())
}

pub fn method_error(msg: &str) -> LookinGlassError {
    LookinGlassError::MethodError(msg.to_string())
}

pub fn conversion_error(msg: &str) -> LookinGlassError {
    LookinGlassError::ConversionError(msg.to_string())
}

pub fn index_error(msg: &str) -> LookinGlassError {
    LookinGlassError::IndexError(msg.to_string())
}

pub fn invalid_operation(msg: &str) -> LookinGlassError {
    LookinGlassError::InvalidOperation(msg.to_string())
}

pub fn cannot_set(msg: &str) -> LookinGlassError {
    LookinGlassError::CannotSet(msg.to_string())
}

pub fn type_mismatch(msg: &str) -> LookinGlassError {
    LookinGlassError::TypeMismatch(msg.to_string())
}

pub fn reflection_error(msg: &str) -> LookinGlassError {
    LookinGlassError::ReflectionError(msg.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = type_error("invalid type");
        assert!(matches!(err, LookinGlassError::TypeError(_)));
        assert_eq!(err.to_string(), "Type error: invalid type");
    }

    #[test]
    fn test_error_conversion_to_cursed_error() {
        let lookin_glass_err = value_error("invalid value");
        let cursed_err: CursedError = lookin_glass_err.into();
        assert!(matches!(cursed_err, CursedError::RuntimeError(_)));
    }
}
