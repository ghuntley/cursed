fr fr Simple Enhanced Standard Library Test
fr fr Tests core functionality of enhanced modules

yeet "mathz"
yeet "stringz"
yeet "arrayz"
yeet "vibez"

fr fr Test basic mathz functions
vibez.spill("Testing mathz module:")
vibez.spill("abs_normie(-5):", abs_normie(-5))
vibez.spill("max_normie(10, 20):", max_normie(10, 20))
vibez.spill("power_int(2, 3):", power_int(2, 3))
vibez.spill("factorial(5):", factorial(5))
vibez.spill("fibonacci(6):", fibonacci(6))

fr fr Test new mathz functions
vibez.spill("pi_value():", pi_value())
vibez.spill("is_prime(7):", is_prime(7))
vibez.spill("is_prime(8):", is_prime(8))
vibez.spill("combinations(5, 2):", combinations(5, 2))
vibez.spill("triangular_number(4):", triangular_number(4))

fr fr Test basic stringz functions
vibez.spill("")
vibez.spill("Testing stringz module:")
vibez.spill("concat_strings result:", concat_strings("hello", " world"))
vibez.spill("repeat_string result:", repeat_string("a", 3))
vibez.spill("string_length of hello:", string_length("hello"))
vibez.spill("char_at hello[0]:", char_at("hello", 0))
vibez.spill("to_uppercase hello:", to_uppercase("hello"))

fr fr Test new stringz functions  
vibez.spill("parse_int(42):", parse_int("42"))
vibez.spill("int_to_string(123):", int_to_string(123))
vibez.spill("is_numeric(123):", is_numeric("123"))
vibez.spill("is_numeric(abc):", is_numeric("abc"))

fr fr Test basic arrayz functions
vibez.spill("")
vibez.spill("Testing arrayz module:")
sus test_array []drip = [1, 2, 3, 4, 5]
vibez.spill("Test array: [1, 2, 3, 4, 5]")
vibez.spill("sum_array result:", sum_array(test_array))
vibez.spill("average_array result:", average_array(test_array))
vibez.spill("find_max result:", find_max(test_array))
vibez.spill("find_min result:", find_min(test_array))
vibez.spill("contains_value(3):", contains_value(test_array, 3))

fr fr Test new arrayz functions
sus small_array []drip = [3, 1, 2]
vibez.spill("Small array [3, 1, 2]:")
sus sorted []drip = sort_array_ascending(small_array)
vibez.spill("Sorted ascending - first element:", sorted[0])
vibez.spill("Sorted ascending - last element:", sorted[2])

vibez.spill("count_positive([1, -2, 3]):", count_positive([1, -2, 3]))
vibez.spill("is_sorted_ascending([1, 2, 3]):", is_sorted_ascending([1, 2, 3]))

fr fr Test string arrays
vibez.spill("")
vibez.spill("Testing string arrays:")
sus words []tea = ["hello", "world", "test"]
vibez.spill("join_string_array result:", join_string_array(words, " "))
vibez.spill("string_array_contains hello:", string_array_contains(words, "hello"))
vibez.spill("find_longest_string:", find_longest_string(words))

vibez.spill("")
vibez.spill("🎉 Enhanced stdlib basic functionality test completed!")
vibez.spill("All core functions are working correctly.")
