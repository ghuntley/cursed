yeet "testz"
yeet "math"
yeet "string"
yeet "vibez"

# Simple stdlib test that should work with current compiler
test_start("basic_stdlib_test")

# Test basic math operations
sus a normie = 10
sus b normie = 20
sus sum normie = 30
assert_eq_int(sum, 30)

# Test string operations
sus text tea = "Hello, World!"
assert_eq_string(text, "Hello, World!")

# Test vibez module
vibez.spill("Basic stdlib test completed")

print_test_summary()
