//! Trigonometric functions for CURSED

use super::{MathResult, constants::*};

/// Basic trigonometric functions
pub fn sin(x: f64) -> f64 {
    x.sin()
}

pub fn cos(x: f64) -> f64 {
    x.cos()
}

pub fn tan(x: f64) -> f64 {
    x.tan()
}

/// Inverse trigonometric functions
pub fn asin(x: f64) -> MathResult<f64> {
    if x < -1.0 || x > 1.0 {
        Err(super::MathError::DomainError {
            function: "asin".to_string(),
            value: x,
            message: "Value must be in range [-1, 1]".to_string(),
        })
    } else {
        Ok(x.asin())
    }
}

pub fn acos(x: f64) -> MathResult<f64> {
    if x < -1.0 || x > 1.0 {
        Err(super::MathError::DomainError {
            function: "acos".to_string(),
            value: x,
            message: "Value must be in range [-1, 1]".to_string(),
        })
    } else {
        Ok(x.acos())
    }
}

pub fn atan(x: f64) -> f64 {
    x.atan()
}

pub fn atan2(y: f64, x: f64) -> f64 {
    y.atan2(x)
}

/// Hyperbolic functions
pub fn sinh(x: f64) -> f64 {
    x.sinh()
}

pub fn cosh(x: f64) -> f64 {
    x.cosh()
}

pub fn tanh(x: f64) -> f64 {
    x.tanh()
}

/// Degree conversion functions
pub fn sin_deg(degrees: f64) -> f64 {
    sin(degrees * DEGREES_TO_RADIANS)
}

pub fn cos_deg(degrees: f64) -> f64 {
    cos(degrees * DEGREES_TO_RADIANS)
}

pub fn tan_deg(degrees: f64) -> f64 {
    tan(degrees * DEGREES_TO_RADIANS)
}

/// Angle normalization
pub fn normalize_radians(angle: f64) -> f64 {
    let two_pi = 2.0 * PI;
    angle - two_pi * (angle / two_pi).floor()
}

pub fn normalize_degrees(angle: f64) -> f64 {
    angle - 360.0 * (angle / 360.0).floor()
}

/// For backward compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, crate::error::CursedError> {
    Ok("CURSED trigonometric functions enabled".to_string())
}
