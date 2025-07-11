// Complete error handling test with all features

vibez.spill("=== Complete Error Handling Test ===")

// Test 1: Basic error creation
vibez.spill("Test 1: Basic error creation")
yikes basic_error := "This is a basic error"
vibez.spill("✓ Created basic error")

// Test 2: Error propagation with shook
vibez.spill("Test 2: Error propagation with shook")
sus propagated_error := (shook basic_error)
vibez.spill("✓ Propagated error with shook")

// Test 3: Error recovery with fam
vibez.spill("Test 3: Error recovery with fam")
fam {
    yikes recovery_error := "Error in recovery block"
    vibez.spill("✓ Created error in fam block")
}

// Test 4: Function with error handling
vibez.spill("Test 4: Function with error handling")
slay error_function(should_fail lit) lit {
    if should_fail {
        yikes function_error := "Function intentionally failed"
        damn cap
    }
    damn based
}

sus success_result := error_function(cap)
vibez.spill("✓ Function call succeeded: ", success_result)

sus fail_result := error_function(based)
vibez.spill("✓ Function call failed: ", fail_result)

// Test 5: Goroutine with error handling
vibez.spill("Test 5: Goroutine with error handling")
slay goroutine_with_error() {
    yikes goroutine_error := "Error in goroutine"
    sus propagated_in_goroutine := (shook goroutine_error)
    vibez.spill("✓ Handled error in goroutine")
}

yolo goroutine_with_error()

// Test 6: Nested error handling
vibez.spill("Test 6: Nested error handling")
fam {
    yikes outer_error := "Outer error"
    
    fam {
        yikes inner_error := "Inner error"
        sus nested_propagated := (shook inner_error)
        vibez.spill("✓ Nested error handling")
    }
}

vibez.spill("=== All error handling tests completed successfully! ===")
