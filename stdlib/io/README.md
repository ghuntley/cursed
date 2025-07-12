# Pure CURSED I/O Module

Comprehensive I/O operations implemented in pure CURSED without FFI dependencies.

## Features

### Console I/O Operations
- `console_print(message)` - Print message to console
- `console_println(message)` - Print message with newline
- `console_print_int(value)` - Print integer value
- `console_print_float(value)` - Print float value
- `console_print_bool(value)` - Print boolean value

### Buffered I/O Operations
- `create_buffer(capacity)` - Create I/O buffer
- `buffer_write(buffer, data)` - Write data to buffer
- `buffer_read(buffer, length)` - Read data from buffer
- `buffer_flush(buffer)` - Flush buffer contents

### Stream I/O Operations
- `create_stream(id, readable, writable)` - Create I/O stream
- `stream_write(stream, data)` - Write data to stream
- `stream_read(stream, length)` - Read data from stream
- `stream_seek(stream, position)` - Seek to position in stream

### File I/O Operations
- `file_write(filename, content)` - Write content to file
- `file_read(filename)` - Read content from file
- `file_exists(filename)` - Check if file exists
- `file_delete(filename)` - Delete file

### Interactive I/O Operations
- `prompt_user(message)` - Prompt user for input
- `confirm_user(message)` - Ask user for confirmation
- `select_option(message, options)` - Present multiple choice

## Types

### IOResult
Result type for I/O operations containing:
- `success` - Operation success status
- `data` - Operation result data
- `error` - Error message if failed

### IOBuffer
Buffer for efficient I/O operations:
- `data` - Buffer content
- `capacity` - Buffer capacity
- `position` - Current position
- `size` - Current size

### IOStream
Stream handle for I/O operations:
- `id` - Stream identifier
- `buffer` - Stream buffer
- `position` - Current position
- `size` - Stream size
- `readable` - Read permission
- `writable` - Write permission

## Usage Examples

### Basic Console I/O
```cursed
yeet "io"

sus result IOResult = console_println("Hello, World!")
bestie result.success {
    console_println("Print successful")
}
```

### Buffered I/O
```cursed
yeet "io"

sus buffer IOBuffer = create_buffer(1024)
sus write_result IOResult = buffer_write(buffer, "Hello Buffer")
sus read_result IOResult = buffer_read(buffer, 5)
```

### Stream I/O
```cursed
yeet "io"

sus stream IOStream = create_stream(1, based, based)
sus write_result IOResult = stream_write(stream, "Hello Stream")
sus read_result IOResult = stream_read(stream, 5)
```

### File I/O
```cursed
yeet "io"

sus write_result IOResult = file_write("test.txt", "Hello File")
sus read_result IOResult = file_read("test.txt")
```

### Interactive I/O
```cursed
yeet "io"

sus name_result IOResult = prompt_user("Enter your name")
sus confirm_result IOResult = confirm_user("Are you sure?")
sus options []tea = ["Yes", "No", "Maybe"]
sus select_result IOResult = select_option("Choose", options)
```

## Legacy Compatibility

The module provides legacy compatibility functions that match the original API:
- `print()`, `println()`, `print_int()`, `print_float()`, `print_bool()`
- `read_line()`, `read_int()`
- `write_file()`, `read_file()`

## Architecture

This implementation eliminates all FFI dependencies from the original Rust code:
- No `std::io` usage
- Pure CURSED data structures
- Runtime integration through language primitives
- Comprehensive error handling with `IOResult` type

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/io/test_io.csd
```

Test both interpretation and compilation modes:
```bash
cargo run --bin cursed stdlib/io/test_io.csd
cargo run --bin cursed -- compile stdlib/io/test_io.csd
./test_io
```

## Migration from Rust Implementation

This module replaces the Rust implementations in:
- `src/stdlib/io/console.rs` → Pure CURSED console operations
- `src/stdlib/io/buffered.rs` → Pure CURSED buffered operations
- `src/stdlib/io/streams.rs` → Pure CURSED stream operations
- `src/stdlib/io/interactive.rs` → Pure CURSED interactive operations

All functionality is preserved while eliminating FFI dependencies.
