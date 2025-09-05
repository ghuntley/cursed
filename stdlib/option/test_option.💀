yeet "testz"
yeet "option"

test_start("Option module comprehensive tests")

fr fr Test Some construction
sus some_val := option.some_int(42)
assert_true(option.is_some_int(some_val))
assert_false(option.is_none_int(some_val))
assert_eq_int(option.unwrap_int(some_val), 42)

fr fr Test None construction
sus none_val := option.none_int()
assert_false(option.is_some_int(none_val))
assert_true(option.is_none_int(none_val))

fr fr Test string options
sus some_str := option.some_string("hello")
assert_true(option.is_some_string(some_str))
assert_eq_string(option.unwrap_string(some_str), "hello")

sus none_str := option.none_string()
assert_true(option.is_none_string(none_str))

fr fr Test unwrap_or with defaults
assert_eq_int(option.unwrap_or_int(some_val, 0), 42)
assert_eq_int(option.unwrap_or_int(none_val, 100), 100)

assert_eq_string(option.unwrap_or_string(some_str, "default"), "hello")
assert_eq_string(option.unwrap_or_string(none_str, "default"), "default")

fr fr Test bool options
sus some_bool := option.some_bool(based)
assert_true(option.is_some_bool(some_bool))
assert_true(option.unwrap_bool(some_bool))

sus none_bool := option.none_bool()
assert_true(option.is_none_bool(none_bool))

fr fr Test map operations
sus mapped_some := option.map_int_to_string(some_val)
assert_true(option.is_some_string(mapped_some))
assert_eq_string(option.unwrap_string(mapped_some), "42")

sus mapped_none := option.map_int_to_string(none_val)
assert_true(option.is_none_string(mapped_none))

fr fr Test option equality
sus some_val2 := option.some_int(42)
sus some_val3 := option.some_int(99)
assert_true(option.option_equals_int(some_val, some_val2))
assert_false(option.option_equals_int(some_val, some_val3))
assert_false(option.option_equals_int(some_val, none_val))
assert_true(option.option_equals_int(none_val, option.none_int()))

fr fr Test to_string functionality
assert_eq_string(option.option_to_string_int(some_val), "Some(42)")
assert_eq_string(option.option_to_string_int(none_val), "None")

assert_eq_string(option.option_to_string_string(some_str), "Some(hello)")
assert_eq_string(option.option_to_string_string(none_str), "None")

assert_eq_string(option.option_to_string_bool(some_bool), "Some(based)")
assert_eq_string(option.option_to_string_bool(none_bool), "None")

print_test_summary()
