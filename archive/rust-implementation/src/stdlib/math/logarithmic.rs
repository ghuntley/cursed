//! Logarithmic and exponential functions for CURSED

use super::{MathResult, MathError};

/// Natural logarithm
pub fn ln(x: f64) -> MathResult<f64> {
    if x <= 0.0 {
        Err(MathError::NegativeInput {
            function: "ln".to_string(),
            value: x,
        })
    } else {
        Ok(x.ln())
    }
}

/// Base-10 logarithm
pub fn log10(x: f64) -> MathResult<f64> {
    if x <= 0.0 {
        Err(MathError::NegativeInput {
            function: "log10".to_string(),
            value: x,
        })
    } else {
        Ok(x.log10())
    }
}

/// Base-2 logarithm
pub fn log2(x: f64) -> MathResult<f64> {
    if x <= 0.0 {
        Err(MathError::NegativeInput {
            function: "log2".to_string(),
            value: x,
        })
    } else {
        Ok(x.log2())
    }
}

/// Exponential function
pub fn exp(x: f64) -> f64 {
    x.exp()
}

/// 2^x
pub fn exp2(x: f64) -> f64 {
    x.exp2()
}

/// For backward compatibility
pub struct MinimalImplementation;
impl MinimalImplementation {
    pub fn new() -> Self { Self }
}
pub fn get_minimal_result() -> Result<String, crate::error::CursedError> {
    Ok("CURSED logarithmic functions enabled".to_string())
}
