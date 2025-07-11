// Working Error Handling Test for CURSED
// Simple test to verify basic error handling works

vibez.spill("=== Working Error Handling Test ===")

// Test 1: Basic yikes error
vibez.spill("Test 1: Basic yikes error")
yikes test_error := "Test error message"
vibez.spill("Error created:", test_error)

// Test 2: Simple error recovery
vibez.spill("Test 2: Simple error recovery")
fam {
    yikes panic_error := "Panic error message"
    vibez.spill("Error before panic:", panic_error)
} sus caught_error {
    vibez.spill("Error caught:", caught_error)
}

// Test 3: Error propagation
vibez.spill("Test 3: Error propagation")
slay error_func() {
    yikes func_error := "Function error"
    vibez.spill("Function error:", func_error)
    damn func_error shook
}

sus result := error_func()
vibez.spill("Function result:", result)

vibez.spill("=== Test Complete ===")
