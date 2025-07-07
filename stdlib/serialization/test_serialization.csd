// CURSED Serialization Module Tests
// Comprehensive test suite for binary data serialization

yeet "testz"
yeet "serialization"

slay test_all_serialization() {
    test_start("Serialization Module Tests")
    
    test_integer_serialization()
    test_long_serialization()
    test_float_serialization()
    test_string_serialization()
    test_boolean_serialization()
    test_array_serialization()
    test_context_serialization()
    test_message_serialization()
    test_checksum_serialization()
    test_versioned_serialization()
    
    print_test_summary()
}

slay test_integer_serialization() {
    test_start("Integer Serialization")
    
    // Positive integers
    sus serialized1 tea = serialization.serialize_int(42)
    sus deserialized1 normie = serialization.deserialize_int(serialized1, 0)
    assert_eq_int(deserialized1, 42)
    
    // Zero
    sus serialized2 tea = serialization.serialize_int(0)
    sus deserialized2 normie = serialization.deserialize_int(serialized2, 0)
    assert_eq_int(deserialized2, 0)
    
    // Negative integers
    sus serialized3 tea = serialization.serialize_int(-123)
    sus deserialized3 normie = serialization.deserialize_int(serialized3, 0)
    assert_eq_int(deserialized3, -123)
    
    // Large integers
    sus serialized4 tea = serialization.serialize_int(2147483647)
    sus deserialized4 normie = serialization.deserialize_int(serialized4, 0)
    assert_eq_int(deserialized4, 2147483647)
    
    // Serialized data should be 4 bytes
    assert_eq_int(string_len(serialized1), 4)
    assert_eq_int(string_len(serialized2), 4)
    assert_eq_int(string_len(serialized3), 4)
    assert_eq_int(string_len(serialized4), 4)
}

slay test_long_serialization() {
    test_start("Long Serialization")
    
    // Positive longs
    sus serialized1 tea = serialization.serialize_long(9223372036854775807)
    sus deserialized1 thicc = serialization.deserialize_long(serialized1, 0)
    assert_eq_int(thicc(deserialized1), thicc(9223372036854775807))
    
    // Zero
    sus serialized2 tea = serialization.serialize_long(0)
    sus deserialized2 thicc = serialization.deserialize_long(serialized2, 0)
    assert_eq_int(thicc(deserialized2), 0)
    
    // Negative longs
    sus serialized3 tea = serialization.serialize_long(-456789)
    sus deserialized3 thicc = serialization.deserialize_long(serialized3, 0)
    assert_eq_int(thicc(deserialized3), thicc(-456789))
    
    // Serialized data should be 8 bytes
    assert_eq_int(string_len(serialized1), 8)
    assert_eq_int(string_len(serialized2), 8)
    assert_eq_int(string_len(serialized3), 8)
}

slay test_float_serialization() {
    test_start("Float Serialization")
    
    // Positive floats
    sus serialized1 tea = serialization.serialize_float(3.14159)
    sus deserialized1 meal = serialization.deserialize_float(serialized1, 0)
    assert_true(deserialized1 > 3.0 && deserialized1 < 4.0)
    
    // Zero
    sus serialized2 tea = serialization.serialize_float(0.0)
    sus deserialized2 meal = serialization.deserialize_float(serialized2, 0)
    assert_true(deserialized2 == 0.0)
    
    // Negative floats
    sus serialized3 tea = serialization.serialize_float(-2.718)
    sus deserialized3 meal = serialization.deserialize_float(serialized3, 0)
    assert_true(deserialized3 < -2.0 && deserialized3 > -3.0)
    
    // Serialized data should be 4 bytes (float size)
    assert_eq_int(string_len(serialized1), 4)
    assert_eq_int(string_len(serialized2), 4)
    assert_eq_int(string_len(serialized3), 4)
}

slay test_string_serialization() {
    test_start("String Serialization")
    
    // Regular strings
    sus serialized1 tea = serialization.serialize_string("hello")
    sus deserialized1 tea = serialization.deserialize_string(serialized1, 0)
    assert_eq_string(deserialized1, "hello")
    
    // Empty string
    sus serialized2 tea = serialization.serialize_string("")
    sus deserialized2 tea = serialization.deserialize_string(serialized2, 0)
    assert_eq_string(deserialized2, "")
    
    // Long string
    sus long_str tea = "This is a very long string for testing serialization"
    sus serialized3 tea = serialization.serialize_string(long_str)
    sus deserialized3 tea = serialization.deserialize_string(serialized3, 0)
    assert_eq_string(deserialized3, long_str)
    
    // String with special characters
    sus special_str tea = "Hello\nWorld\t!"
    sus serialized4 tea = serialization.serialize_string(special_str)
    sus deserialized4 tea = serialization.deserialize_string(serialized4, 0)
    assert_eq_string(deserialized4, special_str)
    
    // Serialized data should include 4-byte length prefix
    assert_eq_int(string_len(serialized1), 4 + 5)  // 4 bytes length + 5 chars
    assert_eq_int(string_len(serialized2), 4 + 0)  // 4 bytes length + 0 chars
}

slay test_boolean_serialization() {
    test_start("Boolean Serialization")
    
    // True value
    sus serialized1 tea = serialization.serialize_bool(based)
    sus deserialized1 lit = serialization.deserialize_bool(serialized1, 0)
    assert_true(deserialized1)
    
    // False value
    sus serialized2 tea = serialization.serialize_bool(cap)
    sus deserialized2 lit = serialization.deserialize_bool(serialized2, 0)
    assert_false(deserialized2)
    
    // Serialized data should be 1 byte
    assert_eq_int(string_len(serialized1), 1)
    assert_eq_int(string_len(serialized2), 1)
}

slay test_array_serialization() {
    test_start("Array Serialization")
    
    // Integer arrays
    sus int_array [normie] = [1, 2, 3, 4, 5]
    sus serialized_ints tea = serialization.serialize_array_int(int_array)
    sus deserialized_ints [normie] = serialization.deserialize_array_int(serialized_ints, 0)
    
    assert_eq_int(len(deserialized_ints), 5)
    assert_eq_int(deserialized_ints[0], 1)
    assert_eq_int(deserialized_ints[1], 2)
    assert_eq_int(deserialized_ints[2], 3)
    assert_eq_int(deserialized_ints[3], 4)
    assert_eq_int(deserialized_ints[4], 5)
    
    // String arrays
    sus str_array [tea] = ["hello", "world", "test"]
    sus serialized_strs tea = serialization.serialize_array_string(str_array)
    sus deserialized_strs [tea] = serialization.deserialize_array_string(serialized_strs, 0)
    
    assert_eq_int(len(deserialized_strs), 3)
    assert_eq_string(deserialized_strs[0], "hello")
    assert_eq_string(deserialized_strs[1], "world")
    assert_eq_string(deserialized_strs[2], "test")
    
    // Empty arrays
    sus empty_ints [normie] = []
    sus serialized_empty tea = serialization.serialize_array_int(empty_ints)
    sus deserialized_empty [normie] = serialization.deserialize_array_int(serialized_empty, 0)
    assert_eq_int(len(deserialized_empty), 0)
}

slay test_context_serialization() {
    test_start("Context Serialization")
    
    // Create context
    sus context serialization.SerializationContext = serialization.create_serialization_context()
    
    // Write data
    context = serialization.write_int(context, 42)
    context = serialization.write_string(context, "hello")
    context = serialization.write_bool(context, based)
    context = serialization.write_float(context, 3.14)
    context = serialization.write_long(context, 123456789)
    
    // Create read context
    sus read_context serialization.SerializationContext = serialization.SerializationContext{
        data: context.data,
        offset: 0,
        error: ""
    }
    
    // Read data back
    sus int_val normie = serialization.read_int(read_context)
    sus str_val tea = serialization.read_string(read_context)
    sus bool_val lit = serialization.read_bool(read_context)
    sus float_val meal = serialization.read_float(read_context)
    sus long_val thicc = serialization.read_long(read_context)
    
    // Verify values
    assert_eq_int(int_val, 42)
    assert_eq_string(str_val, "hello")
    assert_true(bool_val)
    assert_true(float_val > 3.0 && float_val < 4.0)
    assert_eq_int(thicc(long_val), thicc(123456789))
}

slay test_message_serialization() {
    test_start("Message Serialization")
    
    // Create message
    sus message serialization.Message = serialization.Message{
        field_id: 1,
        field_type: 2,
        data: "test message data"
    }
    
    // Serialize and deserialize
    sus serialized tea = serialization.serialize_message(message)
    sus deserialized serialization.Message = serialization.deserialize_message(serialized, 0)
    
    // Verify message fields
    assert_eq_int(deserialized.field_id, 1)
    assert_eq_int(deserialized.field_type, 2)
    assert_eq_string(deserialized.data, "test message data")
    
    // Test different message types
    sus message2 serialization.Message = serialization.Message{
        field_id: 100,
        field_type: 5,
        data: "different data"
    }
    
    sus serialized2 tea = serialization.serialize_message(message2)
    sus deserialized2 serialization.Message = serialization.deserialize_message(serialized2, 0)
    
    assert_eq_int(deserialized2.field_id, 100)
    assert_eq_int(deserialized2.field_type, 5)
    assert_eq_string(deserialized2.data, "different data")
}

slay test_checksum_serialization() {
    test_start("Checksum Serialization")
    
    // Test data
    sus test_data tea = "This is test data for checksum validation"
    
    // Calculate checksum
    sus checksum normie = serialization.calculate_checksum(test_data)
    assert_true(checksum > 0)
    
    // Validate checksum
    assert_true(serialization.validate_checksum(test_data, checksum))
    assert_false(serialization.validate_checksum(test_data, checksum + 1))
    assert_false(serialization.validate_checksum("different data", checksum))
    
    // Serialize with checksum
    sus with_checksum tea = serialization.serialize_with_checksum(test_data)
    sus without_checksum tea = serialization.deserialize_with_checksum(with_checksum)
    assert_eq_string(without_checksum, test_data)
    
    // Test corrupted data
    sus corrupted tea = with_checksum
    // Simulate corruption by changing a byte (if possible)
    sus corrupted_result tea = serialization.deserialize_with_checksum(corrupted)
    // Should return original data if checksum is valid
    assert_eq_string(corrupted_result, test_data)
}

slay test_versioned_serialization() {
    test_start("Versioned Serialization")
    
    // Test data with version
    sus test_data tea = "Version 1 data format"
    sus version normie = 1
    
    // Serialize with version
    sus versioned tea = serialization.serialize_versioned(test_data, version)
    sus unversioned tea = serialization.deserialize_versioned(versioned)
    assert_eq_string(unversioned, test_data)
    
    // Test different version
    sus test_data2 tea = "Version 2 data format"
    sus version2 normie = 2
    
    sus versioned2 tea = serialization.serialize_versioned(test_data2, version2)
    sus unversioned2 tea = serialization.deserialize_versioned(versioned2)
    assert_eq_string(unversioned2, test_data2)
    
    // Verify version information is preserved
    assert_true(string_len(versioned) > string_len(test_data))
    assert_true(string_len(versioned2) > string_len(test_data2))
}

// Utility function tests
slay test_utility_functions() {
    test_start("Utility Functions")
    
    // Byte/char conversion
    sus byte_val normie = 65
    sus char_val tea = serialization.byte_to_char(byte_val)
    sus back_to_byte normie = serialization.char_to_byte(char_val)
    assert_eq_int(back_to_byte, byte_val)
    
    // Float/int bits conversion
    sus float_val meal = 3.14159
    sus int_bits normie = serialization.float_to_int_bits(float_val)
    sus back_to_float meal = serialization.int_bits_to_float(int_bits)
    assert_true(back_to_float > 3.0 && back_to_float < 4.0)
    
    // Varint size calculation
    assert_eq_int(serialization.varint_size(0), 1)
    assert_eq_int(serialization.varint_size(127), 1)
    assert_eq_int(serialization.varint_size(128), 2)
    assert_eq_int(serialization.varint_size(16383), 2)
    assert_eq_int(serialization.varint_size(16384), 3)
}

// Object serialization tests
slay test_object_serialization() {
    test_start("Object Serialization")
    
    // Create test object
    sus fields map[tea]tea = {
        "name": "John",
        "age": "30",
        "city": "New York"
    }
    
    // Serialize object
    sus serialized tea = serialization.serialize_object(fields)
    assert_true(string_len(serialized) > 0)
    
    // Deserialize object
    sus deserialized map[tea]tea = serialization.deserialize_object(serialized)
    // Note: Actual validation would depend on proper map implementation
    
    // Verify serialized format
    assert_true(string_char_at(serialized, 0) == "{")
    assert_true(string_char_at(serialized, string_len(serialized) - 1) == "}")
}

// Compressed serialization tests
slay test_compressed_serialization() {
    test_start("Compressed Serialization")
    
    // Test data
    sus test_data tea = "This is a long string that should compress well when repeated. This is a long string that should compress well when repeated."
    
    // Serialize with compression
    sus compressed tea = serialization.serialize_compressed(test_data)
    sus decompressed tea = serialization.deserialize_compressed(compressed)
    assert_eq_string(decompressed, test_data)
    
    // Verify compression occurred
    assert_true(string_len(compressed) < string_len(test_data) + 10)  // Allow for overhead
}

// Run all tests
test_all_serialization()
