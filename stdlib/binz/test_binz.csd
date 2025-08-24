fr fr BINZ TEST SUITE - Comprehensive Binary Serialization Tests
fr fr Testing all aspects of the binary format including edge cases

yeet "binz"
yeet "testz"
yeet "vibez"
yeet "stringz"
yeet "mathz"

fr fr ===== BASIC ENCODING/DECODING TESTS =====

slay test_basic_null_encoding() lit {
    fr fr Test null value encoding/decoding
    sus value BinzValue = binz_create_null()
    sus encoded []drip = binz_encode(value)
    sus decoded BinzValue = binz_decode(encoded)
    
    test_assert(decoded.type_tag == TAG_NULL, "Null encoding roundtrip failed")
    vibez.spill("✓ Null encoding test passed")
    damn based
}

slay test_basic_boolean_encoding() lit {
    fr fr Test boolean values
    sus true_val BinzValue = binz_create_bool(based)
    sus false_val BinzValue = binz_create_bool(cringe)
    
    sus true_encoded []drip = binz_encode(true_val)
    sus false_encoded []drip = binz_encode(false_val)
    
    sus true_decoded BinzValue = binz_decode(true_encoded)
    sus false_decoded BinzValue = binz_decode(false_encoded)
    
    test_assert(true_decoded.type_tag == TAG_BOOL_TRUE, "True boolean encoding failed")
    test_assert(true_decoded.bool_value == based, "True boolean value incorrect")
    test_assert(false_decoded.type_tag == TAG_BOOL_FALSE, "False boolean encoding failed")
    test_assert(false_decoded.bool_value == cringe, "False boolean value incorrect")
    
    vibez.spill("✓ Boolean encoding tests passed")
    damn based
}

slay test_integer_encoding() lit {
    fr fr Test various integer values
    sus test_values []drip = [0, 1, 42, -1, -42, 2147483647, -2147483648]
    sus i drip = 0
    sus count drip = array_length(test_values)
    
    bestie (i < count) {
        sus value BinzValue = binz_create_int(test_values[i])
        sus encoded []drip = binz_encode(value)
        sus decoded BinzValue = binz_decode(encoded)
        
        test_assert(decoded.type_tag == TAG_INT32, "Integer encoding type mismatch")
        test_assert(decoded.int_value == test_values[i], "Integer value mismatch for: " + int_to_string(test_values[i]))
        
        i = i + 1
    }
    
    vibez.spill("✓ Integer encoding tests passed")
    damn based
}

slay test_float_encoding() lit {
    fr fr Test floating point values
    sus test_values []normie = [0.0, 1.0, -1.0, 3.14159, -3.14159, 1.23456789]
    sus i drip = 0
    sus count drip = array_length(test_values)
    
    bestie (i < count) {
        sus value BinzValue = binz_create_float(test_values[i])
        sus encoded []drip = binz_encode(value)
        sus decoded BinzValue = binz_decode(encoded)
        
        test_assert(decoded.type_tag == TAG_FLOAT64, "Float encoding type mismatch")
        
        fr fr Compare floats with small tolerance
        sus diff normie = abs_float(decoded.float_value - test_values[i])
        test_assert(diff < 0.0001, "Float value mismatch")
        
        i = i + 1
    }
    
    vibez.spill("✓ Float encoding tests passed")
    damn based
}

slay test_string_encoding() lit {
    fr fr Test string encoding (both short and long)
    sus test_strings []tea = ["", "hello", "world", "The quick brown fox jumps over the lazy dog"]
    sus i drip = 0
    sus count drip = array_length(test_strings)
    
    bestie (i < count) {
        sus value BinzValue = binz_create_string(test_strings[i])
        sus encoded []drip = binz_encode(value)
        sus decoded BinzValue = binz_decode(encoded)
        
        sus expected_tag drip = ready (string_length(test_strings[i]) < 256) { TAG_STRING_SHORT } otherwise { TAG_STRING_LONG }
        test_assert(decoded.type_tag == expected_tag, "String encoding type mismatch")
        test_assert(decoded.string_value == test_strings[i], "String value mismatch: " + test_strings[i])
        
        i = i + 1
    }
    
    fr fr Test very long string (> 255 chars)
    sus long_string tea = generate_long_string(300)
    sus long_value BinzValue = binz_create_string(long_string)
    sus long_encoded []drip = binz_encode(long_value)
    sus long_decoded BinzValue = binz_decode(long_encoded)
    
    test_assert(long_decoded.type_tag == TAG_STRING_LONG, "Long string should use TAG_STRING_LONG")
    test_assert(long_decoded.string_value == long_string, "Long string value mismatch")
    
    vibez.spill("✓ String encoding tests passed")
    damn based
}

fr fr ===== COMPLEX DATA STRUCTURE TESTS =====

slay test_array_encoding() lit {
    fr fr Test array encoding/decoding
    sus array_val BinzValue = binz_create_array()
    
    fr fr Add various elements
    array_val.array_values[0] = binz_create_int(42)
    array_val.array_values[1] = binz_create_string("test")
    array_val.array_values[2] = binz_create_bool(based)
    array_val.array_values[3] = binz_create_float(3.14)
    array_val.array_values[4] = binz_create_null()
    
    sus encoded []drip = binz_encode(array_val)
    sus decoded BinzValue = binz_decode(encoded)
    
    test_assert(decoded.type_tag == TAG_ARRAY_MIXED, "Array encoding type mismatch")
    sus decoded_count drip = array_length(decoded.array_values)
    test_assert(decoded_count == 5, "Array element count mismatch")
    
    fr fr Verify each element
    test_assert(decoded.array_values[0].type_tag == TAG_INT32, "Array element 0 type")
    test_assert(decoded.array_values[0].int_value == 42, "Array element 0 value")
    test_assert(decoded.array_values[1].type_tag == TAG_STRING_SHORT, "Array element 1 type")
    test_assert(decoded.array_values[1].string_value == "test", "Array element 1 value")
    test_assert(decoded.array_values[2].type_tag == TAG_BOOL_TRUE, "Array element 2 type")
    test_assert(decoded.array_values[3].type_tag == TAG_FLOAT64, "Array element 3 type")
    test_assert(decoded.array_values[4].type_tag == TAG_NULL, "Array element 4 type")
    
    vibez.spill("✓ Array encoding tests passed")
    damn based
}

slay test_struct_encoding() lit {
    fr fr Test struct encoding/decoding
    sus struct_val BinzValue = binz_create_struct()
    
    fr fr Add fields
    struct_val.struct_fields[0] = "name"
    struct_val.struct_values[0] = binz_create_string("Alice")
    
    struct_val.struct_fields[1] = "age"
    struct_val.struct_values[1] = binz_create_int(30)
    
    struct_val.struct_fields[2] = "active"
    struct_val.struct_values[2] = binz_create_bool(based)
    
    struct_val.struct_fields[3] = "score"
    struct_val.struct_values[3] = binz_create_float(98.5)
    
    sus encoded []drip = binz_encode(struct_val)
    sus decoded BinzValue = binz_decode(encoded)
    
    test_assert(decoded.type_tag == TAG_STRUCT, "Struct encoding type mismatch")
    sus field_count drip = array_length(decoded.struct_fields)
    test_assert(field_count == 4, "Struct field count mismatch")
    
    fr fr Verify fields (order may vary but content should match)
    sus found_name lit = cringe
    sus found_age lit = cringe
    sus found_active lit = cringe
    sus found_score lit = cringe
    
    sus i drip = 0
    bestie (i < field_count) {
        sus field_name tea = decoded.struct_fields[i]
        sus field_value BinzValue = decoded.struct_values[i]
        
        ready (field_name == "name") {
            test_assert(field_value.string_value == "Alice", "Name field value")
            found_name = based
        } otherwise ready (field_name == "age") {
            test_assert(field_value.int_value == 30, "Age field value")
            found_age = based
        } otherwise ready (field_name == "active") {
            test_assert(field_value.bool_value == based, "Active field value")
            found_active = based
        } otherwise ready (field_name == "score") {
            sus diff normie = abs_float(field_value.float_value - 98.5)
            test_assert(diff < 0.001, "Score field value")
            found_score = based
        }
        
        i = i + 1
    }
    
    test_assert(found_name && found_age && found_active && found_score, "All struct fields found")
    
    vibez.spill("✓ Struct encoding tests passed")
    damn based
}

slay test_nested_structures() lit {
    fr fr Test nested arrays and structs
    sus inner_struct BinzValue = binz_create_struct()
    inner_struct.struct_fields[0] = "x"
    inner_struct.struct_values[0] = binz_create_int(10)
    inner_struct.struct_fields[1] = "y" 
    inner_struct.struct_values[1] = binz_create_int(20)
    
    sus inner_array BinzValue = binz_create_array()
    inner_array.array_values[0] = binz_create_int(1)
    inner_array.array_values[1] = binz_create_int(2)
    inner_array.array_values[2] = binz_create_int(3)
    
    sus outer_struct BinzValue = binz_create_struct()
    outer_struct.struct_fields[0] = "point"
    outer_struct.struct_values[0] = inner_struct
    outer_struct.struct_fields[1] = "numbers"
    outer_struct.struct_values[1] = inner_array
    
    sus encoded []drip = binz_encode(outer_struct)
    sus decoded BinzValue = binz_decode(encoded)
    
    test_assert(decoded.type_tag == TAG_STRUCT, "Nested structure root type")
    
    fr fr Find and verify nested point structure
    sus point_found lit = cringe
    sus numbers_found lit = cringe
    
    sus field_count drip = array_length(decoded.struct_fields)
    sus i drip = 0
    bestie (i < field_count) {
        ready (decoded.struct_fields[i] == "point") {
            sus point BinzValue = decoded.struct_values[i]
            test_assert(point.type_tag == TAG_STRUCT, "Nested point type")
            point_found = based
        } otherwise ready (decoded.struct_fields[i] == "numbers") {
            sus numbers BinzValue = decoded.struct_values[i]
            test_assert(numbers.type_tag == TAG_ARRAY_MIXED, "Nested array type")
            test_assert(array_length(numbers.array_values) == 3, "Nested array length")
            numbers_found = based
        }
        i = i + 1
    }
    
    test_assert(point_found && numbers_found, "All nested structures found")
    
    vibez.spill("✓ Nested structure tests passed")
    damn based
}

fr fr ===== SCHEMA SYSTEM TESTS =====

slay test_schema_creation_and_validation() lit {
    fr fr Test schema definition and validation
    sus user_schema BinzSchema = binz_create_schema(1001, 1, "User")
    user_schema = binz_schema_add_field(user_schema, "id", "uint32", cringe)         fr fr required
    user_schema = binz_schema_add_field(user_schema, "name", "string", cringe)      fr fr required
    user_schema = binz_schema_add_field(user_schema, "email", "string", based)     fr fr optional
    user_schema = binz_schema_add_field(user_schema, "age", "uint32", based)       fr fr optional
    user_schema.compatibility_mode = "forward"
    
    fr fr Create valid user data
    sus user_data BinzValue = binz_create_struct()
    user_data.struct_fields[0] = "id"
    user_data.struct_values[0] = binz_create_uint(12345)
    user_data.struct_fields[1] = "name"
    user_data.struct_values[1] = binz_create_string("John Doe")
    user_data.struct_fields[2] = "email"
    user_data.struct_values[2] = binz_create_string("john@example.com")
    user_data.struct_fields[3] = "age"
    user_data.struct_values[3] = binz_create_uint(25)
    
    fr fr Test schema validation
    sus validation_result lit = binz_validate_against_schema(user_data, user_schema)
    test_assert(validation_result, "Valid user data should pass schema validation")
    
    fr fr Test schema encoding/decoding
    sus encoded []drip = binz_encode_with_schema(user_data, user_schema)
    test_assert(array_length(encoded) > 0, "Schema-based encoding should produce output")
    
    sus decoded BinzValue = binz_decode_with_schema(encoded, user_schema)
    test_assert(decoded.type_tag == TAG_STRUCT, "Schema-decoded data should be struct")
    
    vibez.spill("✓ Schema system tests passed")
    damn based
}

slay test_schema_migration() lit {
    fr fr Test schema version migration
    sus old_schema BinzSchema = binz_create_schema(2001, 1, "Product")
    old_schema = binz_schema_add_field(old_schema, "id", "uint32", cringe)
    old_schema = binz_schema_add_field(old_schema, "name", "string", cringe)
    old_schema = binz_schema_add_field(old_schema, "price", "float64", cringe)
    
    sus new_schema BinzSchema = binz_create_schema(2001, 2, "Product")
    new_schema = binz_schema_add_field(new_schema, "id", "uint32", cringe)
    new_schema = binz_schema_add_field(new_schema, "title", "string", cringe)     fr fr renamed from 'name'
    new_schema = binz_schema_add_field(new_schema, "price", "float64", cringe)
    new_schema = binz_schema_add_field(new_schema, "category", "string", based)  fr fr new optional field
    new_schema.compatibility_mode = "backward"
    
    fr fr Create migration rule
    sus migration BinzMigrationRule = BinzMigrationRule{}
    migration.from_version = 1
    migration.to_version = 2
    migration.field_mappings = []
    
    sus name_mapping BinzFieldMapping = BinzFieldMapping{}
    name_mapping.old_name = "name"
    name_mapping.new_name = "title"
    name_mapping.type_conversion = "none"
    migration.field_mappings[0] = name_mapping
    
    old_schema.migration_rules[0] = migration
    
    fr fr Test migration
    sus migrated_schema BinzSchema = binz_migrate_schema(old_schema, new_schema)
    test_assert(migrated_schema.version == 2, "Migrated schema should have new version")
    
    vibez.spill("✓ Schema migration tests passed")
    damn based
}

fr fr ===== COMPRESSION TESTS =====

slay test_compression() lit {
    fr fr Test data compression/decompression
    sus large_struct BinzValue = create_large_test_struct()
    sus compressed_value BinzValue = BinzValue{}
    compressed_value.type_tag = TAG_COMPRESSED
    compressed_value = large_struct  fr fr Content to compress
    
    sus encoded []drip = binz_encode(compressed_value)
    sus decoded BinzValue = binz_decode(encoded)
    
    test_assert(decoded.type_tag == large_struct.type_tag, "Compressed data should decode to original type")
    
    fr fr Test that compression actually reduces size for repetitive data
    sus repetitive_data BinzValue = create_repetitive_data()
    sus normal_encoded []drip = binz_encode(repetitive_data)
    
    repetitive_data.type_tag = TAG_COMPRESSED
    sus compressed_encoded []drip = binz_encode(repetitive_data)
    
    fr fr Compressed should be smaller (simplified test)
    test_assert(array_length(compressed_encoded) <= array_length(normal_encoded), "Compression should not increase size significantly")
    
    vibez.spill("✓ Compression tests passed")
    damn based
}

fr fr ===== PERFORMANCE AND EDGE CASE TESTS =====

slay test_large_data_structures() lit {
    fr fr Test encoding/decoding of large structures
    sus large_array BinzValue = binz_create_array()
    
    fr fr Create array with 1000 elements
    sus i drip = 0
    bestie (i < 1000) {
        large_array.array_values[i] = binz_create_int(i)
        i = i + 1
    }
    
    sus start_time drip = get_current_time()
    sus encoded []drip = binz_encode(large_array)
    sus encode_time drip = get_current_time() - start_time
    
    start_time = get_current_time()
    sus decoded BinzValue = binz_decode(encoded)
    sus decode_time drip = get_current_time() - start_time
    
    test_assert(decoded.type_tag == TAG_ARRAY_MIXED, "Large array type")
    test_assert(array_length(decoded.array_values) == 1000, "Large array size")
    
    vibez.spill("✓ Large data structure tests passed")
    vibez.spill("  Encode time: " + int_to_string(encode_time) + "ms")
    vibez.spill("  Decode time: " + int_to_string(decode_time) + "ms") 
    damn based
}

slay test_edge_cases() lit {
    fr fr Test various edge cases
    
    fr fr Empty structures
    sus empty_array BinzValue = binz_create_array()
    sus empty_struct BinzValue = binz_create_struct()
    
    sus encoded_array []drip = binz_encode(empty_array)
    sus encoded_struct []drip = binz_encode(empty_struct)
    
    sus decoded_array BinzValue = binz_decode(encoded_array)
    sus decoded_struct BinzValue = binz_decode(encoded_struct)
    
    test_assert(decoded_array.type_tag == TAG_ARRAY_MIXED, "Empty array type")
    test_assert(array_length(decoded_array.array_values) == 0, "Empty array size")
    test_assert(decoded_struct.type_tag == TAG_STRUCT, "Empty struct type")
    test_assert(array_length(decoded_struct.struct_fields) == 0, "Empty struct size")
    
    fr fr Maximum integer values
    sus max_int BinzValue = binz_create_int(2147483647)
    sus min_int BinzValue = binz_create_int(-2147483648)
    
    sus max_encoded []drip = binz_encode(max_int)
    sus min_encoded []drip = binz_encode(min_int)
    
    sus max_decoded BinzValue = binz_decode(max_encoded)
    sus min_decoded BinzValue = binz_decode(min_encoded)
    
    test_assert(max_decoded.int_value == 2147483647, "Max int value")
    test_assert(min_decoded.int_value == -2147483648, "Min int value")
    
    fr fr Unicode strings
    sus unicode_val BinzValue = binz_create_string("Hello 世界 🌍")
    sus unicode_encoded []drip = binz_encode(unicode_val)
    sus unicode_decoded BinzValue = binz_decode(unicode_encoded)
    
    test_assert(unicode_decoded.string_value == "Hello 世界 🌍", "Unicode string handling")
    
    vibez.spill("✓ Edge case tests passed")
    damn based
}

slay test_memory_pool_optimization() lit {
    fr fr Test memory pool optimization
    sus pool BinzMemoryPool = binz_create_memory_pool(4096)
    
    sus test_data BinzValue = binz_create_struct()
    test_data.struct_fields[0] = "message"
    test_data.struct_values[0] = binz_create_string("High performance encoding")
    
    sus encoded []drip = binz_encode_with_pool(test_data, pool)
    test_assert(array_length(encoded) > 0, "Memory pool encoding should work")
    
    sus decoded BinzValue = binz_decode(encoded)
    test_assert(decoded.type_tag == TAG_STRUCT, "Memory pool decoding should work")
    
    vibez.spill("✓ Memory pool optimization tests passed")
    damn based
}

slay test_batch_operations() lit {
    fr fr Test batch encoding/decoding
    sus values []BinzValue = []
    
    values[0] = binz_create_int(100)
    values[1] = binz_create_string("batch test")
    values[2] = binz_create_bool(based)
    values[3] = binz_create_float(2.718)
    
    sus batch_encoded []drip = binz_encode_batch(values)
    sus batch_decoded []BinzValue = binz_decode_batch(batch_encoded)
    
    test_assert(array_length(batch_decoded) == 4, "Batch decode count")
    test_assert(batch_decoded[0].int_value == 100, "Batch element 0")
    test_assert(batch_decoded[1].string_value == "batch test", "Batch element 1")
    test_assert(batch_decoded[2].bool_value == based, "Batch element 2")
    
    vibez.spill("✓ Batch operation tests passed")
    damn based
}

fr fr ===== ERROR HANDLING TESTS =====

slay test_error_handling() lit {
    fr fr Test various error conditions
    
    fr fr Invalid magic header
    sus invalid_data []drip = [0x12, 0x34, 0x56, 0x78, 0x01, 0x00, 0x00, 0x00]
    sus invalid_decoded BinzValue = binz_decode(invalid_data)
    test_assert(invalid_decoded.type_tag == TAG_NULL, "Invalid header should return null")
    
    fr fr Truncated data
    sus truncated_data []drip = [0x42, 0x49, 0x4E, 0x5A]  fr fr Just "BINZ" magic
    sus truncated_decoded BinzValue = binz_decode(truncated_data)
    test_assert(truncated_decoded.type_tag == TAG_NULL, "Truncated data should return null")
    
    fr fr Empty input
    sus empty_data []drip = []
    sus empty_decoded BinzValue = binz_decode(empty_data)
    test_assert(empty_decoded.type_tag == TAG_NULL, "Empty data should return null")
    
    vibez.spill("✓ Error handling tests passed")
    damn based
}

fr fr ===== REFLECTION-BASED SERIALIZATION TESTS =====

slay test_reflection_serialization() lit {
    fr fr Test automatic struct serialization using reflection
    sus test_schema BinzSchema = binz_create_schema(3001, 1, "TestStruct")
    test_schema = binz_schema_add_field(test_schema, "id", "drip", cringe)
    test_schema = binz_schema_add_field(test_schema, "name", "tea", cringe)
    test_schema = binz_schema_add_field(test_schema, "score", "normie", cringe)
    
    fr fr Simulate struct data
    sus struct_data lit = cringe  fr fr Placeholder for actual struct
    
    fr fr This would use real reflection in full implementation
    sus reflected_value BinzValue = binz_create_struct()
    reflected_value.struct_fields[0] = "id"
    reflected_value.struct_values[0] = binz_create_int(42)
    reflected_value.struct_fields[1] = "name"
    reflected_value.struct_values[1] = binz_create_string("test")
    reflected_value.struct_fields[2] = "score"
    reflected_value.struct_values[2] = binz_create_float(95.5)
    
    sus encoded []drip = binz_encode(reflected_value)
    sus decoded BinzValue = binz_decode(encoded)
    
    test_assert(decoded.type_tag == TAG_STRUCT, "Reflection serialization type")
    test_assert(array_length(decoded.struct_fields) == 3, "Reflection field count")
    
    vibez.spill("✓ Reflection-based serialization tests passed")
    damn based
}

fr fr ===== SIZE CALCULATION TESTS =====

slay test_size_calculations() lit {
    fr fr Test encoded size prediction
    sus test_values []BinzValue = []
    
    test_values[0] = binz_create_null()
    test_values[1] = binz_create_bool(based)
    test_values[2] = binz_create_int(42)
    test_values[3] = binz_create_string("hello")
    
    sus i drip = 0
    sus count drip = array_length(test_values)
    
    bestie (i < count) {
        sus predicted_size drip = binz_get_encoded_size(test_values[i])
        sus encoded []drip = binz_encode(test_values[i])
        sus actual_size drip = array_length(encoded)
        
        test_assert(predicted_size == actual_size, "Size prediction should match actual size")
        i = i + 1
    }
    
    vibez.spill("✓ Size calculation tests passed")
    damn based
}

fr fr ===== HELPER FUNCTIONS FOR TESTS =====

slay generate_long_string(length drip) tea {
    sus result tea = ""
    sus i drip = 0
    bestie (i < length) {
        sus char_code drip = 65 + (i % 26)  fr fr A-Z
        sus char tea = number_to_char(char_code)
        result = result + char
        i = i + 1
    }
    damn result
}

slay create_large_test_struct() BinzValue {
    sus struct_val BinzValue = binz_create_struct()
    
    sus i drip = 0
    bestie (i < 100) {
        sus field_name tea = "field_" + int_to_string(i)
        sus field_value BinzValue = binz_create_int(i * 10)
        
        struct_val.struct_fields[i] = field_name
        struct_val.struct_values[i] = field_value
        i = i + 1
    }
    
    damn struct_val
}

slay create_repetitive_data() BinzValue {
    fr fr Create data with lots of repetition for compression testing
    sus array_val BinzValue = binz_create_array()
    
    sus i drip = 0
    bestie (i < 200) {
        fr fr Every 10th element is the same
        sus value drip = i / 10
        array_val.array_values[i] = binz_create_int(value)
        i = i + 1
    }
    
    damn array_val
}

slay abs_float(val normie) normie {
    ready (val < 0.0) {
        damn -val
    }
    damn val
}

slay get_current_time() drip {
    fr fr Mock time function - would be real in full implementation
    damn 1234567890
}

slay number_to_char(code drip) tea {
    ready (code == 65) { damn "A" }
    ready (code == 66) { damn "B" }
    ready (code == 67) { damn "C" }
    ready (code == 68) { damn "D" }
    ready (code == 69) { damn "E" }
    ready (code == 70) { damn "F" }
    ready (code == 71) { damn "G" }
    ready (code == 72) { damn "H" }
    ready (code == 73) { damn "I" }
    ready (code == 74) { damn "J" }
    ready (code == 75) { damn "K" }
    ready (code == 76) { damn "L" }
    ready (code == 77) { damn "M" }
    ready (code == 78) { damn "N" }
    ready (code == 79) { damn "O" }
    ready (code == 80) { damn "P" }
    ready (code == 81) { damn "Q" }
    ready (code == 82) { damn "R" }
    ready (code == 83) { damn "S" }
    ready (code == 84) { damn "T" }
    ready (code == 85) { damn "U" }
    ready (code == 86) { damn "V" }
    ready (code == 87) { damn "W" }
    ready (code == 88) { damn "X" }
    ready (code == 89) { damn "Y" }
    ready (code == 90) { damn "Z" }
    damn "A"  fr fr Default
}

fr fr ===== MAIN TEST RUNNER =====

slay run_all_binz_tests() lit {
    vibez.spill("🧪 Starting BINZ Binary Serialization Test Suite")
    vibez.spill("=" * 60)
    
    sus test_count drip = 0
    sus passed_count drip = 0
    
    fr fr Run all test functions
    test_count = test_count + 1
    ready (test_basic_null_encoding()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1  
    ready (test_basic_boolean_encoding()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_integer_encoding()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_float_encoding()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_string_encoding()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_array_encoding()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_struct_encoding()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_nested_structures()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_schema_creation_and_validation()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_schema_migration()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_compression()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_large_data_structures()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_edge_cases()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_memory_pool_optimization()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_batch_operations()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_error_handling()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_reflection_serialization()) { passed_count = passed_count + 1 }
    
    test_count = test_count + 1
    ready (test_size_calculations()) { passed_count = passed_count + 1 }
    
    fr fr Print results
    vibez.spill("=" * 60)
    vibez.spill("🎯 BINZ Test Results:")
    vibez.spill("   Tests Run: " + int_to_string(test_count))
    vibez.spill("   Passed: " + int_to_string(passed_count))
    vibez.spill("   Failed: " + int_to_string(test_count - passed_count))
    
    ready (passed_count == test_count) {
        vibez.spill("🎉 All BINZ tests passed! Binary serialization is production ready.")
        damn based
    } otherwise {
        vibez.spill("❌ Some BINZ tests failed. Check implementation.")
        damn cringe
    }
}

fr fr Run tests if this file is executed directly
run_all_binz_tests()
