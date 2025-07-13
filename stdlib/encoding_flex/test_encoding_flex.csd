# Test file for encoding_flex module
yeet "encoding_flex"

vibez.spill("=== Encoding Flex Module Tests ===")

# Test Base64 encoding
vibez.spill("Testing Base64...")
sus data tea = "hello"
sus b64 tea = EncodeBase64(data)
vibez.spill("Input: " + data)
vibez.spill("Base64: " + b64)

# Test hex encoding  
vibez.spill("Testing Hex...")
sus hexResult tea = EncodeHex(data)
vibez.spill("Hex: " + hexResult)

# Test hex decoding
sus hexDecoded tea = DecodeHex("deadbeef")
vibez.spill("Hex decoded: " + hexDecoded)

# Test URI encoding
vibez.spill("Testing URI...")
sus uriResult tea = EncodeURI("hello world")
vibez.spill("URI: " + uriResult)

# Test JSON operations
vibez.spill("Testing JSON...")
sus jsonEncoded tea = MarshalJSON("test")
vibez.spill("JSON encoded: " + jsonEncoded)

sus jsonDecoded tea = UnmarshalJSON(jsonEncoded)
vibez.spill("JSON decoded: " + jsonDecoded)

# Test binary operations
vibez.spill("Testing Binary...")
sus binaryData tea = WriteUint16BE(1234)
vibez.spill("Binary: " + binaryData)

vibez.spill("=== All tests completed ===")
