fr fr Test Pure CURSED Standard Library Implementations
fr fr This tests the new pure CURSED implementations that replace Zig FFI functions

yeet "mathz"
yeet "stringz"
yeet "arrayz"
yeet "testz"

test_start("Pure CURSED Stdlib Tests")

fr fr ===== TEST MATH FUNCTIONS =====
vibez.spill("Testing Pure CURSED Math Functions...")

fr fr Test sqrt_integer function
sus sqrt_result drip = sqrt_integer(16)
assert_eq_int(sqrt_result, 4)
vibez.spill("sqrt_integer(16) =", sqrt_result)

sus sqrt_result2 drip = sqrt_integer(25)
assert_eq_int(sqrt_result2, 5)
vibez.spill("sqrt_integer(25) =", sqrt_result2)

sus sqrt_result3 drip = sqrt_integer(100)
assert_eq_int(sqrt_result3, 10)
vibez.spill("sqrt_integer(100) =", sqrt_result3)

fr fr Test power_float_approx function
sus power_result drip = power_float_approx(2, 3)
assert_eq_int(power_result, 8)
vibez.spill("power_float_approx(2, 3) =", power_result)

sus power_result2 drip = power_float_approx(5, 2)
assert_eq_int(power_result2, 25)
vibez.spill("power_float_approx(5, 2) =", power_result2)

fr fr ===== TEST STRING FUNCTIONS =====
vibez.spill("Testing Pure CURSED String Functions...")

fr fr Test char_to_digit function
sus digit_result drip = char_to_digit("5")
assert_eq_int(digit_result, 5)
vibez.spill("char_to_digit('5') =", digit_result)

sus digit_result2 drip = char_to_digit("9")
assert_eq_int(digit_result2, 9)
vibez.spill("char_to_digit('9') =", digit_result2)

fr fr Test digit_to_char function
sus char_result tea = digit_to_char(7)
assert_eq_string(char_result, "7")
vibez.spill("digit_to_char(7) =", char_result)

sus char_result2 tea = digit_to_char(3)
assert_eq_string(char_result2, "3")
vibez.spill("digit_to_char(3) =", char_result2)

fr fr Test basic string to int conversion
sus str_to_int_result drip = parse_int("42")
assert_eq_int(str_to_int_result, 42)
vibez.spill("parse_int('42') =", str_to_int_result)

fr fr Test basic int to string conversion
sus int_to_str_result tea = int_to_string(123)
assert_eq_string(int_to_str_result, "123")
vibez.spill("int_to_string(123) =", int_to_str_result)

fr fr ===== TEST ARRAY FUNCTIONS =====
vibez.spill("Testing Pure CURSED Array Functions...")

fr fr Test array length functions
sus test_nums []drip = [1, 2, 3, 4, 5]
sus array_len_result drip = array_length_int(test_nums)
assert_eq_int(array_len_result, 5)
vibez.spill("array_length_int([1,2,3,4,5]) =", array_len_result)

sus test_strings []tea = ["hello", "world", "test"]
sus string_array_len_result drip = array_length_string(test_strings)
assert_eq_int(string_array_len_result, 3)
vibez.spill("array_length_string(['hello','world','test']) =", string_array_len_result)

fr fr Test array append functions
sus small_array []drip = [1, 2]
sus appended_array []drip = append_to_int_array(small_array, 3)
assert_eq_int(len(appended_array), 3)
assert_eq_int(appended_array[2], 3)
vibez.spill("append_to_int_array([1,2], 3) length =", len(appended_array))
vibez.spill("appended value =", appended_array[2])

sus small_string_array []tea = ["a", "b"]
sus appended_string_array []tea = append_to_string_array(small_string_array, "c")
assert_eq_int(len(appended_string_array), 3)
assert_eq_string(appended_string_array[2], "c")
vibez.spill("append_to_string_array(['a','b'], 'c') length =", len(appended_string_array))
vibez.spill("appended string =", appended_string_array[2])

fr fr Test array copy functions
sus original_nums []drip = [10, 20, 30]
sus copied_nums []drip = copy_int_array(original_nums)
assert_eq_int(len(copied_nums), 3)
assert_eq_int(copied_nums[1], 20)
vibez.spill("copy_int_array([10,20,30]) length =", len(copied_nums))
vibez.spill("copied value [1] =", copied_nums[1])

sus original_strings []tea = ["x", "y", "z"]
sus copied_strings []tea = copy_string_array(original_strings)
assert_eq_int(len(copied_strings), 3)
assert_eq_string(copied_strings[1], "y")
vibez.spill("copy_string_array(['x','y','z']) length =", len(copied_strings))
vibez.spill("copied string [1] =", copied_strings[1])

fr fr ===== TEST EXISTING STDLIB FUNCTIONS =====
vibez.spill("Testing existing stdlib functions still work...")

fr fr Test math functions
sus abs_result drip = abs_normie(-10)
assert_eq_int(abs_result, 10)
vibez.spill("abs_normie(-10) =", abs_result)

sus max_result drip = max_normie(15, 8)
assert_eq_int(max_result, 15)
vibez.spill("max_normie(15, 8) =", max_result)

fr fr Test string functions
sus concat_result tea = concat_strings("hello", " world")
assert_eq_string(concat_result, "hello world")
vibez.spill("concat_strings('hello', ' world') =", concat_result)

sus length_result drip = string_length("test")
assert_eq_int(length_result, 4)
vibez.spill("string_length('test') =", length_result)

fr fr Test array functions
sus sum_result drip = sum_array([1, 2, 3, 4, 5])
assert_eq_int(sum_result, 15)
vibez.spill("sum_array([1,2,3,4,5]) =", sum_result)

sus max_array_result drip = find_max([3, 7, 2, 9, 1])
assert_eq_int(max_array_result, 9)
vibez.spill("find_max([3,7,2,9,1]) =", max_array_result)

print_test_summary()
vibez.spill("✅ Pure CURSED Stdlib implementation tests completed!")
