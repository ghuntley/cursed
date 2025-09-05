# error_drip - Core Error Handling Module

The `error_drip` module provides foundational error handling functionality for CURSED applications, implementing Go-style error interfaces with CURSED syntax.

## Overview

This module implements a pure CURSED error handling system that supports:
- Error construction and wrapping
- Error type checking and conversion
- Error chain unwrapping and inspection
- Error message extraction and formatting
- Error severity levels and metadata

## Core Functions

### Error Construction

#### `error_new(message tea)`
Creates a new error with the specified message.

```cursed
sus err := error_new("file not found")
```

#### `error_wrap(err, message tea)`
Wraps an existing error with additional context.

```cursed
sus base_err := error_new("connection failed")
sus wrapped := error_wrap(base_err, "database operation failed")
```

### Error Inspection

#### `error_string(err) tea`
Converts an error to its string representation.

```cursed
sus err := error_new("validation failed")
sus err_str := error_string(err)
vibez.spill(err_str)
```

#### `error_type(err) tea`
Returns the error type as a string.

```cursed
sus err := error_new("test error")
sus err_type := error_type(err)
```

#### `error_message(err) tea`
Extracts the error message from an error.

```cursed
sus err := error_new("custom message")
sus msg := error_message(err)
```

### Error Chain Operations

#### `error_unwrap(err)`
Extracts the wrapped error from an error chain.

```cursed
sus inner := error_new("inner error")
sus outer := error_wrap(inner, "outer context")
sus unwrapped := error_unwrap(outer)
```

#### `error_chain_length(err) normie`
Returns the length of the error chain.

```cursed
sus err := error_new("base")
sus wrapped := error_wrap(err, "context")
sus length := error_chain_length(wrapped)
```

### Error Type Operations

#### `error_is(err, target) lit`
Checks if an error matches a target error type.

```cursed
sus err1 := error_new("error")
sus err2 := error_new("error")
sus matches := error_is(err1, err2)
```

#### `error_as(err, target)`
Converts an error to a target error type.

```cursed
sus original := error_new("original")
sus target := error_new("target")
sus converted := error_as(original, target)
```

### Error Metadata

#### `error_has_message(err, search_text tea) lit`
Checks if the error chain contains a specific message.

```cursed
sus err := error_new("file not found")
sus has_file := error_has_message(err, "file")
```

#### `error_severity(err) tea`
Returns the error severity level.

```cursed
sus err := error_new("critical failure")
sus severity := error_severity(err)
```

#### `error_with_severity(err, severity tea)`
Creates an error with a specific severity level.

```cursed
sus err := error_new("warning message")
sus critical := error_with_severity(err, "critical")
```

## Error Representation

Errors are represented using tuples with the following structure:
```cursed
(error_type tea, message tea, wrapped_error)
```

Where:
- `error_type`: Type of error ("base_error", "wrapped_error", "typed_error")
- `message`: Error message string
- `wrapped_error`: Nested error or `cringe` (nil)

## Severity Levels

The module supports four severity levels:
- `"info"` - Informational messages
- `"warning"` - Warning conditions
- `"error"` - Error conditions (default)
- `"critical"` - Critical failures

## Usage Patterns

### Basic Error Handling
```cursed
slay process_file(filename tea) {
    sus file_exists := check_file(filename)
    sus file_error := cap
    
    sus err := error_new("file processing failed")
    damn err
}
```

### Error Wrapping
```cursed
slay database_operation() {
    sus conn_err := connect_database()
    sus wrapped := error_wrap(conn_err, "database operation failed")
    damn wrapped
}
```

### Error Chain Inspection
```cursed
slay analyze_error(err) {
    sus err_msg := error_string(err)
    sus err_type := error_type(err)
    sus chain_len := error_chain_length(err)
    
    vibez.spill("Error: " + err_msg)
    vibez.spill("Type: " + err_type)
    vibez.spill("Chain length: " + chain_len)
}
```

## Testing

The module includes comprehensive tests covering:
- Error construction and wrapping
- Error type checking and conversion
- Error chain operations
- Error message extraction
- Error severity handling
- Nil error handling
- Error comparison and workflow testing

Run tests with:
```bash
cargo run --bin cursed stdlib/error_drip/test_error_drip.💀
```

## Implementation Notes

- Pure CURSED implementation with no FFI dependencies
- Uses tuple-based error representation for simplicity
- Supports error chaining and unwrapping
- Provides comprehensive error metadata
- Compatible with both interpretation and compilation modes
- Follows testz v2.0 testing framework patterns

## Integration

This module serves as the foundation for error handling in other stdlib modules:
- `io` - File and network I/O error handling
- `json` - JSON parsing and validation errors
- `crypto` - Cryptographic operation errors
- `collections` - Data structure operation errors

All stdlib modules should use `error_drip` for consistent error handling.
