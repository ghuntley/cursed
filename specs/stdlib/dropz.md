# dropz (Core I/O)

## Overview
`dropz` provides the fundamental I/O operations for CURSED, serving as the core I/O package similar to Go's `io` and `os` packages combined. This module is critical for self-hosting as it provides file operations, standard I/O, and Reader/Writer interfaces that the compiler needs.

## Core Interfaces

### `Reader`
Interface for reading data from a source.

```cursed
collab Reader {
    read(p []byte) (n normie, err tea)
}
```

### `Writer` 
Interface for writing data to a destination.

```cursed
collab Writer {
    write(p []byte) (n normie, err tea)
}
```

### `ReadWriter`
Combined Reader and Writer interface.

```cursed
collab ReadWriter {
    read(p []byte) (n normie, err tea)
    write(p []byte) (n normie, err tea)
}
```

### `Closer`
Interface for closeable resources.

```cursed
collab Closer {
    close() tea
}
```

### `Seeker`
Interface for seeking within a data stream.

```cursed
collab Seeker {
    seek(offset thicc, whence normie) (thicc, tea)
}
```

## File Operations

### Core File Functions

```cursed
# Read entire file as bytes
slay read_file(filename tea) ([]byte, tea)

# Read entire file as text 
slay read_text_file(filename tea) (tea, tea)

# Write bytes to file
slay write_file(filename tea, data []byte, perm normie) tea

# Write text to file
slay write_text_file(filename tea, content tea, perm normie) tea

# Append bytes to file
slay append_file(filename tea, data []byte, perm normie) tea

# Copy file
slay copy_file(src tea, dst tea) (thicc, tea)
```

### File Handle Operations

```cursed
struct File {
    fd normie,
    name tea,
    flag normie
}

# Open file for reading
slay open(filename tea) (*File, tea)

# Create file for writing  
slay create(filename tea) (*File, tea)

# Open file with specific flags and permissions
slay open_file(filename tea, flag normie, perm normie) (*File, tea)

# File methods
slay (f *File) read(b []byte) (normie, tea)
slay (f *File) write(b []byte) (normie, tea)
slay (f *File) close() tea
slay (f *File) seek(offset thicc, whence normie) (thicc, tea)
slay (f *File) stat() (FileInfo, tea)
slay (f *File) truncate(size thicc) tea
slay (f *File) sync() tea
```

## Directory Operations

```cursed
struct DirEntry {
    name tea,
    is_dir lit,
    is_file lit,
    size thicc,
    mode normie,
    mod_time thicc
}

# Create directory
slay mkdir(dirname tea, perm normie) tea

# Create directory and all parent directories
slay mkdir_all(dirname tea, perm normie) tea

# Remove directory (must be empty)
slay rmdir(dirname tea) tea  

# Remove directory and all contents
slay remove_all(dirname tea) tea

# List directory contents
slay read_dir(dirname tea) ([]DirEntry, tea)

# Walk directory tree
slay walk_dir(root tea, fn WalkDirFunc) tea

# Working directory operations
slay getwd() (tea, tea)
slay chdir(dir tea) tea
```

### File Info Operations

```cursed
struct FileInfo {
    name tea,
    size thicc,
    mode normie,
    mod_time thicc,
    is_dir lit
}

# Get file/directory info
slay stat(path tea) (FileInfo, tea)
slay lstat(path tea) (FileInfo, tea)

# Check if path exists
slay exists(path tea) lit

# Check if path is directory
slay is_dir(path tea) lit

# Check if path is regular file
slay is_file(path tea) lit
```

## Standard I/O

```cursed
# Standard streams
sus stdin Reader = ...
sus stdout Writer = ...
sus stderr Writer = ...

# Print functions
slay print(a ...collab{}) (normie, tea)
slay println(a ...collab{}) (normie, tea)
slay printf(format tea, a ...collab{}) (normie, tea)

# Input functions
slay read_line() (tea, tea)
slay read_all(r Reader) ([]byte, tea)
```

## Utility Functions

```cursed
# Copy from Reader to Writer
slay copy(dst Writer, src Reader) (thicc, tea)

# Copy with buffer size limit
slay copy_buffer(dst Writer, src Reader, buf []byte) (thicc, tea)

# Copy exactly n bytes
slay copy_n(dst Writer, src Reader, n thicc) (thicc, tea)

# Read exactly n bytes
slay read_full(r Reader, buf []byte) (normie, tea)

# Read at least min bytes
slay read_at_least(r Reader, buf []byte, min normie) (normie, tea)

# Write all data
slay write_string(w Writer, s tea) (normie, tea)
```

## Buffered I/O

```cursed
struct BufReader {
    rd Reader,
    buf []byte,
    r normie,
    w normie
}

struct BufWriter {
    wr Writer,
    buf []byte,
    n normie
}

# Create buffered reader
slay new_reader(rd Reader) *BufReader
slay new_reader_size(rd Reader, size normie) *BufReader

# Create buffered writer  
slay new_writer(wr Writer) *BufWriter
slay new_writer_size(wr Writer, size normie) *BufWriter

# BufReader methods
slay (b *BufReader) read(p []byte) (normie, tea)
slay (b *BufReader) read_byte() (byte, tea)
slay (b *BufReader) read_line() ([]byte, lit, tea)
slay (b *BufReader) read_string(delim byte) (tea, tea)

# BufWriter methods
slay (b *BufWriter) write(p []byte) (normie, tea)
slay (b *BufWriter) write_byte(c byte) tea
slay (b *BufWriter) write_string(s tea) (normie, tea)
slay (b *BufWriter) flush() tea
```

## Path Operations

```cursed
# Path manipulation
slay join(elem ...tea) tea
slay clean(path tea) tea
slay dir(path tea) tea
slay base(path tea) tea
slay ext(path tea) tea
slay abs(path tea) (tea, tea)
slay rel(basepath tea, targpath tea) (tea, tea)

# Path testing
slay is_abs(path tea) lit
slay has_prefix(p tea, prefix tea) lit
slay has_suffix(p tea, suffix tea) lit
```

## Constants

```cursed
# File open flags
fact O_RDONLY normie = 0
fact O_WRONLY normie = 1  
fact O_RDWR normie = 2
fact O_APPEND normie = 1024
fact O_CREATE normie = 64
fact O_EXCL normie = 128
fact O_SYNC normie = 1052672
fact O_TRUNC normie = 512

# File permissions
fact MODE_REGULAR normie = 0644
fact MODE_EXECUTABLE normie = 0755
fact MODE_DIR normie = 0755

# Seek whence values
fact SEEK_START normie = 0
fact SEEK_CURRENT normie = 1
fact SEEK_END normie = 2

# EOF error
fact EOF tea = "EOF"
```

## Error Types

```cursed
struct PathError {
    op tea,
    path tea,
    err tea
}

slay (e *PathError) error() tea {
    damn e.op + " " + e.path + ": " + e.err
}

# Common errors
fact ErrInvalid tea = "invalid argument"
fact ErrPermission tea = "permission denied"
fact ErrExist tea = "file already exists"
fact ErrNotExist tea = "file does not exist"
fact ErrClosed tea = "file already closed"
```

## Self-Hosting Support

### Compiler-Specific Functions

```cursed
# Read source file for compilation
slay read_source_file(filename tea) (tea, tea)

# Write compiled output
slay write_compiled_output(filename tea, content tea) tea

# Create temporary file for intermediate compilation
slay temp_file(pattern tea) (*File, tea)

# Write object file
slay write_object_file(filename tea, data []byte) tea

# Read configuration file
slay read_config_file(filename tea) (tea, tea)
```

## Implementation Notes

### FFI-Free Design
- All operations implemented in pure CURSED without external FFI dependencies
- Uses CURSED runtime system for actual file system operations
- Maintains compatibility with both interpretation and compilation modes

### Performance Considerations
- Buffered I/O reduces system call overhead
- Efficient memory management for large file operations
- Optimized path operations using string manipulation

### Thread Safety
- File operations are thread-safe at the OS level
- Buffered operations require external synchronization
- Standard streams are safe for concurrent access

### Error Handling
- Consistent error return patterns following CURSED conventions
- Detailed error information including operation and path context
- Graceful handling of common file system errors

## Testing Strategy

```cursed
# Test file operations
slay test_file_operations()
slay test_directory_operations()
slay test_buffered_io()
slay test_path_operations()
slay test_error_handling()

# Integration tests for self-hosting
slay test_compiler_integration()
slay test_source_file_reading()
slay test_output_generation()
```

## Dependencies

- `core`: Basic types and error handling
- `vibez`: Output operations (for debugging/logging)
- CURSED runtime: Low-level file system operations

## Security Considerations

- Path traversal protection in file operations
- Permission checking for file access
- Secure temporary file creation
- Input validation for all path operations
