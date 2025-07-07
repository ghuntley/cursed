# CURSED Compression Module

Pure CURSED implementation for data compression and decompression algorithms.

## Overview

The compression module provides various compression algorithms implemented entirely in CURSED without external dependencies. It includes run-length encoding, LZ77-style compression, dictionary compression, and utility functions for compression analysis.

## Features

### Compression Algorithms
- Run-Length Encoding (RLE)
- LZ77-style compression
- Dictionary-based compression
- Frequency-based compression
- Variable-length integer encoding (varint)

### Utilities
- Auto-detection of best compression method
- Compression ratio calculation
- Compression savings analysis
- Checksum validation

### Message Format
- Protocol buffer style serialization
- Message field encoding
- Binary data handling

## Usage Examples

```cursed
yeet "compression"

// Run-length encoding
sus data tea = "aaabbbccc"
sus compressed tea = compression.rle_compress(data)
sus decompressed tea = compression.rle_decompress(compressed)
vibez.spill("Original: " + data)
vibez.spill("Decompressed: " + decompressed)

// LZ77 compression
sus text tea = "hello world hello world"
sus lz77_compressed tea = compression.lz77_compress(text)
sus lz77_decompressed tea = compression.lz77_decompress(lz77_compressed)
vibez.spill("LZ77 result: " + lz77_decompressed)

// Auto compression (picks best method)
sus auto_compressed tea = compression.auto_compress("test data test data")
sus auto_decompressed tea = compression.auto_decompress(auto_compressed)
vibez.spill("Auto result: " + auto_decompressed)

// Calculate compression ratio
sus ratio meal = compression.compression_ratio("original", "compressed")
sus savings meal = compression.calculate_savings("original", "compressed")
vibez.spill("Compression ratio: " + tea(ratio))
vibez.spill("Space savings: " + tea(savings) + "%")

// Dictionary compression
sus dict_data tea = "hello world hello world"
sus dict_compressed tea = compression.dictionary_compress(dict_data)
sus dictionary [tea] = compression.build_dictionary(dict_data)
sus dict_decompressed tea = compression.dictionary_decompress(dict_compressed, dictionary)

// Variable integer encoding
sus varint tea = compression.serialize_varint(16384)
sus decoded normie = compression.deserialize_varint(varint, 0)
vibez.spill("Varint decoded: " + tea(decoded))
```

## API Reference

### Run-Length Encoding

#### `rle_compress(data tea) tea`
Compress data using run-length encoding.

#### `rle_decompress(compressed tea) tea`
Decompress RLE-encoded data.

### LZ77 Compression

#### `lz77_compress(data tea) tea`
Compress data using LZ77-style algorithm.

#### `lz77_decompress(compressed tea) tea`
Decompress LZ77-encoded data.

### Dictionary Compression

#### `dictionary_compress(data tea) tea`
Compress data using dictionary-based method.

#### `dictionary_decompress(compressed tea, dictionary [tea]) tea`
Decompress dictionary-encoded data.

#### `build_dictionary(data tea) [tea]`
Build compression dictionary from data.

#### `find_longest_dictionary_match(data tea, pos normie, dictionary [tea]) tea`
Find longest matching phrase in dictionary.

#### `find_dictionary_index(dictionary [tea], phrase tea) normie`
Find index of phrase in dictionary.

#### `contains_phrase(dictionary [tea], phrase tea) lit`
Check if dictionary contains phrase.

### Frequency Compression

#### `frequency_compress(data tea) tea`
Compress data based on character frequency.

#### `build_frequency_map(data tea) map[tea]normie`
Build character frequency map.

#### `build_simple_encoding(freq_map map[tea]normie) map[tea]tea`
Build encoding map based on frequencies.

### Variable Integer Encoding

#### `serialize_varint(value normie) tea`
Encode integer using variable-length encoding.

#### `deserialize_varint(data tea, offset normie) normie`
Decode variable-length integer.

#### `varint_size(value normie) normie`
Calculate size of varint encoding.

### Auto Compression

#### `auto_compress(data tea) tea`
Automatically select and apply best compression method.

#### `auto_decompress(compressed tea) tea`
Automatically detect and decompress data.

### Analysis Functions

#### `compression_ratio(original tea, compressed tea) meal`
Calculate compression ratio (compressed/original).

#### `calculate_savings(original tea, compressed tea) meal`
Calculate space savings percentage.

### Message Serialization

#### `serialize_message(message Message) tea`
Serialize message with field information.

#### `deserialize_message(data tea, offset normie) Message`
Deserialize message from binary data.

### Checksum Functions

#### `calculate_checksum(data tea) normie`
Calculate simple checksum for data validation.

#### `validate_checksum(data tea, expected_checksum normie) lit`
Validate data against expected checksum.

#### `serialize_with_checksum(data tea) tea`
Serialize data with embedded checksum.

#### `deserialize_with_checksum(data tea) tea`
Deserialize and validate checksum.

## Data Structures

### Match
```cursed
be_like Match squad {
    distance normie    // Distance to match
    length normie      // Length of match
    next_char tea      // Next character
}
```

### Message
```cursed
be_like Message squad {
    field_id normie    // Field identifier
    field_type normie  // Field type
    data tea           // Message data
}
```

## Compression Algorithms

### Run-Length Encoding (RLE)
- Best for data with repeated characters
- Format: `<count><character><count><character>...`
- Example: `"aaabbb"` → `"3a3b"`

### LZ77 Compression
- Dictionary-based compression with sliding window
- Uses back-references to previous data
- Format: Mix of literals and `[distance,length]` tokens

### Dictionary Compression
- Builds dictionary of common phrases
- Replaces phrases with dictionary indices
- Format: Mix of literals and `#<index>#` references

### Frequency Compression
- Assigns shorter codes to frequent characters
- Simplified Huffman-style encoding
- Builds frequency map first

### Variable Integer Encoding
- Efficient encoding for small integers
- Uses continuation bits for larger values
- Saves space for small numbers

## Performance Characteristics

| Algorithm | Best For | Time Complexity | Space Efficiency |
|-----------|----------|-----------------|------------------|
| RLE | Repeated chars | O(n) | High for runs |
| LZ77 | General text | O(n²) | Good overall |
| Dictionary | Repeated phrases | O(n×d) | Good for patterns |
| Frequency | Mixed content | O(n) | Variable |
| Varint | Small integers | O(1) | Excellent |

## Auto-Compression Logic

The `auto_compress` function:
1. Tries all compression methods
2. Compares compressed sizes
3. Returns best result with format prefix
4. Prefixes: `"RLE:"`, `"LZ77:"`, `"DICT:"`

## Dependencies

- `string` module for string manipulation

## Testing

Run the test suite:
```bash
cargo run --bin cursed stdlib/compression/test_compression.csd
```

The test suite includes:
- RLE compression/decompression tests
- LZ77 algorithm tests
- Dictionary compression tests
- Varint encoding tests
- Utility function tests
- Auto-compression tests
- Compression ratio tests

## Implementation Notes

This is a pure CURSED implementation focusing on:
- Educational value and code clarity
- No external dependencies
- Comprehensive test coverage
- Multiple algorithm approaches

### Limitations
- Algorithms are simplified for clarity
- Performance optimized for readability
- Not suitable for production compression needs
- Dictionary size limited for memory efficiency

### Extensions
To extend this module:
- Add more sophisticated Huffman encoding
- Implement LZW compression
- Add streaming compression support
- Optimize algorithms for speed

The module serves as a foundation for understanding compression algorithms and can be extended based on specific needs.
