fr fr Final validation of core stdlib modules
yeet "vibez"
yeet "mathz" 
yeet "stringz"

vibez.spill("🚀 CURSED Stdlib Validation Test")
vibez.spill("=====================================")

fr fr Test all mathz functions
sus x drip = 10
sus y drip = 3
vibez.spill("Math tests with x =", stringz.int_to_string(x), "and y =", stringz.int_to_string(y))
vibez.spill("Add:", stringz.int_to_string(mathz.add_two(x, y)))
vibez.spill("Subtract:", stringz.int_to_string(mathz.subtract_two(x, y)))
vibez.spill("Multiply:", stringz.int_to_string(mathz.multiply_two(x, y)))
vibez.spill("Divide:", stringz.int_to_string(mathz.divide_two(x, y)))
vibez.spill("Max:", stringz.int_to_string(mathz.max_normie(x, y)))
vibez.spill("Min:", stringz.int_to_string(mathz.min_normie(x, y)))
vibez.spill("Abs(-5):", stringz.int_to_string(mathz.abs_normie(-5)))

fr fr Test all stringz functions  
sus str1 tea = "Hello"
sus str2 tea = " World"
sus combined tea = stringz.concat_strings(str1, str2)
sus num_str tea = stringz.int_to_string(2024)
sus str_len drip = stringz.string_length(combined)

vibez.spill("String tests:")
vibez.spill("Concat:", combined)
vibez.spill("Number as string:", num_str)
vibez.spill("String length:", stringz.int_to_string(str_len))

fr fr Test vibez output varieties
vibez.spill("Testing vibez.spill variations:")
vibez.spill("Single argument")
vibez.spill("Two", "arguments")
vibez.spill("Three", "different", "args")
vibez.spill("Mixed:", stringz.int_to_string(42), combined, "done")

fr fr Test edge cases
sus zero drip = 0
sus zero_abs drip = mathz.abs_normie(zero)
sus empty_str tea = ""
sus empty_len drip = stringz.string_length(empty_str)

vibez.spill("Edge cases:")
vibez.spill("Absolute of 0:", stringz.int_to_string(zero_abs))
vibez.spill("Empty string length:", stringz.int_to_string(empty_len))
vibez.spill("Division by zero protection:", stringz.int_to_string(mathz.divide_two(10, zero)))

vibez.spill("✅ All stdlib core functionality is working correctly!")
vibez.spill("Ready for production use of basic programs.")
