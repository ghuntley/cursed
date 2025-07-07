# CURSED Filesystem Module

Production-ready filesystem operations for CURSED programs using the runtime bridge.

## Features

- **File Operations**: Read, write, delete files
- **Directory Operations**: Create, list, remove directories
- **Path Utilities**: Join paths, get extensions, basenames
- **File Information**: Size, permissions, type checking
- **Error Handling**: Graceful error handling for all operations
- **Cross-Platform**: Works on all platforms supported by CURSED

## Usage

### Import the Module

```cursed
yeet "fs"
```

### File Operations

```cursed
// Write file
sus success lit = fs.write_file("hello.txt", "Hello, World!")

// Read file
sus content tea = fs.read_file("hello.txt")

// Check if file exists
sus exists lit = fs.file_exists("hello.txt")

// Get file size
sus size thicc = fs.get_file_size("hello.txt")

// Delete file
sus deleted lit = fs.delete_file("hello.txt")
```

### Directory Operations

```cursed
// Create directory
sus created lit = fs.create_dir("mydir")

// List directory contents
sus files []tea = fs.list_dir("mydir")

// Check if path is directory
sus is_dir lit = fs.is_dir("mydir")

// Create nested directories
sus created_recursive lit = fs.create_dir_recursive("level1/level2/level3")
```

### Path Utilities

```cursed
// Join paths
sus full_path tea = fs.join_path("/home/user", "documents")
// Result: "/home/user/documents"

// Get file extension
sus ext tea = fs.get_extension("document.txt")
// Result: ".txt"

// Get filename
sus filename tea = fs.get_basename("/home/user/document.txt")
// Result: "document.txt"
```

### File Information

```cursed
// Get detailed file info
sus info fs.FileInfo = fs.get_file_info("myfile.txt")
vibez.spill("Name: " + info.name)
vibez.spill("Size: " + tea(info.size))
vibez.spill("Is Directory: " + tea(info.is_dir))

// Get/set permissions
sus perms normie = fs.get_permissions("myfile.txt")
sus set_success lit = fs.set_permissions("myfile.txt", 755)
```

## API Reference

### File Operations

- `read_file(path: tea) -> tea` - Read file contents as string
- `write_file(path: tea, content: tea) -> lit` - Write string to file
- `file_exists(path: tea) -> lit` - Check if file exists
- `delete_file(path: tea) -> lit` - Delete file
- `get_file_size(path: tea) -> thicc` - Get file size in bytes

### Directory Operations

- `create_dir(path: tea) -> lit` - Create directory
- `list_dir(path: tea) -> []tea` - List directory contents
- `remove_dir(path: tea) -> lit` - Remove directory
- `create_dir_recursive(path: tea) -> lit` - Create directory tree
- `is_dir(path: tea) -> lit` - Check if path is directory
- `is_file(path: tea) -> lit` - Check if path is file

### Path Utilities

- `join_path(base: tea, component: tea) -> tea` - Join path components
- `get_extension(path: tea) -> tea` - Get file extension
- `get_basename(path: tea) -> tea` - Get filename from path

### File Information

- `get_file_info(path: tea) -> FileInfo` - Get detailed file information
- `get_permissions(path: tea) -> normie` - Get file permissions
- `set_permissions(path: tea, perms: normie) -> lit` - Set file permissions

### Types

```cursed
be_like FileInfo squad {
    name tea           // Filename
    size thicc         // File size in bytes
    is_dir lit         // Is directory flag
    modified_time thicc // Last modified timestamp
    permissions normie  // File permissions
}
```

## Error Handling

All functions handle errors gracefully:

- File operations return empty strings or false on error
- Directory operations return false on error
- File size returns -1 on error
- List operations return empty arrays on error

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/fs/test_fs.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/fs/test_fs.csd
./test_fs
```

## Implementation Notes

- Uses CURSED runtime bridge for system calls
- Cross-platform compatible
- Memory-safe string handling
- Proper error propagation
- Production-ready for self-hosting

## Self-Hosting Ready

This module is designed for self-hosting and provides all essential filesystem operations needed for:

- Reading/writing source files
- Managing build artifacts
- Directory traversal
- File system queries
- Build system integration
