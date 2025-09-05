yeet "testz"
yeet "encoding_flex/mod_enhanced"

test_start("Base64 encoding/decoding")
sus data tea = "Hello"
sus encoded tea = base64_encode(data)
assert_true(len(encoded) > 0)

sus decoded, err = base64_decode(encoded)
assert_eq_string(err, "")

test_start("Hex encoding/decoding")
sus hex_encoded tea = hex_encode("AB")
assert_true(len(hex_encoded) > 0)

sus hex_decoded, hex_err = hex_decode("4142")  fr fr "AB" in hex
assert_eq_string(hex_err, "")

test_start("URL encoding/decoding")
sus url_data tea = "hello world"
sus url_encoded tea = url_encode(url_data)
assert_true(len(url_encoded) > 0)

sus url_decoded, url_err = url_decode("hello%20world")
assert_eq_string(url_err, "")

test_start("JSON encoding/decoding")
sus json_data tea = "hello"
sus json_encoded tea = json_encode(json_data)
assert_true(string_starts_with(json_encoded, "\""))

sus json_decoded, json_err = json_decode("\"hello\"")
assert_eq_string(json_err, "")

test_start("Binary encoding")
sus binary_data tea = write_uint16_be(1234)
assert_true(len(binary_data) == 2)

sus read_value, read_err = read_uint16_be(binary_data)
assert_eq_string(read_err, "")

sus binary32 tea = write_uint32_be(123456)
assert_true(len(binary32) == 4)

sus read_value32, read_err32 = read_uint32_be(binary32)
assert_eq_string(read_err32, "")

test_start("Error handling")
sus invalid_b64, b64_err = base64_decode("invalid!")
assert_true(b64_err != "")

sus invalid_hex, hex_invalid_err = hex_decode("xyz")
assert_true(hex_invalid_err != "")

sus invalid_url, url_invalid_err = url_decode("%xy")
assert_true(url_invalid_err != "")

sus invalid_json, json_invalid_err = json_decode("not_quoted")
assert_true(json_invalid_err != "")

test_start("Edge cases")
sus empty_b64 tea = base64_encode("")
assert_eq_string(empty_b64, "")

sus empty_hex tea = hex_encode("")
assert_eq_string(empty_hex, "")

sus empty_url tea = url_encode("")
assert_eq_string(empty_url, "")

print_test_summary()
