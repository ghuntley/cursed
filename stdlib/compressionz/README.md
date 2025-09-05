# CURSED Enhanced Compression Module (compressionz)

## Overview

The `compressionz` module provides advanced compression algorithms implemented in pure CURSED language. This module extends beyond the basic `compression` module with enhanced implementations of LZ4, ZSTD, and GZIP algorithms, offering better compression ratios, performance optimizations, and comprehensive metrics.

## Supported Algorithms

### 1. LZ4 Enhanced
- **Type**: Fast compression algorithm
- **Best for**: Real-time applications, streaming data
- **Features**: 
  - Advanced hash table matching
  - Improved distance encoding
  - Multiple compression levels
  - Optimized for speed over compression ratio

### 2. ZSTD Advanced  
- **Type**: High-performance compression with excellent ratios
- **Best for**: File archiving, database compression, general purpose
- **Features**:
  - Block-based compression
  - Finite State Entropy (FSE) encoding
  - Adaptive compression context
  - Superior compression ratios

### 3. GZIP Enhanced
- **Type**: Widely compatible deflate-based compression
- **Best for**: Web content, cross-platform compatibility
- **Features**:
  - Improved DEFLATE implementation
  - Enhanced hash chain matching
  - CRC32 integrity checking
  - RFC 1952 compatible headers

## Compression Levels

```cursed
sus COMPRESS_LEVEL_FASTEST normie = 1    # Maximum speed, basic compression
sus COMPRESS_LEVEL_FAST normie = 3       # Good speed, decent compression  
sus COMPRESS_LEVEL_BALANCED normie = 6   # Balanced speed/ratio (default)
sus COMPRESS_LEVEL_BEST normie = 9       # Better compression, slower
sus COMPRESS_LEVEL_MAXIMUM normie = 12   # Maximum compression, slowest
```

## API Reference

### Core Functions

#### `compress_data(input tea, algorithm normie, level normie) CompressionResult`
Compresses input data using the specified algorithm and compression level.

**Parameters:**
- `input`: String data to compress
- `algorithm`: Algorithm constant (ALGO_LZ4, ALGO_ZSTD, ALGO_GZIP)
- `level`: Compression level (1-12)

**Returns:** `CompressionResult` with compressed data and metrics

#### `decompress_data(compressed tea) CompressionResult`
Decompresses data by auto-detecting the compression algorithm.

**Parameters:**
- `compressed`: Compressed data string

**Returns:** `CompressionResult` with decompressed data and metrics

### Data Structures

#### `CompressionResult`
```cursed
squad CompressionResult {
    spill data tea                    # Compressed/decompressed data
    spill success lit                 # Operation success flag
    spill error_message tea           # Error description if failed
    spill metrics CompressionMetrics  # Performance metrics
}
```

#### `CompressionMetrics`
```cursed
squad CompressionMetrics {
    spill original_size normie        # Original data size in bytes
    spill compressed_size normie      # Compressed data size in bytes
    spill compression_ratio normie    # Compression ratio percentage
    spill compression_time normie     # Time taken to compress (ms)
    spill decompression_time normie   # Time taken to decompress (ms)
    spill algorithm normie            # Algorithm used
    spill level normie               # Compression level used
    spill block_size normie          # Block size for block-based algorithms
    spill entropy_score normie       # Data entropy analysis
}
```

## Usage Examples

### Basic Compression

```cursed
yeet "compressionz"

fr fr Compress text with ZSTD
sus input tea = "The quick brown fox jumps over the lazy dog"
sus result CompressionResult = compress_data(input, ALGO_ZSTD, COMPRESS_LEVEL_BALANCED)

vibes result.success {
    vibez.spill("Compressed size: " + result.metrics.compressed_size)
    vibez.spill("Compression ratio: " + result.metrics.compression_ratio + "%")
}
```

### Decompression

```cursed
fr fr Decompress data (auto-detects algorithm)
sus decompressed CompressionResult = decompress_data(result.data)

vibes decompressed.success {
    vibez.spill("Original data: " + decompressed.data)
    vibez.spill("Decompression time: " + decompressed.metrics.decompression_time + "ms")
}
```

### Algorithm Comparison

```cursed
sus test_data tea = "Large amount of test data for compression comparison..."

fr fr Test all algorithms
sus lz4_result CompressionResult = compress_data(test_data, ALGO_LZ4, COMPRESS_LEVEL_FAST)
sus zstd_result CompressionResult = compress_data(test_data, ALGO_ZSTD, COMPRESS_LEVEL_BALANCED)
sus gzip_result CompressionResult = compress_data(test_data, ALGO_GZIP, COMPRESS_LEVEL_BALANCED)

fr fr Compare results
vibez.spill("LZ4 ratio: " + lz4_result.metrics.compression_ratio + "%")
vibez.spill("ZSTD ratio: " + zstd_result.metrics.compression_ratio + "%")
vibez.spill("GZIP ratio: " + gzip_result.metrics.compression_ratio + "%")
```

### Performance Analysis

```cursed
fr fr Analyze compression performance
sus large_data tea = generate_test_data(10000)

sus perf_result CompressionResult = compress_data(large_data, ALGO_ZSTD, COMPRESS_LEVEL_BEST)

vibez.spill("Performance Metrics:")
vibez.spill("  Algorithm: " + get_algorithm_name(perf_result.metrics.algorithm))
vibez.spill("  Original size: " + perf_result.metrics.original_size + " bytes")
vibez.spill("  Compressed size: " + perf_result.metrics.compressed_size + " bytes")  
vibez.spill("  Compression ratio: " + perf_result.metrics.compression_ratio + "%")
vibez.spill("  Compression time: " + perf_result.metrics.compression_time + "ms")
vibez.spill("  Block size: " + perf_result.metrics.block_size + " bytes")
vibez.spill("  Entropy score: " + perf_result.metrics.entropy_score)
```

## Algorithm Details

### LZ4 Enhanced Implementation

The LZ4 implementation uses:
- **Hash Table**: 65536-entry hash table for fast match finding
- **Match Length**: Variable length matches (4-258 bytes)
- **Distance Encoding**: Optimized distance encoding for better compression
- **Hash Function**: 4-byte hash for improved collision distribution

**Best Use Cases:**
- Real-time data compression
- Network packet compression
- Temporary file compression
- Streaming applications

### ZSTD Advanced Implementation

The ZSTD implementation features:
- **Block Compression**: Adaptive block size based on compression level
- **FSE Encoding**: Finite State Entropy for literal encoding
- **Context Management**: Compression context with sequence buffers
- **Hash Chains**: Multiple hash table sizes for different block sizes

**Best Use Cases:**
- File archiving
- Database compression
- Log file compression
- General-purpose compression

### GZIP Enhanced Implementation

The GZIP implementation includes:
- **DEFLATE Engine**: Enhanced DEFLATE with improved hash chains
- **CRC32 Verification**: Full CRC32 checksum validation
- **Header Support**: Complete RFC 1952 header implementation
- **Multiple Levels**: 12 compression levels for different speed/ratio trade-offs

**Best Use Cases:**
- Web content compression
- Cross-platform file exchange
- HTTP compression
- Legacy system compatibility

## Performance Characteristics

| Algorithm | Speed | Compression Ratio | Memory Usage | Compatibility |
|-----------|-------|------------------|--------------|---------------|
| LZ4       | ⭐⭐⭐⭐⭐ | ⭐⭐⭐           | ⭐⭐⭐⭐      | ⭐⭐⭐⭐       |
| ZSTD      | ⭐⭐⭐⭐   | ⭐⭐⭐⭐⭐        | ⭐⭐⭐        | ⭐⭐⭐         |
| GZIP      | ⭐⭐⭐     | ⭐⭐⭐⭐          | ⭐⭐⭐⭐      | ⭐⭐⭐⭐⭐      |

## Memory Safety Features

- **Bounds Checking**: All array accesses are bounds-checked
- **Integer Overflow Protection**: Safe arithmetic operations
- **Buffer Management**: Proper buffer allocation and deallocation
- **Error Handling**: Comprehensive error detection and reporting

## Testing

Run the comprehensive test suite:

```bash
./zig-out/bin/cursed stdlib/compressionz/test_compressionz.💀
```

The test suite covers:
- ✅ All compression algorithms
- ✅ Multiple compression levels
- ✅ Round-trip compression/decompression
- ✅ Edge cases and error conditions
- ✅ Performance benchmarking
- ✅ Memory efficiency
- ✅ Data integrity verification

## Integration with Other Modules

The `compressionz` module integrates well with:
- `filez`: File compression/decompression
- `networkz`: Network data compression
- `jsonz`: JSON compression for APIs
- `archivez`: Archive creation with compression

## Future Enhancements

Planned improvements:
- **BROTLI**: Google's Brotli compression algorithm
- **LZMA**: High-ratio LZMA compression
- **Streaming API**: Stream-based compression for large files
- **Multi-threading**: Parallel compression for better performance
- **Custom Dictionaries**: Pre-trained dictionaries for specialized data

## Contributing

When contributing to the compression module:
1. Maintain pure CURSED implementation (no FFI)
2. Include comprehensive tests for new algorithms
3. Update performance benchmarks
4. Ensure memory safety compliance
5. Document algorithm-specific optimizations

## License

This module is part of the CURSED standard library and follows the same licensing terms as the main CURSED project.
