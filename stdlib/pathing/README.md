# Pathing Module

Cross-platform path manipulation utilities for CURSED programs. This module provides comprehensive path operations without external dependencies, using pure CURSED implementations.

## Features

- **Cross-platform compatibility**: Works on Unix-like systems and Windows
- **Pure CURSED implementation**: No FFI dependencies
- **Comprehensive path operations**: Join, split, clean, and manipulate paths
- **Glob pattern matching**: Simple wildcard pattern support
- **Path normalization**: Clean and resolve relative paths
- **Extension handling**: Extract and manipulate file extensions

## Functions

### Core Path Operations

#### `path_join(parts [tea]) tea`
Join multiple path components with the appropriate separator.

```cursed
sus parts [tea] = ["usr", "local", "bin"]
sus path tea = path_join(parts)  // "usr/local/bin"
```

#### `path_split(path tea) [tea]`
Split a path into its components.

```cursed
sus parts [tea] = path_split("usr/local/bin")
// Returns: ["usr", "local", "bin"]
```

#### `path_basename(path tea) tea`
Get the last component of a path (filename).

```cursed
sus name tea = path_basename("usr/local/bin/file.txt")  // "file.txt"
```

#### `path_dirname(path tea) tea`
Get the directory portion of a path.

```cursed
sus dir tea = path_dirname("usr/local/bin/file.txt")  // "usr/local/bin"
```

#### `path_ext(path tea) tea`
Get the file extension including the dot.

```cursed
sus ext tea = path_ext("file.txt")  // ".txt"
sus ext2 tea = path_ext("archive.tar.gz")  // ".gz"
```

### Path Normalization

#### `path_clean(path tea) tea`
Clean and normalize a path by resolving `.` and `..` components.

```cursed
sus clean tea = path_clean("usr/./local/../bin")  // "usr/bin"
```

#### `path_abs(path tea) tea`
Convert a relative path to an absolute path.

```cursed
sus abs tea = path_abs("file.txt")  // "/home/user/file.txt"
```

#### `path_rel(base tea, target tea) tea`
Get the relative path from base to target.

```cursed
sus rel tea = path_rel("/usr/local", "/usr/local/bin")  // "bin"
```

### Path Analysis

#### `path_is_abs(path tea) lit`
Check if a path is absolute.

```cursed
assert_true(path_is_abs("/usr/local/bin"))
assert_false(path_is_abs("usr/local/bin"))
```

#### `path_match(pattern tea, path tea) lit`
Match a path against a glob pattern (supports * wildcard).

```cursed
assert_true(path_match("*.txt", "file.txt"))
assert_true(path_match("test_*.💀", "test_file.💀"))
```

### Path Conversion

#### `path_from_slash(path tea) tea`
Convert from slash notation to platform-specific separators.

```cursed
sus native tea = path_from_slash("usr/local/bin")
// On Windows: "usr\\local\\bin"
// On Unix: "usr/local/bin"
```

#### `path_to_slash(path tea) tea`
Convert to slash notation from platform-specific separators.

```cursed
sus portable tea = path_to_slash("usr\\local\\bin")  // "usr/local/bin"
```

## Usage Examples

### Basic Path Manipulation

```cursed
yeet "pathing"

// Join path components
sus config_path tea = path_join(["home", "user", ".config", "app.conf"])
vibez.spill("Config path: " + config_path)

// Extract filename and directory
sus full_path tea = "/home/user/documents/report.pdf"
sus filename tea = path_basename(full_path)  // "report.pdf"
sus directory tea = path_dirname(full_path)  // "/home/user/documents"
sus extension tea = path_ext(full_path)      // ".pdf"

vibez.spill("File: " + filename)
vibez.spill("Dir: " + directory)
vibez.spill("Ext: " + extension)
```

### Path Cleaning and Normalization

```cursed
yeet "pathing"

// Clean up messy paths
sus messy_path tea = "usr/./local/../bin/./program"
sus clean_path tea = path_clean(messy_path)  // "usr/bin/program"

// Convert relative to absolute
sus relative tea = "../config/settings.json"
sus absolute tea = path_abs(relative)

// Get relative path between directories
sus base tea = "/home/user/projects"
sus target tea = "/home/user/documents/file.txt"
sus relative_path tea = path_rel(base, target)  // "../documents/file.txt"
```

### Pattern Matching

```cursed
yeet "pathing"

// Check if files match patterns
sus files [tea] = ["test_math.💀", "test_string.💀", "main.💀", "README.md"]

bestie i := 0; i < files.length; i++ {
    skit path_match("test_*.💀", files[i]) {
        vibez.spill("Test file: " + files[i])
    }
    
    skit path_match("*.md", files[i]) {
        vibez.spill("Markdown file: " + files[i])
    }
}
```

### Cross-Platform Compatibility

```cursed
yeet "pathing"

// Handle different path separators
sus unix_path tea = "usr/local/bin"
sus windows_path tea = "usr\\local\\bin"

// Convert to standard format
sus standard1 tea = path_to_slash(unix_path)     // "usr/local/bin"
sus standard2 tea = path_to_slash(windows_path)  // "usr/local/bin"

// Convert back to platform-specific
sus native1 tea = path_from_slash(standard1)
sus native2 tea = path_from_slash(standard2)
```

## Testing

The module includes comprehensive tests covering all functions:

```bash
# Run pathing module tests
cargo run --bin cursed stdlib/pathing/test_pathing.💀

# Test native compilation
cargo run --bin cursed -- compile stdlib/pathing/test_pathing.💀
./test_pathing
```

## Implementation Notes

- **Pure CURSED**: All functions implemented in CURSED without external dependencies
- **Cross-platform**: Handles both Unix-like and Windows path conventions
- **String utilities**: Includes helper functions for string manipulation
- **Glob support**: Basic wildcard pattern matching with `*` support
- **Path normalization**: Resolves `.` and `..` components properly
- **Memory efficient**: Minimal memory allocation for path operations

## Constants

- `PATH_SEPARATOR`: Unix-like path separator (`"/"`)
- `PATH_SEPARATOR_WIN`: Windows path separator (`"\\"`)
- `PATH_LIST_SEPARATOR`: Unix-like path list separator (`":"`)
- `PATH_LIST_SEPARATOR_WIN`: Windows path list separator (`";"`)

## Error Handling

All functions handle edge cases gracefully:
- Empty paths return appropriate defaults
- Invalid paths are cleaned when possible
- Relative path operations handle corner cases
- Pattern matching fails safely for invalid patterns

## Performance

The implementation is optimized for:
- Minimal string copying
- Efficient path component parsing
- Fast pattern matching for common cases
- Memory-conscious operations

This module provides a solid foundation for path manipulation in CURSED programs, supporting both simple and complex path operations across different platforms.
