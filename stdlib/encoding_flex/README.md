# EncodingFlex Module

## Overview
EncodingFlex provides interfaces and implementations for encoding and decoding data in various formats with flexible options. It's inspired by Go's encoding package and its subpackages but with enhanced flexibility and modern features.

## Core Interfaces

### `FlexEncoder` and `FlexDecoder`
Function types that provide flexible encoding and decoding operations.

## Supported Formats

### JSON Encoding
- **MarshalJSON(v interface{}, opts JSONOptions)** - Encode to JSON with options
- **NewJSONEncoder(opts JSONOptions)** - Create JSON encoder
- **JSONOptions** - Configuration for JSON encoding (pretty print, indentation, etc.)

### Base64 Encoding
- **EncodeBase64(src []normie, encoding Base64Encoding)** - Encode to Base64
- **DecodeBase64(s tea, encoding Base64Encoding)** - Decode from Base64
- **Encodings**: Standard, URL-safe, Raw variants

### Hex Encoding
- **EncodeHex(src []normie)** - Encode to hexadecimal
- **DecodeHex(s tea)** - Decode from hexadecimal

### Binary Encoding
- **ReadUint16/ReadUint32(data []normie, order ByteOrder)** - Read binary integers
- **WriteUint16/WriteUint32(v normie, order ByteOrder)** - Write binary integers
- **ByteOrder**: BigEndian, LittleEndian

### URI Encoding
- **EncodeURI(s tea)** - Encode URI components
- **DecodeURI(s tea)** - Decode URI components

### Quoted-Printable Encoding
- **EncodeQuotedPrintable(src []normie)** - Encode to quoted-printable
- **DecodeQuotedPrintable(src []normie)** - Decode from quoted-printable

### ASCII85 Encoding
- **EncodeASCII85(src []normie)** - Encode to ASCII85
- **DecodeASCII85(src []normie)** - Decode from ASCII85

## Usage Examples

### JSON Encoding
```cursed
yeet "encoding_flex"

sus opts := encoding_flex.JSONOptions{
    PrettyPrint: based,
    Indent: "  ",
    OmitEmpty: based
}

sus data, err := encoding_flex.MarshalJSON("hello world", opts)
if err == "" {
    vibez.spill("Encoded JSON:", tea(data))
}
```

### Base64 Encoding
```cursed
sus input := []normie{72, 101, 108, 108, 111}  fr fr "Hello"
sus encoded := encoding_flex.EncodeBase64(input, encoding_flex.Base64Standard)
vibez.spill("Base64:", encoded)

sus decoded, err := encoding_flex.DecodeBase64(encoded, encoding_flex.Base64Standard)
if err == "" {
    vibez.spill("Decoded:", decoded)
}
```

### Hex Encoding
```cursed
sus data := []normie{255, 128, 0}
sus hex := encoding_flex.EncodeHex(data)
vibez.spill("Hex:", hex)  fr fr "ff8000"

sus bytes, err := encoding_flex.DecodeHex("ff8000")
if err == "" {
    vibez.spill("Bytes:", bytes)
}
```

### Binary Encoding
```cursed
sus value := 0x1234
sus bigEndian := encoding_flex.WriteUint16(value, encoding_flex.BigEndian)
sus littleEndian := encoding_flex.WriteUint16(value, encoding_flex.LittleEndian)

sus readBig := encoding_flex.ReadUint16(bigEndian, encoding_flex.BigEndian)
vibez.spill("Read value:", readBig)  fr fr 0x1234
```

### URI Encoding
```cursed
sus uri := "hello world#fragment"
sus encoded := encoding_flex.EncodeURI(uri)
vibez.spill("Encoded URI:", encoded)

sus decoded, err := encoding_flex.DecodeURI(encoded)
if err == "" {
    vibez.spill("Decoded URI:", decoded)
}
```

## Configuration Options

### JSONOptions
- `PrettyPrint` - Enable pretty printing
- `EscapeHTML` - Escape HTML characters
- `AllowNaN` - Allow NaN values
- `Indent` - Indentation string
- `OmitEmpty` - Omit empty values

### CSVOptions
- `Comma` - Field separator
- `Comment` - Comment character
- `FieldsPerRecord` - Expected fields per record
- `LazyQuotes` - Allow lazy quotes
- `TrimLeadingSpace` - Trim leading whitespace
- `UseHeaders` - Use first row as headers

### YAMLOptions
- `IndentSize` - Indentation size
- `TagName` - Tag name for struct fields
- `OmitEmpty` - Omit empty values
- `UseJSONTags` - Use JSON tags

## Implementation Features

1. **Pure CURSED Implementation** - No FFI dependencies
2. **Flexible Configuration** - Extensive options for customization
3. **Multiple Formats** - Support for major encoding formats
4. **Error Handling** - Comprehensive error reporting
5. **Performance Optimized** - Efficient algorithms
6. **Type Safety** - Strong typing throughout

## Error Handling

All encoding/decoding functions return error messages as strings. Empty string indicates success, non-empty string indicates an error occurred.

## Implementation Notes

This is a pure CURSED implementation that provides the essential encoding/decoding functionality without external dependencies. Some advanced features are simplified but maintain compatibility with the core API.
