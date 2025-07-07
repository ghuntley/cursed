# VIBEZ Module - Formatted I/O Operations

The `vibez` module provides comprehensive formatted input/output operations for the CURSED programming language. It serves as the core module for all output, formatting, and display operations.

## Overview

The `vibez` module is essential for:
- Basic output operations (`spill`, `println`)
- String formatting and conversion
- Type-safe formatted output
- Debug and development utilities
- Colored terminal output
- Table and structured formatting

## Core Functions

### Basic Output Functions

```cursed
// Basic output
vibez.spill("Hello, World!")
vibez.spill_int(42)
vibez.spill_float(3.14159)
vibez.spill_bool(based)
vibez.spill_char('X')

// Output with newlines
vibez.println("This is a line")
vibez.println_int(123)
vibez.println_float(2.718)
vibez.println_bool(cap)
vibez.println_char('Y')
```

### String Formatting

```cursed
// Convert values to strings
sus int_str tea = vibez.format_int(42)          // "42"
sus float_str tea = vibez.format_float(3.14)    // "3.14"
sus bool_str tea = vibez.format_bool(based)     // "based"
sus char_str tea = vibez.format_char('Z')       // "Z"
```

### Advanced Formatting

```cursed
// String formatting with placeholders
sus result tea = vibez.sprintf("Hello, {}!", ["World"])
// Result: "Hello, World!"

sus multi tea = vibez.sprintf("{0} + {1} = {2}", ["2", "3", "5"])
// Result: "2 + 3 = 5"

// Formatted output
vibez.printf("The answer is {}\n", ["42"])
vibez.printfln("Formatted line: {}", ["complete"])
```

### Type-Safe Formatting

```cursed
// Format values with explicit types
sus int_result tea = vibez.format_with_type(42, "int")
sus float_result tea = vibez.format_with_type(123, "float")
sus bool_result tea = vibez.format_with_type(1, "bool")

// Output with type specification
vibez.spill_with_type(42, "int")
```

## Debug and Development

```cursed
// Debug output
vibez.debug_print("This is a debug message")
vibez.debug_print_int("counter", 100)
vibez.debug_print_float("pi", 3.14159)
vibez.debug_print_bool("flag", based)

// Message levels
vibez.info_print("Information message")
vibez.error_print("Error message")
vibez.warning_print("Warning message")
```

## Utility Functions

### String Manipulation

```cursed
// Repeat characters
sus stars tea = vibez.repeat_char('*', 5)       // "*****"

// Padding
sus padded_left tea = vibez.pad_left("test", 8, '0')    // "0000test"
sus padded_right tea = vibez.pad_right("test", 8, '-')  // "test----"

// Centering
sus centered tea = vibez.center_text("hi", 6, ' ')      // "  hi  "
```

### Number Formatting

```cursed
// Padded integers
sus padded_int tea = vibez.format_int_padded(42, 5)     // "00042"

// Float precision
sus float_precision tea = vibez.format_float_precision(3.14159, 2)

// Percentage formatting
sus percentage tea = vibez.format_percentage(0.75)      // "75.0%"
```

## Formatted Output

### Table Formatting

```cursed
// Print separator line
vibez.print_separator(30, '-')

// Print header
vibez.print_header("System Status", 40)

// Print table row
sus columns []tea = ["Column1", "Column2", "Column3"]
vibez.print_row(columns, 30)
```

### Example Table Output

```cursed
slay print_status_table() {
    vibez.print_header("System Status", 40)
    
    sus columns []tea = ["Component", "Status", "Version"]
    vibez.print_row(columns, 40)
    vibez.print_separator(40, '-')
    
    sus row1 []tea = ["Compiler", "Ready", "1.0.0"]
    sus row2 []tea = ["Runtime", "Ready", "1.0.0"]
    sus row3 []tea = ["Stdlib", "Ready", "1.0.0"]
    
    vibez.print_row(row1, 40)
    vibez.print_row(row2, 40)
    vibez.print_row(row3, 40)
    
    vibez.print_separator(40, '=')
}
```

## Color Output

```cursed
// Colored text
sus red_text tea = vibez.color_red("Error message")
sus green_text tea = vibez.color_green("Success message")
sus blue_text tea = vibez.color_blue("Info message")
sus yellow_text tea = vibez.color_yellow("Warning message")

// Colored messages
vibez.success_print("Operation completed successfully")
vibez.error_print_colored("This is an error")
vibez.warning_print_colored("This is a warning")
vibez.info_print_colored("This is information")
```

## Usage Examples

### Basic Usage

```cursed
yeet "vibez"

slay main() {
    vibez.println("Welcome to CURSED!")
    vibez.spill("Enter your name: ")
    
    // Format and display information
    sus name tea = "Developer"
    sus age normie = 25
    sus experience meal = 3.5
    
    vibez.printfln("Name: {}", [name])
    vibez.printf("Age: {}, Experience: {} years\n", [
        vibez.format_int(age), 
        vibez.format_float(experience)
    ])
}
```

### Debug Output

```cursed
yeet "vibez"

slay debug_example() {
    sus counter normie = 0
    sus pi meal = 3.14159
    sus ready lit = based
    
    vibez.debug_print("Starting calculations...")
    vibez.debug_print_int("counter", counter)
    vibez.debug_print_float("pi", pi)
    vibez.debug_print_bool("ready", ready)
    
    vibez.info_print("Debug session completed")
}
```

### Formatted Reports

```cursed
yeet "vibez"

slay generate_report() {
    vibez.print_header("Performance Report", 50)
    
    sus metrics []tea = ["CPU Usage", "Memory", "Disk I/O"]
    sus values []tea = ["45%", "2.1GB", "150MB/s"]
    
    vibez.print_row(metrics, 50)
    vibez.print_separator(50, '-')
    vibez.print_row(values, 50)
    
    vibez.print_separator(50, '=')
    vibez.success_print("Report generated successfully")
}
```

## Implementation Details

### Core Primitives

The module relies on these core primitives:
- `builtin_print(msg)` - Basic string output
- `builtin_print_int(value)` - Integer output
- `builtin_print_float(value)` - Float output
- `builtin_print_bool(value)` - Boolean output
- `builtin_print_char(value)` - Character output

### String Conversion

Built-in conversion functions:
- `builtin_int_to_string(value)` - Integer to string
- `builtin_float_to_string(value)` - Float to string
- `builtin_char_to_string(value)` - Character to string

### Dependencies

The module depends on:
- `string` module for string manipulation functions
- Core type system for type conversions
- Runtime support for output operations

## Testing

The module includes comprehensive tests covering:
- Basic output operations
- String formatting functions
- Advanced formatting features
- Type-safe operations
- Debug utilities
- Color output
- Integration scenarios

Run tests with:
```bash
cargo run --bin cursed stdlib/vibez/test_vibez.csd
```

## Integration with CURSED

The `vibez` module is designed to be the primary output interface for CURSED programs. It provides:

1. **Type Safety**: Functions for each CURSED type
2. **Consistency**: Uniform API across all output operations
3. **Flexibility**: From basic output to complex formatting
4. **Debugging**: Comprehensive debug and development tools
5. **Presentation**: Professional output formatting capabilities

## Performance Considerations

- String concatenation is optimized for formatting operations
- Color codes are applied only when needed
- Debug functions can be conditionally compiled
- Formatting functions minimize memory allocations

## Future Enhancements

Planned improvements include:
- File output operations
- Binary data formatting
- Custom format specifiers
- Localization support
- Performance optimizations
- Stream-based I/O

## Compatibility

The `vibez` module is compatible with:
- Both interpretation and compilation modes
- All CURSED data types
- Cross-platform terminal output
- ANSI color code support

This module is essential for self-hosting and provides the foundation for all formatted output operations in the CURSED programming language.
