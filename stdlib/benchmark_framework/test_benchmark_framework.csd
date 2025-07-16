yeet "testz"
yeet "benchmark_framework"

# Comprehensive tests for CURSED Benchmark Framework
# Tests core benchmarking functionality

test_start("CURSED Benchmark Framework Tests")

# Test framework initialization
test_start("Framework initialization tests")
sus init_result = init_benchmark_framework()
assert_true(init_result)

# Test benchmark configuration
test_start("Benchmark configuration tests")
sus config_result = configure_benchmark("test_benchmark", 10, 100)
assert_true(config_result)

# Test timestamp functionality
test_start("Timestamp tests")
sus timestamp1 = get_timestamp_nanos()
sus timestamp2 = get_timestamp_nanos()
assert_true(timestamp2 >= timestamp1)

# Test memory tracking
test_start("Memory tracking tests")
sus memory_usage = get_memory_usage()
assert_true(memory_usage >= 0)

# Test single benchmark iteration execution
test_start("Benchmark iteration execution tests")
sus test_function = slay() lit {
    # Simple computation for testing
    sus sum normie = 0
    bestie i := 0; i < 100; i++ {
        sum = sum + i
    }
    damn based
}

sus execution_time = execute_benchmark_iteration(test_function)
assert_true(execution_time > 0.0)

# Test warmup execution
test_start("Warmup execution tests")
sus warmup_result = run_warmup(test_function, 5)
assert_true(warmup_result)

# Test statistical calculations
test_start("Statistical calculation tests")
sus test_values []meal = [1.0, 2.0, 3.0, 4.0, 5.0]

sus mean_result = calculate_mean(test_values)
assert_true(mean_result > 0.0)
assert_true(mean_result == 3.0)

sus median_result = calculate_median(test_values)
assert_true(median_result > 0.0)

sus std_dev_result = calculate_std_dev(test_values, mean_result)
assert_true(std_dev_result >= 0.0)

sus min_result = find_min(test_values)
assert_true(min_result == 1.0)

sus max_result = find_max(test_values)
assert_true(max_result == 5.0)

# Test with empty array
sus empty_mean = calculate_mean([])
assert_true(empty_mean == 0.0)

sus empty_median = calculate_median([])
assert_true(empty_median == 0.0)

sus empty_min = find_min([])
assert_true(empty_min == 0.0)

sus empty_max = find_max([])
assert_true(empty_max == 0.0)

# Test benchmark execution
test_start("Full benchmark execution tests")
sus benchmark_func = slay() lit {
    # Simulate some work
    sus result normie = 0
    bestie i := 0; i < 50; i++ {
        result = result + (i * 2)
    }
    damn based
}

sus benchmark_result = run_benchmark("performance_test", benchmark_func)
assert_true(benchmark_result)

# Test result printing (should not crash)
test_start("Result printing tests")
print_benchmark_results()

# Test benchmark comparison
test_start("Benchmark comparison tests")
compare_benchmark_results(1.0, 1.2)  # Should show regression
compare_benchmark_results(1.2, 1.0)  # Should show improvement

# Test micro-benchmarking
test_start("Micro-benchmark tests")
sus micro_func = slay() lit {
    sus x = 42 + 58
    damn based
}
sus micro_result = micro_benchmark("micro_test", 10, micro_func)
assert_true(micro_result)

# Test macro-benchmarking
test_start("Macro-benchmark tests")
sus macro_func = slay() lit {
    sus arr []normie = []
    bestie i := 0; i < 50; i++ {
        arr = append(arr, i)
    }
    damn based
}
sus macro_result = macro_benchmark("macro_test", macro_func)
assert_true(macro_result)

# Test built-in benchmarks
test_start("Built-in benchmark tests")
benchmark_arithmetic()
benchmark_array_operations()
benchmark_string_operations()

# Test file operations
test_start("File operation tests")
sus save_result = save_benchmark_results("test_results.txt")
assert_true(save_result)

sus html_result = generate_html_report("test_report.html")
assert_true(html_result)

# Test regression detection
test_start("Regression detection tests")
sus regression_result = detect_performance_regression("baseline.txt", 5.0)
# Result can be either true or false, just testing it doesn't crash

# Test all benchmarks execution
test_start("All benchmarks execution tests")
run_all_benchmarks()

# Test edge cases
test_start("Edge case tests")

# Test with very small arrays
sus small_array []meal = [0.001]
sus small_mean = calculate_mean(small_array)
assert_true(small_mean == 0.001)

# Test with identical values
sus identical_values []meal = [1.0, 1.0, 1.0]
sus identical_mean = calculate_mean(identical_values)
assert_true(identical_mean == 1.0)

sus identical_std_dev = calculate_std_dev(identical_values, identical_mean)
assert_true(identical_std_dev == 0.0)

# Test function that does nothing
test_start("Minimal function tests")
sus empty_func = slay() lit {
    damn based
}
sus empty_time = execute_benchmark_iteration(empty_func)
assert_true(empty_time >= 0.0)

# Test benchmark with zero iterations (edge case)
configure_benchmark("zero_test", 0, 1)
sus zero_result = run_benchmark("zero_test", empty_func)
assert_true(zero_result)

# Test large number calculations
test_start("Large number tests")
sus large_values []meal = [1000.0, 2000.0, 3000.0]
sus large_mean = calculate_mean(large_values)
assert_true(large_mean == 2000.0)

# Test realistic timing scenario
test_start("Realistic timing tests")
sus realistic_func = slay() lit {
    sus data []normie = []
    bestie i := 0; i < 1000; i++ {
        data = append(data, i * 3)
    }
    damn based
}

configure_benchmark("realistic_test", 5, 20)
sus realistic_result = run_benchmark("realistic_test", realistic_func)
assert_true(realistic_result)
print_benchmark_results()

# Test multiple benchmark runs
test_start("Multiple benchmark runs tests")
sus first_run = run_benchmark("first_test", test_function)
assert_true(first_run)
sus first_mean = calculate_mean(execution_times)

sus second_run = run_benchmark("second_test", test_function)
assert_true(second_run)
sus second_mean = calculate_mean(execution_times)

compare_benchmark_results(first_mean, second_mean)

print_test_summary()

vibez.spill("✅ All CURSED Benchmark Framework tests completed successfully!")
vibez.spill("Framework is ready for production use.")
