/// Trigonometric functions module for CURSED programming language
/// 
/// Provides comprehensive trigonometric functionality including basic trigonometric functions,
/// inverse trigonometric functions, hyperbolic functions, angle conversions, and advanced
/// trigonometric utilities.
/// 
/// All functions handle special cases (NaN, infinity) appropriately and provide domain
/// validation for inverse functions. Angles can be specified in radians or degrees
/// depending on the function variant used.

use crate::stdlib::math::{MathError, MathResult, validate_float, domain_error, PI, TAU, FRAC_PI_2};

/// Computes the sine of a number (in radians).
/// 
/// # Formula
/// sin(x) using standard trigonometric definition
/// 
/// # Arguments
/// * `x` - The angle in radians
/// 
/// # Returns
/// * `MathResult<f64>` - The sine of x, or an error for invalid inputs
/// 
/// # Examples
/// ```
/// use crate::stdlib::math::sin;
/// assert_eq!(sin(0.0).unwrap(), 0.0);
/// assert!((sin(PI / 2.0).unwrap() - 1.0).abs() < 1e-10);
/// ```
pub fn sin(x: f64) -> MathResult<f64> {
    validate_float("sin", "x", x)?;
    Ok(x.sin())
}

/// Computes the cosine of a number (in radians).
/// 
/// # Formula
/// cos(x) using standard trigonometric definition
/// 
/// # Arguments
/// * `x` - The angle in radians
/// 
/// # Returns
/// * `MathResult<f64>` - The cosine of x, or an error for invalid inputs
pub fn cos(x: f64) -> MathResult<f64> {
    validate_float("cos", "x", x)?;
    Ok(x.cos())
}

/// Computes the tangent of a number (in radians).
/// 
/// # Formula
/// tan(x) = sin(x) / cos(x)
/// 
/// # Arguments
/// * `x` - The angle in radians
/// 
/// # Returns
/// * `MathResult<f64>` - The tangent of x, or an error for invalid inputs
/// 
/// # Notes
/// Returns infinity for odd multiples of π/2 where tangent is undefined
pub fn tan(x: f64) -> MathResult<f64> {
    validate_float("tan", "x", x)?;
    Ok(x.tan())
}

/// Computes the arcsine (inverse sine) of a number.
/// 
/// # Formula
/// asin(x) = arcsin(x), domain: [-1, 1], range: [-π/2, π/2]
/// 
/// # Arguments
/// * `x` - The value whose arcsine is computed (must be in [-1, 1])
/// 
/// # Returns
/// * `MathResult<f64>` - The arcsine in radians, or domain error if x is outside [-1, 1]
pub fn asin(x: f64) -> MathResult<f64> {
    validate_float("asin", "x", x)?;
    if x < -1.0 || x > 1.0 {
        return Err(domain_error("asin", x, "input must be in range [-1, 1]"));
    }
    Ok(x.asin())
}

/// Computes the arccosine (inverse cosine) of a number.
/// 
/// # Formula
/// acos(x) = arccos(x), domain: [-1, 1], range: [0, π]
/// 
/// # Arguments
/// * `x` - The value whose arccosine is computed (must be in [-1, 1])
/// 
/// # Returns
/// * `MathResult<f64>` - The arccosine in radians, or domain error if x is outside [-1, 1]
pub fn acos(x: f64) -> MathResult<f64> {
    validate_float("acos", "x", x)?;
    if x < -1.0 || x > 1.0 {
        return Err(domain_error("acos", x, "input must be in range [-1, 1]"));
    }
    Ok(x.acos())
}

/// Computes the arctangent (inverse tangent) of a number.
/// 
/// # Formula
/// atan(x) = arctan(x), domain: (-∞, ∞), range: (-π/2, π/2)
/// 
/// # Arguments
/// * `x` - The value whose arctangent is computed
/// 
/// # Returns
/// * `MathResult<f64>` - The arctangent in radians
pub fn atan(x: f64) -> MathResult<f64> {
    validate_float("atan", "x", x)?;
    Ok(x.atan())
}

/// Computes the four-quadrant arctangent of y and x.
/// 
/// # Formula
/// atan2(y, x) = arctan(y/x) with proper quadrant handling
/// 
/// # Arguments
/// * `y` - The y coordinate
/// * `x` - The x coordinate
/// 
/// # Returns
/// * `MathResult<f64>` - The angle in radians from the positive x-axis to the point (x, y)
/// 
/// # Notes
/// This function correctly handles all quadrants and the signs of both arguments
/// to determine the appropriate quadrant of the result.
pub fn atan2(y: f64, x: f64) -> MathResult<f64> {
    validate_float("atan2", "y", y)?;
    validate_float("atan2", "x", x)?;
    Ok(y.atan2(x))
}

/// Computes the hyperbolic sine of a number.
/// 
/// # Formula
/// sinh(x) = (e^x - e^(-x)) / 2
/// 
/// # Arguments
/// * `x` - The input value
/// 
/// # Returns
/// * `MathResult<f64>` - The hyperbolic sine of x
pub fn sinh(x: f64) -> MathResult<f64> {
    validate_float("sinh", "x", x)?;
    Ok(x.sinh())
}

/// Computes the hyperbolic cosine of a number.
/// 
/// # Formula
/// cosh(x) = (e^x + e^(-x)) / 2
/// 
/// # Arguments
/// * `x` - The input value
/// 
/// # Returns
/// * `MathResult<f64>` - The hyperbolic cosine of x (always ≥ 1)
pub fn cosh(x: f64) -> MathResult<f64> {
    validate_float("cosh", "x", x)?;
    Ok(x.cosh())
}

/// Computes the hyperbolic tangent of a number.
/// 
/// # Formula
/// tanh(x) = sinh(x) / cosh(x) = (e^x - e^(-x)) / (e^x + e^(-x))
/// 
/// # Arguments
/// * `x` - The input value
/// 
/// # Returns
/// * `MathResult<f64>` - The hyperbolic tangent of x (always in range [-1, 1])
pub fn tanh(x: f64) -> MathResult<f64> {
    validate_float("tanh", "x", x)?;
    Ok(x.tanh())
}

/// Computes the inverse hyperbolic sine of a number.
/// 
/// # Formula
/// asinh(x) = ln(x + √(x² + 1))
/// 
/// # Arguments
/// * `x` - The input value (any real number)
/// 
/// # Returns
/// * `MathResult<f64>` - The inverse hyperbolic sine of x
pub fn asinh(x: f64) -> MathResult<f64> {
    validate_float("asinh", "x", x)?;
    Ok(x.asinh())
}

/// Computes the inverse hyperbolic cosine of a number.
/// 
/// # Formula
/// acosh(x) = ln(x + √(x² - 1))
/// 
/// # Arguments
/// * `x` - The input value (must be ≥ 1)
/// 
/// # Returns
/// * `MathResult<f64>` - The inverse hyperbolic cosine of x, or domain error if x < 1
pub fn acosh(x: f64) -> MathResult<f64> {
    validate_float("acosh", "x", x)?;
    if x < 1.0 {
        return Err(domain_error("acosh", x, "input must be >= 1"));
    }
    Ok(x.acosh())
}

/// Computes the inverse hyperbolic tangent of a number.
/// 
/// # Formula
/// atanh(x) = (1/2) * ln((1 + x) / (1 - x))
/// 
/// # Arguments
/// * `x` - The input value (must be in (-1, 1))
/// 
/// # Returns
/// * `MathResult<f64>` - The inverse hyperbolic tangent of x, or domain error if |x| ≥ 1
pub fn atanh(x: f64) -> MathResult<f64> {
    validate_float("atanh", "x", x)?;
    if x <= -1.0 || x >= 1.0 {
        return Err(domain_error("atanh", x, "input must be in range (-1, 1)"));
    }
    Ok(x.atanh())
}

/// Converts degrees to radians.
/// 
/// # Formula
/// radians = degrees * π / 180
/// 
/// # Arguments
/// * `degrees` - The angle in degrees
/// 
/// # Returns
/// * `MathResult<f64>` - The angle in radians
pub fn degrees_to_radians(degrees: f64) -> MathResult<f64> {
    validate_float("degrees_to_radians", "degrees", degrees)?;
    Ok(degrees.to_radians())
}

/// Converts radians to degrees.
/// 
/// # Formula
/// degrees = radians * 180 / π
/// 
/// # Arguments
/// * `radians` - The angle in radians
/// 
/// # Returns
/// * `MathResult<f64>` - The angle in degrees
pub fn radians_to_degrees(radians: f64) -> MathResult<f64> {
    validate_float("radians_to_degrees", "radians", radians)?;
    Ok(radians.to_degrees())
}

/// Alias for degrees_to_radians for convenience.
pub fn deg_to_rad(degrees: f64) -> MathResult<f64> {
    degrees_to_radians(degrees)
}

/// Alias for radians_to_degrees for convenience.
pub fn rad_to_deg(radians: f64) -> MathResult<f64> {
    radians_to_degrees(radians)
}

/// Computes the sine of an angle in degrees.
/// 
/// # Arguments
/// * `degrees` - The angle in degrees
/// 
/// # Returns
/// * `MathResult<f64>` - The sine of the angle
pub fn sin_deg(degrees: f64) -> MathResult<f64> {
    let radians = degrees_to_radians(degrees)?;
    sin(radians)
}

/// Computes the cosine of an angle in degrees.
/// 
/// # Arguments
/// * `degrees` - The angle in degrees
/// 
/// # Returns
/// * `MathResult<f64>` - The cosine of the angle
pub fn cos_deg(degrees: f64) -> MathResult<f64> {
    let radians = degrees_to_radians(degrees)?;
    cos(radians)
}

/// Computes the tangent of an angle in degrees.
/// 
/// # Arguments
/// * `degrees` - The angle in degrees
/// 
/// # Returns
/// * `MathResult<f64>` - The tangent of the angle
pub fn tan_deg(degrees: f64) -> MathResult<f64> {
    let radians = degrees_to_radians(degrees)?;
    tan(radians)
}

/// Computes the secant (reciprocal of cosine).
/// 
/// # Formula
/// sec(x) = 1 / cos(x)
/// 
/// # Arguments
/// * `x` - The angle in radians
/// 
/// # Returns
/// * `MathResult<f64>` - The secant of x, or domain error if cos(x) = 0
pub fn sec(x: f64) -> MathResult<f64> {
    validate_float("sec", "x", x)?;
    let cos_x = x.cos();
    if cos_x.abs() < f64::EPSILON {
        return Err(MathError::DivisionByZero { function: "sec".to_string() });
    }
    Ok(1.0 / cos_x)
}

/// Computes the cosecant (reciprocal of sine).
/// 
/// # Formula
/// csc(x) = 1 / sin(x)
/// 
/// # Arguments
/// * `x` - The angle in radians
/// 
/// # Returns
/// * `MathResult<f64>` - The cosecant of x, or domain error if sin(x) = 0
pub fn csc(x: f64) -> MathResult<f64> {
    validate_float("csc", "x", x)?;
    let sin_x = x.sin();
    if sin_x.abs() < f64::EPSILON {
        return Err(MathError::DivisionByZero { function: "csc".to_string() });
    }
    Ok(1.0 / sin_x)
}

/// Computes the cotangent (reciprocal of tangent).
/// 
/// # Formula
/// cot(x) = 1 / tan(x) = cos(x) / sin(x)
/// 
/// # Arguments
/// * `x` - The angle in radians
/// 
/// # Returns
/// * `MathResult<f64>` - The cotangent of x, or domain error if sin(x) = 0
pub fn cot(x: f64) -> MathResult<f64> {
    validate_float("cot", "x", x)?;
    let sin_x = x.sin();
    if sin_x.abs() < f64::EPSILON {
        return Err(MathError::DivisionByZero { function: "cot".to_string() });
    }
    Ok(x.cos() / sin_x)
}

/// Normalizes an angle to the range [0, 2π).
/// 
/// # Arguments
/// * `angle` - The angle in radians
/// 
/// # Returns
/// * `MathResult<f64>` - The normalized angle in range [0, 2π)
pub fn normalize_angle(angle: f64) -> MathResult<f64> {
    validate_float("normalize_angle", "angle", angle)?;
    let normalized = angle % TAU;
    Ok(if normalized < 0.0 { normalized + TAU } else { normalized })
}

/// Normalizes an angle to the range (-π, π].
/// 
/// # Arguments
/// * `angle` - The angle in radians
/// 
/// # Returns
/// * `MathResult<f64>` - The normalized angle in range (-π, π]
pub fn normalize_angle_signed(angle: f64) -> MathResult<f64> {
    validate_float("normalize_angle_signed", "angle", angle)?;
    let mut normalized = angle % TAU;
    if normalized > PI {
        normalized -= TAU;
    } else if normalized <= -PI {
        normalized += TAU;
    }
    Ok(normalized)
}

/// Computes the sinc function: sin(x) / x.
/// 
/// # Formula
/// sinc(x) = sin(x) / x for x ≠ 0, sinc(0) = 1
/// 
/// # Arguments
/// * `x` - The input value
/// 
/// # Returns
/// * `MathResult<f64>` - The sinc function value
/// 
/// # Notes
/// This function is particularly important in signal processing and the theory of Fourier transforms.
pub fn sinc(x: f64) -> MathResult<f64> {
    validate_float("sinc", "x", x)?;
    if x.abs() < f64::EPSILON {
        Ok(1.0)
    } else {
        Ok(x.sin() / x)
    }
}

/// Computes the haversine of an angle.
/// 
/// # Formula
/// haversine(θ) = sin²(θ/2) = (1 - cos(θ)) / 2
/// 
/// # Arguments
/// * `theta` - The angle in radians
/// 
/// # Returns
/// * `MathResult<f64>` - The haversine of the angle
/// 
/// # Notes
/// The haversine function is used in navigation, particularly in calculating
/// great-circle distances on a sphere (like Earth).
pub fn haversine(theta: f64) -> MathResult<f64> {
    validate_float("haversine", "theta", theta)?;
    let half_theta = theta / 2.0;
    let sin_half = half_theta.sin();
    Ok(sin_half * sin_half)
}

/// Computes the third side of a triangle using the law of cosines.
/// 
/// # Formula
/// c² = a² + b² - 2ab*cos(C)
/// c = √(a² + b² - 2ab*cos(C))
/// 
/// # Arguments
/// * `a` - Length of first side (must be positive)
/// * `b` - Length of second side (must be positive)
/// * `angle_c` - The angle opposite to side c, in radians
/// 
/// # Returns
/// * `MathResult<f64>` - The length of the third side, or error for invalid inputs
/// 
/// # Errors
/// Returns error if sides are non-positive or if the triangle inequality would be violated.
pub fn law_of_cosines(a: f64, b: f64, angle_c: f64) -> MathResult<f64> {
    validate_float("law_of_cosines", "a", a)?;
    validate_float("law_of_cosines", "b", b)?;
    validate_float("law_of_cosines", "angle_c", angle_c)?;
    
    if a <= 0.0 {
        return Err(domain_error("law_of_cosines", a, "side a must be positive"));
    }
    if b <= 0.0 {
        return Err(domain_error("law_of_cosines", b, "side b must be positive"));
    }
    
    let cos_c = angle_c.cos();
    let c_squared = a * a + b * b - 2.0 * a * b * cos_c;
    
    if c_squared < 0.0 {
        return Err(domain_error("law_of_cosines", angle_c, "invalid angle - would result in negative side length"));
    }
    
    Ok(c_squared.sqrt())
}

/// Computes the haversine distance between two points on a sphere.
/// 
/// # Formula
/// d = 2r * arcsin(√(haversine(Δφ) + cos(φ1) * cos(φ2) * haversine(Δλ)))
/// 
/// # Arguments
/// * `lat1` - Latitude of first point in radians
/// * `lon1` - Longitude of first point in radians
/// * `lat2` - Latitude of second point in radians
/// * `lon2` - Longitude of second point in radians
/// * `radius` - Radius of the sphere (e.g., Earth's radius)
/// 
/// # Returns
/// * `MathResult<f64>` - The great-circle distance between the two points
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64, radius: f64) -> MathResult<f64> {
    validate_float("haversine_distance", "lat1", lat1)?;
    validate_float("haversine_distance", "lon1", lon1)?;
    validate_float("haversine_distance", "lat2", lat2)?;
    validate_float("haversine_distance", "lon2", lon2)?;
    validate_float("haversine_distance", "radius", radius)?;
    
    if radius <= 0.0 {
        return Err(domain_error("haversine_distance", radius, "radius must be positive"));
    }
    
    let delta_lat = lat2 - lat1;
    let delta_lon = lon2 - lon1;
    
    let a = haversine(delta_lat)? + lat1.cos() * lat2.cos() * haversine(delta_lon)?;
    let c = 2.0 * a.sqrt().asin();
    
    Ok(radius * c)
}

/// Computes all six trigonometric functions for a given angle.
/// 
/// # Arguments
/// * `angle` - The angle in radians
/// 
/// # Returns
/// * `MathResult<(f64, f64, f64, f64, f64, f64)>` - Tuple of (sin, cos, tan, sec, csc, cot)
/// 
/// # Notes
/// This function efficiently computes all six trig functions with only two transcendental
/// function calls (sin and cos), which is more efficient than calling each function separately.
pub fn all_trig_functions(angle: f64) -> MathResult<(f64, f64, f64, f64, f64, f64)> {
    validate_float("all_trig_functions", "angle", angle)?;
    
    let sin_val = angle.sin();
    let cos_val = angle.cos();
    
    // Check for division by zero cases
    let tan_val = if cos_val.abs() < f64::EPSILON {
        f64::INFINITY.copysign(sin_val)
    } else {
        sin_val / cos_val
    };
    
    let sec_val = if cos_val.abs() < f64::EPSILON {
        f64::INFINITY.copysign(cos_val)
    } else {
        1.0 / cos_val
    };
    
    let csc_val = if sin_val.abs() < f64::EPSILON {
        f64::INFINITY.copysign(sin_val)
    } else {
        1.0 / sin_val
    };
    
    let cot_val = if sin_val.abs() < f64::EPSILON {
        f64::INFINITY.copysign(cos_val)
    } else {
        cos_val / sin_val
    };
    
    Ok((sin_val, cos_val, tan_val, sec_val, csc_val, cot_val))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4, FRAC_PI_6};

    #[test]
    fn test_basic_trig_functions() {
        assert!((sin(0.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((cos(0.0).unwrap() - 1.0).abs() < 1e-10);
        assert!((tan(0.0).unwrap() - 0.0).abs() < 1e-10);
        
        assert!((sin(FRAC_PI_2).unwrap() - 1.0).abs() < 1e-10);
        assert!((cos(FRAC_PI_2).unwrap() - 0.0).abs() < 1e-10);
        
        assert!((sin(PI).unwrap() - 0.0).abs() < 1e-10);
        assert!((cos(PI).unwrap() - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_trig_functions() {
        assert!((asin(0.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((acos(1.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((atan(0.0).unwrap() - 0.0).abs() < 1e-10);
        
        // Test domain errors
        assert!(asin(2.0).is_err());
        assert!(asin(-2.0).is_err());
        assert!(acos(2.0).is_err());
        assert!(acos(-2.0).is_err());
    }

    #[test]
    fn test_hyperbolic_functions() {
        assert!((sinh(0.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((cosh(0.0).unwrap() - 1.0).abs() < 1e-10);
        assert!((tanh(0.0).unwrap() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_hyperbolic_functions() {
        assert!((asinh(0.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((acosh(1.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((atanh(0.0).unwrap() - 0.0).abs() < 1e-10);
        
        // Test domain errors
        assert!(acosh(0.5).is_err());
        assert!(atanh(1.0).is_err());
        assert!(atanh(-1.0).is_err());
    }

    #[test]
    fn test_angle_conversion() {
        assert!((degrees_to_radians(180.0).unwrap() - PI).abs() < 1e-10);
        assert!((radians_to_degrees(PI).unwrap() - 180.0).abs() < 1e-10);
        assert!((deg_to_rad(90.0).unwrap() - FRAC_PI_2).abs() < 1e-10);
        assert!((rad_to_deg(FRAC_PI_2).unwrap() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn test_degree_based_functions() {
        assert!((sin_deg(30.0).unwrap() - 0.5).abs() < 1e-10);
        assert!((cos_deg(60.0).unwrap() - 0.5).abs() < 1e-10);
        assert!((tan_deg(45.0).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_reciprocal_functions() {
        assert!((sec(0.0).unwrap() - 1.0).abs() < 1e-10);
        assert!((csc(FRAC_PI_2).unwrap() - 1.0).abs() < 1e-10);
        assert!((cot(FRAC_PI_4).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_angle_normalization() {
        assert!((normalize_angle(3.0 * PI).unwrap() - PI).abs() < 1e-10);
        assert!((normalize_angle_signed(3.0 * PI).unwrap() - (-PI)).abs() < 1e-10);
    }

    #[test]
    fn test_sinc_function() {
        assert!((sinc(0.0).unwrap() - 1.0).abs() < 1e-10);
        assert!((sinc(PI).unwrap() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_haversine() {
        assert!((haversine(0.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((haversine(PI).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_law_of_cosines() {
        // Right triangle: 3-4-5
        let c = law_of_cosines(3.0, 4.0, FRAC_PI_2).unwrap();
        assert!((c - 5.0).abs() < 1e-10);
        
        // Test domain errors
        assert!(law_of_cosines(-1.0, 4.0, FRAC_PI_2).is_err());
        assert!(law_of_cosines(3.0, -1.0, FRAC_PI_2).is_err());
    }

    #[test]
    fn test_all_trig_functions() {
        let (sin_val, cos_val, tan_val, sec_val, csc_val, cot_val) = all_trig_functions(FRAC_PI_4).unwrap();
        
        let sqrt_2_inv = 1.0 / 2.0_f64.sqrt();
        assert!((sin_val - sqrt_2_inv).abs() < 1e-10);
        assert!((cos_val - sqrt_2_inv).abs() < 1e-10);
        assert!((tan_val - 1.0).abs() < 1e-10);
        assert!((sec_val - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!((csc_val - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!((cot_val - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_nan_input_handling() {
        assert!(sin(f64::NAN).is_err());
        assert!(cos(f64::NAN).is_err());
        assert!(tan(f64::NAN).is_err());
    }

    #[test]
    fn test_infinity_input_handling() {
        assert!(sin(f64::INFINITY).is_err());
        assert!(cos(f64::INFINITY).is_err());
        assert!(tan(f64::INFINITY).is_err());
    }
}
