# VIBEZ Module - I/O Operations

Essential I/O operations for CURSED programs.

## Functions

### Core Output
- `spill(msg tea)` - Print a message
- `spill_two(msg1 tea, msg2 tea)` - Print two messages
- `spill_three(msg1, msg2, msg3 tea)` - Print three messages
- `spillln(msg tea)` - Print with newline

### Formatting
- `print_header(title tea)` - Print formatted header
- `print_separator()` - Print separator line
- `print_success(msg tea)` - Print success message with ✅
- `print_error(msg tea)` - Print error message with ❌
- `print_warning(msg tea)` - Print warning with ⚠️
- `print_info(msg tea)` - Print info with ℹ️

### Numbered Output
- `print_numbered_item(number drip, item tea)` - Print numbered list item
- `print_result(label tea, value tea)` - Print label: value format

### Debug
- `debug_print(msg tea)` - Print debug message
- `trace_print(function_name tea, msg tea)` - Print trace message

## Usage

```cursed
yeet "vibez"

print_header("My Program")
spill("Hello, World!")
print_success("Program running")
print_numbered_item(1, "First item")
print_result("Status", "OK")
```
