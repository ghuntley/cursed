# ByteFit - Pure CURSED Byte Manipulation Library

ByteFit is a comprehensive byte manipulation library implemented in pure CURSED without any FFI dependencies. It provides a complete suite of functions for byte operations, bit manipulation, array processing, and encoding/decoding utilities.

## Features

### 🔧 Byte Manipulation Functions
- **Bit Operations**: Set, clear, toggle, and test individual bits
- **Bit Counting**: Count ones and zeros in bytes
- **Bit Transformations**: Reverse bits, rotate left/right, swap nibbles

### 📊 Byte Array Operations
- **Array Creation**: Create and fill byte arrays
- **Array Manipulation**: Copy, reverse, find, and count operations
- **Bitwise Operations**: XOR, AND, OR operations on byte arrays
- **Array Utilities**: Sum calculations and pattern matching

### 🎯 Byte Encoding/Decoding
- **Hexadecimal**: Convert bytes to/from hex strings
- **Binary**: Convert bytes to/from binary strings
- **Array Encoding**: Batch encode/decode byte arrays

### ⚡ Bit Manipulation Utilities
- **Bit Patterns**: Extract and set bit patterns within bytes
- **Checksums**: Calculate parity, checksum, and CRC8
- **Character Utils**: ASCII, printable, digit, alpha checks
- **Case Conversion**: Upper/lowercase conversion for byte values

## API Reference

### Bit Operations
```cursed
byte_set_bit(b byte, pos normie) byte       # Set bit at position
byte_clear_bit(b byte, pos normie) byte     # Clear bit at position
byte_toggle_bit(b byte, pos normie) byte    # Toggle bit at position
byte_test_bit(b byte, pos normie) lit       # Test if bit is set
```

### Bit Counting
```cursed
byte_count_ones(b byte) normie              # Count set bits
byte_count_zeros(b byte) normie             # Count unset bits
```

### Bit Transformations
```cursed
byte_reverse_bits(b byte) byte              # Reverse bit order
byte_rotate_left(b byte, pos normie) byte   # Rotate bits left
byte_rotate_right(b byte, pos normie) byte  # Rotate bits right
byte_swap_nibbles(b byte) byte              # Swap high/low nibbles
```

### Array Operations
```cursed
byte_array_create(size normie) [byte]       # Create byte array
byte_array_fill(arr [byte], value byte) [byte]  # Fill with value
byte_array_reverse(arr [byte]) [byte]       # Reverse array
byte_array_find(arr [byte], value byte) normie  # Find value index
byte_array_count(arr [byte], value byte) normie # Count occurrences
byte_array_sum(arr [byte]) normie           # Sum all values
```

### Bitwise Array Operations
```cursed
byte_array_xor(arr1 [byte], arr2 [byte]) [byte]  # XOR arrays
byte_array_and(arr1 [byte], arr2 [byte]) [byte]  # AND arrays
byte_array_or(arr1 [byte], arr2 [byte]) [byte]   # OR arrays
```

### Encoding/Decoding
```cursed
byte_to_hex(b byte) tea                     # Byte to hex string
hex_to_byte(hex tea) byte                   # Hex string to byte
byte_array_to_hex(arr [byte]) tea           # Array to hex string
hex_to_byte_array(hex tea) [byte]           # Hex string to array
byte_to_binary(b byte) tea                  # Byte to binary string
binary_to_byte(binary tea) byte             # Binary string to byte
```

### Bit Pattern Manipulation
```cursed
get_bit_pattern(b byte, start normie, length normie) byte    # Extract bits
set_bit_pattern(b byte, start normie, length normie, value byte) byte  # Set bits
```

### Checksums and Validation
```cursed
byte_parity(b byte) lit                     # Check even parity
byte_checksum(arr [byte]) byte              # Calculate checksum
byte_crc8(arr [byte]) byte                  # Calculate CRC8
```

### Character Utilities
```cursed
byte_is_ascii(b byte) lit                   # Check if ASCII
byte_is_printable(b byte) lit               # Check if printable
byte_is_digit(b byte) lit                   # Check if digit
byte_is_alpha(b byte) lit                   # Check if alphabetic
byte_is_uppercase(b byte) lit               # Check if uppercase
byte_is_lowercase(b byte) lit               # Check if lowercase
byte_to_uppercase(b byte) byte              # Convert to uppercase
byte_to_lowercase(b byte) byte              # Convert to lowercase
```

## Usage Examples

### Basic Bit Operations
```cursed
yeet "bytefit"

# Set bit 3 in byte
sus b byte = 0b00000000
sus result byte = byte_set_bit(b, 3)
# result = 0b00001000

# Count set bits
sus count normie = byte_count_ones(0b11010101)
# count = 5

# Reverse bits
sus reversed byte = byte_reverse_bits(0b10110001)
# reversed = 0b10001101
```

### Array Operations
```cursed
yeet "bytefit"

# Create and fill array
sus arr [byte] = byte_array_create(5)
sus filled [byte] = byte_array_fill(arr, 0xFF)

# XOR two arrays
sus arr1 [byte] = [0b11110000, 0b00001111]
sus arr2 [byte] = [0b10101010, 0b01010101]
sus xor_result [byte] = byte_array_xor(arr1, arr2)
```

### Encoding/Decoding
```cursed
yeet "bytefit"

# Convert byte to hex
sus hex tea = byte_to_hex(0xFF)
# hex = "FF"

# Convert hex array to string
sus data [byte] = [0x12, 0x34, 0xAB]
sus hex_string tea = byte_array_to_hex(data)
# hex_string = "1234AB"

# Binary conversion
sus binary tea = byte_to_binary(0b11010101)
# binary = "11010101"
```

### Bit Pattern Manipulation
```cursed
yeet "bytefit"

# Extract 3 bits starting at position 2
sus pattern byte = get_bit_pattern(0b11010110, 2, 3)
# pattern = 0b101

# Set 3 bits starting at position 2
sus result byte = set_bit_pattern(0b11111111, 2, 3, 0b010)
# result = 0b11101011
```

### Checksums
```cursed
yeet "bytefit"

# Calculate checksum
sus data [byte] = [0x12, 0x34, 0x56, 0x78]
sus checksum byte = byte_checksum(data)

# Check parity
sus is_even lit = byte_parity(0b11000011)
# is_even = based (true)
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/bytefit/test_bytefit.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/bytefit/test_bytefit.💀
./test_bytefit
```

## Test Coverage

The ByteFit module includes 50+ comprehensive tests covering:

- ✅ Bit manipulation operations (set, clear, toggle, test)
- ✅ Bit counting and transformations
- ✅ Byte array creation and manipulation
- ✅ Bitwise array operations (XOR, AND, OR)
- ✅ Hexadecimal and binary encoding/decoding
- ✅ Bit pattern extraction and manipulation
- ✅ Checksum calculations (parity, checksum, CRC8)
- ✅ Character utility functions
- ✅ Case conversion operations
- ✅ Edge cases and error conditions

## Design Principles

### Pure CURSED Implementation
- **No FFI Dependencies**: Implemented entirely in CURSED without external libraries
- **Self-Contained**: All functions implemented using only CURSED language features
- **Portable**: Works consistently across all platforms and execution modes

### Performance Optimized
- **Efficient Algorithms**: Optimized bit manipulation using bitwise operations
- **Minimal Allocations**: Efficient memory usage for array operations
- **Fast Execution**: Optimized for both interpretation and compilation modes

### Type Safety
- **Strong Typing**: All functions use appropriate CURSED types
- **Bounds Checking**: Safe array access with proper boundary validation
- **Error Handling**: Graceful handling of edge cases and invalid inputs

### Comprehensive Coverage
- **Complete API**: Full suite of byte manipulation functions
- **Consistent Interface**: Uniform function naming and parameter conventions
- **Extensive Testing**: Thorough test coverage for all functionality

## Implementation Notes

### Bit Manipulation Techniques
- Uses bitwise operators (`&`, `|`, `^`, `<<`, `>>`) for efficient operations
- Implements bit counting using Brian Kernighan's algorithm
- Provides both left and right rotation with proper wraparound

### Array Operations
- Efficient array copying and manipulation
- Supports variable-length arrays with proper bounds checking
- Implements bitwise operations element-wise for array processing

### Encoding Algorithms
- Hexadecimal encoding uses lookup table approach
- Binary encoding provides full 8-bit representation
- CRC8 implementation uses polynomial 0x07 (standard CRC-8)

### Character Processing
- ASCII range checking (0-127)
- Printable character range (32-126)
- Standard case conversion (A-Z ↔ a-z)

## Future Enhancements

Potential areas for expansion:
- Additional checksum algorithms (CRC16, CRC32)
- More encoding formats (Base64, Base32)
- Advanced bit manipulation (population count, leading zeros)
- Cryptographic byte operations
- Performance optimizations for large arrays

## License

This module is part of the CURSED language standard library and follows the same licensing terms as the main CURSED project.
