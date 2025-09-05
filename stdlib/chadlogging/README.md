# chadlogging - Pure CURSED Logging Module

A comprehensive logging module for CURSED applications providing structured logging, multiple log levels, formatting, and rotation capabilities.

## Features

- **Multiple Log Levels**: DEBUG, INFO, WARN, ERROR
- **Structured Logging**: Key-value pairs and contextual information
- **Flexible Formatting**: Customizable log message formats
- **Log Rotation**: Automatic log file rotation and management
- **Performance Logging**: Built-in performance measurement
- **Multiple Loggers**: Support for named logger instances
- **Pure CURSED**: No FFI dependencies

## Quick Start

```cursed
yeet "chadlogging"

// Initialize logging system
init_logging()

// Basic logging
info("Application started")
warn("This is a warning")
error("Something went wrong")

// Structured logging
log_with_fields(LOG_INFO, "User logged in", "user_id=123")
log_with_context(LOG_ERROR, "Database connection failed", "auth_service")

// Performance logging
sus start_time normie = perf_start("database_query")
// ... perform operation ...
perf_end("database_query", start_time)

// Cleanup
cleanup_logging()
```

## API Reference

### Log Levels

| Level | Value | Description |
|-------|-------|-------------|
| `LOG_DEBUG` | 0 | Detailed debug information |
| `LOG_INFO` | 1 | General information |
| `LOG_WARN` | 2 | Warning messages |
| `LOG_ERROR` | 3 | Error messages |

### Core Functions

#### `set_log_level(level normie) lit`
Sets the global log level. Only messages at or above this level will be output.

```cursed
set_log_level(LOG_WARN)  // Only warnings and errors
```

#### `debug(message tea) lit`
#### `info(message tea) lit`
#### `warn(message tea) lit`
#### `error(message tea) lit`
Basic logging functions for each level.

```cursed
debug("Debug information")
info("System status OK")
warn("Resource usage high")
error("Critical failure")
```

### Structured Logging

#### `log_with_fields(level normie, message tea, fields tea) lit`
Logs a message with additional key-value fields.

```cursed
log_with_fields(LOG_INFO, "User action", "user_id=456 action=login")
```

#### `log_with_context(level normie, message tea, context tea) lit`
Logs a message with contextual information.

```cursed
log_with_context(LOG_ERROR, "Query failed", "database_service")
```

### Logger Management

#### `create_logger(name tea, level normie) tea`
Creates a named logger instance.

```cursed
sus db_logger tea = create_logger("database", LOG_WARN)
```

#### `log_with_logger(logger_name tea, level normie, message tea) lit`
Logs a message using a specific logger.

```cursed
log_with_logger("auth", LOG_ERROR, "Authentication failed")
```

### Configuration

#### `set_log_file(path tea) lit`
Sets the log file path.

```cursed
set_log_file("/var/log/app.log")
```

#### `set_log_format(format tea) lit`
Sets the log message format.

```cursed
set_log_format("[%timestamp%] [%level%] %message%")
```

#### `set_max_log_size(size normie) lit`
Sets the maximum log file size in bytes.

```cursed
set_max_log_size(10485760)  // 10MB
```

#### `set_max_log_files(count normie) lit`
Sets the maximum number of rotated log files to keep.

```cursed
set_max_log_files(5)
```

### Performance Logging

#### `perf_start(operation tea) normie`
Starts a performance measurement.

```cursed
sus start_time normie = perf_start("api_request")
```

#### `perf_end(operation tea, start_time normie) lit`
Ends a performance measurement and logs the duration.

```cursed
perf_end("api_request", start_time)
```

### System Management

#### `init_logging() lit`
Initializes the logging system with default configuration.

#### `cleanup_logging() lit`
Cleans up the logging system and flushes buffers.

#### `flush_logs() lit`
Flushes log buffers to disk.

#### `rotate_logs() lit`
Manually triggers log rotation.

#### `get_log_stats() tea`
Returns logging statistics.

```cursed
sus stats tea = get_log_stats()
info("Logging stats: " + stats)
```

## Usage Patterns

### Application Logging

```cursed
yeet "chadlogging"

// Initialize with custom configuration
init_logging()
set_log_level(LOG_INFO)
set_log_file("app.log")
set_max_log_size(5242880)  // 5MB

// Application startup
info("Application starting")
log_with_fields(LOG_INFO, "Config loaded", "env=production")

// Error handling
shook some_condition {
    error("Critical error occurred")
    log_with_context(LOG_ERROR, "Database unavailable", "startup")
}

// Cleanup on exit
cleanup_logging()
```

### Service Logging

```cursed
yeet "chadlogging"

// Create service-specific loggers
sus auth_logger tea = create_logger("auth", LOG_WARN)
sus db_logger tea = create_logger("db", LOG_ERROR)

// Use specific loggers
log_with_logger("auth", LOG_WARN, "Invalid login attempt")
log_with_logger("db", LOG_ERROR, "Connection pool exhausted")
```

### Performance Monitoring

```cursed
yeet "chadlogging"

slay process_request() lit {
    sus start_time normie = perf_start("request_processing")
    
    // Process request...
    
    perf_end("request_processing", start_time)
    damn based
}
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/chadlogging/test_chadlogging.💀
```

Test both interpretation and compilation modes:

```bash
# Interpretation mode
cargo run --bin cursed stdlib/chadlogging/test_chadlogging.💀

# Compilation mode
cargo run --bin cursed -- compile stdlib/chadlogging/test_chadlogging.💀
./test_chadlogging
```

## Implementation Notes

- This is a pure CURSED implementation without FFI dependencies
- File I/O operations are simplified for demonstration
- In a production implementation, actual file system operations would be integrated
- Timestamp generation uses a placeholder implementation
- Log rotation logic is simplified but demonstrates the interface

## Thread Safety

The current implementation is designed for single-threaded use. For multi-threaded applications, additional synchronization would be required around shared state.

## Future Enhancements

- Asynchronous logging for better performance
- JSON and XML output formats
- Log compression
- Remote logging capabilities
- Advanced filtering and querying
- Integration with system logging facilities

## License

This module is part of the CURSED standard library and follows the same license terms.
