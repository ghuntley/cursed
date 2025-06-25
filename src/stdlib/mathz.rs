/// MathZ - Mathematical functions with Gen Z flair 🧮
/// 
/// This module provides comprehensive mathematical operations using CURSED language
/// conventions and Gen Z naming. All functions work with CURSED numeric types and
/// provide the mathematical foundation for computational applications.
/// 
/// # Why MathZ matters:
/// - Essential for any computational application  
/// - Provides type-safe mathematical operations with CURSED types
/// - Includes advanced functions for scientific computing
/// - Optimized for performance while maintaining precision

// use crate::stdlib::math::{self, MathError, MathResult};
use crate::error::CursedError;
use std::collections::HashMap;

/// CursedError type for MathZ operations
pub type MathzError = MathError;

/// Result type for MathZ operations
pub type MathzResult<T> = MathResult<T>;

// CURSED type aliases for Gen Z mathematics
/// `normie` - Standard 32-bit integer (the basic number)
pub type Normie = i32;

/// `thicc` - 64-bit integer (the big number)  
pub type Thicc = i64;

/// `smol` - 32-bit float (the small decimal)
pub type Smol = f32;

/// `chonky` - 64-bit float (the big decimal)
pub type Chonky = f64;

// ================================
// BASIC ARITHMETIC (NORMIE VIBES)
// ================================

/// Absolute value for normie numbers (make it positive vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = abs_normie(-42) // 42
/// ```
pub fn abs_normie(x: Normie) -> Normie {
    math::abs_i32(x)
/// Minimum of two normie numbers (smallest vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = min_normie(10, 20) // 10
/// ```
pub fn min_normie(a: Normie, b: Normie) -> Normie {
    math::min_i32(a, b)
/// Maximum of two normie numbers (biggest vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = max_normie(10, 20) // 20
/// ```
pub fn max_normie(a: Normie, b: Normie) -> Normie {
    math::max_i32(a, b)
/// Clamp normie between bounds (keep it in range vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = clamp_normie(25, 10, 20) // 20
/// ```
pub fn clamp_normie(value: Normie, min: Normie, max: Normie) -> Normie {
    math::clamp_i32(value, min, max)
/// Sign of normie number (direction vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = sign_normie(-42) // -1
/// facts result = sign_normie(42) // 1
/// facts result = sign_normie(0) // 0
/// ```
pub fn sign_normie(x: Normie) -> Normie {
    if x > 0 { 1 } else if x < 0 { -1 } else { 0 }
}

/// Check if normie is even (divisible by 2 vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = is_even_normie(42) // true
/// facts result = is_even_normie(43) // false
/// ```
pub fn is_even_normie(x: Normie) -> bool {
    math::is_even(x)
/// Check if normie is odd (not divisible by 2 vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = is_odd_normie(42) // false
/// facts result = is_odd_normie(43) // true
/// ```
pub fn is_odd_normie(x: Normie) -> bool {
    math::is_odd(x)
/// Greatest common divisor of normie numbers (common factor vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = gcd_normie(12, 8) // 4
/// ```
pub fn gcd_normie(a: Normie, b: Normie) -> Normie {
    math::gcd(a, b)
/// Least common multiple of normie numbers (common multiple vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = lcm_normie(12, 8) // 24
/// ```
pub fn lcm_normie(a: Normie, b: Normie) -> Normie {
    math::lcm(a, b)
// ================================
// THICC ARITHMETIC (BIG VIBES)
// ================================

/// Absolute value for thicc numbers (make it positive big vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = abs_thicc(-9223372036854775807) // 9223372036854775807
/// ```
pub fn abs_thicc(x: Thicc) -> Thicc {
    math::abs_i64(x)
/// Minimum of two thicc numbers (smallest big vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = min_thicc(1000000000, 2000000000) // 1000000000
/// ```
pub fn min_thicc(a: Thicc, b: Thicc) -> Thicc {
    math::min_i64(a, b)
/// Maximum of two thicc numbers (biggest big vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = max_thicc(1000000000, 2000000000) // 2000000000
/// ```
pub fn max_thicc(a: Thicc, b: Thicc) -> Thicc {
    math::max_i64(a, b)
/// Clamp thicc between bounds (keep it in range big vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = clamp_thicc(2500000000, 1000000000, 2000000000) // 2000000000
/// ```
pub fn clamp_thicc(value: Thicc, min: Thicc, max: Thicc) -> Thicc {
    math::clamp_i64(value, min, max)
/// Sign of thicc number (direction big vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = sign_thicc(-1000000000) // -1
/// ```
pub fn sign_thicc(x: Thicc) -> Thicc {
    if x > 0 { 1 } else if x < 0 { -1 } else { 0 }
}

/// Check if thicc is even (divisible by 2 big vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = is_even_thicc(1000000000) // true
/// ```
pub fn is_even_thicc(x: Thicc) -> bool {
    x % 2 == 0
/// Check if thicc is odd (not divisible by 2 big vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = is_odd_thicc(1000000001) // true
/// ```
pub fn is_odd_thicc(x: Thicc) -> bool {
    x % 2 != 0
// ================================
// CHONKY ARITHMETIC (PRECISE VIBES)
// ================================

/// Absolute value for chonky numbers (make it positive precise vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = abs_chonky(-3.14159) // 3.14159
/// ```
pub fn abs_chonky(x: Chonky) -> Chonky {
    math::abs(x)
/// Minimum of two chonky numbers (smallest precise vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = min_chonky(3.14, 2.71) // 2.71
/// ```
pub fn min_chonky(a: Chonky, b: Chonky) -> Chonky {
    math::min(a, b)
/// Maximum of two chonky numbers (biggest precise vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = max_chonky(3.14, 2.71) // 3.14
/// ```
pub fn max_chonky(a: Chonky, b: Chonky) -> Chonky {
    math::max(a, b)
/// Clamp chonky between bounds (keep it in range precise vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = clamp_chonky(5.0, 1.0, 4.0) // 4.0
/// ```
pub fn clamp_chonky(value: Chonky, min: Chonky, max: Chonky) -> Chonky {
    math::clamp(value, min, max)
/// Sign of chonky number (direction precise vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = sign_chonky(-3.14) // -1.0
/// ```
pub fn sign_chonky(x: Chonky) -> Chonky {
    math::sign(x)
/// Floor of chonky number (round down vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = floor_chonky(3.7) // 3.0
/// ```
pub fn floor_chonky(x: Chonky) -> Chonky {
    math::floor(x)
/// Ceiling of chonky number (round up vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = ceil_chonky(3.2) // 4.0
/// ```
pub fn ceil_chonky(x: Chonky) -> Chonky {
    math::ceil(x)
/// Round chonky number (nearest vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = round_chonky(3.7) // 4.0
/// facts result = round_chonky(3.2) // 3.0
/// ```
pub fn round_chonky(x: Chonky) -> Chonky {
    math::round(x)
/// Truncate chonky number (cut decimal vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = truncate_chonky(3.7) // 3.0
/// facts result = truncate_chonky(-3.7) // -3.0
/// ```
pub fn truncate_chonky(x: Chonky) -> Chonky {
    math::math_truncate(x)
/// Fractional part of chonky number (decimal vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = fract_chonky(3.7) // 0.7
/// ```
pub fn fract_chonky(x: Chonky) -> Chonky {
    math::fract(x)
/// Check if chonky is zero (empty vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = is_zero_chonky(0.0) // true
/// facts result = is_zero_chonky(0.0001) // false
/// ```
pub fn is_zero_chonky(x: Chonky) -> bool {
    math::is_zero(x)
/// Check if two chonky numbers are equal (same vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = is_equal_chonky(3.14159, 3.14159, 0.00001) // true
/// ```
pub fn is_equal_chonky(a: Chonky, b: Chonky, epsilon: Chonky) -> bool {
    math::is_equal(a, b, epsilon)
// ================================
// POWER AND ROOT FUNCTIONS (EXPONENTIAL VIBES)
// ================================

/// Power function for chonky numbers (exponential vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = pow_chonky(2.0, 3.0) // Ok(8.0)
/// ```
pub fn pow_chonky(base: Chonky, exponent: Chonky) -> MathzResult<Chonky> {
    math::pow(base, exponent)
/// Integer power function for chonky numbers (whole exponent vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = powi_chonky(2.0, 3) // Ok(8.0)
/// ```
pub fn powi_chonky(base: Chonky, exponent: Normie) -> MathzResult<Chonky> {
    math::powi(base, exponent)
/// Square function (double vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = square_chonky(5.0) // 25.0
/// ```
pub fn square_chonky(x: Chonky) -> Chonky {
    math::square(x)
/// Cube function (triple vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = cube_chonky(3.0) // 27.0
/// ```
pub fn cube_chonky(x: Chonky) -> Chonky {
    math::cube(x)
/// Square root function (half power vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = sqrt_chonky(25.0) // Ok(5.0)
/// ```
pub fn sqrt_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::sqrt(x)
/// Cube root function (third root vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = cbrt_chonky(27.0) // Ok(3.0)
/// ```
pub fn cbrt_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::cbrt(x)
/// Nth root function (any root vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = nth_root_chonky(16.0, 4.0) // Ok(2.0)
/// ```
pub fn nth_root_chonky(x: Chonky, n: Chonky) -> MathzResult<Chonky> {
    math::nth_root(x, n)
/// Hypotenuse function (triangle vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = hypot_chonky(3.0, 4.0) // 5.0
/// ```
pub fn hypot_chonky(x: Chonky, y: Chonky) -> Chonky {
    math::hypot(x, y)
/// Reciprocal function (inverse vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = reciprocal_chonky(4.0) // Ok(0.25)
/// ```
pub fn reciprocal_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::reciprocal(x)
// ================================
// LOGARITHMIC AND EXPONENTIAL FUNCTIONS (LOG VIBES)
// ================================

/// Natural logarithm (ln vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = ln_chonky(2.71828) // Ok(≈1.0)
/// ```
pub fn ln_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::ln(x)
/// Base-10 logarithm (log vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = log10_chonky(100.0) // Ok(2.0)
/// ```
pub fn log10_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::log10(x)
/// Base-2 logarithm (binary log vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = log2_chonky(8.0) // Ok(3.0)
/// ```
pub fn log2_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::log2(x)
/// Arbitrary base logarithm (custom log vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = log_chonky(27.0, 3.0) // Ok(3.0)
/// ```
pub fn log_chonky(x: Chonky, base: Chonky) -> MathzResult<Chonky> {
    math::log(x, base)
/// Natural exponential (e^x vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = exp_chonky(1.0) // Ok(≈2.71828)
/// ```
pub fn exp_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::exp(x)
/// Base-2 exponential (2^x vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = exp2_chonky(3.0) // Ok(8.0)
/// ```
pub fn exp2_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::exp2(x)
/// Base-10 exponential (10^x vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = exp10_chonky(2.0) // Ok(100.0)
/// ```
pub fn exp10_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::exp10(x)
// ================================
// TRIGONOMETRIC FUNCTIONS (TRIG VIBES)
// ================================

/// Sine function (sin vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = sin_chonky(PI / 2.0) // Ok(1.0)
/// ```
pub fn sin_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::sin(x)
/// Cosine function (cos vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = cos_chonky(0.0) // Ok(1.0)
/// ```
pub fn cos_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::cos(x)
/// Tangent function (tan vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = tan_chonky(PI / 4.0) // Ok(1.0)
/// ```
pub fn tan_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::tan(x)
/// Arcsine function (inverse sin vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = asin_chonky(1.0) // Ok(PI / 2.0)
/// ```
pub fn asin_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::asin(x)
/// Arccosine function (inverse cos vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = acos_chonky(1.0) // Ok(0.0)
/// ```
pub fn acos_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::acos(x)
/// Arctangent function (inverse tan vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = atan_chonky(1.0) // Ok(PI / 4.0)
/// ```
pub fn atan_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::atan(x)
/// Two-argument arctangent (atan2 vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = atan2_chonky(1.0, 1.0) // Ok(PI / 4.0)
/// ```
pub fn atan2_chonky(y: Chonky, x: Chonky) -> MathzResult<Chonky> {
    math::atan2(y, x)
// ================================
// HYPERBOLIC FUNCTIONS (HYPERBOLIC VIBES)
// ================================

/// Hyperbolic sine (sinh vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = sinh_chonky(0.0) // Ok(0.0)
/// ```
pub fn sinh_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::sinh(x)
/// Hyperbolic cosine (cosh vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = cosh_chonky(0.0) // Ok(1.0)
/// ```
pub fn cosh_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::cosh(x)
/// Hyperbolic tangent (tanh vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = tanh_chonky(0.0) // Ok(0.0)
/// ```
pub fn tanh_chonky(x: Chonky) -> MathzResult<Chonky> {
    math::tanh(x)
// ================================
// MATHEMATICAL CONSTANTS (CONST VIBES)
// ================================

/// Pi constant (circle vibes)
pub const PI_CHONKY: Chonky = std::f64::consts::PI;

/// Tau constant (full circle vibes)
pub const TAU_CHONKY: Chonky = std::f64::consts::TAU;

/// Euler's number (e vibes)
pub const E_CHONKY: Chonky = std::f64::consts::E;

/// Golden ratio (phi vibes)
pub const PHI_CHONKY: Chonky = 1.618033988749895;

/// Square root of 2 (sqrt 2 vibes)
pub const SQRT_2_CHONKY: Chonky = std::f64::consts::SQRT_2;

/// Natural log of 2 (ln 2 vibes)
pub const LN_2_CHONKY: Chonky = std::f64::consts::LN_2;

/// Natural log of 10 (ln 10 vibes)
pub const LN_10_CHONKY: Chonky = std::f64::consts::LN_10;

// ================================
// RANDOM NUMBER GENERATION (RANDOM VIBES)
// ================================

/// Generate random chonky number [0.0, 1.0) (random vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = random_chonky() // 0.42...
/// ```
pub fn random_chonky() -> Chonky {
    math::random()
/// Generate random chonky number in range (ranged random vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = random_range_chonky(1.0, 10.0) // 7.3...
/// ```
pub fn random_range_chonky(min: Chonky, max: Chonky) -> Chonky {
    math::random_range(min, max)
/// Generate random normie number (random int vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = random_normie() // 42
/// ```
pub fn random_normie() -> Normie {
    math::random_int()
/// Generate random boolean (random truth vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = random_bool() // true or false
/// ```
pub fn random_bool() -> bool {
    math::random_bool()
// ================================
// INTERPOLATION FUNCTIONS (SMOOTH VIBES)
// ================================

/// Linear interpolation (smooth transition vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = lerp_chonky(0.0, 10.0, 0.5) // 5.0
/// ```
pub fn lerp_chonky(start: Chonky, end: Chonky, t: Chonky) -> Chonky {
    math::lerp(start, end, t)
/// Inverse linear interpolation (reverse smooth vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = inverse_lerp_chonky(0.0, 10.0, 5.0) // 0.5
/// ```
pub fn inverse_lerp_chonky(start: Chonky, end: Chonky, value: Chonky) -> Chonky {
    math::inverse_lerp(start, end, value)
/// Smooth step interpolation (cubic smooth vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = smooth_step_chonky(0.0, 1.0, 0.5) // 0.5
/// ```
pub fn smooth_step_chonky(edge0: Chonky, edge1: Chonky, x: Chonky) -> Chonky {
    math::smooth_step(edge0, edge1, x)
/// Smoother step interpolation (quintic smooth vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = smoother_step_chonky(0.0, 1.0, 0.5) // 0.5
/// ```
pub fn smoother_step_chonky(edge0: Chonky, edge1: Chonky, x: Chonky) -> Chonky {
    math::smoother_step(edge0, edge1, x)
/// Map value from one range to another (range mapping vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = map_range_chonky(5.0, 0.0, 10.0, 0.0, 100.0) // 50.0
/// ```
pub fn map_range_chonky(value: Chonky, from_min: Chonky, from_max: Chonky, to_min: Chonky, to_max: Chonky) -> Chonky {
    math::map_range(value, from_min, from_max, to_min, to_max)
// ================================
// UTILITY FUNCTIONS
// ================================

/// Validate chonky number (check for NaN/infinity)
/// 
/// # Examples
/// ```cursed
/// facts result = is_valid_chonky(3.14) // true
/// facts result = is_valid_chonky(Chonky::NAN) // false
/// ```
pub fn is_valid_chonky(x: Chonky) -> bool {
    math::is_valid_float(x)
/// Round chonky to specified decimal places (precision vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = round_to_decimals_chonky(3.14159, 2) // Ok(3.14)
/// ```
pub fn round_to_decimals_chonky(x: Chonky, decimals: u32) -> MathzResult<Chonky> {
    math::round_to_decimals(x, decimals)
/// Calculate average of chonky numbers (mean vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = average_chonky(&[1.0, 2.0, 3.0, 4.0, 5.0]) // 3.0
/// ```
pub fn average_chonky(values: &[Chonky]) -> MathzResult<Chonky> {
    if values.is_empty() {
        return Err(MathzError::DomainError("Cannot compute average of empty slice".to_string()));
    let sum: Chonky = values.iter().sum();
    Ok(sum / values.len() as Chonky)
/// Calculate geometric mean of chonky numbers (geo mean vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = geometric_mean_chonky(&[1.0, 4.0, 9.0]) // Ok(3.0)
/// ```
pub fn geometric_mean_chonky(values: &[Chonky]) -> MathzResult<Chonky> {
    if values.is_empty() {
        return Err(MathzError::DomainError("Cannot compute geometric mean of empty slice".to_string()));
    if values.iter().any(|&x| x <= 0.0) {
        return Err(MathzError::DomainError("All values must be positive for geometric mean".to_string()));
    let product: Chonky = values.iter().map(|&x| x.ln()).sum();
    Ok((product / values.len() as Chonky).exp())
/// Calculate harmonic mean of chonky numbers (harmonic mean vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = harmonic_mean_chonky(&[1.0, 2.0, 4.0]) // Ok(1.71...)
/// ```
pub fn harmonic_mean_chonky(values: &[Chonky]) -> MathzResult<Chonky> {
    if values.is_empty() {
        return Err(MathzError::DomainError("Cannot compute harmonic mean of empty slice".to_string()));
    if values.iter().any(|&x| x == 0.0) {
        return Err(MathzError::DomainError("Cannot compute harmonic mean with zero values".to_string()));
    let reciprocal_sum: Chonky = values.iter().map(|&x| 1.0 / x).sum();
    Ok(values.len() as Chonky / reciprocal_sum)
/// Module initialization function
pub fn init_mathz() -> MathzResult<()> {
    // Initialize any global state for MathZ module
    Ok(())
/// Get module statistics and information
pub fn get_mathz_stats() -> HashMap<String, String> {
    let mut stats = HashMap::new();
    stats.insert("version".to_string(), "1.0.0".to_string());
    stats.insert("functions".to_string(), "70+".to_string());
    stats.insert("features".to_string(), "CURSED types, Gen Z naming, comprehensive math".to_string());
    stats.insert("types".to_string(), "normie, thicc, smol, chonky".to_string());
    stats
