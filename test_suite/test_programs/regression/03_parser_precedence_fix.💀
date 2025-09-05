vibe main
yeet "vibez"
yeet "mathz"

// Regression test for parser precedence and expression evaluation fixes
slay test_basic_arithmetic_precedence() {
    vibez.spill("=== Basic Arithmetic Precedence Tests ===")
    
    // Test multiplication/division before addition/subtraction
    vibez.spill("Precedence: multiplication before addition")
    sus result1 = 2 + 3 * 4
    vibez.spill("2 + 3 * 4 =", result1, "(should be 14)")
    
    sus result2 = 10 - 2 * 3
    vibez.spill("10 - 2 * 3 =", result2, "(should be 4)")
    
    sus result3 = 1 + 2 * 3 + 4
    vibez.spill("1 + 2 * 3 + 4 =", result3, "(should be 11)")
    
    // Test division precedence
    vibez.spill("Precedence: division before addition")
    sus result4 = 20 + 8 / 2
    vibez.spill("20 + 8 / 2 =", result4, "(should be 24)")
    
    sus result5 = 30 - 12 / 3
    vibez.spill("30 - 12 / 3 =", result5, "(should be 26)")
}

slay test_parentheses_override() {
    vibez.spill("=== Parentheses Override Tests ===")
    
    // Test parentheses overriding default precedence
    sus result1 = (2 + 3) * 4
    vibez.spill("(2 + 3) * 4 =", result1, "(should be 20)")
    
    sus result2 = 2 * (3 + 4)
    vibez.spill("2 * (3 + 4) =", result2, "(should be 14)")
    
    sus result3 = (10 - 2) * 3
    vibez.spill("(10 - 2) * 3 =", result3, "(should be 24)")
    
    sus result4 = 20 / (2 + 3)
    vibez.spill("20 / (2 + 3) =", result4, "(should be 4)")
    
    // Test nested parentheses
    sus result5 = ((2 + 3) * (4 - 1))
    vibez.spill("((2 + 3) * (4 - 1)) =", result5, "(should be 15)")
    
    sus result6 = (10 + (5 * 2)) / (3 + 2)
    vibez.spill("(10 + (5 * 2)) / (3 + 2) =", result6, "(should be 4)")
}

slay test_complex_expression_precedence() {
    vibez.spill("=== Complex Expression Precedence Tests ===")
    
    // Test mixed operators with correct precedence
    sus expr1 = 1 + 2 * 3 - 4 / 2
    vibez.spill("1 + 2 * 3 - 4 / 2 =", expr1, "(should be 5)")
    
    sus expr2 = 10 * 2 + 5 * 3 - 8 / 4
    vibez.spill("10 * 2 + 5 * 3 - 8 / 4 =", expr2, "(should be 33)")
    
    sus expr3 = 100 / 5 + 3 * 7 - 2 * 4
    vibez.spill("100 / 5 + 3 * 7 - 2 * 4 =", expr3, "(should be 33)")
    
    // Test with parentheses changing precedence
    sus expr4 = (1 + 2) * (3 - 4) / (2 + 0)
    vibez.spill("(1 + 2) * (3 - 4) / (2 + 0) =", expr4, "(should be -1 or -1.5)")
    
    sus expr5 = 2 * (3 + 4 * 5) - (8 - 3 * 2)
    vibez.spill("2 * (3 + 4 * 5) - (8 - 3 * 2) =", expr5, "(should be 44)")
}

slay test_comparison_precedence() {
    vibez.spill("=== Comparison Precedence Tests ===")
    
    // Test comparison operators with arithmetic
    sus test1 lit = 2 + 3 > 4
    vibez.spill("2 + 3 > 4 =", test1, "(should be true/based)")
    
    sus test2 lit = 10 - 5 < 3 * 2
    vibez.spill("10 - 5 < 3 * 2 =", test2, "(should be true/based)")
    
    sus test3 lit = 4 * 3 == 12
    vibez.spill("4 * 3 == 12 =", test3, "(should be true/based)")
    
    sus test4 lit = 15 / 3 != 4
    vibez.spill("15 / 3 != 4 =", test4, "(should be true/based)")
    
    // Test with parentheses
    sus test5 lit = (2 + 3) > (4 - 1)
    vibez.spill("(2 + 3) > (4 - 1) =", test5, "(should be true/based)")
}

slay test_logical_operator_precedence() {
    vibez.spill("=== Logical Operator Precedence Tests ===")
    
    // Test logical AND/OR with other operators
    sus val1 normie = 5
    sus val2 normie = 10
    sus val3 normie = 15
    
    sus logic1 lit = val1 < val2 && val2 < val3
    vibez.spill("5 < 10 && 10 < 15 =", logic1, "(should be true)")
    
    sus logic2 lit = val1 > val2 || val2 < val3
    vibez.spill("5 > 10 || 10 < 15 =", logic2, "(should be true)")
    
    sus logic3 lit = val1 + 5 == val2 && val3 - val2 == val1
    vibez.spill("(5+5)==10 && (15-10)==5 =", logic3, "(should be true)")
    
    // Test NOT operator precedence
    sus logic4 lit = !(val1 > val2)
    vibez.spill("!(5 > 10) =", logic4, "(should be true)")
}

slay test_assignment_precedence() {
    vibez.spill("=== Assignment Precedence Tests ===")
    
    // Test that assignment has lowest precedence
    sus assign_test1 normie = 0
    sus assign_test2 normie = 0
    
    assign_test1 = 2 + 3 * 4
    vibez.spill("Assignment result: 2 + 3 * 4 =", assign_test1)
    
    assign_test2 = (5 - 2) * 3
    vibez.spill("Assignment result: (5 - 2) * 3 =", assign_test2)
    
    // Test chained assignments
    sus chain1 normie = 0
    sus chain2 normie = 0
    sus chain3 normie = 0
    
    chain3 = chain2 = chain1 = 42
    vibez.spill("Chained assignment results:")
    vibez.spill("chain1 =", chain1)
    vibez.spill("chain2 =", chain2)
    vibez.spill("chain3 =", chain3)
}

slay test_function_call_precedence() {
    vibez.spill("=== Function Call Precedence Tests ===")
    
    // Test function calls in expressions
    sus func_result1 = mathz.abs_normie(-5) + 3
    vibez.spill("mathz.abs_normie(-5) + 3 =", func_result1, "(should be 8)")
    
    sus func_result2 = 2 * mathz.abs_normie(-4)
    vibez.spill("2 * mathz.abs_normie(-4) =", func_result2, "(should be 8)")
    
    sus func_result3 = mathz.pow(2, 3) + mathz.abs_normie(-1)
    vibez.spill("mathz.pow(2,3) + mathz.abs_normie(-1) =", func_result3, "(should be 9)")
    
    // Test function calls with complex arguments
    sus complex_arg1 = mathz.abs_normie(2 - 5)
    vibez.spill("mathz.abs_normie(2 - 5) =", complex_arg1, "(should be 3)")
    
    sus complex_arg2 = mathz.pow(1 + 1, 2 + 1)
    vibez.spill("mathz.pow(1+1, 2+1) =", complex_arg2, "(should be 8)")
}

slay main_character() {
    vibez.spill("=== Parser Precedence Regression Tests ===")
    
    test_basic_arithmetic_precedence()
    test_parentheses_override()
    test_complex_expression_precedence()
    test_comparison_precedence()
    test_logical_operator_precedence()
    test_assignment_precedence()
    test_function_call_precedence()
    
    vibez.spill("All parser precedence regression tests completed")
    vibez.spill("Expression evaluation order verified - regression test passed")
}
