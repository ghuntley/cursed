yeet "testz"

test_start("Specific Expression Evaluation Fixes")

# Test 1: Float variables are properly stored and used
vibez.spill("=== Test 1: Float Storage and Operations ===")
sus x normie = 10.5
sus y normie = 2.5

# Check floats are preserved 
vibez.spill("x =", x, "(should be 10.5)")
vibez.spill("y =", y, "(should be 2.5)")

sus float_result normie = x + y
vibez.spill("x + y =", float_result, "(should be 13.0)")

sus float_precise normie = x / y
vibez.spill("x / y =", float_precise, "(should be 4.2)")

# Test 2: Unary operators work correctly
vibez.spill("=== Test 2: Unary Operators ===")
sus a drip = 42
sus negative_literal drip = -15
sus negative_var drip = -a

vibez.spill("negative_literal =", negative_literal, "(should be -15)")
vibez.spill("negative_var =", negative_var, "(should be -42)")

# Test 3: Operator precedence
vibez.spill("=== Test 3: Operator Precedence ===")
sus precedence_test drip = 2 + 3 * 4
vibez.spill("2 + 3 * 4 =", precedence_test, "(should be 14)")

sus precedence_complex drip = (2 + 3) * 4
vibez.spill("(2 + 3) * 4 =", precedence_complex, "(should be 20)")

# Test 4: Mixed type operations
vibez.spill("=== Test 4: Mixed Type Operations ===")
sus mixed_add normie = 10 + 2.5
vibez.spill("10 + 2.5 =", mixed_add, "(should be 12.5)")

sus mixed_mul normie = 3.5 * 2
vibez.spill("3.5 * 2 =", mixed_mul, "(should be 7.0)")

# Test 5: Complex expressions with correct evaluation
vibez.spill("=== Test 5: Complex Expressions ===")
sus complex_expr drip = (10 + 5) / (3 * 1) + 2
vibez.spill("(10 + 5) / (3 * 1) + 2 =", complex_expr, "(should be 7)")

sus complex_float normie = (10.5 + 2.5) / 2.0 - 1.5
vibez.spill("(10.5 + 2.5) / 2.0 - 1.5 =", complex_float, "(should be 5.0)")

# Test 6: Comparison operations with different types
vibez.spill("=== Test 6: Comparison Operations ===")
sus int_gt_test lit = 10 > 5
vibez.spill("10 > 5 =", int_gt_test, "(should be true)")

sus float_eq_test lit = 3.14 == 3.14
vibez.spill("3.14 == 3.14 =", float_eq_test, "(should be true)")

sus mixed_lt_test lit = 5 < 7.5
vibez.spill("5 < 7.5 =", mixed_lt_test, "(should be true)")

vibez.spill("=== All Tests Completed ===")
print_test_summary()
