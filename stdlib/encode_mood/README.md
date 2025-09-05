# encode_mood Module

The `encode_mood` module provides comprehensive encoding and decoding operations for the CURSED programming language. This FFI-free module supports multiple encoding formats essential for data interchange, web development, and self-hosting compiler operations.

## Features

### Base64 Encoding
- Standard Base64 encoding/decoding (RFC 4648)
- URL-safe Base64 encoding/decoding
- MIME Base64 encoding with line breaks
- Proper padding handling

### Hexadecimal Encoding
- Standard hexadecimal encoding/decoding
- Both uppercase and lowercase support
- Efficient byte-to-hex conversion

### Binary Encoding
- Binary string representation of data
- Bit-level data manipulation
- 8-bit byte encoding/decoding

### URL Encoding
- Percent encoding for URL-safe strings
- Automatic safe character detection
- RFC 3986 compliant encoding

### Quoted-Printable Encoding
- Email-safe text encoding
- MIME quoted-printable format
- Line length management

## Functions

### Base64 Operations
```cursed
slay base64_encode(input tea) tea
slay base64_decode(input tea) tea
slay base64_url_encode(input tea) tea
slay base64_url_decode(input tea) tea
slay base64_mime_encode(input tea) tea
```

### Hexadecimal Operations
```cursed
slay hex_encode(input tea) tea
slay hex_decode(input tea) tea
```

### Binary Operations
```cursed
slay binary_encode(input tea) tea
slay binary_decode(input tea) tea
```

### URL Operations
```cursed
slay percent_encode(input tea) tea
slay percent_decode(input tea) tea
```

### Quoted-Printable Operations
```cursed
slay quoted_printable_encode(input tea) tea
```

### Validation Functions
```cursed
slay is_valid_base64(input tea) lit
slay is_valid_hex(input tea) lit
slay is_valid_binary(input tea) lit
slay is_base64_char(ch sip) lit
slay is_hex_char(ch sip) lit
slay is_url_safe_char(ch sip) lit
```

### Helper Functions
```cursed
slay base64_char_to_value(ch sip) normie
slay hex_char_to_value(ch sip) normie
```

## Constants

```cursed
facts BASE64_ALPHABET tea = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
facts BASE64_PAD sip = '='
facts HEX_ALPHABET tea = "0123456789ABCDEF"
facts BINARY_ALPHABET tea = "01"
```

## Usage Examples

### Basic Base64 Encoding
```cursed
yeet "encode_mood"

sus data tea = "Hello, CURSED!"
sus encoded tea = base64_encode(data)
vibez.spill("Encoded: " + encoded)

sus decoded tea = base64_decode(encoded)
vibez.spill("Decoded: " + decoded)
```

### URL-Safe Encoding
```cursed
yeet "encode_mood"

sus url_data tea = "Hello+World/Test"
sus url_encoded tea = base64_url_encode(url_data)
vibez.spill("URL-safe: " + url_encoded)
```

### Hexadecimal Encoding
```cursed
yeet "encode_mood"

sus binary_data tea = "CURSED"
sus hex_encoded tea = hex_encode(binary_data)
vibez.spill("Hex: " + hex_encoded)

sus hex_decoded tea = hex_decode(hex_encoded)
vibez.spill("Original: " + hex_decoded)
```

### Binary Representation
```cursed
yeet "encode_mood"

sus byte_data tea = "A"
sus binary tea = binary_encode(byte_data)
vibez.spill("Binary: " + binary)  # Output: 01000001
```

### URL Percent Encoding
```cursed
yeet "encode_mood"

sus url tea = "Hello World & More!"
sus percent_encoded tea = percent_encode(url)
vibez.spill("Percent encoded: " + percent_encoded)

sus percent_decoded tea = percent_decode(percent_encoded)
vibez.spill("Decoded: " + percent_decoded)
```

### Validation
```cursed
yeet "encode_mood"

sus valid_b64 lit = is_valid_base64("SGVsbG8=")
sus valid_hex lit = is_valid_hex("48656C6C6F")
sus valid_bin lit = is_valid_binary("01001000")

vibez.spill("Valid Base64: " + valid_b64)
vibez.spill("Valid Hex: " + valid_hex)
vibez.spill("Valid Binary: " + valid_bin)
```

## Self-Hosting Support

The `encode_mood` module is critical for CURSED's self-hosting capabilities:

- **Compiler Metadata**: Encoding compiler metadata and debug information
- **Source Code Processing**: Handling encoded source files and templates
- **Binary Data**: Processing compiled bytecode and object files
- **Configuration**: Encoding/decoding configuration files
- **Network Operations**: HTTP header encoding and URL processing
- **Documentation**: Processing encoded documentation and examples

## Performance Characteristics

- **Pure CURSED Implementation**: No FFI dependencies for maximum portability
- **Memory Efficient**: Streaming processing for large data
- **Optimized Algorithms**: Efficient bit manipulation and string operations
- **Validation**: Built-in format validation for robust error handling

## Testing

The module includes comprehensive tests covering:

- Basic encoding/decoding operations for all formats
- Round-trip testing to ensure data integrity
- Edge cases (empty strings, null bytes, padding)
- Validation functions for input verification
- Large data handling and performance
- Constants and helper function verification

### Running Tests

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/encode_mood/test_encode_mood.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/encode_mood/test_encode_mood.💀
./test_encode_mood

# Both-mode verification
test_both_modes() {
    cargo run --bin cursed stdlib/encode_mood/test_encode_mood.💀 > interp_output.txt
    cargo run --bin cursed -- compile stdlib/encode_mood/test_encode_mood.💀
    ./test_encode_mood > comp_output.txt
    diff interp_output.txt comp_output.txt
}
```

## Error Handling

The module provides robust error handling:

- **Input Validation**: All functions validate input format and length
- **Graceful Degradation**: Invalid characters return safe default values
- **Format Verification**: Built-in validation functions for all encoding types
- **Boundary Checking**: Safe array and string access patterns

## Standards Compliance

- **RFC 4648**: Base64 and Base32 encoding standards
- **RFC 3986**: URI percent-encoding specification
- **RFC 2045**: MIME quoted-printable encoding
- **ASCII/UTF-8**: Full character set support

## Integration

The `encode_mood` module integrates seamlessly with other CURSED stdlib modules:

- **crypto**: Encoding cryptographic keys and signatures
- **network**: HTTP header and URL processing
- **filesystem**: Binary file encoding/decoding
- **config**: Configuration file processing
- **json**: JSON string encoding/escaping

## Dependencies

**Pure CURSED Implementation** - No external dependencies

Core stdlib functions used:
- String manipulation functions
- Character/byte conversion utilities
- Basic arithmetic operations

## Future Enhancements

Planned features for future versions:
- Base32 encoding support
- Additional character sets
- Streaming encoding for very large files
- Custom alphabet support
- Compression integration
- Unicode normalization

---

The `encode_mood` module represents a cornerstone of CURSED's data processing capabilities, providing enterprise-grade encoding operations essential for modern software development and compiler self-hosting.
