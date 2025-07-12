yeet "testz"
yeet "result"

test_start("Result module comprehensive tests")

# Test Ok construction
sus ok_val := result.ok_int(42)
assert_true(result.is_ok_int(ok_val))
assert_false(result.is_err_int(ok_val))
assert_eq_int(result.unwrap_int(ok_val), 42)

# Test Err construction
sus err_val := result.err_int("error message")
assert_false(result.is_ok_int(err_val))
assert_true(result.is_err_int(err_val))
assert_eq_string(result.unwrap_err_int(err_val), "error message")

# Test string results
sus ok_str := result.ok_string("hello")
assert_true(result.is_ok_string(ok_str))
assert_eq_string(result.unwrap_string(ok_str), "hello")

sus err_str := result.err_string("string error")
assert_true(result.is_err_string(err_str))
assert_eq_string(result.unwrap_err_string(err_str), "string error")

# Test unwrap_or with defaults
assert_eq_int(result.unwrap_or_int(ok_val, 0), 42)
assert_eq_int(result.unwrap_or_int(err_val, 100), 100)

assert_eq_string(result.unwrap_or_string(ok_str, "default"), "hello")
assert_eq_string(result.unwrap_or_string(err_str, "default"), "default")

# Test bool results
sus ok_bool := result.ok_bool(based)
assert_true(result.is_ok_bool(ok_bool))
assert_true(result.unwrap_bool(ok_bool))

sus err_bool := result.err_bool("bool error")
assert_true(result.is_err_bool(err_bool))
assert_eq_string(result.unwrap_err_bool(err_bool), "bool error")

# Test map operations
sus mapped_ok := result.map_int_to_string(ok_val)
assert_true(result.is_ok_string(mapped_ok))
assert_eq_string(result.unwrap_string(mapped_ok), "42")

sus mapped_err := result.map_int_to_string(err_val)
assert_true(result.is_err_string(mapped_err))
assert_eq_string(result.unwrap_err_string(mapped_err), "error message")

# Test result equality
sus ok_val2 := result.ok_int(42)
sus ok_val3 := result.ok_int(99)
sus err_val2 := result.err_int("error message")
sus err_val3 := result.err_int("different error")

assert_true(result.result_equals_int(ok_val, ok_val2))
assert_false(result.result_equals_int(ok_val, ok_val3))
assert_false(result.result_equals_int(ok_val, err_val))
assert_true(result.result_equals_int(err_val, err_val2))
assert_false(result.result_equals_int(err_val, err_val3))

# Test to_string functionality
assert_eq_string(result.result_to_string_int(ok_val), "Ok(42)")
assert_eq_string(result.result_to_string_int(err_val), "Err(error message)")

assert_eq_string(result.result_to_string_string(ok_str), "Ok(hello)")
assert_eq_string(result.result_to_string_string(err_str), "Err(string error)")

assert_eq_string(result.result_to_string_bool(ok_bool), "Ok(based)")
assert_eq_string(result.result_to_string_bool(err_bool), "Err(bool error)")

# Test safe operations
sus div_ok := result.safe_divide(10, 2)
assert_true(result.is_ok_int(div_ok))
assert_eq_int(result.unwrap_int(div_ok), 5)

sus div_err := result.safe_divide(10, 0)
assert_true(result.is_err_int(div_err))
assert_eq_string(result.unwrap_err_int(div_err), "division by zero")

# Test safe string operations
sus str_ok := result.safe_string_index("hello", 1)
assert_true(result.is_ok_string(str_ok))
assert_eq_string(result.unwrap_string(str_ok), "e")

sus str_err := result.safe_string_index("hello", 10)
assert_true(result.is_err_string(str_err))
assert_eq_string(result.unwrap_err_string(str_err), "index out of bounds")

# Test safe parsing
sus parse_ok := result.safe_int_parse("42")
assert_true(result.is_ok_int(parse_ok))
assert_eq_int(result.unwrap_int(parse_ok), 42)

sus parse_err := result.safe_int_parse("invalid")
assert_true(result.is_err_int(parse_err))
assert_eq_string(result.unwrap_err_int(parse_err), "invalid integer format")

print_test_summary()
