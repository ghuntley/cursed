/// Trigonometric functions

use super::{MathError, MathResult, validate_float, domain_error};
use super::constants::{PI, FRAC_PI_2};

/// Sine function
pub fn sin(x: f64) -> MathResult<f64> {
    validate_float("sin", "x", x)?;
    Ok(x.sin())
}

/// Cosine function
pub fn cos(x: f64) -> MathResult<f64> {
    validate_float("cos", "x", x)?;
    Ok(x.cos())
}

/// Tangent function
pub fn tan(x: f64) -> MathResult<f64> {
    validate_float("tan", "x", x)?;
    
    // Check for undefined values at odd multiples of π/2
    let normalized = x / FRAC_PI_2;
    let remainder = normalized % 2.0;
    if (remainder - 1.0).abs() < 1e-10 || (remainder + 1.0).abs() < 1e-10 {
        return Err(domain_error("tan", x, "tangent undefined at odd multiples of π/2"));
    }
    
    Ok(x.tan())
}

/// Arcsine function (inverse sine)
pub fn asin(x: f64) -> MathResult<f64> {
    validate_float("asin", "x", x)?;
    
    if x < -1.0 || x > 1.0 {
        return Err(domain_error("asin", x, "input must be in range [-1, 1]"));
    }
    
    Ok(x.asin())
}

/// Arccosine function (inverse cosine)
pub fn acos(x: f64) -> MathResult<f64> {
    validate_float("acos", "x", x)?;
    
    if x < -1.0 || x > 1.0 {
        return Err(domain_error("acos", x, "input must be in range [-1, 1]"));
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

/// Inverse hyperbolic cosine function
pub fn acosh(x: f64) -> MathResult<f64> {
    validate_float("acosh", "x", x)?;
    
    if x < 1.0 {
        return Err(domain_error("acosh", x, "input must be >= 1"));
    }
    
    Ok(x.acosh())
}

/// Inverse hyperbolic tangent function
pub fn atanh(x: f64) -> MathResult<f64> {
    validate_float("atanh", "x", x)?;
    
    if x <= -1.0 || x >= 1.0 {
        return Err(domain_error("atanh", x, "input must be in range (-1, 1)"));
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
