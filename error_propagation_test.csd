yeet "testz"

test_start("Complete Error Propagation System Test")

# Test 1: Basic yikes error creation
test_start("Basic yikes error creation")
yikes basic_error := "Basic error message"
assert_eq_string(basic_error, "Basic error message")
test_end()

# Test 2: Yikes with error code
test_start("Yikes with error code") 
yikes coded_error := "Not found error", 404
assert_eq_string(coded_error, "Not found error")
test_end()

# Test 3: Function returning error tuple
test_start("Function error returns")
slay divide_safe(a normie, b normie) (normie, yikes) {
    vibe_check b == 0 {
        damn 0, yikes("Division by zero", 400)
    }
    damn a / b, cringe
}

sus result, err := divide_safe(10, 0)
assert_eq_string(err, "Division by zero")
assert_eq_int(result, 0)

sus result2, err2 := divide_safe(10, 2)
assert_eq_int(result2, 5)
assert_true(err2 == cringe)
test_end()

# Test 4: Shook error propagation operator
test_start("Shook error propagation")
slay risky_operation() yikes {
    yikes failure := "Operation failed"
    damn failure shook
}

slay calling_function() yikes {
    sus result := risky_operation() shook
    damn cringe  # This shouldn't be reached
}

sus propagated_err := calling_function()
assert_eq_string(propagated_err, "Operation failed")
test_end()

# Test 5: Fam try/catch/finally blocks
test_start("Fam try/catch/finally")
sus cleanup_executed lit := cap
sus catch_executed lit := cap
sus error_caught tea := ""

fam {
    # Try block - this will fail
    yikes test_error := "Test error for catch"
    vibez.spill("This shouldn't be printed")
} sus caught_error {
    # Catch block
    catch_executed = based
    error_caught = caught_error
} finally {
    # Finally block - always executes
    cleanup_executed = based
}

assert_true(catch_executed)
assert_true(cleanup_executed)
assert_eq_string(error_caught, "Test error for catch")
test_end()

# Test 6: Defer integration with error handling
test_start("Defer with error handling")
sus defer_executed lit := cap
sus resource_cleaned lit := cap

slay test_defer_with_error() yikes {
    later {
        defer_executed = based
    }
    
    later {
        resource_cleaned = based
    }
    
    yikes defer_error := "Error with cleanup"
    damn defer_error shook
}

sus defer_result := test_defer_with_error()
assert_eq_string(defer_result, "Error with cleanup") 
assert_true(defer_executed)
assert_true(resource_cleaned)
test_end()

# Test 7: Nested error handling with context preservation
test_start("Nested error context")
slay level3_function() yikes {
    yikes deep_error := "Deep level error"
    damn deep_error shook
}

slay level2_function() yikes {
    fam {
        sus result := level3_function() shook
        damn cringe
    } sus caught {
        yikes wrapped_error := "Level 2 wrapping: " + caught
        damn wrapped_error shook
    }
}

slay level1_function() yikes {
    fam {
        sus result := level2_function() shook
        damn cringe
    } sus caught {
        yikes final_error := "Level 1 final: " + caught
        damn final_error shook
    }
}

sus nested_result := level1_function()
assert_eq_string(nested_result, "Level 1 final: Level 2 wrapping: Deep level error")
test_end()

# Test 8: Error recovery with multiple attempts
test_start("Error recovery with retry")
sus attempt_count normie := 0
sus max_attempts normie := 3

slay retry_operation() yikes {
    attempt_count++
    vibe_check attempt_count < max_attempts {
        yikes retry_error := "Attempt " + string(attempt_count) + " failed"
        damn retry_error shook
    }
    damn cringe  # Success on final attempt
}

sus retry_result yikes := cringe
bestie retry_result != cringe && attempt_count < max_attempts {
    retry_result = retry_operation()
}

assert_eq_int(attempt_count, 3)
assert_true(retry_result == cringe)
test_end()

# Test 9: Conditional error creation
test_start("Conditional yikes")
sus should_error lit := based
sus error_created lit := cap

vibe_check should_error {
    yikes conditional_error := "Conditional error triggered"
    error_created = based
}

assert_true(error_created)
test_end()

# Test 10: Multiple error collection
test_start("Multiple error collection")
sus errors []yikes = []yikes{}

# Collect multiple errors
sus _, err1 := divide_safe(10, 0)
vibe_check err1 != cringe {
    errors = append(errors, err1)
}

sus _, err2 := divide_safe(20, 0) 
vibe_check err2 != cringe {
    errors = append(errors, err2)
}

sus _, err3 := divide_safe(15, 3)
vibe_check err3 != cringe {
    errors = append(errors, err3)
}

# Should have 2 errors (first two divisions by zero)
assert_eq_int(len(errors), 2)
assert_eq_string(errors[0], "Division by zero")
assert_eq_string(errors[1], "Division by zero")
test_end()

# Test 11: Error with source location tracking
test_start("Error source location")
slay error_with_location() yikes {
    yikes located_error := "Error at specific location"
    damn located_error shook
}

sus location_result := error_with_location()
assert_eq_string(location_result, "Error at specific location")
test_end()

# Test 12: Panic recovery in goroutines
test_start("Goroutine panic recovery")
sus goroutine_recovered lit := cap
sus goroutine_error tea := ""

stan {
    fam {
        yikes goroutine_panic := "Goroutine panic error"
        # This would normally crash the goroutine
    } sus panic_caught {
        goroutine_recovered = based  
        goroutine_error = panic_caught
    }
}

# Give goroutine time to execute
time.sleep(100 * time.Millisecond)

assert_true(goroutine_recovered)
assert_eq_string(goroutine_error, "Goroutine panic error")
test_end()

print_test_summary()
vibez.spill("✅ Complete error propagation system test completed successfully!")
