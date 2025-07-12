//! Error type definitions and conversions for CURSED

use super::structured::{StructuredError, ErrorCode};
use super::CursedError;

/// Enhanced error type that can contain both legacy and structured errors
#[derive(Debug, Clone)]
pub enum EnhancedError {
    Legacy(CursedError),
    Structured(StructuredError),
}

impl From<CursedError> for EnhancedError {
    fn from(error: CursedError) -> Self {
        EnhancedError::Legacy(error)
    }
}

impl From<StructuredError> for EnhancedError {
    fn from(error: StructuredError) -> Self {
        EnhancedError::Structured(error)
    }
}

impl From<CursedError> for StructuredError {
    fn from(error: CursedError) -> Self {
        match error {
            CursedError::SyntaxError(msg) => StructuredError::syntax_error(&msg),
            CursedError::TypeError(msg) => StructuredError::type_error(&msg),
            CursedError::CompilerError(msg) => StructuredError::compile_error(&msg),
            CursedError::RuntimeError(msg) => StructuredError::runtime_error(&msg),
            CursedError::Parse(msg) => StructuredError::new(ErrorCode::E0001, msg),
            CursedError::ImportError(msg) => StructuredError::new(ErrorCode::E0200, msg),
            CursedError::Io(msg) => StructuredError::new(ErrorCode::E0500, msg),
            CursedError::ValidationError(msg) => StructuredError::new(ErrorCode::E0100, msg),
            CursedError::OptimizationError(msg) => StructuredError::new(ErrorCode::E0208, msg),
            CursedError::UnsupportedAlgorithm(msg) => StructuredError::new(ErrorCode::E0405, msg),
            CursedError::RandomGenerationFailed(msg) => StructuredError::new(ErrorCode::E0405, msg),
            CursedError::InternalError(msg) => StructuredError::new(ErrorCode::E0207, msg),
            CursedError::CollectionsError(msg) => StructuredError::new(ErrorCode::E0309, msg),
            CursedError::StringError(msg) => StructuredError::new(ErrorCode::E0309, msg),
            CursedError::General(msg) => StructuredError::new(ErrorCode::E0309, msg),
            CursedError::InvalidOptimizationLevel(msg) => StructuredError::new(ErrorCode::E0208, msg),
            CursedError::ConfigError(msg) => StructuredError::new(ErrorCode::E0208, msg),
            CursedError::IoError(msg) => StructuredError::new(ErrorCode::E0500, msg),
            CursedError::ParseError(msg) => StructuredError::new(ErrorCode::E0001, msg),
            CursedError::SerializationError(msg) => StructuredError::new(ErrorCode::E0309, msg),
            CursedError::FamRecovery(msg) => StructuredError::new(ErrorCode::E0500, msg),
            CursedError::MemoryError(msg) => StructuredError::new(ErrorCode::E0500, msg),
        }
    }
}

impl From<StructuredError> for CursedError {
    fn from(error: StructuredError) -> Self {
        match error.code.category() {
            "Syntax" => CursedError::SyntaxError(error.message),
            "Type" => CursedError::TypeError(error.message),
            "Compilation" => CursedError::CompilerError(error.message),
            "Runtime" => CursedError::RuntimeError(error.message),
            "Security" => CursedError::RuntimeError(error.message),
            "I/O" => CursedError::Io(error.message),
            _ => CursedError::General(error.message),
        }
    }
}

impl std::fmt::Display for EnhancedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnhancedError::Legacy(err) => write!(f, "{}", err),
            EnhancedError::Structured(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for EnhancedError {}

/// Result type using enhanced errors
pub type Result<T> = std::result::Result<T, EnhancedError>;

/// Convenience functions for creating structured errors with common patterns
pub fn syntax_error_with_location(message: &str, line: usize, column: usize, source: Option<String>) -> StructuredError {
    let mut error = StructuredError::syntax_error(message);
    error.location = Some(super::structured::ErrorSourceLocation {
        file: "".to_string(),
        line,
        column,
        length: 1,
        source_line: source,
    });
    error
}

pub fn type_error_with_suggestion(message: &str, suggestion: &str) -> StructuredError {
    StructuredError::type_error(message)
        .with_suggestions(vec![suggestion.to_string()])
}

pub fn compile_error_with_context(message: &str, context: Vec<String>) -> StructuredError {
    StructuredError::compile_error(message)
        .with_context(context)
}
