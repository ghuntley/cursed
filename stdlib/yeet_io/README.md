# YeetIO Module

YeetIO provides the core interfaces and primitives for handling input/output operations in CURSED with a Gen Z twist. It's inspired by Go's io package but adapted for the CURSED language ecosystem.

## Overview

The YeetIO module implements fundamental I/O operations through a set of interfaces and utility functions that enable efficient data transfer between different sources and destinations.

## Key Interfaces

### `Yeeter` Interface
Equivalent to Go's `io.Writer`. Yeets (writes) data to a destination.

```cursed
collab Yeeter {
    Yeet(p []byte) (n normie, err tea)
}
```

### `Yoink` Interface
Equivalent to Go's `io.Reader`. Yoinks (reads) data from a source.

```cursed
collab Yoink {
    Yoink(p []byte) (n normie, err tea)
}
```

### `YoinkYeeter` Interface
Combines `Yoink` and `Yeeter` interfaces (equivalent to `io.ReadWriter`).

```cursed
collab YoinkYeeter {
    Yoink(p []byte) (n normie, err tea)
    Yeet(p []byte) (n normie, err tea)
}
```

## Implementations

### StringYeeter
A simple string-based writer that accumulates data in a string buffer.

```cursed
sus sy StringYeeter = new_string_yeeter()
sus data []byte = []byte("hello world")
sus n normie, err tea = sy.Yeet(data)
sus content tea = sy.get_data()  # Returns "hello world"
```

### ByteYoink
A byte slice reader that reads data from a byte buffer.

```cursed
sus by ByteYoink = new_byte_yoink("hello world")
sus buf []byte = make([]byte, 5)
sus n normie, err tea = by.Yoink(buf)
# buf now contains "hello"
```

### LimitedYoink
A reader that stops with EOF after n bytes.

```cursed
sus source ByteYoink = new_byte_yoink("hello world")
sus limited LimitedYoink = LimitedYoink(source, 5)
sus buf []byte = make([]byte, 10)
sus n normie, err tea = limited.Yoink(buf)
# Only reads first 5 bytes: "hello"
```

### MultiYeeter
Writes to multiple Yeeters simultaneously.

```cursed
sus dest1 StringYeeter = new_string_yeeter()
sus dest2 StringYeeter = new_string_yeeter()
sus multi MultiYeeter = new_multi_yeeter(dest1, dest2)
sus data []byte = []byte("broadcast")
sus n normie, err tea = multi.Yeet(data)
# Both dest1 and dest2 now contain "broadcast"
```

### BufferedYoink
Buffers reads for better performance.

```cursed
sus source ByteYoink = new_byte_yoink("buffered read test")
sus buffered BufferedYoink = new_buffered_yoink(source, 1024)
sus buf []byte = make([]byte, 10)
sus n normie, err tea = buffered.Yoink(buf)
```

## Utility Functions

### `YeetAll`
Copies all data from a Yoink to a Yeeter (like io.Copy).

```cursed
sus source ByteYoink = new_byte_yoink("copy this data")
sus dest StringYeeter = new_string_yeeter()
sus written thicc, err tea = YeetAll(dest, source)
```

### `YeetString`
Writes a string to a Yeeter.

```cursed
sus dest StringYeeter = new_string_yeeter()
sus n normie, err tea = YeetString(dest, "test string")
```

### `YoinkAll`
Reads all data from a Yoink until EOF.

```cursed
sus source ByteYoink = new_byte_yoink("read all this")
sus content tea, err tea = YoinkAll(source)
```

### `YeetLine`
Writes a line with newline character.

```cursed
sus dest StringYeeter = new_string_yeeter()
sus n normie, err tea = YeetLine(dest, "test line")
# Writes "test line\n"
```

### `IsEOF`
Checks if an error represents end-of-file.

```cursed
if IsEOF(err) {
    vibez.spill("Reached end of data")
}
```

## Error Handling

### `ErrYoinkBruh`
Equivalent to `io.EOF` - indicates end of input stream.

```cursed
sus ErrYoinkBruh tea = "no more to yoink, bruh"
```

This error is returned when there's no more data to read from a source.

## Usage Examples

### Basic Read/Write Operations

```cursed
yeet "yeet_io"

# Create a string writer
sus writer StringYeeter = new_string_yeeter()

# Write some data
sus data []byte = []byte("Hello, CURSED!")
sus n normie, err tea = writer.Yeet(data)

# Get the written data
sus result tea = writer.get_data()
vibez.spill(result)  # Prints: Hello, CURSED!
```

### Copy Operations

```cursed
yeet "yeet_io"

# Create source and destination
sus source ByteYoink = new_byte_yoink("Data to copy")
sus dest StringYeeter = new_string_yeeter()

# Copy all data
sus written thicc, err tea = YeetAll(dest, source)

vibez.spill("Copied bytes:", written)
vibez.spill("Result:", dest.get_data())
```

### Buffered Reading

```cursed
yeet "yeet_io"

# Create a buffered reader
sus source ByteYoink = new_byte_yoink("Large amount of data to read efficiently")
sus buffered BufferedYoink = new_buffered_yoink(source, 1024)

# Read in chunks
sus buf []byte = make([]byte, 10)
bestie {
    sus n normie, err tea = buffered.Yoink(buf)
    if n > 0 {
        vibez.spill("Read:", string(buf[:n]))
    }
    if IsEOF(err) {
        ghosted
    }
}
```

## Design Principles

1. **Performance-focused**: Minimizes allocations and copies
2. **Clear error handling**: Consistent error reporting with meaningful messages
3. **Thread-safe**: All implementations are designed to be thread-safe where possible
4. **Backward compatibility**: Maintains compatibility with existing CURSED I/O patterns
5. **Pure CURSED**: No FFI dependencies - fully implemented in CURSED

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/yeet_io/test_yeet_io.💀
```

The test suite includes:
- Basic functionality tests for all interfaces
- Error handling edge cases
- Performance and buffering tests
- Interface compliance verification
- Multiple read/write scenarios

## Integration

YeetIO is designed to work seamlessly with other stdlib packages and serves as a foundation for higher-level I/O operations in packages like `web_vibez`, `dropz`, and file system operations.

The module follows CURSED's naming conventions and integrates with the testz testing framework for comprehensive verification.
