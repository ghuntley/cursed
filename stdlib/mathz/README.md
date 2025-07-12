# mathz Module

The mathz module provides comprehensive mathematical functions and constants for the CURSED programming language. This module implements pure CURSED mathematical operations without external dependencies.

## Features

### Mathematical Constants
- `Pi`: π (3.14159...)
- `E`: Euler's number (2.71828...)
- `Tau`: τ = 2π (6.28318...)
- `MaxFloat32`: Maximum 32-bit float value
- `MaxFloat64`: Maximum 64-bit float value
- `MinFloat32`: Minimum 32-bit float value
- `MinFloat64`: Minimum 64-bit float value
- `Epsilon`: Floating-point precision epsilon

### Basic Operations
- `Abs(x meal) meal`: Absolute value for floats
- `AbsInt(x normie) normie`: Absolute value for integers
- `Max(x meal, y meal) meal`: Maximum of two values
- `Min(x meal, y meal) meal`: Minimum of two values
- `Sign(x meal) normie`: Sign function (-1, 0, or 1)
- `Clamp(x meal, min meal, max meal) meal`: Clamp value between min and max

### Power and Root Functions
- `Pow(base meal, exp normie) meal`: Power function (base^exp)
- `Sqrt(x meal) meal`: Square root using Newton's method

### Rounding Functions
- `Ceil(x meal) meal`: Ceiling function
- `Floor(x meal) meal`: Floor function
- `Round(x meal) meal`: Round to nearest integer

### Trigonometric Functions
- `Sin(x meal) meal`: Sine function (Taylor series)
- `Cos(x meal) meal`: Cosine function (Taylor series)
- `Tan(x meal) meal`: Tangent function

### Logarithmic and Exponential Functions
- `Log(x meal) meal`: Natural logarithm
- `Exp(x meal) meal`: Exponential function (e^x)

### Angle Conversion
- `RadToDeg(rad meal) meal`: Convert radians to degrees
- `DegToRad(deg meal) meal`: Convert degrees to radians

### Utility Functions
- `Mod(x meal, y meal) meal`: Modulo operation for floats
- `Lerp(a meal, b meal, t meal) meal`: Linear interpolation
- `Hypot(x meal, y meal) meal`: Hypotenuse calculation
- `Distance(x1 meal, y1 meal, x2 meal, y2 meal) meal`: Distance between points

### Number Theory Functions
- `Factorial(n normie) normie`: Factorial function
- `GCD(a normie, b normie) normie`: Greatest common divisor
- `LCM(a normie, b normie) normie`: Least common multiple
- `IsPrime(n normie) lit`: Prime number check

## Usage Examples

```cursed
yeet "mathz"

// Using mathematical constants
sus circumference meal = 2.0 * mathz.Pi * radius
sus area meal = mathz.Pi * radius * radius

// Basic operations
sus distance meal = mathz.Abs(x2 - x1)
sus maximum meal = mathz.Max(value1, value2)
sus minimum meal = mathz.Min(value1, value2)

// Power and root operations
sus square meal = mathz.Pow(value, 2)
sus cube meal = mathz.Pow(value, 3)
sus sqrt_value meal = mathz.Sqrt(16.0)  // Returns 4.0

// Trigonometric calculations
sus angle meal = mathz.DegToRad(45.0)  // Convert 45 degrees to radians
sus sine meal = mathz.Sin(angle)
sus cosine meal = mathz.Cos(angle)
sus tangent meal = mathz.Tan(angle)

// Logarithmic and exponential
sus natural_log meal = mathz.Log(mathz.E)  // Returns 1.0
sus exponential meal = mathz.Exp(1.0)      // Returns E

// Rounding operations
sus ceiling meal = mathz.Ceil(3.2)   // Returns 4.0
sus floor meal = mathz.Floor(3.8)    // Returns 3.0
sus rounded meal = mathz.Round(3.6)  // Returns 4.0

// Number theory
sus fact5 normie = mathz.Factorial(5)     // Returns 120
sus gcd_val normie = mathz.GCD(12, 18)    // Returns 6
sus lcm_val normie = mathz.LCM(4, 6)      // Returns 12
sus is_prime lit = mathz.IsPrime(17)      // Returns based (true)

// Utility functions
sus clamped meal = mathz.Clamp(value, 0.0, 100.0)
sus interpolated meal = mathz.Lerp(start, end, 0.5)
sus hypotenuse meal = mathz.Hypot(3.0, 4.0)  // Returns 5.0
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/mathz/test_mathz.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/mathz/test_mathz.csd
./test_mathz
```

## Implementation Notes

- All functions are implemented in pure CURSED without FFI dependencies
- Trigonometric functions use Taylor series approximations
- Square root uses Newton's method for numerical stability
- Logarithmic and exponential functions use series expansions
- All floating-point operations handle edge cases appropriately
- Mathematical constants are defined with high precision

## Precision and Accuracy

- Trigonometric functions: Accurate to ~10^-6 within normal ranges
- Square root: Accurate to ~10^-7 using Newton's method
- Logarithmic/exponential: Accurate to ~10^-6 for typical values
- All functions handle edge cases (zero, negative, infinity) appropriately

## Error Handling

- Functions return sensible default values for invalid inputs
- Negative square roots return 0.0
- Division by zero in trigonometric functions returns MaxFloat64
- Logarithms of non-positive numbers return MinFloat64
- All functions are designed to be robust and avoid crashes

## Performance

- Optimized for correctness over maximum performance
- Iterative algorithms with reasonable iteration limits
- Efficient approximations for transcendental functions
- Memory-efficient implementations without dynamic allocation

## Dependencies

- `testz`: Testing framework (test files only)
- No external FFI dependencies
- Pure CURSED implementation

## License

This module is part of the CURSED programming language standard library.
