fr fr Test of working essential functions without complex modules

fr fr Test core runtime functions that work
sus text tea = "hello world"
sus len drip = len_str(text)
vibez.spill("String length:", len)

fr fr Test basic math
sus abs_val drip = abs_normie(-42)
vibez.spill("Absolute value:", abs_val)

fr fr Test arrays
sus numbers [drip] = [1, 2, 3, 4, 5]
sus arr_len drip = len(numbers)
vibez.spill("Array length:", arr_len)

sus first_num drip = numbers[0]
vibez.spill("First number:", first_num)

sus last_num drip = numbers[4]
vibez.spill("Last number:", last_num)

fr fr Test string indexing
sus first_char drip = runtime_char_at_string("hello", 0)
vibez.spill("First character code:", first_char)

fr fr Test string concatenation
sus greeting tea = "Hello" + " " + "World"
vibez.spill("Concatenated:", greeting)

fr fr Test basic arithmetic
sus sum drip = 10 + 5
vibez.spill("Sum:", sum)

sus product drip = 3 * 4
vibez.spill("Product:", product)

sus division drip = 15 / 3
vibez.spill("Division:", division)

fr fr Test boolean operations
sus is_true lit = based
sus is_false lit = cringe
vibez.spill("Boolean true:", is_true)
vibez.spill("Boolean false:", is_false)

vibez.spill("Essential operations test completed!")
