# archivez - Archive Format Support Package

The `archivez` module provides comprehensive archive handling functionality for ZIP, TAR, GZIP, BZIP2, LZ4, and LZMA formats. This is a pure CURSED implementation focused on performance and reliability.

## Features

### Archive Formats Supported
- **ZIP**: Popular archive format with built-in compression and password support
- **TAR**: POSIX tape archive format with metadata preservation
- **GZIP**: GNU zip compression for single files
- **BZIP2**: Burrows-Wheeler compression with high compression ratios
- **LZ4**: Fast compression/decompression with good ratios
- **LZMA**: Maximum compression ratio (slower but excellent for storage)

### Core Functionality
- **Archive Creation**: Create new archives in any supported format
- **File Operations**: Add/remove files and directories to/from archives
- **Extraction**: Extract individual files or entire archives
- **Compression Control**: Configurable compression levels (0-9)
- **Format Detection**: Automatic format detection from file extensions
- **Validation**: Archive integrity checking and repair
- **Password Protection**: Secure archives with password encryption
- **Statistics**: Detailed compression and archive statistics

### Advanced Features
- **Multiple Compression Algorithms**: Choose best algorithm for your needs
- **Performance Optimization**: Fast compression with LZ4, best ratio with LZMA
- **Memory Efficient**: Stream-based processing for large archives
- **Error Recovery**: Comprehensive error handling with detailed messages
- **Cross-Platform**: Works on all CURSED-supported platforms

## Quick Start

### Basic Usage

```cursed
yeet "archivez"

# Create a new ZIP archive
sus archive tea = create_archive("backup.zip", ZIP_FORMAT) fam {
    when err -> {
        vibez.spill("Failed to create archive: " + err)
        damn
    }
}

# Add files to the archive
add_file("document.txt", "docs/document.txt") fam {
    when err -> vibez.spill("Failed to add file: " + err)
}

add_directory("photos", "images") fam {
    when err -> vibez.spill("Failed to add directory: " + err)
}

# Set compression level
set_compression_level(6) fam {
    when err -> vibez.spill("Failed to set compression: " + err)
}

# Close the archive
close_archive()
```

### Extract Files

```cursed
yeet "archivez"

# Open existing archive
open_archive("backup.zip") fam {
    when err -> {
        vibez.spill("Failed to open archive: " + err)
        damn
    }
}

# List files in archive
sus files []tea = list_files() fam {
    when err -> {
        vibez.spill("Failed to list files: " + err)
        damn
    }
}

vibez.spill("Archive contains:")
bestie (drip i = 0; i < len(files); i = i + 1) {
    vibez.spill("  " + files[i])
}

# Extract all files
sus count drip = extract_all("output_directory") fam {
    when err -> {
        vibez.spill("Failed to extract: " + err)
        damn
    }
}

vibez.spill("Extracted " + to_string(count) + " files")
close_archive()
```

## API Reference

### Archive Management

#### `create_archive(filename tea, format tea) yikes<tea>`
Create a new archive with the specified format.

- **Parameters**:
  - `filename`: Path to the archive file
  - `format`: Archive format (`ZIP_FORMAT`, `TAR_FORMAT`, `GZIP_FORMAT`, `BZIP2_FORMAT`)
- **Returns**: Archive filename on success
- **Errors**: Invalid filename, unsupported format

#### `open_archive(filename tea) yikes<tea>`
Open an existing archive for reading/modification.

- **Parameters**:
  - `filename`: Path to the archive file
- **Returns**: Archive filename on success
- **Errors**: File not found, invalid format, corrupted archive

#### `close_archive() lit`
Close the currently open archive.

- **Returns**: `based` if successful, `cap` if no archive was open

### File Operations

#### `add_file(filepath tea, archive_path tea) yikes<tea>`
Add a file to the current archive.

- **Parameters**:
  - `filepath`: Local file path
  - `archive_path`: Path within archive (optional, defaults to `filepath`)
- **Returns**: Archive path of added file
- **Errors**: No archive open, file not found, archive path invalid

#### `add_directory(dirpath tea, archive_path tea) yikes<tea>`
Add a directory and its contents to the archive.

- **Parameters**:
  - `dirpath`: Local directory path
  - `archive_path`: Path within archive (optional, defaults to `dirpath`)
- **Returns**: Archive path of added directory
- **Errors**: No archive open, directory not found

#### `extract_file(archive_path tea, output_path tea) yikes<tea>`
Extract a single file from the archive.

- **Parameters**:
  - `archive_path`: Path within archive
  - `output_path`: Local output path (optional, defaults to `archive_path`)
- **Returns**: Extracted file data
- **Errors**: No archive open, file not found in archive

#### `extract_all(output_dir tea) yikes<drip>`
Extract all files from the archive.

- **Parameters**:
  - `output_dir`: Directory to extract files to
- **Returns**: Number of files extracted
- **Errors**: No archive open, output directory invalid

### Archive Information

#### `list_files() yikes<[]tea>`
Get list of files in the current archive.

- **Returns**: Array of file paths in archive
- **Errors**: No archive open

#### `get_file_count() yikes<drip>`
Get the number of files in the archive.

- **Returns**: File count
- **Errors**: No archive open

#### `file_exists(archive_path tea) yikes<lit>`
Check if a file exists in the archive.

- **Parameters**:
  - `archive_path`: Path to check within archive
- **Returns**: `based` if file exists, `cap` otherwise
- **Errors**: No archive open

#### `get_archive_info() yikes<tea>`
Get detailed information about the archive.

- **Returns**: Formatted string with archive statistics
- **Errors**: No archive open

### Compression

#### `set_compression_level(level drip) yikes<drip>`
Set the compression level for new files.

- **Parameters**:
  - `level`: Compression level (0-9, where 0=none, 9=maximum)
- **Returns**: The set compression level
- **Errors**: Invalid level

#### `get_compression_level() drip`
Get the current compression level.

- **Returns**: Current compression level (0-9)

### Password Protection

#### `set_password(password tea) yikes<lit>`
Set a password for the archive.

- **Parameters**:
  - `password`: Archive password
- **Returns**: `based` on success
- **Errors**: Empty password

#### `remove_password() lit`
Remove password protection from the archive.

- **Returns**: `based` on success

#### `is_password_protected() lit`
Check if the archive is password protected.

- **Returns**: `based` if password protected, `cap` otherwise

### Validation

#### `validate_archive() yikes<lit>`
Validate the integrity of the current archive.

- **Returns**: `based` if valid, `cap` if corrupted
- **Errors**: No archive open

## Compression Algorithms

### Algorithm Selection

```cursed
yeet "archivez/compression"

# Initialize compression system
init_compression()

# Choose algorithm based on needs
set_compression_algorithm(COMPRESSION_DEFLATE)  # Good balance
set_compression_algorithm(COMPRESSION_LZ4)     # Fast
set_compression_algorithm(COMPRESSION_LZMA)    # Best ratio
set_compression_algorithm(COMPRESSION_GZIP)    # Standard
set_compression_algorithm(COMPRESSION_BZIP2)   # High ratio

# Set compression level
set_compression_level(6)  # Default level

# Compress data
sus original tea = "Large text data to compress..."
sus compressed tea = compress_data(original) fam {
    when err -> {
        vibez.spill("Compression failed: " + err)
        damn
    }
}

# Decompress data
sus decompressed tea = decompress_data(compressed) fam {
    when err -> {
        vibez.spill("Decompression failed: " + err)
        damn
    }
}

# Get statistics
sus stats tea = get_compression_stats()
vibez.spill("Compression Stats:\n" + stats)
```

### Algorithm Comparison

| Algorithm | Speed | Ratio | Use Case |
|-----------|-------|-------|----------|
| **DEFLATE** | Good | Good | General purpose, ZIP files |
| **GZIP** | Good | Good | Single files, web compression |
| **BZIP2** | Slow | Excellent | Archive storage, backup |
| **LZ4** | Excellent | Fair | Real-time compression |
| **LZMA** | Slow | Excellent | Maximum compression needed |

## Format-Specific Features

### ZIP Archives

```cursed
yeet "archivez/zip"

# Initialize ZIP system
init_zip()

# Add files with metadata
zip_add_file("local.txt", "archive.txt", "file content")
zip_add_directory("source", "code")

# Set ZIP-specific options
zip_set_comment("Created by archivez")
zip_set_encryption("password123", "AES256")

# Get ZIP statistics
sus stats tea = zip_get_stats()
vibez.spill("ZIP Stats:\n" + stats)

# Validate and repair if needed
ready (!zip_validate()) {
    vibez.spill("ZIP validation failed, attempting repair...")
    zip_repair()
}
```

### TAR Archives

```cursed
yeet "archivez/tar"

# Initialize TAR system
init_tar()

# Add files with POSIX metadata
tar_add_file("document.pdf", "docs/document.pdf", "PDF content")
tar_add_directory("images", "assets/images")

# Add symbolic and hard links
tar_add_symlink("latest.txt", "version-1.2.3.txt")
tar_add_hardlink("backup.txt", "original.txt")

# Set file permissions and ownership
tar_set_file_mode("docs/document.pdf", "644")
tar_set_ownership("docs/document.pdf", "1000", "1000", "user", "group")

# Get TAR statistics
sus stats tea = tar_get_stats()
vibez.spill("TAR Stats:\n" + stats)
```

## Error Handling

The archivez package uses CURSED's structured error handling with the `yikes`/`fam` system:

```cursed
yeet "archivez"

# Comprehensive error handling example
create_archive("test.zip", ZIP_FORMAT) fam {
    when "filename cannot be empty" -> {
        vibez.spill("Please provide a filename")
        damn
    }
    when "unsupported format" -> {
        vibez.spill("Use ZIP_FORMAT, TAR_FORMAT, GZIP_FORMAT, or BZIP2_FORMAT")
        damn  
    }
    when err -> {
        vibez.spill("Unexpected error: " + err)
        damn
    }
}

# Chain operations with error handling
add_file("document.txt", "docs/document.txt") fam {
    when "no archive is open" -> {
        vibez.spill("Create or open an archive first")
        damn
    }
    when "filepath cannot be empty" -> {
        vibez.spill("Provide a valid file path")
        damn
    }
    when err -> {
        vibez.spill("File addition failed: " + err)
        # Continue with other operations
    }
}
```

## Performance Tips

### Choosing the Right Algorithm

1. **For speed**: Use LZ4 compression
2. **For size**: Use LZMA or BZIP2
3. **For balance**: Use DEFLATE (default for ZIP)
4. **For web**: Use GZIP

### Compression Level Guidelines

- **Level 0**: No compression, fastest
- **Level 1-3**: Fast compression, good for real-time
- **Level 4-6**: Default compression, good balance
- **Level 7-9**: Maximum compression, slower

### Memory Usage

```cursed
# For large files, consider streaming
bestie (file_size > 100000000) {  # 100MB
    set_compression_level(1)  # Use fast compression for large files
}

# Use appropriate format for content type
ready (is_text_file(filename)) {
    set_compression_algorithm(COMPRESSION_DEFLATE)  # Good for text
} otherwise {
    set_compression_algorithm(COMPRESSION_LZ4)      # Fast for binary
}
```

## Examples

### Backup Script

```cursed
yeet "archivez"

slay create_backup(source_dir tea, backup_name tea) {
    # Create timestamped backup archive
    sus timestamp tea = get_timestamp()
    sus archive_name tea = backup_name + "_" + timestamp + ".tar.gz"
    
    # Create TAR archive with GZIP compression
    create_archive(archive_name, TAR_FORMAT) fam {
        when err -> {
            vibez.spill("Failed to create backup: " + err)
            damn
        }
    }
    
    # Set high compression for backups
    set_compression_level(9)
    
    # Add entire source directory
    add_directory(source_dir, "backup") fam {
        when err -> {
            vibez.spill("Failed to add directory: " + err)
            close_archive()
            damn
        }
    }
    
    # Validate backup integrity
    sus valid lit = validate_archive() fam {
        when err -> {
            vibez.spill("Backup validation failed: " + err)
            close_archive()
            damn
        }
    }
    
    ready (valid) {
        vibez.spill("Backup created successfully: " + archive_name)
    } otherwise {
        vibez.spill("Backup validation failed!")
    }
    
    close_archive()
}

# Create backup
create_backup("/home/user/documents", "documents_backup")
```

### Archive Analysis Tool

```cursed
yeet "archivez"

slay analyze_archive(filename tea) {
    open_archive(filename) fam {
        when err -> {
            vibez.spill("Cannot open archive: " + err)
            damn
        }
    }
    
    # Get basic information
    sus info tea = get_archive_info() fam {
        when err -> {
            vibez.spill("Failed to get info: " + err)
            close_archive()
            damn
        }
    }
    
    vibez.spill("=== Archive Analysis ===")
    vibez.spill(info)
    
    # List all files
    sus files []tea = list_files() fam {
        when err -> {
            vibez.spill("Failed to list files: " + err)
            close_archive()
            damn
        }
    }
    
    vibez.spill("\n=== File Listing ===")
    bestie (drip i = 0; i < len(files); i = i + 1) {
        vibez.spill(files[i])
    }
    
    # Validate integrity
    sus valid lit = validate_archive() fam {
        when err -> {
            vibez.spill("Validation error: " + err)
            close_archive()
            damn
        }
    }
    
    vibez.spill("\n=== Integrity Check ===")
    ready (valid) {
        vibez.spill("✓ Archive is valid")
    } otherwise {
        vibez.spill("✗ Archive is corrupted")
    }
    
    close_archive()
}

# Analyze an archive
analyze_archive("example.zip")
```

## Testing

Run the comprehensive test suite:

```bash
# Using CURSED interpreter
./zig-out/bin/cursed-zig stdlib/archivez/test_archivez.csd

# Using CURSED compiler
./zig-out/bin/cursed-zig --compile stdlib/archivez/test_archivez.csd
./test_archivez
```

The test suite covers:
- Archive creation and format detection
- File and directory operations
- Compression algorithms and levels
- Archive validation and repair
- Password protection
- Error handling scenarios
- Performance characteristics
- Format-specific features

## Implementation Notes

### Design Principles
- **Pure CURSED**: No external dependencies or FFI calls
- **Error Safety**: Comprehensive error handling with structured errors
- **Performance**: Optimized for both speed and memory usage
- **Compatibility**: Standard-compliant format implementations
- **Extensibility**: Easy to add new formats and algorithms

### Format Compliance
- **ZIP**: PKZip 2.0+ compatible with extensions
- **TAR**: POSIX.1-1988 (ustar) format compliance
- **GZIP**: RFC 1952 compliant
- **BZIP2**: Compatible with bzip2 1.0+

### Security Considerations
- **Path Traversal**: Protected against directory traversal attacks
- **Password Storage**: Secure password handling (implementation dependent)
- **Validation**: Comprehensive format validation to prevent malicious archives
- **Memory Safety**: Bounds checking and safe memory operations

## Contributing

To add new archive formats or compression algorithms:

1. Create format-specific module in `archivez/`
2. Implement required functions following existing patterns
3. Add format constants and detection logic
4. Update main `mod.csd` to integrate new format
5. Add comprehensive tests in `test_archivez.csd`
6. Update documentation and examples

## License

This module is part of the CURSED standard library and follows the same license terms as the main CURSED project.
