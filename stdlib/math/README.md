# Math Module

Comprehensive mathematical functions and calculations for CURSED.

## Overview

The `math` module provides a complete set of mathematical functions including basic arithmetic, trigonometry, statistics, and specialized mathematical operations. All functions are implemented with high precision and performance.

## Features

### Mathematical Constants
- **PI**: `math_pi()` - π (3.14159...)
- **E**: `math_e()` - Euler's number (2.71828...)
- **TAU**: `math_tau()` - τ = 2π (6.28318...)

### Basic Operations
- **Absolute Value**: `math_abs()`, `math_abs_int()`
- **Min/Max**: `math_min()`, `math_max()`, `math_min_int()`, `math_max_int()`
- **Clamping**: `math_clamp()`
- **Sign**: `math_sign()`

### Power & Logarithmic Functions
- **Power**: `math_pow()`, `math_sqrt()`, `math_cbrt()`
- **Exponential**: `math_exp()`, `math_exp2()`
- **Logarithms**: `math_log()`, `math_log10()`, `math_log2()`

### Trigonometric Functions
- **Basic Trig**: `math_sin()`, `math_cos()`, `math_tan()`
- **Inverse Trig**: `math_asin()`, `math_acos()`, `math_atan()`, `math_atan2()`
- **Hyperbolic**: `math_sinh()`, `math_cosh()`, `math_tanh()`

### Rounding & Truncation
- **Rounding**: `math_floor()`, `math_ceil()`, `math_round()`, `math_trunc()`
- **Fractional**: `math_frac()`

### Statistical Functions
- **Aggregate**: `math_sum()`, `math_mean()`, `math_median()`
- **Variance**: `math_variance()`, `math_std_dev()`

### Random Number Generation
- **Random**: `math_random()`, `math_random_int()`, `math_random_float()`
- **Seeding**: `math_seed_random()`

### Utility Functions
- **Validation**: `math_is_nan()`, `math_is_infinite()`, `math_is_finite()`
- **Conversion**: `math_degrees()`, `math_radians()`
- **Number Theory**: `math_gcd()`, `math_lcm()`, `math_factorial()`, `math_fibonacci()`

### Interpolation
- **Linear**: `math_lerp()`, `math_inverse_lerp()`
- **Smooth**: `math_smoothstep()`

### Distance & Geometry
- **2D/3D Distance**: `math_distance_2d()`, `math_distance_3d()`
- **Vector Operations**: `math_dot_product_2d()`, `math_cross_product_2d()`
- **Magnitude**: `math_magnitude_2d()`, `math_normalize_2d()`

## Usage Examples

```cursed
yeet "math"

// Basic calculations
sus radius meal = 5.0
sus area meal = math_pi() * math_pow(radius, 2.0)
sus circumference meal = 2.0 * math_pi() * radius

// Trigonometry
sus angle meal = math_radians(45.0)  // Convert degrees to radians
sus sin_val meal = math_sin(angle)
sus cos_val meal = math_cos(angle)

// Statistics
sus data [meal] = [1.0, 2.0, 3.0, 4.0, 5.0]
sus average meal = math_mean(data)
sus std_deviation meal = math_std_dev(data)

// Random numbers
math_seed_random(42)
sus random_float meal = math_random()
sus random_int normie = math_random_int(1, 100)

// Number theory
sus gcd_result normie = math_gcd(48, 18)
sus factorial_5 normie = math_factorial(5)
sus fib_10 normie = math_fibonacci(10)
```

## Performance

The math module is optimized for performance:
- Uses efficient algorithms for transcendental functions
- Leverages hardware acceleration where available
- Minimizes floating-point precision errors
- Optimized random number generation

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/math/test_math.csd
```

## Status

✅ **Production Ready**: All functions implemented and tested
✅ **High Precision**: IEEE 754 compliant floating-point operations
✅ **Pure CURSED**: Core implementations use native CURSED code
✅ **Cross-Platform**: Consistent results across all platforms
✅ **Fully Tested**: Comprehensive test coverage including edge cases
