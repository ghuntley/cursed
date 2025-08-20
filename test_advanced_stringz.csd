fr fr Test more advanced stringz functions that should be implemented
yeet "stringz"
yeet "vibez"

fr fr Test some of the more complex string operations
sus test_str tea = "Hello World"
sus char_result tea = stringz.char_at(test_str, 0)
sus substr_result tea = stringz.substring(test_str, 0, 5)

vibez.spill("First character:", char_result)
vibez.spill("Substring:", substr_result)

fr fr Test string validation functions
sus numeric_str tea = "123"
sus is_num_result lit = stringz.is_numeric(numeric_str)
sus alpha_str tea = "hello"
sus is_alpha_result lit = stringz.is_alphabetic(alpha_str)

vibez.spill("Is '123' numeric:", is_num_result)
vibez.spill("Is 'hello' alphabetic:", is_alpha_result)
