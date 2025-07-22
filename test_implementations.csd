# Test the implemented stdlib modules
yeet "testz"

# Test vibez module
test_start("vibez module test")
vibez.spill("Testing vibez.spill function")
result := vibez.format_string("Hello %s", "World")
assert_eq_string(result, "Hello World")
print_test_summary()

# Test simple functionality
test_start("Basic functionality test")
assert_true(based)
assert_false(cap)
vibez.spill("✅ All tests passing!")
print_test_summary()
