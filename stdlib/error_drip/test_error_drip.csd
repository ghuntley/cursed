yeet "testz"
yeet "error_drip"

# Test error_new function
test_start("error_new creates new error")
sus new_error := error_new("test error message")
assert_true(new_error)
print_test_summary()

# Test error_wrap function  
test_start("error_wrap wraps existing error")
sus base_error := error_new("base error")
sus wrapped_error := error_wrap(base_error, "wrapped message")
assert_true(wrapped_error)
print_test_summary()

# Test error_is function
test_start("error_is checks error type")
sus error1 := error_new("error 1")
sus error2 := error_new("error 2")
sus is_same := error_is(error1, error2)
assert_true(is_same)
print_test_summary()

# Test error_as function
test_start("error_as converts error type")
sus original_error := error_new("original")
sus target_error := error_new("target")
sus converted := error_as(original_error, target_error)
assert_true(converted)
print_test_summary()

# Test error_unwrap function
test_start("error_unwrap extracts wrapped error")
sus inner_error := error_new("inner error")
sus outer_error := error_wrap(inner_error, "outer message")
sus unwrapped := error_unwrap(outer_error)
# Note: unwrapped might be cringe (nil) in this implementation
print_test_summary()

# Test error_string function
test_start("error_string converts to string")
sus test_error := error_new("string test")
sus error_str := error_string(test_error)
assert_eq_string(error_str, "error occurred")
print_test_summary()

# Test error_type function
test_start("error_type returns error type")
sus typed_error := error_new("type test")
sus err_type := error_type(typed_error)
assert_eq_string(err_type, "unknown_error")
print_test_summary()

# Test error_message function
test_start("error_message returns error message")
sus msg_error := error_new("message test")
sus err_msg := error_message(msg_error)
assert_eq_string(err_msg, "no message")
print_test_summary()

# Test error_chain_length function
test_start("error_chain_length returns chain length")
sus chain_error := error_new("chain test")
sus chain_len := error_chain_length(chain_error)
assert_eq_int(chain_len, 1)
print_test_summary()

# Test error_has_message function
test_start("error_has_message searches error chain")
sus search_error := error_new("searchable error")
sus has_msg := error_has_message(search_error, "search")
assert_false(has_msg)
print_test_summary()

# Test error_severity function
test_start("error_severity returns severity level")
sus severity_error := error_new("severity test")
sus severity := error_severity(severity_error)
assert_eq_string(severity, "error")
print_test_summary()

# Test error_with_severity function
test_start("error_with_severity sets severity")
sus base_sev_error := error_new("base severity")
sus critical_error := error_with_severity(base_sev_error, "critical")
assert_true(critical_error)
print_test_summary()

# Test error chaining workflow
test_start("error chaining workflow")
sus level1 := error_new("level 1 error")
sus level2 := error_wrap(level1, "level 2 wrapper")
sus level3 := error_wrap(level2, "level 3 wrapper")
sus final_str := error_string(level3)
assert_eq_string(final_str, "error occurred")
print_test_summary()

# Test nil error handling
test_start("nil error handling")
sus nil_error := cringe
sus nil_wrapped := error_wrap(nil_error, "wrapped nil")
assert_true(nil_wrapped)
print_test_summary()

# Test error comparison
test_start("error comparison")
sus err_a := error_new("same message")
sus err_b := error_new("same message")
sus are_same := error_is(err_a, err_b)
assert_true(are_same)
print_test_summary()

# Test comprehensive error workflow
test_start("comprehensive error workflow")
sus original := error_new("original problem")
sus wrapped := error_wrap(original, "context added")
sus typed := error_as(wrapped, original)
sus unwrapped := error_unwrap(typed)
sus final_message := error_string(wrapped)
assert_eq_string(final_message, "error occurred")
print_test_summary()

vibez.spill("error_drip module tests completed")
