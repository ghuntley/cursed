# Vibez Module (fmt)

The `vibez` module provides formatted I/O functions for the CURSED programming language, equivalent to Go's `fmt` package.

## Core Functions

### Basic Output Functions

- `spill(msg tea)` - Print string to output
- `spill_int(value normie)` - Print integer to output
- `spill_float(value meal)` - Print float to output
- `spill_bool(value lit)` - Print boolean to output
- `spill_char(value sip)` - Print character to output

### Print with Newline

- `println(msg tea)` - Print string with newline
- `println_int(value normie)` - Print integer with newline
- `println_float(value meal)` - Print float with newline
- `println_bool(value lit)` - Print boolean with newline
- `println_char(value sip)` - Print character with newline

### Formatted Output (Specification Functions)

- `spillf(format tea, args ...tea)` - Formatted print (like Printf)
- `spillstr(format tea, args ...tea) tea` - Return formatted string (like Sprintf)
- `scan(format tea, args ...tea) normie` - Scan input into variables
- `scanln(format tea, args ...tea) normie` - Scan line input into variables

### String Formatting

- `format_int(value normie) tea` - Convert integer to string
- `format_float(value meal) tea` - Convert float to string
- `format_bool(value lit) tea` - Convert boolean to string
- `format_char(value sip) tea` - Convert character to string

### Advanced Formatting

- `sprintf(format_str tea, args ...tea) tea` - Format string with placeholders
- `printf(format_str tea, args ...tea)` - Print formatted string
- `printfln(format_str tea, args ...tea)` - Print formatted string with newline

## Usage Examples

```cursed
yeet "vibez"

slay main() {
    // Basic output
    vibez.spill("Hello, World!")
    vibez.spill_int(42)
    vibez.spill_bool(based)
    
    // Formatted output
    vibez.spillf("Number: %d", [42])
    
    // String formatting
    sus formatted tea = vibez.spillstr("Value: %d", [123])
    
    // Advanced formatting
    vibez.printf("Hello, {}!", ["World"])
    
    // Debug output
    vibez.debug_print("Debug message")
    vibez.error_print("Error occurred")
}
```

## Testing

Run the test suite with:

```bash
cargo run --bin cursed stdlib/vibez/test_vibez.csd
```

## Implementation Details

- Pure CURSED implementation without FFI dependencies
- Supports both interpretation and compilation modes
- Includes color output support with ANSI escape codes
- Provides debug and logging functions
- Includes utility functions for padding and text formatting

## Color Support

The module includes color output functions:

- `color_red(text tea) tea` - Red colored text
- `color_green(text tea) tea` - Green colored text
- `color_blue(text tea) tea` - Blue colored text
- `color_yellow(text tea) tea` - Yellow colored text
- `color_magenta(text tea) tea` - Magenta colored text
- `color_cyan(text tea) tea` - Cyan colored text

## Debug Functions

- `debug_print(msg tea)` - Debug output with [DEBUG] prefix
- `info_print(msg tea)` - Info output with [INFO] prefix
- `error_print(msg tea)` - Error output with [ERROR] prefix
- `warning_print(msg tea)` - Warning output with [WARNING] prefix
- `success_print(msg tea)` - Success output with [SUCCESS] prefix in green
