//! Basic mathematical operations for CURSED

use super::{MathResult, MathError};

/// Basic arithmetic operations
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> MathResult<f64> {
    if b == 0.0 {
        Err(MathError::DivisionByZero {
            function: "divide".to_string(),
        })
    } else {
        Ok(a / b)
    }
}

/// Power function
pub fn power(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

/// Square root
pub fn sqrt(x: f64) -> MathResult<f64> {
    if x < 0.0 {
        Err(MathError::NegativeInput {
            function: "sqrt".to_string(),
            value: x,
        })
    } else {
        Ok(x.sqrt())
    }
}

/// Absolute value
pub fn abs(x: f64) -> f64 {
    x.abs()
}

/// Rounding functions
pub fn round(x: f64) -> f64 {
    x.round()
}

pub fn floor(x: f64) -> f64 {
    x.floor()
}

pub fn ceil(x: f64) -> f64 {
    x.ceil()
}

/// Minimum and maximum
pub fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}

pub fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}

/// For backward compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, crate::error::CursedError> {
    Ok("CURSED basic math functions enabled".to_string())
}
