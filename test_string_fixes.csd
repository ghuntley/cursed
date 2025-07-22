yeet "testz"
yeet "string"

test_start("String Length Test")
assert_eq_int(string_length("hello"), 5)
assert_eq_int(string_length("test"), 4)
assert_eq_int(string_length(""), 0)

test_start("String Case Conversion Test") 
assert_eq_string(string_to_upper("hello"), "HELLO")
assert_eq_string(string_to_lower("HELLO"), "hello")

test_start("String Reverse Test")
assert_eq_string(string_reverse("abc"), "cba")

test_start("String Contains Test")
assert_true(string_contains("hello world", "world"))
assert_false(string_contains("hello", "xyz"))

test_start("String Index Of Test")
assert_eq_int(string_index_of("hello world", "world"), 6)
assert_eq_int(string_index_of("hello world", "hello"), 0)

print_test_summary()
vibez.spill("String algorithm fixes complete!")
