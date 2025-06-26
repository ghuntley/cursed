//! String manipulation module for CURSED programming language
//! 
//! This module provides comprehensive string operations including:
//! - Core operations (length, concatenation, reversal)  
//! - Search and replace functionality
//! - String transformations (case conversion, trimming)
//! - Splitting and joining utilities
//! - Character-level operations
//! - String validation and formatting
//! - Regular expression support

// Import and re-export the complete string module
pub use super::string::*;

// Compatibility functions to maintain existing API
use crate::error::CursedError;

/// Compatibility function for existing code
pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED string manipulation functions enabled".to_string())
}

/// Helper struct for backward compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MinimalImplementation {
    fn default() -> Self {
        Self::new()
    }
}
