# vibez - Core I/O Operations

## Overview

The `vibez` module provides essential input/output operations for CURSED programs, including console output, string formatting, and basic printing functionality. This module is the foundation for most CURSED programs and provides clean, intuitive APIs for common I/O tasks.

## Quick Start

```cursed
yeet "vibez"

vibez.spill("Hello, CURSED!")
vibez.spill("Number:", 42)
vibez.spill("Boolean:", based)
```

## API Reference

### Core Functions

#### `spill(...args)`
Prints values to stdout with automatic formatting and newline.

**Parameters:**
- `...args`: Variable arguments of any type

**Example:**
```cursed
vibez.spill("Hello")                    // "Hello"
vibez.spill("Value:", 42)               // "Value: 42" 
vibez.spill("Multiple", "values", 123)  // "Multiple values 123"
```

#### `spill_no_newline(...args)`
Prints values to stdout without adding a newline.

**Example:**
```cursed
vibez.spill_no_newline("Loading")
vibez.spill_no_newline(".")
vibez.spill_no_newline(".")
vibez.spill("Done!")  // Output: "Loading..Done!"
```

#### `spill_error(...args)`
Prints error messages to stderr with red coloring (if terminal supports it).

**Example:**
```cursed
vibez.spill_error("Error: File not found")
vibez.spill_error("Failed to connect:", error_code)
```

#### `format(template tea, ...args) tea`
Formats a string template with provided arguments.

**Parameters:**
- `template`: Format string with `{}` placeholders
- `...args`: Values to substitute

**Returns:** Formatted string

**Example:**
```cursed
sus message tea = vibez.format("Hello {}, you are {} years old", "Alice", 25)
// message = "Hello Alice, you are 25 years old"

sus json tea = vibez.format(`{"name": "{}", "age": {}}`, "Bob", 30)
// json = `{"name": "Bob", "age": 30}`
```

### Advanced Formatting

#### `format_number(value drip, precision drip) tea`
Formats numbers with specific precision.

**Example:**
```cursed
sus pi_formatted tea = vibez.format_number(3.14159, 2)  // "3.14"
sus large_num tea = vibez.format_number(1000000, 0)     // "1,000,000"
```

#### `format_bytes(bytes drip) tea`
Formats byte counts in human-readable format.

**Example:**
```cursed
vibez.spill(vibez.format_bytes(1024))      // "1.0 KB"
vibez.spill(vibez.format_bytes(1048576))   // "1.0 MB"
vibez.spill(vibez.format_bytes(1073741824)) // "1.0 GB"
```

### Input Operations

#### `read_line() tea`
Reads a line from stdin, blocking until input is available.

**Returns:** String input (without newline)

**Example:**
```cursed
vibez.spill("Enter your name: ")
sus name tea = vibez.read_line()
vibez.spill("Hello,", name)
```

#### `read_char() tea`
Reads a single character from stdin.

**Returns:** Single character as string

**Example:**
```cursed
vibez.spill("Press any key to continue...")
sus key tea = vibez.read_char()
vibez.spill("You pressed:", key)
```

### Color and Style Support

#### `color(text tea, color_name tea) tea`
Applies color to text for terminal output.

**Available Colors:**
- `"red"`, `"green"`, `"blue"`, `"yellow"`, `"cyan"`, `"magenta"`
- `"bright_red"`, `"bright_green"`, etc.
- `"reset"` to clear formatting

**Example:**
```cursed
vibez.spill(vibez.color("Success!", "green"))
vibez.spill(vibez.color("Warning:", "yellow") + " Check your input")
vibez.spill(vibez.color("Error:", "red") + " Operation failed")
```

#### `bold(text tea) tea`
Makes text bold (if terminal supports it).

**Example:**
```cursed
vibez.spill(vibez.bold("Important:") + " This is a critical message")
```

## Usage Patterns

### Structured Logging
```cursed
yeet "vibez"
yeet "timez"

slay log_info(message tea) {
    sus timestamp tea = timez.now().format("15:04:05")
    vibez.spill(
        vibez.color("[INFO]", "blue"),
        timestamp,
        message
    )
}

slay log_error(message tea) {
    sus timestamp tea = timez.now().format("15:04:05")
    vibez.spill_error(
        vibez.color("[ERROR]", "red"),
        timestamp,
        message
    )
}

// Usage
log_info("Application started")
log_error("Database connection failed")
```

### Progress Indicators
```cursed
yeet "vibez"

slay show_progress(current drip, total drip) {
    sus percentage drip = (current * 100) / total
    sus bar tea = repeat("█", percentage / 2) + repeat("░", 50 - (percentage / 2))
    
    vibez.spill_no_newline("\r[" + bar + "] " + percentage.(tea) + "%")
    
    ready (current == total) {
        vibez.spill(" Complete!")
    }
}

// Usage
bestie (sus i drip = 0; i <= 100; i++) {
    show_progress(i, 100)
    sleep(50) // Simulate work
}
```

### Interactive Menus
```cursed
yeet "vibez"

slay show_menu() drip {
    vibez.spill(vibez.bold("Main Menu"))
    vibez.spill("1. Start Game")
    vibez.spill("2. Settings")
    vibez.spill("3. Exit")
    vibez.spill_no_newline("Choose option (1-3): ")
    
    sus choice tea = vibez.read_line()
    damn choice.parse_int() fam {
        when _ -> damn 0  // Invalid input
    }
}
```

### Data Presentation
```cursed
yeet "vibez"

slay print_table(headers []tea, rows [][]tea) {
    // Print header
    bestie (sus header tea : headers) {
        vibez.spill_no_newline(vibez.bold(header.pad_right(15)))
    }
    vibez.spill("")
    
    // Print separator
    bestie (_ : headers) {
        vibez.spill_no_newline("─".repeat(15))
    }
    vibez.spill("")
    
    // Print rows
    bestie (sus row []tea : rows) {
        bestie (sus cell tea : row) {
            vibez.spill_no_newline(cell.pad_right(15))
        }
        vibez.spill("")
    }
}

// Usage
sus headers []tea = ["Name", "Age", "City"]
sus data [][]tea = [
    ["Alice", "25", "New York"],
    ["Bob", "30", "London"],
    ["Carol", "22", "Tokyo"]
]

print_table(headers, data)
```

## Performance Characteristics

### Output Performance
- **Console Output**: Optimized for minimal latency (~1μs per call)
- **String Formatting**: Zero-allocation formatting where possible
- **Color Support**: Automatic terminal capability detection

### Memory Usage
- **Stack Allocation**: Most operations use stack-allocated buffers
- **String Interning**: Common format strings are cached
- **Buffer Reuse**: Output buffers are reused across calls

### Benchmarks
```cursed
yeet "vibez"
yeet "testz"

slay benchmark_output() {
    sus start drip = get_time_microseconds()
    
    bestie (sus i drip = 0; i < 10000; i++) {
        vibez.spill("Benchmark output", i)
    }
    
    sus elapsed drip = get_time_microseconds() - start
    vibez.spill("10,000 outputs in", elapsed, "μs")
    vibez.spill("Average:", elapsed / 10000, "μs per output")
}
```

## Error Handling

### Input Validation
```cursed
slay safe_read_number() yikes<drip> {
    sus input tea = vibez.read_line()
    
    ready (input.is_empty()) {
        yikes "empty input"
    }
    
    damn input.parse_int() fam {
        when "invalid format" -> yikes "not a valid number"
        when _ -> yikes "parsing error"
    }
}

// Usage
sus number drip = safe_read_number() fam {
    when "empty input" -> {
        vibez.spill_error("Please enter a number")
        damn 0
    }
    when "not a valid number" -> {
        vibez.spill_error("Invalid number format")
        damn 0
    }
    when _ -> {
        vibez.spill_error("Unexpected error")
        damn 0
    }
}
```

### Output Error Recovery
```cursed
slay safe_output(message tea) lit {
    vibez.spill(message) fam {
        when "broken pipe" -> {
            // Output destination closed
            damn false
        }
        when "permission denied" -> {
            // No write permission
            damn false
        }
        when _ -> damn false
    }
    damn based  // Success
}
```

## Testing

### Unit Tests
```cursed
// stdlib/vibez/test_vibez.💀
yeet "testz"
yeet "vibez"

slay test_format_basic() {
    sus result tea = vibez.format("Hello {}", "world")
    testz.assert_eq_string(result, "Hello world")
}

slay test_format_multiple() {
    sus result tea = vibez.format("{} + {} = {}", 2, 3, 5)
    testz.assert_eq_string(result, "2 + 3 = 5")
}

slay test_format_bytes() {
    testz.assert_eq_string(vibez.format_bytes(1024), "1.0 KB")
    testz.assert_eq_string(vibez.format_bytes(1048576), "1.0 MB")
    testz.assert_eq_string(vibez.format_bytes(1073741824), "1.0 GB")
}

slay test_color_codes() {
    sus red_text tea = vibez.color("error", "red")
    testz.assert_true(red_text.contains("\x1b[31m"))  // ANSI red code
}

slay main() {
    testz.start_suite("vibez Tests")
    test_format_basic()
    test_format_multiple()
    test_format_bytes()
    test_color_codes()
    testz.print_summary()
}
```

### Integration Tests
```bash
# Test interactive input/output
echo "test input" | ./zig-out/bin/cursed-zig stdlib/vibez/interactive_test.💀

# Test color output
./zig-out/bin/cursed-zig stdlib/vibez/color_test.💀 | less -R

# Performance benchmarks
./zig-out/bin/cursed-zig stdlib/vibez/benchmark_test.💀
```

## Platform Support

### Terminal Compatibility
- **ANSI Colors**: Full support on Unix terminals
- **Windows Console**: ConPTY and legacy console support
- **No-TTY**: Graceful fallback without color codes

### Character Encoding
- **UTF-8**: Native support for Unicode text
- **ASCII**: Optimized path for ASCII-only content
- **Locale**: Automatic locale detection for number formatting

## Best Practices

### Performance
1. **Use `spill_no_newline` for progress indicators**
2. **Cache formatted strings when possible**
3. **Avoid excessive color formatting in tight loops**
4. **Use `format` instead of string concatenation for complex strings**

### Maintainability
1. **Create wrapper functions for consistent logging**
2. **Use structured output formats for data**
3. **Handle input validation explicitly**
4. **Test interactive components with scripted input**

### Security
1. **Sanitize user input before display**
2. **Avoid displaying sensitive information**
3. **Validate format string parameters**
4. **Handle output errors gracefully**

## Migration Guide

### From Other Languages

#### From Go
```go
// Go
fmt.Println("Hello", name)
fmt.Printf("Value: %d\n", value)

// CURSED
vibez.spill("Hello", name)
vibez.spill("Value:", value)
```

#### From Rust
```rust
// Rust
println!("Hello {}", name);
eprintln!("Error: {}", error);

// CURSED
vibez.spill("Hello", name)
vibez.spill_error("Error:", error)
```

## Future Enhancements

### Planned Features
- **Structured Logging**: JSON/XML output formats
- **Performance Monitoring**: Built-in timing and profiling
- **Advanced Formatting**: Custom format specifiers
- **Stream Redirection**: Programmatic output routing

### Experimental Features
- **Async I/O**: Non-blocking input/output operations
- **Binary Output**: Structured binary data writing
- **Network Streams**: Direct network output support

---

The `vibez` module is the cornerstone of CURSED I/O operations, providing reliable, performant, and user-friendly APIs for all output needs. Its design emphasizes simplicity while maintaining the flexibility needed for complex applications.
