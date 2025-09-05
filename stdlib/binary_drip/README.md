# Binary Drip Module

The `binary_drip` module provides comprehensive binary data manipulation and endian-aware operations for the CURSED programming language. This module enables efficient handling of binary data formats, network protocols, and file format parsing.

## Features

- **Endian-aware operations**: Support for both little-endian and big-endian byte ordering
- **Multi-size integer support**: 8-bit, 16-bit, 32-bit, and 64-bit integer operations
- **Variable-length encoding**: LEB128 varint encoding/decoding
- **Pure CURSED implementation**: No external FFI dependencies
- **Comprehensive testing**: Full test coverage with testz v2.0 framework

## Functions

### Read Operations

#### `read_u8(data []byte, offset normie) byte`
Reads an unsigned 8-bit value from the byte array at the specified offset.

**Parameters:**
- `data`: Byte array to read from
- `offset`: Zero-based offset position

**Returns:** Unsigned 8-bit value (0-255)

**Example:**
```cursed
sus data [4]byte = [4]byte{0x10, 0x20, 0x30, 0x40}
sus value byte = read_u8(data, 1)  # Returns 0x20
```

#### `read_u16_le(data []byte, offset normie) mid`
Reads an unsigned 16-bit value in little-endian format.

**Parameters:**
- `data`: Byte array to read from
- `offset`: Zero-based offset position

**Returns:** Unsigned 16-bit value in host byte order

**Example:**
```cursed
sus data [4]byte = [4]byte{0x34, 0x12, 0x78, 0x56}
sus value mid = read_u16_le(data, 0)  # Returns 0x1234
```

#### `read_u16_be(data []byte, offset normie) mid`
Reads an unsigned 16-bit value in big-endian format.

**Parameters:**
- `data`: Byte array to read from
- `offset`: Zero-based offset position

**Returns:** Unsigned 16-bit value in host byte order

**Example:**
```cursed
sus data [4]byte = [4]byte{0x12, 0x34, 0x56, 0x78}
sus value mid = read_u16_be(data, 0)  # Returns 0x1234
```

#### `read_u32_le(data []byte, offset normie) normie`
Reads an unsigned 32-bit value in little-endian format.

#### `read_u32_be(data []byte, offset normie) normie`
Reads an unsigned 32-bit value in big-endian format.

#### `read_u64_le(data []byte, offset normie) thicc`
Reads an unsigned 64-bit value in little-endian format.

#### `read_u64_be(data []byte, offset normie) thicc`
Reads an unsigned 64-bit value in big-endian format.

### Write Operations

#### `write_u8(data []byte, offset normie, val byte) lit`
Writes an unsigned 8-bit value to the byte array at the specified offset.

**Parameters:**
- `data`: Byte array to write to
- `offset`: Zero-based offset position
- `val`: Value to write (0-255)

**Returns:** `based` if successful, `cap` if failed

**Example:**
```cursed
sus data [4]byte = [4]byte{0x00, 0x00, 0x00, 0x00}
sus success lit = write_u8(data, 0, 0xFF)
```

#### `write_u16_le(data []byte, offset normie, val mid) lit`
Writes an unsigned 16-bit value in little-endian format.

#### `write_u16_be(data []byte, offset normie, val mid) lit`
Writes an unsigned 16-bit value in big-endian format.

#### `write_u32_le(data []byte, offset normie, val normie) lit`
Writes an unsigned 32-bit value in little-endian format.

#### `write_u32_be(data []byte, offset normie, val normie) lit`
Writes an unsigned 32-bit value in big-endian format.

#### `write_u64_le(data []byte, offset normie, val thicc) lit`
Writes an unsigned 64-bit value in little-endian format.

#### `write_u64_be(data []byte, offset normie, val thicc) lit`
Writes an unsigned 64-bit value in big-endian format.

### Variable-Length Integer Operations

#### `varint_encode(value thicc) []byte`
Encodes a 64-bit integer using LEB128 (Little Endian Base 128) variable-length encoding.

**Parameters:**
- `value`: 64-bit integer to encode

**Returns:** Byte array containing the encoded value

**Example:**
```cursed
sus encoded []byte = varint_encode(128)  # Returns [0x80, 0x01]
```

#### `varint_decode(data []byte) thicc`
Decodes a LEB128 encoded variable-length integer.

**Parameters:**
- `data`: Byte array containing encoded value

**Returns:** Decoded 64-bit integer

**Example:**
```cursed
sus data [2]byte = [2]byte{0x80, 0x01}
sus value thicc = varint_decode(data)  # Returns 128
```

## Usage Examples

### Reading Network Protocol Data

```cursed
# Parse a simple network packet header
sus packet [12]byte = [12]byte{0x04, 0x00, 0x00, 0x00, 0x78, 0x56, 0x34, 0x12, 0x08, 0x00, 0x00, 0x00}

# Read protocol version (32-bit little-endian)
sus version normie = read_u32_le(packet, 0)

# Read sequence number (32-bit big-endian)
sus seq_num normie = read_u32_be(packet, 4)

# Read payload length (32-bit little-endian)
sus payload_len normie = read_u32_le(packet, 8)

vibez.spill("Version:", version)
vibez.spill("Sequence:", seq_num)
vibez.spill("Payload Length:", payload_len)
```

### Creating Binary Data

```cursed
# Create a binary structure
sus header [16]byte = [16]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}

# Write magic number (32-bit big-endian)
write_u32_be(header, 0, 0x12345678)

# Write version (16-bit little-endian)
write_u16_le(header, 4, 0x0001)

# Write flags (8-bit)
write_u8(header, 6, 0xFF)

# Write timestamp (64-bit little-endian)
write_u64_le(header, 8, 0x123456789ABCDEF0)
```

### Variable-Length Integer Encoding

```cursed
# Encode different size integers
sus small_encoded []byte = varint_encode(42)      # Single byte
sus medium_encoded []byte = varint_encode(16384)  # Three bytes
sus large_encoded []byte = varint_encode(0x123456789ABCDEF0)  # Ten bytes

# Decode back to integers
sus small_decoded thicc = varint_decode(small_encoded)
sus medium_decoded thicc = varint_decode(medium_encoded)
sus large_decoded thicc = varint_decode(large_encoded)
```

## Testing

The module includes comprehensive tests covering:

- All read and write operations
- Both little-endian and big-endian formats
- Boundary conditions and error handling
- Round-trip operations (write then read)
- Variable-length integer encoding/decoding
- Multiple operations on the same buffer
- Edge cases and error conditions

### Running Tests

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/binary_drip/test_binary_drip.💀

# Test native compilation mode
cargo run --bin cursed -- compile stdlib/binary_drip/test_binary_drip.💀
./test_binary_drip
```

## Common Use Cases

1. **Network Protocol Parsing**: Reading structured binary data from network packets
2. **File Format Handling**: Parsing binary file headers and data structures
3. **Serialization**: Converting structured data to/from binary format
4. **Embedded Systems**: Handling hardware registers and memory-mapped I/O
5. **Data Compression**: Variable-length encoding for efficient storage

## Performance Considerations

- Read operations are optimized for sequential access
- Write operations modify arrays in-place
- Variable-length encoding uses standard LEB128 format for compatibility
- All operations are bounds-checked for safety
- Pure CURSED implementation provides predictable performance

## Endianness Guide

**Little-Endian (LE)**: Least significant byte first
- Used by: x86/x64, ARM (typically), most modern CPUs
- Example: 0x12345678 stored as [0x78, 0x56, 0x34, 0x12]

**Big-Endian (BE)**: Most significant byte first
- Used by: Network protocols, some file formats, PowerPC
- Example: 0x12345678 stored as [0x12, 0x34, 0x56, 0x78]

## Error Handling

- Out-of-bounds reads return 0
- Out-of-bounds writes return `cap` (false)
- Invalid offsets are handled gracefully
- No exceptions or crashes on invalid input

## Dependencies

- `testz` module for testing framework
- Pure CURSED implementation (no external FFI)
- Compatible with both interpretation and compilation modes

## Version History

- v1.0.0: Initial implementation with full binary operation support
- Comprehensive test coverage with testz v2.0 framework
- Production-ready for enterprise deployment
