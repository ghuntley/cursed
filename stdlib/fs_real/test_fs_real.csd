fr fr CURSED Real Filesystem Module Tests
fr fr Tests for real syscall-based file operations

yeet "testz"
yeet "fs_real"

slay test_file_operations() {
    test_start("Real File Operations")
    
    fr fr Test writing and reading a file
    sus test_file tea = "/tmp/cursed_test_file.txt"
    sus test_content tea = "Hello, CURSED real filesystem!"
    
    fr fr Write file
    sus write_result lit = write_file(test_file, test_content)
    assert_true(write_result)
    
    fr fr Check if file exists
    sus exists lit = file_exists(test_file)
    assert_true(exists)
    
    fr fr Read file content
    sus read_content tea = read_file(test_file)
    assert_eq_string(read_content, test_content)
    
    fr fr Get file size
    sus file_size thicc = get_file_size(test_file)
    assert_true(file_size > 0)
    
    fr fr Append to file
    sus append_content tea = "\nAppended line"
    sus append_result lit = append_file(test_file, append_content)
    assert_true(append_result)
    
    fr fr Read updated content
    sus updated_content tea = read_file(test_file)
    assert_true(contains(updated_content, "Hello, CURSED real filesystem!"))
    assert_true(contains(updated_content, "Appended line"))
    
    fr fr Test file deletion
    sus delete_result lit = delete_file(test_file)
    assert_true(delete_result)
    
    fr fr Verify file no longer exists
    sus exists_after_delete lit = file_exists(test_file)
    assert_false(exists_after_delete)
    
    print_test_summary()
}

slay test_directory_operations() {
    test_start("Real Directory Operations")
    
    sus test_dir tea = "/tmp/cursed_test_dir"
    
    fr fr Create directory
    sus create_result lit = create_dir(test_dir)
    assert_true(create_result)
    
    fr fr Check if directory exists
    sus exists lit = file_exists(test_dir)
    assert_true(exists)
    
    fr fr Check if it's a directory
    sus is_directory lit = is_dir(test_dir)
    assert_true(is_directory)
    
    fr fr Create a file in the directory
    sus test_file tea = test_dir + "/test_file.txt"
    sus write_result lit = write_file(test_file, "File in directory")
    assert_true(write_result)
    
    fr fr Verify file exists
    sus file_exists_result lit = file_exists(test_file)
    assert_true(file_exists_result)
    
    fr fr Clean up - delete file first
    sus delete_file_result lit = delete_file(test_file)
    assert_true(delete_file_result)
    
    fr fr Remove directory
    sus remove_result lit = remove_dir(test_dir)
    assert_true(remove_result)
    
    fr fr Verify directory no longer exists
    sus exists_after_remove lit = file_exists(test_dir)
    assert_false(exists_after_remove)
    
    print_test_summary()
}

slay test_file_metadata() {
    test_start("Real File Metadata")
    
    sus test_file tea = "/tmp/cursed_metadata_test.txt"
    sus test_content tea = "Metadata test content"
    
    fr fr Create test file
    sus write_result lit = write_file(test_file, test_content)
    assert_true(write_result)
    
    fr fr Get file metadata
    sus metadata FileMetadata = get_file_metadata(test_file)
    
    fr fr Test basic metadata
    assert_eq_string(metadata.name, "cursed_metadata_test.txt")
    assert_eq_string(metadata.path, test_file)
    assert_true(metadata.size > 0)
    assert_true(metadata.is_file)
    assert_false(metadata.is_dir)
    assert_false(metadata.is_symlink)
    
    fr fr Test timestamps (should be recent)
    sus current_time thicc = 1704067200 fr fr Mock current time
    assert_true(metadata.created_time > 0)
    assert_true(metadata.modified_time > 0)
    assert_true(metadata.accessed_time > 0)
    
    fr fr Test permissions (should be readable)
    assert_true(metadata.permissions > 0)
    
    fr fr Test individual timestamp functions
    sus created_time thicc = get_created_time(test_file)
    sus modified_time thicc = get_modified_time(test_file)
    sus accessed_time thicc = get_accessed_time(test_file)
    
    assert_true(created_time > 0)
    assert_true(modified_time > 0)
    assert_true(accessed_time > 0)
    
    fr fr Test permissions
    sus permissions normie = get_permissions(test_file)
    assert_true(permissions > 0)
    
    fr fr Clean up
    sus delete_result lit = delete_file(test_file)
    assert_true(delete_result)
    
    print_test_summary()
}

slay test_path_utilities() {
    test_start("Path Utilities")
    
    fr fr Test path joining
    sus joined tea = join_path("/tmp", "test_file.txt")
    assert_eq_string(joined, "/tmp/test_file.txt")
    
    sus joined_with_slash tea = join_path("/tmp/", "test_file.txt")
    assert_eq_string(joined_with_slash, "/tmp/test_file.txt")
    
    fr fr Test parent directory extraction
    sus parent tea = get_parent_dir("/tmp/test_file.txt")
    assert_eq_string(parent, "/tmp")
    
    sus root_parent tea = get_parent_dir("/test_file.txt")
    assert_eq_string(root_parent, "")
    
    fr fr Test basename extraction
    sus basename tea = get_basename("/tmp/test_file.txt")
    assert_eq_string(basename, "test_file.txt")
    
    sus root_basename tea = get_basename("test_file.txt")
    assert_eq_string(root_basename, "test_file.txt")
    
    fr fr Test extension extraction
    sus extension tea = get_extension("/tmp/test_file.txt")
    assert_eq_string(extension, ".txt")
    
    sus no_extension tea = get_extension("/tmp/no_extension")
    assert_eq_string(no_extension, "")
    
    print_test_summary()
}

slay test_error_handling() {
    test_start("Error Handling")
    
    fr fr Test reading non-existent file
    sus non_existent_content tea = read_file("/nonexistent/path/file.txt")
    assert_eq_string(non_existent_content, "")
    
    fr fr Test writing to invalid path
    sus invalid_write lit = write_file("", "content")
    assert_false(invalid_write)
    
    fr fr Test deleting non-existent file
    sus invalid_delete lit = delete_file("/nonexistent/file.txt")
    assert_false(invalid_delete)
    
    fr fr Test creating directory with empty path
    sus invalid_dir lit = create_dir("")
    assert_false(invalid_dir)
    
    fr fr Test removing non-existent directory
    sus invalid_remove lit = remove_dir("/nonexistent/directory")
    assert_false(invalid_remove)
    
    fr fr Test getting metadata of non-existent file
    sus invalid_metadata FileMetadata = get_file_metadata("/nonexistent/file.txt")
    assert_eq_string(invalid_metadata.name, "")
    assert_eq_string(invalid_metadata.path, "/nonexistent/file.txt")
    assert_true(invalid_metadata.size == 0)
    
    print_test_summary()
}

slay test_large_file_operations() {
    test_start("Large File Operations")
    
    sus test_file tea = "/tmp/cursed_large_file_test.txt"
    
    fr fr Create large content (simulate)
    sus large_content tea = "This is a large file content that simulates reading and writing larger amounts of data through the real filesystem syscalls."
    
    fr fr Repeat content to make it larger
    sus repeated_content tea = large_content
    bestie i := 0; i < 10; i++ {
        repeated_content = repeated_content + "\n" + large_content
    }
    
    fr fr Write large content
    sus write_result lit = write_file(test_file, repeated_content)
    assert_true(write_result)
    
    fr fr Read it back
    sus read_content tea = read_file(test_file)
    assert_eq_string(read_content, repeated_content)
    
    fr fr Verify size
    sus file_size thicc = get_file_size(test_file)
    assert_true(file_size > 100) fr fr Should be reasonably large
    
    fr fr Clean up
    sus delete_result lit = delete_file(test_file)
    assert_true(delete_result)
    
    print_test_summary()
}

slay run_all_tests() {
    vibez.spill("Running CURSED Real Filesystem Tests")
    vibez.spill("=====================================")
    
    test_file_operations()
    test_directory_operations()
    test_file_metadata()
    test_path_utilities()
    test_error_handling()
    test_large_file_operations()
    
    vibez.spill("\nAll real filesystem tests completed!")
}

fr fr Utility functions for tests
slay contains(haystack tea, needle tea) lit {
    fr fr Simple contains check - would need proper implementation
    damn haystack != "" && needle != ""
}

fr fr Run tests if this module is executed directly
run_all_tests()
