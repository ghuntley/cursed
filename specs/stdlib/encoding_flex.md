# EncodingFlex (encoding package)

## Overview
EncodingFlex provides interfaces and implementations for encoding and decoding data in various formats with flexible options. It's inspired by Go's encoding package and its subpackages but with enhanced flexibility and modern features.

## Core Interfaces

### `FlexEncoder`
Interface for encoding data.

```go
type FlexEncoder interface {
    Encode(v interface{}) ([]byte, error)
}
```

### `FlexDecoder`
Interface for decoding data.

```go
type FlexDecoder interface {
    Decode(data []byte, v interface{}) error
}
```

### `FlexCodec`
Combined encoder and decoder interface.

```go
type FlexCodec interface {
    FlexEncoder
    FlexDecoder
}
```

## JSON Encoding

### `JSONFlex`
JSON encoder/decoder with flexible options.

```go
type JSONOptions struct {
    PrettyPrint      bool
    EscapeHTML       bool
    AllowNaN         bool
    DisallowUnknown  bool
    CaseSensitive    bool
    EmitDefaults     bool
    Indent           string
    TagName          string
    OmitEmpty        bool
    FloatPrecision   int
    CamelCase        bool
    SnakeCase        bool
}

func NewJSONEncoder(opts *JSONOptions) FlexEncoder
func NewJSONDecoder(opts *JSONOptions) FlexDecoder
func NewJSONCodec(opts *JSONOptions) FlexCodec

// Convenience functions
func MarshalJSON(v interface{}, opts *JSONOptions) ([]byte, error)
func UnmarshalJSON(data []byte, v interface{}, opts *JSONOptions) error
```

## XML Encoding

### `XMLFlex`
XML encoder/decoder with flexible options.

```go
type XMLOptions struct {
    Indent        string
    Prefix        string
    NamespacesMap map[string]string
    OmitXMLHeader bool
    TagName       string
    CDataFields   []string
    AttributePrefix string
    OmitEmpty     bool
}

func NewXMLEncoder(opts *XMLOptions) FlexEncoder
func NewXMLDecoder(opts *XMLOptions) FlexDecoder
func NewXMLCodec(opts *XMLOptions) FlexCodec

// Convenience functions
func MarshalXML(v interface{}, opts *XMLOptions) ([]byte, error)
func UnmarshalXML(data []byte, v interface{}, opts *XMLOptions) error
```

## Base64 Encoding

### `Base64Flex`
Base64 encoding/decoding with different encodings.

```go
type Base64Encoding int

const (
    Base64Standard Base64Encoding = iota
    Base64URL
    Base64RawStandard
    Base64RawURL
)

func EncodeBase64(src []byte, encoding Base64Encoding) string
func DecodeBase64(s string, encoding Base64Encoding) ([]byte, error)
```

## Hex Encoding

### `HexFlex`
Hex encoding/decoding functions.

```go
func EncodeHex(src []byte) string
func DecodeHex(s string) ([]byte, error)
```

## CSV Encoding

### `CSVFlex`
CSV encoding/decoding with flexible options.

```go
type CSVOptions struct {
    Comma            rune
    Comment          rune
    FieldsPerRecord  int
    LazyQuotes       bool
    TrimLeadingSpace bool
    Headers          []string
    UseHeaders       bool
    StructTags       bool
}

type CSVReader struct {}

// Constructor
func NewCSVReader(r YeetIO.Yoink, opts *CSVOptions) *CSVReader

// Methods
func (r *CSVReader) Read() ([]string, error)
func (r *CSVReader) ReadAll() ([][]string, error)
func (r *CSVReader) ReadStruct(v interface{}) error
func (r *CSVReader) ReadAllStructs(slice interface{}) error
func (r *CSVReader) ReadMap() (map[string]string, error)
func (r *CSVReader) ReadAllMaps() ([]map[string]string, error)

type CSVWriter struct {}

// Constructor
func NewCSVWriter(w YeetIO.Yeeter, opts *CSVOptions) *CSVWriter

// Methods
func (w *CSVWriter) Write(record []string) error
func (w *CSVWriter) WriteAll(records [][]string) error
func (w *CSVWriter) WriteStruct(v interface{}) error
func (w *CSVWriter) WriteAllStructs(slice interface{}) error
func (w *CSVWriter) WriteMap(m map[string]string) error
func (w *CSVWriter) WriteAllMaps(maps []map[string]string) error
func (w *CSVWriter) Flush()
func (w *CSVWriter) Error() error
```

## GOB Encoding

### `GobFlex`
Gob encoding/decoding with enhanced features.

```go
func RegisterGobType(value interface{})

type GobEncoder struct {}

// Constructor
func NewGobEncoder(w YeetIO.Yeeter) *GobEncoder

// Methods
func (e *GobEncoder) Encode(v interface{}) error
func (e *GobEncoder) EncodeMultiple(values ...interface{}) error

type GobDecoder struct {}

// Constructor
func NewGobDecoder(r YeetIO.Yoink) *GobDecoder

// Methods
func (d *GobDecoder) Decode(v interface{}) error
func (d *GobDecoder) DecodeMultiple(values ...interface{}) error
```

## YAML Encoding

### `YAMLFlex`
YAML encoding/decoding with options.

```go
type YAMLOptions struct {
    IndentSize     int
    TagName        string
    OmitEmpty      bool
    UseJSONTags    bool
    DisallowUnknown bool
    FloatPrecision int
}

func NewYAMLEncoder(opts *YAMLOptions) FlexEncoder
func NewYAMLDecoder(opts *YAMLOptions) FlexDecoder
func NewYAMLCodec(opts *YAMLOptions) FlexCodec

// Convenience functions
func MarshalYAML(v interface{}, opts *YAMLOptions) ([]byte, error)
func UnmarshalYAML(data []byte, v interface{}, opts *YAMLOptions) error
```

## TOML Encoding

### `TOMLFlex`
TOML encoding/decoding with options.

```go
type TOMLOptions struct {
    IndentTables   bool
    ArraysMultiline bool
    TagName        string
    OmitEmpty      bool
    UseJSONTags    bool
}

func NewTOMLEncoder(opts *TOMLOptions) FlexEncoder
func NewTOMLDecoder(opts *TOMLOptions) FlexDecoder
func NewTOMLCodec(opts *TOMLOptions) FlexCodec

// Convenience functions
func MarshalTOML(v interface{}, opts *TOMLOptions) ([]byte, error)
func UnmarshalTOML(data []byte, v interface{}, opts *TOMLOptions) error
```

## Protocol Buffers

### `ProtoBufFlex`
Protocol Buffers encoding/decoding.

```go
type ProtoBufMessage interface {
    ProtoMessage()
    Reset()
    String() string
    ProtoReflect() protoreflect.Message
}

func MarshalProto(v ProtoBufMessage) ([]byte, error)
func UnmarshalProto(data []byte, v ProtoBufMessage) error
```

## Binary Encoding

### `BinaryFlex`
Binary encoding/decoding utilities.

```go
type ByteOrder int

const (
    BigEndian ByteOrder = iota
    LittleEndian
)

func ReadUint16(data []byte, order ByteOrder) uint16
func ReadUint32(data []byte, order ByteOrder) uint32
func ReadUint64(data []byte, order ByteOrder) uint64
func ReadInt16(data []byte, order ByteOrder) int16
func ReadInt32(data []byte, order ByteOrder) int32
func ReadInt64(data []byte, order ByteOrder) int64
func ReadFloat32(data []byte, order ByteOrder) float32
func ReadFloat64(data []byte, order ByteOrder) float64

func WriteUint16(v uint16, order ByteOrder) []byte
func WriteUint32(v uint32, order ByteOrder) []byte
func WriteUint64(v uint64, order ByteOrder) []byte
func WriteInt16(v int16, order ByteOrder) []byte
func WriteInt32(v int32, order ByteOrder) []byte
func WriteInt64(v int64, order ByteOrder) []byte
func WriteFloat32(v float32, order ByteOrder) []byte
func WriteFloat64(v float64, order ByteOrder) []byte
```

## ASCII85 Encoding

### `ASCII85Flex`
ASCII85 encoding/decoding functions.

```go
func EncodeASCII85(src []byte) []byte
func DecodeASCII85(src []byte) ([]byte, error)
```

## URI Encoding

### `URIFlex`
URI encoding/decoding functions.

```go
func EncodeURI(s string) string
func DecodeURI(s string) (string, error)
func EncodeURIComponent(s string) string
func DecodeURIComponent(s string) (string, error)
```

## Quoted-Printable Encoding

### `QPFlex`
Quoted-Printable encoding/decoding.

```go
func EncodeQuotedPrintable(src []byte) []byte
func DecodeQuotedPrintable(src []byte) ([]byte, error)
```

## Usage Example

```go
// JSON encoding with custom options
opts := &encoding_flex.JSONOptions{
    PrettyPrint: true,
    Indent:     "  ",
    OmitEmpty:   true,
}

type Person struct {
    Name  string `json:"name"`
    Age   int    `json:"age,omitempty"`
    Email string `json:"email"`
}

person := Person{Name: "Alice", Email: "alice@example.com"}

// Using convenience functions
jsonData, err := encoding_flex.MarshalJSON(person, opts)
if err != nil {
    vibez.spill("Error encoding JSON:", err)
    return
}

vibez.spill(string(jsonData))
// Output:
// {
//   "name": "Alice",
//   "email": "alice@example.com"
// }

// Using codec
jsonCodec := encoding_flex.NewJSONCodec(opts)
jsonData, err = jsonCodec.Encode(person)
if err != nil {
    vibez.spill("Error encoding JSON:", err)
    return
}

var decodedPerson Person
if err := jsonCodec.Decode(jsonData, &decodedPerson); err != nil {
    vibez.spill("Error decoding JSON:", err)
    return
}

vibez.spill(decodedPerson.Name) // "Alice"

// CSV encoding
csvOpts := &encoding_flex.CSVOptions{
    Headers:    []string{"Name", "Email"},
    UseHeaders: true,
}

buffer := &bytefit.FitBuffer{}
csv := encoding_flex.NewCSVWriter(buffer, csvOpts)

csv.WriteStruct(person)
csv.Flush()

vibez.spill(buffer.String())
// Output:
// Name,Email
// Alice,alice@example.com

// Base64 encoding
originalData := []byte("Hello, World!")
encoded := encoding_flex.EncodeBase64(originalData, encoding_flex.Base64Standard)
vibez.spill(encoded) // "SGVsbG8sIFdvcmxkIQ=="

decoded, err := encoding_flex.DecodeBase64(encoded, encoding_flex.Base64Standard)
if err != nil {
    vibez.spill("Error decoding Base64:", err)
    return
}

vibez.spill(string(decoded)) // "Hello, World!"

// YAML encoding
yamlOpts := &encoding_flex.YAMLOptions{
    IndentSize: 2,
    OmitEmpty:  true,
}

yamlData, err := encoding_flex.MarshalYAML(person, yamlOpts)
if err != nil {
    vibez.spill("Error encoding YAML:", err)
    return
}

vibez.spill(string(yamlData))
// Output:
// name: Alice
// email: alice@example.com
```

## Implementation Guidelines
1. Provide consistent interfaces across different encoding formats
2. Support both streaming and one-shot encoding/decoding operations
3. Implement proper error handling with descriptive error messages
4. Optimize for performance, especially for large data sets
5. Support struct tags for field mapping in all supported formats
6. Ensure thread safety for concurrent encoding/decoding operations