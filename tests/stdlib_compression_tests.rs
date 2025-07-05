//! Unit tests for CURSED squish_core compression library

use cursed::stdlib::squish_core::{self as squish, zlib};

#[test]
fn test_zlib_compression_decompression() {
    let test_data = b"Hello, CURSED compression world! This is a test string for compression.";
    
    // Test basic compression and decompression
    let compressed = zlib::zlib_compress(test_data).unwrap();
    let decompressed = zlib::zlib_decompress(&compressed).unwrap();
    assert_eq!(decompressed, test_data);
    
    // For small data, compression overhead can make it larger than original
    // Test with larger, more compressible data 
    let large_test_data = b"This is a repeating pattern for compression testing. This is a repeating pattern for compression testing. This is a repeating pattern for compression testing. This is a repeating pattern for compression testing. This is a repeating pattern for compression testing. This is a repeating pattern for compression testing. This is a repeating pattern for compression testing. This is a repeating pattern for compression testing.";
    
    let large_compressed = zlib::zlib_compress(large_test_data).unwrap();
    let large_decompressed = zlib::zlib_decompress(&large_compressed).unwrap();
    
    assert_eq!(large_decompressed, large_test_data);
    // With repetitive data, compression should be effective
    assert!(large_compressed.len() < large_test_data.len());
}

#[test]
fn test_zlib_format_detection() {
    let test_data = b"Test data for ZLIB compression";
    let compressed = zlib::zlib_compress(test_data).unwrap();
    
    // Test format detection
    assert!(zlib::is_zlib_data(&compressed));
    assert!(!zlib::is_zlib_data(b"not compressed data"));
}

#[test]
fn test_compression_level() {
    let test_data = b"This is test data for compression level testing. It should be long enough to see differences in compression levels.";
    
    // Test different compression levels
    let level_1 = zlib::zlib_compress_level(test_data, squish::constants::CompressionLevel::Fast).unwrap();
    let level_9 = zlib::zlib_compress_level(test_data, squish::constants::CompressionLevel::Best).unwrap();
    
    // Both should decompress to the same data
    let decompressed_1 = zlib::zlib_decompress(&level_1).unwrap();
    let decompressed_9 = zlib::zlib_decompress(&level_9).unwrap();
    
    assert_eq!(decompressed_1, test_data);
    assert_eq!(decompressed_9, test_data);
}

#[test]
fn test_squish_unsquish() {
    let test_data = b"Test data for quick compression functions";
    
    // Test the convenience functions
    let compressed = squish::squish(test_data).unwrap();
    let decompressed = squish::unsquish(&compressed).unwrap();
    
    assert_eq!(decompressed, test_data);
}

#[test]
fn test_compression_with_empty_data() {
    let empty_data = b"";
    
    let compressed = zlib::zlib_compress(empty_data).unwrap();
    let decompressed = zlib::zlib_decompress(&compressed).unwrap();
    
    assert_eq!(decompressed, empty_data);
}

#[test]
fn test_compression_module_initialization() {
    // Test that initialization doesn't panic
    squish::initialize();
    zlib::initialize();
}

#[test]
fn test_compression_validation() {
    // Test valid compression level validation
    assert!(zlib::is_valid_compression_level(0));
    assert!(zlib::is_valid_compression_level(6));
    assert!(zlib::is_valid_compression_level(9));
    assert!(zlib::is_valid_compression_level(-1)); // Default
    assert!(!zlib::is_valid_compression_level(10));
    assert!(!zlib::is_valid_compression_level(-2));
}

#[test]
fn test_compression_file_extensions() {
    assert_eq!(zlib::file_extension(), ".zlib");
    assert_eq!(zlib::mime_type(), "application/zlib");
}
