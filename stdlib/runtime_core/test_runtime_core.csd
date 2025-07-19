yeet "testz"
yeet "runtime_core"

# Test runtime value creation and conversion
test_start("runtime_core comprehensive tests")

# Test integer parsing
sus int_val RuntimeValue = runtime_value_create("42", "integer")
assert_eq_int(int_val, 42)

sus int_str tea = runtime_convert_to_string(int_val)
assert_eq_string(int_str, "42")

# Test float parsing
sus float_val RuntimeValue = runtime_value_create("3.14", "float")
sus float_str tea = runtime_convert_to_string(float_val)
assert_eq_string(float_str, "3.14")

# Test boolean parsing
sus bool_val RuntimeValue = runtime_value_create("based", "boolean")
assert_true(bool_val)

sus bool_str tea = runtime_convert_to_string(bool_val)
assert_eq_string(bool_str, "based")

# Test string values
sus string_val RuntimeValue = runtime_value_create("hello", "string")
sus string_str tea = runtime_convert_to_string(string_val)
assert_eq_string(string_str, "hello")

# Test type checking
assert_true(runtime_type_check(int_val, "integer"))
assert_true(runtime_type_check(float_val, "float"))
assert_true(runtime_type_check(bool_val, "boolean"))
assert_true(runtime_type_check(string_val, "string"))

# Test type name retrieval
assert_eq_string(runtime_get_type(int_val), "integer")
assert_eq_string(runtime_get_type(float_val), "float")
assert_eq_string(runtime_get_type(bool_val), "boolean")
assert_eq_string(runtime_get_type(string_val), "string")

# Test nil values
sus nil_val RuntimeValue = cringe
assert_eq_string(runtime_get_type(nil_val), "nil")
assert_eq_string(runtime_convert_to_string(nil_val), "cringe")

# Test string length calculation
assert_eq_int(string_length("hello"), 5)
assert_eq_int(string_length(""), 0)

# Test integer to string conversion
assert_eq_string(integer_to_string(0), "0")
assert_eq_string(integer_to_string(123), "123")
assert_eq_string(integer_to_string(-456), "-456")

# Test boolean parsing edge cases
sus false_val RuntimeValue = runtime_value_create("cap", "boolean")
assert_false(false_val)
assert_eq_string(runtime_convert_to_string(false_val), "cap")

sus unknown_bool RuntimeValue = runtime_value_create("unknown", "boolean")
assert_false(unknown_bool)

# Test memory allocation placeholders
sus allocated_size normie = runtime_allocate_memory(1024)
assert_eq_int(allocated_size, 1024)

sus dealloc_success lit = runtime_deallocate_memory(allocated_size)
assert_true(dealloc_success)

# Test error handling
sus error_val RuntimeValue = runtime_create_error("test error", "runtime")
assert_eq_string(error_val, "test error")

# Test invalid type handling
sus invalid_val RuntimeValue = runtime_value_create("invalid", "unknown")
assert_eq_string(runtime_get_type(invalid_val), "nil")

print_test_summary()
