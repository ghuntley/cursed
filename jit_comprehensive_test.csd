// Comprehensive JIT Execution Engine Test
// Tests all major CURSED language features with JIT compilation

slay test_basic_math() normie {
    sus a normie = 10
    sus b normie = 5
    damn a + b
}

slay test_string_operations() tea {
    sus greeting tea = "Hello"
    sus name tea = "CURSED"
    damn greeting + " " + name + "!"
}

slay test_conditional_logic() lit {
    sus x normie = 42
    lowkey (x > 40) {
        damn based
    } highkey {
        damn cringe
    }
}

slay test_struct_creation() {
    squad Point {
        spill x normie
        spill y normie
    }
    
    sus p Point = Point{ x: 10, y: 20 }
    vibez.spillf("Point: ({}, {})", p.x, p.y)
}

slay test_function_calls() {
    slay inner_function(val normie) normie {
        damn val * 2
    }
    
    sus result normie = inner_function(21)
    vibez.spillf("Function result: {}", result)
}

slay test_type_conversions() {
    sus int_val normie = 42
    sus float_val meal = int_val.(meal)
    sus string_val tea = float_val.(tea)
    sus bool_val lit = int_val.(lit)
    
    vibez.spill("Type conversions complete")
}

slay test_error_handling() {
    ready {
        sus risky_val normie = 10 / 0
        vibez.spillf("Unexpected: {}", risky_val)
    } yikes (err) {
        vibez.spillf("Caught error: {}", err.message)
    }
}

slay main() {
    vibez.spill("🧪 Starting JIT Comprehensive Tests")
    
    // Test basic math
    sus math_result normie = test_basic_math()
    vibez.spillf("Math test result: {}", math_result)
    
    // Test string operations
    sus string_result tea = test_string_operations() 
    vibez.spillf("String test result: {}", string_result)
    
    // Test conditional logic
    sus logic_result lit = test_conditional_logic()
    vibez.spillf("Logic test result: {}", logic_result)
    
    // Test struct creation
    test_struct_creation()
    
    // Test function calls
    test_function_calls()
    
    // Test type conversions
    test_type_conversions()
    
    // Test error handling
    test_error_handling()
    
    vibez.spill("✅ JIT Comprehensive Tests Complete")
}

main()
