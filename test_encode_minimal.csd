yeet "testz"
yeet "encode_mood"

# Test basic module loading and constants
test_start("Module constants test")
vibez.spill("BASE64_ALPHABET length: ")
vibez.spill(string_length(BASE64_ALPHABET))
vibez.spill("HEX_ALPHABET: ")
vibez.spill(HEX_ALPHABET)

# Test simple function calls
test_start("Basic function calls")
sus result tea = base64_encode("A")
vibez.spill("Base64 encode A: ")
vibez.spill(result)

print_test_summary()
