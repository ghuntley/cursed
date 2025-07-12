yeet "testz"
yeet "error_handling"

test_start("Error Handling Module Tests")

// Test 1: Basic error creation
vibez.spill("=== Test 1: Basic error creation ===")
fam {
    sus error_result := create_error_with_code("Test error", 404)
    vibez.spill("This should not execute")
} sus basic_error {
    vibez.spill("✓ Basic error creation works:", basic_error)
    assert_true(basic_error != cringe)
}

// Test 2: Error with context
vibez.spill("=== Test 2: Error with context ===")
fam {
    sus context_result := create_error_with_context("Database connection failed", "PostgreSQL localhost:5432")
    vibez.spill("This should not execute")
} sus context_error {
    vibez.spill("✓ Error with context works:", context_error)
    assert_true(context_error != cringe)
}

// Test 3: Error wrapping
vibez.spill("=== Test 3: Error wrapping ===")
fam {
    sus wrapped_result := wrap_error("Connection timeout", "Database operation")
    vibez.spill("This should not execute")
} sus wrapped_error {
    vibez.spill("✓ Error wrapping works:", wrapped_error)
    assert_true(wrapped_error != cringe)
}

// Test 4: Error recovery check
vibez.spill("=== Test 4: Error recovery check ===")
sus recoverable1 lit = is_recoverable_error("Connection timeout")
sus recoverable2 lit = is_recoverable_error("fatal error")
assert_true(recoverable1)
assert_false(recoverable2)
vibez.spill("✓ Error recovery check works")

// Test 5: Retry operation
vibez.spill("=== Test 5: Retry operation ===")
fam {
    sus retry_result := retry_operation("test_operation", 3)
    vibez.spill("✓ Retry operation succeeded:", retry_result)
    assert_eq_string(retry_result, "success")
} sus retry_error {
    vibez.spill("✗ Retry operation failed:", retry_error)
    assert_true(cap)  // Should not reach here
}

// Test 6: Error logging
vibez.spill("=== Test 6: Error logging ===")
log_error("Test info message", 0)
log_error("Test warning message", 1)
log_error("Test error message", 2)
log_error("Test critical message", 3)
log_error("Test fatal message", 4)
vibez.spill("✓ Error logging works")

// Test 7: Panic handling with recovery
vibez.spill("=== Test 7: Panic handling with recovery ===")
sus panic_result := handle_panic_with_recovery("safe_operation")
assert_eq_string(panic_result, "success")

sus recovery_result := handle_panic_with_recovery("dangerous_operation")
assert_eq_string(recovery_result, "recovered")
vibez.spill("✓ Panic handling with recovery works")

// Test 8: Structured error creation
vibez.spill("=== Test 8: Structured error creation ===")
fam {
    sus structured_result := create_structured_error("Database connection failed", 500, "Connection pool exhausted")
    vibez.spill("This should not execute")
} sus structured_error {
    vibez.spill("✓ Structured error creation works:", structured_error)
    assert_true(structured_error != cringe)
}

// Test 9: Error configuration validation
vibez.spill("=== Test 9: Error configuration validation ===")
sus config_valid lit = validate_error_config()
assert_true(config_valid)
vibez.spill("✓ Error configuration validation works")

// Test 10: Complex error scenario
vibez.spill("=== Test 10: Complex error scenario ===")
slay complex_operation() {
    fam {
        sus inner_result := create_error_with_code("Inner operation failed", 400)
        damn inner_result
    } sus inner_error {
        sus wrapped_result := wrap_error(inner_error, "Complex operation")
        damn wrapped_result shook
    }
}

fam {
    sus complex_result := complex_operation()
    vibez.spill("This should not execute")
} sus complex_error {
    vibez.spill("✓ Complex error scenario works:", complex_error)
    assert_true(complex_error != cringe)
}

print_test_summary()
