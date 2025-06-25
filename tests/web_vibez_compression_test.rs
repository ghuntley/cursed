/// Comprehensive tests for the production-ready compression system
use cursed::stdlib::web_vibez::compression::*;
use std::collections::HashMap;

#[test]
fn test_compression_type_priority() {
    assert_eq!(CompressionType::Brotli.priority(), 10);
    assert_eq!(CompressionType::Zstd.priority(), 9);
    assert_eq!(CompressionType::Gzip.priority(), 8);
    assert_eq!(CompressionType::Deflate.priority(), 7);
    assert_eq!(CompressionType::Identity.priority(), 0);
}

#[test]
fn test_complex_accept_encoding_parsing() {
    let complex_header = "gzip;q=0.8, deflate;q=0.6, br;q=1.0, zstd;q=0.9, *;q=0.1";
    let encodings = CompressionType::from_accept_encoding(complex_header);

    assert_eq!(encodings.len(), 5);
    
    // Should be sorted by quality (descending)
    assert_eq!(encodings[0], (CompressionType::Brotli, 1.0));
    assert_eq!(encodings[1], (CompressionType::Zstd, 0.9));
    assert_eq!(encodings[2], (CompressionType::Gzip, 0.8));
    assert_eq!(encodings[3], (CompressionType::Deflate, 0.6));
    assert_eq!(encodings[4], (CompressionType::Identity, 0.1));
}

#[test]
fn test_malformed_accept_encoding() {
    let malformed = "gzip;q=invalid, deflate;q=, br;q=2.0, unknown";
    let encodings = CompressionType::from_accept_encoding(malformed);

    // Should handle malformed quality values gracefully
    assert!(encodings.iter().any(|(t, _)| *t == CompressionType::Gzip));
    assert!(encodings.iter().any(|(t, _)| *t == CompressionType::Deflate));
    assert!(encodings.iter().any(|(t, _)| *t == CompressionType::Brotli));
    
    // Unknown encoding should be ignored
    assert!(!encodings.iter().any(|(t, _)| matches!(t, CompressionType::Identity)));
}

#[test]
fn test_compression_config_validation() {
    let config = CompressionConfig {
        level: 15, // Should be clamped
        min_size_threshold: 500,
        max_size_threshold: 1000000,
        compressible_types: vec!["text/html".to_string()],
        buffer_size: 8192,
        enabled_types: vec![CompressionType::Gzip, CompressionType::Brotli],
        quality_threshold: 0.5,
    };

    let compressor = ResponseCompressor::with_config(config.clone());
    assert_eq!(compressor.get_config().level, 15); // Config stores original value
    assert_eq!(compressor.get_config().min_size_threshold, 500);
    assert_eq!(compressor.get_config().enabled_types.len(), 2);
}

#[test]
fn test_should_compress_with_custom_config() {
    let config = CompressionConfig {
        min_size_threshold: 100,
        max_size_threshold: 2000,
        compressible_types: vec!["application/custom".to_string()],
        ..Default::default()
    };
    
    let compressor = ResponseCompressor::with_config(config);
    
    // Should compress content within thresholds
    let medium_content = vec![b'x'; 500];
    assert!(compressor.should_compress(&medium_content, "application/custom"));
    
    // Should not compress too small content
    let small_content = vec![b'x'; 50];
    assert!(!compressor.should_compress(&small_content, "application/custom"));
    
    // Should not compress too large content
    let large_content = vec![b'x'; 3000];
    assert!(!compressor.should_compress(&large_content, "application/custom"));
    
    // Should not compress non-compressible types
    let medium_binary = vec![b'x'; 500];
    assert!(!compressor.should_compress(&medium_binary, "image/png"));
}

#[test]
fn test_compression_selection_with_quality_threshold() {
    let config = CompressionConfig {
        quality_threshold: 0.7,
        enabled_types: vec![CompressionType::Gzip, CompressionType::Brotli],
        ..Default::default()
    };
    
    let compressor = ResponseCompressor::with_config(config);
    
    // Should select high-quality encoding
    let high_quality = compressor.select_compression("gzip;q=0.8, deflate;q=0.6");
    assert_eq!(high_quality, CompressionType::Gzip);
    
    // Should reject low-quality encoding
    let low_quality = compressor.select_compression("gzip;q=0.5, deflate;q=0.4");
    assert_eq!(low_quality, CompressionType::Identity);
    
    // Should select best available above threshold
    let mixed_quality = compressor.select_compression("gzip;q=0.5, br;q=0.8");
    assert_eq!(mixed_quality, CompressionType::Brotli);
}

#[test]
fn test_all_compression_algorithms() {
    let mut compressor = ResponseCompressor::new();
    let test_data = b"This is a test string that should compress well because it has repetitive patterns and common words.".repeat(10);
    
    let algorithms = vec![
        CompressionType::Gzip,
        CompressionType::Deflate,
        CompressionType::Brotli,
        CompressionType::Zstd,
    ];
    
    for algorithm in algorithms {
        // Test compression
        let compressed = compressor.compress(&test_data, algorithm.clone())
            .expect(&format!("Failed to compress with {:?}", algorithm));
        
        assert!(compressed.len() < test_data.len(), 
                "Compression with {:?} should reduce size", algorithm);
        
        // Test decompression
        let decompressed = compressor.decompress(&compressed, algorithm.clone())
            .expect(&format!("Failed to decompress with {:?}", algorithm));
        
        assert_eq!(decompressed, test_data, 
                   "Round-trip with {:?} should preserve data", algorithm);
    }
}

#[test]
fn test_compression_levels() {
    let test_data = vec![b'a'; 1000]; // Highly compressible data
    
    let levels = vec![1, 6, 9];
    let mut sizes = Vec::new();
    
    for level in levels {
        let mut compressor = ResponseCompressor::new().with_level(level);
        let compressed = compressor.compress(&test_data, CompressionType::Gzip).unwrap();
        sizes.push(compressed.len());
    }
    
    // Higher levels should generally produce smaller or equal sizes
    assert!(sizes[2] <= sizes[1]); // Level 9 <= Level 6
    assert!(sizes[1] <= sizes[0]); // Level 6 <= Level 1
}

#[test]
fn test_compression_response_with_timing() {
    let mut compressor = ResponseCompressor::new();
    let content = vec![b'x'; 5000];
    let content_type = "text/plain";
    let accept_encoding = "gzip, deflate";
    
    let result = compressor.compress_response(&content, content_type, accept_encoding);
    
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);
    assert!(result.compression_time > std::time::Duration::ZERO);
    assert!(result.compression_ratio > 0.0 && result.compression_ratio < 1.0);
    assert!(result.savings_percentage() > 0.0);
    assert!(result.is_effective());
    assert!(result.throughput_mbps() > 0.0);
}

#[test]
fn test_streaming_compressor_large_data() {
    let mut compressor = StreamingCompressor::new(CompressionType::Gzip, 1024);
    let mut total_compressed = Vec::new();
    
    // Write data in chunks
    for i in 0..10 {
        let chunk = format!("This is chunk number {} with some repetitive data. ", i).repeat(50);
        let compressed_chunk = compressor.write(chunk.as_bytes()).unwrap();
        total_compressed.extend(compressed_chunk);
    }
    
    // Finish and get final data
    let final_chunk = compressor.finish().unwrap();
    total_compressed.extend(final_chunk);
    
    // Should have compressed data
    assert!(!total_compressed.is_empty());
    
    // Verify statistics
    let (input, output, ratio) = compressor.get_stats();
    assert!(input > 0);
    assert!(output > 0);
    assert!(ratio > 0.0 && ratio < 1.0);
}

#[test]
fn test_streaming_compressor_with_custom_config() {
    let config = CompressionConfig {
        level: 9,
        buffer_size: 512,
        ..Default::default()
    };
    
    let mut compressor = StreamingCompressor::with_config(CompressionType::Brotli, config);
    
    // Test with small writes that don't trigger immediate flush
    let small_data = b"hello";
    let result = compressor.write(small_data).unwrap();
    assert!(result.is_empty());
    
    // Add enough data to trigger flush
    let large_data = vec![b'x'; 600];
    let result = compressor.write(&large_data).unwrap();
    assert!(!result.is_empty());
}

#[test]
fn test_compression_middleware_advanced_features() {
    let config = CompressionConfig {
        level: 8,
        min_size_threshold: 100,
        enabled_types: vec![CompressionType::Brotli, CompressionType::Gzip],
        ..Default::default()
    };
    
    let mut middleware = CompressionMiddleware::with_config(config)
        .force_compression(true)
        .vary_header(true);
    
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());
    headers.insert("Accept-Encoding".to_string(), "gzip, br".to_string());
    
    // Test with small content (force compression enabled)
    let small_content = vec![b'a'; 50];
    let processed = middleware.process_response(&small_content, &mut headers);
    
    // Should be compressed due to force flag
    assert!(processed.len() < small_content.len() || 
            headers.get("Content-Encoding").map(|s| s != "identity").unwrap_or(false));
    
    // Check for performance headers
    assert!(headers.contains_key("X-Compression-Ratio"));
    assert!(headers.contains_key("X-Compression-Time"));
    assert!(headers.contains_key("Vary"));
}

#[test]
fn test_compression_middleware_disabled() {
    let mut middleware = CompressionMiddleware::new().enabled(false);
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());
    headers.insert("Accept-Encoding".to_string(), "gzip".to_string());
    
    let content = vec![b'a'; 2048];
    let processed = middleware.process_response(&content, &mut headers);
    
    // Should return original content
    assert_eq!(processed, content);
    assert!(!headers.contains_key("Content-Encoding"));
}

#[test]
fn test_compression_statistics() {
    let mut compressor = ResponseCompressor::new();
    let test_data = vec![b'x'; 1000];
    
    // Perform multiple operations
    for _ in 0..3 {
        let _ = compressor.compress(&test_data, CompressionType::Gzip);
        let _ = compressor.compress(&test_data, CompressionType::Brotli);
    }
    
    let stats = compressor.get_stats();
    
    assert_eq!(stats.total_compressions, 6);
    assert!(stats.total_bytes_compressed > 0);
    assert!(stats.total_compression_time > std::time::Duration::ZERO);
    assert_eq!(stats.compression_failures, 0);
    assert_eq!(stats.compression_success_rate(), 1.0);
    assert!(stats.average_compression_ratio() > 0.0);
    assert!(stats.average_compression_throughput() > 0.0);
    
    // Check algorithm-specific stats
    assert!(stats.algorithm_stats.contains_key("gzip"));
    assert!(stats.algorithm_stats.contains_key("br"));
    
    let gzip_stats = &stats.algorithm_stats["gzip"];
    assert_eq!(gzip_stats.compressions, 3);
    assert!(gzip_stats.input_bytes > 0);
    assert!(gzip_stats.output_bytes > 0);
}

#[test]
fn test_error_handling_scenarios() {
    let mut compressor = ResponseCompressor::new();
    
    // Test with corrupted GZIP data
    let corrupted_gzip = vec![0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00]; // Incomplete GZIP
    let result = compressor.decompress(&corrupted_gzip, CompressionType::Gzip);
    assert!(result.is_err());
    
    // Test with corrupted Deflate data
    let corrupted_deflate = vec![0xff, 0xff, 0xff];
    let result = compressor.decompress(&corrupted_deflate, CompressionType::Deflate);
    assert!(result.is_err());
    
    // Test with empty data (should work)
    let empty_data = vec![];
    let result = compressor.compress(&empty_data, CompressionType::Gzip);
    assert!(result.is_ok());
    
    // Verify error statistics
    let stats = compressor.get_stats();
    assert!(stats.decompression_failures > 0);
    assert!(stats.compression_success_rate() < 1.0 || stats.total_compressions == 1);
}

#[test]
fn test_compression_result_effectiveness() {
    let mut compressor = ResponseCompressor::new();
    let highly_compressible = vec![b'a'; 1000];
    let result = compressor.compress_response(&highly_compressible, "text/plain", "gzip");
    
    assert!(result.is_effective());
    assert!(result.is_effective_with_threshold(0.8));
    assert!(result.savings_percentage() > 10.0);
    
    // Test with less compressible data
    let random_data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    let result = compressor.compress_response(&random_data, "text/plain", "gzip");
    
    // Might not be as effective
    let savings = result.savings_percentage();
    assert!(savings >= 0.0); // Should never be negative
}

#[test]
fn test_content_type_matching() {
    let compressor = ResponseCompressor::new();
    
    // Test exact matches
    assert!(compressor.should_compress(&vec![b'x'; 2000], "text/html"));
    assert!(compressor.should_compress(&vec![b'x'; 2000], "application/json"));
    
    // Test with charset
    assert!(compressor.should_compress(&vec![b'x'; 2000], "text/html; charset=utf-8"));
    assert!(compressor.should_compress(&vec![b'x'; 2000], "application/json; charset=utf-8"));
    
    // Test case insensitive
    assert!(compressor.should_compress(&vec![b'x'; 2000], "TEXT/HTML"));
    assert!(compressor.should_compress(&vec![b'x'; 2000], "Application/JSON"));
    
    // Test non-compressible
    assert!(!compressor.should_compress(&vec![b'x'; 2000], "image/png"));
    assert!(!compressor.should_compress(&vec![b'x'; 2000], "video/mp4"));
}

#[test]
fn test_header_generation() {
    let result = CompressionResult {
        content: vec![1, 2, 3],
        compression_type: CompressionType::Brotli,
        original_size: 1000,
        compressed_size: 700,
        compression_ratio: 0.7,
        compression_time: std::time::Duration::from_millis(50),
    };
    
    let headers = result.get_headers();
    
    // Check required headers
    assert!(headers.iter().any(|(k, v)| k == "Content-Encoding" && v == "br"));
    assert!(headers.iter().any(|(k, v)| k == "Content-Length" && v == "700"));
    assert!(headers.iter().any(|(k, v)| k == "Vary" && v == "Accept-Encoding"));
    
    // Test identity compression (no encoding headers)
    let identity_result = CompressionResult {
        content: vec![1, 2, 3],
        compression_type: CompressionType::Identity,
        original_size: 1000,
        compressed_size: 1000,
        compression_ratio: 1.0,
        compression_time: std::time::Duration::ZERO,
    };
    
    let identity_headers = identity_result.get_headers();
    assert!(!identity_headers.iter().any(|(k, _)| k == "Content-Encoding"));
    assert!(identity_headers.iter().any(|(k, v)| k == "Content-Length" && v == "1000"));
}

#[test]
fn test_benchmark_functionality() {
    let test_data = vec![b'x'; 2000];
    let results = benchmark::benchmark_algorithms(&test_data, 3);
    
    // Should have results for at least some algorithms
    assert!(!results.algorithm_results.is_empty());
    
    // Check if results are reasonable
    for (algorithm, benchmark) in &results.algorithm_results {
        assert!(benchmark.compression_ratio > 0.0);
        assert!(benchmark.compression_ratio <= 1.0);
        assert!(benchmark.avg_compression_time > std::time::Duration::ZERO);
        assert!(benchmark.throughput_mbps > 0.0);
        assert!(benchmark.success_rate >= 0.0 && benchmark.success_rate <= 1.0);
        
        println!("Algorithm: {}, Ratio: {:.3}, Throughput: {:.2} MB/s, Success: {:.2}%",
                 algorithm, benchmark.compression_ratio, benchmark.throughput_mbps,
                 benchmark.success_rate * 100.0);
    }
    
    // Test best algorithm selection
    if let Some((algo, benchmark)) = results.best_compression_ratio() {
        println!("Best compression: {} with ratio {:.3}", algo, benchmark.compression_ratio);
    }
    
    if let Some((algo, benchmark)) = results.fastest_algorithm() {
        println!("Fastest algorithm: {} with {:.2} MB/s", algo, benchmark.throughput_mbps);
    }
}

#[test]
fn test_stress_compression() {
    let mut compressor = ResponseCompressor::new();
    
    // Test with various data patterns
    let patterns = vec![
        vec![b'a'; 10000], // Highly repetitive
        (0..10000u8).cycle().take(10000).collect::<Vec<u8>>(), // Pattern
        (0..256u8).cycle().take(10000).collect::<Vec<u8>>(), // Less repetitive
    ];
    
    for (i, pattern) in patterns.iter().enumerate() {
        for algorithm in &[CompressionType::Gzip, CompressionType::Brotli, CompressionType::Zstd] {
            let compressed = compressor.compress(pattern, algorithm.clone()).unwrap();
            let decompressed = compressor.decompress(&compressed, algorithm.clone()).unwrap();
            
            assert_eq!(decompressed, *pattern, 
                       "Pattern {} failed round-trip with {:?}", i, algorithm);
        }
    }
}

#[test]
fn test_concurrent_compression() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let compressor = Arc::new(Mutex::new(ResponseCompressor::new()));
    let test_data = Arc::new(vec![b'x'; 1000]);
    let mut handles = vec![];
    
    // Spawn multiple threads doing compression
    for i in 0..4 {
        let compressor = Arc::clone(&compressor);
        let test_data = Arc::clone(&test_data);
        
        let handle = thread::spawn(move || {
            let algorithm = match i % 4 {
                0 => CompressionType::Gzip,
                1 => CompressionType::Deflate,
                2 => CompressionType::Brotli,
                _ => CompressionType::Zstd,
            };
            
            let mut comp = compressor.lock().unwrap();
            let compressed = comp.compress(&test_data, algorithm.clone()).unwrap();
            let decompressed = comp.decompress(&compressed, algorithm).unwrap();
            
            assert_eq!(decompressed, *test_data);
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Check final statistics
    let stats = compressor.lock().unwrap().get_stats();
    assert_eq!(stats.total_compressions, 4);
    assert_eq!(stats.total_decompressions, 4);
}

#[test]
fn test_memory_efficiency() {
    let huge_data = vec![b'a'; 1_000_000]; // 1MB of data
    
    // Test streaming compression for memory efficiency
    let mut streaming = StreamingCompressor::new(CompressionType::Gzip, 8192);
    
    // Process in chunks
    for chunk in huge_data.chunks(10000) {
        let _ = streaming.write(chunk).unwrap();
    }
    
    let _ = streaming.finish().unwrap();
    let (input, output, ratio) = streaming.get_stats();
    
    assert_eq!(input, huge_data.len());
    assert!(output > 0);
    assert!(ratio < 1.0); // Should achieve compression
}

#[test]
fn test_configuration_edge_cases() {
    // Test with extreme configurations
    let extreme_config = CompressionConfig {
        level: 255, // Way too high
        min_size_threshold: 0,
        max_size_threshold: usize::MAX,
        compressible_types: vec![],
        buffer_size: 1,
        enabled_types: vec![],
        quality_threshold: 2.0, // Invalid quality
    };
    
    let compressor = ResponseCompressor::with_config(extreme_config);
    
    // Should handle gracefully
    let data = vec![b'x'; 100];
    assert!(!compressor.should_compress(&data, "text/html")); // No compressible types
    
    let compression = compressor.select_compression("gzip");
    assert_eq!(compression, CompressionType::Identity); // No enabled types
}
