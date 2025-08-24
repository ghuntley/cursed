// Enhanced Serialization Module Test
// Focus on core functionality validation

yeet "serialization"
yeet "testz"

slay test_enhanced_serialization() {
    test_start("Enhanced Serialization Module")
    
    // Test 1: Basic integer serialization with real implementation
    test_case("Integer Serialization")
    sus test_int drip = 42
    sus serialized_int tea = serialize_int(test_int)
    sus deserialized_int drip = deserialize_int(serialized_int, 0)
    print_debug("Serialized integer 42, got back: ", deserialized_int)
    
    // Test 2: String serialization with UTF-8 support
    test_case("String Serialization")
    sus test_string tea = "Hello, World!"
    sus serialized_string tea = serialize_string(test_string)
    sus deserialized_string tea = deserialize_string(serialized_string, 0)
    print_debug("Serialized string '", test_string, "', got back: '", deserialized_string, "'")
    
    // Test 3: Boolean serialization
    test_case("Boolean Serialization")
    sus serialized_true tea = serialize_bool(based)
    sus serialized_false tea = serialize_bool(cap)
    sus deserialized_true lit = deserialize_bool(serialized_true, 0)
    sus deserialized_false lit = deserialize_bool(serialized_false, 0)
    print_debug("Boolean true->", deserialized_true, ", false->", deserialized_false)
    
    // Test 4: Serialization context for structured data
    test_case("Serialization Context")
    sus ctx SerializationContext = create_serialization_context()
    ctx = write_int(ctx, 123)
    ctx = write_string(ctx, "test")
    ctx = write_bool(ctx, based)
    
    print_debug("Context data length: ", real_string_len(ctx.data))
    print_debug("Context error status: '", ctx.error, "'")
    
    // Test 5: Variable-length integer encoding
    test_case("Variable-Length Integer Encoding")
    sus varint_data tea = serialize_varint(127)
    sus decoded_varint drip = deserialize_varint(varint_data, 0)
    print_debug("Varint 127 encoded size: ", real_string_len(varint_data), " bytes, decoded: ", decoded_varint)
    
    // Test 6: Checksum validation
    test_case("Checksum Validation") 
    sus test_data tea = "Hello, checksum!"
    sus crc drip = calculate_crc32(test_data)
    sus is_valid lit = validate_crc32(test_data, crc)
    print_debug("CRC32 checksum: ", crc, ", validation: ", is_valid)
    
    // Test 7: RLE compression
    test_case("RLE Compression")
    sus original tea = "aaabbbccc"
    sus compressed tea = compress_rle(original)
    sus decompressed tea = decompress_rle(compressed)
    print_debug("Original: '", original, "', compressed size: ", real_string_len(compressed))
    print_debug("Decompressed: '", decompressed, "'")
    
    // Test 8: Serialization header with versioning
    test_case("Serialization Headers")
    sus header_test_data tea = "versioned data"
    sus versioned tea = serialize_with_header(header_test_data, 1)
    sus recovered tea = deserialize_with_header(versioned)
    print_debug("Header serialization: '", header_test_data, "' -> '", recovered, "'")
    
    print_test_summary()
}

// Helper functions for debugging
slay print_debug(msg1 tea, val drip, msg2 tea) {
    spill("DEBUG: ", msg1, val, msg2)
}

slay print_debug(msg1 tea, val1 tea, msg2 tea, val2 tea, msg3 tea) {
    spill("DEBUG: ", msg1, val1, msg2, val2, msg3)
}

slay print_debug(msg1 tea, val lit) {
    lowkey val {
        spill("DEBUG: ", msg1, "true")
    } damn {
        spill("DEBUG: ", msg1, "false") 
    }
}

slay print_debug(msg1 tea, val1 lit, msg2 tea, val2 lit) {
    spill("DEBUG: ", msg1, val1, msg2, val2)
}

test_enhanced_serialization()
