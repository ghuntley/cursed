# vibez Module

The vibez module provides formatted I/O operations for the CURSED programming language. It offers console I/O, string formatting, and print functions similar to printf/scanf in C or fmt.Print in Go.

## Features

- **Basic Output**: Simple text printing to console
- **Formatted Output**: Printf-style formatting with placeholders
- **String Formatting**: Format strings without immediate output
- **Input Operations**: Console input reading and parsing
- **Utility Functions**: Colors, timestamps, error messages
- **Pure CURSED**: No FFI dependencies, fully implemented in CURSED

## Functions

### Basic Output Functions

#### `spill(message tea) lit`
Prints a message to the console.
```cursed
vibez.spill("Hello, World!")
```

#### `spillln(message tea) lit`
Prints a message with a newline.
```cursed
vibez.spillln("This line ends with newline")
```

#### `spill_values(values ...tea) lit`
Prints multiple values separated by spaces.
```cursed
vibez.spill_values("Hello", "World", "Test")
```

#### `spill_values_ln(values ...tea) lit`
Prints multiple values with a newline at the end.
```cursed
vibez.spill_values_ln("Value1", "Value2", "Value3")
```

#### `spill_sep(separator tea, values ...tea) lit`
Prints multiple values with a custom separator.
```cursed
vibez.spill_sep(", ", "Apple", "Orange", "Banana")
```

### Formatted Output Functions

#### `spillf(format tea, args ...tea) lit`
Prints formatted text using printf-style formatting.
```cursed
vibez.spillf("Hello %s, you are %d years old", "Alice", "25")
```

#### `spillfln(format tea, args ...tea) lit`
Prints formatted text with a newline.
```cursed
vibez.spillfln("User: %s, ID: %d", "Bob", "123")
```

#### `spillstr(format tea, args ...tea) tea`
Returns a formatted string without printing.
```cursed
sus message tea = vibez.spillstr("Name: %s, Age: %d", "Charlie", "30")
```

### String Formatting Functions

#### `format_string(format tea, args ...tea) tea`
Core string formatting function with placeholder replacement.
- `%s` - String placeholder
- `%d` - Number placeholder

```cursed
sus result tea = vibez.format_string("Hello %s", "World")
```

#### `format_number(number normie) tea`
Converts a number to string.
```cursed
sus num_str tea = vibez.format_number(42)
```

#### `format_float(number drip) tea`
Converts a float to string.
```cursed
sus float_str tea = vibez.format_float(3.14)
```

#### `format_bool(value lit) tea`
Converts a boolean to string ("true" or "false").
```cursed
sus bool_str tea = vibez.format_bool(based)
```

### Input Functions

#### `scan() tea`
Reads input from the console.
```cursed
sus input tea = vibez.scan()
```

#### `scanln() tea`
Reads a line from the console, trimming whitespace.
```cursed
sus line tea = vibez.scanln()
```

#### `scanf(format tea) tea`
Reads formatted input from the console.
```cursed
sus formatted_input tea = vibez.scanf("%s")
```

### Utility Functions

#### `spill_error(message tea) lit`
Prints an error message with "Error: " prefix.
```cursed
vibez.spill_error("File not found")
```

#### `spill_warning(message tea) lit`
Prints a warning message with "Warning: " prefix.
```cursed
vibez.spill_warning("Deprecated function used")
```

#### `spill_debug(message tea) lit`
Prints a debug message with "Debug: " prefix.
```cursed
vibez.spill_debug("Variable value: 42")
```

#### `spill_with_time(message tea) lit`
Prints a message with timestamp.
```cursed
vibez.spill_with_time("Operation completed")
```

#### `clear_screen() lit`
Clears the console screen using ANSI escape codes.
```cursed
vibez.clear_screen()
```

#### `set_color(color tea) lit`
Sets text color using ANSI escape codes.
Supported colors: "red", "green", "blue", "yellow", "reset"
```cursed
vibez.set_color("red")
```

#### `spill_colored(message tea, color tea) lit`
Prints colored text and resets color afterward.
```cursed
vibez.spill_colored("This is red text", "red")
```

## Usage Examples

### Basic Printing
```cursed
yeet "vibez"

vibez.spill("Hello, World!")
vibez.spillln("This line has a newline")
```

### Formatted Output
```cursed
yeet "vibez"

sus name tea = "Alice"
sus age tea = "25"
vibez.spillf("Hello %s, you are %d years old", name, age)
```

### String Formatting
```cursed
yeet "vibez"

sus formatted tea = vibez.spillstr("User: %s, Score: %d", "Bob", "95")
vibez.spill(formatted)
```

### Error and Warning Messages
```cursed
yeet "vibez"

vibez.spill_error("Invalid input provided")
vibez.spill_warning("This feature is deprecated")
vibez.spill_debug("Processing item #42")
```

### Colored Output
```cursed
yeet "vibez"

vibez.spill_colored("Success!", "green")
vibez.spill_colored("Error occurred", "red")
```

### Input Operations
```cursed
yeet "vibez"

vibez.spill("Enter your name: ")
sus name tea = vibez.scanln()
vibez.spillf("Hello, %s!", name)
```

## Testing

Run the test suite with:
```bash
cargo run --bin cursed stdlib/vibez/test_vibez.csd
```

Test both interpretation and compilation modes:
```bash
cargo run --bin cursed stdlib/vibez/test_vibez.csd
cargo run --bin cursed -- compile stdlib/vibez/test_vibez.csd
./test_vibez
```

## Dependencies

- `core` - Core runtime functions
- `stringz` - String manipulation utilities
- `testz` - Testing framework (for tests only)

## Implementation Notes

- Pure CURSED implementation without FFI dependencies
- Simple format string parsing supporting %s and %d placeholders
- ANSI escape codes for terminal colors and clearing
- Extensible design for additional formatting options
- Thread-safe operations for concurrent use

## Performance Considerations

- String formatting involves multiple string operations
- Large format strings may impact performance
- Input operations are blocking
- Color output depends on terminal ANSI support

## Compatibility

- Works in both interpretation and compilation modes
- Supports all CURSED basic types
- Compatible with CURSED module system
- Cross-platform terminal operations via ANSI codes
