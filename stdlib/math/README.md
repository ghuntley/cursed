# CURSED Math Module

A comprehensive mathematical library for the CURSED programming language, providing advanced mathematical functions with high numerical accuracy.

## Overview

The math module implements a full suite of mathematical functions including:
- Basic arithmetic operations
- Trigonometric and inverse trigonometric functions
- Logarithmic and exponential functions
- Hyperbolic functions
- Special functions (gamma, beta, Bessel)
- Statistical analysis functions
- Numerical analysis tools
- Number theory utilities

## Core Features

### Mathematical Constants
- `PI` - π (3.141592653589793)
- `E` - Euler's number (2.718281828459045)
- `LN2` - Natural logarithm of 2
- `LN10` - Natural logarithm of 10
- `SQRT2` - Square root of 2
- `EPSILON` - Machine epsilon for floating-point comparisons

### Function Categories

#### Basic Arithmetic
```cursed
yeet "math"

# Integer operations
sus sum normie = add(5, 3)                    # Returns 8
sus diff normie = subtract(10, 4)             # Returns 6
sus product normie = multiply(6, 7)           # Returns 42
sus quotient normie = divide(15, 3)           # Returns 5

# Power functions
sus power_result normie = pow_int(2, 3)       # Returns 8
sus float_power meal = pow_float(2.5, 3.0)    # Returns 15.625

# Absolute values
sus abs_int normie = abs_int(-5)              # Returns 5
sus abs_float meal = abs_float(-3.14)         # Returns 3.14
```

#### Exponential and Logarithmic Functions
```cursed
# Exponential functions
sus exp_result meal = exp_float(1.0)          # Returns e ≈ 2.718
sus exp2_result meal = exp2(3.0)              # Returns 2³ = 8.0
sus sqrt_result meal = sqrt(16.0)             # Returns 4.0

# Logarithmic functions
sus ln_result meal = ln(E)                    # Returns 1.0
sus log10_result meal = log10(100.0)          # Returns 2.0
sus log2_result meal = log2(8.0)              # Returns 3.0
sus logn_result meal = log_base(8.0, 2.0)     # Returns 3.0
```

#### Trigonometric Functions
```cursed
# Basic trigonometric functions
sus sin_result meal = sin(PI / 2.0)           # Returns 1.0
sus cos_result meal = cos(0.0)                # Returns 1.0
sus tan_result meal = tan(PI / 4.0)           # Returns 1.0

# Inverse trigonometric functions
sus asin_result meal = asin(1.0)              # Returns π/2
sus acos_result meal = acos(0.0)              # Returns π/2
sus atan_result meal = atan(1.0)              # Returns π/4
sus atan2_result meal = atan2(1.0, 1.0)       # Returns π/4
```

#### Hyperbolic Functions
```cursed
# Hyperbolic functions
sus sinh_result meal = sinh(1.0)              # Returns (e - e⁻¹)/2
sus cosh_result meal = cosh(0.0)              # Returns 1.0
sus tanh_result meal = tanh(0.0)              # Returns 0.0
```

#### Special Functions
```cursed
# Special mathematical functions
sus factorial_result meal = factorial(5)       # Returns 120.0
sus gamma_result meal = gamma(4.0)            # Returns 6.0 (3!)
sus beta_result meal = beta(2.0, 3.0)         # Returns Γ(2)Γ(3)/Γ(5)
sus bessel_result meal = bessel_j0(0.0)       # Returns 1.0
```

#### Statistical Functions
```cursed
# Statistical analysis
sus data [5]meal = [1.0, 2.0, 3.0, 4.0, 5.0]
sus mean_val meal = mean(data, 5)             # Returns 3.0
sus variance_val meal = variance(data, 5)     # Returns sample variance
sus std_dev meal = standard_deviation(data, 5) # Returns standard deviation
sus median_val meal = median(data, 5)         # Returns 3.0 (middle value)
```

#### Numerical Analysis
```cursed
# Numerical integration using Simpson's rule
sus function_values [5]meal = [1.0, 4.0, 6.0, 4.0, 1.0]
sus integral meal = integrate_simpson(function_values, 5, 0.5)

# Numerical differentiation using central difference
sus derivative meal = differentiate_central(function_values, 5, 0.5, 2)

# Linear system solver (2x2)
sus x1 meal = solve_linear_2x2(2.0, 1.0, 3.0, 1.0, 1.0, 2.0)  # Solve Ax = b
```

#### Utility Functions
```cursed
# Rounding and floor/ceiling
sus floor_val meal = floor_float(3.7)         # Returns 3.0
sus ceil_val meal = ceil_float(3.2)           # Returns 4.0
sus round_val meal = round_float(3.6)         # Returns 4.0

# Number theory
sus gcd_result normie = gcd(48, 18)           # Returns 6
sus lcm_result normie = lcm(4, 6)             # Returns 12
sus is_prime_result lit = is_prime(17)        # Returns based (true)
sus fib_result normie = fibonacci(10)         # Returns 55
```

## Type System

The math module uses CURSED's native type system:
- `normie` - 32-bit signed integers (i32)
- `meal` - 64-bit floating-point numbers (f64) 
- `snack` - 32-bit floating-point numbers (f32)
- `lit` - Boolean values (`based`/`cap`)

## Numerical Accuracy

All functions are implemented with high numerical accuracy:
- Taylor series expansions for trigonometric functions
- Newton-Raphson methods for square roots
- Stirling's approximation for gamma function
- IEEE 754 compliance for floating-point operations
- Epsilon-based convergence testing (1e-15)

## Error Handling

The module handles edge cases gracefully:
- Division by zero returns safe defaults
- Invalid domain inputs return appropriate boundary values
- Negative inputs to sqrt return 0.0
- Overflow conditions are managed with large finite values

## Performance

The implementation prioritizes accuracy over raw speed:
- Iterative algorithms with convergence testing
- Maximum iteration limits to prevent infinite loops
- Optimized series expansions for common functions
- Efficient Newton-Raphson implementations

## Usage Examples

### Complex Calculations
```cursed
yeet "math"

# Calculate trajectory using projectile motion
sus angle meal = PI / 4.0  # 45 degrees
sus velocity meal = 20.0
sus gravity meal = 9.81

sus vx meal = velocity * cos(angle)
sus vy meal = velocity * sin(angle)
sus flight_time meal = 2.0 * vy / gravity
sus range meal = vx * flight_time

vibez.spill("Projectile range:", range)
```

### Statistical Analysis
```cursed
yeet "math"

# Analyze data set
sus measurements [10]meal = [12.5, 13.1, 12.8, 13.0, 12.7, 13.2, 12.9, 13.1, 12.6, 13.0]

sus avg meal = mean(measurements, 10)
sus var meal = variance(measurements, 10)
sus std meal = standard_deviation(measurements, 10)

vibez.spill("Mean:", avg)
vibez.spill("Standard deviation:", std)
```

### Numerical Integration
```cursed
yeet "math"

# Integrate x² from 0 to 2 using Simpson's rule
sus n normie = 21  # Number of points (must be odd)
sus h meal = 2.0 / (n - 1).(meal)
sus values [21]meal

# Fill with x² values
sus i normie = 0
bestie i < n {
    sus x meal = i.(meal) * h
    values[i] = x * x
    i = i + 1
}

sus result meal = integrate_simpson(values, n, h)
vibez.spill("Integral of x² from 0 to 2:", result)  # Should be ≈ 2.67
```

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/math/test_math.💀
```

The test suite includes:
- 100+ test cases covering all functions
- Accuracy validation within numerical tolerance
- Edge case testing
- Statistical function validation
- Numerical analysis verification

## Implementation Details

### Trigonometric Functions
- Uses Taylor series expansions
- Input normalization to [-π, π] range
- Convergence testing with machine epsilon
- Special handling for exact values (0, π/2, π)

### Logarithmic Functions
- Newton's method for natural logarithm
- Base conversion using change of base formula
- Domain validation for positive inputs
- High precision for values near 1

### Special Functions
- Stirling's approximation for gamma function
- Asymptotic expansions for Bessel functions
- Recursive relationships for factorial computation
- Series expansions with controlled convergence

### Statistical Functions
- Sample variance calculation (n-1 denominator)
- Numerically stable algorithms
- Efficient single-pass computations where possible
- Proper handling of edge cases (empty/single element arrays)

## Dependencies

The math module is implemented in pure CURSED without external dependencies:
- No FFI bridges required
- Self-contained mathematical implementations
- Platform-independent algorithms
- Compatible with both interpretation and compilation modes

## Contributing

When adding new mathematical functions:
1. Implement using pure CURSED syntax
2. Add comprehensive test cases
3. Document function behavior and accuracy
4. Handle edge cases gracefully
5. Use appropriate CURSED types (`meal`, `normie`, `lit`)
6. Follow existing naming conventions

## License

This module is part of the CURSED standard library and follows the same licensing terms as the main CURSED project.
