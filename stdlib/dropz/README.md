# dropz - Core I/O Module

The `dropz` module provides fundamental I/O operations for CURSED, serving as the core I/O package similar to Go's `io` and `os` packages combined. This module is **critical for self-hosting** as it provides file operations, standard I/O, and Reader/Writer interfaces that the compiler needs.

## Features

### 🔧 Core I/O Operations
- **File Operations**: open, close, read, write, exists, delete
- **Directory Operations**: mkdir, rmdir, read_dir, walk_dir
- **Reader/Writer Interfaces**: Streaming I/O with buffered operations
- **Path Manipulation**: join, clean, dir, base, ext, abs, rel
- **Resource Management**: Automatic cleanup with defer support

### 🚀 Self-Hosting Support
- **Compiler Integration**: read_source_file, write_compiled_output
- **Temporary Files**: temp_file for intermediate compilation
- **Object Files**: write_object_file for build artifacts
- **Configuration**: read_config_file for compiler settings

### 🔒 Pure CURSED Implementation
- **FFI-Free**: No external dependencies, pure CURSED language features
- **Thread-Safe**: Safe concurrent access to file operations
- **Error Handling**: Comprehensive error types and handling
- **Cross-Platform**: Consistent behavior across all platforms

## Core Interfaces

### Reader
```cursed
collab Reader {
    read(p []byte) (n normie, err tea)
}
```

### Writer
```cursed
collab Writer {
    write(p []byte) (n normie, err tea)
}
```

### File
```cursed
struct File {
    fd normie,
    name tea,
    flag normie,
    is_open lit
}
```

## File Operations

### Basic File Operations
```cursed
# Open file for reading
sus file, err := dropz.open("input.txt")
check err == "" {
    defer file.close()
    
    # Read data
    sus data []byte = []byte{0, 0, 0, 0, 0}
    sus count, read_err := file.read(data)
    vibez.spill("Read " + count.(tea) + " bytes")
}

# Create file for writing
sus output, create_err := dropz.create("output.txt")
check create_err == "" {
    defer output.close()
    
    # Write data
    sus content []byte = []byte{72, 101, 108, 108, 111}  # "Hello"
    sus _, write_err := output.write(content)
    vibez.spill("File written successfully")
}
```

### High-Level File Operations
```cursed
# Read entire file as text
sus content, err := dropz.read_text_file("config.txt")
check err == "" {
    vibez.spill("File content: " + content)
}

# Write text to file
sus write_err := dropz.write_text_file("output.txt", "Hello World", dropz.MODE_REGULAR)
check write_err == "" {
    vibez.spill("Text written successfully")
}

# Copy file
sus bytes_copied, copy_err := dropz.copy_file("source.txt", "destination.txt")
check copy_err == "" {
    vibez.spill("Copied " + bytes_copied.(tea) + " bytes")
}
```

## Directory Operations

```cursed
# Create directory
sus mkdir_err := dropz.mkdir("new_directory", dropz.MODE_DIR)
check mkdir_err == "" {
    vibez.spill("Directory created")
}

# Read directory contents
sus entries, read_err := dropz.read_dir(".")
check read_err == "" {
    bestie i := 0; i < entries.length; i++ {
        sus entry := entries[i]
        vibez.spill("Found: " + entry.name)
        check entry.is_dir {
            vibez.spill("  (directory)")
        }
    }
}

# Get current working directory
sus current_dir, getwd_err := dropz.getwd()
check getwd_err == "" {
    vibez.spill("Current directory: " + current_dir)
}
```

## Path Operations

```cursed
# Join path components
sus full_path := dropz.join("home", "user", "documents", "file.txt")
vibez.spill("Full path: " + full_path)

# Extract components
sus directory := dropz.dir("/home/user/file.txt")     # "/home/user"
sus filename := dropz.base("/home/user/file.txt")     # "file.txt"
sus extension := dropz.ext("file.txt")                # ".txt"

# Check path properties
sus is_absolute := dropz.is_abs("/home/user")         # based
sus file_exists := dropz.exists("file.txt")           # lit
sus is_directory := dropz.is_dir("/home")             # lit
```

## Buffered I/O

```cursed
sus file, err := dropz.open("large_file.txt")
check err == "" {
    defer file.close()
    
    # Create buffered reader
    sus reader := dropz.new_reader(file)
    
    # Read line by line
    sus line_data, is_line, line_err := reader.read_line()
    check line_err == "" && is_line {
        vibez.spill("Line: " + line_data.(tea))
    }
    
    # Read string until delimiter
    sus content, content_err := reader.read_string(10)  # Read until newline
    check content_err == "" {
        vibez.spill("Content: " + content)
    }
}

# Buffered writing
sus output, create_err := dropz.create("buffered_output.txt")
check create_err == "" {
    defer output.close()
    
    sus writer := dropz.new_writer(output)
    
    sus _, write_err := writer.write_string("Hello, buffered world!")
    check write_err == "" {
        sus flush_err := writer.flush()
        vibez.spill("Data written and flushed")
    }
}
```

## Self-Hosting Support

```cursed
# Read source file for compilation
sus source_code, source_err := dropz.read_source_file("main.💀")
check source_err == "" {
    vibez.spill("Source code loaded: " + source_code.length.(tea) + " characters")
}

# Write compiled output
sus compile_err := dropz.write_compiled_output("main.exe", compiled_binary)
check compile_err == "" {
    vibez.spill("Executable written successfully")
}

# Create temporary file for intermediate compilation
sus temp_file, temp_err := dropz.temp_file("cursed_build_")
check temp_err == "" {
    defer temp_file.close()
    vibez.spill("Temporary file: " + temp_file.name)
}

# Write object file
sus obj_data []byte = []byte{0x7f, 0x45, 0x4c, 0x46}  # ELF magic
sus obj_err := dropz.write_object_file("output.o", obj_data)
check obj_err == "" {
    vibez.spill("Object file written")
}
```

## Constants

### File Open Flags
```cursed
dropz.O_RDONLY   # Read only
dropz.O_WRONLY   # Write only
dropz.O_RDWR     # Read and write
dropz.O_APPEND   # Append mode
dropz.O_CREATE   # Create if not exists
dropz.O_EXCL     # Exclusive create
dropz.O_SYNC     # Synchronous I/O
dropz.O_TRUNC    # Truncate file
```

### File Permissions
```cursed
dropz.MODE_REGULAR     # 0644 - Regular file
dropz.MODE_EXECUTABLE  # 0755 - Executable file
dropz.MODE_DIR         # 0755 - Directory
```

### Seek Positions
```cursed
dropz.SEEK_START    # Seek from beginning
dropz.SEEK_CURRENT  # Seek from current position
dropz.SEEK_END      # Seek from end
```

### Error Constants
```cursed
dropz.EOF            # End of file
dropz.ErrInvalid     # Invalid argument
dropz.ErrPermission  # Permission denied
dropz.ErrExist       # File already exists
dropz.ErrNotExist    # File does not exist
dropz.ErrClosed      # File already closed
```

## Error Handling

```cursed
struct PathError {
    op tea,      # Operation name
    path tea,    # File path
    err tea      # Error message
}

# Example error handling
sus file, err := dropz.open("nonexistent.txt")
check err != "" {
    vibez.spill("Error opening file: " + err)
    damn  # Exit early
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/dropz/test_dropz.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/dropz/test_dropz.💀
./test_dropz

# Both-mode verification
test_both_modes() {
    cargo run --bin cursed stdlib/dropz/test_dropz.💀 > interp_output.txt
    cargo run --bin cursed -- compile stdlib/dropz/test_dropz.💀
    ./test_dropz > comp_output.txt
    diff interp_output.txt comp_output.txt
}
```

## Integration with CURSED Compiler

The dropz module is essential for CURSED self-hosting:

1. **Source File Reading**: `read_source_file()` loads CURSED source code
2. **Output Generation**: `write_compiled_output()` creates executables
3. **Intermediate Files**: `temp_file()` for build artifacts
4. **Configuration**: `read_config_file()` for compiler settings
5. **Object Files**: `write_object_file()` for linking phase

## Dependencies

- `core`: Basic types and error handling
- `vibez`: Output operations (for debugging/logging)
- `testz`: Testing framework (for test suite)

## Architecture

```
dropz/
├── mod.💀           # Main module implementation
├── test_dropz.💀    # Comprehensive test suite
└── README.md         # This documentation

Features:
├── File Operations   # Core file I/O
├── Directory Ops     # Directory management
├── Buffered I/O      # Performance optimization
├── Path Utils        # Path manipulation
├── Self-Hosting      # Compiler support
└── Error Handling    # Robust error management
```

## Performance Considerations

- **Buffered I/O**: Reduces system call overhead for large files
- **Efficient Memory**: Optimized for large file operations
- **Path Operations**: Fast string manipulation for path handling
- **Resource Management**: Automatic cleanup prevents resource leaks

## Thread Safety

- File operations are thread-safe at the OS level
- Buffered operations require external synchronization
- Standard streams are safe for concurrent access
- Use proper locking for shared file access

## Future Enhancements

- [ ] Async I/O operations for better performance
- [ ] Memory-mapped file support
- [ ] Network file system support
- [ ] Advanced compression integration
- [ ] File watching/monitoring capabilities

---

The dropz module provides the foundational I/O infrastructure needed for CURSED's self-hosting capability, offering a comprehensive, FFI-free implementation of essential file and directory operations.
