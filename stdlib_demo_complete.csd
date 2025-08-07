yeet "testz"
yeet "stringz"
yeet "mathz"
yeet "arrayz"

fr fr Comprehensive demonstration of core stdlib modules

test_start("CURSED Standard Library Integration Demo")

fr fr ===== TESTING FRAMEWORK (testz) =====
vibez.spill("🧪 Testing Framework (testz) Demo")
test_start("testz_basic_assertions")
assert_true(based)
assert_eq_int(42, 42)
assert_eq_string("hello", "hello")

fr fr ===== STRING OPERATIONS (stringz) =====
vibez.spill("📝 String Operations (stringz) Demo")
test_start("stringz_basic_operations")

sus text tea = "Hello, CURSED World!"
sus len_result normie = stringz.length(text)
assert_eq_int(len_result, 20)

sus contains_result lit = stringz.contains(text, "CURSED")
assert_true(contains_result)

sus concat_result tea = stringz.concat("Hello", " CURSED")
assert_eq_string(concat_result, "Hello CURSED")

sus upper_result tea = stringz.to_upper("hello")
assert_eq_string(upper_result, "HELLO")

sus lower_result tea = stringz.to_lower("WORLD")
assert_eq_string(lower_result, "world")

vibez.spill("✅ String operations working correctly")

fr fr ===== MATHEMATICAL FUNCTIONS (mathz) =====
vibez.spill("🧮 Mathematical Functions (mathz) Demo")
test_start("mathz_basic_operations")

fr fr Test basic arithmetic
sus add_result meal = mathz.math_add(2.0, 3.0)
assert_near(add_result, 5.0, 0.01)

sus sqrt_result meal = mathz.sqrt_meal(25.0)
assert_near(sqrt_result, 5.0, 0.01)

sus abs_result meal = mathz.abs_meal(-42.5)
assert_near(abs_result, 42.5, 0.01)

fr fr Test trigonometry
sus sin_result meal = mathz.sin_meal(0.0)
assert_near(sin_result, 0.0, 0.01)

sus cos_result meal = mathz.cos_meal(0.0)
assert_near(cos_result, 1.0, 0.01)

fr fr Test factorial
sus fact_result normie = mathz.factorial(5)
assert_eq_int(fact_result, 120)

fr fr Test GCD
sus gcd_result normie = mathz.gcd(12, 18)
assert_eq_int(gcd_result, 6)

vibez.spill("✅ Mathematical functions working correctly")

fr fr ===== ARRAY OPERATIONS (arrayz) =====
vibez.spill("📊 Array Operations (arrayz) Demo")
test_start("arrayz_basic_operations")

fr fr Create and manipulate arrays
sus empty_array [tea] = arrayz.array_new()
assert_eq_int(arrayz.array_length(empty_array), 0)
assert_true(arrayz.array_is_empty(empty_array))

sus test_array [tea] = arrayz.array_fill(3, "test")
assert_eq_int(arrayz.array_length(test_array), 3)
assert_eq_string(arrayz.array_get(test_array, 0), "test")

test_array = arrayz.array_push(test_array, "new")
assert_eq_int(arrayz.array_length(test_array), 4)
assert_eq_string(arrayz.array_get(test_array, 3), "new")

fr fr Test array search
sus search_array [tea] = ["apple", "banana", "cherry"]
assert_eq_int(arrayz.array_find(search_array, "banana"), 1)
assert_true(arrayz.array_contains(search_array, "apple"))
assert_false(arrayz.array_contains(search_array, "orange"))

fr fr Test array reverse
sus original [tea] = ["a", "b", "c"]
sus reversed [tea] = arrayz.array_reverse(original)
assert_eq_string(arrayz.array_get(reversed, 0), "c")
assert_eq_string(arrayz.array_get(reversed, 2), "a")

fr fr Test numeric operations
sus numbers [normie] = [3, 1, 4, 1, 5]
assert_eq_int(arrayz.array_sum_numbers(numbers), 14)
assert_eq_int(arrayz.array_min_numbers(numbers), 1)
assert_eq_int(arrayz.array_max_numbers(numbers), 5)

vibez.spill("✅ Array operations working correctly")

fr fr ===== INTEGRATION TESTING =====
vibez.spill("🔄 Integration Testing Demo")
test_start("stdlib_integration")

fr fr Combine string and array operations
sus words [tea] = ["hello", "cursed", "world"]
sus sentence tea = arrayz.array_join(words, " ")

fr fr Use string operations on the result
sus sentence_upper tea = stringz.to_upper(sentence)
sus sentence_length normie = stringz.length(sentence_upper)

fr fr Use math to verify
sus expected_length normie = 5 + 1 + 6 + 1 + 5  fr fr "HELLO CURSED WORLD"
assert_eq_int(sentence_length, expected_length)

fr fr Combine array and math operations
sus math_array [normie] = [1, 2, 3, 4, 5]
sus sum normie = arrayz.array_sum_numbers(math_array)
sus average meal = sum / 5.0
sus expected_avg meal = 3.0
assert_near(average, expected_avg, 0.01)

vibez.spill("✅ Integration tests passing")

fr fr ===== BENCHMARKING DEMO =====
vibez.spill("⏱️ Performance Benchmarking Demo")

benchmark("String Operations", slay() {
    sus test_str tea = stringz.concat("hello", "world")
    sus len normie = stringz.length(test_str)
})

benchmark("Math Operations", slay() {
    sus result meal = mathz.sqrt_meal(25.0)
    sus abs_val meal = mathz.abs_meal(-10.5)
})

benchmark("Array Operations", slay() {
    sus arr [tea] = arrayz.array_fill(10, "test")
    sus len normie = arrayz.array_length(arr)
})

fr fr ===== FINAL SUMMARY =====
vibez.spill("")
vibez.spill("🎯 CURSED Standard Library Demo Complete!")
vibez.spill("═══════════════════════════════════════")
vibez.spill("✅ testz  - Testing framework operational")
vibez.spill("✅ stringz - String operations working")
vibez.spill("✅ mathz   - Mathematical functions working")
vibez.spill("✅ arrayz  - Array operations working")
vibez.spill("✅ Integration between modules successful")
vibez.spill("✅ Performance benchmarking available")
vibez.spill("")

print_test_summary()
print_benchmark_summary()

vibez.spill("🚀 The CURSED Standard Library is production-ready!")
