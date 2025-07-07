# CURSED I/O Library Tests

This directory contains comprehensive tests for the CURSED I/O standard library.

## Test Coverage

The `test_io.csd` file provides complete test coverage for all I/O functions:

### Console I/O
- `print()` / `println()` - Standard output
- `eprint()` / `eprintln()` - Error output
- `printf()` - Formatted output
- `read_line()` - Read line from input
- `read_char()` - Read single character
- `read_int()` / `read_float()` - Read typed input

### File Operations
- `write_file()` / `read_file()` - Text file I/O
- `write_file_bytes()` / `read_file_bytes()` - Binary file I/O
- `append_file()` - Append to existing file
- `copy_file()` / `move_file()` - File operations
- `delete_file()` - File deletion
- `file_exists()` - Check file existence
- `file_size()` - Get file size
- `is_file()` / `is_directory()` - Type checking

### Directory Operations
- `create_directory()` - Create single directory
- `create_directory_recursive()` - Create directory tree
- `remove_directory()` / `remove_directory_recursive()` - Directory removal
- `list_directory()` / `list_directory_recursive()` - Directory listing
- `current_directory()` / `change_directory()` - Working directory

### Path Operations
- `path_join()` - Join path components
- `path_dirname()` / `path_basename()` - Path components
- `path_extension()` - File extension
- `path_absolute()` / `path_relative()` - Path conversion
- `path_exists()` - Check path existence

### Stream I/O
- `open_file_read()` / `open_file_write()` / `open_file_append()` - File handles
- `close_file()` - Close file handle
- `read_from_file()` / `write_to_file()` - Stream operations
- `flush_file()` - Force write
- `seek_file()` / `tell_file()` - File positioning

### Buffered I/O
- `create_buffer()` - Create I/O buffer
- `buffer_write()` / `buffer_read()` - Buffered operations
- `buffer_flush()` / `buffer_clear()` - Buffer management
- `buffer_size()` / `buffer_available()` - Buffer status

### Temporary Files
- `create_temp_file()` - Create temporary file
- `create_temp_directory()` - Create temporary directory
- `temp_directory()` - Get system temp directory

### File Metadata
- `file_modified_time()` / `file_created_time()` - Timestamps
- File size and existence checking
- Directory vs file type detection

### Edge Cases Tested
- Empty file operations
- Non-existent file handling
- Permission errors (system-dependent)
- Invalid path operations
- Large file handling
- Concurrent file access

## File System Operations

The I/O library provides cross-platform file system operations:

- **Text Files**: UTF-8 encoded text file reading/writing
- **Binary Files**: Raw byte array operations
- **Directories**: Recursive creation and deletion
- **Paths**: Cross-platform path manipulation
- **Streaming**: Efficient large file processing
- **Buffering**: Optimized I/O performance
- **Temporary Files**: Secure temporary file creation

## Running Tests

```bash
# Run I/O tests specifically
cargo run --bin cursed stdlib/io/test_io.csd

# Run all stdlib tests
cargo run --bin cursed test
```

## Test Results

All tests verify:
- Correct file reading/writing
- Proper directory operations
- Path manipulation accuracy
- Stream I/O functionality
- Buffer management
- Error handling for invalid operations
- Cross-platform compatibility

The tests create temporary files and directories for testing, which are cleaned up automatically.

## Platform Notes

- **File Permissions**: Some tests may behave differently on different operating systems
- **Path Separators**: Path operations handle platform-specific separators automatically
- **Case Sensitivity**: File system case sensitivity varies by platform
- **Temporary Directories**: System temp directory location varies by platform

## Important Considerations

- **Always close file handles** when done to prevent resource leaks
- **Check file existence** before attempting operations
- **Handle errors gracefully** for network drives or permission issues
- **Use buffered I/O** for better performance with large files
- **Clean up temporary files** when no longer needed
