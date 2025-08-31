fr fr Test multiple modules together in compiled mode

yeet "mathz"
yeet "stringz"

vibez.spill("Testing mixed module usage:")

sus num1 drip = 8
sus num2 drip = 3

sus sum drip = mathz.add_two(num1, num2)
vibez.spill("8 + 3 =")
vibez.spill(sum)

sum = sum + 1
vibez.spill("sum + 1 =")
vibez.spill(sum)

sus product drip = mathz.multiply_two(sum, 2)
vibez.spill("sum * 2 =")
vibez.spill(product)

sus is_even_result lit = mathz.is_even(product)
vibez.spill("Is product even?")
vibez.spill(is_even_result)

sus text1 tea = "Hello"
sus text2 tea = "World"

sus combined tea = stringz.concat(text1, text2)
vibez.spill("Concatenated string:")
vibez.spill(combined)

sus length_result drip = stringz.length(text1)
vibez.spill("Length of 'Hello':")
vibez.spill(length_result)

sus max_val drip = mathz.max_normie(sum, product)
vibez.spill("Max of sum and product:")
vibez.spill(max_val)

sus final_result drip = mathz.abs_normie(-max_val)
vibez.spill("Absolute value of negative max:")
vibez.spill(final_result)
