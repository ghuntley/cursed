# encode_mood (encoding)

## Overview
The `encode_mood` module provides interfaces and functionality for encoding and decoding data between different formats. It defines core encoding/decoding interfaces that other packages like `json_tea` implement.

## Core Types and Interfaces

### BinaryMarshaler/BinaryUnmarshaler
Interfaces for converting between binary form and Go values.

```csd
type BinaryMarshaler interface {
  MarshalBinary() (data []byte, err error)
}

type BinaryUnmarshaler interface {
  UnmarshalBinary(data []byte) error
}
```

### TextMarshaler/TextUnmarshaler
Interfaces for converting between textual form and Go values.

```csd
type TextMarshaler interface {
  MarshalText() (text []byte, err error)
}

type TextUnmarshaler interface {
  UnmarshalText(text []byte) error
}
```

### Encoder/Decoder
Generic encoding and decoding interfaces.

```csd
type Encoder interface {
  Encode(v interface{}) error
}

type Decoder interface {
  Decode(v interface{}) error
}
```

## Core Functions

```csd
// Register an encoding format
func RegisterFormat(name string, magic []byte, decoder func(io.Reader) (interface{}, error))

// Detect encoding format from a byte slice
func DetectFormat(data []byte) (format string, confidence float64)

// Create a new encoder for a specific format
func NewEncoder(format string, w io.Writer) (Encoder, error)

// Create a new decoder for a specific format
func NewDecoder(format string, r io.Reader) (Decoder, error)
```

## Base Encodings

### Base64

```csd
func EncodeToString(src []byte) string
func DecodeString(s string) ([]byte, error)

type Encoding struct {
  // fields not directly accessible
}

func NewEncoding(encoder string) *Encoding
func (enc *Encoding) EncodeToString(src []byte) string
func (enc *Encoding) DecodeString(s string) ([]byte, error)
func (enc *Encoding) Encode(dst, src []byte)
func (enc *Encoding) Decode(dst, src []byte) (n int, err error)

// Standard encodings
var StdEncoding *Encoding
var URLEncoding *Encoding
var RawStdEncoding *Encoding
var RawURLEncoding *Encoding
```

### Hex

```csd
func EncodeToString(src []byte) string
func DecodeString(s string) ([]byte, error)
func Encode(dst, src []byte) int
func Decode(dst, src []byte) (int, error)
```

## Enhanced Features

- **Format Registry**: Central registry of supported encoding formats
  ```csd
  encode_mood.RegisterFormat("myformat", []byte{0x1F, 0x8B}, myFormatDecoder)
  ```

- **Streaming Encoders/Decoders**: Process large data in chunks
  ```csd
  encoder := encode_mood.NewStreamingEncoder("json", writer)
  encoder.EncodeChunk(data1)
  encoder.EncodeChunk(data2)
  encoder.Finish()
  ```

- **Format Detection**: Automatically detect and decode unknown formats
  ```csd
  format, _ := encode_mood.DetectFormat(data)
  decoder, _ := encode_mood.NewDecoder(format, reader)
  ```

- **Performance Optimizations**: Specialized encoders for common types
  ```csd
  // Optimized for map[string]interface{}
  encoder.EncodeMapStringInterface(data)
  ```

- **Custom Encoding Directives**: Define behavior via struct tags
  ```csd
  type User struct {
    Name string `encode:"name,omitempty"`
    Password string `encode:"-"` // Skip this field
  }
  ```

## Usage Examples

```csd
// Base64 encoding example
original := "Hello, World!"
bytes := []byte(original)

// Standard Base64 encoding
encoded := encode_mood.base64.EncodeToString(bytes)
vibez.spill("Base64 encoded: %s", encoded)

// URL-safe Base64 encoding
urlEncoded := encode_mood.base64.URLEncoding.EncodeToString(bytes)
vibez.spill("URL-safe encoded: %s", urlEncoded)

// Decoding
decoded, err := encode_mood.base64.DecodeString(encoded)
if err != nil {
  vibez.spill("Decode error: %v", err)
  return
}
vibez.spill("Decoded: %s", string(decoded))

// Hex encoding example
hexEncoded := encode_mood.hex.EncodeToString(bytes)
vibez.spill("Hex encoded: %s", hexEncoded)

hexDecoded, err := encode_mood.hex.DecodeString(hexEncoded)
if err != nil {
  vibez.spill("Hex decode error: %v", err)
  return
}
vibez.spill("Hex decoded: %s", string(hexDecoded))

// Custom object marshaling example
type Person struct {
  Name string
  Age int
}

// Implement BinaryMarshaler
func (p Person) MarshalBinary() ([]byte, error) {
  return []byte(vibez.spill_to_string("%s:%d", p.Name, p.Age)), nil
}

// Implement BinaryUnmarshaler
func (p *Person) UnmarshalBinary(data []byte) error {
  parts := stringz.Split(string(data), ":")
  if len(parts) != 2 {
    return error_drip.New("invalid format")
  }
  p.Name = parts[0]
  age, err := no_cap.Atoi(parts[1])
  if err != nil {
    return err
  }
  p.Age = age
  return nil
}

// Using the custom marshaler
person := Person{"Alice", 30}
binaryData, err := encode_mood.BinaryMarshal(person)
if err != nil {
  vibez.spill("Marshal error: %v", err)
  return
}

vibez.spill("Binary marshaled: %v", binaryData)

var newPerson Person
err = encode_mood.BinaryUnmarshal(binaryData, &newPerson)
if err != nil {
  vibez.spill("Unmarshal error: %v", err)
  return
}

vibez.spill("Unmarshaled person: %s, %d", newPerson.Name, newPerson.Age)
```

## Implementation Guidelines

- Interface definitions should be minimal but complete
- Error handling should be clear and descriptive
- Performance should be prioritized for common encoding operations
- Memory usage should be optimized for large data sets
- Support for custom encoders and decoders should be straightforward
- Thread safety is required for all encoding/decoding operations
- All encoders/decoders should validate input data
- Provide sensible defaults while allowing customization