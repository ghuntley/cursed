# SketchyMath Module

A comprehensive pure CURSED mathematics library providing approximate mathematical functions and probabilistic operations. This module implements a subset of advanced mathematical operations without external dependencies.

## Features

### Constants
- Mathematical constants (PI, E, PHI, SQRT2, LN2, LN10)
- Floating-point limits (MAX_FLOAT64, MIN_FLOAT64)

### Basic Mathematical Functions
- **Absolute Value**: `abs(x)`, `abs_int(x)`
- **Square/Cube Root**: `sqrt(x)`, `cbrt(x)`
- **Exponentiation**: `pow(x, y)`, `exp(x)`
- **Logarithms**: `ln(x)`, `log10(x)`, `log2(x)`

### Rounding Functions
- `ceil(x)` - Round up to nearest integer
- `floor(x)` - Round down to nearest integer
- `round(x)` - Round to nearest integer
- `trunc(x)` - Truncate to integer

### Trigonometric Functions
- `sin(x)`, `cos(x)`, `tan(x)` - Basic trigonometric functions
- Uses Taylor series approximations for accurate results

### Classification Functions
- `is_nan(x)` - Check if value is NaN
- `is_inf(x)` - Check if value is infinite
- `is_finite(x)` - Check if value is finite
- `sign(x)` - Get sign of number

### Fuzzy Mathematics
- `almost_equal(a, b, epsilon)` - Compare with tolerance
- `almost_zero(x, epsilon)` - Check if nearly zero
- `fuzzy_equals(a, b)` - Compare with default tolerance

### Random Number Generation
- `set_random_seed(seed)` - Set random seed
- `random_float64()` - Random float [0,1)
- `random_float64_range(min, max)` - Random float in range
- `random_int_range(min, max)` - Random integer in range
- `random_normal(mean, stddev)` - Normal distribution
- `random_bernoulli(p)` - Bernoulli trial

### Statistical Functions
- `norm_pdf(x)` - Normal probability density function
- `norm_cdf(x)` - Normal cumulative distribution function

### Utility Functions
- `min(a, b)`, `max(a, b)` - Minimum/maximum values
- `clamp(x, min, max)` - Clamp value to range

### Gen Z Math Features
- `vibecheck(x)` - Score based on numerical properties
- `super_bussin(x)` - Check if number is "excellent"
- `no_cap(x)` - Check if number is legitimate
- `yeet_clamp(x, min, max)` - Optimized clamping
- `sussy_calc(result, min, max)` - Detect suspicious results

### Fast Approximations
- `fast_sqrt(x)` - Fast square root approximation
- `fast_inv_sqrt(x)` - Fast inverse square root
- `fast_sin(x)`, `fast_cos(x)` - Fast trigonometric functions
- `fast_exp(x)`, `fast_log(x)` - Fast exponential/logarithm

### Advanced Functions
- `factorial(n)` - Factorial function
- `combination(n, k)` - Binomial coefficient
- `permutation(n, k)` - Permutation count
- `gamma(x)` - Gamma function (Stirling's approximation)
- `erf(x)`, `erfc(x)` - Error functions
- `integrate_simple(a, b, n)` - Numerical integration
- `derivative_simple(x, h)` - Numerical derivative
- `bisection_root(a, b, tolerance)` - Root finding

## Usage Examples

```cursed
yeet "sketchy_math"

// Basic operations
sus x meal = 16.0
sus y meal = -4.0

vibez.spill(sketchy_math.sqrt(x))     // 4.0
vibez.spill(sketchy_math.abs(y))      // 4.0
vibez.spill(sketchy_math.pow(x, 0.5)) // 4.0

// Trigonometric functions
sus angle meal = sketchy_math.PI / 4.0
vibez.spill(sketchy_math.sin(angle))  // ~0.707
vibez.spill(sketchy_math.cos(angle))  // ~0.707

// Random numbers
sketchy_math.set_random_seed(42)
sus random meal = sketchy_math.random_float64()
sus normal meal = sketchy_math.random_normal(0.0, 1.0)

// Gen Z features
vibez.spill(sketchy_math.super_bussin(420.0))    // true
vibez.spill(sketchy_math.no_cap(1e308))          // false
vibez.spill(sketchy_math.yeet_clamp(100.0, 0.0, 10.0)) // 10.0
```

## Testing

Run comprehensive tests:
```bash
cargo run --bin cursed stdlib/sketchy_math/test_sketchy_math.csd
```

Test native compilation:
```bash
cargo run --bin cursed -- compile stdlib/sketchy_math/test_sketchy_math.csd
./test_sketchy_math
```

## Implementation Notes

- All functions are implemented in pure CURSED without FFI dependencies
- Uses Taylor series and numerical approximations for accuracy
- Includes proper error handling for edge cases (NaN, infinity, etc.)
- Implements both fast approximations and accurate algorithms
- Random number generation uses linear congruential generator
- Statistical functions use standard mathematical approximations
- Gen Z features add modern naming conventions and utility functions

## Accuracy

- Basic math functions: High precision using established algorithms
- Trigonometric: Taylor series with sufficient terms for accuracy
- Random generation: Statistical quality suitable for most applications
- Fast approximations: Trade accuracy for speed where appropriate
- Numerical methods: Configurable precision and iteration limits

This module provides a solid foundation for mathematical computations in CURSED programs while maintaining the language's unique styling and approach.
