# Simple test of testz module
yeet "testz"

# Test basic functionality
test_start("testz validation")
assert_eq_int(42, 42)
assert_eq_string("hello", "hello")
assert_true(based)
assert_false(cap)
print_test_summary()
