fr fr Test comprehensive stdlib functionality
yeet "vibez"
yeet "mathz"  
yeet "stringz"

fr fr Test mathz module
sus a drip = 10
sus b drip = 5
sus sum drip = mathz.add_two(a, b)
sus difference drip = mathz.subtract_two(a, b)
sus product drip = mathz.multiply_two(a, b)
sus quotient drip = mathz.divide_two(a, b)
sus absolute drip = mathz.abs_normie(-7)
sus maximum drip = mathz.max_normie(a, b)
sus minimum drip = mathz.min_normie(a, b)

vibez.spill("=== MATHZ MODULE TESTS ===")
vibez.spill("Sum:", stringz.int_to_string(sum))
vibez.spill("Difference:", stringz.int_to_string(difference))
vibez.spill("Product:", stringz.int_to_string(product))
vibez.spill("Quotient:", stringz.int_to_string(quotient))
vibez.spill("Absolute:", stringz.int_to_string(absolute))
vibez.spill("Maximum:", stringz.int_to_string(maximum))
vibez.spill("Minimum:", stringz.int_to_string(minimum))

fr fr Test stringz module
sus greeting tea = "Hello"
sus target tea = "CURSED"
sus combined tea = stringz.concat_strings(greeting, target)
sus number tea = stringz.int_to_string(42)
sus length drip = stringz.string_length(greeting)

vibez.spill("=== STRINGZ MODULE TESTS ===")
vibez.spill("Greeting:", greeting)
vibez.spill("Target:", target)
vibez.spill("Combined:", combined)
vibez.spill("Number as string:", number)
vibez.spill("Length of greeting:", stringz.int_to_string(length))

fr fr Test vibez module
vibez.spill("=== VIBEZ MODULE TESTS ===")
vibez.spill("Single argument test")
vibez.spill("Multiple", "arguments", "test")
vibez.spill("Mixed types:", stringz.int_to_string(123), "and strings")

vibez.spill("All stdlib tests completed successfully!")
