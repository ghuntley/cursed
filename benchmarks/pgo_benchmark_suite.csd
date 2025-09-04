# CURSED PGO Benchmark Suite
# Comprehensive performance testing for Profile-Guided Optimization

yeet "vibez"
yeet "mathz" 
yeet "arrayz"
yeet "timez"
yeet "testz"

# Benchmark configuration
sus ITERATIONS drip = 100000
sus ARRAY_SIZE drip = 10000
sus FIBONACCI_N drip = 35

# Hot path benchmark - frequently called function
slay hot_computation(n drip) drip {
    sus sum drip = 0
    bestie (sus i drip = 0; i < n; i = i + 1) {
        sum = sum + (i * i + i / 2)
    }
    damn sum
}

# Cold path benchmark - infrequently called function  
slay cold_computation(n drip) drip {
    sus product drip = 1
    bestie (sus i drip = 1; i <= n; i = i + 1) {
        ready (i % 1000 == 0) {
            product = product * 2
        }
    }
    damn product
}

# Recursive benchmark for call graph optimization
slay fibonacci_recursive(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

# Array processing benchmark
slay array_processing_benchmark() drip {
    sus arr []drip = make_array_drip(ARRAY_SIZE)
    
    # Initialize array with hot path
    bestie (sus i drip = 0; i < ARRAY_SIZE; i = i + 1) {
        arr[i] = hot_computation(i % 100)
    }
    
    # Process array with branch patterns
    sus processed_sum drip = 0
    bestie (sus i drip = 0; i < ARRAY_SIZE; i = i + 1) {
        ready (arr[i] % 2 == 0) {  # Hot branch - even numbers more common
            processed_sum = processed_sum + arr[i] * 2
        } otherwise {  # Cold branch
            processed_sum = processed_sum + cold_computation(arr[i] % 10)
        }
    }
    
    damn processed_sum
}

# Memory access pattern benchmark
slay memory_access_benchmark() drip {
    sus matrix [][]drip = make_2d_array_drip(100, 100)
    sus sum drip = 0
    
    # Hot cache-friendly access pattern
    bestie (sus i drip = 0; i < 100; i = i + 1) {
        bestie (sus j drip = 0; j < 100; j = j + 1) {
            matrix[i][j] = i * j + hot_computation(i + j)
            sum = sum + matrix[i][j]
        }
    }
    
    # Cold random access pattern (less frequent)
    bestie (sus k drip = 0; k < 1000; k = k + 1) {
        sus rand_i drip = k % 100
        sus rand_j drip = (k * 17 + 23) % 100
        sum = sum + matrix[rand_i][rand_j] + cold_computation(k % 5)
    }
    
    damn sum
}

# Function call frequency benchmark
slay call_frequency_benchmark() drip {
    sus result drip = 0
    
    # Hot function called many times
    bestie (sus i drip = 0; i < ITERATIONS; i = i + 1) {
        result = result + hot_computation(i % 50)
    }
    
    # Cold function called rarely  
    bestie (sus j drip = 0; j < ITERATIONS / 1000; j = j + 1) {
        result = result + cold_computation(j % 10)
    }
    
    damn result
}

# Branch prediction benchmark
slay branch_prediction_benchmark() drip {
    sus predictable_sum drip = 0
    sus unpredictable_sum drip = 0
    
    # Predictable branch pattern (hot path)
    bestie (sus i drip = 0; i < ITERATIONS; i = i + 1) {
        ready (i % 4 != 0) {  # 75% taken - highly predictable
            predictable_sum = predictable_sum + hot_computation(i % 10)
        } otherwise {
            predictable_sum = predictable_sum + 1
        }
    }
    
    # Unpredictable branch pattern (cold path)
    bestie (sus j drip = 0; j < ITERATIONS / 10; j = j + 1) {
        sus random_val drip = (j * 1103515245 + 12345) % 2147483647
        ready (random_val % 2 == 0) {  # 50% taken - unpredictable
            unpredictable_sum = unpredictable_sum + cold_computation(j % 5)
        } otherwise {
            unpredictable_sum = unpredictable_sum + (j * 2)
        }
    }
    
    damn predictable_sum + unpredictable_sum
}

# Main benchmark execution
slay main_character() {
    test_start("PGO Benchmark Suite")
    
    vibez.spill("🔥 Running CURSED PGO Performance Benchmarks...")
    vibez.spill("Iterations: {}, Array Size: {}, Fibonacci N: {}", .{ITERATIONS, ARRAY_SIZE, FIBONACCI_N})
    
    # Warm up the system
    vibez.spill("🔧 Warming up...")
    bestie (sus warmup drip = 0; warmup < 10; warmup = warmup + 1) {
        _ = hot_computation(100)
    }
    
    # Run benchmarks with timing
    sus start_time drip = get_time_ms()
    
    vibez.spill("📊 Hot/Cold Path Benchmark...")
    sus call_freq_result drip = call_frequency_benchmark()
    
    vibez.spill("📊 Array Processing Benchmark...")  
    sus array_result drip = array_processing_benchmark()
    
    vibez.spill("📊 Memory Access Pattern Benchmark...")
    sus memory_result drip = memory_access_benchmark()
    
    vibez.spill("📊 Branch Prediction Benchmark...")
    sus branch_result drip = branch_prediction_benchmark()
    
    vibez.spill("📊 Recursive Fibonacci Benchmark...")
    sus fib_result drip = fibonacci_recursive(FIBONACCI_N)
    
    sus end_time drip = get_time_ms()
    sus total_time drip = end_time - start_time
    
    # Display results
    vibez.spill("")
    vibez.spill("🏆 Benchmark Results:")
    vibez.spill("  Call Frequency Result: {}", .{call_freq_result})
    vibez.spill("  Array Processing Result: {}", .{array_result})
    vibez.spill("  Memory Access Result: {}", .{memory_result})
    vibez.spill("  Branch Prediction Result: {}", .{branch_result})
    vibez.spill("  Fibonacci({}) Result: {}", .{FIBONACCI_N, fib_result})
    vibez.spill("  Total Execution Time: {}ms", .{total_time})
    
    # Performance assertions for PGO validation
    assert_true(call_freq_result > 0, "Call frequency benchmark should produce positive result")
    assert_true(array_result > 0, "Array processing benchmark should produce positive result")
    assert_true(fib_result > 0, "Fibonacci benchmark should produce positive result")
    assert_true(total_time > 0, "Total time should be positive")
    
    vibez.spill("✅ PGO Benchmark Suite completed successfully")
    print_test_summary()
}
