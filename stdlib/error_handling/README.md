# Error Handling (error_handling)

The `error_handling` module provides comprehensive error management and recovery mechanisms for CURSED programs.

## Purpose

This module implements structured error handling, error propagation, recovery strategies, and debugging support for robust CURSED applications.

## Main Functions

### Error Creation and Management
- `error_handling.new_error(message)` - Create new error with message
- `error_handling.new_error_with_code(code, message)` - Create error with code
- `error_handling.wrap_error(error, context)` - Wrap error with additional context
- `error_handling.is_error(value)` - Check if value is an error
- `error_handling.error_message(error)` - Get error message
- `error_handling.error_code(error)` - Get error code

### Error Propagation
- `error_handling.propagate(result)` - Propagate error up call stack
- `error_handling.try_catch(fn, handler)` - Try-catch mechanism
- `error_handling.maybe(value)` - Convert value to Maybe type
- `error_handling.unwrap_or(result, default)` - Unwrap with default value

### Error Recovery
- `error_handling.retry(fn, max_attempts)` - Retry operation with backoff
- `error_handling.timeout(fn, duration)` - Execute with timeout
- `error_handling.fallback(primary, fallback)` - Fallback strategy
- `error_handling.circuit_breaker(fn, threshold)` - Circuit breaker pattern

### Debugging and Logging
- `error_handling.print_stack_trace(error)` - Print detailed stack trace
- `error_handling.log_error(error, level)` - Log error with severity
- `error_handling.set_error_handler(handler)` - Global error handler
- `error_handling.get_last_error()` - Get last error for debugging

## Usage Examples

### Basic Error Handling

```cursed
yeet "error_handling"

slay risky_operation(value normie) Result<normie, tea> {
    if value < 0 {
        damn error_handling.new_error("Negative value not allowed")
    }
    damn error_handling.ok(value * 2)
}

sus result = risky_operation(-5)
if error_handling.is_error(result) {
    vibez.spillf("Error: {}", error_handling.error_message(result))
} else {
    vibez.spillf("Success: {}", result.value)
}
```

### Error Wrapping and Context

```cursed
yeet "error_handling"

slay file_operation(filename tea) Result<tea, tea> {
    sus content = read_file(filename)
    if error_handling.is_error(content) {
        damn error_handling.wrap_error(content, "Failed to read configuration file")
    }
    damn content
}

sus result = file_operation("config.json")
match result {
    error_handling.Error(e) => {
        error_handling.print_stack_trace(e)
        vibez.spillf("Operation failed: {}", error_handling.error_message(e))
    },
    error_handling.Ok(data) => {
        vibez.spillf("File content: {}", data)
    }
}
```

### Retry and Recovery Patterns

```cursed
yeet "error_handling"

slay unreliable_network_call() Result<tea, tea> {
    fr fr Simulate network operation
    if math.random() < 0.7 {
        damn error_handling.new_error("Network timeout")
    }
    damn error_handling.ok("Network response")
}

fr fr Retry with exponential backoff
sus result = error_handling.retry(unreliable_network_call, 3)
match result {
    error_handling.Error(e) => vibez.spillf("Failed after retries: {}", e),
    error_handling.Ok(data) => vibez.spillf("Success: {}", data)
}

fr fr Circuit breaker pattern
sus protected_call = error_handling.circuit_breaker(unreliable_network_call, 5)
sus circuit_result = protected_call()
```

### Try-Catch Mechanism

```cursed
yeet "error_handling"

sus result = error_handling.try_catch(
    slay() {
        sus value = risky_calculation()
        damn process_value(value)
    },
    slay(error) {
        error_handling.log_error(error, "WARNING")
        damn error_handling.ok("Default value")
    }
)

vibez.spillf("Final result: {}", result)
```

### Global Error Handling

```cursed
yeet "error_handling"

fr fr Set up global error handler
error_handling.set_error_handler(slay(error) {
    error_handling.log_error(error, "ERROR")
    error_handling.print_stack_trace(error)
    fr fr Could send to monitoring system here
})

fr fr Now all unhandled errors will be caught
sus result = dangerous_operation()
```

## Compilation Examples

### Interpretation Mode
```bash
echo 'yeet "error_handling"
sus err = error_handling.new_error("Test error")
vibez.spillf("Error: {}", error_handling.error_message(err))' > error_test.csd

./cursed-unified error_test.csd
```

### Compilation Mode
```bash
./cursed-unified --compile error_test.csd
./error_test
```

## Error Types

### Result Type
```cursed
collab Result<T, E> {
    Ok(T)
    Error(E)
}
```

### Maybe Type
```cursed
collab Maybe<T> {
    Some(T)
    None
}
```

### Error Severity Levels
- `"DEBUG"` - Debug information
- `"INFO"` - Informational messages
- `"WARNING"` - Warning conditions
- `"ERROR"` - Error conditions
- `"CRITICAL"` - Critical errors

## Implementation Notes

- Zero-cost abstractions when errors don't occur
- Stack trace preservation for debugging
- Thread-safe error handling
- Integration with CURSED's pattern matching
- Structured error codes for programmatic handling

## Dependencies

- `memory` - For error object management
- `logging` - For error logging functionality
- `time` - For timeout operations
- No external dependencies (pure CURSED)

## Performance Considerations

- Minimal overhead when no errors occur
- Efficient error propagation up call stack
- Memory-efficient error object representation
- Optimized retry mechanisms with backoff

## Best Practices

1. **Use Result types** for operations that can fail
2. **Wrap errors with context** for better debugging  
3. **Implement retry logic** for transient failures
4. **Use circuit breakers** for external dependencies
5. **Log errors appropriately** with proper severity
6. **Handle errors at appropriate level** in call stack
7. **Provide meaningful error messages** for users
