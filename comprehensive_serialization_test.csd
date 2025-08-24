// Comprehensive Serialization Module Test
// Tests all enhanced serialization functionality

yeet "serialization"
yeet "testz"

slay test_basic_integer_serialization() lit {
    // Test basic integer serialization/deserialization
    sus test_values []drip = [0, 1, -1, 42, -42, 1234567890, -1234567890]
    
    bestie i := 0; i < 7; i++ {
        sus original drip = test_values[i]
        sus serialized tea = serialize_int(original)
        sus deserialized drip = deserialize_int(serialized, 0)
        
        lowkey deserialized != original {
            damn cap
        }
    }
    
    damn based
}

slay test_long_serialization() lit {
    // Test 64-bit long serialization
    sus test_values []thicc = [0, 1, -1, 9223372036854775807, -9223372036854775808]
    
    bestie i := 0; i < 5; i++ {
        sus original thicc = test_values[i]
        sus serialized tea = serialize_long(original)
        sus deserialized thicc = deserialize_long(serialized, 0)
        
        lowkey deserialized != original {
            damn cap
        }
    }
    
    damn based
}

slay test_float_serialization() lit {
    // Test float serialization with IEEE 754
    sus test_values []meal = [0.0, 1.0, -1.0, 3.14159, -2.71828, 1.23456e10, -9.87654e-10]
    
    bestie i := 0; i < 7; i++ {
        sus original meal = test_values[i]
        sus serialized tea = serialize_float(original)
        sus deserialized meal = deserialize_float(serialized, 0)
        
        // Allow small floating point precision differences
        sus diff meal = original - deserialized
        lowkey diff < 0.0 {
            diff = -diff
        }
        
        lowkey diff > 0.001 {
            damn cap
        }
    }
    
    damn based
}

slay test_string_serialization() lit {
    // Test UTF-8 string serialization
    sus test_strings []tea = ["", "hello", "world", "Hello, 世界!", "🚀 CURSED", "Special chars: \n\t\r"]
    
    bestie i := 0; i < 6; i++ {
        sus original tea = test_strings[i]
        sus serialized tea = serialize_string(original)
        sus deserialized tea = deserialize_string(serialized, 0)
        
        lowkey deserialized != original {
            damn cap
        }
    }
    
    damn based
}

slay test_boolean_serialization() lit {
    // Test boolean serialization
    sus serialized_true tea = serialize_bool(based)
    sus serialized_false tea = serialize_bool(cap)
    
    sus deserialized_true lit = deserialize_bool(serialized_true, 0)
    sus deserialized_false lit = deserialize_bool(serialized_false, 0)
    
    lowkey !deserialized_true || deserialized_false {
        damn cap
    }
    
    damn based
}

slay test_array_serialization() lit {
    // Test integer array serialization
    sus original_ints []drip = [1, 2, 3, 42, -10, 0, 999]
    sus serialized_ints tea = serialize_array_int(original_ints)
    sus deserialized_ints []drip = deserialize_array_int(serialized_ints, 0)
    
    // Test string array serialization
    sus original_strings []tea = ["hello", "world", "test", ""]
    sus serialized_strings tea = serialize_array_string(original_strings)
    sus deserialized_strings []tea = deserialize_array_string(serialized_strings, 0)
    
    // Basic validation (would compare arrays in full implementation)
    lowkey array_len_int(deserialized_ints) != array_len_int(original_ints) {
        damn cap
    }
    
    lowkey array_len_string(deserialized_strings) != array_len_string(original_strings) {
        damn cap
    }
    
    damn based
}

slay test_serialization_context() lit {
    // Test structured serialization context
    sus ctx SerializationContext = create_serialization_context()
    
    // Write multiple values
    ctx = write_int(ctx, 42)
    ctx = write_string(ctx, "hello")
    ctx = write_bool(ctx, based)
    ctx = write_float(ctx, 3.14)
    
    // Check no errors occurred
    lowkey ctx.error != "" {
        damn cap
    }
    
    // Create reading context
    sus read_ctx SerializationContext = SerializationContext{
        data: ctx.data,
        offset: 0,
        error: "",
        checksum_enabled: cap
    }
    
    // Read values back
    sus read_int drip = read_int(read_ctx)
    sus read_string tea = read_string(read_ctx)
    sus read_bool lit = read_bool(read_ctx)
    sus read_float meal = read_float(read_ctx)
    
    // Validate values
    lowkey read_int != 42 || read_string != "hello" || !read_bool {
        damn cap
    }
    
    sus float_diff meal = read_float - 3.14
    lowkey float_diff < 0.0 {
        float_diff = -float_diff
    }
    lowkey float_diff > 0.01 {
        damn cap
    }
    
    damn based
}

slay test_varint_encoding() lit {
    // Test variable-length integer encoding
    sus test_values []drip = [0, 1, 127, 128, 255, 16383, 16384, -1, -127, -128]
    
    bestie i := 0; i < 10; i++ {
        sus original drip = test_values[i]
        sus serialized tea = serialize_varint(original)
        sus deserialized drip = deserialize_varint(serialized, 0)
        
        lowkey deserialized != original {
            damn cap
        }
        
        // Check that small values use fewer bytes
        lowkey original >= 0 && original < 128 {
            lowkey real_string_len(serialized) != 1 {
                damn cap
            }
        }
    }
    
    damn based
}

slay test_checksum_validation() lit {
    // Test CRC32 checksum validation
    sus test_data tea = "Hello, World! This is test data for CRC32."
    sus crc drip = calculate_crc32(test_data)
    
    // Validate checksum with correct data
    lowkey !validate_crc32(test_data, crc) {
        damn cap
    }
    
    // Test serialization with checksum
    sus serialized_with_crc tea = serialize_with_crc32(test_data)
    sus deserialized_data tea = deserialize_with_crc32(serialized_with_crc)
    
    lowkey deserialized_data != test_data {
        damn cap
    }
    
    damn based
}

slay test_rle_compression() lit {
    // Test Run-Length Encoding compression
    sus test_data tea = "aaabbbccccdddddeeeeeeffffff"
    sus compressed tea = compress_rle(test_data)
    sus decompressed tea = decompress_rle(compressed)
    
    lowkey decompressed != test_data {
        damn cap
    }
    
    // Compression should reduce size for repetitive data
    lowkey real_string_len(compressed) >= real_string_len(test_data) {
        damn cap
    }
    
    damn based
}

slay test_lz77_compression() lit {
    // Test LZ77-style compression
    sus test_data tea = "abcdefghijklmnopqrstuvwxyz"
    sus repeated_data tea = test_data + test_data + test_data
    
    sus compressed tea = compress_lz77(repeated_data)
    sus decompressed tea = decompress_lz77(compressed)
    
    lowkey decompressed != repeated_data {
        damn cap
    }
    
    // Should achieve some compression on repeated data
    lowkey real_string_len(compressed) >= real_string_len(repeated_data) {
        damn cap
    }
    
    damn based
}

slay test_serialization_header() lit {
    // Test header-based serialization with versioning
    sus test_data tea = "This is versioned serialization data"
    sus version drip = SERIALIZATION_VERSION_CURRENT
    
    sus serialized tea = serialize_with_header(test_data, version)
    sus deserialized tea = deserialize_with_header(serialized)
    
    lowkey deserialized != test_data {
        damn cap
    }
    
    // Test header validation
    sus header SerializationHeader = deserialize_header(serialized, 0)
    lowkey !validate_header(header) {
        damn cap
    }
    
    lowkey header.version != version {
        damn cap
    }
    
    damn based
}

slay test_error_handling() lit {
    // Test error handling in deserialization
    sus invalid_data tea = "xx"  // Too short for integer
    sus result drip = deserialize_int(invalid_data, 0)
    
    // Should handle gracefully (return 0 for invalid data)
    lowkey result != 0 {
        damn cap
    }
    
    // Test context error handling
    sus ctx SerializationContext = SerializationContext{
        data: invalid_data,
        offset: 0,
        error: "",
        checksum_enabled: cap
    }
    
    sus read_result drip = read_int(ctx)
    lowkey ctx.error == "" {
        damn cap  // Should have set an error
    }
    
    damn based
}

slay test_binary_format_integrity() lit {
    // Test that binary format is consistent across different value types
    sus mixed_data SerializationContext = create_serialization_context()
    
    // Write mixed data types
    mixed_data = write_int(mixed_data, 0x12345678)
    mixed_data = write_string(mixed_data, "test")
    mixed_data = write_bool(mixed_data, based)
    mixed_data = write_float(mixed_data, 1.5)
    mixed_data = write_long(mixed_data, 0x123456789ABCDEF0)
    
    // Verify binary structure by checking known patterns
    sus binary_data tea = mixed_data.data
    
    // Integer should be first 4 bytes (little-endian)
    sus first_int drip = deserialize_int(binary_data, 0)
    lowkey first_int != 0x12345678 {
        damn cap
    }
    
    // String length should be next 4 bytes (4 for "test")
    sus string_len drip = deserialize_int(binary_data, 4)
    lowkey string_len != 4 {
        damn cap
    }
    
    damn based
}

slay test_large_data_handling() lit {
    // Test handling of larger data structures
    sus large_string tea = "Large test data: "
    bestie i := 0; i < 100; i++ {
        large_string = large_string + "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    }
    
    // Test serialization of large string
    sus serialized tea = serialize_string(large_string)
    sus deserialized tea = deserialize_string(serialized, 0)
    
    lowkey deserialized != large_string {
        damn cap
    }
    
    // Test compression on large data
    sus compressed tea = compress_rle(large_string)
    sus decompressed tea = decompress_rle(compressed)
    
    lowkey decompressed != large_string {
        damn cap
    }
    
    damn based
}

// Run all tests
slay run_comprehensive_serialization_tests() lit {
    test_start("Comprehensive Serialization Tests")
    
    test_group("Basic Type Serialization")
    assert_true(test_basic_integer_serialization(), "Integer serialization")
    assert_true(test_long_serialization(), "Long serialization") 
    assert_true(test_float_serialization(), "Float serialization")
    assert_true(test_string_serialization(), "String serialization")
    assert_true(test_boolean_serialization(), "Boolean serialization")
    
    test_group("Array Serialization")
    assert_true(test_array_serialization(), "Array serialization")
    
    test_group("Structured Serialization")
    assert_true(test_serialization_context(), "Serialization context")
    
    test_group("Advanced Encoding")
    assert_true(test_varint_encoding(), "Variable-length integer encoding")
    
    test_group("Data Integrity")
    assert_true(test_checksum_validation(), "Checksum validation")
    
    test_group("Compression")
    assert_true(test_rle_compression(), "Run-length encoding compression")
    assert_true(test_lz77_compression(), "LZ77-style compression")
    
    test_group("Versioning and Headers")
    assert_true(test_serialization_header(), "Serialization headers")
    
    test_group("Error Handling")
    assert_true(test_error_handling(), "Error handling")
    
    test_group("Format Integrity")
    assert_true(test_binary_format_integrity(), "Binary format integrity")
    
    test_group("Performance")
    assert_true(test_large_data_handling(), "Large data handling")
    
    print_test_summary()
    damn based
}

// Performance benchmark
slay benchmark_serialization_performance() {
    test_start("Serialization Performance Benchmarks")
    
    // Benchmark integer array serialization
    sus large_int_array []drip = []
    bestie i := 0; i < 10000; i++ {
        large_int_array = append_int_array(large_int_array, i * 42)
    }
    
    benchmark_start("Large integer array serialization")
    sus serialized_array tea = serialize_array_int(large_int_array)
    benchmark_end()
    
    benchmark_start("Large integer array deserialization")
    sus deserialized_array []drip = deserialize_array_int(serialized_array, 0)
    benchmark_end()
    
    // Benchmark string operations
    sus test_string tea = "Performance test string with moderate length for benchmarking"
    
    benchmark_start("String serialization (1000 iterations)")
    bestie i := 0; i < 1000; i++ {
        sus temp tea = serialize_string(test_string)
    }
    benchmark_end()
    
    // Benchmark compression
    sus repeated_data tea = ""
    bestie i := 0; i < 1000; i++ {
        repeated_data = repeated_data + "ABCDEFGHIJKLMNOP"
    }
    
    benchmark_start("RLE compression")
    sus compressed tea = compress_rle(repeated_data)
    benchmark_end()
    
    benchmark_start("RLE decompression")
    sus decompressed tea = decompress_rle(compressed)
    benchmark_end()
    
    print_benchmark_summary()
}

// Main test execution
run_comprehensive_serialization_tests()
benchmark_serialization_performance()
