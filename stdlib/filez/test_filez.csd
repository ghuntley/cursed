fr fr CURSED File Operations Testing
yeet "testz"
yeet "filez"

fr fr Test basic file operations
test_start("File Operations Tests")

fr fr Test file_exists function
assert_true(file_exists("README.md"))
assert_false(file_exists("nonexistent_file.txt"))

fr fr Test read_file and write_file functions
sus test_content tea = "Hello, CURSED file operations!"
sus test_filename tea = "test_file.txt"

fr fr Write content to file
sus write_err tea = write_file(test_filename, test_content)
assert_eq_string(write_err, "")

fr fr Read content back from file
(read_content, read_err) := read_file(test_filename)
assert_eq_string(read_err, "")
assert_eq_string(read_content, test_content)

fr fr Test file_size function
(size, size_err) := file_size(test_filename)
assert_eq_string(size_err, "")
assert_true(size > 0)

fr fr Test copy_file function
sus copy_filename tea = "test_file_copy.txt"
sus copy_err tea = copy_file(test_filename, copy_filename)
assert_eq_string(copy_err, "")

fr fr Verify copied file content
(copy_content, copy_read_err) := read_file(copy_filename)
assert_eq_string(copy_read_err, "")
assert_eq_string(copy_content, test_content)

fr fr Test append_file function
sus append_content tea = "\nAppended line"
sus append_err tea = append_file(test_filename, append_content)
assert_eq_string(append_err, "")

fr fr Read appended content
(appended_content, append_read_err) := read_file(test_filename)
assert_eq_string(append_read_err, "")

fr fr Test delete_file function
sus delete_err tea = delete_file(copy_filename)
assert_eq_string(delete_err, "")
assert_false(file_exists(copy_filename))

fr fr Test error conditions
sus empty_filename_err tea = write_file("", "content")
assert_true(len(empty_filename_err) > 0)

sus empty_content_err tea = write_file("test.txt", "")
assert_true(len(empty_content_err) > 0)

fr fr Test file operations on non-existent files
(_, nonexistent_err) := read_file("definitely_not_a_file.txt")
assert_true(len(nonexistent_err) > 0)

(_, size_nonexistent_err) := file_size("definitely_not_a_file.txt")
assert_true(len(size_nonexistent_err) > 0)

fr fr Test filename validation
assert_true(is_valid_filename("normal_file.txt"))
assert_false(is_valid_filename(""))

fr fr Cleanup test file
delete_file(test_filename)

print_test_summary()
