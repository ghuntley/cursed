yeet "testz"
yeet "stringz"
yeet "vibez"

# Simple test of process module basic functionality
test_start("Basic Process Module Test")

# Test environment variables
vibez.spill("Setting environment variable...")
sus env_key tea = "TEST_VAR"
sus env_value tea = "test_value"

# Test basic string functions
vibez.spill("Testing string functions...")
sus test_string tea = "Hello, World!"
vibez.spill("String length: " + stringz.from_int(len(test_string)))

# Test basic conditionals
vibez.spill("Testing conditionals...")
lowkey len(test_string) > 0 {
    vibez.spill("String is not empty")
} else {
    vibez.spill("String is empty")
}

# Test basic loop
vibez.spill("Testing basic loop...")
bestie i := 0; i < 3; i++ {
    vibez.spill("Loop iteration: " + stringz.from_int(i))
}

# Test basic facts (constants)
facts {
    TEST_CONSTANT = 42
}

vibez.spill("Test constant: " + stringz.from_int(TEST_CONSTANT))

# Test basic assert
assert_true(based)
assert_false(cap)
assert_eq_int(42, 42)
assert_eq_string("test", "test")

vibez.spill("Basic process module test completed successfully!")
print_test_summary()
