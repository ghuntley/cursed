//! Error handling for CURSED
//!
//! This module collects various error-related modules and types.

pub mod type_assertion_error;

// Re-export the primary type assertion error type
pub use type_assertion_error::TypeAssertionError;

// Re-export the error location type from the parent module
pub use crate::error::SourceLocation;

// Re-export helper functions from the error module
pub use crate::error::ErrorReporter;