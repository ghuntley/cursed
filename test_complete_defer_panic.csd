// Test complete defer/panic recovery LLVM codegen implementation

slay test_defer_with_panic() lit {
    vibez.spill("Starting function with defer and panic")
    
    // Add multiple defer statements for LIFO testing
    defer vibez.spill("Cleanup 1 - should execute last")
    defer vibez.spill("Cleanup 2 - should execute second")
    defer vibez.spill("Cleanup 3 - should execute first")
    
    // Conditional panic with proper defer cleanup
    lowkey (based) {
        vibez.spill("About to panic with defer cleanup")
        yeet_error "Test panic with defer cleanup"
    }
    
    vibez.spill("This should never execute")
    damn based
}

slay test_defer_normal_return() lit {
    vibez.spill("Function with normal return and defer")
    
    defer vibez.spill("Normal cleanup 1")
    defer vibez.spill("Normal cleanup 2")
    
    vibez.spill("Returning normally")
    damn based
}

slay test_nested_defer_scopes() lit {
    vibez.spill("Testing nested defer scopes")
    
    defer vibez.spill("Outer defer 1")
    
    lowkey (based) {
        defer vibez.spill("Inner defer 1")
        defer vibez.spill("Inner defer 2")
        
        vibez.spill("Inner scope operations")
    }
    
    defer vibez.spill("Outer defer 2")
    
    vibez.spill("Function ending")
    damn based
}

slay main() normie {
    vibez.spill("Testing complete defer/panic recovery system")
    
    // Test normal defer execution
    test_defer_normal_return()
    
    // Test nested defer scopes
    test_nested_defer_scopes()
    
    // Test defer with panic (this will cause the program to exit)
    // Uncomment to test panic recovery:
    // test_defer_with_panic()
    
    vibez.spill("All defer tests completed")
    damn 0
}
