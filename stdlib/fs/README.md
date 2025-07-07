# CURSED Filesystem Module

The `fs` module provides comprehensive file system operations for CURSED programs, including file I/O, directory management, and path utilities.

## Features

- **File Operations**: Read, write, and delete files
- **Directory Management**: Create, list, and remove directories
- **Path Utilities**: Join paths, extract extensions and basenames
- **File Information**: Get file size, permissions, and metadata
- **Cross-Platform**: Works on all supported CURSED platforms

## Usage

```cursed
yeet "fs"

fr fr Read file contents
sus content tea = fs.read_file("config.txt")
vibez.spill("File contents: " + content)

fr fr Write to file
sus success lit = fs.write_file("output.txt", "Hello, World!")
lowkey success {
    vibez.spill("File written successfully")
}

fr fr Check if file exists
lowkey fs.file_exists("important.txt") {
    vibez.spill("Important file found")
}

fr fr Create directory
fs.create_dir("new_folder")

fr fr List directory contents
sus files []tea = fs.list_dir(".")
bestie file yeet files {
    vibez.spill("Found file: " + file)
}
```

## API Reference

### File Operations

#### `read_file(path tea) tea`
Reads the entire contents of a file as a string.

**Parameters:**
- `path tea` - Path to the file to read

**Returns:**
- `tea` - File contents as string

**Example:**
```cursed
sus content tea = fs.read_file("data.txt")
```

#### `write_file(path tea, content tea) lit`
Writes a string to a file, creating it if it doesn't exist.

**Parameters:**
- `path tea` - Path to the file to write
- `content tea` - Content to write to the file

**Returns:**
- `lit` - `based` if successful, `cap` if failed

**Example:**
```cursed
sus success lit = fs.write_file("output.txt", "Hello, World!")
```

#### `file_exists(path tea) lit`
Checks if a file exists at the given path.

**Parameters:**
- `path tea` - Path to check

**Returns:**
- `lit` - `based` if file exists, `cap` otherwise

**Example:**
```cursed
lowkey fs.file_exists("config.json") {
    vibez.spill("Config file found")
}
```

#### `delete_file(path tea) lit`
Deletes a file at the given path.

**Parameters:**
- `path tea` - Path to the file to delete

**Returns:**
- `lit` - `based` if successful, `cap` if failed

**Example:**
```cursed
fs.delete_file("temp.txt")
```

#### `get_file_size(path tea) normie`
Gets the size of a file in bytes.

**Parameters:**
- `path tea` - Path to the file

**Returns:**
- `normie` - File size in bytes

**Example:**
```cursed
sus size normie = fs.get_file_size("data.bin")
vibez.spill("File size: " + tea(size) + " bytes")
```

### Directory Operations

#### `create_dir(path tea) lit`
Creates a directory at the given path.

**Parameters:**
- `path tea` - Path to the directory to create

**Returns:**
- `lit` - `based` if successful, `cap` if failed

**Example:**
```cursed
fs.create_dir("new_folder")
```

#### `create_dir_recursive(path tea) lit`
Creates a directory and all necessary parent directories.

**Parameters:**
- `path tea` - Path to the directory to create

**Returns:**
- `lit` - `based` if successful, `cap` if failed

**Example:**
```cursed
fs.create_dir_recursive("deep/nested/directory")
```

#### `list_dir(path tea) []tea`
Lists all files and directories in the given directory.

**Parameters:**
- `path tea` - Path to the directory to list

**Returns:**
- `[]tea` - Array of file and directory names

**Example:**
```cursed
sus files []tea = fs.list_dir(".")
bestie file yeet files {
    vibez.spill("Found: " + file)
}
```

#### `remove_dir(path tea) lit`
Removes an empty directory.

**Parameters:**
- `path tea` - Path to the directory to remove

**Returns:**
- `lit` - `based` if successful, `cap` if failed

**Example:**
```cursed
fs.remove_dir("empty_folder")
```

#### `is_dir(path tea) lit`
Checks if the given path is a directory.

**Parameters:**
- `path tea` - Path to check

**Returns:**
- `lit` - `based` if path is a directory, `cap` otherwise

**Example:**
```cursed
lowkey fs.is_dir("src") {
    vibez.spill("src is a directory")
}
```

#### `is_file(path tea) lit`
Checks if the given path is a file.

**Parameters:**
- `path tea` - Path to check

**Returns:**
- `lit` - `based` if path is a file, `cap` otherwise

**Example:**
```cursed
lowkey fs.is_file("README.md") {
    vibez.spill("README.md is a file")
}
```

### Path Utilities

#### `join_path(base tea, component tea) tea`
Joins two path components with the appropriate separator.

**Parameters:**
- `base tea` - Base path
- `component tea` - Path component to join

**Returns:**
- `tea` - Joined path

**Example:**
```cursed
sus full_path tea = fs.join_path("/home/user", "documents")
fr fr Result: "/home/user/documents"
```

#### `get_extension(path tea) tea`
Extracts the file extension from a path.

**Parameters:**
- `path tea` - File path

**Returns:**
- `tea` - File extension including the dot (e.g., ".txt")

**Example:**
```cursed
sus ext tea = fs.get_extension("file.txt")
fr fr Result: ".txt"
```

#### `get_basename(path tea) tea`
Extracts the filename from a path.

**Parameters:**
- `path tea` - File path

**Returns:**
- `tea` - Filename without directory path

**Example:**
```cursed
sus name tea = fs.get_basename("/path/to/file.txt")
fr fr Result: "file.txt"
```

### File Information

#### `get_file_info(path tea) FileInfo`
Gets detailed information about a file or directory.

**Parameters:**
- `path tea` - Path to the file or directory

**Returns:**
- `FileInfo` - Structure containing file information

**Example:**
```cursed
sus info FileInfo = fs.get_file_info("data.txt")
vibez.spill("File: " + info.name)
vibez.spill("Size: " + tea(info.size) + " bytes")
```

#### `FileInfo` Structure
```cursed
be_like FileInfo squad {
    name tea          fr fr Filename
    size normie       fr fr File size in bytes
    is_dir lit        fr fr true if directory
    modified_time normie  fr fr Last modified timestamp
    permissions normie    fr fr File permissions (octal)
}
```

### File Permissions

#### `set_permissions(path tea, perms normie) lit`
Sets file permissions using octal notation.

**Parameters:**
- `path tea` - Path to the file
- `perms normie` - Permissions in octal (e.g., 644, 755)

**Returns:**
- `lit` - `based` if successful, `cap` if failed

**Example:**
```cursed
fs.set_permissions("script.sh", 755)  fr fr Make executable
```

#### `get_permissions(path tea) normie`
Gets file permissions in octal notation.

**Parameters:**
- `path tea` - Path to the file

**Returns:**
- `normie` - Permissions in octal notation

**Example:**
```cursed
sus perms normie = fs.get_permissions("config.txt")
```

## Testing

Run the filesystem module tests:

```bash
cargo run --bin cursed stdlib/fs/test_fs.csd
```

## Implementation Status

Currently, all functions are implemented as stubs that return mock values and log their operations. The actual file system operations will be implemented through FFI (Foreign Function Interface) calls to the underlying system.

## Error Handling

Future implementations will include proper error handling for:
- File not found errors
- Permission denied errors
- Disk space errors
- Invalid path errors
- Network file system errors

## Platform Support

The filesystem module is designed to work across all platforms supported by CURSED:
- Linux
- macOS
- Windows
- FreeBSD

## Security Considerations

- Path traversal protection
- Permission validation
- Sandboxing support
- Safe file operations
- Input validation

## Future Enhancements

- File watching and monitoring
- Atomic file operations
- File locking mechanisms
- Symbolic link support
- Extended file attributes
- Compression support
- Backup and versioning utilities
