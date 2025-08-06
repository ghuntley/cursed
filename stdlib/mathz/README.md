# mathz Module - CURSED Mathematical Functions

The `mathz` module provides comprehensive mathematical functions and constants for the CURSED programming language, equivalent to Go's `math` package.

## Mathematical Constants

- `PI` - π (3.141592653589793)
- `E` - Euler's number (2.718281828459045)
- `TAU` - 2π (6.283185307179586)
- `SQRT_2` - √2 (1.4142135623730951)
- `SQRT_3` - √3 (1.7320508075688772)
- `LN_2` - ln(2) (0.6931471805599453)
- `LN_10` - ln(10) (2.302585092994046)
- `GOLDEN_RATIO` - φ (1.618033988749895)
- `EPSILON` - Machine epsilon for floating-point comparisons

## Basic Arithmetic

- `math_add(a, b meal)` - Addition
- `math_subtract(a, b meal)` - Subtraction
- `math_multiply(a, b meal)` - Multiplication
- `math_divide(a, b meal)` - Division (safe, returns 0 for division by zero)

## Absolute Value Functions

- `abs_meal(x meal)` - Absolute value for floats
- `abs_normie(x normie)` - Absolute value for integers

## Min/Max Functions

- `max_meal(a, b meal)` - Maximum of two floats
- `max_normie(a, b normie)` - Maximum of two integers
- `min_meal(a, b meal)` - Minimum of two floats
- `min_normie(a, b normie)` - Minimum of two integers

## Rounding Functions

- `floor_meal(x meal)` - Floor function (rounds down)
- `ceil_meal(x meal)` - Ceiling function (rounds up)
- `round_meal(x meal)` - Round to nearest integer

## Power and Root Functions

- `pow_meal(base meal, exp normie)` - Power function (integer exponent)
- `pow_meal_meal(base meal, exp meal)` - Power function (float exponent)
- `sqrt_meal(x meal)` - Square root using Newton's method

## Exponential and Logarithmic Functions

- `exp_meal(x meal)` - Exponential function (e^x)
- `ln_meal(x meal)` - Natural logarithm using Taylor series
- `log10_meal(x meal)` - Base-10 logarithm
- `log2_meal(x meal)` - Base-2 logarithm

## Trigonometric Functions

- `sin_meal(x meal)` - Sine function (radians)
- `cos_meal(x meal)` - Cosine function (radians)
- `tan_meal(x meal)` - Tangent function (radians)
- `sin_deg(degrees meal)` - Sine function (degrees)
- `cos_deg(degrees meal)` - Cosine function (degrees)
- `tan_deg(degrees meal)` - Tangent function (degrees)

## Inverse Trigonometric Functions

- `asin_meal(x meal)` - Arcsine function (returns radians)
- `acos_meal(x meal)` - Arccosine function (returns radians)
- `atan_meal(x meal)` - Arctangent function (returns radians)

## Hyperbolic Functions

- `sinh_meal(x meal)` - Hyperbolic sine
- `cosh_meal(x meal)` - Hyperbolic cosine
- `tanh_meal(x meal)` - Hyperbolic tangent

## Angle Utilities

- `normalize_radians(angle meal)` - Normalize angle to [0, 2π]
- `normalize_degrees(angle meal)` - Normalize angle to [0, 360]

## Utility Functions

- `is_approximately_equal(a, b, epsilon meal)` - Floating-point equality
- `is_zero(x meal)` - Check if value is approximately zero
- `is_positive_meal(x meal)` - Check if positive
- `is_negative_meal(x meal)` - Check if negative
- `is_even(x normie)` - Check if integer is even
- `is_odd(x normie)` - Check if integer is odd
- `clamp_meal(value, min_val, max_val meal)` - Clamp value to range (floats)
- `clamp_normie(value, min_val, max_val normie)` - Clamp value to range (integers)
- `lerp_meal(a, b, t meal)` - Linear interpolation between a and b
- `sign_meal(x meal)` - Sign function for floats (-1, 0, or 1)
- `sign_normie(x normie)` - Sign function for integers (-1, 0, or 1)
- `trunc_meal(x meal)` - Truncate towards zero
- `frac_meal(x meal)` - Fractional part of a number
- `fmod_meal(x, y meal)` - Floating-point modulo
- `is_nan(x meal)` - Check if value is NaN
- `is_infinite(x meal)` - Check if value is infinite
- `is_finite(x meal)` - Check if value is finite

## Number Theory Functions

- `factorial(n normie)` - Factorial function
- `gcd(a, b normie)` - Greatest common divisor
- `lcm(a, b normie)` - Least common multiple
- `fibonacci(n normie)` - Fibonacci sequence
- `is_prime(n normie)` - Check if number is prime

## Random Number Generation

- `set_random_seed(seed normie)` - Set random seed
- `random_int()` - Generate random integer
- `random_meal()` - Generate random float [0, 1]
- `random_range(min_val, max_val normie)` - Random integer in range
- `random_meal_range(min_val, max_val meal)` - Random float in range
- `random_gaussian()` - Generate Gaussian-distributed random number

## Mathematical Series

- `arithmetic_sum(first, last, count normie)` - Sum of arithmetic series
- `geometric_sum(first, ratio meal, count normie)` - Sum of geometric series

## Distance Functions

- `distance_2d(x1, y1, x2, y2 meal)` - Euclidean distance in 2D
- `distance_3d(x1, y1, z1, x2, y2, z2 meal)` - Euclidean distance in 3D

## Example Usage

```cursed
yeet "mathz"

slay main() {
    fr fr Basic calculations
    sus area meal = mathz.PI * mathz.pow_meal(5.0, 2)
    vibez.spillf("Circle area: %f", area)
    
    fr fr Trigonometry
    sus angle meal = 45.0 * mathz.DEGREES_TO_RADIANS
    sus sin_val meal = mathz.sin_meal(angle)
    vibez.spillf("sin(45°): %f", sin_val)
    
    fr fr Square root
    sus sqrt_val meal = mathz.sqrt_meal(25.0)
    vibez.spillf("√25 = %f", sqrt_val)
    
    fr fr Random numbers
    mathz.set_random_seed(12345)
    sus rand_val meal = mathz.random_meal()
    vibez.spillf("Random: %f", rand_val)
    
    fr fr Number theory
    sus fact_val normie = mathz.factorial(5)
    vibez.spillf("5! = %d", fact_val)
    
    sus gcd_val normie = mathz.gcd(48, 18)
    vibez.spillf("gcd(48, 18) = %d", gcd_val)
}
```

## Implementation Details

- Pure CURSED implementation with no external dependencies
- Taylor series approximations for transcendental functions
- Newton's method for square root calculation
- Linear congruential generator for random numbers
- Euclidean algorithm for GCD computation
- Safe fallbacks for edge cases (negative square roots, division by zero)

## Accuracy and Performance

- High precision for most mathematical operations
- Iterative algorithms with convergence criteria
- Optimized for common mathematical use cases
- Thread-safe random number generation

## Testing

Run tests with:
```bash
cargo run --bin cursed stdlib/mathz/test_mathz.csd
```

The comprehensive test suite covers all mathematical functions, constants, edge cases, and numerical accuracy.

## Future Enhancements

- Complex number support
- Matrix operations
- Statistical functions
- Arbitrary precision arithmetic
- Hardware-accelerated operations
- Additional special functions (gamma, beta, etc.)
