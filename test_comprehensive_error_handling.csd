yeet "testz"

test_start("Comprehensive Error Handling Tests")

// Test 1: Basic yikes error creation with context
vibez.spill("=== Test 1: Basic yikes error creation ===")
yikes basic_error := "This is a basic error"
assert_eq_string(basic_error, "This is a basic error")
vibez.spill("✓ Basic error creation works")

// Test 2: Error propagation with shook
vibez.spill("=== Test 2: Error propagation with shook ===")
slay risky_function() {
    yikes func_error := "Function failed"
    damn func_error shook  // Propagate error
}

// Test error propagation
fam {
    sus result := risky_function()
    vibez.spill("This should not execute")
} sus caught_error {
    vibez.spill("✓ Error propagation caught:", caught_error)
    assert_true(caught_error != cringe)
}

// Test 3: Nested error handling
vibez.spill("=== Test 3: Nested error handling ===")
slay nested_error_function() {
    fam {
        yikes inner_error := "Inner error"
        damn inner_error shook
    } sus inner_caught {
        vibez.spill("Inner error caught:", inner_caught)
        yikes outer_error := "Outer error from inner: " + inner_caught
        damn outer_error shook
    }
}

fam {
    sus result := nested_error_function()
    vibez.spill("This should not execute")
} sus final_error {
    vibez.spill("✓ Nested error handling works:", final_error)
    assert_true(final_error != cringe)
}

// Test 4: Error recovery mechanisms
vibez.spill("=== Test 4: Error recovery mechanisms ===")
sus recovery_successful lit = cap
fam {
    yikes recoverable_error := "Recoverable error"
    damn recoverable_error shook
} sus recovery_error {
    vibez.spill("Recovering from error:", recovery_error)
    recovery_successful = based
}

assert_true(recovery_successful)
vibez.spill("✓ Error recovery successful")

// Test 5: Multiple error scenarios
vibez.spill("=== Test 5: Multiple error scenarios ===")
slay multiple_error_function() {
    sus error_count normie = 0
    
    fam {
        yikes error1 := "First error"
        error_count++
        damn error1 shook
    } sus e1 {
        vibez.spill("Caught first error:", e1)
        
        fam {
            yikes error2 := "Second error"
            error_count++
            damn error2 shook
        } sus e2 {
            vibez.spill("Caught second error:", e2)
            damn error_count
        }
    }
}

sus final_count := multiple_error_function()
assert_eq_int(final_count, 2)
vibez.spill("✓ Multiple error scenarios work")

// Test 6: Error context and stack traces
vibez.spill("=== Test 6: Error context and stack traces ===")
slay function_with_context() {
    yikes context_error := "Error with context information"
    damn context_error shook
}

fam {
    sus result := function_with_context()
    vibez.spill("This should not execute")
} sus context_error {
    vibez.spill("✓ Error with context caught:", context_error)
    assert_true(context_error != cringe)
}

// Test 7: Error handling with function calls
vibez.spill("=== Test 7: Error handling with function calls ===")
slay helper_function() {
    yikes helper_error := "Helper function error"
    damn helper_error shook
}

slay calling_function() {
    fam {
        sus result := helper_function()
        damn result
    } sus helper_err {
        yikes calling_error := "Calling function error: " + helper_err
        damn calling_error shook
    }
}

fam {
    sus result := calling_function()
    vibez.spill("This should not execute")
} sus final_err {
    vibez.spill("✓ Function call error handling works:", final_err)
    assert_true(final_err != cringe)
}

// Test 8: Error handling with different data types
vibez.spill("=== Test 8: Error handling with different data types ===")
slay type_error_function() {
    yikes type_error := "Type error occurred"
    damn type_error shook
}

fam {
    sus result := type_error_function()
    vibez.spill("This should not execute")
} sus type_err {
    vibez.spill("✓ Type error handling works:", type_err)
    assert_true(type_err != cringe)
}

print_test_summary()
