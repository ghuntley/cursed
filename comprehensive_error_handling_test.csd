yeet "testz"

test_start("Comprehensive Error Handling Test Suite")

# Test 1: Basic yikes error creation and storage
test_start("Basic yikes error creation")
yikes basic_error := "Test error message"
assert_eq_string(basic_error, "Test error message")
test_end()

# Test 2: Yikes error with context
test_start("Yikes error with context")
yikes context_error := "Database connection failed"
assert_eq_string(context_error, "Database connection failed")
test_end()

# Test 3: Function that returns errors
test_start("Function error returns")
slay divide_with_error(a normie, b normie) (normie, yikes) {
    vibe_check b == 0 {
        damn 0, yikes("Division by zero error")
    }
    damn a / b, cringe
}

sus result, err := divide_with_error(10, 0)
assert_eq_string(err, "Division by zero error")
assert_eq_int(result, 0)
test_end()

# Test 4: Shook error propagation
test_start("Shook error propagation")
slay risky_operation() yikes {
    yikes op_error := "Operation failed"
    damn op_error shook
}

sus propagated_error := risky_operation()
assert_eq_string(propagated_error, "Operation failed")
test_end()

# Test 5: Fam error recovery block
test_start("Fam error recovery")
sus recovery_successful lit := cap

fam {
    yikes test_panic := "Recoverable panic"
    # This should be caught by the recovery block
} sus caught_error {
    recovery_successful = based
    assert_eq_string(caught_error, "Recoverable panic")
}

assert_true(recovery_successful)
test_end()

# Test 6: Multiple error handling
test_start("Multiple error handling")
sus errors []yikes = []yikes{}
sus op1_result, op1_err := divide_with_error(10, 2)
sus op2_result, op2_err := divide_with_error(20, 0)

vibe_check op1_err != cringe {
    errors = append(errors, op1_err)
}
vibe_check op2_err != cringe {
    errors = append(errors, op2_err)
}

# Should have one error from op2
assert_eq_int(len(errors), 1)
assert_eq_string(errors[0], "Division by zero error")
test_end()

# Test 7: Error recovery with cleanup
test_start("Error recovery with cleanup")
sus cleanup_executed lit := cap

fam {
    defer {
        cleanup_executed = based
    }
    yikes cleanup_error := "Error requiring cleanup"
} sus caught_error {
    assert_eq_string(caught_error, "Error requiring cleanup")
}

assert_true(cleanup_executed)
test_end()

# Test 8: Nested error handling
test_start("Nested error handling")
slay nested_error_function() yikes {
    fam {
        yikes inner_error := "Inner error"
        damn inner_error shook
    } sus inner_caught {
        yikes outer_error := "Outer error wrapping: " + inner_caught
        damn outer_error shook
    }
}

sus nested_result := nested_error_function()
assert_eq_string(nested_result, "Outer error wrapping: Inner error")
test_end()

# Test 9: Error context preservation
test_start("Error context preservation")
slay database_operation() yikes {
    yikes db_error := "Connection timeout"
    damn db_error shook
}

slay service_operation() yikes {
    sus err := database_operation()
    vibe_check err != cringe {
        yikes service_error := "Service failed: " + err
        damn service_error shook
    }
    damn cringe
}

sus service_result := service_operation()
assert_eq_string(service_result, "Service failed: Connection timeout")
test_end()

# Test 10: Error severity levels
test_start("Error severity levels")
yikes info_error := "Information: Operation completed with warnings"
yikes warning_error := "Warning: Resource usage high"
yikes error_error := "Error: Operation failed"
yikes critical_error := "Critical: System instability detected"

# All should be created successfully
assert_eq_string(info_error, "Information: Operation completed with warnings")
assert_eq_string(warning_error, "Warning: Resource usage high")
assert_eq_string(error_error, "Error: Operation failed")
assert_eq_string(critical_error, "Critical: System instability detected")
test_end()

print_test_summary()
