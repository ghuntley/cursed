yeet "testz"
yeet "string_pure"

test_start("Basic String Functions Test")
assert_eq_int(string_length("hello"), 5)
assert_eq_int(string_length(""), 0)
assert_true(string_is_empty(""))
print_test_summary()
