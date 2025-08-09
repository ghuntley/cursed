fr fr CURSED STDLIB SIMPLE PERFORMANCE TEST

yeet "testz"
yeet "mathz"
yeet "stringz"
yeet "arrayz"

test_start("stdlib_performance")

fr fr ===== MATHEMATICAL OPERATIONS PERFORMANCE =====

sus math_result1 drip = factorial(10)
assert_eq_int(math_result1, 3628800)

sus math_result2 drip = fibonacci(15)
assert_eq_int(math_result2, 610)

sus math_result3 drip = power_int(2, 10)
assert_eq_int(math_result3, 1024)

fr fr ===== STRING OPERATIONS PERFORMANCE =====

sus string_result1 tea = repeat_string("test", 10)
sus string_result2 tea = concat_strings(string_result1, "_suffix")
sus string_result3 tea = build_string_four("a", "b", "c", "d")
assert_eq_string(string_result3, "abcd")

fr fr ===== ARRAY OPERATIONS PERFORMANCE =====

sus perf_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
sus array_sum drip = sum_array(perf_array)
sus array_max drip = find_max(perf_array)
sus array_min drip = find_min(perf_array)

assert_eq_int(array_sum, 210)
assert_eq_int(array_max, 20)
assert_eq_int(array_min, 1)

sus positive_count drip = count_positive(perf_array)
assert_eq_int(positive_count, 20)

fr fr ===== STRING ARRAY PERFORMANCE =====

sus str_array []tea = ["apple", "banana", "cherry", "date", "elderberry"]
sus joined_str tea = join_string_array(str_array, ", ")
sus contains_banana lit = string_array_contains(str_array, "banana")
assert_true(contains_banana)

print_test_summary()

vibez.spill("Performance test completed successfully!")
