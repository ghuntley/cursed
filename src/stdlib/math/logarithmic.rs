/// Logarithmic and exponential functions

use super::{MathError, MathResult, validate_float, domain_error, negative_input_error, division_by_zero_error};

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
    Ok(x * x)
}

/// Cube function
pub fn cube(x: f64) -> MathResult<f64> {
    validate_float("cube", "x", x)?;
    Ok(x * x * x)
}
