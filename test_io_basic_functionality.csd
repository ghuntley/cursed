yeet "testz"

# Basic functionality test - no imports needed

test_start("Basic Functionality Test")

# Test basic string operations
sus message tea = "Hello World!"
vibez.spill(message)
assert_eq_string(message, "Hello World!")

# Test basic boolean operations
sus success lit = based
assert_true(success)

sus failure lit = cap
assert_false(failure)

# Test basic integer operations
sus count normie = 42
assert_eq_int(count, 42)

print_test_summary()

vibez.spill("✅ Basic functionality test completed!")
