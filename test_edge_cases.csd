yeet "testz"

test_start("Edge Cases for Expression Evaluation")

# Test 1: Deeply nested parentheses
vibez.spill("=== Test 1: Nested Parentheses ===")
sus nested drip = ((2 + 3) * (4 + 1)) / 5
vibez.spill("((2 + 3) * (4 + 1)) / 5 =", nested, "(should be 5)")

# Test 2: Multiple unary operators
vibez.spill("=== Test 2: Multiple Unary Operators ===")
sus double_neg drip = -(-10)
vibez.spill("-(-10) =", double_neg, "(should be 10)")

# Test 3: Division by zero handling 
vibez.spill("=== Test 3: Division Tests ===")
sus safe_div drip = 10 / 2
vibez.spill("10 / 2 =", safe_div, "(should be 5)")

# Test 4: Modulus operations
vibez.spill("=== Test 4: Modulus Operations ===")
sus mod_test drip = 17 % 5  
vibez.spill("17 % 5 =", mod_test, "(should be 2)")

sus mod_float normie = 7.5 % 2.5
vibez.spill("7.5 % 2.5 =", mod_float, "(should be 0.0)")

# Test 5: Very long expression chain
vibez.spill("=== Test 5: Long Expression Chain ===")
sus long_expr drip = 1 + 2 * 3 - 4 / 2 + 5 % 3
vibez.spill("1 + 2 * 3 - 4 / 2 + 5 % 3 =", long_expr, "(should be 7)")

# Test 6: Mixed operations with precedence
vibez.spill("=== Test 6: Mixed Operations with Precedence ===")
sus mixed_prec normie = 10.0 / 2 + 3 * 4 - 1
vibez.spill("10.0 / 2 + 3 * 4 - 1 =", mixed_prec, "(should be 16.0)")

# Test 7: Comparison chains (not chained, but multiple comparisons)
vibez.spill("=== Test 7: Multiple Comparisons ===")
sus comp1 lit = 5 > 3
sus comp2 lit = 10 <= 10
sus comp3 lit = 7.5 != 7.0
vibez.spill("5 > 3 =", comp1, "(should be true)")
vibez.spill("10 <= 10 =", comp2, "(should be true)")
vibez.spill("7.5 != 7.0 =", comp3, "(should be true)")

# Test 8: Zero operations
vibez.spill("=== Test 8: Zero Operations ===")
sus zero_add drip = 0 + 42
sus zero_mul drip = 0 * 999
sus add_zero drip = 42 + 0
vibez.spill("0 + 42 =", zero_add, "(should be 42)")
vibez.spill("0 * 999 =", zero_mul, "(should be 0)")
vibez.spill("42 + 0 =", add_zero, "(should be 42)")

vibez.spill("=== All Edge Cases Completed ===")
print_test_summary()
