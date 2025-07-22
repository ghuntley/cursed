yeet "testz"

# Simple test to verify testz module core functionality
test_start("integer equality test")
assert_eq_int(42, 42)
assert_eq_int(1 + 1, 2)

test_start("string equality test")
assert_eq_string("hello", "hello")
assert_eq_string("world", "world")

test_start("boolean test")
assert_true(based)
assert_false(cringe)

test_start("comparison tests")
assert_greater_than(10, 5)
assert_less_than(3, 7)
assert_not_eq_int(10, 20)

print_test_summary()
