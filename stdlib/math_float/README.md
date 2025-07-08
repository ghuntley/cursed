# Pure CURSED Math Float Module

A comprehensive IEEE 754 compliant floating-point mathematical library implemented entirely in pure CURSED without any FFI dependencies.

## Features

### Mathematical Constants
- `PI()`, `E()`, `TAU()` - Fundamental constants
- `SQRT_2()`, `SQRT_3()` - Square root constants
- `LN_2()`, `LN_10()`, `LOG2_E()`, `LOG10_E()` - Logarithm constants
- `GOLDEN_RATIO()`, `EULER_MASCHERONI()` - Special constants

### IEEE 754 Special Values
- `INFINITY()`, `NEG_INFINITY()` - Infinite values
- `NAN()` - Not-a-Number value
- `EPSILON()` - Machine epsilon for floating-point precision

### Basic Operations
- `abs(x)` - Absolute value
- `sign(x)` - Sign function (-1, 0, 1)
- `min(a, b)`, `max(a, b)` - Minimum and maximum
- `clamp(x, min, max)` - Clamp value to range

### Rounding Functions
- `floor(x)` - Round down to nearest integer
- `ceil(x)` - Round up to nearest integer
- `round(x)` - Round to nearest integer
- `trunc(x)` - Truncate to integer part
- `frac(x)` - Get fractional part

### Power Functions
- `pow_int(base, exp)` - Integer power (fast exponentiation)
- `sqrt(x)` - Square root (Newton's method)
- `cbrt(x)` - Cube root (Newton's method)

### Exponential Functions
- `exp(x)` - Natural exponential (Taylor series)
- `exp2(x)` - Base-2 exponential

### Logarithmic Functions
- `ln(x)` - Natural logarithm (Taylor series)
- `log10(x)` - Base-10 logarithm
- `log2(x)` - Base-2 logarithm

### Trigonometric Functions
- `sin(x)`, `cos(x)`, `tan(x)` - Basic trigonometric functions (Taylor series)
- `asin(x)`, `acos(x)`, `atan(x)` - Inverse trigonometric functions
- `atan2(y, x)` - Two-argument arctangent

### Hyperbolic Functions
- `sinh(x)`, `cosh(x)`, `tanh(x)` - Hyperbolic functions

### Utility Functions
- `is_nan(x)`, `is_infinite(x)`, `is_finite(x)` - IEEE 754 classification
- `is_zero(x)` - Check if value is effectively zero
- `approximately_equal(a, b, epsilon)` - Floating-point comparison
- `fmod(x, y)` - Floating-point remainder
- `remainder(x, y)` - IEEE remainder

### Conversion Functions
- `degrees(radians)` - Convert radians to degrees
- `radians(degrees)` - Convert degrees to radians

### Linear Interpolation
- `lerp(a, b, t)` - Linear interpolation
- `inverse_lerp(a, b, value)` - Inverse linear interpolation
- `smoothstep(edge0, edge1, x)` - Smooth interpolation

## Implementation Details

### Taylor Series Precision
- All transcendental functions use Taylor series expansions
- Convergence criteria: either 50 iterations or epsilon precision
- Optimized for accuracy within IEEE 754 double precision

### Newton's Method
- Square root and cube root use Newton's method
- Convergence criteria: 20 iterations or epsilon precision
- Handles edge cases (negative inputs, zero)

### Fast Exponentiation
- Integer powers use binary exponentiation
- O(log n) complexity for integer exponents
- Handles negative exponents correctly

### IEEE 754 Compliance
- Proper handling of special values (NaN, infinity)
- Correct behavior for edge cases
- Standard rounding modes and precision

## Usage Examples

```cursed
yeet "math_float"

// Basic operations
sus x meal = abs(-3.14);
sus y meal = sqrt(2.0);
sus z meal = sin(PI() / 4.0);

// Power and exponential
sus power meal = pow_int(2.0, 10);
sus natural_exp meal = exp(1.0);
sus logarithm meal = ln(E());

// Trigonometry
sus angle meal = PI() / 6.0;
sus sine meal = sin(angle);
sus cosine meal = cos(angle);
sus tangent meal = tan(angle);

// Utility functions
sus is_valid lit = is_finite(x);
sus is_equal lit = approximately_equal(y, z, EPSILON());

// Interpolation
sus interpolated meal = lerp(0.0, 10.0, 0.5);
sus smooth meal = smoothstep(0.0, 1.0, 0.3);
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/math_float/test_math_float.csd
```

The test suite includes:
- All mathematical constants
- IEEE 754 special values
- Basic operations and edge cases
- Rounding function accuracy
- Power function precision
- Exponential and logarithmic accuracy
- Trigonometric function precision
- Hyperbolic function correctness
- Utility function behavior
- Conversion accuracy
- Interpolation correctness

## Performance

- Pure CURSED implementation with no FFI overhead
- Optimized algorithms for best performance/accuracy trade-off
- Suitable for production mathematical computations
- Consistent behavior across compilation and interpretation modes

## Accuracy

- Double precision IEEE 754 compliance
- Relative error typically < 1e-14 for most functions
- Proper handling of special cases and edge conditions
- Extensive test coverage for numerical accuracy
