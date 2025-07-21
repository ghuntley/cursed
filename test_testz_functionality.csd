yeet "testz"

# Test the testz framework functionality
test_start("Basic testz functionality test")

# Test integer assertions
assert_eq_int(5, 5)
assert_eq_int(10, 10)

# Test string assertions  
assert_eq_string("hello", "hello")
assert_eq_string("world", "world")

# Test boolean assertions
assert_true(based)
assert_false(cap)

# Test comparison assertions
assert_gt(10, 5)
assert_lt(3, 7)
assert_gte(5, 5)
assert_lte(8, 8)
assert_not_eq(4, 9)

# Test not null assertion
assert_not_null("not empty")

print_test_summary()
