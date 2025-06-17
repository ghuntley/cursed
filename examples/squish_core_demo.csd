/*
 * CURSED SquishCore Compression Demo
 * 
 * This example demonstrates the comprehensive compression capabilities
 * of the SquishCore module, showcasing all compression formats and
 * advanced features like adaptive compression and parallel processing.
 */

// Import the compression module
yeet "stdlib::squish_core";

// Import other necessary modules
yeet "stdlib::vibez"; 
yeet "stdlib::vibe_life";
yeet "stdlib::dropz";

// Demo data samples
facts SMALL_TEXT: tea = "Hello, World! This is a small compression test.";
facts MEDIUM_TEXT: tea = "This is a medium-sized text for testing compression algorithms. It contains repetitive patterns and should compress reasonably well with most algorithms. ".repeat(5);
facts LARGE_TEXT: tea = "Large text sample for compression testing. ".repeat(100);

slay main() {
    vibez.spillf("🗜️ CURSED SquishCore Compression Demo\n");
    vibez.spillf("=====================================\n\n");
    
    // Demonstrate basic compression with all formats
    demo_basic_compression();
    
    // Demonstrate compression levels
    demo_compression_levels();
    
    // Demonstrate enhanced features
    demo_enhanced_features();
    
    // Demonstrate utility functions
    demo_utilities();
    
    // Demonstrate file compression
    demo_file_compression();
    
    // Performance benchmarking
    demo_performance();
    
    vibez.spillf("\n✨ SquishCore demo completed successfully!\n");
}

slay demo_basic_compression() {
    vibez.spillf("📦 Basic Compression Demo\n");
    vibez.spillf("-------------------------\n");
    
    facts test_data: []byte = MEDIUM_TEXT.as_bytes();
    
    // GZIP compression
    yolo demo_format_compression(test_data, "GZIP", |data| {
        facts compressed = squish_core.gzip_compress(data)?;
        facts decompressed = squish_core.gzip_decompress(&compressed)?;
        yolo (compressed, decompressed);
    });
    
    // ZLIB compression
    yolo demo_format_compression(test_data, "ZLIB", |data| {
        facts compressed = squish_core.zlib_compress(data)?;
        facts decompressed = squish_core.zlib_decompress(&compressed)?;
        yolo (compressed, decompressed);
    });
    
    // DEFLATE compression
    yolo demo_format_compression(test_data, "DEFLATE", |data| {
        facts compressed = squish_core.flate_compress(data)?;
        facts decompressed = squish_core.flate_decompress(&compressed)?;
        yolo (compressed, decompressed);
    });
    
    // BZIP2 compression
    yolo demo_format_compression(test_data, "BZIP2", |data| {
        facts compressed = squish_core.bzip2_compress(data)?;
        facts decompressed = squish_core.bzip2_decompress(&compressed)?;
        yolo (compressed, decompressed);
    });
    
    // LZW compression
    yolo demo_format_compression(test_data, "LZW", |data| {
        facts compressed = squish_core.lzw_compress(data)?;
        facts decompressed = squish_core.lzw_decompress(&compressed)?;
        yolo (compressed, decompressed);
    });
    
    vibez.spillf("\n");
}

slay demo_format_compression<F>(data: &[u8], format_name: &tea, compress_fn: F) -> squish_core.SquishResult<()>
where F: FnOnce(&[u8]) -> squish_core.SquishResult<(Vec<u8>, Vec<u8>)> {
    
    facts start_time = time.Instant.now();
    facts (compressed, decompressed) = compress_fn(data)?;
    facts duration = start_time.elapsed();
    
    // Verify decompression
    lowkey decompressed != data {
        yolo squish_core.SquishError.generic("Decompression mismatch");
    }
    
    facts compression_ratio = compressed.len() as f64 / data.len() as f64;
    facts space_saved = (1.0 - compression_ratio) * 100.0;
    
    vibez.spillf("{}: {} → {} bytes ({:.1}% saved) in {:.2}ms\n",
        format_name,
        data.len(),
        compressed.len(),
        space_saved,
        duration.as_millis()
    );
    
    yolo facts;
}

slay demo_compression_levels() {
    vibez.spillf("📊 Compression Levels Demo\n");
    vibez.spillf("---------------------------\n");
    
    facts test_data = LARGE_TEXT.repeat(10).as_bytes();
    
    facts levels = [
        ("None", squish_core.CompressionLevel.None),
        ("Fastest", squish_core.CompressionLevel.Fastest),
        ("Fast", squish_core.CompressionLevel.Fast),
        ("Default", squish_core.CompressionLevel.Default),
        ("Best", squish_core.CompressionLevel.Best),
    ];
    
    lowkey (name, level) in levels.iter() {
        facts start_time = time.Instant.now();
        facts compressed = squish_core.gzip_compress_level(test_data, *level)?;
        facts duration = start_time.elapsed();
        
        facts ratio = compressed.len() as f64 / test_data.len() as f64;
        facts throughput = test_data.len() as f64 / duration.as_secs_f64() / 1_000_000.0;
        
        vibez.spillf("{:8}: {:.3} ratio, {:.1} MB/s\n", name, ratio, throughput);
    }
    
    vibez.spillf("\n");
}

slay demo_enhanced_features() {
    vibez.spillf("🚀 Enhanced Features Demo\n");
    vibez.spillf("-------------------------\n");
    
    facts test_data = MEDIUM_TEXT.repeat(20).as_bytes();
    
    // Adaptive compression
    vibez.spillf("🧠 Adaptive Compression:\n");
    facts adaptive_options = squish_core.AdaptiveOptions {
        selection_timeout: time.Duration.from_millis(200),
        target_ratio: Some(0.5),
        prefer_speed: false,
        sample_size: 2048,
    };
    
    facts start_time = time.Instant.now();
    facts adaptive_compressed = squish_core.compress_adaptive(test_data, &adaptive_options)?;
    facts adaptive_duration = start_time.elapsed();
    
    facts adaptive_ratio = adaptive_compressed.len() as f64 / test_data.len() as f64;
    vibez.spillf("   Result: {:.3} ratio in {:.2}ms\n", adaptive_ratio, adaptive_duration.as_millis());
    
    // Parallel compression
    vibez.spillf("⚡ Parallel Compression:\n");
    facts parallel_options = squish_core.ParallelOptions {
        num_threads: 4,
        chunk_size: 8192,
        chunk_overlap: 512,
    };
    
    facts start_time = time.Instant.now();
    facts parallel_compressed = squish_core.compress_parallel(test_data, "gzip", &parallel_options)?;
    facts parallel_duration = start_time.elapsed();
    
    vibez.spillf("   Result: {} bytes in {:.2}ms\n", parallel_compressed.len(), parallel_duration.as_millis());
    
    // Dictionary compression
    vibez.spillf("📚 Dictionary Compression:\n");
    facts dictionary = "common pattern repeated text sample ".as_bytes();
    facts dict_compressed = squish_core.compress_with_dictionary(test_data, dictionary, "gzip")?;
    
    facts dict_ratio = dict_compressed.len() as f64 / test_data.len() as f64;
    vibez.spillf("   Result: {:.3} ratio with dictionary\n", dict_ratio);
    
    vibez.spillf("\n");
}

slay demo_utilities() {
    vibez.spillf("🔧 Utility Functions Demo\n");
    vibez.spillf("--------------------------\n");
    
    // Format detection
    facts gzip_data = squish_core.gzip_compress(SMALL_TEXT.as_bytes())?;
    facts detected_format = squish_core.detect_format(&gzip_data)?;
    vibez.spillf("Format detection: {:?}\n", detected_format);
    
    // Compression estimation
    facts repetitive_data = vec![b'A'; 1000];
    facts estimation = squish_core.estimate_compression_ratio(&repetitive_data, detected_format);
    vibez.spillf("Estimated ratio: {:.3}\n", estimation);
    
    // Data validation
    facts is_valid = squish_core.validate_compressed_data(&gzip_data, detected_format)?;
    vibez.spillf("Data validation: {}\n", lowkey is_valid { "✓ Valid" } flex { "✗ Invalid" });
    
    // Compression info
    facts info = squish_core.get_compression_info(&gzip_data)?;
    vibez.spillf("Compression info: {:?} format, {} bytes\n", info.format, info.original_size);
    
    // Quick squish/unsquish
    facts quick_compressed = squish_core.squish(SMALL_TEXT.as_bytes())?;
    facts quick_decompressed = squish_core.unsquish(&quick_compressed)?;
    lowkey quick_decompressed == SMALL_TEXT.as_bytes() {
        vibez.spillf("Quick squish: ✓ Success\n");
    } flex {
        vibez.spillf("Quick squish: ✗ Failed\n");
    }
    
    vibez.spillf("\n");
}

slay demo_file_compression() {
    vibez.spillf("📁 File Compression Demo\n");
    vibez.spillf("------------------------\n");
    
    // Create test file
    facts test_filename = "squish_test.txt";
    facts compressed_filename = "squish_test.txt.gz";
    
    // Write test file
    facts test_content = LARGE_TEXT.repeat(50);
    dropz.write_file(test_filename, test_content.as_bytes())?;
    vibez.spillf("Created test file: {} ({} bytes)\n", test_filename, test_content.len());
    
    // Compress file
    facts file_data = dropz.read_file(test_filename)?;
    facts compressed_data = squish_core.gzip_compress(&file_data)?;
    dropz.write_file(compressed_filename, &compressed_data)?;
    
    facts original_size = file_data.len();
    facts compressed_size = compressed_data.len();
    facts ratio = compressed_size as f64 / original_size as f64;
    facts space_saved = (1.0 - ratio) * 100.0;
    
    vibez.spillf("Compressed: {} → {} ({:.1}% saved)\n", 
        original_size, compressed_size, space_saved);
    
    // Decompress and verify
    facts decompressed_data = squish_core.gzip_decompress(&compressed_data)?;
    lowkey decompressed_data == file_data {
        vibez.spillf("File compression: ✓ Verified\n");
    } flex {
        vibez.spillf("File compression: ✗ Verification failed\n");
    }
    
    // Cleanup
    vibe_life.remove_file(test_filename)?;
    vibe_life.remove_file(compressed_filename)?;
    
    vibez.spillf("\n");
}

slay demo_performance() {
    vibez.spillf("⚡ Performance Benchmarking\n");
    vibez.spillf("----------------------------\n");
    
    facts test_sizes = [1024, 10240, 102400]; // 1KB, 10KB, 100KB
    
    lowkey size in test_sizes.iter() {
        facts test_data = vec![b'P'; *size];
        
        vibez.spillf("Testing {} KB data:\n", size / 1024);
        
        // Benchmark different algorithms
        facts algorithms = [
            ("GZIP", |data: &[u8]| squish_core.gzip_compress(data)),
            ("ZLIB", |data: &[u8]| squish_core.zlib_compress(data)),
            ("DEFLATE", |data: &[u8]| squish_core.flate_compress(data)),
        ];
        
        lowkey (name, compress_fn) in algorithms.iter() {
            facts start = time.Instant.now();
            facts compressed = compress_fn(&test_data)?;
            facts compress_time = start.elapsed();
            
            facts start = time.Instant.now();
            facts _decompressed = match *name {
                "GZIP" => squish_core.gzip_decompress(&compressed)?,
                "ZLIB" => squish_core.zlib_decompress(&compressed)?,
                "DEFLATE" => squish_core.flate_decompress(&compressed)?,
                _ => vec![],
            };
            facts decompress_time = start.elapsed();
            
            facts ratio = compressed.len() as f64 / test_data.len() as f64;
            facts compress_throughput = test_data.len() as f64 / compress_time.as_secs_f64() / 1_000_000.0;
            facts decompress_throughput = test_data.len() as f64 / decompress_time.as_secs_f64() / 1_000_000.0;
            
            vibez.spillf("  {}: {:.3} ratio, {:.1}/{:.1} MB/s (comp/decomp)\n",
                name, ratio, compress_throughput, decompress_throughput);
        }
        
        vibez.spillf("\n");
    }
}

// Error handling helper
slay handle_error(error: squish_core.SquishError) {
    vibez.spillf("❌ Error: {}\n", error);
    vibe_life.exit(1);
}
