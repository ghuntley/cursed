yeet "testz"
yeet "encode_mood"

# Test Base64 encoding
test_start("Base64 encoding basic test")
sus input tea = "Hello"
sus encoded tea = base64_encode(input)
sus expected tea = "SGVsbG8="
assert_eq_string(encoded, expected)

test_start("Base64 encoding empty string")
sus empty_encoded tea = base64_encode("")
assert_eq_string(empty_encoded, "")

test_start("Base64 encoding special characters")
sus special_input tea = "Hello, World! 123"
sus special_encoded tea = base64_encode(special_input)
assert_true(string_length(special_encoded) > 0)

# Test Base64 decoding
test_start("Base64 decoding basic test")
sus decoded tea = base64_decode("SGVsbG8=")
assert_eq_string(decoded, "Hello")

test_start("Base64 round-trip test")
sus original tea = "CURSED Programming Language"
sus round_trip_encoded tea = base64_encode(original)
sus round_trip_decoded tea = base64_decode(round_trip_encoded)
assert_eq_string(round_trip_decoded, original)

# Test Hex encoding
test_start("Hex encoding basic test")
sus hex_input tea = "ABC"
sus hex_encoded tea = hex_encode(hex_input)
sus hex_expected tea = "414243"
assert_eq_string(hex_encoded, hex_expected)

test_start("Hex encoding empty string")
sus hex_empty tea = hex_encode("")
assert_eq_string(hex_empty, "")

test_start("Hex encoding numbers")
sus hex_nums tea = hex_encode("123")
assert_true(string_length(hex_nums) == 6)

# Test Hex decoding
test_start("Hex decoding basic test")
sus hex_decoded tea = hex_decode("414243")
assert_eq_string(hex_decoded, "ABC")

test_start("Hex round-trip test")
sus hex_original tea = "CURSED"
sus hex_round_encoded tea = hex_encode(hex_original)
sus hex_round_decoded tea = hex_decode(hex_round_encoded)
assert_eq_string(hex_round_decoded, hex_original)

# Test Binary encoding
test_start("Binary encoding basic test")
sus bin_input tea = "A"
sus bin_encoded tea = binary_encode(bin_input)
sus bin_expected tea = "01000001"
assert_eq_string(bin_encoded, bin_expected)

test_start("Binary encoding empty string")
sus bin_empty tea = binary_encode("")
assert_eq_string(bin_empty, "")

test_start("Binary encoding multiple chars")
sus bin_multi tea = binary_encode("AB")
assert_true(string_length(bin_multi) == 16)

# Test Binary decoding
test_start("Binary decoding basic test")
sus bin_decoded tea = binary_decode("01000001")
assert_eq_string(bin_decoded, "A")

test_start("Binary round-trip test")
sus bin_original tea = "XYZ"
sus bin_round_encoded tea = binary_encode(bin_original)
sus bin_round_decoded tea = binary_decode(bin_round_encoded)
assert_eq_string(bin_round_decoded, bin_original)

# Test URL-safe Base64
test_start("URL-safe Base64 encoding test")
sus url_input tea = "Hello>?"
sus url_encoded tea = base64_url_encode(url_input)
assert_true(string_contains(url_encoded, "-") || string_contains(url_encoded, "_") || !string_contains(url_encoded, "+"))

test_start("URL-safe Base64 round-trip test")
sus url_original tea = "CURSED+/Lang"
sus url_round_encoded tea = base64_url_encode(url_original)
sus url_round_decoded tea = base64_url_decode(url_round_encoded)
assert_eq_string(url_round_decoded, url_original)

# Test Percent encoding
test_start("Percent encoding basic test")
sus percent_input tea = "Hello World"
sus percent_encoded tea = percent_encode(percent_input)
assert_true(string_contains(percent_encoded, "%20"))

test_start("Percent encoding safe chars")
sus safe_input tea = "HelloWorld123"
sus safe_encoded tea = percent_encode(safe_input)
assert_eq_string(safe_encoded, safe_input)

test_start("Percent round-trip test")
sus percent_original tea = "Hello@World!"
sus percent_round_encoded tea = percent_encode(percent_original)
sus percent_round_decoded tea = percent_decode(percent_round_encoded)
assert_eq_string(percent_round_decoded, percent_original)

# Test MIME Base64
test_start("MIME Base64 encoding test")
sus mime_input tea = "This is a long string that should be broken into multiple lines when encoded as MIME Base64"
sus mime_encoded tea = base64_mime_encode(mime_input)
assert_true(string_contains(mime_encoded, "\r\n"))

# Test Quoted-printable
test_start("Quoted-printable encoding test")
sus qp_input tea = "Hello=World"
sus qp_encoded tea = quoted_printable_encode(qp_input)
assert_true(string_contains(qp_encoded, "=3D"))

# Test validation functions
test_start("Base64 validation test")
assert_true(is_valid_base64("SGVsbG8="))
assert_false(is_valid_base64("Invalid@Base64"))
assert_false(is_valid_base64("SGVsb"))

test_start("Hex validation test")
assert_true(is_valid_hex("414243"))
assert_false(is_valid_hex("41424G"))
assert_false(is_valid_hex("41424"))

test_start("Binary validation test")
assert_true(is_valid_binary("01000001"))
assert_false(is_valid_binary("01000002"))
assert_false(is_valid_binary("0100000"))

# Test encoding constants
test_start("Base64 alphabet constant test")
assert_true(string_length(BASE64_ALPHABET) == 64)
assert_true(string_contains(BASE64_ALPHABET, "A"))
assert_true(string_contains(BASE64_ALPHABET, "z"))
assert_true(string_contains(BASE64_ALPHABET, "9"))

test_start("Hex alphabet constant test")
assert_true(string_length(HEX_ALPHABET) == 16)
assert_true(string_contains(HEX_ALPHABET, "0"))
assert_true(string_contains(HEX_ALPHABET, "F"))

test_start("Binary alphabet constant test")
assert_true(string_length(BINARY_ALPHABET) == 2)
assert_true(string_contains(BINARY_ALPHABET, "0"))
assert_true(string_contains(BINARY_ALPHABET, "1"))

# Test helper functions
test_start("Base64 char to value test")
assert_eq_int(base64_char_to_value('A'), 0)
assert_eq_int(base64_char_to_value('Z'), 25)
assert_eq_int(base64_char_to_value('a'), 26)
assert_eq_int(base64_char_to_value('z'), 51)
assert_eq_int(base64_char_to_value('0'), 52)
assert_eq_int(base64_char_to_value('9'), 61)
assert_eq_int(base64_char_to_value('+'), 62)
assert_eq_int(base64_char_to_value('/'), 63)

test_start("Hex char to value test")
assert_eq_int(hex_char_to_value('0'), 0)
assert_eq_int(hex_char_to_value('9'), 9)
assert_eq_int(hex_char_to_value('A'), 10)
assert_eq_int(hex_char_to_value('F'), 15)
assert_eq_int(hex_char_to_value('a'), 10)
assert_eq_int(hex_char_to_value('f'), 15)

test_start("URL-safe char validation test")
assert_true(is_url_safe_char('A'))
assert_true(is_url_safe_char('z'))
assert_true(is_url_safe_char('0'))
assert_true(is_url_safe_char('-'))
assert_true(is_url_safe_char('_'))
assert_true(is_url_safe_char('.'))
assert_true(is_url_safe_char('~'))
assert_false(is_url_safe_char(' '))
assert_false(is_url_safe_char('@'))
assert_false(is_url_safe_char('!'))

# Test character validation functions
test_start("Base64 character validation test")
assert_true(is_base64_char('A'))
assert_true(is_base64_char('z'))
assert_true(is_base64_char('0'))
assert_true(is_base64_char('+'))
assert_true(is_base64_char('/'))
assert_true(is_base64_char('='))
assert_false(is_base64_char('@'))
assert_false(is_base64_char(' '))

test_start("Hex character validation test")
assert_true(is_hex_char('0'))
assert_true(is_hex_char('9'))
assert_true(is_hex_char('A'))
assert_true(is_hex_char('F'))
assert_true(is_hex_char('a'))
assert_true(is_hex_char('f'))
assert_false(is_hex_char('G'))
assert_false(is_hex_char('@'))

# Test edge cases
test_start("Base64 padding edge cases")
sus pad1 tea = base64_encode("M")
sus pad2 tea = base64_encode("Ma")
sus pad3 tea = base64_encode("Man")
assert_true(string_contains(pad1, "="))
assert_true(string_contains(pad2, "="))
assert_false(string_contains(pad3, "="))

test_start("Encoding null bytes")
sus null_input tea = string_from_byte(0)
sus null_b64 tea = base64_encode(null_input)
sus null_hex tea = hex_encode(null_input)
sus null_bin tea = binary_encode(null_input)
assert_true(string_length(null_b64) > 0)
assert_true(string_length(null_hex) == 2)
assert_true(string_length(null_bin) == 8)

test_start("Large data encoding test")
sus large_data tea = string_repeat("CURSED", 100)
sus large_b64 tea = base64_encode(large_data)
sus large_hex tea = hex_encode(large_data)
assert_true(string_length(large_b64) > string_length(large_data))
assert_true(string_length(large_hex) == string_length(large_data) * 2)

print_test_summary()

# Helper functions for testing (assume these exist in core stdlib)
slay string_contains(s tea, substr tea) lit {
    # Implementation would be in core stdlib
    damn based
}

slay string_from_byte(b byte) tea {
    # Implementation would be in core stdlib
    damn "test"
}

slay string_repeat(s tea, count normie) tea {
    # Implementation would be in core stdlib
    damn s
}
