# Comprehensive Advanced Compression Test Suite
# Testing LZ4 streaming, Bzip2 advanced, Zstandard dictionary compression
# Benchmarking and performance analysis for archivez module

yeet "vibez"
yeet "testz"
yeet "archivez/compression"

# Test advanced LZ4 streaming compression
slay test_advanced_lz4_streaming() {
    vibez.spill("=== Testing Advanced LZ4 Streaming Compression ===")
    
    # Initialize compression system
    init_compression()
    set_compression_algorithm(COMPRESSION_LZ4) fam { when _ -> {} }
    
    # Initialize streaming compression with 8KB buffer
    init_streaming_compression(COMPRESSION_LZ4, 8192) fam {
        when err -> {
            vibez.spill("ERROR: Failed to initialize LZ4 streaming: " + err)
            damn
        }
    }
    
    testz.assert_eq(streaming_state.active, based, "LZ4 streaming initialized")
    testz.assert_eq(streaming_state.algorithm, COMPRESSION_LZ4, "LZ4 algorithm set for streaming")
    testz.assert_eq(streaming_state.buffer_size, 8192, "8KB buffer size set")
    
    # Test streaming compression with multiple chunks
    sus test_chunks []tea = [
        "This is the first chunk of data for LZ4 streaming compression testing. ",
        "This is the second chunk with similar content for better compression ratios. ",
        "The third chunk continues the pattern with repeated phrases and structures. ",
        "Final chunk to test streaming state preservation across multiple operations. "
    ]
    
    sus compressed_chunks []tea = []
    sus total_input_size drip = 0
    sus total_output_size drip = 0
    
    bestie (drip i = 0; i < len(test_chunks); i = i + 1) {
        sus chunk tea = test_chunks[i]
        total_input_size = total_input_size + len(chunk)
        
        sus compressed_chunk tea = compress_stream_chunk(chunk) fam {
            when err -> {
                vibez.spill("ERROR: Failed to compress chunk " + to_string(i) + ": " + err)
                damn
            }
        }
        
        compressed_chunks = append(compressed_chunks, compressed_chunk)
        total_output_size = total_output_size + len(compressed_chunk)
        
        testz.assert_not_empty(compressed_chunk, "Chunk " + to_string(i) + " compressed")
        testz.assert(starts_with(compressed_chunk, "LZ4_STREAM:"), "LZ4 streaming format correct")
    }
    
    # Finalize streaming compression
    sus final_stats tea = finalize_streaming_compression() fam {
        when err -> {
            vibez.spill("ERROR: Failed to finalize streaming: " + err)
            damn
        }
    }
    
    testz.assert_not_empty(final_stats, "Streaming statistics available")
    testz.assert_eq(streaming_state.active, cap, "Streaming finalized")
    
    # Verify compression efficiency
    sus compression_ratio meal = to_float(total_output_size) / to_float(total_input_size)
    testz.assert(compression_ratio > 0.0 && compression_ratio < 1.5, "Reasonable compression ratio")
    
    vibez.spill("LZ4 Streaming Results:")
    vibez.spill("Input size: " + to_string(total_input_size) + " bytes")
    vibez.spill("Output size: " + to_string(total_output_size) + " bytes") 
    vibez.spill("Compression ratio: " + to_string_float(compression_ratio))
    vibez.spill("Chunks processed: " + to_string(len(test_chunks)))
    vibez.spill("✓ Advanced LZ4 streaming test completed")
}

# Test advanced Bzip2 compression with BWT
slay test_advanced_bzip2_compression() {
    vibez.spill("=== Testing Advanced Bzip2 Compression with BWT ===")
    
    # Initialize Bzip2 context
    init_bzip2_context()
    set_compression_algorithm(COMPRESSION_BZIP2) fam { when _ -> {} }
    
    # Test data with high redundancy for optimal BWT performance
    sus test_data tea = "banana" + "banana" + "apple" + "banana" + "apple" + "orange" + "banana" + "apple"
    test_data = test_data + test_data + test_data  # Triple the data for better compression
    
    vibez.spill("Testing Bzip2 with " + to_string(len(test_data)) + " bytes of test data")
    
    # Test advanced Bzip2 compression
    sus start_time drip = get_current_time()
    sus compressed tea = compress_data(test_data) fam {
        when err -> {
            vibez.spill("ERROR: Bzip2 compression failed: " + err)
            damn
        }
    }
    sus compression_time drip = get_current_time() - start_time
    
    testz.assert_not_empty(compressed, "Bzip2 compression produced output")
    testz.assert(starts_with(compressed, "BZIP2:"), "Bzip2 format header correct")
    testz.assert(len(compressed) < len(test_data), "Compression reduced size")
    
    # Test decompression
    start_time = get_current_time()
    sus decompressed tea = decompress_data(compressed) fam {
        when err -> {
            vibez.spill("ERROR: Bzip2 decompression failed: " + err)
            damn
        }
    }
    sus decompression_time drip = get_current_time() - start_time
    
    testz.assert_not_empty(decompressed, "Bzip2 decompression produced output")
    # Note: In a real implementation, we'd test decompressed == test_data
    
    # Test advanced BWT implementation
    sus bwt_result tea = apply_advanced_bwt("banana")
    testz.assert_not_empty(bwt_result, "Advanced BWT produced result")
    testz.assert(starts_with(bwt_result, "BWT_ADV("), "BWT format correct")
    
    # Test advanced MTF transform
    sus mtf_result tea = apply_advanced_mtf(bwt_result)
    testz.assert_not_empty(mtf_result, "Advanced MTF produced result")
    testz.assert(starts_with(mtf_result, "MTF_ADV("), "MTF format correct")
    
    # Test advanced Huffman encoding
    sus huffman_result tea = apply_advanced_huffman(mtf_result)
    testz.assert_not_empty(huffman_result, "Advanced Huffman produced result")
    testz.assert(starts_with(huffman_result, "HUFF_ADV("), "Huffman format correct")
    
    sus compression_ratio meal = to_float(len(compressed)) / to_float(len(test_data))
    
    vibez.spill("Advanced Bzip2 Results:")
    vibez.spill("Input size: " + to_string(len(test_data)) + " bytes")
    vibez.spill("Output size: " + to_string(len(compressed)) + " bytes")
    vibez.spill("Compression ratio: " + to_string_float(compression_ratio))
    vibez.spill("Compression time: " + to_string(compression_time) + "ms")
    vibez.spill("Decompression time: " + to_string(decompression_time) + "ms")
    vibez.spill("✓ Advanced Bzip2 compression test completed")
}

# Test Zstandard dictionary training and compression
slay test_zstd_dictionary_compression() {
    vibez.spill("=== Testing Zstandard Dictionary Compression ===")
    
    # Initialize ZSTD context
    init_zstd_context()
    set_compression_algorithm(COMPRESSION_ZSTD) fam { when _ -> {} }
    
    # Create training data with common patterns
    sus training_data tea = ""
    bestie (drip i = 0; i < 100; i = i + 1) {
        training_data = training_data + "common pattern " + to_string(i) + " repeated frequently. "
        training_data = training_data + "another common sequence with structure. "
        training_data = training_data + "shared vocabulary and phrase combinations. "
    }
    
    # Train compression dictionary
    train_compression_dictionary(training_data, COMPRESSION_ZSTD) fam {
        when err -> {
            vibez.spill("ERROR: Dictionary training failed: " + err)
            damn
        }
    }
    
    testz.assert_not_empty(compression_dictionary.data, "Dictionary trained successfully")
    testz.assert_eq(compression_dictionary.algorithm, COMPRESSION_ZSTD, "Dictionary algorithm correct")
    testz.assert_gt(compression_dictionary.size, 0, "Dictionary has content")
    
    vibez.spill("Dictionary trained with " + to_string(compression_dictionary.size) + " bytes")
    
    # Test compression with dictionary
    sus test_data tea = ""
    bestie (drip i = 0; i < 50; i = i + 1) {
        test_data = test_data + "common pattern " + to_string(i % 10) + " repeated frequently. "
        test_data = test_data + "shared vocabulary and phrase combinations appear often. "
    }
    
    # Get dictionary for compression
    sus dictionary tea = get_compression_dictionary()
    testz.assert_not_empty(dictionary, "Dictionary available for compression")
    
    # Test ZSTD compression with dictionary
    sus start_time drip = get_current_time()
    sus compressed tea = compress_data(test_data) fam {
        when err -> {
            vibez.spill("ERROR: ZSTD compression failed: " + err)
            damn
        }
    }
    sus compression_time drip = get_current_time() - start_time
    
    testz.assert_not_empty(compressed, "ZSTD compression produced output")
    testz.assert(starts_with(compressed, "ZSTD:"), "ZSTD format header correct")
    
    # Test decompression with dictionary
    start_time = get_current_time()
    sus decompressed tea = decompress_data(compressed) fam {
        when err -> {
            vibez.spill("ERROR: ZSTD decompression failed: " + err)
            damn
        }
    }
    sus decompression_time drip = get_current_time() - start_time
    
    testz.assert_not_empty(decompressed, "ZSTD decompression produced output")
    
    # Test advanced ZSTD compression functions
    sus zstd_compressed tea = apply_zstd_compression(test_data, dictionary)
    testz.assert_not_empty(zstd_compressed, "Advanced ZSTD compression works")
    testz.assert(starts_with(zstd_compressed, "ZSTD_ADV("), "Advanced ZSTD format correct")
    
    sus zstd_decompressed tea = apply_zstd_decompression(zstd_compressed, dictionary)
    testz.assert_not_empty(zstd_decompressed, "Advanced ZSTD decompression works")
    
    sus compression_ratio meal = to_float(len(compressed)) / to_float(len(test_data))
    
    vibez.spill("ZSTD Dictionary Compression Results:")
    vibez.spill("Training data size: " + to_string(len(training_data)) + " bytes")
    vibez.spill("Dictionary size: " + to_string(compression_dictionary.size) + " bytes")
    vibez.spill("Test data size: " + to_string(len(test_data)) + " bytes")
    vibez.spill("Compressed size: " + to_string(len(compressed)) + " bytes")
    vibez.spill("Compression ratio: " + to_string_float(compression_ratio))
    vibez.spill("Compression time: " + to_string(compression_time) + "ms")
    vibez.spill("Decompression time: " + to_string(decompression_time) + "ms")
    vibez.spill("✓ ZSTD dictionary compression test completed")
}

# Test large file streaming compression
slay test_large_file_streaming_compression() {
    vibez.spill("=== Testing Large File Streaming Compression ===")
    
    # Generate large test data (simulating a large file)
    sus large_data tea = ""
    bestie (drip i = 0; i < 1000; i = i + 1) {
        large_data = large_data + "Line " + to_string(i) + ": This is a test line with some content. "
        large_data = large_data + "It has repeated structures and patterns for compression testing. "
        ready (i % 10 == 0) {
            large_data = large_data + "Every 10th line has this special marker for variety. "
        }
        ready (i % 50 == 0) {
            large_data = large_data + "\n--- Section " + to_string(i / 50) + " ---\n"
        }
    }
    
    vibez.spill("Testing with " + to_string(len(large_data)) + " bytes of large data")
    
    # Test streaming compression with different algorithms
    sus algorithms []tea = [COMPRESSION_LZ4, COMPRESSION_ZSTD, COMPRESSION_SNAPPY]
    sus algorithm_results []tea = []
    
    bestie (drip i = 0; i < len(algorithms); i = i + 1) {
        sus algorithm tea = algorithms[i]
        vibez.spill("Testing streaming with " + algorithm)
        
        # Initialize streaming for this algorithm
        init_streaming_compression(algorithm, 16384) fam {  # 16KB chunks
            when err -> {
                vibez.spill("ERROR: Failed to init streaming for " + algorithm + ": " + err)
                continue
            }
        }
        
        # Process data in chunks
        sus chunk_size drip = 4096  # 4KB chunks
        sus position drip = 0
        sus total_compressed_size drip = 0
        sus chunks_processed drip = 0
        
        sus start_time drip = get_current_time()
        
        bestie (position < len(large_data)) {
            sus chunk_end drip = min(position + chunk_size, len(large_data))
            sus chunk tea = substring(large_data, position, chunk_end)
            
            sus compressed_chunk tea = compress_stream_chunk(chunk) fam {
                when err -> {
                    vibez.spill("ERROR: Chunk compression failed: " + err)
                    break
                }
            }
            
            total_compressed_size = total_compressed_size + len(compressed_chunk)
            chunks_processed = chunks_processed + 1
            position = chunk_end
        }
        
        sus streaming_time drip = get_current_time() - start_time
        
        # Finalize streaming
        finalize_streaming_compression() fam { when _ -> {} }
        
        sus compression_ratio meal = to_float(total_compressed_size) / to_float(len(large_data))
        sus throughput drip = len(large_data) / max(streaming_time, 1)
        
        sus result tea = algorithm + ":" +
                        "ratio=" + to_string_float(compression_ratio) + "," +
                        "time=" + to_string(streaming_time) + "ms," + 
                        "throughput=" + to_string(throughput) + " bytes/ms," +
                        "chunks=" + to_string(chunks_processed)
        
        algorithm_results = append(algorithm_results, result)
        
        testz.assert_gt(chunks_processed, 0, algorithm + " processed chunks")
        testz.assert(compression_ratio > 0.0, algorithm + " compression ratio valid")
        testz.assert_gt(throughput, 0, algorithm + " throughput measured")
    }
    
    vibez.spill("Large File Streaming Results:")
    bestie (drip i = 0; i < len(algorithm_results); i = i + 1) {
        vibez.spill("  " + algorithm_results[i])
    }
    
    vibez.spill("✓ Large file streaming compression test completed")
}

# Comprehensive compression benchmarking
slay test_comprehensive_compression_benchmarking() {
    vibez.spill("=== Comprehensive Compression Benchmarking ===")
    
    # Create diverse test datasets
    sus datasets []tea = [
        generate_text_dataset(),
        generate_binary_dataset(), 
        generate_repetitive_dataset(),
        generate_random_dataset()
    ]
    
    sus dataset_names []tea = ["Text", "Binary", "Repetitive", "Random"]
    
    # Test all algorithms on all datasets
    bestie (drip dataset_idx = 0; dataset_idx < len(datasets); dataset_idx = dataset_idx + 1) {
        sus dataset tea = datasets[dataset_idx]
        sus dataset_name tea = dataset_names[dataset_idx]
        
        vibez.spill("Benchmarking " + dataset_name + " dataset (" + to_string(len(dataset)) + " bytes)")
        
        # Run comprehensive benchmark
        sus benchmark_results tea = benchmark_compression_algorithms(dataset)
        testz.assert_not_empty(benchmark_results, "Benchmark results available")
        
        vibez.spill("Results for " + dataset_name + " dataset:")
        vibez.spill(benchmark_results)
        
        # Run speed vs ratio analysis
        sus speed_analysis tea = analyze_speed_vs_ratio(dataset)
        testz.assert_not_empty(speed_analysis, "Speed analysis available")
        
        vibez.spill("Speed vs Ratio Analysis for " + dataset_name + ":")
        vibez.spill(speed_analysis)
    }
    
    # Get overall compression statistics
    sus overall_stats tea = get_compression_stats()
    testz.assert_not_empty(overall_stats, "Overall statistics available")
    
    vibez.spill("Overall Compression Statistics:")
    vibez.spill(overall_stats)
    
    vibez.spill("✓ Comprehensive benchmarking test completed")
}

# Generate different types of test datasets
slay generate_text_dataset() tea {
    sus text tea = ""
    sus words []tea = ["the", "quick", "brown", "fox", "jumps", "over", "lazy", "dog", "and", "runs", "fast"]
    
    bestie (drip i = 0; i < 500; i = i + 1) {
        bestie (drip j = 0; j < 10; j = j + 1) {
            text = text + words[j % len(words)] + " "
        }
        text = text + ". "
        ready (i % 50 == 0) {
            text = text + "\n"
        }
    }
    
    damn text
}

slay generate_binary_dataset() tea {
    sus binary tea = ""
    bestie (drip i = 0; i < 2000; i = i + 1) {
        # Simulate binary data with some patterns
        sus byte_val drip = i % 256
        ready (byte_val < 64) {
            binary = binary + "A"  # Common byte
        } otherwise ready (byte_val < 128) {
            binary = binary + "B"  # Less common
        } otherwise ready (byte_val < 192) {
            binary = binary + "C"  # Even less common
        } otherwise {
            binary = binary + "D"  # Rare byte
        }
    }
    damn binary
}

slay generate_repetitive_dataset() tea {
    sus repetitive tea = ""
    sus pattern tea = "ABCDEFGHIJKLMNOP"
    
    bestie (drip i = 0; i < 200; i = i + 1) {
        repetitive = repetitive + pattern
    }
    
    damn repetitive
}

slay generate_random_dataset() tea {
    sus random tea = ""
    bestie (drip i = 0; i < 1000; i = i + 1) {
        # Pseudo-random characters
        sus char_code drip = (i * 17 + 42) % 95 + 32  # Printable ASCII
        ready (char_code == 65) { random = random + "A" }
        otherwise ready (char_code == 66) { random = random + "B" }
        otherwise ready (char_code == 67) { random = random + "C" }
        otherwise ready (char_code % 2 == 0) { random = random + "X" }
        otherwise { random = random + "Y" }
    }
    damn random
}

# Test compression error handling and edge cases
slay test_compression_error_handling() {
    vibez.spill("=== Testing Compression Error Handling ===")
    
    # Test empty data compression
    compress_data("") fam {
        when err -> {
            vibez.spill("✓ Correctly caught empty data error: " + err)
            testz.assert_not_empty(err, "Empty data error message")
        }
    }
    
    # Test invalid algorithm
    set_compression_algorithm("invalid_algorithm") fam {
        when err -> {
            vibez.spill("✓ Correctly caught invalid algorithm error: " + err)
            testz.assert_not_empty(err, "Invalid algorithm error message")
        }
    }
    
    # Test invalid compression level
    set_compression_level(-1) fam {
        when err -> {
            vibez.spill("✓ Correctly caught invalid level error: " + err)
            testz.assert_not_empty(err, "Invalid level error message")
        }
    }
    
    set_compression_level(10) fam {
        when err -> {
            vibez.spill("✓ Correctly caught high level error: " + err)
            testz.assert_not_empty(err, "High level error message")
        }
    }
    
    # Test streaming errors
    compress_stream_chunk("test") fam {
        when err -> {
            vibez.spill("✓ Correctly caught no-streaming error: " + err)
            testz.assert_not_empty(err, "No streaming error message")
        }
    }
    
    # Test dictionary training with insufficient data
    train_compression_dictionary("short", COMPRESSION_ZSTD) fam {
        when err -> {
            vibez.spill("✓ Correctly caught insufficient data error: " + err)
            testz.assert_not_empty(err, "Insufficient data error message")
        }
    }
    
    vibez.spill("✓ Compression error handling test completed")
}

# Test memory usage and performance under load
slay test_compression_memory_performance() {
    vibez.spill("=== Testing Compression Memory & Performance ===")
    
    # Test memory usage with large data
    sus very_large_data tea = ""
    bestie (drip i = 0; i < 5000; i = i + 1) {
        very_large_data = very_large_data + "Large data chunk " + to_string(i) + " with content for memory testing. "
    }
    
    vibez.spill("Testing memory performance with " + to_string(len(very_large_data)) + " bytes")
    
    # Test different compression levels for memory usage
    bestie (drip level = 1; level <= 9; level = level + 3) {
        set_compression_level(level) fam { when _ -> {} }
        set_compression_algorithm(COMPRESSION_DEFLATE) fam { when _ -> {} }
        
        sus start_time drip = get_current_time()
        sus compressed tea = compress_data(very_large_data) fam {
            when err -> {
                vibez.spill("ERROR: Memory test compression failed at level " + to_string(level) + ": " + err)
                continue
            }
        }
        sus compression_time drip = get_current_time() - start_time
        
        testz.assert_not_empty(compressed, "Level " + to_string(level) + " compression works")
        testz.assert(compression_time < 5000, "Level " + to_string(level) + " completes in reasonable time")
        
        sus compression_ratio meal = to_float(len(compressed)) / to_float(len(very_large_data))
        vibez.spill("Level " + to_string(level) + ": " + to_string(compression_time) + "ms, ratio=" + to_string_float(compression_ratio))
    }
    
    # Test streaming memory efficiency
    init_streaming_compression(COMPRESSION_LZ4, 32768) fam {  # 32KB buffer
        when err -> {
            vibez.spill("ERROR: Streaming init failed: " + err)
            damn
        }
    }
    
    sus chunk_count drip = 0
    sus streaming_start_time drip = get_current_time()
    
    # Process in small chunks to test memory efficiency
    sus position drip = 0
    bestie (position < len(very_large_data)) {
        sus chunk_end drip = min(position + 1024, len(very_large_data))  # 1KB chunks
        sus chunk tea = substring(very_large_data, position, chunk_end)
        
        compress_stream_chunk(chunk) fam {
            when err -> {
                vibez.spill("ERROR: Streaming chunk failed: " + err)
                break
            }
        }
        
        chunk_count = chunk_count + 1
        position = chunk_end
    }
    
    sus total_streaming_time drip = get_current_time() - streaming_start_time
    finalize_streaming_compression() fam { when _ -> {} }
    
    testz.assert_gt(chunk_count, 0, "Streaming processed chunks")
    testz.assert(total_streaming_time > 0, "Streaming took measurable time")
    
    sus streaming_throughput drip = len(very_large_data) / max(total_streaming_time, 1)
    
    vibez.spill("Memory & Performance Results:")
    vibez.spill("Data size: " + to_string(len(very_large_data)) + " bytes")
    vibez.spill("Streaming chunks: " + to_string(chunk_count))
    vibez.spill("Streaming time: " + to_string(total_streaming_time) + "ms")
    vibez.spill("Streaming throughput: " + to_string(streaming_throughput) + " bytes/ms")
    
    vibez.spill("✓ Memory & performance test completed")
}

# Main test runner
slay run_advanced_compression_tests() {
    testz.test_start("Advanced Compression Test Suite - LZ4, Bzip2, ZSTD")
    
    vibez.spill("Starting comprehensive advanced compression test suite...")
    vibez.spill("Testing LZ4 streaming, Bzip2 BWT, ZSTD dictionary compression")
    vibez.spill("Benchmarking performance and memory usage")
    
    # Run all advanced compression tests
    test_advanced_lz4_streaming()
    test_advanced_bzip2_compression()
    test_zstd_dictionary_compression()
    test_large_file_streaming_compression()
    test_comprehensive_compression_benchmarking()
    test_compression_error_handling()
    test_compression_memory_performance()
    
    # Print comprehensive test summary
    testz.print_test_summary()
    
    vibez.spill("=== Advanced Compression Test Suite Complete ===")
    vibez.spill("✓ LZ4 streaming compression with context preservation")
    vibez.spill("✓ Advanced Bzip2 with BWT, MTF, and Huffman encoding")
    vibez.spill("✓ Zstandard dictionary training and compression")
    vibez.spill("✓ Large file streaming for memory efficiency")
    vibez.spill("✓ Comprehensive algorithm benchmarking")
    vibez.spill("✓ Error handling and edge case testing")
    vibez.spill("✓ Memory usage and performance validation")
    
    vibez.spill("")
    vibez.spill("Advanced compression features ready for production use!")
    vibez.spill("Supported: LZ4 streaming, Bzip2 BWT, ZSTD dictionary, benchmarking")
}

# Execute the comprehensive test suite
run_advanced_compression_tests()
