// Test Suite for zip_zilla - Archive/Compression Module
// Comprehensive testing using testz v2.0 framework

yeet "testz"
yeet "zip_zilla"

// Test deflate compression/decompression
slay test_deflate_compression() {
    test_start("deflate compression basic")
    
    sus original_data tea = "hello world hello world"
    sus compressed tea = deflate_compress(original_data, 6)
    sus decompressed tea = deflate_decompress(compressed)
    
    assert_eq_string(decompressed, original_data)
    assert_true(compressed.length <= original_data.length)
}

slay test_deflate_empty_data() {
    test_start("deflate empty data")
    
    sus original_data tea = ""
    sus compressed tea = deflate_compress(original_data, 6)
    sus decompressed tea = deflate_decompress(compressed)
    
    assert_eq_string(decompressed, original_data)
}

slay test_deflate_repetitive_data() {
    test_start("deflate repetitive data")
    
    sus original_data tea = "aaaaaaaaaaaaaaaaaaaaaa"
    sus compressed tea = deflate_compress(original_data, 6)
    sus decompressed tea = deflate_decompress(compressed)
    
    assert_eq_string(decompressed, original_data)
    assert_true(compressed.length < original_data.length)
}

slay test_deflate_mixed_data() {
    test_start("deflate mixed data")
    
    sus original_data tea = "abcdefghijklmnopqrstuvwxyz0123456789"
    sus compressed tea = deflate_compress(original_data, 6)
    sus decompressed tea = deflate_decompress(compressed)
    
    assert_eq_string(decompressed, original_data)
}

slay test_deflate_compression_levels() {
    test_start("deflate compression levels")
    
    sus original_data tea = "test data for compression levels"
    sus compressed_1 tea = deflate_compress(original_data, 1)
    sus compressed_9 tea = deflate_compress(original_data, 9)
    
    sus decompressed_1 tea = deflate_decompress(compressed_1)
    sus decompressed_9 tea = deflate_decompress(compressed_9)
    
    assert_eq_string(decompressed_1, original_data)
    assert_eq_string(decompressed_9, original_data)
}

// Test ZIP archive functions
slay test_zip_create_basic() {
    test_start("zip create basic")
    
    sus files [tea] = ["file1.txt", "file2.txt"]
    sus contents [tea] = ["content1", "content2"]
    
    sus result lit = zip_create("test.zip", files, contents)
    assert_true(result)
}

slay test_zip_create_empty() {
    test_start("zip create empty")
    
    sus files [tea] = []
    sus contents [tea] = []
    
    sus result lit = zip_create("empty.zip", files, contents)
    assert_true(result)
}

slay test_zip_create_mismatched_arrays() {
    test_start("zip create mismatched arrays")
    
    sus files [tea] = ["file1.txt", "file2.txt"]
    sus contents [tea] = ["content1"]
    
    sus result lit = zip_create("test.zip", files, contents)
    assert_false(result)
}

slay test_zip_extract_basic() {
    test_start("zip extract basic")
    
    sus archive_data tea = "PK\x03\x04test_data"
    sus (success, files, contents) = zip_extract(archive_data)
    
    assert_true(success)
    assert_true(files.length >= 0)
    assert_true(contents.length >= 0)
}

slay test_zip_extract_invalid_signature() {
    test_start("zip extract invalid signature")
    
    sus archive_data tea = "INVALID"
    sus (success, files, contents) = zip_extract(archive_data)
    
    assert_false(success)
    assert_eq_int(files.length, 0)
    assert_eq_int(contents.length, 0)
}

slay test_zip_extract_empty_data() {
    test_start("zip extract empty data")
    
    sus archive_data tea = ""
    sus (success, files, contents) = zip_extract(archive_data)
    
    assert_false(success)
    assert_eq_int(files.length, 0)
    assert_eq_int(contents.length, 0)
}

// Test GZIP functions
slay test_gzip_compress_basic() {
    test_start("gzip compress basic")
    
    sus original_data tea = "hello gzip world"
    sus compressed tea = gzip_compress(original_data, 6)
    
    assert_true(compressed.length >= 10)  // GZIP header is 10 bytes
    assert_true(compressed.substring(0, 2) == "\x1f\x8b")  // GZIP magic
}

slay test_gzip_decompress_basic() {
    test_start("gzip decompress basic")
    
    sus original_data tea = "hello gzip world"
    sus compressed tea = gzip_compress(original_data, 6)
    sus (success, decompressed) = gzip_decompress(compressed)
    
    assert_true(success)
    assert_eq_string(decompressed, original_data)
}

slay test_gzip_decompress_invalid_magic() {
    test_start("gzip decompress invalid magic")
    
    sus invalid_data tea = "invalid gzip data"
    sus (success, decompressed) = gzip_decompress(invalid_data)
    
    assert_false(success)
    assert_eq_string(decompressed, "")
}

slay test_gzip_decompress_short_data() {
    test_start("gzip decompress short data")
    
    sus short_data tea = "short"
    sus (success, decompressed) = gzip_decompress(short_data)
    
    assert_false(success)
    assert_eq_string(decompressed, "")
}

slay test_gzip_empty_data() {
    test_start("gzip empty data")
    
    sus original_data tea = ""
    sus compressed tea = gzip_compress(original_data, 6)
    sus (success, decompressed) = gzip_decompress(compressed)
    
    assert_true(success)
    assert_eq_string(decompressed, original_data)
}

// Test compression utilities
slay test_calculate_compression_ratio() {
    test_start("calculate compression ratio")
    
    sus ratio meal = calculate_compression_ratio(100, 50)
    assert_true(ratio >= 49.0 && ratio <= 51.0)  // 50% compression
    
    sus ratio_zero meal = calculate_compression_ratio(0, 0)
    assert_true(ratio_zero == 0.0)
    
    sus ratio_no_compression meal = calculate_compression_ratio(100, 100)
    assert_true(ratio_no_compression == 0.0)
}

slay test_compress_file_deflate() {
    test_start("compress file deflate")
    
    sus result lit = compress_file("test.txt", "deflate", 6)
    assert_true(result)
}

slay test_compress_file_gzip() {
    test_start("compress file gzip")
    
    sus result lit = compress_file("test.txt", "gzip", 6)
    assert_true(result)
}

slay test_compress_file_invalid_type() {
    test_start("compress file invalid type")
    
    sus result lit = compress_file("test.txt", "invalid", 6)
    assert_false(result)
}

slay test_decompress_file_deflate() {
    test_start("decompress file deflate")
    
    sus (success, data) = decompress_file("test.txt", "deflate")
    assert_true(success)
    assert_true(data.length >= 0)
}

slay test_decompress_file_gzip() {
    test_start("decompress file gzip")
    
    sus (success, data) = decompress_file("test.txt", "gzip")
    assert_true(success)
    assert_true(data.length >= 0)
}

slay test_decompress_file_invalid_type() {
    test_start("decompress file invalid type")
    
    sus (success, data) = decompress_file("test.txt", "invalid")
    assert_false(success)
    assert_eq_string(data, "")
}

// Test archive management
slay test_create_archive_zip() {
    test_start("create archive zip")
    
    sus files [tea] = ["file1.txt", "file2.txt"]
    sus result lit = create_archive("test.zip", files, "zip")
    assert_true(result)
}

slay test_create_archive_invalid_type() {
    test_start("create archive invalid type")
    
    sus files [tea] = ["file1.txt"]
    sus result lit = create_archive("test.unknown", files, "unknown")
    assert_false(result)
}

slay test_extract_archive_zip() {
    test_start("extract archive zip")
    
    sus (success, files, contents) = extract_archive("test.zip", "zip")
    assert_true(success)
    assert_true(files.length >= 0)
    assert_true(contents.length >= 0)
}

slay test_extract_archive_invalid_type() {
    test_start("extract archive invalid type")
    
    sus (success, files, contents) = extract_archive("test.unknown", "unknown")
    assert_false(success)
    assert_eq_int(files.length, 0)
    assert_eq_int(contents.length, 0)
}

// Test compression benchmarking
slay test_benchmark_compression() {
    test_start("benchmark compression")
    
    sus data tea = "benchmark test data for compression algorithms"
    sus algorithms [tea] = ["deflate", "gzip"]
    sus results [meal] = benchmark_compression(data, algorithms)
    
    assert_eq_int(results.length, 2)
    assert_true(results[0] >= 0.0)
    assert_true(results[1] >= 0.0)
}

slay test_benchmark_compression_empty_algorithms() {
    test_start("benchmark compression empty algorithms")
    
    sus data tea = "test data"
    sus algorithms [tea] = []
    sus results [meal] = benchmark_compression(data, algorithms)
    
    assert_eq_int(results.length, 0)
}

// Test integrity checking
slay test_verify_archive_integrity_zip() {
    test_start("verify archive integrity zip")
    
    sus archive_data tea = "PK\x03\x04valid_zip_data"
    sus result lit = verify_archive_integrity(archive_data, "zip")
    assert_true(result)
}

slay test_verify_archive_integrity_gzip() {
    test_start("verify archive integrity gzip")
    
    sus gzip_data tea = gzip_compress("test data", 6)
    sus result lit = verify_archive_integrity(gzip_data, "gzip")
    assert_true(result)
}

slay test_verify_archive_integrity_invalid_type() {
    test_start("verify archive integrity invalid type")
    
    sus archive_data tea = "test data"
    sus result lit = verify_archive_integrity(archive_data, "unknown")
    assert_false(result)
}

slay test_calculate_checksum() {
    test_start("calculate checksum")
    
    sus data tea = "test data"
    sus checksum normie = calculate_checksum(data)
    assert_true(checksum >= 0)
    
    sus checksum_same normie = calculate_checksum(data)
    assert_eq_int(checksum, checksum_same)
    
    sus checksum_different normie = calculate_checksum("different data")
    assert_true(checksum != checksum_different)
}

slay test_calculate_checksum_empty() {
    test_start("calculate checksum empty")
    
    sus data tea = ""
    sus checksum normie = calculate_checksum(data)
    assert_eq_int(checksum, 0)
}

// Test round-trip operations
slay test_deflate_roundtrip() {
    test_start("deflate roundtrip")
    
    sus original tea = "The quick brown fox jumps over the lazy dog"
    sus compressed tea = deflate_compress(original, 6)
    sus decompressed tea = deflate_decompress(compressed)
    
    assert_eq_string(decompressed, original)
}

slay test_gzip_roundtrip() {
    test_start("gzip roundtrip")
    
    sus original tea = "GZIP compression test with various characters!@#$%^&*()"
    sus compressed tea = gzip_compress(original, 6)
    sus (success, decompressed) = gzip_decompress(compressed)
    
    assert_true(success)
    assert_eq_string(decompressed, original)
}

slay test_compression_ratio_calculation() {
    test_start("compression ratio calculation")
    
    sus original tea = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    sus compressed tea = deflate_compress(original, 9)
    sus ratio meal = calculate_compression_ratio(original.length, compressed.length)
    
    assert_true(ratio > 0.0)  // Should achieve some compression
    assert_true(ratio <= 100.0)  // Cannot compress more than 100%
}

// Run all tests
test_deflate_compression()
test_deflate_empty_data()
test_deflate_repetitive_data()
test_deflate_mixed_data()
test_deflate_compression_levels()

test_zip_create_basic()
test_zip_create_empty()
test_zip_create_mismatched_arrays()
test_zip_extract_basic()
test_zip_extract_invalid_signature()
test_zip_extract_empty_data()

test_gzip_compress_basic()
test_gzip_decompress_basic()
test_gzip_decompress_invalid_magic()
test_gzip_decompress_short_data()
test_gzip_empty_data()

test_calculate_compression_ratio()
test_compress_file_deflate()
test_compress_file_gzip()
test_compress_file_invalid_type()
test_decompress_file_deflate()
test_decompress_file_gzip()
test_decompress_file_invalid_type()

test_create_archive_zip()
test_create_archive_invalid_type()
test_extract_archive_zip()
test_extract_archive_invalid_type()

test_benchmark_compression()
test_benchmark_compression_empty_algorithms()

test_verify_archive_integrity_zip()
test_verify_archive_integrity_gzip()
test_verify_archive_integrity_invalid_type()
test_calculate_checksum()
test_calculate_checksum_empty()

test_deflate_roundtrip()
test_gzip_roundtrip()
test_compression_ratio_calculation()

print_test_summary()
