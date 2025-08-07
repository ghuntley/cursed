yeet "testz"
yeet "compression"

test_start("LZ4 compression/decompression")
sus test_data tea = "Hello World! This is a test string for compression."
sus lz4_compressed tea = lz4_compress_data(test_data, COMPRESS_LEVEL_BALANCED)
assert_true(len(lz4_compressed) > 0)

sus lz4_decompressed tea = lz4_decompress_data(lz4_compressed)
assert_true(len(lz4_decompressed) > 0)

test_start("DEFLATE compression/decompression")
sus deflate_compressed tea = deflate_compress_data(test_data, COMPRESS_LEVEL_BALANCED)
assert_true(len(deflate_compressed) > 0)

sus deflate_decompressed tea = deflate_decompress_data(deflate_compressed)
assert_true(len(deflate_decompressed) > 0)

test_start("GZIP compression/decompression")
sus gzip_compressed tea = gzip_compress_data(test_data, COMPRESS_LEVEL_BALANCED)
assert_true(len(gzip_compressed) > 0)

sus gzip_decompressed tea = gzip_decompress_data(gzip_compressed)
assert_true(len(gzip_decompressed) > 0)

test_start("High-level compression interface")
sus compressed_lz4 tea = compress_slay(test_data, ALGO_LZ4, COMPRESS_LEVEL_FAST)
assert_true(len(compressed_lz4) > 0)

sus compressed_deflate tea = compress_slay(test_data, ALGO_DEFLATE, COMPRESS_LEVEL_MAX)
assert_true(len(compressed_deflate) > 0)

sus compressed_gzip tea = compress_slay(test_data, ALGO_GZIP, COMPRESS_LEVEL_BALANCED)
assert_true(len(compressed_gzip) > 0)

test_start("Auto-detection")
sus detected_lz4 normie = auto_detect_algorithm(compressed_lz4)
assert_eq_int(detected_lz4, ALGO_LZ4)

sus detected_deflate normie = auto_detect_algorithm(compressed_deflate)
assert_eq_int(detected_deflate, ALGO_DEFLATE)

sus detected_gzip normie = auto_detect_algorithm(compressed_gzip)
assert_eq_int(detected_gzip, ALGO_GZIP)

test_start("Decompression")
sus decompressed_lz4 tea = decompress_vibes(compressed_lz4, ALGO_LZ4)
assert_true(len(decompressed_lz4) > 0)

sus decompressed_deflate tea = decompress_vibes(compressed_deflate, ALGO_DEFLATE)
assert_true(len(decompressed_deflate) > 0)

sus decompressed_gzip tea = decompress_vibes(compressed_gzip, ALGO_GZIP)
assert_true(len(decompressed_gzip) > 0)

test_start("Compression metrics")
sus result CompressionResult = compress_with_metrics(test_data, ALGO_GZIP, COMPRESS_LEVEL_BALANCED)
assert_true(result.success)
assert_true(result.original_size > 0)
assert_true(result.compressed_size > 0)
assert_true(result.compression_ratio >= 0)
assert_eq_int(result.algorithm, ALGO_GZIP)

test_start("Decompression with validation")
sus decomp_result CompressionResult = decompress_with_validation(compressed_gzip)
assert_true(decomp_result.success)
assert_eq_string(decomp_result.error_message, "")

test_start("Compression ratio calculation")
sus ratio normie = calculate_compression_ratio(100, 75)
assert_eq_int(ratio, 75)

sus ratio_zero normie = calculate_compression_ratio(0, 50)
assert_eq_int(ratio_zero, 100)

test_start("Algorithm names")
sus lz4_name tea = get_algorithm_name(ALGO_LZ4)
assert_eq_string(lz4_name, "LZ4")

sus deflate_name tea = get_algorithm_name(ALGO_DEFLATE)
assert_eq_string(deflate_name, "DEFLATE")

sus gzip_name tea = get_algorithm_name(ALGO_GZIP)
assert_eq_string(gzip_name, "GZIP")

test_start("Compression detection")
sus is_lz4_compressed lit = is_compressed_vibes(compressed_lz4)
assert_true(is_lz4_compressed)

sus is_raw_compressed lit = is_compressed_vibes("plain text")
assert_false(is_raw_compressed)

test_start("Multi-algorithm comparison")
sus best_compression tea = compress_multiple_algorithms(test_data, COMPRESS_LEVEL_BALANCED)
assert_true(len(best_compression) > 0)

test_start("String manipulation helpers")
sus starts_with_lz4 lit = string_starts_with("LZ4:data", "LZ4")
assert_true(starts_with_lz4)

sus starts_with_false lit = string_starts_with("DEF:data", "LZ4")
assert_false(starts_with_false)

sus substring tea = string_substring("GZ5B:content", 5, 7)
assert_true(len(substring) > 0)

test_start("Edge cases")
sus empty_compression tea = compress_slay("", ALGO_LZ4, COMPRESS_LEVEL_FAST)
assert_eq_string(empty_compression, "")

sus empty_decompression tea = decompress_vibes("", ALGO_LZ4)
assert_eq_string(empty_decompression, "")

sus unknown_algo normie = auto_detect_algorithm("unknown_format")
assert_eq_int(unknown_algo, 0)

print_test_summary()
