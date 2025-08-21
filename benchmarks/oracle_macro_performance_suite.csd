# Oracle Macro Performance Test Suite
# Comprehensive performance benchmarks for PGO validation

yeet "vibez"
yeet "mathz" 
yeet "arrayz"
yeet "timez"
yeet "testz"
yeet "concurrenz"

# Configuration constants
sus LARGE_ARRAY_SIZE drip = 100000
sus COMPUTATION_ITERATIONS drip = 1000000
sus CONCURRENT_WORKERS drip = 16
sus MEMORY_STRESS_SIZE drip = 50000

# CPU-intensive benchmark
slay cpu_intensive_benchmark() drip {
    sus result drip = 0
    sus prime_count drip = 0
    
    # Prime number calculation (CPU bound)
    bestie (sus n drip = 2; n < 10000; n = n + 1) {
        sus is_prime lit = based
        sus sqrt_n drip = mathz.sqrt_int(n)
        
        bestie (sus i drip = 2; i <= sqrt_n; i = i + 1) {
            ready (n % i == 0) {
                is_prime = fake
                break
            }
        }
        
        ready (is_prime) {
            prime_count = prime_count + 1
            result = result + n
        }
    }
    
    vibez.spill("Prime numbers found: {}, Sum: {}", .{prime_count, result})
    damn result
}

# Memory-intensive benchmark
slay memory_intensive_benchmark() drip {
    sus matrices [][]drip = make_2d_array_drip(500, 500)
    sus total_sum drip = 0
    
    # Matrix initialization and multiplication
    bestie (sus i drip = 0; i < 500; i = i + 1) {
        bestie (sus j drip = 0; j < 500; j = j + 1) {
            matrices[i][j] = (i * j + i + j) % 1000
        }
    }
    
    # Matrix operations (memory bound)
    bestie (sus i drip = 0; i < 500; i = i + 1) {
        bestie (sus j drip = 0; j < 500; j = j + 1) {
            sus sum drip = 0
            bestie (sus k drip = 0; k < 500; k = k + 1) {
                sum = sum + (matrices[i][k] * matrices[k][j]) % 10000
            }
            total_sum = total_sum + sum
        }
    }
    
    vibez.spill("Matrix computation result: {}", .{total_sum})
    damn total_sum
}

# Branch-heavy benchmark (prediction testing)
slay branch_prediction_benchmark() drip {
    sus data []drip = make_array_drip(LARGE_ARRAY_SIZE)
    sus processed_count drip = 0
    sus total_result drip = 0
    
    # Initialize with pseudo-random data
    bestie (sus i drip = 0; i < LARGE_ARRAY_SIZE; i = i + 1) {
        data[i] = (i * 1103515245 + 12345) % 1000
    }
    
    # Complex branching pattern
    bestie (sus i drip = 0; i < LARGE_ARRAY_SIZE; i = i + 1) {
        sus value drip = data[i]
        
        ready (value < 100) {
            # Frequent branch
            total_result = total_result + value * 2
            processed_count = processed_count + 1
        } otherwise ready (value < 300) {
            # Medium frequency branch
            total_result = total_result + value * 3 + 10
            processed_count = processed_count + 1
        } otherwise ready (value < 600) {
            # Medium frequency branch
            total_result = total_result + (value * 4) % 500
            processed_count = processed_count + 1
        } otherwise ready (value < 800) {
            # Less frequent branch
            total_result = total_result + mathz.pow_int(value, 2) % 1000
            processed_count = processed_count + 1
        } otherwise {
            # Rare branch
            bestie (sus j drip = 0; j < 10; j = j + 1) {
                total_result = total_result + (value + j) % 100
            }
            processed_count = processed_count + 1
        }
    }
    
    vibez.spill("Branch benchmark processed: {}, result: {}", .{processed_count, total_result})
    damn total_result
}

# Function call frequency benchmark
slay function_call_benchmark() drip {
    sus accumulator drip = 0
    
    # Hot function (called very frequently)
    slay hot_function(x drip) drip {
        damn x * x + x / 2 + 42
    }
    
    # Warm function (called moderately)
    slay warm_function(x drip) drip {
        sus result drip = x
        bestie (sus i drip = 0; i < 10; i = i + 1) {
            result = result + (i * x) % 100
        }
        damn result
    }
    
    # Cold function (called rarely)
    slay cold_function(x drip) drip {
        sus result drip = 1
        bestie (sus i drip = 1; i <= x % 20; i = i + 1) {
            result = result * (i % 10 + 1)
            result = result % 10000
        }
        damn result
    }
    
    # Execute with different call frequencies
    bestie (sus i drip = 0; i < COMPUTATION_ITERATIONS; i = i + 1) {
        # Hot function called 90% of time
        ready (i % 10 < 9) {
            accumulator = accumulator + hot_function(i % 100)
        } otherwise ready (i % 100 < 95) {
            # Warm function called 5% of time
            accumulator = accumulator + warm_function(i % 50)
        } otherwise {
            # Cold function called 5% of time
            accumulator = accumulator + cold_function(i % 25)
        }
    }
    
    vibez.spill("Function call benchmark result: {}", .{accumulator})
    damn accumulator
}

# Loop optimization benchmark
slay loop_optimization_benchmark() drip {
    sus data []drip = make_array_drip(LARGE_ARRAY_SIZE)
    sus result drip = 0
    
    # Initialize data
    bestie (sus i drip = 0; i < LARGE_ARRAY_SIZE; i = i + 1) {
        data[i] = i % 1000
    }
    
    # Vectorizable loop (sequential access)
    bestie (sus i drip = 0; i < LARGE_ARRAY_SIZE; i = i + 1) {
        data[i] = data[i] * 2 + 1
        result = result + data[i]
    }
    
    # Unrollable loop (fixed iterations)
    bestie (sus i drip = 0; i < LARGE_ARRAY_SIZE; i = i + 8) {
        result = result + data[i] + data[i+1] + data[i+2] + data[i+3]
        result = result + data[i+4] + data[i+5] + data[i+6] + data[i+7]
    }
    
    # Loop with data dependencies (harder to optimize)
    bestie (sus i drip = 1; i < LARGE_ARRAY_SIZE; i = i + 1) {
        data[i] = data[i] + data[i-1] % 100
        result = result + data[i]
    }
    
    vibez.spill("Loop optimization result: {}", .{result})
    damn result
}

# Concurrent benchmark
slay concurrent_benchmark() drip {
    sus shared_counter drip = 0
    sus results []drip = make_array_drip(CONCURRENT_WORKERS)
    
    # Worker function
    slay worker_function(worker_id drip, iterations drip) drip {
        sus local_result drip = 0
        
        bestie (sus i drip = 0; i < iterations; i = i + 1) {
            local_result = local_result + (worker_id * 1000 + i) % 500
            
            # Simulate some computation
            bestie (sus j drip = 0; j < 100; j = j + 1) {
                local_result = local_result + (j * worker_id) % 50
            }
        }
        
        damn local_result
    }
    
    # Launch concurrent workers
    sus worker_iterations drip = COMPUTATION_ITERATIONS / CONCURRENT_WORKERS
    
    bestie (sus worker drip = 0; worker < CONCURRENT_WORKERS; worker = worker + 1) {
        results[worker] = worker_function(worker, worker_iterations)
    }
    
    # Aggregate results
    sus total_result drip = 0
    bestie (sus i drip = 0; i < CONCURRENT_WORKERS; i = i + 1) {
        total_result = total_result + results[i]
    }
    
    vibez.spill("Concurrent benchmark with {} workers: {}", .{CONCURRENT_WORKERS, total_result})
    damn total_result
}

# I/O simulation benchmark
slay io_simulation_benchmark() drip {
    sus buffer_size drip = 1024
    sus iterations drip = 1000
    sus total_bytes drip = 0
    
    bestie (sus round drip = 0; round < iterations; round = round + 1) {
        # Simulate file I/O with memory operations
        sus buffer []drip = make_array_drip(buffer_size)
        
        # "Write" operation
        bestie (sus i drip = 0; i < buffer_size; i = i + 1) {
            buffer[i] = (round * 1000 + i) % 256
        }
        
        # "Read" and process operation
        sus checksum drip = 0
        bestie (sus i drip = 0; i < buffer_size; i = i + 1) {
            checksum = checksum + buffer[i]
            buffer[i] = (buffer[i] + 1) % 256
        }
        
        total_bytes = total_bytes + buffer_size
        
        # Simulate I/O latency with computation
        ready (checksum > 100000) {
            bestie (sus delay drip = 0; delay < 10; delay = delay + 1) {
                checksum = checksum % 1000000
            }
        }
    }
    
    vibez.spill("I/O simulation processed {} bytes", .{total_bytes})
    damn total_bytes
}

# Memory allocation stress test
slay memory_allocation_benchmark() drip {
    sus allocation_count drip = 1000
    sus total_allocated drip = 0
    
    bestie (sus round drip = 0; round < 100; round = round + 1) {
        # Allocate arrays of varying sizes
        bestie (sus i drip = 0; i < allocation_count; i = i + 1) {
            sus size drip = (i % 100 + 1) * 10
            sus temp_array []drip = make_array_drip(size)
            
            # Use the array to prevent optimization away
            bestie (sus j drip = 0; j < size; j = j + 1) {
                temp_array[j] = (i + j) % 1000
                total_allocated = total_allocated + temp_array[j]
            }
        }
        
        # Simulate memory pressure
        ready (round % 20 == 0) {
            # Force some memory operations
            bestie (sus cleanup drip = 0; cleanup < 1000; cleanup = cleanup + 1) {
                total_allocated = total_allocated % 1000000
            }
        }
    }
    
    vibez.spill("Memory allocation benchmark total: {}", .{total_allocated})
    damn total_allocated
}

# Cache performance benchmark
slay cache_performance_benchmark() drip {
    sus matrix_size drip = 1000
    sus matrix [][]drip = make_2d_array_drip(matrix_size, matrix_size)
    sus cache_friendly_sum drip = 0
    sus cache_unfriendly_sum drip = 0
    
    # Initialize matrix
    bestie (sus i drip = 0; i < matrix_size; i = i + 1) {
        bestie (sus j drip = 0; j < matrix_size; j = j + 1) {
            matrix[i][j] = (i * 1000 + j) % 500
        }
    }
    
    # Cache-friendly access (row-major)
    bestie (sus i drip = 0; i < matrix_size; i = i + 1) {
        bestie (sus j drip = 0; j < matrix_size; j = j + 1) {
            cache_friendly_sum = cache_friendly_sum + matrix[i][j]
        }
    }
    
    # Cache-unfriendly access (column-major)
    bestie (sus j drip = 0; j < matrix_size; j = j + 1) {
        bestie (sus i drip = 0; i < matrix_size; i = i + 1) {
            cache_unfriendly_sum = cache_unfriendly_sum + matrix[i][j]
        }
    }
    
    sus total_sum drip = cache_friendly_sum + cache_unfriendly_sum
    vibez.spill("Cache benchmark - Friendly: {}, Unfriendly: {}", .{cache_friendly_sum, cache_unfriendly_sum})
    damn total_sum
}

# Main benchmark execution
slay main() {
    test_start("Oracle Macro Performance Suite")
    
    vibez.spill("🚀 Starting Oracle Macro Performance Benchmarks")
    vibez.spill("Configuration:")
    vibez.spill("  Large Array Size: {}", .{LARGE_ARRAY_SIZE})
    vibez.spill("  Computation Iterations: {}", .{COMPUTATION_ITERATIONS})
    vibez.spill("  Concurrent Workers: {}", .{CONCURRENT_WORKERS})
    vibez.spill("")
    
    # Record overall start time
    sus overall_start drip = get_time_ms()
    
    # Run benchmarks with timing
    vibez.spill("🧮 CPU Intensive Benchmark...")
    sus cpu_start drip = get_time_ms()
    sus cpu_result drip = cpu_intensive_benchmark()
    sus cpu_time drip = get_time_ms() - cpu_start
    
    vibez.spill("💾 Memory Intensive Benchmark...")
    sus mem_start drip = get_time_ms()
    sus mem_result drip = memory_intensive_benchmark()
    sus mem_time drip = get_time_ms() - mem_start
    
    vibez.spill("🌿 Branch Prediction Benchmark...")
    sus branch_start drip = get_time_ms()
    sus branch_result drip = branch_prediction_benchmark()
    sus branch_time drip = get_time_ms() - branch_start
    
    vibez.spill("📞 Function Call Benchmark...")
    sus func_start drip = get_time_ms()
    sus func_result drip = function_call_benchmark()
    sus func_time drip = get_time_ms() - func_start
    
    vibez.spill("🔄 Loop Optimization Benchmark...")
    sus loop_start drip = get_time_ms()
    sus loop_result drip = loop_optimization_benchmark()
    sus loop_time drip = get_time_ms() - loop_start
    
    vibez.spill("⚡ Concurrent Benchmark...")
    sus conc_start drip = get_time_ms()
    sus conc_result drip = concurrent_benchmark()
    sus conc_time drip = get_time_ms() - conc_start
    
    vibez.spill("💿 I/O Simulation Benchmark...")
    sus io_start drip = get_time_ms()
    sus io_result drip = io_simulation_benchmark()
    sus io_time drip = get_time_ms() - io_start
    
    vibez.spill("🏠 Memory Allocation Benchmark...")
    sus alloc_start drip = get_time_ms()
    sus alloc_result drip = memory_allocation_benchmark()
    sus alloc_time drip = get_time_ms() - alloc_start
    
    vibez.spill("🎯 Cache Performance Benchmark...")
    sus cache_start drip = get_time_ms()
    sus cache_result drip = cache_performance_benchmark()
    sus cache_time drip = get_time_ms() - cache_start
    
    sus overall_time drip = get_time_ms() - overall_start
    
    # Display comprehensive results
    vibez.spill("")
    vibez.spill("🏆 Oracle Macro Performance Results")
    vibez.spill("====================================")
    
    vibez.spill("CPU Intensive:")
    vibez.spill("  Result: {}, Time: {}ms", .{cpu_result, cpu_time})
    
    vibez.spill("Memory Intensive:")
    vibez.spill("  Result: {}, Time: {}ms", .{mem_result, mem_time})
    
    vibez.spill("Branch Prediction:")
    vibez.spill("  Result: {}, Time: {}ms", .{branch_result, branch_time})
    
    vibez.spill("Function Calls:")
    vibez.spill("  Result: {}, Time: {}ms", .{func_result, func_time})
    
    vibez.spill("Loop Optimization:")
    vibez.spill("  Result: {}, Time: {}ms", .{loop_result, loop_time})
    
    vibez.spill("Concurrent Processing:")
    vibez.spill("  Result: {}, Time: {}ms", .{conc_result, conc_time})
    
    vibez.spill("I/O Simulation:")
    vibez.spill("  Result: {}, Time: {}ms", .{io_result, io_time})
    
    vibez.spill("Memory Allocation:")
    vibez.spill("  Result: {}, Time: {}ms", .{alloc_result, alloc_time})
    
    vibez.spill("Cache Performance:")
    vibez.spill("  Result: {}, Time: {}ms", .{cache_result, cache_time})
    
    vibez.spill("")
    vibez.spill("Overall Execution Time: {}ms", .{overall_time})
    
    # Performance assertions for validation
    assert_true(cpu_result > 0, "CPU benchmark should produce result")
    assert_true(mem_result > 0, "Memory benchmark should produce result")
    assert_true(branch_result > 0, "Branch benchmark should produce result")
    assert_true(func_result > 0, "Function call benchmark should produce result")
    assert_true(loop_result > 0, "Loop benchmark should produce result")
    assert_true(conc_result > 0, "Concurrent benchmark should produce result")
    assert_true(io_result > 0, "I/O benchmark should produce result")
    assert_true(alloc_result > 0, "Allocation benchmark should produce result")
    assert_true(cache_result > 0, "Cache benchmark should produce result")
    assert_true(overall_time > 0, "Overall time should be positive")
    
    # Performance regression detection assertions
    assert_true(cpu_time < 10000, "CPU benchmark should complete in reasonable time")
    assert_true(mem_time < 15000, "Memory benchmark should complete in reasonable time")
    assert_true(overall_time < 60000, "Overall suite should complete within 60 seconds")
    
    vibez.spill("✅ Oracle Macro Performance Suite completed successfully!")
    print_test_summary()
}
