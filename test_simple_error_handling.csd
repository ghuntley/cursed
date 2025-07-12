// Simple error handling test

// Test 1: Basic yikes error creation
vibez.spill("=== Test 1: Basic yikes error creation ===")
yikes basic_error := "This is a basic error"
vibez.spill("Created error:", basic_error)
vibez.spill("✓ Basic error creation works")

// Test 2: Error recovery with fam
vibez.spill("=== Test 2: Error recovery with fam ===")
fam {
    yikes panic_error := "This should be caught"
    vibez.spill("This should not be reached")
} sus caught_error {
    vibez.spill("✓ Caught error:", caught_error)
}

// Test 3: Error propagation with shook
vibez.spill("=== Test 3: Error propagation with shook ===")
slay risky_function() {
    yikes func_error := "Function error"
    damn func_error shook
}

fam {
    sus result := risky_function()
    vibez.spill("This should not execute")
} sus propagated_error {
    vibez.spill("✓ Error propagation works:", propagated_error)
}

// Test 4: Complex error scenario
vibez.spill("=== Test 4: Complex error scenario ===")
slay complex_operation() {
    fam {
        yikes inner_error := "Inner operation failed"
        damn inner_error shook
    } sus inner_caught {
        yikes outer_error := "Outer error: " + inner_caught
        damn outer_error shook
    }
}

fam {
    sus result := complex_operation()
    vibez.spill("This should not execute")
} sus final_error {
    vibez.spill("✓ Complex error scenario works:", final_error)
}

// Test 5: Error handling with variables
vibez.spill("=== Test 5: Error handling with variables ===")
sus error_count normie = 0

fam {
    error_count = error_count + 1
    yikes counting_error := "Error number " + string(error_count)
    damn counting_error shook
} sus count_error {
    vibez.spill("✓ Error with variables works:", count_error)
    vibez.spill("Error count:", error_count)
}

vibez.spill("=== All Error Handling Tests Complete ===")
