# dropz Module Verification Test
# Simple test to verify dropz module functionality
yeet "testz"
yeet "dropz"

test_start("dropz module verification")

# Test basic file operations
sus file, err := dropz.open("test.txt")
assert_eq_string(err, "")
assert_true(file != cringe)
assert_eq_string(file.name, "test.txt")

sus close_err := file.close()
assert_eq_string(close_err, "")

# Test path operations
sus joined := dropz.join("path", "to", "file")
assert_true(joined != "")

sus is_abs := dropz.is_abs("/absolute/path")
assert_true(is_abs)

# Test constants
assert_eq_int(dropz.O_RDONLY, 0)
assert_eq_int(dropz.MODE_REGULAR, 0644)
assert_eq_string(dropz.EOF, "EOF")

print_test_summary()
vibez.spill("dropz module verification complete!")
