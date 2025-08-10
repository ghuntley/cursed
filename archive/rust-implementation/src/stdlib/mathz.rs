//! Mathematics module for CURSED programming language
//! 
//! This module provides comprehensive mathematical functions including basic arithmetic,
//! trigonometry, logarithms, special functions, constants, random number generation,
//! statistics, and advanced mathematical utilities.

// Import and re-export the complete math module
pub use super::math::*;

// Compatibility functions to maintain existing API
use crate::error::CursedError;

/// Compatibility function for existing code
pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED mathematical functions enabled".to_string())
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
