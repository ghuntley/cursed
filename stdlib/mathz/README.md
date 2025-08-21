# MATHZ - Complete Mathematical Operations Library

**Version**: 1.0.0 Production Ready  
**Language**: Pure CURSED Implementation  
**Performance**: Optimized for speed and precision

The `mathz` module provides comprehensive mathematical functionality for CURSED programs, implemented entirely in pure CURSED language without external dependencies. All floating-point operations use integer scaling (×10000) for precision.

## 📚 Function Categories

### 🔢 Mathematical Constants
```cursed
PI()                    // π ≈ 3.1416
E()                     // e ≈ 2.7183
TAU()                   // 2π ≈ 6.2832
SQRT_2()                // √2 ≈ 1.4142
SQRT_3()                // √3 ≈ 1.7321
GOLDEN_RATIO()          // φ ≈ 1.618
LN_2()                  // ln(2) ≈ 0.6931
LN_10()                 // ln(10) ≈ 2.3026
```

### ➕ Basic Arithmetic Operations
```cursed
abs(x)                  // Absolute value
max(a, b)               // Maximum of two numbers
min(a, b)               // Minimum of two numbers
add(a, b)               // Addition
subtract(a, b)          // Subtraction
multiply(a, b)          // Multiplication
divide(a, b)            // Safe division (returns 0 for division by zero)
power(base, exp)        // Exponentiation with negative exponent support
mod(a, b)               // Modulo operation
```

### 🚀 Advanced Mathematical Functions
```cursed
sqrt(x)                 // Square root using Newton's method
factorial(n)            // Factorial with lookup table optimization
gcd(a, b)               // Greatest common divisor
lcm(a, b)               // Least common multiple
```

### 📐 Trigonometric Functions
```cursed
degrees_to_radians(deg) // Convert degrees to radians
radians_to_degrees(rad) // Convert radians to degrees
sin(x)                  // Sine using Taylor series
cos(x)                  // Cosine using Taylor series
tan(x)                  // Tangent (sin/cos)
```

### 📊 Logarithmic and Exponential Functions
```cursed
log2(x)                 // Base-2 logarithm
log10(x)                // Base-10 logarithm
ln(x)                   // Natural logarithm
exp(x)                  // Exponential function (e^x)
```

### 🔄 Rounding and Precision Functions
```cursed
floor(x)                // Floor function
ceil(x)                 // Ceiling function
round(x)                // Round to nearest integer
trunc(x)                // Truncate decimal part
precision_round(x, dp)  // Round to specified decimal places
```

### 🛠️ Utility Functions
```cursed
is_even(n)              // Check if number is even
is_odd(n)               // Check if number is odd
sign(x)                 // Sign function (-1, 0, 1)
clamp(val, min, max)    // Clamp value to range
is_approximately_equal(a, b, eps) // Floating-point equality
lerp(a, b, t)           // Linear interpolation
```

### 📈 Statistical Functions
```cursed
sum_array(arr, size)    // Sum of array elements
average(arr, size)      // Average of array elements
find_min(arr, size)     // Minimum element in array
find_max(arr, size)     // Maximum element in array
```

### 🔍 Number Theory Functions
```cursed
is_prime(n)             // Prime number test
next_prime(n)           // Next prime after n
prime_factors_count(n)  // Count of prime factors
```

### 🎯 Combinatorial Functions
```cursed
combinations(n, k)      // Binomial coefficient C(n,k)
permutations(n, k)      // Permutations P(n,k)
```

### 📊 Sequence Functions
```cursed
fibonacci(n)            // nth Fibonacci number
sum_range(start, end)   // Sum of arithmetic sequence
triangular_number(n)    // nth triangular number
square_number(n)        // nth square number
pentagonal_number(n)    // nth pentagonal number
hexagonal_number(n)     // nth hexagonal number
```

### 📏 Geometric Functions
```cursed
distance_2d(x1, y1, x2, y2)    // 2D Euclidean distance
area_circle(radius)            // Circle area
circumference_circle(radius)   // Circle circumference
area_rectangle(width, height)  // Rectangle area
area_triangle(base, height)    // Triangle area
```

### 🔄 Conversion Functions
```cursed
celsius_to_fahrenheit(c)       // Temperature conversion
fahrenheit_to_celsius(f)       // Temperature conversion
km_to_miles(km)                // Distance conversion
miles_to_km(miles)             // Distance conversion
```

### 🔢 Modular Arithmetic
```cursed
mod_add(a, b, mod)      // Modular addition
mod_multiply(a, b, mod) // Modular multiplication
mod_power(base, exp, mod) // Modular exponentiation
```

### 🎲 Random Number Helpers
```cursed
simple_hash(seed)              // Deterministic hash function
random_range(min, max, seed)   // Pseudo-random in range
```

### 🔟 Bitwise Mathematical Operations
```cursed
count_set_bits(n)       // Count 1-bits in binary
is_power_of_2(n)        // Check if power of 2
next_power_of_2(n)      // Next power of 2
```

### 🛡️ Safe Mathematical Operations
```cursed
safe_divide(a, b, default)     // Division with default on zero
safe_sqrt(x)                   // Square root with negative handling
safe_log(x, default)           // Logarithm with domain checking
clamp_positive(x)              // Ensure non-negative value
```

### 🧮 Advanced Mathematical Computations
```cursed
sum_of_squares(n)       // Sum of first n squares
sum_of_cubes(n)         // Sum of first n cubes
arithmetic_mean(a, b)   // Arithmetic mean
geometric_mean(a, b)    // Geometric mean
harmonic_mean(a, b)     // Harmonic mean
```

### ⚖️ Precision Scaling Utilities
```cursed
scale_up(x)             // Multiply by 10000 for precision
scale_down(x)           // Divide by 10000 from precision
```

## 🚀 Quick Start

```cursed
yeet "mathz"

fr fr Basic arithmetic
sus result1 drip = add(10, 20)           // 30
sus result2 drip = power(2, 3)           // 8
sus result3 drip = sqrt(16)              // 4

fr fr Mathematical constants
sus pi_val drip = PI()                   // 31416 (3.1416 * 10000)
sus e_val drip = E()                     // 27183 (2.7183 * 10000)

fr fr Trigonometric functions
sus radians drip = degrees_to_radians(90) // π/2
sus sine drip = sin(radians)             // 1.0 (scaled)

fr fr Advanced functions
sus fact drip = factorial(5)             // 120
sus gcd_result drip = gcd(48, 18)        // 6
sus prime_check lit = is_prime(17)       // based

fr fr Statistical operations
sus numbers []drip = [1, 2, 3, 4, 5]
sus avg drip = average(numbers, 5)       // 3
sus total drip = sum_array(numbers, 5)   // 15

fr fr Output results
vibez.spill("Power result:", result2)
vibez.spill("Square root:", result3)
vibez.spill("Average:", avg)
vibez.spill("Is 17 prime:", prime_check)
```

## 🎯 Performance Features

- **Pure CURSED Implementation**: No external dependencies
- **Integer Scaling**: All floating-point operations use integer arithmetic with 10000× scaling
- **Lookup Tables**: Optimized with precomputed values for common cases
- **Newton's Method**: Fast convergence for square root and other approximations
- **Taylor Series**: High-precision trigonometric functions
- **Memory Efficient**: Minimal memory footprint with iterative algorithms

## 📏 Precision Notes

- All decimal values are scaled by 10000 for integer precision
- PI() returns 31416 (representing 3.1416)
- Results may need to be scaled back using `scale_down()` for display
- Safe operations prevent domain errors and provide sensible defaults

## 🔧 Error Handling

- Division by zero returns 0 (safe default)
- Square root of negative numbers returns 0
- Logarithm of non-positive numbers returns negative infinity approximation
- Invalid combinations/permutations return 0
- Modular operations handle edge cases gracefully

## 🧪 Testing

Comprehensive test suite covers:
- All mathematical operations
- Edge cases and boundary conditions
- Precision and accuracy validation
- Performance benchmarks
- Error handling scenarios

## 📖 Examples

See [tests.csd](tests.csd) for comprehensive usage examples and validation tests.

## 🏗️ Implementation Details

- **Total Functions**: 80+ mathematical operations
- **Code Quality**: Production-ready with comprehensive error handling
- **Performance**: Optimized algorithms with O(1) to O(n) complexity
- **Compatibility**: Works in both interpreter and compiled modes
- **Memory Safety**: No dynamic allocation, stack-based operations

---

*MATHZ Module - Empowering CURSED with mathematical excellence* 🚀
