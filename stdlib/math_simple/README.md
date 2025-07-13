# math_simple Module

A comprehensive mathematical operations module for CURSED language providing basic arithmetic, common mathematical functions, and utility operations.

## Functions

### Basic Arithmetic Operations

#### Integer Operations
- `add_int(a normie, b normie) normie` - Addition of two integers
- `subtract_int(a normie, b normie) normie` - Subtraction of two integers  
- `multiply_int(a normie, b normie) normie` - Multiplication of two integers
- `divide_int(a normie, b normie) normie` - Division of two integers (with zero check)

#### Float Operations
- `add_float(a meal, b meal) meal` - Addition of two floats
- `subtract_float(a meal, b meal) meal` - Subtraction of two floats
- `multiply_float(a meal, b meal) meal` - Multiplication of two floats
- `divide_float(a meal, b meal) meal` - Division of two floats (with zero check)

### Mathematical Functions

#### Absolute Value
- `abs_int(x normie) normie` - Absolute value of integer
- `abs_float(x meal) meal` - Absolute value of float

#### Min/Max Operations
- `min_int(a normie, b normie) normie` - Minimum of two integers
- `max_int(a normie, b normie) normie` - Maximum of two integers
- `min_float(a meal, b meal) meal` - Minimum of two floats
- `max_float(a meal, b meal) meal` - Maximum of two floats

#### Power Functions
- `power_int(base normie, exp normie) normie` - Integer power with integer exponent
- `power_float(base meal, exp normie) meal` - Float power with integer exponent

#### Advanced Mathematical Functions
- `sqrt_float(x meal) meal` - Square root using Newton's method approximation
- `factorial(n normie) normie` - Factorial of integer n
- `gcd(a normie, b normie) normie` - Greatest Common Divisor
- `lcm(a normie, b normie) normie` - Least Common Multiple

### Validation Functions

#### Integer Validation
- `is_positive_int(x normie) lit` - Check if integer is positive
- `is_negative_int(x normie) lit` - Check if integer is negative
- `is_zero_int(x normie) lit` - Check if integer is zero

#### Float Validation
- `is_positive_float(x meal) lit` - Check if float is positive
- `is_negative_float(x meal) lit` - Check if float is negative
- `is_zero_float(x meal) lit` - Check if float is zero

### Type Conversion
- `int_to_float(x normie) meal` - Convert integer to float
- `float_to_int(x meal) normie` - Convert float to integer (truncation)

### Mathematical Constants
- `PI meal` - Pi constant (3.141592653589793)
- `E meal` - Euler's number (2.718281828459045)

## Usage Examples

```cursed
yeet "math_simple"

# Basic arithmetic
sus sum normie = add_int(5, 3)        # Result: 8
sus product meal = multiply_float(2.5, 4.0)  # Result: 10.0

# Mathematical functions
sus absolute normie = abs_int(-42)    # Result: 42
sus maximum normie = max_int(10, 15)  # Result: 15
sus power_result normie = power_int(2, 8)  # Result: 256

# Validation
sus is_pos lit = is_positive_int(42)  # Result: based (true)
sus is_neg lit = is_negative_float(-3.14)  # Result: based (true)

# Constants
sus circle_area meal = PI * 2.0 * 2.0  # Using PI constant
sus exponential meal = power_float(E, 2)  # Using E constant

# Advanced operations
sus root meal = sqrt_float(25.0)      # Result: ~5.0
sus fact normie = factorial(5)        # Result: 120
sus greatest normie = gcd(12, 18)     # Result: 6
```

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/math_simple/test_math_simple.csd
```

Test both interpretation and compilation modes:
```bash
cargo run --bin cursed stdlib/math_simple/test_math_simple.csd
cargo run --bin cursed -- compile stdlib/math_simple/test_math_simple.csd
./test_math_simple
```

## Implementation Notes

- **Pure CURSED**: All functions implemented using only CURSED language features, no FFI dependencies
- **Type Safety**: Separate functions for integer and float operations maintain type safety
- **Error Handling**: Division operations include basic zero-checking
- **Performance**: Optimized algorithms for common operations like GCD using Euclidean algorithm
- **Precision**: Square root uses Newton's method with configurable precision
- **Testing**: Comprehensive test coverage using testz framework with 25+ test cases

## Module Structure

- `mod.csd` - Main implementation with all mathematical functions
- `test_math_simple.csd` - Comprehensive test suite using testz framework
- `README.md` - This documentation file

## Dependencies

- `testz` - Testing framework for module validation
- No external FFI dependencies - pure CURSED implementation

## Compatibility

- ✅ Interpretation mode
- ✅ Compilation mode  
- ✅ Both-mode verification
- ✅ Enterprise-ready for production use
