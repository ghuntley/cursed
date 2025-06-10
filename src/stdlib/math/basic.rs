/// Basic mathematical operations and utility functions

use super::{MathError, MathResult, validate_float, division_by_zero_error, negative_input_error};

/// Returns the absolute value of a number
pub fn abs(x: f64) -> f64 {
    x.abs()
}

/// Returns the minimum of two numbers
pub fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}

/// Returns the maximum of two numbers
pub fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}

/// Clamps a value between a minimum and maximum
pub fn clamp(value: f64, min_val: f64, max_val: f64) -> MathResult<f64> {
    validate_float("clamp", "value", value)?;
    validate_float("clamp", "min", min_val)?;
    validate_float("clamp", "max", max_val)?;
    
    if min_val > max_val {
        return Err(MathError::InvalidInput {
            function: "clamp".to_string(),
            parameter: "min > max".to_string(),
            value: min_val,
        });
    }
    
    Ok(value.clamp(min_val, max_val))
}

/// Returns the sign of a number (-1, 0, or 1)
pub fn sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

/// Returns the largest integer less than or equal to the input
pub fn floor(x: f64) -> MathResult<f64> {
    validate_float("floor", "x", x)?;
    Ok(x.floor())
}

/// Returns the smallest integer greater than or equal to the input
pub fn ceil(x: f64) -> MathResult<f64> {
    validate_float("ceil", "x", x)?;
    Ok(x.ceil())
}

/// Rounds to the nearest integer
pub fn round(x: f64) -> MathResult<f64> {
    validate_float("round", "x", x)?;
    Ok(x.round())
}

/// Truncates the fractional part (rounds toward zero)
pub fn math_truncate(x: f64) -> MathResult<f64> {
    validate_float("math_truncate", "x", x)?;
    Ok(x.trunc())
}

/// Returns the fractional part of a number
pub fn fract(x: f64) -> MathResult<f64> {
    validate_float("fract", "x", x)?;
    Ok(x.fract())
}

/// Floating-point remainder of division
pub fn remainder(dividend: f64, divisor: f64) -> MathResult<f64> {
    validate_float("remainder", "dividend", dividend)?;
    validate_float("remainder", "divisor", divisor)?;
    
    if divisor == 0.0 {
        return Err(division_by_zero_error("remainder"));
    }
    
    Ok(dividend % divisor)
}

/// Euclidean modulo (always positive result)
pub fn modulo(dividend: f64, divisor: f64) -> MathResult<f64> {
    validate_float("modulo", "dividend", dividend)?;
    validate_float("modulo", "divisor", divisor)?;
    
    if divisor == 0.0 {
        return Err(division_by_zero_error("modulo"));
    }
    
    let result = dividend % divisor;
    if result < 0.0 {
        Ok(result + divisor.abs())
    } else {
        Ok(result)
    }
}

/// Greatest common divisor using Euclidean algorithm
pub fn gcd(a: i64, b: i64) -> MathResult<i64> {
    let mut x = a.abs();
    let mut y = b.abs();
    
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    
    Ok(x)
}

/// Least common multiple
pub fn lcm(a: i64, b: i64) -> MathResult<i64> {
    if a == 0 || b == 0 {
        return Ok(0);
    }
    
    let gcd_val = gcd(a, b)?;
    let result = (a.abs() / gcd_val).checked_mul(b.abs());
    
    match result {
        Some(val) => Ok(val),
        None => Err(MathError::IntegerOverflow {
            function: "lcm".to_string(),
            value: a.max(b),
        }),
    }
}

/// Returns true if the number is even
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}

/// Returns true if the number is odd
pub fn is_odd(n: i64) -> bool {
    n % 2 != 0
}

/// Linear interpolation between two values
pub fn lerp(start: f64, end: f64, t: f64) -> MathResult<f64> {
    validate_float("lerp", "start", start)?;
    validate_float("lerp", "end", end)?;
    validate_float("lerp", "t", t)?;
    
    Ok(start + t * (end - start))
}

/// Inverse linear interpolation - returns t for given value between start and end
pub fn inverse_lerp(start: f64, end: f64, value: f64) -> MathResult<f64> {
    validate_float("inverse_lerp", "start", start)?;
    validate_float("inverse_lerp", "end", end)?;
    validate_float("inverse_lerp", "value", value)?;
    
    if (end - start).abs() < f64::EPSILON {
        return Err(division_by_zero_error("inverse_lerp"));
    }
    
    Ok((value - start) / (end - start))
}

/// Smooth step interpolation (Hermite interpolation)
pub fn smooth_step(start: f64, end: f64, t: f64) -> MathResult<f64> {
    validate_float("smooth_step", "start", start)?;
    validate_float("smooth_step", "end", end)?;
    validate_float("smooth_step", "t", t)?;
    
    let clamped_t = t.clamp(0.0, 1.0);
    let smooth_t = clamped_t * clamped_t * (3.0 - 2.0 * clamped_t);
    
    Ok(start + smooth_t * (end - start))
}

/// Smoother step interpolation (Ken Perlin's smootherstep)
pub fn smoother_step(start: f64, end: f64, t: f64) -> MathResult<f64> {
    validate_float("smoother_step", "start", start)?;
    validate_float("smoother_step", "end", end)?;
    validate_float("smoother_step", "t", t)?;
    
    let clamped_t = t.clamp(0.0, 1.0);
    let smooth_t = clamped_t * clamped_t * clamped_t * (clamped_t * (clamped_t * 6.0 - 15.0) + 10.0);
    
    Ok(start + smooth_t * (end - start))
}

/// Integer absolute value
pub fn abs_i32(x: i32) -> i32 {
    x.abs()
}

/// Integer absolute value for i64
pub fn abs_i64(x: i64) -> i64 {
    x.abs()
}

/// Integer minimum
pub fn min_i32(a: i32, b: i32) -> i32 {
    a.min(b)
}

/// Integer maximum
pub fn max_i32(a: i32, b: i32) -> i32 {
    a.max(b)
}

/// Integer clamp
pub fn clamp_i32(value: i32, min_val: i32, max_val: i32) -> MathResult<i32> {
    if min_val > max_val {
        return Err(MathError::InvalidInput {
            function: "clamp_i32".to_string(),
            parameter: "min > max".to_string(),
            value: min_val as f64,
        });
    }
    
    Ok(value.clamp(min_val, max_val))
}
