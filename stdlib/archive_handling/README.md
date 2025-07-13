# Archive Handling Module

The `archive_handling` module provides comprehensive archive management functionality for TAR, ZIP, GZIP, and BZIP2 formats. This is a pure CURSED implementation without FFI dependencies.

## Features

- **Multiple Format Support**: TAR, ZIP, GZIP, BZIP2
- **Compression Control**: Configurable compression levels (0-9)
- **File Operations**: Add, remove, extract files and directories
- **Metadata Management**: Comments, custom metadata, creation time
- **Password Protection**: Set and remove archive passwords
- **Batch Operations**: Create and extract multiple archives
- **Archive Validation**: Integrity checking and repair
- **Format Conversion**: Convert between different archive formats
- **Archive Splitting**: Split large archives into smaller parts
- **Advanced Features**: Incremental archives, signature verification, file search

## Basic Usage

### Creating an Archive

```cursed
yeet "archive_handling"

# Create a new ZIP archive
sus success lit = archive_create("my_archive.zip", "zip")
bestie success {
    vibez.spill("Archive created successfully!")
}

# Add files to the archive
archive_add_file("local_file.txt", "archived_file.txt")
archive_add_directory("local_dir", "archived_dir")

# Close the archive
archive_close()
```

### Opening and Extracting Archives

```cursed
yeet "archive_handling"

# Open an existing archive
sus opened lit = archive_open("existing_archive.zip")
bestie opened {
    # List archive contents
    sus files tea = archive_list_files()
    vibez.spill("Archive contains: " + files)
    
    # Extract specific file
    archive_extract_file("file1.txt", "output_file.txt")
    
    # Extract all files
    archive_extract_all("output_directory")
    
    archive_close()
}
```

### Archive Information

```cursed
yeet "archive_handling"

archive_open("sample.zip")

# Get archive statistics
sus file_count normie = archive_get_file_count()
sus total_size normie = archive_get_total_size()
sus compression_ratio meal = archive_get_compression_ratio()

vibez.spill("Files: " + file_count)
vibez.spill("Total size: " + total_size + " bytes")
vibez.spill("Compression ratio: " + compression_ratio)

# Check if specific file exists
bestie archive_file_exists("important.txt") {
    sus file_size normie = archive_get_file_size("important.txt")
    vibez.spill("important.txt size: " + file_size + " bytes")
}

archive_close()
```

## API Reference

### Archive Management

- `archive_create(filename tea, format tea) lit` - Create new archive
- `archive_open(filename tea) lit` - Open existing archive
- `archive_close() lit` - Close current archive
- `archive_is_open() lit` - Check if archive is open
- `archive_get_type() tea` - Get archive format
- `archive_get_filename() tea` - Get archive filename

### File Operations

- `archive_add_file(filepath tea, archive_path tea) lit` - Add file to archive
- `archive_add_directory(dirpath tea, archive_path tea) lit` - Add directory to archive
- `archive_remove_file(archive_path tea) lit` - Remove file from archive
- `archive_extract_file(archive_path tea, output_path tea) lit` - Extract specific file
- `archive_extract_all(output_directory tea) lit` - Extract all files

### Archive Information

- `archive_list_files() tea` - Get list of files in archive
- `archive_get_file_count() normie` - Get number of files
- `archive_get_file_size(archive_path tea) normie` - Get specific file size
- `archive_get_total_size() normie` - Get total archive size
- `archive_file_exists(archive_path tea) lit` - Check if file exists
- `archive_get_file_info(archive_path tea) tea` - Get detailed file information

### Compression Settings

- `archive_set_compression_level(level normie) lit` - Set compression level (0-9)
- `archive_get_compression_level() normie` - Get current compression level
- `archive_enable_compression() lit` - Enable compression
- `archive_disable_compression() lit` - Disable compression
- `archive_get_compression_ratio() meal` - Get compression ratio

### Archive Validation

- `archive_validate() lit` - Validate archive integrity
- `archive_repair() lit` - Attempt to repair damaged archive
- `archive_test_integrity() lit` - Test archive integrity
- `archive_verify_signature() lit` - Verify archive signature

### Metadata Management

- `archive_set_comment(comment tea) lit` - Set archive comment
- `archive_get_comment() tea` - Get archive comment
- `archive_set_metadata(key tea, value tea) lit` - Set custom metadata
- `archive_get_metadata(key tea) tea` - Get custom metadata
- `archive_get_creation_time() tea` - Get archive creation time
- `archive_get_stats() tea` - Get comprehensive statistics

### Password Protection

- `archive_set_password(password tea) lit` - Set archive password
- `archive_remove_password() lit` - Remove archive password
- `archive_is_password_protected() lit` - Check if password protected

### Format Conversion

- `archive_convert_format(new_format tea) lit` - Convert to different format
- `archive_split(max_size normie) lit` - Split archive into parts
- `archive_merge(part_files tea) lit` - Merge archive parts

### Batch Operations

- `archive_batch_create(file_list tea, archive_name tea, format tea) lit` - Create archive from file list
- `archive_batch_extract(archive_list tea, output_dir tea) normie` - Extract multiple archives

### Advanced Features

- `archive_create_incremental(base_archive tea, changes tea) lit` - Create incremental archive
- `archive_create_index() lit` - Create archive index for faster access
- `archive_search_files(pattern tea) tea` - Search files by pattern

## Supported Formats

### TAR (Tape Archive)
- Standard TAR format
- Preserves file permissions and metadata
- No built-in compression (use with GZIP/BZIP2)

### ZIP 
- ZIP archive format
- Built-in compression support
- Password protection available
- Metadata support

### GZIP
- GNU zip compression
- Single file compression
- High compression ratio
- Fast compression/decompression

### BZIP2
- Burrows-Wheeler compression
- Better compression than GZIP
- Slower but higher compression ratio

## Examples

### Comprehensive Archive Management

```cursed
yeet "archive_handling"

# Create a new archive with compression
archive_create("backup.zip", "zip")
archive_set_compression_level(6)
archive_set_comment("Daily backup archive")

# Add multiple files and directories
archive_add_file("/home/user/document.txt", "documents/document.txt")
archive_add_directory("/home/user/photos", "photos")

# Set metadata
archive_set_metadata("creator", "backup_script")
archive_set_metadata("version", "1.0")

# Validate and close
bestie archive_validate() {
    vibez.spill("Archive validation passed")
    archive_close()
} else {
    vibez.spill("Archive validation failed")
    archive_repair()
    archive_close()
}
```

### Batch Processing

```cursed
yeet "archive_handling"

# Create multiple archives from file lists
sus files1 tea = "file1.txt,file2.txt,file3.txt"
sus files2 tea = "data1.csv,data2.csv"

archive_batch_create(files1, "documents.zip", "zip")
archive_batch_create(files2, "data.tar", "tar")

# Extract multiple archives
sus archives tea = "archive1.zip,archive2.tar,archive3.gz"
sus extracted_count normie = archive_batch_extract(archives, "/tmp/extracted")
vibez.spill("Extracted " + extracted_count + " archives")
```

### Archive Analysis

```cursed
yeet "archive_handling"

archive_open("analysis_target.zip")

# Get comprehensive statistics
sus stats tea = archive_get_stats()
vibez.spill("Archive statistics: " + stats)

# Search for specific files
sus txt_files tea = archive_search_files("*.txt")
vibez.spill("Text files found: " + txt_files)

# Analyze compression effectiveness
sus ratio meal = archive_get_compression_ratio()
bestie ratio < 0.5 {
    vibez.spill("Excellent compression achieved")
} else bestie ratio < 0.8 {
    vibez.spill("Good compression")
} else {
    vibez.spill("Poor compression - consider different format")
}

archive_close()
```

## Error Handling

All functions return boolean values (`lit` type) to indicate success (`based`) or failure (`cap`). Always check return values:

```cursed
yeet "archive_handling"

sus result lit = archive_create("test.zip", "zip")
bestie !result {
    vibez.spill("Failed to create archive")
    damn
}

# Continue with archive operations...
```

## Testing

Run the comprehensive test suite:

```bash
# Interpretation mode
cargo run --bin cursed stdlib/archive_handling/test_archive_handling.csd

# Compilation mode  
cargo run --bin cursed -- compile stdlib/archive_handling/test_archive_handling.csd
./test_archive_handling
```

## Implementation Notes

- This is a pure CURSED implementation without FFI dependencies
- All operations are simulated for demonstration purposes
- Real file I/O would require integration with the dropz module
- Compression algorithms are abstracted for the pure CURSED environment
- Password protection uses secure hashing (implementation dependent)
- Archive validation includes format-specific integrity checks

## Performance Considerations

- Use appropriate compression levels (6 is default, good balance)
- For large files, consider splitting archives
- Batch operations are more efficient than individual operations
- Archive indexing improves search performance for large archives
- Choose format based on use case:
  - ZIP: General purpose, good compatibility
  - TAR: Unix/Linux systems, preserves permissions
  - GZIP: Single file compression, fast
  - BZIP2: Maximum compression, slower

## Security Notes

- Always validate archives before extraction
- Use password protection for sensitive data
- Verify archive signatures when available
- Be cautious with file paths during extraction (path traversal attacks)
- Test archive integrity before deployment
