yeet "testz"
yeet "io"

# Simple test to validate I/O module syntax
test_start("I/O Module Basic Test")

# Test initialization
sus init_result IOResult = init_io()
assert_true(init_result.success)

# Test basic file operations
sus read_result IOResult = read_file("test.csd")
assert_true(read_result.success)

vibez.spill("I/O module syntax test complete")
print_test_summary()
