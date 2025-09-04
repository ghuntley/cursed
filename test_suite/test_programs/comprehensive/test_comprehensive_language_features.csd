vibe main
yeet "vibez"
yeet "mathz"

slay simple_multiply(x normie, y normie) normie {
    damn x * y
}

slay test_arithmetic_operations() {
    sus a normie = 15
    sus b normie = 7
    
    vibez.spill("Testing arithmetic operations:")
    vibez.spill("Addition:")
    vibez.spill(a + b)
    
    vibez.spill("Subtraction:")
    vibez.spill(a - b)
    
    vibez.spill("Multiplication:")
    vibez.spill(a * b)
    
    vibez.spill("Division:")
    vibez.spill(a / b)
    
    vibez.spill("Math operations completed")
}

slay test_control_flow(value normie) {
    ready (value > 10) {
        vibez.spill("Value is large")
    } otherwise ready (value > 5) {
        vibez.spill("Value is medium") 
    } otherwise {
        vibez.spill("Value is small")
    }
}

slay test_loops() {
    vibez.spill("Testing periodt loop:")
    sus i normie = 1
    periodt (i <= 3) {
        vibez.spill("Loop iteration:")
        vibez.spill(i)
        i = i + 1
    }
}

slay main() normie {
    vibez.spill("CURSED Comprehensive Language Features Test")
    vibez.spill("=========================================")
    
    sus integer_var normie = 42
    vibez.spill("Variable declaration:")
    vibez.spill(integer_var)
    
    vibez.spill("Function call test - multiply 5 * 6:")
    sus multiply_result normie = simple_multiply(5, 6)
    vibez.spill(multiply_result)
    
    vibez.spill("Control flow test:")
    test_control_flow(15)
    test_control_flow(7)
    test_control_flow(3)
    
    test_loops()
    test_arithmetic_operations()
    
    vibez.spill("All tests completed successfully!")
    damn 0
}
