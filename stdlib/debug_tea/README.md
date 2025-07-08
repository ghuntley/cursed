# Debug Tea - Pure CURSED Debug Utilities Module

A comprehensive debug utilities module for CURSED applications, providing debugging tools, variable inspection, stack trace utilities, and debug assertions.

## Features

### Debug Level Management
- **Configurable Debug Levels**: 6 levels from NONE to TRACE
- **Dynamic Enable/Disable**: Runtime control of debug output
- **Level Filtering**: Show only messages at or below specified level

### Variable Inspection
- **Type-Specific Inspection**: Dedicated functions for different data types
- **Formatted Output**: Clean, readable variable dumps
- **Multi-Type Support**: integers, strings, booleans, floats

### Debug Assertions
- **Condition Assertions**: Assert true/false conditions with custom messages
- **Equality Assertions**: Compare values with detailed failure output
- **Stack Trace Integration**: Automatic stack trace on assertion failure

### Stack Trace Utilities
- **Call Stack Tracking**: Enter/exit function tracking
- **Stack Trace Printing**: Detailed execution path information
- **Debugging Context**: Function call hierarchy visualization

### Performance Monitoring
- **Timer Functions**: Start/end timer utilities for performance measurement
- **Memory Inspection**: Memory usage reporting and analysis
- **Benchmarking Support**: Built-in timing and profiling tools

## Usage

### Basic Debug Setup

```cursed
yeet "debug_tea"

# Enable debug output
enable_debug()
set_debug_level(DEBUG_LEVEL_INFO)

# Basic debug messages
debug_info("Application started")
debug_error("Critical error occurred")
debug_warn("Warning: deprecated function used")
```

### Variable Inspection

```cursed
sus user_id normie = 12345
sus user_name tea = "alice"
sus is_admin lit = based

# Inspect variables
inspect_int("user_id", user_id)
inspect_var("user_name", user_name)
inspect_bool("is_admin", is_admin)
```

### Debug Assertions

```cursed
# Assert conditions
debug_assert_true(user_id > 0, "User ID must be positive")
debug_assert_eq_string(user_name, "alice", "User name verification")
debug_assert_eq_int(calculate_total(), 100, "Total calculation check")
```

### Stack Trace Monitoring

```cursed
slay process_user_data(user_id normie) {
    debug_print_call_stack("process_user_data")
    
    # Function logic here
    debug_validate_positive_int(user_id, "user_id")
    
    debug_print_return_stack("process_user_data")
}
```

### Performance Monitoring

```cursed
debug_start_timer("database_query")
# Execute database query
debug_end_timer("database_query")

debug_print_memory_usage()
```

### Validation Helpers

```cursed
# Validate function parameters
debug_validate_not_nil(input_data, "input_data")
debug_validate_range_int(page_size, 1, 100, "page_size")
debug_validate_positive_int(timeout, "timeout")
```

## API Reference

### Debug Control Functions

- `enable_debug()` - Enable debug output
- `disable_debug()` - Disable debug output
- `set_debug_level(level normie)` - Set debug level (0-5)
- `is_debug_enabled() lit` - Check if debug is enabled
- `get_debug_level() normie` - Get current debug level

### Debug Output Functions

- `debug_error(message tea)` - Error level message
- `debug_warn(message tea)` - Warning level message
- `debug_info(message tea)` - Info level message
- `debug_trace(message tea)` - Trace level message

### Variable Inspection Functions

- `inspect_var(name tea, value tea)` - Inspect string variable
- `inspect_int(name tea, value normie)` - Inspect integer variable
- `inspect_bool(name tea, value lit)` - Inspect boolean variable
- `inspect_float(name tea, value meal)` - Inspect float variable

### Debug Assertion Functions

- `debug_assert(condition lit, message tea)` - Assert condition
- `debug_assert_true(condition lit, message tea)` - Assert true
- `debug_assert_false(condition lit, message tea)` - Assert false
- `debug_assert_eq_int(actual normie, expected normie, message tea)` - Assert integer equality
- `debug_assert_eq_string(actual tea, expected tea, message tea)` - Assert string equality

### Stack Trace Functions

- `debug_print_stack_trace()` - Print complete stack trace
- `debug_print_call_stack(function_name tea)` - Print function entry
- `debug_print_return_stack(function_name tea)` - Print function exit

### Performance Monitoring Functions

- `debug_start_timer(name tea)` - Start performance timer
- `debug_end_timer(name tea)` - End performance timer
- `debug_print_memory_usage()` - Print memory usage information

### Validation Helper Functions

- `debug_validate_not_nil(value tea, name tea)` - Validate non-nil value
- `debug_validate_range_int(value normie, min normie, max normie, name tea)` - Validate integer range
- `debug_validate_positive_int(value normie, name tea)` - Validate positive integer

### Utility Functions

- `debug_print_config()` - Print debug configuration
- `debug_print_hex(value normie, name tea)` - Print hexadecimal representation
- `debug_print_binary(value normie, name tea)` - Print binary representation
- `debug_breakpoint(message tea)` - Simulate breakpoint
- `debug_test_section(section_name tea)` - Mark test section
- `debug_print_summary()` - Print debug module summary

## Debug Levels

| Level | Constant | Description |
|-------|----------|-------------|
| 0 | DEBUG_LEVEL_NONE | No debug output |
| 1 | DEBUG_LEVEL_ERROR | Error messages only |
| 2 | DEBUG_LEVEL_WARN | Error and warning messages |
| 3 | DEBUG_LEVEL_INFO | Error, warning, and info messages |
| 4 | DEBUG_LEVEL_DEBUG | All messages except trace |
| 5 | DEBUG_LEVEL_TRACE | All debug messages |

## Testing

Run the comprehensive test suite:

```bash
# Test in interpretation mode
cargo run --bin cursed stdlib/debug_tea/test_debug_tea.csd

# Test in compilation mode
cargo run --bin cursed -- compile stdlib/debug_tea/test_debug_tea.csd
./test_debug_tea
```

## Implementation Details

- **Pure CURSED Implementation**: No FFI dependencies
- **Runtime Configurable**: Debug settings can be changed at runtime
- **Performance Conscious**: Minimal overhead when debug is disabled
- **Comprehensive Coverage**: 25+ test functions covering all functionality
- **Cross-Platform**: Works in both interpretation and compilation modes

## Best Practices

1. **Use Appropriate Debug Levels**: Choose the right level for each message
2. **Disable in Production**: Set debug level to NONE for production builds
3. **Validate Early**: Use validation helpers at function entry points
4. **Stack Trace Context**: Use call stack functions for complex debugging
5. **Performance Monitoring**: Use timers to identify bottlenecks

## Contributing

When adding new debug utilities:

1. Follow the existing naming conventions
2. Add corresponding test cases
3. Update this README with new functions
4. Ensure compatibility with both interpretation and compilation modes
5. Test with debug enabled and disabled states
