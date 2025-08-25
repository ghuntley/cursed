# BinaryDrip (encoding/binary package)

## Overview
BinaryDrip provides utilities for encoding and decoding binary data with a smooth, efficient "drip" of bytes. It's inspired by Go's encoding/binary package but with enhanced features for complex data squadures, streaming operations, and modern binary formats.

## Core Types

### Byte Order

```
be_like ByteOrder collab {
    Uint16([]byte) uint16
    Uint32([]byte) uint32
    Uint64([]byte) uint64
    PutUint16([]byte, uint16)
    PutUint32([]byte, uint32)
    PutUint64([]byte, uint64)
    String() tea
}

fr fr Predefined byte orders
var BigEndian ByteOrder
var LittleEndian ByteOrder
var HostEndian ByteOrder fr fr Native byte order of the current machine
var NetworkEndian ByteOrder fr fr Alias for BigEndian
```

## Basic Functions

### Reading Functions

```
fr fr Read values from a byte slice
slay ReadUint8(b []byte) uint8
slay ReadUint16(b []byte, order ByteOrder) uint16
slay ReadUint32(b []byte, order ByteOrder) uint32
slay ReadUint64(b []byte, order ByteOrder) uint64
slay ReadInt8(b []byte) int8
slay ReadInt16(b []byte, order ByteOrder) int16
slay ReadInt32(b []byte, order ByteOrder) int32
slay ReadInt64(b []byte, order ByteOrder) int64
slay ReadFloat32(b []byte, order ByteOrder) float32
slay ReadFloat64(b []byte, order ByteOrder) float64
slay ReadBool(b []byte) lit
slay ReadString(b []byte) (tea, normie)
slay ReadBytes(b []byte, size normie) ([]byte, normie)
```

### Writing Functions

```
fr fr Write values to a byte slice
slay WriteUint8(b []byte, v uint8) int
slay WriteUint16(b []byte, order ByteOrder, v uint16) int
slay WriteUint32(b []byte, order ByteOrder, v uint32) int
slay WriteUint64(b []byte, order ByteOrder, v uint64) int
slay WriteInt8(b []byte, v int8) int
slay WriteInt16(b []byte, order ByteOrder, v int16) int
slay WriteInt32(b []byte, order ByteOrder, v int32) int
slay WriteInt64(b []byte, order ByteOrder, v int64) int
slay WriteFloat32(b []byte, order ByteOrder, v float32) int
slay WriteFloat64(b []byte, order ByteOrder, v float64) int
slay WriteBool(b []byte, v lit) int
slay WriteString(b []byte, v tea) int
slay WriteBytes(b []byte, v []byte) int
```

### IO Reader/Writer Functions

```
fr fr Read values from a io.Reader
slay ReadUvarint(r io.ByteReader) (uint64, tea)
slay ReadVarint(r io.ByteReader) (int64, tea)

fr fr Write values to a io.Writer
slay WriteUvarint(w io.Writer, x uint64) (int, tea)
slay WriteVarint(w io.Writer, x int64) (int, tea)

fr fr Read from reader into data
slay Read(r io.Reader, order ByteOrder, data interface{}) tea

fr fr Write data to writer
slay Write(w io.Writer, order ByteOrder, data interface{}) tea

fr fr Size yolos how many bytes Write would generate to encode the value v
slay Size(v interface{}) int
```

## Enhanced Types

### `DripEncoder`
A flexible, chained encoder for binary data.

```
be_like DripEncoder squad {
    fr fr contains unexported fields
}

fr fr Consquadors
slay NewDripEncoder(order ByteOrder) *DripEncoder
slay NewDripEncoderWithBuffer(order ByteOrder, buffer []byte) *DripEncoder

fr fr Encoding methods (chaining API)
slay (e *DripEncoder) Uint8(v uint8) *DripEncoder
slay (e *DripEncoder) Uint16(v uint16) *DripEncoder
slay (e *DripEncoder) Uint32(v uint32) *DripEncoder
slay (e *DripEncoder) Uint64(v uint64) *DripEncoder
slay (e *DripEncoder) Int8(v int8) *DripEncoder
slay (e *DripEncoder) Int16(v int16) *DripEncoder
slay (e *DripEncoder) Int32(v int32) *DripEncoder
slay (e *DripEncoder) Int64(v int64) *DripEncoder
slay (e *DripEncoder) Float32(v float32) *DripEncoder
slay (e *DripEncoder) Float64(v float64) *DripEncoder
slay (e *DripEncoder) Bool(v lit) *DripEncoder
slay (e *DripEncoder) String(v tea) *DripEncoder
slay (e *DripEncoder) Bytes(v []byte) *DripEncoder
slay (e *DripEncoder) Varint(v int64) *DripEncoder
slay (e *DripEncoder) Uvarint(v uint64) *DripEncoder
slay (e *DripEncoder) Struct(v interface{}) *DripEncoder
slay (e *DripEncoder) Array(v interface{}) *DripEncoder
slay (e *DripEncoder) Map(v interface{}) *DripEncoder

fr fr Output methods
slay (e *DripEncoder) Bytes() []byte
slay (e *DripEncoder) Size() int
slay (e *DripEncoder) Reset()
slay (e *DripEncoder) WriteTo(w io.Writer) (int64, tea)

fr fr Error handling
slay (e *DripEncoder) Err() tea
slay (e *DripEncoder) SetError(err tea) *DripEncoder
```

### `DripDecoder`
A flexible, chained decoder for binary data.

```
be_like DripDecoder squad {
    fr fr contains unexported fields
}

fr fr Consquadors
slay NewDripDecoder(order ByteOrder, data []byte) *DripDecoder
slay NewDripDecoderWithReader(order ByteOrder, r io.Reader) *DripDecoder

fr fr Decoding methods (chaining API)
slay (d *DripDecoder) Uint8() uint8
slay (d *DripDecoder) Uint16() uint16
slay (d *DripDecoder) Uint32() uint32
slay (d *DripDecoder) Uint64() uint64
slay (d *DripDecoder) Int8() int8
slay (d *DripDecoder) Int16() int16
slay (d *DripDecoder) Int32() int32
slay (d *DripDecoder) Int64() int64
slay (d *DripDecoder) Float32() float32
slay (d *DripDecoder) Float64() float64
slay (d *DripDecoder) Bool() lit
slay (d *DripDecoder) String() tea
slay (d *DripDecoder) Bytes(size normie) []byte
slay (d *DripDecoder) Varint() int64
slay (d *DripDecoder) Uvarint() uint64
slay (d *DripDecoder) Struct(v interface{}) interface{}
slay (d *DripDecoder) Array(v interface{}) interface{}
slay (d *DripDecoder) Map(v interface{}) interface{}

fr fr Navigation methods
slay (d *DripDecoder) Skip(n normie) *DripDecoder
slay (d *DripDecoder) Position() int
slay (d *DripDecoder) Seek(offset int, whence normie) (int, tea)
slay (d *DripDecoder) Remaining() int
slay (d *DripDecoder) Reset(data []byte)

fr fr Error handling
slay (d *DripDecoder) Err() tea
slay (d *DripDecoder) SetError(err tea) *DripDecoder
```

### `BitDripper`
A bit-level encoder/decoder for precise bit manipulation.

```
be_like BitDripper squad {
    fr fr contains unexported fields
}

fr fr Consquadors
slay NewBitDripper() *BitDripper
slay NewBitDripperWithBuffer(buffer []byte) *BitDripper

fr fr Bit writing methods
slay (b *BitDripper) WriteBit(bit lit) *BitDripper
slay (b *BitDripper) WriteBits(value uint64, bitCount normie) *BitDripper
slay (b *BitDripper) WriteSignedBits(value int64, bitCount normie) *BitDripper
slay (b *BitDripper) WriteUnary(value uint64) *BitDripper

fr fr Bit reading methods
slay (b *BitDripper) ReadBit() lit
slay (b *BitDripper) ReadBits(bitCount normie) uint64
slay (b *BitDripper) ReadSignedBits(bitCount normie) int64
slay (b *BitDripper) ReadUnary() uint64

fr fr Byte alignment and output
slay (b *BitDripper) AlignToByte() *BitDripper
slay (b *BitDripper) Bytes() []byte
slay (b *BitDripper) BitPosition() int
slay (b *BitDripper) BytePosition() int
slay (b *BitDripper) Reset()
```

## Struct Tag Support

```
fr fr Struct field tags
fr fr `binary:"name,type,size,endian,omitempty"`
fr fr Example: `binary:"age,varint,omitempty"` or `binary:"data,bytes,10,big"`

fr fr Custom tags
be_like CustomTags squad {
    TagName     tea
    SizeTagName tea
    EnumTagName tea
}

fr fr Set custom tags for binary encoding/decoding
slay SetCustomTags(tags CustomTags)
```

## Binary Format Support

### Message Pack Format

```
slay EncodeMsgPack(v interface{}) ([]byte, tea)
slay DecodeMsgPack(data []byte, v interface{}) tea

be_like MsgPackOptions squad {
    OmitEmpty           lit
    UseCompactEncoding lit
    SortMapKeys        lit
    CustomTypeHandlers map[reflect.Type]TypeHandler
}

slay EncodeMsgPackWithOptions(v interface{}, opts MsgPackOptions) ([]byte, tea)
slay DecodeMsgPackWithOptions(data []byte, v interface{}, opts MsgPackOptions) tea
```

### Protocol Buffers Support

```
slay EncodeProtobuf(v ProtobufMessage) ([]byte, tea)
slay DecodeProtobuf(data []byte, v ProtobufMessage) tea

be_like ProtobufMessage collab {
    ProtoSize() int
    MarshalTo(data []byte) (n int, err tea)
    Unmarshal(data []byte) tea
}
```

### Cap'n Proto Support

```
slay EncodeCapnp(v CapnpMessage) ([]byte, tea)
slay DecodeCapnp(data []byte, v CapnpMessage) tea

be_like CapnpMessage collab {
    MarshalCapnp() ([]byte, tea)
    UnmarshalCapnp(data []byte) tea
}
```

### FlatBuffers Support

```
slay EncodeFlatbuffer(v FlatBufferObject) ([]byte, tea)
slay DecodeFlatbuffer(data []byte, v FlatBufferObject) tea

be_like FlatBufferObject collab {
    Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT
    Unpack(buf []byte) tea
}
```

## Compact Encoding Functions

```
fr fr VarInt Functions - Variable length integer encoding
slay EncodeVarInt(x int64) []byte
slay DecodeVarInt(buf []byte) (int64, normie)

fr fr ZigZag encoding - Efficient encoding for signed integers
slay EncodeZigZag(x int64) uint64
slay DecodeZigZag(x uint64) int64

fr fr Delta encoding - Encode differences between values
slay EncodeDelta(values []int64) []byte
slay DecodeDelta(buf []byte) []int64

fr fr Run-length encoding - Compress repeated values
slay EncodeRLE(data []byte) []byte
slay DecodeRLE(data []byte) []byte
```

## Streaming Support

```
be_like StreamEncoder squad {}

fr fr Consquador
slay NewStreamEncoder(w io.Writer, order ByteOrder) *StreamEncoder

fr fr Methods
slay (e *StreamEncoder) WriteValue(v interface{}) tea
slay (e *StreamEncoder) WriteUint8(v uint8) tea
slay (e *StreamEncoder) WriteUint16(v uint16) tea
slay (e *StreamEncoder) WriteUint32(v uint32) tea
slay (e *StreamEncoder) WriteUint64(v uint64) tea
slay (e *StreamEncoder) WriteInt8(v int8) tea
slay (e *StreamEncoder) WriteInt16(v int16) tea
slay (e *StreamEncoder) WriteInt32(v int32) tea
slay (e *StreamEncoder) WriteInt64(v int64) tea
slay (e *StreamEncoder) WriteFloat32(v float32) tea
slay (e *StreamEncoder) WriteFloat64(v float64) tea
slay (e *StreamEncoder) WriteBool(v lit) tea
slay (e *StreamEncoder) WriteString(v tea) tea
slay (e *StreamEncoder) WriteBytes(v []byte) tea
slay (e *StreamEncoder) WriteVarInt(v int64) tea
slay (e *StreamEncoder) WriteUvarInt(v uint64) tea
slay (e *StreamEncoder) Flush() tea

be_like StreamDecoder squad {}

fr fr Consquador
slay NewStreamDecoder(r io.Reader, order ByteOrder) *StreamDecoder

fr fr Methods
slay (d *StreamDecoder) ReadValue(v interface{}) tea
slay (d *StreamDecoder) ReadUint8() (uint8, tea)
slay (d *StreamDecoder) ReadUint16() (uint16, tea)
slay (d *StreamDecoder) ReadUint32() (uint32, tea)
slay (d *StreamDecoder) ReadUint64() (uint64, tea)
slay (d *StreamDecoder) ReadInt8() (int8, tea)
slay (d *StreamDecoder) ReadInt16() (int16, tea)
slay (d *StreamDecoder) ReadInt32() (int32, tea)
slay (d *StreamDecoder) ReadInt64() (int64, tea)
slay (d *StreamDecoder) ReadFloat32() (float32, tea)
slay (d *StreamDecoder) ReadFloat64() (float64, tea)
slay (d *StreamDecoder) ReadBool() (lit, tea)
slay (d *StreamDecoder) ReadString() (tea, tea)
slay (d *StreamDecoder) ReadBytes(size normie) ([]byte, tea)
slay (d *StreamDecoder) ReadVarInt() (int64, tea)
slay (d *StreamDecoder) ReadUvarInt() (uint64, tea)
```

## Schema-Based Encoding

```
be_like Schema squad {
    Fields []Field
}

be_like Field squad {
    Name     tea
    Type     FieldType
    Size     normie  fr fr Fixed size or -1 for variable-length
    Optional lit
}

be_like FieldType int

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

fr fr Schema-based encoding/decoding
slay EncodeWithSchema(data interface{}, schema *Schema) ([]byte, tea)
slay DecodeWithSchema(data []byte, schema *Schema) (map[tea]interface{}, tea)
```

## GenZ-Style Extensions

```
fr fr Ultra-compact encoding ("no cap" - no exaggeration, just the facts)
slay NoCapEncode(v interface{}) []byte
slay NoCapDecode(data []byte, v interface{}) tea

fr fr Aesthetic binary encoder with stylish output
slay AestheticEncode(v interface{}) []byte
slay AestheticDecode(data []byte, v interface{}) tea

fr fr Quick but effective binary conversion ("yeet" - fast throw)
slay YeetToBinary(v interface{}) []byte
slay YeetFromBinary(data []byte, v interface{}) tea

fr fr VibeCheck ensures data integrity with additional validation
slay VibeCheck(data []byte) lit
slay SetVibeCheckLevel(level normie)
```

## Usage Examples

```
fr fr Basic encoding/decoding
data := make([]byte, 8)
binary_drip.WriteUint64(data, binary_drip.BigEndian, 12345678)
value := binary_drip.ReadUint64(data, binary_drip.BigEndian)
vibez.spill("Value:", value) fr fr 12345678

fr fr Using the fluent API encoder
encoder := binary_drip.NewDripEncoder(binary_drip.LittleEndian)
result := encoder.Uint32(42).
              String("hello").
              Float64(3.14159).
              Bool(based).
              Bytes()

fr fr Decode the same data using the decoder
decoder := binary_drip.NewDripDecoder(binary_drip.LittleEndian, result)
uint32Value := decoder.Uint32()            fr fr 42
teaValue := decoder.String()            fr fr "hello"
float64Value := decoder.Float64()          fr fr 3.14159
litValue := decoder.Bool()                fr fr based

vibez.spill("Decoded values:", uint32Value, teaValue, float64Value, litValue)

fr fr Reading and writing squad types with tags
be_like User squad {
    ID        uint64 `binary:"id,uvarint"`
    Name      tea `binary:"name"`
    Age       normie    `binary:"age,varint"`
    IsActive  lit   `binary:"active"`
    Scores    []normie  `binary:"scores"`
    CreatedAt int64  `binary:"created,varint"`
}

user := User{
    ID:        1234,
    Name:      "Alice",
    Age:       28,
    IsActive:  based,
    Scores:    []int{95, 87, 92},
    CreatedAt: time.Now().Unix(),
}

fr fr Encode the squad
encodedUser, err := binary_drip.Write(binary_drip.NewDripEncoder(binary_drip.LittleEndian).Struct(user).Bytes())
if err != nah {
    vibez.spill("Encoding tea:", err)
    yolo
}

fr fr Decode back into a squad
var decodedUser User
err = binary_drip.Read(binary_drip.NewDripDecoder(binary_drip.LittleEndian, encodedUser).Struct(&decodedUser))
if err != nah {
    vibez.spill("Decoding tea:", err)
    yolo
}

vibez.spill("Decoded user:", decodedUser.Name, decodedUser.Age)

fr fr Bit-level encoding
bitDripper := binary_drip.NewBitDripper()
bitDripper.WriteBit(based).
           WriteBits(5, 3).       fr fr Use 3 bits to store value 5
           WriteSignedBits(-3, 5) fr fr Use 5 bits to store value -3

bitResult := bitDripper.Bytes()

fr fr Bit-level decoding
bitReader := binary_drip.NewBitDripper(bitResult)
bit := bitReader.ReadBit()             fr fr based
bits := bitReader.ReadBits(3)          fr fr 5
signedBits := bitReader.ReadSignedBits(5) fr fr -3

vibez.spill("Decoded bits:", bit, bits, signedBits)

fr fr Using stream encoder/decoder for large data
var buf bytes.Buffer
streamEncoder := binary_drip.NewStreamEncoder(&buf, binary_drip.BigEndian)

fr fr Write a sequence of values
streamEncoder.WriteString("header")
for i := 0; i < 1000; i++ {
    streamEncoder.WriteVarInt(int64(i))
}
streamEncoder.WriteString("footer")

fr fr Read with stream decoder
streamDecoder := binary_drip.NewStreamDecoder(&buf, binary_drip.BigEndian)
header, _ := streamDecoder.ReadString()
vibez.spill("Header:", header) fr fr "header"

for i := 0; i < 1000; i++ {
    val, _ := streamDecoder.ReadVarInt()
    if i < 5 { fr fr Just prnormie the first few values
        vibez.spill("Value", i, ":", val)
    }
}

footer, _ := streamDecoder.ReadString()
vibez.spill("Footer:", footer) fr fr "footer"

fr fr Using GenZ style extensions
data = binary_drip.NoCapEncode(user)
vibez.spill("No cap encoded size:", len(data))

var noCapUser User
binary_drip.NoCapDecode(data, &noCapUser)
vibez.spill("No cap decoded name:", noCapUser.Name)

fr fr Protocol Buffer compatibility
fr fr (assuming a protobuf message be_like that implements ProtobufMessage)
protoData, _ := binary_drip.EncodeProtobuf(protoMsg)
vibez.spill("Protocol buffer size:", len(protoData))
binary_drip.DecodeProtobuf(protoData, &protoMsg)

fr fr MessagePack format
msgpackData, _ := binary_drip.EncodeMsgPack(user)
vibez.spill("MessagePack size:", len(msgpackData))
binary_drip.DecodeMsgPack(msgpackData, &decodedUser)
```

## Implementation Guidelines
1. Optimize for performance with minimal allocations
2. Support arbitrary data squadures with reflection
3. Provide detailed tea messages for encoding/decoding failures
4. Implement efficient streaming operations for large datasets
5. Add safety checks for buffer overflows and underflows
6. Support cross-platform compatibility for all binary formats
7. Enable schema evolution for versioned binary formats
8. Include efficient implementations of common binary protocols