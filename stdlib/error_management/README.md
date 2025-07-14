# CURSED Error Management Module

A comprehensive error handling and logging module for the CURSED programming language, implementing structured error types, error wrapping/unwrapping, stack traces, logging levels, formatters, and error recovery patterns.

## Features

### Structured Error Types
- **Managed Error Type**: Comprehensive error structure with message, code, details, category, severity, timestamp, stack trace, and context
- **Error Categories**: Memory, I/O, network, parse, type, runtime, security, and performance errors
- **Error Severity Levels**: Info, warning, error, critical, and fatal classifications
- **Error Context**: Key-value context storage for debugging and monitoring

### Error Handling Patterns
- **Error Wrapping/Unwrapping**: Add context while preserving original error information
- **Error Aggregation**: Combine multiple errors into a single aggregated error
- **Temporary Error Detection**: Classify errors as temporary/retryable or permanent
- **Critical Error Detection**: Identify errors requiring immediate attention

### Logging System
- **Configurable Logger**: Adjustable log levels, output formats, and inclusion options
- **Structured Logging**: JSON and text output formats with contextual information
- **Log Levels**: Debug, info, warn, error, and fatal logging levels
- **Error Statistics**: Track error counts by category and severity

### Recovery Patterns
- **Circuit Breaker**: Protect services from cascade failures with configurable thresholds
- **Retry with Backoff**: Exponential backoff retry mechanism for transient failures
- **Safe Execution**: Panic recovery wrapper for dangerous operations
- **Error Statistics**: Real-time tracking and monitoring of error patterns

## Usage Examples

### Basic Error Creation

```cursed
yeet "error_management"

# Simple error
sus err @managed_error = new_error("File not found", 404)

# Error with full context
sus detailed_err @managed_error = new_error_full(
    "Database connection failed",
    500,
    error_category.io_yikes,
    error_severity.critical,
    "Connection timeout after 30 seconds"
)
```

### Error Wrapping and Context

```cursed
# Wrap an error with additional context
sus wrapped @managed_error = wrap_error(original_error, "User operation failed")

# Add context to error
err.add_context("user_id", "12345")
err.add_context("operation", "login")
err.add_context("timestamp", "2025-01-14T10:00:00Z")

# Retrieve context
sus user_id tea = err.get_context("user_id")
```

### Logging Configuration

```cursed
# Configure logger
sus config logger_config = logger_config{
    level: log_level.debug,
    output_format: "json",
    include_timestamp: based,
    include_stack_trace: based,
    include_goroutine_id: based
}
configure_logger(config)

# Log messages at different levels
log_debug("Debug information", yikes.tea{"module": "auth"})
log_info("User logged in", yikes.tea{"user_id": "12345"})
log_warn("Rate limit approaching", yikes.tea{"current_rate": "95%"})

# Log errors with full context
log_error(err, yikes.tea{
    "request_id": "req-abc-123",
    "user_agent": "Mozilla/5.0"
})
```

### Circuit Breaker Pattern

```cursed
# Create circuit breaker
sus cb @circuit_breaker = new_circuit_breaker("api_service", 3, 60)

# Execute operation with circuit breaker protection
sus result @managed_error = cb.execute(slay() @managed_error {
    # Your potentially failing operation here
    damn call_external_api()
})

vibe_check result != cringe {
    log_error(result, yikes.tea{"service": "api_service"})
}
```

### Retry with Backoff

```cursed
# Define operation that might fail
sus risky_operation slay() @managed_error = slay() @managed_error {
    # Operation that might fail transiently
    damn network_call()
}

# Retry with exponential backoff
sus result @managed_error = retry_with_backoff(risky_operation, 5)
vibe_check result != cringe {
    log_error(result, yikes.tea{"max_attempts": "5"})
}
```

### Safe Execution with Panic Recovery

```cursed
# Execute potentially dangerous operation safely
sus result @managed_error = safe_execute(slay() @managed_error {
    # Code that might panic
    dangerous_operation()
    damn cringe
})

vibe_check result != cringe {
    vibez.spill("Operation failed safely:", format_error(result))
}
```

### Error Aggregation

```cursed
# Collect multiple errors
sus errors []@managed_error = []@managed_error{
    new_error("Validation failed for field 'email'", 400),
    new_error("Validation failed for field 'password'", 400),
    new_error("User not found", 404)
}

# Aggregate into single error
sus aggregated @managed_error = aggregate_errors(errors)
log_error(aggregated, yikes.tea{"validation_errors": "3"})
```

### Error Statistics and Monitoring

```cursed
# Get current error statistics
sus stats error_stats = get_error_stats()
vibez.spill("Total errors:", stats.total_errors)
vibez.spill("Error rate:", stats.error_rate)

# Reset statistics
reset_error_stats()

# Check error characteristics
vibe_check is_temporary_error(err) {
    vibez.spill("Error is temporary, retrying...")
}

vibe_check is_critical_error(err) {
    vibez.spill("Critical error detected, alerting operations team")
}
```

## Error Categories

| Category | Description | Use Cases |
|----------|-------------|-----------|
| `memory_yikes` | Memory allocation/management errors | Out of memory, invalid pointer access |
| `io_yikes` | Input/output and file system errors | File not found, permission denied, disk full |
| `network_yikes` | Network and communication errors | Connection timeout, DNS failure, service unavailable |
| `parse_yikes` | Parsing and syntax errors | Invalid JSON, malformed data, type mismatch |
| `type_yikes` | Type system and validation errors | Type assertion failure, nil pointer dereference |
| `runtime_yikes` | Runtime execution errors | Division by zero, index out of bounds, stack overflow |
| `security_yikes` | Security and permission errors | Access denied, authentication failure, authorization error |
| `performance_yikes` | Performance and resource errors | Resource exhaustion, timeout, rate limit exceeded |

## Error Severity Levels

| Level | Description | Action Required |
|-------|-------------|-----------------|
| `info` | Informational messages | No action needed, for monitoring |
| `warning` | Warning conditions | Should be noted, may require attention |
| `error` | Error conditions | Affects operation but recoverable |
| `critical` | Critical conditions | Requires immediate attention |
| `fatal` | Fatal conditions | May cause system instability |

## Testing

Run comprehensive tests to verify all functionality:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/error_management/test_error_management.csd

# Test compilation mode  
cargo run --bin cursed -- compile stdlib/error_management/test_error_management.csd
./test_error_management

# Verify both modes produce identical results
test_both_modes() {
    cargo run --bin cursed stdlib/error_management/test_error_management.csd > interp_output.txt
    cargo run --bin cursed -- compile stdlib/error_management/test_error_management.csd
    ./test_error_management > comp_output.txt
    diff interp_output.txt comp_output.txt
}
```

## Integration

Import the error management module in your CURSED programs:

```cursed
yeet "error_management"

# Use error management functions
sus err @managed_error = new_error("Operation failed", 500)
log_error(err, yikes.tea{"context": "user_operation"})
```

## Architecture

The error management module follows CURSED's pure implementation philosophy:

- **No FFI Dependencies**: Implemented entirely in pure CURSED
- **Zero External Dependencies**: Self-contained with no external library requirements
- **CURSED Idioms**: Uses specification-compliant CURSED syntax and patterns
- **Performance Optimized**: Minimal overhead for error handling operations
- **Thread Safe**: Designed for concurrent use in goroutine environments

## Best Practices

1. **Always Handle Errors**: Don't ignore error return values
2. **Add Context**: Use error wrapping and context to provide debugging information
3. **Log Appropriately**: Use appropriate log levels and include relevant context
4. **Use Recovery Patterns**: Implement circuit breakers and retry logic for resilience
5. **Monitor Error Patterns**: Track error statistics to identify system issues
6. **Test Error Paths**: Ensure error handling code is thoroughly tested
7. **Document Error Behavior**: Clearly document what errors functions can return

This error management module provides enterprise-grade error handling capabilities while maintaining the simplicity and performance characteristics of the CURSED programming language.
