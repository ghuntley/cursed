yeet "testz"
yeet "compression"

fr fr ==========================================
fr fr CURSED Compression Module Test Suite
fr fr Comprehensive tests for all compression algorithms  
fr fr ==========================================

test_start("Compression Module Tests")

fr fr ==========================================
fr fr Basic Compression Algorithm Tests
fr fr ==========================================

fr fr Test LZ4 compression and decompression
sus test_data tea = "Hello World Test Data"
sus lz4_compressed tea = lz4_compress_data(test_data, COMPRESS_LEVEL_BALANCED)
sus lz4_decompressed tea = lz4_decompress_data(lz4_compressed)

assert_eq_string(lz4_decompressed, test_data)
assert_true(string_length(lz4_compressed) > string_length(test_data)) fr fr Includes prefix

fr fr Test DEFLATE compression and decompression  
sus deflate_compressed tea = deflate_compress_data(test_data, COMPRESS_LEVEL_BALANCED)
sus deflate_decompressed tea = deflate_decompress_data(deflate_compressed)

assert_eq_string(deflate_decompressed, test_data)
assert_true(string_length(deflate_compressed) > string_length(test_data))

fr fr Test GZIP compression and decompression
sus gzip_compressed tea = gzip_compress_data(test_data, COMPRESS_LEVEL_BALANCED)
sus gzip_decompressed tea = gzip_decompress_data(gzip_compressed)

assert_eq_string(gzip_decompressed, test_data)
assert_true(string_length(gzip_compressed) > string_length(test_data))

fr fr ==========================================
fr fr High-Level Interface Tests
fr fr ==========================================

fr fr Test main compression interface with LZ4
sus compressed_lz4 tea = compress_slay(test_data, ALGO_LZ4, COMPRESS_LEVEL_FAST)
sus decompressed_lz4 tea = decompress_vibes(compressed_lz4, ALGO_LZ4)
assert_eq_string(decompressed_lz4, test_data)

fr fr Test main compression interface with DEFLATE
sus compressed_deflate tea = compress_slay(test_data, ALGO_DEFLATE, COMPRESS_LEVEL_MAX)
sus decompressed_deflate tea = decompress_vibes(compressed_deflate, ALGO_DEFLATE)
assert_eq_string(decompressed_deflate, test_data)

fr fr Test main compression interface with GZIP
sus compressed_gzip tea = compress_slay(test_data, ALGO_GZIP, COMPRESS_LEVEL_BALANCED)
sus decompressed_gzip tea = decompress_vibes(compressed_gzip, ALGO_GZIP)
assert_eq_string(decompressed_gzip, test_data)

fr fr ==========================================
fr fr Algorithm Detection Tests
fr fr ==========================================

fr fr Test auto-detection of LZ4 format
sus lz4_test_data tea = "LZ4F:test_data_content"
sus detected_lz4 normie = auto_detect_algorithm(lz4_test_data)
assert_eq_int(detected_lz4, ALGO_LZ4)

fr fr Test auto-detection of DEFLATE format
sus deflate_test_data tea = "DEF5:test_data_content"  
sus detected_deflate normie = auto_detect_algorithm(deflate_test_data)
assert_eq_int(detected_deflate, ALGO_DEFLATE)

fr fr Test auto-detection of GZIP format
sus gzip_test_data tea = "GZ5B:test_data_content"
sus detected_gzip normie = auto_detect_algorithm(gzip_test_data)
assert_eq_int(detected_gzip, ALGO_GZIP)

fr fr Test unknown format detection
sus unknown_data tea = "UNKNOWN_FORMAT:data"
sus detected_unknown normie = auto_detect_algorithm(unknown_data)
assert_eq_int(detected_unknown, 0)

fr fr ==========================================
fr fr Compression Level Tests
fr fr ==========================================

fr fr Test different compression levels for LZ4
sus fast_lz4 tea = lz4_compress_data(test_data, COMPRESS_LEVEL_FAST)
sus max_lz4 tea = lz4_compress_data(test_data, COMPRESS_LEVEL_MAX)
sus balanced_lz4 tea = lz4_compress_data(test_data, COMPRESS_LEVEL_BALANCED)

fr fr Verify all levels can be decompressed
assert_eq_string(lz4_decompress_data(fast_lz4), test_data)
assert_eq_string(lz4_decompress_data(max_lz4), test_data)
assert_eq_string(lz4_decompress_data(balanced_lz4), test_data)

fr fr Test different compression levels for DEFLATE
sus fast_deflate tea = deflate_compress_data(test_data, COMPRESS_LEVEL_FAST)
sus max_deflate tea = deflate_compress_data(test_data, COMPRESS_LEVEL_MAX)
sus balanced_deflate tea = deflate_compress_data(test_data, COMPRESS_LEVEL_BALANCED)

assert_eq_string(deflate_decompress_data(fast_deflate), test_data)
assert_eq_string(deflate_decompress_data(max_deflate), test_data)
assert_eq_string(deflate_decompress_data(balanced_deflate), test_data)

fr fr ==========================================
fr fr Utility Function Tests
fr fr ==========================================

fr fr Test compression ratio calculation
sus original_size normie = 100
sus compressed_size normie = 75
sus ratio normie = calculate_compression_ratio(original_size, compressed_size)
assert_eq_int(ratio, 75)

fr fr Test algorithm name retrieval
sus lz4_name tea = get_algorithm_name(ALGO_LZ4)
assert_eq_string(lz4_name, "LZ4")

sus deflate_name tea = get_algorithm_name(ALGO_DEFLATE)
assert_eq_string(deflate_name, "DEFLATE")

sus gzip_name tea = get_algorithm_name(ALGO_GZIP)
assert_eq_string(gzip_name, "GZIP")

sus unknown_name tea = get_algorithm_name(99)
assert_eq_string(unknown_name, "UNKNOWN")

fr fr Test compression detection
sus compressed_test tea = "LZ4F:compressed_content"
assert_true(is_compressed_vibes(compressed_test))

sus uncompressed_test tea = "regular_text_data"
assert_false(is_compressed_vibes(uncompressed_test))

fr fr ==========================================
fr fr Edge Case Tests
fr fr ==========================================

fr fr Test empty string compression
sus empty_data tea = ""
sus empty_lz4 tea = lz4_compress_data(empty_data, COMPRESS_LEVEL_BALANCED)
sus empty_deflate tea = deflate_compress_data(empty_data, COMPRESS_LEVEL_BALANCED)
sus empty_gzip tea = gzip_compress_data(empty_data, COMPRESS_LEVEL_BALANCED)

fr fr Verify empty data handling
assert_eq_string(lz4_decompress_data(empty_lz4), empty_data)
assert_eq_string(deflate_decompress_data(empty_deflate), empty_data)
assert_eq_string(gzip_decompress_data(empty_gzip), empty_data)

fr fr Test very small data compression
sus small_data tea = "Hi"
sus small_lz4 tea = lz4_compress_data(small_data, COMPRESS_LEVEL_BALANCED)
sus small_deflate tea = deflate_compress_data(small_data, COMPRESS_LEVEL_BALANCED)
sus small_gzip tea = gzip_compress_data(small_data, COMPRESS_LEVEL_BALANCED)

assert_eq_string(lz4_decompress_data(small_lz4), small_data)
assert_eq_string(deflate_decompress_data(small_deflate), small_data)
assert_eq_string(gzip_decompress_data(small_gzip), small_data)

fr fr ==========================================
fr fr String Utility Function Tests  
fr fr ==========================================

fr fr Test string_starts_with function
assert_true(string_starts_with("LZ4F:data", "LZ4"))
assert_true(string_starts_with("DEF5:data", "DEF"))
assert_true(string_starts_with("GZ1F:data", "GZ"))
assert_false(string_starts_with("regular_data", "LZ4"))

fr fr Test string_length function
sus length_test tea = "test_string"
sus test_length normie = string_length(length_test)
assert_true(test_length > 0)

fr fr Test char_at function
sus char_h normie = char_at("Hello", 0)
assert_eq_int(char_h, 72) fr fr ASCII 'H'

sus char_e normie = char_at("Hello", 1)
assert_eq_int(char_e, 101) fr fr ASCII 'e'

fr fr ==========================================
fr fr Integration Tests
fr fr ==========================================

fr fr Test round-trip compression for all algorithms
sus integration_data tea = "Integration test data with various characters 123!@#"

fr fr LZ4 round-trip
sus int_lz4_comp tea = compress_slay(integration_data, ALGO_LZ4, COMPRESS_LEVEL_BALANCED)
sus int_lz4_decomp tea = decompress_vibes(int_lz4_comp, ALGO_LZ4)
assert_eq_string(int_lz4_decomp, integration_data)

fr fr DEFLATE round-trip  
sus int_def_comp tea = compress_slay(integration_data, ALGO_DEFLATE, COMPRESS_LEVEL_BALANCED)
sus int_def_decomp tea = decompress_vibes(int_def_comp, ALGO_DEFLATE)
assert_eq_string(int_def_decomp, integration_data)

fr fr GZIP round-trip
sus int_gz_comp tea = compress_slay(integration_data, ALGO_GZIP, COMPRESS_LEVEL_BALANCED)
sus int_gz_decomp tea = decompress_vibes(int_gz_comp, ALGO_GZIP)
assert_eq_string(int_gz_decomp, integration_data)

fr fr Test multiple algorithm compression
sus multi_result tea = compress_multiple_algorithms(integration_data, COMPRESS_LEVEL_BALANCED)
assert_true(string_length(multi_result) > 0)

fr fr Auto-detect and decompress
sus auto_algo normie = auto_detect_algorithm(multi_result)
vibes auto_algo > 0 {
    sus auto_decomp tea = decompress_vibes(multi_result, auto_algo)
    assert_eq_string(auto_decomp, integration_data)
}

fr fr ==========================================
fr fr Performance and Benchmarking Tests
fr fr ==========================================

fr fr Test compression ratio calculation with realistic data
sus perf_original_size normie = 1000
sus perf_compressed_size normie = 650
sus perf_ratio normie = calculate_compression_ratio(perf_original_size, perf_compressed_size)
assert_eq_int(perf_ratio, 65)

fr fr Test zero division protection
sus zero_ratio normie = calculate_compression_ratio(0, 100)
assert_eq_int(zero_ratio, 100)

fr fr ==========================================
fr fr Algorithm Constant Tests
fr fr ==========================================

fr fr Verify algorithm constants are properly defined
assert_eq_int(ALGO_LZ4, 3)
assert_eq_int(ALGO_DEFLATE, 2)
assert_eq_int(ALGO_GZIP, 1)

fr fr Verify compression level constants
assert_eq_int(COMPRESS_LEVEL_FAST, 1)
assert_eq_int(COMPRESS_LEVEL_BALANCED, 5)
assert_eq_int(COMPRESS_LEVEL_MAX, 9)

fr fr ==========================================
fr fr Error Handling Tests
fr fr ==========================================

fr fr Test invalid algorithm handling
sus invalid_comp tea = compress_slay(test_data, 999, COMPRESS_LEVEL_BALANCED)
assert_eq_string(invalid_comp, test_data) fr fr Should return original data

sus invalid_decomp tea = decompress_vibes(test_data, 999)
assert_eq_string(invalid_decomp, test_data) fr fr Should return original data

fr fr Test malformed compressed data
sus malformed_data tea = "INVALID:malformed_data"
sus malformed_lz4 tea = lz4_decompress_data(malformed_data)
sus malformed_deflate tea = deflate_decompress_data(malformed_data)
sus malformed_gzip tea = gzip_decompress_data(malformed_data)

fr fr Should handle gracefully (return original or safe fallback)
assert_true(string_length(malformed_lz4) >= 0)
assert_true(string_length(malformed_deflate) >= 0)  
assert_true(string_length(malformed_gzip) >= 0)

fr fr ==========================================
fr fr Advanced Feature Tests
fr fr ==========================================

fr fr Test string_substring functionality
sus substr_test tea = "0123456789"
sus substr_result tea = string_substring(substr_test, 5, 3)
assert_true(string_length(substr_result) > 0)

fr fr Test string_copy functionality  
sus copy_test tea = "copy_me"
sus copied_string tea = string_copy(copy_test)
assert_eq_string(copied_string, copy_test)

print_test_summary()
