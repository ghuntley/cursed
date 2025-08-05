yeet "testz"

# Comprehensive Error Handling Test
# Tests the new error handling system in Zig implementation

test_start("Error Handling Comprehensive Test")

# Test basic error creation and handling
sus error_value yikes = yikes.init("Test error", 1)
sus result_error shook = shook.err(error_value)

# Test error checking functions
assert_false(result_error.isOk())
assert_true(result_error.isError())

# Test safe unwrap with default
sus default_value = 42
sus unwrapped_default = result_error.unwrapOr(default_value)
assert_eq_int(unwrapped_default, 42)

# Test error propagation with proper handling
sus test_result = result_error.propagate()
# This should return an error, test that it doesn't crash

# Test successful result
sus success_value = 100
sus result_ok shook = shook.ok(success_value)

assert_true(result_ok.isOk())
assert_false(result_ok.isError())

# Test successful unwrap
sus unwrapped_ok = result_ok.unwrap()
# This should work without error

# Test error information retrieval
sus error_info = result_error.getError()
# Should not be null for error result

print_test_summary()
