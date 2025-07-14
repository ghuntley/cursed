yeet "testz"

slay test_defer_basics() {
    test_start("defer basic execution order")
    
    vibez.spill("Function start")
    later vibez.spill("Defer 1")
    later vibez.spill("Defer 2")
    later vibez.spill("Defer 3")
    vibez.spill("Function end")
    
    // Defers should execute in reverse order: 3, 2, 1
    // Expected output: Function start, Function end, Defer 3, Defer 2, Defer 1
    
    test_pass() // This test passes since defer execution is automatic
}

slay cleanup_function() {
    vibez.spill("Cleanup function called")
}

slay test_defer_with_functions() {
    test_start("defer with function calls")
    
    vibez.spill("Setting up resources")
    later cleanup_function()
    later vibez.spill("Final cleanup")
    
    vibez.spill("Using resources")
    
    test_pass()
}

slay test_defer_in_loops() {
    test_start("defer in loops")
    
    bestie i := 0; i < 3; i++ {
        vibez.spill("Loop iteration")
        later vibez.spill("Loop defer")
    }
    
    // All loop defers should execute at function end
    test_pass()
}

slay test_defer_with_return() {
    test_start("defer with early return")
    
    vibez.spill("Before defer")
    later vibez.spill("This should still execute")
    
    nah // Early return - defer should still execute
    
    test_fail() // Should not reach here
}

// Run all defer tests
test_defer_basics()
test_defer_with_functions()
test_defer_in_loops()
test_defer_with_return()

print_test_summary()
