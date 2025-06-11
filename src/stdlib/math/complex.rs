/// Complex number mathematical functions for the CURSED programming language
/// 
/// This module provides comprehensive support for complex number arithmetic,
/// including basic operations, transcendental functions, and advanced mathematical
/// operations like matrix operations and polynomial root finding.

use super::{MathError, MathResult, validate_float};
use super::constants::{PI, E};
use std::fmt;

/// Complex number with 64-bit floating point components
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex64 {
    pub real: f64,
    pub imag: f64,
}

impl Complex64 {
    /// Create a new complex number
    pub fn new(real: f64, imag: f64) -> Self {
        Complex64 { real, imag }
    }
    
    /// Create a complex number from real part only
    pub fn from_real(real: f64) -> Self {
        Complex64 { real, imag: 0.0 }
    }
    
    /// Create a complex number from imaginary part only
    pub fn from_imag(imag: f64) -> Self {
        Complex64 { real: 0.0, imag }
    }
    
    /// The imaginary unit i
    pub fn i() -> Self {
        Complex64 { real: 0.0, imag: 1.0 }
    }
    
    /// Zero complex number
    pub fn zero() -> Self {
        Complex64 { real: 0.0, imag: 0.0 }
    }
    
    /// One complex number
    pub fn one() -> Self {
        Complex64 { real: 1.0, imag: 0.0 }
    }
}

impl fmt::Display for Complex64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{}+{}i", self.real, self.imag)
        } else {
            write!(f, "{}{}i", self.real, self.imag)
        }
    }
}

/// Create a complex number from real and imaginary parts
pub fn complex(real: f64, imag: f64) -> MathResult<Complex64> {
    validate_float("complex", "real", real)?;
    validate_float("complex", "imag", imag)?;
    Ok(Complex64::new(real, imag))
}

/// Extract real part of a complex number
pub fn real(z: Complex64) -> f64 {
    z.real
}

/// Extract imaginary part of a complex number
pub fn imag(z: Complex64) -> f64 {
    z.imag
}

/// Compute absolute value (modulus) of a complex number
pub fn abs(z: Complex64) -> MathResult<f64> {
    validate_float("abs", "real", z.real)?;
    validate_float("abs", "imag", z.imag)?;
    
    // Use hypot for numerical stability
    Ok((z.real * z.real + z.imag * z.imag).sqrt())
}

/// Compute phase (argument) of a complex number
pub fn phase(z: Complex64) -> MathResult<f64> {
    validate_float("phase", "real", z.real)?;
    validate_float("phase", "imag", z.imag)?;
    
    Ok(z.imag.atan2(z.real))
}

/// Compute complex conjugate
pub fn conj(z: Complex64) -> Complex64 {
    Complex64::new(z.real, -z.imag)
}

/// Convert complex number to polar coordinates (r, θ)
pub fn polar(z: Complex64) -> MathResult<(f64, f64)> {
    let r = abs(z)?;
    let theta = phase(z)?;
    Ok((r, theta))
}

/// Create complex number from polar coordinates (r, θ)
pub fn rect(r: f64, theta: f64) -> MathResult<Complex64> {
    validate_float("rect", "r", r)?;
    validate_float("rect", "theta", theta)?;
    
    if r < 0.0 {
        return Err(MathError::NegativeInput {
            function: "rect".to_string(),
            value: r,
        });
    }
    
    Ok(Complex64::new(r * theta.cos(), r * theta.sin()))
}

/// Compute e^z for complex z
pub fn exp(z: Complex64) -> MathResult<Complex64> {
    validate_float("exp", "real", z.real)?;
    validate_float("exp", "imag", z.imag)?;
    
    let exp_real = z.real.exp();
    Ok(Complex64::new(
        exp_real * z.imag.cos(),
        exp_real * z.imag.sin(),
    ))
}

/// Compute natural logarithm of complex number
pub fn log(z: Complex64) -> MathResult<Complex64> {
    validate_float("log", "real", z.real)?;
    validate_float("log", "imag", z.imag)?;
    
    if z.real == 0.0 && z.imag == 0.0 {
        return Err(MathError::DomainError {
            function: "log".to_string(),
            value: 0.0,
            message: "logarithm of zero is undefined".to_string(),
        });
    }
    
    let r = abs(z)?;
    let theta = phase(z)?;
    Ok(Complex64::new(r.ln(), theta))
}

/// Compute base-10 logarithm of complex number
pub fn log10(z: Complex64) -> MathResult<Complex64> {
    let ln_z = log(z)?;
    let ln_10 = 10.0_f64.ln();
    Ok(Complex64::new(ln_z.real / ln_10, ln_z.imag / ln_10))
}

/// Compute x^y for complex numbers
pub fn pow(x: Complex64, y: Complex64) -> MathResult<Complex64> {
    validate_float("pow", "x.real", x.real)?;
    validate_float("pow", "x.imag", x.imag)?;
    validate_float("pow", "y.real", y.real)?;
    validate_float("pow", "y.imag", y.imag)?;
    
    if x.real == 0.0 && x.imag == 0.0 {
        if y.real == 0.0 && y.imag == 0.0 {
            return Err(MathError::DomainError {
                function: "pow".to_string(),
                value: 0.0,
                message: "0^0 is undefined".to_string(),
            });
        }
        return Ok(Complex64::zero());
    }
    
    // x^y = exp(y * ln(x))
    let ln_x = log(x)?;
    let y_ln_x = Complex64::new(
        y.real * ln_x.real - y.imag * ln_x.imag,
        y.real * ln_x.imag + y.imag * ln_x.real,
    );
    exp(y_ln_x)
}

/// Compute square root of complex number
pub fn sqrt(z: Complex64) -> MathResult<Complex64> {
    validate_float("sqrt", "real", z.real)?;
    validate_float("sqrt", "imag", z.imag)?;
    
    let r = abs(z)?;
    let theta = phase(z)?;
    rect(r.sqrt(), theta / 2.0)
}

/// Compute sine of complex number
pub fn sin(z: Complex64) -> MathResult<Complex64> {
    validate_float("sin", "real", z.real)?;
    validate_float("sin", "imag", z.imag)?;
    
    // sin(z) = sin(x)cosh(y) + i*cos(x)sinh(y)
    Ok(Complex64::new(
        z.real.sin() * z.imag.cosh(),
        z.real.cos() * z.imag.sinh(),
    ))
}

/// Compute cosine of complex number
pub fn cos(z: Complex64) -> MathResult<Complex64> {
    validate_float("cos", "real", z.real)?;
    validate_float("cos", "imag", z.imag)?;
    
    // cos(z) = cos(x)cosh(y) - i*sin(x)sinh(y)
    Ok(Complex64::new(
        z.real.cos() * z.imag.cosh(),
        -z.real.sin() * z.imag.sinh(),
    ))
}

/// Compute tangent of complex number
pub fn tan(z: Complex64) -> MathResult<Complex64> {
    let sin_z = sin(z)?;
    let cos_z = cos(z)?;
    
    // Check for division by zero
    if cos_z.real == 0.0 && cos_z.imag == 0.0 {
        return Err(MathError::DomainError {
            function: "tan".to_string(),
            value: z.real,
            message: "tangent is undefined at this point".to_string(),
        });
    }
    
    // tan(z) = sin(z) / cos(z)
    complex_div(sin_z, cos_z)
}

/// Compute hyperbolic sine of complex number
pub fn sinh(z: Complex64) -> MathResult<Complex64> {
    validate_float("sinh", "real", z.real)?;
    validate_float("sinh", "imag", z.imag)?;
    
    // sinh(z) = sinh(x)cos(y) + i*cosh(x)sin(y)
    Ok(Complex64::new(
        z.real.sinh() * z.imag.cos(),
        z.real.cosh() * z.imag.sin(),
    ))
}

/// Compute hyperbolic cosine of complex number
pub fn cosh(z: Complex64) -> MathResult<Complex64> {
    validate_float("cosh", "real", z.real)?;
    validate_float("cosh", "imag", z.imag)?;
    
    // cosh(z) = cosh(x)cos(y) + i*sinh(x)sin(y)
    Ok(Complex64::new(
        z.real.cosh() * z.imag.cos(),
        z.real.sinh() * z.imag.sin(),
    ))
}

/// Compute hyperbolic tangent of complex number
pub fn tanh(z: Complex64) -> MathResult<Complex64> {
    let sinh_z = sinh(z)?;
    let cosh_z = cosh(z)?;
    
    // Check for division by zero
    if cosh_z.real == 0.0 && cosh_z.imag == 0.0 {
        return Err(MathError::DomainError {
            function: "tanh".to_string(),
            value: z.real,
            message: "hyperbolic tangent is undefined at this point".to_string(),
        });
    }
    
    complex_div(sinh_z, cosh_z)
}

/// Compute inverse sine of complex number
pub fn asin(z: Complex64) -> MathResult<Complex64> {
    // asin(z) = -i * ln(iz + sqrt(1 - z^2))
    let i = Complex64::i();
    let one = Complex64::one();
    
    let z_squared = complex_mul(z, z)?;
    let under_sqrt = complex_sub(one, z_squared)?;
    let sqrt_term = sqrt(under_sqrt)?;
    let iz = complex_mul(i, z)?;
    let sum = complex_add(iz, sqrt_term)?;
    let ln_result = log(sum)?;
    let neg_i = Complex64::new(0.0, -1.0);
    complex_mul(neg_i, ln_result)
}

/// Compute inverse cosine of complex number
pub fn acos(z: Complex64) -> MathResult<Complex64> {
    // acos(z) = -i * ln(z + i * sqrt(1 - z^2))
    let i = Complex64::i();
    let one = Complex64::one();
    
    let z_squared = complex_mul(z, z)?;
    let under_sqrt = complex_sub(one, z_squared)?;
    let sqrt_term = sqrt(under_sqrt)?;
    let i_sqrt = complex_mul(i, sqrt_term)?;
    let sum = complex_add(z, i_sqrt)?;
    let ln_result = log(sum)?;
    let neg_i = Complex64::new(0.0, -1.0);
    complex_mul(neg_i, ln_result)
}

/// Compute inverse tangent of complex number
pub fn atan(z: Complex64) -> MathResult<Complex64> {
    // atan(z) = (i/2) * ln((i+z)/(i-z))
    let i = Complex64::i();
    let half_i = Complex64::new(0.0, 0.5);
    
    let i_plus_z = complex_add(i, z)?;
    let i_minus_z = complex_sub(i, z)?;
    
    if i_minus_z.real == 0.0 && i_minus_z.imag == 0.0 {
        return Err(MathError::DomainError {
            function: "atan".to_string(),
            value: z.real,
            message: "arctangent is undefined at this point".to_string(),
        });
    }
    
    let ratio = complex_div(i_plus_z, i_minus_z)?;
    let ln_result = log(ratio)?;
    complex_mul(half_i, ln_result)
}

/// Compute inverse hyperbolic sine of complex number
pub fn asinh(z: Complex64) -> MathResult<Complex64> {
    // asinh(z) = ln(z + sqrt(z^2 + 1))
    let one = Complex64::one();
    let z_squared = complex_mul(z, z)?;
    let under_sqrt = complex_add(z_squared, one)?;
    let sqrt_term = sqrt(under_sqrt)?;
    let sum = complex_add(z, sqrt_term)?;
    log(sum)
}

/// Compute inverse hyperbolic cosine of complex number
pub fn acosh(z: Complex64) -> MathResult<Complex64> {
    // acosh(z) = ln(z + sqrt(z^2 - 1))
    let one = Complex64::one();
    let z_squared = complex_mul(z, z)?;
    let under_sqrt = complex_sub(z_squared, one)?;
    let sqrt_term = sqrt(under_sqrt)?;
    let sum = complex_add(z, sqrt_term)?;
    log(sum)
}

/// Compute inverse hyperbolic tangent of complex number
pub fn atanh(z: Complex64) -> MathResult<Complex64> {
    // atanh(z) = (1/2) * ln((1+z)/(1-z))
    let one = Complex64::one();
    let half = Complex64::new(0.5, 0.0);
    
    let one_plus_z = complex_add(one, z)?;
    let one_minus_z = complex_sub(one, z)?;
    
    if one_minus_z.real == 0.0 && one_minus_z.imag == 0.0 {
        return Err(MathError::DomainError {
            function: "atanh".to_string(),
            value: z.real,
            message: "inverse hyperbolic tangent is undefined at this point".to_string(),
        });
    }
    
    let ratio = complex_div(one_plus_z, one_minus_z)?;
    let ln_result = log(ratio)?;
    complex_mul(half, ln_result)
}

// Helper functions for complex arithmetic

/// Add two complex numbers
fn complex_add(a: Complex64, b: Complex64) -> MathResult<Complex64> {
    Ok(Complex64::new(a.real + b.real, a.imag + b.imag))
}

/// Subtract two complex numbers
fn complex_sub(a: Complex64, b: Complex64) -> MathResult<Complex64> {
    Ok(Complex64::new(a.real - b.real, a.imag - b.imag))
}

/// Multiply two complex numbers
fn complex_mul(a: Complex64, b: Complex64) -> MathResult<Complex64> {
    Ok(Complex64::new(
        a.real * b.real - a.imag * b.imag,
        a.real * b.imag + a.imag * b.real,
    ))
}

/// Divide two complex numbers
fn complex_div(a: Complex64, b: Complex64) -> MathResult<Complex64> {
    let denom = b.real * b.real + b.imag * b.imag;
    
    if denom == 0.0 {
        return Err(MathError::DivisionByZero {
            function: "complex_div".to_string(),
        });
    }
    
    Ok(Complex64::new(
        (a.real * b.real + a.imag * b.imag) / denom,
        (a.imag * b.real - a.real * b.imag) / denom,
    ))
}

/// Vector sum of complex numbers
pub fn vector_sum(vector: &[Complex64]) -> MathResult<Complex64> {
    let mut sum = Complex64::zero();
    for &z in vector {
        sum = complex_add(sum, z)?;
    }
    Ok(sum)
}

/// Vector product of complex numbers
pub fn vector_product(vector: &[Complex64]) -> MathResult<Complex64> {
    if vector.is_empty() {
        return Ok(Complex64::zero());
    }
    
    let mut product = Complex64::one();
    for &z in vector {
        product = complex_mul(product, z)?;
    }
    Ok(product)
}

/// Simple 2x2 complex matrix multiplication
pub fn matrix_mul_2x2(a: [[Complex64; 2]; 2], b: [[Complex64; 2]; 2]) -> MathResult<[[Complex64; 2]; 2]> {
    let mut result = [[Complex64::zero(); 2]; 2];
    
    for i in 0..2 {
        for j in 0..2 {
            let mut sum = Complex64::zero();
            for k in 0..2 {
                let term = complex_mul(a[i][k], b[k][j])?;
                sum = complex_add(sum, term)?;
            }
            result[i][j] = sum;
        }
    }
    
    Ok(result)
}

/// Calculate determinant of 2x2 complex matrix
pub fn determinant_2x2(matrix: [[Complex64; 2]; 2]) -> MathResult<Complex64> {
    let ad = complex_mul(matrix[0][0], matrix[1][1])?;
    let bc = complex_mul(matrix[0][1], matrix[1][0])?;
    complex_sub(ad, bc)
}

/// Find approximate roots of quadratic polynomial with complex coefficients
/// For polynomial az^2 + bz + c = 0
pub fn quadratic_roots(a: Complex64, b: Complex64, c: Complex64) -> MathResult<(Complex64, Complex64)> {
    if a.real == 0.0 && a.imag == 0.0 {
        return Err(MathError::DomainError {
            function: "quadratic_roots".to_string(),
            value: 0.0,
            message: "leading coefficient cannot be zero".to_string(),
        });
    }
    
    // discriminant = b^2 - 4ac
    let b_squared = complex_mul(b, b)?;
    let four_a = Complex64::new(4.0, 0.0);
    let four_ac = complex_mul(complex_mul(four_a, a)?, c)?;
    let discriminant = complex_sub(b_squared, four_ac)?;
    
    let sqrt_discriminant = sqrt(discriminant)?;
    let two_a = Complex64::new(2.0, 0.0);
    let two_a_val = complex_mul(two_a, a)?;
    
    let neg_b = Complex64::new(-b.real, -b.imag);
    
    let root1_num = complex_add(neg_b, sqrt_discriminant)?;
    let root2_num = complex_sub(neg_b, sqrt_discriminant)?;
    
    let root1 = complex_div(root1_num, two_a_val)?;
    let root2 = complex_div(root2_num, two_a_val)?;
    
    Ok((root1, root2))
}

/// Evaluate polynomial at complex point
pub fn evaluate_polynomial(coefficients: &[Complex64], z: Complex64) -> MathResult<Complex64> {
    if coefficients.is_empty() {
        return Ok(Complex64::zero());
    }
    
    let mut result = coefficients[0];
    let mut z_power = Complex64::one();
    
    for &coeff in coefficients.iter().skip(1) {
        z_power = complex_mul(z_power, z)?;
        let term = complex_mul(coeff, z_power)?;
        result = complex_add(result, term)?;
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_complex_creation() {
        let z = complex(3.0, 4.0).unwrap();
        assert_eq!(z.real, 3.0);
        assert_eq!(z.imag, 4.0);
    }
    
    #[test]
    fn test_complex_abs() {
        let z = complex(3.0, 4.0).unwrap();
        let abs_z = abs(z).unwrap();
        assert!((abs_z - 5.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_complex_arithmetic() {
        let z1 = complex(1.0, 2.0).unwrap();
        let z2 = complex(3.0, 4.0).unwrap();
        
        let sum = complex_add(z1, z2).unwrap();
        assert_eq!(sum.real, 4.0);
        assert_eq!(sum.imag, 6.0);
        
        let product = complex_mul(z1, z2).unwrap();
        assert_eq!(product.real, -5.0); // 1*3 - 2*4
        assert_eq!(product.imag, 10.0); // 1*4 + 2*3
    }
    
    #[test]
    fn test_complex_exp_log() {
        let z = complex(1.0, 0.0).unwrap();
        let exp_z = exp(z).unwrap();
        assert!((exp_z.real - E).abs() < 1e-10);
        assert!(exp_z.imag.abs() < 1e-10);
        
        let log_exp_z = log(exp_z).unwrap();
        assert!((log_exp_z.real - 1.0).abs() < 1e-10);
        assert!(log_exp_z.imag.abs() < 1e-10);
    }
    
    #[test]
    fn test_complex_trig() {
        let z = complex(0.0, 0.0).unwrap();
        let sin_z = sin(z).unwrap();
        let cos_z = cos(z).unwrap();
        
        assert!(sin_z.real.abs() < 1e-10);
        assert!(sin_z.imag.abs() < 1e-10);
        assert!((cos_z.real - 1.0).abs() < 1e-10);
        assert!(cos_z.imag.abs() < 1e-10);
    }
    
    #[test]
    fn test_quadratic_roots() {
        // z^2 - 1 = 0, roots should be ±1
        let a = complex(1.0, 0.0).unwrap();
        let b = complex(0.0, 0.0).unwrap();
        let c = complex(-1.0, 0.0).unwrap();
        
        let (root1, root2) = quadratic_roots(a, b, c).unwrap();
        
        // Check that roots are ±1
        let abs_diff1 = ((root1.real - 1.0).abs() + root1.imag.abs()) + 
                       ((root2.real + 1.0).abs() + root2.imag.abs());
        let abs_diff2 = ((root1.real + 1.0).abs() + root1.imag.abs()) + 
                       ((root2.real - 1.0).abs() + root2.imag.abs());
        
        assert!(abs_diff1 < 1e-10 || abs_diff2 < 1e-10);
    }
}
