#!/usr/bin/env cursed

# Test 1: Basic defer with LIFO order
slay test_defer_basic() {
    vibez.spill("Function start")
    later vibez.spill("Defer 1")
    later vibez.spill("Defer 2")
    later vibez.spill("Defer 3")
    vibez.spill("Function end")
}

# Test 2: Defer with early return
slay test_defer_early_return() {
    vibez.spill("Function start")
    later vibez.spill("Defer before return")
    later vibez.spill("Defer cleanup")
    
    sus x := 42
    cap x > 30 {
        vibez.spill("Early return")
        damn x
    }
    
    vibez.spill("Should not reach here")
    damn 0
}

# Test 3: Defer with nested function calls
slay test_defer_nested() {
    vibez.spill("Outer function start")
    later vibez.spill("Outer defer 1")
    later vibez.spill("Outer defer 2")
    
    slay inner_func() {
        vibez.spill("Inner function start")
        later vibez.spill("Inner defer 1")
        later vibez.spill("Inner defer 2")
        vibez.spill("Inner function end")
    }
    
    inner_func()
    vibez.spill("Outer function end")
}

# Test 4: Defer with loops
slay test_defer_with_loop() {
    vibez.spill("Function start")
    later vibez.spill("Function defer")
    
    bestie i := 0; i < 3; i++ {
        vibez.spill("Loop iteration: " + i)
        later vibez.spill("Loop defer: " + i)
    }
    
    vibez.spill("Function end")
}

# Test 5: Defer with variables (closure capture)
slay test_defer_with_variables() {
    vibez.spill("Function start")
    sus message := "Hello from defer"
    sus count := 42
    
    later vibez.spill("Defer with message: " + message)
    later vibez.spill("Defer with count: " + count)
    
    # Change variables after defer registration
    message = "Changed message"
    count = 99
    
    vibez.spill("Function end")
}

# Test 6: Defer with complex expressions
slay test_defer_complex() {
    vibez.spill("Function start")
    sus x := 10
    sus y := 20
    
    later vibez.spill("Complex defer: " + (x + y))
    later {
        sus result := x * y
        vibez.spill("Block defer result: " + result)
    }
    
    vibez.spill("Function end")
}

# Test 7: Defer error handling
slay test_defer_error_handling() {
    vibez.spill("Function start")
    later vibez.spill("Cleanup even with error")
    later vibez.spill("Second cleanup")
    
    # This should cause an error but defers should still execute
    vibez.spill("About to cause error")
    # sus badVar := undefinedVariable # This would cause an error
    
    vibez.spill("Function end")
}

# Run all tests
vibez.spill("=== Testing Basic Defer ===")
test_defer_basic()

vibez.spill("\n=== Testing Early Return ===")
sus result := test_defer_early_return()
vibez.spill("Return value: " + result)

vibez.spill("\n=== Testing Nested Functions ===")
test_defer_nested()

vibez.spill("\n=== Testing Defer with Loop ===")
test_defer_with_loop()

vibez.spill("\n=== Testing Defer with Variables ===")
test_defer_with_variables()

vibez.spill("\n=== Testing Complex Defer ===")
test_defer_complex()

vibez.spill("\n=== Testing Error Handling ===")
test_defer_error_handling()

vibez.spill("\n=== All defer tests completed ===")
