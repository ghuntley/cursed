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

test_start("Advanced File Operations")

fr fr Test read_file_lines function
sus line_content tea = "Line 1\nLine 2\nLine 3"
sus lines_filename tea = "test_lines.txt"
sus write_lines_err tea = write_file(lines_filename, line_content)
assert_eq_string(write_lines_err, "")

(lines, lines_err) := read_file_lines(lines_filename)
assert_eq_string(lines_err, "")
assert_true(len(lines) >= 3)

fr fr Test write_file_lines function
sus new_lines tea[value] = ["First line", "Second line", "Third line"]
sus write_lines_result tea = write_file_lines("test_write_lines.txt", new_lines)
assert_eq_string(write_lines_result, "")

fr fr Test file_info function
(info, info_err) := file_info(test_filename)
assert_eq_string(info_err, "")
assert_true(info.is_file)
assert_false(info.is_directory)
assert_true(info.size > 0)

fr fr Test is_file and is_directory functions
assert_true(is_file(test_filename))
assert_false(is_directory(test_filename))

test_start("Directory Operations")

fr fr Test directory creation and operations
sus test_dir tea = "test_directory"
sus create_dir_err tea = create_directory(test_dir)
assert_eq_string(create_dir_err, "")
assert_true(directory_exists(test_dir))

fr fr Test directory listing
(entries, list_err) := list_directory(".")
assert_eq_string(list_err, "")
assert_true(len(entries) > 0)

fr fr Test working directory operations
(original_cwd, cwd_err) := get_working_directory()
assert_eq_string(cwd_err, "")
assert_true(len_str(original_cwd) > 0)

fr fr Test temp directory
(temp_dir, temp_err) := get_temp_directory()
assert_eq_string(temp_err, "")
assert_true(len_str(temp_dir) > 0)

fr fr Test temp file creation
(temp_file, temp_file_err) := create_temp_file("test", ".tmp")
assert_eq_string(temp_file_err, "")
assert_true(len_str(temp_file) > 0)

test_start("File Metadata Operations")

fr fr Test file modification time
(mod_time, mod_time_err) := file_modified_time(test_filename)
assert_eq_string(mod_time_err, "")
assert_true(mod_time > 0)

fr fr Test file sync
sus sync_err tea = sync_file(test_filename)
assert_eq_string(sync_err, "")

test_start("Binary File Operations")

fr fr Test binary file operations
sus binary_data normie[value] = [72, 101, 108, 108, 111] // "Hello" in ASCII
sus binary_filename tea = "test_binary.bin"
sus write_binary_err tea = write_file_bytes(binary_filename, binary_data)
assert_eq_string(write_binary_err, "")

(read_binary, read_binary_err) := read_file_bytes(binary_filename, 1024)
assert_eq_string(read_binary_err, "")
assert_true(len(read_binary) == len(binary_data))

test_start("Error Handling Tests")

fr fr Test error conditions
(nonexistent_content, nonexistent_err) := read_file("nonexistent.txt")
assert_true(len_str(nonexistent_err) > 0)

sus empty_write_err tea = write_file("", "content")
assert_true(len_str(empty_write_err) > 0)

sus empty_content_err tea = write_file("test.txt", "")
assert_true(len_str(empty_content_err) > 0)

(zero_size, size_error) := file_size("nonexistent.txt")
assert_true(len_str(size_error) > 0)

test_start("Cleanup")

fr fr Clean up test files
delete_file(test_filename)
delete_file(lines_filename)
delete_file("test_write_lines.txt")
delete_file(binary_filename)
delete_file(temp_file)
remove_directory(test_dir)

fr fr Test filename validation
assert_true(is_valid_filename("normal_file.txt"))
assert_false(is_valid_filename(""))

print_test_summary()
