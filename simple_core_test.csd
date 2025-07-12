yeet "testz"
yeet "core"

test_start("Simple core test")
assert_eq_int(max(5, 3), 5)
assert_eq_string(string_from_int(42), "42")
assert_true(lit_from_int(1))
print_test_summary()
