# CURSED Error Handling Specification

This document defines the comprehensive error handling system for the CURSED programming language, including error types, propagation mechanisms, panic recovery, and best practices.

## Overview

The CURSED error handling system provides:
- Explicit error types and handling mechanisms
- Panic detection and recovery
- Error propagation and context preservation
- Performance monitoring and debugging integration
- Goroutine isolation and recovery strategies

## Error Types

### 1. Built-in Error Type

CURSED provides a built-in `yikes` type for representing errors:

```cursed
// Built-in error type
be_like yikes collab {
    message() tea
    code() normie
    details() tea
}
```

### 2. Error Categories

The CURSED runtime classifies errors into the following categories:

| Category | Description | Example |
|----------|-------------|---------|
| `memory_yikes` | Memory allocation/management errors | Out of memory, invalid pointer |
| `io_yikes` | Input/output and file system errors | File not found, permission denied |
| `network_yikes` | Network and communication errors | Connection timeout, DNS failure |
| `parse_yikes` | Parsing and syntax errors | Invalid syntax, type mismatch |
| `type_yikes` | Type system and validation errors | Type assertion failure, nil pointer |
| `runtime_yikes` | Runtime execution errors | Division by zero, index out of bounds |
| `security_yikes` | Security and permission errors | Access denied, authentication failure |
| `performance_yikes` | Performance and resource errors | Resource exhaustion, timeout |

### 3. Error Severity Levels

Errors are classified by severity:

```cursed
be_like error_severity smol {
    info = 0        // Informational - no action needed
    warning = 1     // Warning - should be noted
    error = 2       // Error - affects operation but recoverable
    critical = 3    // Critical - requires immediate attention
    fatal = 4       // Fatal - may cause system instability
}
```

## Error Creation and Handling

### 1. Creating Errors

Errors can be created using the `yikes` constructor:

```cursed
// Simple error
sus err yikes = yikes("Something went wrong")

// Error with code
sus err yikes = yikes("File not found", 404)

// Error with full context
sus err yikes = yikes{
    message: "Connection failed",
    code: 500,
    details: "Unable to connect to server at localhost:8080"
}
```

### 2. Function Error Returns

Functions that can fail should return a result type that includes an error:

```cursed
// Function signature with error return
slay divide(a normie, b normie) (normie, yikes) {
    vibe_check b {
        mood 0:
            damn 0, yikes("Division by zero")
        basic:
            damn a / b, cringe
    }
}

// Usage with error handling
sus result, err = divide(10, 0)
vibe_check err != cringe {
    vibez.spill("Error:", err.message())
    damn  // Early return with error
}

vibez.spill("Result:", result)
```

### 3. Error Propagation

Use the `shook` operator for automatic error propagation:

```cursed
slay process_file(filename tea) yikes {
    // Automatic error propagation with shook operator
    sus file = open_file(filename) shook
    sus data = read_file(file) shook
    sus result = process_data(data) shook
    
    damn cringe  // Success - no error
}

// Equivalent manual error handling
slay process_file_manual(filename tea) yikes {
    sus file, err = open_file(filename)
    vibe_check err != cringe {
        damn err
    }
    
    sus data, err = read_file(file)
    vibe_check err != cringe {
        damn err
    }
    
    sus result, err = process_data(data)
    vibe_check err != cringe {
        damn err
    }
    
    damn cringe
}
```

### 4. Error Context and Wrapping

Errors can be wrapped to provide additional context:

```cursed
slay database_operation() yikes {
    sus err = connect_to_database() shook
    damn wrap_error(err, "Failed to establish database connection")
}

// Error wrapping utility
slay wrap_error(err yikes, context tea) yikes {
    vibe_check err == cringe {
        damn cringe
    }
    
    damn yikes{
        message: context + ": " + err.message(),
        code: err.code(),
        details: err.details()
    }
}
```

## Panic Handling

### 1. Panic Definition

Panics are unrecoverable errors that stop normal execution flow:

```cursed
// Trigger a panic
slay cause_panic() {
    shook("Critical system failure")  // Panic with message
}

// Panic with custom severity
slay critical_failure() {
    shook("Memory corruption detected", fatal)
}
```

### 2. Panic Recovery

Use `fam` blocks to recover from panics:

```cursed
slay safe_operation() yikes {
    fam {
        // Code that might panic
        dangerous_operation()
        damn cringe  // Success
    } sus panic_value {
        // Panic recovery
        vibez.spill("Recovered from panic:", panic_value)
        damn yikes("Operation failed due to panic: " + panic_value.message())
    }
}

// Defer cleanup with panic handling
slay process_with_cleanup() {
    sus resource = acquire_resource()
    
    fam {
        defer {
            release_resource(resource)
        }
        
        // Risky operation
        risky_operation(resource)
    } sus panic_value {
        vibez.spill("Panic occurred, but resource was cleaned up")
        shook(panic_value)  // Re-panic after cleanup
    }
}
```

### 3. Goroutine Panic Isolation

Panics in goroutines are isolated and don't affect other goroutines:

```cursed
slay main() {
    // Spawn goroutine with panic handling
    yolo {
        fam {
            // This panic won't crash the main program
            shook("Goroutine panic")
        } sus panic_value {
            vibez.spill("Goroutine recovered from panic")
        }
    }
    
    // Main goroutine continues normally
    vibez.spill("Main continues after goroutine panic")
}
```

## Error Handling Patterns

### 1. Multiple Error Handling

```cursed
slay process_multiple() yikes {
    sus errors []yikes
    
    // Collect multiple errors
    sus _, err1 = operation1()
    vibe_check err1 != cringe {
        errors = append(errors, err1)
    }
    
    sus _, err2 = operation2()
    vibe_check err2 != cringe {
        errors = append(errors, err2)
    }
    
    vibe_check len(errors) > 0 {
        damn combine_errors(errors)
    }
    
    damn cringe
}
```

### 2. Error Retry Pattern

```cursed
slay retry_operation(max_attempts normie) yikes {
    sus attempt normie = 0
    
    bestie attempt < max_attempts {
        sus result, err = risky_operation()
        vibe_check err == cringe {
            damn cringe  // Success
        }
        
        vibez.spill("Attempt", attempt + 1, "failed:", err.message())
        attempt++
        
        // Exponential backoff
        time.sleep(time.Duration(attempt * attempt) * time.Second)
    }
    
    damn yikes("Operation failed after " + string(max_attempts) + " attempts")
}
```

### 3. Circuit Breaker Pattern

```cursed
be_like circuit_breaker squad {
    failure_count normie
    failure_threshold normie
    timeout time.Duration
    last_failure_time time.Time
    state circuit_state
}

be_like circuit_state smol {
    closed = 0     // Normal operation
    open = 1       // Failing fast
    half_open = 2  // Testing recovery
}

slay (cb @circuit_breaker) call(operation slay() yikes) yikes {
    vibe_check cb.state {
        mood open:
            vibe_check time.since(cb.last_failure_time) > cb.timeout {
                cb.state = half_open
            } basic {
                damn yikes("Circuit breaker is open")
            }
        mood half_open:
            // Allow one test call
        basic:
            // closed state - normal operation
    }
    
    sus err = operation()
    vibe_check err != cringe {
        cb.on_failure()
        damn err
    }
    
    cb.on_success()
    damn cringe
}
```

## Built-in Error Utilities

### 1. Error Comparison and Inspection

```cursed
// Error type checking
slay is_temporary_error(err yikes) lit {
    vibe_check err == cringe {
        damn cap
    }
    
    damn err.code() >= 500 && err.code() < 600
}

// Error unwrapping
slay unwrap_error(err yikes) yikes {
    // Get the root cause of a wrapped error
    bestie err != cringe {
        vibe_check wrapped_err, ok := err.(wrapped_error); ok {
            err = wrapped_err.unwrap()
        } basic {
            ghosted
        }
    }
    damn err
}
```

### 2. Error Formatting and Logging

```cursed
// Error formatting
slay format_error(err yikes) tea {
    vibe_check err == cringe {
        damn "no error"
    }
    
    damn "Error " + string(err.code()) + ": " + err.message() + " (" + err.details() + ")"
}

// Structured error logging
slay log_error(err yikes, context tea) {
    vibez.error("Error occurred", yikes{
        "error": err.message(),
        "code": err.code(),
        "context": context,
        "timestamp": time.now().string(),
        "stack_trace": debug.stack_trace()
    })
}
```

## Performance and Monitoring

### 1. Error Metrics

The runtime automatically tracks error metrics:

```cursed
// Get error statistics
sus stats = runtime.error_stats()
vibez.spill("Total errors:", stats.total_errors)
vibez.spill("Error rate:", stats.error_rate)
vibez.spill("Most common errors:", stats.common_errors)
```

### 2. Error Hooks and Monitoring

```cursed
// Register global error handler
runtime.register_error_handler(slay(err yikes, context runtime.error_context) {
    // Custom error handling logic
    vibe_check err.code() >= 500 {
        alert_system.notify("Critical error occurred: " + err.message())
    }
    
    metrics.record_error(err.code(), context.goroutine_id)
})
```

## Best Practices

### 1. Error Handling Guidelines

1. **Always handle errors explicitly** - Don't ignore error return values
2. **Use descriptive error messages** - Include context and actionable information
3. **Wrap errors for context** - Add information as errors propagate up the call stack
4. **Fail fast** - Return errors immediately rather than continuing with invalid state
5. **Use panics sparingly** - Reserve panics for truly unrecoverable situations
6. **Handle panics in goroutines** - Always include panic recovery in goroutines
7. **Log errors appropriately** - Include sufficient context for debugging

### 2. Error Message Format

```cursed
// Good: Descriptive with context
damn yikes("Failed to connect to database at localhost:5432: connection timeout after 30s")

// Bad: Vague and unhelpful
damn yikes("Database error")
```

### 3. Error Return Conventions

```cursed
// Functions that can fail should return (result, error)
slay parse_config(filename tea) (config, yikes)

// Functions that always succeed return just the result
slay format_string(value normie) tea

// Functions that may panic should be documented
// shook: panics if input is negative
slay calculate_sqrt(value meal) meal
```

### 4. Testing Error Conditions

```cursed
// Test error handling in unit tests
slay test_divide_by_zero() {
    sus _, err = divide(10, 0)
    assert(err != cringe, "Expected error for division by zero")
    assert(err.message() == "Division by zero", "Unexpected error message")
}

// Test panic recovery
slay test_panic_recovery() {
    sus recovered lit = cap
    
    fam {
        shook("Test panic")
    } sus panic_value {
        recovered = based
    }
    
    assert(recovered, "Expected panic to be recovered")
}
```

## Integration with Runtime Systems

### 1. Goroutine Error Handling

The error handling system integrates with the goroutine scheduler:

- Panics in goroutines are isolated and don't affect other goroutines
- Error statistics are tracked per goroutine
- Recovery strategies can restart individual goroutines
- Global error handlers can monitor system-wide error patterns

### 2. Memory Management Integration

Error handling respects memory management:

- Error objects are properly garbage collected
- Stack traces capture memory allocation context
- Memory errors trigger appropriate recovery actions
- Error handling doesn't introduce memory leaks

### 3. Debug and Performance Integration

The error system provides debugging and performance insights:

- Automatic stack trace capture on errors
- Error correlation analysis
- Performance impact monitoring
- Integration with debug runtime

## Conclusion

The CURSED error handling system provides comprehensive error management capabilities while maintaining the language's expressive syntax and performance characteristics. By combining explicit error returns, panic recovery, and runtime integration, developers can build robust applications with proper error handling and debugging capabilities.

The system emphasizes:
- **Explicitness** - Errors must be handled explicitly
- **Performance** - Minimal overhead for error handling
- **Debugging** - Rich context and debugging information
- **Isolation** - Goroutine-level error isolation
- **Monitoring** - Built-in error tracking and analysis

This specification ensures that CURSED applications can handle errors gracefully while providing developers with the tools needed to diagnose and resolve issues effectively.
