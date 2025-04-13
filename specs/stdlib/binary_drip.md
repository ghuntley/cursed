# BinaryDrip (encoding/binary package)

## Overview
BinaryDrip provides utilities for encoding and decoding binary data with a smooth, efficient "drip" of bytes. It's inspired by Go's encoding/binary package but with enhanced features for complex data structures, streaming operations, and modern binary formats.

## Core Types

### Byte Order

```go
type ByteOrder interface {
    Uint16([]byte) uint16
    Uint32([]byte) uint32
    Uint64([]byte) uint64
    PutUint16([]byte, uint16)
    PutUint32([]byte, uint32)
    PutUint64([]byte, uint64)
    String() string
}

// Predefined byte orders
var BigEndian ByteOrder
var LittleEndian ByteOrder
var HostEndian ByteOrder // Native byte order of the current machine
var NetworkEndian ByteOrder // Alias for BigEndian
```

## Basic Functions

### Reading Functions

```go
// Read values from a byte slice
func ReadUint8(b []byte) uint8
func ReadUint16(b []byte, order ByteOrder) uint16
func ReadUint32(b []byte, order ByteOrder) uint32
func ReadUint64(b []byte, order ByteOrder) uint64
func ReadInt8(b []byte) int8
func ReadInt16(b []byte, order ByteOrder) int16
func ReadInt32(b []byte, order ByteOrder) int32
func ReadInt64(b []byte, order ByteOrder) int64
func ReadFloat32(b []byte, order ByteOrder) float32
func ReadFloat64(b []byte, order ByteOrder) float64
func ReadBool(b []byte) bool
func ReadString(b []byte) (string, int)
func ReadBytes(b []byte, size int) ([]byte, int)
```

### Writing Functions

```go
// Write values to a byte slice
func WriteUint8(b []byte, v uint8) int
func WriteUint16(b []byte, order ByteOrder, v uint16) int
func WriteUint32(b []byte, order ByteOrder, v uint32) int
func WriteUint64(b []byte, order ByteOrder, v uint64) int
func WriteInt8(b []byte, v int8) int
func WriteInt16(b []byte, order ByteOrder, v int16) int
func WriteInt32(b []byte, order ByteOrder, v int32) int
func WriteInt64(b []byte, order ByteOrder, v int64) int
func WriteFloat32(b []byte, order ByteOrder, v float32) int
func WriteFloat64(b []byte, order ByteOrder, v float64) int
func WriteBool(b []byte, v bool) int
func WriteString(b []byte, v string) int
func WriteBytes(b []byte, v []byte) int
```

### IO Reader/Writer Functions

```go
// Read values from a io.Reader
func ReadUvarint(r io.ByteReader) (uint64, error)
func ReadVarint(r io.ByteReader) (int64, error)

// Write values to a io.Writer
func WriteUvarint(w io.Writer, x uint64) (int, error)
func WriteVarint(w io.Writer, x int64) (int, error)

// Read from reader into data
func Read(r io.Reader, order ByteOrder, data interface{}) error

// Write data to writer
func Write(w io.Writer, order ByteOrder, data interface{}) error

// Size returns how many bytes Write would generate to encode the value v
func Size(v interface{}) int
```

## Enhanced Types

### `DripEncoder`
A flexible, chained encoder for binary data.

```go
type DripEncoder struct {
    // contains unexported fields
}

// Constructors
func NewDripEncoder(order ByteOrder) *DripEncoder
func NewDripEncoderWithBuffer(order ByteOrder, buffer []byte) *DripEncoder

// Encoding methods (chaining API)
func (e *DripEncoder) Uint8(v uint8) *DripEncoder
func (e *DripEncoder) Uint16(v uint16) *DripEncoder
func (e *DripEncoder) Uint32(v uint32) *DripEncoder
func (e *DripEncoder) Uint64(v uint64) *DripEncoder
func (e *DripEncoder) Int8(v int8) *DripEncoder
func (e *DripEncoder) Int16(v int16) *DripEncoder
func (e *DripEncoder) Int32(v int32) *DripEncoder
func (e *DripEncoder) Int64(v int64) *DripEncoder
func (e *DripEncoder) Float32(v float32) *DripEncoder
func (e *DripEncoder) Float64(v float64) *DripEncoder
func (e *DripEncoder) Bool(v bool) *DripEncoder
func (e *DripEncoder) String(v string) *DripEncoder
func (e *DripEncoder) Bytes(v []byte) *DripEncoder
func (e *DripEncoder) Varint(v int64) *DripEncoder
func (e *DripEncoder) Uvarint(v uint64) *DripEncoder
func (e *DripEncoder) Struct(v interface{}) *DripEncoder
func (e *DripEncoder) Array(v interface{}) *DripEncoder
func (e *DripEncoder) Map(v interface{}) *DripEncoder

// Output methods
func (e *DripEncoder) Bytes() []byte
func (e *DripEncoder) Size() int
func (e *DripEncoder) Reset()
func (e *DripEncoder) WriteTo(w io.Writer) (int64, error)

// Error handling
func (e *DripEncoder) Err() error
func (e *DripEncoder) SetError(err error) *DripEncoder
```

### `DripDecoder`
A flexible, chained decoder for binary data.

```go
type DripDecoder struct {
    // contains unexported fields
}

// Constructors
func NewDripDecoder(order ByteOrder, data []byte) *DripDecoder
func NewDripDecoderWithReader(order ByteOrder, r io.Reader) *DripDecoder

// Decoding methods (chaining API)
func (d *DripDecoder) Uint8() uint8
func (d *DripDecoder) Uint16() uint16
func (d *DripDecoder) Uint32() uint32
func (d *DripDecoder) Uint64() uint64
func (d *DripDecoder) Int8() int8
func (d *DripDecoder) Int16() int16
func (d *DripDecoder) Int32() int32
func (d *DripDecoder) Int64() int64
func (d *DripDecoder) Float32() float32
func (d *DripDecoder) Float64() float64
func (d *DripDecoder) Bool() bool
func (d *DripDecoder) String() string
func (d *DripDecoder) Bytes(size int) []byte
func (d *DripDecoder) Varint() int64
func (d *DripDecoder) Uvarint() uint64
func (d *DripDecoder) Struct(v interface{}) interface{}
func (d *DripDecoder) Array(v interface{}) interface{}
func (d *DripDecoder) Map(v interface{}) interface{}

// Navigation methods
func (d *DripDecoder) Skip(n int) *DripDecoder
func (d *DripDecoder) Position() int
func (d *DripDecoder) Seek(offset int, whence int) (int, error)
func (d *DripDecoder) Remaining() int
func (d *DripDecoder) Reset(data []byte)

// Error handling
func (d *DripDecoder) Err() error
func (d *DripDecoder) SetError(err error) *DripDecoder
```

### `BitDripper`
A bit-level encoder/decoder for precise bit manipulation.

```go
type BitDripper struct {
    // contains unexported fields
}

// Constructors
func NewBitDripper() *BitDripper
func NewBitDripperWithBuffer(buffer []byte) *BitDripper

// Bit writing methods
func (b *BitDripper) WriteBit(bit bool) *BitDripper
func (b *BitDripper) WriteBits(value uint64, bitCount int) *BitDripper
func (b *BitDripper) WriteSignedBits(value int64, bitCount int) *BitDripper
func (b *BitDripper) WriteUnary(value uint64) *BitDripper

// Bit reading methods
func (b *BitDripper) ReadBit() bool
func (b *BitDripper) ReadBits(bitCount int) uint64
func (b *BitDripper) ReadSignedBits(bitCount int) int64
func (b *BitDripper) ReadUnary() uint64

// Byte alignment and output
func (b *BitDripper) AlignToByte() *BitDripper
func (b *BitDripper) Bytes() []byte
func (b *BitDripper) BitPosition() int
func (b *BitDripper) BytePosition() int
func (b *BitDripper) Reset()
```

## Struct Tag Support

```go
// Struct field tags
// `binary:"name,type,size,endian,omitempty"`
// Example: `binary:"age,varint,omitempty"` or `binary:"data,bytes,10,big"`

// Custom tags
type CustomTags struct {
    TagName     string
    SizeTagName string
    EnumTagName string
}

// Set custom tags for binary encoding/decoding
func SetCustomTags(tags CustomTags)
```

## Binary Format Support

### Message Pack Format

```go
func EncodeMsgPack(v interface{}) ([]byte, error)
func DecodeMsgPack(data []byte, v interface{}) error

type MsgPackOptions struct {
    OmitEmpty           bool
    UseCompactEncoding bool
    SortMapKeys        bool
    CustomTypeHandlers map[reflect.Type]TypeHandler
}

func EncodeMsgPackWithOptions(v interface{}, opts MsgPackOptions) ([]byte, error)
func DecodeMsgPackWithOptions(data []byte, v interface{}, opts MsgPackOptions) error
```

### Protocol Buffers Support

```go
func EncodeProtobuf(v ProtobufMessage) ([]byte, error)
func DecodeProtobuf(data []byte, v ProtobufMessage) error

type ProtobufMessage interface {
    ProtoSize() int
    MarshalTo(data []byte) (n int, err error)
    Unmarshal(data []byte) error
}
```

### Cap'n Proto Support

```go
func EncodeCapnp(v CapnpMessage) ([]byte, error)
func DecodeCapnp(data []byte, v CapnpMessage) error

type CapnpMessage interface {
    MarshalCapnp() ([]byte, error)
    UnmarshalCapnp(data []byte) error
}
```

### FlatBuffers Support

```go
func EncodeFlatbuffer(v FlatBufferObject) ([]byte, error)
func DecodeFlatbuffer(data []byte, v FlatBufferObject) error

type FlatBufferObject interface {
    Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT
    Unpack(buf []byte) error
}
```

## Compact Encoding Functions

```go
// VarInt Functions - Variable length integer encoding
func EncodeVarInt(x int64) []byte
func DecodeVarInt(buf []byte) (int64, int)

// ZigZag encoding - Efficient encoding for signed integers
func EncodeZigZag(x int64) uint64
func DecodeZigZag(x uint64) int64

// Delta encoding - Encode differences between values
func EncodeDelta(values []int64) []byte
func DecodeDelta(buf []byte) []int64

// Run-length encoding - Compress repeated values
func EncodeRLE(data []byte) []byte
func DecodeRLE(data []byte) []byte
```

## Streaming Support

```go
type StreamEncoder struct {}

// Constructor
func NewStreamEncoder(w io.Writer, order ByteOrder) *StreamEncoder

// Methods
func (e *StreamEncoder) WriteValue(v interface{}) error
func (e *StreamEncoder) WriteUint8(v uint8) error
func (e *StreamEncoder) WriteUint16(v uint16) error
func (e *StreamEncoder) WriteUint32(v uint32) error
func (e *StreamEncoder) WriteUint64(v uint64) error
func (e *StreamEncoder) WriteInt8(v int8) error
func (e *StreamEncoder) WriteInt16(v int16) error
func (e *StreamEncoder) WriteInt32(v int32) error
func (e *StreamEncoder) WriteInt64(v int64) error
func (e *StreamEncoder) WriteFloat32(v float32) error
func (e *StreamEncoder) WriteFloat64(v float64) error
func (e *StreamEncoder) WriteBool(v bool) error
func (e *StreamEncoder) WriteString(v string) error
func (e *StreamEncoder) WriteBytes(v []byte) error
func (e *StreamEncoder) WriteVarInt(v int64) error
func (e *StreamEncoder) WriteUvarInt(v uint64) error
func (e *StreamEncoder) Flush() error

type StreamDecoder struct {}

// Constructor
func NewStreamDecoder(r io.Reader, order ByteOrder) *StreamDecoder

// Methods
func (d *StreamDecoder) ReadValue(v interface{}) error
func (d *StreamDecoder) ReadUint8() (uint8, error)
func (d *StreamDecoder) ReadUint16() (uint16, error)
func (d *StreamDecoder) ReadUint32() (uint32, error)
func (d *StreamDecoder) ReadUint64() (uint64, error)
func (d *StreamDecoder) ReadInt8() (int8, error)
func (d *StreamDecoder) ReadInt16() (int16, error)
func (d *StreamDecoder) ReadInt32() (int32, error)
func (d *StreamDecoder) ReadInt64() (int64, error)
func (d *StreamDecoder) ReadFloat32() (float32, error)
func (d *StreamDecoder) ReadFloat64() (float64, error)
func (d *StreamDecoder) ReadBool() (bool, error)
func (d *StreamDecoder) ReadString() (string, error)
func (d *StreamDecoder) ReadBytes(size int) ([]byte, error)
func (d *StreamDecoder) ReadVarInt() (int64, error)
func (d *StreamDecoder) ReadUvarInt() (uint64, error)
```

## Schema-Based Encoding

```go
type Schema struct {
    Fields []Field
}

type Field struct {
    Name     string
    Type     FieldType
    Size     int  // Fixed size or -1 for variable-length
    Optional bool
}

type FieldType int

const (
    TypeUint8 FieldType = iota
    TypeUint16
    TypeUint32
    TypeUint64
    TypeInt8
    TypeInt16
    TypeInt32
    TypeInt64
    TypeFloat32
    TypeFloat64
    TypeBool
    TypeString
    TypeBytes
    TypeVarInt
    TypeUVarInt
    TypeStruct
    TypeArray
    TypeMap
)

// Schema-based encoding/decoding
func EncodeWithSchema(data interface{}, schema *Schema) ([]byte, error)
func DecodeWithSchema(data []byte, schema *Schema) (map[string]interface{}, error)
```

## GenZ-Style Extensions

```go
// Ultra-compact encoding ("no cap" - no exaggeration, just the facts)
func NoCapEncode(v interface{}) []byte
func NoCapDecode(data []byte, v interface{}) error

// Aesthetic binary encoder with stylish output
func AestheticEncode(v interface{}) []byte
func AestheticDecode(data []byte, v interface{}) error

// Quick but effective binary conversion ("yeet" - fast throw)
func YeetToBinary(v interface{}) []byte
func YeetFromBinary(data []byte, v interface{}) error

// VibeCheck ensures data integrity with additional validation
func VibeCheck(data []byte) bool
func SetVibeCheckLevel(level int)
```

## Usage Examples

```go
// Basic encoding/decoding
data := make([]byte, 8)
binary_drip.WriteUint64(data, binary_drip.BigEndian, 12345678)
value := binary_drip.ReadUint64(data, binary_drip.BigEndian)
vibez.spill("Value:", value) // 12345678

// Using the fluent API encoder
encoder := binary_drip.NewDripEncoder(binary_drip.LittleEndian)
result := encoder.Uint32(42).
              String("hello").
              Float64(3.14159).
              Bool(true).
              Bytes()

// Decode the same data using the decoder
decoder := binary_drip.NewDripDecoder(binary_drip.LittleEndian, result)
uint32Value := decoder.Uint32()            // 42
stringValue := decoder.String()            // "hello"
float64Value := decoder.Float64()          // 3.14159
boolValue := decoder.Bool()                // true

vibez.spill("Decoded values:", uint32Value, stringValue, float64Value, boolValue)

// Reading and writing struct types with tags
type User struct {
    ID        uint64 `binary:"id,uvarint"`
    Name      string `binary:"name"`
    Age       int    `binary:"age,varint"`
    IsActive  bool   `binary:"active"`
    Scores    []int  `binary:"scores"`
    CreatedAt int64  `binary:"created,varint"`
}

user := User{
    ID:        1234,
    Name:      "Alice",
    Age:       28,
    IsActive:  true,
    Scores:    []int{95, 87, 92},
    CreatedAt: time.Now().Unix(),
}

// Encode the struct
encodedUser, err := binary_drip.Write(binary_drip.NewDripEncoder(binary_drip.LittleEndian).Struct(user).Bytes())
if err != nil {
    vibez.spill("Encoding error:", err)
    return
}

// Decode back into a struct
var decodedUser User
err = binary_drip.Read(binary_drip.NewDripDecoder(binary_drip.LittleEndian, encodedUser).Struct(&decodedUser))
if err != nil {
    vibez.spill("Decoding error:", err)
    return
}

vibez.spill("Decoded user:", decodedUser.Name, decodedUser.Age)

// Bit-level encoding
bitDripper := binary_drip.NewBitDripper()
bitDripper.WriteBit(true).
           WriteBits(5, 3).       // Use 3 bits to store value 5
           WriteSignedBits(-3, 5) // Use 5 bits to store value -3

bitResult := bitDripper.Bytes()

// Bit-level decoding
bitReader := binary_drip.NewBitDripper(bitResult)
bit := bitReader.ReadBit()             // true
bits := bitReader.ReadBits(3)          // 5
signedBits := bitReader.ReadSignedBits(5) // -3

vibez.spill("Decoded bits:", bit, bits, signedBits)

// Using stream encoder/decoder for large data
var buf bytes.Buffer
streamEncoder := binary_drip.NewStreamEncoder(&buf, binary_drip.BigEndian)

// Write a sequence of values
streamEncoder.WriteString("header")
for i := 0; i < 1000; i++ {
    streamEncoder.WriteVarInt(int64(i))
}
streamEncoder.WriteString("footer")

// Read with stream decoder
streamDecoder := binary_drip.NewStreamDecoder(&buf, binary_drip.BigEndian)
header, _ := streamDecoder.ReadString()
vibez.spill("Header:", header) // "header"

for i := 0; i < 1000; i++ {
    val, _ := streamDecoder.ReadVarInt()
    if i < 5 { // Just print the first few values
        vibez.spill("Value", i, ":", val)
    }
}

footer, _ := streamDecoder.ReadString()
vibez.spill("Footer:", footer) // "footer"

// Using GenZ style extensions
data = binary_drip.NoCapEncode(user)
vibez.spill("No cap encoded size:", len(data))

var noCapUser User
binary_drip.NoCapDecode(data, &noCapUser)
vibez.spill("No cap decoded name:", noCapUser.Name)

// Protocol Buffer compatibility
// (assuming a protobuf message type that implements ProtobufMessage)
protoData, _ := binary_drip.EncodeProtobuf(protoMsg)
vibez.spill("Protocol buffer size:", len(protoData))
binary_drip.DecodeProtobuf(protoData, &protoMsg)

// MessagePack format
msgpackData, _ := binary_drip.EncodeMsgPack(user)
vibez.spill("MessagePack size:", len(msgpackData))
binary_drip.DecodeMsgPack(msgpackData, &decodedUser)
```

## Implementation Guidelines
1. Optimize for performance with minimal allocations
2. Support arbitrary data structures with reflection
3. Provide detailed error messages for encoding/decoding failures
4. Implement efficient streaming operations for large datasets
5. Add safety checks for buffer overflows and underflows
6. Support cross-platform compatibility for all binary formats
7. Enable schema evolution for versioned binary formats
8. Include efficient implementations of common binary protocols