yeet "testz"
yeet "error_core"

# Test Error Core Module with yikes/shook/fam patterns

test_start("Basic Error Creation - yikes")
sus runtime_err = yikes_runtime("Runtime failure")
lowkey runtime_err != cringe {
    test_pass("Runtime error created successfully")
} else {
    test_fail("Failed to create runtime error")
}

sus logic_err = yikes_logic("Logic error occurred")
lowkey logic_err != cringe {
    test_pass("Logic error created successfully")
} else {
    test_fail("Failed to create logic error")
}

test_start("Error Type Creation")
sus io_err = yikes_io("File not found")
sus memory_err = yikes_memory("Out of memory")
sus validation_err = yikes_validation("Invalid input")

lowkey io_err != cringe {
    test_pass("I/O error created")
} else {
    test_fail("I/O error creation failed")
}

test_start("Error Wrapping - shook")
sus original_err = yikes_runtime("Original error")
sus wrapped_err = shook_wrap(original_err, "Additional context")

lowkey wrapped_err != cringe {
    test_pass("Error wrapping successful")
} else {
    test_fail("Error wrapping failed")
}

test_start("Error Context Addition")
sus context_err = shook_context(original_err, "Function: test_function")
lowkey context_err != cringe {
    test_pass("Context addition successful")
} else {
    test_fail("Context addition failed")
}

test_start("Error Handling - fam pattern")
sus test_error = yikes_validation("Test error")
sus handled_result = fam_handle(test_error, "default_value")

assert_eq_string(handled_result, "default_value")

test_start("Error Recovery")
sus recovery_result = fam_recover(test_error, "recovery_function")
assert_eq_string(recovery_result, "Recovered from error")

test_start("Error Ignoring")
sus ignore_result lit = fam_ignore(test_error)
assert_true(ignore_result)

test_start("Error Validation")
sus is_err lit = is_error(test_error)
assert_true(is_err)

sus no_err lit = is_error(cringe)
assert_false(no_err)

test_start("Error Information Extraction")
sus err_type tea = error_type(test_error)
assert_eq_string(err_type, "runtime")

sus err_msg tea = error_message(test_error)
assert_eq_string(err_msg, "Error occurred")

sus err_code normie = error_code(test_error)
assert_eq_int(err_code, 1000)

test_start("Error Propagation")
sus should_prop lit = should_propagate(test_error)
assert_false(should_prop)

sus critical_err = yikes_new("critical", "Critical failure", 9000)
sus should_prop_critical lit = should_propagate(critical_err)
assert_true(should_prop_critical)

test_start("Error Recovery Attempts")
sus recovery_success lit = try_recovery(test_error, 3)
assert_true(recovery_success)

sus recovery_fail lit = try_recovery(test_error, 0)
assert_false(recovery_fail)

test_start("Panic Handling")
panic_with("Test panic")
sus recovered lit = recover_from_panic()
assert_true(recovered)

test_start("Error Statistics")
sus stats tea = error_stats()
lowkey stats != "" {
    test_pass("Error statistics available")
} else {
    test_fail("Error statistics empty")
}

test_start("Error Classification")
sus critical_test lit = is_critical_error(critical_err)
assert_true(critical_test)

sus recoverable_test lit = is_recoverable_error(test_error)
assert_true(recoverable_test)

test_start("Safe Operations")
sus div_result = safe_divide(10, 2)
lowkey div_result != cringe {
    test_pass("Safe division successful")
} else {
    test_fail("Safe division failed")
}

sus div_error = safe_divide(10, 0)
lowkey div_error != cringe {
    test_pass("Division by zero error caught")
} else {
    test_fail("Division by zero not detected")
}

test_start("Safe Access")
sus access_result = safe_access("data", 0)
assert_eq_string(access_result, "safe_value")

sus access_error = safe_access("data", -1)
lowkey access_error != cringe {
    test_pass("Negative index error caught")
} else {
    test_fail("Negative index not detected")
}

test_start("Error State Management")
sus last_err = get_last_error()
lowkey last_err != cringe {
    test_pass("Last error retrieved")
} else {
    test_fail("No last error found")
}

clear_errors()
sus cleared_err = get_last_error()
lowkey cleared_err == cringe {
    test_pass("Errors cleared successfully")
} else {
    test_fail("Errors not cleared")
}

print_test_summary()
