# CURSED Error Core Module

The `error_core` module provides a comprehensive error handling hierarchy and propagation system for CURSED applications, implementing the full error handling specification with advanced features for robust error management.

## Features

### Core Error Types
- **Base Error**: Foundation error type with message, code, details, severity, and stack trace
- **IO Error**: File system and input/output operations
- **Value Error**: Data validation and conversion errors
- **Type Error**: Type system and casting errors
- **Memory Error**: Memory allocation and management errors
- **Network Error**: Network operations and communication errors
- **Parse Error**: Parsing and syntax errors with position information
- **Security Error**: Security violations and permission errors
- **Runtime Error**: Runtime execution errors with goroutine context

### Error Propagation
- **Error Wrapping**: Add context to errors as they propagate up the call stack
- **Error Chaining**: Chain multiple related errors together
- **Error Combining**: Aggregate multiple errors into a single error
- **Automatic Propagation**: Use `shook` operator for seamless error propagation

### Advanced Features
- **Circuit Breaker**: Prevent cascading failures with automatic circuit breaking
- **Retry Mechanism**: Automatic retry with exponential backoff for temporary errors
- **Error Statistics**: Track error patterns and metrics
- **Error Context**: Capture execution context including goroutine, function, and file information
- **Severity Levels**: Classify errors by severity (info, warning, error, critical, fatal)

## Usage Examples

### Basic Error Creation

```cursed
yeet "error_core"

# Create a simple error
sus err = new_error("Something went wrong", 1000)
vibez.spill(err.message())  # "Something went wrong"
vibez.spill(err.code())     # 1000

# Create specific error types
sus io_err = new_io_error("File not found", "/path/to/file.txt", "read")
sus value_err = new_value_error("Invalid number", "abc", "normie")
sus type_err = new_type_error("Type mismatch", "tea", "normie")
```

### Error Propagation and Wrapping

```cursed
slay process_file(filename tea) yikes {
    sus file, err = open_file(filename)
    vibe_check err != cringe {
        damn wrap_error(err, "Failed to open file")
    }
    
    sus data, err = read_file(file)
    vibe_check err != cringe {
        damn wrap_error(err, "Failed to read file")
    }
    
    damn cringe  # Success
}

# Usage with error handling
sus err = process_file("nonexistent.txt")
vibe_check err != cringe {
    vibez.spill("Error:", format_error(err))
}
```

### Error Chaining and Combining

```cursed
# Chain related errors
sus base_err = yikes{message: "Database connection failed", code: 5001, details: ""}
sus new_err = yikes{message: "User authentication failed", code: 7001, details: ""}
sus chained = chain_error(base_err, new_err)

# Combine multiple errors
sus errors []yikes = []yikes{
    yikes{message: "Error 1", code: 1000, details: ""},
    yikes{message: "Error 2", code: 2000, details: ""},
    yikes{message: "Error 3", code: 3000, details: ""}
}
sus combined = combine_errors(errors)
```

### Circuit Breaker Pattern

```cursed
sus cb = new_circuit_breaker(3, 2)  # 3 failures to open, 2 successes to close

slay protected_operation() yikes {
    damn cb.call(slay() yikes {
        # Your risky operation here
        damn risky_network_call()
    })
}
```

### Retry with Backoff

```cursed
slay resilient_operation() yikes {
    damn retry_with_backoff(slay() yikes {
        # Operation that might fail temporarily
        damn network_request()
    }, 5, 100)  # 5 max attempts, 100ms base delay
}
```

### Error Type Detection

```cursed
sus err = some_operation()
vibe_check err != cringe {
    vibe_check is_temporary_error(err) {
        vibez.spill("Temporary error, might retry")
    } basic vibe_check is_critical_error(err) {
        vibez.spill("Critical error, immediate attention needed")
    }
    
    vibe_check is_error_type(err, "network_error") {
        vibez.spill("Network-related error")
    }
}
```

### Error Formatting

```cursed
sus err = yikes{message: "Test error", code: 1000, details: "Test details"}

# Standard formatting
sus formatted = format_error(err)
# Output: "[Error 1000] Test error | Details: Test details"

# JSON formatting
sus json_formatted = format_error_json(err)
# Output: {"error": {"code": 1000, "message": "Test error", "details": "Test details"}}
```

### Error Statistics and Monitoring

```cursed
# Record errors for monitoring
record_error(some_error)

# Get error statistics
sus stats = get_error_stats()
vibez.spill("Total errors:", stats.total_errors)
vibez.spill("Errors by type:", stats.errors_by_type)
```

## Error Code Ranges

The module uses structured error codes for easy categorization:

- **1000-1999**: IO Errors
- **2000-2999**: Value Errors
- **3000-3999**: Type Errors
- **4000-4999**: Memory Errors
- **5000-5999**: Network Errors
- **6000-6999**: Parse Errors
- **7000-7999**: Security Errors
- **8000-8999**: Runtime Errors
- **9000-9999**: System/Combined Errors

## Error Severity Levels

```cursed
be_like error_severity smol {
    info = 0        # Informational - no action needed
    warning = 1     # Warning - should be noted
    error = 2       # Error - affects operation but recoverable
    critical = 3    # Critical - requires immediate attention
    fatal = 4       # Fatal - may cause system instability
}
```

## Integration with CURSED Error Handling

The module integrates seamlessly with CURSED's built-in error handling syntax:

### Using `yikes` for Error Creation
```cursed
sus err yikes = yikes("Something went wrong")
sus detailed_err yikes = yikes{
    message: "Complex error",
    code: 1000,
    details: "Additional context"
}
```

### Using `shook` for Error Propagation
```cursed
slay process_data() yikes {
    sus data = load_data() shook  # Automatic propagation
    sus result = transform_data(data) shook
    damn cringe
}
```

### Using `fam` for Panic Recovery
```cursed
slay safe_operation() yikes {
    fam {
        # Risky operation that might panic
        dangerous_operation()
        damn cringe
    } sus panic_value {
        # Convert panic to error
        damn yikes{
            message: "Operation panicked: " + panic_value.message(),
            code: 9999,
            details: "Panic recovered"
        }
    }
}
```

## Best Practices

1. **Always Handle Errors**: Never ignore error return values
2. **Provide Context**: Use error wrapping to add context as errors propagate
3. **Use Appropriate Types**: Choose the right error type for the situation
4. **Log Errors**: Use `record_error()` for monitoring and debugging
5. **Handle Temporary Errors**: Use retry mechanisms for temporary failures
6. **Monitor Critical Errors**: Set up alerting for critical error types
7. **Test Error Paths**: Include error scenarios in your unit tests

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/error_core/test_error_core.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/error_core/test_error_core.csd
./test_error_core
```

## Integration with Other Modules

The error_core module is designed to be used throughout the CURSED standard library:

```cursed
# In other stdlib modules
yeet "error_core"

slay some_function() yikes {
    # Use error_core types
    vibe_check some_condition {
        damn new_value_error("Invalid input", input_value, "expected_type")
    }
    damn cringe
}
```

This module provides the foundation for robust error handling across the entire CURSED ecosystem, enabling applications to handle errors gracefully and provide meaningful feedback to users and developers.
