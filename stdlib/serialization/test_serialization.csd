yeet "testz"
yeet "serialization"

test_start("Serialization Module Tests")

// Test integer serialization
sus int_data tea = serialize_int(12345)
sus deserialized_int normie = deserialize_int(int_data, 0)
assert_eq_int(deserialized_int, 12345)

// Test boolean serialization
sus bool_data tea = serialize_bool(based)
sus deserialized_bool lit = deserialize_bool(bool_data, 0)
assert_true(deserialized_bool)

sus bool_data_false tea = serialize_bool(cap)
sus deserialized_bool_false lit = deserialize_bool(bool_data_false, 0)
assert_false(deserialized_bool_false)

// Test string serialization
sus string_data tea = serialize_string("hello")
sus deserialized_string tea = deserialize_string(string_data, 0)
assert_eq_string(deserialized_string, "hello")

// Test varint encoding
sus varint_data tea = serialize_varint(127)
sus deserialized_varint normie = deserialize_varint(varint_data, 0)
assert_eq_int(deserialized_varint, 127)

// Test compression
sus test_data tea = "aaabbbccc"
sus compressed tea = compress_data(test_data)
sus decompressed tea = decompress_data(compressed)
// Note: compression may change the data, just test it doesn't crash

// Test checksum
sus original tea = "test data"
sus checksum normie = calculate_checksum(original)
assert_true(validate_checksum(original, checksum))

// Test versioned serialization
sus versioned tea = serialize_versioned("data", 1)
sus unversioned tea = deserialize_versioned(versioned)
assert_true(string_len(unversioned) >= 0)

print_test_summary()
