# Enhanced VIBEZ Module

The `vibez` module provides comprehensive I/O operations, string manipulation, console formatting, and logging capabilities for CURSED programs.

## Overview

This module serves as the primary interface for:
- Console input/output operations
- File and directory manipulation
- String formatting and processing
- Console color and cursor control
- Logging with different severity levels
- Data validation and conversion

## Core Functions

### Output Functions

#### `spill(message tea) lit`
Basic print function that outputs text to the console.

```cursed
spill("Hello, world!")
```

#### `spill_multi(args ...tea) lit`
Print multiple arguments with automatic spacing.

```cursed
spill_multi("Hello", "world", "from", "CURSED")
// Output: Hello world from CURSED
```

#### `spillln(message tea) lit`
Print with automatic newline.

```cursed
spillln("This line ends with newline")
```

#### `spillf(format tea, args ...tea) lit`
Formatted print with placeholder replacement.

```cursed
spillf("Hello %s, you have %d messages", ["Alice", "5"])
// Output: Hello Alice, you have 5 messages
```

#### `spillfln(format tea, args ...tea) lit`
Formatted print with newline.

### String Formatting

#### `format_string_advanced(format tea, args ...tea) tea`
Advanced string formatting with comprehensive placeholder support.

**Supported placeholders:**
- `%s` - String replacement
- `%d` - Integer replacement  
- `%f` - Float replacement

```cursed
sus formatted tea = format_string_advanced("User: %s, Score: %f", ["Bob", "89.5"])
```

### Input Functions

#### `read_line() tea`
Read a complete line from standard input.

```cursed
sus input tea = read_line()
```

#### `read_line_prompt(prompt tea) tea`
Read line with custom prompt display.

```cursed
sus name tea = read_line_prompt("Enter your name: ")
```

#### `read_int(prompt tea) normie`
Read and validate integer input.

```cursed
sus age normie = read_int("Enter your age: ")
```

#### `read_float(prompt tea) meal`
Read and validate floating-point input.

```cursed
sus price meal = read_float("Enter price: ")
```

#### `read_bool(prompt tea) lit`
Read and parse boolean input.

```cursed
sus confirmed lit = read_bool("Confirm (yes/no): ")
```

### File Operations

#### `read_file_safe(filename tea) (tea, tea)`
Read entire file content with error handling.

```cursed
sus (content, error) = read_file_safe("config.txt")
check error == "" {
    spill("File content: " + content)
} highkey {
    log_error("Failed to read file: " + error)
}
```

#### `write_file_safe(filename tea, content tea) (lit, tea)`
Write content to file with error handling.

```cursed
sus (success, error) = write_file_safe("output.txt", "Hello, file!")
check !success {
    log_error("Write failed: " + error)
}
```

#### `append_file_safe(filename tea, content tea) (lit, tea)`
Append content to existing file.

```cursed
sus (success, error) = append_file_safe("log.txt", "New log entry\n")
```

### Directory Operations

#### `list_directory_safe(path tea) ([]tea, tea)`
List directory contents with error handling.

```cursed
sus (files, error) = list_directory_safe("/home/user")
check error == "" {
    bestie file in files {
        spillln(file)
    }
}
```

#### `create_directory_recursive(path tea) (lit, tea)`
Create directory and all parent directories.

```cursed
sus (success, error) = create_directory_recursive("path/to/nested/dir")
```

### Console Formatting

#### Color Functions

Set text colors using ANSI escape codes:

```cursed
set_text_color("red")
spill("This is red text")
set_text_color("reset")

set_background_color("blue")
spill("Blue background")
```

**Available colors:** black, red, green, yellow, blue, magenta, cyan, white, reset

#### `spill_colored(text tea, color tea) lit`
Print colored text with automatic reset.

```cursed
spill_colored("Success!", "green")
spill_colored("Error occurred", "red")
```

#### `clear_screen() lit`
Clear the console screen.

#### `move_cursor(row normie, col normie) lit`
Move cursor to specific position.

```cursed
move_cursor(10, 5)  // Row 10, Column 5
```

### Logging Functions

#### Log Levels

```cursed
log_error("Something went wrong")        // [ERROR] message in red
log_warning("This is a warning")         // [WARNING] message in yellow  
log_info("Information message")          // [INFO] message in green
log_debug("Debug information")           // [DEBUG] message in cyan
```

#### `log_with_timestamp(level tea, message tea) lit`
Log with timestamp prefix.

```cursed
log_with_timestamp("CUSTOM", "Custom log entry")
// Output: [2024-01-01T12:00:00Z] CUSTOM: Custom log entry
```

### String Utilities

#### `string_length(text tea) normie`
Get string length.

```cursed
sus len normie = string_length("hello")  // Returns 5
```

#### `string_char_at(text tea, index normie) normie`
Get character ASCII code at index.

```cursed
sus char normie = string_char_at("hello", 0)  // Returns 104 ('h')
```

#### `string_contains(text tea, substring tea) lit`
Check if string contains substring.

```cursed
check string_contains("hello world", "world") {
    spill("Found substring!")
}
```

#### `string_to_lower(text tea) tea`
Convert string to lowercase.

```cursed
sus lower tea = string_to_lower("HELLO")  // Returns "hello"
```

#### `string_to_upper(text tea) tea`
Convert string to uppercase.

```cursed
sus upper tea = string_to_upper("hello")  // Returns "HELLO"
```

#### `string_replace_at(text tea, start normie, length normie, replacement tea) tea`
Replace substring at specific position.

```cursed
sus result tea = string_replace_at("hello", 1, 2, "ay")  // Returns "haylo"
```

#### `string_substring_safe(text tea, start normie, end normie) tea`
Extract substring with bounds checking.

```cursed
sus substr tea = string_substring_safe("hello", 1, 4)  // Returns "ell"
```

### Validation Functions

#### `is_numeric_string(text tea) lit`
Check if string represents a valid integer.

```cursed
check is_numeric_string("123") {     // true
    spill("Valid number")
}
check is_numeric_string("-456") {    // true
    spill("Valid negative number")  
}
check is_numeric_string("abc") {     // false
    spill("Not a number")
}
```

#### `is_float_string(text tea) lit`
Check if string represents a valid float.

```cursed
check is_float_string("3.14") {      // true
    spill("Valid float")
}
check is_float_string("-2.5") {      // true
    spill("Valid negative float")
}
check is_float_string("12..3") {     // false
    spill("Invalid float")
}
```

### Conversion Functions

#### `char_to_string(ascii_code normie) tea`
Convert ASCII code to string character.

```cursed
sus char tea = char_to_string(65)  // Returns "A"
```

#### `string_to_int_safe(text tea) normie`
Safe string to integer conversion.

```cursed
sus num normie = string_to_int_safe("42")    // Returns 42
sus invalid normie = string_to_int_safe("abc")  // Returns 0
```

#### `string_to_float_safe(text tea) meal`
Safe string to float conversion.

```cursed
sus pi meal = string_to_float_safe("3.14")    // Returns 3.14
sus invalid meal = string_to_float_safe("xyz")   // Returns 0.0
```

### Error Handling

#### `get_last_error() tea`
Get the last error message from I/O operations.

```cursed
sus error tea = get_last_error()
check error != "" {
    log_error("Last operation failed: " + error)
}
```

#### `clear_last_error() cringe`
Clear the last error state.

```cursed
clear_last_error()
```

## Usage Examples

### Basic I/O

```cursed
yeet "vibez"

spillln("Welcome to CURSED!")
sus name tea = read_line_prompt("Enter your name: ")
spillf("Hello, %s!\n", [name])
```

### File Processing

```cursed
yeet "vibez"

sus (content, error) = read_file_safe("input.txt")
check error == "" {
    sus processed tea = string_to_upper(content)
    sus (success, write_error) = write_file_safe("output.txt", processed)
    check success {
        log_info("File processed successfully")
    }
} highkey {
    log_error("Failed to read input file: " + error)
}
```

### Colorful Logging

```cursed
yeet "vibez"

clear_screen()
move_cursor(1, 1)

spill_colored("=== System Status ===", "cyan")
spillln("")

log_info("System started successfully")
log_warning("Low disk space detected")
log_error("Connection failed")

spillln("")
spill_colored("=== End Status ===", "cyan")
```

### Data Validation

```cursed
yeet "vibez"

bestie based {
    sus input tea = read_line_prompt("Enter a number: ")
    
    check is_numeric_string(input) {
        sus number normie = string_to_int_safe(input)
        spillf("You entered: %d\n", [convert_int_to_string(number)])
        ghosted
    } highkey is_float_string(input) {
        sus number meal = string_to_float_safe(input)
        spillf("You entered: %f\n", [convert_float_to_string(number)])
        ghosted
    } highkey {
        log_error("Invalid number format")
        spill_colored("Please enter a valid number!", "red")
    }
}
```

## Runtime Interface

The vibez module interfaces with the Zig runtime through a set of runtime functions:

- `runtime_print_string()` - Core console output
- `runtime_read_char()` - Character input
- `runtime_string_length()` - String length calculation
- `runtime_file_exists()` - File existence checking
- `runtime_read_file_content()` - File reading
- `runtime_write_file_content()` - File writing
- And many more...

These functions provide the bridge between CURSED code and the underlying system operations implemented in Zig.

## Thread Safety

The vibez module is designed to be thread-safe for concurrent use in CURSED programs with goroutines. All I/O operations include appropriate synchronization.

## Error Handling Philosophy

The enhanced vibez module follows a comprehensive error handling approach:

1. **Safe Functions**: Functions ending in `_safe` return error information
2. **Graceful Degradation**: Invalid operations return safe defaults
3. **Comprehensive Validation**: Input validation before processing
4. **Clear Error Messages**: Descriptive error reporting

## Performance Considerations

- String operations use efficient algorithms
- File I/O includes buffering where appropriate
- Memory allocation is minimized through careful design
- Console operations are optimized for responsiveness

## Dependencies

The vibez module requires:
- CURSED runtime with Zig backend
- Standard system I/O capabilities
- Terminal with ANSI escape code support (for colors)

This module serves as the foundation for all I/O operations in CURSED programs and provides a robust, feature-rich interface for building interactive applications.
