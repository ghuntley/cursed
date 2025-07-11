# Pure CURSED Math Library

A comprehensive mathematical library implemented entirely in pure CURSED without any FFI dependencies.

## Features

### ✅ FFI-Free Implementation
- **Zero External Dependencies**: All mathematical functions implemented in pure CURSED
- **Self-Contained**: No Rust FFI bridges or external library calls
- **Portable**: Works consistently across all platforms and compilation modes
- **Performance Optimized**: Efficient algorithms for both interpretation and compilation

### 🧮 Mathematical Constants
- `math_pi()` - π (pi) constant with full precision
- `math_e()` - e (Euler's number) constant with full precision  
- `math_tau()` - τ (tau) constant (2π) with full precision

### 🔢 Basic Operations
- `math_abs(x)` - Absolute value for floats
- `math_abs_int(x)` - Absolute value for integers
- `math_min(a, b)` / `math_max(a, b)` - Minimum/maximum for floats
- `math_min_int(a, b)` / `math_max_int(a, b)` - Minimum/maximum for integers
- `math_clamp(x, min, max)` - Clamp value between bounds
- `math_sign(x)` - Sign of a number (-1, 0, or 1)

### 🔋 Power Functions
- `math_pow(base, exponent)` - Power function using Newton-Raphson method
- `math_sqrt(x)` - Square root using Newton-Raphson method
- `math_cbrt(x)` - Cube root using Newton-Raphson method

### 📊 Logarithmic Functions
- `math_log(x)` - Natural logarithm using Taylor series
- `math_log10(x)` - Base-10 logarithm
- `math_log2(x)` - Base-2 logarithm

### 🌊 Exponential Functions
- `math_exp(x)` - Natural exponential using Taylor series
- `math_exp2(x)` - Base-2 exponential

### 📐 Trigonometric Functions
- `math_sin(x)` - Sine using Taylor series
- `math_cos(x)` - Cosine using Taylor series
- `math_tan(x)` - Tangent (sin/cos)
- `math_asin(x)` - Inverse sine
- `math_acos(x)` - Inverse cosine
- `math_atan(x)` - Inverse tangent using Taylor series
- `math_atan2(y, x)` - Two-argument arctangent

### 🌀 Hyperbolic Functions
- `math_sinh(x)` - Hyperbolic sine
- `math_cosh(x)` - Hyperbolic cosine
- `math_tanh(x)` - Hyperbolic tangent

### 🔄 Rounding Functions
- `math_floor(x)` - Floor function
- `math_ceil(x)` - Ceiling function
- `math_round(x)` - Round to nearest integer
- `math_trunc(x)` - Truncate decimal part
- `math_frac(x)` - Fractional part

### 🎯 Utility Functions
- `math_is_nan(x)` - Check if value is NaN
- `math_is_infinite(x)` - Check if value is infinite
- `math_is_finite(x)` - Check if value is finite
- `math_degrees(radians)` - Convert radians to degrees
- `math_radians(degrees)` - Convert degrees to radians

### 🔢 Number Theory
- `math_gcd(a, b)` - Greatest common divisor using Euclidean algorithm
- `math_lcm(a, b)` - Least common multiple
- `math_factorial(n)` - Factorial function
- `math_fibonacci(n)` - Fibonacci sequence

### 🎲 Random Numbers
- `math_random()` - Random float between 0 and 1
- `math_random_int(min, max)` - Random integer in range
- `math_random_float(min, max)` - Random float in range
- `math_seed_random(seed)` - Seed the random number generator

### 📈 Statistical Functions
- `math_sum(values)` - Sum of array elements
- `math_mean(values)` - Arithmetic mean
- `math_median(values)` - Median value
- `math_variance(values)` - Variance
- `math_std_dev(values)` - Standard deviation

### 🎨 Interpolation Functions
- `math_lerp(a, b, t)` - Linear interpolation
- `math_inverse_lerp(a, b, value)` - Inverse linear interpolation
- `math_smoothstep(edge0, edge1, x)` - Smooth interpolation

### 📏 Geometry Functions
- `math_distance_2d(x1, y1, x2, y2)` - 2D distance
- `math_distance_3d(x1, y1, z1, x2, y2, z2)` - 3D distance
- `math_dot_product_2d(x1, y1, x2, y2)` - 2D dot product
- `math_cross_product_2d(x1, y1, x2, y2)` - 2D cross product
- `math_magnitude_2d(x, y)` - 2D vector magnitude
- `math_normalize_2d(x, y)` - 2D vector normalization

## Usage

```cursed
yeet "math"

// Basic operations
sus absolute meal = math_abs(-42.5)
sus minimum meal = math_min(10.0, 20.0)
sus maximum meal = math_max(10.0, 20.0)

// Power functions
sus power meal = math_pow(2.0, 3.0)  // 8.0
sus square_root meal = math_sqrt(16.0)  // 4.0

// Trigonometry
sus sine meal = math_sin(math_pi() / 2.0)  // 1.0
sus cosine meal = math_cos(0.0)  // 1.0

// Logarithms and exponentials
sus natural_log meal = math_log(math_e())  // 1.0
sus exponential meal = math_exp(1.0)  // e

// Random numbers
math_seed_random(42)
sus random_val meal = math_random()
sus random_int normie = math_random_int(1, 100)

// Geometry
sus distance meal = math_distance_2d(0.0, 0.0, 3.0, 4.0)  // 5.0
sus interpolated meal = math_lerp(0.0, 10.0, 0.5)  // 5.0
```

## Testing

Run the comprehensive test suite:

```bash
# Test pure CURSED implementation
cargo run --bin cursed stdlib/math/test_math_pure.csd

# Test both interpretation and compilation modes
cargo run --bin cursed stdlib/math/test_math_pure.csd           # Interpretation
cargo run --bin cursed -- compile stdlib/math/test_math_pure.csd  # Compilation
./test_math_pure                                               # Run compiled tests
```

## Implementation Details

### Algorithm Choices

1. **Newton-Raphson Method**: Used for square root, cube root, and power functions
2. **Taylor Series**: Used for trigonometric, logarithmic, and exponential functions
3. **Argument Reduction**: Applied to improve convergence for transcendental functions
4. **Linear Congruential Generator**: Used for random number generation

### Precision and Accuracy

- **Floating Point**: Uses 64-bit double precision (`meal` type)
- **Convergence**: Iterative algorithms use epsilon = 1e-15 for convergence
- **Special Cases**: Proper handling of edge cases (NaN, infinity, division by zero)
- **Range Reduction**: Trigonometric functions normalized to [-π, π] range

### Performance Optimizations

- **Efficient Algorithms**: Optimized for both interpretation and compilation modes
- **Minimal Allocations**: Avoids unnecessary memory allocations
- **Inlined Constants**: Mathematical constants computed at compile time
- **Branch Optimization**: Efficient conditional logic for special cases

## Migration from FFI

This library has been completely migrated from Rust FFI to pure CURSED:

### ✅ Eliminated Dependencies
- Removed all `extern` function declarations
- Eliminated Rust FFI bridges
- Removed external library dependencies
- No more unsafe code blocks

### ✅ Maintained Compatibility
- All function signatures preserved
- Identical behavior to FFI version
- Same precision and accuracy
- Compatible with existing code

### ✅ Performance Benefits
- Native CURSED compilation optimizations
- Reduced function call overhead
- Better integration with CURSED type system
- Improved debugging and error reporting

## Backward Compatibility

The pure CURSED implementation maintains 100% backward compatibility:

- All existing function calls work unchanged
- Same return types and parameter types
- Identical numerical results (within floating-point precision)
- No breaking changes to the API

## Future Enhancements

- Additional statistical functions (mode, quartiles, etc.)
- More geometry functions (polygon area, angle calculations)
- Complex number support
- Matrix operations
- Numerical integration and differentiation
- Optimization algorithms (gradient descent, etc.)

## License

This math library is part of the CURSED programming language project and follows the same license terms.
