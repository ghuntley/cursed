# mathz (Essential Mathematics)

## Overview
`mathz` provides essential mathematical functions and constants for CURSED, serving as the core mathematics package. This module includes fundamental arithmetic operations, mathematical constants, and common utility functions needed for numerical computations. All functions are implemented in pure CURSED without external dependencies.

## Mathematical Constants

```cursed
# Fundamental mathematical constants
fact PI meal = 3.141592653589793
fact E meal = 2.718281828459045
fact TAU meal = 6.283185307179586
```

## Absolute Value Functions

### Integer Absolute Value
```cursed
slay abs_normie(x normie) normie
```
Returns the absolute value of an integer.

**Parameters:**
- `x normie`: The integer value

**Returns:**
- `normie`: The absolute value of x

**Examples:**
```cursed
sus result := abs_normie(-42)  # Returns 42
sus zero := abs_normie(0)      # Returns 0
sus positive := abs_normie(15) # Returns 15
```

### Float Absolute Value
```cursed
slay abs_meal(x meal) meal
```
Returns the absolute value of a floating-point number.

**Parameters:**
- `x meal`: The floating-point value

**Returns:**
- `meal`: The absolute value of x

**Examples:**
```cursed
sus result := abs_meal(-3.14)  # Returns 3.14
sus zero := abs_meal(0.0)      # Returns 0.0
sus positive := abs_meal(2.71) # Returns 2.71
```

## Comparison Functions

### Maximum Functions

#### Integer Maximum
```cursed
slay max_normie(a normie, b normie) normie
```
Returns the larger of two integer values.

**Parameters:**
- `a normie`: First integer value
- `b normie`: Second integer value

**Returns:**
- `normie`: The larger of a and b

#### Float Maximum
```cursed
slay max_meal(a meal, b meal) meal
```
Returns the larger of two floating-point values.

**Parameters:**
- `a meal`: First floating-point value
- `b meal`: Second floating-point value

**Returns:**
- `meal`: The larger of a and b

### Minimum Functions

#### Integer Minimum
```cursed
slay min_normie(a normie, b normie) normie
```
Returns the smaller of two integer values.

**Parameters:**
- `a normie`: First integer value
- `b normie`: Second integer value

**Returns:**
- `normie`: The smaller of a and b

#### Float Minimum
```cursed
slay min_meal(a meal, b meal) meal
```
Returns the smaller of two floating-point values.

**Parameters:**
- `a meal`: First floating-point value
- `b meal`: Second floating-point value

**Returns:**
- `meal`: The smaller of a and b

**Examples:**
```cursed
sus max_int := max_normie(10, 20)    # Returns 20
sus min_int := min_normie(10, 20)    # Returns 10
sus max_float := max_meal(3.14, 2.71) # Returns 3.14
sus min_float := min_meal(3.14, 2.71) # Returns 2.71
```

## Power and Root Functions

### Power Function
```cursed
slay pow_meal(base meal, exp normie) meal
```
Raises a floating-point base to an integer exponent.

**Parameters:**
- `base meal`: The base value
- `exp normie`: The integer exponent (must be non-negative)

**Returns:**
- `meal`: base^exp

**Examples:**
```cursed
sus square := pow_meal(3.0, 2)    # Returns 9.0
sus cube := pow_meal(2.0, 3)      # Returns 8.0
sus identity := pow_meal(5.0, 1)  # Returns 5.0
sus unit := pow_meal(42.0, 0)     # Returns 1.0
```

### Square Root Function
```cursed
slay sqrt_meal(x meal) meal
```
Computes the square root of a floating-point number using Newton's method.

**Parameters:**
- `x meal`: The input value (must be non-negative)

**Returns:**
- `meal`: The square root of x, or 0.0 if x is negative

**Examples:**
```cursed
sus root := sqrt_meal(16.0)  # Returns 4.0
sus precise := sqrt_meal(2.0) # Returns ~1.414213
sus zero := sqrt_meal(0.0)   # Returns 0.0
sus invalid := sqrt_meal(-1.0) # Returns 0.0 (error case)
```

**Implementation Notes:**
- Uses iterative Newton's method with precision of 0.000001
- Handles edge cases: negative inputs return 0.0, zero input returns 0.0
- Convergence typically achieved in 3-5 iterations for most inputs

## Utility Functions

### Sign Testing Functions

#### Positive Test
```cursed
slay is_positive_meal(x meal) lit
```
Tests if a floating-point number is positive.

**Parameters:**
- `x meal`: The value to test

**Returns:**
- `lit`: `based` if valid, `cringe` otherwise

#### Negative Test
```cursed
slay is_negative_meal(x meal) lit
```
Tests if a floating-point number is negative.

**Parameters:**
- `x meal`: The value to test

**Returns:**
- `lit`: `based` if valid, `cringe` otherwise

#### Zero Test
```cursed
slay is_zero_meal(x meal) lit
```
Tests if a floating-point number is exactly zero.

**Parameters:**
- `x meal`: The value to test

**Returns:**
- `lit`: `based` if valid, `cringe` otherwise

**Examples:**
```cursed
sus pos := is_positive_meal(3.14)  # Returns based
sus neg := is_negative_meal(-2.71) # Returns based
sus zero := is_zero_meal(0.0)      # Returns based
sus not_zero := is_zero_meal(0.001) # Returns cap
```

## Type Definitions

### Numeric Types
- `normie`: 32-bit signed integer
- `meal`: 64-bit floating-point number
- `lit`: Boolean type (`based` for true, `cap` for false)

## Error Handling

### Input Validation
- All functions handle edge cases gracefully
- Negative inputs to `sqrt_meal` return 0.0
- Division by zero should be avoided in client code
- No panics or runtime errors are generated

### Precision Considerations
- Square root uses Newton's method with 1e-6 precision
- Floating-point operations follow IEEE 754 standards
- Constants are defined with maximum double precision

## Performance Characteristics

### Time Complexity
- Basic operations (abs, max, min): O(1)
- Power function: O(n) where n is the exponent
- Square root: O(log n) with Newton's method convergence

### Memory Usage
- All functions use constant memory
- No dynamic allocations
- Stack-based computation only

## Usage Patterns

### Common Mathematical Operations
```cursed
yeet "mathz"

# Distance calculation
slay distance(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    sus dx := abs_meal(x2 - x1)
    sus dy := abs_meal(y2 - y1)
    damn sqrt_meal(pow_meal(dx, 2) + pow_meal(dy, 2))
}

# Clamp value to range
slay clamp(value meal, min_val meal, max_val meal) meal {
    damn min_meal(max_meal(value, min_val), max_val)
}

# Check if value is in range
slay in_range(value meal, min_val meal, max_val meal) lit {
    damn value >= min_val && value <= max_val
}
```

### Scientific Computing
```cursed
# Calculate circle area
slay circle_area(radius meal) meal {
    damn PI * pow_meal(radius, 2)
}

# Calculate compound interest
slay compound_interest(principal meal, rate meal, periods normie) meal {
    damn principal * pow_meal(1.0 + rate, periods)
}
```

## Implementation Notes

### Pure CURSED Implementation
- All functions implemented in pure CURSED without FFI
- Compatible with both interpretation and compilation modes
- No external mathematical library dependencies

### Numerical Stability
- Newton's method implementation is numerically stable
- Constants defined with maximum precision
- Proper handling of edge cases and overflow conditions

### Thread Safety
- All functions are pure and thread-safe
- No shared state or global variables
- Safe for concurrent use in goroutines

## Testing Strategy

### Unit Tests
```cursed
yeet "testz"
yeet "mathz"

# Test absolute values
test_start("abs functions")
assert_eq_meal(abs_meal(-3.14), 3.14)
assert_eq_normie(abs_normie(-42), 42)

# Test comparisons
test_start("comparison functions")
assert_eq_normie(max_normie(10, 20), 20)
assert_eq_meal(min_meal(3.14, 2.71), 2.71)

# Test power and roots
test_start("power and root functions")
assert_eq_meal(pow_meal(2.0, 3), 8.0)
assert_true(abs_meal(sqrt_meal(16.0) - 4.0) < 0.001)

print_test_summary()
```

### Integration Tests
- Verify mathematical properties (commutativity, associativity)
- Test edge cases and boundary conditions
- Performance benchmarks for iterative algorithms

## Dependencies

- `core`: Basic types and language primitives
- No external mathematical libraries or FFI dependencies

## Security Considerations

- Input validation prevents invalid operations
- No buffer overflows or memory safety issues
- Deterministic behavior across all platforms
- No timing-dependent vulnerabilities

## Compatibility

### Language Versions
- Compatible with all CURSED language versions
- Uses only core language features
- No version-specific syntax or features

### Platform Support
- Works on all supported CURSED platforms
- Consistent numerical behavior across architectures
- No platform-specific optimizations or code paths
