# Simple I/O Module (io_simple)

A minimal, pure CURSED I/O module providing essential console input/output operations without external dependencies.

## Overview

The `io_simple` module provides basic I/O operations for console interaction, including printing various data types, reading user input, and parsing input strings. This module is designed as a lightweight alternative to the full `io` module, perfect for simple programs and testing scenarios.

## Features

- **Console Output**: Print strings, integers, floats, and booleans
- **Console Input**: Read lines, characters, and typed data from console
- **Input Parsing**: Convert string input to integers, floats, and booleans
- **Pure CURSED**: No FFI dependencies or external libraries
- **Testing-Friendly**: Includes simulated input for testing scenarios

## API Reference

### Console Output Functions

#### `io_print(message tea) lit`
Print a message to console without newline.

```cursed
yeet "io_simple"

io_print("Hello, ")
io_print("CURSED!")
# Outputs: Hello, CURSED!
```

#### `io_println(message tea) lit`
Print a message to console with newline.

```cursed
yeet "io_simple"

io_println("First line")
io_println("Second line")
# Outputs:
# First line
# Second line
```

#### `io_print_int(value normie) lit`
Print an integer value to console.

```cursed
yeet "io_simple"

io_print_int(42)
# Outputs: [INTEGER]
```

#### `io_print_float(value meal) lit`
Print a float value to console.

```cursed
yeet "io_simple"

io_print_float(3.14)
# Outputs: [FLOAT]
```

#### `io_print_bool(value lit) lit`
Print a boolean value to console.

```cursed
yeet "io_simple"

io_print_bool(based)  # Outputs: based
io_print_bool(cap)    # Outputs: cap
```

#### `io_printf(format tea, args []tea) lit`
Simple formatted printing (basic implementation).

```cursed
yeet "io_simple"

io_printf("Hello, %s!", ["CURSED"])
# Outputs: Hello, %s!
```

### Console Input Functions

#### `io_read_line() tea`
Read a complete line from console input.

```cursed
yeet "io_simple"

sus input tea = io_read_line()
io_println("You entered: " + input)
# Returns: "simulated user input line" (in testing mode)
```

#### `io_read_char() tea`
Read a single character from console input.

```cursed
yeet "io_simple"

sus char tea = io_read_char()
io_println("First character: " + char)
# Returns: "s" (in testing mode)
```

#### `io_read_int() normie`
Read and parse an integer from console input.

```cursed
yeet "io_simple"

sus number normie = io_read_int()
io_print_int(number)
# Returns: 42 (in testing mode)
```

#### `io_read_float() meal`
Read and parse a float from console input.

```cursed
yeet "io_simple"

sus decimal meal = io_read_float()
io_print_float(decimal)
# Returns: 3.14 (in testing mode)
```

### Input Parsing Functions

#### `io_parse_int(str tea) normie`
Parse a string into an integer.

```cursed
yeet "io_simple"

sus num1 normie = io_parse_int("42")    # Returns: 42
sus num2 normie = io_parse_int("-5")    # Returns: -5
sus num3 normie = io_parse_int("invalid") # Returns: 0
```

#### `io_parse_float(str tea) meal`
Parse a string into a float.

```cursed
yeet "io_simple"

sus float1 meal = io_parse_float("3.14")   # Returns: 3.14
sus float2 meal = io_parse_float("-1.5")   # Returns: -1.5
sus float3 meal = io_parse_float("invalid") # Returns: 0.0
```

#### `io_parse_bool(str tea) lit`
Parse a string into a boolean.

```cursed
yeet "io_simple"

sus bool1 lit = io_parse_bool("based")  # Returns: based
sus bool2 lit = io_parse_bool("true")   # Returns: based
sus bool3 lit = io_parse_bool("cap")    # Returns: cap
sus bool4 lit = io_parse_bool("false")  # Returns: cap
```

### Utility Functions

#### `io_get_current_timestamp() normie`
Get current timestamp (placeholder implementation).

```cursed
yeet "io_simple"

sus timestamp normie = io_get_current_timestamp()
# Returns: 1640995200 (placeholder value)
```

## Usage Examples

### Basic Console Interaction

```cursed
yeet "io_simple"

slay interactive_demo() lit {
    io_println("Welcome to CURSED!")
    io_print("Enter your name: ")
    
    sus name tea = io_read_line()
    io_println("Hello, " + name + "!")
    
    io_print("Enter your age: ")
    sus age normie = io_read_int()
    io_print("You are ")
    io_print_int(age)
    io_println(" years old.")
    
    damn based
}

interactive_demo()
```

### Input Validation

```cursed
yeet "io_simple"

slay validate_input() lit {
    io_println("Testing input validation:")
    
    # Test integer parsing
    sus valid_int normie = io_parse_int("42")
    sus invalid_int normie = io_parse_int("not_a_number")
    
    io_print("Valid int: ")
    io_print_int(valid_int)
    io_println("")
    
    # Test boolean parsing
    sus valid_bool lit = io_parse_bool("based")
    sus invalid_bool lit = io_parse_bool("maybe")
    
    io_print("Valid bool: ")
    io_print_bool(valid_bool)
    io_println("")
    
    damn based
}

validate_input()
```

### Simple Calculator

```cursed
yeet "io_simple"

slay simple_calculator() lit {
    io_println("Simple Calculator")
    io_println("================")
    
    io_print("Enter first number: ")
    sus num1 normie = io_read_int()
    
    io_print("Enter second number: ")
    sus num2 normie = io_read_int()
    
    sus sum normie = num1 + num2
    
    io_print("Result: ")
    io_print_int(sum)
    io_println("")
    
    damn based
}

simple_calculator()
```

## Testing

The module includes comprehensive test coverage:

```bash
# Run module tests
./cursed-unified stdlib/io_simple/test_io_simple.csd

# Test individual functions
echo 'yeet "testz"
yeet "io_simple"

test_start("io_simple basic functions")
io_println("Testing io_simple...")
sus result tea = io_read_line()
assert_eq_string(result, "simulated user input line")
print_test_summary()' > test_basic.csd

./cursed-unified test_basic.csd
```

## Implementation Notes

### Testing Mode
The module includes simulated input functions for testing scenarios:
- `io_read_line()` returns `"simulated user input line"`
- `io_read_int()` returns `42`
- `io_read_float()` returns `3.14`
- `io_read_char()` returns `"s"`

### Production Deployment
In production environments, these functions would interface directly with the CURSED runtime's I/O subsystem for real console interaction.

### Error Handling
- Invalid integer strings default to `0`
- Invalid float strings default to `0.0`
- Invalid boolean strings default to `cap` (false)

## Dependencies

- **None** - Pure CURSED implementation
- **Runtime Integration** - Uses `vibez.spill()` for output

## Related Modules

- [`io`](../io/README.md) - Full-featured I/O module
- [`fmt`](../fmt/README.md) - Advanced string formatting
- [`testz`](../testz/README.md) - Testing framework

## Best Practices

1. **Input Validation**: Always validate parsed input before use
2. **Error Handling**: Check for default values when parsing fails
3. **Testing**: Use simulated input for unit tests
4. **Production**: Replace with full `io` module for real applications

## Version History

- **v1.0.0** - Initial pure CURSED implementation with basic I/O operations

## Contributing

When contributing to `io_simple`:

1. Maintain pure CURSED implementation (no FFI)
2. Keep functions simple and focused
3. Include comprehensive test coverage
4. Update documentation for new functions
5. Ensure simulated input works for testing scenarios

## License

Part of the CURSED programming language stdlib - see main project license.
