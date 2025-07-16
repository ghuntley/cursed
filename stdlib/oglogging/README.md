# oglogging Module

Enterprise-grade logging facility for CURSED applications with timestamps, prefixes, and configurable formatting options. Inspired by Go's `log` package but enhanced for modern development needs.

## Features

- **Standard Logger**: Pre-configured logger for immediate use
- **Custom Loggers**: Create loggers with specific outputs, prefixes, and flags
- **Flexible Formatting**: Configurable timestamp, file info, and prefix formats
- **Structured Logging**: Log with structured fields and different levels
- **Log Rotation**: Automatic log file rotation based on size limits
- **Multi-Writer Support**: Log to multiple destinations simultaneously
- **Performance Monitoring**: Built-in performance logging and metrics
- **Thread Safety**: Safe for concurrent use across goroutines

## Quick Start

```cursed
yeet "oglogging"

slay main() {
    # Basic logging
    oglogging.spill("Hello, world!")
    oglogging.spillf("User %s logged in", "alice")
    
    # Set custom prefix
    oglogging.setPrefix("APP: ")
    oglogging.spill("Application started")
    
    # Configure output format
    oglogging.setFlags(oglogging.Ldate | oglogging.Ltime | oglogging.Lshortfile)
    oglogging.spill("Formatted log message")
}
```

## Logger Configuration

### Output Flags

- `Ldate`: Include date (2025/04/13)
- `Ltime`: Include time (15:04:05)
- `Lmicroseconds`: Include microseconds (15:04:05.123456)
- `Llongfile`: Include full file path and line
- `Lshortfile`: Include just filename and line
- `LUTC`: Use UTC time instead of local time
- `Lmsgprefix`: Move prefix before message instead of line start
- `LstdFlags`: Default flags (Ldate | Ltime)

### Custom Logger

```cursed
yeet "oglogging"
yeet "dropz"

slay main() {
    # Create custom logger
    sus file, err := dropz.Create("app.log")
    if err != "" {
        shook("Failed to create log file")
    }
    defer file.Close()
    
    sus logger := oglogging.new(file, "DEBUG: ", oglogging.LstdFlags)
    
    logger.spill("Custom log message")
    logger.spillf("User %s performed action %s", "bob", "login")
}
```

## Structured Logging

```cursed
yeet "oglogging"

slay main() {
    sus logger := oglogging.new(dropz.stdout, "", oglogging.LstdFlags)
    sus structured := oglogging.NewStructuredLogger(logger)
    
    # Set log level
    structured.SetLevel(oglogging.INFO)
    
    # Log with different levels
    structured.Debug("This won't appear")
    structured.Info("Application started")
    structured.Warn("Configuration file not found, using defaults")
    structured.Error("Database connection failed")
    
    # Log with fields
    sus userLogger := structured.WithField("user", "alice").WithField("session", "abc123")
    userLogger.Info("User logged in successfully")
    
    # Log with multiple fields
    sus fields := make(map[tea]interface{})
    fields["action"] = "purchase"
    fields["amount"] = "99.99"
    fields["currency"] = "USD"
    
    structured.WithFields(fields).Info("Transaction completed")
}
```

## Log Rotation

```cursed
yeet "oglogging"

slay main() {
    # Create rotating logger (max 10MB per file, keep 5 files)
    sus rotatingLogger := oglogging.NewRotatingLogger("app.log", 10485760, 5)
    
    # Log messages that will automatically rotate when size limit is reached
    for i := 0; i < 1000; i++ {
        rotatingLogger.Log("This is log message number " + intToString(i))
    }
}
```

## Multi-Writer Logging

```cursed
yeet "oglogging"
yeet "dropz"

slay main() {
    # Log to both console and file
    sus file, _ := dropz.Create("app.log")
    defer file.Close()
    
    sus writers := []dropz.Writer{dropz.stdout, file}
    sus multiWriter := oglogging.NewMultiWriter(writers)
    
    sus logger := oglogging.new(multiWriter, "MULTI: ", oglogging.LstdFlags)
    logger.spill("This goes to both console and file")
}
```

## Performance Monitoring

```cursed
yeet "oglogging"

slay main() {
    sus logger := oglogging.new(dropz.stdout, "", oglogging.LstdFlags)
    sus perfLogger := oglogging.NewPerfLogger(logger)
    
    # Monitor operation performance
    perfLogger.StartOperation("database_query")
    
    # Simulate database work
    performDatabaseQuery()
    
    perfLogger.EndOperation("database_query")
    perfLogger.LogPerformanceReport()
}
```

## Error and Fatal Logging

```cursed
yeet "oglogging"

slay main() {
    # Fatal logging (exits with code 1)
    oglogging.fatal("Critical error: cannot continue")
    
    # Panic logging (triggers panic)
    oglogging.shook("Unexpected state detected")
    
    # Formatted versions
    oglogging.fatalf("Configuration error in %s", "database.yaml")
    oglogging.shookf("Invalid state: %s", "connection lost")
}
```

## Testing

Run the test suite:

```bash
cargo run --bin cursed stdlib/oglogging/test_oglogging.csd
```

The module includes comprehensive tests for:
- Basic logging functionality
- Logger creation and configuration
- Flag combinations and formatting
- Structured logging with levels and fields
- Log rotation mechanics
- Multi-writer support
- Performance logging
- Error handling
- Concurrent logging safety

## Implementation Notes

- All logging operations are thread-safe
- Log messages automatically get newlines appended if not present
- File paths in log output are extracted to show just filenames for readability
- Performance logger tracks operation timing with nanosecond precision
- Structured logging supports arbitrary field types through interface{} values
- Multi-writer logging continues even if individual writers fail

## Integration Examples

The oglogging module integrates seamlessly with other CURSED stdlib modules:

```cursed
# With dropz for file I/O
sus file, _ := dropz.Create("logs/application.log")
sus logger := oglogging.new(file, "APP: ", oglogging.LstdFlags)

# With timez for custom timestamps
sus now := timez.Now()
logger.spillf("System started at %s", now.Format("2006-01-02 15:04:05"))

# With web_vibez for HTTP request logging
logger.spillf("HTTP %s %s - %d", method, url, statusCode)
```

This makes oglogging an essential component for production CURSED applications requiring comprehensive logging capabilities.
