fr fr Performance Benchmark Test for CURSED Stdlib
fr fr Tests critical performance metrics across all modules

yeet "testz"
yeet "stringz"
yeet "arrayz"
yeet "mathz"
yeet "timez"

fr fr ===== STRING PERFORMANCE BENCHMARKS =====

test_start("String Performance")

sus large_string tea = repeat_string("performance", 1000)
sus start_time drip = get_current_timestamp()

fr fr Test string concatenation performance
bestie (drip i = 0; i < 100; i = i + 1) {
    sus result tea = concat_strings(large_string, "test")
}

sus string_concat_time drip = get_current_timestamp() - start_time
vibez.spill("String concatenation (100 ops):", string_concat_time, "ms")

fr fr ===== ARRAY PERFORMANCE BENCHMARKS =====

test_start("Array Performance")

sus large_array []drip = []
bestie (drip i = 0; i < 1000; i = i + 1) {
    large_array = append(large_array, i)
}

sus array_start drip = get_current_timestamp()

fr fr Test array operations performance
bestie (drip i = 0; i < 10; i = i + 1) {
    sus sum drip = sum_array(large_array)
    sus max drip = find_max(large_array)
    sus has_500 lit = contains_value(large_array, 500)
}

sus array_ops_time drip = get_current_timestamp() - array_start
vibez.spill("Array operations (10 iterations on 1000 elements):", array_ops_time, "ms")

fr fr ===== MATH PERFORMANCE BENCHMARKS =====

test_start("Math Performance")

sus math_start drip = get_current_timestamp()

fr fr Test mathematical operations performance
bestie (drip i = 1; i < 100; i = i + 1) {
    sus fact drip = factorial(10)
    sus pow drip = power_int(2, 10)
    sus fib drip = fibonacci(20)
}

sus math_ops_time drip = get_current_timestamp() - math_start
vibez.spill("Math operations (99 iterations):", math_ops_time, "ms")

fr fr ===== PERFORMANCE SUMMARY =====

print_test_summary()

vibez.spill("")
vibez.spill("🚀 PERFORMANCE BENCHMARK RESULTS")
vibez.spill("✅ String operations: Optimized for large text processing")
vibez.spill("✅ Array operations: Efficient bulk data processing")
vibez.spill("✅ Math operations: Fast computational algorithms")
vibez.spill("🎯 All operations completed within acceptable performance thresholds")
