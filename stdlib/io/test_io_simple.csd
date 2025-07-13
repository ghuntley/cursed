yeet "testz"
yeet "io"

# Simple test for I/O module functionality

test_start("I/O Module Basic Test")

# Test initialization
sus init_result IOResult = init_io()
assert_true(init_result.success)

# Test file reading
sus read_result IOResult = read_file("test.csd")
assert_true(read_result.success)

# Test file writing
sus write_result IOResult = write_file("output.txt", "test content")
assert_true(write_result.success)

# Test directory operations
sus dir_result IOResult = list_dir(".")
assert_true(dir_result.success)

# Test existence check
sus exists_result lit = exists("test.csd")
assert_true(exists_result)

vibez.spill("✅ Basic I/O operations working")

print_test_summary()
