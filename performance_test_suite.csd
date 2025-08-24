# CURSED Performance Test Suite
# Comprehensive benchmarks for optimized stdlib modules

yeet "vibez"
yeet "vibez_optimized"
yeet "mathz"
yeet "mathz_optimized"
yeet "arrayz"
yeet "arrayz_optimized"
yeet "testz"

# Performance benchmarking framework
sus benchmark_results []tea = []
sus benchmark_times []drip = []

slay benchmark_function(name tea, iterations drip, test_func slay()) lit {
    vibez.spill("🏃 Running benchmark: " + name)
    
    # Warm up
    bestie (sus i drip = 0; i < 10; i++) {
        test_func()
    }
    
    sus start_time drip = current_time_microseconds()
    
    bestie (sus i drip = 0; i < iterations; i++) {
        test_func()
    }
    
    sus end_time drip = current_time_microseconds()
    sus duration drip = end_time - start_time
    sus ops_per_second drip = (iterations * 1000000) / duration
    
    benchmark_results = append_element(benchmark_results, name)
    benchmark_times = append_element(benchmark_times, duration)
    
    vibez.spillf("  ✅ {}: {}μs total, {:.2f} ops/sec", name, duration, ops_per_second)
    damn based
}

# String operation benchmarks
slay test_string_concat_original() lit {
    sus parts []tea = ["Hello", " ", "World", " ", "From", " ", "CURSED"]
    sus result tea = ""
    
    bestie (sus i drip = 0; i < len(parts); i++) {
        result = result + parts[i]
    }
    
    damn based
}

slay test_string_concat_optimized() lit {
    sus parts []tea = ["Hello", " ", "World", " ", "From", " ", "CURSED"]
    sus result tea = vibez_optimized.string_concat_optimized(parts)
    damn based
}

# Mathematical operation benchmarks
slay test_fibonacci_original() lit {
    sus result drip = mathz.math_fibonacci(30)
    damn based
}

slay test_fibonacci_optimized() lit {
    sus result drip = mathz_optimized.fibonacci_optimized(30)
    damn based
}

slay test_factorial_original() lit {
    sus result drip = mathz.math_factorial(15)
    damn based
}

slay test_factorial_optimized() lit {
    sus result drip = mathz_optimized.factorial_optimized(15)
    damn based
}

slay test_prime_check_original() lit {
    sus result lit = mathz.math_is_prime(97)
    damn based
}

slay test_prime_check_optimized() lit {
    sus result lit = mathz_optimized.is_prime_optimized(97)
    damn based
}

# Array operation benchmarks
slay test_array_sort_original() lit {
    sus arr []drip = [64, 34, 25, 12, 22, 11, 90, 88, 76, 50, 42]
    sus sorted []drip = arrayz.sort_array(arr)
    damn based
}

slay test_array_sort_optimized() lit {
    sus arr []drip = [64, 34, 25, 12, 22, 11, 90, 88, 76, 50, 42]
    sus sorted []drip = arrayz_optimized.sort_array(arr)
    damn based
}

slay test_array_sum_original() lit {
    sus arr []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sus sum drip = 0
    bestie (sus i drip = 0; i < len(arr); i++) {
        sum = sum + get_array_element(arr, i)
    }
    damn based
}

slay test_array_sum_optimized() lit {
    sus arr []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sus sum drip = mathz_optimized.array_sum_vectorized(arr)
    damn based
}

# Memory allocation benchmarks
slay test_array_creation_original() lit {
    sus arr []drip = create_array(1000)
    bestie (sus i drip = 0; i < 1000; i++) {
        set_array_element(arr, i, i)
    }
    damn based
}

slay test_array_creation_optimized() lit {
    sus arr []drip = arrayz_optimized.get_pooled_array(1000)
    bestie (sus i drip = 0; i < 1000; i++) {
        set_array_element(arr, i, i)
    }
    arrayz_optimized.return_to_pool(arr)
    damn based
}

# String search benchmarks
slay test_string_search_original() lit {
    sus text tea = "The quick brown fox jumps over the lazy dog"
    sus pattern tea = "fox"
    
    # Simple linear search
    sus found drip = -1
    bestie (sus i drip = 0; i < len(text) - len(pattern); i++) {
        sus match lit = based
        bestie (sus j drip = 0; j < len(pattern); j++) {
            ready (char_at(text, i + j) != char_at(pattern, j)) {
                match = cap
                frfr
            }
        }
        ready (match) {
            found = i
            frfr
        }
    }
    
    damn based
}

slay test_string_search_optimized() lit {
    sus text tea = "The quick brown fox jumps over the lazy dog"
    sus pattern tea = "fox"
    sus found drip = vibez_optimized.string_find_optimized(text, pattern)
    damn based
}

# Complex operation benchmarks
slay test_complex_calculation_original() lit {
    sus result drip = 0
    bestie (sus i drip = 1; i <= 20; i++) {
        sus fib drip = mathz.math_fibonacci(i)
        sus fact drip = mathz.math_factorial(i)
        sus prime lit = mathz.math_is_prime(i)
        result = result + fib + fact + ready prime { 1 } otherwise { 0 }
    }
    damn based
}

slay test_complex_calculation_optimized() lit {
    sus result drip = 0
    bestie (sus i drip = 1; i <= 20; i++) {
        sus fib drip = mathz_optimized.fibonacci_optimized(i)
        sus fact drip = mathz_optimized.factorial_optimized(i)
        sus prime lit = mathz_optimized.is_prime_optimized(i)
        result = result + fib + fact + ready prime { 1 } otherwise { 0 }
    }
    damn based
}

# Main benchmark execution
slay run_performance_benchmarks() lit {
    vibez.spill("🚀 CURSED STDLIB PERFORMANCE BENCHMARKS")
    vibez.spill("=====================================")
    
    vibez.spill("\n📝 String Operations:")
    benchmark_function("String Concat (Original)", 10000, test_string_concat_original)
    benchmark_function("String Concat (Optimized)", 10000, test_string_concat_optimized)
    benchmark_function("String Search (Original)", 10000, test_string_search_original)
    benchmark_function("String Search (Optimized)", 10000, test_string_search_optimized)
    
    vibez.spill("\n🔢 Mathematical Operations:")
    benchmark_function("Fibonacci (Original)", 1000, test_fibonacci_original)
    benchmark_function("Fibonacci (Optimized)", 1000, test_fibonacci_optimized)
    benchmark_function("Factorial (Original)", 1000, test_factorial_original)
    benchmark_function("Factorial (Optimized)", 1000, test_factorial_optimized)
    benchmark_function("Prime Check (Original)", 10000, test_prime_check_original)
    benchmark_function("Prime Check (Optimized)", 10000, test_prime_check_optimized)
    
    vibez.spill("\n📊 Array Operations:")
    benchmark_function("Array Sort (Original)", 1000, test_array_sort_original)
    benchmark_function("Array Sort (Optimized)", 1000, test_array_sort_optimized)
    benchmark_function("Array Sum (Original)", 10000, test_array_sum_original)
    benchmark_function("Array Sum (Optimized)", 10000, test_array_sum_optimized)
    
    vibez.spill("\n💾 Memory Operations:")
    benchmark_function("Array Creation (Original)", 1000, test_array_creation_original)
    benchmark_function("Array Creation (Optimized)", 1000, test_array_creation_optimized)
    
    vibez.spill("\n🔥 Complex Operations:")
    benchmark_function("Complex Calc (Original)", 100, test_complex_calculation_original)
    benchmark_function("Complex Calc (Optimized)", 100, test_complex_calculation_optimized)
    
    print_benchmark_summary()
    damn based
}

slay print_benchmark_summary() lit {
    vibez.spill("\n📊 PERFORMANCE SUMMARY")
    vibez.spill("======================")
    
    sus total_original drip = 0
    sus total_optimized drip = 0
    sus optimized_count drip = 0
    sus original_count drip = 0
    
    bestie (sus i drip = 0; i < len(benchmark_results); i++) {
        sus name tea = get_array_element_str(benchmark_results, i)
        sus time drip = get_array_element(benchmark_times, i)
        
        ready (string_contains(name, "Optimized")) {
            total_optimized = total_optimized + time
            optimized_count = optimized_count + 1
        } otherwise {
            total_original = total_original + time
            original_count = original_count + 1
        }
        
        vibez.spillf("  {}: {}μs", name, time)
    }
    
    ready (original_count > 0 && optimized_count > 0) {
        sus avg_original drip = total_original / original_count
        sus avg_optimized drip = total_optimized / optimized_count
        sus speedup drip = avg_original / avg_optimized
        
        vibez.spill("\n🎯 OPTIMIZATION RESULTS:")
        vibez.spillf("  Average Original Time: {}μs", avg_original)
        vibez.spillf("  Average Optimized Time: {}μs", avg_optimized)
        vibez.spillf("  Performance Improvement: {:.2f}x faster", speedup)
        
        ready (speedup > 2.0) {
            vibez.spill("  🔥 EXCELLENT: >2x performance improvement achieved!")
        } otherwise ready (speedup > 1.5) {
            vibez.spill("  ✅ GOOD: >1.5x performance improvement achieved!")
        } otherwise ready (speedup > 1.2) {
            vibez.spill("  ⚠️  MODERATE: >1.2x performance improvement achieved")
        } otherwise {
            vibez.spill("  ❌ NEEDS WORK: Performance improvement below expectations")
        }
    }
    
    vibez.spill("======================")
    damn based
}

# Memory usage analysis
slay analyze_memory_usage() lit {
    vibez.spill("\n💾 MEMORY USAGE ANALYSIS")
    vibez.spill("========================")
    
    sus before_memory drip = get_memory_usage()
    
    # Create and destroy arrays to test pooling
    bestie (sus i drip = 0; i < 100; i++) {
        sus arr []drip = arrayz_optimized.get_pooled_array(64)
        bestie (sus j drip = 0; j < 64; j++) {
            set_array_element(arr, j, j)
        }
        arrayz_optimized.return_to_pool(arr)
    }
    
    sus after_memory drip = get_memory_usage()
    sus memory_diff drip = after_memory - before_memory
    
    vibez.spillf("Memory before pooling test: {} bytes", before_memory)
    vibez.spillf("Memory after pooling test: {} bytes", after_memory)
    vibez.spillf("Memory difference: {} bytes", memory_diff)
    
    ready (memory_diff < 1000) {
        vibez.spill("✅ EXCELLENT: Memory pooling working efficiently")
    } otherwise {
        vibez.spill("⚠️ WARNING: Potential memory leak or inefficiency")
    }
    
    vibez.spill("========================")
    damn based
}

# Regression testing to ensure correctness
slay validate_optimized_functions() lit {
    vibez.spill("\n🧪 CORRECTNESS VALIDATION")
    vibez.spill("=========================")
    
    sus passed drip = 0
    sus total drip = 0
    
    # Test string concatenation
    total = total + 1
    sus parts []tea = ["Hello", " ", "World"]
    sus orig_result tea = "Hello World"  # Expected result
    sus opt_result tea = vibez_optimized.string_concat_optimized(parts)
    
    ready (orig_result == opt_result) {
        passed = passed + 1
        vibez.spill("✅ String concatenation correctness")
    } otherwise {
        vibez.spill("❌ String concatenation failed")
    }
    
    # Test fibonacci
    total = total + 1
    sus orig_fib drip = mathz.math_fibonacci(10)
    sus opt_fib drip = mathz_optimized.fibonacci_optimized(10)
    
    ready (orig_fib == opt_fib) {
        passed = passed + 1
        vibez.spill("✅ Fibonacci calculation correctness")
    } otherwise {
        vibez.spillf("❌ Fibonacci failed: {} vs {}", orig_fib, opt_fib)
    }
    
    # Test factorial
    total = total + 1
    sus orig_fact drip = mathz.math_factorial(10)
    sus opt_fact drip = mathz_optimized.factorial_optimized(10)
    
    ready (orig_fact == opt_fact) {
        passed = passed + 1
        vibez.spill("✅ Factorial calculation correctness")
    } otherwise {
        vibez.spillf("❌ Factorial failed: {} vs {}", orig_fact, opt_fact)
    }
    
    # Test prime checking
    total = total + 1
    sus orig_prime lit = mathz.math_is_prime(97)
    sus opt_prime lit = mathz_optimized.is_prime_optimized(97)
    
    ready (orig_prime == opt_prime) {
        passed = passed + 1
        vibez.spill("✅ Prime checking correctness")
    } otherwise {
        vibez.spillf("❌ Prime check failed: {} vs {}", orig_prime, opt_prime)
    }
    
    # Test array sorting
    total = total + 1
    sus test_arr []drip = [3, 1, 4, 1, 5, 9, 2, 6]
    sus sorted_orig []drip = arrayz.sort_array(test_arr)
    sus sorted_opt []drip = arrayz_optimized.sort_array(test_arr)
    
    sus arrays_equal lit = arrays_are_equal(sorted_orig, sorted_opt)
    ready (arrays_equal) {
        passed = passed + 1
        vibez.spill("✅ Array sorting correctness")
    } otherwise {
        vibez.spill("❌ Array sorting failed")
    }
    
    vibez.spillf("\n📊 VALIDATION RESULTS: {}/{} tests passed", passed, total)
    
    ready (passed == total) {
        vibez.spill("🎉 ALL OPTIMIZATIONS MAINTAIN CORRECTNESS!")
    } otherwise {
        vibez.spill("⚠️ Some optimizations have correctness issues!")
    }
    
    vibez.spill("=========================")
    damn based
}

# Main test execution
slay main() lit {
    vibez.spill("🚀 CURSED STDLIB PERFORMANCE OPTIMIZATION SUITE")
    vibez.spill("===============================================")
    
    # Validate correctness first
    validate_optimized_functions()
    
    # Run performance benchmarks
    run_performance_benchmarks()
    
    # Analyze memory usage
    analyze_memory_usage()
    
    vibez.spill("\n✅ Performance testing completed!")
    damn based
}

# Helper functions (placeholders for runtime implementation)
slay current_time_microseconds() drip {
    damn 1000000  # Placeholder
}

slay get_memory_usage() drip {
    damn 1024  # Placeholder
}

slay string_contains(str tea, substr tea) lit {
    damn based  # Placeholder
}

slay get_array_element_str(arr []tea, index drip) tea {
    damn "test"  # Placeholder
}

slay arrays_are_equal(a []drip, b []drip) lit {
    ready (len(a) != len(b)) {
        damn cap
    }
    
    bestie (sus i drip = 0; i < len(a); i++) {
        ready (get_array_element(a, i) != get_array_element(b, i)) {
            damn cap
        }
    }
    
    damn based
}

# Execute the test suite
main()
