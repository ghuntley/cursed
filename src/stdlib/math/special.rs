/// Special mathematical functions

use super::{MathError, MathResult, validate_float, domain_error, negative_input_error};
use super::constants::{PI, E};
use crate::error::CursedError;

/// Factorial function for non-negative integers
pub fn factorial(n: u64) -> MathResult<u64> {
    if n > 20 {
        return Err(MathError::IntegerOverflow {
        });
    let mut result = 1u64;
    for i in 2..=n {
        result = result.checked_mul(i).ok_or_else(|| MathError::IntegerOverflow {
        })?;
    Ok(result)
/// Factorial function returning f64 for larger values
pub fn factorial_f64(n: f64) -> MathResult<f64> {
    validate_float("factorial_f64", "n", n)?;
    
    if n < 0.0 || n.fract() != 0.0 {
        return Err(domain_error("factorial_f64", n, "factorial requires non-negative integer"));
    if n > 170.0 {
        return Err(MathError::Overflow {
        });
    let mut result = 1.0;
    for i in 2..=(n as u64) {
        result *= i as f64;
    Ok(result)
/// Gamma function Γ(x) (generalization of factorial)
pub fn gamma(x: f64) -> MathResult<f64> {
    validate_float("gamma", "x", x)?;
    
    if x <= 0.0 && x.fract() == 0.0 {
        return Err(domain_error("gamma", x, "gamma function undefined for non-positive integers"));
    // Use the reflection formula for negative values
    if x < 0.0 {
        let sin_pi_x = (PI * x).sin();
        if sin_pi_x.abs() < f64::EPSILON {
            return Err(domain_error("gamma", x, "gamma function pole"));
        }
        let gamma_1_minus_x = gamma(1.0 - x)?;
        return Ok(PI / (sin_pi_x * gamma_1_minus_x));
    // Stirling's approximation for large values
    if x > 100.0 {
        let ln_gamma_val = ln_gamma_stirling(x);
        return Ok(ln_gamma_val.exp());
    // Use the recurrence relation to reduce to [1, 2)
    let mut result = 1.0;
    let mut z = x;
    
    while z >= 2.0 {
        z -= 1.0;
        result *= z;
    while z < 1.0 {
        result /= z;
        z += 1.0;
    // Approximate gamma(z) for z in [1, 2) using series expansion
    let gamma_1_to_2 = gamma_series(z);
    Ok(result * gamma_1_to_2)
/// Logarithm of gamma function using Stirling's approximation
fn ln_gamma_stirling(x: f64) -> f64 {
    let ln_sqrt_2pi = 0.9189385332046727; // ln(sqrt(2π))
    (x - 0.5) * x.ln() - x + ln_sqrt_2pi + 1.0 / (12.0 * x)
/// Gamma function series expansion for z in [1, 2)
fn gamma_series(z: f64) -> f64 {
    // Coefficients for the series expansion around z = 1
    const COEFFS: &[f64] = &[
        -0.5772156649015329,  // -γ (Euler-Mascheroni constant)
    ];
    
    let dt = z - 1.0;
    let mut result = 0.0;
    let mut dt_pow = 1.0;
    
    for &coeff in COEFFS {
        result += coeff * dt_pow;
        dt_pow *= dt;
    result
/// Beta function B(x, y) = Γ(x)Γ(y)/Γ(x+y)
pub fn beta(x: f64, y: f64) -> MathResult<f64> {
    validate_float("beta", "x", x)?;
    validate_float("beta", "y", y)?;
    
    if x <= 0.0 || y <= 0.0 {
        return Err(domain_error("beta", x.min(y), "beta function requires positive arguments"));
    let gamma_x = gamma(x)?;
    let gamma_y = gamma(y)?;
    let gamma_xy = gamma(x + y)?;
    
    Ok(gamma_x * gamma_y / gamma_xy)
/// Binomial coefficient "n choose k"
pub fn binomial(n: u64, k: u64) -> MathResult<u64> {
    if k > n {
        return Ok(0);
    if k == 0 || k == n {
        return Ok(1);
    // Use symmetry: C(n,k) = C(n,n-k)
    let k = k.min(n - k);
    
    let mut result = 1u64;
    for i in 0..k {
        result = result
            .checked_mul(n - i)
            .and_then(|r| r.checked_div(i + 1))
            .ok_or_else(|| MathError::IntegerOverflow {
            })?;
    Ok(result)
/// Binomial coefficient for large values (returns f64)
pub fn binomial_f64(n: f64, k: f64) -> MathResult<f64> {
    validate_float("binomial_f64", "n", n)?;
    validate_float("binomial_f64", "k", k)?;
    
    if k < 0.0 || n < 0.0 || k > n {
        return Ok(0.0);
    if k.fract() != 0.0 || n.fract() != 0.0 {
        return Err(domain_error("binomial_f64", k, "binomial coefficient requires integer arguments"));
    // Use the gamma function: C(n,k) = Γ(n+1) / (Γ(k+1) * Γ(n-k+1))
    let gamma_n_plus_1 = gamma(n + 1.0)?;
    let gamma_k_plus_1 = gamma(k + 1.0)?;
    let gamma_n_minus_k_plus_1 = gamma(n - k + 1.0)?;
    
    Ok(gamma_n_plus_1 / (gamma_k_plus_1 * gamma_n_minus_k_plus_1))
/// Number of permutations P(n, k) = n! / (n-k)!
pub fn permutations(n: u64, k: u64) -> MathResult<u64> {
    if k > n {
        return Ok(0);
    let mut result = 1u64;
    for i in 0..k {
        result = result.checked_mul(n - i).ok_or_else(|| MathError::IntegerOverflow {
        })?;
    Ok(result)
/// CursedError function erf(x)
pub fn erf(x: f64) -> MathResult<f64> {
    validate_float("erf", "x", x)?;
    
    // Abramowitz and Stegun approximation
    let a1 =  0.254829592;
    let a2 = -0.284496736;
    let a3 =  1.421413741;
    let a4 = -1.453152027;
    let a5 =  1.061405429;
    let p  =  0.3275911;
    
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    
    Ok(sign * y)
/// Complementary error function erfc(x) = 1 - erf(x)
pub fn erfc(x: f64) -> MathResult<f64> {
    validate_float("erfc", "x", x)?;
    
    let erf_val = erf(x)?;
    Ok(1.0 - erf_val)
/// Inverse error function
pub fn erf_inv(x: f64) -> MathResult<f64> {
    validate_float("erf_inv", "x", x)?;
    
    if x <= -1.0 || x >= 1.0 {
        return Err(domain_error("erf_inv", x, "inverse error function domain is (-1, 1)"));
    // Approximation using rational functions
    // This is a simplified implementation
    if x.abs() <= 0.7 {
        let x2 = x * x;
        let r = x * (0.886226899 + x2 * (-1.645349621 + x2 * 0.914624893));
        Ok(r)
    } else {
        let sign = if x > 0.0 { 1.0 } else { -1.0 };
        let t = (-((1.0 - x.abs()).ln())).sqrt();
        let result = sign * (2.515517 + t * (0.802853 + t * 0.010328)) / 
                     (1.0 + t * (1.432788 + t * (0.189269 + t * 0.001308)));
        Ok(result - sign * t)
    }
}

/// Bessel function of the first kind J₀(x)
pub fn bessel_j0(x: f64) -> MathResult<f64> {
    validate_float("bessel_j0", "x", x)?;
    
    // Simplified implementation for J₀(x)
    // For small x, use series expansion
    if x.abs() < 1.0 {
        let x2 = x * x;
        Ok(1.0 - x2 / 4.0 + x2 * x2 / 64.0)
    } else {
        // For larger x, use asymptotic expansion
        let sqrt_2_pi_x = (2.0 / (PI * x)).sqrt();
        let phase = x - PI / 4.0;
        Ok(sqrt_2_pi_x * phase.cos())
    }
}

/// Bessel function of the first kind J₁(x)
pub fn bessel_j1(x: f64) -> MathResult<f64> {
    validate_float("bessel_j1", "x", x)?;
    
    // Simplified implementation for J₁(x)
    // For small x, use series expansion
    if x.abs() < 1.0 {
        let x2 = x * x;
        Ok(x / 2.0 * (1.0 - x2 / 8.0 + x2 * x2 / 192.0))
    } else {
        // For larger x, use asymptotic expansion
        let sqrt_2_pi_x = (2.0 / (PI * x)).sqrt();
        let phase = x - 3.0 * PI / 4.0;
        Ok(sqrt_2_pi_x * phase.cos())
    }
}

/// Bessel function of the second kind Y₀(x)
pub fn bessel_y0(x: f64) -> MathResult<f64> {
    validate_float("bessel_y0", "x", x)?;
    
    if x <= 0.0 {
        return Err(domain_error("bessel_y0", x, "Bessel Y0 requires positive argument"));
    // Simplified implementation for Y₀(x)
    if x < 1.0 {
        let j0_val = bessel_j0(x)?;
        Ok((2.0 / PI) * (x.ln() + 0.5772156649) * j0_val)
    } else {
        let sqrt_2_pi_x = (2.0 / (PI * x)).sqrt();
        let phase = x - PI / 4.0;
        Ok(sqrt_2_pi_x * phase.sin())
    }
}

/// Bessel function of the second kind Y₁(x)
pub fn bessel_y1(x: f64) -> MathResult<f64> {
    validate_float("bessel_y1", "x", x)?;
    
    if x <= 0.0 {
        return Err(domain_error("bessel_y1", x, "Bessel Y1 requires positive argument"));
    // Simplified implementation for Y₁(x)
    if x < 1.0 {
        let j1_val = bessel_j1(x)?;
        Ok((2.0 / PI) * (x.ln() + 0.5772156649 - 1.0) * j1_val - 2.0 / (PI * x))
    } else {
        let sqrt_2_pi_x = (2.0 / (PI * x)).sqrt();
        let phase = x - 3.0 * PI / 4.0;
        Ok(sqrt_2_pi_x * phase.sin())
    }
}

/// Fibonacci number
pub fn fibonacci(n: u64) -> MathResult<u64> {
    if n > 93 {
        return Err(MathError::IntegerOverflow {
        });
    if n <= 1 {
        return Ok(n);
    let mut a = 0u64;
    let mut b = 1u64;
    
    for _ in 2..=n {
        let next = a.checked_add(b).ok_or_else(|| MathError::IntegerOverflow {
        })?;
        a = b;
        b = next;
    Ok(b)
/// Lucas number
pub fn lucas(n: u64) -> MathResult<u64> {
    if n == 0 {
        return Ok(2);
    }
    if n == 1 {
        return Ok(1);
    if n > 92 {
        return Err(MathError::IntegerOverflow {
        });
    let mut a = 2u64;
    let mut b = 1u64;
    
    for _ in 2..=n {
        let next = a.checked_add(b).ok_or_else(|| MathError::IntegerOverflow {
        })?;
        a = b;
        b = next;
    Ok(b)
/// Catalan number
pub fn catalan(n: u64) -> MathResult<u64> {
    if n > 33 {
        return Err(MathError::IntegerOverflow {
        });
    if n == 0 {
        return Ok(1);
    // C_n = (2n)! / ((n+1)! * n!)
    // Or equivalently: C_n = C(2n, n) / (n+1)
    let binom_2n_n = binomial(2 * n, n)?;
    Ok(binom_2n_n / (n + 1))
}
