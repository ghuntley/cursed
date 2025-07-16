yeet "testz"
yeet "string"

test_start("String Library Basic Tests")

test_start("string_length function")
assert_eq_int(string_length("test"), 4)
assert_eq_int(string_length("hello"), 5)
assert_eq_int(string_length(""), 0)

test_start("string_concat function")
assert_eq_string(string_concat("hello", " world"), "hello world")
assert_eq_string(string_concat("test", "ing"), "testing")

test_start("string_contains function")
assert_true(string_contains("hello world", "world"))
assert_true(string_contains("hello world", "hello"))
assert_false(string_contains("hello", "xyz"))

test_start("string_to_upper function")
assert_eq_string(string_to_upper("hello"), "HELLO")
assert_eq_string(string_to_upper("test"), "TEST")

test_start("string_to_lower function")
assert_eq_string(string_to_lower("HELLO"), "hello")
assert_eq_string(string_to_lower("TEST"), "test")

print_test_summary()

vibez.spill("🎉 String Library Tests Complete!")
