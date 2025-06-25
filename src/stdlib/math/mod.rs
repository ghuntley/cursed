/// Mathematics module for CURSED programming language
/// 
/// Provides comprehensive mathematical functions including basic arithmetic,
/// trigonometry, logarithms, special functions, constants, random number generation,
/// statistics, and advanced mathematical utilities.
/// 
/// This module integrates all mathematical capabilities into a unified, cohesive
/// library that's optimized for performance and ease of use.

use std::fmt;
use crate::error::CursedError;

// Core mathematical modules
pub mod basic;
pub mod trigonometry;
pub mod logarithmic;
pub mod constants;
pub mod special;
pub mod random;
pub mod statistics;
pub mod utilities;
pub mod complex;
pub mod advanced;
pub mod matrix;
pub mod big_mood;

// Re-export core mathematical functions with explicit imports to avoid conflicts
// BASIC OPERATIONS - Fundamental arithmetic and utility functions
pub use basic::{
    // Basic arithmetic operations
    // Rounding and precision functions
    // Integer arithmetic
    // Interpolation and smoothing
    // Type-specific operations
    // Power and utility functions
    // Statistical basics (for two values)
    // These functions exist in both basic and logarithmic modules - use basic for simple cases
// };

// TRIGONOMETRIC FUNCTIONS - Complete trigonometric operations
pub use trigonometry::{
    // Primary trigonometric functions
    // Hyperbolic functions
    // Angle conversion utilities
    // Degree-based convenience functions
    // Reciprocal trigonometric functions
    // Angle normalization
// };

// LOGARITHMIC & EXPONENTIAL FUNCTIONS - Advanced mathematical operations
pub use logarithmic::{
    // Core logarithmic functions
    // Enhanced exponential functions
    // Advanced power functions (primary implementations for complex operations)
    // Root functions (primary implementations)
    // Mathematical utility functions
    // Logarithmic transformations and utilities
    // Enhanced domain validation and safety
// };

// MATHEMATICAL CONSTANTS - Fundamental mathematical values
pub use constants::{
    // Primary constants
    // Pi-related constants
    // Square roots
    // Logarithmic constants
    // Special mathematical constants
    // Conversion factors
    // Physical constants
    // Additional physical constants
    // Floating point limits and properties
    // Common fractions
    // Unit conversion constants
    // Constant collections
    // Utility functions
// };

// RANDOM NUMBER GENERATION - Comprehensive random utilities
pub use random::{
    // Basic random functions
    // Collection utilities
    // String and byte generation
    // Seeding and control
    // Statistical distributions
// };

// STATISTICAL FUNCTIONS - Data analysis and statistics
pub use statistics::{
    // Descriptive statistics
    // Quantiles and percentiles
    // Range and spread measures
    // Probability density and distribution functions
    // Statistical analysis
    // Statistical tests
    // Regression analysis
    // Data validation and cleaning
// };

// SPECIAL FUNCTIONS - Advanced mathematical functions
pub use special::{
    // Factorial variants (special module implementations)
    // Gamma and beta functions
    // Combinatorial functions
    // Number sequences
    // CursedError functions
    // Bessel functions
// };

// MATHEMATICAL UTILITIES - Advanced computational mathematics
pub use utilities::{
    // Number theory
    // Combinatorics (primary implementations from utilities)
    // Advanced special functions
    // Numerical analysis methods
    // Sequences and series
    // Modular arithmetic and conversions
    // Advanced mathematical utilities
// };

// COMPLEX NUMBERS - Comprehensive complex number mathematics
pub use complex::{
    // Complex number type and creation
    // Basic complex operations
    // Complex exponential and logarithmic functions
    // Complex trigonometric functions
    // Complex inverse trigonometric functions
    // Complex vector and matrix operations
    // Complex polynomial operations
// };

// ADVANCED MATHEMATICAL FUNCTIONS - Sophisticated numerical methods and algorithms
pub use advanced::{
    // Advanced numerical methods (with prefixes to avoid conflicts)
    // Optimization algorithms
    // Fourier transforms and signal processing
    // Interpolation and approximation
    // Matrix operations
    // Mathematical modeling
// };

// MATRIX OPERATIONS - Linear algebra and matrix computations
pub use matrix::{
    // Matrix structure and creation
    // Matrix decompositions
    // Eigenvalue computations
    // Matrix utilities
// };

// BIG MOOD - Arbitrary precision arithmetic operations
pub use big_mood::{
    // Core arbitrary precision types
    // Accuracy and rounding control
    // Parsing functions
    // Utility functions
    // Mathematical functions for big numbers
    // Performance optimizations
// };

/// CursedError types for mathematical operations
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    /// Domain error: input value outside valid domain
    /// Range error: result would be outside representable range
    /// Overflow error: result too large to represent
    /// Underflow error: result too small to represent
    /// Division by zero
    /// Invalid input parameter
    /// Negative input to function requiring positive values
    /// Integer overflow in discrete math functions
    /// General mathematical computation error
// impl fmt::Display for MathError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             MathError::DomainError { function, value, message } => {
//                 write!(f, "Domain error in {}: value {} - {}", function, value, message)
//             }
//             MathError::RangeError { function, message } => {
//                 write!(f, "Range error in {}: {}", function, message)
//             }
//             MathError::Overflow { function, value } => {
//                 write!(f, "Overflow in {}: value {} is too large", function, value)
//             }
//             MathError::Underflow { function, value } => {
//                 write!(f, "Underflow in {}: value {} is too small", function, value)
//             }
//             MathError::DivisionByZero { function } => {
//                 write!(f, "Division by zero in {}", function)
//             }
//             MathError::InvalidInput { function, parameter, value } => {
//                 write!(f, "Invalid input in {}: parameter {} = {}", function, parameter, value)
//             }
//             MathError::NegativeInput { function, value } => {
//                 write!(f, "Negative input in {}: {} (positive value required)", function, value)
//             }
//             MathError::IntegerOverflow { function, value } => {
//                 write!(f, "Integer overflow in {}: value {}", function, value)
//             }
//             MathError::ComputationError { function, message } => {
//                 write!(f, "Computation error in {}: {}", function, message)
//             }
//         }
//     }
// }

// impl std::error::CursedError for MathError {}
// 
/// Result type for mathematical operations
pub type MathResult<T> = std::result::Result<T, MathError>;

/// Helper function to create domain errors
pub fn domain_error(function: &str, value: f64, message: &str) -> MathError {
    MathError::DomainError {
    }
}

/// Helper function to create range errors
pub fn range_error(function: &str, message: &str) -> MathError {
    MathError::RangeError {
    }
}

/// Helper function to create division by zero errors
pub fn division_by_zero_error(function: &str) -> MathError {
    MathError::DivisionByZero {
    }
}

/// Helper function to create negative input errors
pub fn negative_input_error(function: &str, value: f64) -> MathError {
    MathError::NegativeInput {
    }
}

/// Helper function to check if a floating point value is valid (not NaN or infinite)
pub fn is_valid_float(value: f64) -> bool {
    value.is_finite()
/// Helper function to validate floating point inputs
pub fn validate_float(function: &str, parameter: &str, value: f64) -> MathResult<()> {
    if value.is_nan() {
        Err(MathError::InvalidInput {
        })
    } else if value.is_infinite() {
        Err(MathError::RangeError {
        })
    } else {
        Ok(())
    }
}
