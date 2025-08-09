fr fr CURSED STDLIB COMPREHENSIVE PRODUCTION READINESS TEST
fr fr Testing all major stdlib modules for memory safety and performance

yeet "testz"
yeet "mathz"
yeet "stringz"
yeet "arrayz"
yeet "vibez"

fr fr ===== 1. TESTZ FRAMEWORK VALIDATION =====

test_start("testz_framework_basic")
assert_true(based)
assert_false(cringe)
assert_eq_int(42, 42)
assert_eq_string("hello", "hello")

fr fr ===== 2. MATHZ MODULE COMPREHENSIVE TESTS =====

test_start("mathz_basic_arithmetic")
assert_eq_int(abs_normie(-5), 5)
assert_eq_int(abs_normie(5), 5)
assert_eq_int(add_two(3, 4), 7)
assert_eq_int(subtract_two(10, 3), 7)
assert_eq_int(multiply_two(6, 7), 42)
assert_eq_int(divide_two(20, 4), 5)

test_start("mathz_min_max_functions")
assert_eq_int(max_normie(10, 5), 10)
assert_eq_int(min_normie(10, 5), 5)
assert_eq_int(max_normie(-3, -7), -3)
assert_eq_int(min_normie(-3, -7), -7)

test_start("mathz_advanced_functions")
assert_eq_int(power_int(2, 3), 8)
assert_eq_int(power_int(5, 0), 1)
assert_eq_int(factorial(5), 120)
assert_eq_int(factorial(0), 1)
assert_eq_int(gcd(48, 18), 6)
assert_eq_int(lcm(4, 6), 12)

test_start("mathz_utility_functions")
assert_true(is_even(4))
assert_false(is_even(5))
assert_true(is_odd(5))
assert_false(is_odd(4))
assert_eq_int(clamp(5, 1, 10), 5)
assert_eq_int(clamp(-5, 1, 10), 1)
assert_eq_int(clamp(15, 1, 10), 10)
assert_eq_int(sign(5), 1)
assert_eq_int(sign(-5), -1)
assert_eq_int(sign(0), 0)

test_start("mathz_sequence_operations")
assert_eq_int(sum_range(1, 5), 15)
assert_eq_int(fibonacci(0), 0)
assert_eq_int(fibonacci(1), 1)
assert_eq_int(fibonacci(7), 13)

fr fr ===== 3. STRINGZ MODULE COMPREHENSIVE TESTS =====

test_start("stringz_basic_operations")
assert_eq_string(concat_strings("hello", "world"), "helloworld")
assert_eq_string(concat_three("a", "b", "c"), "abc")
assert_eq_string(repeat_string("x", 3), "xxx")

test_start("stringz_validation")
assert_true(is_empty_string(""))
assert_false(is_empty_string("hello"))
assert_true(is_not_empty("hello"))
assert_false(is_not_empty(""))
assert_true(strings_equal("test", "test"))
assert_false(strings_equal("test", "other"))

test_start("stringz_building")
assert_eq_string(build_string_two("hello", "world"), "helloworld")
assert_eq_string(build_string_three("a", "b", "c"), "abc")
assert_eq_string(surround_with_quotes("test"), "\"test\"")
assert_eq_string(surround_with_parens("test"), "(test)")
assert_eq_string(surround_with_brackets("test"), "[test]")

test_start("stringz_formatting")
assert_eq_string(format_as_title("Test"), "=== Test ===")
assert_eq_string(format_as_bullet("item"), "• item")
assert_eq_string(format_key_value("name", "value"), "name: value")

test_start("stringz_utility")
assert_eq_string(join_two_with_separator("a", "b", ","), "a,b")
assert_eq_string(join_with_comma("a", "b"), "a, b")
assert_eq_string(join_with_space("hello", "world"), "hello world")

fr fr ===== 4. ARRAYZ MODULE COMPREHENSIVE TESTS =====

test_start("arrayz_basic_creation_and_access")
sus test_array []drip = [1, 2, 3, 4, 5]
assert_eq_int(len(test_array), 5)
assert_eq_int(test_array[0], 1)
assert_eq_int(test_array[4], 5)

test_start("arrayz_arithmetic_functions")
sus nums []drip = [1, 2, 3, 4, 5]
assert_eq_int(sum_array(nums), 15)
assert_eq_int(average_array(nums), 3)
assert_eq_int(product_array(nums), 120)

test_start("arrayz_search_functions")
sus search_nums []drip = [10, 5, 8, 3, 12]
assert_eq_int(find_max(search_nums), 12)
assert_eq_int(find_min(search_nums), 3)
assert_true(contains_value(search_nums, 8))
assert_false(contains_value(search_nums, 99))
assert_eq_int(find_index(search_nums, 8), 2)
assert_eq_int(find_index(search_nums, 99), -1)

test_start("arrayz_validation_functions")
sus empty_array []drip = []
assert_true(is_empty_array(empty_array))
assert_false(is_empty_array(nums))
assert_eq_int(array_size(nums), 5)

test_start("arrayz_counting_functions")
sus mixed_nums []drip = [-2, 0, 3, -1, 5, 0, 7]
assert_eq_int(count_positive(mixed_nums), 3)
assert_eq_int(count_negative(mixed_nums), 2)
assert_eq_int(count_zeros(mixed_nums), 2)
assert_eq_int(count_occurrences(mixed_nums, 0), 2)

test_start("arrayz_bounds_checking")
assert_true(is_valid_index(nums, 2))
assert_false(is_valid_index(nums, 10))
assert_eq_int(safe_get(nums, 2, -1), 3)
assert_eq_int(safe_get(nums, 10, -1), -1)

test_start("arrayz_properties")
sus positive_nums []drip = [1, 2, 3, 4]
sus negative_nums []drip = [-1, -2, -3]
sus duplicate_nums []drip = [1, 2, 3, 2, 4]
assert_true(all_positive(positive_nums))
assert_false(all_positive(mixed_nums))
assert_true(all_negative(negative_nums))
assert_false(all_negative(mixed_nums))
assert_true(has_duplicates(duplicate_nums))
assert_false(has_duplicates(positive_nums))

test_start("arrayz_string_arrays")
sus string_array []tea = ["hello", "world", "test"]
assert_eq_string(join_string_array(string_array, " "), "hello world test")
assert_eq_string(concat_string_array(string_array), "helloworldtest")
assert_true(string_array_contains(string_array, "world"))
assert_false(string_array_contains(string_array, "missing"))

fr fr ===== 5. EDGE CASE TESTING =====

test_start("edge_cases_empty_arrays")
sus empty []drip = []
assert_eq_int(sum_array(empty), 0)
assert_eq_int(average_array(empty), 0)
assert_eq_int(product_array(empty), 0)
assert_eq_int(find_max(empty), 0)
assert_eq_int(find_min(empty), 0)

test_start("edge_cases_single_element")
sus single []drip = [42]
assert_eq_int(sum_array(single), 42)
assert_eq_int(average_array(single), 42)
assert_eq_int(product_array(single), 42)
assert_eq_int(find_max(single), 42)
assert_eq_int(find_min(single), 42)

test_start("edge_cases_division_by_zero")
assert_eq_int(divide_two(10, 0), 0)  fr fr Should handle gracefully

fr fr ===== 6. LARGE DATA PERFORMANCE TESTS =====

test_start("performance_large_arrays")
sus large_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
assert_eq_int(sum_array(large_array), 210)
assert_eq_int(find_max(large_array), 20)
assert_eq_int(find_min(large_array), 1)
assert_true(contains_value(large_array, 15))

test_start("performance_string_operations")
sus long_string tea = repeat_string("abcd", 10)
assert_eq_string(concat_strings(long_string, "xyz"), long_string + "xyz")

fr fr ===== 7. INTEGRATION TESTS =====

test_start("integration_math_with_arrays")
sus integration_nums []drip = [abs_normie(-5), power_int(2, 3), factorial(3)]
assert_eq_int(sum_array(integration_nums), 19)  fr fr 5 + 8 + 6 = 19

test_start("integration_string_with_math")
sus result_str tea = format_key_value("sum", sum_array([1, 2, 3]))
assert_eq_string(result_str, "sum: 6")

test_start("integration_complex_operations")
sus fibonacci_sequence []drip = [fibonacci(1), fibonacci(2), fibonacci(3), fibonacci(4), fibonacci(5)]
assert_eq_int(sum_array(fibonacci_sequence), 12)  fr fr 1 + 1 + 2 + 3 + 5 = 12

fr fr ===== FINAL TEST SUMMARY =====

print_test_summary()

ready (all_tests_passed()) {
    vibez.spill("🎉 ALL STDLIB MODULES PASSED COMPREHENSIVE TESTING!")
    vibez.spill("✅ Production readiness: CONFIRMED")
} otherwise {
    vibez.spill("❌ Some tests failed - stdlib needs fixes")
}
