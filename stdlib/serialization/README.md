# CURSED Serialization Module

Pure CURSED implementation for binary data serialization and deserialization.

## Overview

The serialization module provides comprehensive binary data handling capabilities implemented entirely in CURSED. It supports serialization of basic types, arrays, structured data, and includes features like checksums, versioning, and compression integration.

## Features

### Basic Type Serialization
- 32-bit integers (`normie`)
- 64-bit integers (`thicc`)
- Floating-point numbers (`meal`)
- Strings with length prefixes
- Boolean values
- Arrays of basic types

### Structured Serialization
- Context-based serialization
- Message format serialization
- Object serialization
- Variable-length integer encoding

### Data Integrity
- Checksum calculation and validation
- Version information embedding
- Corruption detection

### Advanced Features
- Compression integration
- Protocol buffer style encoding
- Binary format optimization

## Usage Examples

```cursed
yeet "serialization"

// Basic type serialization
sus int_data tea = serialization.serialize_int(42)
sus int_value normie = serialization.deserialize_int(int_data, 0)
vibez.spill("Integer: " + tea(int_value))

sus str_data tea = serialization.serialize_string("hello")
sus str_value tea = serialization.deserialize_string(str_data, 0)
vibez.spill("String: " + str_value)

sus bool_data tea = serialization.serialize_bool(based)
sus bool_value lit = serialization.deserialize_bool(bool_data, 0)
vibez.spill("Boolean: " + tea(bool_value))

// Array serialization
sus numbers [normie] = [1, 2, 3, 4, 5]
sus array_data tea = serialization.serialize_array_int(numbers)
sus restored_numbers [normie] = serialization.deserialize_array_int(array_data, 0)

sus strings [tea] = ["hello", "world", "test"]
sus string_array_data tea = serialization.serialize_array_string(strings)
sus restored_strings [tea] = serialization.deserialize_array_string(string_array_data, 0)

// Context-based serialization
sus context serialization.SerializationContext = serialization.create_serialization_context()
context = serialization.write_int(context, 42)
context = serialization.write_string(context, "hello")
context = serialization.write_bool(context, based)

// Read back
sus read_context serialization.SerializationContext = serialization.SerializationContext{
    data: context.data,
    offset: 0,
    error: ""
}
sus int_val normie = serialization.read_int(read_context)
sus str_val tea = serialization.read_string(read_context)
sus bool_val lit = serialization.read_bool(read_context)

// Message serialization
sus message serialization.Message = serialization.Message{
    field_id: 1,
    field_type: 2,
    data: "message content"
}
sus msg_data tea = serialization.serialize_message(message)
sus restored_msg serialization.Message = serialization.deserialize_message(msg_data, 0)

// Checksum validation
sus test_data tea = "important data"
sus checksum normie = serialization.calculate_checksum(test_data)
sus with_checksum tea = serialization.serialize_with_checksum(test_data)
sus validated_data tea = serialization.deserialize_with_checksum(with_checksum)

// Versioned serialization
sus versioned tea = serialization.serialize_versioned("data", 1)
sus unversioned tea = serialization.deserialize_versioned(versioned)
```

## API Reference

### Basic Type Serialization

#### `serialize_int(value normie) tea`
Serialize 32-bit integer to 4-byte binary string.

#### `deserialize_int(data tea, offset normie) normie`
Deserialize 32-bit integer from binary data.

#### `serialize_long(value thicc) tea`
Serialize 64-bit integer to 8-byte binary string.

#### `deserialize_long(data tea, offset normie) thicc`
Deserialize 64-bit integer from binary data.

#### `serialize_float(value meal) tea`
Serialize float to 4-byte IEEE 754 format.

#### `deserialize_float(data tea, offset normie) meal`
Deserialize float from IEEE 754 format.

#### `serialize_string(value tea) tea`
Serialize string with 4-byte length prefix.

#### `deserialize_string(data tea, offset normie) tea`
Deserialize string from length-prefixed format.

#### `serialize_bool(value lit) tea`
Serialize boolean to single byte (0 or 1).

#### `deserialize_bool(data tea, offset normie) lit`
Deserialize boolean from single byte.

### Array Serialization

#### `serialize_array_int(values [normie]) tea`
Serialize array of integers with length prefix.

#### `deserialize_array_int(data tea, offset normie) [normie]`
Deserialize array of integers.

#### `serialize_array_string(values [tea]) tea`
Serialize array of strings with length prefix.

#### `deserialize_array_string(data tea, offset normie) [tea]`
Deserialize array of strings.

### Context-Based Serialization

#### `create_serialization_context() SerializationContext`
Create new serialization context.

#### `write_int(context SerializationContext, value normie) SerializationContext`
Write integer to context.

#### `write_string(context SerializationContext, value tea) SerializationContext`
Write string to context.

#### `write_bool(context SerializationContext, value lit) SerializationContext`
Write boolean to context.

#### `write_float(context SerializationContext, value meal) SerializationContext`
Write float to context.

#### `write_long(context SerializationContext, value thicc) SerializationContext`
Write long integer to context.

#### `read_int(context SerializationContext) normie`
Read integer from context.

#### `read_string(context SerializationContext) tea`
Read string from context.

#### `read_bool(context SerializationContext) lit`
Read boolean from context.

#### `read_float(context SerializationContext) meal`
Read float from context.

#### `read_long(context SerializationContext) thicc`
Read long integer from context.

### Variable-Length Encoding

#### `serialize_varint(value normie) tea`
Serialize integer using variable-length encoding.

#### `deserialize_varint(data tea, offset normie) normie`
Deserialize variable-length integer.

#### `varint_size(value normie) normie`
Calculate size of varint encoding.

### Message Serialization

#### `serialize_message(message Message) tea`
Serialize message with field metadata.

#### `deserialize_message(data tea, offset normie) Message`
Deserialize message from binary data.

### Object Serialization

#### `serialize_object(fields map[tea]tea) tea`
Serialize object to JSON-like format.

#### `deserialize_object(data tea) map[tea]tea`
Deserialize object from serialized format.

### Checksum Functions

#### `calculate_checksum(data tea) normie`
Calculate 16-bit checksum for data validation.

#### `validate_checksum(data tea, expected_checksum normie) lit`
Validate data against expected checksum.

#### `serialize_with_checksum(data tea) tea`
Serialize data with embedded checksum.

#### `deserialize_with_checksum(data tea) tea`
Deserialize and validate checksum.

### Versioning

#### `serialize_versioned(data tea, version normie) tea`
Serialize data with version information.

#### `deserialize_versioned(data tea) tea`
Deserialize versioned data.

### Compression Integration

#### `serialize_compressed(data tea) tea`
Serialize data with compression.

#### `deserialize_compressed(data tea) tea`
Deserialize compressed data.

## Data Structures

### SerializationContext
```cursed
be_like SerializationContext squad {
    data tea       // Serialized binary data
    offset normie  // Current read/write position
    error tea      // Error message if any
}
```

### Message
```cursed
be_like Message squad {
    field_id normie    // Field identifier
    field_type normie  // Field type code
    data tea           // Serialized field data
}
```

## Binary Format Specifications

### Integer Format (4 bytes)
```
Byte 0: Bits 0-7 of value
Byte 1: Bits 8-15 of value
Byte 2: Bits 16-23 of value
Byte 3: Bits 24-31 of value
```

### Long Integer Format (8 bytes)
```
Bytes 0-3: Lower 32 bits
Bytes 4-7: Upper 32 bits
```

### String Format
```
Bytes 0-3: Length (32-bit integer)
Bytes 4+: UTF-8 string data
```

### Array Format
```
Bytes 0-3: Element count (32-bit integer)
Bytes 4+: Serialized elements
```

### Variable Integer Format
```
Each byte: [continuation_bit][7_data_bits]
continuation_bit = 1: more bytes follow
continuation_bit = 0: last byte
```

### Message Format
```
Bytes 0+: Field ID (varint)
Bytes N+: Field Type (varint)
Bytes M+: String length (varint)
Bytes P+: String data
```

## Performance Characteristics

| Operation | Time Complexity | Space Overhead |
|-----------|-----------------|----------------|
| Integer | O(1) | 4 bytes |
| Long | O(1) | 8 bytes |
| String | O(n) | 4 + n bytes |
| Array | O(n) | 4 + element_size × n |
| Varint | O(1) | 1-5 bytes |
| Checksum | O(n) | 2 bytes |

## Error Handling

The module provides basic error detection through:
- Length validation for arrays and strings
- Offset bounds checking
- Checksum validation
- Version compatibility checking

## Dependencies

- `string` module for string manipulation

## Testing

Run the test suite:
```bash
cargo run --bin cursed stdlib/serialization/test_serialization.💀
```

The test suite includes:
- Basic type serialization tests
- Array serialization tests
- Context-based serialization tests
- Message format tests
- Checksum validation tests
- Versioning tests
- Utility function tests

## Implementation Notes

This is a pure CURSED implementation designed for:
- Cross-platform compatibility
- Deterministic binary formats
- Educational clarity
- No external dependencies

### Design Principles
- Little-endian byte ordering
- Length-prefixed variable data
- Simple checksum algorithm
- Straightforward versioning

### Limitations
- Simplified float representation
- Basic checksum (not cryptographically secure)
- Limited to 32-bit field IDs
- No schema validation

### Extensions
To extend this module:
- Add complex data structure support
- Implement schema validation
- Add compression algorithms
- Support for custom serializers
- Streaming serialization support

The module provides a solid foundation for binary data handling and can be extended based on specific application requirements.
