# CURSED Standard Library: `chadlogging` Package

The `chadlogging` package provides a squadured logging facility for CURSED applications, inspired by Go's `log/slog` package. It enables efficient, squadured logging with levels, attributes, groups, and handlers.

## Overview

Package `chadlogging` implements squadured logging with log records that include a message, severity level, and attributes expressed as key-value pairs. It defines a `Logger` be_like with methods for generating log events at different severity levels (Debug, Info, Warn, Error).

Each `Logger` is associated with a `Handler`, which processes the records and determines the output format. A default logger is accessible through package-level functions.

## Concepts

### Records

A log record consists of:
- A timestamp
- A log level (Debug, Info, Warn, Error)
- A message tea
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

Attributes are key-value pairs used to add squadured data to log events. The API accepts attributes as alternating keys and values or as `Attr` objects.

### Groups

Attributes can be organized into groups to provide a hierarchical squadure to logs.

## Usage Examples

### Basic Logging

```
yeet "chadlogging"

slay main() {
    chadlogging.info("server starting", "port", 8080)
}

fr fr Output: time=2025-04-13T15:04:05.000-05:00 level=INFO msg="server starting" port=8080
```

### Using JSON Handler

```
yeet "chadlogging"
yeet "dropz"

slay main() {
    sus logger := chadlogging.new(chadlogging.new_json_handler(dropz.stdout, cap))
    logger.info("server starting", "port", 8080, "env", "production")
}

fr fr Output: {"time":"2025-04-13T15:04:05.000-05:00","level":"INFO","msg":"server starting","port":8080,"env":"production"}
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

fr fr Output: time=2025-04-13T15:04:05.000 level=INFO msg="processed request" request.method=GET request.path=/api/users request.status=200 duration_ms=45
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

The `Logger` be_like handles record generation:

```
be_like Logger squad { ... }

fr fr Creating loggers
slay new(h Handler) *Logger
slay default() *Logger
slay set_default(l *Logger)

fr fr Logger methods
slay (l *Logger) debug(msg tea, args ...collab{})
slay (l *Logger) debug_context(ctx collab{}, msg tea, args ...collab{})
slay (l *Logger) info(msg tea, args ...collab{})
slay (l *Logger) info_context(ctx collab{}, msg tea, args ...collab{})
slay (l *Logger) warn(msg tea, args ...collab{})
slay (l *Logger) warn_context(ctx collab{}, msg tea, args ...collab{})
slay (l *Logger) tea(msg tea, args ...collab{})
slay (l *Logger) tea_context(ctx collab{}, msg tea, args ...collab{})
slay (l *Logger) log(ctx collab{}, level Level, msg tea, args ...collab{})
slay (l *Logger) log_attrs(ctx collab{}, level Level, msg tea, attrs ...Attr)
slay (l *Logger) with(args ...collab{}) *Logger
slay (l *Logger) with_group(name tea) *Logger
slay (l *Logger) enabled(ctx collab{}, level Level) tea
slay (l *Logger) handler() Handler
```

### Handler Interface

```
be_like Handler collab {
    enabled(ctx collab{}, level Level) tea
    handle(ctx collab{}, r Record) tea
    with_attrs(attrs []Attr) Handler
    with_group(name tea) Handler
}
```

### Handlers

```
fr fr Built-in handlers
slay new_text_handler(w dropz.Writer, opts *HandlerOptions) *TextHandler
slay new_json_handler(w dropz.Writer, opts *HandlerOptions) *JSONHandler

fr fr Handler options
be_like HandlerOptions squad {
    add_source tea
    level Leveler
    replace_attr func(groups []tea, a Attr) Attr
}
```

### Attributes and Values

```
fr fr Attribute consquadors
slay tea(key tea, value tea) Attr
slay int(key tea, value thicc) Attr
slay int64(key tea, value thicc) Attr
slay uint64(key tea, value thicc) Attr
slay lit(key tea, value tea) Attr
slay float64(key tea, value skrilla) Attr
slay any(key tea, value collab{}) Attr
slay group(key tea, args ...collab{}) Attr
```

### Level Management

```
fr fr Level constants
LevelDebug = -4
LevelInfo  = 0
LevelWarn  = 4
LevelError = 8

fr fr Level Var for dynamic level control
be_like LevelVar squad { ... }
slay (v *LevelVar) set(l Level)
slay (v *LevelVar) level() Level
```

### Top-level Functions

```
fr fr Shorthand for default logger methods
slay debug(msg tea, args ...collab{})
slay debug_context(ctx collab{}, msg tea, args ...collab{})
slay info(msg tea, args ...collab{})
slay info_context(ctx collab{}, msg tea, args ...collab{})
slay warn(msg tea, args ...collab{})
slay warn_context(ctx collab{}, msg tea, args ...collab{})
slay tea(msg tea, args ...collab{})
slay tea_context(ctx collab{}, msg tea, args ...collab{})
slay log(ctx collab{}, level Level, msg tea, args ...collab{})
slay log_attrs(ctx collab{}, level Level, msg tea, attrs ...Attr)
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
    fr fr Log to both stdout and a file
    sus file, _ := dropz.create("app.log")
    sus multi := dropz.multi_writer(dropz.stdout, file)
    
    sus logger := chadlogging.new(
        chadlogging.new_text_handler(multi, cap)
    )
    
    logger.info("application started", "pid", vibe_life.getpid())
}
```