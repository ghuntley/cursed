//! Documentation extraction and analysis system for the CURSED language
//!
//! This module provides comprehensive documentation analysis capabilities including:
//! - AST-based documentation extraction from source code
//! - Documentation completeness and accuracy analysis
//! - Cross-reference validation and link checking
//! - Example code syntax validation
//! - Coverage reporting for documented vs undocumented code

pub mod extractor;
pub mod analyzer;

// Re-export main public API
pub use extractor::{DocumentationExtractor, DocumentationItem, ExtractionResult};
pub use analyzer::{DocumentationAnalyzer, AnalysisResult, CoverageReport, ValidationIssue};

use crate::error::{Error, SourceLocation};

/// Documentation-specific error type
#[derive(Debug, Clone)]
pub enum DocumentationError {
    /// Error extracting documentation from AST
    ExtractionError {
        location: SourceLocation,
        message: String,
    },
    /// Documentation validation error
    ValidationError {
        location: SourceLocation,
        message: String,
    },
    /// Cross-reference resolution error
    LinkError {
        location: SourceLocation,
        reference: String,
        message: String,
    },
    /// Example code syntax error
    ExampleError {
        location: SourceLocation,
        code: String,
        message: String,
    },
}

impl std::fmt::Display for DocumentationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentationError::ExtractionError { location, message } => {
                write!(f, "Documentation extraction error at {}: {}", location, message)
            }
            DocumentationError::ValidationError { location, message } => {
                write!(f, "Documentation validation error at {}: {}", location, message)
            }
            DocumentationError::LinkError { location, reference, message } => {
                write!(f, "Link error at {} for '{}': {}", location, reference, message)
            }
            DocumentationError::ExampleError { location, code, message } => {
                write!(f, "Example code error at {}: {} in code:\n{}", location, message, code)
            }
        }
    }
}

impl std::error::Error for DocumentationError {}

impl From<DocumentationError> for Error {
    fn from(err: DocumentationError) -> Self {
        match err {
            DocumentationError::ExtractionError { location, message } => {
                Error::Parser { location, message }
            }
            DocumentationError::ValidationError { location, message } => {
                Error::Type { location, message }
            }
            DocumentationError::LinkError { location, message, .. } => {
                Error::Type { location, message }
            }
            DocumentationError::ExampleError { location, message, .. } => {
                Error::Type { location, message }
            }
        }
    }
}

/// Result type for documentation operations
pub type DocumentationResult<T> = Result<T, DocumentationError>;
