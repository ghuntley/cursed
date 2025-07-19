# mathz - Comprehensive Mathematics Module

## Overview

The `mathz` module provides a complete suite of mathematical functions implemented in pure CURSED without any FFI dependencies. This module has been migrated from the Rust implementation to enable full self-hosting capability and maximum portability.

## Features

### Mathematical Constants
- `PI`, `E`, `TAU` - Fundamental mathematical constants
- `SQRT_2`, `SQRT_3` - Common square roots
- `LN_2`, `LN_10`, `LOG2_E`, `LOG10_E` - Logarithmic constants
- `GOLDEN_RATIO`, `EULER_MASCHERONI` - Special mathematical constants
- `DEGREES_TO_RADIANS`, `RADIANS_TO_DEGREES` - Angle conversion constants
- `EPSILON` - Floating-point precision constant

### Basic Arithmetic
- `math_add(a, b)` - Addition
- `math_subtract(a, b)` - Subtraction
- `math_multiply(a, b)` - Multiplication
- `math_divide(a, b)` - Safe division (handles division by zero)

### Absolute Value and Comparison
- `abs_meal(x)`, `abs_normie(x)` - Absolute value for different types
- `max_meal(a, b)`, `max_normie(a, b)` - Maximum of two values
- `min_meal(a, b)`, `min_normie(a, b)` - Minimum of two values

### Rounding and Truncation
- `floor_meal(x)` - Floor function (round down)
- `ceil_meal(x)` - Ceiling function (round up)
- `round_meal(x)` - Round to nearest integer

### Power and Root Functions
- `pow_meal(base, exp)` - Integer exponentiation
- `pow_meal_meal(base, exp)` - Floating-point exponentiation
- `sqrt_meal(x)` - Square root using Newton's method

### Logarithmic and Exponential Functions
- `ln_meal(x)` - Natural logarithm (Taylor series)
- `exp_meal(x)` - Exponential function (Taylor series)

### Trigonometric Functions
- `sin_meal(x)`, `cos_meal(x)`, `tan_meal(x)` - Basic trigonometric functions
- `sin_deg(x)`, `cos_deg(x)`, `tan_deg(x)` - Degree-based trigonometric functions
- `normalize_radians(angle)`, `normalize_degrees(angle)` - Angle normalization

### Utility Functions
- `is_approximately_equal(a, b, epsilon)` - Floating-point comparison
- `is_zero(x)` - Zero check with epsilon tolerance
- `is_positive_meal(x)`, `is_negative_meal(x)` - Sign checking
- `is_even(x)`, `is_odd(x)` - Parity checking

### Number Theory
- `factorial(n)` - Factorial calculation
- `gcd(a, b)` - Greatest common divisor
- `lcm(a, b)` - Least common multiple
- `fibonacci(n)` - Fibonacci sequence

### Random Number Generation
- `set_random_seed(seed)` - Set random number seed
- `random_int()` - Generate random integer
- `random_meal()` - Generate random float [0, 1)
- `random_range(min, max)` - Generate random integer in range

### Array Statistics
- `mean_array(values, count)` - Calculate mean of array
- `sum_array(values, count)` - Sum all elements in array
- `max_array(values, count)`, `min_array(values, count)` - Find extrema

### Complex Numbers
- `Complex` - Complex number type with real and imaginary parts
- `complex_new(real, imag)` - Create complex number
- `complex_add(a, b)` - Complex addition
- `complex_multiply(a, b)` - Complex multiplication
- `complex_magnitude(c)` - Complex magnitude

### Matrix Operations (2x2)
- `Matrix2x2` - 2x2 matrix type
- `matrix_new(a00, a01, a10, a11)` - Create matrix
- `matrix_add(m1, m2)` - Matrix addition
- `matrix_multiply(m1, m2)` - Matrix multiplication
- `matrix_determinant(m)` - Matrix determinant

## Implementation Details

### Pure CURSED Implementation
All functions are implemented using only CURSED language features:
- No FFI calls to external libraries
- No unsafe operations
- Taylor series approximations for transcendental functions
- Newton's method for square roots
- Linear congruential generator for random numbers

### Error Handling
- Safe fallbacks for undefined operations (e.g., division by zero returns 0.0)
- Domain checking for functions like square root (negative inputs return 0.0)
- Convergence limits for iterative algorithms to prevent infinite loops

### Performance Considerations
- Optimized algorithms with reasonable iteration limits
- Epsilon-based convergence for floating-point precision
- Efficient implementations suitable for compiler use

## Usage Examples

```cursed
yeet "mathz"

# Basic arithmetic
sus result meal = math_add(5.0, 3.0)
vibez.spill("5 + 3 =", result)

# Trigonometry
sus sine meal = sin_meal(PI / 2.0)
vibez.spill("sin(π/2) =", sine)

# Complex numbers
sus c Complex = complex_new(3.0, 4.0)
sus magnitude meal = complex_magnitude(c)
vibez.spill("Magnitude of 3+4i =", magnitude)

# Statistics
sus data []meal = [1.0, 2.0, 3.0, 4.0, 5.0]
sus avg meal = mean_array(data, 5)
vibez.spill("Average =", avg)
```

## Testing

Run the comprehensive test suite:

```bash
# Interpretation mode
cargo run --bin cursed stdlib/mathz/test_mathz.csd

# Compilation mode
cargo run --bin cursed -- compile stdlib/mathz/test_mathz.csd
./test_mathz
```

## Migration Status

✅ **COMPLETE**: Successfully migrated from Rust implementation
- All critical mathematical functions ported
- Comprehensive test coverage
- Both interpretation and compilation modes supported
- Performance validated for compiler use
- Zero FFI dependencies achieved

## Integration

This module is designed for:
- Self-hosting CURSED compiler mathematical operations
- Application-level mathematical computations
- Educational and scientific computing
- Game development mathematical functions

The `mathz` module provides enterprise-grade mathematical functionality suitable for production use in both interpreted and compiled CURSED programs.
