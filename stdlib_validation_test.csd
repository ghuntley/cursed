fr fr Comprehensive Standard Library Validation Test
fr fr Tests all core modules: vibez, mathz, stringz, arrayz, testz

yeet "testz"  
yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "arrayz"

fr fr Start test suite
testz.run_test_suite("CURSED Standard Library Validation")
testz.test_section("🧪 Core Module Testing")

fr fr ===== VIBEZ MODULE TESTS =====
testz.test_section("📢 VIBEZ Module Tests")

testz.test_start("vibez_basic_output")
vibez.spill("Hello from vibez!")
vibez.spill_two("Part 1", "Part 2")
vibez.spillln("Line with newline")
testz.assert_true(based)  fr fr Passed if no crashes

testz.test_start("vibez_formatting")
vibez.print_header("Test Header")
vibez.print_separator()
vibez.print_success("Success message")
vibez.print_error("Error message")
vibez.print_warning("Warning message")
vibez.print_info("Info message")
testz.assert_true(based)

fr fr ===== MATHZ MODULE TESTS =====
testz.test_section("🔢 MATHZ Module Tests")

testz.test_start("mathz_basic_arithmetic")
testz.assert_eq_int(mathz.abs_normie(-5), 5)
testz.assert_eq_int(mathz.abs_normie(3), 3)
testz.assert_eq_int(mathz.max_normie(10, 5), 10)
testz.assert_eq_int(mathz.min_normie(10, 5), 5)
testz.assert_eq_int(mathz.add_two(3, 4), 7)
testz.assert_eq_int(mathz.multiply_two(6, 7), 42)

testz.test_start("mathz_advanced_functions")
testz.assert_eq_int(mathz.power_int(2, 3), 8)
testz.assert_eq_int(mathz.factorial(5), 120)
testz.assert_eq_int(mathz.gcd(12, 8), 4)
testz.assert_true(mathz.is_even(4))
testz.assert_true(mathz.is_odd(5))

testz.test_start("mathz_sequences")
testz.assert_eq_int(mathz.sum_range(1, 10), 55)
testz.assert_eq_int(mathz.fibonacci(7), 13)
testz.assert_true(mathz.is_prime(7))
testz.assert_false(mathz.is_prime(8))

fr fr ===== STRINGZ MODULE TESTS =====
testz.test_section("📝 STRINGZ Module Tests")

testz.test_start("stringz_basic_operations")
testz.assert_eq_string(stringz.concat_strings("hello", "world"), "helloworld")
testz.assert_eq_string(stringz.repeat_string("x", 3), "xxx")
testz.assert_true(stringz.is_empty_string(""))
testz.assert_false(stringz.is_empty_string("test"))
testz.assert_true(stringz.strings_equal("abc", "abc"))

testz.test_start("stringz_formatting")
testz.assert_eq_string(stringz.format_as_title("Test"), "=== Test ===")
testz.assert_eq_string(stringz.surround_with_quotes("text"), "\"text\"")
testz.assert_eq_string(stringz.join_with_comma("a", "b"), "a, b")

testz.test_start("stringz_transformations")
testz.assert_eq_string(stringz.to_uppercase("hello"), "HELLO")
testz.assert_eq_string(stringz.to_lowercase("WORLD"), "world")
testz.assert_eq_string(stringz.reverse_string("abc"), "cba")

testz.test_start("stringz_parsing")
testz.assert_eq_int(stringz.parse_int("42"), 42)
testz.assert_eq_string(stringz.int_to_string(123), "123")
testz.assert_true(stringz.is_numeric("456"))
testz.assert_false(stringz.is_numeric("abc"))

fr fr ===== ARRAYZ MODULE TESTS =====
testz.test_section("📊 ARRAYZ Module Tests")

testz.test_start("arrayz_basic_operations")
sus test_array []drip = [1, 2, 3, 4, 5]
testz.assert_eq_int(arrayz.sum_array(test_array), 15)
testz.assert_eq_int(arrayz.average_array(test_array), 3)
testz.assert_eq_int(arrayz.find_max(test_array), 5)
testz.assert_eq_int(arrayz.find_min(test_array), 1)

testz.test_start("arrayz_search_operations")
testz.assert_true(arrayz.contains_value(test_array, 3))
testz.assert_false(arrayz.contains_value(test_array, 10))
testz.assert_eq_int(arrayz.find_index(test_array, 4), 3)
testz.assert_eq_int(arrayz.find_index(test_array, 99), -1)

testz.test_start("arrayz_validation")
testz.assert_false(arrayz.is_empty_array(test_array))
testz.assert_eq_int(arrayz.array_size(test_array), 5)
testz.assert_true(arrayz.is_valid_index(test_array, 2))
testz.assert_false(arrayz.is_valid_index(test_array, 10))

testz.test_start("arrayz_counting")
sus mixed_array []drip = [-2, -1, 0, 1, 2]
testz.assert_eq_int(arrayz.count_positive(mixed_array), 2)
testz.assert_eq_int(arrayz.count_negative(mixed_array), 2)
testz.assert_eq_int(arrayz.count_zeros(mixed_array), 1)

testz.test_start("arrayz_string_operations")
sus string_array []tea = ["hello", "world", "test"]
testz.assert_eq_string(arrayz.join_string_array(string_array, " "), "hello world test")
testz.assert_true(arrayz.string_array_contains(string_array, "world"))
testz.assert_false(arrayz.string_array_contains(string_array, "missing"))

fr fr ===== TESTZ MODULE TESTS =====
testz.test_section("🧪 TESTZ Module Tests")

testz.test_start("testz_self_validation")
fr fr Testing testz functions themselves
testz.assert_true(based)
testz.assert_false(cringe)
testz.assert_eq_int(42, 42)
testz.assert_eq_string("test", "test")
testz.assert_not_eq_int(1, 2)
testz.assert_not_eq_string("a", "b")

fr fr ===== INTEGRATION TESTS =====
testz.test_section("🔗 Integration Tests")

testz.test_start("cross_module_integration")
fr fr Test using multiple modules together
sus numbers []drip = [10, 20, 30]
sus sum drip = arrayz.sum_array(numbers)
sus sum_str tea = stringz.int_to_string(sum)
sus formatted tea = stringz.format_as_title(sum_str)
vibez.spill("Formatted sum:", formatted)
testz.assert_eq_int(sum, 60)
testz.assert_eq_string(sum_str, "60")

testz.test_start("math_string_integration")
sus result drip = mathz.power_int(2, 4)
sus is_even lit = mathz.is_even(result)
sus result_str tea = stringz.int_to_string(result)
testz.assert_eq_int(result, 16)
testz.assert_true(is_even)
testz.assert_eq_string(result_str, "16")

testz.test_start("array_string_integration")
sus words []tea = ["cursed", "is", "awesome"]
sus joined tea = arrayz.join_string_array(words, " ")
sus upper_joined tea = stringz.to_uppercase(joined)
testz.assert_eq_string(joined, "cursed is awesome")
testz.assert_eq_string(upper_joined, "CURSED IS AWESOME")

fr fr ===== ERROR HANDLING TESTS =====
testz.test_section("⚠️ Error Handling Tests")

testz.test_start("division_by_zero_handling")
sus safe_div drip = mathz.divide_two(10, 0)
testz.assert_eq_int(safe_div, 0)  fr fr Should return 0 for division by zero

testz.test_start("empty_array_handling")
sus empty_array []drip = []
testz.assert_true(arrayz.is_empty_array(empty_array))
testz.assert_eq_int(arrayz.sum_array(empty_array), 0)
testz.assert_eq_int(arrayz.find_max(empty_array), 0)

testz.test_start("invalid_string_parsing")
sus invalid_num drip = stringz.parse_int("invalid")
testz.assert_eq_int(invalid_num, 0)  fr fr Should return 0 for invalid input

fr fr ===== FINAL SUMMARY =====
testz.print_test_summary()

lowkey (testz.all_tests_passed()) {
    vibez.spill("")
    vibez.spill("🎉🎉🎉 ALL STANDARD LIBRARY TESTS PASSED! 🎉🎉🎉")
    vibez.spill("✅ vibez: I/O operations working correctly")
    vibez.spill("✅ mathz: Mathematical functions working correctly") 
    vibez.spill("✅ stringz: String operations working correctly")
    vibez.spill("✅ arrayz: Array operations working correctly")
    vibez.spill("✅ testz: Testing framework working correctly")
    vibez.spill("✅ Integration: Cross-module functionality working")
} otherwise {
    vibez.spill("")
    vibez.spill("❌ SOME TESTS FAILED - CHECK OUTPUT ABOVE")
    vibez.spill("Standard library needs investigation")
}

vibez.spill("")
vibez.spill("Standard library validation complete.")
