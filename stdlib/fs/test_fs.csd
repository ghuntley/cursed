fr fr CURSED Filesystem Module Tests
fr fr Comprehensive test suite for filesystem operations

yeet "testz"
yeet "fs"

fr fr ================================
fr fr Test Setup
fr fr ================================

slay setup_test_env() {
    fr fr Create test directory if it doesn't exist
    fs.create_dir("test_fs_temp")
}

slay cleanup_test_env() {
    fr fr Clean up test files
    fs.delete_file("test_fs_temp/test_file.txt")
    fs.delete_file("test_fs_temp/test_write.txt")
    fs.delete_file("test_fs_temp/test_content.txt")
    fs.remove_dir("test_fs_temp")
}

fr fr ================================
fr fr File Operations Tests
fr fr ================================

slay test_file_operations() {
    testz.test_start("File Operations")
    
    fr fr Setup test environment
    setup_test_env()
    
    fr fr Test 1: Create and write file
    sus test_path tea = "test_fs_temp/test_file.txt"
    sus test_content tea = "Hello, CURSED filesystem!"
    
    sus write_success lit = fs.write_file(test_path, test_content)
    testz.assert_true(write_success)
    
    fr fr Test 2: Check file exists
    sus exists lit = fs.file_exists(test_path)
    testz.assert_true(exists)
    
    fr fr Test 3: Read file content
    sus read_content tea = fs.read_file(test_path)
    testz.assert_eq_string(read_content, test_content)
    
    fr fr Test 4: Get file size
    sus file_size thicc = fs.get_file_size(test_path)
    testz.assert_true(file_size > 0)
    
    fr fr Test 5: Delete file
    sus delete_success lit = fs.delete_file(test_path)
    testz.assert_true(delete_success)
    
    fr fr Test 6: Verify file no longer exists
    sus exists_after_delete lit = fs.file_exists(test_path)
    testz.assert_false(exists_after_delete)
    
    fr fr Cleanup
    cleanup_test_env()
}

slay test_directory_operations() {
    testz.test_start("Directory Operations")
    
    fr fr Test 1: Create directory
    sus test_dir tea = "test_fs_dir"
    sus create_success lit = fs.create_dir(test_dir)
    testz.assert_true(create_success)
    
    fr fr Test 2: Check directory exists
    sus dir_exists lit = fs.file_exists(test_dir)
    testz.assert_true(dir_exists)
    
    fr fr Test 3: Check is directory
    sus is_directory lit = fs.is_dir(test_dir)
    testz.assert_true(is_directory)
    
    fr fr Test 4: List directory contents (should be empty)
    sus files []tea = fs.list_dir(test_dir)
    testz.assert_true(files.length == 0)
    
    fr fr Test 5: Create file in directory
    sus file_in_dir tea = test_dir + "/test_file.txt"
    sus file_content tea = "File in directory"
    sus write_success lit = fs.write_file(file_in_dir, file_content)
    testz.assert_true(write_success)
    
    fr fr Test 6: List directory contents (should have one file)
    sus files_after []tea = fs.list_dir(test_dir)
    testz.assert_true(files_after.length == 1)
    
    fr fr Test 7: Check file type
    sus is_file lit = fs.is_file(file_in_dir)
    testz.assert_true(is_file)
    
    fr fr Cleanup
    fs.delete_file(file_in_dir)
    fs.remove_dir(test_dir)
}

slay test_path_utilities() {
    testz.test_start("Path Utilities")
    
    fr fr Test 1: Join paths
    sus base tea = "/home/user"
    sus component tea = "documents"
    sus joined tea = fs.join_path(base, component)
    testz.assert_eq_string(joined, "/home/user/documents")
    
    fr fr Test 2: Join paths with trailing slash
    sus base_with_slash tea = "/home/user/"
    sus joined_with_slash tea = fs.join_path(base_with_slash, component)
    testz.assert_eq_string(joined_with_slash, "/home/user/documents")
    
    fr fr Test 3: Get file extension
    sus filename tea = "document.txt"
    sus extension tea = fs.get_extension(filename)
    testz.assert_eq_string(extension, ".txt")
    
    fr fr Test 4: Get filename without extension
    sus full_path tea = "/home/user/document.txt"
    sus basename tea = fs.get_basename(full_path)
    testz.assert_eq_string(basename, "document.txt")
    
    fr fr Test 5: Get extension from path
    sus path_extension tea = fs.get_extension(full_path)
    testz.assert_eq_string(path_extension, ".txt")
    
    fr fr Test 6: No extension case
    sus no_ext tea = "README"
    sus no_ext_result tea = fs.get_extension(no_ext)
    testz.assert_eq_string(no_ext_result, "")
}

slay test_file_info() {
    testz.test_start("File Information")
    
    fr fr Setup test file
    sus test_path tea = "test_info_file.txt"
    sus test_content tea = "Test file for info"
    
    sus write_success lit = fs.write_file(test_path, test_content)
    testz.assert_true(write_success)
    
    fr fr Test 1: Get file info
    sus info fs.FileInfo = fs.get_file_info(test_path)
    testz.assert_eq_string(info.name, "test_info_file.txt")
    testz.assert_true(info.size > 0)
    testz.assert_false(info.is_dir)
    
    fr fr Test 2: File permissions
    sus perms normie = fs.get_permissions(test_path)
    testz.assert_eq_int(perms, 644)
    
    fr fr Test 3: Set permissions
    sus set_perms_success lit = fs.set_permissions(test_path, 755)
    testz.assert_true(set_perms_success)
    
    fr fr Cleanup
    fs.delete_file(test_path)
}

slay test_error_handling() {
    testz.test_start("Error Handling")
    
    fr fr Test 1: Read non-existent file
    sus nonexistent_content tea = fs.read_file("nonexistent_file.txt")
    testz.assert_eq_string(nonexistent_content, "")
    
    fr fr Test 2: Check non-existent file
    sus nonexistent_exists lit = fs.file_exists("nonexistent_file.txt")
    testz.assert_false(nonexistent_exists)
    
    fr fr Test 3: Delete non-existent file
    sus delete_nonexistent lit = fs.delete_file("nonexistent_file.txt")
    testz.assert_false(delete_nonexistent)
    
    fr fr Test 4: List non-existent directory
    sus nonexistent_files []tea = fs.list_dir("nonexistent_dir")
    testz.assert_true(nonexistent_files.length == 0)
    
    fr fr Test 5: Get size of non-existent file
    sus nonexistent_size thicc = fs.get_file_size("nonexistent_file.txt")
    testz.assert_true(nonexistent_size == -1)
}

slay test_large_files() {
    testz.test_start("Large File Operations")
    
    fr fr Test 1: Create large file content
    sus large_content tea = ""
    bestie i := 0; i < 1000; i++ {
        large_content = large_content + "Line " + tea(i) + " of large file content\n"
    }
    
    fr fr Test 2: Write large file
    sus large_file_path tea = "large_test_file.txt"
    sus write_large_success lit = fs.write_file(large_file_path, large_content)
    testz.assert_true(write_large_success)
    
    fr fr Test 3: Read large file
    sus read_large_content tea = fs.read_file(large_file_path)
    testz.assert_eq_string(read_large_content, large_content)
    
    fr fr Test 4: Verify large file size
    sus large_file_size thicc = fs.get_file_size(large_file_path)
    testz.assert_true(large_file_size > 10000)
    
    fr fr Cleanup
    fs.delete_file(large_file_path)
}

slay test_recursive_directory_creation() {
    testz.test_start("Recursive Directory Creation")
    
    fr fr Test 1: Create nested directory structure
    sus nested_path tea = "level1/level2/level3"
    sus create_recursive_success lit = fs.create_dir_recursive(nested_path)
    testz.assert_true(create_recursive_success)
    
    fr fr Test 2: Verify nested directories exist
    testz.assert_true(fs.file_exists("level1"))
    testz.assert_true(fs.file_exists("level1/level2"))
    testz.assert_true(fs.file_exists("level1/level2/level3"))
    
    fr fr Test 3: Create file in nested directory
    sus nested_file tea = "level1/level2/level3/nested_file.txt"
    sus nested_content tea = "File in nested directory"
    sus write_nested_success lit = fs.write_file(nested_file, nested_content)
    testz.assert_true(write_nested_success)
    
    fr fr Test 4: Read file from nested directory
    sus read_nested_content tea = fs.read_file(nested_file)
    testz.assert_eq_string(read_nested_content, nested_content)
    
    fr fr Cleanup (manual cleanup due to recursive nature)
    fs.delete_file(nested_file)
    fs.remove_dir("level1/level2/level3")
    fs.remove_dir("level1/level2")
    fs.remove_dir("level1")
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

test_file_operations()
test_directory_operations()
test_path_utilities()
test_file_info()
test_error_handling()
test_large_files()
test_recursive_directory_creation()

fr fr Print test summary
testz.print_test_summary()
