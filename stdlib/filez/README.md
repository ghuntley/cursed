# filez Module - Pure CURSED File System Operations

The `filez` module provides comprehensive, cross-platform file and directory operations implemented in pure CURSED language. It offers production-ready file I/O capabilities with proper error handling, security validation, and atomic operations for system programming and application development.

## Features

### Core File Operations
- **File I/O**: Reading, writing, and appending text files with full error handling
- **Binary Operations**: Reading and writing binary data with size limits and safety checks  
- **File Management**: Copying, moving, renaming, and deleting files with validation
- **Metadata Access**: File size, modification time, permissions, and comprehensive file info
- **Atomic Operations**: Safe file operations with proper resource cleanup and sync support

### Directory Operations
- **Directory Management**: Creating, removing, and listing directories with error handling
- **Recursive Operations**: Deep directory copying with full subdirectory support
- **Working Directory**: Getting and setting current working directory safely  
- **Directory Validation**: Existence checking and type verification
- **Cross-Platform Paths**: Automatic path separator handling for all platforms

### Advanced Features
- **Line-Based I/O**: Reading and writing files as arrays of lines with proper newline handling
- **Binary File Support**: Byte array operations with configurable size limits and safety checks
- **Temporary Files**: System temp directory access and unique temporary file creation
- **File Synchronization**: Force file sync to disk for data persistence guarantees
- **Security Validation**: Filename validation to prevent path traversal and injection attacks

### Error Handling & Safety
- **Comprehensive Error Returns**: All operations return descriptive error messages on failure
- **Input Validation**: Rigorous checking of all filenames, paths, and content parameters
- **Resource Safety**: Automatic cleanup and proper handling of system resources
- **Atomic Operations**: Safe file operations that maintain consistency during failures

## Usage Examples

### Basic File Operations
```cursed
yeet "filez"
yeet "vibez"

fr fr Write content to file
sus content tea = "Hello, CURSED file system!"
sus write_err tea = write_file("hello.txt", content)
ready (write_err == "") {
    vibez.spill("File written successfully")
} otherwise {
    vibez.spill("Write error:", write_err)
}

fr fr Read content from file  
(read_content, read_err) := read_file("hello.txt")
ready (read_err == "") {
    vibez.spill("File content:", read_content)
} otherwise {
    vibez.spill("Read error:", read_err)
}

fr fr Check if file exists
ready (file_exists("hello.txt")) {
    vibez.spill("File exists and is accessible")
} otherwise {
    vibez.spill("File does not exist")
}
```

### File Metadata and Information
```cursed
// Get file size
(size, size_err) := filez.file_size("hello.txt")
ready (size_err == "") {
    vibez.spill("File size:", size, "bytes")
}

// Get comprehensive file information
(info, info_err) := filez.file_info("hello.txt")
ready (info_err == "") {
    vibez.spill("File:", info.name)
    vibez.spill("Size:", info.size)
    vibez.spill("Modified:", info.modified_time)
    vibez.spill("Is file:", info.is_file)
    vibez.spill("Is directory:", info.is_directory)
}

// Get file modification time
(mod_time, time_err) := filez.file_modified_time("hello.txt")
ready (time_err == "") {
    vibez.spill("Modified at:", mod_time)
}
```

### File Operations
```cursed
// Copy file
sus copy_err tea = filez.copy_file("source.txt", "destination.txt")

// Move/rename file
sus move_err tea = filez.move_file("old_name.txt", "new_name.txt")

// Append to file
sus append_err tea = filez.append_file("log.txt", "New log entry\n")

// Delete file
sus delete_err tea = filez.delete_file("temporary.txt")

// Sync file to disk
sus sync_err tea = filez.sync_file("important.txt")
```

### Line-Based File Operations
```cursed
// Read file as lines
(lines, lines_err) := filez.read_file_lines("config.txt")
ready (lines_err == "") {
    sus i normie = 0
    bestie (i < len(lines)) {
        vibez.spill("Line", i + 1, ":", lines[i])
        i = i + 1
    }
}

// Write lines to file
sus config_lines []tea = [
    "server=localhost",
    "port=8080", 
    "debug=true"
]
sus write_lines_err tea = filez.write_file_lines("config.txt", config_lines)
```

### Binary File Operations
```cursed
// Write binary data
sus binary_data []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F] // "Hello"
sus binary_err tea = filez.write_file_bytes("binary.dat", binary_data)

// Read binary data with size limit
(read_binary, read_err) := filez.read_file_bytes("binary.dat", 1024)
ready (read_err == "") {
    vibez.spill("Read", len(read_binary), "bytes")
}
```

### Directory Operations
```cursed
// Create directory
sus mkdir_err tea = filez.create_directory("new_folder")

// Check if directory exists
ready (filez.directory_exists("new_folder")) {
    vibez.spill("Directory created successfully")
}

// List directory contents
(entries, list_err) := filez.list_directory(".")
ready (list_err == "") {
    sus i normie = 0
    bestie (i < len(entries)) {
        vibez.spill("Entry:", entries[i])
        i = i + 1
    }
}

// Remove directory
sus rmdir_err tea = filez.remove_directory("new_folder")

// Copy directory recursively
sus copy_dir_err tea = filez.copy_directory("source_dir", "dest_dir")
```

### Working Directory Management
```cursed
// Get current working directory
(cwd, cwd_err) := filez.get_working_directory()
ready (cwd_err == "") {
    vibez.spill("Current directory:", cwd)
}

// Change working directory
sus chdir_err tea = filez.set_working_directory("/path/to/new/dir")
ready (chdir_err == "") {
    vibez.spill("Changed to new directory")
}
```

### Temporary Files and Directories
```cursed
// Get system temp directory
(temp_dir, temp_err) := filez.get_temp_directory()
ready (temp_err == "") {
    vibez.spill("Temp directory:", temp_dir)
}

// Create temporary file
(temp_file, temp_file_err) := filez.create_temp_file("myapp", ".tmp")
ready (temp_file_err == "") {
    vibez.spill("Created temp file:", temp_file)
    
    // Use temp file
    filez.write_file(temp_file, "Temporary data")
    
    // Clean up
    filez.delete_file(temp_file)
}
```

### File Type Checking
```cursed
// Check if path is a file
ready (filez.is_file("document.txt")) {
    vibez.spill("Path is a regular file")
}

// Check if path is a directory
ready (filez.is_directory("folder")) {
    vibez.spill("Path is a directory")
}
```

### Permission Management
```cursed
// Get file permissions
(perms, perms_err) := filez.file_permissions("secure.txt")
ready (perms_err == "") {
    vibez.spill("File permissions:", perms)
}

// Set file permissions
sus set_perms_err tea = filez.set_file_permissions("secure.txt", "644")
```

## Data Types

### FileInfo Structure
```cursed
be_like FileInfo = squad {
    spill name tea           // File name
    spill size normie        // File size in bytes
    spill modified_time normie // Modification time (Unix timestamp)
    spill is_file lit        // True if regular file
    spill is_directory lit   // True if directory
    spill is_symlink lit     // True if symbolic link
    spill permissions tea    // File permissions string
}
```

## Function Reference

### Core File Operations
- `read_file(filename)` - Read file content as string
- `write_file(filename, content)` - Write string content to file
- `append_file(filename, content)` - Append content to file
- `file_exists(filename)` - Check if file exists
- `file_size(filename)` - Get file size in bytes
- `delete_file(filename)` - Delete file
- `copy_file(source, dest)` - Copy file
- `move_file(source, dest)` - Move/rename file

### Line-Based Operations
- `read_file_lines(filename)` - Read file as array of lines
- `write_file_lines(filename, lines)` - Write array of lines to file

### Binary Operations
- `read_file_bytes(filename, max_bytes)` - Read file as byte array
- `write_file_bytes(filename, bytes)` - Write byte array to file

### File Metadata
- `file_info(filename)` - Get comprehensive file information
- `file_modified_time(filename)` - Get modification timestamp
- `file_permissions(filename)` - Get file permissions
- `set_file_permissions(filename, permissions)` - Set file permissions
- `sync_file(filename)` - Force file sync to disk

### File Type Checking
- `is_file(path)` - Check if path is a regular file
- `is_directory(path)` - Check if path is a directory

### Directory Operations
- `create_directory(dirname)` - Create directory
- `remove_directory(dirname)` - Remove empty directory
- `directory_exists(dirname)` - Check if directory exists
- `list_directory(dirname)` - List directory contents
- `copy_directory(source, dest)` - Copy directory recursively

### Working Directory
- `get_working_directory()` - Get current working directory
- `set_working_directory(dirname)` - Change working directory

### Temporary Files
- `get_temp_directory()` - Get system temp directory
- `create_temp_file(prefix, suffix)` - Create unique temporary file

### Utility Functions
- `is_valid_filename(filename)` - Validate filename
- `rename_file(old_name, new_name)` - Rename file

## Constants

- `FILE_READ_MODE` = 0 - File read mode
- `FILE_WRITE_MODE` = 1 - File write mode
- `FILE_APPEND_MODE` = 2 - File append mode
- `MAX_FILENAME_LENGTH` = 255 - Maximum filename length
- `BUFFER_SIZE` = 4096 - Default buffer size for operations

## Error Handling

All file operations return error strings. An empty string indicates success, while any non-empty string indicates an error occurred with a descriptive message.

### Common Error Patterns
```cursed
// Check for errors in file operations
(content, err) := filez.read_file("data.txt")
ready (err != "") {
    vibez.spill("Error reading file:", err)
    damn // Exit early
}

// Use content safely
vibez.spill("File content:", content)
```

### Error Types
- **File not found errors**: "File not found: filename.txt"
- **Permission errors**: "Permission denied: filename.txt"
- **Invalid input errors**: "Empty filename not allowed"
- **System errors**: "Failed to read file: filename.txt - system error"

## Implementation Notes

### Runtime Bridge Pattern
The filez module uses a runtime bridge pattern where critical functions are implemented in the Zig runtime for optimal performance, with pure CURSED fallbacks for compatibility.

**Runtime Bridge Functions:**
- `runtime_read_file()` - System file reading
- `runtime_write_file()` - System file writing
- `runtime_file_exists()` - File existence checking
- `runtime_file_size()` - File size retrieval
- `runtime_delete_file()` - File deletion
- `runtime_create_directory()` - Directory creation
- `runtime_list_directory()` - Directory listing
- And many more for comprehensive system integration

### Performance Optimization
- Efficient buffer management for large files
- Atomic operations for file safety
- Minimal system call overhead
- Proper resource cleanup and error handling

### Security Considerations
- Filename validation to prevent path traversal
- Permission checking before operations
- Safe handling of temporary files
- Proper error messages without information leakage

## Testing
Run the comprehensive test suite:
```bash
./zig-out/bin/cursed stdlib/filez/test_filez.csd
```

The test suite covers:
- Basic file read/write operations
- File metadata and permissions
- Directory operations and management
- Line-based and binary file operations
- Temporary file and directory handling
- Error handling scenarios
- File type checking and validation
- Working directory management

## Cross-Platform Compatibility

### Supported Platforms
- Linux (all distributions)
- macOS (Intel and Apple Silicon)
- Windows (native and WSL)
- FreeBSD and other Unix-like systems

### Path Handling
- Automatic path separator normalization
- Unicode filename support
- Long path handling on Windows
- Symbolic link resolution

### Permissions
- Unix-style permission strings (e.g., "644", "755")
- Windows ACL integration where available
- Proper permission inheritance
- Safe default permissions for created files

## Best Practices

### File Operations
- Always check error returns from file operations
- Use appropriate file modes for your use case
- Close or sync files after critical writes
- Validate filenames before operations

### Directory Management
- Check if directories exist before operations
- Use temporary directories for scratch work
- Clean up temporary files and directories
- Handle permission errors gracefully

### Error Handling
- Provide meaningful error messages to users
- Log detailed errors for debugging
- Implement retry logic for transient errors
- Use proper error propagation patterns

### Performance
- Use binary operations for large files
- Batch directory operations when possible
- Consider file size limits for memory usage
- Use appropriate buffer sizes for I/O operations

### Security
- Validate all input filenames and paths
- Use temporary files for sensitive operations
- Set appropriate file permissions
- Avoid exposing internal paths in error messages
