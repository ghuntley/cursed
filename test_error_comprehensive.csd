// Comprehensive error handling test

vibez.spill("=== CURSED Error Handling Test ===")

// Test 1: Basic yikes error creation
vibez.spill("Test 1: Basic yikes error creation")
yikes test_error := "This is a test error"
vibez.spill("Created error:", test_error)
vibez.spill("Error type:", test_error.type_name())

// Test 2: Function with error return
vibez.spill("Test 2: Function with error return")
slay risky_function() {
    yikes func_error := "Function failed"
    damn func_error shook  // This should propagate the error
}

// Test 3: Fam error recovery with function call
vibez.spill("Test 3: Fam error recovery")
fam {
    // This should trigger an error that gets caught
    sus result := risky_function()
    vibez.spill("This should not be reached")
} sus caught_error {
    vibez.spill("Caught error:", caught_error)
}

// Test 4: Error checking
vibez.spill("Test 4: Error checking")
slay safe_function() {
    yikes potential_error := "Safe function error"
    damn potential_error  // Return the error
}

sus function_result := safe_function()
vibez.spill("Function result:", function_result)

// Test 5: Multiple error handling
vibez.spill("Test 5: Multiple error handling")
yikes error1 := "First error"
yikes error2 := "Second error"
vibez.spill("Error 1:", error1)
vibez.spill("Error 2:", error2)

vibez.spill("=== Error handling tests complete ===")
