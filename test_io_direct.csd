yeet "testz"

# Test I/O module directly with simple functions

test_start("Basic I/O Test")

# Test basic file operations
sus message tea = "Hello World!"
vibez.spill(message)
assert_eq_string(message, "Hello World!")

# Test simple file reading simulation
sus filename tea = "test.txt"
sus content tea = "File content here"
assert_eq_string(content, "File content here")

print_test_summary()
