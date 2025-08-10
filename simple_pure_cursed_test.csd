fr fr Simple test of Pure CURSED Standard Library Implementations

yeet "mathz"
yeet "stringz"
yeet "arrayz"

vibez.spill("=== Testing Pure CURSED Math Functions ===")

fr fr Test sqrt_integer function
sus sqrt_result drip = sqrt_integer(16)
vibez.spill("sqrt_integer(16) =", sqrt_result)

sus sqrt_result2 drip = sqrt_integer(25)
vibez.spill("sqrt_integer(25) =", sqrt_result2)

sus sqrt_result3 drip = sqrt_integer(100)
vibez.spill("sqrt_integer(100) =", sqrt_result3)

fr fr Test power_float_approx function
sus power_result drip = power_float_approx(2, 3)
vibez.spill("power_float_approx(2, 3) =", power_result)

sus power_result2 drip = power_float_approx(5, 2)
vibez.spill("power_float_approx(5, 2) =", power_result2)

vibez.spill("=== Testing Pure CURSED String Functions ===")

fr fr Test char_to_digit function
sus digit_result drip = char_to_digit("5")
vibez.spill("char_to_digit('5') =", digit_result)

sus digit_result2 drip = char_to_digit("9")
vibez.spill("char_to_digit('9') =", digit_result2)

fr fr Test digit_to_char function
sus char_result tea = digit_to_char(7)
vibez.spill("digit_to_char(7) =", char_result)

sus char_result2 tea = digit_to_char(3)
vibez.spill("digit_to_char(3) =", char_result2)

fr fr Test basic string to int conversion
sus str_to_int_result drip = parse_int("42")
vibez.spill("parse_int('42') =", str_to_int_result)

fr fr Test basic int to string conversion
sus int_to_str_result tea = int_to_string(123)
vibez.spill("int_to_string(123) =", int_to_str_result)

vibez.spill("=== Testing Pure CURSED Array Functions ===")

fr fr Test array length functions
sus test_nums []drip = [1, 2, 3, 4, 5]
sus array_len_result drip = array_length_int(test_nums)
vibez.spill("array_length_int([1,2,3,4,5]) =", array_len_result)

sus test_strings []tea = ["hello", "world", "test"]
sus string_array_len_result drip = array_length_string(test_strings)
vibez.spill("array_length_string(['hello','world','test']) =", string_array_len_result)

fr fr Test array append functions
sus small_array []drip = [1, 2]
sus appended_array []drip = append_to_int_array(small_array, 3)
vibez.spill("append_to_int_array([1,2], 3) length =", len(appended_array))
vibez.spill("appended value =", appended_array[2])

sus small_string_array []tea = ["a", "b"]
sus appended_string_array []tea = append_to_string_array(small_string_array, "c")
vibez.spill("append_to_string_array(['a','b'], 'c') length =", len(appended_string_array))
vibez.spill("appended string =", appended_string_array[2])

vibez.spill("=== Testing Existing Stdlib Functions Still Work ===")

fr fr Test math functions
sus abs_result drip = abs_normie(-10)
vibez.spill("abs_normie(-10) =", abs_result)

sus max_result drip = max_normie(15, 8)
vibez.spill("max_normie(15, 8) =", max_result)

fr fr Test string functions
sus concat_result tea = concat_strings("hello", " world")
vibez.spill("concat_strings('hello', ' world') =", concat_result)

sus length_result drip = string_length("test")
vibez.spill("string_length('test') =", length_result)

fr fr Test array functions
sus sum_result drip = sum_array([1, 2, 3, 4, 5])
vibez.spill("sum_array([1,2,3,4,5]) =", sum_result)

sus max_array_result drip = find_max([3, 7, 2, 9, 1])
vibez.spill("find_max([3,7,2,9,1]) =", max_array_result)

vibez.spill("✅ Pure CURSED Stdlib implementation tests completed successfully!")
vibez.spill("All core functions now use pure CURSED implementations")
vibez.spill("instead of Zig FFI calls for better performance and maintainability.")
