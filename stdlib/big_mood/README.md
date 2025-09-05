# big_mood: Arbitrary-Precision Arithmetic Module

The `big_mood` module provides arbitrary-precision integer arithmetic operations for CURSED programs. This module implements big integer operations using pure CURSED code without external dependencies.

## Overview

The `big_mood` module represents large integers as arrays of digits, enabling arithmetic operations on numbers larger than the standard integer types. The implementation uses base 10^9 for efficiency while maintaining readability.

## Data Structure

Big integers are represented as arrays of `normie` (32-bit integers) where:
- Elements 0-8: Store digits in base 10^9 (little-endian)
- Element 9: Stores the length (number of significant digits)
- Maximum supported size: 9 digits (up to 10^81 - 1)

## Functions

### Creation and Conversion

#### `bigint_new(value normie) [normie]`
Creates a new big integer from a regular integer value.

**Parameters:**
- `value`: Regular integer to convert

**Returns:** Big integer array

**Example:**
```cursed
sus big_num [10]normie
big_num = bigint_new(12345)
```

#### `bigint_from_string(str tea) [normie]`
Creates a big integer from a string representation.

**Parameters:**
- `str`: String representation of the number

**Returns:** Big integer array

**Example:**
```cursed
sus big_num [10]normie
big_num = bigint_from_string("123456789")
```

#### `bigint_to_string(bigint [normie]) tea`
Converts a big integer to its string representation.

**Parameters:**
- `bigint`: Big integer array

**Returns:** String representation

**Example:**
```cursed
sus result tea
result = bigint_to_string(big_num)
vibez.spill(result)
```

### Arithmetic Operations

#### `bigint_add(a [normie], b [normie]) [normie]`
Adds two big integers.

**Parameters:**
- `a`: First big integer
- `b`: Second big integer

**Returns:** Sum as big integer

**Example:**
```cursed
sus a [10]normie = bigint_new(12345)
sus b [10]normie = bigint_new(67890)
sus sum [10]normie = bigint_add(a, b)
```

#### `bigint_sub(a [normie], b [normie]) [normie]`
Subtracts two big integers (assumes a >= b).

**Parameters:**
- `a`: Minuend (must be >= subtrahend)
- `b`: Subtrahend

**Returns:** Difference as big integer

**Example:**
```cursed
sus a [10]normie = bigint_new(67890)
sus b [10]normie = bigint_new(12345)
sus diff [10]normie = bigint_sub(a, b)
```

#### `bigint_mul(a [normie], b [normie]) [normie]`
Multiplies two big integers.

**Parameters:**
- `a`: First factor
- `b`: Second factor

**Returns:** Product as big integer

**Example:**
```cursed
sus a [10]normie = bigint_new(123)
sus b [10]normie = bigint_new(456)
sus product [10]normie = bigint_mul(a, b)
```

#### `bigint_div(a [normie], b [normie]) [normie]`
Divides two big integers (returns quotient).

**Parameters:**
- `a`: Dividend
- `b`: Divisor (must not be zero)

**Returns:** Quotient as big integer

**Example:**
```cursed
sus dividend [10]normie = bigint_new(12345)
sus divisor [10]normie = bigint_new(123)
sus quotient [10]normie = bigint_div(dividend, divisor)
```

#### `bigint_mod(a [normie], b [normie]) [normie]`
Computes modulo of two big integers.

**Parameters:**
- `a`: Dividend
- `b`: Divisor (must not be zero)

**Returns:** Remainder as big integer

**Example:**
```cursed
sus dividend [10]normie = bigint_new(12345)
sus divisor [10]normie = bigint_new(123)
sus remainder [10]normie = bigint_mod(dividend, divisor)
```

#### `bigint_pow(base [normie], exp normie) [normie]`
Computes base raised to the power of exponent.

**Parameters:**
- `base`: Base as big integer
- `exp`: Exponent as regular integer

**Returns:** Result as big integer

**Example:**
```cursed
sus base [10]normie = bigint_new(2)
sus result [10]normie = bigint_pow(base, 10)  // 2^10 = 1024
```

### Comparison and Utility

#### `bigint_cmp(a [normie], b [normie]) normie`
Compares two big integers.

**Parameters:**
- `a`: First big integer
- `b`: Second big integer

**Returns:** 
- `-1` if a < b
- `0` if a == b
- `1` if a > b

**Example:**
```cursed
sus a [10]normie = bigint_new(123)
sus b [10]normie = bigint_new(456)
sus comparison normie = bigint_cmp(a, b)  // Returns -1
```

#### `bigint_gcd(a [normie], b [normie]) [normie]`
Computes the Greatest Common Divisor using Euclidean algorithm.

**Parameters:**
- `a`: First big integer
- `b`: Second big integer

**Returns:** GCD as big integer

**Example:**
```cursed
sus a [10]normie = bigint_new(48)
sus b [10]normie = bigint_new(18)
sus gcd [10]normie = bigint_gcd(a, b)  // Returns 6
```

#### `bigint_lcm(a [normie], b [normie]) [normie]`
Computes the Least Common Multiple.

**Parameters:**
- `a`: First big integer
- `b`: Second big integer

**Returns:** LCM as big integer

**Example:**
```cursed
sus a [10]normie = bigint_new(12)
sus b [10]normie = bigint_new(18)
sus lcm [10]normie = bigint_lcm(a, b)  // Returns 36
```

## Usage Examples

### Basic Arithmetic
```cursed
yeet "big_mood"

// Create big integers
sus a [10]normie = bigint_new(123456789)
sus b [10]normie = bigint_new(987654321)

// Perform operations
sus sum [10]normie = bigint_add(a, b)
sus product [10]normie = bigint_mul(a, b)
sus quotient [10]normie = bigint_div(b, a)

// Display results
vibez.spill("Sum: ", bigint_to_string(sum))
vibez.spill("Product: ", bigint_to_string(product))
vibez.spill("Quotient: ", bigint_to_string(quotient))
```

### Comparison and Conditional Logic
```cursed
yeet "big_mood"

sus num1 [10]normie = bigint_new(12345)
sus num2 [10]normie = bigint_new(67890)

sus comparison normie = bigint_cmp(num1, num2)

yeet comparison < 0 {
    vibez.spill("num1 is less than num2")
} else yeet comparison > 0 {
    vibez.spill("num1 is greater than num2")
} else {
    vibez.spill("num1 equals num2")
}
```

### Mathematical Operations
```cursed
yeet "big_mood"

sus base [10]normie = bigint_new(2)
sus power_result [10]normie = bigint_pow(base, 20)  // 2^20

sus num1 [10]normie = bigint_new(48)
sus num2 [10]normie = bigint_new(18)
sus gcd_result [10]normie = bigint_gcd(num1, num2)  // GCD(48, 18) = 6
sus lcm_result [10]normie = bigint_lcm(num1, num2)  // LCM(48, 18) = 144

vibez.spill("2^20 = ", bigint_to_string(power_result))
vibez.spill("GCD(48, 18) = ", bigint_to_string(gcd_result))
vibez.spill("LCM(48, 18) = ", bigint_to_string(lcm_result))
```

## Implementation Notes

### Limitations
- Current implementation supports up to 9 digits (10^81 - 1)
- Division and modulo operations use simplified algorithms
- String parsing is placeholder implementation
- No negative number support in this version

### Performance Considerations
- Uses base 10^9 for efficient storage and computation
- Implements exponentiation by squaring for power operations
- Uses Euclidean algorithm for GCD computation
- Optimized for small to medium-sized big integers

### Future Enhancements
- Support for negative numbers
- More efficient division algorithms
- Full string parsing implementation
- Dynamic array sizing for unlimited precision
- Additional mathematical functions (sqrt, factorial, etc.)

## Testing

Run the test suite to verify functionality:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/big_mood/test_big_mood.💀

# Test compilation mode  
cargo run --bin cursed -- compile stdlib/big_mood/test_big_mood.💀
./test_big_mood
```

The test suite includes:
- Basic arithmetic operations
- Comparison functions
- Mathematical utilities (GCD, LCM)
- Edge cases and error conditions
- Large number handling within limits

## Dependencies

- `testz`: Testing framework for comprehensive test coverage
- Pure CURSED implementation (no external FFI dependencies)

## License

This module is part of the CURSED standard library and follows the same license terms as the CURSED programming language.
