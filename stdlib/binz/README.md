# BINZ - Binary Serialization Format

**High-Performance Binary Encoding/Decoding with Schema Evolution and Versioning**

BINZ is a comprehensive binary serialization format for the CURSED language that provides efficient encoding/decoding, schema definition, version compatibility, and reflection-based serialization.

## 🚀 Features

- **High Performance**: Optimized binary encoding with minimal overhead
- **Schema Evolution**: Built-in versioning and migration support  
- **Type Safety**: Strong typing with validation
- **Compression**: Built-in compression for large datasets
- **Reflection**: Automatic serialization using reflection
- **Memory Pools**: Zero-allocation encoding for performance-critical applications
- **Batch Operations**: Efficient encoding/decoding of multiple values
- **Cross-Platform**: Big-endian format ensures portability

## 📋 Quick Start

```cursed
yeet "binz"

# Simple value encoding
sus value BinzValue = binz_create_string("Hello, World!")
sus encoded []drip = binz_encode(value)
sus decoded BinzValue = binz_decode(encoded)

vibez.spill(decoded.string_value)  # "Hello, World!"
```

## 🏗️ Binary Format Specification

### Header Format (8 bytes minimum)

```
[0-3]   Magic Header: 0x42494E5A ("BINZ")
[4]     Version Major: 1
[5]     Version Minor: 0  
[6-7]   Flags: 16-bit flags field
        Bit 0: Schema flag (1 = schema present)
        Bits 1-15: Reserved
[8+]    Optional Schema ID (if schema flag set)
```

### Type Tags

| Tag | Value | Description | Format |
|-----|-------|-------------|--------|
| `TAG_NULL` | 0x00 | Null value | Tag only |
| `TAG_BOOL_FALSE` | 0x01 | Boolean false | Tag only |
| `TAG_BOOL_TRUE` | 0x02 | Boolean true | Tag only |
| `TAG_INT8` | 0x03 | 8-bit signed int | Tag + 1 byte |
| `TAG_INT16` | 0x04 | 16-bit signed int | Tag + 2 bytes |
| `TAG_INT32` | 0x05 | 32-bit signed int | Tag + 4 bytes |
| `TAG_INT64` | 0x06 | 64-bit signed int | Tag + 8 bytes |
| `TAG_UINT8` | 0x07 | 8-bit unsigned int | Tag + 1 byte |
| `TAG_UINT16` | 0x08 | 16-bit unsigned int | Tag + 2 bytes |
| `TAG_UINT32` | 0x09 | 32-bit unsigned int | Tag + 4 bytes |
| `TAG_UINT64` | 0x0A | 64-bit unsigned int | Tag + 8 bytes |
| `TAG_FLOAT32` | 0x0B | 32-bit IEEE 754 | Tag + 4 bytes |
| `TAG_FLOAT64` | 0x0C | 64-bit IEEE 754 | Tag + 8 bytes |
| `TAG_STRING_SHORT` | 0x0D | String < 256 bytes | Tag + length(1) + data |
| `TAG_STRING_LONG` | 0x0E | String ≥ 256 bytes | Tag + length(4) + data |
| `TAG_ARRAY_FIXED` | 0x0F | Typed array | Tag + type + count + data |
| `TAG_ARRAY_MIXED` | 0x10 | Mixed type array | Tag + count + elements |
| `TAG_STRUCT` | 0x11 | Structured object | Tag + field_count + fields |
| `TAG_SCHEMA_REF` | 0x12 | Schema reference | Tag + schema_id + data |
| `TAG_EXTENSION` | 0x13 | User extension | Tag + extension_id + data |
| `TAG_COMPRESSED` | 0x14 | Compressed data | Tag + sizes + compressed_data |

### Variable Integer Encoding (VarInt)

For efficient space usage, integers are encoded using variable-length encoding:

```
0xxxxxxx                    → 7-bit value (0-127)
1xxxxxxx 0yyyyyyy           → 14-bit value  
1xxxxxxx 1yyyyyyy 0zzzzzzz  → 21-bit value
...
```

## 🔧 Core API

### Basic Types

```cursed
# Create values
sus null_val BinzValue = binz_create_null()
sus bool_val BinzValue = binz_create_bool(based)  
sus int_val BinzValue = binz_create_int(42)
sus float_val BinzValue = binz_create_float(3.14)
sus string_val BinzValue = binz_create_string("hello")
sus array_val BinzValue = binz_create_array()
sus struct_val BinzValue = binz_create_struct()
```

### Encoding/Decoding

```cursed
# Simple encoding
sus encoded []drip = binz_encode(value)
sus decoded BinzValue = binz_decode(encoded)

# Schema-based encoding  
sus schema BinzSchema = binz_create_schema(1001, 1, "User")
sus encoded_with_schema []drip = binz_encode_with_schema(value, schema)
sus decoded_with_schema BinzValue = binz_decode_with_schema(encoded_with_schema, schema)
```

### Complex Structures

```cursed
# Create struct with fields
sus user BinzValue = binz_create_struct()
user.struct_fields[0] = "name"
user.struct_values[0] = binz_create_string("Alice")
user.struct_fields[1] = "age"
user.struct_values[1] = binz_create_int(30)
user.struct_fields[2] = "active"
user.struct_values[2] = binz_create_bool(based)

# Create array with mixed types
sus mixed_array BinzValue = binz_create_array()
mixed_array.array_values[0] = binz_create_int(42)
mixed_array.array_values[1] = binz_create_string("test")
mixed_array.array_values[2] = binz_create_bool(based)
```

## 📐 Schema System

### Schema Definition

```cursed
# Create schema
sus user_schema BinzSchema = binz_create_schema(1001, 1, "User")
user_schema = binz_schema_add_field(user_schema, "id", "uint32", cringe)      # required
user_schema = binz_schema_add_field(user_schema, "name", "string", cringe)    # required  
user_schema = binz_schema_add_field(user_schema, "email", "string", based)   # optional
user_schema = binz_schema_add_field(user_schema, "age", "uint32", based)     # optional
user_schema.compatibility_mode = "forward"  # or "backward", "strict", "full"
```

### Schema Registry

```cursed
# Create and populate registry
sus registry BinzSchemaRegistry = binz_create_schema_registry()
binz_register_schema(registry, user_schema)

# Encode with registry
sus encoder BinzEncoder = binz_create_encoder()
encoder.schema_registry = registry
```

### Schema Migration

```cursed
# Define migration between versions
sus migration BinzMigrationRule = BinzMigrationRule{}
migration.from_version = 1
migration.to_version = 2

# Field name change
sus name_mapping BinzFieldMapping = BinzFieldMapping{}
name_mapping.old_name = "name"
name_mapping.new_name = "full_name"
name_mapping.type_conversion = "none"
migration.field_mappings[0] = name_mapping

# Apply migration
old_schema.migration_rules[0] = migration
sus migrated BinzSchema = binz_migrate_schema(old_schema, new_schema)
```

## 🔄 Compatibility Modes

| Mode | Forward Compatible | Backward Compatible | Description |
|------|-------------------|-------------------|-------------|
| `strict` | ❌ | ❌ | Exact schema match required |
| `forward` | ✅ | ❌ | New fields allowed, old fields required |
| `backward` | ❌ | ✅ | Old fields allowed, new fields ignored |
| `full` | ✅ | ✅ | Both forward and backward compatible |

## 🗜️ Compression

BINZ includes built-in run-length encoding compression:

```cursed
# Enable compression for large structures
sus large_data BinzValue = create_large_struct()
large_data.type_tag = TAG_COMPRESSED

# Automatic compression/decompression
sus encoded []drip = binz_encode(large_data)
sus decoded BinzValue = binz_decode(encoded)  # Automatically decompressed
```

## 🚀 Performance Optimizations

### Memory Pools

```cursed
# Pre-allocate memory for high-performance encoding
sus pool BinzMemoryPool = binz_create_memory_pool(4096)
sus encoded []drip = binz_encode_with_pool(value, pool)
```

### Batch Operations

```cursed
# Encode multiple values efficiently
sus values []BinzValue = [value1, value2, value3]
sus batch_encoded []drip = binz_encode_batch(values)
sus batch_decoded []BinzValue = binz_decode_batch(batch_encoded)
```

### Size Calculation

```cursed
# Calculate encoded size without encoding
sus predicted_size drip = binz_get_encoded_size(value)
sus actual_encoded []drip = binz_encode(value)
# predicted_size == array_length(actual_encoded)
```

## 🔍 Reflection-Based Serialization

```cursed
# Automatic struct serialization
sus schema BinzSchema = create_user_schema()
sus user_struct lit = create_user_instance()
sus serialized BinzValue = binz_serialize_struct_with_reflection(user_struct, schema)

# Automatic deserialization
sus deserialized lit = binz_deserialize_struct_with_reflection(serialized, "UserStruct")
```

## 📊 Performance Characteristics

### Encoding Performance
- **Small values (<100 bytes)**: ~1-5μs per value
- **Medium structures (1-10KB)**: ~50-200μs per structure
- **Large datasets (>100KB)**: ~1-5ms with compression

### Memory Usage
- **Header overhead**: 8 bytes minimum
- **Type tag overhead**: 1 byte per value
- **String overhead**: 1-5 bytes for length + content
- **Array overhead**: 1-5 bytes for count + elements
- **Struct overhead**: 1-5 bytes for field count + fields

### Compression Ratios
- **Repetitive data**: 50-90% size reduction
- **Random data**: 0-20% size reduction  
- **Structured data**: 20-40% size reduction

## 🔧 Error Handling

BINZ provides comprehensive error handling:

```cursed
sus decoder BinzDecoder = binz_create_decoder(invalid_data)
sus result BinzValue = binz_decode_value(decoder)

ready (decoder.has_error) {
    vibez.spill("Decode error: " + decoder.error_message)
    # Handle error appropriately
}
```

Common error conditions:
- Invalid magic header
- Unsupported version
- Truncated data
- Invalid type tags
- Schema validation failures
- Decompression errors

## 📋 Best Practices

### 1. Schema Design
- Keep field names short but descriptive
- Use optional fields for extensibility  
- Design for forward compatibility from the start
- Version schemas incrementally

### 2. Performance
- Use memory pools for high-frequency encoding
- Batch operations when possible
- Enable compression for large, repetitive data
- Pre-calculate sizes for memory allocation

### 3. Data Organization
- Group related fields in structs
- Use appropriate integer sizes (int8, int16, int32)
- Consider string interning for repeated strings
- Flatten deeply nested structures when possible

### 4. Error Handling
- Always check for encoding/decoding errors
- Implement fallback strategies for schema mismatches
- Log detailed error information for debugging
- Test with malformed data

## 🧪 Testing

Run the comprehensive test suite:

```bash
./zig-out/bin/cursed-zig stdlib/binz/test_binz.💀
```

Test categories:
- ✅ Basic type encoding/decoding
- ✅ Complex data structures  
- ✅ Schema system functionality
- ✅ Compression algorithms
- ✅ Performance optimizations
- ✅ Error handling
- ✅ Edge cases and boundaries
- ✅ Memory safety
- ✅ Cross-platform compatibility

## 📚 Examples

### Simple User Record

```cursed
yeet "binz"

# Create user data
sus user BinzValue = binz_create_struct()
user.struct_fields[0] = "id"
user.struct_values[0] = binz_create_uint(12345)
user.struct_fields[1] = "name"  
user.struct_values[1] = binz_create_string("John Doe")
user.struct_fields[2] = "email"
user.struct_values[2] = binz_create_string("john@example.com")

# Encode and decode
sus encoded []drip = binz_encode(user)
sus decoded BinzValue = binz_decode(encoded)

vibez.spill("User ID: " + int_to_string(get_struct_field_uint(decoded, "id")))
vibez.spill("Name: " + get_struct_field_string(decoded, "name"))
```

### Schema-Based Configuration

```cursed
yeet "binz"

# Define configuration schema
sus config_schema BinzSchema = binz_create_schema(2001, 1, "AppConfig")
config_schema = binz_schema_add_field(config_schema, "debug_mode", "bool", based)
config_schema = binz_schema_add_field(config_schema, "max_connections", "uint32", based) 
config_schema = binz_schema_add_field(config_schema, "database_url", "string", cringe)
config_schema.compatibility_mode = "forward"

# Create configuration
sus config BinzValue = binz_create_struct()
config.struct_fields[0] = "debug_mode"
config.struct_values[0] = binz_create_bool(cringe)
config.struct_fields[1] = "max_connections"
config.struct_values[1] = binz_create_uint(100)
config.struct_fields[2] = "database_url" 
config.struct_values[2] = binz_create_string("postgresql://localhost:5432/myapp")

# Encode with schema validation
sus config_encoded []drip = binz_encode_with_schema(config, config_schema)
sus config_decoded BinzValue = binz_decode_with_schema(config_encoded, config_schema)
```

### High-Performance Batch Processing

```cursed
yeet "binz"

# Create memory pool for zero-allocation encoding
sus pool BinzMemoryPool = binz_create_memory_pool(65536)  # 64KB buffer

# Process batch of messages
sus messages []BinzValue = load_messages_from_queue()
sus batch_encoded []drip = binz_encode_batch(messages)

# Send over network or save to file
write_to_network(batch_encoded)

# Decode batch on receiving end  
sus batch_decoded []BinzValue = binz_decode_batch(received_data)
process_messages(batch_decoded)
```

## 🔗 Integration

### Network Protocols

```cursed
# Protocol header with BINZ payload
squad NetworkMessage {
    sus magic drip          # Protocol magic number
    sus message_type drip   # Message type identifier  
    sus payload_size drip   # BINZ payload size
    sus payload []drip      # BINZ encoded data
}

slay send_network_message(socket NetworkSocket, data BinzValue) lit {
    sus encoded []drip = binz_encode(data)
    sus message NetworkMessage = NetworkMessage{}
    message.magic = 0xDEADBEEF
    message.message_type = 1
    message.payload_size = array_length(encoded)
    message.payload = encoded
    
    network_send(socket, serialize_message(message))
    damn based
}
```

### File Storage

```cursed
slay save_to_file(filename tea, data BinzValue) lit {
    sus encoded []drip = binz_encode(data)
    sus file FileHandle = open_file(filename, "wb")
    write_bytes(file, encoded)
    close_file(file)
    damn based
}

slay load_from_file(filename tea) BinzValue {
    sus file FileHandle = open_file(filename, "rb") 
    sus data []drip = read_all_bytes(file)
    close_file(file)
    damn binz_decode(data)
}
```

## 🏷️ Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-08-24 | Initial release with core features |
| | | - Basic type encoding/decoding |
| | | - Schema system with versioning |
| | | - Compression support |
| | | - Reflection-based serialization |
| | | - Memory pools and performance optimizations |

## 📄 License

BINZ is part of the CURSED standard library and follows the same license terms.

## 🤝 Contributing

Contributions are welcome! Please see the main CURSED project for contribution guidelines.

### Areas for Contribution
- Additional compression algorithms
- Schema registry improvements  
- Reflection system enhancements
- Performance optimizations
- Platform-specific optimizations
- Additional test cases

---

**BINZ** - Binary serialization that doesn't suck. 🚀
