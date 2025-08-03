# vibez Module - CURSED I/O Operations

The `vibez` module provides formatted I/O functions for the CURSED programming language, equivalent to Go's `fmt` package.

## Core Functions

### Basic Output

- `spill(message tea)` - Print message to console
- `spillf(format tea, args ...tea)` - Formatted print with placeholders
- `spillstr(format tea, args ...tea)` - Return formatted string
- `spillln(message tea)` - Print message with newline
- `spillfln(format tea, args ...tea)` - Formatted print with newline

### Advanced Output

- `spill_values(values ...tea)` - Print multiple values with spaces
- `spill_sep(separator tea, values ...tea)` - Print values with custom separator
- `spill_error(message tea)` - Print error message to stderr
- `spill_warning(message tea)` - Print warning message
- `spill_debug(message tea)` - Print debug message
- `spill_colored(message tea, color tea)` - Print colored text

### Input Functions

- `scan()` - Read input until whitespace
- `scanln()` - Read full line from console
- `scanf(format tea)` - Formatted input scanning

### Formatting Functions

- `format_string_enhanced(format tea, args ...tea)` - Enhanced string formatting
- `format_number(num normie)` - Format integer to string
- `format_float(value meal)` - Format float to string
- `format_bool(value lit)` - Format boolean to string

### Console Control

- `clear_screen()` - Clear console screen
- `set_color(color tea)` - Set text color (red, green, blue, reset)

## Format Specifiers

- `%s` - String placeholder
- `%d` - Integer placeholder
- `%f` - Float placeholder

## Example Usage

```cursed
yeet "vibez"

slay main() {
    vibez.spill("Hello, World!")
    
    name := "bestie"
    vibez.spillf("Hey %s, what's good?", name)
    
    sus output tea = vibez.spillstr("Value: %d", "42")
    vibez.spill(output)
    
    vibez.spill_colored("Success!", "green")
}
```

## Implementation Details

- Pure CURSED implementation with runtime interface functions
- Enhanced placeholder parsing for common formatting patterns
- ANSI escape code support for colored output
- Thread-safe output operations
- Input validation and error handling

## Testing

Run tests with:
```bash
cargo run --bin cursed stdlib/vibez/test_vibez.csd
```

The test suite covers all major functions including formatting, input/output, and console control features.
