//! Mathematical constants for CURSED

use std::f64::consts as std_consts;

/// Fundamental mathematical constants
pub const PI: f64 = std_consts::PI;
pub const E: f64 = std_consts::E;
pub const TAU: f64 = std_consts::TAU;
pub const SQRT_2: f64 = std_consts::SQRT_2;
pub const SQRT_3: f64 = 1.7320508075688772;
pub const LN_2: f64 = std_consts::LN_2;
pub const LN_10: f64 = std_consts::LN_10;
pub const LOG2_E: f64 = std_consts::LOG2_E;
pub const LOG10_E: f64 = std_consts::LOG10_E;

/// Physical constants
pub const GOLDEN_RATIO: f64 = 1.618033988749895;
pub const EULER_MASCHERONI: f64 = 0.5772156649015329;

/// Conversion constants
pub const DEGREES_TO_RADIANS: f64 = PI / 180.0;
pub const RADIANS_TO_DEGREES: f64 = 180.0 / PI;

/// Common fractions
pub const ONE_THIRD: f64 = 1.0 / 3.0;
pub const TWO_THIRDS: f64 = 2.0 / 3.0;
pub const ONE_SIXTH: f64 = 1.0 / 6.0;

/// Floating point limits
pub const EPSILON: f64 = f64::EPSILON;
pub const INFINITY: f64 = f64::INFINITY;
pub const NEG_INFINITY: f64 = f64::NEG_INFINITY;
pub const NAN: f64 = f64::NAN;

/// Utility functions
pub fn is_approximately_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

pub fn is_zero(x: f64) -> bool {
    x.abs() < EPSILON
}

/// For backward compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, crate::error::CursedError> {
    Ok("CURSED mathematical constants enabled".to_string())
}
