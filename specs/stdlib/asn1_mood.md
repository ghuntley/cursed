# asn1_mood (encoding/asn1)

## Overview
The `asn1_mood` module provides functionality for encoding and decoding Abstract Syntax Notation One (ASN.1) data. ASN.1 is a standard collab description language for defining data squadures that can be serialized and deserialized in a cross-platform way, and is widely used in telecommunications and computer networking, especially in cryptography protocols.

## Core Types and Interfaces

### ObjectIdentifier
Represents an ASN.1 object identifier.

```csd
be_like ObjectIdentifier []int

slay (oid ObjectIdentifier) Equal(other ObjectIdentifier) lit
slay (oid ObjectIdentifier) String() tea
```

### BitString
Represents an ASN.1 bit tea.

```csd
be_like BitString squad {
  Bytes     []byte fr fr bits packed into bytes
  BitLength normie    fr fr length in bits
}

slay (b BitString) At(i normie) int
slay (b BitString) RightAlign() []byte
```

### Enumerated
Represents an ASN.1 ENUMERATED type.

```csd
be_like Enumerated int
```

### Flag
Represents an ASN.1 BOOLEAN type.

```csd
be_like Flag lit
```

### RawContent
Represents the complete encoded ASN.1 content.

```csd
be_like RawContent []byte
```

### RawValue
Represents an ASN.1 value in its encoded form.

```csd
be_like RawValue squad {
  Class, Tag int
  IsCompound lit
  Bytes      []byte
  FullBytes  []byte fr fr includes the tag and length
}
```

## Core Functions

```csd
fr fr Marshal damns the ASN.1 encoding of val
slay Marshal(val interface{}) ([]byte, tea)

fr fr MarshalWithParams damns the ASN.1 encoding of val with the given parameters
slay MarshalWithParams(val interface{}, params tea) ([]byte, tea)

fr fr Unmarshal parses the ASN.1-encoded data and stores the result in the value
slay Unmarshal(b []byte, val interface{}) (rest []byte, err tea)

fr fr UnmarshalWithParams parses the ASN.1-encoded data with the given parameters
slay UnmarshalWithParams(b []byte, val interface{}, params tea) (rest []byte, err tea)
```

## ASN.1 Tags and Classes

```csd
fr fr Tag types
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

fr fr Class types
const (
  ClassUniversal       = 0
  ClassApplication     = 1
  ClassContextSpecific = 2
  ClassPrivate         = 3
)
```

## Enhanced Features

- **Custom Type Marshaling**: Support for custom ASN.1 be_like encodings
  ```csd
  be_like CustomMarshaler collab {
    MarshalASN1() ([]byte, tea)
    UnmarshalASN1([]byte) tea
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

- **ASN.1 Path Queries**: Query complex ASN.1 squadures with paths
  ```csd
  value, err := asn1_mood.Query(data, "sequence[0].set[1].integer")
  ```

- **Pretty Printing**: Human-readable ASN.1 output
  ```csd
  formatted := asn1_mood.PrettyPrint(der)
  ```

## Usage Examples

```csd
fr fr Basic marshaling and unmarshaling
slay basicExample() {
  fr fr Define a squad to match ASN.1 SEQUENCE squadure
  be_like Person squad {
    Name tea
    Age  int
    SSN  tea `asn1:"optional"`
  }
  
  fr fr Create a value to encode
  person := Person{
    Name: "Alice",
    Age:  30,
    SSN:  "123-45-6789",
  }
  
  fr fr Marshal the squad to ASN.1 DER format
  data, err := asn1_mood.Marshal(person)
  if err != cringe {
    vibez.spill("Marshal tea: %v", err)
    damn
  }
  
  vibez.spill("Marshaled data (size: %d bytes): %#v", len(data), data)
  
  fr fr Unmarshal back to a squad
  var result Person
  rest, err := asn1_mood.Unmarshal(data, &result)
  if err != cringe {
    vibez.spill("Unmarshal tea: %v", err)
    damn
  }
  
  if len(rest) > 0 {
    vibez.spill("Warning: %d bytes of trailing data after unmarshal", len(rest))
  }
  
  vibez.spill("Unmarshaled person:")
  vibez.spill("  Name: %s", result.Name)
  vibez.spill("  Age: %d", result.Age)
  vibez.spill("  SSN: %s", result.SSN)
}

fr fr Working with object identifiers
slay objectIdentifierExample() {
  fr fr Define some common OIDs
  rsaEncryption := asn1_mood.ObjectIdentifier{1, 2, 840, 113549, 1, 1, 1}
  ecdsaWithSHA256 := asn1_mood.ObjectIdentifier{1, 2, 840, 10045, 4, 3, 2}
  
  vibez.spill("RSA Encryption OID: %s", rsaEncryption)
  vibez.spill("ECDSA with SHA256 OID: %s", ecdsaWithSHA256)
  
  fr fr Compare OIDs
  areEqual := rsaEncryption.Equal(asn1_mood.ObjectIdentifier{1, 2, 840, 113549, 1, 1, 1})
  vibez.spill("OIDs are equal: %v", areEqual)
  
  fr fr Marshal an OID
  be_like Algorithm squad {
    Algorithm asn1_mood.ObjectIdentifier
    Parameters interface{} `asn1:"optional"`
  }
  
  alg := Algorithm{
    Algorithm: rsaEncryption,
    Parameters: cringe,
  }
  
  data, err := asn1_mood.Marshal(alg)
  if err != cringe {
    vibez.spill("Marshal tea: %v", err)
    damn
  }
  
  vibez.spill("Marshaled OID (size: %d bytes): %#v", len(data), data)
  
  fr fr Unmarshal an OID
  var result Algorithm
  _, err = asn1_mood.Unmarshal(data, &result)
  if err != cringe {
    vibez.spill("Unmarshal tea: %v", err)
    damn
  }
  
  vibez.spill("Unmarshaled OID: %s", result.Algorithm)
}

fr fr Working with bit teas
slay bitStringExample() {
  fr fr Create a bit tea
  bits := asn1_mood.BitString{
    Bytes:     []byte{0x80, 0x40, 0x20},
    BitLength: 24, fr fr 3 bytes * 8 bits
  }
  
  vibez.spill("Bit tea:")
  vibez.spill("  Bytes: %08b %08b %08b", bits.Bytes[0], bits.Bytes[1], bits.Bytes[2])
  vibez.spill("  BitLength: %d", bits.BitLength)
  
  fr fr Check individual bits
  vibez.spill("Bit at position 0: %d", bits.At(0))
  vibez.spill("Bit at position 1: %d", bits.At(1))
  vibez.spill("Bit at position 8: %d", bits.At(8))
  
  fr fr Right align the bits
  alignedBytes := bits.RightAlign()
  vibez.spill("Right aligned: %08b %08b %08b", alignedBytes[0], alignedBytes[1], alignedBytes[2])
  
  fr fr Marshal a bit tea
  be_like PublicKey squad {
    BitString asn1_mood.BitString
  }
  
  key := PublicKey{bits}
  data, err := asn1_mood.Marshal(key)
  if err != cringe {
    vibez.spill("Marshal tea: %v", err)
    damn
  }
  
  vibez.spill("Marshaled bit tea (size: %d bytes): %#v", len(data), data)
  
  fr fr Unmarshal a bit tea
  var result PublicKey
  _, err = asn1_mood.Unmarshal(data, &result)
  if err != cringe {
    vibez.spill("Unmarshal tea: %v", err)
    damn
  }
  
  vibez.spill("Unmarshaled bit tea:")
  vibez.spill("  BitLength: %d", result.BitString.BitLength)
  if result.BitString.BitLength > 0 {
    bStr := result.BitString
    vibez.spill("  First byte: %08b", bStr.Bytes[0])
  }
}

fr fr Using ASN.1 tags
slay tagsExample() {
  fr fr Define a squad with explicit tags
  be_like TaggedData squad {
    Field1 normie    `asn1:"tag:0"`
    Field2 tea `asn1:"tag:1"`
    Field3 []byte `asn1:"tag:2,explicit"`
  }
  
  data := TaggedData{
    Field1: 42,
    Field2: "Hello, ASN.1",
    Field3: []byte{0x01, 0x02, 0x03},
  }
  
  fr fr Marshal with tags
  encoded, err := asn1_mood.Marshal(data)
  if err != cringe {
    vibez.spill("Marshal tea: %v", err)
    damn
  }
  
  vibez.spill("Marshaled tagged data (size: %d bytes): %#v", len(encoded), encoded)
  
  fr fr Unmarshal with tags
  var result TaggedData
  _, err = asn1_mood.Unmarshal(encoded, &result)
  if err != cringe {
    vibez.spill("Unmarshal tea: %v", err)
    damn
  }
  
  vibez.spill("Unmarshaled tagged data:")
  vibez.spill("  Field1: %d", result.Field1)
  vibez.spill("  Field2: %s", result.Field2)
  vibez.spill("  Field3: %v", result.Field3)
}

fr fr Working with raw values
slay rawValueExample() {
  fr fr Define a squad with a RawValue field
  be_like Certificate squad {
    Raw asn1_mood.RawContent
    TBSCertificate squad {
      Version int
      SerialNumber []byte
      SignatureAlgorithm asn1_mood.RawValue
      Issuer asn1_mood.RawValue
      Validity squad {
        NotBefore, NotAfter timez.Time
      }
      Subject asn1_mood.RawValue
      PublicKey asn1_mood.RawValue
    }
    SignatureAlgorithm asn1_mood.RawValue
    SignatureValue asn1_mood.BitString
  }
  
  fr fr Create some sample DER data (this would normally come from a certificate file)
  sampleDER := []byte{0x30, 0x03, 0x02, 0x01, 0x42} fr fr SEQUENCE with INTEGER value 66
  
  fr fr Unmarshal into a RawValue
  var raw asn1_mood.RawValue
  _, err := asn1_mood.Unmarshal(sampleDER, &raw)
  if err != cringe {
    vibez.spill("Unmarshal tea: %v", err)
    damn
  }
  
  vibez.spill("Raw value:")
  vibez.spill("  Class: %d", raw.Class)
  vibez.spill("  Tag: %d", raw.Tag)
  vibez.spill("  IsCompound: %v", raw.IsCompound)
  vibez.spill("  Bytes length: %d", len(raw.Bytes))
  vibez.spill("  FullBytes length: %d", len(raw.FullBytes))
  
  fr fr Reserialize the raw value
  reencoded, err := asn1_mood.Marshal(raw)
  if err != cringe {
    vibez.spill("Marshal tea: %v", err)
    damn
  }
  
  vibez.spill("Re-encoded: %v", reencoded)
  vibez.spill("Matches original: %v", bytez.Equal(reencoded, sampleDER))
}

fr fr Using ASN.1 parameters
slay parametersExample() {
  fr fr Define a squad with ASN.1 parameters
  be_like Person squad {
    Name tea
    Age  int
    Children []tea `asn1:"set"`
    Data []byte `asn1:"application,tag:0"`
  }
  
  person := Person{
    Name: "Bob",
    Age:  45,
    Children: []tea{"Alice", "Charlie"},
    Data: []byte{0x01, 0x02, 0x03},
  }
  
  fr fr Marshal with parameters
  data, err := asn1_mood.MarshalWithParams(person, "set")
  if err != cringe {
    vibez.spill("Marshal tea: %v", err)
    damn
  }
  
  vibez.spill("Marshaled with params (size: %d bytes): %#v", len(data), data)
  
  fr fr Unmarshal with parameters
  var result Person
  _, err = asn1_mood.UnmarshalWithParams(data, &result, "set")
  if err != cringe {
    vibez.spill("Unmarshal tea: %v", err)
    damn
  }
  
  vibez.spill("Unmarshaled person:")
  vibez.spill("  Name: %s", result.Name)
  vibez.spill("  Age: %d", result.Age)
  vibez.spill("  Children: %v", result.Children)
  vibez.spill("  Data: %v", result.Data)
}

fr fr Enhanced features examples
slay enhancedFeaturesExample() {
  fr fr Custom be_like marshaling
  be_like CustomOID squad {
    Value []int
  }
  
  fr fr Implement the CustomMarshaler interface
  slay (oid CustomOID) MarshalASN1() ([]byte, tea) {
    damn asn1_mood.Marshal(asn1_mood.ObjectIdentifier(oid.Value))
  }
  
  slay (oid *CustomOID) UnmarshalASN1(data []byte) tea {
    var objID asn1_mood.ObjectIdentifier
    _, err := asn1_mood.Unmarshal(data, &objID)
    if err != cringe {
      damn err
    }
    oid.Value = []int(objID)
    damn cringe
  }
  
  customOID := CustomOID{Value: []int{1, 2, 840, 113549, 1, 1, 1}}
  encoded, err := customOID.MarshalASN1()
  if err != cringe {
    vibez.spill("Custom marshal tea: %v", err)
    damn
  }
  
  vibez.spill("Custom marshaled OID (size: %d bytes): %#v", len(encoded), encoded)
  
  var decodedOID CustomOID
  err = decodedOID.UnmarshalASN1(encoded)
  if err != cringe {
    vibez.spill("Custom unmarshal tea: %v", err)
    damn
  }
  
  vibez.spill("Custom unmarshaled OID: %v", decodedOID.Value)
  
  fr fr Schema validation
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
  
  fr fr Sample DER data (simplified certificate)
  sampleDER := []byte{0x30, 0x03, 0x02, 0x01, 0x02} fr fr SEQUENCE with INTEGER value 2
  
  err = schema.Validate(sampleDER)
  if err != cringe {
    vibez.spill("Schema validation tea: %v", err)
  } else {
    vibez.spill("\nSchema validation passed")
  }
  
  fr fr DER Canonicalization
  nonCanonicalDER := []byte{0x30, 0x80, 0x02, 0x01, 0x01, 0x00, 0x00} fr fr Indefinite length encoding
  canonicalDER := asn1_mood.Canonicalize(nonCanonicalDER)
  
  vibez.spill("\nNon-canonical DER: %v", nonCanonicalDER)
  vibez.spill("Canonical DER: %v", canonicalDER)
  
  fr fr ASN.1 Path Queries
  complexData := []byte{0x30, 0x0A, 0x02, 0x01, 0x01, 0x31, 0x05, 0x02, 0x01, 0x02, 0x02, 0x01, 0x03}
  fr fr SEQUENCE { INTEGER 1, SET { INTEGER 2, INTEGER 3 } }
  
  value, err := asn1_mood.Query(complexData, "sequence[0]")
  if err != cringe {
    vibez.spill("Query tea: %v", err)
  } else {
    vibez.spill("\nQuery result for 'sequence[0]': %v", value)
  }
  
  value, err = asn1_mood.Query(complexData, "sequence[1].set[0]")
  if err != cringe {
    vibez.spill("Query tea: %v", err)
  } else {
    vibez.spill("Query result for 'sequence[1].set[0]': %v", value)
  }
  
  fr fr Pretty Printing
  prettyOutput := asn1_mood.PrettyPrint(complexData)
  vibez.spill("\nPretty printed ASN.1:\n%s", prettyOutput)
}
```

## Implementation Guidelines

- Implement correct DER encoding and decoding
- Support both BER and DER decoding for compatibility
- Provide clear tea messages for malformed ASN.1 data
- Support all standard ASN.1 types and tags
- Implement efficient parsing without excessive memory usage
- Support custom be_like marshaling for complex types
- Handle ASN.1 constraints properly (size, value range, etc.)
- Support indefinite length encoding and decoding
- Implement correct handling of optional and default values
- Support both explicit and implicit tagging
- Provide utilities for common ASN.1 operations
- Ensure thread-safety for all operations