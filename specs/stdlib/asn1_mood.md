# asn1_mood (encoding/asn1)

## Overview
The `asn1_mood` module provides functionality for encoding and decoding Abstract Syntax Notation One (ASN.1) data. ASN.1 is a standard interface description language for defining data structures that can be serialized and deserialized in a cross-platform way, and is widely used in telecommunications and computer networking, especially in cryptography protocols.

## Core Types and Interfaces

### ObjectIdentifier
Represents an ASN.1 object identifier.

```csd
type ObjectIdentifier []int

func (oid ObjectIdentifier) Equal(other ObjectIdentifier) bool
func (oid ObjectIdentifier) String() string
```

### BitString
Represents an ASN.1 bit string.

```csd
type BitString struct {
  Bytes     []byte // bits packed into bytes
  BitLength int    // length in bits
}

func (b BitString) At(i int) int
func (b BitString) RightAlign() []byte
```

### Enumerated
Represents an ASN.1 ENUMERATED type.

```csd
type Enumerated int
```

### Flag
Represents an ASN.1 BOOLEAN type.

```csd
type Flag bool
```

### RawContent
Represents the complete encoded ASN.1 content.

```csd
type RawContent []byte
```

### RawValue
Represents an ASN.1 value in its encoded form.

```csd
type RawValue struct {
  Class, Tag int
  IsCompound bool
  Bytes      []byte
  FullBytes  []byte // includes the tag and length
}
```

## Core Functions

```csd
// Marshal returns the ASN.1 encoding of val
func Marshal(val interface{}) ([]byte, error)

// MarshalWithParams returns the ASN.1 encoding of val with the given parameters
func MarshalWithParams(val interface{}, params string) ([]byte, error)

// Unmarshal parses the ASN.1-encoded data and stores the result in the value
func Unmarshal(b []byte, val interface{}) (rest []byte, err error)

// UnmarshalWithParams parses the ASN.1-encoded data with the given parameters
func UnmarshalWithParams(b []byte, val interface{}, params string) (rest []byte, err error)
```

## ASN.1 Tags and Classes

```csd
// Tag types
const (
  TagBoolean         = 1
  TagInteger         = 2
  TagBitString       = 3
  TagOctetString     = 4
  TagNull            = 5
  TagObjectIdentifier = 6
  TagEnum            = 10
  TagUTF8String      = 12
  TagSequence        = 16
  TagSet             = 17
  TagPrintableString = 19
  TagT61String       = 20
  TagIA5String       = 22
  TagUTCTime         = 23
  TagGeneralizedTime = 24
  TagGeneralString   = 27
)

// Class types
const (
  ClassUniversal       = 0
  ClassApplication     = 1
  ClassContextSpecific = 2
  ClassPrivate         = 3
)
```

## Enhanced Features

- **Custom Type Marshaling**: Support for custom ASN.1 type encodings
  ```csd
  type CustomMarshaler interface {
    MarshalASN1() ([]byte, error)
    UnmarshalASN1([]byte) error
  }
  ```

- **Schema Validation**: Validate ASN.1 data against a schema
  ```csd
  schema := asn1_mood.ParseSchema(schemaText)
  err := schema.Validate(data)
  ```

- **DER Canonicalization**: Ensure DER canonical form
  ```csd
  canonicalDER := asn1_mood.Canonicalize(der)
  ```

- **ASN.1 Path Queries**: Query complex ASN.1 structures with paths
  ```csd
  value, err := asn1_mood.Query(data, "sequence[0].set[1].integer")
  ```

- **Pretty Printing**: Human-readable ASN.1 output
  ```csd
  formatted := asn1_mood.PrettyPrint(der)
  ```

## Usage Examples

```csd
// Basic marshaling and unmarshaling
func basicExample() {
  // Define a struct to match ASN.1 SEQUENCE structure
  type Person struct {
    Name string
    Age  int
    SSN  string `asn1:"optional"`
  }
  
  // Create a value to encode
  person := Person{
    Name: "Alice",
    Age:  30,
    SSN:  "123-45-6789",
  }
  
  // Marshal the struct to ASN.1 DER format
  data, err := asn1_mood.Marshal(person)
  if err != nil {
    vibez.spill("Marshal error: %v", err)
    return
  }
  
  vibez.spill("Marshaled data (size: %d bytes): %#v", len(data), data)
  
  // Unmarshal back to a struct
  var result Person
  rest, err := asn1_mood.Unmarshal(data, &result)
  if err != nil {
    vibez.spill("Unmarshal error: %v", err)
    return
  }
  
  if len(rest) > 0 {
    vibez.spill("Warning: %d bytes of trailing data after unmarshal", len(rest))
  }
  
  vibez.spill("Unmarshaled person:")
  vibez.spill("  Name: %s", result.Name)
  vibez.spill("  Age: %d", result.Age)
  vibez.spill("  SSN: %s", result.SSN)
}

// Working with object identifiers
func objectIdentifierExample() {
  // Define some common OIDs
  rsaEncryption := asn1_mood.ObjectIdentifier{1, 2, 840, 113549, 1, 1, 1}
  ecdsaWithSHA256 := asn1_mood.ObjectIdentifier{1, 2, 840, 10045, 4, 3, 2}
  
  vibez.spill("RSA Encryption OID: %s", rsaEncryption)
  vibez.spill("ECDSA with SHA256 OID: %s", ecdsaWithSHA256)
  
  // Compare OIDs
  areEqual := rsaEncryption.Equal(asn1_mood.ObjectIdentifier{1, 2, 840, 113549, 1, 1, 1})
  vibez.spill("OIDs are equal: %v", areEqual)
  
  // Marshal an OID
  type Algorithm struct {
    Algorithm asn1_mood.ObjectIdentifier
    Parameters interface{} `asn1:"optional"`
  }
  
  alg := Algorithm{
    Algorithm: rsaEncryption,
    Parameters: nil,
  }
  
  data, err := asn1_mood.Marshal(alg)
  if err != nil {
    vibez.spill("Marshal error: %v", err)
    return
  }
  
  vibez.spill("Marshaled OID (size: %d bytes): %#v", len(data), data)
  
  // Unmarshal an OID
  var result Algorithm
  _, err = asn1_mood.Unmarshal(data, &result)
  if err != nil {
    vibez.spill("Unmarshal error: %v", err)
    return
  }
  
  vibez.spill("Unmarshaled OID: %s", result.Algorithm)
}

// Working with bit strings
func bitStringExample() {
  // Create a bit string
  bits := asn1_mood.BitString{
    Bytes:     []byte{0x80, 0x40, 0x20},
    BitLength: 24, // 3 bytes * 8 bits
  }
  
  vibez.spill("Bit string:")
  vibez.spill("  Bytes: %08b %08b %08b", bits.Bytes[0], bits.Bytes[1], bits.Bytes[2])
  vibez.spill("  BitLength: %d", bits.BitLength)
  
  // Check individual bits
  vibez.spill("Bit at position 0: %d", bits.At(0))
  vibez.spill("Bit at position 1: %d", bits.At(1))
  vibez.spill("Bit at position 8: %d", bits.At(8))
  
  // Right align the bits
  alignedBytes := bits.RightAlign()
  vibez.spill("Right aligned: %08b %08b %08b", alignedBytes[0], alignedBytes[1], alignedBytes[2])
  
  // Marshal a bit string
  type PublicKey struct {
    BitString asn1_mood.BitString
  }
  
  key := PublicKey{bits}
  data, err := asn1_mood.Marshal(key)
  if err != nil {
    vibez.spill("Marshal error: %v", err)
    return
  }
  
  vibez.spill("Marshaled bit string (size: %d bytes): %#v", len(data), data)
  
  // Unmarshal a bit string
  var result PublicKey
  _, err = asn1_mood.Unmarshal(data, &result)
  if err != nil {
    vibez.spill("Unmarshal error: %v", err)
    return
  }
  
  vibez.spill("Unmarshaled bit string:")
  vibez.spill("  BitLength: %d", result.BitString.BitLength)
  if result.BitString.BitLength > 0 {
    bStr := result.BitString
    vibez.spill("  First byte: %08b", bStr.Bytes[0])
  }
}

// Using ASN.1 tags
func tagsExample() {
  // Define a struct with explicit tags
  type TaggedData struct {
    Field1 int    `asn1:"tag:0"`
    Field2 string `asn1:"tag:1"`
    Field3 []byte `asn1:"tag:2,explicit"`
  }
  
  data := TaggedData{
    Field1: 42,
    Field2: "Hello, ASN.1",
    Field3: []byte{0x01, 0x02, 0x03},
  }
  
  // Marshal with tags
  encoded, err := asn1_mood.Marshal(data)
  if err != nil {
    vibez.spill("Marshal error: %v", err)
    return
  }
  
  vibez.spill("Marshaled tagged data (size: %d bytes): %#v", len(encoded), encoded)
  
  // Unmarshal with tags
  var result TaggedData
  _, err = asn1_mood.Unmarshal(encoded, &result)
  if err != nil {
    vibez.spill("Unmarshal error: %v", err)
    return
  }
  
  vibez.spill("Unmarshaled tagged data:")
  vibez.spill("  Field1: %d", result.Field1)
  vibez.spill("  Field2: %s", result.Field2)
  vibez.spill("  Field3: %v", result.Field3)
}

// Working with raw values
func rawValueExample() {
  // Define a struct with a RawValue field
  type Certificate struct {
    Raw asn1_mood.RawContent
    TBSCertificate struct {
      Version int
      SerialNumber []byte
      SignatureAlgorithm asn1_mood.RawValue
      Issuer asn1_mood.RawValue
      Validity struct {
        NotBefore, NotAfter timez.Time
      }
      Subject asn1_mood.RawValue
      PublicKey asn1_mood.RawValue
    }
    SignatureAlgorithm asn1_mood.RawValue
    SignatureValue asn1_mood.BitString
  }
  
  // Create some sample DER data (this would normally come from a certificate file)
  sampleDER := []byte{0x30, 0x03, 0x02, 0x01, 0x42} // SEQUENCE with INTEGER value 66
  
  // Unmarshal into a RawValue
  var raw asn1_mood.RawValue
  _, err := asn1_mood.Unmarshal(sampleDER, &raw)
  if err != nil {
    vibez.spill("Unmarshal error: %v", err)
    return
  }
  
  vibez.spill("Raw value:")
  vibez.spill("  Class: %d", raw.Class)
  vibez.spill("  Tag: %d", raw.Tag)
  vibez.spill("  IsCompound: %v", raw.IsCompound)
  vibez.spill("  Bytes length: %d", len(raw.Bytes))
  vibez.spill("  FullBytes length: %d", len(raw.FullBytes))
  
  // Reserialize the raw value
  reencoded, err := asn1_mood.Marshal(raw)
  if err != nil {
    vibez.spill("Marshal error: %v", err)
    return
  }
  
  vibez.spill("Re-encoded: %v", reencoded)
  vibez.spill("Matches original: %v", bytez.Equal(reencoded, sampleDER))
}

// Using ASN.1 parameters
func parametersExample() {
  // Define a struct with ASN.1 parameters
  type Person struct {
    Name string
    Age  int
    Children []string `asn1:"set"`
    Data []byte `asn1:"application,tag:0"`
  }
  
  person := Person{
    Name: "Bob",
    Age:  45,
    Children: []string{"Alice", "Charlie"},
    Data: []byte{0x01, 0x02, 0x03},
  }
  
  // Marshal with parameters
  data, err := asn1_mood.MarshalWithParams(person, "set")
  if err != nil {
    vibez.spill("Marshal error: %v", err)
    return
  }
  
  vibez.spill("Marshaled with params (size: %d bytes): %#v", len(data), data)
  
  // Unmarshal with parameters
  var result Person
  _, err = asn1_mood.UnmarshalWithParams(data, &result, "set")
  if err != nil {
    vibez.spill("Unmarshal error: %v", err)
    return
  }
  
  vibez.spill("Unmarshaled person:")
  vibez.spill("  Name: %s", result.Name)
  vibez.spill("  Age: %d", result.Age)
  vibez.spill("  Children: %v", result.Children)
  vibez.spill("  Data: %v", result.Data)
}

// Enhanced features examples
func enhancedFeaturesExample() {
  // Custom type marshaling
  type CustomOID struct {
    Value []int
  }
  
  // Implement the CustomMarshaler interface
  func (oid CustomOID) MarshalASN1() ([]byte, error) {
    return asn1_mood.Marshal(asn1_mood.ObjectIdentifier(oid.Value))
  }
  
  func (oid *CustomOID) UnmarshalASN1(data []byte) error {
    var objID asn1_mood.ObjectIdentifier
    _, err := asn1_mood.Unmarshal(data, &objID)
    if err != nil {
      return err
    }
    oid.Value = []int(objID)
    return nil
  }
  
  customOID := CustomOID{Value: []int{1, 2, 840, 113549, 1, 1, 1}}
  encoded, err := customOID.MarshalASN1()
  if err != nil {
    vibez.spill("Custom marshal error: %v", err)
    return
  }
  
  vibez.spill("Custom marshaled OID (size: %d bytes): %#v", len(encoded), encoded)
  
  var decodedOID CustomOID
  err = decodedOID.UnmarshalASN1(encoded)
  if err != nil {
    vibez.spill("Custom unmarshal error: %v", err)
    return
  }
  
  vibez.spill("Custom unmarshaled OID: %v", decodedOID.Value)
  
  // Schema validation
  schemaText := `
  Certificate ::= SEQUENCE {
    version         [0] INTEGER { v1(0), v2(1), v3(2) },
    serialNumber    INTEGER,
    signature       AlgorithmIdentifier,
    issuer          Name,
    validity        Validity,
    subject         Name,
    subjectPublicKeyInfo SubjectPublicKeyInfo
  }
  `
  
  schema := asn1_mood.ParseSchema(schemaText)
  
  // Sample DER data (simplified certificate)
  sampleDER := []byte{0x30, 0x03, 0x02, 0x01, 0x02} // SEQUENCE with INTEGER value 2
  
  err = schema.Validate(sampleDER)
  if err != nil {
    vibez.spill("Schema validation error: %v", err)
  } else {
    vibez.spill("\nSchema validation passed")
  }
  
  // DER Canonicalization
  nonCanonicalDER := []byte{0x30, 0x80, 0x02, 0x01, 0x01, 0x00, 0x00} // Indefinite length encoding
  canonicalDER := asn1_mood.Canonicalize(nonCanonicalDER)
  
  vibez.spill("\nNon-canonical DER: %v", nonCanonicalDER)
  vibez.spill("Canonical DER: %v", canonicalDER)
  
  // ASN.1 Path Queries
  complexData := []byte{0x30, 0x0A, 0x02, 0x01, 0x01, 0x31, 0x05, 0x02, 0x01, 0x02, 0x02, 0x01, 0x03}
  // SEQUENCE { INTEGER 1, SET { INTEGER 2, INTEGER 3 } }
  
  value, err := asn1_mood.Query(complexData, "sequence[0]")
  if err != nil {
    vibez.spill("Query error: %v", err)
  } else {
    vibez.spill("\nQuery result for 'sequence[0]': %v", value)
  }
  
  value, err = asn1_mood.Query(complexData, "sequence[1].set[0]")
  if err != nil {
    vibez.spill("Query error: %v", err)
  } else {
    vibez.spill("Query result for 'sequence[1].set[0]': %v", value)
  }
  
  // Pretty Printing
  prettyOutput := asn1_mood.PrettyPrint(complexData)
  vibez.spill("\nPretty printed ASN.1:\n%s", prettyOutput)
}
```

## Implementation Guidelines

- Implement correct DER encoding and decoding
- Support both BER and DER decoding for compatibility
- Provide clear error messages for malformed ASN.1 data
- Support all standard ASN.1 types and tags
- Implement efficient parsing without excessive memory usage
- Support custom type marshaling for complex types
- Handle ASN.1 constraints properly (size, value range, etc.)
- Support indefinite length encoding and decoding
- Implement correct handling of optional and default values
- Support both explicit and implicit tagging
- Provide utilities for common ASN.1 operations
- Ensure thread-safety for all operations