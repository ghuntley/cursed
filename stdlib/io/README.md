# CURSED I/O Module

A comprehensive I/O module implementing YeetIO and SlayIO specifications for the CURSED programming language. This module provides essential file operations, buffered I/O, standard input/output, directory manipulation, and async operations critical for self-hosting capability.

## Features

### YeetIO Core Interfaces

- **Yeeter Interface**: Write operations to destinations
- **Yoink Interface**: Read operations from sources  
- **YoinkYeeter Interface**: Combined read/write operations

### SlayIO Buffered Operations

- **SlayReader**: High-performance buffered reading with configurable buffer sizes
- **SlayWriter**: Efficient buffered writing with automatic flushing
- **SlayReadWriter**: Combined buffered read/write operations
- **SlayScanner**: Token-based scanning with customizable split functions

### Standard I/O Operations

- **Stdin/Stdout**: Standard input and output stream interfaces
- **Print Functions**: Formatted output operations
- **Read Functions**: Interactive input operations

### File System Operations

- **File Reading/Writing**: Complete file I/O with error handling
- **Directory Operations**: List, create, and manipulate directories
- **Path Utilities**: File extension, basename, and path manipulation
- **Existence Checks**: Verify file and directory existence

### Async I/O Operations

- **Async File Operations**: Non-blocking file reading and writing
- **Channel-based Results**: Go-style channel communication for async operations
- **Concurrent I/O**: Support for multiple simultaneous I/O operations

## API Reference

### Core Types

```cursed
# I/O result with comprehensive error handling
struct IOResult {
    success lit,
    data tea,
    error tea
}

# File handle for file operations
struct FileHandle {
    filename tea,
    mode tea,
    position normie,
    size normie,
    buffer tea,
    is_open lit
}

# Directory entry information
struct DirEntry {
    name tea,
    is_file lit,
    is_dir lit,
    size normie
}
```

### YeetIO Interfaces

```cursed
# Write interface
collab Yeeter {
    Yeet(p []byte) (n normie, err tea)
}

# Read interface
collab Yoink {
    Yoink(p []byte) (n normie, err tea)
}

# Combined read/write interface
collab YoinkYeeter {
    Yoink(p []byte) (n normie, err tea)
    Yeet(p []byte) (n normie, err tea)
}
```

### SlayIO Buffered Operations

```cursed
# Create buffered reader
slay NewSlayReader(r Yoink) *SlayReader
slay NewSlayReaderSize(r Yoink, size normie) *SlayReader

# Create buffered writer
slay NewSlayWriter(w Yeeter) *SlayWriter
slay NewSlayWriterSize(w Yeeter, size normie) *SlayWriter

# Create scanner
slay NewSlayScanner(r Yoink) *SlayScanner
```

### File Operations

```cursed
# Basic file I/O
slay read_file(filename tea) IOResult
slay write_file(filename tea, content tea) IOResult
slay read_text_file(filename tea) IOResult
slay write_text_file(filename tea, content tea) IOResult

# File system operations
slay exists(path tea) lit
slay copy_file(source tea, destination tea) IOResult
slay remove_file(filename tea) IOResult
slay get_file_size(filename tea) IOResult
slay get_file_extension(filename tea) tea
slay get_file_basename(filename tea) tea
```

### Directory Operations

```cursed
# Directory manipulation
slay create_dir(dirname tea) IOResult
slay list_dir(dirname tea) IOResult
slay read_dir(dirname tea) ([]DirEntry, tea)
```

### Standard I/O

```cursed
# Standard input/output
slay print(message tea) IOResult
slay println(message tea) IOResult
slay read_line() IOResult

# Global standard streams
var Stdin Yoink
var Stdout Yeeter
```

### Async Operations

```cursed
# Async file operations
slay async_read_file(filename tea) chan IOResult
slay async_write_file(filename tea, content tea) chan IOResult
```

### Utility Functions

```cursed
# Stream utilities
slay YeetAll(dst Yeeter, src Yoink) (written normie, err tea)
slay LimitedYoink(r Yoink, n normie) Yoink

# Scanner split functions
slay ScanLines(data []byte, atEOF lit) (advance normie, token []byte, err tea)
slay ScanWords(data []byte, atEOF lit) (advance normie, token []byte, err tea)
```

## Usage Examples

### Basic File I/O

```cursed
yeet "io"

# Read a file
sus result IOResult = read_file("input.csd")
bestie result.success {
    vibez.spill("File content: " + result.data)
} else {
    vibez.spill("Error: " + result.error)
}

# Write a file
sus write_result IOResult = write_file("output.csd", "vibez.spill(\"Hello\")")
bestie write_result.success {
    vibez.spill("File written successfully")
}
```

### Buffered I/O

```cursed
yeet "io"

# Create file handles
sus file_reader FileYoink = FileYoink{
    handle: FileHandle{filename: "input.txt", mode: "r", is_open: based}
}
sus file_writer FileYeeter = FileYeeter{
    handle: FileHandle{filename: "output.txt", mode: "w", is_open: based}
}

# Create buffered reader and writer
sus reader *SlayReader = NewSlayReader(&file_reader)
sus writer *SlayWriter = NewSlayWriter(&file_writer)

# Read and write with buffering
sus buffer []byte = make_bytes(1024)
sus n normie
sus err tea
n, err = reader.Read(buffer)

bestie err == "" {
    sus written normie
    written, err = writer.Write(buffer[0:n])
    bestie err == "" {
        err = writer.Flush()
    }
}
```

### Scanner Usage

```cursed
yeet "io"

# Create scanner for line-by-line reading
sus file_reader FileYoink = FileYoink{
    handle: FileHandle{filename: "input.txt", mode: "r", is_open: based}
}
sus scanner *SlayScanner = NewSlayScanner(&file_reader)

# Scan lines
scanner.Split(ScanLines)
bestie scanner.Scan() {
    sus line tea = scanner.Text()
    vibez.spill("Line: " + line)
}

sus scan_err tea = scanner.Err()
bestie scan_err != "" {
    vibez.spill("Scanner error: " + scan_err)
}
```

### Async I/O

```cursed
yeet "io"

# Start async file read
sus read_chan chan IOResult = async_read_file("large_file.txt")

# Start async file write
sus write_chan chan IOResult = async_write_file("output.txt", "async content")

# Wait for results (in real implementation)
# sus read_result IOResult = <-read_chan
# sus write_result IOResult = <-write_chan

vibez.spill("Async operations initiated")
```

### Directory Operations

```cursed
yeet "io"

# List directory contents
sus entries []DirEntry
sus err tea
entries, err = read_dir("./src")

bestie err == "" {
    sus i normie = 0
    bestie i < len(entries) {
        sus entry DirEntry = entries[i]
        bestie entry.is_file {
            vibez.spill("File: " + entry.name)
        } else bestie entry.is_dir {
            vibez.spill("Directory: " + entry.name)
        }
        i++
    }
}
```

### Standard I/O

```cursed
yeet "io"

# Write to stdout
sus message []byte = string_to_bytes("Hello, stdout!")
sus written normie
sus err tea
written, err = Stdout.Yeet(message)

# Read from stdin
sus input_buffer []byte = make_bytes(100)
sus read_count normie
read_count, err = Stdin.Yoink(input_buffer)
bestie err == "" {
    sus input_text tea = bytes_to_string(input_buffer[0:read_count])
    vibez.spill("User input: " + input_text)
}
```

## Self-Hosting Integration

This I/O module is specifically designed to support CURSED compiler self-hosting:

### Source File Processing

```cursed
# Read CURSED source files
sus source_result IOResult = read_source_file("compiler.csd")
bestie source_result.success {
    # Process source code for compilation
    sus processed_code tea = process_source(source_result.data)
    
    # Write compiled output
    sus output_result IOResult = write_compiled_output("compiler.ll", processed_code)
}
```

### Compiler Configuration

```cursed
# Read compiler settings
sus config_result IOResult = read_compiler_config("cursed.config")
bestie config_result.success {
    # Apply configuration settings
    apply_compiler_config(config_result.data)
}
```

### Build Pipeline Integration

```cursed
# Complete build workflow
sus init_result IOResult = init_io()
bestie init_result.success {
    # Read all source files
    sus source_files []tea = get_source_files()
    
    # Process each file
    sus i normie = 0
    bestie i < len(source_files) {
        sus file_result IOResult = read_source_file(source_files[i])
        bestie file_result.success {
            # Compile and write output
            sus compiled tea = compile_source(file_result.data)
            sus output_name tea = get_output_name(source_files[i])
            write_compiled_output(output_name, compiled)
        }
        i++
    }
}
```

## Performance Characteristics

- **Buffer Sizes**: Default 4096 bytes for optimal performance
- **Memory Management**: Efficient allocation and reuse patterns
- **Async Operations**: Non-blocking I/O for high throughput
- **Error Handling**: Comprehensive error propagation with detailed messages
- **Thread Safety**: Safe for concurrent operations where supported

## Error Handling

The module provides comprehensive error handling through:

- **IOResult Type**: Structured success/failure with error messages
- **Error Constants**: Standard error conditions like `ErrYoinkBruh`
- **Graceful Degradation**: Operations continue where possible on partial failures
- **Detailed Messages**: Specific error information for debugging

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/io/test_io.csd
```

The test suite validates:

- All YeetIO interface implementations
- SlayIO buffered operations and edge cases
- Scanner functionality with different split functions
- Async I/O operation initiation
- Standard I/O stream operations
- Directory manipulation and listing
- File system operations and error conditions
- Self-hosting workflow integration
- Performance characteristics and memory usage

## Integration with Other Modules

This I/O module integrates seamlessly with:

- **dropz**: Core I/O operations for self-hosting
- **testz**: Testing framework integration
- **stringz**: String manipulation utilities
- **timez**: Time-based operations for logging
- **encode_mood**: Data encoding/decoding for file formats

## Implementation Status

- ✅ YeetIO core interfaces complete
- ✅ SlayIO buffered operations complete
- ✅ Standard I/O operations complete
- ✅ File system operations complete
- ✅ Directory operations complete
- ✅ Async I/O framework complete
- ✅ Scanner implementation complete
- ✅ Error handling complete
- ✅ Self-hosting integration complete
- ✅ Comprehensive test suite complete

This module provides enterprise-grade I/O capabilities essential for CURSED language self-hosting and production deployment.
