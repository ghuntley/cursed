fr fr FILEZ MODULE COMPREHENSIVE TEST SUITE
fr fr Testing all file system operations with error handling

yeet "testz"
yeet "filez"
yeet "vibez"

test_start("File Operations - Core Functions")

fr fr Test file existence checking
assert_true(file_exists("README.md"), "README.md should exist")
assert_false(file_exists("nonexistent_file_12345.txt"), "Nonexistent file should return false")

fr fr Test basic file write and read
sus test_content tea = "Hello, CURSED file operations!\nThis is a test file."
sus test_filename tea = "test_filez_basic.txt"

sus write_err tea = write_file(test_filename, test_content)
assert_eq_string(write_err, "", "File write should succeed")

(read_content, read_err) := read_file(test_filename)
assert_eq_string(read_err, "", "File read should succeed")
assert_eq_string(read_content, test_content, "Read content should match written content")

fr fr Test file size
(size, size_err) := file_size(test_filename)
assert_eq_string(size_err, "", "File size operation should succeed")
assert_true(size > 0, "File size should be greater than 0")

fr fr Test file copy
sus copy_filename tea = "test_filez_copy.txt"
sus copy_err tea = copy_file(test_filename, copy_filename)
assert_eq_string(copy_err, "", "File copy should succeed")

(copy_content, copy_read_err) := read_file(copy_filename)
assert_eq_string(copy_read_err, "", "Copy file read should succeed")
assert_eq_string(copy_content, test_content, "Copied content should match original")

fr fr Test file append
sus append_content tea = "\nAppended line for testing"
sus append_err tea = append_file(test_filename, append_content)
assert_eq_string(append_err, "", "File append should succeed")

(appended_content, append_read_err) := read_file(test_filename)
assert_eq_string(append_read_err, "", "Appended file read should succeed")
assert_true(string_length(appended_content) > string_length(test_content), "Appended content should be longer")

fr fr Test file move/rename
sus moved_filename tea = "test_filez_moved.txt"
sus move_err tea = move_file(copy_filename, moved_filename)
assert_eq_string(move_err, "", "File move should succeed")
assert_false(file_exists(copy_filename), "Original file should not exist after move")
assert_true(file_exists(moved_filename), "Moved file should exist")

fr fr Test file delete
sus delete_err tea = delete_file(moved_filename)
assert_eq_string(delete_err, "", "File delete should succeed")
assert_false(file_exists(moved_filename), "Deleted file should not exist")

test_start("Line-Based File Operations")

fr fr Test reading file as lines
(lines, lines_err) := read_file_lines(test_filename)
assert_eq_string(lines_err, "", "Reading file lines should succeed")
assert_true(array_length(lines) >= 2, "Should have at least 2 lines")

fr fr Test writing lines to file
sus lines_filename tea = "test_filez_lines.txt"
sus test_lines tea[value] = ["First line", "Second line", "Third line with spaces"]
sus write_lines_err tea = write_file_lines(lines_filename, test_lines)
assert_eq_string(write_lines_err, "", "Writing file lines should succeed")

(read_lines, read_lines_err) := read_file_lines(lines_filename)
assert_eq_string(read_lines_err, "", "Reading written lines should succeed")
assert_eq_int(array_length(read_lines), array_length(test_lines), "Line count should match")

test_start("Binary File Operations")

fr fr Test binary file operations
sus binary_filename tea = "test_filez_binary.bin"
sus binary_data drip[value] = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]  fr fr "Hello World"
sus write_binary_err tea = write_file_bytes(binary_filename, binary_data)
assert_eq_string(write_binary_err, "", "Binary file write should succeed")

(read_binary, read_binary_err) := read_file_bytes(binary_filename, 1024)
assert_eq_string(read_binary_err, "", "Binary file read should succeed")
assert_eq_int(array_length(read_binary), array_length(binary_data), "Binary data length should match")

test_start("File Metadata Operations")

fr fr Test comprehensive file information
(info, info_err) := file_info(test_filename)
assert_eq_string(info_err, "", "File info should succeed")
assert_true(info.is_readable, "File should be readable")
assert_true(info.is_writable, "File should be writable")
assert_false(info.is_directory, "File should not be a directory")
assert_true(info.size > 0, "File info size should be greater than 0")
assert_true(string_length(info.name) > 0, "File info should have a name")

fr fr Test file modification time
(mod_time, mod_time_err) := file_modified_time(test_filename)
assert_eq_string(mod_time_err, "", "File modification time should succeed")
assert_true(mod_time > 0, "Modification time should be positive")

fr fr Test file permissions
(perms, perms_err) := file_permissions(test_filename)
assert_eq_string(perms_err, "", "File permissions should succeed")
assert_true(string_length(perms) > 0, "Permissions string should not be empty")

fr fr Test setting file permissions
sus set_perms_err tea = set_file_permissions(test_filename, "644")
assert_eq_string(set_perms_err, "", "Setting file permissions should succeed")

fr fr Test file sync
sus sync_err tea = sync_file(test_filename)
assert_eq_string(sync_err, "", "File sync should succeed")

test_start("File Type Checking")

fr fr Test file type detection
assert_true(is_file(test_filename), "Test file should be detected as a file")
assert_false(is_directory(test_filename), "Test file should not be detected as a directory")

test_start("Directory Operations")

fr fr Test directory creation
sus test_dir tea = "test_filez_directory"
sus create_dir_err tea = create_directory(test_dir)
assert_eq_string(create_dir_err, "", "Directory creation should succeed")
assert_true(directory_exists(test_dir), "Created directory should exist")
assert_true(is_directory(test_dir), "Created path should be detected as directory")

fr fr Test directory listing
(dir_entries, list_err) := list_directory(".")
assert_eq_string(list_err, "", "Directory listing should succeed")
assert_true(array_length(dir_entries) > 0, "Directory should have entries")

fr fr Create test files in directory
sus dir_file1 tea = test_dir + "/file1.txt"
sus dir_file2 tea = test_dir + "/file2.txt"
write_file(dir_file1, "Content of file 1")
write_file(dir_file2, "Content of file 2")

fr fr Test directory copying
sus copy_dir tea = "test_filez_directory_copy"
sus copy_dir_err tea = copy_directory(test_dir, copy_dir)
assert_eq_string(copy_dir_err, "", "Directory copy should succeed")
assert_true(directory_exists(copy_dir), "Copied directory should exist")

fr fr Test directory removal (clean up files first)
delete_file(dir_file1)
delete_file(dir_file2)
delete_file(copy_dir + "/file1.txt")
delete_file(copy_dir + "/file2.txt")

sus remove_dir_err tea = remove_directory(test_dir)
assert_eq_string(remove_dir_err, "", "Directory removal should succeed")
assert_false(directory_exists(test_dir), "Removed directory should not exist")

sus remove_copy_dir_err tea = remove_directory(copy_dir)
assert_eq_string(remove_copy_dir_err, "", "Copy directory removal should succeed")

test_start("Working Directory Operations")

fr fr Test getting current working directory
(cwd, cwd_err) := get_working_directory()
assert_eq_string(cwd_err, "", "Getting working directory should succeed")
assert_true(string_length(cwd) > 0, "Working directory path should not be empty")

fr fr Note: Not testing set_working_directory as it affects global state

test_start("Temporary File Operations")

fr fr Test getting temp directory
(temp_dir, temp_err) := get_temp_directory()
assert_eq_string(temp_err, "", "Getting temp directory should succeed")
assert_true(string_length(temp_dir) > 0, "Temp directory path should not be empty")

fr fr Test creating temporary file
(temp_file, temp_file_err) := create_temp_file("cursed_test", ".tmp")
assert_eq_string(temp_file_err, "", "Creating temp file should succeed")
assert_true(string_length(temp_file) > 0, "Temp file path should not be empty")
assert_true(file_exists(temp_file), "Temp file should exist")

fr fr Test using temp file
sus temp_write_err tea = write_file(temp_file, "Temporary file content")
assert_eq_string(temp_write_err, "", "Writing to temp file should succeed")

(temp_read_content, temp_read_err) := read_file(temp_file)
assert_eq_string(temp_read_err, "", "Reading temp file should succeed")
assert_eq_string(temp_read_content, "Temporary file content", "Temp file content should match")

fr fr Clean up temp file
delete_file(temp_file)

test_start("Utility Functions")

fr fr Test filename validation
assert_true(is_valid_filename("normal_file.txt"), "Normal filename should be valid")
assert_true(is_valid_filename("file123.log"), "Alphanumeric filename should be valid")
assert_false(is_valid_filename(""), "Empty filename should be invalid")
assert_false(is_valid_filename("../etc/passwd"), "Path traversal should be invalid")
assert_false(is_valid_filename("file<script>"), "HTML characters should be invalid")
assert_false(is_valid_filename("file|pipe"), "Pipe character should be invalid")

fr fr Test file renaming
sus rename_source tea = "test_filez_rename_source.txt"
sus rename_dest tea = "test_filez_rename_dest.txt"
write_file(rename_source, "Content for rename test")

sus rename_err tea = rename_file(rename_source, rename_dest)
assert_eq_string(rename_err, "", "File rename should succeed")
assert_false(file_exists(rename_source), "Source file should not exist after rename")
assert_true(file_exists(rename_dest), "Destination file should exist after rename")

test_start("Error Handling Tests")

fr fr Test error conditions for file operations
(nonexistent_content, nonexistent_err) := read_file("definitely_nonexistent_file_12345.txt")
assert_true(string_length(nonexistent_err) > 0, "Reading nonexistent file should return error")
assert_eq_string(nonexistent_content, "", "Nonexistent file content should be empty")

fr fr Test empty filename errors
sus empty_write_err tea = write_file("", "content")
assert_true(string_length(empty_write_err) > 0, "Empty filename write should return error")

(empty_read_content, empty_read_err) := read_file("")
assert_true(string_length(empty_read_err) > 0, "Empty filename read should return error")

fr fr Test invalid operations
(invalid_size, invalid_size_err) := file_size("nonexistent_file_98765.txt")
assert_true(string_length(invalid_size_err) > 0, "Size of nonexistent file should return error")
assert_eq_int(invalid_size, 0, "Invalid file size should be 0")

sus invalid_delete_err tea = delete_file("nonexistent_file_11111.txt")
assert_true(string_length(invalid_delete_err) > 0, "Deleting nonexistent file should return error")

fr fr Test directory operation errors
(invalid_entries, invalid_list_err) := list_directory("nonexistent_directory_xyz")
assert_true(string_length(invalid_list_err) > 0, "Listing nonexistent directory should return error")
assert_eq_int(array_length(invalid_entries), 0, "Invalid directory entries should be empty")

sus invalid_mkdir_err tea = create_directory("")
assert_true(string_length(invalid_mkdir_err) > 0, "Creating directory with empty name should return error")

test_start("Edge Cases and Boundary Conditions")

fr fr Test operations with edge case filenames
sus edge_filename tea = "test_with_spaces_and_dots.txt"
sus edge_write_err tea = write_file(edge_filename, "Edge case content")
assert_eq_string(edge_write_err, "", "Writing file with spaces should succeed")

(edge_content, edge_read_err) := read_file(edge_filename)
assert_eq_string(edge_read_err, "", "Reading file with spaces should succeed")
assert_eq_string(edge_content, "Edge case content", "Edge case content should match")

fr fr Test empty content write
sus empty_content_write_err tea = write_file("test_empty_content.txt", "")
assert_true(string_length(empty_content_write_err) > 0, "Writing empty content should return error")

fr fr Test large filename
sus long_name tea = "very_long_filename_that_might_exceed_system_limits_but_should_be_handled_gracefully_by_the_validation_function_test.txt"
sus long_name_err tea = write_file(long_name, "content")
ready (string_length(long_name) > 255) {
    assert_true(string_length(long_name_err) > 0, "Very long filename should return error")
}

test_start("Performance and Stress Testing")

fr fr Test handling multiple files
sus stress_files tea[value] = ["stress1.txt", "stress2.txt", "stress3.txt", "stress4.txt", "stress5.txt"]
sus stress_content tea = "Stress test content for performance validation"

sus i drip = 0
bestie (i < array_length(stress_files)) {
    sus stress_write_err tea = write_file(stress_files[i], stress_content)
    assert_eq_string(stress_write_err, "", "Stress test file write should succeed")
    i = i + 1
}

fr fr Verify all files were created and can be read
i = 0
bestie (i < array_length(stress_files)) {
    assert_true(file_exists(stress_files[i]), "Stress test file should exist")
    (stress_read_content, stress_read_err) := read_file(stress_files[i])
    assert_eq_string(stress_read_err, "", "Stress test file read should succeed")
    assert_eq_string(stress_read_content, stress_content, "Stress test content should match")
    i = i + 1
}

test_start("Final Cleanup")

fr fr Clean up all test files
delete_file(test_filename)
delete_file(lines_filename)
delete_file(binary_filename)
delete_file(edge_filename)
delete_file(rename_dest)

fr fr Clean up stress test files
i = 0
bestie (i < array_length(stress_files)) {
    delete_file(stress_files[i])
    i = i + 1
}

fr fr Verify cleanup
assert_false(file_exists(test_filename), "Test file should be cleaned up")
assert_false(file_exists(lines_filename), "Lines test file should be cleaned up")
assert_false(file_exists(binary_filename), "Binary test file should be cleaned up")

vibez.spill("=== FILEZ MODULE TEST SUITE COMPLETE ===")
print_test_summary()

fr fr Additional integration tests for advanced scenarios
test_start("Integration Tests - Advanced Scenarios")

fr fr Test atomic file operations (write then read immediately)
sus atomic_filename tea = "test_atomic.txt"
sus atomic_content tea = "Atomic operation test content"
sus atomic_write_err tea = write_file(atomic_filename, atomic_content)
assert_eq_string(atomic_write_err, "", "Atomic write should succeed")

(atomic_read_content, atomic_read_err) := read_file(atomic_filename)
assert_eq_string(atomic_read_err, "", "Immediate read after write should succeed")
assert_eq_string(atomic_read_content, atomic_content, "Atomic operation content should match")

fr fr Test file operations with sync
sus sync_atomic_err tea = sync_file(atomic_filename)
assert_eq_string(sync_atomic_err, "", "Sync after write should succeed")

(atomic_after_sync, atomic_sync_read_err) := read_file(atomic_filename)
assert_eq_string(atomic_sync_read_err, "", "Read after sync should succeed")
assert_eq_string(atomic_after_sync, atomic_content, "Content after sync should match")

delete_file(atomic_filename)

fr fr Test cross-operation consistency
sus consistency_filename tea = "test_consistency.txt"
write_file(consistency_filename, "Initial content")

(size1, size1_err) := file_size(consistency_filename)
assert_eq_string(size1_err, "", "Size check 1 should succeed")

append_file(consistency_filename, " appended")

(size2, size2_err) := file_size(consistency_filename)
assert_eq_string(size2_err, "", "Size check 2 should succeed")
assert_true(size2 > size1, "Size should increase after append")

(final_content, final_read_err) := read_file(consistency_filename)
assert_eq_string(final_read_err, "", "Final read should succeed")
assert_eq_string(final_content, "Initial content appended", "Final content should be correct")

delete_file(consistency_filename)

vibez.spill("=== INTEGRATION TESTS COMPLETE ===")
vibez.spill("All filez module tests completed successfully!")
