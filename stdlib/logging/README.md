# CURSED Logging Module 📝

A comprehensive, pure CURSED logging system with structured logging, multiple levels, named loggers, and Gen Z vibes.

## Features ✨

- **Multiple Log Levels**: TRACE, DEBUG, INFO, WARN, ERROR, FATAL with filtering
- **Named Loggers**: Create logger instances with custom names and prefixes
- **Structured Logging**: Log with key-value fields for better analysis
- **Performance Logging**: Built-in performance and memory usage tracking
- **Error Handling**: Specialized error logging with context and stack traces
- **Conditional Logging**: Log based on runtime conditions
- **Debug Utilities**: Function entry/exit, variable logging, checkpoints
- **Application Lifecycle**: Startup, shutdown, configuration change logging
- **Network & Security**: Request, network event, and security logging
- **Transaction & Health**: Database, cache, batch, and health monitoring
- **Goroutine Support**: Concurrent operation logging
- **Pure CURSED**: No FFI dependencies, self-hosting ready

## Usage 🚀

### Import the Module

```cursed
yeet "logging"
```

### Basic Logging (Core Drip)

```cursed
// Basic log levels
logging.log_trace("Detailed trace information")    // Most detailed
logging.log_debug("Debug information")             // For debugging
logging.log_info("General information")            // Default level
logging.log_warn("Warning message")                // Potential issues  
logging.log_error("Error occurred")                // Something broke
logging.log_fatal("Fatal error")                   // Major failure
```

### Configuration (Setup That Vibe)

```cursed
// Set minimum log level (only INFO and above will be logged)
logging.set_log_level(logging.LOG_INFO)

// Set custom log prefix
logging.set_log_prefix("[MyApp]")
```

### Named Loggers (Custom Logger Drip)

```cursed
// Create named loggers for different components
sus db_logger logging.Logger = logging.create_logger("Database")
sus api_logger logging.Logger = logging.create_logger("API")
sus auth_logger logging.Logger = logging.create_logger("Auth")

// Use named loggers
logging.logger_info(db_logger, "Database connection established")
logging.logger_warn(api_logger, "API rate limit approaching")
logging.logger_error(auth_logger, "Authentication failed")
```

### Conditional Logging (Smart Drip)

```cursed
// Log based on conditions
sus debug_enabled lit = based
logging.log_if(debug_enabled, logging.LOG_DEBUG, "Debug mode is enabled")

sus error_occurred lit = cap
logging.log_if(error_occurred, logging.LOG_ERROR, "This won't appear")
```

### Performance Logging (Perf Drip)

```cursed
// Log performance metrics
logging.log_performance("database_query", 150)     // 150ms
logging.log_performance("file_upload", 3000)       // 3 seconds

// Log memory usage
logging.log_memory_usage("data_processing", 1024000)  // 1MB
logging.log_memory_usage("cache_allocation", 512000)  // 512KB
```

### Error Logging (Error Context Drip)

```cursed
// Log error with context
logging.log_error_with_context("Database connection failed", "timeout after 30s")

// Log exception details
logging.log_exception("NullPointerException", "Variable was null", "processor.csd:25")
```

### Debug Utilities (Debug Drip)

```cursed
// Function entry/exit logging
logging.log_function_entry("process_user_data")
// ... function code ...
logging.log_checkpoint("data_validated")
// ... more function code ...
logging.log_function_exit("process_user_data")

// Variable logging
sus user_id normie = 12345
logging.log_variable("user_id", tea(user_id))
```

### Structured Logging (Struct Drip)

```cursed
// Single field logging
logging.log_info_with_field("User login", "user_id", "12345")
logging.log_warn_with_field("Rate limit hit", "client_ip", "192.168.1.100")
logging.log_error_with_field("Payment failed", "order_id", "ORD-789")

// Multi-field logging
logging.log_info_with_fields("Transaction processed", "tx_id", "TX-123", "amount", "$99.99")
```

### Application Lifecycle (Lifecycle Drip)

```cursed
// Application startup/shutdown
logging.log_app_start("CursedApp", "1.0.0")
logging.log_config_change("max_connections", "100", "200")
logging.log_app_shutdown("CursedApp")
```

### Network & Request Logging (Network Drip)

```cursed
// HTTP request logging
logging.log_request("GET", "/api/users", 200)
logging.log_request("POST", "/api/login", 401)

// Network events
logging.log_network_event("Connection established", "127.0.0.1", 8080)
logging.log_network_event("Connection closed", "192.168.1.100", 3306)
```

### Database Logging (DB Drip)

```cursed
// Database operations
logging.log_db_operation("SELECT", "users", 15)      // 15ms query
logging.log_db_operation("INSERT", "orders", 45)     // 45ms insert
logging.log_db_operation("UPDATE", "inventory", 120) // 120ms update
```

### Security Logging (Security Drip)

```cursed
// Security events
logging.log_security_event("LOGIN_SUCCESS", "john.doe", "IP: 192.168.1.50")
logging.log_security_event("LOGIN_FAILURE", "anonymous", "Invalid credentials")
logging.log_security_event("PERMISSION_DENIED", "guest", "Attempted admin access")
```

### Cache & Batch Operations (Cache Drip)

```cursed
// Cache operations
logging.log_cache_operation("GET", "user:12345", based)  // cache hit
logging.log_cache_operation("GET", "user:67890", cap)    // cache miss

// Batch operations
logging.log_batch_operation("user_import", 1000, 5500)   // 1000 items in 5.5s
logging.log_batch_operation("email_send", 250, 12000)    // 250 emails in 12s
```

### Transaction Logging (Transaction Drip)

```cursed
// Transaction lifecycle
logging.log_transaction_start("TX-001")
logging.log_transaction_complete("TX-001", based, 150)  // success in 150ms

logging.log_transaction_start("TX-002")
logging.log_transaction_complete("TX-002", cap, 75)     // failure after 75ms
```

### Health & Monitoring (Health Drip)

```cursed
// Health checks
logging.log_health_check("database", "healthy", 25)     // 25ms response
logging.log_health_check("redis", "degraded", 150)      // 150ms response
logging.log_health_check("api_gateway", "unhealthy", 5000) // 5s timeout

// Rate limiting
logging.log_rate_limit("client_123", "/api/data", 95, 100)   // 95/100 requests
logging.log_rate_limit("client_456", "/api/upload", 105, 100) // over limit
```

### Goroutine Logging (Concurrent Drip)

```cursed
// Goroutine lifecycle
logging.log_goroutine_start("worker-001", "process_images")
logging.log_goroutine_complete("worker-001", based, 2500)  // success in 2.5s

logging.log_goroutine_start("worker-002", "send_emails")
logging.log_goroutine_complete("worker-002", cap, 1200)    // failure after 1.2s
```

## API Reference 📚

### Log Levels (Level Drip Constants)
- `LOG_TRACE` = 0 - Most detailed information
- `LOG_DEBUG` = 1 - Debug information
- `LOG_INFO` = 2 - General information (default)
- `LOG_WARN` = 3 - Warning messages
- `LOG_ERROR` = 4 - Error messages
- `LOG_FATAL` = 5 - Fatal errors

### Configuration Functions (Setup Drip)
- `set_log_level(level: normie)` - Set minimum log level
- `set_log_prefix(prefix: tea)` - Set log message prefix
- `get_timestamp() -> tea` - Get current timestamp

### Basic Logging Functions (Core Drip)
- `log_trace(message: tea)` - Log trace message
- `log_debug(message: tea)` - Log debug message
- `log_info(message: tea)` - Log info message
- `log_warn(message: tea)` - Log warning message
- `log_error(message: tea)` - Log error message
- `log_fatal(message: tea)` - Log fatal message

### Conditional Logging (Smart Drip)
- `log_if(condition: lit, level: normie, message: tea)` - Log if condition is true

### Named Logger Functions (Logger Instance Drip)
- `create_logger(name: tea) -> Logger` - Create named logger
- `logger_trace(logger: Logger, message: tea)` - Log trace with logger
- `logger_debug(logger: Logger, message: tea)` - Log debug with logger
- `logger_info(logger: Logger, message: tea)` - Log info with logger
- `logger_warn(logger: Logger, message: tea)` - Log warning with logger
- `logger_error(logger: Logger, message: tea)` - Log error with logger
- `logger_fatal(logger: Logger, message: tea)` - Log fatal with logger

### Performance Logging (Perf Drip)
- `log_performance(operation: tea, duration_ms: normie)` - Log performance metrics
- `log_memory_usage(operation: tea, bytes_used: normie)` - Log memory usage

### Error Logging (Error Context Drip)
- `log_error_with_context(message: tea, context: tea)` - Log error with context
- `log_exception(type: tea, message: tea, location: tea)` - Log exception details

### Debug Utilities (Debug Drip)
- `log_variable(var_name: tea, value: tea)` - Log variable value
- `log_function_entry(function_name: tea)` - Log function entry
- `log_function_exit(function_name: tea)` - Log function exit
- `log_checkpoint(name: tea)` - Log checkpoint

### Formatted Logging (Format Drip)
- `log_info_f(format: tea, value: tea)` - Formatted info log
- `log_warn_f(format: tea, value: tea)` - Formatted warning log
- `log_error_f(format: tea, value: tea)` - Formatted error log

### Application Lifecycle (Lifecycle Drip)
- `log_app_start(app_name: tea, version: tea)` - Log application start
- `log_app_shutdown(app_name: tea)` - Log application shutdown
- `log_config_change(key: tea, old_value: tea, new_value: tea)` - Log config changes

### Network & Request Logging (Network Drip)
- `log_request(method: tea, path: tea, status: normie)` - Log HTTP request
- `log_network_event(event: tea, address: tea, port: normie)` - Log network event

### Database Logging (DB Drip)
- `log_db_operation(operation: tea, table: tea, duration_ms: normie)` - Log DB operation

### Security Logging (Security Drip)
- `log_security_event(event_type: tea, user: tea, details: tea)` - Log security event

### Cache & Batch Operations (Cache Drip)
- `log_cache_operation(operation: tea, key: tea, hit: lit)` - Log cache operation
- `log_batch_operation(operation: tea, count: normie, duration_ms: normie)` - Log batch operation

### Structured Logging (Struct Drip)
- `log_info_with_field(message: tea, key: tea, value: tea)` - Info with single field
- `log_warn_with_field(message: tea, key: tea, value: tea)` - Warning with single field
- `log_error_with_field(message: tea, key: tea, value: tea)` - Error with single field
- `log_info_with_fields(message: tea, key1: tea, val1: tea, key2: tea, val2: tea)` - Info with multiple fields

### Transaction Logging (Transaction Drip)
- `log_transaction_start(tx_id: tea)` - Log transaction start
- `log_transaction_complete(tx_id: tea, success: lit, duration_ms: normie)` - Log transaction completion

### Health & Monitoring (Health Drip)
- `log_health_check(service: tea, status: tea, response_time_ms: normie)` - Log health check
- `log_rate_limit(client: tea, endpoint: tea, current_count: normie, limit: normie)` - Log rate limiting

### Goroutine Logging (Concurrent Drip)
- `log_goroutine_start(routine_id: tea, operation: tea)` - Log goroutine start
- `log_goroutine_complete(routine_id: tea, success: lit, duration_ms: normie)` - Log goroutine completion

## Types 🏗️

```cursed
be_like Logger squad {
    name tea           // Logger name
    level normie       // Minimum log level
    prefix tea         // Log message prefix
}
```

## Testing 🧪

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/logging/test_logging.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/logging/test_logging.csd
./test_logging

# Both-mode verification
test_both_modes() {
    cargo run --bin cursed stdlib/logging/test_logging.csd > interp_output.txt
    cargo run --bin cursed -- compile stdlib/logging/test_logging.csd
    ./test_logging > comp_output.txt
    diff interp_output.txt comp_output.txt
}
```

## Best Practices 💡

### Performance Considerations
```cursed
// Use appropriate log levels for production
logging.set_log_level(logging.LOG_INFO)  // Hide trace/debug in production

// Use named loggers for component separation
sus db_logger logging.Logger = logging.create_logger("Database")
sus api_logger logging.Logger = logging.create_logger("API")
```

### Structured Logging Best Practices
```cursed
// Use consistent field names across the application
logging.log_info_with_fields("User login", "user_id", "12345", "session_id", "abc123")
logging.log_info_with_fields("User logout", "user_id", "12345", "session_id", "abc123")
```

### Error Handling Best Practices
```cursed
// Always log errors with context
logging.log_error_with_context("Database query failed", "timeout after 30s")
logging.log_exception("SQLException", "Connection timeout", "db_manager.csd:150")
```

### Security Logging Best Practices
```cursed
// Log all security events for audit trails
logging.log_security_event("LOGIN_SUCCESS", user_id, "IP: " + client_ip)
logging.log_security_event("PERMISSION_DENIED", user_id, "Attempted: " + resource)
```

## Implementation Notes 📋

- **Pure CURSED**: No FFI dependencies, implemented entirely in CURSED language
- **Thread-Safe**: Safe for use in concurrent goroutines
- **Efficient**: Minimal overhead with level filtering
- **Extensible**: Easy to add new logging functions and formatters
- **Self-Hosting Ready**: Essential for compiler debugging and build systems
- **Production Ready**: Suitable for enterprise applications and monitoring

## Output Format 📄

Default log format: `[PREFIX] [TIMESTAMP] LEVEL: MESSAGE`

Example outputs:
```
[CURSED] [2025-01-13T12:00:00Z] INFO: Application started successfully
[CURSED][Database] [2025-01-13T12:00:01Z] DEBUG: Connection pool initialized
[CURSED][API] [2025-01-13T12:00:02Z] WARN: Rate limit approaching for client_123
[CURSED] [2025-01-13T12:00:03Z] ERROR: Database connection failed | Context: timeout after 30s
```

## Use Cases 🎯

### Development
- Function tracing with `log_function_entry`/`log_function_exit`
- Variable debugging with `log_variable`
- Checkpoint debugging with `log_checkpoint`

### Production Monitoring
- Performance tracking with `log_performance`
- Health monitoring with `log_health_check`
- Security auditing with `log_security_event`

### Debugging
- Error context with `log_error_with_context`
- Exception tracking with `log_exception`
- Conditional debugging with `log_if`

### Self-Hosting
- Compiler diagnostics and debugging
- Build system logging and error reporting
- Runtime error tracking and performance monitoring

This logging module provides a comprehensive, production-ready logging solution that's essential for CURSED's self-hosting capabilities and enterprise deployment. 🚀
