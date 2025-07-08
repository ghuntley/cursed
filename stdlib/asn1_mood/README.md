# ASN.1 Module (asn1_mood) for CURSED

The `asn1_mood` module provides comprehensive ASN.1 (Abstract Syntax Notation One) encoding and decoding capabilities for the CURSED programming language. This module implements DER (Distinguished Encoding Rules) and BER (Basic Encoding Rules) support with a pure CURSED implementation.

## Features

- **Complete ASN.1 Support**: Full implementation of ASN.1 encoding/decoding
- **DER/BER Format Support**: Support for both DER and BER encoding formats
- **Universal Tag Types**: Support for all common ASN.1 universal tag types
- **Pure CURSED Implementation**: No external dependencies, built on binary_drip
- **Comprehensive Testing**: 20+ test functions covering all functionality
- **Enterprise-Ready**: Production-quality ASN.1 processing

## Supported ASN.1 Types

### Universal Tag Classes
- `ASN1_UNIVERSAL` (0) - Universal tag class
- `ASN1_APPLICATION` (1) - Application-specific tag class
- `ASN1_CONTEXT_SPECIFIC` (2) - Context-specific tag class
- `ASN1_PRIVATE` (3) - Private tag class

### Universal Tag Types
- `ASN1_INTEGER` (2) - Integer values
- `ASN1_BIT_STRING` (3) - Bit string values
- `ASN1_OCTET_STRING` (4) - Octet string values
- `ASN1_NULL` (5) - Null values
- `ASN1_OBJECT_IDENTIFIER` (6) - Object identifier values
- `ASN1_SEQUENCE` (16) - Sequence (constructed)
- `ASN1_SET` (17) - Set (constructed)
- `ASN1_PRINTABLE_STRING` (19) - Printable string values
- `ASN1_T61_STRING` (20) - T61 string values
- `ASN1_IA5_STRING` (22) - IA5 string values
- `ASN1_UTC_TIME` (23) - UTC time values
- `ASN1_GENERALIZED_TIME` (24) - Generalized time values

## Core Data Structures

### ASN1Tag
```cursed
struct ASN1Tag {
    class normie        # Tag class (universal, application, etc.)
    constructed lit     # Whether tag is constructed
    tag_number normie   # Tag number
}
```

### ASN1Object
```cursed
struct ASN1Object {
    tag ASN1Tag        # ASN.1 tag
    length normie      # Content length
    data tea          # Content data
}
```

## Core Functions

### Object Creation Functions

#### `asn1_tag_new(class, constructed, tag_number)`
Create a new ASN.1 tag.
- `class`: Tag class (ASN1_UNIVERSAL, ASN1_APPLICATION, etc.)
- `constructed`: Whether tag is constructed (based/cap)
- `tag_number`: Tag number (ASN1_INTEGER, ASN1_SEQUENCE, etc.)
- Returns: `ASN1Tag` structure

#### `asn1_int_new(value)`
Create an ASN.1 integer object.
- `value`: Integer value to encode
- Returns: `ASN1Object` with INTEGER tag

#### `asn1_string_new(value)`
Create an ASN.1 string object.
- `value`: String value to encode
- Returns: `ASN1Object` with OCTET_STRING tag

#### `asn1_sequence_new()`
Create an ASN.1 sequence object.
- Returns: `ASN1Object` with SEQUENCE tag (constructed)

#### `asn1_set_new()`
Create an ASN.1 set object.
- Returns: `ASN1Object` with SET tag (constructed)

#### `asn1_oid_new(oid)`
Create an ASN.1 object identifier.
- `oid`: OID string (e.g., "1.2.3.4")
- Returns: `ASN1Object` with OBJECT_IDENTIFIER tag

#### `asn1_time_new(time)`
Create an ASN.1 time object.
- `time`: Time string (e.g., "20231207120000Z")
- Returns: `ASN1Object` with UTC_TIME tag

#### `asn1_bitstring_new(bits)`
Create an ASN.1 bit string object.
- `bits`: Bit string value
- Returns: `ASN1Object` with BIT_STRING tag

### Encoding Functions

#### `asn1_encode(obj)`
Generic ASN.1 encoding function.
- `obj`: ASN1Object to encode
- Returns: Encoded byte string

#### `asn1_encode_der(obj)`
Encode ASN.1 object to DER format.
- `obj`: ASN1Object to encode
- Returns: DER-encoded byte string

#### `asn1_encode_ber(obj)`
Encode ASN.1 object to BER format.
- `obj`: ASN1Object to encode
- Returns: BER-encoded byte string

### Decoding Functions

#### `asn1_decode(data)`
Generic ASN.1 decoding function.
- `data`: Encoded byte string
- Returns: Decoded ASN1Object

#### `asn1_parse_der(data)`
Parse ASN.1 DER-encoded data.
- `data`: DER-encoded byte string
- Returns: Parsed ASN1Object

#### `asn1_parse_ber(data)`
Parse ASN.1 BER-encoded data.
- `data`: BER-encoded byte string
- Returns: Parsed ASN1Object

## Usage Examples

### Basic Object Creation
```cursed
yeet "asn1_mood"

# Create integer object
sus int_obj ASN1Object = asn1_int_new(42)

# Create string object
sus str_obj ASN1Object = asn1_string_new("Hello ASN.1")

# Create sequence object
sus seq_obj ASN1Object = asn1_sequence_new()

# Create OID object
sus oid_obj ASN1Object = asn1_oid_new("1.2.840.113549.1.1.1")
```

### Encoding and Decoding
```cursed
yeet "asn1_mood"

# Create and encode an object
sus original ASN1Object = asn1_int_new(12345)
sus encoded tea = asn1_encode_der(original)

# Decode the encoded data
sus decoded ASN1Object = asn1_parse_der(encoded)

# Verify the tag type
bestie decoded.tag.tag_number == ASN1_INTEGER {
    vibez.spill("Successfully decoded integer!")
}
```

### Working with Different Types
```cursed
yeet "asn1_mood"

# Create various ASN.1 objects
sus int_obj ASN1Object = asn1_int_new(255)
sus str_obj ASN1Object = asn1_string_new("test data")
sus time_obj ASN1Object = asn1_time_new("20231207120000Z")
sus bits_obj ASN1Object = asn1_bitstring_new("10110100")

# Encode all objects
sus encoded_int tea = asn1_encode(int_obj)
sus encoded_str tea = asn1_encode(str_obj)
sus encoded_time tea = asn1_encode(time_obj)
sus encoded_bits tea = asn1_encode(bits_obj)
```

### OID Handling
```cursed
yeet "asn1_mood"

# Create OID objects
sus rsa_oid ASN1Object = asn1_oid_new("1.2.840.113549.1.1.1")
sus sha256_oid ASN1Object = asn1_oid_new("2.16.840.1.101.3.4.2.1")
sus simple_oid ASN1Object = asn1_oid_new("1.2.3")

# Encode OIDs
sus encoded_rsa tea = asn1_encode_der(rsa_oid)
sus encoded_sha256 tea = asn1_encode_der(sha256_oid)
```

## Testing

The module includes comprehensive tests covering all functionality:

```bash
# Run ASN.1 module tests (interpretation mode)
cargo run --bin cursed stdlib/asn1_mood/test_asn1_mood.csd

# Run ASN.1 module tests (compilation mode)
cargo run --bin cursed -- compile stdlib/asn1_mood/test_asn1_mood.csd
./test_asn1_mood
```

## Test Coverage

The test suite includes 20+ comprehensive tests:

- **Object Creation Tests**: Tag creation, integer objects, string objects, sequences, sets, OIDs, time objects, bit strings
- **Encoding Tests**: DER encoding, BER encoding, generic encoding
- **Decoding Tests**: DER parsing, BER parsing, generic decoding
- **Tag Constant Tests**: Verification of all tag constants
- **Length Encoding Tests**: Short and long length encoding
- **Integer Encoding Tests**: Positive, zero, and negative integers
- **OID Encoding Tests**: Simple and complex OID encoding
- **Comprehensive Tests**: Multi-object encoding/decoding
- **Error Handling Tests**: Edge cases and error conditions

## Integration with Other Modules

The ASN.1 module builds on the `binary_drip` module for efficient binary operations and integrates well with:

- **Crypto Module**: For ASN.1 encoding of cryptographic structures
- **Network Module**: For ASN.1 protocol implementations
- **Certificate Module**: For X.509 certificate processing

## Performance Characteristics

- **Memory Efficient**: Minimal memory allocation for ASN.1 objects
- **Fast Encoding**: Optimized encoding algorithms
- **Robust Parsing**: Comprehensive error handling and validation
- **Scalable**: Handles both simple and complex ASN.1 structures

## Standards Compliance

This implementation follows:
- **ITU-T X.690**: ASN.1 encoding rules specification
- **RFC 5280**: X.509 certificate and CRL profile
- **RFC 3370**: Cryptographic algorithms and identifiers

## Common Use Cases

1. **X.509 Certificate Processing**: Encoding/decoding certificate structures
2. **Cryptographic Key Management**: ASN.1 encoding of public/private keys
3. **Protocol Implementation**: ASN.1-based network protocols
4. **Data Serialization**: Structured data encoding/decoding
5. **Standards Compliance**: Meeting ASN.1 requirements in enterprise applications

## Error Handling

The module provides robust error handling for:
- Invalid ASN.1 structures
- Malformed encoding
- Unsupported tag types
- Length encoding errors
- OID parsing failures

## Future Enhancements

Planned improvements include:
- Additional ASN.1 tag types
- Streaming ASN.1 processing
- Performance optimizations
- Extended error reporting
- Integration with certificate validation

## License

This module is part of the CURSED programming language standard library and follows the same licensing terms.
