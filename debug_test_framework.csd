yeet "testz"

vibez.spill("Testing test framework components")

# Test basic test_start
test_start("basic test")
vibez.spill("test_start works")

# Test assertions
assert_true(based)
vibez.spill("assert_true works")

assert_eq_string("hello", "hello")
vibez.spill("assert_eq_string works")

assert_eq_int(42, 42)
vibez.spill("assert_eq_int works")

# Test summary should be last
print_test_summary()
