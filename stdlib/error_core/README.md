# Error Core Module

Advanced pure CURSED error handling system implementing the `yikes`, `shook`, and `fam` error handling patterns for enterprise-grade error management.

## Features

- **yikes Pattern**: Error creation with typed error categories
- **shook Pattern**: Error wrapping and context addition  
- **fam Pattern**: Error handling, recovery, and fallback strategies
- **Panic System**: Critical error handling with recovery mechanisms
- **Error Classification**: Critical vs recoverable error categorization
- **Safe Operations**: Error-aware wrapper functions for common operations
- **Error Statistics**: Tracking and debugging support

## Error Patterns

### yikes - Error Creation
```cursed
# Create specific error types
sus runtime_err = yikes_runtime("Runtime failure")
sus logic_err = yikes_logic("Invalid operation")
sus io_err = yikes_io("File not found")
sus memory_err = yikes_memory("Out of memory")
sus validation_err = yikes_validation("Invalid input")
```

### shook - Error Wrapping
```cursed
# Wrap errors with additional context
sus original_err = yikes_runtime("Database connection failed")
sus wrapped_err = shook_wrap(original_err, "During user authentication")
sus context_err = shook_context(wrapped_err, "Function: authenticate_user")
```

### fam - Error Handling
```cursed
# Handle errors with fallbacks
sus result = fam_handle(potential_error, "default_value")
sus recovered = fam_recover(error, recovery_function)
sus ignored lit = fam_ignore(non_critical_error)
```

## Functions

### Error Creation (yikes pattern)
- `yikes_new(error_type tea, message tea, code normie)` - Create custom error
- `yikes_runtime(message tea)` - Create runtime error
- `yikes_logic(message tea)` - Create logic error
- `yikes_io(message tea)` - Create I/O error
- `yikes_memory(message tea)` - Create memory error
- `yikes_validation(message tea)` - Create validation error

### Error Wrapping (shook pattern)
- `shook_wrap(original_error, wrap_message tea)` - Wrap error with context
- `shook_context(error, context_info tea)` - Add context information

### Error Handling (fam pattern)
- `fam_handle(error, default_value)` - Handle with fallback value
- `fam_recover(error, recovery_function)` - Attempt error recovery
- `fam_ignore(error) lit` - Safely ignore non-critical errors

### Error Analysis
- `is_error(value) lit` - Check if value is an error
- `error_type(error) tea` - Extract error type
- `error_message(error) tea` - Extract error message
- `error_code(error) normie` - Extract error code
- `is_critical_error(error) lit` - Check if error is critical
- `is_recoverable_error(error) lit` - Check if error can be recovered

### Error Propagation
- `should_propagate(error) lit` - Determine if error should propagate
- `propagate_error(error, caller_context tea)` - Propagate with context
- `try_recovery(error, max_attempts normie) lit` - Attempt recovery

### Panic System
- `panic_with(message tea)` - Trigger panic state
- `recover_from_panic() lit` - Recover from panic

### Safe Operations
- `safe_divide(a normie, b normie)` - Division with error handling
- `safe_access(data, index normie)` - Safe array access

### Error Management
- `error_stats() tea` - Get error statistics
- `clear_errors()` - Clear error state
- `get_last_error()` - Get last error

## Usage Examples

### Basic Error Handling
```cursed
yeet "error_core"

# Create and handle errors
sus err = yikes_validation("Invalid user input")
sus result = fam_handle(err, "default_response")
vibez.spill(result)  # Outputs: "default_response"
```

### Error Chaining
```cursed
# Chain errors with context
sus db_err = yikes_io("Database connection failed")
sus auth_err = shook_wrap(db_err, "During user authentication")
sus final_err = shook_context(auth_err, "Login endpoint")

# Handle the chained error
sus fallback = fam_handle(final_err, "Please try again later")
```

### Safe Operations
```cursed
# Safe division with error handling
sus division_result = safe_divide(10, 0)
lowkey is_error(division_result) {
    vibez.spill("Division failed: " + error_message(division_result))
} else {
    vibez.spill("Result: " + division_result)
}
```

### Error Recovery
```cursed
# Attempt recovery with retries
sus operation_error = yikes_io("Network timeout")
sus recovered lit = try_recovery(operation_error, 3)

lowkey recovered {
    vibez.spill("Operation recovered successfully")
} else {
    vibez.spill("Recovery failed after 3 attempts")
}
```

## Testing

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/error_core/test_error_core.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/error_core/test_error_core.csd
./test_error_core
```

## Error Types

| Type | Code Range | Description |
|------|------------|-------------|
| runtime | 1001+ | Runtime execution errors |
| logic | 2001+ | Logic and algorithm errors |
| io | 3001+ | Input/output operation errors |
| memory | 4001+ | Memory allocation/management errors |
| validation | 5001+ | Data validation errors |
| context | 8888 | Context wrapper errors |
| wrapped | 9999 | Wrapped error containers |
| panic | 9999 | Critical panic-level errors |

## Implementation Notes

- Pure CURSED implementation without FFI dependencies
- Tuple-based error representation for flexibility
- Compatible with both interpretation and compilation modes
- Enterprise-grade error handling patterns
- Thread-safe error state management
- Comprehensive error categorization and recovery

## Dependencies

- `testz` - Testing framework (for tests only)

## Status

✅ **Complete** - Full implementation of CURSED error handling patterns with yikes/shook/fam keywords
