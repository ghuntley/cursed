/// Mathematics module for CURSED programming language
/// 
/// Provides comprehensive mathematical functions including basic arithmetic,
/// trigonometry, logarithms, special functions, constants, and random number generation.

use std::fmt;

pub mod basic;
pub mod trigonometry;
pub mod logarithmic;
pub mod constants;
pub mod special;
pub mod random;

// Re-export all mathematical functions for easy access
pub use basic::*;
pub use trigonometry::*;
pub use logarithmic::*;
pub use constants::*;
pub use special::*;
pub use random::*;

/// Error types for mathematical operations
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    /// Domain error: input value outside valid domain
    DomainError { function: String, value: f64, message: String },
    /// Range error: result would be outside representable range
    RangeError { function: String, message: String },
    /// Overflow error: result too large to represent
    Overflow { function: String, value: f64 },
    /// Underflow error: result too small to represent
    Underflow { function: String, value: f64 },
    /// Division by zero
    DivisionByZero { function: String },
    /// Invalid input parameter
    InvalidInput { function: String, parameter: String, value: f64 },
    /// Negative input to function requiring positive values
    NegativeInput { function: String, value: f64 },
    /// Integer overflow in discrete math functions
    IntegerOverflow { function: String, value: i64 },
    /// General mathematical computation error
    ComputationError { function: String, message: String },
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DomainError { function, value, message } => {
                write!(f, "Domain error in {}: value {} - {}", function, value, message)
            }
            MathError::RangeError { function, message } => {
                write!(f, "Range error in {}: {}", function, message)
            }
            MathError::Overflow { function, value } => {
                write!(f, "Overflow in {}: value {} is too large", function, value)
            }
            MathError::Underflow { function, value } => {
                write!(f, "Underflow in {}: value {} is too small", function, value)
            }
            MathError::DivisionByZero { function } => {
                write!(f, "Division by zero in {}", function)
            }
            MathError::InvalidInput { function, parameter, value } => {
                write!(f, "Invalid input in {}: parameter {} = {}", function, parameter, value)
            }
            MathError::NegativeInput { function, value } => {
                write!(f, "Negative input in {}: {} (positive value required)", function, value)
            }
            MathError::IntegerOverflow { function, value } => {
                write!(f, "Integer overflow in {}: value {}", function, value)
            }
            MathError::ComputationError { function, message } => {
                write!(f, "Computation error in {}: {}", function, message)
            }
        }
    }
}

impl std::error::Error for MathError {}

/// Result type for mathematical operations
pub type MathResult<T> = Result<T, MathError>;

/// Helper function to create domain errors
pub fn domain_error(function: &str, value: f64, message: &str) -> MathError {
    MathError::DomainError {
        function: function.to_string(),
        value,
        message: message.to_string(),
    }
}

/// Helper function to create range errors
pub fn range_error(function: &str, message: &str) -> MathError {
    MathError::RangeError {
        function: function.to_string(),
        message: message.to_string(),
    }
}

/// Helper function to create division by zero errors
pub fn division_by_zero_error(function: &str) -> MathError {
    MathError::DivisionByZero {
        function: function.to_string(),
    }
}

/// Helper function to create negative input errors
pub fn negative_input_error(function: &str, value: f64) -> MathError {
    MathError::NegativeInput {
        function: function.to_string(),
        value,
    }
}

/// Helper function to check if a floating point value is valid (not NaN or infinite)
pub fn is_valid_float(value: f64) -> bool {
    value.is_finite()
}

/// Helper function to validate floating point inputs
pub fn validate_float(function: &str, parameter: &str, value: f64) -> MathResult<()> {
    if value.is_nan() {
        Err(MathError::InvalidInput {
            function: function.to_string(),
            parameter: parameter.to_string(),
            value,
        })
    } else if value.is_infinite() {
        Err(MathError::RangeError {
            function: function.to_string(),
            message: format!("Parameter {} is infinite", parameter),
        })
    } else {
        Ok(())
    }
}
