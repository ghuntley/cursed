// Comprehensive test for shook/fam error handling patterns
yeet "testz"

test_start("Advanced Error Handling - shook/fam blocks")

// Test 1: Basic shook error propagation
slay test_shook_propagation() lit {
    shook {
        yikes "Test error for propagation"
        damn cringe  // Should not execute
    } fam err {
        vibez.spill("Caught error:", err)
        damn based
    }
}

assert_true(test_shook_propagation())
vibez.spill("✅ Test 1: Basic shook propagation passed")

// Test 2: Nested shook/fam blocks
slay test_nested_error_handling() lit {
    shook {
        shook {
            yikes "Inner error"
        } fam inner_err {
            vibez.spill("Inner catch:", inner_err)
            yikes "Re-throwing from inner"
        }
    } fam outer_err {
        vibez.spill("Outer catch:", outer_err)
        damn based
    }
}

assert_true(test_nested_error_handling())
vibez.spill("✅ Test 2: Nested error handling passed")

// Test 3: Function with error propagation
slay risky_divide(a drip, b drip) drip {
    ready (b == 0) {
        yikes "Division by zero error"
    }
    damn a / b
}

slay test_function_error_propagation() lit {
    shook {
        sus result drip = risky_divide(10, 0)
        vibez.spill("Should not reach here:", result)
        damn cringe
    } fam div_err {
        vibez.spill("Caught division error:", div_err)
        damn based
    }
}

assert_true(test_function_error_propagation())
vibez.spill("✅ Test 3: Function error propagation passed")

// Test 4: Complex error handling with multiple catch handlers
slay test_multiple_error_types() lit {
    shook {
        sus error_type drip = 2
        ready (error_type == 1) {
            yikes "Type 1 error"
        } otherwise ready (error_type == 2) {
            yikes "Type 2 error"
        } otherwise {
            yikes "Unknown error type"
        }
        damn cringe
    } fam caught_err {
        ready (std.mem.indexOf(caught_err, "Type 2") != null) {
            vibez.spill("Correctly caught Type 2 error")
            damn based
        } otherwise {
            vibez.spill("Unexpected error type:", caught_err)
            damn cringe
        }
    }
}

assert_true(test_multiple_error_types())
vibez.spill("✅ Test 4: Multiple error types passed")

// Test 5: Error handling in loops with shook
slay test_error_in_loop() lit {
    sus success_count drip = 0
    sus i drip = 0
    
    bestie (i < 5) {
        shook {
            ready (i == 3) {
                yikes "Error at iteration 3"
            }
            success_count = success_count + 1
        } fam loop_err {
            vibez.spill("Caught error in loop iteration", i, ":", loop_err)
            // Continue with next iteration
        }
        i = i + 1
    }
    
    ready (success_count == 4) {  // 5 iterations - 1 error = 4 successes
        damn based
    } otherwise {
        vibez.spill("Expected 4 successes, got:", success_count)
        damn cringe
    }
}

assert_true(test_error_in_loop())
vibez.spill("✅ Test 5: Error handling in loops passed")

// Test 6: Recovery and continuation after error
slay test_error_recovery() lit {
    sus recovery_successful lit = cringe
    
    shook {
        yikes "Recoverable error"
    } fam recovery_err {
        vibez.spill("Starting recovery from:", recovery_err)
        recovery_successful = based
        vibez.spill("Recovery completed successfully")
    }
    
    damn recovery_successful
}

assert_true(test_error_recovery())
vibez.spill("✅ Test 6: Error recovery passed")

// Test 7: shook with expression evaluation
slay test_shook_expression() lit {
    sus result drip = shook {
        damn 42  // Normal return
    } fam expr_err {
        vibez.spill("Unexpected error in expression:", expr_err)
        damn -1
    }
    
    ready (result == 42) {
        damn based
    } otherwise {
        vibez.spill("Expected 42, got:", result)
        damn cringe
    }
}

assert_true(test_shook_expression())
vibez.spill("✅ Test 7: shook expression evaluation passed")

// Test 8: Error state preservation across function calls
slay error_producing_function() tea {
    yikes "Function-level error"
    damn "should not return this"
}

slay test_error_state_preservation() lit {
    shook {
        sus error_result tea = error_producing_function()
        vibez.spill("Should not execute:", error_result)
        damn cringe
    } fam preserved_err {
        vibez.spill("Error preserved across function call:", preserved_err)
        damn based
    }
}

assert_true(test_error_state_preservation())
vibez.spill("✅ Test 8: Error state preservation passed")

print_test_summary()
vibez.spill("🎉 All advanced shook/fam error handling tests completed!")
