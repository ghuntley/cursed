# Comprehensive performance comparison suite for CURSED
# This benchmark tests various language features against equivalent implementations

yeet "vibez"
yeet "mathz"
yeet "arrayz"
yeet "stringz"
yeet "timez"

# Benchmark 1: Mathematical computations (CPU-intensive)
slay benchmark_mathematical_operations() drip {
    vibez.spill("Running mathematical operations benchmark...")
    
    sus start_time drip = timez.now_ms()
    sus result drip = 0
    
    bestie (i drip = 0; i < 1000000; i = i + 1) {
        sus x drip = mathz.sqrt(mathz.pow(i, 2))
        sus y drip = mathz.sin(x) + mathz.cos(x)
        sus z drip = mathz.log(x + 1)
        result = result + z
    }
    
    sus end_time drip = timez.now_ms()
    sus duration drip = end_time - start_time
    
    vibez.spill("Mathematical operations completed in", duration, "ms")
    vibez.spill("Result:", result)
    
    damn duration
}

# Benchmark 2: Array operations (memory-intensive)
slay benchmark_array_operations() drip {
    vibez.spill("Running array operations benchmark...")
    
    sus start_time drip = timez.now_ms()
    
    # Create large array
    sus large_array []drip = []
    bestie (i drip = 0; i < 100000; i = i + 1) {
        arrayz.push(large_array, i)
    }
    
    # Perform operations on array
    sus sum drip = 0
    bestie (i drip = 0; i < arrayz.len(large_array); i = i + 1) {
        sum = sum + large_array[i]
    }
    
    # Sort array (if sorting is available)
    bestie (i drip = 0; i < arrayz.len(large_array) - 1; i = i + 1) {
        bestie (j drip = i + 1; j < arrayz.len(large_array); j = j + 1) {
            ready (large_array[i] > large_array[j]) {
                sus temp drip = large_array[i]
                large_array[i] = large_array[j]
                large_array[j] = temp
            }
        }
    }
    
    sus end_time drip = timez.now_ms()
    sus duration drip = end_time - start_time
    
    vibez.spill("Array operations completed in", duration, "ms")
    vibez.spill("Sum:", sum)
    
    damn duration
}

# Benchmark 3: String processing (string-intensive)
slay benchmark_string_operations() drip {
    vibez.spill("Running string operations benchmark...")
    
    sus start_time drip = timez.now_ms()
    
    sus base_string tea = "The quick brown fox jumps over the lazy dog. "
    sus result tea = ""
    
    bestie (i drip = 0; i < 10000; i = i + 1) {
        sus numbered_string tea = stringz.concat(base_string, stringz.from_int(i))
        result = stringz.concat(result, numbered_string)
        
        # Simulate string processing
        sus words []tea = stringz.split(numbered_string, " ")
        sus word_count drip = arrayz.len(words)
    }
    
    sus end_time drip = timez.now_ms()
    sus duration drip = end_time - start_time
    
    vibez.spill("String operations completed in", duration, "ms")
    vibez.spill("Result length:", stringz.len(result))
    
    damn duration
}

# Benchmark 4: Recursive algorithms
slay fibonacci_recursive(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

slay benchmark_recursive_algorithms() drip {
    vibez.spill("Running recursive algorithms benchmark...")
    
    sus start_time drip = timez.now_ms()
    
    sus results []drip = []
    bestie (i drip = 1; i <= 35; i = i + 1) {
        sus fib_result drip = fibonacci_recursive(i)
        arrayz.push(results, fib_result)
    }
    
    sus end_time drip = timez.now_ms()
    sus duration drip = end_time - start_time
    
    vibez.spill("Recursive algorithms completed in", duration, "ms")
    vibez.spill("Results count:", arrayz.len(results))
    
    damn duration
}

# Benchmark 5: Memory allocation patterns
slay benchmark_memory_allocation() drip {
    vibez.spill("Running memory allocation benchmark...")
    
    sus start_time drip = timez.now_ms()
    
    bestie (i drip = 0; i < 1000; i = i + 1) {
        # Allocate various data structures
        sus int_array []drip = []
        sus string_array []tea = []
        
        bestie (j drip = 0; j < 1000; j = j + 1) {
            arrayz.push(int_array, j)
            arrayz.push(string_array, stringz.from_int(j))
        }
        
        # Process the data
        sus int_sum drip = 0
        sus string_length drip = 0
        
        bestie (k drip = 0; k < arrayz.len(int_array); k = k + 1) {
            int_sum = int_sum + int_array[k]
            string_length = string_length + stringz.len(string_array[k])
        }
    }
    
    sus end_time drip = timez.now_ms()
    sus duration drip = end_time - start_time
    
    vibez.spill("Memory allocation benchmark completed in", duration, "ms")
    
    damn duration
}

# Benchmark 6: Control flow intensive
slay benchmark_control_flow() drip {
    vibez.spill("Running control flow benchmark...")
    
    sus start_time drip = timez.now_ms()
    sus operations drip = 0
    
    bestie (i drip = 0; i < 100000; i = i + 1) {
        sus value drip = i % 100
        
        ready (value < 25) {
            operations = operations + 1
        } otherwise ready (value < 50) {
            operations = operations + 2
        } otherwise ready (value < 75) {
            operations = operations + 3
        } otherwise {
            operations = operations + 4
        }
        
        # Nested loops and conditions
        bestie (j drip = 0; j < 10; j = j + 1) {
            ready (j % 2 == 0) {
                operations = operations + j
            } otherwise {
                operations = operations - j
            }
        }
    }
    
    sus end_time drip = timez.now_ms()
    sus duration drip = end_time - start_time
    
    vibez.spill("Control flow benchmark completed in", duration, "ms")
    vibez.spill("Total operations:", operations)
    
    damn duration
}

# Main performance comparison suite
slay performance_comparison_main() {
    vibez.spill("CURSED Performance Comparison Suite")
    vibez.spill("===================================")
    
    sus math_time drip = benchmark_mathematical_operations()
    sus array_time drip = benchmark_array_operations()
    sus string_time drip = benchmark_string_operations()
    sus recursive_time drip = benchmark_recursive_algorithms()
    sus memory_time drip = benchmark_memory_allocation()
    sus control_time drip = benchmark_control_flow()
    
    sus total_time drip = math_time + array_time + string_time + recursive_time + memory_time + control_time
    
    vibez.spill("")
    vibez.spill("Performance Summary:")
    vibez.spill("===================")
    vibez.spill("Mathematical Operations:", math_time, "ms")
    vibez.spill("Array Operations:", array_time, "ms")
    vibez.spill("String Operations:", string_time, "ms")
    vibez.spill("Recursive Algorithms:", recursive_time, "ms")
    vibez.spill("Memory Allocation:", memory_time, "ms")
    vibez.spill("Control Flow:", control_time, "ms")
    vibez.spill("Total Time:", total_time, "ms")
    
    vibez.spill("")
    vibez.spill("Performance comparison suite completed successfully")
}

performance_comparison_main()
