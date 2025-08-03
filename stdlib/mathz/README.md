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

## Trigonometric Functions

- `sin_meal(x meal)` - Sine function (radians)
- `cos_meal(x meal)` - Cosine function (radians)
- `tan_meal(x meal)` - Tangent function (radians)
- `sin_deg(degrees meal)` - Sine function (degrees)
- `cos_deg(degrees meal)` - Cosine function (degrees)
- `tan_deg(degrees meal)` - Tangent function (degrees)

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

## Number Theory Functions

- `factorial(n normie)` - Factorial function
- `gcd(a, b normie)` - Greatest common divisor
- `lcm(a, b normie)` - Least common multiple
- `fibonacci(n normie)` - Fibonacci sequence

## Random Number Generation

- `set_random_seed(seed normie)` - Set random seed
- `random_int()` - Generate random integer
- `random_meal()` - Generate random float [0, 1]
- `random_range(min_val, max_val normie)` - Random integer in range

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
