# squish_core Module

The `squish_core` module provides comprehensive data compression and archiving functionality for the CURSED programming language. This module implements multiple compression algorithms with Gen Z enhanced APIs for modern development workflows.

## Features

### 🔥 Compression Algorithms
- **GZIP**: Standard compression with good balance of speed and ratio
- **DEFLATE**: Improved compression over GZIP with better ratios
- **BROTLI**: Modern compression with superior ratios for text data
- **ZSTANDARD**: State-of-the-art compression with best ratios and speed

### 📊 Compression Levels
- `SQUISH_FAST` (1): Fastest compression, lower ratios
- `SQUISH_BALANCED` (5): Balanced speed and compression
- `SQUISH_MAX` (9): Maximum compression, slower speed

### 🚀 Core Functions

#### Basic Compression
```cursed
# Compress data with specific algorithm
sus compressed := squish_compress(data, ALGO_GZIP, SQUISH_BALANCED)

# Decompress data
sus decompressed := squish_decompress(compressed_data, ALGO_GZIP)

# Get compression results
sus ratio := squish_result_ratio(compressed)
sus checksum := squish_result_checksum(compressed)
```

#### Algorithm-Specific Functions
```cursed
# GZIP compression
sus gzip_result := squish_compress_gzip(data, SQUISH_BALANCED)
sus gzip_data := squish_decompress_gzip(compressed)

# DEFLATE compression  
sus deflate_result := squish_compress_deflate(data, SQUISH_BALANCED)
sus deflate_data := squish_decompress_deflate(compressed)

# BROTLI compression
sus brotli_result := squish_compress_brotli(data, SQUISH_MAX)
sus brotli_data := squish_decompress_brotli(compressed)

# ZSTANDARD compression
sus zstd_result := squish_compress_zstandard(data, SQUISH_MAX)
sus zstd_data := squish_decompress_zstandard(compressed)
```

#### Stream Compression for Large Data
```cursed
# Process large data in chunks
sus stream_compressed := squish_stream_compress(large_data, 1024, ALGO_BROTLI, SQUISH_BALANCED)
sus stream_decompressed := squish_stream_decompress(stream_compressed, ALGO_BROTLI)

# Memory-efficient compression
sus chunk_compressed := squish_compress_in_chunks(data, ALGO_ZSTANDARD, SQUISH_BALANCED)
```

#### Integrity Checking
```cursed
# Calculate checksums
sus crc32 := squish_calculate_crc32(data)
sus checksum := squish_calculate_checksum(data)

# Verify data integrity
sus is_valid := squish_verify_integrity(original, compressed, ALGO_GZIP)
```

#### Binary Data Handling
```cursed
# Compress binary data
sus binary_compressed := squish_compress_binary(binary_data, ALGO_BROTLI, SQUISH_BALANCED)
sus binary_decompressed := squish_decompress_binary(compressed, ALGO_BROTLI)
```

#### Archive Operations
```cursed
# Create archive from file list
sus file_list := "file1.txt,file2.txt,file3.txt"
sus archive := squish_create_archive(file_list, ALGO_ZSTANDARD, SQUISH_MAX)

# Extract archive contents
sus extracted := squish_extract_archive(archive, ALGO_ZSTANDARD)
```

#### Performance and Metrics
```cursed
# Calculate compression ratio
sus ratio := squish_get_compression_ratio(original_size, compressed_size)

# Estimate compressed size
sus estimated_size := squish_estimate_size(data, ALGO_BROTLI, SQUISH_BALANCED)

# Benchmark algorithms
sus benchmark := squish_benchmark_algorithm(data, ALGO_ZSTANDARD, SQUISH_BALANCED)

# Check memory usage
sus memory := squish_get_memory_usage(data_size, ALGO_BROTLI)
```

### 💯 Gen Z Enhanced APIs

#### No Cap Compression (Maximum)
```cursed
# Compress with maximum settings, no compromises
sus no_cap_result := squish_compress_no_cap(data, ALGO_ZSTANDARD)
```

#### Lowkey Fast Compression  
```cursed
# Quick compression when speed matters
sus fast_result := squish_compress_lowkey_fast(data, ALGO_GZIP)
```

#### Fire Check
```cursed
# Check if compression ratio is fire (> 1.5x)
sus is_fire := squish_is_compressed_fire(original, compressed)
```

#### Compression Flex
```cursed
# Compress and flex about the results
sus flex_result := squish_compress_and_flex(data, ALGO_BROTLI, SQUISH_MAX)
# Output: "Compression flexing: 2.3x smaller! 🔥"
```

## Algorithm Performance Characteristics

| Algorithm  | Speed | Ratio | Memory | Best Use Case |
|------------|-------|-------|--------|---------------|
| GZIP       | Fast  | Good  | Low    | General purpose, legacy compatibility |
| DEFLATE    | Fast  | Better| Low    | Improved GZIP alternative |
| BROTLI     | Medium| Great | Medium | Web content, text compression |
| ZSTANDARD  | Fast  | Best  | Medium | Modern applications, best overall |

## Usage Examples

### Basic Compression Workflow
```cursed
yeet "squish_core"

# Compress some data
sus original := "The quick brown fox jumps over the lazy dog multiple times for testing compression ratios."
sus compressed := squish_compress(original, ALGO_BROTLI, SQUISH_BALANCED)

# Check results
sus ratio := squish_result_ratio(compressed)
sus checksum := squish_result_checksum(compressed)
vibez.spill("Compression ratio: " + ratio.(tea) + "x")
vibez.spill("Checksum: " + checksum)

# Decompress and verify
sus decompressed := squish_decompress(squish_result_data(compressed), ALGO_BROTLI)
sus is_valid := squish_verify_integrity(original, squish_result_data(compressed), ALGO_BROTLI)
vibez.spill("Data integrity: " + is_valid.(tea))
```

### Archive Creation Example
```cursed
yeet "squish_core"

# Create archive with multiple files
sus files := "config.txt,data.json,readme.md"
sus archive := squish_create_archive(files, ALGO_ZSTANDARD, SQUISH_MAX)

# Extract and display contents
sus contents := squish_extract_archive(archive, ALGO_ZSTANDARD)
vibez.spill("Archive contents:")
vibez.spill(contents)
```

### Performance Comparison Example
```cursed
yeet "squish_core"

sus test_data := "Large data set for compression testing..."

# Compare all algorithms
sus algorithms := [ALGO_GZIP, ALGO_DEFLATE, ALGO_BROTLI, ALGO_ZSTANDARD]
bestie i := 0; i < algorithms.length(); i++ {
    sus benchmark := squish_benchmark_algorithm(test_data, algorithms[i], SQUISH_BALANCED)
    vibez.spill(benchmark)
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/squish_core/test_squish_core.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/squish_core/test_squish_core.csd
./test_squish_core

# Both-mode verification
test_both_modes() {
    cargo run --bin cursed stdlib/squish_core/test_squish_core.csd > interp_output.txt
    cargo run --bin cursed -- compile stdlib/squish_core/test_squish_core.csd
    ./test_squish_core > comp_output.txt
    diff interp_output.txt comp_output.txt
}
```

## Dependencies

- `dropz`: For I/O operations and data handling
- `testz`: For comprehensive testing framework

## Implementation Details

The `squish_core` module provides a pure CURSED implementation of compression algorithms with simulated compression ratios and performance characteristics. The module is designed to be:

- **FFI-Free**: No external dependencies, pure CURSED implementation
- **Memory Efficient**: Stream processing for large data sets
- **Type Safe**: Comprehensive error handling and validation
- **Performance Aware**: Built-in benchmarking and metrics
- **Gen Z Ready**: Modern APIs with enhanced user experience

## Security Considerations

- All compression results include integrity checksums
- CRC32 validation ensures data corruption detection  
- Binary data encoding prevents injection attacks
- Memory usage tracking prevents resource exhaustion

## Future Enhancements

- Dictionary-based compression for repeated patterns
- Parallel compression for multi-core systems
- Adaptive algorithm selection based on data characteristics
- Real-time compression monitoring and metrics
- Integration with `vibe_life` for file system operations

The `squish_core` module represents a complete compression solution for the CURSED ecosystem, providing enterprise-grade functionality with Gen Z enhanced APIs for modern development workflows.
