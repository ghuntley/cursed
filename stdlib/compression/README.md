# Compression Module

Advanced data compression and decompression algorithms for CURSED.

## Overview

The `compression` module provides multiple compression algorithms including Run-Length Encoding (RLE), LZ77, dictionary compression, and frequency-based compression. It includes utilities for compression analysis and automatic algorithm selection.

## Features

### Compression Algorithms

#### Run-Length Encoding (RLE)
- **Compression**: `rle_compress()` - Compress repeated characters
- **Decompression**: `rle_decompress()` - Decompress RLE data
- **Best For**: Data with many repeated characters

#### LZ77 Compression
- **Compression**: `lz77_compress()` - Dictionary-based compression
- **Decompression**: `lz77_decompress()` - Decompress LZ77 data
- **Best For**: Text with repeated patterns

#### Dictionary Compression
- **Compression**: `dictionary_compress()` - Custom dictionary compression
- **Decompression**: `dictionary_decompress()` - Decompress with dictionary
- **Dictionary Building**: `build_dictionary()` - Create compression dictionary
- **Best For**: Domain-specific data with known patterns

#### Frequency Compression
- **Compression**: `frequency_compress()` - Huffman-style compression
- **Frequency Analysis**: `build_frequency_map()` - Character frequency analysis
- **Best For**: General text compression

### Compression Analysis
- **Ratio Calculation**: `compression_ratio()` - Calculate compression efficiency
- **Savings Calculation**: `calculate_savings()` - Calculate space savings percentage
- **Auto Selection**: `auto_compress()`, `auto_decompress()` - Automatic algorithm selection

### Utility Functions
- **Match Finding**: `find_best_match()` - Find optimal compression matches
- **Dictionary Operations**: `find_longest_dictionary_match()`, `find_dictionary_index()`
- **String Utilities**: `int_to_string()`, `string_to_int()`, `char_to_digit()`

## Data Structures

### Match
```cursed
be_like Match squad {
    distance normie    // Distance to match
    length normie      // Length of match
    next_char tea      // Next character after match
}
```

## Usage Examples

```cursed
yeet "compression"

// RLE Compression
sus original tea = "aaabbbccc"
sus rle_compressed tea = rle_compress(original)
sus rle_decompressed tea = rle_decompress(rle_compressed)

// LZ77 Compression
sus text tea = "hello world hello world"
sus lz77_compressed tea = lz77_compress(text)
sus lz77_decompressed tea = lz77_decompress(lz77_compressed)

// Dictionary Compression
sus data tea = "the quick brown fox jumps over the lazy dog"
sus dictionary [tea] = build_dictionary(data)
sus dict_compressed tea = dictionary_compress(data)
sus dict_decompressed tea = dictionary_decompress(dict_compressed, dictionary)

// Frequency Compression
sus freq_map map[tea]normie = build_frequency_map("hello world")
sus freq_compressed tea = frequency_compress("hello world")

// Compression Analysis
sus ratio meal = compression_ratio(original, rle_compressed)
sus savings meal = calculate_savings(original, rle_compressed)

// Auto Compression
sus auto_compressed tea = auto_compress(data)
sus auto_decompressed tea = auto_decompress(auto_compressed)
```

## Compression Algorithms

### RLE (Run-Length Encoding)
- **Format**: `3a2b4c` (3 a's, 2 b's, 4 c's)
- **Efficiency**: Excellent for data with long runs of repeated characters
- **Use Cases**: Simple graphics, basic text patterns

### LZ77 (Lempel-Ziv 77)
- **Format**: `hello[5,1]world` (copy 1 character from 5 positions back)
- **Efficiency**: Good for text with repeated patterns
- **Use Cases**: General text compression, similar to DEFLATE

### Dictionary Compression
- **Format**: `hello #1# world` (reference to dictionary entry #1)
- **Efficiency**: Excellent for domain-specific data
- **Use Cases**: Log files, structured data with known patterns

### Frequency Compression
- **Format**: Binary codes based on character frequency
- **Efficiency**: Good for general text compression
- **Use Cases**: Similar to Huffman coding

## Performance

The compression module is optimized for performance:
- Linear time complexity for most algorithms
- Efficient dictionary lookup and matching
- Minimal memory overhead during compression
- Fast decompression with streaming support

## Auto-Selection Logic

The `auto_compress()` function automatically selects the best compression algorithm:
1. Tests all available algorithms
2. Compares compression ratios
3. Returns the best result with algorithm prefix
4. Supports transparent decompression

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/compression/test_compression.csd
```

## Status

✅ **Production Ready**: All major algorithms implemented and tested
✅ **Pure CURSED**: No external compression library dependencies
✅ **Cross-Platform**: Consistent compression results across platforms
✅ **Extensible**: Easy to add new compression algorithms
✅ **Fully Tested**: Comprehensive test coverage including edge cases
