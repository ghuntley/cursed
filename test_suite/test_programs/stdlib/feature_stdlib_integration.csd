vibe main_character

yeet "mathz"
yeet "vibez"

fr fr Standard Library Integration Test
fr fr Tests: Multiple stdlib modules working together
fr fr Expected: All stdlib functions integrate correctly

slay main_character() {
    vibez.spill("=== Stdlib Integration Test ===")
    
    fr fr Test mathz functions
    vibez.spill("Testing mathz.abs with negative:")
    sus abs_result normie = mathz.abs(-25)
    vibez.spill(abs_result)
    
    vibez.spill("Testing mathz.max with two values:")
    sus max_result normie = mathz.max(17, 23)
    vibez.spill(max_result)
    
    vibez.spill("Testing mathz.min with two values:")
    sus min_result normie = mathz.min(45, 32)
    vibez.spill(min_result)
    
    vibez.spill("Testing mathz.add_two function:")
    sus add_result normie = mathz.add_two(15, 27)
    vibez.spill(add_result)
    
    fr fr Chain operations using stdlib results
    sus chained_result normie = mathz.abs(mathz.min(-10, -5))
    vibez.spill("Chained operations mathz.abs(mathz.min(-10, -5)):")
    vibez.spill(chained_result)
    
    fr fr Use stdlib results in expressions
    sus expr_result normie = mathz.max(10, 20) + mathz.abs(-8)
    vibez.spill("Expression with stdlib (max(10,20) + abs(-8)):")
    vibez.spill(expr_result)
    
    vibez.spill("=== Integration Test Complete ===")
}
