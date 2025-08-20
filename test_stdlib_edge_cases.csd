fr fr Test edge cases and error handling
yeet "vibez"
yeet "mathz"
yeet "stringz"

fr fr Test division by zero handling
sus zero drip = 0
sus safe_division drip = mathz.divide_two(10, 2)
sus zero_division drip = mathz.divide_two(10, zero)

vibez.spill("Safe division:", stringz.int_to_string(safe_division))
vibez.spill("Division by zero:", stringz.int_to_string(zero_division))

fr fr Test absolute value with negative numbers
sus negative drip = -15
sus abs_result drip = mathz.abs_normie(negative)
vibez.spill("Absolute of", stringz.int_to_string(negative), "is", stringz.int_to_string(abs_result))

fr fr Test string operations
sus empty_str tea = ""
sus empty_len drip = stringz.string_length(empty_str)
sus normal_str tea = "Hello World"
sus normal_len drip = stringz.string_length(normal_str)

vibez.spill("Empty string length:", stringz.int_to_string(empty_len))
vibez.spill("Normal string length:", stringz.int_to_string(normal_len))

fr fr Test string concatenation
sus part1 tea = "Hello"
sus part2 tea = " World!"
sus concatenated tea = stringz.concat_strings(part1, part2)
vibez.spill("Concatenated:", concatenated)

vibez.spill("Edge case tests completed!")
