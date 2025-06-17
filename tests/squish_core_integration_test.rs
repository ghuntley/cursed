/// Comprehensive integration tests for SquishCore compression module
/// 
/// This test suite validates the entire SquishCore compression ecosystem
/// including all compression formats, enhanced features, and edge cases.

use cursed::stdlib::squish_core::{
    SquishError, SquishResult, CompressionLevel,
    // Core functions
    compress, decompress, compress_with_level, decompress_with_validation,
    // Format-specific functions
    gzip_compress, gzip_decompress, gzip_compress_level,
    zlib_compress, zlib_decompress, zlib_compress_level,
    flate_compress, flate_decompress, flate_compress_level,
    bzip2_compress, bzip2_decompress, bzip2_compress_level,
    lzw_compress, lzw_decompress,
    // Enhanced features
    compress_adaptive, compress_parallel, compress_with_dictionary,
    AdaptiveOptions, ParallelOptions,
    // Utilities
    detect_format, estimate_compression_ratio, validate_compressed_data,
    get_compression_info, format_compression_stats,
    // Quick functions
    squish, unsquish,
};

/// Test data samples for compression testing
const SMALL_TEXT: &[u8] = b"Hello, World! This is a small test.";
const MEDIUM_TEXT: &[u8] = b"This is a medium-sized text for testing compression algorithms. It contains repetitive patterns and should compress reasonably well with most algorithms. ";
const REPETITIVE_DATA: &[u8] = &[b'A'; 1000];
const RANDOM_DATA: &[u8] = &[0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x6f, 0x70, 0x81, 0x92, 0xa3, 0xb4, 0xc5];

#[test]
fn test_squish_core_initialization() {
    // Test that the module initializes without errors
    cursed::stdlib::squish_core::initialize();
}

#[test]
fn test_basic_compression_roundtrip() {
    // Test basic compression and decompression with all formats
    
    // GZIP
    let gzip_compressed = gzip_compress(MEDIUM_TEXT).expect("GZIP compression should succeed");
    let gzip_decompressed = gzip_decompress(&gzip_compressed).expect("GZIP decompression should succeed");
    assert_eq!(gzip_decompressed, MEDIUM_TEXT);
    
    // ZLIB
    let zlib_compressed = zlib_compress(MEDIUM_TEXT).expect("ZLIB compression should succeed");
    let zlib_decompressed = zlib_decompress(&zlib_compressed).expect("ZLIB decompression should succeed");
    assert_eq!(zlib_decompressed, MEDIUM_TEXT);
    
    // DEFLATE
    let flate_compressed = flate_compress(MEDIUM_TEXT).expect("DEFLATE compression should succeed");
    let flate_decompressed = flate_decompress(&flate_compressed).expect("DEFLATE decompression should succeed");
    assert_eq!(flate_decompressed, MEDIUM_TEXT);
    
    // BZIP2
    let bzip2_compressed = bzip2_compress(MEDIUM_TEXT).expect("BZIP2 compression should succeed");
    let bzip2_decompressed = bzip2_decompress(&bzip2_compressed).expect("BZIP2 decompression should succeed");
    assert_eq!(bzip2_decompressed, MEDIUM_TEXT);
    
    // LZW
    let lzw_compressed = lzw_compress(MEDIUM_TEXT).expect("LZW compression should succeed");
    let lzw_decompressed = lzw_decompress(&lzw_compressed).expect("LZW decompression should succeed");
    assert_eq!(lzw_decompressed, MEDIUM_TEXT);
}

#[test]
fn test_compression_levels() {
    // Test different compression levels
    let levels = [
        CompressionLevel::None,
        CompressionLevel::Fastest,
        CompressionLevel::Fast,
        CompressionLevel::Default,
        CompressionLevel::Best,
    ];
    
    for level in levels {
        let gzip_compressed = gzip_compress_level(REPETITIVE_DATA, level).expect("GZIP level compression should succeed");
        let gzip_decompressed = gzip_decompress(&gzip_compressed).expect("GZIP level decompression should succeed");
        assert_eq!(gzip_decompressed, REPETITIVE_DATA);
        
        let zlib_compressed = zlib_compress_level(REPETITIVE_DATA, level).expect("ZLIB level compression should succeed");
        let zlib_decompressed = zlib_decompress(&zlib_compressed).expect("ZLIB level decompression should succeed");
        assert_eq!(zlib_decompressed, REPETITIVE_DATA);
    }
}

#[test]
fn test_empty_data_compression() {
    // Test compression of empty data
    let empty = b"";
    
    let gzip_compressed = gzip_compress(empty).expect("GZIP empty compression should succeed");
    let gzip_decompressed = gzip_decompress(&gzip_compressed).expect("GZIP empty decompression should succeed");
    assert_eq!(gzip_decompressed, empty);
    
    let zlib_compressed = zlib_compress(empty).expect("ZLIB empty compression should succeed");
    let zlib_decompressed = zlib_decompress(&zlib_compressed).expect("ZLIB empty decompression should succeed");
    assert_eq!(zlib_decompressed, empty);
}

#[test]
fn test_large_data_compression() {
    // Test compression of larger data sets
    let large_data = vec![b'X'; 10000];
    
    let gzip_compressed = gzip_compress(&large_data).expect("GZIP large compression should succeed");
    let gzip_decompressed = gzip_decompress(&gzip_compressed).expect("GZIP large decompression should succeed");
    assert_eq!(gzip_decompressed, large_data);
    
    // Should achieve good compression ratio on repetitive data
    assert!(gzip_compressed.len() < large_data.len() / 10);
}

#[test]
fn test_compression_ratio_expectations() {
    // Test that compression ratios meet expectations
    
    // Highly repetitive data should compress very well
    let repetitive_compressed = gzip_compress(REPETITIVE_DATA).expect("Repetitive compression should succeed");
    let repetitive_ratio = repetitive_compressed.len() as f64 / REPETITIVE_DATA.len() as f64;
    assert!(repetitive_ratio < 0.1, "Repetitive data should compress to <10%");
    
    // Random data should not compress much
    let random_compressed = gzip_compress(RANDOM_DATA).expect("Random compression should succeed");
    let random_ratio = random_compressed.len() as f64 / RANDOM_DATA.len() as f64;
    // Note: Very small random data might actually grow due to headers
    assert!(random_ratio > 0.5, "Random data should not compress much");
}

#[test]
fn test_high_level_compression_functions() {
    // Test the high-level compress/decompress functions
    
    let gzip_compressed = compress(MEDIUM_TEXT, "gzip").expect("High-level GZIP compression should succeed");
    let gzip_decompressed = decompress(&gzip_compressed, "gzip").expect("High-level GZIP decompression should succeed");
    assert_eq!(gzip_decompressed, MEDIUM_TEXT);
    
    let zlib_compressed = compress(MEDIUM_TEXT, "zlib").expect("High-level ZLIB compression should succeed");
    let zlib_decompressed = decompress(&zlib_compressed, "zlib").expect("High-level ZLIB decompression should succeed");
    assert_eq!(zlib_decompressed, MEDIUM_TEXT);
    
    // Test with compression levels
    let level_compressed = compress_with_level(MEDIUM_TEXT, "gzip", CompressionLevel::Best).expect("Level compression should succeed");
    let level_decompressed = decompress(&level_compressed, "gzip").expect("Level decompression should succeed");
    assert_eq!(level_decompressed, MEDIUM_TEXT);
}

#[test]
fn test_compression_with_validation() {
    // Test decompression with size validation
    let compressed = gzip_compress(MEDIUM_TEXT).expect("Compression for validation test should succeed");
    
    // Valid size
    let decompressed = decompress_with_validation(&compressed, "gzip", Some(MEDIUM_TEXT.len())).expect("Valid size validation should succeed");
    assert_eq!(decompressed, MEDIUM_TEXT);
    
    // Invalid size should fail
    let result = decompress_with_validation(&compressed, "gzip", Some(MEDIUM_TEXT.len() + 100));
    assert!(result.is_err(), "Invalid size validation should fail");
}

#[test]
fn test_quick_squish_functions() {
    // Test the quick squish/unsquish functions
    let compressed = squish(MEDIUM_TEXT).expect("Quick squish should succeed");
    let decompressed = unsquish(&compressed).expect("Quick unsquish should succeed");
    assert_eq!(decompressed, MEDIUM_TEXT);
}

#[test]
fn test_format_detection() {
    // Test automatic format detection
    let gzip_compressed = gzip_compress(MEDIUM_TEXT).expect("GZIP compression for detection should succeed");
    let gzip_format = detect_format(&gzip_compressed).expect("GZIP format detection should succeed");
    assert_eq!(gzip_format, cursed::stdlib::squish_core::utils::CompressionFormat::Gzip);
    
    let zlib_compressed = zlib_compress(MEDIUM_TEXT).expect("ZLIB compression for detection should succeed");
    let zlib_format = detect_format(&zlib_compressed).expect("ZLIB format detection should succeed");
    assert_eq!(zlib_format, cursed::stdlib::squish_core::utils::CompressionFormat::Zlib);
}

#[test]
fn test_compression_estimation() {
    // Test compression ratio estimation
    let gzip_estimate = estimate_compression_ratio(REPETITIVE_DATA, cursed::stdlib::squish_core::utils::CompressionFormat::Gzip);
    assert!(gzip_estimate < 0.5, "Repetitive data should have low estimated ratio");
    
    let random_estimate = estimate_compression_ratio(RANDOM_DATA, cursed::stdlib::squish_core::utils::CompressionFormat::Gzip);
    assert!(random_estimate > 0.5, "Random data should have high estimated ratio");
}

#[test]
fn test_data_validation() {
    // Test compressed data validation
    let valid_gzip = gzip_compress(MEDIUM_TEXT).expect("Valid GZIP for validation should succeed");
    let is_valid = validate_compressed_data(&valid_gzip, cursed::stdlib::squish_core::utils::CompressionFormat::Gzip).expect("Validation should succeed");
    assert!(is_valid, "Valid GZIP data should validate");
    
    let invalid_data = vec![0x00, 0x01, 0x02, 0x03];
    let is_invalid = validate_compressed_data(&invalid_data, cursed::stdlib::squish_core::utils::CompressionFormat::Gzip).expect("Invalid validation should succeed");
    assert!(!is_invalid, "Invalid data should not validate");
}

#[test]
fn test_compression_info() {
    // Test compression information extraction
    let gzip_compressed = gzip_compress(MEDIUM_TEXT).expect("GZIP for info test should succeed");
    let info = get_compression_info(&gzip_compressed).expect("Compression info should be extractable");
    
    assert_eq!(info.format, cursed::stdlib::squish_core::utils::CompressionFormat::Gzip);
    assert!(info.is_valid);
    assert_eq!(info.original_size, gzip_compressed.len());
}

#[test] 
fn test_adaptive_compression() {
    // Test adaptive compression feature
    let options = AdaptiveOptions::default();
    let compressed = compress_adaptive(MEDIUM_TEXT, &options).expect("Adaptive compression should succeed");
    assert!(!compressed.is_empty());
    assert!(compressed.len() < MEDIUM_TEXT.len() + 100); // Should achieve some compression
}

#[test]
fn test_parallel_compression() {
    // Test parallel compression feature
    let large_data = vec![b'P'; 5000];
    let options = ParallelOptions {
        num_threads: 2,
        chunk_size: 1000,
        chunk_overlap: 100,
    };
    
    let compressed = compress_parallel(&large_data, "gzip", &options).expect("Parallel compression should succeed");
    assert!(!compressed.is_empty());
}

#[test]
fn test_dictionary_compression() {
    // Test dictionary-based compression
    let dictionary = b"common pattern ";
    let data = b"common pattern in data common pattern again common pattern everywhere";
    
    let compressed = compress_with_dictionary(data, dictionary, "gzip").expect("Dictionary compression should succeed");
    assert!(!compressed.is_empty());
}

#[test]
fn test_error_handling() {
    // Test various error conditions
    
    // Invalid algorithm
    let result = compress(MEDIUM_TEXT, "invalid_algorithm");
    assert!(result.is_err(), "Invalid algorithm should fail");
    
    // Invalid compressed data
    let invalid_data = vec![0xff, 0xfe, 0xfd, 0xfc];
    let result = gzip_decompress(&invalid_data);
    assert!(result.is_err(), "Invalid compressed data should fail");
    
    // Unsupported format in high-level functions
    let result = decompress(&invalid_data, "unknown_format");
    assert!(result.is_err(), "Unknown format should fail");
}

#[test]
fn test_compression_statistics() {
    // Test that compression operations provide meaningful statistics
    let data = MEDIUM_TEXT.repeat(10); // Larger data for meaningful stats
    
    let timer = std::time::Instant::now();
    let compressed = gzip_compress(&data).expect("Compression for stats should succeed");
    let duration = timer.elapsed();
    
    // Create manual stats for verification
    let stats = cursed::stdlib::squish_core::CompressionStats::new(
        data.len(),
        compressed.len(),
        duration,
        "gzip".to_string(),
        Some(6),
    );
    
    assert_eq!(stats.input_size, data.len());
    assert_eq!(stats.output_size, compressed.len());
    assert!(stats.compression_ratio > 0.0);
    assert!(stats.space_saved_percent >= 0.0);
    assert!(stats.throughput_bps > 0.0);
    
    // Test formatting
    let formatted = format_compression_stats(&stats);
    assert!(formatted.contains("gzip"));
    assert!(formatted.contains(&data.len().to_string()));
}

#[test]
fn test_compression_level_conversion() {
    // Test compression level conversions
    assert_eq!(CompressionLevel::None.to_numeric(), 0);
    assert_eq!(CompressionLevel::Fastest.to_numeric(), 1);
    assert_eq!(CompressionLevel::Default.to_numeric(), 6);
    assert_eq!(CompressionLevel::Best.to_numeric(), 9);
    
    assert_eq!(CompressionLevel::from_numeric(0).unwrap(), CompressionLevel::None);
    assert_eq!(CompressionLevel::from_numeric(1).unwrap(), CompressionLevel::Fastest);
    assert_eq!(CompressionLevel::from_numeric(6).unwrap(), CompressionLevel::Default);
    assert_eq!(CompressionLevel::from_numeric(9).unwrap(), CompressionLevel::Best);
    
    // Invalid levels
    assert!(CompressionLevel::from_numeric(-10).is_err());
    assert!(CompressionLevel::from_numeric(100).is_err());
}

#[test]
fn test_cross_format_compatibility() {
    // Test that different formats produce different but valid results
    let gzip_compressed = gzip_compress(MEDIUM_TEXT).expect("GZIP compression should succeed");
    let zlib_compressed = zlib_compress(MEDIUM_TEXT).expect("ZLIB compression should succeed");
    let flate_compressed = flate_compress(MEDIUM_TEXT).expect("DEFLATE compression should succeed");
    
    // Different formats should produce different compressed data
    assert_ne!(gzip_compressed, zlib_compressed);
    assert_ne!(gzip_compressed, flate_compressed);
    assert_ne!(zlib_compressed, flate_compressed);
    
    // But all should decompress to the same original data
    assert_eq!(gzip_decompress(&gzip_compressed).unwrap(), MEDIUM_TEXT);
    assert_eq!(zlib_decompress(&zlib_compressed).unwrap(), MEDIUM_TEXT);
    assert_eq!(flate_decompress(&flate_compressed).unwrap(), MEDIUM_TEXT);
}

#[test]
fn test_module_constants() {
    // Test module constants and metadata
    assert_eq!(cursed::stdlib::squish_core::VERSION, "1.0.0");
    assert_eq!(cursed::stdlib::squish_core::MODULE_NAME, "squish_core");
    
    let info = cursed::stdlib::squish_core::module_info();
    assert!(info.contains("squish_core"));
    assert!(info.contains("1.0.0"));
    assert!(info.contains("Compression"));
}

#[test]
fn test_edge_cases() {
    // Test various edge cases
    
    // Single byte
    let single_byte = vec![42u8];
    let compressed = gzip_compress(&single_byte).expect("Single byte compression should succeed");
    let decompressed = gzip_decompress(&compressed).expect("Single byte decompression should succeed");
    assert_eq!(decompressed, single_byte);
    
    // All zeros
    let zeros = vec![0u8; 100];
    let compressed = gzip_compress(&zeros).expect("Zero compression should succeed");
    let decompressed = gzip_decompress(&compressed).expect("Zero decompression should succeed");
    assert_eq!(decompressed, zeros);
    assert!(compressed.len() < zeros.len() / 10, "Zeros should compress very well");
    
    // All 255s
    let max_bytes = vec![255u8; 100];
    let compressed = gzip_compress(&max_bytes).expect("Max byte compression should succeed");
    let decompressed = gzip_decompress(&compressed).expect("Max byte decompression should succeed");
    assert_eq!(decompressed, max_bytes);
}

#[test]
fn test_unicode_text_compression() {
    // Test compression of Unicode text
    let unicode_text = "Hello, 世界! 🌍 This is Unicode text with émojis and spëcial characters.";
    let unicode_bytes = unicode_text.as_bytes();
    
    let compressed = gzip_compress(unicode_bytes).expect("Unicode compression should succeed");
    let decompressed = gzip_decompress(&compressed).expect("Unicode decompression should succeed");
    assert_eq!(decompressed, unicode_bytes);
    
    let decompressed_text = String::from_utf8(decompressed).expect("Unicode should be valid UTF-8");
    assert_eq!(decompressed_text, unicode_text);
}

#[test]
fn test_performance_characteristics() {
    // Test that compression has reasonable performance characteristics
    let large_data = vec![b'T'; 50000];
    
    let start_time = std::time::Instant::now();
    let compressed = gzip_compress(&large_data).expect("Large data compression should succeed");
    let compression_time = start_time.elapsed();
    
    let start_time = std::time::Instant::now();
    let decompressed = gzip_decompress(&compressed).expect("Large data decompression should succeed");
    let decompression_time = start_time.elapsed();
    
    assert_eq!(decompressed, large_data);
    
    // Performance should be reasonable (less than 1 second for 50KB)
    assert!(compression_time.as_secs() < 1, "Compression should be fast");
    assert!(decompression_time.as_secs() < 1, "Decompression should be fast");
    
    // Compression should be effective for repetitive data
    let compression_ratio = compressed.len() as f64 / large_data.len() as f64;
    assert!(compression_ratio < 0.1, "Repetitive data should compress to <10%");
}
