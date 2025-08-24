yeet "testz"

# Test enhanced testz framework with real timing and memory functions
test_start("Enhanced testz Framework")

# Test timing functions
sus start_time drip = get_time_microseconds()
sleep_microseconds(1000)  # 1ms
sus end_time drip = get_time_microseconds()
sus elapsed drip = end_time - start_time
assert_true(elapsed >= 1000, "Timing function should measure at least 1ms")

# Test memory tracking functions
sus start_memory drip = get_memory_usage()
sus large_array []drip = allocate_array(1000)
sus end_memory drip = get_memory_usage()
assert_true(end_memory > start_memory, "Memory tracking should detect allocation")

# Test benchmark functions
benchmark_start("Array allocation test")
sus test_array []drip = allocate_array(500)
sus duration drip = benchmark_end()
assert_true(duration >= 0, "Benchmark should return non-negative duration")

# Test assertion enhancements
assert_eq_float(3.14159, 3.14159, 0.00001, "Float comparison with tolerance")
assert_not_null(test_array, "Array should not be null")
assert_array_length(test_array, 500, "Array should have correct length")

vibez.spill("✅ Enhanced testz framework: All real timing and memory functions working")
print_test_summary()
