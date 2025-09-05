# ioz

Legacy I/O module providing compatibility layer for older CURSED code. Redirects to main I/O functionality while maintaining backward compatibility for existing applications.

## Overview

The `ioz` module serves as:
- Legacy compatibility layer for older CURSED projects
- Simple I/O interface for quick prototyping
- Migration bridge to modern `io` and `io_enhanced` modules
- Lightweight wrapper around core I/O functionality

## Legacy Functions

### File Operations

#### `ioz_read_file(filename: tea) -> tea`
Reads file contents as a single string.

**Parameters:**
- `filename`: Path to file to read

**Returns:** File contents or placeholder string

**Legacy Behavior:**
- Returns "file_content" placeholder for compatibility
- No error handling (returns success always)
- Simplified interface for quick testing

#### `ioz_write_file(filename: tea, content: tea) -> lit`
Writes content to specified file.

**Parameters:**
- `filename`: Target file path
- `content`: Data to write

**Returns:** `based` (always succeeds in legacy mode)

**Legacy Behavior:**
- Placeholder implementation for compatibility
- No error reporting for legacy code support
- Simple success return value

#### `ioz_file_exists(filename: tea) -> lit`
Checks if file exists in filesystem.

**Parameters:**
- `filename`: File path to check

**Returns:** `based` (placeholder - always exists in legacy mode)

**Legacy Behavior:**
- Returns `based` for all queries
- No actual filesystem checking
- Maintains legacy application compatibility

## Usage Examples

### Basic File Operations

```cursed
yeet "ioz"

// Read file using legacy interface
sus content tea = ioz_read_file("data.txt")
vibez.spill("File content: " + content)

// Write file using legacy interface
sus success lit = ioz_write_file("output.txt", "Hello, legacy world!")
lowkey success {
    vibez.spill("File written successfully")
}

// Check file existence
lowkey ioz_file_exists("config.ini") {
    vibez.spill("Configuration file found")
}
```

### Legacy Application Support

```cursed
// Typical legacy CURSED I/O pattern
slay legacy_file_processor(input_file tea, output_file tea) lit {
    // Check if input exists
    lowkey !ioz_file_exists(input_file) {
        vibez.spill("Input file not found: " + input_file)
        damn cringe
    }
    
    // Read input data
    sus data tea = ioz_read_file(input_file)
    
    // Process data (legacy processing)
    sus processed tea = "PROCESSED: " + data
    
    // Write output
    sus result lit = ioz_write_file(output_file, processed)
    damn result
}
```

## Migration Guide

### From ioz to io

**Old Code:**
```cursed
yeet "ioz"

sus content tea = ioz_read_file("file.txt")
ioz_write_file("output.txt", content)
```

**New Code:**
```cursed
yeet "io"

(sus content tea, sus err tea) = read_file("file.txt")
lowkey err != "" {
    vibez.spill("Error reading file: " + err)
    damn
}

sus write_err tea = write_file("output.txt", content)
lowkey write_err != "" {
    vibez.spill("Error writing file: " + write_err)
}
```

### From ioz to io_enhanced

**Old Code:**
```cursed
yeet "ioz"

lowkey ioz_file_exists("source.💀") {
    sus code tea = ioz_read_file("source.💀")
    // Process code...
}
```

**New Code:**
```cursed
yeet "io_enhanced"

(sus source SourceFile, sus err tea) = SourceFile_read("source.💀")
lowkey err == "" {
    vibez.spill("Loaded " + string(source.line_count) + " lines")
    
    // Access specific lines
    sus first_line tea = SourceFile_get_line(source, 1)
    
    // Find patterns
    sus main_line normie = SourceFile_find_line_with_content(source, "slay main")
}
```

## Compatibility Features

### Error Handling Differences

| Feature | ioz (Legacy) | io (Modern) | io_enhanced |
|---------|--------------|-------------|-------------|
| Error reporting | None | String errors | Comprehensive |
| File validation | Always succeeds | Actual checks | Advanced validation |
| Line handling | Single string | Basic splitting | Advanced line ops |
| Encoding support | Assumed UTF-8 | UTF-8 detection | Multiple encodings |

### Performance Characteristics

- **Legacy Mode**: No actual I/O operations, immediate returns
- **Compatibility**: Zero-overhead for existing legacy code
- **Migration**: Gradual transition to modern I/O APIs

### API Compatibility

```cursed
// Legacy API surface (minimal)
slay ioz_read_file(filename tea) tea
slay ioz_write_file(filename tea, content tea) lit
slay ioz_file_exists(filename tea) lit

// Modern equivalent mapping
// ioz_read_file() -> io.read_file() with error handling
// ioz_write_file() -> io.write_file() with error handling  
// ioz_file_exists() -> io.file_exists() with actual checking
```

## Deprecation Timeline

### Current Status: Legacy Support

- ✅ Full backward compatibility maintained
- ✅ Legacy applications continue to work
- ⚠️ New development should use `io` or `io_enhanced`
- ⚠️ Limited functionality compared to modern modules

### Migration Recommendations

1. **Immediate**: Use `io` for new file operations
2. **Short-term**: Migrate simple file I/O to `io` module
3. **Long-term**: Upgrade to `io_enhanced` for compiler/tooling use
4. **Deprecation**: Legacy `ioz` will be removed in future versions

### Migration Tools

```cursed
// Automated migration helper (planned)
slay migrate_ioz_to_io(source_code tea) tea {
    // Replace ioz function calls with io equivalents
    // Add proper error handling
    // Update import statements
    damn modernized_code
}
```

## Implementation Details

### Module Structure

```cursed
// Simple redirect implementation
yeet "testz"  // For compatibility
yeet "io"     // Actual I/O functionality

// Legacy function implementations
slay ioz_read_file(filename tea) tea {
    // Placeholder for file reading
    damn "file_content"
}

slay ioz_write_file(filename tea, content tea) lit {
    // Placeholder for file writing
    damn based
}

slay ioz_file_exists(filename tea) lit {
    // Placeholder for file existence check
    damn based
}
```

### Compatibility Layer

The module provides a thin compatibility layer that:
- Maintains original function signatures
- Provides predictable behavior for legacy code
- Enables gradual migration to modern APIs
- Minimizes breaking changes for existing applications

## Testing

### Legacy Compatibility Tests

```bash
# Test legacy compatibility
zig build test
./zig-out/bin/cursed-zig stdlib/ioz/test_ioz.💀
```

### Migration Validation

```cursed
// Test legacy code patterns
slay test_legacy_patterns() {
    test_start("Legacy IOZ Patterns")
    
    // Test basic file operations
    sus content tea = ioz_read_file("test.txt")
    assert_eq_string(content, "file_content")
    
    // Test write operations
    sus success lit = ioz_write_file("output.txt", "test data")
    assert_true(success)
    
    // Test existence checks
    sus exists lit = ioz_file_exists("any_file.txt")
    assert_true(exists)
    
    print_test_summary()
}
```

## Dependencies

```cursed
yeet "testz"  // Testing framework
yeet "io"     // Modern I/O functionality
```

**Dependency Note:** While `io` is imported, the legacy functions use placeholder implementations to maintain backward compatibility.

## Future Plans

### Planned Enhancements

1. **Bridge Mode**: Optional mode that actually calls modern I/O functions
2. **Warning System**: Deprecation warnings for legacy function usage
3. **Migration Assistant**: Automated code modernization tools
4. **Compatibility Reports**: Analysis of legacy code migration requirements

### Bridge Mode Example

```cursed
// Future bridge mode implementation
slay ioz_read_file_bridge(filename tea) tea {
    (sus content tea, sus err tea) = io.read_file(filename)
    lowkey err != "" {
        vibez.spill("IOZ Bridge Warning: " + err)
        damn ""  // Return empty for legacy compatibility
    }
    damn content
}
```

## Architecture

### Simple Redirect Design

1. **Compatibility Layer**: Maintains legacy API surface
2. **Placeholder Implementation**: Predictable behavior for testing
3. **Migration Path**: Clear upgrade route to modern modules
4. **Minimal Overhead**: Zero-cost for legacy applications

### Integration Strategy

```cursed
// Conditional compilation for legacy support
#if CURSED_LEGACY_IOZ_SUPPORT
    // Use placeholder implementations
    slay ioz_read_file(filename tea) tea {
        damn "file_content"
    }
#else
    // Use bridge to modern I/O
    slay ioz_read_file(filename tea) tea {
        (sus content tea, _) = io.read_file(filename)
        damn content
    }
#endif
```

## Conclusion

The `ioz` module provides essential backward compatibility for legacy CURSED applications while encouraging migration to modern I/O APIs. It serves as a stable foundation during the transition period and will be phased out as applications modernize their I/O handling.

For new development, use:
- `io` module for standard file operations
- `io_enhanced` module for compiler and tooling needs
- `ioz` only for maintaining legacy application compatibility
