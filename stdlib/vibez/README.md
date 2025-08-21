# VIBEZ Module - I/O Operations

The `vibez` module is the core I/O library for CURSED programs, providing comprehensive input/output functionality with a focus on ease of use and reliability.

## Overview

The `vibez` module handles:
- Console output (printing text, formatted output)
- User input (reading from stdin)
- File operations (reading and writing files)
- Error handling and validation
- Specialized output formatting

## Installation

Import the module in your CURSED program:

```cursed
yeet "vibez"
```

## Core Functions

### Output Functions

#### `vibez.spill(message)`
Primary output function for printing text to stdout.

```cursed
vibez.spill("Hello, CURSED!")
vibez.spill("Basic text output")
```

**Parameters:**
- `message` (tea): The text to print

**Returns:** `lit` - `based` on success, `cap` on failure

---

#### `vibez.spillln(message)`
Print text with automatic newline character.

```cursed
vibez.spillln("Line with newline")
vibez.spillln("Each call adds newline automatically")
```

**Parameters:**
- `message` (tea): The text to print

**Returns:** `lit` - `based` on success, `cap` on failure

---

#### `vibez.spillf(format, args)`
Formatted print function with placeholder support.

```cursed
vibez.spillf("Hello %s, you are %s years old", ["Alice", "25"])
vibez.spillf("Status: %s", ["Active"])
```

**Parameters:**
- `format` (tea): Format string with `%s` placeholders
- `args` ([]tea): Array of arguments to substitute

**Returns:** `lit` - `based` on success, `cap` on failure

---

#### `vibez.spillstr(format, args)`
Format string without printing (returns formatted string).

```cursed
sus formatted tea = vibez.spillstr("Hello %s", ["World"])
vibez.spill(formatted)  // Prints: Hello World
```

**Parameters:**
- `format` (tea): Format string with `%s` placeholders  
- `args` ([]tea): Array of arguments to substitute

**Returns:** `tea` - Formatted string

---

#### `vibez.spill_values(values)`
Print multiple values separated by spaces.

```cursed
vibez.spill_values(["Value1", "Value2", "Value3"])
// Output: Value1 Value2 Value3
```

**Parameters:**
- `values` ([]tea): Array of strings to print

**Returns:** `lit` - `based` on success, `cap` on failure

---

#### `vibez.spill_sep(separator, values)`
Print values with custom separator.

```cursed
vibez.spill_sep(",", ["Name", "Age", "City"])
// Output: Name,Age,City

vibez.spill_sep(" | ", ["A", "B", "C"])  
// Output: A | B | C
```

**Parameters:**
- `separator` (tea): String to use between values
- `values` ([]tea): Array of strings to print

**Returns:** `lit` - `based` on success, `cap` on failure

### Specialized Output

#### `vibez.spill_error(message)`
Print error message with "ERROR:" prefix.

```cursed
vibez.spill_error("File not found")
// Output: ERROR: File not found
```

#### `vibez.spill_warning(message)`  
Print warning message with "WARNING:" prefix.

```cursed
vibez.spill_warning("Deprecated function used")
// Output: WARNING: Deprecated function used
```

#### `vibez.spill_debug(message)`
Print debug message with "DEBUG:" prefix.

```cursed
vibez.spill_debug("Variable x = 42")
// Output: DEBUG: Variable x = 42
```

### Input Functions

#### `vibez.input(prompt)`
Read user input with optional prompt.

```cursed
sus name tea = vibez.input("Enter your name: ")
sus age tea = vibez.input("")  // No prompt
```

**Parameters:**
- `prompt` (tea): Optional prompt to display before reading input

**Returns:** `tea` - User input string (trimmed of whitespace)

---

#### `vibez.read_line()`
Read a line from stdin without prompt.

```cursed
sus line tea = vibez.read_line()
```

**Returns:** `tea` - Input line as string

### File I/O Functions

#### `vibez.read_file(filename)`
Read entire file contents as string.

```cursed
sus content tea, error tea = vibez.read_file("config.txt")
ready error != "" {
    vibez.spill_error(error)
} otherwise {
    vibez.spill("File content:", content)
}
```

**Parameters:**
- `filename` (tea): Path to file to read

**Returns:** `(tea, tea)` - (file_content, error_message)

---

#### `vibez.write_file(filename, content)`
Write content to file, creating or overwriting as needed.

```cursed
sus success lit, error tea = vibez.write_file("output.txt", "Hello World")
ready error != "" {
    vibez.spill_error(error)  
} otherwise {
    vibez.spill("File written successfully")
}
```

**Parameters:**
- `filename` (tea): Path to file to write
- `content` (tea): Content to write to file

**Returns:** `(lit, tea)` - (success_flag, error_message)

## Error Handling

The `vibez` module provides comprehensive error handling:

### `vibez.get_last_error()`
Get the last error message from I/O operations.

```cursed
sus error tea = vibez.get_last_error()
ready error != "" {
    vibez.spill("Last error:", error)
}
```

### `vibez.clear_error()`
Clear the current error state.

```cursed
vibez.clear_error()
```

## Usage Examples

### Basic Output
```cursed
yeet "vibez"

slay main() {
    vibez.spill("Welcome to CURSED!")
    vibez.spillln("This line has automatic newline")
    
    sus name tea = "Alice"
    sus age normie = 25
    vibez.spillf("Hello %s, age %s", [name, "25"])
}
```

### File Operations
```cursed
yeet "vibez"

slay main() {
    fr fr Write to file
    sus success lit, write_error tea = vibez.write_file("hello.txt", "Hello, CURSED!")
    ready write_error != "" {
        vibez.spill_error("Failed to write file: " + write_error)
        damn
    }
    
    fr fr Read from file
    sus content tea, read_error tea = vibez.read_file("hello.txt")
    ready read_error != "" {
        vibez.spill_error("Failed to read file: " + read_error)
        damn
    }
    
    vibez.spill("File content:", content)
}
```

### User Interaction
```cursed
yeet "vibez"

slay main() {
    sus name tea = vibez.input("Enter your name: ")
    sus age tea = vibez.input("Enter your age: ")
    
    vibez.spillf("Hello %s! You are %s years old.", [name, age])
}
```

### Error Handling Pattern
```cursed
yeet "vibez"

slay safe_file_operation(filename tea) {
    sus content tea, error tea = vibez.read_file(filename)
    ready error != "" {
        vibez.spill_error("File operation failed: " + error)
        damn
    }
    
    vibez.spill("Successfully read:", content)
}
```

## Implementation Notes

- The `vibez` module is implemented in pure CURSED language
- Runtime functions (`runtime_print`, `runtime_read_line`, etc.) are provided by the CURSED runtime system
- All functions include comprehensive error handling and validation
- String formatting supports basic `%s` placeholder substitution
- File operations return both result and error for explicit error handling
- The module automatically initializes when imported

## Performance Considerations

- `spill_values()` and `spill_sep()` are optimized for multiple value output
- `spillstr()` allows string formatting without immediate output for better control
- Error states are maintained globally but can be cleared explicitly
- File I/O operations validate inputs before attempting system calls

## Thread Safety

The `vibez` module maintains minimal global state and is designed to be thread-safe for most operations. File I/O operations are atomic at the system level.
