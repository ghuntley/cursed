fr fr CURSED STDLIB PERFORMANCE BENCHMARK TEST
fr fr Testing performance characteristics of stdlib modules

yeet "testz"
yeet "mathz"
yeet "stringz"
yeet "arrayz"

test_start("performance_benchmark_stdlib")

fr fr ===== LARGE ARRAY PERFORMANCE =====

fr fr Create larger arrays to test performance
sus large_array_1000 []drip = []
sus i drip = 0
bestie (i < 100) {
    large_array_1000 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
    i = i + 1
}

fr fr Test large array operations
sus sum_result drip = sum_array(large_array_1000)
sus max_result drip = find_max(large_array_1000)
sus min_result drip = find_min(large_array_1000)
assert_eq_int(max_result, 20)
assert_eq_int(min_result, 1)

fr fr ===== STRING PERFORMANCE =====

fr fr Test repeated string operations
sus base_string tea = "performance_test_"
sus repeated_string tea = repeat_string(base_string, 20)
sus concatenated tea = concat_strings(repeated_string, "_suffix")

fr fr Test string building performance
sus builder_result tea = build_string_four("part1_", "part2_", "part3_", "part4")
assert_eq_string(builder_result, "part1_part2_part3_part4")

fr fr ===== MATHEMATICAL COMPUTATION PERFORMANCE =====

fr fr Test intensive mathematical operations
sus factorial_10 drip = factorial(10)
assert_eq_int(factorial_10, 3628800)

fr fr Test recursive operations
sus fib_15 drip = fibonacci(15)
assert_eq_int(fib_15, 610)

fr fr Test power operations
sus power_result drip = power_int(2, 10)
assert_eq_int(power_result, 1024)

fr fr ===== COMPLEX ARRAY OPERATIONS =====

fr fr Create complex mixed array
sus complex_array []drip = [-10, -5, 0, 5, 10, 15, 20, 25, 30, 35]

fr fr Test comprehensive array analysis
sus positive_count drip = count_positive(complex_array)
sus negative_count drip = count_negative(complex_array)
sus zero_count drip = count_zeros(complex_array)

assert_eq_int(positive_count, 7)
assert_eq_int(negative_count, 2)
assert_eq_int(zero_count, 1)

fr fr Test search performance
sus contains_result lit = contains_value(complex_array, 25)
sus index_result drip = find_index(complex_array, 25)
assert_true(contains_result)
assert_eq_int(index_result, 7)

fr fr ===== STRING ARRAY PERFORMANCE =====

sus string_performance_array []tea = ["alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta"]
sus joined_result tea = join_string_array(string_performance_array, " | ")
sus contains_string lit = string_array_contains(string_performance_array, "gamma")
assert_true(contains_string)

fr fr ===== INTEGRATION PERFORMANCE =====

fr fr Complex operation combining multiple modules
sus math_results []drip = [
    abs_normie(-100),
    power_int(3, 4),
    factorial(6),
    fibonacci(10),
    gcd(48, 18),
    lcm(12, 8)
]

sus total_sum drip = sum_array(math_results)
sus max_math drip = find_max(math_results)
sus min_math drip = find_min(math_results)

vibez.spill("Performance test results:")
vibez.spill("Total sum:", total_sum)
vibez.spill("Max value:", max_math)
vibez.spill("Min value:", min_math)

print_test_summary()
