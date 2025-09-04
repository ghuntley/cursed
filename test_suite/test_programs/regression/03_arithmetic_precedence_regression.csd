vibe main
yeet "vibez"
yeet "mathz"

fr fr Arithmetic Precedence Regression Test
fr fr Tests: Operator precedence and expression evaluation order
fr fr Expected: Correct mathematical precedence maintained

slay main_character() {
    vibez.spill("=== Arithmetic Precedence Regression Test ===")
    
    vibez.spill("Testing basic precedence...")
    sus result1 = 2 + 3 * 4
    vibez.spill("2 + 3 * 4 =", result1, "(should be 14)")
    
    sus result2 = 10 - 2 * 3
    vibez.spill("10 - 2 * 3 =", result2, "(should be 4)")
    
    sus result3 = 20 + 8 / 2
    vibez.spill("20 + 8 / 2 =", result3, "(should be 24)")
    
    vibez.spill("Testing parentheses override...")
    sus paren1 = (2 + 3) * 4
    vibez.spill("(2 + 3) * 4 =", paren1, "(should be 20)")
    
    sus paren2 = 2 * (3 + 4)
    vibez.spill("2 * (3 + 4) =", paren2, "(should be 14)")
    
    sus paren3 = 20 / (2 + 3)
    vibez.spill("20 / (2 + 3) =", paren3, "(should be 4)")
    
    vibez.spill("Testing complex expressions...")
    sus complex1 = 1 + 2 * 3 - 4 / 2
    vibez.spill("1 + 2 * 3 - 4 / 2 =", complex1, "(should be 5)")
    
    sus complex2 = (1 + 2) * (3 + 4) - (8 / 2)
    vibez.spill("(1+2)*(3+4)-(8/2) =", complex2, "(should be 17)")
    
    vibez.spill("Testing with mathz functions...")
    sus math_result1 = mathz.abs_normie(-5) + 3
    vibez.spill("abs(-5) + 3 =", math_result1, "(should be 8)")
    
    sus math_result2 = 2 * mathz.abs_normie(-4)
    vibez.spill("2 * abs(-4) =", math_result2, "(should be 8)")
    
    vibez.spill("Arithmetic precedence regression test completed")
}
