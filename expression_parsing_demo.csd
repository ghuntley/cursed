# Complex Expression Parsing Fixes - Demonstration
yeet "vibez"

vibez.spill("=== COMPLEX EXPRESSION PARSING FIXES DEMO ===")

# Test 1: Comment removal in expressions (FIXED)
vibez.spill("Test 1: Comments in expressions")
sus result_with_comment drip = 5 + 3  # This comment should be stripped
vibez.spill("Result with comment:", result_with_comment)

# Test 2: Nested parentheses (Working)
vibez.spill("Test 2: Nested parentheses")
sus nested_parens drip = ((2 + 3) * (4 + 1))
vibez.spill("Nested parentheses result:", nested_parens)

# Test 3: Operator precedence (Working)
vibez.spill("Test 3: Operator precedence")
sus precedence_test drip = 2 + 3 * 4 - 1  # Should be 2 + (3 * 4) - 1 = 13
vibez.spill("Operator precedence (2 + 3 * 4 - 1):", precedence_test)

# Test 4: Complex arithmetic expressions (Working)
vibez.spill("Test 4: Complex arithmetic")
sus complex_math drip = (10 + 5) * 2 - (8 / 4) + 3
vibez.spill("Complex math:", complex_math)

# Test 5: Boolean logic expressions (Working)
vibez.spill("Test 5: Boolean logic")
sus a lit = based
sus b lit = cringe
sus logical_result lit = (a and (not b)) or (b and (not a))
vibez.spill("Logical expression result:", logical_result)

# Test 6: String concatenation with expressions (Working)
vibez.spill("Test 6: String concatenation")
sus num1 drip = 5
sus num2 drip = 3
sus concat_result tea = "Sum: " ++ (num1 + num2) ++ " Product: " ++ (num1 * num2)
vibez.spill("String concatenation:", concat_result)

# Test 7: Array operations with complex indexing (Working)
vibez.spill("Test 7: Array operations")
sus numbers []drip = [10, 20, 30, 40, 50]
sus index_calc drip = 1 + 1  # Dynamic index calculation
sus array_result drip = numbers[index_calc] + numbers[0]
vibez.spill("Array operation result:", array_result)

vibez.spill("=== All expression parsing tests completed ===")
