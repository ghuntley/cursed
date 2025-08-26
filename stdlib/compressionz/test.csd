yeet "testz"
yeet "compressionz"

test_start("compressionz Tests")

// Test LZ4 Enhanced Compression
test_case("LZ4 Enhanced Compression - Fast Level") {
    sus input tea = "Hello, CURSED compression world!"
    sus compressed tea = lz4_compress_enhanced(input, COMPRESS_LEVEL_FASTEST)
    sus decompressed tea = lz4_decompress_enhanced(compressed)
    
    assert_eq_string(input, decompressed)
    assert(string_starts_with(compressed, "LZ4E:FAST:"))
}

test_case("LZ4 Enhanced Compression - Best Level") {
    sus input tea = "This is a test string for best compression level"
    sus compressed tea = lz4_compress_enhanced(input, COMPRESS_LEVEL_BEST)
    sus decompressed tea = lz4_decompress_enhanced(compressed)
    
    assert_eq_string(input, decompressed)
    assert(string_starts_with(compressed, "LZ4E:BEST:"))
}

test_case("LZ4 Enhanced Compression - Balanced Level") {
    sus input tea = "Balanced compression testing"
    sus compressed tea = lz4_compress_enhanced(input, COMPRESS_LEVEL_BALANCED)
    sus decompressed tea = lz4_decompress_enhanced(compressed)
    
    assert_eq_string(input, decompressed)
    assert(string_starts_with(compressed, "LZ4E:BAL:"))
}

// Test GZIP Enhanced Compression
test_case("GZIP Enhanced Compression") {
    sus input tea = "GZIP compression test data"
    sus compressed tea = gzip_compress_enhanced(input, COMPRESS_LEVEL_BALANCED)
    sus decompressed tea = gzip_decompress_enhanced(compressed)
    
    assert_eq_string(input, decompressed)
    assert(string_starts_with(compressed, "GZIPE:"))
}

// Test ZSTD Enhanced Compression
test_case("ZSTD Enhanced Compression") {
    sus input tea = "ZSTD modern compression algorithm"
    sus compressed tea = zstd_compress_enhanced(input, COMPRESS_LEVEL_BALANCED)
    sus decompressed tea = zstd_decompress_enhanced(compressed)
    
    assert_eq_string(input, decompressed)
    assert(string_starts_with(compressed, "ZSTDE:"))
}

// Test Edge Cases
test_case("Empty String Compression") {
    sus input tea = ""
    sus compressed tea = lz4_compress_enhanced(input, COMPRESS_LEVEL_FAST)
    sus decompressed tea = lz4_decompress_enhanced(compressed)
    
    assert_eq_string(input, decompressed)
}

test_case("Small String Compression") {
    sus input tea = "Hi"
    sus compressed tea = lz4_compress_enhanced(input, COMPRESS_LEVEL_FAST)
    sus decompressed tea = lz4_decompress_enhanced(compressed)
    
    assert_eq_string(input, decompressed)
    assert(string_starts_with(compressed, "LZ4E:"))
}

// Performance Test
test_case("Large String Compression Performance") {
    sus large_input tea = string_repeat("CURSED compression benchmark data! ", 1000)
    
    sus start_time drip = get_current_time_ms()
    sus compressed tea = lz4_compress_enhanced(large_input, COMPRESS_LEVEL_FAST)
    sus compress_time drip = get_current_time_ms() - start_time
    
    start_time = get_current_time_ms()
    sus decompressed tea = lz4_decompress_enhanced(compressed)
    sus decompress_time drip = get_current_time_ms() - start_time
    
    assert_eq_string(large_input, decompressed)
    print_test_status("Large string compression time: " + string_from_int(compress_time) + "ms")
    print_test_status("Large string decompression time: " + string_from_int(decompress_time) + "ms")
    
    // Verify compression efficiency
    sus original_size drip = len(large_input)
    sus compressed_size drip = len(compressed)
    sus compression_ratio drip = (original_size - compressed_size) * 100 / original_size
    print_test_status("Compression ratio: " + string_from_int(compression_ratio) + "%")
}

// Test Compression Algorithm Selection
test_case("Best Compression Algorithm Selection") {
    sus input tea = "Testing different compression algorithms for efficiency"
    
    sus lz4_compressed tea = lz4_compress_enhanced(input, COMPRESS_LEVEL_BALANCED)
    sus gzip_compressed tea = gzip_compress_enhanced(input, COMPRESS_LEVEL_BALANCED)
    sus zstd_compressed tea = zstd_compress_enhanced(input, COMPRESS_LEVEL_BALANCED)
    
    sus best_algo tea = select_best_compression_algorithm(input)
    
    assert(best_algo == "LZ4" || best_algo == "GZIP" || best_algo == "ZSTD")
    print_test_status("Best algorithm for test data: " + best_algo)
}

print_test_summary()
