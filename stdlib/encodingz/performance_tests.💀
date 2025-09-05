fr fr ===== ENCODINGZ PERFORMANCE TESTS =====
fr fr Comprehensive performance benchmarking for all encoding operations
fr fr Includes throughput testing, memory usage analysis, and scalability tests

yeet "encodingz" 
yeet "vibez"
yeet "timez"
yeet "stringz"

fr fr ===== PERFORMANCE TEST CONFIGURATION =====

sus SMALL_DATA_SIZE drip = 1024          fr fr 1KB
sus MEDIUM_DATA_SIZE drip = 65536        fr fr 64KB  
sus LARGE_DATA_SIZE drip = 1048576       fr fr 1MB
sus XLARGE_DATA_SIZE drip = 10485760     fr fr 10MB

sus BENCHMARK_ITERATIONS drip = 1000
sus STREAMING_CHUNK_SIZE drip = 8192

fr fr ===== PERFORMANCE DATA STRUCTURES =====

squad PerformanceResult {
    sus operation tea
    sus data_size drip
    sus iterations drip
    sus total_time_ms drip
    sus throughput_mbps drip
    sus ops_per_second drip
    sus memory_usage_kb drip
}

squad StreamingBenchmark {
    sus encoding_type tea
    sus total_bytes drip
    sus chunk_count drip
    sus total_time_ms drip
    sus peak_memory_kb drip
    sus avg_chunk_time_ms drip
}

fr fr ===== TEST DATA GENERATION =====

slay generate_test_data(size drip, pattern tea) tea {
    fr fr Generate test data with specified size and pattern
    sus result tea = ""
    sus pattern_len drip = string_length(pattern)
    sus bytes_generated drip = 0
    
    bestie bytes_generated < size {
        sus remaining drip = size - bytes_generated
        ready remaining >= pattern_len {
            result = result + pattern
            bytes_generated = bytes_generated + pattern_len
        } otherwise {
            result = result + substring(pattern, 0, remaining)
            bytes_generated = size
        }
    }
    
    damn result
}

slay generate_random_data(size drip) tea {
    fr fr Generate pseudo-random data for testing
    sus charset tea = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?"
    sus charset_len drip = string_length(charset)
    sus result tea = ""
    sus i drip = 0
    
    bestie i < size {
        fr fr Simple pseudo-random selection (replace with better RNG)
        sus index drip = (i * 17 + 31) % charset_len
        result = result + char_to_string(char_at(charset, index))
        i = i + 1
    }
    
    damn result
}

slay generate_binary_data(size drip) tea {
    fr fr Generate binary-like data (all byte values)
    sus result tea = ""
    sus i drip = 0
    
    bestie i < size {
        sus byte_val drip = i % 256
        fr fr Simulate binary data with various byte values
        ready byte_val < 32 {
            result = result + "."  fr fr Control characters as dots
        } otherwise ready byte_val < 127 {
            result = result + char_from_code(byte_val)
        } otherwise {
            result = result + "?"  fr fr High byte values
        }
        i = i + 1
    }
    
    damn result
}

fr fr ===== BASE64 PERFORMANCE TESTS =====

slay benchmark_base64_encoding(data_size drip) PerformanceResult {
    vibez.spill("📊 Benchmarking Base64 encoding for " + int_to_string(data_size) + " bytes...")
    
    sus test_data tea = generate_test_data(data_size, "The quick brown fox jumps over the lazy dog. ")
    sus start_time drip = get_current_time_ms()
    
    sus i drip = 0
    bestie i < BENCHMARK_ITERATIONS {
        sus encoded tea = base64_encode(test_data)
        i = i + 1
    }
    
    sus end_time drip = get_current_time_ms()
    sus total_time drip = end_time - start_time
    sus total_bytes drip = data_size * BENCHMARK_ITERATIONS
    sus throughput drip = (total_bytes * 1000) / (total_time * 1024 * 1024)  fr fr MB/s
    sus ops_per_sec drip = (BENCHMARK_ITERATIONS * 1000) / total_time
    
    damn PerformanceResult{
        operation: "base64_encode",
        data_size: data_size,
        iterations: BENCHMARK_ITERATIONS,
        total_time_ms: total_time,
        throughput_mbps: throughput,
        ops_per_second: ops_per_sec,
        memory_usage_kb: estimate_memory_usage(data_size, 133)  fr fr Base64 is ~33% larger
    }
}

slay benchmark_base64_decoding(data_size drip) PerformanceResult {
    vibez.spill("📊 Benchmarking Base64 decoding for " + int_to_string(data_size) + " bytes...")
    
    sus test_data tea = generate_test_data(data_size, "Hello World! This is a test. ")
    sus encoded_data tea = base64_encode(test_data)
    sus start_time drip = get_current_time_ms()
    
    sus i drip = 0
    bestie i < BENCHMARK_ITERATIONS {
        sus decoded tea = base64_decode(encoded_data) fam {
            when _ -> ""
        }
        i = i + 1
    }
    
    sus end_time drip = get_current_time_ms()
    sus total_time drip = end_time - start_time
    sus total_bytes drip = string_length(encoded_data) * BENCHMARK_ITERATIONS
    sus throughput drip = (total_bytes * 1000) / (total_time * 1024 * 1024)
    sus ops_per_sec drip = (BENCHMARK_ITERATIONS * 1000) / total_time
    
    damn PerformanceResult{
        operation: "base64_decode",
        data_size: string_length(encoded_data),
        iterations: BENCHMARK_ITERATIONS,
        total_time_ms: total_time,
        throughput_mbps: throughput,
        ops_per_second: ops_per_sec,
        memory_usage_kb: estimate_memory_usage(string_length(encoded_data), 75)  fr fr ~25% reduction
    }
}

fr fr ===== HEX PERFORMANCE TESTS =====

slay benchmark_hex_encoding(data_size drip) PerformanceResult {
    vibez.spill("📊 Benchmarking Hex encoding for " + int_to_string(data_size) + " bytes...")
    
    sus test_data tea = generate_binary_data(data_size)
    sus start_time drip = get_current_time_ms()
    
    sus i drip = 0
    bestie i < BENCHMARK_ITERATIONS {
        sus encoded tea = hex_encode(test_data)
        i = i + 1
    }
    
    sus end_time drip = get_current_time_ms()
    sus total_time drip = end_time - start_time
    sus total_bytes drip = data_size * BENCHMARK_ITERATIONS
    sus throughput drip = (total_bytes * 1000) / (total_time * 1024 * 1024)
    sus ops_per_sec drip = (BENCHMARK_ITERATIONS * 1000) / total_time
    
    damn PerformanceResult{
        operation: "hex_encode",
        data_size: data_size,
        iterations: BENCHMARK_ITERATIONS,
        total_time_ms: total_time,
        throughput_mbps: throughput,
        ops_per_second: ops_per_sec,
        memory_usage_kb: estimate_memory_usage(data_size, 200)  fr fr Hex doubles size
    }
}

slay benchmark_hex_decoding(data_size drip) PerformanceResult {
    vibez.spill("📊 Benchmarking Hex decoding for " + int_to_string(data_size) + " bytes...")
    
    sus test_data tea = generate_binary_data(data_size)
    sus encoded_data tea = hex_encode(test_data)
    sus start_time drip = get_current_time_ms()
    
    sus i drip = 0
    bestie i < BENCHMARK_ITERATIONS {
        sus decoded tea = hex_decode(encoded_data) fam {
            when _ -> ""
        }
        i = i + 1
    }
    
    sus end_time drip = get_current_time_ms()
    sus total_time drip = end_time - start_time
    sus total_bytes drip = string_length(encoded_data) * BENCHMARK_ITERATIONS
    sus throughput drip = (total_bytes * 1000) / (total_time * 1024 * 1024)
    sus ops_per_sec drip = (BENCHMARK_ITERATIONS * 1000) / total_time
    
    damn PerformanceResult{
        operation: "hex_decode",
        data_size: string_length(encoded_data),
        iterations: BENCHMARK_ITERATIONS,
        total_time_ms: total_time,
        throughput_mbps: throughput,
        ops_per_second: ops_per_sec,
        memory_usage_kb: estimate_memory_usage(string_length(encoded_data), 50)
    }
}

fr fr ===== STREAMING PERFORMANCE TESTS =====

slay benchmark_streaming_base64(total_size drip) StreamingBenchmark {
    vibez.spill("📊 Benchmarking streaming Base64 for " + int_to_string(total_size) + " bytes...")
    
    sus encoder StreamEncoder = create_stream_encoder("base64")
    sus test_data tea = generate_test_data(STREAMING_CHUNK_SIZE, "Streaming test data chunk. ")
    sus chunks_needed drip = total_size / STREAMING_CHUNK_SIZE
    sus start_time drip = get_current_time_ms()
    sus peak_memory drip = 0
    
    sus chunk_times drip[value] = []
    sus i drip = 0
    
    bestie i < chunks_needed {
        sus chunk_start drip = get_current_time_ms()
        sus result tea = stream_encode_chunk(encoder, test_data)
        sus chunk_end drip = get_current_time_ms()
        
        chunk_times = append_drip_to_array(chunk_times, chunk_end - chunk_start)
        
        fr fr Simulate memory usage tracking
        sus current_memory drip = estimate_streaming_memory(encoder)
        ready current_memory > peak_memory {
            peak_memory = current_memory
        }
        
        i = i + 1
    }
    
    sus final_result tea = stream_finalize(encoder)
    sus end_time drip = get_current_time_ms()
    sus total_time drip = end_time - start_time
    sus avg_chunk_time drip = total_time / chunks_needed
    
    damn StreamingBenchmark{
        encoding_type: "base64_streaming",
        total_bytes: total_size,
        chunk_count: chunks_needed,
        total_time_ms: total_time,
        peak_memory_kb: peak_memory,
        avg_chunk_time_ms: avg_chunk_time
    }
}

slay benchmark_streaming_hex(total_size drip) StreamingBenchmark {
    vibez.spill("📊 Benchmarking streaming Hex for " + int_to_string(total_size) + " bytes...")
    
    sus encoder StreamEncoder = create_stream_encoder("hex")
    sus test_data tea = generate_binary_data(STREAMING_CHUNK_SIZE)
    sus chunks_needed drip = total_size / STREAMING_CHUNK_SIZE
    sus start_time drip = get_current_time_ms()
    sus peak_memory drip = 0
    
    sus i drip = 0
    bestie i < chunks_needed {
        sus result tea = stream_encode_chunk(encoder, test_data)
        
        sus current_memory drip = estimate_streaming_memory(encoder)
        ready current_memory > peak_memory {
            peak_memory = current_memory
        }
        
        i = i + 1
    }
    
    sus final_result tea = stream_finalize(encoder)
    sus end_time drip = get_current_time_ms()
    sus total_time drip = end_time - start_time
    sus avg_chunk_time drip = total_time / chunks_needed
    
    damn StreamingBenchmark{
        encoding_type: "hex_streaming",
        total_bytes: total_size,
        chunk_count: chunks_needed,
        total_time_ms: total_time,
        peak_memory_kb: peak_memory,
        avg_chunk_time_ms: avg_chunk_time
    }
}

fr fr ===== COMPARATIVE PERFORMANCE TESTS =====

slay benchmark_all_encodings(data_size drip) {
    vibez.spill("🔥 Comprehensive Encoding Performance Test")
    vibez.spill("Data Size: " + int_to_string(data_size) + " bytes")
    vibez.spill("Iterations: " + int_to_string(BENCHMARK_ITERATIONS))
    vibez.spill("=" * 60)
    
    fr fr Base64 benchmarks
    sus base64_encode_result PerformanceResult = benchmark_base64_encoding(data_size)
    sus base64_decode_result PerformanceResult = benchmark_base64_decoding(data_size)
    
    fr fr Hex benchmarks  
    sus hex_encode_result PerformanceResult = benchmark_hex_encoding(data_size)
    sus hex_decode_result PerformanceResult = benchmark_hex_decoding(data_size)
    
    fr fr ASCII85 benchmark (simplified)
    sus ascii85_result PerformanceResult = benchmark_ascii85_encoding(data_size)
    
    fr fr URL encoding benchmark
    sus url_result PerformanceResult = benchmark_url_encoding(data_size)
    
    fr fr Display results
    print_performance_results([
        base64_encode_result,
        base64_decode_result,
        hex_encode_result,
        hex_decode_result,
        ascii85_result,
        url_result
    ])
    
    fr fr Streaming benchmarks for larger data
    ready data_size >= LARGE_DATA_SIZE {
        vibez.spill("\n🌊 Streaming Performance Tests")
        vibez.spill("=" * 40)
        
        sus streaming_b64 StreamingBenchmark = benchmark_streaming_base64(data_size)
        sus streaming_hex StreamingBenchmark = benchmark_streaming_hex(data_size)
        
        print_streaming_results([streaming_b64, streaming_hex])
    }
}

slay benchmark_ascii85_encoding(data_size drip) PerformanceResult {
    vibez.spill("📊 Benchmarking ASCII85 encoding for " + int_to_string(data_size) + " bytes...")
    
    sus test_data tea = generate_binary_data(data_size)
    sus start_time drip = get_current_time_ms()
    
    sus i drip = 0
    bestie i < BENCHMARK_ITERATIONS {
        sus encoded tea = ascii85_encode(test_data)
        i = i + 1
    }
    
    sus end_time drip = get_current_time_ms()
    sus total_time drip = end_time - start_time
    sus total_bytes drip = data_size * BENCHMARK_ITERATIONS
    sus throughput drip = (total_bytes * 1000) / (total_time * 1024 * 1024)
    sus ops_per_sec drip = (BENCHMARK_ITERATIONS * 1000) / total_time
    
    damn PerformanceResult{
        operation: "ascii85_encode",
        data_size: data_size,
        iterations: BENCHMARK_ITERATIONS,
        total_time_ms: total_time,
        throughput_mbps: throughput,
        ops_per_second: ops_per_sec,
        memory_usage_kb: estimate_memory_usage(data_size, 125)  fr fr ~25% increase
    }
}

slay benchmark_url_encoding(data_size drip) PerformanceResult {
    vibez.spill("📊 Benchmarking URL encoding for " + int_to_string(data_size) + " bytes...")
    
    fr fr Generate data with mix of safe and unsafe URL characters
    sus test_data tea = generate_test_data(data_size, "Hello World! @#$%^&*()+={}[]|\\:;\"'<>?,./ ")
    sus start_time drip = get_current_time_ms()
    
    sus i drip = 0
    bestie i < BENCHMARK_ITERATIONS {
        sus encoded tea = url_encode(test_data)
        i = i + 1
    }
    
    sus end_time drip = get_current_time_ms()
    sus total_time drip = end_time - start_time
    sus total_bytes drip = data_size * BENCHMARK_ITERATIONS
    sus throughput drip = (total_bytes * 1000) / (total_time * 1024 * 1024)
    sus ops_per_sec drip = (BENCHMARK_ITERATIONS * 1000) / total_time
    
    damn PerformanceResult{
        operation: "url_encode",
        data_size: data_size,
        iterations: BENCHMARK_ITERATIONS,
        total_time_ms: total_time,
        throughput_mbps: throughput,
        ops_per_second: ops_per_sec,
        memory_usage_kb: estimate_memory_usage(data_size, 180)  fr fr Variable expansion
    }
}

fr fr ===== SCALABILITY TESTS =====

slay test_scalability() {
    vibez.spill("🔄 Scalability Test - Multiple Data Sizes")
    vibez.spill("=" * 50)
    
    sus test_sizes drip[value] = [
        SMALL_DATA_SIZE,    fr fr 1KB
        MEDIUM_DATA_SIZE,   fr fr 64KB
        LARGE_DATA_SIZE,    fr fr 1MB
        XLARGE_DATA_SIZE    fr fr 10MB
    ]
    
    sus i drip = 0
    bestie i < array_length(test_sizes) {
        sus size drip = test_sizes[i]
        vibez.spill("\n📏 Testing size: " + format_bytes(size))
        benchmark_all_encodings(size)
        i = i + 1
    }
}

fr fr ===== UTILITY FUNCTIONS =====

slay estimate_memory_usage(input_size drip, expansion_percent drip) drip {
    fr fr Estimate memory usage in KB
    sus base_memory drip = input_size / 1024
    sus expansion_memory drip = (base_memory * expansion_percent) / 100
    sus overhead drip = 4  fr fr 4KB overhead for buffers/metadata
    damn base_memory + expansion_memory + overhead
}

slay estimate_streaming_memory(encoder StreamEncoder) drip {
    fr fr Estimate streaming encoder memory usage
    sus buffer_memory drip = string_length(encoder.input_buffer) + string_length(encoder.output_buffer)
    sus overhead drip = 2048  fr fr 2KB overhead for encoder state
    damn (buffer_memory + overhead) / 1024
}

slay print_performance_results(results PerformanceResult[value]) {
    sus i drip = 0
    bestie i < array_length(results) {
        sus result PerformanceResult = results[i]
        vibez.spill(
            "📈 " + result.operation + 
            " | " + int_to_string(result.throughput_mbps) + " MB/s" +
            " | " + int_to_string(result.ops_per_second) + " ops/s" +
            " | " + int_to_string(result.memory_usage_kb) + " KB"
        )
        i = i + 1
    }
}

slay print_streaming_results(results StreamingBenchmark[value]) {
    sus i drip = 0
    bestie i < array_length(results) {
        sus result StreamingBenchmark = results[i]
        vibez.spill(
            "🌊 " + result.encoding_type +
            " | " + int_to_string(result.total_bytes / 1024) + " KB" +
            " | " + int_to_string(result.chunk_count) + " chunks" +
            " | " + int_to_string(result.avg_chunk_time_ms) + " ms/chunk" +
            " | " + int_to_string(result.peak_memory_kb) + " KB peak"
        )
        i = i + 1
    }
}

slay format_bytes(bytes drip) tea {
    ready bytes < 1024 {
        damn int_to_string(bytes) + " B"
    } otherwise ready bytes < 1048576 {
        damn int_to_string(bytes / 1024) + " KB"
    } otherwise {
        damn int_to_string(bytes / 1048576) + " MB"
    }
}

slay get_current_time_ms() drip {
    fr fr Mock timestamp - replace with real implementation
    damn get_timestamp()
}

fr fr ===== MAIN PERFORMANCE TEST RUNNER =====

slay main_character() {
    vibez.spill("🚀 EncodingZ Performance Test Suite")
    vibez.spill("=" * 50)
    
    fr fr Individual encoding tests
    vibez.spill("🎯 Individual Performance Tests")
    benchmark_all_encodings(MEDIUM_DATA_SIZE)
    
    fr fr Scalability analysis
    test_scalability()
    
    fr fr Memory usage analysis
    vibez.spill("\n🧠 Memory Usage Analysis")
    vibez.spill("=" * 30)
    sus memory_test_data tea = generate_test_data(LARGE_DATA_SIZE, "Memory test data. ")
    sus base64_memory drip = estimate_memory_usage(LARGE_DATA_SIZE, 133)
    sus hex_memory drip = estimate_memory_usage(LARGE_DATA_SIZE, 200)
    sus ascii85_memory drip = estimate_memory_usage(LARGE_DATA_SIZE, 125)
    
    vibez.spill("📊 Memory usage for " + format_bytes(LARGE_DATA_SIZE) + " input:")
    vibez.spill("   Base64: " + int_to_string(base64_memory) + " KB")
    vibez.spill("   Hex:    " + int_to_string(hex_memory) + " KB")
    vibez.spill("   ASCII85:" + int_to_string(ascii85_memory) + " KB")
    
    fr fr Performance recommendations
    vibez.spill("\n💡 Performance Recommendations")
    vibez.spill("=" * 35)
    vibez.spill("• Use Base64 for optimal space/speed balance")
    vibez.spill("• Use streaming for data > 1MB to control memory")
    vibez.spill("• Use Hex for debugging and simple binary representation")
    vibez.spill("• Use ASCII85 when space efficiency is critical")
    vibez.spill("• Pre-allocate buffers for high-frequency operations")
    
    vibez.spill("\n🎉 Performance Test Suite Complete!")
}

main()
