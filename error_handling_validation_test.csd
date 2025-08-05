yeet "testz"

# Error Handling Validation Test
# Validates that panic calls have been replaced with proper error handling

test_start("Error Handling Validation - No More Panics")

# Test 1: Create error and test safe unwrap
sus error_value yikes = yikes.init("Division by zero", 5)
sus result_error shook = shook.err(error_value)

# This should NOT crash the program
sus default_val = 42
sus safe_result = result_error.unwrapOr(default_val)
assert_eq_int(safe_result, 42)

# Test 2: Test successful result unwrap
sus success_val = 100
sus result_ok shook = shook.ok(success_val)
sus unwrapped_success = result_ok.unwrap()
# Should succeed without error

# Test 3: Test error propagation
sus propagation_result = result_error.propagate()
# Should return error without crashing

# Test 4: Test error information retrieval
sus error_info = result_error.getError()
# Should contain error information

assert_true(result_error.isError())
assert_false(result_error.isOk())
assert_true(result_ok.isOk())
assert_false(result_ok.isError())

print_test_summary()

vibez.spill("✅ Error handling validation complete - no panics detected!")
