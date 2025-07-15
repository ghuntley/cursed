# CURSED Filesystem Module

The `fs` module provides comprehensive file system operations for CURSED programs. This is a pure CURSED implementation that works in both interpretation and compilation modes.

## Features

- **File Operations**: Read, write, delete files
- **Directory Operations**: Create, remove, list directories
- **Path Utilities**: Basename, extension, path joining
- **File Information**: Size, timestamps, permissions
- **Metadata Access**: Comprehensive file metadata structures
- **Cross-Platform**: Consistent behavior across platforms

## Installation

```cursed
yeet "fs"
```

## Core Functions

### File Operations

#### `read_file(path tea) tea`
Read the contents of a file as a string.

```cursed
sus content tea = read_file("example.txt")
```

#### `write_file(path tea, content tea) lit`
Write content to a file.

```cursed
sus success lit = write_file("output.txt", "Hello, World!")
```

#### `file_exists(path tea) lit`
Check if a file exists.

```cursed
lowkey file_exists("config.txt") {
    vibez.spill("Config file found!")
}
```

#### `delete_file(path tea) lit`
Delete a file.

```cursed
sus deleted lit = delete_file("temp.txt")
```

#### `get_file_size(path tea) thicc`
Get the size of a file in bytes.

```cursed
sus size thicc = get_file_size("document.pdf")
```

### Directory Operations

#### `create_dir(path tea) lit`
Create a directory.

```cursed
sus created lit = create_dir("new_folder")
```

#### `remove_dir(path tea) lit`
Remove a directory.

```cursed
sus removed lit = remove_dir("old_folder")
```

#### `is_dir(path tea) lit`
Check if a path is a directory.

```cursed
lowkey is_dir("documents") {
    vibez.spill("It's a directory!")
}
```

#### `is_file(path tea) lit`
Check if a path is a regular file.

```cursed
lowkey is_file("document.txt") {
    vibez.spill("It's a file!")
}
```

#### `list_dir(path tea) []tea`
List the contents of a directory.

```cursed
sus files []tea = list_dir(".")
```

### Path Utilities

#### `join_path(base tea, component tea) tea`
Join path components with the proper separator.

```cursed
sus full_path tea = join_path("home/user", "documents")
# Result: "home/user/documents"
```

#### `get_extension(path tea) tea`
Get the file extension including the dot.

```cursed
sus ext tea = get_extension("document.pdf")
# Result: ".pdf"
```

#### `get_basename(path tea) tea`
Get the filename without the directory path.

```cursed
sus filename tea = get_basename("path/to/file.txt")
# Result: "file.txt"
```

### File Timestamps

#### `get_modified_time(path tea) thicc`
Get the file modification time as a Unix timestamp.

```cursed
sus mod_time thicc = get_modified_time("document.txt")
```

#### `get_created_time(path tea) thicc`
Get the file creation time as a Unix timestamp.

```cursed
sus created_time thicc = get_created_time("document.txt")
```

#### `get_accessed_time(path tea) thicc`
Get the file access time as a Unix timestamp.

```cursed
sus access_time thicc = get_accessed_time("document.txt")
```

#### `set_modified_time(path tea, timestamp thicc) lit`
Set the file modification time.

```cursed
sus success lit = set_modified_time("document.txt", 1704067200)
```

### File Permissions

#### `get_permissions(path tea) normie`
Get file permissions as Unix-style octal notation.

```cursed
sus perms normie = get_permissions("script.sh")
# Result: 755 (for executable) or 644 (for regular file)
```

#### `set_permissions(path tea, perms normie) lit`
Set file permissions using Unix-style octal notation.

```cursed
sus success lit = set_permissions("script.sh", 755)
```

#### `is_readable(path tea) lit`
Check if a file is readable.

```cursed
lowkey is_readable("config.txt") {
    vibez.spill("Can read the config file")
}
```

#### `is_writable(path tea) lit`
Check if a file is writable.

```cursed
lowkey is_writable("log.txt") {
    vibez.spill("Can write to log file")
}
```

#### `is_executable(path tea) lit`
Check if a file is executable.

```cursed
lowkey is_executable("script.sh") {
    vibez.spill("Can execute the script")
}
```

## Data Structures

### FileInfo
Basic file information structure.

```cursed
be_like FileInfo squad {
    name tea           # Filename
    size thicc         # File size in bytes
    is_dir lit         # Is directory flag
    modified_time thicc # Last modification time
    permissions normie  # File permissions
}
```

### FileMetadata
Comprehensive file metadata structure.

```cursed
be_like FileMetadata squad {
    name tea           # Filename
    path tea           # Full path
    size thicc         # File size in bytes
    is_dir lit         # Is directory flag
    is_file lit        # Is regular file flag
    is_symlink lit     # Is symbolic link flag
    created_time thicc # Creation time
    modified_time thicc # Last modification time
    accessed_time thicc # Last access time
    permissions normie  # File permissions
    owner_id normie    # Owner user ID
    group_id normie    # Group ID
}
```

### Usage Examples

#### Basic File Operations

```cursed
yeet "fs"

# Write a file
sus content tea = "Hello, CURSED filesystem!"
assert_true(write_file("hello.txt", content))

# Read the file back
sus read_content tea = read_file("hello.txt")
assert_eq_string(read_content, content)

# Check file information
sus info FileInfo = get_file_info("hello.txt")
vibez.spill("File size: " + info.size.to_string())

# Clean up
assert_true(delete_file("hello.txt"))
```

#### Directory Operations

```cursed
yeet "fs"

# Create a directory
assert_true(create_dir("test_dir"))

# Check if it's a directory
assert_true(is_dir("test_dir"))
assert_false(is_file("test_dir"))

# Get directory metadata
sus dir_metadata FileMetadata = get_file_metadata("test_dir")
assert_true(dir_metadata.is_dir)
assert_false(dir_metadata.is_file)

# Remove directory
assert_true(remove_dir("test_dir"))
```

#### Path Manipulation

```cursed
yeet "fs"

# Join paths
sus full_path tea = join_path("home", "user/documents")
vibez.spill("Full path: " + full_path)

# Extract filename
sus filename tea = get_basename("path/to/document.pdf")
vibez.spill("Filename: " + filename)

# Get extension
sus extension tea = get_extension("document.pdf")
vibez.spill("Extension: " + extension)
```

## Testing

The module includes comprehensive tests that verify:

- Basic file operations (read, write, delete)
- Directory operations (create, remove, list)
- Path utilities (basename, extension, joining)
- File timestamps and permissions
- Metadata access and manipulation
- Error handling for non-existent files
- Cross-platform compatibility

Run the tests with:

```bash
# Interpretation mode
cargo run --bin cursed stdlib/fs/test_fs.csd

# Compilation mode  
cargo run --bin cursed -- compile stdlib/fs/test_fs.csd
./test_fs
```

## Implementation Details

This module provides a mock implementation suitable for testing and development. In a production environment, these functions would interface with the actual operating system file system APIs.

The module follows CURSED language conventions:
- Uses `tea` for string types
- Uses `lit` for boolean types
- Uses `normie` for 32-bit integers
- Uses `thicc` for 64-bit integers
- Uses `lowkey` for conditional statements
- Uses `damn` for return statements

## License

This module is part of the CURSED standard library and follows the same license as the CURSED language.
