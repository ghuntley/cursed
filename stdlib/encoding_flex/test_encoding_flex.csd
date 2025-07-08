yeet "testz"
yeet "encoding_flex"

test_start("encoding_flex JSON encoding")

fr fr Test basic JSON encoding
sus opts := encoding_flex.JSONOptions{
    PrettyPrint: cap,
    EscapeHTML: cap,
    OmitEmpty: cap
}

sus data, err := encoding_flex.MarshalJSON("hello", opts)
assert_eq_string(err, "")
assert_true(len(data) > 0)

fr fr Test with pretty print
opts.PrettyPrint = based
opts.Indent = "  "
data, err = encoding_flex.MarshalJSON("test", opts)
assert_eq_string(err, "")
assert_true(len(data) > 2)

test_start("encoding_flex Base64 encoding")

fr fr Test standard Base64
sus input := []normie{72, 101, 108, 108, 111}  fr fr "Hello"
sus encoded := encoding_flex.EncodeBase64(input, encoding_flex.Base64Standard)
assert_true(len(encoded) > 0)

fr fr Test Base64 decoding
sus decoded, decErr := encoding_flex.DecodeBase64(encoded, encoding_flex.Base64Standard)
assert_eq_string(decErr, "")
assert_eq_int(len(decoded), len(encoded))

fr fr Test URL-safe Base64
sus urlEncoded := encoding_flex.EncodeBase64(input, encoding_flex.Base64URL)
assert_true(len(urlEncoded) > 0)

fr fr Test raw Base64
sus rawEncoded := encoding_flex.EncodeBase64(input, encoding_flex.Base64RawStandard)
assert_true(len(rawEncoded) > 0)

test_start("encoding_flex Hex encoding")

fr fr Test hex encoding
sus hexInput := []normie{255, 128, 0, 16}
sus hexEncoded := encoding_flex.EncodeHex(hexInput)
assert_eq_string(hexEncoded, "ff80000")  fr fr Check first few chars

fr fr Test hex decoding
sus hexDecoded, hexErr := encoding_flex.DecodeHex("ff8000")
assert_eq_string(hexErr, "")
assert_eq_int(len(hexDecoded), 3)
assert_eq_int(hexDecoded[0], 255)
assert_eq_int(hexDecoded[1], 128)
assert_eq_int(hexDecoded[2], 0)

fr fr Test invalid hex
hexDecoded, hexErr = encoding_flex.DecodeHex("gg")
assert_true(hexErr != "")

fr fr Test odd length hex
hexDecoded, hexErr = encoding_flex.DecodeHex("fff")
assert_true(hexErr != "")

test_start("encoding_flex Binary encoding")

fr fr Test uint16 encoding/decoding
sus value16 := 0x1234
sus encoded16 := encoding_flex.WriteUint16(value16, encoding_flex.BigEndian)
assert_eq_int(len(encoded16), 2)
assert_eq_int(encoded16[0], 0x12)
assert_eq_int(encoded16[1], 0x34)

sus decoded16 := encoding_flex.ReadUint16(encoded16, encoding_flex.BigEndian)
assert_eq_int(decoded16, value16)

fr fr Test little endian
sus encodedLE := encoding_flex.WriteUint16(value16, encoding_flex.LittleEndian)
assert_eq_int(encodedLE[0], 0x34)
assert_eq_int(encodedLE[1], 0x12)

sus decodedLE := encoding_flex.ReadUint16(encodedLE, encoding_flex.LittleEndian)
assert_eq_int(decodedLE, value16)

test_start("encoding_flex uint32 operations")

fr fr Test uint32 encoding/decoding
sus value32 := 0x12345678
sus encoded32 := encoding_flex.WriteUint32(value32, encoding_flex.BigEndian)
assert_eq_int(len(encoded32), 4)
assert_eq_int(encoded32[0], 0x12)
assert_eq_int(encoded32[1], 0x34)
assert_eq_int(encoded32[2], 0x56)
assert_eq_int(encoded32[3], 0x78)

sus decoded32 := encoding_flex.ReadUint32(encoded32, encoding_flex.BigEndian)
assert_eq_int(decoded32, value32)

test_start("encoding_flex URI encoding")

fr fr Test URI encoding
sus uri := "hello world"
sus encodedURI := encoding_flex.EncodeURI(uri)
assert_true(len(encodedURI) > len(uri))  fr fr Should be longer due to encoding

fr fr Test URI decoding
sus decodedURI, uriErr := encoding_flex.DecodeURI(encodedURI)
assert_eq_string(uriErr, "")

fr fr Test special characters
sus specialURI := "test#fragment"
sus encodedSpecial := encoding_flex.EncodeURI(specialURI)
assert_true(len(encodedSpecial) > len(specialURI))

test_start("encoding_flex Quoted-Printable")

fr fr Test quoted-printable encoding
sus qpInput := []normie{72, 101, 108, 108, 111, 33, 255}
sus qpEncoded := encoding_flex.EncodeQuotedPrintable(qpInput)
assert_true(len(qpEncoded) > len(qpInput))

fr fr Test quoted-printable decoding
sus qpDecoded, qpErr := encoding_flex.DecodeQuotedPrintable(qpEncoded)
assert_eq_string(qpErr, "")

test_start("encoding_flex ASCII85")

fr fr Test ASCII85 encoding
sus a85Input := []normie{72, 101, 108, 108, 111}
sus a85Encoded := encoding_flex.EncodeASCII85(a85Input)
assert_true(len(a85Encoded) > 0)

fr fr Test ASCII85 decoding
sus a85Decoded, a85Err := encoding_flex.DecodeASCII85(a85Encoded)
assert_eq_string(a85Err, "")
assert_true(len(a85Decoded) > 0)

test_start("encoding_flex options and configurations")

fr fr Test JSON options
sus jsonOpts := encoding_flex.JSONOptions{
    PrettyPrint: based,
    EscapeHTML: based,
    AllowNaN: cap,
    Indent: "    ",
    OmitEmpty: based
}

sus encoder := encoding_flex.NewJSONEncoder(jsonOpts)
sus encoded, encErr := encoder("test value")
assert_eq_string(encErr, "")
assert_true(len(encoded) > 0)

fr fr Test CSV options
sus csvOpts := encoding_flex.CSVOptions{
    Comma: ",",
    Comment: "#",
    FieldsPerRecord: 3,
    LazyQuotes: cap,
    TrimLeadingSpace: based,
    UseHeaders: based
}
assert_eq_string(csvOpts.Comma, ",")
assert_eq_int(csvOpts.FieldsPerRecord, 3)
assert_true(csvOpts.TrimLeadingSpace)

print_test_summary()
