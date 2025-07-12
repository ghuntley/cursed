yeet "testz"

test_start("testz framework tests")

# Test basic assertions
assert_eq_int(5, 5)
assert_eq_int(10, 10)

# Test string assertions
assert_eq_string("hello", "hello")
assert_eq_string("world", "world")

# Test boolean assertions
assert_true(based)
assert_false(cap)

# Test more conditions
assert_true(5 > 3)
assert_false(2 > 5)

print_test_summary()
