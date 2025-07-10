#!/usr/bin/env cursed

# Test 1: Basic defer execution order (LIFO)
slay test_defer_lifo() {
    vibez.spill("Function start")
    later vibez.spill("Defer 1")
    later vibez.spill("Defer 2")
    later vibez.spill("Defer 3")
    vibez.spill("Function end")
}

# Test 2: Defer with early return
slay test_defer_early_return() {
    vibez.spill("Function start")
    later vibez.spill("Cleanup after early return")
    
    cap based {
        vibez.spill("Early return")
        damn 42
    }
    
    vibez.spill("Should not reach here")
    damn 0
}

# Test 3: Defer with nested function calls
slay test_defer_nested() {
    vibez.spill("Outer start")
    later vibez.spill("Outer cleanup")
    
    slay inner_func() {
        vibez.spill("Inner start")
        later vibez.spill("Inner cleanup")
        vibez.spill("Inner end")
    }
    
    inner_func()
    vibez.spill("Outer end")
}

# Test 4: Defer with variables
slay test_defer_variables() {
    vibez.spill("Function start")
    sus msg := "Hello"
    later vibez.spill("Defer: " + msg)
    
    msg = "Changed"
    vibez.spill("Function end")
}

# Run tests
vibez.spill("=== Testing LIFO Order ===")
test_defer_lifo()

vibez.spill("\n=== Testing Early Return ===")
sus result := test_defer_early_return()
vibez.spill("Result: " + result)

vibez.spill("\n=== Testing Nested Functions ===")
test_defer_nested()

vibez.spill("\n=== Testing Variables ===")
test_defer_variables()

vibez.spill("\n=== Tests completed ===")
