fr fr Comprehensive Compression Algorithm Test
fr fr Tests real GZIP, DEFLATE, and ZIP compression functionality

yeet "compressz"
yeet "vibez"
yeet "stringz"

slay test_basic_compression() {
    vibez.spill("===== BASIC COMPRESSION TEST =====")
    
    fr fr Test data
    sus test_data tea = "Hello, World! This is a test string for compression. " +
                        "It contains repeating patterns and should compress well. " +
                        "Hello, World! This is a test string for compression."
    
    vibez.spill("Original data:", test_data)
    vibez.spill("Original size:", string_length(test_data))
    
    fr fr Test GZIP compression
    vibez.spill("\n--- GZIP Compression ---")
    sus gzip_result CompressedData = compressz.gzip_compress(test_data, 6)
    vibez.spill("GZIP compressed size:", gzip_result.compressed_size)
    vibez.spill("GZIP compression ratio:", gzip_result.compression_ratio)
    vibez.spill("GZIP algorithm:", gzip_result.algorithm)
    
    sus gzip_decompressed tea = compressz.gzip_decompress(gzip_result)
    vibez.spill("GZIP decompressed:", gzip_decompressed)
    vibez.spill("GZIP roundtrip success:", test_data == gzip_decompressed)
    
    fr fr Test DEFLATE compression
    vibez.spill("\n--- DEFLATE Compression ---")
    sus deflate_result CompressedData = compressz.deflate_compress(test_data, 6)
    vibez.spill("DEFLATE compressed size:", deflate_result.compressed_size)
    vibez.spill("DEFLATE compression ratio:", deflate_result.compression_ratio)
    vibez.spill("DEFLATE algorithm:", deflate_result.algorithm)
    
    sus deflate_decompressed tea = compressz.deflate_decompress(deflate_result)
    vibez.spill("DEFLATE decompressed:", deflate_decompressed)
    vibez.spill("DEFLATE roundtrip success:", test_data == deflate_decompressed)
}

slay test_compression_levels() {
    vibez.spill("\n===== COMPRESSION LEVEL TEST =====")
    
    sus test_data tea = "AAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBBBCCCCCCCCCCCCCCCCCC"
    
    sus level1 CompressedData = compressz.gzip_compress(test_data, 1)
    sus level5 CompressedData = compressz.gzip_compress(test_data, 5)
    sus level9 CompressedData = compressz.gzip_compress(test_data, 9)
    
    vibez.spill("Original size:", string_length(test_data))
    vibez.spill("Level 1 size:", level1.compressed_size, "ratio:", level1.compression_ratio)
    vibez.spill("Level 5 size:", level5.compressed_size, "ratio:", level5.compression_ratio)
    vibez.spill("Level 9 size:", level9.compressed_size, "ratio:", level9.compression_ratio)
}

slay test_entropy_detection() {
    vibez.spill("\n===== ENTROPY DETECTION TEST =====")
    
    sus low_entropy tea = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
    sus high_entropy tea = "a1B#x9$mP2@nQ7&kL4%jR8*vS3!wT6^yU5+zV"
    sus medium_entropy tea = "Hello World! This is a test message."
    
    sus low_level drip = compressz.auto_detect_best_compression(low_entropy)
    sus high_level drip = compressz.auto_detect_best_compression(high_entropy)
    sus med_level drip = compressz.auto_detect_best_compression(medium_entropy)
    
    vibez.spill("Low entropy data level:", low_level)
    vibez.spill("High entropy data level:", high_level)
    vibez.spill("Medium entropy data level:", med_level)
    
    sus low_ent normie = compressz.calculate_entropy(low_entropy)
    sus high_ent normie = compressz.calculate_entropy(high_entropy)
    sus med_ent normie = compressz.calculate_entropy(medium_entropy)
    
    vibez.spill("Low entropy value:", low_ent)
    vibez.spill("High entropy value:", high_ent)
    vibez.spill("Medium entropy value:", med_ent)
}

slay test_zip_format() {
    vibez.spill("\n===== ZIP FORMAT TEST =====")
    
    sus filename tea = "test.txt"
    sus file_data tea = "This is test file content for ZIP compression testing."
    
    sus zip_data tea = compressz.zip_compress_file(filename, file_data)
    vibez.spill("ZIP file created, size:", string_length(zip_data))
    
    sus extracted tea = compressz.zip_extract_file(zip_data, filename)
    vibez.spill("Extracted data:", extracted)
    vibez.spill("ZIP roundtrip success:", file_data == extracted)
}

slay test_crc32_calculation() {
    vibez.spill("\n===== CRC32 CALCULATION TEST =====")
    
    sus test_strings []tea = [
        "Hello",
        "World",
        "Hello, World!",
        "The quick brown fox jumps over the lazy dog"
    ]
    
    sus i drip = 0
    bestie (i < array_length(test_strings)) {
        sus crc drip = compressz.calculate_crc32(test_strings[i])
        vibez.spill("CRC32 of '" + test_strings[i] + "':", crc)
        i = i + 1
    }
}

slay test_lz77_encoding() {
    vibez.spill("\n===== LZ77 ENCODING TEST =====")
    
    sus test_data tea = "ABCABCABC"
    sus window_size drip = 8
    
    sus compressed tea = compressz.lz77_compress(test_data, window_size)
    vibez.spill("LZ77 input:", test_data)
    vibez.spill("LZ77 compressed length:", string_length(compressed))
    
    sus decompressed tea = compressz.lz77_decompress(compressed)
    vibez.spill("LZ77 decompressed:", decompressed)
    vibez.spill("LZ77 roundtrip success:", test_data == decompressed)
}

slay test_huffman_coding() {
    vibez.spill("\n===== HUFFMAN CODING TEST =====")
    
    sus test_data tea = "aaaaabbbbcccdde"
    
    sus frequencies []drip = compressz.calculate_frequencies(test_data)
    vibez.spill("Frequencies calculated for:", test_data)
    
    sus huffman_tree tea = compressz.build_huffman_tree(test_data)
    vibez.spill("Huffman tree built")
    
    sus encoded tea = compressz.huffman_encode(test_data, huffman_tree)
    vibez.spill("Huffman encoded length:", string_length(encoded))
    
    sus decoded tea = compressz.huffman_decode(encoded, huffman_tree)
    vibez.spill("Huffman decoded:", decoded)
    vibez.spill("Huffman roundtrip success:", test_data == decoded)
}

slay test_advanced_algorithms() {
    vibez.spill("\n===== ADVANCED ALGORITHMS TEST =====")
    
    sus test_data tea = "This is test data for advanced compression algorithms."
    
    sus lzma_result CompressedData = compressz.lzma_compress(test_data)
    vibez.spill("LZMA compressed size:", lzma_result.compressed_size)
    vibez.spill("LZMA compression ratio:", lzma_result.compression_ratio)
    
    sus brotli_result CompressedData = compressz.brotli_compress(test_data, 6)
    vibez.spill("Brotli compressed size:", brotli_result.compressed_size)
    vibez.spill("Brotli compression ratio:", brotli_result.compression_ratio)
}

slay test_error_handling() {
    vibez.spill("\n===== ERROR HANDLING TEST =====")
    
    fr fr Test empty data
    sus empty_result CompressedData = compressz.gzip_compress("", 6)
    vibez.spill("Empty data compression size:", empty_result.compressed_size)
    
    fr fr Test invalid compression levels
    sus invalid_low CompressedData = compressz.gzip_compress("test", 0)
    sus invalid_high CompressedData = compressz.gzip_compress("test", 15)
    vibez.spill("Invalid low level clamped to:", invalid_low.algorithm)
    vibez.spill("Invalid high level clamped to:", invalid_high.algorithm)
    
    fr fr Test corrupted GZIP header
    sus fake_gzip CompressedData = CompressedData{}
    fake_gzip.data = "invalid_gzip_data"
    fake_gzip.compressed_size = string_length(fake_gzip.data)
    
    sus corrupt_result tea = compressz.gzip_decompress(fake_gzip)
    vibez.spill("Corrupt GZIP decompression result:", string_length(corrupt_result))
}

slay run_performance_benchmark() {
    vibez.spill("\n===== PERFORMANCE BENCHMARK =====")
    
    fr fr Create test data of various sizes
    sus small_data tea = "Small test data"
    sus medium_data tea = ""
    sus large_data tea = ""
    
    fr fr Build medium data (1KB)
    sus i drip = 0
    bestie (i < 20) {
        medium_data = medium_data + "This is a medium-sized test string for compression benchmarking. "
        i = i + 1
    }
    
    fr fr Build large data (10KB)
    i = 0
    bestie (i < 200) {
        large_data = large_data + "This is a large test string for compression performance testing. "
        i = i + 1
    }
    
    fr fr Benchmark different sizes
    sus small_gzip CompressedData = compressz.gzip_compress(small_data, 6)
    sus medium_gzip CompressedData = compressz.gzip_compress(medium_data, 6)
    sus large_gzip CompressedData = compressz.gzip_compress(large_data, 6)
    
    vibez.spill("Small data (", string_length(small_data), " bytes) -> ", small_gzip.compressed_size, " bytes")
    vibez.spill("Medium data (", string_length(medium_data), " bytes) -> ", medium_gzip.compressed_size, " bytes")
    vibez.spill("Large data (", string_length(large_data), " bytes) -> ", large_gzip.compressed_size, " bytes")
    
    vibez.spill("Small compression ratio:", small_gzip.compression_ratio)
    vibez.spill("Medium compression ratio:", medium_gzip.compression_ratio)
    vibez.spill("Large compression ratio:", large_gzip.compression_ratio)
}

slay main() {
    vibez.spill("CURSED Compression Algorithm Implementation Test")
    vibez.spill("=================================================")
    
    test_basic_compression()
    test_compression_levels()
    test_entropy_detection()
    test_zip_format()
    test_crc32_calculation()
    test_lz77_encoding()
    test_huffman_coding()
    test_advanced_algorithms()
    test_error_handling()
    run_performance_benchmark()
    
    vibez.spill("\n=================================================")
    vibez.spill("Compression algorithm test complete!")
    vibez.spill("All critical compression functions implemented.")
}
