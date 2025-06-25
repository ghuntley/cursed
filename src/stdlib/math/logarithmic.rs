use crate::error::CursedError;
/// Comprehensive logarithmic and exponential functions for CURSED
/// 
/// This module provides extensive mathematical functions for logarithmic and exponential
/// operations with proper domain validation, overflow protection, and high precision.

use super::{MathError, MathResult, validate_float, domain_error, negative_input_error, division_by_zero_error};
use std::f64::consts::{E, LN_2, LN_10, LOG2_E, LOG10_E};

/// Natural logarithm (base e)
pub fn ln(x: f64) -> MathResult<f64> {
    validate_float("ln", "x", x)?;
    
    if x <= 0.0 {
        return Err(negative_input_error("ln", x));
    }
    
    Ok(x.ln())
}

/// Common logarithm (base 10)
pub fn log10(x: f64) -> MathResult<f64> {
    validate_float("log10", "x", x)?;
    
    if x <= 0.0 {
        return Err(negative_input_error("log10", x));
    }
    
    Ok(x.log10())
}

/// Binary logarithm (base 2)
pub fn log2(x: f64) -> MathResult<f64> {
    validate_float("log2", "x", x)?;
    
    if x <= 0.0 {
        return Err(negative_input_error("log2", x));
    }
    
    Ok(x.log2())
}

/// Logarithm with arbitrary base
pub fn log(x: f64, base: f64) -> MathResult<f64> {
    validate_float("log", "x", x)?;
    validate_float("log", "base", base)?;
    
    if x <= 0.0 {
        return Err(negative_input_error("log", x));
    }
    
    if base <= 0.0 || base == 1.0 {
        return Err(domain_error("log", base, "base must be positive and not equal to 1"));
    }
    
    Ok(x.ln() / base.ln())
}

/// Natural exponential function (e^x)
pub fn exp(x: f64) -> MathResult<f64> {
    validate_float("exp", "x", x)?;
    
    let result = x.exp();
    if !result.is_finite() {
        return Err(MathError::Overflow {
            function: "exp".to_string(),
            value: x,
        });
    }
    
    Ok(result)
}

/// Exponential function base 2 (2^x)
pub fn exp2(x: f64) -> MathResult<f64> {
    validate_float("exp2", "x", x)?;
    
    let result = x.exp2();
    if !result.is_finite() {
        return Err(MathError::Overflow {
            function: "exp2".to_string(),
            value: x,
        });
    }
    
    Ok(result)
}

/// Exponential function base 10 (10^x)
pub fn exp10(x: f64) -> MathResult<f64> {
    validate_float("exp10", "x", x)?;
    
    let result = 10.0_f64.powf(x);
    if !result.is_finite() {
        return Err(MathError::Overflow {
            function: "exp10".to_string(),
            value: x,
        });
    }
    
    Ok(result)
}

/// Power function (base^exponent)
pub fn pow(base: f64, exponent: f64) -> MathResult<f64> {
    validate_float("pow", "base", base)?;
    validate_float("pow", "exponent", exponent)?;
    
    // Special cases
    if base == 0.0 && exponent < 0.0 {
        return Err(domain_error("pow", base, "0^(negative) is undefined"));
    }
    
    if base < 0.0 && exponent.fract() != 0.0 {
        return Err(domain_error("pow", base, "negative base with non-integer exponent"));
    }
    
    let result = base.powf(exponent);
    if !result.is_finite() {
        return Err(MathError::Overflow {
            function: "pow".to_string(),
            value: base,
        });
    }
    
    Ok(result)
}

/// Integer power function (optimized for integer exponents)
pub fn powi(base: f64, exponent: i32) -> MathResult<f64> {
    validate_float("powi", "base", base)?;
    
    if base == 0.0 && exponent < 0 {
        return Err(domain_error("powi", base, "0^(negative) is undefined"));
    }
    
    let result = base.powi(exponent);
    if !result.is_finite() {
        return Err(MathError::Overflow {
            function: "powi".to_string(),
            value: base,
        });
    }
    
    Ok(result)
}

/// Square root
pub fn sqrt(x: f64) -> MathResult<f64> {
    validate_float("sqrt", "x", x)?;
    
    if x < 0.0 {
        return Err(negative_input_error("sqrt", x));
    }
    
    Ok(x.sqrt())
}

/// Cube root
pub fn cbrt(x: f64) -> MathResult<f64> {
    validate_float("cbrt", "x", x)?;
    Ok(x.cbrt())
}

/// nth root
pub fn nth_root(x: f64, n: f64) -> MathResult<f64> {
    validate_float("nth_root", "x", x)?;
    validate_float("nth_root", "n", n)?;
    
    if n == 0.0 {
        return Err(division_by_zero_error("nth_root"));
    }
    
    // For even roots, x must be non-negative
    if n % 2.0 == 0.0 && x < 0.0 {
        return Err(domain_error("nth_root", x, "even root of negative number"));
    }
    
    let result = if x < 0.0 && n % 2.0 != 0.0 {
        // Odd root of negative number
        -(-x).powf(1.0 / n)
    } else {
        x.powf(1.0 / n)
    };
    
    if !result.is_finite() {
        return Err(MathError::Overflow {
            function: "nth_root".to_string(),
            value: x,
        });
    }
    
    Ok(result)
}

/// Hypotenuse function (sqrt(x^2 + y^2))
pub fn hypot(x: f64, y: f64) -> MathResult<f64> {
    validate_float("hypot", "x", x)?;
    validate_float("hypot", "y", y)?;
    
    Ok(x.hypot(y))
}

/// 3D hypotenuse function (sqrt(x^2 + y^2 + z^2))
pub fn hypot3(x: f64, y: f64, z: f64) -> MathResult<f64> {
    validate_float("hypot3", "x", x)?;
    validate_float("hypot3", "y", y)?;
    validate_float("hypot3", "z", z)?;
    
    let result = (x * x + y * y + z * z).sqrt();
    Ok(result)
}

/// exp(x) - 1, computed accurately for small x
pub fn expm1(x: f64) -> MathResult<f64> {
    validate_float("expm1", "x", x)?;
    
    let result = x.exp_m1();
    if !result.is_finite() {
        return Err(MathError::Overflow {
            function: "expm1".to_string(),
            value: x,
        });
    }
    
    Ok(result)
}

/// ln(1 + x), computed accurately for small x
pub fn ln1p(x: f64) -> MathResult<f64> {
    validate_float("ln1p", "x", x)?;
    
    if x <= -1.0 {
        return Err(domain_error("ln1p", x, "argument must be > -1"));
    }
    
    Ok(x.ln_1p())
}

/// Multiply and add: (x * y) + z
pub fn mul_add(x: f64, y: f64, z: f64) -> MathResult<f64> {
    validate_float("mul_add", "x", x)?;
    validate_float("mul_add", "y", y)?;
    validate_float("mul_add", "z", z)?;
    
    Ok(x.mul_add(y, z))
}

/// Fast inverse square root (1/sqrt(x))
pub fn inv_sqrt(x: f64) -> MathResult<f64> {
    validate_float("inv_sqrt", "x", x)?;
    
    if x <= 0.0 {
        return Err(negative_input_error("inv_sqrt", x));
    }
    
    Ok(1.0 / x.sqrt())
}

/// Logarithm of the gamma function
pub fn ln_gamma(x: f64) -> MathResult<f64> {
    validate_float("ln_gamma", "x", x)?;
    
    if x <= 0.0 {
        return Err(domain_error("ln_gamma", x, "gamma function undefined for non-positive integers"));
    }
    
    // Use Stirling's approximation for large values
    if x > 100.0 {
        let ln_sqrt_2pi = 0.9189385332046727; // ln(sqrt(2π))
        Ok((x - 0.5) * x.ln() - x + ln_sqrt_2pi)
    } else {
        // For smaller values, use a more accurate method
        // This is a simplified implementation using Stirling's approximation
        let ln_sqrt_2pi = 0.9189385332046727; // ln(sqrt(2π))
        Ok((x - 0.5) * x.ln() - x + ln_sqrt_2pi + 1.0 / (12.0 * x))
    }
}

/// Square function
pub fn square(x: f64) -> MathResult<f64> {
    validate_float("square", "x", x)?;
    
    // Check for overflow
    if x.abs() > f64::sqrt(f64::MAX) {
        return Err(MathError::Overflow {
            function: "square".to_string(),
            value: x,
        });
    }
    
    Ok(x * x)
}

/// Cube function
pub fn cube(x: f64) -> MathResult<f64> {
    validate_float("cube", "x", x)?;
    
    // Check for overflow
    if x.abs() > f64::cbrt(f64::MAX) {
        return Err(MathError::Overflow {
            function: "cube".to_string(),
            value: x,
        });
    }
    
    Ok(x * x * x)
}

// ==================== Enhanced Exponential Functions ====================

/// 2^x - 1, computed accurately for small x
pub fn exp2m1(x: f64) -> MathResult<f64> {
    validate_float("exp2m1", "x", x)?;
    
    // For small x, use the identity: 2^x - 1 = e^(x*ln(2)) - 1
    if x.abs() < 0.5 {
        let result = (x * LN_2).exp_m1();
        if !result.is_finite() {
            return Err(MathError::Overflow {
                function: "exp2m1".to_string(),
                value: x,
            });
        }
        Ok(result)
    } else {
        let result = x.exp2() - 1.0;
        if !result.is_finite() {
            return Err(MathError::Overflow {
                function: "exp2m1".to_string(),
                value: x,
            });
        }
        Ok(result)
    }
}

/// 10^x - 1, computed accurately for small x
pub fn exp10m1(x: f64) -> MathResult<f64> {
    validate_float("exp10m1", "x", x)?;
    
    // For small x, use the identity: 10^x - 1 = e^(x*ln(10)) - 1
    if x.abs() < 0.1 {
        let result = (x * LN_10).exp_m1();
        if !result.is_finite() {
            return Err(MathError::Overflow {
                function: "exp10m1".to_string(),
                value: x,
            });
        }
        Ok(result)
    } else {
        let result = exp10(x)? - 1.0;
        Ok(result)
    }
}

/// Exponential function with arbitrary base (base^x)
pub fn exp_base(base: f64, x: f64) -> MathResult<f64> {
    validate_float("exp_base", "base", base)?;
    validate_float("exp_base", "x", x)?;
    
    if base <= 0.0 {
        return Err(domain_error("exp_base", base, "base must be positive"));
    }
    
    if base == 1.0 {
        return Ok(1.0);
    }
    
    // Use the identity: base^x = e^(x * ln(base))
    let ln_base = base.ln();
    let exponent = x * ln_base;
    
    // Check for potential overflow before computing
    // Use a more conservative threshold since 2^1000 * ln(2) ≈ 693 > 700
    if exponent > 600.0 {
        return Err(MathError::Overflow {
            function: "exp_base".to_string(),
            value: x,
        });
    }
    
    let result = exponent.exp();
    if !result.is_finite() {
        return Err(MathError::Overflow {
            function: "exp_base".to_string(),
            value: x,
        });
    }
    
    Ok(result)
}

// ==================== Logarithmic Utilities and Transformations ====================

/// Log base 2 of absolute value (useful for bit operations)
pub fn log2_abs(x: f64) -> MathResult<f64> {
    validate_float("log2_abs", "x", x)?;
    
    if x == 0.0 {
        return Err(domain_error("log2_abs", x, "log of zero is undefined"));
    }
    
    Ok(x.abs().log2())
}

/// Log base 10 of absolute value
pub fn log10_abs(x: f64) -> MathResult<f64> {
    validate_float("log10_abs", "x", x)?;
    
    if x == 0.0 {
        return Err(domain_error("log10_abs", x, "log of zero is undefined"));
    }
    
    Ok(x.abs().log10())
}

/// Natural log of absolute value
pub fn ln_abs(x: f64) -> MathResult<f64> {
    validate_float("ln_abs", "x", x)?;
    
    if x == 0.0 {
        return Err(domain_error("ln_abs", x, "log of zero is undefined"));
    }
    
    Ok(x.abs().ln())
}

/// Logarithmic mean: (x - y) / (ln(x) - ln(y))
pub fn log_mean(x: f64, y: f64) -> MathResult<f64> {
    validate_float("log_mean", "x", x)?;
    validate_float("log_mean", "y", y)?;
    
    if x <= 0.0 || y <= 0.0 {
        return Err(domain_error("log_mean", x.min(y), "both values must be positive"));
    }
    
    if (x - y).abs() < f64::EPSILON {
        return Ok(x); // limit as x approaches y
    }
    
    let ln_x = x.ln();
    let ln_y = y.ln();
    
    if (ln_x - ln_y).abs() < f64::EPSILON {
        return Ok(x); // avoid division by zero
    }
    
    Ok((x - y) / (ln_x - ln_y))
}

/// Sigmoid function: 1 / (1 + e^(-x))
pub fn sigmoid(x: f64) -> MathResult<f64> {
    validate_float("sigmoid", "x", x)?;
    
    // Use stable computation to avoid overflow
    if x > 500.0 {
        Ok(1.0)
    } else if x < -500.0 {
        Ok(0.0)
    } else if x >= 0.0 {
        let exp_neg_x = (-x).exp();
        Ok(1.0 / (1.0 + exp_neg_x))
    } else {
        let exp_x = x.exp();
        Ok(exp_x / (1.0 + exp_x))
    }
}

/// Logistic function with parameters: L / (1 + e^(-k*(x-x0)))
pub fn logistic(x: f64, l: f64, k: f64, x0: f64) -> MathResult<f64> {
    validate_float("logistic", "x", x)?;
    validate_float("logistic", "l", l)?;
    validate_float("logistic", "k", k)?;
    validate_float("logistic", "x0", x0)?;
    
    if k == 0.0 {
        return Err(division_by_zero_error("logistic"));
    }
    
    let exponent = -k * (x - x0);
    
    // Use stable computation
    if exponent > 500.0 {
        Ok(0.0)
    } else if exponent < -500.0 {
        Ok(l)
    } else {
        let exp_val = exponent.exp();
        Ok(l / (1.0 + exp_val))
    }
}

/// Softmax function for a single value with reference values
pub fn softmax_single(x: f64, reference_values: &[f64]) -> MathResult<f64> {
    validate_float("softmax_single", "x", x)?;
    
    if reference_values.is_empty() {
        return Err(domain_error("softmax_single", 0.0, "reference values cannot be empty"));
    }
    
    // Validate all reference values
    for (i, &val) in reference_values.iter().enumerate() {
        validate_float("softmax_single", &format!("reference[{}]", i), val)?;
    }
    
    // Find max for numerical stability
    let max_val = reference_values.iter().fold(x, |acc, &val| acc.max(val));
    
    let exp_x = (x - max_val).exp();
    let sum_exp: f64 = reference_values.iter()
        .map(|&val| (val - max_val).exp())
        .sum::<f64>() + exp_x;
    
    if sum_exp == 0.0 {
        return Err(domain_error("softmax_single", max_val, "sum of exponentials is zero"));
    }
    
    Ok(exp_x / sum_exp)
}

/// Log-sum-exp function for numerical stability: log(exp(x1) + exp(x2) + ... + exp(xn))
pub fn log_sum_exp(values: &[f64]) -> MathResult<f64> {
    if values.is_empty() {
        return Err(domain_error("log_sum_exp", 0.0, "values cannot be empty"));
    }
    
    // Validate all values
    for (i, &val) in values.iter().enumerate() {
        validate_float("log_sum_exp", &format!("values[{}]", i), val)?;
    }
    
    let max_val = values.iter().fold(f64::NEG_INFINITY, |acc, &val| acc.max(val));
    
    if max_val.is_infinite() {
        if max_val.is_sign_positive() {
            return Ok(f64::INFINITY);
        } else {
            return Ok(f64::NEG_INFINITY);
        }
    }
    
    let sum_exp: f64 = values.iter()
        .map(|&val| (val - max_val).exp())
        .sum();
    
    if sum_exp == 0.0 {
        return Ok(f64::NEG_INFINITY);
    }
    
    Ok(max_val + sum_exp.ln())
}

// ==================== Power Functions with Different Bases ====================

/// Raise to power of e: e^x (alias for exp for clarity)
pub fn pow_e(x: f64) -> MathResult<f64> {
    exp(x)
}

/// Raise to power of 2: 2^x (alias for exp2 for clarity)
pub fn pow_2(x: f64) -> MathResult<f64> {
    exp2(x)
}

/// Raise to power of 10: 10^x (alias for exp10 for clarity)
pub fn pow_10(x: f64) -> MathResult<f64> {
    exp10(x)
}

/// Tetration: x^x^x^... (n times)
pub fn tetration(x: f64, n: u32) -> MathResult<f64> {
    validate_float("tetration", "x", x)?;
    
    if n == 0 {
        return Ok(1.0);
    }
    
    if x <= 0.0 {
        return Err(domain_error("tetration", x, "base must be positive"));
    }
    
    let mut result = x;
    for i in 1..n {
        if result > 10.0 {
            // Prevent overflow for large intermediate results
            return Err(MathError::Overflow {
                function: "tetration".to_string(),
                value: result,
            });
        }
        result = pow(x, result)?;
        
        // Additional overflow check
        if !result.is_finite() {
            return Err(MathError::Overflow {
                function: "tetration".to_string(),
                value: result,
            });
        }
    }
    
    Ok(result)
}

// ==================== Enhanced Domain Validation ====================

/// Check if a value is a valid logarithm input (positive and finite)
pub fn is_valid_log_input(x: f64) -> bool {
    x.is_finite() && x > 0.0
}

/// Check if a value is a valid exponential input (finite)
pub fn is_valid_exp_input(x: f64) -> bool {
    x.is_finite()
}

/// Safe logarithm that returns None for invalid inputs instead of error
pub fn safe_ln(x: f64) -> Option<f64> {
    if is_valid_log_input(x) {
        Some(x.ln())
    } else {
        None
    }
}

/// Safe exponential that returns None for invalid inputs or overflow
pub fn safe_exp(x: f64) -> Option<f64> {
    if is_valid_exp_input(x) {
        let result = x.exp();
        if result.is_finite() {
            Some(result)
        } else {
            None
        }
    } else {
        None
    }
}

/// Clamped logarithm - clamps input to valid range before computing
pub fn clamped_ln(x: f64, min_val: f64) -> MathResult<f64> {
    validate_float("clamped_ln", "x", x)?;
    validate_float("clamped_ln", "min_val", min_val)?;
    
    if min_val <= 0.0 {
        return Err(domain_error("clamped_ln", min_val, "minimum value must be positive"));
    }
    
    let clamped_x = x.max(min_val);
    Ok(clamped_x.ln())
}

/// Clamped exponential - clamps result to prevent overflow
pub fn clamped_exp(x: f64, max_result: f64) -> MathResult<f64> {
    validate_float("clamped_exp", "x", x)?;
    validate_float("clamped_exp", "max_result", max_result)?;
    
    if max_result <= 0.0 {
        return Err(domain_error("clamped_exp", max_result, "maximum result must be positive"));
    }
    
    let result = x.exp();
    if result.is_finite() {
        Ok(result.min(max_result))
    } else {
        Ok(max_result)
    }
}
