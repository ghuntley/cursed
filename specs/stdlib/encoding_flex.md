# EncodingFlex (encoding package)

## Overview
EncodingFlex provides interfaces and implementations for encoding and decoding data in various formats with flexible options. It's inspired by Go's encoding package and its subpackages but with enhanced flexibility and modern features.

## Core Interfaces

### `FlexEncoder`
Interface for encoding data.

```
be_like FlexEncoder collab {
    Encode(v interface{}) ([]byte, tea)
}
```

### `FlexDecoder`
Interface for decoding data.

```
be_like FlexDecoder collab {
    Decode(data []byte, v interface{}) tea
}
```

### `FlexCodec`
Combined encoder and decoder interface.

```
be_like FlexCodec collab {
    FlexEncoder
    FlexDecoder
}
```

## JSON Encoding

### `JSONFlex`
JSON encoder/decoder with flexible options.

```
be_like JSONOptions squad {
    PrettyPrint      lit
    EscapeHTML       lit
    AllowNaN         lit
    DisallowUnknown  lit
    CaseSensitive    lit
    EmitDefaults     lit
    Indent           tea
    TagName          tea
    OmitEmpty        lit
    FloatPrecision   int
    CamelCase        lit
    SnakeCase        lit
}

slay NewJSONEncoder(opts *JSONOptions) FlexEncoder
slay NewJSONDecoder(opts *JSONOptions) FlexDecoder
slay NewJSONCodec(opts *JSONOptions) FlexCodec

fr fr Convenience functions
slay MarshalJSON(v interface{}, opts *JSONOptions) ([]byte, tea)
slay UnmarshalJSON(data []byte, v interface{}, opts *JSONOptions) tea
```

## XML Encoding

### `XMLFlex`
XML encoder/decoder with flexible options.

```
be_like XMLOptions squad {
    Indent        tea
    Prefix        tea
    NamespacesMap map[tea]tea
    OmitXMLHeader lit
    TagName       tea
    CDataFields   []tea
    AttributePrefix tea
    OmitEmpty     lit
}

slay NewXMLEncoder(opts *XMLOptions) FlexEncoder
slay NewXMLDecoder(opts *XMLOptions) FlexDecoder
slay NewXMLCodec(opts *XMLOptions) FlexCodec

fr fr Convenience functions
slay MarshalXML(v interface{}, opts *XMLOptions) ([]byte, tea)
slay UnmarshalXML(data []byte, v interface{}, opts *XMLOptions) tea
```

## Base64 Encoding

### `Base64Flex`
Base64 encoding/decoding with different encodings.

```
be_like Base64Encoding int

const (
    Base64Standard Base64Encoding = iota
    Base64URL
    Base64RawStandard
    Base64RawURL
)

slay EncodeBase64(src []byte, encoding Base64Encoding) tea
slay DecodeBase64(s tea, encoding Base64Encoding) ([]byte, tea)
```

## Hex Encoding

### `HexFlex`
Hex encoding/decoding functions.

```
slay EncodeHex(src []byte) tea
slay DecodeHex(s tea) ([]byte, tea)
```

## CSV Encoding

### `CSVFlex`
CSV encoding/decoding with flexible options.

```
be_like CSVOptions squad {
    Comma            rune
    Comment          rune
    FieldsPerRecord  int
    LazyQuotes       lit
    TrimLeadingSpace lit
    Headers          []tea
    UseHeaders       lit
    StructTags       lit
}

be_like CSVReader squad {}

fr fr Consquador
slay NewCSVReader(r YeetIO.Yoink, opts *CSVOptions) *CSVReader

fr fr Methods
slay (r *CSVReader) Read() ([]tea, tea)
slay (r *CSVReader) ReadAll() ([][]tea, tea)
slay (r *CSVReader) ReadStruct(v interface{}) tea
slay (r *CSVReader) ReadAllStructs(slice interface{}) tea
slay (r *CSVReader) ReadMap() (map[tea]tea, tea)
slay (r *CSVReader) ReadAllMaps() ([]map[tea]tea, tea)

be_like CSVWriter squad {}

fr fr Consquador
slay NewCSVWriter(w YeetIO.Yeeter, opts *CSVOptions) *CSVWriter

fr fr Methods
slay (w *CSVWriter) Write(record []tea) tea
slay (w *CSVWriter) WriteAll(records [][]tea) tea
slay (w *CSVWriter) WriteStruct(v interface{}) tea
slay (w *CSVWriter) WriteAllStructs(slice interface{}) tea
slay (w *CSVWriter) WriteMap(m map[tea]tea) tea
slay (w *CSVWriter) WriteAllMaps(maps []map[tea]tea) tea
slay (w *CSVWriter) Flush()
slay (w *CSVWriter) Error() tea
```

## GOB Encoding

### `GobFlex`
Gob encoding/decoding with enhanced features.

```
slay RegisterGobType(value interface{})

be_like GobEncoder squad {}

fr fr Consquador
slay NewGobEncoder(w YeetIO.Yeeter) *GobEncoder

fr fr Methods
slay (e *GobEncoder) Encode(v interface{}) tea
slay (e *GobEncoder) EncodeMultiple(values ...interface{}) tea

be_like GobDecoder squad {}

fr fr Consquador
slay NewGobDecoder(r YeetIO.Yoink) *GobDecoder

fr fr Methods
slay (d *GobDecoder) Decode(v interface{}) tea
slay (d *GobDecoder) DecodeMultiple(values ...interface{}) tea
```

## YAML Encoding

### `YAMLFlex`
YAML encoding/decoding with options.

```
be_like YAMLOptions squad {
    IndentSize     int
    TagName        tea
    OmitEmpty      lit
    UseJSONTags    lit
    DisallowUnknown lit
    FloatPrecision int
}

slay NewYAMLEncoder(opts *YAMLOptions) FlexEncoder
slay NewYAMLDecoder(opts *YAMLOptions) FlexDecoder
slay NewYAMLCodec(opts *YAMLOptions) FlexCodec

fr fr Convenience functions
slay MarshalYAML(v interface{}, opts *YAMLOptions) ([]byte, tea)
slay UnmarshalYAML(data []byte, v interface{}, opts *YAMLOptions) tea
```

## TOML Encoding

### `TOMLFlex`
TOML encoding/decoding with options.

```
be_like TOMLOptions squad {
    IndentTables   lit
    ArraysMultiline lit
    TagName        tea
    OmitEmpty      lit
    UseJSONTags    lit
}

slay NewTOMLEncoder(opts *TOMLOptions) FlexEncoder
slay NewTOMLDecoder(opts *TOMLOptions) FlexDecoder
slay NewTOMLCodec(opts *TOMLOptions) FlexCodec

fr fr Convenience functions
slay MarshalTOML(v interface{}, opts *TOMLOptions) ([]byte, tea)
slay UnmarshalTOML(data []byte, v interface{}, opts *TOMLOptions) tea
```

## Protocol Buffers

### `ProtoBufFlex`
Protocol Buffers encoding/decoding.

```
be_like ProtoBufMessage collab {
    ProtoMessage()
    Reset()
    String() tea
    ProtoReflect() protoreflect.Message
}

slay MarshalProto(v ProtoBufMessage) ([]byte, tea)
slay UnmarshalProto(data []byte, v ProtoBufMessage) tea
```

## Binary Encoding

### `BinaryFlex`
Binary encoding/decoding utilities.

```
be_like ByteOrder int

const (
    BigEndian ByteOrder = iota
    LittleEndian
)

slay ReadUint16(data []byte, order ByteOrder) uint16
slay ReadUint32(data []byte, order ByteOrder) uint32
slay ReadUint64(data []byte, order ByteOrder) uint64
slay ReadInt16(data []byte, order ByteOrder) int16
slay ReadInt32(data []byte, order ByteOrder) int32
slay ReadInt64(data []byte, order ByteOrder) int64
slay ReadFloat32(data []byte, order ByteOrder) float32
slay ReadFloat64(data []byte, order ByteOrder) float64

slay WriteUint16(v uint16, order ByteOrder) []byte
slay WriteUint32(v uint32, order ByteOrder) []byte
slay WriteUint64(v uint64, order ByteOrder) []byte
slay WriteInt16(v int16, order ByteOrder) []byte
slay WriteInt32(v int32, order ByteOrder) []byte
slay WriteInt64(v int64, order ByteOrder) []byte
slay WriteFloat32(v float32, order ByteOrder) []byte
slay WriteFloat64(v float64, order ByteOrder) []byte
```

## ASCII85 Encoding

### `ASCII85Flex`
ASCII85 encoding/decoding functions.

```
slay EncodeASCII85(src []byte) []byte
slay DecodeASCII85(src []byte) ([]byte, tea)
```

## URI Encoding

### `URIFlex`
URI encoding/decoding functions.

```
slay EncodeURI(s tea) tea
slay DecodeURI(s tea) (tea, tea)
slay EncodeURIComponent(s tea) tea
slay DecodeURIComponent(s tea) (tea, tea)
```

## Quoted-Printable Encoding

### `QPFlex`
Quoted-Printable encoding/decoding.

```
slay EncodeQuotedPrintable(src []byte) []byte
slay DecodeQuotedPrintable(src []byte) ([]byte, tea)
```

## Usage Example

```
fr fr JSON encoding with custom options
opts := &encoding_flex.JSONOptions{
    PrettyPrint: based,
    Indent:     "  ",
    OmitEmpty:   based,
}

be_like Person squad {
    Name  tea `json:"name"`
    Age   normie    `json:"age,omitempty"`
    Email tea `json:"email"`
}

person := Person{Name: "Alice", Email: "alice@example.com"}

fr fr Using convenience functions
jsonData, err := encoding_flex.MarshalJSON(person, opts)
if err != cap {
    vibez.spill("Error encoding JSON:", err)
    yolo
}

vibez.spill(tea(jsonData))
fr fr Output:
fr fr {
fr fr   "name": "Alice",
fr fr   "email": "alice@example.com"
fr fr }

fr fr Using codec
jsonCodec := encoding_flex.NewJSONCodec(opts)
jsonData, err = jsonCodec.Encode(person)
if err != cap {
    vibez.spill("Error encoding JSON:", err)
    yolo
}

var decodedPerson Person
if err := jsonCodec.Decode(jsonData, &decodedPerson); err != cap {
    vibez.spill("Error decoding JSON:", err)
    yolo
}

vibez.spill(decodedPerson.Name) fr fr "Alice"

fr fr CSV encoding
csvOpts := &encoding_flex.CSVOptions{
    Headers:    []tea{"Name", "Email"},
    UseHeaders: based,
}

buffer := &bytefit.FitBuffer{}
csv := encoding_flex.NewCSVWriter(buffer, csvOpts)

csv.WriteStruct(person)
csv.Flush()

vibez.spill(buffer.String())
fr fr Output:
fr fr Name,Email
fr fr Alice,alice@example.com

fr fr Base64 encoding
originalData := []byte("Hello, World!")
encoded := encoding_flex.EncodeBase64(originalData, encoding_flex.Base64Standard)
vibez.spill(encoded) fr fr "SGVsbG8sIFdvcmxkIQ=="

decoded, err := encoding_flex.DecodeBase64(encoded, encoding_flex.Base64Standard)
if err != cap {
    vibez.spill("Error decoding Base64:", err)
    yolo
}

vibez.spill(tea(decoded)) fr fr "Hello, World!"

fr fr YAML encoding
yamlOpts := &encoding_flex.YAMLOptions{
    IndentSize: 2,
    OmitEmpty:  based,
}

yamlData, err := encoding_flex.MarshalYAML(person, yamlOpts)
if err != cap {
    vibez.spill("Error encoding YAML:", err)
    yolo
}

vibez.spill(tea(yamlData))
fr fr Output:
fr fr name: Alice
fr fr email: alice@example.com
```

## Implementation Guidelines
1. Provide consistent interfaces across different encoding formats
2. Support both streaming and one-shot encoding/decoding operations
3. Implement proper tea handling with descriptive tea messages
4. Optimize for performance, especially for large data sets
5. Support squad tags for field mapping in all supported formats
6. Ensure thread safety for concurrent encoding/decoding operations