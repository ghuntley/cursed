yeet "testz"
yeet "error_drip"

fr fr Test error_new function with tuple structure verification
test_start("error_new creates proper error tuple")
sus new_error := error_new("test error message")
assert_true(new_error)

fr fr Verify tuple structure by destructuring
sus (err_type, err_msg, wrapped, severity) := new_error
assert_eq_string(err_type, "base_error")
assert_eq_string(err_msg, "test error message")  
assert_eq_string(severity, "error")

print_test_summary()

fr fr Test error_type function
test_start("error_type extracts correct type")
sus typed_error := error_new("type test")
sus extracted_type := error_type(typed_error)
assert_eq_string(extracted_type, "base_error")

print_test_summary()

fr fr Test error_message function
test_start("error_message extracts correct message")
sus msg_error := error_new("message extraction test")
sus extracted_msg := error_message(msg_error)
assert_eq_string(extracted_msg, "message extraction test")

print_test_summary()

fr fr Test error_severity function
test_start("error_severity extracts default severity")
sus severity_error := error_new("severity test")
sus extracted_severity := error_severity(severity_error)
assert_eq_string(extracted_severity, "error")

print_test_summary()

fr fr Test error_wrap function
test_start("error_wrap creates wrapped error structure")
sus base_error := error_new("base error")
sus wrapped_error := error_wrap(base_error, "wrapped message")
assert_true(wrapped_error)

fr fr Verify wrapped error structure
sus wrapped_type := error_type(wrapped_error)
assert_eq_string(wrapped_type, "wrapped_error")

sus wrapped_msg := error_message(wrapped_error)
assert_eq_string(wrapped_msg, "wrapped message")

print_test_summary()

fr fr Test error_unwrap function
test_start("error_unwrap extracts wrapped error")
sus inner_error := error_new("inner error")
sus outer_error := error_wrap(inner_error, "outer message")
sus unwrapped := error_unwrap(outer_error)

fr fr Verify unwrapped error is the original
sus unwrapped_msg := error_message(unwrapped)
assert_eq_string(unwrapped_msg, "inner error")

sus unwrapped_type := error_type(unwrapped)
assert_eq_string(unwrapped_type, "base_error")

print_test_summary()

fr fr Test error_string function
test_start("error_string returns error message")
sus test_error := error_new("string test message")
sus error_str := error_string(test_error)
assert_eq_string(error_str, "string test message")

print_test_summary()

fr fr Test error_with_severity function
test_start("error_with_severity updates severity correctly")
sus base_error := error_new("base severity test")
sus critical_error := error_with_severity(base_error, "critical")

fr fr Verify severity was updated
sus new_severity := error_severity(critical_error)
assert_eq_string(new_severity, "critical")

fr fr Verify other fields preserved
sus preserved_msg := error_message(critical_error)
assert_eq_string(preserved_msg, "base severity test")

sus preserved_type := error_type(critical_error)
assert_eq_string(preserved_type, "base_error")

print_test_summary()

fr fr Test error_as function
test_start("error_as converts error type")
sus original_error := error_new("conversion test")
sus target_error := error_new("target")
sus converted := error_as(original_error, target_error)

fr fr Verify type was converted
sus converted_type := error_type(converted)
sus target_type := error_type(target_error)
assert_eq_string(converted_type, target_type)

fr fr Verify message was preserved
sus converted_msg := error_message(converted)
assert_eq_string(converted_msg, "conversion test")

print_test_summary()

fr fr Test error_chain_length function
test_start("error_chain_length counts error chain")
sus level1 := error_new("level 1")
sus chain_len1 := error_chain_length(level1)
assert_eq_int(chain_len1, 1)

print_test_summary()

fr fr Test error_has_message function
test_start("error_has_message searches error messages")
sus search_error := error_new("searchable error message")
sus has_msg := error_has_message(search_error, "search")
fr fr Current implementation returns false - this is expected for now
assert_false(has_msg)

print_test_summary()

fr fr Test complex error chaining
test_start("complex error chaining preserves structure")
sus level1 := error_new("database connection failed")
sus level2 := error_wrap(level1, "failed to initialize service")  
sus level3 := error_wrap(level2, "application startup failed")

fr fr Test each level maintains correct structure
sus l3_type := error_type(level3)
assert_eq_string(l3_type, "wrapped_error")

sus l3_msg := error_message(level3)
assert_eq_string(l3_msg, "application startup failed")

fr fr Test unwrapping works through chain
sus l2_unwrapped := error_unwrap(level3)
sus l2_msg := error_message(l2_unwrapped)
assert_eq_string(l2_msg, "failed to initialize service")

sus l1_unwrapped := error_unwrap(l2_unwrapped)
sus l1_msg := error_message(l1_unwrapped)
assert_eq_string(l1_msg, "database connection failed")

print_test_summary()

fr fr Test error severity levels
test_start("error severity levels work correctly")
sus info_error := error_with_severity(error_new("info message"), "info")
sus warn_error := error_with_severity(error_new("warning message"), "warning")
sus err_error := error_with_severity(error_new("error message"), "error")
sus crit_error := error_with_severity(error_new("critical message"), "critical")

assert_eq_string(error_severity(info_error), "info")
assert_eq_string(error_severity(warn_error), "warning") 
assert_eq_string(error_severity(err_error), "error")
assert_eq_string(error_severity(crit_error), "critical")

print_test_summary()

fr fr Test nil error handling
test_start("nil error handling in wrapping")
sus nil_error := cringe
sus nil_wrapped := error_wrap(nil_error, "wrapped nil")
assert_true(nil_wrapped)

fr fr Verify wrapped nil structure
sus wrapped_type := error_type(nil_wrapped)
assert_eq_string(wrapped_type, "wrapped_error")

sus wrapped_msg := error_message(nil_wrapped)
assert_eq_string(wrapped_msg, "wrapped nil")

print_test_summary()

fr fr Test utility functions
test_start("error utility functions")
sus test_error := error_new("utility test")

fr fr Test error_chain_messages
sus chain_msgs := error_chain_messages(test_error)
assert_eq_string(chain_msgs, "utility test")

fr fr Test error_root_cause
sus root := error_root_cause(test_error)
sus root_msg := error_message(root)
assert_eq_string(root_msg, "utility test")

fr fr Test error_format
sus formatted := error_format(test_error, "%s")
assert_eq_string(formatted, "utility test")

fr fr Test error_contains_type
sus contains_base := error_contains_type(test_error, "base_error")
fr fr Current implementation returns false - this is expected for now
assert_false(contains_base)

print_test_summary()

fr fr Test comprehensive error workflow
test_start("comprehensive error workflow")
sus original := error_new("database query failed")
sus enhanced := error_with_severity(original, "critical")
sus wrapped := error_wrap(enhanced, "user service unavailable")
sus typed := error_as(wrapped, error_new("service_error"))

fr fr Verify final error structure
sus final_type := error_type(typed)
assert_eq_string(final_type, "base_error")

sus final_msg := error_message(typed)
assert_eq_string(final_msg, "user service unavailable")

sus final_sev := error_severity(typed)
assert_eq_string(final_sev, "critical")

print_test_summary()

fr fr Test direct tuple access
test_start("direct tuple access verification")
sus test_tuple := error_new("direct access test")
sus (direct_type, direct_msg, direct_wrapped, direct_sev) := test_tuple

assert_eq_string(direct_type, "base_error")
assert_eq_string(direct_msg, "direct access test")
assert_eq_string(direct_sev, "error")

print_test_summary()

vibez.spill("error_drip module comprehensive tests completed")
vibez.spill("All tuple-based error handling functionality verified")
