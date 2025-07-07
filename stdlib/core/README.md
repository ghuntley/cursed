# Core Module

The **core** module provides essential built-in functions, type conversions, and utilities that form the foundation of the CURSED programming language. This module is critical for self-hosting and provides the fundamental operations that every CURSED program might need.

## Features

### Type Conversion Functions
- **String ↔ Integer**: Convert between strings and integers with error handling
- **String ↔ Boolean**: Convert between strings and booleans with multiple format support
- **String ↔ Float**: Convert between strings and floating-point numbers
- **Cross-type Conversions**: Comprehensive type conversion utilities

### Mathematical Operations
- **Absolute Values**: `abs()` and `abs_float()` for integers and floats
- **Min/Max Functions**: Find minimum and maximum values for integers and floats
- **Clamping**: Constrain values within specified ranges
- **Comparison**: Three-way comparison functions for ordering

### Error Handling
- **Panic System**: Controlled program termination with error messages
- **Assertions**: Runtime condition checking with automatic failure handling
- **Expectations**: Soft assertions that return boolean results

### Memory Utilities
- **Size Information**: Get size of basic types
- **Type Information**: Runtime type checking and identification
- **Default Values**: Get default/zero values for all types

### String Utilities
- **Length Calculation**: Get string length
- **Character Access**: Access individual characters in strings
- **String Operations**: Concatenation, comparison, substring extraction
- **String Conversion**: Convert characters to strings

### Utility Functions
- **Swapping**: Swap values of integers, floats, and strings
- **Range Checking**: Check if values are within specified ranges
- **Default Values**: Get default values for all basic types

## Usage Examples

### Type Conversions
```cursed
yeet "core"

# String to integer conversion
sus age normie = core.string_to_int("25")
vibez.spill("Age: " + core.int_to_string(age))

# Boolean conversions
sus flag lit = core.string_to_bool("based")
vibez.spill("Flag: " + core.bool_to_string(flag))

# Float conversions
sus pi meal = core.string_to_float("3.14159")
vibez.spill("Pi: " + core.float_to_string(pi))
```

### Mathematical Operations
```cursed
yeet "core"

# Absolute values
sus negative normie = -42
sus positive normie = core.abs(negative)
vibez.spill("Absolute value: " + core.int_to_string(positive))

# Min/Max operations
sus smaller normie = core.min(10, 20)
sus larger normie = core.max(10, 20)

# Clamping values
sus clamped normie = core.clamp(150, 0, 100)  # Result: 100
```

### Error Handling
```cursed
yeet "core"

# Assertions
core.assert(age > 0, "Age must be positive")

# Expectations (soft assertions)
sus valid lit = core.expect(input != "", "Input cannot be empty")
yolo !valid {
    # Handle the error case
    vibez.spill("Invalid input provided")
}

# Panic for critical errors
yolo critical_error {
    core.panic("Critical system failure")
}
```

### String Operations
```cursed
yeet "core"

# String length
sus text tea = "Hello, CURSED!"
sus length normie = core.len_string(text)
vibez.spill("Length: " + core.int_to_string(length))

# String concatenation
sus greeting tea = core.string_concat("Hello, ", "World!")
vibez.spill(greeting)

# String comparison
sus equal lit = core.string_equals("test", "test")
vibez.spill("Equal: " + core.bool_to_string(equal))
```

### Utility Functions
```cursed
yeet "core"

# Swapping values
sus a normie = 10
sus b normie = 20
(a, b) = core.swap_int(a, b)
# Now a = 20, b = 10

# Range checking
sus value normie = 75
sus in_range lit = core.in_range(value, 0, 100)
yolo in_range {
    vibez.spill("Value is valid")
}

# Default values
sus default_num normie = core.default_int()     # 0
sus default_text tea = core.default_string()    # ""
sus default_flag lit = core.default_bool()      # cap
```

### Comparison Functions
```cursed
yeet "core"

# Three-way comparison
sus a normie = 10
sus b normie = 20
sus result normie = core.compare_int(a, b)
# result = -1 (a < b)

yolo result < 0 {
    vibez.spill("a is less than b")
} else yolo result > 0 {
    vibez.spill("a is greater than b")
} else {
    vibez.spill("a equals b")
}
```

## Function Reference

### Type Conversions
- `string_to_int(s tea) normie` - Convert string to integer
- `int_to_string(n normie) tea` - Convert integer to string
- `bool_to_string(b lit) tea` - Convert boolean to string
- `string_to_bool(s tea) lit` - Convert string to boolean
- `float_to_string(f meal) tea` - Convert float to string
- `string_to_float(s tea) meal` - Convert string to float

### Mathematical Operations
- `abs(n normie) normie` - Absolute value of integer
- `abs_float(f meal) meal` - Absolute value of float
- `min(a, b normie) normie` - Minimum of two integers
- `max(a, b normie) normie` - Maximum of two integers
- `min_float(a, b meal) meal` - Minimum of two floats
- `max_float(a, b meal) meal` - Maximum of two floats
- `clamp(value, min_val, max_val normie) normie` - Clamp integer to range
- `clamp_float(value, min_val, max_val meal) meal` - Clamp float to range

### Error Handling
- `panic(message tea)` - Panic with error message
- `assert(condition lit, message tea)` - Assert condition or panic
- `expect(condition lit, message tea) lit` - Soft assertion

### Memory Utilities
- `size_of_int() normie` - Size of integer type
- `size_of_float() normie` - Size of float type
- `size_of_bool() normie` - Size of boolean type
- `type_of_string(value interface{}) tea` - Get type name as string

### String Utilities
- `len_string(s tea) normie` - Get string length
- `char_at(s tea, index normie) sip` - Get character at index
- `substring(s tea, start, end normie) tea` - Extract substring
- `string_char(ch sip) tea` - Convert character to string
- `string_concat(a, b tea) tea` - Concatenate strings
- `string_equals(a, b tea) lit` - Check string equality
- `string_contains(haystack, needle tea) lit` - Check if string contains substring

### Utility Functions
- `swap_int(a, b normie) (normie, normie)` - Swap two integers
- `swap_float(a, b meal) (meal, meal)` - Swap two floats
- `swap_string(a, b tea) (tea, tea)` - Swap two strings
- `compare_int(a, b normie) normie` - Compare integers (-1, 0, 1)
- `compare_float(a, b meal) normie` - Compare floats (-1, 0, 1)
- `compare_string(a, b tea) normie` - Compare strings lexicographically
- `in_range(value, min_val, max_val normie) lit` - Check if integer in range
- `in_range_float(value, min_val, max_val meal) lit` - Check if float in range

### Default Values
- `default_int() normie` - Default integer value (0)
- `default_float() meal` - Default float value (0.0)
- `default_bool() lit` - Default boolean value (cap)
- `default_string() tea` - Default string value ("")
- `default_char() sip` - Default character value ('\0')

## Testing

The core module includes comprehensive tests covering all functions:

```bash
# Run core module tests
cargo run --bin cursed stdlib/core/test_core.csd

# Test in compilation mode
cargo run --bin cursed -- compile stdlib/core/test_core.csd
./test_core
```

## Implementation Notes

### Runtime Dependencies
- Some functions (like `char_at`, `substring`) require runtime support
- Type checking functions need runtime type information
- Array/slice utilities need runtime memory management

### Error Handling
- Type conversions return default values on failure (e.g., 0 for invalid strings)
- String operations handle edge cases gracefully
- Range checking prevents out-of-bounds access

### Performance
- Functions are designed for efficiency with minimal overhead
- String operations use efficient algorithms
- Mathematical operations leverage hardware instructions

### Self-Hosting Support
- All functions are implemented in pure CURSED
- No external dependencies beyond runtime system
- Essential for compiler self-hosting capability

## Best Practices

1. **Error Handling**: Always check return values from conversion functions
2. **Range Validation**: Use `in_range` functions before array access
3. **String Safety**: Use `len_string` before character access
4. **Type Safety**: Use appropriate conversion functions for type safety
5. **Performance**: Use `expect` instead of `assert` for non-critical checks

## Integration with Other Modules

The core module is designed to be used by:
- **All stdlib modules** - For basic type conversions and utilities
- **Application code** - For fundamental operations
- **Compiler internals** - For self-hosting support
- **Runtime system** - For error handling and memory management

This module forms the foundation of the CURSED standard library and is essential for the language's self-hosting capability.
