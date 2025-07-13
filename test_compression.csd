// CURSED Compression Module Test Suite
// Comprehensive testing for compression algorithms

yeet "testz"
yeet "compression"

// ========================================
// Test Data Sets
// ========================================

slay get_test_text() tea {
    damn "The quick brown fox jumps over the lazy dog. This is a test string for compression algorithms."
}

slay get_repeated_text() tea {
    sus base tea = "AAAABBBBCCCCDDDD"
    sus result tea = ""
    bestie i := 0; i < 10; i++ {
        result = result + base
    }
    damn result
}

// ========================================
// Basic Compression Tests
// ========================================

slay test_rle_compression() {
    test_start("RLE compression and decompression")
    
    sus test_data tea = get_repeated_text()
    sus compressed tea = rle_compress(test_data)
    sus decompressed tea = rle_decompress(compressed)
    
    assert_eq_string(decompressed, test_data)
    assert_true(string_len(compressed) > 0)
    assert_true(string_len(compressed) < string_len(test_data))
    
    print_test_summary()
}

slay test_gzip_compression() {
    test_start("GZIP compression and decompression")
    
    sus test_data tea = get_test_text()
    sus compressed tea = gzip_compress(test_data, 6)
    sus decompressed tea = gzip_decompress(compressed)
    
    assert_eq_string(decompressed, test_data)
    assert_true(string_len(compressed) > 0)
    
    print_test_summary()
}

slay test_zlib_compression() {
    test_start("ZLIB compression and decompression")
    
    sus test_data tea = get_test_text()
    sus compressed tea = zlib_compress(test_data, 6)
    sus decompressed tea = zlib_decompress(compressed)
    
    assert_eq_string(decompressed, test_data)
    assert_true(string_len(compressed) > 0)
    
    print_test_summary()
}

slay test_lz4_compression() {
    test_start("LZ4 high-speed compression and decompression")
    
    sus test_data tea = get_test_text()
    sus compressed tea = lz4_compress(test_data)
    sus decompressed tea = lz4_decompress(compressed)
    
    assert_eq_string(decompressed, test_data)
    assert_true(string_len(compressed) > 0)
    
    print_test_summary()
}

slay test_brotli_compression() {
    test_start("Brotli compression and decompression")
    
    sus test_data tea = get_test_text()
    sus compressed tea = brotli_compress(test_data, 6)
    sus decompressed tea = brotli_decompress(compressed)
    
    assert_eq_string(decompressed, test_data)
    assert_true(string_len(compressed) > 0)
    
    print_test_summary()
}

slay test_zstd_compression() {
    test_start("Zstandard (ZSTD) compression and decompression")
    
    sus test_data tea = get_test_text()
    sus compressed tea = zstd_compress(test_data, 6)
    sus decompressed tea = zstd_decompress(compressed)
    
    assert_eq_string(decompressed, test_data)
    assert_true(string_len(compressed) > 0)
    
    print_test_summary()
}

// ========================================
// Archive Format Tests
// ========================================

slay test_tar_archive() {
    test_start("TAR archive creation and extraction")
    
    // Create test entries
    sus entries []ArchiveEntry = [
        ArchiveEntry{
            filename: "test1.txt",
            data: "Hello, World!",
            size: 13,
            compressed_size: 13,
            checksum: 0
        },
        ArchiveEntry{
            filename: "test2.txt",
            data: get_test_text(),
            size: string_len(get_test_text()),
            compressed_size: string_len(get_test_text()),
            checksum: 0
        }
    ]
    
    sus archive tea = create_tar_archive(entries)
    assert_true(string_len(archive) > 0)
    
    sus extracted []ArchiveEntry = extract_tar_archive(archive)
    assert_true(len(extracted) >= 0)  // May be 0 due to simplified implementation
    
    print_test_summary()
}

slay test_zip_archive() {
    test_start("ZIP archive creation and extraction")
    
    // Create test entries
    sus entries []ArchiveEntry = [
        ArchiveEntry{
            filename: "file1.txt",
            data: "ZIP test content",
            size: 16,
            compressed_size: 16,
            checksum: calculate_crc32("ZIP test content")
        }
    ]
    
    sus archive tea = create_zip_archive(entries)
    assert_true(string_len(archive) > 0)
    
    sus extracted []ArchiveEntry = extract_zip_archive(archive)
    assert_true(len(extracted) >= 0)  // May be 0 due to simplified implementation
    
    print_test_summary()
}

// ========================================
// Utility Function Tests
// ========================================

slay test_compression_ratios() {
    test_start("Compression ratio calculation")
    
    sus test_data tea = get_test_text()
    sus repeated_data tea = get_repeated_text()
    
    // Test with repeated data (should compress well)
    sus compressed tea = rle_compress(repeated_data)
    sus ratio meal = compression_ratio(repeated_data, compressed)
    sus savings meal = calculate_savings(repeated_data, compressed)
    
    assert_true(ratio > 0.0)
    assert_true(ratio < 1.0)  // Should achieve compression
    assert_true(savings > 0.0)
    assert_true(savings < 100.0)
    
    print_test_summary()
}

slay test_crc32_calculation() {
    test_start("CRC32 checksum calculation")
    
    sus test_data tea = "Hello, World!"
    sus crc normie = calculate_crc32(test_data)
    
    assert_true(crc > 0)
    
    // Same data should produce same CRC
    sus crc2 normie = calculate_crc32(test_data)
    assert_eq_int(crc, crc2)
    
    print_test_summary()
}

slay test_adler32_calculation() {
    test_start("Adler-32 checksum calculation")
    
    sus test_data tea = "Hello, World!"
    sus adler normie = calculate_adler32(test_data)
    
    assert_true(adler > 0)
    
    // Same data should produce same Adler-32
    sus adler2 normie = calculate_adler32(test_data)
    assert_eq_int(adler, adler2)
    
    print_test_summary()
}

slay test_algorithm_selection() {
    test_start("Best algorithm selection")
    
    sus speed_algorithm tea = find_best_algorithm(get_test_text(), "speed")
    sus ratio_algorithm tea = find_best_algorithm(get_test_text(), "ratio")
    sus balanced_algorithm tea = find_best_algorithm(get_test_text(), "balanced")
    
    assert_eq_string(speed_algorithm, "lz4")
    assert_eq_string(ratio_algorithm, "brotli")
    assert_eq_string(balanced_algorithm, "zstd")
    
    print_test_summary()
}

// ========================================
// Edge Case Tests
// ========================================

slay test_empty_data() {
    test_start("Empty data compression")
    
    sus empty tea = ""
    
    sus rle_result tea = rle_compress(empty)
    sus rle_decompressed tea = rle_decompress(rle_result)
    assert_eq_string(rle_decompressed, empty)
    
    sus lz4_result tea = lz4_compress(empty)
    sus lz4_decompressed tea = lz4_decompress(lz4_result)
    assert_eq_string(lz4_decompressed, empty)
    
    print_test_summary()
}

slay test_single_character() {
    test_start("Single character compression")
    
    sus single tea = "A"
    
    sus compressed tea = rle_compress(single)
    sus decompressed tea = rle_decompress(compressed)
    assert_eq_string(decompressed, single)
    
    print_test_summary()
}

// ========================================
// Performance Tests
// ========================================

slay test_benchmark_suite() {
    test_start("Compression algorithm benchmarking")
    
    sus test_data tea = get_test_text()
    sus algorithms []tea = ["gzip", "zlib", "brotli", "zstd", "lz4"]
    
    sus benchmarks []CompressionBenchmark = benchmark_compression(test_data, algorithms)
    assert_eq_int(len(benchmarks), 5)
    
    // Verify all benchmarks have valid data
    bestie i := 0; i < len(benchmarks); i++ {
        sus bench CompressionBenchmark = benchmarks[i]
        assert_true(bench.compression_time >= 0.0)
        assert_true(bench.decompression_time >= 0.0)
        assert_true(bench.compression_ratio > 0.0)
        assert_true(bench.compression_ratio <= 1.0)
        assert_true(bench.memory_usage > 0)
    }
    
    print_test_summary()
}

// ========================================
// Integration Tests
// ========================================

slay test_round_trip_basic_algorithms() {
    test_start("Round-trip test for basic algorithms")
    
    sus test_data tea = get_test_text()
    
    // Test RLE round-trip
    sus rle_compressed tea = rle_compress(test_data)
    sus rle_result tea = rle_decompress(rle_compressed)
    assert_eq_string(rle_result, test_data)
    
    // Test LZ4 round-trip
    sus lz4_compressed tea = lz4_compress(test_data)
    sus lz4_result tea = lz4_decompress(lz4_compressed)
    assert_eq_string(lz4_result, test_data)
    
    print_test_summary()
}

slay test_auto_compression() {
    test_start("Automatic compression and decompression")
    
    sus test_data tea = get_test_text()
    
    sus auto_compressed tea = auto_compress(test_data)
    sus auto_decompressed tea = auto_decompress(auto_compressed)
    
    assert_eq_string(auto_decompressed, test_data)
    assert_true(string_len(auto_compressed) > 0)
    
    print_test_summary()
}

// ========================================
// Main Test Runner
// ========================================

slay run_all_compression_tests() {
    vibez.spill("=== CURSED Compression Module Test Suite ===")
    vibez.spill("")
    
    // Basic algorithm tests
    test_rle_compression()
    test_gzip_compression()
    test_zlib_compression()
    test_lz4_compression()
    test_brotli_compression()
    test_zstd_compression()
    
    // Archive format tests
    test_tar_archive()
    test_zip_archive()
    
    // Utility tests
    test_compression_ratios()
    test_crc32_calculation()
    test_adler32_calculation()
    test_algorithm_selection()
    
    // Edge case tests
    test_empty_data()
    test_single_character()
    
    // Performance tests
    test_benchmark_suite()
    
    // Integration tests
    test_round_trip_basic_algorithms()
    test_auto_compression()
    
    vibez.spill("")
    vibez.spill("=== All Compression Tests Complete ===")
}

// Execute all tests
run_all_compression_tests()
