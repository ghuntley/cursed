yeet "testz"
yeet "compressz"

# Comprehensive test suite for compressz module
# Real functional tests for compression operations

test_start("test_gzip_compress")
# Test GZIP compression with sample data
sus test_data tea = "Hello World! This is test data for GZIP compression testing."
sus compressed tea = gzip_compress(test_data)
assert_true(len(compressed) > 0)

# Test with empty data
sus empty_compressed tea = gzip_compress("")
assert_true(len(empty_compressed) >= 0)

# Test with repetitive data (should compress well)
sus repetitive tea = "AAAAAAAAAABBBBBBBBBBCCCCCCCCCC"
sus rep_compressed tea = gzip_compress(repetitive)
assert_true(len(rep_compressed) > 0)
assert_true(len(rep_compressed) < len(repetitive))  # Should be smaller
print_test_summary()

test_start("test_gzip_decompress")
# Test GZIP decompression
sus original tea = "Test data for compression and decompression cycle"
sus compressed tea = gzip_compress(original)
sus decompressed tea = gzip_decompress(compressed)
assert_eq_tea(decompressed, original)

# Test with different data
sus original2 tea = "Different test data 12345 !@#$%"
sus compressed2 tea = gzip_compress(original2)
sus decompressed2 tea = gzip_decompress(compressed2)
assert_eq_tea(decompressed2, original2)
print_test_summary()

test_start("test_deflate_compress")
# Test DEFLATE compression
sus test_data tea = "DEFLATE compression test data with various patterns"
sus deflated tea = deflate_compress(test_data)
assert_true(len(deflated) > 0)

# Test with binary-like data
sus binary_data tea = "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09"
sus binary_deflated tea = deflate_compress(binary_data)
assert_true(len(binary_deflated) > 0)
print_test_summary()

test_start("test_deflate_compress_internal")
# Test internal DEFLATE compression with level control
sus data tea = "Internal DEFLATE test with compression level control"
sus level1 tea = deflate_compress_internal(data, 1)  # Fast compression
sus level9 tea = deflate_compress_internal(data, 9)  # Best compression
assert_true(len(level1) > 0)
assert_true(len(level9) > 0)
assert_true(len(level9) <= len(level1))  # Better compression should be smaller or equal
print_test_summary()

test_start("test_deflate_decompress")
# Test DEFLATE decompression
sus original tea = "Round-trip test for DEFLATE compression and decompression"
sus compressed tea = deflate_compress(original)
sus decompressed tea = deflate_decompress(compressed)
assert_eq_tea(decompressed, original)

# Test with special characters
sus special_data tea = "Special: àáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ"
sus special_compressed tea = deflate_compress(special_data)
sus special_decompressed tea = deflate_decompress(special_compressed)
assert_eq_tea(special_decompressed, special_data)
print_test_summary()

test_start("test_deflate_decompress_internal")
# Test internal DEFLATE decompression
sus data tea = "Internal DEFLATE decompression testing"
sus compressed tea = deflate_compress_internal(data, 6)
sus decompressed tea = deflate_decompress_internal(compressed)
assert_eq_tea(decompressed, data)
print_test_summary()

test_start("test_lz77_compress")
# Test LZ77 compression algorithm
sus test_text tea = "The quick brown fox jumps over the lazy dog. The dog was lazy."
sus lz77_result tea = lz77_compress(test_text)
assert_true(len(lz77_result) > 0)

# Test with repeating patterns
sus pattern_text tea = "abcabcabcabcabcdefdefdefdef"
sus pattern_result tea = lz77_compress(pattern_text)
assert_true(len(pattern_result) > 0)
assert_true(len(pattern_result) < len(pattern_text))  # Should compress
print_test_summary()

test_start("test_lz77_decompress")
# Test LZ77 decompression
sus original tea = "LZ77 compression test with repeated patterns and unique content"
sus compressed tea = lz77_compress(original)
sus decompressed tea = lz77_decompress(compressed)
assert_eq_tea(decompressed, original)

# Test with short text
sus short_text tea = "Short"
sus short_compressed tea = lz77_compress(short_text)
sus short_decompressed tea = lz77_decompress(short_compressed)
assert_eq_tea(short_decompressed, short_text)
print_test_summary()

test_start("test_find_longest_match")
# Test finding longest match in sliding window
sus text tea = "abcdefabcdefghijklabcdef"
sus match_len drip = find_longest_match(text, 10, 6)  # Look for match at position 10
assert_true(match_len >= 0)

# Test with no matches
sus no_match_text tea = "abcdefghijklmnop"
sus no_match_len drip = find_longest_match(no_match_text, 8, 4)
assert_eq_int(no_match_len, 0)
print_test_summary()

test_start("test_build_huffman_tree")
# Test building Huffman tree for encoding
sus text tea = "aaaaabbbbbccccdde"
sus tree HuffmanTree = build_huffman_tree(text)
assert_true(tree.root != nil)

# Test with uniform distribution
sus uniform_text tea = "abcdefghij"
sus uniform_tree HuffmanTree = build_huffman_tree(uniform_text)
assert_true(uniform_tree.root != nil)

# Test with single character
sus single_char tea = "aaaaaaaaaa"
sus single_tree HuffmanTree = build_huffman_tree(single_char)
assert_true(single_tree.root != nil)
print_test_summary()

test_start("test_huffman_encode")
# Test Huffman encoding
sus text tea = "hello world hello"
sus tree HuffmanTree = build_huffman_tree(text)
sus encoded tea = huffman_encode(text, tree)
assert_true(len(encoded) > 0)

# Encoded data should typically be shorter for repetitive text
sus repetitive tea = "aaaaaabbbbbbccccccdddddd"
sus rep_tree HuffmanTree = build_huffman_tree(repetitive)
sus rep_encoded tea = huffman_encode(repetitive, rep_tree)
assert_true(len(rep_encoded) > 0)
print_test_summary()

test_start("test_huffman_decode")
# Test Huffman decoding (round trip)
sus original tea = "huffman encoding and decoding test"
sus tree HuffmanTree = build_huffman_tree(original)
sus encoded tea = huffman_encode(original, tree)
sus decoded tea = huffman_decode(encoded, tree)
assert_eq_tea(decoded, original)

# Test with simple text
sus simple tea = "aab"
sus simple_tree HuffmanTree = build_huffman_tree(simple)
sus simple_encoded tea = huffman_encode(simple, simple_tree)
sus simple_decoded tea = huffman_decode(simple_encoded, simple_tree)
assert_eq_tea(simple_decoded, simple)
print_test_summary()

test_start("test_calculate_crc32")
# Test CRC32 calculation
sus data tea = "CRC32 checksum test data"
sus crc drip = calculate_crc32(data)
assert_true(crc != 0)

# Same data should produce same CRC
sus crc2 drip = calculate_crc32(data)
assert_eq_int(crc, crc2)

# Different data should produce different CRC
sus different_data tea = "Different CRC32 test data"
sus different_crc drip = calculate_crc32(different_data)
assert_true(different_crc != crc)

# Empty data CRC
sus empty_crc drip = calculate_crc32("")
assert_true(empty_crc != crc)
print_test_summary()

test_start("test_create_gzip_header")
# Test GZIP header creation
sus header tea = create_gzip_header("testfile.txt")
assert_true(len(header) > 0)

# Header should start with GZIP magic numbers
assert_eq_int(char_code(char_at(header, 0)), 0x1f)
assert_eq_int(char_code(char_at(header, 1)), 0x8b)

# Test with different filename
sus header2 tea = create_gzip_header("different.dat")
assert_true(len(header2) > 0)
assert_true(len(header2) >= len(header))  # Longer filename = longer header
print_test_summary()

test_start("test_verify_gzip_header")
# Test GZIP header verification
sus valid_header tea = create_gzip_header("test.txt")
sus is_valid lit = verify_gzip_header(valid_header)
assert_true(is_valid)

# Test with invalid header
sus invalid_header tea = "Not a GZIP header"
sus is_invalid lit = verify_gzip_header(invalid_header)
assert_true(!is_invalid)

# Test with partial header
sus partial_header tea = "\x1f"  # Only first magic byte
sus is_partial lit = verify_gzip_header(partial_header)
assert_true(!is_partial)
print_test_summary()

# Performance and stress tests
test_start("performance_compression")
# Test compression performance with various data sizes
sus small_data tea = "Small test data"
sus medium_data tea = ""
bestie i := 0; i < 100; i++ {
    medium_data = medium_data + "Medium size test data for performance testing. "
}

# Test multiple compression algorithms
sus small_gzip tea = gzip_compress(small_data)
sus small_deflate tea = deflate_compress(small_data)
sus small_lz77 tea = lz77_compress(small_data)

assert_true(len(small_gzip) > 0)
assert_true(len(small_deflate) > 0)
assert_true(len(small_lz77) > 0)

# Test medium data
sus medium_gzip tea = gzip_compress(medium_data)
sus medium_deflate tea = deflate_compress(medium_data)

assert_true(len(medium_gzip) > 0)
assert_true(len(medium_deflate) > 0)
print_test_summary()

# Edge cases and error handling
test_start("edge_cases_compression")
# Test with very short data
sus tiny_data tea = "x"
sus tiny_compressed tea = gzip_compress(tiny_data)
sus tiny_decompressed tea = gzip_decompress(tiny_compressed)
assert_eq_tea(tiny_decompressed, tiny_data)

# Test with binary data
sus binary_data tea = "\x00\x01\x02\x03\xFF\xFE\xFD\xFC"
sus binary_compressed tea = gzip_compress(binary_data)
sus binary_decompressed tea = gzip_decompress(binary_compressed)
assert_eq_tea(binary_decompressed, binary_data)

# Test with highly repetitive data
sus repetitive tea = ""
bestie i := 0; i < 1000; i++ {
    repetitive = repetitive + "A"
}
sus rep_compressed tea = gzip_compress(repetitive)
sus rep_decompressed tea = gzip_decompress(rep_compressed)
assert_eq_tea(rep_decompressed, repetitive)
assert_true(len(rep_compressed) < len(repetitive) / 10)  # Should compress very well
print_test_summary()

# Integration test - Full compression workflow
test_start("integration_compression_workflow")
# Complete compression workflow with multiple algorithms
sus original_data tea = "This is a comprehensive test of the compression module. " +
                       "It includes various compression algorithms like GZIP, DEFLATE, " +
                       "LZ77, and Huffman encoding. The goal is to verify that all " +
                       "compression and decompression operations work correctly together."

# Test GZIP workflow
sus gzip_compressed tea = gzip_compress(original_data)
sus gzip_header tea = create_gzip_header("test.gz")
assert_true(verify_gzip_header(gzip_header))
sus gzip_decompressed tea = gzip_decompress(gzip_compressed)
assert_eq_tea(gzip_decompressed, original_data)

# Test DEFLATE workflow
sus deflate_compressed tea = deflate_compress(original_data)
sus deflate_decompressed tea = deflate_decompress(deflate_compressed)
assert_eq_tea(deflate_decompressed, original_data)

# Test LZ77 workflow
sus lz77_compressed tea = lz77_compress(original_data)
sus lz77_decompressed tea = lz77_decompress(lz77_compressed)
assert_eq_tea(lz77_decompressed, original_data)

# Test Huffman workflow
sus huffman_tree HuffmanTree = build_huffman_tree(original_data)
sus huffman_encoded tea = huffman_encode(original_data, huffman_tree)
sus huffman_decoded tea = huffman_decode(huffman_encoded, huffman_tree)
assert_eq_tea(huffman_decoded, original_data)

# Verify integrity with CRC32
sus original_crc drip = calculate_crc32(original_data)
sus gzip_result_crc drip = calculate_crc32(gzip_decompressed)
sus deflate_result_crc drip = calculate_crc32(deflate_decompressed)
sus lz77_result_crc drip = calculate_crc32(lz77_decompressed)
sus huffman_result_crc drip = calculate_crc32(huffman_decoded)

assert_eq_int(original_crc, gzip_result_crc)
assert_eq_int(original_crc, deflate_result_crc)
assert_eq_int(original_crc, lz77_result_crc)
assert_eq_int(original_crc, huffman_result_crc)

vibez.spill("Compression integration test completed successfully")
print_test_summary()
