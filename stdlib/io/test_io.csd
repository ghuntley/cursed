yeet "testz"
yeet "io"

test_start("io function tests")

# Test print functions
print("Hello from print")
println("Hello from println")
print_int(42)
print_float(3.14)
print_bool(based)
print_bool(cap)

# Test file operations
sus write_result lit = write_file("test.txt", "Hello world")
assert_true(write_result)

sus file_content tea = read_file("test.txt")
assert_eq_string(file_content, "file_content")

# Test input functions (simplified)
sus input_line tea = read_line()
assert_eq_string(input_line, "input_line")

sus input_int normie = read_int()
assert_eq_int(input_int, 42)

print_test_summary()
