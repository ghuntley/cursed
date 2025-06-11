# CURSED Mathematics Library - Comprehensive Guide

## Overview

The CURSED Mathematics Library provides a complete, unified mathematical framework for scientific computing, statistical analysis, and numerical methods. The library is designed for high performance, ease of use, and comprehensive functionality across all areas of mathematical computation.

## Architecture

The mathematics library is organized into eight core modules, each specializing in specific mathematical domains:

### Core Modules

1. **`basic`** - Fundamental arithmetic and utility functions
2. **`trigonometry`** - Complete trigonometric operations  
3. **`logarithmic`** - Logarithmic, exponential, and power functions
4. **`constants`** - Mathematical constants and fundamental values
5. **`random`** - Random number generation and statistical distributions
6. **`statistics`** - Statistical analysis and data processing
7. **`special`** - Advanced mathematical functions (gamma, bessel, etc.)
8. **`utilities`** - Computational mathematics and numerical methods

### Design Principles

- **Unified API**: All functions accessible through `import "stdlib::math"`
- **Naming Consistency**: Clear function naming without conflicts
- **Error Handling**: Comprehensive error types with meaningful messages
- **Performance**: Optimized implementations using standard mathematical libraries
- **Safety**: Domain validation and overflow protection
- **Interoperability**: Seamless integration between all modules

## Function Categories

### Basic Operations (`basic` module)

#### Arithmetic Functions
```cursed
abs(x: f64) -> f64                    // Absolute value
min(a: f64, b: f64) -> f64           // Minimum of two values
max(a: f64, b: f64) -> f64           // Maximum of two values
clamp(x: f64, min: f64, max: f64) -> f64  // Clamp value to range
sign(x: f64) -> f64                  // Sign function (-1, 0, or 1)
```

#### Rounding and Precision
```cursed
floor(x: f64) -> f64                 // Round down to integer
ceil(x: f64) -> f64                  // Round up to integer
round(x: f64) -> f64                 // Round to nearest integer
math_truncate(x: f64) -> f64         // Truncate fractional part
fract(x: f64) -> f64                 // Fractional part only
round_to_decimals(x: f64, decimals: i32) -> f64  // Round to specific decimal places
```

#### Integer Arithmetic
```cursed
remainder(x: f64, y: f64) -> MathResult<f64>     // Floating point remainder
modulo(x: f64, y: f64) -> MathResult<f64>        // Modulo operation
gcd(a: i64, b: i64) -> i64                       // Greatest common divisor
lcm(a: i64, b: i64) -> i64                       // Least common multiple
is_even(n: i64) -> bool                          // Check if number is even
is_odd(n: i64) -> bool                           // Check if number is odd
```

#### Interpolation and Smoothing
```cursed
lerp(a: f64, b: f64, t: f64) -> f64              // Linear interpolation
inverse_lerp(a: f64, b: f64, value: f64) -> f64  // Inverse linear interpolation
smooth_step(t: f64) -> f64                       // Smooth step function
smoother_step(t: f64) -> f64                     // Smoother step function
map_range(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64
```

### Trigonometric Functions (`trigonometry` module)

#### Primary Functions
```cursed
sin(x: f64) -> f64                   // Sine
cos(x: f64) -> f64                   // Cosine  
tan(x: f64) -> f64                   // Tangent
asin(x: f64) -> f64                  // Arcsine
acos(x: f64) -> f64                  // Arccosine
atan(x: f64) -> f64                  // Arctangent
atan2(y: f64, x: f64) -> f64         // Two-argument arctangent
```

#### Hyperbolic Functions
```cursed
sinh(x: f64) -> f64                  // Hyperbolic sine
cosh(x: f64) -> f64                  // Hyperbolic cosine
tanh(x: f64) -> f64                  // Hyperbolic tangent
asinh(x: f64) -> f64                 // Inverse hyperbolic sine
acosh(x: f64) -> f64                 // Inverse hyperbolic cosine
atanh(x: f64) -> f64                 // Inverse hyperbolic tangent
```

#### Angle Utilities
```cursed
degrees_to_radians(degrees: f64) -> f64         // Convert degrees to radians
radians_to_degrees(radians: f64) -> f64         // Convert radians to degrees
deg_to_rad(degrees: f64) -> f64                 // Alias for degrees_to_radians
rad_to_deg(radians: f64) -> f64                 // Alias for radians_to_degrees

sin_deg(degrees: f64) -> f64                    // Sine in degrees
cos_deg(degrees: f64) -> f64                    // Cosine in degrees
tan_deg(degrees: f64) -> f64                    // Tangent in degrees

normalize_angle(angle: f64) -> f64              // Normalize to [0, 2π)
normalize_angle_signed(angle: f64) -> f64       // Normalize to (-π, π]
```

#### Reciprocal Functions
```cursed
sec(x: f64) -> f64                   // Secant (1/cos)
csc(x: f64) -> f64                   // Cosecant (1/sin)
cot(x: f64) -> f64                   // Cotangent (1/tan)
```

### Logarithmic and Exponential Functions (`logarithmic` module)

#### Core Logarithmic Functions
```cursed
ln(x: f64) -> MathResult<f64>                   // Natural logarithm
log10(x: f64) -> MathResult<f64>                // Common logarithm (base 10)
log2(x: f64) -> MathResult<f64>                 // Binary logarithm (base 2)
log(x: f64, base: f64) -> MathResult<f64>       // Logarithm with arbitrary base
expm1(x: f64) -> f64                            // exp(x) - 1, precise for small x
ln1p(x: f64) -> MathResult<f64>                 // ln(1 + x), precise for small x
```

#### Exponential Functions
```cursed
exp(x: f64) -> MathResult<f64>                  // Exponential function e^x
exp2(x: f64) -> MathResult<f64>                 // Base-2 exponential 2^x
exp10(x: f64) -> MathResult<f64>                // Base-10 exponential 10^x
exp2m1(x: f64) -> f64                           // 2^x - 1, precise for small x
exp10m1(x: f64) -> f64                          // 10^x - 1, precise for small x
exp_base(base: f64, x: f64) -> MathResult<f64>  // base^x
```

#### Power Functions
```cursed
pow(x: f64, y: f64) -> MathResult<f64>          // x^y
powi(x: f64, n: i32) -> f64                     // x^n (integer exponent)
pow_e(x: f64) -> MathResult<f64>                // e^x (alias for exp)
pow_2(x: f64) -> MathResult<f64>                // 2^x (alias for exp2)
pow_10(x: f64) -> MathResult<f64>               // 10^x (alias for exp10)
tetration(base: f64, height: usize) -> MathResult<f64>  // Power tower
```

#### Root Functions
```cursed
sqrt(x: f64) -> MathResult<f64>                 // Square root
cbrt(x: f64) -> f64                             // Cube root
nth_root(x: f64, n: f64) -> MathResult<f64>     // nth root
hypot(x: f64, y: f64) -> f64                    // √(x² + y²)
hypot3(x: f64, y: f64, z: f64) -> f64          // √(x² + y² + z²)
inv_sqrt(x: f64) -> MathResult<f64>             // 1/√x
```

#### Advanced Functions
```cursed
sigmoid(x: f64) -> MathResult<f64>              // Sigmoid function 1/(1+e^(-x))
logistic(x: f64, k: f64, x0: f64) -> MathResult<f64>  // Generalized logistic
softmax_single(x: f64, values: &[f64]) -> MathResult<f64>  // Softmax normalization
log_sum_exp(values: &[f64]) -> MathResult<f64>  // Numerically stable log-sum-exp
```

#### Safety and Validation
```cursed
is_valid_log_input(x: f64) -> bool              // Check if valid for logarithm
is_valid_exp_input(x: f64) -> bool              // Check if valid for exponential
safe_ln(x: f64) -> Option<f64>                  // Safe logarithm returning Option
safe_exp(x: f64) -> Option<f64>                 // Safe exponential returning Option
clamped_ln(x: f64, min_result: f64) -> MathResult<f64>    // Clamped logarithm
clamped_exp(x: f64, max_result: f64) -> MathResult<f64>   // Clamped exponential
```

### Mathematical Constants (`constants` module)

#### Primary Constants
```cursed
const PI: f64 = 3.141592653589793           // π
const TAU: f64 = 6.283185307179586          // τ = 2π
const E: f64 = 2.718281828459045            // Euler's number
const PHI: f64 = 1.618033988749895          // Golden ratio φ
const INV_PHI: f64 = 0.6180339887498948     // 1/φ
const EULER_GAMMA: f64 = 0.5772156649015329 // Euler-Mascheroni constant γ
```

#### Pi-Related Constants
```cursed
const FRAC_PI_2: f64 = 1.5707963267948966   // π/2
const FRAC_PI_3: f64 = 1.0471975511965979   // π/3
const FRAC_PI_4: f64 = 0.7853981633974483   // π/4
const FRAC_PI_6: f64 = 0.5235987755982988   // π/6
const FRAC_PI_8: f64 = 0.39269908169872414  // π/8
const FRAC_1_PI: f64 = 0.3183098861837907   // 1/π
const FRAC_2_PI: f64 = 0.6366197723675814   // 2/π
const FRAC_2_SQRT_PI: f64 = 1.1283791670955126  // 2/√π
```

#### Square Root Constants
```cursed
const SQRT_2: f64 = 1.4142135623730951      // √2
const FRAC_1_SQRT_2: f64 = 0.7071067811865476  // 1/√2
const SQRT_3: f64 = 1.7320508075688772      // √3
const SQRT_5: f64 = 2.23606797749979        // √5
const SQRT_PI: f64 = 1.7724538509055159     // √π
```

#### Logarithmic Constants
```cursed
const LN_2: f64 = 0.6931471805599453        // ln(2)
const LN_10: f64 = 2.302585092994046        // ln(10)
const LOG2_E: f64 = 1.4426950408889634      // log₂(e)
const LOG2_10: f64 = 3.3219280948873626     // log₂(10)
const LOG10_E: f64 = 0.4342944819032518     // log₁₀(e)
const LOG10_2: f64 = 0.3010299956639812     // log₁₀(2)
```

#### Conversion Factors
```cursed
const DEG_TO_RAD: f64 = 0.017453292519943295  // π/180
const RAD_TO_DEG: f64 = 57.29577951308232      // 180/π
```

#### Floating Point Limits
```cursed
const EPSILON: f64 = 2.220446049250313e-16   // Machine epsilon
const MIN_POSITIVE: f64 = 2.2250738585072014e-308  // Smallest positive normal
const MAX: f64 = 1.7976931348623157e+308     // Maximum finite value
const MIN: f64 = -1.7976931348623157e+308    // Minimum finite value
const INFINITY: f64 = f64::INFINITY          // Positive infinity
const NEG_INFINITY: f64 = f64::NEG_INFINITY  // Negative infinity
const NAN: f64 = f64::NAN                    // Not a number
```

### Random Number Generation (`random` module)

#### Basic Random Functions
```cursed
random() -> f64                              // Random float [0.0, 1.0)
random_range(min: f64, max: f64) -> f64      // Random float in range
random_int(min: i32, max: i32) -> i32        // Random integer in range
random_u64() -> u64                          // Random 64-bit unsigned integer
random_bool() -> bool                        // Random boolean
set_seed(seed: u64)                          // Set random seed
```

#### Collection Utilities
```cursed
choice<T>(items: &[T]) -> &T                 // Choose random item
choices<T>(items: &[T], k: usize) -> Vec<&T> // Choose k random items
weighted_choice<T>(items: &[(T, f64)]) -> &T // Weighted random choice
shuffle<T>(items: &mut [T])                  // Shuffle array in-place
shuffled<T>(items: &[T]) -> Vec<T>           // Return shuffled copy
sample<T>(items: &[T], k: usize) -> Vec<&T>  // Random sample without replacement
```

#### String and Byte Generation
```cursed
random_bytes(length: usize) -> Vec<u8>       // Random byte array
random_string(length: usize) -> String       // Random ASCII string
random_alphanumeric(length: usize) -> String // Random alphanumeric string
random_hex(length: usize) -> String          // Random hex string
```

#### Statistical Distributions
```cursed
random_normal(mean: f64, std_dev: f64) -> f64      // Normal distribution
random_exponential(lambda: f64) -> f64             // Exponential distribution
random_uniform(min: f64, max: f64) -> f64          // Uniform distribution
random_poisson(lambda: f64) -> u32                 // Poisson distribution
random_beta(alpha: f64, beta: f64) -> f64          // Beta distribution
random_gamma(shape: f64, scale: f64) -> f64        // Gamma distribution
```

### Statistical Functions (`statistics` module)

#### Descriptive Statistics
```cursed
mean(data: &[f64]) -> f64                    // Arithmetic mean
median(data: &[f64]) -> f64                  // Median value
mode(data: &[f64]) -> Vec<f64>               // Mode(s)
variance(data: &[f64]) -> f64                // Population variance
standard_deviation(data: &[f64]) -> f64      // Standard deviation
skewness(data: &[f64]) -> f64                // Skewness
kurtosis(data: &[f64]) -> f64                // Kurtosis
```

#### Quantiles and Percentiles
```cursed
quantile(data: &[f64], q: f64) -> f64        // Quantile (0.0 to 1.0)
percentile(data: &[f64], p: f64) -> f64      // Percentile (0.0 to 100.0)
quartiles(data: &[f64]) -> (f64, f64, f64)   // Q1, Q2, Q3
iqr(data: &[f64]) -> f64                     // Interquartile range
```

#### Range and Spread
```cursed
range(data: &[f64]) -> f64                   // Range (max - min)
min_max(data: &[f64]) -> (f64, f64)          // Minimum and maximum
coefficient_of_variation(data: &[f64]) -> f64 // CV = std_dev / mean
```

#### Correlation and Regression
```cursed
correlation(x: &[f64], y: &[f64]) -> f64     // Pearson correlation coefficient
covariance(x: &[f64], y: &[f64]) -> f64      // Covariance
linear_regression(x: &[f64], y: &[f64]) -> (f64, f64)  // (slope, intercept)
r_squared(x: &[f64], y: &[f64]) -> f64       // Coefficient of determination
```

#### Statistical Tests
```cursed
z_score(value: f64, mean: f64, std_dev: f64) -> f64    // Z-score
t_test_one_sample(data: &[f64], expected_mean: f64) -> f64  // One-sample t-test
t_test_two_sample(data1: &[f64], data2: &[f64]) -> f64     // Two-sample t-test
chi_squared_test(observed: &[f64], expected: &[f64]) -> f64 // Chi-squared test
```

#### Data Processing
```cursed
normalize_data(data: &[f64]) -> Vec<f64>     // Normalize to [0, 1]
standardize_data(data: &[f64]) -> Vec<f64>   // Standardize (z-scores)
outlier_detection(data: &[f64], threshold: f64) -> Vec<usize>  // Outlier indices
```

#### Moving Statistics
```cursed
moving_average(data: &[f64], window: usize) -> Vec<f64>       // Moving average
exponential_smoothing(data: &[f64], alpha: f64) -> Vec<f64>   // Exponential smoothing
```

### Special Functions (`special` module)

#### Factorial and Gamma
```cursed
special_factorial(n: usize) -> usize         // Factorial n!
factorial_f64(x: f64) -> f64                 // Factorial for floating point
gamma(x: f64) -> f64                         // Gamma function Γ(x)
ln_gamma(x: f64) -> f64                      // Natural log of gamma function
beta(a: f64, b: f64) -> f64                  // Beta function B(a,b)
```

#### Combinatorial Functions
```cursed
binomial(n: usize, k: usize) -> usize        // Binomial coefficient C(n,k)
binomial_f64(n: f64, k: f64) -> f64          // Binomial coefficient (float)
special_permutations(n: usize, k: usize) -> usize  // Permutations P(n,k)
```

#### Number Sequences
```cursed
special_fibonacci(n: usize) -> usize         // Fibonacci number F(n)
lucas(n: usize) -> usize                     // Lucas number L(n)
catalan(n: usize) -> usize                   // Catalan number C(n)
```

#### Error Functions
```cursed
erf(x: f64) -> f64                           // Error function
erfc(x: f64) -> f64                          // Complementary error function
erf_inv(x: f64) -> f64                       // Inverse error function
```

#### Bessel Functions
```cursed
bessel_j0(x: f64) -> f64                     // Bessel function of first kind, order 0
bessel_j1(x: f64) -> f64                     // Bessel function of first kind, order 1
bessel_y0(x: f64) -> f64                     // Bessel function of second kind, order 0
bessel_y1(x: f64) -> f64                     // Bessel function of second kind, order 1
```

### Mathematical Utilities (`utilities` module)

#### Number Theory
```cursed
extended_gcd(a: i64, b: i64) -> (i64, i64, i64)     // Extended Euclidean algorithm
is_prime(n: i64) -> bool                             // Primality test
sieve_of_eratosthenes(limit: usize) -> Vec<usize>    // Generate primes up to limit
prime_factorization(n: i64) -> Vec<i64>              // Prime factorization
next_prime(n: i64) -> i64                            // Next prime after n
euler_totient(n: i64) -> i64                         // Euler's totient function φ(n)
```

#### Combinatorics (Primary Implementations)
```cursed
factorial(n: usize) -> usize                         // Factorial n!
double_factorial(n: usize) -> usize                  // Double factorial n!!
factorial_stirling(n: usize) -> f64                  // Stirling's approximation
permutations(n: usize, k: usize) -> usize            // P(n,k)
combinations(n: usize, k: usize) -> usize            // C(n,k)
binomial_coefficient(n: usize, k: usize) -> usize    // Binomial coefficient
multicombinations(n: usize, k: usize) -> usize       // Multicombinations
catalan_number(n: usize) -> usize                    // Catalan number
```

#### Advanced Special Functions
```cursed
gamma_function(x: f64) -> f64                        // Gamma function
beta_function(a: f64, b: f64) -> f64                 // Beta function
error_function(x: f64) -> f64                        // Error function
complementary_error_function(x: f64) -> f64          // Complementary error function
```

#### Numerical Analysis Methods
```cursed
simpson_integration(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64
numerical_derivative(f: fn(f64) -> f64, x: f64, h: f64) -> f64
newton_raphson(f: fn(f64) -> f64, df: fn(f64) -> f64, x0: f64, tol: f64, max_iter: usize) -> f64
bisection_method(f: fn(f64) -> f64, a: f64, b: f64, tol: f64, max_iter: usize) -> f64
```

#### Sequences and Series
```cursed
fibonacci(n: usize) -> usize                         // Fibonacci sequence
lucas_number(n: usize) -> usize                      // Lucas sequence
tribonacci(n: usize) -> usize                        // Tribonacci sequence
factorial_sequence_sum(n: usize) -> usize            // Sum of factorials 1! + 2! + ... + n!
harmonic_number(n: usize) -> f64                     // nth harmonic number
```

#### Modular Arithmetic
```cursed
mod_pow(base: i64, exp: i64, modulus: i64) -> i64    // Modular exponentiation
mod_inverse(a: i64, m: i64) -> Option<i64>           // Modular multiplicative inverse
convert_base(number: i64, from_base: u32, to_base: u32) -> String  // Base conversion
gcd_multiple(numbers: &[i64]) -> i64                 // GCD of multiple numbers
lcm_multiple(numbers: &[i64]) -> i64                 // LCM of multiple numbers
```

#### Advanced Mathematical Utilities
```cursed
FibonacciMemo                                        // Memoized Fibonacci calculator
is_perfect_number(n: i64) -> bool                    // Perfect number test
digital_root(n: i64) -> i64                          // Digital root calculation
```

## Error Handling

### Error Types

The mathematics library uses a comprehensive error system:

```cursed
enum MathError {
    DomainError { function: String, value: f64, message: String },
    RangeError { function: String, message: String },
    Overflow { function: String, value: f64 },
    Underflow { function: String, value: f64 },
    DivisionByZero { function: String },
    InvalidInput { function: String, parameter: String, value: f64 },
    NegativeInput { function: String, value: f64 },
    IntegerOverflow { function: String, value: i64 },
    ComputationError { function: String, message: String },
}
```

### Helper Functions

```cursed
domain_error(function: &str, value: f64, message: &str) -> MathError
range_error(function: &str, message: &str) -> MathError
division_by_zero_error(function: &str) -> MathError
negative_input_error(function: &str, value: f64) -> MathError
is_valid_float(value: f64) -> bool
validate_float(function: &str, parameter: &str, value: f64) -> MathResult<()>
```

### Usage Examples

```cursed
// Functions that can fail return MathResult<T>
match ln(-1.0) {
    Ok(result) => println("ln(-1) = {}", result),
    Err(MathError::DomainError { function, value, message }) => {
        println("Error in {}: {} for value {}", function, message, value);
    }
}

// Safe functions return Option<T>
if let Some(result) = safe_ln(5.0) {
    println("ln(5) = {}", result);
} else {
    println("Invalid input for logarithm");
}

// Clamped functions provide bounded results
let result = clamped_exp(100.0, 1000.0)?; // Clamps result to max 1000.0
```

## Performance Considerations

### Optimization Features

1. **Efficient Algorithms**: Implementations use optimized mathematical algorithms
2. **Branch Prediction**: Optimized for common use cases
3. **Minimal Allocations**: Stack-based operations where possible
4. **SIMD Support**: Where available through standard library
5. **Lookup Tables**: For frequently computed values

### Performance Guidelines

1. **Prefer Specific Functions**: Use `powi` for integer exponents instead of `pow`
2. **Use Constants**: Pre-computed constants are faster than calculations
3. **Batch Operations**: Process arrays when possible for better cache performance
4. **Choose Appropriate Precision**: Use `f32` when `f64` precision isn't needed

### Benchmarking

The library includes comprehensive benchmarks:

```cursed
// Example performance test
use std::time::Instant;

let start = Instant::now();
for i in 0..10000 {
    let _ = sin(i as f64);
}
let duration = start.elapsed();
println("10,000 sin calculations: {:?}", duration);
```

## Integration Examples

### Scientific Computing
```cursed
import "stdlib::math";

// Signal processing
slay analyze_signal(samples: &[f64], sample_rate: f64) {
    let fft_size = samples.len();
    let frequency_resolution = sample_rate / fft_size as f64;
    
    // Apply window function
    let windowed: Vec<f64> = samples.iter().enumerate()
        .map(|(i, &sample)| {
            let window = 0.5 - 0.5 * cos(2.0 * PI * i as f64 / (fft_size - 1) as f64);
            sample * window
        })
        .collect();
    
    // Calculate power spectrum
    let mean_power = mean(&windowed);
    let rms = sqrt(mean(&windowed.iter().map(|&x| x * x).collect()));
    
    (mean_power, rms, frequency_resolution)
}
```

### Statistical Analysis
```cursed
// Quality control analysis
slay quality_control(measurements: &[f64], target: f64, tolerance: f64) {
    let sample_mean = mean(measurements);
    let sample_std = standard_deviation(measurements);
    
    // Process capability
    let cp = tolerance / (3.0 * sample_std);
    let cpk = min((target - sample_mean).abs(), tolerance) / (3.0 * sample_std);
    
    // Control limits
    let ucl = sample_mean + 3.0 * sample_std;
    let lcl = sample_mean - 3.0 * sample_std;
    
    // Outlier detection
    let outliers = outlier_detection(measurements, 3.0);
    
    (cp, cpk, ucl, lcl, outliers)
}
```

### Numerical Methods
```cursed
// Solve differential equation using Euler's method
slay solve_ode(f: fn(f64, f64) -> f64, y0: f64, x0: f64, xf: f64, h: f64) -> Vec<(f64, f64)> {
    let mut result = Vec::new();
    let mut x = x0;
    let mut y = y0;
    
    while x <= xf {
        result.push((x, y));
        y = y + h * f(x, y);
        x = x + h;
    }
    
    result
}
```

### Monte Carlo Simulation
```cursed
// Monte Carlo estimation of π
slay estimate_pi(num_samples: usize) -> f64 {
    set_seed(42);
    let mut count_inside = 0;
    
    for _ in 0..num_samples {
        let x = random();
        let y = random();
        if x*x + y*y <= 1.0 {
            count_inside += 1;
        }
    }
    
    4.0 * count_inside as f64 / num_samples as f64
}
```

## Testing and Validation

### Test Categories

1. **Unit Tests**: Individual function validation
2. **Integration Tests**: Cross-module functionality  
3. **Performance Tests**: Benchmark validation
4. **Accuracy Tests**: Mathematical precision verification
5. **Edge Case Tests**: Boundary condition handling

### Running Tests

```bash
# Run all math tests
cargo test math_

# Run specific test suites
cargo test math_basic_test
cargo test math_integration_test
cargo test math_performance_test

# Run with optimizations
cargo test --release math_
```

### Mathematical Validation

The library validates mathematical identities:

```cursed
// Trigonometric identity: sin²(x) + cos²(x) = 1
assert!((sin(x).powi(2) + cos(x).powi(2) - 1.0).abs() < EPSILON);

// Logarithmic identity: ln(ab) = ln(a) + ln(b)  
assert!((ln(a * b) - (ln(a) + ln(b))).abs() < EPSILON);

// Exponential identity: e^(ln(x)) = x
assert!((exp(ln(x)) - x).abs() < EPSILON);
```

## Future Enhancements

### Planned Features

1. **Complex Numbers**: Full complex number arithmetic
2. **Vector Operations**: SIMD-optimized vector math
3. **Matrix Operations**: Linear algebra support
4. **Quaternions**: 3D rotation mathematics
5. **Arbitrary Precision**: BigNum integration
6. **GPU Acceleration**: CUDA/OpenCL support for parallel operations

### Contributing

The mathematics library welcomes contributions in:

1. **New Functions**: Additional mathematical operations
2. **Optimizations**: Performance improvements
3. **Documentation**: Examples and guides
4. **Testing**: Additional test coverage
5. **Algorithms**: Alternative implementations

## Conclusion

The CURSED Mathematics Library provides a comprehensive, high-performance mathematical framework suitable for scientific computing, statistical analysis, and numerical methods. With its unified API, extensive functionality, and robust error handling, it serves as a solid foundation for mathematical computation in CURSED applications.

For detailed examples and usage patterns, see the comprehensive demo program and integration tests included with the library.
