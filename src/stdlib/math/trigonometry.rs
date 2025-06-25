use crate::error::CursedError;
/// Comprehensive trigonometric functions for CURSED programming language
/// 
/// This module provides all standard trigonometric and hyperbolic functions with
/// proper domain validation, special angle optimizations, and comprehensive error handling.

use super::{MathError, MathResult, validate_float, domain_error};
use super::constants::{PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, SQRT_2, SQRT_3};

/// Sine function with special angle optimizations
pub fn sin(x: f64) -> MathResult<f64> {
    validate_float("sin", "x", x)?;
    
    // Handle special angles for improved precision
    let normalized = normalize_angle_signed(x)?;
    
    // Check for common angles with exact values
    if (normalized.abs() - 0.0).abs() < 1e-15 {
        return Ok(0.0);
    }
    if (normalized.abs() - FRAC_PI_2).abs() < 1e-15 {
        return Ok(if normalized > 0.0 { 1.0 } else { -1.0 });
    }
    if (normalized.abs() - PI).abs() < 1e-15 {
        return Ok(0.0);
    }
    if (normalized.abs() - FRAC_PI_6).abs() < 1e-15 {
        return Ok(if normalized > 0.0 { 0.5 } else { -0.5 });
    }
    if (normalized.abs() - FRAC_PI_4).abs() < 1e-15 {
        return Ok(if normalized > 0.0 { SQRT_2 / 2.0 } else { -SQRT_2 / 2.0 });
    }
    if (normalized.abs() - FRAC_PI_3).abs() < 1e-15 {
        return Ok(if normalized > 0.0 { SQRT_3 / 2.0 } else { -SQRT_3 / 2.0 });
    }
    
    Ok(x.sin())
}

/// Cosine function with special angle optimizations
pub fn cos(x: f64) -> MathResult<f64> {
    validate_float("cos", "x", x)?;
    
    // Handle special angles for improved precision
    let normalized = normalize_angle_signed(x)?;
    
    // Check for common angles with exact values
    if (normalized.abs() - 0.0).abs() < 1e-15 {
        return Ok(1.0);
    }
    if (normalized.abs() - FRAC_PI_2).abs() < 1e-15 {
        return Ok(0.0);
    }
    if (normalized.abs() - PI).abs() < 1e-15 {
        return Ok(-1.0);
    }
    if (normalized.abs() - FRAC_PI_6).abs() < 1e-15 {
        return Ok(SQRT_3 / 2.0);
    }
    if (normalized.abs() - FRAC_PI_4).abs() < 1e-15 {
        return Ok(SQRT_2 / 2.0);
    }
    if (normalized.abs() - FRAC_PI_3).abs() < 1e-15 {
        return Ok(0.5);
    }
    
    Ok(x.cos())
}

/// Tangent function with enhanced domain checking and special angle optimizations
pub fn tan(x: f64) -> MathResult<f64> {
    validate_float("tan", "x", x)?;
    
    // Handle special angles for improved precision
    let normalized = normalize_angle_signed(x)?;
    
    // Check for undefined values at odd multiples of π/2
    let half_pi_multiple = normalized / FRAC_PI_2;
    if (half_pi_multiple % 2.0 - 1.0).abs() < 1e-15 || (half_pi_multiple % 2.0 + 1.0).abs() < 1e-15 {
        return Err(domain_error("tan", x, "tangent undefined at odd multiples of π/2"));
    }
    
    // Check for common angles with exact values
    if (normalized.abs() - 0.0).abs() < 1e-15 {
        return Ok(0.0);
    }
    if (normalized.abs() - PI).abs() < 1e-15 {
        return Ok(0.0);
    }
    if (normalized.abs() - FRAC_PI_6).abs() < 1e-15 {
        return Ok(if normalized > 0.0 { 1.0 / SQRT_3 } else { -1.0 / SQRT_3 });
    }
    if (normalized.abs() - FRAC_PI_4).abs() < 1e-15 {
        return Ok(if normalized > 0.0 { 1.0 } else { -1.0 });
    }
    if (normalized.abs() - FRAC_PI_3).abs() < 1e-15 {
        return Ok(if normalized > 0.0 { SQRT_3 } else { -SQRT_3 });
    }
    
    Ok(x.tan())
}

/// Arcsine function (inverse sine) with enhanced domain validation
pub fn asin(x: f64) -> MathResult<f64> {
    validate_unit_domain("asin", x)?;
    
    // Handle special values for improved precision
    if x == 0.0 {
        return Ok(0.0);
    }
    if x == 1.0 {
        return Ok(FRAC_PI_2);
    }
    if x == -1.0 {
        return Ok(-FRAC_PI_2);
    }
    if x == 0.5 {
        return Ok(FRAC_PI_6);
    }
    if x == -0.5 {
        return Ok(-FRAC_PI_6);
    }
    if (x - SQRT_2 / 2.0).abs() < f64::EPSILON {
        return Ok(FRAC_PI_4);
    }
    if (x + SQRT_2 / 2.0).abs() < f64::EPSILON {
        return Ok(-FRAC_PI_4);
    }
    if (x - SQRT_3 / 2.0).abs() < f64::EPSILON {
        return Ok(FRAC_PI_3);
    }
    if (x + SQRT_3 / 2.0).abs() < f64::EPSILON {
        return Ok(-FRAC_PI_3);
    }
    
    Ok(x.asin())
}

/// Arccosine function (inverse cosine) with enhanced domain validation
pub fn acos(x: f64) -> MathResult<f64> {
    validate_unit_domain("acos", x)?;
    
    // Handle special values for improved precision
    if x == 1.0 {
        return Ok(0.0);
    }
    if x == -1.0 {
        return Ok(PI);
    }
    if x == 0.0 {
        return Ok(FRAC_PI_2);
    }
    if x == 0.5 {
        return Ok(FRAC_PI_3);
    }
    if x == -0.5 {
        return Ok(2.0 * FRAC_PI_3);
    }
    if (x - SQRT_2 / 2.0).abs() < f64::EPSILON {
        return Ok(FRAC_PI_4);
    }
    if (x + SQRT_2 / 2.0).abs() < f64::EPSILON {
        return Ok(3.0 * FRAC_PI_4);
    }
    if (x - SQRT_3 / 2.0).abs() < f64::EPSILON {
        return Ok(FRAC_PI_6);
    }
    if (x + SQRT_3 / 2.0).abs() < f64::EPSILON {
        return Ok(5.0 * FRAC_PI_6);
    }
    
    Ok(x.acos())
}

/// Arctangent function (inverse tangent)
pub fn atan(x: f64) -> MathResult<f64> {
    validate_float("atan", "x", x)?;
    Ok(x.atan())
}

/// Two-argument arctangent function
pub fn atan2(y: f64, x: f64) -> MathResult<f64> {
    validate_float("atan2", "y", y)?;
    validate_float("atan2", "x", x)?;
    
    if x == 0.0 && y == 0.0 {
        return Err(domain_error("atan2", 0.0, "both arguments cannot be zero"));
    }
    
    Ok(y.atan2(x))
}

/// Hyperbolic sine function
pub fn sinh(x: f64) -> MathResult<f64> {
    validate_float("sinh", "x", x)?;
    
    let result = x.sinh();
    if !result.is_finite() {
        return Err(MathError::Overflow {
            function: "sinh".to_string(),
            value: x,
        });
    }
    
    Ok(result)
}

/// Hyperbolic cosine function
pub fn cosh(x: f64) -> MathResult<f64> {
    validate_float("cosh", "x", x)?;
    
    let result = x.cosh();
    if !result.is_finite() {
        return Err(MathError::Overflow {
            function: "cosh".to_string(),
            value: x,
        });
    }
    
    Ok(result)
}

/// Hyperbolic tangent function
pub fn tanh(x: f64) -> MathResult<f64> {
    validate_float("tanh", "x", x)?;
    Ok(x.tanh())
}

/// Inverse hyperbolic sine function
pub fn asinh(x: f64) -> MathResult<f64> {
    validate_float("asinh", "x", x)?;
    Ok(x.asinh())
}

/// Inverse hyperbolic cosine function with enhanced domain validation
pub fn acosh(x: f64) -> MathResult<f64> {
    validate_acosh_domain(x)?;
    
    // Handle special values
    if x == 1.0 {
        return Ok(0.0);
    }
    
    Ok(x.acosh())
}

/// Inverse hyperbolic tangent function with enhanced domain validation
pub fn atanh(x: f64) -> MathResult<f64> {
    validate_atanh_domain(x)?;
    
    // Handle special values
    if x == 0.0 {
        return Ok(0.0);
    }
    
    Ok(x.atanh())
}

/// Convert degrees to radians
pub fn degrees_to_radians(degrees: f64) -> MathResult<f64> {
    validate_float("degrees_to_radians", "degrees", degrees)?;
    Ok(degrees * PI / 180.0)
}

/// Convert radians to degrees
pub fn radians_to_degrees(radians: f64) -> MathResult<f64> {
    validate_float("radians_to_degrees", "radians", radians)?;
    Ok(radians * 180.0 / PI)
}

/// Convenient alias for degrees_to_radians
pub fn deg_to_rad(degrees: f64) -> MathResult<f64> {
    degrees_to_radians(degrees)
}

/// Convenient alias for radians_to_degrees
pub fn rad_to_deg(radians: f64) -> MathResult<f64> {
    radians_to_degrees(radians)
}

/// Sine function with input in degrees
pub fn sin_deg(degrees: f64) -> MathResult<f64> {
    let radians = degrees_to_radians(degrees)?;
    sin(radians)
}

/// Cosine function with input in degrees
pub fn cos_deg(degrees: f64) -> MathResult<f64> {
    let radians = degrees_to_radians(degrees)?;
    cos(radians)
}

/// Tangent function with input in degrees
pub fn tan_deg(degrees: f64) -> MathResult<f64> {
    let radians = degrees_to_radians(degrees)?;
    tan(radians)
}

/// Secant function
pub fn sec(x: f64) -> MathResult<f64> {
    validate_float("sec", "x", x)?;
    
    let cos_val = cos(x)?;
    if cos_val.abs() < f64::EPSILON {
        return Err(domain_error("sec", x, "secant undefined when cosine is zero"));
    }
    
    Ok(1.0 / cos_val)
}

/// Cosecant function
pub fn csc(x: f64) -> MathResult<f64> {
    validate_float("csc", "x", x)?;
    
    let sin_val = sin(x)?;
    if sin_val.abs() < f64::EPSILON {
        return Err(domain_error("csc", x, "cosecant undefined when sine is zero"));
    }
    
    Ok(1.0 / sin_val)
}

/// Cotangent function
pub fn cot(x: f64) -> MathResult<f64> {
    validate_float("cot", "x", x)?;
    
    let tan_val = tan(x)?;
    if tan_val.abs() < f64::EPSILON {
        return Err(domain_error("cot", x, "cotangent undefined when tangent is zero"));
    }
    
    Ok(1.0 / tan_val)
}

/// Normalize angle to range [0, 2π)
pub fn normalize_angle(angle: f64) -> MathResult<f64> {
    validate_float("normalize_angle", "angle", angle)?;
    
    let two_pi = 2.0 * PI;
    let mut normalized = angle % two_pi;
    
    if normalized < 0.0 {
        normalized += two_pi;
    }
    
    Ok(normalized)
}

/// Normalize angle to range [-π, π)
pub fn normalize_angle_signed(angle: f64) -> MathResult<f64> {
    validate_float("normalize_angle_signed", "angle", angle)?;
    
    let two_pi = 2.0 * PI;
    let mut normalized = angle % two_pi;
    
    if normalized > PI {
        normalized -= two_pi;
    } else if normalized <= -PI {
        normalized += two_pi;
    }
    
    Ok(normalized)
}

// ========== SPECIAL ANGLE VALUES AND UTILITIES ==========

/// Get sine value for common angles (returns None if not a special angle)
pub fn sin_special_angle(radians: f64) -> Option<f64> {
    let normalized = radians % (2.0 * PI);
    let angle = if normalized < 0.0 { normalized + 2.0 * PI } else { normalized };
    
    const EPSILON: f64 = 1e-15;
    
    if (angle - 0.0).abs() < EPSILON || (angle - PI).abs() < EPSILON || (angle - 2.0 * PI).abs() < EPSILON {
        Some(0.0)
    } else if (angle - FRAC_PI_2).abs() < EPSILON {
        Some(1.0)
    } else if (angle - 3.0 * FRAC_PI_2).abs() < EPSILON {
        Some(-1.0)
    } else if (angle - FRAC_PI_6).abs() < EPSILON || (angle - 5.0 * FRAC_PI_6).abs() < EPSILON {
        Some(0.5)
    } else if (angle - 7.0 * FRAC_PI_6).abs() < EPSILON || (angle - 11.0 * FRAC_PI_6).abs() < EPSILON {
        Some(-0.5)
    } else if (angle - FRAC_PI_4).abs() < EPSILON || (angle - 3.0 * FRAC_PI_4).abs() < EPSILON {
        Some(SQRT_2 / 2.0)
    } else if (angle - 5.0 * FRAC_PI_4).abs() < EPSILON || (angle - 7.0 * FRAC_PI_4).abs() < EPSILON {
        Some(-SQRT_2 / 2.0)
    } else if (angle - FRAC_PI_3).abs() < EPSILON || (angle - 2.0 * FRAC_PI_3).abs() < EPSILON {
        Some(SQRT_3 / 2.0)
    } else if (angle - 4.0 * FRAC_PI_3).abs() < EPSILON || (angle - 5.0 * FRAC_PI_3).abs() < EPSILON {
        Some(-SQRT_3 / 2.0)
    } else {
        None
    }
}

/// Get cosine value for common angles (returns None if not a special angle)
pub fn cos_special_angle(radians: f64) -> Option<f64> {
    let normalized = radians % (2.0 * PI);
    let angle = if normalized < 0.0 { normalized + 2.0 * PI } else { normalized };
    
    const EPSILON: f64 = 1e-15;
    
    if (angle - 0.0).abs() < EPSILON || (angle - 2.0 * PI).abs() < EPSILON {
        Some(1.0)
    } else if (angle - FRAC_PI_2).abs() < EPSILON || (angle - 3.0 * FRAC_PI_2).abs() < EPSILON {
        Some(0.0)
    } else if (angle - PI).abs() < EPSILON {
        Some(-1.0)
    } else if (angle - FRAC_PI_6).abs() < EPSILON || (angle - 11.0 * FRAC_PI_6).abs() < EPSILON {
        Some(SQRT_3 / 2.0)
    } else if (angle - 5.0 * FRAC_PI_6).abs() < EPSILON || (angle - 7.0 * FRAC_PI_6).abs() < EPSILON {
        Some(-SQRT_3 / 2.0)
    } else if (angle - FRAC_PI_4).abs() < EPSILON || (angle - 7.0 * FRAC_PI_4).abs() < EPSILON {
        Some(SQRT_2 / 2.0)
    } else if (angle - 3.0 * FRAC_PI_4).abs() < EPSILON || (angle - 5.0 * FRAC_PI_4).abs() < EPSILON {
        Some(-SQRT_2 / 2.0)
    } else if (angle - FRAC_PI_3).abs() < EPSILON || (angle - 5.0 * FRAC_PI_3).abs() < EPSILON {
        Some(0.5)
    } else if (angle - 2.0 * FRAC_PI_3).abs() < EPSILON || (angle - 4.0 * FRAC_PI_3).abs() < EPSILON {
        Some(-0.5)
    } else {
        None
    }
}

/// Check if an angle (in radians) is a special angle with exact trigonometric values
pub fn is_special_angle(radians: f64) -> bool {
    sin_special_angle(radians).is_some()
}

/// Convert degrees to radians for special degree values (0, 30, 45, 60, 90, etc.)
pub fn special_degrees_to_radians(degrees: f64) -> Option<f64> {
    let normalized = degrees % 360.0;
    let angle = if normalized < 0.0 { normalized + 360.0 } else { normalized };
    
    match angle as i32 {
        0 | 360 => Some(0.0),
        30 => Some(FRAC_PI_6),
        45 => Some(FRAC_PI_4),
        60 => Some(FRAC_PI_3),
        90 => Some(FRAC_PI_2),
        120 => Some(2.0 * FRAC_PI_3),
        135 => Some(3.0 * FRAC_PI_4),
        150 => Some(5.0 * FRAC_PI_6),
        180 => Some(PI),
        210 => Some(7.0 * FRAC_PI_6),
        225 => Some(5.0 * FRAC_PI_4),
        240 => Some(4.0 * FRAC_PI_3),
        270 => Some(3.0 * FRAC_PI_2),
        300 => Some(5.0 * FRAC_PI_3),
        315 => Some(7.0 * FRAC_PI_4),
        330 => Some(11.0 * FRAC_PI_6),
        _ => None,
    }
}

// ========== ENHANCED DOMAIN VALIDATION ==========

/// Validate domain for inverse trig functions [-1, 1]
pub fn validate_unit_domain(function: &str, x: f64) -> MathResult<()> {
    validate_float(function, "x", x)?;
    if x < -1.0 || x > 1.0 {
        Err(domain_error(function, x, "input must be in range [-1, 1]"))
    } else {
        Ok(())
    }
}

/// Validate domain for inverse hyperbolic cosine [1, ∞)
pub fn validate_acosh_domain(x: f64) -> MathResult<()> {
    validate_float("acosh", "x", x)?;
    if x < 1.0 {
        Err(domain_error("acosh", x, "input must be >= 1"))
    } else {
        Ok(())
    }
}

/// Validate domain for inverse hyperbolic tangent (-1, 1)
pub fn validate_atanh_domain(x: f64) -> MathResult<()> {
    validate_float("atanh", "x", x)?;
    if x <= -1.0 || x >= 1.0 {
        Err(domain_error("atanh", x, "input must be in range (-1, 1)"))
    } else {
        Ok(())
    }
}

// ========== ADVANCED TRIGONOMETRIC FUNCTIONS ==========

/// Versine function: versin(x) = 1 - cos(x)
pub fn versin(x: f64) -> MathResult<f64> {
    let cos_val = cos(x)?;
    Ok(1.0 - cos_val)
}

/// Coversine function: coversin(x) = 1 - sin(x) 
pub fn coversin(x: f64) -> MathResult<f64> {
    let sin_val = sin(x)?;
    Ok(1.0 - sin_val)
}

/// Haversine function: haversin(x) = sin²(x/2) = (1 - cos(x))/2
pub fn haversin(x: f64) -> MathResult<f64> {
    let cos_val = cos(x)?;
    Ok((1.0 - cos_val) / 2.0)
}

/// Exsecant function: exsec(x) = sec(x) - 1
pub fn exsec(x: f64) -> MathResult<f64> {
    let sec_val = sec(x)?;
    Ok(sec_val - 1.0)
}

/// Excosecant function: excsc(x) = csc(x) - 1
pub fn excsc(x: f64) -> MathResult<f64> {
    let csc_val = csc(x)?;
    Ok(csc_val - 1.0)
}

/// Chord function: chord(x) = 2 * sin(x/2)
pub fn chord(x: f64) -> MathResult<f64> {
    let sin_val = sin(x / 2.0)?;
    Ok(2.0 * sin_val)
}

// ========== COMBINED OPERATIONS ==========

/// Sine and cosine computed together for efficiency
pub fn sincos(x: f64) -> MathResult<(f64, f64)> {
    validate_float("sincos", "x", x)?;
    
    // Check for special angles first
    if let (Some(sin_val), Some(cos_val)) = (sin_special_angle(x), cos_special_angle(x)) {
        return Ok((sin_val, cos_val));
    }
    
    Ok((x.sin(), x.cos()))
}

/// Compute all six trigonometric functions at once
pub fn trig_all(x: f64) -> MathResult<(f64, f64, f64, f64, f64, f64)> {
    let (sin_val, cos_val) = sincos(x)?;
    
    // Check for division by zero
    if cos_val.abs() < f64::EPSILON {
        return Err(domain_error("trig_all", x, "cosine is zero (tan, sec undefined)"));
    }
    if sin_val.abs() < f64::EPSILON {
        return Err(domain_error("trig_all", x, "sine is zero (cot, csc undefined)"));
    }
    
    let tan_val = sin_val / cos_val;
    let sec_val = 1.0 / cos_val;
    let csc_val = 1.0 / sin_val;
    let cot_val = cos_val / sin_val;
    
    Ok((sin_val, cos_val, tan_val, sec_val, csc_val, cot_val))
}
