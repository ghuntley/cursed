yeet "testz"

# Test all essential testing primitives
test_start("Core testing primitives")

# Basic assertions
assert_true(based)
assert_false(cap)
assert_eq_int(42, 42)
assert_eq_string("hello", "hello")

# Integer comparisons
assert_gt_int(100, 50)
assert_lt_int(25, 100)
assert_ge_int(50, 50)
assert_le_int(30, 30)

# Range assertions
assert_range_int(75, 50, 100)

# Error handling
assert_no_throw()

test_end()

# Test control functions
test_start("Test control features")
skip_test("Demonstration of skip functionality")
test_end()

test_start("Test utilities")
pending_test("Feature under development")
focus_test()
assert_true(based)
test_end()

# Final summary
print_test_summary()
