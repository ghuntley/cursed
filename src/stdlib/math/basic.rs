use crate::error::CursedError;
/// Basic mathematical operations and utility functions

use super::{MathError, MathResult, validate_float, division_by_zero_error, negative_input_error};

/// Returns the absolute value of a number
pub fn abs(x: f64) -> f64 {
    x.abs()
/// Returns the minimum of two numbers
pub fn min(a: f64, b: f64) -> f64 {
    a.min(b)
/// Returns the maximum of two numbers
pub fn max(a: f64, b: f64) -> f64 {
    a.max(b)
/// Clamps a value between a minimum and maximum
pub fn clamp(value: f64, min_val: f64, max_val: f64) -> MathResult<f64> {
    validate_float("clamp", "value", value)?;
    validate_float("clamp", "min", min_val)?;
    validate_float("clamp", "max", max_val)?;
    
    if min_val > max_val {
        return Err(MathError::InvalidInput {
        });
    Ok(value.clamp(min_val, max_val))
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
/// Returns the smallest integer greater than or equal to the input
pub fn ceil(x: f64) -> MathResult<f64> {
    validate_float("ceil", "x", x)?;
    Ok(x.ceil())
/// Rounds to the nearest integer
pub fn round(x: f64) -> MathResult<f64> {
    validate_float("round", "x", x)?;
    Ok(x.round())
/// Truncates the fractional part (rounds toward zero)
pub fn math_truncate(x: f64) -> MathResult<f64> {
    validate_float("math_truncate", "x", x)?;
    Ok(x.trunc())
/// Returns the fractional part of a number
pub fn fract(x: f64) -> MathResult<f64> {
    validate_float("fract", "x", x)?;
    Ok(x.fract())
/// Floating-point remainder of division
pub fn remainder(dividend: f64, divisor: f64) -> MathResult<f64> {
    validate_float("remainder", "dividend", dividend)?;
    validate_float("remainder", "divisor", divisor)?;
    
    if divisor == 0.0 {
        return Err(division_by_zero_error("remainder"));
    Ok(dividend % divisor)
/// Euclidean modulo (always positive result)
pub fn modulo(dividend: f64, divisor: f64) -> MathResult<f64> {
    validate_float("modulo", "dividend", dividend)?;
    validate_float("modulo", "divisor", divisor)?;
    
    if divisor == 0.0 {
        return Err(division_by_zero_error("modulo"));
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
    Ok(x)
/// Least common multiple
pub fn lcm(a: i64, b: i64) -> MathResult<i64> {
    if a == 0 || b == 0 {
        return Ok(0);
    let gcd_val = gcd(a, b)?;
    let result = (a.abs() / gcd_val).checked_mul(b.abs());
    
    match result {
        None => Err(MathError::IntegerOverflow {
    }
}

/// Returns true if the number is even
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
/// Returns true if the number is odd
pub fn is_odd(n: i64) -> bool {
    n % 2 != 0
/// Linear interpolation between two values
pub fn lerp(start: f64, end: f64, t: f64) -> MathResult<f64> {
    validate_float("lerp", "start", start)?;
    validate_float("lerp", "end", end)?;
    validate_float("lerp", "t", t)?;
    
    Ok(start + t * (end - start))
/// Inverse linear interpolation - returns t for given value between start and end
pub fn inverse_lerp(start: f64, end: f64, value: f64) -> MathResult<f64> {
    validate_float("inverse_lerp", "start", start)?;
    validate_float("inverse_lerp", "end", end)?;
    validate_float("inverse_lerp", "value", value)?;
    
    if (end - start).abs() < f64::EPSILON {
        return Err(division_by_zero_error("inverse_lerp"));
    Ok((value - start) / (end - start))
/// Smooth step interpolation (Hermite interpolation)
pub fn smooth_step(start: f64, end: f64, t: f64) -> MathResult<f64> {
    validate_float("smooth_step", "start", start)?;
    validate_float("smooth_step", "end", end)?;
    validate_float("smooth_step", "t", t)?;
    
    let clamped_t = t.clamp(0.0, 1.0);
    let smooth_t = clamped_t * clamped_t * (3.0 - 2.0 * clamped_t);
    
    Ok(start + smooth_t * (end - start))
/// Smoother step interpolation (Ken Perlin's smootherstep)
pub fn smoother_step(start: f64, end: f64, t: f64) -> MathResult<f64> {
    validate_float("smoother_step", "start", start)?;
    validate_float("smoother_step", "end", end)?;
    validate_float("smoother_step", "t", t)?;
    
    let clamped_t = t.clamp(0.0, 1.0);
    let smooth_t = clamped_t * clamped_t * clamped_t * (clamped_t * (clamped_t * 6.0 - 15.0) + 10.0);
    
    Ok(start + smooth_t * (end - start))
/// Integer absolute value
pub fn abs_i32(x: i32) -> i32 {
    x.abs()
/// Integer absolute value for i64
pub fn abs_i64(x: i64) -> i64 {
    x.abs()
/// Integer minimum
pub fn min_i32(a: i32, b: i32) -> i32 {
    a.min(b)
/// Integer maximum
pub fn max_i32(a: i32, b: i32) -> i32 {
    a.max(b)
/// Integer clamp
pub fn clamp_i32(value: i32, min_val: i32, max_val: i32) -> MathResult<i32> {
    if min_val > max_val {
        return Err(MathError::InvalidInput {
        });
    Ok(value.clamp(min_val, max_val))
/// Integer minimum for i64
pub fn min_i64(a: i64, b: i64) -> i64 {
    a.min(b)
/// Integer maximum for i64
pub fn max_i64(a: i64, b: i64) -> i64 {
    a.max(b)
/// Integer clamp for i64
pub fn clamp_i64(value: i64, min_val: i64, max_val: i64) -> MathResult<i64> {
    if min_val > max_val {
        return Err(MathError::InvalidInput {
        });
    Ok(value.clamp(min_val, max_val))
/// Computes the nth power of 2 (2^n)
pub fn pow2(n: u32) -> MathResult<f64> {
    if n > 1023 {
        return Err(MathError::Overflow {
        });
    Ok(2.0_f64.powi(n as i32))
/// Computes the nth power of 10 (10^n)
pub fn pow10(n: i32) -> MathResult<f64> {
    if n > 308 {
        return Err(MathError::Overflow {
        });
    }
    if n < -324 {
        return Err(MathError::Underflow {
        });
    Ok(10.0_f64.powi(n))
/// Computes x raised to the power of y (x^y)
pub fn pow(x: f64, y: f64) -> MathResult<f64> {
    validate_float("pow", "x", x)?;
    validate_float("pow", "y", y)?;
    
    // Handle special cases
    if x == 0.0 && y < 0.0 {
        return Err(MathError::DivisionByZero {
        });
    if x < 0.0 && y.fract() != 0.0 {
        return Err(MathError::DomainError {
        });
    let result = x.powf(y);
    
    if !result.is_finite() && x.is_finite() && y.is_finite() {
        if result.is_infinite() {
            return Err(MathError::Overflow {
            });
        } else {
            return Err(MathError::ComputationError {
            });
        }
    }
    
    Ok(result)
/// Computes the square root of a number
pub fn sqrt(x: f64) -> MathResult<f64> {
    validate_float("sqrt", "x", x)?;
    
    if x < 0.0 {
        return Err(negative_input_error("sqrt", x));
    Ok(x.sqrt())
/// Computes the cube root of a number
pub fn cbrt(x: f64) -> MathResult<f64> {
    validate_float("cbrt", "x", x)?;
    Ok(x.cbrt())
/// Computes the nth root of a number
pub fn nth_root(x: f64, n: f64) -> MathResult<f64> {
    validate_float("nth_root", "x", x)?;
    validate_float("nth_root", "n", n)?;
    
    if n == 0.0 {
        return Err(division_by_zero_error("nth_root"));
    if x < 0.0 && n.fract() != 0.0 {
        return Err(MathError::DomainError {
        });
    let result = x.powf(1.0 / n);
    
    if !result.is_finite() && x.is_finite() && n.is_finite() {
        return Err(MathError::ComputationError {
        });
    Ok(result)
/// Computes the hypotenuse (sqrt(x^2 + y^2))
pub fn hypot(x: f64, y: f64) -> MathResult<f64> {
    validate_float("hypot", "x", x)?;
    validate_float("hypot", "y", y)?;
    
    Ok(x.hypot(y))
/// Computes the multiplicative inverse (1/x)
pub fn reciprocal(x: f64) -> MathResult<f64> {
    validate_float("reciprocal", "x", x)?;
    
    if x == 0.0 {
        return Err(division_by_zero_error("reciprocal"));
    Ok(1.0 / x)
/// Computes the square of a number (x^2)
pub fn square(x: f64) -> MathResult<f64> {
    validate_float("square", "x", x)?;
    
    let result = x * x;
    
    if !result.is_finite() && x.is_finite() {
        return Err(MathError::Overflow {
        });
    Ok(result)
/// Computes the cube of a number (x^3)
pub fn cube(x: f64) -> MathResult<f64> {
    validate_float("cube", "x", x)?;
    
    let result = x * x * x;
    
    if !result.is_finite() && x.is_finite() {
        return Err(MathError::Overflow {
        });
    Ok(result)
/// Checks if a number is close to zero within epsilon
pub fn is_zero(x: f64, epsilon: f64) -> MathResult<bool> {
    validate_float("is_zero", "x", x)?;
    validate_float("is_zero", "epsilon", epsilon)?;
    
    if epsilon < 0.0 {
        return Err(negative_input_error("is_zero", epsilon));
    Ok(x.abs() <= epsilon)
/// Checks if two numbers are approximately equal within epsilon
pub fn is_equal(a: f64, b: f64, epsilon: f64) -> MathResult<bool> {
    validate_float("is_equal", "a", a)?;
    validate_float("is_equal", "b", b)?;
    validate_float("is_equal", "epsilon", epsilon)?;
    
    if epsilon < 0.0 {
        return Err(negative_input_error("is_equal", epsilon));
    Ok((a - b).abs() <= epsilon)
/// Rounds to a specified number of decimal places
pub fn round_to_decimals(x: f64, decimals: u32) -> MathResult<f64> {
    validate_float("round_to_decimals", "x", x)?;
    
    if decimals > 15 {
        return Err(MathError::InvalidInput {
        });
    let multiplier = pow10(decimals as i32)?;
    Ok((x * multiplier).round() / multiplier)
/// Maps a value from one range to another
pub fn map_range(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> MathResult<f64> {
    validate_float("map_range", "value", value)?;
    validate_float("map_range", "from_min", from_min)?;
    validate_float("map_range", "from_max", from_max)?;
    validate_float("map_range", "to_min", to_min)?;
    validate_float("map_range", "to_max", to_max)?;
    
    if (from_max - from_min).abs() < f64::EPSILON {
        return Err(division_by_zero_error("map_range"));
    let normalized = (value - from_min) / (from_max - from_min);
    Ok(to_min + normalized * (to_max - to_min))
/// Calculates the average of two numbers
pub fn average(a: f64, b: f64) -> MathResult<f64> {
    validate_float("average", "a", a)?;
    validate_float("average", "b", b)?;
    
    Ok((a + b) / 2.0)
/// Calculates the geometric mean of two positive numbers
pub fn geometric_mean(a: f64, b: f64) -> MathResult<f64> {
    validate_float("geometric_mean", "a", a)?;
    validate_float("geometric_mean", "b", b)?;
    
    if a < 0.0 {
        return Err(negative_input_error("geometric_mean", a));
    }
    if b < 0.0 {
        return Err(negative_input_error("geometric_mean", b));
    Ok((a * b).sqrt())
/// Calculates the harmonic mean of two positive numbers
pub fn harmonic_mean(a: f64, b: f64) -> MathResult<f64> {
    validate_float("harmonic_mean", "a", a)?;
    validate_float("harmonic_mean", "b", b)?;
    
    if a <= 0.0 {
        return Err(negative_input_error("harmonic_mean", a));
    }
    if b <= 0.0 {
        return Err(negative_input_error("harmonic_mean", b));
    Ok(2.0 / (1.0 / a + 1.0 / b))
}
