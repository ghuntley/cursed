# zip_zilla - Comprehensive Archive Handling Module

The `zip_zilla` module provides complete archive handling functionality for the CURSED language with Gen Z enhanced APIs. It supports multiple archive formats including ZIP, TAR, TAR.GZ, TAR.BZ2, and 7-ZIP with advanced features like password protection, progress reporting, and archive modification.

## Features

### 🔥 Core Archive Operations
- **ZIP Archive Creation & Extraction** - Standard ZIP format support
- **TAR Archive Support** - Uncompressed and compressed TAR archives
- **Multiple Compression Formats** - TAR.GZ, TAR.BZ2 support
- **Password Protection** - Secure archive encryption
- **Archive Validation** - Integrity checking and format detection

### ✨ Advanced Functionality
- **Progress Reporting** - Real-time progress for large archives
- **Single File Extraction** - Surgical file extraction without full extraction
- **Archive Modification** - Add/remove files from existing archives
- **Recompression** - Optimize existing archives with better compression
- **Directory Archiving** - Recursive directory archiving with progress
- **Archive Information** - Detailed archive statistics and metadata

### 🛡️ Security & Reliability
- **Robust Error Handling** - Comprehensive error detection and reporting
- **Password Strength Validation** - Ensures secure passwords for protected archives
- **Corruption Detection** - Validates archive integrity before operations
- **Safe Extraction** - Prevents directory traversal attacks

## Quick Start

```cursed
yeet "zip_zilla"
yeet "dropz"

# Create a simple ZIP archive
sus files [tea] = ["file1.txt", "file2.txt", "file3.txt"]
sus result tea = zip_zilla.create_zip_archive("my_archive.zip", files, zip_zilla.BALANCED_COMPRESSION)
vibez.spill(result)

# Extract the archive
sus extract_result tea = zip_zilla.extract_zip_archive("my_archive.zip", "extracted_files", "")
vibez.spill(extract_result)
```

## Archive Formats

### ZIP Format
```cursed
# Create ZIP with different compression levels
zip_zilla.create_zip_archive("fast.zip", files, zip_zilla.FAST_COMPRESSION)
zip_zilla.create_zip_archive("balanced.zip", files, zip_zilla.BALANCED_COMPRESSION)
zip_zilla.create_zip_archive("max.zip", files, zip_zilla.MAX_COMPRESSION)

# Password-protected ZIP
zip_zilla.create_protected_archive("secure.zip", files, "supersecret123", zip_zilla.MAX_COMPRESSION)
```

### TAR Format
```cursed
# Create different TAR formats
zip_zilla.create_tar_archive("archive.tar", "source_directory", zip_zilla.TAR_FORMAT)
zip_zilla.create_tar_archive("archive.tar.gz", "source_directory", zip_zilla.TAR_GZ_FORMAT)
zip_zilla.create_tar_archive("archive.tar.bz2", "source_directory", zip_zilla.TAR_BZ2_FORMAT)
```

## API Reference

### Constants

```cursed
# Archive formats
facts ZIP_FORMAT normie = 1
facts TAR_FORMAT normie = 2
facts TAR_GZ_FORMAT normie = 3
facts TAR_BZ2_FORMAT normie = 4
facts SEVEN_ZIP_FORMAT normie = 5

# Compression levels
facts NO_COMPRESSION normie = 0
facts FAST_COMPRESSION normie = 1
facts BALANCED_COMPRESSION normie = 5
facts MAX_COMPRESSION normie = 9
```

### Core Functions

#### `create_zip_archive(archive_path tea, file_paths [tea], compression_level normie) ArchiveResult`
Creates a ZIP archive with specified files and compression level.

**Parameters:**
- `archive_path` - Path where the ZIP archive will be created
- `file_paths` - Array of file paths to include in the archive
- `compression_level` - Compression level (0-9, use constants)

**Returns:** Result message indicating success or error details

**Example:**
```cursed
sus files [tea] = ["doc1.txt", "doc2.txt", "images/photo.jpg"]
sus result tea = zip_zilla.create_zip_archive("documents.zip", files, zip_zilla.BALANCED_COMPRESSION)
vibez.spill(result)
```

#### `extract_zip_archive(archive_path tea, destination_path tea, password tea) ArchiveResult`
Extracts a ZIP archive to the specified destination.

**Parameters:**
- `archive_path` - Path to the ZIP archive to extract
- `destination_path` - Directory where files will be extracted
- `password` - Password for protected archives (empty string if not protected)

**Returns:** Result message indicating success or error details

#### `create_tar_archive(archive_path tea, source_directory tea, compression_type normie) ArchiveResult`
Creates a TAR archive from a source directory.

**Parameters:**
- `archive_path` - Path where the TAR archive will be created
- `source_directory` - Directory to archive recursively
- `compression_type` - TAR format type (TAR_FORMAT, TAR_GZ_FORMAT, TAR_BZ2_FORMAT)

### Advanced Functions

#### `create_protected_archive(archive_path tea, file_paths [tea], password tea, compression_level normie) ArchiveResult`
Creates a password-protected ZIP archive.

**Example:**
```cursed
sus sensitive_files [tea] = ["confidential.doc", "private_key.pem"]
sus result tea = zip_zilla.create_protected_archive("secure_backup.zip", sensitive_files, "mySecurePass123", zip_zilla.MAX_COMPRESSION)
```

#### `list_archive_contents(archive_path tea) [ArchiveEntry]`
Lists all files and directories in an archive without extracting.

**Returns:** Array of ArchiveEntry tuples containing (path, size, modified_time, is_directory)

**Example:**
```cursed
sus contents [zip_zilla.ArchiveEntry] = zip_zilla.list_archive_contents("my_archive.zip")
bestie i normie = 0; i < contents.length; i++ {
    sus entry zip_zilla.ArchiveEntry = contents[i]
    vibez.spill(stringz.format("File: {}, Size: {} bytes", entry.0, entry.1))
}
```

#### `validate_archive_integrity(archive_path tea) lit`
Validates the integrity of an archive file.

**Returns:** `based` if archive is valid, `cap` if corrupted or invalid

#### `detect_archive_format(archive_path tea) normie`
Automatically detects the format of an archive file.

**Returns:** Format constant (ZIP_FORMAT, TAR_FORMAT, etc.) or 0 for unknown

#### `extract_single_file(archive_path tea, file_path tea, destination_path tea) ArchiveResult`
Extracts a single file from an archive without extracting everything.

**Example:**
```cursed
# Extract just one specific file
sus result tea = zip_zilla.extract_single_file("large_archive.zip", "docs/important.txt", "important_extracted.txt")
```

#### `archive_directory_with_progress(source_dir tea, archive_path tea, progress_callback ProgressCallback) ArchiveResult`
Archives a directory with real-time progress reporting.

**Example:**
```cursed
# Define progress callback
slay my_progress_callback(current normie, total normie) lit {
    sus percent normie = (current * 100) / total
    vibez.spill(stringz.format("Progress: {}% ({}/{})", percent, current, total))
    damn based
}

# Archive with progress
sus result tea = zip_zilla.archive_directory_with_progress("large_project", "project_backup.zip", my_progress_callback)
```

#### `get_archive_info(archive_path tea) tea`
Retrieves detailed information about an archive.

**Returns:** Formatted string with archive statistics and metadata

#### `add_file_to_archive(archive_path tea, file_path tea) ArchiveResult`
Adds a file to an existing archive.

#### `remove_file_from_archive(archive_path tea, file_path tea) ArchiveResult`
Removes a file from an existing archive.

#### `recompress_archive(archive_path tea, new_compression_level normie) ArchiveResult`
Recompresses an existing archive with a different compression level.

## Error Handling

The `zip_zilla` module provides comprehensive error handling with descriptive messages:

```cursed
sus result tea = zip_zilla.create_zip_archive("test.zip", ["nonexistent.txt"], zip_zilla.BALANCED_COMPRESSION)
lowkey stringz.contains(result, "Error") {
    vibez.spill("Operation failed: " + result)
} else {
    vibez.spill("Success: " + result)
}
```

Common error scenarios:
- **File not found** - When input files don't exist
- **Invalid compression level** - When compression level is outside 0-9 range
- **Weak password** - When password is less than 8 characters
- **Corrupted archive** - When archive fails integrity validation
- **Permission denied** - When unable to create directories or files
- **Archive not found** - When trying to extract nonexistent archives

## Best Practices

### Compression Levels
- **NO_COMPRESSION (0)** - Fastest, largest file size
- **FAST_COMPRESSION (1)** - Good balance for frequent operations
- **BALANCED_COMPRESSION (5)** - Recommended default for most use cases
- **MAX_COMPRESSION (9)** - Smallest file size, slower compression

### Password Security
- Use passwords with at least 8 characters
- Include mix of letters, numbers, and symbols
- Avoid common words or predictable patterns

### Large Archives
- Use progress callbacks for operations that might take time
- Consider TAR.GZ for better compression ratios on large datasets
- Validate archives after creation for critical data

### Memory Efficiency
- Use single file extraction when you only need specific files
- List contents before extraction to understand archive structure
- Clean up temporary files and directories after operations

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/zip_zilla/test_zip_zilla.csd
```

The test suite covers:
- ZIP creation and extraction
- TAR format support
- Password protection
- Format detection and validation
- Archive modification
- Error handling scenarios
- Compression level testing
- Progress reporting
- Single file operations

## Dependencies

The `zip_zilla` module depends on:
- `testz` - Testing framework
- `dropz` - File I/O operations
- `squish_core` - Core compression algorithms
- `encode_mood` - Encoding/decoding utilities
- `stringz` - String manipulation
- `error_drip` - Error handling utilities

## Integration Example

```cursed
yeet "zip_zilla"
yeet "dropz"
yeet "stringz"

# Complete backup workflow
slay backup_project(project_dir tea, backup_name tea) lit {
    # Create timestamped backup name
    sus timestamp tea = timez.now_rfc3339()
    sus backup_path tea = stringz.format("backups/{}_{}.tar.gz", backup_name, timestamp)
    
    # Archive with progress reporting
    slay progress_handler(current normie, total normie) lit {
        lowkey current % 100 == 0 {  # Report every 100 files
            sus percent normie = (current * 100) / total
            vibez.spill(stringz.format("Backup progress: {}%", percent))
        }
        damn based
    }
    
    # Create compressed backup
    sus result tea = zip_zilla.archive_directory_with_progress(project_dir, backup_path, progress_handler)
    
    # Validate backup
    lowkey stringz.contains(result, "successfully") {
        sus valid lit = zip_zilla.validate_archive_integrity(backup_path)
        lowkey valid {
            vibez.spill("✅ Backup completed and validated successfully!")
            damn based
        } else {
            vibez.spill("❌ Backup created but validation failed!")
            damn cap
        }
    } else {
        vibez.spill("❌ Backup failed: " + result)
        damn cap
    }
}

# Usage
backup_project("my_important_project", "daily_backup")
```

## License

Part of the CURSED standard library. Licensed under the same terms as the CURSED programming language.

---

*zip_zilla - Making archive handling absolutely fire! 🔥📁*
