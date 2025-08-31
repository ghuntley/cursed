vibe main

fr fr Test: Function parameters and return values
fr fr Purpose: Test function definition and calling with various signatures
fr fr Expected: All function calls should work correctly

slay main_character() {
    fr fr Test void function
    simple_function()
    
    fr fr Test single parameter function
    sus result1 drip = double_value(21)
    
    fr fr Test multiple parameter function
    sus result2 drip = add_three(5, 10, 15)
    
    fr fr Test function with boolean
    sus is_greater lit = compare_values(10, 5)
    
    fr fr Test nested function calls
    sus nested_result drip = double_value(add_three(1, 2, 3))
    
    damn 0
}

slay simple_function() {
    sus x drip = 42
}

slay double_value(value drip) normie {
    damn value * 2
}

slay add_three(a drip, b drip, c drip) normie {
    damn a + b + c
}

slay compare_values(x drip, y drip) lit {
    damn x > y
}
