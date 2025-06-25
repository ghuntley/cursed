/// Mathematical utilities module for CURSED programming language
/// 
/// Provides advanced mathematical functions including number theory, combinatorics,
/// special functions, numerical methods, sequences, and modular arithmetic.

use crate::error::CursedError;
// use crate::stdlib::math::{MathError, MathResult, validate_float, domain_error, range_error, division_by_zero_error, negative_input_error};
use std::collections::HashMap;

// ============================================================================
// Number Theory Functions
// ============================================================================

/// Extended Euclidean algorithm that returns (gcd, x, y) where ax + by = gcd(a, b)
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        (gcd, y1, x1 - (a / b) * y1)
    }
}

/// Check if a number is prime using trial division with optimizations
pub fn is_prime(n: i64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
/// Generate all prime numbers up to n using Sieve of Eratosthenes
pub fn sieve_of_eratosthenes(n: usize) -> Vec<i64> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    (2..=n).filter(|&i| is_prime[i]).map(|i| i as i64).collect()
/// Prime factorization of a number
pub fn prime_factorization(mut n: i64) -> crate::error::Result<()> {
    if n <= 0 {
        return Err(negative_input_error("prime_factorization", n as f64));
    let mut factors = Vec::new();
    let mut count = 0;
    
    // Handle factor 2
    while n % 2 == 0 {
        count += 1;
        n /= 2;
    }
    if count > 0 {
        factors.push((2, count));
    // Handle odd factors
    let mut i = 3;
    while i * i <= n {
        count = 0;
        while n % i == 0 {
            count += 1;
            n /= i;
        }
        if count > 0 {
            factors.push((i, count));
        }
        i += 2;
    // If n is still greater than 1, it's a prime factor
    if n > 1 {
        factors.push((n, 1));
    Ok(factors)
/// Find next prime number after n
pub fn next_prime(n: i64) -> i64 {
    let mut candidate = if n < 2 { 2 } else { n + 1 };
    while !is_prime(candidate) {
        candidate += 1;
    }
    candidate
/// Euler's totient function φ(n) - count of integers ≤ n that are coprime to n
pub fn euler_totient(n: i64) -> crate::error::Result<()> {
    if n <= 0 {
        return Err(negative_input_error("euler_totient", n as f64));
    let factors = prime_factorization(n)?;
    let mut result = n;
    
    for (p, _) in factors {
        result = result - result / p;
    Ok(result)
// ============================================================================
// Combinatorics Functions
// ============================================================================

/// Factorial function with overflow protection
pub fn factorial(n: i64) -> crate::error::Result<()> {
    if n < 0 {
        return Err(negative_input_error("factorial", n as f64));
    if n > 20 {  // 20! is max that fits in i64
        return Err(MathError::IntegerOverflow { function: "factorial".to_string(), value: n });
    let mut result = 1i64;
    for i in 2..=n {
        result = result.checked_mul(i).ok_or_else(|| 
            MathError::IntegerOverflow { function: "factorial".to_string(), value: n }
        )?;
    Ok(result)
/// Double factorial n!! = n * (n-2) * (n-4) * ...
pub fn double_factorial(n: i64) -> crate::error::Result<()> {
    if n < 0 {
        return Err(negative_input_error("double_factorial", n as f64));
    let mut result = 1i64;
    let mut i = n;
    while i > 0 {
        result = result.checked_mul(i).ok_or_else(|| 
            MathError::IntegerOverflow { function: "double_factorial".to_string(), value: n }
        )?;
        i -= 2;
    Ok(result)
/// Stirling's approximation for factorial (returns f64)
pub fn factorial_stirling(n: i64) -> crate::error::Result<()> {
    if n < 0 {
        return Err(negative_input_error("factorial_stirling", n as f64));
    let n_f = n as f64;
    // Stirling's approximation: n! ≈ √(2πn) * (n/e)^n
    let result = (2.0 * std::f64::consts::PI * n_f).sqrt() * (n_f / std::f64::consts::E).powf(n_f);
    
    validate_float("factorial_stirling", "result", result)?;
    Ok(result)
/// Number of permutations P(n, r) = n! / (n-r)!
pub fn permutations(n: i64, r: i64) -> crate::error::Result<()> {
    if n < 0 || r < 0 {
        return Err(domain_error("permutations", n as f64, &format!("n={}, r={} (both must be non-negative)", n, r)));
    if r > n {
        return Ok(0);
    let mut result = 1i64;
    for i in (n - r + 1)..=n {
        result = result.checked_mul(i).ok_or_else(|| 
            MathError::IntegerOverflow { function: "permutations".to_string(), value: n }
        )?;
    Ok(result)
/// Number of combinations C(n, r) = n! / (r! * (n-r)!)
pub fn combinations(n: i64, r: i64) -> crate::error::Result<()> {
    if n < 0 || r < 0 {
        return Err(domain_error("combinations", n as f64, &format!("n={}, r={} (both must be non-negative)", n, r)));
    if r > n {
        return Ok(0);
    // Use symmetry property: C(n, r) = C(n, n-r)
    let r = std::cmp::min(r, n - r);
    
    let mut result = 1i64;
    for i in 0..r {
        result = result.checked_mul(n - i).ok_or_else(|| 
            MathError::IntegerOverflow { function: "combinations".to_string(), value: n }
        )?;
        result /= i + 1;
    Ok(result)
/// Binomial coefficient using Pascal's triangle (alternative implementation)
pub fn binomial_coefficient(n: i64, k: i64) -> crate::error::Result<()> {
    combinations(n, k)
/// Multicombinations - combinations with repetition
pub fn multicombinations(n: i64, r: i64) -> crate::error::Result<()> {
    if n <= 0 || r < 0 {
        return Err(domain_error("multicombinations", n as f64, &format!("n={}, r={} (n must be positive, r non-negative)", n, r)));
    combinations(n + r - 1, r)
/// Catalan number C_n = (1/(n+1)) * C(2n, n)
pub fn catalan_number(n: i64) -> crate::error::Result<()> {
    if n < 0 {
        return Err(negative_input_error("catalan_number", n as f64));
    let c_2n_n = combinations(2 * n, n)?;
    Ok(c_2n_n / (n + 1))
// ============================================================================
// Special Mathematical Functions
// ============================================================================

/// Gamma function Γ(x) using simple recurrence and approximation
pub fn gamma_function(x: f64) -> crate::error::Result<()> {
    validate_float("gamma_function", "x", x)?;
    
    if x <= 0.0 {
        return Err(domain_error("gamma_function", x, "positive values only"));
    // Simple cases using Γ(n) = (n-1)! for integers
    if x == 1.0 { return Ok(1.0); }
    if x == 2.0 { return Ok(1.0); }
    if x == 3.0 { return Ok(2.0); }
    if x == 4.0 { return Ok(6.0); }
    // For Γ(0.5) = √π
    if (x - 0.5).abs() < 1e-10 {
        return Ok(std::f64::consts::PI.sqrt());
    // Use recurrence relation: Γ(x+1) = x*Γ(x)
    if x < 1.0 {
        let gamma_x_plus_1 = gamma_function(x + 1.0)?;
        Ok(gamma_x_plus_1 / x)
    } else if x > 1.0 && x < 2.0 {
        // Γ(x) = (x-1)! for x close to integers, approximate for others
        let int_part = x.floor();
        let frac_part = x - int_part;
        
        if frac_part < 1e-10 {
            // Close to integer, use factorial
            let mut result = 1.0;
            for i in 1..(int_part as i32) {
                result *= i as f64;
            }
            Ok(result)
        } else {
            // Linear interpolation for demonstration
            let gamma_floor = gamma_function(int_part)?;
            let gamma_ceil = gamma_function(int_part + 1.0)?;
            Ok(gamma_floor + frac_part * (gamma_ceil - gamma_floor))
        }
    } else {
        // For x >= 2, use recurrence: Γ(x) = (x-1)*Γ(x-1)
        let gamma_prev = gamma_function(x - 1.0)?;
        Ok((x - 1.0) * gamma_prev)
    }
}

/// Beta function B(x, y) = Γ(x)Γ(y)/Γ(x+y)
pub fn beta_function(x: f64, y: f64) -> crate::error::Result<()> {
    validate_float("beta_function", "x", x)?;
    validate_float("beta_function", "y", y)?;
    
    if x <= 0.0 || y <= 0.0 {
        return Err(domain_error("beta_function", x, "both arguments must be positive"));
    let gamma_x = gamma_function(x)?;
    let gamma_y = gamma_function(y)?;
    let gamma_xy = gamma_function(x + y)?;
    
    let result = (gamma_x * gamma_y) / gamma_xy;
    validate_float("beta_function", "result", result)?;
    Ok(result)
/// CursedError function erf(x) using series expansion
pub fn error_function(x: f64) -> crate::error::Result<()> {
    validate_float("error_function", "x", x)?;
    
    let abs_x = x.abs();
    
    // Use series expansion for small values
    if abs_x < 3.0 {
        let mut sum = 0.0;
        let mut term = abs_x;
        let x_squared = x * x;
        
        for n in 0..100 {  // Series converges rapidly
            sum += term / (2 * n + 1) as f64;
            term *= -x_squared / (n + 1) as f64;
            if term.abs() < 1e-15 { break; }
        }
        
        let result = if x >= 0.0 { 
            2.0 / std::f64::consts::PI.sqrt() * sum 
        } else { 
            -2.0 / std::f64::consts::PI.sqrt() * sum 
        
        validate_float("error_function", "result", result)?;
        Ok(result)
    } else {
        // For large values, erf(x) ≈ ±1
        Ok(if x > 0.0 { 1.0 } else { -1.0 })
    }
}

/// Complementary error function erfc(x) = 1 - erf(x)
pub fn complementary_error_function(x: f64) -> crate::error::Result<()> {
    let erf_x = error_function(x)?;
    Ok(1.0 - erf_x)
// ============================================================================
// Numerical Methods
// ============================================================================

/// Simpson's rule for numerical integration
pub fn simpson_integration<F>(f: F, a: f64, b: f64, n: usize) -> crate::error::Result<()> 
where
{
    validate_float("simpson_integration", "a", a)?;
    validate_float("simpson_integration", "b", b)?;
    
    if n == 0 || n % 2 != 0 {
        return Err(domain_error("simpson_integration", n as f64, "n must be positive and even"));
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    
    for i in 1..n {
        let x = a + i as f64 * h;
        let fx = f(x);
        if !fx.is_finite() {
            return Err(MathError::ComputationError {
            });
        sum += if i % 2 == 0 { 2.0 * fx } else { 4.0 * fx };
    let result = (h / 3.0) * sum;
    validate_float("simpson_integration", "result", result)?;
    Ok(result)
/// Numerical differentiation using central difference
pub fn numerical_derivative<F>(f: F, x: f64, h: f64) -> crate::error::Result<()>
where
{
    validate_float("numerical_derivative", "x", x)?;
    validate_float("numerical_derivative", "h", h)?;
    
    if h == 0.0 {
        return Err(division_by_zero_error("numerical_derivative"));
    let f_plus = f(x + h);
    let f_minus = f(x - h);
    
    if !f_plus.is_finite() || !f_minus.is_finite() {
        return Err(MathError::ComputationError {
        });
    let result = (f_plus - f_minus) / (2.0 * h);
    validate_float("numerical_derivative", "result", result)?;
    Ok(result)
/// Newton-Raphson method for root finding
pub fn newton_raphson<F, DF>(f: F, df: DF, x0: f64, tolerance: f64, max_iterations: usize) -> crate::error::Result<()>
where
{
    validate_float("newton_raphson", "x0", x0)?;
    validate_float("newton_raphson", "tolerance", tolerance)?;
    
    if tolerance <= 0.0 {
        return Err(domain_error("newton_raphson", tolerance, "tolerance must be positive"));
    let mut x = x0;
    
    for _ in 0..max_iterations {
        let fx = f(x);
        let dfx = df(x);
        
        if !fx.is_finite() || !dfx.is_finite() {
            return Err(MathError::ComputationError {
            });
        if dfx.abs() < 1e-15 {
            return Err(division_by_zero_error("newton_raphson"));
        let x_new = x - fx / dfx;
        
        if (x_new - x).abs() < tolerance {
            validate_float("newton_raphson", "result", x_new)?;
            return Ok(x_new);
        x = x_new;
    Err(MathError::ComputationError {
    })
/// Bisection method for root finding
pub fn bisection_method<F>(f: F, a: f64, b: f64, tolerance: f64, max_iterations: usize) -> crate::error::Result<()>
where
{
    validate_float("bisection_method", "a", a)?;
    validate_float("bisection_method", "b", b)?;
    validate_float("bisection_method", "tolerance", tolerance)?;
    
    if tolerance <= 0.0 {
        return Err(domain_error("bisection_method", tolerance, "tolerance must be positive"));
    let fa = f(a);
    let fb = f(b);
    
    if !fa.is_finite() || !fb.is_finite() {
        return Err(MathError::ComputationError {
        });
    if fa * fb > 0.0 {
        return Err(domain_error("bisection_method", a, "function must have different signs at endpoints"));
    let mut left = a;
    let mut right = b;
    
    for _ in 0..max_iterations {
        let mid = (left + right) / 2.0;
        let fmid = f(mid);
        
        if !fmid.is_finite() {
            return Err(MathError::ComputationError {
            });
        if fmid.abs() < tolerance || (right - left) / 2.0 < tolerance {
            validate_float("bisection_method", "result", mid)?;
            return Ok(mid);
        if f(left) * fmid < 0.0 {
            right = mid;
        } else {
            left = mid;
        }
    }
    
    Err(MathError::ComputationError {
    })
// ============================================================================
// Sequence and Series Functions
// ============================================================================

/// Fibonacci number (iterative implementation for efficiency)
pub fn fibonacci(n: i64) -> crate::error::Result<()> {
    if n < 0 {
        return Err(negative_input_error("fibonacci", n as f64));
    if n == 0 { return Ok(0); }
    if n == 1 { return Ok(1); }
    
    if n > 92 {  // F(93) overflows i64
        return Err(MathError::IntegerOverflow { function: "fibonacci".to_string(), value: n });
    let mut a = 0i64;
    let mut b = 1i64;
    
    for _ in 2..=n {
        let temp = a.checked_add(b).ok_or_else(|| 
            MathError::IntegerOverflow { function: "fibonacci".to_string(), value: n }
        )?;
        a = b;
        b = temp;
    Ok(b)
/// Lucas number L_n = L_{n-1} + L_{n-2}, L_0 = 2, L_1 = 1
pub fn lucas_number(n: i64) -> crate::error::Result<()> {
    if n < 0 {
        return Err(negative_input_error("lucas_number", n as f64));
    if n == 0 { return Ok(2); }
    if n == 1 { return Ok(1); }
    
    let mut a = 2i64;
    let mut b = 1i64;
    
    for _ in 2..=n {
        let temp = a.checked_add(b).ok_or_else(|| 
            MathError::IntegerOverflow { function: "lucas_number".to_string(), value: n }
        )?;
        a = b;
        b = temp;
    Ok(b)
/// Tribonacci number T_n = T_{n-1} + T_{n-2} + T_{n-3}
pub fn tribonacci(n: i64) -> crate::error::Result<()> {
    if n < 0 {
        return Err(negative_input_error("tribonacci", n as f64));
    if n == 0 { return Ok(0); }
    if n == 1 || n == 2 { return Ok(1); }
    
    let mut a = 0i64;
    let mut b = 1i64;
    let mut c = 1i64;
    
    for _ in 3..=n {
        let temp = a.checked_add(b).and_then(|sum| sum.checked_add(c)).ok_or_else(|| 
            MathError::IntegerOverflow { function: "tribonacci".to_string(), value: n }
        )?;
        a = b;
        b = c;
        c = temp;
    Ok(c)
/// Factorial sequence sum: sum of k! for k from 0 to n
pub fn factorial_sequence_sum(n: i64) -> crate::error::Result<()> {
    if n < 0 {
        return Err(negative_input_error("factorial_sequence_sum", n as f64));
    let mut sum = 0i64;
    let mut fact = 1i64;
    
    for k in 0..=n {
        if k > 0 {
            fact = fact.checked_mul(k).ok_or_else(|| 
                MathError::IntegerOverflow { function: "factorial_sequence_sum".to_string(), value: n }
            )?;
        }
        sum = sum.checked_add(fact).ok_or_else(|| 
            MathError::IntegerOverflow { function: "factorial_sequence_sum".to_string(), value: n }
        )?;
    Ok(sum)
/// Harmonic number H_n = 1 + 1/2 + 1/3 + ... + 1/n
pub fn harmonic_number(n: i64) -> crate::error::Result<()> {
    if n < 1 {
        return Err(domain_error("harmonic_number", n as f64, "n must be positive"));
    let mut sum = 0.0;
    for i in 1..=n {
        sum += 1.0 / i as f64;
    validate_float("harmonic_number", "result", sum)?;
    Ok(sum)
// ============================================================================
// Modular Arithmetic and Base Conversions
// ============================================================================

/// Modular exponentiation: (base^exp) mod modulus
pub fn mod_pow(base: i64, exp: i64, modulus: i64) -> crate::error::Result<()> {
    if modulus <= 0 {
        return Err(domain_error("mod_pow", modulus as f64, "modulus must be positive"));
    }
    if exp < 0 {
        return Err(negative_input_error("mod_pow", exp as f64));
    let mut result = 1i64;
    let mut base = base % modulus;
    let mut exp = exp;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    Ok(result)
/// Modular multiplicative inverse using extended Euclidean algorithm
pub fn mod_inverse(a: i64, m: i64) -> crate::error::Result<()> {
    if m <= 0 {
        return Err(domain_error("mod_inverse", m as f64, "modulus must be positive"));
    let (gcd, x, _) = extended_gcd(a, m);
    
    if gcd != 1 {
        return Err(MathError::ComputationError {
        });
    Ok(((x % m) + m) % m)
/// Convert number from one base to another
pub fn convert_base(number: &str, from_base: u32, to_base: u32) -> crate::error::Result<()> {
    if from_base < 2 || from_base > 36 || to_base < 2 || to_base > 36 {
        return Err(domain_error("convert_base", from_base as f64, "base must be between 2 and 36"));
    // Convert from source base to decimal
    let decimal = i64::from_str_radix(number, from_base)
        .map_err(|_| MathError::ComputationError {
        })?;
    
    // Convert from decimal to target base
    if decimal == 0 {
        return Ok("0".to_string());
    let mut result = String::new();
    let mut num = decimal.abs();
    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    while num > 0 {
        let remainder = (num % to_base as i64) as usize;
        result.insert(0, digits.chars().nth(remainder).unwrap());
        num /= to_base as i64;
    if decimal < 0 {
        result.insert(0, '-');
    Ok(result)
/// Greatest common divisor of multiple numbers
pub fn gcd_multiple(numbers: &[i64]) -> crate::error::Result<()> {
    if numbers.is_empty() {
        return Err(MathError::ComputationError {
        });
    let mut result = numbers[0].abs();
    for &num in &numbers[1..] {
        result = gcd_two(result, num.abs());
        if result == 1 { break; }  // Early termination optimization
    Ok(result)
/// Least common multiple of multiple numbers
pub fn lcm_multiple(numbers: &[i64]) -> crate::error::Result<()> {
    if numbers.is_empty() {
        return Err(MathError::ComputationError {
        });
    let mut result = numbers[0].abs();
    for &num in &numbers[1..] {
        let num_abs = num.abs();
        let gcd = gcd_two(result, num_abs);
        result = result.checked_mul(num_abs / gcd).ok_or_else(|| 
            MathError::IntegerOverflow { function: "lcm_multiple".to_string(), value: result }
        )?;
    Ok(result)
// Helper function for GCD of two numbers
fn gcd_two(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
// ============================================================================
// Advanced Utilities
// ============================================================================

/// Memoized Fibonacci for better performance with repeated calls
pub struct FibonacciMemo {
impl FibonacciMemo {
    pub fn new() -> Self {
        let mut cache = HashMap::new();
        cache.insert(0, 0);
        cache.insert(1, 1);
        FibonacciMemo { cache }
    }
    
    pub fn fibonacci(&mut self, n: i64) -> crate::error::Result<()> {
        if n < 0 {
            return Err(negative_input_error("fibonacci_memo", n as f64));
        if let Some(&result) = self.cache.get(&n) {
            return Ok(result);
        if n > 92 {
            return Err(MathError::IntegerOverflow { function: "fibonacci_memo".to_string(), value: n });
        let fib_n_1 = self.fibonacci(n - 1)?;
        let fib_n_2 = self.fibonacci(n - 2)?;
        let result = fib_n_1.checked_add(fib_n_2).ok_or_else(|| 
            MathError::IntegerOverflow { function: "fibonacci_memo".to_string(), value: n }
        )?;
        
        self.cache.insert(n, result);
        Ok(result)
    }
}

/// Perfect number checker
pub fn is_perfect_number(n: i64) -> crate::error::Result<()> {
    if n <= 0 {
        return Err(negative_input_error("is_perfect_number", n as f64));
    let mut sum = 1i64;  // 1 is always a divisor
    let sqrt_n = (n as f64).sqrt() as i64;
    
    for i in 2..=sqrt_n {
        if n % i == 0 {
            sum = sum.checked_add(i).ok_or_else(|| 
                MathError::IntegerOverflow { function: "is_perfect_number".to_string(), value: n }
            )?;
            
            if i != n / i {  // Avoid counting the square root twice
                sum = sum.checked_add(n / i).ok_or_else(|| 
                    MathError::IntegerOverflow { function: "is_perfect_number".to_string(), value: n }
                )?;
            }
        }
    Ok(sum == n)
/// Digital root (repeated sum of digits until single digit)
pub fn digital_root(mut n: i64) -> crate::error::Result<()> {
    if n < 0 {
        return Err(negative_input_error("digital_root", n as f64));
    // Digital root formula: 1 + ((n - 1) % 9)
    // But we'll use the iterative approach for clarity
    while n >= 10 {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        n = sum;
    Ok(n)
}
