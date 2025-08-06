yeet "testz"
yeet "stringz"

fr fr Simple validation test for stringz module
test_start("Stringz Module Basic Validation")

fr fr Test basic string operations
vibez.spill("Testing basic stringz operations...")

fr fr Test length function
sus len_result normie = stringz.length("hello")
vibez.spill("String length of 'hello': " + len_result)
assert_eq_int(len_result, 5)

fr fr Test concatenation
sus concat_result tea = stringz.concat("hello", "world")
vibez.spill("Concatenation result: " + concat_result)
assert_eq_string(concat_result, "helloworld")

fr fr Test contains function
sus contains_result lit = stringz.contains("hello world", "world")
vibez.spill("Contains 'world' in 'hello world': " + contains_result)
assert_true(contains_result)

fr fr Test starts_with function
sus starts_result lit = stringz.starts_with("hello", "he")
vibez.spill("Starts with 'he': " + starts_result)
assert_true(starts_result)

fr fr Test case conversion
sus upper_result tea = stringz.to_upper("hello")
vibez.spill("Uppercase 'hello': " + upper_result)
assert_eq_string(upper_result, "HELLO")

sus lower_result tea = stringz.to_lower("WORLD")
vibez.spill("Lowercase 'WORLD': " + lower_result)
assert_eq_string(lower_result, "world")

fr fr Test character validation
sus alpha_result lit = stringz.is_alpha_char('a')
vibez.spill("Is 'a' alphabetic: " + alpha_result)
assert_true(alpha_result)

sus digit_result lit = stringz.is_digit_char('5')
vibez.spill("Is '5' digit: " + digit_result)
assert_true(digit_result)

vibez.spill("All basic stringz operations validated successfully!")

print_test_summary()
