# Minimal test for enhanced testz framework
yeet "testz"

# Simple test to verify basic functionality
suite_start("Basic Tests")

test_start("simple integer test")
assert_eq_int(2 + 2, 4)
test_end()

test_start("simple string test")
assert_eq_string("hello", "hello")
test_end()

suite_end()

print_test_summary()
