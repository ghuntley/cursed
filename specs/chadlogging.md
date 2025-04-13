# CURSED Standard Library: `chadlogging` Package

The `chadlogging` package provides a structured logging facility for CURSED applications, inspired by Go's `log/slog` package. It enables efficient, structured logging with levels, attributes, groups, and handlers.

## Overview

Package `chadlogging` implements structured logging with log records that include a message, severity level, and attributes expressed as key-value pairs. It defines a `Logger` type with methods for generating log events at different severity levels (Debug, Info, Warn, Error).

Each `Logger` is associated with a `Handler`, which processes the records and determines the output format. A default logger is accessible through package-level functions.

## Concepts

### Records

A log record consists of:
- A timestamp
- A log level (Debug, Info, Warn, Error)
- A message string
- A set of key-value pairs (attributes)

### Levels

The package defines standard severity levels with integer values:

```
LevelDebug = -4
LevelInfo  = 0 
LevelWarn  = 4
LevelError = 8
```

### Handlers

Handlers determine how log records are processed and formatted. Two built-in handlers are provided:

- `TextHandler`: Formats records as key=value pairs for easy machine parsing
- `JSONHandler`: Formats records as line-delimited JSON objects

### Attributes

Attributes are key-value pairs used to add structured data to log events. The API accepts attributes as alternating keys and values or as `Attr` objects.

### Groups

Attributes can be organized into groups to provide a hierarchical structure to logs.

## Usage Examples

### Basic Logging

```
yeet "chadlogging"

slay main() {
    chadlogging.info("server starting", "port", 8080)
}

// Output: time=2025-04-13T15:04:05.000-05:00 level=INFO msg="server starting" port=8080
```

### Using JSON Handler

```
yeet "chadlogging"
yeet "dropz"

slay main() {
    sus logger := chadlogging.new(chadlogging.new_json_handler(dropz.stdout, nil))
    logger.info("server starting", "port", 8080, "env", "production")
}

// Output: {"time":"2025-04-13T15:04:05.000-05:00","level":"INFO","msg":"server starting","port":8080,"env":"production"}
```

### With Attributes

```
yeet "chadlogging"

slay main() {
    sus requestID := "req-123456"
    sus logger := chadlogging.default().with("request_id", requestID)
    
    logger.info("processing request", "path", "/api/users")
    logger.debug("detailed info", "query_time_ms", 15)
}
```

### Using Groups

```
yeet "chadlogging"

slay main() {
    sus logger := chadlogging.default()
    logger.info("processed request",
        chadlogging.group("request", 
            "method", "GET",
            "path", "/api/users",
            "status", 200
        ),
        "duration_ms", 45
    )
}

// Output: time=2025-04-13T15:04:05.000 level=INFO msg="processed request" request.method=GET request.path=/api/users request.status=200 duration_ms=45
```

### Using Context

```
yeet "chadlogging"

slay handler(ctx collab{}) {
    chadlogging.info_context(ctx, "handling request")
}
```

## Main Types and Functions

### Logger Type

The `Logger` type handles record generation:

```
type Logger struct { ... }

// Creating loggers
func new(h Handler) *Logger
func default() *Logger
func set_default(l *Logger)

// Logger methods
func (l *Logger) debug(msg tea, args ...collab{})
func (l *Logger) debug_context(ctx collab{}, msg tea, args ...collab{})
func (l *Logger) info(msg tea, args ...collab{})
func (l *Logger) info_context(ctx collab{}, msg tea, args ...collab{})
func (l *Logger) warn(msg tea, args ...collab{})
func (l *Logger) warn_context(ctx collab{}, msg tea, args ...collab{})
func (l *Logger) error(msg tea, args ...collab{})
func (l *Logger) error_context(ctx collab{}, msg tea, args ...collab{})
func (l *Logger) log(ctx collab{}, level Level, msg tea, args ...collab{})
func (l *Logger) log_attrs(ctx collab{}, level Level, msg tea, attrs ...Attr)
func (l *Logger) with(args ...collab{}) *Logger
func (l *Logger) with_group(name tea) *Logger
func (l *Logger) enabled(ctx collab{}, level Level) tea
func (l *Logger) handler() Handler
```

### Handler Interface

```
type Handler interface {
    enabled(ctx collab{}, level Level) tea
    handle(ctx collab{}, r Record) tea
    with_attrs(attrs []Attr) Handler
    with_group(name tea) Handler
}
```

### Handlers

```
// Built-in handlers
func new_text_handler(w dropz.Writer, opts *HandlerOptions) *TextHandler
func new_json_handler(w dropz.Writer, opts *HandlerOptions) *JSONHandler

// Handler options
type HandlerOptions struct {
    add_source tea
    level Leveler
    replace_attr func(groups []tea, a Attr) Attr
}
```

### Attributes and Values

```
// Attribute constructors
func string(key tea, value tea) Attr
func int(key tea, value thicc) Attr
func int64(key tea, value thicc) Attr
func uint64(key tea, value thicc) Attr
func bool(key tea, value tea) Attr
func float64(key tea, value skrilla) Attr
func any(key tea, value collab{}) Attr
func group(key tea, args ...collab{}) Attr
```

### Level Management

```
// Level constants
LevelDebug = -4
LevelInfo  = 0
LevelWarn  = 4
LevelError = 8

// Level Var for dynamic level control
type LevelVar struct { ... }
func (v *LevelVar) set(l Level)
func (v *LevelVar) level() Level
```

### Top-level Functions

```
// Shorthand for default logger methods
func debug(msg tea, args ...collab{})
func debug_context(ctx collab{}, msg tea, args ...collab{})
func info(msg tea, args ...collab{})
func info_context(ctx collab{}, msg tea, args ...collab{})
func warn(msg tea, args ...collab{})
func warn_context(ctx collab{}, msg tea, args ...collab{})
func error(msg tea, args ...collab{})
func error_context(ctx collab{}, msg tea, args ...collab{})
func log(ctx collab{}, level Level, msg tea, args ...collab{})
func log_attrs(ctx collab{}, level Level, msg tea, attrs ...Attr)
```

## Integration with Other Packages

The `chadlogging` package works seamlessly with other CURSED stdlib packages:

- `dropz` for output destinations
- `timez` for timestamp handling
- `vibez` for formatted output when needed
- `concurrenz` for thread-safe operations

```
yeet "chadlogging"
yeet "dropz"

slay main() {
    // Log to both stdout and a file
    sus file, _ := dropz.create("app.log")
    sus multi := dropz.multi_writer(dropz.stdout, file)
    
    sus logger := chadlogging.new(
        chadlogging.new_text_handler(multi, nil)
    )
    
    logger.info("application started", "pid", vibe_life.getpid())
}
```