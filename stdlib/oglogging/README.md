# CURSED Standard Library: `oglogging` Package

The `oglogging` package provides an advanced logging facility for CURSED applications with support for multiple log levels and structured output.

## Overview

The `oglogging` package implements a powerful logging system that provides both simple logging for basic use cases and advanced features for production applications. It offers multiple log levels, constants for log level management, and clean, formatted output.

## Quick Start

```cursed
yeet "oglogging"

slay main() {
    # Basic logging
    Spill("Application started")
    
    # Level-based logging
    Debug("Debug information")
    Info("General information")
    Warn("Warning message")
    Error("Error occurred")
    Fatal("Critical failure")
}
```

## Features

### ✅ Multiple Log Levels
- **DEBUG**: Detailed diagnostic information
- **INFO**: General information messages
- **WARN**: Warning messages for potentially harmful situations
- **ERROR**: Error messages for serious problems
- **FATAL**: Critical errors (note: does not terminate program in this implementation)

### ✅ Clean Output Formatting
- **Prefixed output**: Each log level has its own prefix
- **Consistent formatting**: All messages follow the same format
- **Unicode support**: Full support for international characters and emojis

### ✅ Constants and Configuration
- **Log level constants**: DEBUG=0, INFO=1, WARN=2, ERROR=3, FATAL=4
- **Easy integration**: Simple function calls for all log levels
- **Thread safe**: Safe for use in concurrent CURSED programs

## Basic Usage

### Simple Logging

```cursed
yeet "oglogging"

slay main() {
    # Basic message logging
    Spill("This is a basic log message")
    
    # Level-specific logging
    Debug("Debugging application flow")
    Info("User logged in successfully")
    Warn("Configuration file not found, using defaults")
    Error("Database connection failed")
    Fatal("Unable to start critical service")
}
```

Output:
```
LOG: This is a basic log message
[DEBUG] Debugging application flow
[INFO] User logged in successfully
[WARN] Configuration file not found, using defaults
[ERROR] Database connection failed
[FATAL] Unable to start critical service
```

### Using Log Level Constants

```cursed
yeet "oglogging"

slay main() {
    # Access log level constants
    vibez.spill("DEBUG level: " + DEBUG)    # Outputs: DEBUG level: 0
    vibez.spill("INFO level: " + INFO)      # Outputs: INFO level: 1
    vibez.spill("WARN level: " + WARN)      # Outputs: WARN level: 2
    vibez.spill("ERROR level: " + ERROR)    # Outputs: ERROR level: 3
    vibez.spill("FATAL level: " + FATAL)    # Outputs: FATAL level: 4
    
    # Use constants for conditional logging
    sus currentLevel := INFO
    lowkey currentLevel <= DEBUG {
        Debug("This debug message will be shown")
    }
}
```

## API Reference

### Logging Functions

- `Spill(message tea)` - Basic logging with "LOG:" prefix
- `Debug(message tea)` - Debug level logging with "[DEBUG]" prefix
- `Info(message tea)` - Info level logging with "[INFO]" prefix
- `Warn(message tea)` - Warning level logging with "[WARN]" prefix
- `Error(message tea)` - Error level logging with "[ERROR]" prefix
- `Fatal(message tea)` - Fatal level logging with "[FATAL]" prefix

### Log Level Constants

- `DEBUG normie = 0` - Debug level constant
- `INFO normie = 1` - Info level constant
- `WARN normie = 2` - Warning level constant
- `ERROR normie = 3` - Error level constant
- `FATAL normie = 4` - Fatal level constant

## Examples

### Application Startup Logging

```cursed
yeet "oglogging"

slay startApplication() {
    Info("Application starting...")
    
    # Configuration loading
    Info("Loading configuration...")
    lowkey !loadConfig() {
        Warn("Configuration file not found, using defaults")
    }
    
    # Database connection
    Info("Connecting to database...")
    lowkey !connectDatabase() {
        Error("Database connection failed")
        Fatal("Cannot start without database")
        damn
    }
    
    Info("Application started successfully")
}
```

### Error Handling

```cursed
yeet "oglogging"

slay processRequest(userID tea) {
    Debug("Processing request for user: " + userID)
    
    lowkey userID == "" {
        Error("Invalid user ID provided")
        damn
    }
    
    # Process the request
    Info("Request processed successfully for user: " + userID)
}

slay handleError(err tea) {
    Error("An error occurred: " + err)
    Debug("Error handling completed")
}
```

### Development vs Production Logging

```cursed
yeet "oglogging"

sus isProduction lit = cap  # Set to based for production

slay log(level normie, message tea) {
    # Only log INFO and above in production
    lowkey isProduction {
        lowkey level >= INFO {
            logMessage(level, message)
        }
    } nah {
        # Log everything in development
        logMessage(level, message)
    }
}

slay logMessage(level normie, message tea) {
    lowkey level == DEBUG {
        Debug(message)
    } nah lowkey level == INFO {
        Info(message)
    } nah lowkey level == WARN {
        Warn(message)
    } nah lowkey level == ERROR {
        Error(message)
    } nah {
        Fatal(message)
    }
}
```

## Performance Considerations

### High-Performance Logging

1. **Use appropriate log levels** - Only log what's necessary in production
2. **Minimize string concatenation** in hot paths
3. **Consider log filtering** at the application level

### Memory Usage

- The `oglogging` module has minimal memory overhead
- All functions use simple string operations
- No global state accumulation

## Best Practices

1. **Use appropriate log levels** for different types of messages
2. **Include context** in log messages (user IDs, operation names, etc.)
3. **Keep messages concise** but informative
4. **Use consistent formatting** across your application
5. **Test logging** in both development and production scenarios

## Integration Examples

### Web Application Logging

```cursed
yeet "oglogging"

slay handleWebRequest(path tea, userID tea) {
    Info("Web request started - Path: " + path + " User: " + userID)
    
    # Process request logic here
    
    Info("Web request completed - Path: " + path + " User: " + userID)
}

slay handleWebError(path tea, err tea) {
    Error("Web request failed - Path: " + path + " Error: " + err)
}
```

### Service Monitoring

```cursed
yeet "oglogging"

slay monitorService() {
    Info("Service monitoring started")
    
    # Check service health
    lowkey !isServiceHealthy() {
        Warn("Service health check failed")
        # Take corrective action
    }
    
    Debug("Service monitoring cycle completed")
}
```

## Testing

Run the test suite:

```bash
cargo run --bin cursed stdlib/oglogging/test_oglogging.💀
```

The test suite covers:
- All logging functions (Spill, Debug, Info, Warn, Error, Fatal)
- Log level constants (DEBUG, INFO, WARN, ERROR, FATAL)
- Unicode and special character support
- Edge cases (empty messages, special characters)
- Performance with multiple log calls

## Status

✅ **Production Ready** - The `oglogging` package is fully implemented and tested with comprehensive functionality suitable for production CURSED applications.

### Current Implementation Features

- ✅ All log level functions implemented
- ✅ Log level constants available
- ✅ Unicode and emoji support
- ✅ Clean, consistent output formatting
- ✅ Thread-safe implementation
- ✅ Comprehensive test coverage
- ✅ Zero external dependencies

### Future Enhancements

Future versions may include:
- File output capabilities
- JSON structured logging
- Log rotation features
- Custom formatting options
- Performance optimizations

The current implementation provides a solid foundation for logging in CURSED applications and can be extended as needed.
