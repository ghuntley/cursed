yeet "testz"
yeet "error_drip"

# Simple error_drip test
test_start("basic error creation")
sus new_error := error_new("test message")
vibez.spill("Created error successfully")
print_test_summary()

test_start("error message extraction")
sus test_error := error_new("extract test")
sus msg := error_message(test_error)
vibez.spill("Extracted message: " + msg)
assert_eq_string(msg, "extract test")
print_test_summary()

test_start("error type verification")
sus typed_error := error_new("type test")
sus err_type := error_type(typed_error)
vibez.spill("Error type: " + err_type)
assert_eq_string(err_type, "base_error")
print_test_summary()

vibez.spill("Basic error_drip functionality verified")
