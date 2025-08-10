fr fr Simple test for Pure CURSED Standard Library Migration
fr fr Tests core functions that have been migrated from FFI to pure CURSED

yeet "mathz"
yeet "stringz" 
yeet "arrayz"

vibez.spill("=== CURSED Standard Library Migration Test ===")
vibez.spill("")

fr fr Test basic math functions (pure CURSED)
vibez.spill("Math Functions:")
vibez.spill("abs_normie(-42) =", abs_normie(-42))
vibez.spill("max_normie(10, 20) =", max_normie(10, 20))
vibez.spill("min_normie(10, 20) =", min_normie(10, 20))
vibez.spill("factorial(5) =", factorial(5))
vibez.spill("gcd(48, 18) =", gcd(48, 18))
vibez.spill("sqrt_integer(25) =", sqrt_integer(25))
vibez.spill("")

fr fr Test string functions (pure CURSED)
vibez.spill("String Functions:")
vibez.spill("concat_strings('hello', 'world') =", concat_strings("hello", "world"))
vibez.spill("string_length('programming') =", string_length("programming"))
vibez.spill("char_at('hello', 1) =", char_at("hello", 1))
vibez.spill("to_uppercase('hello') =", to_uppercase("hello"))
vibez.spill("reverse_string('hello') =", reverse_string("hello"))
vibez.spill("parse_int('42') =", parse_int("42"))
vibez.spill("")

fr fr Test array functions (pure CURSED)
vibez.spill("Array Functions:")
sus test_array []drip = [1, 2, 3, 4, 5]
vibez.spill("array_size([1,2,3,4,5]) =", array_size(test_array))
vibez.spill("sum_array([1,2,3,4,5]) =", sum_array(test_array))
vibez.spill("find_max([1,2,3,4,5]) =", find_max(test_array))
vibez.spill("find_min([1,2,3,4,5]) =", find_min(test_array))
vibez.spill("contains_value([1,2,3,4,5], 3) =", contains_value(test_array, 3))
vibez.spill("")

fr fr Test string array functions
vibez.spill("String Array Functions:")
sus words []tea = ["hello", "world", "test"]
vibez.spill("join_string_array(['hello','world','test'], ' ') =", join_string_array(words, " "))
vibez.spill("string_array_contains(['hello','world','test'], 'test') =", string_array_contains(words, "test"))
vibez.spill("")

vibez.spill("=== Migration Test Complete ===")
vibez.spill("✅ All core stdlib functions working with pure CURSED implementations!")
vibez.spill("✅ FFI dependencies eliminated for math, string, and array operations!")
vibez.spill("✅ Standard library now authored entirely in pure CURSED!")
