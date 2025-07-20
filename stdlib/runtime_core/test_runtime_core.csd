yeet "testz"
yeet "runtime_core"

test_start("Runtime Core Module Tests")

// Test value creation
sus int_val RuntimeValue = runtime_value_create("42", "integer")
sus float_val RuntimeValue = runtime_value_create("3.14", "float")  
sus string_val RuntimeValue = runtime_value_create("hello", "string")
sus bool_val RuntimeValue = runtime_value_create("based", "boolean")

// Test type checking
assert_true(runtime_type_check(int_val, "integer"))
assert_true(runtime_type_check(float_val, "float"))
assert_true(runtime_type_check(string_val, "string"))
assert_true(runtime_type_check(bool_val, "boolean"))

// Test type names
assert_eq_string(runtime_get_type(int_val), "integer")
assert_eq_string(runtime_get_type(float_val), "float")
assert_eq_string(runtime_get_type(string_val), "string")
assert_eq_string(runtime_get_type(bool_val), "boolean")

// Test value conversion
sus int_str tea = runtime_convert_to_string(int_val)
sus float_str tea = runtime_convert_to_string(float_val)
sus string_str tea = runtime_convert_to_string(string_val)
sus bool_str tea = runtime_convert_to_string(bool_val)

assert_true(string_length(int_str) > 0)
assert_true(string_length(float_str) > 0)
assert_true(string_length(string_str) > 0)
assert_true(string_length(bool_str) > 0)

// Test integer parsing
assert_eq_int(parse_integer("123"), 123)
assert_eq_int(parse_integer("0"), 0)

// Test boolean parsing
assert_true(parse_boolean("based"))
assert_true(parse_boolean("true"))
assert_false(parse_boolean("cap"))
assert_false(parse_boolean("false"))

// Test enhanced functions
assert_true(string_length_enhanced("test") >= 0)

print_test_summary()
