fr fr Test the essential functions module

yeet "essential_funcs"

fr fr Test string functions
sus text tea = "Hello World"
sus len drip = str_length(text)
vibez.spill("String length:", len)

sus upper tea = str_upper("hello")
vibez.spill("Uppercase:", upper)

sus lower tea = str_lower("HELLO")
vibez.spill("Lowercase:", lower)

sus has_world lit = str_contains("hello world", "world")
vibez.spill("Contains world:", has_world)

fr fr Test array functions
sus arr [tea] = ["apple", "banana", "cherry"]
sus arr_len drip = arr_length(arr)
vibez.spill("Array length:", arr_len)

sus first tea = arr_get(arr, 0)
vibez.spill("First element:", first)

sus has_apple lit = arr_contains(arr, "apple")
vibez.spill("Contains apple:", has_apple)

fr fr Test math functions
sus abs_val drip = math_abs(-42)
vibez.spill("Absolute value:", abs_val)

sus max_val drip = math_max(10, 5)
vibez.spill("Max value:", max_val)

sus min_val drip = math_min(10, 5)
vibez.spill("Min value:", min_val)

sus sqrt_val meal = math_sqrt(16.0)
vibez.spill("Square root:", sqrt_val)

sus power_val meal = math_power(2.0, 3)
vibez.spill("2^3:", power_val)

vibez.spill("Essential functions test completed!")
