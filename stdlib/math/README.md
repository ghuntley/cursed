# CURSED Math Library Tests

This directory contains comprehensive tests for the CURSED math standard library.

## Test Coverage

The `test_math.csd` file provides complete test coverage for all math functions:

### Mathematical Constants
- `math_pi()` - π constant (3.141592653589793)
- `math_e()` - Euler's number (2.718281828459045)
- `math_tau()` - τ constant (6.283185307179586)

### Basic Operations
- `math_abs()` - Absolute value for floats
- `math_abs_int()` - Absolute value for integers
- `math_min()` / `math_max()` - Minimum and maximum values
- `math_clamp()` - Clamp value between bounds
- `math_sign()` - Sign function

### Power and Logarithms
- `math_pow()` - Power function
- `math_sqrt()` - Square root
- `math_cbrt()` - Cube root
- `math_log()` - Natural logarithm
- `math_log10()` - Base-10 logarithm
- `math_log2()` - Base-2 logarithm
- `math_exp()` - Exponential function

### Trigonometric Functions
- `math_sin()`, `math_cos()`, `math_tan()` - Basic trig functions
- `math_asin()`, `math_acos()`, `math_atan()` - Inverse trig functions
- `math_atan2()` - Two-argument arctangent
- `math_sinh()`, `math_cosh()`, `math_tanh()` - Hyperbolic functions

### Rounding and Truncation
- `math_floor()` - Floor function
- `math_ceil()` - Ceiling function
- `math_round()` - Rounding function
- `math_trunc()` - Truncation function

### Utility Functions
- `math_degrees()` / `math_radians()` - Angle conversion
- `math_distance_2d()` / `math_distance_3d()` - Distance calculations
- `math_lerp()` - Linear interpolation
- `math_gcd()` / `math_lcm()` - Greatest common divisor/least common multiple
- `math_factorial()` - Factorial function
- `math_fibonacci()` - Fibonacci sequence

### Random Numbers
- `math_random()` - Random float [0,1)
- `math_random_int()` - Random integer in range
- `math_random_float()` - Random float in range

### Edge Cases Tested
- Division by zero protection
- Negative square roots (NaN)
- Infinite number detection
- Finite number validation

## Running Tests

```bash
# Run math tests specifically
cargo run --bin cursed stdlib/math/test_math.csd

# Run all stdlib tests
cargo run --bin cursed test
```

## Test Results

All tests verify:
- Correct mathematical calculations
- Proper handling of edge cases
- Expected return types
- Error conditions and special values

The tests ensure that the math library functions behave correctly in both interpretation and native compilation modes.
