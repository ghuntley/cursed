// Comprehensive Error Handling Test for CURSED
// Tests all aspects of the enhanced error handling system

// Test 1: Basic error creation and handling
vibez.spill("Test 1: Basic error creation")
yikes basic_error := "This is a basic error"
vibez.spill("Created basic error:", basic_error)

// Test 2: Error propagation
vibez.spill("Test 2: Error propagation")
slay error_function() {
    yikes func_error := "Function error"
    vibez.spill("Function error created:", func_error)
    damn func_error shook
}

// Test 3: Error recovery with fam
vibez.spill("Test 3: Error recovery")
fam {
    yikes recovery_error := "Error to be recovered"
    vibez.spill("Error before recovery:", recovery_error)
} sus caught_error {
    vibez.spill("Error recovered successfully:", caught_error)
}

// Test 4: Multiple error types
vibez.spill("Test 4: Multiple error types")
yikes type_error1 := "Type error 1"
yikes type_error2 := "Type error 2"
yikes type_error3 := "Type error 3"
vibez.spill("Multiple errors created:", type_error1, type_error2, type_error3)

// Test 5: Error in loops
vibez.spill("Test 5: Error in loops")
bestie i := 0; i < 3; i++ {
    yikes loop_error := "Loop error iteration"
    vibez.spill("Loop error", i, ":", loop_error)
}

// Test 6: Complex error handling scenario
vibez.spill("Test 6: Complex error handling")
slay complex_function() {
    fam {
        yikes complex_error := "Complex error scenario"
        vibez.spill("Complex error:", complex_error)
        damn complex_error shook
    } sus inner_error {
        vibez.spill("Inner error handled:", inner_error)
        yikes escalated_error := "Escalated error"
        damn escalated_error shook
    }
}

fam {
    complex_function()
} sus final_error {
    vibez.spill("Final error handled:", final_error)
}

// Test 7: Error with different data types
vibez.spill("Test 7: Error with different data types")
yikes string_error := "String error"
yikes number_error := 42
yikes boolean_error := based
vibez.spill("Different error types:", string_error, number_error, boolean_error)

vibez.spill("=== Comprehensive Error Handling Tests Complete ===")
