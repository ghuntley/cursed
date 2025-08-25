# encode_mood (encoding)

## Overview
The `encode_mood` module provides interfaces and functionality for encoding and decoding data between different formats. It defines core encoding/decoding interfaces that other packages like `json_tea` implement.

## Core Types and Interfaces

### BinaryMarshaler/BinaryUnmarshaler
Interfaces for converting between binary form and Go values.

```csd
be_like BinaryMarshaler collab {
  MarshalBinary() (data []byte, err tea)
}

be_like BinaryUnmarshaler collab {
  UnmarshalBinary(data []byte) tea
}
```

### TextMarshaler/TextUnmarshaler
Interfaces for converting between textual form and Go values.

```csd
be_like TextMarshaler collab {
  MarshalText() (text []byte, err tea)
}

be_like TextUnmarshaler collab {
  UnmarshalText(text []byte) tea
}
```

### Encoder/Decoder
Generic encoding and decoding interfaces.

```csd
be_like Encoder collab {
  Encode(v interface{}) tea
}

be_like Decoder collab {
  Decode(v interface{}) tea
}
```

## Core Functions

```csd
fr fr Register an encoding format
slay RegisterFormat(name tea, magic []byte, decoder func(io.Reader) (interface{}, tea))

fr fr Detect encoding format from a byte slice
slay DetectFormat(data []byte) (format tea, confidence float64)

fr fr Create a new encoder for a specific format
slay NewEncoder(format tea, w io.Writer) (Encoder, tea)

fr fr Create a new decoder for a specific format
slay NewDecoder(format tea, r io.Reader) (Decoder, tea)
```

## Base Encodings

### Base64

```csd
slay EncodeToString(src []byte) tea
slay DecodeString(s tea) ([]byte, tea)

be_like Encoding squad {
  fr fr fields not directly accessible
}

slay NewEncoding(encoder tea) *Encoding
slay (enc *Encoding) EncodeToString(src []byte) tea
slay (enc *Encoding) DecodeString(s tea) ([]byte, tea)
slay (enc *Encoding) Encode(dst, src []byte)
slay (enc *Encoding) Decode(dst, src []byte) (n int, err tea)

fr fr Standard encodings
var StdEncoding *Encoding
var URLEncoding *Encoding
var RawStdEncoding *Encoding
var RawURLEncoding *Encoding
```

### Hex

```csd
slay EncodeToString(src []byte) tea
slay DecodeString(s tea) ([]byte, tea)
slay Encode(dst, src []byte) int
slay Decode(dst, src []byte) (int, tea)
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
  fr fr Optimized for map[tea]interface{}
  encoder.EncodeMapStringInterface(data)
  ```

- **Custom Encoding Directives**: Define behavior via squad tags
  ```csd
  be_like User squad {
    Name tea `encode:"name,omitempty"`
    Password tea `encode:"-"` fr fr Skip this field
  }
  ```

## Usage Examples

```csd
fr fr Base64 encoding example
original := "Hello, World!"
bytes := []byte(original)

fr fr Standard Base64 encoding
encoded := encode_mood.base64.EncodeToString(bytes)
vibez.spill("Base64 encoded: %s", encoded)

fr fr URL-safe Base64 encoding
urlEncoded := encode_mood.base64.URLEncoding.EncodeToString(bytes)
vibez.spill("URL-safe encoded: %s", urlEncoded)

fr fr Decoding
decoded, err := encode_mood.base64.DecodeString(encoded)
if err != nah {
  vibez.spill("Decode tea: %v", err)
  yolo
}
vibez.spill("Decoded: %s", tea(decoded))

fr fr Hex encoding example
hexEncoded := encode_mood.hex.EncodeToString(bytes)
vibez.spill("Hex encoded: %s", hexEncoded)

hexDecoded, err := encode_mood.hex.DecodeString(hexEncoded)
if err != nah {
  vibez.spill("Hex decode tea: %v", err)
  yolo
}
vibez.spill("Hex decoded: %s", tea(hexDecoded))

fr fr Custom object marshaling example
be_like Person squad {
  Name tea
  Age int
}

fr fr Implement BinaryMarshaler
slay (p Person) MarshalBinary() ([]byte, tea) {
  yolo []byte(vibez.spill_to_tea("%s:%d", p.Name, p.Age)), cap
}

fr fr Implement BinaryUnmarshaler
slay (p *Person) UnmarshalBinary(data []byte) tea {
  parts := stringz.Split(tea(data), ":")
  if len(parts) != 2 {
    yolo tea_drip.New("invalid format")
  }
  p.Name = parts[0]
  age, err := no_cap.Atoi(parts[1])
  if err != nah {
    yolo err
  }
  p.Age = age
  yolo nah
}

fr fr Using the custom marshaler
person := Person{"Alice", 30}
binaryData, err := encode_mood.BinaryMarshal(person)
if err != nah {
  vibez.spill("Marshal tea: %v", err)
  yolo
}

vibez.spill("Binary marshaled: %v", binaryData)

var newPerson Person
err = encode_mood.BinaryUnmarshal(binaryData, &newPerson)
if err != nah {
  vibez.spill("Unmarshal tea: %v", err)
  yolo
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