yeet "testz"
yeet "squish_core"

# Test compression algorithms
test_start("GZIP compression test")
sus test_data := "Hello, World! This is test data for compression. The quick brown fox jumps over the lazy dog."
sus compressed := squish_compress_gzip(test_data, SQUISH_BALANCED)
sus decompressed := squish_decompress_gzip(squish_result_data(compressed))
assert_eq_string(decompressed, test_data)
assert_true(squish_result_ratio(compressed) > 1.0)
vibez.spill("GZIP test passed - ratio: " + squish_result_ratio(compressed).(tea))

test_start("DEFLATE compression test")
sus deflate_compressed := squish_compress_deflate(test_data, SQUISH_BALANCED)
sus deflate_decompressed := squish_decompress_deflate(squish_result_data(deflate_compressed))
assert_eq_string(deflate_decompressed, test_data)
assert_true(squish_result_ratio(deflate_compressed) > 1.0)
vibez.spill("DEFLATE test passed - ratio: " + squish_result_ratio(deflate_compressed).(tea))

test_start("BROTLI compression test")
sus brotli_compressed := squish_compress_brotli(test_data, SQUISH_BALANCED)
sus brotli_decompressed := squish_decompress_brotli(squish_result_data(brotli_compressed))
assert_eq_string(brotli_decompressed, test_data)
assert_true(squish_result_ratio(brotli_compressed) > 1.0)
vibez.spill("BROTLI test passed - ratio: " + squish_result_ratio(brotli_compressed).(tea))

test_start("ZSTANDARD compression test")
sus zstd_compressed := squish_compress_zstandard(test_data, SQUISH_BALANCED)
sus zstd_decompressed := squish_decompress_zstandard(squish_result_data(zstd_compressed))
assert_eq_string(zstd_decompressed, test_data)
assert_true(squish_result_ratio(zstd_compressed) > 1.0)
vibez.spill("ZSTD test passed - ratio: " + squish_result_ratio(zstd_compressed).(tea))

# Test universal compression functions
test_start("Universal compression API test")
sus universal_gzip := squish_compress(test_data, ALGO_GZIP, SQUISH_BALANCED)
sus universal_decompressed := squish_decompress(squish_result_data(universal_gzip), ALGO_GZIP)
assert_eq_string(universal_decompressed, test_data)
vibez.spill("Universal API test passed")

# Test compression levels
test_start("Compression levels test")
sus fast_compressed := squish_compress(test_data, ALGO_GZIP, SQUISH_FAST)
sus balanced_compressed := squish_compress(test_data, ALGO_GZIP, SQUISH_BALANCED)
sus max_compressed := squish_compress(test_data, ALGO_GZIP, SQUISH_MAX)
assert_true(squish_result_ratio(fast_compressed) > 0.0)
assert_true(squish_result_ratio(balanced_compressed) > 0.0)
assert_true(squish_result_ratio(max_compressed) > 0.0)
vibez.spill("Compression levels test passed")

# Test stream compression
test_start("Stream compression test")
sus large_data := test_data + test_data + test_data + test_data
sus stream_compressed := squish_stream_compress(large_data, 50, ALGO_GZIP, SQUISH_BALANCED)
sus stream_decompressed := squish_stream_decompress(stream_compressed, ALGO_GZIP)
assert_eq_string(stream_decompressed, large_data)
vibez.spill("Stream compression test passed")

# Test integrity checking
test_start("Integrity checking test")
sus integrity_compressed := squish_compress(test_data, ALGO_DEFLATE, SQUISH_BALANCED)
sus integrity_valid := squish_verify_integrity(test_data, squish_result_data(integrity_compressed), ALGO_DEFLATE)
assert_true(integrity_valid)
vibez.spill("Integrity checking test passed")

# Test CRC32 calculation
test_start("CRC32 checksum test")
sus crc1 := squish_calculate_crc32(test_data)
sus crc2 := squish_calculate_crc32(test_data)
sus crc3 := squish_calculate_crc32("different data")
assert_eq_string(crc1, crc2)
assert_false(crc1 == crc3)
vibez.spill("CRC32 test passed: " + crc1)

# Test binary data handling
test_start("Binary data compression test")
sus binary_data := "BINARY_DATA_12345"
sus binary_compressed := squish_compress_binary(binary_data, ALGO_BROTLI, SQUISH_BALANCED)
sus binary_decompressed := squish_decompress_binary(squish_result_data(binary_compressed), ALGO_BROTLI)
assert_eq_string(binary_decompressed, binary_data)
vibez.spill("Binary data test passed")

# Test performance metrics
test_start("Performance metrics test")
sus original_size := test_data.length()
sus compressed_size := squish_estimate_size(test_data, ALGO_GZIP, SQUISH_BALANCED)
sus ratio := squish_get_compression_ratio(original_size, compressed_size)
assert_true(ratio > 1.0)
vibez.spill("Performance metrics test passed - estimated ratio: " + ratio.(tea))

# Test algorithm benchmarking
test_start("Algorithm benchmarking test")
sus benchmark_result := squish_benchmark_algorithm(test_data, ALGO_BROTLI, SQUISH_BALANCED)
assert_true(benchmark_result.contains("Algorithm: brotli"))
assert_true(benchmark_result.contains("Time:"))
assert_true(benchmark_result.contains("Ratio:"))
vibez.spill("Benchmark test passed: " + benchmark_result)

# Test archive functionality
test_start("Archive creation and extraction test")
sus file_list := "file1.txt,file2.txt,file3.txt"
sus archive := squish_create_archive(file_list, ALGO_ZSTANDARD, SQUISH_BALANCED)
sus extracted := squish_extract_archive(archive, ALGO_ZSTANDARD)
assert_true(extracted.contains("file1.txt"))
assert_true(extracted.contains("file2.txt"))
assert_true(extracted.contains("file3.txt"))
vibez.spill("Archive test passed")

# Test memory-efficient operations
test_start("Memory-efficient compression test")
sus chunk_compressed := squish_compress_in_chunks(large_data, ALGO_DEFLATE, SQUISH_BALANCED)
sus memory_usage := squish_get_memory_usage(large_data.length(), ALGO_BROTLI)
assert_true(memory_usage > 0)
vibez.spill("Memory-efficient test passed - memory usage: " + memory_usage.(tea))

# Test Gen Z enhanced APIs
test_start("Gen Z enhanced APIs test")
sus no_cap_compressed := squish_compress_no_cap(test_data, ALGO_GZIP)
sus lowkey_fast_compressed := squish_compress_lowkey_fast(test_data, ALGO_GZIP)
sus is_fire := squish_is_compressed_fire(test_data, no_cap_compressed)
assert_true(squish_result_ratio(no_cap_compressed) > 0.0)
assert_true(squish_result_ratio(lowkey_fast_compressed) > 0.0)
vibez.spill("Gen Z APIs test passed - compression is fire: " + is_fire.(tea))

# Test compression with flexing
test_start("Compression flex test")
sus flex_result := squish_compress_and_flex(test_data, ALGO_ZSTANDARD, SQUISH_MAX)
assert_true(squish_result_ratio(flex_result) > 0.0)
vibez.spill("Flex test passed")

# Test all algorithms comparison
test_start("Algorithm comparison test")
sus gzip_result := squish_compress(test_data, ALGO_GZIP, SQUISH_BALANCED)
sus deflate_result := squish_compress(test_data, ALGO_DEFLATE, SQUISH_BALANCED)
sus brotli_result := squish_compress(test_data, ALGO_BROTLI, SQUISH_BALANCED)
sus zstd_result := squish_compress(test_data, ALGO_ZSTANDARD, SQUISH_BALANCED)

# ZSTD should have the best compression ratio
assert_true(squish_result_ratio(zstd_result) >= squish_result_ratio(brotli_result))
assert_true(squish_result_ratio(brotli_result) >= squish_result_ratio(deflate_result))
assert_true(squish_result_ratio(deflate_result) >= squish_result_ratio(gzip_result))
vibez.spill("Algorithm comparison test passed")

# Test edge cases
test_start("Edge cases test")
sus empty_data := ""
sus empty_compressed := squish_compress(empty_data, ALGO_GZIP, SQUISH_BALANCED)
sus empty_decompressed := squish_decompress(squish_result_data(empty_compressed), ALGO_GZIP)
assert_eq_string(empty_decompressed, empty_data)

sus single_char := "a"
sus single_compressed := squish_compress(single_char, ALGO_DEFLATE, SQUISH_BALANCED)
sus single_decompressed := squish_decompress(squish_result_data(single_compressed), ALGO_DEFLATE)
assert_eq_string(single_decompressed, single_char)
vibez.spill("Edge cases test passed")

# Test checksum functionality
test_start("Checksum calculation test")
sus checksum1 := squish_calculate_checksum(test_data)
sus checksum2 := squish_calculate_checksum(test_data)
sus checksum3 := squish_calculate_checksum("different")
assert_eq_string(checksum1, checksum2)
assert_false(checksum1 == checksum3)
vibez.spill("Checksum test passed: " + checksum1)

# Test result structure helpers
test_start("Result structure helpers test")
sus test_result := squish_result_new("compressed_data", 2.5, "checksum123")
sus extracted_data := squish_result_data(test_result)
sus extracted_ratio := squish_result_ratio(test_result)
sus extracted_checksum := squish_result_checksum(test_result)
assert_eq_string(extracted_data, "compressed_data")
assert_eq_drip(extracted_ratio, 2.5)
assert_eq_string(extracted_checksum, "checksum123")
vibez.spill("Result structure test passed")

# Test size estimation accuracy
test_start("Size estimation test")
sus estimated_gzip := squish_estimate_size(test_data, ALGO_GZIP, SQUISH_BALANCED)
sus estimated_brotli := squish_estimate_size(test_data, ALGO_BROTLI, SQUISH_BALANCED)
sus estimated_zstd := squish_estimate_size(test_data, ALGO_ZSTANDARD, SQUISH_BALANCED)
assert_true(estimated_zstd < estimated_brotli)
assert_true(estimated_brotli < estimated_gzip)
vibez.spill("Size estimation test passed")

print_test_summary()
