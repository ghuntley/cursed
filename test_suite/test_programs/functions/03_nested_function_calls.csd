vibe main

yeet "vibez"

slay double_value(x normie) normie {
    damn x * 2
}

slay add_one(x normie) normie {
    damn x + 1
}

slay complex_calc(x normie) normie {
    damn double_value(add_one(x))
}

slay main_character() {
    vibez.spill("=== Nested Function Calls Test ===")
    
    vibez.spill("double_value(5):")
    vibez.spill(double_value(5))
    
    vibez.spill("add_one(7):")
    vibez.spill(add_one(7))
    
    vibez.spill("complex_calc(3) = double_value(add_one(3)):")
    vibez.spill(complex_calc(3))
    
    vibez.spill("Nested in expression:")
    vibez.spill(double_value(add_one(2)) + 1)
    
    vibez.spill("=== Test Complete ===")
}
