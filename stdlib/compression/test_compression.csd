// CURSED Compression Module Tests
// Comprehensive test suite for data compression algorithms

yeet "testz"
yeet "compression"

slay test_all_compression() {
    test_start("Compression Module Tests")
    
    test_rle_compression()
    test_lz77_compression()
    test_dictionary_compression()
    test_frequency_compression()
    test_varint_encoding()
    test_compression_utilities()
    test_auto_compression()
    test_compression_ratios()
    
    print_test_summary()
}

slay test_rle_compression() {
    test_start("Run-Length Encoding")
    
    // Basic RLE compression
    sus compressed1 tea = compression.rle_compress("aaabbbccc")
    sus decompressed1 tea = compression.rle_decompress(compressed1)
    assert_eq_string(decompressed1, "aaabbbccc")
    
    // Single character
    sus compressed2 tea = compression.rle_compress("a")
    sus decompressed2 tea = compression.rle_decompress(compressed2)
    assert_eq_string(decompressed2, "a")
    
    // Mixed characters
    sus compressed3 tea = compression.rle_compress("aabbccdd")
    sus decompressed3 tea = compression.rle_decompress(compressed3)
    assert_eq_string(decompressed3, "aabbccdd")
    
    // Empty string
    sus compressed4 tea = compression.rle_compress("")
    sus decompressed4 tea = compression.rle_decompress(compressed4)
    assert_eq_string(decompressed4, "")
    
    // Long runs
    sus compressed5 tea = compression.rle_compress("aaaaaaaaaaaa")
    sus decompressed5 tea = compression.rle_decompress(compressed5)
    assert_eq_string(decompressed5, "aaaaaaaaaaaa")
}

slay test_lz77_compression() {
    test_start("LZ77 Compression")
    
    // Basic LZ77 compression
    sus compressed1 tea = compression.lz77_compress("hello world hello")
    sus decompressed1 tea = compression.lz77_decompress(compressed1)
    assert_eq_string(decompressed1, "hello world hello")
    
    // Repeated patterns
    sus compressed2 tea = compression.lz77_compress("abcabcabc")
    sus decompressed2 tea = compression.lz77_decompress(compressed2)
    assert_eq_string(decompressed2, "abcabcabc")
    
    // Single character
    sus compressed3 tea = compression.lz77_compress("a")
    sus decompressed3 tea = compression.lz77_decompress(compressed3)
    assert_eq_string(decompressed3, "a")
    
    // Empty string
    sus compressed4 tea = compression.lz77_compress("")
    sus decompressed4 tea = compression.lz77_decompress(compressed4)
    assert_eq_string(decompressed4, "")
    
    // No repetition
    sus compressed5 tea = compression.lz77_compress("abcdefg")
    sus decompressed5 tea = compression.lz77_decompress(compressed5)
    assert_eq_string(decompressed5, "abcdefg")
}

slay test_dictionary_compression() {
    test_start("Dictionary Compression")
    
    // Basic dictionary compression
    sus test_data tea = "hello world hello world"
    sus compressed tea = compression.dictionary_compress(test_data)
    sus dictionary [tea] = compression.build_dictionary(test_data)
    sus decompressed tea = compression.dictionary_decompress(compressed, dictionary)
    assert_eq_string(decompressed, test_data)
    
    // Build dictionary
    sus dict [tea] = compression.build_dictionary("abcabc")
    assert_true(len(dict) > 0)
    assert_true(compression.contains_phrase(dict, "ab"))
    assert_true(compression.contains_phrase(dict, "bc"))
    
    // Find dictionary matches
    sus match tea = compression.find_longest_dictionary_match("hello", 0, ["he", "hel", "hello"])
    assert_eq_string(match, "hello")
    
    // Dictionary index
    sus index normie = compression.find_dictionary_index(["hello", "world"], "world")
    assert_eq_int(index, 1)
    
    sus no_index normie = compression.find_dictionary_index(["hello", "world"], "test")
    assert_eq_int(no_index, -1)
}

slay test_frequency_compression() {
    test_start("Frequency Compression")
    
    // Build frequency map
    sus freq_map map[tea]normie = compression.build_frequency_map("hello")
    assert_eq_int(compression.get_frequency(freq_map, "l"), 2)
    assert_eq_int(compression.get_frequency(freq_map, "h"), 1)
    assert_eq_int(compression.get_frequency(freq_map, "e"), 1)
    assert_eq_int(compression.get_frequency(freq_map, "o"), 1)
    
    // Frequency compression
    sus compressed tea = compression.frequency_compress("hello world")
    assert_true(string_len(compressed) > 0)
    
    // Character encoding
    sus encoding_map map[tea]tea = compression.build_simple_encoding(freq_map)
    sus encoding tea = compression.get_encoding(encoding_map, "l")
    assert_eq_string(encoding, "0")  // Placeholder assertion
}

slay test_varint_encoding() {
    test_start("Variable Integer Encoding")
    
    // Small numbers
    sus varint1 tea = compression.serialize_varint(0)
    sus decoded1 normie = compression.deserialize_varint(varint1, 0)
    assert_eq_int(decoded1, 0)
    
    sus varint2 tea = compression.serialize_varint(127)
    sus decoded2 normie = compression.deserialize_varint(varint2, 0)
    assert_eq_int(decoded2, 127)
    
    // Large numbers
    sus varint3 tea = compression.serialize_varint(16384)
    sus decoded3 normie = compression.deserialize_varint(varint3, 0)
    assert_eq_int(decoded3, 16384)
    
    // Varint size calculation
    assert_eq_int(compression.varint_size(0), 1)
    assert_eq_int(compression.varint_size(127), 1)
    assert_eq_int(compression.varint_size(128), 2)
    assert_eq_int(compression.varint_size(16384), 3)
}

slay test_compression_utilities() {
    test_start("Compression Utilities")
    
    // Math utilities
    assert_eq_int(compression.max(5, 3), 5)
    assert_eq_int(compression.max(3, 5), 5)
    assert_eq_int(compression.min(5, 3), 3)
    assert_eq_int(compression.min(3, 5), 3)
    
    // String/int conversion
    assert_eq_string(compression.int_to_string(0), "0")
    assert_eq_string(compression.int_to_string(123), "123")
    assert_eq_string(compression.int_to_string(-42), "-42")
    
    assert_eq_int(compression.string_to_int("0"), 0)
    assert_eq_int(compression.string_to_int("123"), 123)
    assert_eq_int(compression.string_to_int("-42"), -42)
    
    // Character utilities
    assert_true(compression.is_digit("5"))
    assert_false(compression.is_digit("a"))
    assert_eq_int(compression.char_to_digit("5"), 5)
    assert_eq_int(compression.char_to_digit("0"), 0)
    
    // String utilities
    assert_eq_int(compression.string_index_of("hello", "l", 0), 2)
    assert_eq_int(compression.string_index_of("hello", "x", 0), -1)
    assert_true(compression.string_starts_with("hello", "hel"))
    assert_false(compression.string_starts_with("hello", "world"))
}

slay test_auto_compression() {
    test_start("Auto Compression")
    
    // Auto-detect best compression
    sus test_data tea = "hello world hello world"
    sus compressed tea = compression.auto_compress(test_data)
    sus decompressed tea = compression.auto_decompress(compressed)
    assert_eq_string(decompressed, test_data)
    
    // Check compression prefixes
    assert_true(compression.string_starts_with(compressed, "RLE:") || 
                compression.string_starts_with(compressed, "LZ77:") || 
                compression.string_starts_with(compressed, "DICT:"))
    
    // Auto-decompress different formats
    sus rle_data tea = "RLE:3a2b"
    sus rle_result tea = compression.auto_decompress(rle_data)
    assert_eq_string(rle_result, "aaabb")
    
    sus lz77_data tea = "LZ77:hello[5,1]world"
    sus lz77_result tea = compression.auto_decompress(lz77_data)
    assert_true(string_len(lz77_result) > 0)
}

slay test_compression_ratios() {
    test_start("Compression Ratios")
    
    // Calculate compression ratio
    sus ratio meal = compression.compression_ratio("hello world", "hlo wrd")
    assert_true(ratio > 0.0 && ratio < 1.0)
    
    // Calculate savings
    sus savings meal = compression.calculate_savings("hello world", "hlo wrd")
    assert_true(savings > 0.0 && savings < 100.0)
    
    // Perfect compression
    sus perfect_ratio meal = compression.compression_ratio("aaaa", "4a")
    assert_true(perfect_ratio == 0.5)
    
    // No compression
    sus no_ratio meal = compression.compression_ratio("abcd", "abcd")
    assert_true(no_ratio == 1.0)
    
    // Empty string edge case
    sus empty_ratio meal = compression.compression_ratio("", "")
    assert_true(empty_ratio == 0.0)
}

// Message serialization tests
slay test_message_serialization() {
    test_start("Message Serialization")
    
    // Create and serialize message
    sus message compression.Message = compression.Message{
        field_id: 1,
        field_type: 2,
        data: "test data"
    }
    
    sus serialized tea = compression.serialize_message(message)
    sus deserialized compression.Message = compression.deserialize_message(serialized, 0)
    
    assert_eq_int(deserialized.field_id, 1)
    assert_eq_int(deserialized.field_type, 2)
    assert_eq_string(deserialized.data, "test data")
}

// Checksum tests
slay test_checksums() {
    test_start("Checksums")
    
    // Calculate checksum
    sus checksum1 normie = compression.calculate_checksum("hello")
    sus checksum2 normie = compression.calculate_checksum("hello")
    assert_eq_int(checksum1, checksum2)
    
    // Different strings should have different checksums
    sus checksum3 normie = compression.calculate_checksum("world")
    assert_true(checksum1 != checksum3)
    
    // Validate checksum
    assert_true(compression.validate_checksum("hello", checksum1))
    assert_false(compression.validate_checksum("hello", checksum3))
    
    // Serialize with checksum
    sus with_checksum tea = compression.serialize_with_checksum("test data")
    sus without_checksum tea = compression.deserialize_with_checksum(with_checksum)
    assert_eq_string(without_checksum, "test data")
}

// Run all tests
test_all_compression()
