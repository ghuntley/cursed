# CURSED Logging Module

Production-ready logging system for CURSED programs with structured logging, multiple log levels, and flexible output formatting.

## Features

- **Multiple Log Levels**: TRACE, DEBUG, INFO, WARN, ERROR, FATAL
- **Structured Logging**: Log with key-value fields for better analysis
- **Named Loggers**: Create logger instances with custom names
- **Flexible Formatting**: Customizable log message formats
- **File Output**: Log to files with rotation support
- **Performance Logging**: Built-in performance and memory usage tracking
- **Error Handling**: Specialized error logging with stack traces
- **Conditional Logging**: Log based on runtime conditions
- **Debug Utilities**: Function entry/exit, variable logging, checkpoints

## Usage

### Import the Module

```cursed
yeet "logging"
```

### Basic Logging

```cursed
// Basic log levels
logging.log_trace("Detailed trace information")
logging.log_debug("Debug information")
logging.log_info("General information")
logging.log_warn("Warning message")
logging.log_error("Error occurred")
logging.log_fatal("Fatal error")
```

### Log Level Configuration

```cursed
// Set minimum log level (only INFO and above will be logged)
logging.set_log_level(logging.LOG_INFO)

// Set custom log format
logging.set_log_format("[{timestamp}] {level}: {message}")

// Set log file output
logging.set_log_file("app.log")
```

### Structured Logging

```cursed
// Log with structured fields
sus fields map[tea]tea = map[tea]tea{
    "user_id": "12345",
    "action": "login",
    "ip_address": "192.168.1.1",
    "status": "success"
}

logging.info_with_fields("User login", fields)
logging.warn_with_fields("Suspicious activity", fields)
logging.error_with_fields("Authentication failed", fields)
```

### Named Loggers

```cursed
// Create named logger
sus db_logger logging.Logger = logging.create_logger("Database")
sus api_logger logging.Logger = logging.create_logger("API")

// Use named loggers
logging.logger_info(db_logger, "Database connection established")
logging.logger_warn(api_logger, "API rate limit approaching")
logging.logger_error(db_logger, "Query execution failed")
```

### Performance Logging

```cursed
// Log performance metrics
logging.log_performance("database_query", 150)  // 150ms
logging.log_performance("file_upload", 3000)    // 3 seconds

// Log memory usage
logging.log_memory_usage("data_processing", 1024000)  // 1MB
logging.log_memory_usage("cache_allocation", 512000)  // 512KB
```

### Error Logging

```cursed
// Log error with stack trace
sus stack_trace tea = "at main() line 42\nat process_data() line 15"
logging.log_error_with_stack("Database connection failed", stack_trace)

// Log exception details
logging.log_exception("NullPointerException", "Variable was null", "processor.csd:25")
```

### Debug Utilities

```cursed
// Function entry/exit logging
logging.log_function_entry("process_user_data")
// ... function code ...
logging.log_checkpoint("data_validated")
// ... more function code ...
logging.log_function_exit("process_user_data")

// Variable logging
sus user_id tea = "12345"
logging.log_variable("user_id", user_id)
```

### Conditional Logging

```cursed
// Log based on conditions
sus debug_mode lit = based
logging.log_if(debug_mode, logging.LOG_DEBUG, "Debug mode is enabled")

sus error_occurred lit = cap
logging.log_if(error_occurred, logging.LOG_ERROR, "An error occurred")
```

### Log File Management

```cursed
// Get current log file size
sus file_size thicc = logging.get_log_file_size()
vibez.spill("Log file size: " + tea(file_size) + " bytes")

// Rotate log file (moves current to .old, creates new)
logging.rotate_log_file()

// Clear log file
logging.clear_log_file()
```

## API Reference

### Log Levels (Constants)
- `LOG_TRACE` = 0 - Detailed trace information
- `LOG_DEBUG` = 1 - Debug information
- `LOG_INFO` = 2 - General information
- `LOG_WARN` = 3 - Warning messages
- `LOG_ERROR` = 4 - Error messages
- `LOG_FATAL` = 5 - Fatal errors

### Configuration Functions
- `set_log_level(level: normie)` - Set minimum log level
- `set_log_file(filename: tea)` - Set log file output
- `set_log_format(format: tea)` - Set log message format

### Basic Logging Functions
- `log_trace(message: tea)` - Log trace message
- `log_debug(message: tea)` - Log debug message
- `log_info(message: tea)` - Log info message
- `log_warn(message: tea)` - Log warning message
- `log_error(message: tea)` - Log error message
- `log_fatal(message: tea)` - Log fatal message

### Structured Logging Functions
- `info_with_fields(message: tea, fields: map[tea]tea)` - Log info with fields
- `warn_with_fields(message: tea, fields: map[tea]tea)` - Log warning with fields
- `error_with_fields(message: tea, fields: map[tea]tea)` - Log error with fields

### Named Logger Functions
- `create_logger(name: tea) -> Logger` - Create named logger
- `logger_trace(logger: Logger, message: tea)` - Log trace with logger
- `logger_debug(logger: Logger, message: tea)` - Log debug with logger
- `logger_info(logger: Logger, message: tea)` - Log info with logger
- `logger_warn(logger: Logger, message: tea)` - Log warning with logger
- `logger_error(logger: Logger, message: tea)` - Log error with logger
- `logger_fatal(logger: Logger, message: tea)` - Log fatal with logger

### Performance Logging
- `log_performance(operation: tea, duration_ms: thicc)` - Log performance metrics
- `log_memory_usage(operation: tea, bytes_used: thicc)` - Log memory usage

### Error Logging
- `log_error_with_stack(message: tea, stack_trace: tea)` - Log error with stack trace
- `log_exception(type: tea, message: tea, location: tea)` - Log exception details

### Debug Utilities
- `log_variable(var_name: tea, value: tea)` - Log variable value
- `log_function_entry(function_name: tea)` - Log function entry
- `log_function_exit(function_name: tea)` - Log function exit
- `log_checkpoint(checkpoint_name: tea)` - Log checkpoint

### Conditional Logging
- `log_if(condition: lit, level: normie, message: tea)` - Log if condition is true

### File Management
- `rotate_log_file()` - Rotate current log file
- `clear_log_file()` - Clear current log file
- `get_log_file_size() -> thicc` - Get log file size

## Types

```cursed
be_like Logger squad {
    name tea           // Logger name
    level normie       // Minimum log level
    format tea         // Log message format
    file tea          // Log file path
}
```

## Log Format Placeholders

- `{timestamp}` - Current timestamp
- `{level}` - Log level name
- `{message}` - Log message
- `{logger}` - Logger name (for named loggers)

## Best Practices

### Performance Considerations
```cursed
// Use appropriate log levels
logging.log_trace("Very detailed debug info")  // Only in development
logging.log_debug("Debug info")                // Development and staging
logging.log_info("Normal operations")          // Production info
logging.log_warn("Potential issues")           // Always log
logging.log_error("Errors occurred")           // Always log
```

### Structured Logging
```cursed
// Use consistent field names
sus fields map[tea]tea = map[tea]tea{
    "user_id": "12345",
    "session_id": "abc123",
    "action": "login",
    "timestamp": "2025-01-07T12:00:00Z"
}
logging.info_with_fields("User action", fields)
```

### Error Handling
```cursed
// Always log errors with context
logging.log_error_with_stack("Database query failed", stack_trace)
logging.log_exception("SQLException", "Connection timeout", "db_manager.csd:150")
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/logging/test_logging.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/logging/test_logging.csd
./test_logging
```

## Implementation Notes

- Uses pure CURSED language features
- Minimal FFI dependencies through runtime bridge
- Thread-safe logging operations
- Efficient string formatting
- Configurable output destinations
- Production-ready error handling

## Self-Hosting Ready

This module is essential for self-hosting and provides all logging capabilities needed for:

- Compiler debugging and diagnostics
- Build system logging
- Runtime error reporting
- Performance monitoring
- Development debugging
- Production monitoring and alerting
