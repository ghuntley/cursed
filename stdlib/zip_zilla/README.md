# zip_zilla - Archive/Compression Module

The `zip_zilla` module provides comprehensive archive and compression functionality for CURSED, implemented in pure CURSED without FFI dependencies.

## Features

### Core Compression Algorithms
- **Deflate compression/decompression** - Standard compression algorithm used in ZIP and GZIP
- **GZIP compression/decompression** - Popular compression format with headers
- **Compression level support** - Configurable compression levels (1-9)
- **Round-trip integrity** - Guaranteed data integrity through compression/decompression cycles

### Archive Management
- **ZIP archive creation** - Create ZIP archives from multiple files
- **ZIP archive extraction** - Extract files from ZIP archives
- **Archive integrity verification** - Verify archive structure and data integrity
- **Multi-file support** - Handle multiple files in a single archive

### Compression Utilities
- **Compression ratio calculation** - Measure compression efficiency
- **Benchmarking tools** - Compare different compression algorithms
- **Checksum calculation** - Data integrity verification
- **File compression/decompression** - High-level file operations

## Usage Examples

### Basic Compression

```cursed
yeet "zip_zilla"

// Compress data using deflate algorithm
sus original_data tea = "Hello, world! This is test data for compression."
sus compressed tea = deflate_compress(original_data, 6)
sus decompressed tea = deflate_decompress(compressed)

// Calculate compression ratio
sus ratio meal = calculate_compression_ratio(original_data.length, compressed.length)
vibez.spill("Compression ratio: " + ratio + "%")
```

### GZIP Compression

```cursed
yeet "zip_zilla"

// Compress and decompress using GZIP
sus data tea = "GZIP compression example"
sus gzip_data tea = gzip_compress(data, 9)
sus (success, decompressed) = gzip_decompress(gzip_data)

yef success {
    vibez.spill("GZIP decompression successful: " + decompressed)
} else {
    vibez.spill("GZIP decompression failed")
}
```

### Creating ZIP Archives

```cursed
yeet "zip_zilla"

// Create a ZIP archive
sus files [tea] = ["document.txt", "config.json", "data.csv"]
sus contents [tea] = ["Document content", "Config data", "CSV data"]

sus success lit = zip_create("archive.zip", files, contents)
yef success {
    vibez.spill("ZIP archive created successfully")
}
```

### Extracting ZIP Archives

```cursed
yeet "zip_zilla"

// Extract files from ZIP archive
sus archive_data tea = "..."  // ZIP archive data
sus (success, files, contents) = zip_extract(archive_data)

yef success {
    vibez.spill("Extracted " + files.length + " files")
    bestie i := 0; i < files.length; i++ {
        vibez.spill("File: " + files[i] + " (" + contents[i].length + " bytes)")
    }
}
```

### Compression Benchmarking

```cursed
yeet "zip_zilla"

// Benchmark different compression algorithms
sus test_data tea = "Large dataset for compression benchmarking..."
sus algorithms [tea] = ["deflate", "gzip"]
sus results [meal] = benchmark_compression(test_data, algorithms)

bestie i := 0; i < algorithms.length; i++ {
    vibez.spill(algorithms[i] + " compression ratio: " + results[i] + "%")
}
```

### Archive Integrity Verification

```cursed
yeet "zip_zilla"

// Verify archive integrity
sus archive_data tea = "..."  // Archive data
sus is_valid lit = verify_archive_integrity(archive_data, "zip")

yef is_valid {
    vibez.spill("Archive integrity verified")
} else {
    vibez.spill("Archive is corrupted")
}
```

## Function Reference

### Compression Functions

#### `deflate_compress(data tea, level normie) tea`
Compresses data using the deflate algorithm.
- `data`: Input data to compress
- `level`: Compression level (1-9, higher = better compression)
- Returns: Compressed data

#### `deflate_decompress(compressed_data tea) tea`
Decompresses deflate-compressed data.
- `compressed_data`: Compressed data to decompress
- Returns: Original uncompressed data

#### `gzip_compress(data tea, level normie) tea`
Compresses data using GZIP format.
- `data`: Input data to compress
- `level`: Compression level (1-9)
- Returns: GZIP-compressed data with headers

#### `gzip_decompress(gzip_data tea) (lit, tea)`
Decompresses GZIP-compressed data.
- `gzip_data`: GZIP-compressed data
- Returns: (success, decompressed_data) tuple

### Archive Functions

#### `zip_create(filename tea, files [tea], contents [tea]) lit`
Creates a ZIP archive from files and contents.
- `filename`: Name of the ZIP archive
- `files`: Array of filenames
- `contents`: Array of file contents
- Returns: Success status

#### `zip_extract(archive_data tea) (lit, [tea], [tea])`
Extracts files from a ZIP archive.
- `archive_data`: ZIP archive data
- Returns: (success, filenames, contents) tuple

#### `create_archive(archive_name tea, files [tea], archive_type tea) lit`
Creates an archive with specified format.
- `archive_name`: Name of the archive
- `files`: Array of files to include
- `archive_type`: Archive format ("zip")
- Returns: Success status

#### `extract_archive(archive_name tea, archive_type tea) (lit, [tea], [tea])`
Extracts files from an archive.
- `archive_name`: Name of the archive
- `archive_type`: Archive format ("zip")
- Returns: (success, filenames, contents) tuple

### Utility Functions

#### `calculate_compression_ratio(original_size normie, compressed_size normie) meal`
Calculates compression ratio as a percentage.
- `original_size`: Size of original data
- `compressed_size`: Size of compressed data
- Returns: Compression ratio percentage

#### `compress_file(filename tea, compression_type tea, level normie) lit`
Compresses a file using specified algorithm.
- `filename`: Name of the file to compress
- `compression_type`: Compression algorithm ("deflate", "gzip")
- `level`: Compression level (1-9)
- Returns: Success status

#### `decompress_file(filename tea, compression_type tea) (lit, tea)`
Decompresses a file using specified algorithm.
- `filename`: Name of the file to decompress
- `compression_type`: Compression algorithm ("deflate", "gzip")
- Returns: (success, decompressed_data) tuple

#### `benchmark_compression(data tea, algorithms [tea]) [meal]`
Benchmarks compression algorithms on given data.
- `data`: Test data for benchmarking
- `algorithms`: Array of algorithm names
- Returns: Array of compression ratios

#### `verify_archive_integrity(archive_data tea, archive_type tea) lit`
Verifies the integrity of an archive.
- `archive_data`: Archive data to verify
- `archive_type`: Archive format ("zip", "gzip")
- Returns: Integrity status

#### `calculate_checksum(data tea) normie`
Calculates a simple checksum for data integrity.
- `data`: Data to calculate checksum for
- Returns: Checksum value

## Implementation Notes

### Pure CURSED Implementation
- No FFI dependencies - implemented entirely in CURSED
- Portable across all CURSED environments
- Self-contained compression algorithms
- Suitable for embedded or restricted environments

### Performance Characteristics
- Deflate algorithm uses simplified LZ77 compression
- GZIP includes standard headers for compatibility
- ZIP archives support basic file storage and compression
- Compression ratios depend on data characteristics

### Limitations
- Simplified compression algorithms for pure CURSED implementation
- Basic ZIP archive support (full ZIP specification not implemented)
- No encryption or advanced compression features
- File I/O operations are placeholders in current implementation

### Testing
The module includes comprehensive tests covering:
- All compression algorithms and formats
- Edge cases and error conditions
- Round-trip integrity verification
- Performance benchmarking
- Archive creation and extraction

## Error Handling

The module includes robust error handling:
- Invalid archive formats return appropriate error codes
- Compression/decompression failures are detected
- Data integrity is verified through checksums
- Empty and malformed data is handled gracefully

## Future Enhancements

Potential improvements for future versions:
- Advanced compression algorithms (LZMA, Brotli)
- Full ZIP specification compliance
- Encryption support for archives
- Streaming compression for large files
- Multi-threading for parallel compression

## Testing

Run the test suite to verify functionality:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/zip_zilla/test_zip_zilla.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/zip_zilla/test_zip_zilla.csd
./test_zip_zilla
```

The test suite includes 43 comprehensive tests covering all module functionality.
