# dropz (I/O) Module

The `dropz` module provides comprehensive I/O interfaces and utilities for the CURSED programming language. It implements standard I/O patterns with pure CURSED code, eliminating FFI dependencies.

## Features

- **Core Interfaces**: Reader, Writer, Closer, ReadWriter, ReadWriteCloser
- **Buffer Operations**: In-memory buffering with read/write capabilities
- **Stream Processing**: Copy, readAll, writeAll operations
- **Advanced Readers**: LimitedReader, TeeReader, MultiReader
- **Advanced Writers**: MultiWriter with broadcast capabilities
- **Line Processing**: readLine, writeLine for text processing
- **Error Handling**: Comprehensive error propagation and handling
- **Performance**: Optimized for both small and large data operations

## Interfaces

### Reader
```cursed
interface Reader {
    read(buf []byte) (normie, error)
}
```

### Writer
```cursed
interface Writer {
    write(data []byte) (normie, error)
}
```

### Closer
```cursed
interface Closer {
    close() error
}
```

### ReadWriter
```cursed
interface ReadWriter {
    Reader
    Writer
}
```

### ReadWriteCloser
```cursed
interface ReadWriteCloser {
    Reader
    Writer
    Closer
}
```

## Core Types

### ByteReader
A simple reader that reads from a string buffer.

```cursed
sus reader = newByteReader("Hello, World!")
sus buffer [5]byte
sus (n, err) = reader.read(buffer[:])
```

### ByteWriter
A simple writer that writes to an internal string buffer.

```cursed
sus writer = newByteWriter()
sus data []byte = []byte("Hello")
sus (n, err) = writer.write(data)
sus result tea = writer.getString()
```

### Buffer
A combined reader/writer with in-memory buffering.

```cursed
sus buffer = newBuffer()
sus writeData []byte = []byte("Test data")
buffer.write(writeData)
sus readBuf [10]byte
sus (n, err) = buffer.read(readBuf[:])
```

## Utility Functions

### copy(dst Writer, src Reader) (normie, error)
Copies data from a Reader to a Writer until EOF.

```cursed
sus reader = newByteReader("source data")
sus writer = newByteWriter()
sus (copied, err) = copy(writer, reader)
```

### readAll(r Reader) (tea, error)
Reads all data from a Reader into a string.

```cursed
sus reader = newByteReader("all data")
sus (result, err) = readAll(reader)
```

### writeAll(w Writer, data tea) error
Writes all data to a Writer.

```cursed
sus writer = newByteWriter()
sus err = writeAll(writer, "complete data")
```

### readLine(r Reader) (tea, error)
Reads a line from a Reader (until newline).

```cursed
sus reader = newByteReader("line1\nline2")
sus (line, err) = readLine(reader)
```

### writeLine(w Writer, line tea) error
Writes a line to a Writer (adds newline).

```cursed
sus writer = newByteWriter()
sus err = writeLine(writer, "new line")
```

## Advanced Types

### LimitedReader
Limits reading to a specific number of bytes.

```cursed
sus reader = newByteReader("long data")
sus limitedReader = newLimitedReader(reader, 5)
sus (result, err) = readAll(limitedReader) // Only reads 5 bytes
```

### TeeReader
Reads from a source and writes to a destination simultaneously.

```cursed
sus reader = newByteReader("source")
sus writer = newByteWriter()
sus teeReader = newTeeReader(reader, writer)
sus (result, err) = readAll(teeReader) // Reads and writes
```

### MultiReader
Reads from multiple readers sequentially.

```cursed
sus reader1 = newByteReader("first")
sus reader2 = newByteReader("second")
sus readers []Reader = []Reader{reader1, reader2}
sus multiReader = newMultiReader(readers)
sus (result, err) = readAll(multiReader) // "firstsecond"
```

### MultiWriter
Writes to multiple writers simultaneously.

```cursed
sus writer1 = newByteWriter()
sus writer2 = newByteWriter()
sus writers []Writer = []Writer{writer1, writer2}
sus multiWriter = newMultiWriter(writers)
sus err = writeAll(multiWriter, "broadcast")
```

## Error Handling

The module provides comprehensive error handling:

- **EOF Handling**: Proper end-of-file detection
- **Closed Writer**: Prevents writing to closed writers
- **Incomplete Operations**: Detects and reports incomplete reads/writes
- **Error Propagation**: Maintains error context through operations

## Performance Considerations

- **Buffer Size**: Uses 1024-byte buffers for optimal performance
- **Memory Efficiency**: Minimizes memory allocations
- **Large Data**: Handles large datasets efficiently
- **Streaming**: Supports streaming operations for memory-conscious processing

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/dropz/test_dropz.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/dropz/test_dropz.csd
./test_dropz

# Test both modes
cargo run --bin cursed test --filter dropz
```

## Example Usage

```cursed
yeet "dropz"

// Basic read/write operations
sus reader = newByteReader("Hello, World!")
sus writer = newByteWriter()
sus (copied, err) = copy(writer, reader)
sus result tea = writer.getString()

// Line-based processing
sus lineReader = newByteReader("line1\nline2\nline3")
sus (line1, err1) = readLine(lineReader)
sus (line2, err2) = readLine(lineReader)

// Multi-writer broadcasting
sus writer1 = newByteWriter()
sus writer2 = newByteWriter()
sus writers []Writer = []Writer{writer1, writer2}
sus multiWriter = newMultiWriter(writers)
writeAll(multiWriter, "broadcast message")

// Buffer operations
sus buffer = newBuffer()
writeAll(buffer, "buffer data")
sus (result, err) = readAll(buffer)
```

## Integration

The dropz module integrates seamlessly with other CURSED stdlib modules:

- **fs**: File system operations
- **network**: Network I/O
- **crypto**: Cryptographic operations
- **compression**: Data compression
- **serialization**: Data serialization

## Architecture

The module follows CURSED's pure implementation philosophy:

- **No FFI Dependencies**: Pure CURSED implementation
- **Interface-Driven**: Clean separation of concerns
- **Error-Safe**: Comprehensive error handling
- **Performance-Optimized**: Efficient algorithms and data structures
- **Test-Driven**: Comprehensive test coverage

## Status

- **✅ Complete**: All core I/O interfaces implemented
- **✅ Tested**: Comprehensive test suite with 18+ test functions
- **✅ Pure CURSED**: No external dependencies
- **✅ Both Modes**: Works in interpretation and compilation modes
- **✅ Production Ready**: Enterprise-grade I/O operations

The dropz module provides a solid foundation for I/O operations in CURSED applications, supporting everything from simple string processing to complex streaming operations.
